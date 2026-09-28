> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence Marketplace

> Référence complète des champs marketplace.json, des entrées de plugin et des objets source de plugin et marketplace, avec les emplacements où chacun est valide.

`marketplace.json` est le fichier qui définit une marketplace de plugin. Il contient le nom de la marketplace, son propriétaire et une entrée par plugin. La source de plugin de chaque entrée indique où Claude Code récupère ce plugin.

Une source marketplace est un objet séparé qui indique où Claude Code récupère le fichier marketplace lui-même. Vous en écrivez un dans les paramètres, ou Claude Code en crée un lorsque vous exécutez `claude plugin marketplace add`.

Cette référence est destinée aux responsables de marketplace qui ont besoin d'un nom ou d'une valeur de champ exact, et aux administrateurs qui ont besoin de savoir quelles valeurs `source` sont valides dans [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces), [`strictKnownMarketplaces`](/docs/fr/settings-reference#strictknownmarketplaces) et [`blockedMarketplaces`](/docs/fr/plugins/org#restrict-what-users-can-install).

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Créer ou héberger une marketplace** : voir [Créer une marketplace](/docs/fr/plugins/create-marketplace) et [Héberger et maintenir une marketplace](/docs/fr/plugins/host-marketplace)
  * **Recettes de liste blanche et liste noire** : voir [Gérer les plugins pour votre organisation](/docs/fr/plugins/org)
</Note>

Trouvez la section pour ce que vous écrivez ou lisez :

* **Le fichier marketplace** : [Champs de niveau supérieur](#top-level-fields) et [Entrées de plugin](#plugin-entries)
* **La `source` d'une entrée** : [Sources de plugin](#plugin-sources)
* **Un objet `source` dans les paramètres** : [Sources marketplace](#marketplace-sources)
* **Sortie de [`claude plugin validate <path>`](/docs/fr/plugins/cli-reference)** : [Messages de validation](#validation-messages), qui mappe chaque message au champ qu'il nomme

<h2 id="marketplace-file">
  Fichier marketplace
</h2>

Enregistrez le fichier marketplace à `.claude-plugin/marketplace.json` dans le répertoire de votre marketplace. Si vous conservez le fichier ailleurs dans le référentiel, les utilisateurs doivent déclarer la marketplace dans [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces) avec `path` défini sur sa source, car `claude plugin marketplace add` n'a pas d'option pour cela.

Le répertoire qui contient `.claude-plugin/` s'appelle la racine marketplace, et chaque source de plugin relative se résout à partir de celui-ci, pas à partir de `.claude-plugin/`.

Chaque utilisateur enregistre une marketplace par `name`, donc un utilisateur ne peut pas avoir deux marketplaces avec le même nom enregistrées à la fois.

Claude Code ignore une clé de niveau supérieur inconnue ou une clé d'entrée de plugin plutôt que de la rejeter, donc une faute de frappe se charge silencieusement. `claude plugin validate` signale chaque clé inconnue comme un avertissement.

<h3 id="reserved-names">
  Noms réservés
</h3>

Vous ne pouvez pas donner à votre marketplace l'un des noms suivants :

* **Noms de marketplace officiels** : `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins` et `claude-tag-plugins`. Réservés sauf si la marketplace provient d'une [source marketplace](#marketplace-sources) `github` ou `git` sous `github.com/anthropics/`.
* **Noms de marketplace communautaire** : `claude-community`, `claude-plugins-community` et `healthcare`. Réservés selon la même règle que les noms officiels.
* **Noms de répertoire de plugin** : `anthropic-plugin-directory` et `claude-plugin-directory`. Réservés selon la même règle que les noms officiels.
* **Noms qui usurpent l'identité d'une marketplace officielle** : des noms tels que `official-claude-plugins` ou `claude-plugins-v2`, et tout nom contenant un caractère non-ASCII. L'erreur est `Marketplace name impersonates an official Anthropic/Claude marketplace`. Un caractère de contrôle ou de formatage bidirectionnel dans un nom signale également `Marketplace name cannot contain control or bidirectional-formatting characters`.
* <span id="reserved-name-spellings" />**Une autre orthographe d'un nom réservé** : un nom qui diffère d'un nom réservé uniquement par un point final, ou par un symbole autre qu'un trait d'union à la place d'un trait d'union, donc `claude.code.plugins` compte comme `claude-code-plugins`. `claude plugin validate` accepte un tel nom ; l'ajout de la marketplace échoue avec [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/fr/errors#marketplace-name-is-another-spelling-of-a-reserved-name), et une marketplace déjà enregistrée sous l'une d'elles cesse de se charger. Cette vérification nécessite Claude Code v2.1.280 ou ultérieure.
* **Noms que Claude Code utilise pour les plugins qui ne proviennent pas d'une marketplace** : `inline` pour les plugins chargés avec [`--plugin-dir`](/docs/fr/cli-reference), `builtin` pour les plugins intégrés, `skills-dir` pour les plugins chargés automatiquement à partir de [`.claude/skills/`](/docs/fr/skills) et `synced` pour les plugins synchronisés à partir de votre compte claude.ai. `claude-plugin-test` est également réservé. `skills-dir` apparaît également comme `{"source": "skills-dir"}` dans `strictKnownMarketplaces` et `blockedMarketplaces`, décrits sous [Valeurs source valides uniquement dans les listes de politique](#source-values-valid-only-in-policy-lists).
* **`npm`, `pip`, `uv`, `cargo`, `github` et `gh`** : réservés dans n'importe quelle casse. Cette vérification nécessite Claude Code v2.1.275 ou ultérieure.
* **Noms commençant par `claudeai-`** : réservés pour les marketplaces hébergées sur claude.ai. `claude plugin marketplace add` refuse toute autre marketplace qui en utilise un avec `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai`.

<h2 id="top-level-fields">
  Champs de niveau supérieur
</h2>

Le tableau liste chaque clé que Claude Code lit à partir de `marketplace.json`. `name`, `owner` et `plugins` sont obligatoires.

| Champ                                      | Type             | Description                                                                                                                                                                                                                                                                                                       |
| :----------------------------------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                     | string           | Identifiant de marketplace. Pas d'espaces, de caractères de contrôle ou de caractères de formatage bidirectionnel, pas de `/` ou `\`, pas de `..` et pas `.`. Voir [Noms réservés](#reserved-names). Les utilisateurs le tapent après `@` lorsqu'ils installent un plugin                                         |
| `owner`                                    | object           | Informations du responsable. `name` est obligatoire ; `email` et `url` sont optionnels                                                                                                                                                                                                                            |
| `plugins`                                  | array            | [Entrées de plugin](#plugin-entries). Chaque entrée est validée indépendamment, donc une entrée invalide ne fait pas échouer la marketplace                                                                                                                                                                       |
| `$schema`                                  | string           | URL JSON Schema pour l'autocomplétion de l'éditeur. Ignorée au moment du chargement                                                                                                                                                                                                                               |
| `description`                              | string           | Description de la marketplace affichée aux utilisateurs. `claude plugin validate` avertit lorsqu'elle est manquante                                                                                                                                                                                               |
| `version`                                  | string           | Version du manifeste marketplace                                                                                                                                                                                                                                                                                  |
| `metadata.description`, `metadata.version` | string           | Emplacement alternatif pour `description` et `version`                                                                                                                                                                                                                                                            |
| `metadata.pluginRoot`                      | string           | Répertoire sous lequel les noms de source de plugin nus se résolvent. Voir [Source de plugin avec chemin relatif](#relative-path-plugin-source). Nécessite Claude Code v2.1.239 ou ultérieure                                                                                                                     |
| `forceRemoveDeletedPlugins`                | boolean          | Lorsque `true`, un plugin que vous supprimez de `plugins` est désinstallé sur les machines des utilisateurs. Voir [Héberger et maintenir une marketplace](/docs/fr/plugins/host-marketplace)                                                                                                                           |
| `allowCrossMarketplaceDependenciesOn`      | array of strings | Noms de marketplace dont les plugins peuvent être installés en tant que dépendances des plugins de cette marketplace. Lorsque vous installez un plugin, seule la liste de la propre marketplace du plugin s'applique, pour toute sa chaîne de dépendances. Voir [Dépendances de plugin](/docs/fr/plugins/dependencies) |
| `renames`                                  | object           | Mappage d'un ancien `name` de plugin à son nom actuel, ou à `null` pour un plugin que vous avez supprimé. Nécessite Claude Code v2.1.193 ou ultérieure. Voir [Héberger et maintenir une marketplace](/docs/fr/plugins/host-marketplace)                                                                                |

<h2 id="plugin-entries">
  Entrées de plugin
</h2>

Chaque objet du tableau `plugins` de niveau supérieur de `marketplace.json` nomme un plugin et indique où le récupérer. `name` et `source` sont obligatoires.

Une entrée accepte également tous les [champs `plugin.json`](/docs/fr/plugins/manifest-reference), tels que `description`, `version`, `author`, `commands` et `hooks`. Pour savoir quand ces champs s'appliquent, consultez [Comment une entrée se combine avec plugin.json](#entry-and-plugin-json).

Le tableau répertorie les champs propres à l'entrée et les champs du manifeste dont le sens change dans une entrée.

| Champ            | Type             | Description                                                                                                                                                                                                                                                                                                                                                                         |
| :--------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`           | string           | Identifiant du plugin, sans espaces, caractères de contrôle ou caractères de formatage bidirectionnel. Les utilisateurs le tapent avant `@` lors de l'installation, même si le propre `plugin.json` du plugin définit un `name` différent                                                                                                                                           |
| `source`         | string ou object | Où récupérer le plugin. Consultez [Sources de plugin](#plugin-sources)                                                                                                                                                                                                                                                                                                              |
| `description`    | string           | Affiché dans les listes et détails [`/plugin`](/docs/fr/plugins/install)                                                                                                                                                                                                                                                                                                                 |
| `version`        | string           | Chaîne de version pour le plugin. Quand `plugin.json` définit également `version`, `plugin.json` a la priorité et `claude plugin validate` avertit. Consultez [Référence de chargement de plugin](/docs/fr/plugins/loading)                                                                                                                                                              |
| `category`       | string           | Catégorie libre pour organiser le catalogue                                                                                                                                                                                                                                                                                                                                         |
| `tags`           | array of strings | Balises libres pour la recherche                                                                                                                                                                                                                                                                                                                                                    |
| `strict`         | boolean          | Par défaut `true`. Si `plugin.json` est la source définitive des composants du plugin. Consultez [Mode strict](#strict-mode)                                                                                                                                                                                                                                                        |
| `relevance`      | object           | Signaux qui indiquent à Claude Code quand suggérer le plugin. Consultez [Recommander des plugins pour votre organisation](/docs/fr/plugins/relevance)                                                                                                                                                                                                                                    |
| `dependencies`   | array            | Plugins qui doivent être activés pour que celui-ci fonctionne. Chaque élément est `"name"`, `"name@marketplace"` ou un objet. Consultez [Dépendances de plugin](/docs/fr/plugins/dependencies)                                                                                                                                                                                           |
| `defaultEnabled` | boolean          | Par défaut `true`. Si le plugin démarre activé quand l'utilisateur ne l'a pas défini dans [`enabledPlugins`](/docs/fr/settings-reference#enabledplugins). La valeur de l'entrée a la priorité sur `plugin.json`                                                                                                                                                                          |
| `displayName`    | string           | Nom lisible affiché dans l'interface utilisateur. Quand ni l'entrée ni le `plugin.json` du plugin n'en définit un, les utilisateurs voient le `name` du plugin                                                                                                                                                                                                                      |
| `metadata`       | object           | Objet libre pour vos propres champs. Claude Code ne le lit pas. Nécessite Claude Code v2.1.222 ou ultérieur                                                                                                                                                                                                                                                                         |
| `headers`        | object           | En-têtes HTTP que Claude Code envoie quand il télécharge l'[archive](#archive-plugin-source) de cette entrée. Un en-tête défini ici remplace un en-tête du même nom de la source du marketplace [`headers`](#fields-by-type). Nécessite Claude Code v2.1.238 ou ultérieur                                                                                                           |
| `headersHelper`  | string           | Commande qui imprime les en-têtes de téléchargement d'archive de cette entrée sous la forme d'un objet JSON, pour une accréditation qui expire. L'entrée doit également définir [`"strict": false`](#strict-mode). Nécessite Claude Code v2.1.238 ou ultérieur. Consultez [Authentifier les téléchargements d'archive](/docs/fr/plugins/host-marketplace#authenticate-archive-downloads) |

<h3 id="entry-and-plugin-json">
  Comment une entrée se combine avec plugin.json
</h3>

Les champs de l'entrée s'appliquent différemment à un plugin récupéré qui a son propre `.claude-plugin/plugin.json` et à un qui n'en a pas :

* **Pas de `plugin.json`** : l'entrée est le manifeste indépendamment de `strict`. Chaque champ de manifeste dans l'entrée s'applique, y compris [`mcpServers`, `lspServers`, `userConfig` et `channels`](/docs/fr/plugins/manifest-reference).
* **`plugin.json` présent** : `plugin.json` est le manifeste. Le [mode strict](#strict-mode) décide si les six champs de composant de l'entrée, `commands`, `agents`, `skills`, `hooks`, `outputStyles` et `themes`, sont combinés avec lui ou rejetés comme un conflit. L'entrée `mcpServers`, `lspServers`, `userConfig` et `channels` ne s'appliquent pas. Déclarez-les dans `plugin.json`.

<h4 id="hooks-in-an-entry">
  Hooks dans une entrée
</h4>

Écrivez les `hooks` d'entrée comme un objet en ligne qui mappe les noms d'événements de hook aux tableaux de correspondance. Si vous écrivez un chemin de fichier ou un tableau à la place, `claude plugin validate` le passe. Ces hooks ne s'exécutent jamais, et Claude Code signale une erreur `not yet supported in a marketplace entry` pour le plugin. Mettez les hooks basés sur des fichiers dans le propre [`hooks/hooks.json`](/docs/fr/plugins/components) du plugin ou `plugin.json`.

<h4 id="display-fields">
  Champs d'affichage
</h4>

L'entrée et le propre `plugin.json` du plugin peuvent tous deux définir les champs d'affichage `displayName`, `description`, `author`, `homepage`, `repository`, `license` et `keywords`. Les utilisateurs voient ces valeurs dans les listes et détails des plugins, avant et après l'installation :

* Pour un champ que vous définissez sur l'entrée, les utilisateurs voient la valeur de l'entrée, même quand `plugin.json` en définit une différente.
* Pour un champ que l'entrée laisse non défini, les utilisateurs voient la valeur `plugin.json`.

Avant l'installation, Claude Code ne peut lire `plugin.json` que pour les entrées avec une [source de chemin relatif](#relative-path-plugin-source), dont les fichiers de plugin se trouvent à l'intérieur du marketplace lui-même. Pour une entrée avec tout autre type de source, les utilisateurs ne voient que les champs propres de l'entrée jusqu'à ce qu'ils installent le plugin.

<h3 id="strict-mode">
  Mode strict
</h3>

`strict` décide ce qui se passe quand le plugin récupéré a son propre `plugin.json` et que l'entrée déclare également l'un des [champs de composant](#entry-and-plugin-json) : `commands`, `agents`, `skills`, `hooks`, `outputStyles` ou `themes`. Avec `strict: true`, la valeur par défaut, Claude Code ajoute les champs de composant de l'entrée à `plugin.json`, sauf `hooks`, dont les correspondances remplacent celles du manifeste par événement. Avec `strict: false`, une entrée qui déclare un champ de composant est un conflit, et le plugin ne se charge pas. Le tableau montre chaque combinaison de `strict`, `plugin.json` et des champs de composant de l'entrée.

| `strict`                     | `plugin.json` | Champs de composant d'entrée | Résultat                                                                                                                                                                                                                                                         |
| :--------------------------- | :------------ | :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| any                          | absent        | any                          | L'entrée est le manifeste                                                                                                                                                                                                                                        |
| `true`, la valeur par défaut | présent       | any                          | `plugin.json` est l'autorité. Claude Code ajoute les champs de composant de l'entrée à celui-ci, sauf `hooks`, dont les correspondances [remplacent celles du manifeste par événement](/docs/fr/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`                      | présent       | none                         | `plugin.json` est le manifeste, comme avec `true`                                                                                                                                                                                                                |
| `false`                      | présent       | un ou plusieurs              | Conflit. Le plugin ne se charge pas avec `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components`                                                                                                                    |

<h2 id="plugin-sources">
  Sources de plugin
</h2>

La `source` d'une entrée de plugin indique où Claude Code récupère ce plugin. C'est soit une chaîne de chemin relatif, soit un objet dont la propre clé `source` nomme le type, donc une entrée ressemble à `"source": { "source": "github", "repo": "your-org/formatter" }`.

Le tableau liste chaque type de source de plugin et ses champs.

| Type           | Champs                           | Notes                                                                                                                                                                                                                                                |
| :------------- | :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Chemin relatif | la chaîne elle-même              | Un répertoire à l'intérieur de la marketplace, résolu à partir de la racine marketplace. Doit commencer par `./`, sauf si vous écrivez un [nom nu sous `metadata.pluginRoot`](#relative-path-plugin-source). `"."` seul signifie la racine elle-même |
| `github`       | `repo`, `ref`, `sha`             | Référentiel GitHub sous la forme `owner/repo`                                                                                                                                                                                                        |
| `url`          | `url`, `ref`, `sha`              | Tout référentiel git par URL                                                                                                                                                                                                                         |
| `git-subdir`   | `url`, `path`, `ref`, `sha`      | Un sous-répertoire d'un référentiel git, récupéré avec un clone partiel clairsemé                                                                                                                                                                    |
| `npm`          | `package`, `version`, `registry` | Package npm, récupéré avec votre client npm et décompressé sans exécuter les scripts d'installation                                                                                                                                                  |
| `archive`      | `url`, `sha256`                  | Archive Zip sur HTTPS. Nécessite Claude Code v2.1.224 ou ultérieure                                                                                                                                                                                  |
| `command`      | `command`, `timeout`, `mode`     | Répertoire imprimé par une commande que Claude Code exécute sur la machine de l'utilisateur. Nécessite Claude Code v2.1.229 ou ultérieure                                                                                                            |

Les noms `url` et `github` sont également des types de [source marketplace](#marketplace-sources), où `url` signifie un lien direct vers un fichier `marketplace.json` plutôt qu'un référentiel git. `git` n'existe que comme source marketplace, et `npm` existe comme les deux. `git-subdir`, `archive` et `command` n'existent que comme sources de plugin.

Utilisez un chemin relatif pour un plugin dans un sous-répertoire du référentiel marketplace lui-même. Utilisez `git-subdir` pour un sous-répertoire d'un autre référentiel.

Les sources `github`, `url` et `git-subdir` partagent les champs `ref` et `sha` :

* **`ref`** : une branche ou une balise. Par défaut, la branche par défaut du référentiel.
* **`sha`** : un SHA de commit complet de 40 caractères en minuscules. Lorsque vous définissez à la fois `ref` et `sha`, Claude Code extrait `sha`. Sur la plupart des hôtes git, y compris GitHub, GitLab et Bitbucket, cela signifie que l'installation réussit même si la branche ou la balise nommée par `ref` a depuis été supprimée en amont, tant que le commit est toujours accessible à partir du référentiel. Certains serveurs, tels que AWS CodeCommit, ne supportent pas la récupération de commits par SHA. Sur ces serveurs, le `ref` doit toujours exister et le commit épinglé doit être accessible à partir de celui-ci.

Pour savoir comment chaque type est récupéré, mis en cache et versionné, voir [Référence de chargement de plugin](/docs/fr/plugins/loading).

<h3 id="relative-path-plugin-source">
  Source de plugin avec chemin relatif
</h3>

Le chemin se résout à partir de la racine marketplace. `./plugins/formatter` est `<root>/plugins/formatter` même si le fichier marketplace est dans `<root>/.claude-plugin/`.

Un chemin contenant `..` échoue la validation. Sur macOS et Linux, Claude Code refuse un chemin d'entrée qui contient une barre oblique inverse n'importe où après le `./` initial, donc écrivez le chemin avec des barres obliques avant.

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

Un chemin relatif se résout uniquement lorsque Claude Code a les fichiers de la marketplace, donc vérifiez le type de [source marketplace](#marketplace-sources) :

* **`github`, `git`, `file` et `directory`** : Claude Code a les fichiers de la marketplace.
* **`url`** : Claude Code récupère uniquement `marketplace.json`, donc les chemins relatifs ne peuvent pas se résoudre. Donnez à chaque plugin une source d'objet à la place, telle que `github` ou `git-subdir`.
* **`settings`** : les chemins relatifs sont rejetés d'emblée.

<h4 id="bare-names-under-pluginroot">
  Noms nus sous pluginRoot
</h4>

Un nom nu est un seul nom de répertoire sans `/`, tel que `"formatter"`. Pour écrire des noms nus au lieu de chemins `./`, définissez [`metadata.pluginRoot`](#top-level-fields) sur le répertoire sous lequel ils se résolvent. Avec `"pluginRoot": "./plugins"`, `"source": "formatter"` se résout à `./plugins/formatter`. Nécessite Claude Code v2.1.239 ou ultérieure.

`metadata.pluginRoot` a ces limites :

* Il doit lui-même être un chemin relatif à l'intérieur de la marketplace.
* Il n'a aucun effet sur une source qui commence déjà par `./`.
* Une source qui contient un `/`, telle que `team-a/formatter`, n'est pas un nom nu et a toujours besoin du préfixe `./`, même lorsque `metadata.pluginRoot` est défini.

<h3 id="github-plugin-source">
  Source de plugin github
</h3>

`repo` prend `owner/repo`. `ref` et `sha` sont optionnels.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  Source de plugin url
</h3>

`url` est une URL git complète : `https://`, `http://`, `file://` ou `git@`. Un suffixe `.git` n'est pas requis, donc les URL Azure DevOps et AWS CodeCommit fonctionnent telles qu'elles sont écrites. Ce type ne prend pas le raccourci `owner/repo`.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  Source de plugin git-subdir
</h3>

`url` accepte une URL git complète ou le raccourci GitHub `owner/repo`. `path` est le sous-répertoire qui contient le plugin, et Claude Code télécharge uniquement ce sous-répertoire.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  Source de plugin npm
</h3>

Une source `npm` prend ces champs :

* `package` : un nom de package, ou un nom scopé tel que `@your-org/formatter`
* `version` : une version ou une plage
* `registry` : une URL de registre pour un package qui n'est pas sur le registre par défaut

Claude Code récupère le package avec votre client npm. Les scripts d'installation du package, tels que `preinstall` ou `postinstall`, ne s'exécutent jamais, et ses dépendances ne sont pas installées lors de la récupération. Si le package a un fichier de verrouillage supporté à côté de son `package.json`, Claude Code installe ces [dépendances de package Node.js](/docs/fr/plugins/loading#node-js-package-dependencies) dans une étape séparée, également avec les scripts désactivés.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  Source de plugin archive
</h3>

`url` doit utiliser `https://` et ne peut pas pointer vers un hôte loopback, link-local ou cloud-metadata.

La racine du plugin peut être au sommet du zip ou un répertoire plus bas.

`sha256` est le digest de l'archive en tant que 64 caractères hexadécimaux, majuscules ou minuscules. Lorsque vous le définissez, Claude Code refuse un téléchargement qui ne correspond pas.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  Source de plugin command
</h3>

Utilisez une source `command` lorsqu'un outil installé sur la machine de l'utilisateur produit le répertoire du plugin, tel qu'un IDE qui rend son plugin pour la chaîne d'outils que l'utilisateur a sélectionnée. Claude Code exécute la commande lorsque l'utilisateur installe ou met à jour le plugin, et [à nouveau une fois par session](/docs/fr/plugins/loading#when-a-command-source-re-runs), donc les utilisateurs obtiennent la sortie modifiée de l'outil sans réinstaller.

Une source `command` prend ces champs :

* `command` : une commande shell qui imprime le chemin absolu du répertoire du plugin en une seule ligne et quitte 0. Claude Code affiche la chaîne entière aux utilisateurs pour examen avant de l'exécuter. Écrivez-la en ASCII imprimable, au maximum 500 caractères, sans suite de quatre espaces ou plus.
* `timeout` : un nombre entier de secondes de 1 à 600. Par défaut 60.
* `mode` : `copy`, la valeur par défaut, ou `link`. Voir [Mode copie et mode lien](#copy-mode-and-link-mode).

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

Pour savoir comment les utilisateurs acceptent la commande, voir [Installer à partir de votre shell](/docs/fr/plugins/install#install-from-your-shell). Pour ce que les utilisateurs voient après l'avoir modifiée, voir [Modifier la commande d'une source command](/docs/fr/plugins/host-marketplace#change-the-command-of-a-command-source). Les administrateurs désactivent les sources command avec [`disableCommandPluginSources`](/docs/fr/settings-reference#disablecommandpluginsources).

<h4 id="what-the-command-must-do">
  Ce que la commande doit faire
</h4>

Écrivez la commande pour répondre à ces exigences :

* **Shell et répertoire de travail** : Claude Code exécute la commande via `sh`, ou via `cmd.exe` sur Windows, à partir du répertoire personnel de l'utilisateur. Donnez un chemin absolu ou une commande sur `PATH`.
* **Sortie** : imprimez exactement une ligne sur stdout, le chemin absolu du répertoire du plugin, et quittez 0 dans les `timeout` secondes.
* **Contenu du répertoire** : le répertoire contient le plugin complet au moment où la commande quitte. Le chemin peut différer d'une exécution à l'autre.

<h4 id="output-that-fails-the-install-or-update">
  Sortie qui échoue l'installation ou la mise à jour
</h4>

L'installation ou la mise à jour échoue lorsque la commande quitte non-zéro, s'exécute plus longtemps que `timeout`, ou imprime autre chose qu'un chemin absolu. Elle échoue également lorsque le répertoire imprimé est l'un de ceux-ci :

* **Pas de contenu de plugin** : le répertoire imprimé n'a pas de contenu de plugin à son niveau supérieur, tel qu'un répertoire `.claude-plugin/` ou un répertoire `skills/`, `commands/`, `agents/` ou `hooks/`.
* **Le répertoire de la session elle-même** : le répertoire imprimé est celui dans lequel Claude Code a été démarré, ou l'un de ses parents.
* **Un chemin réseau** : sur Windows, le chemin imprimé est un chemin UNC.
* **Trop volumineux à copier** : en mode copie, le répertoire est plus grand que 256 MiB ou a plus de 20 000 entrées.

<h4 id="copy-mode-and-link-mode">
  Mode copie et mode lien
</h4>

`mode` décide si Claude Code copie le répertoire imprimé ou l'utilise sur place :

* **`copy`** : Claude Code copie le répertoire dans le cache du plugin et dérive la [version du plugin](/docs/fr/plugins/loading#how-claude-code-computes-the-version) d'un hash des fichiers copiés. Votre outil peut supprimer ou réécrire le répertoire après la sortie de la commande. Une réexécution qui produit des fichiers identiques compte comme à jour.
* **`link`** : Claude Code remplit l'entrée du cache du plugin avec un lien vers chaque entrée de niveau supérieur du répertoire imprimé et charge les fichiers sur place. Rien n'est copié, les contenus de fichiers ne sont pas hashés, et les limites de taille ne s'appliquent pas. Utilisez-le pour un répertoire trop volumineux à copier, tel qu'une exportation SDK rendue.

Un plugin en mode lien a ces exigences :

* **Gardez le répertoire en place** : Claude Code charge le plugin via les liens à chaque démarrage, donc le répertoire imprimé doit rester où il est tant que le plugin reste installé.
* **Imprimez un chemin différent pour signaler un nouveau contenu** : la version provient du chemin réel du répertoire imprimé et de ses entrées de niveau supérieur, pas des fichiers à l'intérieur.
* **Gardez les symlinks de niveau supérieur à l'intérieur du répertoire** : l'installation échoue si une entrée de niveau supérieur est un symlink qui pointe en dehors du répertoire imprimé.
* **Incluez `node_modules`** : Claude Code saute l'[installation de dépendance de package Node.js](/docs/fr/plugins/loading#node-js-package-dependencies) pour un plugin en mode lien, donc imprimez un répertoire qui contient déjà les packages dont le plugin a besoin.
* **Sessions démarrées à l'intérieur du répertoire** : une session démarrée dans le répertoire imprimé ou n'importe où en dessous ne charge pas le plugin.
* **Pas sur Windows** : Claude Code refuse d'installer un plugin en mode lien sur Windows. Déclarez `"mode": "copy"` là.

<h2 id="marketplace-sources">
  Sources marketplace
</h2>

Une source marketplace indique où Claude Code récupère un `marketplace.json`. La CLI en crée une pour vous lorsque vous ajoutez une marketplace, et vous en écrivez une vous-même dans les paramètres :

* **[`claude plugin marketplace add`](/docs/fr/plugins/cli-reference)** : Claude Code crée la source à partir de la chaîne que vous passez.
* **[`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces)** : vous écrivez la source vous-même en tant qu'objet `source`.
* **[`strictKnownMarketplaces`](/docs/fr/settings-reference#strictknownmarketplaces) et [`blockedMarketplaces`](/docs/fr/plugins/org#restrict-what-users-can-install)** : les administrateurs écrivent les sources dans ces deux listes de politique. `strictKnownMarketplaces` est la liste blanche et `blockedMarketplaces` est la liste noire.

Les noms de type `url`, `git` et `github` signifient quelque chose de différent dans une source marketplace que dans une [source de plugin](#plugin-sources) :

| Nom de type | En tant que source marketplace                                                                         | En tant que source de plugin                                                    |
| :---------- | :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------ |
| `url`       | Un lien direct vers un fichier `marketplace.json`, avec les champs `url`, `headers` et `headersHelper` | Un référentiel git à cloner, avec les champs `url`, `ref` et `sha`              |
| `git`       | Un référentiel git à cloner, avec les champs `url`, `ref`, `path` et `sparsePaths`                     | N'existe pas                                                                    |
| `github`    | Un référentiel GitHub, avec les champs `repo`, `ref`, `path` et `sparsePaths`                          | Un référentiel GitHub, avec les champs `repo`, `ref` et `sha`, et pas de `path` |

Le tableau liste chaque type de source marketplace avec ses champs, l'entrée `claude plugin marketplace add` qui le produit, et ce qu'il fait dans chacune des trois clés de paramètres.

| Type          | Champs                               | Entrée `marketplace add`                                                                                                                                              | `extraKnownMarketplaces`                                         | `strictKnownMarketplaces`                                                                                                                                                                                                                                                                | `blockedMarketplaces`                                                   |
| :------------ | :----------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------- |
| `url`         | `url`, `headers`, `headersHelper`    | Une URL `http://` ou `https://` qui ne correspond pas à une forme git                                                                                                 | Charge                                                           | Permet la même URL                                                                                                                                                                                                                                                                       | Bloque la même URL                                                      |
| `github`      | `repo`, `ref`, `path`, `sparsePaths` | `owner/repo`, `owner/repo@ref` ou `owner/repo#ref`                                                                                                                    | Charge                                                           | Permet le même `repo`, `ref` et `path`. `repo` peut être `owner/*`                                                                                                                                                                                                                       | Bloque le même, et une URL `git` vers le même référentiel               |
| `git`         | `url`, `ref`, `path`, `sparsePaths`  | Une URL `user@host:path`, ou une URL `https://` qui se termine par `.git`, contient `/_git/`, ou nomme un référentiel github.com ou gitlab.com. `#ref` épingle un ref | Charge                                                           | Permet la même URL, `ref` et `path`                                                                                                                                                                                                                                                      | Bloque le même, et d'autres orthographes du même référentiel github.com |
| `npm`         | `package`                            | Non produit                                                                                                                                                           | Échoue à charger : `NPM marketplace sources not yet implemented` | Analyse mais ne correspond à rien, car rien n'enregistre une marketplace `npm`                                                                                                                                                                                                           | Analyse mais ne correspond à rien                                       |
| `file`        | `path`                               | Un chemin vers un fichier `.json`                                                                                                                                     | Charge                                                           | Permet le même chemin                                                                                                                                                                                                                                                                    | Bloque le même chemin                                                   |
| `directory`   | `path`                               | Un chemin vers un répertoire                                                                                                                                          | Charge                                                           | Permet le même chemin                                                                                                                                                                                                                                                                    | Bloque le même chemin                                                   |
| `settings`    | `name`, `plugins`, `owner`           | Non produit                                                                                                                                                           | Charge                                                           | Permet une entrée avec le même `name` et des `plugins` identiques                                                                                                                                                                                                                        | Bloque le même `name`                                                   |
| `skills-dir`  | none                                 | Non produit                                                                                                                                                           | Échoue à charger : `Unsupported marketplace source type`         | Garde les [plugins du répertoire de compétences](/docs/fr/plugins/org#keep-skills-directory-plugins-loading) en cours de chargement tandis qu'une liste blanche est définie. Voir [Valeurs source valides uniquement dans les listes de politique](#source-values-valid-only-in-policy-lists) | Arrête les plugins du répertoire de compétences de charger              |
| `hostPattern` | `hostPattern`                        | Non produit                                                                                                                                                           | Échoue à charger : `Unsupported marketplace source type`         | Permet les sources `github`, `git` et `url` dont l'hôte correspond                                                                                                                                                                                                                       | Bloque ces sources                                                      |
| `pathPattern` | `pathPattern`                        | Non produit                                                                                                                                                           | Échoue à charger : `Unsupported marketplace source type`         | Permet les sources `file` et `directory` dont le `path` correspond                                                                                                                                                                                                                       | Bloque ces sources                                                      |

<h3 id="fields-by-type">
  Champs par type
</h3>

Le tableau liste chaque champ de source marketplace qui a une valeur par défaut, une contrainte ou un sens spécifique à son type.

| Champ           | Types           | Description                                                                                                                                                                                                                                                                         |
| :-------------- | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`           | `url`           | Lien vers le fichier `marketplace.json`. Claude Code télécharge uniquement ce fichier, donc les plugins de la marketplace ne peuvent pas utiliser les [sources avec chemin relatif](#relative-path-plugin-source)                                                                   |
| `url`           | `git`           | Le référentiel git à cloner                                                                                                                                                                                                                                                         |
| `headers`       | `url`           | Mappage des en-têtes HTTP que Claude Code envoie avec la récupération, pour les hôtes authentifiés                                                                                                                                                                                  |
| `headersHelper` | `url`           | Commande qui imprime les en-têtes dont les valeurs sont trop éphémères pour être listées dans `headers`. Nécessite Claude Code v2.1.238 ou ultérieure. Voir [Authentifier les téléchargements d'archive](/docs/fr/plugins/host-marketplace#authenticate-archive-downloads)               |
| `repo`          | `github`        | Dans `marketplace add` et `extraKnownMarketplaces`, `repo` doit nommer un référentiel. `marketplace add` rejette `owner/*` comme n'étant pas un raccourci `owner/repo` valide ; dans `extraKnownMarketplaces` Claude Code le prend littéralement et le clone échoue                 |
| `ref`           | `github`, `git` | Branche ou balise. Par défaut, la branche par défaut du référentiel                                                                                                                                                                                                                 |
| `path`          | `github`, `git` | Le chemin du fichier marketplace à l'intérieur du référentiel. Par défaut `.claude-plugin/marketplace.json`                                                                                                                                                                         |
| `path`          | `file`          | Le fichier marketplace lui-même. Claude Code le lit sur place et prend le répertoire deux niveaux plus haut comme racine marketplace, donc gardez le fichier à `<root>/.claude-plugin/marketplace.json`                                                                             |
| `path`          | `directory`     | La racine marketplace, le répertoire qui contient `.claude-plugin/marketplace.json`                                                                                                                                                                                                 |
| `sparsePaths`   | `github`, `git` | Tableau de répertoires pour un checkout clairsemé, tel que `[".claude-plugin", "plugins"]`. `claude plugin marketplace add --sparse` le définit                                                                                                                                     |
| `skipLfs`       | `github`, `git` | Accepté et n'a aucun effet. Voir [Gardez les fichiers de plugin en dehors de Git LFS](/docs/fr/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs)                                                                                                                                |
| `name`          | `settings`      | Doit égaler la clé `extraKnownMarketplaces` et ne peut pas être un [nom réservé](#reserved-names)                                                                                                                                                                                   |
| `plugins`       | `settings`      | Le catalogue en ligne, sans fichier hébergé. Chaque élément prend `name`, `source`, `description`, `version`, `strict`, `headers` et `headersHelper`. Écrivez la `source` de chaque élément en tant que type d'objet, car un chemin relatif n'a pas de référentiel pour se résoudre |

<h3 id="source-values-valid-only-in-policy-lists">
  Valeurs source valides uniquement dans les listes de politique
</h3>

`hostPattern`, `pathPattern`, `skills-dir` et la forme `owner/*` de `repo` sont valides uniquement dans les deux listes de politique, `strictKnownMarketplaces` et `blockedMarketplaces` :

* **`hostPattern` et `pathPattern`** : expressions régulières que Claude Code teste contre une source avant de la récupérer.
* **`skills-dir`** : pas une source. Si vous définissez `strictKnownMarketplaces` du tout, les [plugins du répertoire de compétences](/docs/fr/plugins/org#keep-skills-directory-plugins-loading) cessent de charger jusqu'à ce que vous ajoutiez `{"source": "skills-dir"}` à cette liste.
* **`owner/*`** : en tant que valeur `repo` `github`, correspond à chaque référentiel sous exactement ce propriétaire GitHub. Nécessite Claude Code v2.1.223 ou ultérieure.

Pour l'ordre de correspondance, la sémantique exacte de `ref` et les recettes, voir [Gérer les plugins pour votre organisation](/docs/fr/plugins/org).

<h3 id="source-objects-in-settings">
  Objets source dans les paramètres
</h3>

Une valeur `extraKnownMarketplaces` est un mappage du nom de marketplace à un objet avec `source`. Cette entrée enregistre une marketplace à partir d'un référentiel git à sa branche `main` :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` et `blockedMarketplaces` sont des tableaux d'objets source. Cette liste blanche admet un propriétaire GitHub et un hôte interne :

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  Messages de validation
</h2>

`claude plugin validate <path>` prend la racine marketplace ou le fichier marketplace lui-même. Il imprime les erreurs et les avertissements. Pour les codes de sortie et `--strict`, voir [plugin validate](/docs/fr/plugins/cli-reference#plugin-validate).

Un message nomme une entrée de plugin par son index, écrit comme `plugins.1.source` ou `plugins[1].source`.

Un message préfixé par un index d'entrée et `plugin.json →`, tel que `plugins[2] plugin.json →`, concerne les propres fichiers de ce plugin. [`claude plugin validate` signale les erreurs](/docs/fr/plugins/troubleshooting#claude-plugin-validate-reports-errors) liste ces messages avec leurs corrections.

Les avertissements qui mentionnent les noms de drapeaux Claude Desktop signalent les noms que Claude Code accepte mais que Claude Desktop rejette, car les règles de nom de Claude Desktop sont plus strictes.

Le tableau mappe les messages au niveau marketplace au champ dont chacun parle.

| Message                                                                                                                                                                                           | Niveau        | Champ                                                                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `Marketplace must have a name`                                                                                                                                                                    | Erreur        | `name` est vide                                                                                                                             |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                                 | Erreur        | `name`                                                                                                                                      |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                             | Erreur        | `name`                                                                                                                                      |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                          | Erreur        | `name`. Voir [Noms réservés](#reserved-names)                                                                                               |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                                  | Erreur        | `name` contient un caractère de contrôle, tel qu'une échappement ou une nouvelle ligne, ou un caractère de formatage bidirectionnel Unicode |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, et les variantes `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github` et `gh` | Erreur        | `name`                                                                                                                                      |
| `Author name cannot be empty`                                                                                                                                                                     | Erreur        | `owner.name`                                                                                                                                |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                           | Erreur        | `plugins[i].name`                                                                                                                           |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                       | Erreur        | `plugins[i].name`                                                                                                                           |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                                  | Erreur        | Deux entrées partagent un `name`                                                                                                            |
| `plugins.i.source: Invalid input`                                                                                                                                                                 | Erreur        | La `source` de l'entrée ne correspond à aucun type. Voir [Entrée invalide sur une source](#invalid-input-on-a-source)                       |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                                   | Erreur        | Une `source` relative qui échappe à la racine marketplace                                                                                   |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                                 | Erreur        | `plugins[i].source`                                                                                                                         |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                        | Erreur        | `plugins[i].headersHelper`, sur une entrée `archive`                                                                                        |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                               | Erreur        | `renames.<old>`                                                                                                                             |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                          | Erreur        | `renames.<old>`                                                                                                                             |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                         | Avertissement | La clé nommée au niveau supérieur, sous `metadata`, dans une entrée, ou sous la `relevance` d'une entrée                                    |
| `Marketplace has no plugins defined`                                                                                                                                                              | Avertissement | `plugins` est vide                                                                                                                          |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                                | Avertissement | `plugins[i].headers` ou `plugins[i].headersHelper`, sur une entrée dont la `source` n'est pas `archive`                                     |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                                      | Avertissement | `plugins[i].source.sha256`                                                                                                                  |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                        | Avertissement | `plugins[i].headers.<name>`                                                                                                                 |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                              | Avertissement | `plugins[i].source`                                                                                                                         |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                                   | Avertissement | `description`                                                                                                                               |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                                   | Avertissement | `plugins[i].version`, sur une entrée avec chemin relatif                                                                                    |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                        | Avertissement | `plugins[i].relevance`                                                                                                                      |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                             | Avertissement | `plugins[i].metadata`                                                                                                                       |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                                | Avertissement | `plugins[i].experimental`                                                                                                                   |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                              | Avertissement | `name` est `org`, `org-provisioned` ou `unknown`. Claude Desktop rejette la marketplace                                                     |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                 | Avertissement | `name`. Claude Desktop rejette la marketplace                                                                                               |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                      | Avertissement | `plugins[i].name`. Claude Desktop supprime l'entrée                                                                                         |

<h3 id="invalid-input-on-a-source">
  Entrée invalide sur une source
</h3>

`Invalid input` sur une `source` signifie que l'objet ne correspondait à aucun type de source. Vérifiez ces causes :

* Un chemin relatif qui ne commence pas par `./`, autre que `"."` ou un [nom nu sous `metadata.pluginRoot`](#relative-path-plugin-source)
* Un `package` `npm` contenant `..`
* Un type `source` qui n'est pas l'un des [sources de plugin](#plugin-sources)
* Un type connu avec un champ obligatoire manquant ou du mauvais type, tel que `github` sans `repo`

<h3 id="failures-that-validation-doesn’t-catch">
  Défaillances que la validation ne détecte pas
</h3>

`claude plugin validate` ne signale pas chaque défaillance. Un `hooks` d'entrée écrit comme un chemin de fichier ou un tableau passe la validation, et l'erreur n'apparaît que lorsque le plugin se charge, comme le décrit [Hooks dans une entrée](#hooks-in-an-entry). Les erreurs de récupération d'une `source` apparaissent également uniquement après l'installation, pas dans la validation.

[`claude plugin list`](/docs/fr/plugins/cli-reference) affiche un plugin qui n'a pas pu se charger avec son erreur, et [Dépanner les plugins](/docs/fr/plugins/troubleshooting) couvre les chaînes de temps de chargement.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Créer une marketplace](/docs/fr/plugins/create-marketplace) : créez une marketplace à partir de ces champs et installez-la localement
* [Héberger et maintenir une marketplace](/docs/fr/plugins/host-marketplace) : où mettre le fichier et comment les utilisateurs reçoivent les modifications
* [Référence du manifeste de plugin](/docs/fr/plugins/manifest-reference) : les champs `plugin.json` qu'une entrée peut remplacer
* [Gérer les plugins pour votre organisation](/docs/fr/plugins/org) : recettes de liste blanche et liste noire qui utilisent ces valeurs source
