> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence des hooks

> Référence pour les événements de hook Claude Code, le schéma de configuration, les formats d'entrée/sortie JSON, les codes de sortie, les hooks asynchrones, les hooks HTTP, les hooks de prompt et les hooks d'outils MCP.

<Tip>
  Pour un guide de démarrage rapide avec des exemples, consultez [Automatiser les actions avec les hooks](/docs/fr/hooks-guide).
</Tip>

Les hooks sont des commandes shell définies par l'utilisateur, des points de terminaison HTTP ou des prompts LLM qui s'exécutent automatiquement à des points spécifiques du cycle de vie de Claude Code. Utilisez cette référence pour consulter les schémas d'événements, les options de configuration, les formats d'entrée/sortie JSON et les fonctionnalités avancées comme les hooks asynchrones, les hooks HTTP et les hooks d'outils MCP. Si vous configurez des hooks pour la première fois, commencez plutôt par le [guide](/docs/fr/hooks-guide).

<h2 id="hook-lifecycle">
  Cycle de vie des hooks
</h2>

Les hooks se déclenchent à des points spécifiques pendant une session Claude Code. Lorsqu'un événement se déclenche et qu'un matcher correspond, Claude Code transmet le contexte JSON de l'événement à votre gestionnaire de hook. Pour les hooks de commande, l'entrée arrive sur stdin. Pour les hooks HTTP, elle arrive dans le corps de la requête POST. Votre gestionnaire peut alors inspecter l'entrée, prendre une action et éventuellement retourner une décision.

Les événements se déclenchent selon trois cadences :

* une fois par session : `SessionStart` et `SessionEnd`
* une fois par tour : `UserPromptSubmit`, `Stop` et `StopFailure`
* à chaque appel d'outil à l'intérieur de la boucle agentique : `PreToolUse` et `PostToolUse`

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/jhXrDR5TrSZ5hgXM/images/hooks-lifecycle.svg?fit=max&auto=format&n=jhXrDR5TrSZ5hgXM&q=85&s=3ca47113d5956460e6e4611b8dbc63b7" alt="Diagramme du cycle de vie des hooks montrant Setup optionnel alimentant SessionStart, puis une boucle par tour contenant UserPromptSubmit, UserPromptExpansion pour les slash commands, la boucle agentique imbriquée (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), et Stop ou StopFailure, suivis de TeammateIdle, PreCompact, PostCompact et SessionEnd, avec Elicitation et ElicitationResult imbriqués dans l'exécution de l'outil MCP, PermissionDenied comme branche latérale de PermissionRequest pour les refus en mode auto, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged et FileChanged comme événements asynchrones autonomes, et MessageDisplay comme événement d'affichage uniquement qui s'exécute pendant que le texte du message de l'assistant est diffusé en continu" width="520" height="1228" data-path="images/hooks-lifecycle.svg" />
  </Frame>
</div>

Le tableau ci-dessous résume le moment où chaque événement se déclenche. La section [Événements de hook](#hook-events) documente le schéma d'entrée complet et les options de contrôle de décision pour chacun.

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

<h3 id="how-a-hook-resolves">
  Comment un hook se résout
</h3>

Pour voir comment ces éléments s'assemblent, considérez ce hook `PreToolUse` qui bloque les commandes shell destructrices. Le `matcher` se limite aux appels d'outil Bash et la condition `if` se limite davantage aux commandes Bash correspondant à `rm *`, donc `block-rm.sh` ne s'exécute que lorsque les deux filtres correspondent :

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(rm *)",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

Le script lit l'entrée JSON depuis stdin, extrait la commande et retourne une `permissionDecision` de `"deny"` si elle contient `rm -rf` :

```bash theme={null}
#!/bin/bash
# .claude/hooks/block-rm.sh
COMMAND=$(jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q 'rm -rf'; then
  jq -n '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: "Destructive command blocked by hook"
    }
  }'
else
  exit 0  # no decision; normal permission flow applies
fi
```

Supposons maintenant que Claude Code décide d'exécuter `Bash "rm -rf /tmp/build"`. Voici ce qui se passe :

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" alt="Diagramme de résolution du hook : PreToolUse se déclenche, le matcher vérifie la correspondance Bash, la condition if vérifie la correspondance Bash(rm *). Si les deux correspondent, la commande du hook s'exécute et retourne permissionDecision deny, donc l'appel d'outil est bloqué et Claude Code continue. Si l'une des vérifications ne correspond pas, le hook est ignoré et l'appel d'outil est autorisé à procéder." width="930" height="270" data-path="images/hook-resolution.svg" />
</Frame>

<Steps>
  <Step title="L'événement se déclenche">
    L'événement `PreToolUse` se déclenche. Claude Code envoie l'entrée de l'outil en JSON sur stdin au hook :

    ```json theme={null}
    { "tool_name": "Bash", "tool_input": { "command": "rm -rf /tmp/build" }, ... }
    ```
  </Step>

  <Step title="Le matcher vérifie">
    Le matcher `"Bash"` correspond au nom de l'outil, donc ce groupe de hook s'active. Si vous omettez le matcher ou utilisez `"*"`, le groupe s'active à chaque occurrence de l'événement.
  </Step>

  <Step title="La condition if vérifie">
    La condition `if` `"Bash(rm *)"` correspond car `rm -rf /tmp/build` est une sous-commande correspondant à `rm *`, donc ce gestionnaire s'exécute. Si la commande avait été `npm test`, la vérification `if` échouerait et `block-rm.sh` ne s'exécuterait jamais, évitant la surcharge de génération de processus. Le champ `if` est optionnel ; sans lui, chaque gestionnaire du groupe correspondant s'exécute.
  </Step>

  <Step title="Le gestionnaire de hook s'exécute">
    Le script inspecte la commande complète et trouve `rm -rf`, donc il imprime une décision sur stdout :

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by hook"
      }
    }
    ```

    Si la commande avait été une variante plus sûre de `rm` comme `rm file.txt`, le script aurait atteint `exit 0` à la place. Un code de sortie 0 sans sortie signifie que le hook n'a pas de décision à signaler, donc l'appel d'outil continue à travers le [flux de permission](/docs/fr/permissions) normal. Le hook peut refuser l'appel, mais rester silencieux ne l'approuve pas.
  </Step>

  <Step title="Claude Code agit sur le résultat">
    Claude Code lit la décision JSON, bloque l'appel d'outil et montre la raison à Claude.
  </Step>
</Steps>

La section [Configuration](#configuration) ci-dessous documente le schéma complet, et chaque section [événement de hook](#hook-events) documente l'entrée que votre commande reçoit et la sortie qu'elle peut retourner.

<h2 id="configuration">
  Configuration
</h2>

Les hooks sont définis dans les fichiers de paramètres JSON. La configuration a trois niveaux d'imbrication :

1. Choisissez un [événement de hook](#hook-events) auquel répondre, comme `PreToolUse` ou `Stop`
2. Ajoutez un [groupe de matcher](#matcher-patterns) pour filtrer quand il se déclenche, comme « uniquement pour l'outil Bash »
3. Définissez un ou plusieurs [gestionnaires de hook](#hook-handler-fields) à exécuter lorsqu'il y a correspondance

Consultez [Comment un hook se résout](#how-a-hook-resolves) ci-dessus pour une procédure pas à pas complète avec un exemple annoté.

<Note>
  Cette page utilise des termes spécifiques pour chaque niveau : **événement de hook** pour le point du cycle de vie, **groupe de matcher** pour le filtre et **gestionnaire de hook** pour la commande shell, le point de terminaison HTTP, l'outil MCP, le prompt ou l'agent qui s'exécute. « Hook » seul fait référence à la fonctionnalité générale.
</Note>

<h3 id="hook-locations">
  Emplacements des hooks
</h3>

L'endroit où vous définissez un hook détermine sa portée :

| Emplacement                                                | Portée                             | Partageable                                     |
| :--------------------------------------------------------- | :--------------------------------- | :---------------------------------------------- |
| `~/.claude/settings.json`                                  | Tous vos projets                   | Non, local à votre machine                      |
| `.claude/settings.json`                                    | Projet unique                      | Oui, peut être commité dans le repo             |
| `.claude/settings.local.json`                              | Projet unique                      | Non, ignoré par git lorsque Claude Code le crée |
| Paramètres de politique gérée                              | À l'échelle de l'organisation      | Oui, contrôlé par l'administrateur              |
| [Plugin](/docs/fr/plugins) `hooks/hooks.json`                   | Lorsque le plugin est activé       | Oui, fourni avec le plugin                      |
| Frontmatter [Skill](/docs/fr/skills) ou [agent](/docs/fr/sub-agents) | Pendant que le composant est actif | Oui, défini dans le fichier du composant        |

Pour plus de détails sur la résolution des fichiers de paramètres, consultez [paramètres](/docs/fr/settings). Les administrateurs d'entreprise peuvent utiliser `allowManagedHooksOnly` pour bloquer les hooks utilisateur, projet et plugin. Les hooks des plugins forcément activés dans les paramètres gérés `enabledPlugins` sont exempts, donc les administrateurs peuvent distribuer les hooks vérifiés via un marketplace d'organisation. Consultez [Configuration des hooks](/docs/fr/settings#hook-configuration).

<h3 id="matcher-patterns">
  Modèles de matcher
</h3>

Le champ `matcher` filtre quand les hooks se déclenchent. La façon dont un matcher est évalué dépend des caractères qu'il contient :

| Valeur du matcher                                                        | Évalué comme                                                                                             | Exemple                                                                                                                                                                                        |
| :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"*"`, `""` ou omis                                                      | Correspondre à tous                                                                                      | se déclenche à chaque occurrence de l'événement                                                                                                                                                |
| Uniquement des lettres, des chiffres, `_`, `-`, des espaces, `,` et `\|` | Chaîne exacte ou liste de chaînes exactes séparées par `\|` ou `,` avec espaces blancs optionnels autour | `Bash` correspond uniquement à l'outil Bash ; `Edit\|Write` et `Edit, Write` correspondent chacun à l'un ou l'autre outil exactement ; `code-reviewer` correspond uniquement à ce type d'agent |
| Contient tout autre caractère                                            | Expression régulière JavaScript, non ancrée                                                              | `^Notebook` correspond à tout outil commençant par Notebook ; `mcp__memory__.*` correspond à chaque outil du serveur `memory`                                                                  |

Un matcher sur le chemin de l'expression régulière est testé avec `RegExp.prototype.test` de JavaScript, qui réussit sur une correspondance n'importe où dans la valeur. `Edit.*` correspond à la fois à `Edit` et à `NotebookEdit` ; enveloppez le modèle dans `^` et `$`, comme dans `^Edit$`, lorsque vous avez besoin d'une correspondance de chaîne entière.

Les séparateurs par virgule et la tolérance des espaces blancs environnants nécessitent Claude Code v2.1.191 ou ultérieur.

Les traits d'union dans l'ensemble de correspondance exacte nécessitent Claude Code v2.1.195 ou ultérieur. Sur les versions antérieures, un nom avec trait d'union comme `code-reviewer` est évalué comme une expression régulière non ancrée, donc il se déclenche également pour `senior-code-reviewer` ; ancrez-le comme `^code-reviewer$` sur ces versions pour correspondre uniquement à ce nom.

`FileChanged` et `StopFailure` utilisent un ensemble de correspondance exacte plus étroit contenant uniquement des lettres, des chiffres, `_` et `|`. Un trait d'union, un espace ou une virgule dans un matcher pour ces deux événements le maintient sur le chemin de l'expression régulière, et seul `|` sépare les alternatives. Tous les autres événements avec support de matcher dans le tableau qui suit acceptent `|` ou `,`.

L'événement `FileChanged` ne suit pas ces règles lors de la construction de sa liste de surveillance. Consultez [FileChanged](#filechanged).

Chaque type d'événement correspond sur un champ différent :

| Événement                                                                                                                                         | Ce que le matcher filtre                                                        | Exemples de valeurs de matcher                                                                                                                                                      |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | nom de l'outil                                                                  | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                    | comment la session a démarré                                                    | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                           | quel drapeau CLI a déclenché la configuration                                   | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                      | pourquoi la session s'est terminée                                              | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                    | type de notification                                                            | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                   | type d'agent                                                                    | `general-purpose`, `Explore`, `Plan`, noms d'agents personnalisés ou noms limités au plugin comme `^my-plugin:reviewer$`                                                            |
| `PreCompact`, `PostCompact`                                                                                                                       | ce qui a déclenché la compaction                                                | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                    | type d'agent                                                                    | mêmes valeurs que `SubagentStart`                                                                                                                                                   |
| `ConfigChange`                                                                                                                                    | source de configuration                                                         | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `CwdChanged`                                                                                                                                      | pas de support de matcher                                                       | se déclenche toujours à chaque changement de répertoire                                                                                                                             |
| `FileChanged`                                                                                                                                     | noms de fichiers littéraux à surveiller (consultez [FileChanged](#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `StopFailure`                                                                                                                                     | type d'erreur                                                                   | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                              | raison du chargement                                                            | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `UserPromptExpansion`                                                                                                                             | nom de la commande                                                              | vos noms de skill ou de commande                                                                                                                                                    |
| `Elicitation`                                                                                                                                     | nom du serveur MCP                                                              | vos noms de serveur MCP configurés                                                                                                                                                  |
| `ElicitationResult`                                                                                                                               | nom du serveur MCP                                                              | mêmes valeurs que `Elicitation`                                                                                                                                                     |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | pas de support de matcher                                                       | se déclenche toujours à chaque occurrence                                                                                                                                           |

Le matcher s'exécute sur un champ de l'[entrée JSON](#hook-input-and-output) que Claude Code envoie à votre hook sur stdin. Pour les événements d'outil, ce champ est `tool_name`. Chaque section [événement de hook](#hook-events) liste l'ensemble complet des valeurs de matcher et le schéma d'entrée pour cet événement.

Cet exemple exécute un script de linting uniquement lorsque Claude écrit ou édite un fichier :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/lint-check.sh"
          }
        ]
      }
    ]
  }
}
```

`UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` et `CwdChanged` ne supportent pas les matchers et se déclenchent toujours à chaque occurrence. Si vous ajoutez un champ `matcher` à ces événements, il est silencieusement ignoré.

Pour les événements d'outil, vous pouvez filtrer plus étroitement en définissant le champ [`if`](#common-fields) sur les gestionnaires de hook individuels. `if` utilise la [syntaxe des règles de permission](/docs/fr/permissions) pour correspondre au nom de l'outil et aux arguments ensemble, donc `"Bash(git *)"` s'exécute lorsqu'une sous-commande quelconque de l'entrée Bash correspond à `git *` et `"Edit(*.ts)"` s'exécute uniquement pour les fichiers TypeScript.

<h4 id="match-mcp-tools">
  Correspondre aux outils MCP
</h4>

Les outils du serveur [MCP](/docs/fr/mcp) apparaissent comme des outils réguliers dans les événements d'outil (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), vous pouvez donc les faire correspondre de la même manière que tout autre nom d'outil.

Les outils MCP suivent le modèle de nommage `mcp__<server>__<tool>`, par exemple :

* `mcp__memory__create_entities` : outil de création d'entités du serveur Memory
* `mcp__filesystem__read_file` : outil de lecture de fichier du serveur Filesystem
* `mcp__github__search_repositories` : outil de recherche du serveur GitHub

Pour correspondre à chaque outil d'un serveur, ajoutez `.*` au préfixe du serveur. Le `.*` est requis : un matcher comme `mcp__memory` ou `mcp__brave-search` contient uniquement des caractères de correspondance exacte, donc il est comparé comme une chaîne exacte et ne correspond à aucun outil.

* `mcp__memory__.*` correspond à tous les outils du serveur `memory`
* `mcp__brave-search__.*` correspond à tous les outils d'un serveur dont le nom contient un trait d'union
* `mcp__.*__write.*` correspond à tout outil dont le nom commence par `write` de n'importe quel serveur

Les traits d'union dans l'ensemble de correspondance exacte nécessitent Claude Code v2.1.195 ou ultérieur. Sur les versions antérieures, un préfixe nu avec trait d'union comme `mcp__brave-search` est évalué comme une expression régulière non ancrée et correspond à chaque outil de ce serveur. La forme `mcp__brave-search__.*` fonctionne sur chaque version.

Les outils d'un [serveur MCP fourni par un plugin](/docs/fr/mcp#plugin-provided-mcp-servers) utilisent un segment de serveur limité qui inclut le nom du plugin : `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Un matcher écrit contre la clé de serveur nue ne se déclenche jamais pour ces outils. Pour un plugin nommé `my-plugin` qui regroupe un serveur sous la clé `db`, un outil `query` apparaît comme `mcp__plugin_my-plugin_db__query`, donc le matcher pour chaque outil de ce serveur est `mcp__plugin_my-plugin_db__.*`. Utilisez le même nom d'outil limité dans le champ [`if`](#common-fields) d'un gestionnaire. Consultez [Serveurs MCP fournis par un plugin](/docs/fr/mcp#plugin-provided-mcp-servers) pour savoir comment le nom limité est construit.

Cet exemple enregistre toutes les opérations du serveur memory et valide les opérations d'écriture de n'importe quel serveur MCP :

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

<h3 id="hook-handler-fields">
  Champs du gestionnaire de hook
</h3>

Chaque objet du tableau `hooks` interne est un gestionnaire de hook : la commande shell, le point de terminaison HTTP, l'outil MCP, le prompt LLM ou l'agent qui s'exécute lorsque le matcher correspond. Il y a cinq types :

* **[Hooks de commande](#command-hook-fields)** (`type: "command"`) : exécutent une commande shell. Votre script reçoit l'[entrée JSON](#hook-input-and-output) de l'événement sur stdin et communique les résultats via les codes de sortie et stdout.
* **[Hooks HTTP](#http-hook-fields)** (`type: "http"`) : envoient l'entrée JSON de l'événement en tant que requête HTTP POST à une URL. Le point de terminaison communique les résultats via le corps de la réponse en utilisant le même [format de sortie JSON](#json-output) que les hooks de commande.
* **[Hooks de l'outil MCP](#mcp-tool-hook-fields)** (`type: "mcp_tool"`) : appellent un outil sur un serveur [MCP](/docs/fr/mcp) déjà connecté. La sortie textuelle de l'outil est traitée comme stdout d'un hook de commande.
* **[Hooks de prompt](#prompt-and-agent-hook-fields)** (`type: "prompt"`) : envoient un prompt à un modèle Claude pour une évaluation en un seul tour. Le modèle retourne une décision oui/non en JSON. Consultez [Hooks basés sur des prompts](#prompt-based-hooks).
* **[Hooks d'agent](#prompt-and-agent-hook-fields)** (`type: "agent"`) : lancent un subagent qui peut utiliser des outils comme Read, Grep et Glob pour vérifier les conditions avant de retourner une décision. Les hooks d'agent sont expérimentaux et peuvent changer. Consultez [Hooks basés sur des agents](#agent-based-hooks).

Tous les hooks correspondants s'exécutent en parallèle, et les gestionnaires identiques sont automatiquement dédupliqués. Les hooks de commande sont dédupliqués par chaîne de commande et `args`, et les hooks HTTP sont dédupliqués par URL.

Les gestionnaires s'exécutent dans le répertoire courant avec l'environnement de Claude Code. La variable d'environnement `$CLAUDE_CODE_REMOTE` est définie à `"true"` dans les environnements web distants et n'est pas définie dans le CLI local. À partir de v2.1.199, [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/fr/env-vars) est défini à l'ID de session [Contrôle à distance](/docs/fr/remote-control) tandis que la session locale a une connexion Contrôle à distance active.

<h4 id="common-fields">
  Champs communs
</h4>

Ces champs s'appliquent à tous les types de hooks :

| Champ           | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :-------------- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | oui    | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` ou `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `if`            | non    | Syntaxe de règle de permission pour filtrer quand ce hook s'exécute, comme `"Bash(git *)"` ou `"Edit(*.ts)"`. Le hook de commande ne s'exécute que si l'appel d'outil correspond au modèle. Consultez le [tableau de correspondance Bash](#bash-if-matching) ci-dessous pour voir comment les modèles Bash s'évaluent par rapport aux sous-commandes, `$()` et aux backticks. Évalué uniquement sur les événements d'outil : `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` et `PermissionDenied`. Sur les autres événements, un hook avec `if` défini ne s'exécute jamais. Utilise la même syntaxe que les [règles de permission](/docs/fr/permissions) |
| `timeout`       | non    | Secondes avant annulation. Valeurs par défaut : 600 pour `command`, `http` et `mcp_tool` ; 30 pour `prompt` ; 60 pour `agent`. [`UserPromptSubmit`](#userpromptsubmit) abaisse la valeur par défaut de `command`, `http` et `mcp_tool` à 30, et [`MessageDisplay`](#messagedisplay) l'abaisse à 10                                                                                                                                                                                                                                                                                                                                                                          |
| `statusMessage` | non    | Message de spinner personnalisé affiché pendant l'exécution du hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `once`          | non    | Si `true`, s'exécute une seule fois par session puis est supprimé. Honoré uniquement pour les hooks déclarés dans le [frontmatter des skills](#hooks-in-skills-and-agents) ; ignoré dans les fichiers de paramètres et le frontmatter des agents                                                                                                                                                                                                                                                                                                                                                                                                                            |

Le champ `if` contient exactement une règle de permission. Il n'y a pas de syntaxe `&&`, `||` ou de liste pour combiner les règles ; pour appliquer plusieurs conditions, définissez un gestionnaire de hook séparé pour chacune.

<span id="bash-if-matching" />Pour les modèles Bash, le fait que votre commande de hook s'exécute dépend de la forme du modèle et de la commande Bash que Claude invoque. Les affectations `VAR=value` en début sont supprimées avant la correspondance.

| Modèle `if`        | Commande Bash          | Le hook s'exécute-t-il ? | Pourquoi                                                                                                                      |
| :----------------- | :--------------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`     | oui                      | les affectations en début sont supprimées ; `git push` correspond                                                             |
| `Bash(git *)`      | `npm test && git push` | oui                      | chaque sous-commande est vérifiée ; `git push` correspond                                                                     |
| `Bash(rm *)`       | `echo $(rm -rf /)`     | oui                      | les commandes à l'intérieur de `$()` et des backticks sont vérifiées ; `rm -rf /` correspond                                  |
| `Bash(rm *)`       | `echo $(date)`         | non                      | aucune sous-commande ne correspond à `rm *`                                                                                   |
| `Bash(git push *)` | `echo $(date)`         | oui                      | les modèles qui spécifient plus que le nom de la commande exécutent le hook de toute façon sur `$()`, les backticks ou `$VAR` |

Le filtre échoue également ouvert, exécutant votre hook indépendamment du modèle, lorsque la commande Bash ne peut pas être analysée. Parce que le filtre `if` est au mieux un effort, utilisez le [système de permission](/docs/fr/permissions) plutôt qu'un hook pour appliquer une autorisation ou un refus strict.

<h4 id="command-hook-fields">
  Champs des hooks de commande
</h4>

En plus des [champs communs](#common-fields), les hooks de commande acceptent ces champs :

| Champ         | Requis | Description                                                                                                                                                                                                                                                                                                                                                             |
| :------------ | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | oui    | Commande shell à exécuter. Avec `args`, l'exécutable à lancer directement. Consultez [Forme exec et forme shell](#exec-form-and-shell-form)                                                                                                                                                                                                                             |
| `args`        | non    | Liste d'arguments. Lorsqu'elle est présente, `command` est résolu comme un exécutable et lancé directement avec `args` comme vecteur d'arguments, sans shell. Consultez [Forme exec et forme shell](#exec-form-and-shell-form)                                                                                                                                          |
| `async`       | non    | Si `true`, s'exécute en arrière-plan sans bloquer. Consultez [Exécuter les hooks en arrière-plan](#run-hooks-in-the-background)                                                                                                                                                                                                                                         |
| `asyncRewake` | non    | Si `true`, s'exécute en arrière-plan et réveille Claude au code de sortie 2. Implique `async`. Le stderr du hook, ou stdout s'il est vide, est affiché à Claude comme un rappel système afin qu'il puisse réagir à un échec en arrière-plan de longue durée                                                                                                             |
| `shell`       | non    | Shell à utiliser pour ce hook. Accepte `"bash"` ou `"powershell"`. Par défaut `"bash"`, ou `"powershell"` sur Windows lorsque Git Bash n'est pas installé. Définir `"powershell"` exécute la commande via PowerShell sur Windows. Ne nécessite pas `CLAUDE_CODE_USE_POWERSHELL_TOOL` puisque les hooks lancent PowerShell directement. Ignoré lorsque `args` est défini |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Forme exec et forme shell
</h5>

Un hook de commande s'exécute en forme exec lorsque `args` est défini, et en forme shell lorsque `args` est omis. Définissez `args` chaque fois que le hook référence un [placeholder de chemin](#reference-scripts-by-path), puisque chaque élément est passé comme un argument sans guillemets. Omettez `args` lorsque vous avez besoin de fonctionnalités shell comme les pipes ou `&&`, ou lorsqu'aucune de ces préoccupations ne s'applique.

**Forme exec** s'exécute lorsque `args` est présent. Claude Code résout `command` comme un exécutable sur `PATH` et le lance directement avec `args` comme vecteur d'arguments. Il n'y a pas de shell, donc chaque élément `args` est un argument exactement tel qu'écrit, et les placeholders de chemin comme `${CLAUDE_PLUGIN_ROOT}` sont substitués dans `command` et dans chaque élément `args` comme des chaînes brutes. Les caractères spéciaux tels que les apostrophes, `$` et les backticks passent verbatim car il n'y a pas de shell pour les interpréter. Aucune tokenisation shell ne se produit sur aucune plateforme.

**Forme shell** s'exécute lorsque `args` est absent. La chaîne `command` est passée à un shell : `sh -c` sur macOS et Linux, Git Bash sur Windows, ou PowerShell lorsque Git Bash n'est pas installé. Définissez le champ `shell` pour choisir explicitement. Le shell tokenise la chaîne, développe les variables et interprète les pipes, `&&`, les redirections et les globs.

<Note>
  Sur Windows, la forme exec nécessite que `command` se résolve en un véritable exécutable tel qu'un `.exe`. Les shims `.cmd` et `.bat` que npm, npx, eslint et d'autres outils installent dans `node_modules/.bin` ne sont pas des exécutables et ne peuvent pas être lancés sans un shell. Pour les exécuter en forme exec, invoquez le script sous-jacent avec `node` directement, par exemple `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. Le modèle `node` plus chemin de script fonctionne sur chaque plateforme car `node.exe` est un vrai binaire. Pour exécuter un shim `.cmd` ou `.bat` par nom, utilisez la forme shell.
</Note>

Cet exemple exécute un script Node fourni avec un plugin. La forme exec passe le chemin du script résolu comme un argument sans guillemets :

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

La forme shell équivalente a besoin de guillemets pour gérer les chemins avec des espaces ou des caractères spéciaux :

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

Les deux formes supportent les mêmes [placeholders de chemin](#reference-scripts-by-path), et les deux les exportent comme variables d'environnement `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT` et `CLAUDE_PLUGIN_DATA` sur le processus lancé, donc un script peut lire `process.env.CLAUDE_PLUGIN_ROOT` indépendamment de la façon dont il a été lancé.

Les hooks de plugin substituent également les valeurs [`${user_config.*}`](/docs/fr/plugins-reference#user-configuration), en forme exec uniquement : la valeur est substituée dans `command` et dans chaque élément `args` comme une chaîne brute, donc aucun shell ne la réanalyse.

Un hook de plugin en forme shell dont la `command` référence `${user_config.*}` échoue avec une [erreur](/docs/fr/errors#plugin-command-references-user-config) au lieu de s'exécuter. Pour utiliser une valeur d'option à partir d'un hook en forme shell, lisez la variable d'environnement `$CLAUDE_PLUGIN_OPTION_<KEY>`, comme `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` pour une option `webhook_url`, ou définissez `args` pour basculer le hook en forme exec. Avant v2.1.207, les commandes de hook de plugin en forme shell substituaient également `${user_config.*}`.

<Note>
  En forme exec, `command` est uniquement le nom ou le chemin de l'exécutable. Si `command` est un nom nu sans séparateur de chemin et contient des espaces aux côtés de `args`, Claude Code enregistre un avertissement car le lancement échouera : il n'y a pas d'exécutable nommé `node script.js`. Déplacez les tokens supplémentaires dans `args`. Les chemins absolus avec des espaces, tels que `C:\Program Files\nodejs\node.exe`, sont un seul exécutable valide et ne déclenchent pas l'avertissement.
</Note>

<h4 id="http-hook-fields">
  Champs des hooks HTTP
</h4>

En plus des [champs communs](#common-fields), les hooks HTTP acceptent ces champs :

| Champ            | Requis | Description                                                                                                                                                                                                                                                 |
| :--------------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`            | oui    | URL vers laquelle envoyer la requête POST                                                                                                                                                                                                                   |
| `headers`        | non    | En-têtes HTTP supplémentaires sous forme de paires clé-valeur. Les valeurs supportent l'interpolation de variables d'environnement en utilisant la syntaxe `$VAR_NAME` ou `${VAR_NAME}`. Seules les variables listées dans `allowedEnvVars` sont résolues   |
| `allowedEnvVars` | non    | Liste des noms de variables d'environnement qui peuvent être interpolés dans les valeurs d'en-tête. Les références aux variables non listées sont remplacées par des chaînes vides. Requis pour que l'interpolation de variables d'environnement fonctionne |

Claude Code envoie l'[entrée JSON](#hook-input-and-output) du hook en tant que corps de la requête POST avec `Content-Type: application/json`. Le corps de la réponse utilise le même [format de sortie JSON](#json-output) que les hooks de commande.

La gestion des erreurs diffère des hooks de commande : les réponses non-2xx, les défaillances de connexion et les délais d'expiration produisent tous des erreurs non-bloquantes qui permettent à l'exécution de continuer. Pour bloquer un appel d'outil ou refuser une permission, retournez une réponse 2xx avec un corps JSON contenant `decision: "block"` ou un `hookSpecificOutput` avec `permissionDecision: "deny"`.

Cet exemple envoie les événements `PreToolUse` à un service de validation local, en s'authentifiant avec un token de la variable d'environnement `MY_TOKEN` :

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/pre-tool-use",
            "timeout": 30,
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

<h4 id="mcp-tool-hook-fields">
  Champs des hooks de l'outil MCP
</h4>

En plus des [champs communs](#common-fields), les hooks de l'outil MCP acceptent ces champs :

| Champ    | Requis | Description                                                                                                                                                                                                                                                                                                                   |
| :------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | oui    | Nom d'un serveur MCP configuré. Pour un [serveur fourni par un plugin](/docs/fr/mcp#plugin-provided-mcp-servers), c'est le nom limité `plugin:<plugin-name>:<server-name>`, comme `plugin:my-plugin:db`, pas la clé de serveur nue. Le serveur doit déjà être connecté ; le hook ne déclenche jamais un flux OAuth ou de connexion |
| `tool`   | oui    | Nom de l'outil à appeler sur ce serveur                                                                                                                                                                                                                                                                                       |
| `input`  | non    | Arguments passés à l'outil. Les valeurs de chaîne supportent la substitution `${path}` de l'[entrée JSON](#hook-input-and-output) du hook, comme `"${tool_input.file_path}"`                                                                                                                                                  |

La sortie textuelle de l'outil est traitée comme stdout d'un hook de commande : si elle s'analyse comme une [sortie JSON](#json-output) valide, elle est traitée comme une décision, sinon elle est affichée en tant que texte brut. Si le serveur nommé n'est pas connecté, ou si l'outil retourne `isError: true`, le hook produit une erreur non-bloquante et l'exécution continue.

Les hooks de l'outil MCP sont disponibles sur chaque événement de hook une fois que Claude Code s'est connecté à vos serveurs MCP. `SessionStart` et `Setup` se déclenchent généralement avant que les serveurs ne finissent de se connecter, donc les hooks sur ces événements doivent s'attendre à l'erreur « non connecté » à la première exécution.

Cet exemple appelle l'outil `security_scan` sur le serveur MCP `my_server` après chaque `Write` ou `Edit`, en passant le chemin du fichier édité :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "security_scan",
            "input": { "file_path": "${tool_input.file_path}" }
          }
        ]
      }
    ]
  }
}
```

<h4 id="prompt-and-agent-hook-fields">
  Champs des hooks de prompt et d'agent
</h4>

En plus des [champs communs](#common-fields), les hooks de prompt et d'agent acceptent ces champs :

| Champ    | Requis | Description                                                                                                                                                                                                        |
| :------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt` | oui    | Texte du prompt à envoyer au modèle. Utilisez `$ARGUMENTS` comme placeholder pour l'entrée JSON du hook. Échappez avec une barre oblique inverse pour inclure du texte littéral : `\$1.00` s'affiche comme `$1.00` |
| `model`  | non    | Modèle à utiliser pour l'évaluation. Par défaut un modèle rapide                                                                                                                                                   |

<h3 id="reference-scripts-by-path">
  Référencer les scripts par chemin
</h3>

Utilisez ces placeholders pour référencer les scripts de hook par rapport à la racine du projet ou du plugin, indépendamment du répertoire de travail lorsque le hook s'exécute :

* `${CLAUDE_PROJECT_DIR}` : la racine du projet. Claude Code définit également cette variable dans l'environnement des [serveurs MCP stdio](/docs/fr/mcp#option-3-add-a-local-stdio-server) et des serveurs LSP de plugin.
* `${CLAUDE_PLUGIN_ROOT}` : le répertoire d'installation du plugin, pour les scripts fournis avec un [plugin](/docs/fr/plugins). Change à chaque mise à jour du plugin.
* `${CLAUDE_PLUGIN_DATA}` : le [répertoire de données persistantes](/docs/fr/plugins-reference#persistent-data-directory) du plugin, pour les dépendances et l'état qui doivent survivre aux mises à jour du plugin.

Préférez la [forme exec](#exec-form-and-shell-form) pour tout hook qui référence un placeholder de chemin. La forme exec passe chaque élément `args` comme un argument sans tokenisation shell, donc les chemins avec des espaces ou des caractères spéciaux n'ont besoin d'aucun guillemet. En forme shell, enveloppez chaque placeholder entre guillemets doubles.

<Tabs>
  <Tab title="Scripts de projet">
    Cet exemple utilise `${CLAUDE_PROJECT_DIR}` pour exécuter un vérificateur de style à partir du répertoire `.claude/hooks/` du projet après tout appel d'outil `Write` ou `Edit` :

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Scripts de plugin">
    Définissez les hooks de plugin dans `hooks/hooks.json` avec un champ `description` optionnel au niveau supérieur. Lorsqu'un plugin est activé, ses hooks fusionnent avec vos hooks utilisateur et projet.

    Cet exemple exécute un script de formatage fourni avec le plugin :

    ```json theme={null}
    {
      "description": "Automatic code formatting",
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
                "args": [],
                "timeout": 30
              }
            ]
          }
        ]
      }
    }
    ```

    Consultez la [référence des composants de plugin](/docs/fr/plugins-reference#hooks) pour plus de détails sur la création de hooks de plugin.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks dans les skills et agents
</h3>

En plus des fichiers de paramètres et des plugins, les hooks peuvent être définis directement dans les [skills](/docs/fr/skills) et les [subagents](/docs/fr/sub-agents) en utilisant le frontmatter. Ces hooks sont limités au cycle de vie du composant et ne s'exécutent que lorsque ce composant est actif.

Tous les événements de hook sont supportés. Pour les subagents, les hooks `Stop` sont automatiquement convertis en `SubagentStop` puisque c'est l'événement qui se déclenche lorsqu'un subagent se termine.

Les hooks utilisent le même format de configuration que les hooks basés sur les paramètres mais sont limités à la durée de vie du composant et nettoyés lorsqu'il se termine.

Ce skill définit un hook `PreToolUse` qui exécute un script de validation de sécurité avant chaque commande `Bash` :

```yaml theme={null}
---
name: secure-operations
description: Perform operations with security checks
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/security-check.sh"
---
```

Les agents utilisent le même format dans leur frontmatter YAML.

<h3 id="the-/hooks-menu">
  Le menu `/hooks`
</h3>

Tapez `/hooks` dans Claude Code pour ouvrir un navigateur en lecture seule pour vos hooks configurés. Le menu affiche chaque événement de hook avec un nombre de hooks configurés, vous permet d'explorer les matchers et affiche les détails complets de chaque gestionnaire de hook. Utilisez-le pour vérifier la configuration, vérifier à partir de quel fichier de paramètres un hook provient ou inspecter la commande, le prompt ou l'URL d'un hook.

Le menu affiche les cinq types de hooks : `command`, `prompt`, `agent`, `http` et `mcp_tool`. Chaque hook est étiqueté avec un préfixe `[type]` et une source indiquant où il a été défini :

* `User` : de `~/.claude/settings.json`
* `Project` : de `.claude/settings.json`
* `Local` : de `.claude/settings.local.json`
* `Plugin` : du `hooks/hooks.json` d'un plugin
* `Session` : enregistré en mémoire pour la session actuelle
* `Built-in` : enregistré en interne par Claude Code

Sélectionner un hook ouvre une vue détaillée affichant son événement, son matcher, son type, son fichier source et la commande, le prompt ou l'URL complet. Le menu est en lecture seule : pour ajouter, modifier ou supprimer des hooks, éditez directement le JSON des paramètres ou demandez à Claude de faire la modification.

<h3 id="disable-or-remove-hooks">
  Désactiver ou supprimer les hooks
</h3>

Pour supprimer un hook, supprimez son entrée du fichier de paramètres JSON.

Pour désactiver temporairement tous les hooks sans les supprimer, définissez `"disableAllHooks": true` dans votre fichier de paramètres. Il n'y a aucun moyen de désactiver un hook individuel tout en le gardant dans la configuration.

Le paramètre `disableAllHooks` respecte la hiérarchie des paramètres gérés. Si un administrateur a configuré des hooks via les paramètres de politique gérée, `disableAllHooks` défini dans les paramètres utilisateur, projet ou local ne peut pas désactiver ces hooks gérés. Seul `disableAllHooks` défini au niveau des paramètres gérés peut désactiver les hooks gérés.

Les éditions directes des hooks dans les fichiers de paramètres sont normalement détectées automatiquement par le moniteur de fichiers.

<h2 id="hook-input-and-output">
  Entrée et sortie des hooks
</h2>

Les hooks de commande reçoivent les données JSON via stdin et communiquent les résultats via les codes de sortie, stdout et stderr. Les hooks HTTP reçoivent le même JSON que le corps de la requête POST et communiquent les résultats via le corps de la réponse HTTP. Cette section couvre les champs et le comportement communs à tous les événements. Chaque section d'événement sous [Événements de hook](#hook-events) inclut son schéma d'entrée spécifique et les options de contrôle de décision.

Sur macOS et Linux, les hooks de commande s'exécutent dans leur propre session sans terminal de contrôle à partir de v2.1.139. Le processus de hook et tous les processus enfants ne peuvent pas ouvrir `/dev/tty` ou envoyer des séquences d'échappement directement à l'interface Claude Code. Windows n'a pas de `/dev/tty`. Pour afficher un message à l'utilisateur sur n'importe quelle plateforme, retournez [`systemMessage`](#json-output) dans la sortie JSON. Pour déclencher une notification de bureau, définir un titre de fenêtre ou sonner la cloche, retournez [`terminalSequence`](#emit-terminal-notifications) à la place.

<h3 id="common-input-fields">
  Champs d'entrée communs
</h3>

Les événements de hook reçoivent ces champs en JSON, en plus des champs spécifiques à l'événement documentés dans chaque section [événement de hook](#hook-events). Pour les hooks de commande, ce JSON arrive via stdin. Pour les hooks HTTP, il arrive dans le corps de la requête POST.

| Champ             | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | Identifiant de session actuel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `prompt_id`       | UUID identifiant le prompt utilisateur actuellement traité. Correspond à l'[attribut `prompt.id` sur les événements OpenTelemetry](/docs/fr/monitoring-usage#event-correlation-attributes), afin que vous puissiez corréler la sortie du hook avec la télémétrie pour un seul prompt. Absent jusqu'à la première entrée utilisateur. Nécessite Claude Code v2.1.196 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `transcript_path` | Chemin vers le JSON de conversation. Le fichier de transcription est écrit de manière asynchrone et peut être en retard par rapport à la conversation en mémoire, il se peut donc qu'il n'inclue pas encore les messages les plus récents du tour actuel lorsqu'un hook se déclenche. Les hooks qui ont besoin du texte final de l'assistant du tour actuel doivent utiliser `last_assistant_message` sur [Stop](#stop) et [SubagentStop](#subagentstop) au lieu de lire la transcription                                                                                                                                                                                                                                                                                                                                              |
| `cwd`             | Répertoire de travail courant lorsque le hook est invoqué                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `permission_mode` | [Mode de permission](/docs/fr/permissions#permission-modes) actuel : `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` ou `"bypassPermissions"`. Le mode étiqueté **Manuel** arrive comme `"default"`, jamais comme `"manual"`, afin que les scripts qui correspondent à `"default"` continuent de fonctionner. Tous les événements ne reçoivent pas ce champ. Consultez l'exemple JSON de chaque [événement de hook](#hook-events)                                                                                                                                                                                                                                                                                                                                                                                             |
| `effort`          | Objet avec un champ `level` contenant le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) actif pour le tour : `"low"`, `"medium"`, `"high"`, `"xhigh"` ou `"max"`. Si l'effort demandé du modèle dépasse ce que le modèle actuel supporte, c'est le niveau réduit que le modèle a réellement utilisé. Ultracode n'est pas un niveau distinct et est signalé comme `"xhigh"`. L'objet correspond au champ `effort` de la [ligne de statut](/docs/fr/statusline#available-data). Présent pour les événements qui se déclenchent dans un contexte d'utilisation d'outil, tels que `PreToolUse`, `PostToolUse`, `Stop` et `SubagentStop`, lorsque le modèle actuel supporte le paramètre d'effort. Le niveau est également disponible pour les commandes de hook et l'outil Bash en tant que variable d'environnement `$CLAUDE_EFFORT`. |
| `hook_event_name` | Nom de l'événement qui s'est déclenché                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

Lors de l'exécution avec `--agent` ou à l'intérieur d'un subagent, deux champs supplémentaires sont inclus :

| Champ        | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `agent_id`   | Identifiant unique pour le subagent. Présent uniquement lorsque le hook se déclenche à l'intérieur d'un appel de subagent. Utilisez ceci pour distinguer les appels de hook de subagent des appels du thread principal.                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agent_type` | Nom de l'agent (par exemple, `"Explore"` ou `"security-reviewer"`). Présent lorsque la session utilise `--agent` ou que le hook se déclenche à l'intérieur d'un subagent. Pour les subagents, le type du subagent prend précédence sur la valeur `--agent` de la session. Pour les [subagents personnalisés](/docs/fr/sub-agents), c'est le champ `name` du frontmatter de l'agent, pas le nom du fichier. Pour les subagents fournis par un [plugin](/docs/fr/plugins), c'est l'identifiant scoped du plugin tel que `my-plugin:reviewer`, pas le nom du frontmatter nu. Consultez [SubagentStart](#subagentstart) pour savoir comment écrire un matcher contre un nom scoped du plugin. |

Seuls les hooks [`SessionStart`](#sessionstart) reçoivent un champ `model`, et sa présence n'est pas garantie. Il n'y a pas de variable d'environnement `$CLAUDE_MODEL`. Un processus de hook hérite de l'environnement parent, il peut donc lire `$ANTHROPIC_MODEL` si vous le définissez dans votre shell, mais cette valeur ne change pas lorsque vous changez de modèle avec `/model` pendant une session. Un ensemble de variables n'est pas hérité : Claude Code [supprime les variables d'exportateur `OTEL_*` de chaque sous-processus qu'il génère](/docs/fr/monitoring-usage#administrator-configuration), y compris les hooks.

Par exemple, un hook `PreToolUse` pour une commande Bash reçoit ceci sur stdin :

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

Les champs `tool_name` et `tool_input` sont spécifiques à l'événement. Chaque section [événement de hook](#hook-events) documente les champs supplémentaires pour cet événement.

<h3 id="exit-code-output">
  Sortie du code de sortie
</h3>

Le code de sortie de votre commande de hook indique à Claude Code si l'action doit procéder, être bloquée ou être ignorée.

**Exit 0** signifie succès. Claude Code analyse stdout pour les [champs de sortie JSON](#json-output). La sortie JSON n'est traitée que sur exit 0. Pour la plupart des événements, stdout est écrit dans le journal de débogage mais n'est pas affiché dans la transcription. Les exceptions sont `UserPromptSubmit`, `UserPromptExpansion` et `SessionStart`, où stdout est ajouté comme contexte que Claude peut voir et sur lequel agir.

**Exit 2** signifie une erreur bloquante. Claude Code ignore stdout et tout JSON qu'il contient. À la place, le texte stderr est renvoyé à Claude comme message d'erreur. L'effet dépend de l'événement : `PreToolUse` bloque l'appel d'outil, `UserPromptSubmit` rejette le prompt, et ainsi de suite. Consultez [comportement du code de sortie 2](#exit-code-2-behavior-per-event) pour la liste complète.

**Tout autre code de sortie** est une erreur non-bloquante pour la plupart des événements de hook. La transcription affiche un avis `<hook name> hook error` suivi de la première ligne de stderr, afin que vous puissiez identifier la cause sans `--debug`. L'exécution continue et le stderr complet est écrit dans le journal de débogage.

Par exemple, un script de commande de hook qui bloque les commandes Bash dangereuses :

```bash theme={null}
#!/bin/bash
# Lit l'entrée JSON depuis stdin, vérifie la commande
command=$(jq -r '.tool_input.command' < /dev/stdin)

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Erreur bloquante : l'appel d'outil est empêché
fi

exit 0  # Pas de décision : le flux de permission normal s'applique
```

<Warning>
  Pour la plupart des événements de hook, seul le code de sortie 2 bloque l'action. Claude Code traite le code de sortie 1 comme une erreur non-bloquante et procède avec l'action, même si 1 est le code d'échec Unix conventionnel. Si votre hook est destiné à appliquer une politique, utilisez `exit 2`. L'exception est `WorktreeCreate`, où tout code de sortie non-zéro abandonne la création du worktree.
</Warning>

<h4 id="exit-code-2-behavior-per-event">
  Comportement du code de sortie 2 par événement
</h4>

Le code de sortie 2 est la façon dont un hook signale « arrêtez, ne faites pas cela ». L'effet dépend de l'événement, car certains événements représentent des actions qui peuvent être bloquées (comme un appel d'outil qui ne s'est pas encore produit) et d'autres représentent des choses qui se sont déjà produites ou ne peuvent pas être empêchées.

| Événement de hook     | Peut bloquer ? | Ce qui se passe sur exit 2                                                                                                                                        |
| :-------------------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Oui            | Bloque l'appel d'outil                                                                                                                                            |
| `PermissionRequest`   | Oui            | Refuse la permission                                                                                                                                              |
| `UserPromptSubmit`    | Oui            | Bloque le traitement du prompt et efface le prompt                                                                                                                |
| `UserPromptExpansion` | Oui            | Bloque l'expansion                                                                                                                                                |
| `Stop`                | Oui            | Empêche Claude de s'arrêter, continue la conversation                                                                                                             |
| `SubagentStop`        | Oui            | Empêche le subagent de s'arrêter                                                                                                                                  |
| `TeammateIdle`        | Oui            | Empêche le coéquipier de devenir inactif, le coéquipier continue de travailler                                                                                    |
| `TaskCreated`         | Oui            | Annule la création de la tâche                                                                                                                                    |
| `TaskCompleted`       | Oui            | Empêche la tâche d'être marquée comme complétée                                                                                                                   |
| `ConfigChange`        | Oui            | Bloque la modification de configuration de prendre effet (sauf `policy_settings`)                                                                                 |
| `StopFailure`         | Non            | La sortie et le code de sortie sont ignorés                                                                                                                       |
| `PostToolUse`         | Non            | Affiche stderr à Claude ; l'outil a déjà s'exécuté                                                                                                                |
| `PostToolUseFailure`  | Non            | Affiche stderr à Claude ; l'outil a déjà échoué                                                                                                                   |
| `PostToolBatch`       | Oui            | Arrête la boucle agentique avant l'appel du modèle suivant                                                                                                        |
| `PermissionDenied`    | Non            | Le code de sortie et stderr sont ignorés car le refus a déjà eu lieu. Utilisez JSON `hookSpecificOutput.retry: true` pour indiquer au modèle qu'il peut réessayer |
| `Notification`        | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                         |
| `SubagentStart`       | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                         |
| `SessionStart`        | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                         |
| `Setup`               | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                         |
| `SessionEnd`          | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                         |
| `CwdChanged`          | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                         |
| `FileChanged`         | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                         |
| `PreCompact`          | Oui            | Bloque la compaction                                                                                                                                              |
| `PostCompact`         | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                         |
| `Elicitation`         | Oui            | Refuse l'élicitation                                                                                                                                              |
| `ElicitationResult`   | Oui            | Bloque la réponse (l'action devient decline)                                                                                                                      |
| `WorktreeCreate`      | Oui            | Tout code de sortie non-zéro provoque l'échec de la création du worktree                                                                                          |
| `WorktreeRemove`      | Non            | Les défaillances sont enregistrées en mode debug uniquement                                                                                                       |
| `InstructionsLoaded`  | Non            | Le code de sortie est ignoré                                                                                                                                      |
| `MessageDisplay`      | Non            | Le texte original est affiché                                                                                                                                     |

Pour `SessionStart`, `Setup` et `SubagentStart`, le stderr du code de sortie 2 s'affiche dans la transcription comme un avis `<hook name> hook error`, de la même manière qu'une [erreur non-bloquante](#exit-code-output). Claude ne le voit pas, et la session ou le subagent procède. Pour `SubagentStart`, l'avis apparaît dans la propre transcription du subagent, pas dans la conversation parent.

À partir de Claude Code v2.1.199, `SessionStart`, `Setup` et `SubagentStart` affichent le stderr du code de sortie 2 dans la transcription. Les versions antérieures l'écrivaient uniquement dans le journal de débogage.

<h3 id="http-response-handling">
  Gestion des réponses HTTP
</h3>

Les hooks HTTP utilisent les codes de statut HTTP et les corps de réponse au lieu des codes de sortie et stdout :

* **2xx avec un corps vide** : succès, équivalent à exit code 0 sans sortie
* **2xx avec un corps en texte brut** : succès, le texte est ajouté comme contexte
* **2xx avec un corps JSON** : succès, analysé en utilisant le même schéma [sortie JSON](#json-output) que les hooks de commande
* **Statut non-2xx** : erreur non-bloquante, l'exécution continue
* **Défaillance de connexion ou délai d'expiration** : erreur non-bloquante, l'exécution continue

Contrairement aux hooks de commande, les hooks HTTP ne peuvent pas signaler une erreur bloquante uniquement via les codes de statut. Pour bloquer un appel d'outil ou refuser une permission, retournez une réponse 2xx avec un corps JSON contenant les champs de décision appropriés.

<h3 id="json-output">
  Sortie JSON
</h3>

Les codes de sortie vous permettent d'autoriser ou de bloquer, mais la sortie JSON vous donne un contrôle plus granulaire. Au lieu de quitter avec le code 2 pour bloquer, quittez 0 et imprimez un objet JSON sur stdout. Claude Code lit les champs spécifiques de ce JSON pour contrôler le comportement, y compris [contrôle de décision](#decision-control) pour bloquer, autoriser ou escalader à l'utilisateur.

<Note>
  Vous devez choisir une approche par hook, pas les deux : soit utiliser les codes de sortie seuls pour signaler, soit quitter 0 et imprimer JSON pour un contrôle structuré. Claude Code ne traite JSON que sur exit 0. Si vous quittez 2, tout JSON est ignoré.
</Note>

La sortie stdout de votre hook doit contenir uniquement l'objet JSON. Si votre profil shell imprime du texte au démarrage, cela peut interférer avec l'analyse JSON. Consultez [Validation JSON échouée](/docs/fr/hooks-guide#json-validation-failed) dans le guide de dépannage.

Les chaînes de sortie du hook, y compris `additionalContext`, `systemMessage` et stdout brut, sont plafonnées à 10 000 caractères. La sortie qui dépasse cette limite est enregistrée dans un fichier et remplacée par un aperçu et un chemin de fichier, de la même manière que les grands résultats d'outils sont gérés.

L'objet JSON supporte trois types de champs :

* **Champs universels** comme `continue` fonctionnent sur tous les événements. Ceux-ci sont listés dans le tableau ci-dessous.
* **`decision` et `reason` au niveau supérieur** sont utilisés par certains événements pour bloquer ou fournir des commentaires.
* **`hookSpecificOutput`** est un objet imbriqué pour les événements qui ont besoin d'un contrôle plus riche. Il nécessite un champ `hookEventName` défini au nom de l'événement.

| Champ              | Par défaut | Description                                                                                                                                                                                                                                                                                                                                                                              |
| :----------------- | :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`     | Si `false`, Claude arrête complètement le traitement après l'exécution du hook. Prend précédence sur tous les champs de décision spécifiques à l'événement                                                                                                                                                                                                                               |
| `stopReason`       | aucun      | Message affiché à l'utilisateur lorsque `continue` est `false`. Non affiché à Claude                                                                                                                                                                                                                                                                                                     |
| `suppressOutput`   | `false`    | Si `true`, masque stdout du hook de la transcription. Stdout apparaît toujours dans le journal de débogage                                                                                                                                                                                                                                                                               |
| `systemMessage`    | aucun      | Message d'avertissement affiché à l'utilisateur                                                                                                                                                                                                                                                                                                                                          |
| `terminalSequence` | aucun      | Une séquence d'échappement de terminal pour Claude Code d'émettre en votre nom, comme une notification de bureau, un titre de fenêtre ou une cloche. Restreint aux OSC `0`/`1`/`2`/`9`/`99`/`777` et BEL. Si la valeur contient quelque chose en dehors de la liste blanche, le champ est ignoré. Utilisez ceci au lieu d'écrire sur `/dev/tty`, qui n'est pas disponible pour les hooks |

Pour arrêter Claude entièrement indépendamment du type d'événement :

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

<h4 id="emit-terminal-notifications">
  Émettre des notifications de terminal
</h4>

Le champ `terminalSequence` nécessite Claude Code v2.1.141 ou ultérieur.

Les hooks s'exécutent sans terminal de contrôle, donc écrire des séquences d'échappement directement sur `/dev/tty` échoue. À la place, retournez la séquence d'échappement dans le champ `terminalSequence` et Claude Code l'émet pour vous via son propre chemin d'écriture de terminal. C'est sans course, fonctionne à l'intérieur de tmux et GNU screen, et fonctionne sur Windows où il n'y a pas de `/dev/tty`.

Le champ accepte une chaîne d'une ou plusieurs séquences d'échappement en liste blanche :

* OSC `0`, `1`, `2` : titres de fenêtre et d'icône
* OSC `9` : notifications iTerm2, ConEmu, Windows Terminal et WezTerm, y compris la progression de la barre des tâches `9;4`
* OSC `99` : notifications Kitty
* OSC `777` : notifications urxvt, Ghostty et Warp
* BEL nu

Les séquences peuvent être terminées avec BEL ou avec ST. Tout ce qui est en dehors de la liste blanche, y compris les séquences de curseur CSI et les séquences de couleur, les séquences de palette OSC, les hyperliens OSC 8, les écritures de presse-papiers OSC 52 et OSC 1337, est rejeté et le champ est ignoré.

L'exemple ci-dessous déclenche une notification de bureau à partir d'un hook `Notification`. La séquence d'échappement est construite avec des échappements octaux `printf` afin que les octets de contrôle n'apparaissent jamais sur la ligne de commande shell, et `jq -n --arg` construit la sortie JSON afin que les guillemets, les barres obliques inverses et les sauts de ligne dans le message de notification soient correctement échappés :

```bash theme={null}
#!/bin/bash
# Hook de notification : ping le bureau lorsque Claude Code a besoin d'attention.
input=$(cat)
title="Claude Code'
body=$(jq -r '.message // 'Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

La forme `{ "terminalSequence": "..." }` est la même à partir de n'importe quel shell ou langage. Sur Windows, construisez la chaîne d'échappement dans PowerShell ou un script et émettez le même objet JSON.

<Note>
  `terminalSequence` est le remplacement pris en charge pour les hooks qui écrivaient précédemment des séquences d'échappement directement sur `/dev/tty`. La liste blanche est restreinte aux séquences qui ne peuvent pas déplacer le curseur ou modifier les couleurs, afin qu'un hook ne puisse jamais corrompre une invite à l'écran.
</Note>

<h4 id="add-context-for-claude">
  Ajouter du contexte pour Claude
</h4>

Le champ `additionalContext` transmet une chaîne de votre hook dans la fenêtre de contexte de Claude. Claude Code enveloppe la chaîne dans un rappel système et l'insère dans la conversation au point où le hook s'est déclenché. Claude lit le rappel lors de la prochaine demande du modèle, mais il n'apparaît pas comme un message de chat dans l'interface.

Retournez `additionalContext` à l'intérieur de `hookSpecificOutput` aux côtés du nom de l'événement :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "This file is generated. Edit src/schema.ts and run `bun generate` instead."
  }
}
```

L'endroit où le rappel apparaît dépend de l'événement :

* [SessionStart](#sessionstart), [Setup](#setup) et [SubagentStart](#subagentstart) : au début de la conversation, avant le premier prompt
* [UserPromptSubmit](#userpromptsubmit) et [UserPromptExpansion](#userpromptexpansion) : aux côtés du prompt soumis
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) et [PostToolBatch](#posttoolbatch) : à côté du résultat de l'outil
* [Stop](#stop) et [SubagentStop](#subagentstop) : à la fin du tour. La conversation continue afin que Claude puisse agir sur les commentaires. Consultez [Contrôle de décision Stop](#stop-decision-control)

Lorsque plusieurs hooks retournent `additionalContext` pour le même événement, Claude reçoit toutes les valeurs. Si une valeur dépasse 10 000 caractères, Claude Code écrit le texte complet dans un fichier du répertoire de session et transmet à Claude le chemin du fichier avec un court aperçu à la place.

Utilisez `additionalContext` pour les informations que Claude devrait connaître sur l'état actuel de votre environnement ou l'opération qui vient de s'exécuter :

* **État de l'environnement** : la branche actuelle, la cible de déploiement ou les drapeaux de fonctionnalité actifs
* **Règles de projet conditionnelles** : quelle commande de test s'applique au fichier qui vient d'être modifié, quels répertoires sont en lecture seule dans ce worktree
* **Données externes** : problèmes ouverts qui vous sont assignés, résultats CI récents, contenu récupéré à partir d'un service interne

Pour les instructions qui ne changent jamais, préférez [CLAUDE.md](/docs/fr/memory). Il se charge sans exécuter de script et est l'endroit standard pour les conventions de projet statiques.

Écrivez le texte sous forme de déclarations factuelles plutôt que d'instructions système impératives. Des formulations telles que « La cible de déploiement est production » ou « Ce repo utilise `bun test` » se lisent comme des informations de projet. Le texte encadré comme des commandes système hors bande peut déclencher les défenses contre l'injection de prompt de Claude, ce qui amène Claude à vous présenter le texte au lieu de le traiter comme du contexte.

Une fois injecté, le texte est enregistré dans la transcription de session. Pour les événements mid-session comme `PostToolUse` ou `UserPromptSubmit`, la reprise avec `--continue` ou `--resume` rejoue le texte enregistré plutôt que de réexécuter le hook pour les tours passés, de sorte que les valeurs comme les horodatages ou les SHA de commit deviennent obsolètes à la reprise. Les hooks `SessionStart` s'exécutent à nouveau à la reprise avec `source` défini sur `"resume"`, afin qu'ils puissent actualiser leur contexte.

<h4 id="decision-control">
  Contrôle de décision
</h4>

Tous les événements ne supportent pas le blocage ou le contrôle du comportement via JSON. Les événements qui le font utilisent chacun un ensemble différent de champs pour exprimer cette décision. Utilisez ce tableau comme référence rapide avant d'écrire un hook :

| Événements                                                                                                                          | Modèle de décision                  | Champs clés                                                                                                                                                                                                                                             |
| :---------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | `decision` au niveau supérieur      | `decision: "block"`, `reason`. Stop et SubagentStop acceptent également `hookSpecificOutput.additionalContext` pour [les commentaires non-erreur qui continuent la conversation](#stop-decision-control)                                                |
| TeammateIdle, TaskCreated, TaskCompleted                                                                                            | Code de sortie ou `continue: false` | Le code de sortie 2 bloque l'action avec commentaires stderr. JSON `{"continue": false, "stopReason": "..."}` arrête également complètement le coéquipier, correspondant au comportement du hook `Stop`                                                 |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                                 |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                | `decision.behavior` (allow/deny)                                                                                                                                                                                                                        |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                | `retry: true` indique au modèle qu'il peut réessayer l'appel d'outil refusé                                                                                                                                                                             |
| WorktreeCreate                                                                                                                      | retour de chemin                    | Le hook de commande imprime le chemin sur stdout ; le hook HTTP retourne `hookSpecificOutput.worktreePath`. L'échec du hook ou l'absence de chemin échoue la création                                                                                   |
| Elicitation                                                                                                                         | `hookSpecificOutput`                | `action` (accept/decline/cancel), `content` (valeurs des champs de formulaire pour accept)                                                                                                                                                              |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                | `action` (accept/decline/cancel), `content` (valeurs des champs de formulaire override)                                                                                                                                                                 |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                | `displayContent` remplace le texte affiché à l'écran. Affichage uniquement : la transcription et ce que Claude voit conservent l'original                                                                                                               |
| SessionStart, Setup, SubagentStart                                                                                                  | Contexte uniquement                 | `hookSpecificOutput.additionalContext` ajoute du contexte pour Claude. SessionStart accepte également [`initialUserMessage`, `watchPaths`, `sessionTitle` et `reloadSkills`](#sessionstart-decision-control). Pas de blocage ou de contrôle de décision |
| WorktreeRemove, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, FileChanged                     | Aucun                               | Pas de contrôle de décision. Utilisé pour les effets secondaires comme la journalisation ou le nettoyage                                                                                                                                                |

Quelques événements peuvent également réécrire le contenu plutôt que seulement l'autoriser ou le bloquer :

* `PreToolUse` : `updatedInput` directement sous `hookSpecificOutput` remplace les arguments d'un outil avant son exécution. Consultez [Contrôle de décision PreToolUse](#pretooluse-decision-control)
* `PermissionRequest` : `updatedInput` à l'intérieur de l'objet `decision`. Consultez [Contrôle de décision PermissionRequest](#permissionrequest-decision-control)
* `PostToolUse` : `updatedToolOutput` remplace le résultat de l'outil. Consultez [Contrôle de décision PostToolUse](#posttooluse-decision-control)
* `UserPromptSubmit` : ne peut pas remplacer le prompt ; injecte uniquement `additionalContext` à côté de celui-ci

Pour les cas d'usage de rédaction ou de transformation, interceptez à `PreToolUse` pour les entrées d'outil sortantes et `PostToolUse` pour les résultats d'outil entrants.

Voici des exemples de chaque modèle en action :

<Tabs>
  <Tab title="Décision au niveau supérieur">
    Utilisé par `UserPromptSubmit`, `UserPromptExpansion`, `PostToolUse`, `PostToolUseFailure`, `PostToolBatch`, `Stop`, `SubagentStop`, `ConfigChange` et `PreCompact`. La seule valeur est `"block"`. Pour autoriser l'action à procéder, omettez `decision` de votre JSON ou quittez 0 sans aucun JSON :

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Utilise `hookSpecificOutput` pour un contrôle plus riche : autoriser, refuser, ou escalader à l'utilisateur. Vous pouvez également modifier l'entrée de l'outil avant son exécution ou injecter du contexte supplémentaire pour Claude. Consultez [Contrôle de décision PreToolUse](#pretooluse-decision-control) pour l'ensemble complet des options.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Database writes are not allowed"
      }
    }
    ```
  </Tab>

  <Tab title="PermissionRequest">
    Utilise `hookSpecificOutput` pour autoriser ou refuser une demande de permission au nom de l'utilisateur. Lors de l'autorisation, vous pouvez également modifier l'entrée de l'outil ou appliquer des règles de permission afin que l'utilisateur ne soit pas invité à nouveau. Consultez [Contrôle de décision PermissionRequest](#permissionrequest-decision-control) pour l'ensemble complet des options.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "allow",
          "updatedInput": {
            "command": "npm run lint"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Pour des exemples étendus incluant la validation de commandes Bash, le filtrage de prompts et les scripts d'approbation automatique, consultez [Ce que vous pouvez automatiser](/docs/fr/hooks-guide#what-you-can-automate) dans le guide et la [implémentation de référence du validateur de commandes Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py).

<h2 id="hook-events">
  Événements de hook
</h2>

Chaque événement correspond à un point du cycle de vie de Claude Code où les hooks peuvent s'exécuter. Les sections ci-dessous sont ordonnées pour correspondre au cycle de vie : de la configuration de session à travers la boucle agentique jusqu'à la fin de session. Chaque section décrit quand l'événement se déclenche, quels matchers il supporte, l'entrée JSON qu'il reçoit et comment contrôler le comportement via la sortie.

<h3 id="sessionstart">
  SessionStart
</h3>

S'exécute lorsque Claude Code démarre une nouvelle session ou reprend une session existante. Utile pour charger le contexte de développement comme les problèmes existants ou les modifications récentes de votre codebase, ou pour configurer les variables d'environnement. Pour le contexte statique qui ne nécessite pas de script, utilisez [CLAUDE.md](/docs/fr/memory) à la place.

SessionStart s'exécute à chaque session, donc gardez ces hooks rapides. Seuls les hooks `type: "command"` et `type: "mcp_tool"` sont supportés.

La valeur du matcher correspond à la façon dont la session a été initiée :

| Matcher   | Quand il se déclenche                 |
| :-------- | :------------------------------------ |
| `startup` | Nouvelle session                      |
| `resume`  | `--resume`, `--continue` ou `/resume` |
| `clear`   | `/clear`                              |
| `compact` | Compaction automatique ou manuelle    |

<h4 id="sessionstart-input">
  Entrée SessionStart
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks SessionStart reçoivent `source` et optionnellement `model`, `agent_type` et `session_title` :

| Champ           | Description                                                                                                                                                                                                                                 |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `source`        | Comment la session a démarré : `"startup"` pour les nouvelles sessions, `"resume"` pour les sessions reprises, `"clear"` après `/clear` ou `"compact"` après compaction                                                                     |
| `model`         | L'identifiant du modèle actif. Il peut être omis, par exemple après `/clear` ou lorsqu'une session est restaurée via la récupération de conversation, donc vérifiez le champ avant de le lire                                               |
| `agent_type`    | Le nom de l'agent, présent lorsque vous démarrez Claude Code avec `claude --agent <name>`                                                                                                                                                   |
| `session_title` | Le titre de session actuel s'il est déjà défini, par exemple via `--name` ou `/rename`. Un hook qui émet `sessionTitle` peut vérifier `session_title` en premier pour éviter de remplacer un titre que l'utilisateur a défini explicitement |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "startup",
  "model": "claude-sonnet-5"
}
```

<h4 id="sessionstart-decision-control">
  Contrôle de décision SessionStart
</h4>

Tout texte que votre script de hook imprime sur stdout est ajouté comme contexte pour Claude. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, vous pouvez retourner ces champs spécifiques à l'événement :

| Champ                | Description                                                                                                                                                                                                                                                                                                                                                  |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Chaîne ajoutée au contexte de Claude au début de la conversation, avant le premier prompt. Consultez [Ajouter du contexte pour Claude](#add-context-for-claude) pour savoir comment le texte est livré et ce qu'il faut y mettre                                                                                                                             |
| `initialUserMessage` | Chaîne utilisée comme premier message utilisateur de la session. S'applique en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`, où elle devient le premier tour même si aucun prompt n'est fourni. Si un prompt est fourni, il suit comme le tour suivant. Contrairement à `additionalContext`, qui s'attache à un tour existant, ceci crée le tour |
| `sessionTitle`       | Définit le titre de la session, avec le même effet que `/rename`. Utilisez pour nommer les sessions automatiquement à partir du dossier de lancement, de la branche git ou du nom du worktree. S'applique uniquement lorsque `source` est `"startup"` ou `"resume"` ; ignoré sur `"clear"` et `"compact"`                                                    |
| `watchPaths`         | Tableau de chemins absolus à surveiller pour les événements [FileChanged](#filechanged) pendant cette session                                                                                                                                                                                                                                                |
| `reloadSkills`       | Booléen. Lorsque `true`, Claude Code réanalyse les répertoires [skill](/docs/fr/skills) et de commandes après que les hooks SessionStart se terminent, donc les skills que le hook a installées sont disponibles dans la même session, à partir du premier prompt                                                                                                 |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Puisque le stdout brut atteint déjà Claude pour cet événement, un hook qui charge uniquement du contexte peut imprimer sur stdout directement sans construire JSON. Utilisez le formulaire JSON lorsque vous avez besoin de combiner le contexte avec d'autres champs tels que `suppressOutput` ou `sessionTitle`.

Utilisez `reloadSkills` lorsqu'un hook SessionStart installe ou met à jour des skills. La découverte de skills s'exécute normalement avant que les hooks SessionStart se terminent, donc les fichiers que le hook écrit dans `~/.claude/skills/` ou `.claude/skills/` n'apparaîtraient autrement que dans la session suivante. Cet exemple synchronise un référentiel de skills partagé et demande la réanalyse :

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

<h4 id="persist-environment-variables">
  Persister les variables d'environnement
</h4>

Les hooks SessionStart ont accès à la variable d'environnement `CLAUDE_ENV_FILE`, qui fournit un chemin de fichier où vous pouvez persister les variables d'environnement pour les commandes Bash suivantes.

Pour définir des variables d'environnement individuelles, écrivez des déclarations `export` dans `CLAUDE_ENV_FILE`. Utilisez l'ajout (`>>`) pour préserver les variables définies par d'autres hooks :

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Pour capturer tous les changements d'environnement à partir des commandes de configuration, comparez les variables exportées avant et après :

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# Exécutez vos commandes de configuration qui modifient l'environnement
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Toutes les variables écrites dans ce fichier seront disponibles dans toutes les commandes Bash suivantes que Claude Code exécute pendant la session.

<Note>
  `CLAUDE_ENV_FILE` est disponible pour les hooks SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged) et [FileChanged](#filechanged). Les autres types de hooks n'ont pas accès à cette variable.
</Note>

<h3 id="setup">
  Setup
</h3>

Se déclenche uniquement lorsque vous lancez Claude Code avec `--init-only`, ou avec `--init` ou `--maintenance` en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`. Il ne se déclenche pas au démarrage normal. Utilisez-le pour l'installation de dépendances ponctuelles ou le nettoyage programmé que vous déclenchez explicitement à partir de CI ou de scripts, séparé du démarrage normal de session. Pour l'initialisation par session, utilisez [SessionStart](#sessionstart) à la place.

La valeur du matcher correspond au drapeau CLI qui a déclenché le hook :

| Matcher       | Quand il se déclenche                      |
| :------------ | :----------------------------------------- |
| `init`        | `claude --init-only` ou `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                  |

`--init-only` exécute les hooks Setup et les hooks SessionStart avec le matcher `startup`, puis quitte sans démarrer une conversation. `--init` et `--maintenance` déclenchent les hooks Setup uniquement lorsqu'ils sont combinés avec `-p` ; dans une session interactive, ces deux drapeaux ne déclenchent actuellement pas les hooks Setup.

Parce que Setup ne se déclenche pas à chaque lancement, un plugin qui a besoin d'une dépendance installée ne peut pas compter sur Setup seul. Le modèle pratique est de vérifier la dépendance à la première utilisation et d'installer en cas d'absence, par exemple un hook ou une skill qui teste `${CLAUDE_PLUGIN_DATA}/node_modules` et exécute `npm install` si absent. Consultez le [répertoire de données persistantes](/docs/fr/plugins-reference#persistent-data-directory) pour savoir où stocker les dépendances installées.

<h4 id="setup-input">
  Entrée Setup
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks Setup reçoivent un champ `trigger` défini à `"init"` ou `"maintenance"` :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Setup",
  "trigger": "init"
}
```

<h4 id="setup-decision-control">
  Contrôle de décision Setup
</h4>

Les hooks Setup ne peuvent pas bloquer. Tout code de sortie non-zéro, y compris 2, affiche stderr à l'utilisateur comme un avis `<hook name> hook error`, et l'exécution continue. En [mode non-interactif](/docs/fr/headless), la sortie du hook n'apparaît que lorsque vous lancez avec `--verbose`.

Pour transmettre des informations au contexte de Claude, retournez `additionalContext` dans la sortie JSON ; le stdout brut est écrit uniquement dans le journal de débogage. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, vous pouvez retourner ces champs spécifiques à l'événement :

| Champ               | Description                                                                           |
| :------------------ | :------------------------------------------------------------------------------------ |
| `additionalContext` | Chaîne ajoutée au contexte de Claude. Les valeurs de plusieurs hooks sont concaténées |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Setup",
    "additionalContext": "Dependencies installed: node_modules, .venv"
  }
}
```

Les hooks Setup ont accès à `CLAUDE_ENV_FILE`. Les variables écrites dans ce fichier persistent dans les commandes Bash suivantes pour la session, tout comme dans les [hooks SessionStart](#persist-environment-variables). Seuls les hooks `type: "command"` et `type: "mcp_tool"` sont supportés.

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Se déclenche lorsqu'un fichier `CLAUDE.md` ou `.claude/rules/*.md` est chargé dans le contexte. Cet événement se déclenche au démarrage de la session pour les fichiers chargés avec impatience et à nouveau plus tard lorsque les fichiers sont chargés avec paresse, par exemple lorsque Claude accède à un sous-répertoire qui contient un `CLAUDE.md` imbriqué ou lorsque les règles conditionnelles avec le frontmatter `paths:` correspondent. Le hook ne supporte pas le blocage ou le contrôle de décision. Il s'exécute de manière asynchrone à des fins d'observabilité.

Le matcher s'exécute sur `load_reason`. Par exemple, utilisez `"matcher": "session_start"` pour se déclencher uniquement pour les fichiers chargés au démarrage de la session, ou `"matcher": "path_glob_match|nested_traversal"` pour se déclencher uniquement pour les chargements paresseux.

<h4 id="instructionsloaded-input">
  Entrée InstructionsLoaded
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks InstructionsLoaded reçoivent ces champs :

| Champ               | Description                                                                                                                                                                                                                                         |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Chemin absolu vers le fichier d'instructions qui a été chargé                                                                                                                                                                                       |
| `memory_type`       | Portée du fichier : `"User"`, `"Project"`, `"Local"` ou `"Managed"`                                                                                                                                                                                 |
| `load_reason`       | Pourquoi le fichier a été chargé : `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"` ou `"compact"`. La valeur `"compact"` se déclenche lorsque les fichiers d'instructions sont rechargés après un événement de compaction |
| `globs`             | Modèles de glob de chemin du frontmatter `paths:` du fichier, le cas échéant. Présent uniquement pour les chargements `path_glob_match`                                                                                                             |
| `trigger_file_path` | Chemin vers le fichier dont l'accès a déclenché ce chargement, pour les chargements paresseux                                                                                                                                                       |
| `parent_file_path`  | Chemin vers le fichier d'instructions parent qui a inclus celui-ci, pour les chargements `include`                                                                                                                                                  |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "InstructionsLoaded",
  "file_path": "/Users/my-project/CLAUDE.md",
  "memory_type": "Project",
  "load_reason": "session_start"
}
```

<h4 id="instructionsloaded-decision-control">
  Contrôle de décision InstructionsLoaded
</h4>

Les hooks InstructionsLoaded n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer ou modifier le chargement des instructions. Utilisez cet événement pour la journalisation d'audit, le suivi de conformité ou l'observabilité.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

S'exécute lorsque l'utilisateur soumet un prompt, avant que Claude ne le traite. Cela vous permet d'ajouter du contexte supplémentaire basé sur le prompt/conversation, de valider les prompts ou de bloquer certains types de prompts.

Les hooks `UserPromptSubmit` ont un délai d'expiration par défaut de 30 secondes pour les types `command`, `http` et `mcp_tool`, plus court que le délai par défaut de 600 secondes pour ces types sur d'autres événements. Parce que ce hook s'exécute avant chaque prompt et bloque le traitement du modèle jusqu'à son achèvement, un hook bloqué paralyse la session. Si votre hook a besoin de plus de temps, définissez le champ `timeout` dans l'entrée du hook.

Un hook `UserPromptSubmit` qui atteint son délai d'expiration est annulé et sa sortie, y compris tout `additionalContext`, est supprimée. Le prompt atteint toujours Claude sans ce contexte. À partir de v2.1.196, la transcription affiche un avis nommant le hook, le délai d'expiration qui s'est déclenché et que la sortie a été supprimée. Les versions antérieures annulent le hook sans avis.

Un hook de rappel [Agent SDK](/docs/fr/agent-sdk/hooks) sur `UserPromptSubmit` qui atteint son délai d'expiration bloque le prompt avec un message nommant le hook et le délai d'expiration, car un rappel là peut agir comme une porte de politique qui ne doit pas échouer ouvertement. La session continue. Avant v2.1.208, un délai d'expiration de rappel sur cet événement terminait le tour avec une erreur d'exécution.

<h4 id="userpromptsubmit-input">
  Entrée UserPromptSubmit
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks UserPromptSubmit reçoivent le champ `prompt` contenant le texte que l'utilisateur a soumis.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

<h4 id="userpromptsubmit-decision-control">
  Contrôle de décision UserPromptSubmit
</h4>

Les hooks `UserPromptSubmit` peuvent contrôler si un prompt utilisateur est traité et ajouter du contexte. Tous les [champs de sortie JSON](#json-output) sont disponibles.

Il y a deux façons d'ajouter du contexte à la conversation sur exit code 0 :

* **Stdout en texte brut** : tout texte non-JSON écrit sur stdout est ajouté comme contexte
* **JSON avec `additionalContext`** : utilisez le format JSON ci-dessous pour plus de contrôle. Le champ `additionalContext` est ajouté comme contexte

Le stdout brut est affiché comme sortie de hook dans la transcription. La valeur `additionalContext` est injectée comme un rappel système que Claude lit sans entrée de transcription visible.

Pour bloquer un prompt, retournez un objet JSON avec `decision` défini à `"block"` :

| Champ                    | Description                                                                                                                           |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `decision`               | `"block"` empêche le prompt d'être traité et l'efface du contexte. Omettez pour autoriser le prompt à procéder                        |
| `reason`                 | Affiché à l'utilisateur lorsque `decision` est `"block"`. Non ajouté au contexte                                                      |
| `additionalContext`      | Chaîne ajoutée au contexte de Claude aux côtés du prompt soumis. Consultez [Ajouter du contexte pour Claude](#add-context-for-claude) |
| `sessionTitle`           | Définit le titre de la session. Utilisez pour nommer les sessions automatiquement en fonction du contenu du prompt                    |
| `suppressOriginalPrompt` | Si `true` lorsque `decision` est `"block"`, omet le texte du prompt original du message de blocage affiché à l'utilisateur            |

```json theme={null}
{
  "decision": "block",
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here",
    "sessionTitle": "My session title"
  }
}
```

<h3 id="userpromptexpansion">
  UserPromptExpansion
</h3>

S'exécute lorsqu'une commande slash tapée par l'utilisateur se développe en un prompt avant d'atteindre Claude. Utilisez ceci pour bloquer des commandes spécifiques de l'invocation directe, injecter du contexte pour une skill particulière ou enregistrer quelles commandes les utilisateurs invoquent. Par exemple, un hook correspondant à `deploy` peut bloquer `/deploy` sauf si un fichier d'approbation est présent, ou un hook correspondant à une skill de révision peut ajouter la liste de contrôle de révision de l'équipe comme `additionalContext`.

Cet événement couvre le chemin que `PreToolUse` ne couvre pas : un hook `PreToolUse` correspondant à l'outil `Skill` se déclenche uniquement lorsque Claude appelle l'outil, mais taper `/skillname` directement contourne `PreToolUse`. `UserPromptExpansion` se déclenche sur ce chemin direct.

Correspond à `command_name`. Laissez le matcher vide pour se déclencher sur chaque slash command de type prompt.

<h4 id="userpromptexpansion-input">
  Entrée UserPromptExpansion
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks UserPromptExpansion reçoivent `expansion_type`, `command_name`, `command_args`, `command_source` et la chaîne `prompt` originale. Le champ `expansion_type` est `slash_command` pour les skills et commandes personnalisées, ou `mcp_prompt` pour les prompts du serveur MCP.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../00893aaf.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptExpansion",
  "expansion_type": "slash_command",
  "command_name": "example-skill",
  "command_args": "arg1 arg2",
  "command_source": "plugin",
  "prompt": "/example-skill arg1 arg2"
}
```

<h4 id="userpromptexpansion-decision-control">
  Contrôle de décision UserPromptExpansion
</h4>

Les hooks `UserPromptExpansion` peuvent bloquer l'expansion ou ajouter du contexte. Tous les [champs de sortie JSON](#json-output) sont disponibles.

| Champ               | Description                                                                                                                              |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` empêche la slash command de se développer. Omettez pour autoriser sa progression                                               |
| `reason`            | Affiché à l'utilisateur lorsque `decision` est `"block"`                                                                                 |
| `additionalContext` | Chaîne ajoutée au contexte de Claude aux côtés du prompt développé. Consultez [Ajouter du contexte pour Claude](#add-context-for-claude) |

```json theme={null}
{
  "decision": "block",
  "reason": "This slash command is not available",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptExpansion",
    "additionalContext": "Additional context for this expansion"
  }
}
```

<h3 id="messagedisplay">
  MessageDisplay
</h3>

S'exécute pendant qu'un message d'assistant se diffuse à l'écran. Claude Code affiche le message par incréments : chaque fois qu'un lot de lignes nouvellement complétées est prêt à être rendu, le hook s'exécute une fois avec ces lignes et Claude Code rend le texte de remplacement du hook à leur place. Un long message produit plusieurs appels ; un court message peut ne produire qu'un seul.

Utilisez MessageDisplay pour :

* supprimer le markdown pour un affichage minimal
* transformer le texte qu'une application Agent SDK affiche à ses utilisateurs
* masquer les clés API ou les noms d'hôtes internes des réponses de Claude

Claude Code retient chaque lot jusqu'à ce que votre hook retourne, donc gardez le hook rapide. Si le hook échoue ou expire, Claude Code affiche le texte original. Le délai d'expiration par défaut pour cet événement est 10 secondes ; si votre hook a besoin de plus de temps, définissez le champ `timeout` dans l'entrée du hook.

MessageDisplay est affichage uniquement : le texte de remplacement change uniquement ce qui est rendu à l'écran. La transcription et ce que Claude voit conservent le texte original, donc Claude ne voit jamais le remplacement, et le mode verbeux affiche l'original. Le hook reçoit uniquement le texte du message d'assistant, donc les résultats d'outil et le texte que vous tapez s'affichent inchangés.

MessageDisplay ne supporte pas les matchers et se déclenche pour chaque message d'assistant qui diffuse du texte ; les messages sans texte, comme les réponses d'appel d'outil uniquement, ne le déclenchent pas.

Dans les exécutions non-interactives, y compris les requêtes Agent SDK et `claude -p`, MessageDisplay s'exécute une fois par message d'assistant au lieu d'une fois par lot de lignes. L'appel unique arrive après que le message se termine et porte le texte du message complet : `index` est `0`, `final` est `true` et `delta` contient le message entier. Un hook qui collecte le texte `delta` pour chaque message reçoit le même texte total dans les deux modes.

<h4 id="messagedisplay-input">
  Entrée MessageDisplay
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks MessageDisplay reçoivent des identifiants pour le tour et le message, la position de cet appel dans le message et le nouveau texte dans `delta`. Les limites de lot dépendent de la façon dont le texte se diffuse, donc utilisez `index` et `final` pour suivre la progression à travers un message plutôt que de vous attendre à ce que les lignes soient groupées d'une manière particulière.

| Champ        | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID du tour actuel                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `message_id` | UUID du message d'assistant en cours d'affichage. Stable sur chaque lot du même message. Ce n'est pas l'API `msg_…` id, donc il ne peut pas être corrélé avec les ids de message de transcription                                                                                                                                                                                                                                                                                          |
| `index`      | Index de base zéro de ce lot dans le message                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `final`      | `true` sur le dernier lot du message. Chaque message a exactement un dernier lot                                                                                                                                                                                                                                                                                                                                                                                                           |
| `delta`      | Les lignes nouvellement complétées depuis le lot précédent, y compris les sauts de ligne de fin. Toujours des lignes entières, sauf le dernier lot qui peut se terminer au milieu d'une ligne. Dans les exécutions interactives, le delta du dernier lot est vide lorsque le message se termine sur un saut de ligne, donc traitez `final`, pas un delta non-vide, comme le signal de fin de message. Dans les exécutions Agent SDK et `claude -p`, l'appel unique porte le message entier |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "MessageDisplay",
  "turn_id": "0c9e6a2f-7d41-4f4e-9a15-3f4f7c2b8d10",
  "message_id": "5b2a9c8e-1f63-4d8a-b7c4-9e0d2a6f1c3b",
  "index": 0,
  "final": false,
  "delta": "Here is the plan:\n"
}
```

<h4 id="messagedisplay-output">
  Sortie MessageDisplay
</h4>

En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, les hooks MessageDisplay peuvent retourner `displayContent` pour remplacer le delta à l'écran :

| Champ            | Description                                                            |
| :--------------- | :--------------------------------------------------------------------- |
| `displayContent` | Texte affiché à la place du delta. Omettez-le pour afficher l'original |

Les hooks MessageDisplay n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer le message ou modifier ce qui est stocké dans la transcription ou envoyé à Claude.

Cet exemple supprime la mise en forme markdown des réponses de Claude pour un affichage en texte brut. Le script lit chaque lot depuis stdin, supprime les marqueurs gras et les backticks de code en ligne du `delta`, et retourne le résultat comme `displayContent`.

<Tabs>
  <Tab title="macOS/Linux">
    Enregistrez un hook de commande pour l'événement dans votre fichier de paramètres :

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Enregistrez ce script dans `.claude/hooks/plain-display.sh` dans votre projet et rendez-le exécutable avec `chmod +x` :

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```

    Le script a besoin de `jq` sur votre `PATH`.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Enregistrez un hook de commande qui exécute le script via PowerShell :

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Le drapeau `-NoProfile` saute le chargement de votre profil PowerShell afin que le hook démarre rapidement, et `-ExecutionPolicy Bypass` permet à PowerShell d'exécuter le fichier de script local.

    Enregistrez ce script dans `.claude/hooks/plain-display.ps1` dans votre projet :

    ```powershell theme={null}
    $batch = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $text = $batch.delta -replace '\*\*', '' -replace '`', ''
    @{
      hookSpecificOutput = @{
        hookEventName = "MessageDisplay"
        displayContent = $text
      }
    } | ConvertTo-Json
    ```
  </Tab>
</Tabs>

Les lots sans markdown passent inchangés. Si le script échoue, par exemple parce que `jq` est manquant, Claude Code affiche le texte original et note l'échec uniquement dans la [sortie de débogage](#debug-hooks), pas dans la session.

<h3 id="pretooluse">
  PreToolUse
</h3>

S'exécute après que Claude crée les paramètres de l'outil et avant le traitement de l'appel d'outil. Correspond au nom de l'outil : `Bash`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `WebFetch`, `WebSearch`, `AskUserQuestion`, `ExitPlanMode` et tout [nom d'outil MCP](#match-mcp-tools).

<Warning>
  PreToolUse s'exécute uniquement lorsque Claude appelle un outil. Les fichiers que vous [référencez avec `@` dans votre prompt](/docs/fr/common-workflows#reference-files-and-directories) sont ajoutés sans aucun appel d'outil : Claude Code insère leurs contenus lors de la construction du prompt, donc aucun hook PreToolUse ne se déclenche pour eux, y compris les hooks correspondant à `Read`. Pour bloquer des chemins spécifiques des références `@`, utilisez une [règle de refus `Read`](/docs/fr/permissions#read-and-edit) à la place.
</Warning>

Utilisez [Contrôle de décision PreToolUse](#pretooluse-decision-control) pour autoriser, refuser, demander ou différer l'appel d'outil.

<h4 id="pretooluse-input">
  Entrée PreToolUse
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PreToolUse reçoivent `tool_name`, `tool_input` et `tool_use_id`. Les champs `tool_input` dépendent de l'outil :

<h5 id="bash">
  Bash
</h5>

Exécute les commandes shell.

| Champ               | Type    | Exemple            | Description                                                                                                                                                            |
| :------------------ | :------ | :----------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | La commande shell à exécuter                                                                                                                                           |
| `description`       | string  | `"Run test suite"` | Description optionnelle de ce que fait la commande                                                                                                                     |
| `timeout`           | number  | `120000`           | Délai d'expiration optionnel en millisecondes. Les valeurs au-dessus du [maximum](/docs/fr/tools-reference#bash-tool-behavior) sont réduites au maximum plutôt que rejetées |
| `run_in_background` | boolean | `false`            | Si la commande doit s'exécuter en arrière-plan                                                                                                                         |

<h5 id="write">
  Write
</h5>

Crée ou écrase un fichier.

| Champ       | Type   | Exemple               | Description                            |
| :---------- | :----- | :-------------------- | :------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Chemin absolu vers le fichier à écrire |
| `content`   | string | `"file content"`      | Contenu à écrire dans le fichier       |

<h5 id="edit">
  Edit
</h5>

Remplace une chaîne dans un fichier existant.

| Champ         | Type    | Exemple               | Description                                       |
| :------------ | :------ | :-------------------- | :------------------------------------------------ |
| `file_path`   | string  | `"/path/to/file.txt"` | Chemin absolu vers le fichier à éditer            |
| `old_string`  | string  | `"original text"`     | Texte à trouver et remplacer                      |
| `new_string`  | string  | `"replacement text"`  | Texte de remplacement                             |
| `replace_all` | boolean | `false`               | Si toutes les occurrences doivent être remplacées |

<h5 id="read">
  Read
</h5>

Lit le contenu des fichiers.

| Champ       | Type   | Exemple               | Description                                                    |
| :---------- | :----- | :-------------------- | :------------------------------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Chemin absolu vers le fichier à lire                           |
| `offset`    | number | `10`                  | Numéro de ligne optionnel à partir duquel commencer la lecture |
| `limit`     | number | `50`                  | Nombre optionnel de lignes à lire                              |

<h5 id="glob">
  Glob
</h5>

Trouve les fichiers correspondant à un modèle glob.

| Champ     | Type   | Exemple          | Description                                                                    |
| :-------- | :----- | :--------------- | :----------------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Modèle glob pour correspondre aux fichiers                                     |
| `path`    | string | `"/path/to/dir"` | Répertoire optionnel à rechercher. Par défaut le répertoire de travail courant |

<h5 id="grep">
  Grep
</h5>

Recherche le contenu des fichiers avec des expressions régulières.

| Champ         | Type    | Exemple          | Description                                                                         |
| :------------ | :------ | :--------------- | :---------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Modèle d'expression régulière à rechercher                                          |
| `path`        | string  | `"/path/to/dir"` | Fichier ou répertoire optionnel à rechercher                                        |
| `glob`        | string  | `"*.ts"`         | Modèle glob optionnel pour filtrer les fichiers                                     |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"` ou `"count"`. Par défaut `"files_with_matches"` |
| `-i`          | boolean | `true`           | Recherche insensible à la casse                                                     |
| `multiline`   | boolean | `false`          | Activer la correspondance multiligne                                                |

<h5 id="webfetch">
  WebFetch
</h5>

Récupère et traite le contenu web.

| Champ    | Type   | Exemple                       | Description                                   |
| :------- | :----- | :---------------------------- | :-------------------------------------------- |
| `url`    | string | `"https://example.com/api"`   | URL à partir de laquelle récupérer le contenu |
| `prompt` | string | `"Extract the API endpoints"` | Prompt à exécuter sur le contenu récupéré     |

<h5 id="websearch">
  WebSearch
</h5>

Recherche sur le web.

| Champ             | Type   | Exemple                        | Description                                                  |
| :---------------- | :----- | :----------------------------- | :----------------------------------------------------------- |
| `query`           | string | `"react hooks best practices"` | Requête de recherche                                         |
| `allowed_domains` | array  | `["docs.example.com"]`         | Optionnel : inclure uniquement les résultats de ces domaines |
| `blocked_domains` | array  | `["spam.example.com"]`         | Optionnel : exclure les résultats de ces domaines            |

<h5 id="agent">
  Agent
</h5>

Lance un [subagent](/docs/fr/sub-agents).

| Champ           | Type   | Exemple                    | Description                                                   |
| :-------------- | :----- | :------------------------- | :------------------------------------------------------------ |
| `prompt`        | string | `"Find all API endpoints"` | La tâche pour l'agent à effectuer                             |
| `description`   | string | `"Find API endpoints"`     | Description courte de la tâche                                |
| `subagent_type` | string | `"Explore"`                | Type d'agent spécialisé à utiliser                            |
| `model`         | string | `"sonnet"`                 | Alias de modèle optionnel pour remplacer la valeur par défaut |

Dans `PostToolUse`, `tool_response` pour un appel Agent complété porte le texte final du subagent ainsi que la télémétrie d'utilisation. Lisez ces champs pour enregistrer le coût par subagent à partir d'un hook :

| Champ               | Type   | Exemple                                               | Description                                                                                                                                                                                                                                        |
| :------------------ | :----- | :---------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` pour les appels synchrones, `"async_launched"` pour les subagents en arrière-plan. À partir de v2.1.198, les subagents s'exécutent en arrière-plan par défaut, donc un `run_in_background` omis produit également `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Identifiant pour l'exécution du subagent                                                                                                                                                                                                           |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Les blocs de texte finaux du subagent                                                                                                                                                                                                              |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Modèle sur lequel le subagent a fonctionné, qui peut différer du modèle demandé. Nécessite Claude Code v2.1.174 ou ultérieur                                                                                                                       |
| `totalTokens`       | number | `12450`                                               | Tokens totaux facturés sur les tours du subagent                                                                                                                                                                                                   |
| `totalDurationMs`   | number | `48211`                                               | Durée murale de l'exécution du subagent                                                                                                                                                                                                            |
| `totalToolUseCount` | number | `7`                                                   | Nombre d'appels d'outil que le subagent a effectués                                                                                                                                                                                                |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Ventilation des tokens par type : `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                                        |

Pour les subagents en arrière-plan, l'outil retourne immédiatement après le lancement du subagent, donc `tool_response` ne porte aucun champ d'utilisation. Il a `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile` et `resolvedModel` à la place.

Le champ `resolvedModel` nomme le modèle sur lequel le subagent fonctionne réellement, qui peut différer de la valeur `model` dans `tool_input`, comme lorsque `availableModels` ou un autre remplacement s'applique. Il nécessite Claude Code v2.1.174 ou ultérieur.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Pose à l'utilisateur une à quatre questions à choix multiples.

| Champ       | Type   | Exemple                                                                                                            | Description                                                                                                                                                                                                                                                |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Questions à présenter, chacune avec une chaîne `question`, un court `header`, un tableau `options` et un drapeau optionnel `multiSelect`                                                                                                                   |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Optionnel. Mappe le texte de la question à l'étiquette de l'option sélectionnée. Les réponses multi-sélection joignent les étiquettes avec des virgules. Claude ne définit pas ce champ ; fournissez-le via `updatedInput` pour répondre par programmation |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Présente un plan et demande à l'utilisateur de l'approuver avant que Claude ne quitte le [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode). Claude écrit le plan dans un fichier sur le disque avant d'appeler l'outil, donc l'`tool_input` littéral du modèle est généralement vide. Claude Code injecte le contenu du plan et le chemin du fichier avant de transmettre l'entrée aux hooks.

| Champ            | Type   | Exemple                                     | Description                                                                                                                                                            |
| :--------------- | :----- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Contenu du plan en Markdown. Injecté à partir du fichier de plan sur le disque                                                                                         |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Chemin vers le fichier de plan. Injecté                                                                                                                                |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Dépréciée. Claude Code accepte le champ mais l'ignore. Avant v2.1.205, il portait les permissions basées sur les prompts que Claude demandait pour implémenter le plan |

Dans `PostToolUse`, `tool_response` est un objet avec les champs `plan` et `filePath` contenant le plan approuvé, plus les drapeaux d'état internes. Lisez `tool_response.plan` pour le contenu du plan plutôt que de relire le fichier depuis le disque.

<h4 id="pretooluse-decision-control">
  Contrôle de décision PreToolUse
</h4>

Les hooks `PreToolUse` peuvent contrôler si un appel d'outil procède. Contrairement aux autres hooks qui utilisent un champ `decision` au niveau supérieur, PreToolUse retourne sa décision à l'intérieur d'un objet `hookSpecificOutput`. Cela lui donne un contrôle plus riche : quatre résultats (autoriser, refuser, demander ou différer) plus la capacité de modifier l'entrée de l'outil avant l'exécution.

| Champ                      | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` contourne le système de permission, sauf pour les [outils qui nécessitent une interaction utilisateur](#pretooluse-decision-control) et les outils connecteur que [votre organisation a défini à `ask`](/docs/fr/mcp#organization-controls-on-connector-tools). `"deny"` empêche l'appel d'outil. `"ask"` demande à l'utilisateur de confirmer. `"defer"` sort gracieusement afin que l'outil puisse être repris plus tard. Les règles [Deny and ask](/docs/fr/permissions#manage-permissions) s'appliquent toujours indépendamment de ce que le hook retourne |
| `permissionDecisionReason` | Pour `"allow"` et `"ask"`, affiché à l'utilisateur mais pas à Claude. Pour `"deny"`, affiché à Claude. Pour `"defer"`, ignoré                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `updatedInput`             | Modifie les paramètres d'entrée de l'outil avant l'exécution. Remplace l'objet d'entrée entier, donc incluez les champs inchangés aux côtés des champs modifiés. Combinez avec `"allow"` pour approuver automatiquement ou `"ask"` pour montrer l'entrée modifiée à l'utilisateur. Pour `"defer"`, ignoré                                                                                                                                                                                                                                                      |
| `additionalContext`        | Chaîne ajoutée au contexte de Claude avant l'exécution de l'outil. Pour `"defer"`, ignoré. Consultez [Ajouter du contexte pour Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                                                |

Lorsque plusieurs hooks PreToolUse retournent des décisions différentes, la précédence est `deny` > `defer` > `ask` > `allow`.

Lorsqu'un hook retourne `"ask"`, le dialogue de permission affiché à l'utilisateur inclut un libellé identifiant d'où provient le hook : par exemple, `[User]`, `[Project]`, `[Plugin]` ou `[Local]`. Cela aide les utilisateurs à comprendre quelle source de configuration demande une confirmation.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "My reason here",
    "updatedInput": {
      "field_to_modify": "new value"
    },
    "additionalContext": "Current environment: production. Proceed with caution."
  }
}
```

`AskUserQuestion` et `ExitPlanMode` nécessitent une interaction utilisateur et bloquent normalement en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`. Retourner `permissionDecision: "allow"` avec `updatedInput` satisfait cette exigence : le hook lit l'entrée de l'outil depuis stdin, collecte la réponse via votre propre interface utilisateur et la retourne dans `updatedInput` afin que l'outil s'exécute sans inviter. Retourner `"allow"` seul n'est pas suffisant pour ces outils. Pour `AskUserQuestion`, renvoyez le tableau `questions` original et ajoutez un objet [`answers`](#askuserquestion) mappant le texte de chaque question à la réponse choisie.

Les outils connecteur que [votre organisation a défini à `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) invitent même lorsqu'un hook retourne `"allow"`.

À partir de v2.1.199, un outil MCP dont le serveur le marque avec [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool) est plus strict : un hook ne peut pas ignorer son dialogue d'approbation avec `"allow"`, avec ou sans `updatedInput`, car Claude Code ne peut pas confirmer que le hook a collecté l'interaction dont l'outil a besoin.

<Note>
  PreToolUse utilisait auparavant les champs `decision` et `reason` au niveau supérieur, mais ceux-ci sont dépréciés pour cet événement. Utilisez `hookSpecificOutput.permissionDecision` et `hookSpecificOutput.permissionDecisionReason` à la place. Les valeurs dépréciées `"approve"` et `"block"` correspondent à `"allow"` et `"deny"` respectivement. Les autres événements comme PostToolUse et Stop continuent d'utiliser `decision` et `reason` au niveau supérieur comme format actuel.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Différer un appel d'outil pour plus tard
</h4>

`"defer"` est pour les intégrations qui exécutent `claude -p` en tant que sous-processus et lisent sa sortie JSON, comme une application Agent SDK ou une interface utilisateur personnalisée construite sur Claude Code. Il permet à ce processus appelant de mettre en pause Claude à un appel d'outil, de collecter l'entrée via sa propre interface et de reprendre où il s'était arrêté. Claude Code honore cette valeur uniquement en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`. Dans les sessions interactives, il enregistre un avertissement et ignore le résultat du hook.

L'outil `AskUserQuestion` est le cas typique : Claude veut poser une question à l'utilisateur, mais il n'y a pas de terminal pour répondre. Le cycle aller-retour fonctionne comme ceci :

1. Claude appelle `AskUserQuestion`. Le hook `PreToolUse` se déclenche.
2. Le hook retourne `permissionDecision: "defer"`. L'outil ne s'exécute pas. Le processus quitte avec `stop_reason: "tool_deferred"` et l'appel d'outil en attente préservé dans la transcription.
3. Le processus appelant lit `deferred_tool_use` du résultat SDK, affiche la question dans sa propre interface utilisateur et attend une réponse.
4. Le processus appelant exécute `claude -p --resume <session-id>`. Le même appel d'outil déclenche `PreToolUse` à nouveau.
5. Le hook retourne `permissionDecision: "allow"` avec la réponse dans `updatedInput`. L'outil s'exécute et Claude continue.

Le champ `deferred_tool_use` porte l'`id`, le `name` et l'`input` de l'outil. L'`input` est les paramètres que Claude a générés pour l'appel d'outil, capturés avant l'exécution :

```json theme={null}
{
  "type": "result",
  "subtype": "success",
  "stop_reason": "tool_deferred",
  "session_id": "abc123",
  "deferred_tool_use": {
    "id": "toolu_01abc",
    "name": "AskUserQuestion",
    "input": { "questions": [{ "question": "Which framework?", "header": "Framework", "options": [{"label": "React"}, {"label": "Vue"}], "multiSelect": false }] }
  }
}
```

Il n'y a pas de délai d'expiration ou de limite de tentatives. La session reste sur le disque jusqu'à ce que vous la repreniez, soumise au balayage de rétention [`cleanupPeriodDays`](/docs/fr/settings#available-settings) qui supprime les fichiers de session après 30 jours par défaut. Si la réponse n'est pas prête lorsque vous reprenez, le hook peut retourner `"defer"` à nouveau et le processus quitte de la même manière. Le processus appelant contrôle quand casser la boucle en retournant finalement `"allow"` ou `"deny"` du hook.

`"defer"` ne fonctionne que lorsque Claude fait un seul appel d'outil dans le tour. Si Claude fait plusieurs appels d'outil à la fois, `"defer"` est ignoré avec un avertissement et l'outil procède à travers le flux de permission normal. La contrainte existe car la reprise ne peut réexécuter qu'un seul outil : il n'y a aucun moyen de différer un appel d'une batch sans laisser les autres non résolus.

Si l'outil différé n'est plus disponible lorsque vous reprenez, le processus quitte avec `stop_reason: "tool_deferred_unavailable"` et `is_error: true` avant que le hook ne se déclenche. Cela se produit lorsqu'un serveur MCP qui a fourni l'outil n'est pas connecté pour la session reprise. La charge utile `deferred_tool_use` est toujours incluse afin que vous puissiez identifier quel outil a disparu.

<Note>
  `--resume` restaure le mode de permission qui était actif lorsque l'outil a été différé, donc vous n'avez pas besoin de passer `--permission-mode` à nouveau. Les exceptions sont `plan` et `bypassPermissions`, qui ne sont jamais reportés. Passer `--permission-mode` explicitement lors de la reprise remplace la valeur restaurée.
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

S'exécute lorsque l'utilisateur est montré un dialogue de permission.
Utilisez [Contrôle de décision PermissionRequest](#permissionrequest-decision-control) pour autoriser ou refuser au nom de l'utilisateur.

Correspond au nom de l'outil, mêmes valeurs que PreToolUse.

<h4 id="permissionrequest-input">
  Entrée PermissionRequest
</h4>

Les hooks PermissionRequest reçoivent les champs `tool_name` et `tool_input` comme les hooks PreToolUse, mais sans `tool_use_id`. Un tableau optionnel `permission_suggestions` contient les options « toujours autoriser » que l'utilisateur verrait normalement dans le dialogue de permission. La différence est quand le hook se déclenche : les hooks PermissionRequest s'exécutent lorsqu'un dialogue de permission est sur le point d'être montré à l'utilisateur, tandis que les hooks PreToolUse s'exécutent avant l'exécution de l'outil indépendamment du statut de permission.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PermissionRequest",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf node_modules",
    "description": "Remove node_modules directory"
  },
  "permission_suggestions": [
    {
      "type": "addRules",
      "rules": [{ "toolName": "Bash", "ruleContent": "rm -rf node_modules" }],
      "behavior": "allow",
      "destination": "localSettings"
    }
  ]
}
```

<h4 id="permissionrequest-decision-control">
  Contrôle de décision PermissionRequest
</h4>

Les hooks `PermissionRequest` peuvent autoriser ou refuser les demandes de permission. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner un objet `decision` avec ces champs spécifiques à l'événement :

| Champ                | Description                                                                                                                                                                                                                                                     |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` accorde la permission, `"deny"` la refuse. Les règles [Deny and ask](/docs/fr/permissions#manage-permissions) sont toujours évaluées, donc un hook retournant `"allow"` ne remplace pas une règle deny correspondante                                      |
| `updatedInput`       | Pour `"allow"` uniquement : modifie les paramètres d'entrée de l'outil avant l'exécution. Remplace l'objet d'entrée entier, donc incluez les champs inchangés aux côtés des champs modifiés. L'entrée modifiée est réévaluée par rapport aux règles deny et ask |
| `updatedPermissions` | Pour `"allow"` uniquement : tableau d'[entrées de mise à jour de permission](#permission-update-entries) à appliquer, comme l'ajout d'une règle d'autorisation ou la modification du mode de permission de session                                              |
| `message`            | Pour `"deny"` uniquement : indique à Claude pourquoi la permission a été refusée                                                                                                                                                                                |
| `interrupt`          | Pour `"deny"` uniquement : si `true`, arrête Claude                                                                                                                                                                                                             |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```

<h4 id="permission-update-entries">
  Entrées de mise à jour de permission
</h4>

Le champ de sortie `updatedPermissions` et le champ d'[entrée `permission_suggestions`](#permissionrequest-input) utilisent tous deux le même tableau d'objets d'entrée. Chaque entrée a un `type` qui détermine ses autres champs, et une `destination` qui contrôle où la modification est écrite.

| `type`              | Champs                             | Effet                                                                                                                                                                                                                              |
| :------------------ | :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `addRules`          | `rules`, `behavior`, `destination` | Ajoute des règles de permission. `rules` est un tableau d'objets `{toolName, ruleContent?}`. Omettez `ruleContent` pour correspondre à l'outil entier. `behavior` est `"allow"`, `"deny"` ou `"ask"`                               |
| `replaceRules`      | `rules`, `behavior`, `destination` | Remplace toutes les règles du `behavior` donné à la `destination` par les `rules` fournies                                                                                                                                         |
| `removeRules`       | `rules`, `behavior`, `destination` | Supprime les règles correspondantes du `behavior` donné                                                                                                                                                                            |
| `setMode`           | `mode`, `destination`              | Change le mode de permission. Les modes valides sont `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan` et `manual` comme alias pour `default`. L'alias `manual` nécessite Claude Code v2.1.200 ou ultérieur |
| `addDirectories`    | `directories`, `destination`       | Ajoute des répertoires de travail. `directories` est un tableau de chaînes de chemin                                                                                                                                               |
| `removeDirectories` | `directories`, `destination`       | Supprime les répertoires de travail                                                                                                                                                                                                |

<Note>
  `setMode` avec `bypassPermissions` ne prend effet que si la session a été lancée avec le mode bypass déjà disponible : `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` ou `permissions.defaultMode: "bypassPermissions"` dans les paramètres, et le mode n'est pas désactivé par [`permissions.disableBypassPermissionsMode`](/docs/fr/permissions#managed-settings). Sinon la mise à jour est un non-op. `bypassPermissions` n'est jamais persisté comme `defaultMode` indépendamment de `destination`.
</Note>

Le champ `destination` sur chaque entrée détermine si la modification reste en mémoire ou persiste dans un fichier de paramètres.

| `destination`     | Écrit dans                                             |
| :---------------- | :----------------------------------------------------- |
| `session`         | en mémoire uniquement, supprimé à la fin de la session |
| `localSettings`   | `.claude/settings.local.json`                          |
| `projectSettings` | `.claude/settings.json`                                |
| `userSettings`    | `~/.claude/settings.json`                              |

Un hook peut renvoyer l'une des `permission_suggestions` qu'il a reçues comme sa propre sortie `updatedPermissions`, ce qui équivaut à l'utilisateur sélectionnant cette option « toujours autoriser » dans le dialogue.

<h3 id="posttooluse">
  PostToolUse
</h3>

S'exécute immédiatement après qu'un outil se termine avec succès.

Correspond au nom de l'outil, mêmes valeurs que PreToolUse.

<h4 id="posttooluse-input">
  Entrée PostToolUse
</h4>

Les hooks `PostToolUse` se déclenchent après qu'un outil s'est déjà exécuté avec succès. L'entrée inclut à la fois `tool_input`, les arguments envoyés à l'outil, et `tool_response`, le résultat qu'il a retourné. Le schéma exact pour les deux dépend de l'outil.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "success": true
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Champ         | Description                                                                                                                              |
| :------------ | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `duration_ms` | Optionnel. Temps d'exécution de l'outil en millisecondes. Exclut le temps passé dans les dialogues de permission et les hooks PreToolUse |

<h4 id="posttooluse-decision-control">
  Contrôle de décision PostToolUse
</h4>

Les hooks `PostToolUse` peuvent fournir des commentaires à Claude après l'exécution de l'outil. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner ces champs spécifiques à l'événement :

| Champ                  | Description                                                                                                                                             |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `decision`             | `"block"` ajoute la `reason` à côté du résultat de l'outil. Claude voit toujours la sortie originale ; pour la remplacer, utilisez `updatedToolOutput`  |
| `reason`               | Explication affichée à Claude lorsque `decision` est `"block"`                                                                                          |
| `additionalContext`    | Chaîne ajoutée au contexte de Claude aux côtés du résultat de l'outil. Consultez [Ajouter du contexte pour Claude](#add-context-for-claude)             |
| `updatedToolOutput`    | Remplace la sortie de l'outil par la valeur fournie avant qu'elle ne soit envoyée à Claude. La valeur doit correspondre à la forme de sortie de l'outil |
| `updatedMCPToolOutput` | Remplace la sortie pour les [outils MCP](#match-mcp-tools) uniquement. Préférez `updatedToolOutput`, qui fonctionne pour tous les outils                |

L'exemple ci-dessous remplace la sortie d'un appel `Bash`. La valeur de remplacement correspond à la forme de sortie de l'outil `Bash` :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude",
    "updatedToolOutput": {
      "stdout": "[redacted]",
      "stderr": "",
      "interrupted": false,
      "isImage": false
    }
  }
}
```

<Warning>
  `updatedToolOutput` change uniquement ce que Claude voit. L'outil a déjà fonctionné au moment où le hook se déclenche, donc tous les fichiers écrits, commandes exécutées ou demandes réseau envoyées ont déjà pris effet. La télémétrie telle que les spans d'outils OpenTelemetry et les événements d'analyse capturent également la sortie originale avant l'exécution du hook. Pour empêcher ou modifier un appel d'outil avant son exécution, utilisez un hook [PreToolUse](#pretooluse) à la place.

  La valeur de remplacement doit correspondre à la forme de sortie de l'outil. Les outils intégrés retournent des objets structurés plutôt que des chaînes brutes. Par exemple, `Bash` retourne un objet avec les champs `stdout`, `stderr`, `interrupted` et `isImage`. Pour les outils intégrés, une valeur qui ne correspond pas au schéma de sortie de l'outil est ignorée et la sortie originale est utilisée. La sortie de l'outil MCP est transmise sans validation de schéma. Supprimer les détails d'erreur dont Claude a besoin peut le faire procéder sur une fausse hypothèse.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

S'exécute lorsqu'une exécution d'outil échoue : l'outil a levé une erreur ou un outil MCP a retourné un résultat d'erreur. Utilisez ceci pour enregistrer les défaillances, envoyer des alertes ou fournir des commentaires correctifs à Claude.

Correspond au nom de l'outil, mêmes valeurs que PreToolUse.

<Note>
  Cet événement ne se déclenche pas pour les appels d'outil rejetés avant l'exécution : un nom d'outil inconnu, une entrée qui échoue la validation de schéma ou spécifique à l'outil, ou un refus de permission. Les rejets de validation sont retournés comme résultats `tool_use_error` et se produisent avant que les hooks ne s'exécutent, donc ils ne déclenchent ni `PreToolUse` ni cet événement. Les refus de permission déclenchent `PreToolUse` mais pas cet événement ; consultez [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  Entrée PostToolUseFailure
</h4>

Les hooks PostToolUseFailure reçoivent les mêmes champs `tool_name` et `tool_input` que PostToolUse, ainsi que les informations d'erreur comme champs au niveau supérieur :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUseFailure",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite"
  },
  "tool_use_id": "toolu_01ABC123...",
  "error": "Command exited with non-zero status code 1",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Champ          | Description                                                                                                                              |
| :------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | Chaîne décrivant ce qui s'est mal passé                                                                                                  |
| `is_interrupt` | Booléen optionnel indiquant si l'échec a été causé par une interruption utilisateur                                                      |
| `duration_ms`  | Optionnel. Temps d'exécution de l'outil en millisecondes. Exclut le temps passé dans les dialogues de permission et les hooks PreToolUse |

<h4 id="posttoolusefailure-decision-control">
  Contrôle de décision PostToolUseFailure
</h4>

Les hooks `PostToolUseFailure` peuvent fournir du contexte à Claude après l'échec d'un outil. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner ces champs spécifiques à l'événement :

| Champ               | Description                                                                                                                      |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Chaîne ajoutée au contexte de Claude aux côtés de l'erreur. Consultez [Ajouter du contexte pour Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Additional information about the failure for Claude"
  }
}
```

<h3 id="posttoolbatch">
  PostToolBatch
</h3>

S'exécute une fois après que chaque appel d'outil dans une batch ait été résolu, avant que Claude Code n'envoie la demande suivante au modèle. `PostToolUse` se déclenche une fois par outil, ce qui signifie qu'il se déclenche simultanément lorsque Claude fait des appels d'outil parallèles. `PostToolBatch` se déclenche exactement une fois avec la batch complète, donc c'est le bon endroit pour injecter du contexte qui dépend de l'ensemble des outils qui ont fonctionné plutôt que sur un seul outil. Il n'y a pas de matcher pour cet événement.

<h4 id="posttoolbatch-input">
  Entrée PostToolBatch
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PostToolBatch reçoivent `tool_calls`, un tableau décrivant chaque appel d'outil dans la batch :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolBatch",
  "tool_calls": [
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/accounts.py"},
      "tool_use_id": "toolu_01...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    },
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/transactions.py"},
      "tool_use_id": "toolu_02...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    }
  ]
}
```

`tool_response` contient le même contenu que le modèle reçoit dans le bloc `tool_result` correspondant. La valeur est une chaîne sérialisée ou un tableau de blocs de contenu, exactement comme l'outil l'a émis. Pour `Read`, cela signifie du texte préfixé par le numéro de ligne plutôt que le contenu brut du fichier. Les réponses peuvent être volumineuses, donc analysez uniquement les champs dont vous avez besoin.

<Note>
  La forme `tool_response` diffère de celle de `PostToolUse`. `PostToolUse` transmet l'objet `Output` structuré de l'outil, comme `{filePath: "...", success: true}` pour `Write` ; `PostToolBatch` transmet le contenu `tool_result` sérialisé que le modèle voit.
</Note>

<h4 id="posttoolbatch-decision-control">
  Contrôle de décision PostToolBatch
</h4>

Les hooks `PostToolBatch` peuvent injecter du contexte pour Claude. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner ces champs spécifiques à l'événement :

| Champ               | Description                                                                                                                                                                                                                                                  |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Chaîne de contexte injectée une fois avant l'appel du modèle suivant. Consultez [Ajouter du contexte pour Claude](#add-context-for-claude) pour les détails de livraison, ce qu'il faut y mettre et comment les sessions reprises gèrent les valeurs passées |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Retourner `decision: "block"` ou `continue: false` arrête la boucle agentique avant l'appel du modèle suivant.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

S'exécute lorsque le classificateur du [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) refuse un appel d'outil. Ce hook ne se déclenche que en mode auto : il ne s'exécute pas lorsque vous refusez manuellement un dialogue de permission, lorsqu'un hook `PreToolUse` bloque un appel ou lorsqu'une règle `deny` correspond. Utilisez-le pour enregistrer les refus du classificateur, ajuster la configuration ou indiquer au modèle qu'il peut réessayer l'appel d'outil.

Correspond au nom de l'outil, mêmes valeurs que PreToolUse.

<h4 id="permissiondenied-input">
  Entrée PermissionDenied
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PermissionDenied reçoivent `tool_name`, `tool_input`, `tool_use_id` et `reason`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "auto",
  "hook_event_name": "PermissionDenied",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf /tmp/build",
    "description": "Clean build directory"
  },
  "tool_use_id": "toolu_01ABC123...",
  "reason": "Auto mode denied: command targets a path outside the project"
}
```

| Champ    | Description                                                                |
| :------- | :------------------------------------------------------------------------- |
| `reason` | L'explication du classificateur pour pourquoi l'appel d'outil a été refusé |

<h4 id="permissiondenied-decision-control">
  Contrôle de décision PermissionDenied
</h4>

Les hooks PermissionDenied peuvent indiquer au modèle qu'il peut réessayer l'appel d'outil refusé. Retournez un objet JSON avec `hookSpecificOutput.retry` défini à `true` :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Lorsque `retry` est `true`, Claude Code ajoute un message à la conversation indiquant au modèle qu'il peut réessayer l'appel d'outil. Le refus lui-même n'est pas inversé. Si votre hook ne retourne pas JSON ou retourne `retry: false`, le refus tient et le modèle reçoit le message de rejet original.

<h3 id="notification">
  Notification
</h3>

S'exécute lorsque Claude Code envoie des notifications. Correspond au type de notification. Omettez le matcher pour exécuter les hooks pour tous les types de notification.

| Matcher                | Quand il se déclenche                                                                                                                                         |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `permission_prompt`    | Claude a besoin de votre approbation pour un appel d'outil                                                                                                    |
| `idle_prompt`          | Claude a terminé et attend votre prochain prompt                                                                                                              |
| `auth_success`         | L'authentification se termine                                                                                                                                 |
| `elicitation_dialog`   | Un serveur MCP ouvre un formulaire d'élicitation                                                                                                              |
| `elicitation_complete` | Un formulaire d'élicitation MCP est soumis ou fermé                                                                                                           |
| `elicitation_response` | Une réponse d'élicitation MCP est renvoyée au serveur                                                                                                         |
| `agent_needs_input`    | Une session en arrière-plan commence à attendre votre entrée. Se déclenche uniquement pendant que la [vue agent](/docs/fr/agent-view) est ouverte dans un terminal |
| `agent_completed`      | Une session en arrière-plan se termine ou échoue. Se déclenche uniquement pendant que la [vue agent](/docs/fr/agent-view) est ouverte dans un terminal             |

Les types `agent_needs_input` et `agent_completed` nécessitent Claude Code v2.1.198 ou ultérieur.

Utilisez des matchers séparés pour exécuter différents gestionnaires selon le type de notification. Cette configuration déclenche un script d'alerte spécifique à la permission lorsque Claude a besoin d'approbation de permission et une notification différente lorsque Claude a été inactif :

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/permission-alert.sh"
          }
        ]
      },
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/idle-notification.sh"
          }
        ]
      }
    ]
  }
}
```

<h4 id="notification-input">
  Entrée Notification
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks Notification reçoivent `message` avec le texte de notification, un `title` optionnel et `notification_type` indiquant quel type s'est déclenché.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission",
  "title": "Permission needed",
  "notification_type": "permission_prompt"
}
```

Les hooks Notification ne peuvent pas bloquer ou modifier les notifications. Ils sont destinés aux effets secondaires tels que le transfert de la notification vers un service externe. Les [champs de sortie JSON](#json-output) communs tels que `systemMessage` s'appliquent.

<h3 id="subagentstart">
  SubagentStart
</h3>

S'exécute lorsqu'un subagent Claude Code est lancé via l'outil Agent. Supporte les matchers pour filtrer par nom de type d'agent. Pour les agents intégrés, c'est le nom de l'agent comme `general-purpose`, `Explore` ou `Plan`. Pour les [subagents personnalisés](/docs/fr/sub-agents), c'est le champ `name` du frontmatter de l'agent, pas le nom du fichier.

Pour les subagents fournis par un [plugin](/docs/fr/plugins), l'identifiant de type d'agent est l'identifiant limité au plugin comme `my-plugin:reviewer`, pas le nom brut du frontmatter. Le deux-points place un nom limité au plugin sur le chemin d'expression régulière, donc ancrez le matcher avec `^` et `$` pour une correspondance exacte : `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  Entrée SubagentStart
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks SubagentStart reçoivent `agent_id` avec l'identifiant unique du subagent et `agent_type` avec le nom de l'agent que le matcher filtre.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SubagentStart",
  "agent_id": "agent-abc123",
  "agent_type": "Explore"
}
```

Les hooks SubagentStart ne peuvent pas bloquer la création de subagent, mais ils peuvent injecter du contexte dans le subagent. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, vous pouvez retourner :

| Champ               | Description                                                                                                                                                        |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Chaîne ajoutée au contexte du subagent au début de sa conversation, avant son premier prompt. Consultez [Ajouter du contexte pour Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

<h3 id="subagentstop">
  SubagentStop
</h3>

S'exécute lorsqu'un subagent Claude Code a terminé sa réponse. Correspond au type d'agent, mêmes valeurs que SubagentStart.

<h4 id="subagentstop-input">
  Entrée SubagentStop
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks SubagentStop reçoivent `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path` et `last_assistant_message`. Le champ `agent_type` est la valeur utilisée pour le filtrage du matcher. Le `transcript_path` est la transcription de la session principale, tandis que `agent_transcript_path` est la propre transcription du subagent stockée dans un dossier `subagents/` imbriqué. Le champ `last_assistant_message` contient le contenu textuel de la réponse finale du subagent, donc les hooks peuvent y accéder sans analyser le fichier de transcription.

Les hooks SubagentStop reçoivent également les tableaux `background_tasks` et `session_crons` décrits sous [Entrée Stop](#stop-input), disponibles dans Claude Code v2.1.145 ou ultérieur. Les deux tableaux sont limités à la session parent, pas au subagent.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "SubagentStop",
  "stop_hook_active": false,
  "agent_id": "def456",
  "agent_type": "Explore",
  "agent_transcript_path": "~/.claude/projects/.../abc123/subagents/agent-def456.jsonl",
  "last_assistant_message": "Analysis complete. Found 3 potential issues...",
  "background_tasks": [],
  "session_crons": []
}
```

Les hooks SubagentStop utilisent le même format de contrôle de décision que les [hooks Stop](#stop-decision-control), y compris `hookSpecificOutput.additionalContext` avec `hookEventName` défini à `"SubagentStop"`, pour les commentaires sans erreur qui gardent le subagent en cours d'exécution. Retourner `decision: "block"` avec une `reason` garde le subagent en cours d'exécution et livre `reason` au subagent comme sa prochaine instruction. Pour injecter du contexte dans la session parent après qu'un subagent retourne, utilisez un hook [`PostToolUse`](#posttooluse) sur l'outil `Agent` à la place.

<h3 id="taskcreated">
  TaskCreated
</h3>

S'exécute lorsqu'une tâche est en cours de création via l'outil `TaskCreate`. Utilisez ceci pour appliquer les conventions de nommage, exiger les descriptions de tâches ou empêcher certaines tâches d'être créées.

Lorsqu'un hook `TaskCreated` quitte avec le code 2, la tâche n'est pas créée et le message stderr est renvoyé au modèle comme commentaire. Pour arrêter complètement le coéquipier au lieu de le relancer, retournez JSON avec `{"continue": false, "stopReason": "..."}`. Les hooks TaskCreated ne supportent pas les matchers et se déclenchent à chaque occurrence.

<h4 id="taskcreated-input">
  Entrée TaskCreated
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks TaskCreated reçoivent `task_id`, `task_subject` et optionnellement `task_description`, `teammate_name` et `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Champ              | Description                                                                           |
| :----------------- | :------------------------------------------------------------------------------------ |
| `task_id`          | Identifiant de la tâche en cours de création                                          |
| `task_subject`     | Titre de la tâche                                                                     |
| `task_description` | Description détaillée de la tâche. Peut être absent                                   |
| `teammate_name`    | Nom du coéquipier créant la tâche. Peut être absent                                   |
| `team_name`        | Dépréciée. Nom d'équipe dérivé de la session ; sera supprimée dans une version future |

<h4 id="taskcreated-decision-control">
  Contrôle de décision TaskCreated
</h4>

Les hooks TaskCreated supportent deux façons de contrôler la création de tâche :

* **Code de sortie 2** : la tâche n'est pas créée et le message stderr est renvoyé au modèle comme commentaire.
* **JSON `{"continue": false, "stopReason": "..."}`** : arrête complètement le coéquipier, correspondant au comportement du hook `Stop`. Le `stopReason` est affiché à l'utilisateur.

Cet exemple bloque les tâches dont les sujets ne suivent pas le format requis :

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

if [[ ! "$TASK_SUBJECT" =~ ^\[TICKET-[0-9]+\] ]]; then
  echo "Task subject must start with a ticket number, e.g. '[TICKET-123] Add feature'" >&2
  exit 2
fi

exit 0
```

<h3 id="taskcompleted">
  TaskCompleted
</h3>

S'exécute lorsqu'une tâche est marquée comme complétée. Cela se déclenche dans deux situations : lorsqu'un agent marque explicitement une tâche comme complétée via l'outil TaskUpdate, ou lorsqu'un coéquipier d'une [équipe d'agents](/docs/fr/agent-teams) termine son tour avec des tâches en cours. Utilisez ceci pour appliquer les critères d'achèvement comme passer les tests ou les vérifications de lint avant qu'une tâche ne puisse se fermer.

Lorsqu'un hook `TaskCompleted` quitte avec le code 2, la tâche n'est pas marquée comme complétée et le message stderr est renvoyé au modèle comme commentaire. Pour arrêter complètement le coéquipier au lieu de le relancer, retournez JSON avec `{"continue": false, "stopReason": "..."}`. Les hooks TaskCompleted ne supportent pas les matchers et se déclenchent à chaque occurrence.

<h4 id="taskcompleted-input">
  Entrée TaskCompleted
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks TaskCompleted reçoivent `task_id`, `task_subject` et optionnellement `task_description`, `teammate_name` et `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCompleted",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Champ              | Description                                                                           |
| :----------------- | :------------------------------------------------------------------------------------ |
| `task_id`          | Identifiant de la tâche en cours de réalisation                                       |
| `task_subject`     | Titre de la tâche                                                                     |
| `task_description` | Description détaillée de la tâche. Peut être absent                                   |
| `teammate_name`    | Nom du coéquipier complétant la tâche. Peut être absent                               |
| `team_name`        | Dépréciée. Nom d'équipe dérivé de la session ; sera supprimée dans une version future |

<h4 id="taskcompleted-decision-control">
  Contrôle de décision TaskCompleted
</h4>

Les hooks TaskCompleted supportent deux façons de contrôler l'achèvement de la tâche :

* **Code de sortie 2** : la tâche n'est pas marquée comme complétée et le message stderr est renvoyé au modèle comme commentaire.
* **JSON `{"continue": false, "stopReason": "..."}`** : arrête complètement le coéquipier, correspondant au comportement du hook `Stop`. Le `stopReason` est affiché à l'utilisateur.

Cet exemple exécute les tests et bloque l'achèvement de la tâche s'ils échouent :

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Exécutez la suite de tests
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

S'exécute lorsque l'agent Claude Code principal a terminé sa réponse. Ne s'exécute pas si l'arrêt s'est produit en raison d'une interruption utilisateur. Les erreurs API déclenchent [StopFailure](#stopfailure) à la place.

<Tip>
  La commande [`/goal`](/docs/fr/goal) est un raccourci intégré pour un hook Stop basé sur un prompt limité à la session. Utilisez-la lorsque vous voulez que Claude continue à travailler jusqu'à ce qu'une condition soit remplie sans écrire de configuration de hook.
</Tip>

<h4 id="stop-input">
  Entrée Stop
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks Stop reçoivent `stop_hook_active`, `last_assistant_message`, `background_tasks` et `session_crons`. Le champ `stop_hook_active` est `true` lorsque Claude Code continue déjà en raison d'un hook stop. Vérifiez cette valeur ou traitez la transcription pour empêcher de bloquer sur une condition qui ne se résoudra jamais. Claude Code remplace le hook et termine le tour après 8 blocages consécutifs.

Le champ `last_assistant_message` contient le contenu textuel de la réponse finale de Claude, donc les hooks peuvent y accéder sans analyser le fichier de transcription.

Les tableaux `background_tasks` et `session_crons`, disponibles dans Claude Code v2.1.145 ou ultérieur, permettent aux hooks de distinguer « la session est terminée » de « la session est en pause en attente du réveil du travail en arrière-plan ». Les deux tableaux sont présents lorsque le registre des tâches est accessible et sont vides lorsque rien n'est en vol ou programmé.

Chaque entrée dans `background_tasks` décrit une tâche en vol et utilise ces champs :

| Champ         | Description                                                                                                                                                                                                                                                                   |
| :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Identifiant de la tâche                                                                                                                                                                                                                                                       |
| `type`        | Étiquette de type de tâche conviviale telle que `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session` ou `MCP task`. Chaque étiquette identifie quelle fonctionnalité Claude Code a créé la tâche. Revient au discriminant brut pour les types non reconnus |
| `status`      | Statut actuel de la tâche                                                                                                                                                                                                                                                     |
| `description` | Description en texte libre, limitée à 1000 caractères avec un marqueur `… [+N chars]` en chaîne lorsqu'elle est coupée                                                                                                                                                        |
| `command`     | Ligne de commande shell, limitée à 1000 caractères. Présent uniquement pour les tâches `shell`                                                                                                                                                                                |
| `agent_type`  | Nom du type de subagent. Présent uniquement pour les tâches `subagent`                                                                                                                                                                                                        |
| `server`      | Nom du serveur MCP. Présent uniquement pour les tâches `monitor` et `MCP task`                                                                                                                                                                                                |
| `tool`        | Nom de l'outil MCP. Présent uniquement pour les tâches `monitor` et `MCP task`                                                                                                                                                                                                |
| `name`        | Nom du workflow. Présent uniquement pour les tâches `workflow`                                                                                                                                                                                                                |

Chaque entrée dans `session_crons` décrit un réveil programmé limité à la session, provenant de `CronCreate`, `ScheduleWakeup` et `/loop` :

| Champ       | Description                                                                                                                                                       |
| :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Identifiant de la tâche cron                                                                                                                                      |
| `schedule`  | Expression cron, par exemple `0 9 * * 1-5`                                                                                                                        |
| `recurring` | `false` pour les réveils ponctuels dont le calendrier encode un seul moment de déclenchement, `true` pour les tâches qui se redéclenchent à chaque correspondance |
| `prompt`    | Prompt soumis lorsque le cron se déclenche, limité à 1000 caractères avec le même marqueur `… [+N chars]`                                                         |

Cet exemple montre une entrée Stop avec une tâche shell en vol et un cron récurrent :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": true,
  "last_assistant_message": "I've completed the refactoring. Here's a summary...",
  "background_tasks": [
    {
      "id": "task-001",
      "type": "shell",
      "status": "running",
      "description": "tail logs",
      "command": "tail -f /var/log/syslog"
    }
  ],
  "session_crons": [
    {
      "id": "cron-001",
      "schedule": "0 9 * * 1-5",
      "recurring": true,
      "prompt": "check the build"
    }
  ]
}
```

<h4 id="stop-decision-control">
  Contrôle de décision Stop
</h4>

Les hooks `Stop` et `SubagentStop` peuvent contrôler si Claude continue. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner ces champs spécifiques à l'événement :

| Champ                                  | Description                                                                                                                                                                                                                                 |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `decision`                             | `"block"` empêche Claude de s'arrêter. Omettez pour autoriser Claude à s'arrêter                                                                                                                                                            |
| `reason`                               | Requis lorsque `decision` est `"block"`. Indique à Claude pourquoi il doit continuer                                                                                                                                                        |
| `hookSpecificOutput.additionalContext` | Commentaires sans erreur pour Claude. La conversation continue afin que Claude puisse agir dessus, mais contrairement à `decision: "block"`, elle est affichée dans la transcription comme commentaire de hook plutôt qu'une erreur de hook |

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Utilisez `additionalContext` lorsque le hook fonctionne comme prévu et donne des conseils à Claude, comme « exécutez la suite de tests avant de terminer ». Cela garde la conversation en cours à travers les mêmes protections de boucle que `decision: "block"`, à savoir l'entrée `stop_hook_active` et le plafond de 8 continuations consécutives, mais la transcription l'étiquette `Stop hook feedback` et aucune notification d'erreur de hook n'est affichée :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "additionalContext": "Please run the test suite before finishing"
  }
}
```

<h3 id="stopfailure">
  StopFailure
</h3>

S'exécute à la place de [Stop](#stop) lorsque le tour se termine en raison d'une erreur API. La sortie et le code de sortie sont ignorés. Utilisez ceci pour enregistrer les défaillances, envoyer des alertes ou prendre des mesures de récupération lorsque Claude ne peut pas terminer une réponse en raison de limites de débit, de problèmes d'authentification ou d'autres erreurs API.

<h4 id="stopfailure-input">
  Entrée StopFailure
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks StopFailure reçoivent `error`, optionnellement `error_details` et optionnellement `last_assistant_message`. Le champ `error` identifie le type d'erreur et est utilisé pour le filtrage du matcher.

| Champ                    | Description                                                                                                                                                                                                                                                          |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Type d'erreur : `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens` ou `unknown`                                                                |
| `error_details`          | Détails supplémentaires sur l'erreur, le cas échéant                                                                                                                                                                                                                 |
| `last_assistant_message` | Le texte d'erreur rendu affiché dans la conversation. Contrairement à `Stop` et `SubagentStop`, où ce champ contient la sortie conversationnelle de Claude, pour `StopFailure` il contient la chaîne d'erreur API elle-même, comme `"API Error: Rate limit reached"` |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "StopFailure",
  "error": "rate_limit",
  "error_details": "429 Too Many Requests",
  "last_assistant_message": "API Error: Rate limit reached"
}
```

Les hooks StopFailure n'ont pas de contrôle de décision. Ils s'exécutent à des fins de notification et de journalisation uniquement.

<h3 id="teammateidle">
  TeammateIdle
</h3>

S'exécute lorsqu'un coéquipier d'une [équipe d'agents](/docs/fr/agent-teams) est sur le point de devenir inactif après avoir terminé son tour. Utilisez ceci pour appliquer des portes de qualité avant qu'un coéquipier ne cesse de travailler, comme exiger des vérifications de lint réussies ou vérifier que les fichiers de sortie existent.

Lorsqu'un hook `TeammateIdle` quitte avec le code 2, le coéquipier reçoit le message stderr comme commentaire et continue de travailler au lieu de devenir inactif. Pour arrêter complètement le coéquipier au lieu de le relancer, retournez JSON avec `{"continue": false, "stopReason": "..."}`. Les hooks TeammateIdle ne supportent pas les matchers et se déclenchent à chaque occurrence.

<h4 id="teammateidle-input">
  Entrée TeammateIdle
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks TeammateIdle reçoivent `teammate_name` et `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TeammateIdle",
  "teammate_name": "researcher",
  "team_name": "session-a1b2c3d4"
}
```

| Champ           | Description                                                                           |
| :-------------- | :------------------------------------------------------------------------------------ |
| `teammate_name` | Nom du coéquipier qui est sur le point de devenir inactif                             |
| `team_name`     | Dépréciée. Nom d'équipe dérivé de la session ; sera supprimée dans une version future |

<h4 id="teammateidle-decision-control">
  Contrôle de décision TeammateIdle
</h4>

Les hooks TeammateIdle supportent deux façons de contrôler le comportement du coéquipier :

* **Code de sortie 2** : le coéquipier reçoit le message stderr comme commentaire et continue de travailler au lieu de devenir inactif.
* **JSON `{"continue": false, "stopReason": "..."}`** : arrête complètement le coéquipier, correspondant au comportement du hook `Stop`. Le `stopReason` est affiché à l'utilisateur.

Cet exemple vérifie qu'un artefact de construction existe avant d'autoriser un coéquipier à devenir inactif :

```bash theme={null}
#!/bin/bash

if [ ! -f "./dist/output.js" ]; then
  echo "Build artifact missing. Run the build before stopping." >&2
  exit 2
fi

exit 0
```

<h3 id="configchange">
  ConfigChange
</h3>

S'exécute lorsqu'un fichier de configuration change pendant une session. Utilisez ceci pour auditer les modifications de paramètres, appliquer les politiques de sécurité ou bloquer les modifications non autorisées aux fichiers de configuration.

Les hooks ConfigChange se déclenchent pour les modifications des fichiers de paramètres, les paramètres de politique gérée et les fichiers de skill. Le champ `source` dans l'entrée vous indique quel type de configuration a changé, et le champ optionnel `file_path` fournit le chemin vers le fichier modifié.

Le matcher filtre sur la source de configuration :

| Matcher            | Quand il se déclenche                             |
| :----------------- | :------------------------------------------------ |
| `user_settings`    | `~/.claude/settings.json` change                  |
| `project_settings` | `.claude/settings.json` change                    |
| `local_settings`   | `.claude/settings.local.json` change              |
| `policy_settings`  | Les paramètres de politique gérée changent        |
| `skills`           | Un fichier de skill dans `.claude/skills/` change |

Cet exemple enregistre toutes les modifications de configuration pour l'audit de sécurité :

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/audit-config-change.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

<h4 id="configchange-input">
  Entrée ConfigChange
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks ConfigChange reçoivent `source` et optionnellement `file_path`. Le champ `source` indique quel type de configuration a changé, et `file_path` fournit le chemin vers le fichier spécifique qui a été modifié.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ConfigChange",
  "source": "project_settings",
  "file_path": "/Users/.../my-project/.claude/settings.json"
}
```

<h4 id="configchange-decision-control">
  Contrôle de décision ConfigChange
</h4>

Les hooks ConfigChange peuvent bloquer les modifications de configuration de prendre effet. Utilisez le code de sortie 2 ou une `decision` JSON pour empêcher la modification. Lorsqu'elle est bloquée, les nouveaux paramètres ne sont pas appliqués à la session en cours d'exécution.

| Champ      | Description                                                                                                 |
| :--------- | :---------------------------------------------------------------------------------------------------------- |
| `decision` | `"block"` empêche la modification de configuration d'être appliquée. Omettez pour autoriser la modification |
| `reason`   | Explication affichée à l'utilisateur lorsque `decision` est `"block"`                                       |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

Les modifications `policy_settings` ne peuvent pas être bloquées. Les hooks se déclenchent toujours pour les sources `policy_settings`, vous pouvez donc les utiliser pour la journalisation d'audit, mais toute décision de blocage est ignorée. Cela garantit que les paramètres gérés par l'entreprise prennent toujours effet.

<h3 id="cwdchanged">
  CwdChanged
</h3>

S'exécute lorsque le répertoire de travail change pendant une session, par exemple lorsque Claude exécute une commande `cd`. Utilisez ceci pour réagir aux changements de répertoire : recharger les variables d'environnement, activer les chaînes d'outils spécifiques au projet ou exécuter les scripts de configuration automatiquement. S'associe avec [FileChanged](#filechanged) pour les outils comme [direnv](https://direnv.net/) qui gèrent l'environnement par répertoire.

Les hooks CwdChanged ont accès à `CLAUDE_ENV_FILE`. Les variables écrites dans ce fichier persistent dans les commandes Bash suivantes pour la session, tout comme dans les [hooks SessionStart](#persist-environment-variables).

CwdChanged ne supporte pas les matchers et se déclenche à chaque changement de répertoire.

<h4 id="cwdchanged-input">
  Entrée CwdChanged
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks CwdChanged reçoivent `old_cwd` et `new_cwd`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project/src",
  "hook_event_name": "CwdChanged",
  "old_cwd": "/Users/my-project",
  "new_cwd": "/Users/my-project/src"
}
```

<h4 id="cwdchanged-output">
  Sortie CwdChanged
</h4>

En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, les hooks CwdChanged peuvent retourner `watchPaths` pour définir dynamiquement quels chemins de fichiers [FileChanged](#filechanged) surveille :

| Champ        | Description                                                                                                                                                                                                                                                                  |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Tableau de chemins absolus. Remplace la liste de surveillance dynamique actuelle. Les chemins de votre configuration `matcher` sont toujours surveillés. Retourner un tableau vide efface la liste dynamique, ce qui est typique lors de l'entrée dans un nouveau répertoire |

Les hooks CwdChanged n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer le changement de répertoire.

<h3 id="filechanged">
  FileChanged
</h3>

S'exécute lorsqu'un fichier surveillé change sur le disque. Utile pour recharger les variables d'environnement lorsque les fichiers de configuration du projet sont modifiés.

Le `matcher` pour cet événement sert deux rôles :

* **Construire la liste de surveillance** : la valeur est divisée sur `|` et chaque segment est enregistré comme un nom de fichier littéral dans le répertoire de travail, donc `".envrc|.env"` surveille exactement ces deux fichiers. Les modèles regex ne sont pas utiles ici : une valeur comme `^\.env` surveillerait un fichier littéralement nommé `^\.env`.
* **Filtrer quels hooks s'exécutent** : lorsqu'un fichier surveillé change, la même valeur filtre quels groupes de hook s'exécutent en utilisant les [règles de matcher](#matcher-patterns) standard par rapport au basename du fichier modifié.

Les hooks FileChanged ont accès à `CLAUDE_ENV_FILE`. Les variables écrites dans ce fichier persistent dans les commandes Bash suivantes pour la session, tout comme dans les [hooks SessionStart](#persist-environment-variables).

<h4 id="filechanged-input">
  Entrée FileChanged
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks FileChanged reçoivent `file_path` et `event`.

| Champ       | Description                                                                                                                  |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `file_path` | Chemin absolu vers le fichier qui a changé                                                                                   |
| `event`     | Ce qui s'est passé : `"change"` pour un fichier modifié, `"add"` pour un fichier créé ou `"unlink"` pour un fichier supprimé |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "FileChanged",
  "file_path": "/Users/my-project/.envrc",
  "event": "change"
}
```

<h4 id="filechanged-output">
  Sortie FileChanged
</h4>

En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, les hooks FileChanged peuvent retourner `watchPaths` pour mettre à jour dynamiquement quels chemins de fichiers sont surveillés :

| Champ        | Description                                                                                                                                                                                                                                                                           |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `watchPaths` | Tableau de chemins absolus. Remplace la liste de surveillance dynamique actuelle. Les chemins de votre configuration `matcher` sont toujours surveillés. Utilisez ceci lorsque votre script de hook découvre des fichiers supplémentaires à surveiller en fonction du fichier modifié |

Les hooks FileChanged n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer le changement de fichier de se produire.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

S'exécute lorsqu'un worktree est en cours de création, soit à partir de `claude --worktree` soit à partir d'un [subagent utilisant `isolation: "worktree"`](/docs/fr/sub-agents#choose-the-subagent-scope). Par défaut, Claude Code crée la copie de travail isolée avec `git worktree`. Configurer un hook WorktreeCreate remplace ce comportement git par défaut, vous permettant d'utiliser un système de contrôle de version différent comme SVN, Perforce ou Mercurial.

Parce que le hook remplace le comportement par défaut entièrement, [`.worktreeinclude`](/docs/fr/worktrees#copy-gitignored-files-into-worktrees) n'est pas traité. Si vous avez besoin de copier les fichiers de configuration locaux comme `.env` dans le nouveau worktree, faites-le à l'intérieur de votre script de hook.

Le hook doit retourner le chemin absolu du répertoire du worktree créé. Claude Code utilise ce chemin comme répertoire de travail pour la session isolée. Consultez [Sortie WorktreeCreate](#worktreecreate-output) pour savoir comment chaque type de hook retourne le chemin.

Cet exemple crée une copie de travail SVN et imprime le chemin pour que Claude Code l'utilise. Remplacez l'URL du référentiel par la vôtre :

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Le hook lit le `name` du worktree depuis l'entrée JSON sur stdin, extrait une copie fraîche dans un nouveau répertoire et imprime le chemin du répertoire. Le `echo` sur la dernière ligne est ce que Claude Code lit comme chemin du worktree. Redirigez toute autre sortie vers stderr afin qu'elle n'interfère pas avec le chemin.

<h4 id="worktreecreate-input">
  Entrée WorktreeCreate
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks WorktreeCreate reçoivent le champ `name`. C'est un identifiant slug pour le nouveau worktree, soit spécifié par l'utilisateur, soit généré automatiquement, par exemple `bold-oak-a3f2`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeCreate",
  "name": "feature-auth"
}
```

<h4 id="worktreecreate-output">
  Sortie WorktreeCreate
</h4>

Les hooks WorktreeCreate n'utilisent pas le modèle de décision autoriser/bloquer standard. Au lieu de cela, le succès ou l'échec du hook détermine le résultat. Le hook doit retourner le chemin absolu du répertoire du worktree créé :

* **Hooks de commande** (`type: "command"`) : imprimez le chemin comme la dernière ligne non-vide de stdout. Claude Code supprime les codes d'échappement ANSI avant de lire cette ligne, donc les bannières de démarrage du shell imprimées avant votre `echo` sont ignorées. Redirigez toute autre sortie du hook vers stderr.
* **Hooks HTTP** (`type: "http"`) : retournez `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` dans le corps de la réponse.

Si le hook échoue ou ne produit aucun chemin, la création du worktree échoue avec une erreur.

Claude Code résout un chemin relatif par rapport au répertoire dans lequel le hook s'est exécuté. Si le chemin résultant n'est pas un répertoire dans lequel Claude Code peut entrer, la session imprime une erreur nommant le chemin et quitte avec le code 1. Avant v2.1.205, un chemin relatif ou un chemin qui n'existait pas sur le disque plantait la session au démarrage, et avec `-p` il s'arrêtait pendant environ 30 secondes avant de quitter avec le code 0.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

S'exécute lorsqu'un worktree est en cours de suppression, soit lorsque vous quittez une session `--worktree` et choisissez de la supprimer, soit lorsqu'un subagent avec `isolation: "worktree"` se termine. C'est la contrepartie de nettoyage de [WorktreeCreate](#worktreecreate).

Pour les worktrees basés sur git, Claude Code gère le nettoyage automatiquement avec `git worktree remove`. Si vous avez configuré un hook WorktreeCreate pour un système de contrôle de version non-git, associez-le à un hook WorktreeRemove pour gérer le nettoyage. Sans lui, le répertoire du worktree est laissé sur le disque.

Claude Code transmet le chemin que WorktreeCreate a retourné comme `worktree_path` dans l'entrée du hook. Cet exemple lit ce chemin et supprime le répertoire :

```json theme={null}
{
  "hooks": {
    "WorktreeRemove": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'jq -r .worktree_path | xargs rm -rf'"
          }
        ]
      }
    ]
  }
}
```

<h4 id="worktreeremove-input">
  Entrée WorktreeRemove
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks WorktreeRemove reçoivent le champ `worktree_path`, qui est le chemin absolu du worktree en cours de suppression.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

Les hooks WorktreeRemove n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer la suppression du worktree mais peuvent effectuer des tâches de nettoyage comme supprimer l'état du contrôle de version ou archiver les modifications. Les défaillances des hooks sont enregistrées en mode debug uniquement.

<h3 id="precompact">
  PreCompact
</h3>

S'exécute avant que Claude Code ne soit sur le point d'exécuter une opération de compaction.

La valeur du matcher indique si la compaction a été déclenchée manuellement ou automatiquement :

| Matcher  | Quand il se déclenche                                            |
| :------- | :--------------------------------------------------------------- |
| `manual` | `/compact`                                                       |
| `auto`   | Compaction automatique lorsque la fenêtre de contexte est pleine |

Quittez avec le code 2 pour bloquer la compaction. Pour un `/compact` manuel, le message stderr est affiché à l'utilisateur. Vous pouvez également bloquer en retournant JSON avec `"decision": "block"`.

Le blocage de la compaction automatique a des effets différents selon le moment où il se déclenche. Si la compaction a été déclenchée de manière proactive avant la limite de contexte, Claude Code la saute et la conversation continue sans compaction. Si la compaction a été déclenchée pour récupérer d'une erreur de limite de contexte déjà retourné par l'API, l'erreur sous-jacente remonte et la demande actuelle échoue.

<h4 id="precompact-input">
  Entrée PreCompact
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PreCompact reçoivent `trigger` et `custom_instructions`. Pour `manual`, `custom_instructions` contient ce que l'utilisateur transmet dans `/compact`. Pour `auto`, `custom_instructions` est vide.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": ""
}
```

<h3 id="postcompact">
  PostCompact
</h3>

S'exécute après que Claude Code complète une opération de compaction. Utilisez cet événement pour réagir au nouvel état compacté, par exemple pour enregistrer le résumé généré ou mettre à jour l'état externe.

Les mêmes valeurs de matcher s'appliquent que pour `PreCompact` :

| Matcher  | Quand il se déclenche                                                  |
| :------- | :--------------------------------------------------------------------- |
| `manual` | Après `/compact`                                                       |
| `auto`   | Après compaction automatique lorsque la fenêtre de contexte est pleine |

<h4 id="postcompact-input">
  Entrée PostCompact
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PostCompact reçoivent `trigger` et `compact_summary`. Le champ `compact_summary` contient le résumé de conversation généré par l'opération de compaction.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostCompact",
  "trigger": "manual",
  "compact_summary": "Summary of the compacted conversation..."
}
```

Les hooks PostCompact n'ont pas de contrôle de décision. Ils ne peuvent pas affecter le résultat de la compaction mais peuvent effectuer des tâches de suivi.

<h3 id="sessionend">
  SessionEnd
</h3>

S'exécute lorsqu'une session Claude Code se termine. Utile pour les tâches de nettoyage, la journalisation des statistiques de session ou l'enregistrement de l'état de session. Supporte les matchers pour filtrer par raison de sortie.

Le champ `reason` dans l'entrée du hook indique pourquoi la session s'est terminée :

| Raison                        | Description                                                         |
| :---------------------------- | :------------------------------------------------------------------ |
| `clear`                       | Session effacée avec la commande `/clear`                           |
| `resume`                      | Session basculée via `/resume` interactif                           |
| `logout`                      | L'utilisateur s'est déconnecté                                      |
| `prompt_input_exit`           | L'utilisateur a quitté pendant que l'entrée du prompt était visible |
| `bypass_permissions_disabled` | Le mode de permissions de contournement a été désactivé             |
| `other`                       | Autres raisons de sortie                                            |

<h4 id="sessionend-input">
  Entrée SessionEnd
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks SessionEnd reçoivent un champ `reason` indiquant pourquoi la session s'est terminée. Consultez le [tableau des raisons](#sessionend) ci-dessus pour toutes les valeurs.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

Les hooks SessionEnd n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer la terminaison de session mais peuvent effectuer des tâches de nettoyage.

Les hooks SessionEnd ont un délai d'expiration par défaut de 1,5 secondes. Cela s'applique à la sortie de session, à `/clear` et au basculement de sessions via `/resume` interactif. Si un hook a besoin de plus de temps, définissez un `timeout` par hook dans la configuration du hook. Le budget global est automatiquement augmenté au délai d'expiration par hook le plus élevé configuré dans les fichiers de paramètres, jusqu'à 60 secondes. Les délais d'expiration définis sur les hooks fournis par les plugins ne relèvent pas le budget. Pour remplacer le budget explicitement, définissez la variable d'environnement `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` en millisecondes.

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

<h3 id="elicitation">
  Elicitation
</h3>

S'exécute lorsqu'un serveur MCP demande une entrée utilisateur en milieu de tâche. Par défaut, Claude Code affiche un dialogue interactif pour que l'utilisateur réponde. Les hooks peuvent intercepter cette demande et répondre par programmation, en ignorant complètement le dialogue.

Le champ matcher correspond au nom du serveur MCP.

<h4 id="elicitation-input">
  Entrée Elicitation
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks Elicitation reçoivent `mcp_server_name`, `message` et les champs optionnels `mode`, `url`, `elicitation_id` et `requested_schema`.

Pour l'élicitation en mode formulaire (le cas le plus courant) :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please provide your credentials",
  "mode": "form",
  "requested_schema": {
    "type": "object",
    "properties": {
      "username": { "type": "string", "title": "Username" }
    }
  }
}
```

Pour l'élicitation en mode URL (authentification basée sur navigateur) :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Sortie Elicitation
</h4>

Pour répondre par programmation sans afficher le dialogue, retournez un objet JSON avec `hookSpecificOutput` :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Elicitation",
    "action": "accept",
    "content": {
      "username": "alice"
    }
  }
}
```

| Champ     | Valeurs                       | Description                                                                                    |
| :-------- | :---------------------------- | :--------------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Si accepter, refuser ou annuler la demande                                                     |
| `content` | object                        | Valeurs des champs de formulaire à soumettre. Utilisé uniquement lorsque `action` est `accept` |

Le code de sortie 2 refuse l'élicitation et affiche stderr à l'utilisateur.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

S'exécute après qu'un utilisateur répond à une élicitation MCP. Les hooks peuvent observer, modifier ou bloquer la réponse avant qu'elle ne soit renvoyée au serveur MCP.

Le champ matcher correspond au nom du serveur MCP.

<h4 id="elicitationresult-input">
  Entrée ElicitationResult
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks ElicitationResult reçoivent `mcp_server_name`, `action` et les champs optionnels `mode`, `elicitation_id` et `content`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  Sortie ElicitationResult
</h4>

Pour remplacer la réponse de l'utilisateur, retournez un objet JSON avec `hookSpecificOutput` :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Champ     | Valeurs                       | Description                                                                                          |
| :-------- | :---------------------------- | :--------------------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Remplace l'action de l'utilisateur                                                                   |
| `content` | object                        | Remplace les valeurs des champs de formulaire. Significatif uniquement lorsque `action` est `accept` |

Le code de sortie 2 bloque la réponse, changeant l'action effective en `decline`.

<h2 id="prompt-based-hooks">
  Hooks basés sur des prompts
</h2>

En plus des hooks de commande, HTTP et MCP tool, Claude Code supporte les hooks basés sur des prompts (`type: "prompt"`) qui utilisent un LLM pour évaluer s'il faut autoriser ou bloquer une action, et les hooks d'agent (`type: "agent"`) qui lancent un vérificateur agentique avec accès aux outils. Tous les événements ne supportent pas tous les types de hooks.

Les événements qui supportent les cinq types de hooks (`command`, `http`, `mcp_tool`, `prompt` et `agent`) :

* `PermissionDenied`
* `PermissionRequest`
* `PostToolBatch`
* `PostToolUse`
* `PostToolUseFailure`
* `PreToolUse`
* `Stop`
* `SubagentStop`
* `TaskCompleted`
* `TaskCreated`
* `TeammateIdle`
* `UserPromptExpansion`
* `UserPromptSubmit`

Les événements qui supportent les hooks `command`, `http` et `mcp_tool` mais pas `prompt` ou `agent` :

* `ConfigChange`
* `CwdChanged`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `Notification`
* `PostCompact`
* `PreCompact`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` et `Setup` supportent les hooks `command` et `mcp_tool`. Ils ne supportent pas les hooks `http`, `prompt` ou `agent`.

<h3 id="how-prompt-based-hooks-work">
  Comment fonctionnent les hooks basés sur des prompts
</h3>

Au lieu d'exécuter une commande Bash, les hooks basés sur des prompts :

1. Envoient l'entrée du hook et votre prompt à un modèle Claude, Haiku par défaut
2. Le LLM répond avec JSON structuré contenant une décision
3. Claude Code traite automatiquement la décision

<h3 id="prompt-hook-configuration">
  Configuration des hooks de prompt
</h3>

Définissez `type` à `"prompt"` et fournissez une chaîne `prompt` au lieu d'une `command`. Utilisez le placeholder `$ARGUMENTS` pour injecter les données d'entrée JSON du hook dans votre texte de prompt. Claude Code envoie le prompt combiné et l'entrée à un modèle Claude rapide, qui retourne une décision JSON.

Ce hook `Stop` demande au LLM d'évaluer si toutes les tâches sont complètes avant d'autoriser Claude à terminer :

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Evaluate if Claude should stop: $ARGUMENTS. Check if all tasks are complete."
          }
        ]
      }
    ]
  }
}
```

| Champ             | Requis | Description                                                                                                                                                                                                                                                                                 |
| :---------------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`            | oui    | Doit être `"prompt"`                                                                                                                                                                                                                                                                        |
| `prompt`          | oui    | Le texte du prompt à envoyer au LLM. Utilisez `$ARGUMENTS` comme placeholder pour l'entrée JSON du hook. Si `$ARGUMENTS` n'est pas présent, l'entrée JSON est ajoutée au prompt                                                                                                             |
| `model`           | non    | Modèle à utiliser pour l'évaluation. Par défaut un modèle rapide                                                                                                                                                                                                                            |
| `timeout`         | non    | Délai d'expiration en secondes. Par défaut : 30                                                                                                                                                                                                                                             |
| `continueOnBlock` | non    | Lorsque le prompt retourne `ok: false`, renvoyer la raison à Claude et continuer le tour au lieu de s'arrêter. Par défaut : `false`. Implémenté comme `continue: true` sur la `decision: "block"` résultante. Voir [Schéma de réponse](#response-schema) pour le comportement par événement |

<h3 id="response-schema">
  Schéma de réponse
</h3>

Le LLM doit répondre avec JSON contenant :

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision"
}
```

| Champ    | Description                                                                                                   |
| :------- | :------------------------------------------------------------------------------------------------------------ |
| `ok`     | `true` pour autoriser. `false` produit une `decision: "block"`. Voir le comportement par événement ci-dessous |
| `reason` | Requis lorsque `ok` est `false`. Utilisé comme raison du blocage                                              |

Ce qui se passe sur `ok: false` dépend de l'événement :

* `Stop` et `SubagentStop` : la raison est renvoyée à Claude comme sa prochaine instruction et le tour continue
* `PreToolUse` : l'appel d'outil est refusé et la raison est retournée à Claude comme l'erreur de l'outil, équivalent à un hook de commande avec `permissionDecision: "deny"`
* `PostToolUse` : par défaut le tour se termine et la raison apparaît dans le chat comme une ligne d'avertissement. Définissez `continueOnBlock: true` pour renvoyer la raison à Claude et continuer le tour à la place
* `PostToolBatch`, `UserPromptSubmit` et `UserPromptExpansion` : le tour se termine et la raison apparaît comme une ligne d'avertissement. Ces événements terminent le tour sur `decision: "block"` indépendamment de `continue`
* `PostToolUseFailure`, `TaskCreated` et `TaskCompleted` : la raison est retournée à Claude comme une erreur d'outil, similaire à `PreToolUse`
* `TeammateIdle` : par défaut le coéquipier s'arrête et la raison apparaît comme une ligne d'avertissement. Définissez `continueOnBlock: true` pour renvoyer la raison au coéquipier et le garder actif à la place
* `PermissionRequest` : `ok: false` n'a aucun effet. Pour refuser une approbation d'un hook, utilisez un [hook de commande](#command-hook-fields) retournant `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied` : `ok: false` n'a aucun effet car le refus a déjà eu lieu. La seule sortie que cet événement lit est `hookSpecificOutput.retry`, que les hooks de prompt et d'agent ne peuvent pas définir. Ils s'exécutent sur cet événement, mais leur sortie est ignorée. Utilisez un [hook de commande](#command-hook-fields) pour retourner `retry`

Si vous avez besoin d'un contrôle plus fin sur un événement quelconque, utilisez un [hook de commande](#command-hook-fields) avec les champs par événement décrits dans [Contrôle de décision](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Vérifier plusieurs conditions avant d'arrêter
</h3>

Ce hook `Stop` utilise un prompt détaillé pour vérifier trois conditions avant d'autoriser Claude à s'arrêter. Les hooks `SubagentStop` utilisent le même format pour évaluer si un [subagent](/docs/fr/sub-agents) doit s'arrêter. Si `"ok"` est `false`, Claude continue de travailler avec la raison fournie comme sa prochaine instruction :

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "You are evaluating whether Claude should stop working. Context: $ARGUMENTS\n\nAnalyze the conversation and determine if:\n1. All user-requested tasks are complete\n2. Any errors need to be addressed\n3. Follow-up work is needed\n\nRespond with JSON: {\"ok\": true} to allow stopping, or {\"ok\": false, \"reason\": \"your explanation\"} to continue working.",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<h2 id="agent-based-hooks">
  Hooks basés sur des agents
</h2>

<Warning>
  Les hooks d'agent sont expérimentaux. Le comportement et la configuration peuvent changer dans les versions futures. Pour les workflows de production, préférez les [hooks de commande](#command-hook-fields).
</Warning>

Les hooks basés sur des agents (`type: "agent"`) sont comme les hooks basés sur des prompts mais avec accès aux outils multi-tours. Au lieu d'un seul appel LLM, un hook d'agent lance un subagent qui peut lire des fichiers, rechercher du code et inspecter la codebase pour vérifier les conditions. Les hooks d'agent supportent les mêmes événements que les hooks basés sur des prompts.

<h3 id="how-agent-hooks-work">
  Comment fonctionnent les hooks d'agent
</h3>

Lorsqu'un hook d'agent se déclenche :

1. Claude Code lance un subagent avec votre prompt et l'entrée JSON du hook
2. Le subagent peut utiliser des outils comme Read, Grep et Glob pour enquêter
3. Après jusqu'à 50 tours, le subagent retourne une décision structurée `{ "ok": true/false }`
4. Claude Code traite la décision de la même manière qu'un hook de prompt

Les hooks d'agent sont utiles lorsque la vérification nécessite d'inspecter les fichiers réels ou la sortie des tests, pas seulement d'évaluer les données d'entrée du hook seules.

<h3 id="agent-hook-configuration">
  Configuration des hooks d'agent
</h3>

Définissez `type` à `"agent"` et fournissez une chaîne `prompt`. Les champs de configuration sont les mêmes que les [hooks de prompt](#prompt-hook-configuration), avec un délai d'expiration par défaut plus long :

| Champ     | Requis | Description                                                                                        |
| :-------- | :----- | :------------------------------------------------------------------------------------------------- |
| `type`    | oui    | Doit être `"agent"`                                                                                |
| `prompt`  | oui    | Prompt décrivant ce à vérifier. Utilisez `$ARGUMENTS` comme placeholder pour l'entrée JSON du hook |
| `model`   | non    | Modèle à utiliser. Par défaut un modèle rapide                                                     |
| `timeout` | non    | Délai d'expiration en secondes. Par défaut : 60                                                    |

Le schéma de réponse est le même que les hooks de prompt : `{ "ok": true }` pour autoriser ou `{ "ok": false, "reason": "..." }` pour bloquer.

Ce hook `Stop` vérifie que tous les tests unitaires réussissent avant d'autoriser Claude à terminer :

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

<h2 id="run-hooks-in-the-background">
  Exécuter les hooks en arrière-plan
</h2>

Par défaut, les hooks bloquent l'exécution de Claude jusqu'à ce qu'ils se terminent. Pour les tâches longues comme les déploiements, les suites de tests ou les appels API externes, définissez `"async": true` pour exécuter le hook en arrière-plan tandis que Claude continue de travailler. Les hooks asynchrones ne peuvent pas bloquer ou contrôler le comportement de Claude : les champs de réponse comme `decision`, `permissionDecision` et `continue` n'ont aucun effet, car l'action qu'ils auraient contrôlée s'est déjà produite.

<h3 id="configure-an-async-hook">
  Configurer un hook asynchrone
</h3>

Ajoutez `"async": true` à la configuration d'un hook de commande pour l'exécuter en arrière-plan sans bloquer Claude. Ce champ n'est disponible que sur les hooks `type: "command"`.

Ce hook exécute un script de test après chaque appel d'outil `Write`. Claude continue de travailler immédiatement tandis que `run-tests.sh` s'exécute pendant jusqu'à 120 secondes. Lorsque le script se termine, sa sortie est livrée au tour de conversation suivant :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/run-tests.sh",
            "async": true,
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

Le champ `timeout` définit le temps maximum en secondes pour le processus en arrière-plan. S'il n'est pas spécifié, les hooks asynchrones utilisent la même valeur par défaut de 10 minutes que les hooks synchrones.

<h3 id="how-async-hooks-execute">
  Comment les hooks asynchrones s'exécutent
</h3>

Lorsqu'un hook asynchrone se déclenche, Claude Code démarre le processus du hook et continue immédiatement sans attendre qu'il se termine. Le hook reçoit la même entrée JSON via stdin qu'un hook synchrone.

Après la sortie du processus en arrière-plan, si le hook a produit une réponse JSON avec un champ `additionalContext`, ce contenu est livré à Claude comme contexte au tour de conversation suivant. Un champ `systemMessage` vous est montré, pas à Claude.

Claude Code valide que la réponse JSON respecte le même [schéma de sortie](#json-output) que les hooks synchrones, et supprime tout champ dont la valeur a le mauvais type, comme un `systemMessage` qui n'est pas une chaîne de caractères, au lieu de le livrer. Exécutez avec `--debug` pour voir un avertissement nommant chaque champ supprimé. Avant la v2.1.202, une sortie JSON malformée d'un hook asynchrone pouvait faire planter la session, et le plantage s'est reproduit chaque fois que la session a été reprise.

Les notifications d'achèvement des hooks asynchrones sont supprimées par défaut. Pour les voir, activez le mode verbeux avec `Ctrl+O` ou démarrez Claude Code avec `--verbose`.

<h3 id="run-tests-after-file-changes">
  Exécuter les tests après les modifications de fichiers
</h3>

Ce hook démarre une suite de tests en arrière-plan chaque fois que Claude écrit un fichier, puis rapporte les résultats à Claude lorsque les tests se terminent. Enregistrez ce script dans `.claude/hooks/run-tests-async.sh` dans votre projet et rendez-le exécutable avec `chmod +x` :

```bash theme={null}
#!/bin/bash
# run-tests-async.sh

# Lisez l'entrée du hook depuis stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Exécutez les tests uniquement pour les fichiers source
if [[ "$FILE_PATH" != *.ts && "$FILE_PATH" != *.js ]]; then
  exit 0
fi

# Exécutez les tests et rapportez les résultats à Claude via additionalContext
RESULT=$(npm test 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  MSG="Tests passed after editing $FILE_PATH"
else
  MSG="Tests failed after editing $FILE_PATH: $RESULT"
fi
jq -nc --arg msg "$MSG" '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $msg}}'
```

Ensuite, ajoutez cette configuration à `.claude/settings.json` dans la racine de votre projet. Le drapeau `async: true` permet à Claude de continuer à travailler pendant que les tests s'exécutent :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/run-tests-async.sh",
            "args": [],
            "async": true,
            "timeout": 300
          }
        ]
      }
    ]
  }
}
```

<h3 id="limitations">
  Limitations
</h3>

Les hooks asynchrones ont plusieurs contraintes par rapport aux hooks synchrones :

* Seuls les hooks `type: "command"` supportent `async`. Les hooks basés sur des prompts ne peuvent pas s'exécuter de manière asynchrone.
* Les hooks asynchrones ne peuvent pas bloquer les appels d'outil ou retourner des décisions. Au moment où le hook se termine, l'action qui l'a déclenché a déjà procédé.
* La sortie du hook est livrée au tour de conversation suivant. Si la session est inactive, la réponse attend jusqu'à la prochaine interaction utilisateur. Exception : un hook `asyncRewake` qui quitte avec le code 2 réveille Claude immédiatement même lorsque la session est inactive.
* Chaque exécution crée un processus en arrière-plan séparé. Il n'y a pas de déduplication sur plusieurs déclenchements du même hook asynchrone.

<h2 id="security-considerations">
  Considérations de sécurité
</h2>

<h3 id="disclaimer">
  Avertissement
</h3>

Les hooks de commande s'exécutent avec les permissions complètes de votre utilisateur système.

<Warning>
  Les hooks de commande exécutent les commandes shell avec vos permissions utilisateur complètes. Ils peuvent modifier, supprimer ou accéder à tous les fichiers auxquels votre compte utilisateur peut accéder. Examinez et testez toutes les commandes de hook avant de les ajouter à votre configuration.
</Warning>

<h3 id="security-best-practices">
  Meilleures pratiques de sécurité
</h3>

Gardez ces pratiques à l'esprit lors de l'écriture de hooks :

* **Validez et nettoyez les entrées** : ne faites jamais confiance aux données d'entrée aveuglément
* **Citez toujours les variables shell** : utilisez `"$VAR"` pas `$VAR`
* **Bloquez la traversée de répertoires** : vérifiez les `..` dans les chemins de fichiers
* **Utilisez les chemins absolus** : spécifiez les chemins complets pour les scripts. En forme exec, utilisez `${CLAUDE_PROJECT_DIR}` et le chemin n'a pas besoin de guillemets. En forme shell, enveloppez-le dans des guillemets doubles
* **Ignorez les fichiers sensibles** : évitez `.env`, `.git/`, les clés, etc.

<h2 id="windows-powershell-tool">
  Outil PowerShell sur Windows
</h2>

Sur Windows, vous pouvez exécuter les hooks individuels dans PowerShell en définissant `"shell": "powershell"` sur un hook de commande. Les hooks lancent PowerShell directement, donc cela fonctionne indépendamment de la définition de `CLAUDE_CODE_USE_POWERSHELL_TOOL`. Claude Code détecte automatiquement `pwsh.exe`, l'exécutable PowerShell 7 et versions ultérieures, et bascule vers `powershell.exe` pour Windows PowerShell 5.1.

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "shell": "powershell",
            "command": "Write-Host 'File written'"
          }
        ]
      }
    ]
  }
}
```

Pour référencer la racine du projet à partir d'une commande PowerShell en forme shell, écrivez `${CLAUDE_PROJECT_DIR}` ou `$env:CLAUDE_PROJECT_DIR`. À partir de la v2.1.198, Claude Code réécrit les placeholders `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}` et `${CLAUDE_PLUGIN_DATA}` dans une commande PowerShell en forme shell vers la forme `${env:NAME}` de PowerShell, que le hook soit défini dans `settings.json`, un plugin ou une skill. PowerShell résout ensuite la valeur à partir de l'environnement exporté après l'analyse, donc le placeholder fonctionne à l'intérieur des chaînes entre guillemets doubles mais pas à l'intérieur des chaînes entre guillemets simples, où PowerShell n'étend jamais les variables.

Avant la v2.1.198, cette réécriture s'appliquait uniquement aux hooks de plugin. Sur les versions antérieures, un hook `settings.json` a besoin de la forme `$env:` ou de la [forme exec](#exec-form-and-shell-form), où `${CLAUDE_PROJECT_DIR}` est substitué dans chaque élément `args` indépendamment de l'endroit où le hook est défini.

N'écrivez pas l'orthographe nue `$CLAUDE_PROJECT_DIR` dans un hook PowerShell. PowerShell l'analyse comme une variable locale indéfinie et la résout en `$null`, ce qui laisse le chemin du script sans son préfixe de racine de projet. Claude Code ne réécrit pas cette forme ; il enregistre plutôt un avertissement dans le [journal de débogage](#debug-hooks).

L'exemple ci-dessous montre un hook `settings.json` qui exécute un script de projet avec la forme `$env:`, qui fonctionne sur chaque version :

```json theme={null}
{
  "type": "command",
  "shell": "powershell",
  "command": "& \"$env:CLAUDE_PROJECT_DIR\\.claude\\hooks\\check.ps1\""
}
```

<h2 id="debug-hooks">
  Déboguer les hooks
</h2>

Les détails d'exécution des hooks, y compris les hooks qui ont correspondu, leurs codes de sortie et la sortie complète stdout et stderr, sont écrits dans le fichier journal de débogage. Démarrez Claude Code avec `claude --debug-file <path>` pour écrire le journal à un emplacement connu, ou exécutez `claude --debug` et lisez le journal à `~/.claude/debug/<session-id>.txt`. Le drapeau `--debug` n'imprime pas sur le terminal.

```text theme={null}
[DEBUG] Executing hooks for PostToolUse:Write
[DEBUG] Found 1 hook commands to execute
[DEBUG] Executing hook command: <Your command> with timeout 600000ms
[DEBUG] Hook command completed with status 0: <Your stdout>
```

Pour plus de détails granulaires sur la correspondance des hooks, définissez `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` pour voir des lignes de journal supplémentaires telles que les comptes de matcher de hook et la correspondance de requête.

Pour dépanner les problèmes courants comme les hooks qui ne se déclenchent pas, les hooks Stop qui continuent à bloquer, ou les erreurs de configuration, consultez [Limitations et dépannage](/docs/fr/hooks-guide#limitations-and-troubleshooting) dans le guide. Pour une procédure de diagnostic plus large couvrant `/context`, `/doctor` et la précédence des paramètres, consultez [Déboguer votre configuration](/docs/fr/debug-your-config).
