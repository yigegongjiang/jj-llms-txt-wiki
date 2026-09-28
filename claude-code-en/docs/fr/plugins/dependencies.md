> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dépendances des plugins

> Déclarez les plugins dont votre plugin dépend, avec des plages de versions telles que ^1.2, et découvrez comment Claude Code installe, résout et élague les dépendances.

Une dépendance de plugin est un autre plugin sur lequel votre plugin s'appuie, par exemple celui dont il appelle le serveur MCP ou la compétence. Chaque dépendance suit la dernière version que sa place de marché fournit, sauf si vous déclarez une contrainte de version, une plage de version sémantique telle que `^2.0` ou `~2.1.0` que vous avez testée.

Cette page s'adresse aux auteurs de plugins qui déclarent des dépendances dans `plugin.json` et aux responsables de la place de marché qui balisent les versions.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Installation d'un plugin qui a des dépendances** : voir [Gérer les plugins installés](/docs/fr/plugins/install#manage-installed-plugins)
  * **Lecture d'une erreur de dépendance** : voir [Erreurs de dépendance](/docs/fr/plugins/troubleshooting#dependency-errors)
  * **Déclaration des packages npm et Bun dont le code de votre plugin a besoin** : voir [Dépendances des packages Node.js](/docs/fr/plugins/loading#node-js-package-dependencies)
</Note>

Pour ajouter une contrainte, commencez par [Déclarer une dépendance avec une contrainte de version](#declare-a-dependency-with-a-version-constraint). Si vous maintenez un plugin dont d'autres dépendent, [balisez vos versions](#tag-plugin-releases-for-version-resolution) pour que leurs contraintes puissent se résoudre.

<h2 id="declare-dependencies">
  Déclarer les dépendances
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />Sans contrainte de version, une dépendance se déplace vers chaque nouvelle version que sa place de marché publie la prochaine fois que les utilisateurs mettent à jour. Si cette version renomme un outil MCP que votre plugin appelle, votre plugin se casse pour tous ceux qui mettent à jour.

Avec une contrainte telle que `~2.1.0` sur une dépendance provenant d'une source sauvegardée par git, les utilisateurs qui ont votre plugin installé continuent à recevoir les correctifs `2.1.x` de la dépendance et ne passent jamais à `2.2`. Pour mettre à niveau selon votre propre calendrier, testez contre une version plus récente, puis publiez une nouvelle version de votre plugin avec une contrainte plus large.

<h3 id="declare-a-dependency-with-a-version-constraint">
  Déclarer une dépendance avec une contrainte de version
</h3>

Listez les dépendances dans le tableau `dependencies` du fichier `.claude-plugin/plugin.json` de votre plugin. Le manifeste suivant déclare une dépendance sans version et une dépendance avec contrainte :

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

Une entrée peut être une chaîne : le nom du plugin seul, tel que `"audit-logger"` dans ce manifeste, ou `"name@marketplace"` pour le résoudre dans une autre place de marché. Avec une simple chaîne, votre plugin dépend de la version que la place de marché de ce plugin fournit.

Pour définir une contrainte de version, utilisez un objet avec ces champs, chacun étant une chaîne :

| Champ         | Description                                                                                                                                                                                                                                                                                                                      |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Le nom du plugin de dépendance, tel qu'il apparaît dans son entrée de place de marché. Claude Code le recherche dans la même place de marché que le plugin déclarant, sauf si vous définissez `marketplace`. Obligatoire.                                                                                                        |
| `version`     | Une [plage de version sémantique](https://github.com/npm/node-semver#ranges) telle que `~2.1.0`, `^2.0`, `>=1.4`, ou `=2.1.0`. La dépendance s'installe à la balise git la plus élevée qui satisfait cette plage, donc le responsable de la dépendance doit [baliser les versions](#tag-plugin-releases-for-version-resolution). |
| `marketplace` | Une place de marché différente pour résoudre `name` dedans. Une liste d'autorisation contrôle les dépendances inter-places de marché, décrites dans [Dépendre d'un plugin d'une autre place de marché](#depend-on-a-plugin-from-another-marketplace).                                                                            |

Une plage ne correspond pas aux versions de pré-version telles que `2.0.0-beta.1` sauf si vous acceptez avec un suffixe de pré-version tel que `^2.0.0-0`.

<h3 id="bundle-plugins-for-a-team">
  Regrouper les plugins pour une équipe
</h3>

Pour permettre aux ingénieurs d'installer un ensemble curé de plugins avec une seule commande, publiez un plugin dont le manifeste contient un `name` et un tableau `dependencies`. Un manifeste de plugin n'a besoin que de `name`, c'est donc un plugin valide, et l'installer installe chaque dépendance.

Par exemple, une équipe de plateforme peut publier des bundles spécifiques aux rôles dans une place de marché interne afin que les ingénieurs exécutent une seule `claude plugin install` au lieu d'installer chaque plugin séparément :

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

Pour ajouter un plugin à l'ensemble standard ultérieurement, publiez une nouvelle version `backend-standard` avec la dépendance supplémentaire. Lorsque la place de marché ne [met pas à jour automatiquement par défaut](/docs/fr/plugins/loading#which-marketplaces-and-plugins-auto-update), les ingénieurs activent soit la mise à jour automatique pour la place de marché, soit mettent à jour manuellement :

* **Activer la mise à jour automatique pour la place de marché** : la prochaine mise à jour automatique déplace le bundle vers la nouvelle version et installe toutes les dépendances qu'il ajoute.
* **Mettre à jour manuellement** : exécutez `claude plugin update backend-standard` dans un shell, puis `/reload-plugins` dans une session ouverte pour installer les dépendances nouvellement ajoutées.

Pour les étapes côté ingénieur, voir [Garder les plugins à jour](/docs/fr/plugins/install#keep-plugins-updated).

Pour déployer un bundle à tous les membres d'une organisation, un administrateur l'ajoute à `enabledPlugins` dans les paramètres gérés. Voir [Pré-installer et exiger des plugins](/docs/fr/plugins/org#pre-install-and-require-plugins).

<h3 id="depend-on-a-plugin-from-another-marketplace">
  Dépendre d'un plugin d'une autre place de marché
</h3>

Par défaut, Claude Code n'installe pas une dépendance d'une place de marché différente de celle du plugin déclarant, sauf si l'utilisateur a déjà cette dépendance installée et activée au même niveau. Cette valeur par défaut empêche une place de marché d'installer silencieusement des plugins d'une source que l'utilisateur n'a pas examinée.

Pour autoriser l'installation, ajoutez le nom de la place de marché cible à `allowCrossMarketplaceDependenciesOn` dans le `marketplace.json` de la place de marché racine. La place de marché racine est celle qui héberge le plugin que l'utilisateur installe. Seule la liste d'autorisation de la place de marché racine s'applique.

Le `marketplace.json` suivant permet à `deploy-kit` de dépendre d'un plugin de `your-shared-marketplace` :

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

Si `allowCrossMarketplaceDependenciesOn` est manquant ou n'inclut pas la place de marché cible, Claude Code n'installe pas la dépendance. Lorsque la dépendance est déclarée dans l'entrée de la place de marché, l'installation elle-même est refusée avec un message qui commence par `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` et nomme le champ à définir. Lorsqu'elle est déclarée dans `plugin.json`, l'installation se termine sans la dépendance et votre plugin échoue alors à charger.

La vérification de la liste d'autorisation ne s'applique pas à une dépendance qui est déjà activée. Si un utilisateur installe d'abord `audit-logger` de `your-shared-marketplace` lui-même, au même niveau, `deploy-kit` s'installe alors sans aucune modification de la liste d'autorisation.

<h3 id="test-a-plugin-and-its-dependency-locally">
  Tester un plugin et sa dépendance localement
</h3>

Si vous développez un plugin et le plugin dont il dépend en même temps, démarrez Claude Code à partir de votre shell et chargez les deux avec [`--plugin-dir`](/docs/fr/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) :

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

La copie locale de la dépendance satisfait l'entrée de dépendance de votre plugin, vous n'avez donc pas besoin d'installer la dépendance à partir de sa place de marché.

* **Pas de `version` nécessaire** : le `plugin.json` local n'a pas besoin non plus d'une `version`, car une [contrainte de version](#declare-a-dependency-with-a-version-constraint) n'est pas vérifiée par rapport à une copie locale.
* **Entrées qui nomment une place de marché** : une entrée qui nomme une place de marché correspond également à la copie locale sur Claude Code v2.1.242 ou ultérieur.

Jusqu'à ce que vous installiez la dépendance à partir de sa place de marché, votre plugin cesse de charger chaque fois que la copie locale est désactivée ou absente :

* **Vous avez désactivé la copie locale** : votre plugin est désactivé au prochain chargement de plugin, avec une erreur qui se termine par `is disabled — enable it or remove the dependency`. Lorsque l'erreur nomme la dépendance comme `<name>@inline`, cet identifiant fait référence à la copie `--plugin-dir`.
* **Vous avez démarré une session sans le drapeau `--plugin-dir` de la dépendance** : l'erreur signale que la dépendance n'est pas installée. Passez le drapeau à nouveau, ou installez la dépendance à partir de sa place de marché.

Lorsque les deux plugins se trouvent dans un dossier parent, vous pouvez passer ce dossier à `--plugin-dir` une seule fois. Si le dossier n'est pas lui-même un plugin, Claude Code charge chaque dossier enfant qui a un `.claude-plugin/plugin.json`. Nécessite Claude Code v2.1.265 ou ultérieur.

<h2 id="tag-plugin-releases-for-version-resolution">
  Publier un plugin dont d'autres dépendent
</h2>

Si vous maintenez un plugin dont d'autres plugins dépendent avec une contrainte de version, balisez ses versions pour que ces contraintes puissent se résoudre. Une contrainte se résout par rapport aux balises git du référentiel qui héberge le plugin. Balisez le référentiel vers lequel la [source du plugin](/docs/fr/plugins/marketplace-reference#plugin-sources) du plugin dans `marketplace.json` pointe :

* **Source `github`, `url`, ou `git-subdir`** : le référentiel du plugin lui-même, donc l'auteur du plugin crée les balises
* **Chemin relatif tel que `./plugins/secrets-vault`** : le référentiel de la place de marché, donc le responsable de la place de marché crée les balises

<h3 id="create-a-release-tag">
  Créer une balise de version
</h3>

Balisez chaque version comme `<plugin-name>--v<version>`, où `<version>` correspond au champ `version` dans le `plugin.json` de ce commit. Le préfixe plugin-name permet à un référentiel de place de marché d'héberger plusieurs plugins avec des historiques de version indépendants.

Créez la balise à partir du répertoire du plugin, avec une télécommande `origin` configurée pour recevoir la balise poussée, en utilisant [`claude plugin tag`](/docs/fr/plugins/cli-reference#plugin-tag) :

```bash theme={null}
claude plugin tag --push
```

La commande construit le nom de la balise à partir du manifeste du plugin. Avant de créer la balise, elle exécute ces vérifications :

* Valide le plugin
* Vérifie que `plugin.json` et l'entrée de la place de marché s'accordent sur la version, lorsque le répertoire du plugin se trouve dans un checkout de place de marché
* Nécessite un arbre de travail propre sous le répertoire du plugin
* Refuse si la balise existe déjà

Une exécution réussie imprime `Created tag secrets-vault--v2.1.0`. Avec `--push`, elle imprime également `Pushed to origin`. Sans `--push`, elle imprime la commande `git push` à exécuter vous-même.

Passez `--dry-run` pour voir le plan sans rien créer.

La [référence `claude plugin tag`](/docs/fr/plugins/cli-reference#plugin-tag) liste les drapeaux restants.

Vous pouvez également exécuter `git tag secrets-vault--v2.1.0` directement, tant que vous gardez la `version` dans `plugin.json` et dans l'entrée de la place de marché synchronisées vous-même.

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  Contraindre une dépendance qui a une source non-git
</h3>

La résolution basée sur les balises s'applique uniquement aux sources sauvegardées par git. Pour une dépendance avec une source de plugin `npm`, `archive`, ou `command` [plugin source](/docs/fr/plugins/marketplace-reference#plugin-sources), la contrainte ne contrôle pas quelle version est récupérée. Elle est toujours vérifiée lorsque le plugin charge, et le plugin dépendant est désactivé si la version installée ne la satisfait pas.

Pour les sources `npm`, `archive`, et `command`, la version vérifiée est la `version` dans le `plugin.json` de la dépendance. Définissez-en une là avant de contraindre cette dépendance, car un `plugin.json` qui ne définit pas de version ne satisfait aucune contrainte.

Claude Code n'installe jamais une dépendance avec une source `command` lui-même, donc les utilisateurs [l'installent d'abord](/docs/fr/plugins/marketplace-reference#command-plugin-source). Il n'exécute jamais non plus le [`headersHelper`](/docs/fr/plugins/host-marketplace#authenticate-archive-downloads) d'une dépendance, donc les utilisateurs installent également une dépendance dont l'entrée de la place de marché en définit une avant d'installer votre plugin.

En plus de `claude plugin install`, ces opérations installent également toute dépendance déclarée manquante, et les limites `command` et `headersHelper` s'appliquent à elles aussi :

* `/reload-plugins`
* Mise à jour automatique de la place de marché du plugin dépendant
* Réexécution de `claude plugin install` sur le plugin dépendant
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  Comment les dépendances se comportent pour vos utilisateurs
</h2>

Ces sections décrivent comment Claude Code résout, vérifie et combine les contraintes que vous déclarez une fois que votre plugin est installé aux côtés d'autres.

<h3 id="how-a-constraint-resolves-against-tags">
  Comment une contrainte se résout par rapport aux balises
</h3>

Lorsqu'un utilisateur installe un plugin qui déclare `{ "name": "secrets-vault", "version": "~2.1.0" }`, la dépendance s'installe à partir de la balise `secrets-vault--v` la plus élevée qui satisfait `~2.1.0` sur le référentiel qui héberge `secrets-vault`. Lorsqu'aucune balise ne satisfait la plage, l'installation échoue ou utilise la copie actuelle de la place de marché :

* **Plugin avec son propre référentiel** : l'installation échoue avec un message contenant `Dependency "secrets-vault@your-marketplace" has no git tag satisfying`.
* **Plugin référencé par un chemin relatif** : l'installation utilise la copie actuelle de la place de marché à la place, et la contrainte est vérifiée lorsque le plugin charge. Si cette copie est en dehors de la plage, le plugin dépendant reste désactivé et `claude plugin list` affiche `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`.

Pour un plugin que la place de marché référence par un chemin relatif, une place de marché que vous avez ajoutée comme chemin de dossier local résout également les contraintes par rapport aux balises git de ce dossier, lorsque le dossier est un référentiel git. Cela nécessite Claude Code v2.1.196 ou ultérieur. Un dossier local qui n'est pas un référentiel git n'a pas de balises, donc Claude Code installe la dépendance à partir du contenu actuel du dossier à la place.

<h3 id="confirm-the-resolved-version">
  Confirmer la version résolue
</h3>

Pour confirmer quelle version une contrainte s'est résolue, exécutez `claude plugin list` dans votre shell. Une dépendance résolue par balise affiche sa version avec un suffixe de commit de 12 caractères, tel que `2.1.0-8713c5b11005`.

Les vérifications de contrainte utilisent la version de la balise plutôt que la `version` dans `plugin.json`, même si `plugin.json` à ce commit est en retard.

Si vous forcez le déplacement d'une balise vers un commit différent, la prochaine installation récupère le contenu de ce commit au lieu de réutiliser une copie en cache obsolète. Voir [Versions et mises à jour](/docs/fr/plugins/loading#versions-and-updates) pour savoir comment la version d'un plugin devient sa clé de cache.

<h3 id="combine-constraints-from-several-plugins">
  Combiner les contraintes de plusieurs plugins
</h3>

Lorsque plusieurs plugins installés contraignent la même dépendance, la dépendance se résout à la version la plus élevée qui satisfait toutes leurs plages. Les combinaisons courantes se résolvent comme ceci :

| Plugin A nécessite | Plugin B nécessite | Résultat                                                                                                                                          |
| :----------------- | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `^2.0`             | `>=2.1`            | Une installation à la balise `2.x` la plus élevée à ou au-dessus de `2.1.0`. Les deux plugins se chargent.                                        |
| `~2.1`             | `~3.0`             | L'installation du plugin B échoue avec un message `has conflicting version requirements`. Le plugin A et la dépendance restent comme ils étaient. |
| `=2.1.0`           | aucun              | La dépendance reste à `2.1.0`. La mise à jour automatique ignore les versions plus récentes tant que le plugin A est installé.                    |

La mise à jour automatique récupère une dépendance contrainte à la balise git la plus élevée qui satisfait la plage de chaque plugin installé, plutôt qu'à la dernière version de la place de marché. Si les plages des plugins installés ne se chevauchent pas, la mise à jour automatique laisse cette dépendance à sa version actuelle, et l'onglet **Erreurs** de `/plugin` affiche une entrée nommant le plugin contraignant. S'ils se chevauchent mais qu'aucune balise ne tombe dans la plage, la mise à jour automatique récupère la copie actuelle de la place de marché et ignore la mise à jour lorsque la `version` de cette copie tombe en dehors de la plage de tout plugin installé.

Lorsqu'un utilisateur désinstalle le dernier plugin qui contraint une dépendance, la dépendance n'est plus contrainte à une plage de version et reprend le suivi de son entrée de place de marché à la prochaine mise à jour.

<h2 id="see-also">
  Voir aussi
</h2>

* [`claude plugin prune`](/docs/fr/plugins/cli-reference#plugin-prune) : supprimer les dépendances auto-installées dont aucun plugin n'a plus besoin
* [Héberger une place de marché](/docs/fr/plugins/host-marketplace) : canaux de version et recommandation d'autres plugins
