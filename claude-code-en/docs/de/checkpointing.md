> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Verfolgen, zurückspulen und fassen Sie Claudes Bearbeitungen und Konversation zusammen, um den Sitzungsstatus zu verwalten.

Claude Code verfolgt automatisch Claudes Dateibearbeitungen während Sie arbeiten, sodass Sie Änderungen schnell rückgängig machen und zu vorherigen Zuständen zurückspulen können, falls etwas schiefgeht.

<h2 id="how-checkpoints-work">
  Wie Checkpoints funktionieren
</h2>

Während Sie mit Claude arbeiten, erfasst Checkpointing automatisch den Zustand Ihres Codes vor jedem Prompt, den Sie senden und der einen Turn startet.

<h3 id="automatic-tracking">
  Automatische Verfolgung
</h3>

Claude Code verfolgt alle Änderungen, die von seinen Datei-Bearbeitungswerkzeugen vorgenommen werden:

* Jeder Prompt, den Sie senden und der einen Turn startet, erstellt einen neuen Checkpoint
* Claude Code speichert Datei-Snapshots für die 100 neuesten Checkpoints in einer Sitzung. Das Verwerfen eines älteren Checkpoints löscht die Snapshot-Dateien, auf die kein verbleibender Checkpoint verweist, außer dem ersten Snapshot jeder Datei, den die VS Code-Erweiterung als Baseline für ihre Sitzungs-Diffs verwendet.
* Claude Code speichert Checkpoints mit der Konversation, sodass Sie `/rewind` auch nach dem Fortsetzen einer Sitzung noch ausführen können
* Claude Code löscht die Datei-Snapshots einer Sitzung im [Aufbewahrungsdurchlauf](/docs/de/claude-directory#cleaned-up-automatically), standardmäßig etwa 30 Tage, nachdem die Sitzung zuletzt einen gespeichert hat. Das Zurückspulen zu einem Checkpoint, dessen Snapshots weg sind, kann mit [`No files were restored`](/docs/de/errors#no-files-were-restored) fehlschlagen. Um Snapshots länger zu behalten, setzen Sie [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays).

<h3 id="rewind-and-summarize">
  Rewind und Zusammenfassung
</h3>

Führen Sie `/rewind` aus, oder drücken Sie `Esc` zweimal, wenn das Prompt-Eingabefeld leer ist, um das Rewind-Menü zu öffnen.

<Note>
  Wenn das Prompt-Eingabefeld Text enthält, löscht doppeltes `Esc` diesen stattdessen, anstatt das Menü zu öffnen. Der gelöschte Text wird in Ihrem Eingabeverlauf gespeichert, sodass Sie `Up` drücken können, um ihn abzurufen, nachdem Sie im Rewind-Menü fertig sind.
</Note>

Das Rewind-Menü listet jeden Prompt auf, den Sie während der Sitzung gesendet haben, außer [Nachrichten, die sich einem laufenden Turn angeschlossen haben](#messages-sent-mid-turn-not-checkpointed). Wählen Sie den Punkt aus, auf den Sie einwirken möchten, und wählen Sie dann eine Aktion:

* **Code und Konversation wiederherstellen**: Revert sowohl Code als auch Konversation zu diesem Punkt
* **Konversation wiederherstellen**: Zurückspulen zu dieser Nachricht, während der aktuelle Code beibehalten wird
* **Code wiederherstellen**: Dateiänderungen rückgängig machen, während die Konversation beibehalten wird
* **Von hier aus zusammenfassen**: Komprimieren Sie die Konversation von diesem Punkt an in eine Zusammenfassung und geben Sie Kontextfensterplatz frei
* **Bis hier zusammenfassen**: Komprimieren Sie die Konversation vor diesem Punkt in eine Zusammenfassung und behalten Sie spätere Nachrichten bei
* **Abbrechen**: Kehren Sie zur Nachrichtenliste zurück, ohne Änderungen vorzunehmen

Die beiden Code-Wiederherstellungsoptionen werden nur angezeigt, wenn der ausgewählte Checkpoint nachverfolgbare Dateiänderungen zum Rückgängigmachen hat. Wenn nach diesem Punkt keine Dateibearbeitungen erfasst wurden, bietet das Menü nur **Konversation wiederherstellen**, die Zusammenfassungsoptionen und **Abbrechen**.

Nach dem Wiederherstellen der Konversation oder nach Auswahl von „Von hier aus zusammenfassen" wird der ursprüngliche Prompt aus der ausgewählten Nachricht in das Eingabefeld wiederhergestellt, sodass Sie ihn erneut senden oder bearbeiten können.

Die Auswahl von „Bis hier zusammenfassen" lässt Sie am Ende der Konversation mit leerem Eingabefeld zurück. Bei beiden Zusammenfassungsoptionen wird ein **Zusammengefasste Konversation**-Marker in der Konversation angezeigt, wo die komprimierten Nachrichten waren.

<h4 id="rewind-past-a-cleared-conversation">
  Rewind über eine gelöschte Konversation hinweg
</h4>

Wenn Sie `/clear` früher im selben Claude Code-Prozess ausgeführt haben, zeigt das Rewind-Menü einen zusätzlichen Eintrag oben in der Liste mit der Bezeichnung `/resume <session-id> (previous session)`. Wählen Sie ihn aus, um die Konversation fortzusetzen, die vor dem Ausführen von `/clear` aktiv war. Der Eintrag ist verfügbar, bis Sie Claude Code beenden oder eine andere Sitzung fortsetzen.

<h4 id="guide-a-summary">
  Eine Zusammenfassung leiten
</h4>

Das Zusammenfassen ändert keine Dateien auf der Festplatte, und die ursprünglichen Nachrichten bleiben im Sitzungstranskript, sodass Claude die Details immer noch referenzieren kann. Um zu lenken, worauf sich die Zusammenfassung konzentriert, markieren Sie eine **Zusammenfassen**-Option mit den Pfeiltasten und geben Sie Anweisungen ein, wo die Zeile **add context (optional)** liest, und drücken Sie dann `Enter`. Die Auswahl der Option mit ihrer Zahlentaste fasst sofort ohne Anweisungen zusammen.

<Note>
  Zusammenfassen hält Sie in derselben Sitzung und komprimiert den Kontext, ähnlich wie ein gezieltes `/compact`. Um abzuzweigen und einen anderen Ansatz zu versuchen, während die ursprüngliche Sitzung intakt bleibt, verwenden Sie stattdessen [`/branch`](/docs/de/sessions#branch-a-session) oder `claude --continue --fork-session`.
</Note>

<h2 id="common-use-cases">
  Häufige Anwendungsfälle
</h2>

Checkpoints sind besonders nützlich, wenn:

* **Alternativen erkunden**: Versuchen Sie verschiedene Implementierungsansätze, ohne Ihren Ausgangspunkt zu verlieren
* **Fehler beheben**: Machen Sie schnell Änderungen rückgängig, die Fehler eingeführt oder Funktionalität unterbrochen haben
* **Funktionen iterieren**: Experimentieren Sie mit Variationen, da Sie zu funktionierenden Zuständen zurückkehren können
* **Kontextplatz freigeben**: Fassen Sie eine ausführliche Debugging-Sitzung von der Mitte an zusammen, während Sie Ihre ursprünglichen Anweisungen intakt halten

<h2 id="limitations">
  Einschränkungen
</h2>

<h3 id="bash-command-changes-not-tracked">
  Bash-Befehlsänderungen werden nicht verfolgt
</h3>

Checkpointing verfolgt keine Dateien, die durch Bash-Befehle geändert werden. Wenn Claude Code beispielsweise ausführt:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Diese Dateiänderungen können nicht durch Zurückspulen rückgängig gemacht werden. Nur direkte Dateibearbeitungen, die durch Claudes Datei-Bearbeitungswerkzeuge vorgenommen werden, werden verfolgt.

<h3 id="subagent-edits-not-restored">
  Subagent-Bearbeitungen werden nicht wiederhergestellt
</h3>

Ein [Subagent](/docs/de/sub-agents) führt Bearbeitungen mit Claudes Datei-Bearbeitungswerkzeugen durch, aber Claude Code erfasst diese Bearbeitungen normalerweise nicht in den Checkpoints Ihrer Sitzung. Ob das Zurückspulen sie wiederherstellt, hängt davon ab, wie der Subagent ausgeführt wird:

* **Foreground-Forked-Skill**: ein [Skill mit `context: fork`](/docs/de/skills#run-skills-in-a-subagent), der im Vordergrund ausgeführt wird, bearbeitet Ihren Arbeitsbaum während Ihres eigenen Zuges, sodass das Zurückspulen seine Bearbeitungen wie gewohnt wiederherstellt. Setzen Sie `background: false`, um einen Fork im Vordergrund auszuführen; einige Situationen, [aufgelistet auf der Skills-Seite](/docs/de/skills#run-skills-in-a-subagent), führen ihn dort unabhängig von der Einstellung aus.
* **Jeder andere Subagent**: Das Zurückspulen stellt die Bearbeitungen nicht wieder her. Verwenden Sie Git, um sie rückgängig zu machen. Dies umfasst einen Forked-Skill, der im Hintergrund ausgeführt wird (Standard), und einen Hintergrund-[`/code-review --fix`](/docs/de/code-review)-Lauf.

<h3 id="external-changes-not-tracked">
  Externe Änderungen werden nicht verfolgt
</h3>

Checkpointing verfolgt nur Dateien, die in der aktuellen Sitzung bearbeitet wurden. Manuelle Änderungen, die Sie an Dateien außerhalb von Claude Code vornehmen, und Bearbeitungen aus anderen gleichzeitigen Sitzungen werden normalerweise nicht erfasst, es sei denn, sie ändern zufällig dieselben Dateien wie die aktuelle Sitzung.

<h3 id="messages-sent-mid-turn-not-checkpointed">
  Nachrichten, die während eines Zuges gesendet werden, werden nicht als Checkpoint erstellt
</h3>

Wenn eine Nachricht, die Sie [in die Warteschlange einreihen, während Claude arbeitet](/docs/de/interactive-mode#queue-messages-while-claude-works), Claude innerhalb des laufenden Zuges erreicht, wird sie in diesen Zug integriert, anstatt einen neuen zu starten. Die Nachricht wird in der Unterhaltung angezeigt, aber Claude Code erstellt keinen Checkpoint dafür, und das Zurückspulen-Menü listet sie nicht auf. Eine in die Warteschlange eingereihte Nachricht, die Claude Code als eigenen Zug sendet, erhält wie gewohnt einen Checkpoint.

Um eine solche Nachricht zu entfernen oder die Bearbeitungen rückgängig zu machen, die Claude nach ihr vorgenommen hat, spulen Sie zu dem Prompt zurück, der den Zug gestartet hat. Dies spult den gesamten Zug zurück, einschließlich der Arbeit, die Claude vor dem Eintreffen Ihrer Nachricht geleistet hat.

<h3 id="symlinked-and-hard-linked-paths-not-restored">
  Symverlinkte und hart verlinkte Pfade werden nicht wiederhergestellt
</h3>

Checkpointing stellt symverlinkte oder hart verlinkte Dateien nicht wieder her. Wenn Sie **Code wiederherstellen** oder **Code und Unterhaltung wiederherstellen** aus dem `/rewind`-Menü auswählen, überspringt Claude Code jeden verfolgten Pfad, der ein Symlink oder Hard Link ist, und zeigt eine Warnung `Restored the code, but skipped N files` an. Die übersprungenen Dateien behalten ihren aktuellen Inhalt. Um die Änderungen der Sitzung an einer von ihnen rückgängig zu machen, bitten Sie Claude, die Bearbeitung rückgängig zu machen, oder bearbeiten Sie die Datei selbst. Konfigurationsdateien, die ein Dotfile-Manager in Ihr Projekt symlinkt, und Dateien, die pnpm hart verlinkt, fallen beide in diese Kategorie.

Um zu sehen, welche Pfade eine Wiederherstellung überspringt, aktivieren Sie Debug-Protokollierung mit `/debug`, bevor Sie wiederherstellen: das Debug-Protokoll unter `~/.claude/debug/<session-id>.txt` nennt jeden übersprungenen Pfad. Für jeden Grund zum Überspringen und die Wiederherstellungsschritte siehe [den Eintrag „skipped-files" in der Fehlerreferenz](/docs/de/errors#restored-the-code-but-skipped-files).

<h3 id="not-a-replacement-for-version-control">
  Kein Ersatz für Versionskontrolle
</h3>

Checkpoints sind für schnelle, sitzungsebene Wiederherstellung konzipiert. Für permanente Versionshistorie und Zusammenarbeit verwenden Sie weiterhin Versionskontrolle wie Git für Commits, Branches und langfristige Historie.

<h2 id="see-also">
  Siehe auch
</h2>

* [Interaktiver Modus](/docs/de/interactive-mode) - Tastaturkürzel und Sitzungssteuerungen
* [Befehle](/docs/de/commands) - Zugriff auf Checkpoints mit `/rewind`
* [CLI-Referenz](/docs/de/cli-reference) - Befehlszeilenoptionen
