> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence du manifeste de plugin

> Référence complète pour plugin.json : chaque champ avec son type et sa valeur par défaut, les formes de chemin acceptées, et les schémas userConfig et variables d'environnement.

Un manifeste de plugin est le fichier `plugin.json` dans le répertoire `.claude-plugin/` d'un plugin. Il contient les métadonnées du plugin et les valeurs [`userConfig`](#user-configuration) que Claude Code demande à l'utilisateur. Il déclare également tout composant que vous définissez en ligne ou que vous conservez en dehors de son [emplacement par défaut](#standard-layout).

Cette référence est destinée aux créateurs de plugins et aux propriétaires de marketplace qui mettent des champs de composant dans une entrée de marketplace.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Apprendre à créer un plugin** : commencez par [Créer un plugin](/docs/fr/plugins/create)
  * **Ce que chaque composant fait à l'exécution** : voir [Composants de plugin](/docs/fr/plugins/components)
</Note>

Commencez par la section qui correspond à ce que vous recherchez :

* Un champ : le tableau [Champs](#fields) donne le type de chaque champ, s'il est obligatoire, sa valeur par défaut et ce qu'il accepte. [Règles de chemin](#path-rules) couvre le préfixe `./` et le confinement pour chaque chemin de composant
* Une option `userConfig` ou une entrée `channels` : les schémas [Configuration utilisateur](#user-configuration) et [Canaux](#channels)
* `${CLAUDE_PLUGIN_ROOT}` ou une autre variable qu'un plugin peut référencer : [Variables d'environnement](#environment-variables)
* Où vont les fichiers de chaque composant : [Disposition standard](#standard-layout)
* Un message de `claude plugin validate` : la [page de dépannage](/docs/fr/plugins/troubleshooting) liste chaque message avec sa correction et des liens vers les sections pertinentes de cette page

<h2 id="manifest-file">
  Fichier manifeste
</h2>

Le manifeste est optionnel. Sans lui, Claude Code charge les composants qu'il trouve dans la [disposition standard](#standard-layout). Le nom du plugin provient alors de l'entrée de marketplace, ou du nom du répertoire lorsque vous chargez le plugin avec `--plugin-dir`.

Écrivez un manifeste lorsque vous voulez des métadonnées, un composant en dehors de son répertoire par défaut, `userConfig`, ou une définition de composant en ligne.

Enregistrez le manifeste à `.claude-plugin/plugin.json` sous la racine du plugin. Mettez tous les autres fichiers de plugin à la racine du plugin, pas à l'intérieur de `.claude-plugin/`. Cela inclut `skills/`, `commands/`, et `hooks/`.

L'exemple suivant définit la plupart des clés du tableau [Champs](#fields). Il passe la validation dans un répertoire de plugin qui contient chaque chemin référencé.

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

<h3 id="unrecognized-fields">
  Champs non reconnus
</h3>

Une clé de niveau supérieur non reconnue est supprimée, et une clé non reconnue à l'intérieur d'une option `userConfig`, d'une entrée `channels`, d'une config `lspServers`, ou d'une entrée `monitors` est rejetée :

* **Champs de niveau supérieur** : le champ est supprimé et le plugin se charge. `claude plugin validate` signale chaque champ de niveau supérieur non reconnu comme un avertissement
* **Objets stricts** : les options `userConfig`, les entrées `channels`, les configs `lspServers`, et les entrées `monitors` sont stricts. Une clé inconnue à l'intérieur de l'une d'elles est une erreur, et le plugin ne se charge pas

<h3 id="validate-the-manifest">
  Valider le manifeste
</h3>

`claude plugin validate` est la vérification faisant autorité pour un manifeste. Exécutez-le depuis votre shell par rapport au répertoire du plugin :

```bash theme={null}
claude plugin validate ./my-plugin
```

La commande signale l'un de ces résultats :

* **`Validation passed`** : le manifeste se charge
* **`Validation passed with warnings`** : le manifeste se charge, mais le validateur a trouvé quelque chose à corriger, comme un champ de niveau supérieur inconnu que Claude Code supprime, un `name` qui n'est pas en kebab-case, ou un `version`, `description`, ou `author` manquant. Passez `--strict` pour transformer les avertissements en échecs dans CI
* **`Validation failed`** : le manifeste a une incompatibilité de type, un chemin qui est manquant ou s'échappe de la racine du plugin, ou une clé inconnue à l'intérieur d'une option `userConfig`, d'une entrée `channels`, d'une config `lspServers`, ou d'une entrée `monitors`. Claude Code signale le même problème lorsqu'il charge le plugin

<h2 id="fields">
  Champs
</h2>

Le tableau liste les clés de niveau supérieur dans `plugin.json`. `name` est la seule clé obligatoire. Lorsqu'un nom de champ est un lien, la section liée a ses règles complètes.

Pour les clés de composant telles que `commands` et `hooks`, [Formes de chemin de composant](#component-path-forms) montre chaque forme acceptée avec un exemple, et chaque chemin suit les [règles de chemin](#path-rules) pour le préfixe `./`, les extensions, et le confinement.

| Champ                                | Type                             | Description                                                                                                                                                                                                                                                                                                                                                              |
| :----------------------------------- | :------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                            | String                           | URL du schéma JSON pour l'autocomplétion de l'éditeur. Claude Code l'ignore au moment du chargement                                                                                                                                                                                                                                                                      |
| [`name`](#name)                      | String                           | Identifiant du plugin, obligatoire. Utilisez kebab-case. Chaque composant est espacé de noms sous celui-ci                                                                                                                                                                                                                                                               |
| [`displayName`](#displayname)        | String                           | Nom affiché dans l'interface utilisateur à la place de `name`                                                                                                                                                                                                                                                                                                            |
| [`version`](#version)                | String                           | Chaîne de version. La définir maintient les utilisateurs sur cette version jusqu'à ce que vous la changiez                                                                                                                                                                                                                                                               |
| `description`                        | String                           | Explication brève de ce que le plugin fournit                                                                                                                                                                                                                                                                                                                            |
| `author`                             | Object                           | `name`, qui est obligatoire, plus `email` et `url` optionnels                                                                                                                                                                                                                                                                                                            |
| `homepage`                           | String                           | URL de documentation. Doit être analysée comme une URL, sinon le plugin ne se charge pas                                                                                                                                                                                                                                                                                 |
| `repository`                         | String                           | URL du référentiel source. Non validée                                                                                                                                                                                                                                                                                                                                   |
| `license`                            | String                           | Identifiant SPDX tel que `MIT` ou `Apache-2.0`                                                                                                                                                                                                                                                                                                                           |
| `keywords`                           | Array of strings                 | Balises de découverte                                                                                                                                                                                                                                                                                                                                                    |
| [`metadata`](#metadata)              | Object                           | Objet de forme libre pour vos propres données. Claude Code ne le lit pas                                                                                                                                                                                                                                                                                                 |
| [`defaultEnabled`](#defaultenabled)  | Boolean                          | Si le plugin démarre activé lorsque l'utilisateur ne l'a pas défini. Par défaut `true`                                                                                                                                                                                                                                                                                   |
| [`dependencies`](#dependencies)      | Array of strings or objects      | Plugins qui doivent être activés pour que celui-ci fonctionne                                                                                                                                                                                                                                                                                                            |
| [`settings`](#settings)              | Object                           | Paramètres que Claude Code applique tandis que le plugin est activé. Seuls `agent` et `subagentStatusLine` prennent effet                                                                                                                                                                                                                                                |
| [`userConfig`](#user-configuration)  | Object                           | Valeurs que Claude Code demande à l'utilisateur lorsque le plugin est activé                                                                                                                                                                                                                                                                                             |
| [`channels`](#channels)              | Array of objects                 | Canaux de message que le plugin fournit, chacun lié à l'un de ses serveurs MCP                                                                                                                                                                                                                                                                                           |
| `skills`                             | Path, or array of paths          | Répertoires à analyser pour les skills, chacun étant un répertoire de dossiers `<name>/SKILL.md` ou un dossier contenant directement `SKILL.md`. `"."` nomme la racine du plugin. S'ajoute à l'analyse par défaut `skills/`                                                                                                                                              |
| [`commands`](#commands)              | Path, array of paths, or object  | Fichiers de commande `.md` plats, répertoires de ceux-ci, ou une carte d'objets du nom de commande à `source` ou `content`. Remplace l'analyse par défaut `commands/`                                                                                                                                                                                                    |
| `agents`                             | Path, or array of paths          | Fichiers d'agent `.md`. Les répertoires ne sont pas acceptés. Remplace l'analyse par défaut `agents/`                                                                                                                                                                                                                                                                    |
| [`hooks`](#hooks)                    | Path, object, or array of either | Fichiers hook `.json` ou config hook en ligne. Chargés ensemble avec `hooks/hooks.json`                                                                                                                                                                                                                                                                                  |
| [`mcpServers`](#mcpservers)          | Path, object, or array of either | Fichiers config MCP `.json`, bundles `.mcpb` ou `.dxt`, ou configs de serveur en ligne clés par nom. Chargés ensemble avec `.mcp.json` ; un nom de serveur déclaré plus tard remplace un nom antérieur                                                                                                                                                                   |
| [`lspServers`](#lspservers)          | Path, object, or array of either | Fichiers config LSP `.json` ou configs de serveur en ligne clés par nom. Chargés ensemble avec `.lsp.json`                                                                                                                                                                                                                                                               |
| `outputStyles`                       | Path, or array of paths          | Fichiers de style de sortie ou répertoires. Remplace l'analyse par défaut `output-styles/`                                                                                                                                                                                                                                                                               |
| `workflows`                          | Path, or array of paths          | Fichiers [Workflow](/docs/fr/workflows#distribute-a-workflow-in-a-plugin) `.js` ou répertoires. Remplace l'analyse par défaut `workflows/`                                                                                                                                                                                                                                    |
| `experimental`                       | Object                           | Conteneur pour `themes`, `monitors`, et `evals`, dont la forme de manifeste peut encore changer                                                                                                                                                                                                                                                                          |
| `experimental.themes`                | Path, or array of paths          | Fichiers de thème ou répertoires. Remplace l'analyse par défaut `themes/`. Une clé `themes` de niveau supérieur se charge toujours, avec un avertissement `claude plugin validate`                                                                                                                                                                                       |
| [`experimental.monitors`](#monitors) | Path, or inline array            | Un fichier `.json` contenant le tableau monitors, ou le tableau lui-même. Par défaut `monitors/monitors.json`. Une clé `monitors` de niveau supérieur se charge toujours, avec un avertissement `claude plugin validate`. Les monitors ne s'exécutent que dans les sessions interactives, et non sur Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry |
| `experimental.evals`                 | Path, or array of paths          | Répertoire qui contient les [cas d'évaluation](/docs/fr/plugin-evals#use-a-different-eval-directory) du plugin lorsqu'il n'est pas le répertoire par défaut `evals/`. `claude plugin eval --eval-dir` le remplace                                                                                                                                                             |

Dans la colonne Type, un chemin est une chaîne relative à la racine du plugin, comme `"./custom/commands"`.

<h3 id="name">
  `name`
</h3>

L'identifiant du plugin. Il doit être non vide, sans espaces, `@`, `:`, séparateurs de chemin, caractères de contrôle, ou caractères de formatage bidirectionnel ; utilisez kebab-case.

Claude Code espace de noms chaque composant sous celui-ci, donc un agent `reviewer` dans le plugin `deploy-tools` apparaît comme `deploy-tools:reviewer`.

<h3 id="displayname">
  `displayName`
</h3>

Le nom affiché dans l'interface utilisateur à la place de `name`. Il peut contenir des espaces et n'importe quelle casse, et il n'est pas utilisé pour l'espacement de noms ou la recherche.

Pour un plugin installé depuis le marketplace, un `displayName` sur l'[entrée de marketplace](/docs/fr/plugins/marketplace-reference#plugin-entries) prend précédence sur cette valeur.

<h3 id="version">
  `version`
</h3>

Une chaîne de version, non vérifiée par rapport à semver. La définir épingle le plugin à cette version jusqu'à ce que vous la changiez ; voir [Versions et mises à jour](/docs/fr/plugins/loading#versions-and-updates). Un plugin avec une [`command` source](/docs/fr/plugins/marketplace-reference), un plugin d'un [marketplace hébergé sur claude.ai](/docs/fr/plugins/install#add-from-claude-ai), et un plugin [chargé sur place](/docs/fr/plugins/loading#find-plugins-on-disk) à partir d'un marketplace ajouté en tant que répertoire local ne sont pas épinglés par ce champ.

<h3 id="metadata">
  `metadata`
</h3>

Un objet de forme libre pour vos propres données, comme des champs de catalogue ou de droit. Claude Code ne le lit pas. Nécessite Claude Code v2.1.222 ou ultérieur.

<h3 id="defaultenabled">
  `defaultEnabled`
</h3>

Si le plugin démarre activé lorsque l'utilisateur ne l'a pas défini dans [`enabledPlugins`](/docs/fr/settings-reference#enabledplugins). Par défaut `true`. Un plugin qu'un plugin activé dépend démarre activé indépendamment. Le même champ dans l'entrée de marketplace remplace celui-ci.

Une fois qu'une entrée `enabledPlugins` d'un utilisateur est écrite, elle persiste à travers les mises à jour de plugin, donc changer `defaultEnabled` dans une version ultérieure ne change pas le paramètre pour un utilisateur existant.

<h3 id="dependencies">
  `dependencies`
</h3>

Plugins qui doivent être activés pour que celui-ci fonctionne. Chaque entrée est `"name"`, `"name@marketplace"`, ou `{ "name": "...", "marketplace": "...", "version": "..." }`. Les noms nus se résolvent par rapport au propre marketplace de ce plugin. Voir [contraintes de dépendance](/docs/fr/plugins/dependencies).

<h3 id="settings">
  `settings`
</h3>

Paramètres que Claude Code applique tandis que le plugin est activé. Seuls `agent` et `subagentStatusLine` prennent effet ; les autres clés sont supprimées au chargement. Un `settings.json` à la racine du plugin prend précédence sur cette clé. Voir [Paramètres par défaut](/docs/fr/plugins/components#default-settings).

<h2 id="component-path-forms">
  Formes de chemin de composant
</h2>

Chaque clé de composant accepte un chemin relatif à la racine du plugin. `hooks`, `mcpServers`, `lspServers`, et `experimental.monitors` acceptent également une configuration en ligne, `commands` accepte également une carte d'objets, et `mcpServers` accepte également des chemins de bundle MCP et des URL. Les exemples qui suivent montrent chaque forme acceptée une fois. Pour ce que chaque composant fait à l'exécution, voir [Composants de plugin](/docs/fr/plugins/components).

<h3 id="path-only-fields">
  Champs réservés au chemin
</h3>

`agents`, `skills`, `outputStyles`, `workflows`, et `experimental.themes` prennent un chemin ou un tableau de chemins. Les entrées `agents` doivent être des fichiers `.md`, et les entrées `skills` doivent être des répertoires. Les trois autres acceptent un répertoire ou un fichier.

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

<h3 id="commands">
  `commands`
</h3>

`commands` prend un chemin, un tableau de chemins, ou une carte d'objets. Un chemin nomme un fichier de commande `.md` plat ou un répertoire. Dans la carte d'objets, chaque clé devient le nom de la commande après le préfixe du plugin. Par exemple, `"about"` dans le plugin `deploy-tools` s'exécute comme `/deploy-tools:about`.

Chaque valeur définit exactement l'une de `source` ou `content`, et une entrée qui définit les deux ou aucune échoue la validation. Les autres champs de ce tableau sont optionnels :

| Champ          | Type             | Description                                                                   |
| :------------- | :--------------- | :---------------------------------------------------------------------------- |
| `source`       | string           | Chemin vers le fichier Markdown de la commande, relatif à la racine du plugin |
| `content`      | string           | Markdown en ligne pour le corps de la commande, au lieu de `source`           |
| `description`  | string           | Description affichée pour la commande                                         |
| `argumentHint` | string           | Indice d'argument affiché après le nom de la commande, comme `[file]`         |
| `model`        | string           | Modèle par défaut pour la commande                                            |
| `allowedTools` | array of strings | Outils que la commande peut utiliser sans demander                            |

Cette carte déclare une commande à partir d'un fichier et une à partir du contenu en ligne :

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

<h3 id="hooks">
  `hooks`
</h3>

`hooks` prend un chemin de fichier `.json`, un objet hooks en ligne dans la même forme que [`hooks` dans `settings.json`](/docs/fr/hooks#configuration), ou un tableau mélangeant les deux. Pour les événements hook et les champs de gestionnaire, voir la [référence hooks](/docs/fr/hooks#hook-events).

Claude Code fusionne tout ce que vous déclarez avec `hooks/hooks.json` lorsque ce fichier existe.

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

<h3 id="mcpservers">
  `mcpServers`
</h3>

`mcpServers` prend un chemin de fichier `.json`, un chemin de bundle MCP ou une URL, une carte en ligne, ou un tableau mélangeant les deux. Pour les champs de config de serveur, voir [serveurs MCP fournis par plugin](/docs/fr/mcp#plugin-provided-mcp-servers).

Claude Code charge `.mcp.json` à la racine du plugin en premier, puis chaque forme déclarée dans l'ordre. Un nom de serveur déclaré plus tard remplace un nom antérieur.

Une valeur `mcpServers` prend l'une de ces formes :

| Forme                     | Exemple de valeur                                                                      | Ce que Claude Code fait                                                                                      |
| :------------------------ | :------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| Chemin de fichier `.json` | `"./mcp/servers.json"`                                                                 | Lit le fichier comme une carte `mcpServers`                                                                  |
| Chemin de bundle MCP      | `"./bundle.mcpb"`                                                                      | Extrait le bundle `.mcpb` ou `.dxt` dans `.mcpb-cache/` sous la racine du plugin et lit sa config de serveur |
| URL de bundle MCP         | `"https://example.com/server.mcpb"`                                                    | Télécharge le bundle dans `.mcpb-cache/`, puis le lit                                                        |
| Carte en ligne            | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | Utilise la carte comme configs de serveur clés par nom                                                       |

Un chemin de bundle ou une URL doit se terminer par `.mcpb` ou `.dxt`. Toute autre extension échoue la validation.

<h3 id="lspservers">
  `lspServers`
</h3>

`lspServers` prend un chemin de fichier `.json`, une carte en ligne du nom de serveur à la config, ou un tableau de l'un ou l'autre.

Claude Code charge `.lsp.json` à la racine du plugin en premier, puis chaque config déclarée dans l'ordre. Un nom de serveur déclaré plus tard remplace un nom antérieur.

Chaque config de serveur est un objet strict avec ces champs. Une clé inconnue échoue la validation.

| Champ                   | Obligatoire | Description                                                                                                                                                                                                           |
| :---------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`               | Yes         | Binaire du serveur de langage. Pas d'espaces sauf si la valeur commence par `/` ; mettez les arguments dans `args`                                                                                                    |
| `extensionToLanguage`   | Yes         | Carte de l'extension de fichier à l'ID de langage LSP, au moins une entrée. Les clés commencent par un point, comme `".go"`                                                                                           |
| `args`                  | No          | Arguments passés au serveur                                                                                                                                                                                           |
| `transport`             | No          | Transport de communication : `stdio` (par défaut) ou `socket`. Claude Code accepte `socket` mais exécute chaque serveur sur stdio, donc les règles du protocole stdout s'appliquent à tous les serveurs               |
| `env`                   | No          | Variables d'environnement pour le processus du serveur                                                                                                                                                                |
| `initializationOptions` | No          | Options envoyées dans la demande d'initialisation                                                                                                                                                                     |
| `settings`              | No          | Paramètres envoyés par `workspace/didChangeConfiguration`                                                                                                                                                             |
| `workspaceFolder`       | No          | Chemin du dossier d'espace de travail pour le serveur                                                                                                                                                                 |
| `startupTimeout`        | No          | Millisecondes à attendre pour le démarrage, un entier positif                                                                                                                                                         |
| `shutdownTimeout`       | No          | Millisecondes à attendre pour un arrêt gracieux, un entier positif. Lorsque le délai d'attente s'écoule, Claude Code termine le processus du serveur. Lorsqu'il n'est pas défini, aucun délai d'attente ne s'applique |
| `restartOnCrash`        | No          | Si le serveur doit redémarrer après un crash. Par défaut `true`. Définissez à `false` pour laisser un serveur planté arrêté au lieu de le redémarrer                                                                  |
| `maxRestarts`           | No          | Tentatives de redémarrage avant d'abandonner, zéro ou plus                                                                                                                                                            |
| `diagnostics`           | No          | Si les diagnostics doivent être poussés dans le contexte après les éditions. Par défaut `true`                                                                                                                        |

Cette config en ligne exécute `gopls` pour les fichiers `.go` :

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

Pour les serveurs de langage qu'Anthropic publie en tant que plugins et comment les serveurs se comportent à l'exécution, voir [Intelligence du code](/docs/fr/plugins/code-intelligence).

<h3 id="monitors">
  `monitors`
</h3>

`experimental.monitors` prend un chemin de fichier `.json` ou le tableau en ligne. Lorsque vous omettez la clé, Claude Code charge `monitors/monitors.json` s'il existe.

Chaque entrée est un objet strict avec ces champs.

| Champ         | Obligatoire | Description                                                                                                                                                                                                |
| :------------ | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Yes         | Identifiant unique au sein du plugin                                                                                                                                                                       |
| `command`     | Yes         | Commande shell que Claude Code exécute en tant que processus d'arrière-plan persistant dans le répertoire de travail de la session                                                                         |
| `description` | Yes         | Résumé court affiché dans le panneau des tâches et les résumés de notification                                                                                                                             |
| `when`        | No          | Avec `"always"`, la valeur par défaut, le monitor démarre au démarrage de la session et au rechargement du plugin. Avec `"on-skill-invoke:<skill>"`, il démarre la première fois que cette skill s'exécute |

Ce tableau en ligne déclare un monitor qui démarre la première fois que la skill `deploy` s'exécute :

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

Une commande `command` de monitor ne peut pas référencer `${user_config.*}`. Voir [Champs qui s'exécutent via un shell](#fields-that-run-through-a-shell).

<h2 id="path-rules">
  Règles de chemin
</h2>

Chaque chemin de composant dans un manifeste est relatif à la racine du plugin et doit commencer par `./`. Un chemin comme `commands/foo.md` échoue la validation. `skills` et `mcpServers` acceptent chacun une forme en dehors de cette règle :

* **`skills`** : accepte également `"."`. À la fois `"."` et `"./"` désignent la racine du plugin. Avant v2.1.221, `"."` échouait la validation du manifeste, donc utilisez `"./"` lorsque le plugin doit se charger sur les versions antérieures
* **`mcpServers`** : accepte également une URL de bundle `https://`

<h3 id="containment-and-existence">
  Confinement et existence
</h3>

Chaque chemin de composant doit se résoudre à l'intérieur de la racine du plugin et doit exister. `claude plugin validate` ne vérifie pas les chemins `outputStyles`, `lspServers`, `monitors`, ou `themes`, donc un mauvais chemin dans ces champs échoue uniquement lorsque le plugin se charge :

* **Confinement** : un chemin qui se résout en dehors de la racine du plugin ne se charge pas, et l'onglet **Errors** `/plugin` affiche `<component> path escapes plugin directory: <path>`. Un chemin contenant `..` est le cas habituel, et `claude plugin validate` le signale comme `Path contains ".." which could be a path traversal attempt`
* **Existence** : un chemin qui n'existe pas ne se charge pas, et l'onglet **Errors** `/plugin` affiche `<component> path not found: <path>`. `claude plugin validate` le signale comme `Path not found`

<h3 id="how-each-key-combines-with-its-default-location">
  Comment chaque clé se combine avec son emplacement par défaut
</h3>

Chaque clé de composant remplace son emplacement par défaut, s'y ajoute, ou le fusionne :

* **Remplace la valeur par défaut** : `commands`, `agents`, `outputStyles`, `workflows`, `experimental.themes`, `experimental.monitors`. Lorsque vous définissez `commands`, le répertoire par défaut `commands/` n'est pas analysé. Pour conserver la valeur par défaut et en ajouter d'autres, listez-la explicitement : `"commands": ["./commands/", "./extras/"]`
* **S'ajoute à la valeur par défaut** : `skills`. Le répertoire `skills/` est toujours analysé, et les répertoires listés se chargent à côté de celui-ci
* **Fusionne** : `hooks`, `mcpServers`, `lspServers`. Le fichier par défaut se charge en premier, et ce que le manifeste déclare fusionne avec celui-ci, comme décrit sous [Formes de chemin de composant](#component-path-forms)

Si un plugin a un dossier par défaut comme `commands/` et définit également la clé de manifeste qui le remplace, Claude Code charge les chemins du manifeste et non le dossier. `claude plugin list` et l'interface `/plugin` affichent alors l'avertissement `Default <folder>/ folder is ignored because the manifest sets "<key>"`.

Pour éviter l'avertissement, définissez la clé sur un chemin à l'intérieur de ce dossier : `"commands": ["./commands/deploy.md"]` nomme un fichier dans le dossier par défaut et ne produit aucun avertissement.

<h2 id="user-configuration">
  Configuration utilisateur
</h2>

`userConfig` déclare les valeurs que Claude Code demande à l'utilisateur lorsque le plugin est activé, afin que les utilisateurs ne modifient pas `settings.json` eux-mêmes.

Les clés sont des identifiants composés de lettres, de chiffres et de traits de soulignement, et ne peuvent pas commencer par un chiffre.

Chaque valeur est un objet strict avec ces champs. Une clé inconnue échoue la validation.

| Champ         | Obligatoire | Description                                                                                                                                                                                                             |
| :------------ | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`        | Yes         | L'un de `string`, `number`, `boolean`, `directory`, ou `file`                                                                                                                                                           |
| `title`       | Yes         | Étiquette affichée dans la boîte de dialogue de configuration                                                                                                                                                           |
| `description` | Yes         | Texte d'aide affiché sous le champ                                                                                                                                                                                      |
| `required`    | No          | Si `true`, la boîte de dialogue de configuration n'accepte pas une valeur vide                                                                                                                                          |
| `default`     | No          | Valeur utilisée lorsque l'utilisateur ne fournit rien : une chaîne, un nombre, un booléen, ou un tableau de chaînes                                                                                                     |
| `options`     | No          | Pour `string`, les valeurs que le champ accepte, affichées comme un sélecteur dans `/config`. Voir [Limiter un champ à des options fixes](#limit-a-field-to-fixed-options). Nécessite Claude Code v2.1.271 ou ultérieur |
| `multiple`    | No          | Pour `string`, permet un tableau de chaînes                                                                                                                                                                             |
| `sensitive`   | No          | Si `true`, masque l'entrée et stocke la valeur dans le stockage sécurisé au lieu de `settings.json`                                                                                                                     |
| `min` / `max` | No          | Limites pour `number`                                                                                                                                                                                                   |

Chaque option de chaque plugin activé apparaît également comme une ligne dans le panneau `/config`, sauf les options `sensitive` et les listes `multiple`. Les lignes `/config` nécessitent Claude Code v2.1.269 ou ultérieur.

Cette `userConfig` déclare un point de terminaison et un jeton masqué :

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

<h3 id="limit-a-field-to-fixed-options">
  Limiter un champ à des options fixes
</h3>

Définissez `options` sur un champ `userConfig` pour que les utilisateurs choisissent sa valeur dans une liste fixe.

Pour limiter un champ `tone` à trois options, listez-les dans `options` et définissez `default` sur l'une d'elles :

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

Si vous déclarez `options` sur n'importe quel champ, les utilisateurs sur les versions de Claude Code antérieures à v2.1.271 ne peuvent pas charger le plugin.

`options` s'applique à un champ `string` qui n'est pas `multiple` ou `sensitive`. Définissez `default` sur l'une des valeurs listées, ou définissez `required: true` afin que l'utilisateur en choisisse une. Chaque option est une étiquette simple de 1 à 64 caractères, et `claude plugin validate`, que vous exécutez dans votre shell, signale tout ce qu'il rejette. Un plugin dont `options` cassent ces règles ne se charge pas.

<h3 id="where-values-are-stored">
  Où les valeurs sont stockées
</h3>

Les valeurs non sensibles sont enregistrées sous [`pluginConfigs`](/docs/fr/settings-reference#pluginconfigs) dans le `settings.json` de l'utilisateur. Les valeurs sensibles vont au stockage de credentials sécurisé de la plateforme à la place. La [page des paramètres](/docs/fr/settings-reference#pluginconfigs) liste les fichiers de paramètres à partir desquels `pluginConfigs` est lu.

<h3 id="reference-a-saved-value">
  Référencer une valeur enregistrée
</h3>

Référencez une valeur enregistrée où le plugin en a besoin, dans l'une de ces deux formes :

* **`${user_config.KEY}`** : substitué dans la config du serveur MCP, la config du serveur LSP, les `args` du hook [exec-form](/docs/fr/hooks#exec-form-and-shell-form), et le contenu de skill et d'agent. Dans le contenu de skill et d'agent, seules les valeurs non sensibles sont substituées, et une valeur sensible là devient un placeholder
* **`CLAUDE_PLUGIN_OPTION_<KEY>`** : exporté aux processus hook pour chaque option, avec `<KEY>` en majuscules. Un hook de forme shell lit `$CLAUDE_PLUGIN_OPTION_API_TOKEN` pour `api_token`

<h3 id="fields-that-run-through-a-shell">
  Champs qui s'exécutent via un shell
</h3>

Les commandes hook de forme shell, les commandes monitor, et le MCP [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication) rejettent `${user_config.*}`. Un composant qui le référence dans l'un de ces champs échoue avec une [erreur](/docs/fr/errors#plugin-command-references-user-config) au lieu de s'exécuter, car la valeur du champ est passée à un shell qui ré-analyserait la valeur substituée.

Le tableau montre comment la valeur peut atteindre chacun de ces champs à la place.

| Champ                         | Comment la valeur peut l'atteindre                                                                                                                                                                                               |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Commandes hook de forme shell | Utilisez [exec form](/docs/fr/hooks#exec-form-and-shell-form) avec `args`, ou lisez `CLAUDE_PLUGIN_OPTION_<KEY>` à partir de l'environnement du hook                                                                                  |
| Commandes monitor             | Pas via Claude Code. Les processus monitor ne reçoivent pas `CLAUDE_PLUGIN_OPTION_<KEY>`, donc le script monitor doit obtenir la valeur par lui-même                                                                             |
| MCP `headersHelper`           | Pas via Claude Code. L'environnement du helper porte `CLAUDE_PLUGIN_ROOT`, `CLAUDE_CODE_MCP_SERVER_NAME`, et `CLAUDE_CODE_MCP_SERVER_URL` mais aucune valeur d'option, donc le script helper doit obtenir la valeur par lui-même |

<h2 id="channels">
  Canaux
</h2>

`channels` déclare les canaux de message qu'un plugin fournit, comme un pont vers une application de chat. Lorsque vous en déclarez un, Claude Code peut demander la configuration du canal lorsque le plugin est activé. Pour comment le serveur injecte les messages, voir la [référence des canaux](/docs/fr/channels-reference#package-as-a-plugin).

Chaque entrée est un objet strict lié à l'un des serveurs MCP du plugin, avec ces champs :

| Champ         | Obligatoire | Description                                                                                                                                                                                         |
| :------------ | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server`      | Yes         | Clé du serveur MCP dans le `mcpServers` de ce plugin auquel le canal se lie                                                                                                                         |
| `displayName` | No          | Nom affiché dans le titre de la boîte de dialogue de configuration. Par défaut le nom du serveur                                                                                                    |
| `userConfig`  | No          | Options à demander, dans la même forme que [top-level `userConfig`](#user-configuration). Les valeurs enregistrées se substituent dans les références `${user_config.KEY}` dans le `env` du serveur |

Ce manifeste lie un canal au serveur MCP `telegram` du plugin et demande un jeton de bot qui se substitue dans le `env` du serveur :

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

<h2 id="environment-variables">
  Variables d'environnement
</h2>

Claude Code fournit trois variables de chemin aux composants de plugin. Référencez-les comme `${NAME}` dans les champs listés sous [Où chaque variable se résout](#where-each-variable-resolves), et lisez-les comme variables d'environnement dans les processus qui les reçoivent.

| Variable                | Se résout à                                                                                                                                                                                                                           | Utilisez-la pour                                                    |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------ |
| `${CLAUDE_PLUGIN_ROOT}` | Chemin absolu de la version installée du plugin                                                                                                                                                                                       | Scripts, binaires, et fichiers config regroupés avec le plugin      |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`, créé à la première référence et conservé à travers les mises à jour de plugin. `<id>` est l'identifiant du plugin avec chaque caractère autre qu'une lettre, un chiffre, `_`, ou `-` remplacé par `-` | Dépendances installées comme `node_modules`, code généré, et caches |
| `${CLAUDE_PROJECT_DIR}` | La racine du projet                                                                                                                                                                                                                   | Scripts et fichiers config locaux au projet                         |

`${CLAUDE_PLUGIN_ROOT}` change lorsque le plugin se met à jour, donc n'écrivez pas d'état là. Pour où la racine se déplace et quand l'ancien répertoire est nettoyé, voir la [page de chargement](/docs/fr/plugins/loading).

Lorsque vous désinstallez le plugin du dernier endroit où il est installé, le répertoire `${CLAUDE_PLUGIN_DATA}` est supprimé sauf si vous passez [`--keep-data`](/docs/fr/plugins/cli-reference).

<h3 id="where-each-variable-resolves">
  Où chaque variable se résout
</h3>

Dans chaque composant de plugin, les références `${...}` se résolvent en ligne dans des champs spécifiques, et certains composants reçoivent également les variables dans leur environnement de processus :

| Composant de plugin                       | Champs où `${...}` se résout                | Exporté au processus                                                                              |
| :---------------------------------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------ |
| Commandes hook                            | N'importe où dans `command` et `args`       | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`, et `CLAUDE_PLUGIN_OPTION_<KEY>` |
| Commandes monitor                         | N'importe où dans `command`                 | Non exporté                                                                                       |
| Serveurs MCP `stdio`                      | `command`, `args`, `env`                    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`                                                        |
| Serveurs MCP `http`, `sse`, `ws`          | `url`, `headers`, `headersHelper`           | Non applicable                                                                                    |
| Serveurs LSP                              | `command`, `args`, `env`, `workspaceFolder` | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`                                  |
| Contenu de skill, de commande, et d'agent | N'importe où dans le corps Markdown         | Non applicable                                                                                    |

Les variables ne sont pas présentes dans l'environnement des commandes que Claude exécute via l'outil Bash, dans la session principale ou dans un sous-agent. Dans le contenu de skill, de commande, et d'agent, écrivez la référence `${...}` dans le corps Markdown à la place, et Claude Code substitue le chemin en ligne lorsqu'il charge le contenu.

<h3 id="quoting-and-path-separators">
  Guillemets et séparateurs de chemin
</h3>

Gardez chaque chemin substitué comme un seul argument :

* **Commandes hook** : utilisez [exec form](/docs/fr/hooks#exec-form-and-shell-form) avec `args` afin que chaque chemin soit un argument sans guillemets
* **Hooks de forme shell et commandes monitor** : enveloppez la variable entre guillemets doubles afin qu'un chemin avec des espaces reste un mot

Ce hook de forme shell exécute un script regroupé avec le plugin :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

Sur Windows, les chemins substitués utilisent des barres obliques avant afin qu'un shell ne lise pas les barres obliques inverses comme des échappements.

<h2 id="standard-layout">
  Disposition standard
</h2>

Chaque type de composant a un emplacement par défaut sous la racine du plugin, utilisé lorsque le manifeste ne pointe pas ailleurs.

| Composant        | Emplacement par défaut       | Contenu                                                                                                                                                                                                                                                                                                                                                          |
| :--------------- | :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Manifeste        | `.claude-plugin/plugin.json` | Métadonnées et configuration du plugin. Optionnel                                                                                                                                                                                                                                                                                                                |
| Skills           | `skills/`                    | Un `<name>/SKILL.md` par skill. Un plugin avec `SKILL.md` à sa racine, pas de `skills/`, et pas de clé `skills` se charge comme une seule skill                                                                                                                                                                                                                  |
| Commandes        | `commands/`                  | Fichiers de commande Markdown plats. Préférez `skills/` pour les nouveaux plugins                                                                                                                                                                                                                                                                                |
| Agents           | `agents/`                    | Fichiers Markdown d'agent. Les sous-dossiers font partie du [nom d'agent](/docs/fr/plugins/components#agents)                                                                                                                                                                                                                                                         |
| Hooks            | `hooks/hooks.json`           | Configuration des hooks                                                                                                                                                                                                                                                                                                                                          |
| Serveurs MCP     | `.mcp.json`                  | Définitions des serveurs MCP                                                                                                                                                                                                                                                                                                                                     |
| Serveurs LSP     | `.lsp.json`                  | Configurations des serveurs LSP                                                                                                                                                                                                                                                                                                                                  |
| Styles de sortie | `output-styles/`             | Fichiers de style de sortie Markdown                                                                                                                                                                                                                                                                                                                             |
| Workflows        | `workflows/`                 | Fichiers Workflow `.js`                                                                                                                                                                                                                                                                                                                                          |
| Thèmes           | `themes/`                    | Fichiers de thème JSON                                                                                                                                                                                                                                                                                                                                           |
| Monitors         | `monitors/monitors.json`     | Le tableau monitors                                                                                                                                                                                                                                                                                                                                              |
| Exécutables      | `bin/`                       | Les fichiers ici sont sur le `PATH` de l'outil Bash tandis que le plugin est activé, donc Claude les exécute comme des commandes nues. claude.ai et Cowork n'installent pas un plugin qui a ce répertoire, y compris un que vous [distribuez via les paramètres d'organisation claude.ai](/docs/fr/plugins/host-marketplace#distribute-through-organization-settings) |
| Paramètres       | `settings.json`              | Valeurs par défaut `agent` et `subagentStatusLine` appliquées tandis que le plugin est activé                                                                                                                                                                                                                                                                    |

Un plugin qui utilise chaque emplacement par défaut, plus un dossier `scripts/` que ses hooks appellent, est disposé comme ceci :

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

Pour cliquer à travers cette disposition et lire ce que chaque fichier fait, ouvrez l'[explorateur de plugin](/docs/fr/plugins/components#explore-the-plugin-directory).

Un `CLAUDE.md` à la racine du plugin n'est pas chargé comme contexte, et `claude plugin validate` avertit lorsqu'il en trouve un. Pour inclure des instructions qui se chargent dans le contexte de Claude, mettez-les dans une skill.

<h2 id="marketplace-entries-and-the-manifest">
  Entrées de marketplace et le manifeste
</h2>

Une [entrée de marketplace](/docs/fr/plugins/marketplace-reference) accepte chaque champ de cette page à côté de [ses propres champs](/docs/fr/plugins/marketplace-reference#plugin-entries), y compris `strict`.

Le champ `strict` décide si l'entrée peut ajouter des composants à un plugin qui a son propre `plugin.json`. Il est par défaut `true`.

<h3 id="how-entry-fields-combine-with-plugin-json">
  Comment les champs d'entrée se combinent avec `plugin.json`
</h3>

L'entrée sert soit de manifeste, ajoute des composants à celui-ci, soit entre en conflit avec celui-ci :

* **Pas de `plugin.json`** : l'entrée est le manifeste, indépendamment de `strict`. Les `hooks` d'entrée se chargent uniquement dans la forme d'objet en ligne. Pour un chemin de fichier ou un tableau là, l'onglet **Errors** `/plugin` affiche une erreur `not yet supported in a marketplace entry`
* **`plugin.json` présent, `strict` non défini ou `true`** : Claude Code charge le manifeste et ajoute les `commands`, `agents`, `skills`, `outputStyles`, et `themes` de l'entrée à celui-ci. Pour `hooks`, les matchers de l'entrée pour un événement remplacent les matchers du manifeste pour ce même événement, et les événements que seul le manifeste déclare gardent les leurs
* **`plugin.json` présent, `strict: false`** : une entrée qui déclare l'un de `commands`, `agents`, `skills`, `hooks`, `outputStyles`, ou `themes` est un conflit, et le plugin ne se charge pas avec `Plugin <name> has conflicting manifests`

Lorsqu'une [entrée de marketplace dont la `source` est la racine du marketplace](/docs/fr/plugins/marketplace-reference) liste des sous-répertoires `skills` spécifiques, seuls ces sous-répertoires se chargent, et le répertoire par défaut `skills/` du plugin n'est pas analysé. Une clé `skills` dans le manifeste [s'ajoute à la valeur par défaut](#how-each-key-combines-with-its-default-location).

<h3 id="metadata-precedence">
  Précédence des métadonnées
</h3>

Certains champs de métadonnées ont une précédence fixe indépendamment de `strict` :

* **`defaultEnabled` et champs d'affichage** : le `defaultEnabled` de l'entrée et ses [champs d'affichage](/docs/fr/plugins/marketplace-reference#entry-and-plugin-json) comme `displayName` remplacent ceux du manifeste
* **`version`** : le `version` du manifeste remplace celui de l'entrée
* **`name`** : lorsque l'entrée liste le plugin sous un `name` différent de celui du manifeste, `enabledPlugins` utilise le nom de l'entrée, et les composants sont espacés de noms sous le nom du manifeste

Pour le tableau de précédence complet, voir [Mode strict](/docs/fr/plugins/marketplace-reference).

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Ajouter des composants à un plugin](/docs/fr/plugins/components) : ce que chaque composant fait à l'exécution, avec un exemple qui valide
* [Référence de marketplace](/docs/fr/plugins/marketplace-reference) : les champs d'entrée qu'un marketplace peut définir pour votre plugin
* [Référence des commandes de plugin](/docs/fr/plugins/cli-reference#plugin-validate) : les drapeaux et la sortie de `claude plugin validate`
* [Dépanner les plugins](/docs/fr/plugins/troubleshooting#claude-plugin-validate-reports-errors) : chaque message de validation avec sa correction
