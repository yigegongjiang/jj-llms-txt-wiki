> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence des commandes de plugin

> Référence complète des commandes shell de plugin Claude, /plugin et /reload-plugins dans une session, et les drapeaux qui chargent un plugin pour une seule session.

Vous exécutez les commandes de plugin soit en tant que `claude plugin` depuis votre shell ou un script, soit en tant que `/plugin` et `/reload-plugins` dans une session Claude Code. Cette référence donne les drapeaux, les valeurs par défaut, la sortie et les codes de sortie de chaque commande, ainsi que les deux drapeaux qui chargent un plugin pour une seule session.

Exécutez `claude plugin --help` sur votre build pour confirmer quels sous-commandes votre version possède.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Installer et gérer les étapes, et où `/plugin` s'exécute** : voir [Installer et gérer les plugins](/docs/fr/plugins/install)
  * **Ce qu'une commande change sur le disque et quelle portée prend la priorité** : voir [Référence du chargement des plugins](/docs/fr/plugins/loading)
  * **Ce qu'un message d'erreur signifie** : voir [Dépanner les plugins](/docs/fr/plugins/troubleshooting)
</Note>

<h2 id="claude-plugin-commands">
  Commandes claude plugin
</h2>

Exécutez `claude plugin <subcommand>` depuis votre shell ou un script, en dehors d'une session Claude Code. Ces sous-commandes installent et gèrent les plugins sans ouvrir le panneau [`/plugin`](#plugin-in-a-session).

`claude plugins` est un alias pour `claude plugin`.

Chaque sous-commande partage ces codes de sortie, arguments de plugin et valeurs de portée :

* **Codes de sortie** : `0` en cas de succès et `1` en cas d'échec. `validate` ajoute la sortie `2` pour une erreur inattendue, et `eval` ajoute les codes listés dans [sa section](#plugin-eval).
* **Arguments de plugin** : un argument `<plugin>` est un `name` de plugin ou `name@marketplace`. Quand deux marketplaces offrent le même nom, utilisez la forme qualifiée.
* **Portées** : `--scope` prend `user`, `project`, ou `local`, et nomme le fichier de paramètres que la commande écrit. `update` prend aussi `managed`.

<h3 id="plugin-init">
  plugin init
</h3>

Créez un nouveau plugin à `~/.claude/skills/<name>/`. Il se charge dans votre prochaine session en tant que `<name>@skills-dir` sans étape d'installation.

`new` est un alias pour `init`.

Pour le flux de travail créer, tester et éditer qui commence par cette commande, voir [Créer un plugin](/docs/fr/plugins/create).

```bash theme={null}
claude plugin init <name> [options]
```

`<name>` devient le nom du répertoire sous `~/.claude/skills/` et le `name` du plugin dans son manifeste.

La commande n'a pas de drapeau pour un autre emplacement. Pour créer un scaffold dans un projet à la place, voir [Créer un plugin](/docs/fr/plugins/create).

| Drapeau                  | Description                                                                                                        |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------- |
| `--description <text>`   | Description du manifeste                                                                                           |
| `--author <name>`        | Nom de l'auteur. Par défaut `git config user.name`                                                                 |
| `--author-email <email>` | Email de l'auteur. Par défaut `git config user.email`                                                              |
| `--with <components...>` | Créez aussi des fichiers de démarrage pour `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style`, ou `channel` |
| `-f, --force`            | Écrasez un `.claude-plugin/` existant à la cible                                                                   |

Créez un plugin avec des fichiers de skill et hook de démarrage :

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code valide ce qu'il a écrit et affiche `Created plugin "my-helper" at ~/.claude/skills/my-helper`, suivi de l'id qu'il charge et de la commande `claude plugin disable` qui l'éteint.

Claude Code quitte avec `1` sans écrire quand il ne peut pas créer le scaffold en toute sécurité, et le message nomme la raison. Voici les raisons courantes :

* Une valeur `--with` inconnue
* Un scaffold existant à la cible sans `--force`
* Un paramètre géré qui bloque les plugins du répertoire de skills

<h3 id="plugin-install">
  plugin install
</h3>

Installez un plugin depuis une marketplace que vous avez ajoutée. `i` est un alias pour `install`.

```bash theme={null}
claude plugin install <plugin> [options]
```

La plupart des plugins s'installent sans invite. Pour un plugin dont l'entrée marketplace [exécute une commande pour l'installer](/docs/fr/plugins/host-marketplace) ou [définit un `headersHelper` pour son téléchargement](/docs/fr/plugins/host-marketplace#how-users-accept-a-headershelper-command), Claude Code affiche d'abord la commande et demande `Run this command now? [y/N]`.

| Drapeau                     | Description                                                                                                                                                                                                                                                                                                                                           |
| :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | Portée d'installation : `user`, `project`, ou `local`. Par défaut `user`                                                                                                                                                                                                                                                                              |
| `--config <key=value>`      | Définissez une option [`userConfig`](/docs/fr/plugins/manifest-reference) que le manifeste du plugin déclare. Répétez le drapeau pour chaque option. Nécessite Claude Code v2.1.147 ou ultérieur                                                                                                                                                           |
| `-y, --yes`                 | Acceptez la commande d'installation affichée sans l'invite `Run this command now?`. Ignoré quand la commande s'exécute dans une session Claude Code, comme depuis l'outil Bash ou un hook. Nécessite Claude Code v2.1.229 ou ultérieur                                                                                                                |
| `--accept-command <sha256>` | Acceptez la commande d'installation affichée dont le `sha256` une exécution [`--json` précédente](#plugin-json-result) a rapporté dans `shownCommand`, à la place de `-y`. Ne peut pas être combiné avec `-y`. Voir [Accepter une commande d'installation affichée](#accept-a-displayed-install-command). Nécessite Claude Code v2.1.271 ou ultérieur |
| `--json`                    | Affiche le résultat en tant qu'un objet JSON sur la dernière ligne de stdout au lieu du message lisible par l'homme, pour utilisation dans les scripts. Voir [Format de résultat JSON](#plugin-json-result). Nécessite Claude Code v2.1.268 ou ultérieur                                                                                              |

Passez `-y` depuis votre propre terminal pour accepter la commande affichée sans l'invite. Voici ce qui se passe sans TTY et quand Claude exécute la commande :

* **stdin ou stdout n'est pas un TTY, et vous ne passez ni `-y` ni `--accept-command`** : l'installation est refusée. La sortie dit que la commande a été seulement affichée, et le code de sortie est `1`
* **Claude exécute la commande via son outil Bash** : `-y` est ignoré. Exécutez la commande depuis votre propre terminal à la place

Installez un plugin pour tous ceux qui clonent le projet :

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code affiche `Successfully installed plugin: formatter@my-marketplace (scope: project)`. Quand rien de nouveau n'est installé, la sortie dit pourquoi :

* **Déjà installé à cette portée** : la sortie est `Plugin "formatter@my-marketplace" is already installed (scope: project)` et le code de sortie est `0`
* **Vous refusez une invite de source de commande** : la sortie est `Aborted.` et le code de sortie est `1`
* **Vous refusez une invite `headersHelper`, ou elle ne peut pas être confirmée sans TTY** : la sortie est `Aborted — the command was not run.` et le code de sortie est `1`

<h4 id="plugin-json-result">
  Format de résultat JSON
</h4>

Quand vous passez `--json` à `plugin install`, la dernière ligne de stdout est un objet JSON. Analysez seulement cette ligne, car Claude Code affiche toute commande que la marketplace déclare avant elle.

Trois champs sont toujours présents :

* `command` : la sous-commande qui a été exécutée, comme `install`
* `outcome` : `ok` ou `failed`
* `message` : une description lisible par l'homme du résultat

D'autres champs, comme `pluginId`, `scope`, et `failureCode`, apparaissent seulement quand ils s'appliquent.

L'option `--json` sur `plugin uninstall`, `plugin update`, `plugin enable`, et `plugin disable` affiche le même objet avec les champs propres à cette sous-commande.

Une erreur d'utilisation, comme un `--scope` invalide, n'affiche aucune ligne de résultat et quitte `1` avec la raison sur stderr.

<h4 id="accept-a-displayed-install-command">
  Accepter une commande d'installation affichée
</h4>

Quand une exécution `--json` affiche une commande déclarée par la marketplace et ne l'exécute pas, le résultat `failed` porte aussi un objet `shownCommand`. Ses champs incluent la commande telle qu'affichée, le plugin auquel elle appartient, et le `sha256` de la commande.

Pour accepter exactement cette commande, réexécutez avec ce `sha256` en tant que `--accept-command` depuis votre propre terminal, car le drapeau n'a aucun effet dans une session Claude Code. Nécessite Claude Code v2.1.271 ou ultérieur.

Le `sha256` compte comme acceptation pour exactement cette commande, ce plugin, et ce catalogue de marketplace. Si l'un d'eux a changé depuis que la commande a été affichée, Claude Code n'accepte pas le `sha256` et affiche la commande à nouveau. Un changement que l'actualisation de la marketplace de la propre exécution récupère compte aussi comme tel changement.

Si `shownCommand.acceptCommandMatched` est `false`, le `sha256` que vous avez passé ne correspond pas à la commande maintenant affichée. Examinez cette commande avant de réexécuter avec son `sha256`.

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Supprimez un plugin installé d'une portée. `remove` et `rm` sont des alias pour `uninstall`.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| Drapeau               | Description                                                                                                                                                                                                                                |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Désinstallez de la portée : `user`, `project`, ou `local`. Par défaut `user`                                                                                                                                                               |
| `--keep-data`         | Préservez le répertoire de données persistantes du plugin, `~/.claude/plugins/data/<id>/`                                                                                                                                                  |
| `--prune`             | Supprimez aussi les [dépendances](/docs/fr/plugins/dependencies) auto-installées que nul plugin restant n'a besoin                                                                                                                              |
| `-y, --yes`           | Ignorez l'invite de confirmation `--prune`. Requis avec `--prune` quand stdin ou stdout n'est pas un TTY                                                                                                                                   |
| `--json`              | Affiche le résultat en tant qu'un objet JSON sur la dernière ligne de stdout, dans le [même format que `plugin install --json`](#plugin-json-result). Ne peut pas être combiné avec `--prune`. Nécessite Claude Code v2.1.268 ou ultérieur |

Désinstallez un plugin de la portée du projet :

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code affiche `Successfully uninstalled plugin: formatter (scope: project)`. Quand le plugin n'est pas installé à cette portée, la commande affiche une ligne qui commence par `Failed to uninstall plugin "formatter@my-marketplace":` et quitte `1`.

<h3 id="plugin-enable">
  plugin enable
</h3>

Activez un plugin désactivé. Pour un [plugin synchronisé depuis claude.ai](/docs/fr/plugins/loading#synced-plugins), passez `<name>@synced` en tant que plugin.

```bash theme={null}
claude plugin enable <plugin> [options]
```

| Drapeau               | Description                                                                                                                                                                                       |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-s, --scope <scope>` | Portée pour activer : `user`, `project`, ou `local`. Auto-détecté quand omis                                                                                                                      |
| `--json`              | Affiche le résultat en tant qu'un objet JSON sur la dernière ligne de stdout, dans le [même format que `plugin install --json`](#plugin-json-result). Nécessite Claude Code v2.1.268 ou ultérieur |

Sans `--scope`, la commande vérifie vos fichiers de paramètres dans l'ordre local, projet, utilisateur, et utilise la première portée qui mentionne le plugin.

Si vous passez un `--scope` où le plugin n'est pas déclaré, la commande écrit soit une substitution soit échoue :

* **Une portée qui [prend la priorité](/docs/fr/plugins/loading) sur celle qui le déclare** : Claude Code écrit une substitution à la portée que vous avez passée. Par exemple, `claude plugin disable formatter --scope local` éteint un plugin activé au niveau du projet pour vous seul
* **Toute autre portée** : la commande échoue avec `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.`

Si le plugin est déjà activé à la portée résolue, la commande affiche `Plugin "formatter" is already enabled` et quitte `1`. Avec `--json`, le résultat a `"failureCode": "already_in_goal_state"` et `"alreadyInGoalState": true`, donc un script peut traiter ce cas comme un succès.

Quand le plugin déclare des [dépendances](/docs/fr/plugins/dependencies), Claude Code les active aussi. La commande échoue dans ces cas :

* **Une dépendance n'est pas installée** : l'activation échoue et affiche la commande `claude plugin install` pour chaque dépendance manquante
* **Une dépendance est bloquée par la politique de plugin de votre organisation** : l'activation échoue et nomme la dépendance bloquée
* **Une dépendance est définie à `false` à une portée avec une priorité plus élevée que la portée cible** : l'activation échoue. Activez la dépendance à cette portée, ou passez `--scope` pour écrire là

Réactivez un plugin où qu'il soit déclaré :

```bash theme={null}
claude plugin enable formatter
```

Claude Code affiche `Successfully enabled plugin: formatter (scope: project)`, nommant la portée qu'il a détectée.

<h3 id="plugin-disable">
  plugin disable
</h3>

Désactivez un plugin sans le désinstaller. Pour un [plugin synchronisé depuis claude.ai](/docs/fr/plugins/loading#synced-plugins), passez `<name>@synced` en tant que plugin.

```bash theme={null}
claude plugin disable [plugin] [options]
```

| Drapeau               | Description                                                                                                                                                                                       |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-a, --all`           | Désactivez tous les plugins activés. Ne peut pas être combiné avec un nom de plugin ou `--scope`                                                                                                  |
| `-s, --scope <scope>` | Portée pour désactiver : `user`, `project`, ou `local`. Auto-détecté quand omis                                                                                                                   |
| `--json`              | Affiche le résultat en tant qu'un objet JSON sur la dernière ligne de stdout, dans le [même format que `plugin install --json`](#plugin-json-result). Nécessite Claude Code v2.1.268 ou ultérieur |

Sans `--scope`, la portée est auto-détectée dans le même ordre local, projet, utilisateur que [`plugin enable`](#plugin-enable).

Si vous ne passez ni un nom de plugin ni `--all`, Claude Code affiche `Please specify a plugin name or use --all to disable all plugins` et quitte `1`. Désactiver un plugin qui est déjà désactivé affiche `Plugin "formatter" is already disabled` et quitte `1`, comme [`plugin enable`](#plugin-enable) le fait pour un plugin déjà activé.

La commande échoue pour un plugin qui est toujours requis :

* **Un autre plugin activé [en dépend](/docs/fr/plugins/dependencies)** : la commande échoue et nomme les dépendants à désactiver d'abord
* **Votre organisation l'exige en tant que plugin synchronisé** : la commande échoue et ne sauvegarde rien

Désactivez un plugin :

```bash theme={null}
claude plugin disable formatter
```

Claude Code affiche `Successfully disabled plugin: formatter (scope: project)`.

<h3 id="plugin-update">
  plugin update
</h3>

Mettez à jour un plugin à la dernière version que sa marketplace offre. La nouvelle version se charge dans votre prochaine session, ou après que vous exécutiez `/reload-plugins` dans une session en cours.

```bash theme={null}
claude plugin update <plugin> [options]
```

| Drapeau                     | Description                                                                                                                                                                                                                                                     |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | Portée à mettre à jour : `user`, `project`, `local`, ou `managed`. Par défaut la portée où le plugin est installé                                                                                                                                               |
| `-y, --yes`                 | Acceptez une commande d'installation modifiée d'un plugin [source de commande](/docs/fr/plugins/host-marketplace), sans l'invite. Requis quand stdin ou stdout n'est pas un TTY, sauf si vous passez `--accept-command`. Nécessite Claude Code v2.1.229 ou ultérieur |
| `--accept-command <sha256>` | Acceptez la commande déclarée par la marketplace dont le `sha256` une exécution [`--json` précédente](#plugin-json-result) a rapporté dans `shownCommand`, à la place de `-y`. Ne peut pas être combiné avec `-y`. Nécessite Claude Code v2.1.271 ou ultérieur  |
| `--json`                    | Affiche le résultat en tant qu'un objet JSON sur la dernière ligne de stdout, dans le [même format que `plugin install --json`](#plugin-json-result). Nécessite Claude Code v2.1.268 ou ultérieur                                                               |

`managed` est la seule portée que vous pouvez mettre à jour mais pas installer. Pour les plugins installés par l'administrateur, voir [Gérer les plugins pour votre organisation](/docs/fr/plugins/org).

Mettez à jour un plugin :

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code affiche `Checking for updates for plugin "formatter@my-marketplace"…`, puis le résultat. Quand rien n'est plus récent, il affiche `formatter is already at the latest version (1.0.0).` et quitte `0`.

Vous pouvez passer un nom de plugin nu, que la commande correspond à vos plugins installés. Quand les plugins installés de différentes marketplaces partagent le nom, la commande refuse la mise à jour et liste les commandes `plugin-name@marketplace-name` qualifiées à exécuter à la place. La mise à jour par nom nu nécessite Claude Code v2.1.246 ou ultérieur.

<h3 id="plugin-list">
  plugin list
</h3>

Listez les plugins installés avec leur version, portée et statut.

```bash theme={null}
claude plugin list [options]
```

| Drapeau       | Description                                                                                                        |
| :------------ | :----------------------------------------------------------------------------------------------------------------- |
| `--json`      | Affiche la liste en JSON                                                                                           |
| `--available` | Listez aussi les plugins que vos marketplaces offrent que vous n'avez pas installés. N'a aucun effet sans `--json` |

Claude Code groupe la sortie lisible par l'homme par comment chaque plugin se charge :

* **`Installed plugins:`** : plugins que vous avez installés depuis une marketplace
* **`Session-only plugins (--plugin-dir / --plugin-url):`** : plugins chargés par ces drapeaux dans la même commande, comme dans `claude --plugin-dir ./my-plugin plugin list`
* **`Skills-directory plugins (.claude/skills/*):`** : plugins que Claude Code a trouvés dans un répertoire de skills
* **`Synced from claude.ai`** : [plugins synchronisés depuis votre compte claude.ai](/docs/fr/plugins/loading#synced-plugins)

Sans rien dans aucun groupe, Claude Code affiche ``No plugins installed. Use `claude plugin install` to install a plugin.``

<h4 id="json-output">
  Sortie JSON
</h4>

Avec `--json`, Claude Code affiche un tableau avec un objet par installation. Chaque objet porte les champs ci-dessous. `id`, `version`, `scope`, `enabled`, et `installPath` sont toujours présents, et les autres apparaissent seulement quand ils s'appliquent.

| Champ          | Type             | Description                                                                                                                                                                                                                                                                                  |
| :------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`           | string           | `name@marketplace` pour les installations, `name@inline` pour les plugins de session seulement, `name@skills-dir` pour les plugins du répertoire de skills, `name@synced` pour les plugins synchronisés depuis claude.ai                                                                     |
| `version`      | string           | Pour une installation de marketplace, la [version que Claude Code a calculée](/docs/fr/plugins/loading#versions-and-updates) à l'installation. Pour un plugin de session seulement, du répertoire de skills, ou synchronisé, la `version` du manifeste, ou `unknown` quand il n'en déclare aucune |
| `scope`        | string           | `user`, `project`, `local`, ou `managed` pour les installations ; `user` ou `project` pour les plugins du répertoire de skills ; `session` pour les plugins de session seulement ; `synced` pour les plugins synchronisés depuis claude.ai                                                   |
| `enabled`      | boolean          | Si le plugin est activé dans vos paramètres fusionnés                                                                                                                                                                                                                                        |
| `installPath`  | string           | Répertoire d'où le plugin se charge                                                                                                                                                                                                                                                          |
| `installedAt`  | string           | Timestamp ISO de l'installation. Installations de marketplace seulement                                                                                                                                                                                                                      |
| `lastUpdated`  | string           | Timestamp ISO de la dernière mise à jour. Installations de marketplace seulement                                                                                                                                                                                                             |
| `projectPath`  | string           | Projet auquel l'installation appartient. Portée `project` et `local` seulement                                                                                                                                                                                                               |
| `mcpServers`   | object           | Les définitions de serveur MCP du plugin, quand un plugin installé de marketplace en a                                                                                                                                                                                                       |
| `errors`       | array of strings | Erreurs de chargement, quand le plugin n'a pas pu se charger                                                                                                                                                                                                                                 |
| `notes`        | array of strings | Avertissements de création pour un plugin qui s'est chargé et fonctionne                                                                                                                                                                                                                     |
| `errorDetails` | array of objects | Un objet par entrée `errors`, donnant son `type` de diagnostic et les noms auxquels il se réfère, comme le plugin, la marketplace, le serveur, ou le fichier. Nécessite Claude Code v2.1.268 ou ultérieur                                                                                    |
| `noteDetails`  | array of objects | Les mêmes objets de détail pour chaque entrée `notes`. Nécessite Claude Code v2.1.268 ou ultérieur                                                                                                                                                                                           |

Avec `--json --available`, Claude Code affiche un objet au lieu d'un tableau. Son champ `installed` contient le tableau d'objets de plugin installé, et son champ `available` contient un objet par plugin de marketplace non installé avec les champs ci-dessous.

| Champ             | Type             | Description                                                                                                                   |
| :---------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `pluginId`        | string           | `name@marketplace`                                                                                                            |
| `name`            | string           | Le nom du plugin dans la marketplace                                                                                          |
| `marketplaceName` | string           | La marketplace qui l'offre                                                                                                    |
| `source`          | string or object | La [source](/docs/fr/plugins/marketplace-reference) de l'entrée de marketplace : une string pour un chemin relatif, un objet sinon |
| `description`     | string           | La description de l'entrée, quand elle en a une                                                                               |
| `version`         | string           | La version de l'entrée, quand elle en déclare une                                                                             |
| `installCount`    | number           | Nombre d'installations, quand Claude Code en a un pour le plugin                                                              |

<h3 id="plugin-details">
  plugin details
</h3>

Montrez l'inventaire des composants d'un plugin et son coût de token projeté.

Le plugin doit être chargé : installé, trouvé dans un répertoire de skills, ou passé avec `--plugin-dir` ou `--plugin-url` dans la même commande. Le `<name>` est un `name` de plugin ou `name@marketplace`.

```bash theme={null}
claude plugin details <name>
```

La commande ne prend aucun drapeau au-delà de `--help`.

Montrez ce qu'un plugin installé contribue :

```bash theme={null}
claude plugin details formatter
```

Claude Code affiche le nom, la version, la description et la source du plugin, puis ces sections :

* **`Component inventory`** : les skills, agents, hooks, serveurs MCP et serveurs LSP du plugin
* **`Projected token cost`** : les tokens toujours actifs que le plugin ajoute à chaque session
* **`Per-component (rounded)`** : estimations toujours actives et à l'invocation pour chaque skill, agent et commande. Omis quand le plugin n'en a aucun

Pour ce que les deux chiffres de coût signifient, voir [Mesurer le coût et l'utilisation des plugins](/docs/fr/plugins/measure).

Pour un plugin qui n'est pas chargé, Claude Code affiche ``Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.`` et quitte `1`.

<h3 id="plugin-prune">
  plugin prune
</h3>

Supprimez les [dépendances](/docs/fr/plugins/dependencies) auto-installées que nul plugin installé n'a plus besoin. La commande ne supprime jamais un plugin que vous avez installé vous-même. `autoremove` est un alias pour `prune`.

```bash theme={null}
claude plugin prune [options]
```

| Drapeau               | Description                                                                     |
| :-------------------- | :------------------------------------------------------------------------------ |
| `-s, --scope <scope>` | Élaguer à la portée : `user`, `project`, ou `local`. Par défaut `user`          |
| `--dry-run`           | Listez ce qui serait supprimé sans le supprimer                                 |
| `-y, --yes`           | Ignorez l'invite de confirmation. Requis quand stdin ou stdout n'est pas un TTY |

Prévisualisez ce qu'un élagage supprimerait :

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code liste les dépendances orphelines et se termine par `(dry run — nothing removed)`. Sans rien à élaguer, il affiche une ligne qui commence par `Nothing to prune`.

Sans `--dry-run`, la commande supprime les dépendances orphelines seulement après que vous confirmiez à l'invite ou passiez `-y`.

Le code de sortie est `0` quelle que soit votre réponse à l'invite.

Ce que `prune` fait dépend de si un terminal est attaché et si vous passez `-y`:

| Terminal et drapeaux                 | Ce qui se passe                                                                                 |
| :----------------------------------- | :---------------------------------------------------------------------------------------------- |
| Terminal interactif, pas de `-y`     | Liste les dépendances orphelines et demande `Remove? [y/N]`                                     |
| N'importe quel terminal, `-y`        | Les supprime et affiche `Removed N auto-installed plugins: <names>`                             |
| stdin ou stdout non-TTY, pas de `-y` | Affiche la liste et ``Not a TTY — run `claude plugin prune -y` to remove.``, ne supprimant rien |

<h3 id="plugin-eval">
  plugin eval
</h3>

Exécutez les [cas d'évaluation](/docs/fr/plugin-evals) d'un plugin et rapportez les résultats notés. Nécessite Claude Code v2.1.269 ou ultérieur.

Chaque cas est une invite plus des évaluateurs. Claude Code l'exécute plusieurs fois dans une session isolée avec seulement le plugin cible chargé, et par défaut aussi sans le plugin pour que le rapport montre la différence.

Voir [Tester les plugins avec des évaluations](/docs/fr/plugin-evals) pour le format des cas, les évaluateurs, les résultats et l'utilisation CI.

```bash theme={null}
claude plugin eval [target] [options]
```

La `target` optionnelle par défaut au répertoire courant et prend n'importe laquelle de ces formes :

* Un répertoire de plugin
* Un seul fichier `prompt.md` ou `case.yaml`
* Un plugin installé en tant que `name` ou `name@marketplace`
* `name@skills-dir`

Mettez la cible avant `--tag`, `--allow-tools`, et `--json`. Chacune de ces options prend les mots qui la suivent comme sa valeur, donc une cible écrite après l'une d'elles est lue comme une balise, un nom d'outil, ou le chemin de sortie JSON au lieu de la cible.

Ce tableau liste les options que la plupart des exécutions utilisent. Exécutez `claude plugin eval --help` pour l'ensemble complet, incluant `--case`, `--tag`, `--output-dir`, `--report`, `--allow-real-servers`, `--keep-temp`, et `--verbose`.

| Option                     | Description                                                                                                                                                                                  | Par défaut                                                                                        |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| `--runs <n>`               | Exécutions par cas dans chaque [bras](/docs/fr/plugin-evals#compare-against-a-no-plugin-baseline)                                                                                                 | La `runs` de chaque cas, sinon 3                                                                  |
| `-j, --concurrency <n>`    | Sessions d'agent à exécuter à la fois, 1 à 8. Elles partagent votre limite de débit                                                                                                          | `1`                                                                                               |
| `--model <model>`          | Modèle pour l'agent testé                                                                                                                                                                    | La `model` de chaque cas, sinon `ANTHROPIC_MODEL` s'il est défini, sinon le défaut de Claude Code |
| `--judge-model <model>`    | Modèle pour les évaluateurs `llm` et `baseline`                                                                                                                                              | Un petit modèle rapide                                                                            |
| `--ablation <mode>`        | `none` ou `with-without`. Voir [Comparer à une ligne de base sans plugin](/docs/fr/plugin-evals#compare-against-a-no-plugin-baseline)                                                             | `with-without` quand un plugin se résout, sinon `none`                                            |
| `--threshold <0..1>`       | Quittez 1 si un cas note en dessous de ceci                                                                                                                                                  | `1.0`                                                                                             |
| `--max-cost-usd <usd>`     | Arrêtez avant la prochaine exécution une fois que les dépenses atteignent ceci, quittez 2, et rapportez les résultats partiels                                                               | Pas de limite                                                                                     |
| `--allow-tools <tools...>` | Accordez des outils au-delà de l'ensemble en lecture seule, comme `Bash`, `Write`, `Edit`, ou `"mcp__plugin_<plugin>_<server>__*"`. Voir [Accorder des outils](/docs/fr/plugin-evals#grant-tools) |                                                                                                   |
| `--scaffold`               | Exécutez le [`scaffold_script`](/docs/fr/plugin-evals#add-setup-or-history-with-case-yaml) de chaque cas                                                                                          | Désactivé                                                                                         |
| `--trust-plugin`           | Ignorez l'invite de confiance à la première exécution, pour CI. Voir [Ce qu'une exécution peut accéder](/docs/fr/plugin-evals#security)                                                           | Désactivé                                                                                         |
| `--mocks <mode>`           | `record` ou `off`. Voir [Serveurs MCP fictifs](/docs/fr/plugin-evals#mock-mcp-servers)                                                                                                            | `record`                                                                                          |
| `--eval-dir <dir>`         | Répertoire sous le plugin qui contient les cas                                                                                                                                               | La `experimental.evals` du manifeste, sinon `evals`                                               |
| `--json [path]`            | Affiche le [document de résultat](/docs/fr/plugin-evals#json-result) sur stdout, ou écrivez-le à un chemin `.json`                                                                                |                                                                                                   |
| `--no-publish`             | Gardez le rapport HTML local                                                                                                                                                                 |                                                                                                   |

Le code de sortie rapporte comment l'exécution s'est terminée. Pour agir dessus dans un pipeline, voir [Exécuter les évaluations en CI](/docs/fr/plugin-evals#run-evals-in-ci).

| Code de sortie | Signification                                                                        |
| :------------- | :----------------------------------------------------------------------------------- |
| `0`            | Chaque cas respecte le seuil                                                         |
| `1`            | Un cas défaillant, une erreur de chargement, ou un répertoire de plugin non approuvé |
| `2`            | Une exécution partielle                                                              |
| `130`          | Interrompu                                                                           |
| `143`          | Terminé                                                                              |

<h3 id="plugin-eval-init">
  plugin eval init
</h3>

Créez une suite d'évaluation pour le plugin dans le répertoire courant. Nécessite Claude Code v2.1.269 ou ultérieur. Voir [Créer votre première suite d'évaluation](/docs/fr/plugin-evals#create-your-first-eval-suite).

```bash theme={null}
claude plugin eval init [name] [options]
```

Dans un terminal, la commande ouvre une session Claude Code interactive pour une interview de création. Dans l'interview, Claude fait ce qui suit :

1. Lit le plugin
2. Vous demande ce qu'il devrait bien faire
3. Propose des cas et des évaluateurs
4. Écrit les fichiers de cas
5. Exécute les cas et examine les notes avec vous pour vérifier que les évaluateurs notent comme vous le feriez

Avec `--bare`, ou sans terminal, la commande écrit un modèle de cas unique vierge à la place. Quand Claude exécute la commande depuis l'intérieur d'une session Claude Code, la commande affiche les instructions d'interview pour que cette session suive plutôt que d'écrire un modèle.

Le `name` optionnel est un nom de cas. Il est requis avec `--bare` ou sans terminal, car la commande écrit le modèle vierge pour ce cas. L'interview n'en a pas besoin.

La commande accepte ces options :

| Option              | Description                                                                                          | Par défaut                                          |
| :------------------ | :--------------------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| `--bare`            | Écrivez un `prompt.md` et `graders/criteria.md` vierges pour `<name>` au lieu d'exécuter l'interview |                                                     |
| `-i, --interactive` | Exigez l'interview. Échoue sans terminal au lieu d'écrire un modèle                                  |                                                     |
| `--eval-dir <dir>`  | Répertoire sous le répertoire courant pour écrire les cas                                            | La `experimental.evals` du manifeste, sinon `evals` |

<h3 id="plugin-tag">
  plugin tag
</h3>

Créez une balise git annotée nommée `<name>--v<version>` pour une version de plugin. Avant de baliser, la commande vérifie que le `plugin.json` du plugin et toute entrée de marketplace qui le liste s'accordent sur la version.

Pour quand baliser une version, voir [Publier un plugin](/docs/fr/plugins/publish).

```bash theme={null}
claude plugin tag [path] [options]
```

Le `[path]` est le répertoire du plugin, par défaut le répertoire courant. La commande trouve l'entrée de marketplace en remontant de ce répertoire à un `.claude-plugin/marketplace.json` qui liste le plugin.

| Drapeau               | Description                                                                               |
| :-------------------- | :---------------------------------------------------------------------------------------- |
| `--push`              | Poussez la balise vers `--remote` après l'avoir créée                                     |
| `--dry-run`           | Affiche ce qui serait balisé sans créer la balise                                         |
| `-f, --force`         | Ignorez les vérifications d'arbre de travail sale et de balise existante                  |
| `-m, --message <msg>` | Message d'annotation de balise. `%s` représente la version. Par défaut `<name> <version>` |
| `--remote <name>`     | Distant vers lequel pousser avec `--push`. Par défaut `origin`                            |

Prévisualisez la balise pour un plugin dans un checkout de marketplace :

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code affiche le plan :

* Le nom du plugin
* La version et quel fichier elle provient
* L'entrée de marketplace correspondante, quand il y en a une
* Le nom de la balise
* Les commandes `git tag` et `git push` qu'il exécuterait

Sans `--dry-run`, Claude Code affiche `Created tag formatter--v1.0.0` et soit `Pushed to origin` soit la commande push à exécuter vous-même. Si la poussée échoue, la balise est toujours créée localement et la commande quitte avec une erreur.

La commande quitte `1` et affiche la raison quand elle ne peut pas baliser en toute sécurité. Les raisons courantes sont :

* Pas de `version` dans `plugin.json` ou l'entrée de marketplace
* La balise existe déjà
* L'arbre de travail est sale

<h3 id="plugin-validate">
  plugin validate
</h3>

Validez un manifeste de plugin, un manifeste de marketplace, ou les skills, agents et commandes dans un répertoire, et quittez avec un code qu'un travail CI peut utiliser. Pour le flux de travail créer, tester et éditer, voir [Créer un plugin](/docs/fr/plugins/create). Pour ce que le validateur vérifie dans chaque manifeste, voir la [référence du manifeste de plugin](/docs/fr/plugins/manifest-reference) et la [référence de marketplace](/docs/fr/plugins/marketplace-reference).

```bash theme={null}
claude plugin validate <path> [options]
```

| Drapeau    | Description                                                                                                                                                                                      |
| :--------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--strict` | Traitez les avertissements comme des erreurs, donc les champs non reconnus et les métadonnées manquantes que le runtime tolère échouent l'exécution. Nécessite Claude Code v2.1.145 ou ultérieur |
| `--json`   | Sortez le rapport de validation en tant qu'un objet JSON avec les mêmes codes de sortie. Nécessite Claude Code v2.1.259 ou ultérieur                                                             |

Validez un plugin avant de le valider :

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h4 id="validate-a-directory">
  Validez un répertoire
</h4>

Le `<path>` est un fichier manifeste ou un répertoire. Donné un répertoire, Claude Code choisit ce qu'il faut valider par ce qu'il y trouve :

* `.claude-plugin/marketplace.json`, quand il existe
* Sinon `.claude-plugin/plugin.json`
* Sinon les fichiers de composant, choisis par le nom du répertoire. Valider les fichiers de composant sans manifeste nécessite Claude Code v2.1.233 ou ultérieur :
  * Un répertoire nommé `skills`, `agents`, ou `commands` : les fichiers à l'intérieur
  * Un répertoire nommé `.claude` : les répertoires `skills`, `agents`, et `commands` à l'intérieur
  * N'importe quel autre répertoire : ces trois répertoires sous son `.claude`

Claude Code ne suit pas les liens symboliques à l'intérieur du répertoire que vous nommez. Ce qu'il fait dépend de l'endroit où le lien est :

* **Un répertoire `skills`, `agents`, ou `commands` lié sous la racine du plugin ou `.claude`** : Claude Code avertit que rien dedans n'a été lu.
* **Une entrée liée à l'intérieur d'un répertoire `skills`, `agents`, ou `commands`** : Claude Code la saute et avertit, par répertoire, combien d'entrées il a sautées qu'une session chargerait.
* **Le répertoire `skills`, `agents`, ou `commands` que vous nommez est lui-même un lien symbolique, ou son répertoire parent `.claude` est** : Claude Code rapporte une erreur et ne vérifie rien dedans. Nommez le répertoire réel à la place.

Quelques fichiers ne sont pas lus par une exécution de validation :

* **Un `SKILL.md` à la racine du plugin** : quand vous exécutez `claude plugin validate` contre un répertoire de plugin, Claude Code ne vérifie pas un `SKILL.md` à la racine du plugin
* **Un `CLAUDE.md` à la racine du plugin** : dans une exécution de plugin, Claude Code avertit aussi d'un `CLAUDE.md` à la racine du plugin
* **Fichiers de plugin dans une exécution de marketplace** : depuis un répertoire de marketplace, Claude Code n'ouvre pas les fichiers de skill, agent, commande ou hook des plugins. Pour trouver des erreurs dans ces fichiers, validez chaque répertoire de plugin

<h4 id="output-and-exit-codes">
  Sortie et codes de sortie
</h4>

Claude Code affiche le fichier qu'il a validé, toute erreur et avertissement avec leurs chemins, et une ligne de verdict. Le code de sortie suit le verdict :

| Code de sortie | Ligne de verdict                                                                | Signification                                                          |
| :------------- | :------------------------------------------------------------------------------ | :--------------------------------------------------------------------- |
| `0`            | `Validation passed` ou `Validation passed with warnings`                        | Le manifeste se charge. Avec `--strict`, pas d'avertissements non plus |
| `1`            | `Validation failed` ou `Validation failed (--strict treats warnings as errors)` | Une erreur, ou un avertissement sous `--strict`                        |
| `2`            | `Unexpected error during validation: <reason>`                                  | Le validateur lui-même a échoué, comme sur un chemin illisible         |

Avec `--json`, Claude Code écrit le rapport sur stdout en tant qu'un objet JSON avec ces champs de niveau supérieur :

* `success` : le même verdict que le code de sortie donne
* `strict` : si l'exécution a traité les avertissements comme des erreurs
* `target` : le chemin résolu que Claude Code a validé
* `manifest` : le résultat du manifeste lui-même, ou `null` pour une exécution sans manifeste
* `contents` : résultats par fichier, chacun nommant son `file` et portant des tableaux `errors`, `warnings`, et `notes`

À la sortie `2`, la commande n'écrit rien sur stdout. Le message d'erreur va sur stderr.

<h2 id="claude-plugin-marketplace-commands">
  Commandes claude plugin marketplace
</h2>

Exécutez `claude plugin marketplace <subcommand>` depuis votre shell pour ajouter, lister, actualiser et supprimer les marketplaces d'où vous installez les plugins.

* **Codes de sortie** : ces sous-commandes suivent la [convention de code de sortie](#claude-plugin-commands) des commandes de plugin
* **Portées** : leur drapeau `--scope` n'a pas de forme courte `-s`

Pour ce qu'est une marketplace et comment Claude Code la met en cache, voir [Référence du chargement des plugins](/docs/fr/plugins/loading).

<h3 id="plugin-marketplace-add">
  plugin marketplace add
</h3>

Ajoutez une marketplace depuis un référentiel GitHub, une URL git, une `marketplace.json` hébergée, ou un chemin local, et déclarez-la dans un fichier de paramètres.

Après l'avoir ajoutée, Claude Code installe toute [dépendance](/docs/fr/plugins/dependencies) que vos plugins installés manquaient.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| Drapeau               | Description                                                                                                                                                                        |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>`     | Fichier de paramètres pour déclarer la marketplace : `user`, `project`, ou `local`. Par défaut `user`                                                                              |
| `--sparse <paths...>` | Limitez le checkout git à ces répertoires, pour les monorepos. Sources `github` et `git` seulement                                                                                 |
| `--claudeai`          | Lisez l'argument comme le nom d'une [marketplace hébergée sur claude.ai](/docs/fr/plugins/install#add-from-claude-ai) au lieu d'une source. Nécessite Claude Code v2.1.273 ou ultérieur |

`<source>` prend n'importe laquelle des formes du tableau ci-dessous, et sa forme décide du type de source et comment Claude Code récupère la marketplace. Pour l'objet source résultant, voir la [référence de marketplace](/docs/fr/plugins/marketplace-reference).

| Vous tapez                                                                                        | Type de source | Comment Claude Code la récupère                                                                                                  |
| :------------------------------------------------------------------------------------------------ | :------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `owner/repo`, `owner/repo#ref`, ou `owner/repo@ref`                                               | `github`       | Clone le référentiel GitHub, épinglé à `ref` quand donné. Le propriétaire et le repo doivent suivre les règles de nommage GitHub |
| `user@host:path[.git][#ref]`                                                                      | `git`          | Clone sur SSH                                                                                                                    |
| `https://example.com/repo.git[#ref]`, ou une URL contenant `/_git/`                               | `git`          | Clone sur HTTPS, incluant les URLs Azure DevOps                                                                                  |
| `https://github.com/owner/repo` ou `https://gitlab.com/namespace/project`                         | `git`          | Clone sur HTTPS après avoir ajouté `.git`                                                                                        |
| N'importe quelle autre URL `http://` ou `https://`, incluant un hôte git auto-hébergé sans `.git` | `url`          | Récupère l'URL en tant que `marketplace.json`. Pour cloner un référentiel là à la place, ajoutez `.git`                          |
| `./path`, `../path`, `/path`, ou `~/path` vers un répertoire                                      | `directory`    | Lit le répertoire en place. Sur Windows, les formes `.\`, `..\`, et `C:\` fonctionnent aussi                                     |
| Les mêmes formes de chemin, vers un fichier `.json`                                               | `file`         | Lit le fichier en place                                                                                                          |

Pour un hôte dont les URLs de clone ne portent pas le suffixe `.git`, comme AWS CodeCommit, ajoutez la marketplace en tant qu'entrée git dans [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces) à la place. Claude Code clone une entrée git qu'elle se termine ou non par `.git`.

Claude Code clone aussi une URL `gitlab.com` avec des sous-groupes imbriqués, comme `https://gitlab.com/group/subgroup/project`.

Ajoutez une marketplace et partagez-la avec le projet :

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code affiche `Successfully added marketplace: your-marketplace (declared in project settings)`, utilisant le `name` du manifeste de la marketplace lui-même. Un ajout répété ou une source invalide affiche l'un de ces résultats à la place :

* **Marketplace déjà sur le disque** : la sortie est `Marketplace 'your-marketplace' already on disk — declared in project settings` et le code de sortie est `0`
* **Source non reconnue** : la sortie est `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` et le code de sortie est `1`
* **Hôte nu comme `gitlab.example.com/team/plugins`** : l'ajout échoue en tant que raccourci `owner/repo` invalide, et le message vous dit d'ajouter `https://` ou d'utiliser un chemin local

Ajoutez une [marketplace hébergée sur claude.ai](/docs/fr/plugins/install#add-from-claude-ai) par le nom affiché dans la section `From claude.ai:` de `claude plugin marketplace list`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Avec `--claudeai`, la commande refuse `--scope` et `--sparse`. La marketplace est hébergée pour votre compte, pas déclarée dans un fichier de paramètres, donc vous ne pouvez pas la partager via le `.claude/settings.json` d'un projet.

<h3 id="plugin-marketplace-list">
  plugin marketplace list
</h3>

Listez chaque marketplace que vous avez ajoutée, avec sa source.

```bash theme={null}
claude plugin marketplace list [options]
```

| Drapeau  | Description              |
| :------- | :----------------------- |
| `--json` | Affiche la liste en JSON |

Claude Code affiche `Configured marketplaces:` et une ligne `Source:` par marketplace, ou `No marketplaces configured`.

Avec `--json`, Claude Code affiche un tableau avec un objet par marketplace, portant les champs ci-dessous. Chaque champ est une string.

| Champ             | Description                                                                        |
| :---------------- | :--------------------------------------------------------------------------------- |
| `name`            | Le nom de la marketplace                                                           |
| `source`          | `github`, `git`, `url`, `directory`, `file`, ou `claudeai`                         |
| `repo`            | `owner/repo`. Sources `github` seulement                                           |
| `url`             | L'URL de clone ou de récupération. Sources `git` et `url` seulement                |
| `path`            | Le chemin local. Sources `directory` et `file` seulement                           |
| `ref`             | La branche ou balise épinglée. Sources `github` et `git`, seulement quand épinglée |
| `installLocation` | Où Claude Code a mis en cache la marketplace                                       |

Une [marketplace claude.ai](/docs/fr/plugins/install#add-from-claude-ai) ajoutée n'a pas de clone local, donc son entrée porte ses identifiants claude.ai, `marketplaceId` et `organizationUuid`, à la place de `installLocation`. Elle porte aussi `scope` quand un est enregistré, et `status`.

Si vos sessions de terminal [synchronisent les plugins depuis votre compte claude.ai](/docs/fr/plugins/loading#synced-plugins), la liste de texte se termine par une section `From claude.ai:`. Cette section nomme les marketplaces que claude.ai liste pour votre compte que vous n'avez pas ajoutées, à la fois basées sur git et hébergées. Elle nécessite Claude Code v2.1.273 ou ultérieur.

Pour ajouter une marketplace de cette section, voir [Ajouter une marketplace depuis claude.ai](/docs/fr/plugins/install#add-from-claude-ai).

La sortie `--json` couvre seulement les marketplaces configurées et laisse la section dehors.

<h3 id="plugin-marketplace-remove">
  plugin marketplace remove
</h3>

Supprimez la déclaration d'une marketplace de vos paramètres. `rm` est un alias pour `remove`.

<Warning>
  Quand vous supprimez une marketplace de la dernière portée qui la déclare, Claude Code supprime aussi son cache et désinstalle chaque plugin que vous avez installé depuis elle. Sans `--scope`, la commande supprime la déclaration de chaque portée. Pour actualiser une marketplace sans perdre ses plugins, exécutez `plugin marketplace update` à la place.
</Warning>

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

Le `<name>` est le nom de la marketplace que `plugin marketplace list` affiche, pas la source que vous avez passée à `add`.

| Drapeau           | Description                                                                                                                                          |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>` | Supprimez la déclaration d'une portée de paramètres : `user`, `project`, ou `local`. Sans elle, Claude Code supprime la déclaration de chaque portée |

Supprimez une marketplace de chaque portée :

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code affiche `Successfully removed marketplace: your-marketplace`, ajoutant `(from project settings)` quand vous l'avez scoped. Si vous scoped à un fichier de paramètres qui ne déclare pas la marketplace, la commande échoue avec `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.`

<h3 id="plugin-marketplace-update">
  plugin marketplace update
</h3>

Actualisez une marketplace, ou chaque marketplace, depuis sa source pour récupérer les nouveaux plugins et versions. Une marketplace ajoutée avec une branche ou une balise `ref` s'actualise au dernier commit de cette ref, pas la branche par défaut du référentiel.

```bash theme={null}
claude plugin marketplace update [name]
```

La commande ne prend aucun drapeau au-delà de `--help`.

Actualisez une marketplace :

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code affiche `Successfully updated marketplace: your-marketplace`. Quand vous omettez le nom, il affiche un compte comme `Successfully updated 2 marketplaces`. Sans marketplaces ajoutées, il affiche `No marketplaces configured` et quitte `0`.

<h2 id="plugin-in-a-session">
  /plugin dans une session
</h2>

À l'intérieur d'une session interactive, `/plugin` ouvre le panneau de plugin. Chaque sous-commande ouvre le panneau sur un onglet, exécute une action là, ou affiche un résultat en ligne. `/plugins` et `/marketplace` sont des alias pour `/plugin`.

Vous pouvez exécuter ces commandes seulement dans une session de terminal interactive. Dans une exécution non-interactive comme `claude -p`, Claude Code répond que `/plugin` n'est pas disponible dans cet environnement.

Pour quelles surfaces ont `/plugin`, comment installer sans elle, et ce que chaque onglet du panneau affiche, voir [Installer et gérer les plugins](/docs/fr/plugins/install).

Un `<plugin>` est un `name` de plugin ou `name@marketplace`.

Le tableau ci-dessous liste chaque forme de session. Les sous-commandes shell `init`, `update`, `details`, `prune`, `eval`, et `eval init` n'ont pas de forme de session.

| Commande                                            | Alias                                          | Ce qu'elle fait                                                                                                                                                                                                                                                                                                               |
| :-------------------------------------------------- | :--------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/plugin`                                           |                                                | Ouvre le panneau sur l'onglet **Discover**. N'importe quel premier mot non reconnu après `/plugin` fait la même chose                                                                                                                                                                                                         |
| `/plugin help`                                      | `/plugin --help`, `/plugin -h`                 | Affiche la liste d'utilisation des sous-commandes `/plugin`                                                                                                                                                                                                                                                                   |
| `/plugin list [--enabled\|--disabled]`              | `ls`                                           | Affiche vos plugins installés de marketplace en ligne, avec version, portée et statut. Un drapeau de filtre affiche seulement cet état. Un plugin dont l'état d'activation n'a pas encore été appliqué est marqué `— run /reload-plugins to apply`. Nécessite Claude Code v2.1.163 ou ultérieur                               |
| `/plugin install`                                   | `i`                                            | Ouvre l'onglet **Discover**                                                                                                                                                                                                                                                                                                   |
| `/plugin install <plugin>`                          | `i`                                            | Ouvre les détails du plugin dans l'onglet **Discover**. Avec `name@marketplace`, les ouvre dans la liste de cette marketplace                                                                                                                                                                                                 |
| `/plugin install <plugin> --marketplace <source>`   | `i`                                            | Ajoute la marketplace à `<source>` quand vous ne l'avez pas encore ajoutée, vous demandant de confirmer d'abord, puis ouvre les détails du plugin. Voir [Ajouter une marketplace et installer en une commande](/docs/fr/plugins/install#add-a-marketplace-and-install-in-one-command). Nécessite Claude Code v2.1.275 ou ultérieur |
| `/plugin manage`                                    |                                                | Ouvre l'onglet **Installed**                                                                                                                                                                                                                                                                                                  |
| `/plugin stats`                                     |                                                | Ouvre l'onglet **Stats**, dans les sessions où [`/skill-doctor`](/docs/fr/skills#find-unused-skills) est disponible. N'importe où ailleurs il ouvre le panneau sur l'onglet **Discover**                                                                                                                                           |
| `/plugin enable <plugin>`                           |                                                | Ouvre l'onglet **Installed** au plugin et l'active                                                                                                                                                                                                                                                                            |
| `/plugin disable <plugin>`                          |                                                | Ouvre l'onglet **Installed** au plugin et le désactive                                                                                                                                                                                                                                                                        |
| `/plugin uninstall <plugin>`                        |                                                | Ouvre l'onglet **Installed** au plugin et le désinstalle                                                                                                                                                                                                                                                                      |
| `/plugin configure <plugin>`                        | `config`                                       | Ouvre la boîte de dialogue [`userConfig`](/docs/fr/plugins/manifest-reference) du plugin, ou rapporte que le plugin n'en déclare aucune. Nécessite Claude Code v2.1.147 ou ultérieur                                                                                                                                               |
| `/plugin validate <path>`                           |                                                | Affiche le même rapport que `claude plugin validate`, en ligne                                                                                                                                                                                                                                                                |
| `/plugin tag [path] [--push] [--dry-run] [--force]` |                                                | Crée la balise de version comme `claude plugin tag` le fait. Accepte `--push`, `--dry-run`, et `--force` ou `-f` ; avec n'importe quel autre drapeau ou un argument supplémentaire, Claude Code affiche l'utilisation à la place                                                                                              |
| `/plugin marketplace`                               | `market`                                       | Ne fait rien de visible. Passez `add`, `list`, `update`, ou `remove`                                                                                                                                                                                                                                                          |
| `/plugin marketplace add [source]`                  | `market add`                                   | Avec une source, l'ajoute et rapporte le résultat. Sans une, ouvre l'entrée **Add marketplace**                                                                                                                                                                                                                               |
| `/plugin marketplace list`                          | `market list`                                  | Affiche vos noms de marketplace en ligne                                                                                                                                                                                                                                                                                      |
| `/plugin marketplace update [name]`                 | `market update`                                | Ouvre l'onglet **Marketplaces**. Avec un nom, actualise cette marketplace là                                                                                                                                                                                                                                                  |
| `/plugin marketplace remove [name]`                 | `market remove`, `market rm`, `marketplace rm` | Ouvre l'onglet **Marketplaces**. Avec un nom, supprime cette marketplace là                                                                                                                                                                                                                                                   |

Si vous nommez un plugin qui n'est pas installé dans le projet courant dans `/plugin enable`, `disable`, `uninstall`, ou `configure`, Claude Code affiche `Plugin "<plugin>" is not installed in this project` au lieu d'agir.

<h2 id="reload-plugins">
  /reload-plugins
</h2>

Appliquez les modifications de plugins en attente à la session en cours sans la redémarrer. Les modifications en attente sont les plugins que vous avez installés, mis à jour, activés, désactivés ou modifiés sur le disque depuis le démarrage de la session.

Lorsque vous fermez le panneau `/plugin` avec des modifications en attente que vous y avez apportées, Claude Code exécute `/reload-plugins` pour vous. Exécutez-le vous-même après les modifications de plugins qui se produisent en dehors du panneau, comme une commande `claude plugin` que vous avez exécutée dans un autre terminal.

```text theme={null}
/reload-plugins [--force]
```

| Flag      | Description                                                                                               |
| :-------- | :-------------------------------------------------------------------------------------------------------- |
| `--force` | Appliquez le rechargement même s'il invaliderait le cache de prompt. `force` sans tirets fonctionne aussi |

<h3 id="reload-summary">
  Résumé du rechargement
</h3>

Claude Code recharge chaque plugin actif et affiche une ligne de résumé, `Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers`, en omettant le nombre de serveurs MCP de plugins dans une session sans terminal interactif. Lorsqu'un plugin a échoué, le résumé ajoute `N errors during load. Run /plugin for details.`

Le nombre de skills couvre chaque skill qu'un plugin fournit, à la fois ses entrées `commands/` et ses skills `SKILL.md`. Le nombre d'agents est le nombre d'agents chargés dans la session, y compris ceux qui ne proviennent pas de plugins.

Lorsque les [dépendances](/docs/fr/plugins/dependencies) d'un plugin rechargé sont manquantes, Claude Code les installe, recharge à nouveau, et ajoute `(+ N dependencies: <names>) resolved` au résumé.

<h3 id="reloads-that-change-mcp-tools">
  Rechargements qui modifient les outils MCP
</h3>

Lorsque le rechargement ajouterait ou supprimerait un serveur MCP de plugin ou l'outil `LSP`, et que ce changement invaliderait le [cache de prompt](/docs/fr/prompt-caching#enabling-or-disabling-a-plugin), Claude Code n'applique pas le rechargement. Il affiche une ligne telle que `This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.` Passez `--force` pour l'appliquer quand même.

<h3 id="sessions-without-an-interactive-terminal">
  Sessions sans terminal interactif
</h3>

`/reload-plugins` s'exécute également dans les sessions sans terminal interactif, comme l'application de bureau, le SDK Agent, et le [mode non-interactif](/docs/fr/headless) avec `-p`. Nécessite Claude Code v2.1.260 ou ultérieur.

Dans ces sessions, la commande s'exécute uniquement lorsque vous la tapez vous-même dans la session, comme dans le prompt `-p` ou la boîte de prompt de l'application de bureau. Lorsqu'elle arrive d'une autre manière, comme via [Remote Control](/docs/fr/remote-control) ou un message relayé depuis Slack, la commande répond `/reload-plugins isn't available over a remote connection in this session.` et ne recharge rien.

Le rechargement dans ces sessions ne connecte ni ne déconnecte les serveurs MCP de plugins. Ces modifications prennent effet dans votre prochaine session.

<h2 id="flags-that-load-a-plugin-for-one-session">
  Drapeaux qui chargent un plugin pour une seule session
</h2>

Deux drapeaux `claude` chargent un plugin pour une seule session seulement, sans l'installer. Les deux sont répétables.

Les auteurs de plugin les utilisent pour tester un plugin avant de le publier. Pour le flux de travail charger-éditer-recharger, voir [Développer sans marketplace](/docs/fr/plugins/create#develop-without-a-marketplace).

| Drapeau               | Description                                                                                                                                                                                               | Exemple                                                                     |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `--plugin-dir <path>` | Chargez un plugin depuis un répertoire ou une archive `.zip` de celui-ci. Un dossier de plugins charge chaque dossier enfant qui contient un `.claude-plugin/plugin.json`. Chaque drapeau prend un chemin | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip`                  |
| `--plugin-url <url>`  | Récupérez une archive `.zip` de plugin depuis une URL. Répétez le drapeau, ou passez plusieurs URLs séparées par des espaces dans une valeur entre guillemets                                             | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

Un plugin que l'un ou l'autre drapeau charge est un plugin de session seulement. `claude plugin list` l'affiche en tant que `<name>@inline` avec portée `session`, mais seulement quand le même drapeau précède la sous-commande. Par exemple, exécutez `claude --plugin-dir ./my-plugin plugin list`.

Quand un plugin de session seulement partage un nom avec un plugin installé, Claude Code charge la copie de session seulement pour cette session et saute celle installée. La copie installée se charge à la place si vous avez désactivé la copie de session seulement avec `claude plugin disable <name>@inline`, ou si les paramètres gérés verrouillent ce nom de plugin. Pour la priorité, voir [Référence du chargement des plugins](/docs/fr/plugins/loading).

Un administrateur peut rejeter les deux drapeaux, et les dossiers nommés dans la variable [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/fr/env-vars#variables), avec le paramètre géré [`disableSideloadFlags`](/docs/fr/settings-reference#disablesideloadflags). Claude Code affiche alors que le drapeau est désactivé par les paramètres gérés de votre organisation et quitte `1` sans démarrer.

Depuis le SDK Agent, l'option [`plugins`](/docs/fr/agent-sdk/plugins) est l'équivalent de `--plugin-dir`.

<h2 id="next-steps">
  Prochaines étapes
</h2>

* [Installer et gérer les plugins](/docs/fr/plugins/install) : les mêmes opérations que les étapes, avec ce que vous voyez à chacune
* [Référence du chargement des plugins](/docs/fr/plugins/loading) : ce que chaque commande change sur le disque et quelle portée prend effet
* [Dépanner les plugins](/docs/fr/plugins/troubleshooting) : installer, marketplace, charger, et valider les messages d'erreur avec leurs corrections
* [Référence du manifeste de plugin](/docs/fr/plugins/manifest-reference) : les champs que `claude plugin validate` vérifie
