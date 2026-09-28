> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Todos verfolgen

> Verfolgen Sie Todos in Agent SDK-Sitzungen und rendern Sie Claudes Fortschritt in Ihrer Anwendung aus strukturierten Tool-Aufrufen

Claude Code stellt die [Task-Tracking-Tools](/docs/de/tools-reference#task-tool-availability) standardmäßig nur auf den unter [Modellverfügbarkeit](#model-availability) aufgelisteten Modellen bereit. Neuere Modelle verfolgen mehrstufige Arbeiten ohne eine schriftliche Todo-Liste, daher benötigen Sie auf diesen nichts auf dieser Seite, damit Claude mehrstufige Aufgaben durcharbeitet.

In einer Sitzung, die die Task-Tracking-Tools hat, führt Claude eine schriftliche Todo-Liste, aktualisiert den Status jedes Elements während der Arbeit. Sie sehen jede Änderung im Nachrichtenstrom als strukturierten Tool-Aufruf. Aktivieren Sie eine Sitzung nur, wenn Ihre Anwendung diese Tool-Aufrufe liest, sei es zum Protokollieren von Task-Aktivitäten oder zum Rendern einer eigenen Fortschrittsanzeige.

<h2 id="model-availability">
  Modellverfügbarkeit
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

Bei einem Modell, das die Tools standardmäßig nicht hat, sehen Sie keine `tool_use`-Blöcke für diese im Nachrichtenstrom, es sei denn, Sie aktivieren eine Sitzung. Das Agent SDK wendet diese Standardeinstellungen über die Claude Code-Binärdatei an, die es bündelt. Wenn Sie `pathToClaudeCodeExecutable` (TypeScript) oder `cli_path` (Python) auf Ihre eigene Claude Code-Installation verweisen, erhalten Sie die Tools, die diese Installation bereitstellt, unter ihren eigenen Standardeinstellungen. Um die genaue Menge in einer laufenden Sitzung zu sehen, [überprüfen Sie, welche Tools verfügbar sind](/docs/de/tools-reference#check-which-tools-are-available). Um eine Sitzung zu aktivieren, führen Sie eines der folgenden Verfahren durch:

* Nennen Sie eines der Tools in der Option [`allowedTools`](/docs/de/agent-sdk/permissions#allow-and-deny-rules) (TypeScript) oder `allowed_tools` (Python)
* Listen Sie die Tools in der Option `tools` auf, die die integrierten Tools der Sitzung auf die beschriebenen beschränkt. Fügen Sie die gewünschten Tools neben den anderen integrierten Tools ein, die Sie verwenden
* Setzen Sie `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` in der Option `env`, wie die Beispiele auf dieser Seite. In TypeScript ersetzt `env` die Subprocess-Umgebung, daher verteilen Sie `...process.env`, um vererbte Variablen zu behalten. In Python wird `env` auf die vererbte Umgebung zusammengeführt

<h2 id="todo-lifecycle">
  Todo-Lebenszyklus
</h2>

Claude bewegt jedes Todo durch einen vorhersehbaren Lebenszyklus:

1. **Erstellt**: Claude fügt das Todo als `pending` hinzu, wenn es eine Aufgabe identifiziert
2. **Aktiviert**: Claude setzt das Todo auf `in_progress`, wenn es die Arbeit beginnt
3. **Abgeschlossen**: Claude markiert es als abgeschlossen, wenn die Aufgabe erfolgreich beendet wird
4. **Entfernt**: Claude löscht ein Todo, das es nicht mehr benötigt, indem es `status: "deleted"` in einem `TaskUpdate`-Aufruf setzt

<h2 id="when-claude-creates-todos">
  Wann Claude Todos erstellt
</h2>

In einer [Sitzung, die die Task-Tracking-Tools hat](#model-availability), erstellt Claude Todos für die meisten mehrstufigen Arbeiten, wie zum Beispiel:

* **Komplexe mehrstufige Aufgaben**, die drei oder mehr unterschiedliche Aktionen erfordern
* **Von Benutzern bereitgestellte Aufgabenlisten**, wenn mehrere Elemente erwähnt werden
* **Längere Operationen**, die von der Fortschrittsverfolgung profitieren
* **Explizite Anfragen**, wenn Benutzer um Todo-Organisation bitten

Claude kann Todos für sehr kurze oder einstufige Anfragen überspringen.

<h2 id="examples">
  Beispiele
</h2>

Bevor Sie diese Beispiele ausführen, installieren Sie das Claude Agent SDK, indem Sie dem [Schnellstart](/docs/de/agent-sdk/quickstart) folgen. Jedes Beispiel auf dieser Seite teilt die gleiche Berechtigungseinrichtung und das gleiche Beendigungsverhalten:

* **Berechtigungsmodus**: Die Beispiel-Prompts bitten Claude, echte Arbeit an einem Projekt zu leisten, daher setzt jedes Beispiel `permissionMode: "acceptEdits"` (TypeScript) oder `permission_mode="acceptEdits"` (Python), um die Dateibearbeitungen, die die Arbeit erzeugt, automatisch zu genehmigen. Siehe [Berechtigungsmodi](/docs/de/agent-sdk/permissions#permission-modes) für die Alternativen.
* **Turnus-Limit**: Jedes Beispiel wird ausgeführt, bis der Agent fertig ist und seine endgültige Ergebnismeldung liefert. Wenn eine Sitzung zuerst ihr Turnus-Limit erreicht, hat diese Ergebnismeldung den Subtyp `error_max_turns`. Überprüfen Sie `subtype`, um dieses Ende zu erkennen.
* **Fehlerbehandlung**: Diese Beispiele verwenden Single-Shot-`query()`-Aufrufe. Nach dem Liefern eines `error_max_turns`-Ergebnisses wirft `query()` einen Fehler aus, der `Reached maximum number of turns` enthält. Jedes Beispiel umhüllt seine Schleife in einem Try-Block, um sauber zu beenden, wenn dies geschieht. Siehe [Handle the result](/docs/de/agent-sdk/agent-loop#handle-the-result) für die Ergebnis-Subtypen.

<Note>
  Die Task-Systemmeldungen, [`SDKTaskNotificationMessage`](/docs/de/agent-sdk/typescript#sdktasknotificationmessage) (TypeScript) oder [`TaskNotificationMessage`](/docs/de/agent-sdk/python#tasknotificationmessage) (Python) unter ihnen, berichten über Hintergrund-Tasks wie backgroundete Befehle und Sub-Agenten. Im Nachrichtenstrom sehen Sie Todo-Aktivität als `tool_use`-Blöcke in den Assistent-Meldungen.
</Note>

<h3 id="monitor-todo-changes">
  Überwachen Sie Todo-Änderungen
</h3>

Das folgende Beispiel beobachtet den Assistent-Stream auf `TaskCreate`- und `TaskUpdate`-`tool_use`-Blöcke und druckt eine `+`-Zeile mit dem Betreff jeder neuen Aufgabe und eine Update-Zeile mit der Task-ID und dem neuen Status jeder Statusänderung. Verwenden Sie diese Form, wenn Sie ein Protokoll der Task-Aktivität statt einer gerenderten Anzeige möchten. Die `+`-Zeilen enthalten nicht die zugewiesenen IDs, daher kann dieses Protokoll Updates nicht zurück zu ihren Erstellungen abgleichen. Um diese Korrelation zu behalten, erfassen Sie die IDs wie [Zeigen Sie den Fortschritt in Echtzeit an](#display-progress-in-real-time).

Die gestreamte `tool_use`-Eingabe ist die rohe Form, die das Modell ausgegeben hat. Claude Code repariert einige nahezu korrekte, aber fehlerhafte Schlüsselnamen vor der Ausführung, indem es `id` oder `task_id` auf `taskId` und `active_form` auf `activeForm` abbildet, aber diese Reparatur wird nicht im Stream widergespiegelt. Lesen Sie `TaskUpdate`-Eingabefelder defensiv, wie beide Beispiele auf dieser Seite, anstatt anzunehmen, dass der kanonische Name immer vorhanden ist.

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
  Zeigen Sie den Fortschritt in Echtzeit an
</h3>

Das folgende Beispiel beobachtet den Assistent-Stream auf `TaskCreate`- und `TaskUpdate`-`tool_use`-Blöcke und führt eine Zuordnung von Tasks mit Task-ID in einer `TaskTracker`-Klasse, wobei eine Fortschrittszusammenfassung bei jeder Änderung neu gerendert wird. Die Zusammenfassung zählt abgeschlossene und laufende Tasks und zeigt das `activeForm`-Label jedes aktiven Elements anstelle seines `subject`. Verwenden Sie diese Form, wenn Ihre Anwendung eine Fortschrittsanzeige führt, anstatt jedes Ereignis zu protokollieren.

Die zugewiesene Task-ID befindet sich nicht in der `TaskCreate`-Eingabe. Claude Code liefert die strukturierte Ausgabe jedes Tools in der Benutzermeldung, die seinen `tool_result`-Block trägt, im Feld `tool_use_result`. Für `TaskCreate` ist dieses Objekt für TypeScript als `TaskCreateOutput` unter [Tool Output Types](/docs/de/agent-sdk/typescript#tool-output-types) dokumentiert, und in Python ist das Feld ein einfaches Dict der gleichen Form. Der Tracker paart jeden `tool_result`-Block mit seinem `tool_use`-Aufruf nach `tool_use_id` und liest `task.id` aus der gepaarten Meldung des `tool_use_result`. Claude kann die Liste mit `TaskList` zurücklesen und die vollständigen Details einer Task mit `TaskGet`.

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
  Zugehörige Dokumentation
</h2>

* [Agent SDK-Referenz - TypeScript](/docs/de/agent-sdk/typescript): Die Optionen, Typen und Tool-Schemas für das TypeScript SDK, einschließlich der Task-Tool-Eingabe- und Ausgabetypen
* [Agent SDK-Referenz - Python](/docs/de/agent-sdk/python): Die Optionen, Typen und Tool-Dokumentation für das Python SDK
* [Streaming-Eingabe](/docs/de/agent-sdk/streaming-vs-single-mode): Die zwei Eingabemodi und wann Streaming-Eingabe statt der Single-Shot-Aufrufe verwendet werden sollte, die diese Beispiele verwenden
* [Geben Sie Claude benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools): Definieren Sie Ihre eigenen Tools mit dem In-Process-MCP-Server des SDK
