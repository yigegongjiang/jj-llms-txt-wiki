> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Enterprise-Netzwerkkonfiguration

> Konfigurieren Sie Claude Code für Enterprise-Umgebungen mit Proxy-Servern, benutzerdefinierten Zertifizierungsstellen (CA) und gegenseitiger Transport Layer Security (mTLS)-Authentifizierung.

Claude Code unterstützt verschiedene Enterprise-Netzwerk- und Sicherheitskonfigurationen über Umgebungsvariablen. Dies umfasst das Routing von Datenverkehr über unternehmenseigene Proxy-Server, das Vertrauen in benutzerdefinierte Zertifizierungsstellen (CA) und die Authentifizierung mit gegenseitigen Transport Layer Security (mTLS)-Zertifikaten für erhöhte Sicherheit.

Legen Sie diese Umgebungsvariablen fest, bevor Sie Claude Code starten. Variablen, die in Ihrer Shell exportiert werden, werden einmalig beim Start gelesen, daher werden später vorgenommene Änderungen an Ihrer Shell-Umgebung von einer laufenden Sitzung nicht übernommen.

<Note>
  Alle auf dieser Seite gezeigten Umgebungsvariablen können auch in [`settings.json`](/docs/de/settings) konfiguriert werden.
</Note>

<h2 id="proxy-configuration">
  Proxy-Konfiguration
</h2>

<h3 id="environment-variables">
  Umgebungsvariablen
</h3>

Claude Code respektiert Standard-Proxy-Umgebungsvariablen. In Claude Desktop-Sitzungen, in denen die App die Anbieterverbindung verwaltet, liest Claude Code diese nur aus verwalteten Einstellungen und `~/.claude/settings.json`; siehe [mTLS-Authentifizierung](#mtls-authentication) für die Bereichsregeln.

```bash theme={null}
# HTTPS-Proxy (empfohlen)
export HTTPS_PROXY=https://proxy.example.com:8080

# HTTP-Proxy (falls HTTPS nicht verfügbar)
export HTTP_PROXY=http://proxy.example.com:8080

# Proxy für spezifische Anfragen umgehen – durch Leerzeichen getrennt
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Proxy für spezifische Anfragen umgehen – durch Komma getrennt
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Proxy für alle Anfragen umgehen
export NO_PROXY="*"
```

Kleingeschriebene Varianten funktionieren auch, und Claude Code verwendet die erste, die in der Reihenfolge `https_proxy`, `HTTPS_PROXY`, `http_proxy`, `HTTP_PROXY` gesetzt ist.

Claude Code sendet seine WebSocket-Verbindungen zu `localhost`, `::1` oder `127.0.0.0/8` niemals über den Proxy, daher benötigen Sie keinen Loopback-Eintrag in `NO_PROXY` für diese.

<Note>
  Claude Code unterstützt keine SOCKS-Proxies.
</Note>

<h3 id="basic-authentication">
  Basis-Authentifizierung
</h3>

Wenn Ihr Proxy eine Basis-Authentifizierung erfordert, fügen Sie Anmeldedaten in die Proxy-URL ein:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Vermeiden Sie das Hardcodieren von Passwörtern in Skripten. Verwenden Sie stattdessen Umgebungsvariablen oder sichere Anmeldedatenspeicherung.
</Warning>

<Tip>
  Für Proxies, die erweiterte Authentifizierung erfordern (NTLM, Kerberos usw.), erwägen Sie die Verwendung eines LLM-Gateway-Dienstes, der Ihre Authentifizierungsmethode unterstützt.
</Tip>

<h2 id="ca-certificate-store">
  CA-Zertifikatspeicher
</h2>

Standardmäßig vertraut Claude Code sowohl seinen gebündelten Mozilla-CA-Zertifikaten als auch dem Zertifikatspeicher Ihres Betriebssystems. Das Lesen des Betriebssystem-Speichers erfordert eine Laufzeit mit `tls.getCACertificates`: Das native Installationsprogramm hat es immer, und npm-Installationen benötigen Node 22.15 oder später. Bei älteren Node-Versionen gelten nur der gebündelte Satz und `NODE_EXTRA_CA_CERTS`. Enterprise-TLS-Inspektions-Proxies funktionieren ohne zusätzliche Konfiguration, wenn ihr Root-Zertifikat im Betriebssystem-Vertrauensspeicher installiert ist und die Laufzeit es lesen kann.

`CLAUDE_CODE_CERT_STORE` akzeptiert eine durch Kommas getrennte Liste von Quellen. Erkannte Werte sind `bundled` für den mit Claude Code ausgelieferten Mozilla-CA-Satz und `system` für den Betriebssystem-Vertrauensspeicher. Der Standard ist `bundled,system`.

Um nur dem gebündelten Mozilla-CA-Satz zu vertrauen:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Um nur dem Betriebssystem-Zertifikatspeicher zu vertrauen:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` hat keinen dedizierten `settings.json`-Schemaschlüssel. Setzen Sie ihn über den `env`-Block in `~/.claude/settings.json` oder direkt in der Prozessumgebung.
</Note>

<h2 id="custom-ca-certificates">
  Benutzerdefinierte CA-Zertifikate
</h2>

Wenn Ihre Enterprise-Umgebung eine benutzerdefinierte CA verwendet, konfigurieren Sie Claude Code so, dass dieser direkt vertraut wird:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  mTLS-Authentifizierung
</h2>

Für Unternehmensumgebungen, die eine Client-Zertifikatauthentifizierung erfordern:

```bash theme={null}
# Client-Zertifikat für die Authentifizierung
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Privater Schlüssel des Clients
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Optional: Passphrase für verschlüsselten privaten Schlüssel
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code liest die Zertifikat- und Schlüsseldateien beim Start und liest sie jedes Mal erneut, wenn es Einstellungen anwendet, z. B. wenn Ihre Organisation den `env`-Block in [verwalteten Einstellungen](/docs/de/server-managed-settings) während einer Sitzung ändert.

Um das Zertifikat und den Schlüssel zu rotieren, ersetzen Sie die Dateien unter denselben Pfaden. Claude Code übernimmt den Austausch in einer laufenden Sitzung ohne Neustart. Wenn eine API-Anfrage mit einem Fehler auf Verbindungsebene fehlschlägt, z. B. bei einem Verbindungsabbruch oder einem TLS-Handshake-Fehler, liest es beide Dateien erneut und versucht die Anfrage mit dem neuen Paar erneut. Vor v2.1.232 las Claude Code bei Verbindungsfehlern nicht erneut, daher behielt es das bereits geladene Paar bei, bis es das nächste Mal Einstellungen anwendete oder Sie es neu starteten.

Claude Code liest die Dateien als Reaktion auf fehlgeschlagene Anfragen erneut, nicht durch Überwachung auf Änderungen:

* **Timing**: Claude Code tut nichts in dem Moment, in dem Sie die Dateien ersetzen. Es präsentiert das neue Paar beim Wiederversuch nach einem qualifizierenden Fehler oder bei der nächsten Anfrage nach dem Anwenden von Einstellungen, je nachdem, was zuerst kommt.
* **Gateway-Ablehnungen**: Claude Code liest erneut, wenn Ihr Gateway die Verbindung zurückgesetzt oder den TLS-Handshake abgelehnt hat, nachdem es das alte Paar nicht mehr akzeptiert. Es liest nicht erneut, wenn das Gateway den Handshake abgeschlossen hat und mit einem HTTP-Fehler antwortet. In diesem Fall lädt Claude Code das neue Paar, wenn es das nächste Mal Einstellungen anwendet oder wenn Sie es neu starten.
* **Unvollständige Rotationen**: Wenn Claude Code erneut liest, während Ihre Rotation noch geschrieben wird, z. B. beim Lesen eines Zertifikats und eines Schlüssels, die nicht zusammenpassen, behält es das vorherige Paar und liest beim nächsten Fehler erneut.
* **OTLP-Telemetrie-Exporter**: Claude Code behält das Zertifikat, das die [Exporter](/docs/de/monitoring-usage#mtls-authentication) beim ersten Gebrauch geladen haben, daher starten Sie Claude Code neu, damit ein rotiertes Zertifikat Ihren Telemetrie-Collector erreicht.
* **Das Neuladen ausschalten**: Setzen Sie [`CLAUDE_CODE_DISABLE_MTLS_RELOAD_ON_STALE_CONNECTION=1`](/docs/de/env-vars#variables), um das Neulesen bei Verbindungsfehlern auszuschalten. Claude Code übernimmt dann rotierte Dateien nur, wenn es das nächste Mal Einstellungen anwendet oder beim nächsten Start.

Um zu bestätigen, dass Claude Code eine Rotation übernommen hat, [starten Sie die Sitzung mit Debug-Protokollierung](#verify-your-configuration) und suchen Sie nach `Stale connection — reloaded rotated mTLS client material` im Protokoll. Claude Code protokolliert diese Zeile nicht, wenn es die Rotation beim Anwenden von Einstellungen übernimmt, daher bedeutet eine fehlende Zeile allein nicht, dass die Rotation fehlgeschlagen ist.

Ersetzen Sie die Dateien, bevor das aktuelle Paar abläuft, damit Claude Code beim nächsten Start kein bereits abgelaufenes Paar lädt.

In [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) verwaltet die Hosting-Umgebung die Verbindung zur API, daher ignoriert Claude Code die folgenden Variablen, wenn sie aus einem `env`-Block einer Einstellungsdatei stammen:

* `CLAUDE_CODE_CLIENT_CERT`
* `CLAUDE_CODE_CLIENT_KEY`
* `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`
* `NODE_EXTRA_CA_CERTS`
* `NODE_TLS_REJECT_UNAUTHORIZED`
* `CLAUDE_CODE_OAUTH_SCOPES`

Claude Code notiert jeden ignorierten Schlüssel im Debug-Protokoll der Sitzung.

In [Claude Desktop](/docs/de/desktop)-Sitzungen, in denen die App die Provider-Verbindung verwaltet, z. B. die Registerkarte „Code" auf einem [Drittanbieter-Provider](/docs/de/third-party-integrations) und Cowork-Sitzungen, liest Claude Code diese Variablen und die Proxy-Variablen `HTTP_PROXY`, `HTTPS_PROXY` und `NO_PROXY` nur aus [verwalteten Einstellungen](/docs/de/managed-settings) und `~/.claude/settings.json`: Es ignoriert sie in den eigenen Einstellungsdateien eines Repositorys, daher kann ein ausgechecktes Repository den TLS- oder Proxy-Pfad einer Sitzung, deren Anmeldedaten von der App stammen, nicht umleiten. In einer lokalen, SSH- oder WSL-Code-Registerkarte-Sitzung, die sich über claude.ai anmeldet, verwaltet die App die Verbindung nicht, und Claude Code liest diese Variablen aus jedem Einstellungsbereich, wie jede Terminal-Sitzung; [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) folgen überall dort, wo Sie sie starten, den Cloud-Sitzungsregeln oben. Vor v2.1.217 ignorierte Claude Code diese Variablen in jeder Einstellungsdatei, wenn die App die Verbindung verwaltete.

<h2 id="verify-your-configuration">
  Überprüfen Sie Ihre Konfiguration
</h2>

Normalerweise erfahren Sie von einer falschen Proxy-Adresse oder einem ungültigen Zertifikatspfad durch einen [Verbindungs- oder Zertifikatsfehler](/docs/de/errors#network-and-connection-errors) bei einer späteren Anfrage, da Claude Code die meisten dieser Einstellungen beim Lesen nicht validiert. Die einzige Einstellung, die beim Start überprüft wird, ist die Proxy-URL: Wenn Claude Code den Wert nicht analysieren kann, z. B. wenn das `http://`-Schema fehlt, stoppt Claude Code den Start mit einem Fehler, der die zu behebende Variable benennt.

Um zu bestätigen, dass Ihre Konfiguration geladen wurde, bevor Sie eine Anfrage senden, starten Sie Claude Code mit Debug-Protokollierung:

```bash theme={null}
claude --debug
```

Die Debug-Ausgabe wird in `~/.claude/debug/<session-id>.txt` statt im Terminal oder in einen Pfad geschrieben, den Sie mit `--debug-file <path>` festlegen. Suchen Sie im Protokoll nach den Zeilen, die bestätigen, dass jede Datei geladen wurde:

```text theme={null}
CA certs: Appended extra certificates from NODE_EXTRA_CA_CERTS (/etc/ssl/certs/corp-ca.pem)
mTLS: Loaded client certificate from CLAUDE_CODE_CLIENT_CERT
mTLS: Loaded client key from CLAUDE_CODE_CLIENT_KEY
```

Wenn Claude Code eine dieser Dateien nicht lesen kann, zeigt das Protokoll stattdessen eine `Failed to read`- oder `Failed to load`-Zeile mit dem Grund an.

Sie können auch `/status` in einer interaktiven Sitzung ausführen und diese Zeilen überprüfen:

* **Proxy**: zeigt die aktive Proxy-URL an und markiert einen Wert, den es nicht analysieren kann, als ungültig und ignoriert.
* **mTLS client cert** und **mTLS client key**: werden nur angezeigt, wenn die Dateien geladen wurden, daher bedeutet eine fehlende Zeile, dass das Laden fehlgeschlagen ist und das Debug-Protokoll den Grund enthält.
* **Additional CA cert(s)**: zeigt den `NODE_EXTRA_CA_CERTS`-Pfad an, ohne zu überprüfen, dass die Datei geladen wurde, daher bestätigen Sie diesen im Debug-Protokoll.

<h2 id="apply-network-settings-to-background-agents">
  Netzwerkeinstellungen auf Hintergrund-Agenten anwenden
</h2>

[Hintergrund-Agenten](/docs/de/agent-view) werden nicht im Terminal ausgeführt, das sie gestartet hat. Ein benutzerspezifischer Supervisor-Prozess wird bei Bedarf gestartet, überlebt Ihre Shell und hostet jede `claude agents`-, `--bg`- und `/background`-Sitzung. Siehe [Wie Hintergrund-Sitzungen gehostet werden](/docs/de/agent-view#how-background-sessions-are-hosted). Dies ändert, wie die Konfiguration auf dieser Seite diese Sitzungen erreicht.

<h3 id="set-network-variables-in-settings-not-the-shell">
  Netzwerkvariablen in Einstellungen festlegen, nicht in der Shell
</h3>

Der Supervisor ist ein Prozess, der von jedem Terminal gemeinsam genutzt wird. Er erbt die Umgebung der Shell, die ihn zuerst startet, und ein vom Betriebssystem installierter Supervisor erhält überhaupt keine Shell-Umgebung. Wenn Sie eine Proxy-, CA-Pfad- oder mTLS-Variable nur in Ihrer Shell exportieren, erreicht sie Hintergrund-Agenten, wenn diese Shell den Supervisor kalt gestartet hat, und erreicht sie stillschweigend nicht, wenn eine andere Shell dies getan hat.

Legen Sie stattdessen die gleichen Variablen im `env`-Block von `~/.claude/settings.json` oder in [verwalteten Einstellungen](/docs/de/settings) fest. Jede Variable auf dieser Seite kann dort festgelegt werden, und Einstellungen sind die einzige Konfiguration, die jede Hintergrund-Sitzung auf jedem Computer erreicht.

<h3 id="configure-a-corporate-launcher-as-a-setting">
  Einen Corporate Launcher als Einstellung konfigurieren
</h3>

Einige Organisationen erfordern, dass jeder Claude Code-Prozess über einen Corporate Launcher gestartet wird, der Sandboxing, Netzwerkkontrollen oder Credential Injection anwendet. Der Supervisor und seine Worker starten Claude Code von einem festen Pfad aus, anstatt `claude` auf `PATH` nachzuschlagen, sodass jeder Hintergrund-Agent einen Wrapper umgeht, den Sie früher auf `PATH` platziert haben.

Legen Sie die Einstellung [`processWrapper`](/docs/de/settings-reference#processwrapper) fest, um den Supervisor, seine Worker und die anderen unter [Was der Launcher abdeckt](/docs/de/corporate-launcher#what-the-launcher-covers) aufgelisteten Hintergrund-Prozesse mit Ihrem Launcher zu präfixieren. Die entsprechende Umgebungsvariable [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/de/env-vars) hat Vorrang, wenn beide festgelegt sind, und unterliegt der gleichen Regel: Liefern Sie sie über verwaltete Einstellungen oder `~/.claude/settings.json`, nicht über einen Shell-Export. [Claude Code hinter einem Corporate Launcher ausführen](/docs/de/corporate-launcher) behandelt den Vertrag, den der Launcher erfüllen muss, was er erreicht und nicht erreicht, und wie Sie ihn bereitstellen.

<Note>
  Ein bereits laufender Supervisor behält die Startkonfiguration, mit der er gestartet wurde. Nach der Bereitstellung der Launcher-Einstellung führen Sie [`claude daemon stop --any`](/docs/de/agent-view#the-supervisor-process) aus, damit der nächste `claude agents` oder `--bg` einen Supervisor startet, der sie berücksichtigt. Ein installierter Service benötigt `claude daemon stop` ohne `--any`.
</Note>

<h2 id="streaming-idle-watchdogs">
  Streaming-Idle-Watchdogs
</h2>

Claude Code führt vier unabhängige Timer aus, die eine Streaming-Modellantwort abbrechen, wenn sie stille wird, sodass eine unterbrochene Verbindung fehlschlägt und erneut versucht wird, anstatt zu hängen. Die First-Byte-Frist deckt das Warten auf Antwortheader ab, bevor etwas von der Antwort angekommen ist. Jeder der anderen drei überwacht eine Live-Antwort auf ein anderes Signal.

| Timer                | Bricht ab, wenn                                                                                                                                                                                                                                          | Läuft auf                                                                                                                                                                                                                                                                                                                                                                             | Standard-Timeout                                                                                              |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------ |
| First-Byte-Frist     | Keine Antwortheader kommen an, nachdem Claude Code die Anfrage sendet                                                                                                                                                                                    | Direkte Anthropic-API und [Claude Platform on AWS](/docs/de/claude-platform-on-aws), einschließlich durch einen HTTPS-Proxy, aber nicht, wenn `ANTHROPIC_BASE_URL` oder `ANTHROPIC_AWS_BASE_URL` sie durch ein [Gateway](/docs/de/gateways) leiten. Opt-in auf Amazon Bedrock mit `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; läuft nicht auf Google Cloud's Agent Platform oder Microsoft Foundry | 180 Sekunden auf der direkten Anthropic-API, 300 Sekunden anderswo, plus eine Sekunde pro 32 KB Anfragekörper |
| Event-Level-Watchdog | Keine Antwortereignisse werden analysiert. Bei Verbindungen, bei denen der Byte-Level-Watchdog läuft, setzen ankommende Bytes, einschließlich Keep-Alive-Pings, auch diesen Watchdog zurück, für bis zu etwa fünf Minuten ohne ein analysiertes Ereignis | Jeder Anbieter                                                                                                                                                                                                                                                                                                                                                                        | 300 Sekunden                                                                                                  |
| Byte-Level-Watchdog  | Keine Bytes kommen auf der Leitung an, einschließlich SSE-Keep-Alive-Pings                                                                                                                                                                               | Direkte Anthropic-API, [Claude Platform on AWS](/docs/de/claude-platform-on-aws), und [Gateway](/docs/de/gateways)-Verbindungen, einschließlich einer benutzerdefinierten `ANTHROPIC_BASE_URL`. Opt-in auf Amazon Bedrock `vnd.amazon.eventstream`-Antworten mit `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; läuft nicht auf Google Cloud's Agent Platform oder Microsoft Foundry                  | 180 Sekunden auf der direkten Anthropic-API, 300 Sekunden anderswo                                            |
| Body-Idle-Timeout    | Keine Bytes kommen für 5 Minuten an                                                                                                                                                                                                                      | Anbieter außer der direkten Anthropic-API und Claude Platform on AWS, es sei denn, [`API_FORCE_IDLE_TIMEOUT`](/docs/de/env-vars) ändert das                                                                                                                                                                                                                                                | 5 Minuten                                                                                                     |

Konfigurieren Sie die Timer mit diesen Variablen, die jeweils in der [Referenz für Umgebungsvariablen](/docs/de/env-vars) detailliert beschrieben sind:

* `CLAUDE_ENABLE_STREAM_WATCHDOG` und `CLAUDE_ENABLE_BYTE_WATCHDOG` erzwingen den entsprechenden Watchdog mit `1` ein oder mit `0` aus, innerhalb der Verbindungen, die die Tabelle auflistet; keine Variable erweitert einen Watchdog auf einen Verbindungstyp, den er nicht abdeckt. `CLAUDE_ENABLE_BYTE_WATCHDOG` auf `0` gesetzt schaltet auch die First-Byte-Frist aus.
* `CLAUDE_STREAM_IDLE_TIMEOUT_MS` setzt das Timeout beider Watchdogs. Claude Code erhöht Werte unter 5 Minuten auf 5 Minuten und begrenzt den Wert auf 30 Minuten für den Byte-Level-Watchdog.
* `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` setzt das Timeout des Byte-Level-Watchdogs, ohne das Timeout des Event-Level-Watchdogs zu ändern, begrenzt auf zwischen 10 Sekunden und 30 Minuten, und hat Vorrang vor `CLAUDE_STREAM_IDLE_TIMEOUT_MS` für diesen Watchdog.
* `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` setzt die First-Byte-Frist direkt. Lassen Sie sie ungesetzt und Claude Code verwendet das Timeout des Byte-Level-Watchdogs, sodass `CLAUDE_STREAM_IDLE_TIMEOUT_MS` und `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` die Frist auch ändern. Für die Begrenzungen, die Upload-Zulage, die `API_TIMEOUT_MS`-Obergrenze und wie lange das Retry nach einem No-Response-Abbruch wartet, siehe [Keine Antwort von API](/docs/de/errors#no-response-from-api).
* `API_FORCE_IDLE_TIMEOUT` auf `0` gesetzt schaltet das Body-Idle-Timeout aus, und auf `1` gesetzt schaltet es für jeden Anbieter ein. Die Watchdogs laufen unabhängig davon, sodass Sie, um einen Stream länger als ihre Schwellwerte pausieren zu lassen, auch diese erhöhen oder deaktivieren müssen.

Wenn ein Watchdog einen stagnierenden Stream abbricht, behandelt Claude Code den Abbruch als einen Mid-Stream-Fehler, und was Sie sehen, hängt davon ab, wie weit die Antwort gekommen war. Claude Code versucht die Anfrage erneut oder beendet den Turn mit einem Fehler, behält die abgeschlossene Ausgabe und zeigt eine [Mitteilung über unvollständige Antwort](/docs/de/errors#the-response-above-may-be-incomplete) an, oder beendet den Turn normal. [Automatische Wiederholungen](/docs/de/errors#automatic-retries) sagt, wo jedes Ergebnis zutrifft.

In einer [nicht-interaktiven Sitzung](/docs/de/headless) und für die Antwort eines Subagenten in jeder Sitzung kann Claude Code zuerst Claude auffordern, die abgeschnittene Antwort fortzusetzen; [der Eintrag dieser Mitteilung](/docs/de/errors#the-response-above-may-be-incomplete) sagt, wann dies der Fall ist und wann Sie die Mitteilung immer noch sehen.

Wenn die First-Byte-Frist abläuft, hat keine Antwort begonnen, daher gibt es keine Teilausgabe zu behalten. Für die Art und Weise, wie Claude Code die Anfrage erneut sendet und wann der Turn stattdessen endet, siehe [Keine Antwort von API](/docs/de/errors#no-response-from-api).

<h2 id="network-access-requirements">
  Anforderungen für Netzwerkzugriff
</h2>

Claude Code benötigt Zugriff auf die folgenden URLs. Fügen Sie diese in Ihrer Proxy-Konfiguration und Firewall-Regeln zur Allowlist hinzu, besonders in containerisierten oder Netzwerken mit eingeschränktem Zugriff. Die Konnektivitätsprüfung beim ersten Start verweist hier hin, wenn sie `api.anthropic.com` oder `platform.claude.com` nicht erreichen kann; siehe [Unable to connect to Anthropic services](/docs/de/errors#unable-to-connect-to-anthropic-services) für die Meldungen der Prüfung und Wiederherstellungsschritte.

| URL                                  | Erforderlich für                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                  | Claude API-Anfragen, einschließlich der WebFetch [Domänensicherheitsprüfung](/docs/de/data-usage#webfetch-domain-safety-check), Feature-Flag-Abrufe und Telemetrie-Ereignisprotokollierung                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `claude.ai`                          | claude.ai-Kontoauthentifizierung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `claude.com`                         | claude.ai-Kontoanmeldung öffnet eine `claude.com`-Seite im Browser, die zu `claude.ai` umleitet; vorab genehmigte WebFetch-Dokumentationssuchen erreichen diesen Host auch von der CLI aus                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `platform.claude.com`                | Anthropic Console-Kontoauthentifizierung. OAuth-Token-Austausch, Aktualisierung und Widerruf gehen auch zu diesem Host für claude.ai-Konten, daher erfordern sowohl Console- als auch claude.ai-Anmeldungen ihn                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `mcp-proxy.anthropic.com`            | [MCP-Konnektoren von claude.ai](/docs/de/mcp#use-mcp-servers-from-claude-ai), einschließlich Konnektoren, die ein Organisationsadministrator konfiguriert. Der Konnektordatenverkehr wird durch diesen Proxy geleitet; Konnektoren sind standardmäßig für claude.ai-authentifizierte Benutzer aktiviert. Um Claude Code daran zu hindern, sie abzurufen, setzen Sie [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/de/env-vars) oder die Einstellung [`disableClaudeAiConnectors`](/docs/de/settings-reference#disableclaudeaiconnectors)                                                                                                                                       |
| `downloads.claude.ai`                | Plugin-Executable-Downloads; nativer Installer, nativer Auto-Updater und Versionsprüfungen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `storage.googleapis.com`             | Plugin-Installationszähler und Metadaten, die in `/plugin` angezeigt werden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `storage.googleapis.com`             | Nativer Installer und nativer Auto-Updater in Versionen vor 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `registry.npmjs.org`                 | Plugin-Installationen (Abrufen von npm-Quell-Plugin-Paketen und Installation von Node.js-Paketabhängigkeiten von Plugins), `npx`-gestartete MCP-Server und die Paketregistrierung für npm- und bun-Installationen von Claude Code selbst                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `bridge.claudeusercontent.com`       | [Claude in Chrome](/docs/de/chrome) Erweiterungs-WebSocket-Bridge                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `*.frame.claudeusercontent.com`      | [Artifact](/docs/de/artifacts) Inhaltslesevorgänge. Die CLI ruft die Dateien eines Artifacts von diesem Host ab, wenn Claude eines öffnet, und nur wenn das Artifact-Tool [verfügbar](/docs/de/artifacts#availability) für Ihr Konto ist. Um das Tool auszuschalten und diese Anforderung zu entfernen, setzen Sie [`"enableArtifact": false`](/docs/de/settings-reference#enableartifact) oder [`CLAUDE_CODE_DISABLE_ARTIFACT=1`](/docs/de/env-vars); Claude Code berücksichtigt auch die veraltete Einstellung [`disableArtifact`](/docs/de/settings-reference#disableartifact). Siehe [Disable artifacts](/docs/de/artifacts#disable-artifacts) für die Interaktion dieser Einstellungen |
| `github.com`                         | Klonen von GitHub-gehosteten [Plugin-Marktplätzen](/docs/de/plugins/overview) und Plugins, einschließlich des offiziellen Anthropic-Marktplatzes, über HTTPS oder SSH. Um GitHub `owner/repo` Quellen nur über HTTPS zu klonen, setzen Sie [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/de/env-vars)                                                                                                                                                                                                                                                                                                                                                                     |
| `raw.githubusercontent.com`          | Changelog-Feed für [`/release-notes`](/docs/de/commands). In interaktiven Sitzungen ruft Claude Code ihn auch beim Start im Hintergrund ab, wenn sein zwischengespeichertes Changelog die laufende Version noch nicht abdeckt, z. B. beim ersten Start nach einem Update; nicht-interaktive und Cloud-Sitzungen rufen ihn nie ab                                                                                                                                                                                                                                                                                                                                   |
| `*-review.googlesource.com`          | Gerrit-Änderungssuche bei `googlesource.com`-Checkouts. Wenn eine Claude Desktop Code-Tab-Sitzung auf einem [vertrauenswürdigen](/docs/de/permissions#project-allow-rules-and-workspace-trust) Checkout startet oder fortgesetzt wird, dessen `origin` ein `googlesource.com`-Host ist, fragt Claude Code diesen Host's `-review`-Server anonym nach der offenen Änderung, die HEADs `Change-Id` entspricht, einmal pro Start oder Fortsetzung. Andere Sitzungstypen überspringen die Suche, und kein anderer Gerrit-Host wird kontaktiert. Optional: deaktivieren mit [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars)                                  |
| `http-intake.logs.us5.datadoghq.com` | Operationale Telemetrie-Ereignisse, die nur gesendet werden, wenn die CLI die Anthropic API direkt nutzt, niemals für Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry. Optional: deaktivieren mit [`DISABLE_TELEMETRY`](/docs/de/data-usage#telemetry-services) oder `DO_NOT_TRACK`                                                                                                                                                                                                                                                                                                                                                           |
| `browser-intake-us5-datadoghq.com`   | Operationale Fehlerberichte, die gesendet werden, wenn die CLI die Anthropic API direkt nutzt und ein serverseitiges Rollout-Gate sie aktiviert. Optional: deaktivieren mit `DISABLE_ERROR_REPORTING` oder `DISABLE_TELEMETRY`; siehe [Telemetry services](/docs/de/data-usage#telemetry-services)                                                                                                                                                                                                                                                                                                                                                                 |
| `formulae.brew.sh`                   | Versionsprüfungen auf Homebrew-Installationen aktualisieren. Andere Installationsmethoden kontaktieren diesen Host nicht                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `code.claude.com`                    | Claude Code-Dokumentationssuchen durch den integrierten claude-code-guide-Agent und vorab genehmigte WebFetch-Anfragen. Das Blockieren dieses Hosts betrifft nur Dokumentationssuchen                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

Wenn Sie Claude Code über npm installieren oder Ihre eigene Binärverteilung verwalten, benötigen Endbenutzer nicht die nativen Installer- und Auto-Updater-Verwendungen von `downloads.claude.ai`, aber npm- und bun-Installationen benötigen ihre Paketregistrierung, `registry.npmjs.org`, es sei denn, Ihre Organisation spiegelt sie. Die anderen Verwendungen in der Tabelle gelten unabhängig von der Installationsmethode.

Die beiden Datadog-Intake-Hosts tragen nur optionale operationale Telemetrie, und das Setzen von [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars) deaktiviert beide. Sitzungen bei Drittanbietern senden niemals an diese Hosts, auch wenn eine Plattform [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/de/env-vars) setzt und Telemetrie-Metriken standardmäßig aktiviert sind. Siehe [Telemetry services](/docs/de/data-usage#telemetry-services) für alles, was Claude Code sendet, und wie Sie es deaktivieren, bevor Sie Ihre Allowlist finalisieren.

Bei Verwendung von [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry) oder einer angemeldeten [Claude apps gateway](/docs/de/claude-apps-gateway) Sitzung gehen Modellverkehr und Authentifizierung stattdessen zu Ihrem Anbieter oder Gateway anstelle von `api.anthropic.com`, `claude.ai` oder `platform.claude.com`. Das WebFetch-Tool ruft immer noch `api.anthropic.com` für seine [Domänensicherheitsprüfung](/docs/de/data-usage#webfetch-domain-safety-check) auf, es sei denn, Sie setzen `skipWebFetchPreflight: true` in [settings](/docs/de/settings).

Beim Routing durch ein [LLM gateway](/docs/de/llm-gateway) mit [`ANTHROPIC_BASE_URL`](/docs/de/llm-gateway-connect#set-the-base-url-and-credential) ruft die [fast mode](/docs/de/fast-mode) Verfügbarkeitsprüfung immer noch `api.anthropic.com` anstelle der Gateway-Basis-URL auf. Die Prüfung berücksichtigt einen konfigurierten HTTP-Proxy, daher ist ein Allowlist-Eintrag für `api.anthropic.com` im Proxy die Lösung, wenn eine Netzwerkblockade die Ursache ist. Eine Netzwerkblockade schlägt die Prüfung nur fehl, wenn der Host selbst durch den Proxy nicht erreichbar ist, und fast mode meldet dann einen Konnektivitätsfehler. Der gleiche Konnektivitätsfehler tritt auf, wenn die Prüfung eine vom Gateway ausgestellte Anmeldedaten präsentiert, die Anthropic ablehnt; Allowlisting hilft dort nicht, da nichts blockiert ist. Siehe [use fast mode behind proxies and LLM gateways](/docs/de/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) für die Variablen, die es wiederherstellen.

<h3 id="organization-ip-allowlists-and-proxy-egress">
  Organisations-IP-Allowlists und Proxy-Ausgang
</h3>

Wenn Ihre Organisation [IP allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) für Claude aktiviert hat, leiten Sie `bridge.claudeusercontent.com` durch den gleichen Proxy-Ausgang wie `claude.ai` und `api.anthropic.com`, z. B. indem Sie es in das gleiche Zscaler-App-Segment oder die gleiche Netskope-Steering-Richtlinie platzieren. Wenn Sie es nicht auf diese Weise leiten können, fügen Sie die Ausgangsadresse, die Ihr Proxy für diesen Host verwendet, zu Ihrer Organisations-IP-Allowlist hinzu, aber nur wenn diese Adresse Ihrer Organisation gewidmet ist: ein gemeinsamer Proxy-Ausgangsbereich lässt auch andere Kunden des Proxy-Anbieters zu.

Anthropic überprüft Verbindungen zu `bridge.claudeusercontent.com` gegen Ihre Organisations-IP-Allowlist unter Verwendung der Adresse, von der sie ankommen. Wenn Ihr Proxy Datenverkehr für diesen Host durch eine Adresse sendet, die nicht auf dieser Allowlist steht, kann Claude Code keine Verbindung zur [Claude in Chrome](/docs/de/chrome) Erweiterung herstellen, obwohl der Rest von Claude Code funktioniert.

<h3 id="github-allow-lists-and-firewalls">
  GitHub-Allowlists und Firewalls
</h3>

[Cloud sessions](/docs/de/claude-code-on-the-web) in von Anthropic gehosteten Umgebungen und [Code Review](/docs/de/code-review) verbinden sich mit Ihren Repositories von verwalteter Anthropic-Infrastruktur aus; Sitzungen in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) verbinden sich von innerhalb Ihres Netzwerks, es sei denn, der Runner entscheidet sich für den [Anthropic git proxy](/docs/de/self-hosted-environments-deploy#use-the-anthropic-git-proxy), der von Anthropics Seite abruft.

Wenn Ihre GitHub Enterprise Cloud-Organisation den Zugriff nach IP-Adresse einschränkt, aktivieren Sie [IP allow list inheritance for installed GitHub Apps](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps) und [add an allow list entry](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) für Anthropics [outbound IP addresses](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses). Die Vererbung deckt nur die Anfragen ab, die die Claude GitHub App als Installation macht, nicht die Anfragen, die sie im Namen Ihrer Benutzer macht. Für andere Firewalls siehe die [Anthropic API IP addresses](https://platform.claude.com/docs/en/api/ip-addresses).

Für selbstgehostete [GitHub Enterprise Server](/docs/de/github-enterprise-server) Instanzen hinter einer Firewall, allowlisten Sie Anthropics [outbound IP addresses](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses), damit Anthropic-Infrastruktur Ihren GHES-Host erreichen kann, um Repositories zu klonen und Review-Kommentare zu posten. Sitzungen in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments-deploy#configure-git) erreichen Ihren GHES-Host stattdessen von innerhalb Ihres Netzwerks, daher gilt diese Exposition nur für von Anthropic gehostete Sitzungen, für gehostete Pre-Session-Flows wie den Repository-Picker und für selbstgehostete Runner, die sich für den [Anthropic git proxy](/docs/de/self-hosted-environments-deploy#use-the-anthropic-git-proxy) entscheiden, der von Anthropics Seite abruft. Für einen GHES-Host, der nur innerhalb Ihres Netzwerks erreichbar ist, trägt der [SCM connector](/docs/de/self-hosted-environments-reference#scm-connector-flags) die gehosteten Pre-Session-Flows stattdessen über eine ausgehende Verbindung, daher ist die Allowlist nicht für sie erforderlich.

<h3 id="desktop-and-claude-ai">
  Desktop und claude.ai
</h3>

Die vorherige Tabelle deckt die eigenständige CLI ab. Die Claude Desktop-App und claude.ai in einem Browser laden ihren Anwendungscode und Benutzerinhalte von zusätzlichen Anthropic CDN-Hosts, einschließlich `assets-proxy.anthropic.com` und der anderen `*.claudeusercontent.com` Ursprünge, die [artifacts](/docs/de/artifacts) in diesen Apps bereitstellen. Das Zulassen von `claude.ai` bei gleichzeitiger Blockierung dieser Hosts führt zu einer leeren Seite statt eines Fehlers. Siehe [network access requirements](/docs/de/desktop#network-access-requirements) auf der Desktop-Seite.

Ein [artifact](/docs/de/artifacts), das eine Schriftart von [Google Fonts](/docs/de/artifacts#improve-the-visual-design) lädt, fordert auch `fonts.googleapis.com` und `fonts.gstatic.com` an. Beide Hosts sind optional. Wenn Sie sie blockieren, werden Artifacts in Fallback-Schriftarten gerendert. Blockieren Sie mit einer schnellen Ablehnung statt eines stillen Verwerfens, damit die Schriftartanfrage sofort fehlschlägt, anstatt das erste Rendering der Seite zu verzögern.

Artifacts können auch JavaScript-Bibliotheken wie React oder ein Charting-Paket von `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` und `unpkg.com` laden und von keinem anderen externen Host. Wenn Sie diese Hosts blockieren, funktionieren die Teile eines Artifacts, die von einer Bibliothek abhängen, nicht, und im Gegensatz zu einer blockierten Schriftart hat eine blockierte Bibliothek keinen Fallback. Blockieren Sie auch hier mit einer schnellen Ablehnung, damit eine blockierte Bibliotheksanfrage sofort fehlschlägt, anstatt zu hängen, bis sie abläuft.

<h2 id="additional-resources">
  Zusätzliche Ressourcen
</h2>

* [Einstellungsdateien und Priorität](/docs/de/settings)
* [Umgebungsvariablen-Referenz](/docs/de/env-vars)
* [Fehlerbehebungsleitfaden](/docs/de/troubleshooting)
