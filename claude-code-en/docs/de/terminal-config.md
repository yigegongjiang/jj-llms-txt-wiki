> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurieren Sie Ihr Terminal für Claude Code

> Beheben Sie Shift+Enter für Zeilenumbrüche, erhalten Sie einen Terminal-Gong, wenn Claude fertig ist, konfigurieren Sie tmux, passen Sie das Farbschema an, und aktivieren Sie den Vim-Modus in der Claude Code CLI.

Claude Code funktioniert in jedem Terminal ohne Konfiguration. Diese Seite ist für den Fall, dass sich etwas nicht so verhält, wie Sie es erwarten. Finden Sie Ihr Symptom unten. Wenn alles bereits richtig funktioniert, benötigen Sie diese Seite nicht.

* [Shift+Enter sendet ab, anstatt einen Zeilenumbruch einzufügen](#enter-multiline-prompts)
* [Option-Taste-Verknüpfungen funktionieren nicht auf macOS](#enable-option-key-shortcuts-on-macos)
* [Kein Ton oder Benachrichtigung, wenn Claude fertig ist](#get-a-terminal-bell-or-notification)
* [Sie führen Claude Code in tmux aus](#configure-tmux)
* [Rücktaste löscht ein ganzes Wort unter Windows](#fix-backspace-deleting-a-whole-word-on-windows)
* [Anzeige flackert oder Scrollback springt](#switch-to-fullscreen-rendering)
* [Sie möchten Vim-Tasten in der Eingabeaufforderung](#edit-prompts-with-vim-keybindings)

Diese Seite behandelt das Konfigurieren Ihres Terminals, um die richtigen Signale an Claude Code zu senden. Um zu ändern, auf welche Tasten Claude Code selbst reagiert, siehe stattdessen [Tastenbelegungen](/docs/de/keybindings).

<h2 id="enter-multiline-prompts">
  Mehrzeilige Eingabeaufforderungen eingeben
</h2>

Durch Drücken der Eingabetaste wird Ihre Nachricht abgesendet. Um einen Zeilenumbruch hinzuzufügen, ohne abzusenden, drücken Sie Strg+J, oder geben Sie `\` ein und drücken dann die Eingabetaste. Beide Methoden funktionieren in jedem Terminal ohne Setup.

In den meisten Terminals können Sie auch Umschalt+Eingabe drücken, aber die Unterstützung variiert je nach Terminal-Emulator:

| Terminal                                                                                                | Umschalt+Eingabe für Zeilenumbruch                                   |
| :------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal                                 | Funktioniert ohne Setup                                              |
| Andere Terminals, die das Kitty-Tastaturprotokoll unterstützen, wie foot und Alacritty 0.16 oder später | Funktioniert ohne Setup. Erfordert Claude Code v2.1.269 oder später  |
| VS Code, Cursor, Devin Desktop, Alacritty vor 0.16, Zed                                                 | Führen Sie `/terminal-setup` einmal aus                              |
| gnome-terminal, JetBrains IDEs wie PyCharm und Android Studio                                           | Nicht verfügbar; verwenden Sie Strg+J oder `\` und dann Eingabetaste |

Für VS Code, Cursor, Devin Desktop, Alacritty vor 0.16 und Zed schreibt `/terminal-setup` eine Umschalt+Eingabe-Tastenbindung in die Konfigurationsdatei des Terminals. Bei der ersten Ausführung wird eine Bestätigung wie `Installed VSCode terminal Shift+Enter key binding` angezeigt. Vorhandene Bindungen bleiben erhalten; wenn Sie eine Meldung wie `VSCode terminal Shift+Enter key binding already configured` sehen, wurde keine Änderung vorgenommen. Führen Sie `/terminal-setup` direkt im Host-Terminal aus, nicht innerhalb von tmux oder screen, da es in die Konfiguration des Host-Terminals schreiben muss.

In VS Code, Cursor und Devin Desktop aktualisiert `/terminal-setup` auch zwei Editor-Einstellungen: Es setzt `terminal.integrated.gpuAcceleration` auf `"off"`, um verzerrten Text im integrierten Terminal zu verhindern, und es setzt `terminal.integrated.mouseWheelScrollSensitivity` für sanfteres Scrollen im [Vollbildmodus](/docs/de/fullscreen). Um die GPU-Beschleunigungsänderung rückgängig zu machen, setzen Sie sie auf `"auto"` zurück und laden Sie das Editor-Fenster neu.

In Zed aktualisiert `/terminal-setup` Ihre `keymap.json` an Ort und Stelle:

* Wenn die Keymap bereits Bindungen hat und keine davon eine Terminal-`shift-enter` ist, sichert Claude Code diese zunächst in einer Kopie im selben Verzeichnis, z. B. `keymap.json.1a2b3c4d.bak`, und führt dann die Umschalt+Eingabe-Bindung in Ihre Keymap ein, wobei Ihre anderen Tastenbindungen und Kommentare erhalten bleiben
* Wenn Claude Code die Keymap nicht lesen oder analysieren kann, sie nicht sichern kann oder das zusammengeführte Ergebnis nicht überprüfen kann, [lässt es die Datei unverändert und gibt den Tastenbindungsblock zum manuellen Hinzufügen aus](/docs/de/errors#terminal-setup-left-your-zed-keymap-unchanged)

Wenn Sie innerhalb von tmux ausgeführt werden, erfordert Umschalt+Eingabe auch die [tmux-Konfiguration unten](#configure-tmux), selbst wenn das äußere Terminal dies unterstützt.

Um Zeilenumbruch an eine andere Taste zu binden oder das Verhalten zu tauschen, sodass Eingabetaste einen Zeilenumbruch einfügt und Umschalt+Eingabe absendet, ordnen Sie die Aktionen `chat:newline` und `chat:submit` in Ihrer [Tastenbindungsdatei](/docs/de/keybindings) zu.

<h2 id="enable-option-key-shortcuts-on-macos">
  Option-Taste-Tastenkombinationen auf macOS aktivieren
</h2>

Einige Claude Code-Tastenkombinationen verwenden die Option-Taste, z. B. Option+Eingabe für einen Zeilenumbruch oder Option+P zum Wechsel von Modellen. Auf macOS senden die meisten Terminals die Option-Taste standardmäßig nicht als Modifizierer, daher funktionieren diese Tastenkombinationen nicht, bis Sie dies aktivieren. Die Terminal-Einstellung dafür wird normalerweise als „Use Option as Meta Key" bezeichnet; Meta ist der historische Unix-Name für die Taste, die jetzt als Option oder Alt bezeichnet wird.

<Tabs>
  <Tab title="Apple Terminal">
    Öffnen Sie Einstellungen → Profile → Tastatur und aktivieren Sie „Use Option as Meta Key".

    Wenn Sie die erste Einrichtungsaufforderung des Terminals von Claude Code akzeptiert haben, ist dies bereits erledigt. Diese Aufforderung führt `/terminal-setup` für Sie aus, wodurch Option als Meta aktiviert und die akustische Benachrichtigung in Ihrem Apple Terminal-Profil deaktiviert wird.

    Im [Bildschirmlesemodus](/docs/de/accessibility) lässt `/terminal-setup` die Benachrichtigungseinstellung unverändert, sodass die Terminal-Benachrichtigung hörbar bleibt. Vor v2.1.211 deaktivierte `/terminal-setup` die Benachrichtigung auch im Bildschirmlesemodus. Wenn ein früherer Durchlauf die Benachrichtigung deaktiviert hat, aktivieren Sie sie unter Einstellungen → Profile → Erweitert → „Audible bell" erneut.
  </Tab>

  <Tab title="iTerm2">
    Öffnen Sie Einstellungen → Profile → Tasten → Allgemein und setzen Sie die linke Option-Taste und die rechte Option-Taste auf „Esc+".

    Das Ausführen von `/terminal-setup` in iTerm2 aktiviert „Applications in terminal may access clipboard" unter Einstellungen → Allgemein → Auswahl, damit der Befehl `/copy` in Ihre Systemzwischenablage schreiben kann. Der Befehl erkennt iTerm2 auch, wenn er von innerhalb von tmux ausgeführt wird. Starten Sie iTerm2 neu, damit die Änderung wirksam wird.
  </Tab>

  <Tab title="VS Code">
    Fügen Sie `"terminal.integrated.macOptionIsMeta": true` zu Ihren VS Code-Einstellungen hinzu.
  </Tab>
</Tabs>

Für Ghostty, Kitty und andere Terminals suchen Sie in der Konfigurationsdatei des Terminals nach einer Option-als-Alt- oder Option-als-Meta-Einstellung.

<h2 id="get-a-terminal-bell-or-notification">
  Terminalglockenton oder Benachrichtigung erhalten
</h2>

Wenn Claude eine Aufgabe abschließt oder bei einer Berechtigungsaufforderung pausiert und Sie vom Terminal entfernt zu sein scheinen, wird ein Benachrichtigungsereignis ausgelöst. Siehe [wann jeder Benachrichtigungstyp ausgelöst wird](/docs/de/hooks#notification) für den genauen Zeitpunkt. Wenn Sie dies als Terminalglockenton oder Desktop-Benachrichtigung anzeigen, können Sie zu anderen Aufgaben wechseln, während eine lange Aufgabe läuft.

Standardmäßig sendet Claude Code eine Desktop-Benachrichtigung nur in Ghostty, Kitty und iTerm2. In anderen Terminals setzen Sie [`preferredNotifChannel`](/docs/de/settings-reference#preferrednotifchannel) auf `"terminal_bell"`, um stattdessen den Terminalglockenton zu aktivieren, oder konfigurieren Sie einen [Notification-Hook](#play-a-sound-with-a-notification-hook) für einen benutzerdefinierten Sound oder Befehl. Der folgende Einträge in den Einstellungen aktiviert den Terminalglockenton:

```json ~/.claude/settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

Die Desktop-Benachrichtigung erreicht Ihren lokalen Computer über SSH, sodass eine Remote-Sitzung Sie immer noch benachrichtigen kann. Ghostty und Kitty leiten sie ohne weitere Einrichtung an Ihr Betriebssystem-Benachrichtigungscenter weiter. iTerm2 erfordert, dass Sie die Weiterleitung aktivieren:

<Steps>
  <Step title="Öffnen Sie die iTerm2-Benachrichtigungseinstellungen">
    Gehen Sie zu Einstellungen → Profile → Terminal.
  </Step>

  <Step title="Aktivieren Sie Benachrichtigungen">
    Aktivieren Sie „Notification Center Alerts", klicken Sie dann auf „Filter Alerts" und aktivieren Sie „Send escape sequence-generated alerts".
  </Step>
</Steps>

Wenn Benachrichtigungen immer noch nicht angezeigt werden, bestätigen Sie, dass Ihre Terminalanwendung in Ihren Betriebssystem-Einstellungen Benachrichtigungsberechtigung hat, und wenn Sie in tmux ausgeführt werden, [aktivieren Sie Passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Sound mit einem Notification-Hook abspielen
</h3>

In jedem Terminal können Sie einen [Notification-Hook](/docs/de/hooks-guide#get-notified-when-claude-needs-input) konfigurieren, um einen Sound abzuspielen oder einen benutzerdefinierten Befehl auszuführen, wenn Claude Ihre Aufmerksamkeit benötigt. Hooks werden neben der integrierten Benachrichtigung ausgeführt, anstatt sie zu ersetzen, sodass Terminals, die keine Desktop-Benachrichtigung erhalten, wie Warp oder das in VS Code integrierte Terminal, einen Hook verwenden oder `preferredNotifChannel` stattdessen auf `"terminal_bell"` setzen können.

Das folgende Beispiel spielt einen Systemsound auf macOS ab. Der verlinkte Leitfaden enthält Desktop-Benachrichtigungsbefehle für macOS, Linux und Windows.

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  tmux konfigurieren
</h2>

Wenn Claude Code innerhalb von tmux ausgeführt wird, funktionieren standardmäßig zwei Dinge nicht: Shift+Enter sendet ab, anstatt einen Zeilenumbruch einzufügen, und Desktop-Benachrichtigungen sowie die [Fortschrittsleiste](/docs/de/settings-reference#terminalprogressbarenabled) erreichen niemals das äußere Terminal. Fügen Sie diese Zeilen zu `~/.tmux.conf` hinzu und führen Sie dann `tmux source-file ~/.tmux.conf` aus, um sie auf den laufenden Server anzuwenden:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

Die `allow-passthrough`-Zeile ermöglicht es, dass Benachrichtigungen und Fortschrittsaktualisierungen das äußere Terminal erreichen, anstatt von tmux aufgenommen zu werden. Die `extended-keys`-Zeilen ermöglichen es tmux, Shift+Enter von einfachem Enter zu unterscheiden, damit die Zeilenumbruch-Verknüpfung funktioniert.

<h2 id="fix-backspace-deleting-a-whole-word-on-windows">
  Behebung: Rücktaste löscht auf Windows ein ganzes Wort
</h2>

Unter Windows liest Claude Code eine Rücktaste, die als `^H` ankommt, als Strg+Rücktaste, was [das vorherige Wort löscht](/docs/de/interactive-mode#text-editing), außer wenn `TERM_PROGRAM` `mintty` oder `TERM` `cygwin` ist. Auf macOS und Linux liest Claude Code sie als einfache Rücktaste.

Wenn jedes Drücken der Rücktaste ein ganzes Wort löscht, sendet Ihr Terminal `^H` für die einfache Rücktaste. Setzen Sie [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE=0`](/docs/de/env-vars). Rücktaste und Strg+H löschen dann jeweils ein Zeichen. Wenn Strg+Rücktaste auf macOS oder Linux nur ein Zeichen löscht, weil Ihr Terminal `^H` dafür sendet, setzen Sie die Variable stattdessen auf `1`.

<h2 id="match-the-color-theme">
  Farbschema anpassen
</h2>

Verwenden Sie den Befehl `/theme` oder die Designauswahl in `/config`, um ein Claude Code-Design auszuwählen, das zu Ihrem Terminal passt. Wenn Sie die Option „auto" auswählen, wird der helle oder dunkle Hintergrund Ihres Terminals erkannt, sodass das Design den Änderungen des Betriebssystems folgt, wenn Ihr Terminal dies tut. Claude Code steuert nicht das Farbschema des Terminals selbst, das von der Terminalanwendung festgelegt wird.

Um anzupassen, was am unteren Rand der Benutzeroberfläche angezeigt wird, konfigurieren Sie eine [benutzerdefinierte Statuszeile](/docs/de/statusline), die das aktuelle Modell, das Arbeitsverzeichnis, den Git-Branch oder andere Kontextinformationen anzeigt.

<h3 id="create-a-custom-theme">
  Benutzerdefiniertes Design erstellen
</h3>

Zusätzlich zu den integrierten Voreinstellungen listet `/theme` alle benutzerdefinierten Designs auf, die Sie definiert haben, sowie alle Designs, die von installierten [Plugins](/docs/de/plugins/components#themes-and-output-styles) beigetragen wurden. Wählen Sie **Neues benutzerdefiniertes Design…** am Ende der Liste, um eines interaktiv zu erstellen: Sie benennen das Design und wählen dann einzelne Farbtoken aus, um sie zu überschreiben. Drücken Sie `Ctrl+E`, während ein benutzerdefiniertes Design hervorgehoben ist, um es zu bearbeiten.

Jedes benutzerdefinierte Design ist eine JSON-Datei in `~/.claude/themes/`. Der Dateiname ohne die `.json`-Erweiterung ist der Slug des Designs, und das Auswählen des Designs speichert `custom:<slug>` als Ihre Design-Einstellung. Die Datei hat drei optionale Felder:

| Feld        | Typ    | Beschreibung                                                                                                                                                        |
| :---------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`      | string | Anzeigebezeichnung in `/theme`. Standardmäßig der Dateiname-Slug                                                                                                    |
| `base`      | string | Integrierte Voreinstellung, von der das Design ausgeht: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi` oder `light-ansi`. Standardmäßig `dark` |
| `overrides` | object | Zuordnung von Farbtoken-Namen zu Farbwerten. Token, die hier nicht aufgelistet sind, fallen auf die Basisvoreinstellung zurück                                      |

Farbwerte akzeptieren `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)` oder `ansi:<name>`, wobei `<name>` einer der 16 standardmäßigen ANSI-Farbnamen wie `red` oder `cyanBright` ist. Unbekannte Token und ungültige Farbwerte werden ignoriert, sodass ein Tippfehler das Rendering nicht beschädigen kann.

Das folgende Beispiel definiert ein Design, das die dunkle Voreinstellung beibehält, aber die Eingabeaufforderungs-Akzentfarbe, Fehlertext und Erfolgstexte neu färbt:

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code überwacht `~/.claude/themes/` und lädt neu, wenn eine Datei hinzugefügt oder geändert wird, sodass Änderungen in Ihrem Editor auf eine laufende Sitzung angewendet werden, ohne einen Neustart erforderlich zu machen. Wenn der Ordner `~/.claude/themes/` nicht existierte, als Claude Code gestartet wurde, starten Sie einmal neu, nachdem Sie Ihre erste Design-Datei erstellt haben. Danach werden Änderungen ohne Neustart angewendet.

Die folgende Referenz behandelt die Token, die Sie in `overrides` festlegen können. Der interaktive Editor in `/theme` zeigt die gleichen Token mit einer Live-Vorschau sowie einige spezialisierte Akzente wie Onboarding-Bildschirmfarben, die hier weggelassen sind.

<Accordion title="Farbtoken-Referenz">
  Das folgende Beispiel kombiniert Token aus mehreren der folgenden Gruppen: der Brand-Akzent, der Plan Mode-Rahmen, die Diff-Hintergründe und der Nachrichtenhintergrund.

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    Text- und Akzentfarben
  </h4>

  Steuern Sie den primären Brand-Akzent und die Vordergrund-Textschattierungen, die in der gesamten Benutzeroberfläche verwendet werden.

  | Token         | Steuert                                                                         |
  | :------------ | :------------------------------------------------------------------------------ |
  | `claude`      | Primärer Brand-Akzent, verwendet für den Spinner und die Assistent-Bezeichnung  |
  | `text`        | Standard-Vordergrundtext                                                        |
  | `inverseText` | Text, der auf einem farbigen Hintergrund gezeichnet wird, z. B. Statusabzeichen |
  | `inactive`    | Sekundärer Text wie Hinweise, Zeitstempel und deaktivierte Elemente             |
  | `subtle`      | Schwache Rahmen und de-betonte sekundäre Texte                                  |
  | `suggestion`  | Autocomplete-Vorschläge und Auswahlhervorhebung in Auswahlfeldern               |
  | `permission`  | Dialog-Rahmen, einschließlich Berechtigungsaufforderungen und Auswahlfelder     |
  | `remember`    | Speicher- und `CLAUDE.md`-Indikatoren                                           |

  <h4 id="status-colors">
    Statusfarben
  </h4>

  Signalisieren Sie Erfolgs-, Fehler- und Warnzustände über Nachrichten und Indikatoren.

  | Token     | Steuert                                                 |
  | :-------- | :------------------------------------------------------ |
  | `success` | Erfolgsmeldungen und bestandene Überprüfungen           |
  | `error`   | Fehlermeldungen und Fehler                              |
  | `warning` | Warnungen, Vorsichtsmeldungen und der Auto-Modus-Rahmen |
  | `merged`  | Status der zusammengeführten Pull-Anfrage               |

  <h4 id="input-box-and-mode-indicators">
    Eingabefeld und Modusindikatoren
  </h4>

  Legen Sie die Rahmenfarbe des Eingabefelds und den Akzent fest, der angezeigt wird, während ein Berechtigungsmodus oder Indikator aktiv ist.

  | Token          | Steuert                                                                                                                                                                                                    |
  | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | `promptBorder` | Eingabefeld-Rahmen                                                                                                                                                                                         |
  | `planMode`     | Plan Mode-Akzent, Plan-Nachrichten und Plan-Mode-Dialoge                                                                                                                                                   |
  | `autoAccept`   | Accept-edits-Modus-Akzent                                                                                                                                                                                  |
  | `bashBorder`   | Eingabefeld-Rahmen beim Eingeben eines `!` Shell-Befehls                                                                                                                                                   |
  | `ide`          | IDE-Verbindungsindikator                                                                                                                                                                                   |
  | `fastMode`     | Fast Mode-Indikator                                                                                                                                                                                        |
  | `effortUltra`  | Das `ultracode`-Tag auf dem Eingabefeld-Rahmen, während [ultracode](/docs/de/model-config#adjust-effort-level) aktiviert ist. Ihre Überschreibung dieser Farbe wird in Claude Code v2.1.239 oder später wirksam |

  <h4 id="diff-rendering">
    Diff-Rendering
  </h4>

  Färben Sie hinzugefügte und entfernte Code in Dateibearbeitungen und Überprüfungen.

  | Token               | Steuert                                                                                                           |
  | :------------------ | :---------------------------------------------------------------------------------------------------------------- |
  | `diffAdded`         | Hintergrund hinzugefügter Zeilen                                                                                  |
  | `diffRemoved`       | Hintergrund entfernter Zeilen                                                                                     |
  | `diffAddedDimmed`   | Hintergrund hinzugefügter Zeilen im abgedunkelten Diff, das angezeigt wird, nachdem Sie eine Bearbeitung ablehnen |
  | `diffRemovedDimmed` | Hintergrund entfernter Zeilen im abgedunkelten Diff, das angezeigt wird, nachdem Sie eine Bearbeitung ablehnen    |
  | `diffAddedWord`     | Hervorhebung auf Wortebene innerhalb einer hinzugefügten Zeile                                                    |
  | `diffRemovedWord`   | Hervorhebung auf Wortebene innerhalb einer entfernten Zeile                                                       |

  <h4 id="fullscreen-mode">
    Vollbildmodus
  </h4>

  Claude Code malt `userMessageBackground`, `bashMessageBackgroundColor` und `memoryBackgroundColor` sowohl im Standard- als auch im Vollbild-Renderer. Es verwendet `userMessageBackgroundHover` und `selectionBg` nur im [Vollbild-Rendering-Modus](/docs/de/fullscreen).

  | Token                        | Steuert                                                       |
  | :--------------------------- | :------------------------------------------------------------ |
  | `userMessageBackground`      | Hintergrund hinter Ihren Nachrichten im Transkript            |
  | `userMessageBackgroundHover` | Hintergrund hinter einer Nachricht beim Hovern oder Erweitern |
  | `bashMessageBackgroundColor` | Hintergrund hinter `!` Shell-Befehls-Einträgen im Transkript  |
  | `memoryBackgroundColor`      | Hintergrund hinter `#` Speicher-Einträgen im Transkript       |
  | `selectionBg`                | Hintergrund von mit der Maus ausgewähltem Text                |

  <h4 id="usage-meter-and-speaker-labels">
    Nutzungsmesser und Sprecherkennzeichnungen
  </h4>

  Passen Sie den Balken an, der in der `/usage`-Ansicht angezeigt wird, und die Kennzeichnungen, die Ihre Nachrichten von Claudes unterscheiden.

  | Token              | Steuert                                                    |
  | :----------------- | :--------------------------------------------------------- |
  | `rate_limit_fill`  | Gefüllter Teil des Nutzungsmessers                         |
  | `rate_limit_empty` | Ungefüllter Teil des Nutzungsmessers                       |
  | `briefLabelYou`    | Farbe der `You`-Kennzeichnung auf Ihren Nachrichten        |
  | `briefLabelClaude` | Farbe der `Claude`-Kennzeichnung auf Assistent-Nachrichten |

  <h4 id="shimmer-variants-and-subagent-colors">
    Shimmer-Varianten und Subagent-Farben
  </h4>

  Mehrere Token haben eine gepaarte Shimmer-Variante, die die hellere Farbe liefert, die im animierten Farbverlauf des Spinners verwendet wird. Überschreiben Sie den Shimmer zusammen mit seinem Basis-Token, wenn die Animation nicht übereinstimmt.

  * `claude` und `claudeShimmer`
  * `warning` und `warningShimmer`
  * `permission` und `permissionShimmer`
  * `promptBorder` und `promptBorderShimmer`
  * `inactive` und `inactiveShimmer`
  * `fastMode` und `fastModeShimmer`

  Jeder [Subagent](/docs/de/sub-agents) und jede parallele Aufgabe wird in einer von acht benannten Farben angezeigt, damit Sie sie im Transkript unterscheiden können. Die Token-Namen folgen dem Muster `<color>_FOR_SUBAGENTS_ONLY`, wobei `<color>` `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` oder `cyan` ist. Überschreiben Sie diese, um zu ändern, wie jede benannte Farbe aussieht. Beispielsweise wird ein Subagent mit `color: blue` in seiner Definition mit dem Wert `blue_FOR_SUBAGENTS_ONLY` gezeichnet.

  Claude Code rendert das Schlüsselwort [`ultrathink`](/docs/de/model-config#use-ultrathink-for-one-off-deep-reasoning) in der Eingabeaufforderung mit einem siebenfarbigen Regenbogenfarbverlauf. Die Token-Namen folgen dem Muster `rainbow_<color>` und `rainbow_<color>_shimmer`, wobei `<color>` `red`, `orange`, `yellow`, `green`, `blue`, `indigo` oder `violet` ist.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Zu Vollbildrendering wechseln
</h2>

Im [Bildschirmlesemodus](/docs/de/accessibility) gilt dieser Abschnitt nicht. Claude Code wird immer als einfacher scrollender Text dargestellt, außer in angehängten [Hintergrundsitzungen](/docs/de/agent-view), und wenn Sie `/tui fullscreen` in einer anderen Sitzung ausführen, gibt Claude Code stattdessen eine Erklärung aus, anstatt zu wechseln.

Wenn die Anzeige flackert oder die Scrollposition springt, während Claude arbeitet, wechseln Sie zum [Vollbildrenderingmodus](/docs/de/fullscreen). In diesem Modus scrollen Sie mit der Maus oder PageUp in Claude Code, anstatt mit dem nativen Scrollback Ihres Terminals. Weitere Informationen zum Suchen und Kopieren finden Sie auf der [Vollbildseite](/docs/de/fullscreen#search-and-review-the-conversation).

Wenn Flackern das einzige Problem ist und Ihr Terminal synchronisierte Ausgabe unterstützt, aber nicht automatisch erkannt wird, z. B. Emacs `eat`, setzen Sie [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/de/env-vars), um das Flackern zu stoppen, ohne den Renderer zu wechseln.

Führen Sie `/tui fullscreen` aus, um zu wechseln und die Einstellung zu speichern. Ihre Konversation wird intakt neu gestartet und zukünftige Sitzungen starten im Vollbildmodus, es sei denn, ein [Vollbildstart schlägt fehl](/docs/de/fullscreen#fullscreen-renderer-didnt-finish-starting). Sie können auch die Umgebungsvariable `CLAUDE_CODE_NO_FLICKER` setzen, bevor Sie Claude Code starten:

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  Großen Inhalt einfügen
</h2>

Wenn Sie mehr als 800 Zeichen oder mehr als drei Zeilen in die Eingabeaufforderung einfügen, reduziert Claude Code die Eingabe auf einen Platzhalter wie `[Eingefügter Text #1 +120 Zeilen]`, damit das Eingabefeld nutzbar bleibt, und sendet dennoch den vollständigen Inhalt, wenn Sie die Eingabe absenden. Für sehr große Eingaben wie ganze Dateien oder lange Protokolle schreiben Sie den Inhalt in eine Datei und bitten Sie Claude, diese zu lesen, anstatt sie einzufügen. Das Gesprächstranskript bleibt lesbar und Claude kann die Datei in späteren Durchläufen nach Pfad referenzieren. Das in VS Code integrierte Terminal kann auch Zeichen aus sehr großen Einfügungen verlieren, bevor sie Claude Code erreichen. Verwenden Sie daher dort eine Datei.

Wenn die eingefügte Datei [unsichtbare Unicode-Zeichen](/docs/de/interactive-mode#invisible-characters-in-prompts) enthält, entfernt Claude Code diese, wenn Sie die Eingabetaste drücken, und setzt die bereinigte Eingabeaufforderung in das Eingabefeld zurück, damit Sie sie mit einer weiteren Eingabetaste absenden können.

<h3 id="how-claude-treats-pasted-text">
  Wie Claude eingefügten Text behandelt
</h3>

Wenn Sie absenden, sieht Claude den Inhalt hinter jedem `[Eingefügter Text #N]`-Platzhalter als Text markiert, den Sie von irgendwo anders eingefügt haben, anstatt ihn zu tippen. Claude wird mitgeteilt, dass eine Einfügung Anweisungen enthalten kann, die Sie nicht geschrieben haben, und dass er Anweisungen darin nur dort befolgen soll, wo die von Ihnen eingegebene Nachricht dies verlangt. In Sitzungen, die [Feature-Flags nicht abrufen](/docs/de/env-vars#features-that-need-feature-flag-fetching), werden Einfügungen nicht markiert.

<h3 id="delete-and-restore-a-collapsed-paste">
  Eine reduzierte Einfügung löschen und wiederherstellen
</h3>

Wenn Sie mit einer Wort- oder Zeilenkürzel wie `Ctrl+W` oder `Ctrl+K` löschen oder mit einem Vim-Löschbefehl durch eine `f`/`t`-Bewegung wie `df]` löschen und der gelöschte Bereich in einen `[Eingefügter Text #N]`-Platzhalter hineinreicht, entfernt Claude Code den Platzhalter vollständig. Um ihn wiederherzustellen, fügen Sie das Gelöschte mit [`Ctrl+Y`](/docs/de/interactive-mode#text-editing) nach einem Wort- oder Zeilenkürzel ein, oder mit [`p` im NORMAL-Modus](/docs/de/interactive-mode#editing-normal-mode) nach einem Vim-Löschbefehl.

<h3 id="recall-a-prompt-that-had-pasted-text">
  Eine Eingabeaufforderung mit eingefügtem Text abrufen
</h3>

Claude Code speichert den Inhalt hinter jedem `[Eingefügter Text #N]`-Platzhalter unter `~/.claude/paste-cache/`, sodass Sie beim Abrufen einer Eingabeaufforderung aus dem [Befehlsverlauf](/docs/de/interactive-mode#command-history) und erneuter Absendung der vollständige eingefügte Inhalt erneut gesendet wird, auch in einer späteren Sitzung.

Cache-Dateien, die älter als [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays) sind, werden gemäß den [Aufräumregeln](/docs/de/claude-directory#cleaned-up-automatically) gelöscht, sodass eine abgerufene Eingabeaufforderung auf eingefügten Text verweisen kann, der nicht mehr vorhanden ist. Wenn Sie eine solche Eingabeaufforderung absenden, sendet Claude Code niemals die wörtliche Zeichenkette `[Eingefügter Text #N]` und zeigt eine Benachrichtigung an, die den fehlenden Einfügungstext benennt:

* In einer einfachen Eingabeaufforderung mit verbleibendem Text entfernt Claude Code den Platzhalter und sendet den verbleibenden Text.
* In einem [Shell-Modus](/docs/de/interactive-mode#shell-mode-with-prefix)-Befehl oder einem `/`-Befehl, bei dem die Entfernung das Ausgeführte ändern würde, und in jeder Eingabeaufforderung, bei der die Entfernung zu einer leeren Eingabe führt, bricht Claude Code die Absendung ab und behält den ursprünglichen Text in der Eingabe, wobei der Platzhalter noch darin enthalten ist. Löschen Sie den Platzhalter oder bearbeiten Sie den Befehl, und senden Sie dann erneut ab.

<h2 id="edit-prompts-with-vim-keybindings">
  Prompts mit Vim-Tastenbindungen bearbeiten
</h2>

Claude Code enthält einen Vim-ähnlichen Bearbeitungsmodus für die Eingabeaufforderung. Aktivieren Sie ihn über `/config` → Editor mode, oder indem Sie [`editorMode`](/docs/de/settings-reference#editormode) in `~/.claude/settings.json` auf `"vim"` setzen. Setzen Sie Editor mode zurück auf `normal`, um ihn auszuschalten.

Der Vim-Modus unterstützt eine Teilmenge von NORMAL- und VISUAL-Modus-Bewegungen und Operatoren, wie z. B. `hjkl`-Navigation, `v`/`V`-Auswahl und `d`/`c`/`y` mit Textobjekten. Siehe die [Vim-Editor-Modus-Referenz](/docs/de/interactive-mode#vim-editor-mode) für die vollständige Tabelle der Tasten.

Vim-Bewegungen können nicht über die Tastenbindungsdatei neu zugeordnet werden. Um eine zweitastige INSERT-Modus-Sequenz wie `jj` auf Escape abzubilden, setzen Sie [`vimInsertModeRemaps`](/docs/de/interactive-mode#remap-insert-mode-key-sequences) in Ihren Benutzereinstellungen.

Das Drücken der Eingabetaste sendet Ihre Eingabeaufforderung im INSERT-Modus weiterhin ab, anders als bei Standard-Vim. Verwenden Sie `o` oder `O` im NORMAL-Modus oder Strg+J, um stattdessen eine neue Zeile einzufügen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Interaktiver Modus](/docs/de/interactive-mode): vollständige Tastaturverknüpfungs-Referenz und die Vim-Schlüsseltabelle
* [Tastenbelegungen](/docs/de/keybindings): ordnen Sie jede Claude Code-Verknüpfung neu zu, einschließlich Enter und Shift+Enter
* [Vollbildrendering](/docs/de/fullscreen): Details zum Scrollen, Suchen und Kopieren im Vollbildmodus
* [Hooks-Leitfaden](/docs/de/hooks-guide): weitere Benachrichtigungshook-Beispiele für Linux und Windows
* [Fehlerbehebung](/docs/de/troubleshooting): Behebungen für Probleme außerhalb der Terminalkonfiguration
