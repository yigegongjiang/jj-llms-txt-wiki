> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrieren Sie Subagenten im großen Maßstab mit dynamischen Workflows

> Dynamische Workflows orchestrieren viele Subagenten aus einem Skript, das Claude schreibt und das Sie erneut ausführen können. Verwenden Sie sie für Codebase-Audits, große Migrationen und überprüfte Recherchen.

<Note>
  Dynamische Workflows sind auf allen bezahlten Plänen, mit Anthropic API-Zugriff und auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry verfügbar. Aktivieren Sie sie auf Pro über die Zeile „Dynamic workflows" in `/config`.
</Note>

Ein dynamischer Workflow ist ein JavaScript-Skript, das viele [Subagenten](/docs/de/sub-agents) auf einmal orchestriert. Claude schreibt das Skript für die Aufgabe, die Sie beschreiben, und eine Laufzeit führt es im Hintergrund aus, während Ihre Sitzung reaktionsschnell bleibt.

Greifen Sie zu einem Workflow, wenn eine Aufgabe mehr Agenten benötigt, als ein Gespräch koordinieren kann, oder wenn Sie die Orchestrierung als Skript codifizieren möchten, das Sie lesen und erneut ausführen können. Beispiele sind eine codebase-weite Fehlersuche, eine 500-Datei-Migration, eine Forschungsfrage, die Quellen gegeneinander überprüft, und ein schwieriger Plan, der aus mehreren unabhängigen Blickwinkeln entworfen werden sollte, bevor Sie sich auf einen einigen.

<h2 id="when-to-use-a-workflow">
  Wann sollte man einen Workflow verwenden
</h2>

[Subagenten](/docs/de/sub-agents), [Skills](/docs/de/skills), [Agent-Teams](/docs/de/agent-teams) und Workflows können alle eine mehrstufige Aufgabe ausführen. Der Unterschied liegt darin, wer den Plan hält:

|                                                   | Subagenten                         | Skills                          | Agent-Teams                                      | Workflows                                       |
| :------------------------------------------------ | :--------------------------------- | :------------------------------ | :----------------------------------------------- | :---------------------------------------------- |
| Was es ist                                        | Ein Worker, den Claude erzeugt     | Anweisungen, die Claude befolgt | Ein Lead-Agent, der Peer-Sitzungen beaufsichtigt | Ein Skript, das die Runtime ausführt            |
| Wer entscheidet, was als Nächstes ausgeführt wird | Claude, Zug um Zug                 | Claude, den Anweisungen folgend | Der Lead-Agent, Zug um Zug                       | Das Skript                                      |
| Wo Zwischenergebnisse gespeichert werden          | Claudes Kontextfenster             | Claudes Kontextfenster          | Eine gemeinsame Aufgabenliste                    | Skriptvariablen                                 |
| Was wiederholbar ist                              | Die Worker-Definition              | Die Anweisungen                 | Die Team-Definition                              | Die Orchestrierung selbst                       |
| Skalierung                                        | Einige delegierte Aufgaben pro Zug | Gleich wie Subagenten           | Eine Handvoll langfristiger Peers                | Dutzende bis Hunderte von Agenten pro Durchlauf |
| Unterbrechung                                     | Startet den Zug neu                | Startet den Zug neu             | Teammates laufen weiter                          | Wiederaufnehmbar in derselben Sitzung           |

Ein Workflow verlagert den Plan in Code. Bei Subagenten, Skills und Agent-Teams ist Claude der Orchestrator: Er entscheidet Zug um Zug, was als Nächstes erzeugt oder zugewiesen werden soll, und jedes Ergebnis geht in ein Kontextfenster. Ein Workflow-Skript hält die Schleife, die Verzweigung und die Zwischenergebnisse selbst, sodass Claudes Kontext nur die endgültige Antwort enthält.

Die Verlagerung des Plans in Code ermöglicht es einem Workflow auch, ein wiederholbares Qualitätsmuster anzuwenden, nicht nur mehr Agenten auszuführen: Er kann unabhängige Agenten gegenseitig die Ergebnisse des anderen gegnerisch überprüfen lassen, bevor sie gemeldet werden, oder einen Plan aus mehreren Blickwinkeln entwerfen und diese gegeneinander abwägen, sodass Sie ein vertrauenswürdigeres Ergebnis erhalten als bei einem einzelnen Durchgang.

<h2 id="run-a-bundled-workflow">
  Einen gebündelten Workflow ausführen
</h2>

Die schnellste Möglichkeit, einen Workflow in Aktion zu sehen, ist die Ausführung von `/deep-research`, dem [integrierten Workflow](#bundled-workflows), den Claude Code zum Untersuchen einer Frage über viele Quellen hinweg enthält. Sie sehen, wie Agenten im Hintergrund eine Reihe von Phasen durcharbeiten, während Ihre Sitzung frei bleibt, und erhalten am Ende einen Bericht statt eines Turn-by-Turn-Transkripts.

<Steps>
  <Step title="Workflow ausführen">
    Führen Sie `/deep-research` mit einer Frage aus, die Sie untersuchen möchten. Es verteilt Web-Suchen über mehrere Blickwinkel, ruft die gefundenen Quellen ab und überprüft sie gegenseitig, und erstellt einen zitierten Bericht.

    ```text wrap theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Workflows zulassen">
    Claude Code fragt, ob der Workflow zulässig sein soll. Wählen Sie **Ja**, um fortzufahren. Die genaue Eingabeaufforderung hängt von Ihrem Berechtigungsmodus ab. Siehe [Genehmigen Sie den Plan, bevor er ausgeführt wird](#approve-the-plan-before-it-runs) für die Optionen pro Modus.
  </Step>

  <Step title="Fortschritt beobachten">
    Die Ausführung startet im Hintergrund. Führen Sie `/workflows` aus, verwenden Sie die Pfeiltasten, um die Ausführung auszuwählen, und drücken Sie die Eingabetaste, um die Fortschrittsansicht zu öffnen:

    ```text wrap theme={null}
    /workflows
    ```

    Die Ansicht zeigt jede Phase mit ihrer Agentenzahl, Gesamttoken und verstrichener Zeit. Führen Sie einen Drilldown in jede Phase durch, um ihre Agenten und deren Ergebnisse anzuzeigen. Siehe [Beobachten Sie die Ausführung](#watch-the-run) für den vollständigen Satz von Steuerelementen.

    Sie können auch vom Aufgabenpanel unter dem Eingabefeld aus beobachten: Während die Ausführung läuft, wird dort eine einzeilige Fortschrittsübersicht angezeigt. Drücken Sie die Abwärts-Taste, um den Fokus darauf zu legen, und dann die Eingabetaste, um es zu erweitern.
  </Step>

  <Step title="Bericht lesen">
    Wenn die Ausführung abgeschlossen ist, landet der Bericht in Ihrer Sitzung. Er zitiert die Quellen, aus denen jeder Anspruch stammt, wobei Ansprüche, die die Überprüfung nicht überstanden haben, bereits herausgefiltert sind.

    Wenn die Verifier-Agenten einen Anspruch nicht überprüfen können, z. B. nach einer Ratenbegrenzung oder einem API-Fehler, listet der Bericht diesen Anspruch als unverified statt als widerlegt auf.
  </Step>
</Steps>

Um einen Workflow für Ihre eigene Aufgabe auszuführen, [lassen Sie Claude einen schreiben](#have-claude-write-a-workflow), und sobald eine Ausführung das tut, was Sie wollten, können Sie ihn [speichern](#save-the-workflow-for-reuse) als Befehl Ihres eigenen.

<h3 id="bundled-workflows">
  Gebündelte Workflows
</h3>

Claude Code enthält `/deep-research` als integrierten Workflow:

| Befehl                      | Was er tut                                                                                                                                                                                                                                                                                                                                                                      |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/deep-research <question>` | Verteilt Web-Suchen zu einer Frage über mehrere Blickwinkel, ruft die gefundenen Quellen ab und überprüft sie gegenseitig, stimmt über jeden Anspruch ab und gibt einen zitierten Bericht mit herausgefilterten Ansprüchen zurück, die die Überprüfung nicht überstanden haben. Erfordert, dass das [WebSearch-Tool](/docs/de/tools-reference#websearch-tool-behavior) verfügbar ist |

`/deep-research` wird nur ausgeführt, wenn Sie es aufrufen.

[Workflows, die Sie selbst speichern](#save-the-workflow-for-reuse), werden auf die gleiche Weise zu Befehlen und erscheinen in der `/`-Autovervollständigung neben den gebündelten.

<h3 id="watch-the-run">
  Beobachten Sie die Ausführung
</h3>

Workflows werden im Hintergrund ausgeführt, sodass die Sitzung reaktionsschnell bleibt, während Agenten arbeiten. Führen Sie `/workflows` jederzeit aus, um laufende und abgeschlossene Workflows aufzulisten, und wählen Sie dann einen aus, um die Fortschrittsansicht zu öffnen.

Die Fortschrittsansicht zeigt jede Phase mit ihren Agentenzahlen, Gesamttoken und verstrichener Zeit. Die Fußzeile listet den Schlüssel für jede Aktion auf:

| Taste            | Aktion                                                                                                                                                          |
| :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`        | Wählen Sie eine Phase oder einen Agenten aus                                                                                                                    |
| `Enter` oder `→` | Führen Sie einen Drilldown in die ausgewählte Phase durch, dann in die Details eines Agenten. In den Details erweitert oder reduziert `Enter` diese             |
| `Esc` oder `←`   | Gehen Sie eine Ebene zurück. In v2.1.203 bis v2.1.205 ist `←` nicht aus einer Phase oder einem Agenten zurückgetreten; verwenden Sie `Esc` auf diesen Versionen |
| `j` / `k`        | Scrollen Sie innerhalb der Agenten-Details, wenn diese überläuft                                                                                                |
| `f`              | Filtern Sie die Agentenliste in der ausgewählten Phase nach Status. Drücken Sie erneut, um zu wechseln                                                          |
| `p`              | Unterbrechen oder fortsetzen Sie die Ausführung                                                                                                                 |
| `x`              | Beenden Sie den ausgewählten Agenten, oder beenden Sie den gesamten Workflow, wenn der Fokus auf der Ausführung liegt                                           |
| `r`              | Starten Sie den ausgewählten laufenden Agenten neu                                                                                                              |
| `s`              | [Speichern](#save-the-workflow-for-reuse) Sie das Skript der Ausführung als Befehl                                                                              |

Die Agenten-Details listen die Eingabeaufforderung des Agenten, seine letzten Toolaufrufe und sein Ergebnis auf. Jeder Aufruf zeigt seinen Status an, z. B. noch laufend oder fehlgeschlagen. Wenn der Agent eine eigene Aufgabenliste führt, zeigen die Details diese auch an, mit dem Status jeder Aufgabe.

Drücken Sie `Enter`, um die Details zu erweitern. Die Eingabeaufforderung und das Ergebnis werden dann vollständig angezeigt, und jeder aufgelistete Aufruf zeigt seine Eingabe und den Anfang seines Ergebnisses.

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

Um eine einzelne Aufgabe als Workflow auszuführen, ohne die Anstrengungsebene der Sitzung zu ändern, fügen Sie das Schlüsselwort `ultracode` in Ihrer Eingabeaufforderung ein. Das Fragen in Ihren eigenen Worten, zum Beispiel „einen Workflow verwenden" oder „einen Workflow ausführen", funktioniert auch: Claude behandelt eine direkte Anfrage als die gleiche Opt-in.

```text wrap theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code hebt das Schlüsselwort in Ihrer Eingabe hervor und Claude schreibt stattdessen ein Workflow-Skript für die Aufgabe, anstatt es Zug um Zug durchzuarbeiten. Das Schlüsselwort wählt nur, wie Claude die Arbeit strukturiert: Die Agenten-Tool-Aufrufe erhalten die gleichen Berechtigungsprüfungen und [Sandboxing](/docs/de/sandboxing) wie jeder andere Tool-Aufruf in der Sitzung.

Wenn die Ausführung das tut, was Sie wollten, können Sie [sie danach als Befehl speichern](#save-the-workflow-for-reuse). Wenn Sie bereits einen Orchestrator auf andere Weise erstellt haben, z. B. einen Ordner mit Subagenten-Eingabeaufforderungen oder eine Fähigkeit, die Arbeit verteilt, können Sie Claude darauf hinweisen und einen Workflow anfordern, der dasselbe tut.

<h4 id="dismiss-or-turn-off-the-keyword">
  Verwerfen oder deaktivieren Sie das Schlüsselwort
</h4>

Wenn Sie nicht beabsichtigt haben, einen Workflow zu starten, drücken Sie `Option+W` auf macOS oder `Alt+W` auf Windows und Linux, um die Hervorhebung für diese Eingabeaufforderung zu verwerfen, oder drücken Sie Rücktaste, während sich der Cursor direkt nach dem hervorgehobenen Schlüsselwort befindet. Um zu verhindern, dass das Schlüsselwort überhaupt ausgelöst wird, deaktivieren Sie den Ultracode-Schlüsselwort-Trigger in `/config`.

<h4 id="where-the-keyword-works">
  Wo das Schlüsselwort funktioniert
</h4>

Das Schlüsselwort ist ein Opt-in nur in einer Eingabeaufforderung, die Sie selbst eingeben: an der interaktiven Eingabeaufforderung, in einem IDE-Erweiterungspanel, in einem [Remote Control](/docs/de/remote-control)-Client oder in einer Agent SDK-Anwendung, die Ihre Tastatureingabe mit [`origin`](/docs/de/agent-sdk/typescript#sdkmessageorigin) als `{ kind: "human" }` kennzeichnet. Es startet keinen Workflow, wenn er auf andere Weise in die Sitzung gelangt:

* eine Eingabeaufforderung, die mit `-p` übergeben wird
* eine Eingabeaufforderung, die eine Agent SDK-Anwendung sendet, ohne sie als menschliche Eingabe zu kennzeichnen
* eine geplante Task-Eingabeaufforderung
* eine Webhook-Nutzlast oder ein Pull-Request-Kommentar, der in das Gespräch weitergeleitet wird

<Note>
  Vor v2.1.210 startete das Schlüsselwort einen Workflow auch von diesen Routen aus, einschließlich einer Webhook-Nutzlast oder eines Pull-Request-Kommentars, der in das Gespräch weitergeleitet wird.
</Note>

<h3 id="let-claude-decide-with-ultracode">
  Lassen Sie Claude mit Ultracode entscheiden
</h3>

Ultracode ist eine Claude Code-Einstellung, die `xhigh` [Anstrengungsebene](/docs/de/model-config#adjust-effort-level) mit automatischer Workflow-Orchestrierung kombiniert. Wenn es aktiviert ist, plant Claude einen Workflow für jede wesentliche Aufgabe, anstatt auf Sie zu warten.

```text wrap theme={null}
/effort ultracode
```

Um eine Sitzung mit bereits aktiviertem Ultracode zu starten, starten Sie mit `claude --effort ultracode`. Erfordert Claude Code v2.1.203 oder später.

Um es zu aktivieren, während Sie ein Modell auswählen, verschieben Sie den Anstrengungsregler des `/model`-Pickers mit den Pfeiltasten auf `ultracode`. [Anstrengungsebene anpassen](/docs/de/model-config#adjust-effort-level) listet die Routen auf, die Ultracode aktivieren.

Mit Ultracode aktiviert entscheidet Claude, wann eine Aufgabe einen Workflow rechtfertigt. Eine einzelne Anfrage kann sich in mehrere Workflows hintereinander verwandeln: einen zum Verstehen des Codes, einen zum Vornehmen der Änderung und einen zum Überprüfen. Dies gilt für jede Aufgabe in der Sitzung, sodass jede Anfrage mehr Token verwendet und länger dauert als bei niedrigeren Anstrengungsebenen.

`/effort ultracode` dauert für die aktuelle Sitzung; um jede Sitzung damit zu starten, setzen Sie die [`ultracode`](/docs/de/settings-reference#ultracode)-Einstellung. Gehen Sie mit `/effort high` zurück, wenn Sie zur Routinearbeit zurückkehren. Das `/effort`-Menü bietet es nur [wenn Ultracode verfügbar ist](/docs/de/model-config#when-ultracode-is-available).

<h3 id="approve-the-plan-before-it-runs">
  Genehmigen Sie den Plan, bevor er ausgeführt wird
</h3>

In der CLI zeigt die Eingabeaufforderung pro Ausführung die geplanten Phasen und diese Optionen:

* **Ja, führen Sie es aus**: Starten Sie die Ausführung
* **Ja, und fragen Sie nicht mehr nach `<name>` in `<path>`**: Starten Sie, und überspringen Sie diese Eingabeaufforderung für diesen Workflow in diesem Projekt von nun an. Claude Code bietet diese Option, wenn Sie einen gebündelten, gespeicherten oder Plugin-Workflow nach Name ausführen, nicht für ein Skript, das Claude für die aktuelle Aufgabe geschrieben hat.
* **Rohes Skript anzeigen**: Lesen Sie das Skript, bevor Sie entscheiden
* **Nein**: Abbrechen

`Ctrl+G` öffnet das Skript in Ihrem Editor. `Tab` ermöglicht es Ihnen, die Eingabeaufforderung vor dem Start der Ausführung anzupassen.

Ob Sie diese Eingabeaufforderung sehen, hängt von Ihrem [Berechtigungsmodus](/docs/de/permission-modes) ab:

| Berechtigungsmodus                 | Wann Sie aufgefordert werden                                                                                                                                                                                         |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Automatisch                        | Nur beim ersten Start. Jedes **Ja** zeichnet die Zustimmung in Ihren Benutzereinstellungen auf, und spätere Starts werden ohne Eingabeaufforderung gestartet. Vollständig übersprungen, wenn Ultracode aktiviert ist |
| Manuell, Bearbeitungen akzeptieren | Jede Ausführung, es sei denn, Sie haben **Ja, und fragen Sie nicht mehr** für diesen Workflow in diesem Projekt ausgewählt                                                                                           |
| Berechtigungen umgehen             | Claude Code fordert Sie nicht auf. Die Ausführung startet sofort                                                                                                                                                     |
| `claude -p`, Agent SDK             | Claude Code fordert Sie nicht auf                                                                                                                                                                                    |

In `claude -p` und dem Agent SDK zeigt Claude Code diese Eingabeaufforderung nie. Es führt den Workflow-Tool-Aufruf durch die gleiche [Berechtigungsevaluierung](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) wie der Rest der Sitzung aus, sodass Ablehnungsregeln, Anfragregeln und `dontAsk`-Modus auf den Start angewendet werden, wie sie auf jeden Tool-Aufruf angewendet werden. Um den Workflow-Start in diesen Ausführungen zu ermöglichen, verwenden Sie eines dieser:

* **Berechtigungsregel**: `Workflow` in Ihren Zulassungsregeln genehmigt jeden Workflow, und `Workflow(<name>)` genehmigt einen gespeicherten Workflow nach Name.
* **Automatischer Berechtigungsmodus**: Der [Klassifizierer](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) überprüft den Aufruf und kann ihn genehmigen.
* **Berechtigungen umgehen-Modus**: Claude Code genehmigt den Aufruf.
* **Ein `PreToolUse`-Hook**: Ein [Hook](/docs/de/hooks#pretooluse), der `allow` für den Aufruf zurückgibt, genehmigt ihn.
* **Ihr Host**: Ein [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) genehmigt ihn, oder mit dem Agent SDK ein [`canUseTool`](/docs/de/agent-sdk/permissions)-Callback oder ein [`PermissionRequest`-Hook](/docs/de/hooks#permissionrequest) genehmigt ihn.

In der Desktop-App zeigt eine Genehmigungskarte den Workflow-Namen, die Phasenliste und eine Token-Nutzungswarnung mit den Aktionen **Einmal**, **Immer** und **Ablehnen**. Die Fortschrittsansicht wird im Seitenpanel „Hintergrundaufgaben" angezeigt.

Die Subagenten, die der Workflow spawnt, verwenden Ihre [Berechtigungsregeln](/docs/de/settings-reference#permission-settings), und Claude Code wählt ihren Berechtigungsmodus nach den Regeln unter [welcher Berechtigungsmodus ein Subagent ausgeführt wird](/docs/de/sub-agents#permission-modes). Um Eingabeaufforderungen bei einer langen Ausführung zu vermeiden, fügen Sie die Tools, die die Agenten benötigen, vor dem Start zu Ihren Zulassungsregeln hinzu.

<h3 id="save-the-workflow-for-reuse">
  Speichern Sie den Workflow zur Wiederverwendung
</h3>

Wenn Claude einen Workflow für eine Aufgabe schreibt, die Sie wiederholen werden, können Sie das Skript dieser Ausführung als Befehl speichern. Ein Prozess wie eine Überprüfung, die Sie auf jedem Branch ausführen, führt dann jedes Mal die gleiche Orchestrierung aus.

Führen Sie `/workflows` aus, wählen Sie die Ausführung aus, die Sie behalten möchten, und drücken Sie `s`. Im Speicherdialog wechselt Tab zwischen den beiden Speicherorten:

* `.claude/workflows/` in Ihrem Projekt: Geteilt mit jedem, der das Repo klont
* `~/.claude/workflows/` in Ihrem Home-Verzeichnis: Verfügbar in jedem Projekt, nur für Sie sichtbar. Wenn Sie [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) setzen, ist dieser Speicherort das `workflows/`-Verzeichnis unter diesem Pfad.

Der Speicherdialog zeigt den aufgelösten Pfad für den persönlichen Speicherort an.

Drücken Sie Enter zum Speichern. Der Workflow wird in zukünftigen Sitzungen von beiden Orten aus als `/<name>` ausgeführt.

Claude Code überprüft den Speicherort auf Symlinks, bevor geschrieben wird, und zeigt stattdessen einen Fehler an. Was überprüft wird, hängt davon ab, wo Sie speichern:

* Projektort: Claude Code weigert sich, wenn `.claude`, `.claude/workflows` oder die Zieldatei ein Symlink ist.
* Persönlicher Ort: Claude Code weigert sich nur, wenn die Zieldatei selbst ein Symlink ist, sodass ein von einem Dotfiles-Tool verwaltetes `~/.claude`-Verzeichnis immer noch funktioniert.

Vor v2.1.216 folgte Claude Code dem Link, was die Datei außerhalb des gewählten Speicherorts platzieren konnte.

In einem Monorepo mit mehreren `.claude/`-Verzeichnissen können Sie Workflows neben dem Paket speichern, auf das sie sich beziehen. Das Speichern am Projektort schreibt in das nächste `.claude/workflows/`-Verzeichnis, das bereits zwischen Ihrem Arbeitsverzeichnis und dem Repository-Root vorhanden ist, oder zum Repository-Root, wenn noch keines vorhanden ist. Projekt-Workflows werden auch aus jedem `.claude/workflows/` entlang dieses Pfads geladen, und wenn mehr als einer denselben Namen definiert, führt Claude Code denjenigen aus, der dem Arbeitsverzeichnis am nächsten ist.

Wenn ein Projekt-Workflow und ein persönlicher Workflow denselben Namen teilen, wird der Projekt-Workflow ausgeführt.

<h3 id="distribute-a-workflow-in-a-plugin">
  Verteilen Sie einen Workflow in einem Plugin
</h3>

Um einen Workflow über Teams oder Repositories hinweg zu teilen, fügen Sie ihn in ein [Plugin](/docs/de/plugins/overview) ein. Platzieren Sie das Skript in einem `workflows/`-Verzeichnis im Plugin-Root, oder verweisen Sie mit dem [`workflows`-Manifestfeld](/docs/de/plugins/manifest-reference#fields) auf einen anderen Ort.

Plugin-Workflows werden nach dem Plugin-Namen namensgebunden. Ein Plugin namens `acme-tools`, das ein Skript mit `meta.name` `release-audit` enthält, wird als `/acme-tools:release-audit` ausgeführt.

<h3 id="pass-input-to-a-saved-workflow">
  Übergeben Sie Eingaben an einen gespeicherten Workflow
</h3>

Ein gespeicherter Workflow kann Eingaben über den Parameter `args` akzeptieren. Das Skript liest ihn als globale Variable namens `args`. Verwenden Sie dies, um eine Forschungsfrage, eine Liste von Zielpfaden oder ein Konfigurationsobjekt zur Laufzeit bereitzustellen, anstatt das Skript für jede Ausführung zu bearbeiten.

Die folgende Eingabeaufforderung führt einen gespeicherten Workflow mit einer Liste von Issue-Nummern aus:

```text wrap theme={null}
Run /triage-issues on issues 1024, 1025, and 1030
```

Claude übergibt die Liste als strukturierte Daten, sodass das Skript Array- und Objektmethoden auf `args` direkt aufrufen kann, ohne sie zuerst zu analysieren. Wenn `args` weggelassen wird, ist die globale Variable `undefined` innerhalb des Skripts.

<h2 id="example-workflow-prompts">
  Beispiel-Workflow-Eingabeaufforderungen
</h2>

Ein Workflow passt am besten, wenn die Aufgabe größer ist, als ein Agent in den Kontext passen kann, oder wenn der gleiche Schritt über viele Elemente hinweg ausgeführt werden muss. Die folgenden Eingabeaufforderungen zeigen häufige Formen. Jede fordert Claude auf, einen Workflow für diese Aufgabe zu schreiben und auszuführen; Sie schreiben das Skript nicht selbst.

<h3 id="audit-many-files-for-the-same-issue">
  Viele Dateien für das gleiche Problem überprüfen
</h3>

Fan out one agent per file, then collect and verify the findings.

```text wrap theme={null}
use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Weiter beheben, bis eine Überprüfung besteht
</h3>

Führen Sie eine Überprüfung aus, beheben Sie, was fehlgeschlagen ist, und wiederholen Sie, bis es besteht oder keine Fortschritte mehr macht.

```text wrap theme={null}
use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Viele Dateien parallel migrieren
</h3>

Entdecken Sie die zu migrierenden Dateien, transformieren Sie jede in einer isolierten Kopie, damit Bearbeitungen nicht in Konflikt geraten, und überprüfen Sie jedes Ergebnis.

```text wrap theme={null}
use a workflow to migrate every component under src/components/ from JavaScript to TypeScript, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Überprüfen Sie jede geänderte Datei und schreiben Sie eine Zusammenfassung
</h3>

Führen Sie einen Reviewer pro Datei aus, dann übergeben Sie alle Ergebnisse an einen Agenten, der sie ordnet und dedupliziert.

```text wrap theme={null}
use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Recherchieren Sie ein Thema über viele Quellen
</h3>

Verteilen Sie Leser über Changelogs, Issues und Dokumentation, dann synthetisieren Sie. Der gebündelte `/deep-research`-Workflow tut dies; Sie können auch eine engere Version beschreiben.

```text wrap theme={null}
use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Finden Sie Probleme, bis die Liste nicht mehr wächst
</h3>

Suchen Sie in Runden weiter und stoppen Sie, wenn neue Runden nichts Neues finden.

```text wrap theme={null}
use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
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

Der Body ist einfaches JavaScript mit Top-Level-`await`. `agent()` spawnt einen Subagenten, `pipeline()` führt einen pro Element in einer Liste aus, und `parallel()` führt eine Reihe von Agenten-Aufgaben gleichzeitig aus und wartet auf alle.

Ein `agent()`-Aufruf wird zu `null` aufgelöst, wenn Sie ihn während der Ausführung stoppen oder er auf einen nicht wiederherstellbaren API-Fehler trifft. `pipeline()` behält jeden `null` im Ergebnis-Array, weshalb das Beispiel mit `.filter(Boolean)` endet, um diese Einträge zu entfernen.

In [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) zählt die Eingabeaufforderung, die Ihr Skript an `agent()` übergibt, nicht als eine Anfrage von Ihnen, wenn der Klassifizierer die Aktionen dieses Subagenten überprüft, da Claude Code sie als Text markiert, den das Skript berechnet hat.

Wenn Sie einen `schema` bei einem `agent()`-Aufruf übergeben, gibt der Subagent stattdessen JSON zurück, das der Form entspricht. Claude Code überprüft das Schema, bevor der Subagent startet: Wenn es beweisen kann, dass das Schema sich selbst widerspricht, schlägt der Aufruf mit einem Fehler fehl, der den Widerspruch benennt, und der Subagent startet nie. Ein Widerspruch, den es beweisen kann, ist ein `required`-Schlüssel, den `additionalProperties: false` ausschließt.

Wenn die Ausgabe des Subagenten nach fünf Versuchen immer noch die Validierung nicht besteht, schlägt der Aufruf mit einem Fehler fehl, der den letzten Validierungsfehler enthält. Um die Versuchsanzahl zu ändern, setzen Sie [`MAX_STRUCTURED_OUTPUT_RETRIES`](/docs/de/env-vars).

<h3 id="edit-a-saved-script">
  Ein gespeichertes Skript bearbeiten
</h3>

Um einen [Workflow zu ändern, den Sie gespeichert haben](#save-the-workflow-for-reuse), bearbeiten Sie seine `.js`-Datei oder bitten Sie Claude, die Änderung vorzunehmen. Bevor Sie bearbeiten oder fragen, führen Sie die `/workflow-authoring` [gebündelte Fähigkeit](/docs/de/skills#bundled-skills) aus, um die Skript-Schreib-Referenz zu laden, von der Claude arbeitet. Die Fähigkeit erfordert Claude Code v2.1.248 oder später.

Um die bearbeitete Version in der aktuellen Sitzung auszuführen, führen Sie [`/reload-skills`](/docs/de/commands#all-commands) aus, um die Workflow-Verzeichnisse neu zu lesen, und führen Sie dann `/<name>` erneut aus.

Claude Code wendet diese Regeln auf jeden Teil der Datei an, wenn es das Skript lädt und ausführt:

* **`meta`-Block**: Behalten Sie `export const meta` als erste Anweisung bei, und halten Sie es ein einfaches Objekt-Literal mit einem `name` und einer `description`. Wenn es etwas anderes als Literalwerte enthält, wie eine Variable, einen Funktionsaufruf oder einen Spread, lässt Claude Code `/<name>` aus der `/`-Autovervollständigung fallen.
* **Body**: Neben `agent()`, `pipeline()` und `parallel()` können Sie `phase()` aufrufen, um die folgenden Agenten unter einem Titel in der Fortschrittsansicht zu gruppieren, `log()` aufrufen, um eine Nachricht über den Phasen anzuzeigen, und das globale [`args`](#pass-input-to-a-saved-workflow) lesen. Wenn der Body einen Syntaxfehler hat, meldet Claude Code ihn, wenn Sie den Workflow ausführen.
* **`phases`**: Wenn Sie sie in `meta` auflisten, geben Sie jedem Eintrag genau den Titel an, den Sie an `phase()` übergeben. Ein `phase()`-Titel ohne Eintrag erhält seine eigene Fortschrittsgruppe.
* **Zeitstempel und Zufälligkeit**: Claude Code lässt `Date.now()`, `Math.random()` und ein argumentloses `new Date()` innerhalb des Skripts werfen, damit ein [neu gestarteter Lauf](#resume-after-a-pause) die gleichen `agent()`-Aufrufe wiederholt. Übergeben Sie stattdessen einen Zeitstempel durch `args`.

Sie können auch [das Skript eines einzelnen Laufs](#how-a-workflow-runs) bearbeiten, anstatt die gespeicherte Kopie. [Nach einer Pause fortsetzen](#resume-after-a-pause) behandelt, welche Agenten erneut ausgeführt werden, wenn Sie ein bearbeitetes Skript neu starten. Für die Eingaben des Workflow-Tools siehe seinen Eintrag in der [Agent SDK-Referenz](/docs/de/agent-sdk/typescript#workflow).

<h2 id="how-a-workflow-runs">
  Wie ein Workflow ausgeführt wird
</h2>

Die Workflow-Laufzeit führt das Skript in einer isolierten Umgebung aus, getrennt von Ihrem Gespräch. Zwischenergebnisse bleiben in Skriptvariablen, anstatt in Claudes Kontext zu landen.

Bei jeder Ausführung wird das Skript in eine Datei unter dem Verzeichnis Ihrer Sitzung in `~/.claude/projects/` geschrieben. Claude erhält den Pfad, wenn die Ausführung startet, sodass Sie danach fragen können. Sie können diese Datei öffnen, um die Orchestrierung zu lesen, die Claude geschrieben hat, sie mit dem Skript einer vorherigen Ausführung vergleichen oder sie bearbeiten und Claude bitten, von der bearbeiteten Version neu zu starten.

Claude kann einen Workflow nur aus einer Skriptdatei starten, die die Sitzung bereits lesen darf. Um ein Skript auszuführen, das sich außerhalb Ihres Arbeitsverzeichnisses befindet, fügen Sie zunächst sein Verzeichnis mit [`/add-dir`](/docs/de/permissions#working-directories) oder einer [Read-Erlaubnisregel](/docs/de/permissions#read-and-edit) hinzu.

Die Laufzeit verfolgt das Ergebnis jedes Agenten, während die Ausführung fortschreitet, was macht, dass eine Ausführung [wiederaufnehmbar](#resume-after-a-pause) innerhalb derselben Sitzung ist.

<h3 id="prompt-caching-in-a-fan-out">
  Prompt Caching in einem Fan-out
</h3>

Agenten in derselben Ausführung können den [Prompt Cache](/docs/de/prompt-caching#subagents-and-the-cache) voneinander lesen. Zwei Agenten, die mit demselben Modell, Aufwandsniveau, Agententyp, Tools, Ausgabeschema und Arbeitsverzeichnis ausgeführt werden, erstellen dasselbe Tools-und-System-Prompt-Präfix, sodass ein Agent, der nach der Antwort eines übereinstimmenden Geschwisteragenten startet, den Cache dieses Geschwisteragenten bei seiner ersten Anfrage liest.

Die Anfragen eines Workflow-Agenten fallen außerhalb des [Cache-TTL-Buckets](/docs/de/prompt-caching#which-ttl-each-request-gets) des Hauptgesprächs, sodass sein Cache standardmäßig fünf Minuten lang gültig bleibt, auch bei einem Claude-Abonnement. Um ihn eine Stunde lang zu behalten, setzen Sie [`subagentPromptCacheTtl`](/docs/de/settings-reference#subagentpromptcachettl) auf `1h`. Die API berechnet 1-Stunden-Cache-Schreibvorgänge mit einem höheren Satz ab.

Wenn ein Fan-out mehrere übereinstimmende Agenten gleichzeitig startet, hält Claude Code alle außer dem ersten an, bis die Antwort des ersten Agenten beginnt, und gibt dann die gehaltenen Agenten zusammen frei, sodass ihre ersten Anfragen das gemeinsame Präfix lesen, anstatt dass jeder es ungecacht verarbeitet. Claude Code begrenzt die Wartezeit auf [`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`](/docs/de/env-vars) Millisekunden, standardmäßig `5000`. Setzen Sie es auf `0`, um die Wartezeit zu deaktivieren.

<h3 id="behavior-and-limits">
  Verhalten und Grenzen
</h3>

Die Laufzeit wendet die folgenden Einschränkungen an:

| Einschränkung                                                                                                                                                                                                                                                                                                                                    | Warum                                                                                                                                                                                                                                        |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Keine Benutzereingabe während der Ausführung                                                                                                                                                                                                                                                                                                     | Eine Ausführung wird nur bei Agent-Berechtigungsaufforderungen und einer [Nutzungslimit-Wartezeit](#when-a-run-hits-your-usage-limit) unterbrochen. Für die Genehmigung zwischen Phasen führen Sie jede Phase als ihren eigenen Workflow aus |
| Kein direkter Dateisystem- oder Shell-Zugriff vom Workflow selbst                                                                                                                                                                                                                                                                                | Agenten lesen, schreiben und führen Befehle aus. Das Skript koordiniert die Agenten                                                                                                                                                          |
| Kein Modul-Laden: Ein Skript, das `import()` enthält, schlägt vor dem Start der Ausführung fehl                                                                                                                                                                                                                                                  | Der Skript-Body ist einfaches JavaScript. Legen Sie Arbeiten, die eine Bibliothek benötigen, in die Aufgabe eines Agenten                                                                                                                    |
| Bis zu 16 gleichzeitige Agenten standardmäßig, weniger wenn Claude Code weniger CPUs zur Verfügung hat, auch innerhalb eines CPU-limitierten Containers. Um die Grenze zu ändern, setzen Sie [`CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS`](/docs/de/env-vars#variables) auf einen Wert von 1 bis 256, was Claude Code v2.1.269 oder später erfordert | Begrenzt die lokale Ressourcennutzung                                                                                                                                                                                                        |
| In einem Fan-out starten Agenten, die das Prompt-Cache-Präfix des ersten Agenten teilen, bis zu 5 Sekunden später als dieser standardmäßig                                                                                                                                                                                                       | Alle außer dem ersten lesen das [Präfix, das der erste Agent gecacht hat](#prompt-caching-in-a-fan-out), anstatt dass jeder es ungecacht verarbeitet                                                                                         |
| Bis zu 4.096 Elemente in einem einzelnen `parallel()`- oder `pipeline()`-Aufruf: Die Laufzeit lehnt eine längere Liste mit einem Fehler ab                                                                                                                                                                                                       | Eine stille Obergrenze würde einen Teil der Arbeitslast ablegen, ohne das Skript zu informieren                                                                                                                                              |
| 1.000 Agenten insgesamt pro Ausführung                                                                                                                                                                                                                                                                                                           | Verhindert Endlosschleifen                                                                                                                                                                                                                   |

<h2 id="manage-runs">
  Verwalten Sie Ausführungen
</h2>

Sobald eine Ausführung startet, verwalten Sie sie über die `/workflows`-Ansicht oder durch Erweitern der Fortschrittszeile im Aufgabenpanel unter dem Eingabefeld.

Wenn Sie eine Ausführung stoppen, bleibt sie im Aufgabenpanel, während alle Prozesse ihrer Agenten noch laufen. Wenn Sie sie erneut stoppen, signalisiert Claude Code diese Prozesse erneut.

<h3 id="resume-after-a-pause">
  Fortsetzen nach einer Pause
</h3>

Setzen Sie eine unterbrochene Ausführung von `/workflows` fort, indem Sie sie auswählen und `p` drücken. Für eine Ausführung, die Sie gestoppt haben, bitten Sie Claude, den Workflow mit dem gleichen Skript erneut zu starten. Wenn Agenten aus der gestoppten Ausführung noch nicht beendet wurden, weigert sich Claude Code, den Neustart durchzuführen, bis sie beendet sind, sodass keine zweite Kopie dieser Agenten neben ihnen laufen kann.

Claude Code spielt die Ausführung in der Reihenfolge ab, in der die Agenten gestartet wurden, und jeder Agent gibt entweder sein gespeichertes Ergebnis zurück oder wird erneut ausgeführt:

* **Abgeschlossen**: gibt sein gespeichertes Ergebnis zurück. Der erste Agent, dessen Eingabeaufforderung sich vom vorherigen Durchlauf unterscheidet, weil Sie das Skript bearbeitet haben oder ein früherer Agent etwas anderes zurückgegeben hat, wird erneut ausgeführt, und ebenso jeder Agent danach, auch solche, die abgeschlossen wurden.
* **Noch ausgeführt, als Sie gestoppt haben**: startet neu. Das Stoppen der gesamten Ausführung zählt keinen Agent als fehlgeschlagen.
* **Fehlgeschlagen**: wird erneut ausgeführt, und ebenso jeder Agent, der danach gestartet wurde, auch solche, die abgeschlossen wurden. Das Stoppen eines einzelnen Agenten durch Auswahl in [`/workflows`](#watch-the-run) und Drücken von `x` zählt als Fehler.

Der letzte Fall bedeutet, dass ein Fehler in der Mitte eines Fan-out-Durchlaufs Arbeit erneut ausführt, die bereits abgeschlossen war. Wenn ein Skript A, B, C und D in dieser Reihenfolge startet und B fehlschlägt, gibt das erneute Starten A aus dem Cache zurück und führt B, C und D erneut aus.

Sie können eine Ausführung innerhalb derselben Claude Code-Sitzung fortsetzen. Was mit einem laufenden Workflow geschieht, wenn Sie die Sitzung verlassen, hängt davon ab, wie Sie sie verlassen:

* Wenn Sie [die Sitzung in den Hintergrund verschieben](/docs/de/agent-view#what-carries-over-when-you-background), spielt Claude Code die Ausführung auf die gleiche Weise in der Hintergrund-Sitzung ab und setzt sie fort.
* Wenn Sie Claude Code beenden, während ein Workflow ausgeführt wird, und [die Agent-Ansicht ist aktiviert](/docs/de/agent-view#from-inside-a-session), bietet der Beendigungsdialog `In den Hintergrund verschieben und beenden` an, was die Ausführung auf die gleiche Weise überträgt. Wenn Sie stattdessen `Beenden und Aufgaben stoppen` wählen oder die Option nicht angeboten wird, stoppt die Ausführung mit der Sitzung. Claude Code behält die gespeicherten Ergebnisse der Ausführung im Verzeichnis dieser Sitzung in `~/.claude/projects/` bei, sodass eine Sitzung, die Sie mit `claude --resume` fortsetzen, diese abspielen kann, wenn Sie Claude bitten, den Workflow erneut zu starten. In einer neu gestarteten Sitzung hat Claude keine frühere Ausführung zum Abspielen und startet den Workflow als neue Ausführung.

In einer [Cloud-Sitzung](/docs/de/claude-code-on-the-web) speichert Claude Code auch die Ergebnisse der Ausführung zusammen mit der Gesprächshistorie der Sitzung, die erhalten bleibt, wenn die VM der Sitzung zurückgefordert wird. Wenn Sie [eine solche Sitzung erneut öffnen](/docs/de/claude-code-on-the-web#environment-expired) und Claude bitten, den Workflow erneut zu starten, geben abgeschlossene Agenten immer noch ihre gespeicherten Ergebnisse zurück.

In lokalen und Cloud-Sitzungen gleichermaßen schlägt der Neustart fehl, wenn Claude eine frühere Ausführung erneut startet und Claude Code die gespeicherten Ergebnisse dieser Ausführung überhaupt nicht finden kann, mit einem `nothing to resume`-Fehler, anstatt die Ausführung von selbst neu zu starten. Bitten Sie Claude, den Workflow als neue Ausführung neu zu starten.

<h3 id="when-a-run-hits-your-usage-limit">
  Wenn eine Ausführung Ihr Nutzungslimit erreicht
</h3>

Wenn ein Agent Ihr claude.ai [Nutzungslimit](/docs/de/interactive-mode#wait-for-a-usage-limit-to-reset) erreicht, wird die Ausführung unterbrochen, anstatt diesen Agent fehlschlagen zu lassen: Die Agenten, die das Limit erreicht haben, warten auf den Reset, und es starten keine neuen Agenten. Kurz nach dem Limit-Reset werden die wartenden Agenten erneut ausgeführt und die Ausführung wird automatisch fortgesetzt. Erfordert Claude Code v2.1.271 oder später; in früheren Versionen schlagen die betroffenen Agenten fehl.

Während die Ausführung wartet, zeigen die Fortschrittszeile im Aufgabenpanel und der [`/workflows`](#watch-the-run)-Header an, wann das Limit zurückgesetzt wird.

Die Ausführung wird nur unterbrochen, wenn alle diese Bedingungen erfüllt sind; wenn eine nicht erfüllt ist, schlägt der betroffene Agent stattdessen fehl:

* Die Sitzung ist interaktiv und mit einem claude.ai-Abonnement angemeldet. Eine Ausführung wird nicht unterbrochen im [nicht-interaktiven Modus](/docs/de/headless) mit `claude -p` oder dem [Agent SDK](/docs/de/agent-sdk/overview), in einer [Hintergrund-Sitzung](/docs/de/agent-view), oder in einer [Remote Control](/docs/de/remote-control) oder [Agent-Team](/docs/de/agent-teams) Kollegensitzung.
* [`autoContinueAtUsageLimit`](/docs/de/settings-reference#autocontinueatusagelimit) ist aktiviert, die gleiche Einstellung, die die Sitzung selbst [auf einen Nutzungslimit-Reset warten lässt](/docs/de/interactive-mode#wait-for-a-usage-limit-to-reset). Wenn Sie sie während eines Wartevorgangs ausschalten, endet der Wartevorgang und die wartenden Agenten schlagen fehl.
* Das Limit wird innerhalb von 24 Stunden zurückgesetzt. Ein wöchentliches Limit kann weiter hinaus zurückgesetzt werden.
* Die Ausführung hat nicht bereits zweimal gewartet. Wenn sie das Limit zum dritten Mal erreicht, schlägt der Agent fehl.

<h3 id="cost">
  Kosten
</h3>

Ein Workflow spawnt viele Agenten, sodass eine einzelne Ausführung bedeutend mehr Token verwenden kann als die Bearbeitung der gleichen Aufgabe in einem Gespräch. Ausführungen zählen zur Nutzung und zu Ratenlimits Ihres Plans.

Um die Ausgaben vor der Verpflichtung zu einer großen Aufgabe zu schätzen, führen Sie den Workflow zunächst auf einem kleinen Ausschnitt aus: ein Verzeichnis statt des gesamten Repositorys oder eine enge Frage statt einer breiten. Die `/workflows`-Ansicht zeigt die Token-Nutzung jedes Agenten während der Ausführung an, und Sie können die Ausführung dort jederzeit beenden, ohne abgeschlossene Arbeiten zu verlieren. [Fortsetzen nach einer Pause](#resume-after-a-pause) behandelt, was eine gestoppte Ausführung behält. Die Laufzeit-[Agent-Limits](#behavior-and-limits) begrenzen, wie viele Agenten eine einzelne Ausführung spawnen kann, was die Kosten eines unkontrollierten Skripts begrenzt. Um Ausführungen auf weniger Agenten zu halten, wählen Sie die `small` [Größenrichtlinie](#set-a-size-guideline).

Claude Code kennzeichnet auch eine Ausführung, die ungewöhnlich groß wird. Wenn ein Workflow mehr als 25 Agenten plant oder seine projizierte Token-Gesamtzahl 1,5 Millionen überschreitet, zeigt die Fortschrittszeile im Aufgabenpanel unter dem Eingabefeld eine `Large workflow`-Warnung an. Die Warnung verweist Sie auf [`/workflows`](#watch-the-run), wo Sie die Ausführung beenden können.

Die Warnung ist informativ: Sie pausiert oder begrenzt die Ausführung nicht. Zwei Einstellungen ändern sich, wenn Sie sie sehen:

* Wenn Sie eine [Größenrichtlinie](#set-a-size-guideline) selbst wählen, ersetzt die Agentenzahl dieser Richtlinie den Schwellenwert von 25 Agenten. Die integrierte Standard-Richtlinie lässt den Schwellenwert bei 25.
* Sitzungen mit [ultracode](#let-claude-decide-with-ultracode) aktiviert zeigen die Warnung nicht an, da das Aktivieren von ultracode Sie bereits für große Ausführungen anmeldet.

Claude Code wählt das Modell jedes Workflow-Agenten in der gleichen [Reihenfolge, die es für Subagenten verwendet](/docs/de/sub-agents#choose-a-model). Ein Modell, das das Skript für eine Phase benennt, zählt als das Pro-Aufruf-Modell in dieser Reihenfolge. Wenn nichts anderes eines zuweist, wird der Agent auf dem Modell Ihrer Sitzung ausgeführt.

Um die Modellkosten zu kontrollieren:

* Überprüfen Sie `/model` vor einer großen Ausführung, wenn Sie normalerweise zu einem kleineren Modell für Routinearbeit wechseln
* Bitten Sie Claude, ein kleineres Modell für Phasen zu verwenden, die nicht das stärkste benötigen, wenn Sie die Aufgabe beschreiben

Wenn die [`availableModels`-Zulassungsliste](/docs/de/model-config#restrict-model-selection) Ihrer Organisation ein Modell blockiert, das das Skript für einen Agenten anfordert, wird dieser Agent stattdessen auf einem ersetzten Modell ausgeführt, wobei die gleichen [Ersetzungsregeln wie bei Subagenten](/docs/de/sub-agents#choose-a-model) gelten. Die Fortschrittsansicht der Ausführung in [`/workflows`](#watch-the-run) zeigt eine Warnung, die sowohl das angeforderte als auch das ersetzte Modell benennt.

<h3 id="set-a-size-guideline">
  Legen Sie eine Größenrichtlinie fest
</h3>

Eine Größenrichtlinie teilt Claude mit, wie viele Agenten angestrebt werden sollen, wenn es einen dynamischen Workflow schreibt. Claude Code sendet die Richtlinie an Claude als Ratschlag, nicht als Obergrenze, sodass ein Prompt, der einen anderen Maßstab fordert, diese Richtlinie immer noch überschreibt. Erfordert Claude Code v2.1.202 oder später.

Jeder Wert entspricht einer Agentenzahl:

| Wert           | Agentenzahl, auf die Claude abzielt                             |
| :------------- | :-------------------------------------------------------------- |
| `unrestricted` | Keine Richtlinie: Claude dimensioniert den Workflow zur Aufgabe |
| `small`        | Weniger als 5 Agenten                                           |
| `medium`       | Weniger als 10 Agenten                                          |
| `large`        | Weniger als 50 Agenten                                          |

Der Standard ist `medium`, oder `small`, wenn Sie mit Claude Code v2.1.271 oder später bei einem Pro-Plan angemeldet sind. Bis Sie einen Wert wählen, zeigt die `/config`-Zeile den Wert als Standard an, und die Zeile `Running in background` des Workflows benennt die geltende Größe. Erfordert Claude Code v2.1.219 oder später; frühere Versionen standardmäßig auf `unrestricted`.

Um die Richtlinie zu ändern, wählen Sie einen Wert für die Einstellung „Dynamic workflow size" in `/config` oder führen Sie `/config workflowSizeGuideline=small` aus. Auf v2.1.219 und später können Sie auch den [`workflowSizeGuideline`-Schlüssel](/docs/de/settings-reference#workflowsizeguideline) in jeder Einstellungsdatei festlegen; dieser Wert hat Vorrang vor `/config`, und Claude Code verbirgt die `/config`-Zeile, während eine Einstellungsdatei einen bereitstellt.

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

Wenn Workflows deaktiviert sind, sind die gebündelten Workflow-Befehle und die `/workflow-authoring`-Fähigkeit nicht verfügbar, das Schlüsselwort `ultracode` löst keine Ausführung mehr aus, und `ultracode` wird aus dem `/effort`-Menü entfernt.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Führen Sie Agenten parallel aus](/docs/de/agents): Vergleichen Sie Subagenten, Agent-Ansicht, Agent-Teams und Workflows
* [Erstellen Sie benutzerdefinierte Subagenten](/docs/de/sub-agents): Der Worker-Primitive, den Workflows orchestrieren
* [Verwalten Sie Kosten](/docs/de/costs): Wie Multi-Agent-Ausführungen zu Ihren Nutzungslimits zählen
