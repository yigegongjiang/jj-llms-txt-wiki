> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tastaturkürzel anpassen

> Passen Sie Tastaturkürzel in Claude Code mit einer Keybindings-Konfigurationsdatei an.

Claude Code unterstützt anpassbare Tastaturkürzel. Führen Sie `/keybindings` aus, um Ihre Konfigurationsdatei unter `~/.claude/keybindings.json` zu erstellen oder zu öffnen.

<h2 id="configuration-file">
  Konfigurationsdatei
</h2>

Die Keybindings-Konfigurationsdatei ist ein Objekt mit einem `bindings`-Array. Jeder Block gibt einen Kontext und eine Zuordnung von Tastenkombinationen zu Aktionen an.

<Note>Änderungen an der Keybindings-Datei werden automatisch erkannt und angewendet, ohne Claude Code neu zu starten.</Note>

| Feld       | Beschreibung                                               |
| :--------- | :--------------------------------------------------------- |
| `$schema`  | Optionale JSON-Schema-URL für Editor-Autovervollständigung |
| `$docs`    | Optionale Dokumentations-URL                               |
| `bindings` | Array von Binding-Blöcken nach Kontext                     |

Dieses Beispiel bindet `Ctrl+E` zum Öffnen eines externen Editors im Chat-Kontext und hebt die Bindung von `Ctrl+U` auf:

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/en/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  Kontexte
</h2>

Jeder Binding-Block gibt einen **Kontext** an, in dem die Bindings gelten:

| Kontext           | Beschreibung                                              |
| :---------------- | :-------------------------------------------------------- |
| `Global`          | Gilt überall in der App                                   |
| `Chat`            | Haupteingabebereich für Chat                              |
| `Autocomplete`    | Autovervollständigungsmenü ist offen                      |
| `Settings`        | Einstellungsmenü                                          |
| `Confirmation`    | Berechtigungs- und Bestätigungsdialoge                    |
| `Tabs`            | Tab-Navigationskomponenten                                |
| `Help`            | Hilfemenü ist sichtbar                                    |
| `Transcript`      | Transkript-Viewer                                         |
| `HistorySearch`   | Verlaufssuchmodus (Ctrl+R)                                |
| `Task`            | Hintergrundaufgabe wird ausgeführt                        |
| `ThemePicker`     | Design-Picker-Dialog                                      |
| `Attachments`     | Bildanhang-Navigation in Auswahldialogen                  |
| `Footer`          | Fußzeilen-Indikator-Navigation (Aufgaben, Teams, Diff)    |
| `MessageSelector` | Nachrichtenauswahl für Rewind- und Zusammenfassungsdialog |
| `DiffDialog`      | Diff-Viewer-Navigation                                    |
| `ModelPicker`     | Modell-Picker-Aufwandsstufe                               |
| `Select`          | Generische Select/List-Komponenten                        |
| `Plugin`          | Plugin-Dialog (durchsuchen, entdecken, verwalten)         |
| `Scroll`          | Konversations-Scrolling und Textauswahl im Vollbildmodus  |

Vor v2.1.205 existierten ein `Doctor`-Kontext und eine `doctor:fix`-Aktion für den `/doctor`-Diagnose-Bildschirm.

<h2 id="available-actions">
  Verfügbare Aktionen
</h2>

Aktionen folgen einem `namespace:action`-Format, wie `chat:submit` zum Senden einer Nachricht oder `app:toggleTodos` zum Anzeigen der Aufgabenliste. Jeder Kontext hat spezifische verfügbare Aktionen.

<h3 id="app-actions">
  App-Aktionen
</h3>

Aktionen verfügbar im `Global`-Kontext:

| Aktion                 | Standard         | Beschreibung                                                                                                                  |
| :--------------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C           | Aktuelle Operation abbrechen                                                                                                  |
| `app:exit`             | Ctrl+D           | Claude Code beenden                                                                                                           |
| `app:redraw`           | (nicht gebunden) | Bildschirm neu zeichnen erzwingen                                                                                             |
| `app:toggleTodos`      | Ctrl+T           | Sichtbarkeit der Aufgabenliste von Claude umschalten. Dies ist nicht die [`/tasks`](/docs/de/commands) Hintergrund-Aufgabenansicht |
| `app:toggleTranscript` | Ctrl+O           | Ausführliches Transkript umschalten                                                                                           |

<h3 id="history-actions">
  Verlaufsaktionen
</h3>

Aktionen zum Navigieren im Befehlsverlauf:

| Aktion             | Standard | Beschreibung               |
| :----------------- | :------- | :------------------------- |
| `history:search`   | Ctrl+R   | Verlaufssuche öffnen       |
| `history:previous` | Oben     | Vorheriges Verlaufselement |
| `history:next`     | Unten    | Nächstes Verlaufselement   |

<h3 id="chat-actions">
  Chat-Aktionen
</h3>

Aktionen verfügbar im `Chat`-Kontext:

| Aktion                | Standard                             | Beschreibung                                                                                                                                                                                            |
| :-------------------- | :----------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `chat:cancel`         | Escape                               | Aktuelle Eingabe abbrechen                                                                                                                                                                              |
| `chat:clearInput`     | Ctrl+L                               | Vollständiges Bildschirm-Neuzeichnen erzwingen, Eingabe beibehalten. Im [Vollbildrendering](/docs/de/fullscreen#clear-the-conversation) zweimal innerhalb von zwei Sekunden drücken, um `/clear` auszuführen |
| `chat:clearScreen`    | Cmd+K                                | Im [Vollbildrendering](/docs/de/fullscreen#clear-the-conversation) zweimal innerhalb von zwei Sekunden drücken, um `/clear` auszuführen                                                                      |
| `chat:killAgents`     | Ctrl+X Ctrl+K                        | Alle laufenden [Hintergrund-Subagenten](/docs/de/sub-agents#run-subagents-in-foreground-or-background) in dieser Sitzung beenden                                                                             |
| `chat:cycleMode`      | Shift+Tab\*                          | Berechtigungsmodi durchlaufen                                                                                                                                                                           |
| `chat:modelPicker`    | Meta+P                               | Modell-Picker öffnen                                                                                                                                                                                    |
| `chat:fastMode`       | Meta+O                               | Schnellmodus umschalten                                                                                                                                                                                 |
| `chat:thinkingToggle` | Meta+T                               | Erweitertes Denken umschalten                                                                                                                                                                           |
| `chat:submit`         | Enter                                | Nachricht senden                                                                                                                                                                                        |
| `chat:newline`        | Ctrl+J                               | Zeilenumbruch einfügen, ohne zu senden                                                                                                                                                                  |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-                | Letzte Aktion rückgängig machen                                                                                                                                                                         |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E                | In externem Editor öffnen                                                                                                                                                                               |
| `chat:stash`          | Ctrl+S                               | Aktuelle Eingabeaufforderung speichern                                                                                                                                                                  |
| `chat:imagePaste`     | Ctrl+V (Alt+V unter Windows und WSL) | Bild aus der Zwischenablage einfügen. Unter WSL sind beide Tastenkombinationen standardmäßig gebunden                                                                                                   |

\*Unter Windows ohne VT-Modus (Node \<24.2.0/\<22.17.0, Bun \<1.2.23) Standard auf Meta+M.

<h3 id="autocomplete-actions">
  Autovervollständigungsaktionen
</h3>

Aktionen verfügbar im `Autocomplete`-Kontext:

| Aktion                  | Standard | Beschreibung          |
| :---------------------- | :------- | :-------------------- |
| `autocomplete:accept`   | Tab      | Vorschlag akzeptieren |
| `autocomplete:dismiss`  | Escape   | Menü schließen        |
| `autocomplete:previous` | Oben     | Vorheriger Vorschlag  |
| `autocomplete:next`     | Unten    | Nächster Vorschlag    |

<h3 id="confirmation-actions">
  Bestätigungsaktionen
</h3>

Aktionen verfügbar im `Confirmation`-Kontext:

| Aktion                      | Standard         | Beschreibung                                                                                                                                |
| :-------------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `confirm:yes`               | Y, Enter         | Aktion bestätigen                                                                                                                           |
| `confirm:no`                | N, Escape        | Aktion ablehnen                                                                                                                             |
| `confirm:previous`          | Oben             | Vorherige Option                                                                                                                            |
| `confirm:next`              | Unten            | Nächste Option                                                                                                                              |
| `confirm:nextField`         | Tab              | Nächstes Feld                                                                                                                               |
| `confirm:previousField`     | (nicht gebunden) | Vorheriges Feld                                                                                                                             |
| `confirm:toggle`            | Leertaste        | Auswahl umschalten                                                                                                                          |
| `confirm:cycleMode`         | Shift+Tab        | Berechtigungsmodi durchlaufen                                                                                                               |
| `confirm:toggleExplanation` | Ctrl+E           | Modellgenerierte [Erklärung des Befehls](/docs/de/permissions#permission-system) auf Bash- und PowerShell-Berechtigungsaufforderungen umschalten |

<h3 id="permission-actions">
  Berechtigungsaktionen
</h3>

Aktionen verfügbar im `Confirmation`-Kontext für Berechtigungsdialoge:

| Aktion                   | Standard         | Beschreibung                                                                                                                    |
| :----------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `permission:toggleDebug` | (nicht gebunden) | Berechtigungs-Debug-Info umschalten. Der vorherige Standard von Ctrl+D wurde in v2.1.146 entfernt, da er `app:exit` überlagerte |

<h3 id="transcript-actions">
  Transkript-Aktionen
</h3>

Aktionen verfügbar im `Transcript`-Kontext:

| Aktion                     | Standard          | Beschreibung                     |
| :------------------------- | :---------------- | :------------------------------- |
| `transcript:toggleShowAll` | Ctrl+E            | Alle Inhalte anzeigen umschalten |
| `transcript:exit`          | q, Ctrl+C, Escape | Transkript-Ansicht beenden       |

<h3 id="history-search-actions">
  Verlaufssuch-Aktionen
</h3>

Aktionen verfügbar im `HistorySearch`-Kontext:

| Aktion                     | Standard    | Beschreibung                                   |
| :------------------------- | :---------- | :--------------------------------------------- |
| `historySearch:next`       | Ctrl+R      | Nächster Treffer                               |
| `historySearch:accept`     | Escape, Tab | Auswahl akzeptieren                            |
| `historySearch:cancel`     | Ctrl+C      | Suche abbrechen                                |
| `historySearch:execute`    | Enter       | Ausgewählten Befehl ausführen                  |
| `historySearch:cycleScope` | Ctrl+S      | Bereich durchlaufen: Sitzung, Projekt, überall |

<h3 id="task-actions">
  Aufgaben-Aktionen
</h3>

Aktionen verfügbar im `Task`-Kontext:

| Aktion            | Standard              | Beschreibung                                                                                                                                         |
| :---------------- | :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Aktuelle Aufgabe in den Hintergrund verschieben. Die Ctrl+X Ctrl+B-Kombination erfordert v2.1.169 oder später und vermeidet den tmux-Präfix-Konflikt |

<h3 id="theme-actions">
  Design-Aktionen
</h3>

Aktionen verfügbar im `ThemePicker`-Kontext:

| Aktion                           | Standard | Beschreibung                  |
| :------------------------------- | :------- | :---------------------------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T   | Syntaxhervorhebung umschalten |

<h3 id="help-actions">
  Hilfe-Aktionen
</h3>

Aktionen verfügbar im `Help`-Kontext:

| Aktion         | Standard | Beschreibung        |
| :------------- | :------- | :------------------ |
| `help:dismiss` | Escape   | Hilfemenü schließen |

<h3 id="tabs-actions">
  Tabs-Aktionen
</h3>

Aktionen verfügbar im `Tabs`-Kontext:

| Aktion          | Standard         | Beschreibung   |
| :-------------- | :--------------- | :------------- |
| `tabs:next`     | Tab, Rechts      | Nächster Tab   |
| `tabs:previous` | Shift+Tab, Links | Vorheriger Tab |

<h3 id="attachments-actions">
  Anhänge-Aktionen
</h3>

Aktionen verfügbar im `Attachments`-Kontext:

| Aktion                 | Standard           | Beschreibung                  |
| :--------------------- | :----------------- | :---------------------------- |
| `attachments:next`     | Rechts             | Nächster Anhang               |
| `attachments:previous` | Links              | Vorheriger Anhang             |
| `attachments:remove`   | Rücktaste, Löschen | Ausgewählten Anhang entfernen |
| `attachments:exit`     | Unten, Escape      | Anhang-Navigation beenden     |

<h3 id="footer-actions">
  Fußzeilen-Aktionen
</h3>

Aktionen verfügbar im `Footer`-Kontext:

| Aktion                  | Standard | Beschreibung                                                 |
| :---------------------- | :------- | :----------------------------------------------------------- |
| `footer:next`           | Rechts   | Nächstes Fußzeilen-Element                                   |
| `footer:previous`       | Links    | Vorheriges Fußzeilen-Element                                 |
| `footer:up`             | Oben     | In der Fußzeile nach oben navigieren (Auswahl oben aufheben) |
| `footer:down`           | Unten    | In der Fußzeile nach unten navigieren                        |
| `footer:openSelected`   | Enter    | Ausgewähltes Fußzeilen-Element öffnen                        |
| `footer:clearSelection` | Escape   | Fußzeilen-Auswahl löschen                                    |

<h3 id="message-selector-actions">
  Nachrichtenauswahl-Aktionen
</h3>

Aktionen verfügbar im `MessageSelector`-Kontext:

| Aktion                   | Standard                                     | Beschreibung                    |
| :----------------------- | :------------------------------------------- | :------------------------------ |
| `messageSelector:up`     | Oben, K, Ctrl+P                              | In der Liste nach oben bewegen  |
| `messageSelector:down`   | Unten, J, Ctrl+N                             | In der Liste nach unten bewegen |
| `messageSelector:top`    | Ctrl+Oben, Shift+Oben, Meta+Oben, Shift+K    | Zum Anfang springen             |
| `messageSelector:bottom` | Ctrl+Unten, Shift+Unten, Meta+Unten, Shift+J | Zum Ende springen               |
| `messageSelector:select` | Enter                                        | Nachricht auswählen             |

<h3 id="diff-actions">
  Diff-Aktionen
</h3>

Aktionen verfügbar im `DiffDialog`-Kontext:

| Aktion                | Standard         | Beschreibung                                                                                                                                                         |
| :-------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `diff:dismiss`        | Escape           | Diff-Viewer schließen; aus der Detailansicht zurück zur Dateiliste                                                                                                   |
| `diff:previousSource` | Links            | Vorherige Diff-Quelle                                                                                                                                                |
| `diff:nextSource`     | Rechts           | Nächste Diff-Quelle                                                                                                                                                  |
| `diff:previousFile`   | Oben, K          | Vorherige Datei in der Dateiliste; eine Zeile nach oben in der Detailansicht scrollen                                                                                |
| `diff:nextFile`       | Unten, J         | Nächste Datei in der Dateiliste; eine Zeile nach unten in der Detailansicht scrollen                                                                                 |
| `diff:viewDetails`    | Enter            | Diff-Details anzeigen                                                                                                                                                |
| `diff:back`           | (nicht gebunden) | Im Diff-Viewer zurückgehen. Escape führt die Zurück-Aktion über `diff:dismiss` aus. Der vorherige Standard von Links in der Detailansicht wurde in v2.1.203 entfernt |

Die Detailansicht des Diff bindet auch Pager-ähnliche Tasten an die Standard-[Scroll-Aktionen](#scroll-actions). Diese Bindungen sind Teil des `DiffDialog`-Kontexts und gelten nur in der Detailansicht; die `Scroll`-Kontext-Standards, die unter [Scroll-Aktionen](#scroll-actions) aufgelistet sind, bleiben unverändert.

| Aktion                | Standard           | Beschreibung                            |
| :-------------------- | :----------------- | :-------------------------------------- |
| `scroll:pageUp`       | Bild-Auf           | Halbe Viewport-Höhe nach oben scrollen  |
| `scroll:pageDown`     | Bild-Ab            | Halbe Viewport-Höhe nach unten scrollen |
| `scroll:fullPageUp`   | Shift+Leertaste, B | Volle Viewport-Höhe nach oben scrollen  |
| `scroll:fullPageDown` | Leertaste          | Volle Viewport-Höhe nach unten scrollen |
| `scroll:top`          | G, Pos1            | Zum Anfang springen                     |
| `scroll:bottom`       | Shift+G, Ende      | Zum Ende springen                       |

<h3 id="model-picker-actions">
  Modell-Picker-Aktionen
</h3>

Aktionen verfügbar im `ModelPicker`-Kontext:

| Aktion                        | Standard | Beschreibung                                          |
| :---------------------------- | :------- | :---------------------------------------------------- |
| `modelPicker:decreaseEffort`  | Links    | Aufwandsstufe verringern                              |
| `modelPicker:increaseEffort`  | Rechts   | Aufwandsstufe erhöhen                                 |
| `modelPicker:thisSessionOnly` | s        | Hervorgehobenes Modell nur auf diese Sitzung anwenden |

<h3 id="select-actions">
  Select-Aktionen
</h3>

Aktionen verfügbar im `Select`-Kontext:

| Aktion            | Standard         | Beschreibung        |
| :---------------- | :--------------- | :------------------ |
| `select:next`     | Unten, J, Ctrl+N | Nächste Option      |
| `select:previous` | Oben, K, Ctrl+P  | Vorherige Option    |
| `select:accept`   | Enter            | Auswahl akzeptieren |
| `select:cancel`   | Escape           | Auswahl abbrechen   |

<h3 id="plugin-actions">
  Plugin-Aktionen
</h3>

Aktionen verfügbar im `Plugin`-Kontext:

| Aktion            | Standard  | Beschreibung                                                                                               |
| :---------------- | :-------- | :--------------------------------------------------------------------------------------------------------- |
| `plugin:toggle`   | Leertaste | Plugin-Auswahl umschalten                                                                                  |
| `plugin:install`  | I         | Ausgewählte Plugins installieren                                                                           |
| `plugin:favorite` | F         | Ausgewähltes Plugin als Favorit markieren, damit es oben auf der Registerkarte „Installiert" sortiert wird |

<h3 id="settings-actions">
  Einstellungs-Aktionen
</h3>

Aktionen verfügbar im `Settings`-Kontext. Die Aktionen `select:accept` und `confirm:no` werden aus den Kontexten [Select](#select-actions) und [Confirmation](#confirmation-actions) wiederverwendet mit einstellungsspezifischem Verhalten: Änderungen werden auf jede Einstellung angewendet, sobald Sie sie ändern, daher schließt Escape das Panel mit Ihren gespeicherten Änderungen, anstatt abzulehnen.

| Aktion            | Standard         | Beschreibung                                                         |
| :---------------- | :--------------- | :------------------------------------------------------------------- |
| `settings:search` | /                | Suchmodus aktivieren                                                 |
| `settings:retry`  | R                | Nutzungsdaten neu laden (bei Fehler)                                 |
| `select:accept`   | Enter, Leertaste | Ändern Sie die ausgewählte Einstellung oder öffnen Sie das Untermenü |
| `confirm:no`      | Escape           | Schließen Sie das Panel. Änderungen sind bereits gespeichert         |

<h3 id="voice-actions">
  Sprach-Aktionen
</h3>

Aktionen verfügbar im `Chat`-Kontext, wenn [Sprachdiktat](/docs/de/voice-dictation) aktiviert ist:

| Aktion             | Standard  | Beschreibung                                                             |
| :----------------- | :-------- | :----------------------------------------------------------------------- |
| `voice:pushToTalk` | Leertaste | Eingabeaufforderung diktieren. Halten oder tippen je nach `/voice`-Modus |

<h3 id="scroll-actions">
  Scroll-Aktionen
</h3>

Aktionen verfügbar im `Scroll`-Kontext, wenn [Vollbildrendering](/docs/de/fullscreen) aktiviert ist:

| Aktion                      | Standard             | Beschreibung                                                                                                                   |
| :-------------------------- | :------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `scroll:lineUp`             | (nicht gebunden)     | Eine Zeile nach oben scrollen. Mausrad-Scrolling löst diese Aktion aus                                                         |
| `scroll:lineDown`           | (nicht gebunden)     | Eine Zeile nach unten scrollen. Mausrad-Scrolling löst diese Aktion aus                                                        |
| `scroll:pageUp`             | Bild-Auf             | Halbe Viewport-Höhe nach oben scrollen                                                                                         |
| `scroll:pageDown`           | Bild-Ab              | Halbe Viewport-Höhe nach unten scrollen                                                                                        |
| `scroll:top`                | Ctrl+Pos1            | Zum Anfang der Konversation springen                                                                                           |
| `scroll:bottom`             | Ctrl+Ende            | Zur neuesten Nachricht springen und Auto-Follow erneut aktivieren                                                              |
| `scroll:halfPageUp`         | (nicht gebunden)     | Halbe Viewport-Höhe nach oben scrollen. Gleiches Verhalten wie `scroll:pageUp`, bereitgestellt für Vi-ähnliche Neubindungen    |
| `scroll:halfPageDown`       | (nicht gebunden)     | Halbe Viewport-Höhe nach unten scrollen. Gleiches Verhalten wie `scroll:pageDown`, bereitgestellt für Vi-ähnliche Neubindungen |
| `scroll:fullPageUp`         | (nicht gebunden)     | Volle Viewport-Höhe nach oben scrollen                                                                                         |
| `scroll:fullPageDown`       | (nicht gebunden)     | Volle Viewport-Höhe nach unten scrollen                                                                                        |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | Ausgewählten Text in die Zwischenablage kopieren                                                                               |
| `selection:clear`           | (nicht gebunden)     | Aktive Textauswahl löschen                                                                                                     |
| `selection:extendLeft`      | Shift+Links          | Aktive Auswahl eine Spalte nach links erweitern                                                                                |
| `selection:extendRight`     | Shift+Rechts         | Aktive Auswahl eine Spalte nach rechts erweitern                                                                               |
| `selection:extendUp`        | Shift+Oben           | Aktive Auswahl eine Zeile nach oben erweitern. Scrollt den Viewport, wenn die Auswahl die obere Kante erreicht                 |
| `selection:extendDown`      | Shift+Unten          | Aktive Auswahl eine Zeile nach unten erweitern. Scrollt den Viewport, wenn die Auswahl die untere Kante erreicht               |
| `selection:extendLineStart` | Shift+Pos1           | Aktive Auswahl zum Anfang der Zeile erweitern                                                                                  |
| `selection:extendLineEnd`   | Shift+Ende           | Aktive Auswahl zum Ende der Zeile erweitern                                                                                    |

<h2 id="keystroke-syntax">
  Tastenkombinations-Syntax
</h2>

<h3 id="modifiers">
  Modifizierer
</h3>

Verwenden Sie Modifizierer-Tasten mit dem `+`-Trennzeichen:

* `ctrl` oder `control` - Strg-Taste
* `shift` - Umschalt-Taste
* `alt`, `opt`, `option` oder `meta` - Alt-Taste unter Windows und Linux, Option-Taste unter macOS
* `cmd`, `command`, `super` oder `win` - Befehlstaste unter macOS, Windows-Taste unter Windows, Super-Taste unter Linux

Die `cmd`-Gruppe wird nur in Terminals erkannt, die den Super-Modifizierer melden, wie z. B. solche, die das Kitty-Tastaturprotokoll oder den `modifyOtherKeys`-Modus von xterm unterstützen. Die meisten Terminals senden ihn nicht, daher verwenden Sie `ctrl` oder `meta` für Bindungen, die überall funktionieren sollen.

Beispiele:

```text theme={null}
ctrl+k          Strg + K
shift+tab       Umschalt + Tab
meta+p          Option + P unter macOS, Alt + P anderswo
ctrl+shift+c    Mehrere Modifizierer
```

<h3 id="uppercase-letters">
  Großbuchstaben
</h3>

Ein eigenständiger Großbuchstabe impliziert Umschalt. Zum Beispiel ist `K` gleichbedeutend mit `shift+k`. Dies ist nützlich für Vim-ähnliche Bindungen, bei denen Groß- und Kleinbuchstaben unterschiedliche Bedeutungen haben.

Großbuchstaben mit Modifizierern (z. B. `ctrl+K`) werden als stilistisch behandelt und implizieren **nicht** Umschalt: `ctrl+K` ist dasselbe wie `ctrl+k`.

<h3 id="chords">
  Akkorde
</h3>

Akkorde sind Sequenzen von Tastenkombinationen, die durch Leerzeichen getrennt sind:

```text theme={null}
ctrl+k ctrl+s   Drücken Sie Strg+K, loslassen, dann Strg+S
```

<h3 id="special-keys">
  Spezielle Tasten
</h3>

* `escape` oder `esc` - Escape-Taste
* `enter` oder `return` - Enter-Taste
* `tab` - Tab-Taste
* `space` - Leertaste
* `up`, `down`, `left`, `right` - Pfeiltasten
* `backspace`, `delete` - Löschtasten

<h2 id="unbind-default-shortcuts">
  Standardkürzel aufheben
</h2>

Setzen Sie eine Aktion auf `null`, um ein Standardkürzel aufzuheben:

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

Dies funktioniert auch für Akkord-Bindings. Das Aufheben aller Akkorde, die ein Präfix teilen, gibt dieses Präfix für die Verwendung als Single-Key-Binding frei. Ein Akkord in einem beliebigen aktiven Kontext behält sein Präfix reserviert, daher müssen Sie jeden Akkord in dem Kontext aufheben, der ihn definiert.

Die Standard-`Ctrl+X`-Familie umfasst zwei Kontexte: `ctrl+x ctrl+k` und `ctrl+x ctrl+e` in `Chat` sowie `ctrl+x ctrl+b` in `Task`. Um `ctrl+x` selbst als Single-Key-Binding zurückzugewinnen, heben Sie alle auf:

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

Wenn Sie einige, aber nicht alle Akkorde auf einem Präfix aufheben, führt das Drücken des Präfix immer noch in den Akkord-Wartmodus für die verbleibenden Bindings ein.

<h2 id="reserved-shortcuts">
  Reservierte Kürzel
</h2>

Diese Kürzel können nicht neu gebunden werden:

| Kürzel    | Grund                                              |
| :-------- | :------------------------------------------------- |
| Ctrl+C    | Hardcodierter Interrupt/Abbruch                    |
| Ctrl+D    | Hardcodierter Ausstieg                             |
| Ctrl+M    | Identisch mit Enter in Terminals (beide senden CR) |
| Caps Lock | Nicht an Terminalanwendungen übermittelt           |

<h2 id="terminal-conflicts">
  Terminal-Konflikte
</h2>

Einige Kürzel können mit Terminal-Multiplexern in Konflikt geraten:

| Kürzel | Konflikt                                 |
| :----- | :--------------------------------------- |
| Ctrl+B | tmux-Präfix (zweimal drücken zum Senden) |
| Ctrl+A | GNU Screen-Präfix                        |
| Ctrl+Z | Unix-Prozess-Suspend (SIGTSTP)           |

<h2 id="vim-mode-interaction">
  Vim-Modus-Interaktion
</h2>

Wenn der Vim-Modus aktiviert ist über `/config` → Editor-Modus, arbeiten Keybindings und Vim-Modus unabhängig:

* **Vim-Modus** verarbeitet Eingaben auf der Texteingangsebene (Cursor-Bewegung, Modi, Bewegungen)
* **Keybindings** verarbeiten Aktionen auf der Komponentenebene (Aufgaben umschalten, senden usw.)
* Die Escape-Taste im Vim-Modus wechselt von INSERT zu NORMAL-Modus; sie löst nicht `chat:cancel` aus
* Die meisten Ctrl+Taste-Kürzel werden durch den Vim-Modus zum Keybinding-System weitergeleitet
* Vim-Tasten können nicht über die Keybindings-Datei neu zugeordnet werden. Um eine zwei-Tasten-INSERT-Modus-Sequenz wie `jj` auf Escape abzubilden, verwenden Sie die Einstellung [`vimInsertModeRemaps`](/docs/de/interactive-mode#remap-insert-mode-key-sequences)
* Im Vim-NORMAL-Modus zeigt `?` das Hilfemenü an (Vim-Verhalten)
* Im Vim-NORMAL-Modus öffnet `/` die Verlaufssuche, dasselbe wie Ctrl+R im Standardmodus

<h2 id="validation">
  Validierung
</h2>

Claude Code validiert Ihre Keybindings und zeigt Warnungen für:

* Parse-Fehler (ungültiges JSON oder Struktur)
* Ungültige Kontextnamen
* Reservierte Kürzel-Konflikte
* Terminal-Multiplexer-Konflikte
* Doppelte Bindings im selben Kontext

Claude Code meldet Warnungen, wenn die Datei geladen wird, und schreibt jede einzelne in das Debug-Protokoll. Starten Sie Claude Code mit [`--debug`](/docs/de/cli-reference#cli-flags), um die Details zu sehen.
