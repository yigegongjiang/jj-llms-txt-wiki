> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrieren Sie Subagenten im großen Maßstab mit dynamischen Workflows

> Dynamische Workflows orchestrieren viele Subagenten aus einem Skript, das Claude schreibt und das Sie erneut ausführen können. Verwenden Sie sie für Codebase-Audits, große Migrationen und überprüfte Recherchen.

<Note>
  Dynamische Workflows erfordern Claude Code v2.1.154 oder später und sind auf allen bezahlten Plänen, mit Anthropic API-Zugriff und auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry verfügbar. Aktivieren Sie sie auf Pro über die Zeile „Dynamic workflows" in `/config`.
</Note>

Ein dynamischer Workflow ist ein JavaScript-Skript, das [Subagenten](/docs/de/sub-agents) im großen Maßstab orchestriert. Claude schreibt das Skript für die Aufgabe, die Sie beschreiben, und eine Laufzeit führt es im Hintergrund aus, während Ihre Sitzung reaktionsschnell bleibt.

Greifen Sie zu einem Workflow, wenn eine Aufgabe mehr Agenten benötigt, als ein Gespräch koordinieren kann, oder wenn Sie die Orchestrierung als Skript codifizieren möchten, das Sie lesen und erneut ausführen können. Beispiele sind eine codebase-weite Fehlersuche, eine 500-Datei-Migration, eine Forschungsfrage, die Quellen gegeneinander überprüft, und ein schwieriger Plan, der aus mehreren unabhängigen Blickwinkeln entworfen werden sollte, bevor Sie sich auf einen einigen.

<h2 id="when-to-use-a-workflow">
  Wann Sie einen Workflow verwenden
</h2>

[Subagenten](/docs/de/sub-agents), [Skills](/docs/de/skills), [Agent-Teams](/docs/de/agent-teams) und Workflows können alle eine mehrstufige Aufgabe ausführen. Der Unterschied liegt darin, wer den Plan hält:

|                                                   | Subagenten                           | Skills                               | Agent-Teams                                      | Workflows                                        |
| :------------------------------------------------ | :----------------------------------- | :----------------------------------- | :----------------------------------------------- | :----------------------------------------------- |
| Was es ist                                        | Ein Worker-Claude, den Sie spawnen   | Anweisungen, die Claude befolgt      | Ein Lead-Agent, der Peer-Sitzungen beaufsichtigt | Ein Skript, das die Laufzeit ausführt            |
| Wer entscheidet, was als nächstes ausgeführt wird | Claude, Zug um Zug                   | Claude, nach der Eingabeaufforderung | Der Lead-Agent, Zug um Zug                       | Das Skript                                       |
| Wo Zwischenergebnisse leben                       | Claudes Kontextfenster               | Claudes Kontextfenster               | Eine gemeinsame Aufgabenliste                    | Skriptvariablen                                  |
| Was wiederholbar ist                              | Die Worker-Definition                | Die Anweisungen                      | Die Team-Definition                              | Die Orchestrierung selbst                        |
| Skalierung                                        | Ein paar delegierte Aufgaben pro Zug | Gleich wie Subagenten                | Eine Handvoll langfristiger Peers                | Dutzende bis Hunderte von Agenten pro Ausführung |
| Unterbrechung                                     | Startet den Zug neu                  | Startet den Zug neu                  | Teammates führen weiter aus                      | Wiederaufnehmbar in derselben Sitzung            |

Ein Workflow verschiebt den Plan in Code. Bei Subagenten, Skills und Agent-Teams ist Claude der Orchestrator: Er entscheidet Zug um Zug, was als nächstes gespawnt oder zugewiesen werden soll, und jedes Ergebnis landet in einem Kontextfenster. Ein Workflow-Skript hält die Schleife, die Verzweigung und die Zwischenergebnisse selbst, sodass Claudes Kontext nur die endgültige Antwort enthält.

Das Verschieben des Plans in Code ermöglicht es einem Workflow auch, ein wiederholbares Qualitätsmuster anzuwenden, nicht nur mehr Agenten auszuführen: Er kann unabhängige Agenten die Ergebnisse des anderen gegnerisch überprüfen lassen, bevor sie gemeldet werden, oder einen Plan aus mehreren Blickwinkeln entwerfen und sie gegeneinander abwägen, sodass Sie ein vertrauenswürdigeres Ergebnis als einen einzelnen Durchgang erhalten.

<h2 id="run-a-bundled-workflow">
  Führen Sie einen gebündelten Workflow aus
</h2>

Die schnellste Möglichkeit, einen Workflow in Aktion zu sehen, ist die Ausführung von `/deep-research`, dem [integrierten Workflow](#bundled-workflows), den Claude Code zum Untersuchen einer Frage über viele Quellen hinweg enthält. Sie sehen Agenten, die im Hintergrund eine Reihe von Phasen durcharbeiten, während Ihre Sitzung frei bleibt, und erhalten am Ende einen Bericht statt eines Zug-für-Zug-Transkripts.

<Steps>
  <Step title="Führen Sie den Workflow aus">
    Führen Sie `/deep-research` mit einer Frage aus, die Sie untersuchen möchten. Es verteilt Websuchen über mehrere Blickwinkel, ruft die gefundenen Quellen ab und überprüft sie gegenseitig, und synthetisiert einen zitierten Bericht.

    ```text theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Erlauben Sie Workflows">
    Claude Code fragt, ob der Workflow erlaubt werden soll. Wählen Sie **Ja**, um fortzufahren. Die genaue Eingabeaufforderung hängt von Ihrem Berechtigungsmodus ab. Siehe [Genehmigen Sie den Plan, bevor er ausgeführt wird](#approve-the-plan-before-it-runs) für die Optionen pro Modus.
  </Step>

  <Step title="Überwachen Sie den Fortschritt">
    Die Ausführung startet im Hintergrund. Führen Sie `/workflows` aus, verwenden Sie die Pfeiltasten, um die Ausführung auszuwählen, und drücken Sie Enter, um die Fortschrittsansicht zu öffnen:

    ```text theme={null}
    /workflows
    ```

    Die Ansicht zeigt jede Phase mit ihrer Agentenzahl, Gesamttoken und verstrichener Zeit. Führen Sie einen Drilldown in jede Phase durch, um ihre Agenten und die Ergebnisse der einzelnen Agenten anzuzeigen. Siehe [Überwachen Sie die Ausführung](#watch-the-run) für den vollständigen Satz von Steuerelementen.

    Sie können auch über das Aufgabenpanel unter dem Eingabefeld beobachten: Eine einzeilige Fortschrittsübersicht wird dort angezeigt, während die Ausführung läuft. Drücken Sie die Abwärts-Taste, um es zu fokussieren, dann Enter, um es zu erweitern.
  </Step>

  <Step title="Lesen Sie den Bericht">
    Wenn die Ausführung abgeschlossen ist, landet der Bericht in Ihrer Sitzung. Er zitiert die Quellen, aus denen jeder Anspruch stammt, wobei Ansprüche, die die Überprüfung nicht überlebt haben, bereits gefiltert sind.

    Ab v2.1.196 listet der Bericht einen Anspruch als unverified auf, wenn die Verifier-Agenten einen Anspruch nicht überprüfen können, z. B. nach einem Rate Limit oder API-Fehler, anstatt ihn als widerlegt zu zählen.
  </Step>
</Steps>

Um einen Workflow für Ihre eigene Aufgabe auszuführen, [lassen Sie Claude einen schreiben](#have-claude-write-a-workflow), und sobald eine Ausführung das tut, was Sie wollten, können Sie [ihn speichern](#save-the-workflow-for-reuse) als Befehl Ihres eigenen.

<h3 id="bundled-workflows">
  Gebündelte Workflows
</h3>

Claude Code enthält `/deep-research` als integrierten Workflow:

| Befehl                      | Was er tut                                                                                                                                                                                                                                                                                                                                                           |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | Verteilt Websuchen zu einer Frage über mehrere Blickwinkel, ruft die gefundenen Quellen ab und überprüft sie gegenseitig, stimmt über jeden Anspruch ab und gibt einen zitierten Bericht mit Ansprüchen zurück, die die Überprüfung nicht überlebt haben, gefiltert. Erfordert, dass das [WebSearch-Tool](/docs/de/tools-reference#websearch-tool-behavior) verfügbar ist |

[Workflows, die Sie selbst speichern](#save-the-workflow-for-reuse), werden auf die gleiche Weise zu Befehlen und erscheinen in der `/`-Autovervollständigung neben den gebündelten.

<h3 id="watch-the-run">
  Überwachen Sie die Ausführung
</h3>

Workflows werden im Hintergrund ausgeführt, sodass die Sitzung reaktionsschnell bleibt, während Agenten arbeiten. Führen Sie `/workflows` jederzeit aus, um laufende und abgeschlossene Workflows aufzulisten, und wählen Sie dann einen aus, um die Fortschrittsansicht zu öffnen.

```text theme={null}
/workflows
```

Die Fortschrittsansicht zeigt jede Phase mit ihren Agentenzahlen, Gesamttoken und verstrichener Zeit. Die Fußzeile listet den Schlüssel für jede Aktion auf:

| Schlüssel        | Aktion                                                                                                                                                          |
| :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`        | Wählen Sie eine Phase oder einen Agenten aus                                                                                                                    |
| `Enter` oder `→` | Führen Sie einen Drilldown in die ausgewählte Phase durch, dann in einen Agenten, um seine Eingabeaufforderung, aktuelle Toolaufrufe und Ergebnis zu lesen      |
| `Esc` oder `←`   | Gehen Sie eine Ebene zurück. In v2.1.203 bis v2.1.205 hat `←` nicht aus einer Phase oder einem Agenten zurückgetreten; verwenden Sie `Esc` auf diesen Versionen |
| `j` / `k`        | Scrollen Sie innerhalb der Agent-Details, wenn sie überläuft                                                                                                    |
| `f`              | Filtern Sie die Agentenliste in der ausgewählten Phase nach Status. Drücken Sie erneut, um zu wechseln                                                          |
| `p`              | Unterbrechen oder fortsetzen Sie die Ausführung                                                                                                                 |
| `x`              | Beenden Sie den ausgewählten Agenten, oder beenden Sie den gesamten Workflow, wenn der Fokus auf der Ausführung liegt                                           |
| `r`              | Starten Sie den ausgewählten laufenden Agenten neu                                                                                                              |
| `s`              | [Speichern](#save-the-workflow-for-reuse) Sie das Skript der Ausführung als Befehl                                                                              |

<h2 id="have-claude-write-a-workflow">
  Lassen Sie Claude einen Workflow schreiben
</h2>

Sie können Claude auf zwei Arten einen Workflow für Ihre Aufgabe schreiben lassen:

* [Fordern Sie einen Workflow in Ihrer Eingabeaufforderung an](#ask-for-a-workflow-in-your-prompt), entweder in Ihren eigenen Worten oder durch Einbeziehung des Schlüsselworts `ultracode`, und Claude schreibt einen für die Aufgabe.
* [Lassen Sie Claude mit Ultracode entscheiden](#let-claude-decide-with-ultracode): Setzen Sie `/effort ultracode` und Claude plant einen Workflow für jede wesentliche Aufgabe in der Sitzung.

Sie können auch einen Workflow-Befehl ausführen, der bereits vorhanden ist: ein [gebündelter Workflow](#bundled-workflows) wie `/deep-research` oder einer, den Sie [gespeichert haben](#save-the-workflow-for-reuse).

<h3 id="ask-for-a-workflow-in-your-prompt">
  Fordern Sie einen Workflow in Ihrer Eingabeaufforderung an
</h3>

Um eine einzelne Aufgabe als Workflow auszuführen, ohne die Anstrengungsebene der Sitzung zu ändern, fügen Sie das Schlüsselwort `ultracode` in Ihrer Eingabeaufforderung ein. Das Fragen in Ihren eigenen Worten, zum Beispiel „einen Workflow verwenden" oder „einen Workflow ausführen", funktioniert auch: Claude behandelt eine direkte Anfrage als die gleiche Opt-in. Vor v2.1.160 war das wörtliche Trigger-Schlüsselwort `workflow`; Anfragen in natürlicher Sprache funktionieren in beiden Versionen.

```text theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code hebt das Schlüsselwort in Ihrer Eingabe hervor und Claude schreibt stattdessen ein Workflow-Skript für die Aufgabe, anstatt es Zug um Zug durchzuarbeiten. Wenn Sie nicht beabsichtigt haben, einen Workflow zu starten, drücken Sie `Option+W` auf macOS oder `Alt+W` auf Windows und Linux, um die Hervorhebung für diese Eingabeaufforderung zu verwerfen, oder drücken Sie Rücktaste, während sich der Cursor direkt nach dem hervorgehobenen Schlüsselwort befindet. Um zu verhindern, dass das Schlüsselwort überhaupt ausgelöst wird, deaktivieren Sie den Ultracode-Schlüsselwort-Trigger in `/config`.

Wenn die Ausführung das tut, was Sie wollten, können Sie [sie danach als Befehl speichern](#save-the-workflow-for-reuse).

Wenn Sie bereits einen Orchestrator auf andere Weise erstellt haben, z. B. einen Ordner mit Subagenten-Eingabeaufforderungen oder eine Fähigkeit, die Arbeit verteilt, können Sie Claude darauf hinweisen und einen Workflow anfordern, der dasselbe tut.

<h3 id="let-claude-decide-with-ultracode">
  Lassen Sie Claude mit Ultracode entscheiden
</h3>

Ultracode ist eine Claude Code-Einstellung, die `xhigh` [Anstrengungsebene](/docs/de/model-config#adjust-effort-level) mit automatischer Workflow-Orchestrierung kombiniert. Wenn es aktiviert ist, plant Claude einen Workflow für jede wesentliche Aufgabe, anstatt auf Sie zu warten.

```text theme={null}
/effort ultracode
```

Um eine Sitzung mit bereits aktiviertem Ultracode zu starten, starten Sie mit `claude --effort ultracode`. Erfordert Claude Code v2.1.203 oder später.

Mit Ultracode aktiviert entscheidet Claude, wann eine Aufgabe einen Workflow rechtfertigt. Eine einzelne Anfrage kann sich in mehrere Workflows hintereinander verwandeln: einen zum Verstehen des Codes, einen zum Vornehmen der Änderung und einen zum Überprüfen. Dies gilt für jede Aufgabe in der Sitzung, sodass jede Anfrage mehr Token verwendet und länger dauert als bei niedrigeren Anstrengungsebenen.

Ultracode dauert für die aktuelle Sitzung und wird zurückgesetzt, wenn Sie eine neue starten. Gehen Sie mit `/effort high` zurück, wenn Sie zur Routinearbeit zurückkehren. Es ist auf Modellen verfügbar, die `xhigh` [Anstrengung](/docs/de/model-config#adjust-effort-level) unterstützen; auf anderen Modellen bietet das `/effort`-Menü es nicht an.

<h3 id="approve-the-plan-before-it-runs">
  Genehmigen Sie den Plan, bevor er ausgeführt wird
</h3>

In der CLI zeigt die Eingabeaufforderung pro Ausführung die geplanten Phasen und diese Optionen:

* **Ja, führen Sie es aus**: Starten Sie die Ausführung
* **Ja, und fragen Sie nicht mehr nach `<name>` in `<path>`**: Starten Sie, und überspringen Sie diese Eingabeaufforderung für diesen Workflow in diesem Projekt von nun an
* **Rohes Skript anzeigen**: Lesen Sie das Skript, bevor Sie entscheiden
* **Nein**: Abbrechen

`Ctrl+G` öffnet das Skript in Ihrem Editor. `Tab` ermöglicht es Ihnen, die Eingabeaufforderung vor dem Start der Ausführung anzupassen.

Ob Sie diese Eingabeaufforderung sehen, hängt von Ihrem [Berechtigungsmodus](/docs/de/permission-modes) ab:

| Berechtigungsmodus                             | Wann Sie aufgefordert werden                                                                                                                                                                                         |
| :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Standard, Bearbeitungen akzeptieren            | Jede Ausführung, es sei denn, Sie haben **Ja, und fragen Sie nicht mehr** für diesen Workflow in diesem Projekt ausgewählt                                                                                           |
| Automatisch                                    | Nur beim ersten Start. Jedes **Ja** zeichnet die Zustimmung in Ihren Benutzereinstellungen auf, und spätere Starts werden ohne Eingabeaufforderung gestartet. Vollständig übersprungen, wenn Ultracode aktiviert ist |
| Berechtigungen umgehen, `claude -p`, Agent SDK | Nie. Die Ausführung startet sofort                                                                                                                                                                                   |

In der Desktop-App zeigt eine Genehmigungskarte den Workflow-Namen, die Phasenliste und eine Token-Nutzungswarnung mit den Aktionen **Einmal**, **Immer** und **Ablehnen**. Die Fortschrittsansicht wird im Seitenpanel „Hintergrundaufgaben" angezeigt.

Ihr Berechtigungsmodus steuert nur die oben genannte Startaufforderung. Die Subagenten, die der Workflow spawnt, werden immer im `acceptEdits`-Modus ausgeführt und erben Ihre [Tool-Zulassungsliste](/docs/de/settings#permission-settings), unabhängig vom Modus Ihrer Sitzung. Dateibearbeitungen werden automatisch genehmigt.

Shell-Befehle, Web-Abrufe und MCP-Tools, die nicht in Ihrer Zulassungsliste enthalten sind, können Sie während der Ausführung immer noch auffordern. Um dies bei einer langen Ausführung zu vermeiden, fügen Sie die Befehle, die die Agenten benötigen, vor dem Start zu Ihrer Zulassungsliste hinzu.

In `claude -p` und dem Agent SDK gibt es niemanden zum Auffordern, daher folgen Toolaufrufe Ihren konfigurierten Berechtigungsregeln ohne interaktive Bestätigung.

<h3 id="save-the-workflow-for-reuse">
  Speichern Sie den Workflow zur Wiederverwendung
</h3>

Wenn Claude einen Workflow für eine Aufgabe schreibt, die Sie wiederholen werden, können Sie das Skript dieser Ausführung als Befehl speichern. Ein Prozess wie eine Überprüfung, die Sie auf jedem Branch ausführen, führt dann jedes Mal die gleiche Orchestrierung aus.

Führen Sie `/workflows` aus, wählen Sie die Ausführung aus, die Sie behalten möchten, und drücken Sie `s`. Im Speicherdialog wechselt Tab zwischen den beiden Speicherorten:

* `.claude/workflows/` in Ihrem Projekt: Geteilt mit jedem, der das Repo klont
* `~/.claude/workflows/` in Ihrem Home-Verzeichnis: Verfügbar in jedem Projekt, nur für Sie sichtbar. Wenn Sie [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) setzen, ist dieser Speicherort das `workflows/`-Verzeichnis unter diesem Pfad.

Der Speicherdialog zeigt den aufgelösten Pfad für den persönlichen Speicherort an. Vor v2.1.208 zeigte er `~/.claude/workflows/` auch dann an, wenn `CLAUDE_CONFIG_DIR` gesetzt war; die Datei wurde trotzdem unter dem konfigurierten Verzeichnis gespeichert.

Drücken Sie Enter zum Speichern. Der Workflow wird in zukünftigen Sitzungen von beiden Orten aus als `/<name>` ausgeführt.

In einem Monorepo mit mehreren `.claude/`-Verzeichnissen können Sie Workflows neben dem Paket speichern, auf das sie sich beziehen. Ab v2.1.178 schreibt das Speichern am Projektort in das nächste `.claude/workflows/`-Verzeichnis, das bereits zwischen Ihrem Arbeitsverzeichnis und dem Repository-Root vorhanden ist, oder zum Repository-Root, wenn noch keines vorhanden ist. Projekt-Workflows werden auch aus jedem `.claude/workflows/` entlang dieses Pfads geladen, und wenn mehr als einer denselben Namen definiert, führt Claude Code denjenigen aus, der dem Arbeitsverzeichnis am nächsten ist.

Wenn ein Projekt-Workflow und ein persönlicher Workflow denselben Namen teilen, wird der Projekt-Workflow ausgeführt.

<h3 id="pass-input-to-a-saved-workflow">
  Eingabe an einen gespeicherten Workflow übergeben
</h3>

Ein gespeicherter Workflow kann Eingaben über den Parameter `args` akzeptieren. Das Skript liest ihn als globale Variable namens `args`. Verwenden Sie dies, um eine Forschungsfrage, eine Liste von Zielpfaden oder ein Konfigurationsobjekt zur Laufzeit bereitzustellen, anstatt das Skript für jede Ausführung zu bearbeiten.

Die folgende Eingabeaufforderung führt einen gespeicherten Workflow mit einer Liste von Issue-Nummern aus:

```text theme={null}
> Run /triage-issues on issues 1024, 1025, and 1030
```

Claude übergibt die Liste als strukturierte Daten, sodass das Skript Array- und Objektmethoden auf `args` direkt aufrufen kann, ohne sie zuerst zu analysieren. Wenn `args` weggelassen wird, ist die globale Variable `undefined` innerhalb des Skripts.

<h2 id="example-workflow-prompts">
  Beispiel-Workflow-Eingabeaufforderungen
</h2>

Ein Workflow passt am besten, wenn die Aufgabe größer ist, als ein Agent in den Kontext passen kann, oder wenn der gleiche Schritt über viele Elemente hinweg ausgeführt werden muss. Die folgenden Eingabeaufforderungen zeigen häufige Formen. Jede fordert Claude auf, einen Workflow für diese Aufgabe zu schreiben und auszuführen; Sie schreiben das Skript nicht selbst.

<h3 id="audit-many-files-for-the-same-issue">
  Viele Dateien für das gleiche Problem überprüfen
</h3>

Verteilen Sie einen Agenten pro Datei, dann sammeln und überprüfen Sie die Ergebnisse.

```text theme={null}
> use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Weiter beheben, bis eine Überprüfung besteht
</h3>

Führen Sie eine Überprüfung aus, beheben Sie, was fehlgeschlagen ist, und wiederholen Sie, bis es besteht oder keine Fortschritte mehr macht.

```text theme={null}
> use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Viele Dateien parallel migrieren
</h3>

Entdecken Sie die zu migrierenden Dateien, transformieren Sie jede in einer isolierten Kopie, damit Bearbeitungen nicht in Konflikt geraten, und überprüfen Sie jedes Ergebnis.

```text theme={null}
> use a workflow to migrate every component under src/components/ from styled-components to Tailwind, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Überprüfen Sie jede geänderte Datei und schreiben Sie eine Zusammenfassung
</h3>

Führen Sie einen Reviewer pro Datei aus, dann übergeben Sie alle Ergebnisse an einen Agenten, der sie ordnet und dedupliziert.

```text theme={null}
> use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Recherchieren Sie ein Thema über viele Quellen
</h3>

Verteilen Sie Leser über Changelogs, Issues und Dokumentation, dann synthetisieren Sie. Der gebündelte `/deep-research`-Workflow tut dies; Sie können auch eine engere Version beschreiben.

```text theme={null}
> use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Finden Sie Probleme, bis die Liste nicht mehr wächst
</h3>

Suchen Sie in Runden weiter und stoppen Sie, wenn neue Runden nichts Neues finden.

```text theme={null}
> use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

<h3 id="what-the-saved-script-looks-like">
  Wie das gespeicherte Skript aussieht
</h3>

Wenn Sie [einen Workflow speichern](#save-the-workflow-for-reuse), enthält die Datei in `.claude/workflows/` einen `meta`-Block gefolgt von einem Skript-Body, der Subagenten orchestriert. Sie müssen ihn normalerweise nicht bearbeiten, aber hier ist die Form eines kleinen, damit Sie erkennen können, was Claude generiert hat:

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

Der Body ist einfaches JavaScript mit Top-Level-`await`. `agent()` spawnt einen Subagenten und `pipeline()` führt einen pro Element in einer Liste aus. Wenn Sie ein Skript von Hand bearbeiten möchten, bitten Sie Claude, Sie durch die Änderung zu führen, oder siehe den Workflow-Tool-Eintrag in der [Agent SDK-Referenz](/docs/de/agent-sdk/typescript) für den vollständigen Satz von Optionen.

<h2 id="how-a-workflow-runs">
  Wie ein Workflow ausgeführt wird
</h2>

Die Workflow-Laufzeit führt das Skript in einer isolierten Umgebung aus, getrennt von Ihrem Gespräch. Zwischenergebnisse bleiben in Skriptvariablen, anstatt in Claudes Kontext zu landen.

Bei jeder Ausführung wird das Skript in eine Datei unter dem Verzeichnis Ihrer Sitzung in `~/.claude/projects/` geschrieben. Claude erhält den Pfad, wenn die Ausführung startet, sodass Sie danach fragen können. Sie können diese Datei öffnen, um die Orchestrierung zu lesen, die Claude geschrieben hat, sie mit dem Skript einer vorherigen Ausführung vergleichen oder sie bearbeiten und Claude bitten, von der bearbeiteten Version neu zu starten.

Die Laufzeit verfolgt das Ergebnis jedes Agenten, während die Ausführung fortschreitet, was macht, dass eine Ausführung [wiederaufnehmbar](#resume-after-a-pause) innerhalb derselben Sitzung ist.

<h3 id="behavior-and-limits">
  Verhalten und Grenzen
</h3>

Die Laufzeit wendet die folgenden Einschränkungen an:

| Einschränkung                                                                    | Warum                                                                                                                                                               |
| :------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Keine Benutzereingabe während der Ausführung                                     | Nur Agent-Berechtigungsaufforderungen können eine Ausführung unterbrechen. Für die Genehmigung zwischen Phasen führen Sie jede Phase als ihren eigenen Workflow aus |
| Kein direkter Dateisystem- oder Shell-Zugriff vom Workflow selbst                | Agenten lesen, schreiben und führen Befehle aus. Das Skript koordiniert die Agenten                                                                                 |
| Bis zu 16 gleichzeitige Agenten, weniger auf Maschinen mit begrenzten CPU-Kernen | Begrenzt die lokale Ressourcennutzung                                                                                                                               |
| 1.000 Agenten insgesamt pro Ausführung                                           | Verhindert Endlosschleifen                                                                                                                                          |

<h2 id="manage-runs">
  Verwalten Sie Ausführungen
</h2>

Sobald eine Ausführung startet, verwalten Sie sie über die `/workflows`-Ansicht oder durch Erweitern der Fortschrittszeile im Aufgabenpanel unter dem Eingabefeld.

<h3 id="resume-after-a-pause">
  Fortsetzen nach einer Pause
</h3>

Wenn Sie eine Ausführung beenden, können Sie sie fortsetzen: Agenten, die bereits abgeschlossen sind, geben ihre zwischengespeicherten Ergebnisse zurück, und der Rest wird live ausgeführt. Ein Agent, der noch ausgeführt wurde, als Sie gestoppt haben, wird nicht gespeichert und startet bei der Wiederaufnahme neu, sodass ein Workflow, der Arbeit über viele kleine Agenten verteilt, mehr Fortschritt bewahrt als ein langer Agent. Setzen Sie eine unterbrochene Ausführung von `/workflows` fort, indem Sie sie auswählen und `p` drücken, oder bitten Sie Claude, den Workflow mit dem gleichen Skript erneut zu starten.

Die Wiederaufnahme funktioniert innerhalb derselben Claude Code-Sitzung. Wenn Sie Claude Code beenden, während ein Workflow ausgeführt wird, startet die nächste Sitzung den Workflow von vorne.

<h3 id="cost">
  Kosten
</h3>

Ein Workflow spawnt viele Agenten, sodass eine einzelne Ausführung bedeutend mehr Token verwenden kann als die Bearbeitung der gleichen Aufgabe in einem Gespräch. Ausführungen zählen zur Nutzung und zu Ratenlimits Ihres Plans wie jede andere Sitzung.

Um die Ausgaben vor der Verpflichtung zu einer großen Aufgabe zu schätzen, führen Sie den Workflow zunächst auf einem kleinen Ausschnitt aus: ein Verzeichnis statt des gesamten Repositorys oder eine enge Frage statt einer breiten. Die `/workflows`-Ansicht zeigt die Token-Nutzung jedes Agenten während der Ausführung an, und Sie können die Ausführung dort jederzeit beenden, ohne abgeschlossene Arbeiten zu verlieren. Die Laufzeit-[Agent-Limits](#behavior-and-limits) begrenzen, wie viele Agenten eine einzelne Ausführung spawnen kann, was die Kosten eines unkontrollierten Skripts begrenzt. Um jeden Durchlauf standardmäßig kleiner zu halten, [legen Sie eine Größenrichtlinie fest](#set-a-size-guideline) in `/config`.

Claude Code kennzeichnet auch eine Ausführung, die ungewöhnlich groß wird. Wenn ein Workflow mehr als 25 Agenten plant oder seine projizierte Token-Gesamtzahl 1,5 Millionen überschreitet, zeigt die Fortschrittszeile im Aufgabenpanel unter dem Eingabefeld eine `Large workflow`-Warnung an. Die Warnung verweist Sie auf [`/workflows`](#watch-the-run), wo Sie die Ausführung beenden können. Erfordert Claude Code v2.1.203 oder später.

Die Warnung ist informativ: Sie pausiert oder begrenzt die Ausführung nicht. Zwei Einstellungen ändern sich, wenn Sie sie sehen:

* Wenn Sie [eine Größenrichtlinie festlegen](#set-a-size-guideline), ersetzt die Agentenzahl der Richtlinie den Schwellenwert von 25 Agenten.
* Sitzungen mit [ultracode](#let-claude-decide-with-ultracode) aktiviert zeigen die Warnung nicht an, da das Aktivieren von ultracode Sie bereits für große Ausführungen anmeldet.

Jeder Agent in einem Workflow verwendet das Modell Ihrer Sitzung, es sei denn, das Skript leitet eine Phase zu einem anderen weiter oder die Umgebungsvariable [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/de/model-config#environment-variables) ist gesetzt, was beides überschreibt. Um die Modellkosten zu kontrollieren:

* Überprüfen Sie `/model` vor einer großen Ausführung, wenn Sie normalerweise zu einem kleineren Modell für Routinearbeit wechseln
* Bitten Sie Claude, ein kleineres Modell für Phasen zu verwenden, die nicht das stärkste benötigen, wenn Sie die Aufgabe beschreiben

<h3 id="set-a-size-guideline">
  Legen Sie eine Größenrichtlinie fest
</h3>

Die Einstellung „Dynamic workflow size" in `/config` hält die Workflows, die Claude schreibt, standardmäßig in einem kleineren Maßstab. Claude Code sendet die Einstellung an Claude als Ratschlag, sodass ein Prompt, der einen anderen Maßstab fordert, diese Einstellung immer noch überschreibt. Erfordert Claude Code v2.1.202 oder später.

Jeder Wert legt die Agentenzahl fest, auf die Claude in den von ihm geschriebenen Skripten abzielt.

| Wert           | An Claude gesendete Anleitung                       |
| :------------- | :-------------------------------------------------- |
| `unrestricted` | Keine Richtlinie. Dies ist die Standardeinstellung. |
| `small`        | Ziel: weniger als 5 Agenten.                        |
| `medium`       | Ziel: weniger als 15 Agenten.                       |
| `large`        | Ziel: weniger als 50 Agenten.                       |

Änderungen werden beim nächsten Prompt wirksam. Die [Laufzeit-Agent-Limits](#behavior-and-limits) gelten weiterhin unabhängig von der Einstellung.

<h3 id="turn-workflows-off">
  Schalten Sie Workflows aus
</h3>

Workflows sind in der CLI, der Desktop-App, den IDE-Erweiterungen, [nicht-interaktivem Modus](/docs/de/headless) mit `claude -p` und dem [Agent SDK](/docs/de/agent-sdk/overview) verfügbar. Die gleichen Deaktivierungseinstellungen gelten auf jeder Oberfläche.

Um Workflows für sich selbst auszuschalten:

* Schalten Sie Dynamic workflows in `/config` aus. Bleibt über Sitzungen hinweg erhalten.
* Setzen Sie `"disableWorkflows": true` in `~/.claude/settings.json`. Bleibt über Sitzungen hinweg erhalten.
* Setzen Sie `CLAUDE_CODE_DISABLE_WORKFLOWS=1`. Wird beim Start gelesen, daher gilt es überall dort, wo Sie es setzen.

Um Workflows für Ihre gesamte Organisation auszuschalten, setzen Sie `"disableWorkflows": true` in [verwalteten Einstellungen](/docs/de/server-managed-settings) oder verwenden Sie den Umschalter auf der Seite [Claude Code-Administratoreinstellungen](https://claude.ai/admin-settings/claude-code).

Wenn Workflows deaktiviert sind, sind die gebündelten Workflow-Befehle nicht verfügbar, das Schlüsselwort `ultracode` löst keine Ausführung mehr aus, und `ultracode` wird aus dem `/effort`-Menü entfernt.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Führen Sie Agenten parallel aus](/docs/de/agents): Vergleichen Sie Subagenten, Agent-Ansicht, Agent-Teams und Workflows
* [Erstellen Sie benutzerdefinierte Subagenten](/docs/de/sub-agents): Der Worker-Primitive, den Workflows orchestrieren
* [Verwalten Sie Kosten](/docs/de/costs): Wie Multi-Agent-Ausführungen zu Ihren Nutzungslimits zählen
