> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Créer et distribuer une place de marché de plugins

> Créez et hébergez des places de marché de plugins pour distribuer les extensions Claude Code dans vos équipes et communautés.

Une **place de marché de plugins** est un catalogue qui vous permet de distribuer des plugins à d'autres. Les places de marché offrent une découverte centralisée, un suivi des versions, des mises à jour automatiques et la prise en charge de plusieurs types de sources, notamment les dépôts git et les chemins locaux. Ce guide vous montre comment créer votre propre place de marché pour partager des plugins avec votre équipe ou votre communauté.

Vous cherchez à installer des plugins à partir d'une place de marché existante ? Consultez [Découvrir et installer des plugins préconfigurés](/docs/fr/discover-plugins).

<h2 id="overview">
  Aperçu
</h2>

La création et la distribution d'une place de marché impliquent :

1. **Créer des plugins** : créez un ou plusieurs plugins avec des compétences, des agents, des hooks, des serveurs MCP ou des serveurs LSP. Ce guide suppose que vous avez déjà des plugins à distribuer ; consultez [Créer des plugins](/docs/fr/plugins) pour plus de détails sur la création de plugins.
2. **Créer le fichier de place de marché** : définissez un `marketplace.json` qui répertorie vos plugins et où les trouver. Voir [Créer le fichier de place de marché](#create-the-marketplace-file).
3. **Héberger la place de marché** : poussez vers GitHub, GitLab ou un autre hôte git. Voir [Héberger et distribuer les places de marché](#host-and-distribute-marketplaces).
4. **Partager avec les utilisateurs** : les utilisateurs ajoutent votre place de marché avec `/plugin marketplace add` et installent des plugins individuels. Voir [Découvrir et installer des plugins](/docs/fr/discover-plugins).

Une fois votre place de marché en ligne, vous pouvez la mettre à jour en poussant les modifications vers votre dépôt. Les utilisateurs actualisent leur copie locale avec `/plugin marketplace update`.

<h2 id="walkthrough-create-a-local-marketplace">
  Procédure pas à pas : créer une place de marché locale
</h2>

Cet exemple crée une place de marché avec un plugin : une compétence `quality-review` pour les révisions de code. Vous allez créer la structure de répertoires, ajouter une compétence, créer le manifeste du plugin et le catalogue de la place de marché, puis l'installer et la tester.

<Steps>
  <Step title="Créer la structure de répertoires">
    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/skills/quality-review
    ```
  </Step>

  <Step title="Créer la compétence">
    Créez un fichier `SKILL.md` qui définit ce que fait la compétence `quality-review`.

    ```markdown my-marketplace/plugins/quality-review-plugin/skills/quality-review/SKILL.md theme={null}
    ---
    description: Review code for bugs, security, and performance
    ---

    Review the code I've selected or the recent changes for:
    - Potential bugs or edge cases
    - Security concerns
    - Performance issues
    - Readability improvements

    Be concise and actionable.
    ```
  </Step>

  <Step title="Créer le manifeste du plugin">
    Créez un fichier `plugin.json` qui décrit le plugin. Le manifeste se trouve dans le répertoire `.claude-plugin/`.

    ```json my-marketplace/plugins/quality-review-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "quality-review-plugin",
      "description": "Adds a quality-review skill for quick code reviews",
      "version": "1.0.0"
    }
    ```

    <Note>
      La définition de `version` signifie que les utilisateurs ne reçoivent des mises à jour que lorsque vous modifiez ce champ, donc augmentez-le à chaque version. Si vous omettez `version` et hébergez cette place de marché dans git, chaque commit compte automatiquement comme une nouvelle version. Consultez [Résolution de version](#version-resolution-and-release-channels) pour choisir la bonne approche.
    </Note>
  </Step>

  <Step title="Créer le fichier de place de marché">
    Créez le catalogue de la place de marché qui répertorie votre plugin.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-plugins",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "quality-review-plugin",
          "source": "./plugins/quality-review-plugin",
          "description": "Adds a quality-review skill for quick code reviews"
        }
      ]
    }
    ```
  </Step>

  <Step title="Ajouter et installer">
    Ajoutez la place de marché et installez le plugin.

    ```shell theme={null}
    /plugin marketplace add ./my-marketplace
    /plugin install quality-review-plugin@my-plugins
    ```
  </Step>

  <Step title="Essayer">
    Sélectionnez du code dans votre éditeur et exécutez votre nouvelle compétence. Les compétences des plugins sont espacées avec le nom du plugin.

    ```shell theme={null}
    /quality-review-plugin:quality-review
    ```
  </Step>
</Steps>

Pour en savoir plus sur ce que les plugins peuvent faire, notamment les hooks, les agents, les serveurs MCP et les serveurs LSP, consultez [Plugins](/docs/fr/plugins).

<Note>
  **Comment les plugins sont installés** : Lorsque les utilisateurs installent un plugin, Claude Code copie le répertoire du plugin vers un emplacement de cache. Cela signifie que les plugins ne peuvent pas référencer des fichiers en dehors de leur répertoire en utilisant des chemins comme `../shared-utils`, car ces fichiers ne seront pas copiés.

  Si vous devez partager des fichiers entre les plugins, utilisez des symlinks. Consultez [Plugin caching and file resolution](/docs/fr/plugins-reference#plugin-caching-and-file-resolution) pour plus de détails.
</Note>

<h2 id="create-the-marketplace-file">
  Créer le fichier de place de marché
</h2>

Créez `.claude-plugin/marketplace.json` à la racine de votre dépôt. Ce fichier définit le nom de votre place de marché, les informations du propriétaire et une liste de plugins avec leurs sources.

Chaque entrée de plugin a besoin au minimum d'un `name` et d'une `source` qui indique à Claude Code où la récupérer. Consultez le [schéma complet](#marketplace-schema) ci-dessous pour tous les champs disponibles.

```json theme={null}
{
  "name": "company-tools",
  "owner": {
    "name": "DevTools Team",
    "email": "devtools@example.com"
  },
  "plugins": [
    {
      "name": "code-formatter",
      "source": "./plugins/formatter",
      "description": "Automatic code formatting on save",
      "version": "2.1.0",
      "author": {
        "name": "DevTools Team"
      }
    },
    {
      "name": "deployment-tools",
      "source": {
        "source": "github",
        "repo": "company/deploy-plugin"
      },
      "description": "Deployment automation tools"
    }
  ]
}
```

<h2 id="marketplace-schema">
  Schéma de la place de marché
</h2>

<h3 id="required-fields">
  Champs obligatoires
</h3>

| Champ     | Type   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Exemple         |
| :-------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------- |
| `name`    | string | Identifiant de la place de marché (kebab-case, sans espaces). C'est un élément public : les utilisateurs le voient lors de l'installation de plugins (par exemple, `/plugin install my-tool@your-marketplace`). Chaque utilisateur ne peut enregistrer qu'une seule place de marché par nom : l'ajout d'une deuxième place de marché portant le même nom remplace la première. Pour publier plusieurs plugins sous un seul nom de place de marché, listez-les tous dans un seul [`marketplace.json`](#create-the-marketplace-file). | `"acme-tools"`  |
| `owner`   | object | Informations du responsable de la place de marché ([voir les champs ci-dessous](#owner-fields))                                                                                                                                                                                                                                                                                                                                                                                                                                     |                 |
| `plugins` | array  | Liste des plugins disponibles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Voir ci-dessous |

<Note>
  **Noms réservés** : les noms de place de marché suivants sont réservés à l'usage officiel d'Anthropic et ne peuvent pas être utilisés par les places de marché tierces : `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `claude-plugins-community`, `claude-community`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `knowledge-work-plugins`, `life-sciences`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `healthcare`. Les noms qui usurpent l'identité de places de marché officielles, comme `official-claude-plugins` ou `anthropic-plugins-v2`, sont également bloqués. La réservation de ces noms empêche une place de marché tierce de se présenter comme une source publiée par Anthropic.

  Claude Code revérifie les noms réservés chaque fois qu'il charge une place de marché, pas seulement lorsque vous en ajoutez une. Une place de marché qui a été enregistrée sous l'un de ces noms avant que le nom ne soit réservé cesse de se charger et signale qu'elle est [enregistrée à partir d'une source non fiable](/docs/fr/errors#marketplace-is-registered-from-an-untrusted-source). Supprimez cette place de marché et rajoutez-la à partir de la source officielle d'Anthropic. Une place de marché tierce affectée par un nom nouvellement réservé se charge à nouveau dès que vous la rajoutez sous un nom différent. Avant la v2.1.205, `first-party-plugins` et `healthcare` n'étaient pas réservés, et une place de marché déjà enregistrée sous un nom réservé continuait à se charger.
</Note>

<h3 id="owner-fields">
  Champs du propriétaire
</h3>

| Champ   | Type   | Obligatoire | Description                              |
| :------ | :----- | :---------- | :--------------------------------------- |
| `name`  | string | Oui         | Nom du responsable ou de l'équipe        |
| `email` | string | Non         | Adresse e-mail de contact du responsable |

<h3 id="optional-fields">
  Champs optionnels
</h3>

| Champ                                 | Type   | Description                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$schema`                             | string | URL du schéma JSON pour l'autocomplétion et la validation de l'éditeur. Claude Code ignore ce champ au moment du chargement.                                                                                                                                                                                                                            |
| `description`                         | string | Brève description de la place de marché                                                                                                                                                                                                                                                                                                                 |
| `version`                             | string | Version du manifeste de la place de marché                                                                                                                                                                                                                                                                                                              |
| `metadata.pluginRoot`                 | string | Répertoire de base ajouté aux chemins de source de plugin relatifs (par exemple, `"./plugins"` vous permet d'écrire `"source": "formatter"` au lieu de `"source": "./plugins/formatter"`)                                                                                                                                                               |
| `allowCrossMarketplaceDependenciesOn` | array  | Autres places de marché sur lesquelles les plugins de cette place de marché peuvent dépendre. Les dépendances d'une place de marché non listée ici sont bloquées à l'installation. Voir [Dépendre d'un plugin d'une autre place de marché](/docs/fr/plugin-dependencies#depend-on-a-plugin-from-another-marketplace).                                        |
| `renames`                             | object | Mappage d'un ancien nom de plugin `name` à son nom actuel, ou à `null` si le plugin a été supprimé. Permet aux utilisateurs existants de migrer automatiquement lorsque vous renommez ou supprimez une entrée dans `plugins`. Voir [Renommer ou supprimer un plugin](#rename-or-remove-a-plugin). Nécessite Claude Code v2.1.193 ou version ultérieure. |

`description` et `version` sont également acceptés sous `metadata` pour la compatibilité rétroactive.

<h2 id="plugin-entries">
  Entrées de plugin
</h2>

Chaque entrée de plugin dans le tableau `plugins` décrit un plugin et où le trouver. Vous pouvez inclure n'importe quel champ du [schéma du manifeste du plugin](/docs/fr/plugins-reference#plugin-manifest-schema), tel que `description`, `version`, `author`, `commands` et `hooks`, plus ces champs spécifiques à la place de marché : `source`, `category`, `tags`, `strict` et `relevance`.

<h3 id="required-fields-2">
  Champs obligatoires
</h3>

| Champ    | Type           | Description                                                                                                                                                                           |
| :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`   | string         | Identifiant du plugin (kebab-case, sans espaces). C'est un élément public : les utilisateurs le voient lors de l'installation (par exemple, `/plugin install my-plugin@marketplace`). |
| `source` | string\|object | Où récupérer le plugin (voir [Sources de plugin](#plugin-sources) ci-dessous)                                                                                                         |

<h3 id="optional-plugin-fields">
  Champs de plugin optionnels
</h3>

**Champs de métadonnées standard :**

| Champ            | Type    | Description                                                                                                                                                                                                                                                                                                                                                           |
| :--------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `displayName`    | string  | Nom lisible affiché dans les surfaces de l'interface utilisateur. Revient à `name` lorsqu'il est omis. Peut contenir des espaces et n'importe quelle casse. Non utilisé pour l'espace de noms ou la recherche. Nécessite Claude Code v2.1.143 ou version ultérieure.                                                                                                  |
| `description`    | string  | Brève description du plugin                                                                                                                                                                                                                                                                                                                                           |
| `version`        | string  | Version du plugin. Si défini (ici ou dans `plugin.json`), le plugin est épinglé à cette chaîne et les utilisateurs ne reçoivent des mises à jour que lorsqu'elle change. Omettez pour revenir au SHA du commit git. Voir [Résolution de version](#version-resolution-and-release-channels).                                                                           |
| `author`         | object  | Informations sur l'auteur du plugin (`name` obligatoire, `email` optionnel)                                                                                                                                                                                                                                                                                           |
| `homepage`       | string  | URL de la page d'accueil ou de la documentation du plugin                                                                                                                                                                                                                                                                                                             |
| `repository`     | string  | URL du dépôt du code source                                                                                                                                                                                                                                                                                                                                           |
| `license`        | string  | Identifiant de licence SPDX (par exemple, MIT, Apache-2.0)                                                                                                                                                                                                                                                                                                            |
| `keywords`       | array   | Balises pour la découverte et la catégorisation des plugins                                                                                                                                                                                                                                                                                                           |
| `category`       | string  | Catégorie du plugin pour l'organisation                                                                                                                                                                                                                                                                                                                               |
| `tags`           | array   | Balises pour la recherche                                                                                                                                                                                                                                                                                                                                             |
| `strict`         | boolean | Contrôle si `plugin.json` est l'autorité pour les définitions de composants (par défaut : true). Voir [Mode strict](#strict-mode) ci-dessous.                                                                                                                                                                                                                         |
| `relevance`      | object  | Signaux qui indiquent à Claude Code quand suggérer ce plugin aux utilisateurs. Prend effet uniquement pour les places de marché qu'un administrateur autorise dans les paramètres gérés. Voir [Recommander des plugins pour votre organisation](/docs/fr/plugin-relevance). Nécessite Claude Code v2.1.152 ou version ultérieure.                                          |
| `defaultEnabled` | boolean | Si le plugin est activé après l'installation (par défaut : true). Définissez sur `false` pour installer le plugin désactivé jusqu'à ce que l'utilisateur l'active. Prend la priorité sur le même champ dans le `plugin.json` du plugin. Voir [Activation par défaut](/docs/fr/plugins-reference#default-enablement). Nécessite Claude Code v2.1.154 ou version ultérieure. |

**Champs de configuration des composants :**

| Champ        | Type           | Description                                                                           |
| :----------- | :------------- | :------------------------------------------------------------------------------------ |
| `skills`     | string\|array  | Chemins personnalisés vers les répertoires de compétences contenant `<name>/SKILL.md` |
| `commands`   | string\|array  | Chemins personnalisés vers les fichiers de compétences `.md` plats ou les répertoires |
| `agents`     | string\|array  | Chemins personnalisés vers les fichiers d'agents                                      |
| `hooks`      | string\|object | Configuration personnalisée des hooks ou chemin vers le fichier des hooks             |
| `mcpServers` | string\|object | Configurations du serveur MCP ou chemin vers la configuration MCP                     |
| `lspServers` | string\|object | Configurations du serveur LSP ou chemin vers la configuration LSP                     |

<h2 id="plugin-sources">
  Sources de plugin
</h2>

Les sources de plugin indiquent à Claude Code où récupérer chaque plugin individuel répertorié dans votre place de marché. Elles sont définies dans le champ `source` de chaque entrée de plugin dans `marketplace.json`.

Une fois qu'un plugin est cloné ou téléchargé sur la machine locale, il est copié dans le cache de plugin local versionné à `~/.claude/plugins/cache`.

| Source         | Type                                   | Champs                             | Notes                                                                                                                                                                    |
| -------------- | -------------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Chemin relatif | `string` (par exemple `"./my-plugin"`) | aucun                              | Répertoire local dans le dépôt de la place de marché. Doit commencer par `./`. Résolu par rapport à la racine de la place de marché, pas au répertoire `.claude-plugin/` |
| `github`       | object                                 | `repo`, `ref?`, `sha?`             |                                                                                                                                                                          |
| `url`          | object                                 | `url`, `ref?`, `sha?`              | Source d'URL Git                                                                                                                                                         |
| `git-subdir`   | object                                 | `url`, `path`, `ref?`, `sha?`      | Sous-répertoire dans un dépôt git. Clone partiellement pour minimiser la bande passante pour les monodépôts                                                              |
| `npm`          | object                                 | `package`, `version?`, `registry?` | Installé via `npm install`                                                                                                                                               |

<Note>
  **Sources de place de marché vs sources de plugin** : Ce sont des concepts différents qui contrôlent des choses différentes.

  * **Source de place de marché** : où récupérer le catalogue `marketplace.json` lui-même. Défini lorsque les utilisateurs exécutent `/plugin marketplace add` ou dans les paramètres `extraKnownMarketplaces`. Prend en charge `ref` (branche/tag) mais pas `sha`.
  * **Source de plugin** : où récupérer un plugin individuel répertorié dans la place de marché. Défini dans le champ `source` de chaque entrée de plugin dans `marketplace.json`. Prend en charge à la fois `ref` (branche/tag) et `sha` (commit exact).

  Par exemple, une place de marché hébergée à `acme-corp/plugin-catalog` (source de place de marché) peut répertorier un plugin récupéré à partir de `acme-corp/code-formatter` (source de plugin). La source de place de marché et la source de plugin pointent vers des dépôts différents et sont épinglées indépendamment.
</Note>

Les types de source basés sur git ci-dessous sont `github`, `url` et `git-subdir`. Lorsque `ref` et `sha` sont tous deux définis sur l'un d'eux, le `sha` est l'épingle effective. Claude Code récupère et vérifie le commit épinglé directement.

Sur la plupart des hôtes git, y compris GitHub, GitLab et Bitbucket, cela signifie que l'installation réussit même si la branche ou le tag nommé par `ref` a depuis été supprimé en amont, tant que le commit est toujours accessible à partir du dépôt. Certains serveurs, tels qu'AWS CodeCommit, ne prennent pas en charge la récupération des commits par SHA. Sur ces serveurs, le `ref` doit toujours exister et le commit épinglé doit être accessible à partir de celui-ci.

<h3 id="relative-paths">
  Chemins relatifs
</h3>

Pour les plugins dans le même dépôt, utilisez un chemin commençant par `./` :

```json theme={null}
{
  "name": "my-plugin",
  "source": "./plugins/my-plugin"
}
```

Les chemins se résolvent par rapport à la racine de la place de marché, qui est le répertoire contenant `.claude-plugin/`. Dans l'exemple ci-dessus, `./plugins/my-plugin` pointe vers `<repo>/plugins/my-plugin`, même si `marketplace.json` se trouve à `<repo>/.claude-plugin/marketplace.json`. N'utilisez pas `../` pour référencer des chemins en dehors de la racine de la place de marché.

<Note>
  Les chemins relatifs se résolvent par rapport à une copie locale de la place de marché, donc ils fonctionnent lorsque les utilisateurs ajoutent votre place de marché à partir d'une source git ou d'un répertoire local. Si les utilisateurs ajoutent votre place de marché via une URL directe vers le fichier `marketplace.json`, les chemins relatifs ne se résoudront pas, car seul ce fichier est téléchargé. Pour la distribution basée sur les URL, utilisez plutôt les sources GitHub, npm ou URL git. Consultez [Dépannage](#plugins-with-relative-paths-fail-in-url-based-marketplaces) pour plus de détails.
</Note>

<h3 id="github-repositories">
  Dépôts GitHub
</h3>

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo"
  }
}
```

Vous pouvez épingler à une branche, un tag ou un commit spécifique :

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Champ  | Type   | Description                                                                              |
| :----- | :----- | :--------------------------------------------------------------------------------------- |
| `repo` | string | Obligatoire. Dépôt GitHub au format `owner/repo`                                         |
| `ref`  | string | Optionnel. Branche ou tag Git (par défaut la branche par défaut du dépôt)                |
| `sha`  | string | Optionnel. SHA de commit git complet de 40 caractères pour épingler à une version exacte |

<h3 id="git-repositories">
  Dépôts Git
</h3>

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git"
  }
}
```

Vous pouvez épingler à une branche, un tag ou un commit spécifique :

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git",
    "ref": "main",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Champ | Type   | Description                                                                                                                                                              |
| :---- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url` | string | Obligatoire. URL complète du dépôt git (`https://` ou `git@`). Le suffixe `.git` est optionnel, donc les URL Azure DevOps et AWS CodeCommit sans le suffixe fonctionnent |
| `ref` | string | Optionnel. Branche ou tag Git (par défaut la branche par défaut du dépôt)                                                                                                |
| `sha` | string | Optionnel. SHA de commit git complet de 40 caractères pour épingler à une version exacte                                                                                 |

<h3 id="git-subdirectories">
  Sous-répertoires Git
</h3>

Utilisez `git-subdir` pour pointer vers un plugin qui se trouve dans un sous-répertoire d'un dépôt git. Claude Code utilise un clone partiel et clairsemé pour récupérer uniquement le sous-répertoire, minimisant la bande passante pour les grands monodépôts.

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin"
  }
}
```

Vous pouvez épingler à une branche, un tag ou un commit spécifique :

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

Le champ `url` accepte également un raccourci GitHub (`owner/repo`) ou des URL SSH (`git@github.com:owner/repo.git`).

| Champ  | Type   | Description                                                                                                     |
| :----- | :----- | :-------------------------------------------------------------------------------------------------------------- |
| `url`  | string | Obligatoire. URL du dépôt Git, raccourci GitHub `owner/repo` ou URL SSH                                         |
| `path` | string | Obligatoire. Chemin du sous-répertoire dans le dépôt contenant le plugin (par exemple, `"tools/claude-plugin"`) |
| `ref`  | string | Optionnel. Branche ou tag Git (par défaut la branche par défaut du dépôt)                                       |
| `sha`  | string | Optionnel. SHA de commit git complet de 40 caractères pour épingler à une version exacte                        |

<h3 id="npm-packages">
  Paquets npm
</h3>

Les plugins distribués en tant que paquets npm sont installés à l'aide de `npm install`. Cela fonctionne avec n'importe quel paquet du registre npm public ou d'un registre privé que votre équipe héberge.

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin"
  }
}
```

Pour épingler à une version spécifique, ajoutez le champ `version` :

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "2.1.0"
  }
}
```

Pour installer à partir d'un registre privé ou interne, ajoutez le champ `registry` :

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

| Champ      | Type   | Description                                                                                                 |
| :--------- | :----- | :---------------------------------------------------------------------------------------------------------- |
| `package`  | string | Obligatoire. Nom du paquet ou paquet scopé (par exemple, `@org/plugin`)                                     |
| `version`  | string | Optionnel. Version ou plage de version (par exemple, `2.1.0`, `^2.0.0`, `~1.5.0`)                           |
| `registry` | string | Optionnel. URL du registre npm personnalisé. Par défaut le registre npm du système (généralement npmjs.org) |

<h3 id="advanced-plugin-entries">
  Entrées de plugin avancées
</h3>

Cet exemple montre une entrée de plugin utilisant de nombreux champs optionnels, notamment des chemins personnalisés pour les commandes, les agents, les hooks et les serveurs MCP :

```json theme={null}
{
  "name": "enterprise-tools",
  "source": {
    "source": "github",
    "repo": "company/enterprise-plugin"
  },
  "description": "Enterprise workflow automation tools",
  "version": "2.1.0",
  "author": {
    "name": "Enterprise Team",
    "email": "enterprise@example.com"
  },
  "homepage": "https://docs.example.com/plugins/enterprise-tools",
  "repository": "https://github.com/company/enterprise-plugin",
  "license": "MIT",
  "keywords": ["enterprise", "workflow", "automation"],
  "category": "productivity",
  "commands": [
    "./commands/core/",
    "./commands/enterprise/",
    "./commands/experimental/preview.md"
  ],
  "agents": ["./agents/security-reviewer.md", "./agents/compliance-checker.md"],
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh"
          }
        ]
      }
    ]
  },
  "mcpServers": {
    "enterprise-db": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"]
    }
  },
  "strict": false
}
```

Points clés à noter :

* **`commands` et `agents`** : vous pouvez spécifier plusieurs répertoires ou fichiers individuels. Les chemins sont relatifs à la racine du plugin.
* **`${CLAUDE_PLUGIN_ROOT}`** : utilisez cette variable dans les commandes de hook et les configurations du serveur MCP pour référencer les fichiers dans le répertoire d'installation du plugin. C'est nécessaire car les plugins sont copiés vers un emplacement de cache lors de l'installation.
  * Consultez le [tableau de substitution](/docs/fr/plugins-reference#environment-variables) pour savoir quels champs de configuration le substituent par type de serveur
  * Pour les dépendances ou l'état qui doivent survivre aux mises à jour des plugins, utilisez [`${CLAUDE_PLUGIN_DATA}`](/docs/fr/plugins-reference#persistent-data-directory) à la place
* **`strict: false`** : puisque ceci est défini sur false, le plugin n'a pas besoin de son propre `plugin.json`. L'entrée de la place de marché définit tout. Voir [Mode strict](#strict-mode) ci-dessous.

Par défaut, les compétences d'un plugin se chargent à partir du répertoire `skills/` sous sa `source`. Les chemins répertoriés dans le champ `skills` s'ajoutent à cette analyse :

```json theme={null}
"skills": ["./skills/", "./extra-skills/"]
```

Lorsque plusieurs entrées de plugin partagent un dossier `skills/` à la racine de la place de marché (`source: "./"`), énumérez plutôt des sous-répertoires spécifiques afin que chaque entrée ne charge que ses propres compétences :

```json theme={null}
"source": "./",
"skills": ["./skills/code-review", "./skills/docs"]
```

Avec une source à la racine de la place de marché, les chemins énumérés constituent l'ensemble complet pour cette entrée, et les autres répertoires dans le dossier `skills/` partagé ne se chargent pas. L'énumération du répertoire `skills/` lui-même, ou de la racine du plugin, maintient l'analyse complète. Si aucun des chemins énumérés n'existe, l'analyse par défaut s'exécute à la place.

<h3 id="strict-mode">
  Mode strict
</h3>

Le champ `strict` contrôle si `plugin.json` est l'autorité pour les définitions de composants (compétences, agents, hooks, serveurs MCP, styles de sortie).

| Valeur              | Comportement                                                                                                                                                                     |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `true` (par défaut) | `plugin.json` est l'autorité. L'entrée de la place de marché peut la compléter avec des composants supplémentaires, et les deux sources sont fusionnées.                         |
| `false`             | L'entrée de la place de marché est la définition complète. Si le plugin a également un `plugin.json` qui déclare des composants, c'est un conflit et le plugin ne se charge pas. |

**Quand utiliser chaque mode :**

* **`strict: true`** : le plugin a son propre `plugin.json` et gère ses propres composants. L'entrée de la place de marché peut ajouter des compétences ou des hooks supplémentaires par-dessus. C'est la valeur par défaut et fonctionne pour la plupart des plugins.
* **`strict: false`** : l'opérateur de la place de marché veut le contrôle total. Le dépôt du plugin fournit des fichiers bruts, et l'entrée de la place de marché définit lesquels de ces fichiers sont exposés en tant que compétences, agents, hooks, etc. Utile lorsque la place de marché restructure ou sélectionne les composants d'un plugin différemment de ce que l'auteur du plugin avait prévu.

<h2 id="host-and-distribute-marketplaces">
  Héberger et distribuer les places de marché
</h2>

<h3 id="host-on-github-recommended">
  Héberger sur GitHub (recommandé)
</h3>

GitHub est la méthode recommandée pour héberger et distribuer une place de marché :

1. **Créer un dépôt** : Configurez un nouveau dépôt pour votre place de marché
2. **Ajouter le fichier de place de marché** : Créez `.claude-plugin/marketplace.json` avec vos définitions de plugins
3. **Partager avec les équipes** : Les utilisateurs ajoutent votre place de marché avec `/plugin marketplace add owner/repo`

**Avantages** : Contrôle de version intégré, suivi des problèmes et fonctionnalités de collaboration d'équipe.

<h3 id="host-on-other-git-services">
  Héberger sur d'autres services git
</h3>

N'importe quel service d'hébergement git fonctionne, comme GitLab, Bitbucket et les serveurs auto-hébergés. Les utilisateurs ajoutent avec l'URL complète du dépôt :

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

<h3 id="private-repositories">
  Dépôts privés
</h3>

Claude Code prend en charge l'installation de plugins à partir de dépôts privés. Pour l'installation manuelle et les mises à jour, Claude Code utilise vos assistants de credentials git existants, donc l'accès HTTPS via `gh auth login`, Keychain macOS ou `git-credential-store` fonctionne de la même manière que dans votre terminal. L'accès SSH fonctionne tant que l'hôte est déjà dans votre fichier `known_hosts` et que la clé est chargée dans `ssh-agent`, puisque Claude Code supprime les invites SSH interactives pour l'empreinte digitale de l'hôte et la phrase de passe de la clé. Les sources de raccourci `owner/repo` GitHub clonent par défaut via SSH ; définissez [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/fr/env-vars#variables) pour les cloner via HTTPS à la place.

Les mises à jour automatiques en arrière-plan fonctionnent différemment. Par défaut, l'actualisation en arrière-plan désactive les assistants de credentials git pour son `git pull`, donc le pull ne peut pas s'authentifier auprès des dépôts privés sur HTTPS même lorsqu'un assistant est configuré. Les remotes SSH ne sont pas affectées : une clé chargée dans `ssh-agent` authentifie les pulls en arrière-plan de la même manière que les opérations manuelles. Lorsque le pull en arrière-plan échoue, Claude Code revient à re-cloner la place de marché à partir de zéro. Le re-clone utilise vos credentials git stockés, mais il peut [expirer sur les grands dépôts](#git-operations-time-out), donc les mises à jour automatiques de places de marché privées peuvent échouer par intermittence.

Deux paramètres rendent les places de marché privées prévisibles :

* Définissez `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` pour conserver le clone existant lorsque le pull en arrière-plan échoue, au lieu de supprimer et re-cloner. Vos plugins continuent de fonctionner à partir du dernier état synchronisé, et les mises à jour manuelles avec `/plugin marketplace update` tirent toujours avec vos credentials.
* Configurez un assistant de credentials git, par exemple avec `gh auth setup-git` pour GitHub, afin que le fallback re-clone puisse s'authentifier sans inviter.

Définir un jeton de fournisseur tel que `GITHUB_TOKEN` dans votre environnement n'active pas par lui-même l'authentification en arrière-plan. Les jetons ne prennent effet que via un assistant de credentials configuré, par exemple l'assistant de l'CLI `gh`, qui lit `GH_TOKEN` et `GITHUB_TOKEN`.

Pour que le pull en arrière-plan lui-même s'authentifie sur HTTPS, configurez une réécriture d'URL git globale. La réécriture intègre un jeton dans l'URL distante, donc elle prend effet même si le pull en arrière-plan désactive les assistants de credentials, et un pull réussi ignore le fallback re-clone. L'exemple suivant réécrit l'URL du dépôt de la place de marché pour inclure un jeton d'accès :

```bash theme={null}
git config --global url."https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins".insteadOf "https://github.com/acme-corp/plugins"
```

Limitez la réécriture au dépôt de la place de marché ou au chemin de l'organisation. Une réécriture dont la base est uniquement l'hôte s'applique à chaque fetch et push vers cet hôte sur la machine et remplace vos credentials normaux, y compris les pushes vers vos propres dépôts.

Chaque fournisseur attend un nom d'utilisateur différent dans l'URL réécrite, et la même limitation de chemin s'applique à chaque fournisseur. Pour les serveurs auto-hébergés, remplacez le nom d'hôte par le nom d'hôte de votre serveur :

| Fournisseur | Forme d'URL réécrite                                              |
| :---------- | :---------------------------------------------------------------- |
| GitHub      | `https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins`  |
| GitLab      | `https://oauth2:YOUR_TOKEN@gitlab.com/acme-corp/plugins`          |
| Bitbucket   | `https://x-token-auth:YOUR_TOKEN@bitbucket.org/acme-corp/plugins` |

La réécriture stocke le jeton en texte brut dans votre gitconfig, donc utilisez un jeton avec accès en lecture seule au dépôt de la place de marché.

<Note>
  Dans les environnements CI/CD, configurez un assistant de credentials git avant d'installer des plugins à partir de dépôts privés. Sur GitHub Actions, exportez un jeton avec accès en lecture au dépôt de la place de marché en tant que `GH_TOKEN`, puis exécutez `gh auth setup-git`. Le jeton de workflow par défaut ne peut accéder qu'au dépôt du workflow lui-même, donc une place de marché privée dans un autre dépôt a besoin d'un jeton d'accès personnel ou d'un jeton d'application. Une réécriture d'URL globale configurée dans le pipeline authentifie également le pull en arrière-plan directement.
</Note>

<h3 id="test-locally-before-distribution">
  Tester localement avant la distribution
</h3>

Testez votre place de marché localement avant de la partager :

```shell theme={null}
/plugin marketplace add ./my-marketplace
/plugin install quality-review-plugin@my-plugins
```

Pour la gamme complète de commandes add (GitHub, URL Git, chemins locaux, URL distantes), consultez [Ajouter des places de marché](/docs/fr/discover-plugins#add-marketplaces).

<h3 id="require-marketplaces-for-your-team">
  Exiger des places de marché pour votre équipe
</h3>

Vous pouvez configurer votre dépôt pour que les membres de l'équipe soient automatiquement invités à installer votre place de marché lorsqu'ils font confiance au dossier du projet. Ajoutez votre place de marché à `.claude/settings.json` :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "company-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Vous pouvez également spécifier quels plugins doivent être activés par défaut :

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@company-tools": true,
    "deployment-tools@company-tools": true
  }
}
```

Pour les options de configuration complètes, consultez [Paramètres des plugins](/docs/fr/settings#plugin-settings).

<Note>
  Si vous utilisez une source `directory` ou `file` locale avec un chemin relatif, le chemin se résout par rapport au checkout principal de votre dépôt. Lorsque vous exécutez Claude Code à partir d'une worktree git, le chemin pointe toujours vers le checkout principal, donc toutes les worktrees partagent le même emplacement de place de marché. L'état de la place de marché est stocké une fois par utilisateur dans `~/.claude/plugins/known_marketplaces.json`, pas par projet.
</Note>

<h3 id="pre-populate-plugins-for-containers">
  Pré-remplir les plugins pour les conteneurs
</h3>

Pour les images de conteneur et les environnements CI, vous pouvez pré-remplir un répertoire de plugins au moment de la construction afin que Claude Code démarre avec des places de marché et des plugins déjà disponibles, sans rien cloner au moment de l'exécution. Définissez la variable d'environnement `CLAUDE_CODE_PLUGIN_SEED_DIR` pour pointer vers ce répertoire.

Pour superposer plusieurs répertoires de seed, séparez les chemins avec `:` sur Unix ou `;` sur Windows. Claude Code recherche chaque répertoire dans l'ordre et utilise le premier seed qui contient une place de marché ou un cache de plugin donné.

Le répertoire de seed reflète la structure de `~/.claude/plugins` :

```
$CLAUDE_CODE_PLUGIN_SEED_DIR/
  known_marketplaces.json
  marketplaces/<name>/...
  cache/<marketplace>/<plugin>/<version>/...
```

Pour construire un répertoire de seed, exécutez Claude Code une fois lors de la construction de l'image, installez les plugins dont vous avez besoin, puis copiez le répertoire `~/.claude/plugins` résultant dans votre image et pointez `CLAUDE_CODE_PLUGIN_SEED_DIR` vers lui.

Pour ignorer l'étape de copie, définissez `CLAUDE_CODE_PLUGIN_CACHE_DIR` sur votre chemin de seed cible lors de la construction afin que les plugins s'installent directement là :

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/plugins
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install my-tool@your-plugins
```

Ensuite, définissez `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` dans l'environnement d'exécution de votre conteneur afin que Claude Code lise à partir du seed au démarrage.

Au démarrage, Claude Code enregistre les places de marché trouvées dans le `known_marketplaces.json` du seed dans la configuration principale, et utilise les caches de plugins trouvés sous `cache/` en place sans re-cloner. Cela fonctionne à la fois en mode interactif et en mode non-interactif avec le drapeau `-p`.

Détails du comportement :

* **Lecture seule** : le répertoire de seed n'est jamais écrit. Les mises à jour automatiques sont désactivées pour les places de marché de seed puisque git pull échouerait sur un système de fichiers en lecture seule.
* **Les entrées de seed ont la priorité** : les places de marché déclarées dans le seed remplacent toutes les entrées correspondantes dans la configuration de l'utilisateur à chaque démarrage. Pour refuser un plugin de seed, utilisez `/plugin disable` plutôt que de supprimer la place de marché.
* **Résolution des chemins** : Claude Code localise le contenu de la place de marché en sondant `$CLAUDE_CODE_PLUGIN_SEED_DIR/marketplaces/<name>/` au moment de l'exécution, pas en faisant confiance aux chemins stockés dans le JSON du seed. Cela signifie que le seed fonctionne correctement même lorsqu'il est monté à un chemin différent de celui où il a été construit.
* **La mutation est bloquée** : l'exécution de `/plugin marketplace remove` ou `/plugin marketplace update` contre une place de marché gérée par seed échoue avec des conseils pour demander à votre administrateur de mettre à jour l'image de seed.
* **Compose avec les paramètres** : si `extraKnownMarketplaces` ou `enabledPlugins` déclarent une place de marché qui existe déjà dans le seed, Claude Code utilise la copie du seed au lieu de cloner.

<h3 id="managed-marketplace-restrictions">
  Restrictions des places de marché gérées
</h3>

Pour les organisations nécessitant un contrôle strict sur les sources de plugins, les administrateurs peuvent restreindre les places de marché de plugins que les utilisateurs sont autorisés à ajouter en utilisant le paramètre [`strictKnownMarketplaces`](/docs/fr/settings#strictknownmarketplaces) dans les paramètres gérés. Pour également rejeter les drapeaux CLI qui chargent les plugins, les agents et les serveurs MCP pour une seule exécution, associez-le à [`disableSideloadFlags`](/docs/fr/settings#available-settings). Pour créer une liste blanche des places de marché dont les plugins peuvent apparaître comme suggestions d'installation contextuelle, définissez [`pluginSuggestionMarketplaces`](/docs/fr/settings#available-settings).

Lorsque `strictKnownMarketplaces` est configuré dans les paramètres gérés, le comportement de restriction dépend de la valeur :

| Valeur                  | Comportement                                                                                                        |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Non défini (par défaut) | Aucune restriction. Les utilisateurs peuvent ajouter n'importe quelle place de marché                               |
| Tableau vide `[]`       | Verrouillage complet. Les utilisateurs ne peuvent pas ajouter de nouvelles places de marché                         |
| Liste de sources        | Les utilisateurs ne peuvent ajouter que les places de marché qui correspondent exactement à la liste d'autorisation |

<h4 id="common-configurations">
  Configurations courantes
</h4>

Désactiver tous les ajouts de place de marché :

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

Autoriser uniquement les places de marché spécifiques :

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    }
  ]
}
```

Autoriser toutes les places de marché d'un serveur git interne en utilisant la correspondance de motif regex sur l'hôte. C'est l'approche recommandée pour [GitHub Enterprise Server](/docs/fr/github-enterprise-server#plugin-marketplaces-on-ghes) ou les instances GitLab auto-hébergées :

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Autoriser les places de marché basées sur le système de fichiers à partir d'un répertoire spécifique en utilisant la correspondance de motif regex sur le chemin :

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "pathPattern",
      "pathPattern": "^/opt/approved/"
    }
  ]
}
```

Utilisez `".*"` comme `pathPattern` pour autoriser n'importe quel chemin du système de fichiers tout en contrôlant les sources réseau avec `hostPattern`.

<Note>
  `strictKnownMarketplaces` restreint ce que les utilisateurs peuvent ajouter, mais n'enregistre pas les places de marché par lui-même. Pour rendre les places de marché autorisées disponibles automatiquement sans que les utilisateurs exécutent `/plugin marketplace add`, associez-le à [`extraKnownMarketplaces`](/docs/fr/settings#extraknownmarketplaces) dans le même `managed-settings.json`. Voir [Utiliser les deux ensemble](/docs/fr/settings#strictknownmarketplaces).
</Note>

<h4 id="how-restrictions-work">
  Comment fonctionnent les restrictions
</h4>

Les restrictions sont vérifiées avant toute opération réseau ou système de fichiers. La vérification s'exécute lors de l'ajout de place de marché et lors de l'installation, la mise à jour, l'actualisation et la mise à jour automatique du plugin. Si une place de marché a été ajoutée avant la configuration de la politique et que sa source ne correspond plus à la liste d'autorisation, Claude Code refuse d'installer ou de mettre à jour les plugins à partir de celle-ci. L'application de la même restriction s'applique à `blockedMarketplaces`.

La liste d'autorisation utilise la correspondance exacte pour la plupart des types de sources. Pour qu'une place de marché soit autorisée, tous les champs spécifiés doivent correspondre exactement :

* Pour les sources GitHub : `repo` est obligatoire, et `ref` ou `path` doivent également correspondre s'ils sont spécifiés dans la liste d'autorisation
* Pour les sources URL : l'URL complète doit correspondre exactement
* Pour les sources `hostPattern` : l'hôte de la place de marché est comparé au motif regex
* Pour les sources `pathPattern` : le chemin du système de fichiers de la place de marché est comparé au motif regex

La correspondance exacte ne normalise pas les URL : une barre oblique finale, un suffixe `.git` ou une forme `ssh://` par rapport à `https://` sont traités comme des valeurs différentes. Si la place de marché de votre organisation peut être clonée par plus d'une forme d'URL, préférez une entrée `hostPattern` à une URL littérale afin que toutes les formes correspondent.

Parce que `strictKnownMarketplaces` est défini dans les [paramètres gérés](/docs/fr/settings#settings-files), les configurations individuelles des utilisateurs et des projets ne peuvent pas contourner ces restrictions.

Pour les détails de configuration complets, y compris tous les types de sources pris en charge et la comparaison avec `extraKnownMarketplaces`, consultez la [référence strictKnownMarketplaces](/docs/fr/settings#strictknownmarketplaces).

<h3 id="version-resolution-and-release-channels">
  Résolution des versions et canaux de publication
</h3>

Les versions des plugins déterminent les chemins du cache et la détection des mises à jour : si la version résolue correspond à ce qu'un utilisateur possède déjà, `/plugin update` et la mise à jour automatique ignorent le plugin.

Claude Code résout la version d'un plugin à partir du premier de ces éléments qui est défini :

1. `version` dans le `plugin.json` du plugin
2. `version` dans l'entrée de la place de marché du plugin
3. Le SHA du commit git de la source du plugin

Pour les types de sources basés sur git `github`, `url`, `git-subdir` et les chemins relatifs à l'intérieur d'une place de marché hébergée sur git, vous pouvez omettre entièrement `version` et chaque nouveau commit est traité comme une nouvelle version. C'est la configuration la plus simple pour les plugins internes ou en développement actif.

<Warning>
  Définir `version` épingle le plugin. Si `plugin.json` déclare `"version": "1.0.0"`, pousser de nouveaux commits sans changer cette chaîne ne fait rien pour les utilisateurs existants, car Claude Code voit la même version et conserve la copie en cache. Augmentez le champ à chaque publication, ou omettez-le pour utiliser le SHA du commit.

  Évitez de définir `version` à la fois dans `plugin.json` et dans l'entrée de la place de marché. La valeur `plugin.json` gagne toujours silencieusement, donc une version de manifeste obsolète peut masquer une version que vous avez définie dans `marketplace.json`.
</Warning>

<h4 id="set-up-release-channels">
  Configurer les canaux de publication
</h4>

Pour prendre en charge les canaux de publication « stable » et « latest » pour vos plugins, vous pouvez configurer deux places de marché qui pointent vers différentes refs ou SHAs du même dépôt. Vous pouvez ensuite assigner les deux places de marché à différents groupes d'utilisateurs via les [paramètres gérés](/docs/fr/settings#settings-files).

<Warning>
  Chaque canal doit se résoudre en une version différente. Si vous utilisez des versions explicites, `plugin.json` doit déclarer une `version` différente à chaque ref ou SHA épinglé. Si vous omettez `version`, les SHAs de commit distincts distinguent déjà les canaux. Si deux refs se résolvent en la même chaîne de version, Claude Code les traite comme identiques et ignore la mise à jour.
</Warning>

<h5 id="example">
  Exemple
</h5>

```json theme={null}
{
  "name": "stable-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "stable"
      }
    }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "latest"
      }
    }
  ]
}
```

<h5 id="assign-channels-to-user-groups">
  Assigner les canaux aux groupes d'utilisateurs
</h5>

Assignez chaque place de marché au groupe d'utilisateurs approprié via les paramètres gérés. Par exemple, le groupe stable reçoit :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/stable-tools"
      }
    }
  }
}
```

Le groupe early-access reçoit `latest-tools` à la place :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "latest-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/latest-tools"
      }
    }
  }
}
```

<h4 id="pin-dependency-versions">
  Épingler les versions des dépendances
</h4>

Un plugin peut contraindre ses dépendances à une plage semver afin que les mises à jour d'une dépendance ne cassent pas le plugin dépendant. Consultez [Contraindre les versions des dépendances de plugins](/docs/fr/plugin-dependencies) pour la convention de balise git `{plugin-name}--v{version}`, la syntaxe de plage et la façon dont plusieurs contraintes sur la même dépendance sont combinées.

<h3 id="rename-or-remove-a-plugin">
  Renommer ou supprimer un plugin
</h3>

Le `name` d'un plugin est son identifiant stable. Les utilisateurs le référencent dans `enabledPlugins`, `pluginConfigs` et les commandes `/plugin install`, donc le changer casse chaque installation existante. Pour changer l'étiquette affichée dans l'interface utilisateur sans casser les installations, définissez [`displayName`](#optional-plugin-fields) et gardez `name` inchangé.

Si vous devez changer le `name` d'un plugin, ou si vous supprimez un plugin du tableau `plugins`, ajoutez une entrée `renames` au niveau supérieur afin que les utilisateurs existants migrent au lieu de voir une erreur `plugin-not-found`. La migration automatique nécessite Claude Code v2.1.193 ou ultérieur. Mappez chaque ancien nom à son nouveau nom, ou à `null` si le plugin n'existe plus. L'exemple suivant renomme `formatter` en `code-formatter` et enregistre que `legacy-linter` a été supprimé :

```json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Lorsqu'un utilisateur démarre Claude Code avec l'ancien nom toujours dans ses paramètres, Claude Code suit la carte `renames` :

* Si l'entrée pointe vers un nouveau nom, Claude Code charge le plugin sous son nouveau nom et affiche un avis d'une ligne tel que `Renamed to "code-formatter" in the "acme-tools" marketplace`. Il réécrit ensuite l'ancienne clé vers la nouvelle clé dans les portées de paramètres utilisateur, projet et local pour `enabledPlugins` et `pluginConfigs`, afin que l'avis n'apparaisse qu'une fois.
* Pour une entrée `null`, Claude Code supprime l'ancienne clé et l'avis signale que le plugin a été supprimé de la place de marché.
* Si le plugin renommé utilise une source distante telle que `github` ou `npm`, Claude Code signale `plugin-cache-miss` après le renommage et l'utilisateur doit exécuter `/plugin install` une fois pour le récupérer sous le nouveau nom.

Traitez `renames` comme un historique d'ajout uniquement : gardez les anciennes entrées en place même après vous attendre à ce que chaque utilisateur ait migré. Claude Code suit les chaînes, donc si vous renommez ultérieurement `code-formatter` en `formatter-pro`, ajoutez une deuxième entrée plutôt que de modifier la première. Un utilisateur qui a toujours le `formatter` original activé se résout ensuite à travers les deux entrées vers `formatter-pro`.

Exécutez `claude plugin validate .` après avoir modifié la carte ; il rejette toute entrée dont la chaîne forme un cycle ou ne se termine pas à `null` ou à un nom listé dans `plugins`.

<Note>
  Les paramètres gérés et de politique sont en lecture seule pour Claude Code, donc les plugins activés là ne peuvent pas être réécrits automatiquement. Le plugin renommé se charge toujours à chaque session, mais l'avis de renommage se répète jusqu'à ce qu'un administrateur mette à jour `enabledPlugins` dans le fichier de paramètres gérés pour utiliser le nouveau nom. La même chose s'applique aux plugins activés via d'autres sources en lecture seule telles que `--add-dir`.
</Note>

Les versions antérieures de Claude Code ignorent le champ `renames` et signalent `plugin-not-found` pour l'ancien nom.

<h2 id="validation-and-testing">
  Validation et test
</h2>

Testez votre place de marché avant de la partager.

Validez la syntaxe JSON de votre place de marché :

```bash theme={null}
claude plugin validate .
```

Ou depuis Claude Code :

```shell theme={null}
/plugin validate .
```

Ajoutez la place de marché pour le test :

```shell theme={null}
/plugin marketplace add ./path/to/marketplace
```

Installez un plugin de test pour vérifier que tout fonctionne :

```shell theme={null}
/plugin install test-plugin@marketplace-name
```

Pour les flux de travail complets de test de plugins, consultez [Tester vos plugins localement](/docs/fr/plugins#test-your-plugins-locally). Pour le dépannage technique, consultez [Référence des plugins](/docs/fr/plugins-reference).

<h2 id="manage-marketplaces-from-the-cli">
  Gérer les places de marché à partir de la CLI
</h2>

Claude Code fournit des sous-commandes `claude plugin marketplace` non-interactives pour les scripts et l'automatisation. Elles sont équivalentes aux commandes `/plugin marketplace` disponibles dans une session interactive.

<h3 id="plugin-marketplace-add">
  Plugin marketplace add
</h3>

Ajoutez une place de marché à partir d'un dépôt GitHub, d'une URL git, d'une URL distante ou d'un chemin local.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

**Arguments :**

* `<source>` : Raccourci GitHub `owner/repo`, URL git, URL distante vers un fichier `marketplace.json` ou chemin de répertoire local. Pour épingler à une branche ou un tag, ajoutez `@ref` au raccourci GitHub ou `#ref` à une URL git

Une URL doit inclure son schéma. À partir de Claude Code v2.1.196, un hôte saisi sans schéma, tel que `gitlab.example.com/team/plugins`, est rejeté comme un raccourci `owner/repo` invalide et l'erreur vous indique d'ajouter `https://` ou d'utiliser `./` pour un chemin local. Les versions antérieures l'interprétaient mal comme un chemin de dépôt GitHub et échouent au moment du clonage avec une erreur GitHub non trouvé.

**Options :**

| Option                | Description                                                                                                                                                | Par défaut |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------- |
| `--scope <scope>`     | Où déclarer la place de marché : `user`, `project` ou `local`. Voir [Portées d'installation des plugins](/docs/fr/plugins-reference#plugin-installation-scopes) | `user`     |
| `--sparse <paths...>` | Limiter le checkout à des répertoires spécifiques via git sparse-checkout. Utile pour les monodépôts                                                       |            |

Ajoutez une place de marché à partir de GitHub en utilisant le raccourci `owner/repo` :

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins
```

Épinglez à une branche ou un tag spécifique avec `@ref` :

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins@v2.0
```

Ajoutez à partir d'une URL git sur un hôte non-GitHub :

```bash theme={null}
claude plugin marketplace add https://gitlab.example.com/team/plugins.git
```

Ajoutez à partir d'une URL distante qui sert le fichier `marketplace.json` directement :

```bash theme={null}
claude plugin marketplace add https://example.com/marketplace.json
```

Ajoutez à partir d'un répertoire local pour le test :

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

Déclarez la place de marché à la portée du projet afin qu'elle soit partagée avec votre équipe via `.claude/settings.json` :

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins --scope project
```

Pour un monodépôt, limitez le checkout aux répertoires qui contiennent le contenu du plugin :

```bash theme={null}
claude plugin marketplace add acme-corp/monorepo --sparse .claude-plugin plugins
```

<h3 id="plugin-marketplace-list">
  Plugin marketplace list
</h3>

Listez toutes les places de marché configurées.

```bash theme={null}
claude plugin marketplace list [options]
```

**Options :**

| Option   | Description    |
| :------- | :------------- |
| `--json` | Sortie en JSON |

Avec `--json`, chaque entrée inclut `name`, `source` et des champs spécifiques à la source : `repo` pour les sources GitHub, `url` pour les sources git et URL, et `path` pour les sources locales. Les sources GitHub et git incluent également un champ `ref` lorsque la place de marché a été ajoutée avec une branche ou un tag épinglé.

<h3 id="plugin-marketplace-remove">
  Plugin marketplace remove
</h3>

Supprimez une place de marché configurée. L'alias `rm` est également accepté.

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

**Arguments :**

* `<name>` : nom de la place de marché à supprimer, comme indiqué par `claude plugin marketplace list`. C'est le `name` de `marketplace.json`, pas la source que vous avez passée à `add`

**Options :**

| Option            | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Par défaut           |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `--scope <scope>` | Restreindre la suppression à une seule portée de paramètres : `user`, `project` ou `local`. Voir [Portées d'installation des plugins](/docs/fr/plugins-reference#plugin-installation-scopes). Lorsqu'il est omis, la déclaration est supprimée de chaque portée modifiable. Lorsqu'il est donné, seule la déclaration de cette portée est supprimée ; l'état partagé, le cache et les données des plugins installés sont préservés lorsque la place de marché est toujours déclarée dans une autre portée | (toutes les portées) |

<Warning>
  La suppression d'une place de marché de sa dernière portée restante désinstalle également tous les plugins que vous avez installés à partir de celle-ci. Pour actualiser une place de marché sans perdre les plugins installés, utilisez `claude plugin marketplace update` à la place.
</Warning>

<h3 id="plugin-marketplace-update">
  Plugin marketplace update
</h3>

Actualisez les places de marché à partir de leurs sources pour récupérer les nouveaux plugins et les changements de version. Une place de marché ajoutée avec une branche ou un tag `ref` se met à jour vers le dernier commit de cette ref, pas la branche par défaut du dépôt.

```bash theme={null}
claude plugin marketplace update [name]
```

**Arguments :**

* `[name]` : nom de la place de marché à mettre à jour, comme indiqué par `claude plugin marketplace list`. Met à jour toutes les places de marché si omis

À la fois `remove` et `update` échouent lorsqu'ils sont exécutés contre une place de marché gérée par seed, qui est en lecture seule. Lors de la mise à jour de toutes les places de marché, les entrées gérées par seed sont ignorées et les autres places de marché se mettent toujours à jour. Pour modifier les plugins fournis par seed, demandez à votre administrateur de mettre à jour l'image de seed. Voir [Pré-remplir les plugins pour les conteneurs](#pre-populate-plugins-for-containers).

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="marketplace-not-loading">
  La place de marché ne se charge pas
</h3>

**Symptômes** : Impossible d'ajouter la place de marché ou de voir les plugins qu'elle contient

**Solutions** :

* Vérifiez que l'URL de la place de marché est accessible
* Vérifiez que `.claude-plugin/marketplace.json` existe au chemin spécifié
* Assurez-vous que la syntaxe JSON est valide en utilisant `claude plugin validate` ou `/plugin validate`. Pour vérifier le frontmatter des compétences, agents et commandes, exécutez la commande sur chaque répertoire de plugin
* Pour les dépôts privés, confirmez que vous avez les permissions d'accès

<h3 id="marketplace-validation-errors">
  Erreurs de validation de la place de marché
</h3>

Exécutez `claude plugin validate .` ou `/plugin validate .` à partir de votre répertoire de place de marché pour vérifier les problèmes. Lorsqu'il est pointé sur un répertoire de place de marché, le validateur vérifie `marketplace.json` pour les erreurs de schéma, les noms de plugins en doublon et la traversée de chemin source. Pour chaque entrée dont la `source` est un chemin local, il valide également le `plugin.json` de ce plugin et avertit lorsque la `version` de l'entrée ne correspond pas à celle dans `plugin.json`. Les problèmes trouvés dans le `plugin.json` d'un plugin sont préfixés par l'index d'entrée, sous la forme `plugins[2] plugin.json →`.

À partir de Claude Code v2.1.196, la passe par entrée inclut également :

* les plugins dont la `source` est `.`
* s'exécute lorsque `marketplace.json` est en dehors d'un répertoire `.claude-plugin`, en résolvant les sources par rapport au répertoire du fichier lui-même
* signale les problèmes de chaque entrée même lorsqu'une autre partie du fichier a des erreurs de schéma

Les versions antérieures ignorent les plugins à la racine de la place de marché et ne descendent que depuis un `.claude-plugin/marketplace.json`.

Pour valider le `plugin.json` d'un plugin individuel et ses fichiers de compétence, agent, commande et hook, exécutez la commande sur le répertoire du plugin lui-même, par exemple `claude plugin validate ./plugins/my-plugin`. Erreurs courantes :

| Erreur                                            | Cause                                                               | Solution                                                                                                                                                                    |
| :------------------------------------------------ | :------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: .claude-plugin/marketplace.json` | Manifeste manquant                                                  | Créez `.claude-plugin/marketplace.json` avec les champs obligatoires                                                                                                        |
| `Invalid JSON syntax: Unexpected token...`        | Erreur de syntaxe JSON dans marketplace.json                        | Vérifiez les virgules manquantes, les virgules supplémentaires ou les chaînes non citées                                                                                    |
| `Duplicate plugin name "x" found in marketplace`  | Deux plugins partagent le même nom                                  | Donnez à chaque plugin une valeur `name` unique                                                                                                                             |
| `plugins[0].source: Path contains ".."`           | Le chemin source contient `..`                                      | Utilisez des chemins relatifs à la racine de la place de marché sans `..`. Voir [Chemins relatifs](#relative-paths)                                                         |
| `YAML frontmatter failed to parse: ...`           | YAML invalide dans un fichier de compétence, d'agent ou de commande | Corrigez la syntaxe YAML dans le bloc frontmatter. À l'exécution, ce fichier se charge sans métadonnées. Signalé uniquement lors de la validation d'un répertoire de plugin |
| `Invalid JSON syntax: ...` (hooks.json)           | `hooks/hooks.json` mal formé                                        | Corrigez la syntaxe JSON. Un `hooks/hooks.json` mal formé empêche le chargement du plugin entier. Signalé uniquement lors de la validation d'un répertoire de plugin        |

**Avertissements** (non bloquants) :

* `Marketplace has no plugins defined` : ajoutez au moins un plugin au tableau `plugins`
* `No marketplace description provided` : ajoutez une `description` au niveau supérieur pour aider les utilisateurs à comprendre votre place de marché
* `Plugin name "x" is not kebab-case` : le nom du plugin contient des lettres majuscules, des espaces ou des caractères spéciaux. Renommez en minuscules, chiffres et tirets uniquement (par exemple, `my-plugin`). Claude Code accepte d'autres formes, mais la synchronisation de la place de marché Claude.ai les rejette.

<h3 id="plugin-installation-failures">
  Échecs d'installation de plugins
</h3>

**Symptômes** : La place de marché apparaît mais l'installation du plugin échoue

**Solutions** :

* Vérifiez que les URL sources des plugins sont accessibles
* Vérifiez que les répertoires des plugins contiennent les fichiers requis
* Pour les sources GitHub, assurez-vous que les dépôts sont publics ou que vous avez accès
* Testez manuellement les sources de plugins en les clonant/téléchargeant
* Si la source épingle à la fois `ref` et `sha`, une branche ou un tag en amont supprimé ne bloque pas l'installation sur la plupart des hôtes git, y compris GitHub, GitLab et Bitbucket. Sur les serveurs qui ne supportent pas la récupération des commits par SHA, comme AWS CodeCommit, le `ref` doit toujours exister et le commit épinglé doit être accessible à partir de celui-ci. Si l'installation échoue toujours, confirmez que le commit épinglé existe toujours dans le dépôt

<h3 id="private-repository-authentication-fails">
  L'authentification du dépôt privé échoue
</h3>

**Symptômes** : Erreurs d'authentification lors de l'installation de plugins à partir de dépôts privés

**Solutions** :

Pour l'installation manuelle et les mises à jour :

* Vérifiez que vous êtes authentifié auprès de votre fournisseur git (par exemple, exécutez `gh auth status` pour GitHub)
* Vérifiez que votre assistant de credentials est configuré correctement : `git config --global credential.helper`
* Essayez de cloner le dépôt manuellement pour vérifier que vos credentials fonctionnent

Pour les mises à jour automatiques en arrière-plan :

* Par défaut, les actualisations en arrière-plan désactivent les assistants de credentials git pour la récupération, de sorte que la récupération ne peut pas s'authentifier via HTTPS. Les dépôts SSH avec une clé chargée dans `ssh-agent` s'authentifient toujours. Un échec de récupération déclenche un re-clonage à partir de zéro, qui utilise vos credentials stockés mais peut expirer sur les grands dépôts
* Définissez `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` pour conserver le clone existant lorsque la récupération en arrière-plan échoue
* Configurez un assistant de credentials git, par exemple `gh auth setup-git`, de sorte que le re-clonage de secours puisse s'authentifier
* Si le re-clonage expire sur un grand dépôt, augmentez la limite avec [`CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`](#git-operations-time-out)
* Configurez une [réécriture d'URL git](#private-repositories) limitée au dépôt de la place de marché de sorte que la récupération en arrière-plan s'authentifie directement
* Ou mettez à jour les places de marché privées manuellement avec `/plugin marketplace update <name>`, qui utilise vos credentials

<h3 id="marketplace-updates-fail-in-offline-environments">
  Les mises à jour de la place de marché échouent dans les environnements hors ligne
</h3>

**Symptômes** : Le `git pull` de la place de marché échoue en arrière-plan et Claude Code tente à plusieurs reprises un re-clonage qui ne peut pas réussir.

**Cause** : Par défaut, lorsqu'un `git pull` échoue, Claude Code tente un re-clonage à partir de zéro. Dans les environnements hors ligne ou isolés, le re-clonage échoue de la même manière, et la restauration du cache précédent après est au mieux un effort. L'actualisation s'exécute en arrière-plan après le démarrage, de sorte qu'elle ne retarde pas le démarrage, mais chaque session répète les tentatives échouées et chaque opération git peut attendre le [délai d'expiration de 120 secondes](#git-operations-time-out).

**Solution** : Définissez `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` pour ignorer la tentative de re-clonage et continuer à utiliser le cache existant lorsque la récupération échoue :

```bash theme={null}
export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
```

Avec cette variable définie, Claude Code conserve le clone obsolète de la place de marché en cas d'échec de `git pull` et continue d'utiliser le dernier état connu bon. Pour les déploiements entièrement hors ligne où le dépôt ne sera jamais accessible, utilisez [`CLAUDE_CODE_PLUGIN_SEED_DIR`](#pre-populate-plugins-for-containers) pour pré-remplir le répertoire des plugins au moment de la construction à la place.

<h3 id="git-operations-time-out">
  Les opérations Git expirent
</h3>

**Symptômes** : L'installation du plugin ou les mises à jour de la place de marché échouent avec une erreur de délai d'expiration comme « Git clone timed out after 120s » ou « Git pull timed out after 120s ».

**Cause** : Claude Code utilise un délai d'expiration de 120 secondes pour toutes les opérations git, y compris le clonage des dépôts de plugins et l'extraction des mises à jour de la place de marché. Les grands dépôts ou les connexions réseau lentes peuvent dépasser cette limite.

**Solution** : Augmentez le délai d'expiration en utilisant la variable d'environnement `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`. La valeur est en millisecondes :

```bash theme={null}
export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000  # 5 minutes
```

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Les plugins avec chemins relatifs échouent dans les places de marché basées sur les URL
</h3>

**Symptômes** : Vous avez ajouté une place de marché via URL (comme `https://example.com/marketplace.json`), mais les plugins avec des sources de chemin relatif comme `"./plugins/my-plugin"` échouent à installer avec des erreurs « path not found ».

**Cause** : Les places de marché basées sur les URL téléchargent uniquement le fichier `marketplace.json` lui-même. Elles ne téléchargent pas les fichiers de plugins du serveur. Les chemins relatifs dans l'entrée de la place de marché référencent des fichiers sur le serveur distant qui n'ont pas été téléchargés.

**Solutions** :

* **Utiliser des sources externes** : Changez les entrées de plugins pour utiliser les sources GitHub, npm ou URL git au lieu des chemins relatifs :
  ```json theme={null}
  { "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
  ```
* **Utiliser une place de marché basée sur Git** : Hébergez votre place de marché dans un dépôt Git et ajoutez-la avec l'URL git. Les places de marché basées sur Git clonent le dépôt entier, ce qui rend les chemins relatifs fonctionnels.

<h3 id="files-not-found-after-installation">
  Fichiers non trouvés après l'installation
</h3>

**Symptômes** : Le plugin s'installe mais les références aux fichiers échouent, en particulier les fichiers en dehors du répertoire du plugin

**Cause** : Les plugins sont copiés vers un répertoire de cache plutôt que d'être utilisés sur place. Les chemins qui référencent des fichiers en dehors du répertoire du plugin (comme `../shared-utils`) ne fonctionneront pas car ces fichiers ne sont pas copiés.

**Solutions** : Consultez [Plugin caching and file resolution](/docs/fr/plugins-reference#plugin-caching-and-file-resolution) pour les solutions de contournement, y compris les symlinks et la restructuration des répertoires.

Pour des outils de débogage supplémentaires et des problèmes courants, consultez [Debugging and development tools](/docs/fr/plugins-reference#debugging-and-development-tools).

<h2 id="see-also">
  Voir aussi
</h2>

* [Découvrir et installer des plugins préconfigurés](/docs/fr/discover-plugins) - Installation de plugins à partir de places de marché existantes
* [Plugins](/docs/fr/plugins) - Création de vos propres plugins
* [Référence des plugins](/docs/fr/plugins-reference) - Spécifications techniques complètes et schémas
* [Paramètres des plugins](/docs/fr/settings#plugin-settings) - Options de configuration des plugins
* [Référence strictKnownMarketplaces](/docs/fr/settings#strictknownmarketplaces) - Restrictions des places de marché gérées
