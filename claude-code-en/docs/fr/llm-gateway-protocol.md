> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence du protocole de passerelle

> Le contrat API entre Claude Code et une passerelle LLM : points de terminaison, en-têtes et champs de corps à transmettre, dégradation des fonctionnalités lorsque les champs sont supprimés, en-têtes d'attribution pour le suivi des coûts et découverte des modèles.

Cette page documente les requêtes que Claude Code envoie à une passerelle, y compris les points de terminaison qu'elle appelle, les en-têtes et champs de corps que la passerelle doit transmettre, et les fonctionnalités qui cessent de fonctionner si elle ne le fait pas. Elle est écrite pour les opérateurs configurant un produit de passerelle pour fonctionner avec Claude Code.

Une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) en cours d'exécution sert une version lisible par machine de ce contrat à `GET /protocol`, couvrant les mêmes exigences de transmission plus les points de terminaison spécifiques à la passerelle d'applications Claude pour la connexion SSO, la livraison des paramètres gérés et la télémétrie. La passerelle d'applications Claude s'exécute à partir du même binaire `claude` que l'interface de ligne de commande, donc le [démarrage rapide de la passerelle d'applications Claude](/docs/fr/claude-apps-gateway#quickstart) est le chemin le plus court vers une instance en cours d'exécution à partir de laquelle vous pouvez récupérer la spécification.

<Note>
  * Pour déployer une passerelle existante ou tierce pour votre organisation, consultez [Déployer une passerelle LLM](/docs/fr/llm-gateway-rollout)
  * Si vous êtes un développeur individuel authentifiant Claude Code à une passerelle avec une identifiant que vous avez reçu, consultez [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect)
</Note>

Cette page couvre :

* [Formats API](#api-formats) et les points de terminaison à servir pour chacun
* [En-têtes de requête](#request-headers) : lesquels doivent atteindre l'amont et lesquels votre passerelle peut consommer
* Le [bloc d'attribution du message système](#system-prompt-attribution-block) et comment il interagit avec la mise en cache des invites
* [Transmission des fonctionnalités](#feature-pass-through) : ce qui se casse lorsque les en-têtes ou champs de corps sont supprimés
* [Découverte des modèles](#model-discovery)

Cette page utilise deux termes pour ce que votre passerelle fait avec chaque en-tête et champ de corps :

* **Transmettre inchangé** : le passer à l'amont octet par octet
* **Consommer** : la passerelle peut le lire pour le routage, l'attribution ou le suivi et n'a pas besoin de le transmettre

Tout ce qui n'est pas marqué comme transmis inchangé est à votre disposition pour être consommé ou ignoré.

<h2 id="api-formats">
  Formats API
</h2>

Une passerelle doit exposer au moins l'un des formats API suivants aux clients Claude Code. Le format que Claude Code utilise est déterminé par la configuration du client : la variable dans la colonne Sélectionné par du tableau ci-dessous pointe Claude Code vers votre passerelle dans ce format. Google Cloud's Agent Platform est le point de terminaison Claude de Google Cloud, anciennement Vertex AI ; ses noms de variables conservent l'orthographe `VERTEX`.

| Format                                   | Sélectionné par                                               | Points de terminaison                                                     | Transmettre inchangé                                                                                             |
| :--------------------------------------- | :------------------------------------------------------------ | :------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------- |
| Messages Anthropic                       | `ANTHROPIC_BASE_URL`                                          | `/v1/messages`, `/v1/messages/count_tokens` (optionnel)                   | En-têtes de requête `anthropic-beta` et `anthropic-version`                                                      |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` avec `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`     | Champs de corps de requête `anthropic_beta` et `anthropic_version`                                               |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` avec `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (optionnel) | En-têtes de requête `anthropic-beta` et `anthropic-version`, et le champ de corps de requête `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry et Claude Platform on AWS
</h3>

Microsoft Foundry et la [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) implémentent le format Messages Anthropic. Claude Code les route via leurs propres variables, `ANTHROPIC_FOUNDRY_BASE_URL` et `ANTHROPIC_AWS_BASE_URL`, mais une passerelle les frontalisant implémente la ligne Messages Anthropic ci-dessus. Une passerelle frontalisant la Claude Platform on AWS doit également transmettre l'en-tête `anthropic-workspace-id`, que [cette plateforme exige sur chaque requête](/docs/fr/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Points de terminaison optionnels et trafic de démarrage
</h3>

Les points de terminaison de comptage de jetons sont les seuls optionnels : lorsqu'ils sont absents, Claude Code estime l'utilisation du contexte localement. Les requêtes d'inférence sont envoyées à `/v1/messages?beta=true`, donc faites correspondre le chemin, pas l'URL complète. La méthode Google Cloud's Agent Platform ajoute des suffixes au chemin du modèle de l'éditeur, comme dans `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`.

Une passerelle voit également du trafic de démarrage au meilleur effort qu'elle peut rejeter sans rien casser : une sonde de connectivité `HEAD /`, et sur les passerelles au format Amazon Bedrock une requête `GET /inference-profiles?type=SYSTEM_DEFINED`.

<h3 id="streaming">
  Streaming
</h3>

Les réponses d'inférence doivent être en streaming. Claude Code consomme les événements envoyés par le serveur au fur et à mesure qu'ils arrivent, donc une passerelle qui met en mémoire tampon les réponses complètes avant de les relayer bloque le client.

<h3 id="format-mismatch-with-the-upstream">
  Incompatibilité de format avec l'amont
</h3>

Le format que le client utilise détermine ce que votre passerelle reçoit. Le mode de défaillance courant est une incompatibilité entre le format que le client envoie à votre passerelle et le format que le fournisseur amont derrière elle accepte.

* Lorsque le client utilise le format Amazon Bedrock ou Google Cloud's Agent Platform, Claude Code envoie uniquement le sous-ensemble de son ensemble de capacités complet que ces fournisseurs acceptent
* Lorsque le client utilise le format Messages Anthropic, Claude Code envoie l'ensemble complet, même si votre passerelle transmet à un amont Amazon Bedrock ou Google Cloud's Agent Platform

Combler cette différence est le travail de votre passerelle. [Transmission des fonctionnalités](#feature-pass-through) décrit ce qui se casse lorsqu'elle ne le fait pas.

<h2 id="request-headers">
  En-têtes de requête
</h2>

Claude Code inclut ces en-têtes sur les requêtes API. Les noms d'en-têtes ne sont pas sensibles à la casse sur le fil. Transmettez `anthropic-version` et `anthropic-beta` inchangés, plus `anthropic-workspace-id` lorsque l'amont est la [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) ; le reste, la passerelle peut le consommer pour le routage, l'attribution et le suivi, et n'a pas besoin de le transmettre.

| En-tête                         | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Authorization`, `x-api-key`    | L'identifiant de passerelle du développeur, dans un ou les deux en-têtes selon la [variable d'identifiant](/docs/fr/llm-gateway-connect#set-the-credential-variable) qu'il définit                                                                                                                                                                                                                                                                                                                                |
| `anthropic-version`             | Version de l'API, actuellement `2023-06-01`. Les requêtes au format Amazon Bedrock et Agent Platform de Google Cloud portent également le champ de corps `anthropic_version`, dont la valeur est la chaîne de dialecte du fournisseur, pas la valeur de cet en-tête                                                                                                                                                                                                                                          |
| `anthropic-beta`                | Valeurs de capacité séparées par des virgules pour la requête. Transmettez l'en-tête verbatim ; ne mettez pas en liste blanche les valeurs individuelles, car l'ensemble change avec les versions de Claude Code. Lorsque le développeur s'authentifie avec une connexion claude.ai, ce qui est possible lorsque `ANTHROPIC_BASE_URL` est défini sans variable d'identifiant de passerelle, cet en-tête porte également une capacité OAuth que l'amont exige, et la supprimer échoue ces requêtes avec `401` |
| `x-claude-code-session-id`      | Un identifiant unique pour la session Claude Code actuelle. Utilisez-le pour agréger toutes les requêtes d'une session sans analyser les corps de requête                                                                                                                                                                                                                                                                                                                                                    |
| `x-claude-code-agent-id`        | Identifiant du [sous-agent](/docs/fr/sub-agents) qui a émis la requête, présent uniquement sur les requêtes d'un agent que Claude Code a généré dans la session. Utilisez-le avec l'ID de session pour attribuer le coût aux agents parallèles                                                                                                                                                                                                                                                                    |
| `x-claude-code-parent-agent-id` | Identifiant de l'agent qui a généré l'agent demandeur, présent uniquement pour les agents imbriqués                                                                                                                                                                                                                                                                                                                                                                                                          |

Les ID de sous-agent sont générés à nouveau pour chaque génération. Les agents coéquipiers, les membres nommés d'une [équipe d'agents](/docs/fr/agent-teams), réutilisent un ID stable basé sur le nom à travers les reconnexions. Dans les deux cas, l'ID identifie un agent, pas une personne ou un appareil, donc ne traitez pas l'en-tête d'ID d'agent comme un identifiant d'utilisateur.

Si vos développeurs définissent `ANTHROPIC_CUSTOM_HEADERS`, ces en-têtes apparaissent également sur les requêtes.

<h3 id="forward-as-open-lists">
  Transmettre comme listes ouvertes
</h3>

Traitez les en-têtes et champs de corps comme des listes ouvertes, pas fermées. Claude Code gagne des capacités au fil des versions, et elles arrivent comme de nouvelles valeurs `anthropic-beta`, de nouveaux champs de corps de requête, et occasionnellement de nouveaux en-têtes `anthropic-*` ou `x-claude-code-*`.

Lors de la transmission à un amont au format Anthropic, transmettez les en-têtes de requête `anthropic-*` et les champs de corps de requête inchangés plutôt que de mettre en liste blanche ceux que vous voyez aujourd'hui. Une passerelle épinglée à une liste observée supprime l'en-tête ou le champ de la capacité suivante et la casse à la version qui l'introduit.

L'exception est un amont non-Anthropic tel qu'Amazon Bedrock ou Agent Platform de Google Cloud, où combler la différence de schéma est le travail de la passerelle ; consultez [transmission des fonctionnalités](#feature-pass-through).

<h2 id="system-prompt-attribution-block">
  Bloc d'attribution du message système
</h2>

Claude Code ajoute un bloc d'attribution court au message système contenant la version du client et une empreinte dérivée de la conversation. Le point de terminaison `api.anthropic.com` supprime le bloc avant le traitement lorsqu'il arrive inchangé comme premier bloc système, donc il n'affecte pas la mise en cache des invites de première partie. Tout autre amont le reçoit comme faisant partie de l'invite.

La suppression est positionnelle, donc elle ne fonctionne que lorsque la passerelle transfère le tableau `system` inchangé. Pour garder le bloc hors de l'invite sans perdre d'autre contenu système :

* Transférez le tableau `system` exactement tel que reçu, en gardant le bloc en premier : ajouter un autre bloc système, réorganiser le tableau ou le convertir en une seule chaîne annule la suppression, et le bloc atteint alors le modèle et la clé du cache d'invite.
* Gardez le bloc dans sa propre entrée de tableau : le point de terminaison traite un bloc fusionné qui commence par l'en-tête d'attribution comme une attribution dans son intégralité et supprime tout ce qui y est fusionné, y compris le reste du message système.
* Si votre passerelle doit remodeler le contenu système, définissez [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/fr/env-vars) pour que Claude Code omette le bloc. Anthropic et les points de terminaison Claude des fournisseurs de cloud lisent le bloc pour l'attribution, donc omettez-le au niveau du client plutôt que de le supprimer ou de le déplacer dans la passerelle.

Les requêtes qui atteignent le point de terminaison inmodifiées ne sont pas affectées.

À partir de Claude Code v2.1.181, le bloc est stable pour la durée de vie d'une conversation lorsque les requêtes sont routées via une URL de base personnalisée, donc un cache d'invite côté passerelle basé sur le corps de requête complet fonctionne sans le désactiver. Avant v2.1.181, le bloc incluait un jeton par requête ; sur ces versions, définissez `CLAUDE_CODE_ATTRIBUTION_HEADER=0` si votre passerelle implémente un tel cache.

<h2 id="feature-pass-through">
  Transmission des fonctionnalités
</h2>

Claude Code traite une passerelle `ANTHROPIC_BASE_URL` comme un point de terminaison au format Anthropic et lui envoie les en-têtes bêta et champs de corps de requête qu'il envoie à `api.anthropic.com`, sauf un petit ensemble de diagnostics et de valeurs par défaut réservés aux connexions directes, comme la valeur par défaut de streaming d'outils à grain fin couverte ci-dessous. Cet ensemble varie selon la version, donc ne dépendez pas de son contenu.

Les capacités qui ajoutent des champs de corps les associent à un en-tête bêta, et la paire voyage ensemble. Une passerelle qui supprime l'en-tête tout en transmettant le corps, ou transmet un corps au format Anthropic à un amont avec un schéma différent, produit des erreurs `400` dures ; seulement lorsque les deux moitiés sont absentes ensemble la fonctionnalité s'éteint silencieusement. Une passerelle qui réécrit ou rédige les corps de requête pour l'inspection du contenu casse l'appairage de la même manière que la suppression, donc inspectez sans modifier. Le tableau note où une fonctionnalité s'écarte de l'appairage.

Le streaming d'outils à grain fin est l'une des valeurs par défaut de connexion directe : il est désactivé par défaut chaque fois que les requêtes sont routées via une URL de base personnalisée, et une passerelle le reçoit lorsque les développeurs définissent [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/fr/env-vars).

| Fonctionnalité                                                                                                                                                                                                                              | Paire en-tête et corps                                                                                                                                                                                                                                    | Symptôme lorsque cassé                                                                                                                                 | Correction                                                                                                                                 |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| [Raisonnement adaptatif](/docs/fr/model-config#adjust-effort-level)                                                                                                                                                                              | Pas d'en-tête bêta. Claude Code envoie `thinking: {"type": "adaptive"}` pour Claude 4.6 et versions ultérieures, et traite les noms de modèles qu'il ne reconnaît pas, tels que les alias de passerelle, comme des modèles actuels qui reçoivent le champ | `400` nommant le champ `thinking` ou la balise `adaptive` lorsque la version du modèle amont ne l'accepte pas                                          | Mettez à niveau l'amont. Sur Opus 4.6 et Sonnet 4.6, les développeurs peuvent définir `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` à la place |
| [Gestion du contexte](https://platform.claude.com/docs/en/build-with-claude/context-management)                                                                                                                                             | L'en-tête bêta de gestion du contexte s'associe au champ de corps `context_management`                                                                                                                                                                    | `400` avec `Extra inputs are not permitted`. Courant lorsqu'une passerelle accepte les requêtes au format Anthropic mais les transmet à Amazon Bedrock | Transmettez les deux, ou [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/fr/env-vars)                                                        |
| [Contexte étendu](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) et [pensée entrelacée](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | En-têtes bêta uniquement, pas de champ de corps                                                                                                                                                                                                           | Silencieusement indisponible lorsque l'en-tête est supprimé ; l'amont ne voit jamais la demande de capacité                                            | Transmettez `anthropic-beta` verbatim                                                                                                      |
| Champs d'outil bêta                                                                                                                                                                                                                         | Les en-têtes bêta liés aux outils s'associent aux champs de schéma d'outil tels que `strict` et `defer_loading`                                                                                                                                           | `400` nommant le champ de schéma d'outil non reconnu lorsque le corps passe sans son en-tête                                                           | Transmettez les deux, ou `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`                                                                        |
| [Effort](https://platform.claude.com/docs/en/build-with-claude/effort) et [sorties structurées](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                   | Le champ de corps `output_config` porte les paramètres d'effort, de format de sortie structurée et de budget de tâche ; chacun s'associe à son propre en-tête bêta                                                                                        | `400` nommant `output_config`, souvent `Extra inputs are not permitted`, sur les amonts Amazon Bedrock et Agent Platform de Google Cloud               | Transmettez le champ et ses en-têtes ensemble                                                                                              |
| [Comptage des jetons](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                 | Pas d'appairage bêta ; utilise le point de terminaison `count_tokens`                                                                                                                                                                                     | Claude Code revient à l'estimation de l'utilisation du contexte localement                                                                             | Exposez le point de terminaison si vous voulez des comptages exacts                                                                        |

Les [variables](/docs/fr/model-config) `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` déclarent les capacités du modèle uniquement dans les configurations du fournisseur : `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, et [`CLAUDE_CODE_USE_MANTLE`](/docs/fr/amazon-bedrock#use-the-mantle-endpoint). Elles n'ont aucun effet derrière une passerelle `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Nouvelle tentative automatique et transmission d'erreur
</h3>

Claude Code réessaie automatiquement après certains rejets en amont et désactive la capacité rejetée pour le reste de la conversation. Les rejets du champ `thinking`, des [signatures de pensée](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), et des messages système en milieu de conversation se rétablissent tous de cette manière. Les rejets de gestion du contexte et de champ de schéma d'outil ne réessaient pas ; ces erreurs `400` atteignent le développeur.

La logique de nouvelle tentative correspond à la formulation d'erreur de l'amont, donc transmettez les corps de réponse d'erreur inmodifiés. Une passerelle qui enveloppe les erreurs en amont dans sa propre enveloppe casse le chemin de récupération même lorsqu'elle préserve le code d'état.

<h3 id="disable-pre-release-capabilities">
  Désactiver les capacités de pré-version
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` empêche Claude Code d'envoyer les capacités de pré-version et leurs champs de corps sur chaque fournisseur, y compris la gestion du contexte et les champs d'outil bêta. Il n'affecte pas le raisonnement adaptatif, qui est sélectionné par modèle plutôt que par bêta, et il ne supprime jamais la capacité OAuth que l'authentification par abonnement exige.

L'ensemble des capacités que Claude Code envoie augmente au fil des versions. Pour les chaînes d'en-tête bêta actuelles, consultez la [référence des en-têtes bêta](https://platform.claude.com/docs/en/api/beta-headers) ; testez votre passerelle contre les nouvelles versions de Claude Code plutôt que de vous épingler à une liste observée.

<h2 id="model-discovery">
  Découverte des modèles
</h2>

Lorsque `ANTHROPIC_BASE_URL` pointe vers une passerelle qui expose le format Messages Anthropic, Claude Code peut interroger le point de terminaison `/v1/models` de la passerelle au démarrage et ajouter les modèles retournés au sélecteur `/model`.

Les développeurs l'activent en définissant [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/fr/env-vars), dans leur propre environnement ou via les paramètres gérés. La découverte est désactivée par défaut afin que les passerelles soutenues par une clé API partagée ne surface pas chaque modèle auquel la clé peut accéder à chaque utilisateur. Cela nécessite Claude Code v2.1.129 ou version ultérieure.

<h3 id="when-discovery-runs">
  Quand la découverte s'exécute
</h3>

La découverte s'applique uniquement au format Messages Anthropic. Elle ne s'exécute pas lorsque :

* Toute variable de fournisseur `CLAUDE_CODE_USE_*` est définie, même si `ANTHROPIC_BASE_URL` est également défini
* `ANTHROPIC_BASE_URL` n'est pas défini ou pointe vers `api.anthropic.com`
* Le trafic non essentiel est désactivé, via [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars) ou la politique organisationnelle

<h3 id="request-and-response">
  Requête et réponse
</h3>

La requête est `GET /v1/models?limit=1000` avec un délai d'expiration de 3 secondes, et toute redirection est traitée comme un échec afin que l'identifiant ne puisse pas fuir vers une cible de redirection. Une passerelle qui répond lentement ou redirige `/v1/models`, même `http` vers `https`, échoue silencieusement la découverte ; servez le point de terminaison directement à l'URL de base configurée.

La requête de découverte envoie exactement un en-tête d'identifiant :

* `ANTHROPIC_AUTH_TOKEN` comme jeton porteur, lorsqu'il est défini
* Sinon la clé API résolue, y compris une valeur [`apiKeyHelper`](/docs/fr/llm-gateway-connect#rotate-credentials-with-apikeyhelper), dans l'en-tête `x-api-key`

Cela diffère des requêtes d'inférence, qui envoient une valeur d'aide dans les deux en-têtes. Une passerelle qui authentifie `/v1/models` doit accepter `x-api-key` pour les déploiements d'aide. Tous les en-têtes de `ANTHROPIC_CUSTOM_HEADERS` sont également inclus.

Claude Code lit `id` et le `display_name` optionnel de chaque entrée dans le tableau `data` de la réponse, et ignore les entrées dont `id` ne commence pas par `claude` ou `anthropic` :

```json theme={null}
{
  "data": [
    { "id": "claude-sonnet-4-6", "display_name": "Claude Sonnet 4.6" },
    { "id": "claude-opus-4-8" }
  ]
}
```

<h3 id="picker-entries-and-caching">
  Entrées du sélecteur et mise en cache
</h3>

Le sélecteur est la liste de modèles interactive qui s'ouvre lorsqu'un développeur exécute `/model` dans Claude Code. Chaque entrée découverte est étiquetée « Depuis la passerelle » et utilise `display_name` lorsqu'il est fourni. Le [paramètre géré `availableModels`](/docs/fr/settings#available-settings) limite ce que la découverte peut ajouter.

Un ID découvert est ignoré lorsqu'il correspond exactement à une ligne déjà dans le sélecteur, ou lorsque les ID découvert et existant se résolvent tous deux en [Fable](/docs/fr/model-config#work-with-fable-5). À partir de Claude Code v2.1.197, un ID explicite découvert est également fusionné dans une entrée intégrée lorsque les deux se résolvent au même modèle. Les lignes intégrées sont basées sur des alias tels que `sonnet`, donc un ID découvert explicite du modèle auquel l'alias se résout actuellement, tel que `claude-sonnet-5`, s'effondre dans la ligne `sonnet`, tandis qu'un ID auquel l'alias ne se résout pas, tel que `claude-sonnet-4-6`, ajoute toujours sa propre ligne « Depuis la passerelle » à côté de l'entrée intégrée.

Les résultats sont mis en cache dans `~/.claude/cache/gateway-models.json`, ou `%USERPROFILE%\.claude\cache\gateway-models.json` sur Windows, et actualisés à chaque démarrage. Si la requête échoue ou la passerelle n'implémente pas `/v1/models`, le sélecteur revient à la liste mise en cache du démarrage précédent ou à la liste de modèles intégrée. Si votre passerelle sert les modèles Claude sous des alias qui ne correspondent pas au filtre de découverte, les développeurs peuvent ajouter ces alias manuellement avec les [variables de configuration du modèle](/docs/fr/model-config).

<h2 id="related-resources">
  Ressources connexes
</h2>

Pour le reste de l'ensemble de documentation de passerelle et les références API sous-jacentes :

* [Aperçu des passerelles](/docs/fr/gateways) : ce qu'est une passerelle et comment choisir entre la passerelle Claude apps et un autre produit
* [Autres passerelles LLM](/docs/fr/llm-gateway) : comment déployer une passerelle que votre organisation exécute et comment elle interagit avec les abonnements claude.ai
* [Déployer une passerelle LLM pour votre organisation](/docs/fr/llm-gateway-rollout) : la liste de contrôle d'administration qui utilise ce contrat
* [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect) : configuration par développeur et le tableau de dépannage
* [Référence des en-têtes bêta](https://platform.claude.com/docs/en/api/beta-headers) : l'ensemble actuel des valeurs `anthropic-beta`
* [API Messages](https://platform.claude.com/docs/en/api/messages) : le format API qu'une passerelle au format Anthropic implémente
