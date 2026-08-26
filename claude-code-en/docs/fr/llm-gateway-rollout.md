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
* **Diffuser les réponses** : transmettre les événements envoyés par le serveur au fur et à mesure qu'ils arrivent au lieu de mettre en mémoire tampon la réponse entière
* **Router les noms de modèles Claude** : mapper chaque nom que les développeurs utilisent à un modèle en amont. Claude Code envoie un nom de modèle tel que `claude-sonnet-4-6` dans chaque requête ; dans la plupart des produits de passerelle, le mappage est une liste de modèles ou une table de routage dans la propre configuration de la passerelle
* **Transférer les en-têtes et le corps inchangés** : transmettre `anthropic-beta`, `anthropic-version`, et le corps de la requête dans les deux sens ; le [tableau de passage des fonctionnalités](/docs/fr/llm-gateway-protocol#feature-pass-through) mappe chacun à la fonctionnalité qui se casse sans lui
* **Retourner les erreurs en amont non modifiées** : la récupération automatique de Claude Code correspond au libellé de l'erreur, donc envelopper les erreurs dans l'enveloppe propre de la passerelle la casse
* **Exempter le chemin de l'inspection WAF du corps de la requête** : les invites Claude Code contiennent du code source et des balises de style XML qui correspondent aux règles du corps de cross-site-scripting ; un WAF devant la passerelle retourne `403` sur les vraies sessions tandis que les courtes demandes de test passent

Optionnellement, servez `GET /v1/models` pour que Claude Code puisse remplir le sélecteur de modèles de votre passerelle avec la [découverte de modèles](/docs/fr/llm-gateway-protocol#model-discovery).&#x20;

<h2 id="rollout-steps">
  Étapes du déploiement
</h2>

Le déploiement prend cinq étapes, chacune avec un point de contrôle :

1. [Confirmer que la passerelle route vos modèles](#confirm-the-gateway-routes-your-models)
2. [Émettre une identifiant à chaque développeur](#issue-developer-credentials)
3. [Tester Claude Code contre la passerelle](#test-claude-code-against-the-gateway)
4. [Distribuer l'URL de base et les identifiants](#distribute-the-configuration)
5. [Vérifier à partir d'une machine de développeur](#verify-the-rollout)

Les étapes impliquent trois identifiants différents, et les points de contrôle les nomment par placeholder pour que vous puissiez dire lequel est en cause quand quelque chose échoue :

| Identifiant                                | Qui le détient                                                                                                          | Placeholder dans les points de contrôle                                   |
| :----------------------------------------- | :---------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| Identifiant de fournisseur                 | La passerelle, qui le transfère au fournisseur en amont                                                                 | Configuré sur la passerelle ; n'apparaît jamais dans les commandes client |
| Identifiant administratif de la passerelle | Vous, si votre produit de passerelle en émet un pour son interface d'administration ou de test                          | `<gateway-key>`                                                           |
| Clé de développeur                         | Chaque développeur, émis par la passerelle dans [Émettre des identifiants de développeur](#issue-developer-credentials) | `<developer-key>`                                                         |

<h3 id="confirm-the-gateway-routes-your-models">
  Confirmer que la passerelle route vos modèles
</h3>

Votre passerelle devrait déjà être configurée avec votre identifiant de fournisseur, écoutant à son URL de base, et transférant les requêtes à l'API de votre fournisseur. Testez que le chemin fonctionne de bout en bout avec une requête minimale, en substituant deux valeurs de votre déploiement :

* `<gateway-key>` est tout identifiant qui vous permet d'appeler la passerelle en ce moment : une clé administrative, une clé de test, ou votre propre clé de développeur si vous en avez déjà émis une. Pas tous les produits de passerelle ont un identifiant administratif séparé ; si le vôtre n'en a pas, émettez-vous d'abord une clé de développeur dans [Émettre des identifiants de développeur](#issue-developer-credentials)
* `model` est un nom de modèle Claude que votre passerelle est configurée pour router. L'exemple utilise `claude-sonnet-4-6` ; substituez un nom que vous avez configuré

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

**Point de contrôle** : un `200` avec un champ `content` signifie que la passerelle a atteint le fournisseur avec ce nom de modèle. Un `404` signifie que ce nom n'est pas routé à la passerelle ; un `401` du fournisseur signifie que l'identifiant de fournisseur de la passerelle est incorrect.

Répétez la requête une fois par nom de modèle Claude dans la configuration de routage de votre passerelle. Un nom que la passerelle ne route pas retourne `404` à tout développeur qui le sélectionne, donc testez chaque nom avant le déploiement.

<Note>
  Évitez de servir la passerelle derrière une redirection. Une redirection peut supprimer le corps de la requête ou supprimer l'en-tête d'identifiant sur les requêtes d'inférence, et la [découverte de modèles](/docs/fr/llm-gateway-protocol#model-discovery) traite toute redirection comme un échec pour que l'identifiant ne puisse pas fuir vers une cible de redirection.
</Note>

<h3 id="issue-developer-credentials">
  Émettre des identifiants de développeur
</h3>

Chaque développeur a besoin de sa propre clé de passerelle pour s'authentifier. Créez un identifiant par développeur à la passerelle, en suivant la documentation de gestion des identifiants de votre produit.

Confirmez qu'une clé fraîchement émise fonctionne contre la passerelle avec la même requête que [Confirmer que la passerelle route vos modèles](#confirm-the-gateway-routes-your-models), en remplaçant `<gateway-key>` par la nouvelle `<developer-key>` :

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

**Point de contrôle** : un `200` avec un champ `content` signifie que la clé de développeur atteint la passerelle et que la passerelle la transfère. Un `401` ici, quand [l'étape précédente](#confirm-the-gateway-routes-your-models) a réussi, signifie que la clé de développeur est incorrecte ou n'a pas encore pris effet à la passerelle.

Émettre une clé par développeur plutôt qu'une clé partagée est ce qui rend l'attribution d'utilisation par développeur et le départ individuel possibles. La variable d'environnement qui contient la clé dépend de quel en-tête la passerelle lit. Pour une passerelle qui vérifie les identifiants dans l'en-tête `Authorization: Bearer`, les développeurs définissent leur clé dans `ANTHROPIC_AUTH_TOKEN`. Pour une passerelle qui lit les clés de l'en-tête `x-api-key`, les développeurs définissent `ANTHROPIC_API_KEY` à la place ; le [tableau des identifiants](/docs/fr/llm-gateway-connect#set-the-credential-variable) couvre le mappage.

<h3 id="test-claude-code-against-the-gateway">
  Tester Claude Code contre la passerelle
</h3>

Exécutez Claude Code via la passerelle vous-même avant de distribuer quoi que ce soit, en utilisant la même configuration que le déploiement livrera à l'échelle de la flotte. Tapez-les directement dans un terminal, pas dans un fichier `.env` ou de paramètres ; ils ne durent que pour cette session de terminal, donc la fermer retourne votre machine à sa configuration normale. Utilisez `ANTHROPIC_API_KEY` au lieu de `ANTHROPIC_AUTH_TOKEN` si votre passerelle lit l'en-tête `x-api-key` :

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

Ensuite, envoyez une invite unique via la passerelle :

```bash theme={null}
claude -p "Reply with one word: connected"
```

**Point de contrôle** : l'invite retourne une réponse, et la requête apparaît dans le journal de la passerelle comme un `POST` au chemin `/v1/messages` avec le statut `200`. Claude Code ajoute une chaîne de requête telle que `?beta=true`, donc correspondez sur le chemin, pas l'URL complète. Deux messages d'erreur pointent dans des directions différentes :

* `Not logged in` : vérifiez le journal de la passerelle pour distinguer les deux causes. S'il est vide, aucun identifiant n'a atteint la session et aucune requête n'a quitté la machine ; réexécutez les exports dans le shell que vous testez. S'il affiche une requête rejetée avec `x-api-key` dans le corps `401`, la passerelle s'attend à des clés dans cet en-tête à la place ; basculez vers `ANTHROPIC_API_KEY`
* `Failed to authenticate. API Error: 401` signifie qu'un identifiant a été envoyé et rejeté, et le journal de la passerelle dit où : un `401` nommant `api.anthropic.com` ou le point de terminaison de votre fournisseur signifie que la passerelle a atteint l'amont mais son identifiant de fournisseur a été rejeté, donc la clé de développeur a fonctionné et l'identifiant de fournisseur que la passerelle détient est incorrect ou un placeholder

Une URL de base incorrecte ou inaccessible produit un symptôme différent : Claude Code [réessaie la connexion avec backoff](/docs/fr/errors#automatic-retries) et peut rester sans sortie pendant plusieurs minutes avant de signaler une erreur. Si la commande semble se bloquer, vérifiez le journal de la passerelle au lieu d'attendre ; aucune requête arrivante signifie que `ANTHROPIC_BASE_URL` ne pointe pas vers la passerelle.

<h3 id="distribute-the-configuration">
  Distribuer la configuration
</h3>

Chaque machine de développeur a besoin de l'adresse de la passerelle et d'un identifiant. Vous pouvez les distribuer de manière centralisée via les [paramètres gérés](/docs/fr/settings#settings-files), pour que les développeurs ne configurent rien, ou remettre aux développeurs les valeurs à définir eux-mêmes.

<h4 id="what-to-distribute">
  Ce qu'il faut distribuer
</h4>

Le même ensemble de variables s'applique quel que soit le chemin que vous choisissez. La plupart des déploiements n'ont besoin que de `ANTHROPIC_BASE_URL` et d'un identifiant ; incluez les lignes conditionnelles quand votre configuration de passerelle l'exige.

| Variable ou paramètre                                                                                                                                                                                                              | Ce qu'il fait                                                                                                                                                                                                        | Inclure quand                                                                                                                                                                                                                                                                                                                                                                                                         |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_BASE_URL`                                                                                                                                                                                                               | Envoie les requêtes API de Claude Code à la passerelle au lieu de `api.anthropic.com`                                                                                                                                | Toujours                                                                                                                                                                                                                                                                                                                                                                                                              |
| `apiKeyHelper`, ou un identifiant dans `ANTHROPIC_AUTH_TOKEN` ou `ANTHROPIC_API_KEY`                                                                                                                                               | Authentifie chaque requête à la passerelle. L'assistant exécute une commande pour récupérer la clé ; les variables contiennent une clé statique, envoyée comme `Authorization: Bearer` et `x-api-key` respectivement | Toujours ; l'un des trois                                                                                                                                                                                                                                                                                                                                                                                             |
| `ANTHROPIC_CUSTOM_HEADERS`                                                                                                                                                                                                         | Ajoute des en-têtes HTTP supplémentaires à chaque requête API                                                                                                                                                        | Votre passerelle nécessite un en-tête de locataire ou de routage sur chaque requête                                                                                                                                                                                                                                                                                                                                   |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY`                                                                                                                                                                                       | Interroge `/v1/models` de la passerelle au démarrage et ajoute les noms retournés au sélecteur `/model`                                                                                                              | Votre passerelle sert `/v1/models` et vous voulez que les sélecteurs des développeurs soient remplis à partir de celui-ci                                                                                                                                                                                                                                                                                             |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`                                                                                                                                                                                           | Arrête Claude Code d'envoyer les en-têtes de capacité de pré-version et les champs du corps                                                                                                                          | Votre passerelle transfère à un amont Bedrock ou Vertex qui rejette les champs bêta ; consultez [Exigences de la passerelle](#gateway-requirements)                                                                                                                                                                                                                                                                   |
| `ANTHROPIC_MODEL` ou [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/fr/model-config)                                                                                                                                                           | Définissez quel nom de modèle Claude Code demande pour la session principale et pour le trafic en arrière-plan                                                                                                       | Votre passerelle route les noms de modèles qui ne correspondent pas aux valeurs par défaut de Claude Code, ou vous routez la [fonctionnalité en arrière-plan](/docs/fr/costs#background-token-usage) vers un modèle différent. Routez à la fois les noms de remplacement et les noms par défaut de Claude Code à la passerelle, car certains sous-appels peuvent demander le nom par défaut indépendamment du remplacement |
| `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, `ANTHROPIC_FOUNDRY_BASE_URL`, ou `ANTHROPIC_AWS_BASE_URL` avec les [variables pour ce fournisseur](/docs/fr/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) | Pointez Claude Code vers la passerelle via une URL de base spécifique au fournisseur. Bedrock et Vertex basculent également vers le format de requête natif de ces fournisseurs                                      | Votre passerelle fait face à Bedrock, Vertex, Foundry, ou la Plateforme Claude sur AWS ; consultez [Formats API](/docs/fr/llm-gateway-protocol#api-formats)                                                                                                                                                                                                                                                                |

<h4 id="distribute-through-managed-settings">
  Distribuer via les paramètres gérés
</h4>

Livrez les variables via le bloc `env` d'un [fichier de paramètres gérés](/docs/fr/settings#settings-files), poussé par MDM, politique de registre, ou gestion de configuration :

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

Ajoutez les variables conditionnelles du tableau au même bloc `env`. Un `ANTHROPIC_BASE_URL` géré est appliqué et ne peut pas être remplacé par l'export shell d'un développeur, puisque Claude Code l'applique sur l'environnement du processus et les paramètres de priorité inférieure.

N'incluez pas `forceLoginMethod` ou `forceLoginOrgUUID` dans les paramètres gérés aux côtés d'un identifiant de passerelle. Sur Claude Code v2.1.146 et ultérieur, l'une ou l'autre clé bloque `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, et `apiKeyHelper` au démarrage, pour que les développeurs voient `This machine's managed settings require a first-party login` et ne puissent pas continuer.&#x20;

La livraison des [paramètres gérés par le serveur](/docs/fr/server-managed-settings#platform-availability) nécessite une connexion directe à `api.anthropic.com`, elle n'atteint donc pas les sessions routées par passerelle. Les déploiements de passerelle utilisent ce chemin de paramètres gérés basé sur fichier, qui applique les mêmes clés.

Pour l'identifiant, distribuez une commande [`apiKeyHelper`](/docs/fr/llm-gateway-connect#rotate-credentials-with-apikeyhelper) dans le fichier de paramètres gérés comme indiqué ci-dessus ; la commande s'authentifie auprès de votre magasin de secrets en tant que développeur local, pour que chaque machine reçoive sa propre clé. Alternativement, livrez à chaque développeur sa clé via votre processus de secrets existant et faites-le définir `ANTHROPIC_AUTH_TOKEN` lui-même.

Certains environnements ont besoin d'une livraison séparée :

* L'application de bureau lit le routage de la passerelle uniquement à partir de sa configuration d'inférence tierce livrée par MDM ; déployez ce fichier aux côtés des paramètres gérés pour que les sessions de bureau routent également via la passerelle. Consultez la [documentation de configuration tierce du bureau](https://claude.com/docs/third-party/claude-desktop/configuration) et la [documentation de passerelle du bureau](https://claude.com/docs/third-party/claude-desktop/gateway)
* Les exécuteurs CI ont besoin de `ANTHROPIC_BASE_URL` et de l'identifiant définis dans l'[environnement de l'exécuteur](/docs/fr/llm-gateway-connect#configure-each-surface)
* WSL sur les machines Windows gérées lit les paramètres gérés Windows uniquement quand [`wslInheritsWindowsSettings`](/docs/fr/settings#available-settings) est `true`

<h4 id="hand-developers-the-values-to-set-themselves">
  Remettre aux développeurs les valeurs à définir eux-mêmes
</h4>

Si vous n'avez pas de distribution de paramètres gérés en place, envoyez à chaque développeur ce dont il a besoin pour suivre la [page de connexion](/docs/fr/llm-gateway-connect#configure-claude-code-yourself) :

* L'URL de la passerelle
* Leur identifiant personnel
* **Quelle variable mettre l'identifiant dans** : `ANTHROPIC_AUTH_TOKEN` pour une passerelle bearer-token, ou `ANTHROPIC_API_KEY` pour une passerelle `x-api-key`. Dire aux développeurs lequel économise le procès et erreur décrit sur la [page de connexion](/docs/fr/llm-gateway-connect#set-the-credential-variable)
* Toutes les variables conditionnelles du [tableau Ce qu'il faut distribuer](#what-to-distribute), avec leurs valeurs

La [page de connexion](/docs/fr/llm-gateway-connect#configure-claude-code-yourself) guide les développeurs à travers la définition de chacun.

**Point de contrôle** : sur une machine de développeur, `claude` démarre une session sans afficher l'écran de connexion, puisque l'identifiant distribué satisfait l'authentification. Ensuite, exécutez `/status` et ouvrez l'onglet **Status** : la ligne `Anthropic base URL` affiche l'adresse de la passerelle, et pour la distribution gérée la ligne `Setting sources` inclut les paramètres gérés. Un écran de connexion, ou une ligne `Anthropic base URL` manquante, signifie que la configuration n'a pas atteint la machine.

<h3 id="verify-the-rollout">
  Vérifier le déploiement
</h3>

Confirmez que tout fonctionne à partir d'une machine de développeur, pas de l'hôte de la passerelle, pour que le test couvre le chemin réseau que les développeurs utilisent. Envoyez une requête en streaming, qui vérifie le point de terminaison, le passage en streaming, et le routage des modèles à la fois :

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

Vous devriez voir les lignes `data:` arriver progressivement. La réponse entière arrivant à la fois après une pause signifie que la passerelle met en mémoire tampon, ce qui bloque Claude Code ; un `404` signifie que le nom du modèle n'est pas routé. Répétez par nom de modèle.

Ensuite, démarrez `claude` et envoyez un message. Chaque symptôme à cette étape a une cause :

* Une invite de connexion signifie un écart d'identifiant. Exécutez `/status` et ouvrez l'onglet **Status** : quand la ligne `Setting sources` n'inclut pas les paramètres gérés, la distribution n'a pas atteint la machine ; quand elle le fait, l'identifiant de développeur n'a pas été livré, donc définissez `ANTHROPIC_AUTH_TOKEN` ou le `apiKeyHelper`
* Les erreurs `Failed to authenticate` signifient que la passerelle rejette les requêtes ; son journal dit quel identifiant a échoué. Un rejet que la passerelle enregistre elle-même nomme la clé de développeur, tandis qu'un `401` de `api.anthropic.com` ou du point de terminaison de votre fournisseur signifie que l'identifiant de fournisseur que la passerelle détient a été rejeté
* Une invite d'approbation unique pour la clé est attendue à la première utilisation quand la passerelle s'attend à des clés dans l'en-tête `x-api-key`, défini comme `ANTHROPIC_API_KEY`. Avec `ANTHROPIC_AUTH_TOKEN`, aucune invite n'apparaît et la variable prend le relais silencieusement ; une connexion claude.ai précédemment enregistrée est inactive pour cette session

Enfin, vérifiez les journaux de la passerelle pour le message que vous avez envoyé : l'identifiant identifie le développeur, et l'[en-tête `x-claude-code-session-id`](/docs/fr/llm-gateway-protocol#request-headers) groupe les requêtes par session. Si les fonctionnalités échouent avec les [symptômes de dépannage](/docs/fr/llm-gateway-connect#troubleshoot-gateway-errors), la passerelle supprime les en-têtes ou réécrit les erreurs ; consultez les [exigences de la passerelle](#gateway-requirements) ci-dessus.

<h2 id="maintain-the-gateway">
  Maintenir la passerelle
</h2>

Après le déploiement, trois types de changements atteignent la passerelle au fil du temps. Chacun a un symptôme à surveiller et une action à prendre.

| Changement                                                                                                       | Symptôme quand la passerelle n'a pas suivi                                                                                                                                                         | Action                                                                                                                                                                                                                                                                                              |
| :--------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Les nouvelles versions de Claude Code ajoutent des valeurs `anthropic-beta` et des champs du corps de la requête | Les développeurs signalent des erreurs `400` nommant un nouveau champ après la mise à jour de Claude Code ; consultez [passage des fonctionnalités](/docs/fr/llm-gateway-protocol#feature-pass-through) | Transférez les en-têtes `anthropic-*` et les corps de requête verbatim plutôt que de faire une liste blanche ; testez les nouvelles versions de Claude Code contre la passerelle avant qu'elles n'atteignent les développeurs                                                                       |
| De nouveaux modèles Claude deviennent disponibles                                                                | Les développeurs sélectionnant un nouveau nom de modèle obtiennent `404` ; le sélecteur `/model` ne le liste pas                                                                                   | Ajoutez le nom du modèle à la configuration de routage de la passerelle, puis réexécutez la [vérification du routage](#confirm-the-gateway-routes-your-models). Si vous distribuez `ANTHROPIC_MODEL` ou les variables de modèle par défaut, mettez à jour les paramètres gérés                      |
| Les identifiants expirent ou ont besoin d'une rotation                                                           | Toutes les requêtes des développeurs commencent à échouer avec `401` de l'amont                                                                                                                    | Faites tourner l'identifiant de fournisseur de la passerelle selon son propre calendrier ; les clés de développeur tournent à la passerelle, et un [`apiKeyHelper`](/docs/fr/llm-gateway-connect#rotate-credentials-with-apikeyhelper) gère la rotation par développeur sans redistribuer les paramètres |

Lors du dimensionnement des limites de débit par clé, tenez compte du client [réessayant les défaillances transitoires](/docs/fr/errors#automatic-retries), y compris les réponses `429`, jusqu'à 10 fois avec backoff, en respectant `Retry-After`. Gardez la [référence du protocole](/docs/fr/llm-gateway-protocol) comme le contrat pour ce que chaque version de Claude Code envoie.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect) : les étapes de configuration côté développeur, avec la configuration par surface et un tableau de dépannage que vous pouvez remettre aux développeurs
* [Référence du protocole de passerelle](/docs/fr/llm-gateway-protocol) : le contrat de fil pour les opérateurs de passerelle, couvrant les points de terminaison, les en-têtes à transférer, et le tableau de passage des fonctionnalités
* [Fichiers de paramètres et précédence](/docs/fr/settings#settings-files) : comment les paramètres gérés, de projet et utilisateur se combinent, et où le fichier géré va sur chaque plateforme
* [Configurer Claude Code pour votre organisation](/docs/fr/admin-setup) : le déploiement plus large dont cette passerelle est une partie, y compris l'application des politiques, la visibilité de l'utilisation, et la gestion des données
