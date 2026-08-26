> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Fehlerreferenz

> Schlagen Sie Claude Code-Laufzeitfehlermeldungen nach und erfahren Sie, was jede bedeutet und wie Sie sie beheben.

Diese Seite listet Laufzeitfehler auf, die Claude Code anzeigt, und wie Sie sich von jedem erholen können, sowie was Sie überprüfen sollten, wenn Antworten ohne Fehler seltsam wirken. Für Installationsfehler wie `command not found` oder TLS-Fehler während der Einrichtung siehe [Fehlerbehebung bei Installation und Anmeldung](/docs/de/troubleshoot-install).

Diese Fehler und Wiederherstellungsbefehle gelten für die CLI, die [Desktop-App](/docs/de/desktop) und [Claude Code im Web](/docs/de/claude-code-on-the-web), da alle drei die gleiche Claude Code CLI verwenden. Für oberflächenspezifische Probleme siehe den Abschnitt zur Fehlerbehebung auf der Seite dieser Oberfläche.

<Note>
  Claude Code ruft die Claude API für Modellantworten auf, daher werden die meisten Laufzeitfehler einem zugrunde liegenden API-Fehlercode zugeordnet. Diese Seite behandelt, was jeder Fehler in Claude Code bedeutet und wie Sie sich davon erholen. Für die rohen HTTP-Statuscode-Definitionen siehe die [Claude Platform-Fehlerreferenz](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Finden Sie Ihren Fehler
</h2>

Ordnen Sie die Meldung, die Sie in Ihrem Terminal sehen, einem Abschnitt unten zu.

| Meldung                                                                                            | Abschnitt                                                                                                                       |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `API Error: 500 Internal server error`                                                             | [Serverfehler](#api-error-500-internal-server-error)                                                                            |
| `API Error: Repeated 529 Overloaded errors`                                                        | [Serverfehler](#api-error-repeated-529-overloaded-errors)                                                                       |
| `Request timed out`                                                                                | [Serverfehler](#request-timed-out), oder [Netzwerk](#unable-to-connect-to-api) wenn die Meldung Ihre Internetverbindung erwähnt |
| `Server error mid-response. The response above may be incomplete.`                                 | [Serverfehler](#the-response-above-may-be-incomplete)                                                                           |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [Serverfehler](#the-response-above-may-be-incomplete)                                                                           |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [Serverfehler](#auto-mode-cannot-determine-the-safety-of-an-action)                                                             |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [Serverfehler](#auto-mode-cannot-determine-the-safety-of-an-action)                                                             |
| `Auto mode classifier transcript exceeded context window`                                          | [Serverfehler](#auto-mode-cannot-determine-the-safety-of-an-action)                                                             |
| `Agent terminated early due to an API error`                                                       | [Serverfehler](#agent-terminated-early-due-to-an-api-error)                                                                     |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [Nutzungslimits](#youve-hit-your-session-limit)                                                                                 |
| `Usage credits required for 1M context`                                                            | [Nutzungslimits](#usage-credits-required-for-1m-context)                                                                        |
| `Server is temporarily limiting requests`                                                          | [Nutzungslimits](#server-is-temporarily-limiting-requests)                                                                      |
| `Request rejected (429)`                                                                           | [Nutzungslimits](#request-rejected-429)                                                                                         |
| `Credit balance is too low`                                                                        | [Nutzungslimits](#credit-balance-is-too-low)                                                                                    |
| `Not logged in · Please run /login`                                                                | [Authentifizierung](#not-logged-in)                                                                                             |
| `Could not resolve authentication method`                                                          | [Authentifizierung](#could-not-resolve-authentication-method)                                                                   |
| `Invalid API key`                                                                                  | [Authentifizierung](#invalid-api-key)                                                                                           |
| `Your apiKeyHelper script is failing`                                                              | [Authentifizierung](#your-apikeyhelper-script-is-failing)                                                                       |
| `This organization has been disabled`                                                              | [Authentifizierung](#this-organization-has-been-disabled)                                                                       |
| `Your organization has disabled API key authentication`                                            | [Authentifizierung](#your-organization-has-disabled-api-key-authentication)                                                     |
| `Your organization has disabled Claude subscription access`                                        | [Authentifizierung](#your-organization-has-disabled-claude-subscription-access)                                                 |
| `Routines are disabled by your organization's policy`                                              | [Authentifizierung](#routines-are-disabled-by-your-organizations-policy)                                                        |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [Authentifizierung](#remote-control-requires-the-anthropic-api)                                                                 |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [Authentifizierung](#oauth-token-revoked-or-expired)                                                                            |
| `Login expired · Please run /login`                                                                | [Authentifizierung](#login-expired)                                                                                             |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [Authentifizierung](#login-expired)                                                                                             |
| `does not meet scope requirement user:profile`                                                     | [Authentifizierung](#oauth-scope-requirement)                                                                                   |
| `AWS credentials expired or invalid`                                                               | [Authentifizierung](#aws-credentials-expired-or-invalid)                                                                        |
| `AWS authentication failed`                                                                        | [Authentifizierung](#aws-authentication-failed)                                                                                 |
| `AWS default-chain credential resolve timed out`                                                   | [Authentifizierung](#aws-default-chain-credential-resolve-timed-out)                                                            |
| `Unable to connect to API`                                                                         | [Netzwerk](#unable-to-connect-to-api)                                                                                           |
| `Waiting for API response · will retry in`                                                         | [Automatische Wiederholungen](#automatic-retries), oder [Netzwerk](#unable-to-connect-to-api) wenn es anhält                    |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [Netzwerk](#bedrock-streaming-response-has-an-unexpected-content-type)                                                          |
| `SSL certificate verification failed`                                                              | [Netzwerk](#ssl-certificate-errors)                                                                                             |
| `SSL certificate error (...)` during login or startup                                              | [Netzwerk](#ssl-certificate-errors)                                                                                             |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [Netzwerk](#host-not-allowed-in-a-cloud-session)                                                                                |
| `Couldn't reconnect to your Remote Control session`                                                | [Netzwerk](#couldnt-reconnect-to-your-remote-control-session)                                                                   |
| `Prompt is too long`                                                                               | [Anfragefehler](#prompt-is-too-long)                                                                                            |
| `Error during compaction: Conversation too long`                                                   | [Anfragefehler](#error-during-compaction-conversation-too-long)                                                                 |
| `Request too large`                                                                                | [Anfragefehler](#request-too-large)                                                                                             |
| `Image was too large`                                                                              | [Anfragefehler](#image-was-too-large)                                                                                           |
| `Unable to resize image`                                                                           | [Anfragefehler](#unable-to-resize-image)                                                                                        |
| `PDF too large` / `PDF is password protected`                                                      | [Anfragefehler](#pdf-errors)                                                                                                    |
| `Extra inputs are not permitted`                                                                   | [Anfragefehler](#extra-inputs-are-not-permitted)                                                                                |
| `There's an issue with the selected model`                                                         | [Anfragefehler](#theres-an-issue-with-the-selected-model)                                                                       |
| `Model ... is not a recognized model id`                                                           | [Anfragefehler](#model-is-not-a-recognized-model-id)                                                                            |
| `Claude Opus is not available with the Claude Pro plan`                                            | [Anfragefehler](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                         |
| `Model ... is restricted by your organization's settings`                                          | [Anfragefehler](#model-is-restricted-by-your-organizations-settings)                                                            |
| `thinking.type.enabled is not supported for this model`                                            | [Anfragefehler](#thinking-type-enabled-is-not-supported-for-this-model)                                                         |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [Anfragefehler](#thinking-budget-exceeds-output-limit)                                                                          |
| `API Error: 400 due to tool use concurrency issues`                                                | [Anfragefehler](#tool-use-or-thinking-block-mismatch)                                                                           |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [Anfragefehler](#usage-policy-refusal)                                                                                          |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [Anfragefehler](#safety-measures-flagged-a-cybersecurity-topic)                                                                 |
| `Installation was killed before it could finish (exit code 137)`                                   | [Installationsfehler](#installation-was-killed-before-it-could-finish)                                                          |
| `The connection dropped while downloading the update`                                              | [Installationsfehler](#the-connection-dropped-while-downloading-the-update)                                                     |
| `Download timed out: exceeded the total deadline`                                                  | [Installationsfehler](#the-connection-dropped-while-downloading-the-update)                                                     |
| `--bg and --print conflict`                                                                        | [Befehlszeilenfehler](#command-line-errors)                                                                                     |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [Befehlszeilenfehler](#command-line-errors)                                                                                     |
| `Could not import <server>: <reason>`                                                              | [Befehlszeilenfehler](#could-not-import-a-server-from-claude-desktop)                                                           |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [Befehlszeilenfehler](#mcp-permission-prompt-tool-not-found)                                                                    |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [Plugin-Fehler](#marketplace-is-registered-from-an-untrusted-source)                                                            |
| `references ${user_config.*} in a shell-form command`                                              | [Plugin-Fehler](#plugin-command-references-user-config)                                                                         |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [Plugin-Fehler](#plugin-command-references-user-config)                                                                         |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [Plugin-Fehler](#plugin-command-references-user-config)                                                                         |
| `would be spawned with zero tools — refusing`                                                      | [Werkzeugfehler](#agent-would-be-spawned-with-zero-tools)                                                                       |
| `File is covered by a Read deny rule in your permission settings`                                  | [Werkzeugfehler](#file-is-covered-by-a-read-deny-rule)                                                                          |
| `Can't open MCP settings in a background session`                                                  | [Fehler in Hintergrundsitzungen](#commands-refused-in-a-background-session)                                                     |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [Fehler in Hintergrundsitzungen](#claude_code_process_wrapper-launcher-errors)                                                  |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [Konfigurationswarnungen](#workspace-has-not-been-trusted)                                                                      |
| Responses seem lower quality than usual                                                            | [Antwortqualität](#responses-seem-lower-quality-than-usual)                                                                     |

<h2 id="automatic-retries">
  Automatische Wiederholungen
</h2>

Claude Code wiederholt vorübergehende Fehler, bevor ein Fehler angezeigt wird. Serverfehler, Überlastungsantworten, Anfrage-Timeouts, vorübergehende 429-Drosselungen und unterbrochene Verbindungen werden alle bis zu 10-mal mit exponentiellem Backoff wiederholt. Ab v2.1.198 umfasst dies Verbindungen, die in der Mitte einer Antwort abbrechen, bevor sichtbare Ausgabe gestreamt wurde: Claude Code sendet die Anfrage mit dem gleichen Backoff erneut aus und der Turn wird fortgesetzt, anstatt mit einem Verbindungsfehler zu stoppen. Ab v2.1.199 werden auch vorübergehende 429-Drosselungen, die nicht die Quota-Header Ihres Plans tragen, wiederholt, wenn Sie mit einem claude.ai-Abonnement angemeldet sind; frühere Versionen wiederholten sie nur für API-Schlüssel- und Enterprise-Anmeldungen.

Einige Fehlerklassen werden nicht wiederholt, da eine Wiederholung nicht erfolgreich sein kann:

* Ab v2.1.199 schlägt ein TLS-Zertifikatvalidierungsfehler, wie ein TLS-inspizierender Proxy, ein fehlender `NODE_EXTRA_CA_CERTS` Bundle oder ein abgelaufenes Zertifikat, beim ersten Versuch fehl, daher wird die Behebung sofort angezeigt, anstatt nach dem vollständigen Wiederholungsbudget. Siehe [SSL-Zertifikatsfehler](#ssl-certificate-errors). Vorübergehende TLS-Bedingungen wie ein Handshake-Timeout werden weiterhin wiederholt.
* Ab v2.1.199 behält ein Serverfehler, der ankommt, nachdem Claude bereits sichtbare Ausgabe gestreamt hat, die Teilantwort und fügt stattdessen eine [Benachrichtigung über unvollständige Antwort](#the-response-above-may-be-incomplete) an, anstatt zu wiederholen, da das erneute Ausführen der Anfrage die gleichen Tools zweimal ausführen könnte. Frühere Versionen verwarfen die Teilausgabe und meldeten den Turn als Fehler.
* Eine [Amazon Bedrock Streaming-Antwort mit einem unerwarteten Content-Type](#bedrock-streaming-response-has-an-unexpected-content-type) schlägt beim ersten Versuch fehl, da das Gateway oder der Proxy, der die Antwort umschreibt, die Wiederholung auf die gleiche Weise umschreiben würde. Erfordert Claude Code v2.1.208 oder später.

Während der Wiederholung zeigt der Spinner einen `Retrying in Ns · attempt x/y` Countdown nach einem Fehler-Label an. Das Label benennt den spezifischen Grund aus dem ersten Versuch für Fehler, auf die Sie sofort reagieren können: Das Netzwerk ist ausgefallen, ein TLS-Handshake ist fehlgeschlagen, oder Sie haben ein Ratenlimit erreicht. Für andere Fehler lautet es zunächst `API error`. Ab v2.1.198 wechselt es zum spezifischen Grund aus dem dritten Versuch, oder beim letzten Versuch, wenn `CLAUDE_CODE_MAX_RETRIES` weniger als drei erlaubt; frühere Versionen wechseln nur beim letzten Versuch.

Ab v2.1.198 wird der übliche Spinner-Tipp während Wiederholungen unterdrückt. Sobald der Fehlergrund offenbart wird, wenn der Fehler eine 529-Überlastung ist, benennt die Zeile unter dem Countdown auch, wo der Dienststatus überprüft werden kann: `status.claude.com` auf der Anthropic API, oder der in der Nachricht genannte Provider- oder Gateway-Host bei anderen Konfigurationen.

Wenn 20 Sekunden lang keine Daten im Antwortstrom ankommen, während eine Anfrage noch ausstehend ist, zeigt der Spinner `Waiting for API response · will retry in … · check your network` an, bevor ein Wiederholungsversuch gestartet wurde. Die Anfrage ist noch nicht fehlgeschlagen: Der Countdown läuft bis zu dem Punkt, an dem Claude Code die stillgelegte Verbindung abbricht und wiederholt, sodass das Banner von selbst verschwindet, sobald Daten wieder ankommen oder die Wiederholung erfolgreich ist. Ab v2.1.185 beträgt der Schwellenwert 20 Sekunden; frühere Versionen zeigen das Banner nach 10 Sekunden mit unterschiedlicher Formulierung an. Wenn es bei jedem Versuch erneut angezeigt wird, behandeln Sie es als [Netzwerkproblem](#unable-to-connect-to-api).

Wenn Sie einen der Fehler auf dieser Seite sehen, wurden diese Wiederholungen bereits erschöpft, es sei denn, er gehört zu einer Klasse, die nicht wiederholt wird, wie ein Zertifikatvalidierungsfehler. Sie können das Verhalten mit diesen Umgebungsvariablen anpassen:

| Variable                                     | Standard      | Effekt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------------------- | :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/de/env-vars)    | 10            | Anzahl der Wiederholungsversuche. Ab v2.1.186 auf 15 begrenzt; ab v2.1.199 hebt `CLAUDE_CODE_RETRY_WATCHDOG` den Standard an und entfernt die Obergrenze. Senken Sie den Wert, um Fehler in Skripten schneller anzuzeigen.                                                                                                                                                                                                                                                                                  |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/de/env-vars) | nicht gesetzt | Setzen Sie auf `1` in unbeaufsichtigten Sitzungen wie CI-Jobs, um `429`- und `529`-Kapazitätsfehler unbegrenzt zu wiederholen, anstatt nach `CLAUDE_CODE_MAX_RETRIES`-Versuchen fehlzuschlagen. Ab v2.1.199 erhöht es auch die Standard-Wiederholungsanzahl für andere vorübergehende Fehler, wie Serverfehler, Timeouts und unterbrochene Verbindungen, auf 300, ungefähr drei Stunden Backoff, und entfernt die Obergrenze von 15 auf `CLAUDE_CODE_MAX_RETRIES`, wenn Sie diese Variable explizit setzen. |
| [`API_TIMEOUT_MS`](/docs/de/env-vars)             | 600000        | Pro-Anfrage-Timeout in Millisekunden. Erhöhen Sie es für langsame Netzwerke oder Proxys.                                                                                                                                                                                                                                                                                                                                                                                                                    |

<h2 id="server-errors">
  Serverfehler
</h2>

Diese Fehler stammen vom Inferenzanbieter und nicht von Ihrem Konto oder Ihrer Anfrage. Bei der Anthropic API bedeutet das Anthropic-Infrastruktur. Bei Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder einem benutzerdefinierten Gateway bedeutet das die Infrastruktur dieses Anbieters.

<h3 id="api-error-500-internal-server-error">
  API-Fehler: 500 Interner Serverfehler
</h3>

Claude Code zeigt den Statuscode und die Fehlermeldung der API für jede 5xx-Antwort an. Das folgende Beispiel zeigt eine 500-Antwort auf der Anthropic API:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

Der abschließende Satz benennt, wo die Serviceintegrität überprüft werden kann, und variiert je nach Anbieter. Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry-Konfigurationen benennen die Servicestatuseite dieses Anbieters. Eine benutzerdefinierte `ANTHROPIC_BASE_URL` benennt den Gateway-Host.

Dies deutet auf einen unerwarteten Fehler innerhalb der API hin. Er wird nicht durch Ihren Prompt, Ihre Einstellungen oder Ihr Konto verursacht.

**Was zu tun ist:**

* Überprüfen Sie [status.claude.com](https://status.claude.com) oder die in der Meldung genannte Servicestatusseite des Anbieters auf aktive Vorfälle
* Warten Sie eine Minute und senden Sie Ihre Nachricht erneut. Ihre ursprüngliche Nachricht ist noch im Gespräch, sodass Sie bei einem langen Prompt `try again` eingeben können, anstatt alles erneut einzufügen.
* Wenn der Fehler ohne einen veröffentlichten Vorfall weiterhin auftritt, führen Sie `/feedback` aus, damit Anthropic Ihre Anfrageinformationen untersuchen kann. Siehe [Fehler melden](#report-an-error), wenn `/feedback` in Ihrer Umgebung nicht verfügbar ist.

<h3 id="api-error-repeated-529-overloaded-errors">
  API-Fehler: Wiederholte 529 Overloaded-Fehler
</h3>

Die API ist vorübergehend über alle Benutzer hinweg ausgelastet. Claude Code hat bereits mehrmals versucht, die Anfrage erneut zu senden, bevor diese Meldung angezeigt wird:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

Der abschließende Satz variiert je nach Anbieter auf die gleiche Weise wie der 500-Fehler oben.

Ein 529-Fehler ist nicht Ihr Nutzungslimit und wird nicht auf Ihr Kontingent angerechnet.

**Was zu tun ist:**

* Überprüfen Sie [status.claude.com](https://status.claude.com) oder die in der Meldung genannte Servicestatusseite des Anbieters auf Kapazitätsmitteilungen
* Versuchen Sie es in ein paar Minuten erneut
* Führen Sie `/model` aus und wechseln Sie zu einem anderen Modell, um weiterarbeiten zu können, da die Kapazität pro Modell verfolgt wird. Claude Code fordert Sie dazu auf, wenn ein Modell unter besonders hoher Last steht, zum Beispiel `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Anfrage hat das Zeitlimit überschritten
</h3>

Die API hat nicht vor der Verbindungsfrist geantwortet.

```text theme={null}
Request timed out
```

Dies kann während Zeiten hoher Last auftreten oder wenn das Modell eine sehr große Antwort generiert. Das Standard-Anfrage-Zeitlimit beträgt 10 Minuten.

**Was zu tun ist:**

* Wiederholen Sie die Anfrage
* Teilen Sie die Arbeit bei langfristigen Aufgaben in kleinere Prompts auf
* Wenn eine langsame Netzwerkverbindung oder ein Proxy die Ursache ist, erhöhen Sie `API_TIMEOUT_MS` wie in [Automatische Wiederholungen](#automatic-retries) beschrieben
* Wenn Zeitüberschreitungen häufig auftreten und Ihr Netzwerk ansonsten fehlerfrei ist, siehe [Netzwerk- und Verbindungsfehler](#network-and-connection-errors) unten

<h3 id="the-response-above-may-be-incomplete">
  Die obige Antwort kann unvollständig sein
</h3>

Eine Streaming-Antwort ist fehlgeschlagen, nachdem Claude bereits sichtbare Ausgabe produziert hatte. Das erneute Senden der Anfrage könnte die gleichen Tool-Aufrufe zweimal ausführen, daher behält Claude Code das bereits Gestreamte und fügt stattdessen diese Mitteilung an, anstatt den Zug zu verwerfen. Welche Variante Sie sehen, benennt die Ursache:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* `Server error mid-response`: ein Mid-Stream-Overloaded- oder 5xx-Serverfehler. Diese Variante erfordert Claude Code v2.1.199 oder später; davor verwarf dieser Fall die teilweise Ausgabe und meldete den gesamten Zug als Fehler.
* `Connection closed mid-response`: die Verbindung wurde unterbrochen.
* `Response stalled mid-stream`: der Stream hat das Senden von Daten gestoppt.

**Was zu tun ist:**

* Lesen Sie die Antwort, die gestreamt wurde. Nichts ist verloren gegangen, aber die letzten Sätze oder Tool-Aufrufe können fehlen.
* Antworten Sie mit `continue`, damit Claude dort weitermacht, wo es aufgehört hat
* Wenn der gleiche Fehler vor einer sichtbaren Ausgabe auftritt, wiederholt Claude Code die Anfrage, anstatt sie abzuschließen. Siehe [Automatische Wiederholungen](#automatic-retries).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto-Modus kann die Sicherheit einer Aktion nicht bestimmen
</h3>

Das Modell, das der [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) zur Klassifizierung von Aktionen verwendet, konnte keine Entscheidung treffen, daher genehmigte der Auto-Modus die Aktion nicht automatisch. Die Meldung, die Sie sehen, hängt davon ab, warum der Klassifizierer fehlgeschlagen ist.

Lesevorgänge, Suchen und Bearbeitungen in Ihrem Arbeitsverzeichnis überspringen den Klassifizierer, daher funktionieren sie in all diesen Fällen weiterhin.

Wenn das Klassifizierer-Modell überlastet ist:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**Was zu tun ist:**

* Wiederholen Sie nach ein paar Sekunden; Claude sieht die gleiche Meldung und wiederholt normalerweise automatisch
* Wenn Wiederholungen weiterhin fehlschlagen, fahren Sie mit schreibgeschützten Aufgaben fort und kehren Sie später zur blockierten Aktion zurück
* Dies ist vorübergehend und hängt nicht mit der [Auto-Modus-Berechtigung](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) zusammen; Sie müssen die Einstellungen nicht ändern

Wenn der Klassifizierer eine nicht analysierbare Antwort zurückgegeben hat:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Was zu tun ist:**

* Wiederholen Sie die Aktion; dies ist normalerweise beim nächsten Versuch erfolgreich
* Führen Sie `claude --debug` aus und wiederholen Sie die Aktion, um die zugrunde liegende Klassifizierer-Antwort im Debug-Protokoll zu sehen

Wenn eine separate API-Sicherheitsprüfung die Klassifizierer-Anfrage aufgrund früherer Gesprächsinhalte blockiert hat:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**Was zu tun ist:**

* Dies ist keine Entscheidung über Ihre Aktion. Inhalte, die bereits in Ihrem Gespräch vorhanden sind, haben einen Sicherheitsfilter auf der API ausgelöst, als der Auto-Modus das Gespräch an den Klassifizierer sendete
* Eine Wiederholung hilft nicht; der gleiche Gesprächsinhalt löst den Filter erneut aus
* Wechseln Sie zu einem anderen [Berechtigungsmodus](/docs/de/permission-modes), damit Sie die Aktion bei Aufforderung genehmigen können, oder starten Sie ein neues Gespräch ohne den auslösenden Inhalt

Wenn das Gespräch größer als das Kontextfenster des Klassifizierers geworden ist:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

In einer interaktiven Sitzung fällt der Auto-Modus auf eine normale Berechtigungsaufforderung für diese Aktion zurück, damit Sie sie manuell genehmigen oder ablehnen können. Im [nicht-interaktiven Modus](/docs/de/headless) wird die Ausführung abgebrochen, da das Transkript nur wächst und eine Wiederholung nicht erfolgreich sein kann.

**Was zu tun ist:**

* Genehmigen oder lehnen Sie die Aktion in der angezeigten Aufforderung ab
* Führen Sie `/compact` aus, um die Gesprächsgröße zu reduzieren, damit nachfolgende Aktionen wieder in das Klassifizierer-Fenster passen

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent wurde aufgrund eines API-Fehlers vorzeitig beendet
</h3>

Eine [Subagent](/docs/de/sub-agents)-API-Anfrage ist terminal fehlgeschlagen, zum Beispiel weil ein Nutzungslimit erreicht wurde oder Wiederholungen für einen Serverfehler aufgebraucht wurden, daher stoppte der Subagent, bevor er seine Aufgabe beendete. Diese Meldung erfordert Claude Code v2.1.199 oder später; davor wurde der API-Fehlertext an Claude zurückgegeben, als wäre er das Ergebnis des Subagenten.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Was zu tun ist:**

* Ordnen Sie die Fehlerdetails nach dem Doppelpunkt einem eigenen Abschnitt auf dieser Seite zu, wie z. B. [Nutzungslimits](#usage-limits) oder [Serverfehler](#server-errors), und folgen Sie den Schritten dieses Abschnitts
* Sobald der zugrunde liegende Fehler behoben ist, bitten Sie Claude, die Aufgabe zu wiederholen oder den [Subagenten fortzusetzen](/docs/de/sub-agents#resume-subagents)

Wenn ein Ratenlimit, eine Überlastung oder ein Serverfehler einen Vordergrund-Subagenten unterbricht, der bereits Textausgabe produziert hat, erhält Claude diese teilweise Ausgabe als unvollständig markiert, anstatt diesen Fehler. Ein Subagent, dessen einzige Ausgabe Tool-Aufrufe waren, erhält auch diesen Fehler; in v2.1.199 gab dies stattdessen ein leeres Teilergebnis zurück. Siehe [API-Fehler in Subagenten](/docs/de/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Nutzungslimits
</h2>

Diese Fehler bedeuten, dass ein Kontingent, das an Ihr Konto oder Ihren Plan gebunden ist, erreicht wurde. Sie unterscheiden sich von [Serverfehlern](#server-errors), die alle betreffen.

<h3 id="youve-hit-your-session-limit">
  Sie haben Ihr Sitzungslimit erreicht
</h3>

Abonnementpläne enthalten eine rollende Nutzungszuteilung. Wenn diese aufgebraucht ist, sehen Sie eine dieser Meldungen:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code blockiert weitere Anfragen bis zum in der Meldung angezeigten Zurücksetzzeitpunkt. Die Sitzungs- und Wochenlimits werden über alle Modelle hinweg gemeinsam genutzt, daher stellt das Wechseln von Modellen den Zugriff nicht wieder her. Das Opus-Limit gilt nur für Opus-Anfragen, daher können Sie mit `/model` zu einem anderen Modell wechseln und weiterarbeiten.

Die Nutzung wird gleichzeitig gegen die Sitzungs- und Wochenzuteilungen angerechnet. Ein einzelner Ausbruch intensiver Aktivität, wie z. B. ein großer Workflow-Fanout, kann die Wochenzuteilung aufbrauchen, bevor sich das Sitzungsfenster zurückgesetzt hat.

**Was Sie tun können:**

* Warten Sie auf den im Fehler angezeigten Zurücksetzzeitpunkt
* Führen Sie für das Opus-Limit `/model` aus und wechseln Sie zu einem anderen Modell, um weiterarbeiten zu können
* Führen Sie `/usage` aus, um Ihre Planlimits und deren Zurücksetzzeitpunkte anzuzeigen
* Führen Sie `/usage-credits` aus, um zusätzliche Nutzung auf Pro und Max zu kaufen, oder fordern Sie diese von Ihrem Administrator auf Team und Enterprise an. Siehe [Nutzungsguthaben für bezahlte Pläne](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) für Informationen zur Abrechnung.
* Um Ihren Plan für höhere Basislimits zu aktualisieren, siehe [claude.com/pricing](https://claude.com/pricing)

Um Ihre verbleibende Zuteilung zu überwachen, bevor Sie das Limit erreichen, fügen Sie die `rate_limits`-Felder zu einer [benutzerdefinierten Statuszeile](/docs/de/statusline#rate-limit-usage) hinzu, oder klicken Sie in der Desktop-App auf den [Nutzungsring](/docs/de/desktop#check-usage) neben dem Modellwähler.

<h3 id="usage-credits-required-for-1m-context">
  Nutzungsguthaben erforderlich für 1M-Kontext
</h3>

Das ausgewählte Modell verwendet das 1M-Token-Kontextfenster mit erweiterter Länge, und Ihr Plan enthält es nur über Nutzungsguthaben.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

Dies ist eine Berechtigungsprüfung, keine Kontingenterschöpfung. Sie wird auch dann ausgelöst, wenn Ihre Sitzungs- und Wochenzuteilungen noch Kapazität haben. Siehe [Erweiterter Kontext](/docs/de/model-config#extended-context) für Informationen darüber, welche Pläne 1M-Kontext direkt enthalten und welche Nutzungsguthaben erfordern.

Wenn dieser Fehler während eines Gesprächs auftritt, weil der Kontext über 200K Token gewachsen ist, komprimiert Claude Code das Gespräch automatisch zurück unter das Standard-Kontextlimit und behält die Sitzung danach auf diesem Limit, sodass keine Aktion erforderlich ist. In Versionen vor v2.1.172 wiederholte sich der Fehler bei jeder nachfolgenden Anfrage einschließlich `/compact`; führen Sie `/clear` in diesen Versionen aus, um die Funktion wiederherzustellen. Die folgenden Schritte gelten, wenn Sie explizit ein `[1m]`-Modell ausgewählt haben.

**Was Sie tun können:**

* Führen Sie `/model` aus und wählen Sie die Variante ohne das `[1m]`-Suffix, um auf das Standard-Kontextfenster zurückzufallen
* Führen Sie `/usage-credits` aus, um die getaktete Abrechnung für die 1M-Variante auf Pro und Max zu aktivieren, oder fordern Sie diese von Ihrem Administrator auf Team und Enterprise an
* Wenn der Fehler nach `/model` weiterhin besteht, kann eine 1M-Modell-ID an anderer Stelle festgelegt sein. Siehe [Es gibt ein Problem mit dem ausgewählten Modell](#theres-an-issue-with-the-selected-model) für die Konfigurationsorte, die in Prioritätsreihenfolge zu überprüfen sind.
* Um 1M-Varianten vollständig aus dem Modellwähler zu entfernen, setzen Sie [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/de/env-vars)

<h3 id="server-is-temporarily-limiting-requests">
  Server drosselt Anfragen vorübergehend
</h3>

Die API hat eine kurzlebige Drosselung angewendet, die nicht mit Ihrem Plankontingent zusammenhängt.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code unterscheidet diese von Ihrem Planlimit durch das Fehlen der einheitlichen Kontingent-Header, die eine echte Limit-Antwort trägt. Ab v2.1.199 wird dies [automatisch erneut versucht](#automatic-retries) mit Backoff, bevor es angezeigt wird, unabhängig davon, wie Sie sich authentifizieren. In früheren Versionen schlug eine Sitzung, die mit einem claude.ai-Abonnement angemeldet war, beim ersten Auftreten fehl; nur API-Schlüssel und Enterprise-Anmeldungen wiederholten den Versuch.

**Was Sie tun können:**

* Warten Sie kurz und versuchen Sie es erneut
* Überprüfen Sie [status.claude.com](https://status.claude.com), wenn das Problem weiterhin besteht

<h3 id="request-rejected-429">
  Anfrage abgelehnt (429)
</h3>

Sie haben das für Ihren API-Schlüssel, Ihr Amazon Bedrock-Projekt oder Ihr Google Cloud-Projekt konfigurierte Ratenlimit erreicht.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

Der nachfolgende Satz benennt, wo die Serviceintegrität überprüft werden kann, und variiert je nach Anbieter. Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry-Konfigurationen benennen stattdessen die Servicestatusseite dieses Anbieters anstelle der Anthropic-Statusseite. Eine benutzerdefinierte `ANTHROPIC_BASE_URL` benennt den Gateway-Host.

**Was Sie tun können:**

* Führen Sie `/status` aus und bestätigen Sie, dass die aktive Anmeldeinformation die ist, die Sie erwarten. Ein verwaister `ANTHROPIC_API_KEY` in Ihrer Umgebung kann Anfragen über einen Low-Tier-Schlüssel statt über Ihr Abonnement leiten.
* Überprüfen Sie Ihre Anbieter-Konsole auf die aktiven Limits und fordern Sie einen höheren Tier an, falls erforderlich
* Für Anthropic API-Schlüssel siehe die [Ratenlimit-Referenz](https://platform.claude.com/docs/en/api/rate-limits) für Informationen zur Funktionsweise von Tiers und zum Festlegen von Pro-Workspace-Limits
* Reduzieren Sie die Parallelität: senken Sie [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/de/env-vars), vermeiden Sie die Ausführung vieler paralleler Subagenten, oder wechseln Sie mit `/model` zu einem kleineren Modell für Läufe mit hohem Volumen

<h3 id="credit-balance-is-too-low">
  Guthabensaldo ist zu niedrig
</h3>

Ihre Console-Organisation hat die vorausbezahlten Guthaben aufgebraucht.

```text theme={null}
Credit balance is too low
```

**Was Sie tun können:**

* Fügen Sie Guthaben unter [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing) hinzu, und erwägen Sie, dort das automatische Nachladen zu aktivieren, damit der Saldo aufgefüllt wird, bevor er null erreicht
* Wechseln Sie mit `/login` zur Abonnement-Authentifizierung, wenn Sie einen Pro-, Max-, Team- oder Enterprise-Plan haben
* Legen Sie Pro-Workspace-Ausgabenlimits in der Console fest, um zu verhindern, dass ein einzelnes Projekt das Organisationsguthaben aufbraucht. Siehe [Kosten effektiv verwalten](/docs/de/costs).

<h2 id="authentication-errors">
  Authentifizierungsfehler
</h2>

Diese Fehler bedeuten, dass Claude Code nicht nachweisen kann, wer Sie gegenüber der API sind. Führen Sie jederzeit `/status` aus, um zu sehen, welche Anmeldedaten derzeit aktiv sind.

<h3 id="not-logged-in">
  Nicht angemeldet
</h3>

Für diese Sitzung ist keine gültige Anmeldedaten verfügbar.

```text theme={null}
Not logged in · Please run /login
```

**Was zu tun ist:**

* Führen Sie `/login` aus, um sich mit Ihrem Claude-Abonnement oder Ihrem Console-Konto zu authentifizieren
* Wenn Sie erwartet haben, dass eine Umgebungsvariable Sie authentifiziert, bestätigen Sie, dass `ANTHROPIC_API_KEY` in der Shell, in der Sie `claude` gestartet haben, gesetzt und exportiert ist
* Für CI oder Automatisierung, bei der interaktive Anmeldung nicht möglich ist, konfigurieren Sie ein [`apiKeyHelper`](/docs/de/settings#available-settings)-Skript, das beim Start einen Schlüssel abruft
* Siehe [Authentifizierungspriorität](/docs/de/authentication#authentication-precedence), um zu verstehen, welche Anmeldedaten Claude Code verwendet, wenn mehrere vorhanden sind

Wenn Sie wiederholt zur Anmeldung aufgefordert werden, siehe [Nicht angemeldet oder Token abgelaufen](/docs/de/troubleshoot-install#not-logged-in-or-token-expired) für Systemuhr- und macOS-Keychain-Korrektionen.

<h3 id="could-not-resolve-authentication-method">
  Authentifizierungsmethode konnte nicht aufgelöst werden
</h3>

Die Sitzung erreichte den API-Client ohne Anmeldedaten. Dies erscheint in [Hintergrundsitzungen](/docs/de/agent-view), Cloud-Sitzungen und Agent-SDK-Kontexten, bei denen die interaktive Anmeldungsprüfung nicht vor der ersten Anfrage ausgeführt wird.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Vor v2.1.174 konnte eine Hintergrund- oder Cloud-Sitzung, die einem untätigen vorinitialisiertem Worker zugewiesen war, auf diese Weise fehlschlagen, selbst wenn gültige Anmeldedaten konfiguriert waren. Führen Sie ein Upgrade durch, um die Funktion wiederherzustellen. In aktuellen Versionen bedeutet der Fehler, dass dem Worker-Prozess keine Anmeldedaten verfügbar waren.

**Was zu tun ist:**

* Führen Sie ein Upgrade auf v2.1.174 oder später durch, wenn dies in einer Hintergrund- oder Cloud-Sitzung angezeigt wird und Ihre Anmeldedaten bereits konfiguriert sind
* Bestätigen Sie, dass `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` oder Ihre Cloud-Provider-Anmeldedaten in der Umgebung gesetzt sind, die den Worker startet, nicht nur in Ihrer interaktiven Shell
* Für das Agent SDK siehe [Authentifizierungseinrichtung](/docs/de/agent-sdk/overview#get-started)
* Führen Sie `/status` in einer interaktiven Sitzung in derselben Umgebung aus, um zu bestätigen, welche Anmeldedatenquelle aufgelöst wird

<h3 id="invalid-api-key">
  Ungültiger API-Schlüssel
</h3>

Die Umgebungsvariable `ANTHROPIC_API_KEY` oder das `apiKeyHelper`-Skript hat einen Schlüssel zurückgegeben, den die API abgelehnt hat.

```text theme={null}
Invalid API key · Fix external API key
```

**Was zu tun ist:**

* Überprüfen Sie auf Tippfehler und bestätigen Sie, dass der Schlüssel nicht in der [Console](https://platform.claude.com/settings/keys) widerrufen wurde
* Führen Sie `env | grep ANTHROPIC` in derselben Shell aus. Tools wie direnv, dotenv-Shell-Plugins und IDE-Terminals können einen veralteten Schlüssel aus einer `.env`-Datei in Ihrem Projekt laden, ohne dass Sie ihn explizit setzen
* Heben Sie `ANTHROPIC_API_KEY` auf und führen Sie `/login` aus, um stattdessen Abonnement-Authentifizierung zu verwenden
* Wenn der Schlüssel von einem [`apiKeyHelper`](/docs/de/settings#available-settings)-Skript stammt, führen Sie das Skript direkt aus, um zu bestätigen, dass es einen gültigen Schlüssel auf stdout ausgibt
* Führen Sie `/status` aus, um zu bestätigen, welche Anmeldedatenquelle Claude Code tatsächlich verwendet

<h3 id="your-apikeyhelper-script-is-failing">
  Ihr apiKeyHelper-Skript schlägt fehl
</h3>

Der in der [`apiKeyHelper`](/docs/de/settings#available-settings)-Einstellung konfigurierte Befehl ist mit einem Fehler beendet worden, hat das Zeitlimit überschritten oder hat nichts auf stdout ausgegeben. Ohne einen Schlüssel vom Skript erreicht die Anfrage die API mit einer Platzhalter-Anmeldedaten, und die API lehnt sie mit `401` ab.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code führt das Skript erneut aus und versucht die Anfrage bis zu zwei weitere Male, bevor diese Meldung angezeigt wird, daher wird der Fehler innerhalb von drei Versuchen angezeigt. Vor v2.1.208 verbrauchte Claude Code das gesamte [Wiederholungsbudget](#automatic-retries) beim erneuten Senden der Anfrage mit der Platzhalter-Anmeldedaten und meldete dann einen generischen `401`-Authentifizierungsfehler anstelle des Skriptfehlers.

Das Ausführen von `/login` hilft hier nicht: die Ausgabe des Helfers [hat Vorrang](/docs/de/authentication#authentication-precedence) vor einer gespeicherten Anmeldung, solange die Einstellung vorhanden ist.

**Was zu tun ist:**

* Führen Sie den in `apiKeyHelper` konfigurierten Befehl direkt in Ihrer Shell aus, um den Fehler zu reproduzieren
* Wenn der Befehl eine abgelaufene Sitzung meldet, authentifizieren Sie sich erneut bei Ihrem Anmeldedaten-Anbieter, z. B. indem Sie sich erneut bei Ihrem SSO oder Ihrem Secrets-Tresor anmelden
* Beheben Sie den Befehl so, dass er den Schlüssel auf stdout ausgibt und mit Code 0 beendet wird. Siehe [Anmeldedaten mit apiKeyHelper rotieren](/docs/de/llm-gateway-connect#rotate-credentials-with-apikeyhelper) für ein funktionierendes Setup.
* Führen Sie `/status` aus, um zu bestätigen, dass `apiKeyHelper` die aktive Anmeldedatenquelle ist. Jedes Mal, wenn der Befehl fehlschlägt, erscheinen sein Exit-Code und die Fehlerausgabe in einem `Cloud authentication`-Panel im Terminal.

<h3 id="this-organization-has-been-disabled">
  Diese Organisation wurde deaktiviert
</h3>

Ein veralteter `ANTHROPIC_API_KEY` von einer deaktivierten Console-Organisation überschreibt Ihre Abonnement-Anmeldung.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

Umgebungsvariablen haben Vorrang vor `/login`, daher wird ein Schlüssel, der in Ihrem Shell-Profil exportiert oder aus einer `.env`-Datei geladen wird, verwendet, selbst wenn Sie ein funktionierendes Pro- oder Max-Abonnement haben. Im nicht-interaktiven Modus (`-p`) wird der Schlüssel immer verwendet, wenn er vorhanden ist.

**Was zu tun ist:**

* Heben Sie `ANTHROPIC_API_KEY` in der aktuellen Shell auf und entfernen Sie es aus Ihrem Shell-Profil, dann starten Sie `claude` neu
* Führen Sie danach `/status` aus, um zu bestätigen, dass die aktive Anmeldedaten Ihr Abonnement sind
* Wenn keine Umgebungsvariable gesetzt ist und der Fehler weiterhin besteht, ist die deaktivierte Organisation diejenige, die mit Ihrem `/login` verknüpft ist. Kontaktieren Sie den Support oder melden Sie sich mit einem anderen Konto an.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Ihre Organisation hat die API-Schlüssel-Authentifizierung deaktiviert
</h3>

Diese Meldung erfordert Claude Code v2.1.169 oder später. Der Administrator Ihrer Console-Organisation hat die API-Schlüssel-Authentifizierung deaktiviert, daher lehnt die API den Schlüssel ab, den Claude Code sendet. Der Wiederherstellungshinweis nach dem `·` variiert je nachdem, woher der Schlüssel stammt:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Umgebungsvariablen und `apiKeyHelper` haben Vorrang vor `/login`, daher hilft das alleinige Ausführen von `/login` nicht, während einer von ihnen noch einen Schlüssel bereitstellt. Siehe [Authentifizierungspriorität](/docs/de/authentication#authentication-precedence).

**Was zu tun ist:**

* Wenn die Meldung `ANTHROPIC_API_KEY` nennt, heben Sie es in der aktuellen Shell auf und entfernen Sie es aus Ihrem Shell-Profil oder `.env`-Datei, dann starten Sie `claude` neu
* Wenn die Meldung `apiKeyHelper` nennt, entfernen Sie die [`apiKeyHelper`](/docs/de/settings#available-settings)-Einstellung aus Ihrer `settings.json`
* Führen Sie `/login` aus, um sich mit Ihrem claude.ai-Konto anzumelden
* Führen Sie danach `/status` aus, um zu bestätigen, dass die aktive Anmeldedaten Ihr Abonnement und nicht ein API-Schlüssel sind
* Wenn Sie API-Schlüssel-Authentifizierung für Automatisierung benötigen, bitten Sie Ihren Organisations-Administrator, sie in der Console erneut zu aktivieren

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Ihre Organisation hat den Claude-Abonnementzugriff deaktiviert
</h3>

Ihre Claude-Organisation erlaubt nicht, sich bei Claude Code mit einer Abonnement-Anmeldung anzumelden. Das erneute Ausführen von `/login` mit demselben Konto gibt denselben Fehler zurück.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Dies ist eine serverseitige Organisationseinstellung, daher kann sie nicht durch lokale Einstellungen, Umgebungsvariablen oder CLI-Flags überschrieben werden.

Das Agent SDK und der `-p` nicht-interaktive Modus zeigen dies als `oauth_org_not_allowed`-Fehlercode an.

**Was zu tun ist:**

* Bitten Sie Ihren Administrator, den Claude-Code-Zugriff für Ihre Organisation zu aktivieren
* Authentifizieren Sie sich stattdessen mit einem Console-API-Schlüssel anstelle Ihres Abonnements. Siehe [Claude-Console-Authentifizierung](/docs/de/authentication#claude-console-authentication) für die Einrichtung.
* Wenn Sie der Administrator sind und keine Option zum Aktivieren des Zugriffs sehen, kontaktieren Sie den [Anthropic-Support](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Routinen sind durch die Richtlinie Ihrer Organisation deaktiviert
</h3>

Ein Eigentümer in Ihrer Team- oder Enterprise-Organisation hat Routinen auf Organisationsebene deaktiviert. Der Fehler erscheint, wenn Sie versuchen, eine Routine zu erstellen oder auszuführen, einschließlich von `/schedule` und der [Routinen](/docs/de/routines)-Benutzeroberfläche auf claude.ai/code.

```text theme={null}
Routines are disabled by your organization's policy.
```

Dies ist eine serverseitige Einstellung, daher kann sie nicht durch lokale Einstellungen, Umgebungsvariablen oder CLI-Flags überschrieben werden.

**Was zu tun ist:**

* Bitten Sie einen Eigentümer in Ihrer Organisation, den **Routinen**-Schalter unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) zu aktivieren
* Für einmalige geplante Arbeiten, die keine Routinen auf Organisationsebene erfordern, siehe [geplante Aufgaben](/docs/de/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control erfordert die Anthropic API
</h3>

Die Sitzung spricht nicht direkt mit der Anthropic API, daher gibt es kein claude.ai-Backend für [Remote Control](/docs/de/remote-control) zum Koppeln.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

Dies erscheint auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry. Ab v2.1.196 erscheint es auch, wenn [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) auf einen anderen Host als `api.anthropic.com` verweist, z. B. ein [LLM-Gateway](/docs/de/llm-gateway) oder Proxy, selbst wenn Sie sich mit claude.ai anmelden.

**Was zu tun ist:**

* Heben Sie `ANTHROPIC_BASE_URL` auf und starten Sie die Sitzung neu, oder starten Sie Remote Control von einer Sitzung aus, die direkt mit der Anthropic API spricht
* Für diese und die anderen Remote-Control-Startmeldungen siehe [Remote Control beheben](/docs/de/remote-control#troubleshooting)

<h3 id="oauth-token-revoked-or-expired">
  OAuth-Token widerrufen oder abgelaufen
</h3>

Ihre gespeicherte Anmeldung ist nicht mehr gültig. Ein widerrufenes Token bedeutet, dass Sie sich überall abgemeldet haben oder ein Administrator den Zugriff entfernt hat; ein abgelaufenes Token bedeutet, dass die automatische Aktualisierung während der Sitzung fehlgeschlagen ist.

Beide Meldungen melden eine Ablehnung, die die API für eine Anfrage zurückgegeben hat, die Claude Code gesendet hat. Wenn die gespeicherte Anmeldung bereits nach einer fehlgeschlagenen Aktualisierung gelöscht wurde, sehen Sie stattdessen [Anmeldung abgelaufen](#login-expired).

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**Was zu tun ist:**

* Führen Sie `/login` aus, um sich erneut anzumelden
* Wenn der Fehler nach der erneuten Authentifizierung innerhalb derselben Sitzung zurückkommt, führen Sie zuerst `/logout` aus, um das gespeicherte Token vollständig zu löschen, dann `/login`
* Für wiederholte Anmeldungsaufforderungen über Starts hinweg siehe die Systemuhr- und macOS-Keychain-Prüfungen in [Fehlerbehebung](/docs/de/troubleshoot-install#not-logged-in-or-token-expired)
* Für andere Fehler einschließlich `403 Forbidden` und OAuth-Browser-Probleme siehe [Anmeldung und Authentifizierung](/docs/de/troubleshoot-install#login-and-authentication)

<h3 id="login-expired">
  Anmeldung abgelaufen
</h3>

Claude Code versuchte, Ihre gespeicherte claude.ai- oder Claude-Console-Anmeldung zu erneuern, und der OAuth-Dienst lehnte das gespeicherte Aktualisierungs-Token ab, daher löschte Claude Code die gespeicherten Anmeldedaten. Danach stoppt jede Anfrage lokal, bevor sie die API erreicht, da nur `/login` neue Anmeldedaten erstellen kann. Vor v2.1.206 sendete Claude Code die Anfrage trotzdem mit allen verbleibenden Anmeldedaten in der Umgebung, und dann schlugen alle Modelle mit [Es gibt ein Problem mit dem ausgewählten Modell](#theres-an-issue-with-the-selected-model) oder einem 401 anstelle einer Aufforderung zur Anmeldung fehl.

```text theme={null}
Login expired · Please run /login
```

Im [nicht-interaktiven Modus](/docs/de/headless) (`-p`) und dem [Agent SDK](/docs/de/agent-sdk/overview) lautet die Meldung wie folgt, und der strukturierte Fehlercode ist `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Dies ist nicht derselbe Zustand wie [OAuth-Token widerrufen oder abgelaufen](#oauth-token-revoked-or-expired). Diese Meldungen melden einen 401, den die API zurückgegeben hat. Claude Code selbst erzeugt `Login expired` für eine Anmeldung, die es bereits nicht erneuern konnte, daher sendet es keine Anfrage.

Sitzungen, die mit einem API-Schlüssel, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/de/env-vars) oder einem Drittanbieter authentifiziert sind, verwenden nicht die gespeicherte Anmeldung und sehen diese Meldung nie.

**Was zu tun ist:**

* Führen Sie `/login` aus, um sich erneut anzumelden. Das erneute Versuchen ohne Anmeldung zeigt dieselbe Meldung bei jeder Anfrage.
* Im nicht-interaktiven Modus führen Sie `claude` in derselben Umgebung aus, schließen Sie `/login` ab, dann führen Sie Ihren Befehl erneut aus. Für Automatisierung, die sich nicht interaktiv anmelden kann, authentifizieren Sie sich mit `ANTHROPIC_API_KEY` oder [generieren Sie ein langlebiges Token mit `claude setup-token`](/docs/de/authentication#generate-a-long-lived-token).
* Wenn die Anmeldung weiterhin fehlschlägt, siehe [Anmeldung und Authentifizierung](/docs/de/troubleshoot-install#login-and-authentication)

<h3 id="oauth-scope-requirement">
  OAuth-Bereichsanforderung
</h3>

Das gespeicherte Token stammt von vor einem Berechtigungsbereich, den eine neuere Funktion benötigt. Sie sehen dies am häufigsten von `/usage` und dem Nutzungsindikator in der Statuszeile:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Was zu tun ist:**

* Führen Sie `/login` aus, um ein neues Token mit den aktuellen Bereichen zu erhalten. Sie müssen sich nicht zuerst abmelden.

<h3 id="aws-credentials-expired-or-invalid">
  AWS-Anmeldedaten abgelaufen oder ungültig
</h3>

Diese Meldung erfordert Claude Code v2.1.198 oder später und erscheint nur, wenn [`awsAuthRefresh`](/docs/de/amazon-bedrock#advanced-credential-configuration) in Ihrer Einstellungsdatei gesetzt ist. Ihr AWS-Sitzungs-Token ist abgelaufen oder wurde abgelehnt, und die automatische Aktualisierung, die Claude Code bereits ausgeführt hat, hat keine Anmeldedaten erzeugt, die die API akzeptiert. Es erscheint bei einem 401 von [Claude Platform on AWS](/docs/de/claude-platform-on-aws) oder dem [Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint), wie diese Anbieter ein abgelaufenes Sicherheits-Token melden.

Der Aktionshinweis in der Mitte nennt den `awsAuthRefresh`-Befehl aus Ihren Einstellungen, daher variiert er. Der stabile Teil ist der führende `AWS credentials expired or invalid`:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Ohne `awsAuthRefresh` konfiguriert, zeigt derselbe 401 stattdessen die generische `Please run /login`-Meldung an, die AWS-Anmeldedaten nicht aktualisieren kann.

**Was zu tun ist:**

* Führen Sie den in der Meldung genannten `awsAuthRefresh`-Befehl aus, z. B. `aws sso login --profile myprofile`, in einem anderen Terminal aus und schließen Sie die Browser-Anmeldung ab, dann versuchen Sie es erneut
* Führen Sie in einer interaktiven Sitzung `/login` aus, wählen Sie **3rd-party platform**, dann wählen Sie **Claude Platform on AWS · refresh credentials** unter **Using 3rd-party platforms**, um denselben Befehl auszuführen, ohne Claude Code neu zu starten. Siehe [AWS-Anmeldedaten konfigurieren](/docs/de/claude-platform-on-aws#1-configure-aws-credentials)
* Wenn der Fehler nach dem erfolgreichen Aktualisierungsbefehl wiederholt wird, bestätigen Sie, dass die Identität außerhalb von Claude Code mit `aws sts get-caller-identity` in derselben Shell und demselben Profil gültig ist

<h3 id="aws-authentication-failed">
  AWS-Authentifizierung fehlgeschlagen
</h3>

Diese Meldung erfordert Claude Code v2.1.198 oder später und erscheint nur, wenn [`awsAuthRefresh`](/docs/de/amazon-bedrock#advanced-credential-configuration) in Ihrer Einstellungsdatei gesetzt ist. Ihr AWS-Anbieter hat einen 403 zurückgegeben, oder [Amazon Bedrock](/docs/de/amazon-bedrock) hat einen 401 zurückgegeben.

Claude Code kann nicht sagen, welche Ursache Sie getroffen haben. Amazon Bedrock meldet ein abgelaufenes Sicherheits-Token als 403, aber ein 403 ist auch, wie es eine Autorisierungsverweigerung meldet, z. B. eine `AccessDeniedException` von einer fehlenden IAM-Berechtigung oder ein Modell, das nicht für Ihr Konto aktiviert ist.

Ein 401 von Amazon Bedrock landet hier auch anstelle von [AWS-Anmeldedaten abgelaufen oder ungültig](#aws-credentials-expired-or-invalid), da Amazon Bedrock ein abgelaufenes Token nicht als 401 meldet. Ein 401 von diesem Endpunkt kommt normalerweise von etwas anderem im Anfragepfad, z. B. einem Unternehmens-Proxy.

Eine Anmeldedaten-Aktualisierung behebt ein abgelaufenes Token und kann die anderen Ursachen nicht beheben, daher bietet die Meldung beide an:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

Der Aktionshinweis in der Mitte nennt den `awsAuthRefresh`-Befehl aus Ihren Einstellungen, daher variiert er. Der stabile Teil ist der führende `AWS authentication failed`.

**Was zu tun ist:**

* Führen Sie den in der Meldung genannten `awsAuthRefresh`-Befehl oder `aws sso login` aus, falls ein abgelaufenes Anmeldedaten die Ursache ist
* Wenn Ihre Anmeldedaten aktuell sind, bestätigen Sie, dass die IAM-Berechtigungen in [IAM-Konfiguration](/docs/de/amazon-bedrock#iam-configuration) an die Identität angehängt sind, die Sie verwenden, und dass das ausgewählte Modell für Ihr Konto und Ihre Region aktiviert ist
* Führen Sie `aws sts get-caller-identity` aus, um zu bestätigen, welche Identität Ihre Anfragen verwenden; ein veraltetes `AWS_PROFILE` oder Standardprofil ist eine häufige Ursache für einen Berechtigungskonflikt

<h3 id="aws-default-chain-credential-resolve-timed-out">
  AWS-Standard-Chain-Anmeldedaten-Auflösung hat das Zeitlimit überschritten
</h3>

Der AWS-Standard-Anmeldedaten-Provider-Chain hat keine Anmeldedaten innerhalb von 60 Sekunden erzeugt, daher stoppte Claude Code die Auflösung und schlugen die Anfrage fehl. Der Fehler ist lokale Anmeldedaten-Auflösung: die Anfrage erreichte nie [Amazon Bedrock](/docs/de/amazon-bedrock), [Claude Platform on AWS](/docs/de/claude-platform-on-aws) oder den [Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint). Claude Code löscht seinen [Anmeldedaten-Cache](/docs/de/amazon-bedrock#credential-caching-and-resolution-timeout) und versucht erneut, bevor dieser Fehler angezeigt wird, daher hat die Chain bei wiederholten Versuchen stagniert, wenn Sie ihn sehen.

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

Häufige Ursachen sind ein `credential_process`-Befehl in Ihrem AWS-Profil, der auf Eingaben wartet, die er nicht erhalten kann, und ein Container oder eine VM, deren Instance-Metadaten-Dienst (IMDS) nie auf die Sonde der Chain antwortet. Vor v2.1.207 ließ eine stagnierte Chain die Anfrage auf unbestimmte Zeit warten, anstatt mit dieser Meldung fehlzuschlagen.

**Was zu tun ist:**

* Führen Sie `aws sts get-caller-identity` in derselben Shell mit demselben `AWS_PROFILE` aus. Wenn es auch hängt, beheben Sie das Profil; ein `credential_process`-Befehl, der interaktiv auffordert, ist eine häufige Ursache.
* Schließen Sie den Anmeldeschritt ab, bevor Sie Claude Code starten, z. B. `aws sso login --profile myprofile`, damit die Chain aus dem lokalen SSO-Cache aufgelöst wird, anstatt auf einen Browser-Flow zu warten
* Wenn Ihre Chain eine interaktive Anmeldung ausführt, die legitim mehr als 60 Sekunden benötigt, z. B. SSO mit MFA durch einen Wrapper wie `aws-vault`, erhöhen Sie das Limit in Millisekunden mit [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/de/env-vars)

<h2 id="network-and-connection-errors">
  Netzwerk- und Verbindungsfehler
</h2>

Diese Fehler bedeuten, dass eine Netzwerkanfrage von Claude Code ihr Ziel nicht erreicht hat, oder etwas zwischen Claude Code und der API hat die Antwort auf dem Rückweg verändert. Sie entstehen normalerweise in Ihrem lokalen Netzwerk, Proxy oder Firewall oder in der Netzwerkrichtlinie der Cloud-Umgebung.

<h3 id="unable-to-connect-to-api">
  Verbindung zur API nicht möglich
</h3>

Die TCP-Verbindung zur API ist fehlgeschlagen oder wurde nie abgeschlossen.

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

Häufige Ursachen sind fehlender Internetzugang, ein VPN, das `api.anthropic.com` blockiert, oder ein erforderlicher Unternehmens-Proxy, der nicht konfiguriert ist.

**Was zu tun ist:**

* Bestätigen Sie, dass Sie den API-Host aus derselben Shell erreichen können, indem Sie `curl -I https://api.anthropic.com` ausführen. Verwenden Sie unter Windows PowerShell `curl.exe -I https://api.anthropic.com`, damit der integrierte `Invoke-WebRequest`-Alias nicht verwendet wird.
* Wenn Sie sich hinter einem Unternehmens-Proxy befinden, setzen Sie `HTTPS_PROXY` vor dem Starten von Claude Code und siehe [Netzwerkkonfiguration](/docs/de/network-config)
* Wenn Sie über ein LLM-Gateway oder Relay weiterleiten, setzen Sie [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) auf dessen Adresse. Siehe [Claude Code mit einem LLM-Gateway verbinden](/docs/de/llm-gateway-connect) für die Einrichtung.
* Stellen Sie sicher, dass Ihre Firewall die in [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements) aufgelisteten Hosts zulässt
* Vorübergehende Fehler werden [automatisch wiederholt](#automatic-retries); anhaltende Fehler deuten auf ein lokales Netzwerkproblem hin

Wenn `curl` erfolgreich ist, aber Claude Code immer noch fehlschlägt, liegt die Ursache normalerweise bei etwas zwischen der Laufzeit und dem Netzwerk und nicht beim Netzwerk selbst:

* Unter Linux und WSL überprüfen Sie `/etc/resolv.conf` auf einen unerreichbaren Nameserver. WSL kann insbesondere einen fehlerhaften Resolver vom Host erben.
* Unter macOS kann ein VPN-Client, der getrennt oder deinstalliert wurde, eine Tunnel-Schnittstelle oder Routing-Regel hinterlassen. Überprüfen Sie `ifconfig` auf veraltete `utun`-Schnittstellen und entfernen Sie die Netzwerkerweiterung des VPN in den Systemeinstellungen.
* Docker Desktop und ähnliche Container-Laufzeiten können ausgehenden Datenverkehr abfangen. Beenden Sie diese und versuchen Sie es erneut, um dies auszuschließen.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock-Streaming-Antwort hat einen unerwarteten Content-Type
</h3>

Ein Gateway oder Proxy zwischen Claude Code und [Amazon Bedrock](/docs/de/amazon-bedrock) transformiert den Streaming-Antwortkörper oder seinen `Content-Type`-Header. Amazon Bedrock streamt Antworten als `application/vnd.amazon.eventstream`, und Claude Code lehnt eine erfolgreiche Streaming-Antwort ab, die einen anderen Content-Type meldet, anstatt einen Körper zu dekodieren, den es nicht lesen kann. Die Anfrage wird nicht wiederholt.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Vor v2.1.208 zeigte sich dieselbe Fehlkonfiguration als `API Error: Truncated event message received`, nachdem die gesamte Antwort gepuffert worden war.

**Was zu tun ist:**

* Konfigurieren Sie das Gateway so, dass der `InvokeModelWithResponseStream`-Antwortkörper und sein `Content-Type`-Header unverändert weitergeleitet werden. Ein Vermittler, der den Stream als Server-Sent Events erneut aussendet, ist eine häufige Ursache.
* Wenn das Gateway nur den Header umschreibt und den binären Körper intakt weitergeleitet, setzen Sie [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/de/env-vars), um die Überprüfung zu überspringen, bis das Gateway behoben ist. Siehe [Streaming-Fehler hinter einem Gateway oder Proxy](/docs/de/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  SSL-Zertifikatsfehler
</h3>

Ein Proxy oder eine Sicherheitsappliance in Ihrem Netzwerk fängt TLS-Datenverkehr mit seinem eigenen Zertifikat ab, und Claude Code vertraut ihm nicht.

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

Ab v2.1.199 wird ein Zertifikatvalidierungsfehler nicht wiederholt, daher wird dieser Fehler beim ersten Versuch angezeigt, anstatt nach dem vollständigen [Wiederholungsbudget](#automatic-retries). Frühere Versionen verbrachten einige Minuten mit Wiederholungen, bevor sie angezeigt wurden. Vorübergehende TLS-Bedingungen, wie z. B. ein Handshake-Timeout, werden weiterhin wiederholt.

Während `/login` und der Startup-Konnektivitätsprüfung wird derselbe Fehler mit dem OpenSSL-Code und der Behebung inline gemeldet:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

**Was zu tun ist:**

* Exportieren Sie das CA-Bundle Ihrer Organisation und verweisen Sie Claude Code darauf mit `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Siehe [Netzwerkkonfiguration](/docs/de/network-config#custom-ca-certificates) für vollständige Einrichtungsanweisungen
* Setzen Sie nicht `NODE_TLS_REJECT_UNAUTHORIZED=0`, was die Zertifikatvalidierung vollständig deaktiviert

<h3 id="host-not-allowed-in-a-cloud-session">
  Host nicht zulässig in einer Cloud-Sitzung
</h3>

Eine ausgehende HTTP-Anfrage von einer Cloud-Sitzung oder Routine wurde durch die Netzwerkrichtlinie der Umgebung blockiert.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Möglicherweise wird auch ein TLS-Zertifikat angezeigt, das nicht dem echten Zertifikat des Ziels entspricht. Die Cloud-Umgebung leitet ausgehenden Datenverkehr durch einen Proxy weiter, der die Netzwerkrichtlinie durchsetzt, daher bedeutet ein nicht übereinstimmendes Zertifikat, dass der Proxy die Verbindung beendet hat, nicht das Ziel.

Dies ist kein clientseitiges Netzwerkproblem. Cloud-Sitzungen und [Routinen](/docs/de/routines) werden in einer Sandbox-Umgebung ausgeführt, deren ausgehender Datenverkehr auf die Zulassungsliste der Umgebung gefiltert wird. Die **Standard**-Umgebung verwendet **Vertrauenswürdigen** Zugriff, der die [Standard-Zulassungsliste](/docs/de/claude-code-on-the-web#default-allowed-domains) von Paketregistern, Cloud-Provider-APIs, Container-Registern und häufigen Entwicklungsdomänen zulässt, aber alles andere blockiert.

**Was zu tun ist:**

* Öffnen Sie die Routine zum Bearbeiten oder starten Sie eine Cloud-Sitzung. Wählen Sie das Cloud-Symbol mit dem Namen Ihrer Umgebung, z. B. **Standard**, um die Auswahl zu öffnen. Bewegen Sie den Mauszeiger über Ihre Umgebung und klicken Sie auf das Einstellungssymbol.
* Ändern Sie im Dialog **Cloud-Umgebung aktualisieren** den **Netzwerkzugriff** von **Vertrauenswürdig** zu **Benutzerdefiniert**, und fügen Sie dann die blockierte Domain zu **Zulässige Domains** hinzu. Geben Sie eine Domain pro Zeile ein. Aktivieren Sie **Auch Standard-Liste häufiger Paketmanager einschließen**, um die [Standard-Zulassungsliste](/docs/de/claude-code-on-the-web#default-allowed-domains) neben Ihren benutzerdefinierten Domains zu behalten. Wählen Sie stattdessen **Vollständig**, wenn Sie uneingeschränkten Zugriff möchten.
* Klicken Sie auf **Änderungen speichern**. Die nächste Ausführung verwendet die aktualisierte Zulassungsliste.

Siehe [Netzwerkzugriff](/docs/de/claude-code-on-the-web#network-access) für Zugriffsstufen und die Standard-Zulassungsliste. Lokale CLI-Sitzungen sind von dieser Richtlinie nicht betroffen.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Verbindung zu Ihrer Remote-Control-Sitzung konnte nicht wiederhergestellt werden
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Das Fortsetzen mit `claude --resume` oder `claude --continue` stellt die Verbindung zur [Remote Control](/docs/de/remote-control)-Sitzung wieder her, die in diesem Gespräch aufgezeichnet wurde. Diese Meldung bedeutet, dass die Wiederverbindung aus einem Grund fehlgeschlagen ist, der vorübergehend sein kann, z. B. eine Netzwerkunterbrechung oder ein Serverfehler, daher kann Claude Code nicht bestätigen, ob die Remote-Sitzung noch vorhanden ist. Ihre lokale Sitzung wird ohne Remote Control weiterhin ausgeführt.

**Was zu tun ist:**

* Führen Sie `/remote-control` aus, um die Verbindung erneut zu versuchen
* Starten Sie Claude Code ohne `--resume`, um eine neue Remote-Control-Sitzung zu erstellen
* Weitere Remote-Control-Startmeldungen finden Sie unter [Remote Control beheben](/docs/de/remote-control#troubleshooting)

Diese Meldung wird nicht angezeigt, wenn der Server bestätigt, dass die vorherige Sitzung nicht mehr vorhanden ist. Claude Code erstellt in diesem Fall eine neue. Vor v2.1.200 hat jeder Wiederverbindungsfehler eine neue Remote-Control-Sitzung erstellt, was zusätzliche Sitzungen in der Sitzungsliste unter claude.ai/code hinterlassen hat.

<h2 id="request-errors">
  Anfragefehler
</h2>

Diese Fehler beziehen sich auf den Inhalt Ihrer Anfrage. Die meisten werden von der API zurückgegeben, nachdem sie die Anfrage abgelehnt hat; einige werden lokal von Claude Code erzeugt, bevor eine Anfrage gesendet wird.

<h3 id="prompt-is-too-long">
  Eingabeaufforderung ist zu lang
</h3>

Das Gespräch plus angehängte Dateien überschreitet das Kontextfenster des Modells.

```text theme={null}
Prompt is too long
```

**Was zu tun ist:**

* Führen Sie `/compact` aus, um frühere Turns zusammenzufassen und Platz freizugeben, oder `/clear`, um neu zu beginnen
* Führen Sie `/context` aus, um eine Aufschlüsselung zu sehen, was das Fenster verbraucht: Systemaufforderung, Tools, Speicherdateien und Nachrichten
* Deaktivieren Sie MCP-Server, die Sie nicht verwenden, mit `/mcp disable <name>`, um ihre Tool-Definitionen aus dem Kontext zu entfernen
* Trimmen Sie große `CLAUDE.md`-Speicherdateien, oder verschieben Sie Anweisungen in [pfadgebundene Regeln](/docs/de/memory#path-specific-rules), die nur bei Bedarf geladen werden
* Subagenten erben jede MCP-Tool-Definition von der übergeordneten Sitzung, was ihr Kontextfenster füllen kann, bevor der erste Turn stattfindet. Deaktivieren Sie MCP-Server, die Sie nicht verwenden, bevor Sie Subagenten spawnen.
* Auto-Compact ist standardmäßig aktiviert und verhindert normalerweise diesen Fehler. Wenn Sie [`DISABLE_AUTO_COMPACT`](/docs/de/env-vars) gesetzt haben, aktivieren Sie es erneut oder führen Sie `/compact` manuell aus, bevor das Fenster voll wird.

Siehe [Erkunden Sie das Kontextfenster](/docs/de/context-window) für eine interaktive Ansicht, wie sich der Kontext füllt.

<h3 id="error-during-compaction-conversation-too-long">
  Fehler während der Komprimierung: Gespräch zu lang
</h3>

`/compact` selbst ist fehlgeschlagen, weil nicht genug freier Kontext vorhanden ist, um die erzeugte Zusammenfassung zu halten.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Dies kann passieren, wenn das Fenster bereits voll ist, wenn Auto-Compact ausgelöst wird, oder wenn Sie `/compact` ausführen, nachdem Sie `Prompt is too long` gesehen haben.

**Was zu tun ist:**

* Drücken Sie Esc zweimal, um die Nachrichtenliste zu öffnen und mehrere Turns zurückzugehen. Dies entfernt die neuesten Nachrichten aus dem Kontext. Führen Sie dann `/compact` erneut aus.
* Wenn das Zurückgehen nicht genug Platz freimacht, führen Sie `/clear` aus, um eine neue Sitzung zu starten. Ihr vorheriges Gespräch bleibt erhalten und kann mit `/resume` erneut geöffnet werden.

<h3 id="request-too-large">
  Anfrage zu groß
</h3>

Der rohe Anfragekörper überschritt das Byte-Limit der API vor der Tokenisierung, normalerweise wegen einer großen eingefügten Datei oder eines Anhangs.

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

Dies ist ein Größenlimit für die HTTP-Anfrage, getrennt vom [Kontextfenster-Limit](#prompt-is-too-long).

**Was zu tun ist:**

* Drücken Sie Esc zweimal und gehen Sie zurück zum Turn, der den übergroßen Inhalt hinzugefügt hat
* Referenzieren Sie große Dateien nach Pfad, anstatt ihren Inhalt einzufügen, damit Claude sie in Chunks lesen kann
* Für Bilder siehe [Bild war zu groß](#image-was-too-large) unten

<h3 id="image-was-too-large">
  Bild war zu groß
</h3>

Ein eingefügtes oder angehängtes Bild überschreitet die Größen- oder Dimensionslimits der API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code ersetzt das nicht verarbeitbare Bild durch einen Textplatzhalter und versucht es erneut, sodass nachfolgende Nachrichten erfolgreich sind. In Versionen vor 2.1.142 konnte ein eingefügtes Bild im Gespräch bleiben und denselben Fehler bei jeder nachfolgenden Nachricht wiederholen. Um sich in diesen Versionen zu erholen, drücken Sie Esc zweimal und gehen Sie zurück zum Turn, in dem das Bild hinzugefügt wurde.

**Was zu tun ist:**

* Ändern Sie die Größe des Bildes vor dem Einfügen. Die API akzeptiert Bilder bis zu 8000 Pixeln auf der längsten Kante für ein einzelnes Bild oder 2000 Pixel, wenn viele Bilder im Kontext sind.
* Machen Sie einen engeren Screenshot des relevanten Bereichs anstelle des gesamten Bildschirms

<h3 id="unable-to-resize-image">
  Bild konnte nicht in der Größe geändert werden
</h3>

Claude Code konnte ein angehängtes Bild nicht herunterskalieren, bevor es an die API gesendet wurde.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code ändert normalerweise die Größe großer Bilder automatisch. Diese Fehler bedeuten, dass der native Bildprozessor nicht geladen werden konnte oder einen Fehler zurückgegeben hat, sodass das Bild nicht an die API-Limits angepasst werden konnte.

**Was zu tun ist:**

* Wenn die Nachricht Sie auffordert, das Bild zu konvertieren, konvertieren Sie es in PNG, JPEG, GIF oder WebP und fügen Sie es erneut an. Claude Code kann Dimensionen für diese Formate ohne den Bildprozessor überprüfen.
* Wenn die Nachricht ein Dimensions- oder Größenlimit meldet, ändern Sie die Größe oder komprimieren Sie das Bild unter diesem Limit, bevor Sie es anhängen.

<h3 id="pdf-errors">
  PDF-Fehler
</h3>

Das angehängte PDF konnte nicht verarbeitet werden.

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**Was zu tun ist:**

* Für übergroße PDFs bitten Sie Claude, einen Seitenbereich mit dem Read-Tool zu lesen, anstatt die gesamte Datei anzuhängen, oder extrahieren Sie Text mit einem Tool wie `pdftotext` und referenzieren Sie die Ausgabedatei nach Pfad
* Für geschützte oder ungültige PDFs entfernen Sie das Passwort oder exportieren Sie die Datei erneut aus ihrer Quellanwendung und versuchen Sie es erneut

<h3 id="extra-inputs-are-not-permitted">
  Zusätzliche Eingaben sind nicht zulässig
</h3>

Ein Proxy oder LLM-Gateway zwischen Claude Code und der API hat den `anthropic-beta`-Anfrage-Header entfernt, sodass die API Felder ablehnte, die davon abhängen.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code sendet Beta-only-Felder wie `context_management`, `effort` und Tool-`input_examples` zusammen mit einem `anthropic-beta`-Header, der sie aktiviert. Wenn ein Gateway den Body weiterleitet, aber den Header entfernt, sieht die API Felder, die sie nicht erkennt.

**Was zu tun ist:**

* Konfigurieren Sie Ihr Gateway, um den `anthropic-beta`-Header weiterzuleiten. Siehe [Feature-Durchleitung](/docs/de/llm-gateway-protocol#feature-pass-through) für das, was Gateways weiterleiten müssen.
* Setzen Sie als Fallback [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/de/env-vars) vor dem Start. Dies deaktiviert Funktionen, die den Beta-Header erfordern, sodass Anfragen durch ein Gateway erfolgreich sind, das ihn nicht weiterleiten kann.

<h3 id="theres-an-issue-with-the-selected-model">
  Es gibt ein Problem mit dem ausgewählten Modell
</h3>

Der konfigurierte Modellname wurde nicht erkannt oder Ihr Konto hat keinen Zugriff darauf. Ab v2.1.160 variiert der nachfolgende Hinweis, der hier in seiner interaktiven Form angezeigt wird, je nach Oberfläche.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Was zu tun ist:**

* **Interaktive CLI**: Führen Sie `/model` aus, um aus Modellen auszuwählen, die für Ihr Konto verfügbar sind.
* **Nicht-interaktiver Modus (`-p`)**: Übergeben Sie `--model` mit einem gültigen Alias oder einer ID, oder setzen Sie [`ANTHROPIC_MODEL`](/docs/de/env-vars). Der Fehlertext zeigt `Run --model` auf dieser Oberfläche.
* **Agent SDK**: Der Fehlertext lässt den Hinweis weg, da das Modell programmgesteuert gesetzt wird. Setzen Sie [`model` auf `Options`](/docs/de/agent-sdk/typescript#options) in TypeScript oder [`ClaudeAgentOptions(model=...)`](/docs/de/agent-sdk/python#claudeagentoptions) in Python, und behandeln Sie den strukturierten `model_not_found`-Fehler, um Ihren eigenen Wiederholungs- oder Modellwähler anzuzeigen.
* Verwenden Sie einen Alias wie `sonnet` oder `opus` anstelle einer vollständigen versionierten ID. Aliase werden zu einem verwalteten Standard aufgelöst, sodass sie nicht veralten. Siehe [Modellkonfiguration](/docs/de/model-config).
* Wenn die falsche Modell immer wieder in der CLI zurückkommt, ist eine veraltete ID irgendwo gesetzt. Überprüfen Sie in [Prioritätsreihenfolge](/docs/de/model-config#setting-your-model): das `--model`-Flag, die `ANTHROPIC_MODEL`-Umgebungsvariable, dann das `model`-Feld in `.claude/settings.local.json`, die `.claude/settings.json` Ihres Projekts und `~/.claude/settings.json`. Entfernen Sie den veralteten Wert und Claude Code fällt auf Ihren Kontostandardwert zurück.
* Claude Code meldet einen abgelaufenen claude.ai-Login als [Login abgelaufen](#login-expired), nicht als dieser Fehler. Vor v2.1.206 schlug ein abgelaufener Login, der nicht mehr aktualisiert werden konnte, bei jedem Modell mit diesem Fehler fehl; führen Sie `/login` aus, wenn Sie das auf einer älteren Version sehen.
* Für Google Cloud's Agent Platform-Bereitstellungen siehe [Google Cloud's Agent Platform-Fehlerbehebung](/docs/de/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Modell ist keine erkannte Modell-ID
</h3>

Die Modellzeichenkette, die Sie an einen Modellwechsel übergeben haben, ist kein Modellalias, keine Modell-ID, die diese Claude Code-Version kennt, oder keine ID, die mit `claude-` beginnt. Die üblichen Ursachen sind ein Tippfehler in der ID, ein Anzeigename wie `Sonnet 5`, wobei die ID `claude-sonnet-5` erwartet wird, oder ein Alias, den nur neuere Claude Code-Versionen erkennen. Claude Code lehnt den Wechsel sofort ab. Vor v2.1.200 speicherte Claude Code die Zeichenkette und schlug beim nächsten Request mit [Es gibt ein Problem mit dem ausgewählten Modell](#theres-an-issue-with-the-selected-model) fehl.

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

Der nachfolgende Hinweis nennt den nächsten passenden Alias oder die Modell-ID. Wenn nichts nah genug ist, lautet es stattdessen `Run /model to see available models.`

Claude Code erzeugt diesen Fehler lokal in dem Moment, in dem der Wechsel angefordert wird, bevor eine API-Anfrage gestellt wird. Er gilt, wenn ein Modell durch die [Agent SDK](/docs/de/agent-sdk/typescript) `setModel()`-Methode oder durch eine App wie die [Desktop-App](/docs/de/desktop) gesetzt wird, die die Claude Code CLI für Sie ausführt.

**Was zu tun ist:**

* Führen Sie `/model` ohne Argument aus, um den Wähler zu öffnen und aus den Modellen auszuwählen, die für Ihr Konto verfügbar sind, dann übergeben Sie den dort angezeigten Alias oder die ID
* Wenn Sie einen Alias verwendet haben, den eine neuere Claude Code-Version unterstützt, führen Sie `claude update` aus. Eine vollständige ID, die mit `claude-` beginnt, besteht diese Überprüfung, auch wenn das Modell neuer ist als Ihre Claude Code-Version, sodass ein Upgrade nicht erforderlich ist.
* Ein Modell, das vor v2.1.200 gespeichert wurde, wird durch diese Überprüfung nicht repariert. Wenn ein veralteter Wert immer wieder zurückkommt, entfernen Sie ihn aus den unter [Es gibt ein Problem mit dem ausgewählten Modell](#theres-an-issue-with-the-selected-model) aufgelisteten Speicherorten.
* Die Überprüfung wird nur auf der Anthropic API ausgeführt. Auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/de/claude-platform-on-aws) und hinter einem [LLM-Gateway](/docs/de/llm-gateway) oder einer benutzerdefinierten `ANTHROPIC_BASE_URL` definiert Ihr Anbieter oder Gateway die Modellnamen, sodass Claude Code jede Zeichenkette akzeptiert und durchleitet.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus ist nicht mit dem Claude Pro-Plan verfügbar
</h3>

Ihr aktiver Abonnementplan beinhaltet nicht das Modell, das Sie ausgewählt haben.

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**Was zu tun ist:**

* Führen Sie `/model` aus und wählen Sie ein Modell, das Ihr Plan beinhaltet
* Wenn Sie Ihren Plan kürzlich aktualisiert haben und dies immer noch sehen, führen Sie `/logout` und dann `/login` aus. Das gespeicherte Token spiegelt Ihren Plan zum Zeitpunkt der Anmeldung wider, sodass ein Upgrade im Web in einer bestehenden Sitzung erst nach erneuter Authentifizierung wirksam wird.
* Siehe [claude.com/pricing](https://claude.com/pricing) für die Modelle, die jeder Plan beinhaltet

<h3 id="model-is-restricted-by-your-organizations-settings">
  Modell ist durch die Einstellungen Ihrer Organisation eingeschränkt
</h3>

Ihr Organisationsadministrator hat dieses Modell in der claude.ai-Administratorkonsole deaktiviert, oder es ist durch eine [`availableModels`](/docs/de/model-config#restrict-model-selection)-Zulassungsliste in verwalteten Einstellungen ausgeschlossen. Wenn das eingeschränkte Modell mit `--model`, `ANTHROPIC_MODEL` oder der `model`-Einstellung gesetzt wurde, ersetzt Claude Code ein zulässiges Modell und fährt fort. Das Eingeben von `/model <name>` für ein eingeschränktes Modell wird mit `Run /model to choose a different model.` abgelehnt und die Sitzung behält ihr aktuelles Modell.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code behandelt einen Modellgruppen-Alias, einen von `opus`, `sonnet`, `haiku` oder `fable`, als Anfrage für diese Gruppe statt für ihre neueste Version. Auf der Anthropic API und auf [Claude Platform on AWS](/docs/de/claude-platform-on-aws) wird ein eingeschränkter Gruppen-Alias zur neuesten Version der Gruppe aufgelöst, die Ihre Organisation und die `availableModels`-Zulassungsliste zulassen, und die Ersetzungsmitteilung nennt diese Version. Claude Code lehnt `/model <alias>` nur ab, wenn jede Version der Gruppe eingeschränkt ist. Vor v2.1.205 wurde ein Gruppen-Alias basierend auf seiner neuesten Version allein ersetzt oder abgelehnt, auch wenn eine ältere Version derselben Gruppe zulässig war.

**Was zu tun ist:**

* Führen Sie `/model` aus, um aus den Modellen auszuwählen, die Ihre Organisation zulässt. Eingeschränkte Modelle sind im Wähler verborgen.
* Wenn das eingeschränkte Modell in `--model`, `ANTHROPIC_MODEL` oder dem `model`-Feld einer Einstellungsdatei gesetzt wurde, entfernen oder aktualisieren Sie diesen Wert, sodass die Mitteilung nicht bei jedem Start erneut angezeigt wird
* Wenn Sie Zugriff auf das eingeschränkte Modell benötigen, bitten Sie Ihren Organisationsadministrator, es zu aktivieren. Siehe [Organisationsmodell-Einschränkungen](/docs/de/model-config#organization-model-restrictions).

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled wird für dieses Modell nicht unterstützt
</h3>

Ihre Claude Code-Version ist älter als das Minimum für Sonnet 5, Opus 4.8 oder Opus 4.7. Die CLI hat eine Thinking-Konfiguration gesendet, die das Modell nicht mehr akzeptiert.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Was zu tun ist:**

* Führen Sie `claude update` aus und starten Sie Claude Code neu. Opus 4.7 benötigt v2.1.111 oder später. Opus 4.8 benötigt v2.1.154 oder später. Sonnet 5 benötigt v2.1.197 oder später
* Wenn Sie nicht aktualisieren können, führen Sie `/model` aus und wählen Sie stattdessen Opus 4.6 oder Sonnet 4.6
* Wenn Sie dies im [Agent SDK](/docs/de/agent-sdk/overview) treffen, aktualisieren Sie stattdessen das SDK-Paket. Opus 4.8 benötigt TypeScript SDK v0.3.154 oder später und Python SDK v0.2.88 oder später. Sonnet 5 benötigt TypeScript SDK v0.3.197 oder später

<h3 id="thinking-budget-exceeds-output-limit">
  Thinking-Budget überschreitet Ausgabelimit
</h3>

Das konfigurierte Extended Thinking-Budget überschreitet die maximale Antwortlänge, sodass kein Platz für die eigentliche Antwort bleibt.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code passt diese Werte auf der Anthropic API automatisch an. Sie sehen diesen Fehler normalerweise auf Amazon Bedrock oder Google Cloud's Agent Platform, wenn [`MAX_THINKING_TOKENS`](/docs/de/env-vars) höher als das Ausgabelimit des Anbieters gesetzt ist, oder wenn Plan Mode das Thinking-Budget erhöht.

**Was zu tun ist:**

* Senken Sie `MAX_THINKING_TOKENS`, oder erhöhen Sie [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/de/env-vars) über das Thinking-Budget
* Siehe [Extended Thinking](/docs/de/model-config#extended-thinking) für die Interaktion des Budgets mit der Ausgabelänge

<h3 id="tool-use-or-thinking-block-mismatch">
  Tool-Verwendung oder Thinking-Block-Nichtübereinstimmung
</h3>

Die Gesprächshistorie erreichte die API in einem inkonsistenten Zustand, normalerweise nachdem ein Tool-Aufruf unterbrochen oder ein Turn während des Streams bearbeitet wurde.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Alle drei Varianten bedeuten dasselbe: Die Abfolge von `tool_use`-, `tool_result`- und `thinking`-Blöcken in der Historie stimmt nicht mehr mit dem überein, was die API erwartet.

**Was zu tun ist:**

* Wenn Sie Opus 4.7 oder Opus 4.8 verwenden, führen Sie zuerst `claude update` aus. Versionen vor v2.1.156 können diesen Fehler während normaler Tool-Verwendung auslösen, und `/rewind` löscht ihn nicht.
* Führen Sie `/rewind` aus, oder drücken Sie Esc zweimal, um zu einem Checkpoint vor dem beschädigten Turn zurückzugehen und von dort aus fortzufahren. Siehe [Checkpointing](/docs/de/checkpointing) für die Erstellung und Wiederherstellung von Checkpoints.

<h3 id="usage-policy-refusal">
  Richtlinienverletzung bei Nutzung
</h3>

Die API lehnte es ab zu antworten, da Inhalte im Gespräch eine [Nutzungsrichtlinie](https://www.anthropic.com/legal/aup)-Überprüfung auslösten. Die Nachricht enthält eine Request-ID, die Sie dem Support mitteilen können, wenn Sie glauben, dass die Ablehnung falsch ist.

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

Die Überprüfung bewertet das gesamte Gespräch, nicht nur Ihre neueste Eingabeaufforderung, sodass das Senden einer neuen Nachricht in derselben Sitzung normalerweise dieselbe Ablehnung erneut auslöst. Dasselbe gilt nach dem Beenden und erneuten Öffnen der Sitzung mit `--continue` oder `--resume`, da das Transkript auf der Festplatte immer noch den auslösenden Inhalt enthält. Auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) und [Microsoft Foundry](/docs/de/microsoft-foundry) deckt diese Nachricht auch Anfragen ab, die die Sicherheitsmaßnahmen des Modells als Cybersicherheitsthema gekennzeichnet haben. Siehe [Sicherheitsmaßnahmen haben ein Cybersicherheitsthema gekennzeichnet](#safety-measures-flagged-a-cybersecurity-topic).

**Was zu tun ist:**

* Drücken Sie Esc zweimal oder führen Sie `/rewind` aus, um zu einem Checkpoint vor dem Turn zurückzugehen, der die Ablehnung auslöste, und formulieren Sie dann neu oder versuchen Sie einen anderen Ansatz. Siehe [Checkpointing](/docs/de/checkpointing).
* Wenn Sie nicht identifizieren können, welcher Turn es verursacht hat, führen Sie `/clear` aus, um ein neues Gespräch im selben Projekt zu starten. Ihr vorheriges Gespräch bleibt auf der Festplatte erhalten und ist in `/resume` verfügbar.
* Im [nicht-interaktiven Modus](/docs/de/headless) (`-p`), wo Rewind nicht verfügbar ist, versuchen Sie es erneut mit einer umformulierten Eingabeaufforderung in einer neuen Sitzung ohne `--continue`. Richtlinienüberprüfungen variieren je nach Modell, sodass der Wechsel zu einem anderen Modell mit `--model` die Ablehnung in einigen Fällen auch beheben kann.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Sicherheitsmaßnahmen haben ein Cybersicherheitsthema gekennzeichnet
</h3>

Die Sicherheitsmaßnahmen des Modells haben Inhalte im Gespräch als Cybersicherheitsthema gekennzeichnet. Die Nachricht nennt das Modell, das die Anfrage gekennzeichnet hat:

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

Die Nachricht verlinkt auf das [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), das Zugriff für legitime Cybersicherheitsarbeit gewährt. Die Schutzmaßnahme selbst ist serverseitig und stammt vor v2.1.203; diese Version änderte nur die Formulierung der Nachricht und die Seite, auf die sie verlinkt.

Was Sie sehen, hängt von Ihrem Anbieter und Modus ab:

* Auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) und [Microsoft Foundry](/docs/de/microsoft-foundry) erzeugt eine Cybersicherheits-Kennzeichnung stattdessen die [Richtlinienverletzung bei Nutzung](#usage-policy-refusal)-Nachricht.
* Der [nicht-interaktive Modus](/docs/de/headless) lässt den `/feedback`-Satz weg.

Vor v2.1.203 lautete die Nachricht `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` gefolgt von einem Ausnahmeantragsformular-Link.

**Was zu tun ist:**

* Wenn Ihre Arbeit diesen Inhalt erfordert, beantragen Sie Zugriff durch das [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Wenn Ihre Anfrage nicht über ein Cybersicherheitsthema war, führen Sie `/feedback` aus, um das falsch positive Ergebnis zu melden
* Um in derselben Sitzung weiterzuarbeiten, drücken Sie Esc zweimal oder führen Sie `/rewind` aus, um zu einem Checkpoint vor dem Turn zurückzugehen, der die Kennzeichnung auslöste, und versuchen Sie dann einen anderen Ansatz. Siehe [Checkpointing](/docs/de/checkpointing).

<h2 id="installation-errors">
  Installationsfehler
</h2>

Diese Fehler treten bei der Installation oder Aktualisierung von Claude Code auf, entweder vom [Installationsskript](/docs/de/setup#install-claude-code), von `claude install` oder von `claude update`. Für `command not found`, PATH-, Berechtigungs- und TLS-Probleme während der Einrichtung siehe [Installationen und Anmeldung beheben](/docs/de/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  Installation wurde beendet, bevor sie abgeschlossen werden konnte
</h3>

Das Installationsskript meldet, wenn der `claude install`-Schritt durch ein Signal beendet wird. Unter Linux bedeutet Exit-Code 137, dass der Prozess SIGKILL erhalten hat, und auf einem Host mit wenig Speicher ist das normalerweise der Kernel-Out-of-Memory (OOM)-Killer. Das Skript gibt diese Erklärung aus und beendet sich mit Code 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Für jedes andere tödliche Signal und für Exit-Code 137 auf macOS gibt das Skript `Installation was killed before it could finish (exit code <N>)` mit dem tatsächlichen Exit-Code aus und lässt die Out-of-Memory-Erklärung weg. Die Meldung stammt vom Installationsskript, das macOS und Linux verwenden, das auch Installationen innerhalb von WSL abdeckt; die nativen Windows-Installationsskripte geben es nie aus. Vor v2.1.200 beendete sich das Skript nur mit der bloßen `Killed`-Zeile der Shell.

**Was zu tun ist:**

* Beenden Sie andere Prozesse, um Speicher freizugeben, und führen Sie dann das Installationsprogramm erneut aus
* Fügen Sie Swap-Speicher hinzu oder wechseln Sie zu einer größeren Instanz. Siehe [Installation auf Linux-Servern mit wenig Speicher beendet](/docs/de/troubleshoot-install#install-killed-on-low-memory-linux-servers) für die Swap-Datei-Befehle.

<h3 id="the-connection-dropped-while-downloading-the-update">
  Die Verbindung wurde während des Download der Aktualisierung unterbrochen
</h3>

Die Verbindung zum Download-Server wurde geschlossen, während `claude install`, `claude update` oder das [automatische Update-Programm](/docs/de/setup#auto-updates) die Claude Code-Binärdatei abrief, und die Wiederholungen konnten sich nicht erholen. Claude Code wiederholt den Download, wenn die Verbindung abbricht, die Übertragung steckenbleibt oder die heruntergeladene Datei ihre Prüfsumme nicht besteht, insgesamt bis zu drei Versuche. Ein abgeschlossener HTTP-Fehler, wie z. B. ein 404, wird nicht wiederholt, da der Server bereits geantwortet hat. Vor v2.1.202 führte eine einzelne unterbrochene Verbindung sofort zum Fehlschlag des Downloads mit dem bloßen Fehler `aborted` statt zu wiederholen.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Der Text in Klammern nennt, welcher Versuch fehlgeschlagen ist, und den zugrunde liegenden Netzwerkfehler. `claude update` stellt der Meldung `Error: Failed to install native update` auf stderr voran.

Ein Download, der verbunden bleibt, aber nicht innerhalb von 10 Minuten abgeschlossen wird, schlägt mit `Download timed out: exceeded the total deadline` fehl. Claude Code wiederholt einen abgelaufenen Download nicht, da eine Verbindung, die zu langsam ist, um innerhalb der Frist abgeschlossen zu werden, auch bei einer sofortigen Wiederholung nicht abgeschlossen wird. Die folgenden Schritte gelten für beide Meldungen. Vor v2.1.205 wurde die gleiche 10-Minuten-Frist als generischer `timeout of 600000ms exceeded` des HTTP-Clients gemeldet.

Die übliche Ursache ist ein Proxy oder Gateway, das eine lange Übertragung beendet, bevor sie abgeschlossen ist. Die Claude Code-Binärdatei ist ein großer Download, daher kann eine Proxy-Verbindungsbegrenzung, die normalen API-Verkehr nie beeinträchtigt, dennoch unterbrochen werden.

**Was zu tun ist:**

* Führen Sie `claude update` erneut aus. Bei einem ansonsten gesunden Netzwerk ist der Download normalerweise beim nächsten Durchlauf erfolgreich. Für die Timeout-Meldung führen Sie es erneut aus einem schnelleren oder weniger gedrosselten Netzwerk aus.
* Wenn Ihr Netzwerk einen Proxy erfordert, setzen Sie `HTTPS_PROXY` vor dem Ausführen des Installationsprogramms oder `claude update`. Siehe [Netzwerkkonnektivität überprüfen](/docs/de/troubleshoot-install#check-network-connectivity).
* Wenn ein Unternehmens-Proxy die Übertragung immer wieder beendet, bitten Sie Ihr Netzwerk-Team, den vollständigen Download von `downloads.claude.ai` zuzulassen. Siehe [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements).
* Führen Sie `claude doctor` aus Ihrer Shell aus, um Installationsdiagnosen durchzuführen

<h2 id="command-line-errors">
  Befehlszeilenfehler
</h2>

Diese Fehler stammen aus dem `claude` Befehl und seinen Unterbefehlen. Claude Code gibt sie aus, bevor er Ihren Prompt ausführt oder eine API-Anfrage sendet.

<h3 id="conflict-between-bg-and-print">
  Konflikt zwischen --bg und --print
</h3>

Diese Meldung erfordert Claude Code v2.1.198 oder später. Sie haben `--bg` mit `-p` oder `--print` in derselben `claude` Invokation kombiniert. `--bg` startet eine [Hintergrundsitzung](/docs/de/agent-view#from-your-shell), die Sie später mit `claude agents` anfügen, während `--print` [nicht interaktiv](/docs/de/headless) ausgeführt wird und niemals die interaktive Sitzung startet, die `claude agents` anfügt. Vor v2.1.198 hat diese Kombination stillschweigend einen Hintergrund-Job erstellt, der niemals angefügt werden konnte.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Was zu tun ist:**

* Lassen Sie `-p` oder `--print` weg. `--bg` nimmt den Prompt als sein Positionsargument, also ist `claude --bg "<task>"` der vollständige Befehl. Siehe [Neue Agenten von Ihrer Shell aus versenden](/docs/de/agent-view#from-your-shell).
* Um den Prompt nicht interaktiv auszuführen und das Ergebnis auszudrucken, anstatt eine Hintergrundsitzung zu erstellen, lassen Sie `--bg` weg und führen Sie `claude -p "<task>"` aus

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  Der --json-schema Wert ist kein gültiges JSON Schema
</h3>

Das Schema, das Sie an [`--json-schema`](/docs/de/cli-reference#cli-flags) im [nicht interaktiven Modus](/docs/de/headless#get-structured-output) übergeben haben, ist bei der JSON Schema Kompilierung fehlgeschlagen, daher beendet sich `claude` mit Code 1, anstatt den Prompt auszuführen. Vor v2.1.205 hat ein ungültiges Schema unstrukturierte Ausgabe ohne Fehler erzeugt, und jedes Schema, das das `format` Schlüsselwort verwendete, wurde als ungültig behandelt.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Der Text nach dem zweiten Doppelpunkt ist die Diagnose des Validators und benennt das Schlüsselwort oder die Position, die fehlgeschlagen ist. Schemas, die das `format` Schlüsselwort verwenden, wie `"format": "email"`, sind gültig: Claude Code akzeptiert `format` als Anmerkung und erzwingt es nicht.

Claude Code führt zwei Überprüfungen vor der Schema-Kompilierung durch: Es lehnt einen Wert ab, der nicht als JSON analysierbar ist, mit `Error: --json-schema is not valid JSON`, und gültiges JSON, das kein Objekt ist, mit `Error: --json-schema must be a JSON object`.

**Was zu tun ist:**

* Beheben Sie den Teil des Schemas, den die Diagnose benennt, und führen Sie dann den Befehl erneut aus
* Wenn die Diagnose `schema too large` ist, reduzieren Sie die Verschachtelung des Schemas und die `$ref` Wiederverwendung
* Siehe [Strukturierte Ausgabe abrufen](/docs/de/headless#get-structured-output) für ein funktionierendes Schema und einen Befehl

<h3 id="could-not-import-a-server-from-claude-desktop">
  Konnte keinen Server aus Claude Desktop importieren
</h3>

Claude Code konnte einen der Server, die Sie in `claude mcp add-from-claude-desktop` ausgewählt haben, nicht hinzufügen. Der Befehl importiert weiterhin die anderen ausgewählten Server und gibt eine Zeile pro Server aus, den er nicht hinzufügen konnte. Vor v2.1.205 hat der erste Server, der fehlgeschlagen ist, den Import gestoppt und keiner der ausgewählten Server wurde hinzugefügt.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Der Text nach dem Servernamen ist der Grund. Der häufigste ist die Namensüberprüfung: Claude Desktop erlaubt Zeichen in Servernamen, wie Leerzeichen und Punkte, die `claude mcp` auf Buchstaben, Zahlen, Bindestriche und Unterstriche beschränkt. Andere Gründe sind eine Serverkonfiguration, die die Validierung nicht besteht, und ein Server, der durch die [MCP-Richtlinie](/docs/de/managed-mcp) Ihrer Organisation blockiert wird.

**Was zu tun ist:**

* Benennen Sie den Server in `claude_desktop_config.json` um, um nur Buchstaben, Zahlen, Bindestriche und Unterstriche zu verwenden, und führen Sie dann `claude mcp add-from-claude-desktop` erneut aus
* Fügen Sie diesen Server direkt mit `claude mcp add` oder `claude mcp add-json` unter einem gültigen Namen hinzu. Siehe [MCP-Server aus Claude Desktop importieren](/docs/de/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="mcp-permission-prompt-tool-not-found">
  MCP-Berechtigungsprompt-Tool nicht gefunden
</h3>

Das Tool, das Sie an [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) übergeben haben, war nicht unter den verbundenen MCP-Tools, als der Lauf zum ersten Mal eine Berechtigungsentscheidung benötigte, entweder weil sein Server sich nie verbunden hat oder weil kein verbundener Server ein Tool mit diesem Namen verfügbar macht. Claude Code sendet Ihren Prompt weiterhin: Der [nicht interaktive](/docs/de/headless) Lauf beendet sich mit diesem Fehler und Exit-Code 1 beim ersten Tool-Aufruf, der Genehmigung benötigt, daher wird keine Antwort erzeugt, obwohl die Anfrage gestellt wurde. Vor dem ersten Prompt wartet Claude Code bis zu 30 Sekunden auf das pro-Server-Verbindungs-Timeout, das durch [`MCP_TIMEOUT`](/docs/de/env-vars) festgelegt ist, damit sich dieser Server verbindet. Vor v2.1.206 hat der Start nicht auf das Beenden der Serververbindung gewartet, daher hat ein langsam startender, aber funktionierender Server auch diesen Fehler erzeugt.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

Die Liste nach `Available MCP tools:` benennt die MCP-Tools, die verbunden waren, als das Warten endete.

**Was zu tun ist:**

* Überprüfen Sie, dass der Server startet und verbunden bleibt: Führen Sie `claude mcp list` im selben Verzeichnis aus und bestätigen Sie, dass der Server als verbunden aufgelistet ist
* Bestätigen Sie, dass der Tool-Name dem `mcp__<server>__<tool>` Namen entspricht, den der Server verfügbar macht
* Wenn der Server länger als 30 Sekunden zum Starten benötigt, erhöhen Sie [`MCP_TIMEOUT`](/docs/de/env-vars)

<h2 id="plugin-errors">
  Plugin-Fehler
</h2>

Diese Fehler stammen aus der [Plugin](/docs/de/plugins)- und [Marketplace](/docs/de/plugin-marketplaces)-Konfiguration. Bei Plugin-Problemen, die keine der Meldungen auf dieser Seite erzeugen, wie z. B. eine Marketplace-URL, die nicht geladen wird, oder ein Plugin, das installiert wird, aber nicht angezeigt wird, siehe [Plugin-Fehlerbehebung](/docs/de/discover-plugins#troubleshooting).

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace ist von einer nicht vertrauenswürdigen Quelle registriert
</h3>

Der Marketplace ist unter einem Namen registriert, der [für offizielle Anthropic-Marketplaces reserviert ist](/docs/de/plugin-marketplaces#marketplace-schema), aber seine registrierte Quelle ist kein `anthropics` GitHub-Repository. Claude Code überprüft reservierte Namen jedes Mal, wenn es einen Marketplace lädt oder aktualisiert, sodass der Marketplace und die von ihm installierten Plugins nicht mehr geladen werden. Vor v2.1.205 wurde der Name nur überprüft, wenn der Marketplace hinzugefügt wurde, sodass ein Eintrag, der registriert wurde, bevor sein Name reserviert wurde, weiterhin geladen wurde.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**Was zu tun ist:**

* Führen Sie `claude plugin marketplace remove <name>` aus und fügen Sie den Marketplace dann erneut aus dem offiziellen `github.com/anthropics`-Repository hinzu
* Wenn Sie einen Drittanbieter-Marketplace veröffentlichen, der den Namen verwendet hat, bevor er reserviert wurde, benennen Sie ihn um und bitten Sie Benutzer, ihn von Ihrer Quelle erneut hinzuzufügen
* Siehe die Liste der reservierten Namen unter [Marketplace-Schema](/docs/de/plugin-marketplaces#marketplace-schema)

<h3 id="plugin-command-references-user-config">
  Plugin-Befehl referenziert user\_config in einem Shell-Befehl
</h3>

Ein Plugin-Hook, [monitor](/docs/de/plugins-reference#monitors), oder MCP [`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication)-Befehl referenziert eine `${user_config.KEY}` [Plugin-Option](/docs/de/plugins-reference#user-configuration), und die ersetzte Zeichenkette würde an eine Shell übergeben. Ein konfigurierter Wert, der `$(...)`, Backticks oder `;` enthält, würde dort als Code ausgeführt, daher weigert sich Claude Code, die Komponente zu starten, anstatt den Wert zu ersetzen. Die Überprüfung wird auf der Befehlsvorlage ausgeführt, daher wird der Fehler angezeigt, auch wenn noch kein Wert konfiguriert ist. Vor v2.1.207 wurde der Wert in den Shell-Befehl eingesetzt.

Die Formulierung hängt davon ab, welche Oberfläche die Option referenziert hat. Ein Shell-Form-Hook meldet:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Ein Monitor meldet:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Ein MCP `headersHelper` meldet:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Was zu tun ist:**

* Für einen Hook fügen Sie ein `args`-Array hinzu, damit es in [Exec-Form](/docs/de/hooks#exec-form-and-shell-form) ausgeführt wird, wobei jedes `${user_config.KEY}` zu einem Argument wird, ohne dass eine Shell dazwischen liegt. Oder lassen Sie die Referenz weg und lesen Sie die `$CLAUDE_PLUGIN_OPTION_<KEY>`-Umgebungsvariable innerhalb des Skripts
* Für einen Monitor lassen Sie die Referenz weg und lassen Sie das Monitor-Skript den Wert aus einer Konfigurationsdatei lesen
* Für einen `headersHelper` verschieben Sie `${user_config.KEY}` in das `headers`-Feld des Servers, das nicht shell-geparst wird, oder lesen Sie den Wert innerhalb des Helper-Skripts

<h2 id="tool-errors">
  Werkzeugfehler
</h2>

Diese Fehler entstehen, wenn Claude's integrierte Werkzeuge eine Eingabe ablehnen. Claude korrigiert die meisten Werkzeugfehler automatisch; die beiden folgenden erfordern eine Änderung von Ihnen, da sie aus einer Subagent-Definition oder einer Berechtigungsregel stammen, die Sie kontrollieren.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent würde mit null Werkzeugen erzeugt
</h3>

Nichts in der [Werkzeugliste eines Subagent's](/docs/de/sub-agents#supported-frontmatter-fields) wurde zu einem Werkzeug aufgelöst, daher weigert sich Claude Code, den Subagent zu starten, anstatt einen zu starten, der nicht handeln kann. Die Meldung gruppiert die Einträge danach, warum sie nicht aufgelöst wurden: kein erkanntes Werkzeug, ein Werkzeug, das für Subagents nicht verfügbar ist, oder erkannt, aber kein Werkzeug in der aktuellen Sitzung entsprechend. Das Weglassen des `tools`-Feldes löst diese Ablehnung niemals aus. Ein MCP-Servermuster wie `mcp__github__*` ist nicht ausgenommen: Wenn kein verbundenes Werkzeug von diesem Server stammt, wird der Start mit dem Muster in der Gruppe „nichts gefunden" abgelehnt. Vor v2.1.208 wurde der Subagent mit null Werkzeugen gestartet und gab ein leeres oder verwirrendes Ergebnis zurück.

```text theme={null}
Agent 'code-reviewer' würde mit null Werkzeugen erzeugt — Ablehnung. Seine Werkzeugliste wurde zu nichts aufgelöst: unerkannt [Grpe]. Korrigieren Sie die Werkzeuge-Frontmatter des Agenten oder übergeben Sie einen anderen subagent_type.
```

**Was zu tun ist:**

* Korrigieren Sie jeden Eintrag, den der Fehler gegen die [für Subagents verfügbaren Werkzeuge](/docs/de/sub-agents#available-tools) benennt
* Entfernen Sie Einträge für Werkzeuge, die die Sitzung nicht hat, wie MCP-Werkzeuge von einem Server, der nicht verbunden ist
* Um dem Subagent jedes Werkzeug zu geben, das der übergeordnete Agent hat, löschen Sie stattdessen das `tools`-Feld, anstatt Werkzeuge aufzulisten

<h3 id="file-is-covered-by-a-read-deny-rule">
  Datei wird durch eine Read-Ablehnungsregel abgedeckt
</h3>

Das Edit-Werkzeug wurde auf einem Pfad aufgerufen, der einer [`Read`-Ablehnungsregel](/docs/de/permissions#read-and-edit) entspricht, einschließlich der Erstellung einer neuen Datei in diesem Pfad. Das Bearbeiten schreibt Inhalte um, die Claude zurücklesen können muss, daher wird der Aufruf vor jedem Dateizugriff abgelehnt. Die Regel blockiert nur das Edit-Werkzeug: Write und NotebookEdit werden nicht durch `Read`-Ablehnungsregeln abgedeckt. Vor v2.1.208 blockierten nur eine `Edit`-Ablehnungsregel Bearbeitungen, und eine `Read`-Ablehnungsregel allein tat dies nicht.

```text theme={null}
Die Datei wird durch eine Read-Ablehnungsregel in Ihren Berechtigungseinstellungen abgedeckt und kann nicht bearbeitet werden.
```

**Was zu tun ist:**

* Wenn Claude die Datei bearbeiten können soll, entfernen oder verengen Sie die `Read`-Ablehnungsregel in `/permissions` oder in [Einstellungen](/docs/de/settings#permission-settings)
* Wenn die Datei unverändert bleiben muss, behalten Sie die Regel und fügen Sie eine `Edit`-Ablehnungsregel für denselben Pfad hinzu, damit die Write- und NotebookEdit-Werkzeuge ebenfalls blockiert werden

<h2 id="background-session-errors">
  Fehler in Hintergrund-Sitzungen
</h2>

[Hintergrund-Sitzungen](/docs/de/agent-view) laufen ohne ein eigenes interaktives Terminal, daher verhalten sich Befehle, die eines benötigen, dort anders. Diese Meldungen erscheinen im Transkript einer Hintergrund-Sitzung, in der Agent-Ansicht oder nach dem Anhängen.

<h3 id="commands-refused-in-a-background-session">
  Befehle, die in einer Hintergrund-Sitzung abgelehnt werden
</h3>

Befehle, die einen interaktiven Dialog öffnen, werden in einer Hintergrund-Sitzung mit einer Meldung abgelehnt, die entweder ein dort funktionierendes Formular benennt oder Ihnen mitteilt, den Befehl von einem regulären Terminal aus auszuführen. `/install-github-app`, die `/mcp`-Einstellungsliste und die Authentifizierungsaktionen im MCP-Server-Menü werden alle auf diese Weise abgelehnt. Vor v2.1.208 öffneten sie ihren Dialog innerhalb der Hintergrund-Sitzung.
In v2.1.208 nur wurde die `/model`-Auswahl auch in einer Hintergrund-Sitzung abgelehnt, und `/upgrade` gab die Upgrade-URL aus, anstatt einen Browser zu öffnen.

Die Formulierung benennt den Befehl, der abgelehnt wurde. Die `/mcp`-Einstellungsliste meldet:

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**Was zu tun ist:**

* Verwenden Sie das Formular, das die Meldung benennt, wie `/mcp reconnect <server>`, `/mcp enable` oder `/mcp disable`
* Für Anmelde- und Autorisierungsabläufe führen Sie den Befehl von einer regulären `claude`-Sitzung in einem Terminal aus

<h3 id="claude_code_process_wrapper-launcher-errors">
  CLAUDE\_CODE\_PROCESS\_WRAPPER Launcher-Fehler
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/de/corporate-launcher) ist gesetzt, und sein Wert kann nicht verwendet werden, daher weigert sich Claude Code, den betroffenen Prozess zu starten, anstatt ihn ohne den Launcher auszuführen. Konfigurationsprobleme werden mit einer Meldung gemeldet, die mit dem Variablennamen beginnt und den Grund angibt, zum Beispiel:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Ein Launcher, der startet, aber beendet wird, ohne sich selbst durch Claude Code zu ersetzen, schlägt die Sitzung fehl, die er starten wollte, und die Zeile der Sitzung in der Agent-Ansicht meldet, dass der Launcher `must exec, not daemonize`, gefolgt von allem, was der Launcher ausgegeben hat. Eine Sitzung, die nicht starten oder den Hintergrund-Service nicht erreichen kann, weil des Launchers, meldet das Launcher-Problem als Grund innerhalb von `Couldn't reach the background service (...)`.

**Was zu tun ist:**

* Setzen Sie die Variable auf den absoluten Pfad einer ausführbaren Datei, die mit dem Aufruf von `exec "$@"` endet. Siehe [den Launcher-Vertrag](/docs/de/corporate-launcher#the-launcher-contract) für den vollständigen Vertrag
* Überprüfen Sie `/status`, das den aufgelösten Startbefehl in seinem Self-exec-Eintrag anzeigt und warnt, wenn der laufende Hintergrund-Service nicht damit übereinstimmt, oder führen Sie `claude daemon status` von einer Shell aus
* Nach dem Beheben des Werts im `env`-Block von [Einstellungen](/docs/de/corporate-launcher#set-up-the-launcher) starten Sie den Hintergrund-Service mit `claude daemon stop --any` neu, damit die nächste Verteilung einen umschlossenen startet

<h2 id="configuration-warnings">
  Konfigurationswarnungen
</h2>

Claude Code schreibt diese Meldungen beim Start in stderr statt einen Fehler im Gespräch anzuzeigen. Sie berichten über Konfigurationen, die gelesen, aber nicht angewendet wurden.

<h3 id="workspace-has-not-been-trusted">
  Arbeitsbereich wurde nicht vertraut
</h3>

Claude Code fand `permissions.allow`-Regeln oder `permissions.additionalDirectories`-Einträge in der Datei `.claude/settings.json` oder `.claude/settings.local.json` des Projekts und wendete sie nicht an, da [Allow-Regeln aus Projekteinstellungen Arbeitsbereichsvertrauen erfordern](/docs/de/permissions#project-allow-rules-and-workspace-trust). Die Anzahl, der Einstellungsname und die in der Meldung genannte Datei variieren je nach Ihrer Konfiguration. `deny`- und `ask`-Regeln sind nicht betroffen.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Was zu tun ist:**

* Führen Sie `claude` im Verzeichnis aus und akzeptieren Sie den Vertrauensdialog. Der Dialog wird angezeigt, auch wenn ein übergeordnetes Verzeichnis bereits vertraut ist, listet die zurückgehaltenen Regeln auf und ermöglicht es Ihnen, abzulehnen und ohne diese weiterzuarbeiten. Vor v2.1.200 wurde in dieser Situation kein Dialog angezeigt, daher konnte dieser Schritt dort nicht abgeschlossen werden.
* Im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` wird kein Dialog angezeigt. Legen Sie den `hasTrustDialogAccepted`-Eintrag in `~/.claude.json` mit dem genauen `projects`-Schlüssel fest, den die Meldung ausgibt.
* Wenn die Meldung `.claude/settings.local.json` nennt und Sie Claude Code außerhalb eines Git-Repositorys oder in Ihrem Home-Verzeichnis gestartet haben, aktualisieren Sie auf v2.1.200 oder später. Die Versionen 2.1.196 bis 2.1.199 behandelten Ihre eigene `.claude/settings.local.json` in diesen Arbeitsbereichen als vom Repository bereitgestellt. Auf v2.1.207 und später ist eine Aktualisierung außerhalb eines Git-Repositorys nicht ausreichend, wenn Sie den Ordner nicht vertraut haben: Die Feststellung, dass sich ein Ordner nicht in einem Repository befindet, führt Git aus, und Claude Code führt diese Überprüfung nur durch, nachdem Sie den Vertrauensdialog akzeptiert haben. Verwenden Sie daher den ersten Schritt. Ihr Home-Verzeichnis und alle anderen [Konfigurationshome](/docs/de/permissions#project-allow-rules-and-workspace-trust) sind ausgenommen und warten nicht auf den Dialog. Siehe [Projektallow-Regeln und Arbeitsbereichsvertrauen](/docs/de/permissions#project-allow-rules-and-workspace-trust).

<h2 id="responses-seem-lower-quality-than-usual">
  Antworten scheinen von geringerer Qualität als üblich
</h2>

Wenn Claudes Antworten weniger leistungsfähig erscheinen als erwartet, aber kein Fehler angezeigt wird, liegt die Ursache normalerweise im Gesprächszustand und nicht im Modell selbst. Claude Code ändert nicht stillschweigend Modellversionen. Es kann in drei spezifischen Fällen zu einem Fallback-Modell wechseln:

* Ein konfiguriertes [`--fallback-model`](/docs/de/cli-reference#cli-flags) übernimmt nach einem Verfügbarkeitsfehler für diesen Zug nur mit einer Notiz im Transkript
* Eine Amazon Bedrock oder Google Cloud Agent Platform Startprüfung stellt fest, dass Ihr Standardmodell nicht verfügbar ist
* [Automatisches Modell-Fallback](/docs/de/model-config#automatic-model-fallback) auf Fable 5 verschiebt die Sitzung zum Standard-Opus-Modell und zeigt eine Notiz im Transkript an

Die Modellauswahlprüfung unten erfasst den zweiten und dritten Fall; der erste erscheint als Transkriptnotiz statt als `/model`-Änderung. [Modellkonfiguration](/docs/de/model-config) erklärt, wann jedes Fallback angewendet wird.

Überprüfen Sie diese zuerst:

* **Modellauswahl**: Führen Sie `/model` aus, um zu bestätigen, dass Sie das erwartete Modell verwenden. Eine vorherige `/model`-Auswahl oder eine `ANTHROPIC_MODEL`-Umgebungsvariable kann Sie auf einem kleineren Modell als beabsichtigt platzieren.
* **Anstrengungsgrad**: Führen Sie `/effort` aus, um die aktuelle Reasoning-Stufe zu überprüfen und sie für schwieriges Debugging oder Design-Arbeit zu erhöhen. Die Standardwerte variieren je nach Modell, daher überprüfen Sie, bevor Sie davon ausgehen, dass Sie unter dem Maximum liegen. Siehe [Anstrengungsgrad anpassen](/docs/de/model-config#adjust-effort-level) für modellspezifische Standardwerte und die `ultrathink`-Verknüpfung.
* **Kontextdruck**: Führen Sie `/context` aus, um zu sehen, wie voll das Fenster ist. Wenn es sich der Kapazität nähert, führen Sie `/compact` an einem natürlichen Haltepunkt oder `/clear` aus, um neu zu beginnen. Siehe [Erkunden Sie das Kontextfenster](/docs/de/context-window), um zu erfahren, wie Auto-Compact frühere Züge beeinflusst.
* **Veraltete Anweisungen**: Große oder veraltete `CLAUDE.md`-Dateien und MCP-Tool-Definitionen verbrauchen Kontext und können Antworten lenken. Die `/doctor`-Überprüfung kennzeichnet übergroße Speicherdateien und ungenutzte Erweiterungen, und `/context` zeigt die MCP-Tool-Token-Nutzung an. Vor v2.1.205 öffnete `/doctor` einen Diagnose-Bildschirm, der übergroße Speicherdateien und Subagent-Definitionen kennzeichnete.

Wenn eine Antwort schiefgeht, funktioniert das Zurückspulen normalerweise besser als das Antworten mit Korrektionen. Drücken Sie zweimal Esc oder führen Sie `/rewind` aus, um vor den fehlerhaften Zug zurückzugehen, und formulieren Sie dann die Eingabeaufforderung mit mehr Spezifika um. Korrigieren im Thread behält den falschen Versuch im Kontext, was später Antworten daran verankern kann. Siehe [Checkpointing](/docs/de/checkpointing).

Wenn die Qualität nach Überprüfung der obigen Punkte immer noch schlecht erscheint, führen Sie `/feedback` aus und beschreiben Sie, was Sie erwartet haben im Vergleich zu dem, was Sie erhalten haben. Auf diese Weise eingereichte Rückmeldungen enthalten das Gesprächstranskript, das die schnellste Möglichkeit für Anthropic ist, eine echte Regression zu diagnostizieren. Siehe [Fehler melden](#report-an-error), wenn `/feedback` in Ihrer Umgebung nicht verfügbar ist.

Wenn Claude vor einer vermuteten Prompt-Injection warnt oder eine Anfrage wegen einer vermuteten Injection ablehnt, und der Text, den die Warnung benennt, Kontext ist, den Claude Code automatisch zum Gespräch hinzufügt, anstatt Datei- oder Webinhalte, führen Sie `claude update` aus und versuchen Sie es erneut. Wenn die Warnung nach dem Update wiederholt wird, [melden Sie es](#report-an-error), anstatt den gekennzeichneten Inhalt zurück in die Eingabeaufforderung einzufügen. Vor v2.1.201 lehnten Sonnet 5 einige Anfragen auf die gleiche Weise ab.

<h2 id="report-an-error">
  Fehler melden
</h2>

Für Fehler von Komponenten, die auf dieser Seite nicht behandelt werden, siehe die relevanten Anleitungen:

* MCP-Server konnte sich nicht verbinden oder authentifizieren: [MCP](/docs/de/mcp)
* Hook-Skript ist fehlgeschlagen oder hat ein Tool blockiert: [Debug-Hooks](/docs/de/hooks#debug-hooks)
* Berechtigung verweigert oder Dateisystemfehler während der Installation: [Troubleshoot-Installation und -Anmeldung](/docs/de/troubleshoot-install)

Wenn ein Fehler hier nicht aufgeführt ist oder die vorgeschlagene Lösung nicht hilft:

* Führen Sie `/feedback` in Claude Code aus, um das Transkript und eine Beschreibung an Anthropic zu senden. Der Befehl bietet auch an, ein vorausgefülltes GitHub-Issue zu öffnen. Das Senden an Anthropic erfordert [Authentifizierung](/docs/de/authentication). Bei Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und anderen Drittanbieter-Plattformen oder wenn keine Anthropic-Anmeldedaten konfiguriert sind, speichert `/feedback` ein lokales Archiv, das Sie stattdessen an Ihren Anthropic-Kontorepräsentanten senden können.
* Führen Sie `claude doctor` aus Ihrer Shell aus, um eine schreibgeschützte Diagnose Ihrer Installation zu erhalten, oder führen Sie die `/doctor`-Überprüfung in Claude Code aus, um Setup-Probleme zu finden und zu beheben
* Überprüfen Sie [status.claude.com](https://status.claude.com) auf aktive Vorfälle
* Suchen Sie nach [bestehenden Issues](https://github.com/anthropics/claude-code/issues) auf GitHub
