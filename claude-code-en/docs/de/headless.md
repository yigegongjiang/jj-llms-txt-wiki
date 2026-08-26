> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code programmgesteuert ausführen

> Verwenden Sie das Agent SDK, um Claude Code programmgesteuert über die CLI, Python oder TypeScript auszuführen.

Das [Agent SDK](/docs/de/agent-sdk/overview) bietet Ihnen die gleichen Tools, die Agent-Schleife und das Kontextmanagement, die Claude Code antreiben. Es ist als CLI für Skripte und CI/CD verfügbar oder als [Python](/docs/de/agent-sdk/python)- und [TypeScript](/docs/de/agent-sdk/typescript)-Pakete für vollständige programmgesteuerte Kontrolle.

Um Claude Code im nicht-interaktiven Modus auszuführen, übergeben Sie `-p` mit Ihrer Eingabeaufforderung und allen [CLI-Optionen](/docs/de/cli-reference):

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Diese Seite behandelt die Verwendung des Agent SDK über die CLI (`claude -p`). Für die Python- und TypeScript-SDK-Pakete mit strukturierten Ausgaben, Tool-Genehmigungsrückrufen und nativen Nachrichtenobjekten siehe die [vollständige Agent SDK-Dokumentation](/docs/de/agent-sdk/overview).

<h2 id="basic-usage">
  Grundlegende Verwendung
</h2>

Fügen Sie das Flag `-p` (oder `--print`) zu jedem `claude`-Befehl hinzu, um ihn nicht interaktiv auszuführen. Alle [CLI-Optionen](/docs/de/cli-reference) funktionieren mit `-p`, einschließlich:

* `--continue` zum [Fortsetzen von Gesprächen](#continue-conversations)
* `--allowedTools` zum [automatischen Genehmigen von Tools](#auto-approve-tools)
* `--output-format` für [strukturierte Ausgabe](#get-structured-output)

Dieses Beispiel stellt Claude eine Frage zu Ihrer Codebasis und gibt die Antwort aus:

```bash theme={null}
claude -p "What does the auth module do?"
```

<h3 id="start-faster-with-bare-mode">
  Schneller starten mit Bare-Modus
</h3>

Fügen Sie `--bare` hinzu, um die Startzeit zu verkürzen, indem Sie die automatische Erkennung von hooks, skills, plugins, MCP-Servern, automatischem Speicher und CLAUDE.md überspringen. Ohne diese Option lädt `claude -p` den gleichen [Kontext](/docs/de/how-claude-code-works#the-context-window), den eine interaktive Sitzung hätte, einschließlich alles, was im Arbeitsverzeichnis oder in `~/.claude` konfiguriert ist.

Der Bare-Modus ist nützlich für CI und Skripte, bei denen Sie auf jedem Computer das gleiche Ergebnis benötigen. Ein hook in der `~/.claude` eines Teamkollegen oder ein MCP-Server in der `.mcp.json` des Projekts werden nicht ausgeführt, da der Bare-Modus diese nie liest. Nur Flags, die Sie explizit übergeben, haben Auswirkungen.

Dieses Beispiel führt eine einmalige Zusammenfassungsaufgabe im Bare-Modus aus und genehmigt das Read-Tool vorab, damit der Aufruf ohne Berechtigungsaufforderung abgeschlossen wird:

```bash theme={null}
claude --bare -p "Summarize this file" --allowedTools "Read"
```

Im Bare-Modus hat Claude Zugriff auf die Bash-, Dateilesungs- und Dateibearbeitungstools. Übergeben Sie jeden Kontext, den Sie benötigen, mit einem Flag:

| Zum Laden                  | Verwenden Sie                                           |
| -------------------------- | ------------------------------------------------------- |
| Systemanfrage-Ergänzungen  | `--append-system-prompt`, `--append-system-prompt-file` |
| Einstellungen              | `--settings <file-or-json>`                             |
| MCP-Server                 | `--mcp-config <file-or-json>`                           |
| Benutzerdefinierte Agenten | `--agents <json>`                                       |
| Ein Plugin                 | `--plugin-dir <path>`, `--plugin-url <url>`             |

Der Bare-Modus überspringt OAuth und Keychain-Lesevorgänge. Die Anthropic-Authentifizierung muss von `ANTHROPIC_API_KEY` oder einem `apiKeyHelper` in der an `--settings` übergebenen JSON stammen. Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry verwenden ihre üblichen Anmeldedaten des Anbieters.

<Note>
  `--bare` ist der empfohlene Modus für skriptgesteuerte und SDK-Aufrufe und wird in einer zukünftigen Version zum Standard für `-p`.
</Note>

<h3 id="background-tasks-at-exit">
  Hintergrundaufgaben beim Beenden
</h3>

Wenn Claude während einer `claude -p`-Ausführung eine [Hintergrund-Bash-Aufgabe](/docs/de/tools-reference#bash-tool-behavior) startet, beispielsweise einen Entwicklungsserver oder einen Watch-Build, wird diese Aufgabe etwa fünf Sekunden nach der Rückgabe des endgültigen Ergebnisses durch Claude und dem Schließen von stdin beendet. Die Kulanzfrist ermöglicht es einer Aufgabe, die direkt nach dem Ergebnis endet, ihre Ausgabe noch zu liefern. Vor v2.1.163 würde ein nie endender Hintergrundprozess die `claude -p`-Invokation auf unbestimmte Zeit offen halten.

Hintergrund-[Subagenten](/docs/de/sub-agents) und Workflows sind von der fünfsekündigen Kulanzfrist ausgenommen, da ihr Ergebnis Teil der endgültigen Ausgabe ist, daher wartet `claude -p` auf deren Abschluss. Ab v2.1.182 ist dieses Warten standardmäßig auf zehn Minuten begrenzt, damit ein feststeckender Hintergrund-Agent den Prozess nicht auf unbestimmte Zeit offen halten kann. Passen Sie die Obergrenze mit [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/de/env-vars) an, oder setzen Sie sie auf `0`, um unbegrenzt zu warten.

<h2 id="examples">
  Beispiele
</h2>

Diese Beispiele zeigen häufige CLI-Muster. Für CI und andere skriptgesteuerte Aufrufe fügen Sie [`--bare`](#start-faster-with-bare-mode) hinzu, damit sie nicht abhängig von lokalen Konfigurationen sind.

<h3 id="pipe-data-through-claude">
  Daten durch Claude leiten
</h3>

Der nicht-interaktive Modus liest stdin, sodass Sie Daten wie bei jedem anderen Befehlszeilentool einleiten und die Antwort umleiten können.

Dieses Beispiel leitet ein Build-Protokoll in Claude ein und schreibt die Erklärung in eine Datei:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Mit `--output-format json` enthält die Antwort-Payload `total_cost_usd` und eine Kostenaufschlüsselung pro Modell, sodass skriptgesteuerte Aufrufer die Ausgaben pro Aufruf verfolgen können, ohne das [Nutzungs-Dashboard](/docs/de/costs) zu konsultieren.

<Note>
  Ab Claude Code v2.1.128 ist eingeleiter stdin auf 10 MB begrenzt. Wenn Sie die Grenze überschreiten, beendet sich Claude Code mit einer klaren Fehlermeldung und einem Nicht-Null-Status. Um mit größeren Eingaben zu arbeiten, schreiben Sie den Inhalt in eine Datei und verweisen Sie auf den Dateipfad in Ihrer Eingabeaufforderung, anstatt ihn einzuleiten.
</Note>

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
  Verwenden Sie ein Tool wie [jq](https://jqlang.github.io/jq/), um die Antwort zu analysieren und bestimmte Felder zu extrahieren:

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

Die letzte Zeile des Streams ist eine `result`-Nachricht mit dem endgültigen Antworttext, den Kosten und den Sitzungsmetadaten. Vor v2.1.208 konnte das Piping einer großen Antwort die letzte Zeile abschneiden und die `result`-Nachricht weglassen.

Das folgende Beispiel verwendet [jq](https://jqlang.github.io/jq/), um nach Text-Deltas zu filtern und nur den Streaming-Text anzuzeigen. Das Flag `-r` gibt Rohzeichenketten aus (keine Anführungszeichen) und `-j` verbindet ohne Zeilenumbrüche, sodass Token kontinuierlich gestreamt werden:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Wenn eine API-Anfrage mit einem wiederholbaren Fehler fehlschlägt, gibt Claude Code ein `system/api_retry`-Ereignis vor dem erneuten Versuch aus. Sie können dies verwenden, um Wiederholungsfortschritt anzuzeigen oder benutzerdefinierte Backoff-Logik zu implementieren.

| Feld             | Typ                | Beschreibung                                                                                                                                                                                             |
| ---------------- | ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`         | Nachrichtentyp                                                                                                                                                                                           |
| `subtype`        | `"api_retry"`      | identifiziert dies als Wiederholungsereignis                                                                                                                                                             |
| `attempt`        | Ganzzahl           | aktuelle Versuchsnummer, beginnend bei 1                                                                                                                                                                 |
| `max_retries`    | Ganzzahl           | insgesamt zulässige Wiederholungen                                                                                                                                                                       |
| `retry_delay_ms` | Ganzzahl           | Millisekunden bis zum nächsten Versuch                                                                                                                                                                   |
| `error_status`   | Ganzzahl oder null | HTTP-Statuscode oder `null` für Verbindungsfehler ohne HTTP-Antwort                                                                                                                                      |
| `error`          | Zeichenkette       | Fehlerkategorie: `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens` oder `unknown` |
| `uuid`           | Zeichenkette       | eindeutige Ereigniskennung                                                                                                                                                                               |
| `session_id`     | Zeichenkette       | Sitzung, zu der das Ereignis gehört                                                                                                                                                                      |

Das `system/init`-Ereignis meldet Sitzungsmetadaten einschließlich des Modells, Tools, MCP-Server und geladener Plugins. Es ist das erste Ereignis im Stream, es sei denn, Startereignisse gehen ihm voraus:

* `plugin_install`-Ereignisse, wenn [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/de/env-vars) gesetzt ist.
* [`hook_started`-, `hook_progress`- und `hook_response`-Ereignisse](/docs/de/agent-sdk/typescript#sdkhookstartedmessage), während ein konfigurierter [`SessionStart`](/docs/de/hooks#sessionstart)- oder [`Setup`](/docs/de/hooks#setup)-Hook ausgeführt wird. Diese werden gestreamt, während der Hook sie erzeugt. Claude Code v2.1.169 bis v2.1.203 lieferte sie in einem Batch nach Abschluss des Hooks, immer noch vor `system/init`; v2.1.204 stellte die Live-Lieferung wieder her.

Das Ereignis enthält auch ein optionales Array `capabilities` von Zeichenketten, das die Protokollverhalten benennt, die diese Claude Code-Version implementiert, wie `interrupt_receipt_v1`. Überprüfen Sie es, um Funktionen zu erkennen, anstatt Versionsnummern zu vergleichen, und ignorieren Sie Werte, die Sie nicht erkennen. Das Feld erfordert Claude Code v2.1.205 oder später und fehlt in früheren Versionen. Siehe [`SDKSystemMessage`](/docs/de/agent-sdk/typescript#sdksystemmessage) für die Funktionsliste.

Verwenden Sie die Plugin-Felder, um CI fehlschlagen zu lassen, wenn ein Plugin nicht geladen wurde:

| Feld            | Typ   | Beschreibung                                                                                                                                                                                                                                                                                                              |
| --------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | Array | Plugins, die erfolgreich geladen wurden, jeweils mit `name` und `path`                                                                                                                                                                                                                                                    |
| `plugin_errors` | Array | Plugin-Ladefehler, jeweils mit `plugin`, `type` und `message`. Umfasst nicht erfüllte Abhängigkeitsversionen und `--plugin-dir`-Ladefehler wie einen fehlenden Pfad oder ein ungültiges Archiv. Betroffene Plugins werden herabgestuft und fehlen in `plugins`. Der Schlüssel wird weggelassen, wenn es keine Fehler gibt |

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

Für programmgesteuertes Streaming mit Rückrufen und Nachrichtenobjekten siehe [Antworten in Echtzeit streamen](/docs/de/agent-sdk/streaming-output) in der Agent SDK-Dokumentation.

<h3 id="auto-approve-tools">
  Tools automatisch genehmigen
</h3>

Verwenden Sie `--allowedTools`, um Claude die Verwendung bestimmter Tools ohne Aufforderung zu ermöglichen. Dieses Beispiel führt eine Test-Suite aus und behebt Fehler, wobei Claude Bash-Befehle ausführen und Dateien lesen/bearbeiten kann, ohne um Genehmigung zu fragen:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Um einen Baseline für die gesamte Sitzung festzulegen, anstatt einzelne Tools aufzulisten, übergeben Sie einen [Berechtigungsmodus](/docs/de/permission-modes). `dontAsk` verweigert alles, das nicht in Ihren `permissions.allow`-Regeln oder dem [schreibgeschützten Befehlssatz](/docs/de/permissions#read-only-commands) enthalten ist, was für gesperrte CI-Läufe nützlich ist. `AskUserQuestion`, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden verweigert, auch wenn eine Allow-Regel passt.

`acceptEdits` ermöglicht Claude, Dateien ohne Aufforderung zu schreiben, und genehmigt auch automatisch häufige Dateisystembefehle wie `mkdir`, `touch`, `mv` und `cp`. Andere Shell-Befehle und Netzwerkanfragen benötigen immer noch einen `--allowedTools`-Eintrag oder eine `permissions.allow`-Regel, andernfalls wird der Lauf abgebrochen, wenn einer versucht wird:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="create-a-commit">
  Einen Commit erstellen
</h3>

Dieses Beispiel überprüft bereitgestellte Änderungen und erstellt einen Commit mit einer angemessenen Nachricht:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

Das Flag `--allowedTools` verwendet [Berechtigungsregelsyntax](/docs/de/settings#permission-rule-syntax). Das nachfolgende ` *` ermöglicht Präfix-Matching, sodass `Bash(git diff *)` jeden Befehl erlaubt, der mit `git diff` beginnt. Das Leerzeichen vor `*` ist wichtig: ohne es würde `Bash(git diff*)` auch `git diff-index` entsprechen.

<Note>
  Benutzer-aufgerufene [skills](/docs/de/skills) und benutzerdefinierte Befehle funktionieren im `-p`-Modus: Fügen Sie `/skill-name` in die Eingabeaufforderungszeichenkette ein und Claude Code erweitert sie vor dem Ausführen. Integrierte Befehle, die nur in der Terminalschnittstelle ausgeführt werden, wie `/login`, sind im `-p`-Modus nicht verfügbar. `/model`, `/effort`, `/fast`, `/color` und `/rename` akzeptieren den Wert als Argument, zum Beispiel `/model sonnet`, und `/mcp` ohne Argument gibt eine Textzusammenfassung des Serverstatus aus; diese Formen erfordern Claude Code v2.1.205 oder später und folgen den [Verfügbarkeitshinweisen](/docs/de/commands#all-commands) jedes Befehls. Um eine Einstellung von einer `-p`-Aufruf zu ändern, übergeben Sie `key=value` an `/config`, zum Beispiel `/config thinking=false`.
</Note>

<h3 id="customize-the-system-prompt">
  System-Eingabeaufforderung anpassen
</h3>

Verwenden Sie `--append-system-prompt`, um Anweisungen hinzuzufügen und dabei das Standardverhalten von Claude Code beizubehalten. Dieses Beispiel leitet einen PR-Diff an Claude weiter und weist ihn an, auf Sicherheitslücken zu überprüfen:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Siehe [System-Eingabeaufforderungs-Flags](/docs/de/cli-reference#system-prompt-flags) für weitere Optionen, einschließlich `--system-prompt`, um die Standardeingabeaufforderung vollständig zu ersetzen.

<h3 id="continue-conversations">
  Gespräche fortsetzen
</h3>

Verwenden Sie `--continue`, um das neueste Gespräch fortzusetzen, oder `--resume` mit einer Sitzungs-ID, um ein bestimmtes Gespräch fortzusetzen. Dieses Beispiel führt eine Überprüfung durch und sendet dann Folgeeingabeaufforderungen:

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

Führen Sie beide Befehle aus demselben Verzeichnis aus: Die Sitzungs-ID-Suche ist auf das aktuelle Projektverzeichnis und seine Git-Worktrees beschränkt. Siehe [Eine Sitzung fortsetzen](/docs/de/sessions#resume-a-session) für die vollständigen Bereichsregeln.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Agent SDK Schnellstart](/docs/de/agent-sdk/quickstart): Erstellen Sie Ihren ersten Agent mit Python oder TypeScript
* [CLI-Referenz](/docs/de/cli-reference): alle CLI-Flags und Optionen
* [GitHub Actions](/docs/de/github-actions): Verwenden Sie das Agent SDK in GitHub-Workflows
* [GitLab CI/CD](/docs/de/gitlab-ci-cd): Verwenden Sie das Agent SDK in GitLab-Pipelines
