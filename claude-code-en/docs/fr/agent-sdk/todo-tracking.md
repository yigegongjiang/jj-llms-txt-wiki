> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Suivre les tâches

> Suivre les tâches dans les sessions du SDK Agent et afficher la progression de Claude dans votre application à partir d'appels d'outils structurés

Claude Code fournit les [outils de suivi des tâches](/docs/fr/tools-reference#task-tool-availability) par défaut uniquement sur les modèles listés sous [Disponibilité des modèles](#model-availability). Les modèles plus récents suivent les travaux multi-étapes sans liste de tâches écrite, donc sur ceux-ci vous n'avez besoin de rien sur cette page pour que Claude travaille sur des tâches multi-étapes.

Dans une session qui dispose des outils de suivi des tâches, Claude maintient une liste de tâches écrite, mettant à jour le statut de chaque élément au fur et à mesure qu'il travaille. Vous voyez chaque changement dans le flux de messages sous forme d'appel d'outil structuré. Optez pour une session uniquement lorsque votre application lit ces appels d'outils, que ce soit pour enregistrer l'activité des tâches ou pour afficher son propre écran de progression.

<h2 id="model-availability">
  Disponibilité des modèles
</h2>

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.
</Note>

Sur un modèle qui n'a pas les outils par défaut, sauf si vous optez pour une session, vous ne voyez aucun bloc `tool_use` pour eux dans le flux de messages. Le SDK Agent applique ces paramètres par défaut via le binaire Claude Code qu'il regroupe. Si vous pointez `pathToClaudeCodeExecutable` (TypeScript) ou `cli_path` (Python) vers votre propre installation de Claude Code, vous obtenez les outils que cette installation fournit, selon ses propres paramètres par défaut. Pour voir l'ensemble exact dans une session en cours d'exécution, [vérifiez quels outils sont disponibles](/docs/fr/tools-reference#check-which-tools-are-available). Pour opter pour une session, faites l'une des choses suivantes :

* Nommez l'un des outils dans l'option [`allowedTools`](/docs/fr/agent-sdk/permissions#allow-and-deny-rules) (TypeScript) ou `allowed_tools` (Python)
* Listez les outils dans l'option `tools`, qui restreint les outils intégrés de la session à ceux qu'elle nomme. Incluez les outils que vous souhaitez aux côtés des autres outils intégrés que vous utilisez
* Définissez `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` dans l'option `env`, comme le font les exemples sur cette page. En TypeScript, `env` remplace l'environnement du sous-processus, donc propagez `...process.env` pour conserver les variables héritées. En Python, `env` est fusionné au-dessus de l'environnement hérité

<h2 id="todo-lifecycle">
  Cycle de vie des tâches
</h2>

Claude déplace chaque tâche à travers un cycle de vie prévisible :

1. **Créée** : Claude ajoute la tâche en tant que `pending` lorsqu'il identifie une tâche
2. **Activée** : Claude définit la tâche à `in_progress` lorsqu'il commence le travail
3. **Complétée** : Claude la marque comme complétée lorsque la tâche se termine avec succès
4. **Supprimée** : Claude supprime une tâche dont il n'a plus besoin en définissant `status: "deleted"` dans un appel `TaskUpdate`

<h2 id="when-claude-creates-todos">
  Quand Claude crée des tâches
</h2>

Dans une [session qui dispose des outils de suivi des tâches](#model-availability), Claude crée des tâches pour la plupart des travaux multi-étapes, tels que :

* **Les tâches complexes multi-étapes** nécessitant trois actions distinctes ou plus
* **Les listes de tâches fournies par l'utilisateur** lorsque plusieurs éléments sont mentionnés
* **Les opérations plus longues** qui bénéficient du suivi de la progression
* **Les demandes explicites** lorsque les utilisateurs demandent une organisation des tâches

Claude peut ignorer les tâches pour les demandes très courtes ou à une seule étape.

<h2 id="examples">
  Exemples
</h2>

Avant d'exécuter ces exemples, installez le SDK Claude Agent en suivant le [démarrage rapide](/docs/fr/agent-sdk/quickstart). Chaque exemple sur cette page partage la même configuration de permissions et le même comportement de sortie :

* **Mode de permission** : les exemples de prompt demandent à Claude de faire un vrai travail sur un projet, donc chaque exemple définit `permissionMode: "acceptEdits"` (TypeScript) ou `permission_mode="acceptEdits"` (Python) pour approuver automatiquement les modifications de fichiers que le travail produit. Voir [Modes de permission](/docs/fr/agent-sdk/permissions#permission-modes) pour les alternatives.
* **Limite de tours** : chaque exemple s'exécute jusqu'à ce que l'agent se termine et produise son message de résultat final. Si une session atteint d'abord sa limite de tours, ce message de résultat a le sous-type `error_max_turns`. Vérifiez `subtype` pour détecter cette fin.
* **Gestion des erreurs** : ces exemples utilisent des appels `query()` uniques. Après avoir produit un résultat `error_max_turns`, `query()` lève une erreur qui inclut `Reached maximum number of turns`. Chaque exemple enveloppe sa boucle dans un bloc try pour quitter proprement quand cela se produit. Voir [Gérer le résultat](/docs/fr/agent-sdk/agent-loop#handle-the-result) pour les sous-types de résultat.

<Note>
  Les messages système du système de tâches, [`SDKTaskNotificationMessage`](/docs/fr/agent-sdk/typescript#sdktasknotificationmessage) (TypeScript) ou [`TaskNotificationMessage`](/docs/fr/agent-sdk/python#tasknotificationmessage) (Python) parmi eux, rapportent les tâches en arrière-plan telles que les commandes en arrière-plan et les sous-agents. Dans le flux de messages, vous voyez l'activité des tâches sous forme de blocs `tool_use` dans les messages de l'assistant.
</Note>

<h3 id="monitor-todo-changes">
  Surveiller les modifications des tâches
</h3>

L'exemple suivant regarde le flux de l'assistant pour les blocs `tool_use` `TaskCreate` et `TaskUpdate` et imprime une ligne `+` avec le sujet de chaque nouvelle tâche et une ligne de mise à jour avec l'ID de la tâche et le nouveau statut de chaque changement de statut. Utilisez cette forme lorsque vous souhaitez un journal de l'activité des tâches plutôt qu'un affichage rendu. Les lignes `+` n'incluent pas les ID assignés, donc ce journal ne peut pas faire correspondre les mises à jour à leurs créations. Pour conserver cette corrélation, capturez les ID comme le fait [Afficher la progression en temps réel](#display-progress-in-real-time).

L'entrée `tool_use` diffusée est la forme brute que le modèle a émise. Claude Code répare certains noms de clés proches mais incorrects avant l'exécution, en mappant `id` ou `task_id` à `taskId` et `active_form` à `activeForm`, mais cette réparation n'est pas reflétée dans le flux. Lisez les champs d'entrée `TaskUpdate` de manière défensive, comme le font les deux exemples sur cette page, plutôt que de supposer que le nom canonique est toujours présent.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
      // Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
      options: { maxTurns: 15, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
    })) {
      if (message.type !== "assistant") continue;
      for (const block of message.message.content) {
        if (block.type !== "tool_use") continue;
        if (block.name === "TaskCreate") {
          const input = block.input as { subject: string };
          console.log(`+ ${input.subject}`);
        } else if (block.name === "TaskUpdate") {
          const input = block.input as {
            taskId?: string;
            id?: string;
            task_id?: string;
            status?: string;
          };
          const taskId = input.taskId ?? input.id ?? input.task_id;
          if (taskId && input.status) console.log(`  ${taskId} -> ${input.status}`);
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock

  async def main():
      try:
          async for message in query(
              prompt="Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
              # Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
              options=ClaudeAgentOptions(max_turns=15, permission_mode="acceptEdits", env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"}),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input.get('subject', '')}")
                  elif block.name == "TaskUpdate" and block.input.get("status"):
                      task_id = (
                          block.input.get("taskId")
                          or block.input.get("id")
                          or block.input.get("task_id")
                      )
                      if task_id:
                          print(f"  {task_id} -> {block.input['status']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="display-progress-in-real-time">
  Afficher la progression en temps réel
</h3>

L'exemple suivant regarde le flux de l'assistant pour les blocs `tool_use` `TaskCreate` et `TaskUpdate` et maintient une carte de tâches indexée par ID de tâche dans une classe `TaskTracker`, en réaffichant un résumé de progression à chaque changement. Le résumé compte les tâches complétées et en cours et affiche l'étiquette `activeForm` de chaque élément actif à la place de son `subject`. Utilisez cette forme lorsque votre application maintient un affichage de progression au lieu d'enregistrer chaque événement.

L'ID de tâche assigné ne se trouve pas dans l'entrée `TaskCreate`. Claude Code fournit la sortie structurée de chaque outil sur le message utilisateur qui porte son bloc `tool_result`, dans le champ `tool_use_result`. Pour `TaskCreate`, cet objet est documenté pour TypeScript comme `TaskCreateOutput` sous [Types de sortie d'outil](/docs/fr/agent-sdk/typescript#tool-output-types), et en Python le champ est un dict simple de la même forme. Le tracker apparie chaque bloc `tool_result` avec son appel `tool_use` par `tool_use_id` et lit `task.id` du `tool_use_result` du message appairé. Claude peut relire la liste avec `TaskList` et les détails complets d'une tâche avec `TaskGet`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  type Task = { subject: string; activeForm?: string; status: string };

  class TaskTracker {
    private tasks = new Map<string, Task>();
    private pendingCreates = new Map<string, { subject: string; activeForm?: string }>();

    displayProgress() {
      if (this.tasks.size === 0) {
        console.log("\nProgress: no open tasks\n");
        return;
      }

      const items = [...this.tasks.values()];
      const completed = items.filter((t) => t.status === "completed").length;
      const inProgress = items.filter((t) => t.status === "in_progress").length;

      console.log(`\nProgress: ${completed}/${this.tasks.size} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      for (const [id, task] of this.tasks) {
        const icon =
          task.status === "completed" ? "✅" : task.status === "in_progress" ? "🔧" : "❌";
        const text = task.status === "in_progress" && task.activeForm ? task.activeForm : task.subject;
        console.log(`${id}. ${icon} ${text}`);
      }
    }

    handleToolUse(block: { id: string; name: string; input: unknown }) {
      if (block.name === "TaskCreate") {
        const input = block.input as { subject: string; activeForm?: string; active_form?: string };
        this.pendingCreates.set(block.id, {
          subject: input.subject,
          activeForm: input.activeForm ?? input.active_form,
        });
      } else if (block.name === "TaskUpdate") {
        const input = block.input as {
          taskId?: string;
          id?: string;
          task_id?: string;
          status?: string;
          activeForm?: string;
          active_form?: string;
        };
        const taskId = input.taskId ?? input.id ?? input.task_id;
        if (!taskId) return;
        if (input.status === "deleted") {
          this.tasks.delete(taskId);
          this.displayProgress();
          return;
        }
        const task = this.tasks.get(taskId);
        if (!task) return;
        if (input.status) task.status = input.status;
        const active = input.activeForm ?? input.active_form;
        if (active) task.activeForm = active;
        this.displayProgress();
      }
    }

    handleToolResult(block: { tool_use_id: string; is_error?: boolean }, result: unknown) {
      const create = this.pendingCreates.get(block.tool_use_id);
      if (!create) return;
      this.pendingCreates.delete(block.tool_use_id);
      if (block.is_error) return;
      // The result's user message carries the tool's structured output as
      // tool_use_result; for TaskCreate that's TaskCreateOutput,
      // { task: { id, subject } }.
      const out = result as { task?: { id: string } };
      if (!out?.task?.id) return;
      this.tasks.set(out.task.id, { ...create, status: "pending" });
      this.displayProgress();
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          options: { maxTurns: 20, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use") this.handleToolUse(block);
            }
          }
          if (message.type === "user" && Array.isArray(message.message.content)) {
            for (const block of message.message.content) {
              if (block.type === "tool_result") this.handleToolResult(block, message.tool_use_result);
            }
          }
        }
      } catch (error) {
        // A single-shot query() throws after yielding an error result,
        // such as when the maxTurns limit is hit.
        console.log(`Session ended with an error: ${error}`);
      }
    }
  }

  // Usage
  const tracker = new TaskTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      AssistantMessage,
      UserMessage,
      ToolUseBlock,
      ToolResultBlock,
  )


  class TaskTracker:
      def __init__(self):
          self.tasks: dict[str, dict] = {}
          self.pending_creates: dict[str, dict] = {}

      def display_progress(self):
          if not self.tasks:
              print("\nProgress: no open tasks\n")
              return

          completed = len([t for t in self.tasks.values() if t["status"] == "completed"])
          in_progress = len([t for t in self.tasks.values() if t["status"] == "in_progress"])

          print(f"\nProgress: {completed}/{len(self.tasks)} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for task_id, task in self.tasks.items():
              icon = (
                  "✅"
                  if task["status"] == "completed"
                  else "🔧"
                  if task["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  task["activeForm"]
                  if task["status"] == "in_progress" and task.get("activeForm")
                  else task["subject"]
              )
              print(f"{task_id}. {icon} {text}")

      def handle_tool_use(self, block: ToolUseBlock):
          if block.name == "TaskCreate":
              self.pending_creates[block.id] = {
                  "subject": block.input.get("subject", ""),
                  "activeForm": block.input.get("activeForm") or block.input.get("active_form"),
              }
          elif block.name == "TaskUpdate":
              task_id = (
                  block.input.get("taskId")
                  or block.input.get("id")
                  or block.input.get("task_id")
              )
              if not task_id:
                  return
              if block.input.get("status") == "deleted":
                  self.tasks.pop(task_id, None)
                  self.display_progress()
                  return
              task = self.tasks.get(task_id)
              if not task:
                  return
              if block.input.get("status"):
                  task["status"] = block.input["status"]
              active = block.input.get("activeForm") or block.input.get("active_form")
              if active:
                  task["activeForm"] = active
              self.display_progress()

      def handle_tool_result(self, block: ToolResultBlock, tool_use_result):
          create = self.pending_creates.pop(block.tool_use_id, None)
          if create is None or block.is_error:
              return
          # The result's user message carries the tool's structured output as
          # tool_use_result; for TaskCreate that's {"task": {"id": ..., "subject": ...}}.
          task = (tool_use_result or {}).get("task") or {}
          if not task.get("id"):
              return
          self.tasks[task["id"]] = {**create, "status": "pending"}
          self.display_progress()

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  options=ClaudeAgentOptions(
                      max_turns=20,
                      permission_mode="acceptEdits",
                      env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"},
                  ),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              self.handle_tool_use(block)
                  if isinstance(message, UserMessage) and isinstance(message.content, list):
                      for block in message.content:
                          if isinstance(block, ToolResultBlock):
                              self.handle_tool_result(block, message.tool_use_result)
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TaskTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="related-documentation">
  Documentation connexe
</h2>

* [Référence du SDK Agent - TypeScript](/docs/fr/agent-sdk/typescript) : les options, les types et les schémas d'outils pour le SDK TypeScript, y compris les types d'entrée et de sortie de l'outil Task
* [Référence du SDK Agent - Python](/docs/fr/agent-sdk/python) : les options, les types et la documentation des outils pour le SDK Python
* [Entrée en streaming](/docs/fr/agent-sdk/streaming-vs-single-mode) : les deux modes d'entrée, et quand utiliser l'entrée en streaming au lieu des appels uniques que ces exemples utilisent
* [Donner à Claude des outils personnalisés](/docs/fr/agent-sdk/custom-tools) : définissez vos propres outils avec le serveur MCP en processus du SDK
