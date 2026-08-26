> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Exécuter des sessions parallèles avec worktrees

> Isolez les sessions Claude Code parallèles dans des git worktrees séparés pour que les modifications ne se heurtent pas. Couvre le flag `--worktree`, l'isolation des subagents, `.worktreeinclude`, le nettoyage et les hooks VCS non-git.

Un [git worktree](https://git-scm.com/docs/git-worktree) est un répertoire de travail séparé avec ses propres fichiers et branche, partageant le même historique de dépôt et la même télécommande que votre extraction principale. Exécuter chaque session Claude Code dans son propre worktree signifie que les modifications dans une session ne touchent jamais les fichiers d'une autre, vous pouvez donc avoir Claude construisant une fonctionnalité dans un terminal tout en corrigeant un bug dans un second.

Cette page couvre l'isolation des worktrees dans la CLI. Tout ce qui suit suppose un dépôt git. Pour les autres systèmes de contrôle de version, voir [Contrôle de version non-git](#non-git-version-control). L'[application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions) crée automatiquement un worktree pour chaque nouvelle session.

Les worktrees sont l'une des plusieurs façons d'exécuter Claude en parallèle. Ils isolent les modifications de fichiers, tandis que les [subagents](/docs/fr/sub-agents) et les [équipes d'agents](/docs/fr/agent-teams) coordonnent le travail lui-même. Voir [Exécuter les agents en parallèle](/docs/fr/agents) pour comparer les approches, ou passez directement à [Isoler les subagents avec worktrees](#isolate-subagents-with-worktrees) pour utiliser les worktrees et les subagents ensemble.

<h2 id="start-claude-in-a-worktree">
  Démarrer Claude dans un worktree
</h2>

Passez `--worktree` ou `-w` pour créer un worktree isolé et démarrer Claude dedans. Par défaut, le worktree est créé sous `.claude/worktrees/<value>/` à la racine de votre dépôt, sur une nouvelle branche nommée `worktree-<value>` :

```bash theme={null}
claude --worktree feature-auth
```

Pour placer les worktrees ailleurs, configurez un hook [`WorktreeCreate`](#non-git-version-control). Exécutez la commande à nouveau avec un nom différent dans un autre terminal pour démarrer une deuxième session isolée :

```bash theme={null}
claude --worktree bugfix-123
```

Si vous omettez le nom, Claude en génère un tel que `bright-running-fox` :

```bash theme={null}
claude --worktree
```

Vous pouvez également demander à Claude de « travailler dans un worktree » pendant une session, et il en créera un avec l'outil [`EnterWorktree`](/docs/fr/tools-reference). Une fois dans un worktree, Claude peut basculer directement vers un autre sous `.claude/worktrees/` en appelant `EnterWorktree` avec le chemin cible. Le worktree précédent reste sur le disque intact.

L'entrée dans un chemin en dehors du répertoire `.claude/worktrees/` du dépôt demande d'abord votre approbation, car elle déplace le répertoire de travail de la session, l'accès en écriture et la configuration du projet telle que `CLAUDE.md` et les paramètres vers cet emplacement. Une règle de permission [`EnterWorktree`](/docs/fr/permissions) ou le choix de « ne plus demander » ne supprime pas cette invite ; seul le mode `bypassPermissions` l'ignore. Avant la v2.1.206, Claude pouvait entrer dans n'importe quel chemin de worktree existant sans demander.

À partir de la v2.1.198, l'entrée ou la sortie d'un worktree déplace également la transcription de session vers le stockage de projet de ce répertoire, de la même manière que [`/cd`](/docs/fr/commands) le fait, donc `/desktop` et `--resume` trouvent la session là-bas par la suite. Les worktrees créés par un hook [`WorktreeCreate`](#non-git-version-control) sont exclus et conservent la transcription au répertoire de lancement.

Les worktrees fonctionnent avec [l'isolation du système de fichiers](/docs/fr/sandboxing#filesystem-isolation) activée : le sandbox permet les écritures dans le répertoire `.git` partagé du dépôt principal afin que des commandes telles que `git commit` puissent mettre à jour les refs et l'index depuis l'intérieur d'un worktree lié.

Avant d'utiliser `--worktree` de manière interactive dans un répertoire pour la première fois, acceptez la boîte de dialogue de confiance de l'espace de travail en exécutant `claude` une fois dans ce répertoire. Si la confiance n'a pas encore été acceptée, `--worktree` se termine avec une erreur et vous invite à exécuter `claude` dans le répertoire en premier. Les exécutions non interactives avec `-p` ignorent la [vérification de confiance](/docs/fr/security), donc `claude -p --worktree` procède sans elle.

Si Claude Code ne peut pas entrer dans le répertoire du worktree au démarrage, par exemple parce qu'un hook [`WorktreeCreate`](/docs/fr/hooks#worktreecreate) a affiché quelque chose d'autre que le répertoire qu'il a créé, ou parce que le répertoire a été supprimé après sa configuration, Claude Code affiche une erreur nommant le chemin et se termine avec le code 1. Avant la v2.1.205, cela plantait la session, et avec `-p` cela s'arrêtait pendant environ 30 secondes avant de se terminer avec le code 0.

Les plugins installés à [portée du projet](/docs/fr/plugins-reference#plugin-installation-scopes) à partir de l'extraction principale se chargent également dans les worktrees du même dépôt, vous n'avez donc pas besoin de les réinstaller par worktree. Cela s'applique que vous créiez le worktree avec `--worktree` ou avec `git worktree add`. Nécessite Claude Code v2.1.200 ou ultérieur.

<Tip>
  Ajoutez `.claude/worktrees/` à votre `.gitignore` pour que le contenu des worktrees n'apparaisse pas comme des fichiers non suivis dans votre extraction principale.
</Tip>

<h3 id="choose-the-base-branch">
  Choisir la branche de base
</h3>

Les worktrees se ramifient à partir de la branche par défaut de votre dépôt, `origin/HEAD`, ils commencent donc à partir d'un arbre propre correspondant à la télécommande. Quand aucune télécommande n'a récupéré le dépôt au cours des 24 dernières heures, Claude Code actualise `origin/HEAD` avec une récupération de la branche par défaut, limitée à cinq secondes, et utilise la ref mise en cache localement si la récupération échoue. Si aucune télécommande n'est configurée, ou si `origin/HEAD` n'est pas mis en cache localement et ne peut pas être récupéré, le worktree revient à votre `HEAD` local actuel.

L'actualisation nécessite Claude Code v2.1.208 ou ultérieur ; avant cela, un worktree frais utilisait tout ce qui était déjà mis en cache localement pour `origin/HEAD`.

Pour toujours vous ramifier à partir du `HEAD` local à la place, définissez `worktree.baseRef` sur `"head"` dans [paramètres](/docs/fr/settings#worktree-settings). Définir `baseRef` sur `"head"` fait que les nouveaux worktrees portent vos commits non poussés et l'état de la branche de fonctionnalité, ce qui est utile lors de l'isolation des subagents qui doivent opérer sur un travail en cours. Quand la session s'exécute à l'intérieur d'un worktree lié, `"head"` se résout en `HEAD` de ce worktree, pas en celui de l'extraction principale. Le paramètre n'accepte que `"fresh"` ou `"head"`, pas des refs git arbitraires :

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Pour vous ramifier à partir d'une demande de tirage spécifique, passez le numéro de PR préfixé par `#`, ou une URL complète de demande de tirage GitHub. Claude Code récupère `pull/<number>/head` à partir de `origin` et crée le worktree à `.claude/worktrees/pr-<number>` :

```bash theme={null}
claude --worktree "#1234"
```

Pour un contrôle total sur la façon dont les worktrees sont créés, configurez un hook [`WorktreeCreate`](/docs/fr/hooks#worktreecreate), qui remplace entièrement la logique `git worktree` par défaut.

<h3 id="reuse-a-worktree-name">
  Réutiliser un nom de worktree
</h3>

Réutiliser un nom de worktree dont le répertoire existe déjà reprend ce worktree.

Un worktree repris se réinitialise à la [base actuelle](#choose-the-base-branch) au lieu de reprendre à son ancienne pointe quand tous les éléments suivants sont vrais :

* Il n'a pas de modifications non validées ou de fichiers non suivis.
* Il est toujours sur la branche que Claude Code a créée pour lui.
* Il n'a jamais validé, ou sa demande de tirage a été fusionnée et sa branche distante supprimée.

Avant la v2.1.208, un nom réutilisé reprenait toujours l'ancien worktree à son ancienne pointe.

<h2 id="copy-gitignored-files-into-worktrees">
  Copier les fichiers gitignorés dans les worktrees
</h2>

Un worktree est une extraction fraîche, donc les fichiers non suivis comme `.env` ou `.env.local` de votre dépôt principal ne sont pas présents. Pour les copier automatiquement quand Claude crée un worktree, ajoutez un fichier `.worktreeinclude` à la racine de votre projet.

Le fichier utilise la syntaxe `.gitignore`. Seuls les fichiers qui correspondent à un motif et sont également gitignorés sont copiés, donc les fichiers suivis ne sont jamais dupliqués.

Ce `.worktreeinclude` copie deux fichiers env et une configuration de secrets dans chaque nouveau worktree :

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Cela s'applique aux worktrees créés avec `--worktree`, [worktrees de subagents](#isolate-subagents-with-worktrees), et aux sessions parallèles dans l'[application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions).

<h2 id="isolate-subagents-with-worktrees">
  Isoler les subagents avec worktrees
</h2>

Les subagents peuvent s'exécuter dans leurs propres worktrees pour que les modifications parallèles ne se heurtent pas. Demandez à Claude d'« utiliser les worktrees pour vos agents », ou définissez-le de manière permanente sur un [subagent personnalisé](/docs/fr/sub-agents#supported-frontmatter-fields) en ajoutant `isolation: worktree` au frontmatter. Chaque subagent obtient un worktree temporaire qui est supprimé automatiquement quand le subagent se termine sans modifications.

Les worktrees des subagents utilisent la même [branche de base](#choose-the-base-branch) que `--worktree`, donc ils se ramifient à partir de la branche par défaut de votre référentiel sauf si `worktree.baseRef` est défini sur `"head"`.

<h2 id="clean-up-worktrees">
  Nettoyer les worktrees
</h2>

Quand vous quittez une session worktree, le nettoyage dépend de si vous avez apporté des modifications :

* **Pas de modifications non validées, pas de fichiers non suivis et pas de nouveaux commits** : le worktree et sa branche sont supprimés automatiquement. Si la session a un [nom](/docs/fr/sessions#name-your-sessions), Claude vous demande plutôt de conserver le worktree pour plus tard
* **Des modifications non validées, des fichiers non suivis ou de nouveaux commits existent** : Claude vous demande de conserver ou de supprimer le worktree. Conserver préserve le répertoire et la branche pour que vous puissiez revenir plus tard. Supprimer supprime le répertoire worktree et sa branche, en supprimant toutes les modifications non validées, les fichiers non suivis et les commits
* **Exécutions non interactives** : les worktrees créés avec `--worktree` aux côtés de `-p` ne sont pas nettoyés automatiquement puisqu'il n'y a pas d'invite de sortie. Supprimez-les avec `git worktree remove`

Les worktrees que Claude a créés pour les subagents et les [sessions en arrière-plan](/docs/fr/agent-view#how-file-edits-are-isolated) sont supprimés automatiquement une fois qu'ils sont plus anciens que votre paramètre [`cleanupPeriodDays`](/docs/fr/settings#available-settings), à condition qu'ils n'aient pas de modifications non validées, pas de fichiers non suivis et pas de commits non poussés. Les worktrees que vous créez avec `--worktree` ne sont jamais supprimés par ce balayage.

Pendant qu'un agent s'exécute, Claude exécute `git worktree lock` sur son worktree pour que le nettoyage concurrent ne puisse pas le supprimer. Le verrou est libéré quand l'agent se termine. Pour nettoyer un worktree que le balayage conserve, exécutez `git worktree remove`, en ajoutant `--force` si le worktree a des modifications non validées ou des fichiers non suivis.

Sur Windows, avant de supprimer un worktree, Claude Code supprime toute jonction NTFS ou lien symbolique de répertoire à n'importe quelle profondeur à l'intérieur comme entrée de lien, afin que la suppression du worktree ne supprime pas les fichiers vers lesquels un lien pointe. Avant la v2.1.205, Claude Code supprimait uniquement les liens de niveau supérieur comme entrées de lien, et la suppression d'un worktree avec une jonction imbriquée dans un sous-répertoire pouvait supprimer le contenu du répertoire vers lequel le lien pointait en dehors du worktree.

<h2 id="manage-worktrees-manually">
  Gérer les worktrees manuellement
</h2>

Pour un contrôle total sur l'emplacement du worktree et la configuration de la branche, créez des worktrees directement avec Git. Ceci est utile quand vous devez extraire une branche existante spécifique ou placer le worktree en dehors du dépôt.

Créer un worktree sur une nouvelle branche :

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Créer un worktree à partir d'une branche existante :

```bash theme={null}
git worktree add ../project-bugfix bugfix-123
```

Démarrer Claude dans le worktree :

```bash theme={null}
cd ../project-feature-a && claude
```

Lister vos worktrees :

```bash theme={null}
git worktree list
```

Supprimer un quand vous en avez terminé :

```bash theme={null}
git worktree remove ../project-feature-a
```

Voir la [documentation Git worktree](https://git-scm.com/docs/git-worktree) pour la référence complète des commandes. N'oubliez pas d'initialiser votre environnement de développement dans chaque nouveau worktree : installez les dépendances, configurez les environnements virtuels, ou exécutez tout ce que votre configuration de projet nécessite.

<h2 id="non-git-version-control">
  Contrôle de version non-git
</h2>

L'isolation des worktrees utilise git par défaut. Pour SVN, Perforce, Mercurial ou d'autres systèmes, configurez les hooks [`WorktreeCreate` et `WorktreeRemove`](/docs/fr/hooks#worktreecreate) pour fournir une logique de création et de nettoyage personnalisée. Parce que le hook remplace le comportement git par défaut, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) n'est pas traité quand vous utilisez `--worktree`. Copiez plutôt tous les fichiers de configuration locale à l'intérieur de votre script de hook.

Ce hook `WorktreeCreate` lit le nom du worktree à partir de stdin, extrait une copie de travail SVN fraîche, et imprime le chemin du répertoire pour que Claude Code puisse l'utiliser comme répertoire de travail de la session :

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

<h2 id="see-also">
  Voir aussi
</h2>

Les worktrees gèrent l'isolation des fichiers. Les pages connexes ci-dessous couvrent la délégation du travail dans ces extractions isolées et le basculement entre les sessions que vous créez :

* [Subagents](/docs/fr/sub-agents) : déléguer le travail à des agents isolés dans une session
* [Équipes d'agents](/docs/fr/agent-teams) : coordonner plusieurs sessions Claude automatiquement
* [Gérer les sessions](/docs/fr/sessions) : nommer, reprendre et basculer entre les conversations
* [Sessions parallèles de bureau](/docs/fr/desktop#work-in-parallel-with-sessions) : sessions soutenues par worktree dans l'application de bureau
