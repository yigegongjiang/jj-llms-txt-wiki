> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Vollbildrendering

> Aktivieren Sie einen sanfteren, flimmerfreien Rendering-Modus mit Mausunterstützung und stabiler Speichernutzung in langen Gesprächen.

<Note>
  Vollbildrendering ist eine [Forschungsvorschau](#research-preview). Ob Sie [im Vollbildmodus oder im klassischen Renderer starten](#fullscreen-by-default), hängt von Ihrer Einrichtung ab. Führen Sie `/tui fullscreen` oder `/tui default` aus, um in Ihrem aktuellen Gespräch zu wechseln. Das Verhalten kann sich basierend auf Feedback ändern.
</Note>

Vollbildrendering ist ein alternativer Rendering-Pfad für die Claude Code CLI, der Flimmern eliminiert, die Speichernutzung in langen Gesprächen konstant hält und Mausunterstützung hinzufügt. Es zeichnet die Benutzeroberfläche auf dem alternativen Bildschirmpuffer des Terminals, wie `vim` oder `htop`, und rendert nur Nachrichten, die derzeit sichtbar sind. Dies reduziert die Menge der Daten, die bei jeder Aktualisierung an Ihr Terminal gesendet werden.

Der Unterschied ist am deutlichsten in Terminal-Emulatoren, bei denen der Rendering-Durchsatz der Engpass ist, wie das VS Code integrierte Terminal, tmux und iTerm2. Wenn Ihre Terminal-Scroll-Position nach oben springt, während Claude arbeitet, oder der Bildschirm flackert, während die Tool-Ausgabe einströmt, behebt dieser Modus diese Probleme.

<Note>
  Der Begriff Vollbild beschreibt, wie Claude Code die Zeichenfläche des Terminals übernimmt, wie `vim` es tut. Es hat nichts damit zu tun, Ihr Terminal-Fenster zu maximieren, und funktioniert bei jeder Fenstergröße.
</Note>

<h2 id="enable-fullscreen-rendering">
  Vollbildrendering aktivieren
</h2>

Führen Sie `/tui fullscreen` in einem beliebigen Claude Code Gespräch aus. Die CLI speichert die [`tui` Einstellung](/docs/de/settings-reference#tui) und startet mit Ihrem Gespräch intakt in den Vollbildmodus neu, sodass Sie die Sitzung wechseln können, ohne den Kontext zu verlieren. Führen Sie `/tui default` aus, um zum klassischen Renderer zurückzuwechseln, oder `/tui` ohne Argument, um zu drucken, welcher Renderer aktiv ist.

Im [Bildschirmlesemodus](/docs/de/accessibility) verwendet Claude Code immer den klassischen Renderer, außer in angehängten [Hintergrundsitzungen](/docs/de/agent-view), die weiterhin im Vollbildmodus gerendert werden. Wenn Sie `/tui fullscreen` in einer anderen Sitzung ausführen, gibt Claude Code stattdessen eine Erklärung aus und wechselt nicht, und ändert die gespeicherte `tui` Einstellung nicht.

Claude Code überträgt diese in die neu gestartete Sitzung:

* Das Gespräch, wie es auf dem Bildschirm angezeigt wird. Nach einem [`/rewind`](/docs/de/checkpointing#rewind-and-summarize) bedeutet das:
  * Wenn Sie früher in der Sitzung zurückgespult haben, startet Claude Code vom zurückgespulten Punkt neu, nicht vom längeren Transkript, das auf der Festplatte gespeichert ist. Wenn Sie beispielsweise über Ihre letzten drei Nachrichten hinweg zurückgespult haben, öffnet sich die neu gestartete Sitzung ohne diese
  * Wenn Sie vor Ihre erste Nachricht zurückgespult haben, startet Claude Code mit einem leeren Gespräch neu
* Ihr [Berechtigungsmodus](/docs/de/permission-modes) und [Aufwandsstufe](/docs/de/model-config#adjust-effort-level)
* Das Modell, das Sie zuletzt mit [`/model`](/docs/de/model-config#setting-your-model) ausgewählt haben
* Regeln, die Sie mit [`--allowed-tools` oder `--disallowed-tools`](/docs/de/cli-reference#cli-flags) übergeben haben, und Ihre `--agent`, `--agents`, `--append-system-prompt` und `--system-prompt-snapshot` Flags

Claude Code lehnt einen Neustart ab, wenn die Sitzung eine Einschränkung hat, die es nicht an den neu gestarteten Prozess übergeben kann. Einschränkungen, die es nicht übergeben kann, sind:

* Startflags wie ein [`--system-prompt`](/docs/de/cli-reference#cli-flags) Ersatz, eine [`--tools`](/docs/de/cli-reference#cli-flags) Zulassungsliste oder [`--setting-sources`](/docs/de/cli-reference#cli-flags)
* Deny oder Ask Regeln, die ein [Hook oder SDK Berechtigungsupdate](/docs/de/hooks#permission-update-entries) nur für diese Sitzung hinzugefügt hat

In diesem Fall gibt Claude Code [`Cannot switch renderers in this session`](/docs/de/errors#cannot-switch-renderers-in-this-session) mit den Gründen aus. Es wechselt nicht oder speichert nichts.

Sie können auch die Umgebungsvariable `CLAUDE_CODE_NO_FLICKER` vor dem Starten von Claude Code setzen:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Wie die [`tui`](/docs/de/settings-reference#tui) Einstellung und die Variable kombiniert werden, wenn beide gesetzt sind, finden Sie im Eintrag der Einstellung. Nach einem [fehlgeschlagenen Vollbildstart](#fullscreen-renderer-didnt-finish-starting) berücksichtigt Claude Code die Variable weiterhin, aber nicht die Einstellung. Der `/tui` Befehl löscht `CLAUDE_CODE_NO_FLICKER` aus dem neu gestarteten Prozess, sodass die Einstellung, die er schreibt, wirksam wird.

<h3 id="fullscreen-by-default">
  Vollbild standardmäßig
</h3>

Angehängte [Hintergrundsitzungen](/docs/de/agent-view) werden im Vollbildmodus gerendert, und andere Sitzungen im [Bildschirmlesemodus](/docs/de/accessibility) verwenden den klassischen Renderer. Andernfalls startet Claude Code Sie im Renderer aus der ersten Zeile dieser Tabelle, die Ihrem Setup entspricht:

| Ihre Situation                                                                                                                                                                                | Renderer, in dem Sie starten              |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------- |
| Sie haben [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`](/docs/de/env-vars) oder `CLAUDE_CODE_NO_FLICKER=0` gesetzt                                                                                    | Klassisch                                 |
| Sie haben `CLAUDE_CODE_NO_FLICKER=1` gesetzt                                                                                                                                                  | Vollbild                                  |
| Claude Code hat [Vollbild nach einem fehlgeschlagenen Vollbildstart](#fullscreen-renderer-didnt-finish-starting) auf diesem Computer ausgeschaltet                                            | Klassisch                                 |
| Sie befinden sich im [`tmux -CC` Integrationsmodus](#use-with-tmux) von iTerm2, oder Sie sind über SSH mit Claude Code verbunden, das auf Windows läuft                                       | Klassisch                                 |
| Sie haben eine [`tui` Einstellung](/docs/de/settings-reference#tui) gespeichert                                                                                                                    | Der Renderer, den die Einstellung benennt |
| Ihre Sitzung [ruft keine Feature Flags von Anthropic ab](/docs/de/env-vars#features-that-need-feature-flag-fetching), und Claude Code hat das Startdialog auf diesem Computer nicht mehr angeboten | Klassisch                                 |
| Ihre Sitzung ruft keine Feature Flags von Anthropic ab, und der erste Claude Code Start auf diesem Computer war v2.1.239 oder später                                                          | Vollbild                                  |
| Ihre Sitzung ruft Feature Flags von Anthropic ab, und Sie haben Claude Code zum ersten Mal am oder nach dem 6. Mai 2026 verwendet                                                             | Vollbild                                  |
| Alles andere                                                                                                                                                                                  | Klassisch                                 |

Sitzungen, die keine Feature Flags von Anthropic abrufen, sind solche über [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry), und solche mit ausgeschalteter Telemetrie.

Wenn Sie im klassischen Renderer starten und keine `tui` Einstellung gespeichert haben, kann Claude Code beim Start ein Dialogfeld öffnen, das den Wechsel anbietet:

* Wenn Sie akzeptieren, startet Claude Code auf die gleiche Weise wie `/tui fullscreen` neu, überträgt den gleichen Sitzungszustand und speichert die Einstellung, sobald die neu gestartete Sitzung [erfolgreich gestartet](#fullscreen-renderer-didnt-finish-starting) hat.
* Wenn Sie **Jetzt nicht** wählen, bietet Claude Code dies auf diesem Computer nicht erneut an.
* Claude Code stellt das Angebot ein, nachdem es das Dialogfeld auf drei Starts angezeigt hat, beantwortet oder nicht.

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
  Verwenden Sie die Maus
</h2>

Das Vollbildrendering erfasst Mausereignisse und verarbeitet sie in Claude Code:

* **Klicken Sie in die Eingabeaufforderung**, um Ihren Cursor überall im eingegebenen Text zu positionieren.
* **Klicken Sie auf einen Vorschlag in der `/`-Befehlsliste oder `@`-Dateiliste**, um ihn zu akzeptieren. Das Hovern hebt die Zeile unter Ihrem Cursor hervor.
* **Klicken Sie auf eine Option in einem Auswahlmenü**, um sie auszuwählen. Dies umfasst Berechtigungsaufforderungen, `/model`, `/config` und andere Dialoge, die eine Liste von Optionen anzeigen. Das Hovern zeigt einen Zeiger auf der Zeile unter Ihrem Cursor.
* **Klicken Sie auf eine Option in einem Mehrfachauswahlmenü**, um sie umzuschalten, und klicken Sie auf die Schaltfläche „Senden", um Ihre Auswahl zu bestätigen. Wenn Sie auf eine Freitextzeile klicken, z. B. die Zeile `Other` in einer Multiple-Choice-Frage, wird das Eingabefeld fokussiert, damit Sie eine Antwort eingeben können. Erfordert Claude Code v2.1.208 oder später.
* **Klicken Sie auf den Wert einer Einstellung im `/config`-Bereich**, um ihn zu ändern, und scrollen Sie die Einstellungsliste mit dem Mausrad. Erfordert Claude Code v2.1.271 oder später.
* **Scrollen Sie ein Auswahlmenü oder Mehrfachauswahlmenü mit dem Mausrad**, wenn es mehr Optionen hat, als es auf einmal anzeigt, z. B. die `/model`-Liste in einem kurzen Terminalfenster. Das Rad scrollt die Liste, während sich der Zeiger über den Optionen befindet. Erfordert Claude Code v2.1.280 oder später.
* **Klicken Sie auf ein reduziertes Werkzeugergebnis**, um es zu erweitern und die vollständige Ausgabe anzuzeigen. Klicken Sie erneut, um es zu reduzieren. Der Werkzeugaufruf und sein Ergebnis werden zusammen erweitert. Nur Nachrichten, die mehr zu zeigen haben, sind anklickbar.
  * Das Klicken erweitert auch die Ausgabe eines `!`-Shell-Befehls, ob ein älteres abgeschnittenes Ergebnis oder die Live-Fortschrittszeile während der Befehlsausführung. Erfordert Claude Code v2.1.257 oder später.
* **Halten Sie `Cmd` auf macOS oder `Ctrl` auf Linux und Windows, und klicken Sie auf eine URL oder einen Dateipfad**, um ihn zu öffnen. Einfache `http://`- und `https://`-URLs werden in Ihrem Browser geöffnet, und Dateipfade in der Werkzeugausgabe, wie die nach einem Edit oder Write gedruckten, werden in Ihrer Standardanwendung geöffnet. Ein einfacher Klick ohne den Modifikator öffnet keine Links und entspricht dem nativen Terminal-Verhalten.
  * Claude Code rendert einen Netzwerkpfad (UNC-Pfad), z. B. `\\server\share\file.ts`, als reinen Text ohne Link, da das Öffnen eines Netzwerkpfads Ihre Windows-Anmeldedaten an den Host senden kann, den er benennt.
  * Einige macOS-Terminals leiten `Cmd`+Klick an die laufende App weiter, anstatt den Link selbst zu öffnen, und das Terminal-Mausprotokoll hat keine Möglichkeit, die `Cmd`-Taste zu codieren, daher empfängt Claude Code einen einfachen Klick. In Ghostty und in Warp auf macOS erkennt Claude Code dies und ermöglicht es, dass ein einfacher Klick auf einen Link ihn öffnet, und das Halten von `Cmd` funktioniert immer noch.
  * Im integrierten VS Code-Terminal und ähnlichen xterm.js-basierten Terminals delegiert Claude Code an den eigenen Link-Handler des Terminals, der die gleiche Geste verwendet.
* **Klicken und ziehen**, um Text überall in der Konversation auszuwählen. Doppelklick wählt ein Wort aus und entspricht den Wortgrenzen von iTerm2, sodass ein Dateipfad als eine Einheit ausgewählt wird. Doppelklick auf eine URL wählt die gesamte URL aus, einschließlich des Schemas. Dreifachklick wählt die Zeile aus.
* **Scrollen Sie mit dem Mausrad**, um sich durch die Konversation zu bewegen.

Der ausgewählte Text wird beim Loslassen der Maus automatisch in die Zwischenablage kopiert. Um dies auszuschalten, schalten Sie „Copy on select" in `/config` um.

Wenn „Copy on select" ausgeschaltet ist, drücken Sie `Ctrl+Shift+c`, um manuell zu kopieren. Auf Terminals, die das Kitty-Tastaturprotokoll unterstützen, z. B. Kitty, WezTerm, Ghostty und iTerm2, funktioniert auch `Cmd+c`. Wenn Sie eine Auswahl aktiv haben, kopiert `Ctrl+c` statt zu stornieren.

Wenn eine Auswahl aktiv ist, halten Sie `Shift` und drücken Sie die Pfeiltasten, um sie von der Tastatur aus zu erweitern. `Shift+↑` und `Shift+↓` scrollen den Viewport, wenn die Auswahl die obere oder untere Kante erreicht. `Shift+Home` und `Shift+End` erweitern bis zum Anfang oder Ende der aktuellen Zeile.

In der normalen Eingabeaufforderungsansicht hängt das, was mit einer aktiven Auswahl geschieht, von der Taste ab, die Sie drücken:

* **`Esc`**: Claude Code führt die übliche Aktion der Taste aus, z. B. das Unterbrechen der laufenden Antwort oder das Schließen eines offenen Dialogs, und die Auswahl bleibt hervorgehoben.
* **`PgUp`, `PgDn`, `Ctrl+Home`, `Ctrl+End` oder `Shift`, `Alt` oder `Option` oder `Cmd`, `Win` oder `Super` mit einer Pfeiltaste, `Home` oder `End`-Taste**: Die Auswahl bleibt erhalten.
* **Jede andere Taste, einschließlich einfacher Pfeiltasten, `Enter` und eingegebener Zeichen**: Claude Code löscht die Auswahl.
* **Eine Taste, die an [`selection:clear`](/docs/de/keybindings#scroll-actions) gebunden ist**: Claude Code löscht die Auswahl, auch wenn die Taste `Esc` oder eine andere Taste ist, die sie normalerweise beibehält. Die Aktion hat keine Standardbindung.

Im [Transkriptmodus](#search-and-review-the-conversation) behalten auch die dort aufgelisteten Navigations- und Suchtasten die Auswahl bei.

<h2 id="scroll-the-conversation">
  Durch das Gespräch scrollen
</h2>

Das Vollbildrendering verwaltet das Scrollen innerhalb der App. Verwenden Sie diese Tastenkombinationen zum Navigieren:

| Tastenkombination | Aktion                                                                     |
| :---------------- | :------------------------------------------------------------------------- |
| `PgUp` / `PgDn`   | Um eine halbe Bildschirmseite nach oben oder unten scrollen                |
| `Ctrl+Home`       | Zum Anfang des Gesprächs springen                                          |
| `Ctrl+End`        | Zur neuesten Nachricht springen und automatisches Folgen erneut aktivieren |
| Mausrad           | Um einige Zeilen gleichzeitig scrollen                                     |

Sie können zum Anfang der Sitzung zurückblättern, auch nach [Komprimierung](/docs/de/context-window#what-survives-compaction). Claude arbeitet weiterhin aus der Komprimierungszusammenfassung, aber Claude Code behält jede frühere Nachricht im Vollbild-Scrollback über wiederholte Komprimierungen hinweg.

Auf Tastaturen ohne dedizierte `PgUp`-, `PgDn`-, `Home`- oder `End`-Tasten, wie MacBook-Tastaturen, halten Sie `Fn` mit den Pfeiltasten gedrückt: `Fn+↑` sendet `PgUp`, `Fn+↓` sendet `PgDn`, `Fn+←` sendet `Home` und `Fn+→` sendet `End`. `Ctrl+Fn+→` erreicht Claude Code auf macOS nicht, daher hat eine MacBook-Tastatur standardmäßig keine funktionierende Tastenkombination zum Springen nach unten. Verwenden Sie stattdessen eine dieser Optionen:

* Klicken Sie auf die [Schaltfläche zum Springen nach unten](#auto-follow).
* Scrollen Sie mit dem Mausrad nach unten, um das Folgen fortzusetzen.
* Binden Sie `scroll:bottom` an eine Tastenkombination neu, die Ihre Tastatur senden kann.

Diese Aktionen können neu gebunden werden. Siehe [Scroll-Aktionen](/docs/de/keybindings#scroll-actions) für die vollständige Liste der Aktionsnamen, einschließlich Varianten für halbe und ganze Seiten, die standardmäßig nicht gebunden sind.

Während Sie nach oben gescrollt sind, zeigt eine schwache Kopfzeile oben im Gespräch die neueste Eingabeaufforderung an, die über die Ansicht hinaus gescrollt ist. Klicken Sie auf die Zeile, um zu dieser Eingabeaufforderung zu springen.

<h3 id="auto-follow">
  Automatisches Folgen
</h3>

Das Scrollen nach oben pausiert das automatische Folgen, damit neue Ausgabe Sie nicht zurück nach unten zieht. Eine `Zum unteren Ende springen`-Schaltfläche schwebt über der unteren Kante des Transkripts, während Sie nach oben gescrollt sind, und zeigt eine Anzahl wie `3 neue Nachrichten` an, wenn neue Ausgabe ankommt. Klicken Sie darauf, drücken Sie `Ctrl+End` oder scrollen Sie nach unten, um das Folgen fortzusetzen.

Während das automatische Folgen pausiert ist, bleibt die Ansicht auch dort, wo Sie sie gescrollt haben, wenn eine Antwort das Streaming beendet.

Der Tastaturhinweis der Schaltfläche spiegelt wider, was Ihre Tastatur senden kann. Auf macOS wird vorgeschlagen, zu klicken oder `Fn+↓` zum Scrollen zu verwenden, da `Ctrl+End` Claude Code von einer Mac-Tastatur aus nicht erreicht. Binden Sie [`scroll:bottom`](/docs/de/keybindings#scroll-actions) neu und die Schaltfläche zeigt Ihre Tastenkombination auf jeder Plattform an.

Auf einem Terminal, das zu schmal für die vollständige Beschriftung ist, verkürzt die Schaltfläche den Hinweis, anstatt ihn in die nächste Transkriptzeile umzubrechen.

Um das automatische Folgen vollständig auszuschalten, damit die Ansicht dort bleibt, wo Sie sie verlassen, öffnen Sie `/config` und setzen Sie Auto-Scroll auf aus. Mit deaktiviertem Auto-Scroll springt die Ansicht nie von selbst nach unten. Berechtigungsaufforderungen und andere Dialoge, die eine Antwort benötigen, scrollen unabhängig von dieser Einstellung in die Ansicht.

<h3 id="mouse-wheel-scrolling">
  Mausrad-Scrolling
</h3>

Das Mausrad-Scrolling erfordert, dass Ihr Terminal Mausereignisse an Claude Code weiterleitet. Die meisten Terminals tun dies, wenn eine Anwendung dies anfordert. iTerm2 macht es zu einer Pro-Profil-Einstellung: Wenn das Rad nichts tut, aber `PgUp` und `PgDn` funktionieren, öffnen Sie Einstellungen → Profile → Terminal und aktivieren Sie Mausberichte aktivieren. Die gleiche Einstellung ist auch erforderlich, damit Klick-zum-Erweitern und Textauswahl funktionieren.

Wenn sich das Mausrad-Scrolling langsam anfühlt, sendet Ihr Terminal möglicherweise ein Scroll-Ereignis pro physischer Kerbe ohne Multiplikator. Einige Terminals, wie Ghostty und iTerm2 mit aktiviertem schnellerem Scrolling, verstärken bereits Rad-Ereignisse. Andere, einschließlich des integrierten VS Code-Terminals, senden genau ein Ereignis pro Kerbe. Claude Code kann nicht erkennen, welches.

Setzen Sie `CLAUDE_CODE_SCROLL_SPEED`, um die Basis-Scroll-Distanz zu multiplizieren:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Ein Wert von `3` entspricht dem Standard in `vim` und ähnlichen Anwendungen. Die Einstellung akzeptiert jeden positiven Wert bis zu 20, einschließlich Bruchteile unter 1, wie `0.25`, um beschleunigtes Trackpad- und Mausrad-Scrolling in Terminals zu verlangsamen, die Rad-Ereignisse bereits verstärken.

Um die Scroll-Geschwindigkeit interaktiv anzupassen, führen Sie `/scroll-speed` aus. Der Dialog zeigt ein Lineal, das Sie scrollen können, während er offen ist, damit Sie die Änderung sofort spüren können. Drücken Sie `←` und `→`, um die Geschwindigkeit anzupassen, `r`, um auf den automatisch erkannten Standard zurückzusetzen, und `Enter`, um zu speichern. Der Dialog schreitet in ganzen Zahlen bis zu 10 vor, und auf Terminals, die feinere Kontrolle unterstützen, bietet er auch Viertelschritte bis zu 0,25.

Der Befehl schreibt den gleichen Wert, den die Umgebungsvariable `CLAUDE_CODE_SCROLL_SPEED` setzt, persistent in `~/.claude/settings.json`. Das Maximum des Dialogs ist 10: Wenn Sie einen höheren Wert über die Umgebungsvariable setzen, zeigt der Dialog 10 an, und das Speichern aus dem Dialog speichert 10. Der Befehl ist im JetBrains IDE-Terminal nicht verfügbar.

Unabhängig von der Basisgeschwindigkeit beschleunigt Claude Code die Scroll-Rate, wenn Sie das Rad schnell drehen, sodass eine schnelle Drehung mehr Distanz abdeckt als die gleiche Anzahl langsamer Kerben. Um die Beschleunigung auszuschalten und eine konstante Rate pro Kerbe beizubehalten, setzen Sie `wheelScrollAccelerationEnabled` auf `false` in [`settings.json`](/docs/de/settings-reference#all-settings). Diese Einstellung erfordert Claude Code v2.1.174 oder später.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Scrollen im JetBrains IDE-Terminal
</h3>

Im JetBrains IDE-Terminal wendet Claude Code seine eigene Scroll-Verarbeitung an und ignoriert `CLAUDE_CODE_SCROLL_SPEED`. Das Terminal sendet Scroll-Ereignisse mit einer viel höheren Rate als andere Emulatoren, daher ein anderswo abgestimmter Multiplikator hier zu viel ist.

In 2025.2 hat das Terminal auch Mausrad-Fehler, die fehlerhafte Pfeiltasten und Ereignisse in der falschen Richtung erzeugen. Claude Code erkennt diese zur Laufzeit und mindert sie automatisch ab, sodass Trackpad- und Mausrad-Scrolling ohne Konfiguration funktioniert. Für das beste Scroll-Erlebnis aktualisieren Sie auf 2025.3 oder später. Claude Code zeigt einen Hinweis beim ersten Scrollen an, wenn es den Fehler erkennt.

<h2 id="search-and-review-the-conversation">
  Suche und Überprüfung des Gesprächs
</h2>

`Ctrl+o` schaltet zwischen dem normalen Eingabemodus und dem Transkriptmodus um.

Für eine ruhigere Ansicht, die nur Ihre letzte Eingabe, eine einzeilige Zusammenfassung von Werkzeugaufrufen mit Bearbeitungs-Diffstats und die endgültige Antwort anzeigt, führen Sie `/focus` aus. Die Einstellung bleibt über Sitzungen hinweg erhalten. Führen Sie `/focus` erneut aus, um es auszuschalten.

Der Transkriptmodus bietet `less`-ähnliche Navigation und Suche:

| Taste                                  | Aktion                                                                                                                                               |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                    | Suche öffnen. Geben Sie ein, um Übereinstimmungen zu finden, `Enter` zum Akzeptieren, `Esc` zum Abbrechen und Wiederherstellen Ihrer Scroll-Position |
| `n` / `N`                              | Zur nächsten oder vorherigen Übereinstimmung springen. Funktioniert, nachdem Sie die Suchleiste geschlossen haben                                    |
| `j` / `k` oder `↑` / `↓`               | Eine Zeile scrollen                                                                                                                                  |
| `g` / `G` oder `Home` / `End`          | Zum Anfang oder Ende springen                                                                                                                        |
| `{` / `}`                              | Zur vorherigen oder nächsten Eingabe springen                                                                                                        |
| `Ctrl+u` / `Ctrl+d`                    | Eine halbe Seite scrollen                                                                                                                            |
| `Ctrl+b` / `Ctrl+f` oder `Space` / `b` | Eine ganze Seite scrollen                                                                                                                            |
| `Ctrl+o`, `Esc` oder `q`               | Transkriptmodus beenden und zur Eingabe zurückkehren                                                                                                 |

Die `Cmd+f` Ihres Terminals und die tmux-Suche können das Gespräch nicht sehen, da es sich im alternativen Bildschirmpuffer befindet, nicht im nativen Scrollback. Um den Inhalt an Ihr Terminal zurückzugeben, drücken Sie `Ctrl+o`, um zuerst in den Transkriptmodus zu wechseln, und dann:

* **`[`**: schreibt das vollständige Gespräch in den nativen Scrollback-Puffer Ihres Terminals mit allen erweiterten Werkzeugausgaben. Das Gespräch ist nun gewöhnlicher Text in Ihrem Terminal, sodass `Cmd+f`, tmux-Kopiermodus und alle anderen nativen Tools es durchsuchen oder auswählen können. Lange Sitzungen können einen Moment pausieren, während dies geschieht. Dies dauert an, bis Sie den Transkriptmodus mit `Esc` oder `q` beenden, was Sie zur Vollbilddarstellung zurückbringt. Das nächste `Ctrl+o` beginnt von vorne.
* **`v`**: schreibt das Gespräch in eine temporäre Datei und öffnet es in `$VISUAL` oder `$EDITOR`.

<h2 id="watch-your-changes-in-the-diff-panel">
  Beobachten Sie Ihre Änderungen im Diff-Panel
</h2>

Bei der Vollbildwiedergabe öffnet [`/diff`](/docs/de/interactive-mode#review-changes-with-%2Fdiff) ein Panel neben dem Gespräch, anstatt eines Viewers, den Sie schließen müssen, sodass Sie die Änderungen beobachten können, während Claude arbeitet. In einem breiten Terminal kann sich das Panel auch von selbst öffnen, sobald Claude mit der Bearbeitung von Dateien beginnt. [Diff-Panel](/docs/de/interactive-mode#diff-panel) behandelt, was es anzeigt, wie Sie es geschlossen halten und wie Sie ändern, wogegen es vergleicht.

<h2 id="clear-the-conversation">
  Unterhaltung löschen
</h2>

Führen Sie `/clear` aus, um eine neue Unterhaltung zu starten.

Wenn die Anzeige verzerrt oder teilweise leer aussieht, drücken Sie `Ctrl+L`, um den Bildschirm neu zu zeichnen. Das Neuzeichnen behält die Unterhaltung und Ihre Eingabe bei.

`Cmd+K` hat die gleiche Wirkung wie `Ctrl+L`, wenn Ihr Terminal es an Claude Code weiterleitet. iTerm2 und Terminal.app handhaben `Cmd+K` selbst und löschen ihren eigenen Bildschirm, und Claude Code erkennt den gelöschten Bildschirm und zeichnet die Unterhaltung neu. Vor v2.1.280, beginnend mit v2.1.260, löschte das Drücken von `Ctrl+L` oder `Cmd+K`, wo es Claude Code erreicht, den Bildschirm beim Vollbildrendering. Vor v2.1.238 führte das zweimalige Drücken von `Ctrl+L` innerhalb von zwei Sekunden `/clear` aus.

<h2 id="use-with-tmux">
  Verwendung mit tmux
</h2>

Vollbildrendering funktioniert innerhalb von tmux mit drei Einschränkungen.

Das Scrollen mit dem Mausrad erfordert tmux's Mausmodus. Wenn Ihre `~/.tmux.conf` ihn nicht bereits aktiviert, fügen Sie diese Zeile hinzu und laden Sie Ihre Konfiguration neu:

```bash theme={null}
set -g mouse on
```

Ohne Mausmodus gehen Rad-Ereignisse an tmux statt an Claude Code. Tastatur-Scrollen mit `PgUp` und `PgDn` funktioniert in beiden Fällen. Claude Code gibt einen einmaligen Hinweis beim Start aus, wenn es tmux mit deaktiviertem Mausmodus erkennt.

Vollbildrendering ist nicht kompatibel mit iTerm2's tmux-Integrationsmodus, dem Modus, den Sie mit `tmux -CC` aufrufen. Im Integrationsmodus rendert iTerm2 jeden tmux-Bereich als native Aufteilung, anstatt tmux zum Zeichnen im Terminal zu ermöglichen. Der Alternate-Screen-Buffer und die Mausverfolgung funktionieren dort nicht korrekt: das Mausrad tut nichts, und Doppelklick kann den Terminal-Status beschädigen. Aktivieren Sie kein Vollbildrendering in `tmux -CC`-Sitzungen. Reguläres tmux innerhalb von iTerm2 ohne `-CC` funktioniert einwandfrei.

tmux-Versionen bis zur Serie 3.6 implementieren keine synchronisierte Ausgabe, daher können Sie unter diesen Versionen mehr Flimmern während Neuzeichnungen sehen als beim direkten Ausführen von Claude Code in Ihrem Terminal. Claude Code prüft das Terminal beim Start auf Unterstützung für synchronisierte Ausgabe und nutzt sie, wenn das Terminal dies meldet. Wenn Sie Flimmern unter tmux sehen, aktualisieren Sie auf das neueste tmux oder führen Sie Claude Code in einem eigenen Terminal-Tab außerhalb von tmux aus.

<h2 id="keep-native-text-selection">
  Native Textauswahl beibehalten
</h2>

Mauserfassung ist der häufigste Reibungspunkt, besonders über SSH oder innerhalb von tmux. Wenn Claude Code Mausereignisse erfasst, funktioniert die native Kopieren-bei-Auswahl-Funktion Ihres Terminals nicht mehr. Die Auswahl, die Sie mit Klick-und-Ziehen vornehmen, existiert innerhalb von Claude Code, nicht im Auswahlpuffer Ihres Terminals, daher sehen tmux-Kopiermodus, Kitty-Hinweise und ähnliche Tools sie nicht.

Claude Code schreibt die Auswahl in Ihre Systemzwischenablage, und der Pfad, den es verwendet, hängt von Ihrem Setup ab. Bei einer lokalen Sitzung führt es ein natives Zwischenablage-Tool aus:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` auf Wayland oder `xclip` oder `xsel` auf X11, je nachdem, was installiert ist. Claude Code schreibt sowohl die Zwischenablage als auch die PRIMARY-Auswahl, damit das Einfügen mit der mittleren Maustaste funktioniert.
* **Windows und WSL**: PowerShell `Set-Clipboard`

Innerhalb von tmux schreibt es auch in den tmux-Paste-Puffer. Über SSH fällt es auf OSC-52-Escape-Sequenzen zurück. Innerhalb von GNU screen kopiert Claude Code auch lange Auswahlen in die Zwischenablage. Vor v2.1.219 druckte GNU screen, wenn Sie eine Auswahl länger als etwa 570 Zeichen kopierten, Base64-Text in das Fenster. Claude Code zeigt nach jedem Kopieren einen Toast an, der Ihnen mitteilt, welchen Pfad es verwendet hat.

Einige Terminals blockieren OSC 52 standardmäßig. iTerm2 blockiert es, bis Sie Settings → General → Selection → Applications in terminal may access clipboard aktivieren; das Ausführen von [`/terminal-setup`](/docs/de/terminal-config) in iTerm2 aktiviert dies für Sie.

Für eine einmalige native Auswahl hängt die zu verwendende Taste von Ihrem Terminal ab:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor und Devin Desktop**: `Shift` oder `Option` auf macOS mit der Einstellung `terminal.integrated.macOptionClickForcesSelection` aktiviert
* **Die meisten anderen Terminals**: `Shift`

Halten Sie diese Taste gedrückt, während Sie klicken und ziehen. Ihr Terminal verwaltet die Auswahl selbst, anstatt sie an Claude Code zu übergeben, daher funktionieren Kopier-Tastenkombinationen wie `Cmd+C` auf dem, was Sie auswählen. Claude Code zeigt auch den richtigen Schlüssel in seinem On-Screen-Hinweis an.

Über SSH oder innerhalb von tmux kann Claude Code das Terminal, von dem aus Sie sich verbinden, nicht immer erkennen, daher listet der Hinweis stattdessen die Kandidatentasten auf.

Wenn Sie sich die ganze Zeit auf native Auswahl verlassen, setzen Sie `CLAUDE_CODE_DISABLE_MOUSE=1`, um die Mauserfassung zu deaktivieren und gleichzeitig das flimmerfreie Rendering und den flachen Speicher zu behalten:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Mit deaktivierter Mauserfassung funktioniert das Tastatur-Scrollen mit `PgUp`, `PgDn`, `Ctrl+Home` und `Ctrl+End` immer noch, und Ihr Terminal verwaltet die Auswahl nativ. Sie verlieren das Klicken zum Positionieren des Cursors, das Klicken zum Erweitern der Tool-Ausgabe, das URL-Klicken und das Rad-Scrollen innerhalb von Claude Code.

Um das Rad-Scrollen zu behalten, aber das Klicken, Ziehen und Hover-Handling auszuschalten, setzen Sie stattdessen `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`. Erfordert Claude Code v2.1.195 oder später. `CLAUDE_CODE_DISABLE_MOUSE` hat Vorrang, wenn beide Variablen gesetzt sind.

Mit deaktiviertem Klicken erfasst Claude Code die Maus immer noch, daher scrollen das Rad und das Trackpad das Gespräch, aber linke Klicks funktionieren nicht innerhalb von Claude Code. Sie müssen immer noch die Taste Ihres Terminals halten, um native Klick-und-Ziehen-Auswahl zu ermöglichen. Rechtsklick und Mittelsklick-Einfügen funktionieren weiterhin auf Terminals, die sie unterstützen.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Veralteter oder falsch platzierter Text auf dem Bildschirm
</h3>

Das Vollbildrendering sendet nur die Zellen, die sich zwischen den Frames geändert haben. Einige Terminals, am häufigsten Windows Terminal und andere ConPTY-gestützte Hosts, führen diese positionierten Schreibvorgänge falsch zusammen und hinterlassen Fragmente der früheren Ausgabe auf dem Bildschirm, bis Sie das Fenster vergrößern.

Setzen Sie [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/de/env-vars), um bei jedem Frame alle Zellen neu zu zeichnen, anstatt inkrementelle Updates zu senden.

Unter Windows PowerShell:

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

Unter macOS oder Linux:

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

Unter Windows aktiviert Claude Code das vollständige Neuzeichnen bereits automatisch für Hintergrundsitzungen und [Agent-Ansicht](/docs/de/agent-view), daher müssen Sie die Variable nur für eine interaktive Vollbildsitzung setzen, die Sie direkt gestartet haben.

<h3 id="fullscreen-renderer-didnt-finish-starting">
  `Claude Code's fullscreen renderer didn't finish starting last time` wird beim Start angezeigt
</h3>

Wenn eine Vollbildsitzung auf diesem Computer abstürzt, bevor sie erfolgreich gestartet wurde, startet Claude Code Ihre nächste Sitzung im klassischen Renderer und gibt eine von zwei Zeilen aus. Eine Sitzung hat erfolgreich gestartet, sobald sie ihren ersten Frame gezeichnet hat und dann entweder 10 Sekunden lang aktiv blieb oder Sie sie mit `/exit`, Strg+C oder Strg+D beendet haben. Die Zeile, die Sie sehen, teilt Ihnen mit, was Claude Code nach dieser Sitzung tut:

* Nach einem fehlgeschlagenen Start sehen Sie `Claude Code's fullscreen renderer didn't finish starting last time on this machine`. Claude Code versucht das Vollbildrendering in der nächsten Sitzung, die Sie starten, erneut
* Nach zwei fehlgeschlagenen Starts sehen Sie `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`. Claude Code verwendet weiterhin den klassischen Renderer, bis Sie Claude Code aktualisieren oder `/tui fullscreen` ausführen, und gibt in diesen späteren Sitzungen nichts aus

Um zu bestätigen, dass ein fehlgeschlagener Start der Grund ist, warum Sie sich im klassischen Renderer befinden, führen Sie `/tui` ohne Argument aus. Während ein fehlgeschlagener Start der Grund ist, sagt die Zeile `Current renderer` dies aus.

Um den klassischen Renderer beizubehalten, führen Sie `/tui default` aus, wodurch die `tui`-Einstellung gespeichert wird, ohne neu zu starten. Um das Vollbildrendering erneut zu versuchen, führen Sie `/tui fullscreen` aus. Wenn diese Sitzung auch nicht erfolgreich startet, [melden Sie das Problem](#research-preview).

Vor v2.1.236 startete Claude Code Sitzungen nach einem fehlgeschlagenen Start weiterhin im Vollbildrendering.

<h4 id="how-claude-code-counts-failed-starts">
  Wie Claude Code fehlgeschlagene Starts zählt
</h4>

* Sitzungen, die zählen: nur Sitzungen, die im Vollbildrendering gestartet wurden, weil Ihre `tui`-Einstellung dies vorsieht, weil Sie den [Startup-Dialog](#fullscreen-by-default) akzeptiert haben, oder weil Claude Code Sie standardmäßig im Vollbildmodus startet
* `CLAUDE_CODE_NO_FLICKER=1`: wenn Sie es setzen, rendert Claude Code diese Sitzung auch nach einem fehlgeschlagenen Start im Vollbildmodus und zählt ihn nicht
* Zähler zurücksetzen: Claude Code zählt fehlgeschlagene Starts pro Claude Code-Version, und ein erfolgreicher Vollbildstart setzt den Zähler zurück
* Startup-Dialog: wenn Sie den Dialog akzeptiert haben und die neu gestartete Sitzung abstürzte, gibt Claude Code keine der beiden Zeilen aus und zeigt den Dialog auf dieser Claude Code-Version nicht erneut an

<h2 id="research-preview">
  Forschungsvorschau
</h2>

Die Vollbildwiedergabe ist eine Forschungsvorschau-Funktion. Sie wurde auf gängigen Terminal-Emulatoren getestet, aber Sie können auf weniger gängigen Terminals oder ungewöhnlichen Konfigurationen auf Wiedergabeprobleme stoßen.

Wenn Sie auf ein Problem stoßen, führen Sie `/feedback` in Claude Code aus, um es zu melden, oder öffnen Sie ein Problem im [claude-code GitHub-Repository](https://github.com/anthropics/claude-code/issues). Geben Sie den Namen und die Version Ihres Terminal-Emulators an.

Um die Vollbildwiedergabe auszuschalten, führen Sie `/tui default` aus oder heben Sie die Einstellung `CLAUDE_CODE_NO_FLICKER` auf, falls Sie sie auf diese Weise aktiviert haben. Wenn Sie mit `/tui default` zurückwechseln, zeigt Claude Code möglicherweise zunächst eine optionale Feedback-Eingabeaufforderung an, die fragt, was Sie zum Wechsel bewogen hat. Geben Sie einen Grund ein und drücken Sie `Enter`, um ihn zu senden, oder drücken Sie `Esc`, um zu überspringen. Die CLI wird in jedem Fall in den klassischen Renderer neu gestartet. Um den klassischen Renderer unabhängig von der gespeicherten `tui`-Einstellung zu erzwingen, setzen Sie `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. Der klassische Renderer behält die Konversation im nativen Scrollback Ihres Terminals bei, sodass `Cmd+f` und der tmux-Kopiermodus wie gewohnt funktionieren.

Hintergrundsitzungen, die aus der [Agent-Ansicht](/docs/de/agent-view) oder `claude attach` geöffnet werden, verwenden immer die Vollbildwiedergabe. Das angehängte Terminal wechselt in den alternativen Bildschirmpuffer, um die Sitzung anzuzeigen, und der klassische Renderer hat dort keinen Scrollback oder keine Mausbehandlung, daher gelten die `tui`-Einstellung und `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` nicht für sie.
