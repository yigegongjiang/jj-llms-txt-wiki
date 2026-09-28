> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Interaktiver Modus

> Vollständige Referenz für Tastaturkürzel, Eingabemodi und interaktive Funktionen in Claude Code-Sitzungen.

<h2 id="keyboard-shortcuts">
  Tastaturkürzel
</h2>

<Note>
  Tastaturkürzel können je nach Plattform und Terminal unterschiedlich sein. Im [Vollbildmodus](/docs/de/fullscreen) drücken Sie `?` im Transkript-Viewer, um die dort verfügbaren Kürzel anzuzeigen.

  **macOS-Benutzer**: Tastaturkürzel mit der Option/Alt-Taste (`Alt+B`, `Alt+F`, `Alt+D`, `Alt+Y`, `Alt+P`) erfordern die Konfiguration von Option als Meta in Ihrem Terminal. Siehe [Option-Taste-Kürzel auf macOS aktivieren](/docs/de/terminal-config#enable-option-key-shortcuts-on-macos) für die Einstellung in jedem Terminal.
</Note>

<h3 id="general-controls">
  Allgemeine Steuerelemente
</h3>

| Kürzel                                                                                                       | Beschreibung                                                                                                                                                                                                                                                                                                   | Kontext                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :----------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                                                                                     | Unterbrechen oder Eingabe löschen                                                                                                                                                                                                                                                                              | Unterbricht einen laufenden Vorgang. Wenn nichts läuft, löscht der erste Druck die Eingabeaufforderung und ein zweiter Druck beendet Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `Ctrl+X Ctrl+K`                                                                                              | Alle laufenden [Hintergrund-Subagenten](/docs/de/sub-agents#run-subagents-in-foreground-or-background) in dieser Sitzung stoppen und [automatische Artefakt-Antworten](/docs/de/artifacts#let-claude-reply-to-comments-on-its-own) für den Rest deaktivieren. Zweimal innerhalb von 3 Sekunden drücken, um zu bestätigen | Subagenten-Steuerung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `Ctrl+D`                                                                                                     | Claude Code-Sitzung beenden                                                                                                                                                                                                                                                                                    | Der erste Druck zeigt einen Bestätigungshinweis und ein zweiter Druck innerhalb von 800 ms beendet die Sitzung. Wenn die Eingabeaufforderung Text enthält, löscht `Ctrl+D` das Zeichen nach dem Cursor                                                                                                                                                                                                                                                                                                                                                                                                       |
| `Ctrl+G` oder `Ctrl+X Ctrl+E`                                                                                | Im Standard-Texteditor öffnen                                                                                                                                                                                                                                                                                  | Bearbeiten Sie Ihre Eingabeaufforderung oder benutzerdefinierte Antwort in Ihrem Standard-Texteditor. `Ctrl+X Ctrl+E` ist die readline-native Bindung. Aktivieren Sie **Letzte Antwort im externen Editor anzeigen** in `/config`, um Claudes vorherige Antwort als `#`-kommentierter Kontext über Ihrer Eingabeaufforderung einzufügen; Claude Code entfernt den Kommentarblock, wenn Sie speichern                                                                                                                                                                                                         |
| `Ctrl+L`                                                                                                     | Bildschirm neu zeichnen                                                                                                                                                                                                                                                                                        | Erzwingt ein vollständiges Terminal-Neuzeichnen, wobei Eingabe und Gesprächsverlauf erhalten bleiben. Verwenden Sie dies, um die Anzeige wiederherzustellen, wenn sie verzerrt oder teilweise leer wird. Siehe [Gespräch löschen](/docs/de/fullscreen#clear-the-conversation) für Vollbildmodus                                                                                                                                                                                                                                                                                                                   |
| `Ctrl+O`                                                                                                     | Transkript-Viewer umschalten                                                                                                                                                                                                                                                                                   | Zeigt detaillierte Werkzeugnutzung und Ausführung mit einem Zeitstempel und dem verwendeten Modell bei jeder Assistentnachricht. Erweitert auch Zeilen, die standardmäßig zusammengeklappt sind, wie MCP-Aufrufe, die als einzelne `Called slack 3 times`-Zeile angezeigt werden, und [Nachrichten aus Ihren anderen Sitzungen](/docs/de/cross-session-messaging#what-a-message-looks-like), die als einzeilige `Message from @<sender>`-Vorschau angezeigt werden                                                                                                                                                |
| `Ctrl+R`                                                                                                     | Befehlsverlauf rückwärts durchsuchen                                                                                                                                                                                                                                                                           | Durchsuchen Sie vorherige Befehle interaktiv                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `Ctrl+V` oder `Cmd+V` (iTerm2) oder `Alt+V` (Windows und WSL)                                                | Bild aus Zwischenablage einfügen                                                                                                                                                                                                                                                                               | Fügt einen `[Image #N]`-Chip am Cursor ein, damit Sie ihn positionell in Ihrer Eingabeaufforderung referenzieren können. Unter WSL sind sowohl `Ctrl+V` als auch `Alt+V` gebunden; verwenden Sie `Alt+V`, wenn Ihr Terminal `Ctrl+V` abfängt                                                                                                                                                                                                                                                                                                                                                                 |
| `Ctrl+B`                                                                                                     | Hintergrund-Aufgaben ausführen                                                                                                                                                                                                                                                                                 | Versetzt Bash-Befehle und Agenten in den Hintergrund. Tmux-Benutzer drücken zweimal                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `Ctrl+T`                                                                                                     | Claudes Aufgabenliste umschalten                                                                                                                                                                                                                                                                               | Zeigen oder verbergen Sie [Claudes To-Do-Liste](#task-list) im Statusbereich. Dies ist nicht die Hintergrund-Aufgabenansicht; verwenden Sie [`/tasks`](/docs/de/commands), um laufende Shells und Subagenten anzuzeigen                                                                                                                                                                                                                                                                                                                                                                                           |
| `Ctrl+S`                                                                                                     | Eingabeaufforderung speichern oder wiederherstellen                                                                                                                                                                                                                                                            | Mit Text in der Eingabe speichert es diesen und löscht die Eingabeaufforderung. Erneut auf einer leeren Eingabeaufforderung gedrückt, stellt es den gespeicherten Text, die Cursorposition, eingefügte Inhalte und den Eingabemodus wieder her, sodass ein gespeicherter `!` [Shell-Befehl](#shell-mode-with-prefix) im Shell-Modus zurückkommt                                                                                                                                                                                                                                                              |
| `Ctrl+Z`                                                                                                     | Claude Code unterbrechen                                                                                                                                                                                                                                                                                       | Nur Unix. Unterbricht den Prozess zu Ihrer Shell; führen Sie `fg` aus, um fortzufahren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `Pfeiltasten links/rechts`                                                                                   | Durch Dialog-Registerkarten navigieren                                                                                                                                                                                                                                                                         | Navigieren Sie zwischen Registerkarten in Berechtigungsdialogen und Menüs                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `Tab`                                                                                                        | Einen Autovervollständigungsvorschlag akzeptieren oder einen Kommentar zu einer Berechtigungsantwort hinzufügen                                                                                                                                                                                                | Während Autovervollständigungsvorschläge in der Eingabeaufforderung angezeigt werden, akzeptiert dies den ausgewählten Vorschlag. Bei den meisten Berechtigungsaufforderungen öffnet dies mit **Ja** oder **Nein** fokussiert ein Kommentarfeld für diese Option, und das erneute Drücken schließt das Feld. Siehe [Kommentar hinzufügen, wenn Sie eine Berechtigungsaufforderung beantworten](/docs/de/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                                                                            |
| `Pfeiltasten oben/unten` oder `Ctrl+P`/`Ctrl+N`                                                              | Cursor verschieben oder Befehlsverlauf navigieren                                                                                                                                                                                                                                                              | Wenn sich die Eingabe über mehr als eine visuelle Zeile erstreckt, ob umgebrochen oder mehrzeilig, verschiebt dies zunächst den Cursor in der Eingabeaufforderung. Sobald sich der Cursor in der ersten oder letzten visuellen Zeile befindet, navigiert das erneute Drücken durch den Befehlsverlauf. Während Sie Nachrichten in der Warteschlange haben, [nimmt Oben aus der ersten Zeile diese stattdessen zurück](#take-back-what-you-queued)                                                                                                                                                            |
| `Esc`                                                                                                        | Claude unterbrechen oder einen Dialog schließen                                                                                                                                                                                                                                                                | Stoppen Sie die aktuelle Antwort oder den Werkzeugaufruf in der Mitte des Zuges, damit Sie umleiten können. Claude behält die bisherige Arbeit bei. Wenn Sie [Nachrichten in der Warteschlange haben](#queue-messages-while-claude-works), sendet Claude Code diese als nächstes. Wenn ein Dialog offen ist, schließt `Esc` den Dialog. Bei einer Berechtigungsaufforderung lehnt `Esc` die Aktion ab, genauso wie [**Nein** ohne Kommentar](/docs/de/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                              |
| `Esc` + `Esc`                                                                                                | Eingabeentwurf löschen oder zurückspulen                                                                                                                                                                                                                                                                       | Wenn die Eingabeaufforderung Text enthält, löscht doppeltes `Esc` diesen und speichert den Entwurf im Verlauf, damit `Oben` ihn abruft. Wenn die Eingabe leer ist, öffnet doppeltes `Esc` das [Zurückspul-Menü](/docs/de/checkpointing), um Code und Gespräch von einem früheren Punkt wiederherzustellen oder zusammenzufassen                                                                                                                                                                                                                                                                                   |
| `Ctrl+Enter` oder `Ctrl+X Ctrl+S`                                                                            | Warteschlangen-Nachrichten jetzt senden                                                                                                                                                                                                                                                                        | Sendet Ihre [Nachrichten in der Warteschlange](#queue-messages-while-claude-works) und Ihren Entwurf mit ihnen sofort. [Wenn Claude Code das sendet, das Sie in die Warteschlange eingereiht haben](#when-claude-code-sends-what-you-queued) behandelt, was mit dem Zug passiert, an dem Claude arbeitet. Im [Shell-Modus](#shell-mode-with-prefix) reiht der Schlüssel Ihren Befehl nur in die Warteschlange ein. In Terminals, die erweiterte Tasten nicht melden, kommt `Ctrl+Enter` als einfaches `Enter` an; `Ctrl+X Ctrl+S` funktioniert in jedem Terminal. Erfordert Claude Code v2.1.275 oder später |
| `Shift+Tab` oder `Alt+M` unter Windows, wenn die Node- oder Bun-Laufzeit den VT-Eingabemodus nicht aktiviert | Berechtigungsmodi durchlaufen                                                                                                                                                                                                                                                                                  | Durchlaufen Sie `default` (im Modusindikator als Manuell gekennzeichnet), `acceptEdits`, `plan` und, falls verfügbar, `bypassPermissions` und dann `auto`. Von `auto` wechselt der erste Druck zu `default`. Siehe [Berechtigungsmodi](/docs/de/permission-modes). Bei einer Dateiberechtigungsaufforderung schließt dieselbe Taste ein offenes [Kommentarfeld](/docs/de/permissions#add-a-comment-when-you-answer-a-permission-prompt). Ohne offenes Feld wählt es die Option aus, die die Aktion für den Rest der Sitzung ermöglicht, wenn die Aufforderung diese Option anbietet                                    |
| `Option+P` (macOS) oder `Alt+P` (Windows/Linux)                                                              | Modell wechseln                                                                                                                                                                                                                                                                                                | Wechseln Sie Modelle, ohne Ihre Eingabeaufforderung zu löschen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `Option+T` (macOS) oder `Alt+T` (Windows/Linux)                                                              | Erweitertes Denken umschalten                                                                                                                                                                                                                                                                                  | Aktivieren oder deaktivieren Sie den erweiterten Denkmodus. Hat keine Auswirkung auf Opus 5.5 oder die Fable-Modelle, die immer erweitertes Denken verwenden. Funktioniert auf macOS ohne Konfiguration von Option als Meta                                                                                                                                                                                                                                                                                                                                                                                  |
| `Option+O` (macOS) oder `Alt+O` (Windows/Linux)                                                              | Schnellmodus umschalten                                                                                                                                                                                                                                                                                        | Aktivieren oder deaktivieren Sie den [Schnellmodus](/docs/de/fast-mode)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

<h3 id="text-editing">
  Textbearbeitung
</h3>

| Kürzel                       | Beschreibung                                         | Kontext                                                                                                                                                                                                                         |
| :--------------------------- | :--------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Ctrl+A`                     | Cursor an den Anfang der aktuellen Zeile verschieben | Bei mehrzeiliger Eingabe verschiebt dies den Cursor an den Anfang der aktuellen logischen Zeile                                                                                                                                 |
| `Ctrl+E`                     | Cursor an das Ende der aktuellen Zeile verschieben   | Bei mehrzeiliger Eingabe verschiebt dies den Cursor an das Ende der aktuellen logischen Zeile                                                                                                                                   |
| `Ctrl+K`                     | Bis zum Zeilenende löschen                           | Speichert gelöschten Text zum Einfügen                                                                                                                                                                                          |
| `Ctrl+U`                     | Vom Cursor zum Zeilenanfang löschen                  | Speichert gelöschten Text zum Einfügen. Wiederholen Sie dies, um über Zeilen in mehrzeiliger Eingabe zu löschen. Unter macOS ordnen Terminal-Emulatoren einschließlich iTerm2 und Terminal.app `Cmd+Backspace` diesem Kürzel zu |
| `Ctrl+W`                     | Zurück zum vorherigen Leerzeichen löschen            | Speichert gelöschten Text zum Einfügen. Ein Druck entfernt einen ganzen Pfad oder `--flag=value`. Um nur das vorherige Wort zu löschen, drücken Sie `Option+Delete` auf macOS oder `Ctrl+Backspace` unter Windows               |
| `Ctrl+Y`                     | Gelöschten Text einfügen                             | Fügt den Text ein, den Sie zuletzt mit einem der Wort- oder Zeilenlösch-Kürzel gelöscht haben, wie `Ctrl+K`, `Ctrl+U` oder `Ctrl+W`                                                                                             |
| `Alt+Y` (nach `Ctrl+Y`)      | Einfügeverlauf durchlaufen                           | Nach dem Einfügen durchlaufen Sie den zuvor gelöschten Text. Erfordert [Option als Meta](#keyboard-shortcuts) auf macOS                                                                                                         |
| `Alt+B`                      | Cursor um ein Wort zurück verschieben                | Wortnavigation. Erfordert [Option als Meta](#keyboard-shortcuts) auf macOS                                                                                                                                                      |
| `Alt+F`                      | Cursor um ein Wort vorwärts verschieben              | Verschiebt zum Ende des aktuellen Wortes oder zum Ende des nächsten Wortes, wenn sich der Cursor zwischen Wörtern befindet. Erfordert [Option als Meta](#keyboard-shortcuts) auf macOS                                          |
| `Alt+D`                      | Bis zum Wortende löschen                             | Löscht bis zum Ende des aktuellen Wortes oder zum Ende des nächsten Wortes, wenn sich der Cursor zwischen Wörtern befindet. Speichert gelöschten Text zum Einfügen. Erfordert [Option als Meta](#keyboard-shortcuts) auf macOS  |
| `Ctrl+_` oder `Ctrl+Shift+-` | Letzte Eingabebearbeitung rückgängig machen          | Stellt den vorherigen Eingabetext und die Cursorposition wieder her                                                                                                                                                             |

<h3 id="make-ctrl-w-delete-back-to-whitespace">
  Wortgrenzen in Bearbeitungskürzel
</h3>

Die Wortkürzel `Alt+B`, `Alt+F`, `Alt+D`, `Option+Delete` und `Ctrl+Backspace` behandeln ein Wort als eine Reihe von Buchstaben und Ziffern, daher trennen Satzzeichen wie `_`, `.` und `/` Wörter. Mit `src/utils/foo.ts` in der Eingabeaufforderung halten wiederholte Drücke von `Alt+B` am Anfang von `ts`, `foo`, `utils` und `src` an.

`Ctrl+W` ist anders: Es ignoriert Satzzeichen und löscht zurück zum vorherigen Leerzeichen, daher entfernt ein Druck alle `src/utils/foo.ts`.

In Text, der ohne Leerzeichen geschrieben ist, wie Chinesisch oder Japanisch, verschieben oder löschen die Wortkürzel immer noch ein Wort auf einmal.

Diese Readline-Konventionen gelten in Claude Code v2.1.261 und später. Die [`keybindingFlavor`](/docs/de/settings-reference#keybindingflavor)-Einstellung, die sie in früheren Versionen aktivierte, ist veraltet und hat keine Auswirkung.

Sie können diese Kürzel nicht in der [Tastaturkürzel-Konfigurationsdatei](/docs/de/keybindings) neu zuordnen, die keine Aktionen für sie hat.

<h3 id="theme-and-display">
  Design und Anzeige
</h3>

| Kürzel   | Beschreibung                                 | Kontext                                                                                                 |
| :------- | :------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| `Ctrl+T` | Syntaxhervorhebung für Codeblöcke umschalten | Funktioniert nur im `/theme`-Auswahlmenü. Steuert, ob Code in Claudes Antworten Syntaxfärbung verwendet |

<h3 id="multiline-input">
  Mehrzeilige Eingabe
</h3>

| Methode           | Kürzel          | Kontext                                                                                                                                                                                                |
| :---------------- | :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Schneller Ausgang | `\` + `Enter`   | Funktioniert in allen Terminals                                                                                                                                                                        |
| Option-Taste      | `Option+Enter`  | Nach Aktivierung von [Option als Meta](/docs/de/terminal-config#enable-option-key-shortcuts-on-macos) auf macOS                                                                                             |
| Shift+Enter       | `Shift+Enter`   | Nativ in iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal. Für andere Terminals siehe [Mehrzeilige Eingabeaufforderungen eingeben](/docs/de/terminal-config#enter-multiline-prompts) |
| Steuersequenz     | `Ctrl+J`        | Funktioniert in jedem Terminal ohne Konfiguration                                                                                                                                                      |
| Einfügemodus      | Direkt einfügen | Für Codeblöcke, Protokolle                                                                                                                                                                             |

<h3 id="quick-commands">
  Schnellbefehle
</h3>

| Kürzel                 | Beschreibung                     | Notizen                                                                                                                                                                                                                                                                                                                                                                                                       |
| :--------------------- | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/` am Anfang          | Befehl oder Fähigkeit            | Siehe [Befehle](#commands) und [Fähigkeiten](/docs/de/skills)                                                                                                                                                                                                                                                                                                                                                      |
| `!` am Anfang          | Shell-Modus                      | Führen Sie einen Befehl direkt aus, fügen Sie seine Ausgabe zur Sitzung hinzu und lassen Sie Claude darauf antworten                                                                                                                                                                                                                                                                                          |
| `@`                    | Dateipfad-Erwähnung              | Trigger-Dateipfad-Autovervollständigung. In Sitzungen mit [sitzungsübergreifendem Messaging](/docs/de/cross-session-messaging#message-another-session) schlägt Claude Code auch Ihre anderen Live-Sitzungen auf diesem Computer vor, wenn Sie mindestens einen Buchstaben nach dem `@` eingeben, damit Sie Claude mitteilen können, die ausgewählte zu benachrichtigen. Erfordert Claude Code v2.1.232 oder später |
| `:`                    | Emoji-Shortcode                  | Geben Sie ein vollständiges `:name:` ein, um das Emoji einzufügen, oder zwei oder mehr Zeichen für Vorschläge. Siehe [Emoji-Shortcodes](#emoji-shortcodes). Erfordert Claude Code v2.1.217 oder später                                                                                                                                                                                                        |
| `?` bei leerer Eingabe | Hilfepanel für Kürzel umschalten | Wenn Sie `?` eingeben, während die Eingabe bereits Text enthält, wird das Zeichen eingefügt                                                                                                                                                                                                                                                                                                                   |

<h3 id="transcript-viewer">
  Transkript-Viewer
</h3>

Wenn der Transkript-Viewer offen ist (mit `Ctrl+O` umgeschaltet), sind diese Kürzel verfügbar. Führen Sie `/tui` ohne Argument aus, um zu überprüfen, welcher Renderer aktiv ist. `Ctrl+E` kann über [`transcript:toggleShowAll`](/docs/de/keybindings) neu gebunden werden.

| Kürzel               | Beschreibung                                                                                                                                                                                                                              |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Hilfepanel für Tastaturkürzel umschalten. Erfordert [Vollbildmodus](/docs/de/fullscreen)                                                                                                                                                       |
| `{` / `}`            | Zur vorherigen oder nächsten Benutzereingabeaufforderung springen, wie vim-Absatzbewegung. Erfordert [Vollbildmodus](/docs/de/fullscreen)                                                                                                      |
| `Ctrl+E`             | Alle Inhalte anzeigen umschalten. Nur im klassischen Renderer verfügbar, nicht im [Vollbildmodus](/docs/de/fullscreen)                                                                                                                         |
| `[`                  | Schreiben Sie das gesamte Gespräch in den nativen Scrollback Ihres Terminals, damit `Cmd+F`, tmux-Kopiermodus und andere native Tools es durchsuchen können. Erfordert [Vollbildmodus](/docs/de/fullscreen#search-and-review-the-conversation) |
| `v`                  | Schreiben Sie das Gespräch in eine temporäre Datei und öffnen Sie es in `$VISUAL` oder `$EDITOR`. Erfordert [Vollbildmodus](/docs/de/fullscreen)                                                                                               |
| `q`, `Ctrl+C`, `Esc` | Transkript-Ansicht beenden. Alle drei können über [`transcript:exit`](/docs/de/keybindings) neu gebunden werden                                                                                                                                |

<h3 id="voice-input">
  Spracheingabe
</h3>

| Kürzel                         | Beschreibung | Notizen                                                                                                                                                                                                                            |
| :----------------------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Leertaste` halten oder tippen | Sprachdiktat | Erfordert, dass [Sprachdiktat](/docs/de/voice-dictation) aktiviert ist. Halten Sie gedrückt, um aufzunehmen, oder führen Sie `/voice tap` aus, um zum Umschalten zu tippen. [Neu bindbar](/docs/de/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Befehle
</h2>

Geben Sie `/` in Claude Code ein, um die verfügbaren Befehle anzuzeigen, oder geben Sie `/` gefolgt von beliebigen Buchstaben ein, um zu filtern. Das `/`-Menü listet integrierte Befehle, gebündelte und von Benutzern erstellte [Skills](/docs/de/skills), sowie Befehle auf, die von [Plugins](/docs/de/plugins/overview) und [MCP-Servern](/docs/de/mcp#use-mcp-prompts-as-commands) bereitgestellt werden. Nicht alle integrierten Befehle sind für jeden Benutzer sichtbar, da einige von Ihrer Plattform oder Ihrem Plan abhängen, und [einige verfügbare Befehle sind absichtlich im Menü verborgen](/docs/de/commands#how-the-command-menu-matches-what-you-type) und werden ausgeführt, wenn Sie ihren vollständigen Namen eingeben.

Bei der [Vollbildwiedergabe](/docs/de/fullscreen#use-the-mouse) reagieren die `/`-Befehlsliste und die `@`-Dateivorschlagsliste auch auf die Maus: Das Überfahren mit der Maus hebt eine Zeile hervor und das Anklicken akzeptiert sie.

Weitere Informationen finden Sie in der [Befehlsreferenz](/docs/de/commands) für die vollständige Liste der in Claude Code enthaltenen Befehle.

<h3 id="complete-a-command-mid-prompt">
  Befehl mitten in einer Eingabeaufforderung vervollständigen
</h3>

Die Befehlsvervollständigung funktioniert auch teilweise durch eine Eingabeaufforderung: Geben Sie `/` nach einem Leerzeichen ein, gefolgt von den ersten Buchstaben eines Namens, wie in `Tests ausführen, dann /com`. Nur Befehle, deren Namen mit diesen Buchstaben beginnen, stimmen überein, daher hält ein Dateipfad wie `/tmp/notes.md` keine Liste offen. Claude Code führt einen Befehl nur aus, wenn der Befehl [Ihre Nachricht startet](/docs/de/commands).

* **Bei der [Vollbildwiedergabe](/docs/de/fullscreen)**: Die Übereinstimmungen werden als Liste angezeigt, während Sie eingeben, ohne dass eine Zeile hervorgehoben ist, daher sendet `Enter` Ihre Eingabeaufforderung wie eingegeben. Drücken Sie `Tab`, um die beste Übereinstimmung einzufügen, oder wählen Sie eine Zeile mit den Pfeiltasten und `Enter` aus.
* **Außerhalb des Vollbildmodus**: Der Rest der besten Übereinstimmung wird als Geistertext an Ihrem Cursor angezeigt, mit einer Anzahl wie `+2`, wenn mehr Befehle übereinstimmen. Drücken Sie `Tab`, um die einzige Übereinstimmung einzufügen, oder um die Liste zu öffnen, wenn mehrere übereinstimmen, wählen Sie dann eine Zeile mit den Pfeiltasten und `Enter` aus.

In beiden Renderern drücken Sie `Tab` auf einem bloßen Mid-Prompt `/`, um jeden Befehl aufzulisten.

Ein Plugin-Skill stimmt auch mit seinem bloßen Namen überein, daher findet `/deploy` einen Skill namens `myplugin:deploy-app`. Wenn Sie die Übereinstimmung einfügen, schreibt Claude Code den vollständigen `/myplugin:deploy-app`.

<h2 id="vim-editor-mode">
  Vim-Editor-Modus
</h2>

Aktivieren Sie Vim-ähnliche Bearbeitung über `/config` → Editor mode.

Claude Code behält Ihren Vim-Modus und die Cursorposition bei, wenn Sie den [Transcript Viewer](#transcript-viewer) mit `Ctrl+O` umschalten oder ein Panel wie `/config` öffnen und schließen. Wenn Sie die Eingabeaufforderung im NORMAL-Modus verlassen, ist sie immer noch im NORMAL-Modus, wenn Sie zurückkehren, mit dem Cursor dort, wo Sie ihn verlassen haben.

<h3 id="mode-switching">
  Moduswechsel
</h3>

| Befehl              | Aktion                                                                                                                           | Aus Modus      |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------- | :------------- |
| `Esc` oder `Ctrl+[` | Geben Sie den NORMAL-Modus ein. In Terminals, die das Kitty-Tastaturprotokoll verwenden, erfordert `Ctrl+[` v2.1.242 oder später | INSERT, VISUAL |
| `i`                 | Vor dem Cursor einfügen                                                                                                          | NORMAL         |
| `I`                 | Am Anfang der Zeile einfügen                                                                                                     | NORMAL         |
| `a`                 | Nach dem Cursor einfügen                                                                                                         | NORMAL         |
| `A`                 | Am Ende der Zeile einfügen                                                                                                       | NORMAL         |
| `o`                 | Zeile unten öffnen                                                                                                               | NORMAL         |
| `O`                 | Zeile oben öffnen                                                                                                                | NORMAL         |
| `v`                 | Zeichenweise visuelle Auswahl starten                                                                                            | NORMAL         |
| `V`                 | Zeilenweise visuelle Auswahl starten                                                                                             | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  INSERT-Modus-Tastenkombinationen neu zuordnen
</h3>

Die Einstellung [`vimInsertModeRemaps`](/docs/de/settings-reference#viminsertmoderemaps) ordnet eine zweitastige INSERT-Modus-Sequenz der Escape-Taste zu, sodass eine Zuordnung wie `jj` Sie in den NORMAL-Modus zurückbringt. Erfordert Claude Code v2.1.208 oder später.

Das folgende `~/.claude/settings.json`-Beispiel aktiviert den Vim-Modus und ordnet `jj` der Escape-Taste zu:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Jeder Schlüssel besteht aus genau zwei druckbaren Zeichen, die nacheinander eingegeben werden, und `"<Esc>"` ist das einzige unterstützte Ziel. Einträge mit einer anderen Länge oder einem anderen Ziel werden ignoriert.

Das Eingeben des ersten Zeichens einer Sequenz fügt es normalerweise ein. Das Drücken des zweiten Zeichens innerhalb einer Sekunde entfernt das ausstehende Zeichen und wechselt in den NORMAL-Modus, wobei keines der beiden Zeichen in Ihrer Eingabe verbleibt. Nach dem Einsekundenfenster oder wenn eine andere Taste folgt, bleiben beide Zeichen als Literaltext erhalten, sodass Sie ein Wort mit der Sequenz immer noch eingeben können, indem Sie zwischen den beiden Tasten pausieren.

Claude Code liest diese Einstellung aus Ihrer Benutzereinstellungsdatei, dem Flag `--settings` und [verwalteten Einstellungen](/docs/de/managed-settings) nur. Einträge in der `.claude/settings.json` oder `.claude/settings.local.json` eines Projekts werden ignoriert, sodass ein ausgechecktes Repository Ihre Tastenanschläge nicht neu zuordnen kann.

<h3 id="navigation-normal-mode">
  Navigation (NORMAL-Modus)
</h3>

| Befehl          | Aktion                                                                                                                                                                       |
| :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Nach links/unten/oben/rechts bewegen                                                                                                                                         |
| `Space`         | Nach rechts bewegen                                                                                                                                                          |
| `w`             | Nächstes Wort                                                                                                                                                                |
| `e`             | Ende des Wortes                                                                                                                                                              |
| `b`             | Vorheriges Wort                                                                                                                                                              |
| `0`             | Anfang der Zeile                                                                                                                                                             |
| `$`             | Ende der Zeile                                                                                                                                                               |
| `^`             | Erstes Nicht-Leerzeichen-Zeichen                                                                                                                                             |
| `gg`            | Anfang der Eingabe                                                                                                                                                           |
| `G`             | Ende der Eingabe                                                                                                                                                             |
| `f{char}`       | Zur nächsten Vorkommen des Zeichens springen                                                                                                                                 |
| `F{char}`       | Zum vorherigen Vorkommen des Zeichens springen                                                                                                                               |
| `t{char}`       | Direkt vor das nächste Vorkommen des Zeichens springen                                                                                                                       |
| `T{char}`       | Direkt nach das vorherige Vorkommen des Zeichens springen                                                                                                                    |
| `;`             | Letzte f/F/t/T-Bewegung wiederholen                                                                                                                                          |
| `,`             | Letzte f/F/t/T-Bewegung in umgekehrter Reihenfolge wiederholen                                                                                                               |
| `/`             | Reverse-Verlaufssuche öffnen, dasselbe wie `Ctrl+R`. Der leere Suchprompt zeigt einen Hinweis: Drücken Sie `Esc` dann `i` dann `/`, um stattdessen das Befehlsmenü zu öffnen |

<Note>
  Im Vim-NORMAL-Modus, wenn sich der Cursor am Anfang oder Ende der Eingabe befindet und nicht weiter bewegt werden kann, navigieren `j`/`k` und `↑`/`↓` stattdessen durch den Befehlsverlauf. `←` auf einer leeren Eingabeaufforderung öffnet die [Agent-Ansicht](/docs/de/agent-view) sowohl im NORMAL- als auch im INSERT-Modus; vor v2.1.219 tat `←` auf einer leeren Eingabeaufforderung nichts im NORMAL-Modus.
</Note>

<h3 id="editing-normal-mode">
  Bearbeitung (NORMAL-Modus)
</h3>

| Befehl                | Aktion                                                                                                                                    |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `x`                   | Zeichen löschen                                                                                                                           |
| `dd`                  | Zeile löschen                                                                                                                             |
| `D`                   | Bis zum Ende der Zeile löschen                                                                                                            |
| `dw`/`de`/`db`        | Wort/bis zum Ende/zurück löschen                                                                                                          |
| `df{char}`/`dt{char}` | Bis zum nächsten Vorkommen eines Zeichens löschen und einschließen, oder bis dahin                                                        |
| `cc`                  | Zeile ändern                                                                                                                              |
| `C`                   | Bis zum Ende der Zeile ändern                                                                                                             |
| `cw`/`ce`/`cb`        | Wort/bis zum Ende/zurück ändern                                                                                                           |
| `s`                   | Zeichen ersetzen: Löschen Sie das Zeichen unter dem Cursor und geben Sie den INSERT-Modus ein. Erfordert Claude Code v2.1.211 oder später |
| `S`                   | Zeile ersetzen: Löschen Sie die Zeile und geben Sie den INSERT-Modus ein. Erfordert Claude Code v2.1.211 oder später                      |
| `yy`/`Y`              | Zeile yanken (kopieren)                                                                                                                   |
| `yw`/`ye`/`yb`        | Wort/bis zum Ende/zurück yanken                                                                                                           |
| `p`                   | Nach dem Cursor einfügen                                                                                                                  |
| `P`                   | Vor dem Cursor einfügen                                                                                                                   |
| `>>`                  | Zeile einrücken                                                                                                                           |
| `<<`                  | Zeile ausrücken                                                                                                                           |
| `J`                   | Zeilen verbinden                                                                                                                          |
| `u`                   | Rückgängig machen                                                                                                                         |
| `.`                   | Letzte Änderung wiederholen                                                                                                               |

<h3 id="text-objects-normal-mode">
  Textobjekte (NORMAL-Modus)
</h3>

Textobjekte funktionieren mit Operatoren wie `d`, `c` und `y`:

| Befehl    | Aktion                                       |
| :-------- | :------------------------------------------- |
| `iw`/`aw` | Inneres/um Wort                              |
| `iW`/`aW` | Inneres/um WORT (durch Leerzeichen begrenzt) |
| `i"`/`a"` | Inneres/um doppelte Anführungszeichen        |
| `i'`/`a'` | Inneres/um einfache Anführungszeichen        |
| `i(`/`a(` | Inneres/um Klammern                          |
| `i[`/`a[` | Inneres/um eckige Klammern                   |
| `i{`/`a{` | Inneres/um geschweifte Klammern              |

<h3 id="visual-mode">
  Visueller Modus
</h3>

Drücken Sie `v` für zeichenweise Auswahl oder `V` für zeilenweise Auswahl. Bewegungen erweitern die Auswahl, und Operatoren wirken direkt darauf.

| Befehl           | Aktion                                                        |
| :--------------- | :------------------------------------------------------------ |
| `d`/`x`          | Auswahl löschen                                               |
| `y`              | Auswahl yanken                                                |
| `c`/`s`          | Auswahl ändern                                                |
| `p`              | Auswahl durch Registerinhalte ersetzen                        |
| `r{char}`        | Jedes ausgewählte Zeichen durch `{char}` ersetzen             |
| `~`/`u`/`U`      | Auswahl umschalten, Kleinbuchstaben oder Großbuchstaben       |
| `>`/`<`          | Ausgewählte Zeilen einrücken oder ausrücken                   |
| `J`              | Ausgewählte Zeilen verbinden                                  |
| `o`              | Cursor und Anker tauschen                                     |
| `iw`/`aw`/`i"`/… | Ein Textobjekt auswählen                                      |
| `v`/`V`          | Zwischen zeichenweise und zeilenweise umschalten oder beenden |

Der blockweise visuelle Modus mit `Ctrl+V` wird nicht unterstützt.

<h2 id="command-history">
  Befehlsverlauf
</h2>

Claude Code speichert einen Verlauf der Eingabeaufforderungen, die Sie eingeben, und die Pfeiltaste-nach-oben-Funktion ruft Eingabeaufforderungen aus früheren Sitzungen desselben Projekts ab:

* Der Eingabeverlauf wird pro Arbeitsverzeichnis gespeichert
* Das Ausführen von `/clear` startet eine neue Sitzung: Die Rückruffunktion listet dann die Eingabeaufforderungen der neuen Sitzung zuerst auf, gefolgt von den Eingabeaufforderungen früherer Sitzungen. Das Gespräch der vorherigen Sitzung wird beibehalten und kann fortgesetzt werden.
* Das zweimalige Absenden derselben Eingabeaufforderung hintereinander zeichnet einen Verlaufseintrag auf, sodass das Drücken der Pfeiltaste nach oben zur vorherigen unterschiedlichen Eingabeaufforderung führt
* Wenn Sie eine Eingabeaufforderung abrufen, die eingefügten Text enthielt, sendet Claude Code den vollständigen eingefügten Inhalt erneut, wenn Sie sie erneut absenden. Wenn der Inhalt inzwischen [bereinigt](/docs/de/claude-directory#cleaned-up-automatically) wurde, sendet Claude Code nicht die wörtliche Zeichenkette `[Pasted text #N]`; siehe [Großen Inhalt einfügen](/docs/de/terminal-config#paste-large-content) für Details, was mit der Eingabeaufforderung geschieht
* Die Verlaufserweiterung mit `!` ist standardmäßig deaktiviert

<h3 id="reverse-search-with-ctrl-r">
  Rückwärtssuche mit Strg+R
</h3>

Drücken Sie `Strg+R`, um interaktiv durch Ihren Befehlsverlauf zu suchen. Bei [Vollbilddarstellung](/docs/de/fullscreen) öffnet `Strg+R` stattdessen einen Suchdialog: Geben Sie Text ein, um zu filtern, drücken Sie `Pfeiltaste nach oben` und `Pfeiltaste nach unten`, um sich durch Treffer zu bewegen, und drücken Sie `Strg+S`, um den Bereich durch diese Sitzung, dieses Projekt und alle Projekte zu durchlaufen. Drücken Sie `Eingabe` oder `Tab`, um einen Treffer in die Eingabeaufforderung einzufügen, oder `Esc`, um abzubrechen. Die folgenden Schritte beschreiben die Inline-Suche des klassischen Renderers:

1. **Suche starten**: Drücken Sie `Strg+R`, um die Rückwärtssuche im Verlauf zu aktivieren
2. **Abfrage eingeben**: Geben Sie Text ein, um in vorherigen Befehlen zu suchen. Der Suchbegriff wird in übereinstimmenden Ergebnissen hervorgehoben
3. **Treffer navigieren**: Drücken Sie `Strg+R` erneut, um durch ältere Treffer zu durchlaufen
4. **Suchbereich**: Die Inline-Suche durchsucht immer Eingabeaufforderungen aus allen Projekten
5. **Treffer akzeptieren**:
   * Drücken Sie `Tab` oder `Esc`, um den aktuellen Treffer zu akzeptieren und die Bearbeitung fortzusetzen
   * Drücken Sie `Eingabe`, um den Befehl zu akzeptieren und sofort auszuführen
6. **Suche abbrechen**:
   * Drücken Sie `Strg+C`, um abzubrechen und Ihre ursprüngliche Eingabe wiederherzustellen
   * Drücken Sie `Rücktaste` bei leerer Suche, um abzubrechen

Die Inline-Suche durchsucht Ihren vollständigen Eingabeverlauf, neueste zuerst, mit Duplikaten, die auf das neueste Vorkommen reduziert sind. Der Vollbilddialog durchsucht Ihren gesamten Eingabeverlauf im ausgewählten Bereich, neueste zuerst, mit Duplikaten, die auf das neueste Vorkommen reduziert sind: Die neuesten Eingabeaufforderungen werden sofort angezeigt, und Treffer aus älteren Eingabeaufforderungen werden angezeigt, während Claude Code den Rest des Verlaufs lädt. Übereinstimmende Eingabeaufforderungen werden mit dem hervorgehobenen Suchbegriff angezeigt, sodass Sie vorherige Eingaben finden und wiederverwenden können.

Das Akzeptieren eines Treffers oder das Abbrechen der Suche wird sofort wirksam, auch während Claude Code den Verlauf noch lädt.

<h2 id="background-bash-commands">
  Bash-Befehle im Hintergrund
</h2>

Claude Code unterstützt die Ausführung von Bash-Befehlen im Hintergrund, sodass Sie weiterarbeiten können, während lange laufende Prozesse ausgeführt werden.

<h3 id="how-backgrounding-works">
  Funktionsweise des Hintergrunds
</h3>

Wenn Claude Code einen Befehl im Hintergrund ausführt, wird der Befehl asynchron ausgeführt und eine Hintergrund-Task-ID wird sofort zurückgegeben. Claude Code kann auf neue Eingaben reagieren, während der Befehl im Hintergrund weiterhin ausgeführt wird.

Um Befehle im Hintergrund auszuführen, können Sie entweder:

* Claude Code auffordern, einen Befehl im Hintergrund auszuführen
* `Ctrl+B` drücken, um eine reguläre Bash-Tool-Invokation in den Hintergrund zu verschieben. Tmux-Benutzer müssen `Ctrl+B` zweimal drücken, da Tmux eine Präfixaste hat.

**Wichtigste Funktionen:**

* Die Ausgabe wird in eine Datei geschrieben und Claude kann sie mit dem Read-Tool abrufen
* Hintergrund-Tasks haben eindeutige IDs zur Verfolgung und zum Abrufen der Ausgabe
* Hintergrund-Tasks werden automatisch bereinigt, wenn Claude Code beendet wird. Unter macOS und Linux werden Prozesse, die sich von der Shell der Task abgetrennt haben, wie solche, die unter `setsid` oder `timeout` gestartet wurden, auch beendet, wenn Sie eine Hintergrund-Task von [`/tasks`](/docs/de/commands) aus stoppen oder Claude Code sie beim Beenden stoppt
* Wenn Sie die Sitzung in den Hintergrund verschieben, anstatt sie zu beenden, werden Ihre Hintergrund-Tasks in der Hintergrund-Sitzung weiterhin ausgeführt. Siehe [Sitzung in den Hintergrund verschieben](/docs/de/agent-view#from-inside-a-session)
* Hintergrund-Tasks werden automatisch beendet, wenn die Ausgabe 5 GB überschreitet, mit einer Notiz in stderr, die erklärt, warum
* Unter macOS und Linux beendet Claude Code laufende Hintergrund-Tasks, wenn das Betriebssystem ein Speicherdrucksignal sendet, sofern die Sitzung mindestens 30 Minuten untätig war und kein Turn oder Subagent ausgeführt wird. Erfordert Claude Code v2.1.193 oder später
  * Das [Debug-Protokoll](/docs/de/debug-your-config) zeigt, warum Tasks beendet wurden, oder warum ein Druckereignis sie weiterhin ausführen ließ
  * Setzen Sie [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/de/env-vars) auf `1`, um Speicherdruckstopps auszuschalten
* Hintergrund-Befehle, die sich im Besitz eines [Subagenten](/docs/de/sub-agents) befinden, haben keine Zeitbegrenzung, außer dass ein Befehl, der sich im Besitz eines Subagenten befindet und im Vordergrund ausgeführt wird, endet, wenn dieser Subagent seine endgültige Antwort gibt; siehe [Hintergrund-Befehle](/docs/de/tools-reference#background-commands) in der Tools-Referenz. Vor v2.1.218 deckten weder die Speicherdruckreap noch das 60-Minuten-Limit Befehle ab, die mit `Ctrl+B` in den Hintergrund verschoben wurden

Um alle Hintergrund-Task-Funktionen zu deaktivieren, setzen Sie die Umgebungsvariable `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` auf `1`. Siehe [Umgebungsvariablen](/docs/de/env-vars) für Details.

**Häufig in den Hintergrund verschobene Befehle:**

* Build-Tools (webpack, vite, make)
* Paketmanager (npm, yarn, pnpm)
* Test-Runner (jest, pytest)
* Entwicklungsserver
* Lange laufende Prozesse (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Shell-Modus mit `!`-Präfix
</h3>

Führen Sie Shell-Befehle direkt aus, ohne Claude zu durchlaufen, indem Sie Ihre Eingabe mit `!` präfixieren:

```bash theme={null}
! npm test
! git status
! ls -la
```

Shell-Modus:

* Fügt den Befehl und seine Ausgabe zum Gesprächskontext hinzu
* Zeigt Echtzeit-Fortschritt und Ausgabe
* Unterstützt das gleiche `Ctrl+B`-Backgrounding für lange laufende Befehle
* Erfordert nicht, dass Claude den Befehl interpretiert oder genehmigt
* Unterstützt verlaufsbasierte Autovervollständigung: Geben Sie einen Teilbefehl ein und drücken Sie `Tab`, um aus vorherigen `!`-Befehlen im aktuellen Projekt zu vervollständigen
* Unterstützt Live-Dateipfad-Autovervollständigung ab v2.1.193 auf allen Plattformen: Geben Sie ein Token mit einem Schrägstrich ein, wie `./src/` oder `~/`, um ein Dropdown-Menü mit übereinstimmenden Dateien und Verzeichnissen zu sehen, und drücken Sie dann `Tab`, um zu akzeptieren. Verwenden Sie auch unter Windows Schrägstriche; das Dropdown-Menü wird durch `/` ausgelöst, nicht durch `\`
* Beenden Sie mit `Escape`, `Backspace` oder `Ctrl+U` bei einer leeren Eingabeaufforderung
* Das Einfügen von Text, der mit `!` beginnt, in eine leere Eingabeaufforderung aktiviert automatisch den Shell-Modus und entspricht dem eingegebenen `!`-Verhalten

Sofern Ihre Sitzung nicht eine der unter [striktem Sandbox-Modus](/docs/de/sandboxing#the-unsandboxed-retry-escape-hatch) aufgelisteten ist, werden Befehle, die Sie im Shell-Modus eingeben, außerhalb der [Sandbox](/docs/de/sandboxing) ausgeführt, auch wenn Sie Sandboxing aktiviert haben, da die Sandbox für die Befehle gilt, die Claude ausführt.

Claude antwortet automatisch auf die Befehlsausgabe, sobald sie im Transkript ankommt, sodass Sie `! npm test` ausführen und eine Erklärung der Fehler ohne eine zweite Eingabeaufforderung erhalten können. Die Antwort kostet das Gleiche wie das Senden einer normalen Eingabeaufforderung. Um das frühere Verhalten wiederherzustellen, bei dem die Ausgabe zum Kontext hinzugefügt wird, ohne eine Antwort zu geben, setzen Sie [`respondToBashCommands`](/docs/de/settings-reference#respondtobashcommands) auf `false` in `settings.json`. Vor v2.1.186 hat der Shell-Modus die Ausgabe immer zum Kontext hinzugefügt, ohne eine Antwort zu geben.

<h2 id="queue-messages-while-claude-works">
  Nachrichten in die Warteschlange einreihen, während Claude arbeitet
</h2>

Geben Sie eine Nachricht ein und drücken Sie `Enter`, während Claude arbeitet. Claude Code reiht die Nachricht in die Warteschlange ein, anstatt den Zug zu unterbrechen, und listet die eingereihten Einträge über dem Eingabefeld auf, bis sie gesendet werden. Sie können `!` [Shell-Befehle](#shell-mode-with-prefix) und die meisten [Befehle](/docs/de/commands) auf die gleiche Weise einreihen, mit Ausnahme von Befehlen wie `/status`, die Claude Code sofort nach dem Senden ausführt.

Gesendete und eingereihte Nachrichten werden grau angezeigt, bis Claude mit der Antwort beginnt, sodass Sie sehen können, welche Nachrichten Claude noch nicht bearbeitet hat.

<h3 id="when-claude-code-sends-what-you-queued">
  Wann Claude Code das Eingereihte sendet
</h3>

Wann ein eingereihter Eintrag Claude erreicht, hängt davon ab, was Sie eingereicht haben.

* Nachrichten: Wenn Sie eine Nachricht einreihen, während Claude Tool-Aufrufe ausführt, übergibt Claude Code die Nachricht an Claude, sobald diese Tool-Aufrufe beendet sind, innerhalb desselben Zugs. Wenn der Zug mit noch eingereihten Nachrichten endet, werden sie ohne einen weiteren Tastendruck gesendet, in der Reihenfolge, in der Sie sie eingegeben haben
* Befehle und Shell-Befehle: Claude Code hält sie bis zum Ende des Zugs, dann führt sie nacheinander aus und behält die Reihenfolge bei, in der Sie sie eingereicht haben

Um das Eingereihte zu senden, ohne zu warten, drücken Sie `Ctrl+Enter`. Ihre eingereihten Nachrichten werden sofort gesendet, mit Ihrem Entwurf eingereicht dahinter, falls Sie einen eingegeben haben. Erfordert Claude Code v2.1.275 oder später.

Wenn Sie einen `!` Shell-Befehl vor Ihren Nachrichten eingereicht haben, unterbricht die Taste den Zug. Andernfalls hängt das, was mit dem Zug geschieht, davon ab, was Claude tut, wenn Sie die Taste drücken:

* Shell-Befehle, Subagenten oder andere Arbeiten ausführen, die in den [Hintergrund](#background-bash-commands) verschoben werden können: Diese Arbeit wird in den Hintergrund verschoben und läuft weiter, und Claude liest Ihre Nachrichten im selben Zug
* Nur eine Antwort schreiben oder etwas ausführen, das nicht in den Hintergrund verschoben werden kann: Claude Code unterbricht den Zug und sendet Ihre Nachrichten als nächstes. Vor v2.1.281 unterbrach die Taste den Zug in beiden Fällen

Im [Shell-Modus](#shell-mode-with-prefix) reiht die Taste nur Ihren Befehl ein. In Terminals, die erweiterte Tasten nicht melden, kommt `Ctrl+Enter` als einfaches `Enter` an und reiht den Entwurf stattdessen ein; `Ctrl+X Ctrl+S` funktioniert in jedem Terminal. Beide Tasten sind Bindungen der [`chat:sendNow`-Aktion](/docs/de/keybindings#chat-actions).

Drücken Sie `Esc`, um den Zug zu unterbrechen, ohne Ihren Entwurf zu senden. Claude Code behält das Eingereihte und sendet es sofort.

Claude Code führt einige Befehle sofort nach dem Senden aus, anstatt sie einzureihen, darunter `/model`, `/effort` und `/fast`. Jeder der drei ändert eine Einstellung: das Modell, die Aufwandsstufe oder den Schnellmodus. Ob Claude Code die neue Einstellung auf den Zug anwendet, an dem Claude bereits arbeitet, oder erst ab dem nächsten Zug, unterscheidet sich je nach Befehl:

* [`/model`](/docs/de/model-config#setting-your-model): Nachdem Sie die [Cache-Warnung](/docs/de/prompt-caching#switching-models) bestätigt haben, falls Claude Code eine anzeigt, wendet Claude Code Ihre Änderung auf die nächste Anfrage an, die es in diesem Zug stellt
* [`/effort`](/docs/de/model-config#adjust-effort-level): Nachdem Sie die [Cache-Warnung](/docs/de/prompt-caching#changing-effort-level) bestätigt haben, falls Claude Code eine anzeigt, wendet Claude Code Ihre Änderung auf die nächste Anfrage an, die es in diesem Zug stellt
* [`/fast`](/docs/de/fast-mode#toggle-fast-mode): Claude Code behält die Schnellmodus-Einstellung bei, die beim Start des Zugs aktiv war, sodass Ihre Geschwindigkeitsänderung ab dem nächsten Zug gilt. Wenn Ihr aktuelles Modell den Schnellmodus nicht unterstützt, wird durch das Aktivieren auch [Ihr Modell gewechselt](/docs/de/prompt-caching#turning-on-fast-mode), und Claude Code verwendet das neue Modell ab seiner nächsten Anfrage in diesem Zug

<h3 id="take-back-what-you-queued">
  Nehmen Sie das Eingereihte zurück
</h3>

Drücken Sie `Up` von der ersten Zeile des Eingabefelds, um die eingereihten Nachrichten und Befehle zurückzunehmen. Claude Code entfernt sie aus der Warteschlange und fügt sie in das Eingabefeld ein, einen pro Zeile, vor jedem Text, den Sie eingegeben haben. Bearbeiten Sie den Text und drücken Sie `Enter`, um ihn erneut als einen Eintrag einzureihen, oder löschen Sie das Eingabefeld, um ihn zu verwerfen.

Claude Code nimmt eingereihte Shell-Befehle nur zurück, wenn das Eingabefeld leer ist und Sie nichts anderes eingereicht haben, und es schaltet das Eingabefeld in den Shell-Modus um, wenn es das tut. Andernfalls lässt es sie in der Warteschlange, aufgelistet mit ihrem `!`-Präfix, und führt sie nach dem Ende des Zugs aus.

<h2 id="prompt-suggestions">
  Eingabeaufforderungsvorschläge
</h2>

Wenn Sie eine Sitzung zum ersten Mal öffnen, zeigt Claude Code einen ausgegrauten Beispielbefehl in der Eingabeaufforderung an, um Ihnen den Einstieg zu erleichtern. Dieser wird aus dem Git-Verlauf Ihres Projekts ausgewählt, sodass das Beispiel Dateien widerspiegelt, an denen Sie kürzlich gearbeitet haben.

Nachdem Claude antwortet, kann Claude Code basierend auf Ihrem Gesprächsverlauf einen Vorschlag für Ihre nächste Eingabeaufforderung machen, z. B. einen Folgenschritt aus einer mehrteiligen Anfrage oder eine natürliche Fortsetzung Ihres Arbeitsablaufs.

* Drücken Sie `Tab` oder `Pfeil nach rechts`, um den Vorschlag in die Eingabeaufforderung einzufügen, und dann `Eingabe`, um ihn abzusenden
* Beginnen Sie zu tippen, um ihn zu verwerfen

Claude Code generiert jeden dieser Vorschläge für die nächste Eingabeaufforderung mit einer Hintergrundanfrage an das gleiche Modell, das Ihre Sitzung verwendet. Die Anfrage wird auf die Nutzungslimits Ihres Plans oder Ihre API-Kosten angerechnet. Da sie den Prompt-Cache des Gesprächs wiederverwendet, besteht sie hauptsächlich aus Cache-Lesevorgängen plus einigen Ausgabe-Token, sodass die zusätzlichen Kosten minimal sind.

<h3 id="when-claude-code-skips-suggestions">
  Wenn Claude Code Vorschläge überspringt
</h3>

Im interaktiven Modus deaktiviert Claude Code Eingabeaufforderungsvorschläge standardmäßig und verbirgt den Umschalter **Eingabeaufforderungsvorschläge** in `/config` in einer [Sitzung, die keine Feature-Flags abruft](/docs/de/env-vars#features-that-need-feature-flag-fetching), z. B. eine bei einem Drittanbieter oder über ein Claude-Apps-Gateway, und in einer [ersten Sitzung nach einer Installation oder einem Upgrade](/docs/de/env-vars#first-session-after-an-install-or-upgrade), deren Flags noch nicht angekommen sind.

Claude Code überspringt auch einzelne Vorschläge in mehreren Situationen, einschließlich:

* Der Prompt-Cache ist kalt, um unnötige Kosten zu vermeiden
* Nach dem ersten Zug eines Gesprächs in einigen Sitzungen
* Die vorherige Antwort endete mit einem Fehler
* Während Sie sich im Plan Mode befinden
* Ihr Konto ist nahe an oder hat sein Nutzungslimit erreicht. Um Vorschläge bis zum Erreichen des Limits aktiviert zu halten, setzen Sie [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/de/env-vars) auf `true`. Vor v2.1.238 übersprangen Claude Code diese in der Nähe des Limits auch mit der auf `true` gesetzten Variablen
* In einem [Agent-Team](/docs/de/agent-teams), standardmäßig in den Sitzungen von Teamkollegen. Die Sitzung des Leiters zeigt Vorschläge

Im Druckmodus generiert Claude Code standardmäßig keine Vorschläge. Übergeben Sie [`--prompt-suggestions`](/docs/de/cli-reference#cli-flags) mit `-p "<prompt>" --output-format stream-json --verbose`, damit Claude Code nach jedem Zug, der einen generiert, eine `prompt_suggestion`-Nachricht ausgibt. Der Generator überspringt auch hier sehr kurze Gespräche und kalte Prompt-Caches, sodass eine einzelne kurze `-p`-Abfrage keine ausgeben kann.

<h3 id="turn-prompt-suggestions-off">
  Eingabeaufforderungsvorschläge ausschalten
</h3>

Um Eingabeaufforderungsvorschläge vollständig zu deaktivieren, verwenden Sie eine der folgenden Optionen:

* Schalten Sie **Eingabeaufforderungsvorschläge** in `/config` aus
* Setzen Sie [`promptSuggestionEnabled`](/docs/de/settings-reference#promptsuggestionenabled) in Ihrer Einstellungsdatei auf `false`
* Setzen Sie die Umgebungsvariable [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/de/env-vars) auf `false`, die Vorrang vor der Einstellung hat:
  ```bash theme={null}
  export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
  ```

Um Eingabeaufforderungsvorschläge organisationsweit auszuschalten, setzen Sie `promptSuggestionEnabled` in [verwalteten Einstellungen](/docs/de/managed-settings) auf `false`. Setzen Sie auch `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION` auf `false` unter dem verwalteten Schlüssel [`env`](/docs/de/settings-reference#env), damit Benutzer diese nicht mit ihrer eigenen Umgebungsvariablen erneut aktivieren können.

<h2 id="emoji-shortcodes">
  Emoji-Kurzcodes
</h2>

Geben Sie einen `:` gefolgt von einem Emoji-Kurzcode in die Eingabeaufforderung ein, um das Emoji einzufügen. Erfordert Claude Code v2.1.217 oder später.

* Geben Sie einen vollständigen Kurzcode wie `:heart:` ein, und Claude Code ersetzt ihn mit ❤️ sobald Sie den schließenden `:` eingeben
* Geben Sie `:` plus mindestens zwei Zeichen eines Namens ein, z. B. `:hea`, um ein Vorschlagsfenster zu öffnen, und drücken Sie dann `Tab` oder `Enter`, um das hervorgehobene Emoji einzufügen

Der Kurzcode muss am Anfang der Eingabe oder nach einem Leerzeichen stehen, daher öffnet ein `:` innerhalb eines Wortes oder einer URL keine Vorschläge.

Um die Funktion auszuschalten, setzen Sie [`emojiCompletionEnabled`](/docs/de/settings-reference#emojicompletionenabled) auf `false` in `settings.json`. Dies deaktiviert sowohl das Vorschlagsfenster als auch den Inline-Ersatz.

<h2 id="check-spelling-as-you-type">
  Rechtschreibung während der Eingabe überprüfen
</h2>

Claude Code kann falsch geschriebene Wörter in der Eingabeaufforderung während der Eingabe unterstreichen. Es überprüft nur den Text im Eingabefeld, niemals Claudes Antworten oder Ihre Dateien. Es überprüft auch nichts, während sich das Eingabefeld im [Shell-Modus](#shell-mode-with-prefix) befindet, in der `Ctrl+R`-Verlaufssuche oder in der [Spracherfassung](/docs/de/voice-dictation).

Die Rechtschreibprüfung ist standardmäßig deaktiviert, und Claude Code überprüft nichts im [Bildschirmlesemodus](/docs/de/accessibility). Erfordert Claude Code v2.1.235 oder später.

<h3 id="prerequisites">
  Voraussetzungen
</h3>

* Installieren Sie [aspell](https://github.com/GNUAspell/aspell), [hunspell](https://github.com/hunspell/hunspell) oder [ispell](https://en.wikipedia.org/wiki/Ispell) und stellen Sie sicher, dass es sich in Ihrem `PATH` befindet. Claude Code führt das erste der drei Programme aus, das es findet, in dieser Reihenfolge auf jeder Plattform aus, einschließlich eines `.cmd`-Shims, das ein Paketmanager unter Windows installiert.
* Um zu überprüfen, ob sich das Programm in Ihrem `PATH` befindet, führen Sie `aspell --version`, `hunspell --version` oder `ispell -v` in Ihrem Terminal aus. Ein Fehler „command not found" bedeutet, dass es sich noch nicht in Ihrem `PATH` befindet.

<h3 id="turn-spell-checking-on-or-off">
  Rechtschreibprüfung ein- oder ausschalten
</h3>

Claude Code liest die Einstellung [`spellcheck`](/docs/de/settings-reference#spellcheck) von drei Stellen und ignoriert sie in der `.claude/settings.json` und `.claude/settings.local.json` eines Projekts. Schalten Sie sie von der Stelle ein, die Sie verwenden:

<Tabs>
  <Tab title="Benutzereinstellungen">
    Fügen Sie `spellcheck` zu `~/.claude/settings.json` hinzu. Es gilt in jedem Projekt, das Sie öffnen, wie der Rest Ihrer [Benutzereinstellungen](/docs/de/settings#where-settings-live):

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>

  <Tab title="Befehlszeile">
    Speichern Sie `spellcheck` in einer JSON-Datei, z. B. `spellcheck.json`:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```

    Übergeben Sie dann die Datei an `--settings`. Es gilt nur für diese Sitzung:

    ```bash theme={null}
    claude --settings spellcheck.json
    ```
  </Tab>

  <Tab title="Verwaltete Einstellungen">
    Fügen Sie `spellcheck` zu einer der [verwalteten Einstellungsquellen](/docs/de/permissions#managed-settings) Ihrer Organisation hinzu. Es gilt für jeden Benutzer, der diese Einstellungen erhält, und diese können es nicht ausschalten:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>
</Tabs>

Um zu überprüfen, ob die Rechtschreibprüfung aktiviert ist, geben Sie ein falsch geschriebenes Wort und ein Leerzeichen ein. Claude Code unterstreicht das Wort. Wenn dies nicht der Fall ist, siehe [Wenn Claude Code nichts unterstreicht](#when-claude-code-underlines-nothing). Um die Rechtschreibprüfung wieder auszuschalten, setzen Sie `enabled` an derselben Stelle auf `false` oder entfernen Sie `spellcheck`.

Um auszuwählen, welches der drei Programme Claude Code ausführt, welches Wörterbuch es verwendet oder welche Unterstreichungsfarbe verwendet wird, fügen Sie neben `enabled` an derselben Stelle eines dieser Felder hinzu:

* `checker`: `aspell`, `hunspell` oder `ispell`. Claude Code fällt nicht auf einen benannten Checker zurück und behandelt jeden anderen Wert als `auto`.
* `language`: ein Wörterbuchname in der Form Ihres Checkers, z. B. `en_GB`. Claude Code ignoriert jeden Wert, der kein einfacher Wörterbuchname ist, z. B. ein Pfad oder ein Name mit Leerzeichen, und der Checker verwendet sein Standardwörterbuch.
* `color`: ein Farbname wie `yellow` oder ein `#rrggbb`-, `#rgb`-, `rgb(r,g,b)`-, `ansi256(n)`- oder `ansi:<name>`-Wert. Claude Code verwendet standardmäßig die Fehlerfarbe Ihres Designs und für jeden Wert, den es nicht erkennt.

Beispielsweise führt diese `spellcheck`-Einstellung hunspell mit seinem `en_GB`-Wörterbuch aus und unterstreicht Wörter in Gelb. Es funktioniert gleich in `~/.claude/settings.json`, in der Datei, die Sie an `--settings` übergeben, und in verwalteten Einstellungen:

```json theme={null}
{
  "spellcheck": {
    "enabled": true,
    "checker": "hunspell",
    "language": "en_GB",
    "color": "yellow"
  }
}
```

Wenn mehr als eine der drei Stellen eine `spellcheck`-Einstellung hat, verwendet Claude Code nur eine davon: zuerst verwaltete Einstellungen, dann `--settings`, dann Benutzereinstellungen. Es kombiniert keine Felder von zwei Stellen. Wenn beispielsweise `--settings` `spellcheck` setzt, hat eine `language` in Ihren Benutzereinstellungen keine Auswirkung.

<h3 id="what-claude-code-underlines">
  Was Claude Code unterstreicht
</h3>

Kurz nachdem Sie mit der Eingabe pausieren, unterstreicht Claude Code die Wörter, die das Wörterbuch nicht kennt. Es lässt das Wort, das Sie noch eingeben, allein, bis Sie es verlassen, und es ändert niemals Ihren Text. Es überspringt auch Text, der wie Code aussieht:

* Befehle wie `/help`, `@`-Erwähnungen, URLs, Dateipfade und Flags wie `--verbose`
* Wörter mit Ziffern, Unterstrichen oder einem Großbuchstaben nach dem ersten, und Text in Backticks

Claude Code überspringt auch chinesischen, japanischen, koreanischen, thailändischen, laotischen, kambodschanischen und burmesischen Text.

Claude Code hat keine eigene Wortliste: Ein Wort ist falsch geschrieben, wenn Ihr Checker dies sagt. Um Claude Code davon abzuhalten, ein Wort zu unterstreichen, fügen Sie das Wort zum persönlichen Wörterbuch Ihres Checkers hinzu, gemäß der Dokumentation des Checkers. Claude Code übernimmt das neue Wort, nachdem Sie es neu starten.

<h3 id="when-claude-code-underlines-nothing">
  Wenn Claude Code nichts unterstreicht
</h3>

Claude Code unterstreicht nichts, wenn es einen Checker nicht ausführen kann:

* Kein Checker ist installiert, oder der in `checker` benannte fehlt
* Der Checker schlägt zweimal hintereinander fehl, beim Start oder später in der Sitzung. Claude Code startet ihn nach dem ersten Fehler neu und stoppt die Überprüfung nach dem zweiten, bis Sie Claude Code neu starten
* Der Checker benötigt mehr als 15 Sekunden zum Antworten, dreimal. Jedes Mal lässt Claude Code die Wörter, auf die es wartete, unmarkiert; nach dem dritten Mal stoppt es die Überprüfung, bis Sie Claude Code neu starten

Um herauszufinden, welches dieser Ereignisse passiert ist, starten Sie `claude --debug` mit aktivierter Rechtschreibprüfung und geben Sie ein Wort ein. Suchen Sie dann nach den `[spellcheck]`-Zeilen im Debug-Protokoll unter `~/.claude/debug/<session-id>.txt`. Eine Zeile nennt das Programm, das Claude Code gestartet hat, oder listet die auf, die es gesucht hat und nicht gefunden hat. Spätere Zeilen sagen, warum es gestoppt hat. Ein Fehler „missing-dictionary" dort bedeutet, dass der Checker kein Wörterbuch für Ihren `language`-Wert hat, oder kein Standardwörterbuch, wenn `language` nicht gesetzt ist. Installieren Sie eines, oder setzen Sie `language` auf ein Wörterbuch, das Sie haben.

<h2 id="invisible-characters-in-prompts">
  Unsichtbare Zeichen in Eingabeaufforderungen
</h2>

Eingefügter Text kann Unicode-Zeichen enthalten, die ein Terminal überhaupt nicht anzeigt, wie Tag-Zeichen, bidirektionale Steuerzeichen und Leerzeichen mit Nullbreite, sodass eine Eingabeaufforderung Text enthalten kann, den Sie nie sehen. Um zu verhindern, dass kopierter Text Anweisungen enthält, die Ihr Terminal nicht anzeigt, entfernt Claude Code diese Zeichen, wenn Sie die Eingabetaste drücken, bevor etwas gesendet wird. Es bereinigt sowohl die Eingabeaufforderung als auch den Inhalt aller [eingefügten Textreferenzen](/docs/de/terminal-config#paste-large-content), die die Eingabeaufforderung enthält. Claude Code behält die Verbinder bei, die persische und indische Schriften schreiben, sowie die Selektoren in Emoji-Sequenzen.

Wenn Claude Code etwas entfernt hat, sendet diese Eingabetaste nichts. Die bereinigte Eingabeaufforderung wird mit einer Meldung wie `3 unsichtbare Zeichen entfernt · überprüfen und Eingabetaste drücken zum Senden` in das Eingabefeld zurückgesetzt, und das erneute Drücken der Eingabetaste sendet den Text wie angezeigt.

Wenn Sie eine Eingabeaufforderung in der Befehlszeile übergeben, wie in `claude "fix the login bug"`, oder eine in eine interaktive Sitzung weiterleiten, wartet Claude Code nicht auf eine zweite Eingabetaste. Es entfernt die Zeichen, zeigt eine Meldung an und sendet die bereinigte Eingabeaufforderung. Wenn die bereinigte Eingabeaufforderung mit `/` beginnen würde, setzt Claude Code sie in das Eingabefeld, damit Sie sie überprüfen und senden können.

<h2 id="review-changes-with-/diff">
  Änderungen mit /diff überprüfen
</h2>

Führen Sie `/diff` aus, um die Änderungen in Ihrem Arbeitsverzeichnis zu überprüfen, ohne Claude Code zu verlassen. Sie sehen die bisherigen Bearbeitungen von Claude zusammen mit allem anderen, das Sie noch nicht committed haben.

In den Änderungen, die `/diff` aus Git liest, wird ein Submodul als einzelner Eintrag angezeigt, und nur wenn sich der Commit, auf den es verweist, ändert; Bearbeitungen von Dateien innerhalb des Submoduls werden dort nicht angezeigt.

In der [Vollbilddarstellung](/docs/de/fullscreen) öffnet `/diff` das [Diff-Panel](#diff-panel) neben dem Gespräch, das offen bleibt und sich aktualisiert, während Sie weiterarbeiten. Im klassischen Renderer öffnet `/diff` den [Diff-Viewer](#diff-viewer) anstelle der Eingabeaufforderung, und Sie schließen ihn, wenn Sie fertig sind.

<h3 id="diff-panel">
  Diff-Panel
</h3>

Das Diff-Panel listet die geänderten Dateien mit ihren hinzugefügten und entfernten Zeilenzahlen auf und zeigt das Diff jeder Datei unter der Liste. Claude Code aktualisiert es jedes Mal, wenn Claude eine Datei bearbeitet oder einen Shell-Befehl ausführt. Um es zu schließen, führen Sie `/diff` erneut aus oder klicken Sie auf das `✕` in seiner Kopfzeile.

Um das Panel zu verwenden, benötigen Sie:

* [Vollbilddarstellung](/docs/de/fullscreen)
* Ein Git-Repository
* Ein Terminal mit mindestens 110 Spalten Breite
* Claude Code v2.1.260 oder später

Wenn das Panel nicht geöffnet werden kann, öffnet `/diff` stattdessen den Diff-Viewer oder teilt Ihnen mit, warum.

Das Panel öffnet sich auch von selbst, sobald Claude mit der Bearbeitung von Dateien beginnt, wenn Ihr Terminal mindestens 144 Spalten breit ist. Nachdem Sie es selbst mit `/diff` geöffnet haben, öffnen spätere Sitzungen es sofort, sobald Claude eine Datei in einem Terminal bearbeitet, das breit genug ist. Schließen Sie das Panel und es bleibt geschlossen, in dieser Sitzung und später, bis Sie `/diff` erneut ausführen.

Während das Panel offen ist, können Sie:

* **Zu einer Datei springen**: Klicken Sie auf ihre Zeile in der Liste. Scrollen Sie das Panel mit dem Mausrad. Wenn die Dateiliste selbst zu lang ist, scrollen Sie sie mit `Alt+Up` und `Alt+Down` oder `Ctrl+Up` und `Ctrl+Down`.
* **Claude nach bestimmten Zeilen fragen**: Wählen Sie diese im Panel mit der Maus aus. Claude Code fügt die Auswahl an Ihre nächste Eingabeaufforderung an und zeigt eine Zeilenzahl neben der Eingabe an, bis Sie sie senden.
  * Um die Eingabeaufforderung ohne die Auswahl zu senden, verschieben Sie den Cursor direkt nach dem Zeilenzahl-Indikator und drücken Sie `Backspace`, um ihn zu löschen. Erfordert Claude Code v2.1.271 oder später.
* **Die Dateien anzeigen, die das Panel auslässt**: Die Liste überspringt Testdateien und generierte Dateien und reduziert Änderungen von vor dieser Sitzung auf eine Zeile am unteren Rand. Klicken Sie auf eine der beiden Zeilenzahlen, um sie zu erweitern.
* **Ändern, womit das Panel verglichen wird**: Drücken Sie `Ctrl+X B`, um zwischen den Änderungen dieser Sitzung, Ihren uncommitted Änderungen als eine Liste und allem seit dem Verzweigungspunkt Ihres Branches vom Standard-Branch zu wechseln. Claude Code merkt sich die Auswahl für jedes Projekt.

Um Tasten an diese Aktionen zu binden, siehe [Diff-Panel-Aktionen](/docs/de/keybindings#diff-panel-actions).

<h3 id="diff-viewer">
  Diff-Viewer
</h3>

Der Diff-Viewer ersetzt die Eingabeaufforderung, bis Sie ihn schließen. Seine **Current**-Ansicht zeigt Ihre uncommitted Änderungen aus Git oder, wenn es keine gibt, was Ihr Branch zusätzlich zum Standard-Branch hinzufügt. Der Viewer hat auch eine Turn-Ansicht für jeden Prompt, nach dem Claude Dateien bearbeitet hat, die nur diese Bearbeitungen zeigt. Claude Code erstellt die Turn-Ansichten aus Claudes Dateibearbeitungen und nicht aus Git, daher wird eine Änderung, die Claude durch einen Shell-Befehl vornimmt, nur unter Current angezeigt.

Verwenden Sie diese Tasten im Viewer:

* **Links und Rechts**: Wechsel zwischen Current und den Turn-Ansichten.
* **Oben und Unten**: Wählen Sie eine Datei aus.
* **Enter**: Öffnen Sie das Diff der ausgewählten Datei. Scrollen Sie es mit Oben und Unten oder PageUp und PageDown.
* **Esc**: Kehren Sie vom Diff einer Datei zur Liste zurück oder schließen Sie den Viewer von der Liste aus.

Um diese Tasten neu zu binden, siehe [Diff-Aktionen](/docs/de/keybindings#diff-actions).

<h2 id="side-questions-with-/btw">
  Nebenfragen mit /btw
</h2>

Verwenden Sie `/btw`, um eine Frage zu Ihrer aktuellen Arbeit zu stellen, ohne sie zum Gesprächsverlauf hinzuzufügen.

```
/btw what was the name of that config file again?
```

Claude beantwortet eine Nebenfrage basierend auf dem, was bereits im Gespräch vorhanden ist: Ihre Nachrichten, seine Antworten und die Werkzeugergebnisse, die es gesammelt hat. Sie können nach Code fragen, den Claude bereits gelesen hat, nach Entscheidungen, die es früher getroffen hat, oder nach allem anderen aus der Sitzung. Eine spätere Nebenfrage sieht auch Ihre früheren Nebenfragen: Claude Code wiederholt die neuesten 20 Austausche bei jeder Frage, bis Sie diese löschen. Die Frage und Antwort werden nie zum Gesprächsverlauf hinzugefügt. Im Terminal werden sie in einem verwerfbaren Overlay angezeigt. Das Terminal behält den Thread im Speicher: Drücken Sie `x`, um die früheren Austausche zu löschen, und er ist weg, wenn Sie Claude Code beenden.

Im [Chat-Panel der VS Code-Erweiterung](/docs/de/vs-code#use-the-prompt-box) öffnet `/btw` ein Panel statt des in diesem Abschnitt beschriebenen Overlays, und Sie stellen Folgefragen direkt im Panel. Der Thread des Panels bleibt bei Fenster-Neuladen erhalten, gemäß dem Aufbewahrungsplan auf dieser Seite. Sie benötigen die Erweiterung in Version 2.1.227 oder später. Frühere Erweiterungsversionen bieten `/btw` nicht.

* **Verfügbar während Claude arbeitet**: Sie können `/btw` auch ausführen, während Claude eine Antwort verarbeitet. Die Nebenfrage läuft unabhängig und unterbricht den Hauptzug nicht. Sie sieht alles im Gespräch bisher, außer der Antwort, die Claude noch schreibt.
* **Kein Werkzeugzugriff**: Nebenfragen beantworten nur aus dem, was bereits im Kontext vorhanden ist. Claude kann keine Dateien lesen, Befehle ausführen oder suchen, wenn es eine Nebenfrage beantwortet. Wenn Claude Werkzeugaufrufe trotzdem als Text schreibt, endet die Antwort mit einem Hinweis, dass nichts ausgeführt wurde.
* **Einzelne Antwort**: Es gibt keine Folgezüge im Overlay. Um den Thread fortzusetzen, stellen Sie eine weitere `/btw`-Frage. Um in einer lokalen Sitzung mit vollständigem Werkzeugzugriff fortzufahren, drücken Sie `f`, um diese Frage und Antwort in einen [Hintergrund-Subagenten](/docs/de/sub-agents#fork-the-current-conversation) zu verzweigen.
* **Niedrige Kosten**: Während der [Prompt-Cache](/docs/de/prompt-caching) des Gesprächs warm ist, kostet eine Nebenfrage wenig über die Antwort selbst hinaus.

Ihre fünf neuesten früheren Nebenfragen werden als gedimmte Liste über der aktuellen Antwort angezeigt, mit einer Anzahl älterer. Sie bleiben außerhalb des Gesprächsverlaufs.

Um nach dem Schließen zum Overlay zurückzukehren, führen Sie `/btw` ohne Frage aus. Das Overlay öffnet sich erneut bei Ihrem letzten Austausch. Vor v2.1.212 gab `/btw` ohne Frage stattdessen eine Nutzungsmeldung aus.

Sobald die Antwort angezeigt wird, akzeptiert das Overlay diese Tasten.

| Taste                        | Aktion                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space`, `Enter`, `Escape`   | Schließen Sie die Antwort und kehren Sie zur Eingabeaufforderung zurück                                                                                                                                                                                                                                                                                                                                                               |
| `Up` / `Down`                | Scrollen Sie die Antwort                                                                                                                                                                                                                                                                                                                                                                                                              |
| `Shift+Left` / `Shift+Right` | Wechseln Sie zwischen dieser Antwort und Ihren früheren `/btw`-Antworten. `Shift+Left` wechselt zu älteren Antworten und `Shift+Right` kehrt zur aktuellen zurück. `[` und `]` machen dasselbe, für Terminals, die `Shift` nicht mit Pfeiltasten melden. `Tab` / `Shift+Tab` durchlaufen die gleichen Antworten. Erfordert Claude Code v2.1.257 oder später. Zwischen v2.1.187 und v2.1.256 waren die Tasten einfach `Left` / `Right` |
| `c`                          | Kopieren Sie die Antwort als rohen Markdown in Ihre Zwischenablage. Verwenden Sie dies statt Mausauswahl, die das hart umgebrochene Terminal-Rendering statt des Quelltexts erfasst                                                                                                                                                                                                                                                   |
| `f`                          | Starten Sie einen [verzweigten Subagenten](/docs/de/sub-agents#fork-the-current-conversation), der das übergeordnete Gespräch plus diese Frage und Antwort erbt, damit er mit vollständigem Werkzeugzugriff fortfahren kann. Sie bleiben in der aktuellen Sitzung und finden die Verzweigung im [Panel unter Ihrer Eingabeaufforderung](/docs/de/sub-agents#observe-and-steer-running-forks). Nur in lokalen Sitzungen verfügbar                |
| `x`                          | Löschen Sie die Liste der früheren `/btw`-Austausche, die über der aktuellen Antwort angezeigt werden                                                                                                                                                                                                                                                                                                                                 |

In einer angehängten [Hintergrund-Sitzung](/docs/de/agent-view#attach-to-a-session) trennt `Left` die Verbindung und bringt Sie zur Agent-Ansicht zurück, auch während die Antwort noch ankommt. Die Nebenfrage läuft weiter, während Sie weg sind. Das nächste Mal, wenn Sie sich an die Sitzung anhängen, öffnet sich das Overlay mit der Nebenfrage oder mit ihrer Antwort. Vor v2.1.257 trennte `Left` dort nicht.

`/btw` sieht Ihr vollständiges Gespräch, hat aber keine Werkzeuge. Ein [Subagent](/docs/de/sub-agents) hat Werkzeuge und startet von der Eingabeaufforderung, die er erhält, oder, für eine [Verzweigung](/docs/de/sub-agents#fork-the-current-conversation), von einer Kopie dieses Gesprächs. Verwenden Sie `/btw`, um zu fragen, was Claude bereits aus dieser Sitzung weiß; verwenden Sie einen Subagenten, um etwas Neues herauszufinden.

<h2 id="task-list">
  Aufgabenliste
</h2>

Die Aufgabenliste ist Claudes Checkliste: Elemente, die Claude erstellt hat, um mehrstufige Arbeiten zu planen, mit Indikatoren, die zeigen, was ausstehend, in Bearbeitung oder abgeschlossen ist. Sie ist vom Hintergrund-Task-View getrennt. Um laufende Shells und Subagenten zu sehen, verwenden Sie stattdessen [`/tasks`](/docs/de/commands).

Die Liste wird nur in Sitzungen gefüllt, die die Task-Tracking-Tools haben, die Claude Code standardmäßig auf [Claude 3.x-Modellen, Opus 4 bis 4.7, Sonnet 4 bis 4.6 und Haiku 4.5](/docs/de/tools-reference#task-tool-availability) bereitstellt. Bei jedem anderen Modell, einschließlich einer Modell-ID, die Claude Code nicht erkennt, bleibt die Liste leer, es sei denn, Sie aktivieren sie mit `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` oder einer der anderen Möglichkeiten unter [Task-Tool-Verfügbarkeit](/docs/de/tools-reference#task-tool-availability). Wenn die Sitzung die Tools hat, funktioniert die Aufgabenliste wie folgt:

* Drücken Sie `Ctrl+T`, um die Aufgabenlisten-Ansicht umzuschalten. Die Anzeige zeigt bis zu fünf Aufgaben gleichzeitig. Wenn Claude noch keine Checklistenelemente erstellt hat, hat das Umschalten keine sichtbare Auswirkung, da es nichts anzuzeigen gibt
* Wenn Sie die Liste erweitert lassen, stellt Claude Code die erweiterte Ansicht beim nächsten Start einer Sitzung wieder her, die noch Aufgaben enthält, z. B. mit `--resume` oder `--continue`. Wenn die Aufgabenliste leer ist, startet Claude Code sie eingeklappt
* Um alle Aufgaben anzuzeigen oder zu löschen, fragen Sie Claude direkt: „show me all tasks" oder „clear all tasks"
* Aufgaben bleiben über Kontext-Komprimierungen hinweg bestehen und helfen Claude, bei größeren Projekten organisiert zu bleiben
* Um eine Aufgabenliste über Sitzungen hinweg zu teilen, setzen Sie `CLAUDE_CODE_TASK_LIST_ID`, um ein benanntes Verzeichnis in `~/.claude/tasks/` zu verwenden: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Sitzungsübersicht
</h2>

Wenn Sie zum Terminal zurückkehren, nachdem Sie sich entfernt haben, zeigt Claude Code eine einzeilige Übersicht darüber an, was bisher in der Sitzung passiert ist. Die Übersicht wird im Hintergrund generiert, sobald mindestens drei Minuten seit dem letzten abgeschlossenen Zug vergangen sind und das Terminal nicht fokussiert ist, sodass sie bereit ist, wenn Sie zurückwechseln. Übersichten werden nur angezeigt, wenn die Sitzung mindestens drei Züge hat, und nie zweimal hintereinander.

Führen Sie `/recap` aus, um eine Zusammenfassung bei Bedarf zu generieren. Claude Code begrenzt sowohl automatische Übersichten als auch `/recap`-Ausgabe auf 400 Zeichen. Um automatische Übersichten auszuschalten, öffnen Sie `/config` und deaktivieren Sie **Sitzungsübersicht**.

Sitzungsübersicht ist standardmäßig für jeden Plan und Anbieter aktiviert. Die Übersicht wird im nicht-interaktiven Modus immer übersprungen.

<h2 id="wait-for-a-usage-limit-to-reset">
  Auf das Zurücksetzen eines Nutzungslimits warten
</h2>

Wenn ein claude.ai [Nutzungslimit](/docs/de/errors#youve-hit-your-session-limit) Claude mitten in einer Aufgabe stoppt, wartet Claude Code in der offenen Sitzung und setzt die Aufgabe nach dem Zurücksetzen des Limits automatisch fort. Die automatische Fortsetzung ist standardmäßig in interaktiven Sitzungen aktiviert, die mit einem claude.ai-Abonnement angemeldet sind. Erfordert Claude Code v2.1.234 oder später.

Während Claude Code wartet, zeigt eine Zeile am unteren Ende der Sitzung an, wann die Fortsetzung erfolgt:

```text theme={null}
Usage limit reached · continuing automatically at 3:45pm · esc to cancel
```

Halten Sie die Sitzung offen. Was als Nächstes geschieht, hängt davon ab, wie das Warten endet:

* **Bei dem Zurücksetzen**: Die Zeile liest `continuing shortly`, dann `Usage limit reset · continuing automatically`, und Claude Code sendet Claude einen festen Prompt, um die Aufgabe dort fortzusetzen, wo sie unterbrochen wurde. Es sendet Ihre letzte Nachricht nicht erneut.
* **Nach dem Ruhezustand Ihres Computers**: Wenn dieser länger als etwa 30 Minuten im Ruhezustand war und das Limit während des Ruhezustands zurückgesetzt wurde, liest die Zeile `Your usage limit has reset · press enter to continue`. Drücken Sie `Enter`, um fortzufahren. Nach einem kürzeren Ruhezustand setzt Claude Code automatisch fort.
* **Früher**: Wenn Sie während des Wartens [Nutzungsguthaben](/docs/de/costs#add-usage-credits-to-your-subscription) mit `/usage-credits` hinzufügen, sich nach `/upgrade` erneut anmelden oder Modelle mit `/model` wechseln, prüft Claude Code, ob die Nutzung wieder verfügbar ist, und setzt sofort fort, falls dies der Fall ist. Es prüft nicht nach einem Upgrade oder Kauf, den Sie selbst in einem Browser durchführen. Unter [`opusplan`](/docs/de/model-config#opusplan-model-setting) und anderen Modelleinstellungen, die den Plan Mode auf einem anderen Modell ausführen, wartet Claude Code stattdessen auf das Zurücksetzen.

Die fortgesetzte Aufgabe läuft wie jede andere Runde. Claude Code fragt weiterhin wie gewohnt nach [Berechtigungen](/docs/de/permissions), sodass die Aufgabe bei einer Eingabeaufforderung unterbrochen werden kann, während Sie weg sind. Wenn es das Limit erneut erreicht, rüstet Claude Code das Warten automatisch höchstens zweimal hintereinander erneut aus und stoppt dann mit der Anzeige `Automatic continue stopped after repeated usage-limit hits · /rate-limit-options to try again`.

<h3 id="cancel-the-wait">
  Das Warten abbrechen
</h3>

Drücken Sie `Esc` bei einer leeren Eingabeaufforderung oder `Ctrl+C`, während die Zeile angezeigt wird, oder führen Sie [`/rate-limit-options`](/docs/de/commands#all-commands) aus und wählen Sie **Don't continue automatically**. Claude Code bestätigt dies mit einer Zeile, die mit `Automatic continue cancelled` beginnt.

Nach einem Abbruch wird nichts fortgesetzt, bis Sie einen Prompt senden oder die Zeile, die mit **Wait here, then continue automatically** beginnt, erneut aus `/rate-limit-options` auswählen. Claude Code startet das Warten nicht von selbst erneut für dieses Zurücksetzen-Fenster; das nächste Zurücksetzen-Fenster beginnt von vorne.

Das Warten endet auch ohne Fortsetzung der Aufgabe in diesen Fällen:

* **Sie senden einen Prompt**: Claude Code führt Ihren Prompt aus, anstatt zu warten.
* **Sie beenden Claude Code**: Das Warten wird nicht neu gestartet, wenn Sie die Sitzung fortsetzen.
* **Die Konversation wechselt den Besitzer**: Sie wechseln Konten mit `/login`, löschen oder setzen die Konversation zurück, `/resume` eine andere Sitzung, ziehen eine mit `/teleport`, starten mit `/tui` neu oder übergeben die Sitzung an Claude Desktop, eine Hintergrund-Sitzung oder die Cloud.
* **Die Einstellung wird deaktiviert oder das Zurücksetzen überschreitet 24 Stunden**: Dies beendet nur ein Warten, das Claude Code selbst gestartet hat. Ein Warten, das Sie aus `/rate-limit-options` ausgewählt haben, läuft weiter.
* **Die Fortsetzung ist blockiert**: Ein [`UserPromptSubmit` Hook](/docs/de/hooks#userpromptsubmit), der den Fortsetzungs-Prompt blockiert, oder ein Fehler, bevor er das Modell erreicht, beendet das Warten. Claude Code teilt Ihnen mit, dass die Fortsetzung nicht ausgeführt wurde. Senden Sie einen Prompt, um fortzufahren.

<h3 id="start-a-wait-yourself">
  Starten Sie selbst ein Warten
</h3>

Claude Code startet das Warten nicht von selbst in diesen Fällen:

* **Remote Control und Agent Team Teammate-Sitzungen**: Eine Person an diesem Terminal kann immer noch eine starten.
* **Ein Zurücksetzen mehr als 24 Stunden entfernt**: Ein wöchentliches Limit kann Tage entfernt zurückgesetzt werden.
* **Ein Opus- oder Sonnet-Limit, während Sie ein Modell außerhalb dieser Familie ausführen**: Ihre nächste Runde erreicht möglicherweise nicht dieses Limit. [`opusplan`](/docs/de/model-config#opusplan-model-setting) und andere Modelleinstellungen, die den Plan Mode auf der begrenzten Familie ausführen, erhalten diese Ausnahme nicht.

In diesen Fällen und wenn die automatische Fortsetzung deaktiviert ist, öffnet Claude Code das Menü mit Nutzungslimit-Optionen einmal pro Zurücksetzen-Fenster, wenn Sie ein Limit an Ihrem eigenen Terminal erreichen. Wählen Sie die Zeile, die mit **Wait here, then continue automatically** beginnt, um das Warten zu starten. In einer [Remote Control](/docs/de/remote-control) oder [Agent Team](/docs/de/agent-teams) Teammate-Sitzung führen Sie `/rate-limit-options` selbst aus, um das Menü zu öffnen.

Claude Code bietet das Warten überhaupt nicht in diesen Fällen an:

* **Hintergrund-Sitzungen und `-p` Läufe**: Die Menüzeile ist nicht verfügbar.
* **API-Schlüssel, Cloud-Anbieter und nutzungsbasierte Abrechnung**: Die Nutzung dort wird pro Anfrage gemessen, daher gibt es kein Zurücksetzen, auf das man warten könnte.
* **Ein [LLM Gateway](/docs/de/llm-gateway#subscriptions-and-gateways) ohne gespeicherte claude.ai-Anmeldung**: Claude Code bietet das Warten nur an, wenn eine gespeicherte claude.ai-Anmeldung die aktive Anmeldeinformation ist.

<h3 id="turn-automatic-continue-off">
  Automatische Fortsetzung deaktivieren
</h3>

Deaktivieren Sie in `/config` **Continue automatically at usage limit**, oder setzen Sie [`autoContinueAtUsageLimit`](/docs/de/settings-reference#autocontinueatusagelimit) in Ihren Benutzereinstellungen auf `false`. `/config autoContinueAtUsageLimit=false` funktioniert auch, einschließlich mit `-p`, aber die `key=value`-Form kann es nicht wieder aktivieren, da die Einstellung unbeaufsichtigte Ausführung gewährt. Welche Einstellungsdateien Claude Code für diesen Schlüssel liest, finden Sie in der [Einstellungsreferenz](/docs/de/settings-reference#autocontinueatusagelimit).

<h2 id="pr-review-status">
  PR-Review-Status
</h2>

Wenn Sie an einem Branch mit einem offenen Pull Request arbeiten, zeigt Claude Code einen anklickbaren PR-Link in der Fußzeile an, z. B. „PR #446". Der Link hat eine farbige Unterlinie, die den Review-Status anzeigt:

* Grün: genehmigt
* Gelb: Review ausstehend
* Rot: Änderungen angefordert
* Grau: Entwurf

Das Badge verschwindet, sobald der Pull Request zusammengeführt oder geschlossen wird.

`Cmd+Klick` (macOS) oder `Strg+Klick` (Windows/Linux) auf den Link, um den Pull Request in Ihrem Browser zu öffnen.

Der Status wird aktualisiert, sobald ein `git push` oder ein `gh pr`-Befehl, der den Pull Request ändert, wie `gh pr create` oder `gh pr merge`, in der Sitzung erfolgreich ist.

Claude Code rendert das Badge als Hyperlink, auch wenn es keine Hyperlink-Unterstützung in Ihrem Terminal erkennen kann, was häufig über SSH oder in tmux vorkommt. Setzen Sie [`FORCE_HYPERLINK=0`](/docs/de/env-vars), um das Badge als Klartext zu rendern.

Wenn Sie [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars) setzen, prüft Claude Code den Pull Request- oder Merge Request-Status nicht.

<Note>
  Der PR-Status für GitHub-Repositorys benötigt ein GitHub-Token. Claude Code findet eines basierend auf dem Host des Remote:

  * **github.com**: `GH_TOKEN` oder `GITHUB_TOKEN`, oder das Token, das von `gh auth login` gespeichert wurde. Ohne eines zeigt die Fußzeile `install gh for PR status` an, wenn die `gh` CLI nicht installiert ist, oder `gh auth login for PR status`, wenn sie installiert ist
  * **Ein GitHub Enterprise-Host, der als `GH_HOST` gesetzt ist**: `GH_ENTERPRISE_TOKEN` oder `GITHUB_ENTERPRISE_TOKEN`, oder das Token, das von `gh auth login --hostname <host>` gespeichert wurde. Ohne eines zeigt die Fußzeile die gleichen Hinweise
  * **Jeder andere GitHub-Host**: das Token, das von `gh auth login --hostname <host>` gespeichert wurde. Ohne eines zeigt Claude Code kein Badge und keinen Hinweis
</Note>

<h3 id="gitlab-merge-requests">
  GitLab Merge Requests
</h3>

Wenn Sie an einem Branch mit einem offenen GitLab Merge Request arbeiten, zeigt Claude Code ein anklickbares `MR !N`-Badge in der Fußzeile an, das ansonsten den GitHub PR-Link enthält. `!N` ist GitLabs eigene Referenzsyntax für Merge Request Nummer N. Die farbige Unterlinie zeigt den Status des Merge Request:

* Grün: GitLab meldet, dass der Merge Request zusammenführbar ist
* Gelb: jeder andere offene Status
* Grau: Entwurf

Das Badge verschwindet, sobald der Merge Request zusammengeführt oder geschlossen wird.

Es wird aktualisiert, sobald ein `git push` oder ein `glab mr`-Befehl, der den Merge Request ändert, wie `glab mr create` oder `glab mr merge`, in der Sitzung erfolgreich ist.

Um das Badge zu erhalten, benötigen Sie:

* Claude Code v2.1.234 oder später
* Ein Repository-Remote, das auf Ihren GitLab-Host verweist, entweder gitlab.com oder eine selbstverwaltete Instanz
* Die [`glab` CLI](https://gitlab.com/gitlab-org/cli) auf Ihrem `PATH`, authentifiziert mit `glab auth login`

Claude Code ignoriert `glab`s Token-Umgebungsvariablen, wie `GITLAB_TOKEN`, wenn es den Status prüft, sodass Sie kein Badge von einem exportierten Token allein erhalten. Claude Code sucht auch nach `glab` und nach seinem Login einmal pro Sitzung, daher starten Sie Claude Code neu, nachdem Sie `glab` installiert haben oder `glab auth login` ausgeführt haben.

<h2 id="issue-reference-links">
  Verweislinks für Probleme
</h2>

Wenn Claude ein Problem als `owner/repo#123` erwähnt, können Sie auf den Verweis klicken, um es zu öffnen, solange Ihr Terminal Hyperlinks unterstützt. Wenn Claude Code keine Hyperlink-Unterstützung in Ihrem Terminal erkennt, setzen Sie [`FORCE_HYPERLINK`](/docs/de/env-vars) auf `1`, um die Links einzuschalten, oder auf `0`, um Verweise als einfachen Text beizubehalten.

Sie erhalten einen Link nur für die zweiteilige Form `owner/repo#123`. Diese bleiben einfacher Text:

* Ein einfaches `#123`
* Ein verschachtelter GitLab-Pfad wie `group/subgroup/project#123`
* Jeder Verweis innerhalb einer Code-Spanne oder eines Code-Blocks

Claude Code erstellt den Link für den Host des Repositorys, das es aus Ihrem Git-Remote identifiziert, nicht für das Repository, das der Verweis benennt:

| Host Ihres Repositorys                                                               | Wo `owner/repo#123` verlinkt                 |
| :----------------------------------------------------------------------------------- | :------------------------------------------- |
| github.com, ein GitHub Enterprise-Host oder ein Host, der nicht unten aufgeführt ist | `https://<host>/owner/repo/issues/123`       |
| gitlab.com                                                                           | `https://gitlab.com/owner/repo/-/issues/123` |
| bitbucket.org, codeberg.org oder gitea.com                                           | Kein Link; der Verweis bleibt einfacher Text |

<h2 id="see-also">
  Siehe auch
</h2>

* [Skills](/docs/de/skills) - Benutzerdefinierte Prompts und Workflows
* [Checkpointing](/docs/de/checkpointing) - Spulen Sie Claudes Änderungen zurück und stellen Sie vorherige Zustände wieder her
* [CLI-Referenz](/docs/de/cli-reference) - Befehlszeilenflags und Optionen
* [Einstellungen](/docs/de/settings) - Konfigurationsoptionen
* [Speicherverwaltung](/docs/de/memory) - Verwalten von CLAUDE.md-Dateien
