> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code programmgesteuert ausführen

> Verwenden Sie das Agent SDK, um Claude Code programmgesteuert über die CLI, Python oder TypeScript auszuführen.

Das [Agent SDK](/docs/de/agent-sdk/overview) bietet Ihnen die gleichen Tools, die Agent-Schleife und das Kontextmanagement, die Claude Code antreiben. Es ist als CLI für Skripte und CI/CD verfügbar oder als [Python](/docs/de/agent-sdk/python)- und [TypeScript](/docs/de/agent-sdk/typescript)-Pakete für vollständige programmgesteuerte Kontrolle.

Um Claude Code im nicht-interaktiven Modus auszuführen, übergeben Sie `-p` mit Ihrer Eingabeaufforderung und allen [CLI-Optionen](/docs/de/cli-reference), die Sie benötigen:

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Diese Seite behandelt die Verwendung des Agent SDK über die CLI (`claude -p`). Für die Python- und TypeScript-SDK-Pakete mit strukturierten Ausgaben, Tool-Genehmigungsrückrufen und nativen Nachrichtenobjekten siehe die [vollständige Agent SDK-Dokumentation](/docs/de/agent-sdk/overview).

<h2 id="basic-usage">
  Grundlegende Verwendung
</h2>

Fügen Sie das Flag `-p` (oder `--print`) zu jedem `claude`-Befehl hinzu, um ihn nicht interaktiv auszuführen. Nicht alle [CLI-Optionen](/docs/de/cli-reference) funktionieren mit `-p`. Claude Code lehnt `--bg` ab und lehnt `--cloud` mit einer Aufgabenbeschreibung mit einem Fehler ab, der den Konflikt benennt; `--cloud` mit einer Sitzungs-ID und `-p` [reiht stattdessen eine Nachricht in diese Cloud-Sitzung ein](/docs/de/claude-code-on-the-web#send-follow-ups-from-the-cli) und wird beendet. Optionen, die Sie häufig mit `-p` kombinieren, sind:

* `--continue` zum [Fortsetzen von Gesprächen](#continue-conversations)
* `--allowedTools` zum [automatischen Genehmigen von Tools](#auto-approve-tools)
* `--output-format` für [strukturierte Ausgabe](#get-structured-output)

Dieses Beispiel stellt Claude eine Frage zu Ihrer Codebasis und gibt die Antwort aus:

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code wird mit Code 0 bei Erfolg und mit einem Code ungleich Null beendet, wenn die Ausführung fehlschlägt, sodass Ihre Skripte basierend auf dem Exit-Status verzweigen können. Wenn Sie ein ungültiges Flag übergeben, meldet Claude Code den Fehler an stderr, bevor die Ausführung beginnt. Wenn ein Fehler während der Ausführung auftritt, z. B. fehlende Authentifizierung, gibt Claude Code den Fehler als Ergebnis auf stdout aus.

<h3 id="start-faster-with-bare-mode">
  Schneller starten mit Bare-Modus
</h3>

Fügen Sie `--bare` hinzu, um die Startzeit zu verkürzen, indem Sie die automatische Erkennung von hooks, skills, benutzerdefinierten Befehlen, [Subagenten](/docs/de/sub-agents), installierten Plugins, MCP-Servern, automatischem Speicher und CLAUDE.md überspringen. Ohne diese Option lädt `claude -p` den gleichen [Kontext](/docs/de/how-claude-code-works#the-context-window), den eine interaktive Sitzung hätte, einschließlich alles, was im Arbeitsverzeichnis oder in `~/.claude` konfiguriert ist.

Der Bare-Modus ist nützlich für CI und Skripte, bei denen Sie auf jedem Computer das gleiche Ergebnis benötigen. Ein hook in der `~/.claude` eines Teamkollegen oder ein MCP-Server in der `.mcp.json` des Projekts werden nicht ausgeführt, da der Bare-Modus diese nie liest. Ein Verzeichnis, das Sie mit `--add-dir` benennen, ist eine teilweise Ausnahme: Der Bare-Modus lädt Skills aus seinem `.claude/skills/`-Ordner, überspringt aber immer noch seine `.claude/commands/`- und `.claude/agents/`-Ordner. [Skills aus zusätzlichen Verzeichnissen](/docs/de/skills#skills-from-additional-directories) behandelt, was geladen wird und was nicht.

Ohne `--bare` führt eine `-p`-Sitzung die Hooks in der `.claude/settings.json` eines Projekts aus und verbindet die Server in seiner `.mcp.json`, auch in einem Ordner, dem Sie nie vertraut haben. Eine `-p`-Sitzung zeigt keinen Workspace-Trust-Dialog und keine Pro-Server-Genehmigungsaufforderung an. [Was wird ausgeführt, bevor Sie einem Ordner vertrauen](/docs/de/permissions#what-runs-before-you-trust-a-folder) behandelt jede Art von Repository-Inhalt unter `-p` und wie Sie ihn fernhalten.

Dieses Beispiel führt eine einmalige Zusammenfassungsaufgabe im Bare-Modus aus und genehmigt das Read-Tool vorab, damit der Aufruf ohne Berechtigungsaufforderung abgeschlossen wird. Setzen Sie `ANTHROPIC_API_KEY` vor dem Ausführen, da der Bare-Modus Ihren Abonnement-Login nicht verwendet:

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

Im Bare-Modus liest Claude Code niemals OAuth-Anmeldedaten oder den System-Keychain. Für die Anthropic-API setzen Sie `ANTHROPIC_API_KEY` in der Umgebung mit einem Schlüssel, der in der [Claude Console](https://platform.claude.com) erstellt wurde, oder geben Sie einen `apiKeyHelper` in der `--settings`-JSON an. Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry lesen weiterhin ihre eigenen Anmeldedaten des Anbieters wie gewohnt.

Im Bare-Modus hat Claude Zugriff auf die Bash-, Dateilesungs- und Dateibearbeitungstools. Übergeben Sie jeden Kontext, den Sie benötigen, mit einem Flag:

| Zum Laden                  | Verwenden Sie                                           |
| -------------------------- | ------------------------------------------------------- |
| Systemanfrage-Ergänzungen  | `--append-system-prompt`, `--append-system-prompt-file` |
| Einstellungen              | `--settings <file-or-json>`                             |
| MCP-Server                 | `--mcp-config <file-or-json>`                           |
| Benutzerdefinierte Agenten | `--agents <json>`                                       |
| Ein Plugin                 | `--plugin-dir <path>`, `--plugin-url <url>`             |

<Note>
  `--bare` ist der empfohlene Modus für skriptgesteuerte und SDK-Aufrufe und wird in einer zukünftigen Version zum Standard für `-p`.
</Note>

<h3 id="background-tasks-at-exit">
  Hintergrundaufgaben beim Beenden
</h3>

Wenn Claude während einer `claude -p`-Ausführung eine [Hintergrund-Bash-Aufgabe](/docs/de/tools-reference#bash-tool-behavior) startet, beispielsweise einen Entwicklungsserver oder einen Watch-Build, wird diese Shell etwa fünf Sekunden nach der Rückgabe des endgültigen Ergebnisses durch Claude und dem Schließen von stdin beendet. Die Kulanzfrist ermöglicht es einer Aufgabe, die direkt nach dem Ergebnis endet, ihre Ausgabe noch zu liefern.

Wenn Claude einen Hintergrund-[Subagenten](/docs/de/sub-agents) oder Workflow startet, bleibt `claude -p` stattdessen offen, bis diese Arbeit abgeschlossen ist, da ihr Ergebnis Teil der endgültigen Ausgabe ist.

Standardmäßig endet das Warten nach 10 Minuten kontinuierlichen Leerlauf-Wartens, sodass ein feststeckender Subagent oder Workflow den Prozess nicht auf unbestimmte Zeit offen halten kann. An diesem Punkt stoppt Claude Code, was noch läuft, und verwirft sein Teilergebnis. Um die Obergrenze zu ändern, setzen Sie [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/de/env-vars), oder setzen Sie sie auf `0`, um ohne Obergrenze zu warten.

Wenn Claude einen [Monitor](/docs/de/tools-reference#monitor-tool)-Watch während einer `claude -p`-Ausführung startet, wartet Claude Code auf den Watch, bis er abläuft oder die Zehn-Minuten-Obergrenze das Warten beendet, je nachdem, was zuerst eintritt. Während es wartet, antwortet Claude weiterhin auf das, was der Watch meldet. Standardmäßig läuft ein Watch fünf Minuten ab, nachdem Claude ihn startet.

<h3 id="stop-a-run-with-sigterm">
  Beenden Sie eine Ausführung mit SIGTERM
</h3>

Wenn Sie eine `claude -p`-Ausführung mit SIGTERM beenden, beispielsweise mit `kill` oder von einem Prozessüberwacher, wird Claude Code mit Code 143 beendet. Claude Code lässt den laufenden Turn unvollständig und zeichnet kein Ergebnis dafür auf. Um den Turn stattdessen zu beenden, senden Sie SIGINT oder rufen Sie `interrupt()` des Agent SDK auf, bevor Sie den Prozess stoppen.

Bei SIGTERM beendet Claude Code den Prozessbaum aller noch laufenden Bash-Befehle. Claude Code führt dann [`SessionEnd`-Hooks](/docs/de/hooks#sessionend) aus und wird beendet. Während des Beendens startet Claude Code keinen neuen Tool-Aufruf, sendet keine neue Modellanfrage und führt keinen Hook außer `SessionEnd` aus. Wenn die Ausführung in der Mitte eines Befehls oder beim Warten auf eine Antwort auf eine Berechtigungsaufforderung war, als das Signal ankam, behandelt Claude Code diesen Schritt wie folgt:

* **Ausführung eines Befehls**: Claude Code zeichnet den Befehl als beendet in der Sitzung auf.
* **Warten auf eine Antwort auf eine Berechtigungsaufforderung**: Wenn Sie SIGTERM an den Prozess senden, lässt Claude Code die Aufforderung unbeantwortet. Wenn Ihr Programm die Sitzung durch das Agent SDK schließt, beendet das SDK Claudes Eingabe, bevor es ein Signal sendet, und Claude Code bricht die Aufforderung ab, sobald die Eingabe endet.

Wenn Sie die Sitzung [fortsetzen](#continue-conversations), setzt Claude Code den Turn fort, den SIGTERM unvollständig gelassen hat.

<h2 id="examples">
  Beispiele
</h2>

Diese Beispiele zeigen häufige CLI-Muster. Wenn ein Befehl eine Datei wie `auth.py` oder `build-error.txt` benennt, ersetzen Sie diese durch eine Datei aus Ihrem eigenen Projekt. Fügen Sie in CI oder anderen skriptgesteuerten Umgebungen [`--bare`](#start-faster-with-bare-mode) hinzu, damit Claude Code ohne Laden der Hooks, Plugins, automatischen Speicherung oder `CLAUDE.md` des Hosts startet.

<h3 id="pipe-data-through-claude">
  Daten durch Claude leiten
</h3>

Der nicht-interaktive Modus liest stdin, sodass Sie Daten wie bei jedem anderen Befehlszeilentool einleiten und die Antwort umleiten können.

Dieses Beispiel leitet ein Build-Protokoll in Claude ein und schreibt die Erklärung in eine Datei:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Mit `--output-format json` enthält die Antwort-Payload `total_cost_usd` und eine Kostenaufschlüsselung pro Modell, sodass skriptgesteuerte Aufrufer die Ausgaben verfolgen können, ohne das [Nutzungs-Dashboard](/docs/de/costs) zu konsultieren. Wenn Sie ein früheres Gespräch mit `--continue` oder `--resume` fortsetzen, meldet der Lauf die Gesamtsumme des Gesprächs, [einschließlich der Ausgaben früherer Läufe](/docs/de/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Beide Zahlen sind [clientseitige Schätzungen](/docs/de/agent-sdk/cost-tracking) und können sich von Ihrer tatsächlichen Rechnung unterscheiden.

<Note>
  Eingeleiter stdin ist auf 10 MB begrenzt. Wenn Sie die Grenze überschreiten, beendet sich Claude Code mit einer klaren Fehlermeldung und einem Nicht-Null-Status. Um mit größeren Eingaben zu arbeiten, schreiben Sie den Inhalt in eine Datei und verweisen Sie auf den Dateipfad in Ihrer Eingabeaufforderung, anstatt ihn einzuleiten.
</Note>

Wenn Claude Code stdin nicht lesen kann, beispielsweise weil der Prozess, der es gestartet hat, sein Ende getrennt hat, gibt Claude Code eine Warnung auf stderr aus und setzt die Eingabeaufforderung von der Befehlszeile fort. Vor v2.1.211 führte ein nicht lesbarer stdin unter Windows zum Absturz der Sitzung oder zum stillen Beenden ohne Ausgabe.

<h3 id="add-claude-to-a-build-script">
  Claude zu einem Build-Skript hinzufügen
</h3>

Sie können einen nicht-interaktiven Aufruf in einem Skript einbinden, um Claude als projektspezifischen Linter oder Reviewer zu verwenden.

Dieses `package.json`-Skript leitet den Diff gegen `main` in Claude ein und fordert ihn auf, Tippfehler zu melden. Das Einleiten des Diff bedeutet, dass Claude keine Bash-Berechtigung zum Lesen benötigt, und die maskierten doppelten Anführungszeichen halten das Skript portabel zu Windows:

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

Führen Sie es mit `npm run lint:claude` aus.

<h3 id="get-structured-output">
  Strukturierte Ausgabe abrufen
</h3>

Verwenden Sie `--output-format`, um zu steuern, wie Antworten zurückgegeben werden:

* `text` (Standard): einfache Textausgabe
* `json`: strukturiertes JSON mit Ergebnis, Sitzungs-ID und Metadaten
* `stream-json`: zeilengetrennte JSON für Echtzeit-Streaming

Dieses Beispiel gibt eine Projektzusammenfassung als JSON mit Sitzungsmetadaten zurück, wobei sich das Textergebnis im Feld `result` befindet:

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

Um eine Ausgabe zu erhalten, die einem bestimmten Schema entspricht, verwenden Sie `--output-format json` mit `--json-schema` und einer [JSON Schema](https://json-schema.org/)-Definition. Die Antwort enthält Metadaten über die Anfrage (Sitzungs-ID, Nutzung usw.) mit der strukturierten Ausgabe im Feld `structured_output`.

Dieses Beispiel extrahiert Funktionsnamen und gibt sie als Array von Zeichenketten zurück:

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

Wenn der Wert kein gültiges JSON Schema ist, beendet sich `claude` mit `Error: --json-schema is not a valid JSON Schema` gefolgt von der Diagnose des Validators. Claude Code akzeptiert Schemas, die das Schlüsselwort `format` verwenden, wie `"format": "email"`, behandelt aber `format` als Anmerkung und erzwingt es nicht. Vor v2.1.205 ignorierte Claude Code ein ungültiges Schema stillschweigend und gab unstrukturierten Text zurück, und behandelte jedes Schema, das `format` enthielt, als ungültig.

<Tip>
  Verwenden Sie ein Tool wie [jq](https://jqlang.org/) zum Analysieren der Antwort und zum Extrahieren bestimmter Felder:

  ```bash theme={null}
  # Extract the text result
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # Extract structured output
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  Antworten streamen
</h3>

Verwenden Sie `--output-format stream-json` mit `--verbose` und `--include-partial-messages`, um Token zu empfangen, während sie generiert werden. Jede Zeile ist ein JSON-Objekt, das ein Ereignis darstellt:

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

Die letzte Zeile des Streams ist eine `result`-Nachricht mit dem endgültigen Antworttext, den Kosten und den Sitzungsmetadaten.

Wenn Ihr Consumer den Stream langsam liest, wartet Claude Code darauf, dass die warteschlange Ausgabe abfließt, bevor es beendet wird, und skaliert das Warten mit der noch warteschlange Menge, begrenzt auf 30 Sekunden. Vor v2.1.214 war die Ausstiegswartzeit auf etwa zwei Sekunden begrenzt, was das Ende einer großen Antwort abschneiden konnte.

Das folgende Beispiel verwendet [jq](https://jqlang.org/) zum Filtern nach Text-Deltas und zum Anzeigen nur des Streaming-Texts. Das Flag `-r` gibt Rohzeichenketten aus (keine Anführungszeichen) und `-j` verbindet ohne Zeilenumbrüche, sodass Token kontinuierlich gestreamt werden:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Für programmgesteuertes Streaming mit Rückrufen und Nachrichtenobjekten siehe [Antworten in Echtzeit streamen](/docs/de/agent-sdk/streaming-output) in der Agent SDK-Dokumentation.

<h4 id="follow-subagent-messages">
  Subagenten-Nachrichten folgen
</h4>

Nachrichten von [Subagenten](/docs/de/sub-agents) erscheinen im Stream als `assistant`- und `user`-Nachrichten, deren `parent_tool_use_id`-Feld die ID des Tool-Aufrufs ist, der die Subagent spawnte. Nachrichten aus der Hauptkonversation tragen `null` in diesem Feld.

Die erste Nachricht von einer Subagent, die im [Vordergrund](/docs/de/sub-agents#run-subagents-in-foreground-or-background) läuft, ist eine `user`-Nachricht, die die Eingabeaufforderung trägt, die sie antreibt. Nach dieser ersten Nachricht gibt Claude Code aus:

* **Standardmäßig**: die Subagenten-`tool_use`- und `tool_result`-Blöcke.
* **Mit [`--forward-subagent-text`](/docs/de/cli-reference#cli-flags) oder [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](/docs/de/env-vars)**: auch die Subagenten-Text- und Thinking-Blöcke, damit Sie das Transkript jeder Subagent rekonstruieren können. Dies erfordert Claude Code v2.1.211 oder später.

Wenn Sie eine der beiden Optionen aktivieren, leitet Claude Code Nachrichten von [Subagenten auf jeder Verschachtelungstiefe](/docs/de/sub-agents#let-subagents-spawn-their-own-subagents) weiter, unabhängig davon, ob jede mit dem Agent-Tool oder als [gegabelter Skill](/docs/de/skills#run-skills-in-a-subagent) gestartet wurde. Nachrichten von Subagenten, die ein gegabelter Skill spawnt, und von gegabelten Skills, die in einer Subagent oder einem anderen gegabelten Skill gestartet werden, erfordern Claude Code v2.1.275 oder später. In `parent_tool_use_id` tragen die Nachrichten der verschachtelten Subagent die ID des Agent- oder Skill-Tool-Aufrufs, der sie gestartet hat, sodass Sie den vollständigen Verschachtelungsbaum durch Verfolgung dieser IDs rekonstruieren können. Vor v2.1.219 erschienen Nachrichten von verschachtelten Subagenten nicht im Stream.

Skills, die [in einer Subagent laufen](/docs/de/skills#run-skills-in-a-subagent), erscheinen im Stream auf die gleiche Weise: die erste Nachricht des gegabelten Skills ist eine `user`-Nachricht, die den Skill-Inhalt trägt, der den Lauf antreibt. Wenn Sie eine der beiden Optionen aktivieren, enthält der Stream auch die Text- und Thinking-Blöcke des gegabelten Skills. Vor v2.1.265 erschienen nur die `tool_use`- und `tool_result`-Blöcke eines gegabelten Skills im Stream.

<h4 id="handle-api-retries">
  API-Wiederholungen verarbeiten
</h4>

Wenn eine API-Anfrage mit einem wiederholbaren Fehler fehlschlägt, gibt Claude Code ein `system/api_retry`-Ereignis vor dem erneuten Versuch aus. Bei v2.1.246 oder später, wenn ein `401` oder `403` eine [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Anmeldedaten ablehnt, führt Claude Code die ersten beiden Wiederholungen stillschweigend ohne Ereignis durch, gibt dann das Ereignis wie gewohnt ab der dritten aufeinanderfolgenden Wiederholung aus. Die stillen Wiederholungen zählen immer noch zu `attempt`. Sie können das Ereignis verwenden, um Wiederholungsfortschritt in Ihrer eigenen Schnittstelle anzuzeigen.

| Feld             | Typ                | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ---------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`         | Nachrichtentyp                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `subtype`        | `"api_retry"`      | identifiziert dies als Wiederholungsereignis                                                                                                                                                                                                                                                                                                                                                                                            |
| `attempt`        | Ganzzahl           | aktuelle Versuchsnummer, beginnend bei 1                                                                                                                                                                                                                                                                                                                                                                                                |
| `max_retries`    | Ganzzahl           | insgesamt zulässige Wiederholungen für diese Fehlerursache, die weniger als das sitzungsweite Budget sein kann                                                                                                                                                                                                                                                                                                                          |
| `retry_delay_ms` | Ganzzahl           | Millisekunden bis zum nächsten Versuch                                                                                                                                                                                                                                                                                                                                                                                                  |
| `error_status`   | Ganzzahl oder null | HTTP-Statuscode des fehlgeschlagenen Versuchs oder `null`, wenn der Versuch keine HTTP-Antwort von der API erhielt                                                                                                                                                                                                                                                                                                                      |
| `no_response`    | Objekt, optional   | vorhanden nur, wenn der fehlgeschlagene Versuch [keine Antwortheader rechtzeitig](/docs/de/errors#no-response-from-api) erhielt. `waited_ms` ist, wie lange dieser Versuch wartete, und `retry_wait_ms` ist, wie lange die Wiederholung wartet. In diesen Ereignissen spiegelt `max_retries` die eine Wiederholung wider, die diese Ursache normalerweise erhält, nicht das sitzungsweite Budget. Erfordert Claude Code v2.1.261 oder später |
| `error`          | Zeichenkette       | Fehlerkategorie: `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error` oder `unknown`                                                                                                                                                                                   |
| `uuid`           | Zeichenkette       | eindeutige Ereigniskennung                                                                                                                                                                                                                                                                                                                                                                                                              |
| `session_id`     | Zeichenkette       | Sitzung, zu der das Ereignis gehört                                                                                                                                                                                                                                                                                                                                                                                                     |

<h4 id="read-session-metadata">
  Sitzungsmetadaten lesen
</h4>

Das `system/init`-Ereignis meldet Sitzungsmetadaten einschließlich des Modells, Tools, MCP-Server und geladener Plugins. Es ist das erste Ereignis im Stream, es sei denn, Startereignisse gehen ihm voraus:

* `plugin_install`-Ereignisse, wenn [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/de/env-vars) gesetzt ist.
* [`hook_started`-, `hook_progress`- und `hook_response`-Ereignisse](/docs/de/agent-sdk/typescript#sdkhookstartedmessage), während ein konfigurierter [`SessionStart`](/docs/de/hooks#sessionstart)- oder [`Setup`](/docs/de/hooks#setup)-Hook ausgeführt wird. Diese werden gestreamt, während der Hook sie erzeugt. Claude Code v2.1.169 bis v2.1.203 lieferte sie in einem Batch nach Abschluss des Hooks, immer noch vor `system/init`; v2.1.204 stellte die Live-Lieferung wieder her.

Das Ereignis enthält auch ein optionales Array `capabilities` von Zeichenketten, das die Protokollverhalten benennt, die diese Claude Code-Version implementiert, wie `interrupt_receipt_v1` oder `interrupt_cancel_queued_v1`. Überprüfen Sie es, um Funktionen zu erkennen, anstatt Versionsnummern zu vergleichen, und ignorieren Sie Werte, die Sie nicht erkennen. Das Feld erfordert Claude Code v2.1.205 oder später und fehlt in früheren Versionen. Siehe [`SDKSystemMessage`](/docs/de/agent-sdk/typescript#sdksystemmessage) für die Funktionsliste.

<h4 id="fail-ci-when-a-plugin-or-mcp-server-doesn’t-load">
  CI fehlschlagen lassen, wenn ein Plugin oder MCP-Server nicht geladen wird
</h4>

Verwenden Sie die Plugin-Felder im `system/init`-Ereignis, um ein Plugin zu erfassen, das nicht geladen wurde:

| Feld            | Typ   | Beschreibung                                                                                                                                                                                                                                                                                                              |
| --------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | Array | Plugins, die erfolgreich geladen wurden, jeweils mit `name` und `path`                                                                                                                                                                                                                                                    |
| `plugin_errors` | Array | Plugin-Ladefehler, jeweils mit `plugin`, `type` und `message`. Umfasst nicht erfüllte Abhängigkeitsversionen und `--plugin-dir`-Ladefehler wie einen fehlenden Pfad oder ein ungültiges Archiv. Betroffene Plugins werden herabgestuft und fehlen in `plugins`. Der Schlüssel wird weggelassen, wenn es keine Fehler gibt |

Verwenden Sie die MCP-Server-Felder auf die gleiche Weise. Wenn Sie [`--mcp-config`](/docs/de/cli-reference#cli-flags) mit `-p` übergeben, wartet Claude Code auf noch ausstehende Server, bevor der erste Zug ausgeführt wird, bis zum [`MCP_TIMEOUT`](/docs/de/env-vars)-Starttimeout, standardmäßig 30 Sekunden. Ein Remote-Server mit einer [zwischengespeicherten Tool-Liste](/docs/de/agent-sdk/mcp#connection-timing) überspringt das Warten, zeigt `pending` in `system/init` an und verbindet sich beim ersten Tool-Aufruf. Das Warten erfordert Claude Code v2.1.221 oder später.

Claude Code validiert jeden `--mcp-config`-Eintrag beim Start und überspringt Einträge, die die Validierung nicht bestehen, beispielsweise einen `url`-Eintrag ohne `type`. Der Lauf wird fortgesetzt und beendet sich sauber, überprüfen Sie also diese Felder, um einen Server zu erfassen, der nie geladen wurde:

| Feld                | Typ   | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_servers`       | Array | MCP-Server in der Sitzung, jeweils mit `name` und `status`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `mcp_server_errors` | Array | `--mcp-config`-Einträge, die durch Konfigurationsvalidierung übersprungen wurden, jeweils mit `name`, `type` und `message`. `type` ist eine Überspringungskategorie wie `unknown_type`, `url_missing_type`, `invalid_config` oder `reserved_name`; behandeln Sie Werte, die Sie nicht erkennen, als generisches Überspringen. Betroffene Server fehlen in `mcp_servers`. Der Schlüssel wird weggelassen, wenn es keine Fehler gibt, sodass ein CI-Gate bei einem nicht leeren Array fehlschlagen kann. Erfordert Claude Code v2.1.219 oder später |

Wenn Sie den Befehl von Hand in einem Terminal ausführen, gibt Claude Code auch eine Startwarnmeldung auf stderr aus, wie `Warning: 1 MCP server skipped due to invalid config:`, gefolgt vom Grund für jeden übersprungenen Eintrag. Wenn Sie stderr umleiten oder wenn ein Programm wie ein CI-Runner oder ein SDK-Host es erfasst, gibt Claude Code keine Warnung aus und meldet die übersprungenen Einträge nur im Feld `mcp_server_errors`. Die Warnung erfordert Claude Code v2.1.219 oder später.

<h4 id="track-plugin-installs">
  Plugin-Installationen verfolgen
</h4>

Wenn [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/de/env-vars) gesetzt ist, gibt Claude Code `system/plugin_install`-Ereignisse aus, während Marketplace-Plugins vor dem ersten Zug installiert werden. Verwenden Sie diese, um Installationsfortschritt in Ihrer eigenen Benutzeroberfläche anzuzeigen.

| Feld         | Typ                                                       | Beschreibung                                                                                                       |
| ------------ | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `type`       | `"system"`                                                | Nachrichtentyp                                                                                                     |
| `subtype`    | `"plugin_install"`                                        | identifiziert dies als Plugin-Installationsereignis                                                                |
| `status`     | `"started"`, `"installed"`, `"failed"` oder `"completed"` | `started` und `completed` rahmen die Gesamtinstallation ein; `installed` und `failed` melden einzelne Marketplaces |
| `name`       | Zeichenkette, optional                                    | Marketplace-Name, vorhanden bei `installed` und `failed`                                                           |
| `error`      | Zeichenkette, optional                                    | Fehlermeldung, vorhanden bei `failed`                                                                              |
| `uuid`       | Zeichenkette                                              | eindeutige Ereigniskennung                                                                                         |
| `session_id` | Zeichenkette                                              | Sitzung, zu der das Ereignis gehört                                                                                |

<h3 id="auto-approve-tools">
  Tools automatisch genehmigen
</h3>

Verwenden Sie `--allowedTools`, um Claude die Verwendung bestimmter Tools ohne Aufforderung zu ermöglichen. Dieses Beispiel führt eine Test-Suite aus und behebt Fehler, wobei Claude Bash-Befehle ausführen und Dateien lesen/bearbeiten kann, ohne um Genehmigung zu fragen:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Um einen Baseline für die gesamte Sitzung festzulegen, anstatt einzelne Tools aufzulisten, übergeben Sie einen [Berechtigungsmodus](/docs/de/permission-modes). Für `-p` ist der [integrierte Starterechtigungsmodus](/docs/de/permission-modes#which-mode-a-session-starts-in) auf jedem Plan Manual, übergeben Sie also den Berechtigungsmodus, den Sie möchten:

* **`auto`**: Übergeben Sie `--permission-mode auto`, um einen Klassifizierer die meisten Aktionen überprüfen zu lassen, anstatt Sie
* **`dontAsk`**: Claude Code verweigert jeden Aufruf, der sonst eine Aufforderung auslösen würde, was für gesperrte CI-Läufe nützlich ist. Aktionen, die im Manual-Modus keine Genehmigung benötigen, werden immer noch ausgeführt, wie Dateilesevorgänge in Ihren Arbeitsverzeichnissen und dem [schreibgeschützten Befehlssatz](/docs/de/permissions#read-only-commands), ebenso wie Aktionen, die Ihre `--allowedTools`-Einträge oder `permissions.allow`-Regeln abdecken. `AskUserQuestion`, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden verweigert, auch wenn eine Allow-Regel passt
* **`acceptEdits`**: Claude schreibt Dateien ohne Aufforderung, und Claude Code genehmigt automatisch häufige Dateisystembefehle wie `mkdir`, `touch`, `mv` und `cp`. Die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves), gelten immer noch. Abgesehen vom schreibgeschützten Befehlssatz benötigen andere Shell-Befehle und Netzwerkanfragen immer noch einen `--allowedTools`-Eintrag oder eine `permissions.allow`-Regel. Siehe [was `acceptEdits` automatisch genehmigt](/docs/de/permission-modes#auto-approve-file-edits-with-acceptedits-mode) für die vollständige Liste

Dieses Beispiel wendet Lint-Fixes mit `acceptEdits` als Baseline an:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="turn-off-permission-prompts-in-unattended-runs">
  Berechtigungsaufforderungen in unbeaufsichtigten Läufen ausschalten
</h3>

Übergeben Sie `--permission-prompts none`, wenn niemand verfügbar ist, um Berechtigungsaufforderungen zu beantworten, beispielsweise in einem geplanten Job. Das Flag ist am wichtigsten, wenn Ihr Lauf einen Berechtigungshost hat: eine Agent SDK-App mit einem [`canUseTool`-Rückruf](/docs/de/agent-sdk/user-input) oder ein MCP-Tool, das Sie mit [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) übergeben. Ohne das Flag wartet Ihr Lauf darauf, dass dieser Host jede Berechtigungsanfrage beantwortet.

Mit dem Flag konsultiert Ihr Lauf den Host nicht und wartet nicht auf ihn. Alles, das eine Aufforderung auslösen würde, wird verweigert, es sei denn, ein `PermissionRequest`-Hook erlaubt es, Claude wird mitgeteilt, dass niemand die Anfrage genehmigen kann und nicht, sie erneut zu versuchen, und der Lauf wird fortgesetzt. In einem `-p`-Lauf ohne Host werden diese Anfragen ohnehin verweigert, und das Flag teilt Claude auch mit, sie nicht erneut zu versuchen. Berechtigungsregeln, [`PermissionRequest`-Hooks](/docs/de/hooks#permissionrequest) und der Berechtigungsmodus, den Sie festlegen, entscheiden zuerst jeden Aufruf; Claude Code verweigert nur die Anfragen, die nichts anderes löst.

Dieses Beispiel führt eine unbeaufsichtigte Aufgabe im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) aus. Der Klassifizierer überprüft jede Aktion wie gewohnt, und Claude Code verweigert alles, das auf eine Aufforderung zurückfallen würde:

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

Mit `--permission-prompts none` entfernt Claude Code die Tools, die eine Antwort von einer Person benötigen, wie [`AskUserQuestion`](/docs/de/tools-reference#askuserquestion-tool-behavior), sodass Claude sie nicht aufrufen kann. Jede [MCP-Elicitierungsanfrage](/docs/de/mcp#respond-to-mcp-elicitation-requests), die kein [`Elicitation`-Hook](/docs/de/hooks#elicitation) beantwortet, wird storniert.

Mit `--output-format stream-json` erscheinen Ablehnungen als `permission_denied`-Systemnachrichten, und die endgültige Ergebnismeldung listet sie in `permission_denials` auf.

<Note>
  Das Flag `--permission-prompts` erfordert Claude Code v2.1.259 oder später. Frühere Versionen lehnen es mit einem Fehler für unbekannte Optionen ab.
</Note>

<h3 id="create-a-commit">
  Einen Commit erstellen
</h3>

Dieses Beispiel überprüft bereitgestellte Änderungen und erstellt einen Commit mit einer angemessenen Nachricht:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

Das Flag `--allowedTools` verwendet [Berechtigungsregelsyntax](/docs/de/settings-reference#permission-rule-syntax). Das nachfolgende ` *` ermöglicht Präfix-Matching, sodass `Bash(git diff *)` jeden Befehl erlaubt, der mit `git diff` beginnt. Das Leerzeichen vor `*` ist wichtig: ohne es würde `Bash(git diff*)` auch `git diff-index` entsprechen.

<Note>
  Benutzer-aufgerufene [skills](/docs/de/skills) und benutzerdefinierte Befehle funktionieren im `-p`-Modus. Fügen Sie `/skill-name` in die Eingabeaufforderungszeichenkette ein und Claude Code erweitert sie vor dem Ausführen. Integrierte Befehle, die nur in der Terminalschnittstelle ausgeführt werden, wie `/login`, sind im `-p`-Modus nicht verfügbar. `/model`, `/effort`, `/fast`, `/color` und `/rename` akzeptieren den Wert als Argument, zum Beispiel `/model sonnet`, und `/mcp` ohne Argument gibt eine Textzusammenfassung des Serverstatus aus; diese Formen erfordern Claude Code v2.1.205 oder später und folgen den [Verfügbarkeitshinweisen](/docs/de/commands#all-commands) jedes Befehls. Um eine Einstellung zu ändern, übergeben Sie `key=value` an `/config`, zum Beispiel `/config thinking=false`. `/output-style <style>` wechselt [Ausgabestile](/docs/de/output-styles) und `/output-style` allein listet sie auf. Erfordert Claude Code v2.1.269 oder später.
</Note>

<h3 id="customize-the-system-prompt">
  System-Eingabeaufforderung anpassen
</h3>

Verwenden Sie `--append-system-prompt`, um Anweisungen hinzuzufügen und dabei das Standardverhalten von Claude Code beizubehalten. Dieses Beispiel leitet einen PR-Diff an Claude weiter und weist ihn an, auf Sicherheitslücken zu überprüfen. Speichern Sie es als Shell-Skript, zum Beispiel `review.sh`:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Im Skript steht `"$1"` für das erste Argument, das Sie in der Befehlszeile übergeben. Führen Sie `bash review.sh 123` aus und die Shell ersetzt `"$1"` durch `123`, sodass das Skript den Diff für PR 123 abruft. Claude Code gibt die Überprüfung als JSON aus, wobei sich der Text im Feld `result` befindet.

Siehe [System-Eingabeaufforderungs-Flags](/docs/de/cli-reference#system-prompt-flags) für weitere Optionen, einschließlich `--system-prompt`, um die Standardeingabeaufforderung vollständig zu ersetzen.

<h3 id="continue-conversations">
  Gespräche fortsetzen
</h3>

Verwenden Sie `--continue`, um das neueste Gespräch fortzusetzen, oder `--resume` mit einer Sitzungs-ID, um ein bestimmtes Gespräch fortzusetzen. Bei Claude Code v2.1.257 oder später, wenn Sie `--continue` übergeben, öffnet Claude Code eine [Hintergrund-Sitzung](/docs/de/sessions#resume-a-session), die beendet ist, aber nicht eine, die noch läuft. Dieses Beispiel führt eine Überprüfung durch und sendet dann Folgeeingabeaufforderungen:

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

Wenn Sie mehrere Gespräche führen, erfassen Sie die Sitzungs-ID, um ein bestimmtes Gespräch fortzusetzen:

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

Sie können die beiden Befehle aus verschiedenen Verzeichnissen ausführen: Claude Code [findet die Sitzung anhand ihrer ID](/docs/de/sessions#resume-a-session) in jedem Projekt auf diesem Computer. Vor v2.1.223 suchte Claude Code die ID nur im aktuellen Projektverzeichnis und seinen Git-Worktrees, sodass Sie beide Befehle aus demselben Verzeichnis ausführen mussten.

Anstelle der Sitzungs-ID können Sie `--resume` den absoluten Pfad zu einer Sitzungs-[Transkriptdatei](/docs/de/sessions#where-transcripts-are-stored) im `.jsonl`-Format übergeben, und Claude Code setzt das in dieser Datei gespeicherte Gespräch fort.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Agent SDK Schnellstart](/docs/de/agent-sdk/quickstart): Erstellen Sie Ihren ersten Agent mit Python oder TypeScript
* [CLI-Referenz](/docs/de/cli-reference): alle CLI-Flags und Optionen
* [GitHub Actions](/docs/de/github-actions): Verwenden Sie das Agent SDK in GitHub-Workflows
* [GitLab CI/CD](/docs/de/gitlab-ci-cd): Verwenden Sie das Agent SDK in GitLab-Pipelines
