> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Fehlerbehebung

> Beheben Sie hohe CPU- oder Speichernutzung, Hänger, Auto-Compact-Thrashing und Suchprobleme in Claude Code und finden Sie die richtige Seite für andere Probleme.

Diese Seite behandelt Leistungs-, Stabilitäts- und Suchprobleme, sobald Claude Code läuft. Für andere Probleme beginnen Sie mit der Seite, die zu Ihrer Situation passt:

| Symptom                                                                                                                                                        | Gehen Sie zu                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| `command not found`, Installation schlägt fehl, PATH-Probleme, `EACCES`, TLS-Fehler                                                                            | [Fehlerbehebung bei Installation und Anmeldung](/docs/de/troubleshoot-install)                          |
| Update oder Installation schlägt fehl mit `The connection dropped while downloading the update` oder `aborted`                                                 | [Fehlerreferenz](/docs/de/errors#the-connection-dropped-while-downloading-the-update)                   |
| Anmeldeschleifen, OAuth-Fehler, `403 Forbidden`, „Organisation deaktiviert", Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry-Anmeldedaten | [Fehlerbehebung bei Installation und Anmeldung](/docs/de/troubleshoot-install#login-and-authentication) |
| Einstellungen werden nicht angewendet, Hooks werden nicht ausgelöst, MCP-Server werden nicht geladen                                                           | [Debuggen Sie Ihre Konfiguration](/docs/de/debug-your-config)                                           |
| Sitzung wurde im Auto-Modus gestartet, oder Claude bearbeitet Dateien und führt Befehle aus, ohne zu fragen                                                    | [Welcher Modus eine Sitzung startet](/docs/de/permission-modes#which-mode-a-session-starts-in)          |
| `API Error: 5xx`, `529 Overloaded`, `429`, Request-Validierungsfehler                                                                                          | [Fehlerreferenz](/docs/de/errors)                                                                       |
| `model not found` oder `you may not have access to it`                                                                                                         | [Fehlerreferenz](/docs/de/errors#theres-an-issue-with-the-selected-model)                               |
| VS Code-Erweiterung verbindet sich nicht oder erkennt Claude nicht                                                                                             | [VS Code-Integration](/docs/de/vs-code#fix-common-issues)                                               |
| `Claude Code process exited with code 1` in VS Code oder einer SDK-App                                                                                         | [Fehlerreferenz](/docs/de/errors#claude-code-process-exited-with-code-n)                                |
| JetBrains-Plugin oder IDE wird nicht erkannt                                                                                                                   | [JetBrains-Integration](/docs/de/jetbrains#troubleshooting)                                             |
| Hohe CPU oder Speicher, langsame Antworten, Hänger, Suche findet Dateien nicht                                                                                 | [Leistung und Stabilität](#performance-and-stability) unten                                        |

Wenn Sie sich nicht sicher sind, welcher Fall zutrifft, führen Sie `/doctor` in Claude Code aus, um eine automatisierte Überprüfung Ihrer Installation, Einstellungen, Erweiterungen und Kontextnutzung durchzuführen; es schlägt Korrektionen vor, die es nach Ihrer Bestätigung anwenden kann. Wenn `claude` überhaupt nicht startet, führen Sie stattdessen `claude doctor` aus Ihrer Shell aus. Führen Sie `/mcp` aus, um den MCP-Server-Status zu überprüfen.

<h2 id="performance-and-stability">
  Leistung und Stabilität
</h2>

Diese Abschnitte behandeln Probleme im Zusammenhang mit Ressourcennutzung, Reaktionsfähigkeit und Suchverhalten.

<h3 id="high-cpu-or-memory-usage">
  Hohe CPU- oder Speicherauslastung
</h3>

Claude Code ist für die Zusammenarbeit mit den meisten Entwicklungsumgebungen konzipiert, kann aber bei der Verarbeitung großer Codebases erhebliche Ressourcen verbrauchen. Wenn Sie Leistungsprobleme haben:

1. Verwenden Sie `/compact` regelmäßig, um die Kontextgröße zu reduzieren. Wenn es `Not enough messages to compact.` zurückgibt, hat die Konversation zu wenige Turns zum Zusammenfassen; das kann auch bei vollem Kontext vorkommen, wenn ein einzelnes großes Einfügen ihn gefüllt hat
2. Schließen Sie Claude Code zwischen großen Aufgaben und starten Sie es neu
3. Erwägen Sie, große Build-Verzeichnisse zu Ihrer `.gitignore`-Datei hinzuzufügen
4. Starten Sie mit [`claude --safe-mode`](/docs/de/cli-reference#cli-flags) neu, um zu überprüfen, ob ein Plugin, MCP-Server oder Hook die Ursache ist. Dies deaktiviert alle Anpassungen für die Sitzung; wenn die Auslastung sinkt, siehe [Debug your configuration](/docs/de/debug-your-config#test-against-a-clean-configuration), um herauszufinden, welche es ist

Wenn der Heap-Speicher einer Sitzung 2,5 GB überschreitet, wird eine kritische Speicherauslastungswarnung angezeigt. Um den Speicher freizugeben, starten Sie Claude Code neu und führen Sie [`claude --continue`](/docs/de/cli-reference#cli-flags) aus, um die Konversation in einem neuen Prozess fortzusetzen.

Außerhalb des [Vollbildmodus](/docs/de/fullscreen) gibt das Ausführen von `/compact` auch Speicher frei. Die Warnung verschwindet, sobald die Speicherauslastung wieder unter 2,5 GB fällt.

Wenn die Speicherauslastung nach diesen Schritten hoch bleibt, führen Sie `/heapdump` aus, um zwei Dateien auf `~/Desktop` zu schreiben: einen JavaScript-Heap-Snapshot mit dem Namen `<session-id>.heapsnapshot` und eine Speicheraufschlüsselung mit dem Namen `<session-id>-diagnostics.json`. Claude Code [blendet den Befehl aus dem Befehlsmenü aus](/docs/de/commands#how-the-command-menu-matches-what-you-type); geben Sie ihn vollständig ein. Unter Linux ohne Desktop-Ordner werden die Dateien in Ihr Home-Verzeichnis geschrieben.

<Warning>
  Die `.heapsnapshot`-Datei enthält jeden String im Prozess, einschließlich Ihrer vollständigen Konversation und Anmeldedaten. Fügen Sie sie nicht an ein öffentliches Problem an und teilen Sie sie nicht.
</Warning>

Der Befehl gibt auch eine Zusammenfassung in der Konversation aus, die die Resident Set Size, JS Heap, Array Buffer und nicht berechneten nativen Speicher anzeigt, sowie alle Leak-Indikatoren, die er erkannt hat, wie z. B. eine hohe Speicherwachstumsrate oder eine ungewöhnlich hohe Anzahl offener Handles. Die Zusammenfassung gibt an, ob sich der meiste Speicher im JS Heap befindet, den der Snapshot erfasst, oder im nativen Speicher, den er nicht erfasst.

Melden Sie die Ausgabe oder untersuchen Sie sie selbst:

* **Melden Sie es**: öffnen Sie ein [GitHub-Problem](https://github.com/anthropics/claude-code/issues) und fügen Sie nur die `-diagnostics.json`-Datei an, die die Statistiken hinter der gedruckten Zusammenfassung enthält und keinen Konversationsinhalt oder Anmeldedaten
* **Untersuchen Sie es selbst**: Wenn die Zusammenfassung sagt, dass sich der meiste Speicher im JS Heap befindet, öffnen Sie die `.heapsnapshot`-Datei in Chrome DevTools unter Memory → Load und sortieren Sie nach Retained Size, um zu sehen, was den Speicher hält

Wenn die Zusammenfassung sagt, dass sich der meiste Speicher im nativen Speicher befindet, kann der Snapshot ihn nicht anzeigen; fügen Sie stattdessen die Leak-Indikatoren der Zusammenfassung in Ihren Bericht ein.

<h3 id="large-tables-are-cut-off-in-the-terminal">
  Große Tabellen werden im Terminal abgeschnitten
</h3>

Eine Markdown-Tabelle mit mehr als 200 Zeilen rendert ihre ersten 200 Zeilen gefolgt von einer `… N more rows not shown`-Zeile. Nur die Anzeige ist begrenzt: die vollständige Tabelle bleibt in der Konversation, und [`/copy`](/docs/de/commands) kopiert jede Zeile. Für eine Tabelle, die zu groß ist, um sie im Terminal zu lesen, bitten Sie Claude, sie stattdessen in eine Datei zu schreiben. Vor v2.1.208 renderte Claude Code jede Zeile, daher konnte das Fortsetzen einer Sitzung, die eine sehr große Tabelle enthielt, beim erneuten Rendern steckenbleiben.

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  Auto-Komprimierung stoppt mit einem Thrashing-Fehler
</h3>

Wenn Sie `Autocompact is thrashing: the context refilled to the limit...` sehen, war die automatische Komprimierung erfolgreich, aber eine Datei oder Toolausgabe hat das Kontextfenster sofort mehrmals hintereinander gefüllt. Claude Code stoppt die Wiederholung, um zu vermeiden, dass API-Aufrufe für eine Schleife verschwendet werden, die keinen Fortschritt macht.

Zur Wiederherstellung:

1. Bitten Sie Claude, die übergroße Datei in kleineren Chunks zu lesen, z. B. einen bestimmten Zeilenbereich oder eine Funktion, anstatt die ganze Datei
2. Führen Sie `/compact` mit einem Fokus aus, der die große Ausgabe löscht, z. B. `/compact keep only the plan and the diff`
3. Verschieben Sie die Arbeit mit großen Dateien zu einem [Subagent](/docs/de/sub-agents), damit sie in einem separaten Kontextfenster ausgeführt wird
4. Führen Sie `/clear` aus, wenn die frühere Konversation nicht mehr benötigt wird

<h3 id="command-hangs-or-freezes">
  Befehl hängt oder friert ein
</h3>

Wenn Claude Code nicht reagiert:

1. Drücken Sie Ctrl+C, um zu versuchen, den aktuellen Vorgang abzubrechen
2. Wenn nicht reagiert, müssen Sie möglicherweise das Terminal schließen und neu starten

Das Neustarten verliert Ihre Konversation nicht. Führen Sie `claude --resume` im selben Verzeichnis aus, um die Sitzung wieder aufzugreifen.

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  Verstümmelte oder beschädigte Text in einem integrierten Terminal des Editors
</h3>

Wenn Zeichen als Kästchen, Verschmierungen oder falsche Glyphen angezeigt werden, wenn Claude Code im integrierten Terminal von VS Code, Cursor oder Devin Desktop ausgeführt wird, ist der GPU-Renderer des Terminals wahrscheinlich die Ursache. Führen Sie `/terminal-setup` in Claude Code aus, um `terminal.integrated.gpuAcceleration` auf `"off"` zu setzen, oder setzen Sie es manuell in Ihren Editor-Einstellungen und laden Sie das Fenster neu. Siehe [Terminal configuration](/docs/de/terminal-config) für die anderen Einstellungen, die `/terminal-setup` schreibt.

<h3 id="mouse-wheel-scrolls-one-line-at-a-time-in-fullscreen-rendering">
  Mausrad scrollt im Vollbildmodus eine Zeile nach der anderen
</h3>

Im [Vollbildmodus](/docs/de/fullscreen) scrollt Claude Code die Konversation selbst, anstatt sie Ihrem Terminal zu überlassen. Wenn jede Rad-Kerbe weniger Zeilen bewegt, als Sie möchten, führen Sie `/scroll-speed` aus, um die Anzahl der Zeilen pro Kerbe zu erhöhen und zu speichern, oder setzen Sie die Umgebungsvariable `CLAUDE_CODE_SCROLL_SPEED`, außer im JetBrains IDE-Terminal, wo Claude Code sein eigenes Scroll-Handling anwendet und keines von beiden wirksam wird. Siehe [Mouse wheel scrolling](/docs/de/fullscreen#mouse-wheel-scrolling) für die Werte, die jeder akzeptiert.

Um schneller zu bewegen, ohne die Geschwindigkeit zu ändern, drücken Sie `PgUp` und `PgDn`, um jeweils einen halben Bildschirm zu scrollen. Um stattdessen den nativen Scrollback Ihres Terminals zu verwenden, führen Sie `/tui default` aus, um zum klassischen Renderer zu wechseln.

<h3 id="clipboard-commands-such-as-pbcopy-fail-inside-the-sandbox">
  Clipboard-Befehle wie `pbcopy` schlagen in der Sandbox fehl
</h3>

Wenn [Sandboxing](/docs/de/sandboxing) aktiviert ist, können Clipboard-Dienstprogramme wie `pbcopy`, `xclip` und `wl-copy` nicht auf die Systemzwischenablage von innerhalb eines Sandbox-Bash-Befehls zugreifen, wodurch Ihre Zwischenablage nach dem Piping von Text durch Claude unverändert bleibt.

Um die Ausgabe von Claude in Ihre Zwischenablage zu legen, bitten Sie Claude, den Inhalt in seiner Antwort auszudrucken, und führen Sie dann [`/copy`](/docs/de/commands) aus. `/copy` schreibt von Claude Code-Prozess selbst in die Zwischenablage, nicht von einem Sandbox-Befehl, daher blockiert Sandboxing es nicht. Es kann einen einzelnen Code-Block anstelle der gesamten Antwort kopieren, und es schreibt auch, was es kopiert hat, in eine Datei und gibt den Pfad aus, was Ihnen einen Fallback bietet, wenn der Zwischenablage-Schreibvorgang Ihr Terminal nicht erreicht, z. B. über SSH.

Wenn Claude Text an eines dieser Tools piped, fügt das Hinzufügen von `pbcopy *`, `wl-copy *` oder `xclip *` zu [`excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) diesen Aufruf nicht von selbst aus der Sandbox heraus.

<h3 id="copied-text-doesn’t-reach-your-local-clipboard-over-ssh">
  Kopierter Text erreicht Ihre lokale Zwischenablage nicht über SSH
</h3>

Wenn Claude Code auf einem Remote-Computer über SSH ausgeführt wird, kann es kein Clipboard-Tool auf Ihrem lokalen Computer ausführen. Außerhalb von tmux sendet Claude Code den Text beim Auswählen von Text im [Vollbildmodus](/docs/de/fullscreen) oder beim Ausführen von `/copy` stattdessen als OSC 52-Escape-Sequenz an Ihr Terminal. Ihr Terminal entscheidet, ob es auf Ihre Zwischenablage gelegt wird. `/copy` meldet `Copied to clipboard`, unabhängig davon, ob der Text angekommen ist, und außerhalb von tmux lautet die Auswahlmitteilung `sent N chars via OSC 52`.

Einige Terminals reagieren nicht auf OSC 52. iTerm2 ignoriert es, bis Sie **Settings > General > Selection > Applications in terminal may access clipboard** aktivieren, und macOS Terminal.app unterstützt es nicht.

Um den Text ohne OSC 52 zu erhalten:

* Halten Sie die native Auswahlaste Ihres Terminals gedrückt, während Sie ziehen, und kopieren Sie dann mit der üblichen Verknüpfung Ihres Terminals, z. B. `Cmd+C`. Die Taste ist `Fn` in Terminal.app und `Option` in iTerm2. [Keep native text selection](/docs/de/fullscreen#keep-native-text-selection) listet sie für andere Terminals auf.
* Setzen Sie [`CLAUDE_CODE_DISABLE_MOUSE=1`](/docs/de/env-vars) auf dem Remote-Computer, damit Ihr Terminal die Auswahl für die gesamte Sitzung handhabt.

<h3 id="search-and-discovery-issues">
  Such- und Erkennungsprobleme
</h3>

Wenn das Such-Tool, `@file`-Erwähnungen, benutzerdefinierte Agenten oder benutzerdefinierte Skills Dateien nicht finden, kann die gebündelte `ripgrep`-Binärdatei auf Ihrem System möglicherweise nicht ausgeführt werden. Installieren Sie das `ripgrep`-Paket Ihrer Plattform und teilen Sie Claude Code mit, dass es stattdessen verwendet werden soll:

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    brew install ripgrep
    ```
  </Tab>

  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt install ripgrep
    ```
  </Tab>

  <Tab title="Alpine">
    ```bash theme={null}
    apk add ripgrep
    ```

    `ripgrep` befindet sich im Community-Repository von Alpine. Wenn `apk` meldet, dass das Paket fehlt, siehe [Alpine Linux setup](/docs/de/setup#alpine-linux-and-musl-based-distributions).
  </Tab>

  <Tab title="Arch">
    ```bash theme={null}
    pacman -S ripgrep
    ```
  </Tab>

  <Tab title="Windows">
    ```powershell theme={null}
    winget install BurntSushi.ripgrep.MSVC
    ```
  </Tab>
</Tabs>

Setzen Sie dann `USE_BUILTIN_RIPGREP` auf `0`, entweder in Ihrer Shell [environment](/docs/de/env-vars) oder im `env`-Block Ihrer [`settings.json`](/docs/de/settings-reference#all-settings):

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

Um zu bestätigen, dass der Wechsel wirksam wurde, führen Sie `claude doctor` in Ihrem Terminal aus und überprüfen Sie, dass die Such-Zeile den Pfad Ihres System-ripgrep anstelle von `OK (bundled)` anzeigt.

<h3 id="slow-or-incomplete-search-results-on-wsl">
  Langsame oder unvollständige Suchergebnisse auf WSL
</h3>

Leistungseinbußen beim Lesen von Festplatten beim [Arbeiten über Dateisysteme auf WSL](https://learn.microsoft.com/en-us/windows/wsl/filesystems) können zu weniger als erwarteten Übereinstimmungen führen, wenn Claude Code auf WSL verwendet wird. Die Suche funktioniert immer noch, gibt aber weniger Ergebnisse zurück als auf einem nativen Dateisystem.

<Note>
  `claude doctor` zeigt Search in diesem Fall als OK an.
</Note>

**Lösungen:**

1. **Spezifischere Suchen einreichen**: Reduzieren Sie die Anzahl der durchsuchten Dateien, indem Sie Verzeichnisse oder Dateitypen angeben: "Search for JWT validation logic in the auth-service package" oder "Find use of md5 hash in JS files".

2. **Projekt auf Linux-Dateisystem verschieben**: Stellen Sie sicher, dass sich Ihr Projekt nach Möglichkeit auf dem Linux-Dateisystem (`/home/`) anstelle des Windows-Dateisystems (`/mnt/c/`) befindet.

3. **Verwenden Sie stattdessen natives Windows**: Erwägen Sie, Claude Code nativ unter Windows anstelle von WSL auszuführen, um eine bessere Dateisystem-Leistung zu erzielen.

<h2 id="get-more-help">
  Weitere Hilfe erhalten
</h2>

Wenn Sie Probleme haben, die hier nicht behandelt werden:

1. Führen Sie `/doctor` aus, um eine Installationsprüfung durchzuführen, und `/mcp`, um den MCP-Serverstatus zu überprüfen
2. Verwenden Sie den `/feedback`-Befehl in Claude Code, um Probleme direkt an Anthropic zu melden
3. Überprüfen Sie das [GitHub-Repository](https://github.com/anthropics/claude-code) auf bekannte Probleme
4. Fragen Sie Claude direkt nach seinen Fähigkeiten und Funktionen. Claude hat integrierten Zugriff auf seine Dokumentation.

Bei Problemen mit Ihrem Konto, der Abrechnung oder dem Abonnement wenden Sie sich stattdessen an den Anthropic-Support: Melden Sie sich bei [claude.ai](https://claude.ai) an (Console-Benutzer: [platform.claude.com](https://platform.claude.com)), klicken Sie auf Ihre Initialen in der unteren linken Ecke, und wählen Sie **Hilfe erhalten**. Siehe [How to get support](https://support.claude.com/en/articles/9015913-how-to-get-support) für den vollständigen Ablauf, einschließlich wer auf jedem Plan einen menschlichen Agenten erreichen kann.
