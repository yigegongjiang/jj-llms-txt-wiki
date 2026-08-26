> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuration de la passerelle Claude apps

> Référence pour chaque option gateway.yaml : écouteur et TLS, OIDC, session, magasin Postgres, amonts Amazon Bedrock, Claude Platform sur AWS, Agent Platform de Google Cloud et Microsoft Foundry, routage des modèles, politiques gérées et télémétrie.

Un déploiement de passerelle Claude apps est configuré par un fichier YAML, conventionnellement `gateway.yaml`. Le fichier définit tout ce que fait la passerelle : où elle écoute, comment les développeurs se connectent, où va l'inférence et quelles politiques et télémétrie s'appliquent. Cette page est la référence pour chaque option de ce fichier.

Pour écrire votre première, commencez par le [démarrage rapide](/docs/fr/claude-apps-gateway#quickstart), qui crée une configuration minimale fonctionnelle et l'exécute. Une fois que vous avez une configuration avec laquelle vous êtes satisfait, le [guide de déploiement](/docs/fr/claude-apps-gateway-deploy) couvre la conteneurisation et l'hébergement sur Kubernetes, Cloud Run ou votre propre plateforme.

La passerelle lit le fichier une fois, au démarrage, avec `claude gateway --config /path/to/gateway.yaml`. Chaque option est validée par rapport à un schéma au démarrage, donc une configuration mal formée échoue au démarrage avec une erreur au niveau du champ plutôt qu'à la première utilisation.

L'[exemple complet](#complete-example) à la fin de cette page exerce chaque section.

<h2 id="file-structure">
  Structure du fichier
</h2>

Cinq sections sont [requises](#required-sections). Chaque autre section est [optionnelle](#optional-sections), et une section omise prend ses valeurs par défaut. Les clés inconnues échouent au démarrage, donc une faute de frappe apparaît comme une erreur nommée plutôt qu'un paramètre silencieusement ignoré.

**Sections requises :**

* [`listen`](#listen) : adresse de liaison, URL publique, terminaison TLS
* [`oidc`](#oidc) : votre fournisseur d'identité (IdP), y compris l'émetteur, le client, le mappage des réclamations et qui peut se connecter
* [`session`](#session) : les jetons porteurs que la passerelle émet, avec secret et durée de vie
* [`store`](#store) : PostgreSQL, pour les subventions d'appareils et les compteurs de limite de débit
* [`upstreams`](#upstreams) : où l'inférence va, qu'il s'agisse d'Anthropic, Amazon Bedrock, Claude Platform sur AWS, Agent Platform de Google Cloud ou Microsoft Foundry

**Sections optionnelles :**

* [`admin`](#admin) : authentification de l'API Admin et rétention pour les limites de dépenses
* [`enforcement`](#enforcement) : comportement de limite de dépenses fail-open ou fail-closed
* [`models`](#models) et `auto_include_builtin_models` : liste de modèles curée par l'administrateur et IDs par upstream
* [`managed`](#managed) : politiques de paramètres gérés par groupe IdP
* [`telemetry`](#telemetry) : transfert OTLP vers votre pile d'observabilité
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning) : autorisation/refus IP, plafonds de taille de requête, time-to-first-byte upstream et limites de connexion par IP

<h2 id="secret-expansion">
  Expansion des secrets
</h2>

N'écrivez pas de secrets tels que `client_secret`, `jwt_secret` ou `postgres_url` directement dans `gateway.yaml`. Référencez-les avec l'une des formes ci-dessous, et la passerelle résout la valeur au démarrage à partir d'une variable d'environnement ou d'un fichier :

| Forme           | Résout à                                                              | Utiliser pour                                                                 |
| --------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `${VAR}`        | La variable d'environnement `VAR`. Le démarrage échoue si non défini. | Variables d'environnement de conteneur, AWS Secrets Manager via injection env |
| `${file:/path}` | Contenu du fichier, coupé                                             | Montages de volume Kubernetes Secret, Vault Agent, SOPS                       |

<h2 id="required-sections">
  Sections requises
</h2>

<h3 id="listen">
  `listen`
</h3>

Le bloc `listen` contrôle où la passerelle sert : l'adresse de liaison et le port, l'origine visible en externe, et la terminaison TLS optionnelle.

| Champ                  | Requis            | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ---------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | Non               | Adresse de liaison. Par défaut `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `port`                 | Non               | Port de liaison. Par défaut `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `public_url`           | Derrière un proxy | L'origine `https://` visible en externe, utilisée pour construire le `redirect_uri` IdP et les métadonnées de découverte. Requis derrière tout proxy terminant TLS tel qu'un ALB, Ingress ou Cloud Run, car la passerelle ne fait pas confiance aux en-têtes `X-Forwarded-*` lors de la construction de sa propre origine ; ils sont spoofables par le client. `trusted_proxies` ci-dessous régit uniquement la résolution de l'IP du client. Également requis pour activer la [télémétrie](#telemetry), car la passerelle construit le point de terminaison OTLP qu'elle pousse aux clients à partir de cette URL. |
| `tls.cert` / `tls.key` | Non               | Chemins PEM si la passerelle termine TLS elle-même                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `trusted_proxies`      | Non               | CIDRs ou IPs des équilibreurs de charge devant la passerelle. Lorsqu'il est défini, la passerelle fait confiance à `X-Forwarded-For` uniquement à partir de ces pairs et enregistre l'IP client réelle pour la limitation de débit par IP et l'audit. Équivalent à nginx `set_real_ip_from`.                                                                                                                                                                                                                                                                                                                        |

<h3 id="oidc">
  `oidc`
</h3>

Le bloc `oidc` connecte la passerelle à votre fournisseur d'identité et décide qui peut se connecter. Il nomme l'émetteur et le client OAuth, mappe les réclamations qui portent l'e-mail et les groupes, et restreint la connexion par domaine d'e-mail ou groupe.

OpenID Connect (OIDC) est le protocole SSO que la passerelle utilise avec votre fournisseur d'identité ; voir [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup) pour ce qu'il faut enregistrer du côté IdP.

| Champ                           | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Oui    | Base de découverte OIDC. Doit servir la découverte à `/.well-known/openid-configuration`. Utilisez HTTPS en production ; la passerelle accepte un émetteur `http://`. Un émetteur de boucle locale tel que `http://localhost:8081` est rejeté par la [garde SSRF](/docs/fr/claude-apps-gateway-deploy#threat-model-summary) sauf si `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` est défini dans l'environnement de la passerelle.                                                                                                                                                                                                                                                                                                    |
| `client_id` / `client_secret`   | Oui    | De votre enregistrement de client OAuth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `allowed_email_domains`         | Non    | Rejeter les id\_tokens dont la réclamation `email` n'est pas dans l'un de ces domaines, insensible à la casse. Défense en profondeur contre les erreurs de configuration IdP multi-locataires. Indépendamment de ce paramètre, un id\_token dont la réclamation `email_verified` est explicitement `false` est toujours rejeté.                                                                                                                                                                                                                                                                                                                                                                                       |
| `allowed_groups`                | Non    | Restreindre la connexion aux membres de ces groupes IdP, appariés par rapport à `groups_claim`. Un utilisateur dans un domaine d'e-mail autorisé mais dans aucun de ces groupes est rejeté. Nécessite que l'IdP émette la réclamation de groupes.                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `groups_claim`                  | Non    | Quelle réclamation id\_token porte l'appartenance au groupe. Par défaut `groups`. Microsoft Entra émet les rôles d'application sous `roles`. Accepte une clé plate ou un pointeur JSON RFC 6901 tel que `/resource_access/gateway/roles` pour les réclamations imbriquées.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `google_groups`                 | Non    | Rechercher les groupes de l'utilisateur connecté via l'API Google Workspace Admin SDK Directory, car le id\_token de Google ne porte aucune réclamation de groupes. Définissez `service_account_json_path` sur un fichier de clé de compte de service avec délégation à l'échelle du domaine sur la portée `https://www.googleapis.com/auth/admin.directory.group.readonly`, et `admin_email` sur un administrateur Workspace que le compte de service usurpe ; l'API Directory nécessite un sujet administrateur réel. Les adresses e-mail de groupe de chaque utilisateur deviennent leur réclamation de groupes, donc `allowed_groups` et `managed.policies.match.groups` correspondent sur les e-mails de groupe. |
| `email_claim`                   | Non    | Quelle réclamation id\_token porte l'e-mail de l'utilisateur. Par défaut `email`. Certains IdPs, tels que ADFS et Entra B2C, émettent `upn` ou `preferred_username` à la place. Accepte une clé plate, un pointeur JSON ou une liste de clés de secours où la première clé présente est utilisée.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `scopes`                        | Non    | Remplacement complet des portées OIDC que la passerelle demande. Par défaut `[openid, profile, email, offline_access]`. Définissez lorsque votre IdP rejette les portées qu'il ne reconnaît pas, ou nécessite une portée personnalisée pour émettre des groupes ou un e-mail. Doit inclure `openid`. Supprimer `offline_access` désactive les jetons d'actualisation, donc les développeurs réexécutent la connexion au navigateur tous les `session.ttl_hours`. Voir [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup) pour les recettes de portée par IdP telles que le flux de jeton d'actualisation de Google.                                                    |
| `extra_auth_params`             | Non    | Paramètres de requête supplémentaires ajoutés à la demande d'autorisation IdP, textuellement. C'est le mécanisme de remplacement pour le comportement spécifique à l'IdP, tel que `access_type: offline` pour les jetons d'actualisation Google, `domain_hint` pour certains locataires Entra, ou `acr_values` pour les flux d'escalade. Ne peut pas remplacer les paramètres de protocole gérés par la passerelle : `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` et `client_id`.                                                                                                                                                                                                |
| `userinfo_fallback`             | Non    | Lorsque le id\_token omet l'e-mail ou les groupes, les récupérer à partir de `/userinfo`. Nécessaire pour les jetons d'accès légers Keycloak, le serveur org Okta et les jetons minimaux ADFS. Le id\_token reste faisant autorité ; userinfo remplit uniquement les lacunes. Par défaut `false`.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `use_pkce`                      | Non    | Envoyer un défi PKCE (S256) sur la demande d'autorisation. Par défaut `true`. Définissez `false` uniquement si votre IdP rejette PKCE pour ce client confidentiel.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `clock_skew_seconds`            | Non    | Tolérer la dérive d'horloge lors de la validation des réclamations de temps id\_token. Par défaut `0`, ce qui est strict. Augmentez si vous voyez des erreurs « token expired / not yet valid » juste après la connexion en raison d'une dérive d'horloge hôte/IdP.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `token_endpoint_auth_method`    | Non    | Remplacer la méthode d'authentification du point de terminaison de jeton. Accepte `client_secret_basic` ou `client_secret_post`. Négocié automatiquement par défaut.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `id_token_signed_response_alg`  | Non    | Algorithme de signature id\_token attendu. Par défaut `RS256`. Définissez pour les IdPs qui signent avec ES256, PS256 ou EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `additional_authorized_parties` | Non    | Valeurs `azp` supplémentaires à accepter au-delà de `client_id`, pour les flux de courtier Keycloak et d'échange de jetons                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `discovery_url`                 | Non    | Récupérer le document de découverte à partir de cette URL au lieu de le dériver de `issuer`, pour les IdPs derrière un proxy qui réécrit l'hôte émetteur. Le chemin doit contenir `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `form_action_origins`           | Non    | Origines supplémentaires pour la directive `Content-Security-Policy: form-action` de la page `/device`. La passerelle autorise déjà `'self'` et l'origine du `authorization_endpoint` découverte, mais Chrome applique `form-action` à toute la chaîne de redirection. Si votre IdP redirige via un deuxième hôte, tel que Azure AD fédéré à ADFS, Okta hub-spoke ou un intercepteur SSO d'entreprise, listez chaque origine par laquelle la demande d'autorisation peut rediriger.                                                                                                                                                                                                                                   |
| `ca_cert_pem`                   | Non    | Certificat CA PEM qui remplace le magasin de confiance système pour les demandes IdP uniquement. Utilisez pour Keycloak ou Dex derrière une PKI d'entreprise.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

<h3 id="session">
  `session`
</h3>

Le bloc `session` façonne les jetons porteurs que la passerelle émet après la connexion : le secret qui les signe et combien de temps ils vivent.

| Champ        | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | Oui    | Au moins 32 octets d'entropie, par exemple à partir de `openssl rand -base64 32`. Signe les jetons porteurs HS256 de la passerelle. Accepte une chaîne unique ou un tableau pour la rotation : l'index 0 signe et toutes les entrées vérifient. Pour faire pivoter, prépendez un nouveau secret, attendez `ttl_hours`, puis supprimez l'ancien.                                                                                                                                                                                                         |
| `ttl_hours`  | Non    | Durée de vie du jeton porteur de la passerelle. Par défaut `1`. Le CLI s'actualise silencieusement avant l'expiration lorsque l'IdP émet des jetons d'actualisation. Une durée de vie plus courte déprovisionne plus rapidement ; une plus longue fait moins de trajets IdP. Si votre IdP ne peut pas émettre de jetons d'actualisation car `offline_access` n'est pas disponible, il n'y a pas d'actualisation silencieuse, donc augmentez ceci à `8` ou `12` pour éviter de renvoyer les développeurs à la connexion au navigateur toutes les heures. |

<h3 id="store">
  `store`
</h3>

Le bloc `store` pointe la passerelle vers sa base de données PostgreSQL, qui contient les subventions d'appareils et les compteurs de limite de débit.

| Champ             | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ----------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | Oui    | URL `postgres://` ou `postgresql://`. Requis : le rendez-vous de subvention d'appareil, où le rappel du navigateur écrit et le CLI d'interrogation lit, a besoin d'un état entre répliques. La passerelle exécute ses propres migrations de schéma au démarrage, donc le rôle a besoin de `CREATE TABLE` sur le schéma cible. Si votre politique de sécurité interdit le DDL du rôle d'application, exécutez les migrations avec un rôle administrateur, initialement et à nouveau chaque fois qu'une nouvelle version expédie des migrations, et accordez au rôle d'application `SELECT, INSERT, UPDATE, DELETE` sur les tables de la passerelle. Voir [Mises à jour](/docs/fr/claude-apps-gateway-deploy#upgrades) et [Postgres](/docs/fr/claude-apps-gateway-deploy#postgres). |
| `username`        | Non    | Remplace l'utilisateur dans `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `password`        | Non    | Identifiant de base de données. Définissez-le ici plutôt que dans `postgres_url` pour que l'identifiant reste hors de l'URL. Accepte n'importe quel caractère et prend la priorité sur les identifiants d'URL.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `max_connections` | Non    | Taille du pool de connexions Postgres par réplique. Par défaut `5`, ce qui est conservateur et convivial pour les bases de données partagées. Avec les [limites de dépenses](#admin) activées, le chemin chaud effectue quelques opérations par demande d'inférence, donc augmentez-le pour une base de données dédiée sous charge, et gardez les répliques × ceci en dessous du `max_connections` de la base de données.                                                                                                                                                                                                                                                                                                                                               |

Pour le développement local, pointez `postgres_url` sur un conteneur Postgres jetable, par exemple `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` est une liste ordonnée. La passerelle transfère l'inférence au premier upstream qui résout le modèle demandé. Sur `5xx`, `429`, `401`, `403`, `404`, ou timeout, elle bascule vers le suivant ; les autres `4xx` ne le font pas, car ces erreurs sont attribuables à la demande plutôt qu'à l'upstream. Un `401` ou `403` signifie que l'identifiant propre de la passerelle a échoué contre cet upstream, et un `404` signifie que cet upstream ne sert pas le modèle demandé, donc un upstream ultérieur dans la liste peut toujours le faire.

Le basculement sur `404` nécessite la passerelle v2.1.198 ou ultérieure. Les versions antérieures retournaient le premier `404` au client même lorsqu'un upstream ultérieur dans la liste servait le modèle.

Plusieurs upstreams du même fournisseur doivent définir un `name:` distinct.

Les clients Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform et Microsoft Foundry sont construits une fois au démarrage, et leurs SDKs actualisent les identifiants en interne, donc la rotation des identifiants cloud ne nécessite pas un redémarrage. Les clés API Anthropic statiques et les porteurs sont lus au démarrage ; voir [Anthropic API](#anthropic-api).

<h4 id="anthropic-api">
  Anthropic API
</h4>

L'upstream Anthropic minimal est une clé API de la [Console Claude](https://platform.claude.com) :

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # OU un porteur OAuth (par exemple un jeton échangé par Workload-Identity-Federation) :
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # par défaut ; remplacer pour un proxy de transfert
```

Les deux formes d'identifiants diffèrent dans l'en-tête qu'elles envoient :

* **`api_key`** : envoie `x-api-key`. Faites-la pivoter dans la Console Claude et mettez à jour la variable env.
* **`oauth_token`** : envoie `Authorization: Bearer`. Utilisez la forme porteur lorsque votre organisation émet des jetons de courte durée au lieu de clés API de longue durée. Le porteur est lu une fois au démarrage, donc actualisez en remontant le secret et en redémarrant.

Au lieu d'une clé statique ou d'un porteur, vous pouvez utiliser Workload Identity Federation. Créez une règle de fédération en suivant le [guide Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), puis montez le JWT OIDC de votre charge de travail en tant que fichier, tel qu'un jeton de compte de service projeté Kubernetes ou un id-token de plateforme CI. La passerelle échange le JWT pour un porteur de courte durée et l'actualise automatiquement. Le fichier de jeton est relu à chaque échange, donc les jetons projetés pivotés sont récupérés sans redémarrage.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # requis si la règle couvre >1 espace de travail
      # service_account_id: svac_...   # vérification de cible attendue optionnelle
```

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Pour le déploiement Bedrock côté client que la passerelle remplace ou fronts, voir [Claude Code sur Amazon Bedrock](/docs/fr/amazon-bedrock). L'upstream côté passerelle :

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # préféré : chaîne d'identifiants par défaut AWS
    # OU identifiants explicites :
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # OU un jeton porteur Bedrock API :
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Remplacer le point de terminaison bedrock-runtime pour les déploiements FIPS ou VPC-endpoint :
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

Un bloc `auth` vide utilise la chaîne d'identifiants par défaut du SDK AWS : variables env, `~/.aws/credentials`, rôle de tâche ECS, métadonnées d'instance EC2 ou IRSA sur EKS. En production, donnez au pod de la passerelle un rôle IAM au lieu d'intégrer des clés statiques dans une image de conteneur.

Les identifiants explicites doivent être complets : la passerelle échoue au démarrage lorsque `aws_access_key_id` et `aws_secret_access_key` ne sont pas définis ensemble, ou lorsque `aws_session_token` est défini sans eux. Avant v2.1.207, un bloc `auth:` partiel passait la validation.

| Configuration         | Comment                                                                                                                                                                                                                                                                                                                                                                 |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Permissions IAM       | Accordez au principal de la passerelle `bedrock:InvokeModel` et `bedrock:InvokeModelWithResponseStream` sur les ARNs de profil d'inférence et les ARNs de modèle de fondation sous-jacents. Pour le catalogue intégré dans les régions US : `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` et `arn:aws:bedrock:*::foundation-model/anthropic.*`. |
| Accès au modèle       | Dans la console Bedrock, par région, demandez et activez l'accès au modèle pour les modèles Claude que vous souhaitez. Les profils d'inférence inter-régions (`us.anthropic.*`) nécessitent un accès au modèle dans chaque région que le profil couvre.                                                                                                                 |
| EKS (IRSA)            | Créez un rôle IAM avec la politique ci-dessus et une politique de confiance pour le fournisseur OIDC de votre cluster limité au compte de service de la passerelle. Annotez le compte de service avec `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` le récupère.                                                                    |
| ECS / EC2             | Attachez le rôle IAM à la définition de tâche ou au profil d'instance. `auth: {}` le récupère.                                                                                                                                                                                                                                                                          |
| N'importe où ailleurs | Passez les identifiants via les variables env `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` et `AWS_SESSION_TOKEN`, ou définissez-les explicitement dans `auth:` avec expansion `${VAR}`                                                                                                                                                                                 |
| Région                | `region:` est la région du point de terminaison API. Les profils d'inférence inter-régions routent à travers la géographie (US, EU, APAC) indépendamment de celui que vous choisissez. Pour les régions non-US ou les ARNs de débit provisionné, ajoutez un bloc [`models:`](#models) avec les bons IDs par upstream.                                                   |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS sert l'API Anthropic propriétaire sur l'infrastructure AWS à `aws-external-anthropic.<region>.api.aws`. Il utilise les IDs de modèle propriétaires, honore les en-têtes `anthropic-beta` tels qu'envoyés, et sert `count_tokens`, donc aucune des traductions spécifiques à Bedrock ne s'applique. Le fournisseur `anthropicAws` nécessite Claude Code v2.1.198 ou ultérieur ; les versions antérieures de la passerelle le rejettent au démarrage.

Pour le déploiement côté client de la même plateforme, voir [Claude Code sur Claude Platform on AWS](/docs/fr/claude-platform-on-aws). L'upstream côté passerelle :

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # envoyé en tant que x-api-key
    # OU SigV4 via la chaîne d'identifiants par défaut AWS :
    # auth: {}
    # OU identifiants SigV4 explicites :
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Remplacer le point de terminaison dérivé :
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

La plateforme s'exécute dans un compte AWS séparé d'Amazon Bedrock et signe les demandes SigV4 pour son propre nom de service, `aws-external-anthropic`, donc un rôle IAM limité à Bedrock ne l'autorise pas. Une clé API dans `auth.api_key` prend la priorité lorsque les identifiants SigV4 sont également définis. Un bloc `auth` vide utilise la chaîne d'identifiants par défaut du SDK AWS, la même chaîne que l'upstream [Amazon Bedrock](#amazon-bedrock) utilise.

| Champ                                                   | Requis | Description                                                                                                                                                                                  |
| ------------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `region`                                                | Oui    | Région AWS, lettres minuscules, chiffres et traits d'union. La passerelle dérive le point de terminaison à partir de celui-ci en tant que `https://aws-external-anthropic.<region>.api.aws`. |
| `workspace_id`                                          | Oui    | Envoyé en tant qu'en-tête sur chaque demande ; la plateforme l'exige                                                                                                                         |
| `auth.api_key`                                          | Non    | Clé API pour la plateforme, envoyée en tant que `x-api-key`. Pas un jeton porteur : les deux modes d'authentification sont une clé API ou SigV4.                                             |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | Non    | Identifiants SigV4 explicites. Définir l'un sans l'autre échoue au démarrage. `auth.aws_session_token` est accepté à côté d'eux.                                                             |
| `base_url`                                              | Non    | Remplacer le point de terminaison dérivé                                                                                                                                                     |

Parce que la plateforme résout les IDs de modèle propriétaires, le catalogue intégré route vers elle sans bloc [`models:`](#models). Lorsque vous organisez une liste `models:`, indexez l'entrée `anthropicAws:` avec l'ID propriétaire.

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

Pour la configuration équivalente côté client, voir [Claude Code sur Google Cloud](/docs/fr/google-vertex-ai). L'upstream côté passerelle :

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # préféré : identifiants par défaut d'application
    # OU un fichier de clé de compte de service :
    # auth: { service_account_json: /secrets/sa.json }
    # Remplacer le point de terminaison aiplatform pour Private Service Connect :
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

Un bloc `auth` vide utilise les identifiants par défaut d'application : `GOOGLE_APPLICATION_CREDENTIALS`, métadonnées GCE ou Workload Identity GKE. Les fichiers de clé JSON de compte de service sont pris en charge mais déconseillés ; utilisez Workload Identity ou attachez un compte de service à l'instance GCE ou Cloud Run.

Définissez `region: global` pour utiliser le [point de terminaison global d'Agent Platform](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) au lieu d'un point de terminaison régional. Google route ensuite chaque demande vers une région disponible, donc vous ne suivez pas la disponibilité du modèle par région. Définir une région spécifique épingle chaque demande à celle-ci.

| Configuration           | Comment                                                                                                                                                                                                                               |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Permissions IAM         | Accordez au compte de service de la passerelle `roles/aiplatform.user` sur le projet, ou un rôle personnalisé avec `aiplatform.endpoints.predict`. Activez l'API Agent Platform (`aiplatform.googleapis.com`).                        |
| Accès au modèle         | Dans Model Garden, activez les modèles Claude pour votre projet. Ils publient vers des régions spécifiques ; vérifiez la fiche du modèle pour les régions prises en charge.                                                           |
| GKE (Workload Identity) | Liez un compte de service GCP au compte de service Kubernetes de la passerelle et annotez le KSA avec `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`. `auth: {}` le récupère.                        |
| Cloud Run / GCE         | Définissez le compte de service du service sur un avec `roles/aiplatform.user`. `auth: {}` le récupère.                                                                                                                               |
| N'importe où ailleurs   | `auth: { service_account_json: /secrets/sa.json }`, le chemin vers un fichier de clé JSON monté en tant que secret. Le champ prend un chemin de fichier, pas le contenu de la clé, donc aucune expansion `${file:…}` n'est impliquée. |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Pour le déploiement Foundry côté client, voir [Claude Code sur Microsoft Foundry](/docs/fr/microsoft-foundry). L'upstream côté passerelle :

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # préféré : DefaultAzureCredential / Managed Identity
    # OU une clé API :
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` résout via `DefaultAzureCredential` : Managed Identity sur AKS, ACI ou App Service ; l'Azure CLI ; ou les identifiants d'environnement. Les clés API fonctionnent mais sont à l'échelle du projet et ne pivotent pas automatiquement. Le point de terminaison de Foundry est dérivé de `resource:` ; définissez le `base_url` optionnel pour le remplacer pour les clouds souverains tels que Azure Government.

| Configuration           | Comment                                                                                                                                                                                                     |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                    | Accordez à l'identité de la passerelle `Azure AI User` ou `Cognitive Services User` sur la ressource Foundry                                                                                                |
| Déploiements            | Foundry utilise les noms de déploiement choisis par l'administrateur, pas les IDs de modèle canoniques. Ajoutez un bloc [`models:`](#models) mappant chaque ID canonique à votre nom de déploiement.        |
| AKS (workload identity) | Fédérez une identité gérée affectée par l'utilisateur avec le émetteur OIDC du cluster et liez-la au compte de service de la passerelle. `use_azure_ad: true` le récupère via `WorkloadIdentityCredential`. |
| ACI / App Service       | Activez l'identité gérée affectée par le système ou l'utilisateur sur la ressource. `use_azure_ad: true` le récupère.                                                                                       |
| N'importe où ailleurs   | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Citez `${…}` à l'intérieur de `{ }`.                                                                                                                             |

<h4 id="multiple-upstreams">
  Plusieurs upstreams
</h4>

Le même fournisseur peut apparaître plus d'une fois avec un `name:` distinct. Cela couvre différentes régions, différents comptes via différentes chaînes d'identifiants, débit provisionné par rapport à la demande, et basculement inter-fournisseur.

La passerelle essaie les upstreams dans l'ordre. `5xx`, `429`, `401`, `403`, `404`, timeouts et point de terminaison manquant (`501`) basculent ; les autres `4xx` ne le font pas.

`429` est la capacité par upstream, donc l'épuisement du débit provisionné (PT) bascule vers la demande. `404` est la disponibilité du modèle par upstream, donc un upstream qui n'a pas activé un modèle ne bloque pas un upstream ultérieur qui le sert. Un upstream qui ne peut pas résoudre le modèle demandé est ignoré sans un aller-retour réseau.

Cet exemple route une allocation Bedrock de débit provisionné en premier, déborde vers la demande et un deuxième compte, et revient à l'API Anthropic en dernier :

```yaml theme={null}
upstreams:
  # Principal : débit provisionné dans votre région d'accueil.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Débordement : demande inter-régions.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Compte différent : une allocation Bedrock séparée via des identifiants de rôle assumé.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Dernier recours : API Anthropic directe.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# Les IDs de modèle par upstream sont indexés sur le `name:` de l'upstream ; un upstream
# sans `name:` prend par défaut sa chaîne de fournisseur (par exemple `bedrock`). N'importe quel
# upstream non listé pour un modèle est ignoré, c'est ainsi que vous routez un modèle
# vers le débit provisionné tandis que tout le reste reste à la demande.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Levier                           | Comment                                                                                                                                                                                                                                                      |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Différentes régions              | Un upstream Bedrock par région, chacun avec sa propre `region:`. Avec [`auto_include_builtin_models: true`](#models) les profils d'inférence inter-régions routent automatiquement ; pour les déploiements épinglés à la région, utilisez un bloc `models:`. |
| Différents comptes               | Un upstream Bedrock par compte, chacun avec ses propres identifiants dans `auth:`. La chaîne par défaut (`auth: {}`) utilise l'identité du pod ; pour un deuxième compte, définissez des identifiants explicites ou un jeton porteur.                        |
| Débit provisionné                | Mappez le modèle à l'ARN de débit provisionné dans `models:` pour le nom de cet upstream. Les autres upstreams gardent l'ID à la demande, donc la capacité PT est épuisée avant de basculer.                                                                 |
| Points de terminaison VPC / FIPS | Définissez `base_url:` sur l'upstream vers votre URL de point de terminaison VPC ou FIPS                                                                                                                                                                     |
| Routage limité au modèle         | Omettez un upstream de la carte `upstream_model:` d'un modèle et cet upstream est ignoré pour ce modèle. Par exemple, routez Opus vers le débit provisionné et Sonnet et Haiku vers la demande.                                                              |

Le basculement entre les fournisseurs cloud ou vers l'API Anthropic directe change quel accord, géographie et autres conditions régissent la demande.

Le CLI applique le même contrôle de fonctionnalité aux passerelles indépendamment de quel upstream sert une demande donnée, donc le basculement n'envoie pas un champ de corps qu'un upstream rejetterait.

<h2 id="optional-sections">
  Sections optionnelles
</h2>

<h3 id="admin">
  `admin`
</h3>

Optionnel. Active `/v1/organizations/spend_limits`, qui reflète l'API Admin publique d'Anthropic, et l'application de dépenses par développeur sur `/v1/messages`. Voir [Limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits) pour comment les plafonds sont définis et appliqués ; cette section couvre les clés `gateway.yaml` qui activent la fonctionnalité et l'ajustent.

```yaml theme={null}
admin:
  # Clés API statiques nommées pour les points de terminaison admin, envoyées en tant que x-api-key.
  # L'id apparaît dans le journal d'audit en tant que admin-key:<id> donc chaque clé est
  # attribuable. Tableau pour la rotation : ajoutez la nouvelle clé, déployez les clients,
  # supprimez l'ancienne.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # Groupes IdP accordés un accès administrateur complet via le JWT de passerelle normal (pas de clé API).
  admin_groups: [platform-finops]
  blocked_message: demander une augmentation à https://go.example.com/claude-limits
```

| Champ                     | Requis | Description                                                                                                                                                                                                                                                                                                                |
| ------------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | Non    | Tableau de `{id, key}`. Un `x-api-key` correspondant à l'un de ceux-ci peut lister, définir et supprimer les limites de dépenses. Les valeurs de clé doivent être au moins 32 caractères ; les `id`s doivent être uniques dans `read_keys` et `write_keys`.                                                                |
| `read_keys`               | Non    | Tableau de `{id, key}`. Lecture seule : chaque point de terminaison `GET`, y compris la liste des plafonds, la récupération d'un par ID et la lecture de [`/effective`](/docs/fr/claude-apps-gateway-spend-limits#%2Feffective) et [`/audit`](/docs/fr/claude-apps-gateway-spend-limits#%2Faudit).                                   |
| `admin_groups`            | Non    | Noms de groupes IdP. Un JWT de passerelle dont la réclamation `groups` inclut l'un de ceux-ci a un accès administrateur complet, lecture et écriture, et audite en tant que `oidc:<sub>`. Utilisez ceci pour les administrateurs humains ; utilisez les clés API pour les machines.                                        |
| `blocked_message`         | Non    | Ajouté textuellement au `429 billing_error` qu'un développeur bloqué voit. Écrivez l'instruction complète, telle qu'une URL ou un canal Slack. Non défini, l'erreur est `spend limit reached`.                                                                                                                             |
| `audit_retention_days`    | Non    | Par défaut `365`. Les lignes `admin_audit` plus anciennes sont balayées.                                                                                                                                                                                                                                                   |
| `spend_retention_months`  | Non    | Par défaut `13`. Les lignes du compteur `spend` plus anciennes que ceci sont balayées. La valeur par défaut conserve une année complète plus le mois partiel actuel pour les rapports d'année en année.                                                                                                                    |
| `identity_retention_days` | Non    | Par défaut `90`. TTL de dernière vue pour les lignes `principal_emails`, qui contiennent l'e-mail, le nom d'affichage et les groupes de chaque développeur (PII). Délibérément plus court que la rétention des dépenses pour qu'une identité déprovisionée vieillit tandis que ses compteurs de dépenses anonymes restent. |
| `group_limit_mode`        | Non    | `min` (par défaut) ou `max`. Lorsqu'un développeur est dans plusieurs groupes avec des plafonds, `min` applique le plus restrictif et `max` le moins. Utilisé à la fois par l'application et `/effective`.                                                                                                                 |

<h3 id="enforcement">
  `enforcement`
</h3>

Le bloc `enforcement` contrôle comment les vérifications de limite de dépenses se comportent lorsque le magasin n'est pas disponible.

| Champ                  | Requis | Description                                                                                                                                                                                                                                                                                                                        |
| ---------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | Non    | Par défaut `false`. L'application de dépenses échoue ouvertement sur une panne Postgres, donc l'inférence reste active. Définissez `true` pour échouer fermé : les développeurs au-dessus du plafond sont bloqués, mais tout le monde l'est aussi si le magasin est inaccessible. N'a aucun effet sans un bloc [`admin:`](#admin). |

<h3 id="models">
  `models`
</h3>

Le bloc `models` est une liste de modèles curée par l'administrateur optionnelle, servie à `/v1/models` et utilisée pour traduire les IDs de modèle par upstream. Elle est requise pour les régions Bedrock non-US, les ARNs de débit provisionné Bedrock et les noms de déploiement Foundry.

```yaml theme={null}
auto_include_builtin_models: true   # false : exposer uniquement la liste ci-dessous
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: texte optionnel affiché dans les clients qui le surfacent
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # ou un ARN de profil d'inférence
      foundry: your-opus-deployment-name
```

<h3 id="managed">
  `managed`
</h3>

Le bloc `managed` définit les politiques d'accès basées sur les rôles indexées sur les groupes IdP ou le domaine d'e-mail. Les politiques sont évaluées dans l'ordre ; la première correspondance est sélectionnée, puis fusionnée sur la base de capture-tout `match: {}` décrite ci-dessous. Elles sont servies par utilisateur à `GET /managed/settings` avec mise en cache ETag/304.

```yaml theme={null}
managed:
  policies:
    # Groupes spécifiques en premier.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Capture-tout par défaut en dernier : correspond à tous les utilisateurs authentifiés.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Une capture-tout `match: {}`, conventionnellement listée en dernier, est traitée comme une couche de base. Chaque autre politique hérite de toute clé qu'elle ne définit pas de la capture-tout, donc les entrées par rôle n'ont besoin que de lister ce qui diffère de la valeur par défaut de l'organisation. Les règles de fusion dépendent du type de clé :

* **Listes d'autorisation** : `availableModels` et `permissions.allow`. La liste d'une politique spécifique remplace complètement celle de la base.
* **Listes de refus et tableaux de hooks** : `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` et chaque tableau de type d'événement `hooks`. Ceux-ci prennent l'union de la base et de la politique, donc un refus à l'échelle de l'organisation ou un hook d'audit ne peut pas être accidentellement supprimé par un remplacement par rôle.
* **Clés de type enregistrement** : `env`, `modelOverrides` et `skillOverrides`. Ceux-ci fusionnent superficiellement, donc un bloc `env` par rôle remplace les clés qu'il définit et hérite du reste de la base.

`availableModels` est également appliqué côté serveur à `/v1/messages`, donc un modèle refusé retourne `400` indépendamment de ce que le client envoie.

| Correspondant                                       | Comportement                                                                                                                                                      |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Correspond à tous les utilisateurs authentifiés. Commencez par un de ceux-ci et ajoutez des politiques limitées au groupe plus tard.                              |
| `match: { groups: [a, b] }`                         | Correspond si la réclamation `groups` du JWT contient l'un des groupes listés. Sensible à la casse : les groupes doivent correspondre à la casse exacte de l'IdP. |
| `match: { email_domain: example.com }`              | Correspond à la partie après le dernier `@` dans la réclamation `email` du JWT, insensible à la casse. Accepte un domaine par politique.                          |
| `match: { groups: [a], email_domain: example.com }` | Les deux conditions doivent correspondre                                                                                                                          |

Un utilisateur authentifié qui ne correspond à aucune politique obtient les valeurs par défaut de la passerelle, ce qui signifie chaque modèle du catalogue et aucun paramètre géré. Ajoutez une capture-tout `match: {}` en dernier si vous voulez une politique par défaut garantie.

<Note>
  La passerelle ne garde aucun répertoire d'utilisateurs de son propre. Elle autorise chaque demande à partir du jeton IdP de l'utilisateur, en lisant l'appartenance au groupe à partir de la réclamation `groups` du jeton et en évaluant les politiques par rapport à celle-ci. Il n'y a pas de liste à énumérer et pas de comptes à pré-créer, et donc pas de point de terminaison SCIM, car il n'y a rien pour que SCIM se synchronise.

  Exécutez la gestion du cycle de vie des utilisateurs et des groupes à la source de vérité, qui est la mise en service SCIM native de votre IdP ou une plateforme de gouvernance d'identité dédiée. L'appartenance et la déprovision gouvernées là-bas s'écoulent dans la passerelle automatiquement via le jeton. Si vous voulez la mise en service SCIM des comptes Claude eux-mêmes, c'est une capacité [Claude for Enterprise](/docs/fr/admin-setup).

  Deux horloges de propagation s'appliquent :

  * **Contenu de la politique** : éditer une politique et redéployer atteint les clients connectés lors de leur prochain sondage de paramètres gérés, dans une heure
  * **Appartenance au groupe** : changer l'appartenance au groupe d'un utilisateur change quelle politique les correspond. Cela prend effet lors de la prochaine remise en monnaie de session, ce qui signifie le prochain rafraîchissement silencieux, limité par `session.ttl_hours`.
</Note>

<h4 id="what-goes-in-cli">
  Ce qui va dans `cli`
</h4>

Chaque valeur `cli` est un document complet `managed-settings.json` de Claude Code, le même schéma que vous déploieriez via MDM ou `/etc/claude-code/managed-settings.json`, exprimé ici en YAML. Le CLI applique le document livré au niveau géré, au-dessus des paramètres utilisateur et projet.

La passerelle valide chaque document par rapport au schéma de paramètres du CLI au démarrage, donc une clé de niveau supérieur non reconnue ou une clé reconnue avec une valeur mal formée échoue au démarrage avec une erreur nommant chaque clé offensante. Les parties délibérément ouvertes du schéma acceptent toujours des valeurs arbitraires, car les clients plus récents peuvent reconnaître les entrées que le schéma de la passerelle ne reconnaît pas. Ces clés ouvertes sont `env`, `pluginConfigs` et les clés imbriquées sous `permissions`.

Parce que la validation utilise le schéma fourni avec la version installée de la passerelle, mettre une clé de paramètres de niveau supérieur introduite par une version plus récente de Claude Code dans la configuration gérée nécessite de mettre à niveau la passerelle en premier. Testez une nouvelle politique sur un client avant de la déployer largement.

La référence de clé complète est dans [Paramètres Claude Code](/docs/fr/settings#available-settings). Les clés que les opérateurs atteignent en premier :

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Accès au modèle (également appliqué côté serveur à /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Politique de permission
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # bloque --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignorer les règles de permission utilisateur/projet

        # Environnement poussé dans le processus CLI. DISABLE_UPDATES bloque
        # les mises à jour en arrière-plan et manuelles ; DISABLE_AUTOUPDATER arrête uniquement
        # les mises à jour en arrière-plan.
        env:
          DISABLE_UPDATES: "1"                    # épingler les versions via votre propre distribution

        # Hooks à l'échelle de l'organisation. Les commandes de hook s'exécutent sur les machines des développeurs, pas la
        # passerelle, donc le chemin doit exister sur chaque système d'exploitation client dans la politique.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Clé                                        | Appliquée par    | Effet                                                                                                                                                                                                                                      |
| ------------------------------------------ | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `availableModels`                          | Passerelle + CLI | Liste d'autorisation de modèles. Également vérifiée à `/v1/messages`, donc un client patché ne peut pas la contourner.                                                                                                                     |
| `permissions.allow` / `.deny`              | CLI              | Règles d'outils et de commandes. Voir [Permissions](/docs/fr/permissions).                                                                                                                                                                      |
| `permissions.disableBypassPermissionsMode` | CLI              | Définissez sur `disable` pour bloquer [`bypassPermissions`](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode), le mode qui approuve automatiquement chaque appel d'outil, et l'indicateur `--dangerously-skip-permissions` |
| `allowManagedPermissionRulesOnly`          | CLI              | Lorsque `true`, les règles de permission utilisateur et projet sont ignorées ; seules les règles de ce document s'appliquent                                                                                                               |
| `env`                                      | CLI              | Variables d'environnement fusionnées dans le processus CLI. Utilisez pour la télémétrie, la mise à jour automatique et les remplacements de noms de modèles.                                                                               |
| `hooks`                                    | CLI              | [Hooks](/docs/fr/hooks) à l'échelle de l'organisation                                                                                                                                                                                           |

Parce que ces paramètres arrivent sur le réseau, le CLI affiche à chaque développeur un dialogue d'approbation de sécurité unique avant d'appliquer quoi que ce soit qui puisse exécuter une commande shell ou modifier où le trafic va. Le dialogue couvre :

* `hooks`
* Variables `env` qui ne sont pas sur la liste sûre intégrée du CLI
* Paramètres d'exécution shell tels que `apiKeyHelper` et `statusLine`
* Contenu CLAUDE.md géré

La liste sûre détermine quelles variables `env` s'appliquent sans approbation :

* **Sur la liste sûre** : variables de mise à jour automatique et de nom de modèle
* **Pas sur la liste sûre** : variables de proxy, variables d'URL de base et `OTEL_EXPORTER_OTLP_ENDPOINT`

La configuration de [télémétrie](#telemetry) de la passerelle pousse `OTEL_EXPORTER_OTLP_ENDPOINT`, donc définir `telemetry.forward_to` déclenche le dialogue sur chaque client interactif. Le dialogue protège la machine du développeur d'une passerelle compromise ou hostile, pas l'organisation du développeur.

Une exécution non interactive avec l'indicateur `-p` ne peut pas afficher le dialogue. Elle applique les paramètres poussés pour cette exécution uniquement et ne les enregistre pas comme approuvés, donc la prochaine session interactive du développeur affiche toujours le dialogue. Avant v2.1.207, une exécution non interactive enregistrait les paramètres comme approuvés et aucune session interactive ultérieure n'affichait le dialogue pour eux.

Si un développeur refuse, Claude Code se termine plutôt que d'appliquer la politique. Pousser un nouveau hook ou une variable env non sûre vers une politique large signifie donc une invite d'approbation au démarrage suivant de chaque développeur correspondant.

La clé `cli` s'appelait `settings` dans les versions antérieures. Cette orthographe est toujours acceptée comme alias, mais les nouveaux déploiements doivent utiliser `cli`.

<h4 id="precedence-with-other-managed-sources">
  Précédence avec d'autres sources gérées
</h4>

Si un appareil a également une `managed-settings.json` locale ou une politique livrée par MDM, les sources gérées ne fusionnent pas. La source de priorité la plus élevée fournit tous les paramètres de politique, classés dans cet ordre avec la priorité la plus élevée en premier :

1. L'[assistant de politique](/docs/fr/settings#compute-managed-settings-with-a-policy-helper)
2. Paramètres livrés par la passerelle
3. MDM, via le registre HKLM sur Windows ou un plist sur macOS
4. Le fichier `managed-settings.json`
5. Le registre HKCU, sur Windows uniquement

Les hôtes d'intégration peuvent fournir une politique via l'option SDK `managedSettings`. Elle est ignorée par défaut et s'applique uniquement lorsqu'une source gérée opte pour [`parentSettingsBehavior: "merge"`](/docs/fr/settings#available-settings), filtrée pour qu'elle puisse resserrer la politique mais pas l'assouplir.

L'exception est un petit ensemble de clés inter-sources, honorées lorsqu'une source admin les définit ; le niveau HKCU inscriptible par l'utilisateur est exclu :

* `sandbox.network.allowManagedDomainsOnly` et `sandbox.filesystem.allowManagedReadPathsOnly` : lorsqu'elles sont verrouillées, les listes d'autorisation correspondantes sont unies entre les sources
* [`allowAllClaudeAiMcps`](/docs/fr/settings#available-settings) : remplacement d'autorisation uniquement pour la liste d'autorisation du serveur MCP claude.ai
* `sandbox.bwrapPath` et `sandbox.socatPath` : chemins du système de fichiers vers les binaires d'aide [sandbox](/docs/fr/sandboxing)
* [`forceRemoteSettingsRefresh`](/docs/fr/server-managed-settings) : bloque le démarrage jusqu'à ce que les paramètres gérés distants soient fraîchement récupérés, donc une politique MDM ou fichier qui la définit est honorée même lorsqu'une charge utile distante mise en cache qui manque la clé est la source de priorité la plus élevée

Chaque autre clé, y compris `allowManagedPermissionRulesOnly` et `disableBypassPermissionsMode`, provient uniquement de la source de priorité la plus élevée. Voir [Précédence des paramètres](/docs/fr/settings#settings-precedence) pour la même règle sur la page des paramètres.

Les politiques de passerelle s'appliquent à chaque invocation de Claude Code sur la machine, y compris les exécutions non interactives `claude -p` et les sessions générées par le SDK Agent. Si la passerelle est inaccessible au démarrage, les sessions signées se terminent avec une erreur plutôt que de s'exécuter sans leur politique.

<Warning>
  `mcpServers` à l'intérieur du bloc `cli` d'une politique est rejeté au démarrage de la passerelle. La distribution MCP par groupe n'est pas disponible ; déployez les serveurs MCP via le `managed-mcp.json` basé sur fichier sur chaque appareil ou laissez les développeurs les ajouter localement.
</Warning>

<h3 id="telemetry">
  `telemetry`
</h3>

Le CLI envoie le protocole OpenTelemetry (OTLP) sur les métriques, journaux et, lorsqu'ils sont activés, les traces HTTP à la passerelle, qui les relaye textuellement à chaque destination configurée. Voir [Surveillance de l'utilisation](/docs/fr/monitoring-usage) pour les métriques et événements que le CLI émet.

Le CLI horodate chaque export avec l'identité de l'utilisateur authentifié, lue à partir du JWT émis par la passerelle : les attributs `user.id`, `user.email` et `user.groups`. L'attribution du coût et de l'utilisation par développeur fonctionne donc sans aucune configuration côté développeur.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Opt-in par signal. Par défaut : métriques uniquement.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Chaque destination opte pour `metrics`, `logs` et `traces` indépendamment, et la valeur par défaut est les métriques uniquement. Les signaux diffèrent en sensibilité :

  * **Métriques** : compteurs agrégés tels que les comptages de jetons, les comptages de demandes et la latence
  * **Journaux et traces** : peuvent porter des commandes bash complètes, des entrées d'outils et des chemins de fichiers, couvrant tout ce que Claude Code fait sur la machine d'un développeur

  Activez les journaux et les traces uniquement sur les destinations avec les contrôles d'accès et la politique de rétention que les données justifient.
</Warning>

La télémétrie est désactivée dans le CLI par défaut. Configurer `telemetry.forward_to` avec `listen.public_url` l'active. La passerelle pousse cinq variables env à chaque client connecté via `/managed/settings` :

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER=otlp`
* `OTEL_LOGS_EXPORTER=otlp`
* `OTEL_TRACES_EXPORTER=otlp`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`

Le point de terminaison poussé est construit à partir de l'URL publique, donc les métriques et les journaux n'ont besoin d'aucune configuration OTEL des développeurs ou des politiques. La configuration poussée est appliquée au niveau géré, remplaçant les variables `OTEL_*` qu'un développeur définit localement.

Les [traces](/docs/fr/monitoring-usage#traces-beta) nécessitent en outre `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` sur chaque client. La passerelle ne pousse pas cette variable, donc définissez-la via le bloc `env` d'une politique gérée. Elle n'est pas sur la liste sûre du CLI, donc la livrer via une politique est couverte par le même [dialogue d'approbation de sécurité](#managed) que le point de terminaison OTLP poussé déclenche déjà.

Les encodages OTLP protobuf et JSON sont relayés, et tout backend compatible OpenTelemetry fonctionne comme destination.

<h3 id="http-tuning">
  Réglage HTTP
</h3>

Quatre blocs optionnels de niveau supérieur, `access_control`, `limits`, `timeouts` et `rate_limits`, ajustent la surface HTTP. Les valeurs par défaut conviennent à la plupart des déploiements.

| Bloc             | Clé                                            | Par défaut | Description                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ---------------- | ---------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | vide       | Autorisation/refus IP entrant par adresse client, après résolution `trusted_proxies`. `deny_cidrs` est vérifié en premier ; un client qu'il correspond est rejeté même si `allow_cidrs` correspond également. Si `allow_cidrs` est non vide, la passerelle est par défaut refusée. `/healthz` et `/readyz` sont exempts de `allow_cidrs`.                                                                                     |
| `limits`         | `max_request_bytes`                            | 32 MiB     | Corps de demande entrant max ; les demandes surdimensionnées obtiennent `413` avant que le corps soit mis en mémoire tampon. Augmentez pour les demandes de fichiers ou d'images volumineux.                                                                                                                                                                                                                                  |
| `limits`         | `max_request_header_bytes`                     | non défini | Lorsqu'il est défini, les en-têtes surdimensionnés retournent `431`                                                                                                                                                                                                                                                                                                                                                           |
| `limits`         | `max_url_length`                               | non défini | Lorsqu'il est défini, une URL trop longue retourne `414`                                                                                                                                                                                                                                                                                                                                                                      |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000     | Attente max pour les en-têtes de réponse upstream (time to first byte). Le corps de la réponse s'écoule ensuite sans plafond de temps mural. S'applique au chemin upstream Anthropic direct ; chaque autre fournisseur est limité par le timeout du SDK du fournisseur.                                                                                                                                                       |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600   | Limite de débit par IP sur le point de terminaison d'autorisation d'appareil non authentifié. Augmentez pour une grande organisation derrière une IP de sortie partagée ou NAT. Ces limites s'appliquent uniquement au flux de connexion de subvention d'appareil, pas à l'inférence `/v1/messages`. Voir [Résistance à la force brute du code utilisateur](/docs/fr/claude-apps-gateway-deploy#user-code-brute-force-resistance). |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600   | Limite de débit par IP sur les soumissions `user_code` à `/device`                                                                                                                                                                                                                                                                                                                                                            |

<h2 id="complete-example">
  Exemple complet
</h2>

Cette configuration de référence complète exerce chaque section principale ; les blocs de [réglage HTTP](#http-tuning) gardent leurs valeurs par défaut. Copiez-la, supprimez ce dont vous n'avez pas besoin et remplissez vos valeurs. La configuration du [démarrage rapide](/docs/fr/claude-apps-gateway#quickstart) est une version minimale de celle-ci.

```yaml gateway.yaml theme={null}
# Exécuter avec :
#   claude gateway --config gateway.yaml
#
# La verbosité du journal opérationnel est contrôlée par la variable d'environnement
# CLAUDE_GATEWAY_LOG_LEVEL (info | warn | error ; par défaut info). Elle n'affecte pas
# les événements d'audit, qui sont toujours émis.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omettez le bloc tls lors de l'exécution derrière une entrée terminant TLS.
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # Requis lorsque l'émetteur est le serveur org Okta, dont les id_tokens
  # peuvent omettre l'e-mail et les groupes ; la passerelle les remplit à partir de /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta émet les groupes uniquement lorsque la portée `groups` est demandée et que
  # le filtre de réclamation de groupes de l'application les autorise. La politique des entrepreneurs ci-dessous
  # correspond sur les groupes, donc la portée est demandée ici.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Rôles d'application Entra : utiliser `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5

# Active /v1/organizations/spend_limits (reflète l'API Admin Anthropic)
# et l'application de dépenses par développeur sur /v1/messages. Omettez pour désactiver.
# Les plafonds eux-mêmes sont définis via l'API admin, pas ici.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: demander une augmentation à https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # Contraindre l'option du sélecteur par défaut à availableModels au lieu
        # de la valeur par défaut du niveau, donc les entrepreneurs ne reçoivent pas un 400 sur la valeur par défaut.
        enforceAvailableModels: true
        # allow approuve automatiquement ces outils ; il ne bloque pas le reste.
        # Ajoutez des règles de refus pour restreindre les outils.
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  Paramètres gérés côté client
</h2>

Tout ce qui précède configure le serveur de passerelle. Pointer les machines des développeurs vers celui-ci est configuré séparément, sur chaque appareil, via les [paramètres gérés](/docs/fr/settings#settings-files) de Claude Code. La passerelle ne peut pas pousser ces clés elle-même, car ce sont elles qui disent au client où se trouve la passerelle.

Pour le CLI, définissez les deux clés dans le `managed-settings.json` par système d'exploitation :

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Déployez ce fichier sur chaque appareil, généralement via votre plateforme MDM. Le chemin du fichier diffère selon la plateforme :

| Plateforme   | Chemin                                                                                                                           |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| macOS        | `/Library/Application Support/ClaudeCode/managed-settings.json`, ou le domaine des préférences gérées `com.anthropic.claudecode` |
| Linux et WSL | `/etc/claude-code/managed-settings.json`                                                                                         |
| Windows      | `C:\Program Files\ClaudeCode\managed-settings.json`, ou Group Policy via le registre HKLM                                        |

`forceLoginGatewayUrl` et la valeur `"gateway"` de `forceLoginMethod` sont honorés uniquement à partir du niveau géré contrôlé par l'administrateur. Un développeur les définissant dans son propre `~/.claude/settings.json` n'a aucun effet.

<h2 id="related">
  Connexes
</h2>

* [Aperçu de la passerelle Claude apps](/docs/fr/claude-apps-gateway) : démarrage rapide et connexion des développeurs
* [Guide de déploiement](/docs/fr/claude-apps-gateway-deploy) : configuration IdP, image de conteneur, Kubernetes et Cloud Run, et opérations
* [Limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits) : plafonds par développeur et API Admin
