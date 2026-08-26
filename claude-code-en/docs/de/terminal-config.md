> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurieren Sie Ihr Terminal für Claude Code

> Beheben Sie Shift+Enter für Zeilenumbrüche, erhalten Sie einen Terminalton, wenn Claude fertig ist, konfigurieren Sie tmux, passen Sie das Farbschema an, und aktivieren Sie den Vim-Modus in der Claude Code CLI.

Claude Code funktioniert in jedem Terminal ohne Konfiguration. Diese Seite ist für den Fall, dass etwas nicht so funktioniert, wie Sie es erwarten. Finden Sie Ihr Symptom unten. Wenn alles bereits richtig funktioniert, benötigen Sie diese Seite nicht.

* [Shift+Enter sendet statt einen Zeilenumbruch einzufügen](#enter-multiline-prompts)
* [Option-Taste-Verknüpfungen funktionieren nicht auf macOS](#enable-option-key-shortcuts-on-macos)
* [Kein Ton oder Benachrichtigung, wenn Claude fertig ist](#get-a-terminal-bell-or-notification)
* [Sie führen Claude Code in tmux aus](#configure-tmux)
* [Die Anzeige flimmert oder die Scrollposition springt](#switch-to-fullscreen-rendering)
* [Sie möchten Vim-Tasten in der Eingabeaufforderung](#edit-prompts-with-vim-keybindings)

Diese Seite behandelt das Konfigurieren Ihres Terminals, um die richtigen Signale an Claude Code zu senden. Um zu ändern, auf welche Tasten Claude Code selbst reagiert, siehe stattdessen [Tastenbelegungen](/docs/de/keybindings).

<h2 id="enter-multiline-prompts">
  Mehrzeilige Eingabeaufforderungen eingeben
</h2>

Durch Drücken von Enter wird Ihre Nachricht gesendet. Um einen Zeilenumbruch ohne Absenden hinzuzufügen, drücken Sie Ctrl+J, oder geben Sie `\` ein und drücken dann Enter. Beide funktionieren in jedem Terminal ohne Setup.

In den meisten Terminals können Sie auch Shift+Enter drücken, aber die Unterstützung variiert je nach Terminal-Emulator:

| Terminal                                                                | Shift+Enter für Zeilenumbruch                             |
| :---------------------------------------------------------------------- | :-------------------------------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal | Funktioniert ohne Setup                                   |
| VS Code, Cursor, Devin Desktop, Alacritty, Zed                          | Führen Sie `/terminal-setup` einmal aus                   |
| gnome-terminal, JetBrains IDEs wie PyCharm und Android Studio           | Nicht verfügbar; verwenden Sie Ctrl+J oder `\` dann Enter |

Für VS Code, Cursor, Devin Desktop, Alacritty und Zed schreibt `/terminal-setup` Shift+Enter und andere Tastenbelegungen in die Konfigurationsdatei des Terminals. Vorhandene Bindungen bleiben erhalten; wenn Sie eine Meldung wie `VSCode terminal Shift+Enter key binding already configured` sehen, wurde keine Änderung vorgenommen. Führen Sie `/terminal-setup` direkt im Host-Terminal aus, nicht in tmux oder screen, da es in die Konfiguration des Host-Terminals schreiben muss.

In VS Code, Cursor und Devin Desktop aktualisiert `/terminal-setup` auch zwei Editor-Einstellungen: Es setzt `terminal.integrated.gpuAcceleration` auf `"off"`, um verzerrten Text im integrierten Terminal zu verhindern, und es setzt `terminal.integrated.mouseWheelScrollSensitivity` für sanfteres Scrollen im [Vollbildmodus](/docs/de/fullscreen). Um die GPU-Beschleunigungsänderung rückgängig zu machen, setzen Sie sie auf `"auto"` zurück und laden Sie das Editor-Fenster neu.

Wenn Sie in tmux ausgeführt werden, erfordert Shift+Enter auch die [tmux-Konfiguration unten](#configure-tmux), selbst wenn das äußere Terminal es unterstützt.

Um Zeilenumbruch an eine andere Taste zu binden oder das Verhalten zu tauschen, sodass Enter einen Zeilenumbruch einfügt und Shift+Enter sendet, ordnen Sie die Aktionen `chat:newline` und `chat:submit` in Ihrer [Tastenbelegungsdatei](/docs/de/keybindings) zu.

<h2 id="enable-option-key-shortcuts-on-macos">
  Aktivieren Sie Option-Taste-Verknüpfungen auf macOS
</h2>

Einige Claude Code-Verknüpfungen verwenden die Option-Taste, z. B. Option+Enter für einen Zeilenumbruch oder Option+P zum Wechsel von Modellen. Auf macOS senden die meisten Terminals die Option-Taste standardmäßig nicht als Modifizierer, daher funktionieren diese Verknüpfungen nicht, bis Sie sie aktivieren. Die Terminal-Einstellung dafür wird normalerweise als „Option als Meta-Taste verwenden" bezeichnet; Meta ist der historische Unix-Name für die Taste, die jetzt Option oder Alt genannt wird.

<Tabs>
  <Tab title="Apple Terminal">
    Öffnen Sie Einstellungen → Profile → Tastatur und aktivieren Sie 'Option als Meta-Taste verwenden".

    Wenn Sie die erste Eingabeaufforderung von Claude Code akzeptiert haben, die „Option+Enter für Zeilenumbrüche und visuellen Ton" angeboten hat, ist dies bereits erledigt. Diese Eingabeaufforderung führt `/terminal-setup` für Sie aus, das Option als Meta aktiviert und den Audioton auf einen visuellen Bildschirmblitz in Ihrem Apple Terminal-Profil umschaltet.
  </Tab>

  <Tab title="iTerm2">
    Öffnen Sie Einstellungen → Profile → Tasten → Allgemein und stellen Sie die linke Option-Taste und die rechte Option-Taste auf 'Esc+" ein.

    Das Ausführen von `/terminal-setup` in iTerm2 aktiviert „Anwendungen im Terminal können auf die Zwischenablage zugreifen" unter Einstellungen → Allgemein → Auswahl, damit der Befehl `/copy` in Ihre Systemzwischenablage schreiben kann. Der Befehl erkennt iTerm2 auch, wenn er von innerhalb von tmux ausgeführt wird. Starten Sie iTerm2 neu, damit die Änderung wirksam wird.
  </Tab>

  <Tab title="VS Code">
    Fügen Sie `"terminal.integrated.macOptionIsMeta": true` zu Ihren VS Code-Einstellungen hinzu.
  </Tab>
</Tabs>

Für Ghostty, Kitty und andere Terminals suchen Sie nach einer Option-als-Alt- oder Option-als-Meta-Einstellung in der Konfigurationsdatei des Terminals.

<h2 id="get-a-terminal-bell-or-notification">
  Erhalten Sie einen Terminalton oder eine Benachrichtigung
</h2>

Wenn Claude eine Aufgabe abschließt oder bei einer Berechtigungsaufforderung pausiert, wird ein Benachrichtigungsereignis ausgelöst. Wenn Sie dies als Terminalton oder Desktop-Benachrichtigung anzeigen, können Sie zu anderen Arbeiten wechseln, während eine lange Aufgabe ausgeführt wird.

Standardmäßig sendet Claude Code eine Desktop-Benachrichtigung nur in Ghostty, Kitty und iTerm2. In anderen Terminals setzen Sie [`preferredNotifChannel`](/docs/de/settings#available-settings) auf `"terminal_bell"`, um stattdessen den Terminalton zu aktivieren, oder konfigurieren Sie einen [Benachrichtigungshook](#play-a-sound-with-a-notification-hook) für einen benutzerdefinierten Ton oder Befehl.

Die Desktop-Benachrichtigung erreicht Ihren lokalen Computer über SSH, sodass eine Remote-Sitzung Sie immer noch benachrichtigen kann. Ghostty und Kitty leiten sie ohne weitere Einrichtung an Ihr Betriebssystem-Benachrichtigungscenter weiter. iTerm2 erfordert, dass Sie die Weiterleitung aktivieren:

<Steps>
  <Step title="Öffnen Sie iTerm2-Benachrichtigungseinstellungen">
    Gehen Sie zu Einstellungen → Profile → Terminal.
  </Step>

  <Step title="Aktivieren Sie Benachrichtigungen">
    Aktivieren Sie „Notification Center Alerts", klicken Sie dann auf „Filter Alerts" und aktivieren Sie „Send escape sequence-generated alerts".
  </Step>
</Steps>

Wenn Benachrichtigungen immer noch nicht angezeigt werden, bestätigen Sie, dass Ihre Terminalanwendung in Ihren Betriebssystemeinstellungen Benachrichtigungsberechtigung hat, und wenn Sie in tmux ausgeführt werden, [aktivieren Sie Passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Spielen Sie einen Ton mit einem Benachrichtigungshook ab
</h3>

In jedem Terminal können Sie einen [Benachrichtigungshook](/docs/de/hooks-guide#get-notified-when-claude-needs-input) konfigurieren, um einen Ton abzuspielen oder einen benutzerdefinierten Befehl auszuführen, wenn Claude Ihre Aufmerksamkeit benötigt. Hooks werden neben der Desktop-Benachrichtigung ausgeführt, nicht als Ersatz, sodass Terminals, die keine Desktop-Benachrichtigung erhalten, wie Warp oder das in VS Code integrierte Terminal, einen Hook verwenden oder `preferredNotifChannel` stattdessen auf `"terminal_bell"` setzen können.

Das folgende Beispiel spielt einen Systemton auf macOS ab. Der verlinkte Leitfaden enthält Desktop-Benachrichtigungsbefehle für macOS, Linux und Windows.

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
  Konfigurieren Sie tmux
</h2>

Wenn Claude Code in tmux ausgeführt wird, brechen zwei Dinge standardmäßig: Shift+Enter sendet statt einen Zeilenumbruch einzufügen, und Desktop-Benachrichtigungen und die [Fortschrittsleiste](/docs/de/settings#available-settings) erreichen niemals das äußere Terminal. Fügen Sie diese Zeilen zu `~/.tmux.conf` hinzu und führen Sie dann `tmux source-file ~/.tmux.conf` aus, um sie auf den laufenden Server anzuwenden:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

Die `allow-passthrough`-Zeile ermöglicht es, dass Benachrichtigungen und Fortschrittsaktualisierungen das äußere Terminal erreichen, anstatt von tmux verschluckt zu werden. Die `extended-keys`-Zeilen ermöglichen es tmux, Shift+Enter von einfachem Enter zu unterscheiden, sodass die Zeilenumbruch-Verknüpfung funktioniert.

<h2 id="match-the-color-theme">
  Passen Sie das Farbschema an
</h2>

Verwenden Sie den Befehl `/theme` oder die Designauswahl in `/config`, um ein Claude Code-Design auszuwählen, das Ihrem Terminal entspricht. Wenn Sie die Auto-Option auswählen, wird der helle oder dunkle Hintergrund Ihres Terminals erkannt, sodass das Design den Änderungen des Betriebssystem-Erscheinungsbilds folgt, wenn Ihr Terminal dies tut. Claude Code steuert nicht das Farbschema des Terminals selbst, das von der Terminalanwendung festgelegt wird.

Um anzupassen, was am unteren Rand der Benutzeroberfläche angezeigt wird, konfigurieren Sie eine [benutzerdefinierte Statuszeile](/docs/de/statusline), die das aktuelle Modell, das Arbeitsverzeichnis, den Git-Branch oder andere Kontextinformationen anzeigt.

<h3 id="create-a-custom-theme">
  Erstellen Sie ein benutzerdefiniertes Design
</h3>

<Note>
  Benutzerdefinierte Designs erfordern Claude Code v2.1.118 oder später.
</Note>

Zusätzlich zu den integrierten Voreinstellungen listet `/theme` alle benutzerdefinierten Designs auf, die Sie definiert haben, sowie alle Designs, die von installierten [Plugins](/docs/de/plugins-reference#themes) beigetragen wurden. Wählen Sie **Neues benutzerdefiniertes Design…** am Ende der Liste aus, um eines interaktiv zu erstellen: Sie benennen das Design und wählen dann einzelne Farbtoken aus, um sie zu überschreiben. Drücken Sie `Ctrl+E`, während ein benutzerdefiniertes Design hervorgehoben ist, um es zu bearbeiten.

Jedes benutzerdefinierte Design ist eine JSON-Datei in `~/.claude/themes/`. Der Dateiname ohne die `.json`-Erweiterung ist der Slug des Designs, und das Auswählen des Designs speichert `custom:<slug>` als Ihre Design-Einstellung. Die Datei hat drei optionale Felder:

| Feld        | Typ    | Beschreibung                                                                                                                                                        |
| :---------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`      | string | Anzeigebezeichnung in `/theme`. Standardmäßig der Dateiname-Slug                                                                                                    |
| `base`      | string | Integrierte Voreinstellung, von der das Design ausgeht: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi` oder `light-ansi`. Standardmäßig `dark` |
| `overrides` | object | Zuordnung von Farbtoken-Namen zu Farbwerten. Token, die hier nicht aufgelistet sind, fallen auf die Basisvoreinstellung zurück                                      |

Farbwerte akzeptieren `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)` oder `ansi:<name>`, wobei `<name>` einer der 16 standardmäßigen ANSI-Farbnamen wie `red` oder `cyanBright` ist. Unbekannte Token und ungültige Farbwerte werden ignoriert, sodass ein Tippfehler das Rendering nicht beschädigen kann.

Das folgende Beispiel definiert ein Design, das die dunkle Voreinstellung beibehält, aber die Eingabeaufforderungs-Akzentfarbe, Fehlertext und Erfolgstexte umfärbt:

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

Claude Code überwacht `~/.claude/themes/` und lädt neu, wenn sich eine Datei ändert, sodass Änderungen, die in Ihrem Editor vorgenommen werden, auf eine laufende Sitzung angewendet werden, ohne dass ein Neustart erforderlich ist.

Die Referenz unten behandelt die Token, die Sie in `overrides` festlegen können. Der interaktive Editor in `/theme` zeigt die gleichen Token mit einer Live-Vorschau an, plus einige wenige Einzelzweck-Akzente wie Onboarding-Bildschirmfarben, die hier weggelassen sind.

<Accordion title="Farbtoken-Referenz">
  Das folgende Beispiel kombiniert Token aus mehreren der folgenden Gruppen: der Brand-Akzent, der Plan Mode-Rahmen, die Diff-Hintergründe und der Vollbildmeldungs-Hintergrund.

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

  | Token         | Steuert                                                                          |
  | :------------ | :------------------------------------------------------------------------------- |
  | `claude`      | Primärer Brand-Akzent, verwendet für den Spinner und die Assistenten-Bezeichnung |
  | `text`        | Standard-Vordergrundtext                                                         |
  | `inverseText` | Text, der auf einem farbigen Hintergrund gezeichnet wird, z. B. Statusabzeichen  |
  | `inactive`    | Sekundärer Text wie Hinweise, Zeitstempel und deaktivierte Elemente              |
  | `subtle`      | Schwache Rahmen und de-betonte sekundäre Texte                                   |
  | `suggestion`  | Autovervollständigungs-Vorschläge und Auswahlhervorhebung in Auswahlfeldern      |
  | `permission`  | Dialograhmen, einschließlich Berechtigungsaufforderungen und Auswahlfelder       |
  | `remember`    | Speicher- und `CLAUDE.md`-Indikatoren                                            |

  <h4 id="status-colors">
    Statusfarben
  </h4>

  Signalisieren Sie Erfolgs-, Fehler- und Warnzustände über Meldungen und Indikatoren.

  | Token     | Steuert                                                |
  | :-------- | :----------------------------------------------------- |
  | `success` | Erfolgsmeldungen und bestandene Überprüfungen          |
  | `error`   | Fehlermeldungen und Fehler                             |
  | `warning` | Warnungen, Vorsichtsmeldungen und der Auto Mode-Rahmen |
  | `merged`  | Zusammengeführter Pull Request-Status                  |

  <h4 id="input-box-and-mode-indicators">
    Eingabefeld und Modusindikatoren
  </h4>

  Legen Sie die Eingabefeld-Rahmenfarbe und den Akzent fest, der angezeigt wird, während ein Berechtigungsmodus oder Indikator aktiv ist.

  | Token          | Steuert                                                  |
  | :------------- | :------------------------------------------------------- |
  | `promptBorder` | Eingabefeld-Rahmen im Standard-Berechtigungsmodus        |
  | `planMode`     | Plan Mode-Akzent und Rahmen                              |
  | `autoAccept`   | Accept-edits Mode-Akzent und Rahmen                      |
  | `bashBorder`   | Eingabefeld-Rahmen beim Eingeben eines `!` Shell-Befehls |
  | `ide`          | IDE-Verbindungsindikator                                 |
  | `fastMode`     | Fast Mode-Indikator                                      |

  <h4 id="diff-rendering">
    Diff-Rendering
  </h4>

  Färben Sie hinzugefügte und entfernte Code in Dateibearbeitungen und Überprüfungen.

  | Token               | Steuert                                                                 |
  | :------------------ | :---------------------------------------------------------------------- |
  | `diffAdded`         | Hintergrund hinzugefügter Zeilen                                        |
  | `diffRemoved`       | Hintergrund entfernter Zeilen                                           |
  | `diffAddedDimmed`   | Hintergrund des unveränderten Kontexts in der Nähe hinzugefügter Zeilen |
  | `diffRemovedDimmed` | Hintergrund des unveränderten Kontexts in der Nähe entfernter Zeilen    |
  | `diffAddedWord`     | Hervorhebung auf Wortebene innerhalb einer hinzugefügten Zeile          |
  | `diffRemovedWord`   | Hervorhebung auf Wortebene innerhalb einer entfernten Zeile             |

  <h4 id="fullscreen-mode">
    Vollbildmodus
  </h4>

  Gilt nur im [Vollbild-Rendering-Modus](/docs/de/fullscreen), in dem Meldungen einen Hintergrund-Füllung haben.

  | Token                        | Steuert                                                                       |
  | :--------------------------- | :---------------------------------------------------------------------------- |
  | `userMessageBackground`      | Hintergrund hinter Ihren Meldungen im Transkript                              |
  | `userMessageBackgroundHover` | Hintergrund hinter einer Meldung beim Hovern oder Erweitern                   |
  | `messageActionsBackground`   | Hintergrund hinter der ausgewählten Meldung, wenn die Aktionsleiste offen ist |
  | `bashMessageBackgroundColor` | Hintergrund hinter `!` Shell-Befehls-Einträgen im Transkript                  |
  | `memoryBackgroundColor`      | Hintergrund hinter `#` Speicher-Einträgen im Transkript                       |
  | `selectionBg`                | Hintergrund von Text, der mit der Maus ausgewählt wurde                       |

  <h4 id="usage-meter-and-speaker-labels">
    Nutzungsmesser und Sprecherbeschriftungen
  </h4>

  Passen Sie die Leiste an, die in der `/usage`-Ansicht angezeigt wird, und die Beschriftungen, die Ihre Meldungen von Claudes unterscheiden.

  | Token              | Steuert                                                   |
  | :----------------- | :-------------------------------------------------------- |
  | `rate_limit_fill`  | Gefüllter Teil des Nutzungsmessers                        |
  | `rate_limit_empty` | Ungefüllter Teil des Nutzungsmessers                      |
  | `briefLabelYou`    | Farbe der `You`-Beschriftung auf Ihren Meldungen          |
  | `briefLabelClaude` | Farbe der `Claude`-Beschriftung auf Assistenten-Meldungen |

  <h4 id="shimmer-variants-and-subagent-colors">
    Shimmer-Varianten und Subagent-Farben
  </h4>

  Mehrere Token haben eine gepaarte Shimmer-Variante, die die hellere Farbe liefert, die im animierten Farbverlauf des Spinners verwendet wird. Überschreiben Sie den Shimmer zusammen mit seinem Basis-Token, wenn die Animation nicht übereinstimmend aussieht.

  * `claude` und `claudeShimmer`
  * `warning` und `warningShimmer`
  * `permission` und `permissionShimmer`
  * `promptBorder` und `promptBorderShimmer`
  * `inactive` und `inactiveShimmer`
  * `fastMode` und `fastModeShimmer`

  Jeder [Subagent](/docs/de/sub-agents) und jede parallele Aufgabe wird in einer von acht benannten Farben angezeigt, damit Sie sie im Transkript unterscheiden können. Die Token-Namen folgen dem Muster `<color>_FOR_SUBAGENTS_ONLY`, wobei `<color>` `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` oder `cyan` ist. Überschreiben Sie diese, um zu ändern, wie jede benannte Farbe aussieht. Beispielsweise wird ein Subagent mit `color: blue` in seiner Definition mit dem Wert `blue_FOR_SUBAGENTS_ONLY` gezeichnet.

  Das Schlüsselwort [`ultrathink`](/docs/de/model-config#use-ultrathink-for-one-off-deep-reasoning) und [`ultraplan`](/docs/de/ultraplan) in der Eingabeaufforderung werden mit einem siebenfarbigen Regenbogenfarbverlauf gerendert. Die Token-Namen folgen dem Muster `rainbow_<color>` und `rainbow_<color>_shimmer`, wobei `<color>` `red`, `orange`, `yellow`, `green`, `blue`, `indigo` oder `violet` ist.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Wechseln Sie zum Vollbildrendering
</h2>

Wenn die Anzeige flimmert oder die Scrollposition springt, während Claude arbeitet, wechseln Sie zum [Vollbildrendering-Modus](/docs/de/fullscreen). Es zeichnet auf einen separaten Bildschirm, den das Terminal für Vollbild-Apps reserviert, anstatt an Ihren normalen Scrollback anzuhängen, was die Speichernutzung flach hält und Mausunterstützung zum Scrollen und Auswählen hinzufügt. In diesem Modus scrollen Sie mit der Maus oder PageUp in Claude Code, nicht mit dem nativen Scrollback Ihres Terminals; siehe die [Vollbildseite](/docs/de/fullscreen#search-and-review-the-conversation) für die Suche und das Kopieren.

Wenn Flimmern das einzige Problem ist und Ihr Terminal synchronisierte Ausgabe unterstützt, aber nicht automatisch erkannt wird, wie Emacs `eat`, setzen Sie [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/de/env-vars), um das Flimmern zu stoppen, ohne den Renderer zu wechseln.

Führen Sie `/tui fullscreen` aus, um zu wechseln und die Einstellung zu speichern. Ihr Gespräch wird intakt neu gestartet und zukünftige Sitzungen starten im Vollbildmodus. Sie können auch die Umgebungsvariable `CLAUDE_CODE_NO_FLICKER` setzen, bevor Sie Claude Code starten:

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

Wenn Sie mehr als 10.000 Zeichen in die Eingabeaufforderung einfügen, reduziert Claude Code die Eingabe auf einen `[Pasted text]`-Platzhalter, damit das Eingabefeld verwendbar bleibt. Der vollständige Inhalt wird immer noch an Claude gesendet, wenn Sie absenden.

Das integrierte VS Code-Terminal kann Zeichen aus sehr großen Einfügungen verlieren, bevor sie Claude Code erreichen, daher bevorzugen Sie dort dateibasierte Workflows. Für sehr große Eingaben wie ganze Dateien oder lange Protokolle schreiben Sie den Inhalt in eine Datei und bitten Claude, diese zu lesen, anstatt einzufügen. Dies hält das Gesprächstranskript lesbar und ermöglicht es Claude, die Datei in späteren Zügen nach Pfad zu referenzieren.

<h2 id="edit-prompts-with-vim-keybindings">
  Bearbeiten Sie Eingabeaufforderungen mit Vim-Tastenbelegungen
</h2>

Claude Code enthält einen Vim-ähnlichen Bearbeitungsmodus für die Eingabeaufforderungseingabe. Aktivieren Sie ihn über `/config` → Editor-Modus, oder indem Sie [`editorMode`](/docs/de/settings#available-settings) auf `"vim"` in `~/.claude/settings.json` setzen. Setzen Sie den Editor-Modus zurück auf `normal`, um ihn auszuschalten.

Der Vim-Modus unterstützt eine Teilmenge von NORMAL- und VISUAL-Modus-Bewegungen und Operatoren, wie z. B. `hjkl`-Navigation, `v`/`V`-Auswahl und `d`/`c`/`y` mit Textobjekten. Siehe die [Vim-Editor-Modus-Referenz](/docs/de/interactive-mode#vim-editor-mode) für die vollständige Schlüsseltabelle.

Vim-Bewegungen können nicht über die Tastenbelegungsdatei neu zugeordnet werden. Um eine zwei-Tasten-INSERT-Modus-Sequenz wie `jj` auf Escape abzubilden, setzen Sie [`vimInsertModeRemaps`](/docs/de/interactive-mode#remap-insert-mode-key-sequences) in Ihren Benutzereinstellungen.

Das Drücken von Enter sendet Ihre Eingabeaufforderung immer noch im INSERT-Modus, anders als Standard-Vim. Verwenden Sie `o` oder `O` im NORMAL-Modus oder Ctrl+J, um stattdessen einen Zeilenumbruch einzufügen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Interaktiver Modus](/docs/de/interactive-mode): vollständige Tastaturverknüpfungs-Referenz und die Vim-Schlüsseltabelle
* [Tastenbelegungen](/docs/de/keybindings): ordnen Sie jede Claude Code-Verknüpfung neu zu, einschließlich Enter und Shift+Enter
* [Vollbildrendering](/docs/de/fullscreen): Details zum Scrollen, Suchen und Kopieren im Vollbildmodus
* [Hooks-Leitfaden](/docs/de/hooks-guide): weitere Benachrichtigungshook-Beispiele für Linux und Windows
* [Fehlerbehebung](/docs/de/troubleshooting): Behebungen für Probleme außerhalb der Terminalkonfiguration
