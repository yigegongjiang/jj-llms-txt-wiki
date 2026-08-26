> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# JetBrains IDEs

> Verwenden Sie Claude Code mit JetBrains IDEs einschließlich IntelliJ, PyCharm, WebStorm und mehr

Claude Code integriert sich mit JetBrains IDEs durch ein dediziertes Plugin und bietet Funktionen wie interaktive Diff-Anzeige, Freigabe von Auswahlkontext und mehr.

<h2 id="supported-ides">
  Unterstützte IDEs
</h2>

Das Claude Code Plugin funktioniert mit den meisten JetBrains IDEs, einschließlich:

* IntelliJ IDEA
* PyCharm
* Android Studio
* WebStorm
* PhpStorm
* GoLand

<h2 id="features">
  Funktionen
</h2>

* **Schnellstart**: Verwenden Sie `Cmd+Esc` (Mac) oder `Ctrl+Esc` (Windows/Linux), um Claude Code direkt aus Ihrem Editor zu öffnen, oder klicken Sie auf die Claude Code Schaltfläche in der Benutzeroberfläche
* **Diff-Anzeige**: Code-Änderungen können direkt im IDE Diff-Viewer anstelle des Terminals angezeigt werden
* **Auswahlkontext**: Die aktuelle Auswahl oder der aktuelle Tab in der IDE wird automatisch mit Claude Code geteilt. [`Read` Ablehnungsregeln](/docs/de/permissions#read-and-edit) blockieren diese Freigabe für übereinstimmende Dateien
* **Dateireferenz-Verknüpfungen**: Verwenden Sie `Cmd+Option+K` (Mac) oder `Alt+Ctrl+K` (Linux/Windows), um Dateireferenzen wie `@src/auth.ts#L1-99` einzufügen
* **Diagnose-Freigabe**: Diagnosefehler aus der IDE, wie Lint- und Syntaxfehler, werden automatisch mit Claude geteilt, während Sie arbeiten

<h2 id="installation">
  Installation
</h2>

Das Plugin führt den `claude`-Befehl im integrierten Terminal Ihrer IDE aus und verbindet sich damit. Es bündelt keine eigene Kopie der CLI, daher installieren Sie beide Komponenten:

<Steps>
  <Step title="Installieren Sie die Claude Code CLI">
    Folgen Sie dem [Schnellstart](/docs/de/quickstart), um die CLI zu installieren, falls Sie dies noch nicht getan haben. Das Plugin zeigt eine Benachrichtigung 'Claude Code kann nicht gestartet werden" an, wenn `claude` nicht in Ihrem PATH vorhanden ist.
  </Step>

  <Step title="Installieren Sie das JetBrains-Plugin">
    Installieren Sie das [Claude Code Plugin](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) aus dem JetBrains Marketplace und starten Sie Ihre IDE neu.
  </Step>
</Steps>

Wenn `claude` an einem Ort installiert ist, den Ihre IDE nicht finden kann, legen Sie den vollständigen Pfad in der [Claude-Befehlseinstellung](#general-settings) des Plugins fest.

Claude Code funktioniert mit jedem bezahlten Claude-Abonnement (Pro, Max, Team oder Enterprise) oder einem Claude Console-Konto, und es ist kein API-Schlüssel erforderlich. Sie werden aufgefordert, sich [anzumelden](/docs/de/authentication#log-in-to-claude-code), wenn Sie `claude` zum ersten Mal ausführen.

<Note>
  Nach der Installation des Plugins müssen Sie Ihre IDE möglicherweise vollständig neu starten, damit es wirksam wird.
</Note>

<h2 id="usage">
  Verwendung
</h2>

<h3 id="from-your-ide">
  Aus Ihrer IDE
</h3>

Führen Sie `claude` aus dem integrierten Terminal Ihrer IDE aus, und alle Integrationsfunktionen sind aktiv.

<h3 id="from-external-terminals">
  Aus externen Terminals
</h3>

Verwenden Sie den `/ide` Befehl in einem beliebigen externen Terminal, um Claude Code mit Ihrer JetBrains IDE zu verbinden und alle Funktionen zu aktivieren:

```bash theme={null}
claude
```

```text theme={null}
/ide
```

Wenn Sie möchten, dass Claude Zugriff auf die gleichen Dateien wie Ihre IDE hat, starten Sie Claude Code aus dem gleichen Verzeichnis wie Ihr IDE-Projektstammverzeichnis.

<h2 id="configuration">
  Konfiguration
</h2>

<h3 id="claude-code-settings">
  Claude Code Einstellungen
</h3>

Konfigurieren Sie die IDE-Integration durch Claude Code Einstellungen:

1. Führen Sie `claude` aus
2. Geben Sie den `/config` Befehl ein
3. Stellen Sie das Diff-Tool auf `auto` ein, um Diffs in der IDE anzuzeigen, oder auf `terminal`, um sie im Terminal zu behalten

<h3 id="plugin-settings">
  Plugin-Einstellungen
</h3>

Konfigurieren Sie das Claude Code Plugin, indem Sie zu **Einstellungen → Tools → Claude Code \[Beta]** gehen:

<h4 id="general-settings">
  Allgemeine Einstellungen
</h4>

* **Claude Befehl**: Geben Sie einen benutzerdefinierten Befehl an, um Claude auszuführen, zum Beispiel `claude`, `/usr/local/bin/claude` oder `npx @anthropic-ai/claude-code`
* **Benachrichtigung für Claude-Befehl nicht gefunden unterdrücken**: Überspringen Sie Benachrichtigungen über das Nichtfinden des Claude-Befehls
* **Option+Enter für mehrzeilige Eingabeaufforderungen aktivieren**: Nur auf macOS. Wenn aktiviert, fügt Option+Enter neue Zeilen in Claude Code Eingabeaufforderungen ein. Deaktivieren Sie dies, wenn die Option-Taste unerwartet erfasst wird. Erfordert einen Terminal-Neustart.
* **Automatische Updates aktivieren**: Automatisch nach Plugin-Updates suchen und diese installieren, angewendet beim Neustart

<Tip>
  Für WSL-Benutzer: Stellen Sie `wsl -d Ubuntu -- bash -lic "claude"` als Ihren Claude-Befehl ein (ersetzen Sie `Ubuntu` durch Ihren WSL-Distributionsnamen)
</Tip>

<h4 id="esc-key-configuration">
  ESC-Taste Konfiguration
</h4>

Wenn die ESC-Taste Claude Code Operationen in JetBrains Terminals nicht unterbricht:

1. Gehen Sie zu **Einstellungen → Tools → Terminal**
2. Entweder:
   * Deaktivieren Sie „Fokus mit Escape zum Editor verschieben", oder
   * Klicken Sie auf „Terminal-Tastenkombinationen konfigurieren" und löschen Sie die Verknüpfung „Fokus zum Editor wechseln"
3. Wenden Sie die Änderungen an

Dies ermöglicht es der ESC-Taste, Claude Code Operationen ordnungsgemäß zu unterbrechen.

<h2 id="special-configurations">
  Spezielle Konfigurationen
</h2>

<h3 id="remote-development">
  Remote-Entwicklung
</h3>

<Warning>
  Bei Verwendung von JetBrains Remote Development müssen Sie das Plugin auf dem Remote-Host über **Einstellungen → Plugin (Host)** installieren.
</Warning>

Das Plugin muss auf dem Remote-Host installiert werden, nicht auf Ihrem lokalen Client-Computer.

<h3 id="wsl-configuration">
  WSL-Konfiguration
</h3>

Wenn Sie Claude Code auf WSL2 mit einer JetBrains IDE verwenden und „Keine verfügbaren IDEs erkannt" sehen, ist die Ursache normalerweise WSL2s NAT-Netzwerk oder die Windows Firewall, die die Verbindung zwischen WSL2 und der IDE blockiert, die auf dem Windows-Host ausgeführt wird. WSL1 verwendet das Netzwerk des Hosts direkt und ist nicht betroffen.

<h4 id="allow-wsl2-traffic-through-windows-firewall">
  WSL2-Datenverkehr durch Windows Firewall zulassen
</h4>

Dies ist die empfohlene Lösung, da sie Ihren vorhandenen WSL2-Netzwerkmodus beibehält.

<Steps>
  <Step title="Finden Sie Ihre WSL2 IP-Adresse">
    Führen Sie in Ihrer WSL-Shell Folgendes aus:

    ```bash theme={null}
    hostname -I
    ```

    Notieren Sie sich das Subnetz, zum Beispiel `172.21.123.45` befindet sich in `172.21.0.0/16`.
  </Step>

  <Step title="Erstellen Sie eine Firewall-Regel">
    Öffnen Sie PowerShell als Administrator und führen Sie Folgendes aus, passen Sie den IP-Bereich an Ihr Subnetz an:

    ```powershell theme={null}
    New-NetFirewallRule -DisplayName "Allow WSL2 Internal Traffic" -Direction Inbound -Protocol TCP -Action Allow -RemoteAddress 172.21.0.0/16 -LocalAddress 172.21.0.0/16
    ```
  </Step>

  <Step title="Starten Sie Ihre IDE und Claude Code neu">
    Schließen Sie beide und öffnen Sie sie erneut, damit die neue Regel wirksam wird.
  </Step>
</Steps>

<h4 id="switch-wsl2-to-mirrored-networking">
  Wechseln Sie WSL2 zu gespiegeltem Netzwerk
</h4>

Gespiegeltes Netzwerk erfordert Windows 11 22H2 oder später. Wenn Sie Windows 10 verwenden, verwenden Sie stattdessen die Firewall-Regel oben.

Fügen Sie dies zu `.wslconfig` in Ihrem Windows-Benutzerverzeichnis hinzu:

```ini theme={null}
[wsl2]
networkingMode=mirrored
```

Starten Sie dann WSL mit `wsl --shutdown` von PowerShell neu.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="plugin-not-working">
  Plugin funktioniert nicht
</h3>

Wenn das Plugin installiert ist, aber Claude Code Funktionen nicht in Ihrer IDE angezeigt werden:

* Stellen Sie sicher, dass Sie Claude Code aus dem Projektstammverzeichnis ausführen
* Überprüfen Sie, dass das JetBrains Plugin in den IDE-Einstellungen aktiviert ist
* Starten Sie die IDE vollständig neu (möglicherweise müssen Sie dies mehrmals tun)
* Stellen Sie für Remote Development sicher, dass das Plugin auf dem Remote-Host installiert ist

<h3 id="ide-not-detected">
  IDE nicht erkannt
</h3>

Wenn das Ausführen von `claude` 'Keine verfügbaren IDEs erkannt" anzeigt:

* Überprüfen Sie, dass das Plugin installiert und aktiviert ist
* Starten Sie die IDE vollständig neu
* Überprüfen Sie, dass Sie Claude Code aus dem integrierten Terminal ausführen
* Für WSL-Benutzer lesen Sie [WSL-Konfiguration](#wsl-configuration) oben

<h3 id="command-not-found">
  Befehl nicht gefunden
</h3>

Wenn das Klicken auf das Claude-Symbol „Befehl nicht gefunden" anzeigt:

1. Überprüfen Sie, dass Claude Code installiert ist, indem Sie `claude --version` in einem Terminal ausführen
2. Konfigurieren Sie den Claude-Befehlspfad in den Plugin-Einstellungen
3. Für WSL-Benutzer verwenden Sie das WSL-Befehlsformat, das im Konfigurationsabschnitt erwähnt wird

<h2 id="security-considerations">
  Sicherheitsaspekte
</h2>

Wenn Claude Code in einer JetBrains IDE im [`acceptEdits` Berechtigungsmodus](/docs/de/permission-modes#auto-approve-file-edits-with-acceptedits-mode) ausgeführt wird, kann es möglicherweise IDE-Konfigurationsdateien ändern, die automatisch von Ihrer IDE ausgeführt werden können. Dies kann das Risiko der Ausführung von Claude Code im `acceptEdits` Modus erhöhen und es ermöglichen, Claude Code Berechtigungsaufforderungen für die Bash-Ausführung zu umgehen.

Bei der Ausführung in JetBrains IDEs sollten Sie Folgendes beachten:

* Verwenden Sie den manuellen Genehmigungsmodus für Bearbeitungen
* Achten Sie besonders darauf, dass Claude nur mit vertrauenswürdigen Eingabeaufforderungen verwendet wird
* Seien Sie sich bewusst, welche Dateien Claude Code ändern kann

Für Claude Code Installations- oder Anmeldeprobleme außerhalb der IDE lesen Sie [Troubleshoot installation and login](/docs/de/troubleshoot-install).

<h3 id="the-built-in-ide-mcp-server">
  Der integrierte IDE MCP Server
</h3>

Wenn das Plugin aktiv ist, führt es einen lokalen MCP Server aus, mit dem sich die CLI automatisch verbindet. Auf diese Weise öffnet die CLI Diffs im nativen Diff-Viewer der IDE, liest Ihre aktuelle Auswahl für `@`-Erwähnungen und zieht Inspektionsdiagnostiken in das Gespräch.

Der Server heißt `ide` und ist in `/mcp` verborgen, da es nichts zu konfigurieren gibt. Wenn Ihre Organisation jedoch einen [`PreToolUse` Hook](/docs/de/hooks#pretooluse) verwendet, um MCP-Tools auf eine Zulassungsliste zu setzen, müssen Sie wissen, dass er existiert.

**Auswahl und Kontext der offenen Datei.** Während der Verbindung bezieht die CLI Ihre aktuelle Editor-Auswahl und den Pfad der aktiven Datei als Kontext in jeden Prompt ein, den Sie senden. Das Transkript zeigt eine `⧉ Selected N lines from <file>` Zeile, wenn dies geschieht. Um eine sensible Datei wie `.env` auszuschließen, fügen Sie eine [`Read` Ablehnungsregel](/docs/de/permissions#read-and-edit) für ihren Pfad hinzu. Eine entsprechende Ablehnungsregel verhindert, dass sowohl der ausgewählte Text als auch die Benachrichtigung über die offene Datei für diese Datei Claude erreichen.

**Transport und Authentifizierung.** Der Server lauscht auf einem vom Betriebssystem zugewiesenen kurzlebigen Port, und der Port ist nicht konfigurierbar. Der Transport ist unverschlüsseltes `ws://`; auf Loopback kann jeder Prozess, der den Datenverkehr erfassen kann, auch das Token aus der Lock-Datei lesen, daher würde TLS keinen Schutz gegen einen lokalen Angreifer bieten. Jeder IDE-Start generiert ein neues zufälliges Auth-Token, schreibt es in eine Lock-Datei unter `~/.claude/ide/<port>.lock`, und die CLI muss es als `X-Claude-Code-Ide-Authorization` Header präsentieren, um sich zu verbinden. Wenn `CLAUDE_CONFIG_DIR` gesetzt ist, wird die Lock-Datei stattdessen in `$CLAUDE_CONFIG_DIR/ide/` geschrieben.

**Dem Modell ausgesetzte Tools.** Der Server hostet mehrere Tools, aber nur eines ist für das Modell sichtbar. Der Rest ist internes RPC, das die CLI für ihre eigene Benutzeroberfläche verwendet, wie das Öffnen von Diffs und das Lesen von Auswahlen, und wird gefiltert, bevor die Werkzeugliste Claude erreicht.

| Tool-Name (wie von Hooks gesehen) | Was es tut                                                                                                                                     | Schreibgeschützt |
| --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| `mcp__ide__getDiagnostics`        | Gibt die Inspektionsdiagnostiken der IDE zurück, die Fehler und Warnungen, die im Editor angezeigt werden. Optional auf eine Datei beschränkt. | Ja               |

Das JetBrains Plugin stellt dem Modell kein Code-Ausführungs-Tool zur Verfügung.

**Listening-Schnittstelle.** Welche Netzwerkschnittstelle der Server bindet, wird durch **Accept connections from all network interfaces** unter **Settings → Tools → Claude Code \[Beta] → Networking (Advanced)** gesteuert. Mit der deaktivierten Einstellung lauscht der Server nur auf `127.0.0.1` und ist von anderen Hosts nicht erreichbar. Mit aktivierter Einstellung ist der Port von Ihrem lokalen Netzwerk aus erreichbar. Die Einstellung existiert für Fälle, in denen die CLI die IDE nicht über Loopback erreichen kann, wie WSL2 mit Standard-NAT-Netzwerk oder ein Remote-IDE-Setup; siehe [WSL-Konfiguration](#wsl-configuration) für dieses Szenario.

<Warning>
  Das Aktivieren von **Accept connections from all network interfaces** macht den IDE MCP Port von Ihrem lokalen Netzwerk aus erreichbar. Verbindungen erfordern weiterhin das Auth-Token aus der Lock-Datei, aber da der Transport unverschlüsseltes `ws://` ist, werden sowohl der Sitzungsdatenverkehr als auch dieses Token über das Netzwerk im Klartext übertragen, wenn die Einstellung aktiviert ist. Aktivieren Sie es nur, wenn Loopback wirklich nicht funktioniert. Für WSL2 bevorzugen Sie [mirrored networking](#switch-wsl2-to-mirrored-networking), damit die Windows-Loopback-Schnittstelle mit der Linux-VM geteilt wird und der Socket auf Loopback bleiben kann.
</Warning>
