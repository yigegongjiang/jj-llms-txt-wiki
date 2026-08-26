> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code mit einem Bildschirmleser verwenden

> Richten Sie Claude Code für Bildschirmleser wie VoiceOver und NVDA ein, sowie Einstellungen für Bildschirmlupe, reduzierte Bewegung und farbenblindfreundliche Designs.

Claude Code verfügt über einen Bildschirmleser-Modus, der die visuelle Terminaloberfläche durch einfachen, linearen Text ersetzt. Anstelle von Kästchen, Fortschrittsanimationen und direkten Neuzeichnungen gibt der Modus beschriftete Zeilen aus, die ein Bildschirmleser wie VoiceOver oder NVDA der Reihe nach vorliest, sodass Sie ein vollständiges Gespräch führen, Werkzeugberechtigungen genehmigen und die Ausgabe von Anfang bis Ende überprüfen können.

Der Bildschirmleser-Modus ist optional. Wenn Sie stattdessen eine Bildschirmlupe, reduzierte Bewegung oder ein farbenblindfreundliches Design verwenden, siehe [Barrierefreiheitseinstellungen außerhalb des Bildschirmleser-Modus](#accessibility-settings-beyond-screen-reader-mode).

<Note>
  Der Bildschirmleser-Modus erfordert Claude Code v2.1.181 oder später. Frühere Versionen lehnen das Flag `--ax-screen-reader` mit `error: unknown option '--ax-screen-reader'` ab.
</Note>

<h2 id="turn-on-screen-reader-mode">
  Bildschirmleser-Modus aktivieren
</h2>

Wählen Sie die Methode, die Ihrer Häufigkeit der Bildschirmleser-Nutzung entspricht:

* Für eine Sitzung: Führen Sie `claude --ax-screen-reader` aus.
* Für Sitzungen, die von einer Shell aus gestartet werden: Setzen Sie die Umgebungsvariable `CLAUDE_AX_SCREEN_READER` auf `1`. In Bash oder Zsh führen Sie `export CLAUDE_AX_SCREEN_READER=1` aus; in PowerShell führen Sie `$env:CLAUDE_AX_SCREEN_READER = "1"` aus. Fügen Sie die Zeile zu Ihrem Shell-Profil hinzu, um alle Shells abzudecken.
* Für jede Sitzung auf dem Computer: Fügen Sie `"axScreenReader": true` zu Ihrer Benutzerdatei [Einstellungsdatei](/docs/de/settings) hinzu. Dies deckt jedes Terminal ab, einschließlich des integrierten VS Code-Terminals.

<Note>
  Die Methoden sind in Prioritätsreihenfolge aufgelistet: Das Flag [`--ax-screen-reader`](/docs/de/cli-reference#cli-flags) überschreibt die Umgebungsvariable [`CLAUDE_AX_SCREEN_READER`](/docs/de/env-vars), die die Einstellung [`axScreenReader`](/docs/de/settings#available-settings) überschreibt.
</Note>

Wenn Sie Claude Code über SSH verwenden, setzen Sie die Umgebungsvariable oder Einstellung auf dem Remote-Computer, auf dem Claude Code ausgeführt wird.

Wenn der Modus aktiviert ist, gibt Claude Code zunächst eine Bestätigungszeile aus, die die Methode benennt, die ihn aktiviert hat: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]` oder `[Screen Reader Mode: on via settings]`. Das Methodenbenennungsformat erfordert Claude Code v2.1.206 oder später. Wenn Claude Code sich selbst neu startet, beispielsweise um die Installation eines Updates abzuschließen, erbt der neue Prozess den Modus durch die Umgebungsvariable `CLAUDE_AX_SCREEN_READER`, sodass seine Bestätigungszeile `[Screen Reader Mode: on via env]` lautet, unabhängig davon, welche Methode Sie verwendet haben.
Frühere Versionen geben `[Accessible screen reader mode: on]` aus.

<h2 id="turn-off-screen-reader-mode">
  Bildschirmleser-Modus deaktivieren
</h2>

Kehren Sie die Methode um, die den Modus aktiviert hat: Starten Sie ohne das Flag, heben Sie die Umgebungsvariable auf, oder setzen Sie `axScreenReader` auf `false`. Das Setzen von `CLAUDE_AX_SCREEN_READER=0` hält den Modus aus, auch wenn die Einstellung `true` ist.

<h2 id="what-your-screen-reader-hears">
  Was Ihr Bildschirmleser hört
</h2>

Im Bildschirmleser-Modus schreibt Claude Code flachen Text:

* keine Zeichnungszeichen für die Oberflächenelemente
* keine nur farbbasierten Hinweise
* keine Neuzeichnungen von Inhalten, die sich nicht geändert haben; Fortschrittsanzeigen werden als statischer Text dargestellt
* Tabellen in Claudes Antworten werden als `Header: value`-Sätze statt als Zeichnungsgitter gelesen. Erfordert Claude Code v2.1.198 oder später; frühere Versionen zeichnen Tabellen auch im Bildschirmleser-Modus als Gitter.

Die Ausgabe sammelt sich im Scrollback Ihres Terminals an, sodass Sie frühere Züge mit den Überprüfungsbefehlen Ihres Bildschirmlesers oder der Suchfunktion Ihres Terminals erneut lesen können.

Der Bildschirmleser-Modus wird als einfacher scrollender Text dargestellt, auch wenn Sie das [Vollbildrendering](/docs/de/fullscreen) mit der Einstellung [`tui`](/docs/de/settings#available-settings) aktiviert haben; die Einstellung hat keine Auswirkung, während der Modus aktiv ist. Angehängte Hintergrundsitzungen werden weiterhin im Vollbildmodus dargestellt; siehe [Bekannte Einschränkungen](#known-limitations).

Jede Nachricht im Transkript beginnt mit einer Beschriftung, die Ihr Bildschirmleser ankündigt und benennt, was es ist: Ihre Nachrichten, Claudes Antworten, Werkzeugaktivität, Fehler und Eingabeaufforderungen. Die Beschriftungen sind auch durchsuchbar, sodass Sie zwischen Abschnitten des Transkripts springen können, indem Sie den Scrollback Ihres Terminals durchsuchen:

| Beschriftung           | Bedeutung                                                                                                    |
| :--------------------- | :----------------------------------------------------------------------------------------------------------- |
| `you:`                 | Ihre Nachrichten                                                                                             |
| `claude:`              | Claudes Antworten                                                                                            |
| `tool:`                | Werkzeugaktivität, z. B. eine Dateibearbeitung oder ein ausgeführter Befehl                                  |
| `tool error:`          | Ein Werkzeug, das fehlgeschlagen ist                                                                         |
| `error:`               | Ein Fehler in der Konversation, z. B. eine fehlgeschlagene API-Anfrage                                       |
| `Permission Required:` | Eine Berechtigungsaufforderung, die auf Ihre Antwort wartet                                                  |
| `Cost:`                | Die Sitzungskostenzusammenfassung, wenn Claude Code beendet wird, wenn Ihr Konto [Kosten anzeigt](/docs/de/costs) |

Der Terminal-Cursor folgt der Eingabemarke, sodass der Befehl zum Lesen der aktuellen Zeile eines Bildschirmlesers die Frage „Wo bin ich?" mit der Eingabeaufforderung beantwortet, die Sie bearbeiten.

<h3 id="jump-between-turns">
  Zwischen Zügen springen
</h3>

Claude Code gibt OSC 133-Shell-Integrations-Marker an Zuggrenzen aus, sodass die Taste zum Springen zur vorherigen Eingabeaufforderung Ihres Terminals zwischen Zügen wechselt, ohne das gesamte Transkript zu lesen:

* iTerm2: Cmd+Shift+Up
* VS Code-Terminal: Ctrl+Up unter Windows, Cmd+Up auf macOS
* Windows Terminal: Standardmäßig keine Taste; binden Sie die Aktion `scrollToMark` in seinen Einstellungen
* Kitty und Ghostty: Überprüfen Sie die Dokumentation des Terminals auf seine Taste zum Springen zur Eingabeaufforderung

macOS Terminal reagiert nicht auf die Marker, und Claude Code gibt sie in WezTerm nicht aus. Durchsuchen Sie in diesen Terminals stattdessen den Scrollback nach der Beschriftung `you:`.

<h2 id="answer-menus-and-prompts">
  Menüs und Eingabeaufforderungen beantworten
</h2>

Im Bildschirmleser-Modus werden Menüs, die Sie normalerweise mit den Pfeiltasten navigieren würden, einschließlich Berechtigungsaufforderungen, zu nummerierten Listen. Jede Option wird als nummerierte Zeile angekündigt, gefolgt von einer Eingabeaufforderung `Enter selection`, die den gültigen Bereich benennt. Geben Sie die Nummer der gewünschten Option ein und drücken Sie die Eingabetaste.

* Um ein verwerfbares Menü abzubrechen: Drücken Sie Escape. Seine Eingabeaufforderung endet mit `or Escape to cancel`.
* Wenn Sie eine Nummer eingeben, die nicht auf der Liste steht: Claude Code kündigt den gültigen Bereich an und lässt Sie es erneut versuchen.

Ja-oder-Nein-Aufforderungen fragen nach einer eingegebenen Antwort statt eines Zwei-Optionen-Menüs. Antworten Sie mit `y` oder `n` und drücken Sie die Eingabetaste. `yes` und `no` funktionieren auch.

<h2 id="hear-when-claude-code-needs-you">
  Hören Sie, wenn Claude Code Sie braucht
</h2>

Im Bildschirmleser-Modus läutet Claude Code die Terminal-Glocke, wenn es Ihre Aufmerksamkeit braucht, sodass Sie nicht ständig das Transkript überprüfen müssen. Die Glocke läutet, wenn:

* Claude eine Antwort beendet
* eine Berechtigungsaufforderung erscheint
* ein Werkzeug, das länger als 5 Sekunden lief, beendet wird

Die Glocke ist die Standard-Warnung Ihres Terminals. Um sie stummzuschalten, ändern Sie die Glockeneinstellung in Ihrer Terminalanwendung. Die Glocke erfordert keinen Bildschirmleser-Modus: Außerhalb des Modus setzen Sie [`preferredNotifChannel`](/docs/de/settings#available-settings) auf `"terminal_bell"` für ähnliche Warnungen, wenn Claude auf Sie wartet. Siehe [Terminal-Glocke oder Benachrichtigung erhalten](/docs/de/terminal-config#get-a-terminal-bell-or-notification).

<h2 id="accessibility-settings-beyond-screen-reader-mode">
  Barrierefreiheitseinstellungen außerhalb des Bildschirmleser-Modus
</h2>

Diese Optionen behandeln Barrierefreiheitsanforderungen außerhalb des Bildschirmleser-Modus. Alle funktionieren zusammen mit ihm.

* Die Umgebungsvariable `CLAUDE_CODE_ACCESSIBILITY` [environment variable](/docs/de/env-vars) ist für Bildschirmlupe. Setzen Sie `CLAUDE_CODE_ACCESSIBILITY=1`, um den nativen Terminal-Cursor sichtbar zu halten, damit Lupen wie macOS Zoom die Cursor-Position verfolgen können.
* Die Einstellung `prefersReducedMotion` [setting](/docs/de/settings#available-settings) reduziert oder deaktiviert Spinner, Shimmer und andere Animationen, ohne den Rest der Oberfläche zu ändern.
* Die Einstellung `theme` [setting](/docs/de/settings#available-settings) wählt die Oberflächenfarben aus, einschließlich der farbenblindfreundlichen Designs `dark-daltonized` und `light-daltonized`.

<h2 id="known-limitations">
  Bekannte Einschränkungen
</h2>

Einige Verhaltensweisen sind nicht für den Bildschirmleser-Modus angepasst:

* Der Bildschirmleser-Modus wird nicht automatisch aktiviert, wenn ein Bildschirmleser ausgeführt wird.
* Modusänderungen, z. B. das Eingeben des [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode), werden noch nicht angekündigt.
* Das Anhängen an eine [Hintergrundsitzung](/docs/de/agent-view) mit `claude attach` oder aus der Agent-Ansicht betritt den alternativen Bildschirm des Terminals, der keinen nativen Scrollback hat. Dies ist das [gleiche Verhalten wie bei anderen angehängten Sitzungen](/docs/de/fullscreen). Um herauszukommen, drücken Sie den linken Pfeil bei einer leeren Eingabeaufforderung oder Ctrl+Z, wenn ein Dialog den Fokus hat.
* Claude Code kündigt Kosten in der Zusammenfassung an, die es beim Beenden ausgibt, nicht pro Zug.
* Der Bildschirmleser-Modus ändert den [nicht-interaktiven Modus](/docs/de/headless) mit dem Flag `-p` nicht. Der nicht-interaktive Modus schreibt bereits einfachen Text und bleibt eine Alternative zum Scripting.

<h2 id="report-an-issue">
  Problem melden
</h2>

Wenn etwas mit Ihrem Bildschirmleser, Ihrer Lupe oder Ihrem Terminal nicht funktioniert, öffnen Sie ein Problem im [Claude Code Issue Tracker](https://github.com/anthropics/claude-code/issues) und erwähnen Sie Ihre Hilfstechnologie im Titel. Fügen Sie Ihr Betriebssystem, Ihre Terminalanwendung sowie den Namen und die Version Ihrer Hilfstechnologie in den Bericht ein.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Diese Seiten enthalten die vollständigen Referenzeinträge und verwandte Einrichtung für das, was diese Seite behandelt:

* [Einstellungen](/docs/de/settings#available-settings): die Einträge `axScreenReader`, `prefersReducedMotion`, `theme` und `preferredNotifChannel`
* [Umgebungsvariablen](/docs/de/env-vars): die Einträge `CLAUDE_AX_SCREEN_READER` und `CLAUDE_CODE_ACCESSIBILITY`
* [CLI-Referenz](/docs/de/cli-reference#cli-flags): das Flag `--ax-screen-reader`
* [Terminal-Konfiguration](/docs/de/terminal-config): Glocken, Benachrichtigungen und Designs außerhalb des Bildschirmleser-Modus
* [Nicht-interaktiver Modus](/docs/de/headless): Skriptgesteuerte `claude -p`-Läufe, die einfachen Text ohne Bildschirmleser-Modus schreiben
