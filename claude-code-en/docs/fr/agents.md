> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Exécuter des agents en parallèle

> Comparez les façons dont Claude Code peut gérer plusieurs tâches à la fois : sous-agents, vue agent, équipes d'agents, workflows dynamiques et projets.

Claude Code a cinq façons de travailler sur plusieurs tâches à la fois : [sous-agents](/docs/fr/sub-agents), [vue agent](/docs/fr/agent-view), [équipes d'agents](/docs/fr/agent-teams), [workflows dynamiques](/docs/fr/workflows) et [projets](/docs/fr/claude-projects). Elles diffèrent par le degré d'implication que vous conservez, allant de diriger chaque conversation vous-même à laisser Claude coordonner un groupe de travailleurs, et selon que le travail s'exécute sur votre machine ou dans le cloud.

| Approche                              | Ce qu'elle vous offre                                                                                                                                                                                                                                                                             | Utilisez-la quand                                                                                                                                                                                                                                                                           |
| :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Sous-agents](/docs/fr/sub-agents)         | Des travailleurs délégués dans une session qui effectuent une tâche secondaire dans leur propre contexte et retournent un résumé                                                                                                                                                                  | Une tâche secondaire inonderait votre conversation principale avec des résultats de recherche, des journaux ou des contenus de fichiers que vous ne référencerez plus                                                                                                                       |
| [Vue agent](/docs/fr/agent-view)           | Un écran pour dispatcher et surveiller les sessions s'exécutant en arrière-plan, ouvert avec `claude agents`. Aperçu de recherche                                                                                                                                                                 | Vous avez plusieurs tâches indépendantes et voulez les déléguer, vérifier l'état en un coup d'œil et intervenir uniquement quand l'une d'elles a besoin de vous                                                                                                                             |
| [Équipes d'agents](/docs/fr/agent-teams)   | Plusieurs sessions coordonnées avec une liste de tâches partagée et une messagerie inter-agents, gérées par un responsable. Expérimental et désactivé par défaut                                                                                                                                  | Vous voulez que Claude divise un projet en morceaux, les assigne et maintient les travailleurs synchronisés                                                                                                                                                                                 |
| [Projets](/docs/fr/claude-projects)        | Une conversation continue sur claude.ai/code ou dans l'application de bureau. Claude démarre des sessions cloud parallèles appelées threads, donne à chacune les référentiels, les instructions et la mémoire du projet, et vous montre lesquels ont besoin de vous. Bêta publique sur Pro et Max | Le travail s'étend sur plusieurs tâches pendant des jours ou des semaines, devrait continuer à s'exécuter quand votre machine est éteinte, et vous préférez le décrire une fois plutôt que de dispatcher et suivre chaque session                                                           |
| [Workflows dynamiques](/docs/fr/workflows) | Un script qui exécute de nombreux sous-agents et vérifie leurs résultats, pour un travail trop important pour être coordonné en un seul tour ou qui nécessite plus d'une seule passe                                                                                                              | Une tâche dépasse une poignée de sous-agents, ou vous voulez que les résultats soient vérifiés les uns par rapport aux autres : un audit à l'échelle de la base de code, une migration de 500 fichiers, une recherche vérifiée de manière croisée, ou un plan élaboré sous plusieurs angles |

Dans chaque approche, les travailleurs sont des sessions Claude. Pour impliquer un outil différent, exposez-le à Claude en tant que [serveur MCP](/docs/fr/mcp).

Trois autres outils soutiennent ce travail sans être une façon d'exécuter des agents eux-mêmes :

* [Les worktrees](/docs/fr/worktrees) donnent à chaque session un checkout git séparé, de sorte que les sessions parallèles ne modifient jamais les mêmes fichiers. Utilisez-les pour les sessions que vous exécutez vous-même. Une session que vous dispatchez depuis la vue agent [se déplace dans son propre worktree avant qu'elle ne modifie les fichiers](/docs/fr/agent-view#how-file-edits-are-isolated), et les sous-agents que vous générez peuvent chacun en obtenir un aussi.
* [La messagerie inter-sessions](/docs/fr/cross-session-messaging) permet à Claude de lister et de communiquer avec vos autres sessions Claude Code sur cette machine, sur une autre machine ou [dans le cloud](/docs/fr/claude-code-on-the-web), de sorte que les sessions que vous exécutez vous-même peuvent transmettre les résultats et l'état entre elles.
* [`/batch`](/docs/fr/commands) est une [compétence](/docs/fr/skills) qui a Claude diviser un grand changement en 5 à 30 sous-agents isolés par worktree. C'est une utilisation packagée de sous-agents et de worktrees, pas un style de coordination séparé.

Quelques autres fonctionnalités exécutent Claude sans que vous conduisiez chaque étape, mais elles résolvent un problème différent de celui de diviser le travail entre les agents :

* Une [commande bash en arrière-plan](/docs/fr/interactive-mode#background-bash-commands) exécute une commande shell sans bloquer la conversation. Elle ne génère pas un agent.
* Un [sous-agent forké](/docs/fr/sub-agents#fork-the-current-conversation) est un sous-agent qui hérite de votre contexte de conversation complet au lieu de commencer à zéro. C'est une façon de générer un sous-agent, pas une surface séparée. Démarrez-en un avec `/subtask`. Claude en génère également un lui-même où le [mode fork](/docs/fr/sub-agents#turn-fork-mode-on-or-off) est activé. Pour copier la session entière dans une nouvelle [session en arrière-plan](/docs/fr/agent-view#from-inside-a-session) qui s'exécute à côté, utilisez `/fork`. Avec la [vue agent désactivée](/docs/fr/agent-view#turn-off-agent-view), la commande de sous-agent forké est `/fork` à la place et `/subtask` n'est pas disponible.
* Une [routine](/docs/fr/routines) exécute une session selon un calendrier dans le cloud, pas en parallèle sur votre machine.

<Note>
  L'exécution de plusieurs sessions ou sous-agents à la fois multiplie l'utilisation des tokens. Consultez [Coûts](/docs/fr/costs) pour les détails d'utilisation et de limite de débit.
</Note>

<h2 id="choose-an-approach">
  Choisir une approche
</h2>

La bonne approche dépend de qui coordonne le travail, si les travailleurs ont besoin de communiquer et s'ils modifient les mêmes fichiers :

* **Qui coordonne le travail ?**
  * Claude délègue et collecte les résultats dans une conversation : [sous-agents](/docs/fr/sub-agents)
  * Vous déléguez des tâches indépendantes et les revérifiez plus tard : [vue agent](/docs/fr/agent-view)
  * Claude planifie, assigne et supervise un groupe de travailleurs : [équipes d'agents](/docs/fr/agent-teams), expérimentales et désactivées par défaut
  * Un script assure la coordination au lieu du jugement tour par tour de Claude : [workflows dynamiques](/docs/fr/workflows). Voir [comment les workflows se comparent aux sous-agents et aux skills](/docs/fr/workflows#when-to-use-a-workflow)
* **Les travailleurs ont-ils besoin de se parler ?** Claude peut transmettre les résultats avec [la messagerie inter-sessions](/docs/fr/cross-session-messaging) entre les sessions que vous exécutez vous-même, y compris les sessions que vous distribuez à partir de la vue agent. Les sous-agents rapportent les résultats à la conversation qui les a générés, et les sessions de vue agent rapportent uniquement à vous. Les coéquipiers dans une équipe d'agents s'envoient des messages directement et, quand ils [disposent des outils Task](/docs/fr/tools-reference#task-tool-availability), partagent une liste de tâches.
* **Les tâches touchent-elles les mêmes fichiers ?** Isolez le travail avec [worktrees](/docs/fr/worktrees). Les sous-agents et les sessions que vous exécutez vous-même peuvent chacun utiliser un worktree séparé. Les équipes d'agents n'isolent pas les coéquipiers dans les worktrees, donc [partitionnez le travail](/docs/fr/agent-teams#avoid-file-conflicts) pour que chaque coéquipier possède un ensemble différent de fichiers.

<h2 id="check-on-running-work">
  Vérifier le travail en cours
</h2>

La commande pour vérifier le travail en cours dépend de l'approche que vous avez utilisée :

* Pour les sessions en arrière-plan, `claude agents` ouvre [la vue agent](/docs/fr/agent-view) : un écran montrant chaque session, son état et lesquelles ont besoin de votre entrée.
* Pour les sous-agents dans la session actuelle, les sous-agents nommés en arrière-plan apparaissent dans la saisie semi-automatique @-mention avec leur statut. À partir de la v2.1.198, `/agents` n'ouvre plus un panneau ; il affiche un avis pointant vers les emplacements des fichiers de sous-agents. Pour [créer et modifier des sous-agents personnalisés](/docs/fr/sub-agents#configure-subagents), demandez à Claude ou modifiez les fichiers directement. Malgré le nom similaire, `/agents` est séparé de `claude agents`.
* Pour tout ce qui s'exécute en arrière-plan de la session actuelle, `/tasks` liste chaque élément et vous permet de le vérifier, de vous y attacher ou de l'arrêter. La liste inclut également les sous-agents qui ont terminé.
* Pour les workflows dynamiques, `/workflows` liste les exécutions en cours et terminées, la phase dans laquelle chacune se trouve et le nombre d'agents qui ont terminé.

Pour une vue de bureau de toutes vos sessions, consultez [les sessions parallèles dans l'application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions).

<h2 id="learn-more">
  En savoir plus
</h2>

Chaque guide ci-dessous couvre la configuration et la mise en place pour une approche :

* [Créer des sous-agents personnalisés](/docs/fr/sub-agents) : définissez des spécialistes réutilisables et contrôlez les outils qu'ils peuvent utiliser.
* [Gérer les agents avec la vue agent](/docs/fr/agent-view) : dispatchez les sessions, regardez leur état et attachez-vous quand l'une d'elles a besoin de vous.
* [Orchestrer les équipes d'agents](/docs/fr/agent-teams) : configurez un responsable et des coéquipiers, assignez des tâches et examinez leur travail.
* [Orchestrer les workflows dynamiques](/docs/fr/workflows) : exécutez un workflow groupé ou laissez Claude en écrire un qui exécute de nombreux sous-agents et vérifie leurs résultats les uns par rapport aux autres.
* [Exécuter les sessions parallèles avec les worktrees](/docs/fr/worktrees) : démarrez Claude dans un checkout isolé, contrôlez ce qui est copié et nettoyez après.
