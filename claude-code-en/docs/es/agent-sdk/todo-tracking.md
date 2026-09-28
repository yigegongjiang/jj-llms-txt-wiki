> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rastrear tareas

> Rastrear tareas en sesiones del SDK del Agente y renderizar el progreso de Claude en su aplicación desde llamadas de herramientas estructuradas

Claude Code proporciona las [herramientas de seguimiento de tareas](/docs/es/tools-reference#task-tool-availability) de forma predeterminada solo en los modelos enumerados en [Disponibilidad de modelos](#model-availability). Los modelos más nuevos rastrean trabajo de múltiples pasos sin una lista de tareas escrita, por lo que en esos no necesita nada en esta página para que Claude trabaje en tareas de múltiples pasos.

En una sesión que tiene las herramientas de seguimiento de tareas, Claude mantiene una lista de tareas escrita, actualizando el estado de cada elemento mientras trabaja. Verá cada cambio en la secuencia de mensajes como una llamada de herramienta estructurada. Opte por una sesión solo cuando su aplicación lea esas llamadas de herramientas, ya sea para registrar la actividad de tareas o para renderizar su propia pantalla de progreso.

<h2 id="model-availability">
  Disponibilidad de modelos
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

En un modelo que no tiene las herramientas de forma predeterminada, a menos que opte por una sesión, no verá bloques `tool_use` para ellas en la secuencia de mensajes. El SDK del Agente aplica estos valores predeterminados a través del binario de Claude Code que incluye. Si apunta `pathToClaudeCodeExecutable` (TypeScript) o `cli_path` (Python) a su propia instalación de Claude Code, obtiene las herramientas que esa instalación proporciona, bajo sus propios valores predeterminados. Para ver el conjunto exacto en una sesión en ejecución, [verifique qué herramientas están disponibles](/docs/es/tools-reference#check-which-tools-are-available). Para optar por una sesión, haga uno de lo siguiente:

* Nombre una de las herramientas en la opción [`allowedTools`](/docs/es/agent-sdk/permissions#allow-and-deny-rules) (TypeScript) o `allowed_tools` (Python)
* Liste las herramientas en la opción `tools`, que restringe las herramientas integradas de la sesión a las que nombra. Incluya las herramientas que desea junto con las otras herramientas integradas que utiliza
* Establezca `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` en la opción `env`, como lo hacen los ejemplos en esta página. En TypeScript, `env` reemplaza el entorno del subproceso, así que extienda `...process.env` para mantener las variables heredadas. En Python, `env` se fusiona en la parte superior del entorno heredado

<h2 id="todo-lifecycle">
  Ciclo de vida de tareas
</h2>

Claude mueve cada tarea a través de un ciclo de vida predecible:

1. **Creada**: Claude añade la tarea como `pending` cuando identifica una tarea
2. **Activada**: Claude establece la tarea en `in_progress` cuando comienza el trabajo
3. **Completada**: Claude la marca como completada cuando la tarea finaliza exitosamente
4. **Eliminada**: Claude elimina una tarea que ya no necesita estableciendo `status: "deleted"` en una llamada `TaskUpdate`

<h2 id="when-claude-creates-todos">
  Cuándo Claude crea tareas
</h2>

En una [sesión que tiene las herramientas de seguimiento de tareas](#model-availability), Claude crea tareas para la mayoría del trabajo de múltiples pasos, como:

* **Tareas complejas de múltiples pasos** que requieren tres o más acciones distintas
* **Listas de tareas proporcionadas por el usuario** cuando se mencionan múltiples elementos
* **Operaciones más largas** que se benefician del seguimiento del progreso
* **Solicitudes explícitas** cuando los usuarios piden organización de tareas

Claude puede omitir tareas para solicitudes muy cortas o de un solo paso.

<h2 id="examples">
  Ejemplos
</h2>

Antes de ejecutar estos ejemplos, instale el Claude Agent SDK siguiendo el [inicio rápido](/docs/es/agent-sdk/quickstart). Cada ejemplo en esta página comparte la misma configuración de permisos y comportamiento de salida:

* **Modo de permiso**: los ejemplos de solicitud piden a Claude que haga trabajo real en un proyecto, así que cada ejemplo establece `permissionMode: "acceptEdits"` (TypeScript) o `permission_mode="acceptEdits"` (Python) para aprobar automáticamente las ediciones de archivo que produce el trabajo. Vea [Modos de permiso](/docs/es/agent-sdk/permissions#permission-modes) para las alternativas.
* **Límite de turnos**: cada ejemplo se ejecuta hasta que el agente termina y produce su mensaje de resultado final. Si una sesión alcanza primero su límite de turnos, ese mensaje de resultado tiene el subtipo `error_max_turns`. Verifique `subtype` para detectar ese final.
* **Manejo de errores**: estos ejemplos utilizan llamadas `query()` de un solo disparo. Después de producir un resultado `error_max_turns`, `query()` genera un error que incluye `Reached maximum number of turns`. Cada ejemplo envuelve su bucle en un bloque try para salir limpiamente cuando eso sucede. Vea [Manejar el resultado](/docs/es/agent-sdk/agent-loop#handle-the-result) para los subtipos de resultado.

<Note>
  Los mensajes del sistema de tareas, [`SDKTaskNotificationMessage`](/docs/es/agent-sdk/typescript#sdktasknotificationmessage) (TypeScript) o [`TaskNotificationMessage`](/docs/es/agent-sdk/python#tasknotificationmessage) (Python) entre ellos, reportan tareas de fondo como comandos en segundo plano y subagentes. En la secuencia de mensajes, verá la actividad de tareas como bloques `tool_use` en los mensajes del asistente.
</Note>

<h3 id="monitor-todo-changes">
  Monitorear cambios de tareas
</h3>

El siguiente ejemplo observa la secuencia del asistente para bloques `tool_use` de `TaskCreate` y `TaskUpdate` e imprime una línea `+` con el asunto de cada nueva tarea y una línea de actualización con el ID de tarea y el nuevo estado de cada cambio de estado. Use esta forma cuando desee un registro de actividad de tareas en lugar de una pantalla renderizada. Las líneas `+` no incluyen los IDs asignados, así que este registro no puede hacer coincidir las actualizaciones con sus creaciones. Para mantener esa correlación, capture los IDs como lo hace [Mostrar progreso en tiempo real](#display-progress-in-real-time).

La entrada `tool_use` transmitida es la forma bruta que emitió el modelo. Claude Code repara algunos nombres de clave casi correctos pero incorrectos antes de la ejecución, asignando `id` o `task_id` a `taskId` y `active_form` a `activeForm`, pero esa reparación no se refleja en la secuencia. Lea los campos de entrada de `TaskUpdate` defensivamente, como lo hacen ambos ejemplos en esta página, en lugar de asumir que el nombre canónico siempre está presente.

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
  Mostrar progreso en tiempo real
</h3>

El siguiente ejemplo observa la secuencia del asistente para bloques `tool_use` de `TaskCreate` y `TaskUpdate` y mantiene un mapa de tareas codificado por ID de tarea en una clase `TaskTracker`, rerenderizando un resumen de progreso en cada cambio. El resumen cuenta tareas completadas y en progreso y muestra la etiqueta `activeForm` de cada elemento activo en lugar de su `subject`. Use esta forma cuando su aplicación mantenga una pantalla de progreso en lugar de registrar cada evento.

El ID de tarea asignado no está en la entrada de `TaskCreate`. Claude Code entrega la salida estructurada de cada herramienta en el mensaje del usuario que lleva su bloque `tool_result`, en el campo `tool_use_result`. Para `TaskCreate`, ese objeto se documenta para TypeScript como `TaskCreateOutput` en [Tipos de Salida de Herramientas](/docs/es/agent-sdk/typescript#tool-output-types), y en Python el campo es un dict simple de la misma forma. El rastreador empareja cada bloque `tool_result` con su llamada `tool_use` por `tool_use_id` y lee `task.id` del `tool_use_result` del mensaje emparejado. Claude puede leer la lista de vuelta con `TaskList` y los detalles completos de una tarea con `TaskGet`.

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
  Documentación relacionada
</h2>

* [Referencia del SDK del Agente - TypeScript](/docs/es/agent-sdk/typescript): las opciones, tipos y esquemas de herramientas para el SDK de TypeScript, incluidos los tipos de entrada y salida de la herramienta Task
* [Referencia del SDK del Agente - Python](/docs/es/agent-sdk/python): las opciones, tipos y documentación de herramientas para el SDK de Python
* [Entrada de Streaming](/docs/es/agent-sdk/streaming-vs-single-mode): los dos modos de entrada, y cuándo usar entrada de streaming en lugar de las llamadas de un solo disparo que utilizan estos ejemplos
* [Dar a Claude herramientas personalizadas](/docs/es/agent-sdk/custom-tools): defina sus propias herramientas con el servidor MCP en proceso del SDK
