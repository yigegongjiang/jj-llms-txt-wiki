> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Wiederkehrende Aufgaben in Claude Code Desktop planen

> Richten Sie geplante Aufgaben in Claude Code Desktop ein, um Claude automatisch in regelmäßigen Abständen für tägliche Code-Reviews, Abhängigkeitsprüfungen oder morgendliche Briefings auszuführen.

Geplante Aufgaben starten automatisch eine neue Sitzung zu einem von Ihnen gewählten Zeitpunkt und in einer von Ihnen gewählten Häufigkeit. Verwenden Sie sie für wiederkehrende Arbeiten wie tägliche Code-Reviews, Überprüfungen von Abhängigkeitsaktualisierungen oder morgendliche Briefings, die Informationen aus Ihrem Kalender und Ihrer Inbox abrufen.

Die Seite **Routinen** der Desktop-App ermöglicht es Ihnen, sowohl lokale geplante Aufgaben als auch Remote-[Routinen](/docs/de/routines) zu erstellen. Eine lokale Aufgabe wird auf Ihrem Computer mit direktem Zugriff auf Ihre Dateien und Tools ausgeführt, wird aber nur ausgelöst, wenn die App offen ist und Ihr Computer aktiv ist. Eine Remote-Routine wird auf der von Anthropic verwalteten Cloud-Infrastruktur ausgeführt, auch wenn Ihr Computer ausgeschaltet ist, und kann auch durch API-Aufrufe oder GitHub-Ereignisse ausgelöst werden. Diese Seite behandelt lokale geplante Aufgaben. Informationen zu Remote-Routinen und deren Trigger-Optionen finden Sie unter [Routinen](/docs/de/routines).

<h2 id="compare-scheduling-options">
  Planungsoptionen vergleichen
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<Note>
  Standardmäßig werden geplante Aufgaben für den aktuellen Zustand Ihres Arbeitsverzeichnisses ausgeführt, einschließlich nicht committeter Änderungen. Aktivieren Sie den Worktree-Schalter beim Erstellen der Aufgabe, um jeder Ausführung ihren eigenen isolierten Git-Worktree zu geben, auf die gleiche Weise wie [parallele Sitzungen](/docs/de/desktop#work-in-parallel-with-sessions) funktionieren.
</Note>

<h2 id="create-a-scheduled-task">
  Erstellen Sie eine geplante Aufgabe
</h2>

Klicken Sie auf **Routinen** in der Seitenleiste und dann auf **Neue Routine** und wählen Sie **Lokal**. Konfigurieren Sie diese Felder:

| Feld         | Beschreibung                                                                                                                                                                                                                                                                                                                         |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Name         | Bezeichner für die Aufgabe. Wird in Kleinbuchstaben Kebab-Case konvertiert und als Ordnername auf der Festplatte verwendet. Muss eindeutig über alle Ihre Aufgaben hinweg sein.                                                                                                                                                      |
| Beschreibung | Kurze Zusammenfassung, die in der Aufgabenliste angezeigt wird.                                                                                                                                                                                                                                                                      |
| Anweisungen  | Was Claude tun soll, wenn die Aufgabe ausgeführt wird. Schreiben Sie dies auf die gleiche Weise wie jede Nachricht im Eingabefeld. Die Eingabe für Anweisungen enthält Picker für den Berechtigungsmodus und das Modell, und darunter wählen Sie den Arbeitsordner und ob die Ausführung in einem isolierten Worktree erfolgen soll. |
| Zeitplan     | Wie oft die Aufgabe ausgeführt wird. Siehe [Planungsoptionen](#schedule-options) unten.                                                                                                                                                                                                                                              |

Ein Ordner ist erforderlich, bevor Sie die Aufgabe speichern können. Wenn Sie diesen Ordner noch nicht vertraut haben, fordert Desktop Sie auf, ihn zu vertrauen, bevor Sie speichern.

Sie können auch eine Aufgabe erstellen, indem Sie in einer beliebigen Sitzung beschreiben, was Sie möchten. Zum Beispiel erstellt „Richten Sie ein tägliches Code-Review ein, das jeden Morgen um 9 Uhr ausgeführt wird" eine wiederkehrende Aufgabe, und „Erinnern Sie mich morgen um 15 Uhr, die Bereitstellung zu überprüfen" erstellt eine einmalige Aufgabe, die sich nach der Ausführung selbst deaktiviert.

<h2 id="schedule-options">
  Planungsoptionen
</h2>

Wählen Sie eine Voreinstellung aus dem Zeitplan-Steuerelement:

* **Manuell**: kein Zeitplan, wird nur ausgeführt, wenn Sie auf **Jetzt ausführen** klicken. Nützlich zum Speichern einer Eingabeaufforderung, die Sie bei Bedarf auslösen
* **Stündlich**: wird jede Stunde ausgeführt
* **Täglich**: zeigt eine Zeitauswahl an, Standard ist 9:00 Uhr Ortszeit
* **Wochentage**: wie täglich, aber überspringt Samstag und Sonntag
* **Wöchentlich**: zeigt eine Zeitauswahl und eine Tagesauswahl an

Für Intervalle, die die Auswahl nicht bietet, z. B. alle 15 Minuten, am ersten des Monats oder eine einzelne Ausführung zu einem bestimmten zukünftigen Zeitpunkt, bitten Sie Claude in einer beliebigen Desktop-Sitzung, den Zeitplan festzulegen. Verwenden Sie einfache Sprache; zum Beispiel „Planen Sie eine Aufgabe, um alle Tests alle 6 Stunden auszuführen."

<h2 id="how-scheduled-tasks-run">
  Wie geplante Aufgaben ausgeführt werden
</h2>

Geplante Aufgaben werden auf Ihrem Computer ausgeführt. Desktop überprüft den Zeitplan jede Minute, während die App offen ist, und startet eine neue Sitzung, wenn eine Aufgabe fällig ist, unabhängig von manuellen Sitzungen, die Sie offen haben. Jede Aufgabe erhält eine kleine Verzögerung von einigen Minuten nach der geplanten Zeit, um den API-Verkehr zu staffeln. Die Verzögerung ist deterministisch: die gleiche Aufgabe startet immer mit dem gleichen Offset.

Wenn eine Aufgabe ausgelöst wird, erhalten Sie eine Desktop-Benachrichtigung und eine neue Sitzung wird unter einem Abschnitt **Geplant** in der Seitenleiste angezeigt. Öffnen Sie sie, um zu sehen, was Claude getan hat, Änderungen zu überprüfen oder auf Berechtigungsaufforderungen zu reagieren. Die Sitzung funktioniert wie jede andere: Claude kann Dateien bearbeiten, Befehle ausführen, Commits erstellen und Pull Requests öffnen.

Aufgaben werden nur ausgeführt, während die Desktop-App ausgeführt wird und Ihr Computer aktiv ist. Wenn Ihr Computer durch eine geplante Zeit schläft, wird die Ausführung übersprungen. Um Ruhezustand zu verhindern, aktivieren Sie **Computer aktiv halten** in den Einstellungen unter **Desktop-App → Allgemein**. Das Schließen des Laptop-Deckels versetzt ihn dennoch in den Ruhezustand. Für Aufgaben, die auch ausgeführt werden müssen, wenn Ihr Computer ausgeschaltet ist, oder die durch einen API-Aufruf oder ein GitHub-Ereignis ausgelöst werden sollen, erstellen Sie stattdessen eine Remote-[Routine](/docs/de/routines).

<h2 id="missed-runs">
  Verpasste Ausführungen
</h2>

Wenn die App startet oder Ihr Computer aufwacht, überprüft Desktop, ob jede Aufgabe in den letzten sieben Tagen Ausführungen verpasst hat. Wenn ja, startet Desktop genau eine Nachholausführung für die zuletzt verpasste Zeit und verwirft alles Ältere. Eine tägliche Aufgabe, die sechs Tage verpasst hat, wird einmal beim Aufwachen ausgeführt. Desktop zeigt eine Benachrichtigung an, wenn eine Nachholausführung startet.

Beachten Sie dies beim Schreiben von Eingabeaufforderungen. Eine Aufgabe, die für 9 Uhr geplant ist, könnte um 23 Uhr ausgeführt werden, wenn Ihr Computer den ganzen Tag über im Ruhezustand war. Wenn das Timing wichtig ist, fügen Sie Schutzmaßnahmen zur Eingabeaufforderung selbst hinzu, zum Beispiel: „Überprüfen Sie nur die heutigen Commits. Wenn es nach 17 Uhr ist, überspringen Sie die Überprüfung und posten Sie einfach eine Zusammenfassung dessen, was verpasst wurde."

<h2 id="permissions-for-scheduled-tasks">
  Berechtigungen für geplante Aufgaben
</h2>

Jede Aufgabe hat ihren eigenen Berechtigungsmodus, den Sie beim Erstellen oder Bearbeiten der Aufgabe festlegen. Zulassungsregeln aus `~/.claude/settings.json` gelten auch für geplante Aufgabensitzungen. Wenn eine Aufgabe im Ask-Modus ausgeführt wird und ein Tool ausführen muss, für das sie keine Berechtigung hat, bleibt die Ausführung stehen, bis Sie sie genehmigen. Die Sitzung bleibt in der Seitenleiste offen, damit Sie später antworten können.

Um Stalls zu vermeiden, klicken Sie nach dem Erstellen einer Aufgabe auf **Jetzt ausführen**, achten Sie auf Berechtigungsaufforderungen und wählen Sie für jede „immer zulassen". Zukünftige Ausführungen dieser Aufgabe genehmigen automatisch die gleichen Tools ohne Aufforderung. Sie können diese Genehmigungen auf der Detailseite der Aufgabe überprüfen und widerrufen.

Connector-Tools [die Ihre Organisation auf `ask` eingestellt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden bei jedem Aufruf angefordert und bieten keine Option „immer zulassen". Ausführungen, die diese Tools aufrufen, bleiben jedes Mal stehen.

<h2 id="manage-scheduled-tasks">
  Verwalten Sie geplante Aufgaben
</h2>

Klicken Sie auf eine Aufgabe in der Liste **Routinen**, um ihre Detailseite zu öffnen. Von hier aus können Sie:

* **Jetzt ausführen**: Starten Sie die Aufgabe sofort, ohne auf die nächste geplante Zeit zu warten
* **Status**: Umschalten zwischen Aktiv und Pausiert, um geplante Ausführungen zu pausieren oder fortzusetzen, ohne die Aufgabe zu löschen
* **Bearbeiten**: Ändern Sie die Anweisungen, den Zeitplan, den Ordner oder andere Einstellungen
* **Verlauf überprüfen**: Sehen Sie jede vergangene Ausführung, einschließlich übersprungener Ausführungen. Bewegen Sie den Mauszeiger über einen übersprungenen Eintrag, um zu sehen, warum: Ihr Computer war im Ruhezustand, die vorherige Ausführung war noch in Bearbeitung, oder andere geplante Aufgaben wurden bereits ausgeführt. Klicken Sie auf **Mehr anzeigen**, um ältere Einträge zu laden.
* **Zulässige Berechtigungen überprüfen**: Sehen Sie und widerrufen Sie gespeicherte Tool-Genehmigungen für diese Aufgabe aus dem Bereich **Immer zulässig**
* **Löschen**: Entfernen Sie die Aufgabe und archivieren Sie alle Sitzungen, die sie erstellt hat. Ein Kontrollkästchen **Auch Dateien auf der Festplatte löschen** wird im Bestätigungsdialog angezeigt. Aktivieren Sie es, um auch die Datei `SKILL.md` der Aufgabe und zugehörige Daten aus `~/.claude/scheduled-tasks/` zu entfernen.

Sie können Aufgaben auch auflisten, erstellen, bearbeiten und pausieren, indem Sie Claude in einer beliebigen Desktop-Sitzung bitten. Zum Beispiel „Pausieren Sie meine Aufgabe zur Abhängigkeitsprüfung" oder „Zeigen Sie mir meine geplanten Aufgaben." Um eine Aufgabe zu löschen, verwenden Sie die Schaltfläche **Löschen** auf ihrer Detailseite.

Eine geplante Aufgabe kann auch ihren eigenen Zeitplan oder ihre Eingabeaufforderung innerhalb einer laufenden Sitzung mit dem MCP-Tool `update_scheduled_task` ändern. Dies ermöglicht es einer Aufgabe, sich selbst neu zu planen, basierend auf dem, was sie findet, zum Beispiel das Umplanen eines Code-Reviews, um früher ausgeführt zu werden, wenn es einen Release-Branch erkennt.

Um die Eingabeaufforderung einer Aufgabe auf der Festplatte zu bearbeiten, öffnen Sie `~/.claude/scheduled-tasks/<task-name>/SKILL.md` (oder unter [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars), falls gesetzt). Die Datei verwendet YAML-Frontmatter für `name` und `description`, mit der Eingabeaufforderung als Text. Änderungen werden bei der nächsten Ausführung wirksam. Zeitplan, Ordner, Modell und aktivierter Status befinden sich nicht in dieser Datei: Ändern Sie sie über das Bearbeitungsformular oder bitten Sie Claude.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Routinen](/docs/de/routines): Führen Sie Aufgaben auf der von Anthropic verwalteten Infrastruktur nach einem Zeitplan, über einen API-Aufruf oder als Reaktion auf GitHub-Ereignisse aus, auch wenn Ihr Computer ausgeschaltet ist
* [Eingabeaufforderungen nach einem Zeitplan ausführen](/docs/de/scheduled-tasks): Sitzungsbezogene Planung mit `/loop` in der CLI
* [Claude Code GitHub Actions](/docs/de/github-actions): Führen Sie Claude nach einem Zeitplan in CI statt auf Ihrem Computer aus
* [Verwenden Sie Claude Code Desktop](/docs/de/desktop): Das vollständige Desktop-App-Handbuch
