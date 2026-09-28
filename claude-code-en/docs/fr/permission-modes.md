> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Choisir un mode de permission

> Contrôlez si Claude demande une approbation avant d'agir. Basculez entre les modes avec Maj+Tab dans la CLI, l'indicateur de mode dans VS Code, ou le sélecteur de mode dans Desktop.

Un mode de permission définit les actions que Claude Code peut effectuer dans une session sans vous demander d'abord. En mode Manuel, Claude Code s'arrête et vous demande avant la plupart des actions qui modifient des fichiers, exécutent des commandes shell ou accèdent au réseau. En [mode auto](#eliminate-prompts-with-auto-mode), un second modèle, le classificateur, examine les actions à votre place ; [comment le classificateur évalue les actions](#how-the-classifier-evaluates-actions) énumère les actions qu'il examine et celles qu'il ignore.

Sur les plans Pro, Max et Team, le mode de permission de démarrage intégré est le mode auto. [Quel mode une session démarre](#which-mode-a-session-starts-in) couvre les surfaces et les paramètres qui changent le mode de permission de démarrage. Vous pouvez également modifier le mode de permission d'une session en cours à tout moment.

<h2 id="available-modes">
  Modes disponibles
</h2>

Chaque mode fait un compromis différent entre la commodité et la supervision. Le tableau ci-dessous montre ce que Claude peut faire sans invite de permission dans chaque mode. Le mode Manuel apparaît sous sa valeur de configuration, `default`.

| Mode                                                                | Ce qui s'exécute sans demander                                                                                                   | Idéal pour                                          |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| `default`                                                           | Lectures uniquement                                                                                                              | Examiner chaque action vous-même, travail sensible  |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Lectures, modifications de fichiers et commandes courantes du système de fichiers (`mkdir`, `touch`, `mv`, `cp`, etc.)           | Itération sur le code que vous examinez             |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Lectures, plus commandes approuvées par le classificateur quand le [mode auto](#eliminate-prompts-with-auto-mode) est disponible | Explorer une base de code avant de la modifier      |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Tout, avec des vérifications de sécurité en arrière-plan                                                                         | Tâches longues, réduction de la fatigue des invites |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Lectures et outils pré-approuvés ; tout ce qui déclencherait une invite est refusé                                               | CI verrouillé et scripts                            |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Tout                                                                                                                             | Conteneurs et machines virtuelles isolés uniquement |

Le mode qui examine chaque action s'appelle **Manual** dans la CLI, dans `claude --help`, dans les extensions VS Code et JetBrains, et dans l'application de bureau. Sa valeur de configuration est `default`, ce que les hooks et les intégrations SDK utilisent. La CLI accepte `manual` comme alias partout où vous tapez la valeur, par exemple `claude --permission-mode manual` ou `"defaultMode": "manual"`. L'étiquette Manual et l'alias `manual` nécessitent Claude Code v2.1.200 ou version ultérieure. L'étiquette de l'application de bureau ne dépend pas de votre version CLI.

Les écritures vers les [chemins protégés](#protected-paths) ne sont jamais auto-approuvées sauf en mode `bypassPermissions` et dans les sessions en mode plan où les permissions de contournement sont disponibles, ce qui signifie les sessions démarrées d'une manière qui [place `bypassPermissions` dans le cycle de mode](#switch-permission-modes).

Les modes définissent la ligne de base. Superposez les [règles de permission](/docs/fr/permissions#manage-permissions) sur le dessus pour pré-approuver ou bloquer des outils spécifiques. Les règles de refus bloquent dans tous les modes, y compris `bypassPermissions`. Les règles de refus et de demande ne s'appliquent pas à [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior) tant que Claude a au moins un autre outil qu'il peut appeler. Les règles d'autorisation n'ont aucun effet en mode `bypassPermissions`.

<h3 id="actions-no-mode-auto-approves">
  Actions qu'aucun mode n'auto-approuve
</h3>

Claude Code n'auto-approuve pas les éléments suivants dans aucun mode, y compris `bypassPermissions`. Chaque point renvoie à la section qui dit ce qui se passe à la place dans chaque mode :

* Outils correspondant à une [règle ask](/docs/fr/permissions#manage-permissions) explicite
* Outils de connecteur que votre organisation a [définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), dans les sessions où ce paramètre atteint Claude Code
* Outils qui nécessitent une interaction utilisateur : l'outil intégré `AskUserQuestion` et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool)
* Les suppressions `rm` et `rmdir` ciblant un [chemin critique](#critical-paths), qu'aucune règle allow ou hook `PreToolUse` `"allow"` n'approuve
* Les [protections de messagerie inter-sessions](#skip-all-checks-with-bypasspermissions-mode)
* Les lectures en dehors des répertoires de travail tandis que [`permissions.blockReadsOutsideWorkingDirectories`](/docs/fr/settings-reference#permissions-blockreadsoutsideworkingdirectories) est activé : les commandes Bash reconnues de lecture de fichiers et tout [retry non sandboxé](/docs/fr/sandboxing#the-unsandboxed-retry-escape-hatch) qui nécessite une approbation pour s'exécuter en dehors du sandbox même en mode auto et mode `bypassPermissions`. Nécessite Claude Code v2.1.257 ou version ultérieure.

  Une commande que l'analyseur de shell ne peut pas tracer, comme une qui change de répertoire plus d'une fois ou exécute un sous-shell, demande de la même manière même quand elle ne nomme aucun chemin extérieur. Cette invite ne s'applique pas quand la commande s'exécute dans le [sandbox](/docs/fr/sandboxing) et le sandbox applique le bloc.

<h2 id="common-setups">
  Configurations courantes
</h2>

Les modes de permission décident si Claude demande une confirmation avant une action, et le [sandbox Bash](/docs/fr/sandboxing) et les [limites d'isolation](/docs/fr/sandbox-environments) externes décident ce qu'une action peut atteindre une fois qu'elle s'exécute. Chaque ligne ci-dessous associe un objectif aux drapeaux ou paramètres qui vous y mènent et à l'isolation nécessaire, comme point de départ. [Les modes disponibles](#available-modes) énumère ce qui s'exécute sans invite dans chaque mode.

| Vous voulez                                                          | Commencez par                                                                                                                                                                 | Isolation nécessaire                                                                                                                                                                                                   | Notes                                                                                                                                                                                                                                                                                           |
| :------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Examiner chaque action vous-même                                     | Mode manuel : `claude --permission-mode default`                                                                                                                              | Aucune                                                                                                                                                                                                                 | Travail sensible, code non familier                                                                                                                                                                                                                                                             |
| Itérer localement avec moins d'invites, sans classificateur          | Mode manuel plus le sandbox Bash en [mode auto-allow](/docs/fr/sandboxing#sandbox-modes) : `claude --permission-mode default`, puis exécutez `/sandbox` et sélectionnez auto-allow | Le sandbox Bash intégré, sur macOS, Linux et WSL2                                                                                                                                                                      | Les règles de refus s'appliquent toujours, et les règles d'ask qui nomment une commande, comme `Bash(git push *)`, invitent toujours. Pour activer le sandbox à partir d'un fichier de paramètres à la place, définissez [`sandbox.enabled`](/docs/fr/settings-reference#sandbox-enabled) sur `true` |
| Explorer avant de modifier quoi que ce soit                          | `claude --permission-mode plan`                                                                                                                                               | Aucune                                                                                                                                                                                                                 | Claude Code bloque les modifications jusqu'à ce que vous [approuviez un plan](#review-and-approve-a-plan)                                                                                                                                                                                       |
| Travailler en mode automatique sans intervention                     | `claude --permission-mode auto`, le [mode de permission de démarrage intégré](#which-mode-a-session-starts-in) sur Pro, Max et Team                                           | Aucune ; un sandbox ou un conteneur ajoute une défense en profondeur                                                                                                                                                   | Nécessite un [modèle pris en charge](#eliminate-prompts-with-auto-mode), et votre organisation peut [désactiver le mode auto](#eliminate-prompts-with-auto-mode)                                                                                                                                |
| Exécuter en CI avec une liste d'autorisation exacte                  | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                                                                             | Aucune au-delà de ce que votre exécuteur CI fournit                                                                                                                                                                    | Les [sessions cloud](/docs/fr/claude-code-on-the-web) ignorent `dontAsk` des fichiers de paramètres                                                                                                                                                                                                  |
| Exécuter complètement sans surveillance à l'intérieur d'un conteneur | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                                                                         | Obligatoire : un conteneur, une VM ou le [runtime sandbox](/docs/fr/sandbox-environments#sandbox-runtime) ; sur Linux et macOS, exécutez-le en tant qu'[utilisateur non-root](#skip-all-checks-with-bypasspermissions-mode) | Les sessions cloud ignorent ce mode des fichiers de paramètres. Dans cette exécution `-p`, les [quelques appels qui inviteraient toujours](#skip-all-checks-with-bypasspermissions-mode) sont refusés à la place                                                                                |

Le sandbox Bash et le mode auto fonctionnent indépendamment et se combinent, avec les exceptions énumérées sous [Sandbox modes](/docs/fr/sandboxing#sandbox-modes). Pour l'interaction complète, consultez [How sandboxing relates to permissions and permission modes](/docs/fr/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes) et [How isolation relates to permission modes](/docs/fr/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="which-mode-a-session-starts-in">
  Quel mode une session démarre
</h2>

Quand vous démarrez une nouvelle session dans un terminal, Claude Code prend le mode de permission du premier de ces éléments qui s'applique :

1. Le drapeau `--permission-mode`, ou `--dangerously-skip-permissions`

2. `permissions.defaultMode` dans un [fichier de paramètres](/docs/fr/settings#where-settings-live)

   Si vous définissez `"auto"` dans `.claude/settings.json` ou `.claude/settings.local.json`, la valeur ne prend pas effet, et Claude Code utilise alors la valeur par défaut intégrée plutôt qu'un `defaultMode` de `~/.claude/settings.json`. Si vous définissez `"bypassPermissions"` dans ces deux fichiers, cela ne prend pas effet non plus, et la session démarre en mode Manuel. Les autres valeurs s'appliquent à partir de n'importe quel fichier de paramètres.

3. La valeur par défaut intégrée

Les conversations que l'extension VS Code démarre suivent la propre liste de l'extension dans [Basculer les modes de permission](#switch-permission-modes). Pour le mode de permission dans lequel Claude Code démarre une session reprise, voir [mode de permission à la reprise](/docs/fr/sessions#permission-mode-on-resume).

La valeur par défaut `auto` intégrée nécessite Claude Code v2.1.228 ou version ultérieure sur macOS, Linux et WSL, et v2.1.233 ou version ultérieure sur Windows natif. Sur les versions antérieures, la valeur par défaut intégrée est Manuel.

La valeur par défaut intégrée dépend de la façon dont vous exécutez Claude Code, de votre plan et de la possibilité pour Claude Code de récupérer ses drapeaux de fonctionnalité. La première ligne qui correspond à votre session s'applique. Le tableau couvre les sessions que vous démarrez dans un terminal ou via l'extension VS Code ; pour l'application de bureau et claude.ai, voir les onglets Desktop et Web dans [Basculer les modes de permission](#switch-permission-modes).

| Comment vous exécutez Claude Code                                                                                                                                                                                                                                           | Mode de permission de démarrage intégré |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------- |
| Un fichier de paramètres définit `disableAutoMode` sur `"disable"`                                                                                                                                                                                                          | `default`                               |
| La [récupération des drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching) est désactivée                                                                                                                                                      | `default`                               |
| Votre [première session après l'installation de Claude Code ou la mise à niveau](/docs/fr/env-vars#first-session-after-an-install-or-upgrade) vers une version qui ajoute cette valeur par défaut, sauf après une installation propre, Claude Code récupère les drapeaux à temps | `default`                               |
| `claude -p` ou le [SDK Agent](/docs/fr/agent-sdk/permissions)                                                                                                                                                                                                                    | `default`                               |
| Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry, [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws), ou une session [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectée                                                         | `default`                               |
| Un plan Pro, Max ou Team, dans un terminal ou via l'[extension VS Code](/docs/fr/vs-code)                                                                                                                                                                                        | `auto`                                  |
| Un plan Enterprise ou une clé API Claude Console                                                                                                                                                                                                                            | `default`                               |

Quand la récupération des drapeaux de fonctionnalité est désactivée, ou dans une [première session après une installation ou une mise à niveau](/docs/fr/env-vars#first-session-after-an-install-or-upgrade) où les drapeaux ne sont pas encore arrivés, l'extension VS Code ignore tous les fichiers de paramètres lors du choix du mode de permission de démarrage.

Quand le drapeau, un fichier de paramètres ou la valeur par défaut intégrée sélectionne `auto` mais que le mode auto n'est pas disponible pour la session, Claude Code démarre la session en mode Manuel à la place. Le mode auto est indisponible quand la session ne répond pas aux [exigences de disponibilité](#eliminate-prompts-with-auto-mode), comme un fichier de paramètres le désactivant ou un modèle qui ne le supporte pas, ou quand Anthropic l'a temporairement désactivé côté serveur.

La première fois que la valeur par défaut intégrée démarre l'une de vos sessions en mode auto, Claude Code affiche un avis qui renvoie à cette page :

* Dans un terminal, une fois, en haut de la session
* Dans l'extension VS Code, sous forme de carte sur l'écran de nouvelle conversation qui reste jusqu'à ce que vous la fermiez

Sur les plans Pro, Max et Team, si votre `~/.claude/settings.json` définit un `defaultMode` autre que `auto` et qu'aucun autre fichier de paramètres n'en définit un, vos sessions continuent à démarrer dans ce mode. Claude Code demande une fois, dans le terminal ou dans l'extension VS Code, si vous souhaitez modifier le paramètre en mode auto. Si vous refusez, votre paramètre reste tel quel.

<h3 id="start-in-a-different-mode">
  Démarrer dans un mode de permission différent
</h3>

Vous pouvez définir le mode de permission de démarrage pour une session, ou comme valeur par défaut pour chaque session sur une machine, dans un projet ou dans une organisation. Quand plus d'un fichier de paramètres définit `permissions.defaultMode`, la [précédence des paramètres](/docs/fr/settings#settings-precedence) décide, donc une valeur de projet ou gérée surclasse `~/.claude/settings.json`. Pour modifier le mode de permission d'une session déjà en cours, voir [Basculer les modes de permission](#switch-permission-modes).

| Pour définir le mode de permission de démarrage pour           | Faites ceci                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Une session que vous êtes sur le point de démarrer             | Passez le mode de permission en tant que drapeau, par exemple `claude --permission-mode default`                                                                                                                                                                                                                                                                                                                                                       |
| Chaque session de terminal que vous démarrez sur cette machine | Définissez `permissions.defaultMode` dans `~/.claude/settings.json`. Pour ce que l'extension VS Code lit, voir [Basculer les modes de permission](#switch-permission-modes)                                                                                                                                                                                                                                                                            |
| Chaque session de terminal que vous démarrez dans un projet    | Définissez `permissions.defaultMode` dans le `.claude/settings.json` du projet. Les sessions que vous démarrez dans un terminal honorent chaque valeur sauf `auto` et `bypassPermissions` ; les sessions que l'extension VS Code démarre ne lisent pas les paramètres du projet pour le mode de permission de démarrage                                                                                                                                |
| Chaque session de terminal dans votre organisation             | Définissez `permissions.defaultMode` dans les [paramètres gérés](/docs/fr/managed-settings). Les sessions de terminal démarrent dans ce mode et les gens peuvent toujours basculer vers le mode auto ; pour ce que l'extension VS Code lit, voir [Basculer les modes de permission](#switch-permission-modes). Pour supprimer le mode auto afin que personne ne puisse le sélectionner, définissez `permissions.disableAutoMode` sur `"disable"` à la place |

Cet exemple fait que chaque session de terminal sur votre machine démarre en mode Manuel, dont la valeur de configuration est `default`. Enregistrez-le dans `~/.claude/settings.json` :

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

La session suivante que vous démarrez affiche `⏸ manual mode on` dans la barre d'état.

<h2 id="switch-permission-modes">
  Basculer les modes de permission
</h2>

Chaque interface a son propre contrôle pour basculer les modes de permission pendant une session et sa propre façon de choisir le mode de permission que les nouvelles sessions démarrent. Sélectionnez votre interface pour voir ses contrôles.

<Tabs>
  <Tab title="CLI">
    **En cours de session** : appuyez sur `Shift+Tab` pour parcourir les modes de permission. À partir de `auto`, le premier appui bascule vers `default`, et le cycle s'exécute ensuite `default` → `acceptEdits` → `plan` → retour à `default`. Les modes optionnels, décrits ci-dessous, s'insèrent après `plan`. La barre d'état affiche le mode actif sous la forme d'un `⏸ manual mode on` gris pour `default`, ou sous la forme `⏵⏵ accept edits on`, `⏸ plan mode on`, `⏵⏵ auto mode on`, `⏵⏵ don't ask on`, ou `⏵⏵ bypass permissions on`.

    Tous les modes ne sont pas dans le cycle par défaut :

    * `auto` : apparaît quand le [mode auto est disponible](#eliminate-prompts-with-auto-mode) ; basculer vers celui-ci change les modes de permission sans invite de confirmation
    * `bypassPermissions` : apparaît après que vous ayez démarré avec `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, `--allow-dangerously-skip-permissions`, ou `permissions.defaultMode: "bypassPermissions"` dans les [paramètres utilisateur, `--settings`, ou gérés](/docs/fr/settings-reference#permissions-defaultmode). La variante `--allow-` ajoute le mode de permission au cycle sans l'activer
    * `dontAsk` : n'apparaît jamais dans le cycle ; définissez-le avec `--permission-mode dontAsk`

    Les modes optionnels activés s'insèrent après `plan`, avec `bypassPermissions` en premier et `auto` en dernier. Si vous avez les deux activés, vous parcourrez `bypassPermissions` en allant vers `auto`.

    **À partir d'une invite de permission Bash** : en modes de permission Manuel et `acceptEdits`, quand le [mode auto](#eliminate-prompts-with-auto-mode) est disponible, Claude Code ajoute **Oui, et basculer vers le mode auto** à l'invite de permission d'une commande Bash. Sélectionnez-le pour approuver la commande et basculer la session vers le mode auto. Les invites de l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool) n'offrent pas l'option. Nécessite Claude Code v2.1.247 ou version ultérieure.

    Claude Code n'ajoute pas l'option aux invites forcées par l'une de vos [règles `ask`](/docs/fr/permissions#manage-permissions) ou par un [hook](/docs/fr/hooks#pretooluse-decision-control), car le mode auto vous montre toujours ces invites, donc basculer ne les supprimerait pas.

    **Au démarrage** : passez le mode de permission en tant que drapeau.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Comme valeur par défaut** : définissez `permissions.defaultMode` à la portée que vous souhaitez, comme décrit dans [Démarrer dans un mode de permission différent](#start-in-a-different-mode).

    Le même drapeau `--permission-mode` fonctionne avec `-p` pour les [exécutions non-interactives](/docs/fr/headless).
  </Tab>

  <Tab title="VS Code">
    **En cours de session** : cliquez sur l'indicateur de mode en bas de la zone de saisie. Il utilise ces étiquettes pour les modes sur cette page :

    | Étiquette UI       | Mode                |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    **Comme valeur par défaut** : pour épingler le mode de permission dans lequel les conversations démarrent, définissez `claudeCode.initialPermissionMode` dans vos paramètres utilisateur VS Code sur `default`, `manual`, `acceptEdits`, `plan`, ou `bypassPermissions`. Le paramètre n'accepte pas `auto` ; pour démarrer en Auto, laissez-le non défini et choisissez **Auto** à partir de l'indicateur de mode une fois, comme l'élément 2 ci-dessous le décrit. L'extension démarre chaque nouvelle conversation dans le premier de ces éléments qui s'applique :

    1. `claudeCode.initialPermissionMode`
    2. Le mode que vous avez choisi en dernier à partir de l'indicateur de mode, s'il s'agissait de Manuel, Éditer automatiquement, ou Auto. Choisir Plan ou Bypass permissions s'applique à cette conversation uniquement
    3. `permissions.defaultMode` à partir des [paramètres gérés](/docs/fr/managed-settings) ou `~/.claude/settings.json`, sur les plans Pro, Max et Team avec la [récupération des drapeaux de fonctionnalité](#which-mode-a-session-starts-in) disponible
    4. La [valeur par défaut intégrée](#which-mode-a-session-starts-in) pour votre plan, fournisseur et paramètres d'organisation

    L'extension ne lit jamais le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet pour le mode de permission de démarrage, et dans les conversations qui ne répondent pas aux conditions de l'élément 3, elle ne lit aucun fichier de paramètres du tout. Quand `claudeCode.claudeProcessWrapper` est défini, les éléments 3 et 4 ne s'appliquent pas non plus : ces conversations démarrent en Manuel sauf si l'élément 1 ou l'élément 2 définit un mode de permission.

    Auto apparaît dans l'indicateur de mode quand le [mode auto est disponible](#eliminate-prompts-with-auto-mode).

    Bypass permissions nécessite le bouton bascule **Allow dangerously skip permissions** dans les paramètres de l'extension. Sans lui, le mode de permission n'apparaît pas dans l'indicateur, et une valeur `bypassPermissions` de l'élément 1 ou l'élément 3 démarre la conversation en Manuel à la place. Auto de n'importe quel élément démarre également la conversation en Manuel quand le mode auto n'est pas disponible.

    Voir le [guide VS Code](/docs/fr/vs-code) pour les détails spécifiques à l'extension.
  </Tab>

  <Tab title="JetBrains">
    Le plugin JetBrains exécute Claude Code dans le terminal de l'IDE, donc basculer les modes de permission fonctionne de la même manière que dans la CLI : appuyez sur `Shift+Tab` pour parcourir, ou passez `--permission-mode` lors du lancement.
  </Tab>

  <Tab title="Desktop">
    **En cours de session** : dans l'onglet Code, utilisez le sélecteur de mode à côté du bouton d'envoi. Tous les modes n'apparaissent pas dans le sélecteur :

    * **Auto** : apparaît quand le [mode auto est disponible](#eliminate-prompts-with-auto-mode)
    * **Bypass permissions** : nécessite le bouton bascule **Allow bypass permissions mode** dans les paramètres Desktop sur les plans Pro et Max ; sur les plans Team et Enterprise, la politique organisationnelle le contrôle à la place

    L'onglet Cowork n'utilise pas ces modes. Cowork a ses propres modes de permission, activés séparément, et l'onglet Cowork n'affiche aucun sélecteur de mode du tout jusqu'à ce qu'un mode au-delà de sa valeur par défaut soit activé pour votre compte. Voir la [documentation Cowork](https://claude.com/docs/cowork/overview).

    Pour les détails spécifiques au bureau, voir [Choisir un mode de permission](/docs/fr/desktop#choose-a-permission-mode) dans le guide Desktop.

    **Comme valeur par défaut** : définissez `defaultMode` dans les [paramètres](/docs/fr/settings#where-settings-live). L'application de bureau lit les mêmes fichiers de paramètres que la CLI et applique le mode de permission aux nouvelles sessions locales.

    Un mode que vous choisissez dans le sélecteur de mode est mémorisé par dossier et prend précédence sur `defaultMode` pour ce dossier. Plan est l'exception : le choisir s'applique à la session actuelle uniquement.

    Pour où `defaultMode` va dans un fichier de paramètres, voir l'exemple sous [Démarrer dans un mode de permission différent](#start-in-a-different-mode).
  </Tab>

  <Tab title="Web and mobile">
    Utilisez la liste déroulante de mode à côté de la zone de saisie sur [claude.ai/code](https://claude.ai/code) ou dans l'application mobile. Les invites de permission apparaissent dans claude.ai pour approbation. Les modes qui apparaissent dépendent de l'endroit où la session s'exécute :

    * **[Sessions cloud](/docs/fr/claude-code-on-the-web)** : Accept edits, Plan et Auto. Accept edits correspond au mode `default` : les sessions cloud pré-approuvent les modifications de fichiers quel que soit le mode, donc la liste déroulante affiche Accept edits au lieu de Manual. Les sessions cloud honorent toujours `defaultMode: "acceptEdits"` à partir des paramètres. Le mode Auto n'apparaît que quand votre organisation le permet et que le modèle sélectionné le supporte. Bypass permissions n'est pas disponible.
    * **[Sessions Remote Control](/docs/fr/remote-control)** sur votre machine locale : Manual, Accept edits et Plan. Vous ne pouvez pas sélectionner Auto ou Bypass permissions à partir de l'application.
      * Sauf pour Bypass permissions, la liste déroulante affiche le mode de permission dans lequel se trouve la session locale, y compris un mode défini à partir du terminal. Elle se met à jour quand le mode de permission change dans l'application ou dans le terminal. La session ne signale jamais Bypass permissions à claude.ai, donc basculer vers celui-ci à partir du terminal ne change pas ce que la liste déroulante affiche.
      * Les sessions hébergées par l'[application de bureau](/docs/fr/desktop) ou l'[extension VS Code](/docs/fr/vs-code) signalent les changements de mode de permission à claude.ai au fur et à mesure qu'ils se produisent, de la même manière que les sessions hébergées dans un terminal.
      * Avant v2.1.202, les sessions connectées avec `/remote-control` ou `claude --remote-control` ne signalaient pas du tout leur mode de permission, donc claude.ai et l'application mobile pouvaient afficher un mode de permission dans lequel la session n'était pas. L'inadéquation affectait uniquement l'étiquette. Claude Code générait des invites de permission à partir du mode de permission réel de la session, et elles apparaissaient toujours dans l'application pour approbation.

    Pour Remote Control, la machine locale exécutant la session doit être connectée avec votre compte claude.ai ; les clés API ne sont pas supportées. Vous pouvez également définir le mode de permission de démarrage lors du lancement de cette session locale :

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Auto-approuver les modifications de fichiers avec le mode acceptEdits
</h2>

Le mode `acceptEdits` permet à Claude de créer et modifier des fichiers dans votre répertoire de travail sans inviter. La barre d'état affiche `⏵⏵ accept edits on` tandis que ce mode est actif.

En plus des modifications de fichiers, le mode `acceptEdits` auto-approuve les commandes Bash courantes du système de fichiers : `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, et `sed`. Ces commandes sont également auto-approuvées quand elles sont préfixées par des variables d'environnement sûres telles que `LANG=C` ou `NO_COLOR=1`, ou des wrappers de processus tels que `timeout`, `nice`, ou `nohup`. Comme pour les modifications de fichiers, l'auto-approbation s'applique uniquement aux chemins à l'intérieur de votre répertoire de travail ou `additionalDirectories`. Les chemins en dehors de cette portée, les écritures vers les [chemins protégés](#protected-paths), les suppressions `rm` et `rmdir` ciblant un [chemin critique](#critical-paths), et toutes les autres commandes Bash sauf l'[ensemble intégré en lecture seule](/docs/fr/permissions#read-only-commands) invitent toujours.

Quand l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool) est activé, le mode `acceptEdits` auto-approuve également `Set-Content`, `Add-Content`, `Clear-Content`, et `Remove-Item` sur les chemins dans la portée, ainsi que leurs alias courants. Les mêmes règles de portée et de chemin protégé s'appliquent, et `Remove-Item` obtient [sa propre vérification](#remove-item-in-powershell). Un argument positionnel qui contient un caractère de guillemet, comme l'apostrophe dans `Set-Content .\notes.txt "It's done"`, invite toujours même sur les chemins dans la portée, car Claude Code ne peut pas valider statiquement un argument dont les lectures entre guillemets et sans guillemets diffèrent. Passez le contenu via un paramètre nommé tel que `-Value` pour éviter l'invite.

Utilisez `acceptEdits` quand vous souhaitez examiner les modifications dans votre éditeur ou via `git diff` après coup plutôt que d'approuver chaque modification en ligne.

Appuyez sur `Shift+Tab` une fois à partir du mode Manuel pour y accéder, ou démarrez directement avec celui-ci :

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analysez avant de modifier avec le mode plan
</h2>

Le mode plan indique à Claude de rechercher et de proposer des modifications sans les effectuer. Claude lit les fichiers, exécute des commandes shell pour explorer et rédige un plan, mais ne modifie pas votre source. Sauf dans les sessions avec les [permissions de contournement disponibles](#skip-all-checks-with-bypasspermissions-mode), les modifications restent bloquées jusqu'à ce que vous approuviez le plan.

Quand le [mode auto](/docs/fr/auto-mode-config) est disponible et que le paramètre `useAutoModeDuringPlan` est activé, ce qui est le cas par défaut, le classificateur examine les commandes shell pendant la planification au lieu de vous inviter. Les commandes approuvées s'exécutent, et les commandes rejetées sont bloquées. Sinon, les commandes en dehors de l'[ensemble intégré en lecture seule](/docs/fr/permissions#read-only-commands) invitent une approbation, y compris quand le [mode auto-allow](/docs/fr/sandboxing#sandbox-modes) du sandbox est activé. Dans les sessions avec les permissions de contournement disponibles, ni le classificateur ni une invite ne s'appliquent aux commandes de planification ; [Ignorer tous les contrôles avec le mode bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) couvre les quelques choses qui invitent toujours là. Dans v2.1.212 à v2.1.217, les sessions sans permissions de contournement invitaient pour chaque commande en dehors de l'ensemble en lecture seule, que le mode auto soit disponible ou non.

Entrez en mode plan en appuyant sur `Shift+Tab` ou en préfixant une seule invite avec `/plan`. Vous pouvez également démarrer en mode plan à partir de la CLI :

```bash theme={null}
claude --permission-mode plan
```

Appuyez à nouveau sur `Shift+Tab` pour quitter le mode plan sans approuver un plan.

<h3 id="review-and-approve-a-plan">
  Examinez et approuvez un plan
</h3>

Quand le plan est prêt, Claude le présente et demande comment procéder. À partir de cette invite, vous pouvez choisir :

* **Oui, et utiliser le mode auto** : approuver et démarrer en [mode auto](#eliminate-prompts-with-auto-mode). Si le mode auto n'est pas [disponible pour votre session](#eliminate-prompts-with-auto-mode), par exemple parce que votre organisation l'a désactivé, cette option lit **Oui, auto-accepter les modifications**. Si vous avez démarré la session avec les permissions de contournement activées, l'option lit **Oui, et basculer vers BYPASS PERMISSIONS (aucune invite supplémentaire) pour cette session** à la place.
* **Oui, approuver manuellement les modifications** : approuver et examiner chaque modification individuellement.
* **Non, continuer la planification** : rester en mode plan et dire à Claude ce qu'il faut modifier.

L'approbation d'un plan quitte le mode plan et bascule la session vers le mode de permission que chaque option d'approbation décrit, de sorte que Claude commence à modifier. Pour planifier à nouveau, revenez au mode plan avec `Shift+Tab`, ou préfixez votre prochaine invite avec `/plan`.

Appuyez sur `Ctrl+G` pour ouvrir le plan proposé dans votre éditeur de texte par défaut et le modifier directement avant que Claude ne procède. Quand [`showClearContextOnPlanAccept`](/docs/fr/settings-reference#showclearcontextonplanaccept) est activé, la liste gagne une première option qui approuve le plan et efface le contexte de planification.

L'acceptation d'un plan donne également à la session un [titre généré](/docs/fr/sessions#name-your-sessions) basé sur le plan, sauf si vous avez déjà nommé la session.

<h3 id="set-plan-mode-as-the-default">
  Définissez le mode plan comme valeur par défaut
</h3>

Pour faire du mode plan la valeur par défaut pour les sessions de terminal d'un projet, définissez `defaultMode` sur `plan` dans `.claude/settings.json`, placé comme l'exemple sous [Démarrer dans un mode de permission différent](#start-in-a-different-mode) le montre. Les conversations que l'[extension VS Code](/docs/fr/vs-code) démarre ne lisent pas les paramètres du projet pour le mode de permission de démarrage. Là, définissez `claudeCode.initialPermissionMode` sur `plan` dans vos paramètres utilisateur VS Code à la place.

<h2 id="eliminate-prompts-with-auto-mode">
  Éliminer les invites de permission avec le mode auto
</h2>

Le mode auto permet à Claude d'exécuter sans invites de permission routinières. Un modèle classificateur distinct examine les actions avant leur exécution, bloquant tout ce qui dépasse votre demande, cible une infrastructure non reconnue, ou semble provenir d'un contenu hostile que Claude a lu. Les [règles ask](/docs/fr/permissions#manage-permissions) explicites forcent toujours une invite.

Sur les plans Pro, Max et Team, le mode auto est le [mode de permission de démarrage intégré](#which-mode-a-session-starts-in).

Le classificateur examine également chaque message que Claude envoie à un autre agent avec [`SendMessage`](/docs/fr/tools-reference), qu'il s'agisse de texte brut ou d'un message [d'équipe d'agents](/docs/fr/agent-teams) structuré, avant que Claude Code le livre, à la fois en mode auto et en [mode plan tandis que le classificateur examine les commandes](#analyze-before-you-edit-with-plan-mode) ; l'examen d'envoi nécessite Claude Code v2.1.222 ou ultérieur.

Le classificateur examine également et approuve ou bloque les suppressions `rm` et `rmdir` ciblant un [chemin critique](#critical-paths), comme `rm -rf /` et `rm -rf ~`, y compris lorsque la suppression se trouve à l'intérieur d'une substitution de commande ou de processus.

Le mode auto encourage également Claude à continuer à travailler sans s'arrêter pour des questions de clarification, bien que Claude demande toujours quand votre invite ou une compétence s'y appuie explicitement. Pour un comportement plus autonome dans un mode qui vous invite toujours, définissez plutôt le [style de sortie proactif](/docs/fr/output-styles).

<Warning>
  Le mode auto réduit les invites de permission mais ne garantit pas la sécurité. Utilisez-le pour les tâches où vous faites confiance à la direction générale, pas comme remplacement pour l'examen des opérations sensibles.
</Warning>

Le mode auto est disponible uniquement lorsque votre compte répond à tous ces critères :

* **Plan** : Tous les plans.
* **Organisation** : sur Team et Enterprise, le mode auto est disponible par défaut. Les administrateurs peuvent le désactiver pour l'organisation en définissant `permissions.disableAutoMode` sur `"disable"` dans les [paramètres gérés](/docs/fr/managed-settings).
* **Modèle** : sur l'API Anthropic et [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws), Claude Opus 4.6 ou ultérieur, Sonnet 4.6 ou ultérieur, ou un [modèle Fable](/docs/fr/model-config#work-with-fable). Sur Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry et les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectées, uniquement Claude Sonnet 5, Opus 4.7 ou ultérieur, et les modèles Fable. Les modèles plus anciens, y compris Sonnet 4.5, Opus 4.5, Haiku et les modèles claude-3, ne sont pas pris en charge sur aucun fournisseur.
* **Fournisseur** : disponible par défaut sur l'API Anthropic, Claude Platform sur AWS, Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry et les sessions de passerelle d'applications Claude connectées.

Si Claude Code signale que le mode auto n'est pas disponible, vérifiez d'abord ces critères et si un fichier de paramètres définit [`disableAutoMode`](/docs/fr/settings-reference#disableautomode). Anthropic peut également avoir désactivé le mode auto côté serveur, ou le serveur peut avoir rejeté le mode auto pour votre compte. Une session qui a reçu l'une ou l'autre réponse garde le mode auto désactivé jusqu'à la fin de la session, donc démarrez une nouvelle session plus tard.

Un message distinct qui nomme un modèle et dit que le mode auto « ne peut pas déterminer la sécurité » d'une action signifie qu'une demande de classificateur a échoué. Cet échec est généralement transitoire, mais sur Amazon Bedrock, il peut se répéter jusqu'à ce que votre compte puisse invoquer le modèle nommé. Consultez la [référence des erreurs](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action) pour les causes et ce qu'il faut faire.

Si vous définissez `defaultMode: "auto"` dans les [paramètres](/docs/fr/settings-reference#all-settings) et qu'une session de terminal démarre en mode Manuel sans erreur, le paramètre se trouve probablement dans `.claude/settings.json` ou `.claude/settings.local.json`. `auto` ne prend pas effet à partir de ces fichiers. Déplacez-le vers `~/.claude/settings.json`. Pour une conversation que l'extension VS Code a démarrée, vérifiez plutôt la liste propre de l'extension dans [Basculer les modes de permission](#switch-permission-modes).

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Mode auto sur Bedrock, Agent Platform ou Foundry
</h3>

Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [la plateforme Agent de Google Cloud](/docs/fr/google-vertex-ai), [Microsoft Foundry](/docs/fr/microsoft-foundry) et les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectées, le mode auto apparaît dans le cycle `Shift+Tab` par défaut. L'apparition dans le cycle ne change pas le mode de permission dans lequel une session démarre : sur ces fournisseurs, les sessions de terminal démarrent dans votre [`defaultMode`](/docs/fr/settings-reference#permissions-defaultmode), qui est Manuel sauf si vous le changez, et les conversations dans l'[extension VS Code](/docs/fr/vs-code) démarrent en Manuel sauf si `claudeCode.initialPermissionMode` ou un mode que vous avez choisi dans l'extension en définit un. Seuls Claude Sonnet 5, Opus 4.7 ou ultérieur, et les modèles Fable sont pris en charge sur ces fournisseurs.

Pour faire du mode auto le mode de permission de démarrage par défaut, définissez `"permissions": {"defaultMode": "auto"}` dans les paramètres utilisateur ou gérés. Dans les sessions que l'extension VS Code démarre, sélectionnez plutôt **Auto** dans l'indicateur de mode. [Basculer les modes de permission](#switch-permission-modes) couvre ce qui prime sur ce choix.

Le contrôle [`/doctor`](/docs/fr/commands#all-commands) propose cette valeur par défaut des paramètres utilisateur sur ces fournisseurs de la même manière qu'il le fait sur l'API Anthropic.

Pour empêcher les développeurs d'utiliser le mode auto, définissez `disableAutoMode` sur `"disable"` dans les [paramètres gérés](/docs/fr/managed-settings). Cela supprime `auto` du cycle `Shift+Tab`, et une session démarrée avec `--permission-mode auto` démarre en Manuel à la place. Une session déjà en cours d'exécution en mode auto le quitte lorsque le paramètre l'atteint à partir d'une [source déployée par l'administrateur](/docs/fr/managed-settings#which-managed-source-claude-code-uses), et affiche `auto mode disabled by settings`. Avant v2.1.251, une session en cours d'exécution gardait le mode auto jusqu'à la fin.

Dans v2.1.158 à v2.1.206, le mode auto était désactivé sur ces fournisseurs jusqu'à ce que vous définissiez `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, et Claude Code ignorait `defaultMode: "auto"` sur ces fournisseurs sauf si la variable était également définie. La variable est toujours acceptée pour la compatibilité et n'a aucun effet à partir de v2.1.207.

<h3 id="server-side-classifier-review">
  Examen du classificateur côté serveur
</h3>

En mode auto, Claude Code peut demander au serveur de vérifier les actions que [l'ordre de décision](#how-the-classifier-evaluates-actions) envoie pour examen, dans le cadre des demandes de modèle de la session, à la place d'envoyer ses propres demandes de classificateur. Ces sessions demandent :

* **Une connexion directe à l'API Anthropic** : dans une session de terminal interactive, sur tous les plans claude.ai et sur les comptes qui utilisent l'API Claude, à mesure qu'Anthropic le déploie. Nécessite Claude Code v2.1.271 ou ultérieur sur les plans Pro, Max et Team, et v2.1.278 ou ultérieur sur les plans Enterprise et les comptes API Claude. À partir de v2.1.282, une session qui [ne récupère pas les drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching), par exemple parce que vous avez désactivé la télémétrie, demande au serveur par défaut dans n'importe quel type de session.
* **Un fournisseur cloud, ou une passerelle LLM ou un proxy** : sur [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws), Amazon Bedrock, la plateforme Agent de Google Cloud et Microsoft Foundry, et chaque fois que vous pointez `ANTHROPIC_BASE_URL` vers une [passerelle LLM ou un proxy](/docs/fr/llm-gateway), quel que soit votre plan. Demander au serveur par défaut nécessite Claude Code v2.1.278 ou ultérieur.
* **Une session de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectée** : nécessite Claude Code v2.1.280 ou ultérieur

Là où le serveur examine les actions, ses verdicts les décident. Deux autres résultats sont possibles :

* **Le serveur n'examine pas la session** : une réponse se termine sans résultats d'examen, ou le serveur répond qu'il n'examine pas cette session. Les causes les plus courantes sont une passerelle LLM ou un proxy qui supprime la demande d'examen ou les résultats, et une plateforme, une région ou des credentials qui n'ont pas encore de vérifications côté serveur. Claude Code revient à ses propres demandes de classificateur. Une fois que ce retour se maintient pour le reste de la session, il affiche un [avis sur les frais de demande de classificateur](/docs/fr/auto-mode-classifier-billing) sur les comptes où ces demandes sont facturées.
* **Le serveur ne donne pas de verdict pour une action** : Claude Code refuse l'action plutôt que de l'exécuter sans examen. Sur n'importe quelle connexion, cela se produit lorsque la réponse se termine avant l'arrivée des résultats d'examen ou que les résultats arrivent sous une forme que Claude Code ne peut pas lire. Une passerelle LLM ou un proxy qui raccourcit les réponses ou réécrit les résultats peut causer l'un ou l'autre. Sur une connexion directe à l'API Anthropic, cela se produit également lorsque la vérification du serveur échoue pour l'action, par exemple en dépassant le délai d'attente. [Le serveur n'a retourné aucun verdict de sécurité](/docs/fr/errors#the-server-returned-no-safety-verdict) couvre le message de refus, ce qui se passe lorsque les refus se répètent, et ce qu'il faut faire.

Pour ignorer la demande au serveur et toujours utiliser les propres demandes de classificateur de Claude Code, définissez [`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/fr/env-vars). Sur une connexion directe à l'API Anthropic, la variable nécessite Claude Code v2.1.281 ou ultérieur. La définir sur `1` là-bas active l'examen du serveur dans une session qui ne l'a pas encore, comme une session `-p` ou Agent SDK, sauf si vous avez également défini `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`. Si vous définissez `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` et laissez `CLAUDE_CODE_AUTO_MODE_SERVER` non défini, Claude Code cesse également de demander au serveur.

<h3 id="what-the-classifier-blocks-by-default">
  Ce que le classificateur bloque par défaut
</h3>

Le classificateur fait confiance à votre répertoire de travail et aux télécommandes qui ont été configurées pour celui-ci au démarrage de la session. Une télécommande ajoutée ou réorientée pendant la session avec `git remote add` ou `git remote set-url` n'est pas approuvée, et tout le reste est traité comme externe jusqu'à ce que vous [configuriez l'infrastructure approuvée](/docs/fr/auto-mode-config). Avant v2.1.200, les télécommandes ajoutées en milieu de session étaient également approuvées.

**Bloqué par défaut** :

* Téléchargement et exécution de code, comme `curl | bash`
* Envoi de données sensibles à des points de terminaison externes
* Déploiements et migrations de production
* Suppression en masse sur le stockage cloud
* Octroi de permissions IAM ou de dépôt
* Modification d'une infrastructure partagée
* Destruction irréversible de fichiers qui existaient avant la session
* Forcer la poussée
* Valider ou pousser une modification qui enverrait des secrets ou des données sensibles en dehors du dépôt lors de son exécution, ou élargir ce qu'un déploiement expose. Cela couvre un flux de travail CI ou une configuration de déploiement qui transmet un secret à une destination qui ne le reçoit pas déjà, un script ou une étape de configuration qui lit un magasin de secrets et envoie les données, et une modification de configuration qui élargit ce qu'un déploiement publie, comme un registre, une visibilité, un artefact ou un paramètre de sourcemap. La vérification s'applique sur n'importe quelle branche, s'applique même lorsque le dépôt est public, et se déclenche lorsque la modification est validée ou poussée, que ce commit ou cette poussée déclenche ou non le pipeline ; la clarifier nécessite de nommer l'effet d'exécution, pas seulement le commit ou la poussée. Avant v2.1.211, cette vérification était limitée à la branche par défaut à la place : une poussée là-bas était bloquée lorsqu'elle contenait du contenu sensible, des modifications dissimulées ou mal décrites par rapport à ce que vous avez demandé, du contenu porté de l'extérieur du dépôt, ou contourné autour d'une révision que vous avez demandée
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop` ou `git stash clear`, que le classificateur présume éliminerait les modifications non validées
* `git commit --amend` lorsque le commit à HEAD n'a pas été créé dans cette session
* À partir de v2.1.198, `git commit --amend` lorsque le commit à HEAD a déjà été poussé. Une reformulation de message uniquement n'est pas bloquée : `--amend -m` sans rien de nouvellement préparé, sur un commit que Claude a créé pendant cette session
* `terraform destroy`, `pulumi destroy`, `cdk destroy` ou `terragrunt destroy`, et l'application d'un plan qui détruit des ressources

Claude Code v2.1.195 et ultérieur bloquent plus de catégories par défaut. Plusieurs dépendent d'entrées [d'environnement](/docs/fr/auto-mode-config#define-trusted-infrastructure), comme les cibles de télécommande sensibles et les portées IaC protégées, que vous pouvez affiner à des noms concrets.

* Écriture dans un gestionnaire de secrets, ou modification des enregistrements DNS ou des certificats TLS
* Fusion d'une demande de tirage qu'aucun humain n'a approuvée, approbation de la propre demande de tirage de Claude, ou désactivation des vérifications CI
* Publication d'un commentaire qui est lui-même une commande pour l'automatisation, comme `atlantis apply` ou un `/deploy` ou `/merge` de bot
* Basculement, augmentation ou suppression d'un drapeau de fonctionnalité de production
* Application de modifications d'infrastructure à une portée IaC protégée, ou vidage et suppression de nœuds de cluster
* Écritures dans un cluster de calcul partagé qui vont au-delà de la ressource que vous avez nommée, comme un sélecteur d'étiquette ou `--all` qui capture les travaux d'autres utilisateurs
* Création de ressources Kubernetes qui s'exécutent sur chaque nœud ou interceptent le trafic du cluster, comme les DaemonSets et les webhooks d'admission
* Shells interactifs ou port-forwards vers une cible de télécommande sensible
* Ouverture d'un tunnel ou d'un shell inverse qui rend un service local accessible depuis l'Internet public
* Impression d'une credential ou d'un token en direct dans la transcription ou un fichier
* Accès à un emplacement répertorié comme emplacement de données sensibles dans votre [environnement](/docs/fr/auto-mode-config#define-trusted-infrastructure), ou copie de données en dehors d'un. À partir de v2.1.198, cela bloque également l'envoi de données d'un à un public que l'entrée exclut
* Routage d'une installation de package autour de votre registre de package interne vers un registre public. À partir de v2.1.198, cela s'applique également lorsque vous avez dit à Claude qu'un registre interne ou un miroir existe dans la conversation, pas seulement lorsqu'un est répertorié dans votre environnement
* Exécution d'une commande avec un drapeau qui désarme une garde de sécurité, comme `--insecure`
* Lancement d'une boucle d'agent autonome qui s'exécute sans approbation humaine ou sandbox, comme une lancée avec `--dangerously-skip-permissions` ou `--no-sandbox`. À partir de v2.1.198, cela couvre également l'exécution d'un agent tiers ou d'un harnais d'évaluation avec isolation et approbation par action désactivées, comme un lanceur démarré avec `--yes-always`
* Actions du navigateur [Claude dans Chrome](/docs/fr/chrome) qui pourraient envoyer le contenu de la page, les cookies ou les credentials hors origine

Claude Code v2.1.198 et ultérieur bloquent également ces par défaut :

* Suppression de fichiers dans `/tmp`, `$TMPDIR` ou un autre répertoire de travail partagé ou de cache par caractère générique, glob ou filtre d'âge plutôt que par un chemin nommé spécifique
* Inclusion de détails sensibles dans le contenu envoyé, téléchargé, publié ou écrit à d'autres personnes ou systèmes partagés, lorsque votre propre message n'a pas autorisé ces détails pour ce destinataire. Les corps de PR et de problème, les messages de commit et les commentaires comptent comme ce type de contenu sortant lorsque le dépôt est en dehors de la limite de confiance ou public, y compris les dépôts publics de votre propre organisation ; les chemins de fichiers internes, les noms de code, les données de réponse API en direct comme les e-mails ou les identifiants de compte, et les identifiants d'infrastructure comptent comme des détails sensibles. La portée PR, problème et message de commit nécessite Claude Code v2.1.200 ou ultérieur. Les données personnelles en direct d'une réponse API dans un corps de PR ou de problème, comme une adresse e-mail, un identifiant de compte ou d'organisation, ou une métrique d'utilisation, vous obligent à nommer ces détails et le destinataire indépendamment de la visibilité ou de la limite de confiance du dépôt. Cette vérification nécessite Claude Code v2.1.203 ou ultérieur
* Envoi de frappes à la propre pane tmux de Claude Code pour piloter sa propre interface, que le classificateur traite comme Claude changeant ses propres permissions ou surveillance

Claude Code v2.1.200 et ultérieur bloquent également ces par défaut :

* Commentaire, suppression ou passage en force d'un test ou d'une assertion qui protège le comportement de sécurité, comme l'authentification, le contrôle d'accès, la validation des entrées ou le sandboxing
* Suppression ou démantèlement d'une ressource avec état que Claude n'a pas créée dans la session, lorsqu'aucune règle de suppression plus spécifique ne s'applique et que vous n'avez pas nommé cette ressource
* Réorientation d'une URL de base API, d'un point de terminaison proxy, d'un récepteur webhook ou d'un miroir de registre vers un hôte tiers qui ne correspond pas à la tâche, y compris dans les fichiers d'exemple comme `.env.example`
* Modification de la destination des poussées avec `git remote set-url` ou `git remote add`, sauf si vous avez nommé la nouvelle télécommande
* Poussée de secrets ou de données personnelles ou confiées vers un dépôt connu pour être public, ou poussée de matériel confidentiel là-bas qui ne fait pas partie du travail propre de ce dépôt. Le sujet propre d'un dépôt de dotfiles est la seule exception pour les données personnelles ou confiées, et le contenu d'un dépôt privé atteignant n'importe quelle surface publique est bloqué de la même manière ; les deux raffinements nécessitent Claude Code v2.1.203 ou ultérieur. Avant v2.1.203, les données personnelles étaient regroupées avec le matériel confidentiel et bloquées uniquement lorsqu'elles ne faisaient pas partie du travail propre de ce dépôt. Lorsque la visibilité d'un dépôt n'est pas établie, le classificateur ne bloque pas sur cela seul ; il juge le contenu par rapport aux autres règles à la place
* Ouverture d'une demande de tirage contre un dépôt ou une organisation différente, bifurcation avec `gh repo fork` ou poussée vers un dépôt tiers, sauf si vous avez nommé cette cible externe

Claude Code v2.1.203 et ultérieur bloquent également ces par défaut :

* Contenu d'un magasin local sensible, ou d'un fichier dont le nom, le chemin ou le type le marque comme sensible, entrant dans un commit, une poussée, un texte de PR ou de problème, une gist ou un collage, ou une publication de package, sauf si vous avez nommé à la fois la source et la destination. Les transcriptions de session et les journaux de conversation, les dossiers de configuration et de credential pointant comme les clés SSH, les credentials cloud, les profils de navigateur et l'historique du shell, et les exports de données utilisateur comptent tous, et le dépôt étant privé ne le clarifie pas

Claude Code v2.1.205 et ultérieur bloquent également ces par défaut :

* Écriture dans les transcriptions de session Claude Code, les fichiers d'historique `.jsonl` sous `~/.claude/projects/` ou votre répertoire de configuration configuré, directement ou via une commande shell. La règle couvre également les lignes de métadonnées que Claude Code ajoute à chaque entrée de transcription pour ses propres vérifications. La lecture d'une transcription n'est pas bloquée
* Une suppression forcée récursive comme `rm -rf "$VAR"` ou `Remove-Item -Recurse -Force $dir` dont la cible est une variable shell, ou un glob enraciné à une, qui n'est assignée nulle part dans la conversation que le classificateur voit. La valeur provenait uniquement de la sortie de commande antérieure, que le classificateur ne reçoit jamais, donc le classificateur ne peut pas vérifier la cible de suppression par rapport aux autres règles de suppression. Le bloc s'efface lorsque vous nommez le chemin exact en cours de suppression, ou lorsque Claude réexécute la suppression avec le chemin littéral résolu écrit dans la commande. Les suppressions dont la cible le classificateur peut résoudre ne sont pas affectées. Les cibles `Remove-Item` qui sont un `*` nu ou se terminent par `/*` ou `\*` n'atteignent jamais le classificateur : Claude Code les [refuse directement](#remove-item-in-powershell)

Claude Code v2.1.257 et ultérieur bloquent également ces par défaut :

* Demande de credentials à partir du point de terminaison de métadonnées d'instance cloud, comme `169.254.169.254`, ou authentification explicite d'un appel cloud, cluster ou registre avec l'identité de compte de service ou de nœud de la machine
* Atteinte d'un hôte public par une route autre qu'une demande directe, comme un tunnel, un shell inverse, ou une configuration de résolveur ou de proxy réécrite pour pointer vers l'extérieur
* Lecture de credentials qui appartiennent à l'hôte plutôt qu'à votre tâche, comme les certificats de nœud ou l'authentification du registre de conteneurs du nœud
* Connexion à ou analyse de conteneurs, pods ou VMs frères que Claude n'a pas démarrés, ou le nœud sous le conteneur

Si Claude Code s'exécute quelque part qui est censé permettre l'un de ceux-ci, décrivez cette configuration dans une [entrée Host containment](/docs/fr/auto-mode-config#define-trusted-infrastructure) dans `autoMode.environment`.

Claude Code v2.1.261 et ultérieur bloquent également ces par défaut :

* Publication ou écriture d'un lien vers un service public de collage, de diagramme ou de partage de données dans un message, un texte de PR ou de problème, un document, ou n'importe où ailleurs où le lien sera ouvert ou récupéré, lorsque l'URL elle-même porte le contenu partagé, sauf si vous avez nommé ce service

**Autorisé par défaut** :

* Opérations de fichiers locaux dans votre répertoire de travail
* Installation de dépendances déclarées dans vos fichiers de verrouillage ou manifestes
* Lecture de `.env` et envoi de credentials à leur API correspondante
* Demandes HTTP en lecture seule
* Poussée vers n'importe quelle branche du dépôt sur lequel vous travaillez, y compris la branche par défaut. Une branche non par défaut dont le nom la marque comme cible de déploiement ou de publication, comme `production` ou `gh-pages`, n'est pas couverte : le classificateur juge une poussée là-bas selon ses propres termes. Le contenu de la poussée est toujours vérifié par rapport aux autres règles, les règles [`permissions.deny`](/docs/fr/permissions#manage-permissions) peuvent toujours bloquer les commandes de poussée [telles qu'écrites](/docs/fr/permissions#bash-rule-limits) dans chaque mode, et la protection de branche propre de la télécommande s'applique toujours. Avant v2.1.211, seules les poussées vers la branche sur laquelle vous avez commencé, les branches que Claude a créées, et les poussées routinières vers la branche par défaut étaient autorisées par défaut, et avant v2.1.203 toute poussée directe vers la branche par défaut était bloquée

Claude Code v2.1.195 et ultérieur autorisent également ces par défaut :

* Suppression des travaux exacts que Claude a créés plus tôt dans la même session
* Lecture, examen ou écriture de code, configs et modèles de menace liés à la sécurité dans le cadre de votre tâche
* Messages entre agents travaillant ensemble dans la même session multi-agent
* Envoi de données aux domaines approuvés, buckets et services que vous répertoriez dans [`environment`](/docs/fr/auto-mode-config#define-trusted-infrastructure). Cela couvre le flux de données uniquement, pas les opérations destructrices ou de credential sur la même infrastructure
* [Claude dans Chrome](/docs/fr/chrome) navigation vers un domaine interne approuvé, localhost, ou une URL que vous avez nommée

Les commandes en sandbox n'obtiennent pas d'accès réseau par défaut. Claude nomme les hôtes qu'une commande nécessite sur la commande elle-même, le classificateur les examine avec la commande, et une liste approuvée ouvre ces hôtes pour cette seule commande. [Domaines autorisés par commande](/docs/fr/sandboxing#per-command-allowed-domains-in-auto-mode) couvre ce qu'une liste peut et ne peut pas ouvrir et ce qui se passe lorsqu'une commande atteint un hôte non répertorié.

Exécutez `claude auto-mode defaults` pour imprimer les listes de règles complètes en JSON. Si les actions routinières sont bloquées, un administrateur peut ajouter des dépôts, buckets et services approuvés via le paramètre `autoMode.environment` : voir [Configurer le mode auto](/docs/fr/auto-mode-config).

Pousser vers n'importe quelle branche du dépôt sur lequel vous travaillez et créer une demande de tirage qui correspond à votre demande s'exécutent sans invite, sauf si la poussée ou la demande de tirage tombe sous la [liste bloquée](#what-the-classifier-blocks-by-default), comme des secrets ou des données sensibles quittant le dépôt, ou une demande de tirage qui cible un dépôt ou une organisation différente. Pour exiger un point de contrôle humain avant ces commandes tout en restant en mode auto, ajoutez des règles `permissions.ask`, qui correspondent à la commande [telle qu'écrite](/docs/fr/permissions#bash-rule-limits) : voir [Limites communes](/docs/fr/auto-mode-config#common-boundaries).

<h3 id="first-read-outside-the-working-directories">
  La première lecture en dehors des répertoires de travail
</h3>

Tandis que [`permissions.blockReadsOutsideWorkingDirectories`](/docs/fr/settings-reference#permissions-blockreadsoutsideworkingdirectories) est désactivé, les lectures de fichiers s'exécutent sans invite en mode auto, y compris les lectures en dehors des [répertoires de travail](/docs/fr/permissions#working-directories). La première fois que Claude utilise l'outil Read, Grep ou Glob sur un chemin en dehors d'eux, Claude Code vous demande si vous souhaitez continuer à autoriser ces lectures.

L'invite n'apparaît pas dans les exécutions `-p` non interactives ou les sessions en arrière-plan ; les lectures là-bas s'exécutent comme avant.

Quelle que soit votre réponse, Claude continue à travailler :

* **Continuer à autoriser** : la lecture s'exécute, les lectures ultérieures en dehors des répertoires de travail s'exécutent comme avant, et Claude Code enregistre votre réponse pour que l'invite n'apparaisse plus
* **Bloquer à partir de maintenant** : la lecture est refusée, et Claude Code définit [`permissions.blockReadsOutsideWorkingDirectories`](/docs/fr/settings-reference#permissions-blockreadsoutsideworkingdirectories) sur `true` dans vos paramètres utilisateur, ce qui fait que les outils de fichier refusent ces lectures dans chaque session ultérieure et chaque mode de permission. Pour laisser Claude lire ce chemin plus tard, ajoutez son répertoire avec `/add-dir` ou supprimez le paramètre.
* **Demander à nouveau la prochaine fois** : la lecture est refusée, et la prochaine lecture en dehors des répertoires de travail invite à nouveau

<h3 id="boundaries-you-state-in-conversation">
  Limites que vous énoncez dans la conversation
</h3>

Le classificateur traite les limites que vous énoncez dans la conversation comme un signal de blocage. Si vous dites à Claude « ne pousse pas » ou « attends que j'examine avant de déployer », le classificateur bloque les actions correspondantes même lorsque les règles par défaut les autoriseraient. Une limite reste en vigueur jusqu'à ce que vous la leviez dans un message ultérieur. Le propre jugement de Claude qu'une condition a été remplie ne la lève pas.

Les limites ne sont pas stockées en tant que règles. Le classificateur les relit à partir de la transcription à chaque vérification, donc une limite peut être perdue si la [compaction de contexte](/docs/fr/costs#reduce-token-usage) supprime le message qui l'a énoncée. Pour une garantie ferme, ajoutez plutôt une [règle deny](/docs/fr/permissions#permission-rule-syntax).

<h3 id="approvals-you-state-in-conversation">
  Approbations que vous énoncez dans la conversation
</h3>

Si vous dites à Claude qu'une action bloquée est autorisée, le classificateur lit cela comme votre approbation et peut lever le bloc. La façon dont vous l'avez formulé décide si l'action s'exécute et jusqu'où l'approbation s'étend :

* **Nommez l'action et ses spécificités** : votre message doit nommer l'action et la chose spécifique qui la rend dangereuse, comme la branche d'une poussée forcée. Nommer le verbe seul ne clarifie rien, donc « vous pouvez forcer la poussée » laisse le bloc en place.
* **Attendez-vous à ce qu'il couvre une action** : une approbation couvre l'action destructrice que vous avez nommée, donc une action ultérieure est bloquée à nouveau sauf si vous avez accordé l'approbation comme permanente. Pour arrêter d'approuver un modèle routinier une action à la fois, ajoutez-le à [`autoMode.allow`](/docs/fr/auto-mode-config#override-the-block-and-allow-rules).
* **Certains blocs restent en place** : [l'ordre de précédence du classificateur](/docs/fr/auto-mode-config#override-the-block-and-allow-rules) énonce quels blocs votre approbation peut atteindre. Pour exécuter une étape qu'il ne clarifiera pas, [quittez le mode auto](#switch-permission-modes) et répondez à l'invite de permission.

<h3 id="when-auto-mode-falls-back">
  Quand le mode auto revient en arrière
</h3>

Lorsque le mode auto ne peut pas approuver les actions de votre session, ce qui se passe dépend du cas :

* **Une action bloquée** : Claude Code affiche une notification et répertorie l'action dans `/permissions` sous l'onglet **Recently denied**, où vous pouvez appuyer sur `r` pour la réessayer avec une approbation manuelle. Lorsque le classificateur produit [aucun verdict sur l'action](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action), parce qu'une vérification de sécurité distincte du mode auto a refusé la propre demande du classificateur ou sa réponse n'a pas été analysée, Claude Code refuse l'action sans la notification ou l'entrée **Recently denied**.
* **Blocages répétés** : si le classificateur bloque une action 3 fois de suite ou 20 fois au total, le mode auto s'interrompt et Claude Code reprend l'invite. L'approbation de l'action invitée reprend le mode auto. Ces seuils ne sont pas configurables. Toute action autorisée réinitialise le compteur consécutif, tandis que le compteur total persiste pour la session et se réinitialise uniquement lorsque sa propre limite déclenche un retour. Claude Code ne compte pas un refus vers l'un ou l'autre seuil lorsqu'[une vérification de sécurité distincte du mode auto refuse la propre demande du classificateur](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action) ; l'entrée liée couvre comment Claude Code gère ces refus.
* **Sessions qui ne peuvent pas inviter** : une exécution `-p` [non interactive](/docs/fr/headless) sans [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags) n'a pas d'invite pour revenir. Lorsque les blocages répétés atteignent un seuil, l'action ne s'exécute pas et Claude continue à travailler. La même chose s'applique lorsqu'[une vérification de sécurité distincte du mode auto refuse la demande du classificateur](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action). Claude Code n'arrête pas l'exécution dans l'un ou l'autre cas.
* **Aucun verdict du serveur** : sous [examen du classificateur côté serveur](#server-side-classifier-review), Claude Code refuse une action pour laquelle le serveur ne donne pas de verdict, et arrête le tour après dix réponses de suite sans verdict. Voir [Le serveur n'a retourné aucun verdict de sécurité](/docs/fr/errors#the-server-returned-no-safety-verdict).
* **Un changement de mode pendant une vérification** : si vous changez les modes de permission tandis qu'une vérification de classificateur est en attente, Claude Code rejette un verdict que le nouveau mode n'aurait pas demandé plutôt que de l'appliquer : vous êtes invité à l'approbation à la place, ou l'action est auto-refusée en [mode `dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode).

Les blocages répétés signifient généralement que le classificateur manque de contexte sur votre infrastructure. Utilisez `/feedback` pour signaler les faux positifs, ou demandez à un administrateur de [configurer l'infrastructure approuvée](/docs/fr/auto-mode-config).

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="Comment le classificateur évalue les actions">
    Chaque action passe par un ordre de décision fixe. La première étape correspondante gagne :

    1. Les actions correspondant à vos [règles allow, ask ou deny](/docs/fr/permissions#manage-permissions) se résolvent immédiatement, avec ces exceptions :
       * Les écritures vers [chemins protégés](#protected-paths) sont acheminées vers le classificateur même lorsqu'une règle allow correspond, et il en va de même pour les suppressions `rm` et `rmdir` ciblant un [chemin critique](#critical-paths) dans Claude Code v2.1.218 et ultérieur
       * Les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) vous invitent directement même lorsqu'une règle allow correspond, et il en va de même pour les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) dans les sessions où ce paramètre atteint Claude Code
       * Une commande shell qui porte [domaines autorisés par commande](/docs/fr/sandboxing#per-command-allowed-domains-in-auto-mode) est également acheminée vers le classificateur même lorsqu'une règle allow correspond, parce qu'une règle approuve la commande, pas ses hôtes
       * Les règles ask qui correspondent sur le contenu d'une commande, comme `Bash(git push *)`, reviennent à une invite de permission
    2. Les actions en lecture seule et les éditions de fichiers dans votre répertoire de travail sont auto-approuvées, sauf les écritures vers [chemins protégés](#protected-paths) et [la première lecture en dehors des répertoires de travail](#first-read-outside-the-working-directories), qui vous invitent
       * Dans une session avec [examen du classificateur côté serveur](#server-side-classifier-review), les actions en lecture seule et les commandes shell [en sandbox](/docs/fr/sandboxing#sandbox-modes) attendent cet examen et sont bloquées si elle les signale
    3. Tout le reste va au classificateur. Les outils connecteur et les outils MCP `requiresUserInteraction` qui vous invitent directement à l'étape 1 n'atteignent jamais le classificateur, donc ni une approbation requise par l'organisation ni une étape de consentement n'est auto-approuvée
    4. Si le classificateur bloque, Claude reçoit la raison et essaie une alternative. Dans la plupart des sessions, la raison nomme la règle que le classificateur a correspondante, comme `[Data Exfiltration]`, plutôt que de donner une explication écrite ; voir [Examiner les refus](/docs/fr/auto-mode-config#review-denials)

    En entrant en mode auto, les règles allow larges qui accordent l'exécution de code arbitraire sont supprimées :

    * Blanket `Bash(*)` ou `PowerShell(*)`
    * Interpréteurs avec caractères génériques comme `Bash(python*)`
    * Commandes d'exécution du gestionnaire de packages
    * Règles `Agent` allow
    * Règles [`Monitor`](/docs/fr/tools-reference#monitor-tool) allow, parce que Claude Code exécute les commandes Monitor via le shell

    Les règles étroites comme `Bash(npm test)` restent en vigueur. Claude Code restaure les règles supprimées lorsque vous quittez le mode auto. Avant v2.1.236, Claude Code laissait les règles `Monitor` allow en vigueur en mode auto, donc une règle qui correspondait à l'outil entier approuvait les commandes Monitor sans examen du classificateur.

    Claude Code exécute également `git status` lui-même avant une commande qui éliminerait le travail non validé, comme `git reset --hard` ou `rm -rf`, et montre au classificateur si du travail préparé, modifié ou non suivi est présent. Claude Code signale les fichiers non suivis dans cette vérification même lorsque la configuration git du dépôt définit `status.showUntrackedFiles=no`.

    Dans les demandes de classificateur envoyées par Claude Code lui-même, le classificateur voit les messages utilisateur, les appels d'outils autres que les recherches en lecture seule comme les lectures de fichiers et les recherches, et votre contenu CLAUDE.md. Les résultats d'outils sont supprimés de ces demandes, donc le contenu hostile dans un fichier ou une page web ne peut pas manipuler le classificateur directement.

    Vous pouvez annoter le résultat d'un appel avec le champ `classifierContext` d'un [hook PostToolUse](/docs/fr/hooks#annotate-a-result-for-the-auto-mode-classifier), que le classificateur lit comme contexte fourni par l'application. Le champ nécessite Claude Code v2.1.236 ou ultérieur.

    Une sonde côté serveur distincte analyse les résultats d'outils entrants et signale le contenu suspect avant que Claude ne le lise. Pour plus d'informations sur la façon dont ces couches fonctionnent ensemble, consultez l'[annonce du mode auto](https://claude.com/blog/auto-mode) et la [plongée technique](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Comment le mode auto gère les sous-agents">
    Le classificateur vérifie le travail des [sous-agents](/docs/fr/sub-agents) à trois points :

    1. Avant qu'un sous-agent ne démarre, la description de la tâche déléguée est évaluée, donc une tâche qui semble dangereuse est bloquée au moment du lancement.
    2. Pendant que le sous-agent s'exécute, chacune de ses actions passe par le classificateur avec les mêmes règles que la session parent, et tout `permissionMode` dans le frontmatter du sous-agent est ignoré.
    3. Lorsque le sous-agent se termine, le classificateur examine son travail et son rapport final avant que le parent ne lise le rapport. Lorsque le classificateur signale le travail ou le rapport du sous-agent, ou qu'une vérification de sécurité API distincte refuse l'examen, le rapport est toujours livré, précédé d'un avertissement de sécurité. Lorsque le classificateur n'est pas disponible pour l'examen, le rapport arrive avec une note pour vérifier le travail du sous-agent avant d'agir en fonction de celui-ci.
  </Accordion>

  <Accordion title="Coût et latence">
    Le classificateur s'exécute sur Claude Sonnet 5 par défaut plutôt que sur votre sélection `/model`. Un modèle de classificateur que Anthropic configure côté serveur prime sur ce défaut. Lorsque le modèle de votre session est Claude Sonnet 4.6, ou lorsque [`availableModels`](/docs/fr/model-config#restrict-model-selection) exclut Sonnet 5, le classificateur s'exécute sur le modèle de la session à la place, ou sur un modèle Opus lorsque la session s'exécute sur un [modèle Fable](/docs/fr/model-config#work-with-fable) ; sur les fournisseurs autres que l'API Anthropic, ce retour Opus est le modèle Opus par défaut du fournisseur.

    La première demande en mode auto de la session valide le défaut Sonnet 5 : si la demande réussit, Sonnet 5 reste le modèle de classificateur de la session, et si elle échoue parce que le modèle n'est pas disponible, la session utilise le retour à la place. Après que cette validation se règle, le modèle du classificateur ne change pas pour la session.

    Sur les plans Enterprise et sur les comptes qui utilisent l'API Claude, [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws), Amazon Bedrock, la plateforme Agent de Google Cloud ou Microsoft Foundry, les appels de classificateur comptent vers votre utilisation de tokens. Chaque vérification envoie une partie de la transcription plus l'action en attente, ajoutant un aller-retour avant l'exécution. Les lectures et les éditions de répertoire de travail en dehors des chemins protégés ignorent le classificateur, donc la surcharge provient principalement des commandes shell et des opérations réseau. Là où le serveur examine les actions dans le cadre des demandes de modèle de la session, il n'y a pas d'appels de classificateur distincts à compter ; voir [Examen du classificateur côté serveur](#server-side-classifier-review).

    L'accès réseau en sandbox n'ajoute pas de demandes de classificateur par connexion. Le classificateur juge [les hôtes qu'une commande nomme](/docs/fr/sandboxing#per-command-allowed-domains-in-auto-mode) ensemble avec la commande dans un examen, et Claude Code vérifie chaque connexion par rapport à la liste approuvée sans appeler le classificateur à nouveau.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Autoriser uniquement les outils pré-approuvés avec le mode dontAsk
</h2>

Si vous définissez le mode `dontAsk`, Claude Code refuse automatiquement chaque appel d'outil qui déclencherait autrement une invite. Claude exécute toujours les actions qui ne nécessitent aucune approbation en mode Manual, telles que les lectures de fichiers dans vos répertoires de travail et les [commandes Bash en lecture seule](/docs/fr/permissions#read-only-commands), ainsi que les actions correspondant à vos règles `permissions.allow` et les appels approuvés par un [hook PreToolUse](/docs/fr/permissions#extend-permissions-with-hooks). Utilisez ce mode pour les pipelines CI ou les environnements restreints où vous prédéfinissez ce que Claude peut faire ; la session n'attend jamais d'entrée. La barre d'état affiche `⏵⏵ don't ask on` tandis que ce mode est actif.

Claude Code refuse les appels correspondant à vos [règles `ask` explicites](/docs/fr/permissions#manage-permissions) plutôt que de déclencher une invite. Il refuse également l'outil intégré `AskUserQuestion` même si vos règles allow les correspondent, et il en va de même pour les outils de connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) dans les sessions où ce paramètre atteint Claude Code. Il refuse les outils MCP marqués [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool) de la même manière, car leur carte d'approbation nécessite une réponse que ce mode ne collecte jamais ; cela nécessite Claude Code v2.1.199 ou ultérieur.

Les suppressions `rm` et `rmdir` ciblant un [chemin critique](#critical-paths), comme `rm -rf /` et `rm -rf ~`, sont refusées même quand une règle allow les correspond ou qu'un hook `PreToolUse` les approuve.

Les sessions cloud sur [Claude Code sur le web](/docs/fr/claude-code-on-the-web) ignorent `defaultMode: "dontAsk"` ; voir [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) pour les détails.

Définissez-le au démarrage avec le drapeau :

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Ignorer tous les contrôles avec le mode bypassPermissions
</h2>

Le mode `bypassPermissions` désactive les invites de permission et les contrôles de sécurité afin que les appels d'outils s'exécutent immédiatement, y compris les écritures vers les [chemins protégés](#protected-paths).

Les [actions qu'aucun mode n'auto-approuve](#actions-no-mode-auto-approves) invitent toujours dans ce mode.

Deux [protections de messagerie inter-sessions](/docs/fr/cross-session-messaging) s'appliquent toujours dans ce mode, et dans les sessions en mode plan interactif où les permissions de contournement sont disponibles :

* L'invite d'approbation [`isolatePeerMachines`](/docs/fr/settings-reference#isolatepeermachines) pour les messages vers vos sessions au-delà de cette machine apparaît toujours.
* Quand aucune valeur [`crossSessionInbound`](/docs/fr/cross-session-messaging#control-inbound-messages) ne s'applique, Claude Code retient un message entrant d'une autre de vos sessions pour votre approbation, et le livre sans demander uniquement quand la session d'envoi s'identifie comme contournant également les invites de permission. Si vous quittez le mode de permission tandis que les messages sont retenus, Claude Code réapplique les règles entrantes et livre tout message retenu qu'elles acceptent maintenant.

Dans les sessions de terminal interactif avec les permissions de contournement disponibles, Claude Code n'applique pas non plus les [blocs du mode plan](#analyze-before-you-edit-with-plan-mode). Claude est toujours instruit de planifier sans modifier, mais une modification de fichier ou une commande shell qu'il tente pendant la planification s'exécute sans inviter. Les [règles ask](/docs/fr/permissions#manage-permissions) explicites et les suppressions `rm` et `rmdir` ciblant un [chemin critique](#critical-paths) invitent toujours.

Le mode plan conserve ses blocs partout où Claude Code s'exécute sans terminal interactif, y compris les [exécutions non-interactives](/docs/fr/headless) avec `-p`, les sessions du [SDK Agent](/docs/fr/agent-sdk/permissions#plan-mode-plan), et les conversations dans le [panneau de chat](/docs/fr/vs-code) de l'extension VS Code. Là, `--allow-dangerously-skip-permissions` rend `bypassPermissions` sélectionnable ultérieurement.

<Warning>
  Utilisez ce mode uniquement dans des environnements isolés comme les conteneurs, les machines virtuelles ou les dev containers sans accès à Internet, où Claude Code ne peut pas endommager votre système hôte.
</Warning>

Vous ne pouvez pas entrer dans `bypassPermissions` à partir d'une session que vous avez démarrée sans l'activer. Activez-le au lancement avec [`permissions.defaultMode: "bypassPermissions"`](/docs/fr/settings-reference#permissions-defaultmode) ou avec un drapeau d'activation :

```bash theme={null}
claude --permission-mode bypassPermissions
```

Le drapeau `--dangerously-skip-permissions` est équivalent.

Claude Code refuse `bypassPermissions` dans une session que vous démarrez avec [`--restricted`](/docs/fr/cli-reference#cli-flags). `--restricted` nécessite Claude Code v2.1.248 ou ultérieur.

La première fois que vous démarrez une session interactive avec ce mode activé, Claude Code affiche un dialogue d'avertissement vous demandant d'accepter la responsabilité des actions prises sans vérifications de permission. Claude Code enregistre votre acceptation dans les paramètres utilisateur, donc le dialogue n'apparaît qu'une fois. Si vous refusez, Claude Code quitte. En [mode non-interactif](/docs/fr/headless), aucun dialogue n'est affiché, et une [session en arrière-plan](/docs/fr/agent-view) démarrée avec `--bg` est refusée jusqu'à ce que vous ayez accepté le dialogue dans une session interactive.

Sur Linux et macOS, Claude Code refuse de démarrer dans ce mode lors de l'exécution en tant que root ou sous `sudo` :

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

La vérification est ignorée automatiquement à l'intérieur d'un sandbox reconnu. Pour s'exécuter de manière autonome dans un conteneur, utilisez la configuration du [dev container](/docs/fr/devcontainer), qui exécute Claude Code en tant qu'utilisateur non-root.

[Claude Code sur le web](/docs/fr/claude-code-on-the-web) n'honore pas `defaultMode: "bypassPermissions"` ou `"dontAsk"` de vos fichiers de paramètres, donc les paramètres archivés d'un référentiel ne peuvent pas démarrer une session cloud en mode bypass-permissions. Le paramètre est ignoré silencieusement et la session démarre dans le mode affiché dans la liste déroulante des modes à la place. Voir [Basculer les modes de permission](#switch-permission-modes) pour connaître les modes que les sessions cloud proposent.

<Warning>
  `bypassPermissions` n'offre aucune protection contre l'injection de prompt ou les actions involontaires. Pour les contrôles de sécurité en arrière-plan avec beaucoup moins d'invites de permission, utilisez le [mode auto](#eliminate-prompts-with-auto-mode) à la place. Les administrateurs peuvent bloquer ce mode en définissant `permissions.disableBypassPermissionsMode` sur `"disable"` dans les [paramètres gérés](/docs/fr/managed-settings).
</Warning>

<h2 id="protected-paths">
  Chemins protégés
</h2>

Les écritures vers un petit ensemble de chemins ne sont jamais approuvées automatiquement, sauf en mode `bypassPermissions` et dans les sessions de terminal interactif en mode plan avec les [autorisations de contournement](#skip-all-checks-with-bypasspermissions-mode) disponibles. Cela empêche la corruption accidentelle de l'état du référentiel et de la configuration propre de Claude.

| Mode                     | Écritures de chemins protégés                                                                                                                                                                                                                                                                                                   |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`, `acceptEdits` | Demandé                                                                                                                                                                                                                                                                                                                         |
| `plan`                   | Autorisé dans les sessions de terminal interactif avec les [autorisations de contournement](#skip-all-checks-with-bypasspermissions-mode) disponibles. Sinon, acheminé vers le classificateur quand le [mode auto](#eliminate-prompts-with-auto-mode) est disponible pendant la planification, et demandé quand il ne l'est pas |
| `auto`                   | Acheminé vers le classificateur                                                                                                                                                                                                                                                                                                 |
| `dontAsk`                | Refusé                                                                                                                                                                                                                                                                                                                          |
| `bypassPermissions`      | Autorisé                                                                                                                                                                                                                                                                                                                        |

Dans une session démarrée avec [`--restricted`](/docs/fr/cli-reference#cli-flags), qui nécessite Claude Code v2.1.248 ou version ultérieure, le classificateur ne peut pas approuver les écritures de chemins protégés.

Les règles [`permissions.allow`](/docs/fr/permissions#manage-permissions) dans les fichiers de paramètres ne pré-approuvent pas les écritures de chemins protégés. La vérification de sécurité s'exécute avant que Claude Code n'évalue les règles d'autorisation des paramètres, donc une entrée telle que `Edit(.claude/**)` dans `~/.claude/settings.json` ou `.claude/settings.json` ne change pas le résultat par mode dans le tableau ci-dessus. Dans les modes qui demandent, l'invite pour une écriture `.claude/` offre **Oui, et autoriser Claude à modifier ses propres paramètres pour cette session**, ce qui approuve les écritures `.claude/` ultérieures dans cette session sans demander à nouveau.

Répertoires protégés :

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, sauf pour `.claude/worktrees` où Claude stocke ses propres git worktrees

Fichiers protégés :

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="critical-paths">
  Chemins critiques
</h2>

Claude Code ne laisse jamais une [règle `permissions.allow`](/docs/fr/permissions#manage-permissions) ou un [hook `PreToolUse`](/docs/fr/permissions#extend-permissions-with-hooks) qui retourne `"allow"` approuver une commande `rm` ou `rmdir` qui cible un chemin critique, même dans les modes qui sautent d'autres invites. Ce disjoncteur protège contre l'erreur du modèle. Une règle deny correspondante bloque toujours la commande complètement.

Ce qui se passe à la place dépend de votre mode de permission :

| Mode                     | Ce que Claude Code fait avec une suppression de chemin critique                                                                                                                                                       |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Vous demande de l'approuver                                                                                                                                                                                           |
| `plan`                   | Vous demande de l'approuver. Avec le [mode auto disponible pendant la planification](#analyze-before-you-edit-with-plan-mode) et aucune permission de contournement disponible, l'envoie au classificateur à la place |
| `auto`                   | L'envoie au [classificateur](#eliminate-prompts-with-auto-mode)                                                                                                                                                       |
| `dontAsk`                | La refuse                                                                                                                                                                                                             |
| `bypassPermissions`      | Vous demande de l'approuver                                                                                                                                                                                           |

Si une [règle ask](/docs/fr/permissions#manage-permissions) explicite correspond à la commande, Claude Code vous demande même en mode `auto`. Dans les modes qui demandent, un [hook `PermissionRequest`](/docs/fr/hooks#permissionrequest) peut répondre à l'invite de la même manière qu'il répond à n'importe quelle autre.

Claude Code traite une cible `rm` ou `rmdir` comme un chemin critique quand il s'agit de l'un des éléments suivants :

* La racine du système de fichiers
* Les répertoires de niveau supérieur, ce qui signifie tout enfant direct de la racine, comme `/usr`, `/etc`, ou `/data`
* Votre répertoire personnel
* Les racines des lecteurs Windows et leurs répertoires de niveau supérieur, comme `C:\` et `C:\Windows`
* Votre répertoire de travail et ses parents
* Vos répertoires de travail supplémentaires et leurs parents, mais uniquement quand la suppression est un glob sous l'un d'eux, comme `rm -rf <dir>/*`. `rm -rf <dir>` sur le répertoire lui-même ne déclenche pas cette vérification

Claude Code traite également un glob ou une barre oblique finale directement sous une variable shell, comme `rm -rf "$DIR"/*`, comme une suppression de chemin critique, car la commande devient une suppression de la racine du système de fichiers quand la variable est vide.

L'invite pour ce cas de variable nomme la commande `rm` signalée et indique comment la réécrire pour que la vérification passe :

* Pour une variable comme `$DIR`, protégez chaque expansion pour que le shell s'arrête avec une erreur quand la variable n'est pas définie ou vide, comme dans `rm -rf "${DIR:?}"/*`, ou utilisez un chemin littéral
* Pour une variable qui est normalement définie, comme `$HOME`, utilisez un chemin littéral

Une suppression dont les expansions sont toutes protégées de cette manière n'est pas une suppression de chemin critique, donc en mode `bypassPermissions` elle s'exécute sans invite.

Masquer la suppression à l'intérieur d'une sous-coquille avec `(...)`, un groupe d'accolades avec `{ ...; }`, une substitution de commande avec `$(...)` ou des backticks, ou une substitution de processus avec `<(...)`, ne saute pas la vérification. Claude Code trouve une suppression de chemin critique qu'elle se trouve à l'intérieur de la forme imbriquée, comme dans `(rm -rf ~)` ou `echo "$(rm -rf ~)"`, ou ailleurs dans la même commande.

<h3 id="remove-item-in-powershell">
  Remove-Item dans PowerShell
</h3>

Quand vous activez l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool), Claude Code donne à `Remove-Item` sa propre vérification, distincte de la liste des chemins critiques `rm`. Le résultat dépend de la cible, et le premier cas correspondant s'applique :

* **Chemins système** : la racine du système de fichiers et ses répertoires de niveau supérieur, les racines des lecteurs et leurs répertoires de niveau supérieur, et votre répertoire personnel. Claude Code refuse la commande dans tous les modes, sans vous demander.
* **Wildcards** : un `*` nu, ou n'importe quelle cible se terminant par `/*` ou `\*`, y compris un glob sous une variable shell comme `$dir/*`. Claude Code refuse la commande dans tous les modes, sans vous demander, avant que le [classificateur](#eliminate-prompts-with-auto-mode) ne la voie.
* **Votre répertoire de travail ou l'un de ses parents, avec `-Recurse`** : Claude Code traite la commande comme n'importe quelle autre qui nécessite une approbation dans votre mode de permission, donc elle vous demande dans les modes qui demandent, l'envoie au classificateur en mode `auto`, et la refuse en mode `dontAsk`. Le mode `bypassPermissions` saute cette vérification.

<h2 id="see-also">
  Voir aussi
</h2>

* [Permissions](/docs/fr/permissions) : règles allow, ask et deny ; politiques gérées
* [Configurer le mode auto](/docs/fr/auto-mode-config) : indiquez au classificateur l'infrastructure de confiance de votre organisation
* [Hooks](/docs/fr/hooks) : logique de permission personnalisée via les hooks `PreToolUse` et `PermissionRequest`
* [Sécurité](/docs/fr/security) : protections et bonnes pratiques
* [Sandboxing](/docs/fr/sandboxing) : isolation du système de fichiers et du réseau pour les commandes Bash
* [Mode non-interactif](/docs/fr/headless) : exécutez Claude Code avec le flag `-p`
