> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Exécuter Claude Code par programmation

> Utilisez l'Agent SDK pour exécuter Claude Code par programmation depuis la CLI, Python ou TypeScript.

L'[Agent SDK](/docs/fr/agent-sdk/overview) vous donne accès aux mêmes outils, boucle d'agent et gestion du contexte qui alimentent Claude Code. Il est disponible en tant que CLI pour les scripts et CI/CD, ou en tant que packages [Python](/docs/fr/agent-sdk/python) et [TypeScript](/docs/fr/agent-sdk/typescript) pour un contrôle programmatique complet.

Pour exécuter Claude Code en mode non interactif, passez `-p` avec votre prompt et toute [option CLI](/docs/fr/cli-reference) :

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Cette page couvre l'utilisation de l'Agent SDK via la CLI (`claude -p`). Pour les packages SDK Python et TypeScript avec sorties structurées, callbacks d'approbation d'outils et objets de message natifs, consultez la [documentation complète de l'Agent SDK](/docs/fr/agent-sdk/overview).

<h2 id="basic-usage">
  Utilisation basique
</h2>

Ajoutez le flag `-p` (ou `--print`) à n'importe quelle commande `claude` pour l'exécuter de manière non-interactive. Toutes les [options CLI](/docs/fr/cli-reference) fonctionnent avec `-p`, notamment :

* `--continue` pour [continuer les conversations](#continue-conversations)
* `--allowedTools` pour [approuver automatiquement les outils](#auto-approve-tools)
* `--output-format` pour [obtenir une sortie structurée](#get-structured-output)

Cet exemple pose une question à Claude sur votre base de code et affiche la réponse :

```bash theme={null}
claude -p "What does the auth module do?"
```

<h3 id="start-faster-with-bare-mode">
  Démarrer plus rapidement avec le mode bare
</h3>

Ajoutez `--bare` pour réduire le temps de démarrage en ignorant la découverte automatique des hooks, skills, plugins, serveurs MCP, mémoire automatique et CLAUDE.md. Sans cela, `claude -p` charge le même [contexte](/docs/fr/how-claude-code-works#the-context-window) qu'une session interactive, y compris tout ce qui est configuré dans le répertoire de travail ou `~/.claude`.

Le mode bare est utile pour CI et les scripts où vous avez besoin du même résultat sur chaque machine. Un hook dans le `~/.claude` d'un coéquipier ou un serveur MCP dans le `.mcp.json` du projet ne s'exécutera pas, car le mode bare ne les lit jamais. Seuls les flags que vous passez explicitement prennent effet.

Cet exemple exécute une tâche de résumé ponctuelle en mode bare et pré-approuve l'outil Read pour que l'appel se termine sans invite de permission :

```bash theme={null}
claude --bare -p "Summarize this file" --allowedTools "Read"
```

En mode bare, Claude a accès aux outils Bash, lecture de fichier et modification de fichier. Passez tout contexte dont vous avez besoin avec un flag :

| Pour charger             | Utilisez                                                |
| ------------------------ | ------------------------------------------------------- |
| Ajouts de prompt système | `--append-system-prompt`, `--append-system-prompt-file` |
| Paramètres               | `--settings <file-or-json>`                             |
| Serveurs MCP             | `--mcp-config <file-or-json>`                           |
| Agents personnalisés     | `--agents <json>`                                       |
| Un plugin                | `--plugin-dir <path>`, `--plugin-url <url>`             |

Le mode bare ignore OAuth et les lectures du trousseau. L'authentification Anthropic doit provenir de `ANTHROPIC_API_KEY` ou d'un `apiKeyHelper` dans le JSON passé à `--settings`. Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry utilisent leurs identifiants de fournisseur habituels.

<Note>
  `--bare` est le mode recommandé pour les appels scriptés et SDK, et deviendra le mode par défaut pour `-p` dans une version future.
</Note>

<h3 id="background-tasks-at-exit">
  Tâches en arrière-plan à la sortie
</h3>

Si Claude démarre une [tâche Bash en arrière-plan](/docs/fr/tools-reference#bash-tool-behavior) lors d'une exécution `claude -p`, par exemple un serveur de développement ou une compilation en surveillance, cette tâche est terminée environ cinq secondes après que Claude ait retourné son résultat final et que stdin ait été fermé. La période de grâce permet à une tâche qui se termine juste après le résultat de livrer quand même sa sortie. Avant la v2.1.163, un processus en arrière-plan qui ne s'arrêtait jamais maintiendrait l'invocation `claude -p` ouverte indéfiniment.

Les [sous-agents](/docs/fr/sub-agents) en arrière-plan et les workflows sont exempts de la période de grâce de cinq secondes car leur résultat fait partie de la sortie finale, donc `claude -p` attend qu'ils se terminent. À partir de la v2.1.182, cette attente est plafonnée à dix minutes par défaut afin qu'un agent en arrière-plan bloqué ne puisse pas maintenir le processus ouvert indéfiniment. Ajustez le plafond avec [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/fr/env-vars), ou définissez-le sur `0` pour attendre sans limite.

<h2 id="examples">
  Exemples
</h2>

Ces exemples mettent en évidence les modèles CLI courants. Pour CI et autres appels scriptés, ajoutez [`--bare`](#start-faster-with-bare-mode) pour qu'ils ne reprennent pas ce qui se trouve configuré localement.

<h3 id="pipe-data-through-claude">
  Transmettre des données via Claude
</h3>

Le mode non-interactif lit stdin, vous pouvez donc transmettre des données et rediriger la réponse comme n'importe quel autre outil en ligne de commande.

Cet exemple transmet un journal de compilation à Claude et écrit l'explication dans un fichier :

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Avec `--output-format json`, la charge utile de réponse inclut `total_cost_usd` et une ventilation des coûts par modèle, afin que les appelants scriptés puissent suivre les dépenses par invocation sans consulter le [tableau de bord d'utilisation](/docs/fr/costs).

<Note>
  À partir de Claude Code v2.1.128, stdin transmis est limité à 10 Mo. Si vous dépassez la limite, Claude Code se ferme avec une erreur claire et un statut non nul. Pour travailler avec des entrées plus grandes, écrivez le contenu dans un fichier et référencez le chemin du fichier dans votre prompt au lieu de le transmettre.
</Note>

<h3 id="add-claude-to-a-build-script">
  Ajouter Claude à un script de compilation
</h3>

Vous pouvez envelopper un appel non-interactif dans un script pour utiliser Claude comme linter ou examinateur spécifique au projet.

Ce script `package.json` transmet le diff par rapport à `main` à Claude et lui demande de signaler les fautes de frappe. Transmettre le diff signifie que Claude n'a pas besoin de permission Bash pour le lire, et les guillemets échappés gardent le script portable vers Windows :

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

<h3 id="get-structured-output">
  Obtenir une sortie structurée
</h3>

Utilisez `--output-format` pour contrôler la façon dont les réponses sont retournées :

* `text` (par défaut) : sortie en texte brut
* `json` : JSON structuré avec résultat, ID de session et métadonnées
* `stream-json` : JSON délimité par des sauts de ligne pour le streaming en temps réel

Cet exemple retourne un résumé du projet au format JSON avec les métadonnées de session, avec le résultat textuel dans le champ `result` :

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

Pour obtenir une sortie conforme à un schéma spécifique, utilisez `--output-format json` avec `--json-schema` et une définition [JSON Schema](https://json-schema.org/). La réponse inclut les métadonnées sur la requête (ID de session, utilisation, etc.) avec la sortie structurée dans le champ `structured_output`.

Cet exemple extrait les noms de fonctions et les retourne sous forme de tableau de chaînes :

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

Si la valeur n'est pas un JSON Schema valide, `claude` se ferme avec `Error: --json-schema is not a valid JSON Schema` suivi du diagnostic du validateur. Claude Code accepte les schémas qui utilisent le mot-clé `format`, tel que `"format": "email"`, mais traite `format` comme une annotation et ne l'applique pas. Avant v2.1.205, Claude Code ignorait silencieusement un schéma invalide et retournait du texte non structuré, et traitait tout schéma contenant `format` comme invalide.

<Tip>
  Utilisez un outil comme [jq](https://jqlang.github.io/jq/) pour analyser la réponse et extraire des champs spécifiques :

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

Utilisez `--output-format stream-json` avec `--verbose` et `--include-partial-messages` pour recevoir les tokens au fur et à mesure qu'ils sont générés. Chaque ligne est un objet JSON représentant un événement :

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

La dernière ligne du flux est un message `result` avec le texte de réponse final, le coût et les métadonnées de session. Avant v2.1.208, transmettre une réponse volumineuse pouvait tronquer la dernière ligne et omettre le message `result`.

L'exemple suivant utilise [jq](https://jqlang.github.io/jq/) pour filtrer les deltas de texte et afficher uniquement le texte en streaming. Le flag `-r` affiche les chaînes brutes (sans guillemets) et `-j` joint sans sauts de ligne pour que les tokens se diffusent en continu :

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Quand une requête API échoue avec une erreur réessayable, Claude Code émet un événement `system/api_retry` avant de réessayer. Vous pouvez l'utiliser pour afficher la progression des tentatives ou implémenter une logique de backoff personnalisée.

| Champ            | Type           | Description                                                                                                                                                                                                 |
| ---------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`     | type de message                                                                                                                                                                                             |
| `subtype`        | `"api_retry"`  | identifie ceci comme un événement de tentative                                                                                                                                                              |
| `attempt`        | entier         | numéro de tentative actuel, commençant à 1                                                                                                                                                                  |
| `max_retries`    | entier         | nombre total de tentatives autorisées                                                                                                                                                                       |
| `retry_delay_ms` | entier         | millisecondes jusqu'à la prochaine tentative                                                                                                                                                                |
| `error_status`   | entier ou null | code de statut HTTP, ou `null` pour les erreurs de connexion sans réponse HTTP                                                                                                                              |
| `error`          | chaîne         | catégorie d'erreur : `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, ou `unknown` |
| `uuid`           | chaîne         | identifiant d'événement unique                                                                                                                                                                              |
| `session_id`     | chaîne         | session à laquelle appartient l'événement                                                                                                                                                                   |

L'événement `system/init` rapporte les métadonnées de session, y compris le modèle, les outils, les serveurs MCP et les plugins chargés. C'est le premier événement du flux sauf si les événements de démarrage le précèdent :

* Les événements `plugin_install`, quand [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/fr/env-vars) est défini.
* Les événements [`hook_started`, `hook_progress` et `hook_response`](/docs/fr/agent-sdk/typescript#sdkhookstartedmessage), pendant qu'un hook [`SessionStart`](/docs/fr/hooks#sessionstart) ou [`Setup`](/docs/fr/hooks#setup) configuré s'exécute. Ces événements se diffusent au fur et à mesure que le hook les produit. Claude Code v2.1.169 à v2.1.203 les a livrés en un seul lot après la fin du hook, toujours avant `system/init` ; v2.1.204 a restauré la livraison en direct.

L'événement porte également un tableau `capabilities` optionnel de chaînes nommant les comportements de protocole que cette version de Claude Code implémente, tels que `interrupt_receipt_v1`. Vérifiez-le pour détecter les fonctionnalités au lieu de comparer les chaînes de version, et ignorez les valeurs que vous ne reconnaissez pas. Le champ nécessite Claude Code v2.1.205 ou ultérieur et est absent des versions antérieures. Consultez [`SDKSystemMessage`](/docs/fr/agent-sdk/typescript#sdksystemmessage) pour la liste des capacités.

Utilisez les champs de plugin pour échouer CI quand un plugin n'a pas pu être chargé :

| Champ           | Type    | Description                                                                                                                                                                                                                                                                                                                                        |
| --------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | tableau | plugins qui se sont chargés avec succès, chacun avec `name` et `path`                                                                                                                                                                                                                                                                              |
| `plugin_errors` | tableau | erreurs de chargement de plugin, chacune avec `plugin`, `type` et `message`. Inclut les versions de dépendance non satisfaites et les défaillances de chargement `--plugin-dir` telles qu'un chemin manquant ou une archive invalide. Les plugins affectés sont rétrogradés et absents de `plugins`. La clé est omise quand il n'y a pas d'erreurs |

Quand [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/fr/env-vars) est défini, Claude Code émet des événements `system/plugin_install` pendant que les plugins de marketplace s'installent avant le premier tour. Utilisez-les pour afficher la progression de l'installation dans votre propre interface utilisateur.

| Champ        | Type                                                     | Description                                                                                                                   |
| ------------ | -------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `type`       | `"system"`                                               | type de message                                                                                                               |
| `subtype`    | `"plugin_install"`                                       | identifie ceci comme un événement d'installation de plugin                                                                    |
| `status`     | `"started"`, `"installed"`, `"failed"`, ou `"completed"` | `started` et `completed` encadrent l'installation globale ; `installed` et `failed` rapportent les marketplaces individuelles |
| `name`       | chaîne, optionnel                                        | nom de la marketplace, présent sur `installed` et `failed`                                                                    |
| `error`      | chaîne, optionnel                                        | message d'échec, présent sur `failed`                                                                                         |
| `uuid`       | chaîne                                                   | identifiant d'événement unique                                                                                                |
| `session_id` | chaîne                                                   | session à laquelle appartient l'événement                                                                                     |

Pour le streaming programmatique avec callbacks et objets de message, consultez [Réponses en streaming en temps réel](/docs/fr/agent-sdk/streaming-output) dans la documentation de l'Agent SDK.

<h3 id="auto-approve-tools">
  Approuver automatiquement les outils
</h3>

Utilisez `--allowedTools` pour permettre à Claude d'utiliser certains outils sans demander. Cet exemple exécute une suite de tests et corrige les défaillances, permettant à Claude d'exécuter des commandes Bash et de lire/modifier des fichiers sans demander la permission :

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Pour définir une base de référence pour la session entière au lieu de lister les outils individuels, passez un [mode de permission](/docs/fr/permission-modes). `dontAsk` refuse tout ce qui n'est pas dans vos règles `permissions.allow` ou l'[ensemble de commandes en lecture seule](/docs/fr/permissions#read-only-commands), ce qui est utile pour les exécutions CI verrouillées. `AskUserQuestion`, les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) sont refusés même quand une règle d'autorisation correspond.

`acceptEdits` permet à Claude d'écrire des fichiers sans demander et approuve également automatiquement les commandes de système de fichiers courants telles que `mkdir`, `touch`, `mv` et `cp`. Les autres commandes shell et requêtes réseau ont toujours besoin d'une entrée `--allowedTools` ou d'une règle `permissions.allow`, sinon l'exécution s'arrête quand l'une d'elles est tentée :

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="create-a-commit">
  Créer un commit
</h3>

Cet exemple examine les modifications mises en scène et crée un commit avec un message approprié :

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

Le flag `--allowedTools` utilise la [syntaxe des règles de permission](/docs/fr/settings#permission-rule-syntax). L'espace ` *` à la fin active la correspondance de préfixe, donc `Bash(git diff *)` autorise n'importe quelle commande commençant par `git diff`. L'espace avant `*` est important : sans lui, `Bash(git diff*)` correspondrait également à `git diff-index`.

<Note>
  Les [skills](/docs/fr/skills) invoquées par l'utilisateur et les commandes personnalisées fonctionnent en mode `-p` : incluez `/skill-name` dans la chaîne de prompt et Claude Code l'étend avant d'exécuter. Les commandes intégrées qui ouvrent un dialogue interactif, telles que `/login`, ne sont pas disponibles en mode `-p`. `/model`, `/effort`, `/fast`, `/color` et `/rename` acceptent la valeur comme argument, par exemple `/model sonnet`, et `/mcp` sans argument affiche un résumé textuel du statut du serveur ; ces formes nécessitent Claude Code v2.1.205 ou ultérieur et suivent les notes de disponibilité de chaque commande]\(/fr/commands#all-commands). Pour modifier un paramètre à partir d'une invocation `-p`, passez `key=value` à `/config`, par exemple `/config thinking=false`.
</Note>

<h3 id="customize-the-system-prompt">
  Personnaliser le prompt système
</h3>

Utilisez `--append-system-prompt` pour ajouter des instructions tout en conservant le comportement par défaut de Claude Code. Cet exemple envoie un diff de PR à Claude et lui demande de vérifier les vulnérabilités de sécurité :

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Consultez les [flags de prompt système](/docs/fr/cli-reference#system-prompt-flags) pour plus d'options, notamment `--system-prompt` pour remplacer complètement le prompt par défaut.

<h3 id="continue-conversations">
  Continuer les conversations
</h3>

Utilisez `--continue` pour continuer la conversation la plus récente, ou `--resume` avec un ID de session pour continuer une conversation spécifique. Cet exemple exécute un examen, puis envoie des prompts de suivi :

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

Exécutez les deux commandes à partir du même répertoire : la recherche d'ID de session est limitée au répertoire de projet actuel et à ses git worktrees. Consultez [Reprendre une session](/docs/fr/sessions#resume-a-session) pour les règles de portée complètes.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Démarrage rapide de l'Agent SDK](/docs/fr/agent-sdk/quickstart) : créez votre premier agent avec Python ou TypeScript
* [Référence CLI](/docs/fr/cli-reference) : tous les flags et options CLI
* [GitHub Actions](/docs/fr/github-actions) : utilisez l'Agent SDK dans les workflows GitHub
* [GitLab CI/CD](/docs/fr/gitlab-ci-cd) : utilisez l'Agent SDK dans les pipelines GitLab
