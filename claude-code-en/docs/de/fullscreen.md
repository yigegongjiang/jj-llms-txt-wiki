> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Vollbildrendering

> Aktivieren Sie einen sanfteren, flimmerfreien Rendering-Modus mit Mausunterstützung und stabiler Speichernutzung in langen Gesprächen.

<Note>
  Vollbildrendering ist eine optionale [Forschungsvorschau](#research-preview). Führen Sie `/tui fullscreen` aus, um in Ihrem aktuellen Gespräch zu wechseln. Das Verhalten kann sich basierend auf Feedback ändern.
</Note>

Vollbildrendering ist ein alternativer Rendering-Pfad für die Claude Code CLI, der Flimmern eliminiert, die Speichernutzung in langen Gesprächen konstant hält und Mausunterstützung hinzufügt. Es zeichnet die Benutzeroberfläche auf dem alternativen Bildschirmpuffer des Terminals, wie `vim` oder `htop`, und rendert nur Nachrichten, die derzeit sichtbar sind. Dies reduziert die Menge der Daten, die bei jeder Aktualisierung an Ihr Terminal gesendet werden.

Der Unterschied ist am deutlichsten in Terminal-Emulatoren, bei denen der Rendering-Durchsatz der Engpass ist, wie das VS Code integrierte Terminal, tmux und iTerm2. Wenn Ihre Terminal-Scroll-Position nach oben springt, während Claude arbeitet, oder der Bildschirm flackert, während die Tool-Ausgabe einströmt, behebt dieser Modus diese Probleme.

<Note>
  Der Begriff Vollbild beschreibt, wie Claude Code die Zeichenfläche des Terminals übernimmt, wie `vim` es tut. Es hat nichts damit zu tun, Ihr Terminal-Fenster zu maximieren, und funktioniert bei jeder Fenstergröße.
</Note>

<h2 id="enable-fullscreen-rendering">
  Vollbildrendering aktivieren
</h2>

Führen Sie `/tui fullscreen` in einem beliebigen Claude Code Gespräch aus. Die CLI speichert die [`tui` Einstellung](/docs/de/settings#available-settings) und startet mit Ihrem Gespräch intakt in den Vollbildmodus neu, sodass Sie die Sitzung wechseln können, ohne den Kontext zu verlieren. Führen Sie `/tui default` aus, um zum klassischen Renderer zurückzuwechseln, oder `/tui` ohne Argument, um zu drucken, welcher Renderer aktiv ist.

Die neu gestartete Sitzung behält das Gespräch so bei, wie es auf dem Bildschirm angezeigt wird. Wenn Sie [`/rewind`](/docs/de/checkpointing#rewind-and-summarize) früher in der Sitzung ausgeführt haben, wird der Neustart vom zurückgespulten Punkt aus fortgesetzt, anstatt vom längeren Transkript, das auf der Festplatte gespeichert ist. Vor v2.1.207 stellte das Wechseln von Renderern nach einem Rewind das Gespräch wieder her, das der Rewind entfernt hatte.

Sie können auch die Umgebungsvariable `CLAUDE_CODE_NO_FLICKER` vor dem Starten von Claude Code setzen:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Die `tui` Einstellung und die Umgebungsvariable sind gleichwertig. Der `/tui` Befehl löscht `CLAUDE_CODE_NO_FLICKER` aus dem neu gestarteten Prozess, sodass die Einstellung, die er schreibt, wirksam wird.

<h2 id="what-changes">
  Was sich ändert
</h2>

Vollbildrendering ändert, wie die CLI auf Ihr Terminal zeichnet. Das Eingabefeld bleibt am unteren Bildschirmrand fixiert, anstatt sich zu bewegen, wenn die Ausgabe einströmt. Wenn die Eingabe stillsteht, während Claude arbeitet, ist Vollbildrendering aktiv. Nur sichtbare Nachrichten werden im Render-Baum beibehalten, sodass der Speicher unabhängig von der Gesprächslänge konstant bleibt.

Da das Gespräch im alternativen Bildschirmpuffer statt in Ihrem Terminal-Scrollback lebt, funktionieren einige Dinge anders:

| Vorher                                                              | Jetzt                                                                                   | Details                                                                    |
| :------------------------------------------------------------------ | :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------------- |
| `Cmd+f` oder tmux-Suche zum Finden von Text                         | `Ctrl+o` für Transkript-Modus, dann `/` zum Suchen oder `[` zum Schreiben in Scrollback | [Gespräch durchsuchen und überprüfen](#search-and-review-the-conversation) |
| Natives Klicken und Ziehen des Terminals zum Auswählen und Kopieren | In-App-Auswahl, wird beim Loslassen der Maus automatisch kopiert                        | [Maus verwenden](#use-the-mouse)                                           |
| `Cmd`-Klick zum Öffnen einer URL                                    | `Cmd`-Klick auf macOS, `Ctrl`-Klick anderswo                                            | [Maus verwenden](#use-the-mouse)                                           |

Wenn die Mauserfassung Ihren Arbeitsablauf beeinträchtigt, können Sie sie [deaktivieren](#keep-native-text-selection), während Sie das flimmerfreie Rendering beibehalten.

<h2 id="use-the-mouse">
  Maus verwenden
</h2>

Vollbildrendering erfasst Mausereignisse und verarbeitet sie in Claude Code:

* **Klicken Sie in die Eingabeaufforderung**, um Ihren Cursor überall im eingegebenen Text zu positionieren.
* **Klicken Sie auf einen Vorschlag in der `/`-Befehlsliste oder `@`-Dateiliste**, um ihn zu akzeptieren. Das Hovern hebt die Zeile unter Ihrem Cursor hervor.
* **Klicken Sie auf eine Option in einem Auswahlmenü**, um sie auszuwählen. Dies umfasst Berechtigungsaufforderungen, `/model`, `/config` und andere Dialoge, die eine Liste von Optionen anzeigen. Das Hovern zeigt einen Zeiger auf der Zeile unter Ihrem Cursor. Erfordert Claude Code v2.1.187 oder später.
* **Klicken Sie auf eine Option in einem Mehrfachauswahlmenü**, um sie umzuschalten, und klicken Sie auf die Schaltfläche „Senden", um Ihre Auswahl zu bestätigen. Wenn Sie auf eine Freitextzeile klicken, z. B. die Zeile „Sonstiges" in einer Multiple-Choice-Frage, wird das Eingabefeld fokussiert, damit Sie eine Antwort eingeben können. Erfordert Claude Code v2.1.208 oder später.
* **Klicken Sie auf ein eingeklapptes Tool-Ergebnis**, um es zu erweitern und die vollständige Ausgabe anzuzeigen. Klicken Sie erneut, um es zu reduzieren. Der Tool-Aufruf und sein Ergebnis werden zusammen erweitert. Nur Nachrichten, die mehr zu zeigen haben, sind anklickbar.
* **Halten Sie `Cmd` auf macOS oder `Ctrl` auf Linux und Windows und klicken Sie auf eine URL oder einen Dateipfad**, um ihn zu öffnen. Dateipfade in der Tool-Ausgabe, wie die nach einem Edit oder Write gedruckten, öffnen sich in Ihrer Standardanwendung. Einfache `http://` und `https://` URLs öffnen sich in Ihrem Browser. Ab v2.1.181 öffnet ein einfacher Klick ohne Halten von `Cmd` oder `Ctrl` keine Links mehr, was dem nativen Terminal-Verhalten entspricht. Einige macOS-Terminals leiten `Cmd`+Klick an die laufende Anwendung weiter, anstatt den Link selbst zu öffnen, und das Terminal-Mausprotokoll hat keine Möglichkeit, die `Cmd`-Taste zu kodieren, daher empfängt Claude Code es als einfachen Klick. In Ghostty und ab v2.1.198 in Warp auf macOS erkennt Claude Code dies und ermöglicht es, dass ein einfacher Klick auf einen Link ihn öffnet, und das Halten von `Cmd` funktioniert immer noch. Im VS Code integrierten Terminal und ähnlichen xterm.js-basierten Terminals überlässt Claude Code den Link-Handler des Terminals, der die gleiche Geste verwendet.
* **Klicken und ziehen** Sie, um Text überall im Gespräch auszuwählen. Doppelklick wählt ein Wort aus und entspricht iTerm2s Wortgrenzen, sodass ein Dateipfad als eine Einheit ausgewählt wird. Ab v2.1.198 wählt Doppelklick auf eine URL die gesamte URL aus, einschließlich des Schemas. Dreifachklick wählt die Zeile aus.
* **Scrollen Sie mit dem Mausrad**, um sich durch das Gespräch zu bewegen.

Ausgewählter Text wird beim Loslassen der Maus automatisch in Ihre Zwischenablage kopiert. Um dies auszuschalten, schalten Sie „Beim Auswählen kopieren" in `/config` um.

Mit „Beim Auswählen kopieren" ausgeschaltet, drücken Sie `Ctrl+Shift+c`, um manuell zu kopieren. Auf Terminals, die das Kitty-Tastaturprotokoll unterstützen, wie Kitty, WezTerm, Ghostty und iTerm2, funktioniert auch `Cmd+c`. Wenn Sie eine Auswahl aktiv haben, kopiert `Ctrl+c` statt zu stornieren.

Mit einer aktiven Auswahl halten Sie `Shift` und drücken die Pfeiltasten, um sie von der Tastatur aus zu erweitern. `Shift+↑` und `Shift+↓` scrollen den Viewport, wenn die Auswahl die obere oder untere Kante erreicht. `Shift+Home` und `Shift+End` erweitern bis zum Anfang oder Ende der aktuellen Zeile.

<h2 id="scroll-the-conversation">
  Gespräch scrollen
</h2>

Vollbildrendering verarbeitet das Scrollen in der App. Verwenden Sie diese Verknüpfungen zum Navigieren:

| Verknüpfung     | Aktion                                                                                |
| :-------------- | :------------------------------------------------------------------------------------ |
| `PgUp` / `PgDn` | Scrollen Sie um die Hälfte eines Bildschirms nach oben oder unten                     |
| `Ctrl+Home`     | Springen Sie zum Anfang des Gesprächs                                                 |
| `Ctrl+End`      | Springen Sie zur neuesten Nachricht und aktivieren Sie das automatische Folgen erneut |
| Mausrad         | Scrollen Sie ein paar Zeilen auf einmal                                               |

Auf Tastaturen ohne dedizierte `PgUp`-, `PgDn`-, `Home`- oder `End`-Tasten, wie MacBook-Tastaturen, halten Sie `Fn` mit den Pfeiltasten: `Fn+↑` sendet `PgUp`, `Fn+↓` sendet `PgDn`, `Fn+←` sendet `Home` und `Fn+→` sendet `End`. `Ctrl+Fn+→` erreicht Claude Code auf macOS nicht, daher hat eine MacBook-Tastatur standardmäßig keine funktionierende Verknüpfung zum Springen nach unten. Verwenden Sie stattdessen eine dieser Optionen:

* Klicken Sie auf die [Schaltfläche zum Springen nach unten](#auto-follow).
* Scrollen Sie mit dem Mausrad nach unten, um das Folgen fortzusetzen.
* Binden Sie `scroll:bottom` an eine Verknüpfung neu, die Ihre Tastatur senden kann.

Diese Aktionen sind neu bindbar. Siehe [Scroll-Aktionen](/docs/de/keybindings#scroll-actions) für die vollständige Liste der Aktionsnamen, einschließlich Varianten für halbe Seiten und ganze Seiten, die keine Standardbindung haben.

<h3 id="auto-follow">
  Automatisches Folgen
</h3>

Scrollen nach oben pausiert das automatische Folgen, sodass neue Ausgabe Sie nicht zurück nach unten zieht. Eine `Zum unteren Ende springen`-Schaltfläche schwebt über der unteren Kante des Transkripts, während Sie nach oben gescrollt sind, und zeigt eine Anzahl wie `3 neue Nachrichten` an, wenn neue Ausgabe ankommt. Klicken Sie darauf, drücken Sie `Ctrl+End`, oder scrollen Sie nach unten, um das Folgen fortzusetzen.

Während das automatische Folgen pausiert ist, bleibt die Ansicht auch dort, wo Sie sie gescrollt haben, wenn eine Antwort das Streaming beendet. Vor v2.1.207 konnte die Ansicht über den Anfang der Antwort springen, wenn eine lange Antwort das Streaming beendete.

Der Tastatur-Hinweis der Schaltfläche spiegelt wider, was Ihre Tastatur senden kann. Auf macOS schlägt sie vor, zu klicken oder `Fn+↓` zum Scrollen zu verwenden, da `Ctrl+End` Claude Code von einer Mac-Tastatur nicht erreicht. Binden Sie [`scroll:bottom`](/docs/de/keybindings#scroll-actions) neu und die Schaltfläche zeigt Ihre Verknüpfung auf jeder Plattform an. Vor v2.1.206 schlug die Schaltfläche `Ctrl+End` auf macOS vor.

Auf einem Terminal, das zu schmal für die vollständige Beschriftung ist, verkürzt die Schaltfläche den Hinweis, anstatt auf die Transkriptzeile darunter umzubrechen. Vor v2.1.206 konnte eine lange Beschriftung über das Transkript umgebrochen werden.

Um das automatische Folgen ganz auszuschalten, sodass die Ansicht dort bleibt, wo Sie sie verlassen, öffnen Sie `/config` und setzen Sie „Automatisches Scrollen" auf aus. Mit deaktiviertem automatischen Scrollen springt die Ansicht nie von selbst nach unten. Berechtigungsaufforderungen und andere Dialoge, die eine Antwort benötigen, scrollen unabhängig von dieser Einstellung in die Ansicht.

<h3 id="mouse-wheel-scrolling">
  Mausrad-Scrollen
</h3>

Das Mausrad-Scrollen erfordert, dass Ihr Terminal Mausereignisse an Claude Code weiterleitet. Die meisten Terminals tun dies, wenn eine Anwendung dies anfordert. iTerm2 macht es zu einer Pro-Profil-Einstellung: Wenn das Rad nichts tut, aber `PgUp` und `PgDn` funktionieren, öffnen Sie Einstellungen → Profile → Terminal und aktivieren Sie „Mausberichte aktivieren". Die gleiche Einstellung ist auch erforderlich, damit Klick-zum-Erweitern und Textauswahl funktionieren.

Wenn sich das Scrollen mit dem Mausrad langsam anfühlt, sendet Ihr Terminal möglicherweise ein Scroll-Ereignis pro physischer Kerbe ohne Multiplikator. Einige Terminals, wie Ghostty und iTerm2 mit aktiviertem schnellerem Scrollen, verstärken bereits Rad-Ereignisse. Andere, einschließlich des VS Code integrierten Terminals, senden genau ein Ereignis pro Kerbe. Claude Code kann nicht erkennen, welches.

Setzen Sie `CLAUDE_CODE_SCROLL_SPEED`, um die Basis-Scroll-Distanz zu multiplizieren:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Ein Wert von `3` entspricht dem Standard in `vim` und ähnlichen Anwendungen. Die Einstellung akzeptiert Werte von 1 bis 20 und Bruchteile unter 1, wie `0,5`, um beschleunigtes Trackpad- und Mausrad-Scrollen in Terminals zu verlangsamen, die Rad-Ereignisse bereits verstärken.

Um die Scroll-Geschwindigkeit interaktiv anzupassen, führen Sie `/scroll-speed` aus. Der Dialog zeigt ein Lineal an, das Sie scrollen können, während er offen ist, sodass Sie die Änderung sofort spüren können. Drücken Sie `←` und `→` zum Anpassen, `r` zum Zurücksetzen auf den automatisch erkannten Standard und `Enter` zum Speichern.

Der Befehl schreibt denselben Wert, den die Umgebungsvariable `CLAUDE_CODE_SCROLL_SPEED` setzt, persistent in `~/.claude/settings.json`. Der Befehl ist im JetBrains IDE-Terminal nicht verfügbar.

Unabhängig von der Basisgeschwindigkeit beschleunigt Claude Code die Scroll-Rate, wenn Sie das Rad schnell drehen, sodass eine schnelle Drehung eine größere Distanz abdeckt als die gleiche Anzahl langsamer Kerben. Um die Beschleunigung auszuschalten und eine konstante Rate pro Kerbe beizubehalten, setzen Sie `wheelScrollAccelerationEnabled` auf `false` in [`settings.json`](/docs/de/settings#available-settings). Diese Einstellung erfordert Claude Code v2.1.174 oder später.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Scrollen im JetBrains IDE-Terminal
</h3>

Im JetBrains IDE-Terminal wendet Claude Code seine eigene Scroll-Verarbeitung an und ignoriert `CLAUDE_CODE_SCROLL_SPEED`. Das Terminal sendet Scroll-Ereignisse mit einer viel höheren Rate als andere Emulatoren, sodass ein anderswo abgestimmter Multiplikator hier zu weit geht.

In 2025.2 hat das Terminal auch Scroll-Rad-Fehler, die fehlerhafte Pfeiltasten und Ereignisse in der falschen Richtung erzeugen. Claude Code erkennt diese zur Laufzeit und mindert sie automatisch ab, sodass Trackpad- und Mausrad-Scrollen ohne Konfiguration funktionieren. Für das beste Scroll-Erlebnis führen Sie ein Upgrade auf 2025.3 oder später durch. Claude Code zeigt einen Hinweis beim ersten Scrollen an, wenn es den Fehler erkennt.

<h2 id="search-and-review-the-conversation">
  Gespräch durchsuchen und überprüfen
</h2>

`Ctrl+o` schaltet zwischen der normalen Eingabeaufforderung und dem Transkript-Modus um.

Für eine ruhigere Ansicht, die nur Ihre letzte Eingabeaufforderung, eine einzeilige Zusammenfassung von Tool-Aufrufen mit Edit-Diffstats und die endgültige Antwort zeigt, führen Sie `/focus` aus. Die Einstellung bleibt über Sitzungen hinweg erhalten. Führen Sie `/focus` erneut aus, um es auszuschalten.

Der Transkript-Modus erhält `less`-ähnliche Navigation und Suche:

| Taste                                  | Aktion                                                                                                                                                                         |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                    | Öffnen Sie die Suche. Geben Sie ein, um Übereinstimmungen zu finden, drücken Sie `Enter`, um zu akzeptieren, `Esc`, um abzubrechen und Ihre Scroll-Position wiederherzustellen |
| `n` / `N`                              | Springen Sie zur nächsten oder vorherigen Übereinstimmung. Funktioniert, nachdem Sie die Suchleiste geschlossen haben                                                          |
| `j` / `k` oder `↑` / `↓`               | Scrollen Sie eine Zeile                                                                                                                                                        |
| `g` / `G` oder `Home` / `End`          | Springen Sie nach oben oder unten                                                                                                                                              |
| `Ctrl+u` / `Ctrl+d`                    | Scrollen Sie eine halbe Seite                                                                                                                                                  |
| `Ctrl+b` / `Ctrl+f` oder `Space` / `b` | Scrollen Sie eine ganze Seite                                                                                                                                                  |
| `Ctrl+o`, `Esc` oder `q`               | Beenden Sie den Transkript-Modus und kehren Sie zur Eingabeaufforderung zurück                                                                                                 |

Das `Cmd+f` Ihres Terminals und die tmux-Suche sehen das Gespräch nicht, da es im alternativen Bildschirmpuffer lebt, nicht im nativen Scrollback. Um den Inhalt an Ihr Terminal zurückzugeben, drücken Sie `Ctrl+o`, um zuerst den Transkript-Modus zu aktivieren, dann:

* **`[`**: schreibt das vollständige Gespräch in den nativen Scrollback-Puffer Ihres Terminals, mit allen erweiterten Tool-Ausgaben. Das Gespräch ist jetzt gewöhnlicher Text in Ihrem Terminal, sodass `Cmd+f`, tmux-Kopiermodus und alle anderen nativen Tools es durchsuchen oder auswählen können. Lange Sitzungen können einen Moment pausieren, während dies geschieht. Dies dauert, bis Sie den Transkript-Modus mit `Esc` oder `q` beenden, was Sie zum Vollbildrendering zurückbringt. Das nächste `Ctrl+o` startet von vorne.
* **`v`**: schreibt das Gespräch in eine temporäre Datei und öffnet es in `$VISUAL` oder `$EDITOR`.

Drücken Sie `Esc` oder `q`, um zur Eingabeaufforderung zurückzukehren.

<h2 id="clear-the-conversation">
  Gespräch löschen
</h2>

Drücken Sie `Ctrl+L` zweimal innerhalb von zwei Sekunden, um `/clear` auszuführen und ein neues Gespräch zu starten. Der erste Druck zeichnet den Bildschirm neu und zeigt einen Hinweis; der zweite Druck löscht das Gespräch. Auf macOS löscht auch das doppelte Drücken von `Cmd+K` das Gespräch mit `/clear`.

<h2 id="use-with-tmux">
  Mit tmux verwenden
</h2>

Vollbildrendering funktioniert in tmux mit drei Einschränkungen.

Das Scrollen mit dem Mausrad erfordert tmux-Mausmodus. Wenn Ihre `~/.tmux.conf` ihn nicht bereits aktiviert, fügen Sie diese Zeile hinzu und laden Sie Ihre Konfiguration neu:

```bash theme={null}
set -g mouse on
```

Ohne Mausmodus gehen Rad-Ereignisse an tmux statt an Claude Code. Tastatur-Scrollen mit `PgUp` und `PgDn` funktioniert in beiden Fällen. Claude Code druckt einen einmaligen Hinweis beim Start, wenn es tmux mit ausgeschaltetem Mausmodus erkennt.

Vollbildrendering ist nicht kompatibel mit iTerm2s tmux-Integrationsmodus, das ist der Modus, den Sie mit `tmux -CC` aktivieren. Im Integrationsmodus rendert iTerm2 jeden tmux-Bereich als natives Split, anstatt tmux auf dem Terminal zeichnen zu lassen. Der alternative Bildschirmpuffer und die Mausverfolgung funktionieren dort nicht korrekt: das Mausrad tut nichts, und Doppelklick kann den Terminal-Status beschädigen. Aktivieren Sie Vollbildrendering nicht in `tmux -CC` Sitzungen. Reguläres tmux in iTerm2 ohne `-CC` funktioniert einwandfrei.

Nicht jede tmux-Version wendet synchronisierte Ausgabe von Anwendungen an, daher können Sie während Neuzeichnungen unter tmux mehr Flimmern sehen als beim direkten Ausführen von Claude Code in Ihrem Terminal. Wenn das Flimmern auffällig ist, besonders über SSH, führen Sie ein Upgrade auf die neueste tmux-Version durch oder führen Sie Claude Code in seiner eigenen Terminal-Registerkarte außerhalb von tmux aus. Überprüfen Sie Ihre tmux-Version mit `tmux -V`.

Claude Code aktiviert synchronisierte Ausgabe automatisch, wenn es tmux 3.4 oder später aus der `TERM_PROGRAM_VERSION`-Variable erkennt, und greift auf direkte Abfragen des Terminals nach Unterstützung für synchronisierte Ausgabe zurück, wenn die Version nicht bestimmt werden kann. Ob Neuzeichnungen tatsächlich atomar werden, hängt davon ab, ob Ihre tmux-Version synchronisierte Ausgabe berücksichtigt; wenn Sie unter tmux 3.4 oder später immer noch Flimmern sehen, führen Sie ein Upgrade auf die neueste tmux-Version durch. Diese Erkennung erfordert Claude Code v2.1.200 oder später.

<h2 id="keep-native-text-selection">
  Native Textauswahl beibehalten
</h2>

Die Mauserfassung ist der häufigste Reibungspunkt, besonders über SSH oder in tmux. Wenn Claude Code Mausereignisse erfasst, funktioniert die native Kopieren-beim-Auswählen Ihres Terminals nicht mehr. Die Auswahl, die Sie mit Klicken und Ziehen treffen, existiert in Claude Code, nicht in Ihrem Terminal-Auswahlpuffer, sodass tmux-Kopiermodus, Kitty-Hinweise und ähnliche Tools sie nicht sehen.

Claude Code schreibt die Auswahl in Ihre Systemzwischenablage, und der Pfad, den es verwendet, hängt von Ihrem Setup ab. In einer lokalen Sitzung führt es ein natives Zwischenablage-Tool aus:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` auf Wayland oder `xclip` oder `xsel` auf X11, je nachdem, was installiert ist. Claude Code schreibt sowohl in die Zwischenablage als auch in die PRIMARY-Auswahl, sodass Mitteltaste-Einfügen funktioniert.
* **Windows und WSL**: PowerShell `Set-Clipboard`

In tmux schreibt es auch in den tmux-Paste-Puffer. Über SSH fällt es auf OSC 52 Escape-Sequenzen zurück. Claude Code druckt nach jeder Kopie einen Toast, der Ihnen mitteilt, welchen Pfad es verwendet hat.

Einige Terminals blockieren OSC 52 standardmäßig. iTerm2 blockiert es, bis Sie Einstellungen → Allgemein → Auswahl → Anwendungen im Terminal dürfen auf Zwischenablage zugreifen aktivieren. Wenn Sie [`/terminal-setup`](/docs/de/terminal-config) in iTerm2 ausführen, wird dies für Sie aktiviert.

Für eine einmalige native Auswahl hängt die zu verwendende Taste von Ihrem Terminal ab:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor und Devin Desktop**: `Shift` oder `Option` auf macOS mit der Einstellung `terminal.integrated.macOptionClickForcesSelection` aktiviert
* **Die meisten anderen Terminals**: `Shift`

Halten Sie diese Taste gedrückt, während Sie klicken und ziehen. Ihr Terminal verarbeitet die Auswahl selbst, anstatt sie an Claude Code weiterzuleiten, sodass Kopierverknüpfungen wie `Cmd+C` auf das funktionieren, was Sie auswählen. Claude Code zeigt auch die richtige Taste in seinem On-Screen-Hinweis an.

Über SSH oder in tmux kann Claude Code das Terminal, von dem aus Sie sich verbinden, nicht immer erkennen, daher listet der Hinweis stattdessen die Kandidatentasten auf.

Wenn Sie sich die ganze Zeit auf native Auswahl verlassen, setzen Sie `CLAUDE_CODE_DISABLE_MOUSE=1`, um die Mauserfassung zu deaktivieren, während Sie das flimmerfreie Rendering und flachen Speicher beibehalten:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Mit deaktivierter Mauserfassung funktioniert Tastatur-Scrollen mit `PgUp`, `PgDn`, `Ctrl+Home` und `Ctrl+End` immer noch, und Ihr Terminal verarbeitet die Auswahl nativ. Sie verlieren Klick-zum-Positionieren-des-Cursors, Klick-zum-Erweitern-der-Tool-Ausgabe, URL-Klicken und Rad-Scrollen in Claude Code.

Um Rad-Scrollen beizubehalten, aber Klick-, Zieh- und Hover-Verarbeitung auszuschalten, setzen Sie stattdessen `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`. Erfordert Claude Code v2.1.195 oder später. `CLAUDE_CODE_DISABLE_MOUSE` hat Vorrang, wenn beide Variablen gesetzt sind.

Mit deaktivierten Klicks erfasst Claude Code die Maus immer noch, sodass das Rad und das Trackpad die Konversation scrollen, aber linke Klicks funktionieren nicht in Claude Code. Sie müssen immer noch die Taste Ihres Terminals halten, um native Klick-und-Zieh-Auswahl zu treffen. Rechtsklick und Mitteltaste-Einfügen funktionieren weiterhin auf Terminals, die sie unterstützen.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Veralteter oder falsch platzierter Text auf dem Bildschirm
</h3>

Das Vollbild-Rendering sendet nur die Zellen, die sich zwischen den Frames geändert haben. Einige Terminals, am häufigsten Windows Terminal und andere ConPTY-gestützte Hosts, führen diese positionierten Schreibvorgänge falsch zusammen und hinterlassen Fragmente der früheren Ausgabe auf dem Bildschirm, bis Sie das Fenster vergrößern.

Setzen Sie [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/de/env-vars), um jede Zelle bei jedem Frame neu zu zeichnen, anstatt inkrementelle Updates zu senden.

Unter Windows PowerShell:

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

Unter macOS oder Linux:

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

Unter Windows aktiviert Claude Code bereits automatisch das vollständige Neuzeichnen für Hintergrund-Sitzungen und [Agent-Ansicht](/docs/de/agent-view), daher müssen Sie die Variable nur für eine interaktive Vollbild-Sitzung setzen, die Sie direkt gestartet haben.

<h2 id="research-preview">
  Forschungsvorschau
</h2>

Vollbildrendering ist eine Forschungsvorschau-Funktion. Es wurde auf gängigen Terminal-Emulatoren getestet, aber Sie können auf weniger gängigen Terminals oder ungewöhnlichen Konfigurationen auf Rendering-Probleme stoßen.

Wenn Sie auf ein Problem stoßen, führen Sie `/feedback` in Claude Code aus, um es zu melden, oder öffnen Sie ein Problem im [claude-code GitHub-Repository](https://github.com/anthropics/claude-code/issues). Geben Sie Ihren Terminal-Emulator-Namen und die Version an.

Um Vollbildrendering auszuschalten, führen Sie `/tui default` aus, oder heben Sie `CLAUDE_CODE_NO_FLICKER` auf, wenn Sie es auf diese Weise aktiviert haben. Um den klassischen Renderer unabhängig von der gespeicherten `tui`-Einstellung zu erzwingen, setzen Sie `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. Der klassische Renderer behält die Konversation im nativen Scrollback Ihres Terminals bei, sodass `Cmd+f` und der tmux-Kopiermodus wie gewohnt funktionieren.

Hintergrund-Sitzungen, die aus der [Agent-Ansicht](/docs/de/agent-view) oder `claude attach` geöffnet werden, verwenden immer Vollbildrendering. Das angehängte Terminal wechselt in den alternativen Bildschirmpuffer, um die Sitzung anzuzeigen, und der klassische Renderer hat dort keinen Scrollback oder Maushandling, daher gelten die `tui`-Einstellung und `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` nicht für diese.
