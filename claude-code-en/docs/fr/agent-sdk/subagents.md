> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sous-agents dans le SDK

> Définissez et invoquez des sous-agents pour isoler le contexte, exécuter des tâches en parallèle et appliquer des instructions spécialisées dans vos applications Claude Agent SDK.

Les sous-agents sont des instances d'agent distinctes que votre agent principal peut générer pour gérer des sous-tâches ciblées.
Utilisez-les pour isoler le contexte, exécuter plusieurs analyses en parallèle et appliquer des instructions spécialisées sans ajouter à l'invite de l'agent principal.

<h2 id="overview">
  Aperçu
</h2>

Vous pouvez créer des sous-agents de trois façons :

* **Par programmation** : utilisez le paramètre `agents` dans vos options `query()`. Consultez les références [TypeScript](/docs/fr/agent-sdk/typescript#agentdefinition) et [Python](/docs/fr/agent-sdk/python#agentdefinition)
* **Basée sur le système de fichiers** : définissez les agents en tant que fichiers markdown dans les répertoires `.claude/agents/`. Consultez [définir les sous-agents en tant que fichiers](/docs/fr/sub-agents)
* **Général intégré** : Claude peut invoquer le sous-agent `general-purpose` intégré à tout moment via l'outil Agent sans que vous ayez besoin de définir quoi que ce soit

Ce guide se concentre sur l'approche par programmation, qui est recommandée pour les applications SDK.

<h2 id="benefits-of-using-subagents">
  Avantages de l'utilisation de sous-agents
</h2>

Parce que les sous-agents sont des instances d'agent distinctes, déléguer du travail à ces derniers vous offre quatre avantages :

* **Isolation du contexte** : chaque sous-agent s'exécute dans sa propre conversation, qui démarre à zéro sauf si le sous-agent est un [fork](/docs/fr/sub-agents#fork-the-current-conversation). De toute façon, les appels d'outils intermédiaires et les résultats restent à l'intérieur du sous-agent ; seul son message final revient au parent. Un sous-agent `research-assistant` peut explorer des dizaines de fichiers sans que tout ce contenu s'accumule dans la conversation principale. Le parent reçoit un résumé concis, pas chaque fichier que le sous-agent a lu. Consultez [What subagents inherit](#what-subagents-inherit) pour savoir exactement ce qui se trouve dans le contexte du sous-agent.
* **Parallélisation** : plusieurs sous-agents peuvent s'exécuter simultanément, de sorte que les sous-tâches indépendantes se terminent dans le temps du plus lent plutôt que la somme de tous. Lors d'une révision de code, vous pouvez exécuter les sous-agents `style-checker`, `security-scanner` et `test-coverage` simultanément au lieu de séquentiellement.
* **Instructions et connaissances spécialisées** : chaque sous-agent peut avoir une invite système adaptée avec une expertise spécifique, des meilleures pratiques et des contraintes. Un sous-agent `database-migration` peut avoir des connaissances détaillées sur les meilleures pratiques SQL, les stratégies de restauration et les vérifications d'intégrité des données qui seraient du bruit inutile dans les instructions de l'agent principal.
* **Restrictions d'outils** : les sous-agents peuvent être limités à des outils spécifiques, réduisant le risque d'actions involontaires. Un sous-agent `doc-reviewer` pourrait n'avoir accès qu'aux outils Read et Grep, garantissant qu'il peut analyser mais ne peut jamais modifier accidentellement vos fichiers de documentation.

<h2 id="create-subagents">
  Créer des sous-agents
</h2>

<h3 id="programmatic-definition-recommended">
  Définition programmatique (recommandée)
</h3>

Définissez les sous-agents directement dans votre code en utilisant le paramètre `agents`. Claude invoque les sous-agents via l'outil `Agent`.

La plupart des exemples de cette page n'affichent que le résultat final. Pour confirmer que Claude a délégué à un sous-agent plutôt que de répondre directement, consultez [Détecter l'invocation de sous-agent](#detect-subagent-invocation).

Cet exemple crée deux sous-agents : un examinateur de code avec accès en lecture seule et un exécuteur de tests qui peut exécuter des commandes.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  Configuration d'AgentDefinition
</h3>

| Champ             | Type                                                        | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                          |
| :---------------- | :---------------------------------------------------------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Oui    | Description en langage naturel de quand utiliser cet agent                                                                                                                                                                                                                                                                                                                                           |
| `prompt`          | `string`                                                    | Oui    | L'invite système de l'agent définissant son rôle et son comportement                                                                                                                                                                                                                                                                                                                                 |
| `tools`           | `string[]`                                                  | Non    | Tableau des noms d'outils autorisés. S'il est omis, hérite de tous les [outils disponibles pour les sous-agents](/docs/fr/sub-agents#available-tools)                                                                                                                                                                                                                                                     |
| `disallowedTools` | `string[]`                                                  | Non    | Tableau des noms d'outils à supprimer de l'ensemble d'outils de l'agent. Les modèles au niveau du serveur MCP sont également acceptés : `mcp__server` ou `mcp__server__*` supprime tous les outils de ce serveur, et `mcp__*` supprime tous les outils MCP de n'importe quel serveur                                                                                                                 |
| `model`           | `string`                                                    | Non    | Remplacement du modèle pour cet agent. Accepte un alias tel que `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, ou un ID de modèle complet. `'inherit'` utilise le modèle principal. Lorsque vous l'omettez, Claude Code choisit le modèle dans l'[ordre des modèles de sous-agent](/docs/fr/sub-agents#choose-a-model)                                                                          |
| `skills`          | `string[]`                                                  | Non    | Liste des noms de compétences à précharger dans le contexte de l'agent au démarrage. Les compétences non listées restent invocables via l'outil Skill                                                                                                                                                                                                                                                |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Non    | Source de mémoire pour cet agent                                                                                                                                                                                                                                                                                                                                                                     |
| `mcpServers`      | `(string \| object)[]`                                      | Non    | Serveurs MCP disponibles pour cet agent, par nom ou configuration en ligne                                                                                                                                                                                                                                                                                                                           |
| `initialPrompt`   | `string`                                                    | Non    | Auto-soumis comme le premier tour utilisateur lorsque cet agent s'exécute en tant qu'agent du thread principal. Ignoré lorsque l'agent est invoqué en tant que sous-agent                                                                                                                                                                                                                            |
| `maxTurns`        | `number`                                                    | Non    | Nombre maximum de tours d'agent avant que l'agent s'arrête. Lorsque l'agent atteint la limite, Claude Code retourne sa sortie marquée comme partielle, et vous pouvez [reprendre l'agent](#resume-subagents) pour continuer. Le marquage partiel nécessite Claude Code v2.1.246 ou ultérieur                                                                                                         |
| `background`      | `boolean`                                                   | Non    | Exécuter cet agent en tant que tâche d'arrière-plan non bloquante lorsqu'il est invoqué                                                                                                                                                                                                                                                                                                              |
| `omitClaudeMd`    | `boolean`                                                   | Non    | Exécuter cet agent sans les fichiers CLAUDE.md utilisateur, projet et local lorsqu'il s'exécute en tant que sous-agent ; les fichiers de politique gérés se chargent toujours. Ignoré lorsque l'agent s'exécute en tant qu'agent du thread principal. Nécessite TypeScript Agent SDK v0.3.271 ou ultérieur. Le SDK Python [`AgentDefinition`](/docs/fr/agent-sdk/python#agentdefinition) n'a pas ce champ |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Non    | Niveau d'effort de raisonnement pour cet agent                                                                                                                                                                                                                                                                                                                                                       |
| `permissionMode`  | `PermissionMode`                                            | Non    | Mode de permission pour l'exécution des outils au sein de cet agent. Les [règles d'héritage des sous-agents](/docs/fr/agent-sdk/permissions#available-modes) décident quand il s'applique                                                                                                                                                                                                                 |

Dans le SDK Python, les noms de champs multi-mots tels que `disallowedTools` et `mcpServers` conservent leur orthographe camelCase pour correspondre au format de transmission plutôt que de suivre la convention snake\_case de Python. Consultez la [référence `AgentDefinition`](/docs/fr/agent-sdk/python#agentdefinition) pour plus de détails.

Les sous-agents s'exécutent en arrière-plan par défaut. Un appel d'outil Agent qui omet l'entrée [`run_in_background`](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) lance un sous-agent d'arrière-plan, et Claude définit `run_in_background: false` lorsqu'il a besoin du résultat avant de continuer. Définissez le champ `background` sur `true` pour forcer l'exécution en arrière-plan pour un agent spécifique, quel que soit ce que Claude demande. Avant Claude Code v2.1.198, la valeur par défaut d'arrière-plan était en cours de déploiement progressif, et un appel d'outil Agent qui omettait `run_in_background` pouvait exécuter le sous-agent de manière synchrone.

Les sous-agents peuvent également générer leurs propres sous-agents. Pour limiter la profondeur de cet imbrication, le nombre de sous-agents qui s'exécutent à la fois et le montant qu'une requête dépense, consultez [Limiter la profondeur, la concurrence et les dépenses des sous-agents](#cap-subagent-depth-concurrency-and-spend).

<h3 id="filesystem-based-definition-alternative">
  Définition basée sur le système de fichiers (alternative)
</h3>

Vous pouvez également définir les sous-agents en tant que fichiers markdown dans les répertoires `.claude/agents/`. Consultez la [documentation des sous-agents Claude Code](/docs/fr/sub-agents) pour plus de détails sur cette approche. Les agents définis par programmation ont la priorité sur les agents basés sur le système de fichiers portant le même nom.

<Note>
  Lorsque Claude appelle l'outil Agent sans `subagent_type`, il obtient le sous-agent intégré `general-purpose`, que Claude peut générer même lorsque vous ne définissez aucun agent vous-même. La définition de [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/fr/env-vars) supprime cette valeur par défaut, et un tel appel échoue avec [`subagent_type is required`](/docs/fr/errors#subagent-type-is-required).
</Note>

<h2 id="what-subagents-inherit">
  Ce que les sous-agents héritent
</h2>

À moins que le sous-agent ne soit un [fork](/docs/fr/sub-agents#fork-the-current-conversation), sa fenêtre de contexte commence vierge, sans conversation parent, mais n'est pas vide. Le seul contenu que vous transmettez du parent au sous-agent est la chaîne de prompt de l'outil Agent, donc incluez directement dans ce prompt tous les chemins de fichiers, messages d'erreur ou décisions dont le sous-agent a besoin.

Un sous-agent qui dispose de l'outil [`SendMessage`](/docs/fr/tools-reference) commence avec une liste des autres agents nommés exécutés dans la session, il sait donc quels noms il peut utiliser pour envoyer des messages. Claude Code ajoute automatiquement la liste au premier tour du sous-agent. Un [fork](/docs/fr/sub-agents#fork-the-current-conversation) ne reçoit pas la liste car il hérite de la conversation parent à la place.

Un sous-agent hérite également de la configuration de la réflexion étendue de la session principale.

Le tableau ci-dessous énumère ce que le contexte d'un sous-agent non-fork contient et ce qu'il exclut.

| Le sous-agent reçoit                                                                                                                                                                                            | Le sous-agent ne reçoit pas                                                           |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------ |
| Son propre prompt système (`AgentDefinition.prompt`) et le prompt de l'outil Agent                                                                                                                              | L'historique de conversation du parent ou les résultats des outils                    |
| Project CLAUDE.md (chargé via [`settingSources`](/docs/fr/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)), sauf si l'agent définit [`omitClaudeMd`](#agentdefinition-configuration) | Contenu de compétences préchargées, sauf s'il est listé dans `AgentDefinition.skills` |
| Définitions d'outils (héritées du parent ou du sous-ensemble dans `tools`, [filtrées pour les exécutions en arrière-plan](/docs/fr/sub-agents#available-tools))                                                      | Le prompt système du parent                                                           |

<Note>
  Le parent reçoit le message final du sous-agent comme résultat de l'outil Agent, mais peut le résumer dans sa propre réponse. Pour préserver la sortie du sous-agent textuellement dans la réponse visible par l'utilisateur, incluez une instruction pour le faire dans le prompt ou l'option `systemPrompt` que vous transmettez à l'appel `query()` principal.

  Dans la v2.1.210 et versions ultérieures, Claude Code [analyse le message final pour détecter des motifs ressemblant à des instructions](/docs/fr/sub-agents#subagent-output-scanning) avant que le parent ne le lise. L'analyse traite trois types de motifs différemment :

  * **Imitation de balise de contrôle** : Claude Code neutralise une balise que seul le harnais émet, comme un bloc `<system-reminder>`, sur place. Il insère une barre oblique inverse après le crochet ouvrant et ne supprime rien.
  * **Mentions de configuration de permissions** : Claude Code conserve les références à la configuration de permissions, comme `.claude/settings.json`, `bypassPermissions`, ou `--dangerously-skip-permissions`, telles qu'elles sont écrites.
  * **Marqueurs de tour** : une ligne qui commence par `Human:` ou `Assistant:` reçoit une barre oblique inverse avant les deux-points, de sorte que le message ne peut pas imiter une limite de tour de conversation.

  Pour une correspondance de balise de contrôle ou de configuration de permissions, Claude Code ajoute une ligne de marqueur `[harness: ...]` nommant les motifs correspondants ; une correspondance de marqueur de tour n'ajoute pas la ligne de marqueur. Ce sont les seules modifications que l'analyse effectue : elle ne supprime jamais ou ne reformule jamais le texte du sous-agent.
</Note>

Une erreur API qui termine le sous-agent prématurément, comme une limite de débit, n'est jamais livrée comme résultat. Voir [Erreurs API dans les sous-agents](/docs/fr/sub-agents#api-errors-in-subagents) pour le comportement au premier plan et en arrière-plan.

<h2 id="invoke-subagents">
  Invoquer des sous-agents
</h2>

<h3 id="automatic-invocation">
  Invocation automatique
</h3>

Claude décide automatiquement quand invoquer des sous-agents en fonction de la tâche et de la `description` de chaque sous-agent. Par exemple, si vous définissez un sous-agent `performance-optimizer` avec la description « Spécialiste de l'optimisation des performances pour l'optimisation des requêtes », Claude l'invoquera quand votre prompt mentionne l'optimisation des requêtes.

Écrivez des descriptions claires et spécifiques pour que Claude puisse faire correspondre les tâches au bon sous-agent.

<h3 id="explicit-invocation">
  Invocation explicite
</h3>

Pour garantir que Claude utilise un sous-agent spécifique, mentionnez-le par son nom dans votre prompt :

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Cela contourne la correspondance automatique et invoque directement le sous-agent nommé.

<h3 id="dynamic-agent-configuration">
  Configuration dynamique d'agent
</h3>

Vous pouvez créer des définitions d'agent de manière dynamique en fonction des conditions d'exécution. Cet exemple crée un examinateur de sécurité avec différents niveaux de rigueur, en utilisant un modèle plus capable pour les examens stricts.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  Détecter l'invocation d'un sous-agent
</h2>

Claude invoque les sous-agents via l'outil Agent. Pour détecter quand un sous-agent est invoqué, recherchez les blocs `tool_use` où `name` est `"Agent"`. Les messages provenant du contexte d'un sous-agent incluent un champ `parent_tool_use_id`.

<Note>
  L'outil apparaît comme `"Agent"` dans les blocs `tool_use` mais comme `"Task"` dans la liste des outils `system:init`. Avant Claude Code v2.1.63, les blocs `tool_use` le nommaient également `"Task"`. Pour que la détection fonctionne sur les versions du SDK, faites correspondre les deux valeurs dans `block.name`.
</Note>

La structure du message diffère entre les SDK. En Python, vous accédez directement aux blocs de contenu via `message.content`. En TypeScript, `SDKAssistantMessage` enveloppe le message de l'API Claude, vous accédez donc au contenu via `message.message.content`.

Cet exemple itère à travers les messages en flux, enregistrant quand un sous-agent est invoqué et quand les messages suivants proviennent du contexte d'exécution de ce sous-agent.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the code-reviewer agent to review this codebase",
    options: {
      allowedTools: ["Read", "Glob", "Grep", "Agent"],
      agents: {
        "code-reviewer": {
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  Reprendre les sous-agents
</h2>

Vous pouvez reprendre un sous-agent pour continuer là où il s'est arrêté plutôt que de recommencer à zéro. Un sous-agent repris conserve son historique de conversation complet, y compris tous les appels d'outils précédents, les résultats et le raisonnement.

Lorsqu'un sous-agent s'arrête à sa limite [`maxTurns`](#agentdefinition-configuration), Claude Code marque la sortie dans le résultat de l'outil Agent comme partielle, afin que Claude sache que l'exécution est inachevée.

Lorsqu'un sous-agent se termine, le résultat de l'outil Agent inclut un bloc de texte contenant `agentId: <id>`. Les agents [`Explore` et `Plan` intégrés](/docs/fr/sub-agents#built-in-subagents) sont ponctuels et ne retournent pas d'`agentId`, donc utilisez un agent personnalisé ou `general-purpose` lorsque vous avez besoin de reprendre. Pour reprendre un sous-agent par programmation :

1. **Capturer l'ID de session** : extraire `session_id` des messages lors de la première requête
2. **Extraire l'ID de l'agent** : analyser `agentId` à partir du texte du résultat de l'outil Agent
3. **Reprendre la session** : passer `resume: sessionId` dans les options de la deuxième requête, et inclure l'ID de l'agent dans votre prompt. Chaque appel `query()` démarre une nouvelle session par défaut, et vous devez reprendre la même session pour accéder à la transcription du sous-agent.

<Note>
  Lorsque vous utilisez un agent personnalisé, passez la même définition d'agent dans le paramètre `agents` pour les deux requêtes.
</Note>

L'exemple ci-dessous définit un agent personnalisé `endpoint-finder`. La première requête l'exécute et capture l'ID de session et l'ID de l'agent à partir du résultat de l'outil Agent, puis la deuxième requête reprend la session pour poser une question de suivi qui nécessite le contexte de la première analyse.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

Les transcriptions des sous-agents sont stockées dans des fichiers séparés et persistent indépendamment de la conversation principale. Voir [reprendre les sous-agents dans Claude Code](/docs/fr/sub-agents#resume-subagents) pour le comportement de compaction et la période de nettoyage `cleanupPeriodDays`.

<h2 id="tool-restrictions">
  Restrictions d'outils
</h2>

Utilisez le champ `tools` pour limiter ce qu'un sous-agent peut faire :

* **Omettre `tools`** : le sous-agent obtient tous les [outils disponibles pour les sous-agents](/docs/fr/sub-agents#available-tools)
* **Lister les outils** : le sous-agent n'obtient que ceux-ci. Par exemple, un examinateur de code qui ne devrait jamais modifier les fichiers obtient `["Read", "Grep", "Glob"]`

Un outil que vous omettez n'est pas du tout dans la session du sous-agent : Claude fonctionne sans lui, sans invite de permission ni erreur.

Cet exemple crée un agent d'analyse en lecture seule qui peut examiner le code mais ne peut pas modifier les fichiers ni exécuter les commandes.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
                  )
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  Combinaisons d'outils courantes
</h3>

| Cas d'usage              | Outils                                  | Description                                                                   |
| :----------------------- | :-------------------------------------- | :---------------------------------------------------------------------------- |
| Analyse en lecture seule | `Read`, `Grep`, `Glob`                  | Peut examiner le code mais pas le modifier ni l'exécuter                      |
| Exécution de tests       | `Bash`, `Read`, `Grep`                  | Peut exécuter des commandes et analyser la sortie                             |
| Modification de code     | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Accès complet en lecture/écriture sans exécution de commandes                 |
| Accès complet            | Tous les outils                         | Hérite des outils disponibles pour les sous-agents (omettre le champ `tools`) |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  Limiter la profondeur, la concurrence et les dépenses des sous-agents
</h2>

<Note>
  Cette section décrit le SDK TypeScript v0.3.219 et le SDK Python v0.2.127 et versions ultérieures, les versions qui incluent Claude Code v2.1.219 ou version ultérieure. Sur les versions antérieures, certaines de ces limites sont manquantes ou ont des valeurs par défaut différentes, donc mettez à jour avant de vous fier à elles pour limiter une exécution. La [référence des variables d'environnement](/docs/fr/env-vars) et [les tours et le budget](/docs/fr/agent-sdk/agent-loop#turns-and-budget) enregistrent la version de Claude Code qui a ajouté chaque variable et l'application de la limite de dépenses aux sous-agents.
</Note>

Claude décide par lui-même quand créer un sous-agent et combien en créer. Chaque sous-agent effectue ses propres requêtes API, qui comptent dans le `total_cost_usd` de la requête, et un sous-agent peut créer ses propres sous-agents, donc une invite peut se transformer en un arbre d'agents.

Vous pouvez limiter cette croissance de trois façons : la profondeur d'imbrication des sous-agents, le nombre qui s'exécutent simultanément et le montant que la requête entière dépense. Définissez les limites de profondeur et de concurrence comme variables d'environnement via l'option [`env`](/docs/fr/agent-sdk/typescript#options), et la limite de dépenses comme option de requête :

| Limite      | Définissez-la avec                                       | Par défaut                                                                                                     | Ce que Claude Code fait à la limite                                                                                                                                                                                                                                                                                                                                                                   |
| :---------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Profondeur  | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/fr/env-vars)   | `3` couches de sous-agents en dessous de votre agent principal. `1` empêche vos sous-agents de créer les leurs | Laisse un sous-agent à la couche inférieure incapable de créer un sous-agent, il effectue donc lui-même son travail délégué. Voir [sous-agents imbriqués](/docs/fr/sub-agents#let-subagents-spawn-their-own-subagents)                                                                                                                                                                                     |
| Concurrence | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/fr/env-vars)   | `20` sous-agents s'exécutant simultanément, en comptant chaque sous-agent que Claude crée avec l'outil Agent   | Refuse de créer un autre sous-agent, en retournant `Concurrent subagent limit reached`, jusqu'à ce que le nombre en cours d'exécution tombe en dessous de la limite. Les sessions avec [ultracode](/docs/fr/model-config#adjust-effort-level) actif ne sont jamais refusées. Voir la [limite de sous-agents concurrents](/docs/fr/sub-agents#concurrent-subagent-limit)                                         |
| Dépenses    | `maxBudgetUsd` en TypeScript, `max_budget_usd` en Python | Pas de limite. Compte les dépenses de l'appel lui-même, requêtes de sous-agents incluses                       | Applique la limite de trois façons : refuse de créer d'autres sous-agents, en retournant `Budget limit reached`, arrête les sous-agents en arrière-plan qui s'exécutent toujours, et termine la requête avec le sous-type de résultat `error_max_budget_usd`. Pour savoir comment les limites se comportent au cours d'une session, voir [tours et budget](/docs/fr/agent-sdk/agent-loop#turns-and-budget) |

Les deux SDK traitent l'option `env` différemment : le SDK TypeScript remplace l'environnement du sous-processus par celui-ci, donc propagez `process.env` dedans pour conserver les variables comme `PATH`, tandis que le SDK Python le fusionne dans l'environnement hérité. Cet exemple désactive l'imbrication, autorise au maximum cinq sous-agents à la fois, et arrête la requête une fois que les dépenses estimées atteignent 5 \$ :

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

Ce que vous voyez dépend de la limite, le cas échéant, que la requête atteint :

* **Sous la limite de dépenses** : vous voyez `success` et le coût estimé.
* **À la limite de dépenses** : vous voyez `error_max_budget_usd` avec un coût égal ou supérieur à `5`, puis votre gestionnaire d'erreur s'exécute.
* **À la limite de concurrence** : vous voyez un bloc `tool_result` dans le flux de messages portant `Concurrent subagent limit reached`. Claude reçoit le même bloc comme résultat de l'outil Agent.

<h3 id="run-opus-5-with-subagents">
  Exécuter Opus 5 avec des sous-agents
</h3>

Claude Opus 5 délègue aux sous-agents plus facilement que les modèles antérieurs, donc les [limites de profondeur, de concurrence et de dépenses](#cap-subagent-depth-concurrency-and-spend) importent le plus sur les requêtes qui exécutent Opus 5. Le [guide d'invite Opus 5](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning) contient une instruction de délégation que vous pouvez ajouter à n'importe quelle invite. Le fait que Claude Code ajoute sa propre instruction dépend du [système d'invite](/docs/fr/agent-sdk/modifying-system-prompts#how-system-prompts-work) que vous utilisez :

* **Préréglage `claude_code`** : quand le modèle est Opus 5, Claude Code ajoute une ligne à son système d'invite indiquant à Claude de ne pas appeler l'outil Agent à moins qu'on ne le lui demande. L'outil Agent reste disponible.
* **Une invite personnalisée, ou pas de `systemPrompt`** : Claude Code ne construit pas son système d'invite, donc cette ligne est absente. Ajoutez l'instruction de délégation du guide d'invite à votre propre invite.

Chaque instruction ne fait que guider Claude, donc définissez également les limites. Claude Code les applique peu importe comment Claude décide de déléguer.

<h2 id="scale-up-with-dynamic-workflows">
  Augmenter l'échelle avec des flux de travail dynamiques
</h2>

Les sous-agents fonctionnent bien pour quelques tâches déléguées par tour. Pour les exécutions qui coordonnent des dizaines à des centaines d'agents, utilisez l'outil `Workflow`, qui déplace l'orchestration dans un script que le runtime exécute en dehors du contexte de conversation. Voir [flux de travail dynamiques](/docs/fr/workflows) pour savoir comment les flux de travail diffèrent de la délégation de sous-agents tour par tour.

L'outil `Workflow` est disponible dans le TypeScript Agent SDK v0.3.149 et versions ultérieures. Incluez `Workflow` dans `allowedTools` pour approuver automatiquement les exécutions de flux de travail. Les schémas d'entrée et de sortie de l'outil sont listés dans la [référence TypeScript](/docs/fr/agent-sdk/typescript#workflow).

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude ne délègue pas aux sous-agents
</h3>

Si Claude complète les tâches directement au lieu de déléguer à votre sous-agent :

* **Utilisez des invites explicites** : mentionnez le sous-agent par son nom dans votre invite, par exemple « Utilisez l'agent code-reviewer pour vérifier le module d'authentification »
* **Écrivez une description claire** : expliquez exactement quand utiliser le sous-agent pour que Claude puisse faire correspondre les tâches de manière appropriée

<h3 id="filesystem-based-agents-not-loading">
  Les agents basés sur le système de fichiers ne se chargent pas
</h3>

Claude Code surveille `~/.claude/agents/` et `.claude/agents/` et détecte un fichier d'agent nouveau ou modifié en quelques secondes, sans redémarrage nécessaire. Si une définition n'apparaît jamais, vérifiez ces causes :

* **Nouveau répertoire `agents`** : le moniteur couvre uniquement les répertoires qui existaient au démarrage de la session, donc le premier fichier dans un nouveau répertoire nécessite un redémarrage de session. C'est la cause la plus courante.
* **Frontmatter invalide ou `name` en doublon** : vérifiez le YAML du fichier et si un agent existant utilise déjà le `name`.
* **`--disable-slash-commands`** : les sessions démarrées avec ce flag ne surveillent pas ces répertoires et nécessitent toujours un redémarrage pour charger les nouveaux fichiers.
* **Un fichier sous un répertoire ajouté** : Claude Code charge `.claude/agents/` à partir des répertoires ajoutés avec l'option `add_dirs` (Python) ou `additionalDirectories` (TypeScript), ou la CLI `--add-dir` ou `/add-dir`, mais ne les surveille pas, donc un fichier nouveau ou modifié là-bas nécessite un redémarrage de session.
* **Un agent programmatique avec le même nom** : les `agents` passés à `query()` remplacent un agent du système de fichiers avec le même nom.

Pour le format de fichier, consultez [comment écrire des fichiers de sous-agent](/docs/fr/sub-agents#write-subagent-files).

<h2 id="related-documentation">
  Documentation connexe
</h2>

* [Sous-agents Claude Code](/docs/fr/sub-agents) : documentation complète des sous-agents incluant les définitions basées sur le système de fichiers
* [Flux de travail dynamiques](/docs/fr/workflows) : orchestrez de nombreux sous-agents à partir d'un script pour les tâches trop importantes pour une seule conversation
* [Aperçu du SDK](/docs/fr/agent-sdk/overview) : prise en main du Claude Agent SDK
