> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utiliser les fonctionnalités de Claude Code dans le SDK

> Chargez les instructions de projet, les compétences, les hooks et autres fonctionnalités de Claude Code dans vos agents SDK.

Le SDK Agent est construit sur la même base que Claude Code, ce qui signifie que vos agents SDK ont accès aux mêmes fonctionnalités basées sur le système de fichiers : instructions de projet (`CLAUDE.md` et règles), compétences, hooks, et bien plus.

Lorsque vous omettez `settingSources`, `query()` lit les mêmes paramètres du système de fichiers que l'interface de ligne de commande Claude Code : paramètres utilisateur, projet et locaux, fichiers `CLAUDE.md`, et compétences, agents et commandes `.claude/`. Pour fonctionner sans ces éléments, passez `settingSources: []`, ce qui limite l'agent à ce que vous configurez par programmation. Les paramètres de politique gérée et la configuration globale `~/.claude.json` sont lus indépendamment de cette option. Voir [Ce que settingSources ne contrôle pas](#what-settingsources-does-not-control).

Pour une vue d'ensemble conceptuelle de ce que chaque fonctionnalité fait et quand l'utiliser, voir [Étendre Claude Code](/docs/fr/features-overview).

<h2 id="control-filesystem-settings-with-settingsources">
  Contrôler les paramètres du système de fichiers avec settingSources
</h2>

L'option des sources de paramètres ([`setting_sources`](/docs/fr/agent-sdk/python#claudeagentoptions) en Python, [`settingSources`](/docs/fr/agent-sdk/typescript#settingsource) en TypeScript) contrôle quels paramètres basés sur le système de fichiers le SDK charge. Passez une liste explicite pour accepter des sources spécifiques, ou passez un tableau vide pour désactiver les paramètres utilisateur, projet et locaux.

Cet exemple charge à la fois les paramètres au niveau utilisateur et au niveau projet en définissant `settingSources` sur `["user", "project"]` :

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage

  async for message in query(
      prompt="Help me refactor the auth module",
      options=ClaudeAgentOptions(
          # "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
          # Together they give the agent access to CLAUDE.md, skills, hooks, and
          # permissions from both locations.
          setting_sources=["user", "project"],
          allowed_tools=["Read", "Edit", "Bash"],
      ),
  ):
      if isinstance(message, AssistantMessage):
          for block in message.content:
              if hasattr(block, "text"):
                  print(block.text)
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(f"\nResult: {message.result}")
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me refactor the auth module",
    options: {
      // "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
      // Together they give the agent access to CLAUDE.md, skills, hooks, and
      // permissions from both locations.
      settingSources: ["user", "project"],
      allowedTools: ["Read", "Edit", "Bash"]
    }
  })) {
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "text") console.log(block.text);
      }
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(`\nResult: ${message.result}`);
    }
  }
  ```
</CodeGroup>

Chaque source charge les paramètres à partir d'un emplacement spécifique, où `<cwd>` est le répertoire de travail que vous passez via l'option `cwd`, ou le répertoire courant du processus s'il n'est pas défini. Pour la définition de type complète, voir [`SettingSource`](/docs/fr/agent-sdk/typescript#settingsource) (TypeScript) ou [`SettingSource`](/docs/fr/agent-sdk/python#settingsource) (Python).

| Source      | Ce qu'elle charge                                                                                            | Emplacement                                                                                                                                                                                        |
| :---------- | :----------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"project"` | CLAUDE.md du projet, `.claude/rules/*.md`, compétences du projet, hooks du projet, `settings.json` du projet | `<cwd>/.claude/` pour `settings.json` et hooks ; `<cwd>` et chaque répertoire parent pour CLAUDE.md et rules ; `<cwd>` et chaque répertoire parent jusqu'à la racine du dépôt pour les compétences |
| `"user"`    | CLAUDE.md utilisateur, `~/.claude/rules/*.md`, compétences utilisateur, paramètres utilisateur               | `~/.claude/`                                                                                                                                                                                       |
| `"local"`   | CLAUDE.local.md, `.claude/settings.local.json`                                                               | `<cwd>/.claude/` pour `settings.local.json` ; `<cwd>` et chaque répertoire parent pour CLAUDE.local.md                                                                                             |

Omettre `settingSources` équivaut à `["user", "project", "local"]`.

L'option `cwd` détermine où le SDK recherche les entrées au niveau du projet. CLAUDE.md et les rules se chargent à partir de `<cwd>` et de chaque répertoire parent. Les compétences se chargent à partir de `<cwd>` et de chaque répertoire parent jusqu'à la racine du dépôt. Le `settings.json` du projet et les hooks se chargent uniquement à partir de `<cwd>/.claude/` sans secours au répertoire parent.

<h3 id="what-settingsources-does-not-control">
  Ce que settingSources ne contrôle pas
</h3>

`settingSources` couvre les paramètres utilisateur, projet et locaux. Quelques entrées sont lues indépendamment de sa valeur :

| Entrée                                                                 | Comportement                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Pour désactiver                                                                                                                                                                                                                                                    |
| :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Paramètres de politique gérée                                          | Politique gérée par le point de terminaison, qu'il s'agisse d'une plist MDM, d'une politique de registre ou de fichiers de paramètres gérés, se charge à partir de l'hôte. Les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) sont récupérés sur une [configuration éligible](/docs/fr/server-managed-settings#platform-availability) quand la session s'authentifie avec une connexion OAuth d'organisation ou une clé API directement configurée | Politique de point de terminaison : supprimez le fichier de paramètres gérés, la plist ou la politique de registre de l'hôte. Paramètres gérés par le serveur : contrôlés par votre administrateur d'organisation ; ne peuvent pas être désactivés à partir du SDK |
| Configuration globale `~/.claude.json`                                 | Toujours lue                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Relocalisez avec `CLAUDE_CONFIG_DIR` dans `env`                                                                                                                                                                                                                    |
| Mémoire automatique à `~/.claude/projects/<project>/memory/`           | Chargée dans l'invite système au démarrage de la session. L'agent écrit les nouvelles mémoires là avec les outils standard `Write` et `Edit` plutôt qu'avec un outil de mémoire dédié, donc ces outils doivent être activés pour que l'agent puisse enregistrer les mémoires                                                                                                                                                                                  | Définissez `autoMemoryEnabled: false` dans les paramètres, ou `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` dans `env`                                                                                                                                                       |
| [Connecteurs MCP de claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) | Chargés quand la méthode d'authentification active est un abonnement claude.ai. Passer `mcpServers: {}` ne les supprime pas                                                                                                                                                                                                                                                                                                                                   | Définissez `strictMcpConfig: true`, [`disableClaudeAiConnectors: true`](/docs/fr/mcp#disable-claude-ai-connectors) dans les paramètres, ou `ENABLE_CLAUDEAI_MCP_SERVERS=false` dans `env`                                                                               |

<Warning>
  Ne vous fiez pas aux options par défaut de `query()` pour l'isolation multi-locataire. Parce que les entrées ci-dessus sont lues indépendamment de `settingSources`, un processus SDK peut récupérer la configuration au niveau de l'hôte et la mémoire par répertoire. Pour les déploiements multi-locataires, exécutez chaque locataire dans son propre système de fichiers et définissez `settingSources: []` plus `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` dans `env`. Les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) sont récupérés quand le processus s'authentifie avec une credential d'organisation ; l'isolation du système de fichiers ne les supprime pas. Voir [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment).
</Warning>

<h2 id="project-instructions-claude-md-and-rules">
  Instructions de projet (CLAUDE.md et règles)
</h2>

Les fichiers `CLAUDE.md` et les fichiers `.claude/rules/*.md` donnent à votre agent un contexte persistant sur votre projet : conventions de codage, commandes de construction, décisions architecturales et instructions. Quand `settingSources` inclut `"project"` (comme dans l'exemple ci-dessus), le SDK charge ces fichiers en contexte au démarrage de la session. L'agent suit ensuite vos conventions de projet sans que vous ayez besoin de les répéter dans chaque invite.

<h3 id="claude-md-load-locations">
  Emplacements de chargement de CLAUDE.md
</h3>

| Niveau                       | Emplacement                                                                      | Quand chargé                                                                                              |
| :--------------------------- | :------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| Projet (racine)              | `<cwd>/CLAUDE.md` ou `<cwd>/.claude/CLAUDE.md`                                   | `settingSources` inclut `"project"`                                                                       |
| Règles du projet             | `<cwd>/.claude/rules/*.md` et `.claude/rules/*.md` dans chaque répertoire parent | `settingSources` inclut `"project"`                                                                       |
| Projet (répertoires parents) | Fichiers `CLAUDE.md` dans les répertoires au-dessus de `cwd`                     | `settingSources` inclut `"project"`, chargés au démarrage de la session                                   |
| Projet (répertoires enfants) | Fichiers `CLAUDE.md` dans les sous-répertoires de `cwd`                          | `settingSources` inclut `"project"`, chargés à la demande quand l'agent lit un fichier dans ce sous-arbre |
| Local                        | `<cwd>/CLAUDE.local.md` et `CLAUDE.local.md` dans chaque répertoire parent       | `settingSources` inclut `"local"`                                                                         |
| Utilisateur                  | `~/.claude/CLAUDE.md`                                                            | `settingSources` inclut `"user"`                                                                          |
| Règles utilisateur           | `~/.claude/rules/*.md`                                                           | `settingSources` inclut `"user"`                                                                          |

Tous les niveaux sont additifs : si les fichiers `CLAUDE.md` du projet et de l'utilisateur existent tous les deux, l'agent voit les deux. Il n'y a pas de règle de précédence stricte entre les niveaux ; si les instructions entrent en conflit, le résultat dépend de la façon dont Claude les interprète. Écrivez des règles sans conflit, ou énoncez explicitement la précédence dans le fichier plus spécifique (« Ces instructions de projet remplacent tout défaut au niveau utilisateur en conflit »).

<Tip>
  Vous pouvez également injecter du contexte directement via `systemPrompt` sans utiliser les fichiers `CLAUDE.md`. Voir [Modifier les invites système](/docs/fr/agent-sdk/modifying-system-prompts). Utilisez `CLAUDE.md` quand vous voulez que le même contexte soit partagé entre les sessions Claude Code interactives et vos agents SDK.
</Tip>

Pour savoir comment structurer et organiser le contenu de `CLAUDE.md`, voir [Gérer la mémoire de Claude](/docs/fr/memory).

<h2 id="skills">
  Compétences
</h2>

Les compétences sont des fichiers markdown qui donnent à votre agent des connaissances spécialisées et des flux de travail invocables. Contrairement à `CLAUDE.md` (qui se charge à chaque session), les compétences se chargent à la demande. L'agent reçoit les descriptions des compétences au démarrage et charge le contenu complet quand c'est pertinent.

Les compétences sont découvertes à partir du système de fichiers via `settingSources`. Quand l'option `skills` sur `query()` est omise, les compétences utilisateur et projet découvertes sont activées et l'outil Skill est disponible, ce qui correspond au comportement de la CLI. Pour contrôler quelles compétences sont activées, passez `skills` comme `"all"`, une liste de noms de compétences, ou `[]` pour désactiver toutes les compétences. Quand `skills` est défini, le SDK ajoute automatiquement l'outil Skill à `allowedTools`. Si vous passez également une liste `tools` explicite, incluez `"Skill"` dans cette liste pour que Claude puisse invoquer les compétences.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Skills in .claude/skills/ are discovered automatically
  # when settingSources includes "project"
  async for message in query(
      prompt="Review this PR using our code review checklist",
      options=ClaudeAgentOptions(
          setting_sources=["user", "project"],
          skills="all",
          allowed_tools=["Read", "Grep", "Glob"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Skills in .claude/skills/ are discovered automatically
  // when settingSources includes "project"
  for await (const message of query({
    prompt: "Review this PR using our code review checklist",
    options: {
      settingSources: ["user", "project"],
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<Note>
  Les compétences doivent être créées en tant qu'artefacts du système de fichiers (`.claude/skills/<name>/SKILL.md`). Le SDK n'a pas d'API programmatique pour enregistrer les compétences. Voir [Agent Skills dans le SDK](/docs/fr/agent-sdk/skills) pour les détails complets.
</Note>

Pour en savoir plus sur la création et l'utilisation des compétences, voir [Agent Skills dans le SDK](/docs/fr/agent-sdk/skills).

<h2 id="hooks">
  Hooks
</h2>

Le SDK supporte deux façons de définir les hooks, et ils s'exécutent côte à côte :

* **Hooks du système de fichiers :** commandes shell définies dans `settings.json`, chargées quand `settingSources` inclut la source pertinente. Ce sont les mêmes hooks que vous configureriez pour les [sessions Claude Code interactives](/docs/fr/hooks-guide).
* **Hooks programmatiques :** fonctions de rappel passées directement à `query()`. Celles-ci s'exécutent dans votre processus d'application et peuvent retourner des décisions structurées. Voir [Contrôler l'exécution avec les hooks](/docs/fr/agent-sdk/hooks).

Les deux types s'exécutent pendant le même cycle de vie des hooks. Si vous avez déjà des hooks dans le `.claude/settings.json` de votre projet et que vous définissez `settingSources: ["project"]`, ces hooks s'exécutent automatiquement dans le SDK sans configuration supplémentaire.

Les rappels de hook reçoivent l'entrée de l'outil et retournent un dictionnaire de décision. Retourner `{}` signifie permettre à l'outil de procéder. Pour bloquer l'exécution, retournez un objet `hookSpecificOutput` avec `permissionDecision: "deny"` et une `permissionDecisionReason`. La raison est envoyée à Claude comme résultat de l'outil. Les champs `decision` et `reason` au niveau supérieur sont dépréciés pour `PreToolUse`. Voir le [guide des hooks](/docs/fr/agent-sdk/hooks) pour la signature de rappel complète et les types de retour.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, ResultMessage


  # PreToolUse hook callback. Positional args:
  #   input_data: HookInput dict with tool_name, tool_input, hook_event_name
  #   tool_use_id: str | None, the ID of the tool call being intercepted
  #   context: HookContext, carries session metadata
  async def audit_bash(input_data, tool_use_id, context):
      command = input_data.get("tool_input", {}).get("command", "")
      if "rm -rf" in command:
          return {
              "hookSpecificOutput": {
                  "hookEventName": "PreToolUse",
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Destructive command blocked",
              }
          }
      return {}  # Empty dict: allow the tool to proceed


  # Filesystem hooks from .claude/settings.json run automatically
  # when settingSources loads them. You can also add programmatic hooks:
  async for message in query(
      prompt="Refactor the auth module",
      options=ClaudeAgentOptions(
          setting_sources=["project"],  # Loads hooks from .claude/settings.json
          hooks={
              "PreToolUse": [
                  HookMatcher(matcher="Bash", hooks=[audit_bash]),
              ]
          },
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query, type HookInput, type HookJSONOutput } from "@anthropic-ai/claude-agent-sdk";

  // PreToolUse hook callback. HookInput is a discriminated union on
  // hook_event_name, so narrowing on it gives TypeScript the right
  // tool_input shape for this event.
  const auditBash = async (input: HookInput): Promise<HookJSONOutput> => {
    if (input.hook_event_name !== "PreToolUse") return {};
    const toolInput = input.tool_input as { command?: string };
    if (toolInput.command?.includes("rm -rf")) {
      return {
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked",
        },
      };
    }
    return {}; // Empty object: allow the tool to proceed
  };

  // Filesystem hooks from .claude/settings.json run automatically
  // when settingSources loads them. You can also add programmatic hooks:
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      settingSources: ["project"], // Loads hooks from .claude/settings.json
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [auditBash] }]
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="when-to-use-which-hook-type">
  Quand utiliser quel type de hook
</h3>

| Type de hook                                | Meilleur pour                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Système de fichiers** (`settings.json`)   | Partager les hooks entre les sessions CLI et SDK. Supporte `"command"` (scripts shell), `"http"` (POST à un point de terminaison), `"mcp_tool"` (appeler l'outil d'un serveur MCP connecté), `"prompt"` (l'LLM évalue une invite), et `"agent"` (génère un agent vérificateur). Ceux-ci s'exécutent dans l'agent principal et tous les sous-agents qu'il génère. |
| **Programmatique** (rappels dans `query()`) | Logique spécifique à l'application, décisions structurées et intégration en processus. Ceux-ci s'exécutent également à l'intérieur des sous-agents. Le rappel reçoit `agent_id` et `agent_type` pour distinguer.                                                                                                                                                 |

<Note>
  Le SDK TypeScript supporte des événements de hook supplémentaires au-delà de Python, notamment `SessionStart`, `SessionEnd`, `TeammateIdle`, et `TaskCompleted`. Voir le [guide des hooks](/docs/fr/agent-sdk/hooks) pour le tableau complet de compatibilité des événements.
</Note>

Pour les détails complets sur les hooks programmatiques, voir [Contrôler l'exécution avec les hooks](/docs/fr/agent-sdk/hooks). Pour la syntaxe des hooks du système de fichiers, voir [Hooks](/docs/fr/hooks).

<h2 id="choose-the-right-feature">
  Choisir la bonne fonctionnalité
</h2>

Le SDK Agent vous donne accès à plusieurs façons d'étendre le comportement de votre agent. Si vous n'êtes pas sûr de celle à utiliser, ce tableau mappe les objectifs courants à la bonne approche.

| Vous voulez...                                                                                                          | Utiliser                                      | Surface SDK                                                                                                                                                                                           |
| :---------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Définir les conventions de projet que votre agent suit toujours                                                         | [CLAUDE.md](/docs/fr/memory)                       | `settingSources: ["project"]` le charge automatiquement                                                                                                                                               |
| Donner à l'agent du matériel de référence qu'il charge quand c'est pertinent                                            | [Skills](/docs/fr/agent-sdk/skills)                | `settingSources` + option `skills`                                                                                                                                                                    |
| Exécuter un flux de travail réutilisable (déployer, examiner, publier)                                                  | [User-invocable skills](/docs/fr/agent-sdk/skills) | `settingSources` + option `skills`                                                                                                                                                                    |
| Déléguer une sous-tâche isolée à un contexte frais (recherche, examen)                                                  | [Subagents](/docs/fr/agent-sdk/subagents)          | Paramètre `agents` + `allowedTools: ["Agent"]`                                                                                                                                                        |
| Coordonner plusieurs instances de Claude Code avec des listes de tâches partagées et la messagerie directe inter-agents | [Agent teams](/docs/fr/agent-teams)                | Non configuré directement via les options SDK. Les équipes d'agents sont une fonctionnalité CLI où une session agit comme le chef d'équipe, coordonnant le travail entre les coéquipiers indépendants |
| Exécuter une logique déterministe sur les appels d'outils (audit, blocage, transformation)                              | [Hooks](/docs/fr/agent-sdk/hooks)                  | Paramètre `hooks` avec rappels, ou scripts shell chargés via `settingSources`                                                                                                                         |
| Donner à Claude un accès structuré aux outils pour un service externe                                                   | [MCP](/docs/fr/agent-sdk/mcp)                      | Paramètre `mcpServers`                                                                                                                                                                                |

<Tip>
  **Subagents versus agent teams :** Les subagents sont éphémères et isolés : conversation fraîche, une tâche, résumé retourné au parent. Les agent teams coordonnent plusieurs instances indépendantes de Claude Code qui partagent une liste de tâches et se envoient des messages directement. Les agent teams sont une fonctionnalité CLI. Voir [What subagents inherit](/docs/fr/agent-sdk/subagents#what-subagents-inherit) et la [agent teams comparison](/docs/fr/agent-teams#compare-with-subagents) pour les détails.
</Tip>

Chaque fonctionnalité que vous activez ajoute à la fenêtre de contexte de votre agent. Pour les coûts par fonctionnalité et comment ces fonctionnalités se superposent, voir [Extend Claude Code](/docs/fr/features-overview#understand-context-costs).

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Étendre Claude Code](/docs/fr/features-overview) : Vue d'ensemble conceptuelle de toutes les fonctionnalités d'extension, avec tableaux de comparaison et analyse des coûts de contexte
* [Compétences dans le SDK](/docs/fr/agent-sdk/skills) : Guide complet pour utiliser les compétences par programmation
* [Sous-agents](/docs/fr/agent-sdk/subagents) : Définir et invoquer les sous-agents pour les sous-tâches isolées
* [Hooks](/docs/fr/agent-sdk/hooks) : Intercepter et contrôler le comportement de l'agent aux points d'exécution clés
* [Permissions](/docs/fr/agent-sdk/permissions) : Contrôler l'accès aux outils avec les modes, les règles et les rappels
* [Invites système](/docs/fr/agent-sdk/modifying-system-prompts) : Injecter du contexte sans fichiers CLAUDE.md
