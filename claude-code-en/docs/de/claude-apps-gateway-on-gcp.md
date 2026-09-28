> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude-Apps-Gateway auf Google Cloud bereitstellen

> Ein praktisches Beispiel für die Ausführung von Claude-Apps-Gateway auf Google Cloud: Cloud Run oder GKE, Cloud SQL für PostgreSQL, Secret Manager und Service-Account-Authentifizierung für Google Clouds Agent Platform.

<Note>
  Diese Seite zeigt eine Möglichkeit, Claude-Apps-Gateway auf Google Cloud auszuführen. Die Konfiguration ist ein funktionierendes Beispiel für kundenverwaltete Infrastruktur und keine unterstützte Produktionsbereitstellung. Verwenden Sie sie, um zu verstehen, wie die einzelnen Komponenten zusammenpassen, bevor Sie sie an Ihre eigene Umgebung anpassen. Für die plattformunabhängigen Anforderungen siehe den [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy).
</Note>

Dieses Beispiel stellt Claude-Apps-Gateway auf Google Cloud mit Google Clouds Agent Platform als Modell-Upstream bereit und verwendet entweder Cloud Run oder GKE für die Berechnung. Google Workspace ist der Beispiel-Identitätsanbieter (IdP), aber jeder OpenID Connect (OIDC) konforme IdP funktioniert. Nur der `oidc`-Block ändert sich. Siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup) für IdP-spezifische Details.

<h2 id="what-you’ll-build">
  Was Sie erstellen werden
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Diagramm von Claude-Apps-Gateway auf Google Cloud: Claude-Code-Clients verbinden sich über HTTPS mit dem Gateway (Cloud Run oder GKE), das sich in einem VPC neben einer privaten IP-Cloud-SQL-Datenbank für Sitzungszustand befindet. Das Gateway meldet Benutzer über OIDC gegen Google Workspace an, liest Konfiguration und Geheimnisse aus Secret Manager, leitet Modellanfragen an Agent Platform weiter und ruft sein Image bei der Bereitstellung aus Artifact Registry ab." width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

Die Bereitstellung besteht aus:

* **Cloud Run**-Service oder **GKE**-Deployment mit dem Gateway-Container
* **Artifact Registry**-Repository für das Gateway-Image
* **Cloud SQL für PostgreSQL**-Instanz, nur private IP, für den [Store](/docs/de/claude-apps-gateway-config#store) des Gateways
* **Secret Manager**-Geheimnisse für `gateway.yaml`, den JWT-Signaturschlüssel, das OIDC-Client-Geheimnis und die Postgres-URL
* **Service Account** mit `roles/aiplatform.user`, direkt auf Cloud Run angehängt oder über Workload Identity auf GKE gebunden
* **HTTPS-Front-End**, das Sie bereitstellen: ein interner Application Load Balancer vor Cloud Run, für den diese Anleitung das Gateway konfiguriert, aber nicht erstellt, oder ein interner **GKE Ingress** der Klasse `gce-internal` auf GKE

<h2 id="prerequisites">
  Voraussetzungen
</h2>

* Ein GCP-Projekt mit aktivierter Abrechnung und Berechtigung zum Erstellen der oben genannten Ressourcen
* Die `gcloud`-CLI, authentifiziert mit `gcloud auth login`, und Docker lokal installiert
* Für den GKE-Pfad: `kubectl` und ein GKE-Cluster auf dem VPC, der in der folgenden Anleitung erstellt wird
* Zugriff auf die Claude-Modelle, die Sie benötigen, in Model Garden, in einer Region, die sie veröffentlicht
* Ein Google Workspace OAuth 2.0-Webanwendungs-Client mit Umleitungs-URI `https://<gateway-host>/oauth/callback`. Siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup)
* Ein TLS-Hostname für das Gateway, typischerweise ein interner DNS-Name, der auf den Load Balancer verweist

Legen Sie das Projekt und die Region einmal fest:

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  Gateway bereitstellen
</h2>

Die folgenden Schritte stellen die vollständige Bereitstellung mit `gcloud`-Befehlen bereit.

<Steps>
  <Step title="APIs aktivieren">
    Aktivieren Sie die Service-APIs, die die Anleitung verwendet:

    ```bash theme={null}
    gcloud services enable \
      aiplatform.googleapis.com \
      artifactregistry.googleapis.com \
      sqladmin.googleapis.com \
      secretmanager.googleapis.com \
      iamcredentials.googleapis.com \
      iam.googleapis.com \
      compute.googleapis.com \
      servicenetworking.googleapis.com \
      run.googleapis.com \
      container.googleapis.com
    ```

    Die APIs, die Sie benötigen, hängen vom Bereitstellungspfad ab:

    * `compute` und `servicenetworking`: erforderlich für den Private-IP-Cloud-SQL-Pfad
    * `run`: nur Cloud Run
    * `container`: nur GKE
  </Step>

  <Step title="Dienstkonto erstellen und IAM-Berechtigung erteilen">
    Das Gateway wird als dediziertes Dienstkonto ausgeführt, das berechtigt ist, Google Cloud's Agent Platform aufzurufen. Es erreicht Cloud SQL über das VPC mit einem Passwortbenutzer, daher ist keine Cloud-SQL-IAM-Rolle erforderlich:

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    Aktivieren Sie dann die Claude-Modelle für das Projekt in Model Garden. Modelle werden in bestimmten Regionen veröffentlicht, überprüfen Sie daher jede Modellkarte.
  </Step>

  <Step title="Image erstellen und in Artifact Registry pushen">
    Erstellen Sie das Image gemäß den [Container-Image-Anforderungen](/docs/de/claude-apps-gateway-deploy#container-image) mit der `linux-x64`-glibc-Binärdatei und pushen Sie es:

    ```bash theme={null}
    gcloud artifacts repositories create claude-gateway \
      --repository-format=docker --location="$REGION"
    gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet

    # Cloud Run erfordert linux/amd64. --provenance=false vermeidet einen buildx OCI
    # Image-Index, den Cloud Run ablehnt.
    docker build --platform=linux/amd64 --provenance=false \
      -t "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" .
    docker push "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>"
    ```
  </Step>

  <Step title="Cloud SQL für PostgreSQL bereitstellen">
    Erstellen Sie die Instanz auf einem VPC über Private Services Access, damit sie keine öffentliche IP hat. Dies erfüllt auch Projekte, bei denen `constraints/sql.restrictPublicIp` erzwungen wird:

    ```bash theme={null}
    VPC=cc-gateway-vpc
    gcloud compute networks create "$VPC" --subnet-mode=custom
    gcloud compute networks subnets create cc-gateway-subnet \
      --network="$VPC" --region="$REGION" --range=10.0.0.0/24

    # Private Services Access: einmalig pro VPC
    gcloud compute addresses create "google-managed-services-${VPC}" \
      --global --purpose=VPC_PEERING --prefix-length=16 --network="$VPC"
    gcloud services vpc-peerings connect \
      --service=servicenetworking.googleapis.com \
      --ranges="google-managed-services-${VPC}" --network="$VPC"

    gcloud sql instances create claude-gateway-db \
      --database-version=POSTGRES_16 --tier=db-g1-small --region="$REGION" \
      --network="projects/${PROJECT_ID}/global/networks/${VPC}" --no-assign-ip
    gcloud sql databases create claude_gateway --instance=claude-gateway-db
    PGPASS="$(openssl rand -hex 24)"
    gcloud sql users create gateway --instance=claude-gateway-db --password="$PGPASS"

    PRIVATE_IP="$(gcloud sql instances describe claude-gateway-db \
      --format='value(ipAddresses[0].ipAddress)')"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${PRIVATE_IP}:5432/claude_gateway?sslmode=require"
    ```

    Die Cloud-Run- oder GKE-Laufzeit muss sich auf diesem VPC befinden oder in diesen VPC weitergeleitet werden.
  </Step>

  <Step title="gateway.yaml schreiben">
    Der `upstreams`-Block verweist auf Google Cloud's Agent Platform mit `auth: {}`, sodass sich das Gateway über Application Default Credentials vom Laufzeit-Dienstkonto authentifiziert. Siehe die [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config) für jedes Feld.

    Zwei `listen`-Felder beschreiben, was das Gateway frontet:

    * `public_url`: der externe `https://`-Ursprung, erforderlich für jede Bindung, die nicht Loopback ist. Siehe die [`listen`-Referenz](/docs/de/claude-apps-gateway-config#listen). Das Gateway erstellt den IdP `redirect_uri` und sein Discovery-Dokument nur aus diesem Wert, niemals aus `X-Forwarded-*`-Headern.
    * `trusted_proxies`: die Quellbereiche des Front-End. Das Gateway berücksichtigt `X-Forwarded-For` nur, wenn der TCP-Peer in dieser Liste ist, durchläuft dann die Kette vorbei an vertrauenswürdigen Hops, sodass Anmelderate-Limits pro IP und Audit-Events Entwickler-IPs statt der IP des Load Balancers aufzeichnen.

    Setzen Sie `trusted_proxies` so, dass es Ihrem Front-End entspricht. Ein externes GKE-Ingress der Klasse `gce` ist nicht aufgelistet: Es stellt eine öffentliche Forwarding-Rule-Adresse bereit, die die `/login`-[Private-Netzwerk-Prüfung](/docs/de/claude-apps-gateway#prerequisites) ablehnt.

    | Front-End                                        | `trusted_proxies`                                     |
    | ------------------------------------------------ | ----------------------------------------------------- |
    | Cloud Run direkt erreicht, kein Load Balancer    | `[169.254.0.0/16]`                                    |
    | Interner Application Load Balancer vor Cloud Run | `169.254.0.0/16` plus CIDR Ihres Proxy-Only-Subnetzes |
    | GKE interner Ingress, Klasse `gce-internal`      | CIDR Ihres Proxy-Only-Subnetzes                       |

    Das folgende Beispiel verwendet die Werte des internen Load Balancers vor Cloud Run.

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [169.254.0.0/16, <your-proxy-only-subnet-cidr>]

    oidc:
      issuer: https://accounts.google.com
      client_id: <your-oauth-client-id>
      client_secret: ${OIDC_CLIENT_SECRET}           # GKE: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # Google ignoriert offline_access; diese ergeben Refresh-Token:
      scopes: [openid, profile, email]
      extra_auth_params: { access_type: offline, prompt: consent }

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # GKE: ${file:/secrets/jwt-secret}

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # GKE: ${file:/secrets/postgres-url}

    upstreams:
      - provider: vertex
        region: <your-region>                        # must match $REGION
        project_id: <your-project>
        auth: {} # ADC via the runtime service account
    ```

    <Note>
      Google-ID-Token enthalten keinen `groups`-Anspruch. Um gruppenbasierte Richtlinien in [`managed.policies`](/docs/de/claude-apps-gateway-config#managed) mit Google Workspace als IdP zu verwenden, konfigurieren Sie [`oidc.google_groups`](/docs/de/claude-apps-gateway-config#oidc), das die Gruppen jedes Benutzers über die Admin SDK Directory API mit einem Dienstkonto mit Domain-Wide-Delegation nachschlägt. Ohne dies sollten Sie stattdessen auf `email_domain` abgleichen.
    </Note>
  </Step>

  <Step title="Geheimnisse in Secret Manager speichern">
    Erstellen Sie vier Geheimnisse und erteilen Sie `roles/secretmanager.secretAccessor` dem `claude-gateway`-Dienstkonto:

    | Geheimnis                    | Quelle                                                     |
    | ---------------------------- | ---------------------------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`                                  |
    | `gateway-oidc-client-secret` | Google Cloud Console → OAuth-Client                        |
    | `gateway-postgres-url`       | `$GATEWAY_POSTGRES_URL` aus dem Cloud-SQL-Schritt          |
    | `gateway-config`             | die vollständige `gateway.yaml` aus dem vorherigen Schritt |

    Wie die Geheimnisse den Container erreichen, unterscheidet sich je nach Pfad:

    * Auf GKE werden sie als Dateien über den Secret Manager CSI-Treiber bereitgestellt, und `gateway.yaml` verweist auf `${file:/secrets/...}`.
    * Auf Cloud Run, das mehrere Geheimnisse nicht in ein Verzeichnis einbinden kann, wird `gateway.yaml` als Datei bereitgestellt und die anderen drei werden als Umgebungsvariablen eingespritzt, sodass `gateway.yaml` stattdessen auf `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` und `${GATEWAY_POSTGRES_URL}` verweist.
  </Step>

  <Step title="Bereitstellen">
    <Tabs>
      <Tab title="Cloud Run">
        Der folgende Befehl stellt für die Produktion hinter einem internen Load Balancer bereit.

        ```bash theme={null}
        gcloud run deploy claude-gateway \
          --image="${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" \
          --region="$REGION" \
          --service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --min-instances=1 \
          --max-instances=8 \
          --timeout=3600 \
          --ingress=internal \
          --network="$VPC" --subnet=cc-gateway-subnet --vpc-egress=private-ranges-only \
          --set-secrets=/etc/claude/gateway.yaml=gateway-config:latest,GATEWAY_JWT_SECRET=gateway-jwt-secret:latest,OIDC_CLIENT_SECRET=gateway-oidc-client-secret:latest,GATEWAY_POSTGRES_URL=gateway-postgres-url:latest \
          --no-invoker-iam-check
        ```

        Direkter VPC-Egress über `--network`, `--subnet` und `--vpc-egress=private-ranges-only` ermöglicht dem Service, die private Cloud-SQL-IP direkt zu erreichen. Jede Instanz hält bis zu [`store.max_connections`](/docs/de/claude-apps-gateway-config#store) Postgres-Verbindungen, standardmäßig fünf, daher halten Sie maximale Instanzen × `store.max_connections` unter dem Verbindungslimit Ihres Cloud-SQL-Tiers. Die [Referenz-Assets](#terraform-reference) begrenzen Instanzen auf 8 für das `db-g1-small`-Tier aus diesem Grund. Öffentlicher Egress zu Google Cloud's Agent Platform-Endpunkten und `accounts.google.com` geht direkt ins Internet statt durch das VPC, daher ist kein Cloud NAT erforderlich.

        Die Invoker-IAM-Prüfung muss offen oder deaktiviert sein. Das Gateway führt sein eigenes OIDC aus und seine Clients tragen kein GCP-Token, daher muss Cloud Run's Invoker-Prüfung unauthentifizierte Anfragen zulassen. Die OIDC-Anmeldung des Gateways authentifiziert die Anfrage, sobald sie den Container erreicht, wobei `allowed_email_domains` bestimmt, welche Domänen sich anmelden dürfen.

        Zwei Flags lassen unauthentifizierte Anfragen zu:

        * `--no-invoker-iam-check`: deaktiviert die Prüfung ohne `allUsers`-Bindung zum Verwalten und funktioniert unter Domain Restricted Sharing
        * `--allow-unauthenticated`: erteilt `allUsers` die `run.invoker`-Rolle. Verwenden Sie dies, wenn Ihre Organisation `--no-invoker-iam-check` nicht zulässt

        Ingress-Einschränkung über `--ingress` ist eine separate, unabhängige Schicht von der Invoker-Prüfung. Halten Sie sie gesetzt, um den Service auf Ihr Unternehmensnetzwerk zu beschränken.

        Standardmäßig wird die Cloud-Run-`*.run.app`-URL zu einer öffentlichen Adresse aufgelöst, die die `/login`-[Private-Netzwerk-Prüfung](/docs/de/claude-apps-gateway#prerequisites) ablehnt. Zwei Topologien geben Entwicklern einen privat auflösbaren Hostnamen, und Cloud Run stellt keinen davon für Sie bereit:

        * **Interner Application Load Balancer**, die Topologie, die die `gateway.yaml` dieser Seite annimmt: Stellen Sie einen internen Application Load Balancer vor dem Service mit einem internen DNS-Namen und Zertifikat bereit, und setzen Sie `listen.public_url` auf diesen Hostnamen. Die `internal`-Ingress-Einstellung lässt bereits Traffic von internen Application Load Balancern zu. `internal-and-cloud-load-balancing` lässt zusätzlich externe Application Load Balancer zu, deren öffentliche Adressen die `/login`-Private-Netzwerk-Prüfung ablehnt, daher benötigt keine Topologie auf dieser Seite dies.
        * **Nur-interner Ingress ohne Load Balancer**: Halten Sie den Deploy-Befehl wie er ist und lassen Sie `listen.public_url` als die `*.run.app`-URL, die Standardeinstellung in den [Referenz-Assets](#terraform-reference) unten. Damit `*.run.app` privat aufgelöst wird, muss Ihr Netzwerk-Team bereits einen Private Service Connect-Endpunkt für Google APIs betreiben, eine Cloud-DNS-Private-Zone, die `*.run.app` darauf auflöst, und On-Premises-Routing zu diesem Endpunkt.

        Google's [Private-Networking-Leitfaden für Cloud Run](https://cloud.google.com/run/docs/securing/private-networking) behandelt die Infrastruktur, die beide Optionen benötigen. Überprüfen Sie die Anmeldung, sobald das Gateway auf einem privaten Hostnamen bereitgestellt wird. Bestätigen Sie bis dahin, dass der Container aus seinen Logs in Cloud Run gestartet wurde.

        Aktualisieren Sie die autorisierten Redirect-URIs des OAuth-Clients auf `<public_url>/oauth/callback` vor der ersten Anmeldung. Stellen Sie erneut bereit, nachdem Sie `public_url` geändert haben, da das Gateway seinen öffentlichen Ursprung nur aus dieser Einstellung erstellt und `X-Forwarded-Host` und `X-Forwarded-Proto` ignoriert. `X-Forwarded-For` wird nur für Client-IPs berücksichtigt, wenn `listen.trusted_proxies` gesetzt ist.
      </Tab>

      <Tab title="GKE">
        Der Cluster muss sich auf dem `$VPC` befinden, das im Cloud-SQL-Schritt erstellt wurde, damit Pods die private IP der Datenbank erreichen können. VPC-Peering allein funktioniert nicht, da die private Cloud-SQL-IP selbst ein Peered-Netzwerk ist und Peering nicht transitiv ist. Um einen neuen Cluster auf diesem VPC zu erstellen, übergeben Sie `--network="$VPC" --subnetwork=cc-gateway-subnet` an `gcloud container clusters create`.

        Aktivieren Sie Workload Identity auf dem Cluster und seinen Node Pools, binden Sie dann das Google-Dienstkonto an das Kubernetes-Dienstkonto, damit Pods seine Anmeldedaten erben:

        ```bash theme={null}
        gcloud container clusters update <cluster> --region="$REGION" \
          --workload-pool="${PROJECT_ID}.svc.id.goog"
        # Auf einem Standard-Cluster benötigen auch vorhandene Node Pools GKE_METADATA.
        # Autopilot aktiviert dies standardmäßig.
        gcloud container node-pools update <pool> --cluster=<cluster> \
          --region="$REGION" --workload-metadata=GKE_METADATA

        kubectl create namespace claude-gateway
        kubectl create serviceaccount gateway -n claude-gateway

        gcloud iam service-accounts add-iam-policy-binding \
          "claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --role roles/iam.workloadIdentityUser \
          --member "serviceAccount:${PROJECT_ID}.svc.id.goog[claude-gateway/gateway]"

        kubectl annotate serviceaccount gateway -n claude-gateway \
          iam.gke.io/gcp-service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"
        ```

        Stellen Sie das Gateway als Standard-Deployment plus Service und einen internen Ingress, Klasse `gce-internal`, wie in [Kubernetes-Bereitstellung](/docs/de/claude-apps-gateway-deploy#kubernetes) beschrieben, mit folgenden Einstellungen bereit:

        * `serviceAccountName: gateway`
        * der Secret Manager CSI-Treiber, der Geheimnisse bei `/secrets` bereitstellt
        * die Readiness-Probe auf `GET /readyz` ausgerichtet

        Fügen Sie eine BackendConfig mit erhöhtem `timeoutSec` zum Gateway-Service an: Der Load-Balancer-Backend-Service hinter GKE-Ingress hat standardmäßig ein 30-Sekunden-Timeout, das lange Streaming-Antworten abschneidet.

        Wenden Sie keine Egress-NetworkPolicy an, die `169.254.169.254` auf einem Workload-Identity-Cluster blockiert. Der Pod muss den Metadaten-Server für Anmeldedaten erreichen. Der integrierte [SSRF-Guard](/docs/de/claude-apps-gateway-deploy#threat-model-summary) des Gateways ist die Verteidigung dort.

        Das Gateway protokolliert eine Boot-Warnung, dass der Metadaten-Endpunkt erreichbar ist und schlägt vor, eine Egress-NetworkPolicy anzuwenden. Unter Workload Identity ist diese Warnung erwartet, da der Pod den Endpunkt benötigt.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Gateway-URL auf Entwicklermaschinen pushen">
    Das Gateway wird jetzt ausgeführt, aber Entwickler können es von `/login` aus nicht erreichen, bis die Gateway-URL auf ihren Maschinen vorhanden ist. Stellen Sie das vollständige [verwaltete Einstellungs-Snippet](/docs/de/claude-apps-gateway#set-the-gateway-url) mit `forceLoginMethod`, `forceLoginGatewayUrl` und dem `parentSettingsBehavior: "merge"`-Opt-in über MDM auf jedem Gerät bereit. Es gibt keine Gateway-Option in der Anmeldungs-Auswahl, die ein Entwickler manuell auswählen kann.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Terraform-Referenz
</h2>

Die [Referenz-Bereitstellungs-Assets](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp) automatisieren den Cloud Run-Pfad auf dieser Seite; die Konfigurations- und Image-Assets gelten für beide Pfade:

* `setup.sh`: ein idempotenter `gcloud`-Provisioner, der den vollständigen Cloud Run-Pfad durchläuft, vom Aktivieren von APIs bis zur ersten Bereitstellung
* `terraform/`: dieselbe Bereitstellung als Infrastructure-as-Code für eine Greenfield-Bereitstellung: ein gezieltes Apply zum Erstellen des Artifact Registry-Repositorys, dann zum Erstellen und Pushen des Images, dann ein vollständiges Apply
* `gateway.yaml.example` und ein `Dockerfile` für das Distroless-Runtime-Image

Die Artefakte standardisieren Cloud Run Ingress auf `internal`, was dem Bereitstellungsbefehl auf dieser Seite entspricht; diese Einstellung funktioniert mit oder ohne einen internen Application Load Balancer vor dem Service, und die Artefakte erstellen den Load Balancer auch nicht. Die Artefakte standardisieren auch die Invoker-Schicht auf eine `allUsers` `run.invoker`-Gewährung statt `--no-invoker-iam-check`, das Gegenteil dieser Seite; beide funktionieren, und die Wahl hängt von den Richtlinieneinschränkungen Ihrer Organisation ab.

Die Assets werden als funktionierende Beispiele bereitgestellt, nicht als unterstütztes Produktionsartefakt; überprüfen und passen Sie sie an Ihre Umgebung an.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Für Gateway-Boot- und Anmeldungsfehler siehe die plattformunabhängige [Fehlerbehebungstabelle](/docs/de/claude-apps-gateway-deploy#troubleshooting). Die folgenden Einträge sind spezifisch für Google Cloud.

| Symptom                                                                                       | Ursache                                                                                                                                  | Behebung                                                                                                                                                                                                                                                  |
| --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cloud Run gibt `403 Forbidden` zurück, bevor der Container erreicht wird                      | Die Invoker IAM-Prüfung ist noch aktiviert                                                                                               | Stellen Sie mit `--no-invoker-iam-check` bereit oder erteilen Sie `allUsers` die `run.invoker`-Rolle mit `--allow-unauthenticated`                                                                                                                        |
| `--no-invoker-iam-check` wird mit `invoker_iam_disabled is not currently available` abgelehnt | Blockiert durch `constraints/run.managed.requireInvokerIam`                                                                              | Verwenden Sie `--allow-unauthenticated`. Wenn Domain Restricted Sharing über `constraints/iam.allowedPolicyMemberDomains` dies auch blockiert, verwenden Sie den GKE-Pfad, der das Gateway auf der Netzwerkebene ohne `allUsers`-Bindung verfügbar macht. |
| `Container manifest type … must support amd64/linux` bei Bereitstellung                       | Image wurde auf einem Nicht-amd64-Host erstellt oder buildx hat einen OCI-Image-Index ausgegeben                                         | Erstellen Sie mit `--platform=linux/amd64 --provenance=false`                                                                                                                                                                                             |
| Gateway-Boot beendet sich mit Postgres-Verbindungs-Timeout-Fehler auf Cloud Run               | Service ist nicht an das VPC angehängt oder Cloud SQL hat keine private IP auf diesem VPC                                                | Stellen Sie mit `--network` und `--subnet` für Direct VPC Egress bereit und erstellen Sie die Cloud SQL-Instanz mit `--no-assign-ip` und `--network`, das auf dasselbe VPC verweist                                                                       |
| Agent Platform-Anfragen geben `403 PERMISSION_DENIED` zurück                                  | Laufzeit verwendet nicht den `claude-gateway`-Service-Account oder das Modell ist nicht in Model Garden für das Projekt aktiviert        | Legen Sie `--service-account` auf Cloud Run fest oder binden Sie Workload Identity auf GKE, und aktivieren Sie jedes Claude-Modell in Model Garden für die Zielregion                                                                                     |
| Streaming-Antworten werden nach einer festen Dauer abgeschnitten                              | Front-End-Anfrage-Timeout: Der Load Balancer Backend-Service hinter GKE Ingress hat standardmäßig 30 Sekunden und Cloud Run 300 Sekunden | Hängen Sie eine BackendConfig mit erhöhtem `timeoutSec` auf GKE an oder stellen Sie mit `--timeout=3600` auf Cloud Run bereit                                                                                                                             |

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config): jede `gateway.yaml`-Option, einschließlich `managed.policies` und `telemetry`
* [Bereitstellung und Betrieb](/docs/de/claude-apps-gateway-deploy): IdP-Setup, Integritätsprüfungen, JWT-Geheimnis-Rotation, Upgrades und das Sicherheitsmodell
* [Claude-Apps-Gateway-Übersicht](/docs/de/claude-apps-gateway): Schnellstart und Verbindung von Entwicklern
