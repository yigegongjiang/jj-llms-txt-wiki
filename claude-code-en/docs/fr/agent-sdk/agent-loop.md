> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Fonctionnement de la boucle d'agent

> Comprenez le cycle de vie des messages, l'exécution des outils, la fenêtre de contexte et l'architecture qui alimentent vos agents SDK.

Le SDK Agent vous permet d'intégrer la boucle d'agent autonome de Claude Code dans vos propres applications. Le SDK est un package autonome qui vous donne un contrôle programmatique sur les outils, les permissions, les limites de coûts et la sortie. Vous n'avez pas besoin d'avoir l'interface de ligne de commande Claude Code installée pour l'utiliser.

Lorsque vous démarrez un agent, le SDK exécute la même [boucle d'exécution qui alimente Claude Code](/docs/fr/how-claude-code-works#the-agentic-loop) : Claude évalue votre prompt, appelle les outils pour agir, reçoit les résultats et répète jusqu'à ce que la tâche soit terminée. Cette page explique ce qui se passe à l'intérieur de cette boucle afin que vous puissiez construire, déboguer et optimiser vos agents efficacement.

<h2 id="the-loop-at-a-glance">
  La boucle en un coup d'œil
</h2>

Chaque session d'agent suit le même cycle :

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-loop-diagram.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=1c6e8f28d80dba14a7287419656f1237" alt="Diagramme de la boucle d'agent : votre prompt entre dans la boucle agentique, où Claude évalue et demande soit des appels d'outils, dont les résultats réintègrent une autre évaluation, soit retourne la réponse finale" width="720" height="212" data-path="images/agent-loop-diagram.svg" />

1. **Recevoir le prompt.** Claude reçoit votre prompt, ainsi que le prompt système, les définitions d'outils et l'historique de conversation. Le SDK produit un [`SystemMessage`](#message-types) avec le sous-type `"init"` contenant les métadonnées de session.
2. **Évaluer et répondre.** Claude évalue l'état actuel et détermine comment procéder. Il peut répondre avec du texte, demander un ou plusieurs appels d'outils, ou les deux. Le SDK produit un [`AssistantMessage`](#message-types) contenant le texte et toutes les demandes d'appels d'outils.
3. **Exécuter les outils.** Le SDK exécute chaque outil demandé et collecte les résultats. Chaque ensemble de résultats d'outils est renvoyé à Claude pour la décision suivante. Vous pouvez utiliser des [hooks](/docs/fr/agent-sdk/hooks) pour intercepter, modifier ou bloquer les appels d'outils avant qu'ils ne s'exécutent.
4. **Répéter.** Les étapes 2 et 3 se répètent en cycle. Chaque cycle complet est un tour. Claude continue à appeler les outils et à traiter les résultats jusqu'à ce qu'il produise une réponse sans appels d'outils.
5. **Retourner le résultat.** Le SDK produit un [`AssistantMessage`](#message-types) final avec la réponse textuelle (sans appels d'outils), suivi d'un [`ResultMessage`](#message-types) avec le texte final, l'utilisation des tokens, le coût et l'ID de session.

Une question rapide (« quels fichiers sont ici ? ») peut prendre un ou deux tours d'appel de `Glob` et de réponse avec les résultats. Une tâche complexe (« refactorisez le module d'authentification et mettez à jour les tests ») peut enchaîner des dizaines d'appels d'outils sur plusieurs tours, en lisant des fichiers, en modifiant du code et en exécutant des tests, avec Claude ajustant son approche en fonction de chaque résultat.

<h2 id="turns-and-messages">
  Tours et messages
</h2>

Un tour est un aller-retour dans la boucle : Claude produit une sortie qui inclut des appels d'outils, le SDK exécute ces outils, et les résultats sont renvoyés à Claude automatiquement. Cela se produit sans rendre le contrôle à votre code. Les tours continuent jusqu'à ce que Claude produise une sortie sans appels d'outils, auquel point la boucle se termine et le résultat final est livré.

Considérez ce qu'une session complète pourrait ressembler pour le prompt « Corrigez les tests défaillants dans auth.ts ».

D'abord, le SDK envoie votre prompt à Claude et produit un [`SystemMessage`](#message-types) avec les métadonnées de session. Ensuite, la boucle commence :

1. **Tour 1 :** Claude appelle `Bash` pour exécuter `npm test`. Le SDK produit un [`AssistantMessage`](#message-types) avec l'appel d'outil, exécute la commande, puis produit un [`UserMessage`](#message-types) avec la sortie (trois défaillances).
2. **Tour 2 :** Claude appelle `Read` sur `auth.ts` et `auth.test.ts`. Le SDK retourne le contenu des fichiers et produit un `AssistantMessage`.
3. **Tour 3 :** Claude appelle `Edit` pour corriger `auth.ts`, puis appelle `Bash` pour relancer `npm test`. Les trois tests réussissent. Le SDK produit un `AssistantMessage`.
4. **Tour final :** Claude produit une réponse textuelle uniquement sans appels d'outils : « Correction du bug d'authentification, les trois tests réussissent maintenant. » Le SDK produit un `AssistantMessage` final avec ce texte, puis un [`ResultMessage`](#message-types) avec le même texte plus le coût et l'utilisation.

C'était quatre tours : trois avec des appels d'outils, un final avec réponse textuelle uniquement.

Vous pouvez limiter la boucle avec `max_turns` / `maxTurns`, qui compte uniquement les tours d'utilisation d'outils. Par exemple, `max_turns=2` dans la boucle ci-dessus aurait arrêté avant l'étape d'édition. Vous pouvez également utiliser `max_budget_usd` / `maxBudgetUsd` pour limiter les tours en fonction d'un seuil de dépense.

Sans limites, la boucle s'exécute jusqu'à ce que Claude termine de lui-même, ce qui est correct pour les tâches bien délimitées mais peut s'exécuter longtemps sur des prompts ouverts (« améliorez cette base de code »). Définir un budget est une bonne valeur par défaut pour les agents de production. Voir [Tours et budget](#turns-and-budget) ci-dessous pour la référence des options.

<h2 id="message-types">
  Types de messages
</h2>

Lorsque la boucle s'exécute, le SDK produit un flux de messages. Chaque message porte un type qui vous indique à quel stade de la boucle il provient. Les cinq types principaux sont :

* **`SystemMessage` :** événements du cycle de vie de la session. Le champ `subtype` les distingue :

  * `"init"` : métadonnées de session pour l'exécution. Lorsqu'un hook `SessionStart` ou `Setup` s'exécute au démarrage de la session, ses [messages du cycle de vie du hook](/docs/fr/agent-sdk/typescript#sdkhookstartedmessage) arrivent avant le message `init`
  * `"compact_boundary"` : se déclenche après [compaction](#automatic-compaction)
  * `"informational"` : bannières de statut en texte brut de la boucle
  * `"worker_shutting_down"` : la boucle se terminera après le tour actuel car l'hôte se ferme ou Remote Control s'est déconnecté

  En TypeScript, chaque sous-type autre que `"init"` est son propre type dans l'union [`SDKMessage`](/docs/fr/agent-sdk/typescript#sdkmessage) plutôt qu'un sous-type de `SDKSystemMessage`.
* **`AssistantMessage` :** émis après chaque réponse de Claude, y compris la réponse textuelle finale. Contient les blocs de contenu textuel et les blocs d'appels d'outils de ce tour.
* **`UserMessage` :** émis après chaque exécution d'outil avec le résultat d'outil renvoyé à Claude. Également émis pour toute entrée utilisateur que vous diffusez en boucle.
* **`StreamEvent` :** émis uniquement lorsque les messages partiels sont activés. Contient les événements de diffusion API bruts (deltas de texte, chunks d'entrée d'outil). Voir [Réponses en flux](/docs/fr/agent-sdk/streaming-output).
* **`ResultMessage` :** marque la fin de la boucle d'agent. Contient le résultat textuel final, l'utilisation des tokens, le coût et l'ID de session. Vérifiez le champ `subtype` pour déterminer si la tâche a réussi ou a atteint une limite. Un petit nombre d'événements système de fin, tels que `prompt_suggestion`, peuvent arriver après, donc itérez le flux jusqu'à la fin plutôt que de vous arrêter au résultat. Voir [Gérer le résultat](#handle-the-result).

Ces cinq types couvrent le cycle de vie complet de la boucle d'agent dans les deux SDK. Le SDK TypeScript produit également des événements d'observabilité supplémentaires (événements de hooks, progression des outils, limites de débit, notifications de tâches) qui fournissent des détails supplémentaires mais ne sont pas nécessaires pour piloter la boucle. Voir la [référence des types de messages Python](/docs/fr/agent-sdk/python#message-types) et la [référence des types de messages TypeScript](/docs/fr/agent-sdk/typescript#message-types) pour les listes complètes.

<h3 id="handle-messages">
  Gérer les messages
</h3>

Les messages que vous gérez dépendent de ce que vous construisez :

* **Résultats finaux uniquement :** gérez `ResultMessage` pour obtenir la sortie, le coût et si la tâche a réussi ou a atteint une limite.
* **Mises à jour de progression :** gérez `AssistantMessage` pour voir ce que Claude fait à chaque tour, y compris les outils qu'il a appelés.
* **Diffusion en direct :** activez les messages partiels (`include_partial_messages` en Python, `includePartialMessages` en TypeScript) pour obtenir les messages `StreamEvent` en temps réel. Voir [Réponses en flux en temps réel](/docs/fr/agent-sdk/streaming-output).

La façon dont vous vérifiez les types de messages dépend du SDK :

* **Python :** vérifiez les types de messages avec `isinstance()` par rapport aux classes importées de `claude_agent_sdk` (par exemple, `isinstance(message, ResultMessage)`).
* **TypeScript :** vérifiez le champ de chaîne `type` (par exemple, `message.type === "result"`). `AssistantMessage` et `UserMessage` enveloppent le message API brut dans un champ `.message`, donc les blocs de contenu sont à `message.message.content`, pas `message.content`.

<Accordion title="Exemple : Vérifier les types de messages et gérer les résultats">
  <CodeGroup>
    ```python Python theme={null}
    import asyncio
    from claude_agent_sdk import query, AssistantMessage, ResultMessage


    async def main():
        try:
            async for message in query(prompt="Summarize this project"):
                if isinstance(message, AssistantMessage):
                    print(f"Turn completed: {len(message.content)} content blocks")
                if isinstance(message, ResultMessage):
                    if message.subtype == "success":
                        print(message.result)
                    else:
                        print(f"Stopped: {message.subtype}")
        except Exception as error:
            # A single-shot query() raises after yielding an error result. If the
            # failure was an error result, the error subtype branches above have
            # already run; connection or process failures yield no result message.
            print(f"Session ended with an error: {error}")


    asyncio.run(main())
    ```

    ```typescript TypeScript theme={null}
    import { query } from "@anthropic-ai/claude-agent-sdk";

    try {
      for await (const message of query({ prompt: "Summarize this project" })) {
        if (message.type === "assistant") {
          console.log(`Turn completed: ${message.message.content.length} content blocks`);
        }
        if (message.type === "result") {
          if (message.subtype === "success") {
            console.log(message.result);
          } else {
            console.log(`Stopped: ${message.subtype}`);
          }
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, the error subtype branches above have
      // already run; connection or process failures yield no result message.
      console.log(`Session ended with an error: ${error}`);
    }
    ```
  </CodeGroup>
</Accordion>

<h2 id="tool-execution">
  Exécution des outils
</h2>

Les outils donnent à votre agent la capacité d'agir. Sans outils, Claude ne peut que répondre avec du texte. Avec les outils, Claude peut lire des fichiers, exécuter des commandes, rechercher du code et interagir avec des services externes.

<h3 id="built-in-tools">
  Outils intégrés
</h3>

Le SDK inclut les mêmes outils qui alimentent Claude Code :

| Catégorie                   | Outils                                                          | Ce qu'ils font                                                                                 |
| :-------------------------- | :-------------------------------------------------------------- | :--------------------------------------------------------------------------------------------- |
| **Opérations sur fichiers** | `Read`, `Edit`, `Write`                                         | Lire, modifier et créer des fichiers                                                           |
| **Recherche**               | `Glob`, `Grep`                                                  | Trouver des fichiers par motif, rechercher du contenu avec regex                               |
| **Exécution**               | `Bash`                                                          | Exécuter des commandes shell, des scripts, des opérations git                                  |
| **Web**                     | `WebSearch`, `WebFetch`                                         | Rechercher le web, récupérer et analyser des pages                                             |
| **Découverte**              | `ToolSearch`                                                    | Trouver et charger dynamiquement les outils à la demande au lieu de les précharger tous        |
| **Orchestration**           | `Agent`, `Skill`, `AskUserQuestion`, `TaskCreate`, `TaskUpdate` | Générer des sous-agents, invoquer des compétences, demander à l'utilisateur, suivre les tâches |

Au-delà des outils intégrés, vous pouvez :

* **Connecter des services externes** avec des [serveurs MCP](/docs/fr/agent-sdk/mcp) (bases de données, navigateurs, API)
* **Définir des outils personnalisés** avec des [gestionnaires d'outils personnalisés](/docs/fr/agent-sdk/custom-tools)
* **Charger les compétences du projet** via des [sources de paramètres](/docs/fr/agent-sdk/claude-code-features) pour des flux de travail réutilisables

<h3 id="tool-permissions">
  Permissions des outils
</h3>

Claude détermine les outils à appeler en fonction de la tâche, mais vous contrôlez si ces appels sont autorisés à s'exécuter. Vous pouvez approuver automatiquement des outils spécifiques, en bloquer d'autres entièrement, ou exiger une approbation pour tout. Trois options fonctionnent ensemble pour déterminer ce qui s'exécute :

* **`allowed_tools` / `allowedTools`** approuve automatiquement les outils listés. Un agent en lecture seule avec `["Read", "Glob", "Grep"]` dans sa liste d'outils autorisés exécute ces outils sans demander. Les outils non listés sont toujours disponibles mais nécessitent une permission.
* **`disallowed_tools` / `disallowedTools`** bloque les outils listés, indépendamment des autres paramètres. Voir [Permissions](/docs/fr/agent-sdk/permissions) pour l'ordre dans lequel les règles sont vérifiées avant qu'un outil s'exécute.
* **`permission_mode` / `permissionMode`** contrôle ce qui se passe pour les outils qui ne sont pas couverts par les règles d'autorisation ou de refus. Voir [Mode de permission](#permission-mode) pour les modes disponibles.

Vous pouvez également délimiter les outils individuels avec des règles comme `"Bash(npm *)"` pour autoriser uniquement des commandes spécifiques. Voir [Permissions](/docs/fr/agent-sdk/permissions) pour la syntaxe complète des règles.

Lorsqu'un outil est refusé, Claude reçoit un message de rejet comme résultat d'outil et tente généralement une approche différente ou signale qu'il ne pouvait pas procéder.

<h3 id="parallel-tool-execution">
  Exécution parallèle des outils
</h3>

Lorsque Claude demande plusieurs appels d'outils en un seul tour, les deux SDK peuvent les exécuter de manière concurrente ou séquentielle selon l'outil. Les outils en lecture seule (comme `Read`, `Glob`, `Grep` et les outils MCP marqués comme en lecture seule) peuvent s'exécuter de manière concurrente. Les outils qui modifient l'état (comme `Edit`, `Write` et `Bash`) s'exécutent séquentiellement pour éviter les conflits.

Les outils personnalisés s'exécutent par défaut de manière séquentielle. Pour activer l'exécution parallèle pour un outil personnalisé, définissez `readOnlyHint` dans ses annotations. Les deux SDK [TypeScript](/docs/fr/agent-sdk/typescript#tool) et [Python](/docs/fr/agent-sdk/python#tool) utilisent ce nom de champ du SDK MCP.

<h2 id="control-how-the-loop-runs">
  Contrôler le fonctionnement de la boucle
</h2>

Vous pouvez limiter le nombre de tours que la boucle prend, combien elle coûte, la profondeur du raisonnement de Claude et si les outils nécessitent une approbation avant de s'exécuter. Tous ces éléments sont des champs sur [`ClaudeAgentOptions`](/docs/fr/agent-sdk/python#claudeagentoptions) (Python) / [`Options`](/docs/fr/agent-sdk/typescript#options) (TypeScript).

<h3 id="turns-and-budget">
  Tours et budget
</h3>

| Option                                         | Ce qu'elle contrôle                          | Valeur par défaut |
| :--------------------------------------------- | :------------------------------------------- | :---------------- |
| Tours max (`max_turns` / `maxTurns`)           | Aller-retours maximum d'utilisation d'outils | Pas de limite     |
| Budget max (`max_budget_usd` / `maxBudgetUsd`) | Coût maximum avant arrêt                     | Pas de limite     |

Lorsque l'une de ces limites est atteinte, le SDK retourne un `ResultMessage` avec un sous-type d'erreur correspondant (`error_max_turns` ou `error_max_budget_usd`). Voir [Gérer le résultat](#handle-the-result) pour savoir comment vérifier ces sous-types et [`ClaudeAgentOptions`](/docs/fr/agent-sdk/python#claudeagentoptions) / [`Options`](/docs/fr/agent-sdk/typescript#options) pour la syntaxe.

Avec [l'entrée en streaming](/docs/fr/agent-sdk/streaming-vs-single-mode), un message que vous envoyez pendant qu'un tour est encore en cours reste en attente lorsque ce tour se termine à la limite de tours max, et il démarre son propre tour avec sa propre limite de tours max. Avant la v2.1.205, un message qui arrivait à l'itération finale du tour pouvait être consommé dans le tour qui se termine et perdu sans jamais atteindre le modèle.

<h3 id="effort-level">
  Niveau d'effort
</h3>

L'option `effort` contrôle la profondeur du raisonnement que Claude applique. Les niveaux d'effort inférieur utilisent moins de tokens par tour et réduisent le coût. Tous les modèles ne supportent pas le paramètre d'effort. Voir [Effort](https://platform.claude.com/docs/fr/build-with-claude/effort) pour savoir quels modèles le supportent.

| Niveau     | Comportement                           | Bon pour                                                                    |
| :--------- | :------------------------------------- | :-------------------------------------------------------------------------- |
| `"low"`    | Raisonnement minimal, réponses rapides | Recherches de fichiers, listage de répertoires                              |
| `"medium"` | Raisonnement équilibré                 | Éditions de routine, tâches standard                                        |
| `"high"`   | Analyse approfondie                    | Refactorisations, débogage                                                  |
| `"xhigh"`  | Profondeur de raisonnement étendue     | Tâches de codage et d'agent ; recommandé sur Fable 5, Opus 4.7+ et Sonnet 5 |
| `"max"`    | Profondeur de raisonnement maximale    | Problèmes multi-étapes nécessitant une analyse approfondie                  |

Si vous ne définissez pas `effort`, les deux SDK laissent le paramètre non défini et s'en remettent au comportement par défaut du modèle.

<Note>
  `effort` échange la latence et le coût des tokens pour la profondeur du raisonnement dans chaque réponse. [Extended thinking](https://platform.claude.com/docs/fr/build-with-claude/extended-thinking) est une fonctionnalité distincte qui produit des blocs de chaîne de pensée visibles dans la sortie. Ils sont indépendants : vous pouvez définir `effort: "low"` avec extended thinking activé, ou `effort: "max"` sans lui.
</Note>

Utilisez un effort inférieur pour les agents effectuant des tâches simples et bien délimitées (comme lister des fichiers ou exécuter un seul grep) pour réduire le coût et la latence. Définissez `effort` dans les options de niveau supérieur `query()` pour la session entière, ou par sous-agent avec le champ `effort` sur [`AgentDefinition`](/docs/fr/agent-sdk/subagents#agentdefinition-configuration) pour remplacer le niveau de session.

<h3 id="permission-mode">
  Mode de permission
</h3>

L'option de mode de permission (`permission_mode` en Python, `permissionMode` en TypeScript) contrôle si l'agent demande une approbation avant d'utiliser les outils :

| Mode                  | Comportement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"default"`           | Les outils non couverts par les règles d'autorisation déclenchent votre rappel d'approbation ; pas de rappel signifie refuser                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `"acceptEdits"`       | Approuve automatiquement les éditions de fichiers et les commandes communes du système de fichiers (`mkdir`, `touch`, `mv`, `cp`, etc.) ; les autres commandes Bash suivent les règles par défaut                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `"plan"`              | Claude explore et planifie sans éditer vos fichiers source ; les éditions de fichiers ne sont jamais approuvées automatiquement et demandent via votre rappel `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `"dontAsk"`           | Ne demande jamais. Les outils pré-approuvés par les [règles de permission](/docs/fr/settings#permission-settings) s'exécutent, tout le reste est refusé. `AskUserQuestion`, les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) sont refusés même si vous les avez autorisés                                                                                                                                                                                                                                        |
| `"auto"`              | Utilise un classificateur de modèle pour approuver ou refuser chaque appel d'outil. Voir [Mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) pour la disponibilité et le comportement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `"bypassPermissions"` | Exécute tous les outils autorisés sans demander, sauf les outils correspondant à une [règle `ask`](/docs/fr/settings#permission-settings) explicite, les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils qui nécessitent une interaction utilisateur ; voir [Comment les permissions sont évaluées](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) pour l'ordre de précédence. Ne peut pas être utilisé lors de l'exécution en tant que root sur Unix. À utiliser uniquement dans des environnements isolés où les actions de l'agent ne peuvent pas affecter les systèmes qui vous intéressent |

Pour les applications interactives, utilisez `"default"` avec un rappel d'approbation d'outil pour afficher les invites d'approbation. Pour les agents autonomes sur une machine de développement, `"acceptEdits"` approuve automatiquement les éditions de fichiers et les commandes communes du système de fichiers (`mkdir`, `touch`, `mv`, `cp`, etc.) tout en gardant les autres commandes `Bash` derrière les règles d'autorisation. Réservez `"bypassPermissions"` pour CI, les conteneurs ou d'autres environnements isolés. Voir [Permissions](/docs/fr/agent-sdk/permissions) pour les détails complets.

<h3 id="model">
  Modèle
</h3>

Si vous ne définissez pas `model`, le SDK utilise la valeur par défaut de Claude Code, qui dépend de votre méthode d'authentification et de votre abonnement. Définissez-le explicitement (par exemple, `model="claude-sonnet-5"`) pour épingler un modèle spécifique ou pour utiliser un modèle plus petit pour des agents plus rapides et moins chers. Voir [modèles](https://platform.claude.com/docs/fr/about-claude/models) pour les ID disponibles.

<h2 id="the-context-window">
  La fenêtre de contexte
</h2>

La fenêtre de contexte est la quantité totale d'informations disponibles pour Claude pendant une session. Elle ne se réinitialise pas entre les tours au sein d'une session. Tout s'accumule : le prompt système, les définitions d'outils, l'historique de conversation, les entrées d'outils et les sorties d'outils. Le contenu qui reste le même entre les tours (prompt système, définitions d'outils, CLAUDE.md) est automatiquement [mis en cache par prompt](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching), ce qui réduit le coût et la latence pour les préfixes répétés.

<h3 id="what-consumes-context">
  Ce qui consomme du contexte
</h3>

Voici comment chaque composant affecte le contexte dans le SDK :

| Source                          | Quand elle se charge                                                             | Impact                                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------------------ | :------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Prompt système**              | Chaque requête                                                                   | Petit coût fixe, toujours présent                                                                                                                                                                                                                                                                                                                                                                       |
| **Fichiers CLAUDE.md**          | Démarrage de session, via [`settingSources`](/docs/fr/agent-sdk/claude-code-features) | Contenu complet dans chaque requête (mais mis en cache par prompt, donc seule la première requête paie le coût complet)                                                                                                                                                                                                                                                                                 |
| **Définitions d'outils**        | Chaque requête ; schémas MCP différés par défaut                                 | Les schémas d'outils intégrés se chargent à chaque requête. La [recherche d'outils](/docs/fr/agent-sdk/mcp#mcp-tool-search) diffère les schémas d'outils MCP par défaut, en revenant au chargement initial sur Google Cloud's Agent Platform ou une `ANTHROPIC_BASE_URL` non-propriétaire. Voir [Configurer la recherche d'outils](/docs/fr/agent-sdk/tool-search#configure-tool-search) pour la matrice complète |
| **Historique de conversation**  | S'accumule au fil des tours                                                      | Croît avec chaque tour : prompts, réponses, entrées d'outils, sorties d'outils                                                                                                                                                                                                                                                                                                                          |
| **Descriptions de compétences** | Démarrage de session, via sources de paramètres                                  | Résumés courts ; le contenu complet se charge uniquement lors de l'invocation                                                                                                                                                                                                                                                                                                                           |

Les grandes sorties d'outils consomment un contexte significatif. Lire un gros fichier ou exécuter une commande avec une sortie détaillée peut utiliser des milliers de tokens en un seul tour. Le contexte s'accumule au fil des tours, donc les sessions plus longues avec de nombreux appels d'outils accumulent beaucoup plus de contexte que les courtes.

<h3 id="automatic-compaction">
  Compaction automatique
</h3>

Lorsque la fenêtre de contexte approche de sa limite, le SDK compacte automatiquement la conversation : il résume l'historique plus ancien pour libérer de l'espace, en gardant vos échanges les plus récents et les décisions clés intacts. Le SDK émet un message avec `type: "system"` et `subtype: "compact_boundary"` dans le flux lorsque cela se produit (en Python c'est un `SystemMessage` ; en TypeScript c'est un type `SDKCompactBoundaryMessage` distinct).

La compaction remplace les messages plus anciens par un résumé, donc les instructions spécifiques du début de la conversation peuvent ne pas être préservées. Les règles persistantes appartiennent à CLAUDE.md (chargé via [`settingSources`](/docs/fr/agent-sdk/claude-code-features)) plutôt qu'au prompt initial, car le contenu CLAUDE.md est réinjecté à chaque requête.

Vous pouvez personnaliser le comportement de compaction de plusieurs façons :

* **Instructions de résumé dans CLAUDE.md :** Le compacteur lit votre CLAUDE.md comme tout autre contexte, donc vous pouvez inclure une section lui indiquant ce qu'il faut préserver lors de la résumé. L'en-tête de section est libre (pas une chaîne magique) ; le compacteur correspond sur l'intention.
* **Hook `PreCompact` :** Exécutez une logique personnalisée avant la compaction, par exemple pour archiver la transcription complète. Le hook reçoit un champ `trigger` (`manual` ou `auto`). Voir [hooks](/docs/fr/agent-sdk/hooks).
* **Compaction manuelle :** Envoyez `/compact` comme chaîne de prompt pour déclencher la compaction à la demande. Les commandes envoyées de cette façon sont des entrées SDK, pas des raccourcis CLI uniquement. Voir [commandes dans le SDK](/docs/fr/agent-sdk/slash-commands).

<Accordion title="Exemple : Instructions de résumé dans CLAUDE.md">
  Ajoutez une section au CLAUDE.md de votre projet indiquant au compacteur ce qu'il faut préserver. Le nom d'en-tête n'est pas spécial ; utilisez n'importe quel label clair.

  ```markdown CLAUDE.md theme={null}
  # Summary instructions

  When summarizing this conversation, always preserve:
  - The current task objective and acceptance criteria
  - File paths that have been read or modified
  - Test results and error messages
  - Decisions made and the reasoning behind them
  ```
</Accordion>

<h3 id="keep-context-efficient">
  Garder le contexte efficace
</h3>

Quelques stratégies pour les agents de longue durée :

* **Utilisez des sous-agents pour les sous-tâches.** Chaque sous-agent commence avec une conversation fraîche (pas d'historique de messages antérieurs, bien qu'il charge son propre prompt système et le contexte au niveau du projet comme CLAUDE.md). Il ne voit pas les tours du parent, et seule sa réponse finale retourne au parent comme résultat d'outil. Le contexte de l'agent principal croît par ce résumé, pas par la transcription complète de la sous-tâche. Voir [Ce que les sous-agents héritent](/docs/fr/agent-sdk/subagents#what-subagents-inherit) pour les détails.
* **Soyez sélectif avec les outils.** Chaque définition d'outil prend de l'espace de contexte. Utilisez le champ `tools` sur [`AgentDefinition`](/docs/fr/agent-sdk/subagents#agentdefinition-configuration) pour délimiter les sous-agents à l'ensemble minimum dont ils ont besoin.
* **Surveillez les coûts des serveurs MCP.** La [recherche d'outils MCP](/docs/fr/agent-sdk/mcp#mcp-tool-search) diffère les schémas d'outils MCP par défaut et les charge à la demande. Lorsque la recherche d'outils est désactivée, sur Google Cloud's Agent Platform, ou derrière une `ANTHROPIC_BASE_URL` non-propriétaire, chaque serveur MCP ajoute tous ses schémas d'outils à chaque requête, donc quelques serveurs avec de nombreux outils peuvent consommer un contexte significatif avant que l'agent ne fasse aucun travail.
* **Utilisez un effort inférieur pour les tâches de routine.** Définissez [effort](#effort-level) à `"low"` pour les agents qui n'ont besoin que de lire des fichiers ou de lister des répertoires. Cela réduit l'utilisation des tokens et le coût.

Pour une ventilation détaillée des coûts de contexte par fonctionnalité, voir [Comprendre les coûts de contexte](/docs/fr/features-overview#understand-context-costs).

<h2 id="sessions-and-continuity">
  Sessions et continuité
</h2>

Chaque interaction avec le SDK crée ou continue une session. Capturez l'ID de session de `ResultMessage.session_id` (disponible dans les deux SDK) pour reprendre plus tard. Le SDK TypeScript l'expose également comme un champ direct sur le `SystemMessage` init ; en Python, il est imbriqué dans `SystemMessage.data`.

Lorsque vous reprenez, le contexte complet des tours précédents est restauré : les fichiers qui ont été lus, l'analyse qui a été effectuée et les actions qui ont été prises. Vous pouvez également forcer une session pour vous brancher dans une approche différente sans modifier l'original.

Voir [Gestion des sessions](/docs/fr/agent-sdk/sessions) pour le guide complet sur les modèles de reprise, de continuation et de bifurcation.

<Note>
  En Python, `ClaudeSDKClient` gère automatiquement les ID de session sur plusieurs appels. Voir la [référence du SDK Python](/docs/fr/agent-sdk/python#choosing-between-query-and-claudesdkclient) pour les détails.
</Note>

<h2 id="handle-the-result">
  Gérer le résultat
</h2>

Lorsque la boucle se termine, le `ResultMessage` vous indique ce qui s'est passé et vous donne la sortie. Le champ `subtype` (disponible dans les deux SDK) est le moyen principal de vérifier l'état de terminaison.

| Sous-type de résultat                 | Ce qui s'est passé                                                                                                                                                                                                         | Champ `result` disponible ? |
| :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------: |
| `success`                             | Claude a terminé la tâche normalement                                                                                                                                                                                      |             Oui             |
| `error_max_turns`                     | Limite de `maxTurns` atteinte avant la fin                                                                                                                                                                                 |             Non             |
| `error_max_budget_usd`                | Limite de `maxBudgetUsd` atteinte avant la fin                                                                                                                                                                             |             Non             |
| `error_during_execution`              | Une erreur a interrompu la boucle (par exemple, une défaillance API ou une requête annulée)                                                                                                                                |             Non             |
| `error_max_structured_output_retries` | Aucune sortie structurée valide n'a été produite dans la limite de tentatives configurée : chaque tentative a échoué la validation, ou un modèle de secours a rétracté la sortie complétée sans nouvelle tentative réussie |             Non             |

Le champ `result` (la sortie textuelle finale) n'est présent que sur la variante `success`, donc vérifiez toujours le sous-type avant de le lire. Tous les sous-types de résultat portent `total_cost_usd`, `usage`, `num_turns` et `session_id` afin que vous puissiez suivre le coût et reprendre même après des erreurs. En Python, `total_cost_usd` et `usage` sont typés comme optionnels et peuvent être `None` sur certains chemins d'erreur, donc gardez-les avant de les formater. Voir [Suivi des coûts et de l'utilisation](/docs/fr/agent-sdk/cost-tracking) pour les détails sur l'interprétation des champs `usage`.

<Note>
  Lorsqu'une requête se termine sur un résultat d'erreur :

  * Un appel `query()` unique produit le message de résultat final, puis lève une erreur qui inclut le texte d'échec, tel que `Reached maximum number of turns`. La levée est intentionnelle — enveloppez la boucle dans un bloc try si votre code doit continuer au-delà. Le processus Claude Code sous-jacent se termine également avec un code non nul.
  * Une session d'entrée en streaming reste active, et vous pouvez continuer à envoyer des messages.
</Note>

Le résultat inclut également un champ `stop_reason` (`string | null` en TypeScript, `str | None` en Python) indiquant pourquoi le modèle a arrêté de générer sur son tour final. Les valeurs courantes sont `end_turn` (le modèle a terminé normalement), `max_tokens` (limite de token de sortie atteinte) et `refusal` (le modèle a refusé la requête). Sur les sous-types de résultat d'erreur, `stop_reason` porte la valeur de la dernière réponse d'assistant avant la fin de la boucle. Pour détecter les refus, vérifiez `stop_reason === "refusal"` (TypeScript) ou `stop_reason == "refusal"` (Python). Voir [`SDKResultMessage`](/docs/fr/agent-sdk/typescript#sdkresultmessage) (TypeScript) ou [`ResultMessage`](/docs/fr/agent-sdk/python#resultmessage) (Python) pour le type complet.

<h2 id="hooks">
  Hooks
</h2>

Les [Hooks](/docs/fr/agent-sdk/hooks) sont des rappels qui se déclenchent à des points spécifiques de la boucle : avant qu'un outil s'exécute, après son retour, lorsque l'agent termine, et ainsi de suite. Certains hooks couramment utilisés sont :

| Hook                             | Quand il se déclenche                        | Utilisations courantes                                 |
| :------------------------------- | :------------------------------------------- | :----------------------------------------------------- |
| `PreToolUse`                     | Avant l'exécution d'un outil                 | Valider les entrées, bloquer les commandes dangereuses |
| `PostToolUse`                    | Après le retour d'un outil                   | Auditer les sorties, déclencher des effets secondaires |
| `UserPromptSubmit`               | Quand un prompt est envoyé                   | Injecter du contexte supplémentaire dans les prompts   |
| `Stop`                           | Quand l'agent termine                        | Valider le résultat, sauvegarder l'état de la session  |
| `SubagentStart` / `SubagentStop` | Quand un sous-agent est généré ou se termine | Suivre et agréger les résultats des tâches parallèles  |
| `PreCompact`                     | Avant la compaction du contexte              | Archiver la transcription complète avant la résumé     |

Les hooks s'exécutent dans le processus de votre application, pas à l'intérieur de la fenêtre de contexte de l'agent, donc ils ne consomment pas de contexte. Les hooks peuvent également court-circuiter la boucle : un hook `PreToolUse` qui rejette un appel d'outil l'empêche de s'exécuter, et Claude reçoit le message de rejet à la place.

Les deux SDK supportent tous les événements ci-dessus. Le SDK TypeScript inclut des événements supplémentaires que Python ne supporte pas encore. Voir [Contrôler l'exécution avec des hooks](/docs/fr/agent-sdk/hooks) pour la liste complète des événements, la disponibilité par SDK et l'API de rappel complète.

<h2 id="put-it-all-together">
  Tout mettre ensemble
</h2>

Cet exemple combine les concepts clés de cette page dans un seul agent qui corrige les tests défaillants. Il configure l'agent avec les outils autorisés (approuvés automatiquement afin que l'agent s'exécute de manière autonome), les paramètres du projet et les limites de sécurité sur les tours et l'effort de raisonnement. Lorsque la boucle s'exécute, elle capture l'ID de session pour une reprise potentielle, gère le résultat final et imprime le coût total.

Parce qu'un appel unique `query()` lève une exception après avoir produit un résultat d'erreur, la boucle est enveloppée dans un bloc try afin que le script se termine proprement lorsqu'une limite est atteinte.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def run_agent():
      session_id = None

      try:
          async for message in query(
              prompt="Find and fix the bug causing test failures in the auth module",
              options=ClaudeAgentOptions(
                  allowed_tools=[
                      "Read",
                      "Edit",
                      "Bash",
                      "Glob",
                      "Grep",
                  ],  # Listing tools here auto-approves them (no prompting)
                  setting_sources=[
                      "project"
                  ],  # Load CLAUDE.md, skills, hooks from current directory
                  max_turns=30,  # Prevent runaway sessions
                  effort="high",  # Thorough reasoning for complex debugging
              ),
          ):
              # Handle the final result
              if isinstance(message, ResultMessage):
                  session_id = message.session_id  # Save for potential resumption

                  if message.subtype == "success":
                      print(f"Done: {message.result}")
                  elif message.subtype == "error_max_turns":
                      # Agent ran out of turns. Resume with a higher limit.
                      print(f"Hit turn limit. Resume session {session_id} to continue.")
                  elif message.subtype == "error_max_budget_usd":
                      print("Hit budget limit.")
                  else:
                      print(f"Stopped: {message.subtype}")
                  if message.total_cost_usd is not None:
                      print(f"Cost: ${message.total_cost_usd:.4f}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branches above have
          # already run; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(run_agent())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Find and fix the bug causing test failures in the auth module",
      options: {
        allowedTools: ["Read", "Edit", "Bash", "Glob", "Grep"], // Listing tools here auto-approves them (no prompting)
        settingSources: ["project"], // Load CLAUDE.md, skills, hooks from current directory
        maxTurns: 30, // Prevent runaway sessions
        effort: "high" // Thorough reasoning for complex debugging
      }
    })) {
      // Save the session ID to resume later if needed
      if (message.type === "system" && message.subtype === "init") {
        sessionId = message.session_id;
      }

      // Handle the final result
      if (message.type === "result") {
        if (message.subtype === "success") {
          console.log(`Done: ${message.result}`);
        } else if (message.subtype === "error_max_turns") {
          // Agent ran out of turns. Resume with a higher limit.
          console.log(`Hit turn limit. Resume session ${sessionId} to continue.`);
        } else if (message.subtype === "error_max_budget_usd") {
          console.log("Hit budget limit.");
        } else {
          console.log(`Stopped: ${message.subtype}`);
        }
        console.log(`Cost: $${message.total_cost_usd.toFixed(4)}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branches above have
    // already run; connection or process failures yield no result message.
    console.log(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous comprenez la boucle, voici où aller en fonction de ce que vous construisez :

* **Vous n'avez pas encore exécuté un agent ?** Commencez par le [démarrage rapide](/docs/fr/agent-sdk/quickstart) pour obtenir le SDK installé et voir un exemple complet s'exécutant de bout en bout.
* **Prêt à vous connecter à votre projet ?** [Chargez CLAUDE.md, les compétences et les hooks du système de fichiers](/docs/fr/agent-sdk/claude-code-features) afin que l'agent suive automatiquement les conventions de votre projet.
* **Construisez une interface utilisateur interactive ?** Activez la [diffusion](/docs/fr/agent-sdk/streaming-output) pour afficher le texte en direct et les appels d'outils lorsque la boucle s'exécute.
* **Avez-vous besoin d'un contrôle plus strict sur ce que l'agent peut faire ?** Verrouillez l'accès aux outils avec les [permissions](/docs/fr/agent-sdk/permissions) et utilisez les [hooks](/docs/fr/agent-sdk/hooks) pour auditer, bloquer ou transformer les appels d'outils avant qu'ils ne s'exécutent.
* **Exécutez des tâches longues ou coûteuses ?** Déléguez le travail isolé aux [sous-agents](/docs/fr/agent-sdk/subagents) pour garder votre contexte principal maigre.

Pour la vue d'ensemble conceptuelle plus large de la boucle d'agent (non spécifique au SDK), voir [Fonctionnement de Claude Code](/docs/fr/how-claude-code-works). Pour un guide pratique de la conception de boucles dans Claude Code, des boucles basées sur les tours aux boucles basées sur les objectifs et proactives, voir [Loop engineering : getting started with loops](https://claude.com/blog/getting-started-with-loops) sur le blog.
