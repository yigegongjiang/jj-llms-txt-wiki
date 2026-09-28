> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Héberger et maintenir une marketplace

> Publiez une marketplace de plugins où les utilisateurs peuvent y accéder, accordez l'accès à une marketplace privée, et publiez des mises à jour et des renommages sans casser les installations.

Héberger une marketplace signifie mettre votre catalogue `marketplace.json` où d'autres personnes peuvent l'ajouter avec `/plugin marketplace add`, installer ses plugins, et continuer à recevoir vos modifications après que vous les ayez publiées.

Cette page est destinée à la personne qui exploite une marketplace.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Vous n'avez pas encore écrit le fichier de catalogue** : commencez par [Créer une marketplace](/docs/fr/plugins/create-marketplace)
  * **Vous êtes un administrateur qui exige, restreint ou pré-installe des marketplaces sur les machines de votre organisation** : lisez [Gérer les plugins pour votre organisation](/docs/fr/plugins/org)
</Note>

Commencez par [Héberger votre marketplace](#host-your-marketplace) pour choisir un hôte et la commande que vos utilisateurs exécutent. Lisez [Tenir les utilisateurs à jour](#keep-users-up-to-date) avant votre première version. Lisez [Renommer ou supprimer un plugin](#rename-or-remove-a-plugin) avant de modifier le `name` d'un plugin.

<h2 id="host-your-marketplace">
  Héberger votre marketplace
</h2>

Vous pouvez héberger la marketplace sur GitHub, sur un autre hôte git, en tant qu'URL `marketplace.json` hébergée, ou dans un répertoire sur un système de fichiers partagé. Envoyez à vos utilisateurs la commande add pour votre hôte et dites-leur ce dont ils ont besoin sur leur machine :

| Hôte                                                              | Les utilisateurs exécutent, dans une session Claude Code               | Ce dont les utilisateurs ont besoin                                                                                                         |
| :---------------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| GitHub                                                            | `/plugin marketplace add your-org/your-marketplace`                    | `git`, et pour un référentiel privé l'accès décrit sous [Accorder l'accès à une marketplace privée](#grant-access-to-a-private-marketplace) |
| GitLab, Bitbucket, GitHub Enterprise Server, ou un autre hôte git | `/plugin marketplace add https://gitlab.example.com/team/plugins.git`  | `git`, et accès à l'hôte depuis leur machine. Envoyez l'URL complète, car le raccourci `owner/repo` signifie toujours github.com            |
| Une URL `marketplace.json` hébergée                               | `/plugin marketplace add https://plugins.example.com/marketplace.json` | Accès HTTPS à l'URL. Les utilisateurs n'ont pas besoin de `git` pour le catalogue lui-même                                                  |
| Un répertoire sur un système de fichiers partagé                  | `/plugin marketplace add /Volumes/shared/claude-plugins`               | Accès en lecture au chemin                                                                                                                  |

Pour épingler une branche ou une étiquette d'une marketplace GitHub ou git-URL, dites aux utilisateurs d'ajouter `#<ref>`, comme dans `your-org/your-marketplace#stable`. La [référence des commandes de plugin](/docs/fr/plugins/cli-reference#plugin-marketplace-add) énumère chaque forme que la commande accepte.

Un ajout réussi affiche `Successfully added marketplace: your-marketplace`. Claude Code prend ce nom du champ `name` dans votre `marketplace.json`, pas du nom du référentiel.

Les utilisateurs installent ensuite un plugin par le `name` de son entrée et le `name` de la marketplace, comme dans `/plugin install code-formatter@your-marketplace`.

<h3 id="register-the-marketplace-for-everyone-in-a-repository">
  Enregistrer la marketplace pour tout le monde dans un référentiel
</h3>

Pour partager la marketplace avec tous ceux qui travaillent dans un référentiel, exécutez `claude plugin marketplace add your-org/your-marketplace --scope project` là une fois depuis votre shell et validez le `.claude/settings.json` qu'il écrit. Claude Code enregistre ensuite la marketplace pour chaque coéquipier qui [fait confiance au dossier](/docs/fr/plugins/org#require-plugins-per-repository).

<h3 id="avoid-relative-path-entries-in-a-url-hosted-marketplace">
  Éviter les entrées de chemin relatif dans une marketplace hébergée sur URL
</h3>

Quand les utilisateurs ajoutent votre marketplace en tant qu'URL `marketplace.json` simple, Claude Code télécharge uniquement ce fichier. Une entrée dans votre tableau `plugins` dont la `source` est un chemin relatif tel que `./plugins/formatter` échoue ensuite à l'installation avec [`its marketplace entry path does not stay inside the marketplace directory`](/docs/fr/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces). Donnez à chaque entrée une source qui peut être récupérée seule, comme un référentiel `github` ou une URL `archive`, ou hébergez la marketplace dans un référentiel git pour que Claude Code clone l'arborescence entière.

<h3 id="edit-plugins-in-place-on-a-shared-directory">
  Éditer les plugins sur place dans un répertoire partagé
</h3>

Quand les utilisateurs ajoutent votre marketplace à partir d'un répertoire partagé, Claude Code lit les plugins avec des sources de chemin relatif directement depuis ce répertoire au lieu de les copier. Les utilisateurs voient vos modifications quand ils démarrent la prochaine session ou exécutent `/reload-plugins`, sans étape de mise à jour ou augmentation de version.

<h3 id="keep-plugin-files-out-of-git-lfs">
  Garder les fichiers de plugin hors de Git LFS
</h3>

Gardez les fichiers dont vos plugins ont besoin hors de [Git LFS](https://git-lfs.com). Quand les utilisateurs ajoutent une marketplace hébergée dans un référentiel git, ou installent un plugin basé sur git qu'elle énumère, Claude Code clone ce référentiel de marketplace ou de plugin sur leur machine. Le clone ne télécharge jamais le contenu LFS, donc les fichiers suivis par LFS arrivent en tant que fichiers pointeurs.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Partager les fichiers au sein d'une marketplace avec des liens symboliques
</h3>

Pour partager les fichiers entre votre plugin et d'autres parties de la même marketplace, créez des liens symboliques à l'intérieur de votre répertoire de plugin. Quand Claude Code copie le plugin dans son cache, il gère chaque lien symbolique par où la cible se résout :

* **Au sein du répertoire du plugin lui-même** : le lien symbolique est préservé en tant que lien symbolique relatif dans le cache, donc il continue à se résoudre à la cible copiée au moment de l'exécution.
* **Ailleurs au sein de la même marketplace** : le lien symbolique est déréférencé. Le contenu de la cible est copié dans le cache à sa place. Cela permet au répertoire `skills/` d'un meta-plugin de se lier aux compétences définies par d'autres plugins dans la marketplace.
* **En dehors de la marketplace** : le lien symbolique est ignoré pour des raisons de sécurité.

Pour les plugins installés à partir d'un chemin local, ou à partir d'une [`command` source](/docs/fr/plugins/marketplace-reference#command-plugin-source) dont le `mode` est le `copy` par défaut, Claude Code préserve uniquement les liens symboliques qui se résolvent au sein du répertoire du plugin lui-même et ignore tous les autres.

La commande suivante crée un lien depuis l'intérieur d'un plugin de marketplace vers une compétence partagée définie par un plugin frère. Sur Windows, utilisez `mklink /D` depuis une invite de commande élevée ou activez le mode développeur :

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

<h2 id="distribute-through-organization-settings">
  Distribuer via les paramètres de l'organisation
</h2>

Sur un plan Team ou Enterprise, vous pouvez également distribuer la marketplace via [**Paramètres de l'organisation > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) sur claude.ai au lieu de l'héberger quelque part où les utilisateurs l'ajoutent eux-mêmes. La synchronisation de l'organisation lit le référentiel via la connexion GitHub ou GitLab de votre organisation sur claude.ai, donc les identifiants git de vos utilisateurs ne sont pas impliqués.

La synchronisation de l'organisation est plus stricte sur le référentiel que `/plugin marketplace add` ne l'est :

* **Référentiel de marketplace** : sur github.com et gitlab.com, il doit être privé ou interne
* **Sources de plugin** : chaque source de plugin doit être de type `github`, `url`, ou `git-subdir`, ou un [chemin relatif](/docs/fr/plugins/marketplace-reference#relative-path-plugin-source) qui commence par `./`
* **Répertoire `bin/` de niveau supérieur** : claude.ai rejette un plugin qui en a un et synchronise le reste de la marketplace. Le message d'erreur commence par `Plugin contains a top-level bin/ directory`. Gardez les exécutables dans un autre répertoire, comme `scripts/`, et référencez-les comme `${CLAUDE_PLUGIN_ROOT}/scripts/<name>` depuis vos hooks ou configurations de serveur MCP

Voir [Gérer les plugins pour votre organisation](https://support.claude.com/en/articles/13837433) pour le flux de travail administrateur.

<h2 id="grant-access-to-a-private-marketplace">
  Accorder l'accès à une marketplace privée
</h2>

Quand un utilisateur ajoute, installe à partir de, ou met à jour votre marketplace, Claude Code exécute `git` sur sa machine avec les invites interactives désactivées et s'appuie sur les identifiants que cette machine détient déjà. Claude Code n'a pas de jeton git qui lui est propre, et `marketplace.json` n'a pas de champ pour en avoir un.

Vous choisissez si le clone s'exécute sur SSH ou HTTPS par la forme de la commande add que vous envoyez aux utilisateurs :

* **GitHub `owner/repo`** : Claude Code sonde `ssh -T git@github.com` et clone sur SSH quand la sonde réussit. Si la sonde échoue, ou le clone SSH lui-même échoue, il clone sur HTTPS. Les utilisateurs sur des machines sans clé SSH GitHub peuvent définir `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` pour ignorer la sonde et cloner sur HTTPS.
* **`git@host:path.git`** : SSH.
* **`https://example.com/repo.git`** : HTTPS.

Dites aux utilisateurs ce que chaque protocole a besoin sur leur machine :

* **SSH** : la clé doit fonctionner sans invite de phrase secrète, par exemple parce qu'elle est chargée dans `ssh-agent`. L'hôte doit déjà être dans `known_hosts`.
* **HTTPS** : Claude Code laisse l'assistant d'identifiants git de l'utilisateur activé mais lui interdit de demander. Un identifiant que l'assistant stocke déjà fonctionne ; un qu'il devrait demander échoue. Sur GitHub, `gh auth login` suivi de `gh auth setup-git` en stocke un.

Pour un hôte GitHub Enterprise Server, les utilisateurs ont besoin d'accès git à cet hôte depuis leur machine. Voir [Plugin marketplaces on GHES](/docs/fr/github-enterprise-server#plugin-marketplaces-on-ghes) pour ce que chaque surface Claude Code a besoin d'atteindre une marketplace hébergée sur GHES.

Si vous distribuez via **Paramètres de l'organisation > Plugins & skills** sur claude.ai à la place, les identifiants git de vos utilisateurs ne sont pas impliqués. Voir [Distribuer via les paramètres de l'organisation](#distribute-through-organization-settings) pour quelles sources de plugin peuvent être privées là.

<h3 id="serve-users-who-have-no-git-host-account">
  Servir les utilisateurs qui n'ont pas de compte sur l'hôte git
</h3>

Les utilisateurs sans compte sur l'hôte git peuvent ajouter une marketplace que vous servez en tant qu'URL `marketplace.json` ou à partir d'un répertoire partagé, mais ils ne peuvent installer que les plugins dont les sources d'entrée ils peuvent aussi atteindre. Une entrée qui pointe vers un référentiel `github` privé échoue toujours à l'installation pour eux, parce que Claude Code la récupère avec le même `git` non-interactif qu'il utilise pour une marketplace hébergée sur git.

Ces sources d'entrée n'ont pas besoin de compte git :

* **`archive`** : un zip téléchargé sur HTTPS. Les utilisateurs n'ont besoin ni de `git` ni de compte, seulement d'accès réseau à l'URL. Nécessite Claude Code v2.1.224 ou ultérieur. Épinglez chaque archive avec `sha256` pour que Claude Code refuse un téléchargement modifié. Pour envoyer des identifiants avec le téléchargement, voir [Authentifier les téléchargements d'archive](#authenticate-archive-downloads).
* **Un référentiel git public** : Claude Code clone une source `url` ou `git-subdir` publique sur HTTPS sans identifiants quand l'entrée donne une URL `https://`. Pour une source `github`, ou une source `git-subdir` écrite comme `owner/repo`, les utilisateurs sans clé SSH GitHub définissent `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`.

Pour une équipe sur un réseau, une marketplace `directory` sur un système de fichiers partagé fonctionne aussi sans comptes git. Les utilisateurs ont besoin seulement d'accès en lecture au chemin.

<h3 id="what-background-auto-update-does-with-credentials">
  Ce que la mise à jour automatique en arrière-plan fait avec les identifiants
</h3>

La mise à jour automatique en arrière-plan est l'actualisation sans surveillance de Claude Code des marketplaces et des plugins installés après le démarrage d'une session. Elle est désactivée pour votre marketplace jusqu'à ce qu'un utilisateur ou un administrateur l'active, comme couvert sous [Tenir les utilisateurs à jour](#keep-users-up-to-date).

Quand elle est activée pour une marketplace privée, la vérification en arrière-plan des nouveaux commits utilise les assistants d'identifiants git configurés de l'utilisateur et ne demande jamais. Chaque type de distant et d'assistant donne un résultat différent :

* **Distants SSH** : une clé chargée dans `ssh-agent` authentifie la vérification.
* **Distants HTTPS avec un identifiant stocké** : un assistant qui peut fournir un identifiant stocké sans demander authentifie la vérification. Git Credential Manager, l'assistant Keychain macOS, et `git-credential-store` fonctionnent de cette façon une fois qu'ils détiennent un identifiant pour l'hôte.
* **Distants HTTPS avec un assistant qui a besoin de demander** : l'assistant ne peut pas répondre en arrière-plan. La mise à jour échoue silencieusement et le checkout existant reste en place, donc les plugins de l'utilisateur continuent à fonctionner à partir du dernier état synchronisé.

Après la vérification, Claude Code fait l'une des choses suivantes :

* **Le checkout est à jour** : Claude Code le laisse tel quel.
* **La vérification trouve de nouveaux commits, ou échoue parce qu'elle ne peut pas atteindre ou s'authentifier au distant** : Claude Code clone la marketplace à nouveau et remplace le checkout existant par le nouveau clone. Si ce clone échoue, le checkout existant reste en place. Le re-clone peut [expirer sur les grands référentiels](/docs/fr/plugins/troubleshooting#git-clone-timed-out-after-120s).

Pour garder une marketplace privée à jour, un utilisateur peut faire l'une des choses suivantes :

* **Stocker un identifiant** : se connecter d'abord à l'assistant d'identifiants pour qu'il détienne un identifiant pour l'hôte. Pour GitHub, exécutez `gh auth login`, puis `gh auth setup-git`.
* **Garder le checkout en cas d'échec** : si l'utilisateur définit `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`, Claude Code garde le checkout existant sans tenter le re-clone quand la vérification en arrière-plan ne peut pas atteindre ou s'authentifier au distant. Les plugins continuent à fonctionner à partir du dernier état synchronisé.

Si un utilisateur définit `GITHUB_TOKEN` ou un autre jeton de fournisseur dans l'environnement, cela seul n'authentifie pas la vérification en arrière-plan. Un jeton prend effet via un assistant d'identifiants, comme l'assistant de l'interface de ligne de commande `gh`, qui lit `GH_TOKEN` et `GITHUB_TOKEN`.

<h2 id="roll-out-to-a-whole-company">
  Déployer dans toute une entreprise
</h2>

Déployer un plugin dans une entreprise implique vous en tant que propriétaire de la marketplace, un administrateur qui contrôle les paramètres gérés, et chaque personne qui utilise Claude Code. Vous pouvez exécuter le déploiement sans l'administrateur, auquel cas chaque personne ajoute la marketplace et installe le plugin elle-même.

| Qui                                     | Ce qu'ils font                                                                                                                                                                  | Où c'est couvert                                                                                                                                                           |
| :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Vous, le propriétaire de la marketplace | Gardez le catalogue dans un référentiel que seule l'entreprise peut lire, envoyez la commande add pour votre hôte, et dites ce que chaque personne a besoin sur sa machine      | [Héberger votre marketplace](#host-your-marketplace) et [Accorder l'accès à une marketplace privée](#grant-access-to-a-private-marketplace)                                |
| Un administrateur                       | Enregistre la marketplace et active ses plugins pour tout le monde avec `extraKnownMarketplaces` et `enabledPlugins` dans les paramètres gérés, et définit `autoUpdate` là      | [Exiger une marketplace et ses plugins](/docs/fr/plugins/org#require-a-marketplace-and-its-plugins) et [Définir la politique de mise à jour](/docs/fr/plugins/org#set-update-policy) |
| Chaque personne                         | A besoin d'accès en lecture à un référentiel git privé, avec des identifiants déjà stockés sur sa machine. Sans administrateur, elle exécute aussi les commandes add et install | [Ajouter une marketplace privée](/docs/fr/plugins/install#add-a-private-marketplace)                                                                                            |

Pour les personnes qui n'ont pas de compte sur l'hôte git, ces sections couvrent chacune une façon de les atteindre :

* **Sources d'entrée qui n'ont pas besoin de compte git** : [Servir les utilisateurs qui n'ont pas de compte sur l'hôte git](#serve-users-who-have-no-git-host-account)
* **Un répertoire de plugins pré-rempli** : [Ensemencer les conteneurs et CI](/docs/fr/plugins/org#seed-containers-and-ci), qui sert aussi les utilisateurs qui n'ont pas de compte sur l'hôte git
* **Paramètres de l'organisation claude.ai** : [Distribuer via les paramètres de l'organisation](#distribute-through-organization-settings), où les identifiants git de vos utilisateurs ne sont pas impliqués

<h2 id="keep-users-up-to-date">
  Tenir les utilisateurs à jour
</h2>

Vos modifications atteignent les utilisateurs via la mise à jour automatique en arrière-plan, une fois qu'elle est activée pour votre marketplace, ou quand les utilisateurs mettent à jour le plugin eux-mêmes. Dans les deux cas, un utilisateur obtient une nouvelle copie d'un plugin uniquement quand sa version calculée change, comme décrit sous [Publier une nouvelle version](#release-a-new-version).

<h3 id="turn-on-auto-update">
  Activer la mise à jour automatique
</h3>

La mise à jour automatique en arrière-plan est désactivée pour votre marketplace par défaut, et `marketplace.json` n'a pas de champ pour l'activer. Un utilisateur ou un administrateur l'active :

* **Dites aux utilisateurs de l'activer** : chaque utilisateur va à **Marketplaces** dans `/plugin`, sélectionne votre marketplace, et sélectionne **Enable auto-update**.
* **Demandez à un administrateur de la définir** : si un administrateur définit `"autoUpdate": true` sur l'entrée `extraKnownMarketplaces` de votre marketplace dans les paramètres gérés, elle est activée pour tout le monde qui reçoit ces paramètres. Voir [Définir la politique de mise à jour](/docs/fr/plugins/org#set-update-policy).

Sans mise à jour automatique, les utilisateurs reçoivent vos modifications quand ils exécutent `/plugin marketplace update <name>` dans une session ou `claude plugin update <plugin>@<name>` dans le shell.

Pour ce que les utilisateurs voient quand une mise à jour les atteint, voir [Quand la mise à jour automatique s'exécute](/docs/fr/plugins/loading#when-auto-update-runs).

<h3 id="release-a-new-version">
  Publier une nouvelle version
</h3>

Pour publier une nouvelle version aux utilisateurs, modifiez le `version` du plugin. Les utilisateurs obtiennent une nouvelle copie uniquement quand la version calculée du plugin diffère de celle qu'ils ont. Cette version provient de `plugin.json` d'abord, puis de l'entrée de marketplace, selon [Versions et mises à jour](/docs/fr/plugins/loading#versions-and-updates).

Un plugin que les utilisateurs [chargent sur place](/docs/fr/plugins/loading#find-plugins-on-disk) à partir d'une marketplace qu'ils ont ajoutée en tant que répertoire local n'est pas contrôlé par `version`. Il charge vos fichiers actuels à chaque démarrage de session, peu importe ce que sa chaîne de version dit.

Pour chaque installation autre qu'un chargement sur place ou un à partir d'une source `command`, soit augmentez `version` à chaque version, soit omettez-la :

* **Augmentez `version` à chaque version** : les utilisateurs restent sur leur copie en cache jusqu'à ce que la chaîne change. Si vous définissez `"version": "1.0.0"` et poussez de nouveaux commits sans le modifier, les utilisateurs ne les reçoivent pas.
* **Omettez `version`** : les utilisateurs suivent vos commits à la place. Laissez `version` hors de `plugin.json` et de l'entrée de marketplace.

Ne définissez pas `version` dans `plugin.json` et l'entrée de marketplace. Si vous le faites, Claude Code utilise la valeur `plugin.json` sans avertissement, et `claude plugin validate` signale l'inadéquation comme `Entry declares version "<a>" but <path>/plugin.json says "<b>"`.

<h3 id="hold-users-on-one-version">
  Garder les utilisateurs sur une version
</h3>

Une marketplace sert une version de chaque plugin à la fois, donc vous gardez les utilisateurs sur une version en choisissant ce que chaque entrée pointe :

* **`ref` et `sha` sur l'entrée du plugin** : `ref` nomme une branche ou une étiquette et `sha` nomme un commit pour une source `github`, `url`, ou `git-subdir`. Voir [Sources de plugin](/docs/fr/plugins/marketplace-reference#plugin-sources).
* **`#<ref>` sur la commande add** : les utilisateurs qui ajoutent `your-org/your-marketplace#stable` obtiennent cette branche ou étiquette du catalogue. Pour deux lignes de version à la fois, voir [Exécuter les canaux de version](#run-release-channels).
* **Étiquettes `<plugin>--v<version>`** : la plage de version d'une dépendance se résout par rapport à ces étiquettes. Voir [Publier un plugin sur lequel d'autres dépendent](/docs/fr/plugins/dependencies#tag-plugin-releases-for-version-resolution).

[Publier une nouvelle version](#release-a-new-version) dit quand une entrée modifiée atteint les utilisateurs.

<h3 id="change-the-command-of-a-command-source">
  Modifier la commande d'une source de commande
</h3>

Si vous modifiez la `command` d'une [`command` source](/docs/fr/plugins/marketplace-reference#command-plugin-source), ou changez son `mode`, chaque utilisateur doit accepter la nouvelle commande avant que Claude Code ne l'exécute. Claude Code exécute uniquement la commande exacte qu'un utilisateur a acceptée quand il a installé ou mis à jour le plugin pour la dernière fois.

Après que la copie de votre marketplace d'un utilisateur récupère la modification, cet utilisateur voit l'un des éléments suivants :

* **Pas plus d'exécutions en arrière-plan** : l'[exécution une fois par session](/docs/fr/plugins/loading#when-a-command-source-re-runs) de la commande s'arrête pour cet utilisateur, donc la nouvelle sortie de l'outil ne les atteint pas.
* **Une entrée dans l'onglet Erreurs `/plugin`** : l'entrée affiche la nouvelle commande et la commande `claude plugin update` à exécuter.

Dites aux utilisateurs d'exécuter la commande `claude plugin update` que cette entrée affiche, dans un terminal. Claude Code leur affiche la nouvelle commande et leur demande de l'accepter.

<h2 id="run-release-channels">
  Exécuter les canaux de version
</h2>

Pour offrir des pistes stables et d'accès anticipé, hébergez deux marketplaces dont les entrées pointent vers différentes refs du même plugin, et laissez chaque utilisateur ajouter celle qu'il veut. Claude Code n'a pas de concept de canal de version, et une marketplace sert une version de chaque plugin à la fois.

Donnez aux deux fichiers `marketplace.json` des valeurs `name` différentes. Claude Code identifie une marketplace par son `name`, donc un utilisateur ne peut pas avoir deux marketplaces avec le même nom enregistrées à la fois.

Avec ces deux catalogues, les utilisateurs qui ajoutent `stable-tools` installent `code-formatter` à partir de la branche `stable`, et les utilisateurs qui ajoutent `latest-tools` l'installent à partir de `latest` :

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

Donnez aux deux refs des versions `plugin.json` différentes, ou omettez `version` pour que le SHA du commit les distingue. Les mises à jour sont détectées en comparant les versions, donc une ref qui se déplace sans changement de version laisse les utilisateurs sur la copie en cache.

Pour assigner les canaux aux groupes d'utilisateurs au lieu de laisser les utilisateurs choisir, un administrateur donne à chaque groupe l'entrée `extraKnownMarketplaces` correspondante, comme décrit sous [Définir la politique de mise à jour](/docs/fr/plugins/org#set-update-policy).

<h2 id="rename-or-remove-a-plugin">
  Renommer ou supprimer un plugin
</h2>

Le `name` d'un plugin est son identifiant. Les utilisateurs le référencent dans les clés de paramètres `enabledPlugins` et `pluginConfigs` et dans `/plugin install`, donc le modifier casse chaque installation existante.

Pour modifier l'étiquette que les utilisateurs voient dans `/plugin` sans casser quoi que ce soit, définissez `displayName` dans `plugin.json` et gardez `name` inchangé.

<h3 id="migrate-users-with-a-renames-map">
  Migrer les utilisateurs avec une carte de renommages
</h3>

Quand vous devez modifier un `name`, ajoutez une carte `renames` de niveau supérieur à `marketplace.json` pour que Claude Code migre les utilisateurs existants au lieu de signaler [`Plugin "<name>" not found in marketplace`](/docs/fr/plugins/troubleshooting#plugin-not-found-in-marketplace). Faites la même chose quand vous supprimez une entrée de `plugins`. La migration automatique nécessite Claude Code v2.1.193 ou ultérieur.

Mappez chaque ancien nom à son nom actuel, ou à `null` quand le plugin est parti. Cette marketplace renomme `formatter` en `code-formatter` et enregistre que `legacy-linter` a été supprimé :

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Après que vous ayez poussé, un utilisateur qui a toujours l'ancien nom activé voit l'un de ces résultats :

* **Entrée renommée** : le plugin se charge sous son nouveau nom. `claude plugin list` et les détails du plugin sous `/plugin` affichent `Renamed to "code-formatter" in the "your-marketplace" marketplace` une fois, et Claude Code réécrit l'ancienne clé à la nouvelle dans `enabledPlugins` et `pluginConfigs` dans les portées de paramètres utilisateur, projet et local.
* **Entrée `null`** : l'ancienne clé est supprimée de ces portées et l'utilisateur voit `Removed from the "your-marketplace" marketplace`.
* **Activé dans les paramètres gérés** : le plugin se charge toujours sous son nouveau nom, mais Claude Code ne peut pas réécrire les paramètres gérés, donc l'avis se répète jusqu'à ce qu'un administrateur mette à jour `enabledPlugins` là.

Pour une marketplace que les utilisateurs ont ajoutée à partir d'un référentiel git ou d'une URL, un plugin renommé signale [`Plugin "<name>" not cached at <path>`](/docs/fr/plugins/troubleshooting#plugin-not-cached-at) jusqu'à ce que l'utilisateur exécute `/plugin install code-formatter@your-marketplace` une fois dans une session.

Traitez `renames` comme un historique d'ajout uniquement. Gardez les anciennes entrées après que tout le monde a migré. Quand vous renommez à nouveau, ajoutez une deuxième entrée plutôt que de modifier la première, parce que Claude Code suit la chaîne à partir du nom le plus ancien.

Dans votre shell, exécutez `claude plugin validate .` après avoir édité la carte. Il rejette une chaîne qui boucle ou qui se termine n'importe où sauf `null` ou un nom dans `plugins`, avec `renames.<name>: chain does not resolve`.

<h3 id="uninstall-removed-plugins-from-users’-machines">
  Désinstaller les plugins supprimés des machines des utilisateurs
</h3>

Pour désinstaller un plugin supprimé des machines des utilisateurs plutôt que de laisser une copie derrière, définissez `"forceRemoveDeletedPlugins": true` au niveau supérieur de `marketplace.json`. Sans le champ, un plugin supprimé reste installé et signale `Plugin "<name>" not found in marketplace` quand une session le charge. Avec lui, Claude Code fait ce qui suit à chaque démarrage de session :

1. Compare ce que les utilisateurs ont installé à partir de votre marketplace par rapport aux entrées et à la carte `renames`, et traite tout plugin qui n'est ni listé ni renommé comme supprimé.
2. Désinstalle chaque plugin supprimé des portées utilisateur, projet et local. Les plugins que seuls les paramètres gérés ont installés restent en place.
3. Liste chaque plugin supprimé sous un en-tête **Flagged** dans `/plugin` avec le statut `Removed from marketplace`.

<h2 id="authenticate-archive-downloads">
  Authentifier les téléchargements d'archive
</h2>

Pour authentifier un téléchargement [`archive`](/docs/fr/plugins/marketplace-reference#archive-plugin-source), comme un téléchargement à partir d'un registre privé, définissez les en-têtes HTTP que Claude Code envoie avec lui. Vous pouvez définir `headers` dans l'un de ces endroits :

* **La source `url` de la marketplace** : la source `url` à partir de laquelle vous avez enregistré la marketplace, comme une entrée [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces).
* **L'entrée du plugin** : sur Claude Code v2.1.238 ou ultérieur, vous pouvez la définir sur l'entrée `marketplace.json` du plugin à la place, à côté de `source`.

Dans l'un ou l'autre endroit, définissez une commande `headersHelper` au lieu de `headers` quand la valeur est de courte durée, comme un jeton que votre registre génère à la demande. Claude Code exécute la commande et envoie l'objet JSON qu'elle imprime comme les en-têtes de cet endroit. Nécessite Claude Code v2.1.238 ou ultérieur.

La [référence de marketplace](/docs/fr/plugins/marketplace-reference#plugin-entries) énumère les champs d'entrée `headers` et `headersHelper`.

L'endroit que vous choisissez décide quels téléchargements obtiennent les en-têtes et quand Claude Code exécute la commande :

| Endroit                     | Téléchargements qui obtiennent les en-têtes                                                                          | Quand Claude Code exécute un `headersHelper` défini là                                                                                                                                                    |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Source `url` de marketplace | Les téléchargements d'archive sur l'origine de l'URL de la marketplace, ce qui signifie le même schéma, hôte et port | Avant chaque récupération du `marketplace.json` de la marketplace et avant chaque téléchargement d'archive sur cette origine. Claude Code réutilise la sortie d'une exécution pendant jusqu'à 60 secondes |
| Entrée de plugin            | Ce téléchargement d'entrée uniquement                                                                                | Uniquement quand un utilisateur installe ou met à jour ce seul plugin par lui-même et [accepte la commande](#how-users-accept-a-headershelper-command)                                                    |

Où les deux endroits définissent un en-tête du même nom, Claude Code envoie la valeur de l'entrée. Au sein d'un endroit, un en-tête que la commande imprime remplace un en-tête du même nom listé dans `headers`.

<h3 id="add-a-headershelper-to-a-plugin-entry">
  Ajouter un headersHelper à une entrée de plugin
</h3>

Cette entrée définit `headersHelper` à côté de `source`. Elle définit aussi [`"strict": false`](/docs/fr/plugins/marketplace-reference#strict-mode), que Claude Code exige d'une entrée `marketplace.json` qui définit `headersHelper` :

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

Pour vérifier l'entrée, exécutez `claude plugin install my-plugin@your-marketplace` dans votre shell. Claude Code vous affiche la commande et l'URL d'archive, et télécharge le zip après que vous l'acceptiez.

<h3 id="write-the-headershelper-command">
  Écrire la commande headersHelper
</h3>

Que vous définissiez `headersHelper` sur une source `url` de marketplace ou sur une entrée de plugin, écrivez la commande pour répondre à ces exigences :

* **Texte de commande** : au maximum 500 caractères ASCII imprimables, sans suite de quatre espaces ou plus.
* **Sortie** : imprimez un objet JSON de noms d'en-têtes et de valeurs de chaîne sur stdout, puis quittez 0 dans les 10 secondes.
* **Shell et répertoire de travail** : Claude Code exécute la commande via `sh`, ou via `cmd.exe` sur Windows. Le répertoire de travail est le répertoire de configuration, qui est `~/.claude` ou [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars#variables). Donnez un chemin absolu ou une commande sur `PATH`, parce qu'un chemin relatif se résout par rapport à ce répertoire, pas au projet de l'utilisateur.
* **Variables que Claude Code supprime** : quand la commande est définie dans une entrée `marketplace.json`, ou dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet, Claude Code supprime de l'environnement chaque variable dont le nom ressemble à un identifiant, par la [même règle qu'elle applique à un `headersHelper` MCP](/docs/fr/mcp#which-variables-a-helper-can-read). `ANTHROPIC_API_KEY` et `MY_REGISTRY_TOKEN` sont tous deux supprimés, donc faites en sorte que la commande lise son identifiant à partir d'un fichier ou d'un magasin d'identifiants. Cette suppression ne s'applique pas à une commande définie dans les paramètres utilisateur, un fichier `--settings`, ou les paramètres gérés.
* **Variables que Claude Code définit** : `CLAUDE_CODE_MARKETPLACE_URL` et `CLAUDE_CODE_MARKETPLACE_NAME` pour la commande d'une source `url`, et `CLAUDE_CODE_PLUGIN_NAME` et `CLAUDE_CODE_PLUGIN_ARCHIVE_URL` pour la commande d'une entrée. `CLAUDE_CODE_MARKETPLACE_NAME` n'est pas défini sur la première récupération après qu'un utilisateur ajoute une marketplace par URL, parce que cette récupération est ce qui fournit le nom.

Une commande qui frappe un jeton porteur imprime un objet comme celui-ci :

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

<h3 id="when-claude-code-skips-a-headershelper-command-or-drops-its-output">
  Quand Claude Code ignore une commande headersHelper ou supprime sa sortie
</h3>

Une commande `headersHelper` ne s'exécute pas, ou les en-têtes de `headers` ou de la sortie de la commande sont supprimés, quand l'un des éléments suivants s'applique :

* **La commande échoue** : si la commande quitte non-zéro, s'exécute au-delà de 10 secondes, ou imprime autre chose qu'un objet JSON de valeurs de chaîne, la récupération ou le téléchargement pour lequel la commande a été exécutée ne se produit pas.
* **L'URL de marketplace ne commence pas par `https://`** : la commande de cette source `url` ne s'exécute pas, et les demandes ne portent que les en-têtes listés dans son champ `headers`.
* **La redirection quitte l'origine** : quand un téléchargement est redirigé hors de l'origine de l'URL d'archive, la demande redirigée ne porte pas de valeurs `headers` ou de sortie de commande de la source `url` de marketplace ou de l'entrée de plugin.
* **L'entrée définit un en-tête de routage ou d'identité** : Claude Code supprime les noms de routage de demande et d'identité de client comme `Host`, `Cookie`, et `X-Forwarded-*` de `headers` et de la sortie de commande d'une entrée, et garde les noms d'authentification comme `Authorization`. Chaque entrée `marketplace.json` est filtrée de cette façon. Pour une entrée de plugin en ligne dans les paramètres, voir [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces).
* **La commande est définie dans les paramètres d'un répertoire `--add-dir`** : la commande est ignorée, sur une source `url` et sur une [entrée de plugin en ligne](/docs/fr/settings-reference#extraknownmarketplaces) de même, et seuls les `headers` de ce fichier sont envoyés.
* **Les paramètres gérés bloquent la commande** : définir [`disableCommandPluginSources`](/docs/fr/settings-reference#disablecommandpluginsources) à `true` bloque les commandes `headersHelper`, et [`allowManagedHooksOnly`](/docs/fr/settings-reference#allowmanagedhooksonly) les bloque aussi sauf si `disableCommandPluginSources` est explicitement `false`. Sous l'un ou l'autre bloc, Claude Code exécute toujours la commande pour une marketplace que les paramètres gérés eux-mêmes déclarent.

<h3 id="how-users-accept-a-headershelper-command">
  Comment les utilisateurs acceptent une commande headersHelper
</h3>

Un utilisateur accepte la commande d'une entrée de plugin chaque fois qu'il installe ou met à jour ce seul plugin par lui-même. Il le fait à partir de la vue propre du plugin dans `/plugin`, ou avec `claude plugin install` ou `claude plugin update`. Claude Code affiche la commande et l'URL d'archive, et exécute la commande uniquement après que l'utilisateur l'accepte.

Dans un shell non-interactif, passez [`--yes`](/docs/fr/plugins/cli-reference#plugin-install) pour accepter la commande. Pour accepter uniquement la commande qu'une exécution `--json` précédente a affichée, passez [`--accept-command`](/docs/fr/plugins/cli-reference#plugin-install) avec le `sha256` que l'exécution a signalé.

Claude Code exécute uniquement la commande qu'il a affichée, pour l'URL d'archive qu'il a affichée. Si la commande ou l'URL d'archive de l'entrée a changé entre-temps, Claude Code refuse l'installation ou la mise à jour. Un changement dans la chaîne de requête seul ne compte pas.

<h3 id="installs-and-updates-that-refuse-the-command-instead-of-asking">
  Installations et mises à jour qui refusent une commande au lieu de demander
</h3>

Sur toute opération autre qu'une installation ou une mise à jour d'un seul plugin, Claude Code n'exécute pas la commande d'une entrée ni ne télécharge son archive. Le plugin reste à sa version installée ou reste désinstallé, et l'utilisateur voit l'un de ces résultats :

* **Installation de plusieurs plugins à la fois, à partir d'une suggestion de plugin, ou en tant que dépendance d'un autre plugin** : Claude Code refuse le plugin qui a la commande et dirige l'utilisateur à la vue propre de ce plugin dans `/plugin`. Les autres plugins dans une installation en masse s'installent toujours. Un plugin qui dépend du plugin refusé échoue à s'installer jusqu'à ce que l'utilisateur installe le plugin refusé par lui-même.
* **Mise à jour automatique en arrière-plan, ou démarrage de session pour un plugin dont l'archive n'a jamais été téléchargée** : Claude Code énumère le plugin dans l'onglet Erreurs `/plugin` pour que l'utilisateur sache l'installer ou le mettre à jour lui-même.

<h3 id="when-a-marketplace-url-sources-command-runs">
  Quand la commande d'une source `url` de marketplace s'exécute
</h3>

Vous déclarez la `headersHelper` d'une source `url` de marketplace dans un fichier de paramètres, comme une entrée [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces), plutôt que dans le catalogue que la marketplace publie. Claude Code ne demande donc pas à l'utilisateur de l'accepter à chaque installation ou mise à jour. Au lieu de cela, le fichier de paramètres qui la déclare décide quand Claude Code l'exécute :

| Fichier de paramètres                                                                             | Quand Claude Code exécute la commande                                                                                                                                                                                                                                                           |
| :------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Paramètres utilisateur, un fichier `--settings`, ou un fichier de paramètres gérés sur la machine | Sans demander, y compris lors d'une actualisation de marketplace en arrière-plan                                                                                                                                                                                                                |
| Le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet                           | Uniquement après que l'utilisateur accepte la [boîte de dialogue de confiance d'espace de travail](/docs/fr/permissions#what-runs-before-you-trust-a-folder) pour ce dossier lui-même. Une session `-p` ou SDK ne compte pas comme l'accepter, et la confiance accordée à un dossier parent non plus |
| Paramètres gérés par serveur                                                                      | Dans une session interactive, uniquement après que l'utilisateur approuve les paramètres livrés dans la [boîte de dialogue d'approbation de sécurité](/docs/fr/server-managed-settings#security-approval-dialogs)                                                                                    |

Pour une [entrée de plugin en ligne](/docs/fr/settings-reference#extraknownmarketplaces) dans l'un de ces fichiers, Claude Code exige la même confiance de dossier ou approbation de paramètres que pour une commande au niveau de la marketplace dans ce fichier, et l'utilisateur accepte aussi la commande de l'entrée à chaque installation ou mise à jour.

<h2 id="depend-on-and-recommend-other-plugins">
  Dépendre d'autres plugins et les recommander
</h2>

Une entrée peut déclarer des dépendances sur d'autres plugins.

* **Plages de version** : une dépendance peut porter une plage semver.
* **Dépendances entre marketplaces** : une dépendance d'une autre marketplace s'installe uniquement quand votre marketplace énumère cette marketplace dans `allowCrossMarketplaceDependenciesOn`.

Pour les plages de version, la convention de balise git `<plugin>--v<version>` qu'elles se résolvent par rapport à, et la confiance entre marketplaces, voir [Dépendances de plugin](/docs/fr/plugins/dependencies).

Pour que Claude Code suggère un plugin quand un projet le correspond, ajoutez un bloc `relevance` à l'entrée avec les signaux qui identifient le projet. Les utilisateurs voient les suggestions de votre marketplace uniquement quand un administrateur l'énumère dans `pluginSuggestionMarketplaces`. Pour les signaux et l'étape d'activation, voir [Pertinence du plugin](/docs/fr/plugins/relevance).

<h2 id="work-around-what-a-marketplace-can’t-do">
  Contourner ce qu'une marketplace ne peut pas faire
</h2>

Certaines choses que les propriétaires demandent n'ont pas de champ dans `marketplace.json`. Voici l'option la plus proche pour chacune :

* **Restreindre ce que d'autres utilisateurs installent** : la liste d'autorisation de marketplace est un paramètre géré, `strictKnownMarketplaces`. Voir [Restreindre ce que les utilisateurs peuvent installer](/docs/fr/plugins/org#restrict-what-users-can-install).
* **Installer ou activer un plugin sans que l'utilisateur le demande** : aucun champ d'entrée n'installe un plugin. Les `enabledPlugins` gérés le font pour une flotte ; voir [Pré-installer et exiger des plugins](/docs/fr/plugins/org#pre-install-and-require-plugins).
* **Afficher des entrées différentes à différents utilisateurs** : les entrées ne portent pas de champ d'audience, et chaque utilisateur qui ajoute la marketplace voit le catalogue entier. Hébergez des marketplaces séparées pour des audiences séparées.
* **Marquer un plugin comme obsolète** : il n'y a pas d'état d'obsolescence. L'option est de supprimer l'entrée, de mapper son nom à `null` dans `renames`, et éventuellement de définir `forceRemoveDeletedPlugins`.
* **Activer la mise à jour automatique pour vos utilisateurs** : chaque utilisateur l'active sous **Marketplaces** dans `/plugin`, ou un administrateur définit `autoUpdate` dans les paramètres gérés. Voir [Activer la mise à jour automatique](#turn-on-auto-update).
* **Porter les identifiants git** : aucun champ de marketplace ne détient un jeton git. L'accès à une marketplace ou un plugin hébergé sur git suit la configuration git de l'utilisateur, selon [Accorder l'accès à une marketplace privée](#grant-access-to-a-private-marketplace). Pour les sources `archive`, une entrée peut définir [`headers` ou `headersHelper`](#authenticate-archive-downloads) à la place.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Référence de marketplace](/docs/fr/plugins/marketplace-reference) : champs `marketplace.json`, types de source, et messages de validation
* [Gérer les plugins pour votre organisation](/docs/fr/plugins/org) : exiger, restreindre, ou ensemencer votre marketplace sur les machines de votre organisation
* [Dépendances de plugin](/docs/fr/plugins/dependencies) : étiqueter les versions pour que les plugins qui dépendent du vôtre puissent résoudre les versions
* [Dépanner les plugins](/docs/fr/plugins/troubleshooting) : les erreurs que vos utilisateurs voient lors de l'ajout ou de la mise à jour à partir de votre marketplace
