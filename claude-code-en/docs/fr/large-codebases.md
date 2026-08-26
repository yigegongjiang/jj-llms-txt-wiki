> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer Claude Code dans un monorepo ou un grand dépôt de code

> Configurez Claude Code pour les monorepos et les grands dépôts à arborescence unique avec des fichiers CLAUDE.md imbriqués, des worktrees clairsemés, l'intelligence du code et des skills par package afin que Claude reste concentré sur le code sur lequel vous travaillez.

Un grand dépôt de code peut être un seul référentiel avec des millions de lignes ou un monorepo avec de nombreux packages. Claude Code fonctionne à n'importe quelle échelle, mais à mesure que le dépôt de code grandit, les paramètres par défaut optimisés pour les petits projets peuvent remplir la fenêtre de contexte avec des instructions et des lectures de fichiers sans rapport avec la tâche, ce qui coûte des tokens et dégrade les performances de Claude.

Ce guide montre aux développeurs individuels et aux équipes d'ingénierie comment limiter Claude à la partie du dépôt de code qu'une tâche touche. Chaque section note si un paramètre est personnel à votre machine ou validé dans le référentiel.

<h2 id="what-this-guide-covers">
  Ce que ce guide couvre
</h2>

Le [tableau ci-dessous](#settings-on-this-page) répertorie chaque paramètre et ce qu'il accomplisse. L'[arborescence de fichiers après](#the-example-monorepo) est le monorepo d'exemple auquel chaque exemple de code sur cette page se réfère.

<h3 id="settings-on-this-page">
  Paramètres sur cette page
</h3>

Chaque paramètre ci-dessous est indépendant. Ils se superposent plutôt que de se remplacer, appliquez donc ceux qui conviennent à votre référentiel. [Choisir où démarrer Claude](#choose-where-to-start-claude) détermine où vivent vos fichiers de paramètres, lisez-le d'abord. [Assembler le tout](#put-it-together) les montre tous combinés.

| Je veux                                                                                                                    | Utiliser                                                                                      |
| :------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------- |
| Charger uniquement les conventions pour le code que vous touchez, au lieu d'un fichier racine couvrant chaque sous-système | Fichiers [CLAUDE.md](#layer-claude-md-files-by-directory) par répertoire                      |
| Exclure les fichiers CLAUDE.md pour les packages dans lesquels vous ne travaillez jamais                                   | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files)                                     |
| Empêcher Claude d'ouvrir la sortie de compilation, le code généré et les dépendances vendues                               | Règles de refus [`Read`](#block-reads-of-generated-and-vendored-code) dans `permissions.deny` |
| Trouver la définition d'un symbole ou les appelants via le serveur de langage au lieu de scanner les fichiers              | Un [plugin d'intelligence du code](#reduce-file-reads-with-code-intelligence)                 |
| Vérifier uniquement les répertoires dont une tâche a besoin lorsque Claude crée un worktree                                | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need)                            |
| Lire et modifier un package frère ou un autre référentiel à partir de la même session                                      | [`--add-dir`](#grant-access-across-packages-or-repositories) ou `additionalDirectories`       |
| Donner à Claude des procédures spécifiques à une zone qui se chargent uniquement lorsqu'elles sont pertinentes             | [Skills](#add-per-directory-skills) par répertoire                                            |
| Remplacer de nombreux fichiers CLAUDE.md par répertoire par un ensemble de conventions que tout le monde installe          | Un [plugin](#centralize-conventions-when-layering-stops-scaling) dans une marketplace interne |

<Tip>
  Pour les techniques de flux de travail qui gardent le contexte petit dans n'importe quel référentiel, comme [exécuter l'exploration dans un sous-agent](/docs/fr/best-practices#use-subagents-for-investigation) afin que les lectures de fichiers restent en dehors de la conversation principale, voir [Meilleures pratiques pour Claude Code](/docs/fr/best-practices). Pour déployer une configuration de base à chaque développeur de votre organisation, voir [Configurer Claude Code pour votre organisation](/docs/fr/admin-setup).
</Tip>

<h3 id="the-example-monorepo">
  Le monorepo d'exemple
</h3>

Les exemples tout au long de cette page se réfèrent à un monorepo avec trois packages. Les mêmes modèles fonctionnent dans un grand dépôt à arborescence unique : où un exemple utilise `packages/api/`, substituez votre propre répertoire de sous-système tel que `src/backend/` ou `lib/core/`.

```text theme={null}
monorepo/
  CLAUDE.md                     # instructions racine
  packages/
    api/
      CLAUDE.md                 # instructions spécifiques à l'API
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # instructions spécifiques au frontend
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # instructions de la bibliothèque partagée
      src/
```

<h2 id="choose-where-to-start-claude">
  Choisir où démarrer Claude
</h2>

L'endroit où vous lancez `claude` détermine quels fichiers Claude peut lire et modifier sans accord de permission supplémentaire, quels fichiers CLAUDE.md se chargent dans le contexte au démarrage et quels paramètres de projet s'appliquent.

| Démarrer à partir de  | Accès aux fichiers                                           | CLAUDE.md chargé au lancement                                                                      | Utiliser quand                                                |
| :-------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| Racine du référentiel | Chaque fichier                                               | Racine uniquement ; les fichiers de sous-répertoire se chargent à la demande lorsque Claude lit là | Les tâches s'étendent sur plusieurs packages ou sous-systèmes |
| Un sous-répertoire    | Ce sous-arbre uniquement, jusqu'à ce que vous accordiez plus | Celui de ce répertoire plus chaque ancêtre                                                         | Le travail est limité à un package ou un sous-système         |

Les paramètres de projet dans `.claude/settings.json` se chargent uniquement à partir de votre répertoire de démarrage et ne sont pas hérités des répertoires parents de la même manière que les fichiers CLAUDE.md : un `.claude/settings.json` à la racine du référentiel s'applique uniquement lorsque vous démarrez à partir de la racine.

Chaque section ci-dessous indique si son fichier de paramètres appartient à la racine du référentiel ou au sous-répertoire à partir duquel vous démarrez, et s'il est validé ou conservé localement.

<h2 id="layer-claude-md-files-by-directory">
  Superposer les fichiers CLAUDE.md par répertoire
</h2>

Dans un grand dépôt de code, un seul CLAUDE.md à la racine du référentiel tend soit à grandir pour couvrir les conventions de chaque sous-système, coûtant du contexte sur les instructions sans rapport avec la tâche actuelle, soit à rester trop générique pour être utile. Diviser les instructions entre des fichiers par répertoire signifie que Claude charge les règles à l'échelle du référentiel plus uniquement les conventions pour le code sur lequel vous travaillez.

Claude Code charge chaque fichier [CLAUDE.md](/docs/fr/memory) à partir de votre répertoire de travail et de chaque répertoire parent au lancement, puis charge le fichier de chaque sous-répertoire à la demande lorsqu'il lit des fichiers là. Un fichier racine définit les règles à l'échelle du référentiel et chaque sous-répertoire ajoute les siennes.

Une division courante est deux niveaux :

* **CLAUDE.md racine** : instructions qui s'appliquent partout, comme les normes de codage, les conventions de commit et la disposition du référentiel
* **CLAUDE.md par sous-répertoire** : conventions spécifiques à la pile de cette zone. Dans un monorepo, c'est un par package. Dans un grand arbre unique, c'est un par sous-système tel que `src/db/` ou `src/api/`

Validez ces fichiers dans le référentiel afin que les coéquipiers les héritent. Le propriétaire de chaque répertoire maintient généralement son fichier.

Le CLAUDE.md racine oriente Claude vers la structure du référentiel :

```markdown CLAUDE.md theme={null}
Ceci est un monorepo avec trois packages sous packages/ :

- packages/api : API REST Node.js avec Express, TypeScript et PostgreSQL
- packages/web : frontend React avec Vite, TypeScript et TailwindCSS
- packages/shared : utilitaires TypeScript partagés utilisés par api et web

Exécutez les commandes à partir du répertoire du package, pas de la racine du monorepo.
Chaque package a son propre tsconfig.json, package.json et suite de tests.
```

Le CLAUDE.md de chaque sous-répertoire, ici `packages/api/CLAUDE.md`, ajoute du contexte spécifique à la pile de cette zone :

```markdown packages/api/CLAUDE.md theme={null}
Ce package est le serveur API REST.

- Exécuter les tests : `npm test` (utilise Vitest)
- Exécuter le serveur de développement : `npm run dev` (port 3001)
- Migrations de base de données : `npm run migrate`
- Variables d'environnement : copiez `.env.example` en `.env`

Les routes API sont dans src/routes/. Chaque fichier de route exporte un routeur Express.
Les requêtes de base de données utilisent Knex dans src/db/. N'écrivez jamais de chaînes SQL brutes dans les gestionnaires de routes.
```

Lorsque vous démarrez Claude à partir de `packages/api/`, il charge à la fois `packages/api/CLAUDE.md` et le CLAUDE.md racine. Claude voit les instructions locales aux côtés des règles à l'échelle du référentiel, sans instructions de `packages/web/` dans le contexte. La même chose s'applique à n'importe quel sous-répertoire dans un arbre non-monorepo.

Quelques façons de garder les fichiers à jour à mesure que le dépôt de code et les modèles changent :

* **Examiner dans les demandes de tirage** : traitez les modifications CLAUDE.md comme n'importe quel autre changement de documentation afin que les conventions suivent le code
* **Revisiter après les versions majeures du modèle** : les instructions qui contournaient une limitation d'un modèle plus ancien peuvent devenir une surcharge une fois qu'un modèle plus récent gère le cas de lui-même. Par exemple, une règle qui force les refactorisations de fichier unique peut être supprimée une fois que la limitation est disparue
* **Ajouter un hook Stop qui propose des mises à jour** : un [hook `Stop`](/docs/fr/hooks#stop) reçoit le chemin de la transcription de session lorsque Claude termine sa réponse, donc un script peut examiner la session et proposer des mises à jour CLAUDE.md tandis que l'écart qu'il a exposé est frais

Pour plus d'informations sur la façon dont les fichiers CLAUDE.md se chargent et interagissent, voir [Mémoire et instructions de projet](/docs/fr/memory).

<h3 id="choose-between-per-directory-claude-md-and-path-scoped-rules">
  Choisir entre CLAUDE.md par répertoire et règles limitées au chemin
</h3>

Les fichiers CLAUDE.md par répertoire et les [règles limitées au chemin](/docs/fr/memory#path-specific-rules) sous `.claude/rules/` vous permettent tous deux de cibler les instructions vers une partie de l'arbre. Ils diffèrent par l'endroit où le fichier vit et quand il se charge.

| Approche                                      | Emplacement du fichier                             | Se charge quand                                                                                          | Utiliser quand                                                                                                           |
| :-------------------------------------------- | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| CLAUDE.md par répertoire                      | À l'intérieur du répertoire, aux côtés de son code | Au lancement lorsque démarré à partir de ce répertoire, ou à la demande lorsque Claude lit un fichier là | Les propriétaires de répertoires maintiennent leurs propres conventions ; les instructions sont versionnées avec le code |
| Règle limitée au chemin dans `.claude/rules/` | `.claude/` central à la racine du repo             | Lorsque Claude travaille avec un fichier correspondant au glob `paths:` de la règle                      | Vous voulez toutes les conventions au même endroit, ou la même règle s'applique à de nombreux chemins dispersés          |

Pour une comparaison qui couvre également les skills, voir [Comparer les fonctionnalités similaires](/docs/fr/features-overview#compare-similar-features).

<h3 id="exclude-irrelevant-claude-md-files">
  Exclure les fichiers CLAUDE.md non pertinents
</h3>

Lorsque vous démarrez Claude à partir de la racine du référentiel, le CLAUDE.md de chaque sous-répertoire se charge dès que Claude lit un fichier dans ce répertoire. Le paramètre `claudeMdExcludes` ignore les fichiers spécifiques par chemin ou modèle glob afin qu'ils ne se chargent jamais.

Utilisez ceci pour les répertoires dans lesquels vous ne travaillez jamais, comme les packages d'autres équipes, le code hérité ou les sous-arbres vendus. La liste d'exclusion est statique, pas un commutateur par tâche. Pour vous concentrer sur un package aujourd'hui et un autre demain, [démarrez Claude à partir du répertoire de ce package](#choose-where-to-start-claude) au lieu de modifier les exclusions.

Si vous ne voulez que ces exclusions pour vous-même, mettez le paramètre dans `.claude/settings.local.json`. Claude Code gitignore ce fichier lorsqu'il le crée ; puisque vous le créez à la main ici, ajoutez-le à votre gitignore. Les modèles utilisent la syntaxe glob correspondant aux chemins de fichiers absolus, donc commencez les modèles de style relatif avec `**/` pour correspondre n'importe où dans l'arbre. L'exemple ci-dessous exclut les packages appartenant à d'autres équipes :

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/admin-dashboard/**",
    "**/packages/legacy-*/**"
  ]
}
```

Cela ignore chaque CLAUDE.md et fichier de règles sous ces packages. Le CLAUDE.md racine et les packages dans lesquels vous travaillez se chargent toujours normalement.

Ces modèles couvrent d'autres cas courants :

* `"**/packages/*/CLAUDE.md"` : exclut le CLAUDE.md de chaque package tout en gardant la racine
* `"**/packages/web/**"` : exclut tout sous le package web, y compris les règles
* `"/home/user/monorepo/legacy/CLAUDE.md"` : exclut un fichier spécifique par chemin absolu

Les fichiers CLAUDE.md de politique gérée ne peuvent pas être exclus, donc les instructions à l'échelle de l'organisation s'appliquent toujours. Vous pouvez définir `claudeMdExcludes` à n'importe quel [scope de paramètres](/docs/fr/settings#configuration-scopes) : utilisateur, projet, local ou géré. Les tableaux fusionnent entre les scopes, donc une équipe peut définir les valeurs par défaut au niveau du projet tandis que les individus ajoutent des remplacements locaux.

Pour la documentation complète d'exclusion, voir [Exclure les fichiers CLAUDE.md spécifiques](/docs/fr/memory#exclude-specific-claude-md-files).

<h2 id="reduce-what-claude-reads">
  Réduire ce que Claude lit
</h2>

Les instructions ne sont qu'une partie de ce qui se retrouve dans le contexte de Claude. Les lectures de fichiers sont un autre coût qui grandit avec le dépôt de code. Les paramètres ci-dessous bloquent les lectures des chemins non pertinents et remplacent les scans exhaustifs de fichiers par des recherches de serveur de langage.

<h3 id="block-reads-of-generated-and-vendored-code">
  Bloquer les lectures du code généré et vendu
</h3>

Les recherches de contenu de Claude respectent `.gitignore` par défaut, donc les chemins déjà listés là, comme `node_modules/`, `dist/` et `build/`, restent en dehors des résultats de recherche sans configuration supplémentaire.

Pour les chemins qui sont validés, comme un SDK vendu ou du code généré validé, ajoutez des règles de refus `Read` dans `permissions.deny` pour empêcher Claude d'ouvrir ces fichiers même lorsqu'une recherche les liste.

Pour appliquer ces exclusions à tous ceux qui travaillent dans le référentiel, validez-les dans `.claude/settings.json`. Pour les garder personnelles, utilisez `.claude/settings.local.json` à la place. Comme d'autres paramètres de projet sur cette page, ces fichiers se chargent uniquement à partir de votre répertoire de démarrage. Placez-les à la racine du référentiel si vous démarrez Claude là, ou dans le `.claude/` de chaque package si vous démarrez à partir de sous-répertoires. Pour appliquer les mêmes règles de refus dans chaque session indépendamment du répertoire de démarrage, définissez-les dans [paramètres gérés](/docs/fr/settings#settings-files), que les paramètres utilisateur et projet ne peuvent pas remplacer.

L'exemple ci-dessous bloque les artefacts de compilation et un SDK vendu :

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)",
      "Read(./**/*.generated.*)",
      "Read(./vendor/**)"
    ]
  }
}
```

Les règles de refus couvrent les outils de fichiers intégrés de Claude et les commandes Bash reconnues, y compris `cat`, `head`, `grep` et `find`, lorsqu'un chemin refusé est passé comme argument. Elles ne filtrent pas les chemins refusés de la sortie d'une recherche récursive, et elles ne couvrent pas les sous-processus arbitraires qui ouvrent les fichiers eux-mêmes. Pour la syntaxe de modèle complète, voir [Règles de permission Read et Edit](/docs/fr/permissions#read-and-edit).

<h3 id="reduce-file-reads-with-code-intelligence">
  Réduire les lectures de fichiers avec l'intelligence du code
</h3>

Dans un grand dépôt de code, trouver où un symbole est défini ou utilisé peut coûter de nombreuses lectures de fichiers et appels grep. Les [plugins d'intelligence du code](/docs/fr/discover-plugins#code-intelligence) connectent Claude à un serveur de langage afin qu'il puisse sauter aux définitions, trouver des références et afficher les erreurs de type directement au lieu de scanner l'arbre.

La marketplace officielle a des plugins pour TypeScript, Python, Go, Rust et d'autres langages courants. L'exemple ci-dessous installe le plugin TypeScript :

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

Pour activer un plugin pour tout le monde dans le référentiel plutôt que de l'installer vous-même, ajoutez-le au paramètre de projet [`enabledPlugins`](/docs/fr/settings#plugin-settings).

Les plugins d'intelligence du code nécessitent le binaire du serveur de langage de la langue sur la machine de chaque développeur. Voir [quel binaire chaque langue nécessite](/docs/fr/discover-plugins#code-intelligence). L'installation à partir de la marketplace officielle nécessite un accès réseau à GitHub, où la marketplace est hébergée. Sur un réseau restreint, [ajoutez la marketplace à partir d'un hôte Git interne ou d'un chemin local](/docs/fr/discover-plugins#add-from-other-git-hosts) à la place.

Cela s'associe bien avec `claudeMdExcludes` et les règles `Read` deny ci-dessus. Ceux-ci gardent le contenu non pertinent en dehors du contexte, et l'intelligence du code empêche Claude de lire ce qui reste pour localiser une définition.

<h2 id="scope-worktrees-and-file-access">
  Limiter les worktrees et l'accès aux fichiers
</h2>

Ces paramètres contrôlent ce qui est sur le disque dans les worktrees et quels répertoires Claude peut lire et écrire au-delà de votre point de démarrage.

<h3 id="check-out-only-the-directories-you-need">
  Vérifier uniquement les répertoires dont vous avez besoin
</h3>

Le flag `--worktree` démarre une session dans un nouveau git worktree afin que les modifications restent isolées de votre checkout principal. Par défaut, il vérifie l'ensemble du référentiel. Dans un grand référentiel, le paramètre `worktree.sparsePaths` utilise git sparse-checkout pour écrire uniquement les répertoires listés plus les fichiers au niveau racine sur le disque, afin que les worktrees démarrent plus rapidement et utilisent moins d'espace.

Si tout le monde travaillant dans ce répertoire a besoin des mêmes chemins, validez le paramètre dans `.claude/settings.json`. Pour ajouter des chemins pour vous-même, utilisez `.claude/settings.local.json` : les listes fusionnent entre les scopes, donc un fichier local peut ajouter des chemins à la liste validée mais pas les supprimer. L'exemple ci-dessous montre le fichier validé :

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

Lorsque Claude crée un worktree, il vérifie uniquement `.claude/`, `packages/api/` et `packages/shared/` au lieu de l'arbre complet. Les chemins dans `sparsePaths` sont relatifs à la racine du référentiel, indépendamment du sous-répertoire à partir duquel vous démarrez Claude. N'importe quel chemin de répertoire fonctionne ici, pas seulement les racines de package.

Ceci est particulièrement utile pour [l'isolation des worktrees de sous-agent](/docs/fr/worktrees#isolate-subagents-with-worktrees). Les sous-agents sont des instances Claude parallèles générées pour les sous-tâches, et chacun qui s'exécute dans un worktree obtient un checkout léger au lieu de l'arbre complet. Tous les worktrees dans une session partagent le même `sparsePaths`, donc si un sous-agent a besoin de `packages/api/` et un autre de `packages/web/`, listez les deux.

Listez les répertoires dans `sparsePaths`, pas les fichiers individuels. Les fichiers au niveau racine comme `package.json`, `tsconfig.base.json` et les fichiers de verrouillage sont toujours vérifiés aux côtés des répertoires que vous listez. Les répertoires au niveau racine ne le sont pas, donc incluez `.claude` dans la liste si vous voulez que le `.claude/settings.json`, `.claude/rules/` ou `.claude/skills/` de la racine du référentiel soit disponible à l'intérieur du worktree.

Le sparse checkout nécessite que git active `extensions.worktreeConfig` dans le `.git/config` partagé du référentiel tandis qu'un worktree sparse existe. Claude Code supprime cette entrée après la suppression du dernier worktree, mais uniquement si Claude Code l'a ajoutée. Il ne supprime jamais une valeur que vous avez définie vous-même. Avant la v2.1.207, l'entrée restait après la suppression du dernier worktree, et les outils basés sur go-git comme `tea` ne pouvaient pas ouvrir le référentiel jusqu'à ce que vous exécutiez `git config --unset extensions.worktreeConfig`.

Pour éviter de dupliquer les grands répertoires comme `node_modules` entre les worktrees, associez `sparsePaths` avec `symlinkDirectories` dans le même `.claude/settings.json` :

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

Cela crée un symlink du `node_modules/` de chaque worktree vers la copie du référentiel principal plutôt que de le dupliquer sur le disque.

<Note>
  Les paramètres `sparsePaths` et `symlinkDirectories` sont lus à partir de votre répertoire de démarrage avant la création du worktree. Après la création, le répertoire de travail de la session est la racine du worktree, pas le sous-répertoire à partir duquel vous avez lancé. Les paramètres de projet à l'intérieur du worktree se chargent donc à partir du `.claude/settings.json` de la racine du worktree, la copie validée du fichier de la racine du référentiel. Mettez tous les autres paramètres dont vous avez besoin à l'intérieur des worktrees, comme les règles de permission ou les hooks, dans le `.claude/settings.json` de la racine du référentiel.
</Note>

Pour la référence complète des paramètres de worktree, voir [Paramètres de worktree](/docs/fr/settings#worktree-settings).

<h3 id="grant-access-across-packages-or-repositories">
  Accorder l'accès entre les packages ou les référentiels
</h3>

Cette section s'applique lorsque vous démarrez Claude à partir d'un sous-répertoire, ou lorsqu'une tâche s'étend sur plusieurs checkouts. Si vous démarrez à partir de la racine du référentiel dans un grand arbre unique, Claude a déjà accès à chaque fichier et vous pouvez ignorer ceci.

Lorsque vous démarrez Claude à partir de `packages/api/`, il peut lire et écrire des fichiers dans ce répertoire. Si une tâche nécessite des modifications entre les packages, comme la mise à jour d'un type partagé que `api` et `web` importent tous les deux, vous devez accorder l'accès au répertoire frère. Le même mécanisme accorde l'accès à un référentiel séparé.

Le paramètre `additionalDirectories` dans `.claude/settings.json` donne à Claude l'accès aux répertoires en dehors du répertoire de travail. L'exemple ci-dessous accorde l'accès à deux packages frères :

```json .claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

Les chemins relatifs se résolvent par rapport au répertoire à partir duquel vous démarrez Claude. Avec cette configuration, Claude peut lire et modifier les fichiers dans `packages/shared/` et `packages/web/` tout en travaillant à partir de `packages/api/`.

Vous pouvez également accorder l'accès à l'exécution sans modifier les paramètres en passant `--add-dir` lorsque vous démarrez Claude :

```bash theme={null}
claude --add-dir ../shared
```

Peu importe comment vous ajoutez un répertoire, Claude peut lire et modifier les fichiers qu'il contient. Le fait que le CLAUDE.md du répertoire, les fichiers `.claude/rules/` et les skills se chargent également dépend de la façon dont vous l'avez ajouté :

| Ajouté avec                             | Charge CLAUDE.md et les règles                         | Charge les skills |
| :-------------------------------------- | :----------------------------------------------------- | :---------------- |
| Paramètre `additionalDirectories`       | Jamais                                                 | Jamais            |
| Flag `--add-dir` ou commande `/add-dir` | Uniquement avec la variable d'environnement ci-dessous | Oui               |

Pour charger les fichiers CLAUDE.md et les règles à partir d'un répertoire ajouté avec `--add-dir` ou `/add-dir`, définissez la variable d'environnement `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD` :

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

La variable d'environnement n'a aucun effet sur les répertoires listés dans le paramètre `additionalDirectories`. Voir [Charger à partir de répertoires supplémentaires](/docs/fr/memory#load-from-additional-directories) pour les détails.

Pour les répertoires frères que tout le monde dans cette zone a besoin, validez `additionalDirectories` dans `.claude/settings.json`. Pour une sélection personnelle ou un accès ponctuel, utilisez `.claude/settings.local.json` ou passez `--add-dir` au lancement.

<h2 id="add-per-directory-skills">
  Ajouter des skills par répertoire
</h2>

N'importe quel sous-répertoire peut définir des [skills](/docs/fr/skills) limités à sa propre pile. Un skill se charge à la demande lorsque Claude détermine qu'il est pertinent, donc les outils spécifiques à l'API ne consomment pas de contexte pendant le travail frontend.

Les skills vivent sous `.claude/skills/` à l'intérieur du répertoire. Validez-les aux côtés du code de cette zone afin que quiconque clone le référentiel les obtienne. Dans un monorepo, cela peut être un ensemble de skills par package. Dans un grand dépôt à arborescence unique, c'est un ensemble par sous-système tel que `src/db/.claude/skills/`.

Créez un répertoire de skill à l'intérieur du sous-répertoire :

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

Puis écrivez `SKILL.md` à l'intérieur de ce répertoire, ici `packages/api/.claude/skills/api-testing/SKILL.md`. Cet exemple enseigne à Claude les modèles de test du package API :

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: Modèles de test pour le package API. Utiliser lors de l'écriture ou de la modification des tests dans packages/api/.
---

## Structure des tests

Les tests sont dans `src/__tests__/` reflétant la structure du répertoire `src/`.
Chaque fichier de route a un fichier `.test.ts` correspondant.

## Exécuter les tests

- Tous les tests : `npm test`
- Fichier unique : `npm test -- src/__tests__/routes/users.test.ts`
- Mode watch : `npm test -- --watch`

## Utilitaires de test

- `src/__tests__/helpers/db.ts` : fournit `setupTestDb()` et `teardownTestDb()` pour les tests de base de données
- `src/__tests__/helpers/auth.ts` : fournit `createTestUser()` et `getAuthToken()` pour les endpoints authentifiés

## Modèles

- Utilisez `supertest` pour les assertions HTTP, pas fetch brut
- Enveloppez toujours les tests de base de données dans une transaction qui se restaure
- Simulez les services externes dans `src/__tests__/mocks/`
```

Un sous-répertoire différent contient différents skills de la même manière : `packages/web/.claude/skills/component-patterns/` décrit les conventions de composants du frontend au lieu des tests. Lorsque Claude travaille sur un fichier dans `packages/api/`, il charge le skill api-testing. Lorsqu'il travaille dans `packages/web/`, il charge component-patterns à la place. Les skills d'aucun répertoire ne se chargent pendant les tâches de l'autre.

Vous pouvez également limiter un skill par modèle de fichier au lieu de par placement. Le [champ frontmatter `paths`](/docs/fr/skills#frontmatter-reference) prend des modèles glob, et Claude charge le skill automatiquement uniquement lorsqu'il travaille avec des fichiers correspondants. Utilisez ceci pour un skill qui vit dans le `.claude/skills/` de la racine du référentiel mais s'applique uniquement à certains fichiers où qu'ils apparaissent, comme un skill de migration de base de données limité à `**/migrations/**`.

Pour plus d'informations sur la création et l'organisation des skills, voir [Skills](/docs/fr/skills).

<h3 id="keep-skills-discoverable">
  Garder les skills découvrables
</h3>

Avec les skills dispersés dans de nombreux répertoires, la liste parmi laquelle Claude choisit peut devenir grande. Claude choisit un skill en lisant le nom et la description de chaque skill découvert, et seul le contenu complet du skill choisi se charge dans le contexte. Cette section couvre comment garder cette liste petite et écrire des descriptions qui survivent au raccourcissement.

Quels skills sont en scope dépend de l'endroit où vous démarrez Claude :

* **À partir d'un sous-répertoire tel que `packages/api/`** : skills de ce répertoire, chaque parent jusqu'à la racine du référentiel, et les niveaux utilisateur et entreprise
* **À partir de la racine du référentiel** : skills de chaque sous-répertoire que Claude touche pendant la session, ce qui peut s'accumuler en centaines
* **Après avoir ajouté un frère avec [`--add-dir`](#grant-access-across-packages-or-repositories)** : les skills de ce frère se chargent aussi. Le paramètre `additionalDirectories` accorde uniquement l'accès aux fichiers et ne charge pas les skills

Les noms se chargent toujours, mais [les descriptions sont raccourcies lorsqu'il y en a beaucoup](/docs/fr/skills#skill-descriptions-are-cut-short), ce qui peut supprimer les mots-clés que Claude utilise pour décider si un skill s'applique. Gardez les descriptions courtes et commencez par les mots qu'une demande contiendrait, comme « écrire ou modifier les tests dans `packages/api/` ».

Pour les skills que de nombreux répertoires partagent, comme les conventions PR ou une liste de contrôle de déploiement, placez-les dans le `.claude/skills/` de la racine du référentiel afin qu'ils se chargent à partir de n'importe quel répertoire de démarrage. Lorsque les skills partagés ont besoin de leur propre historique de version ou doivent fonctionner entre les référentiels, empaquetez-les en tant que [plugin](/docs/fr/plugins) à la place. Les skills de plugin utilisent un espace de noms `plugin-name:skill-name`, donc ils ne collisionnent jamais avec les skills par répertoire. Une équipe de plateforme peut les versionner et les mettre à jour au même endroit.

Pour trouver quels skills restent inutilisés, activez l'exportateur OpenTelemetry [logs](/docs/fr/monitoring-usage) et définissez `OTEL_LOG_TOOL_DETAILS=1` afin que les noms de skills soient enregistrés textuellement au lieu d'être masqués. L'événement [`skill_activated`](/docs/fr/monitoring-usage#skill-activated-event) enregistre chaque invocation dans son attribut `skill.name`, et `invocation_trigger` enregistre si une commande, Claude ou un skill imbriqué l'a invoqué, ce qui vous dit quoi consolider ou retirer.

<h2 id="centralize-conventions-when-layering-stops-scaling">
  Centraliser les conventions lorsque la superposition cesse de s'adapter
</h2>

Les fichiers CLAUDE.md par répertoire peuvent devenir difficiles à gouverner à mesure que le dépôt de code grandit. Les conventions dérivent, les fichiers deviennent obsolètes, et personne ne possède la racine. Résoudre cela incombe généralement à l'équipe qui maintient la configuration Claude Code du référentiel plutôt qu'à chaque développeur travaillant dans sa propre zone.

Déplacez les conventions et le contenu de référence en dehors du CLAUDE.md toujours chargé et dans les mécanismes qui se chargent à la demande :

* [Skills](/docs/fr/skills) : matériel de référence que Claude charge uniquement lorsqu'il est pertinent pour la tâche
* [Plugins](/docs/fr/plugins) : bundles versionnés de skills, hooks et commandes qu'une équipe de plateforme possède centralement
* [Serveurs MCP](/docs/fr/mcp) : si votre organisation exécute déjà une recherche de code ou un index RAG sur le référentiel, exposez-le en tant qu'outil MCP afin que Claude l'interroge au lieu de lire les fichiers directement

Voir [paramètres gérés par serveur ou gérés par endpoint](/docs/fr/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings) pour la façon dont les équipes de plateforme peuvent les appliquer centralement.

<h3 id="recommend-the-right-plugin-at-session-start">
  Recommander le bon plugin au démarrage de la session
</h3>

Une fois que les conventions vivent dans les plugins, un coéquipier démarrant Claude dans une partie inconnue de l'arbre n'a aucun signal sur quel plugin les propriétaires de cette zone maintiennent. Un [hook `SessionStart`](/docs/fr/hooks#sessionstart) peut combler cet écart, puisque tout ce que le hook imprime sur stdout est ajouté au contexte de Claude avant la première invite.

Par exemple, vous pouvez écrire un script qui lit le répertoire de lancement à partir de [l'entrée du hook](/docs/fr/hooks#common-input-fields), le recherche dans une carte chemin-vers-plugin validée dans le référentiel, et imprime la recommandation pour que Claude la relaye dans sa première réponse. Voir [Automatiser les actions avec les hooks](/docs/fr/hooks-guide) pour écrire et enregistrer le hook.

<h2 id="put-it-together">
  Assembler le tout
</h2>

La configuration combinée ci-dessous utilise la disposition du monorepo. Les mêmes fichiers fonctionnent pour n'importe quel sous-répertoire dans un grand arbre unique. Les paramètres de projet se chargent uniquement à partir du répertoire à partir duquel vous démarrez Claude, donc le `.claude/settings.json` de chaque sous-répertoire doit être autonome plutôt que superposé sur un fichier racine.

L'exemple valide `worktree`, `additionalDirectories` et les règles `Read` deny dans `.claude/settings.json` afin que chaque développeur dans `packages/api/` obtienne le même accès aux frères, les chemins clairsemés et les exclusions. Le fichier ci-dessous est les paramètres validés par zone pour `packages/api/` :

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

Parce que cette session démarre à partir de `packages/api/`, les fichiers CLAUDE.md des packages frères sont déjà hors de portée, donc `claudeMdExcludes` n'est pas nécessaire ici. Ajoutez-le au `.claude/settings.local.json` de la racine du référentiel à la place si vous démarrez également des sessions à partir de la racine.

L'entrée `additionalDirectories` s'applique lorsque vous démarrez Claude à partir de `packages/api/` directement. À l'intérieur d'un worktree créé à partir de cette session, le répertoire de travail est la racine du worktree, donc ce fichier de paramètres ne se charge pas. Les packages frères sont déjà accessibles à l'intérieur du worktree sans lui, mais les règles de refus ont besoin d'une deuxième copie dans le `.claude/settings.json` de la racine du référentiel afin que les sessions de worktree les récupèrent, comme la [note des paramètres de worktree](#check-out-only-the-directories-you-need) le décrit :

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

Après la configuration, le référentiel a cette disposition :

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # règles de refus pour les sessions de worktree
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # worktree, additionalDirectories, règles de refus
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

Avec cette configuration, en démarrant Claude à partir de `packages/api/` :

* Charge le CLAUDE.md racine et `packages/api/CLAUDE.md`, ignore `packages/web/CLAUDE.md`
* Peut lire et modifier les fichiers dans `packages/api/` et `packages/shared/`
* Ignore les lectures de la sortie de compilation sous `dist/` et `build/` dans `packages/api/`
* A le skill api-testing disponible à la demande
* Crée des worktrees contenant `.claude/`, `packages/api/`, `packages/shared/` et les fichiers au niveau racine, avec les règles de refus appliquées dans le worktree à partir du fichier de paramètres racine

<h2 id="scope-and-plan-changes-that-span-packages">
  Limiter et planifier les modifications qui s'étendent sur les packages
</h2>

La configuration ci-dessus contrôle ce que Claude voit. Lorsqu'une seule modification touche plusieurs packages, comme la mise à jour d'un type partagé avec chaque site d'appel qui l'utilise, la façon dont vous limitez et séquencez la tâche affecte également le résultat.

Deux techniques aident à garder une modification entre packages cohérente :

* **Donner à Claude le changement entier dans une session** : remettre l'édition partagée et ses sites d'appel ensemble garde les décisions derrière chaque édition cohérentes, plutôt que de les redériver par package
* **Enregistrer le plan dans un fichier avant d'éditer** : [planifiez d'abord](/docs/fr/best-practices#explore-first-then-plan-then-code) et demandez à Claude d'écrire le plan dans un fichier markdown dans le référentiel. Une longue session entre packages [compacte son contexte](/docs/fr/context-window#what-survives-compaction) en cours de route, et le plan enregistré survit où l'historique de conversation peut ne pas le faire

<h2 id="next-steps">
  Prochaines étapes
</h2>

Une fois cette configuration en place, vous pouvez l'affiner :

* Utilisez les [hooks](/docs/fr/hooks-guide) pour exécuter les linters ou les vérificateurs de type par répertoire après que Claude modifie les fichiers
* Examinez [Gérer les coûts efficacement](/docs/fr/costs) pour comprendre comment la taille du dépôt de code affecte l'utilisation des tokens et comment définir les limites de dépenses avant un déploiement plus large
* Lisez [Comment Claude Code fonctionne dans les grands dépôts de code](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start) sur le blog Claude pour les modèles de déploiement organisationnel et les modèles de propriété qui se situent au-dessus de la configuration par référentiel sur cette page
