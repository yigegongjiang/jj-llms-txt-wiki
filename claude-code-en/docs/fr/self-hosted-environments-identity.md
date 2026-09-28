> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Vérifier l'identité de session dans les environnements auto-hébergés

> Vérifiez le JWT CLAUDE_CODE_SESSION_ACCESS_TOKEN afin que les services de votre réseau puissent faire confiance aux demandes provenant de sessions dans votre environnement auto-hébergé.

<Note>
  Les environnements auto-hébergés sont en bêta publique sur les plans Team et Enterprise ; un [propriétaire](/docs/fr/cloud-environments#organization-shared-environments) les active en activant **Allow self-hosted environments** sur la [page d'administration **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Cette page couvre la vérification de l'identité de session ; consultez le [guide de démarrage rapide](/docs/fr/self-hosted-environments-quickstart) pour la configuration et [Déployer en production](/docs/fr/self-hosted-environments-deploy) pour les recettes de flotte.
</Note>

Un [environnement auto-hébergé](/docs/fr/self-hosted-environments) permet aux sessions [Claude Code sur le web](/docs/fr/claude-code-on-the-web) de s'exécuter sur l'infrastructure que vous exploitez au lieu de celle d'Anthropic. Parce que la session s'exécute à l'intérieur de votre réseau, Claude peut appeler directement vos services internes. Ces services ont besoin d'un moyen de confirmer qu'une demande provient d'une session Claude Code dans votre environnement, et d'identifier l'identité de l'utilisateur ou du service qui a créé cette session.

Chaque session dans un environnement auto-hébergé reçoit un JSON Web Token (JWT) signé dans la variable d'environnement `CLAUDE_CODE_SESSION_ACCESS_TOKEN`. Une session présente le token comme n'importe quelle credential de porteur ; par exemple, un script que Claude exécute peut appeler votre service avec `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"`. Anthropic signe le token et publie les clés de vérification à un endpoint JWKS public. Vos services récupèrent ces clés, vérifient la signature et lisent les claims pour décider quel accès accorder.

<h2 id="the-session-token">
  Le token de session
</h2>

Avant d'écrire le code de vérification, comprenez ce que le token établit et la forme que votre bibliothèque JWT verra.

<h3 id="what-the-token-proves">
  Ce que le token prouve
</h3>

Un token valide établit certains faits et délibérément pas d'autres :

* **Prouve** : Anthropic a émis le token pour une session spécifique dans un environnement spécifique, et comment la session a été créée : par un utilisateur de votre organisation, ou par l'identité de service de votre organisation, ce qui est la façon dont les [sessions de canal Claude Tag](https://claude.com/docs/claude-tag/concepts/agent-identity) commencent
* **Ne prouve pas** : quel processus sur l'hôte du runner le présente. Le token se trouve dans une variable d'environnement à l'intérieur de la session, donc tout code que Claude exécute, et tout outil ou serveur MCP que la session démarre, peut le lire et le présenter.

Deux conséquences pour vos services :

* Vérifiez le claim `aud` par rapport à votre ID d'environnement, la valeur `ccpool_...` affichée avec votre environnement sur la [page d'administration **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), pour rejeter les tokens émis pour l'environnement de toute autre organisation.
* Limitez les credentials que vous dérivez du token à ce qu'une seule session de codage devrait pouvoir faire, pas à tout ce que le créateur de la session peut faire. Voir [Limiter les credentials dérivés](#scope-derived-credentials).

<h3 id="token-format">
  Format du token
</h3>

La valeur de `CLAUDE_CODE_SESSION_ACCESS_TOKEN` a un préfixe `sk-ant-cc-` suivi d'un JWT standard en trois parties :

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

Supprimez le préfixe avant de passer la valeur à une bibliothèque JWT. Les tokens émis pour les sessions cloud hébergées par Anthropic portent un préfixe `sk-ant-si-` à la place et sont signés par un ensemble de clés différent, donc rejetez toute valeur qui ne commence pas par `sk-ant-cc-`.

L'algorithme de signature est `ES256`, qui est ECDSA sur la courbe P-256 avec SHA-256. L'en-tête du token porte un `kid` qui identifie quelle clé dans le JWKS l'a signé.

<h2 id="verify-the-token">
  Vérifier le token
</h2>

La vérification s'exécute dans l'un de deux endroits. Les services de votre réseau vérifient le token de manière cryptographique par rapport aux clés publiées d'Anthropic, et les scripts wrapper à l'intérieur de la session peuvent utiliser le décodeur intégré du binaire du runner à la place.

<h3 id="verify-the-token-from-your-service">
  Vérifier le token depuis votre service
</h3>

Anthropic publie les clés de vérification à un endpoint public et non authentifié :

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

La réponse est un [JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517) standard. Anthropic fait tourner les clés de signature périodiquement, et les clés d'avant une rotation restent dans l'ensemble assez longtemps pour que les tokens qu'elles ont signés continuent à se vérifier, donc ne fixez pas une seule clé. L'endpoint définit `Cache-Control: public, max-age=300`, donc mettre en cache l'ensemble de clés et refetcher toutes les cinq minutes est sûr.

Vérifiez chaque token entrant par rapport à ces vérifications :

<Steps>
  <Step title="Vérifier le préfixe">
    Rejetez la valeur si elle ne commence pas par `sk-ant-cc-`, puis supprimez ce préfixe. Le reste est un JWT compact standard.
  </Step>

  <Step title="Vérifier la signature">
    Récupérez le JWKS, sélectionnez la clé dont le `kid` correspond à l'en-tête du token, et vérifiez la signature `ES256`. Rejetez les tokens dont l'en-tête `alg` n'est pas `ES256`. Si un token arrive avec un `kid` qui n'est pas dans votre ensemble de clés en cache, refetchez le JWKS une fois avant de le rejeter : après une rotation, les nouveaux tokens sont signés avec une clé que votre ensemble en cache n'a pas encore.
  </Step>

  <Step title="Vérifier l'émetteur">
    Rejetez le token si `iss` n'est pas exactement `ccr`.
  </Step>

  <Step title="Vérifier l'audience par rapport à votre environnement">
    Le claim `aud` est un tableau. Rejetez le token à moins qu'il ne contienne votre ID d'environnement, qui a la forme `ccpool_...`. L'ID d'environnement est affiché dans la boîte de dialogue de détail de votre environnement sur la [page d'administration **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), et apparaît comme le claim `ccr:pool_id` dans l'un des tokens de session de l'environnement. Cette vérification est ce qui limite le token à votre environnement et rejette les tokens émis pour d'autres organisations.
  </Step>

  <Step title="Vérifier le rôle">
    Rejetez le token si `ccr:role` n'est pas exactement `session_worker`. D'autres tokens émis pour les environnements auto-hébergés, tels que les secrets d'environnement, les tokens de runner et les ordres de travail, sont signés par le même ensemble de clés mais portent des rôles différents.
  </Step>

  <Step title="Vérifier l'expiration">
    Rejetez le token si `exp` est dans le passé. Anthropic émet les tokens de session avec une durée de vie de quatre heures par défaut et un maximum de huit heures. Le runner rafraîchit le token avant l'expiration et pousse la nouvelle valeur à la session, donc les sous-processus que Claude démarre après un rafraîchissement l'héritent. Une session peut donc présenter plusieurs tokens valides distincts à votre service au cours de sa durée de vie.
  </Step>

  <Step title="Lire l'identité">
    L'identité de l'utilisateur créateur est dans le claim `act` : `act.sub` est son ID utilisateur Anthropic sous la forme préfixée `user:<id>`, et `act.email`, quand la surface créatrice en a enregistré un, est son adresse e-mail. Les sessions que l'identité de service de votre organisation crée, y compris les sessions de canal Claude Tag, portent un sujet `agent:` à la place, donc traitez une session comme créée par l'utilisateur uniquement quand `act.sub` porte le préfixe `user:`, plutôt que de tester si les claims d'identité sont absents. Voir la [référence des claims](#claims-reference) pour la structure complète et les claims en double plats.
  </Step>
</Steps>

Les vérifications correspondent directement aux bibliothèques JWT standard. Les exemples ci-dessous implémentent la séquence complète en Node.js avec [`jose`](https://www.npmjs.com/package/jose), qui gère la récupération du JWKS, la mise en cache et la sélection du `kid`, et en Python avec [`PyJWT`](https://pyjwt.readthedocs.io/) et son client JWKS intégré.

<Tabs>
  <Tab title="Node.js (jose)">
    ```typescript theme={null}
    import { createRemoteJWKSet, jwtVerify } from "jose";

    const JWKS = createRemoteJWKSet(
      new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
    );

    const PREFIX = "sk-ant-cc-";
    const EXPECTED_POOL_ID = "ccpool_...";

    export async function verifySessionToken(raw: string) {
      if (!raw.startsWith(PREFIX)) {
        throw new Error("not a self-hosted runner session token");
      }
      const jwt = raw.slice(PREFIX.length);

      const { payload } = await jwtVerify(jwt, JWKS, {
        issuer: "ccr",
        audience: EXPECTED_POOL_ID,
        algorithms: ["ES256"],
      });

      if (payload["ccr:role"] !== "session_worker") {
        throw new Error("token is not a session_worker token");
      }

      const act = payload.act as { email?: string; sub?: string };
      return {
        sessionId: payload["ccr:session_id"] as string,
        poolId: payload["ccr:pool_id"] as string,
        orgId: payload["ccr:org_id"] as string,
        creatorEmail: act?.email,
        creatorSub: act?.sub,
      };
    }
    ```
  </Tab>

  <Tab title="Python (PyJWT)">
    ```python theme={null}
    import jwt
    from jwt import PyJWKClient

    JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
    PREFIX = "sk-ant-cc-"
    EXPECTED_POOL_ID = "ccpool_..."

    jwks = PyJWKClient(JWKS_URL)


    def verify_session_token(raw: str) -> dict:
        if not raw.startswith(PREFIX):
            raise ValueError("not a self-hosted runner session token")
        token = raw.removeprefix(PREFIX)

        signing_key = jwks.get_signing_key_from_jwt(token)
        payload = jwt.decode(
            token,
            signing_key.key,
            algorithms=["ES256"],
            issuer="ccr",
            audience=EXPECTED_POOL_ID,
        )

        if payload.get("ccr:role") != "session_worker":
            raise ValueError("token is not a session_worker token")

        act = payload.get("act") or {}
        return {
            "session_id": payload["ccr:session_id"],
            "pool_id": payload["ccr:pool_id"],
            "org_id": payload["ccr:org_id"],
            "creator_email": act.get("email"),
            "creator_sub": act.get("sub"),
        }
    ```
  </Tab>
</Tabs>

<h3 id="verify-the-token-inside-the-session">
  Vérifier le token à l'intérieur de la session
</h3>

Les [scripts wrapper](/docs/fr/self-hosted-environments-configuration#wrapper-scripts) s'exécutent à l'intérieur de la session, avant que Claude ne démarre. Au lieu d'appeler une bibliothèque JWT, ils peuvent exécuter la sous-commande `self-hosted-runner decode-token` du binaire du runner. La sous-commande lit le token à partir d'un argument positionnel, de `CLAUDE_CODE_SESSION_ACCESS_TOKEN`, ou de stdin canalisé, dans cet ordre, puis supprime le préfixe, vérifie la signature par rapport à l'endpoint JWKS, vérifie l'expiration et imprime les claims en JSON. La sous-commande effectue uniquement les vérifications de signature et d'expiration ; elle ne vérifie pas `iss`, `aud` ou `ccr:role`. Quand la décision d'authentification de votre wrapper dépend de ces claims, lisez-les à partir du JSON imprimé et comparez-les explicitement.

Cette commande extrait l'identité du créateur, en préférant le sujet du fournisseur SSO, puis l'adresse e-mail, puis le sujet `act.sub` du créateur, `user:<id>` ou `agent:<id>` :

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.attested_by.sub // .act.email // .act.sub'
```

Les wrappers reçoivent le chemin absolu du binaire du runner lui-même dans `CLAUDE_RUNNER_CLAUDE_BIN` ; utilisez ce chemin plutôt qu'un `claude` résolu par PATH afin que le décodage s'exécute sur le même binaire que le runner lui-même utilise.

Utilisez `jq -re` plutôt que `jq -r` afin qu'un claim manquant provoque une sortie non-zéro. Avec `-r` seul, un claim manquant imprime la chaîne littérale `null` et sort zéro, ce qui transmet silencieusement une mauvaise valeur en aval. Passez `--no-verify` à `decode-token` uniquement pour l'inspection hors ligne où l'endpoint JWKS est inaccessible.

<h2 id="claims-reference">
  Référence des claims
</h2>

Le tableau ci-dessous énumère les claims du token de session pertinents pour la vérification. Lisez l'identité à partir de l'espace de noms `ccr:*` et de la chaîne `act` ; les claims plats `account_email`, `organization_uuid` et `account_uuid` sont des doublons de compatibilité rétroactive qui peuvent être supprimés. Les sessions que l'identité de service de votre organisation crée, y compris les sessions de canal Claude Tag, portent un sujet `agent:` dans `act.sub` et omettent `act.email`, `ccr:account_id`, `account_email` et `account_uuid`. Les deux claims d'e-mail sont également facultatifs pour les sessions créées par l'utilisateur : Anthropic les enregistre à la création de la session uniquement quand les credentials de la demande créatrice portent un e-mail, et une session envoyée depuis la CLI peut manquer les deux, donc basez l'identité sur `act.sub` ou `ccr:account_id` plutôt que sur l'e-mail. Les tokens peuvent également porter des claims supplémentaires au-delà de ce tableau ; ignorez les claims que vous ne reconnaissez pas.

| Claim               | Type             | Description                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------ | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `iss`               | string           | Toujours `ccr`.                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `sub`               | string           | `ccr:session:<session_id>`.                                                                                                                                                                                                                                                                                                                                                                                                          |
| `aud`               | array of strings | Contient toujours `anthropic-api`. Pour les sessions dans les environnements auto-hébergés, le tableau contient également votre ID d'environnement, tel que `ccpool_...`. Vérifiez l'ID d'environnement, pas `anthropic-api`.                                                                                                                                                                                                        |
| `exp`               | number           | Expiration en tant que timestamp Unix. Durée de vie par défaut de quatre heures, maximum de huit heures.                                                                                                                                                                                                                                                                                                                             |
| `iat`               | number           | Émis à titre de timestamp Unix.                                                                                                                                                                                                                                                                                                                                                                                                      |
| `jti`               | string           | Identifiant de token unique.                                                                                                                                                                                                                                                                                                                                                                                                         |
| `ccr:role`          | string           | Toujours `session_worker` pour les tokens de session.                                                                                                                                                                                                                                                                                                                                                                                |
| `ccr:session_id`    | string           | L'ID de session. Même valeur que le suffixe de `sub`.                                                                                                                                                                                                                                                                                                                                                                                |
| `ccr:pool_id`       | string           | Votre ID d'environnement. Même valeur qui apparaît dans `aud`.                                                                                                                                                                                                                                                                                                                                                                       |
| `ccr:org_id`        | string           | Votre ID d'organisation Anthropic.                                                                                                                                                                                                                                                                                                                                                                                                   |
| `ccr:account_id`    | string           | L'ID de compte Anthropic de l'utilisateur créateur : la valeur de `act.sub` sans le préfixe `user:`, un ID `user_...` étiqueté. La même valeur que le `CLAUDE_RUNNER_ACCOUNT_ID` du [hook spawn-runner](/docs/fr/self-hosted-environments-configuration#the-spawn-runner-hook) porte et que [`--lock-to-account`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) accepte, donc les trois se comparent comme des chaînes égales. |
| `account_email`     | string           | Doublon de `act.email` ; absent chaque fois que `act.email` l'est.                                                                                                                                                                                                                                                                                                                                                                   |
| `organization_uuid` | string           | Votre UUID d'organisation Anthropic.                                                                                                                                                                                                                                                                                                                                                                                                 |
| `account_uuid`      | string           | L'UUID de compte Anthropic de l'utilisateur créateur.                                                                                                                                                                                                                                                                                                                                                                                |
| `act`               | object           | Chaîne de délégation [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693). Voir [La chaîne `act`](#the-act-chain).                                                                                                                                                                                                                                                                                                                     |

<h3 id="the-act-chain">
  La chaîne `act`
</h3>

Le claim `act` enregistre le chemin de délégation complet de l'identité de l'utilisateur ou du service qui a créé la session jusqu'à l'[environnement](/docs/fr/self-hosted-environments#key-concepts) dont le secret a admis le runner, et l'identité qui a créé ce secret. Le créateur est l'acteur le plus externe, donc `act.sub` l'identifie directement.

| Path              | Description                                                                                                                                                                                                                                                                        |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `act.sub`         | L'ID utilisateur Anthropic de l'utilisateur créateur, sous la forme `user:<id>`, ou `agent:<id>` quand l'identité de service de votre organisation a créé la session, comme elle le fait pour les sessions de canal Claude Tag.                                                    |
| `act.email`       | L'adresse e-mail de l'utilisateur créateur, quand une a été enregistrée à la création de la session. Ne l'exigez pas ; basez-vous sur `act.sub`.                                                                                                                                   |
| `act.attested_by` | L'attestation du fournisseur d'identité en amont pour l'utilisateur créateur, quand disponible. `act.attested_by.sub` est le sujet que votre fournisseur SSO, tel que Google ou Okta, a émis. Préférez ceci à `act.email` lors du mappage aux identités dans vos propres systèmes. |
| `act.act`         | Le runner qui a généré la session. `act.act.sub` est `ccr:runner:<runner_id>`.                                                                                                                                                                                                     |
| `act.act.act`     | L'environnement. `act.act.act.sub` est `ccr:pool:<pool_id>`.                                                                                                                                                                                                                       |
| `act.act.act.act` | L'identité qui a créé le secret d'environnement avec lequel le runner s'est enregistré. La chaîne se termine ici.                                                                                                                                                                  |

<h2 id="scope-derived-credentials">
  Limiter les credentials dérivés
</h2>

Le token de session identifie l'utilisateur ou l'identité de service qui a créé la session, mais ne le traitez pas comme équivalent à ce créateur se connectant directement. Le token se trouve dans une variable d'environnement à l'intérieur de la session, donc tout code que Claude exécute, et tout outil ou serveur MCP que la session démarre, peut le lire et le présenter.

La vérification est également hors ligne : un token qui se vérifie par rapport au JWKS reste valide jusqu'à son `exp`, quoi qu'il se soit passé pour la session depuis, et Anthropic ne publie pas un flux de révocation pour les tokens de session. Limitez tout ce que vous dérivez du token en conséquence.

Quand votre service échange le token pour des credentials internes, émettez des credentials limités à ce qu'une seule session de codage devrait atteindre :

* **Limiter les capacités** : accordez l'accès en lecture et en écriture aux ressources dont la session a besoin pour les tâches de codage, pas aux capacités administratives que le créateur détient ailleurs.
* **Limiter la durée de vie** : limitez les credentials dérivés à l'`exp` du token, ou moins.
* **Auditer en tant que session** : enregistrez le `ccr:session_id` et le `jti` aux côtés de l'identité du créateur afin de pouvoir retracer les actions jusqu'à une session spécifique.

<h2 id="related-environment-variables">
  Variables d'environnement associées
</h2>

L'identité du créateur apparaît également dans les variables d'environnement en clair sur deux surfaces qui ne vérifient jamais le token :

* **Le [hook `spawn-runner`](/docs/fr/self-hosted-environments-configuration#the-spawn-runner-hook), sur l'orchestrateur** : le hook s'exécute avant que tout runner n'existe pour une session en attente et reçoit l'identité du créateur dans des variables telles que `CLAUDE_RUNNER_ACCOUNT_EMAIL` et `CLAUDE_RUNNER_ACCOUNT_ID`. L'orchestrateur les lit à partir de l'ordre de travail, le token à usage unique signé qui autorise le spawning d'un runner, sans vérifier la signature de l'ordre de travail lui-même ; les claims sont de confiance car l'ordre de travail arrive sur la connexion de l'orchestrateur à Anthropic, que le secret d'environnement authentifie.
* **[Scripts wrapper](/docs/fr/self-hosted-environments-configuration#wrapper-scripts), à l'intérieur de la session** : les wrappers reçoivent `CCR_SESSION_ACCOUNT_EMAIL`, l'e-mail du créateur pré-extrait du token sans vérification de signature. La variable convient pour l'étiquetage, tel que les remorques de commit, pas pour les décisions d'authentification.

Utilisez les variables en clair pour les décisions du côté de l'orchestrateur telles que la sélection d'une image de machine. Utilisez `CLAUDE_CODE_SESSION_ACCESS_TOKEN` quand un service en aval a besoin d'une preuve cryptographique indépendante plutôt que de faire confiance à l'environnement du runner.

<h2 id="what’s-next">
  Prochaines étapes
</h2>

* [Environnements auto-hébergés](/docs/fr/self-hosted-environments) : l'environnement, le runner et le modèle de session ; le [guide de démarrage rapide](/docs/fr/self-hosted-environments-quickstart) et [Déployer en production](/docs/fr/self-hosted-environments-deploy) contiennent la configuration et les opérations
* [Personnaliser les sessions](/docs/fr/self-hosted-environments-configuration) : les scripts wrapper qui consomment le token, et le hook `spawn-runner`
* [Référence](/docs/fr/self-hosted-environments-reference) : les drapeaux CLI, les variables d'environnement et les métriques
