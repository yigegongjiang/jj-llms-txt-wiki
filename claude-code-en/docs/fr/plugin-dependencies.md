> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Contraindre les versions des dépendances de plugin

> Déclarez des contraintes de version sur les dépendances de plugin, et regroupez un ensemble de plugins organisé derrière une seule installation.

Un plugin peut dépendre d'autres plugins en les listant dans `plugin.json` ou dans son entrée marketplace. Par défaut, une dépendance suit la dernière version disponible, donc une version en amont peut modifier la dépendance sans avertissement. Les contraintes de version vous permettent de maintenir une dépendance à une plage de version testée jusqu'à ce que vous décidiez de la mettre à jour.

Lorsque vous installez un plugin qui déclare des dépendances, Claude Code les résout et les installe automatiquement et liste les dépendances qui ont été ajoutées à la fin de la sortie d'installation. Si une dépendance disparaît ultérieurement, `/reload-plugins` et la mise à jour automatique des plugins en arrière-plan la réinstalleront, à condition que son marketplace soit déjà dans vos marketplaces configurés. Réexécuter `claude plugin install` sur le plugin dépendant, ou ajouter un marketplace avec `claude plugin marketplace add`, résout également toute dépendance manquante en attente. Les dépendances d'un marketplace que vous n'avez pas ajouté restent non résolues.

Ce guide est destiné aux auteurs de plugins qui déclarent des dépendances dans `plugin.json` et aux responsables de marketplace qui balisent les versions. Pour installer des plugins qui ont des dépendances, consultez [Découvrir et installer des plugins](/docs/fr/discover-plugins). Pour le schéma de manifeste complet, consultez la [Référence des plugins](/docs/fr/plugins-reference).

<h2 id="why-constrain-dependency-versions">
  Pourquoi contraindre les versions des dépendances
</h2>

Considérez un marketplace interne où deux équipes publient des plugins. L'équipe plateforme maintient `secrets-vault`, un serveur MCP qui encapsule un backend de secrets. L'équipe de déploiement maintient `deploy-kit`, qui appelle `secrets-vault` pour récupérer les identifiants lors des déploiements.

`deploy-kit` est testé contre `secrets-vault` v2.1.0. Sans contrainte de version, la prochaine fois que l'équipe plateforme balisera une version qui renomme un outil MCP, la mise à jour automatique déplacera chaque `secrets-vault` de l'ingénieur vers la nouvelle version et `deploy-kit` se cassera.

Avec une contrainte de version, `deploy-kit` déclare qu'il a besoin de `secrets-vault` dans la plage `~2.1.0`. Les ingénieurs avec `deploy-kit` installé restent sur le correctif `2.1.x` le plus élevé correspondant. L'équipe de déploiement effectue la mise à niveau selon son propre calendrier en publiant une nouvelle version de `deploy-kit` avec une contrainte plus large.

<h2 id="declare-a-dependency-with-a-version-constraint">
  Déclarer une dépendance avec une contrainte de version
</h2>

Listez les dépendances dans le tableau `dependencies` du `plugin.json` de votre plugin. Chaque entrée est soit un nom de plugin, soit un objet avec une contrainte de version.

Le manifeste suivant déclare une dépendance sans version et une dépendance contrainte :

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

Une entrée peut être une simple chaîne avec uniquement le nom du plugin, comme `"audit-logger"` dans l'exemple ci-dessus, qui dépend de la version que le marketplace de ce plugin fournit. Pour plus de contrôle, utilisez un objet avec ces champs :

| Champ         | Type   | Description                                                                                                                                                                                                                                                                             |
| :------------ | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | string | Nom du plugin. Se résout dans le même marketplace que le plugin déclarant. Obligatoire.                                                                                                                                                                                                 |
| `version`     | string | Une [plage semver](https://github.com/npm/node-semver#ranges) telle que `~2.1.0`, `^2.0`, `>=1.4`, ou `=2.1.0`. La dépendance est récupérée à la version balisée la plus élevée qui satisfait cette plage.                                                                              |
| `marketplace` | string | Un marketplace différent pour résoudre `name` dans. Les dépendances inter-marketplace sont bloquées sauf si le marketplace cible est listé dans [`allowCrossMarketplaceDependenciesOn`](#depend-on-a-plugin-from-another-marketplace) dans le `marketplace.json` du marketplace racine. |

Le champ `version` accepte toute expression supportée par le package `semver` de Node, y compris les plages caret, tilde, hyphen et comparator. Les versions de pré-version telles que `2.0.0-beta.1` sont exclues sauf si votre plage opte pour un suffixe de pré-version comme `^2.0.0-0`.

<h2 id="bundle-plugins-for-a-team">
  Regrouper les plugins pour une équipe
</h2>

En plus du `name` obligatoire, un manifeste de plugin peut se composer uniquement d'un tableau `dependencies`. Son installation récupère chaque dépendance, ce qui en fait un moyen de regrouper un ensemble de plugins curatisé derrière une seule installation.

Par exemple, une équipe de plateforme peut publier des bundles spécifiques à un rôle dans une marketplace interne afin que les ingénieurs exécutent une seule commande `claude plugin install` au lieu d'installer chaque outil séparément :

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

L'installation de `backend-standard` résout et installe les quatre dépendances.

Pour ajouter un outil à l'ensemble standard ultérieurement, publiez une nouvelle version de `backend-standard` avec la dépendance supplémentaire. La mise à jour automatique est désactivée par défaut pour les marketplaces non-Anthropic, donc les ingénieurs récupèrent la nouvelle version de l'une des deux façons suivantes :

* Activez la mise à jour automatique pour la marketplace dans `/plugin`. La prochaine mise à jour automatique déplace le bundle vers la nouvelle version et installe les dépendances qu'il ajoute.
* Exécutez `claude plugin update backend-standard`, puis `/reload-plugins` pour installer les dépendances nouvellement ajoutées.

Pour déployer les bundles dans toute une organisation, ajoutez le plugin bundle à `enabledPlugins` dans les [paramètres gérés](/docs/fr/settings#enabledplugins).

<h2 id="depend-on-a-plugin-from-another-marketplace">
  Dépendre d'un plugin d'un autre marketplace
</h2>

Par défaut, Claude Code refuse d'installer automatiquement une dépendance qui se trouve dans un marketplace différent de celui du plugin qui la déclare. Cela empêche un marketplace de silencieusement extraire des plugins d'une source que vous n'avez pas examinée.

Pour l'autoriser, le responsable du marketplace racine ajoute le nom du marketplace cible à `allowCrossMarketplaceDependenciesOn` dans `marketplace.json`. Le marketplace racine est celui qui héberge le plugin que l'utilisateur installe ; seule sa liste d'autorisation est consultée, donc la confiance ne s'enchaîne pas à travers les marketplaces intermédiaires.

Le `marketplace.json` suivant permet à `deploy-kit` de dépendre d'un plugin de `acme-shared` :

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "allowCrossMarketplaceDependenciesOn": ["acme-shared"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "acme-shared" }
      ]
    }
  ]
}
```

Si le champ est manquant ou n'inclut pas le marketplace cible, l'installation échoue avec une erreur `cross-marketplace` nommant le champ à définir. Les utilisateurs peuvent toujours installer la dépendance manuellement en premier, ce qui satisfait la contrainte sans modifier la liste d'autorisation.

<h2 id="tag-plugin-releases-for-version-resolution">
  Baliser les versions de plugin pour la résolution de version
</h2>

Les contraintes de version se résolvent par rapport aux balises git sur le référentiel du marketplace. Pour que Claude Code trouve les versions disponibles d'une dépendance, les versions du plugin en amont doivent être balisées en utilisant une convention de nommage spécifique.

Balisez chaque version comme `{plugin-name}--v{version}`, où `{version}` correspond au champ `version` dans le `plugin.json` de ce commit. À partir du répertoire du plugin, exécutez :

```bash theme={null}
claude plugin tag --push
```

La commande `claude plugin tag` dérive le nom de la balise à partir du manifeste du plugin et de l'entrée du marketplace qui l'entoure. Avant de créer la balise, elle valide le contenu du plugin, vérifie que `plugin.json` et l'entrée du marketplace s'accordent sur la version, exige un arbre de travail propre sous le répertoire du plugin, et refuse si la balise existe déjà. Ajoutez `--dry-run` pour voir ce qui serait balisé sans le créer. L'exécution directe de `git tag secrets-vault--v2.1.0` est équivalente si vous maintenez `plugin.json` et l'entrée du marketplace en synchronisation vous-même.

Le préfixe du nom du plugin permet à un référentiel de marketplace d'héberger plusieurs plugins avec des lignes de version indépendantes. Le séparateur `--v` est analysé comme une correspondance de préfixe sur le nom complet du plugin, donc les noms de plugins qui contiennent des traits d'union sont gérés correctement.

Lorsque vous installez un plugin qui déclare `{ "name": "secrets-vault", "version": "~2.1.0" }`, Claude Code liste les balises du marketplace, filtre celles commençant par `secrets-vault--v`, et récupère la version la plus élevée satisfaisant `~2.1.0`. Si aucune balise correspondante n'existe, le plugin dépendant est désactivé avec une erreur listant les versions disponibles.

Un marketplace ajouté comme chemin de dossier local résout les balises de la même manière lorsque le dossier est un référentiel git. Cela nécessite Claude Code v2.1.196 ou version ultérieure. Dans deux cas, Claude Code installe la dépendance à partir du contenu actuel du dossier à la place :

* Les versions antérieures ne lisent pas les balises à partir d'un marketplace de dossier local, donc une dépendance contrainte se charge uniquement si cette copie satisfait la plage.
* Un dossier local qui n'est pas un référentiel git n'a pas de balises, quelle que soit la version.

Le semver de la balise résolue est enregistré séparément de la `version` du `plugin.json`, donc les vérifications de contrainte utilisent la balise qui a été réellement récupérée même si la `version` du `plugin.json` à ce commit a une valeur obsolète. Le nom du répertoire de cache pour une installation résolue par balise inclut un suffixe SHA de commit de 12 caractères, donc si un responsable force-déplace une balise vers un commit différent, l'installation suivante obtient un répertoire de cache frais au lieu de réutiliser du contenu obsolète.

<Note>
  Pour les sources de marketplace `npm`, la contrainte ne contrôle pas quelle version est récupérée, car la résolution basée sur les balises s'applique uniquement aux sources soutenues par git. La contrainte est toujours vérifiée au moment du chargement, et le plugin dépendant est désactivé avec `dependency-version-unsatisfied` si la version installée ne la satisfait pas.
</Note>

<h2 id="how-constraints-interact">
  Comment les contraintes interagissent
</h2>

Lorsque plusieurs plugins installés contraignent la même dépendance, Claude Code intersecte leurs plages et résout la dépendance à la version la plus élevée qui satisfait tous les critères. Le tableau ci-dessous montre comment les combinaisons courantes se résolvent.

| Plugin A nécessite | Plugin B nécessite | Résultat                                                                                                                       |
| :----------------- | :----------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `^2.0`             | `>=2.1`            | Une installation à la balise `2.x` la plus élevée à ou au-dessus de `2.1.0`. Les deux plugins se chargent.                     |
| `~2.1`             | `~3.0`             | L'installation du plugin B échoue avec `range-conflict`. Le plugin A et la dépendance restent comme ils étaient.               |
| `=2.1.0`           | aucun              | La dépendance reste à `2.1.0`. La mise à jour automatique ignore les versions plus récentes tant que le plugin A est installé. |

La mise à jour automatique récupère une dépendance contrainte à la balise git la plus élevée qui satisfait la plage de chaque plugin installé, plutôt qu'à la dernière version du marketplace, de sorte que la dépendance continue à recevoir des mises à jour dans sa plage autorisée. Si aucune balise ne satisfait toutes les plages, la mise à jour automatique ignore cette dépendance et répertorie l'ignorance dans l'onglet Erreurs de `/plugin`, en nommant le plugin contraignant.

Lorsque vous désinstallez le dernier plugin qui contraint une dépendance, la dépendance n'est plus maintenue et reprend le suivi de son entrée marketplace lors de la prochaine mise à jour.

<h2 id="enable-or-disable-a-plugin-with-dependencies">
  Activer ou désactiver un plugin avec des dépendances
</h2>

L'activation d'un plugin active également les plugins dont il dépend, et la désactivation d'un plugin est bloquée si un autre plugin activé en a toujours besoin. Les deux comportements nécessitent Claude Code v2.1.143 ou version ultérieure. Les versions antérieures activent ou désactivent uniquement le plugin nommé et affichent une erreur `dependency-unsatisfied` au prochain chargement.

Lorsque vous activez un plugin, Claude Code active également ses dépendances au même scope. Si une dépendance a ses propres dépendances, Claude Code les active également. Le message de succès liste ce qui d'autre a été activé avec le plugin que vous avez nommé. Si une dépendance ne peut pas être activée, la commande refuse et vous dit ce qui bloque et comment corriger :

| Condition                                                                                          | Résultat                                                                                                              |
| :------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| Une dépendance n'est pas installée                                                                 | L'activation échoue et affiche la commande `claude plugin install` pour chaque dépendance manquante.                  |
| Une dépendance est bloquée par la politique de plugin de votre organisation                        | L'activation échoue et nomme la dépendance bloquée.                                                                   |
| Une dépendance est définie sur `false` à un scope avec une priorité plus élevée que le scope cible | L'activation échoue. Activez la dépendance à ce scope, ou passez `--scope` pour écrire là.                            |
| Toutes les dépendances sont installées et autorisées                                               | L'activation réussit et écrit `true` pour le plugin et chaque dépendance qui n'était pas déjà activée au scope cible. |

Ceci s'applique même lorsqu'une dépendance définit [`defaultEnabled: false`](/docs/fr/plugins-reference#default-enablement) dans son manifeste, car Claude Code écrit un `true` explicite pour celle-ci. La même chose s'applique à l'installation : une dépendance extraite pour satisfaire un plugin actif s'installe avec `true` indépendamment de sa propre valeur par défaut.

Lorsque vous désactivez un plugin, Claude Code refuse si un autre plugin activé en dépend toujours. L'erreur nomme les plugins qui en dépendent et vous donne une commande chaînée qui les désactive dans le bon ordre, se terminant par celui que vous avez demandé.

Par exemple, si `deploy-kit` dépend de `secrets-vault`, la désactivation de `secrets-vault` seule échoue avec une sortie similaire à ce qui suit :

```text theme={null}
secrets-vault is still required by deploy-kit. Disable that plugin first, or
disable everything together: claude plugin disable deploy-kit@acme-tools && claude plugin disable secrets-vault@acme-tools
```

Copiez la commande chaînée de l'erreur pour désactiver l'ensemble complet en une seule étape.

<h2 id="remove-orphaned-auto-installed-dependencies">
  Supprimer les dépendances auto-installées orphelines
</h2>

Les dépendances auto-installées restent sur le disque après la désinstallation des plugins qui les ont installées, au cas où vous réinstalliez un plugin dépendant ou souhaiteriez continuer à utiliser la dépendance directement. Pour les nettoyer, exécutez `claude plugin prune` pour lister les dépendances auto-installées qui n'ont plus aucun plugin installé les exigeant et les supprimer après une invite de confirmation. Cela nécessite Claude Code v2.1.121 ou version ultérieure.

```bash theme={null}
claude plugin prune
```

Par défaut, prune fonctionne à la portée utilisateur. Utilisez `--scope project` ou `--scope local` pour cibler une portée différente. Passez `--dry-run` pour lister ce qui serait supprimé sans rien modifier. Passez `-y` pour ignorer l'invite de confirmation. Lorsque stdin ou stdout n'est pas un terminal, prune liste les orphelins et se termine sans les supprimer sauf si `-y` est passé.

Pour nettoyer dans le cadre d'une désinstallation, passez `--prune` à `claude plugin uninstall`. Après suppression du plugin nommé, Claude Code analyse et supprime toute dépendance auto-installée qui est maintenant orpheline. Les plugins que vous avez installés vous-même ne sont jamais nettoyés, uniquement ceux installés automatiquement via le tableau `dependencies` d'un autre plugin.

Par exemple, pour désinstaller `deploy-kit` et nettoyer les dépendances qu'il laisse derrière :

```bash theme={null}
claude plugin uninstall deploy-kit --prune
```

<h2 id="resolve-dependency-errors">
  Résoudre les erreurs de dépendance
</h2>

Les problèmes de dépendance apparaissent dans `claude plugin list` et dans l'interface `/plugin`. Claude Code désactive le plugin affecté jusqu'à ce que vous résolviez l'erreur. Le tableau ci-dessous liste les erreurs les plus courantes et comment les résoudre.

| Erreur                           | Signification                                                                                                                                                                                                                                                                 | Comment résoudre                                                                                                                                                                                                                                                                                  |
| :------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `dependency-unsatisfied`         | Une dépendance déclarée n'est pas installée, ou elle est installée mais désactivée.                                                                                                                                                                                           | Exécutez la commande `claude plugin install` affichée dans le message d'erreur. Si la marketplace de la dépendance n'est pas encore configurée, ajoutez-la avec `claude plugin marketplace add` et Claude Code résout la dépendance automatiquement. Si la dépendance est désactivée, activez-la. |
| `range-conflict`                 | Les exigences de version pour une dépendance ne peuvent pas être combinées. Le message d'erreur nomme la cause : aucune version ne satisfait toutes les plages, une plage n'est pas une syntaxe semver valide, ou les plages combinées sont trop complexes à intersectionner. | Désinstallez ou mettez à jour l'un des plugins en conflit, corrigez toute chaîne `version` invalide, simplifiez les longues chaînes `\|\|`, ou demandez à l'auteur en amont d'élargir sa contrainte.                                                                                              |
| `dependency-version-unsatisfied` | La version de la dépendance installée est en dehors de la plage déclarée de ce plugin.                                                                                                                                                                                        | Exécutez `claude plugin install <dependency>@<marketplace>` pour re-résoudre la dépendance par rapport à toutes les contraintes actuelles.                                                                                                                                                        |
| `no-matching-tag`                | Le référentiel de la dépendance n'a pas de balise `{name}--v*` satisfaisant la plage.                                                                                                                                                                                         | Vérifiez que l'amont a balisé les versions en utilisant la convention ci-dessus, ou assouplissez votre plage.                                                                                                                                                                                     |

Pour vérifier ces erreurs par programmation, exécutez `claude plugin list --json` et lisez le champ `errors` sur chaque plugin.

<h2 id="see-also">
  Voir aussi
</h2>

* [Créer des plugins](/docs/fr/plugins) : créez des plugins avec des skills, des agents et des hooks
* [Créer et distribuer un marketplace de plugins](/docs/fr/plugin-marketplaces) : hébergez des plugins pour votre équipe
* [Référence des plugins](/docs/fr/plugins-reference#plugin-manifest-schema) : le schéma complet de `plugin.json`
* [Gestion des versions](/docs/fr/plugins-reference#version-management) : comment la version propre d'un plugin est résolue et utilisée comme clé de cache
