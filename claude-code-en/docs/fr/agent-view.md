> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gérer plusieurs agents avec la vue agent

> Lancez et gérez plusieurs sessions Claude Code à partir d'un seul écran. La vue agent affiche ce que chaque session fait et lesquelles ont besoin de votre intervention.

La vue agent, ouverte avec `claude agents`, est un seul écran pour toutes vos sessions en arrière-plan : ce qui s'exécute, ce qui a besoin de votre intervention, et ce qui est terminé. Lancez de nouvelles sessions, observez leur état en un coup d'œil au lieu de faire défiler les transcriptions, et intervenez uniquement quand l'une d'elles a besoin de vous. Chaque session en arrière-plan est une conversation Claude Code complète qui continue de s'exécuter sans terminal attaché, vous pouvez donc l'ouvrir, répondre et partir quand vous le souhaitez.

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-light.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=7a186c96ed47d6700d084d77e786be65" className="dark:hidden" alt="Vue agent dans un terminal : l'en-tête affiche Claude Code v2.1.140, le modèle, le répertoire de travail et un résumé du nombre. Les sessions sont regroupées sous Nécessite une intervention, En cours d'exécution et Terminé, avec une entrée de lancement en bas et un pied de page avec des indices de clavier." width="1772" height="780" data-path="images/agent-view-light.png" />

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-dark.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=a5bed7434bae368faea3a8f023b52aa2" className="hidden dark:block" alt="Vue agent dans un terminal : l'en-tête affiche Claude Code v2.1.140, le modèle, le répertoire de travail et un résumé du nombre. Les sessions sont regroupées sous Nécessite une intervention, En cours d'exécution et Terminé, avec une entrée de lancement en bas et un pied de page avec des indices de clavier." width="1772" height="780" data-path="images/agent-view-dark.png" />

Utilisez la vue agent quand vous avez plusieurs tâches indépendantes sur lesquelles Claude peut travailler sans que vous regardiez chaque étape. Lancez une correction de bug, un examen de pull request et une enquête sur un test instable sous forme de trois lignes, continuez à travailler dans une autre fenêtre et vérifiez quand une ligne indique qu'elle a besoin de vous ou qu'elle a un résultat.

Quand vous voulez travailler plus directement dans la session d'un agent, attachez-vous à la ligne pour entrer dans la conversation complète.

Pour comparer la vue agent avec les sous-agents, les équipes d'agents et les worktrees, consultez [Exécuter les agents en parallèle](/docs/fr/agents).

<Note>
  La vue agent est un aperçu de recherche et nécessite Claude Code v2.1.139 ou ultérieur. Vérifiez votre version avec `claude --version`. L'interface et les raccourcis clavier peuvent changer à mesure que la fonctionnalité évolue.
</Note>

Cette page couvre :

* [Démarrage rapide](#quick-start) : donnez à Claude une tâche à accomplir en arrière-plan, vérifiez-la et intervenez quand c'est nécessaire
* [Surveiller les sessions avec la vue agent](#monitor-sessions-with-agent-view), y compris les icônes d'état, l'aperçu et la réponse, l'attachement, l'organisation et les raccourcis clavier
* [Lancer de nouveaux agents](#dispatch-new-agents) à partir de la vue agent, depuis l'intérieur d'une session, ou depuis votre shell
* [Gérer les sessions depuis le shell](#manage-sessions-from-the-shell) avec `claude agents`, `claude attach` et les commandes associées
* [Comment les sessions en arrière-plan sont hébergées](#how-background-sessions-are-hosted) par le processus superviseur

<h2 id="quick-start">
  Démarrage rapide
</h2>

Cette procédure pas à pas couvre la boucle principale de la vue agent : dispatcher une tâche, regarder sa ligne se mettre à jour au fur et à mesure que Claude travaille, jeter un coup d'œil pour vérifier et répondre, et s'attacher pour la conversation complète. La session que vous dispatcher continue de s'exécuter après que vous fermiez la vue agent, vous pouvez donc partir et y revenir.

<Steps>
  <Step title="Ouvrir la vue agent">
    Depuis votre shell, exécutez :

    ```bash theme={null}
    claude agents
    ```

    La vue agent s'ouvre avec une entrée en bas et un tableau qui se remplit au fur et à mesure que les sessions commencent. Appuyez sur `Esc` à tout moment pour revenir à votre shell. Vos sessions continuent de s'exécuter pendant que vous êtes absent et réapparaissent la prochaine fois que vous ouvrez la vue agent.
  </Step>

  <Step title="Dispatcher une session">
    Tapez une invite décrivant une tâche et appuyez sur `Entrée`. Une nouvelle session en arrière-plan démarre sur cette tâche et apparaît sous forme de ligne indiquant si elle fonctionne, attend votre intervention, ou est terminée. La nouvelle session utilise le modèle affiché dans l'en-tête de la vue agent et le même [mode de permission](#permission-mode-model-and-effort) que vous obtiendriez en exécutant `claude` dans ce répertoire.

    Chaque invite que vous entrez ici démarre sa propre nouvelle session. Taper une autre invite et appuyer sur `Entrée` lance une deuxième session aux côtés de la première plutôt que d'envoyer un suivi à celle-ci. Vous pouvez en exécuter plusieurs en parallèle de cette façon.

    Chaque session utilise votre quota d'abonnement indépendamment, consultez donc [Limitations](#limitations) avant de dispatcher plusieurs à la fois.
  </Step>

  <Step title="Jeter un coup d'œil et répondre">
    Sélectionnez une ligne avec les touches fléchées et appuyez sur `Espace` pour ouvrir le panneau d'aperçu. Il affiche la sortie la plus récente de la session, ou la question sur laquelle elle attend, plutôt que la transcription complète. Tapez une réponse et appuyez sur `Entrée` pour l'envoyer sans quitter la vue agent.
  </Step>

  <Step title="S'attacher et se détacher">
    Appuyez sur `Entrée` ou `→` sur une ligne pour vous attacher quand vous voulez la conversation complète. La session prend le contrôle du terminal en tant que session Claude Code interactive complète. Appuyez sur `←` sur une invite vide pour vous détacher et revenir au tableau.
  </Step>

  <Step title="Amener une session existante">
    Cette étape nécessite une session en cours d'exécution. Si vous avez suivi les étapes précédentes, vous n'en avez pas ouverte dans ce terminal, ouvrez donc une session `claude` régulière dans un autre terminal et envoyez-lui d'abord un message. Pour déplacer une session que vous avez déjà ouverte dans la vue agent, exécutez `/bg` à l'intérieur, ou appuyez sur `←` sur une invite vide pour la mettre en arrière-plan et ouvrir la vue agent en une seule étape. La session continue de s'exécuter et apparaît sous forme de ligne aux côtés de celles que vous avez dispatchées.
  </Step>
</Steps>

Vous pouvez utiliser `claude agents` comme point d'entrée principal au lieu de `claude` : dispatcher chaque tâche à partir de la vue agent, vous attacher quand vous voulez la conversation complète, et appuyer sur `←` pour revenir au tableau.

À l'intérieur d'une session `claude` régulière, l'indice `←` du pied de page de l'invite compte les agents en arrière-plan qui attendent votre intervention, comme `← 2 agents`, et revient à `← for agents` quand aucun n'a besoin d'entrée. Les comptages supérieurs à 99 s'affichent comme `99+`. Le comptage se rafraîchit environ toutes les dix secondes lorsque le terminal est actif et immédiatement quand le focus revient. Il change brièvement de couleur quand il se déplace et quand un agent se termine, sauf si le paramètre [`prefersReducedMotion`](/docs/fr/settings#available-settings) est activé, et il est masqué en [mode lecteur d'écran](/docs/fr/accessibility). Sur [Amazon Bedrock, Google Cloud's Agent Platform, et Microsoft Foundry](/docs/fr/third-party-integrations), l'indice reste sous sa forme simple `← for agents` sans le comptage. Nécessite Claude Code v2.1.205 ou ultérieur.

<h2 id="monitor-sessions-with-agent-view">
  Surveiller les sessions avec la vue agent
</h2>

Exécutez `claude agents` pour ouvrir la vue agent. Elle prend le contrôle du terminal complet et répertorie chaque session groupée par état, avec les sessions épinglées et celles qui ont besoin de vous en haut. Chaque ligne affiche le nom de la session, l'activité actuelle, et son ancienneté, comptée à partir du moment où la session a été créée ; l'ancienneté d'une session terminée se fige à la durée de l'exécution.

Le nom est teinté avec la couleur définie par [`/color`](/docs/fr/commands) dans cette session. À partir de la v2.1.199, la couleur se transporte quand vous [mettez une session en arrière-plan](#from-inside-a-session) avec `←` ou `/background`.

Par défaut, la liste affiche chaque session en arrière-plan que vous avez démarrée, dans tous vos projets. Une session travaillant dans un référentiel et une autre dans une worktree différente apparaissent toutes les deux ici, quel que soit le répertoire à partir duquel vous avez ouvert la vue agent. Pour limiter la liste à un projet, passez `--cwd` :

```bash theme={null}
claude agents --cwd ~/projects/my-app
```

Cela affiche uniquement les sessions démarrées sous ce répertoire. Une session qui a [déménagé dans une worktree](#how-file-edits-are-isolated) sous `~/projects/my-app/.claude/worktrees/` compte toujours comme appartenant à `~/projects/my-app`.

Les sessions interactives que vous avez ouvertes dans d'autres terminaux n'apparaissent pas jusqu'à ce que vous les [mettiez en arrière-plan](#from-inside-a-session). Les [sous-agents](/docs/fr/sub-agents) et les [coéquipiers](/docs/fr/agent-teams) qu'une session génère ne sont pas répertoriés comme des lignes séparées.

```text theme={null}
Épinglées
  ✽ clawd walk cycle          Drawing the walk-cycle sprite frames          3m

Prêtes pour examen
  ∙ jump physics              Opened PR with collision fix                 #2048  2h

Nécessite une intervention
  ✻ power-up design           double jump or wall climb?                    1m

En cours
  ✽ collision detection       Adding swept-AABB checks to CollisionSystem   2m
  ✢ playtest level 3          run 12 · all checkpoints cleared           in 4m

Terminées
  ✻ title screen              result: menu, options, and credits done       9m
  ∙ sound effects             result: 14 SFX exported to assets/audio       4h
  … 6 more
```

<h3 id="read-session-state">
  Lire l'état de la session
</h3>

Chaque ligne commence par une icône dont la couleur et l'animation montrent l'état de la session :

| État                       | L'icône s'affiche comme | Ce que cela signifie                                                              |
| :------------------------- | :---------------------- | :-------------------------------------------------------------------------------- |
| En cours                   | Animée                  | Claude exécute activement des outils ou génère une réponse                        |
| Nécessite une intervention | Jaune                   | Claude attend une question spécifique ou une décision de permission de votre part |
| Inactif                    | Estompé                 | La session n'a rien à faire et est prête pour votre prochain message              |
| Terminée                   | Vert                    | La tâche s'est terminée avec succès                                               |
| Échouée                    | Rouge                   | La tâche s'est terminée avec une erreur                                           |
| Arrêtée                    | Gris                    | La session a été arrêtée avec `Ctrl+X` ou `claude stop`                           |

Séparément, la forme de l'icône indique si le processus sous-jacent s'exécute :

| Forme            | Ce que cela signifie                                                                                                                        |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `✻` ou `✽` animé | Le processus de la session est actif et répond immédiatement                                                                                |
| `∙`              | Le processus a quitté. Vous pouvez toujours apercevoir, répondre, ou vous attacher, et Claude redémarre à partir de là où il s'était arrêté |
| `✢`              | Une session [`/loop`](/docs/fr/scheduled-tasks) dormant entre les itérations. La ligne affiche son nombre d'exécutions et un compte à rebours    |

L'étiquette `#N` qui peut apparaître au bord droit d'une ligne est la [pull request que la session est liée à](#pull-request-status), pas une partie de l'icône d'état.

Le titre de l'onglet du terminal affiche le nombre d'entrées en attente pendant que la vue agent est ouverte : `2 awaiting input · claude agents` quand les sessions ont besoin d'une intervention, ou `claude agents` quand ce n'est pas le cas.

À partir de la v2.1.198, pendant que la vue agent est ouverte, Claude Code envoie également une notification via votre [canal de notification de terminal configuré](/docs/fr/terminal-config#get-a-terminal-bell-or-notification) quand une session en arrière-plan local commence à avoir besoin de votre intervention, se termine, ou échoue. Les sessions qui s'exécutent selon un calendrier, comme les sessions [`/loop`](/docs/fr/scheduled-tasks), ne notifient que quand elles ont besoin de votre intervention. Les notifications utilisent le même paramètre [`preferredNotifChannel`](/docs/fr/settings#available-settings) que le reste de Claude Code et déclenchent le hook [`Notification`](/docs/fr/hooks#notification) avec le type `agent_needs_input` ou `agent_completed`.

Les sessions en arrière-plan n'ont besoin d'aucun terminal ouvert pour continuer à fonctionner. Un [processus superviseur](#the-supervisor-process) séparé les exécute, vous pouvez donc fermer la vue agent, fermer votre shell, ou démarrer une nouvelle session interactive et votre travail lancé continue.

L'état de la session persiste sur le disque via les mises à jour automatiques et les redémarrages du superviseur. Les sessions sont également préservées quand votre machine se met en veille. Leurs processus reprennent au réveil et le superviseur se reconnecte à eux au lieu de traiter l'écart de temps comme inactif. L'arrêt arrête toujours les sessions en cours ; consultez [Les sessions s'affichent comme échouées après l'arrêt](#sessions-show-as-failed-after-shutdown) pour savoir comment les récupérer.

Quand vous ouvrez une session qui a cessé de répondre, le superviseur redémarre son processus et la session continue la réponse interrompue à partir de là où elle s'était arrêtée. Une session peut se retrouver dans cet état quand la machine se met en veille pendant qu'elle est en train de répondre. Nécessite Claude Code v2.1.200 ou ultérieur.

<h3 id="row-summaries">
  Résumés des lignes
</h3>

Le résumé d'une ligne dans chaque ligne est généré par un [modèle de classe Haiku](/docs/fr/model-config) afin que la ligne puisse vous dire ce que la session fait, ce qu'elle a besoin, ou ce qu'elle a produit sans ouvrir la transcription. Pendant qu'une session fonctionne activement, le texte de la ligne se met à jour au maximum une fois toutes les 15 secondes à partir de la sortie récente de la session sans envoyer une demande de modèle, et le modèle écrit un résumé frais quand chaque tour se termine.

Une ligne de travail affiche ce que la session dit qu'elle fait, et une ligne bloquée affiche la question qu'elle pose. Pendant un long tour, le modèle réécrit également le résumé environ une fois par minute, attendant deux fois plus longtemps après chaque réécriture jusqu'à quatre minutes, donc une ligne occupée n'affiche pas continuellement un résumé obsolète. Le texte du résumé remplit la largeur restante de la ligne et ne se tronque qu'au bord droit du terminal ; ouvrez le [panneau d'aperçu](#peek-and-reply) pour lire une phrase que le bord coupe. Avant la v2.1.206, le texte était coupé à 64 colonnes quel que soit la largeur du terminal.

Quand la liste est [groupée par répertoire](#organize-the-list), le résumé s'ouvre avec l'état de la session en tant que mot coloré, comme `Needs input · double jump or wall climb?`. Dans le groupement d'état par défaut, l'en-tête du groupe nomme déjà l'état, donc la ligne affiche uniquement le résumé. Avant la v2.1.205, les lignes groupées par répertoire ne portaient aucun mot d'état.

Un tour dont la sortie entière ne contient aucune lettre ni chiffre, comme une session [`/loop`](/docs/fr/scheduled-tasks) qui imprime un seul symbole sur une itération silencieuse, conserve le résumé et l'état précédents de la ligne. Avant la v2.1.205, ce tour a été reclassé et pouvait faire basculer une session qui attendait votre intervention vers `En cours`.

Le résumé de fin de tour et chaque réécriture à mi-tour sont une courte demande de classe Haiku via votre fournisseur normal, facturée et traitée selon les mêmes [conditions d'utilisation des données](/docs/fr/data-usage) que la session elle-même. Sur les fournisseurs tiers tels que Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, et les passerelles personnalisées, la demande revient au modèle principal de la session quand aucun modèle Haiku n'est configuré. Définissez [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/fr/model-config#environment-variables) pour choisir le modèle pour ces résumés sur ces fournisseurs.

<h3 id="pull-request-status">
  Statut de la pull request
</h3>

Quand une session ouvre une pull request, une étiquette `#1234` apparaît au bord droit de la ligne, liée à la pull request dans les terminaux qui supportent les hyperliens. L'étiquette persiste quand vous envoyez un suivi à la session, donc la pull request reste visible pendant que la ligne revient à la progression en direct. Les sessions en arrière-plan qui ont isolé leurs modifications dans une worktree ouvrent elles-mêmes ces pull requests ; [Comment les modifications de fichiers sont isolées](#how-file-edits-are-isolated) couvre quand cela se produit et ce qu'une session ne fait jamais sans demander.

Une session qui travaille sur une pull request existante y est liée de la même manière. Éditer, commenter, fermer, ou marquer une pull request comme prête avec `gh` lie la pull request que la sortie de la commande elle-même nomme, donc une commande `gh` dont la sortie capturée ne nomme aucune pull request ne crée pas de lien ; `gh pr merge` est le cas courant, car il imprime son résultat uniquement sur un terminal interactif. Vérifier une pull request avec `gh pr checkout`, ou pousser vers une branche qui a une pull request ouverte, la lie en cherchant cette branche avec `gh pr view` à la place. Avant la v2.1.205, seules les pull requests que la session a créées ou vérifiées étaient liées, et une poussée en liait une uniquement quand le nom de la branche locale correspondait.

Claude Code lit la pull request à partir de la sortie complète de la commande, y compris la partie enregistrée dans un fichier quand la sortie d'une commande dépasse la limite en ligne. Avant la v2.1.205, une pull request créée dans un appel Bash dont la sortie dépassait environ 30 000 caractères n'était pas liée.

Quand une session est liée à plus d'une pull request, l'étiquette affiche un nombre à la place, comme `3 PRs`, colorée par la pull request ouverte qui a le plus besoin d'attention. Ouvrez le [panneau d'aperçu](#peek-and-reply) pour les voir toutes.

Le numéro de la pull request est coloré par son statut :

| Couleur | Statut de la pull request                                                |
| :------ | :----------------------------------------------------------------------- |
| Jaune   | En attente de vérifications ou d'examen, ou les vérifications ont échoué |
| Vert    | Les vérifications ont réussi et aucun examen ne bloque                   |
| Violet  | Fusionnée                                                                |
| Gris    | Brouillon ou fermée                                                      |

Pour la plupart des tâches, cette colonne est l'endroit où vous collectez le résultat : examinez et fusionnez la pull request quand son numéro devient vert.

<h3 id="peek-and-reply">
  Aperçu et réponse
</h3>

Appuyez sur `Espace` sur une ligne sélectionnée pour ouvrir le panneau d'aperçu. Il s'ouvre avec la phrase que la ligne tronque au bord du terminal, et quelle phrase c'est dépend de l'état de la session :

* Une session qui attend votre intervention : la question exacte qu'elle pose, au-dessus de l'entrée de réponse
* Une session terminée : son résultat
* Une session en cours : sa phrase d'état complète

Toutes les pull requests liées à la session sont répertoriées ensuite. Pour une session qui attend votre intervention, une ligne comme `waiting 3m` en dessous d'elles montre depuis combien de temps elle attend, et c'est la seule fois affichée dans le panneau. L'ancienneté au bord droit de la ligne est un nombre différent : elle compte à partir du moment où la session a démarré.

La plupart du temps, le panneau d'aperçu est suffisant et vous n'avez pas besoin d'ouvrir la transcription complète.

Avant la v2.1.207, chaque aperçu s'ouvrait avec la phrase d'état et un horodatage nu, et une session bloquée avait sa question affichée en dessous d'eux préfixée avec le même horodatage une deuxième fois.

Tapez une réponse dans le panneau d'aperçu et appuyez sur `Entrée` pour l'envoyer à cette session. Quand la session pose une question à choix multiples, le panneau d'aperçu affiche les options et vous pouvez appuyer sur une touche numérique pour en choisir une. Pour les autres sessions bloquées, appuyez sur `Tab` pour remplir l'entrée avec une réponse suggérée que vous pouvez modifier avant d'envoyer. Préfixez une réponse avec `!` pour envoyer une commande Bash à la place.

Une réponse qui ne peut pas être livrée, parce que le service en arrière-plan est inaccessible ou l'envoi échoue, est enregistrée et envoyée à la session comme son prochain message quand son processus redémarre, et le message d'erreur dit que la réponse a été enregistrée. Une réponse préfixée avec `!` n'est pas enregistrée, car le texte enregistré atteindrait la session comme un message simple plutôt que de s'exécuter comme une commande Bash.

Avec la [dictée vocale](/docs/fr/voice-dictation) activée, maintenez ou appuyez sur votre touche push-to-talk pendant que l'entrée de réponse est active pour dicter une réponse au lieu de la taper. La même chose fonctionne dans l'entrée de lancement en bas de la vue agent.

Utilisez `↑` et `↓` pour apercevoir les sessions adjacentes sans fermer le panneau, ou `→` pour vous attacher.

<h3 id="attach-to-a-session">
  S'attacher à une session
</h3>

Appuyez sur `Entrée` ou `→` sur une ligne sélectionnée pour vous attacher. La vue agent est remplacée par la session interactive complète. Quand vous vous attachez, Claude affiche un court récapitulatif de ce qui s'est passé pendant que vous étiez absent.

Pendant que vous êtes attaché, la session se comporte comme n'importe quelle autre session Claude Code : les [commandes](/docs/fr/commands), raccourcis clavier, et fonctionnalités fonctionnent tous, avec les exceptions ci-dessous.

Une session en arrière-plan refuse `/install-github-app` et la liste des paramètres [`/mcp`](/docs/fr/mcp), y compris ses actions d'authentification, que vous soyez attaché ou que vous répondiez à partir du panneau d'aperçu. Le message vous dirige vers une session `claude` régulière, et `/mcp reconnect <server>`, `/mcp enable`, et `/mcp disable` fonctionnent toujours.

Les sessions attachées s'affichent toujours en [mode plein écran](/docs/fr/fullscreen), quel que soit votre paramètre `tui`, car une session en arrière-plan n'a pas d'historique de terminal à ajouter. Faites défiler avec `PgUp`, `PgDn`, ou la molette de la souris, et appuyez sur `Ctrl+O` pour le mode transcription. Le défilement natif de votre terminal et le mode copie tmux affichent uniquement la fenêtre d'affichage actuelle, comme lorsque vous exécutez n'importe quelle application plein écran.

À partir de la v2.1.198, appuyez sur `←` sur une invite vide, ou exécutez `/exit`, pour vous détacher et revenir à la vue agent. À partir de la v2.1.198, cela fonctionne de la même manière que vous ayez ouvert la session à partir de la vue agent ou avec `claude attach <id>` à partir de votre shell.

`Ctrl+Z` se détache également mais revient à l'endroit où vous avez commencé à la place : la vue agent si vous vous êtes attaché à partir de là, ou votre shell si vous avez exécuté `claude attach`. Utilisez `Ctrl+Z` quand une boîte de dialogue a le focus et ne répond pas à `←`.

`Ctrl+C` conserve son comportement d'interruption standard pendant que vous êtes attaché : il annule une réponse en cours d'exécution ou une commande shell `!` plutôt que de vous détacher. Appuyer sur `Ctrl+C` deux fois sur une invite vide vous détache, comme dans n'importe quelle session.

Se détacher n'arrête jamais une session en arrière-plan : `←`, `Ctrl+Z`, `/exit`, et double `Ctrl+C` ou double `Ctrl+D` la laissent toutes s'exécuter. Pour terminer une session depuis l'intérieur, exécutez `/stop`.

Dans une session s'exécutant au premier plan, une que vous avez démarrée dans le terminal plutôt que de vous y attacher à partir de la vue agent, appuyer sur `←` sur une invite vide la met en arrière-plan et ouvre la vue agent avec cette ligne sélectionnée, vous pouvez donc basculer entre les sessions sans quitter le terminal. La même pression unique détache une session attachée.

Si un outil s'exécute quand vous appuyez sur `←`, Claude Code attend jusqu'à environ dix secondes pour qu'il se termine avant de mettre en arrière-plan, et la réponse continue dans la session en arrière-plan. Appuyez sur `←` à nouveau pour mettre en arrière-plan immédiatement au lieu d'attendre. Quand le travail en cours ne peut pas se transférer à la session en arrière-plan, la boîte de dialogue `Background this session?` apparaît d'abord, la même que avec [`/background`](#from-inside-a-session).

La limite de dix secondes ne s'applique pas pendant que les [sous-agents](/docs/fr/sub-agents) s'exécutent. Claude Code continue d'attendre pour que leur travail se transporte, et affiche un avis `Still backgrounding after the current tool` pendant qu'il attend ; appuyez sur `←` à nouveau pour mettre en arrière-plan sans attendre, ce qui redémarre les sous-agents depuis le début. Avant la v2.1.203, l'attente s'est terminée après dix secondes et les sous-agents en cours d'exécution ont été redémarrés depuis le début sans avertissement.

La ligne est créée même à partir d'une session nouvelle sans historique de conversation, donc `→` y revient. Avant la v2.1.203, la vue agent affichait un indice d'intégration en dessous de cette ligne quand elle était la seule.

Vous pouvez désactiver ce raccourci avec le paramètre `leftArrowOpensAgents` dans `/config`.

<h3 id="organize-the-list">
  Organiser la liste
</h3>

La vue agent groupe les sessions afin que celles qui ont besoin d'une intervention soient en haut, avec `Prêtes pour examen` et `Nécessite une intervention` au-dessus de `En cours` et `Terminées`. Ces noms de groupe ne correspondent pas un-à-un aux [états](#read-session-state) ci-dessus : une session se déplace vers `Prêtes pour examen` quand elle a une pull request ouverte, et `Terminées` collecte les sessions terminées, échouées et arrêtées ensemble.

Appuyez sur `Ctrl+S` pour grouper par répertoire à la place. Votre choix persiste entre les exécutions.

Dans un groupe :

* Appuyez sur `Ctrl+T` pour épingler une session en haut et [garder son processus en cours d'exécution](#the-supervisor-process) pendant l'inactivité
* Appuyez sur `Shift+↑` ou `Shift+↓` pour réorganiser les sessions
* Appuyez sur `Ctrl+R` pour renommer une session
* Appuyez sur `Entrée` sur un en-tête de groupe pour le réduire

Pour supprimer une session de la liste, appuyez sur `Ctrl+X` pour l'arrêter et `Ctrl+X` à nouveau dans les deux secondes pour la supprimer. Appuyer sur `Ctrl+X` sur un en-tête de groupe supprime chaque session de ce groupe après confirmation.

La suppression supprime la session de la vue agent. Si Claude a [créé une worktree](#how-file-edits-are-isolated) pour la session, la suppression supprime également cette worktree, y compris toutes les modifications non validées qu'elle contient, donc poussez ou validez le travail que vous voulez conserver en premier. Une worktree que vous avez créée vous-même et dans laquelle vous avez démarré la session est laissée en place. La transcription de conversation reste sur votre machine locale et reste disponible via `claude --resume`.

La suppression n'enlève jamais une worktree avec des commits qui ne sont poussés nulle part, ou une qui est revendiquée ou verrouillée par une autre session en cours d'exécution. Claude Code garde la worktree et la session, et le pied de page nomme le chemin conservé et la raison. Poussez les commits, ou fermez l'autre session, puis supprimez à nouveau.

La suppression efface également la session de la [liste de sessions du superviseur](#the-supervisor-process), que vous supprimiez avec `Ctrl+X` ou avec [`claude rm`](#manage-sessions-from-the-shell) à partir du shell, donc la suppression persiste entre les redémarrages du superviseur. Avant la v2.1.206, supprimer une session pendant que le superviseur redémarrait ou était inaccessible la laissait dans cette liste, et le superviseur suivant redémarrait son processus et affichait la ligne à nouveau.

Les sessions terminées qui ne tiennent pas à l'écran se replient dans une ligne `… N more`. Les échecs et les sessions avec une pull request ouverte restent toujours visibles. Le groupe `Completed` remplit l'espace vertical restant après les groupes actifs, et sur un terminal court, l'en-tête se compacte en une seule ligne de résumé pour que les sessions qui fonctionnent ou ont besoin d'une intervention restent visibles.

<h3 id="filter-sessions">
  Filtrer les sessions
</h3>

Tapez dans l'entrée de lancement pour filtrer au lieu de lancer :

| Filtre                     | Affiche                                                                                                      |
| :------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `a:<name>`                 | Sessions exécutant l'agent nommé                                                                             |
| `s:<state>`                | Sessions dans l'état donné, comme `s:working`. Accepte également `s:blocked` pour tout ce qui attend de vous |
| `#<number>` ou une URL PR  | La session travaillant sur cette pull request                                                                |
| N'importe quelle autre URL | La session dont la première invite contenait cette URL                                                       |

<h3 id="keyboard-shortcuts">
  Raccourcis clavier
</h3>

Appuyez sur `?` dans la vue agent pour voir chaque raccourci en contexte. Le tableau ci-dessous les résume.

| Raccourci             | Action                                                                                             |
| :-------------------- | :------------------------------------------------------------------------------------------------- |
| `↑` / `↓`             | Se déplacer entre les lignes                                                                       |
| `Entrée`              | S'attacher à la session sélectionnée, ou lancer si du texte est dans l'entrée                      |
| `Espace`              | Ouvrir ou fermer le panneau d'aperçu pour la session sélectionnée                                  |
| `Shift+Entrée`        | Lancer et s'attacher immédiatement                                                                 |
| `→`                   | S'attacher à la session sélectionnée                                                               |
| `Alt+1`..`Alt+9`      | S'attacher à la session 1–9 dans le répertoire de la session active                                |
| `Tab`                 | Sur une entrée vide, parcourir tous les sous-agents. Sinon appliquer la suggestion en surbrillance |
| `Ctrl+S`              | Basculer le groupement entre l'état et le répertoire                                               |
| `Ctrl+T`              | Épingler ou dépingler la session sélectionnée                                                      |
| `Ctrl+R`              | Renommer la session sélectionnée                                                                   |
| `Ctrl+G`              | Ouvrir l'invite de lancement dans votre `$VISUAL` ou `$EDITOR`                                     |
| `Ctrl+X`              | Arrêter la session ; appuyez à nouveau dans les deux secondes pour la supprimer                    |
| `Shift+↑` / `Shift+↓` | Réorganiser la session sélectionnée                                                                |
| `Esc`                 | Fermer le panneau d'aperçu, effacer l'entrée, ou quitter                                           |
| `Ctrl+C`              | Effacer l'entrée ; appuyez deux fois pour quitter                                                  |
| `?`                   | Afficher tous les raccourcis                                                                       |

<h2 id="dispatch-new-agents">
  Lancer de nouveaux agents
</h2>

Vous pouvez lancer de nouvelles sessions en arrière-plan à partir de la vue agent, envoyer une session interactive existante en arrière-plan, ou en démarrer une directement depuis le shell.

<h3 id="from-agent-view">
  À partir de la vue agent
</h3>

Tapez une invite dans l'entrée en bas de la vue agent et appuyez sur `Entrée` pour démarrer une nouvelle session en arrière-plan. La session est nommée automatiquement à partir de l'invite ; renommez-la plus tard avec `Ctrl+R`.

Un nom que la session obtient plus tard apparaît également sur sa ligne, y compris le nom que Claude dérive quand vous [acceptez un plan](/docs/fr/permission-modes#review-and-approve-a-plan) dans cette session. Avant la v2.1.207, une session en arrière-plan nommée en acceptant un plan affichait ce nom dans `/status` mais pas sur sa ligne de vue agent jusqu'à ce que vous la renommiez vous-même.

Collez une image dans l'invite pour inclure une capture d'écran ou un diagramme avec la tâche.

Le texte collé plus long que 800 caractères ou plus de deux lignes s'effondre en un espace réservé `[Pasted text #N]` pour que l'entrée reste sur une ligne ; le texte complet est envoyé quand vous lancez. Pour examiner ou modifier le texte effondré avant de lancer, collez le même texte à nouveau et l'espace réservé se développe dans l'entrée. Un rappel `paste again to expand` apparaît sous l'entrée pendant quelques secondes après le collage sur les terminaux d'au moins 90 colonnes de large. Avant la v2.1.207, coller le même texte à nouveau ajoutait un deuxième espace réservé au lieu de développer le premier.

Préfixez ou mentionnez des parties de l'invite pour contrôler comment la session démarre :

| Entrée                                 | Effet                                                                                                                                                                                                 |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<agent-name> <prompt>`                | Si le premier mot correspond à un nom de [sous-agent](/docs/fr/sub-agents) personnalisé, ce sous-agent s'exécute comme l'agent principal de la session avec la configuration de son frontmatter            |
| `@<agent-name>`                        | Mentionnez un sous-agent personnalisé n'importe où dans l'invite pour l'exécuter comme l'agent principal                                                                                              |
| `@<repo>`                              | Mentionnez un référentiel pour exécuter la session là. Voir [Lancer vers un répertoire spécifique](#dispatch-to-a-specific-directory) pour savoir quels référentiels sont listés                      |
| `/<command>`                           | Suggérez des [skills](/docs/fr/skills) et des [commandes](/docs/fr/commands) à lancer comme l'invite                                                                                                            |
| `! <command>`                          | Exécutez une commande shell comme une tâche en arrière-plan au lieu de démarrer une session Claude. La tâche apparaît comme une ligne à laquelle vous pouvez vous attacher, regarder et vous détacher |
| `#<number>` ou une URL de pull request | Si une session travaille déjà sur cette PR, sélectionnez-la au lieu de lancer                                                                                                                         |
| `Shift+Entrée`                         | Lancer et s'attacher immédiatement à la nouvelle session                                                                                                                                              |

Un petit ensemble de commandes s'exécutent dans la vue agent elle-même au lieu de lancer :

* `/exit` et `/quit` ferment la vue agent
* `/logout` vous déconnecte
* `/model` définit le [modèle de lancement](#set-the-model)
* À partir de la v2.1.198, `/login` ouvre la boîte de dialogue de connexion pour que vous puissiez vous connecter à nouveau sans vous attacher à une session

Les skills, vos propres commandes, et les built-ins qui développent les invites comme `/init` sont envoyés à une nouvelle session en arrière-plan comme leur première invite. Les autres commandes built-in affichent plutôt un indice `attach to a session to run it`. Tout ce que vous avez tapé reste dans l'entrée à côté de l'indice pour que vous puissiez le modifier. Avant la v2.1.203, l'indice effaçait l'entrée et le texte tapé était perdu.

Empaqueter une tâche récurrente comme un [skill](/docs/fr/skills) vous permet de démarrer le même workflow à partir de la vue agent à plusieurs reprises sans retaper l'invite.

Quand le même `@name` correspond à la fois à un sous-agent et à un référentiel frère, le sous-agent prend la priorité. La correspondance du premier mot sans `@` s'applique également, donc une invite qui commence par l'un de vos noms de sous-agent lance ce sous-agent plutôt que de traiter le mot comme du texte brut. Utilisez la forme `@` quand vous voulez être explicite, ou commencez l'invite par un mot différent pour éviter la correspondance.

<h4 id="dispatch-to-a-specific-directory">
  Lancer vers un répertoire spécifique
</h4>

Une nouvelle session s'exécute dans le répertoire à partir duquel vous avez ouvert la vue agent. Pour cibler un répertoire différent, utilisez l'un de ceux-ci :

* Ouvrez `claude agents` dans ce répertoire.
* Ouvrez `claude agents` dans un répertoire parent et mentionnez un référentiel enfant avec `@<repo>` dans l'invite. Taper `@` liste ces cibles :

  * Les référentiels Git un niveau en dessous du répertoire de lancement
  * Les [git worktrees](/docs/fr/worktrees) enregistrés du référentiel à partir duquel vous avez lancé qui vivent à l'intérieur de son arborescence de répertoires, comme ceux que Claude crée sous `.claude/worktrees/`, étiquetés avec leur branche extraite. Les worktrees ajoutés en dehors du référentiel, comme avec `git worktree add ../feature`, ne sont pas listés
  * Tout répertoire qui a déjà une session dans la liste

  Un répertoire dont le nom contient un espace n'est pas listé. Avant la v2.1.203, les worktrees enregistrés n'étaient pas listés, donc lancer dans l'un d'eux signifiait exécuter `claude --bg` à partir du répertoire de ce worktree.
* Depuis le shell, `cd` dans le répertoire et exécutez `claude --bg "<prompt>"`.

Quand la vue agent est groupée par répertoire, le répertoire de la ligne en surbrillance devient la cible de lancement, vous pouvez donc faire défiler jusqu'à un groupe et lancer dedans sans retaper le chemin.

<h3 id="from-inside-a-session">
  À partir d'une session
</h3>

Exécutez `/background` ou son alias `/bg` pour déplacer la conversation actuelle dans une session en arrière-plan. Passez une invite comme `/bg run the test suite and fix any failures` pour donner une instruction supplémentaire d'abord. Si Claude répond quand vous exécutez `/bg`, la réponse continue dans la session en arrière-plan.

Quitter une session interactive qui a encore du travail en arrière-plan en cours d'exécution, comme des sous-agents, des commandes shell en arrière-plan, des workflows, ou des [monitors](/docs/fr/tools-reference#monitor-tool), affiche une boîte de dialogue `Background work is running` au lieu de quitter immédiatement. À partir de la v2.1.198, la boîte de dialogue propose `Move to background and exit` aux côtés de `Exit anyway` et `Stay`. La choisir déplace la session en arrière-plan de la même manière que `/background` le fait, puis vous ramène à votre shell, donc le travail qui peut se poursuivre continue de s'exécuter et la session apparaît dans la vue agent. L'option n'est pas affichée quand la vue agent est [désactivée](#turn-off-agent-view).

Mettre en arrière-plan à partir d'une session interactive démarre un processus nouveau qui reprend à partir de la conversation enregistrée, et le travail en cours se transfère à celui-ci : les commandes shell en arrière-plan en cours d'exécution, les sous-agents mis en arrière-plan, les workflows dynamiques, et les tâches planifiées que vous avez créées avec [`/loop`](/docs/fr/scheduled-tasks) se transfèrent à la session en arrière-plan et continuent de s'exécuter là. Un sous-agent se déplace avec tout ce qu'il a démarré, donc il se transfère uniquement quand tout ce travail peut se transférer aussi, y compris sur Windows. Pour arrêter le travail en cours au lieu de le transférer, définissez la variable d'environnement [`CLAUDE_DISABLE_ADOPT=1`](/docs/fr/env-vars#variables) ; Claude Code vous demande alors de confirmer avant de mettre en arrière-plan.

Le travail qui ne peut pas se transférer, comme un [monitor](/docs/fr/tools-reference#monitor-tool) en cours d'exécution, est arrêté. Un sous-agent mis en arrière-plan qui possède un monitor est arrêté avec lui. Quand un tel travail s'exécute, Claude Code affiche une boîte de dialogue `Background this session?` pour que vous puissiez confirmer avant qu'il soit arrêté.

Une fois en arrière-plan, la session peut démarrer de nouveaux sous-agents, monitors, et commandes en arrière-plan, et ceux-ci continuent de s'exécuter lors des détachements et réattachements ultérieurs.

Les drapeaux de configuration de la session de lancement d'origine se reportent à la session mise en arrière-plan, donc ses serveurs MCP, paramètres et modèle de secours restent en vigueur :

* `--mcp-config` et `--strict-mcp-config`
* `--settings`
* `--add-dir`
* `--plugin-dir`
* `--fallback-model`
* `--allow-dangerously-skip-permissions`

Les répertoires que vous avez ajoutés pendant la session avec [`/add-dir`](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) se reportent également.

Reporter `--allow-dangerously-skip-permissions` maintient `bypassPermissions` accessible dans la session mise en arrière-plan, mais cela ne confère rien de nouveau. Le mode nécessite toujours la même acceptation interactive unique décrite dans [Mode de permission, modèle et effort](#permission-mode-model-and-effort) avant que toute session puisse l'utiliser.

<h3 id="from-your-shell">
  À partir du shell
</h3>

Passez `--bg` ou sa forme longue `--background` pour démarrer une session qui va directement en arrière-plan :

```bash theme={null}
claude --bg "investigate the flaky SettingsChangeDetector test"
```

L'invite est l'argument positionnel, pas une valeur `-p`. À partir de la v2.1.198, combiner `--bg` avec `-p` ou `--print` est rejeté avec une erreur avant que toute session soit créée, car `--print` ne démarre jamais la session interactive à laquelle `claude agents` s'attache.

Pour exécuter un sous-agent spécifique comme l'agent principal de la session, combinez `--bg` avec `--agent` :

```bash theme={null}
claude --agent code-reviewer --bg "address review comments on PR 1234"
```

Passez `--name` pour définir le nom d'affichage de la session dans la vue agent au lieu du nom généré automatiquement :

```bash theme={null}
claude --bg --name "flaky-test-fix" "investigate the flaky SettingsChangeDetector test"
```

Après avoir mis en arrière-plan, Claude affiche l'ID court de la session et les commandes pour la gérer. Quand le service qui héberge les sessions en arrière-plan n'est pas déjà en cours d'exécution, `--bg` peut d'abord afficher `Starting background service…` au-dessus de cette sortie. Quand vous passez `--name`, le nom apparaît après l'ID court :

```text theme={null}
backgrounded · 7c5dcf5d · flaky-test-fix
  claude agents             list sessions
  claude attach 7c5dcf5d    open in this terminal
  claude logs 7c5dcf5d      show recent output
  claude stop 7c5dcf5d      stop this session
```

<h4 id="run-a-shell-command">
  Exécuter une commande shell
</h4>

Pour exécuter une commande shell comme une tâche en arrière-plan au lieu d'une session Claude, tapez `!` comme premier caractère de l'entrée de lancement de la vue agent. Le `!` s'affiche comme un préfixe et tout ce que vous tapez après lui est la commande. L'exemple suivant lance `pytest -x` à partir de la boîte d'entrée de la vue agent :

```text theme={null}
! pytest -x
```

Appuyez sur `Entrée` pour démarrer la tâche. La même tâche peut également être lancée directement depuis votre shell avec `--exec` :

```bash theme={null}
claude --bg --exec 'pytest -x'
```

La commande s'exécute comme une tâche sauvegardée par PTY et apparaît comme une ligne dans la vue agent, avec la ligne de sortie la plus récente comme son statut. Une tâche shell exécute la commande à la place de Claude, donc aucun modèle n'est invoqué et la sortie n'est pas envoyée à aucune session.

Pour voir la sortie, attachez-vous à la ligne, appuyez sur `Espace` pour jeter un œil sans vous attacher, ou exécutez `claude logs <id>` depuis votre shell. La sortie capturée reste en mémoire et n'est pas écrite sur le disque. La ligne et sa sortie se nettoient automatiquement environ cinq minutes après la sortie de la commande, donc lisez-la avant si vous avez besoin du résultat.

<h3 id="how-file-edits-are-isolated">
  Comment les modifications de fichiers sont isolées
</h3>

Chaque session en arrière-plan, qu'elle soit démarrée à partir de la vue agent, `/bg`, ou `claude --bg`, démarre dans votre répertoire de travail. Avant de modifier des fichiers, Claude déplace la session dans une [git worktree](/docs/fr/worktrees) isolée sous `.claude/worktrees/`, afin que les sessions parallèles puissent lire le même checkout mais chacune écrit dans la sienne.

Claude ignore la worktree quand :

* La session est déjà à l'intérieur d'une git worktree liée, que Claude l'ait créée sous `.claude/worktrees/` ou que vous l'ayez créée avec `git worktree add` ailleurs
* Le répertoire de travail n'est pas un référentiel git et aucun hook [`WorktreeCreate`](/docs/fr/hooks#worktreecreate) n'est configuré
* L'écriture est en dehors du répertoire de travail

Pour désactiver l'isolation de worktree pour un référentiel où les git worktrees ne sont pas pratiques, définissez [`worktree.bgIsolation`](/docs/fr/settings#worktree-settings) sur `"none"`. Les sessions en arrière-plan modifient alors votre copie de travail directement sans d'abord se déplacer dans une worktree. Ajoutez le paramètre au fichier `.claude/settings.json` du projet :

```json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

En dehors d'un référentiel git, les sessions écrivent dans le répertoire de travail directement et ne sont pas isolées les unes des autres, donc évitez de lancer des sessions parallèles qui modifient les mêmes fichiers. Si vous utilisez un système de contrôle de version différent, configurez un hook [`WorktreeCreate`](/docs/fr/worktrees#non-git-version-control) et Claude isole les modifications de la même manière qu'il le fait pour git.

Quand le hook échoue dans un répertoire qui n'est pas un référentiel git, la session ignore l'isolation pour ce répertoire et modifie le répertoire de travail sur place. À l'intérieur d'un référentiel git, les écritures restent bloquées jusqu'à ce que la session s'isole. Avant la v2.1.203, une session en arrière-plan dans cet état ne pouvait modifier aucun fichier : chaque écriture était rejetée jusqu'à ce qu'elle s'isole, et le hook ne pouvait jamais isoler ce répertoire.

Supprimer une session supprime ou conserve la worktree que Claude a créée pour elle, selon la façon dont vous la supprimez et ce que la worktree contient :

* Supprimer dans la vue agent avec `Ctrl+X` deux fois supprime la worktree, y compris les modifications non validées, donc validez les modifications que vous voulez conserver d'abord.
* Supprimer depuis le shell avec [`claude rm`](#manage-sessions-from-the-shell) conserve une worktree qui a des modifications non validées, ainsi que sa ligne de session.
* Aucun chemin ne supprime une worktree avec des commits qui ne sont pas poussés nulle part : la worktree est [conservée avec sa session](#organize-the-list) et la sortie nomme le chemin conservé et la raison.
* Une worktree que vous avez créée vous-même et dans laquelle vous avez démarré la session est laissée en place de toute façon.

Pour trouver le chemin de la worktree d'une session, jetez un œil à la session ou attachez-vous et vérifiez son répertoire de travail.

Un [sous-agent](/docs/fr/sub-agents) que la session en arrière-plan lance hérite du répertoire de travail de la session, donc ses modifications de fichiers se retrouvent dans la worktree de la session plutôt que dans votre copie de travail. Pour donner à un sous-agent sa propre worktree séparée à la place, définissez [`isolation: worktree`](/docs/fr/sub-agents#supported-frontmatter-fields) dans son frontmatter ou passez `isolation: "worktree"` lors de son lancement.

À partir de la v2.1.198, une session en arrière-plan qui a isolé ses modifications de code dans une worktree valide également, pousse sa propre branche, et ouvre une demande de tirage en brouillon sans s'arrêter pour demander. L'étiquette [`#N`](#pull-request-status) apparaît sur sa ligne quand la demande de tirage s'ouvre. Elle ne pousse jamais vers `main` ou `master`, ne force jamais les poussées ou les fusions, et elle ignore la demande de tirage quand vous lui avez dit de ne pas en ouvrir une ou que le référentiel n'a pas de remote.

Une session modifiant un checkout qu'elle n'a pas isolée elle-même demande toujours avant de valider ou de changer de branche. Cela s'applique quand l'isolation est définie sur `"none"`, quand le déplacement de la worktree a échoué, ou quand la session a démarré à l'intérieur d'une worktree qui existait déjà.

<h3 id="set-the-model">
  Définir le modèle
</h3>

Le nom du modèle affiché dans l'en-tête de la vue agent est la valeur par défaut du lancement. Les nouvelles sessions que vous démarrez à partir de l'entrée utilisent ce modèle, qui provient du paramètre [`model`](/docs/fr/settings#available-settings) dans vos paramètres utilisateur. Définissez-le en sélectionnant un modèle dans le sélecteur [`/model`](/docs/fr/model-config), ou modifiez le paramètre directement.

Pour le remplacer pour l'ensemble de la session de la vue agent, passez `--model` lors de l'ouverture de la vue agent. Voir [Mode de permission, modèle et effort](#permission-mode-model-and-effort).

Pour changer la valeur par défaut du lancement à partir de la vue agent, tapez `/model` suivi d'un nom de modèle dans l'entrée de lancement et appuyez sur `Entrée`. L'en-tête se met à jour pour afficher ce modèle avec un marqueur `(session)`, et les sessions que vous lancez après utilisent ce modèle. Tapez `/model default` pour effacer le remplacement et revenir à la valeur par défaut du lancement. Ce remplacement dure pour le reste de l'exécution actuelle de `claude agents` et n'écrit pas dans votre fichier de paramètres. L'exemple suivant lance une session sur Opus et la suivante sur Sonnet :

```text theme={null}
/model opus
refactor auth
/model sonnet
run the test suite
```

Chaque session en arrière-plan peut s'exécuter sur un modèle différent. Pour le remplacer pour une session :

* Depuis le shell, passez `--model` avec `claude --bg`.
* Attachez-vous à une session en cours d'exécution et exécutez `/model` pour basculer : un choix du sélecteur, ou un `/model <name>` tapé, s'enregistre comme votre valeur par défaut pour les nouvelles sessions sauf si vous appuyez sur `s` dans le sélecteur pour un basculement réservé à la session. Un basculement réservé à la session persiste si la session est relancée.
* Lancez un [sous-agent](/docs/fr/sub-agents) dont le frontmatter définit un champ `model`.

<h3 id="permission-mode-model-and-effort">
  Mode de permission, modèle et effort
</h3>

Une session en arrière-plan lit ses [paramètres](/docs/fr/settings) à partir du répertoire dans lequel elle s'exécute, de la même manière que si vous aviez démarré `claude` là. Cela inclut les valeurs [`env`](/docs/fr/settings#available-settings) dans les paramètres du projet, donc une variable `ANTHROPIC_MODEL` ou de fournisseur définie là s'applique aux sessions en arrière-plan dans ce répertoire.

La sélection du fournisseur cloud, comme `CLAUDE_CODE_USE_BEDROCK` ou `CLAUDE_CODE_USE_VERTEX`, et les alias `ANTHROPIC_DEFAULT_*_MODEL` suivent le shell qui a lancé la session. Si vous exportez un remplacement de corps de requête [`CLAUDE_CODE_EXTRA_BODY`](/docs/fr/env-vars) dans ce shell, il atteint la session de la même manière. Avant la v2.1.206, les workers en arrière-plan ignoraient un `CLAUDE_CODE_EXTRA_BODY` exporté par le shell.

Si vous exportez une passerelle `ANTHROPIC_BASE_URL` dans le shell de lancement, elle atteint la session aussi, avec `ANTHROPIC_CUSTOM_HEADERS`, quand le superviseur s'exécute avec le même environnement de passerelle et la session s'exécute dans le répertoire à partir duquel vous avez lancé ou est votre propre session mise en arrière-plan avec `←` ou `/background`. C'est le cas normal quand le premier shell à ouvrir la vue agent ou lancer une session en arrière-plan est le shell de passerelle. Lancer dans un répertoire différent avec `@repo` ou `--cwd` ne porte pas la passerelle du shell ; les [paramètres](/docs/fr/settings) de ce projet fournissent le point de terminaison. Voir [le processus superviseur](#the-supervisor-process) pour savoir comment les sessions en arrière-plan sourçent les paramètres du fournisseur et les identifiants.

Le [mode de permission](/docs/fr/permissions) dépend de la façon dont vous avez démarré la session. Mettre en arrière-plan une session existante avec `/bg` ou `←` conserve le mode de permission actuel, donc une session que vous avez basculée vers `acceptEdits` ou `auto` reste dans ce mode après détachement. Lancer à partir de l'entrée de la vue agent ou exécuter `claude --bg` depuis votre shell utilise le `defaultMode` à partir des paramètres de ce répertoire, ou le `permissionMode` à partir du [frontmatter du sous-agent lancé](/docs/fr/sub-agents#supported-frontmatter-fields).

Le mode de permission, le modèle et l'effort avec lesquels une session en arrière-plan a été démarrée, ainsi que les [drapeaux de configuration qu'elle porte](#from-inside-a-session), persistent tous quand le superviseur [arrête et redémarre](#the-supervisor-process) ultérieurement le processus de la session. Une session que vous avez lancée avec `claude --bg --dangerously-skip-permissions` ou `claude --bg --permission-mode bypassPermissions` reste dans `bypassPermissions` après ce redémarrage au lieu de revenir au `defaultMode` du répertoire, et un modèle ou un effort que vous avez changé en milieu de session avec `/model` ou `/effort` est conservé.

Un effort que la session a pris à partir du paramètre [`effortLevel`](/docs/fr/settings#available-settings) plutôt que de `--effort` ou `/effort` n'est pas fixé au lancement : chaque processus démarré pour la session relit le paramètre à nouveau, donc modifier `effortLevel` dans `settings.json` atteint les sessions que vous mettez en arrière-plan avec `←` ou `/bg` et leurs redémarrages ultérieurs. Avant la v2.1.203, mettre en arrière-plan une session enregistrait son effort dérivé des paramètres comme si vous aviez passé `--effort`, donc les modifications ultérieures de `effortLevel` ne l'atteignaient jamais.

Un nom que vous avez défini avec [`/rename`](/docs/fr/commands) ou `Ctrl+R` persiste également lors de ce redémarrage, donc [`claude --resume <name>`](/docs/fr/sessions#name-your-sessions) résout toujours la session. Avant la v2.1.202, le redémarrage revenait au nom avec lequel la session a été lancée et le nouveau nom cessait de résoudre.

Pour définir les valeurs par défaut pour chaque session que vous lancez à partir de la vue agent, passez l'un de `--permission-mode`, `--model`, `--effort`, ou `--agent` lors de son ouverture :

```bash theme={null}
claude agents --permission-mode plan --model opus --effort high
```

`--agent` définit le [sous-agent](/docs/fr/sub-agents) utilisé quand une invite de lancement ne nomme pas un, soit avec `@name` soit comme premier mot. Il prend par défaut le paramètre [`agent`](/docs/fr/settings#available-settings) s'il en existe un, sinon l'agent intégré `claude` fourre-tout. Nommer un sous-agent dans l'entrée de lancement remplace les deux.

`claude agents` accepte également `--dangerously-skip-permissions` comme raccourci pour `--permission-mode bypassPermissions`, et `--allow-dangerously-skip-permissions` pour rendre `bypassPermissions` disponible dans le cycle `Shift+Tab` de chaque session lancée sans démarrer dans ce mode. Les deux correspondent aux [drapeaux CLI de haut niveau](/docs/fr/cli-reference).

Les valeurs par défaut actives apparaissent dans le pied de page sous l'entrée de lancement.

Sans ces drapeaux, la session utilise le `defaultMode` à partir des paramètres de ce répertoire ou le `permissionMode` à partir du [frontmatter du sous-agent lancé](/docs/fr/sub-agents#supported-frontmatter-fields), et le modèle affiché dans l'en-tête de la vue agent.

Utiliser `bypassPermissions` avec `claude --bg --permission-mode` est refusé jusqu'à ce que vous ayez accepté la clause de non-responsabilité du contournement en exécutant `claude --dangerously-skip-permissions` une fois de manière interactive, puisque ce mode permet à une session que vous ne regardez pas d'agir sans approbation. Passer `--dangerously-skip-permissions` ou `--permission-mode bypassPermissions` à `claude agents` affiche la même clause de non-responsabilité quand vous ne l'avez pas acceptée avant, et accepter applique `bypassPermissions` aux sessions que vous lancez à partir de la vue. Passer `--allow-dangerously-skip-permissions` affiche la même clause de non-responsabilité aussi, et accepter rend `bypassPermissions` disponible dans le cycle `Shift+Tab` de ces sessions sans les démarrer dedans.

<h3 id="settings-plugins-and-mcp-servers">
  Paramètres, plugins et serveurs MCP
</h3>

La vue agent accepte les mêmes drapeaux de configuration que `claude` pour charger les paramètres, les plugins, les serveurs MCP et les répertoires supplémentaires. Chaque drapeau s'applique à la vue agent elle-même et est transmis à chaque session que vous lancez à partir de celle-ci, donc un plugin ou un serveur MCP que vous chargez de cette manière est disponible dans ces sessions aussi.

| Drapeau                                                                                          | Effet                                                                                             |
| :----------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| [`--settings <file-or-json>`](/docs/fr/settings)                                                      | Remplacer les paramètres pour la vue agent et les sessions lancées                                |
| [`--add-dir <path>`](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) | Accorder l'accès aux fichiers à un répertoire supplémentaire                                      |
| [`--plugin-dir <path>`](/docs/fr/plugins)                                                             | Charger un plugin à partir d'un répertoire local                                                  |
| [`--mcp-config <file-or-json>`](/docs/fr/mcp)                                                         | Charger les serveurs MCP à partir d'un fichier de configuration ou d'une chaîne JSON              |
| `--strict-mcp-config`                                                                            | Utiliser uniquement les serveurs MCP de `--mcp-config`, en ignorant les autres configurations MCP |

Répétez `--add-dir`, `--plugin-dir`, ou `--mcp-config` une fois par valeur. La forme séparée par des espaces, comme `--add-dir a b c`, n'est pas prise en charge avec `claude agents`.

L'exemple suivant ouvre la vue agent avec un remplacement de paramètres et un répertoire supplémentaire :

```bash theme={null}
claude agents --settings ./ci-settings.json --add-dir ../shared-lib
```

<h2 id="manage-sessions-from-the-shell">
  Gérer les sessions depuis le shell
</h2>

Chaque session en arrière-plan a un ID court que vous pouvez utiliser depuis le shell. L'ID est affiché quand vous démarrez une session avec `claude --bg`, et l'ID de chaque session est son nom de répertoire sous `~/.claude/jobs/`. Ces commandes sont utiles pour les scripts ou quand vous ne voulez pas ouvrir la vue agent.

| Commande                     | Objectif                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude agents`              | Ouvrir la vue agent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `claude agents --cwd <path>` | Ouvrir la vue agent limitée aux sessions démarrées sous `<path>`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `claude agents --json`       | Afficher les sessions actives en tant que tableau JSON et quitter : chaque session active, plus les sessions en arrière-plan qui travaillent encore ou sont bloquées même quand leur processus a quitté. Ajoutez `--all` pour inclure aussi les sessions en arrière-plan terminées. Chaque entrée a `cwd`, `kind`, et `startedAt`. Les entrées en arrière-plan ont aussi `id`, utilisable avec `claude attach`/`logs`/`stop`, et `state` : l'une de `working`, `blocked`, `done`, `failed`, ou `stopped`. `pid` et `status` sont présents uniquement tant que le processus est actif, plus `waitingFor` quand status est `waiting`, qui indique sur quoi la session est bloquée, comme `permission prompt` ou `input needed` ; `sessionId` et `name` apparaissent quand ils sont définis. Une entrée interactive que vous n'avez jamais nommée porte un nom par défaut construit à partir du nom du répertoire de travail plus un suffixe de deux caractères, comme `my-app-3f`. Combinez avec `--cwd <path>` pour filtrer |
| `claude attach <id>`         | S'attacher à une session dans ce terminal                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `claude logs <id>`           | Afficher la sortie récente de la session                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `claude stop <id>`           | Arrêter une session. Accepte aussi `claude kill`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `claude respawn <id>`        | Redémarrer une session, en cours d'exécution ou arrêtée, avec sa conversation intacte, par exemple pour utiliser un binaire Claude Code mis à jour                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `claude respawn --all`       | Redémarrer chaque session en cours d'exécution, par exemple pour déplacer toutes les sessions vers un binaire Claude Code mis à jour en une seule fois                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `claude rm <id>`             | Supprimer une session de la liste. Supprime une worktree que Claude a créée pour la session si elle n'a pas de modifications non validées et aucun commit qui n'est pas poussé nulle part ; sinon la session est conservée aussi, et la commande affiche le chemin de la worktree et la raison afin que vous puissiez la résoudre et exécuter `claude rm` à nouveau. Laisse en place une worktree que vous avez créée vous-même. La transcription de la conversation reste sur votre machine locale et reste disponible via `claude --resume`                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `claude daemon status`       | Afficher l'état du [superviseur](#the-supervisor-process), la version, le répertoire socket et le nombre de workers                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `claude daemon stop --any`   | Arrêter le processus superviseur et les sessions en arrière-plan qu'il héberge. Passez `--keep-workers` pour laisser les sessions en arrière-plan en cours d'exécution afin que le superviseur suivant se reconnecte à elles. Le prochain `claude agents` ou `claude --bg` démarre un nouveau superviseur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h2 id="how-background-sessions-are-hosted">
  Comment les sessions en arrière-plan sont hébergées
</h2>

Chaque session listée dans la vue agent est considérée comme une session en arrière-plan, que vous y soyez actuellement attaché ou non. En contraste, une session démarrée en exécutant `claude` directement est liée à ce terminal et se termine quand il se ferme, sauf si vous [l'envoyez en arrière-plan](#from-inside-a-session).

<h3 id="the-supervisor-process">
  Le processus superviseur
</h3>

Les sessions en arrière-plan sont hébergées par un processus superviseur par utilisateur, séparé de votre terminal et de la vue agent. Le superviseur démarre automatiquement la première fois que vous mettez une session en arrière-plan ou ouvrez la vue agent, et vous ne le gérez pas directement.

Quand une mise à jour a remplacé ou supprimé le binaire à partir duquel un processus Claude Code en cours d'exécution a été lancé, ce processus démarre le superviseur à partir d'une autre copie installée, telle que le lanceur `claude` installé ou la version la plus récente sur le disque.

Le superviseur maintient un processus worker préchauffé prêt pour qu'une dispatch depuis la vue agent ou `claude --bg` démarre sans le délai d'un lancement à froid. Quand vous dispatchez, le superviseur assigne le worker préchauffé à votre session, applique le répertoire, les paramètres et les identifiants de cette session à celui-ci, puis démarre un remplacement pour la prochaine dispatch. Si aucun worker préchauffé sain n'est disponible, le superviseur lance un processus frais à la place.

Le superviseur et ses sessions s'authentifient avec les mêmes identifiants stockés que vos sessions interactives et ne font aucune connexion réseau supplémentaire au-delà de l'API du modèle. Les variables de sélection de fournisseur telles que `CLAUDE_CODE_USE_BEDROCK` et les alias `ANTHROPIC_DEFAULT_*_MODEL` sont lues depuis le shell qui a dispatché chaque session et sont appliquées à son worker.

Le `PATH` du shell de dispatch est appliqué au worker de la même manière, donc les commandes shell que la session exécute trouvent les mêmes outils que votre terminal. Avant v2.1.203, une session en arrière-plan conservait le `PATH` du shell qui a d'abord démarré le superviseur, donc les outils ajoutés à votre `PATH` depuis pourraient manquer, le plus souvent sur Windows.

Une session en arrière-plan n'hérite pas des variables de point de terminaison de passerelle telles que `ANTHROPIC_BASE_URL` ou les variables d'URL de base équivalentes pour Amazon Bedrock, Google Cloud's Agent Platform, et Microsoft Foundry depuis le shell qui a démarré le superviseur. Sans une passerelle exportée dans le shell à partir duquel vous dispatchez, la session utilise vos identifiants stockés et toute valeur `env` dans le bloc [settings](/docs/fr/settings) du répertoire du projet. Pour pointer chaque session dans un projet vers une [passerelle LLM](/docs/fr/llm-gateway), définissez `ANTHROPIC_BASE_URL` dans le bloc `env` du fichier `.claude/settings.json` de ce projet.

Si vous exportez une passerelle `ANTHROPIC_BASE_URL` dans le shell à partir duquel vous dispatchez, elle atteint le worker de cette session. `ANTHROPIC_CUSTOM_HEADERS` et l'identifiant exporté à côté sont transmis avec elle. Cela se produit quand le superviseur a été démarré à partir d'un environnement avec la même passerelle. Le superviseur capture son environnement depuis le premier shell qui ouvre la vue agent ou dispatch une session en arrière-plan, donc démarrer à partir du shell de passerelle lui donne cet environnement. La transmission s'applique également uniquement aux sessions dispatched dans le répertoire à partir duquel vous dispatchez, ou mises en arrière-plan à partir de votre propre session avec `←` ou `/background` : dispatcher dans un répertoire différent avec `@repo` ou `--cwd` ne porte pas la passerelle du shell, et le bloc `env` du `settings.json` de ce projet fournit le point de terminaison à la place. Quand l'environnement du superviseur porte une passerelle différente ou aucune, le worker conserve vos identifiants stockés contre le point de terminaison par défaut au lieu de mélanger l'identifiant d'un environnement avec le point de terminaison d'un autre. Avant v2.1.203, le `ANTHROPIC_BASE_URL` du shell de dispatch était supprimé tandis que le `ANTHROPIC_API_KEY` exporté à côté était conservé, donc la clé de la passerelle était envoyée au point de terminaison par défaut et chaque requête échouait avec un 401.

Le point de terminaison transmis s'applique uniquement à ce processus actif et n'est jamais écrit sur le disque. Quand le superviseur arrête une session inactive et la redémarre plus tard, le processus redémarré lit son point de terminaison à partir de vos paramètres à nouveau : avec un `ANTHROPIC_AUTH_TOKEN` de passerelle, il revient à vos identifiants stockés, et avec un `ANTHROPIC_API_KEY` émis par la passerelle, il peut échouer à s'authentifier jusqu'à ce que la passerelle soit définie dans les paramètres.

Chaque session en arrière-plan est son propre processus Claude Code, géré par le superviseur plutôt que lié à votre terminal. Une session qui fonctionne activement, attend votre entrée, ou a un terminal attaché garde son processus en cours d'exécution. Une commande shell en arrière-plan en cours d'exécution, un sous-agent, un workflow dynamique, ou un monitor compte comme un travail actif, donc un processus de longue durée tel qu'un serveur de développement maintient la session active.

Une fois qu'une session se termine et reste non attachée pendant environ une heure, le superviseur arrête son processus pour libérer des ressources. Une session que vous avez [épinglée](#organize-the-list) avec `Ctrl+T` est exemptée et garde son processus en cours d'exécution pendant qu'elle est inactive. La transcription et l'état restent sur le disque de toute façon, et la prochaine fois que vous vous attachez, apercevez, ou répondez à une session arrêtée, le superviseur démarre un processus frais à partir de là où il s'était arrêté. Quand chaque session s'est terminée et qu'aucun terminal n'est connecté, le superviseur lui-même quitte et redémarre la prochaine fois que vous en avez besoin.

Le travail en arrière-plan que la session elle-même a démarré au niveau supérieur est confié quand son processus est arrêté, redémarré ou mis à jour, y compris sur Windows. Le processus suivant démarré pour cette session reprend le travail :

* Une commande shell en arrière-plan qui s'est terminée entre-temps est signalée comme terminée avec sa sortie
* Un workflow dynamique reprend à partir de là où il s'était arrêté
* Un [sous-agent en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) reprend à partir de sa propre transcription

À partir de v2.1.198, la remise couvre tous les trois. Avant v2.1.198, elle couvrait uniquement les commandes shell et les workflows, donc un sous-agent en arrière-plan s'arrêtait avec le processus et était signalé comme échoué au prochain réveil.

Le travail dont l'état vit uniquement à l'intérieur du processus lui-même s'arrête avec lui au lieu d'être confié. C'est les commandes shell qu'un sous-agent a démarrées, que le sous-agent repris peut redémarrer, et les [monitors](/docs/fr/tools-reference#monitor-tool) en cours d'exécution, dont le flux d'événements ne peut pas être déplacé vers un autre processus.

Supprimer la session arrête tout ce qu'elle a confié. Pour arrêter tout le travail en arrière-plan de la session avec le processus au lieu de le confier, définissez la variable d'environnement [`CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF`](/docs/fr/env-vars#variables) à `1`.

Un processus redémarré trouve la conversation d'une session qui [s'est déplacée dans une worktree](#how-file-edits-are-isolated) en cours de tâche : quand la transcription n'est pas là où la session a commencé, Claude Code regarde également sous les worktrees enregistrées du référentiel. Avant v2.1.207, rouvrir cette session à partir de la vue agent après l'arrêt de son processus pouvait afficher une conversation vide avec uniquement son invite d'origine, la transcription restant intacte sur le disque ; ouvrir la session à nouveau sur v2.1.207 ou ultérieur la récupère.

Si une session redémarrée revient en affichant uniquement son invite d'origine parce que Claude Code a mal lu sa transcription comme vide, la transcription de conversation est renommée avec un suffixe `.orphaned-` au lieu d'être supprimée, donc elle reste sur votre machine.

Une ligne vide laissée après avoir appuyé sur `←` qui n'a jamais reçu d'invite est supprimée entièrement après environ cinq minutes pour que la liste se vide d'elle-même. Les sessions démarrées avec `claude --bg` et les sessions en attente d'une invite de configuration telle qu'une boîte de dialogue de confiance ne sont pas supprimées de cette façon.

Quand l'hôte manque de mémoire, le superviseur arrête d'abord les sessions inactives non épinglées et arrête les sessions épinglées inactives seulement si cela n'a rien libéré.

Le superviseur regarde le binaire Claude Code installé sur le disque et redémarre dans la nouvelle version après que l'[auto-updater](/docs/fr/setup#auto-updates) régulier le remplace. C'est une montre de fichier local, pas une vérification réseau. Les sessions en arrière-plan sont des processus détachés, donc elles continuent de s'exécuter pendant le redémarrage et le nouveau superviseur se reconnecte à elles. Une session épinglée inactive est également redémarrée sur place dans la nouvelle version pour qu'elle récupère la mise à jour sans que vous vous réattachiez.

Une fois que le nouveau superviseur prend le relais, il redémarre également les sessions inactives restantes dans la nouvelle version, quelques-unes à la fois en arrière-plan, après un court délai qui permet aux terminaux attachés lors du redémarrage de se reconnecter d'abord. Une session qui fonctionne, attend votre entrée, ou a un terminal attaché n'est pas interrompue ; elle se déplace vers la nouvelle version la prochaine fois que son processus redémarre. Avant v2.1.206, le superviseur ne déplaçait que quelques sessions inactives par minute vers une nouvelle version, donc les sessions pouvaient continuer à exécuter l'ancienne pendant un certain temps après une mise à jour.

Ces redémarrages ne déplacent jamais une session vers une version plus ancienne. Un superviseur exécutant une version plus ancienne de Claude Code que celle avec laquelle le processus d'une session a été démarré laisse ce processus seul ; la session continue d'exécuter la version plus récente jusqu'à ce qu'un superviseur plus récent prenne le relais.

L'exécution de `claude attach` tandis que le superviseur redémarre une session, que ce soit pour une mise à jour, une stagnation, ou une migration, attend le processus de remplacement au lieu d'échouer. Une ligne d'état telle que `Agent is updating to the new Claude Code…` nomme ce qu'il attend et compte les secondes écoulées, et la commande se connecte dès que la session est prête. Après environ 60 secondes, elle arrête d'attendre et signale une erreur. Avant v2.1.205, `claude attach` arrêtait de réessayer après quelques secondes et imprimait une erreur tandis que la session était encore en cours de redémarrage.

<h3 id="where-state-is-stored">
  Où l'état est stocké
</h3>

L'état de la session est stocké sous votre répertoire de configuration Claude Code. Si vous définissez [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars), le superviseur utilise ce répertoire à la place de `~/.claude` et s'exécute comme une instance séparée avec ses propres sessions.

| Chemin                           | Contenu                                                                                                                      |
| :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `~/.claude/daemon.log`           | Journal du superviseur                                                                                                       |
| `~/.claude/daemon/roster.json`   | Liste des sessions en arrière-plan en cours d'exécution, utilisée pour se reconnecter après un redémarrage                   |
| `~/.claude/jobs/<id>/state.json` | État par session affiché dans la vue agent                                                                                   |
| `~/.claude/jobs/<id>/tmp/`       | Répertoire de travail par session. Les écritures ici ne demandent pas de permission. Supprimé quand la session est supprimée |

Chaque session en arrière-plan a la variable d'environnement `CLAUDE_JOB_DIR` définie à son répertoire `~/.claude/jobs/<id>`, donc les commandes shell que la session exécute peuvent écrire des fichiers temporaires à `$CLAUDE_JOB_DIR/tmp` sans entrer en collision avec les sessions parallèles.

Pour inspecter cet état sans lire les fichiers directement, exécutez `claude daemon status`. Il rapporte si le superviseur est accessible, son ID de processus et sa version, le répertoire socket, et combien de sessions en arrière-plan sont actives.

La commande avertit également quand le superviseur en cours d'exécution est sur une version différente de celle du `claude` que vous avez invoqué, ce qui se produit après une mise à jour que le superviseur n'a pas encore redémarrée. L'avertissement affiche les deux versions et vous dit d'exécuter `claude daemon stop --any` pour récupérer la nouvelle version. Quand Claude Code est installé en tant que service du système d'exploitation, la commande suggérée est `claude daemon stop` sans le drapeau.

Les sessions survivent à ce décalage de version intact : une version plus ancienne de Claude Code qui met à jour le `state.json` d'une session préserve les champs qu'elle ne reconnaît pas et garde la session listée. La liste des sessions dans `roster.json` suit la même règle : une version plus ancienne qui la réécrit préserve les champs qu'une version plus récente a écrit, donc les sessions démarrées par la version plus récente restent accessibles et continuent d'accepter l'entrée après le redémarrage du superviseur. Avant v2.1.200, les versions plus anciennes pouvaient supprimer ces champs lors de la réécriture.

Sur Windows, `claude daemon status` affiche l'erreur de fichier sous-jacente quand le fichier de clé pipe du daemon est verrouillé ou illisible au lieu de signaler un échec de connexion générique.

<h3 id="turn-off-agent-view">
  Désactiver la vue agent
</h3>

Pour désactiver complètement les agents en arrière-plan et la vue agent, définissez le paramètre `disableAgentView` [setting](/docs/fr/settings) à `true` ou définissez la variable d'environnement `CLAUDE_CODE_DISABLE_AGENT_VIEW`. Les administrateurs peuvent appliquer cela via les [paramètres gérés](/docs/fr/permissions#managed-settings).

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="claude-agents-lists-subagents-instead-of-opening-agent-view">
  `claude agents` affiche les sous-agents au lieu d'ouvrir la vue agent
</h3>

Si `claude agents` affiche un nombre suivi de vos sous-agents configurés puis se ferme, la vue agent n'est pas disponible dans votre environnement. Exécutez `claude update` pour installer la dernière version.

Si la vue agent ne s'ouvre toujours pas après la mise à jour, vérifiez si elle a été [désactivée](#turn-off-agent-view) par un paramètre ou une variable d'environnement.

<h3 id="agent-view-opens-with-no-sessions">
  La vue agent s'ouvre sans sessions
</h3>

Avant de lancer votre première session, la vue agent affiche les en-têtes de section vides avec une description sous chacun, plus une explication d'une ligne au-dessus de l'entrée, à la place de la liste des sessions. Tapez une invite dans l'entrée en bas et appuyez sur `Entrée` pour lancer votre première session.

<h3 id="backgrounding-shows-a-background-this-session-dialog">
  Mise en arrière-plan affiche une boîte de dialogue `Background this session?`
</h3>

Si appuyer sur `←` pour mettre la session actuelle en arrière-plan affiche une boîte de dialogue `Background this session?`, la session a du travail en cours qui ne peut pas se transférer à la session en arrière-plan, comme un [monitor](/docs/fr/tools-reference#monitor-tool) en cours d'exécution, et Claude Code ne l'arrêtera pas silencieusement. La boîte de dialogue nomme le travail qui sera arrêté et, séparément, compte les tâches qui se transfèrent. Exécutez `/tasks` pour voir tout ce qui s'exécute, puis confirmez pour mettre en arrière-plan de toute façon ou choisissez `Stay` pour laisser le travail se terminer d'abord. Voir [À partir d'une session](#from-inside-a-session) pour savoir quels types de tâches se transfèrent et lesquels s'arrêtent.

<h3 id="prompt-rejected-as-too-short">
  Invite rejetée comme trop courte
</h3>

L'entrée de dispatch attend une description de tâche, pas une ouverture conversationnelle. Une invite plus courte que quatre caractères est rejetée avec un indice `Too short` pour qu'une frappe accidentelle ne démarre pas une session. Décrivez ce que vous voulez que la session fasse, par exemple `investigate the flaky checkout test`.

<h3 id="sessions-show-as-failed-after-shutdown">
  Les sessions s'affichent comme échouées après l'arrêt
</h3>

L'arrêt ou le redémarrage de votre machine arrête les sessions en arrière-plan en cours d'exécution, elles s'affichent donc comme échouées quand vous ouvrez à nouveau la vue agent. Attachez-vous, apercevez, ou répondez à n'importe laquelle d'entre elles et la session redémarre à partir de là où elle s'était arrêtée.

La mise en veille seule ne cause pas cela. Les sessions sont préservées lors de la mise en veille et le superviseur se reconnecte à elles au réveil.

<h3 id="opening-a-session-says-the-conversation-is-already-open">
  Ouverture d'une session indique que la conversation est déjà ouverte
</h3>

L'ouverture d'une ligne arrêtée dont la conversation est également maintenue ouverte par un autre processus Claude Code non interactif en cours d'exécution, par exemple un worker en arrière-plan pour la même conversation qui se termine toujours, affiche `This conversation is already open in another running Claude session` au lieu de démarrer le processus de la ligne, car deux processus ne peuvent pas écrire à la même transcription. Répondez dans la session qui a déjà la conversation ouverte, ou quittez-la et ouvrez la ligne à nouveau. Une réponse que vous avez tapée avec la tentative refusée n'est pas perdue ; elle est envoyée la prochaine fois que la session démarre.

Avant la v2.1.203, cet état démarrait un deuxième processus de toute façon. Ce processus s'est terminé avec une erreur `currently running as a background agent` et la ligne s'est affichée comme échouée.

<h3 id="a-session-fails-before-starting-with-a-possibly-low-memory-note">
  Une session échoue avant de démarrer avec une note « possibly low memory »
</h3>

À partir de la v2.1.199, quand le processus d'une session en arrière-plan se termine avant de finir de démarrer et que l'hôte manque de mémoire, le statut de la ligne nomme la sortie et ajoute `possibly low memory — free some up and retry`. Les versions antérieures affichaient uniquement la raison de sortie brute pour cet échec.

La note est une hypothèse, pas une cause confirmée. Claude Code l'ajoute uniquement quand le processus s'est terminé silencieusement, sans écrire d'erreur et sans être arrêté par un signal, et que l'hôte a signalé une mémoire faible à ce moment. Quand le processus a écrit une erreur avant de se terminer, la ligne affiche cette erreur à la place.

Libérez de la mémoire sur la machine, puis attachez-vous, apercevez, ou répondez à la ligne et le superviseur démarre un processus frais pour la session. Quand la mémoire reste faible, le superviseur [arrête également les sessions inactives](#the-supervisor-process) pour libérer des ressources de lui-même.

<h3 id="agent-view-says-the-background-service-did-not-respond">
  La vue agent indique que le service en arrière-plan n'a pas répondu
</h3>

Si l'attachement, l'aperçu, ou `claude logs` signale que le service en arrière-plan n'a pas répondu, le processus superviseur a probablement bloqué. Arrêtez-le et laissez le prochain `claude agents` en démarrer un nouveau. Pour garder vos sessions en arrière-plan en cours d'exécution pendant le redémarrage, passez `--keep-workers` :

```bash theme={null}
claude daemon stop --any --keep-workers
```

Le nouveau superviseur se reconnecte aux sessions en cours d'exécution. Sans `--keep-workers`, la commande termine également les sessions en arrière-plan. Le drapeau `--any` confirme que vous voulez arrêter un superviseur qui a démarré à la demande plutôt que comme service installé, ce qui est la valeur par défaut.

Un superviseur qui démarre mais ne peut pas accepter les connexions quitte et libère son verrou de lui-même, donc le prochain `claude agents` démarre un nouveau sans cet arrêt manuel. Les étapes ci-dessus s'appliquent quand un superviseur en cours d'exécution bloque.

Sur Windows, si le superviseur ne répond pas à la demande d'arrêt, la commande affiche son ID de processus. Terminez ce processus avec `taskkill /PID <pid>` pour terminer la récupération. Les sessions en arrière-plan sont toujours préservées quand vous avez passé `--keep-workers`.

<h3 id="dispatch-fails-with-could-not-resolve-authentication-method">
  Le dispatch échoue avec `Could not resolve authentication method`
</h3>

Si un dispatch en arrière-plan échoue avec `Could not resolve authentication method` tandis que les sessions interactives s'authentifient normalement, le worker qui a reçu le dispatch n'a pas récupéré les identifiants. Le superviseur fournit un nouvel instantané d'identifiants quand il assigne un [worker préchauffé](#the-supervisor-process), donc cette erreur signifie qu'aucun identifiant stocké n'était disponible pour le processus superviseur lui-même. Confirmez que vous avez exécuté `/login` ou configuré une clé API, puis arrêtez le superviseur :

```bash theme={null}
claude daemon stop --any --keep-workers
```

Le prochain `claude agents` ou `claude --bg` démarre un nouveau superviseur qui lit vos identifiants stockés. Si vous vous authentifiez avec une variable d'environnement comme `ANTHROPIC_API_KEY` plutôt qu'avec `/login`, exécutez cette prochaine commande à partir d'un shell où la variable est définie.

Voir la [référence d'erreur](/docs/fr/errors#could-not-resolve-authentication-method) pour la liste complète des causes et des correctifs.

<h3 id="background-sessions-can’t-read-desktop-documents-or-downloads-on-macos">
  Les sessions en arrière-plan ne peuvent pas lire Desktop, Documents ou Downloads sur macOS
</h3>

Sur macOS, l'hôte de session en arrière-plan s'exécute comme son propre processus et demande l'accès aux dossiers protégés séparément de votre terminal. Si une session en arrière-plan signale `Operation not permitted` lors de la lecture de `~/Desktop`, `~/Documents`, `~/Downloads`, ou un autre emplacement protégé, accordez l'accès dans Paramètres système sous Confidentialité et sécurité > Fichiers et dossiers, ou activez Accès complet au disque pour l'entrée.

Avec l'installateur natif, l'entrée apparaît comme Claude Code et l'autorisation persiste lors des mises à jour. Avec d'autres méthodes d'installation comme Homebrew ou npm, l'entrée affiche le chemin du binaire et peut avoir besoin d'être accordée à nouveau après la mise à jour.

<h3 id="background-sessions-can’t-reach-local-network-hosts-on-macos">
  Les sessions en arrière-plan ne peuvent pas atteindre les hôtes du réseau local sur macOS
</h3>

Sur macOS 15 et versions ultérieures, le système bloque un processus d'atteindre les appareils sur votre réseau local jusqu'à ce que vous accordiez la permission Réseau local. Avant la v2.1.198, l'hôte de session en arrière-plan n'a jamais demandé cette permission, donc les commandes ciblant une adresse LAN échouaient avec `connect: no route to host` même si la même commande fonctionnait dans un terminal de premier plan. À partir de la v2.1.198, la première commande dans une session en arrière-plan qui se connecte à une adresse de réseau local déclenche l'invite de permission Réseau local de macOS pour Claude Code. Accordez-la une fois et ces commandes atteindront les hôtes LAN de la même manière qu'ils le font dans un terminal de premier plan.

<h3 id="a-session-is-slow-to-respond-after-attaching">
  Une session est lente à répondre après l'attachement
</h3>

Une fois qu'une session s'est terminée et reste non attachée pendant environ une heure, le superviseur arrête son processus pour libérer des ressources. S'attacher démarre un processus frais à partir de là où il s'était arrêté et bascule vers la session immédiatement pendant que le processus redémarre. Les sessions qui fonctionnent, attendent votre intervention, ou sont [épinglées](#organize-the-list) ne sont pas arrêtées de cette façon, donc épinglez une session avec `Ctrl+T` pour la garder réactive.

Pendant que le processus démarre, le dernier écran de la transcription de la session s'affiche avec une note `Session is starting` en dessous, et la session en direct le remplace dès qu'elle est prête.

<h3 id="claude/worktrees/-is-filling-up">
  `.claude/worktrees/` se remplit
</h3>

Supprimer une session dans la vue agent supprime la worktree que Claude a créée pour elle, et une worktree qui ne peut pas être supprimée en toute sécurité [conserve sa ligne de session](#organize-the-list) pour qu'elle ne soit pas orpheline. `claude rm` conserve une worktree qui a des modifications non validées et affiche le chemin conservé. Listez les entrées restantes avec `git worktree list` dans le répertoire du projet et supprimez chacune avec `git worktree remove <path>`. Voir [Nettoyer les worktrees](/docs/fr/worktrees#clean-up-worktrees).

<h2 id="limitations">
  Limitations
</h2>

La vue agent est en aperçu de recherche avec les limitations suivantes :

* **Les limites de débit s'appliquent** : les sessions en arrière-plan consomment votre utilisation d'abonnement de la même manière que les sessions interactives, donc exécuter dix agents en parallèle utilise le quota environ dix fois plus vite qu'en exécuter un seul.
* **Les sessions sont locales** : les sessions en arrière-plan s'exécutent sur votre machine. Elles sont préservées lors de la mise en veille mais s'arrêtent si la machine s'éteint.
* **Les worktrees créées par Claude sont supprimées avec la session en vue agent** : validez les modifications avant de supprimer une session qui a modifié des fichiers dans sa propre worktree. Une worktree avec des commits qui ne sont poussés nulle part est conservée avec la session. `claude rm` conserve également une worktree qui a des modifications non validées avec sa session, et une worktree que vous avez créée vous-même est laissée en place.

<h2 id="related-resources">
  Ressources connexes
</h2>

Pour d'autres façons d'exécuter Claude en parallèle, consultez :

* [Exécuter les agents en parallèle](/docs/fr/agents) : comparez la vue agent avec les sous-agents, les équipes d'agents, et les worktrees
* [Équipes d'agents](/docs/fr/agent-teams) : coordonnez plusieurs sessions qui se messagent mutuellement
* [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : exécutez les sessions dans un environnement cloud géré au lieu de localement

<h2 id="version-history">
  Historique des versions
</h2>

La vue agent a évolué rapidement pendant l'aperçu de recherche. Si vous êtes sur une version plus ancienne de Claude Code, certains comportements sur cette page peuvent différer ; en particulier, `claude agents` rejette les drapeaux qu'il ne supporte pas encore avec une erreur `unknown option`. Le tableau ci-dessous répertorie quand chaque drapeau et comportement a été ajouté.

| Version  | Changement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| v2.1.208 | L'attachement à une session dont le processus s'est arrêté affiche le dernier écran de sa transcription tandis que le processus démarre, au lieu d'une simple note `Session is starting`. Une réponse qui ne peut pas être livrée parce que le service en arrière-plan est inaccessible ou l'envoi échoue est enregistrée et envoyée comme l'invite suivante de la session quand son processus redémarre ; avant cette version, une réponse perdue tandis que le service en arrière-plan était inaccessible était supprimée. Un processus dont le binaire lui-même a été remplacé par une mise à jour peut toujours démarrer le superviseur, à partir du lanceur `claude` installé ou de la version la plus récente sur le disque, au lieu d'échouer jusqu'au redémarrage de Claude Code. Un superviseur exécutant une version plus ancienne ne redémarre jamais une session inactive démarrée par une version plus récente sur son propre binaire plus ancien. La suppression d'une session supprime son worktree même après que la session ait déplacé le worktree sur une branche différente, et garde le worktree ensemble avec la ligne de session quand le worktree a des commits qui ne sont poussés nulle part ou une autre session le revendique, au lieu de détruire les commits ou d'orpheliner le worktree. `/install-github-app` et la liste des paramètres `/mcp` et ses actions d'authentification sont refusées dans une session en arrière-plan avec un message nommant l'alternative ; en v2.1.208 uniquement, le sélecteur `/model` a été refusé de la même manière et un `/model <name>` tapé a basculé cette session uniquement au lieu de sauvegarder également votre modèle par défaut. |
| v2.1.207 | Le panneau d'aperçu s'ouvre avec la phrase que la ligne tronque, comme la question exacte pour une session qui vous attend, et affiche combien de temps une session bloquée a attendu comme une seule ligne `waiting 3m` au lieu de préfixer le même horodatage à la phrase de statut et à la question. Coller le même texte à nouveau dans l'entrée de dispatch développe l'espace réservé `[Pasted text #N]` réduit au lieu d'en ajouter un deuxième. Une session en arrière-plan nommée en acceptant un plan affiche ce nom sur sa ligne. Une session en arrière-plan qui s'est déplacée dans un worktree conserve sa conversation quand son processus est redémarré à partir de la vue agent.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| v2.1.206 | Les résumés des lignes remplissent la largeur restante de la ligne et ne tronquent qu'au bord droit du terminal au lieu de 64 colonnes. Après le redémarrage du superviseur dans une nouvelle version de Claude Code, il redémarre les sessions en arrière-plan inactives restantes sur cette version en arrière-plan au lieu de quelques par minute. La suppression d'une session avec `Ctrl+X` ou `claude rm` l'efface également de la liste de sessions du superviseur, de sorte que la ligne ne réapparaît plus après un redémarrage du superviseur.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| v2.1.205 | Les résumés des lignes affichent le rapport d'une ligne de la session elle-même, tronqué à 64 colonnes, au lieu d'une invocation d'outil brute ou d'un nombre `done/total` ; les lignes groupées par répertoire s'ouvrent avec un mot d'état coloré. Le panneau d'aperçu s'ouvre avec la phrase de statut complète et, pour une session vous attendant, sa question exacte au-dessus de l'entrée de réponse. Les sessions qui modifient, commentent, ferment ou marquent une demande de tirage comme prête avec `gh` y sont liées, pas seulement celles qui créent ou extraient une demande de tirage, une poussée lie une demande de tirage même quand le nom de la branche locale ne correspond pas, et une demande de tirage dont la sortie de la commande de création a dépassé la limite en ligne est également liée. Un tour sans texte lisible conserve l'état précédent de la session au lieu de le basculer vers `Working`. `claude attach` attend jusqu'à environ 60 secondes une session qui redémarre, avec une ligne de statut nommant pourquoi, au lieu d'échouer.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.203 | Une passerelle `ANTHROPIC_BASE_URL` exportée dans le shell de dispatching atteint les sessions dispatchées à partir de celui-ci dans ce même répertoire quand le superviseur partage cet environnement de passerelle, au lieu d'être supprimée tandis que la clé API exportée à côté d'elle était conservée. Le `PATH` du shell de dispatching est appliqué à chaque worker de session. Appuyer sur `←` tandis que les sous-agents s'exécutent les attend au lieu de les redémarrer après dix secondes. La liste vide affiche toujours les en-têtes de section avec une description sous chacun. Taper `@` dans l'entrée de dispatch répertorie également les worktrees git enregistrés du référentiel de lancement qui vivent dans son arborescence de répertoires. Un effort hérité du paramètre `effortLevel` suit les modifications ultérieures de ce paramètre au lieu d'être fixé au dispatching. L'ouverture d'une session arrêtée dont la conversation est déjà ouverte dans une autre session en cours d'exécution est refusée avec un message au lieu d'échouer la ligne. Une commande qui n'est pas disponible dans la vue agent laisse le texte tapé dans l'entrée. Un hook `WorktreeCreate` qui échoue en dehors d'un référentiel git ne bloque plus la session de modifier les fichiers.                                                                                                                                                                                                                                                                                                                                                                                                         |
| v2.1.202 | Un nom défini avec `/rename` ou `Ctrl+R` sur une session en arrière-plan persiste quand le superviseur arrête et redémarre son processus, au lieu de revenir au nom avec lequel la session a été dispatchée.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.200 | Une version plus ancienne de Claude Code qui réécrit la liste de sessions dans `roster.json` préserve les champs écrits par une version plus récente, correspondant à la garantie `state.json` existante, de sorte que les sessions démarrées par la version plus récente continuent d'accepter l'entrée après le redémarrage du superviseur. Quand vous ouvrez une session qui a cessé de répondre, le superviseur redémarre son processus et la session continue la réponse interrompue à partir de là où elle s'était arrêtée.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| v2.1.199 | Une session en arrière-plan dont le processus se termine avant de finir de démarrer sur un hôte à faible mémoire affiche `possibly low memory — free some up and retry` dans le statut de sa ligne au lieu de seulement la raison de sortie brute. Mettre en arrière-plan une session avec `←` ou `/background` transfère son `/color` vers la nouvelle ligne.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| v2.1.198 | La vue agent envoie une notification via `preferredNotifChannel` quand une session en arrière-plan a besoin d'une entrée, se termine ou échoue, et déclenche le hook `Notification` avec le type `agent_needs_input` ou `agent_completed`. `←` et `/exit` à l'intérieur de `claude attach <id>` reviennent à la vue agent au lieu de quitter vers le shell ; `Ctrl+Z` revient au shell. Une session en arrière-plan qui a isolé son travail dans un worktree valide, pousse sa propre branche isolée, jamais `main` ou `master`, et ouvre une demande de tirage en brouillon quand elle se termine au lieu de demander d'abord. `/login` s'exécute dans la vue agent et ouvre la boîte de dialogue de connexion. La boîte de dialogue de sortie `Background work is running` propose `Move to background and exit`. Le transfert de sortie couvre également les sous-agents en arrière-plan, qui reprennent à partir de leur transcription à la prochaine activation au lieu d'être signalés comme échoués. `claude --bg` combiné avec `-p` ou `--print` est rejeté avec une erreur.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| v2.1.196 | Une seule pression sur `←` met en arrière-plan une session au premier plan ; les versions antérieures nécessitaient deux pressions, avec un indice de pied de page et une confirmation. `--dangerously-skip-permissions` passé à `claude agents` affiche la clause de non-responsabilité du contournement au lieu d'être silencieusement supprimé. Les sessions interactives que vous n'avez jamais nommées portent un nom par défaut comme `my-app-3f` dans les listes de sessions et `claude agents --json`. Les commandes shell en arrière-plan et les workflows dynamiques survivent à l'arrêt, au redémarrage ou à la mise à jour du processus de la session, y compris sur Windows ; définissez `CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF=1` pour désactiver le transfert. Une transcription mal lue comme vide au redémarrage est renommée avec un suffixe `.orphaned-` au lieu d'être supprimée.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.195 | Le travail en cours se transfère aussi quand vous mettez en arrière-plan une session sur Windows ; définissez `CLAUDE_DISABLE_ADOPT=1` pour l'arrêter à la place. Le groupe `Completed` remplit l'espace vertical restant et l'en-tête se compacte sur les terminaux courts. Une version plus ancienne de Claude Code ne supprime plus les champs `state.json` plus récents des sessions ou ne cache plus ces sessions de `claude agents`. S'attacher à une session arrêtée bascule immédiatement au lieu d'afficher un écran vide pendant jusqu'à cinq secondes. Un superviseur qui ne peut pas accepter les connexions quitte et libère son verrou de lui-même.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| v2.1.174 | Les sessions en arrière-plan n'héritent plus des variables de point de terminaison de passerelle comme `ANTHROPIC_BASE_URL` du shell de lancement du superviseur ; le superviseur fournit un nouvel instantané d'identifiants aux workers préchauffés, corrigeant les erreurs `Could not resolve authentication method` spurieuses.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.172 | `/model` dans l'entrée de dispatch définit un remplacement du modèle de dispatch limité à la session.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.161 | Les résumés des lignes affichent un nombre `done/total` pour les éléments de travail parallèles ; le panneau d'aperçu nomme l'élément de travail parallèle le plus long en cours d'exécution.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| v2.1.157 | `claude agents` accepte `--agent` ; les sessions dispatchées honorent le paramètre `agent`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| v2.1.145 | La dictée vocale supportée dans l'entrée de réponse du panneau d'aperçu et l'entrée de dispatch.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.143 | Le paramètre `worktree.bgIsolation` ajouté ; `claude agents` accepte `--allow-dangerously-skip-permissions`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.142 | `claude agents` accepte `--permission-mode`, `--model`, `--effort`, `--dangerously-skip-permissions`, `--settings`, `--add-dir`, `--plugin-dir`, `--mcp-config`, et `--strict-mcp-config`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| v2.1.141 | `claude agents` accepte `--cwd` pour limiter la liste à un projet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| v2.1.139 | La vue agent introduite comme un aperçu de recherche.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
