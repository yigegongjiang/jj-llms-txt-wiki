> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence du chargement des plugins

> Tracez d'où Claude Code charge chaque plugin, quel fichier de paramètres décide s'il se charge, et pourquoi une mise à jour n'a rien changé.

Utilisez cette page quand un plugin ne s'est pas chargé, a chargé une copie différente de celle attendue, ou n'a pas appliqué une mise à jour, et vous voulez voir quelle source, portée de paramètres, ou fichier sur disque a décidé cela. Elle donne les règles que Claude Code applique au démarrage d'une session et chaque fois que vous exécutez `/reload-plugins`. Vous pouvez aussi demander à Claude de lire cette page et diagnostiquer votre configuration.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Étapes d'installation, d'activation, de désactivation et de mise à jour** : voir [Installer et gérer les plugins](/docs/fr/plugins/install)
  * **Vous avez un message d'erreur spécifique** : voir [Dépanner les plugins](/docs/fr/plugins/troubleshooting)
</Note>

Commencez par [Vérifier à quel stade un plugin est arrivé](#check-which-stage-a-plugin-reached) pour les trois stades qu'un plugin installé traverse, ou allez à la section qui correspond à ce que vous voyez :

* Un plugin que vous avez désactivé se charge toujours : [Trouver où un plugin est activé](#find-where-a-plugin-is-enabled)
* Une mise à jour n'a rien changé : [Versions et mises à jour](#versions-and-updates)
* Vous regardez les fichiers sous `~/.claude/plugins/` : [Trouver les plugins sur disque](#find-plugins-on-disk)
* Un plugin `--plugin-dir` ne s'est pas chargé, ou un plugin du même nom s'est chargé à la place : [Conflits de noms](#name-conflicts)

<h2 id="check-which-stage-a-plugin-reached">
  Vérifier à quel stade un plugin est arrivé
</h2>

Une entrée `enabledPlugins` devient un plugin que vous pouvez utiliser en stades : vos paramètres le déclarent, Claude Code le récupère sur disque, et la session en cours le charge. Quand un plugin ne se comporte pas comme un fichier de paramètres le suggère, vérifiez à quel stade il est arrivé :

* **Déclaré, dans les paramètres** : `enabledPlugins` dit quels plugins doivent être activés, et `extraKnownMarketplaces` dit quels marchés doivent exister. Quand vous exécutez `claude plugin marketplace add`, Claude Code écrit le marché dans `extraKnownMarketplaces` dans vos paramètres utilisateur ainsi que sur disque
* **Récupéré, sur disque sous `~/.claude/plugins/`** : les enregistrements de ce que Claude Code a récupéré, et les fichiers récupérés eux-mêmes :
  * `known_marketplaces.json` enregistre chaque marché que Claude Code a récupéré, avec sa `source`, `installLocation`, `lastUpdated`, et `autoUpdate`. Il y a un `known_marketplaces.json` par utilisateur, donc un marché que vous ajoutez dans un projet est disponible dans chaque projet
  * `installed_plugins.json` enregistre chaque installation avec sa `scope`, `installPath`, et `version`
  * `cache/` contient les fichiers du plugin
* **Chargé, dans la session en cours** : l'ensemble de plugins que Claude Code a chargé au démarrage ou au dernier `/reload-plugins`. Les modifications des paramètres ou du disque ne atteignent cette couche que quand vous exécutez `/reload-plugins` ou démarrez une nouvelle session. C'est pourquoi `claude plugin update` se termine par `Restart to apply changes.` et les mises à jour en arrière-plan vous invitent avec `Run /reload-plugins to apply`

<h3 id="plugins-and-marketplaces-that-aren’t-on-disk-at-session-start">
  Plugins et marchés qui ne sont pas sur disque au démarrage de la session
</h3>

Les plugins se chargent au démarrage de la session à partir de `installed_plugins.json` et du cache sans utiliser le réseau. Après le démarrage de la session, Claude Code vérifie les marchés déclarés en arrière-plan :

* **Un marché que les paramètres déclarent mais que `known_marketplaces.json` manque** : Claude Code le clone, puis recharge les plugins et télécharge les plugins activés qui ne sont pas en cache
* **Un marché déclaré dont la source a changé dans les paramètres** : Claude Code le récupère à nouveau à partir de la nouvelle source et affiche `Plugins changed. Run /reload-plugins to activate.`

Un plugin activé qu'aucun chemin n'a récupéré et qui n'a pas de répertoire de cache utilisable affiche `Plugin "<name>" not cached at <path>` dans l'onglet **Errors** de `/plugin`, et `claude plugin list` ajoute `— run /plugin to refresh` à la même ligne. Pour le correctif, voir [`Plugin "<name>" not cached at <path>`](/docs/fr/plugins/troubleshooting#plugin-not-cached-at).

<h2 id="find-where-a-plugin-came-from">
  Trouver d'où vient un plugin
</h2>

Chaque plugin a un id de la forme `<name>@<origin>`, ce que vous voyez dans les fichiers de paramètres et dans `claude plugin list --json`. La partie après `@` vous dit où Claude Code a trouvé le plugin :

| L'ID se termine par | Comment le plugin est arrivé là                                                                                                                                                                                                      | Comment vous l'activez ou le désactivez                                                                                                                                                                 |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `@<marketplace>`    | Vous l'avez installé à partir d'un marché que vous avez ajouté                                                                                                                                                                       | `"<name>@<marketplace>": true` ou `false` sous `enabledPlugins` dans un fichier de paramètres                                                                                                           |
| `@inline`           | Vous avez démarré Claude Code avec `--plugin-dir` ou `--plugin-url`, défini [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/fr/env-vars#variables), ou une application Agent SDK a passé l'option `plugins`. Il se charge pour cette session uniquement | Activé pour la session sauf si le manifeste définit `defaultEnabled: false` ou un fichier de paramètres définit `"<name>@inline": false`                                                                |
| `@skills-dir`       | Vous avez enregistré un répertoire de plugin qui a un `.claude-plugin/plugin.json` sous `~/.claude/skills/` ou le `.claude/skills/` du projet                                                                                        | Le `defaultEnabled` du manifeste, sauf si un fichier de paramètres définit `"<name>@skills-dir"` à `true` ou `false`                                                                                    |
| `@synced`           | Vous ou votre organisation l'avez activé pour votre compte claude.ai, et Claude Code l'a [téléchargé](#synced-plugins)                                                                                                               | Activé sauf si le manifeste définit `defaultEnabled: false` ou un fichier de paramètres définit `"<name>@synced": false`. Un plugin que votre organisation marque comme requis se charge indépendamment |

Pour un plugin de marché, `<name>` est le nom d'entrée dans `marketplace.json` ; pour `@inline` et `@skills-dir` c'est le `name` dans le manifeste du plugin.

Les noms d'origine dans ce tableau sont réservés, donc aucun marché ne peut être nommé `inline`, `skills-dir`, ou `synced`.

<h3 id="entry-name-and-manifest-name">
  Nom d'entrée et nom de manifeste
</h3>

Un plugin de marché a deux noms, et ils peuvent différer :

* **Le nom d'entrée dans `marketplace.json`** : la clé d'installation et d'activation. C'est ce que vous écrivez dans `enabledPlugins`, ce que le répertoire de cache est nommé d'après, et ce que `claude plugin list` affiche
* **Le `name` dans le manifeste** : ce sous lequel les composants du plugin sont espacés de noms, et ce que [les conflits de noms](#name-conflicts) comparent

<h3 id="plugins-shared-through-a-repository">
  Plugins partagés via un référentiel
</h3>

Pour partager un plugin via un référentiel, listez-le sous `enabledPlugins` dans `.claude/settings.json` ou placez-le sous `.claude/skills/`. Claude Code ne scanne pas le répertoire `.claude/plugins/` d'un projet.

Une session cloud n'ajoute pas les marchés qu'un référentiel liste sous [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces), car cela nécessite la boîte de dialogue de confiance de l'espace de travail, qu'une session cloud ne montre jamais.

Un plugin de répertoire de compétences de portée de projet se charge uniquement à partir du `.claude/skills/` du [répertoire de travail principal](/docs/fr/permissions#working-directories) de la session, et seulement après que vous acceptiez la [boîte de dialogue de confiance de l'espace de travail](/docs/fr/permissions#what-runs-before-you-trust-a-folder) pour ce dossier. Il ne [recherche pas les répertoires parents jusqu'à la racine du référentiel](/docs/fr/skills#discovery-from-parent-and-nested-directories) comme le font les compétences et commandes ordinaires. Si vous lancez à partir d'un sous-répertoire, un plugin à la racine du référentiel ne se charge pas. Lancez plutôt à partir de la racine du référentiel, ou [déplacez la session là avec `/cd`](/docs/fr/permissions#move-the-session-to-another-directory) sur v2.1.246 ou ultérieur.

Un plugin de portée de projet est archivé dans le référentiel et atteint chaque collaborateur qui le clone. Parce que ce contenu provient du référentiel plutôt que de vous, il se charge seulement après la même vérification de confiance qui s'applique aux règles d'autorisation de projet dans `.claude/settings.json`. Faire confiance à un dossier parent ou exécuter avec `-p` ne suffit pas. Les composants qui exécutent du code sont restreints davantage :

* Les serveurs MCP qu'il déclare passent par l'[approbation par serveur](/docs/fr/mcp) identique qu'un `.mcp.json` de projet
* Les serveurs MCP qu'il déclare comme un [bundle MCP](/docs/fr/plugins/manifest-reference#mcpservers), un fichier `.mcpb` ou `.dxt`, ou à partir d'un fichier en dehors du répertoire du plugin sont ignorés. Déclarez-les en ligne ou dans un `.mcp.json` à l'intérieur du répertoire du plugin
* [Les moniteurs en arrière-plan](/docs/fr/plugins/components#monitors) ne se chargent pas

Les plugins de portée personnelle n'ont aucune de ces restrictions.

Pour savoir comment écrire des plugins `--plugin-dir` et de répertoire de compétences, voir [Créer des plugins](/docs/fr/plugins/create).

<h3 id="synced-plugins">
  Plugins synchronisés à partir de claude.ai
</h3>

Un plugin que vous activez pour votre compte claude.ai se charge aussi dans Claude Code, aux côtés des plugins que vous installez à partir de marchés. Cela inclut les plugins que votre organisation active pour ses membres. Chacun de ces plugins se charge comme `<name>@synced`, sans marché et sans [enregistrement d'installation](#check-which-stage-a-plugin-reached).

Dans les sessions de terminal, les compétences, agents, hooks, serveurs MCP et serveurs LSP d'un plugin synchronisé se chargent tous, avec la même confiance qu'un plugin de marché que vous avez installé.

Pour les composants que Cowork charge, voir [Plugins sur claude.ai et dans Cowork](https://claude.com/docs/plugins/overview) sur claude.com.

Les plugins synchronisés se chargent dans les sessions Cowork et dans les sessions de terminal où vous vous connectez avec votre compte claude.ai :

* **[Cowork](https://claude.com/product/cowork)** : Claude Code les télécharge dans l'environnement propre de la session au démarrage de la session
* **Sessions de terminal** : chaque fois que vous démarrez Claude Code, il se synchronise une fois en arrière-plan, téléchargeant les plugins nouveaux et mis à jour et supprimant ceux que vous ou votre organisation avez désactivés. La synchronisation dans les sessions de terminal nécessite Claude Code v2.1.273 ou ultérieur

<h4 id="sync-timing-in-terminal-sessions">
  Timing de synchronisation dans les sessions de terminal
</h4>

Parce que la synchronisation de terminal s'exécute en arrière-plan, elle peut se terminer après le démarrage de votre session. Quand elle ajoute, met à jour ou supprime un plugin synchronisé dans une session interactive, vous voyez `Plugins changed. Run /reload-plugins to activate.` Exécutez `/reload-plugins` pour charger la modification dans cette session, ou laissez-la pour la prochaine fois que vous démarrez Claude Code.

Si vous activez un plugin sur claude.ai pendant qu'une session s'exécute, le plugin télécharge la prochaine fois que vous démarrez Claude Code.

<h4 id="sign-in-requirements-for-terminal-sync">
  Exigences de connexion pour la synchronisation de terminal
</h4>

Dans votre terminal, les plugins se synchronisent uniquement dans les sessions où vous vous connectez avec votre compte claude.ai.

Si vous vous êtes connecté sur une version antérieure de Claude Code, cette connexion ne couvre pas les plugins jusqu'à ce que Claude Code la renouvelle en arrière-plan. Pour accéder plus tôt, exécutez `/login` à nouveau. La synchronisation des plugins commence alors la prochaine fois que vous démarrez Claude Code.

<h4 id="control-which-synced-plugins-load">
  Contrôler quels plugins synchronisés se chargent
</h4>

Vous pouvez désactiver les plugins synchronisés un par un, sauf un plugin que votre organisation exige, ou désactiver tous les plugins synchronisés sur la machine :

* **Un plugin** : `claude plugin disable <name>@synced` dans votre shell et l'onglet **Installed** de `/plugin` dans une session enregistrent tous les deux `"<name>@synced": false` dans votre [`enabledPlugins`](/docs/fr/settings-reference#enabledplugins) au niveau utilisateur. Pour garder le plugin hors d'un projet dans chaque environnement, définissez la même clé dans le `.claude/settings.json` engagé du projet
* **Tous les plugins synchronisés sur une machine** : définissez [`syncClaudeAiPlugins`](/docs/fr/settings-reference#syncclaudeaiplugins) à `false` dans vos paramètres utilisateur, ou votre organisation le définit dans [les paramètres gérés](/docs/fr/managed-settings). Claude Code arrête de télécharger, et la prochaine fois que vous le démarrez, il déplace les plugins qu'il a déjà synchronisés vers `~/.claude/plugins/.trash/` et ne les charge plus. Si votre organisation désactive les Compétences sur claude.ai, les plugins arrêtent aussi de se synchroniser
* **Un plugin que votre organisation exige** : un plugin que votre organisation marque comme requis sur claude.ai se charge même si vous l'avez désactivé plus tôt. `claude plugin disable` le refuse avec `Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.`, et `claude plugin list` le marque `required by your org`

Pour supprimer un plugin sur claude.ai, voir [Gérer les plugins installés](/docs/fr/plugins/install#manage-installed-plugins).

<h2 id="find-where-a-plugin-is-enabled">
  Trouver où un plugin est activé
</h2>

Vous pouvez définir une entrée `enabledPlugins` dans l'une de six sources. Le tableau les liste de la plus basse à la plus haute précédence, et qui chacune s'applique. Pour les fichiers de paramètres eux-mêmes, voir [Fichiers de paramètres et qui ils affectent](/docs/fr/settings#where-settings-live).

| Source      | Où vous la définissez                                                                                        | Atteint                                                                                                                   |
| :---------- | :----------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `--add-dir` | `.claude/settings.json` ou `.claude/settings.local.json` dans un répertoire que vous passez avec `--add-dir` | Cette session uniquement. Seule une valeur `true` a un effet, et chaque autre source la remplace                          |
| `user`      | `~/.claude/settings.json`                                                                                    | Vous, dans chaque projet                                                                                                  |
| `project`   | `.claude/settings.json`                                                                                      | Tous ceux qui clonent le référentiel                                                                                      |
| `local`     | `.claude/settings.local.json`                                                                                | Vous, dans ce référentiel uniquement                                                                                      |
| `flag`      | La valeur `--settings` que vous passez au lancement                                                          | Cette session uniquement                                                                                                  |
| `managed`   | [Paramètres gérés](/docs/fr/managed-settings)                                                                     | Chaque utilisateur que la politique couvre. `true` force-active et `false` bloque, et aucune autre source ne les remplace |

Ces sources fusionnent clé par clé. Pour chaque id de plugin, la valeur qui s'applique est celle de la source de plus haute précédence qui mentionne l'id. Une source qui ne mentionne pas l'id laisse la valeur de la source de plus basse précédence en effet.

<h3 id="disabled-in-user-settings-but-still-loads">
  Désactivé dans les paramètres utilisateur mais se charge toujours
</h3>

Si vous définissez un plugin à `false` dans `~/.claude/settings.json` et qu'il se charge toujours, un `true` dans une source de plus haute précédence le remplace. La ligne du plugin dans `claude plugin list` et dans `/plugin` affiche `Disabled in ~/.claude/settings.json but still loads — project settings enable it, which overrides your user setting`. Le message nomme la source qui vous a remplacé : `project`, `project, gitignored` pour `.claude/settings.local.json`, `cli flag`, ou `managed`.

Pour refuser un plugin activé par le projet sur votre machine, définissez l'id à `false` dans `.claude/settings.local.json`, qui a une plus haute précédence que le fichier de projet.

<h3 id="enabled-in-project-settings-but-not-installed">
  Activé dans les paramètres de projet mais non installé
</h3>

Quand le seul `true` d'un plugin est dans le `.claude/settings.json` du projet, Claude Code ne le récupère pas sur une machine où il n'est pas installé, sauf si son entrée de marché a une [source de chemin relatif](/docs/fr/plugins/marketplace-reference#plugin-sources) ou un [répertoire de semence](/docs/fr/plugins/org#seed-containers-and-ci) le contient déjà. À la place, l'onglet **Errors** de `/plugin` affiche `Plugin "<name>" is enabled in project settings but isn't installed here`.

Un plugin de chemin relatif n'a besoin d'aucun enregistrement d'installation car il se charge à partir du marché lui-même.

Claude Code récupère un plugin avec une source externe uniquement quand l'une de ces sources le définit à `true` :

* Vos paramètres utilisateur
* Un `.claude/settings.local.json` que git ne suit pas
* L'indicateur `--settings`
* Paramètres gérés

<h2 id="find-plugins-on-disk">
  Trouver les plugins sur disque
</h2>

Claude Code garde les fichiers de plugin et les enregistrements d'état sous une racine de plugins, qui est `~/.claude/plugins` sauf si vous définissez [`CLAUDE_CODE_PLUGIN_CACHE_DIR`](/docs/fr/env-vars). Chaque chemin du tableau est relatif à cette racine.

| Chemin                                                | Ce qu'il contient                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :---------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `cache/<marketplace>/<plugin>/<version>/`             | Un répertoire par version installée d'un plugin de marché. `<plugin>` est le nom d'entrée du marché et `<version>` est la [version résolue](#versions-and-updates). `${CLAUDE_PLUGIN_ROOT}` pointe vers ce répertoire                                                                                                                                                                                                                                               |
| `data/<plugin-id>/`                                   | Le répertoire persistant du plugin, exposé comme `${CLAUDE_PLUGIN_DATA}`. Pour savoir comment `<plugin-id>` est formé, voir [Variables de chemin et données persistantes](/docs/fr/plugins/components#path-variables-and-persistent-data). Claude Code le crée quand un composant de plugin l'utilise d'abord et le garde à travers les mises à jour. Claude Code le supprime quand vous désinstallez le plugin de sa dernière portée, sauf si vous passez `--keep-data` |
| `marketplaces/<name>/`                                | Le clone ou le téléchargement d'un marché ajouté à partir de GitHub, d'un autre hôte Git, ou d'une URL. Un marché ajouté à partir d'une source `file` ou `directory` locale n'a pas de copie ici, et son `installLocation` dans `known_marketplaces.json` est le chemin que vous avez donné                                                                                                                                                                         |
| `synced/`                                             | Les plugins que Claude Code a [synchronisés à partir de votre compte claude.ai](#synced-plugins)                                                                                                                                                                                                                                                                                                                                                                    |
| `.trash/`                                             | Les plugins que la synchronisation claude.ai a supprimés, comme après que vous en ayez désactivé un sur claude.ai ou arrêté la synchronisation                                                                                                                                                                                                                                                                                                                      |
| `installed_plugins.json` et `known_marketplaces.json` | Les enregistrements de ce que Claude Code a installé et quels marchés il a récupérés, décrits sous [Vérifier à quel stade un plugin est arrivé](#check-which-stage-a-plugin-reached). Un [marché hébergé sur claude.ai](/docs/fr/plugins/install#add-from-claude-ai) est enregistré dans `known_marketplaces_claudeai.json` à la place                                                                                                                                   |
| `flagged-plugins.json`                                | Les plugins que Claude Code a désinstallés parce que leur marché les a retirés de la liste. Ils apparaissent dans la section **Flagged** de `/plugin` ; voir [Héberger un marché](/docs/fr/plugins/host-marketplace)                                                                                                                                                                                                                                                     |

Parce que `${CLAUDE_PLUGIN_ROOT}` pointe vers un répertoire de version, le chemin racine d'un plugin change avec chaque version. Gardez les fichiers durables d'un plugin dans `${CLAUDE_PLUGIN_DATA}` à la place.

<h3 id="in-place-and-copied-plugins">
  Plugins en place et copiés
</h3>

Claude Code charge certains plugins en place d'où vous les gardez et copie le reste dans le cache, selon leur origine :

* **Plugins `--plugin-dir` et de répertoire de compétences** : le répertoire se charge en place et n'est jamais copié. Une archive `--plugin-url` ou un `.zip` `--plugin-dir` est d'abord extrait dans un répertoire temporaire de session
* **Plugins de chemin relatif dans un marché que vous avez ajouté à partir d'un répertoire local** : le plugin se charge en place à partir de son chemin à l'intérieur du dossier du marché. Vos modifications du répertoire source prennent effet au prochain démarrage de session ou `/reload-plugins`, et vous n'avez pas besoin d'augmenter la version. Les processus de hook du plugin et les serveurs MCP et LSP reçoivent un `CLAUDE_PLUGIN_ROOT` qui pointe vers le répertoire source. Pour ses dépendances de package Node.js, voir [Quand l'installation de dépendance s'exécute](#when-the-dependency-install-runs)
* **Plugins de source `command` en [mode lien](/docs/fr/plugins/marketplace-reference#command-plugin-source)** : le répertoire que la commande a imprimé se charge en place, via des liens dans l'entrée de cache
* **Tous les autres plugins de marché** : Claude Code copie le plugin dans `cache/<marketplace>/<plugin>/<version>/` à l'installation et charge cette copie. Les fichiers en dehors du répertoire du plugin ne sont pas copiés, donc quand un script à l'intérieur d'un plugin copié lit un chemin au-dessus de la racine du plugin, comme `../shared`, il ne les trouve pas

<h3 id="paths-that-escape-the-plugin-directory">
  Chemins qui s'échappent du répertoire du plugin
</h3>

Qu'un plugin se charge en place ou à partir d'une copie en cache, Claude Code ne le laisse pas déclarer des composants en dehors de son propre répertoire. Il rejette un chemin de composant qui se résout en dehors de la racine du plugin, que le chemin soit déclaré dans `plugin.json` ou dans une entrée de marché :

* **Un chemin qui pointe en dehors du plugin tel qu'écrit**, comme `../shared-utils`
* **Un lien symbolique qui mène en dehors du plugin**, autre que [les liens entre plugins dans un marché](/docs/fr/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)
* **Sur macOS et Linux, un chemin qui contient une barre oblique inverse n'importe où dedans**, même quand le chemin reste à l'intérieur du plugin. Les composants déclarés avec des chemins de barre oblique inverse se chargent donc sur Windows uniquement, donc écrivez les chemins de composant avec des barres obliques avant, comme `./commands/deploy.md`

Un chemin rejeté apparaît comme une erreur [`path escapes plugin directory`](/docs/fr/errors#path-escapes-plugin-directory), et le plugin se charge sans ce composant.

<h3 id="cleanup-of-previous-versions">
  Nettoyage des versions précédentes
</h3>

Quand vous mettez à jour ou désinstallez un plugin, Claude Code écrit un marqueur `.orphaned_at` dans le répertoire de version précédente. Il supprime ce répertoire dans un nettoyage en arrière-plan 14 jours plus tard, donc une session qui a déjà chargé l'ancienne version continue de s'exécuter.

Le balayage s'exécute uniquement tant que `installed_plugins.json` enregistre au moins une installation. Après que vous ayez désinstallé votre dernier plugin, les répertoires orphelins restent jusqu'à ce que vous en installiez un autre.

<h3 id="node-js-package-dependencies">
  Dépendances de package Node.js
</h3>

Quand Claude Code copie un plugin dans le cache, il installe aussi les dépendances de package Node.js du plugin là, donc les hooks et serveurs MCP du plugin peuvent les charger.

Cette section couvre les packages npm et Bun qu'un plugin déclare dans son propre `package.json`. Pour les plugins qui dépendent d'autres plugins, voir [versions de dépendance de plugin](/docs/fr/plugins/dependencies).

<h4 id="when-the-dependency-install-runs">
  Quand l'installation de dépendance s'exécute
</h4>

Claude Code exécute l'installation à l'intérieur du répertoire de version copié chaque fois qu'il en crée un :

* Quand vous installez un plugin
* Quand Claude Code met à jour un plugin vers une nouvelle version
* Au démarrage de la session quand un plugin activé n'est pas en cache, comme sur une nouvelle machine

Pour un plugin de chemin relatif [chargé en place](#in-place-and-copied-plugins) à partir d'un marché de répertoire local, Claude Code n'installe pas les dépendances dans le répertoire source. Installez-les là vous-même, ou à partir d'un hook dans [`${CLAUDE_PLUGIN_DATA}`](/docs/fr/plugins/components#path-variables-and-persistent-data).

L'installation s'exécute uniquement quand le répertoire racine du plugin contient à la fois un `package.json` et un fichier de verrouillage pris en charge. Le fichier de verrouillage décide quelle commande Claude Code exécute :

| Fichier de verrouillage                      | Commande                                         |
| :------------------------------------------- | :----------------------------------------------- |
| `bun.lock` ou `bun.lockb`                    | `bun install --frozen-lockfile --ignore-scripts` |
| `npm-shrinkwrap.json` ou `package-lock.json` | `npm ci --ignore-scripts`                        |

Si un plugin contient plus d'un de ces fichiers de verrouillage, Claude Code utilise la première correspondance, en vérifiant dans l'ordre : `bun.lock`, `bun.lockb`, `npm-shrinkwrap.json`, `package-lock.json`.

Claude Code ignore l'installation pour les fichiers de verrouillage Yarn et pnpm et pour un `bunfig.toml` à côté du fichier de verrouillage Bun :

* Si votre plugin n'a qu'un `yarn.lock` ou `pnpm-lock.yaml`, remplacez-le par un fichier de verrouillage npm
* Si un `bunfig.toml` est dans le même répertoire que le fichier de verrouillage Bun, supprimez le `bunfig.toml`, ou remplacez le fichier de verrouillage Bun par un fichier de verrouillage npm

Incluez un fichier de verrouillage npm pour atteindre le plus d'utilisateurs. Claude Code exécute le gestionnaire de packages du fichier de verrouillage correspondant à partir du PATH de l'utilisateur et n'essaie pas l'autre fichier de verrouillage à la place si ce gestionnaire de packages manque.

Pour un plugin distribué via une source npm, utilisez `npm-shrinkwrap.json`, car npm exclut `package-lock.json` des packages publiés.

<h4 id="limits-on-the-dependency-install">
  Limites sur l'installation de dépendance
</h4>

Claude Code contraint cette installation de dépendance de sorte qu'aucun code du plugin ou de ses packages ne s'exécute pendant celle-ci, et limite combien de temps elle peut s'exécuter :

* **Résolution gelée** : Bun et npm installent exactement ce que le fichier de verrouillage épingle, et échouent plutôt que de re-résoudre les versions quand `package.json` et le fichier de verrouillage ne sont pas d'accord
* **Pas de scripts de cycle de vie** : `--ignore-scripts` empêche les scripts `preinstall`, `install`, et `postinstall` de s'exécuter, donc les dépendances qui construisent des modules natifs dans ces scripts téléchargent mais ne compilent pas pendant cette installation
* **Délai d'expiration de 60 secondes** : Claude Code arrête une installation qui s'exécute plus longtemps et la traite comme échouée

Claude Code récupère un plugin de source npm avant cette installation de dépendance, et aucun des scripts d'installation propres du package ne s'exécute pendant la récupération. Voir [source de plugin npm](/docs/fr/plugins/marketplace-reference#npm-plugin-source).

Vous ne pouvez pas désactiver l'installation automatique. Aucun paramètre ou variable d'environnement ne la désactive.

Dans les réseaux restreints, voir les [exigences d'accès réseau](/docs/fr/network-config#network-access-requirements) pour les hôtes à autoriser.

<h4 id="when-the-dependency-install-fails-or-is-skipped">
  Quand l'installation de dépendance échoue ou est ignorée
</h4>

Une installation échouée ou ignorée ne bloque jamais le plugin, et chaque cas laisse un signe différent :

* Une installation échouée, ou une ignorée à cause d'un fichier de verrouillage Yarn ou pnpm ou un `bunfig.toml`, apparaît comme un avertissement dans la sortie `claude --debug`
* Un plugin avec un `package.json` et aucun fichier de verrouillage est ignoré sans entrée de journal
* Une installation expirée peut laisser un arbre `node_modules` partiel dans la copie en cache

Quand l'installation automatique ne peut pas fournir une dépendance, installez-la à partir d'un hook dans le [répertoire de données persistantes](/docs/fr/plugins/components#path-variables-and-persistent-data). Cela inclut les packages qui ont besoin de leurs scripts de cycle de vie pour construire, les dépendances Python, et les plugins verrouillés avec Yarn ou pnpm.

<h2 id="versions-and-updates">
  Versions et mises à jour
</h2>

Si l'auteur d'un plugin a poussé de nouveaux commits et `claude plugin update` affiche `<name> is already at the latest version (<version>).`, la version que Claude Code calcule pour le plugin est inchangée, donc rien ne change sur disque.

Claude Code calcule une version pour chaque plugin qu'il installe, et c'est comment il détecte une mise à jour. `claude plugin update` et la mise à jour automatique en arrière-plan calculent la version à nouveau et ignorent le plugin quand elle correspond à ce que `installed_plugins.json` enregistre.

La version nomme aussi le répertoire de cache du plugin.

Un manifeste qui épingle `"version"` est une façon que la version calculée reste la même à travers les commits. Voir [Comment Claude Code calcule la version](#how-claude-code-computes-the-version) pour l'ordre de résolution.

Un plugin [chargé en place](#in-place-and-copied-plugins) à partir d'un marché de répertoire local charge ses fichiers source actuels à chaque démarrage de session, quoi que sa chaîne de version dise. Pour un plugin à partir d'un [marché hébergé sur claude.ai](/docs/fr/plugins/install#add-from-claude-ai), la version que claude.ai enregistre pour le plugin est sa version, et le `version` du manifeste n'est pas lu.

<h3 id="how-claude-code-computes-the-version">
  Comment Claude Code calcule la version
</h3>

Pour un marché que vous avez ajouté par source, Claude Code choisit la règle par le type `source` de l'entrée de marché du plugin. La [référence de marché](/docs/fr/plugins/marketplace-reference#plugin-sources) liste les types de source. Pour chaque type de source dans cette liste sauf `command` :

1. Le champ `version` dans le manifeste du plugin vient d'abord
2. Puis le champ `version` dans l'entrée de marché du plugin
3. Quand aucun n'est défini, la version vient du type de source :

| Type de source                                                                            | Version quand aucun champ `version` n'est défini                                                                                                           |
| :---------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `github`, `url`, ou `git-subdir`                                                          | Le SHA du commit de la source, raccourci à 12 caractères. Une version `git-subdir` porte aussi un hash du chemin du sous-répertoire                        |
| `archive`                                                                                 | Le digest SHA-256, raccourci à 12 caractères : l'épingle `sha256` dans l'entrée de marché, ou le digest du fichier téléchargé quand il n'y a pas d'épingle |
| Chemin relatif à l'intérieur d'un marché hébergé sur Git                                  | Le SHA du commit du répertoire installé                                                                                                                    |
| Répertoire local, quand ni le répertoire du plugin ni son marché n'est un référentiel git | `unknown`                                                                                                                                                  |
| `npm`                                                                                     | `unknown`                                                                                                                                                  |

Claude Code ne prend pas la version à partir d'un référentiel qui enferme le chemin d'installation, comme un `~/.claude` géré par git.

Pour une source `command`, Claude Code dérive toujours la version à partir de ce que la commande a produit : un hash de 12 caractères seul, ou `<manifest version>-<hash>` quand le manifeste en définit un. Le `version` de l'entrée de marché est ignoré pour les sources de commande. Pour ce que le hash couvre, voir [Mode copie et mode lien](/docs/fr/plugins/marketplace-reference#copy-mode-and-link-mode).

Parce que le manifeste vient d'abord, un manifeste qui épingle `"version": "1.0.0"` garde chaque utilisateur sur la copie en cache jusqu'à ce que son auteur change la chaîne, cependant de nombreux commits ils poussent. Pour laisser les utilisateurs suivre les commits à la place, laissez `version` hors du manifeste et de l'entrée. [Héberger un marché](/docs/fr/plugins/host-marketplace) couvre quel choix convient à quel setup de version.

<h3 id="when-claude-code-refreshes-a-marketplace-before-an-install">
  Quand Claude Code actualise un marché avant une installation
</h3>

Quand vous installez un plugin, Claude Code le cherche dans sa copie locale du catalogue de marché. Vous pouvez exécuter `/plugin install` dans une session ou `claude plugin install` dans votre shell, et nommer le plugin avec ou sans son marché. Le tableau montre laquelle de ces combinaisons actualise la copie locale.

| Nom du plugin      | Commande                                     | Ce que Claude Code actualise                                                                          |
| :----------------- | :------------------------------------------- | :---------------------------------------------------------------------------------------------------- |
| `name@marketplace` | `/plugin install` ou `claude plugin install` | Le marché nommé, avant la recherche                                                                   |
| `name` seul        | `/plugin install`                            | Uniquement les marchés qui ont l'auto-mise à jour activée, et seulement après que la recherche échoue |
| `name` seul        | `claude plugin install`                      | Rien. Il lit les catalogues en cache sans actualiser                                                  |

L'actualisation avant une installation `name@marketplace` ne dépend pas du paramètre d'auto-mise à jour du marché ou de `DISABLE_AUTOUPDATER`.

Quand l'actualisation échoue, l'installation procède à partir du catalogue en cache et `claude plugin install` rapporte `marketplace not refreshed`.

Claude Code ignore l'actualisation avant une installation `name@marketplace` quand :

* Le marché a été ajouté à partir d'une source `file` ou `directory` locale, ou est défini en ligne dans les paramètres avec une [source `settings`](/docs/fr/settings-reference#extraknownmarketplaces)
* Un [répertoire de semence](/docs/fr/env-vars) fournit le marché
* Claude Code a actualisé le marché dans les 30 dernières secondes
* Vous avez défini `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* [Les paramètres gérés](/docs/fr/plugins/org#restrict-what-users-can-install) bloquent le marché, auquel cas Claude Code refuse aussi l'installation

<h3 id="when-auto-update-runs">
  Quand l'auto-mise à jour s'exécute
</h3>

Dans une session interactive, après que vous ayez envoyé votre premier message, Claude Code attend un délai aléatoire jusqu'à dix minutes. Il actualise ensuite chaque marché avec l'auto-mise à jour activée et met à jour les plugins installés à partir d'eux sur disque.

La session en cours garde les versions qu'elle a chargées, et vous voyez `Plugin updated: <name> · Run /reload-plugins to apply`. Que vous rechargiez ou non, les nouvelles versions se chargent au prochain lancement.

<h4 id="which-marketplaces-and-plugins-auto-update">
  Quels marchés et plugins se mettent à jour automatiquement
</h4>

Qu'un marché se mette à jour automatiquement suit le premier de ceux-ci qui est défini :

1. **`autoUpdate` sur son entrée `extraKnownMarketplaces`** dans un fichier de paramètres
2. **`autoUpdate` sur son entrée `known_marketplaces.json`**, que le bouton bascule **Enable auto-update** sous `/plugin` **Marketplaces** écrit. Quand un fichier de paramètres déclare aussi le marché sous `extraKnownMarketplaces`, le bouton bascule écrit `autoUpdate` à cette entrée de paramètres aussi
3. **La valeur par défaut** : activée pour les marchés officiels d'Anthropic comme `claude-plugins-official`, désactivée pour `knowledge-work-plugins` et `first-party-plugins`, activée pour [les marchés ajoutés à partir de claude.ai](/docs/fr/plugins/install#add-from-claude-ai), et désactivée pour tous les autres marchés

Si vous définissez `DISABLE_UPDATES=1`, `DISABLE_AUTOUPDATER=1`, ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, le passage entier est désactivé et le bouton bascule **Enable auto-update** est caché, sauf si vous définissez aussi `FORCE_AUTOUPDATE_PLUGINS=1`. La [référence des variables d'environnement](/docs/fr/env-vars) couvre l'effet plus large de chaque variable.

L'auto-mise à jour ignore aussi un plugin dont l'entrée de marché déclare un `headersHelper`. [Les installations et mises à jour qui refusent une commande au lieu de demander](/docs/fr/plugins/host-marketplace#installs-and-updates-that-refuse-the-command-instead-of-asking) expliquent quand un tel plugin apparaît dans l'onglet **Errors** de `/plugin` et comment vous le mettez à jour à partir de là.

Quand un plugin copié se met à jour en milieu de session, les commandes de hook, les moniteurs, les serveurs MCP et les serveurs LSP continuent d'utiliser le chemin de la version précédente. Exécutez `/reload-plugins` pour basculer les hooks, les serveurs MCP et les serveurs LSP vers le nouveau chemin. Les moniteurs nécessitent un redémarrage de session.

<h3 id="when-a-command-source-re-runs">
  Quand une source de commande se réexécute
</h3>

Les plugins avec une source `command` n'attendent pas le [passage d'auto-mise à jour](#when-auto-update-runs). Le répertoire imprimé reflète l'état de l'outil au moment où la commande s'est exécutée, donc Claude Code exécute la [commande que vous avez acceptée](/docs/fr/plugins/host-marketplace#change-the-command-of-a-command-source) à nouveau à ces moments :

* Chaque fois que vous installez ou mettez à jour le plugin
* Une fois par session pour chaque plugin activé de source de commande, en arrière-plan, peu après le démarrage de la session. Cette exécution ne dépend pas du paramètre d'auto-mise à jour du marché ou de `DISABLE_AUTOUPDATER`
* Au démarrage ou sur `/reload-plugins`, quand la version installée d'un plugin activé manque du cache de plugin

Claude Code ignore les deux exécutions en arrière-plan quand vous définissez [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars). Les installations et mises à jour explicites exécutent toujours la commande avec cette variable définie.

Quand la sortie hachée de la commande a changé, Claude Code installe le résultat comme une nouvelle version et la recharge dans la session interactive en cours, basculant [les mêmes composants que `/reload-plugins` bascule](/docs/fr/plugins/cli-reference#reload-plugins). Vous voyez une notification que le plugin a été rechargé.

Si recharger en place invaliderait le cache d'invite de la session, Claude Code vous invite plutôt à exécuter `/reload-plugins`, qui [avertit du coût du cache et s'applique quand réexécuté avec `--force`](/docs/fr/prompt-caching#enabling-or-disabling-a-plugin).

<h2 id="name-conflicts">
  Conflits de noms
</h2>

Quand les plugins activés de différentes origines partagent un nom de manifeste, cet ordre décide lequel se charge, de la plus haute à la plus basse précédence :

1. Un plugin dont l'id apparaît dans les paramètres gérés `enabledPlugins`, comme `true` ou `false`. Une copie `--plugin-dir` dont le nom de manifeste correspond à la partie nom de l'id n'est pas chargée, et vous voyez `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`
2. Un plugin `--plugin-dir`, `--plugin-url`, ou `CLAUDE_CODE_PLUGIN_DIRS` activé. Il remplace un plugin de marché installé ou de répertoire de compétences du même nom :
   * **Un plugin de marché installé** : remplacé silencieusement. `claude plugin list` affiche toujours la ligne du marché comme activée, car cette ligne reflète vos paramètres. Seul le journal que Claude Code écrit sous `~/.claude/debug/` quand vous démarrez avec `--debug` enregistre `Plugin "<name>" from --plugin-dir overrides installed version`
   * **Un plugin de répertoire de compétences** : remplacé par une ligne d'onglet **Errors** de `/plugin` qui lit `Not loaded — the name "<name>" is already taken by a session-only plugin (--plugin-dir / --plugin-url), which takes precedence`
3. Un plugin de marché installé. Un plugin de répertoire de compétences du même nom obtient la même ligne `Not loaded`, nommant le plugin installé
4. Un plugin de répertoire de compétences. Entre deux de ceux-ci, la copie sous `~/.claude/skills/` se charge et la copie `.claude/skills/` du projet est supprimée, avec une ligne qui dit quel chemin l'a masquée
5. Un plugin [synchronisé à partir de claude.ai](#synced-plugins). Quand un plugin activé de toute autre origine correspond à son nom, Claude Code charge ce plugin et rapporte la copie synchronisée comme non chargée. Pour utiliser la copie claude.ai à la place, désactivez votre propre copie

Parce que l'ordre compare les noms de manifeste, un plugin `--plugin-dir` nommé `hello-plugin` remplace `hello@example-marketplace` quand ce plugin's manifeste dit aussi `"name": "hello-plugin"`.

<h3 id="keep-a-session-only-plugin-from-loading">
  Empêcher un plugin de session uniquement de se charger
</h3>

Pour empêcher un plugin `--plugin-dir` de masquer quoi que ce soit, ou pour en désactiver un quand un processus parent passe l'indicateur pour vous, définissez son id à `false` dans n'importe quel fichier de paramètres. Pour un plugin dont le nom de manifeste est `hello-plugin`, l'entrée est `"enabledPlugins": {"hello-plugin@inline": false}`. Un plugin de session uniquement désactivé ne masque pas, donc la copie de marché ou de répertoire de compétences se charge à la place.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Installer et gérer les plugins](/docs/fr/plugins/install) : les étapes d'installation, d'activation, de désactivation et de mise à jour elles-mêmes
* [Dépanner les plugins](/docs/fr/plugins/troubleshooting) : les messages d'erreur par le stade qui les produit
* [Référence des commandes de plugin](/docs/fr/plugins/cli-reference) : les indicateurs et commandes nommés sur cette page
* [Gérer les plugins pour votre organisation](/docs/fr/plugins/org) : les paramètres gérés qui force-activent ou bloquent les plugins
