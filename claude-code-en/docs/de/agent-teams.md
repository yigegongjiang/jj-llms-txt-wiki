> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrieren Sie Teams von Claude Code-Sitzungen

> Koordinieren Sie mehrere Claude Code-Instanzen, die zusammen als Team arbeiten, mit gemeinsamen Aufgaben, Messaging zwischen Agenten und zentraler Verwaltung.

<Warning>
  Agent-Teams sind experimentell und standardmäßig deaktiviert. Aktivieren Sie sie, indem Sie `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` in Ihrer [settings.json](/docs/de/settings) oder Umgebung setzen. Ohne diese Variable wird kein Team beim Sitzungsstart eingerichtet, keine Team-Verzeichnisse werden geschrieben, und Claude spawnt oder schlägt keine Teammates vor. Agent-Teams haben [bekannte Einschränkungen](#limitations) bezüglich Sitzungswiederaufnahme, Aufgabenkoordination und Abschaltungsverhalten.
</Warning>

Agent-Teams ermöglichen es Ihnen, mehrere Claude Code-Instanzen zu koordinieren, die zusammenarbeiten. Eine Sitzung fungiert als Team-Lead und koordiniert die Arbeit, weist Aufgaben zu und synthetisiert Ergebnisse. Teammates arbeiten unabhängig, jeder in seinem eigenen Kontextfenster, und kommunizieren direkt miteinander. Sie können auch direkt mit jedem Teammate sprechen, ohne den Lead einzubeziehen.

Bevor Sie ein Team einrichten, überprüfen Sie, ob eine leichtere Option die Aufgabe erfüllt. [Subagents](/docs/de/sub-agents) arbeiten innerhalb einer einzelnen Sitzung, und mit [sitzungsübergreifendem Messaging](/docs/de/cross-session-messaging) kann Claude Erkenntnisse zwischen den Sitzungen weitergeben, die Sie selbst ausführen.

<h2 id="when-to-use-agent-teams">
  Wann Agent-Teams verwendet werden
</h2>

Agent-Teams sind am effektivsten für Aufgaben, bei denen parallele Exploration echten Wert bietet. Siehe [Anwendungsbeispiele](#use-case-examples) für vollständige Szenarien. Die stärksten Anwendungsfälle sind:

* **Recherche und Überprüfung**: mehrere Teammates können verschiedene Aspekte eines Problems gleichzeitig untersuchen und dann ihre Erkenntnisse austauschen und in Frage stellen
* **Neue Module oder Features**: Teammates können jeweils ein separates Stück besitzen, ohne sich gegenseitig zu behindern
* **Debugging mit konkurrierenden Hypothesen**: Teammates testen verschiedene Theorien parallel und konvergieren schneller zur Antwort
* **Schichtenübergreifende Koordination**: Änderungen, die Frontend, Backend und Tests umfassen, jeweils von einem anderen Teammate verwaltet

Agent-Teams fügen Koordinationsaufwand hinzu und verwenden deutlich mehr Tokens als eine einzelne Sitzung. Sie funktionieren am besten, wenn Teammates unabhängig arbeiten können. Für sequenzielle Aufgaben, Bearbeitungen in derselben Datei oder Arbeit mit vielen Abhängigkeiten sind eine einzelne Sitzung oder [subagents](/docs/de/sub-agents) effektiver.

<h3 id="compare-with-subagents">
  Vergleich mit subagents
</h3>

Sowohl Agent-Teams als auch [subagents](/docs/de/sub-agents) ermöglichen es Ihnen, Arbeit zu parallelisieren, aber sie funktionieren unterschiedlich. Für separate Sitzungen, die Nachrichten aneinander weitergeben, ohne ein Team zu bilden, siehe [sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging).

<Frame caption="Subagents berichten Ergebnisse an den Hauptagenten zurück. Bei Agent-Teams teilen sich Teammates eine Aufgabenliste, beanspruchen Arbeit und kommunizieren direkt miteinander.">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Diagramm zum Vergleich von Subagent- und Agent-Team-Architekturen. Subagents werden vom Hauptagenten erzeugt, führen Arbeit aus und berichten Ergebnisse zurück. Agent-Teams koordinieren sich über eine gemeinsame Aufgabenliste, wobei Teammates direkt miteinander kommunizieren." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Diagramm zum Vergleich von Subagent- und Agent-Team-Architekturen. Subagents werden vom Hauptagenten erzeugt, führen Arbeit aus und berichten Ergebnisse zurück. Agent-Teams koordinieren sich über eine gemeinsame Aufgabenliste, wobei Teammates direkt miteinander kommunizieren." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|                   | Subagents                                                                                                                                                                     | Agent-Teams                                                                                                                                                  |
| :---------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Kontext**       | Eigenes Kontextfenster; Ergebnisse kehren zum Aufrufer zurück                                                                                                                 | Eigenes Kontextfenster; vollständig unabhängig                                                                                                               |
| **Kommunikation** | Ergebnis an den Aufrufer zurückgeben. Subagents, die Claude beim Spawnen benannt hat, können sich auch [gegenseitig Nachrichten senden](/docs/de/sub-agents#what-loads-at-startup) | Teammates senden sich gegenseitig direkt Nachrichten                                                                                                         |
| **Koordination**  | Hauptagent verwaltet alle Arbeiten                                                                                                                                            | Selbstkoordination durch Nachrichten, plus eine gemeinsame Aufgabenliste für [Agenten, die die Task-Tools haben](/docs/de/tools-reference#task-tool-availability) |
| **Am besten für** | Fokussierte Aufgaben, bei denen nur das Ergebnis zählt                                                                                                                        | Komplexe Arbeit, die Diskussion und Zusammenarbeit erfordert                                                                                                 |
| **Token-Kosten**  | Niedriger: Ergebnisse werden zum Hauptkontext zusammengefasst                                                                                                                 | Höher: jeder Teammate ist eine separate Claude-Instanz                                                                                                       |

Verwenden Sie subagents, wenn Sie schnelle, fokussierte Worker benötigen, die berichten. Verwenden Sie Agent-Teams, wenn Teammates Erkenntnisse austauschen, sich gegenseitig in Frage stellen und selbst koordinieren müssen.

<h2 id="enable-agent-teams">
  Agent-Teams aktivieren
</h2>

Agent-Teams sind standardmäßig deaktiviert. Aktivieren Sie sie, indem Sie die Umgebungsvariable `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` auf `1` setzen, entweder in Ihrer Shell-Umgebung oder über [settings.json](/docs/de/settings):

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

Das Aktivieren von Agent-Teams ändert auch die gewöhnliche Delegation. Claude kann [einen Subagenten selbst benennen](/docs/de/sub-agents#subagent-names), und während Agent-Teams aktiviert sind, wird ein Subagent, den Claude benennt, als Teamkollege gestartet, sodass sich Teams auch dann bilden können, wenn Sie nicht um einen gebeten haben. Weitere Informationen finden Sie unter [Wie Claude Agent-Teams startet](#how-claude-starts-agent-teams); um das Verhalten auszuschalten, siehe [Claude startet Teamkollegen statt Subagenten](#claude-spawns-teammates-instead-of-subagents).

Das Starten von Teamkollegen erfordert auch eine interaktive Sitzung. Im [nicht-interaktiven Modus](/docs/de/headless) mit dem Flag `-p`, einschließlich Agent SDK-Sitzungen, startet Claude keine Teamkollegen, und ein Subagent, den Claude benennt, wird als gewöhnlicher [Subagent](/docs/de/sub-agents) ausgeführt, auch wenn Agent-Teams aktiviert sind.

<h2 id="start-your-first-agent-team">
  Starten Sie Ihr erstes Agent-Team
</h2>

Nach der Aktivierung von Agent-Teams beschreiben Sie die Aufgabe und die Teammates, die Sie möchten, in natürlicher Sprache. Claude erzeugt sie und koordiniert die Arbeit basierend auf Ihrem Prompt.

Dieses Beispiel funktioniert gut, weil die drei Rollen unabhängig sind und das Problem erkunden können, ohne aufeinander zu warten:

```text wrap theme={null}
I'm designing a CLI tool that helps developers track TODO comments across
their codebase. Spawn three teammates to explore this from different angles:
one on UX, one on technical architecture, one playing devil's advocate.
```

Von dort aus füllt Claude eine [gemeinsame Aufgabenliste](/docs/de/interactive-mode#task-list) in einer [Sitzung, die die Task-Tools hat](/docs/de/tools-reference#task-tool-availability), erzeugt Teammates für jede Perspektive, lässt sie das Problem erkunden und synthetisiert Erkenntnisse, wenn fertig.

Claude kann manchmal [Subagenten](/docs/de/sub-agents) statt eines Teams erstellen. Subagenten erscheinen im gleichen Agent-Panel wie Teammates, daher bestätigt das Panel allein nicht, dass ein Team gebildet wurde. Wenn Claude stattdessen Subagenten erzeugt hat, fragen Sie erneut und fordern Sie explizit ein Agent-Team an.

Das Terminal des Leads listet Teammates im Agent-Panel unterhalb der Prompt-Eingabe auf. Aus dem Panel:

* **Pfeile nach oben und unten**: Wählen Sie einen Teammate aus
* **Eingabe**: Öffnen Sie das Transkript des ausgewählten Teammates und senden Sie ihm direkt eine Nachricht
* **Escape**: Löschen Sie die Auswahl. Während Sie das Transkript eines Teammates anschauen, unterbricht Escape den aktuellen Zug dieses Teammates

Ab v2.1.199 bleibt die Zeile eines untätigen Teammates im Panel, während noch ein Teammate oder Subagent arbeitet, sodass Sie sie auswählen können, um sein Transkript zu überprüfen oder ihm mehr Arbeit zu geben. Sobald jeder Agent im Panel untätig ist, werden untätige Zeilen nach 30 Sekunden ausgeblendet und erscheinen beim nächsten Zug des Teammates wieder; der Teammate läuft weiter und ist adressierbar, während er ausgeblendet ist. In v2.1.181 bis v2.1.198 wurde eine untätige Zeile 30 Sekunden nach ihrem eigenen Zug ausgeblendet, auch während andere Teammates noch arbeiteten; untätige Zeilen werden in Versionen vor v2.1.181 nicht ausgeblendet.

Wenn mehr als drei Teammates gleichzeitig untätig sind, werden die Zeilen über die ersten drei hinaus in einer einzelnen Zeile zusammengefasst, die die zusammengefassten Teammates zählt, z. B. `2 idle agents`, wenn fünf untätig sind. Wählen Sie sie aus und drücken Sie die Eingabetaste, um die zusammengefassten Zeilen zu erweitern, oder drücken Sie Esc, um sie wieder zusammenzufassen. Arbeitende Teammates, fehlgeschlagene Teammates und der Teammate, den Sie gerade anschauen, behalten immer ihre eigenen Zeilen.

Wenn Sie jeden Teammate in seinem eigenen Split-Pane haben möchten, siehe [Wählen Sie einen Anzeigemodus](#choose-a-display-mode).

<h2 id="control-your-agent-team">
  Kontrolle Ihres Agent-Teams
</h2>

Teilen Sie dem Lead in natürlicher Sprache mit, was Sie möchten. Es kümmert sich um Teamkoordination, Aufgabenzuweisung und Delegation basierend auf Ihren Anweisungen.

<h3 id="choose-a-display-mode">
  Wählen Sie einen Anzeigemodus
</h3>

Agent-Teams unterstützen zwei Anzeigemodi:

* **In-Process**: alle Teammates laufen in Ihrem Hauptterminal. Verwenden Sie die Pfeiltasten nach oben und unten im Agent-Panel, um einen Teammate auszuwählen, drücken Sie dann die Eingabetaste, um ihn anzuzeigen und geben Sie ein, um ihm direkt eine Nachricht zu senden. Funktioniert in jedem Terminal, keine zusätzliche Einrichtung erforderlich.
* **Split Panes**: jeder Teammate erhält seinen eigenen Pane. Sie können die Ausgabe aller gleichzeitig sehen und in einen Pane klicken, um direkt zu interagieren. Erfordert tmux oder iTerm2.

<Note>
  `tmux` hat bekannte Einschränkungen auf bestimmten Betriebssystemen und funktioniert traditionell am besten auf macOS. Die Verwendung von `tmux -CC` in iTerm2 ist der empfohlene Einstiegspunkt in `tmux`.
</Note>

Der Standard ist `"in-process"`. Setzen Sie `"auto"`, um Split Panes zu aktivieren, wenn Sie bereits in einer tmux-Sitzung ausgeführt werden oder Ihr Terminal iTerm2 mit der installierten `it2` CLI ist, und fallen Sie ansonsten auf In-Process zurück. Die Einstellung `"tmux"` aktiviert den Split-Pane-Modus und erkennt automatisch, ob tmux oder iTerm2 basierend auf Ihrem Terminal verwendet werden soll.

Setzen Sie `"iterm2"`, um native Split Panes von iTerm2 explizit zu verwenden. Dieser Modus erfordert die [`it2` CLI](https://github.com/mkusaka/it2) und zeigt einen Fehler mit dem Installationsbefehl an, wenn `it2` fehlt. Die Setup-Eingabeaufforderung, die angeboten wird, `it2` zu installieren oder zu tmux zu wechseln, wird unter `"auto"` oder `"tmux"` angezeigt, wenn Ihr Terminal iTerm2 ist und tmux als Fallback verfügbar ist.

Um den Standard zu überschreiben, setzen Sie [`teammateMode`](/docs/de/settings-reference#teammatemode) in `~/.claude/settings.json`:

```json theme={null}
{
  "teammateMode": "auto"
}
```

Um den Modus für eine einzelne Sitzung festzulegen, übergeben Sie ihn als Flag:

```bash theme={null}
claude --teammate-mode auto
```

Das Flag `--teammate-mode` ist experimentell und wird nicht in `claude --help` angezeigt.

Der Split-Pane-Modus erfordert entweder [tmux](https://github.com/tmux/tmux/wiki) oder iTerm2 mit der [`it2` CLI](https://github.com/mkusaka/it2). Zur manuellen Installation:

* **tmux**: installieren Sie über den Paketmanager Ihres Systems. Siehe das [tmux Wiki](https://github.com/tmux/tmux/wiki/Installing) für plattformspezifische Anweisungen.
* **iTerm2**: installieren Sie die [`it2` CLI](https://github.com/mkusaka/it2), aktivieren Sie dann die Python-API in **iTerm2 → Settings → General → Magic → Enable Python API**.

<h3 id="specify-teammates-and-models">
  Geben Sie Teammates und Modelle an
</h3>

Claude entscheidet die Anzahl der zu erzeugenden Teammates basierend auf Ihrer Aufgabe, oder Sie können genau angeben, was Sie möchten:

```text wrap theme={null}
Spawn 4 teammates to refactor these modules in parallel. Use Sonnet for
each teammate.
```

Claude Code wählt das Modell jedes Teammates aus dem ersten dieser Punkte aus, der zutrifft:

1. Das Modell, das Ihr Spawn-Prompt für diesen Teammate benennt.
2. Für einen Teammate, der aus einer [Subagent-Definition](#use-subagent-definitions-for-teammates) erzeugt wird, die `model` der Definition, wobei `inherit` das Modell des Leads auswählt.
3. [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/de/model-config#environment-variables), wenn es auf etwas anderes als `inherit` gesetzt ist.
4. Das aktuelle Modell des Leads.

Wenn Sie [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE=1`](/docs/de/sub-agents#run-every-subagent-on-one-model) setzen, gelten die ersten beiden Quellen nicht. Claude Code wählt das Modell jedes Teammates aus `CLAUDE_CODE_SUBAGENT_MODEL`, wenn es auf etwas anderes als `inherit` gesetzt ist, und ansonsten aus dem aktuellen Modell des Leads. Erfordert Claude Code v2.1.257 oder später.

Vor v2.1.251 kam `CLAUDE_CODE_SUBAGENT_MODEL` zuerst in dieser Reihenfolge.

<Note>
  `teammateDefaultModel` wurde in v2.1.234 entfernt; Claude Code ignoriert einen verbleibenden Wert. Benennen Sie das Modell stattdessen in Ihrem Prompt.
</Note>

Claude Code überprüft das Modell, das es für einen Teammate auswählt, gegen die [`availableModels`](/docs/de/model-config#restrict-model-selection) Allowlist Ihrer Organisation. Wenn die Allowlist einen Wert blockiert, ersetzt Claude Code ein anderes Modell:

* **Family-Alias wie `opus`**: Auf der Anthropic API und Claude Platform auf AWS führt Claude Code den Teammate auf der neuesten Version dieser Familie aus, die die Allowlist zulässt. Bei Anbietern mit anbieterspezifischen Modell-IDs, bei denen die [Substitution nicht funktioniert](/docs/de/model-config#restrict-model-selection), fällt ein blockierter Alias wie jeder andere blockierte Wert gemäß dem nächsten Punkt zurück
* **Jeder andere blockierte Wert, einschließlich eines Family-Alias bei Anbietern, bei denen die Substitution nicht funktioniert, oder einer Familie, die keine zulässige Version hat**: Claude Code führt den Teammate auf dem Modell des Leads aus. Wenn Sie `CLAUDE_CODE_SUBAGENT_MODEL` setzen, versucht Claude Code zuerst dieses Modell unter denselben Regeln

Teammates erben die [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) des Leads. Im Split-Pane-Modus gilt dies ab v2.1.186; frühere Versionen haben die Anstrengungsstufe der Lead-Sitzung nicht an Split-Pane-Teammates weitergegeben.

<h3 id="have-teammates-plan-before-implementing">
  Genehmigung von Plänen für Teammates erforderlich
</h3>

Für komplexe oder riskante Aufgaben können Sie verlangen, dass Teammates planen, bevor sie implementieren. Ein Teammate, den Claude erzeugt, während der Lead im [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) ist, arbeitet im schreibgeschützten Plan-Modus, bis sein Plan fertig ist. Schalten Sie den Lead zuerst in den Plan-Modus und fragen Sie dann nach dem Teammate:

```text wrap theme={null}
Spawn an architect teammate to refactor the authentication module.
```

Wenn ein Teammate die Planung abgeschlossen hat, sendet er eine Genehmigungsanfrage an den Lead. Claude Code genehmigt den Plan in der Sitzung des Leads, sobald die Anfrage ankommt, ohne dass der Lead ihn überprüft. Die Änderungen und Befehle des Teammates durchlaufen weiterhin die in [Berechtigungen](#permissions) beschriebenen Genehmigungsaufforderungen. Nach der Genehmigung beendet der Teammate den Plan-Modus und beginnt mit der Implementierung.

<h3 id="talk-to-teammates-directly">
  Sprechen Sie direkt mit Teammates
</h3>

Jeder Teammate ist eine vollständige, unabhängige Claude Code-Sitzung. Sie können jedem Teammate direkt eine Nachricht senden, um zusätzliche Anweisungen zu geben, Folgefragen zu stellen oder seinen Ansatz umzuleiten.

* **In-Process-Modus**: verwenden Sie die Pfeiltasten nach oben und unten im Agent-Panel, um einen Teammate auszuwählen, drücken Sie dann die Eingabetaste, um seine Sitzung anzuzeigen und geben Sie ein, um ihm eine Nachricht zu senden. Drücken Sie `x` auf einem ausgewählten Teammate, um ihn zu stoppen. Drücken Sie Ctrl+T, um die Aufgabenliste umzuschalten.
* **Split-Pane-Modus**: klicken Sie in den Pane eines Teammates, um direkt mit seiner Sitzung zu interagieren. Jeder Teammate hat eine vollständige Ansicht seines eigenen Terminals.

Während Sie einen In-Process-Teammate anzeigen, gehen einfacher Text und [Skills](/docs/de/skills) an diesen Teammate, aber integrierte Befehle werden weiterhin in der Sitzung des Leads ausgeführt.

Das Modell und der schnelle Modus eines Teammates sind festgelegt, wenn er spawnt, daher ändern `/model` und `/fast` nur die Einstellungen des Leads. Ab v2.1.199 zeigt die Eingabe eines dieser Befehle während der Anzeige eines Teammates einen Hinweis an, dass die Änderung für den Lead gilt; frühere Versionen haben sie auf den Lead angewendet, ohne Hinweis. `/effort` gilt weiterhin für die späteren Züge des angezeigten Teammates, da Teammates die [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) des Leads befolgen.

<h3 id="assign-and-claim-tasks">
  Aufgaben zuweisen und beanspruchen
</h3>

Die gemeinsame Aufgabenliste koordiniert die Arbeit im Team. Der Lead erstellt Aufgaben und Teammates arbeiten sie durch. Aufgaben haben drei Zustände: ausstehend, in Bearbeitung und abgeschlossen. Aufgaben können auch von anderen Aufgaben abhängen: eine ausstehende Aufgabe mit ungelösten Abhängigkeiten kann nicht beansprucht werden, bis diese Abhängigkeiten erfüllt sind.

Agents [ohne die Task-Tools](/docs/de/tools-reference#task-tool-availability) koordinieren sich stattdessen durch Nachrichten.

Der Lead kann Aufgaben explizit zuweisen oder Teammates können selbst beanspruchen:

* **Lead weist zu**: teilen Sie dem Lead mit, welche Aufgabe welchem Teammate gegeben werden soll
* **Selbst beanspruchen**: nach Abschluss einer Aufgabe wählt ein Teammate die nächste nicht zugewiesene, nicht blockierte Aufgabe selbst aus

Das Beanspruchen von Aufgaben verwendet Dateisperrung, um Race Conditions zu verhindern, wenn mehrere Teammates versuchen, gleichzeitig dieselbe Aufgabe zu beanspruchen.

<h3 id="shut-down-teammates">
  Teammates herunterfahren
</h3>

Um die Sitzung eines Teammates ordnungsgemäß zu beenden, beziehen Sie sich auf ihn mit seinem Namen. Zum Beispiel mit einem Teammate namens researcher:

```text wrap theme={null}
Ask the researcher teammate to shut down
```

Der Lead sendet eine Abschaltungsanfrage. Der Teammate kann zustimmen und ordnungsgemäß beenden oder mit einer Erklärung ablehnen.

Die gemeinsamen Verzeichnisse des Teams werden automatisch bereinigt, wenn die Sitzung endet, daher gibt es keinen separaten Bereinigungsschritt. Siehe [Architektur](#architecture) für die Verzeichnisse, die entfernt werden, und welche für fortgesetzte Sitzungen bestehen bleiben.

<h3 id="enforce-quality-gates-with-hooks">
  Erzwingen Sie Qualitätsgates mit Hooks
</h3>

Verwenden Sie [Hooks](/docs/de/hooks), um Regeln durchzusetzen, wenn Teammates ihre Arbeit abschließen oder Aufgaben erstellt oder abgeschlossen werden:

* [`TeammateIdle`](/docs/de/hooks#teammateidle): wird ausgeführt, wenn ein Teammate im Begriff ist, untätig zu werden. Beenden Sie mit Code 2, um Feedback zu senden und den Teammate weiterarbeiten zu lassen.
* [`TaskCreated`](/docs/de/hooks#taskcreated): wird ausgeführt, wenn eine Aufgabe erstellt wird. Beenden Sie mit Code 2, um die Erstellung zu verhindern und Feedback zu senden.
* [`TaskCompleted`](/docs/de/hooks#taskcompleted): wird ausgeführt, wenn eine Aufgabe als abgeschlossen markiert wird. Beenden Sie mit Code 2, um die Fertigstellung zu verhindern und Feedback zu senden.

<h2 id="how-agent-teams-work">
  Wie Agent-Teams funktionieren
</h2>

Dieser Abschnitt behandelt die Architektur und Mechanik hinter Agent-Teams. Wenn Sie sie verwenden möchten, siehe [Kontrolle Ihres Agent-Teams](#control-your-agent-team) oben.

<h3 id="how-claude-starts-agent-teams">
  Wie Claude Agent-Teams startet
</h3>

Um ein Team zu starten, fordern Sie Claude um Teammates an. Claude startet einen Teammate, wenn er das [Agent-Tool](/docs/de/tools-reference) mit einem [`name`](/docs/de/sub-agents#subagent-names) aufruft, während Agent-Teams aktiviert sind, es sei denn, der Aufruf ist ein [Fork](/docs/de/sub-agents#fork-the-current-conversation) oder übergibt `isolation` beim Aufruf selbst. Claude Code fragt Sie nicht um Bestätigung für den Start.

Claude benennt auch gewöhnliche Subagenten von selbst, damit es sie später anschreiben kann. Diese Aufrufe folgen der gleichen Regel, sodass Teams sich auch dann bilden können, wenn Sie nicht um einen gebeten haben. Wenn Sie stattdessen Subagenten möchten, [schalten Sie Agent-Teams aus](#claude-spawns-teammates-instead-of-subagents).

<h3 id="architecture">
  Architektur
</h3>

Ein Agent-Team besteht aus:

| Komponente        | Rolle                                                                             |
| :---------------- | :-------------------------------------------------------------------------------- |
| **Team Lead**     | Die Haupt-Claude Code-Sitzung, die Teammates erzeugt und die Arbeit koordiniert   |
| **Teammates**     | Separate Claude Code-Instanzen, die jeweils an zugewiesenen Aufgaben arbeiten     |
| **Aufgabenliste** | Gemeinsame Liste von Arbeitselementen, die Teammates beanspruchen und abschließen |
| **Mailbox**       | Nachrichtensystem für Kommunikation zwischen Agenten                              |

Die Mailbox jedes Agenten ist eine JSON-Datei unter `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`. Claude Code validiert jeden Eintrag, wenn es eine Mailbox-Datei liest. Einträge, die nicht dem Nachrichtenformat entsprechen, werden als Fehler gemeldet und aus der Datei entfernt; die gültigen Nachrichten werden trotzdem zugestellt. Vor v2.1.207 verursachte ein einzelner fehlerhafter Mailbox-Eintrag einen wiederholten Fehler jede Sekunde und blockierte die Zustellung für diese Mailbox, bis Sie die Datei manuell löschten.

Claude Code meldet eine Nachricht als gesendet nur, wenn der Schreibvorgang in die Mailbox-Datei des Empfängers erfolgreich ist, ob die Nachricht Klartext oder eine strukturierte Protokollnachricht wie eine Plan-Genehmigung oder Shutdown-Anfrage ist. Wenn der Schreibvorgang fehlschlägt, beispielsweise weil der Speicher voll ist oder das Mailbox-Verzeichnis nicht beschreibbar ist, erhält der sendende Agent einen Fehler und nichts wird gesendet. Siehe [Fehler beim Schreiben in die Mailbox eines Teammates](/docs/de/errors#failed-to-write-to-a-teammate-inbox) für die Fehlermeldungen und Wiederherstellungsschritte.

Claude Code verwaltet Aufgabenabhängigkeiten automatisch: wenn ein Teammate eine Aufgabe abschließt, von der andere Aufgaben abhängen, werden die abhängigen Aufgaben ohne Aktion von Ihnen entsperrt.

Teams und Aufgaben werden lokal unter einem sitzungsabgeleiteten Namen gespeichert. Der Name ist `session-` gefolgt von den ersten acht Zeichen der Sitzungs-ID:

* **Team-Konfiguration**: `~/.claude/teams/{team-name}/config.json`
* **Aufgabenliste**: `~/.claude/tasks/{team-name}/`

Claude Code generiert beide automatisch beim Sitzungsstart und aktualisiert sie, wenn Teammates beitreten, untätig werden oder gehen. Das Team-Konfigurationsverzeichnis wird entfernt, wenn die Sitzung endet. Das Aufgabenlisten-Verzeichnis bleibt lokal bestehen und wird nie hochgeladen, sodass fortgesetzte Sitzungen ihre Aufgaben behalten. Die Aufbewahrung wird durch die gleiche [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays) gesteuert, die Sie bereits für Sitzungstranskripte kontrollieren, nach den [Aufbewahrungssweep-Regeln](/docs/de/claude-directory#cleaned-up-automatically).

Die Team-Konfiguration enthält Laufzeitzustand wie Sitzungs-IDs und tmux-Pane-IDs, also bearbeiten Sie sie nicht von Hand oder verfassen Sie sie nicht im Voraus: Ihre Änderungen werden beim nächsten Zustandsupdate überschrieben.

Um wiederverwendbare Teammate-Rollen zu definieren, verwenden Sie stattdessen [Subagent-Definitionen](#use-subagent-definitions-for-teammates).

Die Team-Konfiguration enthält ein `members`-Array mit dem Namen und der Agent-ID jedes Mitglieds. Der Eintrag des Leads trägt immer den Agent-Typ `team-lead`. Der Eintrag eines Teammates trägt den Agent-Typ, den der Lead beim Erzeugen benannt hat, ob ein [integrierter Typ](/docs/de/sub-agents#built-in-subagents) oder eine [Subagent-Definition](#use-subagent-definitions-for-teammates), und lässt das Feld weg, wenn der Lead keinen benannt hat. Teammates können diese Datei lesen, um andere Teammitglieder zu entdecken.

Es gibt kein Projekt-Level-Äquivalent der Team-Konfiguration. Eine Datei wie `.claude/teams/teams.json` in Ihrem Projektverzeichnis wird nicht als Konfiguration erkannt; Claude behandelt sie als gewöhnliche Datei.

<h3 id="use-subagent-definitions-for-teammates">
  Verwenden Sie Subagent-Definitionen für Teammates
</h3>

Beim Erzeugen eines Teammates in beiden Anzeigemodi können Sie einen [Subagent](/docs/de/sub-agents)-Typ aus dem Projekt, Benutzer oder verwalteten [Subagent-Bereich](/docs/de/sub-agents#choose-the-subagent-scope) referenzieren. Dies ermöglicht es Ihnen, eine Rolle einmal zu definieren, wie z. B. einen Security-Reviewer oder Test-Runner, und sie sowohl als delegierter Subagent als auch als Agent-Team-Teammate wiederzuverwenden.

Um eine Subagent-Definition zu verwenden, benennen Sie sie, wenn Sie Claude auffordern, den Teammate zu erzeugen:

```text wrap theme={null}
Spawn a teammate using the security-reviewer agent type to audit the auth module.
```

Claude Code liest die Subagent-Definition, die Sie benannt haben, und wendet diese Teile davon auf den Teammate an. Wenn ein Teil vom [Anzeigemodus](#choose-a-display-mode) des Teammates abhängt, sagt der Eintrag dies:

* **`tools`**: Claude Code beschränkt den Teammate auf die Tools in der `tools`-Liste der Definition. Für einen In-Process-Teammate fügt Claude Code `SendMessage` zu dieser Liste hinzu, und in einer [Sitzung, die die Task-Tools hat](/docs/de/tools-reference#task-tool-availability), fügt es auch `TaskCreate`, `TaskGet`, `TaskList` und `TaskUpdate` hinzu.
* **`model`**: Claude Code verwendet das `model` der Definition in beiden Anzeigemodi, wenn Ihr Spawn-Prompt keinen benennt. Siehe [wie Claude Code das Modell eines Teammates auswählt](#specify-teammates-and-models).
* **Body**: Für einen In-Process-Teammate hängt Claude Code den Body der Definition an seinen Standard-System-Prompt als zusätzliche Anweisungen an. Für einen Split-Pane-Teammate verwendet Claude Code den Body anstelle seines Standard-System-Prompts.
* **`skills`**: Claude Code wendet die `skills` der Definition nicht auf einen Teammate in beiden Anzeigemodi an. Der Teammate lädt Skills aus Ihren Projekt- und Benutzereinstellungen.
* **`mcpServers`**: Für einen Split-Pane-Teammate wendet Claude Code das `mcpServers` der Definition unter den [Regeln für dieses Feld](/docs/de/sub-agents#scope-mcp-servers-to-a-subagent) an, die auch eine Sitzung abdecken, die mit `--agent` gestartet wurde. Ein In-Process-Teammate ignoriert das Feld und lädt MCP-Server aus Ihren Projekt- und Benutzereinstellungen.

Wenn Claude einen In-Process-Teammate anschreibt, der nicht mehr läuft, bringt Claude Code ihn in der gleichen Sitzung zurück, stellt alle für ihn gespeicherten Gespräche wieder her und gibt ihm die Nachricht als seinen nächsten Prompt. Nachdem Sie eine Sitzung fortsetzen, werden Teammates auf diese Weise nicht zurückgebracht, gemäß [der Fortsetzen-Einschränkung](#limitations).

Für einen Teammate, den es zurückbringt, wendet Claude Code eine Definition, die aus dem `.claude/agents/`-Verzeichnis eines Projekts oder einem `--add-dir`-Verzeichnis stammt, nur erneut an, wenn Sie [dem Ordner vertraut haben, in dem die Agent-Datei ist](/docs/de/permissions#what-runs-before-you-trust-a-folder). Einem übergeordneten Ordner zu vertrauen zählt nicht. Bis dahin kommt der Teammate zurück, ohne die Tools oder Anweisungen der Definition zu haben, und behält nur die Tools, die Claude Code jedem In-Process-Teammate hinzufügt. Siehe [die Agent-Definition des Teammates wurde nicht wiederhergestellt](/docs/de/errors#teammate-agent-definition-not-restored) für den Benachrichtigungstext.

<h3 id="permissions">
  Berechtigungen
</h3>

Teammates starten mit dem Berechtigungsmodus des Leads, außer dem [`dontAsk`-Modus](/docs/de/permission-modes#allow-only-pre-approved-tools-with-dontask-mode), den sie nicht erben. Wenn der Lead mit `--dangerously-skip-permissions` ausgeführt wird, tun dies auch alle Teammates. Nach dem Erzeugen können Sie den Berechtigungsmodus eines einzelnen Teammates ändern, aber Sie können keine Pro-Teammate-Berechtigungsmodi zum Zeitpunkt des Erzeugung setzen.

Teammate-Berechtigungsaufforderungen werden an die Lead-Sitzung weitergeleitet, also genehmigen Sie sie dort selbst. [Plan-Genehmigung](#have-teammates-plan-before-implementing) ist die konzipierte Ausnahme: die Lead-Sitzung gewährt Teammate-Plan-Genehmigungen ohne eine separate Aufforderung an Sie.

<h4 id="messages-between-agents">
  Nachrichten zwischen Agenten
</h4>

Wenn ein Agent einem anderen eine Nachricht über `SendMessage` sendet, wird dem empfangenden Agent mitgeteilt, dass sie von einer anderen Claude-Sitzung stammt, nicht von Ihnen. Ein Teammate kann eine Berechtigungsaufforderung nicht genehmigen oder Zustimmung in Ihrem Namen erteilen, und ein Teammate, dem eine Aktion verweigert wurde, kann sie nicht an einen anderen Teammate weitergeben, um die Überprüfung zu umgehen. Die gleichen Regeln gelten für eine Nachricht, die von [einer Ihrer anderen Claude Code-Sitzungen](/docs/de/cross-session-messaging#how-a-session-treats-an-incoming-message) ankommt, außerhalb des Teams ganz.

Im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) wendet der Klassifizierer zwei Überprüfungen auf Nachrichten zwischen Agenten an:

* Er behandelt einen Genehmigungsanspruch, der von einem anderen Agenten weitergeleitet wird, als nicht vertrauenswürdige Eingabe statt als Bestätigung von Ihnen.
* Er überprüft jede Nachricht, bevor Claude Code sie liefert, ob eine einfache Nachricht oder eine strukturierte Protokollnachricht wie eine Shutdown-Anfrage oder Plan-Genehmigungsantwort. Eine Nachricht, die er blockiert, erreicht den Empfänger nie.

<h3 id="context-and-communication">
  Kontext und Kommunikation
</h3>

Jeder Teammate hat sein eigenes Kontextfenster. Beim Erzeugen lädt ein Teammate denselben Projektkontext wie eine reguläre Sitzung: CLAUDE.md, MCP-Server und Skills. Er erhält auch den Spawn-Prompt vom Lead. Die Gesprächshistorie des Leads wird nicht übertragen.

**Wie Teammates Informationen teilen:**

* **Automatische Nachrichtenlieferung**: wenn Teammates Nachrichten senden, werden sie automatisch an Empfänger geliefert. Der Lead muss nicht auf Updates abfragen.
* **Untätigkeitsbenachrichtigungen**: wenn ein Teammate fertig ist und stoppt, benachrichtigt er automatisch den Lead und enthält seine endgültige Antwort in der Benachrichtigung. Ein Teammate, dessen Zug bei einem API-Fehler endet, benachrichtigt den Lead, dass er fehlgeschlagen ist, und enthält den Fehlertext.
* **Gemeinsame Aufgabenliste**: [Agenten, die die Task-Tools haben](/docs/de/tools-reference#task-tool-availability), können den Aufgabenstatus sehen und verfügbare Arbeit beanspruchen.
* **Teammate-Messaging**: senden Sie eine Nachricht an einen bestimmten Teammate nach Name. Um alle zu erreichen, senden Sie eine Nachricht pro Empfänger.

Der Lead weist jedem Teammate einen Namen zu, wenn er ihn erzeugt, und jeder Teammate kann jeden anderen nach diesem Namen anschreiben. Um vorhersehbare Namen zu erhalten, die Sie in späteren Prompts referenzieren können, teilen Sie dem Lead mit, wie er jeden Teammate in Ihrer Spawn-Anweisung nennen soll.

<h3 id="token-usage">
  Token-Nutzung
</h3>

Agent-Teams verwenden deutlich mehr Tokens als eine einzelne Sitzung. Jeder Teammate hat sein eigenes Kontextfenster, und die Token-Nutzung skaliert mit der Anzahl der aktiven Teammates. Für Recherche, Überprüfung und neue Feature-Arbeit sind die zusätzlichen Tokens normalerweise lohnenswert. Für Routineaufgaben ist eine einzelne Sitzung kostengünstiger. Siehe [Agent-Team-Token-Kosten](/docs/de/costs#agent-team-token-costs) für Nutzungsleitfäden.

Ein In-Process-Teammate's Anfragen fallen außerhalb des Cache-TTL-Buckets der Hauptkonversation]\(/de/prompt-caching#which-ttl-each-request-gets), sodass sein Cache standardmäßig fünf Minuten hält, auch bei einem Claude-Abonnement. Um es für eine Stunde zu behalten, setzen Sie [`subagentPromptCacheTtl`](/docs/de/settings-reference#subagentpromptcachettl) auf `1h`. Die API berechnet 1-Stunden-Cache-Schreibvorgänge mit einem höheren Satz.

<h2 id="use-case-examples">
  Anwendungsbeispiele
</h2>

Diese Beispiele zeigen, wie Agent-Teams Aufgaben handhaben, bei denen parallele Exploration Wert bietet.

<h3 id="run-a-parallel-code-review">
  Führen Sie eine parallele Code-Überprüfung durch
</h3>

Ein einzelner Reviewer neigt dazu, sich jeweils auf eine Art von Problem zu konzentrieren. Das Aufteilen von Überprüfungskriterien in unabhängige Domänen bedeutet, dass Sicherheit, Leistung und Testabdeckung alle gleichzeitig gründlich beachtet werden. Der Prompt weist jedem Teammate eine unterschiedliche Perspektive zu, damit sie sich nicht überlappen:

```text wrap theme={null}
Spawn three teammates to review PR #142:
- One focused on security implications
- One checking performance impact
- One validating test coverage
Have them each review and report findings.
```

Jeder Reviewer arbeitet vom selben PR aus, wendet aber einen anderen Filter an. Der Lead synthetisiert Erkenntnisse über alle drei nach Abschluss.

<h3 id="investigate-with-competing-hypotheses">
  Untersuchen Sie mit konkurrierenden Hypothesen
</h3>

Wenn die Grundursache unklar ist, neigt ein einzelner Agent dazu, eine plausible Erklärung zu finden und zu stoppen. Der Prompt bekämpft dies, indem er Teammates explizit gegnerisch macht: die Aufgabe jedes ist nicht nur, seine eigene Theorie zu untersuchen, sondern auch die anderen in Frage zu stellen.

```text wrap theme={null}
Users report the app exits after one message instead of staying connected.
Spawn 5 agent teammates to investigate different hypotheses. Have them talk to
each other to try to disprove each other's theories, like a scientific
debate. Update the findings doc with whatever consensus emerges.
```

Die Debattenstruktur ist der Schlüsselmechanismus hier. Sequenzielle Untersuchung leidet unter Verankerung: sobald eine Theorie untersucht wird, ist die nachfolgende Untersuchung zu ihr vorgespannt.

Mit mehreren unabhängigen Ermittlern, die aktiv versuchen, sich gegenseitig zu widerlegen, ist die Theorie, die überlebt, viel wahrscheinlicher die tatsächliche Grundursache.

<h2 id="best-practices">
  Best Practices
</h2>

<h3 id="give-teammates-enough-context">
  Geben Sie Teammates genug Kontext
</h3>

Teammates laden Projektkontext automatisch, einschließlich CLAUDE.md, MCP-Server und Skills, aber sie erben nicht die Gesprächshistorie des Leads. Siehe [Kontext und Kommunikation](#context-and-communication) für Details. Fügen Sie aufgabenspezifische Details in den Spawn-Prompt ein:

```text wrap theme={null}
Spawn a security reviewer teammate with the prompt: "Review the authentication module
at src/auth/ for security vulnerabilities. Focus on token handling, session
management, and input validation. The app uses JWT tokens stored in
httpOnly cookies. Report any issues with severity ratings."
```

<h3 id="choose-an-appropriate-team-size">
  Wählen Sie eine angemessene Teamgröße
</h3>

Es gibt keine harte Grenze für die Anzahl der Teammates, aber praktische Einschränkungen gelten:

* **Token-Kosten skalieren linear**: jeder Teammate hat sein eigenes Kontextfenster und verbraucht Tokens unabhängig. Siehe [Agent-Team-Token-Kosten](/docs/de/costs#agent-team-token-costs) für Details.
* **Koordinationsaufwand nimmt zu**: mehr Teammates bedeutet mehr Kommunikation, Aufgabenkoordination und Konfliktpotenzial
* **Sinkende Erträge**: über einen bestimmten Punkt hinaus beschleunigen zusätzliche Teammates die Arbeit nicht proportional

Beginnen Sie mit 3-5 Teammates für die meisten Workflows. Dies balanciert parallele Arbeit mit verwaltbarer Koordination. Wenn Sie 15 unabhängige Aufgaben haben, sind 3 Teammates ein guter Ausgangspunkt.

Skalieren Sie nur auf, wenn die Arbeit wirklich davon profitiert, dass Teammates gleichzeitig arbeiten. Drei fokussierte Teammates übertreffen oft fünf verstreute.

<h3 id="size-tasks-appropriately">
  Dimensionieren Sie Aufgaben angemessen
</h3>

* **Zu klein**: Koordinationsaufwand übersteigt den Nutzen
* **Zu groß**: Teammates arbeiten zu lange ohne Check-ins, was das Risiko verschwendeter Anstrengungen erhöht
* **Genau richtig**: in sich geschlossene Einheiten, die ein klares Ergebnis liefern, wie eine Funktion, eine Testdatei oder eine Überprüfung

<Tip>
  Der Lead teilt Arbeit in Aufgaben auf und weist sie Teammates automatisch zu. Wenn er nicht genug Aufgaben erstellt, bitten Sie ihn, die Arbeit in kleinere Stücke aufzuteilen. Mit 5-6 Aufgaben pro Teammate bleibt jeder produktiv und der Lead kann Arbeit neu zuweisen, wenn jemand steckenbleibt.
</Tip>

<h3 id="wait-for-teammates-to-finish">
  Warten Sie, bis Teammates fertig sind
</h3>

Manchmal beginnt der Lead, Aufgaben selbst zu implementieren, anstatt auf Teammates zu warten. Wenn Sie dies bemerken:

```text wrap theme={null}
Wait for your teammates to complete their tasks before proceeding
```

<h3 id="start-with-research-and-review">
  Beginnen Sie mit Recherche und Überprüfung
</h3>

Wenn Sie neu bei Agent-Teams sind, beginnen Sie mit Aufgaben, die klare Grenzen haben und nicht das Schreiben von Code erfordern: Überprüfung eines PR, Recherche einer Bibliothek oder Untersuchung eines Bugs. Diese Aufgaben zeigen den Wert paralleler Exploration ohne die Koordinationschallenges, die mit paralleler Implementierung einhergehen.

<h3 id="avoid-file-conflicts">
  Vermeiden Sie Dateikonflikte
</h3>

Zwei Teammates, die dieselbe Datei bearbeiten, führen zu Überschreibungen. Teilen Sie die Arbeit so auf, dass jeder Teammate einen anderen Satz von Dateien besitzt.

<h3 id="monitor-and-steer">
  Überwachen und lenken Sie
</h3>

Überprüfen Sie den Fortschritt der Teammates, leiten Sie Ansätze um, die nicht funktionieren, und synthetisieren Sie Erkenntnisse, wenn sie eintreffen. Ein Team zu lange unbeaufsichtigt laufen zu lassen, erhöht das Risiko verschwendeter Anstrengungen.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="teammates-not-appearing">
  Teammates erscheinen nicht
</h3>

Wenn Teammates nicht erscheinen, nachdem Sie Claude aufgefordert haben, sie zu erzeugen:

* Im In-Process-Modus erscheinen Teammates im Agent-Panel unterhalb der Eingabeaufforderung. Verwenden Sie die Pfeiltasten nach oben und unten, um einen auszuwählen, und drücken Sie dann die Eingabetaste, um ihn anzuzeigen.
* Eine Teammate-Zeile, die nach dem Leerlauf verschwunden ist, wurde ausgeblendet, nicht gestoppt. Leerlauf-Zeilen werden 30 Sekunden nach dem Leerlauf des gesamten Panels ausgeblendet und erscheinen beim nächsten Zug des Teammates wieder. Wenn mehr als drei Teammates untätig sind, werden ihre überschüssigen Zeilen in einer einzelnen `N idle agents`-Zeile zusammengefasst, die Enter erweitert. Senden Sie dem Teammate eine Nachricht nach Name, um eine ausgeblendete Zeile zurückzubringen.
* Überprüfen Sie, dass die Aufgabe, die Sie Claude gegeben haben, komplex genug war, um ein Team zu rechtfertigen. Claude entscheidet basierend auf der Aufgabe, ob Teammates erzeugt werden sollen.
* Wenn Sie explizit Split Panes angefordert haben, stellen Sie sicher, dass tmux installiert ist und in Ihrem PATH verfügbar ist:
  ```bash theme={null}
  which tmux
  ```
* Für iTerm2 überprüfen Sie, dass die `it2` CLI installiert ist und die Python-API in iTerm2-Einstellungen aktiviert ist.

<h3 id="claude-spawns-teammates-instead-of-subagents">
  Claude erzeugt Teammates statt Subagenten
</h3>

Während Agent-Teams aktiviert sind, wird ein Subagent, den Claude in der Sitzung des Lead benennt, als Teammate gestartet. Claude [kann Subagenten selbstständig benennen](#how-claude-starts-agent-teams), daher kann dies während einer Delegation geschehen, die Sie nie als Teamarbeit formuliert haben.

Um benannte Subagenten wieder als Subagenten zu starten, deaktivieren Sie Agent-Teams, indem Sie `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` auf `0` setzen:

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "0"
  }
}
```

Sie müssen keine neue Sitzung starten: Claude Code wendet `env`-Werte aus der Einstellungsdatei auf die laufende Sitzung an, wenn Sie speichern, und liest die Variable jedes Mal neu, wenn Claude einen Subagenten erzeugt, sodass der nächste Subagent, den Claude benennt, als Subagent gestartet wird.

Das Setzen der Variable auf `0` in Ihrer Benutzer-`settings.json` überschreibt einen Shell-Export. Andere Einstellungsquellen können Agent-Teams immer noch aktivieren:

* **Einstellungsdateien mit höherer Priorität**: Projekteinstellungen, lokale Einstellungen und eine `--settings`-Nutzlast werden nach Benutzereinstellungen angewendet, daher gewinnt ein `env`-Eintrag, der die Variable auf `1` in einer von ihnen setzt. Siehe [Einstellungspriorität](/docs/de/settings#settings-precedence).
* **Verwaltete Einstellungen**: [Verwaltete Einstellungen](/docs/de/server-managed-settings) werden nach jeder anderen Quelle angewendet. Wenn Ihre Organisation Agent-Teams dort aktiviert, bitten Sie Ihren Administrator, den verwalteten Wert zu ändern.

Nach der Änderung kann Claude immer noch Subagenten benennen, und der Name funktioniert weiterhin als [`SendMessage`-Adresse](/docs/de/sub-agents#resume-subagents). Claude erhält das Ergebnis jedes Subagenten, wenn dieser abgeschlossen ist.

<h3 id="too-many-permission-prompts">
  Zu viele Berechtigungsaufforderungen
</h3>

Teammate-Berechtigungsanfragen sprudeln zum Lead auf, was zu Reibung führen kann. Genehmigen Sie häufige Operationen in Ihren [Berechtigungseinstellungen](/docs/de/permissions) vor dem Erzeugen von Teammates, um Unterbrechungen zu reduzieren.

<h3 id="agents-stopping-early">
  Agents stoppen früh
</h3>

Teammates können nach Fehlern stoppen, anstatt sich zu erholen. Überprüfen Sie ihre Ausgabe, indem Sie den Teammate im Agent-Panel auswählen und die Eingabetaste im In-Process-Modus drücken, oder indem Sie im Split-Modus auf den Pane klicken, dann entweder:

* Geben Sie ihnen zusätzliche Anweisungen direkt
* Erzeugen Sie einen Ersatz-Teammate, um die Arbeit fortzusetzen

Eine Nachricht vom Lead oder einem anderen Teammate weckt einen In-Process-Teammate auf, der darauf wartet, eine fehlgeschlagene API-Anfrage erneut zu versuchen, sodass er sofort erneut versucht wird, anstatt auf die vollständige Wiederholungsverzögerung zu warten.

Der Lead kann auch früh stoppen und entscheiden, dass das Team fertig ist, bevor alle Aufgaben tatsächlich abgeschlossen sind. Wenn das geschieht, teilen Sie ihm mit, dass er weitermachen soll.

<h3 id="orphaned-tmux-sessions">
  Verwaiste tmux-Sitzungen
</h3>

Wenn eine tmux-Sitzung nach dem Ende der Claude Code-Sitzung bestehen bleibt, wurde sie möglicherweise nicht vollständig bereinigt. Listen Sie Sitzungen auf und beenden Sie die vom Team erstellte:

```bash theme={null}
tmux ls
tmux kill-session -t <session-name>
```

<h2 id="limitations">
  Einschränkungen
</h2>

Agent-Teams sind experimentell. Aktuelle Einschränkungen, die Sie beachten sollten:

* **Keine Sitzungswiederaufnahme mit In-Process-Teammates**: `/resume` und `/rewind` stellen In-Process-Teammates nicht wieder her. Nach der Wiederaufnahme einer Sitzung kann der Lead versuchen, mit Teammates zu kommunizieren, die nicht mehr existieren. Wenn dies geschieht, teilen Sie dem Lead mit, neue Teammates zu erzeugen.
* **Aufgabenstatus kann verzögert sein**: Teammates markieren Aufgaben manchmal nicht als abgeschlossen, was abhängige Aufgaben blockiert. Wenn eine Aufgabe steckenbleibt, überprüfen Sie, ob die Arbeit tatsächlich erledigt ist, und aktualisieren Sie den Aufgabenstatus manuell oder teilen Sie dem Lead mit, den Teammate zu anstoßen.
* **Abschaltung kann langsam sein**: Teammates beenden ihre aktuelle Anfrage oder ihren Werkzeugaufruf, bevor sie herunterfahren, was Zeit in Anspruch nehmen kann.
* **Ein Team pro Sitzung**: eine Sitzung hat genau ein Team, das auf diese Sitzung beschränkt ist. Sie können keine zusätzlichen benannten Teams erstellen oder ein Team über Sitzungen hinweg freigeben.
* **Keine verschachtelten Teams**: Teammates können ihre eigenen Teammates nicht erzeugen. Nur der Lead kann das Team verwalten.
* **Keine Hintergrund-Subagenten von In-Process-Teammates**: die eigenen Subagenten eines In-Process-Teammates laufen im Vordergrund, da die Hintergrundarbeit eines Teammates nicht länger als der Prozess des Leads bestehen kann. Claude Code gibt einen Fehler zurück, wenn ein Teammate einen Subagenten erzeugt, dessen Definition `background: true` setzt. Eine `run_in_background: true`-Anfrage eines Teammates schlägt ebenfalls fehl, entweder mit einem Fehler oder durch stilles Ausführen im Vordergrund, wie in [wie Claude Code Vordergrund oder Hintergrund auswählt](/docs/de/sub-agents#run-subagents-in-foreground-or-background) beschrieben. Subagenten, die aus der Hauptkonversation gestartet werden, folgen dem [Hintergrund-Standard](/docs/de/sub-agents#run-subagents-in-foreground-or-background).
* **Lead ist fest**: die Hauptsitzung ist der Lead für seine Lebensdauer. Sie können einen Teammate nicht zum Lead befördern oder die Führung übertragen.
* **Berechtigungen beim Erzeugen gesetzt**: Teammates starten mit dem Berechtigungsmodus, der unter [Berechtigungen](#permissions) beschrieben ist. Sie können den Berechtigungsmodus eines einzelnen Teammates nach dem Erzeugen ändern, aber Sie können keine Pro-Teammate-Berechtigungsmodi zum Zeitpunkt des Erzeugung setzen.
* **Split Panes erfordern tmux oder iTerm2**: der Standard-In-Process-Modus funktioniert in jedem Terminal. Der Split-Pane-Modus wird in VS Code's integriertem Terminal, Windows Terminal oder Ghostty nicht unterstützt.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Erkunden Sie verwandte Ansätze für parallele Arbeit und Delegation:

* **Leichte Delegation**: [subagents](/docs/de/sub-agents) erzeugen Helper-Agenten für Recherche oder Überprüfung innerhalb Ihrer Sitzung, besser für Aufgaben, die keine Inter-Agent-Koordination benötigen
* **Messaging zwischen Ihren eigenen Sitzungen**: [Cross-Session-Messaging](/docs/de/cross-session-messaging) ermöglicht es Claude, Erkenntnisse zwischen den Sitzungen zu übergeben, die Sie selbst ausführen
* **Manuelle parallele Sitzungen**: [Git worktrees](/docs/de/worktrees) ermöglichen es Ihnen, mehrere Claude Code-Sitzungen selbst ohne automatisierte Teamkoordination auszuführen
