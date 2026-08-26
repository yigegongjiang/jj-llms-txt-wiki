> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mit MCP-Servern verbinden

> Fügen Sie einen MCP-Server zu Claude Code hinzu, überprüfen Sie die Verbindung und finden Sie die Konfiguration auf der Festplatte.

Das [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction) ermöglicht Claude Code die Verwendung von Tools über seinen integrierten Satz hinaus, wie z. B. die Suche in einem Issue-Tracker, das Abfragen einer Datenbank oder die Steuerung eines Webbrowsers. Diese Tools stammen von MCP-Servern, die auf Ihrem Computer oder als gehostete Dienste ausgeführt werden.

Diese Anleitung führt Sie durch die End-to-End-Verbindung eines MCP-Servers mit der Claude Code CLI. Am Ende haben Sie einen verbundenen und reagierenden Server, wissen, wo seine Konfiguration auf der Festplatte gespeichert ist, und wissen, wie Sie die häufigsten Verbindungsfehler beheben.

<Note>
  Sie können MCP-Server auch von anderen Oberflächen aus hinzufügen, einschließlich der Desktop-App, VS Code und dem Web. Siehe [Von anderen Oberflächen verbinden](#connect-from-other-surfaces).
</Note>

Für alle Möglichkeiten zum Verbinden und Konfigurieren von MCP-Servern in Claude Code siehe die [MCP-Referenz](/docs/de/mcp).

<h2 id="before-you-begin">
  Bevor Sie beginnen
</h2>

Stellen Sie sicher, dass Sie haben:

* [Claude Code installiert](/docs/de/quickstart) und authentifiziert
* Ein Terminal in einem Projektverzeichnis geöffnet. Jedes Verzeichnis funktioniert, auch ein leeres.

<h2 id="add-and-verify-a-server">
  Einen Server hinzufügen und überprüfen
</h2>

Das folgende Beispiel verbindet sich mit dem [Claude Code-Dokumentations-MCP-Server](https://code.claude.com/docs/mcp), einem gehosteten Server mit Volltextsuche über die Claude Code-Dokumentation. Er erfordert keine Authentifizierung oder spezielle Konfiguration, daher funktioniert er gut als erster Server zum Testen des Setup-Ablaufs.

Die Schritte sind für jeden Server gleich: fügen Sie ihn hinzu, überprüfen Sie den Verbindungsstatus, verwenden Sie ihn dann in einer Sitzung, mit einem optionalen Bereinigungsschritt am Ende. Einige Server fügen einen Schritt hinzu, wie z. B. eine Browser-Anmeldung, die in [Zusätzliche MCP-Server-Beispiele](#additional-mcp-server-examples) gezeigt wird. Weitere Server zum Verbinden finden Sie im [Anthropic-Verzeichnis](/docs/de/mcp#find-and-build-mcp-servers).

<Steps>
  <Step title="Fügen Sie den MCP-Server hinzu">
    Registrieren Sie den Server bei Claude Code. Führen Sie dies in Ihrem Terminal aus, nicht in einer `claude`-Sitzung: Sie konfigurieren den Server vor dem Starten einer Konversation.

    ```bash theme={null}
    claude mcp add --transport http claude-code-docs https://code.claude.com/docs/mcp
    ```

    Die Teile des Befehls:

    * `claude mcp add`: registriert einen Server bei Claude Code.
    * `--transport http`: Der Server wird unter einer URL gehostet, anstatt als lokaler Prozess ausgeführt zu werden.
    * `claude-code-docs`: ein Name, den Sie sich ausdenken. Das Aufrufen desselben Servers als `docs` würde identisch funktionieren. Claude Code verwendet den Namen, den Sie wählen, um die Tools des Servers in Claudes Ausgabe zu kennzeichnen und um auf den Server in Befehlen wie `claude mcp remove` zu verweisen.
    * `https://code.claude.com/docs/mcp`: die URL, unter der der Server gehostet wird.

    Der Befehl gibt eine Bestätigung wie `Added HTTP MCP server claude-code-docs with URL: https://code.claude.com/docs/mcp to local config` aus. Der Teil `local config` bedeutet, dass der Server bei Ihnen in diesem Projekt registriert ist: Wenn Sie Claude Code in einem anderen Projekt starten, ist dieser Server dort nicht aktiv. Um einen Server einmal für alle Ihre Projekte zu registrieren, fügen Sie ihn im Benutzerbereich hinzu, was in [Server-Bereich ändern](#change-server-scope) behandelt wird.
  </Step>

  <Step title="Überprüfen Sie den Verbindungsstatus">
    Bestätigen Sie, dass der Server in Ihrer Serverliste angezeigt wird, und überprüfen Sie seinen Status:

    ```bash theme={null}
    claude mcp list
    ```

    Der Server wird mit einem Statusindikator angezeigt:

    | Status                             | Bedeutung                                                                                                                                                                                                                    |
    | :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `✓ Connected`                      | Einsatzbereit. Das sollten Sie für `claude-code-docs` sehen                                                                                                                                                                  |
    | `! Connected · tools fetch failed` | Der Server hat sich verbunden, konnte aber seine Tools nicht auflisten. Führen Sie `claude mcp get <name>` aus, um die Fehlerdetails zu erhalten                                                                             |
    | `! Needs authentication`           | Der Server ist erreichbar, benötigt aber eine Browser-Anmeldung oder ein Token, das mit `--header` übergeben wird. Siehe [Verbinden Sie einen Server, der eine Anmeldung erfordert](#connect-a-server-that-requires-sign-in) |
    | `✗ Failed to connect`              | Server hat nicht reagiert. Siehe [Fehlerbehebung](#troubleshooting)                                                                                                                                                          |
    | `✗ Connection error`               | Der Verbindungsversuch hat einen Fehler ausgelöst. Siehe [Fehlerbehebung](#troubleshooting)                                                                                                                                  |
    | `⏸ Pending approval`               | Ein projektbezogener Server, den Sie noch nicht genehmigt haben. Siehe [Bearbeiten Sie .mcp.json direkt](#edit-mcp-json-directly)                                                                                            |
  </Step>

  <Step title="Verwenden Sie den Server">
    Starten Sie eine Sitzung und bitten Sie Claude, den neuen Server nach Name zu verwenden:

    ```bash theme={null}
    claude
    ```

    ```text theme={null}
    Use the claude-code-docs server to look up what MCP_TIMEOUT does
    ```

    <Info>
      Sie müssen normalerweise keinen Server in Ihrer Eingabeaufforderung benennen, da Claude relevante Tools automatisch auswählt. Das Benennen hier garantiert, dass die Demonstration über den neuen Server läuft, anstatt über ein anderes Tool wie Web-Abruf, das die gleiche Frage beantworten könnte.
    </Info>

    Wenn Claude den Server zum ersten Mal aufruft, fragt es um Erlaubnis, das neue Tool zu verwenden. Genehmigen Sie es, um fortzufahren. Der Tool-Aufruf in Claudes Ausgabe ist mit dem Servernamen gekennzeichnet, anhand dessen Sie bestätigen, dass die Antwort vom MCP-Server und nicht von Claudes integriertem Wissen stammt.
  </Step>

  <Step title="Entfernen Sie den Server">
    Dieser Schritt ist optional. Wenn Sie mit dem Experimentieren fertig sind, können Sie den Server entfernen:

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    <Note>
      Jeder verbundene Server nimmt Platz in [Claudes Kontextfenster](/docs/de/how-claude-code-works#the-context-window) ein, da seine Tool-Namen und Server-Anweisungen in jede Sitzung geladen werden. Das Entfernen von Servern, die Sie nicht mehr verwenden, hält diesen Platz frei.
    </Note>
  </Step>
</Steps>

<h2 id="where-servers-are-saved">
  Wo Server gespeichert werden
</h2>

Der Befehl `claude mcp add` schreibt die Details des Servers in eine Konfigurationsdatei. Standardmäßig registriert er den Server im `local`-Bereich: privat für Sie, aktiv nur im aktuellen Projekt. Übergeben Sie `--scope user`, um ihn einmal für alle Ihre Projekte zu registrieren, oder `--scope project`, um ihn mit Ihrem Team zu teilen. [Server-Bereich ändern](#change-server-scope) führt Sie durch beide.

<Note>
  `claude mcp add` funktioniert in jeder Shell gleich, einschließlich PowerShell und Command Prompt. Verwenden Sie in einer `claude`-Sitzung den Befehl `/mcp`, um bereits hinzugefügte Server zu überprüfen und zu verwalten.
</Note>

Es gibt andere Möglichkeiten, einen Server hinzuzufügen, die später auf dieser Seite behandelt werden:

* [Fügen Sie einen lokalen Server hinzu](#add-a-local-server): Führen Sie ein Programm auf Ihrem Computer aus, anstatt sich mit einer URL zu verbinden.
* [Bearbeiten Sie `.mcp.json` direkt](#edit-mcp-json-directly): Schreiben Sie den JSON-Eintrag selbst, anstatt den Befehl zu verwenden.
* [Verbinden Sie einen Server, der eine Anmeldung erfordert](#connect-a-server-that-requires-sign-in): Fügen Sie einen gehosteten Server hinzu, der eine Browser-Anmeldung benötigt, bevor seine Tools funktionieren.

<h3 id="find-your-configuration-on-disk">
  Finden Sie Ihre Konfiguration auf der Festplatte
</h3>

Der Befehl `claude mcp add` schreibt den Server in einen von drei Bereichen, die über zwei Dateien verteilt sind, je nach dem Flag `--scope`. Sie müssen diese Dateien nicht direkt bearbeiten, aber zu wissen, wo sie sich befinden, hilft beim Debuggen und bei der Versionskontrolle.

| Bereich   | Datei                                                        | Verfügbar für                             |
| :-------- | :----------------------------------------------------------- | :---------------------------------------- |
| `local`   | `~/.claude.json`, unter dem Eintrag für dieses Projekt       | Nur Sie, nur dieses Projekt. Der Standard |
| `project` | `.mcp.json` im Projektstammverzeichnis                       | Jeder, der das Projekt klont              |
| `user`    | `~/.claude.json`, unter dem Top-Level-Schlüssel `mcpServers` | Nur Sie, alle Projekte                    |

Unter Windows wird `~/.claude.json` zu `%USERPROFILE%\.claude.json`, typischerweise `C:\Users\YourName\.claude.json`. Wenn Sie [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) gesetzt haben, liest Claude Code `.claude.json` stattdessen aus diesem Verzeichnis.

Führen Sie `claude mcp get claude-code-docs` aus, um zu sehen, welcher Bereich die Definition eines Servers enthält. Wie die Bereiche interagieren, wenn derselbe Server in mehr als einem definiert ist, siehe [MCP-Installationsbereiche](/docs/de/mcp#mcp-installation-scopes).

<h2 id="change-server-scope">
  Server-Bereich ändern
</h2>

Der Bereich eines Servers ist beim Hinzufügen festgelegt, daher bedeutet das Ändern des Bereichs, den Eintrag zu entfernen und ihn im neuen Bereich erneut hinzuzufügen. Beide Fälle unten beginnen damit, den lokalen Eintrag aus der ersten Anleitung zu entfernen, damit der Server nur eine Definition hat. Wenn Sie ihn bereits am Ende dieser Anleitung entfernt haben, überspringen Sie diesen Befehl:

```bash theme={null}
claude mcp remove claude-code-docs --scope local
```

<h3 id="use-a-server-in-all-your-projects">
  Verwenden Sie einen Server in allen Ihren Projekten
</h3>

Fügen Sie den Server im `user`-Bereich erneut hinzu, um ihn in jedem Projekt aktiv zu machen, das Sie öffnen, immer noch privat für Sie:

```bash theme={null}
claude mcp add --scope user --transport http claude-code-docs https://code.claude.com/docs/mcp
```

<h3 id="share-a-server-with-your-team">
  Teilen Sie einen Server mit Ihrem Team
</h3>

Fügen Sie den Server im `project`-Bereich erneut hinzu, der in `.mcp.json` im Projektstammverzeichnis schreibt:

```bash theme={null}
claude mcp add --scope project --transport http claude-code-docs https://code.claude.com/docs/mcp
```

Committen Sie `.mcp.json` zur Versionskontrolle. Teamkollegen, die das Repository klonen und Claude Code starten, sehen eine Aufforderung zur Genehmigung des Servers, dann verbindet er sich auch für sie.

<h2 id="additional-mcp-server-examples">
  Zusätzliche MCP-Server-Beispiele
</h2>

Die erste Anleitung verwendete einen gehosteten Server, der sich ohne Anmeldung verbindet. Die folgenden Beispiele behandeln die anderen zwei häufigen Formen mit dem gleichen Hinzufügen-, Überprüfen-, Verwenden-Ablauf.

<h3 id="add-a-local-server">
  Fügen Sie einen lokalen Server hinzu
</h3>

Ein lokaler stdio-Server ist ein Programm, das Claude Code als Unterprozess auf Ihrem Computer startet, anstatt es über eine URL zu erreichen. Verwenden Sie einen für Tools, die Zugriff auf lokale Ressourcen wie einen Browser, Ihr Dateisystem oder einen Datenbanksocket benötigen.

Der [Playwright MCP-Server](https://github.com/microsoft/playwright-mcp) ist ein guter zum Ausprobieren: Er gibt Claude einen Browser, den er navigieren, klicken und lesen kann, und benötigt kein Konto. Er läuft über `npx`, daher benötigt er [Node.js](https://nodejs.org/en/download) 18 oder später.

<Steps>
  <Step title="Fügen Sie den Playwright-Server hinzu">
    Registrieren Sie den Server mit dem Befehl, den Claude Code ausführen soll, um ihn zu starten:

    ```bash theme={null}
    claude mcp add playwright -- npx -y @playwright/mcp@latest
    ```

    Dieser Befehl unterscheidet sich vom gehosteten Beispiel auf drei Arten:

    * Es gibt kein `--transport`-Flag, da lokale Server das Standard-`stdio`-Transport verwenden.
    * Alles nach dem `--`-Trennzeichen ist der Befehl, den Claude Code ausführt, um den Server zu starten.
    * `-y` teilt `npx` mit, das Paket ohne Aufforderung zu installieren.

    Playwright steuert, welcher Chrome bereits auf Ihrem Computer installiert ist. Um einen anderen Browser zu verwenden, fügen Sie `--browser` mit dem Browser-Namen an, z. B. `--browser firefox`, nach `@playwright/mcp@latest`.
  </Step>

  <Step title="Überprüfen Sie die Verbindung">
    Die `Added`-Bestätigung bedeutet, dass der Eintrag gespeichert wurde, nicht dass der Befehl ausgeführt wird. Überprüfen Sie die Verbindung:

    ```bash theme={null}
    claude mcp list
    ```

    Die erste Überprüfung kann `✗ Failed to connect` anzeigen, während `npx` das Paket herunterlädt, daher warten Sie einen Moment und führen Sie es erneut aus.
  </Step>

  <Step title="Verwenden Sie den Browser">
    Geben Sie Claude eine Aufgabe, die den Browser benötigt:

    ```text theme={null}
    Use playwright to open https://example.com and tell me the page title
    ```

    Ein Browserfenster öffnet sich, damit Sie sehen können, wie es funktioniert, und die Tool-Aufrufe in Claudes Ausgabe sind mit dem `playwright`-Servernamen und der Aktion gekennzeichnet, wie z. B. `browser_navigate`.

    Versuchen Sie, es auf Ihren lokalen Dev-Server zu verweisen, um zu überprüfen, dass eine Seite nach einer Änderung immer noch gerendert wird, oder lassen Sie es einen Fehlerbericht Schritt für Schritt durchgehen.
  </Step>
</Steps>

<h3 id="connect-a-server-that-requires-sign-in">
  Verbinden Sie einen Server, der eine Anmeldung erfordert
</h3>

Gehostete Dienste wie Sentry, Linear und Notion führen ihre MCP-Server hinter OAuth aus: Sie fügen die URL des Servers hinzu, dann melden Sie sich über Ihren Browser an.

Die folgenden Schritte verwenden Sentry als Beispiel. Um einen anderen Dienst zu verbinden, ersetzen Sie seine URL, die Sie im [Anthropic-Verzeichnis](/docs/de/mcp#find-and-build-mcp-servers) oder in der Dokumentation des Dienstes finden können.

<Steps>
  <Step title="Fügen Sie den Server hinzu">
    Der `add`-Befehl ist derselbe wie für den Dokumentationsserver, mit Sentrys URL:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```

    Nach dem Hinzufügen zeigt `claude mcp list` den Server mit `! Needs authentication` an. Das ist erwartet: Der nächste Schritt schließt die Anmeldung ab.
  </Step>

  <Step title="Authentifizieren Sie sich in Ihrem Browser">
    Starten Sie eine Claude Code-Sitzung und öffnen Sie das MCP-Panel:

    ```text theme={null}
    /mcp
    ```

    Wählen Sie `sentry` aus der Liste, drücken Sie Enter, und wählen Sie `Authenticate`. Ihr Browser öffnet sich auf Sentrys Anmeldeseite. Genehmigen Sie die Verbindung dort.

    Zurück in Claude Code ändert sich der Status des Servers zu verbunden. Wenn die Anmeldung fehlschlägt oder der Browser nicht öffnet, siehe [Fehlerbehebung](#troubleshooting).
  </Step>

  <Step title="Verwenden Sie den Server">
    Fragen Sie Claude etwas, das den Dienst benötigt, wie z. B. `What Sentry projects do I have access to?`, und suchen Sie nach Tool-Aufrufen, die mit dem `sentry`-Servernamen in seiner Ausgabe gekennzeichnet sind.
  </Step>
</Steps>

Server, die sich mit einem statischen Token anstelle von OAuth authentifizieren, nehmen das Token zum Hinzufügungszeitpunkt mit `--header "Authorization: Bearer <token>"` an. Siehe das [GitHub-Beispiel](/docs/de/mcp#example-connect-to-github-for-code-reviews) für eine durchgearbeitete Version.

<h2 id="edit-mcp-json-directly">
  Bearbeiten Sie .mcp.json direkt
</h2>

Jede Datei in der [Bereichstabelle](#find-your-configuration-on-disk) verwendet das gleiche JSON-Format für Server-Einträge. Dieser Abschnitt bearbeitet `.mcp.json`, die projektbezogene Datei. Es ist die, die sich am meisten lohnt, von Hand zu schreiben, da sie im Repository eingecheckt wird, wo sie auch als Konfiguration-als-Code für Ihr Team dient.

Erstellen Sie `.mcp.json` im Projektstammverzeichnis. Das folgende Beispiel definiert beide Server aus dieser Anleitung, den gehosteten Dokumentationsserver, der über HTTP erreichbar ist, und den Playwright-Server als lokalen `stdio`-Prozess:

```json theme={null}
{
  "mcpServers": {
    "claude-code-docs": {
      "type": "http",
      "url": "https://code.claude.com/docs/mcp"
    },
    "playwright": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@playwright/mcp@latest"]
    }
  }
}
```

Die Felder unterscheiden sich je nach Servertyp:

* Für HTTP-Server ist `url` der Endpunkt, mit dem sich Claude Code verbindet.
* Für stdio-Server sind `command` und `args` das Programm, das es ausführt.

Nach dem Speichern der Datei starten Sie eine neue Claude Code-Sitzung im Projekt. Claude Code liest `.mcp.json` beim Start.

Wenn Claude Code zum ersten Mal einen projektbezogenen Server sieht, fragt es Sie, ihn zu genehmigen. Die Aufforderung existiert, damit ein Repository, das Sie klonen, keine Prozesse auf Ihrem Computer ohne Ihre Zustimmung starten kann. Genehmigen Sie die Aufforderung, oder führen Sie `/mcp` aus, um später zu genehmigen, wenn Sie sie verpasst haben.

Nachdem Sie genehmigt haben, führen Sie `/mcp` aus und überprüfen Sie, dass die Server als verbunden angezeigt werden. Wenn einer stattdessen einen Fehler anzeigt, siehe [Fehlerbehebung](#troubleshooting).

<h2 id="connect-from-other-surfaces">
  Von anderen Oberflächen verbinden
</h2>

Diese Anleitung verwendet die `claude mcp` CLI-Befehle, aber jede Claude Code-Oberfläche kann sich mit MCP-Servern verbinden:

* **Claude Code Desktop-App**: Fügen Sie Server über die [Connectors-Benutzeroberfläche](/docs/de/desktop#connect-external-tools) hinzu.
* **Claude Desktop Chat-App**: eine separate App von Claude Code. Um Server aus ihrer `claude_desktop_config.json` in die CLI zu kopieren, führen Sie `claude mcp add-from-claude-desktop` auf macOS oder WSL aus.
* **VS Code**: siehe [Mit externen Tools mit MCP verbinden](/docs/de/vs-code#connect-to-external-tools-with-mcp).
* **Claude Code im Web**: liest `.mcp.json` aus Ihrem Repository. Siehe [Bearbeiten Sie .mcp.json direkt](#edit-mcp-json-directly).
* **Claude.ai**: Connectors, die Sie unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors) hinzufügen, werden automatisch in die CLI geladen, wenn Sie sich mit diesem Konto anmelden. Siehe [Verwenden Sie MCP-Server von Claude.ai](/docs/de/mcp#use-mcp-servers-from-claude-ai).

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Wenn sich ein Server nicht verbindet, überprüfen Sie seinen Status mit `/mcp` in einer Sitzung oder `claude mcp list` aus Ihrer Shell, dann gleichen Sie das Symptom unten ab. Das `/mcp`-Panel ermöglicht es Ihnen auch, sich erneut zu verbinden oder zu authentifizieren, ohne die Sitzung zu verlassen.

<AccordionGroup>
  <Accordion title="/mcp shows No MCP servers configured">
    Claude Code hat keine Server für das aktuelle Verzeichnis gefunden. Die häufigsten Ursachen:

    * Sie haben `claude mcp add` aus einem anderen Projekt ausgeführt. Lokal begrenzte Server sind an das Projekt gebunden, in dem Sie sie hinzugefügt haben: das Repository-Stammverzeichnis oder das genaue Verzeichnis, wenn Sie sich nicht in einem Git-Repository befanden. Fügen Sie den Server aus dem Projekt erneut hinzu, in dem Sie sich jetzt befinden, oder fügen Sie ihn mit `--scope user` hinzu, damit er nicht an ein Projekt gebunden ist.
    * Sie haben eine Konfigurationsdatei unter dem falschen Pfad bearbeitet. Die richtigen Dateien sind `~/.claude.json` und `<project>/.mcp.json`. Claude Code liest keine Pfade wie `~/.claude/.mcp.json`, `~/.claude/config/mcp.json`, `~/.claude/mcp.json` oder `%APPDATA%\Claude\mcp.json`. Für benutzerbereichsbegrenzte Server führen Sie `claude mcp add --scope user` aus, das in den `mcpServers`-Schlüssel in `~/.claude.json` schreibt; für projektbereichsbegrenzte Server bearbeiten Sie `.mcp.json` im Projektstammverzeichnis.
  </Accordion>

  <Accordion title="Status shows Failed to connect or Connection error">
    Beide Status bedeuten, dass der Server nicht gestartet wurde oder die URL nicht reagiert hat. Sie können auch für HTTP-Server angezeigt werden, die ein Token erwarten, anstatt der Browser-Anmeldung, die in [Verbinden Sie einen Server, der eine Anmeldung erfordert](#connect-a-server-that-requires-sign-in) behandelt wird.

    Ab v2.1.191 zeigt ein HTTP-Server, der `404 Not Found` zurückgibt, `MCP endpoint not found at <url>. Check the URL in your MCP config.` an, wenn Sie den Server in `/mcp` auswählen, mit der URL, die Claude Code versucht hat. Frühere Versionen zeigen eine generische `Error POSTing to endpoint`-Nachricht ohne die URL an. Vergleichen Sie die URL mit dem dokumentierten MCP-Endpunkt-Pfad des Servers, führen Sie dann `claude mcp remove <name>` aus und fügen Sie ihn mit der korrekten URL erneut hinzu.

    Für HTTP-Server bestätigen Sie, dass die URL von Ihrem Computer aus erreichbar ist:

    ```bash theme={null}
    curl -I https://mcp.sentry.dev/mcp
    ```

    In PowerShell verwenden Sie `curl.exe` anstelle von `curl`, damit die Anfrage zum echten curl-Binary geht, anstatt zum `Invoke-WebRequest`-Alias.

    Die Antwort sagt Ihnen, welche Art von Problem Sie haben:

    * Ein `404` oder `405`: Der Server ist aktiv. Viele MCP-Endpunkte antworten nur auf POST-Anfragen, daher bestätigt dies immer noch, dass die URL von Ihrem Computer aus erreichbar ist.
    * Ein `401` oder `403`: Der Server ist aktiv und Sie müssen sich authentifizieren. Verwenden Sie die Browser-Anmeldung in [Verbinden Sie einen Server, der eine Anmeldung erfordert](#connect-a-server-that-requires-sign-in), oder für Server, die stattdessen ein Token annehmen, wie GitHubs, übergeben Sie es mit `--header "Authorization: Bearer <token>"` im `claude mcp add`-Befehl.
    * Keine Antwort: Überprüfen Sie die URL und Ihr Netzwerk.

    Für stdio-Server führen Sie den konfigurierten Befehl direkt in Ihrem Terminal aus, um den zugrunde liegenden Fehler zu sehen. Für den Playwright-Server aus dieser Anleitung führen Sie aus:

    ```bash theme={null}
    npx -y @playwright/mcp@latest
    ```

    Was danach passiert, sagt Ihnen, wo das Problem liegt:

    * Der Befehl startet und wartet auf Eingabe: Der Server selbst funktioniert. Führen Sie `claude mcp get <name>` aus und bestätigen Sie, dass der dort angezeigte Befehl dem entspricht, den Sie gerade ausgeführt haben. Wenn sich der angezeigte Befehl von dem unterscheidet, den Sie eingegeben haben, haben Sie wahrscheinlich das `--`-Trennzeichen vor dem Server-Befehl weggelassen. Entfernen Sie den Server und fügen Sie ihn mit `--` an der richtigen Stelle erneut hinzu. Wenn Sie `.mcp.json` von Hand geschrieben haben, überprüfen Sie seine Syntax und seinen Speicherort.
    * Der Befehl gibt einen Fehler aus: Die Nachricht nennt, was fehlt, wie z. B. Node.js oder ein Browser.
  </Accordion>

  <Accordion title="Connection timed out at startup">
    Der Server hat länger als das Standard-30-Sekunden-Startup-Timeout gedauert. Der erste Lauf eines stdio-Servers kann langsam sein, während `npx` das Paket herunterlädt. Erhöhen Sie das Limit mit der Umgebungsvariablen [`MCP_TIMEOUT`](/docs/de/env-vars), in Millisekunden:

    ```bash theme={null}
    MCP_TIMEOUT=60000 claude
    ```

    In PowerShell setzen Sie die Variable vor dem Befehl auf der gleichen Zeile:

    ```powershell theme={null}
    $env:MCP_TIMEOUT = "60000"; claude
    ```
  </Accordion>

  <Accordion title="Server already exists">
    Sie haben bereits einen Server mit diesem Namen im gleichen Bereich hinzugefügt. Entfernen Sie entweder den vorhandenen Eintrag zuerst oder wählen Sie einen anderen Namen:

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    Wenn der Name in mehr als einem Bereich existiert, meldet `remove` `exists in multiple scopes`. Übergeben Sie `--scope`, um zu wählen, welche Kopie gelöscht werden soll, z. B. `claude mcp remove claude-code-docs --scope local`.
  </Accordion>

  <Accordion title="Server connects but no tools appear">
    Führen Sie `/mcp` in einer Sitzung aus und wählen Sie den Server, um seine Tool-Liste zu sehen. Wenn die Liste leer ist, ist der Server gestartet, hat aber keine Tools registriert, was normalerweise bedeutet, dass ihm eine erforderliche Umgebungsvariable wie ein API-Schlüssel fehlt.

    Übergeben Sie die Variable mit `--env KEY=value` im `claude mcp add`-Befehl, oder im `env`-Feld des Server-Eintrags in `.mcp.json`. Die Dokumentation des Servers listet die Variablen auf, die er benötigt.
  </Accordion>

  <Accordion title="Changes to .mcp.json don't take effect">
    Claude Code liest `.mcp.json` beim Sitzungsstart. Beenden Sie die Sitzung und starten Sie sie nach dem Bearbeiten der Datei neu.

    Wenn Ihre Server immer noch nicht angezeigt werden, führen Sie `/mcp` aus und suchen Sie nach einer Parse-Warnung. Claude Code überspringt fehlerhafte Einträge und zeigt das betroffene Feld dort an.

    Wenn Sie den Server zuvor abgelehnt haben, wenn Sie dazu aufgefordert wurden, setzen Sie Projektgenehmigungen zurück:

    ```bash theme={null}
    claude mcp reset-project-choices
    ```
  </Accordion>

  <Accordion title="OAuth sign-in fails or browser doesn't open">
    Führen Sie `/mcp` aus, wählen Sie den Server, und wählen Sie `Authenticate` erneut. Wenn der Browser nicht automatisch öffnet, kopieren Sie die im Terminal angezeigte URL und öffnen Sie sie manuell. Siehe [Authentifizieren Sie sich mit Remote-MCP-Servern](/docs/de/mcp#authenticate-with-remote-mcp-servers) für feste Callback-Ports und vorkonfigurierte Anmeldedaten.
  </Accordion>
</AccordionGroup>

<h2 id="next-steps">
  Nächste Schritte
</h2>

Mit einem verbundenen Server erkunden Sie den Rest dessen, was MCP ermöglicht:

* [Finden Sie weitere MCP-Server](/docs/de/mcp#find-and-build-mcp-servers) im Anthropic-Verzeichnis
* [Teilen Sie Server mit Ihrem Team](/docs/de/mcp#mcp-installation-scopes) mit Installationsbereichen
* [Verwalten Sie MCP-Zugriff für eine Organisation](/docs/de/managed-mcp) mit verwalteten Einstellungen und Richtlinienkontrollen
* [Referenzieren Sie MCP-Ressourcen](/docs/de/mcp#use-mcp-resources) in Eingabeaufforderungen mit @-Erwähnungen
* [Führen Sie MCP-Eingabeaufforderungen als Befehle aus](/docs/de/mcp#use-mcp-prompts-as-commands) aus dem `/`-Menü
* [Erstellen Sie Ihren eigenen Server](https://modelcontextprotocol.io/quickstart/server) mit dem MCP SDK
