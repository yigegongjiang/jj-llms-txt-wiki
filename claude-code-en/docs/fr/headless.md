> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Exécuter Claude Code par programmation

> Utilisez l'Agent SDK pour exécuter Claude Code par programmation depuis la CLI, Python ou TypeScript.

L'[Agent SDK](/docs/fr/agent-sdk/overview) vous donne accès aux mêmes outils, boucle d'agent et gestion du contexte qui alimentent Claude Code. Il est disponible en tant que CLI pour les scripts et CI/CD, ou en tant que packages [Python](/docs/fr/agent-sdk/python) et [TypeScript](/docs/fr/agent-sdk/typescript) pour un contrôle programmatique complet.

Pour exécuter Claude Code en mode non interactif, passez `-p` avec votre prompt et les [options CLI](/docs/fr/cli-reference) dont vous avez besoin :

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Cette page couvre l'utilisation de l'Agent SDK via la CLI (`claude -p`). Pour les packages SDK Python et TypeScript avec sorties structurées, callbacks d'approbation d'outils et objets de message natifs, consultez la [documentation complète de l'Agent SDK](/docs/fr/agent-sdk/overview).

<h2 id="basic-usage">
  Utilisation basique
</h2>

Ajoutez le flag `-p` (ou `--print`) à n'importe quelle commande `claude` pour l'exécuter de manière non-interactive. Toutes les [options CLI](/docs/fr/cli-reference) ne se combinent pas avec `-p`. Claude Code rejette `--bg`, et rejette `--cloud` avec une description de tâche, avec une erreur nommant le conflit ; `--cloud` avec un ID de session et `-p` à la place [met en file d'attente un message dans cette session cloud](/docs/fr/claude-code-on-the-web#send-follow-ups-from-the-cli) et se termine. Les options que vous combinerez souvent avec `-p` incluent :

* `--continue` pour [continuer les conversations](#continue-conversations)
* `--allowedTools` pour [approuver automatiquement les outils](#auto-approve-tools)
* `--output-format` pour [obtenir une sortie structurée](#get-structured-output)

Cet exemple pose une question à Claude sur votre base de code et affiche la réponse :

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code se termine avec le code 0 en cas de succès et un code non-zéro quand l'exécution échoue, donc vos scripts peuvent se brancher sur le code de sortie. Si vous passez un flag invalide, Claude Code signale l'erreur sur stderr avant le démarrage de l'exécution. Quand une défaillance se produit à l'intérieur de l'exécution, comme une authentification manquante, Claude Code affiche la défaillance comme le résultat sur stdout.

<h3 id="start-faster-with-bare-mode">
  Démarrer plus rapidement avec le mode bare
</h3>

Ajoutez `--bare` pour réduire le temps de démarrage en ignorant la découverte automatique des hooks, skills, commandes personnalisées, [sous-agents](/docs/fr/sub-agents), plugins installés, serveurs MCP, mémoire automatique et CLAUDE.md. Sans cela, `claude -p` charge le même [contexte](/docs/fr/how-claude-code-works#the-context-window) qu'une session interactive, y compris tout ce qui est configuré dans le répertoire de travail ou `~/.claude`.

Le mode bare est utile pour CI et les scripts où vous avez besoin du même résultat sur chaque machine. Un hook dans le `~/.claude` d'un coéquipier ou un serveur MCP dans le `.mcp.json` du projet ne s'exécutera pas, car le mode bare ne les lit jamais. Un répertoire que vous nommez avec `--add-dir` est une exception partielle : le mode bare charge les skills de son dossier `.claude/skills/`, mais ignore toujours ses dossiers `.claude/commands/` et `.claude/agents/`. [Skills from additional directories](/docs/fr/skills#skills-from-additional-directories) couvre ce qui se charge et ce qui ne se charge pas.

Sans `--bare`, une session `-p` exécute les hooks dans le `.claude/settings.json` d'un projet et connecte les serveurs dans son `.mcp.json`, même dans un dossier que vous n'avez jamais approuvé. Une session `-p` n'affiche aucune boîte de dialogue de confiance d'espace de travail et aucune invite d'approbation par serveur. [What runs before you trust a folder](/docs/fr/permissions#what-runs-before-you-trust-a-folder) couvre chaque type de contenu de référentiel sous `-p` et comment le garder à l'écart.

Cet exemple exécute une tâche de résumé ponctuelle en mode bare et pré-approuve l'outil Read pour que l'appel se termine sans invite de permission. Définissez `ANTHROPIC_API_KEY` avant de l'exécuter, car le mode bare n'utilise pas votre connexion d'abonnement :

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

En mode bare, Claude Code ne lit jamais les identifiants OAuth ou le trousseau système. Pour l'API Anthropic, définissez `ANTHROPIC_API_KEY` dans l'environnement, avec une clé créée dans la [Claude Console](https://platform.claude.com), ou fournissez un `apiKeyHelper` dans le JSON `--settings`. Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry continuent à lire leurs propres identifiants de fournisseur comme d'habitude.

En mode bare, Claude a accès aux outils Bash, lecture de fichier et modification de fichier. Passez tout contexte dont vous avez besoin avec un flag :

| Pour charger             | Utilisez                                                |
| ------------------------ | ------------------------------------------------------- |
| Ajouts de prompt système | `--append-system-prompt`, `--append-system-prompt-file` |
| Paramètres               | `--settings <file-or-json>`                             |
| Serveurs MCP             | `--mcp-config <file-or-json>`                           |
| Agents personnalisés     | `--agents <json>`                                       |
| Un plugin                | `--plugin-dir <path>`, `--plugin-url <url>`             |

<Note>
  `--bare` est le mode recommandé pour les appels scriptés et SDK, et deviendra le mode par défaut pour `-p` dans une version future.
</Note>

<h3 id="background-tasks-at-exit">
  Tâches en arrière-plan à la sortie
</h3>

Si Claude démarre une [tâche Bash en arrière-plan](/docs/fr/tools-reference#bash-tool-behavior) lors d'une exécution `claude -p`, par exemple un serveur de développement ou une compilation en surveillance, ce shell est terminé environ cinq secondes après que Claude ait retourné son résultat final et que stdin ait été fermé. La période de grâce permet à une tâche qui se termine juste après le résultat de livrer quand même sa sortie.

Si Claude démarre un [sous-agent](/docs/fr/sub-agents) en arrière-plan ou un workflow, `claude -p` reste plutôt ouvert jusqu'à ce que ce travail se termine, car son résultat fait partie de la sortie finale.

Par défaut, l'attente se termine après 10 minutes d'attente continue inactive, donc un sous-agent ou un workflow bloqué ne peut pas maintenir le processus ouvert indéfiniment. À ce stade, Claude Code arrête tout ce qui s'exécute toujours et abandonne son résultat partiel. Pour modifier la limite, définissez [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/fr/env-vars), ou définissez-le sur `0` pour attendre sans limite.

Si Claude démarre une montre [Monitor](/docs/fr/tools-reference#monitor-tool) lors d'une exécution `claude -p`, Claude Code attend la montre jusqu'à ce qu'elle expire ou que le plafond de dix minutes termine l'attente, selon ce qui arrive en premier. Pendant qu'il attend, Claude continue à répondre à ce que la montre signale. Par défaut, une montre expire cinq minutes après que Claude la démarre.

<h3 id="stop-a-run-with-sigterm">
  Arrêter une exécution avec SIGTERM
</h3>

Si vous arrêtez une exécution `claude -p` avec SIGTERM, par exemple avec `kill` ou depuis un superviseur de processus, Claude Code se termine avec le code 143. Claude Code laisse le tour en cours inachevé et n'enregistre aucun résultat pour celui-ci. Pour terminer le tour à la place, envoyez SIGINT, ou appelez `interrupt()` du SDK Agent, avant d'arrêter le processus.

Sur SIGTERM, Claude Code termine l'arborescence des processus de toute commande Bash qui s'exécute toujours. Claude Code exécute ensuite les hooks [`SessionEnd`](/docs/fr/hooks#sessionend) et se termine. Lors de la sortie, Claude Code ne démarre aucun nouvel appel d'outil, n'envoie aucune nouvelle demande de modèle et n'exécute aucun hook autre que `SessionEnd`. Si l'exécution était au milieu d'une commande ou en attente d'une réponse à une invite de permission quand le signal est arrivé, Claude Code gère cette étape comme suit :

* **Exécution d'une commande** : Claude Code enregistre la commande comme tuée dans la session.
* **En attente d'une réponse à une invite de permission** : si vous envoyez SIGTERM au processus, Claude Code laisse l'invite sans réponse. Si votre programme ferme la session via le SDK Agent, le SDK termine l'entrée de Claude Code avant d'envoyer un signal, et Claude Code annule l'invite dès que l'entrée se termine.

Quand vous [reprenez la session](#continue-conversations), Claude Code continue le tour que SIGTERM a laissé inachevé.

<h2 id="examples">
  Exemples
</h2>

Ces exemples mettent en évidence les modèles CLI courants. Lorsqu'une commande nomme un fichier tel que `auth.py` ou `build-error.txt`, remplacez-le par un fichier de votre propre projet. Dans les environnements CI ou autres environnements scriptés, ajoutez [`--bare`](#start-faster-with-bare-mode) pour que Claude Code démarre sans charger les hooks, plugins, mémoire automatique ou `CLAUDE.md` de l'hôte.

<h3 id="pipe-data-through-claude">
  Transmettre des données via Claude
</h3>

Le mode non interactif lit stdin, vous pouvez donc transmettre des données et rediriger la réponse comme n'importe quel autre outil en ligne de commande.

Cet exemple transmet un journal de compilation à Claude et écrit l'explication dans un fichier :

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Avec `--output-format json`, la charge utile de réponse inclut `total_cost_usd` et une ventilation des coûts par modèle, afin que les appelants scriptés puissent suivre les dépenses sans consulter le [tableau de bord d'utilisation](/docs/fr/costs). Lorsque vous continuez une conversation antérieure avec `--continue` ou `--resume`, l'exécution rapporte le total de la conversation, [les dépenses des exécutions antérieures incluses](/docs/fr/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Les deux chiffres sont des [estimations côté client](/docs/fr/agent-sdk/cost-tracking) et peuvent différer de votre facture réelle.

<Note>
  L'entrée stdin transmise est limitée à 10 Mo. Si vous dépassez la limite, Claude Code se ferme avec une erreur claire et un statut non nul. Pour travailler avec des entrées plus volumineuses, écrivez le contenu dans un fichier et référencez le chemin du fichier dans votre invite au lieu de le transmettre.
</Note>

Si Claude Code ne peut pas lire stdin, par exemple parce que le processus qui l'a démarré a déconnecté son extrémité, Claude Code imprime un avertissement sur stderr et continue avec l'invite de la ligne de commande. Avant la v2.1.211, une stdin illisible sur Windows plantait la session ou la fermait silencieusement sans sortie.

<h3 id="add-claude-to-a-build-script">
  Ajouter Claude à un script de compilation
</h3>

Vous pouvez envelopper un appel non interactif dans un script pour utiliser Claude comme linter ou examinateur spécifique au projet.

Ce script `package.json` transmet le diff par rapport à `main` à Claude et lui demande de signaler les fautes de frappe. Transmettre le diff signifie que Claude n'a pas besoin de permission Bash pour le lire, et les guillemets échappés gardent le script portable vers Windows :

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

Exécutez-le avec `npm run lint:claude`.

<h3 id="get-structured-output">
  Obtenir une sortie structurée
</h3>

Utilisez `--output-format` pour contrôler la façon dont les réponses sont renvoyées :

* `text` (par défaut) : sortie en texte brut
* `json` : JSON structuré avec résultat, ID de session et métadonnées
* `stream-json` : JSON délimité par des sauts de ligne pour le streaming en temps réel

Cet exemple retourne un résumé du projet en JSON avec les métadonnées de session, avec le résultat textuel dans le champ `result` :

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

Pour obtenir une sortie conforme à un schéma spécifique, utilisez `--output-format json` avec `--json-schema` et une définition [JSON Schema](https://json-schema.org/). La réponse inclut les métadonnées de la requête (ID de session, utilisation, etc.) avec la sortie structurée dans le champ `structured_output`.

Cet exemple extrait les noms de fonction et les retourne sous forme de tableau de chaînes :

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

Si la valeur n'est pas un JSON Schema valide, `claude` se ferme avec `Error: --json-schema is not a valid JSON Schema` suivi du diagnostic du validateur. Claude Code accepte les schémas qui utilisent le mot-clé `format`, tel que `"format": "email"`, mais traite `format` comme une annotation et ne l'applique pas. Avant la v2.1.205, Claude Code ignorait silencieusement un schéma invalide et retournait du texte non structuré, et traitait tout schéma contenant `format` comme invalide.

<Tip>
  Utilisez un outil comme [jq](https://jqlang.org/) pour analyser la réponse et extraire des champs spécifiques :

  ```bash theme={null}
  # Extract the text result
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # Extract structured output
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  Réponses en streaming
</h3>

Utilisez `--output-format stream-json` avec `--verbose` et `--include-partial-messages` pour recevoir les jetons au fur et à mesure qu'ils sont générés. Chaque ligne est un objet JSON représentant un événement :

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

La dernière ligne du flux est un message `result` avec le texte de réponse final, le coût et les métadonnées de session.

Si votre consommateur lit le flux lentement, Claude Code attend que la sortie en file d'attente se vide avant de se fermer, en mettant à l'échelle l'attente en fonction de la quantité restante en file d'attente, plafonnée à 30 secondes. Avant la v2.1.214, l'attente de fermeture était plafonnée à environ deux secondes, ce qui pouvait couper la fin d'une grande réponse.

L'exemple suivant utilise [jq](https://jqlang.org/) pour filtrer les deltas de texte et afficher uniquement le texte en streaming. L'indicateur `-r` génère des chaînes brutes (sans guillemets) et `-j` joint sans sauts de ligne pour que les jetons se transmettent continuellement :

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Pour le streaming programmatique avec des rappels et des objets de message, consultez [Réponses en streaming en temps réel](/docs/fr/agent-sdk/streaming-output) dans la documentation du SDK Agent.

<h4 id="follow-subagent-messages">
  Suivre les messages des sous-agents
</h4>

Les messages des [sous-agents](/docs/fr/sub-agents) apparaissent dans le flux sous forme de messages `assistant` et `user` dont le champ `parent_tool_use_id` est l'ID de l'appel d'outil qui a généré le sous-agent. Les messages de la conversation principale portent `null` dans ce champ.

Le premier message d'un sous-agent s'exécutant en [avant-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) est un message `user` portant l'invite qui le pilote. Après ce premier message, Claude Code émet :

* **Par défaut** : les blocs `tool_use` et `tool_result` du sous-agent.
* **Avec [`--forward-subagent-text`](/docs/fr/cli-reference#cli-flags) ou [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](/docs/fr/env-vars)** : les blocs de texte et de réflexion du sous-agent également, afin que vous puissiez reconstruire la transcription de chaque sous-agent. Cela nécessite Claude Code v2.1.211 ou ultérieur.

Lorsque vous activez l'une ou l'autre option, Claude Code transfère les messages des [sous-agents à chaque profondeur d'imbrication](/docs/fr/sub-agents#let-subagents-spawn-their-own-subagents), qu'ils aient été générés avec l'outil Agent ou démarrés en tant que [compétence dupliquée](/docs/fr/skills#run-skills-in-a-subagent). Les messages des sous-agents qu'une compétence dupliquée génère, et des compétences dupliquées démarrées à l'intérieur d'un sous-agent ou d'une autre compétence dupliquée, nécessitent Claude Code v2.1.275 ou ultérieur. Dans `parent_tool_use_id`, les messages du sous-agent imbriqué portent l'ID de l'appel d'outil Agent ou Skill qui l'a démarré, afin que vous puissiez reconstruire l'arborescence d'imbrication complète en suivant ces ID. Avant la v2.1.219, les messages des sous-agents imbriqués n'apparaissaient pas dans le flux.

Les compétences qui [s'exécutent dans un sous-agent](/docs/fr/skills#run-skills-in-a-subagent) apparaissent dans le flux de la même manière : le premier message de la compétence dupliquée est un message `user` portant le contenu de la compétence qui pilote l'exécution. Si vous activez l'une ou l'autre option, le flux porte également les blocs de texte et de réflexion de la compétence dupliquée. Avant la v2.1.265, seuls les blocs `tool_use` et `tool_result` d'une compétence dupliquée apparaissaient dans le flux.

<h4 id="handle-api-retries">
  Gérer les tentatives d'API
</h4>

Lorsqu'une requête API échoue avec une erreur réessayable, Claude Code émet un événement `system/api_retry` avant de réessayer. Sur la v2.1.246 ou ultérieur, lorsqu'un `401` ou `403` rejette une credential [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper), Claude Code effectue les deux premières tentatives silencieusement sans événement, puis émet l'événement comme d'habitude à partir de la troisième tentative consécutive. Les tentatives silencieuses comptent toujours vers `attempt`. Vous pouvez utiliser l'événement pour afficher la progression des tentatives dans votre propre interface.

| Champ            | Type             | Description                                                                                                                                                                                                                                                                                                                                                                                                           |
| ---------------- | ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`       | type de message                                                                                                                                                                                                                                                                                                                                                                                                       |
| `subtype`        | `"api_retry"`    | identifie ceci comme un événement de tentative                                                                                                                                                                                                                                                                                                                                                                        |
| `attempt`        | entier           | numéro de tentative actuel, commençant à 1                                                                                                                                                                                                                                                                                                                                                                            |
| `max_retries`    | entier           | tentatives totales autorisées pour la cause de cet échec, qui peut être inférieur au budget de session                                                                                                                                                                                                                                                                                                                |
| `retry_delay_ms` | entier           | millisecondes jusqu'à la prochaine tentative                                                                                                                                                                                                                                                                                                                                                                          |
| `error_status`   | entier ou null   | code de statut HTTP de la tentative échouée, ou `null` lorsque la tentative n'a pas reçu de réponse HTTP de l'API                                                                                                                                                                                                                                                                                                     |
| `no_response`    | objet, optionnel | présent uniquement lorsque la tentative échouée n'a pas reçu [d'en-têtes de réponse à temps](/docs/fr/errors#no-response-from-api). `waited_ms` est la durée d'attente de cette tentative et `retry_wait_ms` est la durée d'attente de la tentative. Dans ces événements, `max_retries` reflète la tentative que cette cause obtient normalement, et non le budget de session. Nécessite Claude Code v2.1.261 ou ultérieur |
| `error`          | chaîne           | catégorie d'erreur : `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, ou `unknown`                                                                                                                                                              |
| `uuid`           | chaîne           | identifiant d'événement unique                                                                                                                                                                                                                                                                                                                                                                                        |
| `session_id`     | chaîne           | session à laquelle appartient l'événement                                                                                                                                                                                                                                                                                                                                                                             |

<h4 id="read-session-metadata">
  Lire les métadonnées de session
</h4>

L'événement `system/init` rapporte les métadonnées de session, y compris le modèle, les outils, les serveurs MCP et les plugins chargés. C'est le premier événement du flux sauf si des événements de démarrage le précèdent :

* Événements `plugin_install`, lorsque [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/fr/env-vars) est défini.
* [Événements `hook_started`, `hook_progress` et `hook_response`](/docs/fr/agent-sdk/typescript#sdkhookstartedmessage), tandis qu'un hook [`SessionStart`](/docs/fr/hooks#sessionstart) ou [`Setup`](/docs/fr/hooks#setup) configuré s'exécute. Ceux-ci se transmettent au fur et à mesure que le hook les produit. Claude Code v2.1.169 à v2.1.203 les a livrés en un seul lot après la fin du hook, toujours avant `system/init` ; v2.1.204 a restauré la livraison en direct.

L'événement porte également un tableau optionnel `capabilities` de chaînes nommant les comportements de protocole que cette version de Claude Code implémente, tels que `interrupt_receipt_v1` ou `interrupt_cancel_queued_v1`. Vérifiez-le pour détecter les fonctionnalités au lieu de comparer les chaînes de version, et ignorez les valeurs que vous ne reconnaissez pas. Le champ nécessite Claude Code v2.1.205 ou ultérieur et est absent des versions antérieures. Consultez [`SDKSystemMessage`](/docs/fr/agent-sdk/typescript#sdksystemmessage) pour la liste des capacités.

<h4 id="fail-ci-when-a-plugin-or-mcp-server-doesn’t-load">
  Échouer CI lorsqu'un plugin ou un serveur MCP ne se charge pas
</h4>

Utilisez les champs de plugin dans l'événement `system/init` pour détecter un plugin qui ne s'est pas chargé :

| Champ           | Type    | Description                                                                                                                                                                                                                                                                                                                                 |
| --------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | tableau | plugins qui se sont chargés avec succès, chacun avec `name` et `path`                                                                                                                                                                                                                                                                       |
| `plugin_errors` | tableau | erreurs de chargement de plugin, chacune avec `plugin`, `type` et `message`. Inclut les versions de dépendance non satisfaites et les échecs de chargement `--plugin-dir` tels qu'un chemin manquant ou une archive invalide. Les plugins affectés sont rétrogradés et absents de `plugins`. La clé est omise lorsqu'il n'y a pas d'erreurs |

Utilisez les champs du serveur MCP de la même manière. Lorsque vous passez [`--mcp-config`](/docs/fr/cli-reference#cli-flags) avec `-p`, Claude Code attend les serveurs toujours en attente avant d'exécuter le premier tour, jusqu'au délai d'expiration de démarrage [`MCP_TIMEOUT`](/docs/fr/env-vars), 30 secondes par défaut. Un serveur distant avec une [liste d'outils mise en cache](/docs/fr/agent-sdk/mcp#connection-timing) ignore l'attente, affiche `pending` dans `system/init` et se connecte lors de son premier appel d'outil. L'attente nécessite Claude Code v2.1.221 ou ultérieur.

Claude Code valide chaque entrée `--mcp-config` au démarrage et ignore les entrées qui échouent la validation, par exemple une entrée `url` sans `type`. L'exécution continue et se ferme correctement, vérifiez donc ces champs pour détecter un serveur qui ne s'est jamais chargé :

| Champ               | Type    | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_servers`       | tableau | serveurs MCP dans la session, chacun avec `name` et `status`                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `mcp_server_errors` | tableau | entrées `--mcp-config` ignorées par la validation de configuration, chacune avec `name`, `type` et `message`. `type` est une catégorie d'ignorance telle que `unknown_type`, `url_missing_type`, `invalid_config` ou `reserved_name` ; traitez les valeurs que vous ne reconnaissez pas comme un ignorage générique. Les serveurs affectés sont absents de `mcp_servers`. La clé est omise lorsqu'il n'y a pas d'erreurs, donc une porte CI peut échouer sur un tableau non vide. Nécessite Claude Code v2.1.219 ou ultérieur |

Lorsque vous exécutez la commande à la main dans un terminal, Claude Code imprime également un avertissement de démarrage sur stderr, tel que `Warning: 1 MCP server skipped due to invalid config:`, suivi de la raison de chaque entrée ignorée. Lorsque vous redirigez stderr, ou lorsqu'un programme tel qu'un exécuteur CI ou un hôte SDK le capture, Claude Code n'imprime aucun avertissement et rapporte les entrées ignorées uniquement dans le champ `mcp_server_errors`. L'avertissement nécessite Claude Code v2.1.219 ou ultérieur.

<h4 id="track-plugin-installs">
  Suivre les installations de plugins
</h4>

Lorsque [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/fr/env-vars) est défini, Claude Code émet des événements `system/plugin_install` tandis que les plugins de marketplace s'installent avant le premier tour. Utilisez-les pour afficher la progression de l'installation dans votre propre interface utilisateur.

| Champ        | Type                                                    | Description                                                                                                                 |
| ------------ | ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `type`       | `"system"`                                              | type de message                                                                                                             |
| `subtype`    | `"plugin_install"`                                      | identifie ceci comme un événement d'installation de plugin                                                                  |
| `status`     | `"started"`, `"installed"`, `"failed"` ou `"completed"` | `started` et `completed` encadrent l'installation globale ; `installed` et `failed` rapportent les marketplaces individuels |
| `name`       | chaîne, optionnel                                       | nom du marketplace, présent sur `installed` et `failed`                                                                     |
| `error`      | chaîne, optionnel                                       | message d'échec, présent sur `failed`                                                                                       |
| `uuid`       | chaîne                                                  | identifiant d'événement unique                                                                                              |
| `session_id` | chaîne                                                  | session à laquelle appartient l'événement                                                                                   |

<h3 id="auto-approve-tools">
  Approuver automatiquement les outils
</h3>

Utilisez `--allowedTools` pour permettre à Claude d'utiliser certains outils sans demander. Cet exemple exécute une suite de tests et corrige les défaillances, permettant à Claude d'exécuter des commandes Bash et de lire/modifier des fichiers sans demander la permission :

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Pour définir une base de référence pour la session entière au lieu de lister les outils individuels, passez un [mode de permission](/docs/fr/permission-modes). Pour `-p`, le [mode de permission de démarrage intégré](/docs/fr/permission-modes#which-mode-a-session-starts-in) est Manual sur tous les plans, donc passez le mode de permission que vous souhaitez :

* **`auto`** : passez `--permission-mode auto` pour qu'un classificateur examine la plupart des actions au lieu de vous
* **`dontAsk`** : Claude Code refuse chaque appel qui demanderait autrement, ce qui est utile pour les exécutions CI verrouillées. Les actions qui n'ont pas besoin d'approbation en mode Manual s'exécutent toujours, telles que les lectures de fichiers dans vos répertoires de travail et l'[ensemble de commandes en lecture seule](/docs/fr/permissions#read-only-commands), ainsi que les actions que vos entrées `--allowedTools` ou les règles `permissions.allow` couvrent. `AskUserQuestion`, les outils de connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) sont refusés même lorsqu'une règle d'autorisation correspond
* **`acceptEdits`** : Claude écrit les fichiers sans demander, et Claude Code approuve automatiquement les commandes de système de fichiers courants telles que `mkdir`, `touch`, `mv` et `cp`. Les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves) s'appliquent toujours. Hormis l'ensemble de commandes en lecture seule, les autres commandes shell et requêtes réseau ont toujours besoin d'une entrée `--allowedTools` ou d'une règle `permissions.allow`. Consultez [ce que `acceptEdits` approuve automatiquement](/docs/fr/permission-modes#auto-approve-file-edits-with-acceptedits-mode) pour la liste complète

Cet exemple applique les corrections de lint avec `acceptEdits` comme base de référence :

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="turn-off-permission-prompts-in-unattended-runs">
  Désactiver les invites de permission dans les exécutions sans surveillance
</h3>

Passez `--permission-prompts none` lorsque personne n'est disponible pour répondre aux invites de permission, par exemple dans une tâche planifiée. L'indicateur est plus important lorsque votre exécution a un hôte de permission : une application Agent SDK avec un rappel [`canUseTool`](/docs/fr/agent-sdk/user-input), ou un outil MCP que vous passez avec [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags). Sans l'indicateur, votre exécution attend que cet hôte réponde à chaque demande de permission.

Avec l'indicateur, votre exécution ne consulte pas l'hôte et n'attend pas. Tout ce qui demanderait est refusé sauf si un hook `PermissionRequest` l'autorise, Claude est informé que personne ne peut approuver la demande et ne pas la réessayer, et l'exécution continue. Dans une exécution `-p` sans hôte, ces demandes sont refusées de toute façon, et l'indicateur indique également à Claude de ne pas les réessayer. Les règles de permission, les hooks [`PermissionRequest`](/docs/fr/hooks#permissionrequest) et le mode de permission que vous définissez décident toujours de chaque appel en premier ; Claude Code refuse uniquement les demandes que rien d'autre ne résout.

Cet exemple exécute une tâche sans surveillance en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode). Le classificateur examine chaque action comme d'habitude, et Claude Code refuse tout ce qui aurait autrement entraîné une invite :

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

Avec `--permission-prompts none`, Claude Code supprime les outils qui ont besoin d'une réponse d'une personne, tels que [`AskUserQuestion`](/docs/fr/tools-reference#askuserquestion-tool-behavior), afin que Claude ne puisse pas les appeler. Toute [demande d'élicitation MCP](/docs/fr/mcp#respond-to-mcp-elicitation-requests) à laquelle aucun hook [`Elicitation`](/docs/fr/hooks#elicitation) ne répond est annulée.

Avec `--output-format stream-json`, les refus apparaissent sous forme de messages système `permission_denied`, et le message de résultat final les énumère dans `permission_denials`.

<Note>
  L'indicateur `--permission-prompts` nécessite Claude Code v2.1.259 ou ultérieur. Les versions antérieures le rejettent avec une erreur d'option inconnue.
</Note>

<h3 id="create-a-commit">
  Créer un commit
</h3>

Cet exemple examine les modifications mises en scène et crée un commit avec un message approprié :

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

L'indicateur `--allowedTools` utilise la [syntaxe de règle de permission](/docs/fr/settings-reference#permission-rule-syntax). Le ` *` de fin active la correspondance de préfixe, donc `Bash(git diff *)` autorise toute commande commençant par `git diff`. L'espace avant `*` est important : sans lui, `Bash(git diff*)` correspondrait également à `git diff-index`.

<Note>
  Le support des commandes diffère en mode `-p` :

  * Les [compétences](/docs/fr/skills) invoquées par l'utilisateur et les commandes personnalisées fonctionnent. Incluez `/skill-name` dans la chaîne d'invite et Claude Code l'étend avant d'exécuter.
  * Les commandes intégrées qui ne s'exécutent que dans l'interface du terminal, telles que `/login`, ne sont pas disponibles.
  * `/model`, `/effort`, `/fast`, `/color` et `/rename` acceptent la valeur comme argument, par exemple `/model sonnet`, et `/mcp` sans argument imprime un résumé textuel du statut du serveur. Ces formes nécessitent Claude Code v2.1.205 ou ultérieur et suivent les [notes de disponibilité de chaque commande](/docs/fr/commands#all-commands).
  * Pour modifier un paramètre, passez `key=value` à `/config`, par exemple `/config thinking=false`.
  * `/output-style <style>` bascule les [styles de sortie](/docs/fr/output-styles) et `/output-style` seul les énumère. Nécessite Claude Code v2.1.269 ou ultérieur.
</Note>

<h3 id="customize-the-system-prompt">
  Personnaliser l'invite système
</h3>

Utilisez `--append-system-prompt` pour ajouter des instructions tout en conservant le comportement par défaut de Claude Code. Cet exemple transmet un diff PR à Claude et lui demande de vérifier les vulnérabilités de sécurité. Enregistrez-le en tant que script shell, par exemple `review.sh` :

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Dans le script, `"$1"` représente le premier argument que vous passez sur la ligne de commande. Exécutez `bash review.sh 123` et le shell remplace `"$1"` par `123`, donc le script récupère le diff pour la PR 123. Claude Code imprime l'examen en JSON, avec le texte dans le champ `result`.

Consultez les [indicateurs d'invite système](/docs/fr/cli-reference#system-prompt-flags) pour plus d'options, y compris `--system-prompt` pour remplacer complètement l'invite par défaut.

<h3 id="continue-conversations">
  Continuer les conversations
</h3>

Utilisez `--continue` pour continuer la conversation la plus récente, ou `--resume` avec un ID de session pour continuer une conversation spécifique. Sur Claude Code v2.1.257 ou ultérieur, lorsque vous passez `--continue`, Claude Code ouvre une [session en arrière-plan](/docs/fr/sessions#resume-a-session) qui a terminé, mais pas une qui s'exécute toujours. Cet exemple exécute un examen, puis envoie des invites de suivi :

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

Si vous exécutez plusieurs conversations, capturez l'ID de session pour reprendre une conversation spécifique :

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

Vous pouvez exécuter les deux commandes à partir de répertoires différents : Claude Code [trouve la session par son ID](/docs/fr/sessions#resume-a-session) dans n'importe quel projet sur cette machine. Avant la v2.1.223, Claude Code recherchait l'ID uniquement dans le répertoire du projet actuel et ses git worktrees, vous deviez donc exécuter les deux commandes à partir du même répertoire.

À la place de l'ID de session, vous pouvez passer à `--resume` le chemin absolu vers le fichier de [transcription](/docs/fr/sessions#where-transcripts-are-stored) `.jsonl` d'une session, et Claude Code continue la conversation stockée dans ce fichier.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Démarrage rapide de l'Agent SDK](/docs/fr/agent-sdk/quickstart) : créez votre premier agent avec Python ou TypeScript
* [Référence CLI](/docs/fr/cli-reference) : tous les flags et options CLI
* [GitHub Actions](/docs/fr/github-actions) : utilisez l'Agent SDK dans les workflows GitHub
* [GitLab CI/CD](/docs/fr/gitlab-ci-cd) : utilisez l'Agent SDK dans les pipelines GitLab
