> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude-Apps-Gateway für Amazon Bedrock, Claude Platform auf AWS, Google Cloud und Microsoft Foundry

> Führen Sie Claude Code über Amazon Bedrock, Claude Platform auf AWS, Google Cloud oder Microsoft Foundry hinter einem selbstgehosteten Gateway mit SSO-Anmeldung, Modellzugriff pro Gruppe und OTLP-Telemetrie aus.

<Note>
  Das Claude-Apps-Gateway ist für Organisationen konzipiert, die Inferenzen über ihren eigenen Cloud-Anbieter leiten müssen oder möchten – beispielsweise um [Anforderungen zur Datenresidenz](/docs/de/claude-apps-gateway-deploy#compliance-posture) zu erfüllen. Wenn Sie diese Anforderung nicht haben und Zugriff auf andere Funktionen wie SCIM-Bereitstellung oder Claude Code im Web und auf Mobilgeräten wünschen, ist Claude Enterprise möglicherweise besser geeignet. Weitere Informationen finden Sie auf der Seite zur [Funktionsverfügbarkeit](/docs/de/feature-availability), um einen vollständigen Vergleich aller Bereitstellungsmethoden zu erhalten.
</Note>

Claude-Apps-Gateway ist ein selbstgehosteter Service, der sich zwischen den Claude-Code-Clients Ihrer Entwickler und Ihrem Modellanbietern befindet. Entwickler melden sich mit Ihrem Unternehmensidentitätsanbieter (IdP) an, anstatt API-Schlüssel oder Cloud-Anmeldedaten zu halten. Das Gateway hält die Upstream-Anmeldedaten, erzwingt Modellzugriff und [verwaltete Einstellungen](/docs/de/managed-settings) nach IdP-Gruppe und leitet Nutzungstelemetrie an Ihren eigenen Observability-Stack weiter.

Es ist in der `claude`-Binärdatei enthalten, daher führt die gleiche ausführbare Datei, die Claude Code auf einem Laptop ausführt, den Gateway-Server mit `claude gateway --config gateway.yaml` aus.

Diese Seite behandelt:

* [Warum Claude-Apps-Gateway](#why-claude-apps-gateway), was es gegenüber dem Betrieb Ihres eigenen hinzufügt, und wann etwas anderes besser passt
* Ein [Schnellstart](#quickstart) mit [Voraussetzungen](#prerequisites), der ein Gateway von Null zu einem angemeldeten Entwickler bringt
* [Entwickler verbinden](#connect-developers), einschließlich der Einstellung der Gateway-URL durch verwaltete Einstellungen
* [Verfügbarkeit und Einschränkungen](#availability-and-limitations), die abdecken, welche Claude-Code-Funktionen über das Gateway funktionieren und was der Server unterstützt

Begleitseiten gehen tiefer. Die [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config) behandelt jede Option in der YAML-Datei, die der Schnellstart schreibt, und der [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy) behandelt IdP-spezifische Einrichtung, Kubernetes- und Cloud-Run-Bereitstellung sowie Operationen.

<h2 id="why-claude-apps-gateway">
  Warum Claude apps gateway
</h2>

Die [Gateway-Übersicht](/docs/de/gateways) behandelt, was ein Gateway tut und warum Sie eines ausführen würden. Claude apps gateway ist Anthropics eigenes Gateway, das in die `claude`-Binärdatei integriert und neben jeder Claude-Code-Version getestet wird, daher leitet es die Header und Anforderungsfelder weiter, die Claude Code sendet, ohne dass Operatoren eine separate Zulassungsliste verwalten müssen. Nach der Bereitstellung erhalten Sie:

* **Anmeldedaten**: Der Upstream-API-Schlüssel oder die Cloud-Anmeldedaten befinden sich nur in Ihrer Infrastruktur. Entwickler authentifizieren sich mit Corporate SSO und erhalten kurzlebige Bearer-Token, daher erfolgt das Offboarding in Ihrem IdP. Heben Sie die Bereitstellung eines Benutzers auf und sein Gateway-Zugriff läuft innerhalb der Sitzungsdauer ab, standardmäßig eine Stunde.
* **Zugriffskontrolle**: Ihre IdP-Gruppen werden Modellzulassungslisten und [verwaltete Einstellungen](/docs/de/managed-settings)-Richtlinien zugeordnet. Das Gateway erzwingt Modellzugriff auf der Serverseite, lehnt Anfragen für nicht gewährte Modelle ab und wählt die verwaltete Einstellungsrichtlinie jeder Gruppe aus, die die CLI auf der [Ebene der verwalteten Einstellungen](/docs/de/settings#settings-precedence) anwendet. Verschiedene Teams erhalten verschiedene Modelle, Tools und Berechtigungen, und ein Entwickler kann nicht überschreiben, was seine Richtlinie sperrt.
* **Einstellungsbereitstellung**: Das Gateway liefert verwaltete Einstellungen selbst an angemeldete Clients und ersetzt [servergesteuerte Einstellungen](/docs/de/server-managed-settings) aus der claude.ai-Admin-Konsole.
* **Telemetrie**: Jedes konfigurierte Ziel empfängt [OpenTelemetry-Protokoll (OTLP)-Metriken](/docs/de/monitoring-usage) mit Token-Zählungen, Modell, Benutzeridentität und Latenz standardmäßig, mit Protokollen und Traces als optionale Ziele pro Ziel.
* **Upstream-Routing**: Clients sprechen die Anthropic Messages API mit dem Gateway, und das Gateway übersetzt für jeden Upstream, ob Amazon Bedrock, [Claude Platform on AWS](/docs/de/claude-platform-on-aws), Google Clouds Agent Platform, Microsoft Foundry oder die Anthropic API, mit Failover zwischen ihnen. Sie können Regionen, Anbieter oder Failover-Reihenfolge ändern, ohne dass Entwickler dies bemerken oder neu konfigurieren müssen.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagramm, das Claude-Code-Clients zeigt, die sich über HTTPS mit Bearer-Token mit einem selbstgehosteten Claude-Apps-Gateway in Ihrer Infrastruktur verbinden, das Benutzer gegen Ihren IdP authentifiziert, Auth-Status in PostgreSQL speichert, Telemetrie an Ihren OTLP-Collector weiterleitet und Inferenzen an Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry oder die Anthropic API weiterleitet" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  Die Datenebene des Gateways selbst sendet nichts an die Anthropic-Infrastruktur, es sei denn, die Anthropic API ist ein konfigurierter Upstream. Sie kontrollieren, wohin Telemetrie, Audit-Protokolle, verwaltete Einstellungen und die IdP-Identität Ihrer Entwickler gehen, und das Gateway sendet keine davon an Anthropic. Für den verbleibenden Datenverkehr, den der CLI-Prozess senden kann, und wie man ihn schließt, siehe [Compliance-Postur](/docs/de/claude-apps-gateway-deploy#compliance-posture).
</Note>

Welche Claude-Code-Funktionen über das Gateway funktionieren und was der Server selbst unterstützt, siehe [Verfügbarkeit und Einschränkungen](#availability-and-limitations) unten. Für Entscheidungen wie Kosten, Umgehung, Ausführung mehrerer Gateways und serverlose Plattformen siehe den [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Andere Gateway-Implementierungen
</h3>

Wenn Sie bereits ein LLM-Gateway oder API-Gateway ausführen, das Ihre Anforderungen erfüllt, verwenden Sie es weiterhin; [Andere LLM-Gateways](/docs/de/llm-gateway) behandelt die Konfiguration von Claude Code dagegen.

Die [Gateway-Kompatibilitätsleitfaden](/docs/de/llm-gateway-protocol) dokumentiert, was Claude Code von jedem Gateway erwartet: die Endpunkte, die es aufruft, die Header und Body-Felder zum Weiterleiten und was nicht funktioniert, wenn sie entfernt werden. Ein laufendes Claude-Apps-Gateway bedient auch sein eigenes Protokoll unter `GET /protocol`, das die Endpunkte beschreibt, die es Claude-Code-Clients bereitstellt: SSO-Anmeldung, Inferenz, Verwaltete-Einstellungen-Bereitstellung, Modellermittlung und Telemetrie. Rufen Sie es mit `curl https://claude-gateway.internal.example.com/protocol` von jedem bereitgestellten Gateway ab, wie dem, das der [Schnellstart](#quickstart) unten erzeugt.

Breaking Changes am Protokoll werden im Voraus angekündigt, aber unbegrenzte Rückwärtskompatibilität ist nicht garantiert.

<h2 id="quickstart">
  Schnellstart
</h2>

Dieser Schnellstart führt den minimalen Weg: Registrieren Sie einen OAuth-Client in Ihrem IdP, schreiben Sie eine `gateway.yaml`, führen Sie das Gateway zusammen mit Postgres mit Docker Compose aus und überprüfen Sie die Anmeldung von Ende zu Ende. Es verwendet einen Amazon-Bedrock-Upstream; Claude Platform auf AWS, Google Clouds Agent Platform, Microsoft Foundry und die Anthropic API werden gleichermaßen unterstützt, indem Sie den `upstreams`-Block wie in der [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config#upstreams) gezeigt austauschen. Am Ende haben Sie ein Gateway, bei dem sich ein Entwickler `/login` anmelden kann.

<Note>
  **Bereitstellung in Ihrem privaten Netzwerk.** Claude Code verbindet sich nur mit einem Gateway, dessen Adresse privat ist. Dies ist eine Sicherheitsmaßnahme, da ein vertrauenswürdiges Gateway Einstellungen pushen kann, die Befehle auf Entwicklermaschinen ausführen. Platzieren Sie das Gateway hinter einem internen Load Balancer oder VPN und geben Sie ihm einen Hostnamen, der nur zu privaten IPs aufgelöst wird. Wenn Ihr internes Netzwerk aus öffentlichem IPv4-Adressraum nummeriert ist, den Ihre Organisation besitzt, siehe [Gateway auf öffentlichem Adressraum zulassen, den Sie besitzen](#allow-a-gateway-on-public-address-space-you-own).
</Note>

<h3 id="prerequisites">
  Voraussetzungen
</h3>

Haben Sie diese vor dem Start bereit:

| Sie benötigen                            | Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 oder später         | Der `claude gateway`-Unterbefehl und der Gateway-Anmeldungsfluss werden in v2.1.195 ausgeliefert. Frühere öffentliche Builds enthalten sie nicht. Sowohl die Maschine, auf der der Gateway-Server läuft, als auch die Maschine jedes Entwicklers müssen v2.1.195 oder später sein; führen Sie `claude update` aus, um die neueste Version zu erhalten. Die [Claude Platform auf AWS Upstream](/docs/de/claude-apps-gateway-config#claude-platform-on-aws) erfordert Claude Code v2.1.198 oder später auf dem Gateway-Server.                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| OpenID Connect (OIDC)-Identitätsanbieter | Okta, Microsoft Entra ID, Google Workspace, Keycloak oder Dex, oder ein anderer OIDC-konformer IdP wie PingFederate. Das Gateway führt Standard-OIDC-Erkennung und den Authorization-Code-Flow dagegen aus. SAML und LDAP werden nicht unterstützt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| PostgreSQL 14 oder später                | Unterstützt den Geräte-Anmeldungsfluss, bei dem der Browser-Callback schreibt und die Polling-CLI liest, sowie Rate-Limit-Zähler. Jedes verwaltete Postgres funktioniert, einschließlich der kleinsten Stufe. Ohne konfigurierte Ausgabenlimits speichert das Gateway einige KB kurzlebigen Auth-Status; mit [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) enthält es auch dauerhafte Ausgaben-, Audit- und Identitätstabellen, die gesichert werden sollten. TLS über `?sslmode=require` wird empfohlen.                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Modell-Upstream                          | Amazon-Bedrock-Anmeldedaten, Claude Platform auf AWS Anmeldedaten, Google-Cloud-Anmeldedaten, eine Microsoft-Foundry-Ressource oder einen Anthropic-API-Schlüssel. Mehrere Upstreams werden mit Failover unterstützt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| HTTPS                                    | Das Gateway muss über `https://` von Entwickler-Laptops und von jedem Browser, der für die Anmeldung verwendet wird, erreichbar sein; das Gateway bedient die Geräteüberprüfungsseite auf dem gleichen Listener. Stellen Sie entweder ein TLS-Zertifikat über `listen.tls` bereit, oder führen Sie hinter einem TLS-terminierenden Ingress aus und setzen Sie `listen.public_url` auf den externen Ursprung in beiden Fällen. Ein einfacher `http://`-Ursprung wird nur akzeptiert, wenn der Gateway-Host Loopback ist: `localhost`, `127.0.0.1` oder `::1`.                                                                                                                                                                                                                                                                                                                                                                                                    |
| Private-Netzwerk-Adresse                 | Bei `/login` erfordert Claude Code, dass der Hostname oder die IP-Adresse des Gateways nur zu privaten Adressen aufgelöst wird: RFC 1918, Link-lokal, CGNAT `100.64.0.0/10`, IPv6 ULA `fc00::/7` oder Loopback. Für ein von Ihnen gehostetes Gateway wird jede öffentliche Adresse außerhalb eines Blocks, den Sie deklarieren, abgelehnt; siehe das [Bedrohungsmodell](/docs/de/claude-apps-gateway-deploy#threat-model-summary) im Bereitstellungsleitfaden. Wenn Entwicklermaschinen HTTPS über einen Unternehmens-Proxy leiten, erfordert die Anmeldung auch, dass der Proxy-Host zu privaten Adressen aufgelöst wird; wenn nicht, fügen Sie den Gateway-Host zu `NO_PROXY` hinzu, damit die CLI direkt verbunden wird. Wenn Ihr internes Netzwerk aus öffentlichem IPv4-Adressraum nummeriert ist, den Ihre Organisation besitzt, [deklarieren Sie diese Blöcke](#allow-a-gateway-on-public-address-space-you-own), damit `/login` ein Gateway dort akzeptiert. |
| Linux-Laufzeit                           | Der Gateway-Server läuft nur auf der nativen Linux-Binärdatei. macOS funktioniert für lokale Entwicklung. Windows wird nicht als Serverplattform unterstützt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

<h3 id="steps">
  Schritte
</h3>

<Steps>
  <Step title="Registrieren Sie einen OAuth-Client in Ihrem IdP">
    Entscheiden Sie zuerst den Hostnamen des Gateways, da der Redirect-URI damit übereinstimmen muss. Erstellen Sie eine neue OIDC-Webanwendung und setzen Sie den Redirect-URI auf `https://claude-gateway.<your-domain>/oauth/callback`, wobei der Host der gleiche Wert ist, den Sie als [`listen.public_url`](/docs/de/claude-apps-gateway-config#listen) in Schritt 3 setzen. Notieren Sie sich die `client_id` und `client_secret`. IdP-spezifische Anweisungen finden Sie unter [Identitätsanbieter-Einrichtung](/docs/de/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Stellen Sie eine PostgreSQL-Datenbank bereit">
    Jedes Postgres 14 oder später funktioniert, einschließlich der kleinsten verwalteten Stufe. Das Gateway führt seine eigenen Schema-Migrationen beim Start aus, daher benötigt die Datenbankrolle Rechte zum Erstellen und Ändern von Tabellen; siehe [`store`](/docs/de/claude-apps-gateway-config#store).
  </Step>

  <Step title="Schreiben Sie gateway.yaml">
    Geheimnisse werden über `${ENV_VAR}`-Erweiterung gelesen, daher kann die Datei selbst in der Versionskontrolle leben. Verwenden Sie einen `public_url`-Hostnamen, der zu einer privaten IP in Ihrem Netzwerk aufgelöst wird, da `/login` öffentliche Adressen ablehnt. Die minimale Konfiguration hat fünf Abschnitte, und jedes andere Feld hat einen Standard:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Erforderlich, sofern der Host keine Loopback-Adresse ist. Wird für den IdP
      # redirect_uri und das Discovery-Dokument verwendet.
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # muss /.well-known/openid-configuration bedienen
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # lehne id_tokens außerhalb Ihrer Organisation ab
      userinfo_fallback: true                  # für IdPs, deren id_token E-Mail/Gruppen auslässt; ansonsten harmlos

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # begrenzt auch die Widerrufungslatenz bei IdP-Entbereitstellung

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # fügen Sie ?sslmode=require für verwaltetes Postgres hinzu

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # leer: AWS-Standard-Anmeldekette
    # (IRSA, EC2/ECS-Task-Rolle, Umgebungsvariablen, ~/.aws)

    # Modelle werden pro Upstream automatisch übersetzt. Der integrierte Katalog
    # ordnet claude-opus-4-8 us.anthropic.claude-opus-4-8 zu und so weiter für jedes
    # von Bedrock unterstützte Claude-Modell. Setzen Sie false und fügen Sie eine `models:`-Liste hinzu, um
    # nur bestimmte Modelle verfügbar zu machen.
    auto_include_builtin_models: true
    ```

    Diese Konfiguration reicht für eine funktionierende Anmeldeschleife mit dem Standard-Bedrock-Modellkatalog aus. Sobald es läuft, fügen Sie Pro-Gruppen-RBAC und verwaltete Einstellungen über [`managed.policies`](/docs/de/claude-apps-gateway-config#managed) hinzu, Telemetrie-Verteilung über [`telemetry`](/docs/de/claude-apps-gateway-config#telemetry) und Multi-Upstream-Failover, bereitgestellte Durchsatz-ARNs oder Nicht-US-Regionen über [`models`](/docs/de/claude-apps-gateway-config#models).

    <Note>
      Der Amazon-Bedrock-Upstream benötigt einen AWS-Principal mit `bedrock:InvokeModel` und `bedrock:InvokeModelWithResponseStream` auf beiden `inference-profile/us.anthropic.*`-ARNs und den zugrunde liegenden `foundation-model/anthropic.*`-ARNs. Er benötigt auch Anthropics einmalige Anwendungsform, die für das Konto aus der Bedrock-Konsole des Modellkatalogs eingereicht werden muss. Stellen Sie die Anmeldedaten mit IRSA auf EKS, einer ECS-Task-Rolle oder einem EC2-Instance-Profil bereit, anstatt statische Schlüssel zu verwenden. Die [`upstreams`-Referenz](/docs/de/claude-apps-gateway-config#upstreams) hat die vollständigen IAM-Details, die Cloud-übergreifende Anmeldedaten-Matrix und die `auth`-Blöcke für die anderen Anbieter.
    </Note>
  </Step>

  <Step title="Führen Sie es aus">
    Erstellen Sie ein Container-Image um die `claude`-Binärdatei, die die [Image-Anforderungen](/docs/de/claude-apps-gateway-deploy#container-image) erfüllt, und führen Sie es zusammen mit Postgres aus. Die Compose-Datei verweist auf das Image als `registry.example.com/claude-gateway:2.1.198`; ersetzen Sie Ihre eigene Registry und Ihr Image-Tag:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: registry.example.com/claude-gateway:2.1.198
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # AWS-Anmeldedaten: in der Produktion diese auslassen und eine Instance-Rolle verwenden.
          # Für lokales Compose-Testen, übergeben Sie Ihre eigenen:
          AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
          AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
          AWS_SESSION_TOKEN: ${AWS_SESSION_TOKEN}
        depends_on:
          postgres:
            condition: service_healthy
      postgres:
        image: postgres:16-alpine
        environment: { POSTGRES_USER: gw, POSTGRES_PASSWORD: pw, POSTGRES_DB: gateway }
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U gw"]
          interval: 5s
        volumes: ["pgdata:/var/lib/postgresql/data"]
    volumes: { pgdata: }
    ```

    Das Gateway ist eine einzelne Linux-Binärdatei, die die Konfiguration liest, sich mit Postgres verbindet und seine Schema-Migrationen anwendet, OIDC-Erkennung gegen Ihren IdP ausführt, Upstream-Clients erstellt und mit dem Abhören beginnt. Der Start ist fail-closed für die Konfiguration, die Postgres-Verbindung, OIDC-Erkennung und Upstream-Client-Konstruktion. Wenn einer dieser Punkte unerreichbar oder falsch konfiguriert ist, beendet sich das Gateway mit einem Fehler, anstatt Datenverkehr in einem degradierten Zustand zu bedienen.

    Ein erfolgreicher Start validiert nicht den Inferenzpfad, da Amazon Bedrock und Google Clouds Agent Platform Instance-Anmeldedaten bei der ersten Anfrage aufgelöst werden, nicht beim Start.

    Beobachten Sie stderr auf die Boot-Sequenz. Log-Zeilen verwenden das Format `[gateway] <timestamp> <level> <message>`, Audit-Events sind einzeilige JSON mit einem `evt`-Feld, und ein Startup-Banner, unten weggelassen, wird zwischen der Migration und den Listening-Zeilen gedruckt. Eine frische Datenbank druckt eine `migration N applied`-Zeile pro Schema-Migration; eine bereits migrierte Datenbank druckt keine. Sie sollten in dieser Reihenfolge sehen:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.395Z info waiting for migration lock (another replica may be migrating; check pg_locks for key 6775156 if this persists)
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    …
    [gateway] 2026-06-10T17:03:21.431Z info migration 6 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    Das Gateway protokolliert auch eine Warnung, dass `access_control.allow_cidrs` leer ist. Das ist hier erwartet, da nichts die Client-Adressen begrenzt, die das Gateway bedient, bis Sie eine Zulassungsliste festlegen. Die [`access_control`-Referenz](/docs/de/claude-apps-gateway-config#http-tuning) hat die empfohlenen Bereiche.

    Wenn der Start vor der `claude gateway listening on`-Zeile beendet wird, benennt die letzte Zeile von stderr das Problem:

    * ein unerreichbares Postgres
    * eine Postgres-Rolle ohne DDL-Berechtigung
    * ein unerreichbares oder ungültiges OIDC-Discovery-Dokument
    * eine Konfigurationsschema-Verletzung mit dem betroffenen Feldpfad

    Beheben Sie es und starten Sie neu.

    Wenn Sie bereits einen TLS-terminierenden Ingress haben, überspringen Sie Compose und führen Sie die Binärdatei direkt mit `claude gateway --config gateway.yaml` aus. Setzen Sie `public_url` auf den Ingress-Ursprung und binden Sie `listen` an eine Loopback- oder Cluster-interne Adresse.
  </Step>

  <Step title="Überprüfen Sie die Auth-Oberfläche">
    Drei Überprüfungen bestätigen, dass das Gateway einen echten Benutzer authentifizieren kann, bevor Sie es an einen Entwickler übergeben.

    Die Beispiele verwenden die öffentliche URL des Gateways; für das lokale Compose-Setup ohne Ingress ersetzen Sie `http://localhost:8080` in den ersten beiden Überprüfungen. Die dritte Überprüfung öffnet `verification_uri_complete`, das aus `public_url` erstellt wird, daher setzen Sie für lokales Compose `public_url: http://localhost:8080` in `gateway.yaml` und fügen Sie `http://localhost:8080/oauth/callback` als zweiten Redirect-URI auf dem OAuth-Client aus Schritt 1 hinzu, da das Gateway den IdP `redirect_uri` aus `public_url` erstellt. Der Überprüfungslink wird dann in Ihrem lokalen Browser geöffnet.

    Führen Sie in Windows PowerShell `curl.exe` aus; das bloße `curl` ist ein Alias für `Invoke-WebRequest` und lehnt diese Flags ab.

    Rufen Sie zunächst das Discovery-Dokument ab, das bestätigt, dass das Gateway aktiv ist, die Konfiguration gültig ist und alle Boot-Überprüfungen bestanden haben:

    ```bash theme={null}
    curl -s https://claude-gateway.internal.example.com/.well-known/oauth-authorization-server | jq
    ```

    ```json theme={null}
    {
      "issuer": "https://claude-gateway.internal.example.com",
      "device_authorization_endpoint": "…/oauth/device_authorization",
      "token_endpoint": "…/oauth/token",
      "grant_types_supported": ["urn:ietf:params:oauth:grant-type:device_code", "refresh_token"]
    }
    ```

    Die Antwort enthält zusätzliche Felder wie `response_types_supported` und `scopes_supported`.

    Fordern Sie zweitens eine Geräteautorisierung an, die bestätigt, dass der Geräte-Anmeldungsfluss funktioniert und Postgres erreichbar und beschreibbar ist:

    ```bash theme={null}
    curl -s -X POST https://claude-gateway.internal.example.com/oauth/device_authorization | jq
    ```

    ```json theme={null}
    {
      "device_code": "…",
      "user_code": "WDJB-MJHT",
      "verification_uri": "https://claude-gateway.internal.example.com/device",
      "verification_uri_complete": "https://claude-gateway.internal.example.com/device?user_code=WDJB-MJHT",
      "expires_in": 600,
      "interval": 5
    }
    ```

    Testen Sie drittens das Browser-Bein, indem Sie `verification_uri_complete` in einem Browser öffnen und den Code bestätigen. Sie sollten zur Anmeldungsseite Ihres IdP weitergeleitet werden, und nach der Anmeldung auf dem Gateway mit einer angemeldeten Bestätigung landen.

    Verwenden Sie die erste fehlgeschlagene Überprüfung, um das Problem zu lokalisieren:

    * **Erste Überprüfung schlägt fehl**: Der Start wurde nicht abgeschlossen; überprüfen Sie stderr
    * **Zweite Überprüfung schlägt fehl**: Postgres ist vom Gateway nicht erreichbar oder die Rolle kann nicht schreiben; überprüfen Sie die Verbindungszeichenfolge und Berechtigungen
    * **Dritte Überprüfung erreicht den IdP nicht**: Überprüfen Sie, dass der Redirect-URI des IdP genau `https://<gateway>/oauth/callback` entspricht
    * **Dritte Überprüfung erreicht den IdP, springt aber mit einem Fehler zurück**: Lesen Sie das Audit-Protokoll des Gateways, das jede Auth-Ablehnung mit dem Grund aufzeichnet, z. B. `email domain not allowed`
  </Step>

  <Step title="Melden Sie einen Entwickler an">
    Dieser letzte Schritt erfolgt auf einer Entwicklermaschine, nicht auf dem Server. Setzen Sie `forceLoginMethod` auf `"gateway"` und `forceLoginGatewayUrl` auf die `public_url` Ihres Gateways in der [verwalteten Einstellungsdatei](/docs/de/managed-settings#delivery-mechanisms) dieser Maschine, führen Sie dann `/login` aus, drücken Sie Enter auf dem **Cloud gateway**-Bildschirm und schließen Sie die Browser-Anmeldung ab. [Gateway-URL festlegen](#set-the-gateway-url) unten behandelt die Verteilung beider Schlüssel auf jeder Entwicklermaschine.
  </Step>
</Steps>

<h2 id="connect-developers">
  Entwickler verbinden
</h2>

Entwickler verbinden sich von ihren eigenen Laptops mit einer Browser-Anmeldung, indem sie ihr Unternehmensarbeitskonto verwenden. Sie benötigen kein claude.ai-Konto, keinen API-Schlüssel und kein Abonnement, da Anfragen an das Modell über das Gateway mit den Upstream-Anmeldedaten der Organisation gehen. Die Verbindung wird durch die [clientseitigen verwalteten Einstellungen](/docs/de/claude-apps-gateway-config#client-side-managed-settings) gesteuert, die Sie über MDM pushen, daher gibt es keine manuelle Einrichtung auf der Entwicklerseite; dieser Abschnitt behandelt, was der Admin konfiguriert.

Die CLI fingerabdruckt das TLS-Blatt-Zertifikat des Gateways beim ersten Verbinden und heftet es pro Hostname an. Sie überprüft diesen Pin erneut während der Anmeldung, bei stillen Sitzungsaktualisierungen und beim Abrufen verwalteter Einstellungen, während Inferenzanfragen die standardmäßige TLS-Validierung ohne Pin verwenden. Anfragen, die über einen HTTPS-Proxy weitergeleitet werden, überspringen die Pin-Überprüfung, daher fügen Sie den Gateway-Host zu `NO_PROXY` hinzu, um sie direkt zu halten.

Veröffentlichen Sie den erwarteten SHA-256-Fingerabdruck zusammen mit der Gateway-URL, damit Entwickler etwas zum Vergleichen haben. Die `/login`-Eingabeaufforderung zeigt die ersten 16 Zeichen des Fingerabdrucks als Kleinbuchstaben-Hexadezimal ohne Doppelpunkte. Um den vollständigen Fingerabdruck in dieser Form aus der Zertifikatsdatei auszudrucken, führen Sie aus:

```bash theme={null}
openssl x509 -noout -fingerprint -sha256 -in cert.pem | cut -d= -f2 | tr -d : | tr 'A-F' 'a-f'
```

Wenn das Zertifikat rotiert, sieht jeder Entwickler die Vertrauensaufforderung erneut, daher behandeln Sie Rotationen als geplantes Ereignis und veröffentlichen Sie den Fingerabdruck erneut. Wenn Ihre Gateway-Richtlinie [Einstellungen enthält, die Genehmigung benötigen](/docs/de/server-managed-settings#security-approval-dialogs), sieht der Entwickler auch diesen Genehmigungsdialog erneut nach Akzeptanz des neuen Zertifikats, da Claude Code die [Genehmigungserinnerung](/docs/de/server-managed-settings#approval-memory) an das angeheftete Zertifikat bindet.

Ein Gateway kann das optionale Feld `email` in seiner Token-Antwort zurückgeben, um das Konto zu benennen, das eine Anmeldung verwendet hat. Wenn es das tut, bestätigt der Entwickler das Konto, bevor Claude Code die Anmeldedaten speichert. Nach einer bestätigten Anmeldung zeigt `/status` das Konto an.

Die Bestätigung erfordert Claude Code v2.1.275 oder später auf der Entwicklermaschine; ein Client unter dieser Version ignoriert das Feld. Der Gateway-Server in der `claude`-Binärdatei gibt das Feld nicht zurück, daher werden seine Anmeldungen ohne die Bestätigung abgeschlossen.

Sobald sich der Entwickler angemeldet hat, zeigt die [Modellauswahl](/docs/de/model-config) die Modelle in der `availableModels`-Zulassungsliste des Entwicklers. Verwaltete Einstellungen werden beim Start angewendet und stündlich aktualisiert, und Telemetrie wird an Ihren Collector weitergeleitet.

Sitzungen werden vor Ablauf von `ttl_hours` stillschweigend aktualisiert. Wenn eine Aktualisierung nach IdP-Entbereitstellung fehlschlägt, fordert Claude Code den Entwickler auf, sich erneut anzumelden.

<h3 id="set-the-gateway-url">
  Gateway-URL festlegen
</h3>

Drei Schlüssel gehen in die Pro-OS-[Datei mit verwalteten Einstellungen](/docs/de/managed-settings#delivery-mechanisms), die Sie über MDM oder direkt auf der Festplatte bereitstellen. `forceLoginMethod` und `forceLoginGatewayUrl` öffnen `/login` direkt auf dem **Cloud gateway**-Bildschirm mit der ausgefüllten URL, und `parentSettingsBehavior: "merge"` ermöglicht es Claude Desktop, die Egress-Zulassungsliste des Gateways an die Claude Code-Sitzungen zu liefern, die es startet, wie in [Richtlinie an Claude Desktop-Sitzungen liefern](#deliver-policy-to-claude-desktop-sessions) erläutert:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

Der Entwickler drückt Enter, um sich zu verbinden. Die [Fingerabdruck-Eingabeaufforderung beim ersten Verbinden](#connect-developers) wird immer noch angezeigt. Sobald die Datei auf einer Maschine vorhanden ist, sieht ein Entwickler, der die Gateway-Anmeldung nicht abgeschlossen hat, eine der unter [Administratorrichtlinie erfordert eine Cloud-Gateway-Anmeldung](/docs/de/errors#administrator-policy-requires-a-cloud-gateway-sign-in) beschriebenen Meldungen. Entwickler, die einen Cloud-Anbieter über eine Umgebungsvariable wie `CLAUDE_CODE_USE_BEDROCK` auswählen, benötigen die Gateway-Anmeldung nicht.

Ein Entwickler kann dies nicht manuell einrichten. Die Anmeldungsauswahl hat keine Gateway-Option, und `forceLoginGatewayUrl` wird in den eigenen Einstellungsdateien eines Entwicklers ignoriert. `forceLoginMethod` allein, ohne URL, lässt den Entwickler bei einer „Kontaktieren Sie Ihren IT-Administrator"-Nachricht. Die Anmeldeschlüssel gehören in die Datei, die Sie auf Maschinen pushen, nicht in den `managed.policies[].cli`-Block des Gateways, der nur bereits verbundene Clients erreicht.

<h3 id="allow-a-gateway-on-public-address-space-you-own">
  Ein Gateway auf öffentlichem Adressraum zulassen, den Sie besitzen
</h3>

Einige Organisationen nummerieren ihr internes Netzwerk aus einem öffentlichen IPv4-Block, den sie besitzen, wie z. B. den eigenen Adressraum eines Carriers oder ein Legacy-`/8`, daher kann ihr Gateway keine private Adresse haben. Listen Sie diese Blöcke in der verwalteten Einstellung `gatewayInternalNetworks` auf. `/login` akzeptiert dann ein Gateway in einem aufgelisteten Block, wenn sich die Maschine des Entwicklers von einer Adresse in demselben Block aus damit verbindet. Dies erfordert Claude Code v2.1.268 oder später auf der Entwicklermaschine; frühere Versionen ignorieren den Schlüssel und wenden die Regel für private Adressen an.

<Warning>
  `gatewayInternalNetworks` ist für interne Netzwerke, die zufällig aus öffentlichem Adressraum nummeriert sind. Es macht es nicht sicher, ein Gateway im Internet verfügbar zu machen: Ein vertrauenswürdiges Gateway kann Einstellungen pushen, die Befehle auf Entwicklermaschinen ausführen.

  Halten Sie das Gateway mit Ihren Firewall- oder Load-Balancer-Regeln von außerhalb Ihres Netzwerks unerreichbar. Setzen Sie die [`access_control.allow_cidrs`](/docs/de/claude-apps-gateway-config#http-tuning) des Gateways auf dieselben Blöcke, die Sie hier deklarieren, damit das Gateway selbst Clients von überall sonst ablehnt. Hinter einem Load Balancer oder Ingress setzen Sie auch `listen.trusted_proxies` auf dieses Front-End, da das Gateway sonst `allow_cidrs` gegen die eigene Adresse des Front-Ends statt gegen die des Entwicklers abgleicht.
</Warning>

Fügen Sie den Schlüssel zur gleichen verwalteten Einstellungsquelle wie die Anmeldeschlüssel hinzu: die Datei mit verwalteten Einstellungen, das MDM-Profil oder die Registrierungsrichtlinie. Claude Code ignoriert ihn in Benutzer-, Projekt- und Server-verwalteten Einstellungen.

Dieses Beispiel deklariert einen Block. Ersetzen Sie `203.0.113.0/24` durch Ihren eigenen Block. Es ist ein Dokumentationsbereich, und Claude Code lehnt diese ab.

```json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Claude Code validiert die Liste bei `/login`, bevor sie sich mit einem Gateway verbindet:

* Jeder Eintrag ist ein IPv4-Block, geschrieben als seine erste Adresse und ein Präfix von `/8` bis `/32`.
* Die Liste enthält höchstens vier Blöcke, und keine zwei überlappen sich.
* Kein Block überlappt privaten Adressraum: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `127.0.0.0/8`, `169.254.0.0/16` und `100.64.0.0/10`. `/login` akzeptiert ein Gateway dort bereits ohne diesen Schlüssel.
* Kein Block überlappt Raum, der niemals das Netzwerk einer Organisation ist: `198.18.0.0/15` und `192.0.0.0/24`, die VPN- und NAT64-Clients als lokale Adressen halten; die Dokumentationsbereiche `192.0.2.0/24`, `198.51.100.0/24` und `203.0.113.0/24`; und die reservierten Bereiche `0.0.0.0/8`, `192.88.99.0/24` und Multicast `224.0.0.0/4`. Sie können Blöcke in `240.0.0.0/4` deklarieren, die einige große Netzwerke als internen Unicast-Raum verwenden.

Blöcke aus `managed-settings.json` und seinen `managed-settings.d/`-Drop-in-Dateien kombinieren sich zu einer Liste, und diese Grenzen gelten für die kombinierte Liste. Um einen Block zu verengen, ersetzen Sie seinen Eintrag, anstatt einen zweiten, überlappenden in einem Drop-in hinzuzufügen; `/login` lehnt die Überlappung ab.

Wenn ein Eintrag eine Regel bricht oder der Wert keine Liste von Strings ist, lehnt Claude Code jede neue Gateway-Anmeldung auf dieser Maschine ab und benennt das Problem in der Nachricht. Die Anmeldung bei einem Gateway auf einer privaten Adresse schlägt ebenfalls fehl, und bestehende Anmeldungen funktionieren weiterhin. Versuchen Sie den Wert auf einer Maschine, bevor Sie ihn bereitstellen. Claude Code listet auch einen falsch typisierten Wert unter den [ungültigen verwalteten Einstellungen auf, die es meldet](/docs/de/managed-settings#keys-that-fail-closed).

Mit einer gültigen Liste wendet `/login` drei Überprüfungen auf ein Gateway an, dessen Adresse in einem aufgelisteten Block liegt:

* Jede Adresse, zu der sich der Hostname des Gateways auflöst, liegt in diesem einen Block. Claude Code lehnt einen Namen ab, der auch Datensätze außerhalb davon, private und IPv6-Adressen eingeschlossen, hat.
* Die Maschine des Entwicklers verbindet sich von innerhalb desselben Blocks. Claude Code lehnt eine Maschine hinter NAT, in einem Container oder WSL2 oder auf einem VPN ab, dessen Adresspool außerhalb des Blocks liegt, und benennt die Adresse, von der sich die Maschine verbunden hat.
* Die Verbindung ist direkt. Wenn `HTTPS_PROXY` für den Gateway-Host gilt, lehnt `/login` ab und benennt den `NO_PROXY`-Eintrag, der hinzugefügt werden soll.

Wenn alle drei bestanden werden, fügt die [Vertrauensaufforderung](#connect-developers) eine Zeile hinzu, die die Adresse der Maschine, die Adresse des Gateways und den deklarierten Block, der beide enthält, benennt.

Der Schlüssel ändert nichts für andere Gateways: Die Anmeldung bei einem auf einer privaten Adresse funktioniert wie zuvor, und die Anmeldung bei einem auf einer öffentlichen Adresse außerhalb jeden aufgelisteten Blocks wird wie zuvor abgelehnt.

Ein deklarierter Block verengt, wer sich anmelden kann, beweist aber nicht, wo sich eine Maschine befindet, daher deklarieren Sie nur Adressraum, den Ihre Organisation kontrolliert. Ein Block, der mit anderen Mandanten geteilt wird, wie z. B. ein öffentlicher Bereich eines Cloud-Anbieters, lässt jeden darin die gleiche Überprüfung bestehen.

<h3 id="deliver-policy-to-claude-desktop-sessions">
  Richtlinie an Claude Desktop-Sitzungen liefern
</h3>

Claude Desktop führt seine Cowork- und Code-Registerkarten sowie die Chat-Registerkarte, wenn Sie diese aktivieren, auf eingebetteten Claude Code-Sitzungen aus und sendet ihre Modellanfragen durch das Gateway. Es übergibt die Richtlinie an jede dieser Sitzungen, aufgebaut aus der Konfiguration, die das Gateway ihr unter `/user/bootstrap` bereitstellt: die Modellzulassungsliste, deaktivierte Tools und die Egress-Zulassungsliste, die aus dem `cli`-Block der übereinstimmenden Richtlinie abgeleitet ist, plus die [`desktop`-Überlagerung](/docs/de/claude-apps-gateway-config#claude-desktop-overlay).

Andere `cli`-Schlüssel, wie Hooks, `env` und bereichsbezogene Berechtigungsregeln wie `Bash(npm *)`, erreichen nur Clients, die sich über `/login` anmelden. Claude Desktop liest die Gateway-URL aus seiner eigenen verwalteten Konfiguration und meldet sich mit seinem eigenen Fluss an, getrennt von den `forceLoginMethod`- und `forceLoginGatewayUrl`-Schlüsseln in [Gateway-URL festlegen](#set-the-gateway-url).

Einstellungen, die von einem startenden Prozess übergeben werden, sind übergeordnete Einstellungen. Claude Code ignoriert übergeordnete Einstellungen auf jeder Maschine, die eine von einem Admin bereitgestellte verwaltete Quelle hat, es sei denn, die [Quelle, die die Richtlinie liefert](/docs/de/managed-settings#which-managed-source-claude-code-uses), setzt `parentSettingsBehavior: "merge"`.

<h4 id="which-machines-need-the-opt-in">
  Welche Maschinen benötigen die Opt-in
</h4>

Maschinen, die nur Claude Desktop ausführen, benötigen sie. Claude Desktop wendet die Modelliste und die Liste der deaktivierten Tools auf eingebettete Sitzungen selbst an, aber die Egress-Zulassungsliste erreicht sie nur als übergeordnete Einstellungen, in Form von `WebFetch`-Domänenregeln und Sandbox-Netzwerkregeln. Ohne die Opt-in führen diese Sitzungen ohne die Egress-Einschränkung aus, und nichts warnt Sie. Das Gateway lehnt weiterhin Inferenzanfragen für Modelle ab, die die Richtlinie nicht gewährt.

Maschinen, auf denen sich Entwickler über `/login` anmelden, benötigen sie nicht; jede Claude Code-Sitzung ruft ihre Richtlinie vom Gateway ab.

Flotten, deren [`policyHelper`](/docs/de/settings-reference#policyhelper) verwaltete Einstellungen liefert, können sie nicht verwenden: Claude Code führt übergeordnete Einstellungen auf diesen Flotten niemals zusammen, da es verwaltete Einstellungen nur aus der Ausgabe des Helpers liest.

<h4 id="set-the-opt-in">
  Opt-in festlegen
</h4>

Stellen Sie das Snippet für verwaltete Einstellungen aus [Gateway-URL festlegen](#set-the-gateway-url) bereit, spiegeln Sie es auf jede clientseitige Quelle, die die Datei überragt, und überprüfen Sie es.

<Steps>
  <Step title="Opt-in in der Datei mit verwalteten Einstellungen bereitstellen">
    Das [Snippet oben](#set-the-gateway-url) enthält bereits `parentSettingsBehavior: "merge"`, daher trägt die Datei, die Sie auf Maschinen pushen, sie.
  </Step>

  <Step title="Snippet auf jede Quelle spiegeln, die die Datei überragt">
    Claude Code liest `parentSettingsBehavior` nur aus der [ausgewählten Quelle](/docs/de/managed-settings#which-managed-source-claude-code-uses). Das Hinzufügen eines beliebigen Richtlinienschlüssels zu einer Quelle kann diese Quelle zur ausgewählten machen, daher spiegeln Sie in einer clientseitigen Quelle das gesamte Snippet statt nur `parentSettingsBehavior`. [Clientseitige verwaltete Einstellungen](/docs/de/claude-apps-gateway-config#client-side-managed-settings) behandelt Flotten, die Richtlinien über Gruppenrichtlinie oder Konfigurationsprofile liefern. Eine verwaltete Preferences-Plist auf macOS oder eine HKLM-Richtlinie auf Windows überragt die `managed-settings.json`-Datei, und die eigenen Remote-Verwaltungseinstellungen des Gateways überragen beide, daher setzen Sie auf Maschinen, die sich beim Gateway anmelden, auch `parentSettingsBehavior` im [`cli`-Block](/docs/de/claude-apps-gateway-config#managed) der Gateway-Richtlinie.
  </Step>

  <Step title="Überprüfen Sie, welche Quelle ausgewählt ist">
    Auf einer Maschine, die nur Claude Desktop ausführt, rufen Sie die [`resolveSettings()`](/docs/de/agent-sdk/typescript#resolvesettings) des Agent SDK auf und lesen Sie `policyOrigin` auf dem `managed`-Eintrag in seiner `sources`-Liste. Der Wert benennt die ausgewählte clientseitige Quelle, `plist`, `hklm` oder `file`, die die Quelle ist, die das Snippet tragen muss. Die eingebetteten Sitzungen von Claude Desktop rufen die Gateway-Richtlinie nicht ab, daher zählt der `cli`-Block des Gateways niemals als ausgewählte Quelle für sie.
  </Step>
</Steps>

<h3 id="restrict-parent-settings">
  Übergeordnete Einstellungen einschränken
</h3>

Sobald Sie `parentSettingsBehavior: "merge"` bereitstellen, kann jeder Host-Prozess, der Claude Code startet, übergeordnete Einstellungen liefern, nicht nur Claude Desktop, sondern auch eine Agent SDK-Anwendung oder eine IDE-Erweiterung.

Claude Code filtert übergeordnete Einstellungen gegen eine Zulassungsliste restriktiver Schlüssel, aber einige zulässige Schlüssel können Zugriff gewähren statt ihn einzuschränken. Wenn Sie die `allowManaged*Only`-Sperren nicht setzen, gelten Berechtigungszulassungsregeln und Sandbox-Zulassungslisten, die vom Host bereitgestellt werden, weiterhin. Die Deny- und Ask-Regeln Ihrer Richtlinie bleiben in jedem Fall gültig; [sie werden vor jeder Zulassungsregel ausgewertet](/docs/de/permissions#manage-permissions).

Claude Code leitet übergeordnete [`sandbox.credentials`](/docs/de/settings-reference#sandbox-credentials)-Einträge in gestreifter Form weiter:

* **`deny`-Einträge**: weitergeleitet mit nur ihrem `path` oder `name` und dem Modus.
* **Dateieinträge mit [`mode: mask`](/docs/de/sandboxing#mask-credential-files)**: weitergeleitet nur als Sentinel, als vollständige Dateimaskierung, deren `injectHosts` die leere Liste ist, daher ersetzt der Proxy niemals den echten Wert für einen übergeordneten Eintrag auf einer beliebigen Plattform. Alle strukturierten Maskierungsfelder werden ebenfalls gelöscht, daher kann ein übergeordnetes Extraktionsmuster keine strengere Maskierung verdrängen, die eine andere Quelle für denselben Pfad setzt.
* **`envVars`-Einträge mit `mode: mask`**: nicht weitergeleitet. `deny` ist die einzige Einschränkung, die der übergeordnete Kanal durch `envVars`-Einträge ausdrücken kann.
* **[`awsPairs` und `sigv4`](/docs/de/sandboxing#re-sign-aws-requests)**: Einschränkung nur weitergeleitet. Aus `sigv4` werden nur `deny`-Werte beibehalten, und ein übergeordneter, der einen `sigv4`-Block überhaupt definiert, heftet alle drei Anforderungsformen, `streaming`, `presigned` und `sigv4a`, an `deny`. Ein `awsPairs`-Paar wird niemals in einer Form weitergeleitet, die erneut signieren kann; ein Paar, das eine der konventionellen AWS-Variablen benennt, wird durch einen inerten Eintrag ersetzt, der die automatische Kopplung von `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` und `AWS_SESSION_TOKEN` unterdrückt.

<h4 id="deploy-the-locks">
  Sperren bereitstellen
</h4>

Um übergeordnete Einstellungen so nah wie möglich an restriktiv zu halten, wie der Filter unterstützt, fügen Sie alle fünf `allowManaged*Only`-Sperren und die Zulassungslisten, die sie regeln, zu denselben Quellen wie die Merge-Opt-in hinzu:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge",
  "allowManagedPermissionRulesOnly": true,
  "allowManagedMcpServersOnly": true,
  "allowManagedHooksOnly": true,
  "allowedMcpServers": [{ "serverUrl": "https://mcp.internal.example.com/*" }],
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    },
    "filesystem": {
      "allowManagedReadPathsOnly": true,
      "denyRead": ["~/"],
      "allowRead": ["~/projects"]
    }
  }
}
```

Eine OS-Richtlinie, wie eine HKLM-Registrierungsrichtlinie oder eine verwaltete Preferences-Plist, überragt diese Datei, daher liefern Sie das gesamte Snippet stattdessen durch sie. Die Remote-Verwaltungseinstellungen des Gateways überragen die OS-Richtlinie und Dateiquellen, erreichen aber nur verbundene Clients. Spiegeln Sie die Sperren, die Zulassungslisten und die Merge-Opt-in in den [`cli`-Block](/docs/de/claude-apps-gateway-config#managed) der Richtlinie und behalten Sie diese Datei bereitgestellt, da Maschinen, die sich niemals verbinden, einschließlich solcher, die nur Claude Desktop ausführen, ihre Richtlinie nur aus der Datei erhalten.

<h4 id="lock-behavior-across-sources">
  Sperrverhalten über Quellen hinweg
</h4>

Das Setzen einer Sperre schränkt die anderen nicht ein; jeder Schlüssel ist in der [Einstellungsreferenz](/docs/de/settings-reference#all-settings) dokumentiert. Aus einer Admin-Quelle unter dem Gewinner gelten die beiden Sandbox-Sperren immer noch, und `allowManagedPermissionRulesOnly` blockiert weiterhin übergeordnete Zulassungsregeln und `additionalDirectories`. Auf Claude Code v2.1.273 oder später gilt auch die MCP-Server-Sperre aus einer Quelle unter dem Gewinner, und während sie aktiv ist, kommt die verwaltete `allowedMcpServers`-Liste aus der höchsten Prioritäts-Admin-Quelle, die eine setzt.

Die Hooks-Sperre und die Auswirkung von `allowManagedPermissionRulesOnly` auf die eigenen Regeln des Entwicklers benötigen standardmäßig die Gewinnerquelle; unter der Merge-Opt-in `managedSourcesBehavior` in [wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources), wendet Claude Code den strengsten Wert an, den jede Quelle für jede Sperre setzt. Auf [`policyHelper`](/docs/de/settings-reference#policyhelper)-Flotten werden die Sperren nur aus der Ausgabe des Helpers gelesen.

Jede Sperre lässt Claude Code die eigenen Einträge des Entwicklers für diese Einstellung ignorieren, daher schließen Sie die Zulassungslisten Ihrer Organisation neben den Sperren ein:

* **Netzwerkdomänen**: Das Sperren mit einer leeren verwalteten Domänenliste blockiert den gesamten ausgehenden Sandbox-Verkehr.
* **MCP-Server**: Das Sperren ohne `allowedMcpServers` in einer Admin-Quelle oder in den übergeordneten Einstellungen lädt jeden Server, den `deniedMcpServers` nicht blockiert.
* **Lesepfade**: `allowRead`-Einträge erlauben nur Pfade in `denyRead`-Regionen erneut, daher koppeln Sie sie mit einem verwalteten `denyRead`.

<h4 id="settings-the-locks-don’t-cover">
  Einstellungen, die die Sperren nicht abdecken
</h4>

Sechs übergeordnete Einstellungen passieren den Filter auch mit allen fünf Sperren gesetzt. Unter der Standard-First-Wins-Einstellung ist der Admin-Wert, der den übergeordneten blockiert, der in der höchsten Prioritäts-Admin-Quelle, außer für `allowedMcpServers` während die [MCP-Server-Sperre](#lock-behavior-across-sources) aktiv ist. Unter der Merge-Opt-in `managedSourcesBehavior` sagt [wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources), welcher Quellenwert stattdessen gilt.

* **`forceLoginOrgUUID`**: Claude Code ehrt einen übergeordneten Wert, wenn die höchste Prioritäts-Admin-Quelle keine Organisations-UUID setzt. Gateway-Anmeldung überprüft diesen Schlüssel nicht, daher ist er nur für Flotten wichtig, die auch First-Party-Anthropic-Anmeldungen verwenden. Eine Organisations-UUID in der höchsten Prioritäts-Admin-Quelle blockiert den übergeordneten Wert und ist der, den Claude Code erzwingt, daher setzen Sie `forceLoginOrgUUID` dort.
* **`allowedMcpServers`**: Claude Code ehrt eine übergeordnete Zulassungsliste, wenn keine Admin-Liste in Kraft ist. `allowManagedMcpServersOnly` blockiert sie nicht, da die Sperre erzwingt, welche Liste auch immer gewinnt, als verwalteter Wert, einschließlich einer übergeordneten Liste, wenn keine Admin-Quelle eine Liste liefert. Eine Liste in der höchsten Prioritäts-Admin-Quelle blockiert die übergeordnete und ist die Liste, die Claude Code erzwingt, daher setzen Sie `allowedMcpServers` dort, neben der Sperre. Vor v2.1.223 blockierte ein Wert für einen der beiden Schlüssel in jeder Admin-Quelle den übergeordneten.
* **`availableModels`**: Claude Code ehrt eine übergeordnete Modelliste, wenn die Gewinnerquelle für verwaltete Einstellungen keine setzt. Wenn Ihre Flotte Modelle einschränkt, setzen Sie `availableModels` in der Gewinnerquelle.
* **`strictKnownMarketplaces`**: Claude Code ehrt eine übergeordnete Plugin-Marketplace-Zulassungsliste, wenn die Gewinnerquelle für verwaltete Einstellungen keine setzt. Wenn Ihre Flotte Marketplaces einschränkt, setzen Sie `strictKnownMarketplaces` in der Gewinnerquelle. Erfordert Claude Code v2.1.282 oder später.
* **`blockedMarketplaces`**: eine übergeordnete Marketplace-Blockliste passiert und addiert zu jeder Blockliste, die eine verwaltete Quelle setzt, da eine Blockliste nur weiter einschränken kann. Erfordert Claude Code v2.1.282 oder später.
* **`strictPluginOnlyCustomization`**: dieser Schlüssel passiert den Filter unabhängig von jeder Sperre, und er lässt Claude Code die eigene Anpassung des Entwicklers ignorieren, einschließlich schützender Hooks. Keine Sperre blockiert ihn.

<h3 id="connect-claude-desktop">
  Claude Desktop verbinden
</h3>

[Claude Desktop](/docs/de/desktop) verbindet sich mit demselben Gateway über einen anderen MDM-Schlüssel: setzen Sie `bootstrapUrl` in Claude Desktops [verwalteter Konfiguration](https://claude.com/docs/third-party/claude-desktop/configuration) auf `<listen.public_url>/user/bootstrap`, und opt-in die Richtlinie des Benutzers mit einem `desktop`-Schlüssel. [Claude Desktop-Überlagerung](/docs/de/claude-apps-gateway-config#claude-desktop-overlay) behandelt beide Hälften. Erfordert Claude Code v2.1.203 oder später auf dem Gateway-Server.

Claude Desktop meldet den Entwickler über den Identity Provider des Gateways mit demselben Browser-SSO-Schritt an, ruft dann seine Konfiguration vom Gateway statt von Anthropic ab. Modellzugriff und Richtlinie folgen denselben Pro-Gruppen-Regeln wie die CLI. Ein Entwickler, der sowohl die CLI als auch Claude Desktop verwendet, meldet sich bei jedem separat an; die Gateway-Sitzung wird nicht zwischen ihnen geteilt.

Nach der Verbindung sendet Claude Desktop Modellanfragen von jeder aktivierten Registerkarte durch das Gateway. Es zeigt die Cowork- und Code-Registerkarten standardmäßig an. Um die Chat-Registerkarte auch zu aktivieren, setzen Sie `chatTabEnabled` auf `true` in Claude Desktops [verwalteter Konfiguration](https://claude.com/docs/third-party/claude-desktop/configuration), oder im [`desktop`-Block](/docs/de/claude-apps-gateway-config#claude-desktop-overlay) der Richtlinie auf einem Gateway, das Claude Code v2.1.227 oder später ausführt.

<h3 id="ci-pipelines-and-remote-machines">
  CI-Pipelines und Remote-Maschinen
</h3>

Es gibt keinen Service-Token-Fluss für unbeaufsichtigte Pipelines. Gateway-Anmeldung führt immer den Browser-Gerätefluss aus, daher kann ein CI-Job ohne Entwickler zur Genehmigung der Anmeldung nicht authentifizieren; konfigurieren Sie diese direkt gegen Ihren Anbieter.

Sobald sich ein Entwickler angemeldet hat, verwendet jede Claude Code-Sitzung auf dieser Maschine die Gateway-Sitzung, einschließlich nicht-interaktiver `claude -p`-Läufe und Sitzungen, die vom Agent SDK gestartet werden. Claude Code wendet die [Gateway-Richtlinie](/docs/de/claude-apps-gateway-config#managed) auf jede von ihnen an.

Der Gerätefluss trennt die Polling-CLI vom genehmigenden Browser, daher funktioniert eine Remote-Entwicklungsbox ohne Display immer noch: Der Entwickler führt `/login` über SSH auf der Remote-Maschine aus und öffnet den Überprüfungslink im Browser auf seinem Laptop.

<h3 id="whats-enforced-on-developers">
  Was auf Entwickler erzwungen wird
</h3>

Diese Garantien gelten für jede Sitzung, die sich über `/login` angemeldet hat. Die eingebetteten Sitzungen, die Claude Desktop startet, erhalten ihre Richtlinie wie in [Richtlinie an Claude Desktop-Sitzungen liefern](#deliver-policy-to-claude-desktop-sessions) beschrieben, und die Telemetrie-Aufzählung sagt, wohin ihre Exporte gehen.

* **Modellzugriff**: Anfragen für Modelle, die die Richtlinie nicht gewährt, geben 400 zurück, und die `/model`-Auswahl wird auf die `availableModels`-Zulassungsliste der Richtlinie gefiltert. Setzen Sie [`enforceAvailableModels: true`](/docs/de/model-config#default-model-behavior) in der Richtlinie, damit die Standard-Option zu einem Modell in `availableModels` aufgelöst wird, anstatt zu Claude Codes integriertem Standard; ohne sie bleibt Standard wählbar und wird bei der Anfrageverarbeitung abgelehnt, wenn dieses Modell nicht gewährt wird.
* **Telemetrie-Ziel**: In Sitzungen, die sich über `/login` angemeldet haben, sendet die CLI ihre OTLP/HTTP-Exporte an das Gateway statt an einen lokal gesetzten `OTEL_EXPORTER_OTLP_ENDPOINT`, es sei denn, eine Richtlinie [benennt Ihren Collector als Endpunkt](/docs/de/claude-apps-gateway-config#export-directly-to-your-collector). Das Gateway leitet die Exporte, die es empfängt, an die Ziele in [`telemetry.forward_to`](/docs/de/claude-apps-gateway-config#telemetry) weiter.
  * In den eingebetteten Sitzungen, die [Claude Desktop startet](#connect-claude-desktop), sendet die CLI ihre Exporte an den konfigurierten `OTEL_EXPORTER_OTLP_ENDPOINT`. Die CLI hängt das Gateway-Sitzungstoken an diese Exporte nur an, wenn dieser Endpunkt auf das Gateway selbst zeigt.
  * Ohne konfiguriertes Ziel für ein Signal akzeptiert das Gateway es und verwirft es.
  * Wenn Sie bereits Claude Code-Telemetrie direkt erfassen, fügen Sie Ihren Collector als `forward_to`-Ziel hinzu, oder benennen Sie ihn in einer Richtlinie, um das Relay zu überspringen.
* **Anmeldedaten**: Das Gateway-Token ist die einzige Anmeldedaten der Sitzung. [Anthropic-Profile](/docs/de/authentication#anthropic-profiles-and-federation-credentials) und jede frühere claude.ai-Anmeldung werden ignoriert, während angemeldet, daher müssen sich Entwickler nicht zuerst von claude.ai abmelden. Für einen konfigurierten `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper`-Anmeldedaten siehe [Administratorrichtlinie erfordert eine Cloud-Gateway-Anmeldung](/docs/de/errors#administrator-policy-requires-a-cloud-gateway-sign-in).
* **Verwaltete Einstellungen**: Gesperrte Schlüssel können nicht lokal überschrieben werden. Die CLI wendet die Richtlinie beim Start an und wendet Änderungen bei jeder stündlichen Abfrage an, abgesehen von den [Änderungen, die nur beim nächsten Start angewendet werden](/docs/de/server-managed-settings#fetch-and-caching-behavior).
* **Startup mit dem Gateway unerreichbar**: Angemeldete Sitzungen beenden sich beim Start mit einem Fehler nach etwa 10 Sekunden, anstatt ohne ihre Einstellungen zu starten.
* **Startup nach dem Gateway beendet die Sitzung**: siehe [Fail-Closed-Startup erzwingen](/docs/de/server-managed-settings#enforce-fail-closed-startup) für die Starts, die abgemeldet vom Gateway öffnen, und die, die beenden, wenn das Gateway mit einem `401` antwortet.
* **Entbereitstellung**: Eine Sitzung, deren Benutzer im IdP deaktiviert ist, läuft innerhalb von `ttl_hours` ab, wenn die nächste Aktualisierung fehlschlägt.
* **Abmeldung**: `/logout` löscht die Gateway-Anmeldedaten von der Maschine des Entwicklers.
  * Wenn das Discovery-Dokument des Gateways einen `revocation_endpoint` auf dem eigenen Schema, Host und Port der Gateway-URL bewirbt, sendet `/logout` auch die gespeicherten Token an diesen Endpunkt, damit das Gateway die Sitzung auf seiner Seite beenden kann. Die Anfrage ist Best Effort, daher wird die Abmeldung auf der Maschine des Entwicklers abgeschlossen, unabhängig davon, ob der Endpunkt antwortet. Die Sperrung erfordert Claude Code v2.1.275 oder später auf der Entwicklermaschine.
  * Der Gateway-Server in der `claude`-Binärdatei bewirbt keine, daher beendet eine Abmeldung von ihm die Sitzung auf der Maschine des Entwicklers nur. Um Sitzungen serverseitig zu erzwingen, siehe [JWT-Geheimnis-Rotation](/docs/de/claude-apps-gateway-deploy#jwt-secret-rotation).

<h3 id="what-the-organization-can-see">
  Was die Organisation sehen kann
</h3>

Nutzungstelemetrie trägt die Identität des Entwicklers, Token-Zählungen, Modell und Latenz zum Collector der Organisation. Das Gateway protokolliert oder speichert keinen Prompt- oder Completion-Inhalt. Ob umfangreichere Telemetrie wie Protokolle und Traces erfasst wird, die Befehle und Dateipfade enthalten können, ist die [Pro-Ziel-Wahl](/docs/de/claude-apps-gateway-config#telemetry) der Organisation.

<h2 id="availability-and-limitations">
  Verfügbarkeit und Einschränkungen
</h2>

Die Tabelle behandelt, welche Claude-Code-Funktionen funktionieren, wenn Entwickler sich über das Gateway verbinden, und was der Gateway-Server selbst unterstützt. Wenn etwas nicht unterstützt wird, gibt die Spalte Notizen die Alternative an.

Das Gateway liefert die [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers)-Werte, die die CLI an jeden Upstream sendet, daher verwalten Operatoren keine Beta-Zulassungsliste. Für Amazon Bedrock, das den Header ignoriert, verschiebt das Gateway die Werte in das `anthropic_beta`-Feld des Request-Body; die anderen Upstreams erhalten den Header wie gesendet.

| Funktion                                                                                                                        | Status               | Notizen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------------------------------------------------------------------------------------------------------------------- | -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Inferenz-Weiterleitung (Amazon Bedrock, Claude Platform auf AWS, Agent Platform von Google Cloud, Microsoft Foundry, Anthropic) | Verfügbar            | Mit Pro-Upstream-Modellübersetzung und Failover. Der Amazon-Bedrock-Upstream verwendet den `bedrock-runtime`-Endpunkt und die AWS-Standard-Anmeldekette; der Amazon-Bedrock-[Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint) ist kein unterstützter Upstream. Der [Claude-Platform-auf-AWS-Upstream](/docs/de/claude-apps-gateway-config#claude-platform-on-aws) erfordert Claude Code v2.1.198 oder später auf dem Gateway-Server.                                                                                                                    |
| Modellzugriff und verwaltete Einstellungen nach IdP-Gruppe                                                                      | Verfügbar            | Modellzugriff wird auf der Serverseite erzwungen; verwaltete Einstellungen werden pro IdP-Gruppe bereitgestellt und von der CLI auf der [Ebene der verwalteten Einstellungen](/docs/de/settings#settings-precedence) angewendet                                                                                                                                                                                                                                                                                                                                     |
| Claude Desktop                                                                                                                  | Verfügbar mit Opt-in | Das Gateway stellt die Konfiguration von Claude Desktop unter `/user/bootstrap` bereit, sobald eine Richtlinie [sich mit einem `desktop`-Schlüssel anmeldet](/docs/de/claude-apps-gateway-config#claude-desktop-overlay), und Claude Desktop sendet Modellanfragen von seinen Cowork- und Code-Registerkarten sowie von der Chat-Registerkarte, wenn Sie diese aktivieren, über das Gateway. Um die Chat-Registerkarte zu aktivieren, siehe [Claude Desktop verbinden](#connect-claude-desktop). Erfordert Claude Code v2.1.203 oder später auf dem Gateway-Server. |
| Telemetrie-Verteilung (OTLP/HTTP)                                                                                               | Verfügbar            | Identitäts-gestempelt pro Export; sowohl Protobuf- als auch JSON-Codierungen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| OIDC-Identitätsanbieter                                                                                                         | Verfügbar            | Alle OIDC-konformen IdPs; das Gateway führt Standard-OIDC-Erkennung und den Authorization-Code-Flow durch. Siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup) für Pro-IdP-Konfiguration                                                                                                                                                                                                                                                                                                                                  |
| Pro-Benutzer- und Pro-Gruppen-Ausgabenlimits                                                                                    | Verfügbar            | Siehe [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Serverseitige Websuche                                                                                                          | Nicht verfügbar      | Die CLI kann nicht sehen, welchen Upstream-Anbieter das Gateway leitet, daher kann sie Websuche-Unterstützung nicht überprüfen und deaktiviert WebSearch auf Gateway-Sitzungen                                                                                                                                                                                                                                                                                                                                                                                 |
| [Remote Control](/docs/de/remote-control)                                                                                            | Nicht verfügbar      | Die CLI zeigt [einen Fehler an, der das Gateway benennt](/docs/de/errors#remote-control-requires-the-anthropic-api)                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| [`/design-sync`](/docs/de/commands#all-commands) und `/design-login`                                                                 | Nicht verfügbar      | Beide benötigen claude.ai, das die CLI auf Gateway-Sitzungen nicht kontaktiert, daher erscheint keiner der Befehle dort                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Funktionen, die Abrufen von Feature-Flags benötigen, wie `/import` und `claude import`                                          | Nicht verfügbar      | Die CLI überspringt den Flag-Abruf auf Gateway-Sitzungen. [Funktionen, die Abrufen von Feature-Flags benötigen](/docs/de/env-vars#features-that-need-feature-flag-fetching) listet auf, was das deaktiviert                                                                                                                                                                                                                                                                                                                                                         |
| Standard-Prompt-Caching                                                                                                         | Verfügbar            | Das Gateway leitet `cache_control`-Breakpoints an jeden Upstream weiter. [Wo der Cache lebt](/docs/de/prompt-caching#where-the-cache-lives) behandelt, welche Blöcke die CLI markiert, einschließlich des Systemkontexts, den sie mitten im Gespräch anhängt                                                                                                                                                                                                                                                                                                        |
| 1-Stunden-Cache-TTL                                                                                                             | Nicht verfügbar      | Die CLI lässt die Extended-Cache-TTL-Beta auf Gateway-Sitzungen aus, da nicht jeder Upstream, zu dem das Gateway leiten kann, die 1-Stunden-TTL unterstützt, daher verwendet Prompt-Caching über das Gateway die 5-Minuten-TTL; siehe die Beta-Header-Notiz oben                                                                                                                                                                                                                                                                                               |
| Auto-Modus                                                                                                                      | Verfügbar            | Folgt den [Regeln für Drittanbieter-Anbieter](/docs/de/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): Nur die Modelle, die auf Drittanbieter-Anbietern berechtigt sind, können es verwenden. Vor v2.1.207 erforderte Auto-Modus auf Gateway-Sitzungen das Setzen von `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, lieferbar über den verwalteten Richtlinien-`env`-Block                                                                                                                                                                             |
| First-Party-Only-Optimierungen wie globaler Cache-Umfang und Token-effiziente Tools                                             | Nicht verfügbar      | Die CLI aktiviert sie nicht auf Gateway-Sitzungen; siehe die Beta-Header-Notiz oben                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| OTLP/gRPC                                                                                                                       | Nicht unterstützt    | Nur OTLP über HTTP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| SAML, LDAP und andere nicht-OIDC-Auth                                                                                           | Nicht unterstützt    | Nur OIDC. Front mit einer OIDC-Brücke, falls erforderlich                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Multi-Tenant (mehrere OIDC-Aussteller)                                                                                          | Nicht unterstützt    | Ein Aussteller pro Gateway. Führen Sie separate Instanzen aus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Windows-Server                                                                                                                  | Nicht unterstützt    | Bereitstellung auf Linux. macOS nur für lokale Entwicklung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Helm-Diagramm                                                                                                                   | Nicht verfügbar      | Das Gateway läuft als Standard-Stateless-Deployment; siehe den [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Admin-UI                                                                                                                        | Nicht verfügbar      | Konfiguration ist die YAML-Datei; stellen Sie erneut bereit, um sie zu ändern                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h2 id="next-steps">
  Nächste Schritte
</h2>

Der Schnellstart lässt Sie mit einer minimalen Konfiguration unter Docker Compose. Um es weiter zu bringen:

* Erweitern Sie `gateway.yaml` über die minimale Konfiguration hinaus, um beispielsweise Pro-Gruppen-RBAC, Multi-Upstream-Failover oder Telemetrie-Ziele hinzuzufügen. Die [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config) behandelt jede Option.
* Wechseln Sie von Compose zu einer Produktionsbereitstellung auf Kubernetes oder Cloud Run, richten Sie Ihren IdP ordnungsgemäß ein und überprüfen Sie das Sicherheitsmodell. Der [Bereitstellungs- und Operationsleitfaden](/docs/de/claude-apps-gateway-deploy) behandelt IdP-spezifische Einrichtung, Container-Image-Anforderungen, Health-Probes und Fehlerbehebung.
* Legen Sie Ausgabengrenzen für einzelne Entwickler oder Gruppen fest, damit eine unkontrollierte Workload Ihre gesamte Verpflichtung nicht verbrauchen kann. [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) behandelt die Admin-API und wie die Durchsetzung funktioniert.
* Für ein vollständiges durchgearbeitetes Beispiel auf AWS mit ECS Fargate oder EKS, Amazon RDS und Secrets Manager siehe [Bereitstellung auf AWS](/docs/de/claude-apps-gateway-on-aws).
* Für ein vollständiges durchgearbeitetes Beispiel auf Google Cloud mit Cloud Run, Cloud SQL und Secret Manager siehe [Bereitstellung auf Google Cloud](/docs/de/claude-apps-gateway-on-gcp).
