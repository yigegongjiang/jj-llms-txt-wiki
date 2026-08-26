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
| `API Error: 5xx`, `529 Overloaded`, `429`, Request-Validierungsfehler                                                                                          | [Fehlerreferenz](/docs/de/errors)                                                                       |
| `model not found` oder `you may not have access to it`                                                                                                         | [Fehlerreferenz](/docs/de/errors#theres-an-issue-with-the-selected-model)                               |
| VS Code-Erweiterung verbindet sich nicht oder erkennt Claude nicht                                                                                             | [VS Code-Integration](/docs/de/vs-code#fix-common-issues)                                               |
| JetBrains-Plugin oder IDE wird nicht erkannt                                                                                                                   | [JetBrains-Integration](/docs/de/jetbrains#troubleshooting)                                             |
| Hohe CPU oder Speicher, langsame Antworten, Hänger, Suche findet Dateien nicht                                                                                 | [Leistung und Stabilität](#performance-and-stability) unten                                        |

Wenn Sie sich nicht sicher sind, welcher Fall zutrifft, führen Sie `/doctor` in Claude Code aus, um eine automatisierte Überprüfung Ihrer Installation, Einstellungen, Erweiterungen und Kontextnutzung durchzuführen; es schlägt Korrektionen vor, die es nach Ihrer Bestätigung anwenden kann. Wenn `claude` überhaupt nicht startet, führen Sie stattdessen `claude doctor` aus Ihrer Shell aus. Führen Sie `/mcp` aus, um den MCP-Server-Status zu überprüfen.

<h2 id="performance-and-stability">
  Leistung und Stabilität
</h2>

Diese Abschnitte behandeln Probleme im Zusammenhang mit Ressourcennutzung, Reaktionsfähigkeit und Suchverhalten.

<h3 id="high-cpu-or-memory-usage">
  Hohe CPU- oder Speichernutzung
</h3>

Claude Code ist für die Zusammenarbeit mit den meisten Entwicklungsumgebungen konzipiert, kann aber bei der Verarbeitung großer Codebases erhebliche Ressourcen verbrauchen. Wenn Sie Leistungsprobleme haben:

1. Verwenden Sie `/compact` regelmäßig, um die Kontextgröße zu reduzieren
2. Schließen und starten Sie Claude Code zwischen großen Aufgaben neu
3. Erwägen Sie, große Build-Verzeichnisse zu Ihrer `.gitignore`-Datei hinzuzufügen
4. Starten Sie mit [`claude --safe-mode`](/docs/de/cli-reference#cli-flags) neu, um zu überprüfen, ob ein Plugin, MCP-Server oder Hook die Quelle ist. Dies deaktiviert alle Anpassungen für die Sitzung; wenn die Nutzung sinkt, siehe [Konfiguration debuggen](/docs/de/debug-your-config#test-against-a-clean-configuration), um herauszufinden, welche

Wenn die Speichernutzung nach diesen Schritten hoch bleibt, führen Sie `/heapdump` aus, um einen JavaScript-Heap-Snapshot und eine Speicheraufschlüsselung auf `~/Desktop` zu schreiben. Auf Linux ohne Desktop-Ordner werden die Dateien in Ihr Home-Verzeichnis geschrieben.

Die Aufschlüsselung zeigt die Resident Set Size, JS Heap, Array Buffer und nicht berechneten nativen Speicher, was hilft zu identifizieren, ob das Wachstum in JavaScript-Objekten oder in nativem Code liegt. Um Retainer zu überprüfen, öffnen Sie die `.heapsnapshot`-Datei in Chrome DevTools unter Memory → Load; die Aufschlüsselung ist die Datei, die auf `-diagnostics.json` endet.

<Warning>
  Die `.heapsnapshot`-Datei enthält jeden String im Prozess. Fügen Sie sie nicht an ein öffentliches Issue an und teilen Sie sie nicht. Fügen Sie nur die `-diagnostics.json`-Datei an, wenn Sie ein Speicherproblem auf [GitHub](https://github.com/anthropics/claude-code/issues) melden. Diese Datei enthält Speicherstatistiken und keinen Gesprächsinhalt oder Anmeldedaten.
</Warning>

<h3 id="large-tables-are-cut-off-in-the-terminal">
  Große Tabellen werden im Terminal abgeschnitten
</h3>

Eine Markdown-Tabelle mit mehr als 200 Zeilen rendert ihre ersten 200 Zeilen gefolgt von einer `… N more rows not shown`-Zeile. Nur die Anzeige ist begrenzt: Die vollständige Tabelle bleibt in der Konversation, und [`/copy`](/docs/de/commands) kopiert jede Zeile. Für eine Tabelle, die zu groß ist, um sie im Terminal zu lesen, bitten Sie Claude, sie stattdessen in eine Datei zu schreiben. Vor v2.1.208 renderte Claude Code jede Zeile, daher konnte das Fortsetzen einer Sitzung, die eine sehr große Tabelle enthielt, beim erneuten Rendern steckenbleiben.

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  Auto-Kompaktierung stoppt mit einem Thrashing-Fehler
</h3>

Wenn Sie `Autocompact is thrashing: the context refilled to the limit...` sehen, war die automatische Kompaktierung erfolgreich, aber eine Datei oder ein Tool-Output hat das Kontextfenster sofort mehrmals hintereinander gefüllt. Claude Code stoppt die Wiederholung, um zu vermeiden, dass API-Aufrufe auf einer Schleife verschwendet werden, die keinen Fortschritt macht.

Um sich zu erholen:

1. Bitten Sie Claude, die übergroße Datei in kleineren Chunks zu lesen, z. B. einen bestimmten Zeilenbereich oder eine Funktion, statt der ganzen Datei
2. Führen Sie `/compact` mit einem Fokus aus, der die große Ausgabe löscht, z. B. `/compact keep only the plan and the diff`
3. Verschieben Sie die Arbeit mit großen Dateien zu einem [Subagenten](/docs/de/sub-agents), damit er in einem separaten Kontextfenster ausgeführt wird
4. Führen Sie `/clear` aus, wenn das frühere Gespräch nicht mehr benötigt wird

<h3 id="command-hangs-or-freezes">
  Befehl hängt oder friert ein
</h3>

Wenn Claude Code nicht reagiert:

1. Drücken Sie Strg+C, um zu versuchen, den aktuellen Vorgang abzubrechen
2. Wenn nicht reagiert, müssen Sie möglicherweise das Terminal schließen und neu starten

Das Neustarten verliert Ihre Konversation nicht. Führen Sie `claude --resume` im selben Verzeichnis aus, um die Sitzung fortzusetzen.

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  Verzerrter oder beschädigter Text im integrierten Terminal eines Editors
</h3>

Wenn Zeichen als Kästchen, Verschmierungen oder falsche Glyphen angezeigt werden, wenn Sie Claude Code im integrierten Terminal von VS Code, Cursor oder Devin Desktop ausführen, ist der GPU-Renderer des Terminals wahrscheinlich die Ursache. Führen Sie `/terminal-setup` in Claude Code aus, um `terminal.integrated.gpuAcceleration` auf `"off"` zu setzen, oder setzen Sie es manuell in Ihren Editor-Einstellungen und laden Sie das Fenster neu. Siehe [Terminalkonfiguration](/docs/de/terminal-config) für die anderen Einstellungen, die `/terminal-setup` schreibt.

<h3 id="search-and-discovery-issues">
  Such- und Erkennungsprobleme
</h3>

Wenn das Such-Tool, `@file`-Erwähnungen, benutzerdefinierte Agenten oder benutzerdefinierte Skills Dateien nicht finden, kann die gebündelte `ripgrep`-Binärdatei auf Ihrem System möglicherweise nicht ausgeführt werden. Installieren Sie das `ripgrep`-Paket Ihrer Plattform und teilen Sie Claude Code mit, es stattdessen zu verwenden:

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

Setzen Sie dann `USE_BUILTIN_RIPGREP=0` in Ihrer [Umgebung](/docs/de/env-vars).

<h3 id="slow-or-incomplete-search-results-on-wsl">
  Langsame oder unvollständige Suchergebnisse auf WSL
</h3>

Leistungseinbußen beim Lesen von Festplatten beim [Arbeiten über Dateisysteme auf WSL](https://learn.microsoft.com/en-us/windows/wsl/filesystems) können zu weniger als erwarteten Übereinstimmungen führen, wenn Sie Claude Code auf WSL verwenden. Die Suche funktioniert immer noch, gibt aber weniger Ergebnisse zurück als auf einem nativen Dateisystem.

<Note>
  `claude doctor` zeigt in diesem Fall die Suche als OK an.
</Note>

**Lösungen:**

1. **Senden Sie spezifischere Suchen**: Reduzieren Sie die Anzahl der durchsuchten Dateien, indem Sie Verzeichnisse oder Dateitypen angeben: „Search for JWT validation logic in the auth-service package" oder „Find use of md5 hash in JS files".

2. **Verschieben Sie das Projekt auf das Linux-Dateisystem**: Stellen Sie sicher, dass sich Ihr Projekt auf dem Linux-Dateisystem (`/home/`) statt auf dem Windows-Dateisystem (`/mnt/c/`) befindet.

3. **Verwenden Sie stattdessen natives Windows**: Erwägen Sie, Claude Code nativ unter Windows statt über WSL auszuführen, um eine bessere Dateisystem-Leistung zu erzielen.

<h2 id="get-more-help">
  Weitere Hilfe erhalten
</h2>

Wenn Sie Probleme haben, die hier nicht behandelt werden:

1. Führen Sie `/doctor` aus, um eine Installationsprüfung durchzuführen, und `/mcp`, um den MCP-Serverstatus zu überprüfen
2. Verwenden Sie den `/feedback`-Befehl in Claude Code, um Probleme direkt an Anthropic zu melden
3. Überprüfen Sie das [GitHub-Repository](https://github.com/anthropics/claude-code) auf bekannte Probleme
4. Fragen Sie Claude direkt nach seinen Fähigkeiten und Funktionen. Claude hat integrierten Zugriff auf seine Dokumentation.
