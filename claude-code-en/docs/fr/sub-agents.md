> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Créer des sous-agents personnalisés

> Créez et utilisez des sous-agents IA spécialisés dans Claude Code pour des workflows spécifiques à des tâches et une meilleure gestion du contexte.

Les sous-agents sont des assistants IA spécialisés qui gèrent des types de tâches spécifiques. Utilisez-en un lorsqu'une tâche secondaire inonderait votre conversation principale avec des résultats de recherche, des journaux ou des contenus de fichiers que vous ne référencerez plus : le sous-agent effectue ce travail dans son propre contexte et retourne uniquement le résumé. Définissez un sous-agent personnalisé lorsque vous générez constamment le même type de travailleur avec les mêmes instructions.

Chaque sous-agent s'exécute dans sa propre fenêtre de contexte avec une invite système personnalisée, un accès à des outils spécifiques et des permissions indépendantes. Lorsque Claude rencontre une tâche qui correspond à la description d'un sous-agent, il délègue à ce sous-agent, qui fonctionne indépendamment et retourne les résultats. Pour voir les économies de contexte en pratique, la [visualisation de la fenêtre de contexte](/docs/fr/context-window) vous guide à travers une session où un sous-agent gère la recherche dans sa propre fenêtre séparée.

<Note>
  Les sous-agents fonctionnent au sein d'une seule session. Pour exécuter de nombreuses sessions indépendantes en parallèle et les surveiller depuis un seul endroit, consultez [les agents en arrière-plan](/docs/fr/agent-view). Pour les sessions distinctes qui se transmettent des messages, consultez [la messagerie entre sessions](/docs/fr/cross-session-messaging). Pour une équipe coordonnée de sessions que Claude crée et supervise, consultez [les équipes d'agents](/docs/fr/agent-teams).
</Note>

Les sous-agents vous aident à :

* **Préserver le contexte** en gardant l'exploration et l'implémentation en dehors de votre conversation principale
* **Appliquer des contraintes** en limitant les outils qu'un sous-agent peut utiliser
* **Réutiliser les configurations** dans les projets avec des sous-agents au niveau utilisateur
* **Spécialiser le comportement** avec des invites système ciblées pour des domaines spécifiques
* **Contrôler les coûts** en acheminant les tâches vers des modèles plus rapides et moins chers comme Haiku

Claude utilise la description de chaque sous-agent pour décider quand déléguer les tâches. Lorsque vous créez un sous-agent, écrivez une description claire pour que Claude sache quand l'utiliser.

Ces descriptions consomment du contexte, donc gardez-les courtes. Lorsque les descriptions combinées de vos sous-agents, à l'exception des sous-agents intégrés, dépassent 15 000 jetons, Claude Code affiche un [avertissement au démarrage avec le nombre total de jetons](/docs/fr/errors#agent-descriptions-are-over-the-15000-token-limit). Réduisez les champs `description` de vos sous-agents et déplacez les détails dans l'invite système de chaque sous-agent, qui ne se charge que lorsque ce sous-agent s'exécute.

<h2 id="built-in-subagents">
  Sous-agents intégrés
</h2>

Claude Code inclut des sous-agents intégrés que Claude utilise automatiquement le cas échéant. Chacun hérite des permissions de la conversation parent ; la plupart s'exécutent avec un ensemble d'outils restreint.

Explore et Plan ignorent vos fichiers CLAUDE.md et l'instantané de l'état git pour maintenir la recherche rapide et économique. Tous les autres sous-agents intégrés et [sous-agents personnalisés](#configure-subagents) chargent les deux, sauf si sa définition définit le champ [`omitClaudeMd`](#supported-frontmatter-fields) pour ignorer les fichiers CLAUDE.md utilisateur, projet et local. Pour la ventilation complète de ce qui atteint un sous-agent, consultez [ce qui se charge au démarrage](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Un agent rapide et en lecture seule optimisé pour la recherche et l'analyse de bases de code.

    * **Modèle** : hérité de la conversation principale, limité à Opus sur l'API Claude, donc Explore ne s'exécute jamais sur un modèle plus coûteux que celui que vous avez déjà choisi pour la session, sauf si vous définissez `CLAUDE_CODE_SUBAGENT_MODEL` et [le forcez sur chaque sous-agent](#run-every-subagent-on-one-model)
    * **Outils** : outils en lecture seule ; Write et Edit sont refusés
    * **Objectif** : découverte de fichiers, recherche de code, exploration de base de code

    À partir de la v2.1.198, Explore hérite du modèle de la conversation principale au lieu de toujours s'exécuter sur Haiku. Sur l'API Claude, le modèle hérité est limité à Opus : une conversation principale sur un niveau supérieur exécute Explore sur Opus, et une conversation principale sur Sonnet ou Haiku exécute Explore sur ce même modèle. Sur tout autre fournisseur, tel que [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou Claude Platform on AWS](/docs/fr/third-party-integrations), Explore hérite directement du modèle de la conversation principale.

    Un [sous-agent utilisateur ou projet](#choose-the-subagent-scope) nommé `Explore` remplace le sous-agent intégré et conserve son propre champ `model`, donc définissez-en un avec `model: haiku` pour maintenir l'exploration sur un modèle moins coûteux.

    Claude délègue à Explore lorsqu'il doit rechercher ou comprendre une base de code sans apporter de modifications. Cela garde les résultats d'exploration en dehors du contexte de votre conversation principale.

    Lors de l'invocation d'Explore, Claude spécifie un niveau de minutie : **quick** pour les recherches ciblées, **medium** pour l'exploration équilibrée, ou **very thorough** pour l'analyse complète.
  </Tab>

  <Tab title="Plan">
    Un agent de recherche utilisé pendant le [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) pour rassembler le contexte avant de présenter un plan.

    * **Modèle** : hérité de la conversation principale, sauf si vous définissez `CLAUDE_CODE_SUBAGENT_MODEL` et [le forcez sur chaque sous-agent](#run-every-subagent-on-one-model)
    * **Outils** : outils en lecture seule ; Write et Edit sont refusés
    * **Objectif** : recherche de base de code pour la planification

    Lorsque vous êtes en mode plan et que Claude doit comprendre votre base de code, il délègue la recherche au sous-agent Plan afin que la sortie d'exploration reste dans une fenêtre de contexte séparée tandis que la conversation principale reste en lecture seule.
  </Tab>

  <Tab title="General-purpose">
    Un agent capable pour les tâches complexes et multi-étapes qui nécessitent à la fois l'exploration et l'action.

    * **Modèle** : le modèle [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model) si vous en définissez un et que rien d'autre n'assigne un modèle d'une autre manière, sinon le modèle de la conversation principale ; [Choisir un modèle](#choose-a-model) indique l'ordre complet, et [Exécuter chaque sous-agent sur un modèle](#run-every-subagent-on-one-model) montre comment faire en sorte que la variable remplace ces sources
    * **Outils** : tous les outils [disponibles pour les sous-agents](#available-tools)
    * **Objectif** : recherche complexe, opérations multi-étapes, modifications de code

    Claude délègue à general-purpose lorsque la tâche nécessite à la fois l'exploration et la modification, un raisonnement complexe pour interpréter les résultats, ou plusieurs étapes dépendantes.
  </Tab>

  <Tab title="Other">
    Claude Code inclut des agents d'assistance supplémentaires pour des tâches spécifiques. Ceux-ci sont généralement invoqués automatiquement, vous n'avez donc pas besoin de les utiliser directement.

    | Agent             | Modèle                                                                                                                  | Quand Claude l'utilise                                                                                                                                                                                                                                                                                                                                                                                      |
    | :---------------- | :---------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | claude            | Aucun qui lui soit propre ; suit l'[ordre des modèles](#choose-a-model) lorsque Claude le génère en tant que sous-agent | Lorsqu'une tâche ne correspond pas à un agent plus spécialisé. Un agent fourre-tout avec tous les outils [disponibles pour les sous-agents](#available-tools). Également l'agent par défaut pour une [session en arrière-plan](/docs/fr/agent-view) distribuée ; [le mode de permission dans lequel il démarre](/docs/fr/agent-view#permission-mode-model-and-effort) dépend de la façon dont la session a été lancée |
    | statusline-setup  | Sonnet                                                                                                                  | Lorsque vous exécutez `/statusline` pour configurer votre ligne d'état                                                                                                                                                                                                                                                                                                                                      |
    | claude-code-guide | Haiku                                                                                                                   | Lorsque vous posez des questions sur les fonctionnalités de Claude Code                                                                                                                                                                                                                                                                                                                                     |
  </Tab>
</Tabs>

Les sous-agents intégrés sont enregistrés par défaut dans les sessions interactives. Pour les restreindre :

* Pour bloquer un type intégré spécifique, ajoutez-le à `permissions.deny` comme indiqué dans [Désactiver des sous-agents spécifiques](#disable-specific-subagents).
* Pour empêcher Claude de déléguer à un sous-agent, refusez l'outil `Agent` lui-même avec [`permissions.deny`](/docs/fr/permissions#tool-specific-permission-rules).
* Pour supprimer uniquement les sous-agents intégrés `Explore` et `Plan`, définissez [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/fr/env-vars). Claude lit et explore les fichiers directement au lieu de déléguer à ces sous-agents. Nécessite Claude Code v2.1.198 ou version ultérieure.
* En [mode non-interactif](/docs/fr/headless) et le [SDK Agent](/docs/fr/agent-sdk/overview), définissez [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/fr/env-vars) pour supprimer tous les types intégrés et fournir uniquement les vôtres.

Un appel d'outil Agent qui omet `subagent_type` échoue avec [`subagent_type is required`](/docs/fr/errors#subagent-type-is-required) lorsque la session n'a pas de sous-agent `general-purpose` sur lequel se replier.

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

    Claude délègue à votre nouveau sous-agent, qui analyse la base de code et retourne les suggestions d'amélioration. Dans la transcription, la délégation apparaît sous la forme d'une ligne d'appel d'outil montrant le nom du sous-agent suivi d'une brève description de tâche, comme `code-improver(Suggest code improvements)`.

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
| Répertoire `agents/` du plugin | Où le plugin est activé       | 5 (la plus basse)  | Installé avec les [plugins](/docs/fr/plugins/overview)   |

**Les sous-agents de projet** (`.claude/agents/`) sont idéaux pour les sous-agents spécifiques à une base de code. Enregistrez-les dans le contrôle de version pour que votre équipe puisse les utiliser et les améliorer de manière collaborative.

Les sous-agents de projet sont découverts en remontant à partir du répertoire de travail actuel, donc chaque `.claude/agents/` entre celui-ci et la racine du référentiel est analysé. Lorsque plusieurs de ces répertoires imbriqués définissent le même `name`, Claude Code utilise la définition la plus proche du répertoire de travail.

Lorsque vous ajoutez un répertoire avec `--add-dir` ou `/add-dir`, Claude Code charge également son dossier `.claude/agents/`, aux côtés de vos sous-agents de projet. Consultez [Répertoires supplémentaires](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) pour voir quels autres types de configuration se chargent à partir de `--add-dir`. Pour partager les sous-agents entre les projets sans `--add-dir`, utilisez `~/.claude/agents/` ou un [plugin](/docs/fr/plugins/overview).

**Les sous-agents utilisateur** (`~/.claude/agents/`) sont des sous-agents personnels disponibles dans tous vos projets.

Claude Code analyse `.claude/agents/` et `~/.claude/agents/` de manière récursive, vous pouvez donc organiser les définitions dans des sous-dossiers tels que `agents/review/` ou `agents/research/`. Le chemin du sous-répertoire n'affecte pas la façon dont un sous-agent est identifié ou invoqué, car l'identité provient uniquement du champ frontmatter `name`.

Gardez les valeurs `name` uniques dans tout l'arborescence : si deux fichiers dans le même répertoire `.claude/agents/`, y compris ses sous-dossiers, déclarent le même nom, Claude Code en charge un seul, choisi par l'ordre de lecture du système de fichiers plutôt que par une précédence documentée. Entre les répertoires de projet imbriqués, la définition la plus proche du répertoire de travail gagne, comme décrit ci-dessus. La vérification de configuration [`/doctor`](/docs/fr/commands#all-commands) signale les fichiers dans le même répertoire qui partagent un nom et propose de renommer ou de supprimer tous sauf un. Avant la v2.1.205, `/doctor` ouvrait un écran de diagnostics qui listait les doublons et montrait quelle définition était active.

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

Le drapeau `--agents` accepte JSON avec un champ `prompt` plus ces champs de [frontmatter](#supported-frontmatter-fields) : `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `omitClaudeMd` et `isolation`. Utilisez `prompt` pour l'invite système, équivalent au corps markdown dans les sous-agents basés sur fichier. `color` et `experimental` ne sont pas acceptés ici et sont ignorés plutôt que rejetés.

Chaque clé de niveau supérieur dans le JSON est le nom de l'agent. Ne commencez pas un nom par `-`.

Pour savoir ce que Claude Code fait avec une valeur qu'il ne peut pas charger, et les drapeaux et variables d'environnement qui ignorent cette vérification, consultez [`Invalid --agents configuration`](/docs/fr/errors#invalid-agents-configuration).

**Les sous-agents gérés** sont déployés par les administrateurs de l'organisation. Placez les fichiers markdown dans `.claude/agents/` à l'intérieur du [répertoire des paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms), en utilisant le même format de frontmatter que les sous-agents de projet et utilisateur. Les définitions gérées prennent précédence sur les sous-agents de projet et utilisateur portant le même nom.

**Les sous-agents de plugin** proviennent des [plugins](/docs/fr/plugins/overview) que vous avez installés. Ils se chargent automatiquement aux côtés de vos sous-agents personnalisés et apparaissent dans la saisie semi-automatique @-mention sous leur nom limité. Consultez la [référence des composants de plugin](/docs/fr/plugins/components#agents) pour plus de détails sur la création de sous-agents de plugin.

<Note>
  Pour des raisons de sécurité, les sous-agents de plugin ne prennent pas en charge les champs frontmatter `hooks`, `mcpServers` ou `permissionMode`. Ces champs sont ignorés lors du chargement des agents à partir d'un plugin. Si vous en avez besoin, copiez le fichier d'agent dans `.claude/agents/` ou `~/.claude/agents/`. Vous pouvez également ajouter des règles à [`permissions.allow`](/docs/fr/settings-reference#permissions-allow) dans `settings.json` ou `settings.local.json`, mais ces règles s'appliquent à l'ensemble de la session, pas seulement au sous-agent du plugin.
</Note>

Les définitions de sous-agent de l'une de ces portées sont également disponibles pour les [équipes d'agents](/docs/fr/agent-teams#use-subagent-definitions-for-teammates) : lors du lancement d'un coéquipier, vous pouvez référencer un type de sous-agent, et Claude Code applique des parties de cette définition au coéquipier. Consultez [équipes d'agents](/docs/fr/agent-teams#use-subagent-definitions-for-teammates) pour voir quelles parties s'appliquent dans chaque mode d'affichage.

<h3 id="write-subagent-files">
  Écrire des fichiers de sous-agent
</h3>

Les fichiers de sous-agent utilisent du frontmatter YAML pour la configuration, suivi de l'invite système en Markdown :

<Note>
  Claude Code surveille `~/.claude/agents/` et `.claude/agents/`. Lorsque vous ajoutez ou modifiez un fichier de sous-agent sur le disque, ou demandez à Claude d'en écrire un pour vous, Claude Code détecte le changement en quelques secondes et la prochaine délégation utilise la définition mise à jour, sans redémarrage nécessaire.

  Trois cas nécessitent toujours un redémarrage :

  * L'observateur couvre uniquement les répertoires qui existaient au démarrage de la session, donc après la création du premier fichier d'agent d'une portée dans un nouveau répertoire `agents`, redémarrez pour le charger.
  * Claude Code ne surveille pas `.claude/agents/` à l'intérieur des répertoires ajoutés avec `--add-dir` ou `/add-dir`, donc après l'ajout ou la modification d'un sous-agent là-bas, redémarrez pour charger la modification.
  * Les sessions démarrées avec `--disable-slash-commands` ne surveillent pas du tout ces répertoires.
</Note>

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Le frontmatter définit les métadonnées et la configuration du sous-agent. Le corps devient l'invite système qui guide le comportement du sous-agent. Les sous-agents reçoivent uniquement cette invite système plus les détails d'environnement de base comme le répertoire de travail, pas l'invite système de Claude Code.

En [mode non interactif](/docs/fr/headless), passez [`--append-subagent-system-prompt`](/docs/fr/cli-reference#cli-flags) pour ajouter votre texte à la fin de l'invite système de chaque sous-agent, y compris les sous-agents imbriqués, à l'exception d'un [sous-agent forké](#fork-the-current-conversation), qui réutilise l'invite de la conversation. Nécessite Claude Code v2.1.205 ou ultérieur. Si votre texte est trop long pour être passé sur la ligne de commande, enregistrez-le dans un fichier et passez le chemin avec `--append-subagent-system-prompt-file` à la place. Le drapeau de fichier nécessite Claude Code v2.1.261 ou ultérieur.

Un sous-agent démarre dans le répertoire de travail actuel de la conversation principale. Au sein d'un sous-agent, les commandes `cd` ne persistent pas entre les appels d'outils Bash ou PowerShell et n'affectent pas le répertoire de travail de la conversation principale. Pour donner au sous-agent une copie isolée du référentiel à la place, définissez [`isolation: worktree`](#supported-frontmatter-fields).

Un sous-agent avec `isolation: worktree` exécute ses commandes Bash et PowerShell à l'intérieur de son worktree. Une commande dont le répertoire de travail se résout à votre extraction principale à la place, par exemple parce que le répertoire worktree a été supprimé pendant que le sous-agent s'exécutait, échoue avec une erreur. Avant la v2.1.203, une telle commande pouvait s'exécuter dans l'extraction principale.

Cette vérification du répertoire de travail couvre l'ensemble du référentiel contenant le répertoire à partir duquel vous avez lancé Claude Code. Lorsque votre session s'exécute dans un [worktree](/docs/fr/worktrees) lié de son propre chef, la vérification couvre également l'extraction principale à partir de laquelle ce worktree est lié. Avant la v2.1.210, la vérification couvrait uniquement le répertoire de lancement lui-même. Une commande dont le répertoire de travail se résout ailleurs dans le même référentiel, comme la racine du référentiel lorsque vous avez lancé Claude Code à partir d'un sous-répertoire monorepo, s'y exécutait à la place d'échouer.

Pour les commandes Bash, Claude Code vérifie également la commande elle-même de deux façons :

* Il bloque une commande qui redirige git vers l'extraction principale.
* Il refuse une commande lorsqu'il ne peut pas vérifier à partir du texte de la commande que tout git que la commande exécute reste à l'intérieur du worktree, par exemple lorsque le nom de la commande est calculé à l'exécution.

Les vecteurs de redirection et les règles de forme sont listés sous [Comment Claude Code applique l'isolation](/docs/fr/worktrees#how-claude-code-enforces-isolation). Les commandes PowerShell ne reçoivent que la vérification du répertoire de travail.

Les commandes [Monitor](/docs/fr/tools-reference#monitor-tool) passent par les mêmes vérifications du répertoire de travail et du contenu de la commande que les commandes Bash.

Lorsque la conversation principale elle-même s'exécute isolée dans un worktree, Claude Code applique les mêmes vérifications à la session et à chaque sous-agent qu'il génère, y compris les sous-agents sans `isolation: worktree` ; consultez [Comment Claude Code applique l'isolation](/docs/fr/worktrees#how-claude-code-enforces-isolation).

<h3 id="supported-frontmatter-fields">
  Référence de frontmatter
</h3>

Configurez un sous-agent avec du [frontmatter](/docs/fr/glossary#frontmatter) YAML entre les marqueurs `---` en haut de son fichier, et écrivez son invite système en Markdown après le `---` de fermeture. Seuls `name` et `description` sont obligatoires.

Les noms de champs multi-mots utilisent camelCase, tels que `maxTurns` et `disallowedTools`, et doivent correspondre exactement au tableau : Claude Code ignore un champ qu'il ne reconnaît pas sans signaler une erreur. Pour savoir pourquoi un fichier de sous-agent ne s'est pas chargé, consultez [Fichiers de sous-agent que Claude Code ignore](#subagent-files-claude-code-skips).

| Champ             | Obligatoire | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :---------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Oui         | Identifiant unique, tel que `code-reviewer` ou `reviewer-v2`. Les [Hooks](/docs/fr/hooks#subagentstart) reçoivent cette valeur comme `agent_type`. Le nom du fichier n'a pas besoin de correspondre. Les noms ne peuvent pas contenir `:`, qui est réservé aux [identifiants limités au plugin](/docs/fr/plugins/overview) tels que `my-plugin:reviewer`. Claude Code ne charge pas un fichier dont le nom en contient un et enregistre une erreur dans le journal de débogage. Avant la v2.1.218, de tels noms étaient acceptés                                                        |
| `description`     | Oui         | Quand Claude doit déléguer à ce sous-agent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `tools`           | Non         | [Outils](#available-tools) que le sous-agent peut utiliser, sous forme de chaîne séparée par des virgules telle que `Read, Grep, Bash` ou une liste YAML. Hérite de tous les outils disponibles pour les sous-agents s'il est omis. Si aucune entrée de la liste ne se résout en un outil, le sous-agent échoue généralement au [lancement](/docs/fr/errors#agent-would-be-spawned-with-zero-tools) avec une erreur nommant les entrées. Pour précharger les Skills dans le contexte, utilisez le champ `skills` plutôt que de lister `Skill` ici                                  |
| `disallowedTools` | Non         | Outils à refuser, supprimés de la liste héritée ou spécifiée. Même format que `tools`. Une entrée avec un spécificateur, tel que `Bash(git push *)`, supprime toujours l'[outil entier](#available-tools)                                                                                                                                                                                                                                                                                                                                                                     |
| `model`           | Non         | [Modèle](#choose-a-model) à utiliser : `sonnet`, `opus`, `haiku`, `fable`, un ID de modèle complet tel que `claude-opus-5-5`, ou `inherit`. Lorsque vous l'omettez, Claude Code choisit le modèle dans l'[ordre du modèle de sous-agent](#choose-a-model)                                                                                                                                                                                                                                                                                                                     |
| `permissionMode`  | Non         | [Mode de permission](#permission-modes) : `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, ou `manual` comme alias pour `default`. L'alias `manual` nécessite Claude Code v2.1.200 ou ultérieur. Ignoré pour les [sous-agents de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                                 |
| `maxTurns`        | Non         | Nombre maximum de tours d'agent avant que le sous-agent s'arrête. Lorsque le sous-agent atteint la limite, Claude Code retourne sa sortie marquée comme partielle, et Claude peut la [reprendre](#resume-subagents) pour continuer. Le marquage partiel nécessite Claude Code v2.1.246 ou ultérieur                                                                                                                                                                                                                                                                           |
| `skills`          | Non         | [Skills](/docs/fr/skills) à précharger dans le contexte du sous-agent au démarrage. Le contenu complet de la skill est injecté, pas seulement la description. Les sous-agents peuvent toujours invoquer les skills de projet, utilisateur et plugin non listées via l'outil Skill                                                                                                                                                                                                                                                                                                  |
| `mcpServers`      | Non         | [Serveurs MCP](/docs/fr/mcp) disponibles pour ce sous-agent. Chaque entrée est soit un nom de serveur référençant un serveur déjà configuré (par exemple, `"slack"`) soit une définition en ligne avec le nom du serveur comme clé et une [configuration de serveur MCP](/docs/fr/mcp#installing-mcp-servers) complète comme valeur. Ignoré pour les [sous-agents de plugin](#choose-the-subagent-scope)                                                                                                                                                                                |
| `hooks`           | Non         | [Hooks de cycle de vie](#define-hooks-for-subagents) limités à ce sous-agent. Ignoré pour les [sous-agents de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `memory`          | Non         | [Portée de la mémoire persistante](#enable-persistent-memory) : `user`, `project` ou `local`. Active l'apprentissage entre sessions                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `background`      | Non         | Définir sur `true` pour garder ce sous-agent en arrière-plan même lorsque Claude demande de l'exécuter au premier plan. Lorsque le [mode fork](#turn-fork-mode-on-or-off) est activé, Claude Code exécute déjà les sous-agents que Claude génère [en arrière-plan](#run-subagents-in-foreground-or-background)                                                                                                                                                                                                                                                                |
| `omitClaudeMd`    | Non         | Définir sur `true` pour lancer ce sous-agent sans les fichiers CLAUDE.md utilisateur, projet et local ; les [fichiers de politique gérée](/docs/fr/memory#how-claude-md-files-load) se chargent toujours, sauf pour les [sous-agents gérés](#choose-the-subagent-scope). Utilisez-le pour les sous-agents qui prennent tout ce dont ils ont besoin de l'[invite de délégation](#what-loads-at-startup). Ignoré lorsque l'agent s'exécute en tant qu'agent de session principal via `--agent` ou le paramètre `agent`. Nécessite Claude Code v2.1.271 ou ultérieur                  |
| `effort`          | Non         | Niveau d'effort lorsque ce sous-agent est actif. Remplace le niveau d'effort de la session. Par défaut : hérite de la session. Options : `low`, `medium`, `high`, `xhigh`, `max` ; les niveaux disponibles dépendent du modèle                                                                                                                                                                                                                                                                                                                                                |
| `isolation`       | Non         | Définir sur `worktree` pour exécuter le sous-agent dans un [git worktree](/docs/fr/worktrees) temporaire, ce qui lui donne une copie isolée du référentiel branchée par défaut à partir de votre [branche par défaut](/docs/fr/worktrees#choose-the-base-branch) plutôt que du `HEAD` de la session parent. Le worktree est automatiquement nettoyé si le sous-agent n'apporte aucune modification                                                                                                                                                                                      |
| `color`           | Non         | Couleur d'affichage pour le sous-agent dans la liste des tâches et la transcription. Accepte `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` ou `cyan`                                                                                                                                                                                                                                                                                                                                                                                                           |
| `initialPrompt`   | Non         | Auto-soumis comme le premier tour utilisateur lorsque cet agent s'exécute en tant qu'agent de session principal (via `--agent` ou le paramètre `agent`). Les [commandes](/docs/fr/commands) et les [skills](/docs/fr/skills) sont traitées. Préfixé à tout invite fourni par l'utilisateur. Ignoré pour les [sous-agents de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                         |
| `experimental`    | Non         | Carte des options expérimentales. Définissez sa clé `cacheTtl` sur `5m` ou `1h` pour choisir la [durée de vie du cache d'invite](/docs/fr/prompt-caching#choose-the-ttl-yourself) pour les demandes de ce sous-agent, à la place du frontmatter dans la [précédence de la durée de vie du cache](/docs/fr/prompt-caching#choose-the-ttl-yourself). Claude Code ignore toute autre valeur, ignore `1h` tandis que votre abonnement Claude utilise des crédits d'utilisation, et lit le champ uniquement à partir des fichiers de sous-agent. Nécessite Claude Code v2.1.248 ou ultérieur |

Écrivez `cacheTtl` à l'intérieur de la carte `experimental`, pas au niveau supérieur du frontmatter.

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

<h4 id="subagent-files-claude-code-skips">
  Fichiers de sous-agent que Claude Code ignore
</h4>

Claude Code ignore un fichier dans un répertoire `agents` de projet, utilisateur ou géré, ou dans un répertoire que vous ajoutez avec `--add-dir`, sans le signaler dans la session, lorsque le frontmatter a l'un de ces problèmes :

* **Pas de `name`** : Claude Code traite le fichier comme de la documentation conservée à côté de vos agents.
* **Un `---` d'ouverture qui n'est pas la première ligne du fichier** : Claude Code lit le fichier comme n'ayant pas de frontmatter et le traite comme de la documentation.
* **Un `name` qui commence par `-` ou contient `:`** : Claude Code ignore le fichier et écrit une erreur dans le journal de débogage. Consultez la ligne `name` dans le tableau ci-dessus.
* **Un `name` mais pas de `description`** : Claude Code ignore le fichier et écrit la raison dans le journal de débogage.
* **YAML qui ne s'analyse pas** : Claude Code ne lit aucun champ du fichier, l'ignore et écrit l'erreur d'analyse dans le journal de débogage.

Pour voir le journal de débogage, exécutez Claude Code avec `--debug`.

Un [sous-agent de plugin](/docs/fr/plugins/components#agents) dont le frontmatter n'a pas de `name` ou ne s'analyse pas se charge toujours, sous son nom de fichier.

<h5 id="check-an-agents-directory-before-a-session">
  Vérifier un répertoire `agents` avant une session
</h5>

Pour trouver les fichiers dans un répertoire `agents` dont le frontmatter ne s'analyse pas, exécutez `claude plugin validate` contre le répertoire, par exemple `.claude/agents` ou `~/.claude/agents`. Claude Code vérifie uniquement [le répertoire que vous nommez](/docs/fr/plugins/cli-reference#validate-a-directory), et ne signale pas un fichier dont le frontmatter s'analyse mais n'a pas de `name`. Nécessite Claude Code v2.1.233 ou ultérieur.

<h3 id="choose-a-model">
  Choisir un modèle
</h3>

Le champ `model` contrôle quel modèle le sous-agent utilise :

* **Alias de modèle** : utilisez l'un des alias disponibles : `sonnet`, `opus`, `haiku` ou `fable`
* **ID de modèle complet** : utilisez un ID de modèle complet tel que `claude-opus-5-5` ou `claude-sonnet-5`. Accepte les mêmes valeurs que le drapeau `--model`
* **inherit** : utilisez le même modèle que la conversation principale

Lorsque Claude invoque un sous-agent, il peut également passer un paramètre `model` pour cette invocation spécifique. Claude Code résout le modèle du sous-agent dans cet ordre :

1. Le paramètre `model` par invocation
2. Le frontmatter `model` de la définition du sous-agent, où `inherit` sélectionne le modèle de la conversation principale
3. La variable d'environnement [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/fr/model-config#environment-variables), lorsque vous la définissez sur un alias de modèle ou un ID de modèle
4. Le modèle de la conversation principale

Dans deux cas, un alias de famille tel que `opus` dans le paramètre par invocation ou le frontmatter se résout au modèle de la conversation principale au lieu de la [version vers laquelle l'alias pointe](/docs/fr/model-config#model-aliases) :

* **Le modèle de la conversation principale appartient à cette famille** : le sous-agent s'exécute sur le modèle exact de la conversation principale, y compris tout suffixe `[1m]`, donc il obtient la même fenêtre de [contexte étendu](/docs/fr/model-config#extended-context) que la conversation principale.
* **Claude Code ne peut pas dire la famille du modèle de la conversation principale, sur [un fournisseur autre que l'API Anthropic](/docs/fr/third-party-integrations)** : cela peut se produire avec un [ARN de profil d'inférence d'application](/docs/fr/amazon-bedrock#iam-configuration) sur Amazon Bedrock que Claude Code n'a pas résolu à un modèle de support. Ce cas couvre uniquement l'alias `opus`, et ne s'applique pas lorsque vous définissez [`ANTHROPIC_DEFAULT_OPUS_MODEL`](/docs/fr/model-config#environment-variables), puisque `opus` se résout alors au modèle que vous définissez.

Un alias dans `CLAUDE_CODE_SUBAGENT_MODEL` se résout toujours à la version vers laquelle l'alias pointe, même lorsqu'il nomme la famille de la conversation principale.

Définir `CLAUDE_CODE_SUBAGENT_MODEL` seul ne change pas le modèle sur lequel les sous-agents Explore et Plan intégrés s'exécutent. Pour le changer, consultez [Exécuter chaque sous-agent sur un modèle](#run-every-subagent-on-one-model).

Avant la v2.1.251, `CLAUDE_CODE_SUBAGENT_MODEL` venait en premier dans cet ordre et remplaçait à la fois le paramètre par invocation et le frontmatter, y compris `model: inherit`.

Définir la variable sur `inherit` est identique à la laisser non définie. Avant la v2.1.196, cette valeur forçait les sous-agents sur le modèle de la conversation principale et ignorait les autres sources.

Claude Code vérifie le paramètre par invocation, le frontmatter et les valeurs de la variable d'environnement par rapport à la liste blanche [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation. Pour une valeur bloquée, il substitue un autre modèle :

* Lorsque la valeur bloquée est un alias de famille tel que `opus`, Claude Code exécute le sous-agent sur la version la plus récente de cette famille que la liste blanche permet, en suivant les mêmes [règles de substitution et portée du fournisseur](/docs/fr/model-config#restrict-model-selection) que `/model`. Avant la v2.1.222, Claude Code exécutait le sous-agent sur le modèle hérité pour un alias de famille bloqué également.
* Pour toute autre valeur bloquée, sur les fournisseurs où cette substitution ne fonctionne pas, ou lorsque la liste blanche ne permet aucune version de la famille, Claude Code exécute le sous-agent sur le modèle hérité à la place. Si vous définissez `CLAUDE_CODE_SUBAGENT_MODEL`, Claude Code essaie d'abord ce modèle, selon les mêmes règles.

Dans les sessions interactives, Claude Code affiche un avertissement nommant le modèle demandé et le modèle sur lequel le sous-agent s'exécute, pour l'une ou l'autre substitution.

Pour vérifier sur quel modèle un sous-agent s'exécute, exécutez [`/tasks`](/docs/fr/commands). Claude Code nomme le modèle sur la ligne du sous-agent, et ajoute le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) lorsque la définition du sous-agent, ou la skill dont il a forké, définit [`effort`](#supported-frontmatter-fields). Nécessite Claude Code v2.1.242 ou ultérieur.

Un paramètre `model` par invocation s'applique également lorsque le sous-agent est [repris ou reçoit un message de suivi](#resume-subagents), donc le sous-agent reste sur ce modèle. Avant la v2.1.211, la reprise supprimait la valeur par invocation et le sous-agent revenait au champ `model` de sa définition ou, sans un, au modèle de la conversation principale.

À partir de la v2.1.198, les sous-agents héritent également de la configuration [extended thinking](/docs/fr/model-config#extended-thinking) de la conversation principale : si la réflexion est activée dans votre session, elle est activée pour le sous-agent, et si elle est désactivée, elle reste désactivée. Il n'y a pas de paramètre de réflexion par sous-agent. Avant la v2.1.198, les sous-agents s'exécutaient avec la réflexion étendue désactivée indépendamment du paramètre de la conversation principale.

<h4 id="run-every-subagent-on-one-model">
  Exécuter chaque sous-agent sur un modèle
</h4>

`CLAUDE_CODE_SUBAGENT_MODEL` est une valeur par défaut, donc la définition d'un sous-agent ou un modèle que Claude passe prend toujours précédence sur elle. Pour appliquer un modèle à chaque sous-agent, [coéquipier](/docs/fr/agent-teams#specify-teammates-and-models) et [agent de flux de travail](/docs/fr/workflows), définissez également `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` sur `1`. Nécessite Claude Code v2.1.257 ou ultérieur.

* Si vous définissez les deux variables, les sous-agents s'exécutent sur le modèle dans `CLAUDE_CODE_SUBAGENT_MODEL`.
* Si vous définissez uniquement `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, les sous-agents s'exécutent sur le modèle de la conversation principale.

Par exemple, pour exécuter chaque sous-agent sur Haiku, définissez les deux variables dans le bloc `env` d'un [fichier de paramètres](/docs/fr/settings) :

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

Pour vérifier que le paramètre a pris effet, exécutez [`/tasks`](/docs/fr/commands) pendant qu'un sous-agent s'exécute. La ligne du sous-agent affiche le modèle sur lequel il s'exécute.

Tandis que `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` est [activé](/docs/fr/env-vars), Claude Code ignore le champ `model` de chaque définition de sous-agent, y compris les sous-agents Explore et Plan intégrés, et Claude ne peut pas passer un modèle lorsqu'il démarre un sous-agent. Deux types de sous-agent s'exécutent toujours sur le modèle de la conversation principale :

* Un [fork](#fork-the-current-conversation)
* Une [skill qui s'exécute dans un sous-agent](/docs/fr/skills#run-skills-in-a-subagent) avec `model: inherit`

Lorsque vous définissez uniquement `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, le sous-agent Explore intégré conserve son [plafond de modèle](#built-in-subagents).

<h3 id="control-subagent-capabilities">
  Contrôler les capacités des sous-agents
</h3>

Vous pouvez contrôler ce que les sous-agents peuvent faire via l'accès aux outils, les modes de permission et les règles conditionnelles.

<h4 id="available-tools">
  Outils disponibles
</h4>

Les sous-agents héritent des [outils intégrés](/docs/fr/tools-reference) et des outils MCP disponibles dans la conversation principale, réduits par deux filtres : le premier supprime une courte liste d'outils de chaque sous-agent, et le second réduit l'ensemble des outils intégrés pour les sous-agents qui s'exécutent en [arrière-plan](#run-subagents-in-foreground-or-background), ce qui est la valeur par défaut. Sur macOS, Linux et WSL, un sous-agent peut également recevoir les outils Glob et Grep lorsque la conversation principale ne les a pas, comme décrit sous [Comportement de l'outil Glob](/docs/fr/tools-reference#glob-tool-behavior). Les [Forks](#fork-the-current-conversation) ignorent les deux filtres et reçoivent le pool d'outils exact de la conversation principale. Le premier filtre supprime ces outils, même lorsqu'ils sont listés dans le champ `tools` :

* `Agent`, lorsque le sous-agent est à la [limite de profondeur](#let-subagents-spawn-their-own-subagents) ; dans un [fork](#fork-the-current-conversation) l'outil reste listé mais retourne une erreur à la place de générer
* `AskUserQuestion`
* `EndConversation`, qui ne peut terminer que la conversation principale ; consultez [Comportement de l'outil EndConversation](/docs/fr/tools-reference#endconversation-tool-behavior)
* `EnterPlanMode`
* `ExitPlanMode`, sauf si le [`permissionMode`](#permission-modes) du sous-agent est `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

Le second filtre s'applique aux sous-agents s'exécutant en arrière-plan. À part `Agent` et `ExitPlanMode`, qui suivent les conditions du premier filtre partout où le sous-agent s'exécute, un sous-agent en arrière-plan conserve tous les outils MCP mais uniquement ces outils intégrés : `Read`, `Grep`, `Glob`, `LSP`, `Bash`, `PowerShell`, `Edit`, `Write`, `NotebookEdit`, `WebFetch`, `WebSearch`, `TodoWrite`, `Skill`, `ToolSearch`, `EnterWorktree`, `ExitWorktree`, `Monitor`, `TaskStop`, `SendMessage` et `Artifact`, plus [`SubagentHandback`](/docs/fr/tools-reference) pour un sous-agent qui rapporte via celui-ci. Claude Code supprime tous les autres outils intégrés d'un sous-agent en arrière-plan, qu'ils soient hérités ou listés dans le champ `tools`, donc la même définition peut se résoudre en outils différents au premier plan et en arrière-plan. La suppression ne signale aucune erreur sauf si elle laisse la liste `tools` [se résoudre à rien](/docs/fr/errors#agent-would-be-spawned-with-zero-tools).

Avant la v2.1.280, les sous-agents en arrière-plan ne pouvaient pas utiliser `LSP`.

[`ListAgents`](/docs/fr/cross-session-messaging) suit ces filtres comme n'importe quel outil intégré : un sous-agent au premier plan l'hérite dans les sessions où la messagerie entre sessions est activée, et un sous-agent en arrière-plan ne le conserve pas.

Les coéquipiers dans les [équipes d'agents](/docs/fr/agent-teams) conservent en outre les outils de tâche et les outils cron : `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `CronCreate`, `CronDelete` et `CronList`.

Dans une [session sans les outils Task](/docs/fr/tools-reference#task-tool-availability), Claude Code ne fournit pas les outils de tâche aux sous-agents non plus, même lorsque le sous-agent exécute un modèle différent. Un coéquipier en processus suit votre session de la même manière, tandis qu'un coéquipier dans son propre [volet divisé](/docs/fr/agent-teams#choose-a-display-mode) s'exécute en tant que processus Claude Code séparé, donc son propre modèle décide.

Pour restreindre les outils, utilisez le champ `tools` comme liste blanche ou le champ `disallowedTools` comme liste noire. Cet exemple utilise `tools` pour autoriser uniquement Read, Grep, Glob et Bash. Le sous-agent ne peut pas modifier les fichiers, écrire des fichiers ou utiliser des outils MCP :

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Cet exemple utilise `disallowedTools` pour hériter du pool d'outils du sous-agent sauf Write et Edit. Le sous-agent conserve Bash, les outils MCP et le reste de son pool :

```yaml theme={null}
---
name: no-writes
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

Si les deux sont définis, `disallowedTools` est appliqué en premier, puis `tools` est résolu par rapport au pool restant. Un outil listé dans les deux est supprimé.

Lorsque rien dans la liste `tools` ne se résout en un outil, par exemple parce que chaque entrée est mal orthographiée ou nomme un outil qui n'est pas disponible pour les sous-agents, Claude Code refuse généralement de lancer le sous-agent et l'outil Agent retourne une erreur nommant les entrées non résolues ; consultez [Agent would be spawned with zero tools](/docs/fr/errors#agent-would-be-spawned-with-zero-tools) pour le message et comment corriger chaque entrée. Avant la v2.1.208, ce sous-agent se lançait sans outils et pouvait retourner un résultat vide ou confus.

Les deux champs acceptent des modèles au niveau du serveur MCP en plus des noms d'outils exacts : `mcp__<server>` ou `mcp__<server>__*` accorde ou supprime tous les outils du serveur nommé. Dans `disallowedTools`, `mcp__*` supprime également tous les outils MCP de n'importe quel serveur. Cet exemple supprime tous les outils du serveur MCP `github` tout en conservant les outils d'autres serveurs et les outils intégrés dans son pool :

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

Une entrée `disallowedTools` avec un spécificateur, tel que `Bash(git push *)`, supprime toujours l'outil entier du sous-agent, pas seulement les commandes correspondantes. Pour conserver Bash et bloquer des commandes spécifiques, ajoutez une [règle de refus Bash](/docs/fr/permissions#bash) telle que `Bash(git push *)` à `permissions.deny` dans vos paramètres. La règle s'applique à la conversation principale et aux sous-agents.

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

Si vous omettez complètement `Agent` de la liste `tools`, l'agent ne peut générer aucun sous-agent avec l'outil Agent.

La syntaxe de liste blanche `Agent(agent_type)` s'applique uniquement à un agent s'exécutant en tant que thread principal avec `claude --agent`. Dans une définition de sous-agent, lister `Agent` dans `tools` permet à ce sous-agent de générer des sous-agents de son propre chef tandis que la [limite de profondeur](#let-subagents-spawn-their-own-subagents) le permet, mais toute liste de types à l'intérieur des parenthèses est ignorée.

<h4 id="scope-mcp-servers-to-a-subagent">
  Limiter les serveurs MCP à un sous-agent
</h4>

Utilisez le champ `mcpServers` pour donner à un sous-agent l'accès aux serveurs [MCP](/docs/fr/mcp) qui ne sont pas disponibles dans la conversation principale. Les serveurs en ligne définis ici sont connectés au démarrage du sous-agent, selon la [règle de confiance pour le dossier du fichier d'agent](#inline-server-trust), et déconnectés à la fin. Les références de chaîne partagent la connexion de la session parent.

<Note>
  Le champ `mcpServers` s'applique dans les deux contextes où un fichier d'agent peut s'exécuter :

  * En tant que sous-agent, généré via l'outil Agent ou une @-mention
  * En tant que session principale, lancée avec [`--agent`](#invoke-subagents-explicitly) ou le paramètre `agent`

  Lorsque l'agent est la session principale, les définitions de serveur en ligne se connectent au démarrage aux côtés des serveurs de [`.mcp.json`](/docs/fr/mcp) et des fichiers de paramètres, selon la même [règle de confiance pour le dossier du fichier d'agent](#inline-server-trust). Dans `/mcp`, un serveur distant (HTTP ou SSE) que vous avez utilisé auparavant peut afficher le statut [`cached`](/docs/fr/mcp#managing-your-servers) à la place ; Claude Code le connecte lorsque Claude appelle d'abord l'un de ses outils.
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

<span id="inline-server-trust" />Claude Code charge un serveur en ligne à partir d'un fichier d'agent dans le répertoire `.claude/agents/` de votre projet, ou dans le répertoire `.claude/agents/` d'un répertoire `--add-dir`, uniquement après que vous [fassiez confiance au dossier d'où provient le fichier d'agent](/docs/fr/permissions#what-runs-before-you-trust-a-folder). Avant la v2.1.238, Claude Code chargeait ces serveurs sans vérifier la confiance.

* **La confiance qui ne compte pas** : la confiance d'un dossier parent, et la confiance automatique qu'une session `-p` ou SDK obtient pour les [hooks dans les fichiers de paramètres](/docs/fr/permissions#what-runs-before-you-trust-a-folder)
* **Jusqu'à ce moment** : Claude Code ignore tous les serveurs en ligne dans ce fichier d'agent et écrit la clé exacte `projects["<path>"].hasTrustDialogAccepted` pour `~/.claude.json` dans le journal de débogage
* **Répertoires `--add-dir`** : un répertoire en dehors du référentiel de l'espace de travail de confiance de votre organisation a besoin de sa propre entrée de confiance, car ses fichiers `.claude/agents/` n'héritent pas de la confiance de votre espace de travail

Claude Code charge deux types de serveur sans vérifier la confiance pour le dossier d'où provient le fichier d'agent :

* Un nom qui référence un serveur que vous avez déjà configuré
* Un serveur en ligne dans un fichier d'agent de `~/.claude/agents/`, dans un que vous passez avec `--agents` ou l'option `agents` du SDK, ou dans un que les paramètres gérés fournissent

Les restrictions MCP qui s'appliquent à la session principale couvrent également les serveurs déclarés dans le frontmatter du sous-agent :

* [`--strict-mcp-config`](/docs/fr/cli-reference) et [`--bare`](/docs/fr/cli-reference)
* [Configuration MCP gérée en entreprise](/docs/fr/managed-mcp)
* [Politiques `allowedMcpServers` et `deniedMcpServers`](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Lorsque l'une de ces options bloque un serveur, Claude Code le saute et affiche un avertissement nommant les serveurs bloqués.

Les restrictions des paramètres gérés s'appliquent à chaque sous-agent indépendamment de la façon dont il est défini. `--strict-mcp-config` ne filtre pas les serveurs que vous transmettez en ligne via `--agents` ou l'option `agents` du SDK, car il s'agit d'une entrée explicite de l'appelant.

<h4 id="permission-modes">
  Modes de permission
</h4>

Définissez `permissionMode` pour choisir le mode de permission dans lequel un sous-agent s'exécute. Utilisez les valeurs de configuration des modes, donc le mode Manuel est `default`. Si vous le laissez non défini, le sous-agent hérite du mode de la conversation principale, qui commence en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) sur les plans Pro, Max et Team sauf si vos paramètres ou votre organisation le changent.

Le mode de permission de la conversation principale décide si Claude Code utilise la valeur que vous définissez :

* Lorsque la conversation principale est en `bypassPermissions`, `acceptEdits` ou [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), le sous-agent s'exécute dans ce même mode et Claude Code ignore le `permissionMode` que vous définissez. En mode auto, le classificateur évalue les appels d'outils du sous-agent avec les règles de blocage et d'autorisation de la conversation principale. Lorsque le sous-agent se termine, le classificateur examine également son travail et son rapport final avant que le rapport soit livré, comme [Comment le mode auto gère les sous-agents](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) le décrit.
* Lorsque la conversation principale est en mode `default`, `dontAsk` ou `plan`, le sous-agent s'exécute dans le mode de permission que vous définissez, sauf `bypassPermissions`. Un sous-agent qui déclare `bypassPermissions` conserve le mode de la conversation principale à la place. L'exception `bypassPermissions` nécessite Claude Code v2.1.267 ou ultérieur.

`permissionMode` accepte ces valeurs, et `manual` comme alias pour `default` :

| Mode                | Comportement                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Mode Manuel : demande la permission                                                                                                                                                                                                                                                                                                                                                                                                               |
| `acceptEdits`       | Auto-accepter les modifications de fichiers et les commandes courantes du système de fichiers pour les chemins du répertoire de travail ou `additionalDirectories`                                                                                                                                                                                                                                                                                |
| `auto`              | [Mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) : un classificateur examine les commandes et les écritures de répertoire protégé                                                                                                                                                                                                                                                                                               |
| `dontAsk`           | Auto-refuser les invites de permission. Les outils explicitement autorisés fonctionnent toujours ; `AskUserQuestion`, les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) et les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) dans les sessions où ce paramètre atteint Claude Code sont refusés même si vous les avez autorisés |
| `bypassPermissions` | [Ignorer les invites de permission](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode). Un sous-agent s'exécute dans ce mode uniquement lorsque la conversation principale le fait                                                                                                                                                                                                                                                 |
| `plan`              | Mode plan (exploration en lecture seule)                                                                                                                                                                                                                                                                                                                                                                                                          |

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

Vous ne pouvez pas précharger les skills qui définissent [`disable-model-invocation: true`](/docs/fr/skills#control-who-invokes-a-skill), car le préchargement provient du même ensemble de skills que Claude peut invoquer. Cela inclut la skill `/verify` groupée : seul vous pouvez l'exécuter, donc elle ne peut pas être préchargée non plus.

Si une skill listée est manquante ou désactivée, par exemple par la politique de votre organisation, Claude Code la saute et enregistre un avertissement dans le journal de débogage.

<Note>
  C'est l'inverse de [l'exécution d'une skill dans un sous-agent](/docs/fr/skills#run-skills-in-a-subagent). Avec `skills` dans un sous-agent, le sous-agent contrôle l'invite système et charge le contenu de la skill. Avec `context: fork` dans une skill, le contenu de la skill est injecté dans l'agent que vous spécifiez. Dans les deux cas, le sous-agent démarre sans votre historique de conversation.
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

La mémoire du sous-agent fait partie de la [mémoire automatique](/docs/fr/memory#auto-memory) : si vous désactivez la mémoire automatique, avec le paramètre `autoMemoryEnabled` ou `CLAUDE_CODE_DISABLE_AUTO_MEMORY`, le champ `memory` n'a aucun effet et le sous-agent se lance sans les instructions de mémoire ou l'accès à l'outil de mémoire décrit ci-dessous.

Lorsque la mémoire est activée :

* L'invite système du sous-agent inclut des instructions pour lire et écrire dans le répertoire de mémoire.
* L'invite système du sous-agent inclut également les 200 premières lignes ou 25 KB de `MEMORY.md` dans le répertoire de mémoire, selon la première limite atteinte, avec des instructions pour organiser `MEMORY.md` s'il dépasse cette limite.
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

Sur macOS et Linux, rendez le script exécutable, sinon le hook échoue au lieu de bloquer quoi que ce soit :

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Pour tester la règle, demandez au sous-agent d'exécuter une instruction `UPDATE` : le script quitte avec le code 2, Claude Code bloque la commande, et le sous-agent voit le message `Blocked: Only SELECT queries are allowed`.

Consultez [Hook input](/docs/fr/hooks#pretooluse-input) pour le schéma d'entrée complet et [exit codes](/docs/fr/hooks#exit-code-output) pour savoir comment les codes de sortie affectent le comportement. Sur Windows, écrivez les scripts de hook en PowerShell et ajoutez `shell: powershell` à l'entrée du hook comme indiqué dans [exécution de hooks en PowerShell](/docs/fr/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Désactiver des sous-agents spécifiques
</h4>

Vous pouvez empêcher Claude d'utiliser des sous-agents spécifiques en les ajoutant au tableau `deny` dans vos [paramètres](/docs/fr/settings-reference#permission-settings). Utilisez le format `Agent(subagent-name)` où `subagent-name` correspond au champ name du sous-agent.

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

* **Dans le frontmatter du sous-agent** : définir les hooks qui s'exécutent uniquement pendant que ce sous-agent spécifique est actif
* **Dans `settings.json`** : définir les hooks au niveau de la session qui se déclenchent également à l'intérieur des sous-agents. Les événements d'outils tels que `PreToolUse` et `PostToolUse` se déclenchent pour les appels d'outils du sous-agent de la même manière qu'ils le font dans la conversation principale, et `SubagentStart` et `SubagentStop` se déclenchent lorsqu'un sous-agent démarre ou se termine

Les hooks des [fichiers de paramètres, des paramètres de politique gérée et des plugins](/docs/fr/hooks#hook-locations) s'appliquent tous à l'intérieur des sous-agents, donc un hook `PreToolUse` dans `settings.json` s'exécute également avant chaque outil qu'un sous-agent utilise.

<h4 id="hooks-in-subagent-frontmatter">
  Hooks dans le frontmatter du sous-agent
</h4>

Définissez les hooks directement dans le fichier markdown du sous-agent. Ces hooks s'exécutent uniquement pendant que ce sous-agent spécifique est actif et sont nettoyés à la fin.

<Note>
  Les hooks de frontmatter se déclenchent lorsque l'agent est généré en tant que sous-agent via l'outil Agent ou une @-mention, et lorsque l'agent s'exécute en tant que session principale via [`--agent`](#invoke-subagents-explicitly) ou le paramètre `agent`. Dans le cas de la session principale, ils s'exécutent aux côtés de tous les hooks définis dans [`settings.json`](/docs/fr/hooks).
</Note>

Pour que les hooks de frontmatter d'un sous-agent au niveau du projet s'exécutent, acceptez la [boîte de dialogue de confiance de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) pour le dossier qui contient le fichier d'agent. Les hooks des sous-agents au niveau utilisateur dans `~/.claude/agents/` et des définitions que vous passez avec `--agents` s'exécutent sans cette étape. Si vous avez ajouté un dossier avec `--add-dir` en dehors du référentiel de l'espace de travail de confiance de votre organisation, faites confiance à ce dossier séparément : ses hooks `.claude/agents/` n'héritent pas de la confiance de votre espace de travail. Jusqu'à ce que vous fassiez confiance au dossier, le sous-agent s'exécute toujours, mais Claude Code ignore ses hooks de frontmatter et enregistre une erreur dans le journal de débogage expliquant comment faire confiance au dossier. C'est une règle plus stricte que celle pour les hooks dans les fichiers de paramètres : faire confiance à un dossier parent ne suffit pas, et une session `-p` ne compte pas comme de confiance. [What runs before you trust a folder](/docs/fr/permissions#what-runs-before-you-trust-a-folder) compare les deux. Avant la v2.1.218, les hooks de frontmatter pouvaient s'exécuter à partir de dossiers que vous n'aviez pas de confiance, y compris dans les sessions non interactives.

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

Les deux événements prennent en charge les matchers pour cibler des types d'agents spécifiques par nom. La valeur du matcher est le `name` du frontmatter de l'agent pour les sous-agents au niveau du projet et utilisateur, ou l'identifiant limité au plugin tel que `my-plugin:db-agent` pour les [sous-agents de plugin](/docs/fr/plugins/components#agents). Un nom limité contient un deux-points, il est donc évalué comme une [expression régulière non ancrée](/docs/fr/hooks#matcher-patterns) ; ancrez-le avec `^` et `$`, comme dans `^my-plugin:db-agent$`, pour correspondre uniquement à cet agent.

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

Gardez les descriptions brèves : Claude Code affiche un avertissement au démarrage lorsque les descriptions combinées de vos sous-agents dépassent [la limite de 15 000 tokens](/docs/fr/errors#agent-descriptions-are-over-the-15000-token-limit), et charge toujours chaque sous-agent.

Si le sous-agent est fourni dans un [plugin](/docs/fr/plugins/overview), vous pouvez mesurer la fiabilité avec laquelle Claude le délègue sur des invites réalistes au lieu de vérifier une par une : [`claude plugin eval`](/docs/fr/plugin-evals) exécute chaque invite avec et sans le plugin et évalue les résultats.

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

Les sous-agents fournis par un [plugin](/docs/fr/plugins/overview) activé apparaissent dans la saisie semi-automatique sous leur nom délimité, comme `my-plugin:code-reviewer` ou `my-plugin:review:security` lorsque le plugin [organise les agents dans des sous-dossiers](#choose-the-subagent-scope). Les sous-agents d'arrière-plan nommés actuellement en cours d'exécution dans la session apparaissent également dans la saisie semi-automatique, affichant leur statut à côté du nom.

Vous pouvez également taper la mention manuellement sans utiliser le sélecteur : `@agent-<name>` pour les sous-agents locaux, ou `@agent-` suivi du nom délimité pour les sous-agents de plugin, par exemple `@agent-my-plugin:code-reviewer`. Pendant que vous tapez cette forme, la saisie semi-automatique affiche les correspondances de fichiers plutôt que les agents. La mention d'agent se résout toujours lorsque vous soumettez.

**Exécutez la session entière en tant que sous-agent.** Passez [`--agent <name>`](/docs/fr/cli-reference) pour démarrer une session où le thread principal lui-même prend l'invite système, les restrictions d'outils et le modèle de ce sous-agent :

```bash theme={null}
claude --agent code-reviewer
```

L'invite système du sous-agent remplace complètement l'invite système par défaut de Claude Code, de la même manière que [`--system-prompt`](/docs/fr/cli-reference) le fait. Les fichiers `CLAUDE.md` et la mémoire du projet se chargent toujours via le flux de messages normal, même lorsque la définition de l'agent définit [`omitClaudeMd`](#supported-frontmatter-fields).

Le nom de l'agent apparaît comme `@<name>` dans l'en-tête de démarrage pour que vous puissiez confirmer qu'il est actif.

Cela fonctionne avec les sous-agents intégrés et personnalisés, et le choix persiste lorsque vous reprenez la session : Claude Code restaure les restrictions d'outils et le modèle de l'agent ainsi que la conversation. Si l'agent n'existe plus lorsque vous reprenez, la session continue avec les outils par défaut et affiche un [avertissement nommant l'agent](/docs/fr/errors#session-agent-no-longer-available). Pour l'invite système dans l'un ou l'autre cas, voir [Drapeaux d'invite système dans les conversations reprises](/docs/fr/cli-reference#system-prompt-flags-in-resumed-conversations).

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
* **Les sous-agents en arrière-plan** s'exécutent simultanément pendant que vous continuez à travailler. Lorsqu'un sous-agent en arrière-plan atteint un appel d'outil qui nécessite une permission, Claude Code affiche l'invite dans votre session principale et nomme le sous-agent qui demande. Approuvez pour laisser le sous-agent continuer, ou appuyez sur Échap pour refuser cet appel d'outil sans arrêter le sous-agent.

Pour chaque sous-agent que Claude génère avec l'outil Agent, Claude Code choisit le premier plan ou l'arrière-plan parmi les premiers cas qui s'appliquent :

* Si un coéquipier d'une [équipe d'agents](/docs/fr/agent-teams#limitations) en cours de traitement a généré le sous-agent, Claude Code l'exécute au premier plan. Claude Code refuse avec une erreur de générer un sous-agent d'un coéquipier dont la définition définit [`background: true`](#supported-frontmatter-fields). Lorsque le [mode fork](#turn-fork-mode-on-or-off) est désactivé et que vous n'avez pas [désactivé les tâches en arrière-plan](/docs/fr/env-vars), Claude Code refuse également avec une erreur lorsqu'un coéquipier définit `run_in_background: true`.
* Si vous définissez [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/fr/env-vars) sur `1`, Claude Code exécute le sous-agent au premier plan, dans tous les types de session et que le mode fork soit activé ou non.
* Lorsque le [mode fork](#turn-fork-mode-on-or-off) est activé, comme c'est le cas par défaut dans une session interactive, Claude Code exécute le sous-agent en arrière-plan, les sous-agents fork et non-fork, et Claude ne peut pas demander le premier plan.
* Lorsque le mode fork est désactivé, Claude exécute le sous-agent en arrière-plan par défaut et au premier plan lorsqu'il a besoin du résultat avant de continuer. Le mode fork est désactivé en [mode non-interactif](/docs/fr/headless) avec `-p` et dans le SDK Agent sauf si vous l'activez. Pour garder un sous-agent particulier en arrière-plan même lorsque Claude veut le résultat, définissez son champ frontmatter [`background`](#supported-frontmatter-fields) sur `true`.

Pour une skill avec `context: fork`, Claude Code suit les règles dans [Exécuter les skills dans un sous-agent](/docs/fr/skills#run-skills-in-a-subagent) à la place, que le mode fork soit activé ou non.

Les sous-agents en arrière-plan s'exécutent avec un [ensemble d'outils intégrés plus petit](#available-tools) que les sous-agents au premier plan, sauf pour les forks de conversation et les [sous-agents au premier plan repris](#resume-subagents).

Les sous-agents en arrière-plan affichent chaque invite de permission dans votre session principale. Lorsque vous répondez à l'une de ces invites avec un choix qui dure au-delà de cet appel d'outil, comme une autorisation qui dure pour le reste de la session, Claude Code applique votre réponse à la session entière, y compris votre conversation principale.

Un sous-agent en arrière-plan peut laisser une [commande Bash ou PowerShell](/docs/fr/tools-reference#background-commands) en arrière-plan [s'exécuter au-delà de la fin de son tour](/docs/fr/interactive-mode#how-backgrounding-works). Lorsque cette commande se termine, Claude Code envoie au sous-agent une notification.

Les résultats d'un sous-agent en arrière-plan atteignent Claude sous la forme d'une notification d'achèvement dans un tour ultérieur. Claude attend cette notification avant de signaler les résultats du sous-agent, et si vous demandez d'abord des informations sur la progression, il signale que le sous-agent s'exécute toujours. Avant la v2.1.211, Claude signalait parfois les résultats d'un sous-agent en arrière-plan qui n'avait pas terminé.

Vous pouvez également diriger cela vous-même :

* Lorsque le mode fork est désactivé, demandez à Claude d'exécuter une tâche en arrière-plan ou au premier plan
* Appuyez sur **Ctrl+B** pour mettre une tâche en arrière-plan

Claude Code efface la ligne d'un sous-agent en arrière-plan du panneau de sous-agent sous l'entrée d'invite de deux façons, selon la façon dont le sous-agent s'est terminé :

* Lorsqu'un sous-agent se termine avec succès, Claude Code supprime sa ligne immédiatement et, sauf en [mode lecteur d'écran](/docs/fr/accessibility), affiche `/tasks to see subagents` dans le pied de page pendant 30 secondes. Pendant ces 30 secondes, exécutez [`/tasks`](/docs/fr/commands) et appuyez sur `Entrée` sur le sous-agent pour ouvrir sa transcription. Avant la v2.1.232, Claude Code gardait la ligne pendant 30 secondes après la fin du sous-agent, comme un échoué, et n'affichait aucun indice de pied de page.
* Lorsqu'un sous-agent échoue ou que vous l'arrêtez, Claude Code garde sa ligne pendant 30 secondes. Pour effacer la ligne plus tôt, sélectionnez-la et appuyez sur `x`.

Un sous-agent en arrière-plan qui se termine reste listé dans [`/tasks`](/docs/fr/commands), marqué comme terminé et trié sous le travail en cours, pour la même fenêtre que l'indice de pied de page ci-dessus. Sa vue détaillée reste ouverte lorsque le sous-agent se termine. Les sous-agents qui échouent ou que vous arrêtez quittent la liste. Avant la v2.1.208, un sous-agent terminé quittait la liste dès qu'il se terminait et sa vue détaillée se fermait.

<h3 id="subagent-names">
  Noms des sous-agents
</h3>

Claude peut donner un nom à un sous-agent en passant un paramètre `name` sur l'appel de l'outil Agent, et peut le faire de sa propre initiative, sans vous demander d'abord. Le nom rend le sous-agent adressable : Claude peut [lui envoyer un message ou le reprendre par nom](#resume-subagents) après sa fin.

Dans une session interactive avec les [équipes d'agents](/docs/fr/agent-teams) activées, un sous-agent que Claude génère à partir de la conversation principale avec un `name` se lance en tant que coéquipier à la place, sauf si l'appel est un [fork](#fork-the-current-conversation) ou passe `isolation` sur l'appel lui-même. Une valeur `isolation` dans le frontmatter du sous-agent ne l'empêche pas, et le coéquipier s'exécute alors dans le répertoire de travail de la session principale. Voir [Comment Claude démarre les équipes d'agents](/docs/fr/agent-teams#how-claude-starts-agent-teams).

<h3 id="api-errors-in-subagents">
  Erreurs API dans les sous-agents
</h3>

Lorsque quelque chose [coupe la réponse d'un sous-agent en cours de flux](/docs/fr/errors#the-response-above-may-be-incomplete), et que la réponse partielle contient du texte mais pas d'appels d'outils, Claude Code invite le sous-agent à continuer plutôt que de terminer l'exécution. Cela se produit également dans les sessions interactives. L'exécution se termine sur l'erreur uniquement une fois que ces continuations sont épuisées.

À partir de la v2.1.199, un sous-agent dont l'exécution se termine sur une erreur API, comme une limite d'utilisation ou une erreur serveur répétée, signale cet échec à Claude au lieu de retourner le texte d'erreur comme s'il s'agissait des résultats du sous-agent. Ce que Claude reçoit dépend de l'endroit où le sous-agent s'est exécuté :

* **Premier plan** : si une limite de débit, une surcharge ou une erreur serveur coupe un sous-agent qui a déjà produit une sortie, l'outil Agent retourne cette sortie partielle avec une note indiquant que le sous-agent a été coupé et n'a pas terminé sa tâche. Un sous-agent qui n'a rien produit, ou dont la seule sortie était des appels d'outils, échoue avec [`Agent terminated early due to an API error`](/docs/fr/errors#agent-terminated-early-due-to-an-api-error), suivi du détail de l'erreur. Dans la v2.1.199, une limite de débit, une surcharge ou une erreur serveur qui a coupé la forme appels-d'outils-uniquement a retourné un résultat partiel vide contenant uniquement la note de coupure à la place.
* **Arrière-plan** : le sous-agent est marqué comme échoué, et le message que Claude reçoit lorsqu'il se termine nomme l'erreur API et inclut la dernière sortie du sous-agent, de sorte que le travail partiel n'est pas perdu.

Lorsque vous configurez une [chaîne de modèles de secours](/docs/fr/model-config#fallback-model-chains) et qu'un sous-agent rencontre une défaillance que la chaîne couvre, comme l'indisponibilité de son modèle, Claude Code bascule le sous-agent vers le premier modèle de la chaîne qui accepte la demande. Le sous-agent continue à travailler au lieu de se terminer sur l'erreur.

Une fois que l'erreur API sous-jacente est résolue, demandez à Claude de réessayer la tâche ou de [reprendre le sous-agent](#resume-subagents).

<h3 id="subagent-output-scanning">
  Analyse de la sortie du sous-agent
</h3>

Claude Code analyse le rapport final de chaque sous-agent avant que Claude ne le lise. Un sous-agent peut avoir lu des fichiers, des pages web ou une sortie de commande que vous n'avez jamais examinés, et le texte de ces sources peut contenir des instructions destinées à la conversation principale. L'analyse ne supprime ni ne reformule rien ; elle apporte deux types de changements que vous pouvez remarquer dans un rapport :

* **Insertion de barre oblique inverse** : l'analyse insère une barre oblique inverse dans le texte qui imite la sortie propre de Claude Code, comme une balise `<system-reminder>` ou une ligne commençant par `Human:` ou `Assistant:`, de sorte que l'imitation se lit comme du texte ordinaire au lieu d'être prise pour une partie de la conversation.
* **Ligne de marqueur** : l'analyse ajoute une ligne commençant par `[harness: subagent output matched instruction-shaped pattern(s):` lorsque le rapport imite une balise comme `<system-reminder>` ou mentionne des paramètres de permission comme `bypassPermissions` ou `--dangerously-skip-permissions`. Les mentions de paramètres de permission obtiennent la ligne de marqueur, mais le texte lui-même reste tel qu'écrit.

L'analyse ne juge pas si le contenu est malveillant, et elle ne change pas ce qu'une instruction dans un rapport peut faire : un appel d'outil que le rapport amène Claude à faire passe toujours par les [vérifications de permission](/docs/fr/permissions) et le [sandboxing](/docs/fr/sandboxing) de la session. Ce n'est pas un substitut à [restreindre ce qu'un sous-agent peut atteindre](#control-subagent-capabilities).

Un rapport qui revient à Claude en tant que résultat du sous-agent arrive également sous un en-tête le marquant comme sortie de sous-agent. L'en-tête indique que les instructions ou les affirmations d'approbation à l'intérieur du rapport sont les paroles du sous-agent et ne portent aucune autorité de votre part.

Un [rapport de sous-agent en arrière-plan](#run-subagents-in-foreground-or-background) arrive à l'intérieur d'une notification d'achèvement, qui est marquée comme un événement automatisé plutôt qu'un message de votre part.

<Note>
  L'analyse de la sortie du sous-agent nécessite Claude Code v2.1.210 ou ultérieur.
</Note>

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

Pour les tâches qui nécessitent un parallélisme soutenu ou qui dépassent une fenêtre de contexte, exécutez-les dans [des sessions séparées](/docs/fr/agents) et laissez Claude [transmettre les résultats entre elles](/docs/fr/cross-session-messaging).

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
* La latence est importante. Un sous-agent qui n'est pas un [fork](#fork-the-current-conversation) commence à zéro et peut avoir besoin de temps pour rassembler le contexte

Utilisez les **sous-agents** quand :

* La tâche produit une sortie détaillée dont vous n'avez pas besoin dans votre contexte principal
* Vous souhaitez appliquer des restrictions d'outils ou des permissions spécifiques
* Le travail est autonome et peut retourner un résumé

Envisagez plutôt les [Skills](/docs/fr/skills) lorsque vous souhaitez des invites ou des workflows réutilisables qui s'exécutent dans le contexte de la conversation principale plutôt que dans un contexte de sous-agent isolé.

Pour une question rapide sur quelque chose déjà dans votre conversation, utilisez [`/btw`](/docs/fr/interactive-mode#side-questions-with-%2Fbtw) au lieu d'un sous-agent. Il voit votre contexte complet mais n'a pas d'accès aux outils, et la réponse n'est pas ajoutée à l'historique.

<h3 id="let-subagents-spawn-their-own-subagents">
  Laisser les sous-agents générer leurs propres sous-agents
</h3>

Par défaut, un sous-agent peut générer ses propres sous-agents, jusqu'à trois niveaux en dessous de la conversation principale. À la limite de profondeur, Claude Code retient l'outil `Agent` de chaque sous-agent sauf un [fork](#fork-the-current-conversation), de sorte qu'un sous-agent à la limite fait son travail délégué lui-même et retourne un résumé. Un fork à la limite garde `Agent` dans sa liste d'outils héritée, mais l'outil retourne une erreur au lieu de générer.

Les sous-agents imbriqués conviennent à une tâche déléguée qui se divise elle-même en sous-tâches parallèles, comme un sous-agent examinateur qui envoie un vérificateur par résultat. Dans une session interactive, seul le résumé du sous-agent de niveau supérieur vous revient et la sortie intermédiaire reste en dehors de votre conversation principale : un sous-agent qui lance des sous-agents en arrière-plan attend leurs résultats avant de se terminer. En [mode non-interactif](/docs/fr/headless) et dans le SDK Agent, le sous-agent de lancement n'attend pas, de sorte qu'un sous-agent en arrière-plan imbriqué qui se termine après la fin de son lanceur signale à votre conversation principale à la place.

Pour modifier la limite, définissez [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/fr/env-vars) au nombre de niveaux de sous-agent que vous souhaitez en dessous de votre conversation principale. Par exemple, cette entrée dans [`settings.json`](/docs/fr/settings) limite l'imbrication à deux niveaux :

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

Avec cette valeur, vos sous-agents peuvent déléguer à une deuxième couche de leurs propres, et cette deuxième couche ne peut pas déléguer davantage. Définissez `1` pour désactiver l'imbrication.

Un sous-agent imbriqué est configuré de la même manière qu'un sous-agent de niveau supérieur et se résout à partir des mêmes [portées](#choose-the-subagent-scope). Pour empêcher un sous-agent de générer tandis que l'imbrication est activée, comme un examinateur qui doit rester en lecture seule, omettez `Agent` de sa liste [`tools`](#available-tools) ou ajoutez-le à `disallowedTools`.

Claude Code affiche les sous-agents imbriqués sous forme d'arborescence dans le panneau de sous-agent sous l'entrée d'invite et marque chaque ligne qui a encore des descendants dans le panneau avec un nombre `(+N)` d'entre eux. Ouvrez une ligne pour voir les frères et sœurs de ce sous-agent et les enfants directs avec un chemin de retour à `main`.

<Note>
  Les versions antérieures utilisaient des valeurs par défaut différentes :

  * **v2.1.172 à v2.1.216** : les sous-agents pouvaient s'imbriquer par défaut, jusqu'à cinq niveaux de profondeur, et la limite ne pouvait pas être modifiée.
  * **v2.1.217 à v2.1.218** : la limite était par défaut un, donc un sous-agent ne pouvait pas générer le sien à moins que vous ne l'augmentiez ; v2.1.219 a augmenté la valeur par défaut à trois.
</Note>

<h3 id="concurrent-subagent-limit">
  Limite de sous-agent concurrent
</h3>

Deux limites contrôlent l'utilisation des sous-agents, chacune avec sa propre variable : celle-ci empêche Claude de générer plus de sous-agents pendant que trop d'entre eux s'exécutent, et la [limite de profondeur](#let-subagents-spawn-their-own-subagents) limite la profondeur d'imbrication des sous-agents. Il n'y a pas de limite sur le nombre total de sous-agents que Claude peut générer au cours d'une session.

Par défaut, lorsque 20 sous-agents s'exécutent dans une session, la génération d'un autre avec l'outil Agent échoue avec `Concurrent subagent limit reached`, et l'erreur indique à Claude de ne pas réessayer. La génération réussit à nouveau lorsque le nombre en cours d'exécution tombe en dessous de la limite. Pour modifier la limite, définissez [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/fr/env-vars) sur n'importe quel nombre entier positif. Les sessions avec [ultracode](/docs/fr/model-config#adjust-effort-level) actif sont exemptées : la limite n'est pas appliquée là. Nécessite Claude Code v2.1.217 ou ultérieur.

La limite bloque uniquement les sous-agents que Claude génère avec l'outil Agent, mais d'autres exécutions occupent les mêmes emplacements :

* Un fork en session que vous démarrez avec [`/subtask`](#fork-the-current-conversation) prend un emplacement pendant qu'il s'exécute et n'est jamais bloqué par la limite.
* [Reprendre un sous-agent](#resume-subagents) qui a déjà terminé prend un nouvel emplacement sans vérifier la limite, de sorte que les reprises peuvent pousser le nombre en cours d'exécution au-delà.

Les agents que d'autres fonctionnalités exécutent, comme les agents [workflow](/docs/fr/workflows) et les coéquipiers d'une [équipe d'agents](/docs/fr/agent-teams), suivent leurs propres limites à la place.

<h3 id="manage-subagent-context">
  Gérer le contexte du sous-agent
</h3>

<h4 id="what-loads-at-startup">
  Ce qui se charge au démarrage
</h4>

Chaque sous-agent démarre avec une fenêtre de contexte fraîche et isolée. Il ne voit pas votre historique de conversation, les skills que vous avez déjà invoqués, ou les fichiers que Claude a déjà lus. Claude compose un message de délégation qui résume la tâche, et le sous-agent travaille à partir de là. L'exception est un [fork](#fork-the-current-conversation), qui hérite de la conversation parent au lieu de commencer à zéro.

Le contexte initial d'un sous-agent non-fork contient :

* **Invite système** : l'invite propre de l'agent plus les détails d'environnement que Claude Code ajoute, pas l'invite système de Claude Code. Les sous-agents personnalisés définissent la leur dans le [corps markdown](#write-subagent-files) ou le champ `prompt`. Les agents intégrés ont des invites prédéfinies.
* **Message de tâche** : l'invite de délégation que Claude écrit lorsqu'il confie le travail.
* **Fichiers CLAUDE.md** : chaque niveau de la [hiérarchie CLAUDE.md](/docs/fr/memory#how-claude-md-files-load) que la conversation principale charge, y compris `~/.claude/CLAUDE.md`, les règles du projet, `CLAUDE.local.md`, les fichiers de politique gérés et tous les [fichiers `AGENTS.md`](/docs/fr/memory#agents-md) chargés en tant qu'instructions du projet. Les agents Explore et Plan intégrés ignorent cela. Un sous-agent dont la définition définit [`omitClaudeMd`](#supported-frontmatter-fields) charge uniquement les fichiers de politique gérés, ou aucun du tout lorsque la définition provient des [paramètres gérés](#choose-the-subagent-scope).
* **Statut Git** : un instantané que Claude Code lit à partir de votre référentiel lorsque le sous-agent démarre. Absent en dehors d'un référentiel Git ou chaque fois que l'instantané est désactivé ; voir [`includeGitInstructions`](/docs/fr/settings-reference#includegitinstructions). Explore et Plan l'ignorent de toute façon.
* **Skills préchargés** : contenu complet de tout skill nommé dans le champ [`skills`](#preload-skills-into-subagents) de l'agent. Les agents intégrés ne préchargent pas les skills.
* **Roster des frères et sœurs** : un rappel système listant `main` et tous les autres agents nommés dans la session, chacun étant une valeur `to` valide pour [`SendMessage`](#resume-subagents). Nécessite Claude Code v2.1.206 ou ultérieur. Le roster n'apparaît que lorsque les outils du sous-agent incluent `SendMessage` et qu'au moins un autre agent a un nom, que Claude l'ait nommé lors de sa génération ou qu'il s'exécute en tant que coéquipier d'une [équipe d'agents](/docs/fr/agent-teams). C'est un instantané pris lorsque le sous-agent démarre, donc les agents nommés ultérieurement n'apparaissent pas.

Pour lancer l'un de vos propres sous-agents sans les fichiers CLAUDE.md de l'utilisateur, du projet et locaux, définissez [`omitClaudeMd: true`](#supported-frontmatter-fields) dans son frontmatter ou `--agents` JSON.

La conversation principale a toujours votre CLAUDE.md complet lorsqu'elle lit les résultats de ces sous-agents, de sorte que la plupart des règles n'ont pas besoin d'atteindre le sous-agent lui-même. Si une règle doit le faire, comme « ignorer le répertoire `vendor/` », reformulez-la dans l'invite que vous donnez à Claude lors de la délégation.

Vous ne pouvez pas modifier les sous-agents qui reçoivent le statut git. Seuls Explore et Plan l'ignorent.

Certains états de la conversation principale n'atteignent jamais un sous-agent non-fork :

* **Style de sortie** : un sous-agent exécute sa propre invite système, de sorte que votre [style de sortie](/docs/fr/output-styles) ne façonne pas ses réponses, sauf dans un [fork](#fork-the-current-conversation).
* **Mémoire automatique** : la [mémoire automatique](/docs/fr/memory#auto-memory) de la conversation principale n'est pas chargée. Pour donner à un sous-agent une mémoire persistante de son propre, utilisez le champ [`memory`](#enable-persistent-memory).
* **Taille de la fenêtre de contexte** : la fenêtre de contexte d'un sous-agent est dimensionnée par son propre modèle, pas celui du parent. Déléguer à un modèle avec une fenêtre plus petite donne à ce sous-agent la fenêtre plus petite.

<h4 id="resume-subagents">
  Reprendre les sous-agents
</h4>

Chaque invocation de sous-agent crée une nouvelle instance plutôt que de continuer une instance antérieure. Pour continuer le travail d'un sous-agent existant au lieu de recommencer, demandez à Claude de le reprendre.

Les sous-agents repris conservent leur historique de conversation complet, y compris tous les appels d'outils précédents, les résultats et le raisonnement. Si le sous-agent a généré [des sous-agents en arrière-plan de son propre](#let-subagents-spawn-their-own-subagents), cet historique inclut les résultats qu'ils ont livrés pendant qu'il s'exécutait. Le sous-agent reprend exactement où il s'était arrêté plutôt que de recommencer à zéro.

* Lorsqu'un sous-agent se termine, Claude reçoit son ID d'agent.
* Les agents Explore et Plan intégrés sont ponctuels et ne retournent pas d'ID d'agent, donc Claude ne peut pas les reprendre. Utilisez `general-purpose` ou un sous-agent personnalisé lorsque vous avez besoin de continuer le travail.
* Lorsqu'un sous-agent s'arrête à sa limite [`maxTurns`](#supported-frontmatter-fields), Claude Code marque la sortie retournée comme partielle. Pour les sous-agents qui retournent un ID d'agent, Claude Code note également dans le résultat que Claude peut envoyer un message au sous-agent pour continuer à partir de là où il s'est arrêté.

Claude utilise l'outil `SendMessage` avec l'ID de l'agent ou le nom comme champ `to` pour le reprendre. `SendMessage` ne nécessite pas que les [équipes d'agents](/docs/fr/agent-teams) soient activées ; seuls les messages de protocole d'équipe structurés tels que `shutdown_request` et `plan_approval_response` le font. Au-delà des sous-agents et des coéquipiers, dans les sessions où la messagerie inter-sessions est activée, Claude peut utiliser le même outil pour envoyer un message à [vos autres sessions Claude Code](/docs/fr/cross-session-messaging), sur cette machine ou [au-delà](/docs/fr/cross-session-messaging#message-sessions-on-other-machines).

Pour reprendre un sous-agent, demandez à Claude de continuer le travail précédent :

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Lorsqu'un sous-agent terminé reçoit un message avec l'outil `SendMessage`, le sous-agent se reprend automatiquement en arrière-plan sans une nouvelle invocation `Agent`. Le même principe s'applique à un sous-agent que Claude a arrêté avec l'outil `TaskStop`, une fois que sa exécution arrêtée a quitté. L'exécution reprise conserve l'[ensemble d'outils d'où le sous-agent s'est d'abord exécuté](#run-subagents-in-foreground-or-background) et peut continuer à lire le [cache d'invite que l'exécution originale a préchauffé](/docs/fr/prompt-caching#subagents-and-the-cache).

Un sous-agent qui a l'outil `SendMessage` peut aussi envoyer ce message. Dans une session interactive, l'agent repris signale alors au sous-agent qui l'a repris, pas à votre conversation principale. Ce sous-agent attend le résultat avant de terminer son propre travail. Lorsqu'un sous-agent envoie un message à un agent auquel il signale, comme son propre lanceur, Claude Code reprend cet agent sans rediriger ses résultats.

Un sous-agent que vous avez arrêté vous-même, avec `x` dans `/tasks` ou une demande SDK `stop_task`, ne se reprend pas automatiquement. Si Claude lui envoie un message, le message est refusé et Claude est informé que l'agent a été annulé.

Pendant que [la ligne de ce sous-agent est toujours dans le panneau de sous-agent](#run-subagents-in-foreground-or-background), tapez dans sa transcription pour le reprendre vous-même. Après cela, un message de Claude peut le reprendre automatiquement à nouveau.

Reprendre démarre une nouvelle exécution de l'agent sous le même ID, de sorte qu'un sous-agent qui avait déjà échoué ou s'était terminé s'affiche à nouveau comme en cours d'exécution dans la liste des tâches et dans les événements de tâche du SDK Agent. Avant la v2.1.205, il continuait à afficher son statut antérieur échoué ou terminé pendant que l'exécution reprise fonctionnait.

À partir de la v2.1.199, `SendMessage` vérifie qu'un nom fait toujours référence au même agent qu'il a atteint plus tôt dans la conversation. Si un agent plus récent a pris le nom, comme un sous-agent en arrière-plan réengendré qui l'a réutilisé, Claude Code refuse l'envoi plutôt que de le livrer au mauvais agent, et l'erreur signale quel agent le nom atteint maintenant pour que Claude puisse le rediriger. Pour atteindre l'agent antérieur pendant qu'il s'exécute toujours, Claude l'adresse par l'ID d'agent qu'il a reçu lorsqu'il a généré cet agent. La vérification est limitée à la conversation actuelle et se réinitialise sur `/clear`.

À partir de la v2.1.198, un sous-agent traite les messages de l'agent qui l'a lancé comme une direction de tâche normale, y compris les corrections de cours en cours de tâche, et agit en fonction de ceux-ci dans ses propres paramètres de permission. Deux limites tiennent toujours indépendamment de qui a envoyé le message : aucun message d'aucun agent ne compte comme votre approbation pour une invite de permission en attente, et aucun message d'agent ne peut modifier les paramètres de permission d'un sous-agent, `CLAUDE.md`, ou la configuration. Seul le système de permission ou vos propres messages peuvent accorder l'approbation.

Vous pouvez également demander à Claude l'ID d'agent si vous souhaitez le référencer explicitement, ou trouver les ID dans les fichiers de transcription à `~/.claude/projects/{project}/{sessionId}/subagents/`. Chaque transcription est stockée sous la forme `agent-{agentId}.jsonl`.

Les transcriptions de sous-agent persistent indépendamment de la conversation principale :

* **Compaction de la conversation principale** : Lorsque la conversation principale se compacte, les transcriptions de sous-agent ne sont pas affectées. Elles sont stockées dans des fichiers séparés.
* **Persistance de session** : Les transcriptions de sous-agent persistent au sein de leur session. Vous pouvez [reprendre un sous-agent](#resume-subagents) après le redémarrage de Claude Code en reprenant la même session.
* **Nettoyage automatique** : Claude Code supprime les transcriptions de sous-agent après la période de rétention `cleanupPeriodDays`, 30 jours par défaut, en suivant les [règles de balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically).

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
  Exécutez un sous-agent dupliqué avec `/subtask`, qui nécessite Claude Code v2.1.212 ou version ultérieure. Lorsque la [vue agent est désactivée](/docs/fr/agent-view#turn-off-agent-view), `/subtask` n'est pas disponible et `/fork` démarre le sous-agent dupliqué à la place ; sinon `/fork` copie l'intégralité de la session dans une nouvelle [session en arrière-plan](/docs/fr/agent-view#from-inside-a-session).
</Note>

Un fork est un sous-agent qui hérite de l'intégralité de la conversation jusqu'à présent au lieu de commencer à zéro. Cela supprime l'isolation d'entrée que les sous-agents fournissent autrement : un fork voit la même invite système, les mêmes outils, le même modèle et l'historique des messages que la session principale, vous pouvez donc lui confier une tâche secondaire sans réexpliquer la situation. Les appels d'outils du fork restent en dehors de votre conversation et seul son résultat final revient, donc votre fenêtre de contexte principal reste propre. Utilisez un fork lorsqu'un autre sous-agent aurait besoin de trop de contexte pour être utile, ou lorsque vous souhaitez essayer plusieurs approches en parallèle à partir du même point de départ.

Claude démarre un fork en demandant le type de sous-agent `fork` via l'outil Agent. Vous contrôlez s'il peut le faire avec le [mode fork](#turn-fork-mode-on-or-off), qui est activé par défaut dans les sessions interactives.

Vous pouvez démarrer un fork vous-même avec `/subtask` suivi d'une tâche, que le mode fork soit activé ou non. Sur v2.1.161 à v2.1.211, la commande est `/fork`. Claude Code nomme le fork à partir des premiers mots de la tâche. L'exemple suivant duplique la conversation pour rédiger des cas de test pendant que vous continuez avec l'implémentation dans la session principale :

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

Le fork apparaît dans un panneau sous votre invite et s'exécute en arrière-plan pendant que vous continuez à travailler. Lorsqu'il se termine, son résultat arrive sous forme de message dans votre conversation principale. La section suivante couvre les contrôles du panneau pour observer et diriger les forks pendant qu'ils s'exécutent.

<h3 id="observe-and-steer-running-forks">
  Observer et diriger les forks en cours d'exécution
</h3>

Les forks en cours d'exécution apparaissent dans un panneau sous l'entrée d'invite, avec une ligne pour la session principale et une pour chaque fork.

Lorsqu'un fork se termine avec succès, Claude Code supprime sa ligne. Claude Code conserve la ligne d'un fork qui a échoué ou que vous avez arrêté pendant 30 secondes, [identique à tout autre sous-agent en arrière-plan](#run-subagents-in-foreground-or-background). Avant v2.1.232, Claude Code conservait également la ligne d'un fork terminé pendant 30 secondes.

Utilisez ces touches pour interagir avec le panneau :

| Touche    | Action                                                                                                                                                                                                                                                                      |
| :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓` | Se déplacer entre les lignes                                                                                                                                                                                                                                                |
| `Entrée`  | Ouvrir la transcription du fork sélectionné et lui envoyer des messages de suivi                                                                                                                                                                                            |
| `x`       | Arrêter le fork sélectionné s'il est en cours d'exécution, ou ignorer sa ligne s'il n'est plus en cours d'exécution. Sur la ligne de la session principale, ou sur la ligne du fork dont vous avez ouvert la transcription avec `Entrée`, `x` tape dans l'invite à la place |
| `Échap`   | Retourner le focus à l'entrée d'invite                                                                                                                                                                                                                                      |

Avec la transcription d'un fork ou d'un sous-agent ouverte, les messages de suivi et les [skills](/docs/fr/skills) vont à cet agent, mais les commandes intégrées s'exécutent toujours dans votre conversation principale. À partir de v2.1.199, taper `/model` ou `/fast` dans cette vue affiche un avis indiquant que cela change le modèle ou le mode rapide de la conversation principale, et non celui de l'agent visualisé, au lieu de l'exécuter silencieusement.

<h3 id="how-forks-differ-from-other-subagents">
  Comment les forks diffèrent des autres sous-agents
</h3>

Un fork hérite de tout ce que la session principale a au moment où il se génère. Tout autre sous-agent démarre à partir de sa définition.

|                          | Fork                                        | Sous-agent non-fork                                                                                                                      |
| :----------------------- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------- |
| Contexte                 | Historique de conversation complet          | Contexte frais avec l'invite que vous transmettez                                                                                        |
| Invite système et outils | Identique à la session principale           | À partir du [fichier de définition](#write-subagent-files) du sous-agent, [filtré pour les exécutions en arrière-plan](#available-tools) |
| Modèle                   | Identique à la session principale           | À partir du champ `model` du sous-agent                                                                                                  |
| Permissions              | Les invites s'affichent dans votre terminal | [Les invites s'affichent dans votre session principale](#run-subagents-in-foreground-or-background) lors de l'exécution en arrière-plan  |
| Cache d'invite           | Partagé avec la session principale          | Cache séparé                                                                                                                             |

Parce que l'invite système d'un fork et les définitions d'outils sont identiques au parent, sa première demande réutilise le [cache d'invite](/docs/fr/prompt-caching#subagents-and-the-cache) du parent. Cela rend le forking moins cher que la génération d'un sous-agent frais pour les tâches qui ont besoin du même contexte.

Lorsque Claude génère un fork via l'outil Agent, il peut passer `isolation: "worktree"` pour que les modifications de fichiers du fork soient écrites dans un git worktree séparé au lieu de votre extraction. Un fork ne peut pas générer d'autres forks.

<h3 id="turn-fork-mode-on-or-off">
  Activer ou désactiver le mode fork
</h3>

Claude Code active le mode fork par défaut dans les sessions interactives et le laisse désactivé par défaut en [mode non-interactif](/docs/fr/headless) avec `-p` et dans le SDK Agent. La valeur par défaut interactive nécessite Claude Code v2.1.232 ou version ultérieure. Sur les versions antérieures, définissez `CLAUDE_CODE_FORK_SUBAGENT` sur `1` pour activer le mode fork.

Vous pouvez dire que le mode fork est activé à partir de la façon dont Claude Code gère l'outil Agent :

* Claude peut générer un fork en demandant le type de sous-agent `fork`. Lorsque Claude ne demande pas de type, il obtient le sous-agent [general-purpose](#built-in-subagents), si la session a toujours ce type. Les sous-agents générés à partir d'une définition, tels que Explore, fonctionnent comme d'habitude.
* Claude Code exécute les sous-agents que Claude génère en arrière-plan, les forks et les sous-agents non-fork, à l'exception des [cas qui restent au premier plan](#run-subagents-in-foreground-or-background). Claude Code supprime également le paramètre `run_in_background` de l'outil Agent, donc Claude ne peut pas demander le premier plan.

Définissez la variable d'environnement [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/fr/env-vars) pour remplacer les valeurs par défaut :

* `1` active le mode fork en mode non-interactif et dans le SDK Agent également
* `0` désactive le mode fork dans tous les types de session

Pour garder le mode fork activé mais empêcher Claude de générer des forks, [refusez le type de sous-agent `fork`](#disable-specific-subagents) avec une règle `Agent(fork)`. Claude Code exécute toujours les sous-agents que Claude génère en arrière-plan, à l'exception des mêmes [cas qui restent au premier plan](#run-subagents-in-foreground-or-background).

<h2 id="example-subagents">
  Exemples de sous-agents
</h2>

Ces exemples démontrent des modèles efficaces pour construire des sous-agents. Utilisez-les comme points de départ, ou générez une version personnalisée avec Claude.

<Tip>
  **Meilleures pratiques :**

  * **Concevoir des sous-agents ciblés :** chaque sous-agent doit exceller dans une tâche spécifique
  * **Écrire des descriptions qui mettent en avant un seul sous-agent :** Claude utilise la description pour décider quand déléguer. Rendez chaque description suffisamment spécifique pour router vers le bon sous-agent, et gardez l'ensemble combiné dans le [budget de description de 15 000 jetons](#understand-automatic-delegation)
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

L'invite système indique au sous-agent de refuser les demandes d'écriture, donc le hook est une sauvegarde : si le sous-agent tente une écriture de toute façon, Claude Code bloque la commande et le sous-agent voit le message `Blocked: Write operations not allowed. Use SELECT queries only.`.

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous comprenez les sous-agents, explorez ces fonctionnalités connexes :

* [Distribuer les sous-agents avec les plugins](/docs/fr/plugins/components#agents) pour partager les sous-agents entre les équipes ou les projets
* [Exécuter Claude Code par programmation](/docs/fr/headless) avec le SDK Agent pour CI/CD et l'automatisation
* [Utiliser les serveurs MCP](/docs/fr/mcp) pour donner aux sous-agents l'accès aux outils et données externes
