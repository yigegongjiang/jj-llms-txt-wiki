> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatiser les actions avec les hooks

> Exécutez automatiquement des commandes shell lorsque Claude Code modifie des fichiers, termine des tâches ou a besoin d'une entrée. Formatez le code, envoyez des notifications, validez les commandes et appliquez les règles du projet.

Les hooks sont des commandes shell définies par l'utilisateur. Claude Code les exécute à des points spécifiques de son cycle de vie, ce qui vous donne un contrôle déterministe : certaines actions se produisent toujours plutôt que de compter sur le LLM pour choisir de les exécuter. Utilisez les hooks pour appliquer les règles du projet, automatiser les tâches répétitives et intégrer Claude Code avec vos outils existants.

Pour les décisions qui nécessitent un jugement plutôt que des règles déterministes, vous pouvez également utiliser des [hooks basés sur des invites](#prompt-based-hooks) ou des [hooks basés sur des agents](#agent-based-hooks) qui utilisent un modèle Claude pour évaluer les conditions.

Pour d'autres façons d'étendre Claude Code, consultez [skills](/docs/fr/skills) pour donner à Claude des instructions supplémentaires et des commandes exécutables, [subagents](/docs/fr/sub-agents) pour exécuter des tâches dans des contextes isolés, et [plugins](/docs/fr/plugins/overview) pour empaqueter les extensions à partager entre les projets.

<Tip>
  Ce guide couvre les cas d'usage courants et comment commencer. Pour les schémas d'événements complets, les formats d'entrée/sortie JSON et les fonctionnalités avancées comme les hooks asynchrones et les hooks d'outils MCP, consultez la [référence des Hooks](/docs/fr/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Configurer votre premier hook
</h2>

Pour créer un hook, ajoutez un bloc `hooks` à un [fichier de paramètres](#configure-hook-location). Cette procédure crée un hook de notification de bureau, afin que vous soyez alerté chaque fois que Claude attend votre entrée au lieu de regarder le terminal.

<Steps>
  <Step title="Ajouter le hook à vos paramètres">
    Ouvrez `~/.claude/settings.json` et ajoutez un hook `Notification`. Si le fichier n'existe pas, créez-le. L'exemple ci-dessous utilise `osascript` pour macOS ; consultez [Être notifié lorsque Claude a besoin d'une entrée](#get-notified-when-claude-needs-input) pour les commandes Linux et Windows.

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
    Appuyez sur `Esc` pour revenir au CLI. Appuyez sur `Shift+Tab` jusqu'à ce que la barre d'état affiche `⏸ manual mode on`, demandez à Claude de faire quelque chose qui nécessite une permission, puis quittez le terminal. Vous devriez recevoir une notification de bureau.
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

Ce hook utilise l'événement `Notification`, qui se déclenche lorsque Claude attend une entrée ou une permission. Consultez [quand chaque type de notification se déclenche](/docs/fr/hooks#notification) pour le timing exact. Chaque onglet ci-dessous utilise la commande de notification native de la plateforme. Ajoutez ceci à `~/.claude/settings.json` :

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

    <Accordion title="Si aucune notification n'apparaît">
      `notify-send` a besoin d'un daemon de notification de bureau, que les serveurs sans interface graphique, les sessions SSH et la plupart des conteneurs n'ont pas. Testez d'abord la commande directement :

      ```bash theme={null}
      notify-send 'Claude Code' 'test'
      ```

      Si la commande n'est pas trouvée, installez le paquet `libnotify-bin` sur Debian et Ubuntu, ou l'équivalent de votre distribution.
    </Accordion>
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

    <Accordion title="Si aucune boîte de dialogue n'apparaît">
      Cette commande ouvre une boîte de dialogue plutôt qu'une notification dans le coin de votre écran, la boîte de dialogue peut donc s'ouvrir derrière votre fenêtre de terminal. Testez d'abord la commande directement dans PowerShell. Si vous exécutez Claude Code dans WSL, `powershell.exe` doit être disponible sur votre `PATH` via l'interopérabilité Windows.
    </Accordion>
  </Tab>
</Tabs>

Le matcher vide se déclenche sur tous les types de notification. Pour se déclencher uniquement sur des événements spécifiques, définissez-le sur l'une de ces valeurs :

| Matcher                      | Se déclenche quand                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude a besoin que vous approuviez un appel d'outil ou une [requête réseau](/docs/fr/sandboxing#network-isolation) de commande en sandbox, et l'invite a attendu environ six secondes                                                                                                                                                                                                                                                                                                                                                                           |
| `idle_prompt`                | Claude a terminé sa réponse il y a environ 60 secondes et vous n'avez pas tapé depuis                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `auth_success`               | L'authentification se termine                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `elicitation_dialog`         | Un serveur MCP ouvre un formulaire d'élicitation et vous n'avez pas tapé pendant environ six secondes                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `elicitation_url_dialog`     | Un serveur MCP vous demande d'ouvrir une URL de navigateur et vous n'avez pas tapé pendant environ six secondes                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `elicitation_complete`       | Un serveur MCP signale qu'une [élicitation en mode URL](/docs/fr/hooks#elicitation-input) est complète                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `elicitation_response`       | Une réponse d'élicitation MCP est renvoyée au serveur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `agent_needs_input`          | Une session en arrière-plan commence à attendre votre entrée tandis que la [vue agent](/docs/fr/agent-view) est ouverte, ou la session actuelle vous pose une [question de configuration de terminal d'un coéquipier d'équipe agent](/docs/fr/agent-teams#choose-a-display-mode) et vous n'avez pas tapé pendant environ six secondes                                                                                                                                                                                                                                 |
| `agent_completed`            | Une session en arrière-plan se termine ou échoue. Se déclenche uniquement lorsque la [vue agent](/docs/fr/agent-view) est ouverte                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `quota_auto_resume_fired`    | Claude Code continue votre tâche après qu'une limite d'utilisation claude.ai l'ait mise en pause : au réinitialisation, ou plus tôt lorsque quelque chose que vous faites dans Claude Code pendant l'attente, comme l'ajout de crédits d'utilisation, la mise à niveau de votre plan, ou le changement de modèles, rend l'utilisation disponible à nouveau, avec l'[exception de paramètre de modèle](/docs/fr/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                 |
| `quota_auto_resume_stale`    | Une limite d'utilisation claude.ai s'est réinitialisée tandis que votre ordinateur dormait pendant plus d'environ 30 minutes. Claude Code attend que vous appuyiez sur `Entrée` au lieu de continuer. Après un sommeil plus court, il continue et déclenche `quota_auto_resume_fired` à la place                                                                                                                                                                                                                                                            |
| `quota_auto_resume_disabled` | Claude Code termine son attente pour une limite d'utilisation claude.ai sans continuer votre tâche : [`autoContinueAtUsageLimit`](/docs/fr/settings-reference#autocontinueatusagelimit) désactivé ou la réinitialisation s'est déplacée de plus de 24 heures pendant une attente que Claude Code a lancée de lui-même, la tâche continuée a continué à atteindre la limite, ou la continuation a été bloquée avant d'atteindre le modèle. Ne se déclenche pas lorsque vous appuyez sur `Échap` ou `Ctrl+C`, ou sélectionnez **Ne pas continuer automatiquement** |

Claude Code chronométre `permission_prompt` différemment dans un terminal et dans Claude Desktop, l'extension VS Code et d'autres hôtes qui répondent aux demandes de permission via le SDK Agent. Consultez [quand chaque type de notification se déclenche](/docs/fr/hooks#notification) pour les deux timings.

Les matchers `agent_needs_input` et `agent_completed` nécessitent Claude Code v2.1.198 ou version ultérieure.

Les matchers `quota_auto_resume_fired`, `quota_auto_resume_stale` et `quota_auto_resume_disabled` nécessitent Claude Code v2.1.234 ou version ultérieure.

Dans les sessions de terminal, `permission_prompt` pour une requête réseau de commande en sandbox nécessite Claude Code v2.1.246 ou version ultérieure.

`agent_needs_input` pour une question de configuration de terminal d'un coéquipier nécessite Claude Code v2.1.248 ou version ultérieure.

Tapez `/hooks` et sélectionnez `Notification` pour confirmer que le hook est enregistré. Pour le schéma d'événement complet, consultez la [référence Notification](/docs/fr/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Formater automatiquement le code après les modifications
</h3>

Exécutez automatiquement [Prettier](https://prettier.io/) sur chaque fichier que Claude modifie, afin que le formatage reste cohérent sans intervention manuelle.

Ce hook utilise l'événement `PostToolUse` avec un matcher `Edit|Write`, il s'exécute donc uniquement après les outils d'édition de fichiers. La commande extrait le chemin du fichier modifié avec [`jq`](https://jqlang.org/) et le transmet à Prettier. Ajoutez ceci à `.claude/settings.json` à la racine de votre projet :

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

Pour tester le hook, demandez à Claude d'ajouter une ligne avec des chaînes entre guillemets simples à un fichier JavaScript, puis ouvrez le fichier : avec les paramètres par défaut de Prettier, le hook les réécrit en guillemets doubles.

Lorsque le hook réussit, Claude Code n'affiche rien dans la conversation. Pour confirmer que le hook a été exécuté, vérifiez que le fichier modifié est reformaté, ou consultez [Techniques de débogage](#debug-techniques).

Pour reformater un fichier spécifique quelle que soit la façon dont il change, y compris lorsqu'une commande `Bash` le réécrit, utilisez un hook [FileChanged](/docs/fr/hooks#filechanged) à la place.

<Note>
  Les exemples Bash sur cette page utilisent `jq` pour l'analyse JSON. Installez-le avec `brew install jq` sur macOS, `apt-get install jq` sur Debian et Ubuntu, ou consultez les [téléchargements de `jq`](https://jqlang.org/download/).
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

    # Normalize Windows backslash separators so the patterns below match
    FILE_PATH="${FILE_PATH//\\//}"

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

  <Step title="Tester le hook">
    Demandez à Claude d'ajouter un commentaire à votre fichier `.env`. Claude Code bloque la modification avant qu'elle ne s'exécute et transmet le message `Blocked:` du script à Claude comme retour.
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Réinjecter le contexte après compaction
</h3>

Lorsque la fenêtre de contexte de Claude se remplit, la compaction résume la conversation pour libérer de l'espace. Cela peut perdre des détails importants. Utilisez un hook `SessionStart` avec un matcher `compact` pour réinjecter le contexte critique après chaque compaction.

Claude Code ajoute le texte brut que votre commande écrit sur stdout au contexte de Claude. Cet exemple rappelle à Claude les conventions du projet et le travail récent. Ajoutez ceci à `.claude/settings.json` à la racine de votre projet :

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

Pour confirmer que le hook enregistre les modifications, modifiez un fichier de paramètres dans un autre éditeur tandis qu'une session est en cours d'exécution, puis ouvrez `~/claude-config-audit.log` : le hook ajoute une ligne JSON par modification avec l'horodatage, la source et le chemin du fichier.

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

Contrairement aux exemples de code de sortie ci-dessus, l'approbation automatique nécessite que votre hook écrive une décision JSON sur stdout. Claude Code exécute les hooks `PermissionRequest` lorsqu'il est sur le point de vous demander une permission, et si votre hook retourne `"behavior": "allow"`, Claude Code répond à la demande en votre nom.

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
  `bypassPermissions` ne s'applique que si la session a été lancée avec le mode bypass déjà disponible : `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, ou `permissions.defaultMode: "bypassPermissions"` dans les [paramètres utilisateur, `--settings`, ou paramètres gérés](/docs/fr/settings-reference#permissions-defaultmode). Il ne s'applique pas si le mode bypass est désactivé par [`permissions.disableBypassPermissionsMode`](/docs/fr/permissions#managed-settings), ou si vous avez lancé la session en [mode restreint](/docs/fr/cli-reference#cli-flags).

  Claude Code ne le sauvegarde jamais en tant que `defaultMode`.
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

Claude Code déclenche des événements de hook à des points spécifiques de son cycle de vie. Lorsqu'un événement se déclenche, Claude Code exécute tous les hooks correspondants en parallèle ; consultez [Champs de gestionnaire de hook](/docs/fr/hooks#hook-handler-fields) pour savoir comment les gestionnaires en doublon sont traités. Le tableau ci-dessous montre chaque événement et quand il se déclenche :

| Événement             | Quand il se déclenche                                                                                                                                                                                                                                                                                             |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | Quand une session commence ou reprend                                                                                                                                                                                                                                                                             |
| `Setup`               | Quand vous démarrez Claude Code avec `--init-only`, ou avec `--init` ou `--maintenance` en mode `-p`. Pour une préparation unique en CI ou dans les scripts                                                                                                                                                       |
| `UserPromptSubmit`    | Quand vous soumettez une invite, avant que Claude la traite                                                                                                                                                                                                                                                       |
| `UserPromptExpansion` | Quand une commande tapée par l'utilisateur se développe en une invite, avant qu'elle n'atteigne Claude. Peut bloquer l'expansion                                                                                                                                                                                  |
| `PreToolUse`          | Avant qu'un appel d'outil s'exécute. Peut le bloquer                                                                                                                                                                                                                                                              |
| `PermissionRequest`   | Quand un appel d'outil nécessite une décision de permission                                                                                                                                                                                                                                                       |
| `PermissionDenied`    | Quand le mode automatique refuse un appel d'outil, y compris les refus sans verdict du classificateur. Utilisez la sortie JSON `hookSpecificOutput.retry: true` pour indiquer au modèle qu'il peut réessayer l'appel d'outil refusé. Claude Code ignore `retry` quand le classificateur n'a produit aucun verdict |
| `PostToolUse`         | Après qu'un appel d'outil réussisse                                                                                                                                                                                                                                                                               |
| `PostToolUseFailure`  | Après qu'un appel d'outil échoue                                                                                                                                                                                                                                                                                  |
| `PostToolBatch`       | Après qu'un lot complet d'appels d'outils parallèles se résout, avant l'appel du modèle suivant                                                                                                                                                                                                                   |
| `Notification`        | Quand Claude Code envoie une notification                                                                                                                                                                                                                                                                         |
| `MessageDisplay`      | Pendant que le texte du message assistant s'affiche                                                                                                                                                                                                                                                               |
| `SubagentStart`       | Quand un sous-agent est généré                                                                                                                                                                                                                                                                                    |
| `SubagentStop`        | Quand un sous-agent se termine                                                                                                                                                                                                                                                                                    |
| `TaskCreated`         | Quand une tâche est en cours de création via `TaskCreate`                                                                                                                                                                                                                                                         |
| `TaskCompleted`       | Quand une tâche est marquée comme complétée                                                                                                                                                                                                                                                                       |
| `Stop`                | Quand Claude finit de répondre                                                                                                                                                                                                                                                                                    |
| `StopFailure`         | Quand le tour se termine en raison d'une erreur API                                                                                                                                                                                                                                                               |
| `TeammateIdle`        | Quand un coéquipier d'une [équipe d'agents](/docs/fr/agent-teams) est sur le point de devenir inactif                                                                                                                                                                                                                  |
| `InstructionsLoaded`  | Quand un fichier CLAUDE.md ou `.claude/rules/*.md` est chargé dans le contexte. Se déclenche au démarrage de la session et quand les fichiers sont chargés paresseusement pendant une session                                                                                                                     |
| `ConfigChange`        | Quand un fichier de configuration change pendant une session                                                                                                                                                                                                                                                      |
| `CwdChanged`          | Quand le répertoire de travail change, par exemple quand Claude exécute une commande `cd`. Utile pour la gestion réactive de l'environnement avec des outils comme direnv                                                                                                                                         |
| `DirectoryAdded`      | Quand un répertoire de travail est ajouté en milieu de session via `/add-dir` ou la demande de contrôle SDK `register_repo_root`                                                                                                                                                                                  |
| `FileChanged`         | Quand un fichier surveillé change sur le disque. Le champ `matcher` spécifie les noms de fichiers à surveiller                                                                                                                                                                                                    |
| `WorktreeCreate`      | Quand un worktree est en cours de création via `--worktree`, `isolation: "worktree"`, ou pour une session en arrière-plan. Remplace le comportement git par défaut                                                                                                                                                |
| `WorktreeRemove`      | Quand un worktree est supprimé à la sortie de la session, quand un sous-agent se termine, ou quand vous supprimez une session en arrière-plan                                                                                                                                                                     |
| `PreCompact`          | Avant la compaction du contexte                                                                                                                                                                                                                                                                                   |
| `PostCompact`         | Après la compaction du contexte est complétée                                                                                                                                                                                                                                                                     |
| `PreModelSwitch`      | Avant que Claude Code applique un changement de modèle que vous ou un client avez demandé. Peut bloquer le changement                                                                                                                                                                                             |
| `PostModelSwitch`     | Après que le modèle de la session change, y compris les changements que Claude Code effectue de lui-même, comme la restauration du modèle quand vous reprenez une session                                                                                                                                         |
| `Elicitation`         | Quand un serveur MCP demande une entrée utilisateur pendant un appel d'outil                                                                                                                                                                                                                                      |
| `ElicitationResult`   | Après qu'un utilisateur réponde à une élicitation MCP, avant que la réponse soit renvoyée au serveur                                                                                                                                                                                                              |
| `SessionEnd`          | Quand une session se termine                                                                                                                                                                                                                                                                                      |

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

Chaque événement inclut des champs communs comme `session_id`, un ID unique pour la session, et `cwd`, le répertoire de travail lorsque l'événement s'est déclenché, mais chaque type d'événement ajoute des données différentes. Lorsque Claude exécute une commande Bash, un hook `PreToolUse` reçoit ces champs sur stdin :

* `hook_event_name` : l'événement qui a déclenché le hook
* `tool_name` : l'outil que Claude est sur le point d'utiliser
* `tool_input` : les arguments que Claude a passés à l'outil. Pour Bash, son champ `command` contient la commande shell.

Par exemple, l'entrée du hook pour une commande `npm test` ressemble à ceci :

```json theme={null}
{
  "session_id": "abc123",
  "cwd": "/Users/sarah/myproject",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

Votre script peut analyser ce JSON et agir sur n'importe lequel de ces champs. Les hooks `UserPromptSubmit` obtiennent le texte `prompt` à la place, les hooks `SessionStart` obtiennent une `source` de `startup`, `resume`, `clear`, `compact`, ou `fork`, et ainsi de suite. Consultez [Champs d'entrée communs](/docs/fr/hooks#common-input-fields) dans la référence pour les champs partagés, et la section de chaque événement pour les schémas spécifiques à l'événement.

<h4 id="hook-output">
  Sortie du hook
</h4>

Votre script dit à Claude Code quoi faire ensuite en écrivant sur stdout ou stderr et en quittant avec un code spécifique. Le hook `PreToolUse` suivant bloque une commande :

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

* **Exit 0** : votre hook ne signale aucune objection via son code de sortie.
  * Pour un hook `PreToolUse`, cela n'approuve pas l'appel d'outil : le [flux de permission](/docs/fr/permissions) normal s'applique toujours.
  * Pour les hooks `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` et `PostModelSwitch`, Claude Code ajoute stdout qu'il [traite comme du texte brut](/docs/fr/hooks#exit-code-0) au contexte de Claude.
* **Exit 2** : Claude Code bloque l'action. Écrivez une raison sur stderr. Où elle aboutit dépend de l'événement : certains événements la transmettent à Claude comme retour afin qu'il puisse s'ajuster, d'autres la montrent à l'utilisateur, et quelques-uns, comme `ConfigChange` et `Elicitation`, ne font surface à aucun message. Certains événements ne peuvent pas être bloqués : pour `SessionStart` et autres, exit 2 affiche stderr à l'utilisateur et l'exécution continue. Consultez [comportement du code de sortie 2 par événement](/docs/fr/hooks#exit-code-2-behavior-per-event) pour la liste complète.
* **Tout autre code de sortie** : pour la plupart des événements, le résultat dépend de ce que votre hook a imprimé sur stdout :
  * Un objet analysé qui passe la validation du schéma : Claude Code ignore le code de sortie, le JSON seul décide du résultat, et le hook n'est pas signalé comme une erreur. Les exceptions par événement, comme `WorktreeCreate` échouant sur tout code de sortie non nul, sont listées dans la section [Sortie du code de sortie](/docs/fr/hooks#exit-code-output) de la référence.
  * Un objet analysé qui échoue la validation du schéma, ou stdout que Claude Code [essaie d'analyser comme JSON](/docs/fr/hooks#exit-code-0) mais qui n'est pas du JSON valide : une erreur non bloquante ; l'avis porte le message de validation ou d'analyse.
  * Stdout que Claude Code [traite comme du texte brut](/docs/fr/hooks#exit-code-0), ou stdout vide : l'action se poursuit comme une erreur non bloquante. La transcription affiche un avis `<hook name> hook error`, puis la première ligne de stderr préfixée par `Failed with non-blocking status code:`. Pour capturer le stderr complet, activez [le journal de débogage](/docs/fr/hooks#debug-hooks) avec `claude --debug` ou en exécutant `/debug` en milieu de session.

<h4 id="structured-json-output">
  Sortie JSON structurée
</h4>

Les codes de sortie vous donnent seulement deux options : bloquer ou rester silencieux. Pour plus de contrôle, quittez 0 et imprimez un objet JSON sur stdout à la place.

<Note>
  Utilisez exit 2 pour bloquer avec un message stderr, ou exit 0 avec JSON pour un contrôle structuré. Choisissez une approche par hook. Pour ce qui se passe lorsque vous les mélangez, consultez [Sortie du code de sortie](/docs/fr/hooks#exit-code-output).
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

Avec `"deny"`, Claude Code annule l'appel d'outil et transmet `permissionDecisionReason` à Claude.

Sur `PreToolUse`, Claude Code gère chaque valeur `permissionDecision` comme suit :

* `"allow"` : ignorer l'invite de permission interactive. Les règles de refus et d'ask, y compris les listes de refus gérées par l'entreprise, s'appliquent toujours, tout comme les invites pour les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) et pour les outils de connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) dans les sessions où ce paramètre atteint Claude Code
* `"deny"` : annuler l'appel d'outil et envoyer la raison à Claude
* `"ask"` : afficher l'invite de permission à l'utilisateur comme d'habitude

Une quatrième valeur, `"defer"`, est disponible en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`. Elle quitte le processus avec l'appel d'outil préservé afin qu'un wrapper SDK Agent puisse collecter l'entrée et reprendre. Consultez [Différer un appel d'outil pour plus tard](/docs/fr/hooks#defer-a-tool-call-for-later) dans la référence.

Un hook `PreModelSwitch` retourne le même champ `permissionDecision` : `"allow"` laisse un changement de modèle se poursuivre, et `"deny"` l'annule. `"ask"` vous demande de confirmer le changement lorsque vous exécutez `/model` dans une session interactive ; partout ailleurs, Claude Code traite `"ask"` comme un refus. Consultez [Contrôle de décision PreModelSwitch](/docs/fr/hooks#premodelswitch-decision-control).

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

Le matcher `"Edit|Write"` se déclenche uniquement lorsque Claude utilise l'outil `Edit` ou `Write`, pas lorsqu'il utilise `Bash`, `Read`, ou tout autre outil. Une virgule sépare les alternatives de la même manière, donc `"Edit, Write"` est équivalent. Consultez [Modèles de matcher](/docs/fr/hooks#matcher-patterns) pour savoir comment les noms simples et les expressions régulières sont évalués.

<Note>
  Claude peut également créer ou modifier des fichiers en exécutant des commandes shell. Si votre hook doit voir chaque modification de fichier, par exemple pour l'analyse de conformité ou l'enregistrement d'audit, ajoutez un hook [`Stop`](/docs/fr/hooks#stop) qui analyse l'arborescence de travail une fois par tour. Pour une couverture par appel à la place, correspondez également à `Bash|PowerShell` et faites en sorte que votre script liste les fichiers modifiés et non suivis avec `git status --porcelain`. La section [Entrée du hook PowerShell](/docs/fr/hooks#powershell) explique pourquoi correspondre à `Bash` seul n'est pas suffisant. Pour exécuter un hook lorsqu'un fichier spécifique change sur le disque, quel que soit ce qui l'a écrit, utilisez un hook [FileChanged](/docs/fr/hooks#filechanged).
</Note>

Chaque type d'événement correspond à un champ spécifique :

| Événement                                                                                                                                                       | Ce que le matcher filtre                                                                                             | Exemples de valeurs de matcher                                                                                                                                                                                                                                                 |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | nom de l'outil                                                                                                       | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                                  | comment la session a démarré                                                                                         | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                                         | quel drapeau CLI a déclenché la configuration                                                                        | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                                    | pourquoi la session s'est terminée                                                                                   | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                                  | type de notification                                                                                                 | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                                 | type d'agent                                                                                                         | `general-purpose`, `Explore`, `Plan`, ou noms d'agents personnalisés                                                                                                                                                                                                           |
| `PreCompact`, `PostCompact`                                                                                                                                     | ce qui a déclenché la compaction                                                                                     | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                                             | nom canonique du modèle vers lequel la session bascule, comme décrit sous [PreModelSwitch](/docs/fr/hooks#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                                  | type d'agent                                                                                                         | mêmes valeurs que `SubagentStart`                                                                                                                                                                                                                                              |
| `ConfigChange`                                                                                                                                                  | source de configuration                                                                                              | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `DirectoryAdded`                                                                                                                                                | comment le répertoire a été ajouté                                                                                   | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `StopFailure`                                                                                                                                                   | type d'erreur                                                                                                        | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                                            | raison du chargement                                                                                                 | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `Elicitation`                                                                                                                                                   | nom du serveur MCP                                                                                                   | vos noms de serveur MCP configurés                                                                                                                                                                                                                                             |
| `ElicitationResult`                                                                                                                                             | nom du serveur MCP                                                                                                   | mêmes valeurs que `Elicitation`                                                                                                                                                                                                                                                |
| `FileChanged`                                                                                                                                                   | noms de fichiers littéraux à surveiller (consultez [FileChanged](/docs/fr/hooks#filechanged))                             | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `UserPromptExpansion`                                                                                                                                           | nom de la commande                                                                                                   | vos noms de skill ou de commande                                                                                                                                                                                                                                               |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | pas de support de matcher                                                                                            | se déclenche toujours à chaque occurrence                                                                                                                                                                                                                                      |

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

Lorsque Claude Code ne peut pas déterminer quelles commandes l'entrée Bash exécute, il exécute votre hook indépendamment du modèle. Le [tableau de correspondance Bash](/docs/fr/hooks#bash-if-matching) couvre les formes de commande que Claude Code peut et ne peut pas affiner par sous-commande. Parce que le filtre est au mieux un effort, utilisez le [système de permission](/docs/fr/permissions) plutôt qu'un hook pour appliquer un allow ou deny dur.

Le champ `if` accepte les mêmes modèles que les règles de permission : `"Bash(git *)"`, `"Edit(*.ts)"`, et ainsi de suite. Pour correspondre à plusieurs noms d'outils, utilisez des gestionnaires séparés chacun avec sa propre valeur `if`, ou correspondez au niveau du `matcher` où l'alternation par pipe est supportée.

`if` ne fonctionne que sur les événements d'outils : `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` et `PermissionDenied`. L'ajouter à tout autre événement empêche le hook de s'exécuter.

<h3 id="configure-hook-location">
  Configurer l'emplacement du hook
</h3>

L'endroit où vous ajoutez un hook détermine son périmètre :

| Emplacement                                       | Périmètre                                                                                                                                       | Partageable                                                        |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- |
| `~/.claude/settings.json`                         | Tous vos projets                                                                                                                                | Non, local à votre machine                                         |
| `.claude/settings.json`                           | Projet unique                                                                                                                                   | Oui, peut être commité au repo                                     |
| `.claude/settings.local.json`                     | Projet unique                                                                                                                                   | Non, gitignored lorsque Claude Code enregistre un paramètre dedans |
| Paramètres de politique gérés                     | À l'échelle de l'organisation                                                                                                                   | Oui, contrôlé par l'administrateur                                 |
| [Plugin](/docs/fr/plugins/overview) `hooks/hooks.json` | Lorsque le plugin est activé                                                                                                                    | Oui, fourni avec le plugin                                         |
| [Skill](/docs/fr/skills) frontmatter                   | Le reste de la session une fois que le skill est invoqué. Consultez [Hooks dans les skills et les agents](/docs/fr/hooks#hooks-in-skills-and-agents) | Oui, défini dans le fichier du skill                               |
| [Subagent](/docs/fr/sub-agents) frontmatter            | Pendant que ce subagent s'exécute                                                                                                               | Oui, défini dans le fichier du subagent                            |

Exécutez [`/hooks`](/docs/fr/hooks#the-%2Fhooks-menu) dans Claude Code pour parcourir tous les hooks configurés regroupés par événement.

Pour désactiver les hooks, définissez `"disableAllHooks": true` dans votre fichier de paramètres. Claude Code lit la valeur restante après l'application de la [précédence des paramètres](/docs/fr/hooks#disable-or-remove-hooks), donc le fichier de paramètres d'un projet peut remplacer le vôtre. Les hooks configurés dans les paramètres gérés s'exécutent toujours sauf si `disableAllHooks` est également défini là. Pour la portée complète de chaque niveau, consultez [`disableAllHooks`](/docs/fr/settings-reference#disableallhooks).

Si vous modifiez les fichiers de paramètres directement pendant que Claude Code s'exécute, l'observateur de fichiers récupère normalement les modifications de hook automatiquement.

<h2 id="prompt-based-hooks">
  Hooks basés sur des invites
</h2>

Pour les décisions qui nécessitent un jugement plutôt que des règles déterministes, utilisez les hooks `type: "prompt"`. Au lieu d'exécuter une commande shell, Claude Code envoie votre invite et les données d'entrée du hook à un modèle Claude (Haiku par défaut) pour prendre la décision. Vous pouvez spécifier un modèle différent avec le champ `model` si vous avez besoin de plus de capacité.

Le seul travail du modèle est de retourner sa décision en JSON :

* `"ok": true` : l'action se poursuit
* `"ok": false` : ce qui se passe dépend de l'événement :
  * `Stop` et `SubagentStop` : la `reason` est renvoyée à Claude afin qu'il continue à travailler, sauf si la réponse définit également `"impossible": true` pour marquer la condition comme une condition qui ne peut jamais être satisfaite, auquel cas Claude Code autorise l'arrêt et le tour se termine
  * `PreToolUse` : l'appel d'outil est refusé ; par défaut le tour se termine et la `reason` du refus apparaît dans le chat sous forme de ligne d'avertissement. Définissez `continueOnBlock: true` sur le hook pour retourner à la place la `reason` à Claude comme erreur d'outil, afin qu'il puisse s'ajuster et continuer. Avant v2.1.210, la `reason` du refus était retournée à Claude comme erreur d'outil et le tour continuait
  * `PostToolUse` : par défaut le tour se termine et la `reason` apparaît dans le chat sous forme de ligne d'avertissement. Définissez `continueOnBlock: true` pour retourner la `reason` à Claude et continuer le tour à la place
  * `PostToolBatch`, `UserPromptSubmit` et `UserPromptExpansion` : le tour se termine et la `reason` apparaît dans le chat sous forme de ligne d'avertissement

Cet exemple utilise un hook `Stop` pour demander au modèle si toutes les tâches demandées sont complètes. Si le modèle retourne `"ok": false` parce que la condition n'est pas encore satisfaite, Claude continue à travailler et utilise la `reason` comme sa prochaine instruction :

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

Les hooks d'agent utilisent le format de réponse `"ok"` / `"reason"` avec un délai d'expiration par défaut plus long de 60 secondes et jusqu'à 50 tours d'utilisation d'outils. Ils ne supportent pas le champ `impossible` des hooks d'invite. Sur `ok: false`, Claude Code gère un hook d'agent de la même manière qu'il gère un hook d'invite avec `continueOnBlock: true` sur le même événement, donc sur `PreToolUse` et `PostToolUse` le tour continue ; les hooks d'agent n'ont pas de champ `continueOnBlock`. Consultez la [configuration des hooks d'agent](/docs/fr/hooks#agent-hook-configuration) pour les champs, y compris l'espace réservé `$ARGUMENTS` que Claude Code remplace par l'entrée JSON du hook.

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
  * `command`, `http`, `mcp_tool` : 10 minutes. Claude Code réduit cette valeur par défaut à 30 secondes pour les hooks `UserPromptSubmit`, `PreModelSwitch` et `PostModelSwitch`, et à 10 secondes pour `MessageDisplay`.
  * `prompt` : 30 secondes.
  * `agent` : 60 secondes.
  * Les hooks [`SessionEnd`](/docs/fr/hooks#sessionend) de tout type partagent un budget de 1,5 seconde. Si vos paramètres définissent un `timeout` par hook plus long, Claude Code augmente le budget pour correspondre, jusqu'à 60 secondes.
* Les hooks `PostToolUse` ne peuvent pas annuler les actions puisque l'outil a déjà été exécuté.
* Les hooks `PermissionRequest` se déclenchent lorsque Claude Code est sur le point de vous demander une permission.
  * En [mode non-interactif](/docs/fr/headless) avec l'indicateur `-p`, cette invite n'existe que lorsque le callback [`canUseTool`](/docs/fr/agent-sdk/permissions) du SDK Agent la fournit. Dans les exécutions `-p` simples ou avec `--permission-prompt-tool`, utilisez plutôt les hooks `PreToolUse` pour les décisions de permission automatisées.
  * Les sous-agents en arrière-plan ne peuvent pas afficher une invite en mode non-interactif. Claude Code exécute toujours les hooks pour leurs appels d'outils, et si aucun hook ne retourne une décision, il refuse l'appel. Dans une session interactive, les invites des sous-agents en arrière-plan s'affichent dans votre session principale et les hooks se déclenchent comme d'habitude.
* Les hooks `Stop` se déclenchent chaque fois que Claude termine sa réponse, pas seulement à la fin de la tâche. Ils ne se déclenchent pas sur les interruptions de l'utilisateur. Les erreurs API déclenchent [StopFailure](/docs/fr/hooks#stopfailure) à la place.
* Lorsque plusieurs hooks `PreToolUse` retournent [`updatedInput`](/docs/fr/hooks#pretooluse) pour réécrire les arguments d'un outil, le dernier à terminer gagne. Puisque les hooks s'exécutent en parallèle, l'ordre est non-déterministe. Évitez d'avoir plus d'un hook modifier l'entrée du même outil.

<h3 id="hooks-and-permission-modes">
  Hooks et modes de permission
</h3>

Les hooks `PreToolUse` se déclenchent avant toute vérification du mode de permission, dans tous les [modes de permission](/docs/fr/permission-modes), y compris `dontAsk`. Un hook qui retourne `permissionDecision: "deny"` bloque l'outil même en mode `bypassPermissions` ou avec `--dangerously-skip-permissions`. Cela vous permet d'appliquer une politique que les utilisateurs ne peuvent pas contourner en changeant leur mode de permission.

L'inverse n'est pas vrai : un hook retournant `"allow"` ne contourne pas les règles de refus des paramètres, et il ne peut pas supprimer l'invite pour les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) ou pour les outils de connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) dans les sessions où ce paramètre atteint Claude Code. Les hooks peuvent renforcer les restrictions mais pas les assouplir au-delà de ce que les règles de permission permettent.

<h3 id="hook-not-firing">
  Hook ne se déclenche pas
</h3>

Le hook est configuré mais ne s'exécute jamais.

* Exécutez `/hooks` et confirmez que le hook apparaît sous l'événement correct
* Vérifiez que le modèle de matcher correspond exactement au nom de l'outil. Les matchers sont sensibles à la casse
* Vérifiez que vous déclenchez le bon type d'événement : `PreToolUse` se déclenche avant l'exécution de l'outil, `PostToolUse` se déclenche après. Un hook `PermissionRequest` se déclenche lorsque Claude Code est sur le point de vous demander une permission ; consultez les [limitations](#limitations) pour les cas non-interactifs

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
* Si l'avis affiche un message de validation JSON, la sortie de votre hook a été analysée en JSON mais a échoué la validation du schéma. S'il affiche un message d'analyse JSON, la sortie ressemblait à un objet JSON mais n'était pas du JSON valide. Les deux se produisent même à la sortie 0.

  Pour corriger un échec d'analyse, construisez la charge utile avec un encodeur JSON tel que `jq` au lieu de la concaténation de chaînes, afin que les guillemets et les barres obliques inverses à l'intérieur des valeurs soient échappés. La section [Exit code output](/docs/fr/hooks#exit-code-output) de la référence couvre les combinaisons de code de sortie et JSON
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

<h3 id="hook-json-has-no-effect">
  Hook JSON n'a aucun effet
</h3>

Votre hook imprime du JSON valide, mais la décision ne prend pas effet et aucune erreur n'apparaît dans la transcription. Vérifiez quelle cause s'applique :

* **Sortie supplémentaire avant le JSON** : quelque chose d'autre écrit sur stdout en premier, généralement un `echo` inconditionnel dans votre profil shell, donc la sortie ne commence plus par `{` et Claude Code ne l'analyse pas en JSON. La cause et la correction suivent cette liste.
* **Un champ au mauvais niveau** : comparez le placement de chaque champ par rapport au format [JSON output](/docs/fr/hooks#json-output). Par exemple, `permissionDecision` appartient à l'intérieur de `hookSpecificOutput`, pas au niveau supérieur.

Lorsque Claude Code exécute un hook de commande sous forme de shell, un sans `args`, il génère `sh -c` sur macOS et Linux, Git Bash sur Windows, ou PowerShell lorsque Git Bash n'est pas installé par défaut. Ce shell est non-interactif, mais Git Bash et certaines configurations, comme `BASH_ENV` pointant vers `~/.bashrc`, sourcent toujours votre profil. Si ce profil contient des instructions `echo` inconditionnelles, la sortie est ajoutée au début de votre JSON du hook :

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

La sortie combinée ne commence plus par `{`, donc Claude Code traite tout stdout comme du texte brut et ignore le JSON. À la sortie 0, rien n'est signalé dans la transcription ; la tentative d'analyse est enregistrée uniquement dans le [journal de débogage](/docs/fr/hooks#debug-hooks). Pour corriger cela, enveloppez les instructions echo dans votre profil shell afin qu'elles ne s'exécutent que dans les shells interactifs :

```bash theme={null}
# In ~/.zshrc or ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

La variable `$-` contient les drapeaux du shell, et `i` signifie interactif. Les hooks s'exécutent dans des shells non-interactifs, donc l'echo est ignoré.

Lorsque votre hook retourne `permissionDecision` ou `additionalContext` au niveau supérieur au lieu de l'intérieur de `hookSpecificOutput`, le JSON s'analyse toujours, et Claude Code ignore les champs mal placés sans signaler une erreur. Pour voir quels champs il a ignorés, démarrez Claude Code avec `claude --debug` et recherchez dans le [journal de débogage](/docs/fr/hooks#debug-hooks) `Hook JSON output had unrecognized keys`.

<h3 id="debug-techniques">
  Techniques de débogage
</h3>

Appuyez sur `Ctrl+O` pour ouvrir la vue de transcription afin de vérifier le résultat d'une exécution de hook :

* **Exécution réussie** : vous ne voyez rien, sauf si le JSON du hook affiche quelque chose, comme `systemMessage` ou un retour du hook Stop.
  * Pour confirmer qu'un hook a été exécuté, vérifiez son effet, comme un fichier reformaté, ou activez la journalisation de débogage comme décrit ci-dessous et déclenchez le hook à nouveau
* **Erreur de blocage** : sur la plupart des événements, vous voyez le retour du hook. Lorsque le JSON du hook a pris une décision de blocage, le retour est la raison de cette décision ; sinon, c'est stderr du hook. Sur quelques événements, comme `ConfigChange` et `Elicitation`, un blocage ne surface aucun message.
* **Erreur sans blocage** : l'action a procédé, et vous voyez un avis `<hook name> hook error` avec une brève explication, comme la première ligne de stderr préfixée par `Failed with non-blocking status code:`, ou un message de validation ou d'analyse JSON.

Quelles combinaisons de code de sortie et JSON produisent chaque résultat, y compris les exceptions par événement, est défini dans la section [Exit code output](/docs/fr/hooks#exit-code-output) de la référence.

Pour les détails d'exécution complets incluant les hooks qui ont correspondu, leurs codes de sortie, stdout et stderr, lisez le journal de débogage. Démarrez Claude Code avec `claude --debug-file /tmp/claude.log` pour écrire dans un chemin connu, puis `tail -f /tmp/claude.log` dans un autre terminal. Si vous avez démarré sans ce drapeau, exécutez `/debug` en milieu de session pour activer la journalisation et trouver le chemin du journal.

<h2 id="learn-more">
  En savoir plus
</h2>

* [Référence des Hooks](/docs/fr/hooks) : schémas d'événements complets, format de sortie JSON, hooks asynchrones et hooks d'outils MCP
* [Considérations de sécurité](/docs/fr/hooks#security-considerations) : examinez avant de déployer les hooks dans des environnements partagés ou de production
* [Exemple de validateur de commande Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py) : implémentation de référence complète
