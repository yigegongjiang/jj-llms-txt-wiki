> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Distribuire il gateway delle app Claude su Google Cloud

> Un esempio pratico di esecuzione del gateway delle app Claude su Google Cloud: Cloud Run o GKE, Cloud SQL per PostgreSQL, Secret Manager e autenticazione tramite account di servizio verso Agent Platform di Google Cloud.

<Note>
  Questa pagina illustra un modo per eseguire il gateway delle app Claude su Google Cloud. La configurazione è un esempio funzionante per l'infrastruttura gestita dal cliente piuttosto che una distribuzione di produzione supportata; utilizzatela per vedere come i componenti si incastrano insieme prima di adattarla al vostro ambiente. Per i requisiti indipendenti dalla piattaforma, consultate la [guida alla distribuzione](/docs/it/claude-apps-gateway-deploy).
</Note>

Questo esempio esegue il provisioning del gateway delle app Claude su Google Cloud con Agent Platform di Google Cloud come upstream del modello, utilizzando Cloud Run o GKE per il calcolo. Google Workspace è l'esempio di provider di identità (IdP), ma funziona qualsiasi provider conforme a OpenID Connect (OIDC); solo il blocco `oidc` cambia. Consultate [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup) per i dettagli specifici per ogni IdP.

<h2 id="what-you’ll-build">
  Cosa costruirete
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Diagramma del gateway delle app Claude su Google Cloud: i client Claude Code si connettono tramite HTTPS al gateway (Cloud Run o GKE), che viene eseguito all'interno di un VPC insieme a un database Cloud SQL con IP privato per lo stato della sessione. Il gateway accede agli utenti tramite OIDC rispetto a Google Workspace, legge la configurazione e i segreti da Secret Manager, inoltra le richieste del modello ad Agent Platform e estrae la sua immagine da Artifact Registry al momento della distribuzione." width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

La configurazione di riferimento esegue il provisioning di:

* Servizio **Cloud Run** o Deployment **GKE** che esegue il contenitore del gateway
* Repository **Artifact Registry** per l'immagine del gateway
* Istanza **Cloud SQL per PostgreSQL**, solo IP privato, per lo [store](/docs/it/claude-apps-gateway-config#store) del gateway
* Segreti **Secret Manager** per `gateway.yaml`, la chiave di firma JWT, il segreto client OIDC e l'URL Postgres
* **Account di servizio** con `roles/aiplatform.user`, allegato direttamente su Cloud Run o associato tramite Workload Identity su GKE
* **Internal Application Load Balancer** su Cloud Run, o un **GKE Ingress** interno di classe `gce-internal` su GKE, per HTTPS

<h2 id="prerequisites">
  Prerequisiti
</h2>

* Un progetto GCP con fatturazione abilitata e autorizzazione per creare le risorse di cui sopra
* La CLI `gcloud`, autenticata con `gcloud auth login`, e Docker installato localmente
* Per il percorso GKE: `kubectl` e un cluster GKE sul VPC creato nella procedura dettagliata di seguito
* Accesso ai modelli Claude di cui avete bisogno in Model Garden, in una regione che li pubblica
* Un client applicazione web OAuth 2.0 di Google Workspace con URI di reindirizzamento `https://<gateway-host>/oauth/callback`; consultate [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup)
* Un nome host TLS per il gateway, in genere un nome DNS interno che punta al load balancer

Impostate il progetto e la regione una volta:

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  Distribuire il gateway
</h2>

I passaggi seguenti eseguono il provisioning della distribuzione completa con comandi `gcloud`.

<Steps>
  <Step title="Abilitare le API">
    Abilitate le API di servizio utilizzate dalla procedura dettagliata:

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

    Le API di cui avete bisogno dipendono dal percorso di distribuzione:

    * `compute` e `servicenetworking`: necessarie per il percorso Cloud SQL con IP privato
    * `run`: solo Cloud Run
    * `container`: solo GKE
  </Step>

  <Step title="Creare l'account di servizio e concedere IAM">
    Il gateway viene eseguito come account di servizio dedicato con autorizzazione per chiamare Agent Platform. Raggiunge Cloud SQL sul VPC con un utente con password, quindi non è richiesto alcun ruolo IAM di Cloud SQL:

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    Quindi abilitate i modelli Claude per il progetto in Model Garden; i modelli vengono pubblicati in regioni specifiche, quindi controllate ogni scheda del modello.
  </Step>

  <Step title="Compilare e inviare l'immagine ad Artifact Registry">
    Compilate l'immagine secondo i [requisiti dell'immagine del contenitore](/docs/it/claude-apps-gateway-deploy#container-image), utilizzando il binario glibc `linux-x64`, e inviatela:

    ```bash theme={null}
    gcloud artifacts repositories create claude-gateway \
      --repository-format=docker --location="$REGION"
    gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet

    # Cloud Run requires linux/amd64. --provenance=false avoids a buildx OCI
    # image index that Cloud Run rejects.
    docker build --platform=linux/amd64 --provenance=false \
      -t "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" .
    docker push "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>"
    ```
  </Step>

  <Step title="Eseguire il provisioning di Cloud SQL per PostgreSQL">
    Create l'istanza su un VPC tramite Private Services Access in modo che non abbia IP pubblico; questo soddisfa anche i progetti in cui `constraints/sql.restrictPublicIp` è applicato:

    ```bash theme={null}
    VPC=cc-gateway-vpc
    gcloud compute networks create "$VPC" --subnet-mode=custom
    gcloud compute networks subnets create cc-gateway-subnet \
      --network="$VPC" --region="$REGION" --range=10.0.0.0/24

    # Private Services Access: one-time per VPC
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

    Il runtime Cloud Run o GKE deve essere su, o instradato in, questo VPC.
  </Step>

  <Step title="Scrivere gateway.yaml">
    Il blocco `upstreams` punta ad Agent Platform con `auth: {}`, quindi il gateway si autentica tramite Application Default Credentials dall'account di servizio del runtime. Consultate il [riferimento di configurazione](/docs/it/claude-apps-gateway-config) per ogni campo.

    Due campi `listen` dipendono da cosa si trova davanti al gateway:

    * `public_url`: richiesto dietro Cloud Run o un GKE Ingress. Il gateway costruisce l'`redirect_uri` dell'IdP e il suo documento di discovery solo da questo valore, mai da intestazioni `X-Forwarded-*`.
    * `trusted_proxies`: gli intervalli di origine del front end. Il gateway onora `X-Forwarded-For` solo quando il peer TCP è in questo elenco, quindi percorre la catena oltre i hop affidabili, in modo che i limiti di velocità di accesso per IP e gli eventi di audit registrino gli IP degli sviluppatori invece di quello del load balancer.

    Impostate `trusted_proxies` in modo che corrisponda al vostro front end. Un GKE Ingress esterno di classe `gce` non è elencato: esegue il provisioning di un indirizzo di regola di inoltro pubblico, che il controllo [rete privata](/docs/it/claude-apps-gateway#prerequisites) di `/login` rifiuta.

    | Front end                                              | `trusted_proxies`                                    |
    | ------------------------------------------------------ | ---------------------------------------------------- |
    | Cloud Run raggiunto direttamente, nessun load balancer | `[169.254.0.0/16]`                                   |
    | Internal Application Load Balancer davanti a Cloud Run | `169.254.0.0/16` più il CIDR della subnet solo proxy |
    | GKE internal Ingress, classe `gce-internal`            | Il CIDR della subnet solo proxy                      |

    L'esempio seguente utilizza i valori del load balancer interno davanti a Cloud Run.

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
      # Google ignores offline_access; these yield refresh tokens:
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
      I token id di Google non contengono alcuna rivendicazione `groups`. Per utilizzare le politiche basate su gruppi in [`managed.policies`](/docs/it/claude-apps-gateway-config#managed) con Google Workspace come IdP, configurate [`oidc.google_groups`](/docs/it/claude-apps-gateway-config#oidc), che cerca i gruppi di ogni utente tramite l'API Admin SDK Directory utilizzando un account di servizio con delega a livello di dominio. Senza di essa, abbinate invece su `email_domain`.
    </Note>
  </Step>

  <Step title="Archiviare i segreti in Secret Manager">
    Create quattro segreti e concedete `roles/secretmanager.secretAccessor` all'account di servizio `claude-gateway`:

    | Segreto                      | Fonte                                               |
    | ---------------------------- | --------------------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`                           |
    | `gateway-oidc-client-secret` | Google Cloud Console → Client OAuth                 |
    | `gateway-postgres-url`       | `$GATEWAY_POSTGRES_URL` dal passaggio Cloud SQL     |
    | `gateway-config`             | Il completo `gateway.yaml` dal passaggio precedente |

    Il modo in cui i segreti raggiungono il contenitore differisce per traccia:

    * Su GKE vengono montati come file tramite il driver CSI di Secret Manager e `gateway.yaml` fa riferimento a `${file:/secrets/...}`.
    * Su Cloud Run, che non può montare più segreti in una directory, `gateway.yaml` viene montato come file e gli altri tre si iniettano come variabili di ambiente, quindi `gateway.yaml` fa riferimento a `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` e `${GATEWAY_POSTGRES_URL}` invece.
  </Step>

  <Step title="Distribuire">
    <Tabs>
      <Tab title="Cloud Run">
        Il comando seguente distribuisce per la produzione dietro un load balancer interno.

        ```bash theme={null}
        gcloud run deploy claude-gateway \
          --image="${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" \
          --region="$REGION" \
          --service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --min-instances=1 \
          --timeout=3600 \
          --ingress=internal-and-cloud-load-balancing \
          --network="$VPC" --subnet=cc-gateway-subnet --vpc-egress=private-ranges-only \
          --set-secrets=/etc/claude/gateway.yaml=gateway-config:latest,GATEWAY_JWT_SECRET=gateway-jwt-secret:latest,OIDC_CLIENT_SECRET=gateway-oidc-client-secret:latest,GATEWAY_POSTGRES_URL=gateway-postgres-url:latest \
          --no-invoker-iam-check
        ```

        L'uscita VPC diretta, tramite `--network`, `--subnet` e `--vpc-egress=private-ranges-only`, consente al servizio di raggiungere direttamente l'IP privato di Cloud SQL. L'uscita pubblica verso gli endpoint di Agent Platform e `accounts.google.com` va direttamente a Internet piuttosto che attraverso il VPC, quindi non è necessario Cloud NAT.

        Il controllo IAM dell'invoker deve essere aperto o disabilitato. Il gateway esegue il proprio OIDC e i suoi client non portano alcun token GCP, quindi il controllo dell'invoker di Cloud Run deve ammettere richieste non autenticate. Il gateway autentica la richiesta una volta che raggiunge il contenitore con OIDC, con `allowed_email_domains` che controlla quali domini possono accedere.

        Due flag ammettono richieste non autenticate:

        * `--no-invoker-iam-check`: disabilita il controllo senza alcun binding `allUsers` da gestire e funziona con Domain Restricted Sharing
        * `--allow-unauthenticated`: concede al ruolo `run.invoker` `allUsers`; utilizzatelo se la vostra organizzazione non consente `--no-invoker-iam-check`

        La restrizione dell'ingresso tramite `--ingress` è un livello separato e indipendente dal controllo dell'invoker; mantenetelo impostato per limitare il servizio alla vostra rete aziendale.

        Per impostazione predefinita, l'URL `*.run.app` di Cloud Run si risolve in un indirizzo pubblico, che il controllo [rete privata](/docs/it/claude-apps-gateway#prerequisites) di `/login` rifiuta. Due topologie forniscono agli sviluppatori un nome host risolvibile privatamente e Cloud Run non ne esegue il provisioning per voi:

        * **Internal Application Load Balancer**, la topologia che il comando di distribuzione sopra presuppone: distribuire con `--ingress=internal-and-cloud-load-balancing`, eseguire il provisioning di un Internal Application Load Balancer davanti al servizio con un nome DNS interno e un certificato, e impostare `listen.public_url` su quel nome host.
        * **Ingresso solo interno senza load balancer**: distribuire con `--ingress=internal` e lasciare `listen.public_url` come URL `*.run.app`, l'impostazione predefinita negli [asset di riferimento](#terraform-reference) di seguito. Affinché `*.run.app` si risolva privatamente, il vostro team di rete deve già operare un endpoint Private Service Connect per le API di Google, una zona privata Cloud DNS che risolve `*.run.app` ad esso e il routing on-premises a quell'endpoint.

        La [guida alla rete privata di Google per Cloud Run](https://cloud.google.com/run/docs/securing/private-networking) copre l'infrastruttura di cui entrambe le opzioni hanno bisogno. Verificate l'accesso una volta che il gateway è in servizio su un nome host privato; fino ad allora, confermate che il contenitore si è avviato dai suoi log in Cloud Run.

        Aggiornate l'URI di reindirizzamento autorizzato del client OAuth a `<public_url>/oauth/callback` prima del primo accesso. Ridistribuite dopo aver modificato `public_url`, perché il gateway costruisce la sua origine pubblica solo da quell'impostazione e ignora `X-Forwarded-Host` e `X-Forwarded-Proto`. `X-Forwarded-For` viene onorato per gli IP client solo quando `listen.trusted_proxies` è impostato.
      </Tab>

      <Tab title="GKE">
        Il cluster deve essere sul `$VPC` creato nel passaggio Cloud SQL in modo che i pod possano raggiungere l'IP privato del database; il peering VPC da solo non funziona, perché l'IP privato di Cloud SQL è esso stesso una rete peered e il peering non è transitivo. Per creare un nuovo cluster su quel VPC, passate `--network="$VPC" --subnetwork=cc-gateway-subnet` a `gcloud container clusters create`.

        Abilitate Workload Identity sul cluster e sui suoi pool di nodi, quindi associate l'account di servizio Google all'account di servizio Kubernetes in modo che i pod ereditino le sue credenziali:

        ```bash theme={null}
        gcloud container clusters update <cluster> --region="$REGION" \
          --workload-pool="${PROJECT_ID}.svc.id.goog"
        # On a Standard cluster, existing node pools also need GKE_METADATA;
        # Autopilot enables this by default.
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

        Distribuite il gateway come Deployment standard più un Service e un Ingress interno, classe `gce-internal`, come descritto in [Distribuzione Kubernetes](/docs/it/claude-apps-gateway-deploy#kubernetes), con:

        * `serviceAccountName: gateway`
        * il driver CSI di Secret Manager che monta i segreti in `/secrets`
        * il probe di prontezza puntato a `GET /readyz`

        Allegate un BackendConfig con un `timeoutSec` elevato al Service del gateway: il servizio backend del load balancer dietro GKE Ingress ha un timeout predefinito di 30 secondi, che interrompe le risposte di streaming lunghe.

        Non applicate una NetworkPolicy di uscita che blocca `169.254.169.254` su un cluster Workload Identity; il pod deve raggiungere il server di metadati per le credenziali. La [guardia SSRF](/docs/it/claude-apps-gateway-deploy#threat-model-summary) integrata del gateway è la difesa lì.

        Il gateway registra un avviso di avvio che l'endpoint dei metadati è raggiungibile e suggerisce di applicare una NetworkPolicy di uscita. Con Workload Identity quell'avviso è previsto, perché il pod ha bisogno dell'endpoint.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Inviare l'URL del gateway alle macchine degli sviluppatori">
    Il gateway è ora in esecuzione, ma gli sviluppatori non possono raggiungerlo da `/login` fino a quando l'URL del gateway non è sulle loro macchine. Impostate `forceLoginMethod` e `forceLoginGatewayUrl` nel [file di impostazioni gestite](/docs/it/claude-apps-gateway#set-the-gateway-url) che distribuite a ogni dispositivo tramite MDM. Non c'è alcuna opzione di gateway nel selettore di accesso per uno sviluppatore da selezionare manualmente.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Riferimento Terraform
</h2>

Gli [asset di distribuzione di riferimento](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp) automatizzano il percorso Cloud Run su questa pagina; gli asset di configurazione e immagine si applicano a entrambi i percorsi:

* `setup.sh`: un provisioner `gcloud` idempotente che percorre il percorso completo di Cloud Run, dall'abilitazione delle API alla prima distribuzione
* `terraform/`: la stessa distribuzione come infrastruttura come codice, per una distribuzione greenfield: un apply mirato per creare il repository Artifact Registry, quindi compilare e inviare l'immagine, quindi un apply completo
* `gateway.yaml.example` e un `Dockerfile` per l'immagine di runtime distroless

Gli artefatti impostano per impostazione predefinita l'ingresso di Cloud Run su `internal`, quindi non è richiesto alcun load balancer. Per corrispondere alla distribuzione di produzione dietro un ALB di questa pagina, eseguite `setup.sh` con `INGRESS=internal-and-cloud-load-balancing`, o impostate la variabile Terraform `ingress` su `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER`. Gli artefatti impostano anche per impostazione predefinita il livello dell'invoker su una concessione `run.invoker` `allUsers` piuttosto che `--no-invoker-iam-check`, l'inverso della procedura dettagliata di questa pagina; entrambi funzionano e la scelta dipende dai vincoli di policy della vostra organizzazione.

Gli asset sono forniti come esempi funzionanti, non come artefatto di produzione supportato; esaminate e adattate al vostro ambiente.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Per gli errori di avvio del gateway e di accesso, consultate la tabella [risoluzione dei problemi](/docs/it/claude-apps-gateway-deploy#troubleshooting) indipendente dalla piattaforma. Le voci seguenti sono specifiche per Google Cloud.

| Sintomo                                                                                   | Causa                                                                                                                                                              | Soluzione                                                                                                                                                                                                                                        |
| ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Cloud Run restituisce `403 Forbidden` prima di raggiungere il contenitore                 | Il controllo IAM dell'invoker è ancora abilitato                                                                                                                   | Distribuite con `--no-invoker-iam-check`, o concedete a `allUsers` il ruolo `run.invoker` con `--allow-unauthenticated`                                                                                                                          |
| `--no-invoker-iam-check` rifiutato con `invoker_iam_disabled is not currently available`  | Bloccato da `constraints/run.managed.requireInvokerIam`                                                                                                            | Utilizzate `--allow-unauthenticated`. Se Domain Restricted Sharing tramite `constraints/iam.allowedPolicyMemberDomains` blocca anche quello, utilizzate il percorso GKE, che espone il gateway a livello di rete senza alcun binding `allUsers`. |
| `Container manifest type … must support amd64/linux` al momento della distribuzione       | L'immagine è stata compilata su un host non amd64, o buildx ha emesso un indice di immagine OCI                                                                    | Compilate con `--platform=linux/amd64 --provenance=false`                                                                                                                                                                                        |
| L'avvio del gateway esce con un errore di timeout della connessione Postgres su Cloud Run | Il servizio non è allegato al VPC, o Cloud SQL non ha IP privato su quel VPC; lo store smette di aspettare dopo 5 secondi                                          | Distribuite con `--network` e `--subnet` per l'uscita VPC diretta e create l'istanza Cloud SQL con `--no-assign-ip` e `--network` che punta allo stesso VPC                                                                                      |
| Le richieste di Agent Platform restituiscono `403 PERMISSION_DENIED`                      | Il runtime non utilizza l'account di servizio `claude-gateway`, o il modello non è abilitato in Model Garden per il progetto                                       | Impostate `--service-account` su Cloud Run o associate Workload Identity su GKE e abilitate ogni modello Claude in Model Garden per la regione di destinazione                                                                                   |
| Le risposte di streaming si interrompono dopo una durata fissa                            | Timeout della richiesta del front end: il servizio backend del load balancer dietro GKE Ingress ha un timeout predefinito di 30 secondi e Cloud Run di 300 secondi | Allegate un BackendConfig con un `timeoutSec` elevato su GKE, o distribuite con `--timeout=3600` su Cloud Run                                                                                                                                    |

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Riferimento di configurazione](/docs/it/claude-apps-gateway-config): ogni opzione di `gateway.yaml`, incluse `managed.policies` e `telemetry`
* [Distribuzione e operazioni](/docs/it/claude-apps-gateway-deploy): configurazione IdP, controlli di integrità, rotazione della chiave segreta JWT, aggiornamenti e il modello di sicurezza
* [Panoramica del gateway delle app Claude](/docs/it/claude-apps-gateway): guida rapida e connessione degli sviluppatori
