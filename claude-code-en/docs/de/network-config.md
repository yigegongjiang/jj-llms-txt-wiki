> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Enterprise-Netzwerkkonfiguration

> Konfigurieren Sie Claude Code für Enterprise-Umgebungen mit Proxy-Servern, benutzerdefinierten Zertifizierungsstellen (CA) und gegenseitiger Transport Layer Security (mTLS)-Authentifizierung.

Claude Code unterstützt verschiedene Enterprise-Netzwerk- und Sicherheitskonfigurationen über Umgebungsvariablen. Dies umfasst das Routing von Datenverkehr über unternehmenseigene Proxy-Server, das Vertrauen in benutzerdefinierte Zertifizierungsstellen (CA) und die Authentifizierung mit gegenseitigen Transport Layer Security (mTLS)-Zertifikaten für erhöhte Sicherheit.

<Note>
  Alle auf dieser Seite gezeigten Umgebungsvariablen können auch in [`settings.json`](/docs/de/settings) konfiguriert werden.
</Note>

<h2 id="proxy-configuration">
  Proxy-Konfiguration
</h2>

<h3 id="environment-variables">
  Umgebungsvariablen
</h3>

Claude Code respektiert Standard-Proxy-Umgebungsvariablen:

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

Standardmäßig vertraut Claude Code sowohl seinen gebündelten Mozilla-CA-Zertifikaten als auch dem Zertifikatspeicher Ihres Betriebssystems. Das Lesen des Betriebssystem-Speichers erfordert eine Laufzeit mit `tls.getCACertificates`: Das native Installationsprogramm hat es immer, und npm-Installationen benötigen Node 22.15 oder später. Bei älteren Node-Versionen gelten nur der gebündelte Satz und `NODE_EXTRA_CA_CERTS`. Enterprise-TLS-Inspektions-Proxies wie CrowdStrike Falcon und Zscaler funktionieren ohne zusätzliche Konfiguration, wenn ihr Root-Zertifikat im Betriebssystem-Vertrauensspeicher installiert ist und die Laufzeit es lesen kann.

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

Für Enterprise-Umgebungen, die Client-Zertifikat-Authentifizierung erfordern:

```bash theme={null}
# Client-Zertifikat für Authentifizierung
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Privater Schlüssel des Clients
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Optional: Passphrase für verschlüsselten privaten Schlüssel
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code liest die Zertifikat- und Schlüsseldateien beim Start ein und liest sie jedes Mal erneut ein, wenn es Einstellungen anwendet, einschließlich wenn sich die Einstellungen während einer Sitzung ändern. Um das Zertifikat und den Schlüssel zu rotieren, ersetzen Sie die Dateien unter denselben Pfaden.

<h2 id="network-access-requirements">
  Netzwerkzugriffanforderungen
</h2>

Claude Code benötigt Zugriff auf die folgenden URLs. Setzen Sie diese in Ihrer Proxy-Konfiguration und Firewall-Regeln auf die Allowlist, besonders in containerisierten oder eingeschränkten Netzwerkumgebungen.

| URL                            | Erforderlich für                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`            | Claude-API-Anfragen                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `claude.ai`                    | Authentifizierung für claude.ai-Konten                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `platform.claude.com`          | Authentifizierung für Anthropic Console-Konten                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `mcp-proxy.anthropic.com`      | [MCP-Konnektoren von claude.ai](/docs/de/mcp#use-mcp-servers-from-claude-ai), einschließlich Konnektoren, die ein Organisationsadministrator konfiguriert. Der Konnektordatenverkehr wird über diesen Proxy weitergeleitet; Konnektoren sind standardmäßig für claude.ai-authentifizierte Benutzer aktiviert. Zum Deaktivieren setzen Sie [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/de/env-vars) oder die Einstellung [`disableClaudeAiConnectors`](/docs/de/settings#available-settings) |
| `downloads.claude.ai`          | Download von Plugin-Ausführungsdateien; nativer Installer und nativer Auto-Updater                                                                                                                                                                                                                                                                                                                                                                                           |
| `storage.googleapis.com`       | Installationszähler und Plugin-Metadaten, die in `/plugin` angezeigt werden. Signierte [Artifact](/docs/de/artifacts)-Uploads versuchen zuerst diesen Host; die Veröffentlichung fällt auf `api.anthropic.com` zurück, wenn dieser blockiert ist                                                                                                                                                                                                                                  |
| `storage.googleapis.com`       | Nativer Installer und nativer Auto-Updater in Versionen vor 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                          |
| `bridge.claudeusercontent.com` | [Claude in Chrome](/docs/de/chrome) Erweiterungs-WebSocket-Brücke                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `*.claudeusercontent.com`      | Anzeige von [Artifacts](/docs/de/artifacts) auf claude.ai. Der Viewer lädt den Inhalt jedes Artifacts aus einer isolierten Subdomain dieses Ursprungs. Erforderlich im Browser des Viewers, nicht von der CLI selbst                                                                                                                                                                                                                                                              |
| `raw.githubusercontent.com`    | Changelog-Feed für [`/release-notes`](/docs/de/commands) und die Release Notes, die nach dem Update angezeigt werden                                                                                                                                                                                                                                                                                                                                                              |

Wenn Sie Claude Code über npm installieren oder Ihre eigene Binärverteilung verwalten, benötigen Endbenutzer nicht den nativen Installer und der Auto-Updater-Einsatz von `downloads.claude.ai`. Die anderen Verwendungen in der Tabelle gelten unabhängig von der Installationsmethode.

Claude Code sendet standardmäßig optionale operative Telemetrie, die Sie mit Umgebungsvariablen deaktivieren können. Siehe [Telemetrie-Dienste](/docs/de/data-usage#telemetry-services), um zu erfahren, wie Sie diese deaktivieren, bevor Sie Ihre Allowlist finalisieren.

Bei Verwendung von [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry) oder einer angemeldeten [Claude Apps Gateway](/docs/de/claude-apps-gateway)-Sitzung geht der Modell-Datenverkehr und die Authentifizierung zu Ihrem Anbieter oder Gateway statt zu `api.anthropic.com`, `claude.ai` oder `platform.claude.com`. Das WebFetch-Tool ruft weiterhin `api.anthropic.com` für seine [Domain-Sicherheitsprüfung](/docs/de/data-usage#webfetch-domain-safety-check) auf, es sei denn, Sie setzen `skipWebFetchPreflight: true` in [Einstellungen](/docs/de/settings).

[Claude Code im Web](/docs/de/claude-code-on-the-web) und [Code Review](/docs/de/code-review) verbinden sich von der von Anthropic verwalteten Infrastruktur aus mit Ihren Repositories. Wenn Ihre GitHub Enterprise Cloud-Organisation den Zugriff nach IP-Adresse einschränkt, aktivieren Sie [IP-Allowlist-Vererbung für installierte GitHub Apps](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps). Die Claude GitHub App registriert ihre IP-Bereiche, sodass die Aktivierung dieser Einstellung den Zugriff ohne manuelle Konfiguration ermöglicht. Um [die Bereiche stattdessen manuell zu Ihrer Allowlist hinzuzufügen](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) oder um andere Firewalls zu konfigurieren, siehe [Anthropic API IP-Adressen](https://platform.claude.com/docs/en/api/ip-addresses).

Für selbstgehostete [GitHub Enterprise Server](/docs/de/github-enterprise-server)-Instanzen hinter einer Firewall müssen Sie die gleichen [Anthropic API IP-Adressen](https://platform.claude.com/docs/en/api/ip-addresses) auf die Allowlist setzen, damit die Anthropic-Infrastruktur Ihren GHES-Host erreichen kann, um Repositories zu klonen und Review-Kommentare zu posten.

<h3 id="desktop-and-claude-ai">
  Desktop und claude.ai
</h3>

Die vorherige Tabelle behandelt hauptsächlich die eigenständige CLI. Die Claude Desktop-App und claude.ai in einem Browser laden ihren Anwendungscode von zusätzlichen Anthropic CDN-Hosts, einschließlich `assets-proxy.anthropic.com`. Das Zulassen von `claude.ai` bei gleichzeitiger Blockierung dieser Hosts führt zu einer leeren Seite statt zu einem Fehler. Siehe [Netzwerkzugriffanforderungen](/docs/de/desktop#network-access-requirements) auf der Desktop-Seite.

<h2 id="additional-resources">
  Zusätzliche Ressourcen
</h2>

* [Claude Code-Einstellungen](/docs/de/settings)
* [Referenz für Umgebungsvariablen](/docs/de/env-vars)
* [Leitfaden zur Fehlerbehebung](/docs/de/troubleshooting)
