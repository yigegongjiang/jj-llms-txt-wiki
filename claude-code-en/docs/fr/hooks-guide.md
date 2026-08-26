> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatiser les actions avec les hooks

> Exécutez automatiquement des commandes shell lorsque Claude Code modifie des fichiers, termine des tâches ou a besoin d'une entrée. Formatez le code, envoyez des notifications, validez les commandes et appliquez les règles du projet.

Les hooks sont des commandes shell définies par l'utilisateur qui s'exécutent à des points spécifiques du cycle de vie de Claude Code. Ils fournissent un contrôle déterministe du comportement de Claude Code, en garantissant que certaines actions se produisent toujours plutôt que de compter sur le LLM pour choisir de les exécuter. Utilisez les hooks pour appliquer les règles du projet, automatiser les tâches répétitives et intégrer Claude Code avec vos outils existants.

Pour les décisions qui nécessitent un jugement plutôt que des règles déterministes, vous pouvez également utiliser des [hooks basés sur des invites](#prompt-based-hooks) ou des [hooks basés sur des agents](#agent-based-hooks) qui utilisent un modèle Claude pour évaluer les conditions.

Pour d'autres façons d'étendre Claude Code, consultez [skills](/docs/fr/skills) pour donner à Claude des instructions supplémentaires et des commandes exécutables, [subagents](/docs/fr/sub-agents) pour exécuter des tâches dans des contextes isolés, et [plugins](/docs/fr/plugins) pour empaqueter les extensions à partager entre les projets.

<Tip>
  Ce guide couvre les cas d'usage courants et comment commencer. Pour les schémas d'événements complets, les formats d'entrée/sortie JSON et les fonctionnalités avancées comme les hooks asynchrones et les hooks d'outils MCP, consultez la [référence des Hooks](/docs/fr/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Configurer votre premier hook
</h2>

Pour créer un hook, ajoutez un bloc `hooks` à un [fichier de paramètres](#configure-hook-location). Cette procédure crée un hook de notification de bureau, afin que vous soyez alerté chaque fois que Claude attend votre entrée au lieu de regarder le terminal.

<Steps>
  <Step title="Ajouter le hook à vos paramètres">
    Ouvrez `~/.claude/settings.json` et ajoutez un hook `Notification`. L'exemple ci-dessous utilise `osascript` pour macOS ; consultez [Être notifié lorsque Claude a besoin d'une entrée](#get-notified-when-claude-needs-input) pour les commandes Linux et Windows.

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    Si votre fichier de paramètres a déjà une clé `hooks`, ajoutez `Notification` comme frère des clés d'événement existantes plutôt que de remplacer l'objet entier. Chaque nom d'événement est une clé à l'intérieur du seul objet `hooks` :

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    Vous pouvez également demander à Claude d'écrire le hook pour vous en décrivant ce que vous voulez dans le CLI.
  </Step>

  <Step title="Vérifier la configuration">
    Tapez `/hooks` pour ouvrir le navigateur des hooks. Vous verrez une liste de tous les événements de hook disponibles, avec un nombre à côté de chaque événement qui a des hooks configurés. Sélectionnez `Notification` pour confirmer que votre nouveau hook apparaît dans la liste. La sélection du hook affiche ses détails : l'événement, le matcher, le type, le fichier source et la commande.
  </Step>

  <Step title="Tester le hook">
    Appuyez sur `Esc` pour revenir au CLI. Demandez à Claude de faire quelque chose qui nécessite une permission, puis quittez le terminal. Vous devriez recevoir une notification de bureau.
  </Step>
</Steps>

<Tip>
  Le menu `/hooks` est en lecture seule. Pour ajouter, modifier ou supprimer des hooks, modifiez votre JSON de paramètres directement ou demandez à Claude de faire la modification.
</Tip>

<h2 id="what-you-can-automate">
  Ce que vous pouvez automatiser
</h2>

Les hooks vous permettent d'exécuter du code à des points clés du cycle de vie de Claude Code : formater les fichiers après les modifications, bloquer les commandes avant leur exécution, envoyer des notifications lorsque Claude a besoin d'une entrée, injecter du contexte au démarrage de la session, et bien plus. Pour la liste complète des événements de hook, consultez la [référence des Hooks](/docs/fr/hooks#hook-lifecycle).

Chaque exemple inclut un bloc de configuration prêt à l'emploi que vous ajoutez à un [fichier de paramètres](#configure-hook-location).

Pour un exemple de production de hooks qui exécutent un examen de modèle séparé et renvoient les résultats dans la session, consultez [comment le plugin `security-guidance` s'intègre à Claude Code](/docs/fr/security-guidance#how-the-plugin-integrates-with-claude-code).

<h3 id="get-notified-when-claude-needs-input">
  Être notifié lorsque Claude a besoin d'une entrée
</h3>

Recevez une notification de bureau chaque fois que Claude termine son travail et a besoin de votre entrée, afin que vous puissiez passer à d'autres tâches sans vérifier le terminal.

Ce hook utilise l'événement `Notification`, qui se déclenche lorsque Claude attend une entrée ou une permission. Chaque onglet ci-dessous utilise la commande de notification native de la plateforme. Ajoutez ceci à `~/.claude/settings.json` :

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="Si aucune notification n'apparaît">
      `osascript` achemine les notifications via l'application Script Editor intégrée. Si Script Editor n'a pas la permission de notification, la commande échoue silencieusement, et macOS ne vous demandera pas de l'accorder. Exécutez ceci dans Terminal une fois pour que Script Editor apparaisse dans vos paramètres de notification :

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      Rien n'apparaîtra pour l'instant. Ouvrez **Paramètres système > Notifications**, trouvez **Script Editor** dans la liste, et activez **Autoriser les notifications**. Exécutez la commande à nouveau pour confirmer que la notification de test apparaît.
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Le matcher vide se déclenche sur tous les types de notification. Pour se déclencher uniquement sur des événements spécifiques, définissez-le sur l'une de ces valeurs :

| Matcher                | Se déclenche quand                                                                                                                       |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`    | Claude a besoin que vous approuviez un appel d'outil                                                                                     |
| `idle_prompt`          | Claude a terminé et attend votre prochaine invite                                                                                        |
| `auth_success`         | L'authentification se termine                                                                                                            |
| `elicitation_dialog`   | Un serveur MCP ouvre un formulaire d'élicitation                                                                                         |
| `elicitation_complete` | Un formulaire d'élicitation MCP est soumis ou fermé                                                                                      |
| `elicitation_response` | Une réponse d'élicitation MCP est renvoyée au serveur                                                                                    |
| `agent_needs_input`    | Une session en arrière-plan commence à attendre votre entrée. Se déclenche uniquement lorsque la [vue agent](/docs/fr/agent-view) est ouverte |
| `agent_completed`      | Une session en arrière-plan se termine ou échoue. Se déclenche uniquement lorsque la [vue agent](/docs/fr/agent-view) est ouverte             |

Les matchers `agent_needs_input` et `agent_completed` nécessitent Claude Code v2.1.198 ou version ultérieure.

Tapez `/hooks` et sélectionnez `Notification` pour confirmer que le hook est enregistré. Pour le schéma d'événement complet, consultez la [référence Notification](/docs/fr/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Formater automatiquement le code après les modifications
</h3>

Exécutez automatiquement [Prettier](https://prettier.io/) sur chaque fichier que Claude modifie, afin que le formatage reste cohérent sans intervention manuelle.

Ce hook utilise l'événement `PostToolUse` avec un matcher `Edit|Write`, il s'exécute donc uniquement après les outils d'édition de fichiers. La commande extrait le chemin du fichier modifié avec [`jq`](https://jqlang.github.io/jq/) et le transmet à Prettier. Ajoutez ceci à `.claude/settings.json` à la racine de votre projet :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

Sur Claude Code v2.1.191 ou version ultérieure, vous pouvez également écrire le matcher sous la forme `Edit,Write`, puisque `|` et `,` sont des séparateurs de liste interchangeables pour les matchers de noms d'outils sur ces versions.

<Note>
  Les exemples Bash sur cette page utilisent `jq` pour l'analyse JSON. Installez-le avec `brew install jq` sur macOS, `apt-get install jq` sur Debian et Ubuntu, ou consultez les [téléchargements de `jq`](https://jqlang.github.io/jq/download/).
</Note>

<h3 id="block-edits-to-protected-files">
  Bloquer les modifications des fichiers protégés
</h3>

Empêchez Claude de modifier les fichiers sensibles comme `.env`, `package-lock.json`, ou n'importe quoi dans `.git/`. Claude reçoit un retour expliquant pourquoi la modification a été bloquée, afin qu'il puisse ajuster son approche.

Cet exemple utilise un fichier de script séparé que le hook appelle. Le script vérifie le chemin du fichier cible par rapport à une liste de modèles protégés et quitte avec le code 2 pour bloquer la modification.

<Steps>
  <Step title="Créer le script du hook">
    Enregistrez ceci dans `.claude/hooks/protect-files.sh` :

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="Rendre le script exécutable sur macOS et Linux">
    Les scripts de hook doivent être exécutables pour que Claude Code les exécute :

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="Enregistrer le hook">
    Ajoutez un hook `PreToolUse` à `.claude/settings.json` qui exécute le script avant n'importe quel appel d'outil `Edit` ou `Write` :

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Réinjecter le contexte après compaction
</h3>

Lorsque la fenêtre de contexte de Claude se remplit, la compaction résume la conversation pour libérer de l'espace. Cela peut perdre des détails importants. Utilisez un hook `SessionStart` avec un matcher `compact` pour réinjecter le contexte critique après chaque compaction.

Tout texte que votre commande écrit sur stdout est ajouté au contexte de Claude. Cet exemple rappelle à Claude les conventions du projet et le travail récent. Ajoutez ceci à `.claude/settings.json` à la racine de votre projet :

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

Vous pouvez remplacer `echo` par n'importe quelle commande qui produit une sortie dynamique, comme `git log --oneline -5` pour afficher les commits récents. Pour injecter du contexte au démarrage de chaque session, envisagez d'utiliser [CLAUDE.md](/docs/fr/memory) à la place. Pour les variables d'environnement, consultez [`CLAUDE_ENV_FILE`](/docs/fr/hooks#persist-environment-variables) dans la référence.

<h3 id="audit-configuration-changes">
  Auditer les modifications de configuration
</h3>

Suivez quand les fichiers de paramètres ou de skills changent pendant une session. L'événement `ConfigChange` se déclenche lorsqu'un processus externe ou un éditeur modifie un fichier de configuration, afin que vous puissiez enregistrer les modifications pour la conformité ou bloquer les modifications non autorisées.

Cet exemple ajoute chaque modification à un journal d'audit. Ajoutez ceci à `~/.claude/settings.json` :

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

Le matcher filtre par type de configuration : `user_settings`, `project_settings`, `local_settings`, `policy_settings`, ou `skills`. Pour bloquer une modification de prendre effet, quittez avec le code 2 ou retournez `{"decision": "block"}`. Consultez la [référence ConfigChange](/docs/fr/hooks#configchange) pour le schéma d'entrée complet.

<h3 id="reload-environment-when-directory-or-files-change">
  Recharger l'environnement lorsque le répertoire ou les fichiers changent
</h3>

Certains projets définissent des variables d'environnement différentes selon le répertoire dans lequel vous vous trouvez. Des outils comme [direnv](https://direnv.net/) le font automatiquement dans votre shell, mais l'outil Bash de Claude ne récupère pas ces modifications de lui-même.

L'association d'un hook `SessionStart` avec un hook `CwdChanged` corrige cela. `SessionStart` charge les variables pour le répertoire dans lequel vous lancez, et `CwdChanged` les recharge chaque fois que Claude change de répertoire. Les deux écrivent dans `CLAUDE_ENV_FILE`, que Claude Code exécute comme un préambule de script avant chaque commande Bash. Ajoutez ceci à `~/.claude/settings.json` :

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Exécutez `direnv allow` une fois dans chaque répertoire qui a un `.envrc` afin que direnv soit autorisé à le charger. Si vous utilisez devbox ou nix à la place de direnv, le même modèle fonctionne avec `devbox shellenv` ou `devbox global shellenv` à la place de `direnv export bash`.

Pour réagir à des fichiers spécifiques au lieu de chaque changement de répertoire, utilisez `FileChanged` avec un `matcher` listant les noms de fichiers à surveiller, séparés par `|`. Lors de la construction de la liste de surveillance, Claude Code divise cette valeur en noms de fichiers littéraux plutôt que de l'évaluer comme une regex. Consultez [FileChanged](/docs/fr/hooks#filechanged) pour savoir comment la même valeur filtre également les groupes de hooks qui s'exécutent lorsqu'un fichier change. Cet exemple surveille `.envrc` et `.env` dans le répertoire de travail :

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Consultez les entrées de référence [CwdChanged](/docs/fr/hooks#cwdchanged) et [FileChanged](/docs/fr/hooks#filechanged) pour les schémas d'entrée, la sortie `watchPaths`, et les détails de `CLAUDE_ENV_FILE`.

<h3 id="auto-approve-specific-permission-prompts">
  Approuver automatiquement les invites de permission spécifiques
</h3>

Ignorez la boîte de dialogue d'approbation pour les appels d'outils que vous autorisez toujours. Cet exemple approuve automatiquement `ExitPlanMode`, l'outil que Claude appelle lorsqu'il termine de présenter un plan et demande de procéder, afin que vous ne soyez pas invité à chaque fois qu'un plan est prêt.

Contrairement aux exemples de code de sortie ci-dessus, l'approbation automatique nécessite que votre hook écrive une décision JSON sur stdout. Un hook `PermissionRequest` se déclenche lorsque Claude Code est sur le point d'afficher une boîte de dialogue de permission, et retourner `"behavior": "allow"` y répond en votre nom.

Le matcher limite le hook à `ExitPlanMode` uniquement, afin qu'aucune autre invite ne soit affectée. Ajoutez ceci à `~/.claude/settings.json` :

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

Lorsque le hook approuve, Claude Code quitte le mode plan et restaure le mode de permission qui était actif avant que vous entriez en mode plan. La transcription affiche « Allowed by PermissionRequest hook » où la boîte de dialogue aurait apparu. Le chemin du hook garde toujours la conversation actuelle : il ne peut pas effacer le contexte et démarrer une session d'implémentation fraîche comme la boîte de dialogue peut le faire.

Pour définir un mode de permission spécifique à la place, la sortie de votre hook peut inclure un tableau `updatedPermissions` avec une entrée `setMode`. La valeur `mode` est n'importe quel mode de permission comme `default`, `acceptEdits`, ou `bypassPermissions`, et `destination: "session"` l'applique pour la session actuelle uniquement.

<Note>
  `bypassPermissions` ne s'applique que si la session a été lancée avec le mode bypass déjà disponible : `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, ou `permissions.defaultMode: "bypassPermissions"` dans les paramètres, et non désactivé par [`permissions.disableBypassPermissionsMode`](/docs/fr/permissions#managed-settings). Il n'est jamais persisté en tant que `defaultMode`.
</Note>

Pour basculer la session vers `acceptEdits`, votre hook écrit ce JSON sur stdout :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

Gardez le matcher aussi étroit que possible. Correspondre à `.*` ou laisser le matcher vide approuverait automatiquement chaque invite de permission, y compris les écritures de fichiers et les commandes shell. Consultez la [référence PermissionRequest](/docs/fr/hooks#permissionrequest-decision-control) pour l'ensemble complet des champs de décision.

<h2 id="how-hooks-work">
  Comment fonctionnent les hooks
</h2>

Les événements de hook se déclenchent à des points spécifiques du cycle de vie de Claude Code. Lorsqu'un événement se déclenche, tous les hooks correspondants s'exécutent en parallèle, et les commandes de hook identiques sont automatiquement dédupliquées. Le tableau ci-dessous montre chaque événement et quand il se déclenche :

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

Chaque hook a un `type` qui détermine comment il s'exécute. La plupart des hooks utilisent `"type": "command"`, qui exécute une commande shell. Quatre autres types sont disponibles :

* `"type": "http"` : POST les données d'événement vers une URL. Consultez [Hooks HTTP](#http-hooks).
* `"type": "mcp_tool"` : appeler un outil sur un serveur MCP déjà connecté. Consultez [Champs de hooks d'outil MCP](/docs/fr/hooks#mcp-tool-hook-fields).
* `"type": "prompt"` : évaluation LLM à un seul tour. Consultez [Hooks basés sur des invites](#prompt-based-hooks).
* `"type": "agent"` : vérification multi-tour avec accès aux outils. Les hooks d'agent sont expérimentaux et peuvent changer. Consultez [Hooks basés sur des agents](#agent-based-hooks).

<h3 id="combine-results-from-multiple-hooks">
  Combiner les résultats de plusieurs hooks
</h3>

Lorsque plusieurs hooks correspondent au même événement, la commande de chaque hook s'exécute jusqu'à son terme avant que Claude Code ne fusionne les résultats. Un hook retournant `deny` n'empêche pas les hooks frères de s'exécuter. Ne comptez pas sur le `deny` d'un hook pour supprimer les effets secondaires dans un autre hook.

Après que tous les hooks correspondants se terminent, Claude Code combine leurs résultats. Pour les décisions de permission `PreToolUse`, la réponse la plus restrictive gagne, dans l'ordre `deny`, `defer`, `ask`, `allow`. Le texte de `additionalContext` est conservé de chaque hook et transmis à Claude ensemble.

L'exemple ci-dessous enregistre deux hooks `PreToolUse` sur `Bash`. Le premier ajoute chaque commande à un fichier journal et quitte avec le code 0. Le second exécute un script qui quitte avec le code 2 pour refuser lorsque la commande contient `rm -rf` :

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

Lorsque Claude essaie d'exécuter `rm -rf /tmp/build`, les deux hooks s'exécutent en parallèle. Le hook de journalisation écrit la commande dans `~/.claude/bash.log` et quitte avec le code 0, ce qui ne signale aucune décision. Le hook de garde-fou quitte avec le code 2, ce qui refuse l'appel d'outil. Le refus gagne, donc Claude Code bloque la commande et affiche à Claude le stderr du garde-fou. L'entrée du journal est toujours écrite car le hook de journalisation a déjà s'exécuté.

<h3 id="read-input-and-return-output">
  Lire l'entrée et retourner la sortie
</h3>

Les hooks communiquent avec Claude Code via stdin, stdout, stderr et les codes de sortie. Lorsqu'un événement se déclenche, Claude Code transmet les données spécifiques à l'événement en JSON à stdin de votre script. Votre script lit ces données, fait son travail, et dit à Claude Code quoi faire ensuite via le code de sortie.

<h4 id="hook-input">
  Entrée du hook
</h4>

Chaque événement inclut des champs communs comme `session_id` et `cwd`, mais chaque type d'événement ajoute des données différentes. Par exemple, lorsque Claude exécute une commande Bash, un hook `PreToolUse` reçoit quelque chose comme ceci sur stdin :

```json theme={null}
{
  "session_id": "abc123",          // unique ID for this session
  "cwd": "/Users/sarah/myproject", // working directory when the event fired
  "hook_event_name": "PreToolUse", // which event triggered this hook
  "tool_name": "Bash",             // the tool Claude is about to use
  "tool_input": {                  // the arguments Claude passed to the tool
    "command": "npm test"          // for Bash, this is the shell command
  }
}
```

Votre script peut analyser ce JSON et agir sur n'importe lequel de ces champs. Les hooks `UserPromptSubmit` obtiennent le texte `prompt` à la place, les hooks `SessionStart` obtiennent la `source` (startup, resume, clear, compact), et ainsi de suite. Consultez [Champs d'entrée communs](/docs/fr/hooks#common-input-fields) dans la référence pour les champs partagés, et la section de chaque événement pour les schémas spécifiques à l'événement.

<h4 id="hook-output">
  Sortie du hook
</h4>

Votre script dit à Claude Code quoi faire ensuite en écrivant sur stdout ou stderr et en quittant avec un code spécifique. Par exemple, un hook `PreToolUse` qui veut bloquer une commande :

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr becomes Claude's feedback
  exit 2 # exit 2 = block the action
fi

exit 0  # exit 0 = no decision; the normal permission flow applies
```

Le code de sortie détermine ce qui se passe ensuite :

* **Exit 0** : le hook ne signale aucune objection et l'action se poursuit normalement. Pour un hook `PreToolUse`, cela n'approuve pas l'appel d'outil : le [flux de permission](/docs/fr/permissions) normal s'applique toujours. Pour les hooks `UserPromptSubmit`, `UserPromptExpansion` et `SessionStart`, tout ce que vous écrivez sur stdout est ajouté au contexte de Claude.
* **Exit 2** : l'action est bloquée. Écrivez une raison sur stderr, et Claude la reçoit comme retour afin qu'il puisse s'ajuster. Certains événements ne peuvent pas être bloqués : pour `SessionStart`, `Setup`, `Notification` et autres, exit 2 affiche stderr à l'utilisateur et l'exécution continue. Consultez [comportement du code de sortie 2 par événement](/docs/fr/hooks#exit-code-2-behavior-per-event) pour la liste complète.
* **Tout autre code de sortie** : l'action se poursuit. La transcription affiche un avis `<hook name> hook error` suivi de la première ligne de stderr ; le stderr complet va au [journal de débogage](/docs/fr/hooks#debug-hooks).

<h4 id="structured-json-output">
  Sortie JSON structurée
</h4>

Les codes de sortie vous donnent seulement deux options : bloquer ou rester silencieux. Pour plus de contrôle, quittez 0 et imprimez un objet JSON sur stdout à la place.

<Note>
  Utilisez exit 2 pour bloquer avec un message stderr, ou exit 0 avec JSON pour un contrôle structuré. Ne les mélangez pas : Claude Code ignore JSON lorsque vous quittez 2.
</Note>

Par exemple, un hook `PreToolUse` peut refuser un appel d'outil et dire à Claude pourquoi, ou l'escalader à l'utilisateur pour approbation :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

Avec `"deny"`, Claude Code annule l'appel d'outil et renvoie `permissionDecisionReason` à Claude. Ces valeurs `permissionDecision` sont spécifiques à `PreToolUse` :

* `"allow"` : ignorer l'invite de permission interactive. Les règles de refus et d'ask, y compris les listes de refus gérées par l'entreprise, s'appliquent toujours, tout comme les invites pour les outils de connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool)
* `"deny"` : annuler l'appel d'outil et envoyer la raison à Claude
* `"ask"` : afficher l'invite de permission à l'utilisateur comme d'habitude

Une quatrième valeur, `"defer"`, est disponible en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`. Elle quitte le processus avec l'appel d'outil préservé afin qu'un wrapper SDK Agent puisse collecter l'entrée et reprendre. Consultez [Différer un appel d'outil pour plus tard](/docs/fr/hooks#defer-a-tool-call-for-later) dans la référence.

Retourner `"allow"` ignore l'invite interactive mais ne remplace pas les [règles de permission](/docs/fr/permissions#manage-permissions). Si une règle de refus correspond à l'appel d'outil, l'appel est bloqué même lorsque votre hook retourne `"allow"`. Si une règle d'ask correspond, l'utilisateur est toujours invité, tout comme les outils de connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool). Cela signifie que les règles de refus de n'importe quel périmètre de paramètres, y compris les [paramètres gérés](/docs/fr/settings#settings-files), ont toujours la priorité sur les approbations de hook.

D'autres événements utilisent des modèles de décision différents. Par exemple, les hooks `PostToolUse` et `Stop` utilisent un champ `decision: "block"` au niveau supérieur, tandis que `PermissionRequest` utilise `hookSpecificOutput.decision.behavior`. Consultez le [tableau récapitulatif](/docs/fr/hooks#decision-control) dans la référence pour une ventilation complète par événement.

Pour les hooks `UserPromptSubmit`, utilisez `hookSpecificOutput.additionalContext` à la place pour injecter du texte dans le contexte de Claude. Imbriquez `additionalContext` à l'intérieur de `hookSpecificOutput` ; si vous le placez au niveau supérieur du JSON, Claude Code l'ignore silencieusement. Par exemple, cette sortie ajoute l'état de la branche actuelle à chaque invite :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

Consultez [Contrôle de décision UserPromptSubmit](/docs/fr/hooks#userpromptsubmit-decision-control) pour la forme de sortie complète, y compris le blocage des invites et la définition du titre de la session.

Les hooks avec `type: "prompt"` gèrent la sortie différemment : consultez [Hooks basés sur des invites](#prompt-based-hooks).

<h3 id="filter-hooks-with-matchers">
  Filtrer les hooks avec des matchers
</h3>

Sans matcher, un hook se déclenche à chaque occurrence de son événement. Les matchers vous permettent de réduire cela. Par exemple, si vous voulez exécuter un formateur uniquement après les modifications de fichiers, pas après chaque appel d'outil, ajoutez un matcher à votre hook `PostToolUse` :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

Le matcher `"Edit|Write"` se déclenche uniquement lorsque Claude utilise l'outil `Edit` ou `Write`, pas lorsqu'il utilise `Bash`, `Read`, ou tout autre outil. Sur Claude Code v2.1.191 ou ultérieur, une virgule sépare les alternatives de la même manière, donc `"Edit, Write"` est équivalent. Consultez [Modèles de matcher](/docs/fr/hooks#matcher-patterns) pour savoir comment les noms simples et les expressions régulières sont évalués.

<Note>
  Claude peut également créer ou modifier des fichiers en exécutant des commandes shell via l'outil `Bash`. Si votre hook doit voir chaque modification de fichier, par exemple pour l'analyse de conformité ou l'enregistrement d'audit, ajoutez un hook [`Stop`](/docs/fr/hooks#stop) qui analyse l'arborescence de travail une fois par tour. Pour une couverture par appel à la place, correspondez également à `Bash` et faites en sorte que votre script liste les fichiers modifiés et non suivis avec `git status --porcelain`.
</Note>

Chaque type d'événement correspond à un champ spécifique :

| Événement                                                                                                                                                       | Ce que le matcher filtre                                                                 | Exemples de valeurs de matcher                                                                                                                                                      |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | nom de l'outil                                                                           | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                                  | comment la session a démarré                                                             | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                                         | quel drapeau CLI a déclenché la configuration                                            | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                                    | pourquoi la session s'est terminée                                                       | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                                  | type de notification                                                                     | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                                 | type d'agent                                                                             | `general-purpose`, `Explore`, `Plan`, ou noms d'agents personnalisés                                                                                                                |
| `PreCompact`, `PostCompact`                                                                                                                                     | ce qui a déclenché la compaction                                                         | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                                  | type d'agent                                                                             | mêmes valeurs que `SubagentStart`                                                                                                                                                   |
| `ConfigChange`                                                                                                                                                  | source de configuration                                                                  | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `StopFailure`                                                                                                                                                   | type d'erreur                                                                            | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                                            | raison du chargement                                                                     | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `Elicitation`                                                                                                                                                   | nom du serveur MCP                                                                       | vos noms de serveur MCP configurés                                                                                                                                                  |
| `ElicitationResult`                                                                                                                                             | nom du serveur MCP                                                                       | mêmes valeurs que `Elicitation`                                                                                                                                                     |
| `FileChanged`                                                                                                                                                   | noms de fichiers littéraux à surveiller (consultez [FileChanged](/docs/fr/hooks#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `UserPromptExpansion`                                                                                                                                           | nom de la commande                                                                       | vos noms de skill ou de commande                                                                                                                                                    |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | pas de support de matcher                                                                | se déclenche toujours à chaque occurrence                                                                                                                                           |

Les onglets ci-dessous montrent quelques autres matchers sur différents types d'événements.

<Tabs>
  <Tab title="Enregistrer chaque commande Bash">
    Correspond uniquement aux appels d'outil `Bash` et enregistre chaque commande dans un fichier. L'événement `PostToolUse` se déclenche après la fin de la commande, donc `tool_input.command` contient ce qui a été exécuté. Le hook reçoit les données d'événement en JSON sur stdin, et `jq -r '.tool_input.command'` extrait juste la chaîne de commande, que `>>` ajoute au fichier journal :

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Correspondre aux outils MCP">
    Les outils MCP utilisent une convention de nommage différente des outils intégrés : `mcp__<server>__<tool>`, où `<server>` est le nom du serveur MCP et `<tool>` est l'outil qu'il fournit. Par exemple, `mcp__github__search_repositories` ou `mcp__filesystem__read_file`. Les outils d'un [serveur fourni par plugin](/docs/fr/mcp#plugin-provided-mcp-servers) utilisent un segment de serveur délimité à la place, comme `mcp__plugin_my-plugin_db__query`. Utilisez un matcher regex pour cibler tous les outils d'un serveur spécifique, ou correspondre entre les serveurs avec un modèle comme `mcp__.*__write.*`. Consultez [Correspondre aux outils MCP](/docs/fr/hooks#match-mcp-tools) dans la référence pour la liste complète des exemples.

    La commande ci-dessous extrait le nom de l'outil de l'entrée JSON du hook avec `jq` et l'écrit sur stderr. L'écriture sur stderr garde stdout propre pour la sortie JSON et envoie le message au [journal de débogage](/docs/fr/hooks#debug-hooks) :

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Nettoyer à la fin de la session">
    L'événement `SessionEnd` supporte les matchers sur la raison de la fin de la session. Ce hook ne se déclenche que sur `clear` (lorsque vous exécutez `/clear`), pas sur les sorties normales :

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Pour la syntaxe complète du matcher, consultez la [référence des Hooks](/docs/fr/hooks#configuration).

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  Filtrer par nom d'outil et arguments avec le champ `if`
</h4>

Le champ `if` utilise la [syntaxe des règles de permission](/docs/fr/permissions) pour filtrer les hooks par nom d'outil et arguments ensemble, afin que le processus du hook ne soit généré que lorsque l'appel d'outil correspond. Cela va au-delà du `matcher`, qui filtre au niveau du groupe par nom d'outil uniquement.

Par exemple, pour exécuter un hook uniquement lorsque Claude utilise des commandes `git` plutôt que toutes les commandes Bash :

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

Le processus du hook s'exécute selon la forme de votre modèle `if` et la commande Bash que Claude invoque :

| Modèle `if`        | Commande Bash          | Le hook s'exécute-t-il ? | Pourquoi                                                                                                                       |
| :----------------- | :--------------------- | :----------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `git push`             | oui                      | le nom de la commande correspond                                                                                               |
| `Bash(git *)`      | `npm test && git push` | oui                      | chaque sous-commande est vérifiée ; `git push` correspond                                                                      |
| `Bash(git *)`      | `echo $(git log)`      | oui                      | les commandes à l'intérieur de `$()` et des backticks sont vérifiées ; `git log` correspond                                    |
| `Bash(git *)`      | `echo $(date)`         | non                      | aucune sous-commande ne correspond à `git *`                                                                                   |
| `Bash(git push *)` | `echo $(date)`         | oui                      | les modèles qui spécifient plus que le nom de la commande exécutent le hook de toute façon sur `$()`, les backticks, ou `$VAR` |

Le filtre échoue également de manière ouverte, exécutant votre hook indépendamment du modèle, lorsque la commande Bash ne peut pas être analysée. Parce que le filtre est au mieux un effort, utilisez le [système de permission](/docs/fr/permissions) plutôt qu'un hook pour appliquer un allow ou deny dur.

Le champ `if` accepte les mêmes modèles que les règles de permission : `"Bash(git *)"`, `"Edit(*.ts)"`, et ainsi de suite. Pour correspondre à plusieurs noms d'outils, utilisez des gestionnaires séparés chacun avec sa propre valeur `if`, ou correspondez au niveau du `matcher` où l'alternation par pipe est supportée.

`if` ne fonctionne que sur les événements d'outils : `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` et `PermissionDenied`. L'ajouter à tout autre événement empêche le hook de s'exécuter.

<h3 id="configure-hook-location">
  Configurer l'emplacement du hook
</h3>

L'endroit où vous ajoutez un hook détermine son périmètre :

| Emplacement                                                | Périmètre                                 | Partageable                                 |
| :--------------------------------------------------------- | :---------------------------------------- | :------------------------------------------ |
| `~/.claude/settings.json`                                  | Tous vos projets                          | Non, local à votre machine                  |
| `.claude/settings.json`                                    | Projet unique                             | Oui, peut être commité au repo              |
| `.claude/settings.local.json`                              | Projet unique                             | Non, gitignored lorsque Claude Code le crée |
| Paramètres de politique gérés                              | À l'échelle de l'organisation             | Oui, contrôlé par l'administrateur          |
| [Plugin](/docs/fr/plugins) `hooks/hooks.json`                   | Lorsque le plugin est activé              | Oui, fourni avec le plugin                  |
| [Skill](/docs/fr/skills) ou [agent](/docs/fr/sub-agents) frontmatter | Pendant que le skill ou l'agent est actif | Oui, défini dans le fichier du composant    |

Exécutez [`/hooks`](/docs/fr/hooks#the-%2Fhooks-menu) dans Claude Code pour parcourir tous les hooks configurés regroupés par événement.

Pour désactiver les hooks, définissez `"disableAllHooks": true` dans votre fichier de paramètres. Les hooks configurés dans les paramètres gérés s'exécutent toujours sauf si `disableAllHooks` est également défini là.

Si vous modifiez les fichiers de paramètres directement pendant que Claude Code s'exécute, l'observateur de fichiers récupère normalement les modifications de hook automatiquement.

<h2 id="prompt-based-hooks">
  Hooks basés sur des invites
</h2>

Pour les décisions qui nécessitent un jugement plutôt que des règles déterministes, utilisez les hooks `type: "prompt"`. Au lieu d'exécuter une commande shell, Claude Code envoie votre invite et les données d'entrée du hook à un modèle Claude (Haiku par défaut) pour prendre la décision. Vous pouvez spécifier un modèle différent avec le champ `model` si vous avez besoin de plus de capacité.

Le seul travail du modèle est de retourner une décision oui/non en JSON :

* `"ok": true` : l'action se poursuit
* `"ok": false` : ce qui se passe dépend de l'événement :
  * `Stop` et `SubagentStop` : la `reason` est renvoyée à Claude afin qu'il continue à travailler
  * `PreToolUse` : l'appel d'outil est refusé et la `reason` est retournée à Claude comme erreur d'outil, afin qu'il puisse s'ajuster et continuer
  * `PostToolUse`, `PostToolBatch`, `UserPromptSubmit` et `UserPromptExpansion` : le tour se termine et la `reason` apparaît dans le chat sous forme de ligne d'avertissement

Cet exemple utilise un hook `Stop` pour demander au modèle si toutes les tâches demandées sont complètes. Si le modèle retourne `"ok": false`, Claude continue à travailler et utilise la `reason` comme sa prochaine instruction :

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

Pour les options de configuration complètes, consultez [Hooks basés sur des invites](/docs/fr/hooks#prompt-based-hooks) dans la référence.

<h2 id="agent-based-hooks">
  Hooks basés sur des agents
</h2>

<Warning>
  Les hooks d'agent sont expérimentaux. Le comportement et la configuration peuvent changer dans les versions futures. Pour les workflows de production, préférez les [hooks de commande](/docs/fr/hooks#command-hook-fields).
</Warning>

Lorsque la vérification nécessite d'inspecter des fichiers ou d'exécuter des commandes, utilisez les hooks `type: "agent"`. Contrairement aux hooks d'invite qui font un seul appel LLM, les hooks d'agent génèrent un subagent qui peut lire des fichiers, rechercher du code et utiliser d'autres outils pour vérifier les conditions avant de retourner une décision.

Les hooks d'agent utilisent le même format de réponse `"ok"` / `"reason"` que les hooks d'invite, mais avec un délai d'expiration par défaut plus long de 60 secondes et jusqu'à 50 tours d'utilisation d'outils.

Cet exemple vérifie que les tests réussissent avant de permettre à Claude de s'arrêter :

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

Utilisez les hooks d'invite lorsque les données d'entrée du hook seules suffisent pour prendre une décision. Utilisez les hooks d'agent lorsque vous avez besoin de vérifier quelque chose par rapport à l'état réel de la base de code.

Pour les options de configuration complètes, consultez [Hooks basés sur des agents](/docs/fr/hooks#agent-based-hooks) dans la référence.

<h2 id="http-hooks">
  Hooks HTTP
</h2>

Utilisez les hooks `type: "http"` pour POST les données d'événement vers un point de terminaison HTTP au lieu d'exécuter une commande shell. Le point de terminaison reçoit le même JSON qu'un hook de commande recevrait sur stdin, et retourne les résultats via le corps de la réponse HTTP en utilisant le même format JSON.

Les hooks HTTP sont utiles lorsque vous voulez qu'un serveur web, une fonction cloud ou un service externe gère la logique du hook : par exemple, un service d'audit partagé qui enregistre les événements d'utilisation d'outils dans une équipe.

Cet exemple poste chaque utilisation d'outil vers un service de journalisation local :

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
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

Le point de terminaison doit retourner un corps de réponse JSON en utilisant le même [format de sortie](/docs/fr/hooks#json-output) que les hooks de commande. Pour bloquer un appel d'outil, retournez une réponse 2xx avec les champs `hookSpecificOutput` appropriés. Les codes de statut HTTP seuls ne peuvent pas bloquer les actions.

Les valeurs d'en-tête supportent l'interpolation de variables d'environnement en utilisant la syntaxe `$VAR_NAME` ou `${VAR_NAME}`. Seules les variables listées dans le tableau `allowedEnvVars` sont résolues ; toutes les autres références `$VAR` restent vides.

Pour les options de configuration complètes et la gestion des réponses, consultez [Hooks HTTP](/docs/fr/hooks#http-hook-fields) dans la référence.

<h2 id="limitations-and-troubleshooting">
  Limitations et dépannage
</h2>

<h3 id="limitations">
  Limitations
</h3>

Gardez ces contraintes à l'esprit lors de la conception des hooks :

* Les hooks de commande communiquent uniquement via stdout, stderr et les codes de sortie. Ils ne peuvent pas déclencher des commandes `/` ou des appels d'outils. Le texte retourné via `additionalContext` est injecté comme un rappel système que Claude lit en tant que texte brut. Les hooks HTTP communiquent via le corps de la réponse à la place.
* Les délais d'expiration du hook varient selon le type. Remplacez par hook avec le champ `timeout` en secondes.
  * `command`, `http`, `mcp_tool` : 10 minutes. `UserPromptSubmit` les réduit à 30 secondes, et `MessageDisplay` les réduit à 10 secondes.
  * `prompt` : 30 secondes.
  * `agent` : 60 secondes.
* Les hooks `PostToolUse` ne peuvent pas annuler les actions puisque l'outil a déjà été exécuté.
* Les hooks `PermissionRequest` ne se déclenchent pas en [mode non-interactif](/docs/fr/headless) avec l'indicateur `-p`. Utilisez les hooks `PreToolUse` pour les décisions de permission automatisées.
* Les hooks `Stop` se déclenchent chaque fois que Claude termine sa réponse, pas seulement à la fin de la tâche. Ils ne se déclenchent pas sur les interruptions de l'utilisateur. Les erreurs API déclenchent [StopFailure](/docs/fr/hooks#stopfailure) à la place.
* Lorsque plusieurs hooks `PreToolUse` retournent [`updatedInput`](/docs/fr/hooks#pretooluse) pour réécrire les arguments d'un outil, le dernier à terminer gagne. Puisque les hooks s'exécutent en parallèle, l'ordre est non-déterministe. Évitez d'avoir plus d'un hook modifier l'entrée du même outil.

<h3 id="hooks-and-permission-modes">
  Hooks et modes de permission
</h3>

Les hooks `PreToolUse` se déclenchent avant toute vérification du mode de permission. Un hook qui retourne `permissionDecision: "deny"` bloque l'outil même en mode `bypassPermissions` ou avec `--dangerously-skip-permissions`. Cela vous permet d'appliquer une politique que les utilisateurs ne peuvent pas contourner en changeant leur mode de permission.

L'inverse n'est pas vrai : un hook retournant `"allow"` ne contourne pas les règles de refus des paramètres, et il ne peut pas supprimer l'invite pour les outils de connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) ou les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool). Les hooks peuvent renforcer les restrictions mais pas les assouplir au-delà de ce que les règles de permission permettent.

<h3 id="hook-not-firing">
  Hook ne se déclenche pas
</h3>

Le hook est configuré mais ne s'exécute jamais.

* Exécutez `/hooks` et confirmez que le hook apparaît sous l'événement correct
* Vérifiez que le modèle de matcher correspond exactement au nom de l'outil. Les matchers sont sensibles à la casse
* Vérifiez que vous déclenchez le bon type d'événement : `PreToolUse` se déclenche avant l'exécution de l'outil, `PostToolUse` se déclenche après
* Si vous utilisez des hooks `PermissionRequest` en mode non-interactif avec l'indicateur `-p`, passez à `PreToolUse` à la place

<h3 id="hook-error-in-output">
  Erreur du hook dans la sortie
</h3>

Vous voyez un message comme « PreToolUse hook error : ... » dans la transcription.

* Votre script a quitté avec un code non-zéro de manière inattendue. Testez-le manuellement en piping du JSON d'exemple :
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # Check the exit code
  ```
* Si vous voyez « command not found », utilisez des chemins absolus ou `${CLAUDE_PROJECT_DIR}` pour référencer les scripts. Pour éviter complètement les guillemets du shell, ajoutez `"args": []` pour basculer vers la [forme exec](/docs/fr/hooks#exec-form-and-shell-form), qui génère le script directement sans shell
* Si vous voyez « jq: command not found », installez `jq` ou utilisez Python/Node.js pour l'analyse JSON
* Si le script ne s'exécute pas du tout, rendez-le exécutable : `chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks` n'affiche aucun hook configuré
</h3>

Vous avez modifié un fichier de paramètres mais les hooks n'apparaissent pas dans le menu.

* Les modifications de fichiers sont normalement récupérées automatiquement. Si elles n'ont pas apparues après quelques secondes, l'observateur de fichiers peut avoir manqué la modification : redémarrez votre session pour forcer un rechargement.
* Vérifiez que votre JSON est valide : les virgules finales et les commentaires ne sont pas autorisés
* Confirmez que le fichier de paramètres est au bon emplacement : `.claude/settings.json` pour les hooks de projet, `~/.claude/settings.json` pour les hooks globaux

<h3 id="stop-hook-hits-the-block-cap">
  Le hook Stop atteint le plafond de blocage
</h3>

Claude continue à travailler au lieu de s'arrêter, puis termine le tour avec un avertissement selon lequel le hook Stop a bloqué trop de fois consécutives.

Claude Code remplace un hook Stop après qu'il ait bloqué huit fois de suite sans progrès. Votre script de hook doit vérifier s'il a déjà déclenché une continuation. Analysez le champ `stop_hook_active` de l'entrée JSON et quittez tôt s'il est `true` :

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Allow Claude to stop
fi
# ... rest of your hook logic
```

Si votre hook a légitimement besoin de plus de huit itérations pour converger, augmentez le plafond avec [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/fr/env-vars).

<h3 id="json-validation-failed">
  Validation JSON échouée
</h3>

Claude Code affiche une erreur d'analyse JSON même si votre script de hook produit du JSON valide.

Lorsque Claude Code exécute un hook de commande sous forme de shell (un sans `args`), il génère `sh -c` sur macOS et Linux ou Git Bash sur Windows par défaut. Ce shell est non-interactif, mais Git Bash et certaines configurations, comme `BASH_ENV` pointant vers `~/.bashrc`, sourcent toujours votre profil. Si ce profil contient des instructions `echo` inconditionnelles, la sortie est ajoutée au début de votre JSON du hook :

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

Claude Code essaie d'analyser ceci en JSON et échoue. Pour corriger cela, enveloppez les instructions echo dans votre profil shell afin qu'elles ne s'exécutent que dans les shells interactifs :

```bash theme={null}
# In ~/.zshrc or ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

La variable `$-` contient les drapeaux du shell, et `i` signifie interactif. Les hooks s'exécutent dans des shells non-interactifs, donc l'echo est ignoré.

<h3 id="debug-techniques">
  Techniques de débogage
</h3>

La vue de transcription, basculée avec `Ctrl+O`, affiche un résumé d'une ligne pour chaque hook qui s'est déclenché : le succès est silencieux, les erreurs de blocage affichent stderr, et les erreurs sans blocage affichent un avis `<hook name> hook error` suivi de la première ligne de stderr.

Pour les détails d'exécution complets incluant les hooks qui ont correspondu, leurs codes de sortie, stdout et stderr, lisez le journal de débogage. Démarrez Claude Code avec `claude --debug-file /tmp/claude.log` pour écrire dans un chemin connu, puis `tail -f /tmp/claude.log` dans un autre terminal. Si vous avez démarré sans ce drapeau, exécutez `/debug` en milieu de session pour activer la journalisation et trouver le chemin du journal.

<h2 id="learn-more">
  En savoir plus
</h2>

* [Référence des Hooks](/docs/fr/hooks) : schémas d'événements complets, format de sortie JSON, hooks asynchrones et hooks d'outils MCP
* [Considérations de sécurité](/docs/fr/hooks#security-considerations) : examinez avant de déployer les hooks dans des environnements partagés ou de production
* [Exemple de validateur de commande Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py) : implémentation de référence complète
