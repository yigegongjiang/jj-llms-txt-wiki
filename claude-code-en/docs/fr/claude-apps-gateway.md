> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Passerelle Claude apps pour Amazon Bedrock, Claude Platform sur AWS, Google Cloud et Microsoft Foundry

> Exécutez Claude Code via Amazon Bedrock, Claude Platform sur AWS, Google Cloud ou Microsoft Foundry derrière une passerelle auto-hébergée avec authentification SSO, accès aux modèles par groupe et télémétrie OTLP.

<Note>
  La passerelle Claude apps est conçue pour les organisations qui doivent — ou préfèrent — acheminer l'inférence via leur propre fournisseur cloud, par exemple pour respecter les exigences de [résidence des données](/docs/fr/claude-apps-gateway-deploy#compliance-posture). Si vous n'avez pas cette exigence et souhaitez accéder à d'autres fonctionnalités telles que l'approvisionnement SCIM ou Claude Code sur web et mobile, Claude Enterprise peut être un meilleur choix. Consultez la page de [disponibilité des fonctionnalités](/docs/fr/feature-availability) pour une comparaison complète de toutes les méthodes de déploiement.
</Note>

Claude apps gateway est un service auto-hébergé qui se situe entre les clients Claude Code de vos développeurs et votre fournisseur de modèles. Les développeurs se connectent avec votre fournisseur d'identité (IdP) d'entreprise au lieu de détenir des clés API ou des identifiants cloud. La passerelle détient les identifiants en amont, applique l'accès aux modèles et les [paramètres gérés](/docs/fr/permissions#managed-settings) par groupe IdP, et relaye la télémétrie d'utilisation vers votre propre pile d'observabilité.

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
* **Contrôle d'accès** : vos groupes IdP correspondent à des listes d'autorisation de modèles et à des politiques de [paramètres gérés](/docs/fr/permissions#managed-settings). La passerelle applique l'accès aux modèles côté serveur, rejetant les demandes pour les modèles non accordés, et sélectionne la politique de paramètres gérés de chaque groupe, que l'interface de ligne de commande applique au [niveau des paramètres gérés](/docs/fr/settings#settings-precedence). Différentes équipes obtiennent différents modèles, outils et permissions, et un développeur ne peut pas remplacer ce que sa politique verrouille.
* **Livraison des paramètres** : la passerelle livre les paramètres gérés aux clients connectés elle-même, remplaçant les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) de la console d'administration claude.ai.
* **Télémétrie** : chaque destination configurée, comme Datadog, Splunk ou ClickHouse, reçoit les [métriques OpenTelemetry Protocol (OTLP)](/docs/fr/monitoring-usage) avec les nombres de jetons, le modèle, l'identité de l'utilisateur et la latence par défaut, avec les journaux et les traces comme des options par destination.
* **Routage en amont** : les clients parlent l'API Anthropic Messages à la passerelle, et la passerelle traduit pour chaque amont, qu'il s'agisse de Bedrock, de la [plateforme Claude sur AWS](/docs/fr/claude-platform-on-aws), de la plateforme Agent de Google Cloud, de Foundry ou de l'API Anthropic, avec basculement entre eux. Vous pouvez modifier les régions, les fournisseurs ou l'ordre de basculement sans que les développeurs le remarquent ou se reconfigurent.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagramme montrant les clients Claude Code se connectant via HTTPS avec des jetons porteurs à une passerelle Claude apps auto-hébergée dans votre infrastructure, qui connecte les utilisateurs à votre IdP, stocke l'état d'authentification dans PostgreSQL, relaye la télémétrie à votre collecteur OTLP et transmet l'inférence à Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry ou l'API Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  Le plan de données de la passerelle n'envoie rien à l'infrastructure Anthropic sauf si l'API Anthropic est un amont configuré. Vous contrôlez où la télémétrie, les journaux d'audit, les paramètres gérés et l'identité IdP de vos développeurs vont, et la passerelle ne les envoie à Anthropic. Pour le trafic restant que le processus CLI peut envoyer et comment le fermer, voir [Posture de conformité](/docs/fr/claude-apps-gateway-deploy#compliance-posture).
</Note>

Pour les fonctionnalités Claude Code qui fonctionnent via la passerelle et ce que le serveur lui-même supporte, voir [Disponibilité et limitations](#availability-and-limitations) ci-dessous. Pour les décisions telles que le coût, le contournement, l'exécution de plusieurs passerelles et les plates-formes sans serveur, voir le [guide de déploiement](/docs/fr/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Autres implémentations de passerelle
</h3>

Si vous exécutez déjà une passerelle LLM ou une passerelle API qui répond à vos besoins, continuez à l'utiliser ; [Autres passerelles LLM](/docs/fr/llm-gateway) couvre la configuration de Claude Code contre elle.

La [référence du protocole de passerelle](/docs/fr/llm-gateway-protocol) documente le contrat que Claude Code attend de toute passerelle : les points de terminaison qu'elle appelle, les en-têtes et les champs de corps à transmettre, et ce qui cesse de fonctionner quand ils sont supprimés. Une passerelle Claude apps en cours d'exécution sert un sur-ensemble de ce contrat à `GET /protocol`, ajoutant les points de terminaison spécifiques à Claude apps gateway pour la connexion SSO, la livraison des paramètres gérés et la télémétrie. Récupérez-le avec `curl https://claude-gateway.internal.example.com/protocol` à partir de n'importe quelle passerelle déployée, comme celle que le [démarrage rapide](#quickstart) ci-dessous produit.

Les modifications majeures du protocole sont annoncées à l'avance, mais la compatibilité rétroactive indéfinie n'est pas garantie.

<h2 id="quickstart">
  Démarrage rapide
</h2>

Ce démarrage rapide parcourt le chemin minimal : enregistrez un client OAuth dans votre IdP, écrivez un `gateway.yaml`, exécutez la passerelle aux côtés de Postgres avec Docker Compose, et vérifiez la connexion de bout en bout. Il utilise un amont Amazon Bedrock ; Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry, et l'API Anthropic sont également supportés en échangeant le bloc `upstreams` comme indiqué dans la [référence de configuration](/docs/fr/claude-apps-gateway-config#upstreams). À la fin, vous avez une passerelle à laquelle un développeur peut se `/login`.

<Note>
  **Déployez sur votre réseau privé.** Claude Code ne se connecte qu'à une passerelle dont l'adresse est privée. C'est un garde de sécurité, car une passerelle de confiance peut pousser des paramètres qui exécutent des commandes sur les machines des développeurs. Mettez la passerelle derrière un équilibreur de charge interne ou un VPN et donnez-lui un nom d'hôte qui ne se résout qu'à des adresses IP privées.

  Les points de terminaison de passerelle publique exploités par Anthropic sont l'exception : `/login` les accepte sur `https://`. Il s'agit d'un petit ensemble fixe de passerelles qu'Anthropic elle-même exploite ; ce ne sont pas une option de déploiement que vous pouvez sélectionner ou configurer. La liste est compilée dans Claude Code, donc aucune configuration ne peut ajouter un nom d'hôte à celle-ci et aucune passerelle que vous hébergez ne se qualifie pour l'exemption. Avant v2.1.206, `/login` rejetait ces points de terminaison comme toute autre adresse publique.
</Note>

<h3 id="prerequisites">
  Prérequis
</h3>

Ayez ceci en place avant de commencer :

| Vous avez besoin                             | Détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 ou ultérieur            | La sous-commande `claude gateway` et le flux de connexion de la passerelle sont livrés dans v2.1.195. Les versions publiques antérieures ne les incluent pas. La machine exécutant le serveur de passerelle et la machine de chaque développeur doivent être sur v2.1.195 ou ultérieur ; exécutez `claude update` pour obtenir la dernière version. L'[amont Claude Platform on AWS](/docs/fr/claude-apps-gateway-config#claude-platform-on-aws) nécessite Claude Code v2.1.198 ou ultérieur sur le serveur de passerelle.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Fournisseur d'identité OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak, ou Dex, ou tout autre IdP conforme à OIDC comme PingFederate. La passerelle exécute la découverte OIDC standard et le flux de code d'autorisation contre elle. SAML et LDAP ne sont pas supportés.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| PostgreSQL 14 ou ultérieur                   | Soutient le flux de connexion d'appareil, où le rappel du navigateur écrit et l'interface de ligne de commande d'interrogation lit, plus les compteurs de limite de débit. Tout Postgres géré fonctionne, y compris le plus petit niveau. Sans limites de dépenses configurées, la passerelle stocke quelques Ko d'état d'authentification de courte durée ; avec les [limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits), elle détient également des tables de dépenses durables, d'audit et d'identité qui doivent être sauvegardées. TLS via `?sslmode=require` est recommandé.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Amont du modèle                              | Identifiants Amazon Bedrock, identifiants Claude Platform on AWS, identifiants Google Cloud, une ressource Microsoft Foundry, ou une clé API Anthropic. Plusieurs ammonts sont supportés avec basculement.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| HTTPS                                        | La passerelle doit être accessible via `https://` à partir des ordinateurs portables des développeurs et de tout navigateur utilisé pour la connexion ; la passerelle sert la page de vérification d'appareil sur le même écouteur. Fournissez un certificat TLS via `listen.tls`, ou exécutez derrière un ingress qui termine TLS et définissez `listen.public_url`. Une origine `http://` simple est acceptée uniquement sur la boucle locale, pour le développement local.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Adresse de réseau privé                      | À `/login`, Claude Code exige que le nom d'hôte ou l'adresse IP de la passerelle ne se résolve qu'à des adresses privées : RFC 1918, CGNAT `100.64.0.0/10`, ULA IPv6 `fc00::/7`, ou boucle locale pour le développement local. La vérification s'exécute sur chaque adresse IP résolue, donc si une adresse à laquelle le nom se résout est publique, `/login` rejette l'URL. Si les machines des développeurs acheminent HTTPS via un proxy d'entreprise, la connexion exige également que l'hôte proxy se résolve à des adresses privées ; s'il ne le fait pas, ajoutez l'hôte de la passerelle à `NO_PROXY` pour que l'interface de ligne de commande se connecte directement. Les points de terminaison de passerelle exploités par Anthropic sont exemptés des vérifications d'adresse privée et de proxy : `/login` les accepte sur `https://` par correspondance exacte du nom d'hôte, donc l'exigence de réseau privé s'applique uniquement à une passerelle que vous hébergez vous-même. Avant v2.1.206, `/login` rejetait un point de terminaison exploité par Anthropic comme toute autre adresse publique. |
| Runtime Linux                                | Le serveur de passerelle s'exécute uniquement sur le binaire Linux natif. macOS fonctionne pour le développement local. Windows n'est pas supporté comme plate-forme serveur.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

Le serveur de passerelle nécessite le binaire `claude` natif ; téléchargez une version épinglée comme décrit dans [Installer Claude Code](/docs/fr/setup). Le serveur utilise des fonctionnalités d'exécution qui ne sont pas disponibles quand Claude Code s'exécute sous Node. Si vous voyez `requires the native binary` au démarrage, passez à l'une des méthodes d'installation autonomes.

<h3 id="steps">
  Étapes
</h3>

<Steps>
  <Step title="Enregistrez un client OAuth dans votre IdP">
    Décidez d'abord du nom d'hôte de la passerelle, car l'URI de redirection doit le correspondre. Créez une nouvelle application web OIDC et définissez l'URI de redirection sur `https://claude-gateway.<your-domain>/oauth/callback`, où l'hôte est la même valeur que vous définissez comme [`listen.public_url`](/docs/fr/claude-apps-gateway-config#listen) à l'étape 3. Notez le `client_id` et le `client_secret`. Les instructions par IdP sont dans [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Provisionner une base de données PostgreSQL">
    Tout Postgres 14 ou ultérieur fonctionne, y compris le plus petit niveau géré. La passerelle exécute ses propres migrations de schéma au démarrage, donc l'utilisateur de la base de données a besoin de la permission `CREATE TABLE`. Si votre politique de sécurité interdit DDL à partir de rôles d'application, pré-créez le schéma à la place ; voir [`store`](/docs/fr/claude-apps-gateway-config#store).
  </Step>

  <Step title="Écrivez gateway.yaml">
    Les secrets sont lus via l'expansion `${ENV_VAR}` pour que le fichier lui-même puisse vivre dans le contrôle de version. Utilisez un nom d'hôte `public_url` qui se résout à une adresse IP privée sur votre réseau, car `/login` rejette les adresses publiques. La configuration minimale a cinq sections, et tous les autres champs ont une valeur par défaut :

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Requis derrière tout proxy qui termine TLS. Utilisé pour l'IdP
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
      L'amont Bedrock a besoin d'un principal AWS avec `bedrock:InvokeModel` et `bedrock:InvokeModelWithResponseStream` sur les ARNs `inference-profile/us.anthropic.*` et les ARNs `foundation-model/anthropic.*` sous-jacents, et l'accès au modèle activé dans la console Bedrock pour les modèles Claude que vous voulez. Fournissez l'identifiant avec IRSA sur EKS, un rôle de tâche ECS, ou un profil d'instance EC2 plutôt que des clés statiques. La [référence `upstreams`](/docs/fr/claude-apps-gateway-config#upstreams) a les détails IAM complets, la matrice de credentials inter-cloud, et les blocs `auth` pour les autres fournisseurs.
    </Note>
  </Step>

  <Step title="Exécutez-le">
    Construisez une image de conteneur autour du binaire `claude` qui répond aux [exigences d'image](/docs/fr/claude-apps-gateway-deploy#container-image), puis exécutez-la aux côtés de Postgres :

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
          # Identifiants AWS : en production, omettez ceux-ci et utilisez un rôle d'instance.
          # Pour les tests Compose locaux, transmettez les vôtres :
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

    La passerelle est un binaire Linux unique qui lit la configuration, exécute la découverte OIDC contre votre IdP, applique ses migrations de schéma Postgres, construit les clients en amont, et commence à écouter. Le démarrage échoue fermé pour la configuration, la connexion Postgres avec un délai d'attente de 5 secondes, la découverte OIDC, et la construction du client en amont. Si l'un de ceux-ci est inaccessible ou mal configuré, la passerelle se termine avec une erreur plutôt que de servir le trafic dans un état dégradé.

    Un démarrage réussi ne valide pas le chemin d'inférence, car les identifiants d'instance Bedrock et Agent Platform se résolvent à la première demande, pas au démarrage.

    Regardez stderr pour la séquence de démarrage. Les lignes de journal utilisent le format `[gateway] <timestamp> <level> <message>`, les événements d'audit sont JSON sur une seule ligne avec un champ `evt`, et une bannière de démarrage, omise ci-dessous, s'imprime entre les lignes de migration et d'écoute. Vous devriez voir, dans l'ordre :

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

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
    Cette dernière étape se produit sur une machine de développeur, pas le serveur. Définissez `forceLoginMethod` sur `"gateway"` et `forceLoginGatewayUrl` sur l'URL `public_url` de votre passerelle dans le [fichier de paramètres gérés](/docs/fr/settings#settings-files) de cette machine, puis exécutez `/login`, appuyez sur Entrée sur l'écran **Cloud gateway**, et complétez la connexion du navigateur. [Définir l'URL de la passerelle](#set-the-gateway-url) ci-dessous couvre la distribution des deux clés à grande échelle.
  </Step>
</Steps>

<h2 id="connect-developers">
  Connecter les développeurs
</h2>

Les développeurs se connectent à partir de leurs propres ordinateurs portables avec une seule connexion au navigateur, en utilisant leur compte professionnel d'entreprise. Ils n'ont pas besoin d'un compte claude.ai, d'une clé API ou d'un abonnement, car les demandes au modèle passent par la passerelle en utilisant l'identifiant en amont de l'organisation. La connexion est pilotée par les [paramètres gérés côté client](/docs/fr/claude-apps-gateway-config#client-side-managed-settings) que vous poussez via MDM, il n'y a donc pas de configuration manuelle du côté du développeur ; cette section couvre ce que l'administrateur configure.

L'interface de ligne de commande empreinte le certificat feuille TLS de la passerelle à la première connexion et l'épingle par nom d'hôte. Publiez l'empreinte SHA-256 attendue aux côtés de l'URL de la passerelle pour que les développeurs aient quelque chose à comparer. Obtenez l'empreinte du fichier de certificat avec `openssl x509 -noout -fingerprint -sha256 -in cert.pem` ; l'invite `/login` affiche les 16 premiers caractères du résumé en hexadécimal minuscule sans séparateurs.

Quand le certificat tourne, chaque développeur voit à nouveau l'invite de confiance, traitez donc les rotations comme un événement planifié et republier l'empreinte.

Une fois connecté, le [sélecteur de modèle](/docs/fr/model-config) affiche les modèles dans la liste d'autorisation `availableModels` du développeur, les paramètres gérés s'appliquent au démarrage et se rafraîchissent toutes les heures, et la télémétrie s'achemine vers votre collecteur. Les sessions se rafraîchissent silencieusement avant l'expiration de `ttl_hours`, et un rafraîchissement échoué après le déprovisionnement IdP invite à une reconnexion.

<h3 id="set-the-gateway-url">
  Définir l'URL de la passerelle
</h3>

Définissez les deux clés dans le fichier de [paramètres gérés](/docs/fr/settings#settings-files) par système d'exploitation que vous déployez via MDM ou directement sur le disque, et `/login` s'ouvre directement sur l'écran **Cloud gateway** avec l'URL remplie :

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Le développeur appuie sur Entrée pour se connecter. L'invite d'empreinte TLS de première connexion apparaît toujours.

Il n'y a pas d'option de passerelle dans le sélecteur de connexion pour qu'un développeur sélectionne manuellement, et `forceLoginGatewayUrl` est ignoré dans les fichiers de paramètres propres d'un développeur. `forceLoginMethod` seul, sans URL, laisse le développeur à un message « Contactez votre administrateur informatique ». Les deux clés appartiennent au fichier que vous poussez vers les machines, pas au bloc `managed.policies[].cli` de la passerelle, qui ne atteint que les clients déjà connectés.

<h3 id="ci-pipelines-and-remote-machines">
  Pipelines CI et machines distantes
</h3>

Il n'y a pas de flux de jeton de service pour les pipelines sans surveillance. La connexion à la passerelle exécute toujours le flux d'appareil du navigateur, donc un travail CI sans développeur pour approuver la connexion ne peut pas s'authentifier ; configurez ceux-ci directement contre votre fournisseur.

Une fois qu'un développeur s'est connecté, chaque invocation de Claude Code sur cette machine utilise la session de passerelle, y compris les exécutions non-interactives `claude -p` et les sessions démarrées par le SDK Agent, et la [politique de passerelle s'applique à tous](/docs/fr/claude-apps-gateway-config#managed).

Le flux d'appareil sépare l'interface de ligne de commande d'interrogation du navigateur approbateur, donc une boîte de développement distante sans affichage fonctionne toujours : le développeur exécute `/login` via SSH sur la machine distante et ouvre le lien de vérification dans le navigateur sur son ordinateur portable.

<h3 id="what’s-enforced-on-developers">
  Ce qui est appliqué aux développeurs
</h3>

Ces garanties s'appliquent à chaque session de passerelle connectée.

* **Accès au modèle** : les demandes pour les modèles que la politique n'accorde pas retournent 400, et le sélecteur `/model` est filtré à la liste d'autorisation `availableModels` de la politique. Définissez [`enforceAvailableModels: true`](/docs/fr/model-config#default-model-behavior) dans la politique pour que l'option Par défaut se résolve à un modèle à l'intérieur de `availableModels` au lieu du défaut intégré de Claude Code ; sans cela, Par défaut reste sélectionnable et est rejeté au moment de la demande si ce modèle n'est pas accordé.
* **Destination de télémétrie** : quand le [transfert de télémétrie](/docs/fr/claude-apps-gateway-config#telemetry) est configuré, le point de terminaison d'export OTLP est épinglé à la passerelle, et la configuration poussée par la passerelle remplace les variables `OTEL_*` définies localement.
* **Identifiants** : le jeton de passerelle est le seul identifiant de la session. `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, `apiKeyHelper`, et toute connexion claude.ai antérieure sont ignorés lors de la connexion, donc les développeurs n'ont pas besoin de se déconnecter de claude.ai d'abord.
* **Paramètres gérés** : les clés verrouillées ne peuvent pas être remplacées localement. L'interface de ligne de commande applique la politique au démarrage et à chaque sondage horaire.
* **Démarrage** : les sessions connectées se terminent au démarrage avec une erreur après environ 10 secondes quand la passerelle est inaccessible, plutôt que de démarrer sans leurs paramètres.
* **Déprovisionnement** : une session dont l'utilisateur est désactivé dans l'IdP expire dans `ttl_hours` quand le prochain rafraîchissement échoue.

<h3 id="what-the-organization-can-see">
  Ce que l'organisation peut voir
</h3>

La télémétrie d'utilisation porte l'identité du développeur, les nombres de jetons, le modèle et la latence vers le collecteur de l'organisation. La passerelle ne journalise ni ne stocke le contenu des invites ou des complétions. Que la télémétrie plus riche comme les journaux et les traces soit collectée, qui peut inclure les commandes et les chemins de fichiers, est le [choix par destination](/docs/fr/claude-apps-gateway-config#telemetry) de l'organisation.

<h2 id="availability-and-limitations">
  Disponibilité et limitations
</h2>

Le tableau couvre les fonctionnalités Claude Code qui fonctionnent quand les développeurs se connectent via la passerelle, et ce que le serveur de passerelle lui-même supporte. Quand quelque chose n'est pas supporté, la colonne Notes donne l'alternative.

La passerelle livre les valeurs [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) que l'interface de ligne de commande envoie à chaque amont, donc les opérateurs ne maintiennent pas une liste d'autorisation bêta. Pour Amazon Bedrock, qui ignore l'en-tête, la passerelle déplace les valeurs dans le champ `anthropic_beta` du corps de la demande ; les autres ammonts reçoivent l'en-tête tel qu'envoyé.

L'ensemble bêta de session de passerelle de l'interface de ligne de commande omet les bêtas propriétaires uniquement et la bêta extended-cache-ttl, c'est pourquoi ces lignes ci-dessous s'affichent comme non disponibles.

| Fonctionnalité                                                                                                               | Statut         | Notes                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---------------------------------------------------------------------------------------------------------------------------- | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Transfert d'inférence (Amazon Bedrock, Claude Platform on AWS, Agent Platform de Google Cloud, Microsoft Foundry, Anthropic) | Disponible     | Avec traduction de modèle par amont et basculement. L'amont Amazon Bedrock utilise le point de terminaison `bedrock-runtime` et la chaîne de credentials AWS par défaut ; le [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint) d'Amazon Bedrock n'est pas un amont supporté. L'[amont Claude Platform on AWS](/docs/fr/claude-apps-gateway-config#claude-platform-on-aws) nécessite Claude Code v2.1.198 ou ultérieur sur le serveur de passerelle. |
| Accès au modèle et paramètres gérés par groupe IdP                                                                           | Disponible     | L'accès au modèle est appliqué côté serveur ; les paramètres gérés sont livrés par groupe IdP et appliqués par l'interface de ligne de commande au [niveau des paramètres gérés](/docs/fr/settings#settings-precedence)                                                                                                                                                                                                                                                     |
| Fan-out de télémétrie (OTLP/HTTP)                                                                                            | Disponible     | Identité-estampillé par export ; encodages protobuf et JSON                                                                                                                                                                                                                                                                                                                                                                                                            |
| Fournisseurs d'identité OIDC                                                                                                 | Disponible     | Tout IdP conforme à OIDC ; la passerelle exécute la découverte OIDC standard et le flux du code d'autorisation. Voir [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup) pour la configuration par IdP                                                                                                                                                                                                                   |
| Limites de dépenses par utilisateur et par groupe                                                                            | Disponible     | Voir [Limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                                       |
| Recherche web côté serveur                                                                                                   | Non disponible | L'interface de ligne de commande ne peut pas voir quel fournisseur en amont la passerelle achemine vers, donc elle ne peut pas vérifier le support de la recherche web et désactive WebSearch sur les sessions de passerelle                                                                                                                                                                                                                                           |
| Mise en cache des invites standard                                                                                           | Disponible     | Les points d'arrêt `cache_control` sont transférés à chaque amont                                                                                                                                                                                                                                                                                                                                                                                                      |
| TTL de cache d'1 heure                                                                                                       | Non disponible | L'interface de ligne de commande omet la bêta extended-cache-ttl sur les sessions de passerelle, car pas tous les ammonts vers lesquels la passerelle peut acheminer supportent le TTL d'1 heure, donc la mise en cache des invites via la passerelle utilise le TTL de 5 minutes ; voir la note sur l'en-tête bêta ci-dessus                                                                                                                                          |
| Mode Auto                                                                                                                    | Disponible     | Suit les [règles du fournisseur tiers](/docs/fr/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry) : seuls les modèles éligibles sur les fournisseurs tiers peuvent l'utiliser. Avant v2.1.207, le mode auto sur les sessions de passerelle nécessitait de définir `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, livrable via le bloc `env` de la politique gérée                                                                                                  |
| Optimisations propriétaires uniquement comme la portée du cache global et les outils efficaces en jetons                     | Non disponible | L'interface de ligne de commande ne les active pas sur les sessions de passerelle ; voir la note sur l'en-tête bêta ci-dessus                                                                                                                                                                                                                                                                                                                                          |
| OTLP/gRPC                                                                                                                    | Non supporté   | OTLP sur HTTP uniquement                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| SAML, LDAP et autres authentifications non-OIDC                                                                              | Non supporté   | OIDC uniquement. Frontal avec un pont OIDC si nécessaire                                                                                                                                                                                                                                                                                                                                                                                                               |
| Multi-locataire (plusieurs émetteurs OIDC)                                                                                   | Non supporté   | Un émetteur par passerelle. Exécutez des instances séparées                                                                                                                                                                                                                                                                                                                                                                                                            |
| Serveur Windows                                                                                                              | Non supporté   | Déployez sur Linux. macOS pour le développement local uniquement                                                                                                                                                                                                                                                                                                                                                                                                       |
| Graphique Helm                                                                                                               | Non disponible | La passerelle s'exécute comme un Deployment sans état standard ; voir le [guide de déploiement](/docs/fr/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                                             |
| Interface utilisateur d'administration                                                                                       | Non disponible | La configuration est le fichier YAML ; redéployez pour le modifier                                                                                                                                                                                                                                                                                                                                                                                                     |

<h2 id="next-steps">
  Prochaines étapes
</h2>

Le démarrage rapide vous laisse avec une configuration minimale s'exécutant sous Docker Compose. Pour aller plus loin :

* Développez `gateway.yaml` au-delà de la configuration minimale, par exemple pour ajouter RBAC par groupe, basculement multi-amont, ou destinations de télémétrie. La [référence de configuration](/docs/fr/claude-apps-gateway-config) couvre chaque option.
* Passez de Compose à un déploiement de production sur Kubernetes ou Cloud Run, configurez correctement votre IdP, et examinez le modèle de sécurité. Le [guide de déploiement et d'opérations](/docs/fr/claude-apps-gateway-deploy) couvre la configuration par IdP, les exigences d'image de conteneur, les sondes de santé et le dépannage.
* Mettez des plafonds de dépenses sur les développeurs individuels ou les groupes pour qu'une charge de travail incontrôlée ne puisse pas consommer tout votre engagement. [Limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits) couvre l'API d'administration et le fonctionnement de l'application.
* Pour un exemple complet travaillé sur Google Cloud, avec Cloud Run, Cloud SQL et Secret Manager, voir [Déployer sur Google Cloud](/docs/fr/claude-apps-gateway-on-gcp).
