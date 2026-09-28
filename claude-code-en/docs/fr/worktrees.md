> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Exécuter des sessions parallèles avec worktrees

> Isolez les sessions Claude Code parallèles dans des git worktrees séparés pour que les modifications ne se heurtent pas. Couvre le flag `--worktree`, l'isolation des subagents, `.worktreeinclude`, le nettoyage et les hooks VCS non-git.

Un [git worktree](https://git-scm.com/docs/git-worktree) est un répertoire de travail séparé avec ses propres fichiers et branche, partageant le même historique de dépôt et la même télécommande que votre extraction principale. Exécuter chaque session Claude Code dans son propre worktree signifie que les modifications dans une session ne touchent jamais les fichiers d'une autre, vous pouvez donc avoir Claude construisant une fonctionnalité dans un terminal tout en corrigeant un bug dans un second.

<Note>
  Les worktrees nécessitent un dépôt git ; pour les autres systèmes de contrôle de version, [configurez des hooks pour remplacer la logique git](#non-git-version-control). Dans l'[application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions), sélectionnez l'option **worktree** quand vous démarrez une session pour lui donner son propre worktree.
</Note>

Les worktrees sont l'une des plusieurs façons d'exécuter Claude en parallèle. Ils isolent les modifications de fichiers. Les [subagents](/docs/fr/sub-agents) divisent le travail à l'intérieur d'une session, et la [messagerie entre sessions](/docs/fr/cross-session-messaging) permet à Claude de transmettre les résultats entre les sessions dans vos worktrees. Voir [Exécuter les agents en parallèle](/docs/fr/agents) pour comparer les approches, ou passez directement à [Isoler les subagents avec worktrees](#isolate-subagents-with-worktrees) pour utiliser les worktrees et les subagents ensemble.

La plupart des sessions n'ont besoin que des deux premières sections : [démarrer Claude dans un worktree](#start-claude-in-a-worktree), puis [nettoyer quand vous quittez](#clean-up-worktrees). Revenez au reste de la page quand vous avez besoin de [reprendre une session](#resume-a-worktree-session), [modifier la façon dont les worktrees sont créés](#customize-worktree-creation), ou [déboguer une défaillance](#troubleshooting).

<h2 id="start-claude-in-a-worktree">
  Démarrer Claude dans un worktree
</h2>

Passez `--worktree` ou `-w` avec un nom pour créer un worktree isolé et démarrer Claude dedans. Par défaut, le worktree est créé sous `.claude/worktrees/<name>/` à la racine de votre dépôt, sur une nouvelle branche nommée `worktree-<name>` :

```bash theme={null}
claude --worktree feature-auth
```

Exécutez la commande à nouveau avec un nom différent dans un autre terminal pour démarrer une deuxième session isolée. Si vous omettez le nom, Claude en génère un tel que `bright-running-fox`.

Les exécutions interactives nécessitent la [confiance de l'espace de travail](/docs/fr/security) : si vous n'avez pas exécuté Claude dans le répertoire auparavant, exécutez `claude` une fois là-bas pour accepter la boîte de dialogue de confiance, ou `--worktree` se termine avec une erreur vous invitant à le faire. Les exécutions non interactives avec `-p` ignorent la vérification de confiance, donc `claude -p --worktree` procède sans elle.

<Tip>
  Ajoutez `.claude/worktrees/` à votre `.gitignore` pour que le contenu des worktrees n'apparaisse pas comme des fichiers non suivis dans votre extraction principale.
</Tip>

<h3 id="set-up-the-worktree-environment">
  Configurer l'environnement du worktree
</h3>

Un worktree est une extraction fraîche, donc initialisez votre environnement de développement là-bas : demandez à Claude d'installer les dépendances, ou exécutez la configuration de votre projet vous-même dans le répertoire worktree sous `.claude/worktrees/`. Pour transporter les fichiers gitignorés tels que `.env` dans chaque nouveau worktree automatiquement, ajoutez un [fichier `.worktreeinclude`](#copy-gitignored-files-into-worktrees).

<h3 id="ask-claude-to-create-a-worktree">
  Demander à Claude de créer un worktree
</h3>

Vous pouvez également demander à Claude de « travailler dans un worktree » pendant une session, et il en crée un avec l'outil [`EnterWorktree`](/docs/fr/tools-reference). Une fois dans un worktree, Claude peut basculer directement vers un autre sous `.claude/worktrees/` en appelant `EnterWorktree` avec le chemin cible ; le worktree précédent reste sur le disque intact.

Quand Claude entre dans un chemin en dehors du répertoire `.claude/worktrees/` du dépôt, Claude Code demande d'abord votre approbation, car le déplacement prend le répertoire de travail de la session, l'accès en écriture et la configuration du projet telle que `CLAUDE.md` et les paramètres vers cet emplacement. Une règle de permission [`EnterWorktree`](/docs/fr/permissions) ou le choix de « ne plus demander » ne supprime pas cette invite ; seul le mode `bypassPermissions` l'ignore. Avant la v2.1.206, Claude pouvait entrer dans n'importe quel chemin de worktree existant sans demander.

<Note>
  **Les chemins des hooks ne suivent pas le worktree.** Après que Claude entre dans un worktree, Claude Code garde `${CLAUDE_PROJECT_DIR}` dans vos [hooks](/docs/fr/hooks#reference-scripts-by-path) où il était et transmet le chemin du worktree d'une autre manière :

  * **`${CLAUDE_PROJECT_DIR}` reste en place** : il pointe toujours vers la racine du projet où la session a commencé, donc une commande de hook telle que `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` exécute toujours le script dans l'extraction principale.
  * **`cwd` suit Claude** : le champ `cwd` dans le [JSON d'entrée](/docs/fr/hooks#common-input-fields) du hook est la racine du worktree, et il se déplace à nouveau quand Claude exécute `cd`. Lisez-le quand un hook a besoin du chemin du worktree.
</Note>

<h2 id="clean-up-worktrees">
  Nettoyer les worktrees
</h2>

Quand vous quittez une session worktree interactive, Claude vérifie le worktree pour le travail que la suppression supprimerait : fichiers modifiés ou non suivis, travail non validé à l'intérieur des sous-modules extraits, et nouveaux commits.

* **Le worktree est propre** : pour une session sans nom, Claude supprime le worktree et sa branche automatiquement. Une session [nommée](/docs/fr/sessions#name-your-sessions) vous demande d'abord pour que vous puissiez conserver le worktree pour plus tard
* **Le worktree contient du travail** : Claude vous demande de conserver ou de supprimer le worktree. Conserver préserve le répertoire et la branche pour que vous puissiez revenir plus tard. Supprimer supprime le répertoire worktree et sa branche, ainsi que tout le travail qu'ils contiennent
* **L'état du worktree ne peut pas être vérifié** : quand Claude Code ne peut pas compter les modifications du worktree ou ne peut pas inspecter les extraits de ses sous-modules, il vous demande plutôt que de supprimer le worktree automatiquement. L'invite nomme ce qu'il n'a pas pu vérifier

Les exécutions non interactives avec `-p` n'ont pas d'invite de sortie, donc Claude ne nettoie pas leurs worktrees, et Claude Code laisse le verrou qu'il a pris sur chacun à la création en place jusqu'à ce qu'une [balayage de verrous obsolètes](#clean-up-subagent-and-background-session-worktrees) ultérieure le libère. Pour en supprimer un, exécutez `git worktree remove` ; si git refuse parce que le worktree est verrouillé, exécutez d'abord `git worktree unlock` dessus.

Sur Windows, la suppression d'un worktree ne supprime pas les fichiers en dehors de lui. Si un dossier à l'intérieur du worktree est un lien vers ailleurs, comme une jonction NTFS ou un lien symbolique de répertoire, Claude Code supprime uniquement le lien et conserve le dossier vers lequel il pointe. Avant la v2.1.205, la suppression d'un worktree avec un lien imbriqué dans un sous-répertoire pouvait supprimer le dossier vers lequel il pointait.

<h2 id="resume-a-worktree-session">
  Reprendre une session worktree
</h2>

Quand vous reprenez une session qui était à l'intérieur d'un worktree, Claude Code retourne la session à ce worktree. Cela s'applique aux reprises interactives, à `--continue` et `--resume` en [mode non interactif](/docs/fr/headless) avec `-p`, et au SDK Agent. De retour à l'intérieur du worktree, Claude peut toujours le quitter avec l'outil [`ExitWorktree`](/docs/fr/tools-reference).

Avant de retourner la session à son worktree, Claude Code vérifie que le worktree est toujours une extraction séparée de la principale, et refuse de réentrer dans un worktree qui échoue la vérification. Pour un git worktree, la vérification lit ses métadonnées git. Un worktree sans métadonnées git, comme celui qu'un hook [`WorktreeCreate`](#non-git-version-control) a créé, peut réussir la vérification ; les cas que Claude Code refuse toujours sont listés avec leurs récupérations sous [Claude Code refuse d'utiliser un worktree](#claude-code-refuses-to-use-a-worktree). Pour les messages et comment récupérer de chacun, voir [La session reprend en dehors de son worktree](#the-session-resumes-outside-its-worktree).

Où vous lancez et comment vous reprenez changent ce que Claude Code réentre :

* **Répertoire de lancement** : reprenez à partir de l'extraction principale ou d'un autre répertoire du dépôt. Claude Code réentre dans un worktree qu'il a créé avec git sous `.claude/worktrees/` même quand vous lancez depuis l'intérieur. Quand vous lancez depuis l'intérieur de tout autre worktree, Claude Code ne le réentre que s'il peut le justifier de là : un worktree qui est son propre dépôt, un sans métadonnées git, ou un lancement depuis un sous-répertoire d'un worktree que vous avez créé avec `git worktree add` refuse, donc lancez ceux-ci depuis l'extraction principale.
* **`--fork-session`** : la session dupliquée commence dans le répertoire à partir duquel vous avez lancé Claude, et Claude Code laisse le worktree de la session d'origine intact.
* **Worktree supprimé** : si le répertoire worktree n'existe plus, Claude Code reprend la session dans le répertoire à partir duquel vous avez lancé Claude. Il vous dit que le worktree est parti et efface la liaison du worktree de la session.

<Note>
  Avant la v2.1.212, une reprise non interactive restait dans le répertoire de démarrage et `ExitWorktree` signalait qu'il n'y avait pas de session worktree active à quitter.
</Note>

Quand Claude entre ou quitte un worktree que Claude Code a créé avec git, la transcription suit : Claude Code enregistre la session sous le nouveau répertoire de travail de la session, de la même manière que [`/cd`](/docs/fr/commands) le fait, donc `/desktop` et `--resume` la trouvent là. Quitter la déplace de la même manière. Un worktree créé par un hook [`WorktreeCreate`](#non-git-version-control) conserve sa transcription au répertoire de lancement. Nécessite Claude Code v2.1.198 ou ultérieur.

<h2 id="how-claude-code-enforces-isolation">
  Comment Claude Code applique l'isolation
</h2>

Pendant qu'une session est isolée dans un worktree, Claude Code bloque les appels d'outils que les vérifications ci-dessous définissent. Les mêmes règles s'appliquent que vous ayez démarré la session avec `--worktree`, que Claude soit entré dans un worktree avec `EnterWorktree`, ou que vous ayez repris une session worktree.

L'application de l'isolation couvre chaque subagent que Claude génère à partir de la session isolée. Elle s'applique que la session soit interactive ou s'exécute en [arrière-plan](/docs/fr/agent-view#how-file-edits-are-isolated). Les [subagents qui s'exécutent dans leur propre worktree](#isolate-subagents-with-worktrees) portent les mêmes vérifications. Leur historique de version est sous [Écrire les fichiers des subagents](/docs/fr/sub-agents#write-subagent-files).

Claude Code applique quatre vérifications :

* **Modifications de fichiers** : Claude Code bloque une `Edit`, `Write`, ou `NotebookEdit` qui cible un chemin dans l'extraction principale.
* **Répertoire de travail de la commande** : Claude Code bloque une commande Bash, PowerShell, ou Monitor dont le répertoire de travail se résout à l'extraction principale, ou dont le répertoire de travail il ne peut pas vérifier reste en dehors.
* **Redirections Git** : Claude Code bloque une commande Bash ou Monitor qui redirige git vers l'extraction principale. La redirection peut provenir de `git -C`, `--git-dir`, une variable `GIT_DIR` ou `GIT_WORK_TREE`, ou un `cd` dans l'extraction principale avant d'exécuter git.
* **Forme de la commande** : Claude Code bloque une commande Bash ou Monitor quand il ne peut pas vérifier à partir du texte de la commande que tout git que la commande exécute reste à l'intérieur du worktree. Cela se produit, par exemple, quand le nom de la commande est calculé à l'exécution, quand la syntaxe ne peut pas être analysée, ou quand une expansion telle que `${!name}` ou `${ command; }` pourrait exécuter une commande que le texte ne précise pas. Claude Code indique à Claude comment réécrire la commande refusée, par exemple en la divisant en commandes simples et séparées. Vous ne pouvez pas désactiver cette vérification.

Les vérifications s'appliquent au dépôt à partir duquel vous avez lancé Claude Code. Elles couvrent également l'extraction principale à laquelle un worktree lié est lié. Pour les commandes PowerShell, Claude Code applique uniquement la vérification du répertoire de travail.

Claude voit chaque refus comme une erreur d'outil qui nomme le worktree et dit comment procéder. Pour une commande refusée, consultez [ce que le message de refus signifie et comment le résoudre](/docs/fr/errors#command-blocked-by-the-worktree-isolation-checks).

<h2 id="isolate-subagents-with-worktrees">
  Isoler les subagents avec worktrees
</h2>

Les subagents peuvent s'exécuter dans leurs propres worktrees pour que les modifications parallèles ne se heurtent pas. Demandez à Claude d'« utiliser les worktrees pour vos agents », ou rendez l'isolation permanente pour un [subagent personnalisé](/docs/fr/sub-agents#supported-frontmatter-fields) en ajoutant `isolation: worktree` à son frontmatter.

Ce subagent dans `.claude/agents/` s'exécute toujours dans son propre worktree :

```markdown theme={null}
---
name: refactorer
description: Applies mechanical refactors across many files
isolation: worktree
---

Apply the requested refactor across every affected file, then run the tests
and report the results.
```

Chaque subagent obtient un worktree temporaire que Claude Code supprime automatiquement quand le subagent se termine sans modifications ; un worktree avec des modifications reste sur le disque jusqu'à ce que le [balayage périodique ci-dessous](#clean-up-subagent-and-background-session-worktrees) puisse le supprimer sans perdre de travail.

Les worktrees des subagents utilisent la même [branche de base](#choose-the-base-branch) que `--worktree`, donc ils se ramifient à partir de la branche par défaut de votre dépôt sauf si `worktree.baseRef` est défini sur `"head"`.

<h3 id="clean-up-subagent-and-background-session-worktrees">
  Nettoyer les worktrees des subagents et des sessions en arrière-plan
</h3>

Claude Code exécute un balayage périodique qui supprime les worktrees que Claude a créés pour les subagents et les [sessions en arrière-plan](/docs/fr/agent-view#how-file-edits-are-isolated) une fois qu'ils sont plus anciens que votre paramètre [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays), en suivant les [règles de balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically).

Quand vous [mettez en arrière-plan](/docs/fr/agent-view#send-the-session-to-the-background) une session `--worktree`, son worktree devient un worktree de session en arrière-plan que le balayage peut supprimer. Le balayage laisse un worktree en place dans ces cas :

* Le worktree contient toujours du travail : fichiers modifiés ou non suivis, ou commits non poussés.
* Un sous-module extrait dans le worktree contient des fichiers modifiés ou non suivis, ou Claude Code ne peut pas inspecter les sous-modules du worktree. Cette vérification nécessite Claude Code v2.1.274 ou une version ultérieure.
* L'un des [quatre cas qui bloquent également la création de worktree](#git-lfs-content-is-missing-from-a-worktree-claude-code-created) s'applique : Claude Code ne peut pas déterminer quels pilotes de filtre la configuration du dépôt définit, ou y trouve un paramètre qu'il ne peut pas désactiver.
* Le worktree appartient à une session `--worktree` que vous n'avez pas mise en arrière-plan, quel que soit son âge.
* Vous avez créé le worktree vous-même avec `git worktree add`, même si vous avez ensuite exécuté une session `--worktree <name>` dedans et l'avez mise en arrière-plan.

Claude Code écrit un marqueur dans les métadonnées git de chaque worktree qu'il crée avec git, et le balayage conserve tout worktree sans celui-ci, y compris un worktree qu'un hook [`WorktreeCreate`](#non-git-version-control) a créé. Avant la v2.1.246, le balayage ne vérifiait pas le marqueur, et pouvait supprimer un worktree que vous avez créé vous-même quand un ancien enregistrement de session en arrière-plan pointait vers lui.

Pendant qu'un agent s'exécute, Claude Code maintient un `git worktree lock` sur son worktree pour que le nettoyage concurrent ne puisse pas le supprimer, et libère le verrou quand l'agent se termine. Claude Code maintient le même verrou sur le worktree qu'il a créé pour une session mise en arrière-plan pendant que la session s'exécute, pour que le balayage laisse le worktree en place et `git worktree remove` refuse de le supprimer.

Le balayage libère également un verrou que Claude Code a défini pour une session dont le processus a quitté, pour qu'une session en arrière-plan tuée ne laisse pas son worktree définitivement verrouillé. Le balayage ne libère jamais un verrou que vous avez défini vous-même avec `git worktree lock`. Avant la v2.1.210, un verrou laissé par une session tuée restait en place jusqu'à ce que vous exécutiez `git worktree unlock`.

Pour nettoyer un worktree que le balayage conserve, exécutez `git worktree remove`, en ajoutant `--force` si le worktree a des modifications non validées ou des fichiers non suivis. Si git refuse parce que le worktree est verrouillé, exécutez d'abord `git worktree unlock` dessus.

<h2 id="customize-worktree-creation">
  Personnaliser la création de worktrees
</h2>

Les valeurs par défaut de Claude Code pour créer des worktrees couvrent la plupart des sessions : il les crée sous `.claude/worktrees/`, les ramifie à partir de la branche par défaut de votre dépôt, et extrait uniquement les fichiers suivis. Les options de cette section changent ces valeurs par défaut.

<h3 id="choose-the-base-branch">
  Choisir la branche de base
</h3>

Les nouveaux worktrees se ramifient à partir de la branche par défaut du dépôt, donc la plupart des sessions n'ont pas besoin de ce paramètre. Définissez `worktree.baseRef` dans [paramètres](/docs/fr/settings-reference#worktree) pour vous ramifier à partir de votre travail actuel à la place. Le paramètre accepte deux valeurs :

* `"fresh"` (par défaut) : se ramifie à partir de la branche par défaut du dépôt sur la télécommande, généralement `main`, pour que le worktree commence à partir d'un arbre propre correspondant à la télécommande.
* `"head"` : se ramifie à partir de votre `HEAD` local actuel, pour que le worktree porte vos commits non poussés et l'état de la branche de fonctionnalité. Utilisez ceci lors de l'isolation des subagents qui doivent opérer sur un travail en cours. À l'intérieur d'un worktree, `"head"` se résout en `HEAD` de ce worktree, pas en celui de l'extraction principale.

Vous ne pouvez pas définir `worktree.baseRef` sur un nom de branche. Pour démarrer un worktree à partir d'une branche existante spécifique, [créez-le avec git directement](#manage-worktrees-manually).

Pour une base `"fresh"`, Claude Code garde `origin/HEAD` à jour : quand le dépôt n'a pas été récupéré au cours des 24 dernières heures, il récupère la branche par défaut, limité à cinq secondes, et utilise la ref mise en cache localement si la récupération échoue. Si aucune télécommande n'est configurée, ou si `origin/HEAD` n'est pas mis en cache localement et ne peut pas être récupéré, le worktree revient à votre `HEAD` local actuel. Avant la v2.1.208, un worktree frais utilisait tout ce qui était déjà mis en cache localement pour `origin/HEAD`.

Cet exemple fait que chaque nouveau worktree se ramifie à partir de votre travail actuel :

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

<h3 id="branch-from-a-pull-request">
  Se ramifier à partir d'une demande de tirage
</h3>

Pour vous ramifier à partir d'une demande de tirage ou d'une demande de fusion spécifique, passez `--worktree` le numéro préfixé par `#`, une URL de demande de tirage GitHub, ou une URL de demande de fusion GitLab telle que `https://gitlab.com/group/repo/-/merge_requests/123`. Claude Code récupère le commit de tête de ce changement à partir de `origin` et crée le worktree à `.claude/worktrees/pr-<number>`. Citez l'argument pour que votre shell ne traite pas `#` comme le début d'un commentaire :

```bash theme={null}
claude --worktree "#1234"
```

Claude Code lit uniquement le numéro à partir de l'URL. Il récupère toujours à partir de la télécommande `origin` de votre dépôt, et choisit le chemin de récupération par l'hôte de `origin` :

* **github.com** : récupère `pull/<number>/head`
* **gitlab.com** : récupère `merge-requests/<number>/head`
* **GitHub Enterprise, GitLab auto-hébergé, ou tout autre hôte** : essaie d'abord `pull/<number>/head`, puis `merge-requests/<number>/head`

Avant la v2.1.233, Claude Code acceptait uniquement `#<number>` et les URL de demande de tirage de style GitHub pour `--worktree`, et récupérait toujours `pull/<number>/head`.

<h3 id="copy-gitignored-files-into-worktrees">
  Copier les fichiers gitignorés dans les worktrees
</h3>

Un worktree est une extraction fraîche, donc les fichiers non suivis comme `.env` ou `.env.local` de votre dépôt principal ne sont pas présents. Pour les copier automatiquement quand Claude crée un worktree, ajoutez un fichier `.worktreeinclude` à la racine de votre projet.

Le fichier utilise la syntaxe `.gitignore`. Seuls les fichiers qui correspondent à un motif et sont également gitignorés sont copiés, donc les fichiers suivis ne sont jamais dupliqués.

Si vous écrivez un motif qui commence par `**/` et les fichiers que vous voulez sont à l'intérieur d'un répertoire qui est gitignoré dans son ensemble, Claude Code les copie uniquement quand ce répertoire lui-même correspond au motif, ou quand le premier nom après le `**/` est l'un des noms dans le chemin du répertoire. Par exemple, si vous écrivez `**/.claude/skills/*.md`, ce premier nom est `.claude`, donc Claude Code copie les fichiers correspondants hors d'un répertoire `.claude/` ignoré. Pour copier les fichiers hors d'un répertoire ignoré qu'un motif `**/` n'atteint pas, nommez le répertoire dans le motif à la place : écrivez `vendor/**/config.json` plutôt que `**/config.json`. Avant la v2.1.239, Claude Code copiait les fichiers hors d'un répertoire entièrement ignoré pour un motif `**/` uniquement quand le répertoire lui-même correspondait au motif.

Ce `.worktreeinclude` copie deux fichiers env et une configuration de secrets dans chaque nouveau worktree :

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Cela s'applique à chaque worktree que Claude Code crée avec git : les worktrees `--worktree`, les [worktrees de subagents](#isolate-subagents-with-worktrees), et les sessions parallèles dans l'[application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions). Avec un hook [`WorktreeCreate`](#non-git-version-control), copiez les fichiers à l'intérieur du script du hook.

<h3 id="reuse-a-worktree-name">
  Réutiliser un nom de worktree
</h3>

Passer `--worktree` un nom dont le répertoire existe déjà ouvre ce worktree existant au lieu d'en créer un nouveau.

Avec la [base](#choose-the-base-branch) `"fresh"` par défaut, un worktree rouvert se réinitialise à la branche par défaut du dépôt au lieu de continuer à son ancienne pointe quand tous les éléments suivants sont vrais :

* Il n'a pas de modifications non validées ou de fichiers non suivis.
* Il est toujours sur la branche que Claude Code a créée pour lui.
* Il n'a pas de commits propres, ou sa demande de tirage ou de fusion a été fusionnée et sa branche distante supprimée.

Claude Code détecte le cas fusionné à partir de l'état git seul : la branche distante vers laquelle le worktree a poussé n'existe plus, et chaque commit dans le worktree est déjà sur la branche par défaut.

Dans tous les autres cas, Claude Code rouvre le worktree à son ancienne pointe :

* Le worktree échoue l'une des conditions.
* Claude Code ne peut pas vérifier l'état du worktree.
* `worktree.baseRef` est `"head"`.
* Le nom est une référence de demande de tirage ou de fusion.

Avant la v2.1.208, quand vous réutilisiez un nom, Claude Code rouvrait toujours l'ancien worktree à son ancienne pointe.

<h3 id="replace-worktree-creation-with-a-hook">
  Remplacer la création de worktree par un hook
</h3>

Configurez un hook [`WorktreeCreate`](/docs/fr/hooks#worktreecreate) pour remplacer entièrement la logique `git worktree` par défaut, y compris placer les worktrees ailleurs que `.claude/worktrees/`. Pour un exemple complet, voir [Contrôle de version non-git](#non-git-version-control).

<h2 id="what-worktrees-share-with-the-main-checkout">
  Ce que les worktrees partagent avec l'extraction principale
</h2>

Un worktree obtient ses propres fichiers et branche, mais il partage les éléments suivants avec l'extraction principale :

* **Le répertoire `.git` du dépôt** : les commandes git dans un worktree écrivent dans le répertoire `.git` partagé du dépôt principal, et l'[isolation du système de fichiers](/docs/fr/sandboxing#filesystem-isolation) permet ces écritures, donc les commandes telles que `git commit` fonctionnent depuis l'intérieur d'un worktree avec le sandbox activé.
* **Plugins** : les plugins installés à [portée du projet](/docs/fr/plugins/loading#find-where-a-plugin-is-enabled) à partir de l'extraction principale se chargent également dans les worktrees du même dépôt, vous n'avez donc pas besoin de les réinstaller par worktree. Nécessite Claude Code v2.1.200 ou ultérieur.
* **Approbations de permission** : choisir « Oui, et ne plus demander » pour une commande Bash dans une session worktree sauvegarde la règle dans le `.claude/settings.local.json` de l'extraction principale, pour qu'elle s'applique dans l'extraction principale et dans chaque autre worktree du dépôt, et qu'elle survive à la suppression du worktree. Sur Windows et dans les autres cas où Claude Code [n'utilise pas la racine du dépôt](/docs/fr/settings#where-claude-code-looks-for-each-file), la règle reste avec ce worktree. Avant la v2.1.211, une approbation accordée dans un worktree était sauvegardée à l'intérieur de ce worktree, ne s'appliquait pas ailleurs, et était perdue quand le worktree était supprimé. Voir [où les approbations sont sauvegardées](/docs/fr/permissions#permission-system).
* **Compétences, agents et commandes non suivis** : quand l'extraction du worktree n'a pas de répertoire `.claude/skills` à sa racine, par exemple parce que votre `.claude/skills` est gitignored, Claude Code charge les [compétences du projet](/docs/fr/skills#where-skills-live) de l'extraction principale dans la session worktree. Dans un worktree avec son propre répertoire `.claude/skills`, seule cette copie se charge.

  La même lecture couvre `.claude/agents` et `.claude/commands`. Pour les compétences, la lecture nécessite Claude Code v2.1.277 ou ultérieur.

Tous ces éléments s'appliquent que vous créiez le worktree avec `--worktree`, avec `git worktree add`, ou via l'[application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions).

<h2 id="manage-worktrees-manually">
  Gérer les worktrees manuellement
</h2>

Créez des worktrees avec Git directement quand vous avez besoin d'extraire une branche existante spécifique ou de placer le worktree en dehors du dépôt.

Créer un worktree sur une nouvelle branche :

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Créer un worktree à partir d'une branche existante, en remplaçant `fix-issue-456` par une branche qui existe déjà dans votre dépôt :

```bash theme={null}
git worktree add ../project-bugfix fix-issue-456
```

Démarrer Claude dans le worktree :

```bash theme={null}
cd ../project-feature-a
claude
```

Lister vos worktrees :

```bash theme={null}
git worktree list
```

Supprimer un quand vous en avez terminé :

```bash theme={null}
git worktree remove ../project-feature-a
```

Voir la [documentation Git worktree](https://git-scm.com/docs/git-worktree) pour la référence complète des commandes.

<h2 id="non-git-version-control">
  Contrôle de version non-git
</h2>

L'isolation des worktrees utilise git par défaut. Pour SVN, Perforce, Mercurial, ou d'autres systèmes, configurez les hooks [`WorktreeCreate` et `WorktreeRemove`](/docs/fr/hooks#worktreecreate) pour fournir une logique de création et de nettoyage personnalisée. Parce que le hook remplace le comportement git par défaut, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) n'est pas traité quand vous utilisez `--worktree`. Copiez plutôt tous les fichiers de configuration locale à l'intérieur de votre script de hook.

Ce hook `WorktreeCreate` lit le nom du worktree à partir du JSON sur stdin avec `jq`, extrait une copie de travail SVN fraîche, et imprime le chemin du répertoire pour que Claude Code puisse l'utiliser comme répertoire de travail de la session. Ajoutez la configuration à votre [`settings.json`](/docs/fr/settings#where-settings-live) :

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Associez-le à un hook `WorktreeRemove` pour nettoyer quand la session se termine. Voir la [référence des hooks](/docs/fr/hooks#worktreecreate) pour le schéma d'entrée et un exemple de suppression.

Un hook `WorktreeCreate` vous permet également d'exécuter [`/batch`](/docs/fr/commands#all-commands) en dehors d'un dépôt git. Chaque sous-agent `/batch` publie ensuite sa modification avec les commandes de contrôle de version de votre projet et, quand il ne peut pas ouvrir une demande de tirage, signale ce qu'il a publié à la place. L'exécution de `/batch` en dehors d'un dépôt git nécessite Claude Code v2.1.281 ou version ultérieure.

<h2 id="troubleshooting">
  Dépannage
</h2>

Claude Code signale les erreurs ci-dessous quand il crée un worktree, entre dans un au démarrage, ou retourne une session reprise à un.

<h3 id="claude-code-can’t-enter-the-worktree-at-startup">
  Claude Code ne peut pas entrer dans le worktree au démarrage
</h3>

Quand Claude Code ne peut pas entrer dans le répertoire du worktree au démarrage, il imprime une erreur nommant le chemin et se termine avec le code 1. Cela peut se produire quand un hook [`WorktreeCreate`](/docs/fr/hooks#worktreecreate) imprime quelque chose d'autre que le répertoire qu'il a créé, ou quand le répertoire a été supprimé après sa configuration.

<h3 id="worktree-creation-fails-on-a-symlinked-path">
  La création de worktree échoue sur un chemin symlinké
</h3>

Claude Code refuse de créer un worktree quand `.claude`, `.claude/worktrees`, ou le répertoire worktree lui-même est un lien symbolique, et l'erreur nomme le chemin symlinké. Supprimez le lien symbolique et réessayez. Avant la v2.1.212, si le dépôt contenait déjà un lien symbolique validé à l'un de ces chemins, la création de worktree le suivait et pouvait créer des fichiers en dehors du dépôt.

<h3 id="git-lfs-content-is-missing-from-a-worktree-claude-code-created">
  Les fichiers Git LFS sont des fichiers pointeurs dans un worktree que Claude Code a créé
</h3>

Si vous avez configuré [Git LFS](https://git-lfs.com) avec `git lfs install --local`, un worktree que Claude Code crée contient des fichiers pointeurs LFS au lieu des vrais fichiers. Le flag `--local` écrit le filtre LFS dans le `.git/config` du dépôt lui-même plutôt que dans votre config git global. Un simple `git lfs install` écrit dans votre config global et n'est pas affecté. La même chose s'applique à tout autre [pilote de filtre](https://git-scm.com/docs/gitattributes) défini dans le config du dépôt lui-même.

Claude Code ignore les pilotes de filtre du dépôt lui-même quand il crée un worktree parce qu'un pilote de filtre est une commande shell, et n'importe quoi qui peut écrire dans le dépôt, y compris Claude, aurait pu en mettre un là. Avant la v2.1.247, Claude Code exécutait ces pilotes pendant la création de worktree.

Pour obtenir les vrais fichiers, exécutez `git lfs pull` à l'intérieur du worktree.

Dans quatre cas rares, Claude Code crée aucun worktree du tout : il ne peut pas dire quels pilotes de filtre la config du dépôt définit, ou il trouve un paramètre là qu'il ne peut pas désactiver. Faites correspondre l'erreur à sa correction :

* **`Could not read the repository git config to neutralize filter drivers`** : Claude Code n'a pas pu lire le `.git/config` du dépôt, par exemple à cause de ses permissions. Corrigez cela et réessayez.
* **`The repository git config defines a filter driver whose name cannot be neutralized (contains "=" or a newline)`** : renommez ou supprimez ce pilote de filtre dans `.git/config` et réessayez.
* **`The repository git config has a conditional include (includeIf)`** : déplacez les paramètres que l'`includeIf` dans `.git/config` récupère directement dans ce fichier, supprimez l'`includeIf`, et réessayez. Un `includeIf` dans votre config git global ne déclenche pas ceci.
* **`Git was not run: the repository's own git config sets <key>`** : le message nomme une clé qui pointe Git LFS vers un programme à exécuter, comme `lfs.customtransfer.<name>.path` ou `lfs.standalonetransferagent`. Si ce paramètre est le vôtre, déplacez-le vers votre config git global. Si vous ne le reconnaissez pas, supprimez-le du config git du dépôt, puisqu'un outil ou checkout auquel vous ne faites pas confiance peut l'avoir écrit. Réessayez une fois que la clé est partie du config du dépôt.

<h3 id="claude-code-refuses-to-use-a-worktree">
  Claude Code refuse d'utiliser un worktree
</h3>

Une erreur commençant par `Refusing to use <path> as an isolation worktree` signifie que Claude Code a vérifié l'identité git du répertoire avant de l'adopter comme extraction isolée d'une session ou d'un subagent, et l'a refusé. La vérification s'exécute que Claude Code crée le worktree, entre dans un existant, ou réutilise un d'une exécution antérieure.

Dans la plupart des cas, le reste du message dit que les métadonnées git du répertoire se résolvent dans l'extraction principale : par exemple, son fichier `.git` pointe vers le répertoire `.git` du dépôt principal lui-même, ou git résout son arbre de travail à l'extraction principale via une redirection `core.worktree`. À partir d'un tel répertoire, une commande git ordinaire telle que `git reset --hard` agirait sur l'extraction principale au lieu du worktree. Claude Code refuse également quand le répertoire a une entrée `.git` qu'il ne peut pas lire, plutôt que d'assumer que le worktree est sûr.

Un répertoire sans métadonnées git du tout, comme celui que votre hook [`WorktreeCreate`](#non-git-version-control) crée, passe la vérification seulement quand aucun dépôt git ne le contient. Si le hook crée le répertoire à l'intérieur d'un dépôt, git le résout au checkout de ce dépôt et Claude Code le refuse avec le message `git resolves its working tree to`, donc faites en sorte que le hook crée ses répertoires en dehors de tout dépôt.

Claude Code laisse le répertoire refusé en place, puisqu'il peut contenir du travail. Faites correspondre le message à sa récupération, que ce soit après `Refusing to use <path>` ou dans un [message de reprise](#the-session-resumes-outside-its-worktree) ; certaines fins ne se produisent que dans les messages de reprise :

* **Dit `launch from the parent checkout` ou `Run the resume from the project checkout`** : vous avez lancé Claude Code depuis l'intérieur du worktree. Lancez à partir de l'extraction principale à la place ; le worktree n'a pas besoin de recréation.
* **Dit `it cannot be resumed or re-entered`** : rien dans cette session ne justifie le worktree à partir d'où vous avez lancé. Recréez-le ; le répertoire et son travail restent sur le disque pour récupération manuelle, et quand le worktree a une extraction parente, la reprise de là fonctionne aussi.
* **Dit `it contains the protected checkout`** : le répertoire refusé est un parent de votre extraction principale, comme votre répertoire personnel. Ne le supprimez pas. Changez le chemin du worktree, comme le chemin que votre hook `WorktreeCreate` retourne ou la cible `EnterWorktree`, pour que le worktree ne contienne pas l'extraction.
* **Dit `the protected checkout <path> has a .git entry that could not be examined` ou `has git metadata that could not be resolved`** : le problème est les métadonnées git de l'extraction principale, pas celles du worktree. Ne supprimez pas le worktree, et ignorez les conseils de fin du message pour le recréer, qui ne s'appliquent pas à ces deux fins. Réparez l'extraction principale, par exemple un problème de permissions ou un refus git `dubious ownership` sur son `.git`, et réessayez.
* **Dit `its recorded path has a network spelling`** : Claude Code ne reprend jamais dans un worktree à un chemin réseau. Recréez le worktree à un chemin local.
* **Toute autre fin** : le message nomme le problème et sa correction, comme supprimer une redirection `core.worktree` ou recréer le worktree ; suivez-le. Avant de supprimer un répertoire dont le message dit que son identité git n'a pas pu être vérifiée, adressez d'abord la cause nommée, par exemple un lien symbolique dans le chemin du worktree ou git lui-même échouant à s'exécuter, puisque le répertoire peut être sain. Quand vous recréez, sauvegardez tous les changements dont vous avez besoin à partir de l'ancien répertoire d'abord ; il reste sur le disque.

<h3 id="the-session-resumes-outside-its-worktree">
  La session reprend en dehors de son worktree
</h3>

Quand vous reprenez une session de manière interactive et que Claude Code ne peut pas la retourner à son worktree, Claude Code le dit avec l'un des messages ci-dessous. Quand Claude Code efface la liaison du worktree, il enregistre l'effacement dans la transcription de la session. Si vous [supprimez les écritures de transcription](/docs/fr/sessions#where-transcripts-are-stored), le message dit à la place que la liaison n'a pas pu être effacée et que Claude Code revérifier le worktree lors d'une reprise ultérieure.

| Le message commence par                           | Ce qui s'est passé et quoi faire                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Your worktree <path> no longer exists`           | Le répertoire worktree a été supprimé. La session continue dans le répertoire actuel sans isolation, et Claude Code efface la liaison du worktree. Aucune action nécessaire.                                                                                                                                                                                                                                                                                                                                                        |
| `Could not verify your worktree <path> this time` | Claude Code n'a pas pu vérifier le worktree, généralement pour une raison transitoire ; la liaison est conservée, et la session continue dans le répertoire actuel sans isolation. Reprenez à nouveau pour réessayer ; si cela continue de se produire, entrez le worktree dans une nouvelle session et faites correspondre le message de refus sous [Claude Code refuse d'utiliser un worktree](#claude-code-refuses-to-use-a-worktree), qui peut nommer les métadonnées de l'extraction principale plutôt que celles du worktree. |
| `Did not re-enter your worktree <path>`           | Claude Code a refusé la liaison du worktree comme non sûre ; il efface la liaison et la session continue sans isolation. Le message inclut le refus spécifique : faites-le correspondre sous [Claude Code refuse d'utiliser un worktree](#claude-code-refuses-to-use-a-worktree), puisque la correction est la recréation pour certains refus et un changement de chemin pour d'autres.                                                                                                                                             |
| `Could not re-enter your worktree <path>`         | Claude Code ne pouvait pas justifier le worktree à partir d'où vous avez lancé, le plus souvent parce que vous avez lancé depuis l'intérieur ; la liaison est conservée. Le reste du message nomme la correction ; faites-le correspondre sous [Claude Code refuse d'utiliser un worktree](#claude-code-refuses-to-use-a-worktree).                                                                                                                                                                                                 |

En [mode non interactif](/docs/fr/headless) avec `-p`, et sur les reprises que le [SDK Agent](/docs/fr/agent-sdk/sessions) exécute, Claude Code arrête la reprise avec une erreur stderr pour chaque refus sauf un worktree disparu, au lieu de continuer sans isolation.

Avec `--output-format stream-json`, le refus arrive également sur stdout en tant que message `result` avec le sous-type `error_during_execution` dont le tableau `errors` porte le même texte, donc une application SDK Agent reçoit la raison plutôt que seulement une sortie non zéro. Avant la v2.1.260, un refus de reprise de worktree ne produisait aucun message `result`.

Les messages prennent des formes différentes des messages interactifs du tableau :

* `Error: cannot resume into worktree <path>: ...This session was not started.` pour un refus que le tableau montre comme `Did not re-enter`. Claude Code efface la liaison du worktree avant de quitter, et l'erreur le dit ; la prochaine fois que vous reprenez la conversation, la session continue dans le répertoire actuel sans isolation de worktree. Avant la v2.1.260, Claude Code n'écrivait pas la liaison effacée, donc chaque nouvelle tentative de la même reprise échouait avec la même erreur.

  Si vous [supprimez les écritures de transcription](/docs/fr/sessions#where-transcripts-are-stored), l'effacement ne peut pas être sauvegardé. L'erreur dit alors que la même commande sera refusée à nouveau, et nomme `--fork-session` et le démarrage d'une nouvelle conversation comme des façons de continuer sans le worktree.
* `Error: could not verify worktree <path> for this resume, so the resume was aborted...` pour `Could not verify`
* `Error: ...The worktree binding is kept.` pour `Could not re-enter`
* `Notice: the worktree <path> for this session no longer exists...` pour un worktree disparu ; Claude Code l'imprime et continue la session, comme une reprise interactive le fait

La fin du refus intégrée dans chaque erreur est partagée avec les avis interactifs, donc elle correspond toujours à son entrée sous [Claude Code refuse d'utiliser un worktree](#claude-code-refuses-to-use-a-worktree).

Dans le résultat stream-json, [`startup_failure_reason`](/docs/fr/agent-sdk/typescript#startup_failure_reason) est `worktree_unverified` pour l'erreur `could not verify worktree` et `worktree_resume_refused` pour les erreurs `cannot resume into worktree` et `The worktree binding is kept`. Une application peut se brancher dessus au lieu de faire correspondre le texte d'erreur. Avant la v2.1.274, le résultat ne portait aucun champ `startup_failure_reason`.

<h2 id="see-also">
  Voir aussi
</h2>

Les worktrees gèrent l'isolation des fichiers. Les pages connexes ci-dessous couvrent la délégation du travail dans ces extractions isolées, la transmission des résultats entre elles, et le basculement entre les sessions que vous créez :

* [Subagents](/docs/fr/sub-agents) : déléguer le travail à des agents isolés dans une session
* [Messagerie entre sessions](/docs/fr/cross-session-messaging) : laisser les sessions dans vos worktrees se transmettre les résultats
* [Équipes d'agents](/docs/fr/agent-teams) : coordonner plusieurs sessions Claude automatiquement
* [Gérer les sessions](/docs/fr/sessions) : nommer, reprendre et basculer entre les conversations
* [Sessions parallèles de bureau](/docs/fr/desktop#work-in-parallel-with-sessions) : sessions soutenues par worktree dans l'application de bureau
