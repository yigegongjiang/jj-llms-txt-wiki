> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Connecter Claude Code à une passerelle LLM

> Pointez Claude Code vers la passerelle LLM de votre organisation. Vérifiez si votre administrateur l'a déjà configurée, ou définissez vous-même l'URL de base et les identifiants, puis vérifiez la connexion et corrigez les erreurs de passerelle.

Une [passerelle LLM](/docs/fr/llm-gateway) est un proxy que votre organisation exécute entre Claude Code et le fournisseur de modèle. Lorsque votre organisation en utilise une, Claude Code s'authentifie auprès de la passerelle avec un identifiant que votre organisation émet au lieu de votre connexion personnelle claude.ai.

Cette page est destinée aux développeurs exécutant Claude Code via une passerelle que leur organisation exploite. Elle couvre deux chemins : [vérifier si votre administrateur l'a déjà configurée pour vous](#check-for-an-existing-configuration), et [la configurer vous-même](#configure-claude-code-yourself) s'il ne l'a pas fait.

<Note>
  * Pour déployer une passerelle pour votre organisation, voir [Déployer une passerelle LLM](/docs/fr/llm-gateway-rollout)
  * Pour savoir ce que Claude Code envoie à une passerelle, voir la [référence du protocole de passerelle](/docs/fr/llm-gateway-protocol)
</Note>

<h2 id="check-for-an-existing-configuration">
  Vérifier une configuration existante
</h2>

Les administrateurs peuvent distribuer l'adresse de la passerelle et l'identifiant via les [paramètres gérés](/docs/fr/managed-settings), la gestion des appareils, ou un [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), de sorte que Claude Code les récupère au démarrage sans rien à configurer de votre côté. Pour vérifier si votre organisation l'a déjà fait :

<Steps>
  <Step title="Démarrer Claude Code">
    Exécutez `claude`. S'il s'ouvre sur l'écran de connexion au lieu d'une session, aucun identifiant de passerelle n'a été distribué ; [configurez-le vous-même](#configure-claude-code-yourself) ci-dessous.
  </Step>

  <Step title="Vérifier l'onglet Statut">
    Si Claude Code a démarré une session sans afficher l'écran de connexion, exécutez `/status`, qui s'ouvre sur l'onglet **Status**, et vérifiez deux lignes :

    * `Anthropic base URL` : cette ligne n'apparaît que lorsqu'une adresse de passerelle est définie. Si elle n'est pas là, Claude Code n'est pas pointé vers la passerelle ; [configurez-le vous-même](#configure-claude-code-yourself) ci-dessous.
    * `Auth token` ou `API key` : une ligne nommant `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, ou un `apiKeyHelper` confirme qu'un identifiant de passerelle est actif. Une ligne `Login method` nommant un compte claude.ai à la place signifie que l'identifiant n'a pas été distribué ; [définissez-le vous-même](#set-the-credential-variable).
  </Step>

  <Step title="Envoyer un message de test">
    Fermez le menu `/status` et envoyez n'importe quel message dans Claude Code. Une réponse normale de Claude, sans erreur, confirme que la connexion à la passerelle fonctionne.
  </Step>
</Steps>

Si les deux lignes du menu `/status` semblent correctes mais que le message à Claude échoue, voir le [tableau de dépannage](#troubleshoot-gateway-errors).

<h2 id="configure-claude-code-yourself">
  Configurer Claude Code vous-même
</h2>

Pour configurer Claude Code pour la passerelle vous-même, vous avez besoin de votre équipe de passerelle :

* L'URL de base de la passerelle
* Un identifiant : une chaîne de clé ou de jeton, ou une commande qui en récupère un
  * Si votre équipe de passerelle n'a pas dit quel type d'identifiant c'est, la section [variable d'identifiant](#set-the-credential-variable) ci-dessous couvre ce qu'il faut essayer

Les sections ci-dessous couvrent la configuration dans l'ordre :

* [Définir la variable d'identifiant](#set-the-credential-variable) et [définir l'URL de base](#set-the-base-url-and-credential) : les deux variables que chaque connexion de passerelle nécessite
* [Vérifier la connexion](#verify-the-connection) : confirmer qu'elle fonctionne avant de persister quoi que ce soit
* [Configurer chaque surface](#configure-each-surface) : si vous utilisez une surface autre que la CLI Claude Code, comme VS Code, voir comment la configurer avec vos identifiants de passerelle
* [Configuration supplémentaire](#additional-configuration) : variables que certaines passerelles nécessitent au-delà de l'URL de base et de l'identifiant, comme un en-tête personnalisé, un assistant d'identifiant, la découverte de modèle, une URL de base au format fournisseur, ou désactiver le trafic en dehors du chemin de la passerelle. Définissez-les uniquement si votre administrateur les a nommées ou si votre réseau restreint la sortie

<h3 id="set-the-credential-variable">
  Définir la variable d'identifiant
</h3>

Pour authentifier Claude Code auprès de la passerelle, définissez votre identifiant dans une variable d'environnement. Quelle variable dépend de ce que votre équipe de passerelle vous a dit :

| Définir l'identifiant dans                              | Utiliser quand                                                                |
| :------------------------------------------------------ | :---------------------------------------------------------------------------- |
| `ANTHROPIC_AUTH_TOKEN`                                  | Votre équipe de passerelle a dit « bearer token » ou « Authorization header » |
| `ANTHROPIC_API_KEY`                                     | Votre équipe de passerelle a dit « API key » ou « x-api-key »                 |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | L'identifiant tourne ou provient d'un coffre-fort                             |

Si vous n'avez pas été informé du type, utilisez `ANTHROPIC_AUTH_TOKEN` ; la [demande de vérification](#verify-the-connection) ci-dessous montre comment savoir si vous devez basculer.

<h3 id="set-the-base-url-and-credential">
  Définir l'URL de base et l'identifiant
</h3>

Définissez l'URL de base de la passerelle et la variable d'identifiant que vous avez choisie ci-dessus comme variables d'environnement. Les exemples utilisent `ANTHROPIC_AUTH_TOKEN` ; remplacez-le par `ANTHROPIC_API_KEY` si c'est [la variable que vous avez choisie](#set-the-credential-variable). Vous pouvez les définir [dans votre shell](#set-as-shell-environment-variables), ce qui dure une session de terminal, ou [dans un fichier de paramètres Claude Code](#set-in-a-settings-file), ce qui persiste partout où Claude Code s'exécute.

Pour votre première connexion, commencez par les exports shell et exécutez la [demande de vérification](#verify-the-connection) avant de déplacer les valeurs vers un fichier de paramètres.

<h4 id="set-as-shell-environment-variables">
  Définir comme variables d'environnement shell
</h4>

Remplacez les valeurs par celles que votre équipe de passerelle vous a données :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
    ```
  </Tab>
</Tabs>

Les exports shell s'appliquent uniquement à cette session de terminal et aux programmes lancés à partir de celle-ci. Un éditeur lancé depuis le dock ou le menu Démarrer ne les verra pas. Pour les faire persister dans les nouveaux terminaux, ajoutez les mêmes lignes à votre profil shell, comme `~/.zshrc`, `~/.bashrc`, ou votre `$PROFILE` PowerShell.

Si vous exportez la passerelle uniquement dans votre shell, elle n'atteint pas de manière fiable les agents d'arrière-plan hébergés par le [superviseur](/docs/fr/agent-view#how-background-sessions-are-hosted) ; voir [comment chaque session d'arrière-plan récupère sa passerelle](/docs/fr/agent-view#llm-gateway). Utilisez un fichier de paramètres pour toute passerelle que les agents d'arrière-plan doivent toujours router.

<h4 id="set-in-a-settings-file">
  Définir dans un fichier de paramètres
</h4>

Pour que la configuration s'applique partout où Claude Code s'exécute, y compris [les agents d'arrière-plan](/docs/fr/agent-view#how-background-sessions-are-hosted), définissez les variables dans le bloc `env` d'un [fichier de paramètres](/docs/fr/settings) au lieu de vous fier à votre shell. Les fichiers de paramètres ont des portées différentes :

* `~/.claude/settings.json` s'applique à tous vos projets. Sur Windows, le chemin est `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` s'applique à un projet. Claude Code l'ajoute à votre gitignore global quand il enregistre un paramètre là ; si vous le créez à la main ou faites écrire par Claude, ajoutez-le à votre gitignore vous-même d'abord pour ne pas accidentellement valider votre identifiant

<Warning>
  Ne mettez pas l'identifiant dans le `.claude/settings.json` d'un projet. Ce fichier est validé et partagé avec tous ceux qui clonent le référentiel.
</Warning>

Le bloc `env` ressemble au même dans l'un ou l'autre fichier :

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

Quand un export shell et un bloc `env` de fichier de paramètres définissent la même variable, la valeur du fichier de paramètres s'applique. Exécutez `/status` pour voir quelle URL de base et source d'identifiant Claude Code utilise.

<h3 id="verify-the-connection">
  Vérifier la connexion
</h3>

Avec les variables exportées dans votre shell, envoyez une demande d'un jeton à la passerelle directement. Cela confirme que l'URL et l'identifiant fonctionnent avant d'ouvrir Claude Code, de sorte qu'une défaillance pointe vers la passerelle plutôt que votre configuration. Les commandes ci-dessous lisent les variables shell, elles ont donc besoin des [exports shell](#set-as-shell-environment-variables) même si vous mettez aussi les valeurs dans un fichier de paramètres.

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    curl -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
      -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
      -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

Si votre passerelle s'attend à des clés dans l'en-tête `x-api-key`, remplacez l'en-tête `Authorization` par `x-api-key: $ANTHROPIC_API_KEY` dans la commande Bash, ou l'entrée de table de hachage `"Authorization"` par `"x-api-key" = "$env:ANTHROPIC_API_KEY"` dans la commande PowerShell.

Une réponse JSON qui commence par `{"id":"msg_` et inclut un champ `"content":[...]` signifie que la passerelle est accessible et l'identifiant fonctionne. Une erreur nommant un modèle inconnu prouve toujours que l'URL et l'identifiant fonctionnent, puisque la passerelle a authentifié la demande avant de rejeter le nom du modèle ; vous n'avez pas besoin de trouver un modèle que votre passerelle sert pour ce test. Un `401` signifie que l'identifiant a été rejeté : si vous avez deviné la variable, basculez vers l'autre et réexportez.

<h4 id="confirm-in-claude-code">
  Confirmer dans Claude Code
</h4>

Démarrez `claude` à partir du même shell pour qu'il hérite des exports, envoyez un message, et exécutez `/status`.

Sur l'onglet **Status**, la ligne `Anthropic base URL` devrait afficher votre adresse de passerelle, ce qui confirme que les demandes y sont routées ; si la ligne n'est pas là, la variable n'a pas atteint la session. Une ligne `Auth token` ou `API key` nommant la variable que vous avez définie confirme que l'identifiant de passerelle est actif plutôt qu'une connexion claude.ai sauvegardée.

Si le message échoue, ou `/status` n'affiche pas l'URL de la passerelle, voir le [tableau de dépannage](#troubleshoot-gateway-errors) ci-dessous.

<h3 id="how-the-credential-variable-maps-to-a-header">
  Comment la variable d'identifiant mappe à un en-tête
</h3>

Chaque variable envoie l'identifiant dans un en-tête HTTP différent : `ANTHROPIC_AUTH_TOKEN` dans `Authorization: Bearer`, `ANTHROPIC_API_KEY` dans `x-api-key`, et `apiKeyHelper` dans les deux. Un identifiant dans la mauvaise variable atteint la passerelle dans un en-tête qu'elle ne lit pas, et la demande échoue avec `401`. Si la demande de vérification a retourné `401`, basculez vers l'autre variable et réessayez.

<h3 id="conflicts-with-an-existing-login">
  Conflits avec une connexion existante
</h3>

Une variable d'identifiant de passerelle prend précédence sur une connexion claude.ai sauvegardée ou une clé Console. Votre connexion claude.ai reste sauvegardée et inutilisée tandis que la variable est définie ; désactivez la variable et Claude Code revient à elle. Avec `ANTHROPIC_AUTH_TOKEN`, la variable prend précédence immédiatement. Avec `ANTHROPIC_API_KEY`, vous êtes invité une fois en mode interactif à approuver la clé avant qu'elle ne prenne le contrôle.

Exécutez `/status` pour confirmer quelle source d'identifiant est active. Si le démarrage affiche un avertissement de conflit d'authentification nommant deux sources, voir la première ligne du [tableau de dépannage](#troubleshoot-gateway-errors) pour savoir laquelle supprimer. Pour effacer une connexion sauvegardée afin que seul l'identifiant de passerelle reste, exécutez `/logout`.

<h2 id="configure-each-surface">
  Configurer chaque surface
</h2>

La CLI lit les variables d'environnement et les fichiers de paramètres ci-dessus. Les autres surfaces sont l'extension VS Code, l'application de bureau, GitHub Actions, l'Agent SDK, et les surfaces cloud comme Slack et le web ; les sections ci-dessous couvrent si ces paramètres atteignent chacun.

<h3 id="vs-code-extension">
  Extension VS Code
</h3>

Définissez les variables de passerelle pour l'[extension VS Code](/docs/fr/vs-code) dans `claudeCode.environmentVariables`, dans les propres paramètres utilisateur de VS Code ouverts avec la commande **Preferences: Open User Settings (JSON)**. L'extension vérifie les identifiants de ce paramètre avant de lancer, c'est donc l'endroit fiable pour l'identifiant de passerelle ; les valeurs dans `~/.claude/settings.json` atteignent le processus généré mais pas la vérification de connexion propre de l'extension.

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

<h3 id="desktop-app">
  Application de bureau
</h3>

L'application de bureau lit le routage de passerelle à partir de sa [configuration d'inférence tierce](https://claude.com/docs/third-party/claude-desktop/gateway), pas à partir de `ANTHROPIC_BASE_URL` ou `settings.json`. Cette configuration peut provenir de votre organisation ou d'un formulaire dans l'application elle-même :

* **Distribuée par un administrateur** : si votre organisation a [déployé la configuration](/docs/fr/llm-gateway-rollout#distribute-through-managed-settings), l'application de bureau route via la passerelle sans configuration de votre côté
* **Configurée localement** : pour les appareils sans configuration distribuée par un administrateur, ouvrez Help → Troubleshooting → Enable Developer Mode, qui redémarre l'application avec un menu Developer. Ensuite, ouvrez Developer → Configure Third-Party Inference et entrez l'URL de base de votre passerelle. Une configuration distribuée par un administrateur a la priorité et rend ce formulaire en lecture seule

Avec la configuration de passerelle active, l'application de bureau exécute les sessions sur votre machine locale uniquement : le sélecteur d'environnement n'offre pas de sessions SSH ou d'environnements cloud hébergés par Anthropic, et [Remote Control](/docs/fr/remote-control) n'est pas disponible. Pour utiliser Claude Code sur un hôte distant via la passerelle, exécutez la CLI sur cet hôte avec [`ANTHROPIC_BASE_URL` et l'identifiant de passerelle](#set-the-base-url-and-credential) définis là.

Si l'application de bureau affiche `Gateway was unreachable`, l'application n'a pas pu atteindre l'URL de base configurée au démarrage ; vérifiez l'URL et le chemin réseau avec le [test curl ci-dessus](#verify-the-connection).

<h3 id="github-actions">
  GitHub Actions
</h3>

[Claude Code GitHub Actions](/docs/fr/github-actions) lit `ANTHROPIC_BASE_URL` et `ANTHROPIC_CUSTOM_HEADERS` à partir du bloc `env` du workflow. Passez l'identifiant comme entrée `anthropic_api_key` de l'action ; l'action le définit comme `ANTHROPIC_API_KEY`, de sorte qu'il atteint la passerelle dans l'en-tête `x-api-key`.

Pour une passerelle `x-api-key`, définissez l'URL de base dans `env` et passez la clé de passerelle comme entrée :

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Pour une passerelle bearer-token, passez le même secret deux fois : comme entrée `anthropic_api_key` et comme `ANTHROPIC_AUTH_TOKEN` dans le bloc `env` du workflow. L'action nécessite `anthropic_api_key`, `CLAUDE_CODE_OAUTH_TOKEN`, ou la fédération d'identité de charge de travail avant de lancer Claude Code, et elle ne lit pas `ANTHROPIC_AUTH_TOKEN`, de sorte que l'entrée est là uniquement pour satisfaire cette vérification de lancement. La variable d'environnement est ce qui met la clé dans l'en-tête `Authorization` que la passerelle lit ; la copie dans `x-api-key` est ignorée :

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Pour les autres options d'authentification de l'action, y compris `CLAUDE_CODE_OAUTH_TOKEN` et la fédération d'identité de charge de travail, voir [Claude Code GitHub Actions](/docs/fr/github-actions) et le [README](https://github.com/anthropics/claude-code-action#readme) de l'action.

<h3 id="agent-sdk">
  Agent SDK
</h3>

L'[Agent SDK](/docs/fr/agent-sdk/overview) n'a pas d'options spécifiques à la passerelle ; il transmet les variables d'environnement au processus Claude Code qu'il génère. Chaque SDK accepte une option `env` qui définit l'environnement du processus généré, et les SDK TypeScript et Python le traitent différemment :

* TypeScript : le processus généré hérite de l'environnement parent par défaut, mais la définition de `options.env` remplace entièrement l'environnement. Propagez `process.env` dedans pour conserver vos variables de passerelle.
* Python : `ClaudeAgentOptions(env=...)` fusionne au-dessus de l'environnement hérité, de sorte que les variables de passerelle définies dans le processus parent se transmettent sans propagation.

<CodeGroup>
  ```ts TypeScript theme={null}
  const result = query({
    prompt: "...",
    options: {
      env: {
        ...process.env,
        ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
        ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
      },
    },
  })
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
          "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
      }
  )
  ```
</CodeGroup>

<h3 id="slack-cloud-sessions-and-remote-control">
  Slack, sessions cloud et Remote Control
</h3>

[Claude Code dans Slack](/docs/fr/slack) et [sessions cloud](/docs/fr/claude-code-on-the-web) utilisent toujours l'API d'Anthropic ; ils ne font pas partie d'un déploiement de passerelle. Les variables de passerelle définies dans la configuration d'environnement d'une session cloud ne sont pas appliquées. Si votre trafic doit rester sur la passerelle, n'activez pas ces surfaces pour ces utilisateurs.

[Remote Control](/docs/fr/remote-control) et [la dictée vocale](/docs/fr/voice-dictation) dépendent tous deux d'une identité claude.ai : Remote Control pour appairer une session en direct avec votre compte, et la dictée vocale pour atteindre le point de terminaison de transcription claude.ai. Ils ne sont pas disponibles tandis que `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, ou un `apiKeyHelper` est actif. Remote Control est également désactivé tandis que `ANTHROPIC_BASE_URL` pointe vers un hôte non-Anthropic, de sorte que la connexion avec claude.ai n'est pas suffisante en soi. Avant la v2.1.196, une URL de base non-Anthropic ne bloquait pas Remote Control.

Pour restaurer l'une ou l'autre fonctionnalité, connectez-vous avec claude.ai et désactivez les variables de passerelle que la fonctionnalité vérifie. La section Remote Control de `claude doctor` nomme ce qui bloque actuellement Remote Control.

* Dictée vocale : désactivez l'identifiant de passerelle
* Remote Control : désactivez l'identifiant de passerelle et `ANTHROPIC_BASE_URL`

<h2 id="additional-configuration">
  Configuration supplémentaire
</h2>

Ces paramètres couvrent les cas au-delà de l'URL de base et de l'identifiant. Définissez-les uniquement si les instructions de votre administrateur, les règles de sortie de votre réseau, ou le [tableau de dépannage](#troubleshoot-gateway-errors) en appellent un.

<h3 id="send-additional-headers">
  Envoyer des en-têtes supplémentaires
</h3>

Certaines passerelles routent ou balisent les demandes en utilisant un en-tête personnalisé en plus de l'identifiant, par exemple un identifiant de locataire ou une clé de routage. Pour en envoyer un, définissez [`ANTHROPIC_CUSTOM_HEADERS`](/docs/fr/env-vars) avec une paire `Name: Value` par ligne. L'exemple ci-dessous ajoute un en-tête de routage nommé `X-Org-Route` :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
    ```
  </Tab>
</Tabs>

Vous pouvez aussi définir `ANTHROPIC_CUSTOM_HEADERS` dans le bloc `env` d'un fichier de paramètres. Utilisez `\n` entre les paires là, puisque les chaînes JSON ne peuvent pas s'étendre sur plusieurs lignes :

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

Les noms d'en-têtes de routage et de locataire comme ceux-ci comptent comme des [en-têtes qui nécessitent une approbation](/docs/fr/server-managed-settings#environment-variables-and-the-approval-dialog). Quand les en-têtes proviennent d'un fichier de paramètres de projet, Claude Code les applique selon les [règles pour quand il applique les valeurs `env`](/docs/fr/settings-reference#when-claude-code-applies-env-values).

<h3 id="add-gateway-models-to-the-model-picker">
  Ajouter des modèles de passerelle au sélecteur de modèle
</h3>

Avec la découverte de modèle activée, Claude Code interroge la passerelle pour sa liste de modèles au démarrage et ajoute ces noms au sélecteur `/model` à côté des entrées intégrées. Si vous ou votre administrateur définissez `replaceBuiltInOptions` dans une lignée [`modelPicker`](/docs/fr/settings-reference#modelpicker), Claude Code masque aussi les noms découverts. Il conserve une ligne pour le modèle que la session utilise déjà.

Activez-la si votre passerelle sert des noms de modèle qui ne sont pas dans la liste intégrée de Claude Code et que vous voulez les sélectionner à partir du sélecteur. Si les modèles intégrés sont ce que vous utilisez, vous n'avez pas besoin de découverte ; votre administrateur peut aussi l'avoir déjà activée via les paramètres gérés.

Pour l'activer, définissez `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` dans votre shell ou dans le bloc `env` de `~/.claude/settings.json`.

Les modèles découverts apparaissent comme des entrées `/model` supplémentaires. Chaque entrée affiche la description que votre passerelle fournit pour le modèle, ou `From gateway` quand elle n'en fournit pas.

Pour confirmer que la découverte a fonctionné, démarrez `claude --debug` et cherchez les lignes `[gatewayDiscovery]` dans le journal de débogage à `~/.claude/debug/<session-id>.txt`. La première fois que la découverte réussit, Claude Code enregistre combien de modèles il a mis en cache, et il enregistre à nouveau uniquement quand la liste de la passerelle change. Un `404`, un délai d'attente, ou une redirection y apparaît aussi. Pour quand la découverte s'exécute, ce qu'elle filtre, et le format de réponse que les passerelles servent, voir la [référence de découverte de modèle](/docs/fr/llm-gateway-protocol#model-discovery).

<h3 id="rotate-credentials-with-apikeyhelper">
  Faire tourner les identifiants avec apiKeyHelper
</h3>

Un `apiKeyHelper` est une commande que Claude Code exécute pour récupérer votre identifiant de passerelle, au lieu de le lire à partir d'une variable d'environnement statique.

Utilisez un assistant quand l'identifiant expire selon un calendrier, provient d'une commande de coffre-fort ou SSO, ou votre administrateur vous a dit de configurer un. Si votre identifiant est une chaîne fixe que vous définissez une fois, la [variable d'identifiant](#set-the-credential-variable) est tout ce dont vous avez besoin et vous pouvez ignorer cette section.

L'assistant est n'importe quelle commande shell qui imprime l'identifiant actuel sur stdout. Claude Code l'exécute via votre shell système, donc sur Windows il peut être un exécutable ou une invocation PowerShell. La commande ne doit rien imprimer d'autre que l'identifiant. Sur Claude Code v2.1.227 ou ultérieur, une bannière ou une ligne de journal imprimée à côté de la clé fait [échouer l'assistant](/docs/fr/errors#your-apikeyhelper-script-is-failing). Écrivez le script, rendez-le exécutable, et référencez-le à partir de `apiKeyHelper` dans votre [fichier de paramètres](/docs/fr/settings) :

<Tabs>
  <Tab title="Bash ou Zsh">
    Par exemple, un script qui lit à partir d'un coffre-fort :

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Référencez son chemin dans `~/.claude/settings.json` :

    ```json theme={null}
    {
      "apiKeyHelper": "~/bin/get-gateway-key.sh"
    }
    ```
  </Tab>

  <Tab title="PowerShell">
    Par exemple, un script qui lit à partir d'un coffre-fort :

    ```powershell theme={null}
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Référencez l'invocation PowerShell dans `%USERPROFILE%\.claude\settings.json`, en échappant les barres obliques inverses dans la chaîne JSON :

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code met en cache la sortie de l'assistant pendant cinq minutes par défaut et réexécute l'assistant après l'expiration de la durée de vie du cache. Pour changer la durée de vie, définissez `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` en millisecondes, par exemple `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` pour 15 minutes.

Voir [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) pour les autres cas dans lesquels Claude Code réexécute l'assistant.

La valeur de l'assistant est envoyée dans les en-têtes `Authorization` et `x-api-key`, de sorte qu'elle fonctionne quel que soit l'en-tête que votre passerelle lit.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  Désactiver le trafic en dehors du chemin de la passerelle
</h3>

La passerelle achemine les demandes de modèle, mais Claude Code envoie aussi du trafic de fond non essentiel en dehors du chemin de la passerelle, vers Anthropic et vers des services tiers tels que GitHub : vérifications de version, télémétrie, notes de version, et demandes similaires. Sur un réseau qui n'autorise la sortie que vers la passerelle, ces demandes échouent et peuvent apparaître comme des connexions bloquées dans votre surveillance de sortie.

Claude Code attache un identifiant à une demande de télémétrie ou de métriques d'utilisation uniquement quand la demande va vers l'hôte auquel l'identifiant appartient. Tandis que `ANTHROPIC_BASE_URL` pointe vers la passerelle, Claude Code envoie ses événements de télémétrie à Anthropic sans votre identifiant de passerelle. Avec une [variable d'identifiant](#set-the-credential-variable) ou `apiKeyHelper` aussi actif, Claude Code ne signale pas les métriques d'utilisation au tableau de bord [analytique](/docs/fr/analytics#access-analytics-for-api-customers) de la Console. Avant v2.1.246, Claude Code pouvait attacher l'identifiant de passerelle aux demandes de télémétrie et de métriques d'utilisation destinées aux hôtes Anthropic ; les demandes de modèle allaient toujours à la passerelle avec l'identifiant que la passerelle attend.

Pour désactiver ce trafic, définissez `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` à côté des variables de passerelle, dans les mêmes exports shell ou bloc `env` du fichier de paramètres :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
    ```
  </Tab>
</Tabs>

Définir la variable a ces effets et limites :

* Elle désactive les mises à jour automatiques, donc prévoyez un autre chemin de mise à jour, tel que votre gestionnaire de paquets ou distribution gérée.
* Elle supprime la vérification de disponibilité du [mode rapide](/docs/fr/fast-mode). À moins qu'une vérification précédente n'ait déjà activé le mode rapide sur la machine, `/fast` signale que le mode rapide est indisponible.
* Elle n'affecte pas la [découverte de modèle de passerelle](#add-gateway-models-to-the-model-picker), qui interroge uniquement votre passerelle. Avant v2.1.257, la variable arrêtait aussi la découverte de s'actualiser, de sorte que le sélecteur conservait la liste précédemment mise en cache.
* La vérification de sécurité du domaine de l'outil WebFetch n'est pas affectée et appelle toujours `api.anthropic.com`. Désactivez-la séparément avec `skipWebFetchPreflight: true` dans les [paramètres](/docs/fr/settings) si votre réseau bloque cet hôte. Voir la [vérification de sécurité du domaine WebFetch](/docs/fr/data-usage#webfetch-domain-safety-check).
* Pour chaque flux de télémétrie et la variable qui le contrôle, voir [services de télémétrie](/docs/fr/data-usage#telemetry-services).

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  Router vers un fournisseur cloud via une passerelle
</h3>

Ces configurations pointent Claude Code vers une passerelle via une variable d'URL de base spécifique au fournisseur à la place de `ANTHROPIC_BASE_URL`. Les passerelles Amazon Bedrock et Google Cloud's Agent Platform acceptent les formats de demande natifs de ces fournisseurs ; les passerelles Microsoft Foundry et Claude Platform sur AWS acceptent le format Anthropic Messages. Sur les routes Amazon Bedrock et Google Cloud's Agent Platform, Claude Code limite aussi les en-têtes bêta et les champs de demande qu'il envoie à l'ensemble que le fournisseur accepte. Pour ce que votre passerelle reçoit sur chaque route, voir le [guide de compatibilité de passerelle](/docs/fr/llm-gateway-protocol).

Utilisez-en une uniquement si votre équipe de passerelle a spécifiquement nommé Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou Claude Platform sur AWS. Si la [demande de vérification](#verify-the-connection) ci-dessus a retourné JSON, vous pouvez ignorer cette section.

Définissez le bloc pour le fournisseur que votre équipe de passerelle a nommé. Les variables skip-auth dans les blocs Amazon Bedrock, Google Cloud's Agent Platform, et Claude Platform sur AWS disent à Claude Code de ne pas signer les demandes avec les identifiants du fournisseur cloud, puisque la passerelle les détient. Si la passerelle a besoin de son propre jeton, où vous le mettez dépend du fournisseur :

* **Amazon Bedrock, Google Cloud's Agent Platform, ou Claude Platform sur AWS** : ajoutez `ANTHROPIC_AUTH_TOKEN` après le bloc. Claude Code l'envoie à la passerelle comme en-tête `Authorization: Bearer`. Pour un identifiant dans un schéma ou un en-tête différent, utilisez [`ANTHROPIC_CUSTOM_HEADERS`](#send-additional-headers) à la place. Gardez la variable skip-auth définie de toute façon, puisque sans elle Claude Code supprime tout en-tête `Authorization` que `ANTHROPIC_AUTH_TOKEN`, un [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), ou `ANTHROPIC_CUSTOM_HEADERS` ajouterait.
* **Microsoft Foundry** : utilisez `ANTHROPIC_FOUNDRY_API_KEY` comme [son bloc](#microsoft-foundry) le montre

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Laissez `AWS_BEARER_TOKEN_BEDROCK` non défini quand la passerelle émet son propre identifiant. Si vous le définissez, Claude Code envoie cette [clé API Amazon Bedrock](/docs/fr/amazon-bedrock#2-configure-aws-credentials) comme en-tête `Authorization` à la place de votre jeton de passerelle, même avec `CLAUDE_CODE_SKIP_BEDROCK_AUTH` défini.

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
    export CLAUDE_CODE_USE_BEDROCK=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
    $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
    $env:CLAUDE_CODE_USE_BEDROCK = "1"
    ```
  </Tab>
</Tabs>

<h4 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h4>

Remplacez l'ID de projet et la région par vos propres valeurs. Claude Code inclut les deux dans le chemin de chaque demande qu'il envoie à la passerelle :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
    $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
    $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
    $env:CLAUDE_CODE_USE_VERTEX = "1"
    $env:CLOUD_ML_REGION = "us-east5"
    ```
  </Tab>
</Tabs>

Le bloc couvre le routage et l'authentification. Les remplacements de région et les épingles de modèle de la [configuration Agent Platform](/docs/fr/google-vertex-ai#4-configure-claude-code) s'appliquent aussi via une passerelle :

* **Régions par modèle** : si votre passerelle sert certains modèles à partir d'une région autre que `CLOUD_ML_REGION`, définissez la variable `VERTEX_REGION_CLAUDE_*` correspondante pour chacun, par exemple `VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1`. La [référence des variables d'environnement](/docs/fr/env-vars) liste les noms exacts.
* **Versions de modèle** : épinglez `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, et `ANTHROPIC_DEFAULT_HAIKU_MODEL` comme dans [Épingler les versions de modèle](/docs/fr/google-vertex-ai#5-pin-model-versions). Définir `ANTHROPIC_DEFAULT_HAIKU_MODEL` déplace aussi les tâches de fond telles que les titres de session vers ce modèle, et cette section explique quel modèle les exécute autrement.
* **Capacités de modèle** : si vous épinglez un ID de modèle que votre version de Claude Code ne reconnaît pas, les fonctionnalités telles que les niveaux d'effort ou la réflexion étendue peuvent rester désactivées sur celui-ci. Déclarez ce que le modèle supporte avec [`ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES`](/docs/fr/model-config#customize-pinned-model-display-and-capabilities) et ses homologues Sonnet et Haiku.

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Mettez l'identifiant de la passerelle dans `ANTHROPIC_FOUNDRY_API_KEY` ; il est envoyé à la passerelle comme en-tête `x-api-key`. Une passerelle qui s'attend à un jeton porteur peut prendre [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/fr/env-vars) à la place. Claude Code envoie cette valeur comme en-tête `Authorization: Bearer`, et elle prend la priorité sur `ANTHROPIC_FOUNDRY_API_KEY` quand les deux sont définis. Nécessite Claude Code v2.1.203 ou ultérieur.

Pour une passerelle qui injecte son propre en-tête `Authorization`, définissez `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` et laissez les deux variables d'identifiant non définies. Claude Code envoie alors les demandes sans identifiant Azure et préserve l'en-tête `Authorization` que vous fournissez, par exemple via `ANTHROPIC_CUSTOM_HEADERS`. Avant v2.1.203, `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` sans clé API laissait le client Microsoft Foundry incapable d'envoyer des demandes.

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
    export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
    export CLAUDE_CODE_USE_FOUNDRY=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
    $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
    $env:CLAUDE_CODE_USE_FOUNDRY = "1"
    ```
  </Tab>
</Tabs>

<h4 id="claude-platform-on-aws">
  Claude Platform sur AWS
</h4>

Voir [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws) pour l'ID d'espace de travail.

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
    export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
    export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
    export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
    $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
    $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
    $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
    ```
  </Tab>
</Tabs>

<h4 id="confirm-the-provider-route">
  Confirmer la route du fournisseur
</h4>

Démarrez `claude` à partir du shell où vous avez défini le bloc et exécutez `/status`. Avec le bloc Amazon Bedrock, l'onglet **Status** affiche des lignes comme celles-ci :

```text theme={null}
API provider: Amazon Bedrock
Bedrock base URL: https://llm-gateway.example.com/bedrock
AWS auth skipped
```

Les autres blocs produisent les mêmes lignes sous les noms de leur fournisseur, par exemple `Vertex base URL` et `GCP auth skipped` pour Google Cloud's Agent Platform ; le bloc Microsoft Foundry affiche une ligne auth skipped uniquement si vous définissez `CLAUDE_CODE_SKIP_FOUNDRY_AUTH`. Si vous routez aussi via un proxy d'entreprise, une ligne `Proxy` affiche l'URL du proxy. Si la ligne d'URL de base est manquante, la variable n'a pas atteint la session.

<h2 id="troubleshoot-gateway-errors">
  Dépanner les erreurs de passerelle
</h2>

Ce sont les erreurs les plus courantes lors de l'exécution de Claude Code via une passerelle, avec la cause côté passerelle et la correction :

| Erreur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Cause                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Correction                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Un avertissement de démarrage nommant deux sources d'identifiant et se terminant par `auth may not work as expected`. Les versions plus anciennes affichent `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` à la place.                                                                                                                                                                                                                                                                      | Un identifiant de passerelle et une connexion sauvegardée sont tous deux actifs ; la variable est utilisée pour les demandes, mais la connexion obsolète peut causer un comportement d'authentification inattendu                                                                                                                                                                                                                                                                                     | Désactivez la variable pour utiliser la connexion sauvegardée, ou exécutez `/logout` pour utiliser l'identifiant de passerelle                                                                                                                                                                                                                                                                                                                                                           |
| Erreurs `401` nommant un jeton invalide ou non reconnu                                                                                                                                                                                                                                                                                                                                                                                                                                                              | L'identifiant n'en est pas un que la passerelle a émis, ou il est dans un en-tête que la passerelle ne lit pas                                                                                                                                                                                                                                                                                                                                                                                        | Confirmez que la variable correspond à votre type d'identifiant dans le [tableau d'identifiant](#set-the-credential-variable), et régénérez la clé à la passerelle si elle a été révoquée                                                                                                                                                                                                                                                                                                |
| `Your apiKeyHelper script is failing`, ou `apiKeyHelper failed:` sur stderr en mode non interactif                                                                                                                                                                                                                                                                                                                                                                                                                  | La commande dans le paramètre [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) n'a pas produit une clé utilisable, de sorte que les demandes portent une clé d'espace réservé                                                                                                                                                                                                                                                                                                                    | Exécutez la commande directement pour voir pourquoi elle échoue, et réauthentifiez-vous auprès de votre fournisseur d'identifiants s'il signale une session expirée ; voir [la référence d'erreur](/docs/fr/errors#your-apikeyhelper-script-is-failing)                                                                                                                                                                                                                                       |
| `Connection refused — a firewall or proxy may be blocking it (ConnectionRefused)` quand rien ne répond à l'adresse, ou `Can't reach the API server — check your internet or DNS (ENOTFOUND)` quand le nom d'hôte ne se résout pas, souvent après une pause silencieuse tandis que Claude Code [réessaie avec backoff](/docs/fr/errors#automatic-retries). Le code entre parenthèses varie ; [Unable to connect to API](/docs/fr/errors#unable-to-connect-to-api) couvre les orthographes de code et la formulation antérieure | Rien n'a répondu à l'URL de base : l'adresse est mauvaise, ou un VPN ou un pare-feu bloque le chemin vers la passerelle                                                                                                                                                                                                                                                                                                                                                                               | Exécutez le [test curl ci-dessus](#verify-the-connection), qui échoue immédiatement avec la même cause, et confirmez l'URL et le chemin réseau avec votre équipe de passerelle                                                                                                                                                                                                                                                                                                           |
| `API returned an empty or malformed response (HTTP 200)`                                                                                                                                                                                                                                                                                                                                                                                                                                                            | La passerelle ou un proxy intermédiaire a retourné une réponse non-API, souvent une page d'erreur HTML ou de connexion                                                                                                                                                                                                                                                                                                                                                                                | Testez avec la [demande curl ci-dessus](#verify-the-connection) ; corrigez la route de passerelle qui retourne quelque chose d'autre qu'une réponse Claude API. [La référence d'erreur](/docs/fr/errors#api-returned-an-empty-or-malformed-response) explique le détail que le message signale                                                                                                                                                                                                |
| Erreurs `400` nommant `context_management`, `Extra inputs are not permitted`, ou d'autres champs non reconnus                                                                                                                                                                                                                                                                                                                                                                                                       | La passerelle transfère les demandes à un amont qui rejette les champs que Claude Code envoie aux points de terminaison au format Anthropic                                                                                                                                                                                                                                                                                                                                                           | Définissez `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`, qui supprime la plupart des champs de pré-version ; voir [feature pass-through](/docs/fr/llm-gateway-protocol#feature-pass-through). Certaines betas ne sont pas fermées par ce drapeau ; pour celles-ci, définissez la variable de fournisseur `CLAUDE_CODE_USE_*` correspondante de sorte que Claude Code envoie uniquement ce que ce fournisseur accepte                                                                            |
| Erreurs `400` nommant `thinking` ou `adaptive`, comme `Input tag 'adaptive' found`                                                                                                                                                                                                                                                                                                                                                                                                                                  | La version du modèle en amont n'accepte pas le raisonnement adaptatif, que Claude Code demande pour les modèles Claude 4.6 et ultérieurs                                                                                                                                                                                                                                                                                                                                                              | Mettez à niveau l'amont de la passerelle. Sur Opus 4.6 et Sonnet 4.6, `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` fonctionne à la place. Les variables de capacité de [configuration de modèle](/docs/fr/model-config) s'appliquent uniquement aux configurations de fournisseur, comme `CLAUDE_CODE_USE_BEDROCK` et `CLAUDE_CODE_USE_VERTEX`, pas derrière une passerelle `ANTHROPIC_BASE_URL`                                                                                                 |
| Erreurs `400` indiquant une limite de contexte ou de jeton dans les propres termes de la passerelle, comme `ContextWindowExceededError` ou `prompt token count of N exceeds the limit of M`                                                                                                                                                                                                                                                                                                                         | La passerelle applique une fenêtre de contexte plus petite que celle native du modèle et réécrit l'erreur en amont, de sorte que Claude Code ne la reconnaît pas comme une [erreur trop longue](/docs/fr/errors#prompt-is-too-long) et ne compacte pas et ne réessaie pas automatiquement                                                                                                                                                                                                                  | Exécutez `/compact` pour récupérer la session. Pour l'éviter, définissez `CLAUDE_CODE_AUTO_COMPACT_WINDOW` à la limite de la passerelle ; Claude Code limite la valeur à au moins 100 000 jetons et au maximum la fenêtre de contexte du modèle, de sorte qu'une limite de passerelle inférieure à 100 000 ne peut pas être appariée, et `/compact` reste la récupération là. Définissez aussi `CLAUDE_CODE_MAX_OUTPUT_TOKENS` en dessous de la limite de sortie du modèle de passerelle |
| `400` erreurs sur chaque demande, dans les propres termes de la passerelle rejetant un schéma d'entrée d'outil ou son `pattern`, sur Claude Code v2.1.265 à v2.1.267                                                                                                                                                                                                                                                                                                                                                | Dans un déploiement progressif sur ces versions, le schéma de l'[outil Artifact](/docs/fr/artifacts#availability) porte une expression régulière avec des classes de caractères Unicode `\p{...}`. L'API Anthropic l'accepte, mais une passerelle ou un amont qui vérifie le `pattern` de chaque schéma d'outil avec son propre moteur regex rejette la demande entière                                                                                                                                    | Mettez à jour vers v2.1.268 ou ultérieur, qui n'envoie pas l'expression régulière. Sur une version affectée, [désactivez les artifacts](/docs/fr/artifacts#disable-artifacts), ce qui supprime l'outil et son schéma des demandes                                                                                                                                                                                                                                                             |
| `400` erreurs sur chaque demande, dans les propres termes de la passerelle rejetant un type d'outil non reconnu, comme `Input tag 'advisor_20260301'`, sur Claude Code v2.1.275                                                                                                                                                                                                                                                                                                                                     | Dans un déploiement progressif sur cette version, les demandes portent une entrée d'[outil advisor](/docs/fr/advisor) même avec l'advisor désactivé. L'API Anthropic l'accepte, mais une passerelle ou un amont qui valide les types d'outil rejette la demande entière ; celui qui [transfère les champs du corps de la demande sans modification](/docs/fr/llm-gateway-protocol#forward-as-open-lists) la transmet sans être affecté. L'entrée est une déclaration qui ne porte aucun contenu de conversation | Mettez à jour vers v2.1.276 ou ultérieur, qui n'envoie pas l'entrée derrière une passerelle `ANTHROPIC_BASE_URL` à moins que vous n'activiez l'advisor. Sur v2.1.275, définissez [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`](/docs/fr/env-vars), qui supprime l'entrée des demandes                                                                                                                                                                                                                |
| Modèles manquants du sélecteur `/model`                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Les noms de modèle de passerelle ne sont pas dans la liste intégrée de Claude Code, ou Claude Code affiche une ligne [`modelPicker`](/docs/fr/settings-reference#modelpicker) qui remplace les options intégrées                                                                                                                                                                                                                                                                                           | Activez la [découverte de modèle de passerelle](#add-gateway-models-to-the-model-picker) ou ajoutez des noms avec les variables de [configuration de modèle](/docs/fr/model-config). Si Claude Code affiche une ligne `modelPicker` de remplacement, ajoutez les modèles de passerelle à celle-ci, ou demandez à votre administrateur de les ajouter quand les paramètres gérés le fournissent                                                                                                |
| `/fast` signale `Fast mode unavailable due to network connectivity issues` tandis que les demandes d'inférence fonctionnent                                                                                                                                                                                                                                                                                                                                                                                         | La vérification de disponibilité du [mode rapide](/docs/fr/fast-mode) va directement à `api.anthropic.com` et ne suit pas `ANTHROPIC_BASE_URL`, de sorte que l'accès direct sortant bloqué échoue à la vérification. Le même message apparaît sur un réseau ouvert quand la vérification présente une clé émise par passerelle de `ANTHROPIC_API_KEY` ou d'un `apiKeyHelper` et Anthropic la rejette                                                                                                       | Mettez en liste blanche `api.anthropic.com` si l'accès sortant est bloqué, ou définissez une variable de saut ; pour une clé de passerelle rejetée, seules les variables de saut aident. Voir [utiliser le mode rapide derrière les proxies et les passerelles LLM](/docs/fr/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                                                         |
| `/fast` signale `Fast mode has been disabled by your organization` dans une session authentifiée avec `ANTHROPIC_AUTH_TOKEN`, même si l'organisation a le mode rapide activé                                                                                                                                                                                                                                                                                                                                        | La vérification de disponibilité nécessite une connexion claude.ai ou une clé API Anthropic ; avec seulement un jeton porteur, Claude Code traite le mode rapide comme désactivé sans envoyer la vérification                                                                                                                                                                                                                                                                                         | Définissez `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` ; voir [utiliser le mode rapide derrière les proxies et les passerelles LLM](/docs/fr/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                                                                                                                                                                                            |
| Claude Code vous demande de vous connecter même si le [test curl](#verify-the-connection) réussit                                                                                                                                                                                                                                                                                                                                                                                                                   | La CLI n'a pas d'identifiant propre : une URL de base accessible n'en est pas une, et dans une session interactive un bloc `env` dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet s'applique uniquement après l'assistant de première exécution et l'[invite de confiance](/docs/fr/permissions#what-runs-before-you-trust-a-folder)                                                                                                                                           | Définissez `ANTHROPIC_AUTH_TOKEN` quelque part que Claude Code lit avant la configuration de première exécution : un export shell, le bloc `env` dans `~/.claude/settings.json`, ou les paramètres gérés                                                                                                                                                                                                                                                                                 |
| `ANTHROPIC_API_KEY` est défini mais ignoré, sans invite                                                                                                                                                                                                                                                                                                                                                                                                                                                             | La clé a besoin d'une approbation unique dans les sessions interactives, et une clé précédemment refusée est ignorée sans demander à nouveau                                                                                                                                                                                                                                                                                                                                                          | Activez-la sous `/config` avec l'option `Use custom API key`                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `This machine's managed settings require a first-party login`                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Les paramètres gérés incluent `forceLoginMethod` ou `forceLoginOrgUUID`, qui ne peuvent pas coexister avec `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, ou `apiKeyHelper`                                                                                                                                                                                                                                                                                                                             | Votre administrateur doit supprimer `forceLoginMethod` et `forceLoginOrgUUID` des paramètres gérés pour utiliser les identifiants de passerelle, ou supprimer l'identifiant de passerelle pour utiliser la connexion de première partie. Les deux ne peuvent pas être combinés                                                                                                                                                                                                           |
| `403` avec un corps HTML comme `403 Forbidden`, quand les propres journaux de la passerelle ne montrent aucune demande reçue                                                                                                                                                                                                                                                                                                                                                                                        | Un pare-feu d'application web ou un proxy inverse devant la passerelle a bloqué le corps de la demande avant qu'il n'atteigne la passerelle. Les messages Claude Code incluent des balises de style XML et du code source qui correspondent aux règles de corps de cross-site-scripting, de sorte qu'un test curl court réussit tandis qu'une session réelle ne le fait pas                                                                                                                           | Exemptez le chemin `/v1/messages` de la passerelle de l'inspection du corps de la demande. Sur AWS WAF, c'est la règle gérée `CrossSiteScripting_Body` ; sur nginx avec ModSecurity, ce sont les règles de corps OWASP CRS équivalentes                                                                                                                                                                                                                                                  |
| Erreurs de certificat ou TLS comme `SSL certificate verification failed` ou `Self-signed certificate detected`, quand le [test curl](#verify-the-connection) réussit                                                                                                                                                                                                                                                                                                                                                | L'exécution de Claude Code ne fait pas confiance à la même autorité de certification que `curl` utilise. Courant derrière les proxies d'inspection TLS d'entreprise                                                                                                                                                                                                                                                                                                                                   | Définissez `NODE_EXTRA_CA_CERTS` au chemin du bundle CA ; voir [CA certificate store](/docs/fr/network-config#ca-certificate-store)                                                                                                                                                                                                                                                                                                                                                           |

Si Claude Code vous demande de vous connecter à plusieurs reprises après la suppression de la configuration de passerelle, la cause est généralement le stockage d'identifiants plutôt que la passerelle ; voir [erreurs d'authentification](/docs/fr/errors#authentication-errors).

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Aperçu des passerelles LLM](/docs/fr/llm-gateway) : ce qu'est une passerelle et comment elle interagit avec les abonnements claude.ai
* [Déployer une passerelle LLM pour votre organisation](/docs/fr/llm-gateway-rollout) : la liste de contrôle orientée administrateur pour déployer et distribuer la configuration de passerelle
* [Guide de compatibilité de passerelle](/docs/fr/llm-gateway-protocol) : ce que Claude Code envoie à une passerelle, y compris les en-têtes et les champs que la passerelle doit transférer
* [Paramètres](/docs/fr/settings) : où vivent les fichiers de paramètres et comment le bloc `env` est lu
* [Authentification](/docs/fr/authentication) : comment les variables d'identifiant, `apiKeyHelper`, et la connexion OAuth interagissent
