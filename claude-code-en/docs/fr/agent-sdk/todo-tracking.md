> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Listes de tâches

> Suivre et afficher les tâches à l'aide du SDK Claude Agent pour une gestion organisée des tâches

Le suivi des tâches fournit un moyen structuré de gérer les tâches et d'afficher la progression aux utilisateurs. Le SDK Claude Agent inclut une fonctionnalité de tâches intégrée qui aide à organiser les flux de travail complexes et à tenir les utilisateurs informés de la progression des tâches.

<Note>
  À partir du TypeScript Agent SDK 0.3.142 et Claude Code v2.1.142, les sessions utilisent les outils Task structurés `TaskCreate`, `TaskUpdate`, `TaskGet` et `TaskList` à la place de `TodoWrite`. Le SDK Python obtient ce changement à partir de la CLI Claude Code qu'il lance, et non à partir de la version du package Python : le changement s'applique une fois que cette CLI — la copie fournie dans le package pip, ou celle vers laquelle vous pointez avec `cli_path` — est v2.1.142 ou ultérieure. Consultez [Migrer vers les outils Task](#migrate-to-task-tools) pour savoir comment le code de surveillance change. Les exemples de cette page définissent `CLAUDE_CODE_ENABLE_TASKS=0` pour continuer à afficher `TodoWrite` pour les sessions qui n'ont pas encore migré.
</Note>

<h3 id="todo-lifecycle">
  Cycle de vie des tâches
</h3>

Les tâches suivent un cycle de vie prévisible :

1. **Créées** en tant que `pending` lorsque les tâches sont identifiées
2. **Activées** en tant que `in_progress` lorsque le travail commence
3. **Complétées** lorsque la tâche se termine avec succès
4. **Supprimées** lorsque toutes les tâches d'un groupe sont complétées

<h3 id="when-todos-are-used">
  Quand les tâches sont utilisées
</h3>

Le SDK crée des tâches pour la plupart des travaux multi-étapes, tels que :

* **Les tâches complexes multi-étapes** nécessitant 3 actions distinctes ou plus
* **Les listes de tâches fournies par l'utilisateur** lorsque plusieurs éléments sont mentionnés
* **Les opérations non triviales** qui bénéficient du suivi de la progression
* **Les demandes explicites** lorsque les utilisateurs demandent une organisation des tâches

Il peut ignorer les tâches pour les demandes très courtes ou à une seule étape.

<h2 id="examples">
  Exemples
</h2>

Avant d'exécuter ces exemples, installez le Claude Agent SDK en suivant le [démarrage rapide](/docs/fr/agent-sdk/quickstart).

Chaque exemple s'exécute jusqu'à ce que l'agent se termine et produise son message de résultat final. Si une session atteint d'abord sa limite de tours, ce message de résultat a le sous-type `error_max_turns`. Vérifiez `subtype` pour détecter cette fin.

Ces exemples utilisent des appels `query()` uniques. Après avoir produit un résultat `error_max_turns`, `query()` lève une erreur qui inclut `Reached maximum number of turns`. Chaque exemple enveloppe sa boucle dans un bloc try pour quitter proprement quand cela se produit.

Voir [Gérer le résultat](/docs/fr/agent-sdk/agent-loop#handle-the-result) pour les sous-types de résultat.

<h3 id="monitoring-todo-changes">
  Surveillance des modifications des tâches
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      // Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
      // Task tools instead and these tool_use blocks never appear.
      options: { maxTurns: 15, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
    })) {
      // Todo updates are reflected in the message stream
      if (message.type === "assistant") {
        for (const block of message.message.content) {
          if (block.type === "tool_use" && block.name === "TodoWrite") {
            const todos = block.input.todos;

            console.log("Todo Status Update:");
            todos.forEach((todo, index) => {
              const status =
                todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
              console.log(`${index + 1}. ${status} ${todo.content}`);
            });
          }
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // such as when the maxTurns limit is hit.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock


  async def main():
      try:
          async for message in query(
              prompt="Optimize my React app performance and track progress with todos",
              # Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
              # Task tools instead and these tool_use blocks never appear.
              options=ClaudeAgentOptions(max_turns=15, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
          ):
              # Todo updates are reflected in the message stream
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                          todos = block.input["todos"]

                          print("Todo Status Update:")
                          for i, todo in enumerate(todos):
                              status = (
   "✅"
   if todo["status"] == "completed"
   else "🔧"
   if todo["status"] == "in_progress"
   else "❌"
                              )
                              print(f"{i + 1}. {status} {todo['content']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # such as when the max_turns limit is hit.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="real-time-progress-display">
  Affichage de la progression en temps réel
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  class TodoTracker {
    private todos: any[] = [];

    displayProgress() {
      if (this.todos.length === 0) return;

      const completed = this.todos.filter((t) => t.status === "completed").length;
      const inProgress = this.todos.filter((t) => t.status === "in_progress").length;
      const total = this.todos.length;

      console.log(`\nProgress: ${completed}/${total} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      this.todos.forEach((todo, index) => {
        const icon =
          todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
        const text = todo.status === "in_progress" ? todo.activeForm : todo.content;
        console.log(`${index + 1}. ${icon} ${text}`);
      });
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          // Re-enable TodoWrite, which this tracker watches for.
          options: { maxTurns: 20, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use" && block.name === "TodoWrite") {
                this.todos = block.input.todos;
                this.displayProgress();
              }
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
  const tracker = new TodoTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
  from typing import List, Dict


  class TodoTracker:
      def __init__(self):
          self.todos: List[Dict] = []

      def display_progress(self):
          if not self.todos:
              return

          completed = len([t for t in self.todos if t["status"] == "completed"])
          in_progress = len([t for t in self.todos if t["status"] == "in_progress"])
          total = len(self.todos)

          print(f"\nProgress: {completed}/{total} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for i, todo in enumerate(self.todos):
              icon = (
                  "✅"
                  if todo["status"] == "completed"
                  else "🔧"
                  if todo["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  todo["activeForm"]
                  if todo["status"] == "in_progress"
                  else todo["content"]
              )
              print(f"{i + 1}. {icon} {text}")

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  # Re-enable TodoWrite, which this tracker watches for.
                  options=ClaudeAgentOptions(max_turns=20, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                              self.todos = block.input["todos"]
                              self.display_progress()
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TodoTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="migrate-to-task-tools">
  Migrer vers les outils Task
</h2>

Les outils Task divisent l'appel unique `TodoWrite` en `TaskCreate` pour chaque nouvel élément et `TaskUpdate` pour chaque changement de statut, avec `TaskList` et `TaskGet` disponibles pour que le modèle relise la liste actuelle. Votre code de surveillance inspecte toujours les blocs `tool_use` dans le flux assistant, mais maintient une carte indexée par ID de tâche au lieu de remplacer la liste entière à chaque appel. Les outils Task sont le défaut à partir du TypeScript Agent SDK 0.3.142 et Claude Code v2.1.142, donc aucun changement `options.env` n'est nécessaire.

| Avec `TodoWrite`                                    | Avec les outils Task                                                                                                                                                                                                                                                                                                 |
| --------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Un appel d'outil réécrit le tableau `todos` complet | `TaskCreate` ajoute un élément, `TaskUpdate` corrige un élément par `taskId`                                                                                                                                                                                                                                         |
| Correspond à `block.name === "TodoWrite"`           | Correspond à `block.name === "TaskCreate"` ou `"TaskUpdate"`                                                                                                                                                                                                                                                         |
| Forme d'élément : `{ content, status, activeForm }` | Entrée `TaskCreate` : `{ subject, description, activeForm?, metadata? }`. Entrée `TaskUpdate` : `{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`. `status` est `"pending"`, `"in_progress"` ou `"completed"` ; définissez `status: "deleted"` pour supprimer |
| Rendre `block.input.todos` directement              | Accumuler les éléments entre les appels, ou lire un instantané à partir d'un résultat d'outil `TaskList`                                                                                                                                                                                                             |

L'ID de tâche assigné ne se trouve pas dans l'entrée `TaskCreate`. Il revient dans le bloc `tool_result` correspondant sous la forme `{ task: { id, subject } }`, donc capturez-le à partir du bloc de résultat pour indexer votre carte. L'exemple suivant montre le changement minimal à la boucle [Surveillance des modifications des tâches](#monitoring-todo-changes). Il lit uniquement les entrées `tool_use` et ignore la capture des ID à partir des blocs `tool_result`. Pour rendre une liste complète, regardez un résultat d'outil `TaskList` dans le flux ou accumulez les résultats `TaskCreate` et les entrées `TaskUpdate` dans une carte.

Le flux `tool_use` d'entrée est la forme brute que le modèle a émise. Claude Code répare certains noms de clés proches mais incorrects avant l'exécution, en mappant `id` ou `task_id` à `taskId` et `active_form` à `activeForm`, mais cette réparation n'est pas reflétée dans le flux. Lisez les champs d'entrée `TaskUpdate` de manière défensive, comme le font les exemples ci-dessous, plutôt que de supposer que le nom canonique est toujours présent.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      options: { maxTurns: 15 },
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
              prompt="Optimize my React app performance and track progress with todos",
              options=ClaudeAgentOptions(max_turns=15),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input['subject']}")
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

<h2 id="related-documentation">
  Documentation connexe
</h2>

* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript)
* [Référence du SDK Python](/docs/fr/agent-sdk/python)
* [Mode streaming vs mode unique](/docs/fr/agent-sdk/streaming-vs-single-mode)
* [Outils personnalisés](/docs/fr/agent-sdk/custom-tools)
