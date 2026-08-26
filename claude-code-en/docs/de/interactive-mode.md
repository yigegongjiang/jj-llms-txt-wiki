> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Interaktiver Modus

> Vollständige Referenz für Tastaturkürzel, Eingabemodi und interaktive Funktionen in Claude Code-Sitzungen.

<h2 id="keyboard-shortcuts">
  Tastaturkürzel
</h2>

<Note>
  Tastaturkürzel können je nach Plattform und Terminal variieren. Im [Vollbildrendering](/docs/de/fullscreen) drücken Sie `?` im Transkript-Viewer, um die verfügbaren Kürzel dort anzuzeigen.

  **macOS-Benutzer**: Option/Alt-Tastenkürzel (`Alt+B`, `Alt+F`, `Alt+Y`, `Alt+M`, `Alt+P`) erfordern die Konfiguration von Option als Meta in Ihrem Terminal:

  * **iTerm2**: Einstellungen → Profile → Keys → General → Left/Right Option key auf „Esc+" setzen
  * **Apple Terminal**: Einstellungen → Profile → Keyboard → „Use Option as Meta Key" aktivieren
  * **VS Code**: `"terminal.integrated.macOptionIsMeta": true` in VS Code-Einstellungen setzen

  Weitere Informationen finden Sie unter [Terminal-Konfiguration](/docs/de/terminal-config).
</Note>

<h3 id="general-controls">
  Allgemeine Steuerelemente
</h3>

| Kürzel                                                        | Beschreibung                                                                                                                                                                            | Kontext                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Ctrl+C`                                                      | Unterbrechen oder Eingabe löschen                                                                                                                                                       | Unterbricht einen laufenden Vorgang. Wenn nichts läuft, löscht der erste Tastendruck die Eingabeaufforderung und ein zweiter Tastendruck beendet Claude Code                                                                                                                                                                                                                    |
| `Ctrl+X Ctrl+K`                                               | Alle laufenden [Hintergrund-Subagenten](/docs/de/sub-agents#run-subagents-in-foreground-or-background) in dieser Sitzung beenden. Zweimal innerhalb von 3 Sekunden drücken, um zu bestätigen | Subagenten-Steuerung                                                                                                                                                                                                                                                                                                                                                            |
| `Ctrl+D`                                                      | Claude Code-Sitzung beenden                                                                                                                                                             | EOF-Signal                                                                                                                                                                                                                                                                                                                                                                      |
| `Ctrl+G` oder `Ctrl+X Ctrl+E`                                 | Im Standard-Texteditor öffnen                                                                                                                                                           | Bearbeiten Sie Ihren Prompt oder benutzerdefinierte Antwort in Ihrem Standard-Texteditor. `Ctrl+X Ctrl+E` ist die readline-native Bindung. Aktivieren Sie „Show last response in external editor" in `/config`, um Claudes vorherige Antwort als `#`-kommentierter Kontext über Ihrem Prompt einzufügen; der Kommentarblock wird beim Speichern entfernt                        |
| `Ctrl+L`                                                      | Bildschirm neu zeichnen                                                                                                                                                                 | Erzwingt eine vollständige Terminal-Neuzeichnung. Eingabe und Gesprächsverlauf werden beibehalten. Verwenden Sie dies, um die Anzeige wiederherzustellen, wenn sie verzerrt oder teilweise leer wird                                                                                                                                                                            |
| `Ctrl+O`                                                      | Transkript-Viewer umschalten                                                                                                                                                            | Zeigt detaillierte Tool-Nutzung und Ausführung mit einem Zeitstempel und dem auf jeder Assistenten-Nachricht verwendeten Modell an. Erweitert auch MCP-Aufrufe, die standardmäßig zu einer einzelnen Zeile wie „Called slack 3 times" zusammengefasst werden                                                                                                                    |
| `Ctrl+R`                                                      | Reverse-Suche im Befehlsverlauf                                                                                                                                                         | Durchsuchen Sie vorherige Befehle interaktiv                                                                                                                                                                                                                                                                                                                                    |
| `Ctrl+V` oder `Cmd+V` (iTerm2) oder `Alt+V` (Windows und WSL) | Bild aus Zwischenablage einfügen                                                                                                                                                        | Fügt einen `[Image #N]`-Chip an der Cursor-Position ein, sodass Sie ihn positionell in Ihrem Prompt referenzieren können. Unter WSL sind sowohl `Ctrl+V` als auch `Alt+V` gebunden; verwenden Sie `Alt+V`, wenn Ihr Terminal `Ctrl+V` abfängt                                                                                                                                   |
| `Ctrl+B`                                                      | Hintergrund-Ausführung von Aufgaben                                                                                                                                                     | Führt Bash-Befehle und Agenten im Hintergrund aus. Tmux-Benutzer drücken zweimal                                                                                                                                                                                                                                                                                                |
| `Ctrl+T`                                                      | Task-Liste umschalten                                                                                                                                                                   | Zeigen oder verbergen Sie [Claudes Task-Liste](#task-list) im Statusbereich. Dies ist nicht die Hintergrund-Task-Ansicht; verwenden Sie [`/tasks`](/docs/de/commands), um laufende Shells und Subagenten anzuzeigen                                                                                                                                                                  |
| `Left/Right arrows`                                           | Durch Dialog-Registerkarten navigieren                                                                                                                                                  | Navigieren Sie zwischen Registerkarten in Berechtigungsdialogen und Menüs                                                                                                                                                                                                                                                                                                       |
| `Up/Down arrows` oder `Ctrl+P`/`Ctrl+N`                       | Cursor bewegen oder Befehlsverlauf navigieren                                                                                                                                           | Wenn die Eingabe mehr als eine visuelle Zeile umfasst, ob umgebrochen oder mehrzeilig, bewegt sich der Cursor zunächst innerhalb der Eingabeaufforderung. Sobald sich der Cursor bereits am oberen oder unteren Rand befindet, navigiert das erneute Drücken durch den Befehlsverlauf. Ab v2.1.169 verhält sich umgebrochene einzeilige Eingabe genauso wie mehrzeilige Eingabe |
| `Esc`                                                         | Claude unterbrechen oder einen Dialog schließen                                                                                                                                         | Stoppen Sie die aktuelle Antwort oder den Tool-Aufruf in der Mitte des Zuges, um umzuleiten. Claude behält die bisherige Arbeit bei. Wenn ein Dialog wie eine Berechtigungsaufforderung offen ist, schließt `Esc` den Dialog, anstatt Claude zu unterbrechen. Vor v2.1.202 unterbrach `Esc` bei einigen Dialogen Claude und ließ den Dialog offen                               |
| `Esc` + `Esc`                                                 | Eingabeentwurf löschen oder zurückspulen                                                                                                                                                | Wenn die Eingabeaufforderung Text enthält, löscht doppeltes `Esc` sie und speichert den Entwurf im Verlauf, sodass `Up` ihn abruft. Wenn die Eingabe leer ist, öffnet doppeltes `Esc` das [Zurückspul-Menü](/docs/de/checkpointing), um Code und Gespräch von einem vorherigen Punkt wiederherzustellen oder zusammenzufassen                                                        |
| `Shift+Tab` oder `Alt+M` (einige Konfigurationen)             | Berechtigungsmodi umschalten                                                                                                                                                            | Wechseln Sie zwischen `default` (mit „Manual" im Modus-Indikator gekennzeichnet), `acceptEdits`, `plan` und allen Modi, die Sie aktiviert haben, z. B. `auto` oder `bypassPermissions`. Siehe [Berechtigungsmodi](/docs/de/permission-modes).                                                                                                                                        |
| `Option+P` (macOS) oder `Alt+P` (Windows/Linux)               | Modell wechseln                                                                                                                                                                         | Wechseln Sie Modelle, ohne Ihren Prompt zu löschen                                                                                                                                                                                                                                                                                                                              |
| `Option+T` (macOS) oder `Alt+T` (Windows/Linux)               | Extended Thinking umschalten                                                                                                                                                            | Aktivieren oder deaktivieren Sie den Extended Thinking-Modus. Hat keine Auswirkung auf Fable 5, das immer Extended Thinking verwendet. Ab v2.1.132 funktioniert dieses Kürzel auf macOS ohne Konfiguration von Option als Meta                                                                                                                                                  |
| `Option+O` (macOS) oder `Alt+O` (Windows/Linux)               | Schnellmodus umschalten                                                                                                                                                                 | Aktivieren oder deaktivieren Sie den [Schnellmodus](/docs/de/fast-mode)                                                                                                                                                                                                                                                                                                              |

<h3 id="text-editing">
  Textbearbeitung
</h3>

| Kürzel                  | Beschreibung                                     | Kontext                                                                                                                                                                                                                  |
| :---------------------- | :----------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                | Cursor an den Anfang der aktuellen Zeile bewegen | Bei mehrzeiliger Eingabe bewegt sich der Cursor an den Anfang der aktuellen logischen Zeile                                                                                                                              |
| `Ctrl+E`                | Cursor an das Ende der aktuellen Zeile bewegen   | Bei mehrzeiliger Eingabe bewegt sich der Cursor an das Ende der aktuellen logischen Zeile                                                                                                                                |
| `Ctrl+K`                | Bis zum Ende der Zeile löschen                   | Speichert gelöschten Text zum Einfügen                                                                                                                                                                                   |
| `Ctrl+U`                | Vom Cursor bis zum Zeilenanfang löschen          | Speichert gelöschten Text zum Einfügen. Wiederholen Sie, um über Zeilen in mehrzeiliger Eingabe zu löschen. Auf macOS ordnen Terminal-Emulatoren einschließlich iTerm2 und Terminal.app `Cmd+Backspace` diesem Kürzel zu |
| `Ctrl+W`                | Vorheriges Wort löschen                          | Speichert gelöschten Text zum Einfügen. Unter Windows löscht `Ctrl+Backspace` auch das vorherige Wort                                                                                                                    |
| `Ctrl+Y`                | Gelöschten Text einfügen                         | Fügen Sie Text ein, der mit `Ctrl+K`, `Ctrl+U` oder `Ctrl+W` gelöscht wurde                                                                                                                                              |
| `Alt+Y` (nach `Ctrl+Y`) | Einfügeverlauf durchlaufen                       | Nach dem Einfügen können Sie durch zuvor gelöschten Text navigieren. Erfordert [Option als Meta](#keyboard-shortcuts) auf macOS                                                                                          |
| `Alt+B`                 | Cursor um ein Wort nach hinten bewegen           | Wort-Navigation. Erfordert [Option als Meta](#keyboard-shortcuts) auf macOS                                                                                                                                              |
| `Alt+F`                 | Cursor um ein Wort nach vorne bewegen            | Wort-Navigation. Erfordert [Option als Meta](#keyboard-shortcuts) auf macOS                                                                                                                                              |

<h3 id="theme-and-display">
  Design und Anzeige
</h3>

| Kürzel   | Beschreibung                                   | Kontext                                                                                                  |
| :------- | :--------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| `Ctrl+T` | Syntax-Hervorhebung für Code-Blöcke umschalten | Funktioniert nur im `/theme`-Auswahlmenü. Steuert, ob Code in Claudes Antworten Syntax-Färbung verwendet |

<h3 id="multiline-input">
  Mehrzeilige Eingabe
</h3>

| Methode          | Kürzel          | Kontext                                                                                                    |
| :--------------- | :-------------- | :--------------------------------------------------------------------------------------------------------- |
| Schneller Escape | `\` + `Enter`   | Funktioniert in allen Terminals                                                                            |
| Option-Taste     | `Option+Enter`  | Nach Aktivierung von [Option als Meta](/docs/de/terminal-config#enable-option-key-shortcuts-on-macos) auf macOS |
| Shift+Enter      | `Shift+Enter`   | Nativ in iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal                           |
| Steuersequenz    | `Ctrl+J`        | Funktioniert in jedem Terminal ohne Konfiguration                                                          |
| Einfügemodus     | Direkt einfügen | Für Code-Blöcke, Protokolle                                                                                |

<Tip>
  Shift+Enter funktioniert ohne Konfiguration in iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal und Windows Terminal. Für VS Code, Cursor, Devin Desktop, Alacritty und Zed führen Sie `/terminal-setup` aus, um die Bindung zu installieren.
</Tip>

<h3 id="quick-commands">
  Schnellbefehle
</h3>

| Kürzel        | Beschreibung        | Notizen                                                                                                              |
| :------------ | :------------------ | :------------------------------------------------------------------------------------------------------------------- |
| `/` am Anfang | Befehl oder Skill   | Siehe [Befehle](#commands) und [Skills](/docs/de/skills)                                                                  |
| `!` am Anfang | Shell-Modus         | Führen Sie einen Befehl direkt aus, fügen Sie seine Ausgabe zur Sitzung hinzu und lassen Sie Claude darauf antworten |
| `@`           | Dateipfad-Erwähnung | Trigger für Dateipfad-Autovervollständigung                                                                          |

<h3 id="transcript-viewer">
  Transkript-Viewer
</h3>

Wenn der Transkript-Viewer offen ist (umgeschaltet mit `Ctrl+O`), sind diese Kürzel verfügbar. Im [Vollbildrendering](/docs/de/fullscreen) drücken Sie `?`, um das vollständige Kürzel-Referenzpanel im Viewer anzuzeigen. `Ctrl+E` kann über [`transcript:toggleShowAll`](/docs/de/keybindings) neu zugewiesen werden.

| Kürzel               | Beschreibung                                                                                                                                                                                                                                        |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Tastaturkürzel-Hilfepanel umschalten. Erfordert [Vollbildrendering](/docs/de/fullscreen)                                                                                                                                                                 |
| `{` / `}`            | Zur vorherigen oder nächsten Benutzereingabe springen, wie vim-Absatzbewegung. Erfordert [Vollbildrendering](/docs/de/fullscreen)                                                                                                                        |
| `Ctrl+E`             | Alle Inhalte anzeigen umschalten                                                                                                                                                                                                                    |
| `[`                  | Schreiben Sie das vollständige Gespräch in den nativen Scrollback Ihres Terminals, sodass `Cmd+F`, tmux-Kopiermodus und andere native Tools es durchsuchen können. Erfordert [Vollbildrendering](/docs/de/fullscreen#search-and-review-the-conversation) |
| `v`                  | Schreiben Sie das Gespräch in eine temporäre Datei und öffnen Sie es in `$VISUAL` oder `$EDITOR`. Erfordert [Vollbildrendering](/docs/de/fullscreen)                                                                                                     |
| `q`, `Ctrl+C`, `Esc` | Transkript-Ansicht beenden. Alle drei können über [`transcript:exit`](/docs/de/keybindings) neu zugewiesen werden                                                                                                                                        |

<h3 id="voice-input">
  Spracheingabe
</h3>

| Kürzel                     | Beschreibung     | Notizen                                                                                                                                                                                                                          |
| :------------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space` halten oder tippen | Sprach-Diktieren | Erfordert, dass [Sprach-Diktieren](/docs/de/voice-dictation) aktiviert ist. Halten Sie gedrückt zum Aufnehmen, oder führen Sie `/voice tap` aus für Tap-zum-Umschalten. [Neu zuweisbar](/docs/de/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Befehle
</h2>

Geben Sie `/` in Claude Code ein, um alle verfügbaren Befehle anzuzeigen, oder geben Sie `/` gefolgt von beliebigen Buchstaben ein, um zu filtern. Das `/`-Menü zeigt alles, was Sie aufrufen können: integrierte Befehle, gebündelte und von Benutzern erstellte [Skills](/docs/de/skills) sowie Befehle, die von [Plugins](/docs/de/plugins) und [MCP-Servern](/docs/de/mcp#use-mcp-prompts-as-commands) beigetragen werden. Nicht alle integrierten Befehle sind für jeden Benutzer sichtbar, da einige von Ihrer Plattform oder Ihrem Plan abhängen.

In der [Vollbildwiedergabe](/docs/de/fullscreen#use-the-mouse) reagieren die `/`-Befehlsliste und die `@`-Dateivorschlagsliste auch auf die Maus: Das Überfahren mit der Maus hebt eine Zeile hervor und das Anklicken akzeptiert sie.

Siehe die [Befehls-Referenz](/docs/de/commands) für die vollständige Liste der in Claude Code enthaltenen Befehle.

<h2 id="vim-editor-mode">
  Vim-Editor-Modus
</h2>

Aktivieren Sie Vim-ähnliche Bearbeitung über `/config` → Editor mode.

<h3 id="mode-switching">
  Modusumschaltung
</h3>

| Befehl | Aktion                                | Aus Modus      |
| :----- | :------------------------------------ | :------------- |
| `Esc`  | NORMAL-Modus eingeben                 | INSERT, VISUAL |
| `i`    | Vor Cursor einfügen                   | NORMAL         |
| `I`    | Am Anfang der Zeile einfügen          | NORMAL         |
| `a`    | Nach Cursor einfügen                  | NORMAL         |
| `A`    | Am Ende der Zeile einfügen            | NORMAL         |
| `o`    | Zeile unten öffnen                    | NORMAL         |
| `O`    | Zeile oben öffnen                     | NORMAL         |
| `v`    | Zeichenweise visuelle Auswahl starten | NORMAL         |
| `V`    | Zeilenweise visuelle Auswahl starten  | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  INSERT-Modus-Tastenfolgen neu zuordnen
</h3>

Die [`vimInsertModeRemaps`](/docs/de/settings#available-settings) Einstellung ordnet eine zweitastige INSERT-Modus-Sequenz Escape zu, sodass eine Zuordnung wie `jj` Sie in den NORMAL-Modus zurückbringt. Erfordert Claude Code v2.1.208 oder später.

Das folgende `~/.claude/settings.json` Beispiel aktiviert den Vim-Modus und ordnet `jj` Escape zu:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Jeder Schlüssel besteht aus genau zwei druckbaren Zeichen, die nacheinander eingegeben werden, und `"<Esc>"` ist das einzige unterstützte Ziel. Einträge mit einer anderen Länge oder einem anderen Ziel werden ignoriert.

Das Eingeben des ersten Zeichens einer Sequenz fügt es normal ein. Das Drücken des zweiten Zeichens innerhalb einer Sekunde entfernt das ausstehende Zeichen und wechselt in den NORMAL-Modus, wobei keines der beiden Zeichen in Ihrer Eingabe verbleibt. Nach dem Einsekundenfenster oder wenn ein anderes Zeichen folgt, bleiben beide Zeichen als Literaltext erhalten, sodass Sie ein Wort mit der Sequenz immer noch eingeben können, indem Sie zwischen den beiden Zeichen eine Pause machen.

Claude Code liest diese Einstellung aus Ihrer Benutzereinstellungsdatei, dem `--settings` Flag und [verwalteten Einstellungen](/docs/de/permissions#managed-settings) nur. Einträge in der `.claude/settings.json` oder `.claude/settings.local.json` eines Projekts werden ignoriert, sodass ein ausgechecktes Repository Ihre Tastenanschläge nicht neu zuordnen kann.

<h3 id="navigation-normal-mode">
  Navigation (NORMAL-Modus)
</h3>

| Befehl          | Aktion                                                                                                                                                                                       |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Nach links/unten/oben/rechts bewegen                                                                                                                                                         |
| `Space`         | Nach rechts bewegen                                                                                                                                                                          |
| `w`             | Nächstes Wort                                                                                                                                                                                |
| `e`             | Ende des Wortes                                                                                                                                                                              |
| `b`             | Vorheriges Wort                                                                                                                                                                              |
| `0`             | Anfang der Zeile                                                                                                                                                                             |
| `$`             | Ende der Zeile                                                                                                                                                                               |
| `^`             | Erstes Nicht-Leerzeichen-Zeichen                                                                                                                                                             |
| `gg`            | Anfang der Eingabe                                                                                                                                                                           |
| `G`             | Ende der Eingabe                                                                                                                                                                             |
| `f{char}`       | Zum nächsten Vorkommen des Zeichens springen                                                                                                                                                 |
| `F{char}`       | Zum vorherigen Vorkommen des Zeichens springen                                                                                                                                               |
| `t{char}`       | Direkt vor das nächste Vorkommen des Zeichens springen                                                                                                                                       |
| `T{char}`       | Direkt nach das vorherige Vorkommen des Zeichens springen                                                                                                                                    |
| `;`             | Letzte f/F/t/T-Bewegung wiederholen                                                                                                                                                          |
| `,`             | Letzte f/F/t/T-Bewegung in umgekehrter Reihenfolge wiederholen                                                                                                                               |
| `/`             | Umgekehrte Verlaufssuche öffnen, gleich wie `Ctrl+R`. Ab v2.1.191 zeigt die leere Suchleiste einen Hinweis an: Drücken Sie `Esc` dann `i` dann `/`, um stattdessen das Befehlsmenü zu öffnen |

<Note>
  Im Vim-Normal-Modus navigieren `j`/`k` und die Pfeiltasten den Befehlsverlauf, wenn sich der Cursor am Anfang oder Ende der Eingabe befindet und nicht weiter bewegt werden kann.
</Note>

<h3 id="editing-normal-mode">
  Bearbeitung (NORMAL-Modus)
</h3>

| Befehl         | Aktion                         |
| :------------- | :----------------------------- |
| `x`            | Zeichen löschen                |
| `dd`           | Zeile löschen                  |
| `D`            | Bis zum Ende der Zeile löschen |
| `dw`/`de`/`db` | Wort löschen/bis Ende/zurück   |
| `cc`           | Zeile ändern                   |
| `C`            | Bis zum Ende der Zeile ändern  |
| `cw`/`ce`/`cb` | Wort ändern/bis Ende/zurück    |
| `yy`/`Y`       | Zeile yanken (kopieren)        |
| `yw`/`ye`/`yb` | Wort yanken/bis Ende/zurück    |
| `p`            | Nach Cursor einfügen           |
| `P`            | Vor Cursor einfügen            |
| `>>`           | Zeile einrücken                |
| `<<`           | Zeile ausrücken                |
| `J`            | Zeilen verbinden               |
| `u`            | Rückgängig machen              |
| `.`            | Letzte Änderung wiederholen    |

<h3 id="text-objects-normal-mode">
  Textobjekte (NORMAL-Modus)
</h3>

Textobjekte funktionieren mit Operatoren wie `d`, `c` und `y`:

| Befehl    | Aktion                                 |
| :-------- | :------------------------------------- |
| `iw`/`aw` | Inneres/um Wort                        |
| `iW`/`aW` | Inneres/um WORT (Leerzeichen-begrenzt) |
| `i"`/`a"` | Inneres/um doppelte Anführungszeichen  |
| `i'`/`a'` | Inneres/um einfache Anführungszeichen  |
| `i(`/`a(` | Inneres/um Klammern                    |
| `i[`/`a[` | Inneres/um eckige Klammern             |
| `i{`/`a{` | Inneres/um geschweifte Klammern        |

<h3 id="visual-mode">
  Visueller Modus
</h3>

Drücken Sie `v` für zeichenweise Auswahl oder `V` für zeilenweise Auswahl. Bewegungen erweitern die Auswahl, und Operatoren wirken direkt darauf.

| Befehl           | Aktion                                                        |
| :--------------- | :------------------------------------------------------------ |
| `d`/`x`          | Auswahl löschen                                               |
| `y`              | Auswahl yanken                                                |
| `c`/`s`          | Auswahl ändern                                                |
| `p`              | Auswahl durch Registerinhalt ersetzen                         |
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

Claude Code verwaltet den Befehlsverlauf für die aktuelle Sitzung:

* Der Eingabeverlauf wird pro Arbeitsverzeichnis gespeichert
* Der Eingabeverlauf wird zurückgesetzt, wenn Sie `/clear` ausführen, um eine neue Sitzung zu starten. Das Gespräch der vorherigen Sitzung wird beibehalten und kann fortgesetzt werden.
* Das zweimalige Absenden derselben Eingabeaufforderung hintereinander zeichnet einen Verlaufseintrag auf, sodass das Drücken der Nach-oben-Taste zum vorherigen unterschiedlichen Befehl springt
* Verwenden Sie die Pfeiltasten nach oben/unten zum Navigieren (siehe Tastaturkürzel oben)
* Verlaufserweiterung mit `!` ist standardmäßig deaktiviert

<h3 id="reverse-search-with-ctrl-r">
  Reverse-Suche mit Ctrl+R
</h3>

Drücken Sie `Ctrl+R`, um interaktiv durch Ihren Befehlsverlauf zu suchen:

1. **Suche starten**: Drücken Sie `Ctrl+R`, um die Reverse-Verlaufssuche zu aktivieren
2. **Abfrage eingeben**: Geben Sie Text ein, um in vorherigen Befehlen zu suchen. Der Suchbegriff wird in übereinstimmenden Ergebnissen hervorgehoben
3. **Übereinstimmungen navigieren**: Drücken Sie `Ctrl+R` erneut, um durch ältere Übereinstimmungen zu navigieren
4. **Bereich ändern**: Die Suche bezieht sich standardmäßig auf Eingabeaufforderungen aus allen Projekten. Drücken Sie `Ctrl+S`, um den Bereich durch diese Sitzung, dieses Projekt und alle Projekte zu durchlaufen
5. **Übereinstimmung akzeptieren**:
   * Drücken Sie `Tab` oder `Esc`, um die aktuelle Übereinstimmung zu akzeptieren und die Bearbeitung fortzusetzen
   * Drücken Sie `Enter`, um die Übereinstimmung zu akzeptieren und den Befehl sofort auszuführen
6. **Suche abbrechen**:
   * Drücken Sie `Ctrl+C`, um abzubrechen und Ihre ursprüngliche Eingabe wiederherzustellen
   * Drücken Sie `Backspace` bei leerer Suche, um abzubrechen

Die Suche lädt die 100 neuesten eindeutigen Eingabeaufforderungen im ausgewählten Bereich, wobei Duplikate zum neuesten Vorkommen zusammengefasst werden. Übereinstimmende Eingabeaufforderungen werden mit dem hervorgehobenen Suchbegriff angezeigt, sodass Sie vorherige Eingaben finden und wiederverwenden können.

Das Akzeptieren einer Übereinstimmung oder das Abbrechen der Suche wird sofort wirksam, auch während Claude Code den Verlauf noch lädt. Vor v2.1.202 konnte das Akzeptieren oder Abbrechen während dieses Ladevorgangs einen internen Fehler melden.

<h2 id="background-bash-commands">
  Bash-Befehle im Hintergrund
</h2>

Claude Code unterstützt die Ausführung von Bash-Befehlen im Hintergrund, sodass Sie weiterarbeiten können, während lange laufende Prozesse ausgeführt werden.

<h3 id="how-backgrounding-works">
  Wie Hintergrund-Ausführung funktioniert
</h3>

Wenn Claude Code einen Befehl im Hintergrund ausführt, führt es den Befehl asynchron aus und gibt sofort eine Hintergrund-Task-ID zurück. Claude Code kann auf neue Prompts reagieren, während der Befehl weiterhin im Hintergrund ausgeführt wird.

Um Befehle im Hintergrund auszuführen, können Sie entweder:

* Claude Code auffordern, einen Befehl im Hintergrund auszuführen
* Drücken Sie `Ctrl+B`, um eine reguläre Bash-Tool-Invokation in den Hintergrund zu verschieben. Tmux-Benutzer müssen `Ctrl+B` zweimal drücken, da Tmux einen Präfix-Schlüssel hat.

**Wichtige Funktionen:**

* Die Ausgabe wird in eine Datei geschrieben und Claude kann sie mit dem Read-Tool abrufen
* Hintergrund-Tasks haben eindeutige IDs zum Tracking und zur Ausgabebeschaffung
* Hintergrund-Tasks werden automatisch bereinigt, wenn Claude Code beendet wird. Das Hintergrund-Ausführen der Sitzung anstelle des Beendens übergibt sie an die Hintergrund-Sitzung, wo sie weiterhin ausgeführt werden. Siehe [eine laufende Sitzung in den Hintergrund verschieben](/docs/de/agent-view#from-inside-a-session)
* Hintergrund-Tasks werden automatisch beendet, wenn die Ausgabe 5 GB überschreitet, mit einem Hinweis in stderr, der erklärt, warum
* Ab v2.1.193 werden auf macOS und Linux laufende Hintergrund-Tasks beendet, wenn das Betriebssystem ein Speicherdrucksignal sendet, sofern die Sitzung mindestens 30 Minuten untätig war, ohne dass ein Turn oder Subagent ausgeführt wird. Setzen Sie [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/de/env-vars) auf `1`, um dies auszuschalten

Um alle Hintergrund-Task-Funktionen zu deaktivieren, setzen Sie die Umgebungsvariable `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` auf `1`. Siehe [Umgebungsvariablen](/docs/de/env-vars) für Details.

**Häufig im Hintergrund ausgeführte Befehle:**

* Build-Tools (webpack, vite, make)
* Paketmanager (npm, yarn, pnpm)
* Test-Runner (jest, pytest)
* Entwicklungsserver
* Lange laufende Prozesse (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Bash-Modus mit `!`-Präfix
</h3>

Führen Sie Bash-Befehle direkt aus, ohne Claude zu durchlaufen, indem Sie Ihre Eingabe mit `!` präfixieren:

```bash theme={null}
! npm test
! git status
! ls -la
```

Bash-Modus:

* Fügt den Befehl und seine Ausgabe zum Gesprächskontext hinzu
* Zeigt Echtzeit-Fortschritt und Ausgabe
* Unterstützt die gleiche `Ctrl+B`-Hintergrund-Ausführung für lange laufende Befehle
* Erfordert nicht, dass Claude den Befehl interpretiert oder genehmigt
* Unterstützt verlaufsbasierte Autovervollständigung: Geben Sie einen Teilbefehl ein und drücken Sie `Tab`, um aus vorherigen `!`-Befehlen im aktuellen Projekt zu vervollständigen
* Unterstützt Live-Dateipfad-Autovervollständigung ab v2.1.193 auf allen Plattformen: Geben Sie ein Token mit einem Schrägstrich ein, z. B. `./src/` oder `~/`, um ein Dropdown-Menü mit übereinstimmenden Dateien und Verzeichnissen anzuzeigen, und drücken Sie dann `Tab`, um zu akzeptieren. Verwenden Sie auch unter Windows Schrägstriche; das Dropdown-Menü wird durch `/` ausgelöst, nicht durch `\`
* Beenden Sie mit `Escape`, `Backspace` oder `Ctrl+U` bei einer leeren Eingabeaufforderung
* Das Einfügen von Text, der mit `!` beginnt, in eine leere Eingabeaufforderung aktiviert automatisch den Bash-Modus und entspricht dem eingegebenen `!`-Verhalten

Ab v2.1.186 antwortet Claude automatisch auf die Befehlsausgabe, sobald sie im Transkript angezeigt wird, sodass Sie `! npm test` ausführen und eine Erklärung der Fehler ohne eine zweite Eingabeaufforderung erhalten können. Die Antwort kostet das Gleiche wie das Senden einer normalen Eingabeaufforderung. Um das frühere Verhalten wiederherzustellen, bei dem die Ausgabe zum Kontext hinzugefügt wird, ohne eine Antwort zu geben, setzen Sie [`respondToBashCommands`](/docs/de/settings#available-settings) auf `false` in `settings.json`. Vor v2.1.186 hat der Bash-Modus die Ausgabe immer zum Kontext hinzugefügt, ohne eine Antwort zu geben.

Dies ist nützlich für schnelle Shell-Operationen bei Beibehaltung des Gesprächskontexts.

<h2 id="prompt-suggestions">
  Prompt-Vorschläge
</h2>

Wenn Sie eine Sitzung zum ersten Mal öffnen, wird ein ausgegrautes Beispiel-Befehl in der Eingabeaufforderung angezeigt, um Ihnen den Einstieg zu erleichtern. Claude Code wählt dies aus dem Git-Verlauf Ihres Projekts aus, sodass es die Dateien widerspiegelt, an denen Sie kürzlich gearbeitet haben.

Nachdem Claude antwortet, werden weiterhin Vorschläge basierend auf Ihrem Gesprächsverlauf angezeigt, z. B. ein Folgenschritt aus einer mehrteiligen Anfrage oder eine natürliche Fortsetzung Ihres Workflows.

* Drücken Sie `Tab` oder `Rechts-Pfeil`, um den Vorschlag in die Eingabeaufforderung zu platzieren, und dann `Enter`, um einzureichen
* Beginnen Sie zu tippen, um ihn zu verwerfen

Der Vorschlag wird als Hintergrund-Anfrage ausgeführt, die den Prompt-Cache des übergeordneten Gesprächs wiederverwenden, sodass die zusätzlichen Kosten minimal sind. Claude Code überspringt die Vorschlagsgenerierung, wenn der Cache kalt ist, um unnötige Kosten zu vermeiden.

Vorschläge werden automatisch nach dem ersten Turn eines Gesprächs und im Plan Mode übersprungen. Im Print Mode sind sie standardmäßig deaktiviert. Übergeben Sie [`--prompt-suggestions`](/docs/de/cli-reference#cli-flags) mit `--output-format stream-json --verbose`, um stattdessen nach jedem Turn eine `prompt_suggestion`-Nachricht auszugeben.

Um Prompt-Vorschläge vollständig zu deaktivieren, setzen Sie die Umgebungsvariable oder schalten Sie die Einstellung in `/config` um:

```bash theme={null}
export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
```

<h2 id="side-questions-with-/btw">
  Nebenfragen mit /btw
</h2>

Verwenden Sie `/btw`, um eine schnelle Frage zu Ihrer aktuellen Arbeit zu stellen, ohne sie zum Gesprächsverlauf hinzuzufügen. Dies ist nützlich, wenn Sie eine schnelle Antwort möchten, aber nicht den Hauptkontext unordentlich machen oder Claude von einer lange laufenden Aufgabe ablenken möchten.

```
/btw what was the name of that config file again?
```

Nebenfragen haben vollständige Sichtbarkeit des aktuellen Gesprächs, sodass Sie Fragen zu Code stellen können, den Claude bereits gelesen hat, Entscheidungen, die es früher getroffen hat, oder alles andere aus der Sitzung. Die Frage und Antwort sind flüchtig: Sie erscheinen in einer verwerfbaren Überlagerung und gelangen niemals in den Gesprächsverlauf.

* **Verfügbar während Claude arbeitet**: Sie können `/btw` auch ausführen, während Claude eine Antwort verarbeitet. Die Nebenfrage wird unabhängig ausgeführt und unterbricht den Hauptturn nicht.
* **Kein Tool-Zugriff**: Nebenfragen beantworten nur aus dem, was bereits im Kontext ist. Claude kann keine Dateien lesen, Befehle ausführen oder suchen, wenn eine Nebenfrage beantwortet wird.
* **Einzelne Antwort**: Es gibt keine Folgeversuche in der Überlagerung. Um den Thread fortzusetzen, teilen Sie ihn mit `f` in seine eigene Sitzung auf.
* **Niedrige Kosten**: Die Nebenfrage verwendet den Prompt-Cache des übergeordneten Gesprächs wieder, sodass die zusätzlichen Kosten minimal sind.

Frühere Nebenfragen aus derselben Sitzung werden als abgeblendete Liste über der aktuellen Antwort angezeigt. Sie bleiben außerhalb des Gesprächsverlaufs, bleiben aber in der Überlagerung sichtbar, bis Sie sie löschen.

Sobald die Antwort angezeigt wird, akzeptiert die Überlagerung diese Tasten.

| Taste                      | Aktion                                                                                                                                                                                                                                                                                                              |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space`, `Enter`, `Escape` | Verwerfen Sie die Antwort und kehren Sie zur Eingabeaufforderung zurück                                                                                                                                                                                                                                             |
| `Up` / `Down`              | Scrollen Sie die Antwort                                                                                                                                                                                                                                                                                            |
| `Left` / `Right`           | Wechseln Sie zwischen dieser Antwort und Ihren früheren `/btw`-Antworten aus der Sitzung. `Left` wechselt zu älteren Antworten und `Right` kehrt zur aktuellen zurück. Erfordert Claude Code v2.1.187 oder später                                                                                                   |
| `c`                        | Kopieren Sie die Antwort als Raw Markdown in Ihre Zwischenablage. Verwenden Sie dies anstelle der Mausauswahl, die das hart umgebrochene Terminal-Rendering anstelle des Quelltexts erfasst                                                                                                                         |
| `f`                        | In eine neue Sitzung aufteilen. Die Aufteilung erbt das übergeordnete Gespräch plus diese Frage und Antwort als echte Transkript-Turns, sodass Sie mit vollständigem Tool-Zugriff fortfahren können. Die ursprüngliche Sitzung wird unter [`/resume`](/docs/de/commands) beibehalten. Nur in lokalen Sitzungen verfügbar |
| `x`                        | Löschen Sie die Liste der früheren `/btw`-Austausche, die über der aktuellen Antwort angezeigt werden                                                                                                                                                                                                               |

`/btw` ist das Gegenteil eines [Subagenten](/docs/de/sub-agents): Es sieht Ihr vollständiges Gespräch, hat aber keine Tools, während ein Subagent vollständige Tools hat, aber mit einem leeren Kontext beginnt. Verwenden Sie `/btw`, um zu fragen, was Claude bereits aus dieser Sitzung weiß; verwenden Sie einen Subagenten, um etwas Neues herauszufinden.

<h2 id="task-list">
  Task-Liste
</h2>

Die Task-Liste ist Claudes Aufgabenliste: Elemente, die Claude erstellt hat, um mehrstufige Arbeiten zu planen, mit Indikatoren, die zeigen, was ausstehend, in Bearbeitung oder abgeschlossen ist. Sie ist separat von der Hintergrund-Task-Ansicht. Um laufende Shells und Subagenten zu sehen, verwenden Sie stattdessen [`/tasks`](/docs/de/commands).

* Drücken Sie `Ctrl+T`, um die Task-Listen-Ansicht umzuschalten. Die Anzeige zeigt bis zu fünf Tasks gleichzeitig. Wenn Claude noch keine Checklistenelemente erstellt hat, hat der Umschalter keine sichtbare Auswirkung, da es nichts anzuzeigen gibt
* Um alle Tasks anzuzeigen oder zu löschen, fragen Sie Claude direkt: "show me all tasks" oder "clear all tasks"
* Tasks bleiben über Kontext-Kompaktionen hinweg bestehen und helfen Claude, bei größeren Projekten organisiert zu bleiben
* Um eine Task-Liste über Sitzungen hinweg zu teilen, setzen Sie `CLAUDE_CODE_TASK_LIST_ID`, um ein benanntes Verzeichnis in `~/.claude/tasks/` zu verwenden: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Sitzungs-Zusammenfassung
</h2>

Wenn Sie zum Terminal zurückkehren, nachdem Sie sich entfernt haben, zeigt Claude Code eine einzeilige Zusammenfassung dessen an, was bisher in der Sitzung passiert ist. Die Zusammenfassung wird im Hintergrund generiert, sobald mindestens drei Minuten seit dem letzten abgeschlossenen Turn vergangen sind und das Terminal nicht fokussiert ist, sodass sie bereit ist, wenn Sie zurückwechseln. Zusammenfassungen erscheinen nur, wenn die Sitzung mindestens drei Turns hat, und nie zweimal hintereinander.

Führen Sie `/recap` aus, um eine Zusammenfassung auf Anfrage zu generieren. Um automatische Zusammenfassungen auszuschalten, öffnen Sie `/config` und deaktivieren Sie **Session recap**.

Die Sitzungs-Zusammenfassung ist standardmäßig für jeden Plan und Provider aktiviert. Die Zusammenfassung wird im nicht-interaktiven Modus immer übersprungen.

<h2 id="pr-review-status">
  PR-Review-Status
</h2>

Bei der Arbeit an einem Branch mit einem offenen Pull Request zeigt Claude Code einen anklickbaren PR-Link in der Fußzeile an, z. B. „PR #446". Der Link hat eine farbige Unterstreichung, die den Review-Status anzeigt:

* Grün: genehmigt
* Gelb: Review ausstehend
* Rot: Änderungen angefordert
* Grau: Entwurf

Das Badge verschwindet, sobald der Pull Request zusammengeführt oder geschlossen wird. `Cmd+click` (macOS) oder `Ctrl+click` (Windows/Linux) auf den Link, um den Pull Request in Ihrem Browser zu öffnen. Der Status wird alle 60 Sekunden aktualisiert und sofort nach der Ausführung eines `gh pr`- oder `git push`-Befehls in der Sitzung.

<Note>
  Der PR-Status erfordert, dass die `gh` CLI installiert und authentifiziert ist (`gh auth login`).
</Note>

<h2 id="see-also">
  Siehe auch
</h2>

* [Skills](/docs/de/skills) - Benutzerdefinierte Prompts und Workflows
* [Checkpointing](/docs/de/checkpointing) - Spulen Sie Claudes Änderungen zurück und stellen Sie vorherige Zustände wieder her
* [CLI-Referenz](/docs/de/cli-reference) - Befehlszeilenflags und Optionen
* [Einstellungen](/docs/de/settings) - Konfigurationsoptionen
* [Speicherverwaltung](/docs/de/memory) - Verwalten von CLAUDE.md-Dateien
