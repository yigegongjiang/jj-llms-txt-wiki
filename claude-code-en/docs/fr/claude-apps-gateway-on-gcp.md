> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Déployer la passerelle Claude apps sur Google Cloud

> Un exemple concret d'exécution de la passerelle Claude apps sur Google Cloud : Cloud Run ou GKE, Cloud SQL pour PostgreSQL, Secret Manager et authentification par compte de service vers Agent Platform.

<Note>
  Cette page vous guide à travers une façon d'exécuter la passerelle Claude apps sur Google Cloud. La configuration est un exemple fonctionnel pour une infrastructure gérée par le client plutôt qu'un déploiement de production pris en charge ; utilisez-la pour voir comment les éléments s'assemblent avant de l'adapter à votre propre environnement. Pour les exigences indépendantes de la plateforme, consultez le [guide de déploiement](/docs/fr/claude-apps-gateway-deploy).
</Note>

Cet exemple provisionne la passerelle Claude apps sur Google Cloud avec Agent Platform de Google Cloud comme upstream de modèle, en utilisant soit Cloud Run soit GKE pour le calcul. Google Workspace est le fournisseur d'identité (IdP) d'exemple, mais tout fournisseur d'identité conforme à OpenID Connect (OIDC) fonctionne ; seul le bloc `oidc` change. Consultez [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup) pour les détails spécifiques à chaque IdP.

<h2 id="what-you’ll-build">
  Ce que vous allez construire
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Diagramme de la passerelle Claude apps sur Google Cloud : les clients Claude Code se connectent via HTTPS à la passerelle (Cloud Run ou GKE), qui s'exécute à l'intérieur d'un VPC aux côtés d'une base de données Cloud SQL avec IP privée pour l'état de session. La passerelle connecte les utilisateurs via OIDC contre Google Workspace, lit la configuration et les secrets de Secret Manager, transfère les demandes de modèle à Agent Platform et récupère son image d'Artifact Registry au déploiement." width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

La configuration de référence provisionne :

* Un service **Cloud Run** ou un **déploiement GKE** exécutant le conteneur de la passerelle
* Un référentiel **Artifact Registry** pour l'image de la passerelle
* Une instance **Cloud SQL pour PostgreSQL**, IP privée uniquement, pour le [store](/docs/fr/claude-apps-gateway-config#store) de la passerelle
* Des secrets **Secret Manager** pour `gateway.yaml`, la clé de signature JWT, le secret client OIDC et l'URL Postgres
* Un **compte de service** avec `roles/aiplatform.user`, attaché directement sur Cloud Run ou lié via Workload Identity sur GKE
* Un **équilibreur de charge d'application interne** sur Cloud Run, ou un **GKE Ingress** interne de classe `gce-internal` sur GKE, pour HTTPS

<h2 id="prerequisites">
  Prérequis
</h2>

* Un projet GCP avec facturation activée et permission de créer les ressources ci-dessus
* L'interface de ligne de commande `gcloud`, authentifiée avec `gcloud auth login`, et Docker installé localement
* Pour la piste GKE : `kubectl` et un cluster GKE sur le VPC créé dans la procédure pas à pas ci-dessous
* Accès aux modèles Claude dont vous avez besoin dans Model Garden, dans une région qui les publie
* Un client d'application web OAuth 2.0 Google Workspace avec URI de redirection `https://<gateway-host>/oauth/callback` ; consultez [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup)
* Un nom d'hôte TLS pour la passerelle, généralement un nom DNS interne pointant vers l'équilibreur de charge

Définissez le projet et la région une fois :

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  Déployer la passerelle
</h2>

Les étapes ci-dessous provisionnent le déploiement complet avec des commandes `gcloud`.

<Steps>
  <Step title="Activer les API">
    Activez les API de service que la procédure pas à pas utilise :

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

    Les API dont vous avez besoin dépendent de la piste de déploiement :

    * `compute` et `servicenetworking` : nécessaires pour la piste Cloud SQL avec IP privée
    * `run` : Cloud Run uniquement
    * `container` : GKE uniquement
  </Step>

  <Step title="Créer le compte de service et accorder l'IAM">
    La passerelle s'exécute en tant que compte de service dédié avec permission d'appeler Agent Platform. Elle atteint Cloud SQL sur le VPC avec un utilisateur de mot de passe, donc aucun rôle IAM Cloud SQL n'est requis :

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    Ensuite, activez les modèles Claude pour le projet dans Model Garden ; les modèles publient dans des régions spécifiques, donc vérifiez chaque fiche de modèle.
  </Step>

  <Step title="Construire et pousser l'image vers Artifact Registry">
    Construisez l'image selon les [exigences d'image de conteneur](/docs/fr/claude-apps-gateway-deploy#container-image), en utilisant le binaire glibc `linux-x64`, et poussez-la :

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

  <Step title="Provisionner Cloud SQL pour PostgreSQL">
    Créez l'instance sur un VPC via Private Services Access afin qu'elle n'ait pas d'IP publique ; cela satisfait également les projets où `constraints/sql.restrictPublicIp` est appliqué :

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

    Le runtime Cloud Run ou GKE doit être sur, ou routé dans, ce VPC.
  </Step>

  <Step title="Écrire gateway.yaml">
    Le bloc `upstreams` pointe vers Agent Platform avec `auth: {}`, donc la passerelle s'authentifie via Application Default Credentials du compte de service du runtime. Consultez la [référence de configuration](/docs/fr/claude-apps-gateway-config) pour chaque champ.

    Deux champs `listen` dépendent de ce qui se trouve devant la passerelle :

    * `public_url` : requis derrière Cloud Run ou un GKE Ingress. La passerelle construit le `redirect_uri` IdP et son document de découverte uniquement à partir de cette valeur, jamais à partir des en-têtes `X-Forwarded-*`.
    * `trusted_proxies` : les plages source du front-end. La passerelle honore `X-Forwarded-For` uniquement lorsque le pair TCP se trouve dans cette liste, puis parcourt la chaîne au-delà des sauts de confiance, donc les limites de taux de connexion par IP et les événements d'audit enregistrent les IP des développeurs au lieu de celle de l'équilibreur de charge.

    Définissez `trusted_proxies` pour correspondre à votre front-end. Un GKE Ingress externe de classe `gce` n'est pas listé : il provisionne une adresse de règle de transfert publique, que la vérification [réseau privé](/docs/fr/claude-apps-gateway#prerequisites) de `/login` rejette.

    | Front-end                                                    | `trusted_proxies`                                                   |
    | ------------------------------------------------------------ | ------------------------------------------------------------------- |
    | Cloud Run atteint directement, sans équilibreur de charge    | `[169.254.0.0/16]`                                                  |
    | Équilibreur de charge d'application interne devant Cloud Run | `169.254.0.0/16` plus le CIDR de votre sous-réseau réservé au proxy |
    | GKE Ingress interne, classe `gce-internal`                   | Le CIDR de votre sous-réseau réservé au proxy                       |

    L'exemple ci-dessous utilise les valeurs de l'équilibreur de charge interne devant Cloud Run.

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
      Les id\_tokens Google ne portent aucune revendication `groups`. Pour utiliser des politiques basées sur les groupes dans [`managed.policies`](/docs/fr/claude-apps-gateway-config#managed) avec Google Workspace comme IdP, configurez [`oidc.google_groups`](/docs/fr/claude-apps-gateway-config#oidc), qui recherche les groupes de chaque utilisateur via l'API Admin SDK Directory en utilisant un compte de service avec délégation au niveau du domaine. Sans cela, faites correspondre sur `email_domain` à la place.
    </Note>
  </Step>

  <Step title="Stocker les secrets dans Secret Manager">
    Créez quatre secrets et accordez `roles/secretmanager.secretAccessor` au compte de service `claude-gateway` :

    | Secret                       | Source                                          |
    | ---------------------------- | ----------------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`                       |
    | `gateway-oidc-client-secret` | Google Cloud Console → Client OAuth             |
    | `gateway-postgres-url`       | `$GATEWAY_POSTGRES_URL` de l'étape Cloud SQL    |
    | `gateway-config`             | le `gateway.yaml` complet de l'étape précédente |

    La façon dont les secrets atteignent le conteneur diffère selon la piste :

    * Sur GKE, ils se montent en tant que fichiers via le pilote CSI Secret Manager, et `gateway.yaml` référence `${file:/secrets/...}`.
    * Sur Cloud Run, qui ne peut pas monter plusieurs secrets dans un seul répertoire, `gateway.yaml` se monte en tant que fichier et les trois autres s'injectent en tant que variables d'environnement, donc `gateway.yaml` référence `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` et `${GATEWAY_POSTGRES_URL}` à la place.
  </Step>

  <Step title="Déployer">
    <Tabs>
      <Tab title="Cloud Run">
        La commande ci-dessous déploie pour la production derrière un équilibreur de charge interne.

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

        L'accès VPC direct, via `--network`, `--subnet` et `--vpc-egress=private-ranges-only`, permet au service d'atteindre l'IP privée Cloud SQL directement. L'accès public aux points de terminaison Agent Platform et `accounts.google.com` va directement sur Internet plutôt que via le VPC, donc aucun Cloud NAT n'est nécessaire.

        La vérification IAM de l'invocateur doit être ouverte ou désactivée. La passerelle exécute son propre OIDC et ses clients ne portent aucun jeton GCP, donc la vérification de l'invocateur de Cloud Run doit admettre les demandes non authentifiées. L'authentification OIDC de la passerelle authentifie la demande une fois qu'elle atteint le conteneur, avec `allowed_email_domains` contrôlant les domaines autorisés à se connecter.

        Deux drapeaux admettent les demandes non authentifiées :

        * `--no-invoker-iam-check` : désactive la vérification sans liaison `allUsers` à gérer, et fonctionne sous Domain Restricted Sharing
        * `--allow-unauthenticated` : accorde à `allUsers` le rôle `run.invoker` ; utilisez-le si votre organisation n'autorise pas `--no-invoker-iam-check`

        La restriction d'accès via `--ingress` est une couche indépendante et séparée de la vérification de l'invocateur ; gardez-la définie pour limiter le service à votre réseau d'entreprise.

        Par défaut, l'URL Cloud Run `*.run.app` se résout en une adresse publique, que la vérification [réseau privé](/docs/fr/claude-apps-gateway#prerequisites) de `/login` rejette. Deux topologies donnent aux développeurs un nom d'hôte résoluble en privé, et Cloud Run ne provisionne ni l'une ni l'autre pour vous :

        * **Équilibreur de charge d'application interne**, la topologie que la commande de déploiement ci-dessus suppose : déployez avec `--ingress=internal-and-cloud-load-balancing`, provisionnez un équilibreur de charge d'application interne devant le service avec un nom DNS interne et un certificat, et définissez `listen.public_url` sur ce nom d'hôte.
        * **Accès interne uniquement sans équilibreur de charge** : déployez avec `--ingress=internal` et laissez `listen.public_url` comme l'URL `*.run.app`, la valeur par défaut dans les [actifs de référence](#terraform-reference) ci-dessous. Pour que `*.run.app` se résolve en privé, votre équipe réseau doit déjà exploiter un point de terminaison Private Service Connect pour les API Google, une zone privée Cloud DNS résolvant `*.run.app` vers celui-ci, et un routage sur site vers ce point de terminaison.

        Le [guide de mise en réseau privée de Google pour Cloud Run](https://cloud.google.com/run/docs/securing/private-networking) couvre l'infrastructure que les deux options nécessitent. Vérifiez la connexion une fois que la passerelle est servie sur un nom d'hôte privé ; jusqu'à ce moment, confirmez que le conteneur a démarré à partir de ses journaux dans Cloud Run.

        Mettez à jour l'URI de redirection autorisé du client OAuth vers `<public_url>/oauth/callback` avant la première connexion. Redéployez après avoir modifié `public_url`, car la passerelle construit son origine publique uniquement à partir de ce paramètre et ignore `X-Forwarded-Host` et `X-Forwarded-Proto`. `X-Forwarded-For` est honoré pour les IP client uniquement lorsque `listen.trusted_proxies` est défini.
      </Tab>

      <Tab title="GKE">
        Le cluster doit être sur le `$VPC` créé à l'étape Cloud SQL afin que les pods puissent atteindre l'IP privée de la base de données ; l'appairage VPC seul ne fonctionne pas, car l'IP privée Cloud SQL est elle-même un réseau appairé et l'appairage n'est pas transitif. Pour créer un nouveau cluster sur ce VPC, passez `--network="$VPC" --subnetwork=cc-gateway-subnet` à `gcloud container clusters create`.

        Activez Workload Identity sur le cluster et ses pools de nœuds, puis liez le compte de service Google au compte de service Kubernetes afin que les pods héritent de ses identifiants :

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

        Déployez la passerelle en tant que Deployment standard plus un Service et un Ingress interne, classe `gce-internal`, comme décrit dans [Déploiement Kubernetes](/docs/fr/claude-apps-gateway-deploy#kubernetes), avec :

        * `serviceAccountName: gateway`
        * le pilote CSI Secret Manager montant les secrets à `/secrets`
        * la sonde de disponibilité pointée vers `GET /readyz`

        Attachez un BackendConfig avec un `timeoutSec` augmenté au Service de la passerelle : le service backend de l'équilibreur de charge derrière GKE Ingress par défaut à un délai d'expiration de 30 secondes, ce qui coupe les réponses de streaming longues.

        N'appliquez pas une NetworkPolicy d'accès sortant qui bloque `169.254.169.254` sur un cluster Workload Identity ; le pod doit atteindre le serveur de métadonnées pour les identifiants. La [garde SSRF](/docs/fr/claude-apps-gateway-deploy#threat-model-summary) intégrée de la passerelle est la défense là.

        La passerelle enregistre un avertissement de démarrage que le point de terminaison des métadonnées est accessible et suggère d'appliquer une NetworkPolicy d'accès sortant. Sous Workload Identity, cet avertissement est attendu, car le pod a besoin du point de terminaison.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Pousser l'URL de la passerelle vers les machines des développeurs">
    La passerelle s'exécute maintenant, mais les développeurs ne peuvent pas la atteindre à partir de `/login` jusqu'à ce que l'URL de la passerelle soit sur leurs machines. Définissez `forceLoginMethod` et `forceLoginGatewayUrl` dans le [fichier de paramètres gérés](/docs/fr/claude-apps-gateway#set-the-gateway-url) que vous déployez sur chaque appareil via MDM. Il n'y a pas d'option de passerelle dans le sélecteur de connexion pour qu'un développeur sélectionne manuellement.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Référence Terraform
</h2>

Les [actifs de déploiement de référence](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp) automatisent la piste Cloud Run sur cette page ; les actifs de configuration et d'image s'appliquent aux deux pistes :

* `setup.sh` : un provisionneur `gcloud` idempotent qui parcourt le chemin Cloud Run complet, de l'activation des API au premier déploiement
* `terraform/` : le même déploiement en tant qu'infrastructure en tant que code, pour un déploiement sur terrain vierge : une application ciblée pour créer le référentiel Artifact Registry, puis construire et pousser l'image, puis une application complète
* `gateway.yaml.example` et un `Dockerfile` pour l'image de runtime distroless

Les artefacts par défaut Cloud Run ingress à `internal`, donc aucun équilibreur de charge n'est requis. Pour correspondre au déploiement de production derrière un ALB de cette page, exécutez `setup.sh` avec `INGRESS=internal-and-cloud-load-balancing`, ou définissez la variable Terraform `ingress` à `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER`. Les artefacts définissent également par défaut la couche d'invocateur à une concession `allUsers` `run.invoker` plutôt qu'à `--no-invoker-iam-check`, l'inverse de la procédure pas à pas de cette page ; l'un ou l'autre fonctionne, et le choix dépend des contraintes de politique de votre organisation.

Les actifs sont fournis en tant qu'exemples fonctionnels, non en tant qu'artefact de production pris en charge ; examinez-les et adaptez-les à votre environnement.

<h2 id="troubleshooting">
  Dépannage
</h2>

Pour les erreurs de démarrage et de connexion de la passerelle, consultez le tableau [dépannage](/docs/fr/claude-apps-gateway-deploy#troubleshooting) indépendant de la plateforme. Les entrées ci-dessous sont spécifiques à Google Cloud.

| Symptôme                                                                                                           | Cause                                                                                                                                                                   | Correction                                                                                                                                                                                                                            |
| ------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cloud Run retourne `403 Forbidden` avant d'atteindre le conteneur                                                  | La vérification IAM de l'invocateur est toujours activée                                                                                                                | Déployez avec `--no-invoker-iam-check`, ou accordez à `allUsers` le rôle `run.invoker` avec `--allow-unauthenticated`                                                                                                                 |
| `--no-invoker-iam-check` rejeté avec `invoker_iam_disabled is not currently available`                             | Bloqué par `constraints/run.managed.requireInvokerIam`                                                                                                                  | Utilisez `--allow-unauthenticated`. Si Domain Restricted Sharing via `constraints/iam.allowedPolicyMemberDomains` bloque également cela, utilisez la piste GKE, qui expose la passerelle au niveau du réseau sans liaison `allUsers`. |
| `Container manifest type … must support amd64/linux` au déploiement                                                | L'image a été construite sur un hôte non-amd64, ou buildx a émis un index d'image OCI                                                                                   | Construisez avec `--platform=linux/amd64 --provenance=false`                                                                                                                                                                          |
| Le démarrage de la passerelle se termine avec une erreur de délai d'expiration de connexion Postgres sur Cloud Run | Le service n'est pas attaché au VPC, ou Cloud SQL n'a pas d'IP privée sur ce VPC ; le store arrête d'attendre après 5 secondes                                          | Déployez avec `--network` et `--subnet` pour l'accès VPC direct, et créez l'instance Cloud SQL avec `--no-assign-ip` et `--network` pointant vers le même VPC                                                                         |
| Les demandes Agent Platform retournent `403 PERMISSION_DENIED`                                                     | Le runtime n'utilise pas le compte de service `claude-gateway`, ou le modèle n'est pas activé dans Model Garden pour le projet                                          | Définissez `--service-account` sur Cloud Run ou liez Workload Identity sur GKE, et activez chaque modèle Claude dans Model Garden pour la région cible                                                                                |
| Les réponses de streaming se coupent après une durée fixe                                                          | Délai d'expiration de la demande du front-end : le service backend de l'équilibreur de charge derrière GKE Ingress par défaut à 30 secondes et Cloud Run à 300 secondes | Attachez un BackendConfig avec un `timeoutSec` augmenté sur GKE, ou déployez avec `--timeout=3600` sur Cloud Run                                                                                                                      |

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Référence de configuration](/docs/fr/claude-apps-gateway-config) : chaque option `gateway.yaml`, y compris `managed.policies` et `telemetry`
* [Déploiement et opérations](/docs/fr/claude-apps-gateway-deploy) : configuration IdP, vérifications de santé, rotation des clés secrètes JWT, mises à niveau et le modèle de sécurité
* [Aperçu de la passerelle Claude apps](/docs/fr/claude-apps-gateway) : démarrage rapide et connexion des développeurs
