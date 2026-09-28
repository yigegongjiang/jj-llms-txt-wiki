> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Garder Claude orienté vers un objectif

> Définissez une condition d'achèvement avec /goal et Claude continue de travailler jusqu'à ce qu'elle soit satisfaite, qu'un modèle la juge impossible, ou qu'une erreur que vous devez corriger efface l'objectif.

La commande `/goal` définit une condition d'achèvement et Claude continue de travailler vers celle-ci sans que vous ayez besoin de le relancer à chaque étape. Après chaque tour, un petit modèle rapide vérifie si la condition est satisfaite. Si le modèle juge qu'elle n'est pas encore satisfaite, Claude commence un autre tour au lieu de vous rendre le contrôle. L'objectif s'efface automatiquement une fois la condition satisfaite, si le modèle juge la condition impossible à satisfaire, ou si un tour échoue sur [une erreur que vous devez corriger](#errors-you-have-to-fix-clear-the-goal).

Utilisez un objectif pour un travail substantiel avec un état final vérifiable :

* Migrer un module vers une nouvelle API jusqu'à ce que chaque site d'appel se compile et que les tests réussissent
* Implémenter un document de conception jusqu'à ce que tous les critères d'acceptation soient satisfaits
* Diviser un grand fichier en modules ciblés jusqu'à ce que chacun soit en dessous d'un budget de taille
* Traiter une file d'attente de problèmes étiquetés jusqu'à ce que la queue soit vide

<h2 id="compare-ways-to-keep-a-session-running">
  Comparer les façons de maintenir une session en cours
</h2>

Trois approches maintiennent la session actuelle en cours entre les invites. Choisissez en fonction de ce qui devrait démarrer le tour suivant :

| Approche                                                            | Le tour suivant commence quand                                                                                                                                                                                                       | S'arrête quand                                                                                                                                                                                                                         |
| :------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/goal`                                                             | Le tour précédent se termine, ou, dans une session interactive, une [vérification d'inactivité](#background-work-defers-evaluation) ou une [nouvelle tentative automatique](#other-errors-retry-or-pause-the-goal) arrive à échéance | Un modèle confirme que la condition est satisfaite ou juge qu'elle est impossible, ou un tour échoue sur [une erreur que vous devez corriger](#errors-you-have-to-fix-clear-the-goal), ou vous exécutez [`/goal clear`](#clear-a-goal) |
| [`/loop`](/docs/fr/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Un intervalle de temps s'écoule                                                                                                                                                                                                      | Vous l'arrêtez, ou Claude décide que le travail est terminé                                                                                                                                                                            |
| [Stop hook](/docs/fr/hooks-guide#prompt-based-hooks)                     | Le tour précédent se termine                                                                                                                                                                                                         | Votre propre script ou invite décide                                                                                                                                                                                                   |

`/goal` et un Stop hook se déclenchent tous les deux après chaque tour. `/goal` est un raccourci limité à la session : vous tapez une condition et elle est active pour la session actuelle uniquement. Un Stop hook réside dans votre fichier de paramètres, s'applique à chaque session dans sa portée, et peut exécuter un script pour des vérifications déterministes ou une invite pour des vérifications évaluées par le modèle.

[Le mode auto](/docs/fr/auto-mode-config) en lui-même approuve les appels d'outils au sein d'un seul tour mais ne démarre pas un nouveau. Claude s'arrête quand il juge le travail terminé. `/goal` ajoute un évaluateur séparé qui vérifie votre condition après chaque tour, donc l'achèvement est décidé par un modèle frais plutôt que par celui qui effectue le travail. Les deux sont complémentaires : le mode auto supprime les invites par outil, et `/goal` supprime les invites par tour.

<Tip>
  Les approches ci-dessus maintiennent la session actuelle en cours. Vous pouvez également planifier un travail qui s'exécute indépendamment de toute session ouverte, comme des tests nocturnes ou un triage matinal. Voir [les options de planification](/docs/fr/scheduled-tasks#compare-scheduling-options) pour les routines cloud et les tâches planifiées de bureau.
</Tip>

<h2 id="use-/goal">
  Utiliser `/goal`
</h2>

Un seul objectif peut être actif par session. La même commande le définit, le vérifie et l'efface selon l'argument.

<h3 id="set-a-goal">
  Définir un objectif
</h3>

Exécutez `/goal` suivi de la condition que vous souhaitez satisfaire. Si un objectif est déjà actif, le nouveau le remplace.

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

Définir un objectif démarre immédiatement un tour, avec la condition elle-même comme directive. Vous n'avez pas besoin d'envoyer une invite séparée. Pendant que l'objectif est actif, un indicateur `◎ /goal active` montre depuis combien de temps l'objectif s'exécute.

Un objectif ne change pas votre mode de permission. Pour laisser les tours d'objectif s'exécuter sans surveillance, exécutez `/goal` en [mode auto](/docs/fr/auto-mode-config). En [mode Manuel](/docs/fr/permission-modes), Claude demande toujours avant les appels d'outils que vos paramètres ne permettent pas déjà, comme la commande de test ci-dessus.

Pendant que l'objectif est actif, la transcription affiche chaque verdict que l'évaluateur retourne, et vous pouvez appuyer sur Ctrl+O pour voir la raison derrière celui-ci. La vue de statut affiche également la raison la plus récente, afin que vous puissiez voir vers quoi Claude travaille ensuite.

<h3 id="write-an-effective-condition">
  Écrire une condition efficace
</h3>

L'[évaluateur](#how-evaluation-works) juge votre condition par rapport à ce que Claude a présenté dans la conversation. Il n'exécute pas les commandes ou ne lit pas les fichiers indépendamment, donc écrivez la condition comme quelque chose que la propre sortie de Claude peut démontrer. « Tous les tests dans `test/auth` réussissent » fonctionne parce que Claude exécute les tests et le résultat se retrouve dans la transcription pour que l'évaluateur le lise.

Une condition qui tient sur plusieurs tours a généralement :

* **Un état final mesurable** : un résultat de test, un code de sortie de build, un nombre de fichiers, une queue vide
* **Une vérification énoncée** : comment Claude devrait le prouver, comme « `npm test` sort 0 » ou « `git status` est propre »
* **Des contraintes qui importent** : tout ce qui ne doit pas changer en chemin, comme « aucun autre fichier de test n'est modifié »

La condition peut contenir jusqu'à 4 000 caractères.

Pour limiter la durée d'exécution d'un objectif, incluez une clause de tour ou de temps dans la condition, comme `or stop after 20 turns`. Claude rapporte la progression par rapport à cette clause à chaque tour et l'évaluateur la juge à partir de la conversation.

<h3 id="check-status">
  Vérifier le statut
</h3>

Exécutez `/goal` sans arguments pour voir l'état actuel.

```text theme={null}
/goal
```

Si un objectif est actif, le statut affiche :

* La condition
* Depuis combien de temps il s'exécute
* Combien de tours ont été évalués
* La dépense de jetons actuelle
* La raison la plus récente de l'évaluateur

Le nombre de tours et la raison la plus récente apparaissent après la première évaluation.

Si aucun objectif n'est actif mais qu'un a été atteint plus tôt dans la session, le statut affiche la condition atteinte ainsi que sa durée, son nombre de tours et sa dépense de jetons.

<h3 id="clear-a-goal">
  Effacer un objectif
</h3>

Exécutez `/goal clear` pour supprimer un objectif actif avant qu'il ne se résolve.

```text theme={null}
/goal clear
```

Claude affiche `Goal cleared:` suivi de la condition pour confirmer, ou `No goal set` si rien n'était actif.

`stop`, `off`, `reset`, `none`, et `cancel` sont acceptés comme alias pour `clear`. L'exécution de `/clear` pour démarrer une nouvelle conversation supprime également tout objectif actif.

<h3 id="resume-with-an-active-goal">
  Reprendre avec un objectif actif
</h3>

Quand vous reprenez une session, Claude Code restaure un objectif qui était encore actif quand la session s'est terminée. Claude Code le restaure sur chaque route de reprise : `--continue`, `--resume` avec un ID ou un nom de session, ou un [chemin de fichier de transcription](/docs/fr/sessions#resume-a-session), et le [sélecteur de session](/docs/fr/sessions#use-the-session-picker). Avant v2.1.239, Claude Code restaurait l'objectif sur chaque route sauf le sélecteur `claude --resume`.

Claude Code conserve la condition mais réinitialise le nombre de tours, le minuteur et la ligne de base de dépense de jetons. Il ne restaure pas un objectif qui était déjà atteint ou effacé.

<h3 id="run-non-interactively">
  Exécuter de manière non-interactive
</h3>

`/goal` fonctionne en [mode non-interactive](/docs/fr/headless), dans l'[application de bureau](/docs/fr/desktop), et via [Remote Control](/docs/fr/remote-control). Définir un objectif avec `-p` exécute la boucle jusqu'à l'achèvement en une seule invocation :

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Avec le format de sortie texte par défaut, rien ne s'affiche jusqu'à ce que la condition soit satisfaite, donc un objectif qui s'exécute sur plusieurs tours peut sembler bloqué. Ajoutez `--output-format stream-json --verbose` pour émettre chaque message au fur et à mesure que la boucle s'exécute.

Interrompez le processus avec Ctrl+C pour arrêter un objectif non-interactive avant qu'il ne se résolve.

<h2 id="how-evaluation-works">
  Comment fonctionne l'évaluation
</h2>

`/goal` est un wrapper autour d'un [Stop hook basé sur une invite](/docs/fr/hooks#prompt-based-hooks) limité à la session. Chaque fois que Claude termine un tour, Claude Code envoie la condition et la conversation jusqu'à présent à votre [petit modèle rapide](/docs/fr/model-config) configuré, qui par défaut est Haiku sur l'API Claude ; sur un fournisseur tiers, consultez votre [page de fournisseur](/docs/fr/third-party-integrations) pour le défaut de la plateforme. Le modèle retourne l'un des trois verdicts, chacun avec une courte raison :

* **Pas encore atteint** : Claude continue à travailler et prend la raison comme guidance pour le tour suivant.
* **Atteint** : Claude Code efface l'objectif et enregistre une entrée atteinte dans la transcription.
* **Impossible** : l'évaluateur a jugé que la condition ne peut jamais être satisfaite. Claude Code efface l'objectif et enregistre une entrée échouée dans la transcription avec la raison. Vous n'avez pas besoin de l'effacer vous-même.

Si Claude continue à répondre à l'évaluateur sans faire de progrès (pas d'utilisation d'outils pendant plusieurs tours d'affilée), Claude Code arrête la boucle, imprime un avertissement et vous rend le contrôle avec l'objectif toujours défini. L'évaluation reprend après votre prochaine invite. Le [guide des hooks](/docs/fr/hooks-guide#stop-hook-hits-the-block-cap) explique le mécanisme sous-jacent.

<h3 id="when-a-turn-fails">
  Quand un tour échoue
</h3>

Quand un tour échoue, Claude Code efface l'objectif si l'erreur est une que vous devez corriger. Après toute autre erreur, l'objectif reste défini.

<h4 id="errors-you-have-to-fix-clear-the-goal">
  Les erreurs que vous devez corriger effacent l'objectif
</h4>

Si un tour échoue sur une erreur qui ne s'effacera que lorsque vous la corrigerez, Claude Code efface l'objectif et imprime un avertissement nommant la cause. L'avertissement commence par `Goal cleared after an unrecoverable error` et se termine par `Run /goal again to continue`. Corrigez la cause, puis [définissez l'objectif à nouveau](#set-a-goal) avec `/goal <condition>`. Quatre types d'échec effacent l'objectif :

* Une défaillance d'authentification, lorsque Claude Code gère ses propres identifiants. Lorsqu'un hôte les gère pour vous, comme l'application de bureau, l'extension VS Code ou une [session cloud](/docs/fr/claude-code-on-the-web), Claude Code laisse l'objectif actif car l'hôte restaure l'accès de lui-même.
* Un solde de crédit épuisé
* Un débordement de contexte que [l'auto-compaction](/docs/fr/model-config#set-the-auto-compact-window) n'a pas pu effacer
* Un modèle qui n'est pas disponible

<h4 id="other-errors-retry-or-pause-the-goal">
  Les autres erreurs réessaient ou mettent en pause l'objectif
</h4>

Après tout autre échec, l'objectif reste défini. Dans une session interactive sur Claude Code v2.1.269 ou ultérieur, Claude Code imprime également une ligne nommant la cause et réessaye de lui-même ou vous attend :

* **Réessayer** : après un échec qui tend à s'effacer de lui-même, comme un serveur surchargé ou une connexion perdue, un avis commençant par `Goal still active` affiche l'attente avant la prochaine tentative. Après trois tentatives automatiques, l'objectif se met en pause à la place.
* **Pause** : après un échec qu'une nouvelle tentative ne ferait que répéter, comme une limite de débit d'API, une [limite d'utilisation](/docs/fr/errors#youve-hit-your-session-limit) de claude.ai, ou un hook qui a terminé le tour, un avis commençant par `Goal paused` nomme la cause. Si la session [attend de continuer automatiquement quand une limite d'utilisation se réinitialise](/docs/fr/interactive-mode#wait-for-a-usage-limit-to-reset), Claude reprend le travail vers l'objectif à ce moment.

Envoyez un message à tout moment pour démarrer le tour suivant immédiatement. Pour désactiver les tentatives automatiques, définissez [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/fr/env-vars) sur `0`, ce qui désactive également les [vérifications](#background-work-defers-evaluation).

<h3 id="background-work-defers-evaluation">
  Le travail en arrière-plan diffère l'évaluation
</h3>

Si un sous-agent ou une commande shell en arrière-plan est toujours en cours d'exécution lorsqu'un tour se termine, Claude Code ignore l'évaluation pour ce tour. Il évalue à la fin du tour suivant qui se termine sans travail en arrière-plan en cours d'exécution. Lorsque le travail en arrière-plan se termine, Claude Code livre le résultat à Claude en tant que nouveau tour, vous n'avez donc pas besoin de faire une invite.

Une fois que le travail en arrière-plan a gardé l'objectif en attente pendant 30 minutes, une vérification est due. Dans la vérification, Claude Code énumère les tâches en cours d'exécution et demande à Claude de lire leur sortie, de continuer à attendre s'ils progressent, et de corriger ou d'arrêter ceux qui sont bloqués. Après la première vérification, Claude Code attend deux fois plus longtemps avant chaque vérification ultérieure, jusqu'à quatre fois l'intervalle initial : avec la valeur par défaut, 1 heure après la première vérification, puis toutes les 2 heures. Claude Code livre une vérification due, la première incluse, de l'une des deux façons suivantes :

* **Lorsqu'un tour se termine** : Claude Code livre la vérification à la fin du tour suivant qui se termine avec le travail toujours en cours d'exécution. Dans une session non-interactive, comme celle démarrée avec `-p`, c'est la seule façon dont Claude Code livre les vérifications.
* **Pendant que la session est inactive** : dans une session interactive, Claude Code démarre également un tour de lui-même pour livrer la vérification au lieu d'attendre votre prochaine invite. Si le travail en arrière-plan s'est arrêté sans signaler un résultat, Claude Code demande à Claude de continuer vers l'objectif. Claude Code démarre au maximum trois vérifications inactives par objectif entre vos invites. Dans la troisième vérification inactive, Claude Code dit que les vérifications inactives sont en pause jusqu'à ce que vous envoyiez une autre invite. Avant v2.1.246, les vérifications inactives n'étaient pas limitées. Les vérifications inactives nécessitent Claude Code v2.1.236 ou ultérieur.

Avant v2.1.239, seules les vérifications inactives reculaient de cette façon ; une vérification livrée à la fin d'un tour se reproduisait au premier intervalle.

Pour modifier le premier intervalle, définissez [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/fr/env-vars). Claude Code utilise votre valeur à la place de l'intervalle de 30 minutes et met à l'échelle les intervalles ultérieurs avec elle. Définissez-le sur `0` pour désactiver les vérifications et les [tentatives automatiques](#other-errors-retry-or-pause-the-goal).

Les vérifications nécessitent Claude Code v2.1.234 ou ultérieur.

<h3 id="evaluation-model-and-cost">
  Modèle d'évaluation et coût
</h3>

Pour évaluer sur un modèle différent, définissez [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/fr/model-config#environment-variables).

<Warning>
  Claude Code lit `ANTHROPIC_DEFAULT_HAIKU_MODEL` partout où il utilise le petit modèle rapide, pas seulement pour l'évaluation `/goal`. Lorsque vous le définissez, Claude Code résout également l'[alias `haiku`](/docs/fr/model-config#model-aliases) à ce modèle et exécute la [fonctionnalité en arrière-plan](/docs/fr/costs#background-token-usage), comme la résumé de conversation, sur celui-ci.
</Warning>

L'évaluateur s'exécute sur le fournisseur pour lequel votre session est configurée. Il n'appelle pas les outils, donc il ne peut juger que ce que Claude a déjà présenté dans la conversation.

<Note>
  Les jetons d'évaluation sont facturés sur le petit modèle rapide configuré pour votre fournisseur et sont généralement négligeables par rapport à la dépense du tour principal.
</Note>

<h2 id="requirements">
  Exigences
</h2>

Claude Code met `/goal` à disposition selon la même [règle de confiance d'espace de travail que les hooks dans les fichiers de paramètres](/docs/fr/permissions#what-runs-before-you-trust-a-folder), car l'évaluateur fait partie du système de hooks. `/goal` est également indisponible quand [`disableAllHooks`](/docs/fr/hooks#disable-or-remove-hooks) est `true` après l'application de la précédence des paramètres, ou quand [`allowManagedHooksOnly`](/docs/fr/settings-reference#allowmanagedhooksonly) est défini dans les paramètres gérés. Dans chaque cas, la commande vous indique pourquoi au lieu de ne rien faire silencieusement.

<h2 id="see-also">
  Voir aussi
</h2>

* [Exécuter une invite à plusieurs reprises avec `/loop`](/docs/fr/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) : réexécuter sur un intervalle de temps au lieu de jusqu'à ce qu'une condition soit satisfaite
* [Hooks basés sur une invite](/docs/fr/hooks-guide#prompt-based-hooks) : écrivez votre propre Stop hook quand vous avez besoin d'une logique d'évaluation personnalisée
* [Mode auto](/docs/fr/auto-mode-config) : approuvez les appels d'outils automatiquement afin que chaque tour d'objectif s'exécute sans surveillance
* [Comparaison de planification](/docs/fr/scheduled-tasks#compare-scheduling-options) : exécutez un travail selon un calendrier indépendant de toute session ouverte
