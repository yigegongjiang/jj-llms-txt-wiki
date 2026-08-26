> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Créer des sous-agents personnalisés

> Créez et utilisez des sous-agents IA spécialisés dans Claude Code pour des workflows spécifiques à des tâches et une meilleure gestion du contexte.

Les sous-agents sont des assistants IA spécialisés qui gèrent des types de tâches spécifiques. Utilisez-en un lorsqu'une tâche secondaire inonderait votre conversation principale avec des résultats de recherche, des journaux ou des contenus de fichiers que vous ne référencerez plus : le sous-agent effectue ce travail dans son propre contexte et retourne uniquement le résumé. Définissez un sous-agent personnalisé lorsque vous générez constamment le même type de travailleur avec les mêmes instructions.

Chaque sous-agent s'exécute dans sa propre fenêtre de contexte avec une invite système personnalisée, un accès à des outils spécifiques et des permissions indépendantes. Lorsque Claude rencontre une tâche qui correspond à la description d'un sous-agent, il délègue à ce sous-agent, qui fonctionne indépendamment et retourne les résultats. Pour voir les économies de contexte en pratique, la [visualisation de la fenêtre de contexte](/docs/fr/context-window) vous guide à travers une session où un sous-agent gère la recherche dans sa propre fenêtre séparée.

<Note>
  Les sous-agents fonctionnent au sein d'une seule session. Pour exécuter de nombreuses sessions indépendantes en parallèle et les surveiller depuis un seul endroit, consultez [les agents en arrière-plan](/docs/fr/agent-view). Pour les sessions qui communiquent entre elles, consultez [les équipes d'agents](/docs/fr/agent-teams).
</Note>

Les sous-agents vous aident à :

* **Préserver le contexte** en gardant l'exploration et l'implémentation en dehors de votre conversation principale
* **Appliquer des contraintes** en limitant les outils qu'un sous-agent peut utiliser
* **Réutiliser les configurations** dans les projets avec des sous-agents au niveau utilisateur
* **Spécialiser le comportement** avec des invites système ciblées pour des domaines spécifiques
* **Contrôler les coûts** en acheminant les tâches vers des modèles plus rapides et moins chers comme Haiku

Claude utilise la description de chaque sous-agent pour décider quand déléguer les tâches. Lorsque vous créez un sous-agent, écrivez une description claire pour que Claude sache quand l'utiliser.

Claude Code inclut plusieurs sous-agents intégrés comme Explore, Plan et general-purpose. Vous pouvez également créer des sous-agents personnalisés pour gérer des tâches spécifiques.

<h2 id="built-in-subagents">
  Sous-agents intégrés
</h2>

Claude Code inclut des sous-agents intégrés que Claude utilise automatiquement le cas échéant. Chacun hérite des permissions de la conversation parent avec des restrictions d'outils supplémentaires.

Explore et Plan ignorent vos fichiers CLAUDE.md et l'état git de la session parent pour maintenir la recherche rapide et économique. Tous les autres sous-agents intégrés et [sous-agents personnalisés](#configure-subagents) chargent les deux. Pour la ventilation complète de ce qui atteint un sous-agent, consultez [ce qui se charge au démarrage](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Un agent rapide et en lecture seule optimisé pour la recherche et l'analyse de bases de code.

    * **Modèle** : hérité de la conversation principale, limité à Opus sur l'API Claude, donc Explore ne s'exécute jamais sur un modèle plus coûteux que celui que vous avez déjà choisi pour la session
    * **Outils** : outils en lecture seule ; Write et Edit sont refusés
    * **Objectif** : découverte de fichiers, recherche de code, exploration de base de code

    À partir de la v2.1.198, Explore hérite du modèle de la conversation principale au lieu de toujours s'exécuter sur Haiku. Sur l'API Claude, le modèle hérité est limité à Opus : une conversation principale sur un niveau supérieur exécute Explore sur Opus, et une conversation principale sur Sonnet ou Haiku exécute Explore sur ce même modèle. Sur tout autre fournisseur, tel que [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou Claude Platform on AWS](/docs/fr/third-party-integrations), Explore hérite directement du modèle de la conversation principale.

    Un [sous-agent utilisateur ou projet](#choose-the-subagent-scope) nommé `Explore` remplace le sous-agent intégré et conserve son propre champ `model`, donc définissez-en un avec `model: haiku` pour maintenir l'exploration sur un modèle moins coûteux.

    Claude délègue à Explore lorsqu'il doit rechercher ou comprendre une base de code sans apporter de modifications. Cela garde les résultats d'exploration en dehors du contexte de votre conversation principale.

    Lors de l'invocation d'Explore, Claude spécifie un niveau de minutie : **quick** pour les recherches ciblées, **medium** pour l'exploration équilibrée, ou **very thorough** pour l'analyse complète.
  </Tab>

  <Tab title="Plan">
    Un agent de recherche utilisé pendant le [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) pour rassembler le contexte avant de présenter un plan.

    * **Modèle** : hérité de la conversation principale
    * **Outils** : outils en lecture seule ; Write et Edit sont refusés
    * **Objectif** : recherche de base de code pour la planification

    Lorsque vous êtes en mode plan et que Claude doit comprendre votre base de code, il délègue la recherche au sous-agent Plan afin que la sortie d'exploration reste dans une fenêtre de contexte séparée tandis que la conversation principale reste en lecture seule.
  </Tab>

  <Tab title="General-purpose">
    Un agent capable pour les tâches complexes et multi-étapes qui nécessitent à la fois l'exploration et l'action.

    * **Modèle** : hérité de la conversation principale
    * **Outils** : tous les outils
    * **Objectif** : recherche complexe, opérations multi-étapes, modifications de code

    Claude délègue à general-purpose lorsque la tâche nécessite à la fois l'exploration et la modification, un raisonnement complexe pour interpréter les résultats, ou plusieurs étapes dépendantes.
  </Tab>

  <Tab title="Other">
    Claude Code inclut des agents d'assistance supplémentaires pour des tâches spécifiques. Ceux-ci sont généralement invoqués automatiquement, vous n'avez donc pas besoin de les utiliser directement.

    | Agent             | Modèle | Quand Claude l'utilise                                                  |
    | :---------------- | :----- | :---------------------------------------------------------------------- |
    | statusline-setup  | Sonnet | Lorsque vous exécutez `/statusline` pour configurer votre ligne d'état  |
    | claude-code-guide | Haiku  | Lorsque vous posez des questions sur les fonctionnalités de Claude Code |
  </Tab>
</Tabs>

Les sous-agents intégrés sont enregistrés par défaut dans les sessions interactives. Pour les restreindre :

* Pour bloquer un type intégré spécifique, ajoutez-le à `permissions.deny` comme indiqué dans [Désactiver des sous-agents spécifiques](#disable-specific-subagents).
* Pour empêcher Claude de déléguer à un sous-agent, refusez l'outil `Agent` lui-même avec [`permissions.deny`](/docs/fr/permissions#tool-specific-permission-rules).
* Pour supprimer uniquement les sous-agents intégrés `Explore` et `Plan`, définissez [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/fr/env-vars). Claude lit et explore les fichiers directement au lieu de déléguer à ces sous-agents. Nécessite Claude Code v2.1.198 ou version ultérieure.
* En [mode non-interactif](/docs/fr/headless) et le [SDK Agent](/docs/fr/agent-sdk/overview), définissez [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/fr/env-vars) pour supprimer tous les types intégrés et fournir uniquement les vôtres.

Au-delà de ces sous-agents intégrés, vous pouvez créer les vôtres avec des invites personnalisées, des restrictions d'outils, des modes de permission, des hooks et des skills. Les sections suivantes montrent comment commencer et personnaliser les sous-agents.

<h2 id="quickstart-create-your-first-subagent">
  Démarrage rapide : créer votre premier sous-agent
</h2>

Les sous-agents sont des fichiers Markdown avec du frontmatter YAML. Pour en créer un, demandez à Claude de l'écrire pour vous, ou [écrivez le fichier vous-même](#write-subagent-files).

À partir de la v2.1.198, la commande `/agents` n'ouvre plus l'assistant de création interactif ; l'exécuter affiche un rappel pour demander à Claude ou modifier `.claude/agents/` directement. Les fichiers de sous-agents, les champs de frontmatter, et les emplacements `.claude/agents/` et `~/.claude/agents/` restent inchangés ; seul l'assistant terminal est supprimé.

Cette procédure pas à pas crée un sous-agent au niveau utilisateur qui examine le code et suggère des améliorations.

<Steps>
  <Step title="Demander à Claude de créer le sous-agent">
    Dans Claude Code, décrivez le sous-agent que vous souhaitez et où l'enregistrer :

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude écrit le fichier avec un `name`, une `description`, une liste `tools`, un `model`, et une invite système.
  </Step>

  <Step title="Examiner le fichier">
    Ouvrez `~/.claude/agents/code-improver.md` et confirmez que le frontmatter correspond à ce que vous avez demandé. Le résultat ressemble à ceci :

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    Parce que le fichier se trouve dans `~/.claude/agents/`, le sous-agent est disponible dans chaque projet sur votre machine. Pour le limiter à un seul projet, déplacez-le vers le répertoire `.claude/agents/` de ce projet. [Choisir la portée du sous-agent](#choose-the-subagent-scope) compare les deux.
  </Step>

  <Step title="L'essayer">
    Demandez à Claude de déléguer au nouveau sous-agent :

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude délègue à votre nouveau sous-agent, qui analyse la base de code et retourne les suggestions d'amélioration.

    Si Claude ne trouve pas le nouveau sous-agent, redémarrez Claude Code et réessayez. Cela se produit uniquement lorsque `~/.claude/agents/` n'existait pas avant le démarrage de la session, car une session en cours ne détecte pas un répertoire `agents` nouvellement créé.
  </Step>
</Steps>

Vous avez maintenant un sous-agent que vous pouvez utiliser dans n'importe quel projet sur votre machine pour analyser les bases de code et suggérer des améliorations.

Vous pouvez également écrire des fichiers de sous-agents à la main, les définir via des drapeaux CLI, ou les distribuer via des plugins. Les sections suivantes couvrent toutes les options de configuration.

<Note>
  Sur Claude Code v2.1.197 et antérieur, `/agents` ouvre un assistant interactif avec un onglet **Running** qui liste les sous-agents actifs et un onglet **Library** pour les créer, les modifier et les supprimer.&#x20;
</Note>

<h2 id="configure-subagents">
  Configurer les sous-agents
</h2>

La localisation d'un fichier de sous-agent détermine qui y a accès, et son frontmatter détermine ce qu'il peut faire. Cette section couvre l'emplacement des fichiers de sous-agent et chaque champ qu'ils prennent en charge.

<h3 id="choose-the-subagent-scope">
  Choisir la portée du sous-agent
</h3>

Stockez les fichiers de sous-agent dans différents emplacements selon la portée. Lorsque plusieurs sous-agents partagent le même nom, Claude Code utilise celui de l'emplacement de priorité plus élevée.

| Emplacement                    | Portée                        | Priorité           | Comment créer                                       |
| :----------------------------- | :---------------------------- | :----------------- | :-------------------------------------------------- |
| Paramètres gérés               | À l'échelle de l'organisation | 1 (la plus élevée) | Déployé via [paramètres gérés](/docs/fr/settings)        |
| Drapeau CLI `--agents`         | Session actuelle              | 2                  | Passer JSON lors du lancement de Claude Code        |
| `.claude/agents/`              | Projet actuel                 | 3                  | Demander à Claude, ou créer le fichier manuellement |
| `~/.claude/agents/`            | Tous vos projets              | 4                  | Demander à Claude, ou créer le fichier manuellement |
| Répertoire `agents/` du plugin | Où le plugin est activé       | 5 (la plus basse)  | Installé avec les [plugins](/docs/fr/plugins)            |

**Les sous-agents de projet** (`.claude/agents/`) sont idéaux pour les sous-agents spécifiques à une base de code. Enregistrez-les dans le contrôle de version pour que votre équipe puisse les utiliser et les améliorer de manière collaborative.

Les sous-agents de projet sont découverts en remontant à partir du répertoire de travail actuel, donc chaque `.claude/agents/` entre celui-ci et la racine du référentiel est analysé. À partir de la v2.1.178, lorsque plusieurs de ces répertoires imbriqués définissent le même `name`, Claude Code utilise la définition la plus proche du répertoire de travail.

Les répertoires ajoutés avec `--add-dir` sont également analysés : un dossier `.claude/agents/` à l'intérieur d'un répertoire ajouté se charge aux côtés des sous-agents de projet. Consultez [Répertoires supplémentaires](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) pour voir quels autres types de configuration se chargent à partir de `--add-dir`. Pour partager les sous-agents entre les projets sans `--add-dir`, utilisez `~/.claude/agents/` ou un [plugin](/docs/fr/plugins).

**Les sous-agents utilisateur** (`~/.claude/agents/`) sont des sous-agents personnels disponibles dans tous vos projets.

Claude Code analyse `.claude/agents/` et `~/.claude/agents/` de manière récursive, vous pouvez donc organiser les définitions dans des sous-dossiers tels que `agents/review/` ou `agents/research/`. Le chemin du sous-répertoire n'affecte pas la façon dont un sous-agent est identifié ou invoqué, car l'identité provient uniquement du champ frontmatter `name`.

Gardez les valeurs `name` uniques dans tout l'arborescence : si deux fichiers dans une même portée déclarent le même nom, Claude Code en charge un seul, choisi par l'ordre de lecture du système de fichiers plutôt que par une précédence documentée. Entre les répertoires de projet imbriqués, la définition la plus proche du répertoire de travail gagne, comme décrit ci-dessus. La vérification de configuration [`/doctor`](/docs/fr/commands#all-commands) signale les fichiers dans le même répertoire qui partagent un nom et propose de renommer ou de supprimer tous sauf un. Avant la v2.1.205, `/doctor` ouvrait un écran de diagnostics qui listait les doublons et montrait quelle définition était active.

Les répertoires `agents/` des plugins sont également analysés de manière récursive. Contrairement aux portées de projet et utilisateur, un sous-dossier à l'intérieur du répertoire `agents/` d'un plugin devient partie de l'[identifiant limité](#invoke-subagents-explicitly) : un fichier à `agents/review/security.md` dans le plugin `my-plugin` s'enregistre comme `my-plugin:review:security`.

**Les sous-agents définis par CLI** sont passés en JSON lors du lancement de Claude Code. Ils n'existent que pour cette session et ne sont pas enregistrés sur le disque, ce qui les rend utiles pour les tests rapides ou les scripts d'automatisation. Vous pouvez définir plusieurs sous-agents dans un seul appel `--agents` :

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

Le drapeau `--agents` accepte JSON avec les mêmes champs de [frontmatter](#supported-frontmatter-fields) que les sous-agents basés sur fichier : `description`, `prompt`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `isolation` et `color`. Utilisez `prompt` pour l'invite système, équivalent au corps markdown dans les sous-agents basés sur fichier.

**Les sous-agents gérés** sont déployés par les administrateurs de l'organisation. Placez les fichiers markdown dans `.claude/agents/` à l'intérieur du [répertoire des paramètres gérés](/docs/fr/settings#settings-files), en utilisant le même format de frontmatter que les sous-agents de projet et utilisateur. Les définitions gérées prennent précédence sur les sous-agents de projet et utilisateur portant le même nom.

**Les sous-agents de plugin** proviennent des [plugins](/docs/fr/plugins) que vous avez installés. Ils se chargent aux côtés de vos sous-agents personnalisés et apparaissent dans la saisie semi-automatique @-mention sous leur nom limité. Consultez la [référence des composants de plugin](/docs/fr/plugins-reference#agents) pour plus de détails sur la création de sous-agents de plugin.

<Note>
  Pour des raisons de sécurité, les sous-agents de plugin ne prennent pas en charge les champs frontmatter `hooks`, `mcpServers` ou `permissionMode`. Ces champs sont ignorés lors du chargement des agents à partir d'un plugin. Si vous en avez besoin, copiez le fichier d'agent dans `.claude/agents/` ou `~/.claude/agents/`. Vous pouvez également ajouter des règles à [`permissions.allow`](/docs/fr/settings#permission-settings) dans `settings.json` ou `settings.local.json`, mais ces règles s'appliquent à l'ensemble de la session, pas seulement au sous-agent du plugin.
</Note>

Les définitions de sous-agent de l'une de ces portées sont également disponibles pour les [équipes d'agents](/docs/fr/agent-teams#use-subagent-definitions-for-teammates) : lors du lancement d'un coéquipier, vous pouvez référencer un type de sous-agent et le coéquipier utilise ses `tools` et son `model`, avec le corps de la définition ajouté à l'invite système du coéquipier comme instructions supplémentaires. Consultez [équipes d'agents](/docs/fr/agent-teams#use-subagent-definitions-for-teammates) pour voir quels champs de frontmatter s'appliquent sur ce chemin.

<h3 id="write-subagent-files">
  Écrire des fichiers de sous-agent
</h3>

Les fichiers de sous-agent utilisent du frontmatter YAML pour la configuration, suivi de l'invite système en Markdown :

<Note>
  Claude Code surveille `~/.claude/agents/` et `.claude/agents/`. Lorsque vous ajoutez ou modifiez un fichier de sous-agent sur le disque, ou demandez à Claude d'en écrire un pour vous, Claude Code détecte le changement en quelques secondes et la prochaine délégation utilise la définition mise à jour, sans redémarrage nécessaire.

  Deux cas nécessitent toujours un redémarrage :

  * L'observateur couvre uniquement les répertoires qui existaient au démarrage de la session, donc après la création du premier fichier d'agent d'une portée dans un nouveau répertoire `agents`, redémarrez pour le charger.
  * Les sessions démarrées avec `--disable-slash-commands` ne surveillent pas du tout ces répertoires.
</Note>

```markdown theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Le frontmatter définit les métadonnées et la configuration du sous-agent. Le corps devient l'invite système qui guide le comportement du sous-agent. Les sous-agents reçoivent uniquement cette invite système plus les détails d'environnement de base comme le répertoire de travail, pas l'invite système complète de Claude Code.

En [mode non interactif](/docs/fr/headless), le drapeau [`--append-subagent-system-prompt`](/docs/fr/cli-reference#cli-flags) ajoute le texte que vous fournissez à la fin de l'invite système de chaque sous-agent, y compris les sous-agents imbriqués. Nécessite Claude Code v2.1.205 ou ultérieur.

Un sous-agent démarre dans le répertoire de travail actuel de la conversation principale. Au sein d'un sous-agent, les commandes `cd` ne persistent pas entre les appels d'outils Bash ou PowerShell et n'affectent pas le répertoire de travail de la conversation principale. Pour donner au sous-agent une copie isolée du référentiel à la place, définissez [`isolation: worktree`](#supported-frontmatter-fields).

Un sous-agent avec `isolation: worktree` exécute ses commandes Bash et PowerShell à l'intérieur de son worktree. Une commande dont le répertoire de travail se résout à votre extraction principale à la place, par exemple parce que le répertoire worktree a été supprimé pendant que le sous-agent s'exécutait, échoue avec une erreur. Avant la v2.1.203, une telle commande pouvait s'exécuter dans l'extraction principale.

<h4 id="supported-frontmatter-fields">
  Champs de frontmatter pris en charge
</h4>

Les champs suivants peuvent être utilisés dans le frontmatter YAML. Seuls `name` et `description` sont obligatoires.

| Champ             | Obligatoire | Description                                                                                                                                                                                                                                                                                                                                                                                    |
| :---------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Oui         | Identifiant unique utilisant des lettres minuscules et des tirets. Les [Hooks](/docs/fr/hooks#subagentstart) reçoivent cette valeur comme `agent_type`. Le nom du fichier n'a pas besoin de correspondre                                                                                                                                                                                            |
| `description`     | Oui         | Quand Claude doit déléguer à ce sous-agent                                                                                                                                                                                                                                                                                                                                                     |
| `tools`           | Non         | [Outils](#available-tools) que le sous-agent peut utiliser. Hérite de tous les outils s'il est omis. Si aucune entrée de la liste ne se résout en un outil, le sous-agent échoue au lancement avec une erreur nommant les entrées. Pour précharger les Skills dans le contexte, utilisez le champ `skills` plutôt que de lister `Skill` ici                                                    |
| `disallowedTools` | Non         | Outils à refuser, supprimés de la liste héritée ou spécifiée                                                                                                                                                                                                                                                                                                                                   |
| `model`           | Non         | [Modèle](#choose-a-model) à utiliser : `sonnet`, `opus`, `haiku`, `fable`, un ID de modèle complet (par exemple, `claude-opus-4-8`), ou `inherit`. Par défaut `inherit`                                                                                                                                                                                                                        |
| `permissionMode`  | Non         | [Mode de permission](#permission-modes) : `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, ou `manual` comme alias pour `default`. L'alias `manual` nécessite Claude Code v2.1.200 ou ultérieur. Ignoré pour les [sous-agents de plugin](#choose-the-subagent-scope)                                                                                                  |
| `maxTurns`        | Non         | Nombre maximum de tours d'agent avant que le sous-agent s'arrête                                                                                                                                                                                                                                                                                                                               |
| `skills`          | Non         | [Skills](/docs/fr/skills) à précharger dans le contexte du sous-agent au démarrage. Le contenu complet de la skill est injecté, pas seulement la description. Les sous-agents peuvent toujours invoquer les skills de projet, utilisateur et plugin non listées via l'outil Skill                                                                                                                   |
| `mcpServers`      | Non         | [Serveurs MCP](/docs/fr/mcp) disponibles pour ce sous-agent. Chaque entrée est soit un nom de serveur référençant un serveur déjà configuré (par exemple, `"slack"`) soit une définition en ligne avec le nom du serveur comme clé et une [configuration de serveur MCP](/docs/fr/mcp#installing-mcp-servers) complète comme valeur. Ignoré pour les [sous-agents de plugin](#choose-the-subagent-scope) |
| `hooks`           | Non         | [Hooks de cycle de vie](#define-hooks-for-subagents) limités à ce sous-agent. Ignoré pour les [sous-agents de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                              |
| `memory`          | Non         | [Portée de la mémoire persistante](#enable-persistent-memory) : `user`, `project` ou `local`. Active l'apprentissage entre sessions                                                                                                                                                                                                                                                            |
| `background`      | Non         | Définir sur `true` pour toujours exécuter ce sous-agent en tant que [tâche d'arrière-plan](#run-subagents-in-foreground-or-background), même lorsque Claude a besoin de son résultat immédiatement. Lorsque non défini, Claude choisit, et à partir de la v2.1.198 il exécute les sous-agents en arrière-plan par défaut                                                                       |
| `effort`          | Non         | Niveau d'effort lorsque ce sous-agent est actif. Remplace le niveau d'effort de la session. Par défaut : hérite de la session. Options : `low`, `medium`, `high`, `xhigh`, `max` ; les niveaux disponibles dépendent du modèle                                                                                                                                                                 |
| `isolation`       | Non         | Définir sur `worktree` pour exécuter le sous-agent dans un [git worktree](/docs/fr/worktrees) temporaire, ce qui lui donne une copie isolée du référentiel branchée par défaut à partir de votre [branche par défaut](/docs/fr/worktrees#choose-the-base-branch) plutôt que du `HEAD` de la session parent. Le worktree est automatiquement nettoyé si le sous-agent n'apporte aucune modification       |
| `color`           | Non         | Couleur d'affichage pour le sous-agent dans la liste des tâches et la transcription. Accepte `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` ou `cyan`                                                                                                                                                                                                                            |
| `initialPrompt`   | Non         | Auto-soumis comme le premier tour utilisateur lorsque cet agent s'exécute en tant qu'agent de session principal (via `--agent` ou le paramètre `agent`). Les [commandes](/docs/fr/commands) et les [skills](/docs/fr/skills) sont traitées. Préfixé à tout invite fourni par l'utilisateur                                                                                                               |

<h3 id="choose-a-model">
  Choisir un modèle
</h3>

Le champ `model` contrôle quel [modèle IA](/docs/fr/model-config) le sous-agent utilise :

* **Alias de modèle** : Utilisez l'un des alias disponibles : `sonnet`, `opus`, `haiku` ou `fable`
* **ID de modèle complet** : Utilisez un ID de modèle complet tel que `claude-opus-4-8` ou `claude-sonnet-5`. Accepte les mêmes valeurs que le drapeau `--model`
* **inherit** : Utilisez le même modèle que la conversation principale
* **Omis** : S'il n'est pas spécifié, par défaut `inherit` (utilise le même modèle que la conversation principale)

Lorsque Claude invoque un sous-agent, il peut également passer un paramètre `model` pour cette invocation spécifique. Claude Code résout le modèle du sous-agent dans cet ordre :

1. La variable d'environnement [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/fr/model-config#environment-variables), si elle est définie sur un alias de modèle ou un ID de modèle
2. Le paramètre `model` par invocation
3. Le frontmatter `model` de la définition du sous-agent
4. Le modèle de la conversation principale

À partir de la v2.1.196, définir `CLAUDE_CODE_SUBAGENT_MODEL` sur `inherit` est identique à le laisser non défini : la résolution continue avec le paramètre `model` par invocation, puis le frontmatter. Dans les versions antérieures, `inherit` forçait les sous-agents sur le modèle de la conversation principale et ignorait ces deux sources.

Claude Code vérifie la variable d'environnement, le paramètre par invocation et les valeurs du frontmatter par rapport à la liste blanche [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation. Une valeur qui se résout en un modèle exclu n'est pas utilisée et le sous-agent s'exécute sur le modèle hérité à la place.

À partir de la v2.1.198, les sous-agents héritent également de la configuration [extended thinking](/docs/fr/model-config#extended-thinking) de la conversation principale : si la réflexion est activée dans votre session, elle est activée pour le sous-agent, et si elle est désactivée, elle reste désactivée. Il n'y a pas de paramètre de réflexion par sous-agent. Avant la v2.1.198, les sous-agents s'exécutaient avec la réflexion étendue désactivée indépendamment du paramètre de la conversation principale.

<h3 id="control-subagent-capabilities">
  Contrôler les capacités des sous-agents
</h3>

Vous pouvez contrôler ce que les sous-agents peuvent faire via l'accès aux outils, les modes de permission et les règles conditionnelles.

<h4 id="available-tools">
  Outils disponibles
</h4>

Les sous-agents héritent des [outils internes](/docs/fr/tools-reference) et des outils MCP disponibles dans la conversation principale par défaut. Les outils suivants dépendent de l'interface utilisateur ou de l'état de session de la conversation principale et ne sont pas disponibles pour les sous-agents, même s'ils sont listés dans le champ `tools` :

* `AskUserQuestion`
* `EnterPlanMode`
* `ExitPlanMode`, sauf si le [`permissionMode`](#permission-modes) du sous-agent est `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`

Pour restreindre les outils, utilisez soit le champ `tools` (liste blanche) soit le champ `disallowedTools` (liste noire). Cet exemple utilise `tools` pour autoriser exclusivement Read, Grep, Glob et Bash. Le sous-agent ne peut pas modifier les fichiers, écrire des fichiers ou utiliser des outils MCP :

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Cet exemple utilise `disallowedTools` pour hériter de tous les outils de la conversation principale sauf Write et Edit. Le sous-agent conserve Bash, les outils MCP et tout le reste :

```yaml theme={null}
---
name: no-writes
description: Inherits every tool except file writes
disallowedTools: Write, Edit
---
```

Si les deux sont définis, `disallowedTools` est appliqué en premier, puis `tools` est résolu par rapport au pool restant. Un outil listé dans les deux est supprimé.

Lorsque rien dans la liste `tools` ne se résout en un outil, par exemple parce que chaque entrée est mal orthographiée ou nomme un outil qui n'est pas disponible pour les sous-agents, Claude Code refuse de lancer le sous-agent et l'outil Agent retourne une erreur nommant les entrées non résolues. Avant la v2.1.208, ce sous-agent se lançait sans outils et pouvait retourner un résultat vide ou confus.

Les deux champs acceptent des modèles au niveau du serveur MCP en plus des noms d'outils exacts : `mcp__<server>` ou `mcp__<server>__*` accorde ou supprime tous les outils du serveur nommé. Dans `disallowedTools`, `mcp__*` supprime également tous les outils MCP de n'importe quel serveur. Cet exemple supprime tous les outils du serveur MCP `github` tout en conservant les outils d'autres serveurs et tous les outils intégrés :

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

<h4 id="restrict-which-subagents-can-be-spawned">
  Restreindre les sous-agents qui peuvent être générés
</h4>

Lorsqu'un agent s'exécute en tant que thread principal avec `claude --agent`, il peut générer des sous-agents à l'aide de l'outil Agent. Pour restreindre les types de sous-agents qu'il peut générer, utilisez la syntaxe `Agent(agent_type)` dans le champ `tools`.

<Note>Dans la version 2.1.63, l'outil Task a été renommé en Agent. Les références `Task(...)` existantes dans les paramètres et les définitions d'agent fonctionnent toujours comme des alias.</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

C'est une liste blanche : seuls les sous-agents `worker` et `researcher` peuvent être générés. Si l'agent essaie de générer un autre type, la demande échoue et l'agent ne voit que les types autorisés dans son invite. Pour bloquer des agents spécifiques tout en autorisant tous les autres, utilisez plutôt [`permissions.deny`](#disable-specific-subagents).

Pour autoriser la génération de n'importe quel sous-agent sans restrictions, utilisez `Agent` sans parenthèses :

```yaml theme={null}
tools: Agent, Read, Bash
```

Si `Agent` est complètement omis de la liste `tools`, l'agent ne peut générer aucun sous-agent.

La syntaxe de liste blanche `Agent(agent_type)` s'applique uniquement à un agent s'exécutant en tant que thread principal avec `claude --agent`. Dans une définition de sous-agent, lister `Agent` dans `tools` permet à ce sous-agent de [générer des sous-agents imbriqués](#spawn-nested-subagents), mais toute liste de types à l'intérieur des parenthèses est ignorée.

<h4 id="scope-mcp-servers-to-a-subagent">
  Limiter les serveurs MCP à un sous-agent
</h4>

Utilisez le champ `mcpServers` pour donner à un sous-agent l'accès aux serveurs [MCP](/docs/fr/mcp) qui ne sont pas disponibles dans la conversation principale. Les serveurs en ligne définis ici sont connectés au démarrage du sous-agent et déconnectés à la fin. Les références de chaîne partagent la connexion de la session parent.

<Note>
  Le champ `mcpServers` s'applique dans les deux contextes où un fichier d'agent peut s'exécuter :

  * En tant que sous-agent, généré via l'outil Agent ou une @-mention
  * En tant que session principale, lancée avec [`--agent`](#invoke-subagents-explicitly) ou le paramètre `agent`

  Lorsque l'agent est la session principale, les définitions de serveur en ligne se connectent au démarrage aux côtés des serveurs de [`.mcp.json`](/docs/fr/mcp) et des fichiers de paramètres.
</Note>

Chaque entrée de la liste est soit une définition de serveur en ligne, soit une chaîne référençant un serveur MCP déjà configuré dans votre session :

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

Les définitions en ligne utilisent le même schéma que les entrées de serveur `.mcp.json`, indexées par le nom du serveur, et prennent en charge les types `stdio`, `http`, `sse` et `ws`.

Pour garder un serveur MCP en dehors de la conversation principale et éviter que ses descriptions d'outils ne consomment du contexte, définissez-le en ligne ici plutôt que dans `.mcp.json`. Le sous-agent obtient les outils ; la conversation parent ne les obtient pas.

À partir de la v2.1.153, les restrictions MCP qui s'appliquent à la session principale couvrent également les serveurs déclarés dans le frontmatter du sous-agent :

* [`--strict-mcp-config`](/docs/fr/cli-reference) et [`--bare`](/docs/fr/cli-reference)
* [Configuration MCP gérée en entreprise](/docs/fr/managed-mcp)
* [Politiques `allowedMcpServers` et `deniedMcpServers`](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Lorsque l'une de ces options bloque un serveur, Claude Code le saute et affiche un avertissement nommant les serveurs bloqués.

Les restrictions des paramètres gérés s'appliquent à chaque sous-agent indépendamment de la façon dont il est défini. `--strict-mcp-config` ne filtre pas les serveurs que vous transmettez en ligne via `--agents` ou l'option `agents` du SDK, car il s'agit d'une entrée explicite de l'appelant.

<h4 id="permission-modes">
  Modes de permission
</h4>

Le champ `permissionMode` contrôle comment le sous-agent gère les invites de permission. Les sous-agents héritent du contexte de permission de la conversation principale et peuvent remplacer le mode, sauf lorsque le mode parent prend précédence comme décrit ci-dessous.

| Mode                | Comportement                                                                                                                                                       |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Vérification de permission standard avec invites                                                                                                                   |
| `acceptEdits`       | Auto-accepter les modifications de fichiers et les commandes courantes du système de fichiers pour les chemins du répertoire de travail ou `additionalDirectories` |
| `auto`              | [Mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) : un classificateur IA évalue chaque appel d'outil                                              |
| `dontAsk`           | Auto-refuser les invites de permission (les outils explicitement autorisés fonctionnent toujours)                                                                  |
| `bypassPermissions` | Ignorer les invites de permission                                                                                                                                  |
| `plan`              | Mode plan (exploration en lecture seule)                                                                                                                           |

<Warning>
  Utilisez `bypassPermissions` avec prudence. Il ignore les invites de permission, permettant au sous-agent d'exécuter des opérations sans approbation, y compris les écritures dans `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` et `.mvn`.

  Les règles [`ask`](/docs/fr/permissions#manage-permissions) explicites, les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) et les suppressions du répertoire racine et du répertoire personnel comme `rm -rf /` demandent toujours une confirmation. Consultez [modes de permission](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode) pour plus de détails.
</Warning>

Si le parent utilise `bypassPermissions` ou `acceptEdits`, cela prend précédence et ne peut pas être remplacé. Si le parent utilise le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), le sous-agent hérite du mode auto et tout `permissionMode` dans son frontmatter est ignoré : le classificateur évalue les appels d'outils du sous-agent avec les mêmes règles de blocage et d'autorisation que la session parent.

<h4 id="preload-skills-into-subagents">
  Précharger les skills dans les sous-agents
</h4>

Utilisez le champ `skills` pour injecter le contenu de la skill dans le contexte du sous-agent au démarrage. Cela donne au sous-agent des connaissances de domaine sans qu'il ait besoin de découvrir et charger les skills pendant l'exécution.

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

Le contenu complet de chaque skill listée est injecté dans le contexte du sous-agent au démarrage. Ce champ contrôle quelles skills sont préchargées, pas quelles skills le sous-agent peut accéder : sans lui, le sous-agent peut toujours découvrir et invoquer les skills de projet, utilisateur et plugin via l'outil Skill pendant l'exécution. Pour empêcher un sous-agent d'invoquer les skills entièrement, omettez `Skill` de la liste [`tools`](#available-tools) ou ajoutez-le à `disallowedTools`.

Vous ne pouvez pas précharger les skills qui définissent [`disable-model-invocation: true`](/docs/fr/skills#control-who-invokes-a-skill), car le préchargement provient du même ensemble de skills que Claude peut invoquer. Si une skill listée est manquante ou désactivée, Claude Code la saute et enregistre un avertissement dans le journal de débogage.

<Note>
  C'est l'inverse de [l'exécution d'une skill dans un sous-agent](/docs/fr/skills#run-skills-in-a-subagent). Avec `skills` dans un sous-agent, le sous-agent contrôle l'invite système et charge le contenu de la skill. Avec `context: fork` dans une skill, le contenu de la skill est injecté dans l'agent que vous spécifiez. Les deux utilisent le même système sous-jacent.
</Note>

<h4 id="enable-persistent-memory">
  Activer la mémoire persistante
</h4>

Le champ `memory` donne au sous-agent un répertoire persistant qui survit aux conversations. Le sous-agent utilise ce répertoire pour accumuler des connaissances au fil du temps, comme les modèles de base de code, les insights de débogage et les décisions architecturales.

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

Choisissez une portée en fonction de la largeur d'application de la mémoire :

| Portée    | Emplacement                                   | Utiliser quand                                                                                                               |
| :-------- | :-------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | le sous-agent doit se souvenir des apprentissages dans tous les projets                                                      |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | les connaissances du sous-agent sont spécifiques au projet et partageables via le contrôle de version                        |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | les connaissances du sous-agent sont spécifiques au projet mais ne doivent pas être enregistrées dans le contrôle de version |

Lorsque la mémoire est activée :

* L'invite système du sous-agent inclut des instructions pour lire et écrire dans le répertoire de mémoire.
* L'invite système du sous-agent inclut également les 200 premières lignes ou 25 KB de `MEMORY.md` dans le répertoire de mémoire, selon ce qui est le moins important, avec des instructions pour organiser `MEMORY.md` s'il dépasse cette limite.
* Les outils Read, Write et Edit sont automatiquement activés pour que le sous-agent puisse gérer ses fichiers de mémoire.

<h5 id="persistent-memory-tips">
  Conseils de mémoire persistante
</h5>

* `project` est la portée par défaut recommandée. Elle rend les connaissances du sous-agent partageables via le contrôle de version.
* Demandez au sous-agent de consulter sa mémoire avant de commencer le travail : « Examinez cette PR et consultez votre mémoire pour les modèles que vous avez vus auparavant. »
* Demandez au sous-agent de mettre à jour sa mémoire après avoir terminé une tâche : « Maintenant que vous avez terminé, enregistrez ce que vous avez appris dans votre mémoire. » Au fil du temps, cela crée une base de connaissances qui rend le sous-agent plus efficace.
* Incluez les instructions de mémoire directement dans le fichier markdown du sous-agent pour qu'il maintienne proactivement sa propre base de connaissances :

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  Règles conditionnelles avec hooks
</h4>

Pour un contrôle plus dynamique de l'utilisation des outils, utilisez les hooks `PreToolUse` pour valider les opérations avant leur exécution. C'est utile lorsque vous devez autoriser certaines opérations d'un outil tout en en bloquer d'autres.

Cet exemple crée un sous-agent qui n'autorise que les requêtes de base de données en lecture seule. Le hook `PreToolUse` exécute le script spécifié dans `command` avant chaque commande Bash :

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code [passe l'entrée du hook en JSON](/docs/fr/hooks#pretooluse-input) via stdin aux commandes du hook. Le script de validation lit ce JSON, extrait la commande Bash et [quitte avec le code 2](/docs/fr/hooks#exit-code-2-behavior-per-event) pour bloquer les opérations d'écriture :

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

Consultez [Hook input](/docs/fr/hooks#pretooluse-input) pour le schéma d'entrée complet et [exit codes](/docs/fr/hooks#exit-code-output) pour savoir comment les codes de sortie affectent le comportement. Sur Windows, écrivez les scripts de hook en PowerShell et ajoutez `shell: powershell` à l'entrée du hook comme indiqué dans [exécution de hooks en PowerShell](/docs/fr/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Désactiver des sous-agents spécifiques
</h4>

Vous pouvez empêcher Claude d'utiliser des sous-agents spécifiques en les ajoutant au tableau `deny` dans vos [paramètres](/docs/fr/settings#permission-settings). Utilisez le format `Agent(subagent-name)` où `subagent-name` correspond au champ name du sous-agent.

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

Cela fonctionne pour les sous-agents intégrés et personnalisés. Vous pouvez également utiliser le drapeau CLI `--disallowedTools` :

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

Consultez la [documentation Permissions](/docs/fr/permissions#tool-specific-permission-rules) pour plus de détails sur les règles de permission.

<h3 id="define-hooks-for-subagents">
  Définir les hooks pour les sous-agents
</h3>

Les sous-agents peuvent définir des [hooks](/docs/fr/hooks) qui s'exécutent pendant le cycle de vie du sous-agent. Il y a deux façons de configurer les hooks :

* **Dans le frontmatter du sous-agent** : Définir les hooks qui s'exécutent uniquement pendant que ce sous-agent spécifique est actif
* **Dans `settings.json`** : Définir les hooks qui s'exécutent dans la session principale lorsque les sous-agents démarrent ou s'arrêtent

<h4 id="hooks-in-subagent-frontmatter">
  Hooks dans le frontmatter du sous-agent
</h4>

Définissez les hooks directement dans le fichier markdown du sous-agent. Ces hooks s'exécutent uniquement pendant que ce sous-agent spécifique est actif et sont nettoyés à la fin.

<Note>
  Les hooks de frontmatter se déclenchent lorsque l'agent est généré en tant que sous-agent via l'outil Agent ou une @-mention, et lorsque l'agent s'exécute en tant que principal de session via [`--agent`](#invoke-subagents-explicitly) ou le paramètre `agent`. Dans le cas de la session principale, ils s'exécutent aux côtés de tous les hooks définis dans [`settings.json`](/docs/fr/hooks).
</Note>

Tous les [événements de hook](/docs/fr/hooks#hook-events) sont pris en charge. Les événements les plus courants pour les sous-agents sont :

| Événement     | Entrée du matcher | Quand il se déclenche                                                     |
| :------------ | :---------------- | :------------------------------------------------------------------------ |
| `PreToolUse`  | Nom de l'outil    | Avant que le sous-agent utilise un outil                                  |
| `PostToolUse` | Nom de l'outil    | Après que le sous-agent utilise un outil                                  |
| `Stop`        | (aucun)           | Quand le sous-agent se termine (converti en `SubagentStop` à l'exécution) |

Cet exemple valide les commandes Bash avec le hook `PreToolUse` et exécute un linter après les modifications de fichiers avec `PostToolUse` :

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

Lorsque l'agent est invoqué en tant que sous-agent, les hooks `Stop` dans le frontmatter sont automatiquement convertis en événements `SubagentStop`.

<h4 id="project-level-hooks-for-subagent-events">
  Hooks au niveau du projet pour les événements de sous-agent
</h4>

Configurez les hooks dans `settings.json` qui répondent aux événements du cycle de vie du sous-agent dans la session principale.

| Événement       | Entrée du matcher   | Quand il se déclenche                    |
| :-------------- | :------------------ | :--------------------------------------- |
| `SubagentStart` | Nom du type d'agent | Quand un sous-agent commence l'exécution |
| `SubagentStop`  | Nom du type d'agent | Quand un sous-agent se termine           |

Les deux événements prennent en charge les matchers pour cibler des types d'agents spécifiques par nom. La valeur du matcher est le `name` du frontmatter de l'agent pour les sous-agents au niveau du projet et utilisateur, ou l'identifiant limité au plugin tel que `my-plugin:db-agent` pour les [sous-agents de plugin](/docs/fr/plugins). Un nom limité contient un deux-points, il est donc évalué comme une [expression régulière non ancrée](/docs/fr/hooks#matcher-patterns) ; ancrez-le avec `^` et `$`, comme dans `^my-plugin:db-agent$`, pour correspondre uniquement à cet agent.

Cet exemple exécute un script de configuration uniquement lorsque le sous-agent `db-agent` démarre, et un script de nettoyage lorsque n'importe quel sous-agent s'arrête :

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

Un matcher avec tirets comme `db-agent` correspond exactement sur Claude Code v2.1.195 ou ultérieur. Sur les versions antérieures, il est évalué comme une expression régulière non ancrée et se déclenche également pour tout type d'agent qui le contient, comme `prod-db-agent` ; ancrez-le comme `^db-agent$` sur ces versions.

Consultez [Hooks](/docs/fr/hooks) pour le format de configuration complet des hooks.

<h2 id="work-with-subagents">
  Travailler avec les sous-agents
</h2>

<h3 id="understand-automatic-delegation">
  Comprendre la délégation automatique
</h3>

Claude délègue automatiquement les tâches en fonction de la description de la tâche dans votre demande, du champ `description` dans les configurations de sous-agent et du contexte actuel. Pour encourager la délégation proactive, incluez des phrases comme « use proactively » dans le champ description de votre sous-agent.

<h3 id="invoke-subagents-explicitly">
  Invoquer les sous-agents explicitement
</h3>

Lorsque la délégation automatique ne suffit pas, vous pouvez demander un sous-agent vous-même. Trois modèles escaladent d'une suggestion ponctuelle à une valeur par défaut au niveau de la session :

* **Langage naturel** : nommez le sous-agent dans votre invite ; Claude décide s'il faut déléguer
* **@-mention** : garantit que le sous-agent s'exécute pour une tâche
* **Au niveau de la session** : la session entière utilise l'invite système, les restrictions d'outils et le modèle de ce sous-agent via le drapeau `--agent` ou le paramètre `agent`

Pour le langage naturel, il n'y a pas de syntaxe spéciale. Nommez le sous-agent et Claude délègue généralement :

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-mentionnez le sous-agent.** Tapez `@` et choisissez le sous-agent dans la saisie semi-automatique, de la même manière que vous @-mentionnez les fichiers. Cela garantit que ce sous-agent spécifique s'exécute plutôt que de laisser le choix à Claude :

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

Votre message complet va toujours à Claude, qui écrit l'invite de tâche du sous-agent en fonction de ce que vous avez demandé. La @-mention contrôle quel sous-agent Claude invoque, pas quelle invite il reçoit.

Les sous-agents fournis par un [plugin](/docs/fr/plugins) activé apparaissent dans la saisie semi-automatique sous leur nom délimité, comme `my-plugin:code-reviewer` ou `my-plugin:review:security` lorsque le plugin [organise les agents dans des sous-dossiers](#choose-the-subagent-scope). Les sous-agents d'arrière-plan nommés actuellement en cours d'exécution dans la session apparaissent également dans la saisie semi-automatique, affichant leur statut à côté du nom.

Vous pouvez également taper la mention manuellement sans utiliser le sélecteur : `@agent-<name>` pour les sous-agents locaux, ou `@agent-` suivi du nom délimité pour les sous-agents de plugin, par exemple `@agent-my-plugin:code-reviewer`.

**Exécutez la session entière en tant que sous-agent.** Passez [`--agent <name>`](/docs/fr/cli-reference) pour démarrer une session où le thread principal lui-même prend l'invite système, les restrictions d'outils et le modèle de ce sous-agent :

```bash theme={null}
claude --agent code-reviewer
```

L'invite système du sous-agent remplace complètement l'invite système par défaut de Claude Code, de la même manière que [`--system-prompt`](/docs/fr/cli-reference) le fait. Les fichiers `CLAUDE.md` et la mémoire du projet se chargent toujours via le flux de messages normal. Le nom de l'agent apparaît comme `@<name>` dans l'en-tête de démarrage pour que vous puissiez confirmer qu'il est actif.

Cela fonctionne avec les sous-agents intégrés et personnalisés, et le choix persiste lorsque vous reprenez la session.

Pour un sous-agent fourni par un plugin, vous pouvez passer simplement le nom de l'agent et Claude Code le trouvera :

```bash theme={null}
claude --agent security-reviewer
```

Si plusieurs plugins fournissent des agents avec le même nom, passez le nom délimité pour lever l'ambiguïté :

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Si le plugin place l'agent dans un sous-dossier de son répertoire `agents/`, incluez le sous-dossier dans le nom délimité, par exemple `claude --agent my-plugin:review:security`.

Pour en faire la valeur par défaut pour chaque session dans un projet, définissez `agent` dans `.claude/settings.json` :

```json theme={null}
{
  "agent": "code-reviewer"
}
```

Le drapeau CLI remplace le paramètre si les deux sont présents.

<h3 id="run-subagents-in-foreground-or-background">
  Exécuter les sous-agents au premier plan ou en arrière-plan
</h3>

Les sous-agents peuvent s'exécuter au premier plan ou en arrière-plan :

* **Les sous-agents au premier plan** bloquent la conversation principale jusqu'à la fin. Les invites de permission vous sont transmises au fur et à mesure qu'elles se produisent.
* **Les sous-agents en arrière-plan** s'exécutent simultanément pendant que vous continuez à travailler. À partir de la v2.1.186, lorsqu'un sous-agent en arrière-plan atteint un appel d'outil qui nécessite une permission, l'invite s'affiche dans votre session principale et nomme le sous-agent qui demande. Approuvez pour laisser le sous-agent continuer, ou appuyez sur Échap pour refuser cet appel d'outil sans arrêter le sous-agent. Avant la v2.1.186, les sous-agents en arrière-plan refusaient automatiquement tout appel d'outil qui aurait demandé une permission.

À partir de la v2.1.198, les sous-agents s'exécutent en arrière-plan par défaut. Claude exécute un sous-agent au premier plan lorsqu'il a besoin du résultat avant de continuer. Le changement par défaut détermine où un sous-agent s'exécute, pas ce qu'il est autorisé à faire : les sous-agents en arrière-plan affichent toujours chaque invite de permission dans votre session principale. Avant la v2.1.198, Claude choisissait entre le premier plan et l'arrière-plan en fonction de la tâche.

Vous pouvez également diriger cela vous-même :

* Demander à Claude d'exécuter une tâche en arrière-plan ou au premier plan
* Appuyer sur **Ctrl+B** pour mettre une tâche en arrière-plan

Un sous-agent en arrière-plan qui se termine reste listé dans [`/tasks`](/docs/fr/commands), marqué comme terminé et trié sous le travail en cours, jusqu'à ce que la session nettoie sa liste de tâches. Sa vue détaillée reste ouverte lorsque le sous-agent se termine. Les sous-agents qui échouent ou que vous arrêtez quittent la liste. Avant la v2.1.208, un sous-agent terminé quittait la liste dès qu'il se terminait et sa vue détaillée se fermait.

Pour désactiver toute la fonctionnalité de tâche en arrière-plan, définissez la variable d'environnement `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` sur `1`. Consultez [Variables d'environnement](/docs/fr/env-vars).

Lorsque [`CLAUDE_CODE_FORK_SUBAGENT`](#fork-the-current-conversation) est défini sur `1`, chaque génération de sous-agent s'exécute en arrière-plan et le champ frontmatter `background` n'a aucun effet, car le mode fork supprime le paramètre `run_in_background` de l'outil `Agent`. `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` a la priorité sur le mode fork et maintient les générations de sous-agent au premier plan.

<h3 id="api-errors-in-subagents">
  Erreurs API dans les sous-agents
</h3>

À partir de la v2.1.199, un sous-agent dont l'exécution se termine sur une erreur API, comme une limite d'utilisation ou une erreur serveur répétée, signale cet échec à Claude au lieu de retourner le texte d'erreur comme s'il s'agissait des résultats du sous-agent. Ce que Claude reçoit dépend de l'endroit où le sous-agent s'est exécuté :

* **Premier plan** : si une limite de débit, une surcharge ou une erreur serveur coupe un sous-agent qui a déjà produit une sortie, l'outil Agent retourne cette sortie partielle avec une note indiquant que le sous-agent a été coupé et n'a pas terminé sa tâche. Un sous-agent qui n'a rien produit, ou dont la seule sortie était des appels d'outils, échoue avec [`Agent terminated early due to an API error`](/docs/fr/errors#agent-terminated-early-due-to-an-api-error), suivi du détail de l'erreur. Dans la v2.1.199, une limite de débit, une surcharge ou une erreur serveur qui a coupé la forme appels-d'outils-uniquement a retourné un résultat partiel vide contenant uniquement la note de coupure à la place.
* **Arrière-plan** : le sous-agent est marqué comme échoué, et le message que Claude reçoit lorsqu'il se termine nomme l'erreur API et inclut la dernière sortie du sous-agent, de sorte que le travail partiel n'est pas perdu.

Une fois que l'erreur API sous-jacente est résolue, demandez à Claude de réessayer la tâche ou de [reprendre le sous-agent](#resume-subagents).

<h3 id="common-patterns">
  Modèles courants
</h3>

<h4 id="isolate-high-volume-operations">
  Isoler les opérations à haut volume
</h4>

L'une des utilisations les plus efficaces des sous-agents est l'isolation des opérations qui produisent de grandes quantités de résultats. L'exécution de tests, la récupération de documentation ou le traitement de fichiers journaux peuvent consommer un contexte important. En déléguant ces tâches à un sous-agent, la sortie détaillée reste dans le contexte du sous-agent tandis que seul le résumé pertinent revient à votre conversation principale.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  Exécuter la recherche en parallèle
</h4>

Pour les investigations indépendantes, générez plusieurs sous-agents pour travailler simultanément :

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

Chaque sous-agent explore son domaine indépendamment, puis Claude synthétise les résultats. Cela fonctionne mieux lorsque les chemins de recherche ne dépendent pas les uns des autres.

<Warning>
  Lorsque les sous-agents se terminent, leurs résultats reviennent à votre conversation principale. L'exécution de nombreux sous-agents qui retournent chacun des résultats détaillés peut consommer un contexte important.
</Warning>

Pour les tâches qui nécessitent un parallélisme soutenu ou qui dépassent votre fenêtre de contexte, les [équipes d'agents](/docs/fr/agent-teams) donnent à chaque travailleur son propre contexte indépendant.

<h4 id="chain-subagents">
  Chaîner les sous-agents
</h4>

Pour les workflows multi-étapes, demandez à Claude d'utiliser les sous-agents en séquence. Chaque sous-agent termine sa tâche et retourne les résultats à Claude, qui transmet ensuite le contexte pertinent au sous-agent suivant.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Choisir entre les sous-agents et la conversation principale
</h3>

Utilisez la **conversation principale** quand :

* La tâche nécessite des allers-retours fréquents ou un raffinement itératif
* Plusieurs phases partagent un contexte important, comme la planification, l'implémentation et les tests
* Vous apportez une modification rapide et ciblée
* La latence est importante. Les sous-agents commencent à zéro et peuvent avoir besoin de temps pour rassembler le contexte

Utilisez les **sous-agents** quand :

* La tâche produit une sortie détaillée dont vous n'avez pas besoin dans votre contexte principal
* Vous souhaitez appliquer des restrictions d'outils ou des permissions spécifiques
* Le travail est autonome et peut retourner un résumé

Envisagez plutôt les [Skills](/docs/fr/skills) lorsque vous souhaitez des invites ou des workflows réutilisables qui s'exécutent dans le contexte de la conversation principale plutôt que dans un contexte de sous-agent isolé.

Pour une question rapide sur quelque chose déjà dans votre conversation, utilisez [`/btw`](/docs/fr/interactive-mode#side-questions-with-%2Fbtw) au lieu d'un sous-agent. Il voit votre contexte complet mais n'a pas d'accès aux outils, et la réponse est ignorée plutôt que d'être ajoutée à l'historique.

<h3 id="spawn-nested-subagents">
  Générer des sous-agents imbriqués
</h3>

À partir de Claude Code v2.1.172, un sous-agent peut générer ses propres sous-agents. Utilisez ceci lorsqu'une tâche déléguée se divise elle-même en sous-tâches parallèles, comme un sous-agent examinateur qui envoie un vérificateur par résultat, de sorte que la sortie intermédiaire n'atteint jamais votre conversation principale. Seul le résumé du sous-agent de niveau supérieur vous revient.

Un sous-agent imbriqué est configuré de la même manière qu'un sous-agent de niveau supérieur et se résout à partir des mêmes [portées](#choose-the-subagent-scope).

Le panneau de sous-agent sous l'entrée d'invite affiche l'arborescence complète : chaque ligne affiche un nombre `(+N)` de descendants, et à partir de la v2.1.193, l'ouverture d'une ligne affiche les frères et sœurs de ce sous-agent et les enfants directs avec un chemin de retour à `main`.

La profondeur est comptée comme le nombre de niveaux de sous-agent en dessous de la conversation principale, indépendamment du fait que chaque niveau s'exécute en [premier plan ou en arrière-plan](#run-subagents-in-foreground-or-background). Un sous-agent à la profondeur cinq ne reçoit pas l'outil Agent et ne peut pas générer d'autres. La limite est fixe et non configurable.

À partir de Claude Code v2.1.187, la profondeur d'un sous-agent en arrière-plan est fixée lorsqu'il est d'abord généré, et [reprendre](#resume-subagents) celui-ci ultérieurement ne change pas cette profondeur. Par exemple, si votre conversation principale génère le sous-agent A, et A génère un sous-agent en arrière-plan B à la profondeur deux, B est toujours à la profondeur deux lorsque vous le reprenez directement à partir de la conversation principale. Reprendre un sous-agent à partir d'un contexte moins profond ne lui permet pas de générer des niveaux supplémentaires que la limite de profondeur a déjà empêchés.

Pour empêcher un sous-agent spécifique de générer d'autres, omettez `Agent` de sa liste [`tools`](#available-tools) ou ajoutez-le à `disallowedTools`.

Un [fork](#fork-the-current-conversation) ne peut toujours pas générer un autre fork. Il peut générer d'autres types de sous-agents, et ceux-ci comptent vers la limite de profondeur.

<h3 id="manage-subagent-context">
  Gérer le contexte du sous-agent
</h3>

<h4 id="what-loads-at-startup">
  Ce qui se charge au démarrage
</h4>

Chaque sous-agent démarre avec une fenêtre de contexte fraîche et isolée. Il ne voit pas votre historique de conversation, les skills que vous avez déjà invoqués, ou les fichiers que Claude a déjà lus. Claude compose un message de délégation qui résume la tâche, et le sous-agent travaille à partir de là. L'exception est un [fork](#fork-the-current-conversation), qui hérite de la conversation parent au lieu de commencer à zéro.

Le contexte initial d'un sous-agent non-fork contient :

* **Invite système** : l'invite propre de l'agent plus les détails d'environnement que Claude Code ajoute, pas l'invite système complète de Claude Code. Les sous-agents personnalisés définissent la leur dans le [corps markdown](#write-subagent-files) ou le champ `prompt`. Les agents intégrés ont des invites prédéfinies.
* **Message de tâche** : l'invite de délégation que Claude écrit lorsqu'il confie le travail.
* **CLAUDE.md et mémoire** : chaque niveau de la [hiérarchie de mémoire](/docs/fr/memory#how-claude-md-files-load) que la conversation principale charge, y compris `~/.claude/CLAUDE.md`, les règles du projet, `CLAUDE.local.md`, et les fichiers de politique gérés. Les agents Explore et Plan intégrés ignorent cela.
* **Statut Git** : un instantané pris au début de la session parent. Absent lorsque le répertoire de travail n'est pas un référentiel Git ou lorsque [`includeGitInstructions`](/docs/fr/settings#available-settings) est `false`. Explore et Plan l'ignorent de toute façon.
* **Skills préchargés** : contenu complet de tout skill nommé dans le champ [`skills`](#preload-skills-into-subagents) de l'agent. Les agents intégrés ne préchargent pas les skills.
* **Roster des frères et sœurs** : un rappel système listant `main` et tous les autres agents nommés dans la session, chacun étant une valeur `to` valide pour [`SendMessage`](#resume-subagents). Nécessite Claude Code v2.1.206 ou ultérieur. Le roster n'apparaît que lorsque les outils du sous-agent incluent `SendMessage` et qu'au moins un autre agent a un nom, que Claude l'ait nommé lors de sa génération ou qu'il s'exécute en tant que coéquipier d'une [équipe d'agents](/docs/fr/agent-teams). C'est un instantané pris lorsque le sous-agent démarre, donc les agents nommés ultérieurement n'apparaissent pas.

Explore et Plan sont les seuls sous-agents qui omettent CLAUDE.md et le statut git. Il n'y a pas de champ frontmatter ou de paramètre par agent pour modifier les agents qui les ignorent.

La conversation principale lit les résultats d'Explore et Plan avec le contexte CLAUDE.md complet, donc la plupart des règles n'ont pas besoin d'atteindre le sous-agent lui-même. Si une règle doit le faire, comme « ignorer le répertoire `vendor/` », reformulez-la dans l'invite que vous donnez à Claude lors de la délégation.

<h4 id="resume-subagents">
  Reprendre les sous-agents
</h4>

Chaque invocation de sous-agent crée une nouvelle instance avec un contexte frais. Pour continuer le travail d'un sous-agent existant au lieu de recommencer, demandez à Claude de le reprendre.

Les sous-agents repris conservent leur historique de conversation complet, y compris tous les appels d'outils précédents, les résultats et le raisonnement. Le sous-agent reprend exactement où il s'était arrêté plutôt que de recommencer à zéro.

Lorsqu'un sous-agent se termine, Claude reçoit son ID d'agent. Les agents Explore et Plan intégrés sont ponctuels et ne retournent pas d'ID d'agent, donc ils ne peuvent pas être repris ; utilisez `general-purpose` ou un sous-agent personnalisé lorsque vous avez besoin de continuer le travail.

Claude utilise l'outil `SendMessage` avec l'ID de l'agent ou le nom comme champ `to` pour le reprendre. `SendMessage` ne nécessite pas que les [équipes d'agents](/docs/fr/agent-teams) soient activées ; seuls les messages de protocole d'équipe structurés tels que `shutdown_request` et `plan_approval_response` le font.

Pour reprendre un sous-agent, demandez à Claude de continuer le travail précédent :

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Un sous-agent arrêté qui reçoit un `SendMessage` se reprend automatiquement en arrière-plan sans nécessiter une nouvelle invocation `Agent`. Le même principe s'applique à un sous-agent que Claude a arrêté avec l'outil `TaskStop`.

À partir de la v2.1.191, un sous-agent que vous avez arrêté vous-même, avec `x` dans `/tasks` ou une demande SDK `stop_task`, ne se reprend pas automatiquement. L'appel `SendMessage` retourne un refus indiquant à Claude que l'agent a été annulé. Tapez dans la transcription de ce sous-agent dans le panneau de sous-agent pour le reprendre vous-même, ce qui efface l'arrêt pour que les appels `SendMessage` ultérieurs puissent le reprendre automatiquement à nouveau.

Reprendre démarre une nouvelle exécution de l'agent sous le même ID, de sorte qu'un sous-agent qui avait déjà échoué ou s'était terminé s'affiche à nouveau comme en cours d'exécution dans la liste des tâches et dans les événements de tâche du SDK Agent. Avant la v2.1.205, il continuait à afficher son statut antérieur échoué ou terminé pendant que l'exécution reprise fonctionnait.

À partir de la v2.1.199, `SendMessage` vérifie qu'un nom fait toujours référence au même agent qu'il a atteint plus tôt dans la conversation. Si un agent plus récent a pris le nom, comme un sous-agent en arrière-plan réengendré qui l'a réutilisé, Claude Code refuse l'envoi plutôt que de le livrer au mauvais agent, et l'erreur signale quel agent le nom atteint maintenant pour que Claude puisse le rediriger. Pour atteindre l'agent antérieur pendant qu'il s'exécute toujours, Claude l'adresse par l'ID d'agent de son résultat de génération. La vérification est limitée à la conversation actuelle et se réinitialise sur `/clear`.

À partir de la v2.1.198, un sous-agent traite les messages de l'agent qui l'a lancé comme une direction de tâche normale, y compris les corrections de cours en cours de tâche, et agit en fonction de ceux-ci dans ses propres paramètres de permission. Deux limites tiennent toujours indépendamment de qui a envoyé le message : aucun message d'aucun agent ne compte comme votre approbation pour une invite de permission en attente, et aucun message d'agent ne peut modifier les paramètres de permission d'un sous-agent, `CLAUDE.md`, ou la configuration. Seul le système de permission ou vos propres messages peuvent accorder l'approbation.

Vous pouvez également demander à Claude l'ID d'agent si vous souhaitez le référencer explicitement, ou trouver les ID dans les fichiers de transcription à `~/.claude/projects/{project}/{sessionId}/subagents/`. Chaque transcription est stockée sous la forme `agent-{agentId}.jsonl`.

Les transcriptions de sous-agent persistent indépendamment de la conversation principale :

* **Compaction de la conversation principale** : Lorsque la conversation principale se compacte, les transcriptions de sous-agent ne sont pas affectées. Elles sont stockées dans des fichiers séparés.
* **Persistance de session** : Les transcriptions de sous-agent persistent au sein de leur session. Vous pouvez [reprendre un sous-agent](#resume-subagents) après le redémarrage de Claude Code en reprenant la même session.
* **Nettoyage automatique** : Les transcriptions sont nettoyées en fonction du paramètre `cleanupPeriodDays`, qui est par défaut de 30 jours.

<h4 id="auto-compaction">
  Auto-compaction
</h4>

Les sous-agents prennent en charge la compaction automatique en utilisant la même logique que la conversation principale. La compaction se déclenche dans les mêmes conditions, et `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` s'applique également aux sous-agents. Consultez [variables d'environnement](/docs/fr/env-vars) pour savoir quand le remplacement prend effet.

Les événements de compaction sont enregistrés dans les fichiers de transcription de sous-agent :

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

La valeur `preTokens` indique le nombre de tokens utilisés avant la compaction.

<h2 id="fork-the-current-conversation">
  Dupliquer la conversation actuelle
</h2>

<Note>
  Les sous-agents dupliqués nécessitent Claude Code v2.1.117 ou version ultérieure. À partir de v2.1.161, la commande `/fork` est activée par défaut ; sur les versions antérieures, elle nécessite de définir la variable d'environnement [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/fr/env-vars) sur `1`. Laisser Claude lui-même générer des forks est expérimental et peut changer dans les versions futures. Cette capacité peut également être activée dans les sessions interactives dans le cadre d'un déploiement progressif.
</Note>

Un fork est un sous-agent qui hérite de l'intégralité de la conversation jusqu'à présent au lieu de commencer à zéro. Cela supprime l'isolation d'entrée que les sous-agents fournissent autrement : un fork voit la même invite système, les mêmes outils, le même modèle et l'historique des messages que la session principale, vous pouvez donc lui confier une tâche secondaire sans réexpliquer la situation. Les appels d'outils du fork restent en dehors de votre conversation et seul son résultat final revient, donc votre fenêtre de contexte principal reste propre. Utilisez un fork lorsqu'un sous-agent nommé aurait besoin de trop de contexte pour être utile, ou lorsque vous souhaitez essayer plusieurs approches en parallèle à partir du même point de départ.

Pour contrôler le mode fork indépendamment du déploiement progressif, définissez [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/fr/env-vars) sur `1` pour l'activer explicitement ou sur `0` pour le désactiver. La variable est honorée en mode interactif et via le SDK ou `claude -p`.

L'activation du mode fork change Claude Code de deux façons :

* Claude peut générer un fork en demandant explicitement le type de sous-agent `fork`. Les générations sans type de sous-agent utilisent toujours le sous-agent [general-purpose](#built-in-subagents), et les sous-agents nommés tels que Explore se génèrent comme avant.
* Chaque génération de sous-agent s'exécute en [arrière-plan](#run-subagents-in-foreground-or-background), qu'il s'agisse d'un fork ou d'un sous-agent nommé. Définissez `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` sur `1` pour garder les générations synchrones.

Vous pouvez démarrer un fork vous-même avec `/fork` suivi d'une directive, avec ou sans la variable définie. Claude Code nomme le fork à partir des premiers mots de la directive. L'exemple suivant duplique la conversation pour rédiger des cas de test pendant que vous continuez avec l'implémentation dans la session principale :

```text wrap theme={null}
/fork draft unit tests for the parser changes so far
```

Le fork apparaît dans un panneau sous votre invite et s'exécute en arrière-plan pendant que vous continuez à travailler. Lorsqu'il se termine, son résultat arrive sous forme de message dans votre conversation principale. La section suivante couvre les contrôles du panneau pour observer et diriger les forks pendant qu'ils s'exécutent.

<h3 id="observe-and-steer-running-forks">
  Observer et diriger les forks en cours d'exécution
</h3>

Les forks en cours d'exécution apparaissent dans un panneau sous l'entrée d'invite, avec une ligne pour la session principale et une pour chaque fork. Utilisez ces touches pour interagir avec le panneau :

| Touche    | Action                                                                           |
| :-------- | :------------------------------------------------------------------------------- |
| `↑` / `↓` | Se déplacer entre les lignes                                                     |
| `Entrée`  | Ouvrir la transcription du fork sélectionné et lui envoyer des messages de suivi |
| `x`       | Ignorer un fork terminé ou arrêter un fork en cours d'exécution                  |
| `Échap`   | Retourner le focus à l'entrée d'invite                                           |

Avec la transcription d'un fork ou d'un sous-agent ouverte, les messages de suivi et les [skills](/docs/fr/skills) vont à cet agent, mais les commandes intégrées s'exécutent toujours dans votre conversation principale. À partir de v2.1.199, taper `/model` ou `/fast` dans cette vue affiche un avis indiquant que cela change le modèle ou le mode rapide de la conversation principale, et non celui de l'agent visualisé, au lieu de l'exécuter silencieusement.

<h3 id="how-forks-differ-from-named-subagents">
  Comment les forks diffèrent des sous-agents nommés
</h3>

Un fork hérite de tout ce que la session principale a au moment où il se génère. Un sous-agent nommé démarre à partir de sa propre définition.

|                          | Fork                                        | Sous-agent nommé                                                                                                                        |
| :----------------------- | :------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------- |
| Contexte                 | Historique de conversation complet          | Contexte frais avec l'invite que vous transmettez                                                                                       |
| Invite système et outils | Identique à la session principale           | À partir du [fichier de définition](#write-subagent-files) du sous-agent                                                                |
| Modèle                   | Identique à la session principale           | À partir du champ `model` du sous-agent                                                                                                 |
| Permissions              | Les invites s'affichent dans votre terminal | [Les invites s'affichent dans votre session principale](#run-subagents-in-foreground-or-background) lors de l'exécution en arrière-plan |
| Cache d'invite           | Partagé avec la session principale          | Cache séparé                                                                                                                            |

Parce que l'invite système d'un fork et les définitions d'outils sont identiques au parent, sa première demande réutilise le [cache d'invite](/docs/fr/prompt-caching#subagents-and-the-cache) du parent. Cela rend le forking moins cher que la génération d'un sous-agent frais pour les tâches qui ont besoin du même contexte.

Lorsque Claude génère un fork via l'outil Agent, il peut passer `isolation: "worktree"` pour que les modifications de fichiers du fork soient écrites dans un git worktree séparé au lieu de votre extraction.

<h3 id="limitations">
  Limitations
</h3>

Le paramètre `CLAUDE_CODE_FORK_SUBAGENT=1` active le mode fork dans les sessions interactives, le [mode non-interactif](/docs/fr/headless), et le SDK Agent ; le paramètre `0` désactive le mode fork partout, y compris tout déploiement côté serveur. Un fork ne peut pas générer d'autres forks.

<h2 id="example-subagents">
  Exemples de sous-agents
</h2>

Ces exemples démontrent des modèles efficaces pour construire des sous-agents. Utilisez-les comme points de départ, ou générez une version personnalisée avec Claude.

<Tip>
  **Meilleures pratiques :**

  * **Concevoir des sous-agents ciblés :** chaque sous-agent doit exceller dans une tâche spécifique
  * **Écrire des descriptions détaillées :** Claude utilise la description pour décider quand déléguer
  * **Limiter l'accès aux outils :** accordez uniquement les permissions nécessaires pour la sécurité et la concentration
  * **Enregistrer dans le contrôle de version :** partagez les sous-agents de projet avec votre équipe
</Tip>

<h3 id="code-reviewer">
  Examinateur de code
</h3>

Un sous-agent en lecture seule qui examine le code sans le modifier. Cet exemple montre comment concevoir un sous-agent ciblé avec un accès limité aux outils qui exclut Edit et Write, et une invite détaillée qui spécifie exactement ce qu'il faut chercher et comment formater la sortie.

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  Débogueur
</h3>

Un sous-agent qui peut à la fois analyser et corriger les problèmes. Contrairement à l'examinateur de code, celui-ci inclut Edit car corriger les bugs nécessite de modifier le code. L'invite fournit un workflow clair du diagnostic à la vérification.

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  Data scientist
</h3>

Un sous-agent spécialisé dans le domaine pour le travail d'analyse de données. Cet exemple montre comment créer des sous-agents pour des workflows spécialisés en dehors des tâches de codage typiques. Il définit explicitement `model: sonnet` pour une analyse plus capable.

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  Validateur de requête de base de données
</h3>

Un sous-agent qui autorise l'accès à Bash mais valide les commandes pour n'autoriser que les requêtes SQL en lecture seule. Cet exemple montre comment utiliser les hooks `PreToolUse` pour la validation conditionnelle lorsque vous avez besoin d'un contrôle plus fin que le champ `tools` ne le permet.

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [passe l'entrée du hook en JSON](/docs/fr/hooks#pretooluse-input) via stdin aux commandes du hook. Le script de validation lit ce JSON, extrait la commande en cours d'exécution et la vérifie par rapport à une liste d'opérations d'écriture SQL. Si une opération d'écriture est détectée, le script [quitte avec le code 2](/docs/fr/hooks#exit-code-2-behavior-per-event) pour bloquer l'exécution et retourne un message d'erreur à Claude via stderr.

Créez le script de validation n'importe où dans votre projet. Le chemin doit correspondre au champ `command` dans votre configuration de hook :

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

Sur macOS et Linux, rendez le script exécutable :

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Sur Windows, écrivez le script de validation en PowerShell et ajoutez `shell: powershell` à l'entrée du hook. Consultez [exécution des hooks dans PowerShell](/docs/fr/hooks#windows-powershell-tool).

Le hook reçoit JSON via stdin avec la commande Bash dans `tool_input.command`. Le code de sortie 2 bloque l'opération et renvoie le message d'erreur à Claude. Consultez [Hooks](/docs/fr/hooks#exit-code-output) pour plus de détails sur les codes de sortie et [Hook input](/docs/fr/hooks#pretooluse-input) pour le schéma d'entrée complet.

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous comprenez les sous-agents, explorez ces fonctionnalités connexes :

* [Distribuer les sous-agents avec les plugins](/docs/fr/plugins) pour partager les sous-agents entre les équipes ou les projets
* [Exécuter Claude Code par programmation](/docs/fr/headless) avec le SDK Agent pour CI/CD et l'automatisation
* [Utiliser les serveurs MCP](/docs/fr/mcp) pour donner aux sous-agents l'accès aux outils et données externes
