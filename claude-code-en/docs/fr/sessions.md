> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gérer les sessions

> Nommez, reprenez, créez des branches et basculez entre les conversations Claude Code. Couvre `--continue`, `--resume`, `--from-pr`, le sélecteur `/resume`, la dénomination des sessions, l'export des transcriptions et l'emplacement des transcriptions.

Une session est une conversation enregistrée liée à un répertoire de projet. Claude Code la stocke localement au fur et à mesure que vous travaillez, ce qui vous permet de reprendre là où vous vous êtes arrêté, de créer une branche pour essayer une approche différente ou de basculer entre les tâches.

L'[application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions), [Claude Code sur le web](/docs/fr/claude-code-on-the-web) et l'[extension VS Code](/docs/fr/vs-code#resume-past-conversations) maintiennent chacun leur propre historique de sessions. Cette page couvre l'interface CLI.

<h2 id="resume-a-session">
  Reprendre une session
</h2>

Les sessions sont enregistrées en continu dans les [fichiers de transcription locaux](#export-and-locate-session-data) au fur et à mesure que vous travaillez, ce qui vous permet de revenir à l'une d'elles après avoir quitté ou exécuté `/clear`. Utilisez ces points d'entrée :

| Commande                            | Ce qu'elle fait                                                                                                               |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `claude --continue`                 | Reprend la conversation la plus récente dans le répertoire courant                                                            |
| `claude --resume`                   | Ouvre le [sélecteur de sessions](#use-the-session-picker)                                                                     |
| `claude --resume <name>`            | Reprend directement la session nommée                                                                                         |
| `claude --resume <transcript-path>` | Reprend la conversation stockée dans le fichier de [transcription](#where-transcripts-are-stored) `.jsonl` à ce chemin absolu |
| `claude --from-pr <number>`         | Ouvre le sélecteur de sessions filtré aux sessions liées à cette demande de tirage                                            |
| `/resume`                           | Bascule vers une conversation différente depuis une session active                                                            |

Claude Code laisse les sessions créées avec [`claude -p`](/docs/fr/headless) ou le [SDK Agent](/docs/fr/agent-sdk/overview) en dehors du sélecteur de sessions et en dehors de `claude --continue`. Vous pouvez toujours en reprendre une en passant son ID de session à `claude --resume <session-id>`. Avec `claude --continue`, Claude Code ignore également les [sessions dont la première invite était `/loop`](#where-the-session-picker-looks). Lorsque vous exécutez [`claude -p --continue`](/docs/fr/headless#continue-conversations), Claude Code inclut les sessions `-p`, SDK et `/loop`.

`claude --continue` ouvre une [session en arrière-plan](/docs/fr/agent-view) qui s'est terminée, mais pas une qui est toujours en cours d'exécution ; l'ouverture de sessions en arrière-plan terminées nécessite Claude Code v2.1.257 ou ultérieur. Si votre conversation la plus récente est une que vous [avez envoyée en arrière-plan](/docs/fr/agent-view#send-the-session-to-the-background) et qu'elle s'exécute toujours là-bas, Claude Code se termine avec `Your most recent conversation is running in the background` et l'ID de cette session. Attachez-vous à la session depuis [`claude agents`](/docs/fr/agent-view#attach-to-a-session), ou exécutez `claude --resume` pour en choisir une autre.

Vous pouvez exécuter `claude --resume <session-id>` depuis n'importe quel répertoire : Claude Code cherche l'ID dans le répertoire de projet courant et ses git worktrees d'abord, puis dans tous les autres projets sur cette machine, ce qui lui permet de trouver une session qui a démarré ailleurs ou qui s'est déplacée avec [`/cd`](/docs/fr/commands). La recherche inter-projets résout l'ID uniquement lorsqu'exactement un autre projet contient une transcription avec des messages pour celui-ci, donc un doublon copié à la main fait que Claude Code signale non-trouvé plutôt que de reprendre une copie arbitraire. Si aucune session stockée ne correspond à l'ID, Claude Code signale `No conversation found with session ID: <session-id>`. Avant la v2.1.223, la recherche s'arrêtait au répertoire de projet courant et à ses git worktrees, donc vous deviez reprendre depuis le répertoire dans lequel la session avait travaillé en dernier.

<h3 id="what-a-resumed-session-restores">
  Ce qu'une session reprise restaure
</h3>

Une session reprise restaure la conversation ainsi que l'état enregistré en elle :

* Historique de conversation : l'historique complet, y compris les appels d'outils et les résultats. Un outil qui était toujours en cours d'exécution lorsque le processus précédent s'est terminé, par exemple lors d'un plantage, ne se termine pas ou ne s'exécute pas à nouveau lorsque vous reprenez. Claude voit l'appel marqué comme interrompu avant que son résultat ne soit enregistré et on lui dit de vérifier s'il a pris effet avant de l'exécuter à nouveau, sauf si [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](/docs/fr/env-vars#variables) est défini. Avant la v2.1.281, Claude Code supprimait l'appel interrompu de la conversation ou le montrait à Claude comme un appel que vous aviez interrompu.
* Modèle : la session continue sur le modèle qu'elle utilisait. Le modèle n'est pas restauré lorsqu'il a été retiré ou n'est pas autorisé par `availableModels`, lorsqu'un drapeau `--model` ou une variable d'environnement de la famille `ANTHROPIC_MODEL` en choisit un au lancement, ou sur les fournisseurs qui utilisent des ID de déploiement spécifiques au fournisseur, tels que [Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry](/docs/fr/third-party-integrations) ; voir [configuration du modèle](/docs/fr/model-config#setting-your-model) pour l'ordre de résolution.
* Agent : une session démarrée avec [`--agent`](/docs/fr/sub-agents#invoke-subagents-explicitly) ou le paramètre `agent` continue en tant que cet agent, en conservant ses restrictions d'outils et son modèle. Passez `--agent` lors de la reprise pour en choisir un différent ; pour l'invite système dans l'un ou l'autre cas, voir [Drapeaux d'invite système dans les conversations reprises](/docs/fr/cli-reference#system-prompt-flags-in-resumed-conversations). Claude Code cherche l'agent dans deux endroits : le répertoire d'origine de la session, à condition que vous ayez [approuvé cet espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust), puis le répertoire depuis lequel vous reprenez, donc un agent limité au projet se charge toujours lorsque vous reprenez depuis un autre répertoire. Si Claude Code ne trouve pas l'agent dans l'un ou l'autre endroit, la session reprend avec les outils par défaut et affiche un [avertissement nommant l'agent](/docs/fr/errors#session-agent-no-longer-available).
* Mode de permission : si vous reprenez depuis un terminal avec `claude --continue`, `claude --resume <session-id>` ou `claude --resume <name>` lorsque le nom correspond à une session, sans `-p`, Claude Code restaure le mode de permission dans lequel se trouvait la session, sauf dans les cas de [mode de permission à la reprise](#permission-mode-on-resume), qui couvre également le sélecteur de sessions, `/resume` et la reprise avec `claude -p`. Passez `--permission-mode` ou `--dangerously-skip-permissions` pour remplacer le mode restauré.
* Objectif actif : un [objectif](/docs/fr/goal#resume-with-an-active-goal) qui était toujours actif lorsque la session s'est terminée se poursuit ; son nombre de tours, son minuteur et sa ligne de base de dépense de jetons se réinitialisent.
* Tâches planifiées : les [tâches qui n'ont pas expiré](/docs/fr/scheduled-tasks#limitations) sont restaurées. Les tâches Bash en arrière-plan et les tâches de surveillance ne le sont pas.

Tous les drapeaux de configuration du lancement d'origine ne sont pas restaurés. Si la session dépendait de `--mcp-config`, `--settings`, `--plugin-dir`, `--fallback-model` ou de répertoires ajoutés avec `--add-dir`, passez-les à nouveau lorsque vous reprenez ; les répertoires ajoutés en milieu de session avec `/add-dir` ne sont pas restaurés non plus, bien que le sélecteur de sessions les utilise toujours pour localiser la session. Les fichiers de paramètres standard, tels que `settings.json` et `settings.local.json`, sont relus au lancement, donc la configuration qui s'y trouve n'a pas besoin d'être passée à nouveau. Pour `--system-prompt` et `--append-system-prompt`, voir [Drapeaux d'invite système dans les conversations reprises](/docs/fr/cli-reference#system-prompt-flags-in-resumed-conversations).

<h4 id="permission-mode-on-resume">
  Mode de permission à la reprise
</h4>

Le mode de permission dans lequel Claude Code démarre une session reprise dépend de la façon dont vous la reprenez :

* Terminal : `claude --continue`, `claude --resume <session-id>` ou `claude --resume <name>` lorsque le nom correspond à une session, sans `-p`. Claude Code restaure le mode de permission dans lequel se trouvait la session, sauf dans les cas du tableau. Passez `--permission-mode` ou `--dangerously-skip-permissions` pour remplacer le mode restauré.
* Non-interactif : `claude -p --resume` ou `claude -p --continue`. Claude Code démarre l'exécution dans le mode de permission qu'une nouvelle exécution `claude -p` démarrerait, sauf qu'une session qui s'est terminée en mode plan reprend en mode plan selon les [conditions ci-dessous](#resume-in-plan-mode-with-p).
* VS Code : le panneau de conversation de l'extension. Le tableau couvre uniquement une conversation qui s'est terminée en mode plan ; pour le reste, voir [reprendre les conversations passées](/docs/fr/vs-code#resume-past-conversations).
* Sélecteur de sessions au lancement : une session que vous sélectionnez dans le [sélecteur de sessions](#use-the-session-picker), que vous l'ayez ouvert avec `claude --resume` seul, `claude --from-pr` ou un nom qui correspond à plus d'une session. Claude Code ne restaure pas le mode de permission stocké. Il démarre la session dans le mode de permission qu'il démarrerait une nouvelle session depuis la même ligne de commande.
* `/resume` à l'intérieur d'une session, avec ou sans argument : Claude Code ne restaure pas le mode de permission stocké. La conversation vers laquelle vous basculez continue dans le mode de permission dans lequel se trouve votre session actuelle.

La restauration du mode plan sur les chemins non-interactif et VS Code nécessite Claude Code v2.1.246 ou ultérieur. Chaque ligne nomme le mode de permission dans lequel la session s'est terminée, lequel des chemins terminal, non-interactif et VS Code vous la reprenez par, et le mode de permission dans lequel Claude Code démarre la session reprise.

| Session terminée en | Comment vous la reprenez                                                       | Mode de permission après la reprise                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------ | :----------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bypassPermissions` | Terminal                                                                       | Le mode de permission qu'une nouvelle session démarrerait. Pour [contourner les permissions](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode) à nouveau, activez-le au lancement avec l'un de ses drapeaux de lancement ou `permissions.defaultMode: "bypassPermissions"` dans les [paramètres utilisateur, `--settings` ou gérés](/docs/fr/settings-reference#permissions-defaultmode) |
| `plan`              | Terminal                                                                       | Le mode de permission qu'une nouvelle session démarrerait                                                                                                                                                                                                                                                                                                                                           |
| `auto`              | Terminal                                                                       | `auto`, uniquement lorsque votre compte répond toujours aux [exigences du mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                                                                                                                                                         |
| Manuel              | Terminal                                                                       | Manuel lorsqu'une nouvelle session démarrerait en mode auto à partir de la [valeur par défaut intégrée](/docs/fr/permission-modes#which-mode-a-session-starts-in). Lorsqu'un `defaultMode` d'un fichier de paramètres [prend effet](/docs/fr/permission-modes#which-mode-a-session-starts-in), Claude Code démarre la session reprise dans ce mode à la place                                                 |
| `plan`              | Non-interactif, selon les [conditions ci-dessous](#resume-in-plan-mode-with-p) | Mode plan                                                                                                                                                                                                                                                                                                                                                                                           |
| N'importe quel mode | Non-interactif, dans tout autre cas                                            | Le mode de permission qu'une nouvelle exécution `claude -p` démarrerait                                                                                                                                                                                                                                                                                                                             |
| `plan`              | VS Code                                                                        | Mode plan, avec [les exceptions sur la page VS Code](/docs/fr/vs-code#resume-past-conversations)                                                                                                                                                                                                                                                                                                         |

<h5 id="resume-in-plan-mode-with-p">
  Reprendre en mode plan avec `-p`
</h5>

Une exécution `claude -p --resume` ou `claude -p --continue` reprend en mode plan uniquement lorsque les quatre conditions sont remplies :

* Vous passez [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags), afin que Claude Code puisse présenter le plan pour approbation
* Vous ne passez pas `--permission-mode` ou `--dangerously-skip-permissions`
* Vous ne passez pas `--fork-session`
* L'exécution n'est pas démarrée via les [canaux](/docs/fr/channels)

<h3 id="resume-from-a-summary">
  Reprendre à partir d'un résumé
</h3>

Sur un plan Pro ou Max, lorsque vous reprenez une session qui a été inactive pendant plus d'une heure environ et dépasse 100 000 jetons, Claude Code restaure la conversation puis ouvre une boîte de dialogue avant que vous n'envoyiez votre premier message. Le [cache de prompt](/docs/fr/prompt-caching#cache-lifetime) de la session a expiré à ce moment-là, donc la prochaine demande traite l'historique complet une fois, quel que soit l'option de la boîte de dialogue que vous choisissez.

La boîte de dialogue offre trois façons de continuer la session. Elles diffèrent dans la quantité de conversation que chacune porte dans les demandes ultérieures, ce qui est un compromis entre conserver tous les détails et envoyer moins de jetons par demande :

* **Reprendre à partir du résumé** : exécute [`/compact`](/docs/fr/context-window#what-survives-compaction) immédiatement. Claude Code envoie une demande de résumé sur l'historique complet, puis remplace l'historique par le résumé, vos échanges les plus récents et jusqu'à cinq fichiers récemment lus. Les demandes ultérieures portent le résumé au lieu de l'historique complet.
* **Reprendre la session complète telle quelle** : charge la conversation inchangée. Après que vous ayez envoyé votre premier message, Claude Code retraite et re-met en cache l'historique complet, puis le relit à partir du cache sur les demandes ultérieures tant que le cache reste actif.
* **Ne me le demandez plus** : reprend la session complète et arrête d'afficher la boîte de dialogue sur toutes les reprises futures.

Reprendre tel quel conserve tous les détails de la conversation disponibles, à un coût par demande qui s'adapte à la taille de la conversation. Reprendre à partir du résumé coûte moins cher à chaque demande ultérieure car il porte le résumé au lieu de l'historique complet, mais tout ce que le résumé omet n'est plus dans le contexte de Claude. Voir [pourquoi l'utilisation augmente dans une longue session](/docs/fr/costs#why-usage-climbs-in-a-long-session) pour savoir d'où provient ce coût par demande.

<h3 id="where-the-session-picker-looks">
  Où le sélecteur de sessions cherche
</h3>

Claude Code stocke les sessions par répertoire de projet. Par défaut, le sélecteur de sessions affiche :

* Les sessions du worktree courant, y compris les [sessions en arrière-plan](/docs/fr/agent-view), qui sont marquées `bg` dans la liste
* Les sessions démarrées ailleurs qui ont ajouté le répertoire courant avec `/add-dir`

Utilisez `Ctrl+W` pour élargir à tous les worktrees du référentiel ou `Ctrl+A` pour élargir à chaque projet sur cette machine.

Les sessions dont la première invite était une commande [`/loop`](/docs/fr/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) n'apparaissent pas dans le sélecteur, et `claude --continue` les ignore également. L'exécution de `/loop` plus tard dans une conversation ne masque pas la session. Avant la v2.1.211, une exécution `/loop` au début d'une conversation masquait la session du sélecteur de manière permanente.

Le déplacement d'une session avec [`/cd`](/docs/fr/commands) la réinstalle dans le stockage de projet du nouveau répertoire, ce qui la fait apparaître dans le sélecteur de ce répertoire par la suite. À partir de la v2.1.196, une session déplacée reste absente du sélecteur de l'ancien répertoire même après un plantage ou une fermeture forcée. Sur les versions antérieures, elle pouvait aussi réapparaître dans la liste de l'ancien répertoire après une fermeture qui n'était pas propre lorsque l'ancien chemin contenait des caractères spéciaux tels que des traits de soulignement.

Lorsque vous sélectionnez une session d'un autre worktree du même référentiel, Claude Code la reprend sur place ; lorsque le propre worktree de la session n'existe plus, Claude Code [la reprend dans votre répertoire courant](/docs/fr/worktrees#resume-a-worktree-session). Lorsque vous sélectionnez une session d'un projet non lié, Claude Code copie une commande `cd` et de reprise dans votre presse-papiers à la place. Si le répertoire de ce projet n'existe plus, Claude Code reprend la session dans votre répertoire courant plutôt que de copier une commande `cd` qui échouerait.

La reprise par nom se résout dans le référentiel courant et ses worktrees. Les deux formes recherchent une correspondance exacte et la reprennent directement même si elle se trouve dans un worktree différent :

| Commande                 | Correspondance exacte | Nom ambigu                                                                                 |
| :----------------------- | :-------------------- | :----------------------------------------------------------------------------------------- |
| `claude --resume <name>` | Reprend directement   | Ouvre le sélecteur de sessions avec le nom pré-rempli comme terme de recherche             |
| `/resume <name>`         | Reprend directement   | Signale une erreur ; exécutez `/resume` sans argument pour ouvrir le sélecteur de sessions |

<h2 id="name-your-sessions">
  Nommer vos sessions
</h2>

Donnez aux sessions des noms descriptifs pour qu'elles soient trouvables dans le sélecteur de sessions et reprises par nom. Cela est particulièrement important lorsque vous travaillez sur plusieurs tâches en parallèle.

| Quand                                    | Comment définir le nom                                                                                                                                                                             |
| :--------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Au démarrage                             | `claude -n auth-refactor`                                                                                                                                                                          |
| Pendant une session                      | `/rename auth-refactor`. Le nom apparaît également dans la barre d'invite                                                                                                                          |
| À partir du sélecteur de sessions        | Mettez en surbrillance une session et appuyez sur `Ctrl+R`                                                                                                                                         |
| À l'acceptation du plan                  | L'acceptation d'un plan en [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) donne à la session un titre généré basé sur le plan sauf si vous en avez déjà nommé une        |
| Depuis claude.ai ou l'application Claude | Renommez une [session de contrôle à distance](/docs/fr/remote-control#connect-from-another-device) ; Claude Code applique le même nom dans la CLI. Nécessite Claude Code v2.1.221 ou version ultérieure |
| Depuis l'application de bureau           | Renommez une session dans l'[application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions)                                                                                                    |

Une fois qu'une session est nommée via une route CLI ou depuis claude.ai, revenez-y avec `claude --resume <name>` ou `/resume <name>` ; une session d'application de bureau reprend dans l'application, qui conserve son propre historique de sessions. Voir [Reprendre une session](#resume-a-session) pour savoir comment la résolution des noms se comporte entre les worktrees.

Lorsque vous démarrez ou reprenez une session interactive avec un nom qu'une autre session active sur cette machine utilise déjà, ou que vous renommez une session avec un tel nom, Claude Code laisse le nom à la session qui l'a déjà, renomme la vôtre en une variante avec un suffixe de deux mots, comme `auth-refactor-graceful-unicorn`, et vous le signale. Exécutez `/rename` avec un nouveau nom si vous préférez en choisir un vous-même. Avant la v2.1.232, les deux sessions conservaient le nom.

Dans trois cas, Claude Code ne renomme pas le doublon, vous pouvez donc toujours voir deux sessions avec le même nom dans les listes :

* Il ne vérifie pas les titres générés par l'IA ou les noms d'affichage par défaut.
* Il ne vérifie pas le `--name` d'une session [arrière-plan](/docs/fr/agent-view#from-your-shell) ou `-p` au démarrage.
* Il ne peut pas renommer une session sur une version antérieure de Claude Code.

Les sessions que vous ne nommez pas reçoivent quand même deux étiquettes que Claude Code attribue. Seul le titre généré fonctionne comme handle de reprise :

* Nom d'affichage par défaut : les sessions interactives que vous ne nommez jamais reçoivent quand même un nom d'affichage par défaut au démarrage. Nécessite Claude Code v2.1.196 ou version ultérieure. Le nom par défaut combine le nom du répertoire de travail avec un suffixe de deux caractères, par exemple `my-app-3f`, et identifie la session dans les listes de sessions en cours d'exécution, telles que la [vue agent](/docs/fr/agent-view) et la sortie `claude agents --json`. Le nom par défaut n'est pas un handle de reprise. Si vous le transmettez à `claude --resume` ou `/resume`, Claude Code ne trouve pas la session. Nommer la session remplace le nom par défaut dans ces listes, tout comme l'acceptation d'un plan.
* Titre généré : si vous ne nommez pas une session, Claude Code génère un titre de session pour elle. Le titre est un court résumé de votre première invite, écrit par une demande en arrière-plan au modèle petit/rapide, normalement un modèle de classe Haiku. Une exécution `claude -p` que vous démarrez directement à partir d'un shell ou d'un script n'en reçoit pas.

  L'acceptation d'un plan remplace le titre généré par un titre basé sur le plan. Nommer la session le remplace également.

  Vous voyez le titre de la première invite dans le [sélecteur de sessions](#use-the-session-picker) et dans le champ [`session_name`](/docs/fr/statusline) de la barre d'état lorsqu'aucun nom n'est défini. Le titre du plan s'affiche aux mêmes deux endroits et aussi dans les listes de sessions en cours d'exécution, où il remplace le nom d'affichage par défaut.

  Vous pouvez transmettre l'un ou l'autre titre à `claude --resume` ou `/resume`, et Claude Code le résout de la même manière qu'un nom que vous avez défini.

<h2 id="use-the-session-picker">
  Utiliser le sélecteur de sessions
</h2>

Exécutez `/resume` à l'intérieur d'une session, ou `claude --resume` sans arguments, pour ouvrir le sélecteur de sessions interactif. Utilisez ces raccourcis clavier pour naviguer, rechercher et élargir la liste :

| Raccourci                                          | Action                                                                                                                                                                                  |
| :------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                          | Naviguer entre les sessions                                                                                                                                                             |
| `→` / `←`                                          | Développer ou réduire les sessions groupées                                                                                                                                             |
| `Enter`                                            | Reprendre la session mise en surbrillance                                                                                                                                               |
| `Space`                                            | Prévisualiser le contenu de la session. `Ctrl+V` fonctionne également sur les terminaux qui ne le capturent pas comme collage                                                           |
| `Ctrl+R`                                           | Renommer la session mise en surbrillance                                                                                                                                                |
| `/` ou tout caractère imprimable autre que `Space` | Entrer en mode recherche et filtrer les sessions. Collez une URL de demande de tirage ou de fusion GitHub, GitHub Enterprise, GitLab ou Bitbucket pour trouver la session qui l'a créée |
| `Ctrl+A`                                           | Afficher les sessions de tous les projets sur cette machine. Appuyez à nouveau pour revenir au référentiel courant                                                                      |
| `Ctrl+W`                                           | Afficher les sessions de tous les worktrees du référentiel courant. Appuyez à nouveau pour revenir au worktree courant. Affiché uniquement dans les référentiels multi-worktrees        |
| `Ctrl+B`                                           | Filtrer les sessions de la branche git courante. Appuyez à nouveau pour afficher toutes les branches                                                                                    |
| `Esc`                                              | Quitter le sélecteur de sessions ou le mode recherche                                                                                                                                   |

Chaque ligne affiche le nom de la session s'il est défini, sinon le titre de session généré par l'IA, le résumé de la conversation ou la première invite, ainsi que le temps écoulé depuis la dernière activité, la branche git et la taille du fichier. Élargissez à tous les projets avec `Ctrl+A` pour voir également le chemin du projet de chaque session.

Les sessions créées avec `/branch` ou `--fork-session` obtiennent leurs propres ID de session et apparaissent comme des lignes distinctes. Lorsque le sélecteur trouve plus d'une entrée pour la même session, il les groupe sous une seule ligne. Appuyez sur `→` pour développer un groupe.

Si Claude Code ne peut pas charger la session que vous sélectionnez dans le sélecteur `claude --resume`, il affiche [`Impossible de reprendre la conversation`](/docs/fr/errors#failed-to-resume-the-conversation) avec une commande pour réessayer, puis se termine avec le code 1. À partir du sélecteur `/resume` à l'intérieur d'une session, Claude Code signale l'échec et votre conversation actuelle continue de s'exécuter.

<h2 id="branch-a-session">
  Créer une branche d'une session
</h2>

La création d'une branche crée une copie de la conversation jusqu'à présent et vous y bascule, laissant l'original intact. Utilisez-la pour essayer une approche différente sans perdre le chemin sur lequel vous étiez.

À partir d'une session, exécutez `/branch` avec un nom optionnel :

```text theme={null}
/branch try-streaming-approach
```

Si vous omettez le nom, Claude Code nomme la nouvelle branche d'après la première invite de la conversation. À partir de la v2.1.198, cela s'applique également après [compaction](/docs/fr/how-claude-code-works#when-context-fills-up) ; les versions antérieures revenaient au nom littéral `Branched conversation` au lieu de regarder au-delà du résumé de compaction jusqu'à la première invite originale.

À partir de la ligne de commande, combinez `--continue` ou `--resume` avec `--fork-session` :

```bash theme={null}
claude --continue --fork-session
```

La confirmation `/branch` imprime deux ID de session : la nouvelle branche dans laquelle vous êtes maintenant et l'original. L'original est inchangé sur le disque et reste dans le sélecteur de sessions ; revenez-y avec `/resume <original-name>` ou en passant son ID à `/resume`.

`/branch` copie la transcription et bascule le processus Claude Code en cours d'exécution pour écrire dedans. Cette distinction détermine ce que la branche hérite :

| État                                                                                                                                                                                | Après `/branch`                                                                                                                                                                                                                                                |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Historique de conversation                                                                                                                                                          | Copié dans la branche jusqu'au point où vous avez exécuté `/branch`                                                                                                                                                                                            |
| Autorisations « Autoriser pour cette session »                                                                                                                                      | Reportées ; la branche s'exécute dans le même processus, donc vos autorisations existantes s'appliquent toujours. Si vous créez une branche dans un processus séparé avec `--fork-session`, le nouveau processus démarre sans elles et vous les réapprouvez là |
| [Sous-agents en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) et [commandes Bash en arrière-plan](/docs/fr/interactive-mode#background-bash-commands) en cours | Continuent à s'exécuter. Leur sortie apparaît dans la nouvelle branche dans laquelle vous avez basculé, pas dans la session originale                                                                                                                          |
| Connexion [Contrôle à distance](/docs/fr/remote-control)                                                                                                                                 | Reste connectée. Un téléphone ou un navigateur connecté à la session vous suit dans la branche et continue à recevoir de nouveaux messages là                                                                                                                  |

Si vous reprenez la même session dans deux terminaux sans créer de branche, les messages des deux s'entrelacent dans une seule transcription. Pour le rembobinage basé sur les points de contrôle au sein d'une seule session, voir [Points de contrôle](/docs/fr/checkpointing).

<h2 id="manage-context-within-a-session">
  Gérer le contexte au sein d'une session
</h2>

Ces commandes contrôlent ce qui se trouve dans la fenêtre de contexte sans quitter la session :

* **`/clear`** : recommencer avec un contexte vide. Claude Code enregistre la conversation précédente ; reprenez-la avec `/resume`, ou, dans le même processus Claude Code, depuis [l'entrée de session précédente du menu de rembobinage](/docs/fr/checkpointing#rewind-past-a-cleared-conversation). Sans argument, la nouvelle conversation conserve un nom que vous avez défini avec `--name` ou `/rename`, mais pas un titre de session généré par l'IA. Pour nommer la conversation que vous quittez, passez le nom, comme dans `/clear release-prep` ; la nouvelle conversation démarre alors sans nom
* **`/compact [instructions]`** : remplacer l'historique par un résumé, optionnellement axé sur ce que vous spécifiez
* **`/context`** : afficher ce qui consomme actuellement le contexte

Pour savoir comment la compaction interagit avec CLAUDE.md, les compétences et les règles, voir le [guide de la fenêtre de contexte](/docs/fr/context-window). Pour les stratégies sur quand effacer par rapport à compacter, voir [Meilleures pratiques](/docs/fr/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Exporter et localiser les données de session
</h2>

Exécutez `/export` pour ouvrir un menu qui vous permet de copier la conversation courante dans votre presse-papiers ou de l'enregistrer en tant que fichier texte brut, avec les messages et les sorties d'outils rendus sous forme de texte lisible. Passez un nom de fichier pour ignorer le menu et écrire directement dans ce fichier.

<h3 id="access-conversations-from-scripts">
  Accéder aux conversations à partir de scripts
</h3>

`/export` produit une transcription rendue pour qu'une personne la lise. Les interfaces ci-dessous produisent des données structurées pour qu'un script les analyse : un résultat JSON d'une exécution, le chemin vers le fichier de transcription d'une session, ou un flux en direct d'événements. Choisissez en fonction de ce qui déclenche le script :

* **Exécuter Claude une fois et capturer le résultat** : invoquez `claude -p` avec [`--output-format json` ou `stream-json`](/docs/fr/headless#get-structured-output) pour capturer le résultat, l'ID de session, l'utilisation et le coût d'une exécution non interactive sous forme de JSON structuré.
* **Poser une question à une session existante** : passez un ID de session à [`claude -p --resume`](/docs/fr/headless#continue-conversations) pour envoyer une invite de suivi, comme une demande de résumé, et capturer la réponse structurée.
* **Réagir aux événements de session** : lisez le champ `transcript_path` que les [hooks](/docs/fr/hooks#common-input-fields) et les [commandes de ligne d'état](/docs/fr/statusline#available-data) reçoivent en entrée. Un hook `SessionEnd` peut archiver la transcription lorsqu'une session se termine.
* **Intégrer Claude dans une application TypeScript ou Python** : utilisez le [Agent SDK](/docs/fr/agent-sdk/overview) pour recevoir chaque message par programmation.

L'exemple ci-dessous utilise la deuxième interface. Il envoie une invite de suivi à une session existante et lit la réponse avec `jq` :

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Où les transcriptions sont stockées
</h3>

Par défaut, Claude Code stocke les transcriptions en JSONL à `~/.claude/projects/<project>/<session-id>.jsonl`, où `<project>` est votre chemin de répertoire de travail avec les caractères non alphanumériques remplacés par `-`. Pour un répertoire de travail dont le nom converti dépasse 200 caractères, Claude Code tronque le nom à 200 caractères et ajoute un hash du chemin complet, de sorte que le nom du répertoire reste dans les limites du système de fichiers.

Chaque ligne est un objet JSON pour un message, une utilisation d'outil ou une entrée de métadonnées. Le format d'entrée est interne à Claude Code et change entre les versions, donc les scripts qui analysent directement ces fichiers peuvent se casser à chaque version. Pour construire sur les données de session, utilisez `/export` ou les [interfaces de script](#access-conversations-from-scripts) à la place.

L'emplacement, la rétention et le comportement d'écriture sont configurables :

| Pour                                                                                                                       | Définir                                                                                     | Où                                                        |
| -------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| Déplacer le stockage hors de `~/.claude`                                                                                   | [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars)                                                         | Variable d'environnement                                  |
| [Nommer le répertoire `<project>` vous-même](#name-the-project-directory-yourself)                                         | [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/fr/env-vars)                                              | Variable d'environnement                                  |
| Modifier la rétention de 30 jours                                                                                          | [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays)                             | `settings.json`                                           |
| Définir une limite d'âge pour les [transcriptions Claude Desktop et Cowork](/docs/fr/claude-directory#cleaned-up-automatically) | [`desktopSessionCleanupPeriodDays`](/docs/fr/settings-reference#desktopsessioncleanupperioddays) | Paramètres utilisateur, paramètres gérés, ou `--settings` |
| Supprimer les écritures de transcription dans tous les modes                                                               | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/fr/env-vars)                                           | Variable d'environnement                                  |
| Supprimer les écritures pour une exécution non interactive                                                                 | [`--no-session-persistence`](/docs/fr/cli-reference)                                             | Drapeau CLI avec `claude -p`                              |

<h3 id="delete-session-data">
  Supprimer les données de session
</h3>

Les transcriptions arrivent à expiration selon les [règles de nettoyage de rétention](/docs/fr/claude-directory#cleaned-up-automatically). Pour supprimer les transcriptions d'un projet et l'état associé plus tôt, exécutez [`claude project purge`](/docs/fr/claude-directory#clear-local-data). Si vous supprimez une [session en arrière-plan](/docs/fr/agent-view) avec [`claude rm <id>`](/docs/fr/agent-view#what-deleting-a-session-removes), sa transcription reste sur le disque et reste disponible via `claude --resume`.

<h3 id="name-the-project-directory-yourself">
  Nommer le répertoire du projet vous-même
</h3>

Par défaut, Claude Code dérive le nom `<project>` du chemin complet du répertoire de travail. Pour choisir le nom vous-même, définissez `CLAUDE_CODE_PROJECT_DIR_NAME` aux côtés de `CLAUDE_CONFIG_DIR`. Claude Code stocke ensuite les transcriptions de cette session et la [mémoire automatique](/docs/fr/memory#auto-memory) sous votre nom. Cela convient à un hôte qui intègre Claude Code et donne à chaque session son propre répertoire de configuration. Nécessite Claude Code v2.1.234 ou ultérieur.

Par exemple, ce lancement garde les données du locataire A sous `/srv/tenant-a` et nomme son répertoire de projet `work` :

```bash theme={null}
CLAUDE_CONFIG_DIR=/srv/tenant-a CLAUDE_CODE_PROJECT_DIR_NAME=work claude
```

Claude Code écrit les transcriptions de la session dans `/srv/tenant-a/projects/work/` et sa mémoire automatique dans `/srv/tenant-a/projects/work/memory/`, quel que soit le répertoire de travail.

Trois règles s'appliquent lorsque vous le définissez :

* **Définissez aussi `CLAUDE_CONFIG_DIR`** : le nom ne varie pas avec le répertoire de travail, donc sous le `~/.claude` par défaut, il fusionnerait les transcriptions et la mémoire automatique de chaque projet dans un seul répertoire. Claude Code ignore `CLAUDE_CODE_PROJECT_DIR_NAME` lorsque `CLAUDE_CONFIG_DIR` n'est pas défini.
* **Utilisez 1-64 lettres, chiffres, tirets ou traits de soulignement** : n'utilisez pas un nom de périphérique Windows tel que `con`. Claude Code ignore toute autre valeur et utilise le nom dérivé.
* **Définissez-le dans l'environnement shell qui démarre `claude`** : Claude Code le lit une seule fois au démarrage à partir de cet environnement, donc un bloc `env` dans un fichier de paramètres ne peut pas le définir.

Une fois que vous avez nommé le répertoire de projet d'un répertoire de configuration, continuez à lancer avec ce nom. Si vous démarrez Claude Code avec le même `CLAUDE_CONFIG_DIR` mais sans `CLAUDE_CODE_PROJECT_DIR_NAME`, il lit et écrit à nouveau le répertoire dérivé. Les sessions stockées sous votre nom restent sur le disque : appuyez sur `Ctrl+A` dans le [sélecteur de session](#use-the-session-picker) pour lister les sessions de chaque répertoire de projet sous ce répertoire de configuration, celui épinglé inclus, et quelle que soit la façon dont vous lancez, [`claude --resume <session-id>`](#resume-a-session) trouve une session stockée sous l'un ou l'autre nom.

<h2 id="see-also">
  Voir aussi
</h2>

Ces pages couvrent les mécaniques de session et de parallélisme connexes :

* [Worktrees](/docs/fr/worktrees) : exécuter des sessions parallèles isolées sur des branches séparées
* [Points de contrôle](/docs/fr/checkpointing) : rembobiner le code et la conversation à un point antérieur
* [Fenêtre de contexte](/docs/fr/context-window) : ce qui remplit le contexte et ce qui survit à la compaction
* [Mode non interactif](/docs/fr/headless) : comportement de la session sous `claude -p`
