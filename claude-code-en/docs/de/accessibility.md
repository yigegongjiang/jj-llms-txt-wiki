> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code mit einem Bildschirmleser verwenden

> Richten Sie Claude Code für Bildschirmleser wie VoiceOver und NVDA ein, sowie Einstellungen für Bildschirmlupe, reduzierte Bewegung und farbenblindfreundliche Designs.

Claude Code verfügt über einen Bildschirmleser-Modus, der die visuelle Terminaloberfläche durch einfachen, linearen Text ersetzt. Anstelle von Kästchen, Fortschrittsanimationen und direkten Neuzeichnungen gibt Claude Code beschriftete Zeilen aus, die ein Bildschirmleser wie VoiceOver oder NVDA der Reihe nach vorliest. Sie können ein vollständiges Gespräch führen, Werkzeugberechtigungen genehmigen und die Ausgabe von Anfang bis Ende überprüfen.

Der Bildschirmleser-Modus ist optional. Wenn Sie stattdessen eine Bildschirmlupe, reduzierte Bewegung oder ein farbenblindfreundliches Design verwenden, legen Sie `CLAUDE_CODE_ACCESSIBILITY`, `prefersReducedMotion` oder `theme` aus der Tabelle [Barrierefreiheitseinstellungen](#accessibility-settings) fest. Der Bildschirmleser-Modus passt nur die Terminaloberfläche an, daher benötigen Sie ihn nicht im Chat-Panel der VS Code-Erweiterung. In Claude Code v2.1.236 oder später [kündigt die Erweiterung Konversationsaktivität für Ihren Bildschirmleser an](/docs/de/vs-code#use-a-screen-reader), ohne dass eine Einstellung erforderlich ist.

<h2 id="turn-on-screen-reader-mode">
  Bildschirmleser-Modus aktivieren
</h2>

Wählen Sie die Methode, die Ihrer Häufigkeit der Bildschirmleser-Nutzung entspricht:

* Für eine Sitzung: Führen Sie `claude --ax-screen-reader` aus.
* Für Sitzungen, die von einer Shell aus gestartet werden: Setzen Sie die Umgebungsvariable `CLAUDE_AX_SCREEN_READER` auf `1`. In Bash oder Zsh führen Sie `export CLAUDE_AX_SCREEN_READER=1` aus. In PowerShell führen Sie `$env:CLAUDE_AX_SCREEN_READER = "1"` aus. Fügen Sie diese Zeile zu Ihrem Shell-Profil hinzu, um sie für zukünftige Shells beizubehalten.
* Für jede Sitzung auf dem Computer: Fügen Sie `"axScreenReader": true` zu Ihrer Benutzerdatei [Einstellungsdatei](/docs/de/settings) hinzu. Die Einstellung gilt in jedem Terminal, einschließlich des integrierten VS Code-Terminals.

Wenn Sie Methoden kombinieren, wendet Claude Code das Flag [`--ax-screen-reader`](/docs/de/cli-reference#cli-flags) über die Umgebungsvariable [`CLAUDE_AX_SCREEN_READER`](/docs/de/env-vars#variables) an, und die Variable über die Einstellung [`axScreenReader`](/docs/de/settings-reference#axscreenreader).

Wenn Sie Claude Code über SSH verwenden, legen Sie die Umgebungsvariable oder Einstellung auf dem Remote-Computer fest, auf dem Claude Code ausgeführt wird.

Die erste Zeile, die Claude Code ausgibt, bestätigt den Modus: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]` oder `[Screen Reader Mode: on via settings]`.

<h2 id="turn-off-screen-reader-mode">
  Bildschirmleser-Modus deaktivieren
</h2>

Kehren Sie die Methode um, die den Modus aktiviert hat: Starten Sie ohne das Flag, heben Sie die Umgebungsvariable auf, oder setzen Sie `axScreenReader` auf `false`. Wenn Sie `CLAUDE_AX_SCREEN_READER` auf `0` setzen, behält Claude Code den Modus aus, auch wenn die Einstellung `true` ist.

<h2 id="accessibility-settings">
  Barrierefreiheitseinstellungen
</h2>

Die Tabelle listet jede Barrierefreiheitsoption auf, ob Sie sie als Flag, Umgebungsvariable oder Einstellung festlegen, und was sie ändert.

| Option                                                                  | Typ               | Was es ändert                                                                                                                                                                                                                                                   |
| :---------------------------------------------------------------------- | :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`--ax-screen-reader`](/docs/de/cli-reference#cli-flags)                     | Flag              | Bildschirmleser-Modus für eine Sitzung.                                                                                                                                                                                                                         |
| [`CLAUDE_AX_SCREEN_READER`](/docs/de/env-vars#variables)                     | Umgebungsvariable | Bildschirmleser-Modus für Sitzungen, die von der Shell aus gestartet werden, in der Sie sie festlegen.                                                                                                                                                          |
| [`axScreenReader`](/docs/de/settings-reference#axscreenreader)               | Einstellung       | Bildschirmleser-Modus für jede Sitzung, wenn `true`.                                                                                                                                                                                                            |
| [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/de/env-vars#variables)                  | Umgebungsvariable | Wie lange Claude Code nach der Bestätigungszeile wartet, bevor es die erste Eingabeaufforderung im Bildschirmleser-Modus zeichnet. Erfordert Claude Code v2.1.217 oder später.                                                                                  |
| [`CLAUDE_AX_PREPARK_MS`](/docs/de/env-vars#variables)                        | Umgebungsvariable | Wie lange Claude Code mit dem Cursor am Anfang der Zeile wartet, bevor es eine neue oder geänderte Zeile im Bildschirmleser-Modus schreibt. Erfordert Claude Code v2.1.233 oder später.                                                                         |
| [`CLAUDE_CODE_ACCESSIBILITY`](/docs/de/env-vars#variables)                   | Umgebungsvariable | Ein Terminal-Cursor, der für Bildschirmlupe wie macOS Zoom sichtbar bleibt, wenn Sie ihn auf `1` setzen. Der Cursor folgt der Eingabemarke und, in Claude Code v2.1.218 oder später, der hervorgehobenen Zeile in Menüs und Panels wie `/config` und `/plugin`. |
| [`prefersReducedMotion`](/docs/de/settings-reference#prefersreducedmotion)   | Einstellung       | Reduzierte oder keine Spinner, Shimmer und andere Animationen, wenn `true`.                                                                                                                                                                                     |
| [`theme`](/docs/de/settings-reference#theme)                                 | Einstellung       | Die Oberflächenfarben, einschließlich der farbenblindfreundlichen Designs `dark-daltonized` und `light-daltonized`. Sie können auch eines mit [`/theme`](/docs/de/commands#all-commands) auswählen.                                                                  |
| [`preferredNotifChannel`](/docs/de/settings-reference#preferrednotifchannel) | Einstellung       | Mit dem Wert `"terminal_bell"`, eine Terminal-Glocke außerhalb des Bildschirmleser-Modus, wenn Claude auf Sie wartet.                                                                                                                                           |

<h2 id="what-your-screen-reader-hears">
  Was Ihr Bildschirmleser hört
</h2>

Im Bildschirmleser-Modus schreibt Claude Code flachen Text:

* Keine Zeichnungszeichen für die Oberflächenelemente
* Keine nur farbbasierten Hinweise
* Keine Neuzeichnungen von Inhalten, die sich nicht geändert haben. Fortschrittsanzeigen werden als statischer Text dargestellt
* Tabellen in Claudes Antworten werden als `Header: value`-Sätze statt als Zeichnungsgitter gelesen

Claude Code lässt alles, was es ausgibt, in Ihrem Terminal-Scrollback, sodass Sie frühere Züge mit den Überprüfungsbefehlen Ihres Bildschirmlesers oder der Suchfunktion Ihres Terminals erneut lesen können. Claude Code ignoriert die Einstellung [`tui`](/docs/de/settings-reference#tui) im Bildschirmleser-Modus. Abgesehen von den angehängten Hintergrundsitzungen, die unter [Bekannte Einschränkungen](#known-limitations) aufgelistet sind, gibt es scrollenden Text statt [Vollbildrendering](/docs/de/fullscreen) aus.

Claude Code wartet auch an zwei Stellen, damit Ihr Bildschirmleser mithalten kann:

* Nachdem Claude Code die Bestätigungszeile ausgibt, wartet es 3 Sekunden, bevor es die Eingabeaufforderung zeichnet, damit Ihr Bildschirmleser die Zeile beenden kann. Drücken Sie eine beliebige Taste, um das Warten zu beenden. Um die Länge des Wartens zu ändern, legen Sie [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/de/env-vars#variables) fest.
* Bevor Claude Code eine neue oder geänderte Zeile schreibt, z. B. einen Hinweis oder mehr von Claudes Antwort, bewegt es den Cursor an den Anfang der Zeile und wartet 50 Millisekunden. Ihr Bildschirmleser liest dann die Zeile von ihrem ersten Zeichen. Zeichen, die Sie am Ende der Eingabezeile eingeben oder löschen, werden sofort angezeigt. Um die Länge des Wartens zu ändern, legen Sie [`CLAUDE_AX_PREPARK_MS`](/docs/de/env-vars#variables) fest.

Jede Nachricht im Transkript beginnt mit einer Beschriftung, die Ihr Bildschirmleser ankündigt und benennt, was es ist: Ihre Nachrichten, Claudes Antworten und Denken, Werkzeugaktivität, Fehler und Warnungen sowie Eingabeaufforderungen. Die Beschriftungen sind auch durchsuchbar, sodass Sie zwischen Abschnitten des Transkripts springen können, indem Sie Ihren Terminal-Scrollback durchsuchen:

| Beschriftung           | Bedeutung                                                                                                    |
| :--------------------- | :----------------------------------------------------------------------------------------------------------- |
| `you:`                 | Ihre Nachrichten                                                                                             |
| `claude:`              | Claudes Antworten                                                                                            |
| `thinking:`            | Claudes Denken                                                                                               |
| `tool:`                | Werkzeugaktivität, z. B. eine Dateibearbeitung oder ein ausgeführter Befehl                                  |
| `tool error:`          | Ein Werkzeug, das fehlgeschlagen ist                                                                         |
| `error:`               | Ein Fehler in der Konversation, z. B. eine fehlgeschlagene API-Anfrage                                       |
| `warning:`             | Eine Warnung von Claude Code, z. B. ein Wechsel zu einem Fallback-Modell                                     |
| `Permission Required:` | Eine Berechtigungsaufforderung, die auf Ihre Antwort wartet                                                  |
| `Cost:`                | Die Sitzungskostenzusammenfassung, wenn Claude Code beendet wird, wenn Ihr Konto [Kosten anzeigt](/docs/de/costs) |

Claude Code behält den Terminal-Cursor auf der Eingabemarke, sodass der Befehl zum Lesen der aktuellen Zeile Ihres Bildschirmlesers die Eingabeaufforderung liest, die Sie bearbeiten.

Während Sie am Ende der Eingabezeile eingeben, oder dort `Backspace` drücken, schreibt Claude Code nur die Zeichen, die sich ändern. Ihr Bildschirmleser gibt nur diese Zeichen aus.

Wenn Sie ein Wort oder eine Zeile mit einem der [Textbearbeitungs-Shortcuts](/docs/de/interactive-mode#text-editing) löschen, kündigt Claude Code den gelöschten Text an:

* Löschen von Wörtern mit `Ctrl+W` oder `Alt+D`, oder mit `Option+Delete` auf macOS oder `Ctrl+Backspace` unter Windows
* Löschen zum Anfang der Zeile mit `Ctrl+U` oder `Cmd+Backspace`
* Löschen zum Ende der Zeile mit `Ctrl+K`

Wenn Sie [Berechtigungsmodi](/docs/de/permission-modes) mit `Shift+Tab` durchlaufen, kündigt Claude Code den Berechtigungsmodus an, auf dem Sie landen, z. B. `[plan mode on]` oder `[accept edits on]`. Claude Code gibt die Ankündigung einmal aus und wiederholt sie nicht bei späteren Neuzeichnungen.

<h3 id="jump-between-turns">
  Zwischen Zügen springen
</h3>

Claude Code gibt OSC 133 Shell-Integrations-Marker an Zuggrenzen aus, sodass die Taste zum Springen zur vorherigen Eingabeaufforderung Ihres Terminals zwischen Zügen wechselt, ohne das gesamte Transkript zu lesen:

* iTerm2: Cmd+Shift+Up
* VS Code-Terminal: Ctrl+Up unter Windows, Cmd+Up auf macOS
* Windows Terminal: Standardmäßig keine Taste; binden Sie die Aktion `scrollToMark` in seinen Einstellungen
* Kitty und Ghostty: Überprüfen Sie die Dokumentation des Terminals auf seine Taste zum Springen zur Eingabeaufforderung

macOS Terminal reagiert nicht auf die Marker, und Claude Code gibt sie in WezTerm nicht aus. Durchsuchen Sie in diesen Terminals stattdessen den Scrollback nach der Beschriftung `you:`.

<h2 id="answer-menus-and-prompts">
  Menüs und Eingabeaufforderungen beantworten
</h2>

Im Bildschirmleser-Modus werden Menüs, die Sie normalerweise mit den Pfeiltasten navigieren würden, einschließlich Berechtigungsaufforderungen, zu nummerierten Listen. Claude Code kündigt jede Option als nummerierte Zeile an, gefolgt von einer Eingabeaufforderung `Enter selection`, die den gültigen Bereich benennt. Geben Sie die Nummer der gewünschten Option ein und drücken Sie die Eingabetaste.

* Drücken Sie Escape, um ein Menü abzubrechen, dessen Eingabeaufforderung mit `or Escape to cancel` endet.
* Wenn Sie eine Nummer eingeben, die nicht auf der Liste steht, kündigt Claude Code den gültigen Bereich an und lässt Sie es erneut versuchen.

Der [`/effort`](/docs/de/model-config#adjust-effort-level) Selektor, der außerhalb des Bildschirmleser-Modus ein Schieberegler ist, wird zur gleichen Art von nummerierter Liste.

Ja-oder-Nein-Aufforderungen fragen nach einer eingegebenen Antwort statt eines Zwei-Optionen-Menüs. Antworten Sie mit `y` oder `n` und drücken Sie die Eingabetaste. `yes` und `no` funktionieren auch.

<h2 id="hear-when-claude-code-needs-you">
  Hören Sie, wenn Claude Code Sie braucht
</h2>

Im Bildschirmleser-Modus läutet Claude Code die Terminal-Glocke, wenn es Ihre Aufmerksamkeit braucht, sodass Sie nicht ständig das Transkript überprüfen müssen. Die Glocke läutet, wenn:

* Claude eine Antwort beendet
* Eine Eingabeaufforderung oder ein Dialog Ihre Antwort benötigt, z. B. eine Berechtigungsaufforderung
* Ein Werkzeug, das länger als 5 Sekunden lief, beendet wird

Die Glocke ist die Standard-Warnung Ihres Terminals. Um sie stummzuschalten, ändern Sie die Glockeneinstellung in Ihrer Terminalanwendung. Außerhalb des Bildschirmleser-Modus legen Sie [`preferredNotifChannel`](/docs/de/settings-reference#preferrednotifchannel) auf `"terminal_bell"` fest, um eine [ähnliche Glocke](/docs/de/terminal-config#get-a-terminal-bell-or-notification) zu erhalten, wenn Claude auf Sie wartet.

<h2 id="known-limitations">
  Bekannte Einschränkungen
</h2>

Einige Verhaltensweisen sind nicht für den Bildschirmleser-Modus angepasst:

* Der Bildschirmleser-Modus wird nicht automatisch aktiviert, wenn ein Bildschirmleser ausgeführt wird.
* Claude Code kündigt eine Berechtigungsmodus-Änderung nicht an, die auf andere Weise als durch Durchlaufen mit `Shift+Tab` vorgenommen wird, z. B. das Eingeben des [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) aus einem Befehl.
* Das Anhängen an eine [Hintergrundsitzung](/docs/de/agent-view) mit `claude attach` oder aus der Agent-Ansicht betritt den alternativen Bildschirm des Terminals, der keinen nativen Scrollback hat. Dies ist das [gleiche Verhalten wie bei anderen angehängten Sitzungen](/docs/de/fullscreen). Um herauszukommen, drücken Sie den linken Pfeil bei einer leeren Eingabeaufforderung, oder Ctrl+Z, wenn ein Dialog den Fokus hat.
* Claude Code kündigt Kosten in der Zusammenfassung an, die es beim Beenden ausgibt, nicht pro Zug.
* Der Bildschirmleser-Modus ändert den [nicht-interaktiven Modus](/docs/de/headless) mit dem Flag `-p` nicht. Der nicht-interaktive Modus schreibt bereits einfachen Text und bleibt eine Alternative zum Scripting.

<h2 id="report-an-issue">
  Problem melden
</h2>

Wenn etwas mit Ihrem Bildschirmleser, Ihrer Lupe oder Ihrem Terminal nicht funktioniert, öffnen Sie ein Problem im [Claude Code Issue Tracker](https://github.com/anthropics/claude-code/issues) und erwähnen Sie Ihre Hilfstechnologie im Titel. Fügen Sie Ihr Betriebssystem, Ihre Terminalanwendung sowie den Namen und die Version Ihrer Hilfstechnologie in den Bericht ein.
