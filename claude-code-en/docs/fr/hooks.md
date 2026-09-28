> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence des hooks

> Référence pour les événements de hook Claude Code, le schéma de configuration, les formats d'entrée/sortie JSON, les codes de sortie, les hooks asynchrones, les hooks HTTP, les hooks de prompt et les hooks d'outils MCP.

<Tip>
  Pour un guide de démarrage rapide avec des exemples, consultez [Automatiser les actions avec les hooks](/docs/fr/hooks-guide).
</Tip>

Les hooks sont des commandes shell définies par l'utilisateur, des points de terminaison HTTP, des appels d'outils MCP, des prompts LLM ou des sous-agents qui s'exécutent automatiquement à des points spécifiques du cycle de vie de Claude Code. Claude Code déclenche les mêmes événements de hook partout où il s'exécute : les sessions dans le terminal, les extensions IDE, l'[application de bureau](/docs/fr/desktop-quickstart) et [Claude Code sur le web](/docs/fr/claude-code-on-the-web). Utilisez cette référence pour consulter les schémas d'événements, les options de configuration, les formats d'entrée/sortie JSON et les fonctionnalités avancées comme les hooks asynchrones, les hooks HTTP et les hooks d'outils MCP.

<h2 id="hook-lifecycle">
  Cycle de vie des hooks
</h2>

Claude Code exécute les hooks à des points spécifiques pendant une session. Lorsqu'un événement se déclenche et qu'un matcher correspond, Claude Code transmet le contexte JSON de l'événement à votre gestionnaire de hook. Pour les hooks de commande, l'entrée arrive sur stdin. Pour les hooks HTTP, elle arrive dans le corps de la requête POST. Votre gestionnaire peut alors inspecter l'entrée, prendre une action et éventuellement retourner une décision.

Les événements se déclenchent selon trois cadences :

* une fois par session : `SessionStart` et `SessionEnd`
* une fois par tour : `UserPromptSubmit`, `Stop` et `StopFailure`
* à chaque appel d'outil à l'intérieur de la boucle agentique : `PreToolUse` et `PostToolUse`, sauf les appels [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior), qui ignorent les deux

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=81b9256c1bbe8832553485f5d9e9c746" className="dark:hidden" alt="Diagramme du cycle de vie des hooks montrant Setup optionnel alimentant SessionStart, puis une boucle par tour contenant UserPromptSubmit, UserPromptExpansion pour les slash commands, la boucle agentique imbriquée (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), et Stop ou StopFailure, suivis de TeammateIdle, PreCompact, PostCompact et SessionEnd, avec Elicitation et ElicitationResult imbriqués dans l'exécution de l'outil MCP, PermissionDenied comme branche latérale de PermissionRequest pour les refus en mode auto, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged et DirectoryAdded comme événements asynchrones autonomes, PreModelSwitch comme événement séquentiel autonome qui s'exécute avant un changement de modèle demandé, PostModelSwitch comme événement asynchrone autonome qui s'exécute après le changement du modèle de la session, et MessageDisplay comme événement d'affichage uniquement qui s'exécute pendant que le texte du message de l'assistant est diffusé en continu" width="520" height="1336" data-path="images/hooks-lifecycle.svg" />

    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle-dark.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=c9b3d88487335f58cce0b52e2f9e7531" className="hidden dark:block" alt="Diagramme du cycle de vie des hooks montrant Setup optionnel alimentant SessionStart, puis une boucle par tour contenant UserPromptSubmit, UserPromptExpansion pour les slash commands, la boucle agentique imbriquée (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), et Stop ou StopFailure, suivis de TeammateIdle, PreCompact, PostCompact et SessionEnd, avec Elicitation et ElicitationResult imbriqués dans l'exécution de l'outil MCP, PermissionDenied comme branche latérale de PermissionRequest pour les refus en mode auto, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged et DirectoryAdded comme événements asynchrones autonomes, PreModelSwitch comme événement séquentiel autonome qui s'exécute avant un changement de modèle demandé, PostModelSwitch comme événement asynchrone autonome qui s'exécute après le changement du modèle de la session, et MessageDisplay comme événement d'affichage uniquement qui s'exécute pendant que le texte du message de l'assistant est diffusé en continu" width="520" height="1336" data-path="images/hooks-lifecycle-dark.svg" />
  </Frame>
</div>

Le tableau ci-dessous résume le moment où chaque événement se déclenche. La section [Événements de hook](#hook-events) documente le schéma d'entrée complet et les options de contrôle de décision pour chacun.

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

<h3 id="how-a-hook-resolves">
  Comment un hook se résout
</h3>

Pour voir comment l'événement, le matcher et le gestionnaire s'assemblent, considérez ce hook `PreToolUse` qui bloque les commandes shell destructrices.

<Tabs>
  <Tab title="macOS/Linux">
    Le `matcher` se limite aux appels d'outil Bash et la condition `if` se limite davantage aux sous-commandes Bash correspondant à `rm *`, donc `block-rm.sh` ne s'exécute que lorsque les deux filtres correspondent :

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

    Le script lit l'entrée JSON depuis stdin, extrait la commande et retourne une `permissionDecision` de `"deny"` si elle contient `rm -rf`. Enregistrez-le dans `.claude/hooks/block-rm.sh` dans votre projet et rendez-le exécutable avec `chmod +x .claude/hooks/block-rm.sh` pour que Claude Code puisse l'exécuter :

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

    Ce script, comme les autres exemples Bash sur cette page qui analysent l'entrée JSON, utilise `jq`, donc installez `jq` et assurez-vous qu'il se trouve sur votre `PATH` avant de les essayer.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Le matcher `Bash|PowerShell` couvre l'[outil PowerShell](#powershell) ainsi que Bash. Une seule règle `if` correspond aux appels d'un seul outil, donc chaque outil obtient son propre gestionnaire : le premier se limite aux sous-commandes Bash correspondant à `rm *`, le second aux commandes PowerShell correspondant à `Remove-Item *`. Les deux exécutent le même script via `powershell.exe` :

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash|PowerShell",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              },
              {
                "type": "command",
                "if": "PowerShell(Remove-Item *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Le drapeau `-NoProfile` ignore le chargement de votre profil PowerShell pour que le hook démarre rapidement, et `-ExecutionPolicy Bypass` permet à PowerShell d'exécuter le fichier de script local.

    Le script lit l'entrée JSON depuis stdin, extrait la commande et retourne une `permissionDecision` de `"deny"` si elle contient `rm -rf` ou `Remove-Item` suivi de `-Recurse`. Enregistrez-le dans `.claude/hooks/block-rm.ps1` dans votre projet :

    ```powershell theme={null}
    # .claude/hooks/block-rm.ps1
    $callInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $command = $callInput.tool_input.command

    if ($command -match 'rm -rf|Remove-Item.*-Recurse') {
      @{
        hookSpecificOutput = @{
          hookEventName = "PreToolUse"
          permissionDecision = "deny"
          permissionDecisionReason = "Destructive command blocked by hook"
        }
      } | ConvertTo-Json
    } else {
      exit 0  # no decision; normal permission flow applies
    }
    ```
  </Tab>
</Tabs>

Supposons maintenant que Claude Code décide d'exécuter `Bash "rm -rf /tmp/build"` par rapport à la configuration macOS/Linux. Voici ce qui se passe :

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" className="dark:hidden" alt="Diagramme de résolution du hook : PreToolUse se déclenche, le matcher vérifie la correspondance Bash, puis la condition if vérifie la correspondance Bash(rm *). Si les deux correspondent, la commande du hook s'exécute et retourne permissionDecision deny, donc l'appel d'outil est bloqué et Claude Code continue. Si l'une des vérifications ne correspond pas, le hook est ignoré et l'appel d'outil est autorisé à procéder." width="930" height="270" data-path="images/hook-resolution.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/hook-resolution-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e80af91f8507cee6bd51ac3c2dd92f63" className="hidden dark:block" alt="Diagramme de résolution du hook : PreToolUse se déclenche, le matcher vérifie la correspondance Bash, puis la condition if vérifie la correspondance Bash(rm *). Si les deux correspondent, la commande du hook s'exécute et retourne permissionDecision deny, donc l'appel d'outil est bloqué et Claude Code continue. Si l'une des vérifications ne correspond pas, le hook est ignoré et l'appel d'outil est autorisé à procéder." width="930" height="270" data-path="images/hook-resolution-dark.svg" />
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

| Emplacement                                       | Portée                                                                                                                             | Partageable                                                            |
| :------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `~/.claude/settings.json`                         | Tous vos projets                                                                                                                   | Non, local à votre machine                                             |
| `.claude/settings.json`                           | Projet unique                                                                                                                      | Oui, peut être commité dans le repo                                    |
| `.claude/settings.local.json`                     | Projet unique                                                                                                                      | Non, ignoré par git lorsque Claude Code enregistre un paramètre dedans |
| Paramètres de politique gérée                     | À l'échelle de l'organisation                                                                                                      | Oui, contrôlé par l'administrateur                                     |
| [Plugin](/docs/fr/plugins/overview) `hooks/hooks.json` | Lorsque le plugin est activé                                                                                                       | Oui, fourni avec le plugin                                             |
| [Skill](/docs/fr/skills) frontmatter                   | Le reste de la session une fois que le skill est invoqué. Consultez [Hooks dans les skills et agents](#hooks-in-skills-and-agents) | Oui, défini dans le fichier du skill                                   |
| [Subagent](/docs/fr/sub-agents) frontmatter            | Pendant que ce subagent s'exécute                                                                                                  | Oui, défini dans le fichier du subagent                                |

Les sessions cloud sur [Claude Code sur le web](/docs/fr/claude-code-on-the-web) ne lisent pas votre `~/.claude/settings.json` local. Dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments-configuration#permissions-and-tool-approval), Claude Code exécute également les hooks que l'opérateur a ensemencés à partir du `~/.claude/` de l'hôte du runner, et il exécute les hooks dans le fichier de paramètres gérés de l'image du runner lorsque ce fichier figure parmi les [sources gérées que Claude Code applique](/docs/fr/managed-settings#how-claude-code-combines-managed-sources), ce qui par défaut signifie uniquement lorsque ni les paramètres gérés par le serveur ni une politique Claude Code livrée par MDM ne fournissent le niveau géré. Consultez [ce qui se transfère de votre configuration](/docs/fr/cloud-environments#what-carries-over-from-your-setup) pour savoir quels fichiers de paramètres et plugins, et donc quels hooks, atteignent une session cloud.

Pour plus de détails sur la résolution des fichiers de paramètres, consultez [paramètres](/docs/fr/settings).

Les hooks des fichiers de paramètres, des paramètres de politique gérée et des plugins s'exécutent également à l'intérieur des [subagents](/docs/fr/sub-agents). Lorsqu'un subagent appelle un outil, les événements d'outil tels que `PreToolUse` et `PostToolUse` déclenchent les mêmes hooks configurés que dans la conversation principale, et l'entrée porte les champs d'entrée communs `agent_id` et `agent_type` [](#common-input-fields) qui identifient le subagent.

Les administrateurs d'entreprise peuvent utiliser `allowManagedHooksOnly` pour restreindre les hooks qui s'exécutent :

* Vos hooks utilisateur, projet, local et plugin sont bloqués. Les hooks des plugins forcément activés dans les paramètres gérés `enabledPlugins` sont exempts
* Claude Code restreint également votre [`statusLine`](/docs/fr/statusline), [`fileSuggestion`](/docs/fr/settings-reference#filesuggestion) et [`subagentStatusLine`](/docs/fr/statusline#subagent-status-lines) aux paramètres gérés
* Claude Code désactive également les plugins avec une [source `command`](/docs/fr/plugins/marketplace-reference#command-plugin-source), y compris les plugins forcément activés dans les paramètres gérés `enabledPlugins`, sauf si [`disableCommandPluginSources`](/docs/fr/settings-reference#disablecommandpluginsources) est explicitement défini à `false`. Les sources `command` nécessitent Claude Code v2.1.229 ou ultérieur
* Claude Code bloque également les commandes [`headersHelper`](/docs/fr/plugins/host-marketplace#authenticate-archive-downloads) du marketplace sauf si [`disableCommandPluginSources`](/docs/fr/settings-reference#disablecommandpluginsources) est explicitement défini à `false`, sauf pour un marketplace que les paramètres gérés eux-mêmes déclarent

Consultez [ce qui s'exécute sous `allowManagedHooksOnly`](/docs/fr/settings-reference#what-runs-under-allowmanagedhooksonly).

Les entrées de hook fusionnent entre les niveaux de paramètres plutôt que de se remplacer mutuellement : les paramètres utilisateur, projet et local ajoutent leurs propres hooks sans supprimer les hooks gérés, et le paramètre [`disableAllHooks`](#disable-or-remove-hooks) ne peut pas désactiver les hooks gérés en dehors des paramètres gérés.

Les [listes blanches de hooks HTTP](/docs/fr/settings-reference#hook-and-skill-settings) s'appliquent aux hooks de chaque source, y compris les paramètres de politique gérée :

* `allowedHttpHookUrls` : lorsqu'il est défini à n'importe quel niveau de paramètres, Claude Code exécute un gestionnaire de hook HTTP uniquement si son URL correspond à la liste blanche fusionnée
* `httpHookAllowedEnvVars` : lorsqu'il est défini, Claude Code n'interpose que les variables d'environnement de cette liste dans les en-têtes de hook

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

Les traits d'union dans l'ensemble de correspondance exacte nécessitent Claude Code v2.1.195 ou ultérieur. Sur les versions antérieures, un nom avec trait d'union comme `code-reviewer` est évalué comme une expression régulière non ancrée, donc il se déclenche également pour `senior-code-reviewer` ; ancrez-le comme `^code-reviewer$` sur ces versions pour correspondre uniquement à ce nom.

`FileChanged` et `StopFailure` utilisent un ensemble de correspondance exacte plus étroit contenant uniquement des lettres, des chiffres, `_` et `|`. Un trait d'union, un espace ou une virgule dans un matcher pour ces deux événements le maintient sur le chemin de l'expression régulière, et seul `|` sépare les alternatives. Tous les autres événements avec support de matcher dans le tableau qui suit acceptent `|` ou `,`.

L'événement `FileChanged` ne suit pas ces règles lors de la construction de sa liste de surveillance. Consultez [FileChanged](#filechanged).

Chaque type d'événement correspond sur un champ différent :

| Événement                                                                                                                                         | Ce que le matcher filtre                                                                                    | Exemples de valeurs de matcher                                                                                                                                                                                                                                                 |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | nom de l'outil                                                                                              | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                    | comment la session a démarré                                                                                | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                           | quel drapeau CLI a déclenché la configuration                                                               | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                      | pourquoi la session s'est terminée                                                                          | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                    | type de notification                                                                                        | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                   | type d'agent                                                                                                | `general-purpose`, `Explore`, `Plan`, noms d'agents personnalisés ou noms limités au plugin comme `^my-plugin:reviewer$`                                                                                                                                                       |
| `PreCompact`, `PostCompact`                                                                                                                       | ce qui a déclenché la compaction                                                                            | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                               | nom canonique du modèle vers lequel la session bascule, comme décrit sous [PreModelSwitch](#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                    | type d'agent                                                                                                | mêmes valeurs que `SubagentStart`                                                                                                                                                                                                                                              |
| `ConfigChange`                                                                                                                                    | source de configuration                                                                                     | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `CwdChanged`                                                                                                                                      | pas de support de matcher                                                                                   | se déclenche toujours à chaque changement de répertoire                                                                                                                                                                                                                        |
| `DirectoryAdded`                                                                                                                                  | comment le répertoire a été ajouté                                                                          | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `FileChanged`                                                                                                                                     | noms de fichiers littéraux à surveiller (consultez [FileChanged](#filechanged))                             | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `StopFailure`                                                                                                                                     | type d'erreur                                                                                               | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                              | raison du chargement                                                                                        | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `UserPromptExpansion`                                                                                                                             | nom de la commande                                                                                          | vos noms de skill ou de commande                                                                                                                                                                                                                                               |
| `Elicitation`                                                                                                                                     | nom du serveur MCP                                                                                          | vos noms de serveur MCP configurés                                                                                                                                                                                                                                             |
| `ElicitationResult`                                                                                                                               | nom du serveur MCP                                                                                          | mêmes valeurs que `Elicitation`                                                                                                                                                                                                                                                |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | pas de support de matcher                                                                                   | se déclenche toujours à chaque occurrence                                                                                                                                                                                                                                      |

Correspondre à `StopFailure` sur `cloud_credential_error` nécessite Claude Code v2.1.267 ou ultérieur, la première version qui signale les échecs de chargement des identifiants sous cette valeur plutôt que `server_error` ou `unknown`.

Pour la plupart des événements, Claude Code évalue le matcher par rapport à un champ de l'[entrée JSON](#hook-input-and-output) qu'il envoie à votre hook sur stdin. Pour les événements d'outil, ce champ est `tool_name`. Pour `PreModelSwitch` et `PostModelSwitch`, Claude Code évalue le matcher par rapport au nom canonique qu'il dérive de `to_model`, comme décrit sous [PreModelSwitch](#premodelswitch). Chaque section [événement de hook](#hook-events) liste l'ensemble complet des valeurs de matcher et le schéma d'entrée pour cet événement.

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

Si vous ajoutez un champ `matcher` à un événement sans support de matcher, il est silencieusement ignoré.

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
* **[Hooks de prompt](#prompt-and-agent-hook-fields)** (`type: "prompt"`) : envoient un prompt à un modèle Claude pour une évaluation en un seul tour. Le modèle retourne sa décision en JSON. Consultez [Hooks basés sur des prompts](#prompt-based-hooks).
* **[Hooks d'agent](#prompt-and-agent-hook-fields)** (`type: "agent"`) : lancent un subagent qui peut utiliser des outils comme Read, Grep et Glob pour vérifier les conditions avant de retourner une décision. Les hooks d'agent sont expérimentaux et peuvent changer. Consultez [Hooks basés sur des agents](#agent-based-hooks).

Tous les hooks correspondants s'exécutent en parallèle. Si vous définissez le même gestionnaire dans plus d'un fichier de paramètres, il s'exécute une fois. Une copie du même gestionnaire d'un plugin ou d'un skill reste séparée.

Les gestionnaires s'exécutent dans le répertoire courant avec l'environnement de Claude Code. Si le répertoire courant n'existe plus, par exemple un worktree ou un répertoire temporaire qu'un autre shell a supprimé en cours de session, Claude Code exécute les hooks de commande à partir du premier de ceux-ci qui existe toujours : le répertoire dans lequel la session a démarré, la racine du projet, votre répertoire personnel ou le répertoire temporaire du système. Claude Code enregistre un avertissement nommant le répertoire de secours dans le [journal de débogage](#debug-hooks).

La variable d'environnement `$CLAUDE_CODE_REMOTE` est `"true"` dans les environnements web distants et n'est pas définie dans le CLI local. Claude Code v2.1.199 et ultérieur définit [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/fr/env-vars) à l'ID de session [Contrôle à distance](/docs/fr/remote-control) tandis que la session locale a une connexion Contrôle à distance active.

<h4 id="common-fields">
  Champs communs
</h4>

Ces champs s'appliquent à tous les types de hooks :

| Champ           | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :-------------- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | oui    | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` ou `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `if`            | non    | Syntaxe de règle de permission pour filtrer quand ce hook s'exécute, comme `"Bash(git *)"` ou `"Edit(*.ts)"`. Le hook de commande ne s'exécute que si l'appel d'outil correspond au modèle. Consultez le [tableau de correspondance Bash](#bash-if-matching) ci-dessous pour voir comment les modèles Bash s'évaluent par rapport aux sous-commandes, `$()` et aux backticks. Évalué uniquement sur les événements d'outil : `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` et `PermissionDenied`. Sur les autres événements, un hook avec `if` défini ne s'exécute jamais. Utilise la même syntaxe que les [règles de permission](/docs/fr/permissions)                                                             |
| `timeout`       | non    | Secondes avant annulation. Claude Code ne l'applique pas sur un hook de commande que vous exécutez avec [`async: true`](#run-hooks-in-the-background). Valeurs par défaut : 600 pour `command`, `http` et `mcp_tool` ; 30 pour `prompt` ; 60 pour `agent`. Claude Code abaisse la valeur par défaut de `command`, `http` et `mcp_tool` à 30 sur [`UserPromptSubmit`](#userpromptsubmit), [`PreModelSwitch`](#premodelswitch) et [`PostModelSwitch`](#postmodelswitch), et à 10 sur [`MessageDisplay`](#messagedisplay). Les hooks [`SessionEnd`](#sessionend) partagent un budget de 1,5 seconde ; si vos paramètres définissent un `timeout` par hook plus long, Claude Code augmente le budget pour correspondre, jusqu'à 60 secondes |
| `statusMessage` | non    | Message de spinner personnalisé affiché pendant l'exécution du hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `once`          | non    | Si `true`, Claude Code supprime le hook après sa première exécution réussie. Une exécution qui échoue, bloque avec le code de sortie 2 ou expire laisse le hook en place, donc il s'exécute à nouveau au prochain événement correspondant. Honoré uniquement pour les hooks déclarés dans le [frontmatter des skills](#hooks-in-skills-and-agents) ; ignoré dans les fichiers de paramètres et le frontmatter des agents                                                                                                                                                                                                                                                                                                                |

Le champ `if` contient exactement une règle de permission. Il n'y a pas de syntaxe `&&`, `||` ou de liste pour combiner les règles ; pour appliquer plusieurs conditions, définissez un gestionnaire de hook séparé pour chacune.

Dans une condition `if` pour un outil de fichier, un modèle de répertoire à un seul segment comme `"Edit(src/**)"` correspond uniquement au répertoire `src` dans le répertoire de travail et aux fichiers sous celui-ci. Pour correspondre à un répertoire nommé `src` à n'importe quelle profondeur, écrivez `"Edit(**/src/**)"`. Avant v2.1.214, `"Edit(src/**)"` correspondait à un répertoire nommé `src` à n'importe quelle profondeur sous le répertoire de travail.

<span id="bash-if-matching" />Pour les modèles Bash, le fait que votre commande de hook s'exécute dépend de la forme du modèle et de la commande Bash que Claude invoque. Les affectations `VAR=value` en début sont supprimées avant la correspondance.

| Modèle `if`        | Commande Bash               | Le hook s'exécute-t-il ? | Pourquoi                                                                                                                                                            |
| :----------------- | :-------------------------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Bash(git *)`      | `FOO=bar git push`          | oui                      | les affectations en début sont supprimées ; `git push` correspond                                                                                                   |
| `Bash(git *)`      | `npm test && git push`      | oui                      | chaque sous-commande est vérifiée ; `git push` correspond                                                                                                           |
| `Bash(rm *)`       | `echo $(rm -rf /)`          | oui                      | les commandes à l'intérieur de `$()` et des backticks sont vérifiées ; `rm -rf /` correspond                                                                        |
| `Bash(rm *)`       | `echo $(date)`              | non                      | aucune sous-commande ne correspond à `rm *`                                                                                                                         |
| `Bash(cat *)`      | `echo before $(date) after` | non                      | une substitution peut se situer à n'importe quelle position d'argument, donc la commande complète et `date` sont tous deux vérifiés ; aucun ne correspond à `cat *` |
| `Bash(git *)`      | `$TOOL git push`            | oui                      | Claude Code ne peut pas dire à quoi le nom de la commande se développe, donc il exécute le hook                                                                     |
| `Bash(git push *)` | `echo $(date)`              | oui                      | les modèles qui spécifient plus que le nom de la commande exécutent le hook de toute façon sur `$()`, les backticks ou `$VAR`                                       |

Lorsque Claude Code ne peut pas déterminer quelles commandes l'entrée Bash exécute, il exécute votre hook indépendamment du modèle. Parce que le filtre `if` est au mieux un effort, utilisez le [système de permission](/docs/fr/permissions) plutôt qu'un hook pour appliquer une autorisation ou un refus strict.

<h4 id="command-hook-fields">
  Champs des hooks de commande
</h4>

En plus des [champs communs](#common-fields), les hooks de commande acceptent ces champs :

| Champ         | Requis | Description                                                                                                                                                                                                                                                                                                                                                             |
| :------------ | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | oui    | Commande shell à exécuter. Avec `args`, l'exécutable à lancer directement. Consultez [Forme exec et forme shell](#exec-form-and-shell-form)                                                                                                                                                                                                                             |
| `args`        | non    | Liste d'arguments. Lorsqu'elle est présente, `command` est résolu comme un exécutable et lancé directement avec `args` comme vecteur d'arguments, sans shell. Consultez [Forme exec et forme shell](#exec-form-and-shell-form)                                                                                                                                          |
| `async`       | non    | Si `true`, s'exécute en arrière-plan sans bloquer. Consultez [Exécuter les hooks en arrière-plan](#run-hooks-in-the-background)                                                                                                                                                                                                                                         |
| `asyncRewake` | non    | Si `true`, s'exécute en arrière-plan et réveille Claude au code de sortie 2. Le stderr du hook, ou stdout s'il est vide, est affiché à Claude comme un rappel système afin qu'il puisse réagir à un échec en arrière-plan de longue durée                                                                                                                               |
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

Les hooks de plugin substituent également les valeurs [`${user_config.*}`](/docs/fr/plugins/manifest-reference#user-configuration), en forme exec uniquement : la valeur est substituée dans `command` et dans chaque élément `args` comme une chaîne brute, donc aucun shell ne la réanalyse.

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

La gestion des erreurs diffère des hooks de commande ; consultez [Gestion des réponses HTTP](#http-response-handling).

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

Claude Code lit le contenu textuel de l'outil de la même manière qu'il lit stdout d'un hook de commande, en suivant la [règle d'analyse sous le code de sortie 0](#exit-code-0). Si le serveur nommé n'est pas connecté, ou si l'outil retourne `isError: true`, le hook produit une erreur non-bloquante et l'exécution continue.

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

Un hook `mcp_tool` peut s'exécuter uniquement une fois que Claude Code a rendu les serveurs MCP de la session disponibles aux hooks. `SessionStart` et `Setup` peuvent se déclencher avant ce point :

* **Au lancement** : `SessionStart` se déclenche avant que les serveurs ne soient disponibles, y compris lorsque vous lancez avec `--continue` ou `--resume`. Claude Code ignore les hooks `mcp_tool` de l'événement sans appeler leurs outils, et le [journal de débogage](#debug-hooks) enregistre `mcp_tool hooks are not available for the 'SessionStart' hook event (no MCP client context)`.
* **Plus tard dans une session en cours** : après `/clear` ou une compaction, `SessionStart` se déclenche à nouveau avec les serveurs déjà disponibles, et ses hooks `mcp_tool` s'exécutent.
* **Sur `Setup`** : `Setup` se déclenche toujours avant que les serveurs ne soient disponibles, donc Claude Code ignore ses hooks `mcp_tool` à chaque fois et enregistre le même message nommant `Setup`.

Par exemple, cette configuration appelle l'outil `load_context` sur le serveur MCP `my_server` à partir d'un hook `SessionStart` sans matcher, donc elle s'applique à chaque source `SessionStart` :

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "load_context"
          }
        ]
      }
    ]
  }
}
```

Lorsque vous exécutez `claude`, Claude Code ignore ce hook, n'appelle jamais `load_context` et écrit le message `no MCP client context` dans le journal de débogage. Exécutez `/clear` dans cette même session et le hook s'exécute et appelle `load_context`. Un hook `type: "command"` sur `SessionStart` s'exécute au lancement, donc utilisez-en un pour tout ce dont la session a besoin dès son premier tour.

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

* `${CLAUDE_PROJECT_DIR}` : la racine du projet où la session a démarré. Claude Code définit également cette variable dans l'environnement des [serveurs MCP stdio](/docs/fr/mcp#option-3-add-a-local-stdio-server) et des serveurs LSP de plugin.
* `${CLAUDE_PLUGIN_ROOT}` : le répertoire d'installation du plugin, pour les scripts fournis avec un [plugin](/docs/fr/plugins/overview). Consultez [variables d'environnement du plugin](/docs/fr/plugins/manifest-reference#environment-variables) pour savoir comment le chemin se comporte lors des mises à jour.
* `${CLAUDE_PLUGIN_DATA}` : le [répertoire de données persistantes](/docs/fr/plugins/components#path-variables-and-persistent-data) du plugin, pour les dépendances et l'état qui doivent survivre aux mises à jour du plugin.

<Note>
  **Les worktrees sont différents.** Si Claude entre dans un [worktree](/docs/fr/worktrees) pendant la session, Claude Code garde `${CLAUDE_PROJECT_DIR}` où il était et passe le chemin du worktree à vos hooks d'une manière différente :

  * **`${CLAUDE_PROJECT_DIR}` reste en place** : il pointe toujours vers la racine du projet où la session a démarré, donc une commande comme `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` exécute toujours le script dans le checkout principal.
  * **`cwd` suit Claude** : le champ `cwd` dans l'[entrée JSON](#common-input-fields) du hook est la racine du worktree après que Claude entre dans un worktree, et le nouveau répertoire après que Claude exécute `cd`. Lisez-le lorsqu'un hook a besoin de savoir dans quel répertoire Claude travaille.
</Note>

Préférez la [forme exec](#exec-form-and-shell-form) pour tout hook qui référence un placeholder de chemin. En forme shell, enveloppez chaque placeholder entre guillemets doubles.

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

    Consultez la [référence des composants de plugin](/docs/fr/plugins/components#hooks) pour plus de détails sur la création de hooks de plugin.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks dans les skills et agents
</h3>

En plus des fichiers de paramètres et des plugins, les hooks peuvent être définis directement dans les [skills](/docs/fr/skills) et les [subagents](/docs/fr/sub-agents) en utilisant le frontmatter, dans le même format de configuration que les hooks basés sur les paramètres. La durée pendant laquelle Claude Code les garde enregistrés dépend du composant :

* **Hooks de subagent** : Claude Code les exécute uniquement pendant que ce subagent s'exécute et les supprime lorsqu'il se termine. Claude Code convertit un hook `Stop` ici en `SubagentStop`, l'événement qu'il déclenche lorsqu'un subagent se termine.
* **Hooks de skill** : Claude Code les enregistre lorsque vous ou Claude invoquez le skill et continue à les exécuter pour le reste de la session, sur les tours après le tour du skill lui-même. Pour que Claude Code supprime un hook après sa première exécution réussie à la place, définissez [`once: true`](#common-fields) sur celui-ci.

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

Les subagents utilisent le même format dans leur frontmatter YAML.

Les hooks de frontmatter dans un skill de projet suivent la même [règle de confiance de l'espace de travail que les hooks dans les fichiers de paramètres](#workspace-trust). Claude Code les enregistre lorsque vous ou Claude invoquez le skill, y compris dans une exécution `-p` dans un dossier que vous n'avez pas approuvé.

Les hooks de frontmatter dans un subagent de projet s'exécutent uniquement après que vous acceptiez le [dialogue de confiance de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) pour le dossier d'où provient le fichier de l'agent. Une session `-p` ne compte pas comme l'accepter. [Ce qui s'exécute avant que vous approuviez un dossier](/docs/fr/permissions#what-runs-before-you-trust-a-folder) compare cela avec la règle du fichier de paramètres, et la page des subagents liste [quels scopes sont exempts](/docs/fr/sub-agents#hooks-in-subagent-frontmatter). Avant v2.1.218, ces hooks pouvaient s'exécuter à partir de dossiers que vous n'aviez pas approuvés.

<h3 id="the-/hooks-menu">
  Le menu `/hooks`
</h3>

Tapez `/hooks` dans Claude Code pour ouvrir un navigateur en lecture seule pour vos hooks configurés. Le menu affiche chaque événement de hook avec un nombre de hooks configurés, vous permet d'explorer les matchers et affiche les détails complets de chaque gestionnaire de hook. Utilisez-le pour vérifier la configuration, vérifier à partir de quel fichier de paramètres un hook provient ou inspecter la commande, le prompt ou l'URL d'un hook.

Le menu affiche les cinq types de hooks : `command`, `prompt`, `agent`, `http` et `mcp_tool`. Chaque hook est étiqueté avec un préfixe `[type]` et une source indiquant où il a été défini :

* `User Settings` : de `~/.claude/settings.json`
* `Project Settings` : de `.claude/settings.json`
* `Local Settings` : de `.claude/settings.local.json`
* `Plugin Hooks` : du `hooks/hooks.json` d'un plugin
* `Session Hooks` : enregistré en mémoire pour la session actuelle

Sélectionner un hook ouvre une vue détaillée affichant son événement, son matcher, son type, son fichier source et la commande, le prompt ou l'URL complet. Le menu est en lecture seule : pour ajouter, modifier ou supprimer des hooks, éditez directement le JSON des paramètres ou demandez à Claude de faire la modification.

<h3 id="disable-or-remove-hooks">
  Désactiver ou supprimer les hooks
</h3>

Pour supprimer un hook, supprimez son entrée du fichier de paramètres JSON.

Pour désactiver temporairement tous les hooks sans les supprimer, définissez `"disableAllHooks": true` dans votre fichier de paramètres. Claude Code lit la valeur restante après que la [précédence des paramètres](/docs/fr/settings#settings-precedence) s'applique, donc un `"disableAllHooks": false` dans le `.claude/settings.json` d'un projet remplace un `true` dans vos paramètres utilisateur. Pour désactiver les hooks pour une exécution quelle que soit la configuration du projet, passez `--settings '{"disableAllHooks": true}'`, qui prend la précédence sur les paramètres du projet et locaux. Il n'y a aucun moyen de désactiver un hook individuel tout en le gardant dans la configuration.

Le paramètre `disableAllHooks` respecte la hiérarchie des paramètres gérés. Si un administrateur a configuré des hooks via les paramètres de politique gérée, `disableAllHooks` défini dans les paramètres utilisateur, projet ou local ne peut pas désactiver ces hooks gérés. Seul `disableAllHooks` défini au niveau des paramètres gérés peut désactiver les hooks gérés. Pour la portée complète de chaque niveau, consultez [`disableAllHooks`](/docs/fr/settings-reference#disableallhooks).

Les éditions directes des hooks dans les fichiers de paramètres sont normalement détectées automatiquement par le moniteur de fichiers.

<h2 id="hook-input-and-output">
  Entrée et sortie des hooks
</h2>

Les hooks de commande reçoivent les données JSON via stdin et communiquent les résultats via les codes de sortie, stdout et stderr. Les hooks HTTP reçoivent le même JSON que le corps de la requête POST et communiquent les résultats via le corps de la réponse HTTP. Cette section couvre les champs et le comportement communs à tous les événements. Chaque section d'événement sous [Événements de hook](#hook-events) inclut son schéma d'entrée spécifique et les options de contrôle de décision.

Sur macOS et Linux, les hooks de commande s'exécutent dans leur propre session sans terminal de contrôle. Le processus de hook et tous les processus enfants ne peuvent pas ouvrir `/dev/tty` ou envoyer des séquences d'échappement directement à l'interface Claude Code. Windows n'a pas de `/dev/tty`.

Pour afficher un message à l'utilisateur sur n'importe quelle plateforme, retournez [`systemMessage`](#json-output) dans la sortie JSON. Certains événements le rejettent ou le livrent ailleurs, et chaque [section d'événement](#hook-events) le précise. Pour déclencher une notification de bureau, définir un titre de fenêtre ou sonner la cloche, retournez [`terminalSequence`](#emit-terminal-notifications) à la place.

<h3 id="common-input-fields">
  Champs d'entrée communs
</h3>

Les événements de hook reçoivent ces champs en JSON, en plus des champs spécifiques à l'événement documentés dans chaque section [événement de hook](#hook-events). Pour les hooks de commande, ce JSON arrive via stdin. Pour les hooks HTTP, il arrive dans le corps de la requête POST.

| Champ             | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `session_id`      | Identifiant de session actuel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `prompt_id`       | UUID identifiant le prompt utilisateur actuellement traité. Correspond à l'[attribut `prompt.id` sur les événements OpenTelemetry](/docs/fr/monitoring-usage#event-correlation-attributes), afin que vous puissiez corréler la sortie du hook avec la télémétrie pour un seul prompt. Absent jusqu'à la première entrée utilisateur. Nécessite Claude Code v2.1.196 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `transcript_path` | Chemin vers le JSON de conversation. Le fichier de transcription est écrit de manière asynchrone et peut être en retard par rapport à la conversation en mémoire, il se peut donc qu'il n'inclue pas encore les messages les plus récents du tour actuel lorsqu'un hook se déclenche. Les hooks qui ont besoin du texte final de l'assistant du tour actuel doivent utiliser `last_assistant_message` sur [Stop](#stop) et [SubagentStop](#subagentstop) au lieu de lire la transcription                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `cwd`             | Répertoire de travail courant lorsque le hook est invoqué                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `scratchpad_dir`  | Chemin vers le répertoire scratchpad de la session, où Claude conserve les fichiers de travail temporaires. Absent lorsque la session n'a pas de scratchpad ou que le répertoire temporaire n'est pas disponible. Nécessite Claude Code v2.1.257 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `permission_mode` | [Mode de permission](/docs/fr/permissions#permission-modes) actuel : `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` ou `"bypassPermissions"`. Le mode étiqueté **Manuel** arrive comme `"default"`, jamais comme `"manual"`, afin que les scripts qui correspondent à `"default"` continuent de fonctionner. Tous les événements ne reçoivent pas ce champ. Consultez l'exemple JSON de chaque [événement de hook](#hook-events)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `effort`          | Objet avec un champ `level` contenant le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) en vigueur lorsque le hook s'exécute : `"low"`, `"medium"`, `"high"`, `"xhigh"` ou `"max"`. Si vous définissez un niveau que le modèle actif ne supporte pas, `level` rapporte le niveau que Claude Code a exécuté à la place ; [Ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level) explique comment il choisit ce niveau. Ultracode n'est pas un niveau distinct et est signalé comme `"xhigh"`. L'objet correspond au champ `effort` de la [ligne de statut](/docs/fr/statusline#available-data). Présent pour les événements qui se déclenchent dans un contexte d'utilisation d'outil, tels que `PreToolUse`, `PostToolUse`, `Stop` et `SubagentStop`, lorsque le modèle actuel supporte le paramètre d'effort. Le niveau est également disponible pour les commandes de hook et l'outil Bash en tant que variable d'environnement `$CLAUDE_EFFORT`. |
| `hook_event_name` | Nom de l'événement qui s'est déclenché                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

Lors de l'exécution avec `--agent` ou à l'intérieur d'un subagent, deux champs supplémentaires sont inclus :

| Champ        | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Identifiant unique pour le subagent. Présent uniquement lorsque le hook se déclenche à l'intérieur d'un appel de subagent. Utilisez ceci pour distinguer les appels de hook de subagent des appels du thread principal.                                                                                                                                                                                                                                               |
| `agent_type` | Nom de l'agent (par exemple, `"Explore"` ou `"security-reviewer"`). Présent lorsque la session utilise `--agent` ou que le hook se déclenche à l'intérieur d'un subagent. Pour les subagents, le type du subagent prend précédence sur la valeur `--agent` de la session. Consultez [SubagentStart](#subagentstart) pour les valeurs que les subagents personnalisés et fournis par un plugin rapportent et comment écrire un matcher contre un nom scoped du plugin. |

Seuls les hooks [`SessionStart`](#sessionstart) peuvent recevoir un champ `model`, et Claude Code ne l'inclut pas toujours. Les hooks [`PreModelSwitch`](#premodelswitch) et [`PostModelSwitch`](#postmodelswitch) reçoivent `from_model` et `to_model` à la place, utilisez donc un hook PostModelSwitch pour suivre le modèle au fur et à mesure qu'il change pendant une session.

Il n'y a pas de variable d'environnement `$CLAUDE_MODEL`. Le hook peut lire `$ANTHROPIC_MODEL` si vous la définissez dans votre shell, mais cette valeur ne change pas lorsque vous changez de modèle avec `/model` pendant une session.

Un processus de hook hérite de l'environnement parent, à l'exception des variables d'exportateur `OTEL_*` que Claude Code [supprime de chaque sous-processus qu'il génère](/docs/fr/monitoring-usage#administrator-configuration) et, lorsque [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/fr/env-vars#variables) est défini sur `1`, les variables qu'il supprime.

Par exemple, un hook `PreToolUse` pour une commande Bash reçoit ceci sur stdin :

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "scratchpad_dir": "/tmp/claude-1000/-home-user-my-project/abc123/scratchpad",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite",
    "timeout": 120000,
    "run_in_background": false
  },
  "tool_use_id": "toolu_01ABC123..."
}
```

Les champs `tool_name`, `tool_input` et `tool_use_id` sont spécifiques à l'événement. Chaque section [événement de hook](#hook-events) documente les champs supplémentaires pour cet événement.

<h3 id="exit-code-output">
  Sortie du code de sortie
</h3>

Le code de sortie de votre commande de hook indique à Claude Code si l'action doit procéder, être bloquée ou être ignorée. Le code de sortie n'agit pas seul. Claude Code lit les [champs de sortie JSON](#json-output) depuis stdout sur chaque code de sortie, pas seulement 0, et pour les événements qui utilisent le modèle de décision standard, un objet analysé qui passe la validation du schéma prend effet aux côtés du code. Le blocage d'exit 2 est le seul résultat que JSON ne peut pas remplacer.

Deux tableaux possèdent les exceptions par événement : [Comportement du code de sortie 2 par événement](#exit-code-2-behavior-per-event) dit ce que les codes de sortie font pour chaque événement, et [Contrôle de décision](#decision-control) dit quels champs de décision chaque événement honore. Les champs universels tels que `systemMessage` fonctionnent sur la plupart des événements et sont listés dans le tableau [Sortie JSON](#json-output).

<h4 id="exit-code-0">
  Exit code 0
</h4>

Exit 0 signifie succès, et c'est le code de sortie prévu lorsque vous imprimez JSON pour un contrôle structuré.

Pour la plupart des événements, Claude Code écrit stdout dans le journal de débogage et ne l'affiche pas dans la transcription. Les exceptions sont `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` et `PostModelSwitch`, où Claude Code ajoute stdout en texte brut comme contexte que Claude peut voir et sur lequel agir.

Que Claude Code lise votre stdout comme [sortie JSON](#json-output) ou comme texte brut dépend de la façon dont il commence et se termine, en ignorant les espaces blancs environnants :

* **Commence par `{` et se termine par `}`** : Claude Code l'analyse comme JSON. Lorsque la sortie est deux lignes ou plus qui s'analysent chacune comme JSON seules, et aucune ligne n'est un objet [sortie JSON](#json-output) qui définit un champ, Claude Code traite la sortie entière comme du texte brut. Lorsque l'une de ces lignes définit un champ, la sortie entière est un échec d'analyse, décrit ci-dessous.
* **Commence par `{` mais ne se termine pas par `}`** : Claude Code le traite comme du texte brut.
* **Commence par n'importe quoi d'autre** : Claude Code le traite comme du texte brut, un tableau JSON ou une chaîne JSON entre guillemets incluse.

Pour les événements qui utilisent le modèle de décision standard, exit 0 avec un objet analysé qui échoue la validation du schéma est une erreur non-bloquante : l'action procède, et la transcription affiche un avis `<hook name> hook error` avec le message de validation. La même chose se produit sur tout code de sortie autre que 2, tandis que [exit 2 bloque toujours](#exit-code-2).

Pour les événements qui utilisent le modèle de décision standard, lorsque Claude Code essaie d'analyser votre stdout comme JSON et ne peut pas, il rapporte une erreur non-bloquante sur chaque code de sortie autre que 2. La transcription affiche un avis `<hook name> hook error` avec le message d'analyse. Sur les événements qui ajoutent stdout en texte brut comme contexte, Claude Code n'ajoute pas le texte. Avant v2.1.248, Claude Code traitait ce stdout comme du texte brut.

Stderr d'un hook qui quitte 0 va uniquement au journal de débogage, jamais à la transcription, et Claude ne le voit jamais. Pour le lire vous-même, activez [la journalisation de débogage](#debug-hooks). Pour afficher un avertissement à Claude à partir d'un hook `PostToolUse` ou `PostToolUseFailure`, quittez 2 à la place afin que [Claude voie stderr](#exit-code-2-behavior-per-event) même si l'outil a déjà s'exécuté.

<h4 id="exit-code-2">
  Exit code 2
</h4>

Exit 2 signifie une erreur bloquante. Sur [les événements qui peuvent bloquer](#exit-code-2-behavior-per-event), exit 2 bloque que vous imprimiez JSON ou non : même une `permissionDecision` JSON de `"allow"` ne peut pas la remplacer. Claude Code lit toujours tout [sortie JSON](#json-output) valide sur stdout. Sur `Elicitation` et `ElicitationResult`, le `hookSpecificOutput` d'un hook exit-2 est ignoré.

Le message de blocage est la raison de la décision de blocage de votre JSON lorsqu'elle en fait une, et votre texte stderr sinon. Ce que le blocage fait varie selon l'événement : `PreToolUse` bloque l'appel d'outil, `UserPromptSubmit` rejette le prompt, et ainsi de suite. [Comportement du code de sortie 2 par événement](#exit-code-2-behavior-per-event) énumère l'effet pour chaque événement, et chaque section d'événement dit où le message va.

Un hook qui quitte 2 tout en imprimant JSON qui échoue la validation du schéma [sortie JSON](#json-output) bloque toujours : Claude Code utilise stderr comme raison de blocage et enregistre l'échec de validation dans le journal de débogage. Avant v2.1.214, Claude Code traitait cette combinaison comme une erreur non-bloquante et l'action procédait.

Ce script bloque les commandes `rm` en quittant 2 et laisse chaque autre commande au flux de permission normal :

```bash theme={null}
#!/bin/bash
# Lit l'entrée JSON depuis stdin, vérifie la commande
input=$(cat)
command=$(jq -r '.tool_input.command' <<<"$input")

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Erreur bloquante : l'appel d'outil est empêché
fi

exit 0  # Pas de décision : le flux de permission normal s'applique
```

<h4 id="other-exit-codes">
  Autres codes de sortie
</h4>

Tout autre code de sortie ne bloque pas seul pour la plupart des événements de hook. Ce qui se passe dépend de votre stdout :

* Avec un objet analysé qui passe la validation du schéma, pour les événements qui utilisent le modèle de décision standard, Claude Code ignore le code de sortie et le JSON seul décide du résultat :
  * Chaque champ que l'événement supporte est honoré, y compris `permissionDecision`, `additionalContext`, `updatedInput` et `systemMessage`, et le hook n'est pas signalé comme une erreur.
  * [Contrôle de décision](#decision-control) énumère les champs de décision par événement ; les champs universels comme `systemMessage` suivent le tableau [Sortie JSON](#json-output).
* Avec un objet analysé qui échoue la validation du schéma, pour les événements qui utilisent le modèle de décision standard, c'est la même erreur non-bloquante que [sur exit 0](#exit-code-0) : l'action procède, et l'avis `<hook name> hook error` porte le message de validation.
* Avec stdout que Claude Code [essaie d'analyser comme JSON](#exit-code-0) et ne peut pas, Claude Code rapporte la même erreur non-bloquante que sur exit 0 pour les événements qui utilisent le modèle de décision standard. L'action procède, et l'avis porte le message d'analyse.
* Avec stdout que Claude Code [traite comme du texte brut](#exit-code-0), ou avec stdout vide, c'est une erreur non-bloquante pour la plupart des événements de hook : l'action procède, et la transcription affiche un avis `<hook name> hook error` suivi de la première ligne de stderr, préfixée par `Failed with non-blocking status code:`. Pour capturer le stderr complet, activez [la journalisation de débogage](#debug-hooks).

Les événements en dehors du modèle de décision standard gardent leurs propres lignes dans le [tableau par événement](#exit-code-2-behavior-per-event) : `WorktreeCreate` échoue la création sur tout code de sortie non-zéro peu importe ce que votre JSON dit, et les événements qui rejettent complètement la sortie du hook, comme `StopFailure`, ignorent votre JSON sur chaque code de sortie, à part les champs d'effet secondaire comme `terminalSequence`, qui se déclenchent toujours.

Un hook qui ne peut pas démarrer atterrit dans le même bucket non-bloquant. Lorsque le chemin du script n'existe pas ou n'est pas exécutable, le shell quitte avec un code comme 127 et vous voyez le même avis avec le message de l'interpréteur, par exemple `Failed with non-blocking status code: /bin/sh: /path/to/hook.sh: No such file or directory`. Pour la plupart des événements de hook, l'action procède. Lorsque vous configurez un hook de politique, regardez cet avis à sa première exécution : un chemin mal orthographié dans `settings.json` laisse la porte silencieusement désactivée.

<Warning>
  Pour la plupart des événements de hook, exit code 2 est le seul code de sortie qui bloque par le code seul. Sans JSON valide sur stdout, Claude Code traite exit code 1 comme une erreur non-bloquante et procède avec l'action, même si 1 est le code d'échec Unix conventionnel. Si votre hook est destiné à appliquer une politique, utilisez `exit 2`. Les événements worktree diffèrent : tout code de sortie non-zéro de `WorktreeCreate` abandonne la création du worktree, et tout code de sortie non-zéro de `WorktreeRemove` rend la suppression du worktree échouée si le répertoire existe toujours après.
</Warning>

<h4 id="timeouts">
  Délais d'expiration
</h4>

À l'exception d'un hook de commande que vous exécutez avec [`async: true`](#run-hooks-in-the-background), Claude Code annule un hook `command`, `http` ou `mcp_tool` qui atteint son [`timeout`](#common-fields), en rejetant la sortie du hook, donc sur la plupart des événements un hook expiré ne rend aucune décision.

Sur [`PreModelSwitch`](#premodelswitch), un hook annulé à son délai d'expiration bloque le changement de modèle. Sur `PreToolUse`, les deux familles de hooks diffèrent :

* Un hook `command`, `http` ou `mcp_tool` expiré ne bloque pas l'appel d'outil. L'appel continue via le [flux de permission](/docs/fr/permissions) normal, donc ne comptez pas sur un hook bloqué pour agir comme une porte.
* Un hook de rappel [Agent SDK](/docs/fr/agent-sdk/hooks) qui dépasse son délai d'expiration [bloque l'appel d'outil](#pretooluse).

<h4 id="exit-code-2-behavior-per-event">
  Comportement du code de sortie 2 par événement
</h4>

Exit code 2 est la façon dont un hook signale « arrêtez, ne faites pas cela ». L'effet dépend de l'événement, car certains événements représentent des actions qui peuvent être bloquées (comme un appel d'outil qui ne s'est pas encore produit) et d'autres représentent des choses qui se sont déjà produites ou ne peuvent pas être empêchées.

| Événement de hook     | Peut bloquer ? | Ce qui se passe sur exit 2                                                                                                                                                                                                                                     |
| :-------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Oui            | Bloque l'appel d'outil                                                                                                                                                                                                                                         |
| `PermissionRequest`   | Non            | Exit code 2 n'est pas honoré pour cet événement et le flux de permission procède inchangé. Refusez via l'objet [`decision`](#permissionrequest-decision-control) à la place                                                                                    |
| `UserPromptSubmit`    | Oui            | Bloque le traitement du prompt et efface le prompt                                                                                                                                                                                                             |
| `UserPromptExpansion` | Oui            | Bloque l'expansion                                                                                                                                                                                                                                             |
| `Stop`                | Oui            | Empêche Claude de s'arrêter, continue la conversation                                                                                                                                                                                                          |
| `SubagentStop`        | Oui            | Empêche le subagent de s'arrêter                                                                                                                                                                                                                               |
| `TeammateIdle`        | Oui            | Empêche le coéquipier de devenir inactif, le coéquipier continue de travailler                                                                                                                                                                                 |
| `TaskCreated`         | Oui            | Annule la création de la tâche                                                                                                                                                                                                                                 |
| `TaskCompleted`       | Oui            | Empêche la tâche d'être marquée comme complétée                                                                                                                                                                                                                |
| `ConfigChange`        | Oui            | Bloque la modification de configuration de prendre effet (sauf `policy_settings`)                                                                                                                                                                              |
| `StopFailure`         | Non            | La sortie et le code de sortie sont ignorés, sauf `terminalSequence`                                                                                                                                                                                           |
| `PostToolUse`         | Non            | Affiche stderr à Claude ; l'outil a déjà s'exécuté                                                                                                                                                                                                             |
| `PostToolUseFailure`  | Non            | Affiche stderr à Claude ; l'outil a déjà échoué                                                                                                                                                                                                                |
| `PostToolBatch`       | Oui            | Arrête la boucle agentique avant l'appel du modèle suivant                                                                                                                                                                                                     |
| `PermissionDenied`    | Non            | Exit code et stderr sont ignorés car le refus a déjà eu lieu. Utilisez JSON `hookSpecificOutput.retry: true` pour indiquer au modèle qu'il peut réessayer ; Claude Code ignore `retry: true` pour les [refus sans verdict](#permissiondenied-decision-control) |
| `Notification`        | Non            | Exit code et stderr sont ignorés                                                                                                                                                                                                                               |
| `SubagentStart`       | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                                                                                                                      |
| `SessionStart`        | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                                                                                                                      |
| `Setup`               | Non            | Exit code et stderr sont ignorés                                                                                                                                                                                                                               |
| `SessionEnd`          | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                                                                                                                      |
| `CwdChanged`          | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                                                                                                                      |
| `DirectoryAdded`      | Non            | Stderr va au journal de débogage ; le répertoire est déjà ajouté                                                                                                                                                                                               |
| `FileChanged`         | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                                                                                                                      |
| `PreCompact`          | Oui            | Bloque la compaction                                                                                                                                                                                                                                           |
| `PostCompact`         | Non            | Affiche stderr à l'utilisateur uniquement                                                                                                                                                                                                                      |
| `PreModelSwitch`      | Oui            | Bloque le changement de modèle et affiche stderr à l'utilisateur                                                                                                                                                                                               |
| `PostModelSwitch`     | Non            | Affiche stderr à l'utilisateur uniquement ; le modèle a déjà changé                                                                                                                                                                                            |
| `Elicitation`         | Oui            | Refuse l'élicitation                                                                                                                                                                                                                                           |
| `ElicitationResult`   | Oui            | Bloque la réponse (l'action devient decline)                                                                                                                                                                                                                   |
| `WorktreeCreate`      | Oui            | Tout code de sortie non-zéro provoque l'échec de la création du worktree                                                                                                                                                                                       |
| `WorktreeRemove`      | Oui            | Tout code de sortie non-zéro rend la suppression du worktree échouée si le répertoire existe toujours après. Consultez [WorktreeRemove](#worktreeremove) pour ce qui arrive au répertoire                                                                      |
| `InstructionsLoaded`  | Non            | Exit code est ignoré                                                                                                                                                                                                                                           |
| `MessageDisplay`      | Non            | Le texte original est affiché                                                                                                                                                                                                                                  |

Pour `SessionStart`, `SubagentStart` et `PostModelSwitch`, Claude Code rend le stderr du code de sortie 2 dans la transcription comme un avis `<hook name> hook error`, de la même manière qu'il rend une [erreur non-bloquante](#exit-code-output). Claude ne le voit pas, et la session ou le subagent procède. Pour `SubagentStart`, l'avis apparaît dans la propre transcription du subagent, pas dans la conversation parent.

<h3 id="http-response-handling">
  Gestion des réponses HTTP
</h3>

Les hooks HTTP utilisent les codes de statut HTTP et les corps de réponse au lieu des codes de sortie et stdout. Les résultats ci-dessous s'appliquent à la plupart des événements ; un événement avec son propre contrat d'échec dans le [tableau par événement](#exit-code-2-behavior-per-event), tel que `WorktreeCreate`, applique ce contrat à un hook HTTP échoué aussi :

* **2xx avec un corps vide** : succès, équivalent à exit code 0 sans sortie
* **2xx avec un corps d'objet JSON** : analysé en utilisant le même schéma [sortie JSON](#json-output) que les hooks de commande. Un corps qui échoue la validation du schéma est une erreur non-bloquante
* **2xx avec n'importe quel autre corps, comme du texte brut** : erreur non-bloquante, gérée de la même manière qu'un statut non-2xx. Claude Code n'ajoute pas le texte au contexte de Claude
* **Statut non-2xx** : erreur non-bloquante, l'exécution continue
* **Défaillance de connexion** : erreur non-bloquante, l'exécution continue
* **Délai d'expiration** : le hook est annulé, comme décrit sous [Délais d'expiration](#timeouts)

Contrairement aux hooks de commande, les hooks HTTP ne peuvent pas signaler une erreur bloquante uniquement via les codes de statut. Pour bloquer un appel d'outil ou refuser une permission, retournez une réponse 2xx avec un corps JSON contenant les champs de décision appropriés.

<h3 id="json-output">
  Sortie JSON
</h3>

Les codes de sortie vous permettent uniquement de bloquer ou de rester silencieux, mais la sortie JSON vous donne un contrôle plus granulaire. Au lieu de quitter avec le code 2 pour bloquer, quittez 0 et imprimez un objet JSON sur stdout. Claude Code lit les champs spécifiques de ce JSON pour contrôler le comportement, y compris [contrôle de décision](#decision-control) pour bloquer, autoriser ou escalader à l'utilisateur.

<Note>
  Choisissez une approche par hook : soit utiliser les codes de sortie seuls pour signaler, soit quitter 0 et imprimer JSON pour un contrôle structuré. Si vous les mélangez, exit 2 garde son [effet de blocage](#exit-code-2-behavior-per-event), et Claude Code lit toujours les champs JSON, avec l'exception d'élicitation unique notée sous [Exit code 2](#exit-code-2).
</Note>

La sortie stdout de votre hook doit contenir uniquement l'objet JSON. Si votre profil shell imprime du texte au démarrage, cela peut interférer avec l'analyse JSON. Consultez [Hook JSON has no effect](/docs/fr/hooks-guide#hook-json-has-no-effect) dans le guide de dépannage.

Les chaînes de sortie du hook, y compris `additionalContext`, `systemMessage` et `initialUserMessage`, et son stdout brut, sont plafonnées à 10 000 caractères :

* **Portée** : Claude Code mesure chaque chaîne seule, même lorsque plusieurs hooks s'exécutent pour le même événement. Pour la sortie JSON, chaque champ est mesuré séparément ; stdout brut est mesuré dans son ensemble.
* **Au-delà de la limite** : Claude Code enregistre la sortie dans un fichier du répertoire de session et la remplace par le chemin du fichier et un aperçu de jusqu'à 2 000 premiers caractères. Un grand résultat Bash valide est géré de la même manière, décrit sous [Output limits](/docs/fr/tools-reference#output-limits). Contrairement à ce plafond Bash, ce cap n'a pas de paramètre ou de variable d'environnement pour l'augmenter.
* **Lecture du fichier** : Claude Code ne demande pas à Claude de lire le fichier, donc gardez tout ce que Claude doit toujours voir dans le cap.

L'objet JSON supporte trois types de champs :

* **Champs universels** comme `continue` sont listés dans le tableau ci-dessous. Chaque événement les accepte, mais certains événements les rejettent ou livrent `systemMessage` ailleurs que dans la transcription. Chaque section d'événement le précise. `terminalSequence` fonctionne sur ces événements aussi, avec les exceptions listées sous [Émettre des notifications de terminal](#emit-terminal-notifications).
* **`decision` et `reason` au niveau supérieur** sont utilisés par certains événements pour bloquer ou fournir des commentaires.
* **`hookSpecificOutput`** est un objet imbriqué pour les événements qui ont besoin d'un contrôle plus riche. Il nécessite un champ `hookEventName` défini au nom de l'événement.

| Champ              | Par défaut | Description                                                                                                                                                                                                                                                                                                                                                                              |
| :----------------- | :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`     | Si `false`, Claude arrête complètement le traitement après l'exécution du hook. Prend précédence sur tous les champs de décision spécifiques à l'événement                                                                                                                                                                                                                               |
| `stopReason`       | aucun      | Message affiché à l'utilisateur lorsque `continue` est `false`. Il reste dans la conversation, afin que Claude le voie si la conversation continue                                                                                                                                                                                                                                       |
| `suppressOutput`   | `false`    | N'a aucun effet : Claude Code accepte le champ mais n'agit pas dessus. La sortie stdout d'un hook réussi n'est jamais affichée dans la transcription et est enregistrée dans le journal de débogage                                                                                                                                                                                      |
| `systemMessage`    | aucun      | Message d'avertissement affiché à l'utilisateur. Dans [Agent SDK](/docs/fr/agent-sdk/overview) et [`--output-format stream-json`](/docs/fr/headless) sortie, il peut arriver comme un [`SDKInformationalMessage`](/docs/fr/agent-sdk/typescript#sdkinformationalmessage)                                                                                                                                |
| `terminalSequence` | aucun      | Une séquence d'échappement de terminal pour Claude Code d'émettre en votre nom, comme une notification de bureau, un titre de fenêtre ou une cloche. Restreint aux OSC `0`/`1`/`2`/`9`/`99`/`777` et BEL. Si la valeur contient quelque chose en dehors de la liste blanche, le champ est ignoré. Utilisez ceci au lieu d'écrire sur `/dev/tty`, qui n'est pas disponible pour les hooks |

Pour arrêter Claude entièrement :

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

Pour les hooks `PreToolUse` et `PostToolUse`, l'arrêt s'applique même lorsque l'appel d'outil échoue ou se termine tandis que Claude diffuse toujours une réponse.

<h4 id="emit-terminal-notifications">
  Émettre des notifications de terminal
</h4>

Les hooks s'exécutent sans terminal de contrôle, donc écrire des séquences d'échappement directement sur `/dev/tty` échoue. À la place, retournez la séquence d'échappement dans le champ `terminalSequence` et Claude Code l'émet pour vous via son propre chemin d'écriture de terminal. C'est sans course, fonctionne à l'intérieur de tmux et GNU screen, et fonctionne sur Windows où il n'y a pas de `/dev/tty`.

Le champ accepte une chaîne d'une ou plusieurs séquences d'échappement en liste blanche :

* OSC `0`, `1`, `2` : titres de fenêtre et d'icône
* OSC `9` : notifications iTerm2, ConEmu, Windows Terminal et WezTerm, y compris la progression de la barre des tâches `9;4`
* OSC `99` : notifications Kitty
* OSC `777` : notifications urxvt, Ghostty et Warp
* BEL nu

Les séquences peuvent être terminées avec BEL ou avec ST. Tout ce qui est en dehors de la liste blanche, y compris les séquences de curseur CSI et les séquences de couleur, les séquences de palette OSC, les hyperliens OSC 8, les écritures de presse-papiers OSC 52 et OSC 1337, est rejeté et le champ est ignoré.

Claude Code écrit la séquence elle-même lorsqu'il traite la sortie de votre hook, donc le champ fonctionne sur les événements qui rejettent `systemMessage` et `continue`, tels que `Notification` et `StopFailure`. Il a deux limites :

* Claude Code écrit la séquence uniquement dans une session interactive, et uniquement tandis que son interface est à l'écran. En mode non-interactif avec le drapeau `-p` et dans l'Agent SDK, il ignore le champ.
* Un hook de commande `WorktreeCreate` ne peut pas retourner JSON, car Claude Code lit son stdout comme le chemin du worktree. Un hook HTTP `WorktreeCreate` retourne JSON et peut inclure le champ.

L'exemple ci-dessous déclenche une notification de bureau à partir d'un hook `Notification`. La séquence d'échappement est construite avec des échappements octaux `printf` afin que les octets de contrôle n'apparaissent jamais sur la ligne de commande shell, et `jq -n --arg` construit la sortie JSON afin que les guillemets, les barres obliques inverses et les sauts de ligne dans le message de notification soient correctement échappés :

```bash theme={null}
#!/bin/bash
# Hook de notification : ping le bureau lorsque Claude Code a besoin d'attention.
input=$(cat)
title="Claude Code"
body=$(jq -r '.message // "Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

La forme `{ "terminalSequence": "..." }` est la même à partir de n'importe quel shell ou langage.

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

* [SessionStart](#sessionstart) et [SubagentStart](#subagentstart) : au début de la conversation, avant le premier prompt
* [UserPromptSubmit](#userpromptsubmit) et [UserPromptExpansion](#userpromptexpansion) : aux côtés du prompt soumis
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) et [PostToolBatch](#posttoolbatch) : à côté du résultat de l'outil
* [Stop](#stop) et [SubagentStop](#subagentstop) : à la fin du tour. La conversation continue afin que Claude puisse agir sur les commentaires. Consultez [Contrôle de décision Stop](#stop-decision-control)
* [PostModelSwitch](#postmodelswitch) : avec la prochaine demande après le changement. Consultez [Contrôle de décision PostModelSwitch](#postmodelswitch-decision-control) pour le timing

Lorsque plusieurs hooks retournent `additionalContext` pour le même événement, Claude reçoit toutes les valeurs.

Si une valeur dépasse 10 000 caractères, Claude Code écrit le texte dans un fichier du répertoire de session et transmet à Claude le chemin du fichier avec un aperçu de jusqu'à 2 000 premiers caractères à la place. Claude peut lire le fichier, mais Claude Code ne le demande pas.

Utilisez `additionalContext` pour les informations que Claude devrait connaître sur l'état actuel de votre environnement ou l'opération qui vient de s'exécuter :

* **État de l'environnement** : la branche actuelle, la cible de déploiement ou les drapeaux de fonctionnalité actifs
* **Règles de projet conditionnelles** : quelle commande de test s'applique au fichier qui vient d'être modifié, quels répertoires sont en lecture seule dans ce worktree
* **Données externes** : problèmes ouverts qui vous sont assignés, résultats CI récents, contenu récupéré à partir d'un service interne

Pour les instructions qui ne changent jamais, préférez [CLAUDE.md](/docs/fr/memory). Il se charge sans exécuter de script et est l'endroit standard pour les conventions de projet statiques.

Écrivez le texte sous forme de déclarations factuelles plutôt que d'instructions système impératives. Des formulations telles que « La cible de déploiement est production » ou « Ce repo utilise `bun test` » se lisent comme des informations de projet. Le texte encadré comme des commandes système hors bande peut déclencher les défenses contre l'injection de prompt de Claude, ce qui amène Claude à vous présenter le texte au lieu de le traiter comme du contexte.

Claude Code enregistre le texte injecté dans la transcription de session. Pour les événements mid-session comme `PostToolUse` ou `UserPromptSubmit`, lorsque vous reprenez avec `--continue` ou `--resume`, Claude Code rejoue le texte enregistré plutôt que de réexécuter le hook pour les tours passés, de sorte que les valeurs comme les horodatages ou les SHA de commit deviennent obsolètes. Les hooks `SessionStart` s'exécutent à nouveau à la reprise avec `source` défini sur `"resume"`, ou `"fork"` si vous avez ajouté `--fork-session`, afin qu'ils puissent actualiser leur contexte.

<h4 id="decision-control">
  Contrôle de décision
</h4>

Tous les événements ne supportent pas le blocage ou le contrôle du comportement via JSON. Les événements qui le font utilisent chacun un ensemble différent de champs pour exprimer cette décision. Utilisez ce tableau comme référence rapide avant d'écrire un hook :

| Événements                                                                                                                          | Modèle de décision                                     | Champs clés                                                                                                                                                                                                                                                                                                      |
| :---------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | `decision` au niveau supérieur                         | `decision: "block"`, `reason`. Stop et SubagentStop acceptent également `hookSpecificOutput.additionalContext` pour [les commentaires non-erreur qui continuent la conversation](#stop-decision-control)                                                                                                         |
| TeammateIdle, TaskCompleted                                                                                                         | Exit code ou `continue: false`                         | Exit code 2 bloque l'action avec commentaires stderr. JSON `{"continue": false, "stopReason": "..."}` arrête également complètement le coéquipier, correspondant au comportement du hook `Stop` ; [TaskCompleted l'ignore lorsque l'outil `TaskUpdate` a déclenché l'événement](#taskcompleted-decision-control) |
| TaskCreated                                                                                                                         | Exit code ou `decision` au niveau supérieur            | Exit code 2 ou `decision: "block"` [annule la tâche](#taskcreated-decision-control) et retourne le message à Claude. `continue: false` est ignoré                                                                                                                                                                |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                                   | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                                                                                          |
| PreModelSwitch                                                                                                                      | `hookSpecificOutput` ou `decision` au niveau supérieur | `permissionDecision` (allow/deny/ask), `permissionDecisionReason`. `decision: "block"` [annule également le changement](#premodelswitch-decision-control)                                                                                                                                                        |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                                   | `decision.behavior` (allow/deny)                                                                                                                                                                                                                                                                                 |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                                   | `retry: true` indique au modèle qu'il peut réessayer l'appel d'outil refusé ; Claude Code l'ignore pour les [refus sans verdict](#permissiondenied-decision-control)                                                                                                                                             |
| WorktreeCreate                                                                                                                      | retour de chemin                                       | Le hook de commande imprime le chemin sur stdout ; le hook HTTP retourne `hookSpecificOutput.worktreePath`. L'échec du hook ou l'absence de chemin échoue la création                                                                                                                                            |
| WorktreeRemove                                                                                                                      | Exit code                                              | Tout code de sortie non-zéro rend la suppression échouée si le répertoire existe toujours après. La sortie JSON est rejetée                                                                                                                                                                                      |
| Elicitation                                                                                                                         | `hookSpecificOutput`                                   | `action` (accept/decline/cancel), `content` (valeurs des champs de formulaire pour accept)                                                                                                                                                                                                                       |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                                   | `action` (accept/decline/cancel), `content` (valeurs des champs de formulaire override)                                                                                                                                                                                                                          |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                                   | `displayContent` remplace le texte affiché à l'écran. Affichage uniquement : la transcription et ce que Claude voit conservent l'original                                                                                                                                                                        |
| SessionStart, SubagentStart, PostModelSwitch                                                                                        | Contexte uniquement                                    | `hookSpecificOutput.additionalContext` ajoute du contexte pour Claude. SessionStart accepte également [`initialUserMessage`, `watchPaths`, `sessionTitle` et `reloadSkills`](#sessionstart-decision-control). Pas de blocage ou de contrôle de décision                                                          |
| Setup, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, DirectoryAdded, FileChanged              | Aucun                                                  | Pas de contrôle de décision. Utilisé pour les effets secondaires comme la journalisation ou le nettoyage                                                                                                                                                                                                         |

Quelques événements peuvent également réécrire le contenu plutôt que seulement l'autoriser ou le bloquer :

* `PreToolUse` : `updatedInput` directement sous `hookSpecificOutput` remplace les arguments d'un outil avant son exécution. Consultez [Contrôle de décision PreToolUse](#pretooluse-decision-control)
* `PermissionRequest` : `updatedInput` à l'intérieur de l'objet `decision`. Consultez [Contrôle de décision PermissionRequest](#permissionrequest-decision-control)
* `PostToolUse` : `updatedToolOutput` remplace le résultat de l'outil. Consultez [Contrôle de décision PostToolUse](#posttooluse-decision-control)
* `UserPromptSubmit` : ne peut pas remplacer le prompt ; injecte uniquement `additionalContext` à côté de celui-ci

Pour les cas d'usage de rédaction ou de transformation, interceptez à `PreToolUse` pour les entrées d'outil sortantes et `PostToolUse` pour les résultats d'outil entrants.

Voici des exemples de chaque modèle en action :

<Tabs>
  <Tab title="Décision au niveau supérieur">
    La seule valeur pour `decision` est `"block"`. Pour autoriser l'action à procéder, omettez `decision` de votre JSON, ou quittez 0 sans aucun JSON :

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

Chaque événement correspond à un point du cycle de vie de Claude Code où les hooks peuvent s'exécuter. Les sections ci-dessous sont ordonnées pour correspondre au cycle de vie : de la configuration de la session à la boucle agentive jusqu'à la fin de la session. Chaque section décrit quand l'événement se déclenche, quels matchers il supporte, l'entrée JSON qu'il reçoit, et comment contrôler le comportement via la sortie.

<h3 id="sessionstart">
  SessionStart
</h3>

S'exécute quand Claude Code démarre une nouvelle session ou reprend une session existante. Utile pour charger le contexte de développement comme les problèmes existants ou les modifications récentes de votre base de code, ou pour configurer des variables d'environnement. Pour un contexte statique qui ne nécessite pas de script, utilisez plutôt [CLAUDE.md](/docs/fr/memory).

SessionStart s'exécute à chaque session, donc gardez ces hooks rapides. Seuls les hooks `type: "command"` et `type: "mcp_tool"` sont supportés. Voir [Champs de hook MCP tool](#mcp-tool-hook-fields) pour savoir quand les hooks `mcp_tool` s'exécutent.

La valeur du matcher correspond à la façon dont la session a été initiée :

| Matcher   | Quand il se déclenche                                                                                                                                  |
| :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `startup` | Nouvelle session                                                                                                                                       |
| `resume`  | `--resume`, `--continue`, ou `/resume`                                                                                                                 |
| `clear`   | `/clear`                                                                                                                                               |
| `compact` | Compaction automatique ou manuelle                                                                                                                     |
| `fork`    | Une nouvelle session créée à partir d'une session existante : `--fork-session` avec `--resume` ou `--continue`, la copie de fond `/fork`, ou `/branch` |

Avant v2.1.214, les sessions créées par fork signalaient la source `"resume"`.

Quand vous démarrez une session interactive, reprenez une conversation au lancement avec `--continue` ou `--resume`, ou exécutez `/clear`, les hooks SessionStart s'exécutent en arrière-plan. Vous pouvez taper immédiatement, et une conversation que vous avez reprise apparaît sans attendre les hooks. La première réponse de Claude attend toujours que les hooks se terminent, donc leur contexte atteint Claude.

Quand vous changez de conversation avec `/resume` à l'intérieur d'une session, le changement attend que les hooks se terminent. Si vous exécutez `/clear` ou changez vers une autre conversation pendant que les hooks en arrière-plan s'exécutent toujours, rien de ce qu'ils retournent ne s'applique à la session.

La même attente s'applique au lancement, y compris une session reprise : une invite que vous envoyez pendant que les hooks SessionStart s'exécutent toujours n'atteint Claude que quand ils se terminent.

Pendant l'une ou l'autre attente, appuyez sur `Esc` pour reprendre l'invite dans l'entrée sans l'envoyer. Les hooks continuent de s'exécuter.

<h4 id="sessionstart-input">
  Entrée SessionStart
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks SessionStart reçoivent `source` et optionnellement `model`, `agent_type`, et `session_title` :

| Champ           | Description                                                                                                                                                                                                                                         |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Comment la session a démarré : `"startup"` pour les nouvelles sessions, `"resume"` pour les sessions reprises, `"clear"` après `/clear`, `"compact"` après compaction, ou `"fork"` pour une nouvelle session créée à partir d'une session existante |
| `model`         | L'identifiant du modèle actif. Il peut être omis, par exemple après `/clear` ou quand une session est restaurée via la récupération de conversation, donc vérifiez le champ avant de le lire                                                        |
| `agent_type`    | Le nom de l'agent, présent quand vous démarrez Claude Code avec `claude --agent <name>`                                                                                                                                                             |
| `session_title` | Le titre de la session actuelle s'il est déjà défini, par exemple via `--name` ou `/rename`. Un hook qui émet `sessionTitle` peut vérifier `session_title` d'abord pour éviter de remplacer un titre que l'utilisateur a défini explicitement       |

Quand `source` est `"resume"` ou `"fork"` et que la transcription contient au moins une réponse de Claude, les hooks SessionStart reçoivent également les quatre champs ci-dessous. Votre hook peut les utiliser pour signaler le coût de reprendre une conversation obsolète avant la première requête, par exemple dans un [`systemMessage`](#json-output). Ces champs nécessitent Claude Code v2.1.251 ou ultérieur.

| Champ                         | Description                                                                                                                                                                                                     |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `seconds_since_last_response` | Secondes d'horloge murale depuis la dernière réponse dans la transcription reprise                                                                                                                              |
| `context_tokens`              | Tokens que la première requête de la session reprise renvoie comme son invite                                                                                                                                   |
| `prompt_cache_likely_expired` | `true` quand la dernière réponse est plus ancienne que la [durée de vie du cache d'invite](/docs/fr/prompt-caching#cache-lifetime) de la session ou qu'une compaction ultérieure a remplacé la conversation en cache |
| `estimated_cache_write_usd`   | Coût estimé en dollars US de l'écriture de `context_tokens` dans le cache d'invite sur le modèle de la session, excluant la réponse                                                                             |

Cet exemple montre l'entrée pour une session reprise 90 minutes après sa dernière réponse :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "resume",
  "model": "claude-opus-5",
  "seconds_since_last_response": 5400,
  "context_tokens": 182340,
  "prompt_cache_likely_expired": true,
  "estimated_cache_write_usd": 1.1396
}
```

<h4 id="sessionstart-decision-control">
  Contrôle de décision SessionStart
</h4>

Claude Code ajoute la sortie standard qu'il [traite comme du texte brut](#exit-code-0) au contexte de Claude. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, vous pouvez retourner ces champs spécifiques à l'événement :

| Champ                | Description                                                                                                                                                                                                                                                                                                                                                        |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Chaîne ajoutée au contexte de Claude au début de la conversation, avant la première invite. Voir [Ajouter du contexte pour Claude](#add-context-for-claude) pour savoir comment le texte est livré et ce qu'il faut y mettre                                                                                                                                       |
| `initialUserMessage` | Chaîne utilisée comme premier message utilisateur de la session. S'applique en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`, où elle devient le premier tour même si aucune invite n'est fournie. Si une invite est fournie, elle suit comme le tour suivant. Contrairement à `additionalContext`, qui s'attache à un tour existant, ceci crée le tour |
| `sessionTitle`       | Définit le titre de la session, avec le même effet que `/rename`. Utilisez pour nommer les sessions automatiquement à partir du dossier de lancement, de la branche git, ou du nom du worktree. S'applique quand `source` est `"startup"`, `"resume"`, ou `"fork"` ; ignoré sur `"clear"` et `"compact"`                                                           |
| `watchPaths`         | Tableau de chemins absolus à surveiller pour les événements [FileChanged](#filechanged) pendant cette session                                                                                                                                                                                                                                                      |
| `reloadSkills`       | Booléen. Quand `true`, Claude Code réanalyse les répertoires [skill](/docs/fr/skills) et command après que les hooks SessionStart se terminent, donc les skills que le hook a installés sont disponibles dans la même session, à partir de la première invite                                                                                                           |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Puisque la sortie standard brute atteint déjà Claude pour cet événement, un hook qui charge uniquement du contexte peut imprimer sur la sortie standard directement sans construire JSON. Utilisez la forme JSON quand vous devez combiner du contexte avec d'autres champs comme `sessionTitle`.

Utilisez `reloadSkills` quand un hook SessionStart installe ou met à jour des skills. La découverte de skills s'exécute normalement avant que les hooks SessionStart se terminent, donc les fichiers que le hook écrit dans `~/.claude/skills/` ou `.claude/skills/` n'apparaîtraient autrement que dans la session suivante. Cet exemple synchronise un référentiel de skills partagé et demande la réanalyse :

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

L'URL du référentiel est un espace réservé ; remplacez-la par votre propre référentiel de skills. Avec l'espace réservé, le clone échoue et imprime un message `fatal:` sur stderr. Stderr d'un hook SessionStart qui quitte 0 est informatif uniquement, donc la demande `reloadSkills` s'applique toujours.

<h4 id="persist-environment-variables">
  Persister les variables d'environnement
</h4>

Les hooks SessionStart ont accès à la variable d'environnement `CLAUDE_ENV_FILE`, qui fournit un chemin de fichier où vous pouvez persister les variables d'environnement pour les commandes Bash suivantes.

Pour définir des variables d'environnement individuelles, écrivez des instructions `export` dans `CLAUDE_ENV_FILE`. Utilisez l'ajout (`>>`) pour préserver les variables définies par d'autres hooks :

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

# Run your setup commands that modify the environment
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

<Note>
  `CLAUDE_ENV_FILE` est disponible pour les hooks SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged), et [FileChanged](#filechanged). Les autres types de hooks n'ont pas accès à cette variable.
</Note>

<h3 id="setup">
  Setup
</h3>

S'exécute uniquement quand vous lancez Claude Code avec `--init-only`, ou avec `--init` ou `--maintenance` en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`. Il ne s'exécute pas au démarrage normal. Utilisez-le pour l'installation de dépendances ponctuelles ou le nettoyage programmé que vous déclenchez explicitement à partir de CI ou de scripts, séparé du démarrage normal de la session. Pour l'initialisation par session, utilisez plutôt [SessionStart](#sessionstart).

La valeur du matcher correspond au drapeau CLI qui a déclenché le hook :

| Matcher       | Quand il se déclenche                      |
| :------------ | :----------------------------------------- |
| `init`        | `claude --init-only` ou `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                  |

Quand vous exécutez `claude --init-only`, Claude Code exécute les hooks Setup et les hooks `SessionStart` avec le matcher `startup`, puis quitte sans démarrer une conversation.

Quand vous démarrez ou continuez une conversation avec `-p`, vous devez également fournir une invite, comme argument ou piped sur stdin. Vous pouvez ignorer l'invite quand un hook `SessionStart` fournit [`initialUserMessage`](#sessionstart-decision-control) ou quand vous reprenez une session avec un [appel d'outil différé](#defer-a-tool-call-for-later).

En cas de succès, `--init-only` n'imprime rien sur le terminal. Pour confirmer que les hooks se sont exécutés, commencez par `claude --debug-file <path> --init-only`, en remplaçant `<path>` par un emplacement de fichier journal, et vérifiez le journal pour les entrées de hook Setup et SessionStart.

Parce que Setup ne s'exécute pas à chaque lancement, un plugin qui a besoin d'une dépendance installée ne peut pas compter sur Setup seul. Le modèle pratique est de vérifier la dépendance à la première utilisation et d'installer en cas d'absence, par exemple un hook ou skill qui teste `${CLAUDE_PLUGIN_DATA}/node_modules` et exécute `npm install` s'il est absent. Voir le [répertoire de données persistantes](/docs/fr/plugins/components#path-variables-and-persistent-data) pour savoir où stocker les dépendances installées. Si vous distribuez votre plugin via une marketplace, vous n'aurez peut-être pas besoin de ce modèle : Claude Code [installe automatiquement les dépendances de package Node.js éligibles](/docs/fr/plugins/loading#node-js-package-dependencies) quand il met en cache le plugin.

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

Les hooks Setup ne peuvent pas bloquer ; l'exécution continue sur n'importe quel code de sortie. Sur chaque code de sortie, Claude Code rejette les [champs de sortie JSON](#json-output) d'un hook Setup, comme `systemMessage`, `continue`, et `hookSpecificOutput.additionalContext`. Avec `-p`, la sortie standard, la sortie d'erreur, et le code de sortie d'un hook Setup n'apparaissent dans la sortie de la session que comme des événements [`hook_response`](/docs/fr/headless#read-session-metadata) quand vous lancez avec `--output-format stream-json --verbose`.

Les hooks Setup ont accès à `CLAUDE_ENV_FILE`. Les variables écrites dans ce fichier persistent dans les commandes Bash suivantes pour la session, tout comme dans les [hooks SessionStart](#persist-environment-variables). Seuls les hooks `type: "command"` s'exécutent sur `Setup`. Un hook `type: "mcp_tool"` sur `Setup` est toujours ignoré, comme décrit sous [Champs de hook MCP tool](#mcp-tool-hook-fields).

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

S'exécute quand un fichier `CLAUDE.md` ou `.claude/rules/*.md` est chargé dans le contexte. Cet événement se déclenche au démarrage de la session pour les fichiers chargés avec impatience et à nouveau plus tard quand les fichiers sont chargés avec paresse, par exemple quand Claude accède à un sous-répertoire qui contient un `CLAUDE.md` imbriqué ou quand les règles conditionnelles avec le frontmatter `paths:` correspondent. Le hook ne supporte pas le blocage ou le contrôle de décision. Il s'exécute de manière asynchrone à des fins d'observabilité.

Cet événement ne se déclenche pas quand Claude [lit `AGENTS.md` directement](/docs/fr/memory#agents-md) via le paramètre **Project instructions**. Il se déclenche quand un `CLAUDE.md` importe votre `AGENTS.md`, avec `load_reason` défini à `include` comme pour tout autre fichier importé, et quand `CLAUDE.md` est un lien symbolique vers lui, comme un chargement normal de `CLAUDE.md`.

Le matcher s'exécute contre `load_reason`. Par exemple, utilisez `"matcher": "session_start"` pour se déclencher uniquement pour les fichiers chargés au démarrage de la session, ou `"matcher": "path_glob_match|nested_traversal"` pour se déclencher uniquement pour les chargements avec paresse.

<h4 id="instructionsloaded-input">
  Entrée InstructionsLoaded
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks InstructionsLoaded reçoivent ces champs :

| Champ               | Description                                                                                                                                                                                                                                        |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Chemin absolu du fichier d'instructions qui a été chargé                                                                                                                                                                                           |
| `memory_type`       | Portée du fichier : `"User"`, `"Project"`, `"Local"`, ou `"Managed"`                                                                                                                                                                               |
| `load_reason`       | Pourquoi le fichier a été chargé : `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"`, ou `"compact"`. La valeur `"compact"` se déclenche quand les fichiers d'instructions sont rechargés après un événement de compaction |
| `globs`             | Modèles de glob de chemin du frontmatter `paths:` du fichier, le cas échéant. Présent uniquement pour les chargements `path_glob_match`                                                                                                            |
| `trigger_file_path` | Chemin du fichier dont l'accès a déclenché ce chargement, pour les chargements avec paresse                                                                                                                                                        |
| `parent_file_path`  | Chemin du fichier d'instructions parent qui a inclus celui-ci, pour les chargements `include`                                                                                                                                                      |

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

Les hooks InstructionsLoaded n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer ou modifier le chargement des instructions. Claude Code rejette leurs [champs de sortie JSON](#json-output), comme `systemMessage` et `continue`. Utilisez cet événement pour l'audit logging, le suivi de conformité, ou l'observabilité.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

S'exécute quand l'utilisateur soumet une invite, avant que Claude la traite. Cela vous permet d'ajouter du contexte supplémentaire basé sur l'invite/conversation, de valider les invites, ou de bloquer certains types d'invites.

Les hooks `UserPromptSubmit` ont un délai d'expiration par défaut de 30 secondes pour les types `command`, `http`, et `mcp_tool`, plus court que le défaut de 600 secondes pour ces types sur la plupart des autres événements. Parce que ce hook s'exécute avant chaque invite et bloque le traitement du modèle jusqu'à ce qu'il se termine, un hook bloqué paralyse la session. Si votre hook a besoin de plus de temps, définissez le champ `timeout` dans l'entrée du hook.

À part un hook de commande que vous exécutez avec [`async: true`](#run-hooks-in-the-background), un hook de commande, HTTP, ou MCP tool `UserPromptSubmit` qui atteint son délai d'expiration est annulé et sa sortie, y compris tout `additionalContext`, est rejetée. L'invite atteint toujours Claude sans ce contexte. La transcription affiche un avis nommant le hook, le délai d'expiration qui a déclenché, et que la sortie a été rejetée.

Un [hook de rappel Agent SDK](/docs/fr/agent-sdk/hooks) sur `UserPromptSubmit` qui atteint son délai d'expiration bloque l'invite avec un message nommant le hook et le délai d'expiration, parce qu'un rappel là peut agir comme une porte de politique qui ne doit pas échouer ouvertement. La session continue. Avant v2.1.208, un délai d'expiration de rappel sur cet événement terminait le tour avec une erreur d'exécution.

<h4 id="userpromptsubmit-input">
  Entrée UserPromptSubmit
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks UserPromptSubmit reçoivent le champ `prompt` contenant le texte que l'utilisateur a soumis. Le contenu collé qui s'est effondré à un espace réservé `[Pasted text #N]` arrive développé en place. Dans les sessions où Claude Code [marque le texte collé pour Claude](/docs/fr/terminal-config#how-claude-treats-pasted-text), ce contenu développé se situe entre une ligne `<pasted_content id="…">` et une ligne `</pasted_content id="…">`, donc tenez compte de ces lignes si votre hook analyse l'invite.

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

Les hooks `UserPromptSubmit` peuvent contrôler si une invite utilisateur est traitée et ajouter du contexte. Tous les [champs de sortie JSON](#json-output) sont disponibles.

Il y a deux façons d'ajouter du contexte à la conversation sur le code de sortie 0 :

* **Sortie standard en texte brut** : Claude Code ajoute la sortie standard qu'il [traite comme du texte brut](#exit-code-0) au contexte de Claude
* **JSON avec `additionalContext`** : utilisez le format JSON ci-dessous pour plus de contrôle. Le champ `additionalContext` est ajouté comme contexte

Aucun canal ne produit une entrée de transcription visible. La sortie standard brute et la valeur `additionalContext` sont chacune injectées comme un rappel système qui commence par le nom du hook ; Claude lit les deux. Pour confirmer la livraison, vérifiez le [journal de débogage](#debug-hooks).

Pour bloquer une invite, retournez un objet JSON avec `decision` défini à `"block"` :

| Champ                    | Description                                                                                                                         |
| :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `decision`               | `"block"` empêche l'invite d'être traitée et l'efface du contexte. Omettez pour permettre à l'invite de procéder                    |
| `reason`                 | Montré à l'utilisateur quand `decision` est `"block"`. Non ajouté au contexte                                                       |
| `additionalContext`      | Chaîne ajoutée au contexte de Claude aux côtés de l'invite soumise. Voir [Ajouter du contexte pour Claude](#add-context-for-claude) |
| `sessionTitle`           | Définit le titre de la session. Utilisez pour nommer les sessions automatiquement en fonction du contenu de l'invite                |
| `suppressOriginalPrompt` | Si `true` quand `decision` est `"block"`, omet le texte d'invite original du message de blocage montré à l'utilisateur              |

Un hook qui bloque en quittant 2 s'achemine de la même façon que `reason` : le message de blocage montre le texte stderr à l'utilisateur, et il n'est pas ajouté au contexte.

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

S'exécute quand une commande tapée par l'utilisateur se développe en une invite avant d'atteindre Claude. Utilisez ceci pour bloquer des commandes spécifiques de l'invocation directe, injecter du contexte pour une skill particulière, ou enregistrer quelles commandes les utilisateurs invoquent. Par exemple, un hook correspondant à `deploy` peut bloquer `/deploy` sauf si un fichier d'approbation est présent, ou un hook correspondant à une skill de révision peut ajouter la liste de contrôle de révision de l'équipe comme `additionalContext`.

Cet événement couvre le chemin que `PreToolUse` ne couvre pas : un hook `PreToolUse` correspondant à l'outil `Skill` se déclenche uniquement quand Claude appelle l'outil, mais taper `/skillname` directement contourne `PreToolUse`. `UserPromptExpansion` se déclenche sur ce chemin direct.

Correspond à `command_name`. Laissez le matcher vide pour se déclencher sur chaque commande de type invite.

<h4 id="userpromptexpansion-input">
  Entrée UserPromptExpansion
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks UserPromptExpansion reçoivent `expansion_type`, `command_name`, `command_args`, `command_source`, et la chaîne `prompt` originale. Le champ `expansion_type` est `slash_command` pour les skills et commandes personnalisées, ou `mcp_prompt` pour les invites du serveur MCP.

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

| Champ               | Description                                                                                                                            |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` empêche la commande de se développer. Omettez pour permettre à la commande de procéder                                       |
| `reason`            | Montré à l'utilisateur quand `decision` est `"block"`                                                                                  |
| `additionalContext` | Chaîne ajoutée au contexte de Claude aux côtés de l'invite développée. Voir [Ajouter du contexte pour Claude](#add-context-for-claude) |

Un hook qui bloque en quittant 2 s'achemine de la même façon que `reason` : le message de blocage montre le texte stderr à l'utilisateur.

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

S'exécute pendant qu'un message d'assistant s'affiche à l'écran. Claude Code affiche le message par incréments : chaque fois qu'un lot de lignes nouvellement complétées est prêt à être rendu, le hook s'exécute une fois avec ces lignes et Claude Code rend le texte de remplacement du hook à leur place. Un long message produit plusieurs appels ; un court message peut ne produire qu'un seul.

Utilisez MessageDisplay pour :

* supprimer le markdown pour un affichage minimal
* transformer le texte qu'une application Agent SDK montre à ses utilisateurs
* masquer les clés API ou les noms d'hôtes internes des réponses de Claude

Claude Code attend chaque lot jusqu'à ce que votre hook retourne, donc gardez le hook rapide. Si le hook échoue ou expire, Claude Code affiche le texte original. Le délai d'expiration par défaut pour cet événement est 10 secondes ; si votre hook a besoin de plus de temps, définissez le champ `timeout` dans l'entrée du hook.

MessageDisplay est affichage uniquement : le texte de remplacement change uniquement ce qui est rendu à l'écran. La transcription et ce que Claude voit conservent le texte original, donc Claude ne voit jamais le remplacement, et le mode verbeux affiche l'original. Le hook reçoit uniquement le texte du message d'assistant, donc les résultats d'outils et le texte que vous tapez s'affichent inchangés.

MessageDisplay ne supporte pas les matchers et se déclenche pour chaque message d'assistant qui affiche du texte ; les messages sans texte, comme les réponses d'appel d'outil uniquement, ne le déclenchent pas.

Dans les exécutions non-interactives, y compris les requêtes Agent SDK et `claude -p`, MessageDisplay s'exécute une fois par message d'assistant au lieu d'une fois par lot de lignes. L'appel unique arrive après que le message se termine et porte le texte du message complet : `index` est `0`, `final` est `true`, et `delta` contient le message entier. Un hook qui collecte le texte `delta` pour chaque message reçoit le même texte total dans les deux modes.

<h4 id="messagedisplay-input">
  Entrée MessageDisplay
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks MessageDisplay reçoivent des identifiants pour le tour et le message, la position de cet appel dans le message, et le nouveau texte dans `delta`. Les limites de lot dépendent de la façon dont le texte s'affiche, donc utilisez `index` et `final` pour suivre la progression à travers un message plutôt que de vous attendre à ce que les lignes soient groupées d'une manière particulière.

| Champ        | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID du tour actuel                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `message_id` | UUID du message d'assistant en cours d'affichage. Stable à travers chaque lot du même message. Ce n'est pas l'API `msg_…` id, donc il ne peut pas être corrélé avec les ids de message de transcription                                                                                                                                                                                                                                                                              |
| `index`      | Index de base zéro de ce lot dans le message                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `final`      | `true` sur le dernier lot du message. Chaque message a exactement un lot final                                                                                                                                                                                                                                                                                                                                                                                                       |
| `delta`      | Les lignes nouvellement complétées depuis le lot précédent, y compris les sauts de ligne de fin. Toujours des lignes entières, sauf le lot final qui peut se terminer au milieu d'une ligne. Dans les exécutions interactives, le delta du lot final est vide quand le message se termine sur un saut de ligne, donc traitez `final`, pas un delta non-vide, comme le signal de fin de message. Dans les exécutions Agent SDK et `claude -p`, l'appel unique porte le message entier |

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

Les hooks MessageDisplay n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer le message ou changer ce qui est stocké dans la transcription ou envoyé à Claude. Claude Code agit sur `displayContent` de leur sortie JSON et rejette `systemMessage` et `continue`.

Cet exemple supprime la mise en forme markdown des réponses de Claude pour un affichage en texte brut. Le script lit chaque lot depuis stdin, supprime les marqueurs gras et les backticks de code en ligne de `delta`, et retourne le résultat comme `displayContent`.

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

    Le drapeau `-NoProfile` ignore le chargement de votre profil PowerShell pour que le hook démarre rapidement, et `-ExecutionPolicy Bypass` permet à PowerShell d'exécuter le fichier de script local.

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

S'exécute après que Claude crée les paramètres d'outil et avant de traiter l'appel d'outil. Correspond à n'importe quel nom d'outil sauf `EndConversation` : les outils intégrés comme `Bash`, `PowerShell`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `Workflow`, `WebFetch`, `WebSearch`, `AskUserQuestion`, et `ExitPlanMode`, et n'importe quels [noms d'outils MCP](#match-mcp-tools).

Pour exécuter un hook quand un fichier spécifique change sur le disque, peu importe ce qui l'a écrit, utilisez [FileChanged](#filechanged) au lieu de correspondre aux outils d'édition de fichiers par nom. Contrairement à PreToolUse, Claude Code exécute les hooks FileChanged après le changement, et ils n'ont pas de contrôle de décision, donc ils ne peuvent pas bloquer l'écriture.

<Warning>
  PreToolUse s'exécute uniquement quand Claude appelle un outil. Les fichiers que vous [référencez avec `@` dans votre invite](/docs/fr/common-workflows#reference-files-and-directories) sont ajoutés sans aucun appel d'outil : Claude Code insère leur contenu lors de la construction de l'invite, donc aucun hook PreToolUse ne se déclenche pour eux, y compris les hooks correspondant à `Read`. Pour bloquer des chemins spécifiques des références `@`, utilisez plutôt une [règle de refus `Read`](/docs/fr/permissions#read-and-edit).

  PreToolUse ne se déclenche pas non plus pour [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior).
</Warning>

Utilisez [Contrôle de décision PreToolUse](#pretooluse-decision-control) pour permettre, refuser, demander, ou différer l'appel d'outil.

Un [hook de rappel Agent SDK](/docs/fr/agent-sdk/hooks) sur `PreToolUse` qui dépasse son délai d'expiration bloque l'appel d'outil, et Claude reçoit un résultat d'erreur nommant le délai d'expiration. Un refus explicite retourné par un autre hook prend toujours la priorité.

<h4 id="pretooluse-input">
  Entrée PreToolUse
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PreToolUse reçoivent `tool_name`, `tool_input`, et `tool_use_id`.

Pour un [outil MCP](#match-mcp-tools), l'entrée porte également `mcp_server`, un objet avec le `name` du serveur et une `source` qui dit d'où vient la définition du serveur. Les valeurs `source` incluent `plugin`, `sdk`, et les portées de configuration comme `user` et `project`. [`McpServerProvenance`](/docs/fr/agent-sdk/typescript#mcpserverprovenance) dans la référence Agent SDK les énumère tous et dit comment traiter celui que vous ne reconnaissez pas. Basez les décisions de confiance sur `source` plutôt que sur `name` ou le préfixe de nom d'outil `mcp__<server>__`. Le champ `mcp_server` nécessite Claude Code v2.1.274 ou ultérieur.

Pour les outils de fichier `Write`, `Edit`, et `Read`, `tool_input.file_path` est toujours absolu :

* Claude Code développe `~` et les chemins relatifs avant que les hooks s'exécutent, donc un hook qui correspond à des chemins ne peut pas être contourné via `~` ou une orthographe relative du même chemin
* Sur Windows, le chemin arrive avec des séparateurs de barre oblique inverse, même quand votre hook s'exécute sous Git Bash où `$PWD` ressemble à `/c/project`
* Une comparaison écrite avec des barres obliques avant, comme une vérification `/src/`, ne correspond jamais à un chemin de barre oblique inverse, et l'appel d'outil procède comme si le hook n'avait rien à bloquer
* Normalisez les séparateurs avant de comparer : `FILE_PATH="${FILE_PATH//\\//}"` en Bash, ou `file_path.replace("\\", "/")` en Python, puis correspondez à un segment de chemin comme `/src/` plutôt que d'ancrer avec `^`, puisque le chemin est absolu

Un appel `Write` sur Windows livre :

```json theme={null}
{
  "hook_event_name": "PreToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "C:\\project\\src\\index.ts",
    "content": "..."
  },
  ...
}
```

Les champs `tool_input` dépendent de l'outil :

<a id="bash" />

<h5 id="bash">
  Bash
</h5>

Exécute les commandes shell.

| Champ               | Type    | Exemple            | Description                                                                                                                                                            |
| :------------------ | :------ | :----------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | La commande shell à exécuter                                                                                                                                           |
| `description`       | string  | `"Run test suite"` | Description optionnelle de ce que la commande fait                                                                                                                     |
| `timeout`           | number  | `120000`           | Délai d'expiration optionnel en millisecondes. Les valeurs au-dessus du [maximum](/docs/fr/tools-reference#bash-tool-behavior) sont réduites au maximum plutôt que rejetées |
| `run_in_background` | boolean | `false`            | Si la commande doit s'exécuter en arrière-plan                                                                                                                         |

Quand une commande Bash change des fichiers dans un référentiel Git, Claude Code peut enregistrer ce qui a changé. Il enregistre les changements dans chaque mode de permission quand le paramètre [`bashEditDiffEnabled`](/docs/fr/settings-reference#basheditdiffenabled) active l'enregistrement ; l'entrée de ce paramètre dit quels fichiers peuvent le définir. Sinon, il les enregistre uniquement en mode auto et en mode `bypassPermissions`, et uniquement quand Claude Code dirige Claude à éditer des fichiers via Bash. Définissez `bashEditDiffEnabled` à `false` pour désactiver l'enregistrement. Les commandes en arrière-plan et les commandes en lecture seule ne portent pas de diff.

Votre [hook PostToolUse](#posttooluse) reçoit alors les fichiers modifiés dans `tool_response.bashEditDiff`. La liste couvre ce qui a changé sous le référentiel pendant que la commande s'exécutait. Les fichiers que Git ignore et les fichiers dans les sous-modules ne sont pas listés. Nécessite Claude Code v2.1.269 ou ultérieur.

<Note>
  La liste est au mieux un effort et en bêta publique. Claude Code peut manquer un changement, inclure un fichier qu'un autre processus a changé au même moment, ou s'arrêter à ses limites de taille. La forme du champ peut changer. Utilisez la liste pour trouver ce à examiner, pas pour appliquer une politique.
</Note>

`changedFiles` et `files` listent ce que la commande a changé ; les champs restants disent à quel point cette liste est complète et fiable.

| Champ          | Type    | Exemple                                                 | Description                                                                                                                                                                                       |
| :------------- | :------ | :------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `changedFiles` | array   | `["/path/to/src/app.ts"]`                               | Chemins absolus des fichiers que la commande a changés, au maximum 200. Présent chaque fois que `files` contient un diff ou `moreFiles` est au-dessus de zéro                                     |
| `files`        | array   | `[{"filePath": "/path/to/src/app.ts", "hunks": [...]}]` | Diffs de jusqu'à 5 fichiers modifiés, pour l'affichage. `created` ou `deleted` est `true` pour un fichier que la commande a ajouté ou supprimé                                                    |
| `moreFiles`    | number  | `2`                                                     | Nombre de fichiers modifiés sans diff dans `files`                                                                                                                                                |
| `unavailable`  | boolean | `true`                                                  | Défini quand le diff est incomplet ou n'a pas pu être pris                                                                                                                                        |
| `skipped`      | boolean | `true`                                                  | Défini pour une commande Git qui déplace l'arborescence de travail, comme `git checkout` ou `git stash`, donc Claude Code ne prend pas de diff                                                    |
| `shared`       | boolean | `true`                                                  | Défini quand un autre appel d'outil Bash, comme celui d'un sous-agent, s'exécutait dans le même référentiel au même moment, donc certains changements listés peuvent être celui de cette commande |

<a id="powershell" />

<h5 id="powershell">
  PowerShell
</h5>

Exécute les commandes PowerShell. Voir l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool) pour la disponibilité par plateforme.

Les champs correspondent à l'outil Bash, avec la chaîne de commande dans `command` :

| Champ               | Type    | Exemple                    | Description                                        |
| :------------------ | :------ | :------------------------- | :------------------------------------------------- |
| `command`           | string  | `"Get-ChildItem -Recurse"` | La commande PowerShell à exécuter                  |
| `description`       | string  | `"List files recursively"` | Description optionnelle de ce que la commande fait |
| `timeout`           | number  | `120000`                   | Délai d'expiration optionnel en millisecondes      |
| `run_in_background` | boolean | `false`                    | Si la commande doit s'exécuter en arrière-plan     |

Correspondez à `Bash|PowerShell` dans les hooks qui inspectent les commandes shell, pour qu'ils couvrent les deux outils :

* Sur Windows, partout où l'outil PowerShell est activé, Claude traite PowerShell comme le shell principal et achemine les commandes shell à travers lui.
* Sur Windows sans Git Bash, l'outil est activé automatiquement et Claude Code n'enregistre pas l'outil Bash du tout.
* Un hook qui correspond uniquement à `Bash` ne se déclenche jamais là.

<h5 id="write">
  Write
</h5>

Crée ou remplace un fichier.

| Champ       | Type   | Exemple               | Description                       |
| :---------- | :----- | :-------------------- | :-------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Chemin absolu du fichier à écrire |
| `content`   | string | `"file content"`      | Contenu à écrire dans le fichier  |

<h5 id="edit">
  Edit
</h5>

Remplace une chaîne dans un fichier existant.

| Champ         | Type    | Exemple               | Description                                     |
| :------------ | :------ | :-------------------- | :---------------------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Chemin absolu du fichier à éditer               |
| `old_string`  | string  | `"original text"`     | Texte à trouver et remplacer                    |
| `new_string`  | string  | `"replacement text"`  | Texte de remplacement                           |
| `replace_all` | boolean | `false`               | Si tous les occurrences doivent être remplacées |

<h5 id="read">
  Read
</h5>

Lit le contenu des fichiers.

| Champ       | Type   | Exemple               | Description                                         |
| :---------- | :----- | :-------------------- | :-------------------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Chemin absolu du fichier à lire                     |
| `offset`    | number | `10`                  | Numéro de ligne optionnel pour commencer la lecture |
| `limit`     | number | `50`                  | Nombre optionnel de lignes à lire                   |

<h5 id="glob">
  Glob
</h5>

Trouve les fichiers correspondant à un modèle glob.

| Champ     | Type   | Exemple          | Description                                                                   |
| :-------- | :----- | :--------------- | :---------------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Modèle glob pour correspondre aux fichiers                                    |
| `path`    | string | `"/path/to/dir"` | Répertoire optionnel à rechercher. Par défaut le répertoire de travail actuel |

<h5 id="grep">
  Grep
</h5>

Recherche le contenu des fichiers avec des expressions régulières.

| Champ         | Type    | Exemple          | Description                                                                          |
| :------------ | :------ | :--------------- | :----------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Modèle d'expression régulière à rechercher                                           |
| `path`        | string  | `"/path/to/dir"` | Fichier ou répertoire optionnel à rechercher                                         |
| `glob`        | string  | `"*.ts"`         | Modèle glob optionnel pour filtrer les fichiers                                      |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"`, ou `"count"`. Par défaut `"files_with_matches"` |
| `-i`          | boolean | `true`           | Recherche insensible à la casse                                                      |
| `multiline`   | boolean | `false`          | Activer la correspondance multiligne                                                 |

<h5 id="webfetch">
  WebFetch
</h5>

Récupère et traite le contenu web.

| Champ    | Type   | Exemple                       | Description                               |
| :------- | :----- | :---------------------------- | :---------------------------------------- |
| `url`    | string | `"https://example.com/api"`   | URL pour récupérer le contenu             |
| `prompt` | string | `"Extract the API endpoints"` | Invite à exécuter sur le contenu récupéré |

<h5 id="websearch">
  WebSearch
</h5>

Recherche le web.

| Champ             | Type   | Exemple                        | Description                                                  |
| :---------------- | :----- | :----------------------------- | :----------------------------------------------------------- |
| `query`           | string | `"react hooks best practices"` | Requête de recherche                                         |
| `allowed_domains` | array  | `["docs.example.com"]`         | Optionnel : inclure uniquement les résultats de ces domaines |
| `blocked_domains` | array  | `["spam.example.com"]`         | Optionnel : exclure les résultats de ces domaines            |

<h5 id="agent">
  Agent
</h5>

Crée un [sous-agent](/docs/fr/sub-agents).

| Champ           | Type   | Exemple                    | Description                                        |
| :-------------- | :----- | :------------------------- | :------------------------------------------------- |
| `prompt`        | string | `"Find all API endpoints"` | La tâche pour l'agent à effectuer                  |
| `description`   | string | `"Find API endpoints"`     | Description courte de la tâche                     |
| `subagent_type` | string | `"Explore"`                | Type d'agent spécialisé à utiliser                 |
| `model`         | string | `"sonnet"`                 | Alias de modèle optionnel pour remplacer le défaut |

Quand un appel Agent au premier plan se termine, votre [hook PostToolUse](#posttooluse) reçoit le résultat du sous-agent et la télémétrie d'exécution dans `tool_response`. Lisez ces champs pour inspecter l'exécution ; pour les totalisations de tokens et de coûts entre les sous-agents, utilisez les [compteurs de tokens et de coûts](/docs/fr/monitoring-usage#token-counter) filtrés à `query_source` `"subagent"`, puisque `totalTokens` et `usage` couvrent uniquement la requête finale :

| Champ               | Type   | Exemple                                               | Description                                                                                                                                                                                                                                                      |
| :------------------ | :----- | :---------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` pour les sous-agents au premier plan, `"async_launched"` pour les sous-agents en arrière-plan. À partir de v2.1.198, les sous-agents s'exécutent en arrière-plan par défaut, donc un `run_in_background` omis produit également `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Identifiant pour l'exécution du sous-agent                                                                                                                                                                                                                       |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Les blocs de texte final du sous-agent, ou, pour un sous-agent dont le rapport passe par `SubagentHandback`, une brève note à ce sujet à leur place                                                                                                              |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Modèle sur lequel le sous-agent a démarré, qui peut différer du modèle demandé                                                                                                                                                                                   |
| `modelsUsed`        | array  | `["claude-sonnet-4-5", "claude-haiku-4-5"]`           | Modèles utilisés dans l'ordre, avec les répétitions consécutives effondrées ; défini uniquement quand le modèle a été échangé en cours d'exécution. Nécessite Claude Code v2.1.212 ou ultérieur                                                                  |
| `totalTokens`       | number | `12450`                                               | Nombre de tokens de la requête API finale du sous-agent : tokens d'entrée, de sortie, et de cache combinés. Ce n'est pas un total sur toute l'exécution                                                                                                          |
| `totalDurationMs`   | number | `48211`                                               | Durée d'horloge murale de l'exécution du sous-agent                                                                                                                                                                                                              |
| `totalToolUseCount` | number | `7`                                                   | Nombre d'appels d'outil que le sous-agent a effectués                                                                                                                                                                                                            |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Ventilation des tokens par type de la requête API finale : `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                             |

Sur Claude Code v2.1.271 ou ultérieur, un sous-agent qui s'exécute avec l'outil [`SubagentHandback`](/docs/fr/tools-reference), que Claude Code fournit en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), livre son rapport via cet outil plutôt que de le retourner comme texte. Le champ `content` de son résultat `completed` porte alors une brève note à ce sujet plutôt que le rapport lui-même. Pour lire le rapport, correspondez à un hook `PreToolUse` ou `PostToolUse` sur `SubagentHandback` et lisez `tool_input.message`.

Pour les sous-agents en arrière-plan, l'outil retourne quand la tâche passe en arrière-plan, donc `tool_response` ne porte pas de champs d'utilisation : un lancement en arrière-plan retourne immédiatement, et une tâche au premier plan que Claude Code met en arrière-plan en cours d'exécution retourne à cette transition. Il a `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile`, et `resolvedModel`.

Sur une réponse `completed`, `resolvedModel` nomme le modèle sur lequel le sous-agent a démarré, qui peut différer de la valeur `model` dans `tool_input`, comme quand `availableModels` ou un autre remplacement s'applique. Sur une réponse `async_launched`, `resolvedModel` nomme le modèle en utilisation quand l'agent est passé en arrière-plan, donc un échange qui s'est produit avant la mise en arrière-plan est reflété là. Le comportement `modelsUsed` et `resolvedModel` au moment de la mise en arrière-plan nécessitent Claude Code v2.1.212 ou ultérieur.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Pose à l'utilisateur une à quatre questions à choix multiples.

| Champ       | Type   | Exemple                                                                                                            | Description                                                                                                                                                                                                                                                |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Questions à présenter, chacune avec une chaîne `question`, un court `header`, un tableau `options`, et un drapeau `multiSelect` optionnel                                                                                                                  |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Optionnel. Mappe le texte de la question à l'étiquette de l'option sélectionnée. Les réponses multi-sélection joignent les étiquettes avec des virgules. Claude ne définit pas ce champ ; fournissez-le via `updatedInput` pour répondre par programmation |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Présente un plan et demande à l'utilisateur de l'approuver avant que Claude quitte le [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode). Claude écrit le plan dans un fichier sur le disque avant d'appeler l'outil, donc le `tool_input` littéral du modèle est généralement vide. Claude Code injecte le contenu du plan et le chemin du fichier avant de passer l'entrée aux hooks.

| Champ            | Type   | Exemple                                     | Description                                                                                                                                                             |
| :--------------- | :----- | :------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Contenu du plan en Markdown. Injecté à partir du fichier de plan sur le disque                                                                                          |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Chemin du fichier de plan. Injecté                                                                                                                                      |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Déprécié. Claude Code accepte le champ mais l'ignore. Avant v2.1.205, il portait les permissions basées sur les invites que Claude a demandées pour implémenter le plan |

Dans `PostToolUse`, `tool_response` est un objet avec les champs `plan` et `filePath` contenant le plan approuvé, plus les drapeaux d'état internes. Lisez `tool_response.plan` pour le contenu du plan plutôt que de relire le fichier depuis le disque.

<h4 id="pretooluse-decision-control">
  Contrôle de décision PreToolUse
</h4>

Les hooks `PreToolUse` peuvent contrôler si un appel d'outil procède. Contrairement aux autres hooks qui utilisent un champ `decision` de haut niveau, PreToolUse retourne sa décision à l'intérieur d'un objet `hookSpecificOutput`. Cela lui donne un contrôle plus riche : quatre résultats (permettre, refuser, demander, ou différer) plus la capacité de modifier l'entrée d'outil avant l'exécution.

| Champ                      | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` ignore l'invite de permission, sauf pour les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves) et pour `AskUserQuestion` et `ExitPlanMode`, qui ont besoin de [`updatedInput` associé](#allow-with-updatedinput). `"deny"` empêche l'appel d'outil. `"ask"` invite l'utilisateur à confirmer. `"defer"` quitte proprement pour que l'outil puisse être repris plus tard. Les [règles de refus et de demande](/docs/fr/permissions#manage-permissions) sont toujours évaluées indépendamment de ce que le hook retourne |
| `permissionDecisionReason` | Pour `"allow"` et `"ask"`, montré à l'utilisateur mais pas à Claude. Pour `"deny"`, montré à Claude. Pour `"defer"`, ignoré                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `updatedInput`             | Modifie les paramètres d'entrée de l'outil avant l'exécution. Remplace l'objet d'entrée entier, donc incluez les champs inchangés aux côtés des champs modifiés. Claude Code évalue les règles de permission et l'éligibilité de [mise en arrière-plan automatique](/docs/fr/tools-reference#background-commands) d'une commande Bash contre l'entrée que votre hook retourne, pas l'entrée que Claude a envoyée. Combinez avec `"allow"` pour approuver automatiquement, ou `"ask"` pour montrer l'entrée modifiée à l'utilisateur. Pour `"defer"`, ignoré                           |
| `additionalContext`        | Chaîne ajoutée au contexte de Claude aux côtés du résultat d'outil. Ignoré quand `permissionDecision` est `"defer"`. Voir [Ajouter du contexte pour Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                                             |

Quand plusieurs hooks PreToolUse retournent des décisions différentes, la priorité est `deny` > `defer` > `ask` > `allow`.

Un hook qui bloque en quittant 2 s'achemine de la même façon que `"deny"` : Claude voit le message stderr comme la raison du refus.

Quand un hook retourne `"ask"`, l'invite de permission affichée à l'utilisateur inclut une étiquette identifiant d'où vient le hook : `[settings]` pour un hook de n'importe quel fichier de paramètres ou du frontmatter d'agent, `[plugin:<name>]` pour le hook d'un plugin, ou `[skill]` pour un hook du frontmatter de skill. Cela aide les utilisateurs à comprendre quelle source de configuration demande une confirmation.

Un `"ask"` d'un hook force également une invite de permission en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) : le classificateur peut toujours refuser l'appel d'outil, mais il ne peut pas approuver l'appel silencieusement. Avant v2.1.211, le classificateur pouvait approuver une commande Bash s'exécutant en dehors du [sandbox](/docs/fr/sandboxing) sans montrer l'invite que le hook a demandée ; le classificateur appliquait toujours ses propres règles de sécurité à cette commande, et un refus de hook était toujours honoré.

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

<span id="allow-with-updatedinput" />

En [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`, Claude Code offre `AskUserQuestion` et `ExitPlanMode` uniquement quand l'exécution a un [hôte de permission](/docs/fr/headless#turn-off-permission-prompts-in-unattended-runs) pour recevoir l'invite, comme un rappel `canUseTool` d'Agent SDK. Ces outils nécessitent l'interaction de l'utilisateur. Retourner `permissionDecision: "allow"` avec `updatedInput` satisfait cette exigence : le hook lit l'entrée de l'outil depuis stdin, collecte la réponse via votre propre interface utilisateur, et la retourne dans `updatedInput` pour que l'outil s'exécute sans inviter. Retourner `"allow"` seul n'est pas suffisant pour ces outils. Pour `AskUserQuestion`, renvoyez le tableau `questions` original et ajoutez un objet [`answers`](#askuserquestion) mappant le texte de chaque question à la réponse choisie.

À partir de v2.1.199, un outil MCP dont le serveur le marque avec [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool) est plus strict : un hook ne peut pas ignorer son invite d'approbation avec `"allow"`, avec ou sans `updatedInput`, parce que Claude Code ne peut pas confirmer que le hook a collecté l'interaction que l'outil a besoin.

<Note>
  PreToolUse utilisait auparavant les champs `decision` et `reason` de haut niveau, mais ceux-ci sont dépréciés pour cet événement. Utilisez plutôt `hookSpecificOutput.permissionDecision` et `hookSpecificOutput.permissionDecisionReason`. Les valeurs dépréciées `"approve"` et `"block"` correspondent à `"allow"` et `"deny"` respectivement. D'autres événements comme PostToolUse et Stop continuent d'utiliser `decision` et `reason` de haut niveau comme leur format actuel.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Différer un appel d'outil pour plus tard
</h4>

`"defer"` est pour les intégrations qui exécutent `claude -p` comme un sous-processus et lisent sa sortie JSON, comme une application Agent SDK ou une interface utilisateur personnalisée construite au-dessus de Claude Code. Cela permet à ce processus appelant de mettre en pause Claude à un appel d'outil, de collecter l'entrée via sa propre interface, et de reprendre où il s'était arrêté. Claude Code honore cette valeur uniquement en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`. Dans les sessions interactives, il enregistre un avertissement et ignore le résultat du hook.

L'outil `AskUserQuestion` est le cas typique : Claude veut poser une question à l'utilisateur, mais il n'y a pas de terminal pour répondre. Une exécution `-p` offre `AskUserQuestion` uniquement quand elle a un [hôte de permission](/docs/fr/headless#turn-off-permission-prompts-in-unattended-runs), comme un outil MCP que vous passez avec `--permission-prompt-tool`, donc commencez l'exécution avec un. Le aller-retour fonctionne comme ceci :

1. Claude appelle `AskUserQuestion`. Le hook `PreToolUse` se déclenche.
2. Le hook retourne `permissionDecision: "defer"`. L'outil ne s'exécute pas. Le processus quitte avec `stop_reason: "tool_deferred"` et l'appel d'outil en attente préservé dans la transcription.
3. Le processus appelant lit `deferred_tool_use` du résultat SDK, affiche la question dans sa propre interface utilisateur, et attend une réponse.
4. Le processus appelant exécute `claude -p --resume <session-id>` avec le même hôte de permission. Le même appel d'outil déclenche `PreToolUse` à nouveau.
5. Le hook retourne `permissionDecision: "allow"` avec la réponse dans `updatedInput`. L'outil s'exécute et Claude continue.

Le champ `deferred_tool_use` porte l'`id`, le `name`, et l'`input` de l'outil. L'`input` est les paramètres que Claude a générés pour l'appel d'outil, capturés avant l'exécution :

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

Il n'y a pas de délai d'expiration ou de limite de tentatives. La session reste sur le disque jusqu'à ce que vous la repreniez, soumise au [balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically) `cleanupPeriodDays`, qui supprime les fichiers de session après 30 jours par défaut, en suivant les [règles du balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically). Si la réponse n'est pas prête quand vous reprenez, le hook peut retourner `"defer"` à nouveau et le processus quitte de la même façon. Le processus appelant contrôle quand casser la boucle en retournant finalement `"allow"` ou `"deny"` du hook.

`"defer"` fonctionne uniquement quand Claude effectue un seul appel d'outil dans le tour. Si Claude effectue plusieurs appels d'outil à la fois, `"defer"` est ignoré avec un avertissement et l'outil procède via le flux de permission normal. La contrainte existe parce que la reprise ne peut réexécuter qu'un seul outil : il n'y a aucun moyen de différer un appel d'un lot sans laisser les autres non résolus.

Si l'outil différé n'est plus disponible quand vous reprenez, le processus quitte avec `stop_reason: "tool_deferred_unavailable"` et `is_error: true` avant que le hook se déclenche. Cela se produit quand un serveur MCP qui a fourni l'outil n'est pas connecté pour la session reprise. La charge utile `deferred_tool_use` est toujours incluse pour que vous puissiez identifier quel outil a disparu.

<Note>
  Pour reprendre une session différée en mode plan, passez [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags) avec `--resume` pour que Claude Code puisse présenter le plan pour approbation. Sans cela, Claude Code ne restaure pas le mode plan. Nécessite Claude Code v2.1.246 ou ultérieur.

  Quand vous reprenez avec `-p`, Claude Code ne restaure aucun autre mode de permission stocké. Il démarre l'exécution dans le mode de permission qu'une nouvelle exécution `claude -p` démarrerait, donc passez `--permission-mode` ou `--dangerously-skip-permissions` à nouveau si la session différée en utilisait un. Quand vous reprenez avec `claude --resume <session-id>` sans `-p`, Claude Code restaure le mode de permission stocké, avec les exceptions listées dans [mode de permission à la reprise](/docs/fr/sessions#permission-mode-on-resume).
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

S'exécute quand Claude Code est sur le point de vous demander la permission d'utiliser un outil. Dans les sessions qui ne peuvent pas montrer une invite, comme les sous-agents en arrière-plan en [mode non-interactif](/docs/fr/headless), Claude Code exécute toujours ces hooks, et si aucun hook ne retourne une décision, il refuse l'appel d'outil.
Utilisez [Contrôle de décision PermissionRequest](#permissionrequest-decision-control) pour permettre ou refuser au nom de l'utilisateur.

Utilisez cet événement quand vous avez besoin d'un signal au moment où Claude demande la permission d'utiliser un outil. Claude Code exécute un hook [Notification](#notification) avec le type `permission_prompt` uniquement après que l'invite ait attendu environ six secondes.

Claude Code n'exécute pas les hooks PermissionRequest pour la [requête réseau](/docs/fr/sandboxing#network-isolation) d'une commande en sandbox. Pour obtenir un signal pour cette invite, utilisez le type de notification `permission_prompt`.

Correspond au nom de l'outil, mêmes valeurs que PreToolUse.

<h4 id="permissionrequest-input">
  Entrée PermissionRequest
</h4>

Les hooks PermissionRequest reçoivent les champs `tool_name` et `tool_input` comme les hooks PreToolUse, mais sans `tool_use_id`. Pour un outil MCP, ils reçoivent également l'objet [`mcp_server`](#pretooluse-input). Un tableau optionnel `permission_suggestions` contient les [mises à jour de permission](#permission-update-entries) que Claude Code suggère pour cette requête, comme ajouter une règle d'autorisation ou changer le mode de permission.

Le tableau `permission_suggestions` n'est pas une liste exacte des options que vous voyez, parce que chaque dialogue de permission construit ses propres options. Certains dialogues, comme celui pour les éditions de fichiers, ne lisent pas du tout le tableau et dérivent leurs options de la requête elle-même. Un dialogue qui le lit peut toujours retenir une option dont la suggestion reste dans le tableau, par exemple quand [`allowManagedPermissionRulesOnly`](/docs/fr/settings-reference#allowmanagedpermissionrulesonly) cache les options de sauvegarde de règles. Il peut également offrir des options qui n'ont pas d'entrée de suggestion, comme [**Oui, et passer en mode auto**](/docs/fr/permission-modes#switch-permission-modes), qui change le mode de permission directement plutôt que via une mise à jour de permission.

Les hooks PreToolUse s'exécutent avant chaque appel d'outil, qu'il ait besoin de permission ou non. Les hooks PermissionRequest s'exécutent uniquement quand Claude Code est sur le point de vous demander la permission, ou quand il refuserait autrement un appel qui ne peut pas inviter. Aucun événement ne se déclenche pour [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior).

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

Les hooks `PermissionRequest` peuvent permettre ou refuser les demandes de permission. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner un objet `decision` avec ces champs spécifiques à l'événement :

| Champ                | Description                                                                                                                                                                                                                                                           |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` accorde la permission, `"deny"` la refuse. Les [règles de refus et de demande](/docs/fr/permissions#manage-permissions) sont toujours évaluées, donc un hook retournant `"allow"` ne remplace pas une règle de refus correspondante                              |
| `updatedInput`       | Pour `"allow"` uniquement : modifie les paramètres d'entrée de l'outil avant l'exécution. Remplace l'objet d'entrée entier, donc incluez les champs inchangés aux côtés des champs modifiés. L'entrée modifiée est réévaluée contre les règles de refus et de demande |
| `updatedPermissions` | Pour `"allow"` uniquement : tableau des [entrées de mise à jour de permission](#permission-update-entries) à appliquer, comme ajouter une règle d'autorisation ou changer le mode de permission de la session                                                         |
| `message`            | Pour `"deny"` uniquement : dit à Claude pourquoi la permission a été refusée                                                                                                                                                                                          |
| `interrupt`          | Pour `"deny"` uniquement : si `true`, arrête Claude                                                                                                                                                                                                                   |

Un hook qui quitte 2 sans un objet `decision` laisse le flux de permission inchangé, et son stderr est rejeté. Seul l'objet `decision` peut accorder ou refuser la requête.

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

Le champ de sortie `updatedPermissions` et le champ d'entrée [`permission_suggestions`](#permissionrequest-input) utilisent tous deux le même tableau d'objets d'entrée. Chaque entrée a un `type` qui détermine ses autres champs, et une `destination` qui contrôle où le changement est écrit.

| `type`              | Champs                             | Effet                                                                                                                                                                                                                               |
| :------------------ | :--------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `addRules`          | `rules`, `behavior`, `destination` | Ajoute des règles de permission. `rules` est un tableau d'objets `{toolName, ruleContent?}`. Omettez `ruleContent` pour correspondre à l'outil entier. `behavior` est `"allow"`, `"deny"`, ou `"ask"`                               |
| `replaceRules`      | `rules`, `behavior`, `destination` | Remplace toutes les règles du `behavior` donné à la `destination` par les `rules` fournies                                                                                                                                          |
| `removeRules`       | `rules`, `behavior`, `destination` | Supprime les règles correspondantes du `behavior` donné                                                                                                                                                                             |
| `setMode`           | `mode`, `destination`              | Change le mode de permission. Les modes valides sont `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan`, et `manual` comme alias pour `default`. L'alias `manual` nécessite Claude Code v2.1.200 ou ultérieur |
| `addDirectories`    | `directories`, `destination`       | Ajoute des répertoires de travail. `directories` est un tableau de chaînes de chemin                                                                                                                                                |
| `removeDirectories` | `directories`, `destination`       | Supprime les répertoires de travail                                                                                                                                                                                                 |

<Note>
  `setMode` avec `bypassPermissions` ne prend effet que si vous avez lancé la session avec le mode de contournement déjà disponible : `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, ou `permissions.defaultMode: "bypassPermissions"` dans les [paramètres utilisateur, `--settings`, ou gérés](/docs/fr/settings-reference#permissions-defaultmode). Sinon, la mise à jour est un non-op. La mise à jour est également un non-op quand [`permissions.disableBypassPermissionsMode`](/docs/fr/permissions#managed-settings) désactive le mode, ou quand la session démarre en [mode restreint](/docs/fr/cli-reference#cli-flags).

  `bypassPermissions` n'est jamais persisté comme `defaultMode` indépendamment de `destination`.
</Note>

Le champ `destination` sur chaque entrée détermine si le changement reste en mémoire ou persiste dans un fichier de paramètres.

| `destination`     | Écrit dans                                                |
| :---------------- | :-------------------------------------------------------- |
| `session`         | en mémoire uniquement, rejeté quand la session se termine |
| `localSettings`   | `.claude/settings.local.json`                             |
| `projectSettings` | `.claude/settings.json`                                   |
| `userSettings`    | `~/.claude/settings.json`                                 |

Un hook peut renvoyer l'une des `permission_suggestions` qu'il a reçues comme sa propre sortie `updatedPermissions`.

<h3 id="posttooluse">
  PostToolUse
</h3>

S'exécute immédiatement après qu'un outil se termine avec succès.

Correspond au nom de l'outil, mêmes valeurs que PreToolUse.

Correspondez plus largement quand le nom de l'outil n'est pas le bon filtre :

* Pour exécuter un hook après que n'importe quel outil se termine avec succès, omettez le `matcher` ou définissez-le à `"*"`. Votre hook peut alors découvrir ce qui a changé lui-même, par exemple en exécutant `git status --porcelain`, qui liste également les fichiers non suivis que `git diff` manque. Pour les appels d'outil qui échouent, ajoutez le même hook sous [PostToolUseFailure](#posttoolusefailure).
* Pour exécuter un hook quand un fichier spécifique change sur le disque, peu importe ce qui l'a écrit, utilisez [FileChanged](#filechanged). Claude Code n'exécute pas un hook `PostToolUse` correspondant à `Edit|Write` quand une commande `Bash` ou un processus en dehors de Claude Code réécrit le même fichier.

<h4 id="posttooluse-input">
  Entrée PostToolUse
</h4>

Les hooks `PostToolUse` se déclenchent après qu'un outil s'est déjà exécuté avec succès. L'entrée inclut à la fois `tool_input`, les arguments envoyés à l'outil, et `tool_response`, le résultat qu'il a retourné. Le schéma exact pour les deux dépend de l'outil. Les chemins `tool_input` des outils de fichier arrivent dans le même format que pour [PreToolUse](#pretooluse-input) : toujours absolu, avec les séparateurs natifs de la plateforme, donc les barres obliques inverses sur Windows. Pour un outil MCP, l'entrée porte également l'objet [`mcp_server`](#pretooluse-input).

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
    "type": "create"
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Champ         | Description                                                                                                                            |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| `duration_ms` | Optionnel. Temps d'exécution de l'outil en millisecondes. Exclut le temps passé dans les invites de permission et les hooks PreToolUse |

<h4 id="posttooluse-decision-control">
  Contrôle de décision PostToolUse
</h4>

Les hooks `PostToolUse` peuvent fournir des commentaires à Claude après l'exécution de l'outil. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner ces champs spécifiques à l'événement :

| Champ                  | Description                                                                                                                                                                                                                                                                                                               |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `decision`             | `"block"` ajoute la `reason` à côté du résultat de l'outil. Claude voit toujours la sortie originale ; pour la remplacer, utilisez `updatedToolOutput`                                                                                                                                                                    |
| `reason`               | Explication montrée à Claude quand `decision` est `"block"`                                                                                                                                                                                                                                                               |
| `additionalContext`    | Chaîne ajoutée au contexte de Claude aux côtés du résultat de l'outil. Voir [Ajouter du contexte pour Claude](#add-context-for-claude)                                                                                                                                                                                    |
| `classifierContext`    | Brève note sur le résultat de cet appel pour le classificateur du [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) plutôt que pour Claude. Voir [Annoter un résultat pour le classificateur du mode auto](#annotate-a-result-for-the-auto-mode-classifier). Nécessite Claude Code v2.1.236 ou ultérieur |
| `updatedToolOutput`    | Remplace la sortie de l'outil par la valeur fournie avant qu'elle ne soit envoyée à Claude. La valeur doit correspondre à la forme de sortie de l'outil                                                                                                                                                                   |
| `updatedMCPToolOutput` | Remplace la sortie pour les [outils MCP](#match-mcp-tools) uniquement. Préférez `updatedToolOutput`, qui fonctionne pour tous les outils                                                                                                                                                                                  |

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
  `updatedToolOutput` change uniquement ce que Claude voit. L'outil s'est déjà exécuté au moment où le hook se déclenche, donc tous les fichiers écrits, commandes exécutées, ou requêtes réseau envoyées ont déjà pris effet. La télémétrie comme les spans d'outil OpenTelemetry et les événements d'analyse capturent également la sortie originale avant que le hook s'exécute. Pour empêcher ou modifier un appel d'outil avant qu'il s'exécute, utilisez plutôt un hook [PreToolUse](#pretooluse).

  La valeur de remplacement doit correspondre à la forme de sortie de l'outil. Les outils intégrés retournent des objets structurés plutôt que des chaînes brutes. Par exemple, `Bash` retourne un objet avec les champs `stdout`, `stderr`, `interrupted`, et `isImage`. Pour les outils intégrés, une valeur qui ne correspond pas au schéma de sortie de l'outil est ignorée et la sortie originale est utilisée. La sortie d'outil MCP est transmise sans validation de schéma. Supprimer les détails d'erreur dont Claude a besoin peut le faire procéder sur une fausse hypothèse.
</Warning>

<h4 id="annotate-a-result-for-the-auto-mode-classifier">
  Annoter un résultat pour le classificateur du mode auto
</h4>

Retournez `classifierContext` pour envoyer une brève note sur le résultat de l'appel d'outil au classificateur du [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) plutôt qu'à Claude. Le classificateur [ne reçoit jamais les résultats d'outil eux-mêmes](/docs/fr/permission-modes#how-the-classifier-evaluates-actions), donc ce champ est la façon supportée de lui dire quelque chose sur ce qu'un appel a retourné avant qu'il examine les actions ultérieures. Le champ nécessite Claude Code v2.1.236 ou ultérieur.

L'exemple ci-dessous dit au classificateur d'où provient la sortie d'une requête :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "classifierContext": "This query ran against the staging database, not production."
  }
}
```

Le poids que le classificateur donne à la note dépend de l'endroit où vous avez configuré le hook :

* **Hooks configurés dans Claude Code** : pour les hooks des fichiers de paramètres, des plugins, des skills, et du frontmatter d'agent, le classificateur traite la note comme du contexte non vérifié fourni par l'application. La note n'établit jamais l'intention de l'utilisateur, et si elle prétend que vous avez approuvé ou demandé quelque chose, le classificateur vérifie cette affirmation contre vos propres messages dans la conversation
* **Rappels Agent SDK en processus** : quand une application intégrant Claude Code enregistre le hook comme un [rappel SDK TypeScript](/docs/fr/agent-sdk/hooks) et retourne la note pendant la session en direct, le classificateur peut peser une déclaration d'utilisateur relayée dans la note comme intention de l'utilisateur. Une telle déclaration peut satisfaire une exigence de consentement que le classificateur accepterait d'un message que vous envoyez, mais elle ne lève jamais un blocage que votre propre message ne pourrait pas lever non plus. Après qu'une session reprenne, Claude Code traite les notes restaurées comme du contexte non vérifié. Quand les hooks des deux groupes annotent le même appel, le classificateur traite la note combinée comme non vérifiée

Claude Code applique ces limites lors de la livraison de la note :

* **Longueur** : Claude Code plafonne les notes pour un appel d'outil à 2 000 caractères et tronque le reste. Le plafond est partagé entre chaque hook qui répond à cet appel
* **Réponses synchrones uniquement** : Claude Code ignore le champ dans la réponse d'un hook qui [s'exécute en arrière-plan](#run-hooks-in-the-background), parce que cette réponse arrive après que Claude Code enregistre le résultat de l'outil
* **Appels que le classificateur n'enregistre pas** : la transcription du classificateur omet les recherches en lecture seule comme les lectures de fichiers et les recherches. Claude Code rejette une note attachée à l'un de ces appels
* **Interaction avec les réécritures** : quand la note décrit la sortie que vous remplacez avec `updatedToolOutput`, retournez les deux champs dans la même réponse de hook. Claude Code rejette la note si cette réécriture est rejetée ou qu'une réécriture d'un autre hook la remplace. Claude Code livre une note que vous retournez sans réécriture même quand un autre hook réécrit la sortie

<Warning>
  Le classificateur lit le contenu que vous placez dans `classifierContext` comme des informations de l'application hébergeant la session, donc ne copiez pas la sortie d'outil non fiable ou le texte tiers dedans. Gardez la note à une brève affirmation sur cet appel uniquement, comme un fait sur son origine ou une déclaration d'utilisateur à ce sujet ; n'utilisez pas le champ pour livrer des messages non liés ou un flux d'événements.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

S'exécute quand un outil qui a commencé à s'exécuter échoue : l'outil a levé une erreur, ou un outil MCP a retourné un résultat d'erreur. Utilisez ceci pour enregistrer les échecs, envoyer des alertes, ou fournir des commentaires correctifs à Claude.

Correspond au nom de l'outil, mêmes valeurs que PreToolUse.

<Note>
  Cet événement ne se déclenche pas pour les appels d'outil rejetés avant l'exécution : un nom d'outil inconnu, une entrée qui échoue la validation de schéma ou spécifique à l'outil, ou un refus de permission. Les rejets de validation sont retournés comme des résultats `tool_use_error` et se produisent avant que les hooks s'exécutent, donc ils ne déclenchent ni `PreToolUse` ni `PostToolUseFailure`. Les refus de permission déclenchent `PreToolUse` mais pas cet événement ; voir [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  Entrée PostToolUseFailure
</h4>

Les hooks PostToolUseFailure reçoivent les mêmes champs `tool_name` et `tool_input` que PostToolUse, ainsi que les informations d'erreur comme champs de haut niveau. Pour un outil MCP, ils reçoivent également l'objet [`mcp_server`](#pretooluse-input). Par exemple, une commande `npm test` échouée pourrait livrer :

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
  "error": "Exit code 1\nError: Cannot find module 'express'",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Champ          | Description                                                                                                                                                                                                                                                                |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | Chaîne décrivant ce qui s'est mal passé. Le format dépend de l'outil qui a échoué                                                                                                                                                                                          |
| `is_interrupt` | Booléen optionnel. True quand l'échec a atteint Claude Code comme un abandon plutôt que comme une erreur que l'outil a signalée. L'annulation d'un outil en cours d'exécution ne déclenche pas ce hook ; le résultat de l'outil porte le message d'interruption à la place |
| `duration_ms`  | Optionnel. Temps d'exécution de l'outil en millisecondes. Exclut le temps passé dans les invites de permission et les hooks PreToolUse                                                                                                                                     |

La chaîne `error` est généralement le même texte que Claude reçoit comme résultat de l'outil échoué. Son format varie selon l'outil et l'échec. Clé votre hook sur `tool_name`, `is_interrupt`, et la première ligne `Exit code N` ; traitez le reste de la chaîne comme du texte d'affichage, pas un format stable.

* Pour Bash et PowerShell, une commande qui s'est exécutée et a quitté produit une première ligne `Exit code N`, puis toute sortie que la commande a produite comme un bloc avec stdout et stderr entrelacés
* Une charge utile peut également porter un message d'échec nu sans ligne de code de sortie, quand Claude Code n'a pas pu démarrer le processus shell lui-même
* Claude Code tronque au milieu les longues chaînes autour d'un marqueur `... [N characters truncated] ...`, et peut insérer des lignes de son propre, comme `Command timed out after 2m 0s`

<h4 id="posttoolusefailure-decision-control">
  Contrôle de décision PostToolUseFailure
</h4>

Les hooks `PostToolUseFailure` peuvent fournir du contexte à Claude après un échec d'outil. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner ces champs spécifiques à l'événement :

| Champ               | Description                                                                                                                 |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Chaîne ajoutée au contexte de Claude aux côtés de l'erreur. Voir [Ajouter du contexte pour Claude](#add-context-for-claude) |

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

S'exécute une fois après que chaque appel d'outil dans un lot se soit résolu, avant que Claude Code envoie la requête suivante au modèle. `PostToolUse` se déclenche une fois par outil, ce qui signifie qu'il se déclenche simultanément quand Claude effectue des appels d'outil parallèles. `PostToolBatch` se déclenche exactement une fois avec le lot complet, donc c'est le bon endroit pour injecter du contexte qui dépend de l'ensemble des outils qui se sont exécutés plutôt que de n'importe quel outil unique. Il n'y a pas de matcher pour cet événement.

<h4 id="posttoolbatch-input">
  Entrée PostToolBatch
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PostToolBatch reçoivent `tool_calls`, un tableau décrivant chaque appel d'outil dans le lot :

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

`tool_response` contient le même contenu que le modèle reçoit dans le bloc `tool_result` correspondant. La valeur est une chaîne sérialisée ou un tableau de bloc de contenu, exactement comme l'outil l'a émis. Pour `Read`, cela signifie du texte préfixé par le numéro de ligne plutôt que le contenu brut du fichier. Les réponses peuvent être grandes, donc analysez uniquement les champs dont vous avez besoin.

<Note>
  La forme `tool_response` diffère de celle de `PostToolUse`. `PostToolUse` passe l'objet `Output` structuré de l'outil, comme `{filePath: "...", type: "create"}` pour `Write` ; `PostToolBatch` passe le contenu `tool_result` sérialisé que le modèle voit.
</Note>

<h4 id="posttoolbatch-decision-control">
  Contrôle de décision PostToolBatch
</h4>

Les hooks `PostToolBatch` peuvent injecter du contexte pour Claude. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, votre script de hook peut retourner ces champs spécifiques à l'événement :

| Champ               | Description                                                                                                                                                                                                                                              |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Chaîne de contexte injectée une fois avant l'appel du modèle suivant. Voir [Ajouter du contexte pour Claude](#add-context-for-claude) pour les détails de livraison, ce qu'il faut y mettre, et comment les sessions reprises gèrent les valeurs passées |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Retourner `decision: "block"` ou `continue: false` arrête la boucle agentive avant l'appel du modèle suivant. Le message de blocage provient du JSON `reason` ou `stopReason`, ou de stderr sur la sortie 2. Vous le voyez comme un avertissement dans la transcription, et il reste dans la conversation, donc Claude le voit quand la conversation continue.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

S'exécute quand le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) refuse un appel d'outil, y compris quand il refuse sans verdict du classificateur parce qu'une [vérification de sécurité séparée du mode auto a refusé la propre requête du classificateur](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action) ou sa réponse n'a pas analysé. Ce hook ne se déclenche que en mode auto : il ne s'exécute pas quand vous refusez manuellement un dialogue de permission, quand un hook `PreToolUse` bloque un appel, ou quand une règle `deny` correspond. Utilisez-le pour enregistrer les refus, ajuster la configuration, ou dire au modèle qu'il peut réessayer l'appel d'outil.

Correspond au nom de l'outil, mêmes valeurs que PreToolUse.

<h4 id="permissiondenied-input">
  Entrée PermissionDenied
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PermissionDenied reçoivent `tool_name`, `tool_input`, `tool_use_id`, et `reason`. Pour un outil MCP, ils reçoivent également l'objet [`mcp_server`](#pretooluse-input).

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
  "reason": "[Irreversible Local Destruction]"
}
```

| Champ    | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `reason` | La raison du refus. Pour un verdict du classificateur, dans la plupart des sessions, il nomme la règle correspondante entre crochets, comme `[Data Exfiltration]` ; voir [Examiner les refus](/docs/fr/auto-mode-config#review-denials) pour les autres formes. Pour un [refus sans verdict](#permissiondenied-decision-control), il commence par `Auto mode could not evaluate this action and is blocking it for safety`. Pour un refus parce que le modèle du classificateur n'était pas disponible, c'est le texte fixe `Classifier unavailable` |

<h4 id="permissiondenied-decision-control">
  Contrôle de décision PermissionDenied
</h4>

Les hooks PermissionDenied peuvent dire au modèle qu'il peut réessayer l'appel d'outil refusé. Retournez un objet JSON avec `hookSpecificOutput.retry` défini à `true` :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Quand `retry` est `true`, Claude Code ajoute un message à la conversation disant au modèle qu'il peut réessayer l'appel d'outil. Claude Code ne renverse pas le refus lui-même. Si votre hook ne retourne pas JSON, ou retourne `retry: false`, le refus tient et le modèle reçoit le message de rejet original.

Claude Code ignore `retry: true` quand le classificateur a produit [aucun verdict sur l'action](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action) : sa réponse n'a pas analysé, ou une vérification de sécurité séparée du mode auto a refusé la propre requête du classificateur. Pour ces refus, Claude Code dit déjà au modèle dans le message de rejet s'il faut réessayer plus tard ou continuer.

<h3 id="notification">
  Notification
</h3>

S'exécute quand Claude Code envoie des notifications. Correspond au type de notification. Omettez le matcher pour exécuter les hooks pour tous les types de notification.

Vous recevez ces événements de hook même avec les notifications de bureau désactivées : le paramètre `preferredNotifChannel`, y compris `notifications_disabled`, change uniquement comment vous êtes alerté, pas si votre hook s'exécute.

| Matcher                      | Quand il se déclenche                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude a besoin de votre permission pour utiliser un outil ou la [requête réseau](/docs/fr/sandboxing#network-isolation) d'une commande en sandbox, et l'invite a attendu environ six secondes                                                                                                                                                                                                                                                                                                                                                            |
| `idle_prompt`                | Claude a fini de répondre il y a environ 60 secondes et vous n'avez pas tapé depuis                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `auth_success`               | L'authentification se termine                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `elicitation_dialog`         | Un serveur MCP ouvre un formulaire d'élicitation et vous n'avez pas tapé pendant environ six secondes                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `elicitation_url_dialog`     | Un serveur MCP vous demande d'ouvrir une URL de navigateur et vous n'avez pas tapé pendant environ six secondes                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `elicitation_complete`       | Un serveur MCP signale qu'une [élicitation en mode URL](#elicitation-input) est complète                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `elicitation_response`       | Une réponse d'élicitation MCP est renvoyée au serveur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `agent_needs_input`          | Une session en arrière-plan commence à attendre votre entrée pendant que la [vue agent](/docs/fr/agent-view) est ouverte dans un terminal, ou la session actuelle vous pose une [question de configuration de terminal d'un coéquipier d'équipe agent](/docs/fr/agent-teams#choose-a-display-mode) et vous n'avez pas tapé pendant environ six secondes                                                                                                                                                                                                        |
| `agent_completed`            | Une session en arrière-plan se termine ou échoue. Se déclenche uniquement pendant que la [vue agent](/docs/fr/agent-view) est ouverte dans un terminal                                                                                                                                                                                                                                                                                                                                                                                                    |
| `quota_auto_resume_fired`    | Claude Code continue votre tâche après qu'une limite d'utilisation claude.ai l'ait mise en pause : à la réinitialisation, ou plus tôt quand quelque chose que vous faites dans Claude Code pendant l'attente, comme ajouter des crédits d'utilisation, mettre à niveau votre plan, ou changer de modèles, rend l'utilisation disponible à nouveau, avec l'[exception de paramètre de modèle](/docs/fr/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                   |
| `quota_auto_resume_stale`    | Une limite d'utilisation claude.ai s'est réinitialisée pendant que votre ordinateur dormait pendant plus d'environ 30 minutes. Claude Code attend que vous appuyiez sur `Entrée` au lieu de continuer. Après un sommeil plus court, il continue et déclenche `quota_auto_resume_fired` à la place                                                                                                                                                                                                                                                    |
| `quota_auto_resume_disabled` | Claude Code termine son attente pour une limite d'utilisation claude.ai sans continuer votre tâche : [`autoContinueAtUsageLimit`](/docs/fr/settings-reference#autocontinueatusagelimit) s'est désactivé ou la réinitialisation s'est déplacée de plus de 24 heures pendant une attente que Claude Code a démarrée seul, la tâche continuée a continué à frapper la limite, ou la continuation a été bloquée avant d'atteindre le modèle. Ne se déclenche pas quand vous appuyez sur `Esc` ou `Ctrl+C`, ou choisissez **Ne pas continuer automatiquement** |

Les types `agent_needs_input` et `agent_completed` nécessitent Claude Code v2.1.198 ou ultérieur.

Les types `quota_auto_resume_fired`, `quota_auto_resume_stale`, et `quota_auto_resume_disabled` nécessitent Claude Code v2.1.234 ou ultérieur.

Dans les sessions de terminal, `permission_prompt` pour la requête réseau d'une commande en sandbox nécessite Claude Code v2.1.246 ou ultérieur.

`agent_needs_input` pour la question de configuration de terminal d'un coéquipier nécessite Claude Code v2.1.248 ou ultérieur.

<Note>
  Les types `permission_prompt`, `idle_prompt`, `elicitation_dialog`, et `elicitation_url_dialog` partagent leur timing avec les notifications de bureau, donc dans les sessions de terminal vous ne les voyez que quand vous semblez être loin du terminal :

  * Attendez `permission_prompt` une fois que vous n'avez pas tapé pendant environ six secondes. Le minuteur démarre quand l'invite de permission apparaît, et chaque frappe le reporte. Pour exécuter un hook immédiatement quand Claude demande la permission d'utiliser un outil, utilisez [PermissionRequest](#permissionrequest) à la place.
  * Attendez `idle_prompt` environ 60 secondes après que Claude finisse de répondre, et uniquement si vous n'avez pas tapé depuis. Claude Code n'envoie pas `idle_prompt` pendant qu'il attend qu'une limite d'utilisation claude.ai se réinitialise. Quand l'attente se termine d'elle-même, l'un des types `quota_auto_resume_*` se déclenche à la place.
  * Attendez `elicitation_dialog` pour un formulaire d'élicitation, ou `elicitation_url_dialog` pour une requête d'URL de navigateur, une fois que vous n'avez pas tapé pendant environ six secondes. Les deux partagent la même porte de six secondes que `permission_prompt` : le minuteur démarre quand le dialogue apparaît, et chaque frappe le reporte.

  Une requête de permission ou d'élicitation qui arrive pendant qu'un autre dialogue est à l'écran garde la même porte de six secondes, chronométrée à partir de quand la requête arrive. Sa notification peut vous atteindre pendant que la requête attend toujours derrière le dialogue ouvert.
</Note>

Claude Code chronomètre `permission_prompt` différemment dans les sessions où il envoie les requêtes de permission au rappel [`canUseTool`](/docs/fr/agent-sdk/user-input) d'Agent SDK, ce qui est comment Claude Desktop et l'extension VS Code hébergent Claude Code :

* Attendez `permission_prompt` environ six secondes après que Claude demande la permission. Claude Code ne le reporte pas pendant que vous tapez.
* Si vous ou un hook [PermissionRequest](#permissionrequest) répondez plus tôt, Claude Code n'exécute pas `permission_prompt`.
* Définissez [`CLAUDE_CODE_DISABLE_PERMISSION_PROMPT_NOTIFY_HOOKS`](/docs/fr/env-vars) à `1` pour désactiver `permission_prompt` dans ces sessions.

Avant v2.1.233, `permission_prompt` ne se déclenchait pas dans ces sessions.

Utilisez des matchers séparés pour exécuter différents gestionnaires selon le type de notification. Cette configuration déclenche un script d'alerte spécifique à la permission quand Claude a besoin d'approbation de permission et une notification différente quand Claude a été inactif :

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

En plus des [champs d'entrée communs](#common-input-fields), les hooks Notification reçoivent `message` avec le texte de notification, un `title` optionnel, et `notification_type` indiquant quel type s'est déclenché.

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

Les hooks Notification ne peuvent pas bloquer ou modifier les notifications. Claude Code rejette leurs champs `systemMessage` et `continue` mais émet toujours [`terminalSequence`](#emit-terminal-notifications), sur lequel l'exemple de notification de bureau s'appuie. Les hooks Notification sont destinés aux effets secondaires comme le transfert de la notification vers un service externe.

<h3 id="subagentstart">
  SubagentStart
</h3>

S'exécute quand Claude crée un sous-agent avec l'outil Agent, quand Claude [reprend un sous-agent](/docs/fr/sub-agents#resume-subagents), et chaque fois qu'un coéquipier d'[équipe agent](/docs/fr/agent-teams) en processus gère un nouveau message. Supporte les matchers pour filtrer par nom de type d'agent. Pour les agents intégrés, c'est le nom de l'agent comme `general-purpose`, `Explore`, ou `Plan`. Pour les [sous-agents personnalisés](/docs/fr/sub-agents), c'est le champ `name` du frontmatter de l'agent, pas le nom de fichier.

Pour les sous-agents fournis par un [plugin](/docs/fr/plugins), le type d'agent est l'identifiant scoped du plugin comme `my-plugin:reviewer`, pas le nom du frontmatter nu. Le deux-points place un nom scoped du plugin sur le chemin d'expression régulière, donc ancrez le matcher avec `^` et `$` pour une correspondance exacte : `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  Entrée SubagentStart
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks SubagentStart reçoivent `agent_id` avec l'identifiant unique du sous-agent et `agent_type` avec le nom de l'agent sur lequel le matcher filtre.

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

Les hooks SubagentStart ne peuvent pas bloquer la création de sous-agent, mais ils peuvent injecter du contexte dans le sous-agent. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, vous pouvez retourner :

| Champ               | Description                                                                                                                                                     |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Chaîne ajoutée au contexte du sous-agent au début de sa conversation, avant sa première invite. Voir [Ajouter du contexte pour Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

Quand le hook s'exécute à nouveau pour le même sous-agent, Claude Code injecte le contexte retourné uniquement quand le contexte du sous-agent ne contient pas déjà la copie d'une exécution antérieure. La copie injectée au lancement reste en place, laissant le [cache d'invite](/docs/fr/prompt-caching#subagents-and-the-cache) du sous-agent intact. Après que la [compaction automatique](/docs/fr/sub-agents#auto-compaction) rejette cette copie, Claude Code injecte le contexte de la prochaine exécution à nouveau.

<h3 id="subagentstop">
  SubagentStop
</h3>

S'exécute quand un sous-agent Claude Code a fini de répondre. Correspond au type d'agent, mêmes valeurs que SubagentStart.

<h4 id="subagentstop-input">
  Entrée SubagentStop
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks SubagentStop reçoivent `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path`, et `last_assistant_message`. Le champ `agent_type` est la valeur utilisée pour le filtrage du matcher. Le `transcript_path` est la transcription de la session principale, tandis que `agent_transcript_path` est la propre transcription du sous-agent stockée dans un dossier `subagents/` imbriqué. Le champ `last_assistant_message` contient le contenu textuel de la réponse finale du sous-agent, donc les hooks peuvent y accéder sans analyser le fichier de transcription.

Pas chaque événement SubagentStop provient d'un sous-agent que Claude a créé. Claude Code exécute également des agents internes pour certaines de ses propres fonctionnalités, comme les [suggestions d'invite](/docs/fr/interactive-mode#prompt-suggestions) et les [questions latérales `/btw`](/docs/fr/interactive-mode#side-questions-with-%2Fbtw), et SubagentStop se déclenche quand l'un de ceux-ci se termine aussi. Pour ces événements, `agent_type` est le nom de l'agent que la session elle-même exécute, comme celui défini avec [`--agent`](/docs/fr/cli-reference#cli-flags) ou le [paramètre `agent`](/docs/fr/settings-reference#agent), et une chaîne vide quand la session s'exécute sans un.

Un `matcher` qui nomme les types d'agent ne correspond pas à un `agent_type` vide. Un hook dont le matcher est omis, `""`, ou `"*"`, ou est une expression régulière qui correspond à une chaîne vide, s'exécute pour les événements avec un `agent_type` vide aussi.

Sur Claude Code v2.1.271 ou ultérieur, un sous-agent qui s'exécute avec l'outil [`SubagentHandback`](/docs/fr/tools-reference) livre son rapport via cet outil avant qu'il ne s'arrête. Le champ `last_assistant_message` contient alors le texte de fermeture du sous-agent, le cas échéant, qui n'est pas le rapport livré. Le rapport est l'entrée `message` de cet appel, qu'un hook `PreToolUse` ou `PostToolUse` correspondant à `SubagentHandback` reçoit comme `tool_input.message`.

Les hooks SubagentStop reçoivent également les tableaux `background_tasks` et `session_crons` décrits sous [Entrée Stop](#stop-input). Les deux tableaux sont scoped à la session parent, pas au sous-agent.

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

Les hooks SubagentStop utilisent le même format de contrôle de décision que les [hooks Stop](#stop-decision-control), y compris `hookSpecificOutput.additionalContext` avec `hookEventName` défini à `"SubagentStop"`, pour les commentaires sans erreur qui gardent le sous-agent en cours d'exécution. Retourner `decision: "block"` avec une `reason` garde le sous-agent en cours d'exécution et livre `reason` au sous-agent comme sa prochaine instruction. Un hook qui bloque en quittant 2 livre son message stderr de la même façon. Pour injecter du contexte dans la session parent après qu'un sous-agent retourne, utilisez plutôt un hook [`PostToolUse`](#posttooluse) sur l'outil `Agent`.

<h3 id="taskcreated">
  TaskCreated
</h3>

S'exécute quand une tâche est en cours de création via l'outil `TaskCreate`. Utilisez ceci pour appliquer les conventions de nommage, exiger les descriptions de tâche, ou empêcher certaines tâches d'être créées. Dans une [session sans les outils Task](/docs/fr/tools-reference#task-tool-availability), cet événement ne se déclenche pas.

Les hooks TaskCreated ne supportent pas les matchers et se déclenchent à chaque occurrence.

<h4 id="taskcreated-input">
  Entrée TaskCreated
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks TaskCreated reçoivent `task_id`, `task_subject`, et optionnellement `task_description`, `teammate_name`, et `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Champ              | Description                                                                         |
| :----------------- | :---------------------------------------------------------------------------------- |
| `task_id`          | Identifiant de la tâche en cours de création                                        |
| `task_subject`     | Titre de la tâche                                                                   |
| `task_description` | Description détaillée de la tâche. Peut être absent                                 |
| `teammate_name`    | Nom du coéquipier créant la tâche. Peut être absent                                 |
| `team_name`        | Déprécié. Nom d'équipe dérivé de la session ; sera supprimé dans une version future |

<h4 id="taskcreated-decision-control">
  Contrôle de décision TaskCreated
</h4>

Un hook TaskCreated peut bloquer la création de deux façons. De l'une ou l'autre façon, Claude Code supprime la tâche et retourne votre message à Claude comme l'erreur de l'outil. Claude Code ignore `continue: false` de cet événement et Claude continue de travailler.

* **Code de sortie 2** : Claude Code retourne le texte stderr comme le message.
* **JSON `{"decision": "block", "reason": "..."}`** : Claude Code retourne `reason` comme le message.

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

S'exécute quand une tâche est en cours de marquage comme complétée. Cela se déclenche dans deux situations : qu'un agent marque explicitement une tâche comme complétée via l'outil TaskUpdate, ou quand un coéquipier d'[équipe agent](/docs/fr/agent-teams) termine son tour avec des tâches en cours. Utilisez ceci pour appliquer les critères de complétion comme passer les tests ou les vérifications lint avant qu'une tâche puisse se fermer.

Les hooks TaskCompleted ne supportent pas les matchers et se déclenchent à chaque occurrence.

<h4 id="taskcompleted-input">
  Entrée TaskCompleted
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks TaskCompleted reçoivent `task_id`, `task_subject`, et optionnellement `task_description`, `teammate_name`, et `team_name`.

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

| Champ              | Description                                                                         |
| :----------------- | :---------------------------------------------------------------------------------- |
| `task_id`          | Identifiant de la tâche en cours de complétion                                      |
| `task_subject`     | Titre de la tâche                                                                   |
| `task_description` | Description détaillée de la tâche. Peut être absent                                 |
| `teammate_name`    | Nom du coéquipier complétant la tâche. Peut être absent                             |
| `team_name`        | Déprécié. Nom d'équipe dérivé de la session ; sera supprimé dans une version future |

<h4 id="taskcompleted-decision-control">
  Contrôle de décision TaskCompleted
</h4>

Les hooks TaskCompleted supportent deux façons de contrôler la complétion de tâche :

* **Code de sortie 2** : la tâche n'est pas marquée comme complétée et le message stderr est renvoyé au modèle comme commentaire.
* **JSON `{"continue": false, "stopReason": "..."}`** : quand un coéquipier terminant son tour a déclenché l'événement, arrête le coéquipier entièrement, correspondant au comportement du hook `Stop`. Le `stopReason` est montré à l'utilisateur. Quand l'outil `TaskUpdate` a déclenché l'événement, Claude Code ignore `continue: false` ; le code de sortie 2 bloque toujours la complétion.

Cet exemple exécute les tests et bloque la complétion de tâche s'ils échouent :

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Run the test suite
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

S'exécute quand l'agent Claude Code principal a fini de répondre. Ne s'exécute pas si l'arrêt s'est produit en raison d'une interruption utilisateur. Les erreurs API déclenchent plutôt [StopFailure](#stopfailure).

<Tip>
  La commande [`/goal`](/docs/fr/goal) est un raccourci intégré pour un hook Stop scoped à la session basé sur les invites. Utilisez-le quand vous voulez que Claude continue à travailler vers une condition sans écrire la configuration du hook.
</Tip>

<h4 id="stop-input">
  Entrée Stop
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks Stop reçoivent `stop_hook_active`, `last_assistant_message`, `background_tasks`, et `session_crons`. Le champ `stop_hook_active` est `true` quand Claude Code continue déjà en raison d'un hook stop. Vérifiez cette valeur ou traitez la transcription pour éviter de bloquer sur une condition qui ne se résoudra jamais. Claude Code remplace le hook et termine le tour après 8 blocages consécutifs.

Le champ `last_assistant_message` contient le contenu textuel de la réponse finale de Claude, donc les hooks peuvent y accéder sans analyser le fichier de transcription. Pour les hooks qui agissent sur le tour qui vient de se terminer, comme les hooks de lecture à haute voix ou de notification, utilisez ce champ plutôt que de lire `transcript_path` : le fichier de transcription n'est pas garanti d'inclure le message final au moment de Stop sur toutes les versions.

Les tableaux `background_tasks` et `session_crons` permettent aux hooks de distinguer « la session est terminée » de « la session est en pause en attente que le travail en arrière-plan la réveille ». Les deux tableaux sont présents quand le registre de tâches est accessible et sont vides quand rien n'est en vol ou programmé.

Chaque entrée dans `background_tasks` décrit une tâche en vol et utilise ces champs :

| Champ         | Description                                                                                                                                                                                                                                                                |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Identifiant de tâche                                                                                                                                                                                                                                                       |
| `type`        | Étiquette de type de tâche conviviale comme `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session`, ou `MCP task`. Chaque étiquette identifie quelle fonctionnalité Claude Code a créé la tâche. Revient au discriminant brut pour les types non reconnus |
| `status`      | État actuel de la tâche                                                                                                                                                                                                                                                    |
| `description` | Description en texte libre, plafonnée à 1 000 caractères avec un marqueur `… [+N chars]` en chaîne quand coupée                                                                                                                                                            |
| `command`     | Ligne de commande shell, plafonnée à 1 000 caractères. Présent uniquement pour les tâches `shell`                                                                                                                                                                          |
| `agent_type`  | Nom du type de sous-agent. Présent uniquement pour les tâches `subagent`                                                                                                                                                                                                   |
| `server`      | Nom du serveur MCP. Présent uniquement pour les tâches `monitor` et `MCP task`                                                                                                                                                                                             |
| `tool`        | Nom de l'outil MCP. Présent uniquement pour les tâches `monitor` et `MCP task`                                                                                                                                                                                             |
| `name`        | Nom du workflow. Présent uniquement pour les tâches `workflow`                                                                                                                                                                                                             |

Chaque entrée dans `session_crons` décrit un réveil programmé scoped à la session, provenant de `CronCreate`, `ScheduleWakeup`, et `/loop` :

| Champ       | Description                                                                                                                                                  |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Identifiant de tâche cron                                                                                                                                    |
| `schedule`  | Expression cron, par exemple `0 9 * * 1-5`                                                                                                                   |
| `recurring` | `false` pour les réveils ponctuels dont l'horaire encode un seul temps de déclenchement, `true` pour les tâches qui se redéclenchent à chaque correspondance |
| `prompt`    | Invite soumise quand le cron se déclenche, plafonnée à 1 000 caractères avec le même marqueur `… [+N chars]`                                                 |

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

| Champ                                  | Description                                                                                                                                                                                                                                |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` empêche Claude de s'arrêter. Omettez pour permettre à Claude de s'arrêter                                                                                                                                                        |
| `reason`                               | Requis quand `decision` est `"block"`. Dit à Claude pourquoi il devrait continuer                                                                                                                                                          |
| `hookSpecificOutput.additionalContext` | Commentaires sans erreur pour Claude. La conversation continue pour que Claude puisse agir dessus, mais contrairement à `decision: "block"`, elle est montrée dans la transcription comme commentaire de hook plutôt qu'une erreur de hook |

Un hook qui bloque en quittant 2 s'achemine de la même façon que `reason` : Claude reçoit le message stderr comme l'explication de pourquoi il devrait continuer.

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Utilisez `additionalContext` quand le hook fonctionne comme prévu et donne des conseils à Claude, comme « exécutez la suite de tests avant de terminer ». Cela garde la conversation en cours via les mêmes protections de boucle que `decision: "block"`, à savoir l'entrée `stop_hook_active` et le plafond de 8 continuations consécutives, mais la transcription l'étiquette `Stop hook feedback` et aucune notification d'erreur de hook n'est montrée :

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

S'exécute à la place de [Stop](#stop) quand le tour se termine en raison d'une erreur API. Claude Code ignore la sortie et le code de sortie du hook, à part [`terminalSequence`](#emit-terminal-notifications). Utilisez ceci pour enregistrer les échecs, envoyer des alertes, ou prendre des actions de récupération quand Claude ne peut pas terminer une réponse en raison des limites de débit, des problèmes d'authentification, ou d'autres erreurs API.

<h4 id="stopfailure-input">
  Entrée StopFailure
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks StopFailure reçoivent `error`, optionnel `error_details`, et optionnel `last_assistant_message`. Le champ `error` identifie le type d'erreur et est utilisé pour le filtrage du matcher.

| Champ                    | Description                                                                                                                                                                                                                                                          |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Type d'erreur : `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, ou `unknown`                  |
| `error_details`          | Détails supplémentaires sur l'erreur, quand disponibles                                                                                                                                                                                                              |
| `last_assistant_message` | Le texte d'erreur rendu montré dans la conversation. Contrairement à `Stop` et `SubagentStop`, où ce champ contient la sortie conversationnelle de Claude, pour `StopFailure`, il contient la chaîne d'erreur API elle-même, comme `"API Error: Rate limit reached"` |

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

Les hooks StopFailure n'ont pas de contrôle de décision. Ils s'exécutent à des fins de notification et de logging uniquement.

<h3 id="teammateidle">
  TeammateIdle
</h3>

S'exécute quand un coéquipier d'[équipe agent](/docs/fr/agent-teams) est sur le point de devenir inactif après avoir terminé son tour. Utilisez ceci pour appliquer les portes de qualité avant qu'un coéquipier arrête de travailler, comme exiger les vérifications lint de passage ou vérifier que les fichiers de sortie existent.

Les hooks TeammateIdle ne supportent pas les matchers et se déclenchent à chaque occurrence.

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

| Champ           | Description                                                                         |
| :-------------- | :---------------------------------------------------------------------------------- |
| `teammate_name` | Nom du coéquipier qui est sur le point de devenir inactif                           |
| `team_name`     | Déprécié. Nom d'équipe dérivé de la session ; sera supprimé dans une version future |

<h4 id="teammateidle-decision-control">
  Contrôle de décision TeammateIdle
</h4>

Les hooks TeammateIdle supportent deux façons de contrôler le comportement du coéquipier :

* **Code de sortie 2** : le coéquipier reçoit le message stderr comme commentaire et continue de travailler au lieu de devenir inactif.
* **JSON `{"continue": false, "stopReason": "..."}`** : arrête le coéquipier entièrement, correspondant au comportement du hook `Stop`. Le `stopReason` est montré à l'utilisateur.

Cet exemple vérifie qu'un artefact de construction existe avant de permettre à un coéquipier de devenir inactif :

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

S'exécute quand un fichier de configuration change pendant une session. Utilisez ceci pour auditer les changements de paramètres, appliquer les politiques de sécurité, ou bloquer les modifications non autorisées aux fichiers de configuration.

Claude Code exécute les hooks ConfigChange quand un fichier de paramètres, un fichier de politique gérée, ou un fichier de skill change. Pour la politique gérée, il les exécute uniquement quand `managed-settings.json` ou un fichier dans `managed-settings.d/` change. Il applique les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) et les changements aux préférences gérées macOS ou à la politique du registre Windows sans les exécuter. Sur WSL avec [`wslInheritsWindowsSettings`](/docs/fr/settings-reference#wslinheritswindowssettings), il applique également un fichier de paramètres gérés Windows modifié du côté Windows sur son sondage de politique sans les exécuter.

Le matcher filtre sur la source de configuration :

| Matcher            | Quand il se déclenche                                                   |
| :----------------- | :---------------------------------------------------------------------- |
| `user_settings`    | `~/.claude/settings.json` change                                        |
| `project_settings` | `.claude/settings.json` change                                          |
| `local_settings`   | `.claude/settings.local.json` change                                    |
| `policy_settings`  | `managed-settings.json` ou un fichier dans `managed-settings.d/` change |
| `skills`           | Un fichier de skill dans `.claude/skills/` change                       |

Cet exemple enregistre tous les changements de configuration pour l'audit de sécurité :

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

En plus des [champs d'entrée communs](#common-input-fields), les hooks ConfigChange reçoivent `source` et optionnellement `file_path`. Le champ `source` indique quel type de configuration a changé, et `file_path` fournit le chemin du fichier spécifique qui a été modifié.

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

Les hooks ConfigChange peuvent bloquer les changements de configuration de prendre effet. Utilisez le code de sortie 2 ou un JSON `decision` pour empêcher le changement. Quand bloqué, les nouveaux paramètres ne sont pas appliqués à la session en cours d'exécution.

| Champ      | Description                                                                                            |
| :--------- | :----------------------------------------------------------------------------------------------------- |
| `decision` | `"block"` empêche le changement de configuration d'être appliqué. Omettez pour permettre le changement |
| `reason`   | Accepté mais jamais montré                                                                             |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

Les changements `policy_settings` ne peuvent pas être bloqués. Les hooks se déclenchent toujours pour les sources `policy_settings` quand un fichier de paramètres gérés sur la machine change, pour que vous puissiez enregistrer ces édits, mais toute décision de blocage est ignorée. Cela garantit que les paramètres gérés par l'entreprise prennent toujours effet. Claude Code n'exécute pas les hooks `ConfigChange` quand les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) arrivent ou se rafraîchissent.

Claude Code agit sur la décision de blocage de la sortie JSON d'un hook ConfigChange et rejette `systemMessage` et `continue`. Un changement bloqué ne surface aucun message pour vous ou pour Claude, que vous bloquez avec `reason` ou avec stderr sur la sortie 2. Claude Code écrit uniquement une ligne dans le journal de débogage.

<h3 id="cwdchanged">
  CwdChanged
</h3>

S'exécute quand une commande shell dans la conversation principale change le répertoire de travail, par exemple quand Claude exécute une commande `cd`. Utilisez ceci pour réagir aux changements de répertoire : recharger les variables d'environnement, activer les chaînes d'outils spécifiques au projet, ou exécuter les scripts de configuration automatiquement. S'apparie avec [FileChanged](#filechanged) pour les outils comme [direnv](https://direnv.net/) qui gèrent l'environnement par répertoire.

Les hooks CwdChanged ont accès à [`CLAUDE_ENV_FILE`](#persist-environment-variables). Les variables écrites dans ce fichier persistent dans les commandes Bash suivantes jusqu'au prochain événement CwdChanged, quand Claude Code les efface.

CwdChanged ne supporte pas les matchers et se déclenche à chaque occurrence.

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

En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, les hooks CwdChanged peuvent retourner `watchPaths` pour définir dynamiquement quels chemins de fichier [FileChanged](#filechanged) surveille :

| Champ        | Description                                                                                                                                                                                                                                                                   |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Tableau de chemins absolus. Remplace la liste de surveillance dynamique actuelle. Les chemins de votre configuration `matcher` sont toujours surveillés. Retourner un tableau vide efface la liste dynamique, ce qui est typique quand vous entrez dans un nouveau répertoire |

Les hooks CwdChanged n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer le changement de répertoire.

Claude Code lit `watchPaths` et `systemMessage` de leur sortie JSON et rejette `continue`. Dans les sessions interactives, il montre le `systemMessage` comme une brève notification de terminal. Le message n'atteint pas le flux de message SDK.

<h3 id="directoryadded">
  DirectoryAdded
</h3>

S'exécute après que vous ajoutiez un répertoire de travail en cours de session avec la commande `/add-dir`, ou après qu'un client SDK en ajoute un avec la requête de contrôle `register_repo_root`. Utilisez ceci pour préparer un référentiel nouvellement ajouté, par exemple en installant ses dépendances.

Claude Code ne déclenche pas cet événement quand :

* Vous passez un répertoire avec le drapeau de démarrage `--add-dir` ; [SessionStart](#sessionstart) couvre ces répertoires
* Vous ajoutez un répertoire sur l'onglet Workspace `/permissions`
* Vous ajoutez un répertoire qui est déjà un répertoire de travail ou à l'intérieur d'un

Claude Code se déclenche DirectoryAdded après avoir rafraîchi l'état du sandbox et de la permission, donc les outils en sandbox voient déjà le nouveau répertoire quand votre hook s'exécute. Les commandes du hook elles-mêmes s'exécutent sans sandbox.

Claude Code n'attend pas le hook : l'ajout se termine immédiatement, et le hook s'exécute en arrière-plan avec le délai d'expiration par défaut de 600 secondes.

Le matcher filtre sur la façon dont le répertoire a été ajouté :

| Matcher              | Quand il se déclenche                                                               |
| :------------------- | :---------------------------------------------------------------------------------- |
| `slash_command`      | Vous ajoutez un répertoire avec `/add-dir`                                          |
| `register_repo_root` | Un client SDK ajoute un répertoire avec la requête de contrôle `register_repo_root` |

<h4 id="directoryadded-input">
  Entrée DirectoryAdded
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks DirectoryAdded reçoivent `directory` et `source`.

| Champ       | Description                                                                                                                     |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `directory` | Chemin absolu du répertoire qui a été ajouté                                                                                    |
| `source`    | Comment le répertoire a été ajouté, `"slash_command"` pour `/add-dir` ou `"register_repo_root"` pour la requête de contrôle SDK |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "DirectoryAdded",
  "directory": "/Users/my-other-repo",
  "source": "slash_command"
}
```

Les hooks DirectoryAdded n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer l'ajout, qui s'est déjà terminé quand le hook s'exécute. Claude Code rejette le champ `continue` de leur sortie JSON et affiche le reste différemment par source :

* `slash_command` : Claude Code livre le `systemMessage` du hook à Claude comme contexte sur le tour de conversation suivant, plutôt que de vous le montrer. Un nombre de hooks échoués apparaît dans la transcription. La sortie d'échec complète va dans le journal de débogage
* `register_repo_root` : Claude Code écrit la sortie `systemMessage` et la sortie d'échec uniquement dans le journal de débogage

<h3 id="filechanged">
  FileChanged
</h3>

S'exécute quand un fichier surveillé change sur le disque. Claude Code détecte les changements avec un observateur de système de fichiers, pas en inspectant les appels d'outil, donc il exécute le hook peu importe ce qui a changé le fichier : un appel d'outil `Edit` ou `Write`, un script que Claude exécute avec `Bash`, ou un processus en dehors de Claude Code entièrement. Un usage courant est de recharger les variables d'environnement quand les fichiers de configuration du projet changent.

Le `matcher` pour cet événement sert deux rôles :

* **Construire la liste de surveillance** : la valeur est divisée sur `|` et chaque segment est enregistré comme un nom de fichier littéral dans le répertoire de travail, donc `".envrc|.env"` surveille exactement ces deux fichiers. Les modèles regex ne sont pas utiles ici : une valeur comme `^\.env` surveillerait un fichier littéralement nommé `^\.env`.
* **Filtrer quels hooks s'exécutent** : quand un fichier surveillé change, la même valeur filtre quels groupes de hook s'exécutent en utilisant les [règles de matcher](#matcher-patterns) standard contre le nom de base du fichier modifié.

Cet exemple normalise les fins de ligne dans `data.csv` après n'importe quel changement, y compris une commande `Bash` ou un script externe réécrivant le fichier :

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": "data.csv",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/normalize-line-endings.sh"
          }
        ]
      }
    ]
  }
}
```

Le hook lit le chemin absolu du fichier modifié du champ `file_path` de l'[entrée JSON](#filechanged-input) sur stdin. Sa garde `grep` teste la même chose que `perl` supprime, un CR à la fin d'une ligne, donc l'exécution après une normalisation quitte sans toucher le fichier. Une garde plus lâche boucle pour toujours, parce que `perl -i` réécrit le fichier même quand il ne substitue rien et Claude Code exécute le hook à nouveau après chaque réécriture. Enregistrez ce script à `/path/to/normalize-line-endings.sh` et rendez-le exécutable :

```bash theme={null}
#!/bin/bash
FILE=$(jq -r .file_path)
if grep -q $'\r$' "$FILE"; then
  perl -pi -e 's/\r$//' "$FILE"
fi
```

Pour confirmer que le hook fonctionne, demandez à Claude d'ajouter une ligne CRLF à `data.csv` avec une commande `Bash`. Claude Code exécute le hook et le fichier se termine avec les fins de ligne LF.

Pour surveiller les fichiers que vous ne pouvez pas nommer à l'avance, retournez [`watchPaths`](#filechanged-output) d'un hook pour mettre à jour la liste de surveillance dynamiquement. Claude Code démarre l'observateur uniquement quand quelque chose nomme un fichier à surveiller, donc semez la liste avec un groupe FileChanged dont le matcher nomme au moins un fichier, ou avec un hook [SessionStart](#sessionstart-decision-control) ou [CwdChanged](#cwdchanged) qui retourne `watchPaths`. Le matcher filtre toujours quels groupes de hook s'exécutent quand un fichier surveillé change, donc donnez au groupe qui gère les chemins dynamiques un matcher omis, qui correspond à chaque fichier surveillé et n'ajoute rien à la liste de surveillance. Un matcher `"*"` correspond également à chaque fichier, mais Claude Code l'enregistre dans la liste de surveillance comme n'importe quelle autre valeur, comme un fichier littéralement nommé `*`.

Les hooks FileChanged ont accès à [`CLAUDE_ENV_FILE`](#persist-environment-variables). Les variables écrites dans ce fichier persistent dans les commandes Bash suivantes jusqu'au prochain événement [CwdChanged](#cwdchanged), quand Claude Code les efface.

<h4 id="filechanged-input">
  Entrée FileChanged
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks FileChanged reçoivent `file_path` et `event`.

| Champ       | Description                                                                                                                   |
| :---------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `file_path` | Chemin absolu du fichier qui a changé                                                                                         |
| `event`     | Ce qui s'est passé : `"change"` pour un fichier modifié, `"add"` pour un fichier créé, ou `"unlink"` pour un fichier supprimé |

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

En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, les hooks FileChanged peuvent retourner `watchPaths` pour mettre à jour dynamiquement quels chemins de fichier sont surveillés :

| Champ        | Description                                                                                                                                                                                                                                                                         |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Tableau de chemins absolus. Remplace la liste de surveillance dynamique actuelle. Les chemins de votre configuration `matcher` sont toujours surveillés. Utilisez ceci quand votre script de hook découvre des fichiers supplémentaires à surveiller en fonction du fichier modifié |

Les hooks FileChanged n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer le changement de fichier de se produire.

Claude Code lit `watchPaths` et `systemMessage` de leur sortie JSON et rejette `continue`. Dans les sessions interactives, il montre le `systemMessage` comme une brève notification de terminal. Le message n'atteint pas le flux de message SDK.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

S'exécute quand un worktree est en cours de création, que ce soit à partir de `claude --worktree`, à partir d'un [sous-agent utilisant `isolation: "worktree"`](/docs/fr/sub-agents#choose-the-subagent-scope), ou pour une [session en arrière-plan](/docs/fr/agent-view#how-file-edits-are-isolated) que Claude Code isole dans son propre worktree. Par défaut, Claude Code crée la copie de travail isolée avec `git worktree`. Configurer un hook WorktreeCreate remplace ce comportement git par défaut, vous permettant d'utiliser un système de contrôle de version différent comme SVN, Perforce, ou Mercurial.

Parce que le hook remplace le comportement par défaut entièrement, [`.worktreeinclude`](/docs/fr/worktrees#copy-gitignored-files-into-worktrees) n'est pas traité. Si vous avez besoin de copier les fichiers de configuration locaux comme `.env` dans le nouveau worktree, faites-le à l'intérieur de votre script de hook.

Le hook doit retourner le chemin du répertoire worktree créé. Claude Code utilise ce chemin comme le répertoire de travail pour la session isolée. Voir [Sortie WorktreeCreate](#worktreecreate-output) pour savoir comment chaque type de hook retourne le chemin.

Claude Code agit sur le succès du hook et le chemin retourné, et rejette `systemMessage` et `continue`.

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

Le hook lit le `name` du worktree de l'entrée JSON sur stdin, extrait une copie fraîche dans un nouveau répertoire, et imprime le chemin du répertoire. Le `echo` sur la dernière ligne est ce que Claude Code lit comme le chemin du worktree. Redirigez toute autre sortie vers stderr pour qu'elle n'interfère pas avec le chemin.

<h4 id="worktreecreate-input">
  Entrée WorktreeCreate
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks WorktreeCreate reçoivent le champ `name`. C'est un identifiant slug pour le nouveau worktree, soit spécifié par l'utilisateur, soit auto-généré, par exemple `bold-oak-a3f2`.

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

Les hooks WorktreeCreate n'utilisent pas le modèle de décision permettre/bloquer standard. Au lieu de cela, le succès ou l'échec du hook détermine le résultat. Le hook doit retourner le chemin du répertoire worktree créé :

* **Hooks de commande** (`type: "command"`): imprimez le chemin comme la dernière ligne non-vide de stdout. Claude Code supprime les codes d'échappement ANSI avant de lire cette ligne, donc les bannières de démarrage du shell imprimées avant votre `echo` sont ignorées. Redirigez toute autre sortie du hook vers stderr.
* **Hooks HTTP** (`type: "http"`): retournez `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` dans le corps de la réponse.

Si le hook échoue ou ne produit pas de chemin, la création du worktree échoue avec une erreur.

Claude Code résout un chemin relatif contre le répertoire dans lequel le hook s'est exécuté, en effondrant tout segment `.` ou `..` dedans. Si le chemin résultant n'est pas un répertoire que Claude Code peut entrer, la session imprime une erreur nommant le chemin et quitte avec le code 1.

Claude Code refuse un chemin absolu qui contient des segments `.` ou `..`, et n'importe quel chemin qui passe par un lien symbolique en dessous de la racine du référentiel, parce qu'un lien symbolique commis au référentiel pourrait rediriger le worktree en dehors de lui. L'erreur nomme le composant rejeté. Retournez un chemin normalisé qui ne passe pas par un lien symbolique à l'intérieur du référentiel. Avant v2.1.216, la création du worktree suivait le chemin du hook sans ce dépistage.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

S'exécute quand un worktree est en cours de suppression. C'est la contrepartie de nettoyage de [WorktreeCreate](#worktreecreate). L'événement se déclenche quand :

* vous quittez une session `--worktree` et choisissez de la supprimer
* un sous-agent avec `isolation: "worktree"` se termine
* vous supprimez une [session en arrière-plan](/docs/fr/agent-view#what-deleting-a-session-removes) dont le worktree a été créé par le hook

Pour les worktrees basés sur git, Claude Code gère le nettoyage automatiquement avec `git worktree remove`. Si vous avez configuré un hook WorktreeCreate pour un système de contrôle de version non-git, associez-le à un hook WorktreeRemove pour gérer le nettoyage. Sans un, le répertoire worktree est laissé sur le disque.

Claude Code rejette les [champs de sortie JSON](#json-output) d'un hook WorktreeRemove, comme `systemMessage` et `continue`.

Pour une suppression de session en arrière-plan, Claude Code vérifie le chemin du worktree stocké avant d'exécuter le hook et refuse un chemin qui est un lien symbolique ou passe par un en dessous de la racine du référentiel. Le hook s'exécute pour un worktree qui contient toujours des fichiers uniquement quand vous confirmez la suppression dans la [vue agent](/docs/fr/agent-view#what-deleting-a-session-removes) ; pour un tel worktree, [`claude rm`](/docs/fr/agent-view#manage-sessions-from-the-shell) garde la session et le worktree à la place. Avant v2.1.216, le hook s'exécutait sur le chemin stocké sans ces vérifications.

Claude Code passe le chemin retourné par WorktreeCreate comme `worktree_path` dans l'entrée du hook. Cet exemple lit ce chemin et supprime le répertoire :

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

Le code de sortie d'un hook WorktreeRemove décide du résultat. Quand un hook quitte non-zéro et le répertoire à `worktree_path` existe toujours après, la suppression échoue :

* Le worktree reste sur le disque, et la commande du hook et stderr vont dans le [journal de débogage](#debug-hooks).
* Si vous supprimiez une session en arrière-plan, la session reste aussi. Le message de refus dans la [vue agent](/docs/fr/agent-view#what-deleting-a-session-removes) rapporte comment le hook s'est terminé, comme `exited 1`, cite le début de son stderr, et dit si la suppression de la session à nouveau supprime le répertoire de toute façon.

<h3 id="precompact">
  PreCompact
</h3>

S'exécute avant que Claude Code soit sur le point d'exécuter une opération de compaction.

La valeur du matcher indique si la compaction a été déclenchée manuellement ou automatiquement :

| Matcher  | Quand il se déclenche                                                                                                                     |
| :------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `manual` | `/compact`                                                                                                                                |
| `auto`   | Compaction automatique quand la conversation atteint la [fenêtre de compaction automatique](/docs/fr/model-config#set-the-auto-compact-window) |

Quittez avec le code 2 pour bloquer la compaction. Pour un `/compact` manuel, le message stderr est montré à l'utilisateur. Vous pouvez également bloquer en retournant JSON avec `"decision": "block"`.

Bloquer la compaction automatique a des effets différents selon quand elle se déclenche. Si la compaction a été déclenchée de manière proactive avant la limite de contexte, Claude Code la saute et la conversation continue sans compaction. Si la compaction a été déclenchée pour récupérer d'une erreur de limite de contexte déjà retourné par l'API, l'erreur sous-jacente surface et la requête actuelle échoue.

Claude Code rejette les champs `systemMessage` et `continue` d'un hook PreCompact.

<h4 id="precompact-input">
  Entrée PreCompact
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PreCompact reçoivent `trigger` et `custom_instructions`. Pour `manual`, `custom_instructions` contient ce que l'utilisateur passe dans `/compact` et est `null` quand il ne passe rien. Pour `auto`, `custom_instructions` est `null`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": null
}
```

<h3 id="postcompact">
  PostCompact
</h3>

S'exécute après que Claude Code termine une opération de compaction. Utilisez cet événement pour réagir à l'état compacté nouveau, par exemple pour enregistrer le résumé généré ou mettre à jour l'état externe. Claude Code rejette les champs `systemMessage` et `continue` d'un hook PostCompact.

Les mêmes valeurs de matcher s'appliquent que pour `PreCompact` :

| Matcher  | Quand il se déclenche                                                                                                                              |
| :------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| `manual` | Après `/compact`                                                                                                                                   |
| `auto`   | Après la compaction automatique quand la conversation atteint la [fenêtre de compaction automatique](/docs/fr/model-config#set-the-auto-compact-window) |

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

Les hooks PostCompact n'ont pas de contrôle de décision. Ils ne peuvent pas affecter le résultat de la compaction mais peuvent effectuer les tâches de suivi.

<h3 id="premodelswitch">
  PreModelSwitch
</h3>

S'exécute avant que Claude Code applique un changement de modèle que vous ou un client avez demandé. Utilisez-le pour bloquer un changement, exiger une confirmation, ou montrer ce que le changement coûtera avant qu'il se produise.

PreModelSwitch nécessite Claude Code v2.1.251 ou ultérieur. Claude Code l'exécute pour ces requêtes :

* `/model <name>` et le sélecteur `/model`
* Le sélecteur de modèle `Option+P` ou `Alt+P`
* Le paramètre Model dans `/config`
* Activer le [mode rapide](/docs/fr/fast-mode) quand cela change le modèle de la session
* Une requête `set_model`, ou un changement de modèle dans une requête `apply_flag_settings`, d'un hôte [Agent SDK](/docs/fr/agent-sdk/typescript#query-object) ou [Remote Control](/docs/fr/remote-control)

Claude Code n'exécute pas les hooks PreModelSwitch pour les changements qu'il fait de son propre chef, comme un [repli de modèle automatique](/docs/fr/model-config#automatic-model-fallback) ou la restauration du modèle quand vous reprenez une session. Ces changements atteignent [PostModelSwitch](#postmodelswitch) uniquement.

Claude Code compare le matcher contre le nom canonique du modèle vers lequel la session change, en ignorant tout suffixe `[1m]`. Un alias comme `opus`, un ID de modèle daté, et un ID spécifique au fournisseur comme un ID de modèle Amazon Bedrock correspondent tous au seul nom canonique auquel ils se résolvent, donc `claude-opus-5` couvre chaque orthographe d'Opus 5.

Quand Claude Code ne peut pas déterminer un nom canonique pour la cible, par exemple un ID de modèle personnalisé que seule votre [passerelle LLM](/docs/fr/llm-gateway) connaît, il exécute chaque hook PreModelSwitch indépendamment du matcher. Un hook qui bloque devrait donc vérifier `to_model` de son entrée plutôt que de compter sur le matcher seul.

Écrivez le matcher comme un nom exact, une liste séparée par `|` comme `claude-opus-4-6|claude-opus-5`, ou une expression régulière comme `.*opus.*`. Cet exemple utilise un matcher de nom exact et vérifie également `to_model` de l'entrée du hook, donc il refuse un changement vers Opus 4.6 en quittant avec le code 2 et laisse n'importe quelle autre cible passer :

<Tabs>
  <Tab title="macOS/Linux">
    La commande vérifie `to_model` avec `jq` :

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "jq -e '.to_model | test(\"opus-4-6\")' > /dev/null && { echo 'Opus 4.6 is retired for this project. Use a newer model.' >&2; exit 2; }; exit 0"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Enregistrez un hook de commande qui exécute un script via PowerShell :

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-opus-46.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Enregistrez ce script dans `.claude/hooks/block-opus-46.ps1` dans votre projet :

    ```powershell theme={null}
    $hookInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    if ($hookInput.to_model -match 'opus-4-6') {
      [Console]::Error.WriteLine('Opus 4.6 is retired for this project. Use a newer model.')
      exit 2
    }
    exit 0
    ```
  </Tab>
</Tabs>

Pour confirmer que le hook fonctionne, exécutez `/model claude-opus-4-6` à partir d'une session exécutant un modèle différent. Claude Code garde le modèle actuel et rapporte qu'un hook PreModelSwitch a bloqué le changement, avec votre message comme raison.

<h4 id="premodelswitch-input">
  Entrée PreModelSwitch
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks PreModelSwitch reçoivent les champs de ce tableau. Les cinq derniers décrivent ce que renvoyer la conversation au nouveau modèle coûte, pour qu'un hook puisse montrer ce chiffre avant que le changement se produise.

| Champ                       | Type             | Description                                                                                                                                                                                                                                                                                                              |
| :-------------------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `from_model`                | string           | ID du modèle que le changement change                                                                                                                                                                                                                                                                                    |
| `to_model`                  | string           | ID du modèle que le changement change vers. Le matcher compare contre le nom canonique de ce modèle                                                                                                                                                                                                                      |
| `requested_model`           | string ou `null` | Le modèle que la requête a nommé : un alias comme `opus`, un ID de modèle complet, ou `null` quand la requête était pour le modèle par défaut                                                                                                                                                                            |
| `source`                    | string           | D'où provient la requête : `"command"` pour `/model <name>`, le paramètre Model dans `/config`, ou l'activation du mode rapide ; `"picker"` pour un sélecteur de modèle ; `"sdk"` pour une requête `set_model`, ou un changement de modèle dans une requête `apply_flag_settings`, d'un hôte Agent SDK ou Remote Control |
| `context_tokens`            | number           | Tokens que la requête suivante renvoie comme son invite : les tokens d'entrée, de lecture de cache, de création de cache, et de sortie de la dernière réponse dans la conversation principale, combinés. `0` avant la première réponse                                                                                   |
| `prompt_cache_warm`         | boolean          | Si le cache d'invite du modèle actuel est probablement toujours chaud, ce qui signifie que le changement le perd                                                                                                                                                                                                         |
| `cache_ttl`                 | string           | [Durée de vie du cache d'invite](/docs/fr/prompt-caching#cache-lifetime) que Claude Code demande pour cette session : `"5m"` ou `"1h"`                                                                                                                                                                                        |
| `estimated_cache_write_usd` | number           | Coût estimé en dollars US de l'écriture de `context_tokens` dans le cache d'invite sur `to_model` au taux `cache_ttl`, excluant la réponse suivante. Le serveur peut ne pas avoir besoin de recacher le contexte entier, donc traitez-le comme une estimation                                                            |
| `pricing`                   | string           | Comment Claude Code a tarifé `estimated_cache_write_usd` : `"configured"` à vos propres taux d'organisation quand elle les a configurés, `"catalog"` au prix catalogue, ou `"default"` quand `to_model` n'a pas de prix connu et Claude Code a supposé un taux par défaut                                                |

Cet exemple montre l'entrée pour `/model opus` dans une session exécutant Sonnet 5 :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreModelSwitch",
  "from_model": "claude-sonnet-5",
  "to_model": "claude-opus-5",
  "requested_model": "opus",
  "source": "command",
  "context_tokens": 182340,
  "prompt_cache_warm": true,
  "cache_ttl": "5m",
  "estimated_cache_write_usd": 1.1396,
  "pricing": "catalog"
}
```

<h4 id="premodelswitch-decision-control">
  Contrôle de décision PreModelSwitch
</h4>

Les hooks `PreModelSwitch` peuvent annuler le changement, demander à l'utilisateur de le confirmer, ou le laisser procéder. Le code de sortie 2 ou un `decision: "block"` de haut niveau annule le changement.

Pour un contrôle plus fin, retournez `permissionDecision` et `permissionDecisionReason` dans un objet `hookSpecificOutput`, comme sur [PreToolUse](#pretooluse-decision-control). `PreModelSwitch` accepte `"allow"`, `"deny"`, et `"ask"`. Il n'accepte pas `"defer"`, `updatedInput`, ou `additionalContext`. Le tableau ci-dessous décrit les deux champs :

| Champ                      | Description                                                                                                                                                                                                                   |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` procède et ignore la [confirmation que Claude Code montre pendant que le cache d'invite est chaud](/docs/fr/prompt-caching#switching-models). `"deny"` annule le changement. `"ask"` invite l'utilisateur à le confirmer |
| `permissionDecisionReason` | Pour `"deny"`, montré à l'utilisateur comme la raison du blocage du changement, ou retourné comme l'erreur pour une requête `set_model`. Pour `"ask"`, montré dans l'invite de confirmation. Ignoré pour `"allow"`            |

Seul `/model` dans une session interactive peut montrer l'invite `"ask"`. Sur chaque autre surface, y compris le mode non-interactif avec le drapeau `-p`, `/config`, et les requêtes `set_model`, Claude Code traite `"ask"` comme un refus.

Cet exemple demande à l'utilisateur de confirmer et cite le nombre de tokens de `context_tokens` :

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreModelSwitch",
    "permissionDecision": "ask",
    "permissionDecisionReason": "Switching now re-sends about 180k tokens to the new model. Continue?"
  }
}
```

Quand plusieurs hooks PreModelSwitch retournent des décisions différentes, la priorité est `deny` > `ask` > `allow`.

Claude Code montre à l'utilisateur tout `systemMessage` que votre hook retourne indépendamment de la décision, pour qu'un hook de rapport de coûts puisse retourner `{"systemMessage": "..."}` et quitter 0.

Un hook PreModelSwitch qui ne répond pas avant son délai d'expiration bloque le changement. Sur [PreToolUse](#timeouts), par contraste, un hook de commande qui expire laisse l'appel d'outil continuer. Le délai d'expiration par défaut pour cet événement est 30 secondes. `PreModelSwitch` exécute uniquement les hooks `command`, `http`, et `mcp_tool`, donc les défauts `prompt` et `agent` ne s'appliquent pas.

Un hook qui quitte avec un code autre que 0 ou 2 et n'imprime pas de décision JSON ne bloque pas : Claude Code montre son stderr et applique le changement, comme décrit sous [Autres codes de sortie](#other-exit-codes).

<h3 id="postmodelswitch">
  PostModelSwitch
</h3>

S'exécute après que le modèle de la session change. Utilisez-le pour donner à Claude des conseils spécifiques au modèle sans éditer chaque CLAUDE.md, par exemple une instruction à l'échelle de l'organisation qui s'applique sur certains modèles.

PostModelSwitch nécessite Claude Code v2.1.251 ou ultérieur. Il ne peut pas bloquer, parce que le modèle a déjà changé. Claude Code exécute les hooks PostModelSwitch après n'importe lequel de ces changements :

* Un changement que vous ou un client avez demandé
* Un [repli de modèle automatique](/docs/fr/model-config#automatic-model-fallback), qui change le modèle de la session
* Un paramètre comme [`opusplan`](/docs/fr/model-config#opusplan-model-setting) entrant ou quittant le mode plan
* Claude Code restaure le modèle quand vous reprenez une session

Claude Code n'exécute pas les hooks PostModelSwitch quand un modèle d'une [chaîne de modèle de repli](/docs/fr/model-config#fallback-model-chains) sert un tour, parce que cette substitution dure un tour et laisse le modèle de la session inchangé.

Le matcher suit les mêmes règles que [PreModelSwitch](#premodelswitch) : Claude Code le compare contre le nom canonique du modèle vers lequel la session a changé.

Cet exemple ajoute des conseils chaque fois que le modèle de la session change vers n'importe quel modèle Opus :

```json theme={null}
{
  "hooks": {
    "PostModelSwitch": [
      {
        "matcher": ".*opus.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'On Opus, delegate implementation work to subagents and keep this conversation for planning and review.'"
          }
        ]
      }
    ]
  }
}
```

Pour confirmer que le hook fonctionne, changez vers un modèle Opus à partir d'une session exécutant un modèle différent, par exemple exécutez `/model opus` à partir d'une session Sonnet, puis demandez à Claude quels conseils il a sur le modèle actuel.

<h4 id="postmodelswitch-input">
  Entrée PostModelSwitch
</h4>

Les hooks PostModelSwitch reçoivent les mêmes champs que [PreModelSwitch](#premodelswitch-input), avec `hook_event_name` défini à `"PostModelSwitch"` et deux valeurs `source` supplémentaires : `"auto"` pour un repli automatique ou un autre changement que Claude Code a fait de son propre chef, et `"resume"` pour le modèle restauré quand vous reprenez une session.

`requested_model` est `null` quand `source` est `"auto"`. Quand `source` est `"resume"`, c'est le paramètre de modèle sauvegardé que Claude Code a restauré.

<h4 id="postmodelswitch-decision-control">
  Contrôle de décision PostModelSwitch
</h4>

Claude Code prend votre sortie standard [texte brut](#exit-code-0) du hook sur la sortie 0, ou `additionalContext` de la sortie JSON, et la livre à Claude avec la requête suivante après le changement. En plus des [champs de sortie JSON](#json-output) disponibles pour tous les hooks, vous pouvez retourner :

| Champ               | Description                                                                                                                    |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Chaîne ajoutée au contexte de Claude avec la requête suivante. Voir [Ajouter du contexte pour Claude](#add-context-for-claude) |

Si le hook n'a pas fini dans les cinq secondes après que vous envoyiez la requête suivante, Claude Code envoie cette requête sans la sortie et l'attache à la requête suivante à la place. Si le modèle change plusieurs fois avant la requête suivante, Claude Code livre uniquement la sortie pour le modèle cible du dernier changement.

<h3 id="sessionend">
  SessionEnd
</h3>

S'exécute quand une session Claude Code se termine. Utile pour les tâches de nettoyage, l'enregistrement des statistiques de session, ou la sauvegarde de l'état de la session. Supporte les matchers pour filtrer par raison de sortie.

Le champ `reason` dans l'entrée du hook indique pourquoi la session s'est terminée :

| Raison                        | Description                                                                                     |
| :---------------------------- | :---------------------------------------------------------------------------------------------- |
| `clear`                       | Session effacée avec la commande `/clear`                                                       |
| `resume`                      | Session changée via `/resume` interactif                                                        |
| `logout`                      | L'utilisateur s'est déconnecté                                                                  |
| `prompt_input_exit`           | L'utilisateur a quitté pendant que l'entrée d'invite était visible                              |
| `other`                       | Autres raisons de sortie                                                                        |
| `bypass_permissions_disabled` | Supprimé dans v2.1.234 ; Claude Code ne l'envoie pas. Supprimez-le de vos matchers `SessionEnd` |

<h4 id="sessionend-input">
  Entrée SessionEnd
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks SessionEnd reçoivent un champ `reason` indiquant pourquoi la session s'est terminée. Voir le [tableau de raison](#sessionend) ci-dessus pour toutes les valeurs.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

Les hooks SessionEnd n'ont pas de contrôle de décision. Ils ne peuvent pas bloquer la terminaison de la session mais peuvent effectuer les tâches de nettoyage. Claude Code rejette leurs [champs de sortie JSON](#json-output), comme `systemMessage`.

Les hooks SessionEnd ont un délai d'expiration par défaut de 1,5 secondes. Il s'applique quand vous quittez, exécutez `/clear`, ou changez de sessions avec `/resume` interactif. Vous pouvez donner à un hook plus de temps de deux façons :

* **`timeout` par hook** : définissez `timeout` dans la configuration de ce hook. Le budget global augmente automatiquement pour correspondre au `timeout` par hook le plus élevé dans vos fichiers de paramètres, jusqu'à 60 secondes. Si vous augmentez le budget de cette façon, un hook sans son propre `timeout` garde toujours le défaut. Les délais d'expiration définis sur les hooks fournis par les plugins ne lèvent pas le budget.
* **`CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS`** : définissez cette variable d'environnement en millisecondes pour remplacer le budget explicitement. La valeur que vous définissez devient également le délai d'expiration pour chaque hook sans son propre `timeout`.

Cet exemple définit le budget à 5 secondes :

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

Avant v2.1.268, `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` levait uniquement le budget global, et un hook sans son propre `timeout` était toujours annulé après 1,5 secondes.

<h3 id="elicitation">
  Elicitation
</h3>

S'exécute quand un serveur MCP demande l'entrée utilisateur en cours de tâche. Par défaut, Claude Code montre un dialogue interactif pour que l'utilisateur réponde. Les hooks peuvent intercepter cette requête et répondre par programmation, ignorant entièrement le dialogue.

Le champ matcher correspond au nom du serveur MCP.

<h4 id="elicitation-input">
  Entrée Elicitation
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks Elicitation reçoivent `mcp_server_name`, `message`, et les champs optionnels `mode`, `url`, `elicitation_id`, et `requested_schema`.

Pour l'élicitation en mode formulaire, le cas le plus courant :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
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

Pour l'élicitation en mode URL, utilisée pour l'authentification basée sur navigateur :

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
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

Pour répondre par programmation sans montrer le dialogue, retournez un objet JSON avec `hookSpecificOutput` :

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

| Champ     | Valeurs                       | Description                                                                                  |
| :-------- | :---------------------------- | :------------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Si vous acceptez, refusez, ou annulez la requête                                             |
| `content` | object                        | Valeurs des champs de formulaire à soumettre. Utilisé uniquement quand `action` est `accept` |

Le code de sortie 2 refuse l'élicitation. Claude Code n'affiche votre message stderr nulle part.

Claude Code agit sur `hookSpecificOutput` de la sortie JSON d'un hook Elicitation et rejette `systemMessage` et `continue`.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

S'exécute après qu'un utilisateur réponde à une élicitation MCP. Les hooks peuvent observer, modifier, ou bloquer la réponse avant qu'elle ne soit renvoyée au serveur MCP.

Le champ matcher correspond au nom du serveur MCP.

<h4 id="elicitationresult-input">
  Entrée ElicitationResult
</h4>

En plus des [champs d'entrée communs](#common-input-fields), les hooks ElicitationResult reçoivent `mcp_server_name`, `action`, et les champs optionnels `mode`, `elicitation_id`, et `content`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
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

| Champ     | Valeurs                       | Description                                                                                        |
| :-------- | :---------------------------- | :------------------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Remplace l'action de l'utilisateur                                                                 |
| `content` | object                        | Remplace les valeurs des champs de formulaire. Significatif uniquement quand `action` est `accept` |

Le code de sortie 2 bloque la réponse, changeant l'action effective à `decline`. Claude Code n'affiche votre message stderr nulle part.

Claude Code agit sur `hookSpecificOutput` de la sortie JSON d'un hook ElicitationResult et rejette `systemMessage` et `continue`.

<h2 id="prompt-based-hooks">
  Hooks basés sur des prompts
</h2>

En plus des hooks de commande, HTTP et MCP tool, Claude Code supporte les hooks basés sur des prompts (`type: "prompt"`) qui utilisent un LLM pour évaluer s'il faut autoriser ou bloquer une action, et les hooks d'agent (`type: "agent"`) qui lancent un vérificateur agentique avec accès aux outils. Tous les événements ne supportent pas tous les types de hooks.

Les événements qui supportent les cinq types de hooks (`command`, `http`, `mcp_tool`, `prompt` et `agent`) :

* `PermissionDenied`
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

`PermissionRequest` supporte les hooks `command`, `http`, `mcp_tool` et `prompt` mais pas les hooks `agent`. Si vous configurez un hook d'agent sur cet événement, Claude Code le saute et le flux de permission se poursuit sans changement. Pour autoriser ou refuser à partir d'un hook, retournez l'[objet de décision](#permissionrequest-decision-control) à partir d'un hook de commande ou HTTP.

Les événements qui supportent les hooks `command`, `http` et `mcp_tool` mais pas `prompt` ou `agent` :

* `ConfigChange`
* `CwdChanged`
* `DirectoryAdded`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `MessageDisplay`
* `Notification`
* `PostCompact`
* `PostModelSwitch`
* `PreCompact`
* `PreModelSwitch`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` et `Setup` supportent les hooks `command` et `mcp_tool`, et [les champs des hooks MCP tool](#mcp-tool-hook-fields) décrivent quand leurs hooks `mcp_tool` s'exécutent. Ils ne supportent pas les hooks `http`, `prompt` ou `agent`.

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

Définissez `type` à `"prompt"` et fournissez une chaîne `prompt` au lieu d'une `command`. Utilisez le placeholder `$ARGUMENTS` pour injecter les données d'entrée JSON du hook dans votre texte de prompt.

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

| Champ             | Requis | Description                                                                                                                                                                                                                              |
| :---------------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | oui    | Doit être `"prompt"`                                                                                                                                                                                                                     |
| `prompt`          | oui    | Le texte du prompt à envoyer au LLM. Utilisez `$ARGUMENTS` comme placeholder pour l'entrée JSON du hook. Si `$ARGUMENTS` n'est pas présent, l'entrée JSON est ajoutée au prompt                                                          |
| `model`           | non    | Modèle à utiliser pour l'évaluation. Par défaut un modèle rapide                                                                                                                                                                         |
| `timeout`         | non    | Délai d'expiration en secondes. Par défaut : 30                                                                                                                                                                                          |
| `continueOnBlock` | non    | Sur les événements auxquels elle s'applique, `true` renvoie une raison `ok: false` à Claude et continue au lieu de terminer le tour. Par défaut : `false`. Voir [Schéma de réponse](#response-schema) pour le comportement par événement |

<h3 id="response-schema">
  Schéma de réponse
</h3>

Le LLM doit répondre avec JSON contenant :

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision",
  "impossible": true | false
}
```

| Champ        | Description                                                                                                                                                                                                                                                                       |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ok`         | `true` pour autoriser. Pour `false`, voir le comportement par événement ci-dessous                                                                                                                                                                                                |
| `reason`     | Requis lorsque `ok` est `false`                                                                                                                                                                                                                                                   |
| `impossible` | Optionnel. Le modèle le retourne avec `ok: false` lorsqu'il juge que la condition ne peut jamais être satisfaite. Sur `Stop` et `SubagentStop`, Claude Code laisse alors le tour se terminer au lieu de renvoyer la raison. Les hooks d'agent et les autres événements l'ignorent |

Ce qui se passe sur `ok: false` dépend de l'événement :

* `Stop` et `SubagentStop` : la raison est renvoyée à Claude comme sa prochaine instruction et le tour continue, sauf si la réponse définit également `impossible: true`, auquel cas Claude Code autorise l'arrêt et le tour se termine
* `PreToolUse` : l'appel d'outil est refusé ; par défaut le tour se termine et la raison de refus apparaît dans le chat comme une ligne d'avertissement. Définissez `continueOnBlock: true` pour renvoyer la raison à Claude comme l'erreur de l'outil afin qu'il puisse s'ajuster et continuer, équivalent à un hook de commande avec `permissionDecision: "deny"`. Avant v2.1.210, la raison de refus était renvoyée à Claude comme l'erreur de l'outil et le tour continuait
* `PostToolUse` : par défaut le tour se termine et la raison apparaît dans le chat comme une ligne d'avertissement. Définissez `continueOnBlock: true` pour renvoyer la raison à Claude et continuer le tour à la place
* `PostToolBatch`, `UserPromptSubmit` et `UserPromptExpansion` : le tour se termine et la raison apparaît comme une ligne d'avertissement. Ces événements terminent le tour sur `decision: "block"` indépendamment de `continue`
* `PostToolUseFailure` et `TaskCreated` : la raison est retournée à Claude comme une erreur d'outil et le tour continue, indépendamment de `continueOnBlock`
* `TaskCompleted` : lorsqu'il se déclenche parce qu'une tâche est marquée comme complétée pendant un tour, la raison est retournée à Claude comme une erreur d'outil et le tour continue, indépendamment de `continueOnBlock`. Lorsqu'il se déclenche parce qu'un coéquipier s'arrête, il se comporte comme `TeammateIdle` et arrête le coéquipier par défaut
* `TeammateIdle` : par défaut le coéquipier s'arrête et la raison apparaît comme une ligne d'avertissement. Définissez `continueOnBlock: true` pour renvoyer la raison au coéquipier et le garder actif à la place
* `PermissionRequest` : `ok: false` n'a aucun effet. Pour refuser une approbation d'un hook, utilisez un [hook de commande](#command-hook-fields) retournant `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied` : `ok: false` n'a aucun effet car le refus a déjà eu lieu. La seule sortie que cet événement lit est `hookSpecificOutput.retry`, que les hooks de prompt et d'agent ne peuvent pas définir. Ils s'exécutent sur cet événement, mais leur sortie est ignorée. Utilisez un [hook de commande](#command-hook-fields) pour retourner `retry`

Si vous avez besoin d'un contrôle plus fin sur un événement quelconque, utilisez un [hook de commande](#command-hook-fields) avec les champs par événement décrits dans [Contrôle de décision](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Vérifier plusieurs conditions avant d'arrêter
</h3>

Ce hook `Stop` utilise un prompt détaillé pour vérifier trois conditions avant d'autoriser Claude à s'arrêter. Les hooks `SubagentStop` utilisent le même format pour évaluer si un [subagent](/docs/fr/sub-agents) doit s'arrêter. Si le modèle retourne `"ok": false` parce que la condition n'est pas encore satisfaite, Claude continue de travailler avec la raison fournie comme sa prochaine instruction :

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

Les hooks basés sur des agents (`type: "agent"`) sont comme les hooks basés sur des prompts mais avec accès aux outils multi-tours. Au lieu d'un seul appel LLM, un hook d'agent lance un subagent qui peut lire des fichiers, rechercher du code et inspecter la codebase pour vérifier les conditions. Les hooks d'agent supportent les mêmes événements que les [hooks basés sur des prompts](#prompt-based-hooks), sauf `PermissionRequest`.

<h3 id="how-agent-hooks-work">
  Comment fonctionnent les hooks d'agent
</h3>

Lorsqu'un hook d'agent se déclenche :

1. Claude Code lance un subagent avec votre prompt et l'entrée JSON du hook
2. Le subagent peut utiliser des outils comme Read, Grep et Glob pour enquêter
3. Après jusqu'à 50 tours, le subagent retourne une décision structurée `{ "ok": true/false }`
4. Claude Code autorise l'action si `ok` est `true`. Si `ok` est `false`, Claude Code traite le blocage de la même manière qu'un hook de prompt avec `continueOnBlock: true` sur cet événement, comme indiqué sous [Schéma de réponse](#response-schema)

Les hooks d'agent sont utiles lorsque la vérification nécessite d'inspecter les fichiers réels ou la sortie des tests, pas seulement d'évaluer les données d'entrée du hook seules.

<h3 id="agent-hook-configuration">
  Configuration des hooks d'agent
</h3>

Définissez `type` à `"agent"` et fournissez une chaîne `prompt`, en utilisant `$ARGUMENTS` comme placeholder pour l'entrée JSON du hook. Les champs de configuration sont les mêmes que les [hooks de prompt](#prompt-hook-configuration), sauf que les hooks d'agent ont un délai d'expiration par défaut plus long de 60 secondes et aucun champ `continueOnBlock`.

Le schéma de réponse est `{ "ok": true }` pour autoriser ou `{ "ok": false, "reason": "..." }` pour bloquer. Sur `ok: false`, Claude Code traite un hook d'agent de la même manière qu'il traite un [hook de prompt avec `continueOnBlock: true`](#response-schema) sur le même événement ; les hooks d'agent n'ont pas de champ `continueOnBlock` et ne supportent pas le champ `impossible` du hook de prompt.

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

Ce hook exécute un script de test après chaque appel d'outil `Write`. Claude continue de travailler immédiatement tandis que `run-tests.sh` s'exécute. Lorsque le script se termine, sa sortie est livrée au tour de conversation suivant :

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
            "async": true
          }
        ]
      }
    ]
  }
}
```

Une fois qu'un hook asynchrone s'exécute en arrière-plan, Claude Code n'applique pas de `timeout` sur celui-ci. Claude Code applique toujours le `timeout` sur un hook que vous exécutez avec `asyncRewake`.

Claude Code livre les résultats d'un hook asynchrone uniquement pendant que la session s'exécute :

* En [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`, Claude Code tue tout hook asynchrone encore en cours d'exécution lors du démontage et le finalise avec le résultat `cancelled`
* Si le travail de votre hook doit survivre à une session `claude -p`, démarrez un processus complètement détaché à partir de celui-ci

<h3 id="how-async-hooks-execute">
  Comment les hooks asynchrones s'exécutent
</h3>

Lorsqu'un hook asynchrone se déclenche, Claude Code démarre le processus du hook et continue immédiatement sans attendre qu'il se termine. Le hook reçoit la même entrée JSON via stdin qu'un hook synchrone.

Après la sortie du processus en arrière-plan, Claude Code livre les champs `additionalContext` et `systemMessage` de la réponse JSON du hook à Claude au tour de conversation suivant. Contrairement au `systemMessage` d'un hook synchrone, aucun de ces champs ne vous est montré.

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
            "async": true
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

Les hooks asynchrones ont des contraintes supplémentaires par rapport aux hooks synchrones :

* La sortie du hook est livrée au tour de conversation suivant. Si la session est inactive, la réponse attend jusqu'à la prochaine interaction utilisateur. Exception : un hook `asyncRewake` qui quitte avec le code 2 réveille Claude immédiatement même lorsque la session est inactive.
* Chaque exécution crée un processus en arrière-plan séparé. Il n'y a pas de déduplication sur plusieurs déclenchements du même hook asynchrone.

<h2 id="security-considerations">
  Considérations de sécurité
</h2>

<h3 id="disclaimer">
  Avertissement
</h3>

<Warning>
  Les hooks de commande exécutent les commandes shell avec vos permissions utilisateur complètes. Ils peuvent modifier, supprimer ou accéder à tous les fichiers auxquels votre compte utilisateur peut accéder. Examinez et testez toutes les commandes de hook avant de les ajouter à votre configuration.
</Warning>

<h3 id="workspace-trust">
  Confiance de l'espace de travail
</h3>

Claude Code vérifie la confiance de l'espace de travail avant d'exécuter tout hook à partir d'un fichier de paramètres. Ce qui compte comme approuvé dépend du type de session :

* **Session interactive** : Claude Code retient les hooks de tous les fichiers de paramètres, y compris votre propre `~/.claude/settings.json`, jusqu'à ce que vous acceptiez le [dialogue de confiance de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) pour le dossier, ou pour un répertoire parent dont la confiance s'étend à celui-ci
* **Session `-p` ou SDK** : Claude Code n'affiche jamais le dialogue et traite le dossier comme approuvé, donc les hooks validés dans le `.claude/settings.json` d'un référentiel s'exécutent dans un dossier que vous n'avez jamais approuvé

Avant de scripter `claude -p` sur un référentiel que vous n'avez pas écrit, examinez ses fichiers de paramètres `.claude/`, commencez par [`--bare`](/docs/fr/headless#start-faster-with-bare-mode), ou [désactivez les hooks pour cette exécution](#disable-or-remove-hooks) avec `--settings '{"disableAllHooks": true}'`. Les hooks de frontmatter dans un sous-agent de projet suivent une règle plus stricte que les hooks de fichier de paramètres. [Ce qui s'exécute avant que vous approuviez un dossier](/docs/fr/permissions#what-runs-before-you-trust-a-folder) énumère chaque type de contenu de référentiel par type de session.

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

Sur Windows, vous pouvez exécuter les hooks individuels dans PowerShell en définissant `"shell": "powershell"` sur un hook de commande. Claude Code détecte automatiquement `pwsh.exe`, l'exécutable PowerShell 7 et versions ultérieures, et bascule vers `powershell.exe` pour Windows PowerShell 5.1.

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

Les détails d'exécution des hooks sont écrits dans le fichier journal de débogage. Démarrez Claude Code avec `claude --debug-file <path>` pour écrire le journal à un emplacement connu, ou exécutez `claude --debug` et lisez le journal à `~/.claude/debug/<session-id>.txt`. Le drapeau `--debug` n'imprime pas sur le terminal.

Par exemple, un hook `PostToolUse` sur `Write` dont la commande affiche `hook-ran` produit des entrées comme :

```text theme={null}
2026-07-19T02:03:24.382Z [DEBUG] Hook output does not start with {, treating as plain text
2026-07-19T02:03:24.382Z [DEBUG] "Hook PostToolUse:Write (PostToolUse) success:\nhook-ran"
```

Pour plus de détails granulaires sur la correspondance des hooks, définissez `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` pour voir des lignes de journal supplémentaires telles que les comptes de matcher de hook et la correspondance de requête.

Pour dépanner les problèmes courants comme les hooks qui ne se déclenchent pas, les hooks Stop qui continuent à bloquer, ou les erreurs de configuration, consultez [Limitations et dépannage](/docs/fr/hooks-guide#limitations-and-troubleshooting) dans le guide. Pour une procédure de diagnostic plus large couvrant `/context`, `/doctor` et la précédence des paramètres, consultez [Déboguer votre configuration](/docs/fr/debug-your-config).
