> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bereitstellung und Betrieb des Claude-Apps-Gateways

> Registrieren Sie das Gateway bei Ihrem IdP, erstellen Sie den Container, stellen Sie ihn auf Kubernetes oder Cloud Run bereit, und betreiben Sie ihn: Integritätsprüfungen, Geheimnisrotation, Upgrades und Sicherheit.

Diese Seite behandelt den operativen Aspekt des Betriebs des [Claude-Apps-Gateways](/docs/de/claude-apps-gateway): Registrierung eines OAuth-Clients bei Ihrem Identitätsanbieter (IdP), Bereitstellung des Gateways als Container und täglicher Betrieb. Für jede Option in der Datei `gateway.yaml`, die das Gateway beim Start liest, siehe die [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config).

Eine Produktionsbereitstellung folgt vier Schritten in der richtigen Reihenfolge, und die folgenden Abschnitte entsprechen ihnen. Die ersten beiden sind Stellen, an denen Sie Entscheidungen treffen; die zweiten beiden sind Referenzmaterial, das Sie konsultieren können, sobald es läuft.

1. [Richten Sie Ihren Identitätsanbieter ein](#identity-provider-setup): Registrieren Sie den OAuth-Client und überprüfen Sie die IdP-spezifischen Hinweise für Okta, Entra und Google
2. [Stellen Sie das Gateway bereit](#deployment): Erstellen Sie ein gepinntes Container-Image und führen Sie es auf Kubernetes, Cloud Run oder Ihrer eigenen Plattform aus. Dieser Abschnitt behandelt auch Kosten-, Bypass-, Multi-Gateway- und Serverless-Entscheidungen
3. [Richten Sie den Betrieb ein](#operations): Protokolle, Integritätsprüfungen, Ausfallverhalten, Geheimnisrotation und Upgrades. Referenzmaterial für die Einrichtung von Überwachung und Runbooks
4. [Überprüfen Sie die Sicherheitslage](#security): Welche Daten wohin fließen, das Bedrohungsmodell und Compliance-Antworten. Referenzmaterial für eine Sicherheitsüberprüfung

Wenn ein Anmelde- oder Startfehler auftritt, gehen Sie direkt zu [Fehlerbehebung](#troubleshooting), das nach dem Fehler, den Sie sehen, indiziert ist.

<Note>
  **Stellen Sie auf Ihrem privaten Netzwerk bereit.** Claude Code stellt nur eine Verbindung zu einem Gateway her, dessen Adresse privat ist. Dies ist eine Sicherheitsmaßnahme, da ein vertrauenswürdiges Gateway Einstellungen pushen kann, die Befehle auf Entwicklermaschinen ausführen. Platzieren Sie das Gateway hinter einem internen Load Balancer oder VPN und geben Sie ihm einen Hostnamen, der nur zu privaten IPs aufgelöst wird.

  Anthropic-betriebene öffentliche Gateway-Endpunkte sind die Ausnahme: `/login` akzeptiert sie über `https://`. Dies ist eine kleine feste Menge von Gateways, die Anthropic selbst betreibt; sie sind keine Bereitstellungsoption, die Sie auswählen oder konfigurieren können. Die Liste ist in Claude Code kompiliert, daher kann keine Konfiguration einen Hostnamen hinzufügen und kein von Ihnen gehostetes Gateway qualifiziert sich für die Ausnahme. Vor v2.1.206 lehnte `/login` diese Endpunkte wie jede andere öffentliche Adresse ab.
</Note>

<h2 id="identity-provider-setup">
  Identitätsanbieter-Setup
</h2>

Registrieren Sie eine vertrauliche OAuth/OpenID Connect (OIDC) Webanwendung mit einem einzelnen Redirect-URI, `https://<gateway>/oauth/callback`, und weisen Sie sie den Benutzern oder Gruppen zu, die Zugriff auf das Gateway haben sollten.

Jeder OIDC-konforme IdP funktioniert: Okta, Microsoft Entra ID, Google Workspace, Keycloak, Dex, PingFederate und andere. Der IdP muss drei Anforderungen erfüllen:

* Stellt `/.well-known/openid-configuration` über HTTPS in der Produktion bereit; das Gateway akzeptiert einen [`http://` Aussteller](/docs/de/claude-apps-gateway-config#oidc), und ein Loopback-Aussteller erfordert zusätzlich `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`
* Unterstützt den Authorization-Code-Flow. PKCE (Proof Key for Code Exchange) ist standardmäßig aktiviert; deaktivieren Sie es mit `oidc.use_pkce: false` für IdPs, die es nicht unterstützen
* Gibt `email` und optional `groups` im id\_token zurück, oder stellt sie vom Userinfo-Endpoint mit `oidc.userinfo_fallback: true` bereit

Für private PKI setzen Sie `oidc.ca_cert_pem`.

Einige Anbieter handhaben E-Mail- und Gruppen-Claims unterschiedlich:

* **Okta**: Der Org-Autorisierungsserver unter `https://example.okta.com` gibt einen dünnen id\_token zurück, der `email` und `groups` auslässt, daher setzen Sie `oidc.userinfo_fallback: true`, wenn Sie ihn als `issuer` verwenden. Ein benutzerdefinierter Autorisierungsserver wie `https://example.okta.com/oauth2/default`, der `email` und optional `groups` im id\_token enthält, gibt sie direkt aus und benötigt keinen Fallback. Okta gibt `groups` nur aus, wenn der `groups`-Scope in `oidc.scopes` angefordert wird und der Gruppen-Claim-Filter der App dies zulässt; `userinfo_fallback` kann einen Claim nicht ausfüllen, nach dem der IdP nicht gefragt wurde.
* **Microsoft Entra ID**: `issuer` = `https://login.microsoftonline.com/<tenant-id>/v2.0`. Entra gibt Gruppen-Object-IDs statt Namen aus, daher verwenden Sie die GUIDs in `managed.policies.match.groups`, oder verwenden Sie App-Rollen für lesbare Namen. Wenn Ihr Mandant Rollen unter `roles` statt `groups` ausgibt, setzen Sie `oidc.groups_claim: roles`.
* **Google Workspace**: `issuer` = `https://accounts.google.com`. Googles id\_token enthält keine Gruppen. Um gruppenbasierte `allowed_groups` oder `managed.policies` mit Google als IdP zu verwenden, konfigurieren Sie [`oidc.google_groups`](/docs/de/claude-apps-gateway-config#oidc), das die Gruppen jedes Benutzers über die Admin SDK Directory API mit einem Service-Account mit Domain-Wide-Delegation nachschlägt. Ohne dies verwenden Sie `oidc.allowed_email_domains` für Mitgliedschafts-Gating und `managed.policies.match.email_domain` für Richtlinienzuweisung. Google ignoriert auch den Standard-Scope `offline_access`. Für Refresh-Tokens setzen Sie `oidc.scopes: [openid, profile, email]` und `oidc.extra_auth_params: { access_type: offline, prompt: consent }`.

Für Unterstützung bei einem oben nicht behandelten Identitätsanbieter siehe [Troubleshooting](#troubleshooting).

<Warning>
  Refresh-Tokens ermöglichen es dem Gateway, die Sitzung eines Entwicklers stillschweigend zu erneuern, ohne den Entwickler zum Browser zu senden. Sie treiben auch die Deprovisioning an, denn wenn der IdP einen Benutzer deaktiviert, schlägt die nächste Aktualisierung fehl und die Sitzung endet innerhalb von `ttl_hours`. Das Gateway fordert standardmäßig `offline_access` an, um einen Refresh-Token zu erhalten. Wenn Ihr IdP explizite Zustimmung für Offline-Zugriff erfordert, konfigurieren Sie den OAuth-Client, um dies zu ermöglichen.

  Wenn Ihr IdP überhaupt keine Refresh-Tokens ausstellen kann, funktioniert das Gateway immer noch, aber es gibt keine stille Erneuerung, daher führen Entwickler die Browser-Anmeldung erneut aus, wenn ihre Sitzung abläuft. Um zu verhindern, dass dies jede Stunde geschieht, erhöhen Sie [`session.ttl_hours`](/docs/de/claude-apps-gateway-config#session) auf `8` oder `12`. Der Kompromiss ist die Deprovisioning-Latenz, denn ohne Refresh-Tokens behält ein deaktivierter Benutzer Zugriff, bis die längere TTL abläuft.
</Warning>

<h2 id="deployment">
  Bereitstellung
</h2>

Das Gateway ist eine einzelne Linux-Binärdatei. Es skaliert horizontal, da Replikationen zustandslos sind und Postgres die gemeinsame Koordinierungsebene ist. Führen Sie es so aus, wie Sie zustandslose Dienste in Ihrer Umgebung ausführen. Der Rest dieses Abschnitts beschreibt, was das Image benötigt, mit kurzen Hinweisen für Kubernetes und Cloud Run.

Das Gateway ist für die Ausführung in Ihrem Netzwerk konzipiert, da es Ihre Upstream-Anmeldedaten hält und als einzelner Egress-Punkt für Inferenz fungiert. Es kann überall dort ausgeführt werden, wo Ihre Entwickler und Ihr IdP über HTTPS erreichbar sind; behandeln Sie es wie jeden anderen Dienst, der eine Produktionsanmeldedaten hält.

Einige Entscheidungen prägen die Bereitstellung über den Ort hinaus, an dem sie läuft:

* **Kosten**: Es gibt keine separate Lizenz oder Pro-Sitz-Gebühr für das Gateway; es ist Teil der `claude`-Binärdatei. Sie zahlen für Inferenz über Ihr bestehendes Cloud- oder Anthropic-Engagement, plus die Berechnung für den Container und Ihren Telemetrie-Collector.
* **Bypass**: Das Gateway erzwingt nicht, dass die einzige Route zu einem Modell durch es führt. Ein Entwickler mit seinen eigenen Anmeldedaten kann den Anbieter immer noch direkt aufrufen, daher ist das Schließen dieses Pfads eine Netzwerkrichtlinien-Entscheidung, z. B. das Blockieren von Egress zu `api.anthropic.com` außer vom Gateway. Das Blockieren dieses Egress bricht auch die [WebFetch-Domain-Sicherheitsprüfung](/docs/de/data-usage#webfetch-domain-safety-check), die `api.anthropic.com` von jeder Entwicklermaschine aufruft; setzen Sie `skipWebFetchPreflight: true` in der verwalteten Richtlinie, um sie zu deaktivieren.
* **Mehrere Gateways**: Jedes Gateway ist eine separate Bereitstellung mit seiner eigenen Konfiguration. Die CLI speichert ihren Vertrauens-Fingerprint und ihre Anmeldedaten pro Gateway-Hostname, daher können verschiedene Teams sich mit verschiedenen Gateways verbinden, ohne Konflikte. Um mehrere OIDC-Aussteller zu bedienen, führen Sie separate Instanzen aus.
* **Serverless**: Cloud Run funktioniert; setzen Sie `min-instances: 1`, um kalte OIDC-Erkennung zu vermeiden. Lambda und Cloud Functions funktionieren nicht, da das Gateway ein langlebiger HTTP-Server ist.

Jede Produktionstopologie hier platziert einen L7-Proxy, wie einen Ingress, Cloud Runs Front-End oder einen ALB, vor einfachen HTTP-Replikationen. Setzen Sie [`listen.trusted_proxies`](/docs/de/claude-apps-gateway-config#listen) auf die Quellbereiche des Proxys, damit das Gateway Client-IPs aus `X-Forwarded-For` liest. Das Gateway ehrt den Header nur, wenn der TCP-Peer vertrauenswürdig ist; das [Google Cloud-Beispiel](/docs/de/claude-apps-gateway-on-gcp) hat konkrete Werte pro Topologie. Ohne vertrauenswürdige Proxys scheint jede Anfrage von der IP des Proxys zu kommen, was Pro-IP-Ratenlimits in einen gemeinsamen Bucket zusammenfasst und die IP des Proxys in Audit-Events aufzeichnet.

<h3 id="container-image">
  Container-Image
</h3>

Erstellen Sie Ihr eigenes Image um die native `claude`-Binärdatei aus der Standard-Claude-Code-Version:

1. Laden Sie den Linux-Build für Ihre Image-Architektur aus einer gepinnten Version herunter; siehe [Installieren Sie eine bestimmte Version](/docs/de/setup#install-a-specific-version) für die Download-URL.
2. Überprüfen Sie es gegen die GPG-signierte `manifest.json` der Version, wie in [Binäre Integrität und Code-Signierung](/docs/de/setup#binary-integrity-and-code-signing) beschrieben.
3. Kopieren Sie es in den Build-Kontext.

Spiegeln Sie die Version in Ihre interne Registry, wenn Ihre Builds den Release-Host nicht erreichen können, und pinnen Sie die Version, die Ihre Flotte ausführt.

Über die Binärdatei hinaus benötigt das Image:

* **Ein glibc-basiertes Image**: Die einzigen dynamischen Abhängigkeiten des glibc-Builds sind glibc-Bibliotheken. Musl-basierte Images benötigen den `linux-x64-musl`- oder `linux-arm64-musl`-Build plus zusätzliche Pakete; siehe [Alpine-Linux-Setup](/docs/de/setup#alpine-linux-and-musl-based-distributions).
* **Ein beschreibbares Zustandsverzeichnis**: Das Gateway läuft als jeder Benutzer, aber minimale Images haben kein beschreibbares Home. Setzen Sie `CLAUDE_CONFIG_DIR` auf einen beschreibbaren Pfad wie `/tmp/.claude`.
* **Der Container-Befehl**: `claude gateway --config /etc/claude/gateway.yaml`, mit der Konfigurationsdatei schreibgeschützt eingebunden und Geheimnissen als Umgebungsvariablen bereitgestellt; das Gateway lauscht auf `listen.port`, Standard `8080`.

<h3 id="kubernetes">
  Kubernetes
</h3>

Führen Sie das Gateway als Deployment aus, wie jeden zustandslosen Dienst:

* Binden Sie die Konfiguration von einer ConfigMap und Geheimnisse von einem Secret ein; referenzieren Sie Geheimnisse in der YAML über `${file:/path/to/secret}` oder als Umgebungsvariablen
* Beenden Sie TLS am Ingress und setzen Sie `listen.public_url` auf den Ingress-Hostnamen
* Zeigen Sie die Readiness-Probe auf `GET /readyz` und die Liveness-Probe auf `GET /healthz`

<Note>
  **Workload-Identität**

  Bevorzugen Sie die Workload-Identität der Plattform gegenüber statischen Schlüsseln: IRSA auf EKS für Bedrock und für Claude Platform auf AWS, Workload Identity auf GKE für Agent Platform und Workload Identity auf AKS für Foundry. Setzen Sie `auth: {}` im Upstream-Block oder `use_azure_ad: true` für Foundry, und das Gateway nimmt die Identität des Pods durch die Standard-Credential-Chain dieses Anbieters auf. Für eine Cloud-übergreifende Kopplung, wie z. B. einen Bedrock-Upstream auf GKE, setzen Sie explizite Anmeldedaten im `auth`-Block des Upstream statt. Die [`upstreams`-Referenz](/docs/de/claude-apps-gateway-config#upstreams) hat Setup-Details pro Plattform.
</Note>

<h3 id="cloud-run">
  Cloud Run
</h3>

Konfigurieren Sie den Dienst wie folgt:

* Lassen Sie `listen.port` bei seinem Standard von `8080`, das Cloud Runs Standard-`PORT` entspricht, oder setzen Sie `port: ${PORT}`
* Setzen Sie `public_url` auf den extern erreichbaren Ursprung. Für die Produktion ist dies normalerweise der Hostname eines internen Load Balancers, da `/login` [öffentliche Adressen ablehnt](/docs/de/claude-apps-gateway#prerequisites) und die `*.run.app`-URL zu einer aufgelöst wird, daher funktioniert die Cloud-Run-URL allein nur für einen `curl`- oder Browser-Smoke-Test. Die Ausnahme ist ein Netzwerk, in dem `*.run.app` privat über Private Service Connect und eine Cloud-DNS-Private-Zone aufgelöst wird; in dieser Topologie ist die Cloud-Run-URL eine gültige `public_url`. Das [Google-Cloud-Beispiel](/docs/de/claude-apps-gateway-on-gcp#deploy-the-gateway) behandelt beide.
* Binden Sie die Konfiguration als Secret-Volume ein
* Setzen Sie `min-instances: 1`, um kalte OIDC-Erkennung bei der ersten Anfrage zu vermeiden

<Note>
  Für ein vollständiges Beispiel auf Google Cloud, das Cloud Run oder GKE, Cloud SQL und Secret Manager abdeckt, siehe [Bereitstellung auf Google Cloud](/docs/de/claude-apps-gateway-on-gcp).
</Note>

<h3 id="push-the-gateway-url-to-developer-machines">
  Pushen Sie die Gateway-URL zu Entwicklermaschinen
</h3>

Sobald das Gateway bedient wird, pushen Sie `forceLoginMethod` und `forceLoginGatewayUrl` über verwaltete Einstellungen auf jede Entwicklermaschine, über MDM oder durch direktes Schreiben der pro-OS `managed-settings.json`. Ohne dies zeigt `/login` den Standard-Account-Picker ohne Gateway-Option. Siehe [Client-seitige verwaltete Einstellungen](/docs/de/claude-apps-gateway-config#client-side-managed-settings) für die Dateipfade.

<h2 id="operations">
  Betrieb
</h2>

Sobald das Gateway Verkehr bedient, ist der tägliche Betrieb das Lesen seiner Protokolle, das Prüfen seiner Integrität und das Rotieren seiner Geheimnisse nach Ihrem Zeitplan. Die Unterabschnitte behandeln jeweils, plus was Postgres hält und wie Upgrades und Rollbacks sich verhalten.

<h3 id="logs">
  Protokolle
</h3>

Das Gateway schreibt zwei Streams zu stderr, beide JSON-freundlich:

* **Audit-Events**: einzeilige JSON pro sicherheitsrelevantes Event. Leiten Sie stderr an Ihren Log-Aggregator. Die ausgegebenen Events umfassen `config.load`, `session.mint`, `session.refresh`, `device.authorize`, `device.verify`, `auth.denied`, `access.denied`, `inference`, `managed.serve`, `spend.blocked` und `admin.denied`. Felder variieren je nach Event:
  * Erfolgreiche Mint- und Refresh-Events tragen `sub`, `email`, `client_ip` und das Ergebnis
  * Denial-Events tragen den Grund, Pfad und Client-IP, da bei Denial keine Identität existiert
  * `inference` zeichnet auf, welcher Upstream die Anfrage bedient hat und den Antwortstatus
  * `admin.denied` zeichnet einen abgelehnten Admin-API-Auth-Versuch mit dem Grund (`invalid_key` oder `no_credentials`), Client-IP, Methode und Pfad auf, ohne das präsentierte Schlüsselmaterial
* **Operationale Protokolle**: lesbare `[gateway]`-präfixierte Zeilen für Boot, Warnungen und Upstream-Fehler. Die Umgebungsvariable `CLAUDE_GATEWAY_LOG_LEVEL` steuert die Ausführlichkeit und akzeptiert `info`, `warn` oder `error`, mit `info` als Standard. Sie beeinflusst keine Audit-Events, die immer ausgegeben werden.

<h3 id="health">
  Integrität
</h3>

Das Gateway bedient `GET /healthz` als Liveness-Probe und `GET /readyz` als Readiness-Probe; `/readyz` überprüft, dass der Store erreichbar ist. Beide sind von `access_control.allow_cidrs` ausgenommen, daher funktionieren Proben auf einem abgesperrten Listener.

Das OAuth-Discovery-Dokument unter `/.well-known/oauth-authorization-server` gibt auch `200` nur zurück, nachdem Konfigurationslast, OIDC-Erkennung, Upstream-Client-Konstruktion und Postgres-Migration alle erfolgreich sind, daher dient es auch als End-to-End-Boot-Prüfung.

Ein laufendes Gateway bedient auch eine Beschreibung der Pfade und Anfragefiguren, die es unter `<public_url>/protocol` akzeptiert, abgestimmt auf die Version, die Sie ausführen. Der Inhalt ist nicht stabil über Releases hinweg.

<h3 id="outage-behavior">
  Ausfallverhalten
</h3>

Wenn Postgres ausfällt, bedient das Gateway selbst weiterhin angemeldete Entwickler und neue Anmeldungen schlagen fehl. Ob Entwickler tatsächlich weiterarbeiten, hängt davon ab, wie Ihr Orchestrator die Readiness handhabt:

* **Bestehende Sitzungen**: Bearer-Tokens validieren lokal mit dem JWT-Geheimnis, Sitzungs-Refreshes berühren den Store nicht, und der Gateway-Prozess kann immer noch Inferenz bedienen
* **Neue Anmeldungen**: schlagen fehl, bis Postgres wiederhergestellt ist, da der Device-Flow und seine Rate-Limit-Zähler in Postgres leben
* **[Spend-Limit-Durchsetzung](/docs/de/claude-apps-gateway-spend-limits#postgres-availability)**: schlägt während des Ausfalls offen fehl, daher fließt Inferenz weiterhin; schalten Sie es auf Fail-Closed um, wenn Sie lieber blockieren als ungemessen laufen möchten
* **Readiness**: `/readyz` meldet während des Ausfalls nicht bereit, daher entfernen Orchestratoren, die Verkehr auf Readiness gating, jede Replikation auf einmal aus der Rotation. In dieser Topologie schlägt der gesamte Verkehr, einschließlich Inferenz, die das Gateway immer noch bedienen könnte, beim Load Balancer fehl, bis Postgres wiederhergestellt ist. Die Liveness-Probe auf `/healthz` bleibt bestehen, daher werden Replikationen nicht neu gestartet. Zeigen Sie die Readiness-Probe statt auf `/healthz`, wenn Sie lieber möchten, dass angemeldete Entwickler durch einen Store-Ausfall weiterarbeiten; die Kosten sind, dass neue Anmeldungen gegen eine Replikation fehlschlagen, die immer noch bereit meldet.

Wenn Ihr IdP ausfällt, funktionieren bestehende Sitzungen bis `ttl_hours`, und neue Anmeldungen und Refreshes schlagen fehl. Setzen Sie eine längere `ttl_hours`, wenn Ihr IdP häufige Wartungsfenster hat.

<h3 id="jwt-secret-rotation">
  JWT-Geheimnisrotation
</h3>

Rotieren Sie das Signierungsgeheimnis in drei Schritten, damit bestehende Sitzungen gültig bleiben:

1. Generieren Sie ein neues Geheimnis. Stellen Sie es dem `session.jwt_secret`-Array voran.
2. Rollen Sie die Bereitstellung aus. Neue Tokens signieren mit dem neuen Geheimnis; alte Tokens validieren immer noch.
3. Nach `ttl_hours` plus einer Marge entfernen Sie das alte Geheimnis und rollen erneut aus.

Rotation ist auch die einzige Möglichkeit, Sitzungen vor Ablauf zu erzwingen: Bearer-Tokens validieren lokal gegen das JWT-Geheimnis, daher gibt es keine Pro-Sitzungs-Sperrung. Das Ersetzen des Geheimnisses direkt, ohne das alte in dem Array zu behalten, invalidiert jede ausstehende Sitzung auf einmal. Für einzelnes Offboarding deprovisioning Sie den Benutzer in Ihrem IdP; ihre Sitzung endet innerhalb von `ttl_hours`.

<h3 id="postgres">
  Postgres
</h3>

Das Gateway hält fünf Tabellen, alle erstellt durch seine Boot-Zeit-Migrationen:

| Tabelle            | Inhalt                                                            | Aufbewahrung                                                        |
| ------------------ | ----------------------------------------------------------------- | ------------------------------------------------------------------- |
| `kv`               | Device-Grants (10-Minuten-TTL) und Rate-Limit-Zähler              | TTL pro Zeile                                                       |
| `spend`            | Pro-Principal-Periode-bis-Datum-Spend-Zähler, in Cent             | `admin.spend_retention_months`, Standard 13                         |
| `spend_limits`     | Konfigurierte Spend-Caps                                          | Bis gelöscht über die API                                           |
| `admin_audit`      | Admin-API-Mutations-Trail                                         | `admin.audit_retention_days`, Standard 365                          |
| `principal_emails` | E-Mail, Anzeigename und IdP-Gruppen jedes Principal. Enthält PII. | `admin.identity_retention_days` seit letzter Aktivität, Standard 90 |

Eine 30-Sekunden-Schleife läuft `kv`-Zeilen ab ihrer TTL ab, und ein stündlicher Sweep erzwingt die Aufbewahrungsfenster auf den Spend-Tabellen, daher wächst nichts ohne Grenzen. Ohne [Spend-Limits](/docs/de/claude-apps-gateway-spend-limits) konfiguriert, wird nur `kv` geschrieben. Wenn Ihre Sicherheitsrichtlinie DDL von der Anwendungsrolle verbietet, erstellen Sie diese Tabellen und `_migrations` vorab mit einer Admin-Rolle und gewähren Sie der App-Rolle `SELECT, INSERT, UPDATE, DELETE` auf jeder.

Mit Spend-Limits in Verwendung bedeutet eine verlorene Datenbank verlorene Spend-Verfolgung und Caps, nicht nur Entwickler-Neu-Anmeldungen, daher führen Sie regelmäßige Backups durch. Um einen abgegangenen Entwickler sofort zu löschen, statt auf Aufbewahrung zu warten, führen Sie `DELETE FROM principal_emails WHERE principal = '<sub>'` direkt aus; das entfernt die einzige Tabelle, die ihre E-Mail, ihren Namen und ihre Gruppen hält. `spend`- und `admin_audit`-Zeilen referenzieren nur das pseudonyme OIDC-`sub`.

<h3 id="upgrades">
  Upgrades
</h3>

Replikationen sind zustandslos, daher ist ein Rolling Restart jederzeit sicher. Das Gateway führt Schema-Migrationen beim Start aus, was bedeutet, dass die Bereitstellung der neuen Binärdatei die Datenbank selbst migriert. Wenn die Datenbankrolle DDL nicht ausführen kann, erstellen Sie das Schema vorab, einschließlich der `_migrations`-Tabelle, die auf die aktuelle Version gesät ist; andernfalls schlägt der Boot beim Versuch `CREATE TABLE` fehl.

Migrationen sind nur Anhänge, daher ist das Rollback zu einer früheren Binärdatei, die weniger Migrationen kennt, sicher; es ignoriert die zusätzlichen Zeilen. Rollback validiert auch die YAML erneut gegen das ältere Binärdatei-Schema, daher schlägt eine Konfiguration, die einen Schlüssel angenommen hat, der von der neueren Version eingeführt wurde, beim Boot auf der älteren fehl. Entfernen Sie den neuen Schlüssel vor dem Rollback.

Da Sie die Gateway-Version in Ihrem eigenen Image pinnen, erreichen Fixes in neuen Claude Code-Releases, einschließlich Sicherheits-Fixes, Ihre Bereitstellung nur, wenn Sie den Pin aktualisieren und erneut bereitstellen. Beziehen Sie das Gateway in denselben Patching-Zyklus ein, den Sie für andere Dienste verwenden, die Produktionsanmeldedaten halten.

<h2 id="security">
  Sicherheit
</h2>

Dieser Abschnitt beantwortet die Fragen, die eine Sicherheitsüberprüfung stellt: Welche Daten fließen durch das Gateway und wohin sie gehen, welche Angriffe das Design abwehrt, und welche Antworten in einen Compliance-Fragebogen gehören.

<h3 id="data-flow">
  Datenfluss
</h3>

| Daten                                                                                                     | Pfad                                                             | Vom Gateway an Anthropic gesendet                          |
| --------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------- |
| Inferenz (Prompts, Completions)                                                                           | CLI → Gateway → Ihr Upstream                                     | Nur wenn die Anthropic-API ein konfigurierter Upstream ist |
| Telemetrie (OTLP-Metriken, plus [Opt-in-Protokolle und Traces](/docs/de/claude-apps-gateway-config#telemetry)) | CLI → Gateway → Ihr Collector                                    | Nie                                                        |
| Identität (E-Mail, Gruppen, Sub)                                                                          | IdP → Gateway → JWT → CLI; die CLI stempelt sie auf OTLP-Exporte | Nie                                                        |
| Verwaltete Einstellungen                                                                                  | Ihre Gateway-YAML → CLI                                          | Nie                                                        |
| Audit-Protokoll                                                                                           | Gateway-Stderr → Ihr Aggregator                                  | Nie                                                        |

<h3 id="threat-model-summary">
  Bedrohungsmodell-Zusammenfassung
</h3>

Das Gateway sitzt innerhalb Ihres Netzwerk-Perimeters, aber einzelne Entwickler-Laptops werden nicht als vertrauenswürdig behandelt. Das Design berücksichtigt dies auf drei Arten:

* Entwickler halten kurzlebige JWTs statt roher Upstream-Schlüssel. Das CLI-zu-Gateway-Bein verwendet den RFC-8628-Device-Grant, und der Gateway-Autorisierungs-Code-Austausch mit dem IdP führt PKCE in der Standard-Konfiguration aus, daher ist ein abgefangener IdP-Autorisierungs-Code nutzlos.
* Die Device-Verifikationsseite erzwingt Same-Origin-POST und ein Pro-IP-Rate-Limit pro RFC 8628 §5.1. Siehe [User-Code-Brute-Force-Resistenz](#user-code-brute-force-resistance).
* Ausgehende Anfragen gehen durch einen Server-Side-Request-Forgery (SSRF)-Guard, der DNS auflöst, Link-Local- und Cloud-Metadata-Adressen plus Loopback standardmäßig blockiert und die Verbindung zur aufgelösten IP pinnt, daher können Operator-beeinflusste URLs wie der IdP und OTLP-Ziele nicht zu Cloud-Metadata-Endpoints umgeleitet werden. RFC-1918-Private-Bereiche sind absichtlich erlaubt, da IdPs und OTLP-Collector häufig auf privaten IPs leben. Für lokale Entwicklung gegen einen Loopback-IdP oder Collector setzen Sie `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` in der Gateway-Umgebung; lassen Sie es in der Produktion ungesetzt.

Wenn Sie Ihre eigenen Egress-Kontrollen hinzufügen, muss das Gateway den Metadata-Server erreichen, wenn es Instanz-Metadata-Anmeldedaten wie Workload-Identität verwendet.

Zwei Bedrohungen sind außerhalb des Geltungsbereichs, da sie Ihre Infrastruktur sind, um zu sichern:

* **Ein kompromittierter Gateway-Host**: Der Host hält sowohl die Upstream-Anmeldedaten als auch verteilt [verwaltete Einstellungen](/docs/de/claude-apps-gateway-config#managed) an jeden verbundenen Entwickler, daher ist die Kontrolle über die Gateway-Konfiguration vergleichbar mit der Kontrolle über Ihr MDM. Der Einmal-Genehmigungsdialog der CLI für Shell-fähige Einstellungen begrenzt stille Änderungen, ersetzt aber nicht die Host-Sicherheit.
* **Ein böswilliger OIDC-Anbieter**: Der Anbieter signiert die id\_tokens, denen das Gateway vertraut, daher kann er jede Identität behaupten. Das Überprüfen und Sichern Ihres IdP ist Ihre Verantwortung.

<h3 id="user-code-brute-force-resistance">
  User-Code-Brute-Force-Resistenz
</h3>

Der `user_code`, den ein Entwickler auf der `/device`-Verifikationsseite eingibt, sind 8 Zeichen aus einem 20-Zeichen-Alphabet, was 20⁸ oder etwa 2,56×10¹⁰ Kombinationen ergibt, und er läuft nach 10 Minuten ab.

Das Gateway wendet Pro-IP-Rate-Limits auf die Device-Grant-Endpoints an, konfigurierbar über [`rate_limits`](/docs/de/claude-apps-gateway-config#http-tuning). Erhöhen Sie die Limits, wenn sich viele Entwickler von einer einzelnen gemeinsamen Unternehmens-NAT-Adresse anmelden. Die Limits gelten nur für den Anmeldungs-Flow, nicht für Inferenz.

<h3 id="compliance-posture">
  Compliance-Haltung
</h3>

* **Datenresidenz**: Die Datenebene des Gateways selbst sendet nichts an Anthropic, es sei denn, die Anthropic-API ist ein konfigurierter Upstream; wenn sie es ist, gilt Ihre bestehende Datenbehandlungsvereinbarung für den Inferenz-Pfad. Telemetrie, Audit, Identität und Einstellungen gehen nur an die Ziele, die Sie konfigurieren.
* **Host-Prozess-Verkehr**: Der Host-Prozess ist die Claude Code CLI, die Startup-Analytik und Update-Prüfungen an Anthropic senden kann. Für Strict-Egress-Bereitstellungen setzen Sie `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` in der Gateway-Container-Umgebung.
* **Client-Analytik**: Die CLI deaktiviert ihre eigene Nutzungsanalytik, während sie bei einem Gateway angemeldet ist, und die Fehlerberichterstattung ist standardmäßig auf Third-Party-API-Oberflächen deaktiviert.
* **Client-Maschinen**: Entwickler-CLIs senden immer noch WebFetch-Hostname-Prüfungen und Versions-Prüfungen an Anthropic, es sei denn, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` und `skipWebFetchPreflight: true` sind gesetzt. Siehe [Datennutzung](/docs/de/data-usage).
* **Umfrage-Bewertungen**: Die Gateway-Anmeldedaten deaktivieren die Anthropic-gebundene Bewertungs-Senke, daher werden Bewertungen nicht an Anthropic gesendet.
* **Transkript-Freigabe**: Das Wählen von Ja auf einer Umfrage-Transkript-Freigabe-Aufforderung schreibt eine lokale Datei unter `~/.claude/feedback-bundles/` statt zu Anthropic hochzuladen.
* **Client-Updates**: Update-Prüfungen sind getrennt vom Gateway-Verkehr. Pinnen Sie Versionen durch Ihre eigene Verteilung und setzen Sie `DISABLE_UPDATES`, wenn Laptops keine Releases abrufen dürfen. `DISABLE_AUTOUPDATER` stoppt nur Hintergrund-Updates, während `claude update` immer noch funktioniert.
* **TLS**: Bedienen Sie `public_url` über HTTPS in der Produktion, entweder vom Gateway-eigenen Listener über `listen.tls` oder von einem TLS-terminierenden Ingress vor einfachen HTTP-Replikationen mit `listen.public_url` gesetzt. Das Gateway weigert sich nicht, einfaches HTTP. Der IdP muss HTTPS in der Produktion bedienen, und Postgres unterstützt `?sslmode=require`. Setzen Sie `Strict-Transport-Security` bei Ihrem Ingress.
* **Vulnerability-Offenlegung**: Folgen Sie [Sicherheitsprobleme melden](/docs/de/security#reporting-security-issues)

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Für Fragen und Feedback verwenden Sie [Claude-Code-Support](https://support.claude.com/en/collections/14445694-claude-code), oder öffnen Sie ein Issue im [Claude-Code-GitHub-Repository](https://github.com/anthropics/claude-code/issues). Wenn Sie ein Problem melden, beziehen Sie ein:

* **Gateway-Problem**: Das Gateway-Stderr für das relevante Fenster, Ihre `gateway.yaml` mit Geheimnissen redigiert, die Gateway-Version, auf der Landingpage unter `/` und im `x-cc-gateway-version`-Response-Header auf `/managed/settings` angezeigt, und was sich kürzlich geändert hat
* **Anmeldungs-Problem**: Der Entwickler führt `claude --debug-file ./claude-debug.txt` aus, reproduziert und sendet diese Datei plus das Gateway-Audit-Protokoll für dasselbe Fenster
* **Inferenz-Problem**: Das angeforderte Modell, die konfigurierten Upstreams und das Gateway-Audit-Protokoll für die Anfrage, das aufzeichnet, welcher Upstream sie bedient hat und den Response-Status

Die Standardfehlerausgabe des Gateways enthält den Audit-Ereignisstrom, das Audit-Protokoll zeichnet Entwickleridentitäten auf, und die Debug-Datei zeichnet Hook- und MCP-Serverausgaben von der Maschine des Entwicklers auf. Überprüfen und redigieren Sie diese, bevor Sie sie in einem öffentlichen Problem posten.

| Symptom                                                                                                                                                                                        | Ursache                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Behebung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Das `/login` eines Entwicklers zeigt den Standard-Account-Picker statt des **Cloud-Gateway**-Bildschirms                                                                                       | `forceLoginMethod` oder `forceLoginGatewayUrl` ist nicht in verwalteten Einstellungen auf dieser Maschine gesetzt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Stellen Sie die [verwaltete Einstellungsdatei](/docs/de/claude-apps-gateway#set-the-gateway-url) auf dem Gerät bereit; `/login` liest die Gateway-URL von dort                                                                                                                                                                                                                                                                                                                                                             |
| Startup zeigt `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.`                                                            | Der installierte Claude-Code-Build ist älter als die Gateway-Unterstützung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Lassen Sie den Entwickler Claude Code auf eine Version aktualisieren, die Cloud-Gateway-Unterstützung enthält                                                                                                                                                                                                                                                                                                                                                                                                         |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>`                                                     | Der Gateway-Hostname wird zu mindestens einer öffentlichen IP-Adresse aufgelöst. Claude Code überprüft jede aufgelöste Adresse und erfordert, dass jede privat ist. Eine häufige Ursache ist ein Dual-Stack-Name, bei dem eine Familie zu einer öffentlichen Adresse aufgelöst wird, einschließlich AWS-interner Dual-Stack-Load-Balancer, die öffentliche AAAA-Adressen zurückgeben. Anthropic-betriebene öffentliche Gateway-Endpunkte sind von der Überprüfung ausgenommen, und `/login` akzeptiert sie über `https://`. Vor v2.1.206 lehnte `/login` sie wie jede andere öffentliche Adresse ab | Lassen Sie den Gateway-Namen nur zu privaten Adressen auf Entwicklermaschinen auflösen. Für einen Dual-Stack-Namen löschen Sie den öffentlichen Datensatz oder bedienen Sie einen separaten internen DNS-Namen. Siehe die [Private-Network-Voraussetzung](/docs/de/claude-apps-gateway#prerequisites).                                                                                                                                                                                                                     |
| CLI `/login`: `Gateway login requires a direct connection and does not support connecting through an HTTP proxy`                                                                               | Ein `HTTPS_PROXY` oder `HTTP_PROXY` gilt für den Gateway-Host und der Proxy-Hostname wird zu einer öffentlichen Adresse aufgelöst. Ein Proxy, dessen Host nur zu privaten Adressen aufgelöst wird, ist erlaubt und löst diesen Fehler nicht aus                                                                                                                                                                                                                                                                                                                                                     | Fügen Sie den Gateway-Host zu `NO_PROXY` auf der Entwicklermaschine hinzu, damit die Verbindung direkt ist, oder verwenden Sie einen Proxy, dessen Hostname zu privaten Adressen aufgelöst wird                                                                                                                                                                                                                                                                                                                       |
| CLI `/login`: `Could not resolve gateway host <host>`                                                                                                                                          | Die Maschine kann den internen DNS-Namen des Gateways nicht auflösen, typischerweise weil sie nicht im Unternehmens-Netzwerk ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Lassen Sie den Entwickler sich mit Ihrem Netzwerk oder VPN verbinden und versuchen Sie dann `/login` erneut                                                                                                                                                                                                                                                                                                                                                                                                           |
| Boot beendet mit einem Konfigurationsvalidierungsfehler, der `store.postgres_url` benennt                                                                                                      | Kein Postgres konfiguriert; das Gateway erfordert Postgres                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Setzen Sie `store.postgres_url`. Für lokale Entwicklung verwenden Sie einen Wegwerf-Container: `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.                                                                                                                                                                                                                                                                                                                                            |
| Boot beendet: `requires the native binary`                                                                                                                                                     | Läuft unter Node statt der nativen Binärdatei                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Installieren Sie Claude Code mit einer der [Standalone-Installationsmethoden](/docs/de/setup)                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Boot beendet mit einem OIDC-Discovery-Fehler nach `config.load`                                                                                                                                | `oidc.issuer` nicht erreichbar oder TLS-Kette nicht vertraut                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Überprüfen Sie, dass der Aussteller vom Pod erreichbar ist und `/.well-known/openid-configuration` bedient. Setzen Sie `ca_cert_pem` für private PKI.                                                                                                                                                                                                                                                                                                                                                                 |
| Boot beendet mit einem Postgres-Berechtigungsfehler                                                                                                                                            | App-Rolle fehlt `CREATE TABLE`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Erstellen Sie das Schema vorab mit einer Admin-Rolle und gewähren Sie DML der App-Rolle, oder gewähren Sie DDL vorübergehend für Boots, die neue Migrationen anwenden                                                                                                                                                                                                                                                                                                                                                 |
| `/oauth/callback` zeigt "Sign-in could not be completed"                                                                                                                                       | E-Mail-Domain abgelehnt, id\_token-Validierung fehlgeschlagen, oder `email_verified` ist explizit `false`, was das Gateway immer ohne Überschreibung ablehnt                                                                                                                                                                                                                                                                                                                                                                                                                                        | Überprüfen Sie `allowed_email_domains` und dass der IdP einen verifizierten `email`-Claim zurückgibt. Für `email_verified: false` beheben Sie die IdP-seitige Verifikation. Wenn Ihr IdP E-Mail unter einem anderen Claim-Namen ausgibt, setzen Sie `oidc.email_claim`.                                                                                                                                                                                                                                               |
| Protokoll: `token exchange failed: id_token missing email claim`                                                                                                                               | Der IdP enthält `email` nicht standardmäßig im id\_token. Diese Ablehnung wird nur ausgelöst, wenn `allowed_email_domains` gesetzt ist; ohne sie prägt ein fehlende E-Mail eine Sitzung ohne E-Mail                                                                                                                                                                                                                                                                                                                                                                                                 | Konfigurieren Sie den IdP, um `email` im id\_token auszugeben. Okta: Fügen Sie `email` zu den ID-Token-Claims eines benutzerdefinierten Autorisierungsservers hinzu. Entra: Fügen Sie `email` als optionalen Claim bei der App-Registrierung hinzu. PingFederate: Aktivieren Sie eine OpenID-Connect-Richtlinie, die `email` ausgibt. Wenn der IdP `email` vom Userinfo-Endpoint bedient, aber nicht im id\_token einbeziehen wird, wie der Okta-Org-Autorisierungsserver, setzen Sie `oidc.userinfo_fallback: true`. |
| Jede Amazon-Bedrock-Anfrage gibt 502 zurück; Protokoll zeigt `Could not load credentials from any providers`                                                                                   | Auf EC2 blockiert IMDSv2s Standard-Hop-Limit von 1 die Instanz-Metadata-Anfrage von innerhalb des Containers. Boot und `/readyz` bestehen trotzdem, da das AWS SDK Instanz-Anmeldedaten bei der ersten Anfrage auflöst, nicht bei der Client-Konstruktion                                                                                                                                                                                                                                                                                                                                           | Erhöhen Sie das Hop-Limit mit `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`, oder setzen Sie es in der Launch-Vorlage. Die Änderung gilt für jeden Container auf der Instanz. Bevorzugen Sie ECS-Task-Rollen, wo verfügbar, die Anmeldedaten vom ECS-Container-Credentials-Endpoint lesen und die Änderung ganz vermeiden, oder wenden Sie die Änderung auf einer dedizierten Gateway-Instanz an, um die Exposition zu begrenzen.                                     |
| IdP-Fehler: unknown or unsupported scope                                                                                                                                                       | Der IdP lehnt Scopes ab, die er nicht erkennt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Setzen Sie `oidc.scopes` auf genau die Liste, die Ihr IdP akzeptiert; sie muss `openid` enthalten. Der Standard ist `openid profile email offline_access`.                                                                                                                                                                                                                                                                                                                                                            |
| Sitzungen erneuern sich nicht stillschweigend nach dem Setzen von `oidc.scopes`                                                                                                                | `offline_access` wurde aus der Überschreibung gelöscht                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Fügen Sie `offline_access` zurück, wenn Ihr IdP es unterstützt. Ohne einen Refresh-Token führen Entwickler die Browser-Anmeldung alle `session.ttl_hours` erneut aus.                                                                                                                                                                                                                                                                                                                                                 |
| Browser zeigt "This request came from another site and was blocked"                                                                                                                            | Cross-Site-Form-POST, blockiert als CSRF-Schutz. Erwartet für eingebettete oder proxied Seiten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Öffnen Sie den Verifikations-Link direkt                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Chrome blockiert die Approve-Schaltfläche mit "Refused to send form data … violates … Content Security Policy directive: form-action", aber dieselbe Seite funktioniert in Safari oder Firefox | Chrome erzwingt `form-action` gegen die gesamte Redirect-Kette. Ihr IdP leitet weiter zu einem zweiten Host, der nicht auf der Allowlist steht.                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Fügen Sie jeden zusätzlichen Ursprung in der Redirect-Kette zu `oidc.form_action_origins` hinzu. Öffnen Sie Chrome DevTools → Console auf der Approve-Seite, um zu sehen, welcher Ursprung blockiert wurde.                                                                                                                                                                                                                                                                                                           |
| Anmeldung wird beim IdP abgeschlossen, aber der Callback schlägt fehl, mit einem CSP-Fehler in Chrome oder "this sign-in link has expired" in Safari                                           | Der IdP gab den Code über `response_mode=form_post` zurück, das ihn Cross-Origin über POST zu `/oauth/callback` automatisch einreicht. Chrome blockiert das unter einer strikten CSP; Safari erlaubt die Einreichung, aber der Callback liest nur die Query-String                                                                                                                                                                                                                                                                                                                                  | Stellen Sie sicher, dass Ihr IdP `response_mode=query` ehrt, das das Gateway explizit anfordert, damit der Callback eine einfache Umleitung ist                                                                                                                                                                                                                                                                                                                                                                       |
| Anmeldung funktioniert lokal, schlägt aber hinter einem ALB fehl                                                                                                                               | `public_url` nicht gesetzt, daher erhält der IdP den inneren `http://`-Ursprung als `redirect_uri`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Setzen Sie `listen.public_url` auf den externen `https://`-Ursprung                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Entwickler sieht die Vertrauens-Aufforderung wiederholt                                                                                                                                        | TLS-Zert rotiert pro Replikation oder pro Anfrage                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Verwenden Sie ein stabiles Zert beim Ingress, oder beenden Sie TLS einmal und führen Sie Replikationen intern über einfaches HTTP aus                                                                                                                                                                                                                                                                                                                                                                                 |
| CLI `/login`: "Could not verify the gateway's TLS certificate" oder `SELF_SIGNED_CERT_IN_CHAIN`                                                                                                | Die TLS-Kette des Gateways ist von einer privaten CA signiert, die nicht im CLI-Host-Trust-Store ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Claude Code liest den OS-Trust-Store standardmäßig auf der nativen Binärdatei und auf Node 22.15 oder später; [`CLAUDE_CODE_CERT_STORE`](/docs/de/network-config#ca-certificate-store) steuert dieses Verhalten. Wenn die CA im OS-Trust-Store installiert ist, stellen Sie sicher, dass Entwickler auf einer aktuellen Runtime sind. Andernfalls setzen Sie `NODE_EXTRA_CA_CERTS` auf das CA-Zertifikat-PEM, bevor Sie starten. Die First-Connect-Fingerprint-Aufforderung gilt immer noch.                               |

<h2 id="related">
  Verwandt
</h2>

* [Claude-Apps-Gateway-Übersicht](/docs/de/claude-apps-gateway): Schnellstart und Entwickler-Verbindung
* [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config): Jede `gateway.yaml`-Option
