> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Passerelle Claude apps pour Amazon Bedrock, Claude Platform sur AWS, Google Cloud et Microsoft Foundry

> Exécutez Claude Code via Amazon Bedrock, Claude Platform sur AWS, Google Cloud ou Microsoft Foundry derrière une passerelle auto-hébergée avec authentification SSO, accès aux modèles par groupe et télémétrie OTLP.

<Note>
  La passerelle Claude apps est conçue pour les organisations qui doivent — ou préfèrent — acheminer l'inférence via leur propre fournisseur cloud, par exemple pour respecter les exigences de [résidence des données](/docs/fr/claude-apps-gateway-deploy#compliance-posture). Si vous n'avez pas cette exigence et souhaitez accéder à d'autres fonctionnalités telles que l'approvisionnement SCIM ou Claude Code sur web et mobile, Claude Enterprise peut être un meilleur choix. Consultez la page de [disponibilité des fonctionnalités](/docs/fr/feature-availability) pour une comparaison complète de toutes les méthodes de déploiement.
</Note>

Claude apps gateway est un service auto-hébergé qui se situe entre les clients Claude Code de vos développeurs et votre fournisseur de modèles. Les développeurs se connectent avec votre fournisseur d'identité (IdP) d'entreprise au lieu de détenir des clés API ou des identifiants cloud. La passerelle détient les identifiants en amont, applique l'accès aux modèles et les [paramètres gérés](/docs/fr/managed-settings) par groupe IdP, et relaye la télémétrie d'utilisation vers votre propre pile d'observabilité.

Elle est incluse dans le binaire `claude`, donc le même exécutable qui exécute Claude Code sur un ordinateur portable exécute le serveur de passerelle avec `claude gateway --config gateway.yaml`.

Cette page couvre :

* [Pourquoi Claude apps gateway](#why-claude-apps-gateway), ce qu'il ajoute par rapport à l'exécution de votre propre solution, et quand quelque chose d'autre convient mieux
* Un [démarrage rapide](#quickstart) avec les [prérequis](#prerequisites) qui fait passer une passerelle de zéro à un développeur connecté
* [Connecter les développeurs](#connect-developers), y compris la définition de l'URL de la passerelle via les paramètres gérés
* [Disponibilité et limitations](#availability-and-limitations) couvrant les fonctionnalités Claude Code qui fonctionnent via la passerelle et ce que le serveur supporte

Les pages complémentaires approfondissent le sujet. La [référence de configuration](/docs/fr/claude-apps-gateway-config) couvre chaque option du fichier YAML que le démarrage rapide écrit, et le [guide de déploiement](/docs/fr/claude-apps-gateway-deploy) couvre la configuration par IdP, le déploiement Kubernetes et Cloud Run, et les opérations.

<h2 id="why-claude-apps-gateway">
  Pourquoi Claude apps gateway
</h2>

L'[aperçu de la passerelle](/docs/fr/gateways) couvre ce qu'une passerelle fait et pourquoi vous en exécuteriez une. Claude apps gateway est la propre passerelle d'Anthropic, intégrée au binaire `claude` et testée aux côtés de chaque version de Claude Code, elle transmet donc les en-têtes et les champs de requête que Claude Code envoie sans que les opérateurs maintiennent une liste d'autorisation distincte. Une fois déployée, elle vous donne :

* **Identifiants** : la clé API en amont ou l'identifiant cloud ne vit que dans votre infrastructure. Les développeurs s'authentifient avec SSO d'entreprise et reçoivent des jetons porteurs de courte durée, donc le déprovisionnement se fait dans votre IdP. Déprovisionner un utilisateur et son accès à la passerelle expire dans la durée de vie de la session, une heure par défaut.
* **Contrôle d'accès** : vos groupes IdP correspondent à des listes d'autorisation de modèles et à des politiques de [paramètres gérés](/docs/fr/managed-settings). La passerelle applique l'accès aux modèles côté serveur, rejetant les demandes pour les modèles non accordés, et sélectionne la politique de paramètres gérés de chaque groupe, que l'interface de ligne de commande applique au [niveau des paramètres gérés](/docs/fr/settings#settings-precedence). Différentes équipes obtiennent différents modèles, outils et permissions, et un développeur ne peut pas remplacer ce que sa politique verrouille.
* **Livraison des paramètres** : la passerelle livre les paramètres gérés aux clients connectés elle-même, remplaçant les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) de la console d'administration claude.ai.
* **Télémétrie** : chaque destination configurée reçoit les [métriques OpenTelemetry Protocol (OTLP)](/docs/fr/monitoring-usage) avec les nombres de jetons, le modèle, l'identité de l'utilisateur et la latence par défaut, avec les journaux et les traces comme des options par destination.
* **Routage en amont** : les clients parlent l'API Anthropic Messages à la passerelle, et la passerelle traduit pour chaque amont, qu'il s'agisse de Bedrock, de la [plateforme Claude sur AWS](/docs/fr/claude-platform-on-aws), de la plateforme Agent de Google Cloud, de Foundry ou de l'API Anthropic, avec basculement entre eux. Vous pouvez modifier les régions, les fournisseurs ou l'ordre de basculement sans que les développeurs le remarquent ou se reconfigurent.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagramme montrant les clients Claude Code et les onglets Chat, Cowork et Code de Claude Desktop se connectant via HTTPS avec des jetons porteurs à une passerelle Claude apps auto-hébergée dans votre infrastructure, qui connecte les utilisateurs à votre IdP, stocke l'état d'authentification dans PostgreSQL, relaye la télémétrie à votre collecteur OTLP et transmet l'inférence à Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry ou l'API Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  Le plan de données de la passerelle n'envoie rien à l'infrastructure Anthropic sauf si l'API Anthropic est un amont configuré. Vous contrôlez où la télémétrie, les journaux d'audit, les paramètres gérés et l'identité IdP de vos développeurs vont, et la passerelle ne les envoie à Anthropic. Pour le trafic restant que le processus CLI peut envoyer et comment le fermer, voir [Posture de conformité](/docs/fr/claude-apps-gateway-deploy#compliance-posture).
</Note>

Pour les fonctionnalités Claude Code qui fonctionnent via la passerelle et ce que le serveur lui-même supporte, voir [Disponibilité et limitations](#availability-and-limitations) ci-dessous. Pour les décisions telles que le coût, le contournement, l'exécution de plusieurs passerelles et les plates-formes sans serveur, voir le [guide de déploiement](/docs/fr/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Autres implémentations de passerelle
</h3>

Si vous exécutez déjà une passerelle LLM ou une passerelle API qui répond à vos besoins, continuez à l'utiliser ; [Autres passerelles LLM](/docs/fr/llm-gateway) couvre la configuration de Claude Code contre elle.

La [guide de compatibilité de la passerelle](/docs/fr/llm-gateway-protocol) documente ce que Claude Code attend de toute passerelle : les points de terminaison qu'elle appelle, les en-têtes et les champs de corps à transmettre, et ce qui cesse de fonctionner quand ils sont supprimés. Une passerelle Claude apps en cours d'exécution sert également sa propre référence de protocole à `GET /protocol`, qui décrit les points de terminaison qu'elle expose aux clients Claude Code : connexion SSO, inférence, livraison des paramètres gérés, découverte de modèles et télémétrie. Récupérez-le avec `curl https://claude-gateway.internal.example.com/protocol` à partir de n'importe quelle passerelle déployée, comme celle que le [démarrage rapide](#quickstart) ci-dessous produit.

Les modifications majeures du protocole sont annoncées à l'avance, mais la compatibilité rétroactive indéfinie n'est pas garantie.

<h2 id="quickstart">
  Démarrage rapide
</h2>

Ce démarrage rapide parcourt le chemin minimal : enregistrez un client OAuth dans votre IdP, écrivez un `gateway.yaml`, exécutez la passerelle aux côtés de Postgres avec Docker Compose, et vérifiez la connexion de bout en bout. Il utilise un amont Amazon Bedrock ; Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry, et l'API Anthropic sont également supportés en échangeant le bloc `upstreams` comme indiqué dans la [référence de configuration](/docs/fr/claude-apps-gateway-config#upstreams). À la fin, vous avez une passerelle à laquelle un développeur peut se `/login`.

<Note>
  **Déployez sur votre réseau privé.** Claude Code ne se connecte qu'à une passerelle dont l'adresse est privée. C'est un garde de sécurité, car une passerelle de confiance peut pousser des paramètres qui exécutent des commandes sur les machines des développeurs. Mettez la passerelle derrière un équilibreur de charge interne ou un VPN et donnez-lui un nom d'hôte qui ne se résout qu'à des adresses IP privées. Si votre réseau interne est numéroté à partir d'un espace IPv4 public que votre organisation possède, voir [Autoriser une passerelle sur un espace d'adresses public que vous possédez](#allow-a-gateway-on-public-address-space-you-own).
</Note>

<h3 id="prerequisites">
  Prérequis
</h3>

Ayez ceci en place avant de commencer :

| Vous avez besoin                             | Détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 ou ultérieur            | La sous-commande `claude gateway` et le flux de connexion de la passerelle sont livrés dans v2.1.195. Les versions publiques antérieures ne les incluent pas. La machine exécutant le serveur de passerelle et la machine de chaque développeur doivent être sur v2.1.195 ou ultérieur ; exécutez `claude update` pour obtenir la dernière version. L'[amont Claude Platform on AWS](/docs/fr/claude-apps-gateway-config#claude-platform-on-aws) nécessite Claude Code v2.1.198 ou ultérieur sur le serveur de passerelle.                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Fournisseur d'identité OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak, ou Dex, ou tout autre IdP conforme à OIDC comme PingFederate. La passerelle exécute la découverte OIDC standard et le flux de code d'autorisation contre elle. SAML et LDAP ne sont pas supportés.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| PostgreSQL 14 ou ultérieur                   | Soutient le flux de connexion d'appareil, où le rappel du navigateur écrit et l'interface de ligne de commande d'interrogation lit, plus les compteurs de limite de débit. Tout Postgres géré fonctionne, y compris le plus petit niveau. Sans limites de dépenses configurées, la passerelle stocke quelques Ko d'état d'authentification de courte durée ; avec les [limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits), elle détient également des tables de dépenses durables, d'audit et d'identité qui doivent être sauvegardées. TLS via `?sslmode=require` est recommandé.                                                                                                                                                                                                                                                                                                                                                                                       |
| Amont du modèle                              | Identifiants Amazon Bedrock, identifiants Claude Platform on AWS, identifiants Google Cloud, une ressource Microsoft Foundry, ou une clé API Anthropic. Plusieurs ammonts sont supportés avec basculement.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| HTTPS                                        | La passerelle doit être accessible via `https://` à partir des ordinateurs portables des développeurs et de tout navigateur utilisé pour la connexion ; la passerelle sert la page de vérification d'appareil sur le même écouteur. Fournissez un certificat TLS via `listen.tls` ou exécutez derrière un ingress qui termine TLS, et définissez `listen.public_url` à l'origine externe dans les deux cas. Une origine `http://` simple est acceptée uniquement quand l'hôte de la passerelle est une boucle locale : `localhost`, `127.0.0.1`, ou `::1`.                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Adresse de réseau privé                      | À `/login`, Claude Code exige que le nom d'hôte ou l'adresse IP de la passerelle ne se résolve qu'à des adresses privées : RFC 1918, link-local, CGNAT `100.64.0.0/10`, ULA IPv6 `fc00::/7`, ou boucle locale. Pour une passerelle que vous hébergez, toute adresse publique en dehors d'un bloc que vous déclarez est rejetée ; voir le [modèle de menace](/docs/fr/claude-apps-gateway-deploy#threat-model-summary) dans le guide de déploiement. Si les machines des développeurs acheminent HTTPS via un proxy d'entreprise, la connexion exige également que l'hôte proxy se résolve à des adresses privées ; s'il ne le fait pas, ajoutez l'hôte de la passerelle à `NO_PROXY` pour que l'interface de ligne de commande se connecte directement. Si votre réseau interne est numéroté à partir d'un espace IPv4 public que votre organisation possède, [déclarez ces blocs](#allow-a-gateway-on-public-address-space-you-own) pour que `/login` accepte une passerelle là-bas. |
| Runtime Linux                                | Le serveur de passerelle s'exécute uniquement sur le binaire Linux natif. macOS fonctionne pour le développement local. Windows n'est pas supporté comme plate-forme serveur.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

<h3 id="steps">
  Étapes
</h3>

<Steps>
  <Step title="Enregistrez un client OAuth dans votre IdP">
    Décidez d'abord du nom d'hôte de la passerelle, car l'URI de redirection doit le correspondre. Créez une nouvelle application web OIDC et définissez l'URI de redirection sur `https://claude-gateway.<your-domain>/oauth/callback`, où l'hôte est la même valeur que vous définissez comme [`listen.public_url`](/docs/fr/claude-apps-gateway-config#listen) à l'étape 3. Notez le `client_id` et le `client_secret`. Les instructions par IdP sont dans [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Provisionner une base de données PostgreSQL">
    Tout Postgres 14 ou ultérieur fonctionne, y compris le plus petit niveau géré. La passerelle exécute ses propres migrations de schéma au démarrage, donc le rôle de la base de données a besoin de droits pour créer et modifier les tables ; voir [`store`](/docs/fr/claude-apps-gateway-config#store).
  </Step>

  <Step title="Écrivez gateway.yaml">
    Les secrets sont lus via l'expansion `${ENV_VAR}` pour que le fichier lui-même puisse vivre dans le contrôle de version. Utilisez un nom d'hôte `public_url` qui se résout à une adresse IP privée sur votre réseau, car `/login` rejette les adresses publiques. La configuration minimale a cinq sections, et tous les autres champs ont une valeur par défaut :

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Requis sauf si l'hôte est une adresse de boucle locale. Utilisé pour l'IdP
      # redirect_uri et le document de découverte.
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # doit servir /.well-known/openid-configuration
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # rejeter les id_tokens en dehors de votre org
      userinfo_fallback: true                  # pour les IdPs dont l'id_token omet email/groups ; inoffensif sinon

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # limite également la latence de révocation lors du déprovisionnement IdP

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # ajouter ?sslmode=require pour Postgres géré

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # vide : chaîne de credentials AWS par défaut
    # (IRSA, rôle de tâche EC2/ECS, variables d'env, ~/.aws)

    # Les modèles sont traduits par amont automatiquement. Le catalogue intégré
    # mappe claude-opus-4-8 à us.anthropic.claude-opus-4-8 et ainsi de suite pour chaque
    # modèle Claude supporté par Bedrock. Définissez false et ajoutez une liste `models:` pour
    # exposer uniquement des modèles spécifiques.
    auto_include_builtin_models: true
    ```

    Cette configuration est suffisante pour une boucle de connexion fonctionnelle avec le catalogue de modèles Bedrock par défaut. Une fois qu'elle s'exécute, ajoutez RBAC par groupe et paramètres gérés via [`managed.policies`](/docs/fr/claude-apps-gateway-config#managed), fan-out de télémétrie via [`telemetry`](/docs/fr/claude-apps-gateway-config#telemetry), et basculement multi-amont, ARNs de débit provisionné, ou régions non-US via [`models`](/docs/fr/claude-apps-gateway-config#models).

    <Note>
      L'amont Amazon Bedrock a besoin d'un principal AWS avec `bedrock:InvokeModel` et `bedrock:InvokeModelWithResponseStream` sur les ARNs `inference-profile/us.anthropic.*` et les ARNs `foundation-model/anthropic.*` sous-jacents. Il a également besoin du formulaire d'utilisation unique d'Anthropic soumis pour le compte à partir du catalogue de modèles de la console Bedrock.

      Fournissez l'identifiant avec IRSA sur EKS, un rôle de tâche ECS, ou un profil d'instance EC2 plutôt que des clés statiques. La [référence `upstreams`](/docs/fr/claude-apps-gateway-config#upstreams) a les détails IAM complets, la matrice de credentials inter-cloud, et les blocs `auth` pour les autres fournisseurs.
    </Note>
  </Step>

  <Step title="Exécutez-le">
    Construisez une image de conteneur autour du binaire `claude` qui répond aux [exigences d'image](/docs/fr/claude-apps-gateway-deploy#container-image), puis exécutez-la aux côtés de Postgres. Le fichier Compose référence l'image comme `registry.example.com/claude-gateway:2.1.198` ; remplacez votre propre registre et balise d'image :

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
          # Identifiants AWS : en production, omettez ceux-ci et utilisez un rôle
          # d'instance. Pour les tests Compose locaux, transmettez les vôtres :
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

    La passerelle est un binaire Linux unique qui lit la configuration, se connecte à Postgres et applique ses migrations de schéma, exécute la découverte OIDC contre votre IdP, construit les clients en amont, et commence à écouter.

    Le démarrage échoue fermé pour la configuration, la connexion Postgres, la découverte OIDC, et la construction du client en amont. Si l'un de ceux-ci est inaccessible ou mal configuré, la passerelle se termine avec une erreur plutôt que de servir le trafic dans un état dégradé.

    Un démarrage réussi ne valide pas le chemin d'inférence, car les identifiants d'instance Bedrock et Agent Platform se résolvent à la première demande, pas au démarrage.

    Regardez stderr pour la séquence de démarrage. Les lignes de journal utilisent le format `[gateway] <timestamp> <level> <message>`, les événements d'audit sont JSON sur une seule ligne avec un champ `evt`, et une bannière de démarrage, omise ci-dessous, s'imprime entre les lignes de migration et d'écoute. Une base de données fraîche imprime une ligne `migration N applied` par migration de schéma ; une base de données déjà migrée n'en imprime aucune. Vous devriez voir, dans l'ordre :

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.395Z info waiting for migration lock (another replica may be migrating; check pg_locks for key 6775156 if this persists)
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    …
    [gateway] 2026-06-10T17:03:21.431Z info migration 6 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    La passerelle enregistre également un avertissement selon lequel `access_control.allow_cidrs` est vide. C'est attendu ici, car rien ne limite les adresses client que la passerelle sert jusqu'à ce que vous définissiez une liste d'autorisation. La [référence `access_control`](/docs/fr/claude-apps-gateway-config#http-tuning) a les plages recommandées.

    Si le démarrage se termine avant la ligne `claude gateway listening on`, la dernière ligne de stderr nomme le problème :

    * un Postgres inaccessible
    * un rôle Postgres sans permission DDL
    * un document de découverte OIDC inaccessible ou invalide
    * une violation de schéma de configuration avec le chemin de champ offensant

    Corrigez-le et redémarrez.

    Si vous avez déjà un ingress qui termine TLS, ignorez Compose et exécutez le binaire directement avec `claude gateway --config gateway.yaml`. Définissez `public_url` sur l'origine de l'ingress et liez `listen` à une adresse de boucle locale ou interne au cluster.
  </Step>

  <Step title="Vérifiez la surface d'authentification">
    Trois vérifications confirment que la passerelle peut authentifier un utilisateur réel avant de la remettre à un développeur.

    Les exemples utilisent l'URL publique de la passerelle ; pour la configuration Compose locale sans ingress, remplacez `http://localhost:8080` dans les deux premières vérifications. La troisième vérification ouvre `verification_uri_complete`, qui est construite à partir de `public_url`, donc pour Compose local définissez `public_url: http://localhost:8080` dans `gateway.yaml`, et ajoutez `http://localhost:8080/oauth/callback` comme deuxième URI de redirection sur le client OAuth de l'étape 1, car la passerelle construit l'IdP `redirect_uri` à partir de `public_url`. Le lien de vérification s'ouvre alors dans votre navigateur local.

    Dans Windows PowerShell, exécutez `curl.exe` ; le `curl` nu est un alias pour `Invoke-WebRequest` et rejette ces drapeaux.

    Tout d'abord, récupérez le document de découverte, qui confirme que la passerelle est active, la configuration est valide, et tous les contrôles de démarrage ont réussi :

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

    La réponse inclut des champs supplémentaires, tels que `response_types_supported` et `scopes_supported`.

    Deuxièmement, demandez une autorisation d'appareil, qui confirme que le flux de connexion d'appareil fonctionne et que Postgres est accessible et inscriptible :

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

    Troisièmement, testez la jambe du navigateur en ouvrant `verification_uri_complete` dans un navigateur et en confirmant le code. Vous devriez être redirigé vers la page de connexion de votre IdP, et après vous être connecté, atterrir sur la passerelle avec une confirmation de connexion.

    Utilisez la première vérification défaillante pour localiser le problème :

    * **La première vérification échoue** : le démarrage n'a pas été complété ; vérifiez stderr
    * **La deuxième vérification échoue** : Postgres n'est pas accessible à partir de la passerelle ou le rôle ne peut pas écrire ; vérifiez la chaîne de connexion et les permissions
    * **La troisième vérification n'atteint pas l'IdP** : vérifiez que l'URI de redirection de l'IdP correspond exactement à `https://<gateway>/oauth/callback`
    * **La troisième vérification atteint l'IdP mais rebondit avec une erreur** : lisez le journal d'audit de la passerelle, qui enregistre chaque rejet d'authentification avec la raison, comme `email domain not allowed`
  </Step>

  <Step title="Connectez un développeur">
    Cette dernière étape se produit sur une machine de développeur, pas le serveur. Définissez `forceLoginMethod` sur `"gateway"` et `forceLoginGatewayUrl` sur l'URL `public_url` de votre passerelle dans le [fichier de paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms) de cette machine, puis exécutez `/login`, appuyez sur Entrée sur l'écran **Cloud gateway**, et complétez la connexion du navigateur. [Définir l'URL de la passerelle](#set-the-gateway-url) ci-dessous couvre la distribution des deux clés à chaque machine de développeur.
  </Step>
</Steps>

<h2 id="connect-developers">
  Connecter les développeurs
</h2>

Les développeurs se connectent à partir de leurs propres ordinateurs portables avec une seule connexion au navigateur, en utilisant leur compte professionnel d'entreprise. Ils n'ont pas besoin d'un compte claude.ai, d'une clé API ou d'un abonnement, car les demandes au modèle passent par la passerelle en utilisant l'identifiant en amont de l'organisation. La connexion est pilotée par les [paramètres gérés côté client](/docs/fr/claude-apps-gateway-config#client-side-managed-settings) que vous poussez via MDM, il n'y a donc pas de configuration manuelle du côté du développeur ; cette section couvre ce que l'administrateur configure.

L'interface de ligne de commande empreinte le certificat feuille TLS de la passerelle à la première connexion et l'épingle par nom d'hôte. Elle vérifie cette épingle à nouveau lors de la connexion, lors des rafraîchissements de session silencieux et lors des récupérations de paramètres gérés, tandis que les demandes d'inférence utilisent la validation TLS standard sans l'épingle. Les demandes acheminées via un proxy HTTPS ignorent la vérification de l'épingle, donc ajoutez l'hôte de la passerelle à `NO_PROXY` pour les garder directes.

Publiez l'empreinte SHA-256 attendue aux côtés de l'URL de la passerelle pour que les développeurs aient quelque chose à comparer. L'invite `/login` affiche les 16 premiers caractères de l'empreinte en hexadécimal minuscule sans deux-points. Pour imprimer l'empreinte complète sous cette forme à partir du fichier de certificat, exécutez :

```bash theme={null}
openssl x509 -noout -fingerprint -sha256 -in cert.pem | cut -d= -f2 | tr -d : | tr 'A-F' 'a-f'
```

Quand le certificat tourne, chaque développeur voit à nouveau l'invite de confiance, traitez donc les rotations comme un événement planifié et republier l'empreinte. Si votre politique de passerelle inclut [des paramètres qui nécessitent une approbation](/docs/fr/server-managed-settings#security-approval-dialogs), le développeur voit également ce dialogue d'approbation à nouveau après avoir accepté le nouveau certificat, car Claude Code associe la [mémoire d'approbation](/docs/fr/server-managed-settings#approval-memory) au certificat épinglé.

Une passerelle peut retourner le champ optionnel `email` dans sa réponse de jeton pour nommer le compte qu'une connexion a utilisé. Quand elle le fait, le développeur confirme le compte avant que Claude Code ne sauvegarde l'identifiant. Après une connexion confirmée, `/status` affiche le compte.

La confirmation nécessite Claude Code v2.1.275 ou ultérieur sur la machine du développeur ; un client en dessous de cette version ignore le champ. Le serveur de passerelle dans le binaire `claude` ne retourne pas le champ, donc ses connexions se complètent sans la confirmation.

Une fois le développeur connecté, le [sélecteur de modèle](/docs/fr/model-config) affiche les modèles dans la liste d'autorisation `availableModels` du développeur. Les paramètres gérés s'appliquent au démarrage et se rafraîchissent toutes les heures, et la télémétrie s'achemine vers votre collecteur.

Les sessions se rafraîchissent silencieusement avant l'expiration de `ttl_hours`. Quand un rafraîchissement échoue après le déprovisionnement IdP, Claude Code invite le développeur à se reconnecter.

<h3 id="set-the-gateway-url">
  Définir l'URL de la passerelle
</h3>

Trois clés vont dans le fichier de [paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms) par système d'exploitation que vous déployez via MDM ou directement sur le disque. `forceLoginMethod` et `forceLoginGatewayUrl` ouvrent `/login` directement sur l'écran **Cloud gateway** avec l'URL remplie, et `parentSettingsBehavior: "merge"` permet à Claude Desktop de livrer la liste d'autorisation de sortie de la passerelle aux sessions Claude Code qu'il lance, expliqué dans [Livrer la politique à Claude Desktop sessions](#deliver-policy-to-claude-desktop-sessions) :

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

Le développeur appuie sur Entrée pour se connecter. L'invite d'empreinte TLS de [première connexion](#connect-developers) apparaît toujours. Une fois le fichier sur une machine, un développeur qui n'a pas complété la connexion à la passerelle voit l'un des messages décrits sous [La politique d'administrateur nécessite une connexion Cloud gateway](/docs/fr/errors#administrator-policy-requires-a-cloud-gateway-sign-in). Les développeurs qui sélectionnent un fournisseur cloud via une variable d'environnement telle que `CLAUDE_CODE_USE_BEDROCK` n'ont pas besoin de la connexion à la passerelle.

Un développeur ne peut pas configurer cela manuellement. Le sélecteur de connexion n'a pas d'option de passerelle, et `forceLoginGatewayUrl` est ignoré dans les fichiers de paramètres propres d'un développeur. `forceLoginMethod` seul, sans URL, laisse le développeur à un message « Contactez votre administrateur informatique ». Les clés de connexion appartiennent au fichier que vous poussez vers les machines, pas au bloc `managed.policies[].cli` de la passerelle, qui ne atteint que les clients déjà connectés.

<h3 id="allow-a-gateway-on-public-address-space-you-own">
  Autoriser une passerelle sur l'espace d'adresses publiques que vous possédez
</h3>

Certaines organisations numérotent leur réseau interne à partir d'un bloc IPv4 public qu'elles possèdent, comme l'espace d'adresses propre d'un opérateur ou un `/8` hérité, donc leur passerelle ne peut pas avoir d'adresse privée. Listez ces blocs dans le paramètre géré `gatewayInternalNetworks`. `/login` accepte alors une passerelle à l'intérieur d'un bloc listé quand la machine du développeur s'y connecte à partir d'une adresse à l'intérieur du même bloc. Cela nécessite Claude Code v2.1.268 ou ultérieur sur la machine du développeur ; les versions antérieures ignorent la clé et appliquent la règle d'adresse privée.

<Warning>
  `gatewayInternalNetworks` est pour les réseaux internes qui se trouvent être numérotés à partir de l'espace d'adresses public. Cela ne rend pas sûr d'exposer une passerelle à Internet : une passerelle de confiance peut pousser des paramètres qui exécutent des commandes sur les machines des développeurs.

  Gardez la passerelle inaccessible de l'extérieur de votre réseau avec vos règles de pare-feu ou d'équilibreur de charge. Définissez le [`access_control.allow_cidrs`](/docs/fr/claude-apps-gateway-config#http-tuning) de la passerelle aux mêmes blocs que vous déclarez ici, donc la passerelle elle-même refuse les clients de n'importe où ailleurs. Derrière un équilibreur de charge ou une entrée, définissez également `listen.trusted_proxies` à ce front-end, car la passerelle sinon correspond à `allow_cidrs` par rapport à l'adresse propre du front-end plutôt qu'à celle du développeur.
</Warning>

Ajoutez la clé à la même source de paramètres gérés que les clés de connexion : le fichier de paramètres gérés, le profil MDM, ou la politique de registre. Claude Code l'ignore dans les paramètres utilisateur, projet et gérés par serveur.

Cet exemple déclare un bloc. Remplacez `203.0.113.0/24` par votre propre bloc. C'est une plage de documentation, et Claude Code refuse celles-ci.

```json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Claude Code valide la liste à `/login` avant de contacter une passerelle :

* Chaque entrée est un bloc IPv4 écrit comme sa première adresse et un préfixe de `/8` à `/32`.
* La liste contient au maximum quatre blocs, et aucun deux ne se chevauchent.
* Aucun bloc ne chevauche l'espace d'adresses privées : `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `127.0.0.0/8`, `169.254.0.0/16`, et `100.64.0.0/10`. `/login` accepte déjà une passerelle là sans cette clé.
* Aucun bloc ne chevauche l'espace qui n'est jamais le réseau d'une organisation : `198.18.0.0/15` et `192.0.0.0/24`, que les clients VPN et NAT64 détiennent comme adresses locales ; les plages de documentation `192.0.2.0/24`, `198.51.100.0/24`, et `203.0.113.0/24` ; et les plages réservées `0.0.0.0/8`, `192.88.99.0/24`, et multicast `224.0.0.0/4`. Vous pouvez déclarer des blocs à l'intérieur de `240.0.0.0/4`, que certains grands réseaux utilisent comme espace unicast interne.

Les blocs de `managed-settings.json` et ses fichiers drop-in `managed-settings.d/` se combinent en une liste, et ces limites s'appliquent à la liste combinée. Pour réduire un bloc, remplacez son entrée plutôt que d'en ajouter une deuxième qui se chevauche dans un drop-in ; `/login` refuse le chevauchement.

Si une entrée casse une règle, ou la valeur n'est pas une liste de chaînes, Claude Code refuse chaque nouvelle connexion à la passerelle sur cette machine et nomme le problème dans le message. La connexion à une passerelle sur une adresse privée échoue aussi, et les connexions existantes continuent de fonctionner. Essayez la valeur sur une machine avant de la déployer. Claude Code liste également une valeur mal typée parmi les [paramètres gérés invalides qu'il rapporte](/docs/fr/managed-settings#keys-that-fail-closed).

Avec une liste valide, `/login` applique trois vérifications à une passerelle dont l'adresse est à l'intérieur d'un bloc listé :

* Chaque adresse à laquelle le nom d'hôte de la passerelle se résout est à l'intérieur de ce bloc. Claude Code refuse un nom qui a aussi des enregistrements en dehors, y compris les adresses privées et IPv6.
* La machine du développeur se connecte de l'intérieur du même bloc. Claude Code refuse une machine derrière NAT, à l'intérieur d'un conteneur ou WSL2, ou sur un VPN dont le pool d'adresses se situe en dehors du bloc, et nomme l'adresse à partir de laquelle la machine s'est connectée.
* La connexion est directe. Si `HTTPS_PROXY` s'applique à l'hôte de la passerelle, `/login` refuse et nomme l'entrée `NO_PROXY` à ajouter.

Quand les trois passent, l'[invite de confiance](#connect-developers) ajoute une ligne nommant l'adresse de la machine, l'adresse de la passerelle, et le bloc déclaré qui contient les deux.

La clé ne change rien pour les autres passerelles : la connexion à une sur une adresse privée fonctionne comme avant, et la connexion à une sur une adresse publique en dehors de chaque bloc listé est refusée comme avant.

Un bloc déclaré restreint qui peut se connecter mais ne prouve pas où se trouve une machine, donc déclarez uniquement l'espace d'adresses que votre organisation contrôle. Un bloc partagé avec d'autres locataires, comme une plage publique d'un fournisseur cloud, laisse n'importe qui dedans passer la même vérification.

<h3 id="deliver-policy-to-claude-desktop-sessions">
  Livrer la politique à Claude Desktop sessions
</h3>

Claude Desktop exécute ses onglets Cowork et Code, plus l'onglet Chat quand vous l'activez, sur des sessions Claude Code intégrées et envoie leurs demandes de modèle à travers la passerelle. Il transmet la politique à chacune de ces sessions, construite à partir de la configuration que la passerelle lui sert à `/user/bootstrap` : la liste d'autorisation des modèles, les outils désactivés, et la liste d'autorisation de sortie dérivée du bloc `cli` de la politique correspondante, plus la [superposition `desktop`](/docs/fr/claude-apps-gateway-config#claude-desktop-overlay).

D'autres clés `cli`, telles que hooks, `env`, et les règles de permission délimitées comme `Bash(npm *)`, ne sont accessibles qu'aux clients qui se connectent via `/login`. Claude Desktop lit l'URL de la passerelle à partir de sa propre configuration gérée et se connecte avec son propre flux, séparé des clés `forceLoginMethod` et `forceLoginGatewayUrl` dans [Définir l'URL de la passerelle](#set-the-gateway-url).

Les paramètres transmis par un processus de lancement sont des paramètres parents. Claude Code ignore les paramètres parents sur toute machine qui a une source gérée déployée par un administrateur, sauf si la [source qui livre la politique](/docs/fr/managed-settings#which-managed-source-claude-code-uses) définit `parentSettingsBehavior: "merge"`.

<h4 id="which-machines-need-the-opt-in">
  Quelles machines ont besoin de l'opt-in
</h4>

Les machines qui exécutent uniquement Claude Desktop en ont besoin. Claude Desktop applique la liste des modèles et la liste des outils désactivés aux sessions intégrées elle-même, mais la liste d'autorisation de sortie ne les atteint que comme paramètres parents, sous la forme de règles de domaine `WebFetch` et de règles de réseau sandbox. Sans l'opt-in, ces sessions s'exécutent sans la restriction de sortie, et rien ne vous avertit. La passerelle rejette toujours les demandes d'inférence pour les modèles que la politique n'accorde pas.

Les machines où les développeurs se connectent via `/login` n'en ont pas besoin ; chaque session Claude Code récupère sa politique auprès de la passerelle.

Les flottes dont [`policyHelper`](/docs/fr/settings-reference#policyhelper) fournit des paramètres gérés ne peuvent pas l'utiliser : Claude Code ne fusionne jamais les paramètres parents sur ces flottes, car il lit les paramètres gérés à partir de la sortie du helper seule.

<h4 id="set-the-opt-in">
  Définir l'opt-in
</h4>

Déployez l'extrait de paramètres gérés à partir de [Définir l'URL de la passerelle](#set-the-gateway-url), miroir-le vers toute source côté client qui surclasse le fichier, puis vérifiez.

<Steps>
  <Step title="Déployer l'opt-in dans le fichier de paramètres gérés">
    L'[extrait ci-dessus](#set-the-gateway-url) inclut déjà `parentSettingsBehavior: "merge"`, donc le fichier que vous poussez vers les machines le porte.
  </Step>

  <Step title="Miroir l'extrait vers toute source qui surclasse le fichier">
    Claude Code lit `parentSettingsBehavior` uniquement à partir de la [source sélectionnée](/docs/fr/managed-settings#which-managed-source-claude-code-uses). Ajouter une clé de politique à une source peut faire de cette source la source sélectionnée, donc dans une source côté client, miroir l'extrait entier plutôt que `parentSettingsBehavior` seul. [Les paramètres gérés côté client](/docs/fr/claude-apps-gateway-config#client-side-managed-settings) couvrent les flottes qui livrent la politique via Group Policy ou les profils de configuration. Une plist de préférences gérées sur macOS ou une politique HKLM sur Windows surclasse le fichier `managed-settings.json`, et les paramètres gérés distants de la passerelle surclassent les deux, donc sur les machines qui se connectent à la passerelle, définissez également `parentSettingsBehavior` dans le bloc [`cli`](/docs/fr/claude-apps-gateway-config#managed) de la politique de la passerelle.
  </Step>

  <Step title="Vérifier quelle source est sélectionnée">
    Sur une machine qui exécute uniquement Claude Desktop, appelez la méthode [`resolveSettings()`](/docs/fr/agent-sdk/typescript#resolvesettings) du SDK Agent et lisez `policyOrigin` sur l'entrée `managed` dans sa liste `sources`. La valeur nomme la source côté client sélectionnée, `plist`, `hklm`, ou `file`, qui est la source qui doit porter l'extrait. Les sessions intégrées de Claude Desktop ne récupèrent pas la politique de la passerelle, donc le bloc `cli` de la passerelle ne compte jamais comme la source sélectionnée pour elles.
  </Step>
</Steps>

<h3 id="restrict-parent-settings">
  Restreindre les paramètres parents
</h3>

Une fois que vous déployez `parentSettingsBehavior: "merge"`, tout processus hôte qui lance Claude Code peut fournir des paramètres parents, non seulement Claude Desktop mais aussi une application SDK Agent ou une extension IDE.

Claude Code filtre les paramètres parents par rapport à une liste d'autorisation de clés restrictives, mais certaines clés autorisées peuvent accorder l'accès plutôt que de le restreindre. À moins que vous définissiez les verrous `allowManaged*Only`, les règles d'autorisation de permission et les listes d'autorisation sandbox fournies par l'hôte s'appliquent toujours. Les règles de refus et de demande de votre politique restent en vigueur de toute façon ; [elles sont évaluées avant toute règle d'autorisation](/docs/fr/permissions#manage-permissions).

Claude Code transmet les entrées [`sandbox.credentials`](/docs/fr/settings-reference#sandbox-credentials) fournies par le parent sous forme dépouillée :

* **Entrées `deny`** : transmises avec uniquement leur `path` ou `name` et le mode.
* **Entrées de fichier avec [`mode: mask`](/docs/fr/sandboxing#mask-credential-files)** : transmises uniquement sentinelle, comme un masque de fichier entier dont `injectHosts` est la liste vide, donc le proxy ne substitue jamais la valeur réelle pour une entrée fournie par le parent sur aucune plateforme. Tous les champs de masquage structuré sont également supprimés, donc un modèle d'extraction fourni par le parent ne peut pas déplacer un masque plus strict qu'une autre source définit pour le même chemin.
* **Entrées `envVars` avec `mode: mask`** : non transmises. `deny` est la seule restriction que le canal parent peut exprimer via les entrées `envVars`.
* **[`awsPairs` et `sigv4`](/docs/fr/sandboxing#re-sign-aws-requests)** : transmises restriction-uniquement. À partir de `sigv4`, seules les valeurs `deny` sont conservées, et un parent qui définit un bloc `sigv4` du tout épingle les trois formes de demande, `streaming`, `presigned`, et `sigv4a`, à `deny`. Une paire `awsPairs` n'est jamais transmise sous une forme qui peut re-signer ; une paire qui nomme l'une des variables AWS conventionnelles est remplacée par une entrée inerte qui maintient l'appairage automatique de `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, et `AWS_SESSION_TOKEN` supprimé.

<h4 id="deploy-the-locks">
  Déployer les verrous
</h4>

Pour garder les paramètres parents aussi proches de restriction-uniquement que le filtre le supporte, ajoutez les cinq verrous `allowManaged*Only`, et les listes d'autorisation qu'ils gouvernent, aux mêmes sources que l'opt-in de fusion :

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

Une politique du système d'exploitation, telle qu'une politique de registre HKLM ou une plist de préférences gérées, surclasse ce fichier, donc livrez l'extrait entier via celui-ci au lieu du fichier. Les paramètres gérés distants de la passerelle surclassent la politique du système d'exploitation et les sources de fichiers mais ne sont accessibles qu'aux clients connectés. Miroir les verrous, les listes d'autorisation, et l'opt-in de fusion dans le bloc [`cli`](/docs/fr/claude-apps-gateway-config#managed) de la politique et gardez ce fichier déployé, car les machines qui ne se connectent jamais, y compris celles qui exécutent uniquement Claude Desktop, obtiennent leur politique du fichier seul.

<h4 id="lock-behavior-across-sources">
  Comportement des verrous entre les sources
</h4>

Définir un verrou ne restreint pas les autres ; chaque clé est documentée dans la [référence des paramètres](/docs/fr/settings-reference#all-settings).

À partir d'une source d'administrateur en dessous du gagnant, les deux verrous sandbox s'appliquent toujours, et `allowManagedPermissionRulesOnly` bloque toujours les règles d'autorisation fournies par le parent et `additionalDirectories`. Sur Claude Code v2.1.273 ou ultérieur, le verrou serveur MCP s'applique également à partir d'une source en dessous du gagnant, et tandis qu'il est activé, la liste gérée `allowedMcpServers` provient de la source d'administrateur de plus haute priorité qui en définit une.

Les verrous hooks et `allowManagedPermissionRulesOnly`'s effet sur les règles propres du développeur ont besoin de la source gagnante par défaut ; sous l'opt-in de fusion `managedSourcesBehavior` dans [comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources), Claude Code applique la valeur la plus stricte que toute source définit pour chaque verrou. Sur les flottes [`policyHelper`](/docs/fr/settings-reference#policyhelper), Claude Code lit les verrous à partir de la sortie du helper seule.

Chaque verrou fait que Claude Code ignore les entrées propres du développeur pour ce paramètre, donc incluez les listes d'autorisation de votre organisation à côté des verrous :

* **Domaines réseau** : verrouiller avec une liste de domaines gérés vide bloque tout le trafic sortant en sandbox.
* **Serveurs MCP** : verrouiller sans `allowedMcpServers` gérés ou fournis par le parent charge chaque serveur que `deniedMcpServers` ne bloque pas.
* **Chemins de lecture** : les entrées `allowRead` ne réautorisent que les chemins à l'intérieur des régions `denyRead`, donc associez-les à un `denyRead` gérés.

<h4 id="settings-the-locks-don’t-cover">
  Paramètres que les verrous ne couvrent pas
</h4>

Six paramètres fournis par le parent passent le filtre même avec les cinq verrous définis. Sous le paramètre par défaut premier-gagnant, la valeur d'administrateur qui bloque celle du parent est celle dans la source d'administrateur de plus haute priorité, sauf pour `allowedMcpServers` tandis que le [verrou serveur MCP](#lock-behavior-across-sources) est activé. Sous l'opt-in de fusion `managedSourcesBehavior`, [comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) dit quelle valeur de source s'applique à la place.

* **`forceLoginOrgUUID`** : Claude Code honore une valeur fournie par le parent quand la source d'administrateur de plus haute priorité ne définit pas un UUID d'organisation. La connexion à la passerelle ne vérifie pas cette clé, donc elle ne compte que pour les flottes qui utilisent également les connexions Anthropic de première partie. Un UUID d'organisation dans la source d'administrateur de plus haute priorité bloque la valeur du parent et est celui que Claude Code applique, donc définissez `forceLoginOrgUUID` là.
* **`allowedMcpServers`** : Claude Code honore une liste d'autorisation fournie par le parent quand aucune liste d'administrateur n'est en vigueur. `allowManagedMcpServersOnly` ne la bloque pas, car le verrou applique quelle que soit la liste qui gagne comme valeur gérée, y compris une liste fournie par le parent quand aucune source d'administrateur n'en fournit une. Une liste dans la source d'administrateur de plus haute priorité bloque celle du parent et est la liste que Claude Code applique, donc définissez `allowedMcpServers` là, à côté du verrou. Avant v2.1.223, une valeur pour l'une ou l'autre clé dans toute source d'administrateur bloquait celle du parent.
* **`availableModels`** : Claude Code honore une liste de modèles fournie par le parent quand la source gérée gagnante n'en définit pas. Si votre flotte restreint les modèles, définissez `availableModels` dans la source gagnante.
* **`strictKnownMarketplaces`** : Claude Code honore une liste d'autorisation de marché de plugins fournie par le parent quand la source gérée gagnante n'en définit pas. Si votre flotte restreint les marchés, définissez `strictKnownMarketplaces` dans la source gagnante. Nécessite Claude Code v2.1.282 ou ultérieur.
* **`blockedMarketplaces`** : une liste de blocage de marché fournie par le parent passe et s'ajoute à toute liste de blocage qu'une source gérée définit, puisqu'une liste de blocage ne peut que restreindre davantage. Nécessite Claude Code v2.1.282 ou ultérieur.
* **`strictPluginOnlyCustomization`** : cette clé passe le filtre indépendamment de tout verrou, et elle fait que Claude Code ignore la personnalisation propre du développeur, y compris les hooks protecteurs. Aucun verrou ne la bloque.

<h3 id="connect-claude-desktop">
  Connecter Claude Desktop
</h3>

[Claude Desktop](/docs/fr/desktop) se connecte à la même passerelle via une clé MDM différente : définissez `bootstrapUrl` dans la [configuration gérée](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop à `<listen.public_url>/user/bootstrap`, et optez la politique de l'utilisateur avec une clé `desktop`. [La superposition Claude Desktop](/docs/fr/claude-apps-gateway-config#claude-desktop-overlay) couvre les deux moitiés. Nécessite Claude Code v2.1.203 ou ultérieur sur le serveur de passerelle.

Claude Desktop connecte le développeur via le fournisseur d'identité de la passerelle avec la même étape SSO du navigateur, puis récupère sa configuration auprès de la passerelle au lieu d'Anthropic. L'accès au modèle et la politique suivent les mêmes règles par groupe que l'interface de ligne de commande. Un développeur qui utilise à la fois l'interface de ligne de commande et Claude Desktop se connecte à chacun séparément ; la session de passerelle n'est pas partagée entre eux.

Une fois connecté, Claude Desktop envoie les demandes de modèle de chaque onglet activé via la passerelle. Il affiche les onglets Cowork et Code par défaut. Pour activer également l'onglet Chat, définissez `chatTabEnabled` à `true` dans la [configuration gérée](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop, ou dans le bloc [`desktop`](/docs/fr/claude-apps-gateway-config#claude-desktop-overlay) de la politique sur une passerelle exécutant Claude Code v2.1.227 ou ultérieur.

<h3 id="ci-pipelines-and-remote-machines">
  Pipelines CI et machines distantes
</h3>

Il n'y a pas de flux de jeton de service pour les pipelines sans surveillance. La connexion à la passerelle exécute toujours le flux d'appareil du navigateur, donc un travail CI sans développeur pour approuver la connexion ne peut pas s'authentifier ; configurez ceux-ci directement contre votre fournisseur.

Une fois qu'un développeur s'est connecté, chaque session Claude Code sur cette machine utilise la session de passerelle, y compris les exécutions non-interactives `claude -p` et les sessions démarrées par le SDK Agent. Claude Code applique la [politique de passerelle](/docs/fr/claude-apps-gateway-config#managed) à chacune d'elles.

Le flux d'appareil sépare l'interface de ligne de commande d'interrogation du navigateur approbateur, donc une boîte de développement distante sans affichage fonctionne toujours : le développeur exécute `/login` via SSH sur la machine distante et ouvre le lien de vérification dans le navigateur sur son ordinateur portable.

<h3 id="whats-enforced-on-developers">
  Ce qui est appliqué aux développeurs
</h3>

Ces garanties s'appliquent à chaque session connectée via `/login`. Les sessions intégrées que Claude Desktop lance obtiennent leur politique comme décrit dans [Livrer la politique à Claude Desktop sessions](#deliver-policy-to-claude-desktop-sessions), et la puce de télémétrie dit où vont leurs exports.

* **Accès au modèle** : les demandes pour les modèles que la politique n'accorde pas retournent 400, et le sélecteur `/model` est filtré à la liste d'autorisation `availableModels` de la politique. Définissez [`enforceAvailableModels: true`](/docs/fr/model-config#default-model-behavior) dans la politique pour que l'option Par défaut se résolve à un modèle à l'intérieur de `availableModels` au lieu du défaut intégré de Claude Code ; sans cela, Par défaut reste sélectionnable et est rejeté au moment de la demande si ce modèle n'est pas accordé.
* **Destination de télémétrie** : dans les sessions connectées via `/login`, l'interface de ligne de commande envoie ses exports OTLP/HTTP à la passerelle plutôt qu'à un `OTEL_EXPORTER_OTLP_ENDPOINT` défini localement, sauf si une politique [nomme votre collecteur comme point de terminaison](/docs/fr/claude-apps-gateway-config#export-directly-to-your-collector). La passerelle relaie les exports qu'elle reçoit vers les destinations dans [`telemetry.forward_to`](/docs/fr/claude-apps-gateway-config#telemetry).
  * Dans les sessions intégrées que [Claude Desktop lance](#connect-claude-desktop), l'interface de ligne de commande envoie ses exports au `OTEL_EXPORTER_OTLP_ENDPOINT` configuré. L'interface de ligne de commande attache le jeton de session de passerelle à ces exports uniquement quand ce point de terminaison pointe vers la passerelle elle-même.
  * Sans destination configurée pour un signal, la passerelle l'accepte et le rejette.
  * Si vous collectez déjà la télémétrie Claude Code directement, ajoutez votre collecteur comme destination `forward_to`, ou nommez-le dans une politique pour ignorer le relais.
* **Identifiants** : le jeton de passerelle est le seul identifiant de la session. [Les profils Anthropic](/docs/fr/authentication#anthropic-profiles-and-federation-credentials) et toute connexion claude.ai antérieure sont ignorés lors de la connexion, donc les développeurs n'ont pas besoin de se déconnecter de claude.ai d'abord. Pour un identifiant `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, ou `apiKeyHelper` configuré, voir [La politique d'administrateur nécessite une connexion Cloud gateway](/docs/fr/errors#administrator-policy-requires-a-cloud-gateway-sign-in).
* **Paramètres gérés** : les clés verrouillées ne peuvent pas être remplacées localement. L'interface de ligne de commande applique la politique au démarrage et applique les modifications à chaque sondage horaire, à part les [modifications qui s'appliquent uniquement au prochain lancement](/docs/fr/server-managed-settings#fetch-and-caching-behavior).
* **Démarrage avec la passerelle inaccessible** : les sessions connectées se terminent au démarrage avec une erreur après environ 10 secondes plutôt que de démarrer sans leurs paramètres.
* **Démarrage après que la passerelle termine la session** : voir [Appliquer un démarrage fail-closed](/docs/fr/server-managed-settings#enforce-fail-closed-startup) pour les lancements qui s'ouvrent déconnectés de la passerelle et ceux qui se terminent quand la passerelle répond avec un `401`.
* **Déprovisionnement** : une session dont l'utilisateur est désactivé dans l'IdP expire dans `ttl_hours` quand le prochain rafraîchissement échoue.
* **Déconnexion** : `/logout` supprime l'identifiant de passerelle de la machine du développeur.
  * Quand le document de découverte de la passerelle annonce un `revocation_endpoint` sur le schéma, l'hôte et le port propres de l'URL de la passerelle, `/logout` envoie également les jetons stockés à ce point de terminaison pour que la passerelle puisse terminer la session de son côté. La demande est au mieux effort, donc la déconnexion se complète sur la machine du développeur que le point de terminaison réponde ou non. La révocation nécessite Claude Code v2.1.275 ou ultérieur sur la machine du développeur.
  * Le serveur de passerelle dans le binaire `claude` n'en annonce aucun, donc une déconnexion de celui-ci termine la session sur la machine du développeur uniquement. Pour forcer les sessions à sortir côté serveur, voir [Rotation de secret JWT](/docs/fr/claude-apps-gateway-deploy#jwt-secret-rotation).

<h3 id="what-the-organization-can-see">
  Ce que l'organisation peut voir
</h3>

La télémétrie d'utilisation porte l'identité du développeur, les nombres de jetons, le modèle et la latence vers le collecteur de l'organisation. La passerelle ne journalise ni ne stocke le contenu des invites ou des complétions. Que la télémétrie plus riche comme les journaux et les traces soit collectée, qui peut inclure les commandes et les chemins de fichiers, est le [choix par destination](/docs/fr/claude-apps-gateway-config#telemetry) de l'organisation.

<h2 id="availability-and-limitations">
  Disponibilité et limitations
</h2>

Le tableau couvre les fonctionnalités Claude Code qui fonctionnent quand les développeurs se connectent via la passerelle, et ce que le serveur de passerelle lui-même supporte. Quand quelque chose n'est pas supporté, la colonne Notes donne l'alternative.

La passerelle livre les valeurs [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) que l'interface de ligne de commande envoie à chaque amont, donc les opérateurs ne maintiennent pas une liste d'autorisation bêta. Pour Amazon Bedrock, qui ignore l'en-tête, la passerelle déplace les valeurs dans le champ `anthropic_beta` du corps de la demande ; les autres ammonts reçoivent l'en-tête tel qu'envoyé.

| Fonctionnalité                                                                                                               | Statut                 | Notes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ---------------------------------------------------------------------------------------------------------------------------- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Transfert d'inférence (Amazon Bedrock, Claude Platform on AWS, Agent Platform de Google Cloud, Microsoft Foundry, Anthropic) | Disponible             | Avec traduction de modèle par amont et basculement. L'amont Amazon Bedrock utilise le point de terminaison `bedrock-runtime` et la chaîne de credentials AWS par défaut ; le [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint) d'Amazon Bedrock n'est pas un amont supporté. L'[amont Claude Platform on AWS](/docs/fr/claude-apps-gateway-config#claude-platform-on-aws) nécessite Claude Code v2.1.198 ou ultérieur sur le serveur de passerelle.                                |
| Accès au modèle et paramètres gérés par groupe IdP                                                                           | Disponible             | L'accès au modèle est appliqué côté serveur ; les paramètres gérés sont livrés par groupe IdP et appliqués par l'interface de ligne de commande au [niveau des paramètres gérés](/docs/fr/settings#settings-precedence)                                                                                                                                                                                                                                                                                    |
| Claude Desktop                                                                                                               | Disponible avec opt-in | La passerelle sert la configuration de Claude Desktop à `/user/bootstrap` une fois qu'une politique [opte pour une clé `desktop`](/docs/fr/claude-apps-gateway-config#claude-desktop-overlay), et Claude Desktop envoie les demandes de modèle de ses onglets Cowork et Code, et de l'onglet Chat quand vous l'activez, via la passerelle. Pour activer l'onglet Chat, voir [Connecter Claude Desktop](#connect-claude-desktop). Nécessite Claude Code v2.1.203 ou ultérieur sur le serveur de passerelle. |
| Fan-out de télémétrie (OTLP/HTTP)                                                                                            | Disponible             | Identité-estampillé par export ; encodages protobuf et JSON                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Fournisseurs d'identité OIDC                                                                                                 | Disponible             | Tout IdP conforme à OIDC ; la passerelle exécute la découverte OIDC standard et le flux du code d'autorisation. Voir [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup) pour la configuration par IdP                                                                                                                                                                                                                                                  |
| Limites de dépenses par utilisateur et par groupe                                                                            | Disponible             | Voir [Limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Recherche web côté serveur                                                                                                   | Non disponible         | L'interface de ligne de commande ne peut pas voir quel fournisseur en amont la passerelle achemine vers, donc elle ne peut pas vérifier le support de la recherche web et désactive WebSearch sur les sessions de passerelle                                                                                                                                                                                                                                                                          |
| [Contrôle à distance](/docs/fr/remote-control)                                                                                    | Non disponible         | L'interface de ligne de commande affiche [une erreur nommant la passerelle](/docs/fr/errors#remote-control-requires-the-anthropic-api)                                                                                                                                                                                                                                                                                                                                                                     |
| [`/design-sync`](/docs/fr/commands#all-commands) et `/design-login`                                                               | Non disponible         | Les deux ont besoin de claude.ai, que l'interface de ligne de commande ne contacte pas sur les sessions de passerelle, donc aucune des deux commandes n'y apparaît                                                                                                                                                                                                                                                                                                                                    |
| Fonctionnalités qui nécessitent la récupération de drapeaux de fonctionnalité, comme `/import` et `claude import`            | Non disponible         | L'interface de ligne de commande ignore la récupération de drapeau sur les sessions de passerelle. [Les fonctionnalités qui nécessitent la récupération de drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching) énumère ce que cela désactive                                                                                                                                                                                                                           |
| Mise en cache des invites standard                                                                                           | Disponible             | La passerelle transfère les points d'arrêt `cache_control` à chaque amont. [Où le cache réside](/docs/fr/prompt-caching#where-the-cache-lives) couvre les blocs que l'interface de ligne de commande marque, y compris le contexte système qu'elle ajoute en milieu de conversation                                                                                                                                                                                                                        |
| TTL de cache d'1 heure                                                                                                       | Non disponible         | L'interface de ligne de commande omet la bêta extended-cache-ttl sur les sessions de passerelle, car pas tous les ammonts vers lesquels la passerelle peut acheminer supportent le TTL d'1 heure, donc la mise en cache des invites via la passerelle utilise le TTL de 5 minutes ; voir la note sur l'en-tête bêta ci-dessus                                                                                                                                                                         |
| Mode Auto                                                                                                                    | Disponible             | Suit les [règles du fournisseur tiers](/docs/fr/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry) : seuls les modèles éligibles sur les fournisseurs tiers peuvent l'utiliser. Avant v2.1.207, le mode auto sur les sessions de passerelle nécessitait de définir `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, livrable via le bloc `env` de la politique gérée                                                                                                                                 |
| Optimisations propriétaires uniquement comme la portée du cache global et les outils efficaces en jetons                     | Non disponible         | L'interface de ligne de commande ne les active pas sur les sessions de passerelle ; voir la note sur l'en-tête bêta ci-dessus                                                                                                                                                                                                                                                                                                                                                                         |
| OTLP/gRPC                                                                                                                    | Non supporté           | OTLP sur HTTP uniquement                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| SAML, LDAP et autres authentifications non-OIDC                                                                              | Non supporté           | OIDC uniquement. Frontal avec un pont OIDC si nécessaire                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Multi-locataire (plusieurs émetteurs OIDC)                                                                                   | Non supporté           | Un émetteur par passerelle. Exécutez des instances séparées                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Serveur Windows                                                                                                              | Non supporté           | Déployez sur Linux. macOS pour le développement local uniquement                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Graphique Helm                                                                                                               | Non disponible         | La passerelle s'exécute comme un Deployment sans état standard ; voir le [guide de déploiement](/docs/fr/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                                                                            |
| Interface utilisateur d'administration                                                                                       | Non disponible         | La configuration est le fichier YAML ; redéployez pour le modifier                                                                                                                                                                                                                                                                                                                                                                                                                                    |

<h2 id="next-steps">
  Prochaines étapes
</h2>

Le démarrage rapide vous laisse avec une configuration minimale s'exécutant sous Docker Compose. Pour aller plus loin :

* Développez `gateway.yaml` au-delà de la configuration minimale, par exemple pour ajouter RBAC par groupe, basculement multi-amont, ou destinations de télémétrie. La [référence de configuration](/docs/fr/claude-apps-gateway-config) couvre chaque option.
* Passez de Compose à un déploiement de production sur Kubernetes ou Cloud Run, configurez correctement votre IdP, et examinez le modèle de sécurité. Le [guide de déploiement et d'opérations](/docs/fr/claude-apps-gateway-deploy) couvre la configuration par IdP, les exigences d'image de conteneur, les sondes de santé et le dépannage.
* Mettez des plafonds de dépenses sur les développeurs individuels ou les groupes pour qu'une charge de travail incontrôlée ne puisse pas consommer tout votre engagement. [Limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits) couvre l'API d'administration et le fonctionnement de l'application.
* Pour un exemple complet travaillé sur AWS, avec ECS Fargate ou EKS, Amazon RDS et Secrets Manager, voir [Déployer sur AWS](/docs/fr/claude-apps-gateway-on-aws).
* Pour un exemple complet travaillé sur Google Cloud, avec Cloud Run, Cloud SQL et Secret Manager, voir [Déployer sur Google Cloud](/docs/fr/claude-apps-gateway-on-gcp).
