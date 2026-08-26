> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Planifier des tâches récurrentes dans Claude Code Desktop

> Configurez des tâches planifiées dans Claude Code Desktop pour exécuter Claude automatiquement de manière récurrente pour les révisions de code quotidiennes, les audits de dépendances ou les briefings matinaux.

Les tâches planifiées démarrent une nouvelle session automatiquement à une heure et une fréquence que vous choisissez. Utilisez-les pour les travaux récurrents comme les révisions de code quotidiennes, les vérifications de mises à jour de dépendances ou les briefings matinaux qui extraient des données de votre calendrier et de votre boîte de réception.

La page **Routines** de l'application Desktop vous permet de créer à la fois des tâches planifiées locales et des [routines](/docs/fr/routines) distantes. Une tâche locale s'exécute sur votre machine avec un accès direct à vos fichiers et outils, mais ne s'active que lorsque l'application est ouverte et que votre ordinateur est actif. Une routine distante s'exécute sur l'infrastructure cloud gérée par Anthropic même lorsque votre ordinateur est éteint, et peut également s'activer lors d'appels API ou d'événements GitHub. Cette page couvre les tâches planifiées locales ; pour les routines distantes et leurs options de déclenchement, consultez [Routines](/docs/fr/routines).

<h2 id="compare-scheduling-options">
  Comparer les options de planification
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<Note>
  Par défaut, les tâches planifiées s'exécutent sur l'état actuel de votre répertoire de travail, y compris les modifications non validées. Activez le bouton bascule worktree lors de la création de la tâche pour donner à chaque exécution son propre worktree Git isolé, de la même manière que les [sessions parallèles](/docs/fr/desktop#work-in-parallel-with-sessions) fonctionnent.
</Note>

<h2 id="create-a-scheduled-task">
  Créer une tâche planifiée
</h2>

Cliquez sur **Routines** dans la barre latérale, puis cliquez sur **New routine** et choisissez **Local**. Configurez ces champs :

| Champ        | Description                                                                                                                                                                                                                                                                                                                                                   |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Name         | Identifiant de la tâche. Converti en minuscules kebab-case et utilisé comme nom de dossier sur le disque. Doit être unique parmi vos tâches.                                                                                                                                                                                                                  |
| Description  | Résumé court affiché dans la liste des tâches.                                                                                                                                                                                                                                                                                                                |
| Instructions | Ce que Claude doit faire lorsque la tâche s'exécute. Écrivez ceci de la même manière que vous écriviriez n'importe quel message dans la boîte de prompt. L'entrée des instructions inclut des sélecteurs pour le mode de permission et le modèle, et en dessous vous sélectionnez le dossier de travail et si vous souhaitez exécuter dans un worktree isolé. |
| Schedule     | Fréquence d'exécution de la tâche. Voir [options de planification](#schedule-options) ci-dessous.                                                                                                                                                                                                                                                             |

Un dossier est requis avant de pouvoir enregistrer la tâche. Si vous n'avez pas encore approuvé ce dossier, Desktop vous invite à l'approuver avant d'enregistrer.

Vous pouvez également créer une tâche en décrivant ce que vous souhaitez dans n'importe quelle session. Par exemple, « configurer une révision de code quotidienne qui s'exécute chaque matin à 9h » crée une tâche récurrente, et « rappelle-moi à 15h demain de vérifier le déploiement » crée une tâche unique qui se désactive après son exécution.

<h2 id="schedule-options">
  Options de planification
</h2>

Choisissez un préréglage dans le contrôle Schedule :

* **Manual** : pas de planification, s'exécute uniquement lorsque vous cliquez sur **Run now**. Utile pour enregistrer un prompt que vous déclenchez à la demande
* **Hourly** : s'exécute toutes les heures
* **Daily** : affiche un sélecteur d'heure, par défaut 9h00 heure locale
* **Weekdays** : identique à Daily mais ignore samedi et dimanche
* **Weekly** : affiche un sélecteur d'heure et un sélecteur de jour

Pour les intervalles que le sélecteur n'offre pas, comme toutes les 15 minutes, le premier de chaque mois ou une exécution unique à un moment futur spécifique, demandez à Claude dans n'importe quelle session Desktop de définir la planification. Utilisez le langage naturel ; par exemple, « planifier une tâche pour exécuter tous les tests toutes les 6 heures ».

<h2 id="how-scheduled-tasks-run">
  Comment les tâches planifiées s'exécutent
</h2>

Les tâches planifiées s'exécutent sur votre machine. Desktop vérifie la planification chaque minute lorsque l'application est ouverte et démarre une nouvelle session lorsqu'une tâche est due, indépendamment de toute session manuelle que vous avez ouverte. Chaque tâche reçoit un petit délai de quelques minutes après l'heure planifiée pour échelonner le trafic API. Le délai est déterministe : la même tâche démarre toujours au même décalage.

Lorsqu'une tâche s'active, vous recevez une notification de bureau et une nouvelle session apparaît sous une section **Scheduled** dans la barre latérale. Ouvrez-la pour voir ce que Claude a fait, examiner les modifications ou répondre aux invites de permission. La session fonctionne comme n'importe quelle autre : Claude peut modifier des fichiers, exécuter des commandes, créer des commits et ouvrir des pull requests.

Les tâches ne s'exécutent que lorsque l'application de bureau est en cours d'exécution et que votre ordinateur est actif. Si votre ordinateur se met en veille à une heure planifiée, l'exécution est ignorée. Pour empêcher la mise en veille inactive, activez **Keep computer awake** dans Paramètres sous **Desktop app → General**. Fermer le couvercle de l'ordinateur portable le met toujours en veille. Pour les tâches qui doivent s'exécuter même lorsque votre ordinateur est éteint, ou qui doivent être déclenchées par un appel API ou un événement GitHub, créez plutôt une [routine](/docs/fr/routines) distante.

<h2 id="missed-runs">
  Exécutions manquées
</h2>

Lorsque l'application démarre ou que votre ordinateur se réveille, Desktop vérifie si chaque tâche a manqué des exécutions au cours des sept derniers jours. Si c'est le cas, Desktop démarre exactement une exécution de rattrapage pour l'heure la plus récemment manquée et rejette tout ce qui est plus ancien. Une tâche quotidienne qui a manqué six jours s'exécute une fois au réveil. Desktop affiche une notification lorsqu'une exécution de rattrapage démarre.

Gardez cela à l'esprit lors de la rédaction de prompts. Une tâche planifiée pour 9h peut s'exécuter à 23h si votre ordinateur était en veille toute la journée. Si le timing est important, ajoutez des garde-fous au prompt lui-même, par exemple : « Examinez uniquement les commits d'aujourd'hui. S'il est après 17h, ignorez la révision et publiez simplement un résumé de ce qui a été manqué. »

<h2 id="permissions-for-scheduled-tasks">
  Permissions pour les tâches planifiées
</h2>

Chaque tâche a son propre mode de permission, que vous définissez lors de la création ou de la modification de la tâche. Les règles d'autorisation de `~/.claude/settings.json` s'appliquent également aux sessions de tâches planifiées. Si une tâche s'exécute en mode Ask et doit exécuter un outil pour lequel elle n'a pas de permission, l'exécution s'arrête jusqu'à ce que vous l'approuviez. La session reste ouverte dans la barre latérale pour que vous puissiez répondre plus tard.

Pour éviter les arrêts, cliquez sur **Run now** après avoir créé une tâche, surveillez les invites de permission et sélectionnez « always allow » pour chacune. Les exécutions futures de cette tâche approuvent automatiquement les mêmes outils sans demander. Vous pouvez examiner et révoquer ces approbations à partir de la page de détail de la tâche.

Les outils connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) demandent une approbation à chaque appel et n'offrent pas d'option « always allow ». Les exécutions qui appellent ces outils s'arrêtent à chaque fois.

<h2 id="manage-scheduled-tasks">
  Gérer les tâches planifiées
</h2>

Cliquez sur une tâche dans la liste **Routines** pour ouvrir sa page de détail. À partir de là, vous pouvez :

* **Run now** : démarrer la tâche immédiatement sans attendre l'heure planifiée suivante
* **Status** : basculer entre Active et Paused pour mettre en pause ou reprendre les exécutions planifiées sans supprimer la tâche
* **Edit** : modifier les instructions, la planification, le dossier ou d'autres paramètres
* **Review history** : voir chaque exécution passée, y compris les exécutions ignorées. Survolez une entrée ignorée pour voir pourquoi : votre ordinateur était en veille, l'exécution précédente était toujours en cours ou d'autres tâches planifiées s'exécutaient déjà. Cliquez sur **Show more** pour charger les entrées plus anciennes.
* **Review allowed permissions** : voir et révoquer les approbations d'outils enregistrées pour cette tâche à partir du panneau **Always allowed**
* **Delete** : supprimer la tâche et archiver toutes les sessions qu'elle a créées. Une case à cocher **Also delete files on disk** apparaît dans la boîte de dialogue de confirmation ; cochez-la pour supprimer également le fichier `SKILL.md` de la tâche et les données associées de `~/.claude/scheduled-tasks/`.

Vous pouvez également lister, créer, modifier et mettre en pause des tâches en demandant à Claude dans n'importe quelle session Desktop. Par exemple, « pause my dependency-audit task » ou « show me my scheduled tasks ». Pour supprimer une tâche, utilisez le bouton **Delete** sur sa page de détail.

Une tâche planifiée peut également modifier sa propre planification ou son prompt depuis une session en cours d'exécution à l'aide de l'outil MCP `update_scheduled_task`. Cela permet à une tâche de se replanifier en fonction de ce qu'elle trouve, par exemple, replanifier une révision de code pour s'exécuter plus tôt lorsqu'elle détecte qu'une branche de version a été créée.

Pour modifier le prompt d'une tâche sur le disque, ouvrez `~/.claude/scheduled-tasks/<task-name>/SKILL.md` (ou sous [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars) si défini). Le fichier utilise le frontmatter YAML pour `name` et `description`, avec le prompt comme corps. Les modifications prennent effet à la prochaine exécution. La planification, le dossier, le modèle et l'état activé ne sont pas dans ce fichier : modifiez-les via le formulaire Edit ou demandez à Claude.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Routines](/docs/fr/routines) : exécuter des tâches sur l'infrastructure gérée par Anthropic selon un calendrier, via un appel API ou en réponse à des événements GitHub, même lorsque votre ordinateur est éteint
* [Exécuter des prompts selon un calendrier](/docs/fr/scheduled-tasks) : planification au niveau de la session avec `/loop` dans la CLI
* [Claude Code GitHub Actions](/docs/fr/github-actions) : exécuter Claude selon un calendrier dans CI au lieu de sur votre machine
* [Utiliser Claude Code Desktop](/docs/fr/desktop) : le guide complet de l'application Desktop
