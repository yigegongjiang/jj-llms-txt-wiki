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

    Wenn Claude Code meldet, dass der Marketplace nicht gefunden wird, führen Sie zuerst `/plugin marketplace add anthropics/claude-plugins-official` aus und versuchen Sie dann die Installation erneut. Nach der Installation führen Sie `/reload-plugins` aus, um es in der aktuellen Sitzung zu aktivieren.
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

<h3 id="option-2-add-a-remote-sse-server">
  Option 2: Einen Remote-SSE-Server hinzufügen
</h3>

<Warning>
  Das SSE-Transportprotokoll (Server-Sent Events) ist veraltet. Verwenden Sie stattdessen HTTP-Server, wo verfügbar.
</Warning>

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

Diese Variable wird in der Umgebung des Servers gesetzt, nicht in der Umgebung von Claude Code selbst, daher erfordert das Referenzieren über `${VAR}`-Erweiterung in einer projekt- oder benutzergesteuerten `.mcp.json` `command` oder `args` einen Standard wie `${CLAUDE_PROJECT_DIR:-.}`. Von Plugins bereitgestellte MCP-Konfigurationen ersetzen `${CLAUDE_PROJECT_DIR}` direkt und benötigen keinen Standard.

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

  `--env` akzeptiert mehrere `KEY=value`-Paare. Wenn der Servername direkt nach `--env` kommt, liest die CLI den Namen als ein weiteres Paar und lehnt ihn ab, daher platzieren Sie mindestens eine andere Option zwischen `--env` und dem Servernamen, wie in den obigen Beispielen.
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

<h3 id="managing-your-servers">
  Verwalten Ihrer Server
</h3>

Nach der Konfiguration können Sie Ihre MCP-Server mit diesen Befehlen verwalten:

```bash theme={null}
# Alle konfigurierten Server auflisten
claude mcp list

# Details für einen bestimmten Server abrufen
claude mcp get github

# Einen Server entfernen
claude mcp remove github

# (innerhalb von Claude Code) Serverstatus überprüfen
/mcp
```

Projekt-gesteuerte Server aus `.mcp.json`, die auf Ihre Genehmigung warten, erscheinen in `claude mcp list` als `⏸ Genehmigung ausstehend`. Führen Sie `claude` interaktiv aus, um sie zu überprüfen und zu genehmigen. `claude mcp get <name>` zeigt ausstehende Server als `⏸ Genehmigung ausstehend` und abgelehnte Server als `✗ Abgelehnt` an.

Ab v2.1.196 lesen `claude mcp list` und `claude mcp get` `.mcp.json`-Genehmigungen nur aus Einstellungsdateien, die nicht in das Repository eingecheckt werden, bis Sie dem Arbeitsbereich vertrauen, indem Sie `claude` darin ausführen und den Arbeitsbereich-Vertrauensdialog akzeptieren. Ein geklontes Repository kann seine eigenen Server nicht genehmigen: [`enableAllProjectMcpServers` oder `enabledMcpjsonServers`](/docs/de/settings#available-settings), die in die `.claude/settings.json` des Projekts eingecheckt werden, werden in einem nicht vertrauenswürdigen Ordner ignoriert, und der Server bleibt bei `⏸ Genehmigung ausstehend` statt verbunden und Health-Check zu sein.

Genehmigungen aus diesen Quellen gelten weiterhin in einem nicht vertrauenswürdigen Ordner:

* Ihre Benutzer-`~/.claude/settings.json`
* Verwaltete Einstellungen
* Einstellungen, die mit `--settings` übergeben werden

Genehmigungen in einer nicht verfolgten `.claude/settings.local.json` gelten ebenfalls, aber nur, nachdem Sie einen Vertrauensdialog für diesen Ordner oder eines seiner übergeordneten Verzeichnisse akzeptieren: Claude Code führt Git aus, um zu überprüfen, ob die Datei verfolgt wird, und führt diese Überprüfung nur in einem vertrauenswürdigen Ordner durch. In einem Ordner, dem Sie noch nie vertraut haben, warten die Genehmigungen der Datei auf den Vertrauensdialog, es sei denn, der Ordner ist Ihr eigenes Konfigurationsheim: Ihr Home-Verzeichnis oder ein Verzeichnis, dessen `.claude` Sie als [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) festgelegt haben. Vor v2.1.207 genehmigte eine nicht verfolgte `.claude/settings.local.json` Server in einem Ordner, dem Sie noch nie vertraut haben.

Ein `disabledMcpjsonServers`-Eintrag in einer beliebigen Einstellungsdatei lehnt den Server weiterhin ab.

Das `/mcp`-Panel zeigt die Tool-Anzahl neben jedem verbundenen Server an und kennzeichnet Server, die die Tools-Funktion ankündigen, aber keine Tools bereitstellen.

Ein Remote-Server, dessen Konfiguration eine leere `url` hat, wird in `/mcp`, in `claude mcp list` und im [`/plugin`](/docs/de/plugins)-Manager als `not configured` angezeigt, und Claude Code versucht nicht, sich damit zu verbinden. Ein Plugin kann einen Platzhalter-Eintrag wie diesen für einen Connector enthalten, den Sie später konfigurieren, sodass Claude Code ihn nicht als Fehler oder Setup-Problem meldet. Die Detailansicht des Servers in `/mcp` liest `No URL configured for this server`; setzen Sie die `url` des Eintrags, um sich zu verbinden. Vor v2.1.208 meldete Claude Code eine leere `url` als Konfigurationsproblem mit einer Aufforderung zur Wiederverbindung.

Wenn Ihre Anfrage Tools von einem Server benötigt, der sich noch im Hintergrund verbindet, wartet Claude auf diesen Server, bevor er fortfährt. Mit [Tool-Suche](#scale-with-mcp-tool-search) aktiviert, was die Standardeinstellung ist, erfolgt das Warten innerhalb des `ToolSearch`-Aufrufs. In Konfigurationen ohne Tool-Suche, wie Google Cloud's Agent Platform, eine benutzerdefinierte `ANTHROPIC_BASE_URL` oder `ENABLE_TOOL_SEARCH=false`, verwendet Claude stattdessen das `WaitForMcpServers`-Tool.

Einige Servernamen sind für Claude Codes integrierte Server reserviert: `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview` und `Claude Browser`. Wenn Ihre Konfiguration einen Server mit einem reservierten Namen definiert, überspringt Claude Code ihn beim Laden und zeigt eine Warnung an, die Sie auffordert, ihn umzubenennen. `claude mcp add` lehnt einen reservierten Namen mit einem Fehler ab.

`Claude Preview` und `Claude Browser` benennen beide den integrierten Server, den der [Claude Code Desktop-App-Vorschaubereich](/docs/de/desktop#preview-your-app) verwendet. Vor v2.1.205 war `Claude Browser` nicht reserviert, daher konnte ein benutzerkonfigurierter Server sich unter diesem Namen registrieren.

<h3 id="dynamic-tool-updates">
  Dynamische Tool-Updates
</h3>

Claude Code unterstützt MCP-`list_changed`-Benachrichtigungen, die es MCP-Servern ermöglichen, ihre verfügbaren Tools, Prompts und Ressourcen dynamisch zu aktualisieren, ohne dass Sie die Verbindung trennen und erneut verbinden müssen. Wenn ein MCP-Server eine `list_changed`-Benachrichtigung sendet, aktualisiert Claude Code automatisch die verfügbaren Funktionen von diesem Server.

<h3 id="automatic-reconnection">
  Automatische Wiederverbindung
</h3>

Wenn ein HTTP- oder SSE-Server während einer Sitzung die Verbindung trennt, verbindet sich Claude Code automatisch mit exponentiellem Backoff wieder: bis zu fünf Versuche, beginnend mit einer Verzögerung von einer Sekunde und sich jedes Mal verdoppelnd. Der Server wird als ausstehend in `/mcp` angezeigt, während die Wiederverbindung läuft. Nach fünf fehlgeschlagenen Versuchen wird der Server als fehlgeschlagen markiert und Sie können ihn manuell von `/mcp` aus erneut versuchen. Stdio-Server sind lokale Prozesse und werden nicht automatisch wiederverbunden.

Das gleiche Backoff gilt, wenn ein HTTP- oder SSE-Server beim Start seine anfängliche Verbindung nicht herstellt. Ab v2.1.121 versucht Claude Code die anfängliche Verbindung bis zu dreimal bei vorübergehenden Fehlern wie einer 5xx-Antwort, einer Verbindungsverweigerung oder einem Timeout erneut, markiert den Server dann als fehlgeschlagen, wenn er immer noch keine Verbindung herstellen kann. Authentifizierungs- und Not-Found-Fehler werden nicht erneut versucht, da sie eine Konfigurationsänderung erfordern, um behoben zu werden.

Wenn ein konfigurierter Server keine Verbindung herstellen kann, teilt Claude Code Claude mit, welcher Server fehlgeschlagen ist und sein Verbindungsfehler, einschließlich in `ToolSearch`-Ergebnissen, die kein passendes Tool finden, sodass Claude den Verbindungsfehler in seiner Antwort meldet. Erfordert [Tool-Suche](#scale-with-mcp-tool-search), die standardmäßig aktiviert ist. In Konfigurationen ohne Tool-Suche, wie eine benutzerdefinierte `ANTHROPIC_BASE_URL`, `ENABLE_TOOL_SEARCH=false` oder ein Modell, das Tool-Suche nicht unterstützt, und auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry, meldet Claude Code fehlgeschlagene Server-Verbindungen nicht an Claude. Vor v2.1.205 übergab Claude Code Verbindungsfehler nicht an Claude, und Claude konnte antworten, als ob die Tools des fehlgeschlagenen Servers nie konfiguriert wurden.

Ab v2.1.191 versuchen die Funktionsentdeckungsanfragen, die nach einer erfolgreichen Verbindung ausgeführt werden, wie `tools/list`, `prompts/list` und `resources/list`, auch vorübergehende Netzwerk- und Serverfehler bis zu dreimal mit kurzem Backoff erneut. Authentifizierungsfehler, 4xx-Antworten und Request-Timeouts werden nicht erneut versucht.

<h3 id="push-messages-with-channels">
  Push-Nachrichten mit Kanälen
</h3>

Ein MCP-Server kann auch Nachrichten direkt in Ihre Sitzung pushen, sodass Claude auf externe Ereignisse wie CI-Ergebnisse, Überwachungswarnungen oder Chat-Nachrichten reagieren kann. Um dies zu aktivieren, deklariert Ihr Server die Funktion `claude/channel` und Sie aktivieren sie mit dem Flag `--channels` beim Start. Siehe [Kanäle](/docs/de/channels), um einen offiziell unterstützten Kanal zu verwenden, oder [Kanäle-Referenz](/docs/de/channels-reference), um Ihren eigenen zu erstellen.

<Tip>
  Tipps:

  * Verwenden Sie das Flag `-s` oder `--scope`, um anzugeben, wo die Konfiguration gespeichert wird:
    * `local` (Standard): Nur für Sie im aktuellen Projekt verfügbar. Ältere Versionen nannten diesen Bereich `project`
    * `project`: Geteilt mit allen im Projekt über die Datei `.mcp.json`
    * `user`: Für Sie über alle Projekte hinweg verfügbar. Ältere Versionen nannten diesen Bereich `global`
  * Legen Sie Umgebungsvariablen mit `-e` oder `--env`-Flags fest (zum Beispiel `-e KEY=value`)
  * Die Flags `--transport` und `--header` akzeptieren auch die Kurzformen `-t` und `-H`
  * Konfigurieren Sie das Startup-Timeout des MCP-Servers mit der Umgebungsvariablen `MCP_TIMEOUT` (zum Beispiel `MCP_TIMEOUT=10000 claude` setzt ein 10-Sekunden-Timeout)
  * Setzen Sie ein Pro-Server-Tool-Ausführungs-Timeout, indem Sie ein `timeout`-Feld in Millisekunden zum `.mcp.json`-Eintrag dieses Servers hinzufügen, zum Beispiel `"timeout": 600000` für zehn Minuten. Dies überschreibt die Umgebungsvariable `MCP_TOOL_TIMEOUT` nur für diesen Server
  * Claude Code zeigt eine Warnung an, wenn die MCP-Tool-Ausgabe 10.000 Token überschreitet und begrenzt die Ausgabe standardmäßig auf 25.000 Token. Um dieses Limit zu erhöhen, setzen Sie die Umgebungsvariable `MAX_MCP_OUTPUT_TOKENS` (zum Beispiel `MAX_MCP_OUTPUT_TOKENS=50000`); die Warnschwelle ist fest. Siehe [MCP-Ausgabegrenzen und Warnungen](#mcp-output-limits-and-warnings)
  * Verwenden Sie `/mcp`, um sich bei Remote-Servern zu authentifizieren, die OAuth 2.0-Authentifizierung erfordern
</Tip>

Das Pro-Server-`timeout` ist eine harte Wanduhr-Grenze pro Tool-Aufruf, und Fortschrittsbenachrichtigungen vom Server verlängern sie nicht. Werte unter 1000 werden ignoriert und fallen auf `MCP_TOOL_TIMEOUT` zurück, oder auf seinen Standard von etwa 28 Stunden, wenn diese Variable nicht gesetzt ist. Für einen HTTP-, SSE- oder [claude.ai-Connector](/docs/de/mcp#use-mcp-servers-from-claude-ai)-Server gibt es auch einen zweiten, Pro-Request-Timer, der jeden Request bis zur ersten Antwort-Byte des Servers abdeckt. Dieser Timer beträgt 60 Sekunden, es sei denn, Sie setzen das Pro-Server-`timeout` oder `MCP_TOOL_TIMEOUT`; das Setzen eines dieser auf 60 Sekunden oder höher erhöht den Pro-Request-Timer auf diesen Wert, ein niedrigerer Wert verkürzt ihn nicht, und der 28-Stunden-Standard eines nicht gesetzten `MCP_TOOL_TIMEOUT` speist ihn nie. Stdio- und WebSocket-Server haben keinen Pro-Request-Timer. Vor v2.1.162 wurden Werte unter 1000 stattdessen auf eine Sekunde begrenzt.

Ein Pro-Server-`timeout` von mindestens 1000 fungiert auch als Untergrenze für das unten beschriebene Idle-Timeout: Claude Code bricht die Tool-Aufrufe dieses Servers niemals wegen Untätigkeit früher ab als das Pro-Server-`timeout`. Erfordert Claude Code v2.1.203 oder später.

Ein Tool-Aufruf an einen MCP-Server, der für das Idle-Fenster keine Antwort und keine Fortschrittsbenachrichtigung sendet, bricht mit einem Fehler ab, anstatt auf die Wanduhr-Grenze zu warten. Das Idle-Timeout erfordert Claude Code v2.1.187 oder später. Es gilt für jeden Server-Typ außer IDE-Servern und SDK-In-Process-Servern. Das Idle-Fenster beträgt standardmäßig fünf Minuten für HTTP-, SSE-, WebSocket- und [claude.ai-Connector](#use-mcp-servers-from-claude-ai)-Server und 30 Minuten für Stdio-Server. Vor v2.1.203 waren Stdio-Server vom Idle-Timeout ausgenommen.

Setzen Sie die Umgebungsvariable [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/de/env-vars) in Millisekunden, um das Idle-Fenster zu ändern, oder setzen Sie sie auf `0`, um die Überprüfung zu deaktivieren.

<h3 id="plugin-provided-mcp-servers">
  Von Plugins bereitgestellte MCP-Server
</h3>

[Plugins](/docs/de/plugins) können MCP-Server bündeln und automatisch Tools und Integrationen bereitstellen, wenn das Plugin aktiviert ist. Plugin-MCP-Server funktionieren identisch mit benutzerkonfigurierten Servern.

**Wie Plugin-MCP-Server funktionieren**:

* Plugins definieren MCP-Server in `.mcp.json` im Plugin-Root oder inline in `plugin.json`
* Wenn ein Plugin aktiviert ist, starten seine MCP-Server automatisch
* Plugin-MCP-Tools erscheinen neben manuell konfigurierten MCP-Tools
* Plugin-Server werden durch die Plugin-Installation verwaltet, nicht durch `/mcp`-Befehle

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

* **Automatischer Lebenszyklus**: Bei Sitzungsstart verbinden sich Server für aktivierte Plugins automatisch. Wenn Sie ein Plugin während einer Sitzung aktivieren oder deaktivieren, führen Sie `/reload-plugins` aus, um seine MCP-Server zu verbinden oder zu trennen
* **Pfad-Platzhalter**: `${CLAUDE_PLUGIN_ROOT}` wird in das Installationsverzeichnis des Plugins aufgelöst, `${CLAUDE_PLUGIN_DATA}` in sein [persistentes Zustandsverzeichnis](/docs/de/plugins-reference#persistent-data-directory), und `${CLAUDE_PROJECT_DIR}` in das stabile Projektstammverzeichnis. Die Substitution gilt für:
  * `stdio`-Server: `command`, `args`, `env`
  * `http`-, `sse`- und `ws`-Server: `url`, `headers` und `headersHelper`. Vor v2.1.195 übergab `headersHelper` den Platzhalter als Literal-String
* **Zugriff auf Benutzerumgebung**: Zugriff auf die gleichen Umgebungsvariablen wie manuell konfigurierte Server
* **Mehrere Transporttypen**: Unterstützung für Stdio-, SSE-, HTTP- und WebSocket-Transporte, wobei die Transportunterstützung je nach Server variieren kann

**Anzeigen von Plugin-MCP-Servern**:

```bash theme={null}
# Innerhalb von Claude Code alle MCP-Server einschließlich Plugin-Server anzeigen
/mcp
```

Plugin-Server erscheinen in der Liste mit Indikatoren, die zeigen, dass sie von Plugins stammen.

**Plugin-MCP-Tool-Namen**:

Tools von einem Plugin-gebündelten MCP-Server enthalten sowohl den Plugin-Namen als auch den Server-Schlüssel in ihrem aufrufbaren Namen. Die vollständige Form ist `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, wobei jedes Zeichen außerhalb von `A-Z`, `a-z`, `0-9`, `_` und `-` durch `_` ersetzt wird. Für den Server `database-tools`, der in einem Plugin namens `my-plugin` gebündelt ist, ist ein `query`-Tool aufrufbar als:

```
mcp__plugin_my-plugin_database-tools__query
```

Verwenden Sie diesen vollständigen Namen, wenn Sie auf das Tool in [Berechtigungsregeln](/docs/de/permissions), der `allowed-tools`-Liste eines Skills, dem [`tools`-Feld eines Subagenten](/docs/de/sub-agents#available-tools) oder einem [Hook-Matcher](/docs/de/hooks#match-mcp-tools) verweisen. Ein Hook-Matcher, der gegen den bloßen Server-Schlüssel geschrieben wurde, wie `mcp__database-tools__.*`, wird niemals für einen Plugin-gebündelten Server ausgelöst.

Der Server selbst registriert sich unter dem scoped Namen `plugin:<plugin-name>:<server-name>`, wie `plugin:my-plugin:database-tools`. Verwenden Sie diesen Namen, wo ein konfigurierter Server-Name erwartet wird, wie das [`server`-Feld eines `mcp_tool`-Hooks](/docs/de/hooks#mcp-tool-hook-fields).

**Vorteile von Plugin-MCP-Servern**:

* **Gebündelte Verteilung**: Tools und Server zusammen verpackt
* **Automatische Einrichtung**: Keine manuelle MCP-Konfiguration erforderlich
* **Team-Konsistenz**: Alle erhalten die gleichen Tools, wenn das Plugin installiert ist

Siehe die [Plugin-Komponenten-Referenz](/docs/de/plugins-reference#mcp-servers) für Details zum Bündeln von MCP-Servern mit Plugins.

<h2 id="mcp-installation-scopes">
  MCP-Installationsbereiche
</h2>

MCP-Server können auf drei verschiedenen Bereichsebenen konfiguriert werden. Der Bereich, den Sie wählen, steuert, in welchen Projekten der Server geladen wird und ob die Konfiguration mit Ihrem Team geteilt wird. Administratoren können Server auch auf Unternehmensebene über [verwaltete Konfiguration](#managed-mcp-configuration) bereitstellen.

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
  Der Begriff „lokaler Bereich" für MCP-Server unterscheidet sich von allgemeinen lokalen Einstellungen. Lokal begrenzte MCP-Server werden in `~/.claude.json` (Ihr Home-Verzeichnis) gespeichert, während allgemeine lokale Einstellungen `.claude/settings.local.json` (im Projektverzeichnis) verwenden. Siehe [Einstellungen](/docs/de/settings#settings-files) für Details zu Einstellungsdatei-Speicherorten.
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

Projektbegrenzte Server ermöglichen Teamzusammenarbeit durch das Speichern von Konfigurationen in einer `.mcp.json`-Datei im Root-Verzeichnis Ihres Projekts. Diese Datei ist dazu bestimmt, in die Versionskontrolle eingecheckt zu werden, um sicherzustellen, dass alle Teammitglieder Zugriff auf die gleichen MCP-Tools und -Dienste haben. Wenn Sie einen projektbegrenzten Server hinzufügen, erstellt oder aktualisiert Claude Code automatisch diese Datei mit der entsprechenden Konfigurationsstruktur.

```bash theme={null}
# Einen projektbegrenzten Server hinzufügen
claude mcp add --transport http paypal --scope project https://mcp.paypal.com/mcp
```

Die resultierende `.mcp.json`-Datei folgt einem standardisierten Format:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "command": "/path/to/server",
      "args": [],
      "env": {}
    }
  }
}
```

Aus Sicherheitsgründen fordert Claude Code eine Genehmigung an, bevor projektbegrenzte Server aus `.mcp.json`-Dateien verwendet werden. Wenn Sie diese Genehmigungswahlmöglichkeiten zurücksetzen müssen, verwenden Sie den Befehl `claude mcp reset-project-choices`.

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
4. [Von Plugins bereitgestellte Server](/docs/de/plugins)
5. [Claude.ai-Connectoren](#use-mcp-servers-from-claude-ai)

Die drei Bereiche stimmen Duplikate nach Name ab. Plugins und Connectoren stimmen nach Endpunkt ab, daher wird einer, der auf die gleiche URL oder den gleichen Befehl wie ein Server oben verweist, als Duplikat behandelt.

<h3 id="environment-variable-expansion-in-mcp-json">
  Umgebungsvariablen-Erweiterung in `.mcp.json`
</h3>

Claude Code unterstützt die Umgebungsvariablen-Erweiterung in `.mcp.json`-Dateien, die es Teams ermöglicht, Konfigurationen zu teilen und gleichzeitig Flexibilität für maschinenspezifische Pfade und vertrauliche Werte wie API-Schlüssel zu bewahren.

**Unterstützte Syntax:**

* `${VAR}` - Erweitert sich zum Wert der Umgebungsvariablen `VAR`
* `${VAR:-default}` - Erweitert sich zu `VAR`, wenn gesetzt, andernfalls wird `default` verwendet

**Erweiterungsorte:**
Umgebungsvariablen können erweitert werden in:

* `command` - Der Server-Ausführungspfad
* `args` - Befehlszeilenargumente
* `env` - Umgebungsvariablen, die an den Server übergeben werden
* `url` - Für HTTP-Server-Typen
* `headers` - Für HTTP-Server-Authentifizierung

**Beispiel mit Variablenerweiterung:**

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

Wenn eine erforderliche Umgebungsvariable nicht gesetzt ist und keinen Standardwert hat, lässt Claude Code den literalen `${VAR}`-Text in dem Wert und meldet eine Warnung zu fehlender Variable für diesen Server. Die Konfiguration wird trotzdem geladen, daher setzen Sie die Variable oder fügen Sie einen `:-default`-Fallback hinzu, damit der Server mit dem beabsichtigten Wert startet.

<h2 id="practical-examples">
  Praktische Beispiele
</h2>

<h3 id="example-monitor-errors-with-sentry">
  Beispiel: Fehler mit Sentry überwachen
</h3>

```bash theme={null}
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

Authentifizieren Sie sich mit Ihrem Sentry-Konto:

```text theme={null}
/mcp
```

Debuggen Sie dann Produktionsprobleme:

```text theme={null}
What are the most common errors in the last 24 hours?
```

```text theme={null}
Show me the stack trace for error ID abc123
```

```text theme={null}
Which deployment introduced these new errors?
```

<h3 id="example-connect-to-github-for-code-reviews">
  Beispiel: Mit GitHub für Code-Reviews verbinden
</h3>

GitHubs Remote-MCP-Server authentifiziert sich mit einem GitHub-Personal-Access-Token, der als Header übergeben wird. Um einen zu erhalten, öffnen Sie Ihre [GitHub-Token-Einstellungen](https://github.com/settings/personal-access-tokens), generieren Sie ein neues feingranulares Token mit Zugriff auf die Repositories, mit denen Claude arbeiten soll, und fügen Sie dann den Server hinzu:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Arbeiten Sie dann mit GitHub:

```text theme={null}
Review PR #456 and suggest improvements
```

```text theme={null}
Create a new issue for the bug we just found
```

```text theme={null}
Show me all open PRs assigned to me
```

<h3 id="example-query-your-postgresql-database">
  Beispiel: Ihre PostgreSQL-Datenbank abfragen
</h3>

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Fragen Sie dann Ihre Datenbank natürlich ab:

```text theme={null}
What's our total revenue this month?
```

```text theme={null}
Show me the schema for the orders table
```

```text theme={null}
Find customers who haven't made a purchase in 90 days
```

<h2 id="authenticate-with-remote-mcp-servers">
  Mit Remote-MCP-Servern authentifizieren
</h2>

Viele Cloud-basierte MCP-Server erfordern Authentifizierung. Claude Code unterstützt OAuth 2.0 für sichere Verbindungen.

Claude Code markiert einen Remote-Server als authentifizierungsbedürftig, wenn der Server mit `401 Unauthorized` oder `403 Forbidden` antwortet. Für einen Server, bei dem Sie sich nicht angemeldet haben, kennzeichnet jeder dieser Statuscodes den Server in `/mcp`, damit Sie den OAuth-Fluss abschließen können.

Wenn eine Anfrage an einen OAuth-Server, bei dem Sie bereits angemeldet sind, `401 Unauthorized` zurückgibt, aktualisiert Claude Code das gespeicherte Token, verbindet sich erneut und wiederholt die Anfrage einmal. Der Server wird in `/mcp` nur gekennzeichnet, wenn dieser Wiederholungsversuch auch fehlschlägt. Vor v2.1.206 kennzeichnete eine Token-Aktualisierung, die aus einem vorübergehenden Grund fehlschlug, wie z. B. ein Netzwerkfehler, einen OAuth-Server als authentifizierungsbedürftig für den Rest der Sitzung, obwohl sein Refresh-Token noch gültig war.

Ab v2.1.195 zeigt Claude Code sofort einen Hinweis an, der auf `/mcp` verweist, wenn eine Token-Aktualisierung fehlschlägt, weil der Server das gespeicherte Refresh-Token ablehnt. Das Menü des verbundenen Servers dort bietet „Re-authenticate", damit Sie sich erneut anmelden können, bevor der nächste Tool-Aufruf fehlschlägt.

Ein benutzerdefinierter Server, der einen `WWW-Authenticate`-Header zurückgibt, der auf seinen Autorisierungsserver verweist, erhält die gleiche automatische Erkennung wie jeder andere Remote-Server.

Ab v2.1.193 zeigt Claude Code auch einen Starthinweis an, wenn ein oder mehrere konfigurierte Server Authentifizierung benötigen, sodass Sie `/mcp` nicht öffnen müssen, um zu erkennen, welche Server Anmeldung benötigen.

Im nicht-interaktiven Modus gibt es kein `/mcp`-Panel, daher kann Claude Code den OAuth-Fluss nicht für Sie ausführen. Ab v2.1.196 teilt Claude Code Claude mit, dass die Tools des Servers nicht verfügbar sind, bis Sie ihn autorisieren, wenn ein konfigurierter Server während eines `claude -p`- oder Agent SDK-Laufs mit aktivierter [Tool-Suche](#scale-with-mcp-tool-search) (Standard) Authentifizierung benötigt. Claude kann dann den Server benennen, der Anmeldung benötigt, anstatt so zu reagieren, als wäre der Server nicht konfiguriert. Schließen Sie die Anmeldung aus einer interaktiven Sitzung mit `/mcp` oder `claude mcp login <name>` ab.

Wenn Sie `headers.Authorization` für den Server konfiguriert haben und der Server diesen Header ablehnt, meldet Claude Code die Verbindung als fehlgeschlagen, anstatt auf OAuth zurückzugreifen. Überprüfen Sie, dass das Token für den MCP-Endpunkt gültig ist, oder entfernen Sie den Header, um den OAuth-Fluss zu verwenden.

<Steps>
  <Step title="Fügen Sie den Server hinzu, der Authentifizierung erfordert">
    Zum Beispiel:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Verwenden Sie den /mcp-Befehl innerhalb von Claude Code">
    In Claude Code verwenden Sie den Befehl:

    ```text theme={null}
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

Ab v2.1.186 führt `claude mcp login <name>` den OAuth-Fluss eines konfigurierten Servers direkt aus Ihrer Shell aus, sodass Sie das `/mcp`-Panel nicht innerhalb einer Sitzung öffnen müssen.

```bash theme={null}
claude mcp login sentry
```

Um gespeicherte Anmeldedaten später zu löschen, führen Sie `claude mcp logout <name>` aus.

Ab v2.1.191 erkennt der Befehl, wenn kein lokaler Browser verfügbar ist, z. B. während einer SSH-Sitzung oder unter Linux ohne Display-Server, und gibt die Autorisierungs-URL aus, anstatt zu versuchen, einen Browser zu öffnen. Öffnen Sie die URL auf Ihrem lokalen Computer und fügen Sie dann die vollständige Umleitungs-URL aus der Adressleiste Ihres Browsers an der Eingabeaufforderung ein. Der Befehl benötigt ein interaktives Terminal für den Einfügungsschritt, daher verbinden Sie sich mit `ssh -t`. Übergeben Sie `--no-browser`, um die URL-Eingabeaufforderung zu erzwingen, auch wenn ein lokaler Browser erkannt wird.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Verwenden Sie einen festen OAuth-Callback-Port
</h3>

Einige MCP-Server erfordern einen spezifischen Redirect-URI, der im Voraus registriert ist. Standardmäßig wählt Claude Code einen zufällig verfügbaren Port für den OAuth-Callback. Verwenden Sie `--callback-port`, um den Port zu fixieren, damit er einem vorregistrierten Redirect-URI der Form `http://localhost:PORT/callback` entspricht.

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
  </Step>

  <Step title="Fügen Sie den Server mit Ihren Anmeldedaten hinzu">
    Wählen Sie eine der folgenden Methoden. Der für `--callback-port` verwendete Port kann ein beliebiger verfügbarer Port sein. Er muss nur dem Redirect-URI entsprechen, den Sie im vorherigen Schritt registriert haben.

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
  * Wenn der Server einen öffentlichen OAuth-Client ohne Secret verwendet, verwenden Sie nur `--client-id` ohne `--client-secret`
  * `--callback-port` kann mit oder ohne `--client-id` verwendet werden
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

Wenn der Server später einen 403 `insufficient_scope` für einen Tool-Aufruf zurückgibt, authentifiziert sich Claude Code mit den gleichen fixierten Bereichen erneut. Erweitern Sie `oauth.scopes`, wenn ein Tool, das Sie benötigen, einen Bereich außerhalb der Fixierung erfordert.

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
* Der Befehl wird in einer Shell mit einem 10-Sekunden-Timeout ausgeführt, aus dem aktuellen Arbeitsverzeichnis der Sitzung. Verwenden Sie einen absoluten Pfad oder einen Befehl auf `PATH` für das Skript
* Dynamische Header überschreiben alle statischen `headers` mit dem gleichen Namen

Der Helper wird bei jeder Verbindung neu ausgeführt (beim Sitzungsstart und bei Wiederverbindung). Es gibt kein Caching, daher ist Ihr Skript für jede Token-Wiederverwendung verantwortlich.

Ab v2.1.193 führt Claude Code den Helper automatisch erneut aus, wenn ein Tool-Aufruf `401 Unauthorized` oder `403 Forbidden` zurückgibt, verbindet sich mit den neuen Headern erneut und wiederholt den Aufruf einmal. Claude Code markiert den Server als authentifizierungsbedürftig in `/mcp` nur, wenn dieser Wiederholungsversuch auch fehlschlägt.

Claude Code setzt diese Umgebungsvariablen beim Ausführen des Helpers:

| Variable                      | Wert                                                                                                                             |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | der Name des MCP-Servers                                                                                                         |
| `CLAUDE_CODE_MCP_SERVER_URL`  | die URL des MCP-Servers                                                                                                          |
| `CLAUDE_PLUGIN_ROOT`          | das Root-Verzeichnis des Plugins. Wird nur gesetzt, wenn ein [Plugin](/docs/de/plugins-reference#mcp-servers) den Server bereitstellt |

Verwenden Sie diese, um ein einzelnes Helper-Skript zu schreiben, das mehrere MCP-Server bedient.

Für einen von einem Plugin bereitgestellten Server wird der Helper auch mit seinem Arbeitsverzeichnis auf das Plugin-Root gesetzt ausgeführt, sodass ein relativer `headersHelper`-Pfad im Plugin-Verzeichnis aufgelöst wird, anstatt gegen das Arbeitsverzeichnis der Sitzung. Erfordert Claude Code v2.1.195 oder später.

Ein von einem Plugin bereitgestellter `headersHelper` kann nicht auf die [`${user_config.*}`](/docs/de/plugins-reference#user-configuration)-Werte des Plugins verweisen, da der Befehl durch eine Shell ausgeführt wird. Claude Code meldet den Server als fehlkonfiguriert mit einem [Fehler](/docs/de/errors#plugin-command-references-user-config) und ersetzt den Wert nicht. Setzen Sie `${user_config.KEY}` stattdessen in das Feld `headers` des Servers, das nicht shell-geparst wird, oder lassen Sie das Helper-Skript den Wert aus seiner eigenen Umgebung oder einer Konfigurationsdatei lesen. Vor v2.1.207 ersetzte `headersHelper` `${user_config.*}`-Werte.

<Note>
  `headersHelper` führt beliebige Shell-Befehle aus. Wenn es auf Projekt- oder lokalem Bereich definiert ist, wird es nur nach Ihrer Zustimmung zum Workspace-Trust-Dialog ausgeführt.
</Note>

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
  MCP-Server von Claude.ai verwenden
</h2>

Wenn Sie sich in Claude Code mit einem [claude.ai](https://claude.ai)-Konto angemeldet haben, sind MCP-Server, die Sie in Claude.ai hinzugefügt haben, bekannt als [Connectoren](https://claude.com/docs/connectors), automatisch in Claude Code verfügbar:

<Steps>
  <Step title="Konfigurieren Sie MCP-Server in Claude.ai">
    Fügen Sie Server unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors) hinzu. Bei Team- und Enterprise-Plänen können nur Administratoren Server hinzufügen.
  </Step>

  <Step title="Authentifizieren Sie den MCP-Server">
    Führen Sie alle erforderlichen Authentifizierungsschritte in Claude.ai durch.
  </Step>

  <Step title="Zeigen Sie Server in Claude Code an und verwalten Sie sie">
    In Claude Code verwenden Sie den Befehl:

    ```text theme={null}
    /mcp
    ```

    Claude.ai-Server erscheinen in der Liste mit Indikatoren, die zeigen, dass sie von Claude.ai stammen.
  </Step>
</Steps>

Ab v2.1.161 werden Connectoren, bei denen Sie sich noch nie angemeldet haben, hinter einer Zeile `Show unused connectors` am Ende des Claude.ai-Abschnitts eingeklappt, sodass eine von der Organisation bereitgestellte Liste das Panel nicht ausfüllt. Wählen Sie die Zeile aus, um sie zu erweitern. Ein Connector, bei dem Sie sich zuvor angemeldet haben, bleibt sichtbar, auch wenn er derzeit eine erneute Authentifizierung benötigt.

Claude.ai-Connectoren werden nur abgerufen, wenn Ihre aktive [Authentifizierungsmethode](/docs/de/authentication#authentication-precedence) Ihr Claude.ai-Abonnement ist. Sie werden nicht geladen, wenn `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `apiKeyHelper` oder ein Drittanbieter wie Amazon Bedrock oder Google Cloud's Agent Platform aktiv ist, auch wenn Sie zuvor `/login` ausgeführt haben.

Wenn `/mcp` einen Connector, den Sie hinzugefügt haben, nicht auflistet, führen Sie `/status` aus, um zu bestätigen, welche Authentifizierungsmethode aktiv ist, heben Sie diese Umgebungsvariable auf oder entfernen Sie die `apiKeyHelper`-Einstellung, und führen Sie dann `/login` aus, um Ihr Claude.ai-Konto auszuwählen.

Ein Server, den Sie in Claude Code hinzugefügt haben, hat [Vorrang](#scope-hierarchy-and-precedence) vor einem Claude.ai-Connector, der auf dieselbe URL verweist. Wenn dies geschieht, listet `/mcp` den Connector als verborgen auf und zeigt, wie Sie das Duplikat entfernen können, wenn Sie lieber den Connector verwenden möchten.

Einige von Anthropic gehostete Connectoren, wie Microsoft 365, Gmail und Google Calendar, unterstützen keine lokale OAuth von Claude Code aus, da der vorgelagerte Identitätsanbieter nur die Umleitungs-URL akzeptiert, die claude.ai registriert hat. Ab v2.1.162 zeigt die Authentifizierung eines dieser Hosts in `/mcp` eine Nachricht an, die Sie anweist, es stattdessen unter Einstellungen → Connectoren auf claude.ai zu verbinden. Sobald es dort verbunden ist, erscheint der Connector automatisch in Claude Code.

<h3 id="organization-controls-on-connector-tools">
  Organisationssteuerungen für Connector-Tools
</h3>

Ihre Organisation kann Pro-Tool-Steuerungen auf [claude.ai-Connectoren](https://claude.com/docs/connectors) festlegen. Claude Code liest diese Einstellungen beim Start und erzwingt sie lokal. Führen Sie `/mcp` aus, um zu sehen, welche Einstellung für jedes Tool auf einem Connector gilt.

* **Tool auf `ask` gesetzt**: Claude Code fordert bei jedem Aufruf mit dem Grund `Your organization requires approval for this tool` auf. Die Aufforderung erscheint auch in den [Berechtigungsmodi](/docs/de/permissions#permission-modes) `acceptEdits`, `auto` und `bypassPermissions` und bietet niemals eine Option, Ihre Wahl zu speichern. [Zulassungsregeln](/docs/de/permissions), die dem Tool entsprechen, überspringen die Aufforderung auch nicht. Im Modus `dontAsk`, der niemals auffordert, lehnt Claude Code den Aufruf stattdessen ab.
* **Tool auf `blocked` gesetzt**: Claude Code filtert das Tool heraus, bevor Claude es sieht, sodass es niemals in der Tool-Liste erscheint.

Das Erzwingen dieser Steuerungen erfordert Claude Code v2.1.129 oder später. Frühere Versionen ignorieren die Einstellungen und wenden den Standard-Berechtigungsfluss an.

<h3 id="disable-claude-ai-connectors">
  Claude.ai-Connectoren deaktivieren
</h3>

Um Claude.ai-MCP-Server in Claude Code zu deaktivieren, setzen Sie [`disableClaudeAiConnectors`](/docs/de/settings#available-settings) auf `true` in einem beliebigen Einstellungsbereich:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Diese Einstellung verwendet Any-Source-True-Semantik: `true` in einer beliebigen Einstellungsquelle hat Vorrang. Eine eingecheckte Projekt-`.claude/settings.json` kann ein Repository von Cloud-Connectoren ausschließen, aber ein Projekt-Level-`false` kann Connectoren nicht erneut aktivieren, die ein Benutzer- oder Richtlinien-Level-`true` deaktiviert hat. Server, die explizit über `--mcp-config` übergeben werden, sind nicht betroffen.

Sie können auch die Umgebungsvariable `ENABLE_CLAUDEAI_MCP_SERVERS` auf `false` setzen, was denselben Effekt für die aktuelle Shell-Sitzung hat:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Um einzelne Claude.ai-Connectoren zu blockieren, anstatt alle zu blockieren, fügen Sie sie zu [`deniedMcpServers`](/docs/de/managed-mcp) nach Name oder nach URL-Muster hinzu. Beispielsweise blockiert ein `serverName`-Eintrag von `"claude.ai Slack"` den Slack-Connector. Um einen Connector nur für das aktuelle Projekt ein- oder auszuschalten, verwenden Sie das `/mcp`-Panel.

<Note>
  Diese clientseitigen Einstellungen regeln lokale Claude Code-Sitzungen. In [Claude Code im Web](/docs/de/claude-code-on-the-web)-Sitzungen werden Claude.ai-Connectoren vom Remote-Host bereitgestellt und kommen als explizite `--mcp-config`-Einträge an, daher gilt `disableClaudeAiConnectors` dort nicht. Connector-URLs werden auch durch den Sitzungs-Proxy umgeschrieben, daher passt ein `deniedMcpServers`-`serverUrl`-Muster, das auf die Anbieter-URL abzielt, nicht. Verwalten Sie, welche Connectoren eine Cloud-Sitzung verwenden kann, über Ihre Claude.ai-Organisationseinstellungen.
</Note>

<h2 id="use-claude-code-as-an-mcp-server">
  Claude Code als MCP-Server verwenden
</h2>

Sie können Claude Code selbst als MCP-Server verwenden, mit dem sich andere Anwendungen verbinden können:

```bash theme={null}
# Starten Sie Claude als Stdio-MCP-Server
claude mcp serve
```

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
  **Konfigurieren Sie den Pfad der ausführbaren Datei**: Das Feld `command` muss auf die Claude Code-Ausführungsdatei verweisen. Wenn der Befehl `claude` nicht in Ihrem System-PATH vorhanden ist, müssen Sie den vollständigen Pfad zur ausführbaren Datei angeben.

  Um den vollständigen Pfad zu finden:

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

  Ohne den korrekten Pfad der ausführbaren Datei treten Fehler wie `spawn claude ENOENT` auf.
</Warning>

<Tip>
  Tipps:

  * Der Server bietet Zugriff auf Claudes Tools wie View, Edit, LS usw.
  * In Claude Desktop versuchen Sie, Claude aufzufordern, Dateien in einem Verzeichnis zu lesen, Änderungen vorzunehmen und mehr.
  * Dieser MCP-Server stellt nur Claudes Tools für Ihren MCP-Client zur Verfügung, daher ist Ihr eigener Client dafür verantwortlich, Benutzerbestätigung für einzelne Tool-Aufrufe zu implementieren.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  MCP-Ausgabelimits und Warnungen
</h2>

Wenn MCP-Tools große Ausgaben erzeugen, hilft Claude Code bei der Verwaltung der Token-Nutzung, um zu verhindern, dass Ihr Gesprächskontext überwältigt wird:

* **Ausgabe-Warnungsschwelle**: Claude Code zeigt eine Warnung an, wenn eine MCP-Tool-Ausgabe 10.000 Token überschreitet
* **Konfigurierbares Limit**: Sie können die maximale zulässige MCP-Ausgabe-Token mit der Umgebungsvariablen `MAX_MCP_OUTPUT_TOKENS` anpassen
* **Standardlimit**: Das Standardmaximum beträgt 25.000 Token
* **Bereich**: Die Umgebungsvariable gilt für Tools, die kein eigenes Limit deklarieren. Tools, die [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) setzen, verwenden diesen Wert stattdessen für Textinhalte, unabhängig davon, auf was `MAX_MCP_OUTPUT_TOKENS` gesetzt ist. Tools, die Bilddaten zurückgeben, unterliegen immer noch `MAX_MCP_OUTPUT_TOKENS`

Um das Limit für Tools zu erhöhen, die große Ausgaben erzeugen:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

Dies ist besonders nützlich bei der Arbeit mit MCP-Servern, die:

* Große Datensätze oder Datenbanken abfragen
* Detaillierte Berichte oder Dokumentation generieren
* Umfangreiche Protokolldateien oder Debugging-Informationen verarbeiten

<h3 id="raise-the-limit-for-a-specific-tool">
  Erhöhen Sie das Limit für ein bestimmtes Tool
</h3>

Wenn Sie einen MCP-Server erstellen, können Sie einzelnen Tools ermöglichen, Ergebnisse größer als die Standard-Persist-to-Disk-Schwelle zurückzugeben, indem Sie `_meta["anthropic/maxResultSizeChars"]` in der Tool-Antwort des `tools/list` einstellen. Claude Code erhöht die Schwelle dieses Tools auf den annotierten Wert, bis zu einer harten Obergrenze von 500.000 Zeichen.

Dies ist nützlich für Tools, die inhärent große, aber notwendige Ausgaben zurückgeben, wie Datenbankschemas oder vollständige Dateibäume. Ohne die Anmerkung werden Ergebnisse, die die Standard-Schwelle überschreiten, auf die Festplatte persistiert und durch eine Dateireferenz im Gespräch ersetzt.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

Die Anmerkung gilt unabhängig von `MAX_MCP_OUTPUT_TOKENS` für Textinhalte, daher müssen Benutzer die Umgebungsvariable nicht für Tools erhöhen, die sie deklarieren. Tools, die Bilddaten zurückgeben, unterliegen immer noch dem Token-Limit.

<Warning>
  Wenn Sie häufig Ausgabewarnungen bei bestimmten MCP-Servern erhalten, die Sie nicht kontrollieren, erwägen Sie, das Limit `MAX_MCP_OUTPUT_TOKENS` zu erhöhen. Sie können auch den Server-Autor bitten, die Anmerkung `anthropic/maxResultSizeChars` hinzuzufügen oder ihre Antworten zu paginieren. Die Anmerkung hat keine Auswirkung auf Tools, die Bildinhalte zurückgeben; für diese ist das Erhöhen von `MAX_MCP_OUTPUT_TOKENS` die einzige Option.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Tool-Eingabeschemas mit einem Root-Level-Kombinator
</h2>

Einige MCP-Server deklarieren das Eingabeschema eines Tools als JSON Schema Union mit `anyOf`, `oneOf` oder `allOf` auf der obersten Ebene des Schemas. Die Claude API akzeptiert diese Schlüsselwörter nicht auf der Schema-Root. Sie akzeptiert Kombinatoren, die in `properties` verschachtelt sind, die Claude Code unverändert sendet.

Ab Claude Code v2.1.195 bleiben Tools mit einem Root-Level-Kombinator verfügbar. Bevor das Tool an die API gesendet wird, vereinfacht Claude Code das Schema in ein einzelnes Objekt und fügt dem Tool-Beschreibung einen Satz hinzu, der Claude mitteilt, welche Parameter-Gruppen zusammengehören:

* `allOf`: Eigenschaften aus jedem Branch werden zusammengeführt, und die `required`-Liste jedes Branchs gilt weiterhin
* `anyOf` und `oneOf`: Eigenschaften aus jedem Branch werden zusammengeführt, und die `required`-Liste jedes Branchs wird stattdessen in der Tool-Beschreibung beschrieben

Ihr Server empfängt die Argumente, die Claude gewählt hat, daher validieren Sie die Kombination weiterhin serverseitig.

Wenn Claude Code kein Schema erstellen kann, das die API akzeptiert, oder auf einer Bereitstellung, die die Remote-Konfiguration nicht erhält, die die Umschreibung ermöglicht, wie auf einem Offline-Computer, überspringt es dieses eine Tool, zeichnet den Grund im Server-Log auf und lässt die anderen Tools des Servers verfügbar. Versionen älter als v2.1.195 überspringen jedes Tool, dessen Eingabeschema ein Root-Level-`anyOf`, `oneOf` oder `allOf` hat.

<h2 id="require-approval-for-a-specific-tool">
  Genehmigung für ein bestimmtes Tool erforderlich
</h2>

Wenn Sie einen MCP-Server erstellen, können Sie ein Tool als Genehmigung bei jedem Aufruf erforderlich markieren, indem Sie `_meta["anthropic/requiresUserInteraction"]` auf `true` in der Tool-Antwort des `tools/list` einstellen. Der Wert muss das JSON-Boolean `true` sein; jeder andere Wert wird ignoriert.

Claude Code zeigt die Genehmigungsaufforderung dieses Tools bei jedem Aufruf an, auch in `acceptEdits`, `auto` und `bypassPermissions` [Genehmigungsmodi](/docs/de/permissions#permission-modes), und bietet keine Option „Nicht erneut fragen" dafür an. [Allow-Regeln](/docs/de/permissions#permission-rule-syntax), die das Tool abgleichen, überspringen die Aufforderung auch nicht. Im `dontAsk`-Modus, der niemals auffordert, lehnt Claude Code den Aufruf stattdessen ab.

Die Aufforderung muss eine Person erreichen. Im nicht-interaktiven Modus mit [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) wird ein `allow`-Ergebnis aus dem Genehmigungstool für ein gekennzeichnetes Tool in eine Ablehnung mit der Nachricht `MCP tool requires user interaction; not supported via --permission-prompt-tool` umgewandelt. Der [`canUseTool`-Callback](/docs/de/agent-sdk/permissions) des Agent SDK empfängt diese Aufrufe und kann sie genehmigen, da der SDK-Host diese dem Benutzer zeigen soll.

Verwenden Sie dies für Tools, deren Genehmigungsaufforderung selbst der Punkt ist, wie ein Zustimmungs- oder Zugriffsgenehmigungsschritt, bei dem Auto-Genehmigung bedeuten würde, dass kein Mensch jemals zugestimmt hat. Andere Tools vom gleichen Server behalten ihr normales Genehmigungsverhalten.

Der folgende `tools/list`-Eintrag markiert ein Tool als immer Genehmigung erforderlich.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

Die Anmerkung `anthropic/requiresUserInteraction` erfordert Claude Code v2.1.199 oder später. Frühere Versionen ignorieren sie und wenden den Standard-Genehmigungsfluss an.

Wenn eine Sitzung mit [Remote Control](/docs/de/remote-control) oder einem SDK-Host verbunden ist, markiert Claude Code die Genehmigungsanfrage als Benutzerinteraktion erforderlich, daher zeigt der Client die Genehmigungsaufforderung des Tools für Sie an, anstatt einer One-Tap-Genehmigungsaktion.

<h2 id="respond-to-mcp-elicitation-requests">
  Reagieren Sie auf MCP-Elicitierungsanfragen
</h2>

MCP-Server können während einer Aufgabe strukturierte Eingaben von Ihnen anfordern, indem sie Elicitierung verwenden. Wenn ein Server Informationen benötigt, die er nicht selbst abrufen kann, zeigt Claude Code einen interaktiven Dialog an und leitet Ihre Antwort an den Server weiter. Auf Ihrer Seite ist keine Konfiguration erforderlich: Elicitierungs-Dialoge erscheinen automatisch, wenn ein Server sie anfordert.

Server können Eingaben auf zwei Arten anfordern:

* **Formularmodus**: Claude Code zeigt einen Dialog mit Formularfeldern an, die vom Server definiert werden (zum Beispiel eine Eingabeaufforderung für Benutzername und Passwort). Füllen Sie die Felder aus und senden Sie sie ab.
* **URL-Modus**: Claude Code öffnet eine Browser-URL für Authentifizierung oder Genehmigung. Führen Sie den Ablauf im Browser durch und bestätigen Sie dann in der CLI.

Um automatisch auf Elicitierungsanfragen ohne Dialog zu reagieren, verwenden Sie den [`Elicitation`-Hook](/docs/de/hooks#elicitation).

Wenn Sie einen MCP-Server erstellen, der Elicitierung verwendet, siehe die [MCP-Elicitierungs-Spezifikation](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) für Protokolldetails und Schema-Beispiele.

<h2 id="use-mcp-resources">
  MCP-Ressourcen verwenden
</h2>

MCP-Server können Ressourcen verfügbar machen, auf die Sie mit @-Erwähnungen verweisen können, ähnlich wie Sie auf Dateien verweisen.

<h3 id="reference-mcp-resources">
  Referenzieren Sie MCP-Ressourcen
</h3>

<Steps>
  <Step title="Verfügbare Ressourcen auflisten">
    Geben Sie `@` in Ihre Eingabeaufforderung ein, um verfügbare Ressourcen von allen verbundenen MCP-Servern anzuzeigen. Ressourcen erscheinen neben Dateien im Autocomplete-Menü.
  </Step>

  <Step title="Referenzieren Sie eine bestimmte Ressource">
    Verwenden Sie das Format `@server:protocol://resource/path`, um auf eine Ressource zu verweisen:

    ```text theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Mehrere Ressourcenreferenzen">
    Sie können mehrere Ressourcen in einer einzelnen Eingabeaufforderung referenzieren:

    ```text theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Tipps:

  * Ressourcen werden automatisch abgerufen und als Anhänge eingefügt, wenn sie referenziert werden
  * Ressourcenpfade sind fuzzy-durchsuchbar im @-Erwähnungs-Autocomplete
  * Claude Code stellt automatisch Tools zur Verfügung, um MCP-Ressourcen aufzulisten und zu lesen, wenn Server diese unterstützen
  * Ressourcen können jeden Inhaltstyp enthalten, den der MCP-Server bereitstellt (Text, JSON, strukturierte Daten usw.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Mit MCP-Tool-Suche skalieren
</h2>

Die Tool-Suche hält die MCP-Kontextnutzung niedrig, indem Tool-Definitionen aufgeschoben werden, bis Claude sie benötigt. Nur Tool-Namen und Server-Anweisungen werden beim Sitzungsstart geladen, daher hat das Hinzufügen weiterer MCP-Server minimale Auswirkungen auf Ihr Kontextfenster. Claude Code verhängt keine feste Tool-Obergrenze pro Server; die praktische Grenze ist Ihr Kontextfenster-Budget.

<h3 id="how-it-works">
  Wie es funktioniert
</h3>

Die Tool-Suche ist standardmäßig aktiviert. MCP-Tools werden aufgeschoben, anstatt sie vorab in den Kontext zu laden, und Claude verwendet ein Such-Tool, um relevante Tools zu entdecken, wenn eine Aufgabe sie benötigt. Nur die Tools, die Claude tatsächlich verwendet, gelangen in den Kontext. Aus Ihrer Perspektive funktionieren MCP-Tools genau wie zuvor.

Wenn Sie schwellenwertbasiertes Laden bevorzugen, setzen Sie `ENABLE_TOOL_SEARCH=auto`, um Schemas vorab zu laden, wenn sie in 10 % des Kontextfensters passen, und verschieben Sie nur den Überschuss. Siehe [Tool-Suche konfigurieren](#configure-tool-search) für alle Optionen.

<h3 id="for-mcp-server-authors">
  Für MCP-Server-Autoren
</h3>

Wenn Sie einen MCP-Server erstellen, wird das Feld für Server-Anweisungen mit aktivierter Tool-Suche nützlicher. Server-Anweisungen helfen Claude zu verstehen, wann nach Ihren Tools gesucht werden soll, ähnlich wie [Skills](/docs/de/skills) funktionieren.

Fügen Sie klare, aussagekräftige Server-Anweisungen hinzu, die erklären:

* Welche Kategorie von Aufgaben Ihre Tools verarbeiten
* Wann Claude nach Ihren Tools suchen sollte
* Wichtige Funktionen, die Ihr Server bietet

Claude Code schneidet Tool-Beschreibungen und Server-Anweisungen bei 2 KB ab. Halten Sie sie prägnant, um Kürzungen zu vermeiden, und platzieren Sie kritische Details am Anfang.

<h3 id="configure-tool-search">
  Tool-Suche konfigurieren
</h3>

Die Tool-Suche ist standardmäßig aktiviert: MCP-Tools werden aufgeschoben und bei Bedarf entdeckt. Claude Code deaktiviert sie standardmäßig auf Google Cloud's Agent Platform. Sie ist auch deaktiviert, wenn `ANTHROPIC_BASE_URL` auf einen Host von Drittanbietern verweist, da die meisten Proxys `tool_reference`-Blöcke nicht weiterleiten. Setzen Sie `ENABLE_TOOL_SEARCH` explizit fest, um eine der beiden Fallback-Einstellungen zu überschreiben.

Setting [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/de/env-vars) hält die Tool-Suche aus, und `ENABLE_TOOL_SEARCH` kann dies nicht überschreiben. Die Variable entfernt den Beta-Header, den `defer_loading`-Tool-Definitionen und `tool_reference`-Inhaltsblöcke erfordern.

Die Tool-Suche erfordert ein Modell, das `tool_reference`-Blöcke unterstützt: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 und neuere Modelle. Siehe [Modellkompatibilität in der API-Dokumentation](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) für die aktuelle Liste. Auf Google Cloud's Agent Platform wird die Tool-Suche für Claude Sonnet 4.5 und später sowie Claude Opus 4.5 und später unterstützt.

Steuern Sie das Verhalten der Tool-Suche mit der Umgebungsvariablen `ENABLE_TOOL_SEARCH`:

| Wert            | Verhalten                                                                                                                                                                                                                                                                                            |
| :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (nicht gesetzt) | Alle MCP-Tools werden aufgeschoben und bei Bedarf geladen. Fällt auf das Laden vorab zurück auf Google Cloud's Agent Platform oder wenn `ANTHROPIC_BASE_URL` ein Host von Drittanbietern ist                                                                                                         |
| `true`          | Alle MCP-Tools werden aufgeschoben. Claude Code sendet den Beta-Header auch auf Google Cloud's Agent Platform und durch Proxys. Anfragen schlagen fehl bei Google Cloud's Agent Platform-Modellen älter als Sonnet 4.5 oder Opus 4.5 oder bei Proxys, die `tool_reference`-Blöcke nicht unterstützen |
| `auto`          | Schwellenmodus: Tools werden vorab geladen, wenn sie in 10 % des Kontextfensters passen, andernfalls aufgeschoben                                                                                                                                                                                    |
| `auto:N`        | Schwellenmodus mit benutzerdefiniertem Prozentsatz, wobei `N` 0-100 ist. Beispiel: `auto:5` für 5 %                                                                                                                                                                                                  |
| `false`         | Alle MCP-Tools werden vorab geladen, keine Verschiebung                                                                                                                                                                                                                                              |

```bash theme={null}
# Verwenden Sie eine benutzerdefinierte 5%-Schwelle
ENABLE_TOOL_SEARCH=auto:5 claude

# Deaktivieren Sie die Tool-Suche vollständig
ENABLE_TOOL_SEARCH=false claude
```

Oder legen Sie den Wert im Feld `env` Ihrer [settings.json](/docs/de/settings#available-settings) fest.

Sie können das `ToolSearch`-Tool auch spezifisch deaktivieren:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Einen Server von der Verschiebung ausnehmen
</h3>

Wenn die Tools eines Servers für Claude immer sichtbar sein sollten, ohne einen Suchschritt durchzuführen, setzen Sie `alwaysLoad` in der Konfiguration dieses Servers auf `true`. Jedes Tool von diesem Server wird dann beim Sitzungsstart in den Kontext geladen, unabhängig von der Einstellung `ENABLE_TOOL_SEARCH`. Verwenden Sie dies für eine kleine Anzahl von Tools, die Claude bei jedem Durchgang benötigt, da jedes vorab geladene Tool Kontext verbraucht, der sonst für Ihr Gespräch verfügbar wäre.

Der folgende `.mcp.json`-Eintrag nimmt einen HTTP-Server von der Verschiebung aus, während andere Server aufgeschoben bleiben:

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

Das Feld `alwaysLoad` ist auf allen Server-Typen verfügbar und erfordert Claude Code v2.1.121 oder später. Ein MCP-Server kann auch einzelne Tools als immer geladen markieren, indem `"anthropic/alwaysLoad": true` im `_meta`-Objekt des Tools enthalten ist, was denselben Effekt nur für dieses Tool hat.

Das Setzen von `alwaysLoad: true` blockiert auch den Start, bis sich der Server verbindet, begrenzt auf das Standard-Verbindungs-Timeout von 5 Sekunden. Dies gilt auch, obwohl MCP-Startup ansonsten [standardmäßig nicht blockierend ist](/docs/de/env-vars), da die Tools vorhanden sein müssen, wenn der erste Prompt erstellt wird. Andere Server verbinden sich weiterhin im Hintergrund.

<h2 id="use-mcp-prompts-as-commands">
  MCP-Prompts als Befehle verwenden
</h2>

MCP-Server können Prompts verfügbar machen, die in Claude Code als Befehle verfügbar werden.

<h3 id="execute-mcp-prompts">
  Führen Sie MCP-Prompts aus
</h3>

<Steps>
  <Step title="Entdecken Sie verfügbare Prompts">
    Geben Sie `/` ein, um alle verfügbaren Befehle anzuzeigen, einschließlich derer von MCP-Servern. MCP-Prompts erscheinen mit dem Format `/mcp__servername__promptname`.
  </Step>

  <Step title="Führen Sie einen Prompt ohne Argumente aus">
    ```text theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Führen Sie einen Prompt mit Argumenten aus">
    Viele Prompts akzeptieren Argumente. Übergeben Sie sie durch Leerzeichen getrennt nach dem Befehl:

    ```text theme={null}
    /mcp__github__pr_review 456
    ```

    ```text theme={null}
    /mcp__jira__create_issue "Bug in login flow" high
    ```
  </Step>
</Steps>

<Tip>
  Tipps:

  * MCP-Prompts werden dynamisch von verbundenen Servern entdeckt
  * Argumente werden basierend auf den definierten Parametern des Prompts analysiert
  * Prompt-Ergebnisse werden direkt in das Gespräch eingefügt
  * Server- und Prompt-Namen werden normalisiert, wobei Leerzeichen in Unterstriche umgewandelt werden
</Tip>

<h2 id="managed-mcp-configuration">
  Verwaltete MCP-Konfiguration
</h2>

Für Organisationen, die eine zentralisierte Kontrolle über MCP-Server benötigen, die Benutzer verbinden können, siehe [Verwaltete MCP-Konfiguration](/docs/de/managed-mcp). Sie behandelt die Bereitstellung eines festen Serversatzes mit `managed-mcp.json`, die Einschränkung von Servern mit `allowedMcpServers` und `deniedMcpServers` sowie das, was Benutzer sehen, wenn ein Server blockiert ist.
