> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agenten parallel ausführen

> Vergleichen Sie die Möglichkeiten, wie Claude Code mehrere Aufgaben gleichzeitig bewältigen kann: Subagenten, Agent-Ansicht, Agent-Teams und dynamische Workflows.

[Subagenten](/docs/de/sub-agents), [Agent-Ansicht](/docs/de/agent-view), [Agent-Teams](/docs/de/agent-teams) und [dynamische Workflows](/docs/de/workflows) parallelisieren die Arbeit jeweils auf unterschiedliche Weise. Die richtige Wahl hängt davon ab, ob Sie in jeder Konversation selbst bleiben möchten, Aufgaben delegieren und später überprüfen möchten, oder ob Claude eine Gruppe von Mitarbeitern für Sie koordinieren soll.

| Ansatz                                | Was Sie erhalten                                                                                                                                                                                          | Verwenden Sie es, wenn                                                                                                                                                                                                                                        |
| :------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Subagenten](/docs/de/sub-agents)          | Delegierte Mitarbeiter in einer Sitzung, die eine Nebenaufgabe in ihrem eigenen Kontext ausführen und eine Zusammenfassung zurückgeben                                                                    | Eine Nebenaufgabe würde Ihre Hauptkonversation mit Suchergebnissen, Protokollen oder Dateiinhalten überfluten, auf die Sie nicht mehr verweisen werden                                                                                                        |
| [Agent-Ansicht](/docs/de/agent-view)       | Ein Bildschirm zum Versenden und Überwachen von Sitzungen, die im Hintergrund ausgeführt werden, geöffnet mit `claude agents`. Forschungsvorschau                                                         | Sie haben mehrere unabhängige Aufgaben und möchten diese delegieren, den Status auf einen Blick überprüfen und nur eingreifen, wenn eine Aufgabe Sie benötigt                                                                                                 |
| [Agent-Teams](/docs/de/agent-teams)        | Mehrere koordinierte Sitzungen mit einer gemeinsamen Aufgabenliste und Messaging zwischen Agenten, verwaltet von einem Lead. Experimentell und standardmäßig deaktiviert                                  | Sie möchten, dass Claude ein Projekt in Teile aufteilt, diese zuweist und die Mitarbeiter synchron hält                                                                                                                                                       |
| [Dynamische Workflows](/docs/de/workflows) | Ein Skript, das viele Subagenten ausführt und deren Ergebnisse überprüft, für einen Job, der zu groß ist, um ihn in einem Durchgang zu koordinieren, oder der mehr als einen einzelnen Durchgang benötigt | Ein Job wächst über eine Handvoll Subagenten hinaus, oder Sie möchten, dass Ergebnisse gegeneinander überprüft werden: ein codebase-weites Audit, eine 500-Datei-Migration, überprüfte Recherche oder ein Plan, der aus mehreren Blickwinkeln entworfen wurde |

In jedem Ansatz sind die Mitarbeiter Claude-Sitzungen. Um ein anderes Tool einzubeziehen, stellen Sie es Claude als [MCP-Server](/docs/de/mcp) zur Verfügung.

Zwei weitere Tools unterstützen diese Arbeit, ohne selbst eine Möglichkeit zu sein, Agenten auszuführen:

* [Worktrees](/docs/de/worktrees) geben jeder Sitzung einen separaten Git-Checkout, sodass parallele Sitzungen niemals dieselben Dateien bearbeiten. Verwenden Sie sie für Sitzungen, die Sie selbst ausführen. Die Agent-Ansicht verschiebt jede versendete Sitzung automatisch in ihren eigenen Worktree, und Subagenten, die Sie spawnen, können jeweils einen erhalten.
* [`/batch`](/docs/de/commands) ist ein [Skill](/docs/de/skills), der Claude eine große Änderung in 5 bis 30 Worktree-isolierte Subagenten aufteilt, die jeweils einen Pull Request öffnen. Es ist eine gepackte Verwendung von Subagenten und Worktrees, keine separate Koordinationsstil.

Einige andere Funktionen führen Claude aus, ohne dass Sie jeden Schritt steuern, aber sie lösen ein anderes Problem als die Aufteilung der Arbeit auf Agenten:

* Ein [Bash-Befehl im Hintergrund](/docs/de/interactive-mode#background-bash-commands) führt einen Shell-Befehl aus, ohne die Konversation zu blockieren. Es spawnt keinen Agenten.
* Ein [abgezweigter Subagent](/docs/de/sub-agents#fork-the-current-conversation) ist ein Subagent, der Ihren vollständigen Konversationskontext erbt, anstatt neu zu beginnen. Es ist eine Möglichkeit, einen Subagenten zu spawnen, keine separate Oberfläche.
* Eine [Routine](/docs/de/routines) führt eine Sitzung nach einem Zeitplan in Anthropics Cloud aus, nicht parallel auf Ihrem Computer.

<Note>
  Das gleichzeitige Ausführen mehrerer Sitzungen oder Subagenten vervielfacht die Token-Nutzung. Siehe [Kosten](/docs/de/costs) für Details zur Nutzung und Rate-Limit-Details.
</Note>

<h2 id="choose-an-approach">
  Wählen Sie einen Ansatz
</h2>

Der richtige Ansatz hängt davon ab, wer die Arbeit koordiniert, ob die Mitarbeiter kommunizieren müssen und ob sie dieselben Dateien bearbeiten:

* **Wer koordiniert die Arbeit?**
  * Claude delegiert und sammelt Ergebnisse in einer Konversation: [Subagenten](/docs/de/sub-agents)
  * Sie übergeben unabhängige Aufgaben und überprüfen später: [Agent-Ansicht](/docs/de/agent-view)
  * Claude plant, weist zu und beaufsichtigt eine Gruppe von Mitarbeitern: [Agent-Teams](/docs/de/agent-teams), experimentell und standardmäßig deaktiviert
  * Ein Skript hält den Plan statt Claudes schrittweiser Beurteilung: [dynamische Workflows](/docs/de/workflows). Siehe [wie Workflows mit Subagenten und Skills verglichen werden](/docs/de/workflows#when-to-use-a-workflow)
* **Müssen die Mitarbeiter miteinander kommunizieren?** Subagenten berichten Ergebnisse an die Konversation, die sie spawned hat, und Agent-Ansicht-Sitzungen berichten nur an Sie. Teammates in einem Agent-Team teilen eine Aufgabenliste und senden sich gegenseitig direkt Nachrichten.
* **Betreffen die Aufgaben dieselben Dateien?** Isolieren Sie die Arbeit mit [Worktrees](/docs/de/worktrees). Subagenten und Sitzungen, die Sie selbst ausführen, können jeweils einen separaten Worktree verwenden. Agent-Teams isolieren Teammates nicht in Worktrees, daher [partitionieren Sie die Arbeit](/docs/de/agent-teams#avoid-file-conflicts), damit jeder Teammate einen anderen Satz von Dateien besitzt.

<h2 id="check-on-running-work">
  Überprüfen Sie laufende Arbeiten
</h2>

Der Befehl zum Überprüfen laufender Arbeiten hängt davon ab, welchen Ansatz Sie verwendet haben:

* Für Hintergrund-Sitzungen öffnet `claude agents` die [Agent-Ansicht](/docs/de/agent-view): ein Bildschirm, der jede Sitzung, ihren Status und die Sitzungen anzeigt, die Ihre Eingabe benötigen.
* Für Subagenten in der aktuellen Sitzung erscheinen benannte Hintergrund-Subagenten in der @-Mention-Typeahead mit ihrem Status. Ab v2.1.198 öffnet `/agents` kein Panel mehr; es gibt einen Hinweis aus, der auf die Speicherorte der Subagenten-Dateien verweist. Um [benutzerdefinierte Subagenten zu erstellen und zu bearbeiten](/docs/de/sub-agents#configure-subagents), fragen Sie Claude oder bearbeiten Sie die Dateien direkt. Trotz des ähnlichen Namens ist `/agents` getrennt von `claude agents`.
* Für alles, das im Hintergrund der aktuellen Sitzung ausgeführt wird, listet `/tasks` jedes Element auf und ermöglicht es Ihnen, es zu überprüfen, sich daran anzuhängen oder es zu stoppen. Die Liste enthält auch Subagenten, die fertig sind.
* Für dynamische Workflows listet `/workflows` laufende und abgeschlossene Ausführungen, die Phase, in der sich jede befindet, und wie viele Agenten fertig sind, auf.

Für eine Desktop-Ansicht aller Ihrer Sitzungen siehe [parallele Sitzungen in der Desktop-App](/docs/de/desktop#work-in-parallel-with-sessions).

<h2 id="learn-more">
  Weitere Informationen
</h2>

Jeder Leitfaden unten behandelt Setup und Konfiguration für einen Ansatz:

* [Erstellen Sie benutzerdefinierte Subagenten](/docs/de/sub-agents): definieren Sie wiederverwendbare Spezialisten und kontrollieren Sie, welche Tools sie verwenden können.
* [Verwalten Sie Agenten mit Agent-Ansicht](/docs/de/agent-view): versenden Sie Sitzungen, beobachten Sie ihren Status und hängen Sie sich an, wenn eine Sitzung Sie benötigt.
* [Orchestrieren Sie Agent-Teams](/docs/de/agent-teams): richten Sie einen Lead und Teammates ein, weisen Sie Aufgaben zu und überprüfen Sie ihre Arbeit.
* [Orchestrieren Sie dynamische Workflows](/docs/de/workflows): führen Sie einen gebündelten Workflow aus oder lassen Sie Claude einen schreiben, der viele Subagenten ausführt und ihre Ergebnisse gegeneinander überprüft.
* [Führen Sie parallele Sitzungen mit Worktrees aus](/docs/de/worktrees): starten Sie Claude in einem isolierten Checkout, kontrollieren Sie, was kopiert wird, und bereinigen Sie danach.
