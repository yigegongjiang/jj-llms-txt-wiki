> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude-Apps-Gateway für Amazon Bedrock, Claude Platform auf AWS, Google Cloud und Microsoft Foundry

> Führen Sie Claude Code über Amazon Bedrock, Claude Platform auf AWS, Google Cloud oder Microsoft Foundry hinter einem selbstgehosteten Gateway mit SSO-Anmeldung, Modellzugriff pro Gruppe und OTLP-Telemetrie aus.

<Note>
  Das Claude-Apps-Gateway ist für Organisationen konzipiert, die Inferenzen über ihren eigenen Cloud-Anbieter leiten müssen oder möchten – beispielsweise um [Anforderungen zur Datenresidenz](/docs/de/claude-apps-gateway-deploy#compliance-posture) zu erfüllen. Wenn Sie diese Anforderung nicht haben und Zugriff auf andere Funktionen wie SCIM-Bereitstellung oder Claude Code im Web und auf Mobilgeräten wünschen, ist Claude Enterprise möglicherweise besser geeignet. Weitere Informationen finden Sie auf der Seite zur [Funktionsverfügbarkeit](/docs/de/feature-availability), um einen vollständigen Vergleich aller Bereitstellungsmethoden zu erhalten.
</Note>

Claude-Apps-Gateway ist ein selbstgehosteter Service, der sich zwischen den Claude-Code-Clients Ihrer Entwickler und Ihrem Modellanbietern befindet. Entwickler melden sich mit Ihrem Unternehmensidentitätsanbieter (IdP) an, anstatt API-Schlüssel oder Cloud-Anmeldedaten zu halten. Das Gateway hält die Upstream-Anmeldedaten, erzwingt Modellzugriff und [verwaltete Einstellungen](/docs/de/permissions#managed-settings) nach IdP-Gruppe und leitet Nutzungstelemetrie an Ihren eigenen Observability-Stack weiter.

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
* **Zugriffskontrolle**: Ihre IdP-Gruppen werden Modellzulassungslisten und [verwaltete Einstellungen](/docs/de/permissions#managed-settings)-Richtlinien zugeordnet. Das Gateway erzwingt Modellzugriff auf der Serverseite, lehnt Anfragen für nicht gewährte Modelle ab und wählt die verwaltete Einstellungsrichtlinie jeder Gruppe aus, die die CLI auf der [Ebene der verwalteten Einstellungen](/docs/de/settings#settings-precedence) anwendet. Verschiedene Teams erhalten verschiedene Modelle, Tools und Berechtigungen, und ein Entwickler kann nicht überschreiben, was seine Richtlinie sperrt.
* **Einstellungsbereitstellung**: Das Gateway liefert verwaltete Einstellungen selbst an angemeldete Clients und ersetzt [servergesteuerte Einstellungen](/docs/de/server-managed-settings) aus der claude.ai-Admin-Konsole.
* **Telemetrie**: Jedes konfigurierte Ziel, wie Datadog, Splunk oder ClickHouse, empfängt [OpenTelemetry-Protokoll (OTLP)-Metriken](/docs/de/monitoring-usage) mit Token-Zählungen, Modell, Benutzeridentität und Latenz standardmäßig, mit Protokollen und Traces als optionale Ziele.
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

Die [Gateway-Protokollreferenz](/docs/de/llm-gateway-protocol) dokumentiert den Vertrag, den Claude Code von jedem Gateway erwartet: die Endpunkte, die es aufruft, die Header und Body-Felder zum Weiterleiten und was nicht funktioniert, wenn sie entfernt werden. Ein laufendes Claude-Apps-Gateway bedient eine Obermenge dieses Vertrags unter `GET /protocol`, wobei die Claude-Apps-Gateway-spezifischen Endpunkte für SSO-Anmeldung, Verwaltete-Einstellungen-Bereitstellung und Telemetrie hinzugefügt werden. Rufen Sie es mit `curl https://claude-gateway.internal.example.com/protocol` von jedem bereitgestellten Gateway ab, wie dem, das der [Schnellstart](#quickstart) unten erzeugt.

Breaking Changes am Protokoll werden im Voraus angekündigt, aber unbegrenzte Rückwärtskompatibilität ist nicht garantiert.

<h2 id="quickstart">
  Schnellstart
</h2>

Dieser Schnellstart führt den minimalen Weg: Registrieren Sie einen OAuth-Client in Ihrem IdP, schreiben Sie eine `gateway.yaml`, führen Sie das Gateway zusammen mit Postgres mit Docker Compose aus und überprüfen Sie die Anmeldung von Ende zu Ende. Es verwendet einen Amazon-Bedrock-Upstream; Claude Platform auf AWS, Google Clouds Agent Platform, Microsoft Foundry und die Anthropic API werden gleichermaßen unterstützt, indem Sie den `upstreams`-Block wie in der [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config#upstreams) gezeigt austauschen. Am Ende haben Sie ein Gateway, bei dem sich ein Entwickler `/login` anmelden kann.

<Note>
  **Bereitstellung in Ihrem privaten Netzwerk.** Claude Code verbindet sich nur mit einem Gateway, dessen Adresse privat ist. Dies ist eine Sicherheitsmaßnahme, da ein vertrauenswürdiges Gateway Einstellungen pushen kann, die Befehle auf Entwicklermaschinen ausführen. Platzieren Sie das Gateway hinter einem internen Load Balancer oder VPN und geben Sie ihm einen Hostnamen, der nur zu privaten IPs aufgelöst wird.

  Anthropic-betriebene öffentliche Gateway-Endpunkte sind die Ausnahme: `/login` akzeptiert sie über `https://`. Dies ist eine kleine feste Menge von Gateways, die Anthropic selbst betreibt; sie sind keine Bereitstellungsoption, die Sie auswählen oder konfigurieren können. Die Liste ist in Claude Code kompiliert, daher kann keine Konfiguration einen Hostnamen hinzufügen und kein von Ihnen gehostetes Gateway qualifiziert sich für die Ausnahme. Vor v2.1.206 lehnte `/login` diese Endpunkte wie jede andere öffentliche Adresse ab.
</Note>

<h3 id="prerequisites">
  Voraussetzungen
</h3>

Haben Sie diese vor dem Start bereit:

| Sie benötigen                            | Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 oder später         | Der `claude gateway`-Unterbefehl und der Gateway-Anmeldungsfluss werden in v2.1.195 ausgeliefert. Frühere öffentliche Builds enthalten sie nicht. Sowohl die Maschine, auf der der Gateway-Server läuft, als auch die Maschine jedes Entwicklers müssen v2.1.195 oder später sein; führen Sie `claude update` aus, um die neueste Version zu erhalten. Die [Claude Platform auf AWS Upstream](/docs/de/claude-apps-gateway-config#claude-platform-on-aws) erfordert Claude Code v2.1.198 oder später auf dem Gateway-Server.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| OpenID Connect (OIDC)-Identitätsanbieter | Okta, Microsoft Entra ID, Google Workspace, Keycloak oder Dex, oder ein anderer OIDC-konformer IdP wie PingFederate. Das Gateway führt Standard-OIDC-Erkennung und den Authorization-Code-Flow dagegen aus. SAML und LDAP werden nicht unterstützt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| PostgreSQL 14 oder später                | Unterstützt den Geräte-Anmeldungsfluss, bei dem der Browser-Callback schreibt und die Polling-CLI liest, sowie Rate-Limit-Zähler. Jedes verwaltete Postgres funktioniert, einschließlich der kleinsten Stufe. Ohne konfigurierte Ausgabenlimits speichert das Gateway einige KB kurzlebigen Auth-Status; mit [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) enthält es auch dauerhafte Ausgaben-, Audit- und Identitätstabellen, die gesichert werden sollten. TLS über `?sslmode=require` wird empfohlen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Modell-Upstream                          | Amazon-Bedrock-Anmeldedaten, Claude Platform auf AWS Anmeldedaten, Google-Cloud-Anmeldedaten, eine Microsoft-Foundry-Ressource oder einen Anthropic-API-Schlüssel. Mehrere Upstreams werden mit Failover unterstützt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| HTTPS                                    | Das Gateway muss über `https://` von Entwickler-Laptops und von jedem Browser, der für die Anmeldung verwendet wird, erreichbar sein; das Gateway bedient die Geräteüberprüfungsseite auf dem gleichen Listener. Stellen Sie entweder ein TLS-Zertifikat über `listen.tls` bereit, oder führen Sie hinter einem TLS-terminierenden Ingress aus und setzen Sie `listen.public_url`. Ein einfacher `http://`-Ursprung wird nur auf Loopback für lokale Entwicklung akzeptiert.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Private-Netzwerk-Adresse                 | Bei `/login` erfordert Claude Code, dass der Hostname oder die IP-Adresse des Gateways nur zu privaten Adressen aufgelöst wird: RFC 1918, CGNAT `100.64.0.0/10`, IPv6 ULA `fc00::/7` oder Loopback für lokale Entwicklung. Die Überprüfung wird auf jeder aufgelösten IP ausgeführt, daher lehnt `/login` die URL ab, wenn eine Adresse, zu der der Name aufgelöst wird, öffentlich ist. Wenn Entwicklermaschinen HTTPS über einen Unternehmens-Proxy leiten, erfordert die Anmeldung auch, dass der Proxy-Host zu privaten Adressen aufgelöst wird; wenn nicht, fügen Sie den Gateway-Host zu `NO_PROXY` hinzu, damit die CLI direkt verbunden wird. Anthropic-betriebene öffentliche Gateway-Endpunkte sind von den Überprüfungen für private Adressen und Proxys ausgenommen: `/login` akzeptiert sie über `https://` durch exakte Hostname-Übereinstimmung, daher gilt die Private-Netzwerk-Anforderung nur für ein von Ihnen selbst gehostetes Gateway. Vor v2.1.206 lehnte `/login` einen Anthropic-betriebenen Endpunkt wie jede andere öffentliche Adresse ab. |
| Linux-Laufzeit                           | Der Gateway-Server läuft nur auf der nativen Linux-Binärdatei. macOS funktioniert für lokale Entwicklung. Windows wird nicht als Serverplattform unterstützt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

Der Gateway-Server erfordert die native `claude`-Binärdatei; laden Sie eine angeheftete Version wie in [Claude Code installieren](/docs/de/setup) beschrieben herunter. Der Server verwendet Laufzeitfunktionen, die nicht verfügbar sind, wenn Claude Code unter Node ausgeführt wird. Wenn Sie `requires the native binary` beim Start sehen, wechseln Sie zu einer der eigenständigen Installationsmethoden.

<h3 id="steps">
  Schritte
</h3>

<Steps>
  <Step title="Registrieren Sie einen OAuth-Client in Ihrem IdP">
    Entscheiden Sie zuerst den Hostnamen des Gateways, da der Redirect-URI damit übereinstimmen muss. Erstellen Sie eine neue OIDC-Webanwendung und setzen Sie den Redirect-URI auf `https://claude-gateway.<your-domain>/oauth/callback`, wobei der Host der gleiche Wert ist, den Sie als [`listen.public_url`](/docs/de/claude-apps-gateway-config#listen) in Schritt 3 setzen. Notieren Sie sich die `client_id` und `client_secret`. IdP-spezifische Anweisungen finden Sie unter [Identitätsanbieter-Einrichtung](/docs/de/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Stellen Sie eine PostgreSQL-Datenbank bereit">
    Jedes Postgres 14 oder später funktioniert, einschließlich der kleinsten verwalteten Stufe. Das Gateway führt seine eigenen Schema-Migrationen beim Start aus, daher benötigt der Datenbankbenutzer `CREATE TABLE`-Berechtigung. Wenn Ihre Sicherheitsrichtlinie DDL von Anwendungsrollen verbietet, erstellen Sie das Schema stattdessen voraus; siehe [`store`](/docs/de/claude-apps-gateway-config#store).
  </Step>

  <Step title="Schreiben Sie gateway.yaml">
    Geheimnisse werden über `${ENV_VAR}`-Erweiterung gelesen, daher kann die Datei selbst in der Versionskontrolle leben. Verwenden Sie einen `public_url`-Hostnamen, der zu einer privaten IP in Ihrem Netzwerk aufgelöst wird, da `/login` öffentliche Adressen ablehnt. Die minimale Konfiguration hat fünf Abschnitte, und jedes andere Feld hat einen Standard:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Erforderlich hinter jedem TLS-terminierenden Proxy. Wird für den IdP
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
      Der Bedrock-Upstream benötigt einen AWS-Principal mit `bedrock:InvokeModel` und `bedrock:InvokeModelWithResponseStream` auf beiden `inference-profile/us.anthropic.*`-ARNs und den zugrunde liegenden `foundation-model/anthropic.*`-ARNs, und Modellzugriff muss in der Bedrock-Konsole für die Claude-Modelle aktiviert sein, die Sie möchten. Stellen Sie die Anmeldedaten mit IRSA auf EKS, einer ECS-Task-Rolle oder einem EC2-Instance-Profil bereit, anstatt statische Schlüssel zu verwenden. Die [`upstreams`-Referenz](/docs/de/claude-apps-gateway-config#upstreams) hat die vollständigen IAM-Details, die Cloud-übergreifende Anmeldedaten-Matrix und die `auth`-Blöcke für die anderen Anbieter.
    </Note>
  </Step>

  <Step title="Führen Sie es aus">
    Erstellen Sie ein Container-Image um die `claude`-Binärdatei, die die [Image-Anforderungen](/docs/de/claude-apps-gateway-deploy#container-image) erfüllt, und führen Sie es zusammen mit Postgres aus:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: <your-registry>/claude-gateway:<version>
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

    Das Gateway ist eine einzelne Linux-Binärdatei, die die Konfiguration liest, OIDC-Erkennung gegen Ihren IdP ausführt, ihre Postgres-Schema-Migrationen anwendet, Upstream-Clients erstellt und mit dem Abhören beginnt. Der Start ist fail-closed für die Konfiguration, die Postgres-Verbindung mit einem 5-Sekunden-Timeout, OIDC-Erkennung und Upstream-Client-Konstruktion. Wenn einer dieser Punkte unerreichbar oder falsch konfiguriert ist, beendet sich das Gateway mit einem Fehler, anstatt Datenverkehr in einem degradierten Zustand zu bedienen.

    Ein erfolgreicher Start validiert nicht den Inferenzpfad, da Bedrock- und Agent-Platform-Instance-Anmeldedaten bei der ersten Anfrage aufgelöst werden, nicht beim Start.

    Beobachten Sie stderr auf die Boot-Sequenz. Log-Zeilen verwenden das Format `[gateway] <timestamp> <level> <message>`, Audit-Events sind einzeilige JSON mit einem `evt`-Feld, und ein Startup-Banner, unten weggelassen, wird zwischen der Migration und den Listening-Zeilen gedruckt. Sie sollten in dieser Reihenfolge sehen:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

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
    Dieser letzte Schritt erfolgt auf einer Entwicklermaschine, nicht auf dem Server. Setzen Sie `forceLoginMethod` auf `"gateway"` und `forceLoginGatewayUrl` auf die `public_url` Ihres Gateways in der [verwalteten Einstellungsdatei](/docs/de/settings#settings-files) dieser Maschine, führen Sie dann `/login` aus, drücken Sie Enter auf dem **Cloud gateway**-Bildschirm und schließen Sie die Browser-Anmeldung ab. [Gateway-URL festlegen](#set-the-gateway-url) unten behandelt die Verteilung beider Schlüssel im großen Maßstab.
  </Step>
</Steps>

<h2 id="connect-developers">
  Entwickler verbinden
</h2>

Entwickler verbinden sich von ihren eigenen Laptops mit einer Browser-Anmeldung, indem sie ihr Unternehmensarbeitskonto verwenden. Sie benötigen kein claude.ai-Konto, keinen API-Schlüssel und kein Abonnement, da Anfragen an das Modell über das Gateway mit den Upstream-Anmeldedaten der Organisation gehen. Die Verbindung wird durch die [clientseitigen verwalteten Einstellungen](/docs/de/claude-apps-gateway-config#client-side-managed-settings) gesteuert, die Sie über MDM pushen, daher gibt es keine manuelle Einrichtung auf der Entwicklerseite; dieser Abschnitt behandelt, was der Admin konfiguriert.

Die CLI fingerabdruckt das TLS-Blatt-Zertifikat des Gateways beim ersten Verbinden und heftet es pro Hostname an. Veröffentlichen Sie den erwarteten SHA-256-Fingerabdruck zusammen mit der Gateway-URL, damit Entwickler etwas zum Vergleichen haben. Rufen Sie den Fingerabdruck aus der Zertifikatsdatei mit `openssl x509 -noout -fingerprint -sha256 -in cert.pem` ab; die `/login`-Eingabeaufforderung zeigt die ersten 16 Zeichen des Digest als Kleinbuchstaben-Hexadezimal ohne Trennzeichen.

Wenn das Zertifikat rotiert, sieht jeder Entwickler die Vertrauensaufforderung erneut, daher behandeln Sie Rotationen als geplantes Ereignis und veröffentlichen Sie den Fingerabdruck erneut.

Nach der Anmeldung zeigt die [Modellauswahl](/docs/de/model-config) die Modelle in der `availableModels`-Zulassungsliste des Entwicklers, verwaltete Einstellungen werden beim Start angewendet und stündlich aktualisiert, und Telemetrie wird an Ihren Collector weitergeleitet. Sitzungen werden vor Ablauf von `ttl_hours` stillschweigend aktualisiert, und eine fehlgeschlagene Aktualisierung nach IdP-Entbereitstellung fordert eine erneute Anmeldung auf.

<h3 id="set-the-gateway-url">
  Gateway-URL festlegen
</h3>

Setzen Sie beide Schlüssel in der Pro-OS-[Datei mit verwalteten Einstellungen](/docs/de/settings#settings-files), die Sie über MDM oder direkt auf der Festplatte bereitstellen, und `/login` öffnet sich direkt auf dem **Cloud gateway**-Bildschirm mit der ausgefüllten URL:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Der Entwickler drückt Enter, um sich zu verbinden. Die Fingerabdruck-Eingabeaufforderung beim ersten Verbinden wird immer noch angezeigt.

Es gibt keine Gateway-Option in der Anmeldungsauswahl für einen Entwickler, um manuell auszuwählen, und `forceLoginGatewayUrl` wird in den eigenen Einstellungsdateien eines Entwicklers ignoriert. `forceLoginMethod` allein, ohne URL, lässt den Entwickler bei einer "Kontaktieren Sie Ihren IT-Administrator"-Nachricht. Beide Schlüssel gehören in die Datei, die Sie auf Maschinen pushen, nicht in den `managed.policies[].cli`-Block des Gateways, der nur bereits verbundene Clients erreicht.

<h3 id="ci-pipelines-and-remote-machines">
  CI-Pipelines und Remote-Maschinen
</h3>

Es gibt keinen Service-Token-Fluss für unbeaufsichtigte Pipelines. Gateway-Anmeldung führt immer den Browser-Gerätefluss aus, daher kann ein CI-Job ohne Entwickler zur Genehmigung der Anmeldung nicht authentifizieren; konfigurieren Sie diese direkt gegen Ihren Anbieter. Sobald sich ein Entwickler angemeldet hat, verwendet jede Claude Code-Invokation auf dieser Maschine die Gateway-Sitzung, einschließlich nicht-interaktiver `claude -p`-Läufe und Sitzungen, die vom Agent SDK gestartet werden, und die [Gateway-Richtlinie gilt für alle davon](/docs/de/claude-apps-gateway-config#managed).

Der Gerätefluss trennt die Polling-CLI vom genehmigenden Browser, daher funktioniert eine Remote-Entwicklungsbox ohne Display immer noch: Der Entwickler führt `/login` über SSH auf der Remote-Maschine aus und öffnet den Überprüfungslink im Browser auf seinem Laptop.

<h3 id="what’s-enforced-on-developers">
  Was auf Entwickler erzwungen wird
</h3>

Diese Garantien gelten für jede angemeldete Gateway-Sitzung.

* **Modellzugriff**: Anfragen für Modelle, die die Richtlinie nicht gewährt, geben 400 zurück, und die `/model`-Auswahl wird auf die `availableModels`-Zulassungsliste der Richtlinie gefiltert. Setzen Sie [`enforceAvailableModels: true`](/docs/de/model-config#default-model-behavior) in der Richtlinie, damit die Standard-Option zu einem Modell in `availableModels` aufgelöst wird, anstatt zu Claude Codes integriertem Standard; ohne sie bleibt Standard wählbar und wird bei der Anfrageverarbeitung abgelehnt, wenn dieses Modell nicht gewährt wird.
* **Telemetrie-Ziel**: Wenn [Telemetrie-Weiterleitung](/docs/de/claude-apps-gateway-config#telemetry) konfiguriert ist, wird der OTLP-Export-Endpunkt auf das Gateway angeheftet, und die vom Gateway gepushte Konfiguration überschreibt lokal gesetzte `OTEL_*`-Variablen.
* **Anmeldedaten**: Das Gateway-Token ist die einzige Anmeldedaten der Sitzung. `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, `apiKeyHelper` und jede frühere claude.ai-Anmeldung werden ignoriert, während angemeldet, daher müssen sich Entwickler nicht zuerst von claude.ai abmelden.
* **Verwaltete Einstellungen**: Gesperrte Schlüssel können nicht lokal überschrieben werden. Die CLI wendet die Richtlinie beim Start und bei jeder stündlichen Abfrage an.
* **Startup**: Angemeldete Sitzungen beenden sich beim Start mit einem Fehler nach etwa 10 Sekunden, wenn das Gateway unerreichbar ist, anstatt ohne ihre Einstellungen zu starten.
* **Entbereitstellung**: Eine Sitzung, deren Benutzer im IdP deaktiviert ist, läuft innerhalb von `ttl_hours` ab, wenn die nächste Aktualisierung fehlschlägt.

<h3 id="what-the-organization-can-see">
  Was die Organisation sehen kann
</h3>

Nutzungstelemetrie trägt die Identität des Entwicklers, Token-Zählungen, Modell und Latenz zum Collector der Organisation. Das Gateway protokolliert oder speichert keinen Prompt- oder Completion-Inhalt. Ob umfangreichere Telemetrie wie Protokolle und Traces erfasst wird, die Befehle und Dateipfade enthalten können, ist die [Pro-Ziel-Wahl](/docs/de/claude-apps-gateway-config#telemetry) der Organisation.

<h2 id="availability-and-limitations">
  Verfügbarkeit und Einschränkungen
</h2>

Die Tabelle behandelt, welche Claude-Code-Funktionen funktionieren, wenn Entwickler sich über das Gateway verbinden, und was der Gateway-Server selbst unterstützt. Wenn etwas nicht unterstützt wird, gibt die Spalte Notizen die Alternative an.

Das Gateway liefert die [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers)-Werte, die die CLI an jeden Upstream sendet, daher verwalten Operatoren keine Beta-Zulassungsliste. Für Amazon Bedrock, das den Header ignoriert, verschiebt das Gateway die Werte in das `anthropic_beta`-Feld des Request-Body; die anderen Upstreams erhalten den Header wie gesendet.

Das Gateway-Sitzungs-Beta-Set der CLI lässt First-Party-Only-Betas und die Extended-Cache-TTL-Beta aus, weshalb diese Zeilen unten als nicht verfügbar angezeigt werden.

| Funktion                                                                                                                        | Status            | Notizen                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------------------------------------------------------------------------------------------------------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Inferenz-Weiterleitung (Amazon Bedrock, Claude Platform auf AWS, Agent Platform von Google Cloud, Microsoft Foundry, Anthropic) | Verfügbar         | Mit Pro-Upstream-Modellübersetzung und Failover. Der Amazon-Bedrock-Upstream verwendet den `bedrock-runtime`-Endpunkt und die AWS-Standard-Anmeldekette; der Amazon-Bedrock-[Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint) ist kein unterstützter Upstream. Der [Claude-Platform-auf-AWS-Upstream](/docs/de/claude-apps-gateway-config#claude-platform-on-aws) erfordert Claude Code v2.1.198 oder später auf dem Gateway-Server. |
| Modellzugriff und verwaltete Einstellungen nach IdP-Gruppe                                                                      | Verfügbar         | Modellzugriff wird auf der Serverseite erzwungen; verwaltete Einstellungen werden pro IdP-Gruppe bereitgestellt und von der CLI auf der [Ebene der verwalteten Einstellungen](/docs/de/settings#settings-precedence) angewendet                                                                                                                                                                                                                  |
| Telemetrie-Verteilung (OTLP/HTTP)                                                                                               | Verfügbar         | Identitäts-gestempelt pro Export; sowohl Protobuf- als auch JSON-Codierungen                                                                                                                                                                                                                                                                                                                                                                |
| OIDC-Identitätsanbieter                                                                                                         | Verfügbar         | Alle OIDC-konformen IdPs; das Gateway führt Standard-OIDC-Erkennung und den Authorization-Code-Flow durch. Siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup) für Pro-IdP-Konfiguration                                                                                                                                                                                                               |
| Pro-Benutzer- und Pro-Gruppen-Ausgabenlimits                                                                                    | Verfügbar         | Siehe [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                |
| Serverseitige Websuche                                                                                                          | Nicht verfügbar   | Die CLI kann nicht sehen, welchen Upstream-Anbieter das Gateway leitet, daher kann sie Websuche-Unterstützung nicht überprüfen und deaktiviert WebSearch auf Gateway-Sitzungen                                                                                                                                                                                                                                                              |
| Standard-Prompt-Caching                                                                                                         | Verfügbar         | `cache_control`-Breakpoints werden an jeden Upstream weitergeleitet                                                                                                                                                                                                                                                                                                                                                                         |
| 1-Stunden-Cache-TTL                                                                                                             | Nicht verfügbar   | Die CLI lässt die Extended-Cache-TTL-Beta auf Gateway-Sitzungen aus, da nicht jeder Upstream, zu dem das Gateway leiten kann, die 1-Stunden-TTL unterstützt, daher verwendet Prompt-Caching über das Gateway die 5-Minuten-TTL; siehe die Beta-Header-Notiz oben                                                                                                                                                                            |
| Auto-Modus                                                                                                                      | Verfügbar         | Folgt den [Regeln für Drittanbieter-Anbieter](/docs/de/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): Nur die Modelle, die auf Drittanbieter-Anbietern berechtigt sind, können es verwenden. Vor v2.1.207 erforderte Auto-Modus auf Gateway-Sitzungen das Setzen von `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, lieferbar über den verwalteten Richtlinien-`env`-Block                                                          |
| First-Party-Only-Optimierungen wie globaler Cache-Umfang und Token-effiziente Tools                                             | Nicht verfügbar   | Die CLI aktiviert sie nicht auf Gateway-Sitzungen; siehe die Beta-Header-Notiz oben                                                                                                                                                                                                                                                                                                                                                         |
| OTLP/gRPC                                                                                                                       | Nicht unterstützt | Nur OTLP über HTTP                                                                                                                                                                                                                                                                                                                                                                                                                          |
| SAML, LDAP und andere nicht-OIDC-Auth                                                                                           | Nicht unterstützt | Nur OIDC. Front mit einer OIDC-Brücke, falls erforderlich                                                                                                                                                                                                                                                                                                                                                                                   |
| Multi-Tenant (mehrere OIDC-Aussteller)                                                                                          | Nicht unterstützt | Ein Aussteller pro Gateway. Führen Sie separate Instanzen aus                                                                                                                                                                                                                                                                                                                                                                               |
| Windows-Server                                                                                                                  | Nicht unterstützt | Bereitstellung auf Linux. macOS nur für lokale Entwicklung                                                                                                                                                                                                                                                                                                                                                                                  |
| Helm-Diagramm                                                                                                                   | Nicht verfügbar   | Das Gateway läuft als Standard-Stateless-Deployment; siehe den [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                        |
| Admin-UI                                                                                                                        | Nicht verfügbar   | Konfiguration ist die YAML-Datei; stellen Sie erneut bereit, um sie zu ändern                                                                                                                                                                                                                                                                                                                                                               |

<h2 id="next-steps">
  Nächste Schritte
</h2>

Der Schnellstart lässt Sie mit einer minimalen Konfiguration unter Docker Compose. Um es weiter zu bringen:

* Erweitern Sie `gateway.yaml` über die minimale Konfiguration hinaus, um beispielsweise Pro-Gruppen-RBAC, Multi-Upstream-Failover oder Telemetrie-Ziele hinzuzufügen. Die [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config) behandelt jede Option.
* Wechseln Sie von Compose zu einer Produktionsbereitstellung auf Kubernetes oder Cloud Run, richten Sie Ihren IdP ordnungsgemäß ein und überprüfen Sie das Sicherheitsmodell. Der [Bereitstellungs- und Operationsleitfaden](/docs/de/claude-apps-gateway-deploy) behandelt IdP-spezifische Einrichtung, Container-Image-Anforderungen, Health-Probes und Fehlerbehebung.
* Legen Sie Ausgabengrenzen für einzelne Entwickler oder Gruppen fest, damit eine unkontrollierte Workload Ihre gesamte Verpflichtung nicht verbrauchen kann. [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) behandelt die Admin-API und wie die Durchsetzung funktioniert.
* Für ein vollständiges durchgearbeitetes Beispiel auf Google Cloud mit Cloud Run, Cloud SQL und Secret Manager siehe [Bereitstellung auf Google Cloud](/docs/de/claude-apps-gateway-on-gcp).
