> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guide de compatibilité de la passerelle Claude Code

> Maintenez une passerelle LLM compatible avec Claude Code : les points de terminaison qu'elle appelle, les en-têtes et champs de corps à transmettre, et ce qui se casse quand ils sont supprimés.

Cette page documente les requêtes que Claude Code envoie à une passerelle, y compris les points de terminaison qu'elle appelle, les en-têtes et champs de corps que la passerelle doit transmettre, et quelles fonctionnalités cessent de fonctionner si elle ne le fait pas. Elle est écrite pour les opérateurs configurant un produit de passerelle pour fonctionner avec Claude Code.

La [passerelle des applications Claude](/docs/fr/claude-apps-gateway), la passerelle auto-hébergée d'Anthropic, sert sa propre référence de point de terminaison à `GET /protocol`, couvrant les points de terminaison de connexion, d'inférence, de paramètres gérés, de découverte de modèles et de télémétrie de cette passerelle. C'est un document séparé de ce guide.

<Note>
  * Pour déployer une passerelle existante ou tierce pour votre organisation, consultez [Déployer une passerelle LLM](/docs/fr/llm-gateway-rollout)
  * Si vous êtes un développeur individuel authentifiant Claude Code à une passerelle avec une credential qui vous a été donnée, consultez [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect)
</Note>

Cette page couvre :

* [Formats d'API](#api-formats) et les points de terminaison à servir pour chacun
* [Comportement du client par méthode de connexion](#how-the-connection-method-changes-client-behavior) : comment les ID de modèle, les valeurs `anthropic-beta`, les champs de requête et les valeurs par défaut diffèrent entre les formats et une connexion de passerelle des applications Claude
* [En-têtes de requête](#request-headers) : lesquels doivent atteindre l'amont et lesquels votre passerelle peut consommer
* [En-têtes de réponse](#response-headers) : ce qu'il faut retourner pour que la détection de blocage, les tentatives et l'affichage des limites d'utilisation fonctionnent
* Le [bloc d'attribution du message système](#system-prompt-attribution-block) et comment il interagit avec la mise en cache des invites
* [Passage des fonctionnalités](#feature-pass-through) : ce qui se casse quand les en-têtes ou les champs de corps sont supprimés
* [Découverte de modèles](#model-discovery)

Cette page utilise deux termes pour ce que votre passerelle fait avec chaque en-tête et champ de corps :

* **Transmettre inchangé** : le passer à l'amont octet par octet
* **Consommer** : la passerelle peut le lire pour le routage, l'attribution ou le traçage et n'a pas besoin de le transmettre

Tout ce qui n'est pas marqué comme transmettre inchangé est vôtre à consommer ou ignorer.

<h2 id="api-formats">
  Formats d'API
</h2>

Une passerelle doit exposer au moins l'un des formats d'API suivants aux clients Claude Code. Un client choisit un format et pointe Claude Code vers votre passerelle avec les variables dans la colonne Sélectionné par du tableau ci-dessous.

Google Cloud's Agent Platform est le point de terminaison Claude de Google Cloud, anciennement Vertex AI ; ses noms de variables conservent l'orthographe `VERTEX`.

| Format                                   | Sélectionné par                                               | Points de terminaison                                                                                            | Transférer inchangé                                                                                              |
| :--------------------------------------- | :------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                          | `/v1/messages`, `/v1/messages/count_tokens` (optionnel)                                                          | En-têtes de requête `anthropic-beta` et `anthropic-version`                                                      |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` avec `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`, `/model/{model}/count-tokens` (optionnel) | Champs du corps de requête `anthropic_beta` et `anthropic_version`                                               |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` avec `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (optionnel)                                        | En-têtes de requête `anthropic-beta` et `anthropic-version`, et le champ du corps de requête `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry et Claude Platform on AWS
</h3>

Microsoft Foundry et la [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) implémentent le format Anthropic Messages. Claude Code les route via leurs propres variables, `ANTHROPIC_FOUNDRY_BASE_URL` et `ANTHROPIC_AWS_BASE_URL`, mais une passerelle les frontalisant implémente la ligne Anthropic Messages ci-dessus. Une passerelle frontalisant Claude Platform on AWS doit également transférer l'en-tête `anthropic-workspace-id`, que [cette plateforme exige sur chaque requête](/docs/fr/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Points de terminaison optionnels et trafic de démarrage
</h3>

Les points de terminaison de comptage de jetons sont les seuls optionnels : en leur absence, Claude Code revient à une estimation basée sur les caractères de l'utilisation du contexte.

Faites correspondre le chemin, pas l'URL complète :

* Les requêtes d'inférence sont envoyées à `/v1/messages?beta=true`
* La méthode Google Cloud's Agent Platform ajoute des suffixes au chemin du modèle de l'éditeur, comme dans `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`

Une passerelle voit également du trafic de démarrage au meilleur effort qu'elle peut rejeter sans rien casser. Une passerelle au format Anthropic Messages reçoit une sonde de préchauffage de connexion `HEAD /api/hello`, que Claude Code ignore quand un proxy HTTP ou un certificat client est configuré. Une passerelle au format Amazon Bedrock reçoit une requête `GET /inference-profiles?type=SYSTEM_DEFINED` et, quand le modèle configuré est un profil d'inférence, des recherches `GET /inference-profiles/{profile}`.

La vérification de disponibilité du [mode rapide](/docs/fr/fast-mode) n'apparaît jamais dans les journaux de passerelle : elle appelle `api.anthropic.com` directement plutôt que de suivre `ANTHROPIC_BASE_URL`, donc sur un réseau qui bloque la sortie directe vers `api.anthropic.com`, le mode rapide peut signaler une erreur de connectivité tandis que l'inférence via la passerelle continue de fonctionner. La [vérification de sécurité du domaine WebFetch](/docs/fr/data-usage#webfetch-domain-safety-check) appelle également `api.anthropic.com` directement. [Utiliser le mode rapide derrière les proxies et les passerelles LLM](/docs/fr/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) couvre les variables qui le restaurent.

<h3 id="streaming">
  Streaming
</h3>

Diffusez les réponses d'inférence en continu. Claude Code lit le flux au fur et à mesure de son arrivée, donc si votre passerelle met en mémoire tampon les réponses complètes avant de les relayer, Claude Code s'arrête.

Quand le client parle le format Amazon Bedrock, relayez le corps de réponse `InvokeModelWithResponseStream` et son en-tête `Content-Type: application/vnd.amazon.eventstream` sans modification, et ne convertissez pas le flux en événements envoyés par le serveur. Voir [Erreurs de streaming derrière une passerelle ou un proxy](/docs/fr/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

Transférez également les pings de maintien de connexion. Sur les connexions via `ANTHROPIC_BASE_URL` ou `ANTHROPIC_AWS_BASE_URL`, Claude Code compte chaque octet que votre passerelle relaye, y compris les événements SSE `ping` et les lignes de commentaire, et abandonne un flux qui reste silencieux pendant 300 secondes par défaut. Les pings en amont sont le seul trafic pendant les pauses de réflexion prolongées, donc si votre passerelle les supprime ou les met en mémoire tampon, Claude Code abandonne le flux pendant ces pauses ; [Tentatives automatiques](/docs/fr/errors#automatic-retries) couvre ce qu'un flux abandonné signale en fonction de la progression de la réponse. Un amont qui n'envoie aucun ping du tout, comme le flux d'événements binaires d'Amazon Bedrock, laisse ces pauses sans rien à relayer. Lors de la traduction à partir d'un tel amont, émettez vos propres événements `ping` pendant les silences. Les passerelles atteintes via `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, ou `ANTHROPIC_FOUNDRY_BASE_URL` ne sont pas enveloppées par ce chien de garde au niveau des octets, même quand elles relaient le format Anthropic Messages ; là, un [délai d'inactivité de 5 minutes](/docs/fr/env-vars) abandonne un flux silencieux à la place, et sur les connexions `ANTHROPIC_BEDROCK_BASE_URL` vous pouvez ajouter le chien de garde au niveau des octets avec [`CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK`](/docs/fr/env-vars).

<h3 id="format-mismatch-with-the-upstream">
  Incompatibilité de format avec l'amont
</h3>

Le format que le client parle détermine ce que votre passerelle reçoit. Le mode de défaillance courant est une incompatibilité entre le format que le client envoie à votre passerelle et le format que le fournisseur amont derrière elle accepte.

* Quand le client parle le format Amazon Bedrock ou Google Cloud's Agent Platform, Claude Code envoie uniquement le sous-ensemble de son ensemble complet de capacités que ces fournisseurs acceptent
* Quand le client parle le format Anthropic Messages, Claude Code envoie l'ensemble complet, même si votre passerelle relaye vers un amont Amazon Bedrock ou Google Cloud's Agent Platform

Combler cette différence est le travail de votre passerelle. [Passage des fonctionnalités](#feature-pass-through) décrit ce qui se casse quand ce n'est pas le cas.

Si votre amont est Amazon Bedrock ou Google Cloud's Agent Platform, vous pouvez éviter le pontage en exposant le format de ce fournisseur à la place. [Router vers un fournisseur cloud via une passerelle](/docs/fr/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) montre la configuration du client pour ce format.

<h2 id="how-the-connection-method-changes-client-behavior">
  Comment la méthode de connexion modifie le comportement du client
</h2>

La façon dont un développeur se connecte à votre passerelle détermine les ID de modèle, les valeurs `anthropic-beta` et les champs de requête que Claude Code envoie, ainsi que les valeurs par défaut qu'il applique. Votre passerelle voit l'un des trois comportements clients suivants :

* **Format Amazon Bedrock ou Agent Platform** : le développeur définit `CLAUDE_CODE_USE_BEDROCK=1` avec `ANTHROPIC_BEDROCK_BASE_URL`, ou `CLAUDE_CODE_USE_VERTEX=1` avec `ANTHROPIC_VERTEX_BASE_URL`, pointant vers votre passerelle. Claude Code utilise les ID de modèle, les champs de requête et les valeurs par défaut de ce fournisseur.
* **Format Anthropic Messages** : le développeur définit `ANTHROPIC_BASE_URL` sur votre passerelle. Claude Code traite la passerelle comme l'API Claude et ne peut pas déterminer vers quel upstream vous transférez.
* **Connexion à la passerelle des applications Claude** : le développeur se connecte à une [passerelle des applications Claude](/docs/fr/claude-apps-gateway). Cette passerelle utilise le format Anthropic Messages mais peut router vers n'importe quel upstream, donc Claude Code envoie uniquement les valeurs `anthropic-beta` et les hypothèses de capacités de modèle qu'Amazon Bedrock et Agent Platform acceptent également.

<h3 id="requests-and-defaults-by-connection-method">
  Requêtes et valeurs par défaut selon la méthode de connexion
</h3>

Le tableau ci-dessous compare les trois méthodes de connexion, un comportement par ligne. Il omet Microsoft Foundry et Claude Platform sur AWS, qui utilisent également le format Anthropic Messages mais que Claude Code atteint via leurs propres variables. Pour ceux-ci, consultez les pages [Microsoft Foundry](/docs/fr/microsoft-foundry) et [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws).

| Comportement                                                                                                                     | Format Amazon Bedrock ou Agent Platform                                                                                                                                                                                                   | Format Anthropic Messages                                                                                                                                                                                | Connexion à la passerelle des applications Claude                                                                                                  |
| :------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| ID de modèle dans les requêtes par défaut                                                                                        | Le formulaire du fournisseur, tel que `us.anthropic.claude-opus-4-8` sur Amazon Bedrock                                                                                                                                                   | ID Anthropic, tels que `claude-opus-4-8`                                                                                                                                                                 | ID Anthropic                                                                                                                                       |
| Valeurs `anthropic-beta` envoyées                                                                                                | Le sous-ensemble qu'Amazon Bedrock et Agent Platform acceptent                                                                                                                                                                            | L'ensemble complet décrit sous [transmission des fonctionnalités](#feature-pass-through), sauf si le développeur définit [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities)   | Le sous-ensemble qu'Amazon Bedrock et Agent Platform acceptent                                                                                     |
| Champs de requête pour un ID de modèle que Claude Code ne reconnaît pas, tel qu'un alias de passerelle                           | Réflexion avec un budget fixe plutôt que raisonnement adaptatif, et aucun champ de gestion d'effort ou de contexte                                                                                                                        | Tout ce que les modèles Claude actuels acceptent sur l'API Claude, y compris le raisonnement adaptatif, l'effort et la gestion du contexte, qu'un upstream Amazon Bedrock ou Agent Platform peut rejeter | Identique au format Amazon Bedrock ou Agent Platform                                                                                               |
| [TTL du cache de prompt](/docs/fr/prompt-caching#choose-the-ttl-yourself) d'une heure lorsqu'un développeur opte pour cette option    | Demandé via le champ `ttl` dans `cache_control`, sans valeur bêta                                                                                                                                                                         | Demandé via le champ `ttl` plus une valeur `extended-cache-ttl` dans `anthropic-beta`, que vous devez transférer                                                                                         | Consultez le tableau [disponibilité et limitations](/docs/fr/claude-apps-gateway#availability-and-limitations) de la passerelle des applications Claude |
| Modèle pour les [tâches en arrière-plan](/docs/fr/costs#background-token-usage) sauf si `ANTHROPIC_DEFAULT_HAIKU_MODEL` en épingle un | Le modèle Sonnet par défaut, ou le modèle principal une fois qu'un est sélectionné, comme le décrivent les pages [Amazon Bedrock](/docs/fr/amazon-bedrock#4-pin-model-versions) et [Agent Platform](/docs/fr/google-vertex-ai#5-pin-model-versions) | Le modèle principal, ou le modèle Haiku par défaut lorsqu'une clé Anthropic Console est fournie par `ANTHROPIC_API_KEY` ou `apiKeyHelper` et que `ANTHROPIC_AUTH_TOKEN` n'est pas défini                 | Le modèle principal                                                                                                                                |

Pour les fonctionnalités que chaque connexion supporte et la télémétrie qu'elle envoie à Anthropic par défaut, consultez [Disponibilité des fonctionnalités](/docs/fr/feature-availability#availability-by-model-provider) et [Comportements par défaut selon le fournisseur d'API](/docs/fr/data-usage#default-behaviors-by-api-provider).

<h3 id="settings-for-unrecognized-model-ids">
  Paramètres pour les ID de modèle non reconnus
</h3>

Deux paramètres côté client modifient ce que Claude Code suppose pour un ID de modèle qu'il ne reconnaît pas, quelle que soit la méthode de connexion utilisée par le développeur :

* **Fenêtre de contexte** : Claude Code suppose 200K, ou 1M lorsque l'ID porte `[1m]`. Pour déclarer la vraie fenêtre, consultez [Corriger la fenêtre pour une passerelle ou un ID de modèle personnalisé](/docs/fr/model-config#correct-the-window-for-a-gateway-or-custom-model-id)
* **Capacités** : pour donner à un alias de passerelle les capacités du modèle derrière lui, mappez l'ID Anthropic de ce modèle à votre alias avec une entrée [`modelOverrides`](/docs/fr/errors#unrecognized-model-id-on-a-request) dans les paramètres que vous distribuez. Pour savoir où les variables `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` s'appliquent, consultez [transmission des fonctionnalités](#feature-pass-through)

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

<h3 id="gateway-hint-headers">
  En-têtes d'indication de passerelle
</h3>

Claude Code peut également envoyer des indications de routage : des faits par requête qu'une passerelle ou un routeur peut utiliser pour planifier, mettre en cache ou attribuer une requête. Nécessite Claude Code v2.1.273 ou ultérieur.

Le fait qu'une requête les porte dépend de l'endroit où Claude Code l'envoie :

* Connexion directe à l'API Anthropic : envoyée par défaut
* URL de base personnalisée : désactivée par défaut, car un proxy qui rejette les en-têtes inconnus échouerait la requête. Pour les recevoir, définissez [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](/docs/fr/env-vars) pour vos développeurs, par exemple dans le bloc `env` des [paramètres gérés](/docs/fr/managed-settings)
* Tout autre backend, y compris Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry et Claude Platform on AWS : envoyée uniquement lorsque `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1` est défini

Définir `CLAUDE_CODE_GATEWAY_HINT_HEADERS` à `0` arrête les en-têtes sur chaque connexion.

Les en-têtes ne portent que ce que les lignes ci-dessous énumèrent : vocabulaires fixes, noms d'outils et durées, jamais de texte d'invite ou de contenu de fichier. Chaque valeur est ASCII imprimable.

| En-tête                             | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `x-claude-code-request-class`       | Quel type de requête c'est : `main` pour un tour de la conversation principale, `subagent` pour un tour d'un [sous-agent](/docs/fr/sub-agents), `workflow` pour un agent s'exécutant dans un workflow, `compaction` pour la requête de résumé qui compacte une conversation, ou `auxiliary` pour les requêtes latérales telles que les titres de session, les classificateurs et les résumés. Envoyé sur chaque requête                                                                                                                                                                                      |
| `x-claude-code-agent-type`          | Le type de sous-agent qui a émis la requête : un nom de type d'agent intégré tel que `Explore`, `Plan` ou `general-purpose`, ou `custom` pour un agent défini par l'utilisateur, `teammate` pour un membre d'une [équipe d'agents](/docs/fr/agent-teams) s'exécutant dans le processus du leader, ou `fork` pour un [fork](/docs/fr/sub-agents#fork-the-current-conversation). Présent uniquement sur les propres tours d'un sous-agent ; la compaction ou les requêtes latérales d'un sous-agent conservent l'ID d'agent mais ne portent aucun type. Un nom d'agent choisi par l'utilisateur n'est jamais envoyé |
| `x-claude-code-compaction`          | Présent sur la requête qui résume la conversation lors d'une [compaction](/docs/fr/prompt-caching#compacting-the-conversation). La valeur indique ce qui l'a déclenchée : `auto` lorsque la fenêtre de contexte approchait de la capacité, `manual` pour `/compact`, ou `reactive` lorsque l'API a rejeté une requête comme trop longue. Absent sur chaque autre requête                                                                                                                                                                                                                                     |
| `x-claude-code-context-compacted`   | Présent une fois, sur la première requête de conversation principale après une compaction, avec les mêmes valeurs que `x-claude-code-compaction`. Le préfixe de conversation avant cette requête n'est plus utilisé, donc un cache basé sur celui-ci peut être supprimé                                                                                                                                                                                                                                                                                                                                 |
| `x-claude-code-prev-tool-durations` | Temps d'exécution mesuré des appels d'outils dont cette requête porte les résultats, comme `<name>=<ms>;<name>=<ms>`, par exemple `Bash=742;Read=9`. Envoyé sur la requête suivante de la même conversation après un lot d'appels d'outils, depuis la session principale ou un sous-agent                                                                                                                                                                                                                                                                                                               |

Avant d'analyser `x-claude-code-prev-tool-durations`, vérifiez comment Claude Code construit la valeur et ce qu'il omet :

* Entrées : une par appel d'outil qui s'est exécuté, dans l'ordre où son résultat a été collecté, en millisecondes entières
* Limite : Claude Code envoie au maximum 32 entrées et 4 KB, en conservant les premières entrées
* Codage : les noms d'outils sont codés en pourcentage, couvrant `%`, `;`, `=`, virgule, espace et tout caractère en dehors d'ASCII imprimable
* Analyse : divisez sur `;`, puis sur `=`, et décodez chaque nom
* Absence : les appels de compaction, les requêtes latérales et la première requête d'une nouvelle invite ne la portent jamais. Ne lisez pas un en-tête manquant comme un tour qui n'a exécuté aucun outil
* Temps : chacun exclut les invites de permission et les hooks, et les appels d'outils parallèles rapportent chacun leur propre temps, donc les entrées ne s'ajoutent pas à l'écart entre les requêtes

<h3 id="forward-as-open-lists">
  Transmettre comme listes ouvertes
</h3>

Traitez les en-têtes et champs de corps comme des listes ouvertes, pas fermées. Claude Code gagne des capacités au fil des versions, et elles arrivent comme de nouvelles valeurs `anthropic-beta`, de nouveaux champs de corps de requête, et occasionnellement de nouveaux en-têtes `anthropic-*` ou `x-claude-code-*`.

Lors de la transmission à un amont au format Anthropic, transmettez les en-têtes de requête `anthropic-*` et les champs de corps de requête inchangés plutôt que de mettre en liste blanche ceux que vous voyez aujourd'hui. Une passerelle épinglée à une liste observée supprime l'en-tête ou le champ de la capacité suivante et la casse à la version qui l'introduit.

L'exception est un amont non-Anthropic tel qu'Amazon Bedrock ou Agent Platform de Google Cloud, où combler la différence de schéma est le travail de la passerelle ; consultez [transmission des fonctionnalités](#feature-pass-through).

<h2 id="response-headers">
  En-têtes de réponse
</h2>

Claude Code lit ces en-têtes de réponse pour détecter les flux bloqués, pour décider s'il faut relancer et quand, et pour afficher les limites d'utilisation. Le tableau liste ce qu'il faut retourner pour chacun. Transmettez également les corps de réponse d'erreur sans modification, afin que la [récupération de rejet de capacité](#automatic-retry-and-error-forwarding) de Claude Code puisse correspondre à la formulation d'erreur en amont.

| En-tête                         | Ce qu'il faut retourner et pourquoi                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content-type`                  | Retournez `text/event-stream` sur les réponses au format Messages Anthropic en flux continu, et `application/vnd.amazon.eventstream`, sans modification, sur les réponses au format Amazon Bedrock, où [un type différent échoue la demande](/docs/fr/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy). [Streaming](#streaming) liste les connexions qui exécutent la détection de stagnation sur ces flux |
| `retry-after`                   | Retournez des secondes entières plutôt qu'une date HTTP. Claude Code attend au moins ce laps de temps avant la prochaine [relance automatique](/docs/fr/errors#automatic-retries), et en dehors des sessions [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/fr/env-vars) une valeur supérieure à 60 arrête les relances et affiche l'erreur immédiatement                                                                           |
| `x-should-retry`                | Transmettez la valeur en amont sans modification. Claude Code lit cet en-tête comme une entrée lors de la décision de relancer une demande échouée : `true` marque la réponse comme relançable et `false` la marque comme non relançable. Pour les compteurs de relance, le backoff, et les défaillances que Claude Code relance, voir [relances automatiques](/docs/fr/errors#automatic-retries)                    |
| `anthropic-ratelimit-unified-*` | Transmettez les valeurs en amont sans modification sur chaque réponse. Claude Code les lit sur les réponses réussies pour afficher l'utilisation par rapport aux limites du plan aux développeurs connectés avec claude.ai, et sur un `429` pour distinguer une limite de plan ou un plafond de dépenses d'une limitation temporaire ; voir [limites d'utilisation](/docs/fr/errors#usage-limits)                    |

<h2 id="system-prompt-attribution-block">
  Bloc d'attribution du message système
</h2>

Claude Code ajoute un bloc d'attribution court au message système contenant la version du client et une empreinte dérivée de la conversation. Le point de terminaison `api.anthropic.com` supprime le bloc avant le traitement lorsqu'il arrive inchangé comme premier bloc système, donc il n'affecte pas la mise en cache des invites de première partie. Tout autre amont le reçoit comme faisant partie de l'invite.

La suppression est positionnelle, donc elle ne fonctionne que lorsque la passerelle transfère le tableau `system` inchangé. Pour garder le bloc hors de l'invite sans perdre d'autre contenu système :

* Transférez le tableau `system` exactement tel que reçu, en gardant le bloc en premier : ajouter un autre bloc système, réorganiser le tableau ou le convertir en une seule chaîne annule la suppression, et le bloc atteint alors le modèle et la clé du cache d'invite.
* Gardez le bloc dans sa propre entrée de tableau : le point de terminaison traite un bloc fusionné qui commence par l'en-tête d'attribution comme une attribution dans son intégralité et supprime tout ce qui y est fusionné, y compris le reste du message système.
* Si votre passerelle doit remodeler le contenu système, définissez [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/fr/env-vars) pour que Claude Code omette le bloc. Anthropic et les points de terminaison Claude des fournisseurs de cloud lisent le bloc pour l'attribution, donc omettez-le au niveau du client plutôt que de le supprimer ou de le déplacer dans la passerelle.

La variable existe pour la compatibilité des passerelles et de la mise en cache tiers, et non comme contrôle de confidentialité : sur une connexion directe, la requête complète va à l'API Anthropic de toute façon. Lorsque ces deux conditions sont remplies, Claude Code conserve le bloc sur les requêtes du classificateur en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) même lorsque vous définissez la variable sur `0` :

* Les requêtes vont à `api.anthropic.com`, avec `ANTHROPIC_BASE_URL` non défini ou nommant cet hôte et aucun fournisseur tiers sélectionné.
* Les identifiants actifs ne sont pas une [identité de profil Anthropic ou une identité de fédération](/docs/fr/authentication#anthropic-profiles-and-federation-credentials).

Les requêtes du classificateur ignorent le reste du message système de Claude Code, donc sur ces requêtes le bloc est le seul marqueur dans le corps de la requête qui les identifie comme du trafic Claude Code. Lorsque l'une des conditions échoue, via une passerelle LLM, sur un fournisseur tiers, ou avec une identité de profil ou de fédération active, la définition de `0` supprime également le bloc des requêtes du classificateur. Avant v2.1.229, cette exception n'existait pas : la définition de `0` supprimait le bloc de ces requêtes du classificateur, et lorsque l'API refusait les requêtes non identifiées, le mode auto échouait sur chaque action qu'il envoyait au classificateur.

À partir de Claude Code v2.1.181, le bloc est stable pour la durée de vie d'une conversation lorsque les requêtes sont routées via une URL de base personnalisée, donc un cache d'invite côté passerelle basé sur le corps de requête complet fonctionne sans le désactiver, et tout fournisseur vers lequel votre passerelle transfère les requêtes reçoit un préfixe d'invite stable. Avant v2.1.181, le bloc incluait un jeton par requête qui changeait le début du message système à chaque requête. Sur ces versions, définissez `CLAUDE_CODE_ATTRIBUTION_HEADER=0` lorsque votre passerelle fait l'une de ces choses :

* Implémente un cache d'invite basé sur le corps de la requête.
* Transfère les requêtes à un fournisseur tiers tel qu'Amazon Bedrock, Microsoft Foundry ou la plateforme Agent de Google Cloud, au format Messages Anthropic ou au format propre du fournisseur, où le préfixe changeant réduit la réutilisation du cache d'invite sur ce fournisseur.

<h2 id="feature-pass-through">
  Transmission des fonctionnalités
</h2>

Claude Code traite une passerelle `ANTHROPIC_BASE_URL` comme un point de terminaison au format Anthropic et lui envoie les en-têtes bêta et champs de corps de requête qu'il envoie à `api.anthropic.com`, sauf un petit ensemble de diagnostics et de valeurs par défaut réservés aux connexions directes, comme la valeur par défaut de streaming d'outils à grain fin couverte ci-dessous. Cet ensemble varie selon la version, donc ne dépendez pas de son contenu.

Les capacités qui ajoutent des champs de corps les associent à un en-tête bêta, et la paire voyage ensemble. Une passerelle qui supprime l'en-tête tout en transmettant le corps, ou transmet un corps au format Anthropic à un amont avec un schéma différent, produit des erreurs `400` dures ; seulement lorsque les deux moitiés sont absentes ensemble la fonctionnalité s'éteint silencieusement. Une passerelle qui réécrit ou rédige les corps de requête pour l'inspection du contenu casse l'appairage de la même manière que la suppression, donc inspectez sans modifier. Le tableau note où une fonctionnalité s'écarte de l'appairage.

Le streaming d'outils à grain fin est l'une des valeurs par défaut de connexion directe : il est désactivé par défaut chaque fois que les requêtes sont routées via une URL de base personnalisée, et une passerelle le reçoit lorsque les développeurs définissent [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/fr/env-vars).

| Fonctionnalité                                                                                                                                                                                                                              | Paire en-tête et corps                                                                                                                                                                                                                                    | Symptôme lorsque cassé                                                                                                                                                        | Correction                                                                                                                                                     |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Raisonnement adaptatif](/docs/fr/model-config#adjust-effort-level)                                                                                                                                                                              | Pas d'en-tête bêta. Claude Code envoie `thinking: {"type": "adaptive"}` pour Claude 4.6 et versions ultérieures, et traite les noms de modèles qu'il ne reconnaît pas, tels que les alias de passerelle, comme des modèles actuels qui reçoivent le champ | `400` nommant le champ `thinking` ou la balise `adaptive` lorsque la version du modèle amont ne l'accepte pas                                                                 | Mettez à niveau l'amont. Sur Opus 4.6 et Sonnet 4.6, les développeurs peuvent définir `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` à la place                     |
| [Gestion du contexte](https://platform.claude.com/docs/en/build-with-claude/context-editing)                                                                                                                                                | L'en-tête bêta de gestion du contexte s'associe au champ de corps `context_management`                                                                                                                                                                    | `400` avec `Extra inputs are not permitted`. Courant lorsqu'une passerelle accepte les requêtes au format Anthropic mais les transmet à Amazon Bedrock                        | Transmettez les deux, ou [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/fr/env-vars)                                                                            |
| [Contexte étendu](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) et [pensée entrelacée](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | En-têtes bêta uniquement, pas de champ de corps                                                                                                                                                                                                           | Silencieusement indisponible lorsque l'en-tête est supprimé ; l'amont ne voit jamais la demande de capacité                                                                   | Transmettez `anthropic-beta` verbatim                                                                                                                          |
| Champs d'outil [bêta](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                               | Les en-têtes bêta liés aux outils s'associent aux champs de schéma d'outil tels que `strict` et `defer_loading`                                                                                                                                           | `400` nommant le champ de schéma d'outil non reconnu lorsque le corps passe sans son en-tête                                                                                  | Transmettez les deux, ou [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)                                                       |
| [Effort](https://platform.claude.com/docs/en/build-with-claude/effort) et [sorties structurées](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                   | Le champ de corps `output_config` porte les paramètres d'effort, de format de sortie structurée et de budget de tâche ; chacun s'associe à son propre en-tête bêta                                                                                        | `400` nommant `output_config`, souvent `Extra inputs are not permitted`, sur les amonts Amazon Bedrock et Agent Platform de Google Cloud                                      | Transmettez le champ et ses en-têtes ensemble                                                                                                                  |
| [Mise en cache des invites](/docs/fr/prompt-caching)                                                                                                                                                                                             | Pas d'appairage bêta. Claude Code attache les marqueurs `cache_control` aux blocs `system` et aux entrées `messages`, y compris les entrées `role: "system"` ajoutées en milieu de conversation                                                           | Pas d'erreur : la conversation est facturée comme entrée non mise en cache à chaque tour, visible comme `input_tokens` élevé avec peu ou pas d'activité de cache dans `usage` | Transmettez `cache_control` inchangé partout où il apparaît, et ne convertissez pas le formulaire de bloc `system` ou le contenu du message en chaînes simples |
| [Comptage des jetons](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                 | Pas d'appairage bêta ; utilise le point de terminaison `count_tokens`                                                                                                                                                                                     | Pas d'erreur : Claude Code revient à une estimation basée sur les caractères, donc `/context` affiche les comptages approximatifs                                             | Exposez le point de terminaison pour les comptages de jetons exacts                                                                                            |

Les [variables](/docs/fr/model-config) `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` déclarent les capacités du modèle uniquement dans les configurations du fournisseur : `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, et [`CLAUDE_CODE_USE_MANTLE`](/docs/fr/amazon-bedrock#use-the-mantle-endpoint). Elles n'ont aucun effet derrière une passerelle `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Nouvelle tentative automatique et transmission d'erreur
</h3>

Ce que Claude Code fait après un rejet en amont dépend de ce qui a été rejeté :

* Lorsque l'amont rejette le champ `thinking`, un message système en milieu de conversation, ou le marqueur `cache_control` sur un tel message, Claude Code réessaie la requête et désactive la capacité rejetée pour le reste de la conversation
* Lorsque l'amont rejette une [signature de pensée](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), y compris avec un `400` dont le message dit que le bloc est `bound to a different conversation`, Claude Code supprime les blocs de pensée antérieurs de la requête, réessaie, et les garde hors de chaque requête ultérieure. Les nouvelles réponses incluent toujours la pensée
* Lorsque la passerelle ou son amont rejette l'entrée de l'[outil conseiller](/docs/fr/advisor) dans `tools` comme un type d'outil non reconnu, Claude Code réessaie la requête une fois sans cette entrée et sa valeur `anthropic-beta`. Les requêtes ultérieures à cette URL de base laissent le conseiller de côté jusqu'à ce que Claude Code se termine, et `/advisor` n'est pas disponible pour le développeur pendant ce temps. Claude Code reconnaît ce rejet par une réponse `400` ou `422` dont le message nomme le type d'outil après `Input tag`, comme `Input tag 'advisor_20260301'`. Avant v2.1.280, Claude Code ne réessayait pas ce rejet
* Claude Code ne réessaie pas les rejets de gestion du contexte ou de champ de schéma d'outil, donc ces erreurs `400` atteignent le développeur

Le rejet `bound to a different conversation` provient de la vérification de [pensée préservée](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking) de l'API, qui échoue lorsque le contenu `system`, `tools`, ou `messages` antérieurs diffère de la requête qui a produit la pensée. Une passerelle qui réécrit l'un de ces contenus peut causer le rejet lui-même ; [Bibliothèques, proxies et passerelles](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways) couvre ce qu'il faut transmettre inchangé.

La logique de nouvelle tentative correspond à la formulation d'erreur de l'amont, donc transmettez les corps de réponse d'erreur inmodifiés. Une passerelle qui enveloppe les erreurs en amont dans sa propre enveloppe casse le chemin de récupération, même lorsqu'elle préserve le code d'état, sauf si le message de l'enveloppe porte un jeton `capability_rejected:` stable. [La passerelle des applications Claude substitue ces jetons à la formulation d'erreur des fournisseurs de cloud](/docs/fr/claude-apps-gateway-config#upstream-error-messages), par exemple `capability_rejected: prompt_too_long`.

<h3 id="disable-pre-release-capabilities">
  Désactiver les capacités de pré-version
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` empêche Claude Code d'envoyer les capacités de pré-version et leurs champs de corps sur chaque fournisseur, y compris la gestion du contexte et les champs d'outil bêta. La variable n'affecte pas le raisonnement adaptatif, qui est sélectionné par modèle plutôt que par bêta. Elle ne supprime jamais la capacité OAuth que l'authentification par abonnement exige.

Sur Claude Code v2.1.227 ou ultérieur, votre organisation peut maintenir la [recherche d'outils MCP](/docs/fr/mcp#scale-with-mcp-tool-search) activée sous cette variable via les [paramètres gérés](/docs/fr/managed-settings). Ce que Claude Code envoie avec ce remplacement en place dépend de la façon dont vous vous connectez :

* Sur une connexion directe, ou via une passerelle définie avec `ANTHROPIC_BASE_URL`, Claude Code continue d'envoyer l'en-tête bêta de recherche d'outils, les champs d'outil `defer_loading`, et les blocs `tool_reference`, et supprime le reste
* Sur un fournisseur de cloud, ou connecté via une [passerelle des applications Claude](/docs/fr/claude-apps-gateway), le remplacement n'a aucun effet

L'ensemble des capacités que Claude Code envoie augmente au fil des versions. Pour les chaînes d'en-tête bêta actuelles, consultez la [référence des en-têtes bêta](https://platform.claude.com/docs/en/api/beta-headers) ; testez votre passerelle contre les nouvelles versions de Claude Code plutôt que de vous épingler à une liste observée.

<h2 id="model-discovery">
  Découverte des modèles
</h2>

Lorsque `ANTHROPIC_BASE_URL` pointe vers une passerelle qui expose le format Messages Anthropic, Claude Code peut interroger le point de terminaison `/v1/models` de la passerelle au démarrage et ajouter les modèles retournés au sélecteur `/model`. Si vous ou votre administrateur définissez `replaceBuiltInOptions` dans une configuration [`modelPicker`](/docs/fr/settings-reference#modelpicker), Claude Code masque les modèles découverts du sélecteur.

Les développeurs l'activent en définissant [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/fr/env-vars), dans leur propre environnement ou via les paramètres gérés. La découverte est désactivée par défaut afin que les passerelles soutenues par une clé API partagée ne surface pas chaque modèle auquel la clé peut accéder à chaque utilisateur.

<h3 id="when-discovery-runs">
  Quand la découverte s'exécute
</h3>

La découverte s'applique uniquement au format Messages Anthropic. Elle ne s'exécute pas lorsque :

* Toute variable de fournisseur `CLAUDE_CODE_USE_*` est définie, même si `ANTHROPIC_BASE_URL` est également défini
* `ANTHROPIC_BASE_URL` n'est pas défini ou pointe vers `api.anthropic.com`

La découverte s'exécute toujours lorsque [le trafic non essentiel est désactivé](/docs/fr/llm-gateway-connect#turn-off-traffic-outside-the-gateway-path), car la requête ne va que vers votre passerelle. Avant v2.1.257, la découverte ne s'exécutait pas lorsque le trafic non essentiel était désactivé.

<h3 id="request-and-response">
  Requête et réponse
</h3>

La requête est `GET /v1/models?limit=1000` avec un délai d'expiration de 3 secondes par défaut, et toute redirection est traitée comme un échec afin que l'identifiant ne puisse pas fuir vers une cible de redirection. Une passerelle qui répond lentement que le délai d'expiration, ou une qui redirige `/v1/models`, même `http` vers `https`, échoue silencieusement la découverte ; servez le point de terminaison directement à l'URL de base configurée.

Pour donner à une passerelle lente plus de temps, définissez [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](/docs/fr/env-vars#variables). La variable nécessite Claude Code v2.1.269 ou version ultérieure.

Claude Code envoie la requête de découverte avec les deux en-têtes d'identifiant ci-dessous et omet un en-tête dont la valeur ne se résout pas. L'envoi des deux en-têtes nécessite Claude Code v2.1.248 ou version ultérieure. Les versions antérieures envoient uniquement `Authorization` lorsque `ANTHROPIC_AUTH_TOKEN` est défini et uniquement `x-api-key` sinon.

* `Authorization` : `ANTHROPIC_AUTH_TOKEN` comme jeton porteur, sinon la valeur [`apiKeyHelper`](/docs/fr/llm-gateway-connect#rotate-credentials-with-apikeyhelper) comme jeton porteur. Dans ce cas, Claude Code attend que l'aide retourne avant d'envoyer la requête.
* `x-api-key` : la clé API que Claude Code a résolue, telle que `ANTHROPIC_API_KEY`. Lorsqu'une valeur d'aide est la seule identifiant, cet en-tête la porte également, de sorte que la valeur arrive dans les deux en-têtes.

Claude Code envoie également tous les en-têtes de `ANTHROPIC_CUSTOM_HEADERS`. Lorsqu'un en-tête personnalisé a une valeur non vide, Claude Code l'envoie à la place d'un en-tête intégré du même nom, en faisant correspondre les noms sans tenir compte de la casse.

Lorsqu'aucune valeur d'en-tête d'identifiant ne se résout, Claude Code ignore la découverte et écrit une ligne `[gatewayDiscovery] skipped` dans le journal de débogage d'une session `claude --debug`. Si vous fournissez un identifiant uniquement via `ANTHROPIC_CUSTOM_HEADERS`, Claude Code ignore toujours la découverte.

Claude Code lit `id`, le `display_name` optionnel et la `description` optionnelle de chaque entrée dans le tableau `data` de la réponse :

```json theme={null}
{
  "data": [
    {
      "id": "claude-sonnet-4-6",
      "display_name": "Claude Sonnet 4.6",
      "description": "Default model for everyday coding tasks"
    },
    { "id": "claude-opus-4-8" }
  ]
}
```

Claude Code conserve une entrée lorsque son `id` contient `claude` ou `anthropic` n'importe où dans la chaîne, en faisant correspondre sans tenir compte de la casse, et ignore le reste. Les ID préfixés par le fournisseur tels que `vertex_ai/claude-sonnet-4-6` ou `bedrock/anthropic.claude-sonnet-4-5` passent le filtre ; un ID qui ne contient aucune des deux sous-chaînes ne le fait pas. Avant v2.1.223, Claude Code conservait une entrée uniquement lorsque son `id` commençait par `claude` ou `anthropic`, ce qui masquait les ID préfixés par le fournisseur.

<h3 id="picker-entries-and-caching">
  Entrées du sélecteur et mise en cache
</h3>

Le sélecteur est la liste de modèles interactive qui s'ouvre lorsqu'un développeur exécute `/model` dans Claude Code. Chaque entrée découverte utilise `display_name` comme nom lorsque la passerelle en envoie un qui diffère de l'`id`. Sinon, l'entrée affiche le nom du modèle lorsque Claude Code [reconnaît l'`id`](/docs/fr/model-config#customize-pinned-model-display-and-capabilities), et l'`id` lorsqu'il ne le reconnaît pas. Par exemple, une entrée avec l'`id` `my-gateway-claude-sonnet-4-6` et aucun `display_name` apparaît comme `Sonnet 4.6`.

La découverte ajoute uniquement les modèles que le [paramètre géré `availableModels`](/docs/fr/settings-reference#availablemodels) autorise.

Chaque entrée affiche également la `description` du modèle, réduite à une ligne. Une entrée sans `description` lit « Depuis la passerelle » à la place. Avant v2.1.257, chaque entrée découverte lisait « Depuis la passerelle ».

Un ID découvert n'obtient pas sa propre ligne lorsqu'il correspond à une ligne déjà dans le sélecteur :

* Même ID : l'ID découvert correspond exactement à l'ID d'une ligne existante, ou les deux ID sont des orthographes de la même version [Fable](/docs/fr/model-config#work-with-fable).
* Même modèle qu'un alias intégré : lorsqu'un ID explicite découvert nomme le modèle auquel un alias intégré se résout actuellement, le sélecteur affiche uniquement la ligne d'alias. Par exemple, tandis que `sonnet` se résout en `claude-sonnet-5`, un `claude-sonnet-5` découvert s'effondre dans la ligne `sonnet`, et un `claude-sonnet-4-6` découvert obtient toujours sa propre ligne. Avant v2.1.197, Claude Code ne fusionnait pas ces ID dans les lignes intégrées, donc `claude-sonnet-5` obtenait également sa propre ligne « Depuis la passerelle ».

Les résultats sont mis en cache dans `~/.claude/cache/gateway-models.json`, ou `%USERPROFILE%\.claude\cache\gateway-models.json` sur Windows, et actualisés à chaque démarrage. Si vous définissez [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars), le cache se trouve sous ce répertoire à la place. Si la requête échoue ou la passerelle n'implémente pas `/v1/models`, le sélecteur revient à la liste mise en cache du démarrage précédent ou à la liste de modèles intégrée. Si votre passerelle sert les modèles Claude sous des alias qui ne correspondent pas au filtre de découverte, les développeurs peuvent ajouter ces alias manuellement avec les [variables de configuration du modèle](/docs/fr/model-config).

<h2 id="related-resources">
  Ressources connexes
</h2>

Pour le reste de l'ensemble de documentation de passerelle et les références API sous-jacentes :

* [Aperçu des passerelles](/docs/fr/gateways) : ce qu'est une passerelle et comment choisir entre la passerelle Claude apps et un autre produit
* [Autres passerelles LLM](/docs/fr/llm-gateway) : comment déployer une passerelle que votre organisation exécute et comment elle interagit avec les abonnements claude.ai
* [Déployer une passerelle LLM pour votre organisation](/docs/fr/llm-gateway-rollout) : la liste de contrôle d'administration qui utilise ce guide
* [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect) : configuration par développeur et le tableau de dépannage
* [Référence des en-têtes bêta](https://platform.claude.com/docs/en/api/beta-headers) : l'ensemble actuel des valeurs `anthropic-beta`
* [API Messages](https://platform.claude.com/docs/en/api/messages) : le format API qu'une passerelle au format Anthropic implémente
