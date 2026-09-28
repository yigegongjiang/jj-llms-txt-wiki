> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Déployer une passerelle LLM pour votre organisation

> Déployez un produit de passerelle pour Claude Code : configurez-le pour transférer ce que Claude Code envoie, émettez des identifiants de développeur, distribuez la configuration via les paramètres gérés, et vérifiez le déploiement.

Cette page guide un administrateur dans le déploiement d'une passerelle LLM pour Claude Code. Elle suppose que vous avez une passerelle déployée qui répond aux [exigences de la passerelle](#gateway-requirements). Le déploiement ou l'exploitation d'un produit spécifique ne sont pas couverts ici ; déployez le vôtre en suivant la documentation de votre fournisseur.

<Note>
  * Pour connecter Claude Code sur votre propre machine à une passerelle existante, consultez [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect)
  * Pour savoir ce que Claude Code envoie à une passerelle et ce qu'il faut transférer, consultez la [référence du protocole de passerelle](/docs/fr/llm-gateway-protocol)
</Note>

<h2 id="prerequisites">
  Prérequis
</h2>

Pour terminer le déploiement, vous aurez besoin de :

* Une passerelle déployée sur votre infrastructure, servant HTTPS à l'adresse exacte que vous distribuerez aux développeurs, pas une adresse qui redirige vers elle, et configurée pour router les noms de modèles Claude vers votre fournisseur
* Une identifiant de fournisseur pour que la passerelle le transfère avec :
  * Pour l'API Anthropic : une clé API de la [Console Claude](https://platform.claude.com/settings/keys)
  * Pour un fournisseur cloud : des identifiants cloud avec accès aux modèles. Consultez les prérequis sur la page [Amazon Bedrock](/docs/fr/amazon-bedrock#prerequisites), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai#prerequisites), ou [Microsoft Foundry](/docs/fr/microsoft-foundry#prerequisites)
* Un moyen de livrer des fichiers de paramètres aux machines des développeurs, comme MDM ou gestion de configuration
  * Si vous n'en avez pas encore, [comment les paramètres atteignent les appareils](/docs/fr/admin-setup#decide-how-settings-reach-devices) compare les options

<h3 id="gateway-requirements">
  Exigences de la passerelle
</h3>

Quel que soit le produit qui fournit la passerelle, il doit :

* **Accepter un format API pris en charge** : l'un des formats du [tableau des formats API](/docs/fr/llm-gateway-protocol#api-formats). Les étapes de déploiement ci-dessous supposent l'API Messages Anthropic à `POST /v1/messages`, que la plupart des passerelles servent
* **Diffuser les réponses** : transmettre les événements envoyés par le serveur au fur et à mesure qu'ils arrivent, y compris les pings de maintien de connexion, au lieu de mettre en mémoire tampon la réponse entière ; la section [streaming](/docs/fr/llm-gateway-protocol#streaming) couvre ce que la mise en mémoire tampon ou les pings supprimés cassent
* **Router les noms de modèles Claude** : mapper chaque nom que les développeurs utilisent à un modèle en amont. Claude Code envoie un nom de modèle tel que `claude-sonnet-4-6` dans chaque requête ; dans la plupart des produits de passerelle, le mappage est une liste de modèles ou une table de routage dans la propre configuration de la passerelle
* **Transférer les en-têtes et le corps inchangés** : transmettre `anthropic-beta`, `anthropic-version`, et le corps de la requête dans les deux sens ; le [tableau de passage des fonctionnalités](/docs/fr/llm-gateway-protocol#feature-pass-through) mappe chacun à la fonctionnalité qui se casse sans lui
* **Retourner les erreurs en amont non modifiées** : la récupération automatique de Claude Code correspond au libellé de l'erreur, donc envelopper les erreurs dans l'enveloppe propre de la passerelle la casse, sauf si l'enveloppe du message porte l'un des jetons `capability_rejected:` qu'une [passerelle d'applications Claude substitue au libellé d'erreur des fournisseurs cloud](/docs/fr/claude-apps-gateway-config#upstream-error-messages)
* **Exempter le chemin de l'inspection WAF du corps de la requête** : les invites Claude Code contiennent du code source et des balises de style XML qui correspondent aux règles du corps de cross-site-scripting ; un WAF devant la passerelle retourne `403` sur les vraies sessions tandis que les courtes demandes de test passent

Optionnellement, servez `GET /v1/models` pour que Claude Code puisse remplir le sélecteur de modèles de votre passerelle avec la [découverte de modèles](/docs/fr/llm-gateway-protocol#model-discovery).

<h2 id="rollout-steps">
  Étapes du déploiement
</h2>

Le déploiement se fait en cinq étapes, chacune avec un point de contrôle :

1. [Confirmer que la passerelle achemine vos modèles](#confirm-the-gateway-routes-your-models)
2. [Émettre une credential pour chaque développeur](#issue-developer-credentials)
3. [Tester Claude Code par rapport à la passerelle](#test-claude-code-against-the-gateway)
4. [Distribuer l'URL de base et les credentials](#distribute-the-configuration)
5. [Vérifier à partir d'une machine développeur](#verify-the-rollout)

Les étapes impliquent trois credentials différentes, et les points de contrôle les nomment par placeholder afin que vous puissiez identifier celle qui est en cause en cas d'échec :

| Credential                                 | Qui la détient                                                                                                                  | Placeholder dans les points de contrôle                                    |
| :----------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------- |
| Credential du fournisseur                  | La passerelle, qui la transmet au fournisseur en amont                                                                          | Configurée sur la passerelle ; n'apparaît jamais dans les commandes client |
| Credential administrative de la passerelle | Vous, si votre produit de passerelle en émet une pour son interface d'administration ou de test                                 | `<gateway-key>`                                                            |
| Clé développeur                            | Chaque développeur, émise par la passerelle dans [Émettre une credential pour chaque développeur](#issue-developer-credentials) | `<developer-key>`                                                          |

<h3 id="confirm-the-gateway-routes-your-models">
  Confirmer que la passerelle achemine vos modèles
</h3>

Votre passerelle devrait déjà être configurée avec votre credential du fournisseur, à l'écoute sur son URL de base, et transmettre les demandes à l'API de votre fournisseur. Testez que le chemin fonctionne de bout en bout avec une demande minimale, en remplaçant deux valeurs de votre déploiement :

* `<gateway-key>` est la credential qui vous permet d'appeler la passerelle en ce moment : une clé administrative, une clé de test, ou votre propre clé développeur si vous en avez déjà émis une. Tous les produits de passerelle n'ont pas une credential d'administration séparée ; si le vôtre n'en a pas, émettez-vous d'abord une clé développeur dans [Émettre une credential pour chaque développeur](#issue-developer-credentials)
* `model` est un nom de modèle Claude que votre passerelle est configurée pour acheminer. L'exemple utilise `claude-sonnet-4-6` ; remplacez par un nom que vous avez configuré

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <gateway-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <gateway-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Point de contrôle** : un `200` avec un champ `content` signifie que la passerelle a atteint le fournisseur avec ce nom de modèle. Un `404` signifie que ce nom n'est pas acheminé à la passerelle ; un `401` du fournisseur signifie que la credential du fournisseur de la passerelle est incorrecte.

Répétez la demande une fois par nom de modèle Claude dans la configuration d'acheminement de votre passerelle. Un nom que la passerelle n'achemine pas retourne `404` à tout développeur qui le sélectionne, donc testez chaque nom avant le déploiement.

<Note>
  Évitez de servir la passerelle derrière une redirection. Une redirection peut supprimer le corps de la demande ou retirer l'en-tête de credential sur les demandes d'inférence, et la [découverte de modèles](/docs/fr/llm-gateway-protocol#model-discovery) traite toute redirection comme un échec afin que la credential ne puisse pas fuir vers une cible de redirection.
</Note>

<h3 id="issue-developer-credentials">
  Émettre une credential pour chaque développeur
</h3>

Chaque développeur a besoin de sa propre clé de passerelle pour s'authentifier. Créez une credential par développeur à la passerelle, en suivant la documentation de gestion des credentials de votre produit.

Confirmez qu'une clé fraîchement émise fonctionne par rapport à la passerelle avec la même demande que [Confirmer que la passerelle achemine vos modèles](#confirm-the-gateway-routes-your-models), en remplaçant `<gateway-key>` par la nouvelle `<developer-key>` :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <developer-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Point de contrôle** : un `200` avec un champ `content` signifie que la clé développeur atteint la passerelle et que la passerelle la transmet. Un `401` ici, quand [l'étape précédente](#confirm-the-gateway-routes-your-models) a réussi, signifie que la clé développeur est incorrecte ou n'a pas encore pris effet à la passerelle.

Émettre une clé par développeur plutôt qu'une clé partagée est ce qui fait fonctionner l'attribution d'utilisation par développeur et le départ individuel. La variable d'environnement qui contient la clé dépend de l'en-tête que la passerelle lit. Pour une passerelle qui vérifie les credentials dans l'en-tête `Authorization: Bearer`, les développeurs définissent leur clé dans `ANTHROPIC_AUTH_TOKEN`. Pour une passerelle qui lit les clés à partir de l'en-tête `x-api-key`, les développeurs définissent `ANTHROPIC_API_KEY` à la place ; le [tableau des credentials](/docs/fr/llm-gateway-connect#set-the-credential-variable) couvre le mappage.

<h3 id="test-claude-code-against-the-gateway">
  Tester Claude Code par rapport à la passerelle
</h3>

Exécutez Claude Code par la passerelle vous-même avant de distribuer quoi que ce soit, en utilisant la même configuration que le déploiement livrera à l'échelle de la flotte. Tapez-les directement dans un terminal, pas dans un fichier `.env` ou de paramètres ; ils ne durent que pour cette session de terminal, donc la fermer ramène votre machine à sa configuration normale. Utilisez `ANTHROPIC_API_KEY` au lieu de `ANTHROPIC_AUTH_TOKEN` si votre passerelle lit l'en-tête `x-api-key` :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN="<developer-key>"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "<developer-key>"
    ```
  </Tab>
</Tabs>

Ensuite, envoyez une demande unique par la passerelle :

```bash theme={null}
claude -p "Reply with one word: connected"
```

**Point de contrôle** : la demande retourne une réponse, et la demande apparaît dans le journal de la passerelle comme un `POST` au chemin `/v1/messages` avec le statut `200`. Claude Code ajoute une chaîne de requête telle que `?beta=true`, donc faites correspondre le chemin, pas l'URL complète. Deux messages d'erreur pointent dans des directions différentes :

* `Not logged in` : vérifiez le journal de la passerelle pour distinguer les deux causes. S'il est vide, aucune credential n'a atteint la session et aucune demande n'a quitté la machine ; réexécutez les exports dans le shell que vous testez. S'il affiche une demande rejetée avec `x-api-key` dans le corps `401`, la passerelle s'attend à des clés dans cet en-tête à la place ; basculez vers `ANTHROPIC_API_KEY`
* `Failed to authenticate. API Error: 401` signifie qu'une credential a été envoyée et rejetée, et le journal de la passerelle dit où : un `401` nommant `api.anthropic.com` ou le point de terminaison de votre fournisseur signifie que la passerelle a atteint l'amont mais sa credential du fournisseur a été rejetée, donc la clé développeur a fonctionné et la credential du fournisseur que la passerelle détient est incorrecte ou un placeholder

Une URL de base incorrecte ou inaccessible produit un symptôme différent : Claude Code [réessaie la connexion avec backoff](/docs/fr/errors#automatic-retries) et peut rester sans sortie pendant plusieurs minutes avant de signaler une erreur. Si la commande semble se bloquer, vérifiez le journal de la passerelle au lieu d'attendre ; aucune demande arrivante signifie que `ANTHROPIC_BASE_URL` ne pointe pas vers la passerelle.

<h3 id="distribute-the-configuration">
  Distribuer la configuration
</h3>

Chaque machine développeur a besoin de l'adresse de la passerelle et d'une credential. Vous pouvez les distribuer de manière centralisée par le biais des [paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms), afin que les développeurs ne configurent rien, ou donner aux développeurs les valeurs à définir eux-mêmes.

<h4 id="what-to-distribute">
  Ce qu'il faut distribuer
</h4>

Le même ensemble de variables s'applique quel que soit le chemin que vous choisissez. La plupart des déploiements n'ont besoin que de `ANTHROPIC_BASE_URL` et d'une credential ; incluez les lignes conditionnelles quand votre configuration de passerelle l'exige.

| Variable ou paramètre                                                                                                                                                                                                              | Ce qu'il fait                                                                                                                                                                                                                                    | Inclure quand                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_BASE_URL`                                                                                                                                                                                                               | Envoie les demandes API de Claude Code à la passerelle au lieu de `api.anthropic.com`                                                                                                                                                            | Toujours                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `apiKeyHelper`, ou une credential dans `ANTHROPIC_AUTH_TOKEN` ou `ANTHROPIC_API_KEY`                                                                                                                                               | Authentifie chaque demande à la passerelle. L'assistant exécute une commande pour récupérer la clé ; les variables contiennent une clé statique, envoyée comme `Authorization: Bearer` et `x-api-key` respectivement                             | Toujours ; l'un des trois                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `ANTHROPIC_CUSTOM_HEADERS`                                                                                                                                                                                                         | Ajoute des en-têtes HTTP supplémentaires à chaque demande API                                                                                                                                                                                    | Votre passerelle nécessite un en-tête de locataire ou d'acheminement sur chaque demande                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `CLAUDE_CODE_GATEWAY_HINT_HEADERS`                                                                                                                                                                                                 | Envoie les [en-têtes d'indication de passerelle](/docs/fr/llm-gateway-protocol#gateway-hint-headers), qui classent chaque demande pour les décisions d'acheminement et de planification à la passerelle. Nécessite Claude Code v2.1.273 ou ultérieur  | Votre passerelle lit les en-têtes d'indication                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY`                                                                                                                                                                                       | Interroge le `/v1/models` de la passerelle au démarrage et ajoute les noms retournés au sélecteur `/model`                                                                                                                                       | Votre passerelle sert `/v1/models` et vous voulez que les sélecteurs des développeurs soient remplis à partir de celui-ci                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`                                                                                                                                                                                           | Arrête Claude Code d'envoyer les en-têtes et champs de corps de capacités de pré-version. [Désactiver les capacités de pré-version](/docs/fr/llm-gateway-protocol#disable-pre-release-capabilities) couvre la portée exacte                           | Votre passerelle transmet à un amont Amazon Bedrock ou Google Cloud's Agent Platform qui rejette les champs bêta. Voir [Exigences de la passerelle](#gateway-requirements)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` ou `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK`                                                                                                                                              | Restaure le [mode rapide](/docs/fr/fast-mode) quand sa vérification de disponibilité, qui appelle `api.anthropic.com` directement plutôt que de suivre `ANTHROPIC_BASE_URL`, échoue, est interceptée, ou est ignorée faute d'une credential Anthropic | Votre organisation utilise le mode rapide, et les développeurs s'authentifient avec `ANTHROPIC_AUTH_TOKEN` seul, avec une clé émise par la passerelle dans `ANTHROPIC_API_KEY` ou à partir d'un `apiKeyHelper`, ou votre réseau bloque ou intercepte les demandes directes à `api.anthropic.com` ; [utiliser le mode rapide derrière les proxies et les passerelles LLM](/docs/fr/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) couvre laquelle des deux variables correspond à votre configuration                                                                                  |
| `ANTHROPIC_MODEL` ou [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/fr/model-config)                                                                                                                                                           | Définit quel nom de modèle Claude Code demande pour la session principale et pour le trafic en arrière-plan                                                                                                                                      | Votre passerelle achemine des noms de modèles qui ne correspondent pas aux valeurs par défaut de Claude Code, ou vous acheminez la [fonctionnalité en arrière-plan](/docs/fr/costs#background-token-usage) vers un modèle différent. Acheminez à la fois les noms de remplacement et les ID de modèle intégrés que Claude Code demande quand aucun remplacement n'est défini, car certains appels de sous-tâches en arrière-plan demandent un ID intégré indépendamment du remplacement ; la [configuration du modèle](/docs/fr/model-config) couvre quel modèle chaque partie d'une session utilise |
| `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, `ANTHROPIC_FOUNDRY_BASE_URL`, ou `ANTHROPIC_AWS_BASE_URL` avec les [variables pour ce fournisseur](/docs/fr/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) | Pointez Claude Code vers la passerelle par le biais d'une URL de base spécifique au fournisseur. Amazon Bedrock et Google Cloud's Agent Platform basculent également vers le format de demande natif de ces fournisseurs                         | Votre passerelle fait face à Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou la plateforme Claude sur AWS ; voir [Formats API](/docs/fr/llm-gateway-protocol#api-formats)                                                                                                                                                                                                                                                                                                                                                                                                  |

<h4 id="distribute-through-managed-settings">
  Distribuer par le biais des paramètres gérés
</h4>

Livrez les variables par le biais du bloc `env` d'un [fichier de paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms), poussé par MDM, politique de registre, ou gestion de configuration :

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

Ajoutez les variables conditionnelles du tableau au même bloc `env`. Un `ANTHROPIC_BASE_URL` géré est appliqué et ne peut pas être remplacé par une exportation de shell d'un développeur, car Claude Code l'applique sur l'environnement de processus et les paramètres de priorité inférieure.

N'incluez pas `forceLoginMethod` ou `forceLoginOrgUUID` dans les paramètres gérés aux côtés d'une credential de passerelle. L'une ou l'autre clé, avec n'importe quelle valeur, bloque `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, et `apiKeyHelper` au démarrage, et les développeurs ne peuvent pas continuer. Ils voient `This machine's managed settings require a first-party login`, ou [`Administrator policy requires a Cloud gateway sign-in`](/docs/fr/errors#administrator-policy-requires-a-cloud-gateway-sign-in) sous une valeur `"gateway"`.

La livraison des [paramètres gérés par le serveur](/docs/fr/server-managed-settings#platform-availability) nécessite une connexion directe à `api.anthropic.com`, donc elle n'atteint pas les sessions acheminées par la passerelle. Les déploiements de passerelle utilisent ce chemin de paramètres gérés basé sur fichier, qui applique les mêmes clés.

Pour la credential, distribuez une commande [`apiKeyHelper`](/docs/fr/llm-gateway-connect#rotate-credentials-with-apikeyhelper) dans le fichier de paramètres gérés comme indiqué ci-dessus ; la commande s'authentifie auprès de votre magasin de secrets en tant que développeur local, afin que chaque machine reçoive sa propre clé. Alternativement, livrez à chaque développeur sa clé par le biais de votre processus de secrets existant et demandez-leur de définir `ANTHROPIC_AUTH_TOKEN` eux-mêmes.

Certains environnements ont besoin d'une livraison séparée :

* L'application de bureau lit l'acheminement de la passerelle à partir de sa configuration d'inférence tierce, pas à partir des paramètres gérés ; déployez ce fichier par le biais de MDM aux côtés des paramètres gérés afin que les sessions de bureau acheminent également par la passerelle. Voir la [documentation de configuration tierce du bureau](https://claude.com/docs/third-party/claude-desktop/configuration) et la [documentation de passerelle du bureau](https://claude.com/docs/third-party/claude-desktop/gateway)
* Les exécuteurs CI ont besoin de `ANTHROPIC_BASE_URL` et de la credential définis dans l'[environnement de l'exécuteur](/docs/fr/llm-gateway-connect#configure-each-surface)
* WSL sur les machines Windows gérées lit les paramètres gérés Windows uniquement quand [`wslInheritsWindowsSettings`](/docs/fr/settings-reference#wslinheritswindowssettings) est `true`

<h4 id="hand-developers-the-values-to-set-themselves">
  Donner aux développeurs les valeurs à définir eux-mêmes
</h4>

Si vous n'avez pas de distribution de paramètres gérés en place, envoyez à chaque développeur ce dont il a besoin pour suivre la [page de connexion](/docs/fr/llm-gateway-connect#configure-claude-code-yourself) :

* L'URL de la passerelle
* Leur credential personnelle
* **Quelle variable mettre la credential dans** : `ANTHROPIC_AUTH_TOKEN` pour une passerelle de jeton porteur, ou `ANTHROPIC_API_KEY` pour une passerelle `x-api-key`. Dire aux développeurs laquelle leur épargne l'essai-erreur décrit sur la [page de connexion](/docs/fr/llm-gateway-connect#set-the-credential-variable)
* Toutes les variables conditionnelles du [tableau Ce qu'il faut distribuer](#what-to-distribute), avec leurs valeurs

La [page de connexion](/docs/fr/llm-gateway-connect#configure-claude-code-yourself) guide les développeurs à travers la définition de chacune.

**Point de contrôle** : sur une machine développeur, `claude` démarre une session sans afficher l'écran de connexion, car la credential distribuée satisfait l'authentification. Ensuite, exécutez `/status` et ouvrez l'onglet **Status** : la ligne `Anthropic base URL` affiche l'adresse de la passerelle, et pour la distribution gérée la ligne `Setting sources` inclut les paramètres gérés. Un écran de connexion, ou une ligne `Anthropic base URL` manquante, signifie que la configuration n'a pas atteint la machine.

<h3 id="verify-the-rollout">
  Vérifier le déploiement
</h3>

Confirmez que tout fonctionne à partir d'une machine développeur, pas de l'hôte de la passerelle, afin que le test couvre le chemin réseau que les développeurs utilisent. Envoyez une demande en streaming, qui vérifie le point de terminaison, le passage en streaming, et l'acheminement du modèle à la fois :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    curl -N -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $body = '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    $body | curl.exe -N -X POST "https://llm-gateway.example.com/v1/messages" `
      -H "Authorization: Bearer <developer-key>" `
      -H "anthropic-version: 2023-06-01" `
      -H "content-type: application/json" `
      --data-binary '@-'
    ```
  </Tab>
</Tabs>

Vous devriez voir les lignes `data:` arriver de manière progressive. La réponse entière arrivant à la fois après une pause signifie que la passerelle met en mémoire tampon, ce qui bloque Claude Code ; un `404` signifie que le nom du modèle n'est pas acheminé. Répétez par nom de modèle.

Ensuite, démarrez `claude` et envoyez un message. Chaque symptôme à cette étape a une cause :

* Une invite de connexion signifie une lacune de credential. Exécutez `/status` et ouvrez l'onglet **Status** : quand la ligne `Setting sources` n'inclut pas les paramètres gérés, la distribution n'a pas atteint la machine ; quand elle le fait, la credential développeur n'a pas été livrée, donc définissez `ANTHROPIC_AUTH_TOKEN` ou le `apiKeyHelper`
* Les erreurs `Failed to authenticate` signifient que la passerelle rejette les demandes ; son journal dit quelle credential a échoué. Un rejet que la passerelle enregistre elle-même nomme la clé développeur, tandis qu'un `401` de `api.anthropic.com` ou du point de terminaison de votre fournisseur signifie que la credential du fournisseur que la passerelle détient a été rejetée
* Une invite d'approbation unique pour la clé est attendue à la première utilisation quand la passerelle s'attend à des clés dans l'en-tête `x-api-key`, défini comme `ANTHROPIC_API_KEY`. Avec `ANTHROPIC_AUTH_TOKEN`, aucune invite n'apparaît et la variable prend le relais silencieusement ; une connexion claude.ai précédemment enregistrée est inactive pour cette session

Si votre organisation utilise le [mode rapide](/docs/fr/fast-mode), exécutez `/fast` ici aussi : la vérification de disponibilité appelle `api.anthropic.com` directement plutôt que de suivre l'URL de base de la passerelle, donc une session acheminée par la passerelle peut signaler le mode rapide comme indisponible ou désactivé même si l'inférence fonctionne. [Utiliser le mode rapide derrière les proxies et les passerelles LLM](/docs/fr/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) mappe chaque message à la variable qui le restaure, distribuée avec [le reste de la configuration](#distribute-the-configuration).

Enfin, vérifiez les journaux de la passerelle pour le message que vous avez envoyé : la credential identifie le développeur, et l'[en-tête `x-claude-code-session-id`](/docs/fr/llm-gateway-protocol#request-headers) groupe les demandes par session. Si les fonctionnalités échouent avec les [symptômes de dépannage](/docs/fr/llm-gateway-connect#troubleshoot-gateway-errors), la passerelle supprime les en-têtes ou réécrit les erreurs ; voir les [exigences de la passerelle](#gateway-requirements) ci-dessus.

<h2 id="maintain-the-gateway">
  Maintenir la passerelle
</h2>

Après le déploiement, trois types de changements atteignent la passerelle au fil du temps. Chacun a un symptôme à surveiller et une action à prendre.

| Changement                                                                                                       | Symptôme quand la passerelle n'a pas suivi                                                                                                                                                         | Action                                                                                                                                                                                                                                                                                                                                                     |
| :--------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Les nouvelles versions de Claude Code ajoutent des valeurs `anthropic-beta` et des champs du corps de la requête | Les développeurs signalent des erreurs `400` nommant un nouveau champ après la mise à jour de Claude Code ; consultez [passage des fonctionnalités](/docs/fr/llm-gateway-protocol#feature-pass-through) | Transférez les en-têtes `anthropic-*` et les corps de requête verbatim plutôt que de faire une liste blanche ; testez les nouvelles versions de Claude Code contre la passerelle avant qu'elles n'atteignent les développeurs, en vérifiant les domaines dans [Planifier les mises à niveau de version de Claude Code](#plan-claude-code-version-upgrades) |
| De nouveaux modèles Claude deviennent disponibles                                                                | Les développeurs sélectionnant un nouveau nom de modèle obtiennent `404` ; le sélecteur `/model` ne le liste pas                                                                                   | Ajoutez le nom du modèle à la configuration de routage de la passerelle, puis réexécutez la [vérification du routage](#confirm-the-gateway-routes-your-models). Si vous distribuez `ANTHROPIC_MODEL` ou les variables de modèle par défaut, mettez à jour les paramètres gérés                                                                             |
| Les identifiants expirent ou ont besoin d'une rotation                                                           | Toutes les requêtes des développeurs commencent à échouer avec `401` de l'amont                                                                                                                    | Faites tourner l'identifiant de fournisseur de la passerelle selon son propre calendrier ; les clés de développeur tournent à la passerelle, et un [`apiKeyHelper`](/docs/fr/llm-gateway-connect#rotate-credentials-with-apikeyhelper) gère la rotation par développeur sans redistribuer les paramètres                                                        |

Lors du dimensionnement des limites de débit par clé, tenez compte du client [réessayant les défaillances transitoires](/docs/fr/errors#automatic-retries), y compris les réponses `429`, jusqu'à 10 fois avec backoff, en respectant `Retry-After`. Gardez la [guide de compatibilité](/docs/fr/llm-gateway-protocol) comme référence pour ce que chaque version de Claude Code envoie.

<h3 id="plan-claude-code-version-upgrades">
  Planifier les mises à niveau de version de Claude Code
</h3>

Certains comportements de Claude Code sont intégrés à la version installée plutôt que définis à votre passerelle, donc déplacer les développeurs vers une nouvelle version peut modifier le comportement dans tout votre déploiement même quand la configuration de la passerelle n'a pas changé. Pour contrôler quand cela se produit, épinglez les développeurs à une version testée avec [`requiredMaximumVersion`](/docs/fr/settings-reference#requiredmaximumversion), ou avec [`DISABLE_UPDATES`](/docs/fr/setup#disable-auto-updates) si vous distribuez Claude Code via votre propre canal. Avant de relever l'épingle, lisez l'entrée du [changelog](/docs/en/changelog) de la nouvelle version et [testez-la contre la passerelle](#test-claude-code-against-the-gateway).

Quand vous testez une version, les nouveaux en-têtes ou champs de requête que la passerelle rejette apparaissent comme les erreurs `400` décrites dans [Maintenir la passerelle](#maintain-the-gateway). Le tableau ci-dessous couvre les changements dépendants de la version qui ne produisent pas d'erreur, avec le paramètre qui garde chacun constant lors des mises à niveau.

| Domaine                                           | Ce qui peut changer quand les développeurs mettent à niveau                                                                                                                                                                                                                                                                                                                                                                                         | Paramètre qui le garde constant                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Valeurs par défaut des drapeaux de fonctionnalité | Les sessions qui [ne récupèrent pas les drapeaux de fonctionnalité auprès d'Anthropic](/docs/fr/env-vars#features-that-need-feature-flag-fetching), comme les sessions sur un fournisseur cloud ou avec la télémétrie désactivée, utilisent les valeurs par défaut des drapeaux intégrées à la version installée. Quand une version change l'une de ces valeurs par défaut, le comportement change pour ces développeurs dès qu'ils mettent à niveau     | L'épingle de version elle-même, `requiredMaximumVersion` ou `DISABLE_UPDATES`                                                                                                                                                                                                                                                                                                        |
| Hypothèses de capacité du modèle                  | Un ID de modèle que la version installée ne reconnaît pas, comme l'alias de passerelle `prod-opus`, s'exécute sur les hypothèses par défaut pour [le raisonnement adaptatif](/docs/fr/model-config#adaptive-reasoning-and-fixed-thinking-budgets), le paramètre d'effort, et la [fenêtre de contexte](/docs/fr/model-config#correct-the-window-for-a-gateway-or-custom-model-id) jusqu'à ce qu'une version ultérieure reconnaisse l'ID ou que vous le mappiez | Routez les ID de modèle Anthropic à la passerelle, ou ajoutez une entrée [`modelOverrides`](/docs/fr/model-config#override-model-ids-per-version) qui mappe l'ID de modèle Anthropic à votre alias. Sur une connexion de fournisseur cloud, vous pouvez à la place [déclarer les capacités d'un modèle épinglé](/docs/fr/model-config#customize-pinned-model-display-and-capabilities)         |
| Modèle par défaut et alias                        | Le modèle sur lequel les nouvelles sessions commencent par défaut, et les modèles que les alias tels que `opus` et `sonnet` résolvent, sont [intégrés à chaque version](/docs/fr/model-config#pin-models-for-third-party-deployments) et peuvent changer quand les développeurs mettent à niveau                                                                                                                                                         | [`ANTHROPIC_DEFAULT_MODEL`](/docs/fr/model-config#set-a-default-model-for-new-sessions) pour le modèle sur lequel les nouvelles sessions commencent, et les [variables `ANTHROPIC_DEFAULT_*_MODEL`](/docs/fr/model-config#environment-variables), comme `ANTHROPIC_DEFAULT_OPUS_MODEL`, pour ce que chaque alias résout. `ANTHROPIC_DEFAULT_MODEL` nécessite Claude Code v2.1.236 ou ultérieur |

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect) : les étapes de configuration côté développeur, avec la configuration par surface et un tableau de dépannage que vous pouvez remettre aux développeurs
* [Guide de compatibilité de la passerelle](/docs/fr/llm-gateway-protocol) : la référence pour les opérateurs de passerelle, couvrant les points de terminaison, les en-têtes à transférer, et le tableau de passage des fonctionnalités
* [Quelle valeur Claude Code utilise](/docs/fr/settings#which-value-claude-code-uses) : comment les paramètres gérés, de projet et utilisateur se combinent
* [Mécanismes de livraison](/docs/fr/managed-settings#delivery-mechanisms) : où le fichier géré va sur chaque plateforme
* [Configurer Claude Code pour votre organisation](/docs/fr/admin-setup) : le déploiement plus large dont cette passerelle est une partie, y compris l'application des politiques, la visibilité de l'utilisation, et la gestion des données
