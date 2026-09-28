> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code mit Tools über MCP verbinden

> Erfahren Sie, wie Sie Claude Code mit Ihren Tools über das Model Context Protocol verbinden.

Claude Code kann sich über das [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction), einen offenen Standard für KI-Tool-Integrationen, mit Hunderten von externen Tools und Datenquellen verbinden. MCP-Server geben Claude Code Zugriff auf Ihre Tools, Datenbanken und APIs.

Verbinden Sie einen Server, wenn Sie feststellen, dass Sie Daten aus einem anderen Tool wie einem Issue-Tracker oder einem Überwachungs-Dashboard in den Chat kopieren. Nach der Verbindung kann Claude direkt auf dieses System zugreifen und handeln, anstatt mit dem zu arbeiten, was Sie einfügen.

Wenn Sie Ihren ersten Server verbinden, beginnen Sie mit der [MCP-Schnellstartanleitung](/docs/de/mcp-quickstart) für eine Schritt-für-Schritt-Anleitung. Diese Seite ist die vollständige Referenz.

<h2 id="what-you-can-do-with-mcp">
  Was Sie mit MCP tun können
</h2>

Mit verbundenen MCP-Servern können Sie Claude Code auffordern:

* **Funktionen aus Issue-Trackern implementieren**: „Füge die in JIRA-Issue ENG-4521 beschriebene Funktion hinzu und erstelle einen PR auf GitHub."
* **Überwachungsdaten analysieren**: „Überprüfe Sentry und Statsig, um die Nutzung der in ENG-4521 beschriebenen Funktion zu überprüfen."
* **Datenbanken abfragen**: „Finde E-Mail-Adressen von 10 zufälligen Benutzern, die die Funktion ENG-4521 verwendet haben, basierend auf unserer PostgreSQL-Datenbank."
* **Designs integrieren**: „Aktualisiere unsere Standard-E-Mail-Vorlage basierend auf den neuen Figma-Designs, die in Slack gepostet wurden"
* **Workflows automatisieren**: „Erstelle Gmail-Entwürfe, die diese 10 Benutzer zu einer Feedback-Sitzung zur neuen Funktion einladen."
* **Auf externe Ereignisse reagieren**: Ein MCP-Server kann auch als [Kanal](/docs/de/channels) fungieren, der Nachrichten in Ihre Sitzung pusht, sodass Claude auf Telegram-Nachrichten, Discord-Chats oder Webhook-Ereignisse reagiert, während Sie weg sind.

<h2 id="find-and-build-mcp-servers">
  MCP-Server finden und erstellen
</h2>

Durchsuchen Sie überprüfte Konnektoren im [Anthropic Directory](https://claude.ai/directory). Directory-Konnektoren verwenden die gleiche MCP-Infrastruktur wie Claude Code, sodass Sie jeden dort aufgelisteten Remote-Server mit `claude mcp add` hinzufügen können.

<Warning>
  Überprüfen Sie, dass Sie jedem Server vertrauen, bevor Sie ihn verbinden. Server, die externe Inhalte abrufen, können Sie dem [Risiko von Prompt-Injection aussetzen](/docs/de/security#protect-against-prompt-injection).
</Warning>

Um Ihren eigenen Server zu erstellen, lesen Sie das [MCP-Server-Handbuch](https://modelcontextprotocol.io/docs/develop/build-server) für Protokoll-Grundlagen und die [Claude-Konnektoren-Dokumentation zum Erstellen](https://claude.com/docs/connectors/building) für Authentifizierung, Tests und Directory-Einreichung.

Sie können Claude auch einen Server für Sie mit dem offiziellen [`mcp-server-dev` Plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev) erstellen lassen.

<Steps>
  <Step title="Installieren Sie das Plugin">
    Führen Sie in einer Claude Code-Sitzung Folgendes aus:

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    Wenn die Installation fehlschlägt, beachten Sie die Meldung, die Claude Code meldet:

    * `Marketplace "claude-plugins-official" nicht gefunden`: Fügen Sie den Marketplace mit `/plugin marketplace add anthropics/claude-plugins-official` hinzu und versuchen Sie dann die Installation erneut.
    * Das Plugin ist [nicht im Marketplace gefunden](/docs/de/plugins/install#install-a-plugin): Überprüfen Sie den Plugin-Namen.

    Wenn die Installationszusammenfassung `Run /reload-plugins to activate.` meldet, führt Claude Code diesen Befehl dann für Sie aus. Wenn das Neuladen warnt, dass Ihre nächste Nachricht das Gespräch erneut lesen würde, führen Sie `/reload-plugins --force` aus.
  </Step>

  <Step title="Führen Sie die Build-Skill aus">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude fragt nach Ihrem Anwendungsfall und erstellt einen Remote-HTTP- oder lokalen Stdio-Server.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  MCP-Server installieren
</h2>

MCP-Server können je nach Ihren Anforderungen auf mehrere Arten konfiguriert werden:

<h3 id="option-1-add-a-remote-http-server">
  Option 1: Einen Remote-HTTP-Server hinzufügen
</h3>

HTTP-Server sind die empfohlene Option für die Verbindung mit Remote-MCP-Servern. Dies ist das am weitesten unterstützte Transportprotokoll für Cloud-basierte Dienste.

```bash theme={null}
# Grundlegende Syntax
claude mcp add --transport http <name> <url>

# Echtes Beispiel: Mit Notion verbinden
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Beispiel mit Bearer-Token
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Bei der Konfiguration von MCP-Servern über JSON in `.mcp.json`, `~/.claude.json` oder `claude mcp add-json` akzeptiert das Feld `type` `streamable-http` als Alias für `http`. Die MCP-Spezifikation verwendet den Namen `streamable-http` für dieses Transportprotokoll, sodass Konfigurationen, die aus der Server-Dokumentation kopiert werden, ohne Änderungen funktionieren.

Ein JSON-Eintrag, der eine `url` hat, aber keinen `type`, ist ein Konfigurationsfehler, da Claude Code einen Eintrag ohne `type` als Stdio-Server liest. Claude Code überspringt diesen Server und meldet `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Vor v2.1.202 meldete Claude Code diese Fehlkonfiguration als `command: expected string, received undefined`.

Bei `--output-format stream-json`-Läufen meldet Claude Code auch einen übersprungenen `--mcp-config`-Eintrag im [`mcp_server_errors`-Feld](/docs/de/headless#stream-responses) des `system/init`-Ereignisses, sodass Skripte erkennen können, dass der Server nie geladen wurde. Dies erfordert Claude Code v2.1.219 oder später.

<h3 id="option-2-add-a-remote-sse-server">
  Option 2: Einen Remote-SSE-Server hinzufügen
</h3>

<Warning>
  Das SSE-Transportprotokoll (Server-Sent Events) ist veraltet. Verwenden Sie stattdessen HTTP-Server, wo verfügbar.
</Warning>

Einige Dienste stellen nur einen SSE-Endpunkt bereit. Fügen Sie diese mit dem gleichen Befehl `claude mcp add --transport http <name> <url>` wie [ein HTTP-Server](#option-1-add-a-remote-http-server) hinzu. Claude Code versucht zuerst das HTTP-Transportprotokoll und wechselt zu SSE, wenn der Server es nicht akzeptiert. Der automatische Wechsel erfordert Claude Code v2.1.265 oder später.

Bei einer früheren Version oder um sich direkt über SSE zu verbinden, übergeben Sie stattdessen `--transport sse`:

```bash theme={null}
# Grundlegende Syntax
claude mcp add --transport sse <name> <url>

# Echtes Beispiel: Mit Asana verbinden
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Beispiel mit Authentifizierungs-Header
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Option 3: Einen lokalen Stdio-Server hinzufügen
</h3>

Stdio-Server werden als lokale Prozesse auf Ihrem Computer ausgeführt. Sie sind ideal für Tools, die direkten Systemzugriff oder benutzerdefinierte Skripte benötigen.

Claude Code setzt `CLAUDE_PROJECT_DIR` in der Umgebung des erzeugten Servers auf das Projektstammverzeichnis, sodass Ihr Server projektrelative Pfade auflösen kann, ohne vom Arbeitsverzeichnis abhängig zu sein. Dies ist das gleiche Verzeichnis, das Hooks in ihrer `CLAUDE_PROJECT_DIR`-Variable erhalten. Lesen Sie es aus Ihrem Serverprozess, zum Beispiel `process.env.CLAUDE_PROJECT_DIR` in Node oder `os.environ["CLAUDE_PROJECT_DIR"]` in Python.

`CLAUDE_PROJECT_DIR` ist das stabile Projektstammverzeichnis und ändert sich nicht, wenn Sie während einer Sitzung Arbeitsverzeichnisse hinzufügen oder entfernen. Ein Server, der seinen eigenen Dateisystemzugriff auf einen Satz zulässiger Verzeichnisse beschränkt, sollte stattdessen die MCP-Anfrage `roots/list` implementieren. Claude Code antwortet auf `roots/list` mit dem Startverzeichnis der Sitzung plus jedem [zusätzlichen Arbeitsverzeichnis](/docs/de/permissions#working-directories), das Sie mit `--add-dir`, `/add-dir` oder der Einstellung `additionalDirectories` gewährt haben. Claude Code sendet `notifications/roots/list_changed`, wenn sich dieser Satz ändert. Vor v2.1.203 gab `roots/list` nur das Startverzeichnis zurück und Claude Code sendete `notifications/roots/list_changed` nicht.

Diese Variable wird in der Umgebung des Servers gesetzt, nicht in der Umgebung von Claude Code selbst, daher erfordert das Referenzieren über `${VAR}`-Erweiterung in `command` oder `args` eines projektgesteuerten `.mcp.json`-Eintrags oder eines lokal- oder benutzergesteuerten Server-Eintrags in `~/.claude.json` einen Standard wie `${CLAUDE_PROJECT_DIR:-.}`. Von Plugins bereitgestellte MCP-Konfigurationen ersetzen `${CLAUDE_PROJECT_DIR}` direkt und benötigen keinen Standard.

```bash theme={null}
# Grundlegende Syntax
claude mcp add [options] <name> -- <command> [args...]

# Echtes Beispiel: Airtable-Server hinzufügen
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Wichtig: Trennen Sie Server-Argumente mit `--`**

  Bei Stdio-Servern trennt der `--` (Doppelstrich) Claudes eigene Optionen, wie `--transport`, `--env` und `--scope`, vom Befehl und den Argumenten, die den Server ausführen. Alles nach `--` wird unverändert an den Server übergeben.

  Zum Beispiel:

  * `claude mcp add --transport stdio myserver -- npx server` → führt `npx server` aus
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → führt `python server.py --port 8080` mit `KEY=value` in der Umgebung aus

  Ohne `--` würde Claude Code versuchen, die Flags des Servers, wie `--port` oben, als seine eigenen Optionen zu analysieren.

  `--env` akzeptiert mehrere `KEY=value`-Paare. Wenn der Servername direkt nach `--env` kommt, liest die CLI den Namen als ein weiteres Paar und lehnt ihn ab, daher platzieren Sie mindestens eine andere Option, wie `--transport stdio`, zwischen `--env` und dem Servernamen.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Option 4: Einen Remote-WebSocket-Server hinzufügen
</h3>

WebSocket-Server halten eine persistente bidirektionale Verbindung, die sich für Remote-MCP-Server eignet, die Claude unaufgefordert Ereignisse pushen. Verwenden Sie HTTP stattdessen, wenn Ihr Server nur auf Anfragen antwortet, da HTTP OAuth und das Flag `claude mcp add --transport` unterstützt, während WebSocket beides nicht unterstützt.

Konfigurieren Sie WebSocket-Server in `.mcp.json` oder mit `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

Der Eintrag `type: "ws"` akzeptiert die gleichen Felder `url`, `headers`, `headersHelper`, `timeout` und `alwaysLoad` wie `http`. Die Authentifizierung erfolgt nur über Header, daher übergeben Sie ein statisches Token in `headers` oder generieren Sie eines zur Verbindungszeit mit [`headersHelper`](#use-dynamic-headers-for-custom-authentication). Das Flag `claude mcp add --transport` akzeptiert nicht `ws`.

<h3 id="add-a-server-from-setup-instructions-written-for-another-client">
  Einen Server aus Setupanweisungen hinzufügen, die für einen anderen Client geschrieben wurden
</h3>

MCP-Server sind nicht spezifisch für Claude Code, daher können die Setupanweisungen eines Servers für Claude Desktop, Cursor oder einen anderen MCP-Client geschrieben sein und keinen `claude mcp add`-Befehl geben. Um den Server trotzdem hinzuzufügen, suchen Sie in diesen Anweisungen nach einer URL, einem Startbefehl oder einem JSON-Block:

* **Eine URL** wie `https://mcp.example.com/mcp`: Der Server ist Remote.
* **Ein Startbefehl** wie `npx -y @example/mcp-server`: Der Server wird auf Ihrem Computer ausgeführt.
* **Ein `mcpServers`-JSON-Block**: Konfiguration, die für die Einstellungsdatei eines anderen Clients geschrieben wurde.

Jedes ist einer der Eingaben, die die vier Optionen in [MCP-Server installieren](#installing-mcp-servers) benötigen. Finden Sie die Form, die Sie haben, unten, um sie in den Befehl umzuwandeln, den Claude Code akzeptiert. Jeder Befehl schreibt in [lokalen Bereich](#local-scope), es sei denn, Sie fügen `--scope project` oder `--scope user` hinzu.

<h4 id="from-a-url">
  Aus einer URL
</h4>

Eine URL bedeutet, dass der Server Remote ist. Für einen `https://`-Endpunkt fügen Sie ihn mit `--transport http` hinzu, oder folgen Sie [Option 2](#option-2-add-a-remote-sse-server), wenn die Anweisungen sagen, dass der Endpunkt SSE verwendet. Für einen `wss://`-Endpunkt verwenden Sie stattdessen [Option 4](#option-4-add-a-remote-websocket-server), da `--transport` nicht `ws` akzeptiert:

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

Wenn die Anweisungen auch einen API-Schlüssel oder Token-Header geben, übergeben Sie ihn mit `--header`, wie in [Option 1](#option-1-add-a-remote-http-server) gezeigt.

<h4 id="from-an-npx-uvx-or-binary-command">
  Aus einem `npx`-, `uvx`- oder Binary-Befehl
</h4>

Ein Startbefehl bedeutet, dass der Server als lokaler Stdio-Prozess ausgeführt wird. Setzen Sie den gesamten Befehl nach `--`, sodass Claude Code Flags wie `-y` an den Befehl übergibt, der den Server startet, anstatt sie als seine eigenen Optionen zu lesen. Übergeben Sie alle Umgebungsvariablen, die die Anweisungen anfordern, mit `--env`, nach dem Servernamen und vor `--`:

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

[Option 3](#option-3-add-a-local-stdio-server) behandelt den `--`-Separator vollständig.

<h4 id="from-an-mcpservers-json-block">
  Aus einem `mcpServers`-JSON-Block
</h4>

Ein `mcpServers`-Block, der für einen anderen MCP-Client wie Claude Desktop geschrieben wurde, verwendet den Wrapper-Schlüssel und die Eintrag-Form, die Claude Code liest. Übergeben Sie `claude mcp add-json` das Objekt innerhalb von `mcpServers`, nicht den Wrapper. Zwei Einträge benötigen zuerst eine Reparatur:

* **Eine `url` ohne `type`**: Fügen Sie `"type": "http"`, `"type": "sse"` oder `"type": "ws"` hinzu, um dem Endpunkt zu entsprechen. Claude Code liest einen Eintrag ohne `type` als Stdio-Server, daher schlägt ein `url`-Eintrag ohne `type` fehl.
* **Ein Schlüssel mit Zeichen außer Buchstaben, Zahlen, Bindestrichen und Unterstrichen**: Wählen Sie einen Servernamen, der nur diese Zeichen verwendet. Andernfalls ist der Schlüssel der Servername.

Zum Beispiel wird dieser Block:

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

zu diesem Befehl:

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[MCP-Server aus JSON-Konfiguration hinzufügen](#add-mcp-servers-from-json-configuration) behandelt Shell-Escaping und das `--scope`-Flag für `add-json`. Um den Server stattdessen mit Ihrem Team zu teilen, fügen Sie `--scope project` hinzu, oder fügen Sie den Eintrag unter `mcpServers` in `.mcp.json` in Ihrem Projektstammverzeichnis hinzu und committen Sie ihn. [Projektbereich](#project-scope) behandelt, wie Claude Code diese Datei lädt und genehmigt.

Jeder `claude mcp add`- und `claude mcp add-json`-Befehl gibt eine `Added ...`-Zeile aus. Um zu überprüfen, dass Claude Code verbunden ist, führen Sie `claude mcp get <name>` aus; [Serverstatus](#server-status) behandelt die Statuse, die er anzeigt, und den Genehmigungsschritt für `.mcp.json`-Server.

<h3 id="managing-your-servers">
  Verwalten Ihrer Server
</h3>

Nach der Konfiguration können Sie Ihre MCP-Server mit diesen Befehlen verwalten:

```bash theme={null}
# Alle konfigurierten Server auflisten
claude mcp list

# Details für einen bestimmten Server abrufen
claude mcp get notion

# Einen Server entfernen
claude mcp remove notion

# (innerhalb von Claude Code) Serverstatus überprüfen
/mcp
```

Wenn Sie einen Remote-Server entfernen, löscht Claude Code auch die OAuth-Tokens und die Client-Registrierung, die er für diesen Server gespeichert hat.

<h4 id="server-status">
  Serverstatus
</h4>

`claude mcp add` bestätigt ein erfolgreiches Hinzufügen durch Ausgabe einer `Added ...`-Zeile, was bedeutet, dass die Konfiguration geschrieben wurde. `claude mcp list` zeigt dann einen Gesundheitsstatus neben jedem Server an, den es auflistet, wie `✔ Connected`, `! Needs authentication` oder `✘ Failed to connect`. Ein Fehlerstatus bedeutet, dass Claude Code sich nicht mit diesem Server verbinden konnte, nicht dass der List-Befehl fehlgeschlagen ist.

Die Statuse in dieser Liste melden eine Konfigurationsentscheidung statt eines Verbindungsversuchs, daher gibt Claude Code sie aus, ohne sich mit dem Server zu verbinden:

* ``⏸ Pending approval (run `claude` to approve)``: Ein projektgesteuerten Server aus `.mcp.json`, den Sie noch nicht genehmigt haben. Claude Code zeigt ihn sowohl in `claude mcp list` als auch in `claude mcp get <name>`. Führen Sie `claude` interaktiv aus, um ihn zu überprüfen und zu genehmigen.
* `✘ Rejected (see disabledMcpjsonServers in settings)`: Ein `.mcp.json`-Server, den ein [`disabledMcpjsonServers`](/docs/de/settings-reference#disabledmcpjsonservers)-Eintrag ablehnt. Claude Code zeigt ihn nur in `claude mcp get <name>`.
* `⊘ Disabled for this project (re-enable via /mcp)`: Ein Server, den die [`disabledMcpServers`](#disable-a-server-without-removing-it)-Liste des Projekts benennt. Claude Code zeigt ihn sowohl in `claude mcp list` als auch in `claude mcp get <name>`. Schalten Sie den Server über das `/mcp`-Panel wieder ein. Vor v2.1.238 verbanden sich beide Befehle mit einem deaktivierten Server, um ihn zu überprüfen, und meldeten das Verbindungsergebnis.

WebSocket-Server erscheinen nicht in der `claude mcp list`-Ausgabe. Verwenden Sie `claude mcp get <name>` oder das `/mcp`-Panel, um sie zu überprüfen.

<h4 id="project-server-approvals-and-workspace-trust">
  Genehmigungen von Projektservern und Workspace-Vertrauen
</h4>

Ab v2.1.196 lesen `claude mcp list` und `claude mcp get` `.mcp.json`-Genehmigungen nur aus Einstellungsdateien, die nicht in das Repository eingecheckt werden, bis Sie dem Workspace vertrauen, indem Sie `claude` darin ausführen und den Workspace-Vertrauensdialog akzeptieren. Ein geklontes Repository kann seine eigenen Server nicht genehmigen: [`enableAllProjectMcpServers`](/docs/de/settings-reference#enableallprojectmcpservers) oder [`enabledMcpjsonServers`](/docs/de/settings-reference#enabledmcpjsonservers), die in die `.claude/settings.json` des Projekts eingecheckt werden, werden in einem nicht vertrauenswürdigen Ordner ignoriert, und der Server bleibt bei `⏸ Pending approval` statt verbunden und Health-Check zu sein.

Genehmigungen aus diesen Quellen gelten weiterhin in einem nicht vertrauenswürdigen Ordner:

* Ihre Benutzer-`~/.claude/settings.json`
* Verwaltete Einstellungen
* Einstellungen, die mit `--settings` übergeben werden

Claude Code wendet auch Genehmigungen aus einer nicht verfolgten `.claude/settings.local.json` an, aber es führt Git aus, um zu überprüfen, ob die Datei verfolgt wird, und führt diese Überprüfung nur in einem [vertrauenswürdigen Ordner](/docs/de/permissions#project-allow-rules-and-workspace-trust) durch. In einem Ordner, dem Sie noch nie vertraut haben, wartet Claude Code auf den Vertrauensdialog, bevor die Genehmigungen der Datei angewendet werden, es sei denn, der Ordner ist Ihr eigenes Konfigurationsheim: Ihr Home-Verzeichnis oder ein Verzeichnis, dessen `.claude` Sie als [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) festgelegt haben. Vor v2.1.207 genehmigte Claude Code Server aus einer nicht verfolgten `.claude/settings.local.json` sogar in einem Ordner, dem Sie noch nie vertraut haben.

Ein `disabledMcpjsonServers`-Eintrag in einer beliebigen Einstellungsdatei lehnt den Server weiterhin ab.

<h4 id="server-status-detail">
  Serverstatus-Detail
</h4>

In `/mcp`, einschließlich des Menüs eines Servers dort, und im [`/plugin`](/docs/de/plugins/install)-Manager kann ein Remote-HTTP- oder SSE-Server, den Sie zuvor verwendet haben, einen `cached`-Status wie `cached 2h ago · connects on first use · 5 tools` anzeigen. Claude Code lud die Tool-Liste des Servers aus seinem Discovery-Cache, der in einer vorherigen Sitzung gespeichert wurde, anstatt beim Start zu verbinden, und Claude Code verbindet den Server das erste Mal, wenn Claude eines der Tools des Servers aufruft. Die Tools sind ab Ihrer ersten Nachricht verfügbar, daher müssen Sie nichts tun. Der Discovery-Cache und sein `cached`-Status erfordern Claude Code v2.1.221 oder später.

Der Discovery-Cache ist standardmäßig deaktiviert, es sei denn, ein schrittweiser Rollout hat ihn für Ihr Konto aktiviert. Setzen Sie [`MCP_DISCOVERY_CACHE=1`](/docs/de/env-vars), um ihn einzuschalten, oder `0`, um ihn ausgeschaltet zu halten, auch wenn der Rollout ihn aktiviert hat. Vor v2.1.238 war der Cache standardmäßig aktiviert.

Zwei Aktionen im Menü eines Servers in `/mcp` beeinflussen auch den Cache-Eintrag dieses Servers:

* **Reconnect**: Bei einem `cached`-Server verbindet Claude Code ihn jetzt statt beim ersten Tool-Aufruf und behält den Eintrag. Bei einem verbundenen oder fehlgeschlagenen Server verbindet Claude Code ihn erneut und verwirft auch den Eintrag.
* **Clear authentication**: Claude Code widerruft die Authentifizierung des Servers und verwirft auch den Eintrag.

Nach dem Verwerfen des Eintrags ruft Claude Code die Tool-Liste des Servers vom Server ab, anstatt aus dem Cache.

Wenn der Status eines Servers `✘ Failed to connect` ist, hängt `claude mcp list` das Fehlerdetail an diese Statuszeile an, und `claude mcp get <name>` zeigt es auf einer `Issue:`-Zeile: der HTTP-Status oder Fehlercode, plus jeder Fehlertext, den der Server zurückgegeben hat. Die Detailansicht des Servers in `/mcp` enthält den gleichen vom Server gemeldeten Text in ihrer `Issue:`-Zeile. Claude Code redigiert Credential-ähnlichen Text aus diesem Detail und enthält niemals die erweiterte Server-URL, die Geheimnisse tragen kann. Claude Code hängt keinen Detail an einen `✘ Connection error`-Status an, da der Exception-Text, den es dort drucken würde, diese URL einbetten kann. Vor v2.1.219 zeigten beide Befehle nur den bloßen Fehlerstatus ohne Statuscode oder Fehlertext des Servers.

Wenn Sie die Authentifizierung von `/mcp` aus abschließen und die Verbindung immer noch mit einem HTTP-Status oder einem Transport-Fehlercode fehlschlägt, fügt Claude Code diesen Code und den Ursprung der URL des Servers zur Nachricht hinzu, die es nach dem Versuch druckt. Der Ursprung ist das Schema und der Host, plus der Port, wenn die URL einen benennt, wie `https://mcp.example.com`.

* Der Pfad und die Abfrage erscheinen niemals in dieser Nachricht.
* Für einen Server im lokalen, Projekt- oder Benutzer-[Bereich](#mcp-installation-scopes) oder in verwalteter MCP-Konfiguration zeigt der Ursprung den Host, wie er in dieser Konfiguration geschrieben ist, daher wird eine `${VAR}`-Referenz im Host nicht in der Nachricht erweitert.
* Bei einem Fehler ohne Status oder Fehlercode zeigt Claude Code den Fehlertext ohne den Ursprung.

Ein Remote-Server, dessen Konfiguration eine leere `url` hat, wird in `/mcp`, in `claude mcp list` und im [`/plugin`](/docs/de/plugins/install)-Manager als `not configured` angezeigt, und Claude Code versucht nicht, sich damit zu verbinden. Ein Plugin kann einen Platzhalter-Eintrag wie diesen für einen Connector enthalten, den Sie später konfigurieren, sodass Claude Code ihn nicht als Fehler oder Setup-Problem meldet. Die Detailansicht des Servers in `/mcp` liest `No URL configured for this server`; setzen Sie die `url` des Eintrags, um sich zu verbinden. Vor v2.1.208 meldete Claude Code eine leere `url` als Konfigurationsproblem mit einer Aufforderung zur Wiederverbindung.

<h4 id="configuration-warnings">
  Konfigurationswarnungen
</h4>

Claude Code warnt vor den folgenden Konfigurationsproblemen. Jeder Eintrag sagt, was Claude Code überprüft und wie die Warnung gelöscht wird:

* **Versteckte Leerzeichen**: Claude Code warnt, wenn ein MCP-Konfigurationswert versteckte führende oder nachfolgende Leerzeichen trägt, die oft vom Einfügen eines Tokens mit einem nachfolgenden Zeilenumbruch stammen. Claude Code überprüft `command`, `url`, jeden `args`-Eintrag und die Werte und Schlüsselnamen unter `env` und `headers`. Claude Code zeigt die Warnung in der `claude mcp list`-Ausgabe und in `/mcp` an und benennt die betroffenen Felder, ohne ihre Werte zu wiederholen, zum Beispiel `Leading or trailing whitespace in: headers.Authorization`. Claude Code trimmt das Leerzeichen nicht und verwendet die Werte genau wie geschrieben, daher bearbeiten Sie die Konfiguration, um es zu entfernen.
* **Gleicher Name in mehr als einem Bereich**: Wenn Sie den gleichen Servernamen in mehr als einem [Bereich](#mcp-installation-scopes) mit verschiedenen Endpunkten definieren, warnt Claude Code vor dem Konflikt in der `claude mcp list`-Ausgabe und in `/mcp`. Claude Code speichert OAuth-Anmeldungen pro Endpunkt, daher müssen Sie sich, wenn Sie die Definition authentifizieren, die in einem Projekt geladen wird, immer noch separat in einem Projekt anmelden, in dem eine andere Definition geladen wird. Behalten Sie den Endpunkt, den Sie möchten, und entfernen Sie die anderen mit `claude mcp remove <name> --scope <scope>`. In der Warnung zitiert Claude Code den Endpunkt jedes Bereichs, wie er in Ihrer Konfiguration geschrieben ist, mit [`${VAR}`-Referenzen](#environment-variable-expansion-in-mcp-json) nicht erweitert, daher zeigt es niemals einen aufgelösten Wert wie einen API-Schlüssel.
* **Reservierte Namen**: Claude Code reserviert die Namen seiner integrierten Server, einschließlich `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview` und `Claude Browser`. Wenn Ihre Konfiguration einen Server mit einem reservierten Namen definiert, überspringt Claude Code ihn beim Laden und zeigt eine Warnung an, die Sie auffordert, ihn umzubenennen. `claude mcp add` lehnt einen reservierten Namen mit einem Fehler ab. `Claude Preview` und `Claude Browser` benennen beide den integrierten Server, den der [Claude Code Desktop-App-Vorschaubereich](/docs/de/desktop#preview-your-app) verwendet. Vor v2.1.205 war `Claude Browser` nicht reserviert, daher konnte ein benutzerkonfigurierter Server sich unter diesem Namen registrieren.
* **Fehlende Umgebungsvariable**: Wenn eine [`${VAR}`-Referenz](#environment-variable-expansion-in-mcp-json) in der Konfiguration eines Servers eine Variable benennt, die nicht gesetzt ist und keinen `:-default` hat, warnt Claude Code in der `claude mcp list`-Ausgabe und in `/mcp`, benennt die Variable und lädt den Server immer noch mit dem `${VAR}`-Text nicht erweitert. Setzen Sie die Variable oder fügen Sie einen `${VAR:-default}`-Fallback hinzu. In der `url` und `headers` eines Remote-Servers lesen einige Credential-Variablen [als leer](#credential-variables-that-read-as-empty) statt, ohne Warnung.

<h4 id="tool-availability">
  Tool-Verfügbarkeit
</h4>

Das `/mcp`-Panel zeigt die Tool-Anzahl neben jedem verbundenen Server an und kennzeichnet Server, die die Tools-Funktion ankündigen, aber keine Tools bereitstellen.

Wenn Ihre Anfrage Tools von einem Server benötigt, der sich noch im Hintergrund verbindet, wartet Claude auf diesen Server, bevor er fortfährt. Wie das Warten geschieht, hängt von Ihrer Konfiguration ab:

* **Mit [Tool-Suche](#scale-with-mcp-tool-search), der Standardeinstellung**: Das Warten erfolgt innerhalb des `ToolSearch`-Aufrufs.
* **Ohne Tool-Suche**: Claude verwendet stattdessen das `WaitForMcpServers`-Tool. Konfigurationen ohne Tool-Suche enthalten eine benutzerdefinierte `ANTHROPIC_BASE_URL`, `ENABLE_TOOL_SEARCH=false` und ein Modell früher als die Claude 4.5-Generation auf Google Cloud's Agent Platform.
* **Bei einer Microsoft Foundry-[Bereitstellung auf Azure gehostet](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)**: Claude startet auf dem Tool-Suche-Pfad statt mit `WaitForMcpServers`, da Claude Code die serverseitige Ablehnung der Bereitstellung nur von der API entdeckt. Nachdem Claude Code diese Bereitstellung auf [vorausgehendes Laden](#scale-with-mcp-tool-search) umschaltet, werden Tools von einem Server, der die Verbindung beendet, bei Claudes nächster Anfrage verfügbar.

Mit aktivierter Tool-Suche, wenn ein Server die Verbindung beendet, während Claude arbeitet, listet Claude Code die Tool-Namen des Servers bei seiner nächsten Anfrage in der gleichen Runde zu Claude auf. Claude kann dann diese Tools suchen und aufrufen, ohne auf Ihre nächste Nachricht zu warten.

<h3 id="disable-a-server-without-removing-it">
  Einen Server deaktivieren, ohne ihn zu entfernen
</h3>

Schalten Sie einen Server im `/mcp`-Panel aus, um Claude Code daran zu hindern, sich damit zu verbinden, ohne seine Konfiguration zu verlieren. Claude Code listet den Server immer noch in `/mcp` auf, markiert als deaktiviert.

Wenn Sie einen Server umschalten, zeichnet Claude Code Ihre Wahl pro Projekt in `~/.claude.json` in einer von zwei Listen auf, die disjunkte Sätze von Servern abdecken:

* `disabledMcpServers`: Eine Opt-out-Liste für benutzerkonfigurierte Server, Plugin-Server, Server, die Ihre Organisation [über verwaltete Einstellungen bereitstellt](/docs/de/managed-mcp#provide-servers-through-managed-settings), die claude.ai-Connectoren, die Claude Code [selbst abruft](#how-connectors-reach-claude-code), und integrierte Server, die standardmäßig aktiviert sind. Claude Code verbindet sich nicht mit einem Server, den Sie hier auflisten. Wenn Sie einen claude.ai-Connector mit dem Pro-Projekt-`/mcp`-Umschalter deaktivieren, der in [claude.ai-Connectoren deaktivieren](#disable-claude-ai-connectors) beschrieben ist, schreibt Claude Code ihn in diese Liste unter seinem Anzeigenamen, zum Beispiel `claude.ai Slack`.
* `enabledMcpServers`: Eine Opt-in-Liste für integrierte Server, die standardmäßig deaktiviert sind, wie `computer-use`. Claude Code verbindet sich mit einem standardmäßig deaktivierten Server nur, wenn Sie ihn hier auflisten.

Claude Code konsultiert genau eine der beiden Listen für jeden Server, daher überschreibt keine Liste die andere. Wenn Sie einen regulären Server zu `enabledMcpServers` hinzufügen oder einen standardmäßig deaktivierten integrierten Server zu `disabledMcpServers`, ignoriert Claude Code den Eintrag.

`disabledMcpServers` und `enabledMcpServers` sind nicht verwandt mit [`enabledMcpjsonServers`](/docs/de/settings-reference#enabledmcpjsonservers) und [`disabledMcpjsonServers`](/docs/de/settings-reference#disabledmcpjsonservers), die die Genehmigung von Servern steuern, die in der `.mcp.json`-Datei eines Projekts definiert sind.

<h3 id="mcp-client-runtimes">
  MCP-Client-Runtimes
</h3>

Claude Code verbindet sich mit MCP-Servern über eine von zwei Client-Runtimes. Die v1-Runtime basiert auf MCP TypeScript SDK 1.x. Die v2-Runtime ist der gleiche Code auf [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/), das MCP-Protokoll-Revision 2026-07-28 hinzufügt. Der Rest dieser Seite gilt für beide Runtimes, außer wo ein Abschnitt die v2-Runtime benennt.

Claude Code wählt jedes Mal, wenn Sie es starten, eine Runtime aus und behält sie bis zum Beenden. In Sitzungen, in denen es [Feature-Flags abruft](/docs/de/env-vars#features-that-need-feature-flag-fetching), verwendet es die v2-Runtime bei Claude Code v2.1.232 oder später.

In den Sitzungen, in denen es keine Feature-Flags abruft, verwendet Claude Code die v2-Runtime standardmäßig bei Claude Code v2.1.274 oder später:

* Sitzungen auf Amazon Bedrock, Claude Platform auf AWS, Google Cloud's Agent Platform oder Microsoft Foundry, es sei denn, eine Host-Plattform, die Claude Code einbettet, setzt [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/de/env-vars)
* Sitzungen angemeldet über ein [Claude-Apps-Gateway](/docs/de/claude-apps-gateway)
* Sitzungen, in denen Sie Telemetrie oder Feature-Flag-Abruf ausschalten, zum Beispiel mit `DISABLE_TELEMETRY`

Bei v2 macht Claude Code auch:

* Fragt HTTP-Server, ob sie die neuere Revision unterstützen, und verwendet sie mit denen, die das tun. Es fragt auch claude.ai-Connector-Server in Sitzungen, in denen es Feature-Flags abruft. Um es Stdio-Server zu fragen oder Connector-Server in jeder Sitzung zu fragen, setzen Sie [`MCP_PROTOCOL_NEGOTIATION`](/docs/de/env-vars) auf `auto`. Es verbindet sich mit jedem anderen Server wie v1.
* Empfängt `list_changed`-Benachrichtigungen von Servern auf der neueren Revision über einen [Stream, den es offen hält](#notification-streams-on-the-v2-runtime).
* Registriert keinen [Kanal](#push-messages-with-channels)-Server, der sich auf der neueren Revision verbindet, da diese Revision keine Kanal-Nachrichten tragen kann.
* Schlägt eine [MCP-OAuth-Anmeldung](#authenticate-with-remote-mcp-servers) fehl, deren Autorisierungsantwort einen unerwarteten Aussteller benennt.

Anthropic kann einen bestimmten Server auf dem früheren Protokoll halten oder von diesem Stream entfernen, mit einem Feature-Flag, das Claude Code abruft.

Um die Runtime selbst zu wählen, setzen Sie [`MCP_SDK_GENERATION`](/docs/de/env-vars) auf `v1` oder `v2`. Um zu entscheiden, ob Claude Code fragt, setzen Sie [`MCP_PROTOCOL_NEGOTIATION`](/docs/de/env-vars) auf `auto` oder `legacy`.

<h3 id="dynamic-tool-updates">
  Dynamische Tool-Updates
</h3>

Claude Code unterstützt MCP-`list_changed`-Benachrichtigungen, die es MCP-Servern ermöglichen, ihre verfügbaren Tools, Prompts und Ressourcen dynamisch zu aktualisieren, ohne dass Sie die Verbindung trennen und erneut verbinden müssen. Wenn ein MCP-Server eine `list_changed`-Benachrichtigung sendet, aktualisiert Claude Code automatisch die verfügbaren Funktionen von diesem Server.

Wenn eine Aktualisierungsanfrage fehlschlägt, behält Claude Code die zuvor entdeckten Tools, Prompts und Ressourcen des Servers bis zu einer späteren erfolgreichen Aktualisierung. Vor v2.1.214 ersetzte ein vorübergehender Fehler während der Aktualisierung die Tools, Prompts und Ressourcen des Servers durch eine leere Liste.

<h4 id="notification-streams-on-the-v2-runtime">
  Benachrichtigungs-Streams auf der v2-Runtime
</h4>

Bei der [v2-Runtime](#mcp-client-runtimes) empfängt Claude Code `list_changed`-Benachrichtigungen von einem Server auf der neueren Protokoll-Revision über einen Stream, den es offen hält. Wenn der Stream schließt, öffnet Claude Code ihn erneut, mit zwei Grenzen:

* **Der Stream schließt innerhalb von 10 Sekunden erneut**: Claude Code öffnet ihn bis zu dreimal erneut, dann stoppt es für diese Verbindung.
* **Der Stream bleibt länger als 10 Sekunden offen, dann schließt**, wie Streams zu serverlosen Hosts häufig tun: Nach fünf Wiederöffnungen in einer Stunde wartet Claude Code etwa sechs Stunden vor der nächsten.

Bis der Stream erneut öffnet, behalten Sie die zuletzt abgerufenen Tools, Prompts und Ressourcen des Servers. Um seine Änderungen schneller zu erkennen, verbinden Sie den Server von `/mcp` erneut.

<h3 id="automatic-reconnection">
  Automatische Wiederverbindung
</h3>

Claude Code verbindet einen Remote-Server, der während einer Sitzung die Verbindung trennt, erneut und versucht die erste Verbindung eines HTTP- oder SSE-Servers nach einem vorübergehenden Fehler erneut. Stdio-Server sind lokale Prozesse, und Claude Code verbindet sie nicht automatisch erneut.

<h4 id="mid-session-drops-of-a-remote-server">
  Verbindungstrennung eines Remote-Servers während einer Sitzung
</h4>

Claude Code verbindet einen getrennten Remote-Server mit exponentiellem Backoff erneut: bis zu fünf Versuche, beginnend mit einer Verzögerung von einer Sekunde und sich jedes Mal verdoppelnd. Was Sie sehen, hängt davon ab, wie Sie Claude Code ausführen:

* **In einer interaktiven Sitzung**: `/mcp` zeigt den Server als ausstehend an, während Claude Code erneut verbindet. Nach fünf fehlgeschlagenen Versuchen markiert Claude Code den Server als fehlgeschlagen oder als Authentifizierung erforderlich, wenn der Server erneut autorisiert werden muss. Sie können manuell von `/mcp` aus erneut versuchen.
* **In [`claude -p`](/docs/de/headless)-Läufen und [Agent SDK](/docs/de/agent-sdk/overview)-Sitzungen**: Claude Code verbindet sich nach dem gleichen Zeitplan erneut, ohne `/mcp`-Panel, um die Versuche anzuzeigen.

<h4 id="failed-first-connections">
  Fehlgeschlagene erste Verbindungen
</h4>

Wenn die erste Verbindung eines HTTP- oder SSE-Servers mit einem vorübergehenden Fehler fehlschlägt, wie eine 5xx-Antwort, eine Verbindungsverweigerung oder ein Timeout, versucht Claude Code bis zu dreimal erneut. Wenn die Verbindung immer noch fehlschlägt, markiert Claude Code den Server als fehlgeschlagen. Claude Code versucht auf diese Weise beim Start und wenn ein Server während einer Sitzung hinzugefügt wird. Das schließt einen Server ein, den Claude Code zu einer [Cloud-Sitzung](/docs/de/claude-code-on-the-web) aus seiner Konfiguration hinzufügt, und einen Server, den Sie mit der Agent SDK's [`setMcpServers()`](/docs/de/agent-sdk/typescript) hinzufügen.

Claude Code versucht nicht in diesen Fällen erneut:

* Eine WebSocket-Server-Verbindung
* Ein Authentifizierungs- oder Not-Found-Fehler, da er eine Konfigurationsänderung erfordert, um behoben zu werden. Wenn ein [`headersHelper`](#use-dynamic-headers-for-custom-authentication) die einzige Quelle des `Authorization`-Headers des Servers ist, versucht Claude Code einen Authentifizierungsfehler trotzdem erneut, da er den Helper bei jedem Versuch erneut ausführt und eine frische Anmeldedaten abholen kann

<h4 id="failed-discovery-requests">
  Fehlgeschlagene Discovery-Anfragen
</h4>

Nachdem ein Server verbunden ist, sendet Claude Code ihm Funktionsentdeckungsanfragen wie `tools/list`, `prompts/list` und `resources/list`. Claude Code versucht diese Anfragen bis zu dreimal mit kurzem Backoff nach einem vorübergehenden Netzwerk- oder Serverfehler erneut. Es versucht Authentifizierungsfehler, 4xx-Antworten oder Request-Timeouts nicht erneut.

<h4 id="how-claude-learns-that-a-server-failed">
  Wie Claude erfährt, dass ein Server fehlgeschlagen ist
</h4>

Ob Claude Code Claude über einen konfigurierten Server, der sich nicht verbinden konnte, informiert, hängt von [Tool-Suche](#scale-with-mcp-tool-search) ab, die standardmäßig aktiviert ist:

* Mit Tool-Suche teilt Claude Code Claude mit, welcher Server fehlgeschlagen ist und sein Verbindungsfehler, daher meldet Claude den Verbindungsfehler in seiner Antwort. Claude Code enthält die gleichen Informationen in `ToolSearch`-Ergebnissen, die kein passendes Tool finden.
* In jeder [Konfiguration ohne Tool-Suche](#configure-tool-search) meldet Claude Code fehlgeschlagene Server-Verbindungen nicht an Claude.

<h3 id="push-messages-with-channels">
  Push-Nachrichten mit Kanälen
</h3>

Ein MCP-Server kann auch Nachrichten direkt in Ihre Sitzung pushen, sodass Claude auf externe Ereignisse wie CI-Ergebnisse, Überwachungswarnungen oder Chat-Nachrichten reagieren kann. Um dies zu aktivieren, deklariert Ihr Server die Funktion `claude/channel` und Sie aktivieren sie mit dem Flag `--channels` beim Start. Siehe [Kanäle](/docs/de/channels), um einen offiziell unterstützten Kanal zu verwenden, oder [Kanäle-Referenz](/docs/de/channels-reference), um Ihren eigenen zu erstellen.

Bei der [v2-Runtime](#mcp-client-runtimes), wenn Sie [`MCP_PROTOCOL_NEGOTIATION`](/docs/de/env-vars) auf `auto` setzen und ein Kanal-Server die MCP-Protokoll-Revision 2026-07-28 aushandelt, kann er keine Kanal-Nachrichten liefern, daher registriert Claude Code ihn nicht als Kanal. Das Verlassen der Variable ungesetzt oder das Setzen auf `legacy` hält Stdio-Server auf dem früheren Handshake.

<Tip>
  Tipps:

  * Verwenden Sie das Flag `-s` oder `--scope`, um anzugeben, wo die Konfiguration gespeichert wird:
    * `local` (Standard): Nur für Sie im aktuellen Projekt verfügbar
    * `project`: Geteilt mit allen im Projekt über die Datei `.mcp.json`
    * `user`: Für Sie über alle Projekte hinweg verfügbar
  * Legen Sie Umgebungsvariablen mit `-e` oder `--env`-Flags fest (zum Beispiel `-e KEY=value`)
  * Die Flags `--transport` und `--header` akzeptieren auch die Kurzformen `-t` und `-H`
  * Konfigurieren Sie das Startup-Timeout des MCP-Servers mit der Umgebungsvariablen `MCP_TIMEOUT` (zum Beispiel `MCP_TIMEOUT=10000 claude` setzt ein 10-Sekunden-Timeout)
  * Setzen Sie ein Pro-Server-Tool-Ausführungs-Timeout, indem Sie ein `timeout`-Feld in Millisekunden zum `.mcp.json`-Eintrag dieses Servers hinzufügen, zum Beispiel `"timeout": 600000` für zehn Minuten. Dies überschreibt die Umgebungsvariable `MCP_TOOL_TIMEOUT` nur für diesen Server
  * Claude Code zeigt eine Warnung an, wenn die MCP-Tool-Ausgabe 10.000 Token überschreitet und begrenzt die Ausgabe standardmäßig auf 25.000 Token. Um dieses Limit zu erhöhen, setzen Sie die Umgebungsvariable `MAX_MCP_OUTPUT_TOKENS` (zum Beispiel `MAX_MCP_OUTPUT_TOKENS=50000`); die Warnschwelle ist fest. Siehe [MCP-Ausgabegrenzen und Warnungen](#mcp-output-limits-and-warnings)
  * Verwenden Sie `/mcp`, um sich bei Remote-Servern zu authentifizieren, die OAuth 2.0-Authentifizierung erfordern
</Tip>

Das Pro-Server-`timeout` ist eine harte Wanduhr-Grenze pro Tool-Aufruf, und Fortschrittsbenachrichtigungen vom Server verlängern sie nicht. Werte unter 1000 werden ignoriert und fallen auf `MCP_TOOL_TIMEOUT` zurück, oder auf seinen Standard von etwa 28 Stunden, wenn diese Variable nicht gesetzt ist. Für einen HTTP-, SSE- oder [claude.ai-Connector](/docs/de/mcp#use-mcp-servers-from-claude-ai)-Server gibt es auch einen zweiten, Pro-Request-Timer, der jeden Request bis zur ersten Antwort-Byte des Servers abdeckt. Claude Code setzt diesen Timer auf das Maximum von drei Werten: 60 Sekunden, das Tool-Timeout, das für den Server gilt, und `MCP_TIMEOUT`. Der 28-Stunden-Standard eines nicht gesetzten `MCP_TOOL_TIMEOUT` speist diese Vergleichung nicht, und ein Wert unter 60 Sekunden verkürzt den Timer nicht. Stdio- und WebSocket-Server haben keinen Pro-Request-Timer.

Ein Pro-Server-`timeout` von mindestens 1000 fungiert auch als Untergrenze für das unten beschriebene Idle-Timeout: Claude Code bricht die Tool-Aufrufe dieses Servers niemals wegen Untätigkeit früher ab als das Pro-Server-`timeout`. Erfordert Claude Code v2.1.203 oder später.

Ein Tool-Aufruf an einen MCP-Server, der für das Idle-Fenster keine Antwort und keine Fortschrittsbenachrichtigung sendet, bricht mit einem Fehler ab, anstatt auf die Wanduhr-Grenze zu warten. Das Idle-Timeout gilt für jeden Server-Typ außer IDE-Servern und SDK-In-Process-Servern. Das Idle-Fenster beträgt standardmäßig fünf Minuten für HTTP-, SSE-, WebSocket- und [claude.ai-Connector](#use-mcp-servers-from-claude-ai)-Server und 30 Minuten für Stdio-Server. Vor v2.1.203 waren Stdio-Server vom Idle-Timeout ausgenommen.

Setzen Sie die Umgebungsvariable [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/de/env-vars) in Millisekunden, um das Idle-Fenster zu ändern, oder setzen Sie sie auf `0`, um die Überprüfung zu deaktivieren.

Diese Timeouts begrenzen, wie lange ein Aufruf ausgeführt werden kann, nicht immer wie lange er die Sitzung blockiert: Ein Hauptkonversations-Aufruf, der zwei Minuten überschreitet, wird zuerst zu einer Hintergrund-Aufgabe. Siehe [Automatisches Backgrounding von langen Tool-Aufrufen](#automatic-backgrounding-of-long-tool-calls).

<h3 id="automatic-backgrounding-of-long-tool-calls">
  Automatisches Backgrounding von langen Tool-Aufrufen
</h3>

Ein MCP-Tool-Aufruf in der Hauptkonversation, der nach zwei Minuten noch läuft, wird zu einer Hintergrund-Aufgabe, anstatt die Sitzung zu blockieren. Claude erhält die Aufgaben-ID sofort und arbeitet weiter, und das Ergebnis kommt als Aufgaben-Benachrichtigung, wenn der Aufruf sich setzt. Automatisches Backgrounding erfordert Claude Code v2.1.212 oder später.

Die Aufgabe erscheint in [`/tasks`](/docs/de/commands#all-commands), wo Sie sie auch stoppen können, und sie überlebt nicht das Beenden der Sitzung. Die Pro-Aufruf-Grenzen gelten immer noch, während der Aufruf im Hintergrund läuft: die Wanduhr-Grenze, die durch das Pro-Server-`timeout` oder [`MCP_TOOL_TIMEOUT`](/docs/de/env-vars) gesetzt wird, und das Idle-Timeout, das durch [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/de/env-vars) gesetzt wird.

Setzen Sie die Umgebungsvariable [`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](/docs/de/env-vars) in Millisekunden, um den Schwellenwert zu ändern, oder setzen Sie sie auf `0`, um automatisches Backgrounding auszuschalten. Das Setzen von `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` auf `1` schaltet es auch aus, zusammen mit allen anderen Hintergrund-Aufgaben-Funktionen.

Einige Aufrufe werden niemals in den Hintergrund verschoben:

* Aufrufe von [Subagenten](/docs/de/sub-agents); Claude Code verschiebt nur Hauptkonversations-Aufrufe
* Aufrufe an IDE-Server
* Aufrufe im [nicht-interaktiven Modus](/docs/de/headless), es sei denn, `CLAUDE_AUTO_BACKGROUND_TASKS` ist auf `1` gesetzt, da ein One-Shot-Lauf enden kann, bevor das Ergebnis ankommt

Ein Aufruf, der auf einen offenen [Elicitation-Dialog](#respond-to-mcp-elicitation-requests) wartet, wird nicht in den Hintergrund verschoben, während der Dialog offen ist; der Server ist auf Ihre Eingabe blockiert, nicht langsam, daher verschiebt Claude Code die Verschiebung bis zum Schließen des Dialogs.

<h3 id="plugin-provided-mcp-servers">
  Von Plugins bereitgestellte MCP-Server
</h3>

[Plugins](/docs/de/plugins/overview) können MCP-Server bündeln, die Tools und Integrationen bereitstellen, wenn Sie das Plugin aktivieren. Plugin-MCP-Server funktionieren identisch mit benutzerkonfigurierten Servern.

**Wie Plugin-MCP-Server funktionieren**:

* Plugins definieren MCP-Server in `.mcp.json` im Plugin-Root oder inline in `plugin.json`
* Wenn Sie ein Plugin aktivieren, starten seine MCP-Server automatisch
* Claude Code bietet Plugin-MCP-Tools neben manuell konfigurierten MCP-Tools an
* Sie fügen Plugin-Server hinzu und entfernen sie durch Installation oder Deinstallation des Plugins, nicht mit `/mcp`-Befehlen. Sie können einen installierten Plugin-Server immer noch [ohne Entfernung ausschalten](#disable-a-server-without-removing-it) in `/mcp`, was Claude Code daran hindert, sich damit zu verbinden, ohne das Plugin zu entfernen

**Beispiel-Plugin-MCP-Konfiguration**:

In `.mcp.json` im Plugin-Root:

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

Oder inline in `plugin.json`:

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**Plugin-MCP-Funktionen**:

* **Automatischer Lebenszyklus**: Server verbinden und trennen sich an diesen Punkten:
  * Beim Sitzungsstart verbindet Claude Code die Server für aktivierte Plugins automatisch. In `/mcp` kann ein Remote-HTTP- oder SSE-Plugin-Server, den Sie zuvor verwendet haben, den [`cached`-Status](#server-status-detail) statt anzeigen; Claude Code verbindet ihn, wenn Claude eines seiner Tools zum ersten Mal aufruft
  * Wenn Sie ein Plugin während einer Sitzung aktivieren oder deaktivieren, verbindet Claude Code seine MCP-Server oder trennt sie, wenn die Änderung angewendet wird. [Plugin-Änderungen ohne Neustart anwenden](/docs/de/plugins/cli-reference#reload-plugins) beschreibt, wann das ist. In einer Sitzung ohne interaktives Terminal verbindet oder trennt `/reload-plugins` Plugin-MCP-Server nicht; diese Änderungen treten in Ihrer nächsten Sitzung in Kraft
  * Wenn Sie neu laden, behält Claude Code die Live-Verbindungen von Plugin-Servern, deren Konfiguration unverändert ist, und macht das gleiche, wenn Sie [die MCP-Server-Liste der Sitzung](/docs/de/agent-sdk/typescript#mcpsetserversresult) vom Agent SDK ersetzen, ohne sie zu benennen
  * Wenn Sie die Sitzung mit `/cd` [in ein anderes Verzeichnis verschieben](/docs/de/permissions#move-the-session-to-another-directory) bei v2.1.246 oder später, verbindet Claude Code die Server von Plugins, die die Einstellungen des neuen Verzeichnisses aktivieren, und trennt die Server von Plugins, die nicht mehr aktiviert sind, daher müssen Sie `/reload-plugins` nach der Verschiebung nicht ausführen
  * In [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) startet ein MCP-Aufruf an einen Plugin-Server, der noch nicht verbunden ist, wie direkt nach einer Idle-Sitzung, die aufwacht, den Server bei Bedarf und wartet auf die Verbindung
* **Pfad-Platzhalter**: `${CLAUDE_PLUGIN_ROOT}` wird in das Installationsverzeichnis des Plugins aufgelöst, `${CLAUDE_PLUGIN_DATA}` in sein [persistentes Zustandsverzeichnis](/docs/de/plugins/components#path-variables-and-persistent-data), und `${CLAUDE_PROJECT_DIR}` in das stabile Projektstammverzeichnis. Die Substitution gilt für:
  * `stdio`-Server: `command`, `args`, `env`
  * `http`-, `sse`- und `ws`-Server: `url`, `headers` und `headersHelper`. Vor v2.1.195 übergab `headersHelper` den Platzhalter als Literal-String
* **Zugriff auf Benutzerumgebung**: Zugriff auf die gleichen Umgebungsvariablen wie manuell konfigurierte Server
* **Mehrere Transporttypen**: Unterstützung für Stdio-, SSE-, HTTP- und WebSocket-Transporte, wobei die Transportunterstützung je nach Server variieren kann

Plugin-Server erscheinen in `/mcp` mit Indikatoren, die zeigen, dass sie von Plugins stammen.

**Plugin-MCP-Tool-Namen**:

Tools von einem Plugin-gebündelten MCP-Server enthalten sowohl den Plugin-Namen als auch den Server-Schlüssel in ihrem aufrufbaren Namen. Die vollständige Form ist `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, wobei jedes Zeichen außerhalb von `A-Z`, `a-z`, `0-9`, `_` und `-` durch `_` ersetzt wird. Für den Server `database-tools`, der in einem Plugin namens `my-plugin` gebündelt ist, ist ein `query`-Tool aufrufbar als:

```
mcp__plugin_my-plugin_database-tools__query
```

Verwenden Sie diesen vollständigen Namen, wenn Sie auf das Tool in [Berechtigungsregeln](/docs/de/permissions), der `allowed-tools`-Liste eines Skills, dem [`tools`-Feld eines Subagenten](/docs/de/sub-agents#available-tools) oder einem [Hook-Matcher](/docs/de/hooks#match-mcp-tools) verweisen. Ein Hook-Matcher, der gegen den bloßen Server-Schlüssel geschrieben wurde, wie `mcp__database-tools__.*`, wird niemals für einen Plugin-gebündelten Server ausgelöst.

Der Server selbst registriert sich unter dem scoped Namen `plugin:<plugin-name>:<server-name>`, wie `plugin:my-plugin:database-tools`. Verwenden Sie diesen Namen, wo ein konfigurierter Server-Name erwartet wird, wie das [`server`-Feld eines `mcp_tool`-Hooks](/docs/de/hooks#mcp-tool-hook-fields).

Siehe die [Plugin-Komponenten-Referenz](/docs/de/plugins/components#mcp-servers) für Details zum Bündeln von MCP-Servern mit Plugins.

<h2 id="mcp-installation-scopes">
  MCP-Installationsbereiche
</h2>

MCP-Server können auf drei verschiedenen Bereichsebenen konfiguriert werden. Der Bereich, den Sie wählen, steuert, in welchen Projekten der Server geladen wird und ob die Konfiguration mit Ihrem Team geteilt wird. Administratoren können Server auch auf Unternehmensebene über [verwaltete Konfiguration](#managed-mcp-configuration) bereitstellen oder zur Verfügung stellen.

| Bereich                   | Wird geladen in       | Mit Team geteilt           | Gespeichert in              |
| ------------------------- | --------------------- | -------------------------- | --------------------------- |
| [Lokal](#local-scope)     | Nur aktuelles Projekt | Nein                       | `~/.claude.json`            |
| [Projekt](#project-scope) | Nur aktuelles Projekt | Ja, über Versionskontrolle | `.mcp.json` im Projekt-Root |
| [Benutzer](#user-scope)   | Alle Ihre Projekte    | Nein                       | `~/.claude.json`            |

<h3 id="local-scope">
  Lokaler Bereich
</h3>

Der lokale Bereich ist der Standard. Ein lokal begrenzter Server wird nur in dem Projekt geladen, in dem Sie ihn hinzugefügt haben, und bleibt privat für Sie. Claude Code speichert ihn in `~/.claude.json` unter dem Pfad dieses Projekts, daher wird derselbe Server nicht in Ihren anderen Projekten angezeigt. Verwenden Sie den lokalen Bereich für persönliche Entwicklungsserver, experimentelle Konfigurationen oder Server mit Anmeldedaten, die Sie nicht in der Versionskontrolle haben möchten.

<Note>
  Der Begriff „lokaler Bereich" für MCP-Server unterscheidet sich von allgemeinen lokalen Einstellungen. Lokal begrenzte MCP-Server werden in `~/.claude.json` (Ihr Home-Verzeichnis) gespeichert, während allgemeine lokale Einstellungen `.claude/settings.local.json` (im Projektverzeichnis) verwenden. Siehe [Einstellungen](/docs/de/settings#where-settings-live) für Details zu Einstellungsdatei-Speicherorten.
</Note>

```bash theme={null}
# Einen lokal begrenzten Server hinzufügen (Standard)
claude mcp add --transport http stripe https://mcp.stripe.com

# Lokalen Bereich explizit angeben
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

Der Befehl schreibt den Server in den Eintrag für Ihr aktuelles Projekt in `~/.claude.json`. Das folgende Beispiel zeigt das Ergebnis, wenn Sie ihn von `/path/to/your/project` aus ausführen:

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  Projektbereich
</h3>

Projektbegrenzte Server ermöglichen Teamzusammenarbeit durch das Speichern von Konfigurationen in einer `.mcp.json`-Datei im Root-Verzeichnis Ihres Projekts. Wenn Sie einen projektbegrenzten Server hinzufügen, erstellt oder aktualisiert Claude Code automatisch diese Datei mit der entsprechenden Konfigurationsstruktur. Checken Sie `.mcp.json` in die Versionskontrolle ein, damit alle Mitglieder Ihres Teams die gleichen MCP-Tools und -Dienste erhalten.

```bash theme={null}
# Einen projektbegrenzten Server hinzufügen
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

Die resultierende `.mcp.json`-Datei folgt einem standardisierten Format:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

Aus Sicherheitsgründen fordert Claude Code eine Genehmigung an, bevor projektbegrenzte Server aus `.mcp.json`-Dateien in interaktiven Sitzungen verwendet werden. Um diese Genehmigungswahlmöglichkeiten zurückzusetzen, führen Sie `claude mcp reset-project-choices` aus.

In `claude -p`-Läufen, [Agent SDK](/docs/de/headless)-Sitzungen und [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) kann Claude Code diese Eingabeaufforderung nicht anzeigen: Es lädt projektbegrenzte Server ohne Nachfrage. Claude Code überspringt die Eingabeaufforderung auch in einer Sitzung, die Sie im `bypassPermissions`-Modus mit [`skipDangerousModePermissionPrompt`](/docs/de/settings-reference#skipdangerousmodepermissionprompt) in Ihren Benutzereinstellungen oder in verwalteten Einstellungen starten. Um einen Server trotzdem auszuschließen:

* Fügen Sie ihn zu [`disabledMcpjsonServers`](/docs/de/settings-reference#disabledmcpjsonservers) hinzu, was ihn in jedem Berechtigungsmodus blockiert.
* Schließen Sie Projekteinstellungen vollständig mit [`--setting-sources`](/docs/de/cli-reference#cli-flags) oder der SDK-Option `settingSources` aus.
* Starten Sie die Sitzung mit [`--strict-mcp-config`](/docs/de/cli-reference#cli-flags). Claude Code verwendet dann nur die MCP-Server, die Sie mit `--mcp-config` übergeben. Das Überspringen der Genehmigungsaufforderung für die projektbegrenzten Server, die Claude Code nicht lädt, erfordert Claude Code v2.1.246 oder später; vor v2.1.246 wartete eine strikte Sitzung immer noch auf Genehmigung für diese, was Hintergrundsitzungen beim Start wartend ließ. Siehe [Exklusive Kontrolle mit managed-mcp.json](/docs/de/managed-mcp#exclusive-control-with-managed-mcp-json) für das, was das Flag unter einer verwalteten MCP-Datei tut.

[Projektserver-Genehmigungen und Workspace-Vertrauen](#project-server-approvals-and-workspace-trust) behandelt, wie Genehmigungen, die im Repository committed sind, mit Workspace-Vertrauen interagieren.

<h3 id="user-scope">
  Benutzerbereich
</h3>

Benutzerbegrenzte Server werden in `~/.claude.json` gespeichert und bieten projektübergreifende Zugänglichkeit, wodurch sie über alle Projekte auf Ihrem Computer verfügbar sind und gleichzeitig privat für Ihr Benutzerkonto bleiben. Dieser Bereich funktioniert gut für persönliche Utility-Server, Entwicklungstools oder Dienste, die Sie häufig über verschiedene Projekte hinweg verwenden.

```bash theme={null}
# Einen Benutzer-Server hinzufügen
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  Bereichshierarchie und Vorrang
</h3>

Wenn derselbe Server auf mehreren Bereichen definiert ist, verbindet sich Claude Code einmal damit und verwendet die Definition aus der höchsten Vorrangsquelle. Der gesamte Server-Eintrag aus dieser Quelle wird verwendet; Felder werden nicht über Bereiche hinweg zusammengeführt.

1. Lokaler Bereich
2. Projektbereich
3. Benutzerbereich
4. [Von Plugins bereitgestellte Server](/docs/de/plugins/components#mcp-servers)
5. [claude.ai-Connectoren](#use-mcp-servers-from-claude-ai)

Die drei Bereiche stimmen Duplikate nach Name ab. Plugins und Connectoren stimmen nach Endpunkt ab, daher wird einer, der auf die gleiche URL oder den gleichen Befehl wie ein Server oben verweist, als Duplikat behandelt.

Ein Server, den Ihre Organisation über die verwaltete Einstellung [`managedMcpServers`](/docs/de/managed-mcp#provide-servers-through-managed-settings) bereitstellt, hat Vorrang vor all diesen, daher verbindet sich Claude Code mit der Definition der Organisation, wenn einer von ihnen ihn dupliziert. Erfordert Claude Code v2.1.259 oder später.

Wenn Sie eine lokale Sitzung in der [Code-Registerkarte der Desktop-App](/docs/de/desktop#mcp-servers-from-the-claude-desktop-chat-app) mit dem gleichen stdio-Servernamen auf der obersten Ebene von `~/.claude.json` (Benutzerbereich) und in `.mcp.json` öffnen, verwendet die Code-Registerkarte die `~/.claude.json`-Definition.

<h3 id="environment-variable-expansion-in-mcp-json">
  Umgebungsvariablen-Erweiterung in `.mcp.json`
</h3>

Claude Code unterstützt die Umgebungsvariablen-Erweiterung in `.mcp.json`-Dateien, die es Teams ermöglicht, Konfigurationen zu teilen und gleichzeitig Flexibilität für maschinenspezifische Pfade und vertrauliche Werte wie API-Schlüssel zu bewahren.

<h4 id="supported-syntax">
  Unterstützte Syntax
</h4>

* `${VAR}`: erweitert sich zum Wert der Umgebungsvariablen `VAR`
* `${VAR:-default}`: erweitert sich zu `VAR`, wenn gesetzt, andernfalls wird `default` verwendet

<h4 id="expansion-locations">
  Erweiterungsorte
</h4>

Umgebungsvariablen können erweitert werden in:

* `command`: der Server-Ausführungspfad
* `args`: Befehlszeilenargumente
* `env`: Umgebungsvariablen, die an den Server übergeben werden
* `url`: für HTTP-Server-Typen
* `headers`: für HTTP-Server-Authentifizierung

<h4 id="example-with-variable-expansion">
  Beispiel mit Variablenerweiterung
</h4>

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

<h4 id="unset-variables-without-a-default">
  Nicht gesetzte Variablen ohne Standard
</h4>

Wenn eine referenzierte Umgebungsvariable nicht gesetzt ist und keinen Standardwert hat, wird die Konfiguration trotzdem geladen: Claude Code meldet eine Warnung zu fehlender Variable für diesen Server in der Ausgabe von `claude mcp list` und verwendet den nicht erweiterten `${VAR}`-Text unverändert. Setzen Sie die Variable oder fügen Sie einen `:-default`-Fallback hinzu, damit der Server mit dem beabsichtigten Wert startet. In einer Remote-Server-`url` und `headers` lesen einige Anmeldedatenvariablen [als leer](#credential-variables-that-read-as-empty) statt, ohne Warnung.

<h4 id="credential-variables-that-read-as-empty">
  Anmeldedatenvariablen, die als leer gelesen werden
</h4>

In einer Remote-Server-`url` und `headers` liest Claude Code Anmeldedatenvariablen aus Ihrer Umgebung als leer statt sie zu erweitern. Dies verhindert, dass ein Projekt `.mcp.json` oder ein Plugin Ihre Claude Code- oder Cloud-Provider-Anmeldedaten an einen Server sendet, den es benennt. Wenn Sie `Bearer ${ANTHROPIC_AUTH_TOKEN}` schreiben, erhält der Server `Bearer ` ohne Anmeldedaten und lehnt die Anfrage ab, normalerweise mit einem `401`. Claude Code meldet dies als fehlgeschlagene Verbindung.

Die abgedeckten Namen sind:

* Claude Code's eigene Anmeldedaten, wie `ANTHROPIC_API_KEY` und `ANTHROPIC_AUTH_TOKEN`
* Ihre Cloud-Provider-Anmeldedaten, wie `AWS_BEARER_TOKEN_BEDROCK`
* Andere Anmeldedaten, die Ihre Umgebung trägt, wie `HTTPS_PROXY` und `NPM_TOKEN`

Ein abgedeckter Name wird als leer gelesen, unabhängig davon, ob Sie die Variable gesetzt haben oder nicht, und ein `:-default`-Fallback darauf wird ignoriert. Eine Provider-Basis-URL wie `ANTHROPIC_BASE_URL` wird immer noch erweitert, daher funktioniert `"url": "${ANTHROPIC_BASE_URL}/mcp"`, es sei denn, der Wert der URL selbst bettet eine Anmeldedaten wie einen Benutzernamen und ein Passwort ein.

Ein Name außerhalb dieses Satzes, wie `API_KEY`, wird wie geschrieben erweitert. Um dem Server eine der abgedeckten Anmeldedaten zu geben, kopieren Sie sie in eine Variable mit einem Namen Ihrer Wahl und referenzieren Sie stattdessen diesen Namen.

Wenn eine Remote-Server-`url` oder `headers` eine abgedeckte Variable referenziert, die Sie gesetzt haben, nennt Claude Code sie in einer Debug-Log-Zeile. Um die Zeile zu lesen, führen Sie `claude --debug-file /tmp/claude-debug.log` aus und durchsuchen Sie diese Datei nach `never expanded toward a remote server`.

<h4 id="how-references-appear-in-/mcp-and-cli-output">
  Wie Referenzen in `/mcp` und CLI-Ausgabe angezeigt werden
</h4>

Für einen Server im lokalen, Projekt- oder Benutzer-[Bereich](#mcp-installation-scopes) zeigen die folgenden Oberflächen eine `${VAR}`-Referenz nach Name statt als ihren aufgelösten Wert:

* Die URL oder Befehlszeile in der Detailansicht eines Servers unter `/mcp`
* `claude mcp list` und `claude mcp get` Ausgabe

Die `/mcp` Detailansicht zeigt Referenzen auf diese Weise in Claude Code v2.1.268 oder später.

Für einen Server, den Ihre Organisation über die Einstellung `managedMcpServers` bereitstellt, zeigen diese Oberflächen [nur den Host der URL](/docs/de/managed-mcp#what-users-can-see-and-change).

Um zu überprüfen, was `claude mcp list`, `claude mcp get` und `/mcp` anzeigen, wenn eine Verbindung fehlschlägt, siehe [Server-Status-Detail](#server-status-detail).

<h2 id="practical-examples">
  Praktische Beispiele
</h2>

<h3 id="example-connect-to-github-for-code-reviews">
  Beispiel: Mit GitHub für Code-Reviews verbinden
</h3>

GitHubs Remote-MCP-Server authentifiziert sich mit einem GitHub-Personal-Access-Token, der als Header übergeben wird. Um einen zu erhalten, öffnen Sie Ihre [GitHub-Token-Einstellungen](https://github.com/settings/personal-access-tokens), generieren Sie ein neues feingranulares Token mit Zugriff auf die Repositories, mit denen Claude arbeiten soll, und fügen Sie dann den Server hinzu:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Ersetzen Sie `YOUR_GITHUB_PAT` durch Ihren persönlichen Zugriffs-Token. Der Befehl `claude mcp add` speichert die Konfiguration, ohne Anmeldedaten zu validieren, daher wird hier ein Platzhalterwert akzeptiert, aber der Server kann sich später nicht verbinden. Um die Verbindung zu überprüfen, führen Sie `/mcp` aus und überprüfen Sie, dass der Server `connected` anzeigt. Ein Server mit ungültigen Anmeldedaten zeigt `failed`, und die Fehlerdetails enthalten den HTTP-Status, den der Server zurückgegeben hat, z. B. einen 401.

Arbeiten Sie dann mit GitHub:

```text wrap theme={null}
Review PR #456 and suggest improvements
```

```text wrap theme={null}
Create a new issue for the bug we just found
```

```text wrap theme={null}
Show me all open PRs assigned to me
```

<h3 id="example-query-your-postgresql-database">
  Beispiel: Ihre PostgreSQL-Datenbank abfragen
</h3>

[DBHub](https://github.com/bytebase/dbhub), das Paket `@bytebase/dbhub`, ist ein MCP-Server, der Claude mit einer relationalen Datenbank über die Verbindungszeichenfolge verbindet, die Sie in `--dsn` übergeben. Verwenden Sie einen schreibgeschützten Datenbankbenutzer in der Verbindungszeichenfolge, damit die Abfragen, die Claude ausführt, keine Daten ändern können:

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Um zu bestätigen, dass der Server startet, führen Sie `/mcp` aus und überprüfen Sie, dass `db` `connected` anzeigt.

Fragen Sie dann Ihre Datenbank natürlich ab:

```text wrap theme={null}
What's our total revenue this month?
```

```text wrap theme={null}
Show me the schema for the orders table
```

```text wrap theme={null}
Find customers who haven't made a purchase in 90 days
```

<h2 id="authenticate-with-remote-mcp-servers">
  Mit Remote-MCP-Servern authentifizieren
</h2>

Viele Cloud-basierte MCP-Server erfordern Authentifizierung. Claude Code unterstützt OAuth 2.0 für sichere Verbindungen.

Claude Code markiert einen Remote-Server als authentifizierungsbedürftig, wenn der Server mit `401 Unauthorized` oder `403 Forbidden` antwortet. Was Claude Code anzeigt, hängt vom Server ab:

* Für einen Server, bei dem Sie sich nicht angemeldet haben, kennzeichnet jeder dieser Statuscodes den Server in `/mcp`, damit Sie den OAuth-Fluss abschließen können.
* Für einen [claude.ai-Connector](#use-mcp-servers-from-claude-ai) kennzeichnet ein `401`, das durch die Ablehnung Ihres Sitzungs-Tokens durch claude.ai verursacht wird, den Connector nicht, da eine erneute Autorisierung des Connectors Ihre Anmeldung nicht beheben kann. Claude Code zeigt stattdessen den [Zustand „Sitzungs-Token abgelehnt"](/docs/de/errors#claude-ai-rejected-the-session-token) an.
* Für einen Server, dessen `Authorization`-Header Sie konfiguriert haben, in `headers` oder über einen [`headersHelper`](#use-dynamic-headers-for-custom-authentication), kennzeichnet ein `401` oder `403` beim Verbinden den Server nicht, da die Anmeldedaten, die behoben werden müssen, diejenigen sind, die Sie konfiguriert haben. Claude Code meldet die Verbindung stattdessen als fehlgeschlagen. Wenn Sie diesen Header aus einer `${VAR}`-Referenz gesetzt haben, überprüfen Sie, ob diese Variable eine ist, die Claude Code [als leer liest](#credential-variables-that-read-as-empty).
* Für einen Connector, der [an eine Cloud-Sitzung übermittelt wird](#how-connectors-reach-claude-code), führt Claude Code keinen Anmeldungsfluss aus, da die Proxy der Sitzung sich beim Connector mit der Autorisierung authentifiziert, die Sie in claude.ai gewährt haben. Wenn ein Connector dort erneut autorisiert werden muss, verbinden Sie ihn erneut unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors), anstatt aus der Sitzung.

Wenn eine Anfrage an einen OAuth-Server, bei dem Sie bereits angemeldet sind, `401 Unauthorized` zurückgibt, aktualisiert Claude Code das gespeicherte Token, verbindet sich erneut und wiederholt die Anfrage einmal. Der Server wird in `/mcp` nur gekennzeichnet, wenn dieser Wiederholungsversuch auch fehlschlägt. Vor v2.1.206 kennzeichnete eine Token-Aktualisierung, die aus einem vorübergehenden Grund fehlschlug, wie z. B. ein Netzwerkfehler, einen OAuth-Server als authentifizierungsbedürftig für den Rest der Sitzung, obwohl sein Refresh-Token noch gültig war.

Wenn der Server das gespeicherte Refresh-Token ablehnt, zeigt Claude Code sofort einen Hinweis an, der auf `/mcp` verweist. Öffnen Sie `/mcp` und wählen Sie **Re-authenticate** auf dem Server, um sich erneut anzumelden, bevor der nächste Tool-Aufruf fehlschlägt.

Ein benutzerdefinierter Server, der einen `WWW-Authenticate`-Header zurückgibt, der auf seinen Autorisierungsserver verweist, erhält die gleiche automatische Erkennung wie jeder andere Remote-Server.

Claude Code zeigt auch einen Starthinweis an, wenn ein oder mehrere konfigurierte Server Authentifizierung benötigen, sodass Sie `/mcp` nicht öffnen müssen, um zu erkennen, welche Server Anmeldung benötigen. Der Hinweis erfordert Claude Code v2.1.193 oder später. Er zählt nur Server, bei denen Sie sich von Claude Code aus anmelden können. Vor v2.1.218 zählte er auch [claude.ai-Connectors](#use-mcp-servers-from-claude-ai), die nicht in claude.ai verbunden waren, die Sie nur aus den claude.ai-Einstellungen verbinden können.

Der Hinweis kündigt jeden Server einmal an und lässt ihn aus der Zählung bei späteren Starts aus, bis dieser Server verbunden ist und erneut Anmeldung benötigt. `/mcp` listet immer noch jeden Server auf, der Anmeldung benötigt.

Im nicht-interaktiven Modus gibt es kein `/mcp`-Panel, daher kann Claude Code den OAuth-Fluss nicht für Sie ausführen. Ab v2.1.196 teilt Claude Code Claude mit, dass die Tools des Servers nicht verfügbar sind, bis Sie ihn autorisieren, wenn ein konfigurierter Server während eines `claude -p`- oder Agent SDK-Laufs mit aktivierter [Tool-Suche](#scale-with-mcp-tool-search) (Standard) Authentifizierung benötigt. Claude kann dann den Server benennen, der Anmeldung benötigt, anstatt so zu reagieren, als wäre der Server nicht konfiguriert. Schließen Sie die Anmeldung aus einer interaktiven Sitzung mit `/mcp` oder `claude mcp login <name>` ab.

Wenn Sie `headers.Authorization` für den Server konfiguriert haben und der Server diesen Header ablehnt, meldet Claude Code die Verbindung als fehlgeschlagen, anstatt auf OAuth zurückzugreifen. Überprüfen Sie, dass das Token für den MCP-Endpunkt gültig ist, oder entfernen Sie den Header, um den OAuth-Fluss zu verwenden.

<Steps>
  <Step title="Fügen Sie den Server hinzu, der Authentifizierung erfordert">
    Wenn Sie den `sentry`-Server bereits in der [MCP-Schnellstartanleitung](/docs/de/mcp-quickstart#connect-a-server-that-requires-sign-in) hinzugefügt haben, überspringen Sie diesen Schritt: Das Ausführen von `claude mcp add` erneut mit dem gleichen Servernamen im gleichen Bereich schlägt mit `MCP server sentry already exists in local config` fehl. Führen Sie andernfalls aus:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Verwenden Sie den /mcp-Befehl innerhalb von Claude Code">
    In Claude Code verwenden Sie den Befehl:

    ```text wrap theme={null}
    /mcp
    ```

    Folgen Sie dann den Schritten in Ihrem Browser, um sich anzumelden.
  </Step>
</Steps>

<Tip>
  Tipps:

  * Authentifizierungstoken werden sicher gespeichert und automatisch aktualisiert
  * Verwenden Sie „Clear authentication" im `/mcp`-Menü, um den Zugriff zu widerrufen
  * Wenn Ihr Browser nicht automatisch geöffnet wird, kopieren Sie die bereitgestellte URL und öffnen Sie sie manuell
  * Wenn die Browser-Umleitung nach der Authentifizierung mit einem Verbindungsfehler fehlschlägt, fügen Sie die vollständige Callback-URL aus der Adressleiste Ihres Browsers in die URL-Eingabeaufforderung ein, die in Claude Code angezeigt wird
  * OAuth-Authentifizierung funktioniert mit HTTP-Servern
</Tip>

<h3 id="authenticate-from-the-command-line">
  Authentifizieren Sie sich über die Befehlszeile
</h3>

Der Befehl `claude mcp login <name>` führt den OAuth-Fluss eines konfigurierten Servers direkt aus Ihrer Shell aus, sodass Sie das `/mcp`-Panel nicht innerhalb einer Sitzung öffnen müssen.

```bash theme={null}
claude mcp login sentry
```

Um gespeicherte Anmeldedaten später zu löschen, führen Sie `claude mcp logout <name>` aus.

`claude mcp login` erkennt, wenn kein lokaler Browser verfügbar ist, z. B. während einer SSH-Sitzung oder unter Linux ohne Display-Server, und gibt die Autorisierungs-URL aus, anstatt zu versuchen, einen Browser zu öffnen. Öffnen Sie die URL auf Ihrem lokalen Computer und fügen Sie dann die vollständige Umleitungs-URL aus der Adressleiste Ihres Browsers an der Eingabeaufforderung ein. Der Befehl benötigt ein interaktives Terminal für den Einfügungsschritt, daher verbinden Sie sich mit `ssh -t`. Übergeben Sie `--no-browser`, um die URL-Eingabeaufforderung zu erzwingen, auch wenn ein lokaler Browser erkannt wird.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Verwenden Sie einen festen OAuth-Callback-Port
</h3>

Einige MCP-Server erfordern einen spezifischen Redirect-URI, der im Voraus registriert ist. Standardmäßig wählt Claude Code einen zufällig verfügbaren Port für den OAuth-Callback. Verwenden Sie `--callback-port`, um den Port zu fixieren, damit er einem vorregistrierten Redirect-URI der Form `http://localhost:PORT/callback` entspricht. Wenn die Anmeldung auf Claude Code v2.1.229 mit einem Redirect-URI-Mismatch fehlschlägt, siehe die Versionsnote unter [Verwenden Sie vorkonfigurierte OAuth-Anmeldedaten](#use-pre-configured-oauth-credentials).

Sie können `--callback-port` allein (mit dynamischer Client-Registrierung) oder zusammen mit `--client-id` (mit vorkonfigurierten Anmeldedaten) verwenden.

```bash theme={null}
# Fester Callback-Port mit dynamischer Client-Registrierung
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  Verwenden Sie vorkonfigurierte OAuth-Anmeldedaten
</h3>

Einige MCP-Server unterstützen keine automatische OAuth-Einrichtung über Dynamic Client Registration. Wenn Sie einen Fehler wie „Incompatible auth server: does not support dynamic client registration" sehen, erfordert der Server vorkonfigurierte Anmeldedaten. Claude Code unterstützt auch Server, die ein Client ID Metadata Document (CIMD) anstelle von Dynamic Client Registration verwenden, und erkennt diese automatisch. Wenn die automatische Erkennung fehlschlägt, registrieren Sie zunächst eine OAuth-App über das Entwicklerportal des Servers und geben Sie dann die Anmeldedaten beim Hinzufügen des Servers an.

<Steps>
  <Step title="Registrieren Sie eine OAuth-App beim Server">
    Erstellen Sie eine App über das Entwicklerportal des Servers und notieren Sie sich Ihre Client-ID und Ihren Client-Secret.

    Viele Server erfordern auch einen Redirect-URI. Wenn ja, wählen Sie einen Port und registrieren Sie einen Redirect-URI im Format `http://localhost:PORT/callback`. Verwenden Sie denselben Port mit `--callback-port` im nächsten Schritt.

    In v2.1.229 sendete Claude Code `http://127.0.0.1:PORT/callback` stattdessen, und Server, die den registrierten Redirect-URI exakt abgleichen, lehnten die Anmeldung mit einem Redirect-URI-Mismatch ab. Claude Code v2.1.231 stellte die `localhost`-Form wieder her. Um auf v2.1.229 zu beheben, aktualisieren Sie Claude Code, oder fügen Sie vorübergehend die `http://127.0.0.1:PORT/callback`-Form zu den registrierten Redirect-URIs des Servers hinzu.
  </Step>

  <Step title="Fügen Sie den Server mit Ihren Anmeldedaten hinzu">
    Wählen Sie eine der folgenden Methoden. Der für `--callback-port` verwendete Port kann ein beliebiger verfügbarer Port sein. Er muss dem Redirect-URI entsprechen, den Sie im vorherigen Schritt registriert haben.

    <Tabs>
      <Tab title="claude mcp add">
        Verwenden Sie `--client-id`, um die Client-ID Ihrer App zu übergeben. Das Flag `--client-secret` fordert das Secret mit maskierter Eingabe an:

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        Fügen Sie das Objekt `oauth` in die JSON-Konfiguration ein und übergeben Sie `--client-secret` als separates Flag:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json (nur Callback-Port)">
        Verwenden Sie `--callback-port` ohne Client-ID, um den Port zu fixieren und gleichzeitig die dynamische Client-Registrierung zu verwenden:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / Umgebungsvariable">
        Legen Sie das Secret über eine Umgebungsvariable fest, um die interaktive Eingabeaufforderung zu überspringen:

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Authentifizieren Sie sich in Claude Code">
    Führen Sie `/mcp` in Claude Code aus und folgen Sie dem Browser-Login-Ablauf.
  </Step>
</Steps>

<Tip>
  Tipps:

  * Das Client-Secret wird sicher in Ihrem System-Keychain (macOS) oder einer Anmeldedatei gespeichert, nicht in Ihrer Konfiguration
  * Sie können das Client-Secret nur beim Hinzufügen des Servers festlegen. Wenn Sie sich mit `claude mcp login` oder von `/mcp` authentifizieren, verwendet Claude Code das gespeicherte Secret und fordert nicht auf oder liest `MCP_CLIENT_SECRET`
  * Um das Secret später hinzuzufügen oder zu ändern, entfernen Sie den Server mit `claude mcp remove <name>` und fügen Sie ihn dann erneut mit `--client-secret` und dem gleichen `--scope` hinzu
  * Wenn der Server einen öffentlichen OAuth-Client ohne Secret verwendet, verwenden Sie nur `--client-id` ohne `--client-secret`
  * Diese Flags gelten nur für HTTP- und SSE-Transporte. Sie haben keine Auswirkung auf Stdio-Server
  * Verwenden Sie `claude mcp get <name>`, um zu überprüfen, dass OAuth-Anmeldedaten für einen Server konfiguriert sind
</Tip>

<h3 id="override-oauth-metadata-discovery">
  Überschreiben Sie die OAuth-Metadaten-Erkennung
</h3>

Verweisen Sie Claude Code auf eine spezifische OAuth-Autorisierungsserver-Metadaten-URL, um die Standard-Erkennungskette zu umgehen. Legen Sie `authServerMetadataUrl` fest, wenn die Standard-Endpunkte des MCP-Servers Fehler zurückgeben, oder wenn Sie die Erkennung durch einen internen Proxy leiten möchten. Standardmäßig überprüft Claude Code zunächst RFC 9728 Protected Resource Metadata unter `/.well-known/oauth-protected-resource` und fällt dann auf RFC 8414 Authorization Server Metadata unter `/.well-known/oauth-authorization-server` zurück.

Legen Sie `authServerMetadataUrl` im Objekt `oauth` der Konfiguration Ihres Servers in `.mcp.json` fest:

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

Die URL muss `https://` verwenden. Die `scopes_supported` der Metadaten-URL überschreiben die Bereiche, die der Upstream-Server bewirbt.

<h3 id="restrict-oauth-scopes">
  Beschränken Sie OAuth-Bereiche
</h3>

Legen Sie `oauth.scopes` fest, um die Bereiche zu fixieren, die Claude Code während des Autorisierungsflusses anfordert. Dies ist die unterstützte Methode, um einen MCP-Server auf eine von Ihrem Sicherheitsteam genehmigte Teilmenge zu beschränken, wenn der Upstream-Autorisierungsserver mehr Bereiche bewirbt, als Sie gewähren möchten. Der Wert ist eine einzelne durch Leerzeichen getrennte Zeichenkette, die dem `scope`-Parameter-Format in RFC 6749 §3.3 entspricht.

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` hat Vorrang vor sowohl `authServerMetadataUrl` als auch den Bereichen, die der Server unter `/.well-known` entdeckt. Lassen Sie es ungesetzt, damit der MCP-Server den angeforderten Bereichssatz bestimmt.

Ab v2.1.196 fordert Claude Code, wenn `oauth.scopes` nicht gesetzt ist, den Bereich an, der vom `WWW-Authenticate`-Header des Servers oder seinen Protected Resource Metadata bereitgestellt wird, und sendet keinen `scope`-Parameter, wenn keiner von beiden einen bereitstellt. Es fordert nicht mehr den vollständigen `scopes_supported`-Katalog aus automatisch erkannten Authorization Server Metadata an. Das Anfordern dieses Katalogs führte dazu, dass Identity Provider, die Admin-only- oder Template-Bereiche bewerben, die Autorisierungsanfrage mit einem `invalid_scope`-Fehler ablehnten. Metadaten, die von einer konfigurierten `authServerMetadataUrl` abgerufen werden, liefern immer noch ihre `scopes_supported` als die angeforderten Bereiche.

Wenn der Autorisierungsserver `offline_access` in `scopes_supported` bewirbt, fügt Claude Code es zu den fixierten Bereichen hinzu, damit das Zugriffs-Token ohne neue Browser-Anmeldung aktualisiert werden kann.

Wenn der Server später einen 403 `insufficient_scope` für einen Tool-Aufruf zurückgibt, schlägt der Aufruf mit einer [`needs additional permissions`](/docs/de/errors#mcp-server-needs-you-to-sign-in-again)-Nachricht fehl, die den Bereich benennt, den der Server anfordert. Der Server wird als authentifizierungsbedürftig in `/mcp` angezeigt.

Wenn dieser Bereich nicht in Ihrem fixierten `oauth.scopes` enthalten ist, fügen Sie ihn hinzu, führen Sie dann `/mcp` aus und authentifizieren Sie den Server erneut. Claude Code fordert die fixierten Bereiche an, nicht den Bereich, den der Server benannt hat, daher wenn Sie sich erneut authentifizieren, ohne ihn hinzuzufügen, hat das Token, das Sie erhalten, immer noch nicht.

<h3 id="use-dynamic-headers-for-custom-authentication">
  Verwenden Sie dynamische Header für benutzerdefinierte Authentifizierung
</h3>

Wenn Ihr MCP-Server ein anderes Authentifizierungsschema verwendet als OAuth (wie Kerberos, kurzlebige Token oder ein internes SSO), verwenden Sie `headersHelper`, um Request-Header zur Verbindungszeit zu generieren. Claude Code führt den Befehl aus und fügt seine Ausgabe in die Verbindungs-Header ein.

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

Der Befehl kann auch inline sein:

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**Anforderungen:**

* Der Befehl muss ein JSON-Objekt mit String-Schlüssel-Wert-Paaren auf stdout schreiben
* Claude Code führt den Befehl in einer Shell aus und gibt ihn nach 10 Sekunden auf
* Claude Code wählt das Arbeitsverzeichnis des Befehls nach [wo Sie den Server konfiguriert haben](#where-the-helper-runs), daher geben Sie das Skript als absoluten Pfad an oder setzen Sie es auf `PATH`
* Dynamische Header überschreiben alle statischen `headers` mit dem gleichen Namen

Claude Code führt den Helper bei jeder Verbindung neu aus, beim Sitzungsstart und bei Wiederverbindung, sobald die [Vertrauensregel für Projekt- und Local-Scope-Server](#trust-a-folder-before-its-headershelper-runs) es ausführen lässt. Es speichert das Ergebnis nicht zwischen, daher ist Ihr Skript für jede Token-Wiederverwendung verantwortlich.

Wenn ein Tool-Aufruf `401 Unauthorized` oder `403 Forbidden` zurückgibt, führt Claude Code den Helper automatisch erneut unter der gleichen Regel aus, verbindet sich mit den neuen Headern erneut und wiederholt den Aufruf einmal. Claude Code markiert den Server als authentifizierungsbedürftig in `/mcp` nur, wenn dieser Wiederholungsversuch auch fehlschlägt.

Wenn die Ausgabe des Helpers einen `Authorization`-Header enthält, verwendet Claude Code diese Anmeldedaten als Authentifizierung des Servers und fällt nicht auf OAuth für den Server zurück.

Wenn der Server die Anmeldedaten des Helpers beim Verbinden ablehnt, meldet Claude Code die Verbindung als fehlgeschlagen, anstatt den Server als authentifizierungsbedürftig zu markieren. Beheben Sie die Anmeldedaten, die Ihr Helper zurückgibt, und verbinden Sie sich dann von `/mcp` erneut, um den Helper erneut auszuführen.

Claude Code setzt diese Umgebungsvariablen beim Ausführen des Helpers:

| Variable                      | Wert                                                                                                                              |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | der Name des MCP-Servers                                                                                                          |
| `CLAUDE_CODE_MCP_SERVER_URL`  | die URL des MCP-Servers                                                                                                           |
| `CLAUDE_PLUGIN_ROOT`          | das Root-Verzeichnis des Plugins. Wird nur gesetzt, wenn ein [Plugin](/docs/de/plugins/components#mcp-servers) den Server bereitstellt |

Verwenden Sie diese, um ein einzelnes Helper-Skript zu schreiben, das mehrere MCP-Server bedient.

Ein von einem Plugin bereitgestellter `headersHelper` kann nicht auf die [`${user_config.*}`](/docs/de/plugins/manifest-reference#user-configuration)-Werte des Plugins verweisen, da der Befehl durch eine Shell ausgeführt wird. Claude Code meldet den Server als fehlkonfiguriert mit einem [Fehler](/docs/de/errors#plugin-command-references-user-config) und ersetzt den Wert nicht. Setzen Sie `${user_config.KEY}` stattdessen in das Feld `headers` des Servers, das nicht shell-geparst wird, oder lassen Sie das Helper-Skript den Wert aus einer Konfigurationsdatei lesen. Vor v2.1.207 ersetzte `headersHelper` `${user_config.*}`-Werte.

<h4 id="where-the-helper-runs">
  Wo der Helper ausgeführt wird
</h4>

Claude Code wählt das Arbeitsverzeichnis des `headersHelper`-Befehls aus der Konfiguration, die den Server deklariert. Ein `cd`, das Claude in Bash ausführt, verschiebt es nicht, und [`/cd`](/docs/de/permissions#move-the-session-to-another-directory) verschiebt es nur für Server, die aus dem primären Arbeitsverzeichnis der Sitzung ausgeführt werden. Jede Zeile unten gibt das Verzeichnis an, gegen das ein relativer Pfad in Ihrem `headersHelper`-Befehl aufgelöst wird.

| Wo Sie den Server konfiguriert haben                                                                                                                                                                                                  | Arbeitsverzeichnis                                                                                            |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------ |
| Ein [Plugin](/docs/de/plugins/components#mcp-servers)                                                                                                                                                                                      | Das Root-Verzeichnis des Plugins. Erfordert Claude Code v2.1.195 oder später                                  |
| Eine Projekt `.mcp.json` oder ein [Local-Scope](#local-scope)-Server                                                                                                                                                                  | Das Projektverzeichnis, in dem der Server deklariert ist                                                      |
| Eine Agent-Datei in Ihrem Projekt, ein Server aus der SDK-Option `mcpServers` oder der Methode `setMcpServers()`, oder [`--mcp-config`](/docs/de/cli-reference)                                                                            | Das [primäre Arbeitsverzeichnis](/docs/de/permissions#working-directories) der Sitzung                             |
| [User Scope](#user-scope), [verwaltetes MCP](/docs/de/managed-mcp), ein [claude.ai-Connector](#use-mcp-servers-from-claude-ai), oder eine Agent-Datei von außerhalb Ihres Projekts, einschließlich einer aus einem `--add-dir`-Verzeichnis | Ihr Konfigurationsverzeichnis, `~/.claude` sofern Sie nicht [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) gesetzt haben |

Vor v2.1.238 führte Claude Code auch die Helper von User-Scope-, verwalteten und claude.ai-Connector-Servern sowie von Agent-Dateien von außerhalb Ihres Projekts aus dem Verzeichnis aus, in dem Sie es gestartet haben.

<h4 id="which-variables-a-helper-can-read">
  Welche Variablen ein Helper lesen kann
</h4>

Ein `headersHelper`, den ein Repository oder Plugin bereitstellt, ist ein Befehl, den Sie nicht geschrieben haben, daher führt Claude Code ihn ohne die Anmeldedaten-Variablen aus Ihrer Umgebung aus, wie z. B. `ANTHROPIC_API_KEY`. Wo Sie den Server konfiguriert haben, entscheidet, ob dies zutrifft:

* **Entfernt**: ein Server in einer Projekt `.mcp.json` oder in einem Plugin, und ein Inline-Server in einer Agent-Datei aus Ihrem Projekt oder aus einem `--add-dir`-Verzeichnis
* **Nicht entfernt**: ein Server bei [User Scope](#user-scope) oder [Local Scope](#local-scope), in [verwaltetes MCP](/docs/de/managed-mcp), von einem [claude.ai-Connector](#use-mcp-servers-from-claude-ai), oder bereitgestellt von der SDK oder [`--mcp-config`](/docs/de/cli-reference), und ein Inline-Server in einer Agent-Datei aus `~/.claude/agents/`, aus verwalteten Einstellungen, oder übergeben mit `--agents`

Abgesehen von Gits `GIT_CONFIG_KEY_<n>`-Variablen entfernt Claude Code jede Variable aus Ihrer Umgebung, deren Name wie eine Anmeldedaten aussieht, wie z. B. ein Name mit `TOKEN`, `SECRET`, `PASSWORD`, `KEY` oder `AUTH` darin in beiden Buchstabenfällen, daher werden sowohl `ANTHROPIC_API_KEY` als auch `MY_REGISTRY_TOKEN` entfernt. Claude Code entfernt auch eine feste Liste von Anmeldedaten-Variablen, deren Namen diesem Muster nicht folgen, wie z. B. `ANTHROPIC_CUSTOM_HEADERS`.

Wenn dies auf Ihren Helper zutrifft, lassen Sie das Skript seine Anmeldedaten aus einer Datei oder einem Anmeldedaten-Speicher lesen. Wenn die `url` des Servers [eine dieser Variablen erweitert](#environment-variable-expansion-in-mcp-json), hat der `CLAUDE_CODE_MCP_SERVER_URL`-Wert, den der Helper erhält, diesen Teil durch `REDACTED` ersetzt.

<h4 id="trust-a-folder-before-its-headershelper-runs">
  Vertrauen Sie einem Ordner, bevor sein headersHelper ausgeführt wird
</h4>

Claude Code führt einen `headersHelper` als beliebigen Shell-Befehl aus. Für einen Server in einer Projekt `.mcp.json` oder bei [Local Scope](#local-scope) führt es den Helper nur aus, nachdem Sie den [Vertrauensdialog](/docs/de/permissions#project-allow-rules-and-workspace-trust) für das Projektverzeichnis akzeptiert haben, in dem der Server deklariert ist. Vor v2.1.238 führte eine `claude -p`- oder SDK-Sitzung diese Helper ohne Vertrauensprüfung aus, und eine interaktive Sitzung führte sie aus, sobald Sie einen übergeordneten Ordner vertraut hatten.

* **Vertrauen, das nicht zählt**: das Vertrauen eines übergeordneten Ordners und das automatische Vertrauen, das eine `claude -p`- oder SDK-Sitzung für [Hooks in Einstellungsdateien](/docs/de/permissions#what-runs-before-you-trust-a-folder) erhält
* **Bis Sie den Ordner vertrauen**: Claude Code verbindet den Server nur mit seinen statischen `headers`. In einer `claude -p`- oder SDK-Sitzung druckt es auch eine [`headersHelper not run`](/docs/de/errors#headershelper-not-run)-Zeile pro Server auf stderr, die Ihnen sagt, wie Sie das Vertrauen gewähren.
* **Vertrauen ohne Dialog**: setzen Sie `projects["<path>"].hasTrustDialogAccepted` auf `true` in `~/.claude.json`. `<path>` ist der Ordner, auf den [Project allow rules and workspace trust](/docs/de/permissions#project-allow-rules-and-workspace-trust) sagt, dass Claude Code das Vertrauen basiert.

Claude Code wendet die gleiche Regel auf einen Server an, der inline in einer [Agent-Datei](/docs/de/sub-agents#scope-mcp-servers-to-a-subagent) deklariert ist, und überprüft, woher diese Agent-Datei kam: Ihr Projekt, für eine Datei in seinem `.claude/agents/`-Verzeichnis, oder ein `--add-dir`-Verzeichnis. Bis Sie [dieses Projekt oder Verzeichnis selbst vertrauen](/docs/de/permissions#what-runs-before-you-trust-a-folder), lädt Claude Code den Server überhaupt nicht, daher wird sein Helper auch nie ausgeführt.

<h2 id="add-mcp-servers-from-json-configuration">
  MCP-Server aus JSON-Konfiguration hinzufügen
</h2>

Wenn Sie eine JSON-Konfiguration für einen MCP-Server haben, können Sie sie direkt hinzufügen:

<Steps>
  <Step title="Fügen Sie einen MCP-Server aus JSON hinzu">
    ```bash theme={null}
    # Grundlegende Syntax
    claude mcp add-json <name> '<json>'

    # Beispiel: Hinzufügen eines HTTP-Servers mit JSON-Konfiguration
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # Beispiel: Hinzufügen eines Stdio-Servers mit JSON-Konfiguration
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # Beispiel: Hinzufügen eines HTTP-Servers mit vorkonfigurierten OAuth-Anmeldedaten
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="Überprüfen Sie, dass der Server hinzugefügt wurde">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  Tipps:

  * Stellen Sie sicher, dass das JSON in Ihrer Shell ordnungsgemäß escaped ist
  * Das JSON muss dem MCP-Server-Konfigurationsschema entsprechen
  * Sie können `--scope user` verwenden, um den Server zu Ihrer Benutzerkonfiguration statt zur projektspezifischen hinzuzufügen
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  MCP-Server aus Claude Desktop importieren
</h2>

Wenn Sie bereits MCP-Server in Claude Desktop konfiguriert haben, können Sie diese importieren:

<Steps>
  <Step title="Importieren Sie Server aus Claude Desktop">
    ```bash theme={null}
    # Grundlegende Syntax 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="Wählen Sie aus, welche Server importiert werden sollen">
    Nach dem Ausführen des Befehls wird ein interaktives Dialogfeld angezeigt, in dem Sie auswählen können, welche Server Sie importieren möchten.
  </Step>

  <Step title="Überprüfen Sie, dass die Server importiert wurden">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

Servernamen, die über `claude mcp`-Befehle hinzugefügt werden, dürfen nur Buchstaben, Zahlen, Bindestriche und Unterstriche enthalten. Claude Desktop wendet diese Einschränkung nicht an, daher kann ein Claude Desktop-Server, dessen Name ein anderes Zeichen wie ein Leerzeichen enthält, nicht importiert werden. Der Import meldet jeden Namen, den er ablehnt, und importiert weiterhin die anderen Server, die Sie ausgewählt haben. Vor v2.1.205 stoppte der erste ungültige Name den Import und keiner der ausgewählten Server wurde hinzugefügt.

<Tip>
  Tipps:

  * Diese Funktion funktioniert nur auf macOS und Windows Subsystem for Linux (WSL)
  * Sie liest die Claude Desktop-Konfigurationsdatei von ihrem Standardort auf diesen Plattformen
  * Verwenden Sie das Flag `--scope user`, um Server zu Ihrer Benutzerkonfiguration hinzuzufügen
  * Importierte Server behalten die gleichen Namen wie in Claude Desktop, wenn der Name nur Buchstaben, Zahlen, Bindestriche und Unterstriche enthält. Claude Code meldet einen Server, dessen Name ein anderes Zeichen enthält, und überspringt ihn
  * Wenn Server mit den gleichen Namen bereits vorhanden sind, erhalten sie ein numerisches Suffix (zum Beispiel `server_1`)
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  MCP-Server von claude.ai verwenden
</h2>

Wenn Sie sich bei Claude Code mit einem [claude.ai](https://claude.ai)-Konto angemeldet haben, sind MCP-Server, die Sie in claude.ai hinzugefügt haben, bekannt als [Connectors](https://claude.com/docs/connectors), automatisch in Claude Code verfügbar:

<Steps>
  <Step title="MCP-Server in claude.ai konfigurieren">
    Fügen Sie Server unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors) hinzu. Bei Team- und Enterprise-Plänen können nur Administratoren Server hinzufügen.
  </Step>

  <Step title="MCP-Server authentifizieren">
    Führen Sie alle erforderlichen Authentifizierungsschritte in claude.ai durch.
  </Step>

  <Step title="Server in Claude Code anzeigen und verwalten">
    Verwenden Sie in Claude Code den Befehl:

    ```text wrap theme={null}
    /mcp
    ```

    Server von claude.ai werden in der Liste mit Indikatoren angezeigt, die zeigen, dass sie von claude.ai stammen.
  </Step>
</Steps>

Claude Code markiert einen Connector als `managed` in `/mcp` und im [`/plugin`](/docs/de/plugins/install)-Manager, wenn Ihre Organisation dessen Authentifizierung in claude.ai verwaltet. Der Status „Managed" ändert nicht, wie Claude Code sich mit dem Connector verbindet oder wie die [Tool-Kontrollen](#organization-controls-on-connector-tools) Ihrer Organisation angewendet werden.

Connectors, bei denen Sie sich noch nie angemeldet haben, werden hinter einer Zeile `Show unused connectors` am Ende des claude.ai-Abschnitts ausgeblendet, damit eine von der Organisation bereitgestellte Liste das Panel nicht ausfüllt. Wählen Sie die Zeile aus, um sie zu erweitern. Ein Connector, bei dem Sie sich zuvor angemeldet haben, bleibt sichtbar, auch wenn er derzeit eine erneute Authentifizierung benötigt.

Connectors von claude.ai werden nur abgerufen, wenn Ihre aktive [Authentifizierungsmethode](/docs/de/authentication#authentication-precedence) ein claude.ai-Abonnement-Login ist. Sie werden nicht geladen, auch wenn Sie zuvor `/login` ausgeführt haben, wenn:

* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper` aktiv ist
* Ein Drittanbieter wie Amazon Bedrock oder Google Cloud's Agent Platform aktiv ist
* `ANTHROPIC_PROFILE`, die Verbundfariablen oder ein aktives [Anthropic-Profil](/docs/de/authentication#anthropic-profiles-and-federation-credentials) die Anmeldedaten bereitstellt
* `CLAUDE_CODE_OAUTH_TOKEN` ein Token von [`claude setup-token`](/docs/de/authentication#generate-a-long-lived-token) enthält, das nur Modellanfragen stellen kann

Wenn `/mcp` einen Connector, den Sie hinzugefügt haben, nicht auflistet, führen Sie `/status` aus, um zu bestätigen, welche Authentifizierungsmethode aktiv ist. Heben Sie diese Umgebungsvariable auf, entfernen Sie die `apiKeyHelper`-Einstellung oder [schalten Sie das Profil aus](/docs/de/authentication#anthropic-profiles-and-federation-credentials), und führen Sie dann `/login` aus, um Ihr claude.ai-Konto auszuwählen.

Wenn ein temporäres Netzwerkproblem verhindert, dass die Connector-Liste beim Start Ihrer Sitzung geladen wird, versucht Claude Code den Abruf bis zu dreimal im Hintergrund erneut, und die Connectors werden angezeigt, sobald ein erneuter Versuch erfolgreich ist. Wenn sie immer noch nicht angezeigt werden, starten Sie Claude Code neu, um die Liste erneut abzurufen.

Wenn `/mcp` einen Connector als `connected · session token rejected` anzeigt oder seine Detailansicht [`claude.ai rejected the session token`](/docs/de/errors#claude-ai-rejected-the-session-token) anzeigt, hat claude.ai das Token aus Ihrem Claude Code-Login abgelehnt, normalerweise weil das Login abgelaufen ist und nicht aktualisiert werden konnte. Die erneute Autorisierung des Connectors löscht diesen Status nicht, da die eigene Autorisierung des Connectors in claude.ai nicht das ist, was abgelehnt wurde. Um dies zu beheben:

1. Führen Sie `/login` aus, um sich erneut anzumelden.
2. Verbinden Sie den Connector erneut von `/mcp`.

Vor v2.1.222 markierte Claude Code Connectors stattdessen als authentifizierungsbedürftig, und deren Autorisierung löste das Problem nicht.

Ein Server, den Sie in Claude Code hinzugefügt haben, hat [Vorrang](#scope-hierarchy-and-precedence) vor einem claude.ai-Connector, der auf dieselbe URL verweist. In diesem Fall listet `/mcp` den Connector als verborgen auf und zeigt, wie Sie das Duplikat entfernen können, wenn Sie lieber den Connector verwenden möchten.

Einige von Anthropic gehostete Connectors wie Microsoft 365, Gmail und Google Calendar unterstützen kein lokales OAuth von Claude Code, da der vorgelagerte Identitätsanbieter nur die Umleitungs-URL akzeptiert, die claude.ai registriert hat. Wenn ein Server, den Sie mit `claude mcp add` oder in `.mcp.json` hinzugefügt haben, auf einen dieser Hosts verweist und Sie sich von `/mcp` oder mit `claude mcp login` darin anmelden, zeigt Claude Code [`is Anthropic-hosted and doesn't support local OAuth`](/docs/de/errors#anthropic-hosted-and-doesnt-support-local-oauth) an und leitet Sie stattdessen dazu, den Dienst unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors) zu verbinden.

Nachdem Sie Ihren Eintrag mit `claude mcp remove <name>` entfernt und den Dienst auf claude.ai verbunden haben, wird der Connector automatisch in Claude Code angezeigt.

<h3 id="how-connectors-reach-claude-code">
  Wie Connectors Claude Code erreichen
</h3>

Welche Einstellungen einen claude.ai-Connector steuern, hängt davon ab, wo Ihre Sitzung ausgeführt wird, da nur einige Sitzungen Connectors selbst von claude.ai abrufen. Jede Zeile unten benennt, wie Connectors in einer Art von Sitzung ankommen und was sie dort steuert. Die [WSL-Sitzungen](/docs/de/desktop-wsl#what-works-in-a-wsl-session) der Desktop-App haben keine Zeile, da Connectors darin noch nicht verfügbar sind.

| Wo die Sitzung ausgeführt wird                                                                                                | Wie Connectors ankommen                   | Was steuert sie                                                                                                                                                                                                                                       |
| :---------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Terminal-, [VS Code](/docs/de/vs-code)-, [JetBrains](/docs/de/jetbrains)- und [Agent SDK](/docs/de/agent-sdk/claude-code-features)-Sitzungen | Claude Code ruft sie von claude.ai ab     | Die Einstellungen in diesem Abschnitt und [verwaltete MCP-Konfiguration](/docs/de/managed-mcp)                                                                                                                                                             |
| [Cloud-Sitzungen](/docs/de/claude-code-on-the-web)                                                                                 | Der Remote-Host übergibt sie              | Ihre claude.ai-Organisationseinstellungen plus die [Allowlist- und Denylist](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists)-Einstellungen, die die Sitzung erreichen, und alle `managed-mcp.json` auf dem Host, der sie ausführt |
| Die [lokalen und SSH-Sitzungen](/docs/de/desktop) der Desktop-App                                                                  | Die Desktop-App liefert sie prozessintern | `blocked`-Einträge in den [Tool-Kontrollen](#organization-controls-on-connector-tools) Ihrer Organisation für Connectors                                                                                                                              |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors), `ENABLE_CLAUDEAI_MCP_SERVERS` und [`allowAllClaudeAiMcps`](/docs/de/settings-reference#allowallclaudeaimcps) wirken sich nur auf die erste Zeile aus, die Connectors, die Claude Code selbst abruft. Die anderen beiden Zeilen unterscheiden sich davon auf diese Weise:

* **Cloud-Sitzungen**: `allowedMcpServers`- und `deniedMcpServers`-Einträge, die die Sitzung erreichen, beispielsweise durch [Server-verwaltete Einstellungen](/docs/de/server-managed-settings), filtern auch die bereitgestellten Connectors. Der Proxy der Sitzung schreibt die URL jedes Connectors um, daher passt ein `serverUrl`-Muster, das für die eigene URL des Connectors geschrieben wurde, nicht dazu. Um bereitgestellte Connectors neben einer URL-Allowlist in einer selbstgehosteten Umgebung zuzulassen, fügen Sie die `serverUrl`-Einträge hinzu, die unter [Connector-Datenverkehr verlässt Ihr Netzwerk](/docs/de/self-hosted-environments-deploy#connector-traffic-leaves-your-network) aufgelistet sind. Claude Code verwirft die bereitgestellten Connectors, wenn eine `managed-mcp.json` auf dem Host vorhanden ist, der die Sitzung ausführt, z. B. ein [selbstgehosteter Runner-Host](/docs/de/self-hosted-environments-configuration#mcp-servers), unabhängig davon, ob Sie `allowAllClaudeAiMcps` setzen.
* **Desktop-App lokale und SSH-Sitzungen**: Die Desktop-App registriert die Connectors als prozessinterne `type: "sdk"`-Server, und keine MCP-Einstellung oder `managed-mcp.json` erreicht sie. Ein Benutzer hält einen Connector aus seinen eigenen Sitzungen heraus, indem er ihn unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors) trennt. Eine Organisation blockiert die [Tools](#organization-controls-on-connector-tools) eines Connectors oder schaltet [Claude Code in der Desktop-App](/docs/de/desktop#admin-console-controls) ganz aus.

<h3 id="organization-controls-on-connector-tools">
  Organisationskontrollen für Connector-Tools
</h3>

Ihre Organisation kann Pro-Tool-Kontrollen auf [claude.ai-Connectors](https://claude.com/docs/connectors) setzen. Claude Code liest diese Einstellungen beim Start und erzwingt sie lokal, außer in den [lokalen und SSH-Sitzungen](#how-connectors-reach-claude-code) der Desktop-App. Dort hält die Desktop-App `blocked`-Tools zurück, bevor sie einen Connector liefert, und die `ask`-Einstellung erreicht Claude Code nicht, daher wendet es die gewöhnlichen [Berechtigungsregeln](/docs/de/permissions) der Sitzung auf diese Tools an, anstatt bei jedem Aufruf zu fragen. In Sitzungen, in denen Claude Code Connectors selbst abruft, führen Sie `/mcp` aus, um zu sehen, welche Einstellung für jedes Tool auf einem Connector gilt.

* **Tool auf `ask` gesetzt**: Claude Code fragt bei jedem Aufruf mit dem Grund `Your organization requires approval for this tool` nach. Die Aufforderung wird auch in `acceptEdits`-, `auto`- und `bypassPermissions`-[Berechtigungsmodi](/docs/de/permissions#permission-modes) angezeigt und bietet niemals eine Option, Ihre Wahl zu merken. [Allow-Regeln](/docs/de/permissions), die das Tool abgleichen, überspringen die Aufforderung auch nicht. Im `dontAsk`-Modus, der niemals fragt, lehnt Claude Code den Aufruf stattdessen ab.
* **Tool auf `blocked` gesetzt**: Claude Code filtert das Tool heraus, bevor Claude es sieht, daher wird es nie in der Tool-Liste angezeigt. Die Desktop-App und der claude.ai-Chat wenden die gleiche `blocked`-Einstellung an, daher kann Claude das Tool dort auch nicht verwenden, und Sie können ein Tool nicht von den Sitzungen der Desktop-App fernhalten, während Sie es im Chat verfügbar halten. Die Desktop-App überspringt einen Connector, dessen Tools alle blockiert sind.

<h3 id="disable-claude-ai-connectors">
  claude.ai-Connectors deaktivieren
</h3>

Claude Code wendet [`disableClaudeAiConnectors`](/docs/de/settings-reference#disableclaudeaiconnectors) nur auf die Connectors an, die es [selbst abruft](#how-connectors-reach-claude-code), nicht auf die Connectors, die ein Cloud-Host oder die Desktop-App liefert. Um die Connectors auszuschalten, die es abruft, setzen Sie die Einstellung auf `true` in einem beliebigen Einstellungsbereich:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Diese Einstellung verwendet Any-Source-True-Semantik: `true` in einer beliebigen Einstellungsquelle hat Vorrang. Eine eingecheckte Projekt-`.claude/settings.json` kann ein Repository von den Connectors abmelden, die Claude Code selbst abruft, aber ein Projekt-Level-`false` kann Connectors nicht erneut aktivieren, die ein Benutzer- oder Policy-Level-`true` deaktiviert hat. Server, die explizit über `--mcp-config` übergeben werden, sind nicht betroffen.

Sie können auch die Umgebungsvariable `ENABLE_CLAUDEAI_MCP_SERVERS` auf `false` setzen, was den gleichen Effekt für die aktuelle Shell-Sitzung hat:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Um einzelne claude.ai-Connectors zu blockieren, anstatt alle zu blockieren, fügen Sie sie nach Name oder nach URL-Muster zu [`deniedMcpServers`](/docs/de/managed-mcp) hinzu. Beispielsweise blockiert ein `serverName`-Eintrag von `"claude.ai Slack"` den Slack-Connector. Sie können auch `/mcp` ausführen, um einen beliebigen Connector, den Claude Code abruft, für das aktuelle Projekt nur ein- oder auszuschalten.

<h2 id="use-claude-code-as-an-mcp-server">
  Claude Code als MCP-Server verwenden
</h2>

Sie können Claude Code selbst als MCP-Server verwenden, mit dem sich andere Anwendungen verbinden können:

```bash theme={null}
# Claude als stdio MCP-Server starten
claude mcp serve
```

Der Befehl gibt bei der Ausführung nichts aus. Ein stdio MCP-Server kommuniziert über stdin und stdout, daher bedeutet ein stilles, blockiertes Terminal, dass der Server läuft und auf einen Client wartet, der sich verbindet.

Sie können dies in Claude Desktop verwenden, indem Sie diese Konfiguration zu claude\_desktop\_config.json hinzufügen:

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **Konfigurieren des ausführbaren Pfads**: Das Feld `command` muss auf die Claude Code-Ausführungsdatei verweisen. Wenn der Befehl `claude` nicht in Ihrem System-PATH vorhanden ist, müssen Sie den vollständigen Pfad zur Ausführungsdatei angeben.

  So finden Sie den vollständigen Pfad:

  ```bash theme={null}
  which claude
  ```

  Verwenden Sie dann den vollständigen Pfad in Ihrer Konfiguration:

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  Ohne den korrekten Pfad zur Ausführungsdatei treten Fehler wie `spawn claude ENOENT` auf.
</Warning>

<Tip>
  Tipps:

  * Versuchen Sie in Claude Desktop, Claude zu bitten, Dateien in einem Verzeichnis zu lesen, Änderungen vorzunehmen und mehr.
  * Dieser MCP-Server stellt nur die Tools von Claude Code Ihrem MCP-Client zur Verfügung, daher ist Ihr eigener Client dafür verantwortlich, Benutzerbestätigungen für einzelne Tool-Aufrufe zu implementieren.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  MCP-Ausgabebegrenzungen und Warnungen
</h2>

Wenn MCP-Tools große Ausgaben erzeugen, hilft Claude Code dabei, die Token-Nutzung zu verwalten, um Ihren Gesprächskontext nicht zu überlasten:

* **Warnungsschwelle für Ausgaben**: Claude Code zeigt eine Warnung an, wenn eine MCP-Tool-Ausgabe 10.000 Token überschreitet
* **Konfigurierbare Begrenzung**: Sie können die maximale zulässige MCP-Ausgabe-Token-Menge mithilfe der Umgebungsvariablen `MAX_MCP_OUTPUT_TOKENS` anpassen
* **Standardbegrenzung**: Das Standardmaximum beträgt 25.000 Token
* **Geltungsbereich**: Die Umgebungsvariable gilt für Tools, die keine eigene Begrenzung deklarieren. Tools, die [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) setzen, verwenden diesen Wert stattdessen für Textinhalte, unabhängig davon, auf welchen Wert `MAX_MCP_OUTPUT_TOKENS` gesetzt ist. Tools, die Bilddaten zurückgeben, unterliegen weiterhin `MAX_MCP_OUTPUT_TOKENS`
* **Über der Begrenzung**: Wenn ein Ergebnis ohne Bildinhalt die Begrenzung überschreitet, speichert Claude Code es in einer Datei und ersetzt es im Gespräch durch eine Nachricht, die den Dateipfad benennt, damit Claude die Datei liest, wenn sie den Inhalt benötigt. Die Datei befindet sich im Verzeichnis `tool-results` der Sitzung unter [`~/.claude/projects/`](/docs/de/claude-directory#cleaned-up-automatically).

Um die Begrenzung für Tools zu erhöhen, die große Ausgaben erzeugen:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

<h3 id="raise-the-limit-for-a-specific-tool">
  Erhöhen Sie die Begrenzung für ein bestimmtes Tool
</h3>

Wenn Sie einen MCP-Server erstellen, können Sie einzelnen Tools ermöglichen, Ergebnisse zurückzugeben, die größer als der Standard-Persistierungs-Schwellenwert sind, indem Sie `_meta["anthropic/maxResultSizeChars"]` im Eintrag der `tools/list`-Antwort des Tools setzen. Claude Code erhöht den Schwellenwert dieses Tools auf den annotierten Wert, bis zu einer harten Obergrenze von 500.000 Zeichen.

Dies ist nützlich für Tools, die inhärent große, aber notwendige Ausgaben zurückgeben, wie Datenbankschemas oder vollständige Dateistrukturen. Ohne die Anmerkung werden Ergebnisse, die den Standardschwellenwert überschreiten, auf der Festplatte gespeichert und durch einen Dateiverweis im Gespräch ersetzt.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

Die Anmerkung gilt unabhängig von `MAX_MCP_OUTPUT_TOKENS` für Textinhalte, sodass Benutzer die Umgebungsvariable nicht für Tools erhöhen müssen, die sie deklarieren. Tools, die Bilddaten zurückgeben, unterliegen weiterhin der Token-Begrenzung.

<Warning>
  Wenn Sie häufig Ausgabewarnungen bei bestimmten MCP-Servern erhalten, die Sie nicht kontrollieren, sollten Sie die Begrenzung `MAX_MCP_OUTPUT_TOKENS` erhöhen. Sie können auch den Server-Autor bitten, die Anmerkung `anthropic/maxResultSizeChars` hinzuzufügen oder seine Antworten zu paginieren. Die Anmerkung hat keine Auswirkung auf Tools, die Bildinhalte zurückgeben; für diese ist das Erhöhen von `MAX_MCP_OUTPUT_TOKENS` die einzige Option.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Tool-Eingabeschemas mit einem Kombinator auf Root-Ebene
</h2>

Einige MCP-Server deklarieren das Eingabeschema eines Tools als JSON-Schema-Union mit `anyOf`, `oneOf` oder `allOf` auf der obersten Ebene des Schemas. Die Claude API akzeptiert diese Schlüsselwörter nicht auf der Schema-Root. Sie akzeptiert Kombinatoren, die in `properties` verschachtelt sind, die Claude Code unverändert sendet.

Tools mit einem Kombinator auf Root-Ebene bleiben verfügbar. Bevor das Tool an die API gesendet wird, vereinfacht Claude Code das Schema zu einem einzelnen Objekt und stellt dem Beschreibungstext des Tools einen Satz voran, der Claude mitteilt, welche Parametergruppen zusammengehören:

* `allOf`: Eigenschaften aus jedem Branch werden zusammengeführt, und die `required`-Liste jedes Branchs gilt weiterhin
* `anyOf` und `oneOf`: Eigenschaften aus jedem Branch werden zusammengeführt, und die `required`-Liste jedes Branchs wird stattdessen in der Tool-Beschreibung beschrieben, anstatt vom Schema erzwungen zu werden

Ihr Server empfängt die Argumente, die Claude ausgewählt hat, daher sollten Sie die Kombination weiterhin serverseitig validieren.

Wenn Claude Code kein Schema erstellen kann, das die API akzeptiert, oder bei einer Bereitstellung, die die Remote-Konfiguration, die das Umschreiben ermöglicht, nicht erhält, überspringt es dieses eine Tool, protokolliert den Grund im Log des Servers und lässt die anderen Tools des Servers verfügbar. Versionen vor v2.1.195 überspringen jedes Tool, dessen Eingabeschema einen Root-Level `anyOf`, `oneOf` oder `allOf` hat.

<h2 id="tools-with-invalid-input-schemas">
  Tools mit ungültigen Eingabeschemas
</h2>

Die Claude API überprüft das Eingabeschema jedes Tools in einer Anfrage und lehnt die gesamte Anfrage ab, wenn ein Schema fehlschlägt. Ein einzelnes MCP-Tool mit einem fehlerhaften Schema würde daher dazu führen, dass jede Anfrage, die es enthält, mit einem 400-Fehler fehlschlägt. Claude Code führt zwei der API-Überprüfungen selbst durch, wenn es die Tools eines Servers lädt, und schließt jedes Tool aus, das diese Überprüfungen nicht bestehen würde, damit die anderen Tools des Servers weiterhin funktionieren:

* Namen von Eigenschaften auf der obersten Ebene müssen 1 bis 64 Zeichen lang sein und dürfen nur ASCII-Buchstaben und Ziffern, `_`, `.` und `-` verwenden
* Das Schema muss gegen das JSON-Schema-Draft-2020-12-Meta-Schema gültig sein. Claude Code wendet diese Überprüfung auf Schemas an, die kein `$schema` deklarieren, und auf Schemas, die Draft 2020-12 deklarieren. Ein Schema, das einen anderen Dialekt deklariert, überspringt diese Überprüfung, obwohl die obige Überprüfung der Eigenschaftsnamen weiterhin gilt

Claude Code führt die Überprüfungen nach dem [Umschreiben des Root-Level-Kombinators](#tool-input-schemas-with-a-root-level-combinator) auf dem Schema durch, das es tatsächlich senden würde.

Wenn Claude Code ein Tool ausschließt, zeichnet es den Grund im Log des Servers auf und teilt Claude mit, welche Tools es ausgeschlossen hat und warum, damit Sie Claude fragen können, warum ein Tool fehlt. Wenn Sie das Schema auf dem Server korrigieren, wird das Tool beim nächsten Laden der Server-Tools durch Claude Code wieder verfügbar.

Claude Code aktiviert den Ausschluss über ein Feature-Flag, das es von Anthropic abruft. Bei einer [Bereitstellung, bei der das Flag-Abrufen deaktiviert ist](/docs/de/env-vars#features-that-need-feature-flag-fetching), oder auf einem Computer, dessen Flags nie angekommen sind, z. B. auf einem isolierten Computer, führt Claude Code die Überprüfungen weiterhin durch und zeichnet im Log des Servers auf, welches Tool abgelehnt würde, sendet das Schema des Tools aber trotzdem an die API. Die API lehnt eine Anfrage, die dieses Schema enthält, mit [einem 400-Fehler ab, der das Tool nach seiner Position benennt](/docs/de/errors#tool-input-schema-is-invalid). Vor v2.1.216 führte keine Bereitstellung diese Überprüfungen durch.

Die [Root-Level-Kombinator-Behandlung](#tool-input-schemas-with-a-root-level-combinator) ist separat und behält ihr eigenes Verhalten bei, wenn das Flag-Abrufen deaktiviert ist oder die Flags nie angekommen sind.

<h2 id="require-approval-for-a-specific-tool">
  Genehmigung für ein bestimmtes Tool erforderlich
</h2>

Wenn Sie einen MCP-Server erstellen, können Sie ein Tool als erforderlich für explizite Genehmigung bei jedem Aufruf kennzeichnen, indem Sie `_meta["anthropic/requiresUserInteraction"]` in der `tools/list`-Antwort des Tools auf `true` setzen. Der Wert muss der JSON-Boolean `true` sein; alle anderen Werte werden ignoriert.

Claude Code zeigt die Genehmigungsaufforderung dieses Tools bei jedem Aufruf an, auch in den [Genehmigungsmodi](/docs/de/permissions#permission-modes) `acceptEdits`, `auto` und `bypassPermissions`, und bietet keine Option „Nicht erneut fragen" dafür an. [Zulassungsregeln](/docs/de/permissions#permission-rule-syntax), die dem Tool entsprechen, überspringen die Aufforderung ebenfalls nicht. Im Modus `dontAsk`, der niemals eine Aufforderung anzeigt, lehnt Claude Code den Aufruf stattdessen ab.

Die Aufforderung muss eine Person erreichen. Im nicht-interaktiven Modus mit [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) wird ein `allow`-Ergebnis aus dem Prompt-Tool für ein gekennzeichnetes Tool in eine Ablehnung mit der Nachricht `MCP tool requires user interaction; not supported via --permission-prompt-tool` umgewandelt. Der [`canUseTool`-Callback](/docs/de/agent-sdk/permissions) des Agent SDK empfängt diese Aufrufe und kann sie genehmigen, da von Ihrer SDK-Anwendung erwartet wird, dass sie diese einem Benutzer anzeigt.

Verwenden Sie dies für Tools, deren Genehmigungsaufforderung selbst der Zweck ist, z. B. ein Zustimmungs- oder Zugriffsgenehmigungsschritt, bei dem automatische Genehmigung bedeuten würde, dass kein Mensch jemals zugestimmt hat. Andere Tools vom selben Server behalten ihr normales Genehmigungsverhalten.

Der folgende `tools/list`-Eintrag kennzeichnet ein Tool als immer genehmigungspflichtig.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

Die Annotation `anthropic/requiresUserInteraction` erfordert Claude Code v2.1.199 oder später. Frühere Versionen ignorieren sie und wenden den Standard-Genehmigungsablauf an.

Einige Oberflächen, wie [Remote Control](/docs/de/remote-control) und Anwendungen, die auf dem [Agent SDK](/docs/de/agent-sdk/overview) basieren, ermöglichen es Ihnen normalerweise, Tool-Aufrufe mit einem Tippen zu genehmigen. Für ein Tool, das mit dieser Annotation gekennzeichnet ist, hält Claude Code die Aktion mit einem Tippen zurück und zeigt stattdessen die vollständige Genehmigungsaufforderung des Tools an, sodass die Genehmigung weiterhin von einer Person stammt, die die Aufforderung beantwortet, anstatt von einem Tippen.

Claude Code hält die Genehmigung mit einem Tippen auf die gleiche Weise für jede Genehmigungsanfrage zurück, die nur das Terminal-Dialogfeld vollständig rendern kann, z. B. eine, die eine Sicherheitswarnung oder eine Option „Immer zulassen" enthält, die die Remote-Oberfläche nicht anzeigen kann. Sie beantworten diese Anfrage im Terminal-Dialogfeld anstatt von Remote Control. Erfordert Claude Code v2.1.214 oder später.

<h2 id="respond-to-mcp-elicitation-requests">
  Auf MCP-Elicitierungsanfragen reagieren
</h2>

MCP-Server können während einer Aufgabe strukturierte Eingaben von Ihnen anfordern, indem sie Elicitierung verwenden. Wenn ein Server Informationen benötigt, die er nicht selbst abrufen kann, zeigt Claude Code einen interaktiven Dialog an und leitet Ihre Antwort an den Server weiter. Auf Ihrer Seite ist keine Konfiguration erforderlich: Elicitierungsdialoge werden automatisch angezeigt, wenn ein Server sie anfordert.

Server können Eingaben auf zwei Arten anfordern:

* **Formularmodus**: Claude Code zeigt einen Dialog mit Formularfeldern an, die vom Server definiert werden (beispielsweise eine Aufforderung für Benutzernamen und Passwort). Füllen Sie die Felder aus und senden Sie sie ab.
* **URL-Modus**: Claude Code öffnet eine Browser-URL für Authentifizierung oder Genehmigung. Schließen Sie den Ablauf im Browser ab und bestätigen Sie dann in der CLI.

Im URL-Modus übergibt Claude Code die URL als Befehlszeilenargument an den URL-Handler Ihres Systems und begrenzt die Länge dieses Arguments. Wenn die URL nach der Escape-Sequenz für die Befehlszeile diese Grenze überschreitet, können Sie die Anfrage nur ablehnen. Jedes Zeichen, das Escape-Sequenzen benötigt, wie `%` oder `&`, zählt vierfach zur Grenze: sein eigenes Zeichen plus drei Escape-Zeichen. Eine URL ohne diese Zeichen erreicht die Grenze bei etwa 8.000 Zeichen. Eine URL, die größtenteils aus Prozentzeichen-Escape-Sequenzen besteht, bei denen jedes dritte Zeichen ein `%` ist, erreicht sie bei etwa 4.000.

Um automatisch auf Elicitierungsanfragen zu reagieren, ohne einen Dialog anzuzeigen, verwenden Sie den [`Elicitation`-Hook](/docs/de/hooks#elicitation).

Wenn Sie einen MCP-Server erstellen, der Elicitierung verwendet, lesen Sie die [MCP-Elicitierungsspezifikation](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) für Protokolldetails und Schemabeispiele.

<h2 id="use-mcp-resources">
  MCP-Ressourcen verwenden
</h2>

MCP-Server können Ressourcen bereitstellen, auf die Sie mit @ Erwähnungen verweisen können, ähnlich wie Sie auf Dateien verweisen.

<h3 id="reference-mcp-resources">
  MCP-Ressourcen referenzieren
</h3>

<Steps>
  <Step title="Verfügbare Ressourcen auflisten">
    Geben Sie `@` in Ihre Eingabeaufforderung ein, um verfügbare Ressourcen von allen verbundenen MCP-Servern anzuzeigen. Ressourcen werden zusammen mit Dateien im Autocomplete-Menü angezeigt.
  </Step>

  <Step title="Eine bestimmte Ressource referenzieren">
    Verwenden Sie das Format `@server:protocol://resource/path`, um auf eine Ressource zu verweisen:

    ```text wrap theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text wrap theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Mehrere Ressourcenverweise">
    Sie können mehrere Ressourcen in einer einzelnen Eingabeaufforderung referenzieren:

    ```text wrap theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Tipps:

  * Ressourcen werden automatisch abgerufen und als Anhänge eingebunden, wenn sie referenziert werden
  * Ressourcenpfade sind in der @ Erwähnungs-Autocomplete fuzzy-durchsuchbar
  * Claude Code stellt automatisch Tools bereit, um MCP-Ressourcen aufzulisten und zu lesen, wenn Server diese unterstützen
  * Ressourcen können jeden Inhaltstyp enthalten, den der MCP-Server bereitstellt (Text, JSON, strukturierte Daten usw.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Mit MCP-Tool-Suche skalieren
</h2>

Die Tool-Suche hält die MCP-Kontextnutzung niedrig, indem Tool-Definitionen aufgeschoben werden, bis Claude sie benötigt. Nur Tool-Namen und Server-Anweisungen werden beim Sitzungsstart geladen, sodass das Hinzufügen weiterer MCP-Server minimale Auswirkungen auf Ihr Kontextfenster hat. Claude Code erzwingt keine feste Tool-Obergrenze pro Server; die praktische Grenze ist Ihr Kontextfenster-Budget.

<Note>
  Die Tool-Suche wird bei Microsoft Foundry [Bereitstellungen auf Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) nicht unterstützt, die sie serverseitig ablehnen: Claude Code erkennt die Ablehnung und lädt MCP-Tools stattdessen vorab für diese Bereitstellung. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) kann dies nicht überschreiben, da die Ablehnung von der Bereitstellung selbst kommt.
</Note>

<h3 id="for-mcp-server-authors">
  Für MCP-Server-Autoren
</h3>

Wenn Sie einen MCP-Server erstellen, wird das Feld „Server-Anweisungen" mit aktivierter Tool-Suche nützlicher. Server-Anweisungen helfen Claude zu verstehen, wann nach Ihren Tools gesucht werden soll, ähnlich wie [Skills](/docs/de/skills) funktionieren.

Fügen Sie klare, aussagekräftige Server-Anweisungen hinzu, die erklären:

* Welche Kategorie von Aufgaben Ihre Tools verarbeiten
* Wann Claude nach Ihren Tools suchen sollte
* Wichtige Funktionen, die Ihr Server bietet

Claude Code kürzt jede Tool-Beschreibung und jede Server-Anweisung standardmäßig auf 2.048 Zeichen. Halten Sie sie prägnant, und platzieren Sie kritische Details am Anfang.

Um die Grenze für jeden MCP-Server in Ihrer Sitzung zu ändern, setzen Sie [`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](/docs/de/env-vars#variables) auf eine Anzahl von Zeichen. Diese Variable erfordert Claude Code v2.1.280 oder später.

<h3 id="configure-tool-search">
  Tool-Suche konfigurieren
</h3>

Die Tool-Suche ist standardmäßig aktiviert: MCP-Tools werden aufgeschoben und bei Bedarf erkannt. Claude Code deaktiviert sie, wenn `ANTHROPIC_BASE_URL` auf einen Host eines Drittanbieters verweist, da die meisten Proxys `tool_reference`-Blöcke nicht weiterleiten. Setzen Sie `ENABLE_TOOL_SEARCH` explizit, um diesen Fallback zu überschreiben.

Das Setzen von [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/de/env-vars) hält die Tool-Suche aus. Sie können sie nicht durch das Setzen von `ENABLE_TOOL_SEARCH` selbst überschreiben. Ihre Organisation kann die Tool-Suche durch [verwaltete Einstellungen](/docs/de/managed-settings) auf Claude Code v2.1.227 oder später aktiviert halten. [Deaktivieren Sie Pre-Release-Funktionen](/docs/de/llm-gateway-protocol#disable-pre-release-capabilities) behandelt, wo die Überschreibung gilt und was die Variable entfernt.

Die Tool-Suche erfordert ein Modell, das `tool_reference`-Blöcke unterstützt: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 und neuere Modelle. Siehe [Modellkompatibilität in der API-Dokumentation](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) für die aktuelle Liste.

Auf Googles Cloud Agent Platform entscheidet Claude Code nach Modellgeneration:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 und neuere**: Die Tool-Suche ist standardmäßig aktiviert, genauso wie bei der Anthropic API.
* **Frühere Agent Platform-Modelle**: Claude Code lädt alle MCP-Tools vorab, da ihre Serving-Stacks den erforderlichen Beta-Header ablehnen. `ENABLE_TOOL_SEARCH=true` überschreibt dies nicht.

Vor v2.1.221 deaktivierte Claude Code die Tool-Suche für alle Modelle auf Googles Cloud Agent Platform, es sei denn, Sie setzen `ENABLE_TOOL_SEARCH=true`.

Steuern Sie das Verhalten der Tool-Suche mit der Umgebungsvariablen `ENABLE_TOOL_SEARCH`:

| Wert            | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (nicht gesetzt) | Alle MCP-Tools werden aufgeschoben und bei Bedarf geladen. Fallback zum Vorab-Laden auf Googles Cloud Agent Platform-Modellen älter als die Claude 4.5-Generation, wenn `ANTHROPIC_BASE_URL` ein Host eines Drittanbieters ist, oder bei einer Microsoft Foundry-Bereitstellung auf Azure                                                                                                                                                |
| `true`          | Alle MCP-Tools werden aufgeschoben, außer bei einer Microsoft Foundry-Bereitstellung auf Azure, wo die serverseitige Ablehnung immer noch das Vorab-Laden erzwingt, und auf Googles Cloud Agent Platform-Modellen älter als die Claude 4.5-Generation, wo Claude Code Tools weiterhin vorab lädt. Claude Code sendet den Beta-Header durch Proxys, und Anfragen schlagen auf Proxys fehl, die `tool_reference`-Blöcke nicht unterstützen |
| `auto`          | Schwellenmodus: Claude Code lädt die Tools, die es sonst aufgeschoben hätte, vorab, während ihre Definitionen weniger als 10 % des Kontextfensters ausmachen, und schiebt alle auf, sobald die Definitionen 10 % erreichen                                                                                                                                                                                                               |
| `auto:N`        | Schwellenmodus mit einem benutzerdefinierten Prozentsatz, wobei `N` 0-100 ist. Zum Beispiel `auto:5` für 5 %                                                                                                                                                                                                                                                                                                                             |
| `false`         | Alle MCP-Tools werden vorab geladen, kein Aufschub                                                                                                                                                                                                                                                                                                                                                                                       |

```bash theme={null}
# Verwenden Sie einen benutzerdefinierten 5%-Schwellenwert
ENABLE_TOOL_SEARCH=auto:5 claude

# Deaktivieren Sie die Tool-Suche vollständig
ENABLE_TOOL_SEARCH=false claude
```

Oder setzen Sie den Wert in Ihrem [settings.json `env`-Feld](/docs/de/settings-reference#env).

Sie können auch das `ToolSearch`-Tool spezifisch deaktivieren:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Einen Server vom Aufschub ausnehmen
</h3>

Wenn die Tools eines Servers Claude immer sichtbar sein sollten, ohne einen Suchschritt, setzen Sie `alwaysLoad` in der Konfiguration dieses Servers auf `true`. Jedes Tool von diesem Server wird dann unabhängig von der `ENABLE_TOOL_SEARCH`-Einstellung beim Sitzungsstart in den Kontext geladen. Verwenden Sie dies für eine kleine Anzahl von Tools, die Claude bei jedem Durchgang benötigt, da jedes vorab geladene Tool Kontext verbraucht, der sonst für Ihre Konversation verfügbar wäre.

Der folgende `.mcp.json`-Eintrag nimmt einen HTTP-Server aus, während andere Server aufgeschoben bleiben:

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

Das Feld `alwaysLoad` ist auf allen Server-Typen verfügbar. Ein MCP-Server kann auch einzelne Tools als immer geladen markieren, indem er `"anthropic/alwaysLoad": true` im `_meta`-Objekt des Tools einbezieht, was denselben Effekt nur für dieses Tool hat.

Das Setzen von `alwaysLoad: true` lässt auch den Startup auf die Tools des Servers warten, begrenzt auf das Standard-5-Sekunden-Verbindungs-Timeout, da sie vorhanden sein müssen, wenn der erste Prompt erstellt wird. Ein Remote-Server mit einem gültigen [`cached`-Eintrag](#server-status-detail) liefert seine Tools aus dem Cache, ohne sich zu verbinden, sodass er den Startup nicht verzögert. Andere Server verbinden sich standardmäßig im Hintergrund; setzen Sie [`MCP_CONNECTION_NONBLOCKING=0`](/docs/de/env-vars), um auch auf sie zu warten.

<h2 id="use-mcp-prompts-as-commands">
  MCP-Prompts als Befehle verwenden
</h2>

MCP-Server können Prompts bereitstellen, die als Befehle in Claude Code verfügbar werden.

<h3 id="execute-mcp-prompts">
  MCP-Prompts ausführen
</h3>

<Steps>
  <Step title="Verfügbare Prompts entdecken">
    Geben Sie `/` ein, um die Ihnen verfügbaren Befehle anzuzeigen, einschließlich derjenigen von MCP-Servern. Claude Code listet jeden MCP-Prompt als `/servername:promptname (MCP)` auf. Die Eingabe von `/mcp__servername__promptname` führt ihn ebenfalls aus.
  </Step>

  <Step title="Einen Prompt ohne Argumente ausführen">
    ```text wrap theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Einen Prompt mit Argumenten ausführen">
    Viele Prompts akzeptieren Argumente. Übergeben Sie diese durch Leerzeichen getrennt nach dem Befehl. Claude Code teilt die Argumente nach Leerzeichen auf, sodass jedes Argument ein einzelnes Token ist:

    ```text wrap theme={null}
    /mcp__github__pr_review 456
    ```

    ```text wrap theme={null}
    /mcp__jira__create_issue login-bug high
    ```
  </Step>
</Steps>

<Tip>
  Tipps:

  * MCP-Prompts werden dynamisch von verbundenen Servern erkannt
  * Argumente werden basierend auf den definierten Parametern des Prompts analysiert
  * Prompt-Ergebnisse werden direkt in das Gespräch eingefügt
  * In der Form `/mcp__servername__promptname` ersetzt Claude Code alle Zeichen im Servernamen außerhalb von `A-Z`, `a-z`, `0-9`, `_` und `-` durch `_` und verwendet den Prompt-Namen, wie ihn der Server deklariert
</Tip>

<h2 id="managed-mcp-configuration">
  Verwaltete MCP-Konfiguration
</h2>

Für Organisationen, die eine zentralisierte Kontrolle über MCP-Server benötigen, die Benutzer verbinden können, siehe [Verwaltete MCP-Konfiguration](/docs/de/managed-mcp). Sie behandelt die Bereitstellung eines festen Serversatzes mit `managed-mcp.json`, die Bereitstellung von Servern für jeden Benutzer mit `managedMcpServers`, die Einschränkung von Servern mit `allowedMcpServers` und `deniedMcpServers` sowie das, was Benutzer sehen, wenn ein Server blockiert ist.
