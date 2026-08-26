> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Listas de Tareas

> Rastrear y mostrar tareas pendientes utilizando el SDK del Agente Claude para la gestión organizada de tareas

El seguimiento de tareas proporciona una forma estructurada de gestionar tareas y mostrar el progreso a los usuarios. El SDK del Agente Claude incluye funcionalidad de tareas integrada que ayuda a organizar flujos de trabajo complejos y mantener a los usuarios informados sobre la progresión de las tareas.

<Note>
  A partir del TypeScript Agent SDK 0.3.142 y Claude Code v2.1.142, las sesiones utilizan las herramientas Task estructuradas `TaskCreate`, `TaskUpdate`, `TaskGet` y `TaskList` en lugar de `TodoWrite`. El SDK de Python obtiene este cambio de la CLI de Claude Code que lanza, no de la versión del paquete de Python: el cambio se aplica una vez que esa CLI — la copia incluida dentro del paquete pip, o una a la que apunte con `cli_path` — sea v2.1.142 o posterior. Consulte [Migrar a herramientas Task](#migrate-to-task-tools) para ver cómo cambia el código de monitoreo. Los ejemplos en esta página establecen `CLAUDE_CODE_ENABLE_TASKS=0` para seguir mostrando `TodoWrite` para sesiones que aún no han migrado.
</Note>

<h3 id="todo-lifecycle">
  Ciclo de Vida de las Tareas
</h3>

Las tareas siguen un ciclo de vida predecible:

1. **Creadas** como `pending` cuando se identifican las tareas
2. **Activadas** a `in_progress` cuando comienza el trabajo
3. **Completadas** cuando la tarea finaliza exitosamente
4. **Eliminadas** cuando todas las tareas en un grupo se completan

<h3 id="when-todos-are-used">
  Cuándo se Utilizan las Tareas
</h3>

El SDK crea tareas para la mayoría del trabajo de múltiples pasos, como:

* **Tareas complejas de múltiples pasos** que requieren 3 o más acciones distintas
* **Listas de tareas proporcionadas por el usuario** cuando se mencionan múltiples elementos
* **Operaciones no triviales** que se benefician del seguimiento del progreso
* **Solicitudes explícitas** cuando los usuarios piden organización de tareas

Puede omitir tareas para solicitudes muy cortas o de un solo paso.

<h2 id="examples">
  Ejemplos
</h2>

Antes de ejecutar estos ejemplos, instale el Claude Agent SDK siguiendo el [inicio rápido](/docs/es/agent-sdk/quickstart).

Cada ejemplo se ejecuta hasta que el agente termina y produce su mensaje de resultado final. Si una sesión alcanza primero su límite de turnos, ese mensaje de resultado tiene el subtipo `error_max_turns`. Verifique `subtype` para detectar ese final.

Estos ejemplos utilizan llamadas `query()` de un solo disparo. Después de producir un resultado `error_max_turns`, `query()` genera un error que incluye `Reached maximum number of turns`. Cada ejemplo envuelve su bucle en un bloque try para salir limpiamente cuando eso sucede.

Consulte [Manejar el resultado](/docs/es/agent-sdk/agent-loop#handle-the-result) para los subtipos de resultado.

<h3 id="monitoring-todo-changes">
  Monitoreo de Cambios en Tareas
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
  Visualización de Progreso en Tiempo Real
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
  Migrar a herramientas Task
</h2>

Las herramientas Task dividen la única llamada `TodoWrite` en `TaskCreate` para cada elemento nuevo y `TaskUpdate` para cada cambio de estado, con `TaskList` y `TaskGet` disponibles para que el modelo lea la lista actual. Su código de monitoreo aún inspecciona bloques `tool_use` en la secuencia del asistente, pero mantiene un mapa codificado por ID de tarea en lugar de reemplazar la lista completa en cada llamada. Las herramientas Task son las predeterminadas a partir del TypeScript Agent SDK 0.3.142 y Claude Code v2.1.142, por lo que no se necesita cambio en `options.env`.

| Con `TodoWrite`                                                | Con herramientas Task                                                                                                                                                                                                                                                                                                  |
| -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Una llamada de herramienta reescribe el array `todos` completo | `TaskCreate` añade un elemento, `TaskUpdate` parcha un elemento por `taskId`                                                                                                                                                                                                                                           |
| Coincide con `block.name === "TodoWrite"`                      | Coincide con `block.name === "TaskCreate"` o `"TaskUpdate"`                                                                                                                                                                                                                                                            |
| Forma del elemento: `{ content, status, activeForm }`          | Entrada de `TaskCreate`: `{ subject, description, activeForm?, metadata? }`. Entrada de `TaskUpdate`: `{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`. `status` es `"pending"`, `"in_progress"` o `"completed"`; establezca `status: "deleted"` para eliminar |
| Renderice `block.input.todos` directamente                     | Acumule elementos entre llamadas, o lea una instantánea de un resultado de herramienta `TaskList`                                                                                                                                                                                                                      |

El ID de tarea asignado no está en la entrada de `TaskCreate`. Vuelve en el bloque `tool_result` coincidente como `{ task: { id, subject } }`, así que capturelo del bloque de resultado para codificar su mapa. El siguiente ejemplo muestra el cambio mínimo al bucle [Monitoreo de Cambios en Tareas](#monitoring-todo-changes). Lee solo entradas de `tool_use` y omite capturar IDs de bloques `tool_result`. Para renderizar una lista completa, observe un resultado de herramienta `TaskList` en la secuencia o acumule resultados de `TaskCreate` e entradas de `TaskUpdate` en un mapa.

La entrada `tool_use` transmitida es la forma bruta que emitió el modelo. Claude Code repara algunos nombres de clave casi correctos pero incorrectos antes de la ejecución, asignando `id` o `task_id` a `taskId` y `active_form` a `activeForm`, pero esa reparación no se refleja en la secuencia. Lea los campos de entrada de `TaskUpdate` defensivamente, como lo hacen los ejemplos a continuación, en lugar de asumir que el nombre canónico siempre está presente.

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
  Documentación Relacionada
</h2>

* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript)
* [Referencia del SDK de Python](/docs/es/agent-sdk/python)
* [Modo de Streaming vs Modo Único](/docs/es/agent-sdk/streaming-vs-single-mode)
* [Herramientas Personalizadas](/docs/es/agent-sdk/custom-tools)
