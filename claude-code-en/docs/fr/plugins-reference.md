> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence des plugins

> Référence technique complète du système de plugins Claude Code, incluant les schémas, les commandes CLI et les spécifications des composants.

<Tip>
  Vous cherchez à installer des plugins ? Consultez [Découvrir et installer des plugins](/docs/fr/discover-plugins). Pour créer des plugins, consultez [Plugins](/docs/fr/plugins). Pour distribuer des plugins, consultez [Marketplaces de plugins](/docs/fr/plugin-marketplaces).
</Tip>

Cette référence fournit les spécifications techniques complètes du système de plugins Claude Code, incluant les schémas de composants, les commandes CLI et les outils de développement.

Un **plugin** est un répertoire autonome de composants qui étend Claude Code avec des fonctionnalités personnalisées. Les composants de plugin incluent les skills, les agents, les hooks, les serveurs MCP, les serveurs LSP et les moniteurs.

<h2 id="plugin-components-reference">
  Référence des composants de plugin
</h2>

<h3 id="skills">
  Skills
</h3>

Les plugins ajoutent des skills à Claude Code, créant des raccourcis `/name` que vous ou Claude pouvez invoquer.

**Emplacement** : répertoire `skills/` ou `commands/` à la racine du plugin, ou un seul fichier `SKILL.md` à la racine du plugin

**Format de fichier** : Les skills sont des répertoires avec `SKILL.md` ; les commandes sont des fichiers markdown simples

**Structure des skills** :

```text theme={null}
skills/
├── pdf-processor/
│   ├── SKILL.md
│   ├── reference.md (optionnel)
│   └── scripts/ (optionnel)
└── code-reviewer/
    └── SKILL.md
```

**Comportement d'intégration** :

* Les skills et les commandes sont découverts automatiquement lors de l'installation du plugin
* Claude peut les invoquer automatiquement en fonction du contexte de la tâche
* Les skills peuvent inclure des fichiers de support à côté de SKILL.md

Si un plugin n'a pas de répertoire `skills/` et pas de champ manifest `skills`, un `SKILL.md` à la racine du plugin est chargé comme une seule skill. Définissez le champ frontmatter `name` pour contrôler le nom d'invocation de la skill. Sans cela, Claude Code revient au nom du répertoire d'installation, qui pour les plugins installés depuis la marketplace est une chaîne de version qui change à chaque mise à jour. Pour les plugins qui livrent plus d'une skill, utilisez la disposition du répertoire `skills/` montrée ci-dessus.

Pour plus de détails, consultez [Skills](/docs/fr/skills).

<h3 id="agents">
  Agents
</h3>

Les plugins peuvent fournir des subagents spécialisés pour des tâches spécifiques que Claude peut invoquer automatiquement si approprié.

**Emplacement** : répertoire `agents/` à la racine du plugin

**Format de fichier** : Fichiers markdown décrivant les capacités de l'agent

**Structure de l'agent** :

```markdown theme={null}
---
name: agent-name
description: Ce dans quoi cet agent se spécialise et quand Claude devrait l'invoquer
model: sonnet
effort: medium
maxTurns: 20
disallowedTools: Write, Edit
---

Invite système détaillée pour l'agent décrivant son rôle, son expertise et son comportement.
```

Les agents de plugin prennent en charge les champs frontmatter `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background` et `isolation`. La seule valeur `isolation` valide est `"worktree"`. Pour des raisons de sécurité, `hooks`, `mcpServers` et `permissionMode` ne sont pas pris en charge pour les agents fournis par les plugins.

**Points d'intégration** :

* Les agents apparaissent dans la [saisie semi-automatique @-mention](/docs/fr/sub-agents#invoke-subagents-explicitly) sous leur nom délimité, tel que `my-plugin:code-reviewer`, une fois que le plugin est activé
* Claude peut invoquer les agents automatiquement en fonction du contexte de la tâche
* Les agents peuvent être invoqués manuellement par les utilisateurs
* Les agents de plugin fonctionnent aux côtés des agents Claude intégrés

Pour plus de détails, consultez [Subagents](/docs/fr/sub-agents).

<h3 id="hooks">
  Hooks
</h3>

Les plugins peuvent fournir des gestionnaires d'événements qui répondent automatiquement aux événements de Claude Code.

**Emplacement** : `hooks/hooks.json` à la racine du plugin, ou en ligne dans plugin.json

**Format** : Configuration JSON avec des correspondances d'événements et des actions

**Configuration des hooks** :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format-code.sh"
          }
        ]
      }
    ]
  }
}
```

Les hooks de plugin répondent aux mêmes événements de cycle de vie que les [hooks définis par l'utilisateur](/docs/fr/hooks) :

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

**Types de hooks** :

* `command` : exécuter des commandes shell ou des scripts
* `http` : envoyer l'événement JSON en tant que requête POST à une URL
* `mcp_tool` : appeler un outil sur un [serveur MCP](/docs/fr/mcp) configuré
* `prompt` : évaluer une invite avec un LLM (utilise l'espace réservé `$ARGUMENTS` pour le contexte)
* `agent` : exécuter un vérificateur agentic avec des outils pour les tâches de vérification complexes

Les hooks qui ciblent le [serveur MCP groupé](#mcp-servers) du plugin lui-même doivent utiliser ses noms délimités. Les correspondances d'outils et les champs `if` prennent le nom d'outil délimité `mcp__plugin_<plugin-name>_<server-name>__<tool>`, et le champ `server` d'un hook `mcp_tool` prend `plugin:<plugin-name>:<server-name>`. Une correspondance écrite contre la clé du serveur nu ne se déclenche jamais. Consultez [Correspondre les outils MCP](/docs/fr/hooks#match-mcp-tools) et [Serveurs MCP fournis par les plugins](/docs/fr/mcp#plugin-provided-mcp-servers).

<h3 id="mcp-servers">
  Serveurs MCP
</h3>

Les plugins peuvent regrouper des serveurs Model Context Protocol (MCP) pour connecter Claude Code avec des outils et services externes.

**Emplacement** : `.mcp.json` à la racine du plugin, ou en ligne dans plugin.json

**Format** : Configuration standard du serveur MCP

**Configuration du serveur MCP** :

```json theme={null}
{
  "mcpServers": {
    "plugin-database": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_PATH": "${CLAUDE_PLUGIN_ROOT}/data"
      }
    },
    "plugin-api-client": {
      "command": "npx",
      "args": ["@company/mcp-server", "--plugin-mode"]
    }
  }
}
```

**Comportement d'intégration** :

* Les serveurs MCP de plugin démarrent automatiquement quand le plugin est activé
* Les serveurs apparaissent comme des outils MCP standard dans la boîte à outils de Claude
* Les capacités du serveur s'intègrent de manière transparente avec les outils existants de Claude
* Les serveurs de plugin peuvent être configurés indépendamment des serveurs MCP de l'utilisateur

<h3 id="lsp-servers">
  Serveurs LSP
</h3>

<Tip>
  Vous cherchez à utiliser des plugins LSP ? Installez-les depuis la marketplace officielle : recherchez « lsp » dans l'onglet Découvrir de `/plugin`. Cette section documente comment créer des plugins LSP pour les langages non couverts par la marketplace officielle.
</Tip>

Les plugins peuvent fournir des serveurs [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) pour donner à Claude une intelligence de code en temps réel lors du travail sur votre base de code.

L'intégration LSP fournit :

* **Diagnostics instantanés** : Claude voit les erreurs et les avertissements immédiatement après chaque modification
* **Navigation de code** : aller à la définition, trouver les références et les informations au survol
* **Sensibilisation au langage** : informations de type et documentation pour les symboles de code

**Emplacement** : `.lsp.json` à la racine du plugin, ou en ligne dans `plugin.json`

**Format** : Configuration JSON mappant les noms des serveurs de langage à leurs configurations

**Format du fichier `.lsp.json`** :

```json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

**En ligne dans `plugin.json`** :

```json theme={null}
{
  "name": "my-plugin",
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": {
        ".go": "go"
      }
    }
  }
}
```

**Champs obligatoires :**

| Champ                 | Description                                                 |
| :-------------------- | :---------------------------------------------------------- |
| `command`             | Le binaire LSP à exécuter (doit être dans PATH)             |
| `extensionToLanguage` | Mappe les extensions de fichier aux identifiants de langage |

**Champs optionnels :**

| Champ                   | Description                                                                                                                                                                                                                          |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `args`                  | Arguments de ligne de commande pour le serveur LSP                                                                                                                                                                                   |
| `transport`             | Transport de communication : `stdio` (par défaut) ou `socket`                                                                                                                                                                        |
| `env`                   | Variables d'environnement à définir au démarrage du serveur                                                                                                                                                                          |
| `initializationOptions` | Options transmises au serveur lors de l'initialisation                                                                                                                                                                               |
| `settings`              | Paramètres transmis via `workspace/didChangeConfiguration`                                                                                                                                                                           |
| `workspaceFolder`       | Chemin du dossier de l'espace de travail pour le serveur                                                                                                                                                                             |
| `startupTimeout`        | Temps maximum d'attente du démarrage du serveur (millisecondes)                                                                                                                                                                      |
| `shutdownTimeout`       | Temps maximum d'attente de l'arrêt gracieux (millisecondes). Quand le délai d'attente s'écoule, Claude Code termine le processus du serveur. Quand non défini, aucun délai d'attente ne s'applique                                   |
| `restartOnCrash`        | Indique s'il faut redémarrer le serveur après un plantage. Par défaut `true`. Définissez sur `false` pour laisser un serveur planté arrêté au lieu de le redémarrer                                                                  |
| `maxRestarts`           | Nombre maximum de tentatives de redémarrage avant d'abandonner                                                                                                                                                                       |
| `diagnostics`           | Indique s'il faut pousser les diagnostics dans le contexte de Claude après les modifications (par défaut `true`). Définissez sur `false` pour conserver la navigation de code mais supprimer l'injection automatique de diagnostics. |

`restartOnCrash` et `shutdownTimeout` nécessitent Claude Code v2.1.205 ou ultérieur. Avant v2.1.205, le schéma de configuration acceptait les deux options mais définir l'une ou l'autre causait à Claude Code de sauter ce serveur LSP entièrement au démarrage, avec la raison visible uniquement dans la sortie `claude --debug`.

**Plusieurs serveurs pour la même extension** : quand plus d'un serveur LSP activé déclare la même extension de fichier dans `extensionToLanguage`, que les serveurs proviennent d'un plugin ou de différents plugins, le premier serveur enregistré gère les fichiers avec cette extension et les autres ne démarrent jamais. L'interface `/plugin` affiche un avertissement nommant le plugin dont le serveur est actif.

**Serveurs qui échouent à initialiser** : Claude Code saute un serveur dont la configuration est invalide, par exemple un serveur manquant `command` ou `extensionToLanguage`, et les autres serveurs configurés démarrent toujours. Exécutez `claude --debug` pour voir pourquoi un serveur a été sauté.

Un serveur sauté ne réclame pas ses extensions de fichier, donc un autre serveur valide qui déclare la même extension, du même plugin ou d'un plugin différent, gère toujours ces fichiers. Avant v2.1.205, un serveur qui échouait à initialiser réclamait toujours ses extensions et bloquait un autre serveur valide pour la même extension.

<Warning>
  **Vous devez installer le binaire du serveur de langage séparément.** Les plugins LSP configurent comment Claude Code se connecte à un serveur de langage, mais ils n'incluent pas le serveur lui-même. Si vous voyez `Executable not found in $PATH` dans l'onglet Erreurs de `/plugin`, installez le binaire requis pour votre langage.
</Warning>

**Plugins LSP disponibles :**

| Plugin              | Serveur de langage         | Commande d'installation                                                                          |
| :------------------ | :------------------------- | :----------------------------------------------------------------------------------------------- |
| `pyright-lsp`       | Pyright (Python)           | `pip install pyright` ou `npm install -g pyright`                                                |
| `typescript-lsp`    | TypeScript Language Server | `npm install -g typescript-language-server typescript`                                           |
| `rust-analyzer-lsp` | rust-analyzer              | [Voir l'installation de rust-analyzer](https://rust-analyzer.github.io/manual.html#installation) |

Installez d'abord le serveur de langage, puis installez le plugin depuis la marketplace.

<h3 id="monitors">
  Moniteurs
</h3>

Les plugins peuvent déclarer des moniteurs en arrière-plan que Claude Code démarre automatiquement quand le plugin est actif. Chaque moniteur exécute une commande shell pour la durée de la session et livre chaque ligne stdout à Claude en tant que notification, afin que Claude puisse réagir aux entrées de journal, aux changements de statut ou aux événements interrogés sans qu'on lui demande de démarrer la surveillance lui-même.

Les moniteurs de plugin utilisent le même mécanisme que l'[outil Monitor](/docs/fr/tools-reference#monitor-tool) et partagent ses contraintes de disponibilité. Ils s'exécutent uniquement dans les sessions CLI interactives, s'exécutent sans sandbox au même niveau de confiance que les [hooks](#hooks), et sont ignorés sur les hôtes où l'outil Monitor n'est pas disponible.

**Emplacement** : `monitors/monitors.json` à la racine du plugin, ou en ligne dans `plugin.json`

**Format** : Tableau JSON d'entrées de moniteur

Le `monitors/monitors.json` suivant surveille un point de terminaison de statut de déploiement et un journal d'erreurs local :

```json theme={null}
[
  {
    "name": "deploy-status",
    "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
    "description": "Changements de statut de déploiement"
  },
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Journal d'erreurs de l'application",
    "when": "on-skill-invoke:debug"
  }
]
```

Pour déclarer les moniteurs en ligne, définissez `experimental.monitors` dans `plugin.json` sur le même tableau. Pour charger à partir d'un chemin non par défaut, définissez `experimental.monitors` sur une chaîne de chemin relatif telle que `"./config/monitors.json"`. Les moniteurs sont un [composant expérimental](#experimental-components).

**Champs obligatoires :**

| Champ         | Description                                                                                                                           |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | Identifiant unique dans le plugin. Empêche les processus en double quand le plugin se recharge ou qu'une skill est invoquée à nouveau |
| `command`     | Commande shell exécutée en tant que processus en arrière-plan persistant dans le répertoire de travail de la session                  |
| `description` | Résumé court de ce qui est surveillé. Affiché dans le panneau des tâches et dans les résumés de notification                          |

**Champs optionnels :**

| Champ  | Description                                                                                                                                                                                                                                                 |
| :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `when` | Contrôle quand le moniteur démarre. `"always"` le démarre au démarrage de la session et au rechargement du plugin, et est la valeur par défaut. `"on-skill-invoke:<skill-name>"` le démarre la première fois que la skill nommée dans ce plugin est envoyée |

La valeur `command` prend en charge les [substitutions de chemin](#environment-variables) `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}` et `${CLAUDE_PROJECT_DIR}`, plus tout `${ENV_VAR}` de l'environnement. Préfixez la commande avec `cd "${CLAUDE_PLUGIN_ROOT}" && ` si le script doit s'exécuter à partir du répertoire du plugin lui-même.

Une commande `command` de moniteur ne peut pas référencer les valeurs [`${user_config.*}`](#user-configuration). La commande s'exécute via un shell, donc Claude Code rejette le moniteur avec une [erreur](/docs/fr/errors#plugin-command-references-user-config) au lieu de substituer la valeur. Les processus de moniteur ne reçoivent pas les variables d'environnement `CLAUDE_PLUGIN_OPTION_<KEY>`, donc faites en sorte que le script de moniteur lise la valeur à partir d'un fichier de configuration qu'il possède. Avant v2.1.207, les commandes de moniteur substituaient les valeurs `${user_config.*}`.

La désactivation d'un plugin en cours de session n'arrête pas les moniteurs qui sont déjà en cours d'exécution. Ils s'arrêtent quand la session se termine.

<h3 id="themes">
  Thèmes
</h3>

Les plugins peuvent livrer des thèmes de couleur qui apparaissent dans `/theme` aux côtés des présets intégrés et des thèmes locaux de l'utilisateur. Un thème est un fichier JSON dans `themes/` avec un préset `base` et une carte `overrides` clairsemée de jetons de couleur. Les thèmes sont un [composant expérimental](#experimental-components).

```json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

La sélection d'un thème de plugin persiste `custom:<plugin-name>:<slug>` dans la configuration de l'utilisateur. Les thèmes de plugin sont en lecture seule ; appuyer sur `Ctrl+E` sur l'un d'eux dans `/theme` le copie dans `~/.claude/themes/` afin que l'utilisateur puisse modifier la copie.

***

<h2 id="plugin-installation-scopes">
  Portées d'installation des plugins
</h2>

Quand vous installez un plugin, vous choisissez une **portée** qui détermine où le plugin est disponible et qui d'autre peut l'utiliser :

| Portée    | Fichier de paramètres                           | Cas d'usage                                                       |
| :-------- | :---------------------------------------------- | :---------------------------------------------------------------- |
| `user`    | `~/.claude/settings.json`                       | Plugins personnels disponibles dans tous les projets (par défaut) |
| `project` | `.claude/settings.json`                         | Plugins d'équipe partagés via le contrôle de version              |
| `local`   | `.claude/settings.local.json`                   | Plugins spécifiques au projet, ignorés par git                    |
| `managed` | [Paramètres gérés](/docs/fr/settings#settings-files) | Plugins gérés (lecture seule, mise à jour uniquement)             |

Les plugins utilisent le même système de portée que les autres configurations de Claude Code. Pour les instructions d'installation et les drapeaux de portée, consultez [Installer des plugins](/docs/fr/discover-plugins#install-plugins). Pour une explication complète des portées, consultez [Portées de configuration](/docs/fr/settings#configuration-scopes).

***

<h2 id="skills-directory-plugins">
  Plugins du répertoire des skills
</h2>

Tout dossier sous un répertoire de skills qui contient un manifeste `.claude-plugin/plugin.json` est chargé en tant que plugin nommé `<name>@skills-dir` à la session suivante, sans marketplace et sans étape d'installation. Générez un avec [`plugin init`](#plugin-init). Contrairement à une installation marketplace, le plugin est découvert sur place plutôt que copié dans le cache des plugins.

Un arborescence de répertoire de skills prend en charge trois choses distinctes :

| Ce que vous avez                              | Ce que c'est                                                                             |
| :-------------------------------------------- | :--------------------------------------------------------------------------------------- |
| `<skills-dir>/foo/SKILL.md` sans manifeste    | Une [skill](/docs/fr/skills) simple nommée `foo`                                              |
| `<skills-dir>/foo/.claude-plugin/plugin.json` | Un plugin `foo@skills-dir`, qui peut regrouper ses propres skills, agents, hooks et plus |
| `<plugin>/skills/bar/SKILL.md`                | Une skill `bar` emballée à l'intérieur d'un plugin                                       |

<h3 id="choose-where-the-plugin-loads-from">
  Choisir d'où le plugin se charge
</h3>

| Répertoire de skills    | Portée    | Se charge                                                                                                                   |
| :---------------------- | :-------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `~/.claude/skills/`     | personnel | Dans chaque projet, puisque l'emplacement est le vôtre seul                                                                 |
| `<cwd>/.claude/skills/` | projet    | Seulement après que vous acceptiez la boîte de dialogue de [confiance](/docs/fr/settings) de l'espace de travail pour ce dossier |

Un plugin de portée projet est archivé dans le référentiel et atteint chaque collaborateur qui le clone. Parce que ce contenu provient du référentiel plutôt que de vous, il se charge seulement après la même porte de confiance qui régit `.claude/settings.json`, et les composants qui exécutent du code sont davantage restreints :

* Les serveurs MCP qu'il déclare passent par l'[approbation par serveur](/docs/fr/mcp) identique qu'un `.mcp.json` de projet
* Les serveurs LSP démarrent seulement après que vous fassiez confiance à l'espace de travail
* Les [moniteurs en arrière-plan](#monitors) ne se chargent pas

Les plugins de portée personnelle n'ont aucune de ces restrictions.

<Warning>
  Les plugins `@skills-dir` de portée projet se chargent uniquement à partir du `.claude/skills/` du répertoire où vous démarrez Claude Code. Ils ne [remontent pas jusqu'à la racine du référentiel](/docs/fr/skills#automatic-discovery-from-parent-and-nested-directories) de la façon dont les skills et commandes simples le font, donc lancer à partir d'un sous-répertoire manque un plugin qui vit à la racine du référentiel. Lancez à partir de la racine du référentiel, ou exécutez `/reload-plugins` après avoir changé de répertoires.
</Warning>

<h3 id="edit-reload-and-disable-a-skills-directory-plugin">
  Modifier, recharger et désactiver un plugin du répertoire des skills
</h3>

Les modifications que vous apportez au `SKILL.md` d'une skill prennent effet immédiatement dans la session actuelle. Les modifications aux autres composants du plugin, tels que `hooks/`, `.mcp.json`, `agents/` et `output-styles/`, ne le font pas. Exécutez `/reload-plugins` ou redémarrez Claude Code pour les récupérer. Consultez [Détection des changements en direct](/docs/fr/skills#live-change-detection).

Pour arrêter le chargement d'un plugin du répertoire des skills, supprimez son dossier ou désactivez-le par nom. Il n'y a pas d'étape `uninstall` car rien n'a été installé à partir d'une marketplace.

```bash theme={null}
claude plugin disable my-tool@skills-dir
```

***

<h2 id="plugin-manifest-schema">
  Schéma du manifeste du plugin
</h2>

Le fichier `.claude-plugin/plugin.json` définit les métadonnées et la configuration de votre plugin. Cette section documente tous les champs et options pris en charge.

Le manifeste est optionnel. S'il est omis, Claude Code découvre automatiquement les composants dans les [emplacements par défaut](#file-locations-reference) et dérive le nom du plugin du nom du répertoire. Utilisez un manifeste quand vous devez fournir des métadonnées ou des chemins de composants personnalisés.

<h3 id="complete-schema">
  Schéma complet
</h3>

```json theme={null}
{
  "name": "plugin-name",
  "displayName": "Plugin Name",
  "version": "1.2.0",
  "description": "Brief plugin description",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://github.com/author"
  },
  "homepage": "https://docs.example.com/plugin",
  "repository": "https://github.com/author/plugin",
  "license": "MIT",
  "keywords": ["keyword1", "keyword2"],
  "skills": "./custom/skills/",
  "commands": ["./custom/commands/special.md"],
  "agents": ["./custom/agents/reviewer.md"],
  "hooks": "./config/hooks.json",
  "mcpServers": "./mcp-config.json",
  "outputStyles": "./styles/",
  "lspServers": "./.lsp.json",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./monitors.json"
  },
  "dependencies": [
    "helper-lib",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

<h3 id="required-fields">
  Champs obligatoires
</h3>

Si vous incluez un manifeste, `name` est le seul champ obligatoire.

| Champ  | Type   | Description                                                                                                                                                                                                                                                    | Exemple              |
| :----- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `name` | string | Identifiant unique (kebab-case, pas d'espaces). Quand une [entrée de marketplace](/docs/fr/plugin-marketplaces#plugin-entries) liste le plugin sous un nom différent, le nom de l'entrée de marketplace est ce que les clés `enabledPlugins` et `/plugin` utilisent | `"deployment-tools"` |

Ce nom est utilisé pour l'espace de noms des composants. Par exemple, dans l'interface utilisateur, l'agent `agent-creator` pour le plugin avec le nom `plugin-dev` apparaîtra comme `plugin-dev:agent-creator`.

<h3 id="unrecognized-fields">
  Champs non reconnus
</h3>

Claude Code ignore les champs de niveau supérieur qu'il ne reconnaît pas. Vous pouvez conserver les métadonnées d'un autre écosystème dans `plugin.json` et le plugin se charge toujours. Cela rend pratique de maintenir un manifeste unique qui sert également de manifeste d'extension VS Code ou Cursor, un `package.json` npm, ou un manifeste de bundle MCPB/DXT.

`claude plugin validate` signale les champs non reconnus comme des avertissements, pas des erreurs. Si un champ est décalé d'un ou deux caractères par rapport à un champ reconnu, l'avertissement suggère le nom probablement prévu. Un plugin avec seulement des avertissements de champs non reconnus réussit toujours la validation et se charge à l'exécution.

Les champs avec le mauvais type échouent toujours. Par exemple, une valeur `keywords` qui est une chaîne au lieu d'un tableau est une erreur de chargement, et `claude plugin validate` la signale comme telle.

Passez `--strict` pour traiter les avertissements comme des erreurs. Utilisez-le dans CI pour détecter un nom de champ mal orthographié ou un champ laissé par l'outil d'un autre avant la publication, même si le plugin se chargerait à l'exécution.

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h3 id="metadata-fields">
  Champs de métadonnées
</h3>

| Champ            | Type    | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Exemple                                                           |
| :--------------- | :------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- |
| `$schema`        | string  | URL du schéma JSON pour l'autocomplétion et la validation de l'éditeur. Claude Code ignore ce champ au moment du chargement.                                                                                                                                                                                                                                                                                                                                       | `"https://json.schemastore.org/claude-code-plugin-manifest.json"` |
| `displayName`    | string  | Nom lisible affiché dans le sélecteur `/plugin` et autres surfaces de l'interface utilisateur. Revient à `name` quand omis. Contrairement à `name`, peut contenir des espaces et n'importe quelle casse. Non utilisé pour l'espace de noms ou la recherche. Nécessite Claude Code v2.1.143 ou ultérieur.                                                                                                                                                           | `"Deployment Tools"`                                              |
| `version`        | string  | Optionnel. Version sémantique. La définir épingle le plugin à cette chaîne de version, de sorte que les utilisateurs ne reçoivent des mises à jour que lorsque vous la modifiez. Si elle est omise, Claude Code revient au SHA du commit git, de sorte que chaque commit est traité comme une nouvelle version. Si elle est également définie dans l'entrée de la marketplace, `plugin.json` a la priorité. Consultez [Gestion des versions](#version-management). | `"2.1.0"`                                                         |
| `description`    | string  | Explication brève de l'objectif du plugin                                                                                                                                                                                                                                                                                                                                                                                                                          | `"Deployment automation tools"`                                   |
| `author`         | object  | Informations sur l'auteur                                                                                                                                                                                                                                                                                                                                                                                                                                          | `{"name": "Dev Team", "email": "dev@company.com"}`                |
| `homepage`       | string  | URL de documentation                                                                                                                                                                                                                                                                                                                                                                                                                                               | `"https://docs.example.com"`                                      |
| `repository`     | string  | URL du code source                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `"https://github.com/user/plugin"`                                |
| `license`        | string  | Identifiant de licence                                                                                                                                                                                                                                                                                                                                                                                                                                             | `"MIT"`, `"Apache-2.0"`                                           |
| `keywords`       | array   | Balises de découverte                                                                                                                                                                                                                                                                                                                                                                                                                                              | `["deployment", "ci-cd"]`                                         |
| `defaultEnabled` | boolean | Si le plugin démarre dans un état activé quand l'utilisateur n'en a pas défini un. Par défaut à `true`. Consultez [Activation par défaut](#default-enablement). Nécessite Claude Code v2.1.154 ou ultérieur.                                                                                                                                                                                                                                                       | `false`                                                           |

<h3 id="default-enablement">
  Activation par défaut
</h3>

Définissez `defaultEnabled: false` dans `plugin.json` pour livrer un plugin qui s'installe désactivé. L'utilisateur l'active avec `claude plugin enable <plugin>` ou l'interface `/plugin`. Utilisez ceci pour les plugins qui ajoutent un coût ou une portée qu'un utilisateur devrait accepter, comme celui qui se connecte à un service externe. Cela nécessite Claude Code v2.1.154 ou ultérieur. Les versions antérieures ignorent le champ et activent le plugin à l'installation.

`defaultEnabled` est le secours quand rien d'autre n'a décidé l'état du plugin. Deux choses ont la priorité sur lui :

* **Le paramètre de l'utilisateur** : une entrée pour le plugin dans `enabledPlugins` à n'importe quelle portée de paramètres. Une fois écrite, elle persiste entre les mises à jour et réinstallations du plugin, donc changer `defaultEnabled` dans une version ultérieure ne bascule pas un utilisateur existant.
* **Une exigence de dépendance** : quand un plugin est requis par un autre qui est actif, Claude Code écrit `true` pour lui au moment de l'installation ou de l'activation. Cela lui donne un paramètre explicite, donc sa propre valeur par défaut ne s'applique plus. Consultez [Activer ou désactiver un plugin avec des dépendances](/docs/fr/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies).

Le même champ peut apparaître dans l'entrée marketplace d'un plugin, où il a la priorité sur la valeur dans `plugin.json`. Consultez [Champs de plugin optionnels](/docs/fr/plugin-marketplaces#optional-plugin-fields).

<h3 id="component-path-fields">
  Champs de chemin de composant
</h3>

| Champ                   | Type                  | Description                                                                                                                                                                                                                      | Exemple                                              |
| :---------------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `skills`                | string\|array         | Répertoires de skills personnalisés contenant `<name>/SKILL.md`. S'ajoute à l'analyse par défaut `skills/`. Consultez [Règles de comportement des chemins](#path-behavior-rules) pour l'exception de la racine de la marketplace | `"./custom/skills/"`                                 |
| `commands`              | string\|array         | Fichiers de skill markdown plats personnalisés ou répertoires (remplace le répertoire par défaut `commands/`)                                                                                                                    | `"./custom/cmd.md"` ou `["./cmd1.md"]`               |
| `agents`                | string\|array         | Fichiers d'agents personnalisés (remplace le répertoire par défaut `agents/`)                                                                                                                                                    | `"./custom/agents/reviewer.md"`                      |
| `hooks`                 | string\|array\|object | Chemins de configuration des hooks ou configuration en ligne                                                                                                                                                                     | `"./my-extra-hooks.json"`                            |
| `mcpServers`            | string\|array\|object | Chemins de configuration MCP ou configuration en ligne                                                                                                                                                                           | `"./my-extra-mcp-config.json"`                       |
| `outputStyles`          | string\|array         | Fichiers/répertoires de styles de sortie personnalisés (remplace le répertoire par défaut `output-styles/`)                                                                                                                      | `"./styles/"`                                        |
| `lspServers`            | string\|array\|object | Configurations [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) pour l'intelligence de code (aller à la définition, trouver les références, etc.)                                               | `"./.lsp.json"`                                      |
| `experimental.themes`   | string\|array         | Fichiers/répertoires de thèmes de couleur (remplace le répertoire par défaut `themes/`). Consultez [Thèmes](#themes)                                                                                                             | `"./themes/"`                                        |
| `experimental.monitors` | string\|array         | Configurations de [Monitor](/docs/fr/tools-reference#monitor-tool) en arrière-plan qui démarrent automatiquement quand le plugin est actif. Consultez [Moniteurs](#monitors)                                                          | `"./monitors.json"`                                  |
| `userConfig`            | object                | Valeurs configurables par l'utilisateur demandées au moment de l'activation. Consultez [Configuration utilisateur](#user-configuration)                                                                                          | Voir ci-dessous                                      |
| `channels`              | array                 | Déclarations de canaux pour l'injection de messages (style Telegram, Slack, Discord). Consultez [Canaux](#channels)                                                                                                              | Voir ci-dessous                                      |
| `dependencies`          | array                 | Autres plugins que ce plugin nécessite, optionnellement avec des contraintes de version semver. Consultez [Contraindre les versions de dépendance des plugins](/docs/fr/plugin-dependencies)                                          | `[{ "name": "secrets-vault", "version": "~2.1.0" }]` |

<h3 id="experimental-components">
  Composants expérimentaux
</h3>

Les composants sous la clé `experimental`, `themes` et `monitors`, ont un schéma de manifeste qui peut changer entre les versions pendant qu'ils se stabilisent. L'endroit où vous les déclarez est une migration distincte : le niveau supérieur fonctionne toujours, `claude plugin validate` avertit, et une version future exigera `experimental.*`.

<h3 id="user-configuration">
  Configuration utilisateur
</h3>

Le champ `userConfig` déclare les valeurs que Claude Code demande à l'utilisateur lors de l'activation du plugin. Utilisez ceci au lieu d'exiger que les utilisateurs modifient manuellement `settings.json`.

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "Point de terminaison API",
      "description": "Le point de terminaison API de votre équipe"
    },
    "api_token": {
      "type": "string",
      "title": "Jeton API",
      "description": "Jeton d'authentification API",
      "sensitive": true
    }
  }
}
```

Les clés doivent être des identifiants valides. Chaque option prend en charge ces champs :

| Champ         | Obligatoire | Description                                                                                         |
| :------------ | :---------- | :-------------------------------------------------------------------------------------------------- |
| `type`        | Oui         | L'un de `string`, `number`, `boolean`, `directory`, ou `file`                                       |
| `title`       | Oui         | Étiquette affichée dans la boîte de dialogue de configuration                                       |
| `description` | Oui         | Texte d'aide affiché sous le champ                                                                  |
| `sensitive`   | Non         | Si `true`, masque l'entrée et stocke la valeur dans le stockage sécurisé au lieu de `settings.json` |
| `required`    | Non         | Si `true`, la validation échoue quand le champ est vide                                             |
| `default`     | Non         | Valeur utilisée quand l'utilisateur ne fournit rien                                                 |
| `multiple`    | Non         | Pour le type `string`, autoriser un tableau de chaînes                                              |
| `min` / `max` | Non         | Limites pour le type `number`                                                                       |

Chaque valeur est disponible pour la substitution en tant que `${user_config.KEY}` dans les configurations de serveurs MCP et LSP et les commandes de hook. Les valeurs non sensibles peuvent également être substituées dans le contenu des skills et des agents. Toutes les valeurs sont exportées vers les processus de hook en tant que variables d'environnement `CLAUDE_PLUGIN_OPTION_<KEY>`, où `<KEY>` est la clé d'option en majuscules.

Les champs qui s'exécutent dans un shell rejettent `${user_config.*}` : substituer une valeur configurée dans une commande shell laisserait le shell exécuter tout ce que cette valeur contient, donc le composant échoue avec une [erreur](/docs/fr/errors#plugin-command-references-user-config) à la place. Chaque champ rejeté a une façon alternative de passer la valeur :

| Champ rejeté                                                                 | Comment passer la valeur                                                                                                                       |
| :--------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Commandes de hook de forme shell                                             | Utilisez la [forme exec](/docs/fr/hooks#exec-form-and-shell-form) avec `args`, ou lisez `CLAUDE_PLUGIN_OPTION_<KEY>` depuis l'environnement du hook |
| Commandes de [Monitor](#monitors)                                            | Lisez la valeur depuis un fichier de configuration dans le script                                                                              |
| MCP [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication) | Lisez la valeur depuis un fichier de configuration dans le script                                                                              |

Avant v2.1.207, ces champs substituaient les valeurs `${user_config.KEY}` ; mettez à jour les plugins qui s'appuyaient sur ceci.

Les valeurs non sensibles sont stockées sous la clé [`pluginConfigs`](/docs/fr/settings#pluginconfigs) dans `settings.json` en tant que `pluginConfigs[<plugin-id>].options`. Claude Code écrit la clé dans les paramètres utilisateur et la relit depuis les paramètres utilisateur, l'indicateur `--settings`, et les paramètres gérés uniquement ; les entrées dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet sont ignorées. Avant v2.1.207, Claude Code lisait également les paramètres du projet et locaux.

Les valeurs sensibles vont au trousseau macOS, ou à `~/.claude/.credentials.json` sur les plates-formes où aucun trousseau pris en charge n'est disponible. Le stockage du trousseau est partagé avec les jetons OAuth et a une limite totale d'environ 2 KB, donc gardez les valeurs sensibles petites.

<h3 id="channels">
  Canaux
</h3>

Le champ `channels` permet à un plugin de déclarer un ou plusieurs canaux de messages qui injectent du contenu dans la conversation. Chaque canal se lie à un serveur MCP que le plugin fournit.

```json theme={null}
{
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Jeton de bot",
          "description": "Jeton de bot Telegram",
          "sensitive": true
        },
        "owner_id": {
          "type": "string",
          "title": "ID du propriétaire",
          "description": "Votre ID utilisateur Telegram"
        }
      }
    }
  ]
}
```

Le champ `server` est obligatoire et doit correspondre à une clé dans les `mcpServers` du plugin. Le `userConfig` optionnel par canal utilise le même schéma que le champ de niveau supérieur, permettant au plugin de demander des jetons de bot ou des ID de propriétaire lors de l'activation du plugin.

<h3 id="path-behavior-rules">
  Règles de comportement des chemins
</h3>

Qu'un chemin personnalisé remplace ou étende le répertoire par défaut du plugin dépend du champ :

* **Remplace le répertoire par défaut** : `commands`, `agents`, `outputStyles`, `experimental.themes`, `experimental.monitors`. Par exemple, quand le manifeste spécifie `commands`, le répertoire par défaut `commands/` n'est pas analysé. Pour conserver le répertoire par défaut et en ajouter d'autres, listez-le explicitement : `"commands": ["./commands/", "./extras/"]`
* **S'ajoute au répertoire par défaut** : `skills`. Le répertoire par défaut `skills/` est toujours analysé, et les répertoires listés dans `skills` sont chargés à côté de lui. Exception : pour une [entrée de marketplace dont la `source` se résout à la racine de la marketplace](/docs/fr/plugin-marketplaces#advanced-plugin-entries), déclarer des sous-répertoires spécifiques remplace l'analyse par défaut `skills/`
* **Règles de fusion propres** : [hooks](#hooks), [Serveurs MCP](#mcp-servers), et [Serveurs LSP](#lsp-servers). Consultez chaque section pour savoir comment plusieurs sources se combinent

Quand un plugin a à la fois un dossier par défaut et la clé de manifeste correspondante, Claude Code v2.1.140 et versions ultérieures signale le dossier ignoré dans `claude plugin list` et la vue de détail `/plugin`. Le plugin se charge toujours en utilisant les chemins du manifeste. Aucun avertissement n'est affiché quand la clé de manifeste pointe dans le dossier par défaut, par exemple `"commands": ["./commands/deploy.md"]`, car le dossier est adressé explicitement dans ce cas.

Pour tous les champs de chemin :

* Tous les chemins doivent être relatifs à la racine du plugin et commencer par `./`
* Les composants des chemins personnalisés utilisent les mêmes règles de nommage et d'espace de noms
* Plusieurs chemins peuvent être spécifiés sous forme de tableaux
* Quand un chemin de skill pointe vers un répertoire qui contient directement un `SKILL.md`, par exemple `"skills": ["./"]` pointant vers la racine du plugin, le champ frontmatter `name` dans `SKILL.md` détermine le nom d'invocation de la skill. Cela donne un nom stable indépendamment du répertoire d'installation. Si `name` n'est pas défini dans le frontmatter, le nom de base du répertoire est utilisé comme secours.

Un plugin qui a un `SKILL.md` à sa racine, aucun sous-répertoire `skills/`, et aucun champ de manifeste `skills` est automatiquement chargé en tant que plugin à une seule skill dans Claude Code v2.1.142 et versions ultérieures. Vous n'avez pas besoin de définir `"skills": ["./"]` dans `plugin.json` pour cette disposition. Le nom d'invocation de la skill suit la même règle que ci-dessus : le champ frontmatter `name`, ou le nom de base du répertoire comme secours.

**Exemples de chemins** :

```json theme={null}
{
  "commands": [
    "./specialized/deploy.md",
    "./utilities/batch-process.md"
  ],
  "agents": [
    "./custom-agents/reviewer.md",
    "./custom-agents/tester.md"
  ]
}
```

<h3 id="environment-variables">
  Variables d'environnement
</h3>

Claude Code fournit trois variables pour référencer les chemins :

| Variable                | Se résout en                                                                                                            | Utilisez-la pour                                                                                          |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Chemin absolu du répertoire d'installation du plugin                                                                    | Scripts, binaires et fichiers de configuration fournis avec le plugin                                     |
| `${CLAUDE_PLUGIN_DATA}` | [Répertoire persistant](#persistent-data-directory) qui survit aux mises à jour du plugin, créé à la première référence | Dépendances installées telles que `node_modules` ou environnements virtuels Python, code généré et caches |
| `${CLAUDE_PROJECT_DIR}` | La racine du projet                                                                                                     | Scripts et fichiers de configuration locaux au projet                                                     |

Les trois sont exportés en tant que variables d'environnement vers les processus de hook et vers les sous-processus des serveurs MCP et LSP. Les champs où les espaces réservés se résolvent en ligne dépendent du composant du plugin :

| Composant du plugin              | Champs où les espaces réservés se résolvent |
| :------------------------------- | :------------------------------------------ |
| Contenu des skills et des agents | N'importe où l'espace réservé apparaît      |
| Commandes de hook et de moniteur | N'importe où l'espace réservé apparaît      |
| Serveurs MCP `stdio`             | `command`, `args`, `env`                    |
| Serveurs MCP `http`, `sse`, `ws` | `url`, `headers`, `headersHelper`           |
| Serveurs LSP                     | `command`, `args`, `env`, `workspaceFolder` |

Dans les commandes de hook, utilisez la [forme exec](/docs/fr/hooks#exec-form-and-shell-form) avec `args` pour que chaque chemin soit passé comme un seul argument sans guillemets. Dans les hooks de forme shell et les commandes de moniteur, enveloppez les variables entre guillemets doubles, comme dans `"${CLAUDE_PROJECT_DIR}/scripts/server.sh"`. Ce hook de forme shell exécute un script fourni avec un plugin :

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

`${CLAUDE_PLUGIN_ROOT}` change quand le plugin se met à jour. Le répertoire de la version précédente reste sur le disque pendant environ sept jours après une mise à jour avant le nettoyage, mais traitez-le comme éphémère et n'écrivez pas d'état ici.

Quand un plugin se met à jour en cours de session, les commandes de hook, les moniteurs, les serveurs MCP et les serveurs LSP continuent d'utiliser le chemin de la version précédente. Exécutez `/reload-plugins` pour basculer les hooks, les serveurs MCP et les serveurs LSP vers le nouveau chemin ; les moniteurs nécessitent un redémarrage de session.

Les serveurs MCP peuvent également appeler la requête `roots/list` pour lire les répertoires de travail de la session à l'exécution. Consultez [ce que `roots/list` retourne et quand Claude Code notifie le serveur des modifications](/docs/fr/mcp#option-3-add-a-local-stdio-server).

<h4 id="persistent-data-directory">
  Répertoire de données persistantes
</h4>

Le répertoire `${CLAUDE_PLUGIN_DATA}` se résout en `~/.claude/plugins/data/{id}/`, où `{id}` est l'identifiant du plugin avec les caractères en dehors de `a-z`, `A-Z`, `0-9`, `_` et `-` remplacés par `-`. Pour un plugin installé en tant que `formatter@my-marketplace`, le répertoire est `~/.claude/plugins/data/formatter-my-marketplace/`.

Un usage courant est d'installer les dépendances de langage une fois et de les réutiliser entre les sessions et les mises à jour du plugin. Parce que le répertoire de données survit à n'importe quelle version unique du plugin, une vérification de l'existence du répertoire seul ne peut pas détecter quand une mise à jour change le manifeste de dépendance du plugin. Le motif recommandé compare le manifeste fourni par rapport à une copie dans le répertoire de données et réinstalle quand ils diffèrent.

Ce hook `SessionStart` installe `node_modules` à la première exécution et à nouveau chaque fois qu'une mise à jour du plugin inclut un `package.json` modifié :

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

Le `diff` sort avec un code non nul quand la copie stockée est manquante ou diffère de celle fournie, couvrant à la fois la première exécution et les mises à jour changeant les dépendances. Si `npm install` échoue, le `rm` final supprime le manifeste copié pour que la session suivante réessaie.

Les scripts fournis dans `${CLAUDE_PLUGIN_ROOT}` peuvent ensuite s'exécuter contre les `node_modules` persistants :

```json theme={null}
{
  "mcpServers": {
    "routines": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": {
        "NODE_PATH": "${CLAUDE_PLUGIN_DATA}/node_modules"
      }
    }
  }
}
```

Le répertoire de données est supprimé automatiquement quand vous désinstallez le plugin de la dernière portée où il est installé. L'interface `/plugin` affiche la taille du répertoire et demande une confirmation avant la suppression. La CLI supprime par défaut ; passez [`--keep-data`](#plugin-uninstall) pour le conserver.

***

<h2 id="plugin-caching-and-file-resolution">
  Mise en cache des plugins et résolution des fichiers
</h2>

Les plugins sont spécifiés de deux façons :

* Via `claude --plugin-dir` ou `claude --plugin-url`, pour la durée d'une session.
* Via une marketplace, installés pour les sessions futures.

À des fins de sécurité et de vérification, Claude Code copie les plugins de *marketplace* dans le **cache de plugins** local de l'utilisateur (`~/.claude/plugins/cache`) plutôt que de les utiliser sur place. Comprendre ce comportement est important lors du développement de plugins qui référencent des fichiers externes.

Chaque version installée est un répertoire séparé dans le cache. Quand vous mettez à jour ou désinstallez un plugin, le répertoire de version précédente est marqué comme orphelin et supprimé automatiquement 7 jours plus tard. La période de grâce permet aux sessions Claude Code concurrentes qui ont déjà chargé l'ancienne version de continuer à fonctionner sans erreurs.

Les outils Glob et Grep de Claude ignorent les répertoires de version orphelins lors des recherches, donc les résultats de fichiers n'incluent pas le code de plugin obsolète.

<h3 id="path-traversal-limitations">
  Limitations de traversée de répertoires
</h3>

Les plugins installés ne peuvent pas référencer des fichiers en dehors de leur répertoire. Les chemins qui traversent en dehors de la racine du plugin (comme `../shared-utils`) ne fonctionneront pas après l'installation car ces fichiers externes ne sont pas copiés dans le cache.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Partager des fichiers au sein d'une marketplace avec des liens symboliques
</h3>

Si votre plugin doit partager des fichiers avec d'autres parties de la même marketplace, vous pouvez créer des liens symboliques à l'intérieur de votre répertoire de plugin. La façon dont un lien symbolique est traité quand le plugin est copié dans le cache dépend de l'endroit où sa cible se résout :

* **Au sein du répertoire propre du plugin :** le lien symbolique est préservé en tant que lien symbolique relatif dans le cache, donc il continue de se résoudre à la cible copiée au moment de l'exécution.
* **Ailleurs au sein de la même marketplace :** le lien symbolique est déréférencé. Le contenu de la cible est copié dans le cache à sa place. Cela permet au répertoire `skills/` d'un meta-plugin de créer un lien vers les skills définis par d'autres plugins de la marketplace.
* **En dehors de la marketplace :** le lien symbolique est ignoré pour des raisons de sécurité. Cela empêche les plugins de récupérer des fichiers hôtes arbitraires tels que les chemins système dans le cache.

Pour les plugins installés avec `--plugin-dir` ou à partir d'un chemin local, seuls les liens symboliques qui se résolvent au sein du répertoire propre du plugin sont préservés. Tous les autres sont ignorés.

La commande suivante crée un lien à partir de l'intérieur d'un plugin de marketplace vers une skill partagée définie par un plugin frère. Sur Windows, utilisez `mklink /D` à partir d'une invite de commandes élevée ou activez le Mode développeur :

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

Cela offre de la flexibilité tout en maintenant les avantages de sécurité du système de mise en cache.

***

<h2 id="plugin-directory-structure">
  Structure du répertoire des plugins
</h2>

<h3 id="standard-plugin-layout">
  Disposition standard des plugins
</h3>

Un plugin complet suit cette structure :

```text theme={null}
enterprise-plugin/
├── .claude-plugin/           # Répertoire de métadonnées (optionnel)
│   └── plugin.json             # manifeste du plugin
├── skills/                   # Skills
│   ├── code-reviewer/
│   │   └── SKILL.md
│   └── pdf-processor/
│       ├── SKILL.md
│       └── scripts/
├── commands/                 # Skills en tant que fichiers markdown plats
│   ├── status.md
│   └── logs.md
├── agents/                   # Définitions de subagent
│   ├── security-reviewer.md
│   ├── performance-tester.md
│   └── compliance-checker.md
├── output-styles/            # Définitions de style de sortie
│   └── terse.md
├── themes/                   # Définitions de thème de couleur
│   └── dracula.json
├── monitors/                 # Configurations de moniteur en arrière-plan
│   └── monitors.json
├── hooks/                    # Configurations des hooks
│   ├── hooks.json           # Configuration principale des hooks
│   └── security-hooks.json  # Hooks supplémentaires
├── bin/                      # Exécutables de plugin ajoutés à PATH
│   └── my-tool               # Invocable en tant que commande nue dans l'outil Bash
├── settings.json            # Paramètres par défaut pour le plugin
├── .mcp.json                # Définitions du serveur MCP
├── .lsp.json                # Configurations du serveur LSP
├── scripts/                 # Scripts de hooks et d'utilitaires
│   ├── security-scan.sh
│   ├── format-code.py
│   └── deploy.js
├── LICENSE                  # Fichier de licence
└── CHANGELOG.md             # Historique des versions
```

<Warning>
  Le répertoire `.claude-plugin/` contient le fichier `plugin.json`. Tous les autres répertoires (commands/, agents/, skills/, output-styles/, themes/, monitors/, hooks/) doivent être à la racine du plugin, pas à l'intérieur de `.claude-plugin/`.
</Warning>

Un fichier `CLAUDE.md` à la racine du plugin n'est pas chargé comme contexte de projet. Les plugins contribuent au contexte par le biais de skills, d'agents et de hooks plutôt que par CLAUDE.md. Pour livrer des instructions qui se chargent dans le contexte de Claude, mettez-les dans un [skill](#skills).

<h3 id="file-locations-reference">
  Référence des emplacements de fichiers
</h3>

| Composant            | Emplacement par défaut       | Objectif                                                                                                                                                                                                    |
| :------------------- | :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manifeste**        | `.claude-plugin/plugin.json` | Métadonnées et configuration du plugin (optionnel)                                                                                                                                                          |
| **Skills**           | `skills/`                    | Skills avec structure `<name>/SKILL.md`                                                                                                                                                                     |
| **Commandes**        | `commands/`                  | Skills en tant que fichiers Markdown plats. Utilisez `skills/` pour les nouveaux plugins                                                                                                                    |
| **Agents**           | `agents/`                    | Fichiers Markdown de subagent                                                                                                                                                                               |
| **Styles de sortie** | `output-styles/`             | Définitions de style de sortie                                                                                                                                                                              |
| **Thèmes**           | `themes/`                    | Définitions de thème de couleur                                                                                                                                                                             |
| **Hooks**            | `hooks/hooks.json`           | Configuration des hooks                                                                                                                                                                                     |
| **Serveurs MCP**     | `.mcp.json`                  | Définitions du serveur MCP                                                                                                                                                                                  |
| **Serveurs LSP**     | `.lsp.json`                  | Configurations du serveur de langage                                                                                                                                                                        |
| **Moniteurs**        | `monitors/monitors.json`     | Configurations de moniteur en arrière-plan                                                                                                                                                                  |
| **Exécutables**      | `bin/`                       | Exécutables ajoutés au `PATH` de l'outil Bash. Les fichiers ici sont invocables en tant que commandes nues dans n'importe quel appel d'outil Bash tandis que le plugin est activé                           |
| **Paramètres**       | `settings.json`              | Configuration par défaut appliquée quand le plugin est activé. Seules les clés [`agent`](/docs/fr/sub-agents) et [`subagentStatusLine`](/docs/fr/statusline#subagent-status-lines) sont actuellement prises en charge |

***

<h2 id="cli-commands-reference">
  Référence des commandes CLI
</h2>

Claude Code fournit des commandes CLI pour la gestion des plugins non interactive, utile pour les scripts et l'automatisation.

<h3 id="plugin-init">
  plugin init
</h3>

Générez un nouveau plugin à `~/.claude/skills/<name>/`. À la session Claude Code suivante, il se charge automatiquement en tant que `<name>@skills-dir` et apparaît dans `/plugin` et `claude plugin list` sans étape d'installation.

Consultez [Plugins du répertoire des skills](#skills-directory-plugins) pour les exigences de portée et de confiance.

```bash theme={null}
claude plugin init <name> [options]
```

**Arguments :**

* `<name>` : Nom du plugin. Devient l'espace de noms de la skill et le nom du répertoire sous `~/.claude/skills/`, donc il ne peut pas contenir d'espaces ou de séparateurs de chemin.

**Options :**

| Option                   | Description                                                                                                                          | Par défaut              |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :---------------------- |
| `--description <text>`   | Description du manifeste                                                                                                             |                         |
| `--author <name>`        | Nom de l'auteur                                                                                                                      | `git config user.name`  |
| `--author-email <email>` | Email de l'auteur                                                                                                                    | `git config user.email` |
| `--with <components...>` | Générer également des dossiers de composants. Valeurs valides : `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style`, `channel` |                         |
| `-f, --force`            | Écraser un `.claude-plugin/` existant à la cible                                                                                     |                         |
| `-h, --help`             | Afficher l'aide pour la commande                                                                                                     |                         |

**Alias :** `new`

Chaque valeur `--with` ajoute un fichier de démarrage pour ce composant, prêt à être modifié :

| Composant      | Ce qu'il génère                                                                                              |
| :------------- | :----------------------------------------------------------------------------------------------------------- |
| `skills`       | Une skill `<name>:example` supplémentaire nommée à côté de celle par défaut                                  |
| `agents`       | Une définition de subagent `agents/`                                                                         |
| `hooks`        | Un `hooks/hooks.json` avec un gestionnaire d'événements exemple                                              |
| `mcp`          | Un `.mcp.json` avec des exemples de serveur HTTP et stdio                                                    |
| `lsp`          | Un exemple `.lsp.json` de serveur de langage                                                                 |
| `output-style` | Un `output-styles/<name>.md` qui s'applique automatiquement tandis que le plugin est activé                  |
| `channel`      | Un [canal](/docs/fr/channels) basé sur MCP : un serveur stdio (`server.ts`), son `.mcp.json` et un `package.json` |

Le plugin généré utilise la source `@skills-dir` plutôt qu'une marketplace. Les administrateurs peuvent bloquer cette source avec `strictKnownMarketplaces` ou en ajoutant `{"source": "skills-dir"}` à `blockedMarketplaces` dans les [paramètres gérés](/docs/fr/plugin-marketplaces#managed-marketplace-restrictions). Quand bloqué, `plugin init` échoue avant d'écrire.

**Exemples :**

```bash theme={null}
# Générer un plugin minimal
claude plugin init my-helper

# Générer avec des dossiers de skill et de hook
claude plugin init my-helper --with skills hooks

# Écraser un scaffold existant
claude plugin init my-helper --force
```

<h3 id="plugin-install">
  plugin install
</h3>

Installez un plugin à partir des marketplaces disponibles.

```bash theme={null}
claude plugin install <plugin> [options]
```

**Arguments :**

* `<plugin>` : Nom du plugin ou `plugin-name@marketplace-name` pour une marketplace spécifique

**Options :**

| Option                | Description                                           | Par défaut |
| :-------------------- | :---------------------------------------------------- | :--------- |
| `-s, --scope <scope>` | Portée d'installation : `user`, `project`, ou `local` | `user`     |
| `-h, --help`          | Afficher l'aide pour la commande                      |            |

La portée détermine quel fichier de paramètres le plugin installé est ajouté. Par exemple, `--scope project` écrit dans `enabledPlugins` dans .claude/settings.json, rendant le plugin disponible à tous ceux qui clonent le référentiel du projet.

**Exemples :**

```bash theme={null}
# Installer dans la portée utilisateur (par défaut)
claude plugin install formatter@my-marketplace

# Installer dans la portée du projet (partagé avec l'équipe)
claude plugin install formatter@my-marketplace --scope project

# Installer dans la portée locale (ignorée par git)
claude plugin install formatter@my-marketplace --scope local
```

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Supprimez un plugin installé.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

**Arguments :**

* `<plugin>` : Nom du plugin ou `plugin-name@marketplace-name`

**Options :**

| Option                | Description                                                                                                                | Par défaut |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------- | :--------- |
| `-s, --scope <scope>` | Désinstaller de la portée : `user`, `project`, ou `local`                                                                  | `user`     |
| `--keep-data`         | Conserver le [répertoire de données persistantes](#persistent-data-directory) du plugin                                    |            |
| `--prune`             | Supprimer également les dépendances auto-installées qu'aucun autre plugin ne nécessite. Voir [plugin prune](#plugin-prune) |            |
| `-y, --yes`           | Ignorer l'invite de confirmation `--prune`. Requis quand stdin ou stdout n'est pas un TTY                                  |            |
| `-h, --help`          | Afficher l'aide pour la commande                                                                                           |            |

**Alias :** `remove`, `rm`

Par défaut, la désinstallation de la dernière portée restante supprime également le répertoire `${CLAUDE_PLUGIN_DATA}` du plugin. Utilisez `--keep-data` pour le conserver, par exemple lors de la réinstallation après le test d'une nouvelle version.

<h3 id="plugin-prune">
  plugin prune
</h3>

Supprimez les dépendances de plugins auto-installées qui ne sont plus requises par aucun plugin installé. Les dépendances que Claude Code a intégrées pour satisfaire le champ [`dependencies`](/docs/fr/plugin-dependencies) d'un autre plugin sont supprimées ; les plugins que vous avez installés directement ne sont jamais touchés.

```bash theme={null}
claude plugin prune [options]
```

**Options :**

| Option                | Description                                                                     | Par défaut |
| :-------------------- | :------------------------------------------------------------------------------ | :--------- |
| `-s, --scope <scope>` | Nettoyer à la portée : `user`, `project`, ou `local`                            | `user`     |
| `--dry-run`           | Lister ce qui serait supprimé sans rien supprimer                               |            |
| `-y, --yes`           | Ignorer l'invite de confirmation. Requis quand stdin ou stdout n'est pas un TTY |            |
| `-h, --help`          | Afficher l'aide pour la commande                                                |            |

**Alias :** `autoremove`

La commande liste les dépendances orphelines et demande une confirmation avant de les supprimer. Pour supprimer un plugin et nettoyer ses dépendances en une seule étape, exécutez `claude plugin uninstall <plugin> --prune`.

<Note>
  `claude plugin prune` nécessite Claude Code v2.1.121 ou ultérieur.
</Note>

<h3 id="plugin-enable">
  plugin enable
</h3>

Activez un plugin désactivé. Si le plugin déclare des [dépendances](/docs/fr/plugin-dependencies), Claude Code les active transitivement à la même portée, et la commande échoue quand une dépendance n'est pas installée.

```bash theme={null}
claude plugin enable <plugin> [options]
```

**Arguments :**

* `<plugin>` : Nom du plugin ou `plugin-name@marketplace-name`

**Options :**

| Option                | Description                                      | Par défaut |
| :-------------------- | :----------------------------------------------- | :--------- |
| `-s, --scope <scope>` | Portée à activer : `user`, `project`, ou `local` | `user`     |
| `-h, --help`          | Afficher l'aide pour la commande                 |            |

<h3 id="plugin-disable">
  plugin disable
</h3>

Désactivez un plugin sans le désinstaller. Échoue quand un autre plugin activé [dépend de](/docs/fr/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies) la cible. Le message d'erreur inclut une commande chaînée qui désactive d'abord chaque dépendant.

```bash theme={null}
claude plugin disable <plugin> [options]
```

**Arguments :**

* `<plugin>` : Nom du plugin ou `plugin-name@marketplace-name`

**Options :**

| Option                | Description                                         | Par défaut |
| :-------------------- | :-------------------------------------------------- | :--------- |
| `-s, --scope <scope>` | Portée à désactiver : `user`, `project`, ou `local` | `user`     |
| `-h, --help`          | Afficher l'aide pour la commande                    |            |

<h3 id="plugin-update">
  plugin update
</h3>

Mettez à jour un plugin vers la dernière version.

```bash theme={null}
claude plugin update <plugin> [options]
```

**Arguments :**

* `<plugin>` : Nom du plugin ou `plugin-name@marketplace-name`

**Options :**

| Option                | Description                                                       | Par défaut |
| :-------------------- | :---------------------------------------------------------------- | :--------- |
| `-s, --scope <scope>` | Portée à mettre à jour : `user`, `project`, `local`, ou `managed` | `user`     |
| `-h, --help`          | Afficher l'aide pour la commande                                  |            |

***

<h3 id="plugin-list">
  plugin list
</h3>

Listez les plugins installés avec leur version, la marketplace source et le statut d'activation.

```bash theme={null}
claude plugin list [options]
```

**Options :**

| Option        | Description                                                          | Par défaut |
| :------------ | :------------------------------------------------------------------- | :--------- |
| `--json`      | Sortie en JSON                                                       |            |
| `--available` | Inclure les plugins disponibles des marketplaces. Nécessite `--json` |            |
| `-h, --help`  | Afficher l'aide pour la commande                                     |            |

Dans une session interactive, `/plugin list` affiche le même listing en ligne. La forme interactive accepte `--enabled` ou `--disabled` pour afficher uniquement les plugins dans cet état, et `ls` comme raccourci pour `list`.

<h3 id="plugin-details">
  plugin details
</h3>

Afficher l'inventaire des composants d'un plugin et le coût en tokens projeté. La sortie liste tous les composants que le plugin contribue, regroupés en tant que Skills, Agents, Hooks, serveurs MCP et serveurs LSP, ainsi qu'une estimation du nombre de tokens qu'il ajoute à chaque session. Le groupe Skills inclut à la fois les entrées `skills/` et `commands/`.

```bash theme={null}
claude plugin details <name>
```

**Arguments :**

* `<name>` : Nom du plugin ou `plugin-name@marketplace-name`

**Options :**

| Option       | Description                      | Par défaut |
| :----------- | :------------------------------- | :--------- |
| `-h, --help` | Afficher l'aide pour la commande |            |

La sortie affiche deux chiffres de coût pour chaque composant :

* **Always-on :** tokens ajoutés à chaque session par le texte de liste du plugin, comme les descriptions de compétences, les descriptions d'agents et les noms de commandes, indépendamment du fait qu'un composant se déclenche ou non.
* **On-invoke :** tokens qu'un composant coûte quand il se déclenche. Affiché par composant, pas comme un total de plugin, car une session typique n'invoque qu'un sous-ensemble de composants.

Cet exemple montre à quoi ressemble la sortie pour un plugin avec deux compétences :

```
dependency-guard 1.2.0
  Dependency analysis for Claude Code sessions
  Source: dependency-guard@example-marketplace

Component inventory
  Skills (2)  scan-dependencies, review-changes
  Agents (0)
  Hooks (1)  (harness-only — no model context cost)
  MCP servers (0)
  LSP servers (0)

Projected token cost
  Always-on:   ~180 tok   added to every session

Per-component (rounded)
  component            always-on  on-invoke
  scan-dependencies        ~100      ~2400
  review-changes            ~80      ~1800

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Le total always-on est calculé via l'API `count_tokens` pour votre modèle actif. Les nombres par composant sont proportionnellement mis à l'échelle à partir de ce total. Si l'API est inaccessible, la commande revient à une estimation basée sur les caractères.

<h3 id="plugin-tag">
  plugin tag
</h3>

Créez une balise de version git pour le plugin dans le répertoire actuel. Exécutez depuis l'intérieur du dossier du plugin. Voir [Baliser les versions des plugins](/docs/fr/plugin-dependencies#tag-plugin-releases-for-version-resolution).

```bash theme={null}
claude plugin tag [options]
```

**Options :**

| Option        | Description                                                                            | Par défaut |
| :------------ | :------------------------------------------------------------------------------------- | :--------- |
| `--push`      | Pousser la balise vers le serveur distant après sa création                            |            |
| `--dry-run`   | Afficher ce qui serait balisé sans créer la balise                                     |            |
| `-f, --force` | Créer la balise même si l'arborescence de travail est sale ou si la balise existe déjà |            |
| `-h, --help`  | Afficher l'aide pour la commande                                                       |            |

***

<h2 id="debugging-and-development-tools">
  Outils de débogage et de développement
</h2>

<h3 id="debugging-commands">
  Commandes de débogage
</h3>

Utilisez `claude --debug` pour voir les détails du chargement des plugins :

Cela affiche :

* Quels plugins sont en cours de chargement
* Toute erreur dans les manifestes de plugins
* Enregistrement des skills, agents et hooks
* Initialisation du serveur MCP

<h3 id="common-issues">
  Problèmes courants
</h3>

| Problème                            | Cause                              | Solution                                                                                                                                                                                       |
| :---------------------------------- | :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plugin ne se charge pas             | `plugin.json` invalide             | Exécutez `claude plugin validate` ou `/plugin validate` pour vérifier `plugin.json`, le frontmatter des skills/agents/commandes et `hooks/hooks.json` pour les erreurs de syntaxe et de schéma |
| Les skills n'apparaissent pas       | Structure de répertoire incorrecte | Assurez-vous que `skills/` ou `commands/` est à la racine du plugin, pas à l'intérieur de `.claude-plugin/`                                                                                    |
| Les hooks ne se déclenchent pas     | Le script n'est pas exécutable     | Exécutez `chmod +x script.sh`                                                                                                                                                                  |
| Le serveur MCP échoue               | `${CLAUDE_PLUGIN_ROOT}` manquant   | Utilisez la variable pour tous les chemins de plugin                                                                                                                                           |
| Erreurs de chemin                   | Chemins absolus utilisés           | Tous les chemins doivent être relatifs et commencer par `./`                                                                                                                                   |
| LSP `Executable not found in $PATH` | Serveur de langage non installé    | Installez le binaire (par exemple, `npm install -g typescript-language-server typescript`)                                                                                                     |

<h3 id="example-error-messages">
  Exemples de messages d'erreur
</h3>

**Erreurs de validation du manifeste** :

* `Invalid JSON syntax: Unexpected token } in JSON at position 142` : vérifiez les virgules manquantes, les virgules supplémentaires ou les chaînes non citées
* `Plugin has an invalid manifest file at .claude-plugin/plugin.json. Validation errors: name: Required` : un champ obligatoire est manquant
* `Plugin has a corrupt manifest file at .claude-plugin/plugin.json. JSON parse error: ...` : erreur de syntaxe JSON

**Erreurs de chargement du plugin** :

* `Warning: No commands found in plugin my-plugin custom directory: ./cmds. Expected .md files or SKILL.md in subdirectories.` : le chemin de commande existe mais ne contient aucun fichier de commande valide
* `Plugin directory not found at path: ./plugins/my-plugin. Check that the marketplace entry has the correct path.` : le chemin `source` dans marketplace.json pointe vers un répertoire inexistant
* `Plugin my-plugin has conflicting manifests: both plugin.json and marketplace entry specify components.` : supprimez les définitions de composants en double ou supprimez `strict: false` dans l'entrée de la marketplace

<h3 id="hook-troubleshooting">
  Dépannage des hooks
</h3>

**Le script du hook ne s'exécute pas** :

1. Vérifiez que le script est exécutable : `chmod +x ./scripts/your-script.sh`
2. Vérifiez la ligne shebang : La première ligne doit être `#!/bin/bash` ou `#!/usr/bin/env bash`
3. Vérifiez que le chemin utilise `${CLAUDE_PLUGIN_ROOT}` : `"command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/your-script.sh"`
4. Testez le script manuellement : `./scripts/your-script.sh`

**Le hook ne se déclenche pas sur les événements attendus** :

1. Vérifiez que le nom de l'événement est correct (sensible à la casse) : `PostToolUse`, pas `postToolUse`
2. Vérifiez que le motif de correspondance correspond à vos outils : `"matcher": "Write|Edit"` pour les opérations de fichier
3. Confirmez que le type de hook est valide : `command`, `http`, `mcp_tool`, `prompt`, ou `agent`

<h3 id="mcp-server-troubleshooting">
  Dépannage du serveur MCP
</h3>

**Le serveur ne démarre pas** :

1. Vérifiez que la commande existe et est exécutable
2. Vérifiez que tous les chemins utilisent la variable `${CLAUDE_PLUGIN_ROOT}`
3. Vérifiez les journaux du serveur MCP : `claude --debug` affiche les erreurs d'initialisation
4. Testez le serveur manuellement en dehors de Claude Code

**Les outils du serveur n'apparaissent pas** :

1. Assurez-vous que le serveur est correctement configuré dans `.mcp.json` ou `plugin.json`
2. Vérifiez que le serveur implémente correctement le protocole MCP
3. Vérifiez les délais d'expiration de la connexion dans la sortie de débogage

<h3 id="directory-structure-mistakes">
  Erreurs de structure de répertoire
</h3>

**Symptômes** : Le plugin se charge mais les composants (skills, agents, hooks) sont manquants.

**Structure correcte** : Les composants doivent être à la racine du plugin, pas à l'intérieur de `.claude-plugin/`. Seul `plugin.json` appartient à `.claude-plugin/`.

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json      ← Seul le manifeste ici
├── commands/            ← Au niveau racine
├── agents/              ← Au niveau racine
└── hooks/               ← Au niveau racine
```

Si vos composants sont à l'intérieur de `.claude-plugin/`, déplacez-les à la racine du plugin.

**Liste de contrôle de débogage** :

1. Exécutez `claude --debug` et recherchez les messages « loading plugin »
2. Vérifiez que chaque répertoire de composants est listé dans la sortie de débogage
3. Vérifiez que les permissions de fichier permettent de lire les fichiers du plugin

***

<h2 id="distribution-and-versioning-reference">
  Référence de distribution et de versioning
</h2>

<h3 id="version-management">
  Gestion des versions
</h3>

Claude Code utilise la version du plugin comme clé de cache qui détermine si une mise à jour est disponible. Lorsque vous exécutez `/plugin update` ou que la mise à jour automatique se déclenche, Claude Code calcule la version actuelle et ignore la mise à jour si elle correspond à celle déjà installée.

La version est résolue à partir du premier de ces éléments qui est défini :

1. Le champ `version` dans le `plugin.json` du plugin
2. Le champ `version` dans l'entrée marketplace du plugin dans `marketplace.json`
3. Le SHA du commit git du plugin source, pour les sources `github`, `url`, `git-subdir` et relative-path dans une marketplace hébergée sur git
4. `unknown`, pour les sources `npm` ou les répertoires locaux ne se trouvant pas dans un référentiel git

Cela vous donne deux façons de versionner un plugin :

| Approche                  | Comment                                                                 | Comportement de mise à jour                                                                                                                                                                                       | Idéal pour                                             |
| :------------------------ | :---------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------- |
| **Version explicite**     | Définissez `"version": "2.1.0"` dans `plugin.json`                      | Les utilisateurs reçoivent les mises à jour uniquement lorsque vous augmentez ce champ. Pousser de nouveaux commits sans l'augmenter n'a aucun effet, et `/plugin update` signale « déjà à la dernière version ». | Plugins publiés avec des cycles de publication stables |
| **Version SHA du commit** | Omettez `version` à la fois de `plugin.json` et de l'entrée marketplace | Les utilisateurs reçoivent les mises à jour à chaque nouveau commit vers la source git du plugin                                                                                                                  | Plugins internes ou d'équipe en développement actif    |

<Warning>
  Si vous définissez `version` dans `plugin.json`, vous devez l'augmenter chaque fois que vous souhaitez que les utilisateurs reçoivent les modifications. Pousser de nouveaux commits seul ne suffit pas, car Claude Code voit la même chaîne de version et conserve la copie en cache. Si vous itérez rapidement, laissez `version` non défini afin que le SHA du commit git soit utilisé à la place.
</Warning>

Si vous utilisez des versions explicites, suivez le [versioning sémantique](https://semver.org) (`MAJOR.MINOR.PATCH`) : augmentez MAJOR pour les changements cassants, MINOR pour les nouvelles fonctionnalités, PATCH pour les corrections de bugs. Documentez les modifications dans un `CHANGELOG.md`.

***

<h2 id="see-also">
  Voir aussi
</h2>

* [Plugins](/docs/fr/plugins) - Tutoriels et utilisation pratique
* [Marketplaces de plugins](/docs/fr/plugin-marketplaces) - Création et gestion des marketplaces
* [Skills](/docs/fr/skills) - Détails du développement des skills
* [Subagents](/docs/fr/sub-agents) - Configuration et capacités des agents
* [Hooks](/docs/fr/hooks) - Gestion des événements et automatisation
* [MCP](/docs/fr/mcp) - Intégration des outils externes
* [Paramètres](/docs/fr/settings) - Options de configuration pour les plugins
