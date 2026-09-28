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
* [`pricing`](#pricing) : tarifs contractuels et multiplicateur de remise pour le compteur de dépenses et pour les chiffres de coût que les développeurs voient
* [`models`](#models) et `auto_include_builtin_models` : liste de modèles curée par l'administrateur et IDs par upstream
* [`managed`](#managed) : politiques de paramètres gérés par groupe IdP
* [`telemetry`](#telemetry) : transfert OTLP vers votre pile d'observabilité
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning) : autorisation/refus IP, plafonds de taille de requête, time-to-first-byte upstream et limites de connexion par IP
* [`load_test_mode`](#load_test_mode) : tester en charge la passerelle sans appeler un fournisseur de modèle

<h2 id="secret-expansion">
  Expansion des secrets
</h2>

N'écrivez pas de secrets tels que `client_secret`, `jwt_secret` ou `postgres_url` directement dans `gateway.yaml`. Référencez-les avec l'une des formes ci-dessous, et la passerelle résout la valeur au démarrage à partir d'une variable d'environnement ou d'un fichier :

| Forme           | Résout à                                                                                                                                                                                                                                                                                                              | Utiliser pour                                                                 |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `${VAR}`        | La variable d'environnement `VAR`. Le démarrage échoue si non défini.                                                                                                                                                                                                                                                 | Variables d'environnement de conteneur, AWS Secrets Manager via injection env |
| `${file:/path}` | Contenu du fichier à ce chemin absolu, coupé. La référence doit être la valeur entière du champ : contrairement à `${VAR}`, elle n'est pas développée à l'intérieur d'une chaîne plus longue, donc pour un mot de passe de base de données, définissez `store.password` plutôt que de l'intégrer dans `postgres_url`. | Montages de volume Kubernetes Secret, Vault Agent, SOPS                       |

<h2 id="required-sections">
  Sections requises
</h2>

<h3 id="listen">
  `listen`
</h3>

Le bloc `listen` contrôle où la passerelle sert : l'adresse de liaison et le port, l'origine visible en externe, et la terminaison TLS optionnelle.

| Champ                  | Requis                      | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `host`                 | Non                         | Adresse de liaison. Par défaut `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `port`                 | Non                         | Port de liaison. Par défaut `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `public_url`           | Sauf si `host` est loopback | L'origine `https://` visible en externe, utilisée pour construire le `redirect_uri` IdP et les métadonnées de découverte. Requis chaque fois que `host` n'est pas une adresse loopback, que TLS se termine à un proxy tel qu'un ALB, Ingress ou Cloud Run ou à la passerelle elle-même via `tls`, car la passerelle ne dérive jamais sa propre origine à partir des en-têtes `X-Forwarded-*` ; ils sont spoofables par le client. L'amorçage échoue sans lui. `trusted_proxies` ci-dessous régit uniquement la résolution de l'IP du client. Également requis pour activer la [télémétrie](#telemetry), car la passerelle construit le point de terminaison OTLP qu'elle pousse aux clients à partir de cette URL. |
| `tls.cert` / `tls.key` | Non                         | Chemins PEM si la passerelle termine TLS elle-même                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `trusted_proxies`      | Non                         | CIDRs ou IPs des équilibreurs de charge devant la passerelle. Lorsqu'il est défini, la passerelle fait confiance à `X-Forwarded-For` uniquement à partir de ces pairs et enregistre l'IP client réelle pour la limitation de débit par IP et l'audit. Équivalent à nginx `set_real_ip_from`. Les entrées `X-Forwarded-For` écrites en tant que `ipv4:port` ou `[ipv6]:port`, comme certains équilibreurs de charge le font, sont lues avec le port supprimé. Une adresse IPv6 avec un port ajouté et sans crochets peut être lue comme une adresse différente ou ne pas être lue du tout, donc désactivez l'option de port sur tout proxy qui écrit cette forme.                                                   |

<h3 id="oidc">
  `oidc`
</h3>

Le bloc `oidc` connecte la passerelle à votre fournisseur d'identité et décide qui peut se connecter. Il nomme l'émetteur et le client OAuth, mappe les réclamations qui portent l'e-mail et les groupes, et restreint la connexion par domaine d'e-mail ou groupe.

OpenID Connect (OIDC) est le protocole SSO que la passerelle utilise avec votre fournisseur d'identité ; voir [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup) pour ce qu'il faut enregistrer du côté IdP.

| Champ                           | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Oui    | Base de découverte OIDC. Doit servir la découverte à `/.well-known/openid-configuration`. Utilisez HTTPS en production ; la passerelle accepte un émetteur `http://`. Un émetteur de boucle locale tel que `http://localhost:8081` est rejeté par la [garde SSRF](/docs/fr/claude-apps-gateway-deploy#threat-model-summary) sauf si `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` est défini dans l'environnement de la passerelle.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `client_id` / `client_secret`   | Oui    | De votre enregistrement de client OAuth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `allowed_email_domains`         | Non    | Rejeter les id\_tokens dont la réclamation `email` n'est pas dans l'un de ces domaines, insensible à la casse. Défense en profondeur contre les erreurs de configuration IdP multi-locataires. Indépendamment de ce paramètre, un id\_token dont la réclamation `email_verified` est explicitement `false` est toujours rejeté.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowed_groups`                | Non    | Restreindre la connexion aux membres de ces groupes IdP, appariés par rapport à `groups_claim`. Un utilisateur dans un domaine d'e-mail autorisé mais dans aucun de ces groupes est rejeté. Nécessite que l'IdP émette la réclamation de groupes. L'appariement est une comparaison de chaîne exacte et sensible à la casse par rapport aux valeurs de cette réclamation, et la passerelle n'étend pas les groupes imbriqués : pour admettre les membres d'un sous-groupe, listez le sous-groupe ici ou configurez l'IdP pour émettre l'appartenance aplatie.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `groups_claim`                  | Non    | Quelle réclamation id\_token porte l'appartenance au groupe. Par défaut `groups`. Microsoft Entra émet les rôles d'application sous `roles`. Accepte une clé plate ou un pointeur JSON RFC 6901 tel que `/resource_access/gateway/roles` pour les réclamations imbriquées.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `google_groups`                 | Non    | Rechercher les groupes de l'utilisateur connecté via l'API Google Workspace Admin SDK Directory, car le id\_token de Google ne porte aucune réclamation de groupes. Définissez `service_account_json_path` sur un fichier de clé de compte de service avec délégation à l'échelle du domaine sur la portée `https://www.googleapis.com/auth/admin.directory.group.readonly`, et `admin_email` sur un administrateur Workspace que le compte de service usurpe ; l'API Directory nécessite un sujet administrateur réel. Les adresses e-mail de groupe de chaque utilisateur deviennent leur réclamation de groupes, donc `allowed_groups` et `managed.policies.match.groups` correspondent sur les e-mails de groupe.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `email_claim`                   | Non    | Quelle réclamation id\_token porte l'e-mail de l'utilisateur. Par défaut `email`. Certains IdPs, tels que ADFS et Entra B2C, émettent `upn` ou `preferred_username` à la place. Accepte une clé plate, un pointeur JSON ou une liste de clés de secours où la première clé présente est utilisée.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `scopes`                        | Non    | Remplacement complet des portées OIDC que la passerelle demande. Par défaut `[openid, profile, email, offline_access]`. Définissez lorsque votre IdP rejette les portées qu'il ne reconnaît pas, ou nécessite une portée personnalisée pour émettre des groupes ou un e-mail. Doit inclure `openid`. Supprimer `offline_access` désactive les jetons d'actualisation, donc les développeurs réexécutent la connexion au navigateur tous les `session.ttl_hours`. Voir [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup) pour les recettes de portée par IdP telles que le flux de jeton d'actualisation de Google.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `scope_on_refresh`              | Non    | Envoyer également `scope`, avec la même liste que la demande de connexion, lorsque la passerelle échange un jeton d'actualisation. Par défaut `false` : la demande d'actualisation omet `scope`. La plupart des IdPs retournent un id\_token à chaque actualisation et n'en ont pas besoin. Définissez `true` lorsque votre IdP retourne un id\_token lors de l'actualisation uniquement s'il est demandé à nouveau pour `openid`, ce qu'Okta documente pour sa subvention d'actualisation. Sans id\_token, chaque actualisation dépend du point de terminaison userinfo de l'IdP acceptant le jeton d'accès actualisé. Si vous contrôlez la connexion ou les politiques de correspondance sur les groupes et que le id\_token de votre IdP au moment de l'actualisation les omet, définissez également `userinfo_fallback: true` pour que la passerelle les remplisse à partir du point de terminaison userinfo. Un IdP qui a accordé moins de portées que demandé peut rejeter l'actualisation avec `invalid_scope`, y compris pour les sessions existantes si vous ajoutez des entrées à `scopes` pendant que ceci est activé. Décochez la clé si les actualisations commencent à échouer à `token_endpoint` après l'avoir définie. Nécessite Claude Code v2.1.260 ou ultérieur sur le serveur de la passerelle. |
| `extra_auth_params`             | Non    | Paramètres de requête supplémentaires ajoutés à la demande d'autorisation IdP, textuellement. C'est le mécanisme de remplacement pour le comportement spécifique à l'IdP, tel que `access_type: offline` pour les jetons d'actualisation Google, `domain_hint` pour certains locataires Entra, ou `acr_values` pour les flux d'escalade. Ne peut pas remplacer les paramètres de protocole gérés par la passerelle : `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` et `client_id`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `userinfo_fallback`             | Non    | Lorsque le id\_token omet l'e-mail ou les groupes, les récupérer à partir de `/userinfo`. Nécessaire pour les jetons d'accès légers Keycloak, le serveur org Okta et les jetons minimaux ADFS. Le id\_token reste faisant autorité ; userinfo remplit uniquement les lacunes. Par défaut `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `use_pkce`                      | Non    | Envoyer un défi PKCE (S256) sur la demande d'autorisation. Par défaut `true`. Définissez `false` uniquement si votre IdP rejette PKCE pour ce client confidentiel.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `clock_skew_seconds`            | Non    | Tolérer la dérive d'horloge lors de la validation des réclamations de temps id\_token. Par défaut `0`, ce qui est strict. Augmentez si vous voyez des erreurs « token expired / not yet valid » juste après la connexion en raison d'une dérive d'horloge hôte/IdP.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `token_endpoint_auth_method`    | Non    | Remplacer la méthode d'authentification du point de terminaison de jeton. Accepte `client_secret_basic` ou `client_secret_post`. Négocié automatiquement par défaut.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `id_token_signed_response_alg`  | Non    | Algorithme de signature id\_token attendu. Par défaut `RS256`. Définissez pour les IdPs qui signent avec ES256, PS256 ou EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `additional_authorized_parties` | Non    | Valeurs `azp` supplémentaires à accepter au-delà de `client_id`, pour les flux de courtier Keycloak et d'échange de jetons                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `discovery_url`                 | Non    | Récupérer le document de découverte à partir de cette URL au lieu de le dériver de `issuer`, pour les IdPs derrière un proxy qui réécrit l'hôte émetteur. Le chemin doit contenir `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `use_proxy`                     | Non    | Envoyer les propres demandes IdP de la passerelle via le proxy de transfert dans `HTTPS_PROXY` ou `HTTP_PROXY`, en honorer `NO_PROXY`. `false` garde ces demandes directes. Nécessite v2.1.227 ou ultérieur ; voir [Demandes IdP via un proxy de transfert](#idp-requests-through-a-forward-proxy) ci-dessous.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `form_action_origins`           | Non    | Origines supplémentaires pour la directive `Content-Security-Policy: form-action` de la page `/device`. La passerelle autorise déjà `'self'` et l'origine du `authorization_endpoint` découverte, mais Chrome applique `form-action` à toute la chaîne de redirection. Si votre IdP redirige via un deuxième hôte, tel que Azure AD fédéré à ADFS, Okta hub-spoke ou un intercepteur SSO d'entreprise, listez chaque origine par laquelle la demande d'autorisation peut rediriger.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `ca_cert_pem`                   | Non    | Le certificat CA PEM lui-même, pas un chemin vers un fichier. Il remplace le magasin de confiance système pour les demandes IdP uniquement. Pour charger un fichier monté, écrivez `${file:/etc/gateway/idp-ca.pem}`. Utilisez pour Keycloak ou Dex derrière une PKI d'entreprise.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h4 id="idp-requests-through-a-forward-proxy">
  Demandes IdP via un proxy de transfert
</h4>

Les upstreams d'inférence honorent `HTTPS_PROXY` et `HTTP_PROXY` sur chaque version. Les propres demandes de la passerelle à l'IdP, découverte, JWKS, jeton et userinfo, vont directement sauf si vous définissez `oidc.use_proxy: true`, ce qui nécessite v2.1.227 ou ultérieur. Lorsqu'une variable proxy est définie, `use_proxy` n'est pas défini et l'émetteur n'est pas couvert par `NO_PROXY`, la passerelle garde ces demandes directes et enregistre un avis au démarrage vous demandant de choisir ; `use_proxy: false` les garde directes et fait taire l'avis.

Avec `use_proxy: true`, le pod résout lui-même le nom d'hôte de chaque point de terminaison IdP et demande au proxy de `CONNECT` à l'adresse IP résolue, donc le proxy doit accepter `CONNECT` à l'adresse IP de chaque hôte que le document de découverte nomme, pas seulement l'émetteur. Utilisez une URL de proxy `http://`. `ca_cert_pem` et la [garde SSRF](/docs/fr/claude-apps-gateway-deploy#threat-model-summary) s'appliquent également sur le chemin proxifié.

[Egress proxy uniquement](#proxy-only-egress) change les deux : pendant qu'il est actif, les demandes IdP suivent le proxy sauf si vous définissez `use_proxy: false`, et la passerelle remet au proxy chaque nom d'hôte IdP sans le résoudre d'abord.

<h4 id="proxy-only-egress">
  Egress proxy uniquement
</h4>

Définissez `CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1` dans l'environnement de la passerelle, à côté de `HTTPS_PROXY`, lorsque le pod atteint d'autres hôtes uniquement via ce proxy de transfert et ne peut pas résoudre les noms DNS publics lui-même, ou lorsque le proxy refuse `CONNECT` à une adresse IP. Nécessite v2.1.277 ou ultérieur. C'est une variable d'environnement plutôt qu'une clé `gateway.yaml` pour que rien dans le fichier de configuration ne puisse assouplir la vérification d'adresse de la passerelle.

```bash theme={null}
export HTTPS_PROXY=http://proxy.corp.example.com:3128
export NO_PROXY=
export no_proxy=
export CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1
```

La passerelle enregistre une ligne `network:` au démarrage pendant que l'egress proxy uniquement est actif.

Chaque ligne ci-dessous est une classe de demande sortante sur une passerelle avec `HTTPS_PROXY` défini, par défaut et pendant que l'egress proxy uniquement est actif.

| Demande sortante                                                                                                                     | Par défaut                                                                                                                                                                | Egress proxy uniquement actif                                                          |
| ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| Upstreams `provider: anthropic`, échange de jeton Workload Identity Federation, exports `telemetry.forward_to`                       | Résolus et vérifiés localement, puis `CONNECT` à l'adresse IP vérifiée via le proxy. Un collecteur de télémétrie listé dans `NO_PROXY` est atteint directement à la place | Nom d'hôte remis au proxy                                                              |
| Découverte IdP, JWKS, jeton et userinfo                                                                                              | Direct sauf si [`oidc.use_proxy: true`](#idp-requests-through-a-forward-proxy), puis `CONNECT` à l'adresse IP vérifiée                                                    | Nom d'hôte remis au proxy, sauf si `oidc.use_proxy: false` garde un IdP interne direct |
| Upstreams Amazon Bedrock, Claude Platform on AWS, Agent Platform de Google Cloud et Microsoft Foundry ; recherches de groupes Google | Nom d'hôte remis au proxy                                                                                                                                                 | Inchangé                                                                               |

L'egress proxy uniquement reste désactivé sauf si l'environnement de la passerelle répond à ces trois conditions :

* `HTTPS_PROXY` ou `HTTP_PROXY` est défini.
* `NO_PROXY` et `no_proxy` sont vides. Si votre plateforme injecte l'un ou l'autre dans les pods, définissez les deux sur une valeur vide sur le conteneur de la passerelle. Lister un collecteur de télémétrie dans `NO_PROXY` garde l'egress proxy uniquement désactivé.
* `CLAUDE_GATEWAY_ALLOW_LOOPBACK` n'est pas activé. Un collecteur ou IdP sur la propre boucle locale du pod ne peut pas être combiné avec l'egress proxy uniquement, car une adresse loopback remise au proxy serait la propre boucle locale de l'hôte proxy, donc donnez à ces services une adresse que le proxy peut atteindre à la place. Pour la même raison, la passerelle refuse les noms de style `localhost` directement pendant que l'egress proxy uniquement est actif.

Lorsqu'une de ces conditions n'est pas remplie, la passerelle enregistre un avertissement au démarrage nommant la variable qui l'a arrêtée et garde le comportement par défaut.

Une fois que l'egress proxy uniquement est actif, autorisez chaque destination dans le proxy, y compris un collecteur interne et tout hôte configuré par adresse IP. Vous pouvez toujours garder un IdP interne direct avec [`oidc.use_proxy: false`](#idp-requests-through-a-forward-proxy).

<Warning>
  Activez ceci uniquement lorsque la liste d'autorisation du proxy est au moins aussi stricte que la vérification propre de la passerelle. Le proxy doit refuser les points de terminaison de métadonnées cloud tels que `169.254.169.254` et `metadata.google.internal`, les adresses link-local et la propre boucle locale de l'hôte proxy, et il doit les refuser par l'adresse à laquelle un nom se résout, pas seulement par nom, car la passerelle ne capture plus un nom d'hôte qui se résout à l'un d'eux. Un proxy qui se connecte n'importe où où on lui demande supprime la [garde SSRF](/docs/fr/claude-apps-gateway-deploy#threat-model-summary) de la passerelle pour ces demandes.
</Warning>

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

| Champ                     | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`            | Oui    | URL `postgres://` ou `postgresql://`. Requis : le rendez-vous de subvention d'appareil, où le rappel du navigateur écrit et le CLI d'interrogation lit, a besoin d'un état entre répliques. La passerelle exécute ses propres migrations de schéma au démarrage et à la mise à niveau, donc le rôle a besoin de droits pour créer et modifier les tables sur le schéma cible. Voir [Mises à jour](/docs/fr/claude-apps-gateway-deploy#upgrades) et [Postgres](/docs/fr/claude-apps-gateway-deploy#postgres). |
| `username`                | Non    | Remplace l'utilisateur dans `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `password`                | Non    | Identifiant de base de données. Définissez-le ici plutôt que dans `postgres_url` pour que l'identifiant reste hors de l'URL. Accepte n'importe quel caractère et prend la priorité sur les identifiants d'URL.                                                                                                                                                                                                                                                                                     |
| `max_connections`         | Non    | Taille du pool de connexions Postgres par réplique. Par défaut `5`, ce qui est conservateur et convivial pour les bases de données partagées. Avec les [limites de dépenses](#admin) activées, le chemin chaud effectue quelques opérations par demande d'inférence, donc augmentez-le pour une base de données dédiée sous charge, et gardez les répliques × ceci en dessous du `max_connections` de la base de données.                                                                          |
| `connect_timeout_seconds` | Non    | Secondes que la passerelle attend lorsqu'elle ouvre une connexion Postgres. Un nombre entier de `1` à `60`, par défaut `5`. Augmentez-le si les tentatives de connexion expirent lorsqu'une nouvelle instance de passerelle démarre. Nécessite Claude Code v2.1.274 ou ultérieur sur le serveur de la passerelle. Les versions antérieures refusent de démarrer lorsque la clé est définie.                                                                                                        |

Pour le développement local, pointez `postgres_url` sur un conteneur Postgres jetable, par exemple `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` est une liste ordonnée. La passerelle transfère l'inférence au premier upstream qui résout le modèle demandé.

Sur `5xx`, `429`, `401`, `403`, `404`, ou timeout, la passerelle bascule vers le suivant ; les autres `4xx` ne le font pas, car ces erreurs sont attribuables à la demande plutôt qu'à l'upstream. Un `401` ou `403` signifie que l'identifiant propre de la passerelle a échoué contre cet upstream. Un `404` signifie que cet upstream ne sert pas le modèle demandé, donc un upstream ultérieur dans la liste peut toujours le faire.

Si vous définissez `forward_user_identity: true` sur un upstream, un `429` qu'il retourne à une demande qui portait l'e-mail du développeur ne bascule pas. Voir [comment un déni de limite par utilisateur atteint le développeur](#per-user-identity-headers-for-a-proxy-you-run).

Le basculement sur `404` nécessite la passerelle v2.1.198 ou ultérieure. Les versions antérieures retournaient le premier `404` au client même lorsqu'un upstream ultérieur dans la liste servait le modèle.

Plusieurs upstreams du même fournisseur doivent définir un `name:` distinct.

Les clients Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform et Microsoft Foundry sont construits une fois au démarrage, et leurs SDKs actualisent les identifiants en interne, donc la rotation des identifiants cloud ne nécessite pas un redémarrage. Les clés API Anthropic statiques et les porteurs sont lus au démarrage ; voir [Anthropic API](#anthropic-api).

<h4 id="upstream-error-messages">
  Messages d'erreur d'upstream
</h4>

La passerelle retourne la réponse d'erreur d'un upstream, ou son propre `502`, selon la façon dont les upstreams ont répondu :

* **Un upstream a retourné un statut sur lequel la passerelle ne [bascule pas](#multiple-upstreams)** : la réponse de cet upstream. La passerelle n'essaie pas d'autres upstreams.
* **Chaque upstream que la passerelle a essayé a échoué d'une manière sur laquelle elle [bascule](#multiple-upstreams)** : le dernier `429`. Lorsqu'aucun n'a retourné un `429`, la passerelle préfère, dans l'ordre, le dernier `401` ou `403`, le dernier `404` et le dernier `501`. Lorsqu'aucun n'a retourné l'un de ceux-ci, le propre `502` de la passerelle, `all upstreams failed (N attempted)`, où N compte chaque entrée dans [`upstreams`](#upstreams), y compris les entrées que la passerelle a ignorées car elles ne servent pas le modèle demandé.

Lorsque la passerelle retourne la réponse d'un upstream, elle garde le code de statut de l'upstream. Qu'elle garde le message de l'upstream dépend du fournisseur. Le corps d'erreur d'un upstream Anthropic API atteint le développeur inchangé.

Les upstreams Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform et Microsoft Foundry peuvent nommer vos IDs de compte, ARNs de rôle et IDs de projet dans leur texte d'erreur. La passerelle enregistre ce texte complet dans le [journal opérationnel](/docs/fr/claude-apps-gateway-deploy#logs). Ce que le développeur voit de ces upstreams dépend du rejet :

* `400` ou `413` dans l'enveloppe d'erreur standard d'Anthropic : le message propre de l'upstream, tel que `prompt is too long`. Claude Platform on AWS, Agent Platform et Microsoft Foundry retournent cette enveloppe pour les rejets d'API de modèle.
* `400` ou `413` dans la propre forme du fournisseur : un jeton `capability_rejected:`. Lorsque la passerelle ne peut pas classer le rejet, `upstream rejected the request` sur un `400` ou `request too large for this upstream` sur un `413`.
* Tout autre statut : copie générique par statut, tel que `upstream rate limit exceeded` sur un `429`.

Par exemple, la passerelle remplace le `Input is too long for requested model.` d'Amazon Bedrock par `capability_rejected: prompt_too_long`. Claude Code [se compacte automatiquement](/docs/fr/errors#prompt-is-too-long) sur ce jeton, comme il le fait sur `prompt is too long`.

Garder le message `400` ou `413` d'un upstream cloud, ou le remplacer par un jeton `capability_rejected:`, nécessite la passerelle v2.1.233 ou ultérieure.

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

<a id="per-user-identity-headers-for-a-proxy-you-run" />

<h5 id="per-user-identity-headers-for-a-proxy-you-run">
  En-têtes d'identité par utilisateur pour un proxy que vous exécutez
</h5>

Vous pouvez pointer le `base_url` d'un upstream `provider: anthropic` vers un proxy que vous exécutez au lieu de l'API Anthropic. Pour dire à ce proxy quel développeur a envoyé chaque demande, définissez `forward_user_identity: true` sur cet upstream. Le proxy peut alors attribuer les dépenses par développeur. Nécessite une passerelle exécutant Claude Code v2.1.233 ou ultérieur.

Par exemple, pour un proxy à `upstream-gateway.internal.example.com` :

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # par défaut false
```

La passerelle ajoute ces en-têtes à chaque demande qu'elle transfère à cet upstream.

| En-tête                       | Valeur                                                                  |
| ----------------------------- | ----------------------------------------------------------------------- |
| `x-litellm-end-user-id`       | L'e-mail du développeur, lorsque l'IdP l'a fourni.                      |
| `x-claude-gateway-user-id`    | Le sujet IdP du développeur, à partir de la réclamation `sub` du jeton. |
| `x-claude-gateway-user-email` | L'e-mail du développeur, lorsque l'IdP l'a fourni.                      |

Lorsque le jeton IdP ne porte pas d'e-mail, la passerelle envoie uniquement `x-claude-gateway-user-id` et omet les deux en-têtes d'e-mail. Si votre IdP met l'e-mail dans une réclamation différente, définissez [`oidc.email_claim`](#oidc) sur cette réclamation.

Lorsque votre proxy répond `429` à une demande qui portait l'e-mail du développeur, la passerelle retourne cette réponse au développeur telle quelle au lieu de basculer vers le prochain upstream, donc votre limite de budget ou de débit par utilisateur du proxy tient. Les autres réponses du proxy suivent les [règles de basculement](#upstreams) ordinaires. Si le jeton IdP d'un développeur ne porte pas d'e-mail, la passerelle transfère ses demandes sans les en-têtes d'e-mail, donc un `429` à l'une de ces demandes compte comme capacité d'upstream et bascule. Avant v2.1.267 sur le serveur de la passerelle, chaque `429` basculait.

Définissez `forward_user_identity` uniquement sur un upstream dont le `base_url` est un proxy que vous exploitez. La passerelle envoie les e-mails des développeurs à quel que soit le serveur que ce `base_url` nomme. Si le `base_url` est l'API Anthropic, qui est la valeur par défaut, la passerelle refuse de démarrer.

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

| Configuration         | Comment                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Permissions IAM       | Accordez au principal de la passerelle `bedrock:InvokeModel` et `bedrock:InvokeModelWithResponseStream` sur les ARNs de profil d'inférence et les ARNs de modèle de fondation sous-jacents. Pour le catalogue intégré dans les régions US : `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` et `arn:aws:bedrock:*::foundation-model/anthropic.*`. Accordez également `bedrock:CountTokens` sur les ARNs de modèle de fondation. La passerelle l'utilise, sans frais, pour compter les jetons d'entrée d'une demande que le client a abandonnée, donc les [limites de dépenses](#admin) restent exactes. Sans cela, la passerelle revient à une demande Bedrock d'un jeton pour ce compte. |
| Accès au modèle       | Amazon Bedrock active l'accès au modèle par défaut dans les régions commerciales. La porte au niveau du compte restante est celle d'Anthropic : un formulaire d'utilisation unique. Si personne dans votre compte AWS ne l'a soumis, ouvrez la console Amazon Bedrock, sélectionnez un modèle Anthropic dans le catalogue de modèles et complétez le formulaire. Voir [Soumettre les détails du cas d'utilisation](/docs/fr/amazon-bedrock#1-submit-use-case-details) pour le formulaire AWS Organizations et les permissions dont le soumetteur a besoin.                                                                                                                                                           |
| EKS (IRSA)            | Créez un rôle IAM avec la politique ci-dessus et une politique de confiance pour le fournisseur OIDC de votre cluster limité au compte de service de la passerelle. Annotez le compte de service avec `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` le récupère.                                                                                                                                                                                                                                                                                                                                                                                                            |
| ECS / EC2             | Attachez le rôle IAM à la définition de tâche ou au profil d'instance. `auth: {}` le récupère.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| N'importe où ailleurs | Passez les identifiants via les variables env `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` et `AWS_SESSION_TOKEN`, ou définissez-les explicitement dans `auth:` avec expansion `${VAR}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Région                | `region:` est la région du point de terminaison API. Les profils d'inférence inter-régions routent à travers la géographie (US, EU, APAC) indépendamment de celui que vous choisissez. Pour les régions non-US ou les ARNs de débit provisionné, ajoutez un bloc [`models:`](#models) avec les bons IDs par upstream.                                                                                                                                                                                                                                                                                                                                                                                           |

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

<h4 id="static-headers-on-upstream-requests">
  En-têtes statiques sur les demandes d'upstream
</h4>

Pour ajouter des en-têtes fixes aux demandes que la passerelle envoie à un upstream, définissez `headers:` sur cet upstream. Utilisez-le lorsqu'un proxy que vous exécutez devant le fournisseur route ou attribue le trafic par un en-tête.

`headers:` nécessite Claude Code v2.1.277 ou ultérieur sur le serveur de la passerelle. Une passerelle antérieure refuse de démarrer lorsqu'elle trouve la clé. Mettez à niveau chaque réplique avant d'ajouter la clé, et supprimez la clé avant de revenir à une version antérieure.

Les en-têtes vont au serveur que `base_url` nomme, ou au point de terminaison propre du fournisseur lorsque `base_url` n'est pas défini. Le fournisseur les reçoit également sauf si votre proxy les supprime.

Cet exemple atteint un upstream `provider: vertex` via un proxy à `upstream-proxy.internal.example.com`. Il définit l'en-tête `x-source` que le proxy lit, et envoie un jeton de la variable d'environnement `PROXY_TOKEN` en tant que `x-proxy-token` :

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    base_url: https://upstream-proxy.internal.example.com
    auth: {}
    headers:
      x-source: claude-apps-gateway
      x-proxy-token: ${PROXY_TOKEN}
```

Les valeurs sont du texte ASCII imprimable sans espace à chaque extrémité. Citez un nombre, `true` ou `false` pour que YAML le lise comme du texte.

Pour garder un secret hors du fichier de configuration, utilisez [l'expansion de secret](#secret-expansion) pour charger la valeur à partir d'une variable d'environnement avec `${VAR}` ou à partir d'un fichier avec `${file:/path}`. Un `${VAR}` qui se résout à une valeur vide arrête le démarrage de la passerelle.

`headers:` fonctionne sur chaque fournisseur, et chaque upstream envoie uniquement le sien.

Pas chaque demande que la passerelle envoie à un upstream les porte :

| Demande que la passerelle envoie à cet upstream                                    | Porte `headers:`                              |
| ---------------------------------------------------------------------------------- | --------------------------------------------- |
| `/v1/messages`, streaming ou non, et `/v1/messages/count_tokens`                   | Oui                                           |
| Une demande qui a basculé à partir d'un autre upstream                             | Oui, uniquement le `headers:` de cet upstream |
| L'appel `CountTokens` d'Amazon Bedrock pour une demande que le client a abandonnée | Non                                           |
| L'échange de jeton Workload Identity Federation                                    | Non                                           |

Sur un upstream Amazon Bedrock ou Claude Platform on AWS qui signe les demandes avec AWS SigV4, ces en-têtes font partie de la signature, donc votre proxy doit les transmettre inchangés.

Si vous utilisez un nom que la passerelle réserve, elle refuse de démarrer, et l'erreur de démarrage nomme l'en-tête. Les noms réservés incluent :

* `authorization` et `x-api-key`
* `host`, `content-type` et `user-agent`
* Tout nom commençant par `anthropic-`, `x-goog-`, `x-amz-` ou `x-amzn-`

<h4 id="multiple-upstreams">
  Plusieurs upstreams
</h4>

Le même fournisseur peut apparaître plus d'une fois avec un `name:` distinct. Cela couvre différentes régions, différents comptes via différentes chaînes d'identifiants, débit provisionné par rapport à la demande, et basculement inter-fournisseur.

La passerelle essaie les upstreams dans l'ordre. `5xx`, `429`, `401`, `403`, `404`, timeouts et point de terminaison manquant (`501`) basculent ; les autres `4xx` ne le font pas.

`429` est la capacité par upstream, donc l'épuisement du débit provisionné (PT) bascule vers la demande. Si vous définissez [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run) sur un upstream, un `429` à une demande qui portait l'e-mail du développeur est un déni par utilisateur au lieu et ne bascule pas.

Chaque demande commence au premier upstream. Une demande atteint un upstream ultérieur uniquement lorsque chaque upstream devant lui a échoué ou ne sert pas le modèle demandé.

La passerelle ne garde aucun enregistrement des upstreams défaillants, donc pendant qu'un upstream est en panne, chaque demande qui l'atteint l'essaie toujours et attend qu'il échoue avant de passer au suivant.

Pour un upstream Anthropic API, [`timeouts.upstream_ttfb_ms`](#http-tuning) limite l'attente sur un upstream en panne. Ce paramètre ne s'applique pas aux autres fournisseurs, où la passerelle attend jusqu'à une heure pour qu'un upstream commence à répondre.

`404` est la disponibilité du modèle par upstream, donc un upstream qui n'a pas activé un modèle ne bloque pas un upstream ultérieur qui le sert. Un upstream qui ne peut pas résoudre le modèle demandé est ignoré sans un aller-retour réseau.

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

# Les IDs de modèle par upstream sont indexés sur le `name:` de l'upstream.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Levier                           | Comment                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Différentes régions              | Un upstream Bedrock par région, chacun avec sa propre `region:`. Avec [`auto_include_builtin_models: true`](#models) les profils d'inférence inter-régions routent automatiquement ; pour les déploiements épinglés à la région, utilisez un bloc `models:`.                                                                                                                                                                                                                                                             |
| Différents comptes               | Un upstream Bedrock par compte, chacun avec ses propres identifiants dans `auth:`. La chaîne par défaut (`auth: {}`) utilise l'identité du pod ; pour un deuxième compte, définissez des identifiants explicites ou un jeton porteur.                                                                                                                                                                                                                                                                                    |
| Débit provisionné                | Mappez le modèle à l'ARN de débit provisionné dans `models:` pour le nom de cet upstream. Les autres upstreams gardent l'ID à la demande, donc la capacité PT est épuisée avant de basculer.                                                                                                                                                                                                                                                                                                                             |
| Points de terminaison VPC / FIPS | Définissez `base_url:` sur l'upstream vers votre URL de point de terminaison VPC ou FIPS                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Routage limité au modèle         | Seul un modèle personnalisé `id`, celui qui n'est pas un modèle Claude intégré, ignore les upstreams absents de sa carte `upstream_model:`. La passerelle essaie les modèles intégrés sur chaque upstream dans l'ordre et utilise l'ID par défaut du fournisseur où la carte n'a pas d'entrée, donc pour les modèles intégrés la carte change quel ID un upstream reçoit plutôt que s'il est essayé ; un upstream qui rejette l'ID suit les mêmes [règles de basculement](#upstreams) que toute autre erreur d'upstream. |

Le basculement entre les fournisseurs cloud ou vers l'API Anthropic directe change quel accord, géographie et autres conditions régissent la demande.

Le CLI applique le même contrôle de fonctionnalité aux passerelles indépendamment de quel upstream sert une demande donnée, donc le basculement n'envoie pas un champ de corps qu'un upstream rejetterait.

<h2 id="optional-sections">
  Sections optionnelles
</h2>

<h3 id="admin">
  `admin`
</h3>

Optionnel. Active `/v1/organizations/spend_limits`, qui reflète l'API Admin publique d'Anthropic, et l'application des limites de dépenses par développeur sur `/v1/messages`. Consultez [Limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits) pour savoir comment les plafonds sont définis et appliqués ; cette section couvre les clés `gateway.yaml` qui activent la fonctionnalité et l'ajustent.

```yaml theme={null}
admin:
  # Clés API statiques nommées pour les points de terminaison admin, envoyées en tant que x-api-key.
  # L'id apparaît dans le journal d'audit en tant que admin-key:<id> afin que chaque clé soit
  # attribuable. Tableau pour la rotation : ajoutez la nouvelle clé, mettez à jour les clients,
  # supprimez l'ancienne.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # Groupes IdP disposant d'un accès admin complet via le JWT de passerelle normal (pas de clé API).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Champ                     | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | Non    | Tableau de `{id, key}`. Une `x-api-key` correspondant à l'une de ces clés peut lister, définir et supprimer les limites de dépenses. Les valeurs de clé doivent comporter au moins 32 caractères ; les `id` doivent être uniques dans `read_keys` et `write_keys`.                                                                                                                                                                                                                                    |
| `read_keys`               | Non    | Tableau de `{id, key}`. Lecture seule : tous les points de terminaison `GET`, y compris la liste des plafonds, la récupération d'un par ID, et la lecture de [`/effective`](/docs/fr/claude-apps-gateway-spend-limits#%2Feffective) et [`/audit`](/docs/fr/claude-apps-gateway-spend-limits#%2Faudit).                                                                                                                                                                                                          |
| `admin_groups`            | Non    | Noms de groupes IdP. Un JWT de passerelle dont la revendication `groups` inclut l'un de ces groupes dispose d'un accès admin complet, lecture et écriture, et effectue un audit en tant que `oidc:<sub>`. Utilisez ceci pour les administrateurs humains ; utilisez les clés API pour les machines. Une entrée vide dans cette liste arrête la passerelle au démarrage. Consultez [Valeurs de correspondance qui arrêtent la passerelle au démarrage](#matcher-values-that-stop-the-gateway-at-boot). |
| `blocked_message`         | Non    | Ajouté textuellement à l'erreur `429 billing_error` qu'un développeur bloqué voit. Écrivez l'instruction complète, comme une URL ou un canal Slack. Lorsqu'il n'est pas défini, la passerelle envoie uniquement le message par défaut. Consultez [Comment l'application fonctionne](/docs/fr/claude-apps-gateway-spend-limits#how-enforcement-works).                                                                                                                                                      |
| `audit_retention_days`    | Non    | Par défaut `365`. Les lignes `admin_audit` plus anciennes sont supprimées.                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `spend_retention_months`  | Non    | Par défaut `13`. Les lignes du compteur `spend` plus anciennes que cela sont supprimées. La valeur par défaut conserve une année complète plus le mois partiel actuel pour les rapports d'une année sur l'autre.                                                                                                                                                                                                                                                                                      |
| `identity_retention_days` | Non    | Par défaut `90`. TTL de dernière consultation pour les lignes `principal_emails`, qui contiennent l'e-mail, le nom d'affichage et les groupes de chaque développeur (données personnelles). Délibérément plus court que la rétention des dépenses afin qu'une identité déprovisionée expire tandis que ses compteurs de dépenses anonymes restent.                                                                                                                                                    |
| `group_limit_mode`        | Non    | `min` (par défaut) ou `max`. Lorsqu'un développeur se trouve dans plusieurs groupes avec des plafonds, `min` applique le plus restrictif et `max` le moins restrictif. Utilisé à la fois par l'application et par `/effective`.                                                                                                                                                                                                                                                                       |

<h3 id="enforcement">
  `enforcement`
</h3>

Le bloc `enforcement` contrôle le comportement des vérifications de limite de dépenses lorsque le magasin est indisponible.

| Champ                  | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | Non    | Par défaut `false`. L'application des limites de dépenses échoue de manière permissive en cas de panne Postgres, afin que l'inférence reste opérationnelle. Définissez `true` pour échouer de manière restrictive : les développeurs au-delà du plafond sont bloqués, mais tout le monde l'est aussi si le magasin est inaccessible. Nécessite un bloc [`admin:`](#admin) : l'application des limites de dépenses ne s'exécute que lorsque `admin` est configuré, et la passerelle refuse de démarrer si vous définissez ceci à `true` sans bloc. |

<h3 id="pricing">
  `pricing`
</h3>

Le bloc `pricing` indique au compteur de dépenses quoi facturer au lieu du prix catalogue USD, afin que les plafonds et [`/effective`](/docs/fr/claude-apps-gateway-spend-limits#%2Feffective) reflètent vos tarifs contractuels. Les montants restent en USD et restent une estimation, pas une facture. Deux conditions préalables :

* Claude Code v2.1.227 ou ultérieur sur le serveur de passerelle. Les versions antérieures rejettent la clé inconnue au démarrage.
* Un bloc [`admin:`](#admin) ou, dans v2.1.268 ou ultérieur, un bloc [`managed:`](#managed) avec au moins une politique. La passerelle refuse de démarrer avec `pricing` défini et aucun bloc, car rien ne le lirait.

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| Champ        | Requis | Description                                                                                                                                                                                                                                                     |
| ------------ | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | Non    | Par défaut `1`. Le compteur multiplie chaque montant mesuré par ceci, qu'il soit au prix catalogue ou remplacé, donc `0.85` facture 85 % du prix. Doit être supérieur à 0 et au maximum 10, et une valeur supérieure à 1 est une [majoration](#mark-prices-up). |
| `overrides`  | Non    | Lignes de `{upstream, model, input, output, cache_read, cache_write}` en USD par million de jetons. Les quatre tarifs sont obligatoires. Chacun doit être supérieur à 0 et au maximum 10 000.                                                                   |

Comment le compteur correspond à une ligne de remplacement :

* Une ligne remplace le prix catalogue pour les demandes que `upstream`, un [`upstreams[].name`](#upstreams), traite pour `model`. Cela inclut le tarif [mode rapide](/docs/fr/fast-mode#understand-the-cost-tradeoff) plus élevé, afin que les demandes en mode rapide et standard soient mesurées aux mêmes quatre tarifs.
* Un ID intégré tel que `claude-sonnet-4-6`, correspondant comme [`models[].id`](#models), couvre chaque forme datée, forme régionale Amazon Bedrock, ou forme Google Cloud Agent Platform que le compteur évalue comme ce modèle. Toute autre chaîne, comme un alias ou un ARN de profil d'inférence, correspond à l'ID que le client a envoyé ou à la chaîne envoyée en amont, insensible à la casse.
* Lorsque les lignes se chevauchent, le compteur choisit la ligne la plus spécifique plutôt que la première ligne : une ligne dont `model` est la chaîne de modèle exacte envoyée en amont, puis une ligne correspondant à l'ID exact que le client a envoyé, puis une ligne nommant le modèle intégré.
* Un nom d'amont inconnu échoue au démarrage, tout comme deux lignes pour un amont qui nomment le même modèle, y compris deux orthographes d'un modèle intégré. La passerelle avertit au démarrage à propos d'une ligne qu'aucun modèle demandable ne peut utiliser.
* Les demandes de recherche Web restent au prix catalogue de \$0,01 ; le multiplicateur s'y applique toujours.

Pour les tarifs par région, donnez à chaque région son propre amont nommé et une ligne par amont.

<h4 id="mark-prices-up">
  Majorer les prix
</h4>

Avec v2.1.271 ou ultérieur sur le serveur de passerelle, vous pouvez définir `multiplier` au-dessus de 1, jusqu'à 10, pour mesurer plus que ce que le fournisseur facture, par exemple un tarif de rétrofacturation interne. Cet exemple mesure chaque demande à 120 % du prix :

```yaml theme={null}
pricing:
  multiplier: 1.2
```

Avec un bloc [`admin:`](#admin), la majoration s'applique également aux limites de dépenses. Le compteur compte 120 % du prix, afin que les développeurs atteignent leurs plafonds plus tôt. La passerelle enregistre un avertissement au démarrage qui le dit.

Le multiplicateur ne change pas ce que le fournisseur en amont facture pour les demandes.

Si la passerelle [envoie également les tarifs aux clients connectés](#send-the-rates-to-signed-in-clients), les développeurs ont besoin de Claude Code v2.1.271 ou ultérieur pour voir la majoration. Les clients antérieurs ignorent un `multiplier` supérieur à 1 et affichent les coûts sans lui.

Un serveur de passerelle antérieur à v2.1.271 refuse de démarrer si vous définissez un `multiplier` supérieur à 1.

<h4 id="send-the-rates-to-signed-in-clients">
  Envoyer les tarifs aux clients connectés
</h4>

Avec v2.1.268 ou ultérieur sur le serveur de passerelle, la passerelle place également les tarifs de `pricing` dans les politiques [`managed`](#managed) qu'elle sert, en tant que paramètre géré [`modelPricing`](/docs/fr/settings-reference#modelpricing). Les développeurs correspondant à une politique voient alors les tarifs `pricing` pour le premier amont qui sert chaque ID de modèle dans `/usage`, la ligne d'état et OpenTelemetry. Un développeur qui ne correspond à aucune politique ne reçoit aucun paramètre géré, afin que ses chiffres restent au prix catalogue. Les clients appliquent le paramètre dans Claude Code v2.1.242 ou ultérieur.

* Ce que la passerelle ajoute : à moins que le bloc `cli` d'une politique ne définisse déjà `modelPricing`, la passerelle ajoute le `multiplier` et, pour chaque ID de modèle qu'un client peut demander, la ligne de remplacement du premier amont qui sert cet ID. Un tarif que seul un amont de basculement facture reste sur la passerelle.
* Exclure une politique : définissez `modelPricing` à `{}` dans le bloc `cli` de cette politique, et ses développeurs restent au prix catalogue.
* Conserver les tarifs propres d'une politique : une politique dont le bloc `cli` définit `modelPricing` avec son propre `multiplier` ou `overrides` conserve ce `modelPricing` entièrement, et la passerelle n'ajoute aucun de ses propres tarifs à celui-ci.

<h3 id="models">
  `models`
</h3>

Le bloc `models` est une liste de modèles optionnelle organisée par l'administrateur, servie à `/v1/models` et utilisée pour traduire les ID de modèles par amont. Elle est obligatoire pour les régions Amazon Bedrock non-US, les ARN de débit provisionné Amazon Bedrock et les noms de déploiement Microsoft Foundry.

```yaml theme={null}
auto_include_builtin_models: true   # false: expose only the list below
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optional text shown in clients that surface it
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # or an inference-profile ARN
      foundry: your-opus-deployment-name
```

Chaque clé sous `upstream_model` doit correspondre au `name` d'un amont configuré, qui par défaut est le nom du fournisseur. Une clé qui ne correspond à aucun amont échoue au démarrage, donc omettez les lignes pour les fournisseurs que vous n'utilisez pas.

<h3 id="managed">
  `managed`
</h3>

Le bloc `managed` définit les politiques d'accès basées sur les rôles basées sur les groupes IdP ou le domaine de messagerie. Les politiques sont évaluées dans l'ordre ; la première correspondance est sélectionnée, puis fusionnée sur la base de capture-tout `match: {}`. Elles sont servies par utilisateur à `GET /managed/settings` avec mise en cache ETag/304.

```yaml theme={null}
managed:
  policies:
    # Specific groups first.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Default catch-all last: matches everyone who authenticated.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Une capture-tout `match: {}`, conventionnellement listée en dernier, est traitée comme une couche de base. Chaque autre politique hérite de toute clé qu'elle ne définit pas de la capture-tout, afin que les entrées par rôle n'aient besoin de lister que ce qui diffère de la valeur par défaut de l'organisation. Les règles de fusion dépendent du type de clé :

* **Listes d'autorisation** : `availableModels` et `permissions.allow`. La liste d'une politique spécifique remplace entièrement celle de la base.
* **Listes de refus et tableaux de hooks** : `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` et chaque tableau d'événement de type `hooks`. Ceux-ci prennent l'union de la base et de la politique, afin qu'un refus à l'échelle de l'organisation ou un hook d'audit ne soit pas accidentellement supprimé par un remplacement par rôle.
* **Clés de type enregistrement** : `env`, `modelOverrides` et `skillOverrides`. Ceux-ci fusionnent superficiellement, afin qu'un bloc `env` par rôle remplace les clés qu'il définit et hérite du reste de la base.

`availableModels` est également appliqué côté serveur à `/v1/messages`, afin qu'un modèle refusé retourne `400` indépendamment de ce que le client envoie.

La passerelle valide la valeur `model` elle-même avant de relayer une demande, afin qu'une valeur mal formée n'atteigne jamais un amont. Elle rejette la demande avec un `400` dans deux cas :

* Lorsque la valeur est manquante ou vide, la passerelle rejette la demande avec le message `model is required`. Cette vérification nécessite une passerelle exécutant Claude Code v2.1.228 ou ultérieur.
* Lorsque la valeur est présente mais n'est pas une chaîne, la passerelle rejette la demande avec le message `model must be a string`. Nécessite une passerelle exécutant Claude Code v2.1.221 ou ultérieur.

| Correspondance                                      | Comportement                                                                                                                                                        |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Correspond à chaque utilisateur authentifié. Commencez par l'un de ceux-ci et ajoutez des politiques limitées aux groupes au-dessus plus tard.                      |
| `match: { groups: [a, b] }`                         | Correspond si la revendication `groups` du JWT contient l'un des groupes listés. Sensible à la casse : les groupes doivent correspondre à la casse exacte de l'IdP. |
| `match: { email_domain: example.com }`              | Correspond à la partie après le dernier `@` dans la revendication `email` du JWT, insensible à la casse. Accepte un domaine par politique.                          |
| `match: { groups: [a], email_domain: example.com }` | Les deux conditions doivent correspondre                                                                                                                            |

Un utilisateur authentifié qui ne correspond à aucune politique obtient les valeurs par défaut de la passerelle, ce qui signifie chaque modèle du catalogue et aucun paramètre géré. Ajoutez une capture-tout `match: {}` en dernier si vous voulez une politique par défaut garantie.

<Note>
  La passerelle ne conserve aucun répertoire d'utilisateurs propre. Elle autorise chaque demande à partir du jeton IdP de l'utilisateur, en lisant l'appartenance au groupe à partir de la revendication `groups` du jeton et en évaluant les politiques par rapport à celui-ci. Il n'y a pas de liste à énumérer et aucun compte à pré-créer, et donc aucun point de terminaison SCIM, car il n'y a rien pour que SCIM se synchronise.

  Exécutez la gestion du cycle de vie des utilisateurs et des groupes à la source de vérité, qui est le provisionnement SCIM natif de votre IdP ou une plateforme de gouvernance des identités dédiée. L'appartenance et la déprovision gouvernées là-bas s'écoulent dans la passerelle automatiquement via le jeton. Si vous voulez le provisionnement SCIM des comptes Claude eux-mêmes, c'est une capacité de [Claude for Enterprise](/docs/fr/admin-setup).

  Deux horloges de propagation s'appliquent :

  * **Contenu de la politique** : modifier une politique et redéployer atteint les clients connectés lors de leur prochain sondage de paramètres gérés, dans l'heure, à l'exception des [modifications qui s'appliquent uniquement au prochain lancement](/docs/fr/server-managed-settings#fetch-and-caching-behavior)
  * **Appartenance au groupe** : modifier l'appartenance au groupe d'un utilisateur change la politique qui le correspond. Cela prend effet lors de la prochaine réémission de session, ce qui signifie le prochain renouvellement silencieux, limité par `session.ttl_hours`.
</Note>

<h4 id="matcher-values-that-stop-the-gateway-at-boot">
  Valeurs de correspondance qui arrêtent la passerelle au démarrage
</h4>

Au démarrage, la passerelle vérifie le bloc `match` de chaque politique et la liste [`admin_groups`](#admin). L'une de ces valeurs arrête la passerelle avec une erreur qui nomme le champ :

* Une liste `groups` vide
* Une entrée vide dans `groups` ou dans `admin_groups`
* Un `email_domain` vide
* Un `email_domain` qui contient `@`, un espace ou une virgule. La passerelle supprime la valeur et enlève un `@` initial avant cette vérification. Écrivez un domaine nu, comme `example.com`.

Avant v2.1.232, la passerelle démarrait avec ces valeurs. Chaque valeur avait cet effet :

* Un `email_domain` vide : la passerelle a ignoré la vérification du domaine, afin qu'une politique avec un `email_domain` vide et aucune liste `groups` corresponde à chaque utilisateur authentifié
* Une liste `groups` vide : la politique ne correspondait à personne
* Un `email_domain` contenant `@`, un espace ou une virgule : la politique ne correspondait à personne
* Une entrée vide dans `groups` ou dans `admin_groups` : l'entrée correspondait à un utilisateur uniquement lorsque la revendication `groups` IdP de cet utilisateur contenait également une entrée vide. Dans `admin_groups`, cette correspondance accordait l'accès admin. Si votre liste `admin_groups` ne contenait jamais une entrée vide, personne n'a obtenu l'accès admin de cette façon.

<h4 id="what-goes-in-cli">
  Ce qui va dans `cli`
</h4>

Chaque valeur `cli` est un document complet Claude Code `managed-settings.json`, le même schéma que vous déploieriez via MDM ou `/etc/claude-code/managed-settings.json`, exprimé ici en YAML. Le CLI applique le document livré au niveau géré, au-dessus des paramètres utilisateur et projet, à la place des paramètres gérés par le serveur. Il ignore donc les paramètres [restreints aux sources de politique au niveau du système d'exploitation](/docs/fr/server-managed-settings#current-limitations), comme `policyHelper` et `wslInheritsWindowsSettings`.

La passerelle valide chaque document par rapport au schéma de paramètres du CLI au démarrage, afin qu'une clé de niveau supérieur non reconnue échoue au démarrage avec une erreur nommant chaque clé contrevenante. Les parties délibérément ouvertes du schéma acceptent toujours des valeurs arbitraires, car les clients plus récents peuvent reconnaître des entrées que le schéma de la passerelle ne reconnaît pas. Ces clés ouvertes incluent `env`, `pluginConfigs` et les clés imbriquées sous `permissions`.

Parce que la validation utilise le schéma fourni avec la version installée de la passerelle, placer une clé de paramètres de niveau supérieur introduite par une version plus récente de Claude Code dans la configuration gérée nécessite de mettre à niveau la passerelle en premier. Testez une nouvelle politique sur un client avant de la déployer.

La référence complète des clés se trouve dans [Paramètres Claude Code](/docs/fr/settings-reference#all-settings). Les clés que les opérateurs atteignent en premier :

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Model access (also enforced server-side at /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Permission policy
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blocks --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignore user/project permission rules

        # Environment pushed into the CLI process. DISABLE_UPDATES blocks
        # background and manual updates; DISABLE_AUTOUPDATER stops only
        # background updates.
        env:
          DISABLE_UPDATES: "1"                    # pin versions via your own distribution

        # Org-wide hooks. Hook commands run on developer machines, not the
        # gateway, so the path must exist on every client OS in the policy.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Clé                                        | Appliquée par    | Effet                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------------------------------ | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Passerelle + CLI | Liste d'autorisation des modèles. Également vérifiée à `/v1/messages`, afin qu'un client corrigé ne puisse pas la contourner.                                                                                                                                                                                                                                                                                          |
| `permissions.allow` / `.deny`              | CLI              | Règles d'outils et de commandes. Consultez [Permissions](/docs/fr/permissions).                                                                                                                                                                                                                                                                                                                                             |
| `permissions.disableBypassPermissionsMode` | CLI              | Définissez à `disable` pour bloquer [`bypassPermissions`](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode), le mode qui ignore les invites de permission, et l'indicateur `--dangerously-skip-permissions`                                                                                                                                                                                            |
| `allowManagedPermissionRulesOnly`          | CLI              | Lorsque `true`, les paramètres gérés deviennent la seule source de paramètres des règles de permission. L'entrée [`allowManagedPermissionRulesOnly`](/docs/fr/settings-reference#allowmanagedpermissionrulesonly) liste chaque source que Claude Code ignore ensuite.                                                                                                                                                       |
| `env`                                      | CLI              | Variables d'environnement fusionnées dans le processus CLI. Utilisez pour la télémétrie, la mise à jour automatique et les remplacements de noms de modèles.                                                                                                                                                                                                                                                           |
| `hooks`                                    | CLI              | [Hooks](/docs/fr/hooks) à l'échelle de l'organisation                                                                                                                                                                                                                                                                                                                                                                       |
| `managedMcpServers`                        | CLI              | Serveurs MCP distants [fournis à chaque développeur correspondant](/docs/fr/managed-mcp#provide-servers-through-managed-settings) aux côtés des serveurs qu'ils ajoutent eux-mêmes, `http` et `sse` uniquement. Consultez [Serveurs MCP dans une politique](#mcp-servers-in-a-policy). Nécessite Claude Code v2.1.259 ou ultérieur sur le serveur de passerelle et sur les clients. Les clients antérieurs ignorent la clé. |

Parce que ces paramètres arrivent sur le réseau, le CLI affiche à chaque développeur une boîte de dialogue d'approbation de sécurité avant d'appliquer les paramètres listés ci-dessous :

* `hooks`
* Variables `env` qui nécessitent l'approbation du développeur, comme les variables de proxy et d'URL de base
* Paramètres d'exécution de shell comme `apiKeyHelper` et `statusLine`
* Les paramètres binaires du bac à sable `sandbox.bwrapPath`, `sandbox.socatPath` et `sandbox.ripgrep`
* Les paramètres du bac à sable qui interceptent le trafic, injectent des identifiants ou affaiblissent l'isolation, comme `sandbox.network.tlsTerminate` et les paramètres du port proxy. [Boîtes de dialogue d'approbation de sécurité](/docs/fr/server-managed-settings#security-approval-dialogs) les liste tous.

[Mémoire d'approbation](/docs/fr/server-managed-settings#approval-memory) couvre la durée d'une approbation et quand la boîte de dialogue apparaît à nouveau.

Claude Code applique certaines variables `env` livrées sans afficher la boîte de dialogue d'approbation au développeur, comme les paramètres de sélection de modèle et les limites numériques. D'autres variables livrées peuvent nécessiter l'approbation du développeur avant de prendre effet ; une valeur de proxy, d'URL de base ou `OTEL_EXPORTER_OTLP_ENDPOINT` non vide le fait toujours. Lorsqu'une variable livrée a besoin d'approbation, la boîte de dialogue la nomme.

[Variables d'environnement et boîte de dialogue d'approbation](/docs/fr/server-managed-settings#environment-variables-and-the-approval-dialog) a les détails, y compris quatre bascules de confidentialité dont la valeur livrée décide si elles ont besoin d'approbation. Avant v2.1.218, Claude Code appliquait moins de variables sans demander au développeur, afin que plus de variables livrées déclenchent la boîte de dialogue.

La configuration de [télémétrie](#telemetry) de la passerelle pousse `OTEL_EXPORTER_OTLP_ENDPOINT`, afin que la définition de `telemetry.forward_to` déclenche la boîte de dialogue sur chaque client interactif. La boîte de dialogue protège la machine du développeur d'une passerelle compromise ou hostile, pas l'organisation du développeur.

Une exécution non interactive avec l'indicateur `-p` ne peut pas afficher la boîte de dialogue. Elle applique les paramètres poussés pour cette exécution uniquement et ne les enregistre pas comme approuvés, afin que la prochaine session interactive du développeur affiche toujours la boîte de dialogue pour eux. Avant v2.1.207, une exécution non interactive enregistrait les paramètres comme approuvés et aucune session interactive ultérieure n'affichait la boîte de dialogue pour eux.

Si un développeur refuse, Claude Code quitte cette session plutôt que d'appliquer la politique. Lorsque vous poussez un nouveau hook, ou toute variable env qui déclenche la boîte de dialogue, à une politique large, Claude Code affiche donc la boîte de dialogue à chaque développeur correspondant. Il affiche la boîte de dialogue dans une session en cours lors du prochain sondage horaire, et sinon au prochain démarrage du développeur.

La clé `cli` s'appelait `settings` dans les versions antérieures. Cette orthographe est toujours acceptée comme alias, mais les nouveaux déploiements doivent utiliser `cli`.

<h4 id="mcp-servers-in-a-policy">
  Serveurs MCP dans une politique
</h4>

Pour fournir des serveurs MCP aux clients Claude Code qu'une politique correspond, définissez [`managedMcpServers`](/docs/fr/managed-mcp#provide-servers-through-managed-settings) dans le bloc `cli` de cette politique. Vous avez besoin de Claude Code v2.1.259 ou ultérieur sur le serveur de passerelle et sur les clients.

La passerelle vérifie chaque entrée au démarrage avec [les mêmes règles que Claude Code applique sur le client](/docs/fr/managed-mcp#what-an-entry-can-contain), et si une entrée échoue une vérification, la passerelle refuse de démarrer et nomme l'entrée.

Si vous écrivez une référence `${VAR}` dans `gateway.yaml`, la passerelle la résout à partir de son environnement au démarrage via [expansion de secret](#secret-expansion) avant d'exécuter les vérifications d'entrée, afin que chaque client correspondant reçoive la valeur littérale et puisse la lire. L'[orientation d'en-tête pour les serveurs fournis](/docs/fr/managed-mcp#provide-servers-through-managed-settings) s'applique à la valeur étendue.

La passerelle rejette l'orthographe `.mcp.json` `mcpServers` dans un bloc `cli`, et son erreur de démarrage nomme `managedMcpServers` comme clé à utiliser. Avant v2.1.259, la passerelle rejetait toute définition de serveur MCP dans un bloc `cli`.

<h4 id="claude-desktop-overlay">
  Superposition Claude Desktop
</h4>

Si votre organisation déploie également [Claude Desktop](/docs/fr/desktop), la même passerelle sert les deux clients. Pointez `bootstrapUrl`, dans la [configuration gérée](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop, sur `<listen.public_url>/user/bootstrap`. Claude Desktop dérive l'émetteur OAuth de cette URL, exécute la même connexion par code d'appareil contre cette passerelle et récupère sa configuration à partir de la réponse.

<Note>
  Nécessite Claude Code v2.1.203 ou ultérieur sur le serveur de passerelle, et un opt-in explicite : `/user/bootstrap` retourne 404 à moins que la politique correspondant à l'utilisateur ne porte une clé `desktop`. Un `desktop: {}` vide opte une politique, et une clé `desktop` sur la couche de base `match: {}` opte chaque politique qui l'hérite. Le journal d'audit enregistre chaque demande en tant que `desktop_bootstrap.serve` ou `desktop_bootstrap.denied`.
</Note>

La passerelle dérive une grande partie de la réponse du bloc `cli` de la politique correspondante et de la configuration de passerelle de niveau supérieur :

* La liste des modèles, de `availableModels`
* Outils désactivés, à partir des entrées `permissions.deny` de nom d'outil nu. Si vous définissez `disabledBuiltinTools` dans le bloc `desktop` de la politique, la passerelle sert l'union de votre valeur et de la liste dérivée, afin que vous puissiez désactiver plus d'outils de cette façon mais ne puissiez pas réactiver un que vous avez désactivé via `permissions.deny`
* La liste d'autorisation de sortie, de `sandbox.network.allowedDomains`. Si vous définissez `coworkEgressAllowedHosts` dans le bloc `desktop` de la politique, la passerelle utilise cette valeur à la place de la liste dérivée
* Un point de terminaison OTLP qui pointe vers la passerelle elle-même, et les attributs d'identité de l'utilisateur connecté. La passerelle relaie les exportations qu'elle reçoit à ce point de terminaison vers vos destinations `forward_to`. Elle inclut le point de terminaison et les attributs lorsque vous définissez à la fois [`telemetry.forward_to`](#telemetry) et `listen.public_url`.

  Claude Desktop exporte chaque signal avec un encodage : `http/protobuf`, ou `http/json` lorsque vous définissez `OTEL_EXPORTER_OTLP_PROTOCOL` ou l'un de ses variantes par signal à `http/json` dans l'`env` de la politique. Avant Claude Code v2.1.261 sur le serveur de passerelle, la réponse définissait `http/json` indépendamment, afin qu'un collecteur qui accepte uniquement protobuf rejette les exportations de Claude Desktop

Pour définir `disabledBuiltinTools`, `coworkEgressAllowedHosts` ou le paramètre `managedMcpServers` propre de Claude Desktop dans le bloc `desktop` d'une politique, vous avez besoin de Claude Code v2.1.232 ou ultérieur sur le serveur de passerelle. Le `managedMcpServers` de Claude Desktop prend une valeur de tableau plutôt qu'un objet.

La passerelle omet les clés sans équivalent Claude Desktop, comme `hooks` et les règles de permission limitées comme `Bash(npm *)`, de la réponse d'amorçage.

Ajoutez le bloc `desktop` optionnel aux côtés de `cli` pour définir les paramètres Claude Desktop directement. Écrivez les paramètres de la [référence de configuration gérée](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop en tant que noms de clés plats. Laissez de côté les clés que Claude Desktop lit uniquement à partir de MDM ou de fichiers locaux, comme `bootstrapUrl` ; la passerelle les rejette au démarrage. Avant v2.1.232, la passerelle acceptait une liste fixe de 11 clés de porte de fonctionnalité, comme `chatTabEnabled` et `disableAutoUpdates`, et rejetait chaque autre clé au démarrage. Avant v2.1.227, la passerelle rejetait également `chatTabEnabled` et `chatAdvancedFileAnalysisEnabled` au démarrage.

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

Chaque clé est optionnelle ; Claude Desktop applique sa propre valeur par défaut pour toute clé que vous omettez. La passerelle valide chaque bloc `desktop` au démarrage par rapport au schéma de configuration que Claude Desktop lui-même utilise, afin qu'une erreur apparaisse au démarrage de la passerelle en tant qu'erreur nommant la clé plutôt que d'atteindre chaque bureau connecté. La passerelle échoue au démarrage lorsqu'un bloc contient :

* Une clé inconnue
* Une clé reconnue dont la valeur Claude Desktop rejetterait ou supprimerait silencieusement, comme une valeur vide ou une sous-clé mal orthographiée à l'intérieur d'une entrée imbriquée. Avant v2.1.260, la passerelle supprimait silencieusement un champ mal orthographié à l'intérieur d'un objet imbriqué d'une entrée `managedMcpServers` ou `orgPluginSettings` au lieu d'échouer au démarrage.
* Une clé que la passerelle calcule elle-même : la connexion d'inférence, la liste des modèles et le relais OTLP. Configurez ceux-ci via [`upstreams`](#upstreams), [`models`](#models) et la section [`telemetry`](#telemetry) `forward_to`.
* Un alias hérité d'une clé actuelle. Dans l'erreur de démarrage, la passerelle nomme la clé canonique à écrire.

Si vous utilisez une valeur ou une forme d'entrée dépréciée, comme une entrée `managedMcpServers` sans `transport`, la passerelle démarre et enregistre un avertissement qui nomme le remplacement.

La passerelle valide un bloc `desktop` par rapport au schéma fourni avec sa version installée, comme elle le fait pour le bloc `cli`. Pour livrer un paramètre introduit par une version plus récente de Claude Desktop, mettez à niveau la passerelle en premier. Par exemple, `userPluginMarketplacesEnabled` et `userPluginUploadsEnabled` nécessitent Claude Code v2.1.260 ou ultérieur sur le serveur de passerelle et Claude Desktop 1.37937.0 ou ultérieur sur les machines des membres.

Si vous définissez `orgPluginSettings` dans le bloc `desktop` d'une politique, la passerelle le sert sous la forme de tableau que Claude Desktop 1.15200.0 et ultérieur lit. Les anciens bureaux ignorent le tableau et n'appliquent aucune politique d'outil de plugin, afin de mettre à jour les membres vers 1.15200.0 ou ultérieur avant de vous y fier.

La passerelle remplit les clés qu'un bloc `desktop` d'une politique ne définit pas à partir du bloc `desktop` de la capture-tout `match: {}`, de la même manière qu'elle remplit le bloc `cli` d'une politique à partir de la base. Si vous définissez `disabledBuiltinTools` ou `builtinToolPolicy` à la fois dans la base et dans une politique par rôle, la passerelle conserve la restriction de la base :

* `disabledBuiltinTools` : la passerelle utilise l'union de la liste de la base et de la liste de la politique
* `builtinToolPolicy` : si vous définissez un outil à une valeur autre que `allow` dans la base, la passerelle conserve cette valeur même si vous définissez `allow` pour le même outil dans une politique par rôle

Pour chaque autre clé, si vous la définissez dans la politique par rôle, la passerelle utilise la valeur de la politique par rôle. La passerelle remplace un tableau ou un objet imbriqué comme `banner` entièrement, afin que si vous définissez `banner.text` dans une politique par rôle, la passerelle supprime le `banner.backgroundColor` de la base.

Si vous ne déployez pas Claude Desktop, laissez `desktop` complètement hors de vos politiques ; la passerelle retourne alors 404 de `/user/bootstrap` pour chaque utilisateur.

<h4 id="precedence-with-other-managed-sources">
  Précédence avec d'autres sources gérées
</h4>

Si un appareil a également une politique livrée par MDM ou un `managed-settings.json` local, les paramètres livrés par la passerelle sont prioritaires. [Précédence au sein du niveau géré](/docs/fr/managed-settings#precedence-within-the-managed-tier) sur la page des paramètres gérés dit quand les sources locales s'appliquent, et a les [clés que Claude Code lit à partir de chaque source admin](/docs/fr/managed-settings#keys-read-from-every-admin-source) indépendamment de la source qu'il a sélectionnée, comme les clés de verrouillage du bac à sable, `forceRemoteSettingsRefresh` et la fusion `env` par variable. Un [`policyHelper`](/docs/fr/settings-reference#policyhelper) configuré dans un profil MDM ou le fichier de paramètres gérés s'exécute uniquement lorsque la passerelle ne livre aucun paramètre ; l'entrée dit ce que sa sortie remplace.

Les hôtes d'intégration comme [Claude Desktop](/docs/fr/desktop) peuvent fournir une politique via l'option SDK `managedSettings`. [Paramètres parents à partir d'hôtes d'intégration](/docs/fr/managed-settings#parent-settings-from-embedding-hosts) dit quand Claude Code l'applique, et [Restreindre les paramètres parents](/docs/fr/claude-apps-gateway#restrict-parent-settings) liste les paramètres de direction d'autorisation qui s'appliquent toujours sans les verrous `allowManaged*Only`.

Les politiques de passerelle s'appliquent à chaque invocation de Claude Code sur la machine, y compris les exécutions non interactives `claude -p` et les sessions générées par le SDK Agent. Si la passerelle est inaccessible au démarrage, les sessions connectées quittent avec une erreur plutôt que de s'exécuter sans leur politique.

<h3 id="telemetry">
  `telemetry`
</h3>

Le CLI envoie des métriques, des journaux et, lorsqu'ils sont activés, des traces à la passerelle, qui les relaie textuellement à chaque destination configurée. Les exportations utilisent OpenTelemetry Protocol (OTLP) sur HTTP. Pour ignorer le relais et faire exporter les sessions directement à votre collecteur, [nommez le collecteur dans une politique](#export-directly-to-your-collector). Consultez [Surveillance de l'utilisation](/docs/fr/monitoring-usage) pour les métriques et événements que le CLI émet.

Le CLI horodate chaque exportation avec l'identité de l'utilisateur authentifié, lue à partir du JWT émis par la passerelle : les attributs `user.id`, `user.email` et `user.groups`. L'attribution du coût et de l'utilisation par développeur fonctionne donc sans configuration côté développeur.

[Claude Desktop](#claude-desktop-overlay) et les sessions Cowork connectées via la passerelle horodatent leur télémétrie avec `user.email` et `user.groups` aux côtés de `enduser.id`, afin que vous puissiez couvrir l'utilisation du terminal, du bureau et de Cowork avec une requête sur `user.email` ou `user.groups`. `user.groups` est la liste des groupes IdP séparée par des virgules.

La télémétrie du bureau et de Cowork porte également `enduser.sub`, la revendication `sub` que votre fournisseur d'identité émet pour l'utilisateur, qui reste la même lorsque l'e-mail d'un utilisateur change. Les sessions de terminal horodatent la même valeur sous `user.id`, afin qu'une requête qui correspond à `enduser.sub` par rapport à `user.id` du terminal couvre l'utilisation du terminal, du bureau et de Cowork d'un utilisateur ensemble. Sur les exportations du bureau et de Cowork, `user.id` est un identifiant anonyme, pas le sujet.

Comme toutes les données OpenTelemetry de Claude Code, ces attributs vont uniquement aux destinations que votre organisation configure, jamais à Anthropic.

Si la liste des groupes d'un utilisateur est plus longue que 255 caractères une fois codée en pourcentage, ou si un nom de groupe contient une virgule ou un signe égal, la passerelle laisse `user.groups` hors de la télémétrie du bureau et de Cowork de cet utilisateur plutôt que de la tronquer. Les sessions de terminal de cet utilisateur portent toujours la liste complète.

La passerelle laisse `enduser.sub` hors quand le sujet est plus long que 255 caractères une fois codé en pourcentage, ou contient un espace, un caractère en dehors de l'ASCII imprimable, ou l'un de `,` `;` `=` `\` `"` `%`. La télémétrie du bureau et de Cowork de cet utilisateur conserve ses autres attributs.

Vous avez besoin de Claude Code v2.1.265 ou ultérieur sur le serveur de passerelle pour `user.email` et `user.groups` sur la télémétrie du bureau et de Cowork, et Claude Desktop 1.24012 ou ultérieur sur la machine de chaque développeur pour `user.groups`.

Vous avez besoin de Claude Code v2.1.274 ou ultérieur sur le serveur de passerelle pour `enduser.sub`.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Per-signal opt-in. Default: metrics only.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Chaque destination opte pour `metrics`, `logs` et `traces` indépendamment, et la valeur par défaut est les métriques uniquement. Les signaux diffèrent en sensibilité :

  * **Métriques** : compteurs agrégés comme les comptages de jetons, les comptages de demandes et la latence
  * **Journaux et traces** : peuvent porter des commandes Bash complètes, des entrées d'outils et des chemins de fichiers, couvrant tout ce que Claude Code fait sur la machine d'un développeur

  Activez les journaux et les traces uniquement sur les destinations avec les contrôles d'accès et la politique de rétention que les données justifient.
</Warning>

Chaque URL `forward_to` doit utiliser `https://`, avec une exception pour un collecteur sur l'interface de bouclage propre de la passerelle :

* `http://localhost:<port>` passe la validation de configuration, mais la [garde SSRF](/docs/fr/claude-apps-gateway-deploy#threat-model-summary) bloque chaque exportation avec `ECONNREFUSED_SSRF` à moins que vous ne définissiez `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` dans l'environnement de la passerelle
* `http://127.0.0.1:<port>` ou `http://[::1]:<port>` échoue au démarrage à moins que cette variable ne soit définie

Pour un collecteur en cluster, exposez-le sur HTTPS à sa propre adresse interne, ou exécutez-le en tant que sidecar avec la variable définie.

Lorsque `HTTPS_PROXY` est défini, la passerelle envoie les exportations via ce proxy.

Pour atteindre un collecteur interne directement, ajoutez-le à `NO_PROXY` par nom d'hôte ou par un domaine avec un point initial comme `.internal.example.com`, ce qui nécessite Claude Code v2.1.277 ou ultérieur sur le serveur de passerelle. Assurez-vous que la passerelle peut atteindre le collecteur sans le proxy. Une entrée sans point initial correspond uniquement à ce nom exact, pas aux noms en dessous. Les plages CIDR ne correspondent pas.

Avec [sortie proxy uniquement](#proxy-only-egress) activée, autorisez le collecteur dans le proxy à la place, car toute entrée `NO_PROXY` désactive la sortie proxy uniquement.

La télémétrie est désactivée dans le CLI par défaut. Lorsque vous définissez à la fois `telemetry.forward_to` et `listen.public_url`, la passerelle l'active pour les clients connectés en poussant six variables d'environnement via `/managed/settings` :

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` et `OTEL_TRACES_EXPORTER`, chacun défini à `otlp` si au moins une destination `forward_to` active ce signal et à `none` sinon
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

Avant Claude Code v2.1.265 sur le serveur de passerelle, la passerelle poussait les trois sélecteurs d'exportateur en tant que `otlp`, y compris pour les signaux qu'aucune destination n'a activés.

Le point de terminaison poussé est construit à partir de l'URL publique, afin que les métriques et les journaux n'aient besoin d'aucune configuration OTEL de la part des développeurs ou des politiques.

Les développeurs connectés via `/login` ne peuvent pas rediriger les exportations avec leur propre configuration OTEL :

* **Variables définies localement** : Claude Code applique les variables poussées au niveau géré, afin que chacune remplace la valeur qu'un développeur définit pour elle localement.
* **Points de terminaison configurés localement** : avec l'exportation OTLP/HTTP activée, le CLI ignore tout point de terminaison configuré localement, que la passerelle ait poussé les variables de télémétrie ou non. Ses exportations vont à la passerelle à moins qu'une politique [nomme votre collecteur comme point de terminaison](#export-directly-to-your-collector).

Sans destination `forward_to` pour un signal, la passerelle l'accepte et le rejette. Si les développeurs exportent déjà la télémétrie Claude Code vers l'un de vos collecteurs, ajoutez-le en tant que destination `forward_to`, avec les journaux ou les traces activés s'ils les exportent, afin qu'il continue à recevoir leurs données après qu'ils se connectent. Pour ignorer le relais à la place, [nommez le collecteur dans une politique](#export-directly-to-your-collector).

[Les traces](/docs/fr/monitoring-usage#traces-beta) nécessitent également `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` sur chaque client. Définissez-le dans le bloc `env` d'une politique gérée, car la passerelle ne le pousse pas. Les développeurs l'approuvent dans la même [boîte de dialogue d'approbation de sécurité](#managed) que le point de terminaison poussé déclenche déjà.

Définissez-le à `1` uniquement dans les politiques dont vous voulez que les groupes soient tracés. Une politique qui ne le définit pas hérite de la valeur de votre politique de capture-tout `match: {}` si cette politique en définit une, selon les [règles de fusion](#managed). Pour empêcher les clients d'un groupe d'envoyer des traces même lorsqu'un développeur définit la variable localement, définissez-la à `0` dans la politique de ce groupe.

Les encodages OTLP protobuf et JSON sont tous deux relayés, et tout backend compatible OpenTelemetry fonctionne comme destination.

<h4 id="export-directly-to-your-collector">
  Exporter directement vers votre collecteur
</h4>

Pour que les sessions connectées via `/login` envoient la télémétrie directement à votre collecteur au lieu de passer par le relais, définissez `OTEL_EXPORTER_OTLP_ENDPOINT` à l'URL de base `https://` du collecteur dans le bloc `env` d'une [politique gérée](#managed). Claude Code ajoute `/v1/metrics`, `/v1/logs` ou `/v1/traces` à l'URL que vous définissez, comme `https://otel-collector.example.com:4318`, et exporte chaque signal là-bas sur OTLP/HTTP. Nécessite Claude Code v2.1.265 ou ultérieur sur la machine de chaque développeur. Les clients antérieurs exportent via le relais.

Pour vous authentifier auprès du collecteur, définissez `OTEL_EXPORTER_OTLP_HEADERS` dans le même bloc `env`. Les sessions n'envoient jamais le jeton de session de passerelle du développeur à un collecteur nommé de cette façon.

Lorsque vous ajoutez ou modifiez ce point de terminaison dans une politique, Claude Code demande à chaque développeur de l'approuver dans la [boîte de dialogue d'approbation de sécurité](#managed) avant de l'appliquer dans une session interactive.

Claude Code vérifie le point de terminaison avant d'exporter un signal directement, et garde ce signal sur le relais lorsqu'une vérification échoue. Les vérifications incluent :

* Le point de terminaison provient de la passerelle elle-même. Si vous définissez la même variable dans un profil MDM ou un `managed-settings.json` local, les exportations restent sur le relais.
* L'URL utilise `https://`, ou `http://` à une adresse de bouclage
* L'URL se résout en un chemin se terminant par `/v1/<signal>`, sans requête ni fragment. Claude Code construit ce chemin lui-même à partir de la variable générique. Il utilise une variable par signal comme `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` telle qu'écrite, afin d'inclure le chemin complet là-bas.
* L'URL n'est pas l'hôte propre de la passerelle. Un point de terminaison adressé à la passerelle conserve le chemin du relais et son jeton de session.
* Ni vous ni le développeur n'avez configuré [`otelHeadersHelper`](/docs/fr/settings-reference#otelheadershelper) dans aucune source de paramètres. Avec un helper configuré, chaque signal reste sur le relais.

Le point de terminaison que vous nommez change uniquement où les exportations vont. Vous choisissez toujours quels signaux exportent du tout avec les sélecteurs `OTEL_*_EXPORTER`.

Le point de terminaison seul n'active pas l'exportation, afin de définir également les variables qui le font, à moins que la passerelle ne les pousse déjà :

* Si la passerelle [pousse déjà les variables de télémétrie](#telemetry), elles couvrent l'activation, les sélecteurs et le protocole, et votre point de terminaison explicite remplace la valeur `<public_url>` poussée. Définissez un sélecteur `OTEL_*_EXPORTER` à `otlp` vous-même uniquement pour un signal qu'aucune destination `forward_to` n'active.
* Si ce n'est pas le cas, définissez également `CLAUDE_CODE_ENABLE_TELEMETRY=1`, les sélecteurs `OTEL_*_EXPORTER` et `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`.

Lorsque le développeur se déconnecte, ou se connecte à une passerelle différente, les exportations vers le collecteur s'arrêtent et Claude Code supprime chaque lot restant plutôt que de l'envoyer.

<h4 id="when-a-destination-fails">
  Lorsqu'une destination échoue
</h4>

La passerelle ne met pas en mémoire tampon, ne réessaie pas ou ne stocke pas la télémétrie, afin qu'elle rejette une exportation qui n'atteint pas une destination plutôt que de la livrer tard. Chaque destination réussit ou échoue par elle-même, et le client exportateur reçoit une réponse de succès de toute façon, afin qu'une livraison échouée n'apparaisse que dans le journal de la passerelle.

Après cinq livraisons consécutives échouées à une destination, la passerelle met en pause le transfert vers elle en étirements de 30 secondes, enregistrant chaque pause, jusqu'à ce qu'une livraison réussisse. Toute réponse d'erreur, délai d'attente ou erreur de connexion compte comme une livraison échouée, sauf `400`, `413`, `415`, `422` et `431`, qui signifient que le collecteur a refusé la charge utile de cette exportation comme mal formée ou trop grande.

Une charge utile refusée n'avance ni ne réinitialise le compteur d'échecs : la passerelle continue de transférer vers la destination et enregistre un avertissement la nommant et le statut, au premier refus de la destination et tous les centièmes après.

<h3 id="http-tuning">
  Réglage HTTP
</h3>

Quatre blocs optionnels de niveau supérieur, `access_control`, `limits`, `timeouts` et `rate_limits`, règlent la surface HTTP. Les valeurs par défaut conviennent à la plupart des déploiements.

| Bloc             | Clé                                            | Par défaut | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ---------------- | ---------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | vide       | Autorisation/refus IP entrant par adresse client, après résolution `trusted_proxies`. `deny_cidrs` est vérifié en premier ; un client qu'il correspond est rejeté même si `allow_cidrs` correspond également. Si `allow_cidrs` est non vide, la passerelle est par défaut refusée. `/healthz` et `/readyz` sont exempts de `allow_cidrs`. Lorsqu'un proxy de confiance envoie une entrée `X-Forwarded-For` qui n'est pas une adresse IP, le vrai client est inconnu et la passerelle enregistre un avertissement une fois nommant ce qu'il faut vérifier. Où l'une ou l'autre liste s'applique à la demande, elle la refuse avec `403` et la raison d'audit `xff_unparseable`. Où aucune ne le fait, elle sert la demande et utilise l'adresse propre du proxy comme adresse IP client pour les limites de débit par IP et l'audit. |
| `limits`         | `max_request_bytes`                            | 32 MiB     | Corps de demande entrant max ; les demandes surdimensionnées obtiennent `413` avant que le corps ne soit mis en mémoire tampon. Augmentez pour les demandes de fichiers ou d'images volumineux.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `limits`         | `max_request_header_bytes`                     | non défini | Lorsqu'il est défini, les en-têtes surdimensionnés retournent `431`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `limits`         | `max_url_length`                               | non défini | Lorsqu'il est défini, une URL trop longue retourne `414`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000     | Attente max pour les en-têtes de réponse en amont (temps jusqu'au premier octet). Le corps de la réponse s'écoule ensuite sans plafond mural. S'applique au chemin d'amont Anthropic direct ; sur tous les autres fournisseurs, la passerelle attend jusqu'à une heure pour que la réponse commence.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600   | Limite de débit par IP sur le point de terminaison d'autorisation d'appareil non authentifié. Augmentez pour une grande organisation derrière une adresse IP de sortie partagée ou NAT. [Déploiements à grande échelle](/docs/fr/claude-apps-gateway-deploy#large-rollouts) montre comment le dimensionner. Ces limites s'appliquent uniquement au flux de connexion par octroi d'appareil, pas à l'inférence `/v1/messages`. Consultez [Résistance à la force brute du code utilisateur](/docs/fr/claude-apps-gateway-deploy#user-code-brute-force-resistance).                                                                                                                                                                                                                                                                              |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600   | Limite de débit par IP sur les soumissions `user_code` à `/device`. C'est ce qui empêche quelqu'un de deviner le code d'un autre développeur. [Déploiements à grande échelle](/docs/fr/claude-apps-gateway-deploy#large-rollouts) montre jusqu'où l'augmenter.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

Si vous laissez les deux listes `access_control` vides, ce qui est la valeur par défaut, la passerelle sert toute adresse client, afin que seul votre réseau restreigne qui peut l'atteindre. C'est important car une passerelle peut pousser des [paramètres gérés](#managed) qui exécutent des commandes sur les machines des développeurs.

Tandis que `allow_cidrs` est vide, la passerelle avertit à deux endroits, sans changer la façon dont elle répond à toute demande :

* **Au démarrage** : un avertissement dans le journal opérationnel recommande d'autoriser uniquement les plages privées `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `100.64.0.0/10`, `127.0.0.0/8`, `::1/128` et `fc00::/7`, plus toute autre plage interne à partir de laquelle vos développeurs se connectent. Si vous liez la passerelle à une adresse de bouclage et ne définissez ni `trusted_proxies` ni `public_url`, comme dans le développement local, l'avertissement n'apparaît pas.
* **À l'exécution** : la première fois qu'une demande arrive d'une adresse en dehors de ces plages privées, la passerelle enregistre un avertissement et émet un événement d'audit [`access.public_client`](/docs/fr/claude-apps-gateway-deploy#logs) portant l'adresse IP du client. Les deux se déclenchent une fois par processus. Les adresses lien-local, `169.254.0.0/16` et `fe80::/10`, ne comptent pas comme publiques. La passerelle répond à `/healthz` et `/readyz` avant cette vérification, afin que les sondes de santé à partir de plages publiques ne la déclenchent pas.

Les deux signaux utilisent l'adresse client telle que la passerelle la résout. Si un équilibreur de charge, un port-forward ou un tunnel relaie le trafic et n'est pas listé dans `listen.trusted_proxies`, la passerelle voit l'adresse du relais, qui est généralement privée, afin que ni l'avertissement d'exécution ni une liste d'autorisation privée ne l'attrape.

Derrière un tel front-end, définissez [`listen.trusted_proxies`](#listen) en premier afin que la passerelle voie les vraies adresses client, et gardez la passerelle et tout ce qui se trouve devant elle inaccessible à partir d'Internet public indépendamment.

<h3 id="load_test_mode">
  `load_test_mode`
</h3>

Le bloc `load_test_mode` vous permet de tester la charge d'une passerelle sans appeler un fournisseur de modèles. Tandis qu'il est activé, la passerelle construit et signe chaque demande de fournisseur comme d'habitude, la rejette au lieu de l'envoyer, et diffuse une réponse en conserve via son chemin de réponse normal. La réponse est un texte de remplissage qui commence par une phrase disant qu'elle est en conserve.

Nécessite v2.1.283 ou ultérieur. Les versions antérieures refusent de démarrer lorsque la clé est définie, afin de mettre à niveau chaque réplica avant d'ajouter le bloc et de le supprimer avant de revenir en arrière.

L'exemple ci-dessous active le mode avec les valeurs par défaut, une réponse de 750 jetons de sortie diffusée sur environ 10 secondes :

```yaml theme={null}
load_test_mode:
  enabled: true
  reply_tokens: 750     # roughly how many tokens of text each canned reply carries
  reply_seconds: 9.5    # how long a streamed reply takes
```

| Champ           | Requis | Description                                                                                                                                                                            |
| --------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`       | Oui    | `true` active le mode. `false` conserve vos nombres dans le fichier avec le mode désactivé. La passerelle refuse de démarrer si le bloc est présent sans lui.                          |
| `reply_tokens`  | Non    | Par défaut `750`. Environ combien de jetons de texte chaque réponse en conserve porte, un nombre entier de 1 à 100 000.                                                                |
| `reply_seconds` | Non    | Par défaut `9.5`. Combien de temps une réponse diffusée prend, de 0 à 600. `0` envoie la réponse entière à la fois. Une réponse à une demande non diffusée revient toujours à la fois. |

Un test de charge dans ce mode couvre la passerelle, votre Postgres et tout ce qui se trouve devant la passerelle. Il ne couvre pas les limites, la vitesse ou le chemin réseau du fournisseur.

Tandis que le mode est activé, une demande peut porter un en-tête `x-load-test-user` contenant un nombre entier de jusqu'à sept chiffres, et la passerelle compte chaque nombre comme un développeur distinct avec l'e-mail et les groupes du développeur dont le jeton est venu avec la demande. Donnez au déploiement de test de charge sa propre base de données vide, car la passerelle refuse de démarrer avec le mode activé par rapport à une base de données dans laquelle un développeur a déjà dépensé quelque chose.

<Warning>
  Ne jamais activer ceci pour une passerelle que les développeurs utilisent. Chaque demande obtient la réponse en conserve et aucun modèle n'est appelé. La passerelle enregistre un avertissement `load_test_mode is on` au démarrage et marque chaque événement d'audit [`inference`](/docs/fr/claude-apps-gateway-deploy#logs) avec `load_test: true` tandis que le mode est activé.
</Warning>

<h2 id="complete-example">
  Exemple complet
</h2>

Cette configuration de référence complète exerce chaque section centrale ; les [blocs de tuning HTTP](#http-tuning) conservent leurs valeurs par défaut. Copiez-la, supprimez ce dont vous n'avez pas besoin, et remplissez vos valeurs. La configuration dans le [Démarrage rapide](/docs/fr/claude-apps-gateway#quickstart) est une version minimale de celle-ci.

```yaml gateway.yaml theme={null}
# Exécutez avec :
#   claude gateway --config gateway.yaml
#
# La verbosité du journal opérationnel est contrôlée par la variable
# d'environnement CLAUDE_GATEWAY_LOG_LEVEL
# (debug | info | warn | error ; par défaut info). debug
# enregistre également les noms de réclamations dans chaque id_token, pour le diagnostic groups_claim.
# Cela n'affecte pas les événements d'audit, qui sont toujours émis.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omettez le bloc tls lors de l'exécution derrière une entrée qui termine TLS.
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
  # Requis lorsque l'émetteur est le serveur d'organisation Okta, dont les id_tokens
  # peuvent omettre l'e-mail et les groupes ; la passerelle les remplit à partir de /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta émet des groupes uniquement lorsque la portée `groups` est demandée et que
  # le filtre de réclamation de groupes de l'application les autorise. La politique
  # des entrepreneurs ci-dessous correspond aux groupes, donc la portée est demandée ici.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Rôles d'application Entra : utilisez `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5
  # connect_timeout_seconds: 5

# Active /v1/organizations/spend_limits (reflète l'API Admin Anthropic)
# et l'application des limites de dépenses par développeur sur /v1/messages. Omettez pour désactiver.
# Les plafonds eux-mêmes sont définis via l'API admin, pas ici.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

# Testez en charge ce déploiement sans appeler un fournisseur de modèle. Jamais sur une
# passerelle que les développeurs utilisent : chaque demande reçoit une réponse en conserve.
# load_test_mode:
#   enabled: true
#   # reply_tokens: 750
#   # reply_seconds: 9.5

# Mesurez aux tarifs contractuels au lieu du prix catalogue USD. Nécessite admin: ou une
# politique managed:. Avec managed:, les mêmes tarifs vont également aux clients connectés.
# Les tarifs ci-dessous sont des espaces réservés, pas des prix de contrat réels.
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

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
        # Limitez l'option du sélecteur par défaut à availableModels au lieu de
        # la valeur par défaut du niveau, afin que les entrepreneurs n'obtiennent pas une erreur 400 sur la valeur par défaut.
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

Tout ce qui précède configure le serveur de passerelle. Pointer les machines des développeurs vers celui-ci est configuré séparément, sur chaque appareil, via les [paramètres gérés](/docs/fr/managed-settings) de Claude Code. La passerelle ne peut pas pousser les clés de connexion elle-même, car ce sont elles qui disent au client où se trouve la passerelle.

Pour le CLI, définissez ces clés dans le `managed-settings.json` par système d'exploitation. Les deux clés de connexion acheminent chaque `/login` du développeur vers votre passerelle :

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"` maintient le fonctionnement de la livraison de la liste d'autorisation de sortie de Claude Desktop vers ses sessions Claude Code intégrées ; [Deliver policy to Claude Desktop sessions](/docs/fr/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) explique le mécanisme et où l'opt-in doit se situer.

Déployez le fichier `managed-settings.json` sur chaque appareil, généralement via votre plateforme MDM. Le chemin du fichier diffère selon la plateforme. Voir [où chaque mécanisme stocke la stratégie](/docs/fr/managed-settings#where-each-mechanism-stores-the-policy).

Par défaut, une stratégie de registre sur Windows ou un plist de préférences gérées sur macOS remplace le fichier `managed-settings.json` plutôt que de le fusionner avec lui, à l'exception des [clés d'exception et des vérifications entre sources ci-dessus](#precedence-with-other-managed-sources). Les trois clés de cet extrait suivent la règle de source de priorité la plus élevée, donc les flottes qui livrent la stratégie via Group Policy ou les profils de configuration doivent placer les trois dans ce mécanisme à la place.

Pour Claude Desktop, définissez la clé `bootstrapUrl` dans la propre [configuration gérée](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop sur `<listen.public_url>/user/bootstrap`. Le flux de connexion et la stratégie par groupe correspondent alors à ceux du CLI une fois qu'une stratégie opte pour le serveur avec une clé `desktop` ; sans l'opt-in, `/user/bootstrap` retourne 404. Voir [Claude Desktop overlay](#claude-desktop-overlay) pour la moitié côté serveur.

Claude Code honore [`forceLoginGatewayUrl`](/docs/fr/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/fr/settings-reference#gatewayinternalnetworks), et la valeur `"gateway"` de [`forceLoginMethod`](/docs/fr/settings-reference#forceloginmethod) uniquement à partir d'une source gérée sur la machine : `managed-settings.json`, le plist macOS ou le registre HKLM Windows, ou un assistant de stratégie. Un développeur les définissant dans son propre `~/.claude/settings.json` n'a aucun effet, et il en va de même pour les définir dans la charge utile de la passerelle.

<h2 id="related">
  Connexes
</h2>

* [Aperçu de la passerelle Claude apps](/docs/fr/claude-apps-gateway) : démarrage rapide et connexion des développeurs
* [Guide de déploiement](/docs/fr/claude-apps-gateway-deploy) : configuration IdP, image de conteneur, Kubernetes et Cloud Run, et opérations
* [Limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits) : plafonds par développeur et API Admin
