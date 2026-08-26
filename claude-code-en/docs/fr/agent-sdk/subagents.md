> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sous-agents dans le SDK

> Définissez et invoquez des sous-agents pour isoler le contexte, exécuter des tâches en parallèle et appliquer des instructions spécialisées dans vos applications Claude Agent SDK.

Les sous-agents sont des instances d'agent distinctes que votre agent principal peut créer pour gérer des sous-tâches ciblées.
Utilisez les sous-agents pour isoler le contexte, exécuter plusieurs analyses en parallèle et appliquer des instructions spécialisées sans surcharger l'invite de l'agent principal.

Ce guide explique comment définir et utiliser les sous-agents dans le SDK en utilisant le paramètre `agents`.

<h2 id="overview">
  Aperçu
</h2>

Vous pouvez créer des sous-agents de trois façons :

* **Par programmation** : utilisez le paramètre `agents` dans vos options `query()`. Consultez les références [TypeScript](/docs/fr/agent-sdk/typescript#agentdefinition) et [Python](/docs/fr/agent-sdk/python#agentdefinition)
* **Basé sur le système de fichiers** : définissez les agents comme des fichiers markdown dans les répertoires `.claude/agents/`. Consultez [définir les sous-agents comme fichiers](/docs/fr/sub-agents)
* **Général intégré** : Claude peut invoquer le sous-agent `general-purpose` intégré à tout moment via l'outil Agent sans que vous ayez besoin de rien définir

Ce guide se concentre sur l'approche programmatique, qui est recommandée pour les applications SDK.

Lorsque vous définissez des sous-agents, Claude détermine s'il faut les invoquer en fonction du champ `description` de chaque sous-agent. Écrivez des descriptions claires qui expliquent quand utiliser le sous-agent, et Claude délèguera automatiquement les tâches appropriées. Vous pouvez également demander explicitement un sous-agent par son nom dans votre invite, par exemple « Utilisez l'agent code-reviewer pour... ».

<h2 id="benefits-of-using-subagents">
  Avantages de l'utilisation des sous-agents
</h2>

<h3 id="context-isolation">
  Isolation du contexte
</h3>

Chaque sous-agent s'exécute dans sa propre conversation nouvelle. Les appels d'outils intermédiaires et les résultats restent à l'intérieur du sous-agent ; seul son message final revient au parent. Voir [Ce que les sous-agents héritent](#what-subagents-inherit) pour savoir exactement ce qui se trouve dans le contexte du sous-agent.

**Exemple :** un sous-agent `research-assistant` peut explorer des dizaines de fichiers sans que le contenu de ces fichiers s'accumule dans la conversation principale. Le parent reçoit un résumé concis, pas chaque fichier que le sous-agent a lu.

<h3 id="parallelization">
  Parallélisation
</h3>

Plusieurs sous-agents peuvent s'exécuter simultanément, de sorte que les sous-tâches indépendantes se terminent dans le temps du plus lent plutôt que la somme de tous.

**Exemple :** lors d'une révision de code, vous pouvez exécuter les sous-agents `style-checker`, `security-scanner` et `test-coverage` simultanément au lieu de séquentiellement.

<h3 id="specialized-instructions-and-knowledge">
  Instructions et connaissances spécialisées
</h3>

Chaque sous-agent peut avoir des invites système adaptées avec une expertise spécifique, des meilleures pratiques et des contraintes.

**Exemple :** un sous-agent `database-migration` peut avoir des connaissances détaillées sur les meilleures pratiques SQL, les stratégies de restauration et les vérifications d'intégrité des données qui seraient du bruit inutile dans les instructions du principal agent.

<h3 id="tool-restrictions">
  Restrictions d'outils
</h3>

Les sous-agents peuvent être limités à des outils spécifiques, réduisant le risque d'actions involontaires.

**Exemple :** un sous-agent `doc-reviewer` pourrait n'avoir accès qu'aux outils Read et Grep, garantissant qu'il peut analyser mais ne peut jamais modifier accidentellement vos fichiers de documentation.

<h2 id="create-subagents">
  Créer des sous-agents
</h2>

<h3 id="programmatic-definition-recommended">
  Définition programmatique (recommandée)
</h3>

Définissez les sous-agents directement dans votre code en utilisant le paramètre `agents`. Claude invoque les sous-agents via l'outil `Agent`, donc incluez `Agent` dans `allowedTools` pour approuver automatiquement les invocations de sous-agents sans invite de permission.

La plupart des exemples de cette page n'impriment que le résultat final. Pour confirmer que Claude a délégué à un sous-agent plutôt que de répondre directement, voir [Détecter l'invocation de sous-agent](#detect-subagent-invocation).

Cet exemple crée deux sous-agents : un examinateur de code avec accès en lecture seule et un exécuteur de tests qui peut exécuter des commandes.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools, including Agent for subagent invocation
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
      // Auto-approve these tools, including Agent for subagent invocation
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
  Configuration AgentDefinition
</h3>

| Champ             | Type                                                        | Requis | Description                                                                                                                                                                                                                                                                          |
| :---------------- | :---------------------------------------------------------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Oui    | Description en langage naturel de quand utiliser cet agent                                                                                                                                                                                                                           |
| `prompt`          | `string`                                                    | Oui    | L'invite système de l'agent définissant son rôle et son comportement                                                                                                                                                                                                                 |
| `tools`           | `string[]`                                                  | Non    | Tableau des noms d'outils autorisés. S'il est omis, hérite de tous les outils                                                                                                                                                                                                        |
| `disallowedTools` | `string[]`                                                  | Non    | Tableau des noms d'outils à supprimer de l'ensemble d'outils de l'agent. Les modèles au niveau du serveur MCP sont également acceptés : `mcp__server` ou `mcp__server__*` supprime tous les outils de ce serveur, et `mcp__*` supprime tous les outils MCP de n'importe quel serveur |
| `model`           | `string`                                                    | Non    | Remplacement du modèle pour cet agent. Accepte un alias tel que `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, ou un ID de modèle complet. Par défaut le modèle principal s'il est omis                                                                                    |
| `skills`          | `string[]`                                                  | Non    | Liste des noms de compétences à précharger dans le contexte de l'agent au démarrage. Les compétences non listées restent invocables via l'outil Skill                                                                                                                                |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Non    | Source de mémoire pour cet agent                                                                                                                                                                                                                                                     |
| `mcpServers`      | `(string \| object)[]`                                      | Non    | Serveurs MCP disponibles pour cet agent, par nom ou configuration en ligne                                                                                                                                                                                                           |
| `initialPrompt`   | `string`                                                    | Non    | Soumis automatiquement comme premier tour utilisateur lorsque cet agent s'exécute en tant qu'agent de thread principal. Ignoré lorsque l'agent est invoqué en tant que sous-agent                                                                                                    |
| `maxTurns`        | `number`                                                    | Non    | Nombre maximum de tours d'agent avant que l'agent s'arrête                                                                                                                                                                                                                           |
| `background`      | `boolean`                                                   | Non    | Exécuter cet agent comme une tâche de fond non bloquante lorsqu'il est invoqué                                                                                                                                                                                                       |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Non    | Niveau d'effort de raisonnement pour cet agent                                                                                                                                                                                                                                       |
| `permissionMode`  | `PermissionMode`                                            | Non    | Mode de permission pour l'exécution des outils au sein de cet agent                                                                                                                                                                                                                  |

Dans le SDK Python, les noms de champs multi-mots tels que `disallowedTools` et `mcpServers` conservent leur orthographe camelCase pour correspondre au format de transmission plutôt que de suivre la convention snake\_case de Python. Voir la référence [`AgentDefinition`](/docs/fr/agent-sdk/python#agentdefinition) pour plus de détails.

Deux comportements de sous-agent ont changé dans Claude Code v2.1.198 :

* Les sous-agents s'exécutent en arrière-plan par défaut. Un appel d'outil Agent qui omet l'entrée [`run_in_background`](/docs/fr/agent-sdk/typescript) lance un sous-agent en arrière-plan, et Claude définit `run_in_background: false` lorsqu'il a besoin du résultat avant de continuer. Avant v2.1.198, l'omission de `run_in_background` exécutait le sous-agent de manière synchrone. Définissez le champ `background` à `true` pour forcer l'exécution en arrière-plan pour un agent spécifique indépendamment de ce que Claude demande.
* Un sous-agent hérite de la configuration de la réflexion étendue de la session principale. Sur les versions antérieures, la réflexion étendue est désactivée à l'intérieur des sous-agents indépendamment du paramètre de la session principale.

<Note>
  À partir de Claude Code v2.1.172, les sous-agents peuvent créer leurs propres sous-agents. Un sous-agent cinq niveaux en dessous de l'agent principal ne peut pas créer d'autres sous-agents, indépendamment du fait qu'il s'exécute au premier plan ou en arrière-plan. Pour empêcher un sous-agent de créer d'autres agents, omettez `Agent` de son tableau `tools` ou ajoutez-le à `disallowedTools`. Voir [sous-agents imbriqués](/docs/fr/sub-agents#spawn-nested-subagents) pour les règles de profondeur complètes.
</Note>

<h3 id="filesystem-based-definition-alternative">
  Définition basée sur le système de fichiers (alternative)
</h3>

Vous pouvez également définir les sous-agents comme des fichiers markdown dans les répertoires `.claude/agents/`. Voir la [documentation des sous-agents Claude Code](/docs/fr/sub-agents) pour plus de détails sur cette approche. Les agents définis par programmation ont la priorité sur les agents basés sur le système de fichiers portant le même nom.

<Note>
  Même sans définir de sous-agents personnalisés, Claude peut créer le sous-agent `general-purpose` intégré. Ceci est utile pour déléguer des tâches de recherche ou d'exploration sans créer d'agents spécialisés. Incluez `Agent` dans `allowedTools` afin que ces invocations s'approuvent automatiquement sans invite de permission.
</Note>

<h2 id="what-subagents-inherit">
  Ce que les sous-agents héritent
</h2>

La fenêtre de contexte d'un sous-agent commence fraîche, sans conversation parent, mais n'est pas vide. Le seul contenu que vous transmettez du parent au sous-agent est la chaîne d'invite de l'outil Agent, donc incluez tous les chemins de fichiers, messages d'erreur ou décisions dont le sous-agent a besoin directement dans cette invite.

Un sous-agent qui dispose de l'outil [`SendMessage`](/docs/fr/tools-reference) commence avec une liste des autres agents nommés exécutés dans la session, il sait donc quels noms il peut utiliser pour envoyer des messages. Claude Code ajoute la liste au premier tour du sous-agent automatiquement. Un [fork](/docs/fr/sub-agents#fork-the-current-conversation) n'obtient pas la liste car il hérite de la conversation parent à la place. La liste nécessite Claude Code v2.1.206 ou ultérieur.

| Le sous-agent reçoit                                                                                                                       | Le sous-agent ne reçoit pas                                                               |
| :----------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| Sa propre invite système (`AgentDefinition.prompt`) et l'invite de l'outil Agent                                                           | L'historique de conversation du parent ou les résultats des outils                        |
| Le CLAUDE.md du projet (chargé via [`settingSources`](/docs/fr/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)) | Le contenu des compétences préchargées, sauf s'il est listé dans `AgentDefinition.skills` |
| Les définitions d'outils (héritées du parent, ou le sous-ensemble dans `tools`)                                                            | L'invite système du parent                                                                |

<Note>
  Le parent reçoit le message final du sous-agent tel quel comme résultat de l'outil Agent, mais peut le résumer dans sa propre réponse. Pour préserver la sortie du sous-agent tel quel dans la réponse visible par l'utilisateur, incluez une instruction pour le faire dans l'invite ou l'option `systemPrompt` que vous transmettez à l'appel `query()` principal.
</Note>

Une erreur API qui termine le sous-agent prématurément, comme une limite de débit, n'est jamais livrée comme résultat. Si une limite de débit, une surcharge ou une erreur serveur interrompt un sous-agent au premier plan qui a déjà produit une sortie textuelle, l'outil Agent retourne cette sortie partielle avec une note indiquant que le sous-agent n'a pas terminé. Un sous-agent qui n'a rien produit, ou dont la seule sortie était des appels d'outils sans texte, échoue avec un message d'erreur, `Agent terminated early due to an API error`, suivi du détail de l'erreur. Consultez [API errors in subagents](/docs/fr/sub-agents#api-errors-in-subagents) pour le comportement au premier plan et en arrière-plan.

Cette gestion des sorties partielles nécessite Claude Code v2.1.199 ou ultérieur. Dans v2.1.199, une limite de débit, une surcharge ou une erreur serveur laissait la forme contenant uniquement des appels d'outils avec un résultat partiel vide contenant seulement la note d'interruption.

<h2 id="invoke-subagents">
  Invoquer les sous-agents
</h2>

<h3 id="automatic-invocation">
  Invocation automatique
</h3>

Claude décide automatiquement quand invoquer les sous-agents en fonction de la tâche et de la `description` de chaque sous-agent. Par exemple, si vous définissez un sous-agent `performance-optimizer` avec la description « Spécialiste de l'optimisation des performances pour l'optimisation des requêtes », Claude l'invoquera lorsque votre invite mentionne l'optimisation des requêtes.

Écrivez des descriptions claires et spécifiques pour que Claude puisse faire correspondre les tâches au bon sous-agent.

<h3 id="explicit-invocation">
  Invocation explicite
</h3>

Pour garantir que Claude utilise un sous-agent spécifique, mentionnez-le par son nom dans votre invite :

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Cela contourne la correspondance automatique et invoque directement le sous-agent nommé.

<h3 id="dynamic-agent-configuration">
  Configuration d'agent dynamique
</h3>

Vous pouvez créer des définitions d'agent dynamiquement en fonction des conditions d'exécution. Cet exemple crée un examinateur de sécurité avec différents niveaux de rigueur, en utilisant un modèle plus puissant pour les révisions strictes.

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
  Détection de l'invocation de sous-agents
</h2>

Claude invoque les sous-agents via l'outil Agent. Pour détecter quand un sous-agent est invoqué, vérifiez les blocs `tool_use` où `name` est `"Agent"`. Les messages provenant du contexte d'un sous-agent incluent un champ `parent_tool_use_id`.

<Note>
  Le nom de l'outil a été renommé de `"Task"` à `"Agent"` dans Claude Code v2.1.63. Les versions actuelles du SDK émettent `"Agent"` dans les blocs `tool_use` mais utilisent toujours `"Task"` dans la liste des outils `system:init` et dans `result.permission_denials[].tool_name`. Vérifier les deux valeurs dans `block.name` assure la compatibilité entre les versions du SDK.
</Note>

La structure du message diffère entre les SDK. En Python, les blocs de contenu sont accessibles directement via `message.content`. En TypeScript, `SDKAssistantMessage` enveloppe le message de l'API Claude, donc le contenu est accessible via `message.message.content`.

Cet exemple itère à travers les messages en continu, enregistrant quand un sous-agent est invoqué et quand les messages suivants proviennent du contexte d'exécution de ce sous-agent.

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
  Reprise des sous-agents
</h2>

Vous pouvez reprendre un sous-agent pour continuer là où il s'est arrêté plutôt que de recommencer à zéro. Un sous-agent repris conserve son historique de conversation complet, y compris tous les appels d'outils précédents, les résultats et le raisonnement.

Lorsqu'un sous-agent se termine, le résultat de l'outil Agent inclut un bloc de texte contenant `agentId: <id>`. Les agents intégrés [`Explore` et `Plan`](/docs/fr/sub-agents#built-in-subagents) sont ponctuels et ne retournent pas d'`agentId`, donc utilisez un agent personnalisé ou `general-purpose` lorsque vous avez besoin de reprendre. Pour reprendre un sous-agent par programmation :

1. **Capturez l'ID de session** : Extrayez `session_id` des messages lors de la première requête
2. **Extrayez l'ID d'agent** : Analysez `agentId` du texte du résultat de l'outil Agent
3. **Reprenez la session** : Passez `resume: sessionId` dans les options de la deuxième requête, et incluez l'ID d'agent dans votre invite

<Note>
  Vous devez reprendre la même session pour accéder à la transcription du sous-agent. Chaque appel `query()` démarre une nouvelle session par défaut, donc passez `resume: sessionId` pour continuer dans la même session.

  Lorsque vous utilisez un agent personnalisé, passez la même définition d'agent dans le paramètre `agents` pour les deux requêtes.
</Note>

L'exemple ci-dessous définit un agent personnalisé `endpoint-finder`. La première requête l'exécute et capture l'ID de session et l'ID d'agent du résultat de l'outil Agent, puis la deuxième requête reprend la session pour poser une question de suivi qui nécessite le contexte de la première analyse.

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

Les transcriptions des sous-agents persistent indépendamment de la conversation principale :

* **Compaction de la conversation principale** : Lorsque la conversation principale se compacte, les transcriptions des sous-agents ne sont pas affectées. Elles sont stockées dans des fichiers séparés.
* **Persistance de la session** : Les transcriptions des sous-agents persistent au sein de leur session. Vous pouvez reprendre un sous-agent après avoir redémarré Claude Code en reprenant la même session.
* **Nettoyage automatique** : Les transcriptions sont nettoyées en fonction du paramètre `cleanupPeriodDays`, qui est défini par défaut à 30 jours.

<h2 id="tool-restrictions-2">
  Restrictions d'outils
</h2>

Les sous-agents peuvent avoir un accès aux outils restreint via le champ `tools` :

* **Omettez le champ** : l'agent hérite de tous les outils disponibles (par défaut)
* **Spécifiez les outils** : l'agent ne peut utiliser que les outils listés

Cet exemple crée un agent d'analyse en lecture seule qui peut examiner le code mais ne peut pas modifier les fichiers ou exécuter des commandes.

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

| Cas d'usage              | Outils                                  | Description                                                    |
| :----------------------- | :-------------------------------------- | :------------------------------------------------------------- |
| Analyse en lecture seule | `Read`, `Grep`, `Glob`                  | Peut examiner le code mais pas modifier ou exécuter            |
| Exécution de tests       | `Bash`, `Read`, `Grep`                  | Peut exécuter des commandes et analyser la sortie              |
| Modification de code     | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Accès complet en lecture/écriture sans exécution de commandes  |
| Accès complet            | Tous les outils                         | Hérite de tous les outils du parent (omettez le champ `tools`) |

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

* **Vérifiez que les invocations d'Agent sont approuvées** : incluez `Agent` dans `allowedTools` pour approuver automatiquement les appels de sous-agent. Sans cela, les invocations d'Agent passent par votre callback `canUseTool` ou, en mode `dontAsk`, sont refusées
* **Utilisez des invites explicites** : mentionnez le sous-agent par son nom dans votre invite, par exemple « Utilisez l'agent code-reviewer pour... »
* **Écrivez une description claire** : expliquez exactement quand utiliser le sous-agent pour que Claude puisse faire correspondre les tâches de manière appropriée

<h3 id="filesystem-based-agents-not-loading">
  Les agents basés sur le système de fichiers ne se chargent pas
</h3>

Claude Code surveille `~/.claude/agents/` et `.claude/agents/` et détecte un fichier d'agent nouveau ou modifié en quelques secondes, sans redémarrage nécessaire. Si une définition n'apparaît jamais, vérifiez ces causes :

* **Nouveau répertoire `agents`** : le moniteur couvre uniquement les répertoires qui existaient au démarrage de la session, donc le premier fichier dans un nouveau répertoire nécessite un redémarrage de session. C'est la cause la plus courante.
* **Frontmatter invalide ou `name` en doublon** : vérifiez le YAML du fichier et si un agent existant utilise déjà le `name`.
* **`--disable-slash-commands`** : les sessions démarrées avec ce flag ne surveillent pas ces répertoires et nécessitent toujours un redémarrage pour charger les nouveaux fichiers.
* **Un agent programmatique avec le même nom** : les `agents` passés à `query()` remplacent un agent du système de fichiers avec le même nom.

Pour le format de fichier, consultez [comment écrire des fichiers de sous-agent](/docs/fr/sub-agents#write-subagent-files).

<h3 id="long-prompt-failures-on-windows">
  Échecs d'invite longue sur Windows
</h3>

Sur Windows, les sous-agents avec des invites très longues peuvent échouer en raison de la limite de longueur de ligne de commande de 8191 caractères. Gardez les invites concises ou utilisez des agents basés sur le système de fichiers pour les instructions complexes.

<h2 id="related-documentation">
  Documentation connexe
</h2>

* [Sous-agents Claude Code](/docs/fr/sub-agents) : documentation complète des sous-agents incluant les définitions basées sur le système de fichiers
* [Flux de travail dynamiques](/docs/fr/workflows) : orchestrez de nombreux sous-agents à partir d'un script pour les tâches trop importantes pour une seule conversation
* [Aperçu du SDK](/docs/fr/agent-sdk/overview) : prise en main du Claude Agent SDK
