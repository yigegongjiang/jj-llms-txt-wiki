> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Todo-Listen

> Verfolgen und zeigen Sie Todos mit dem Claude Agent SDK für organisierte Aufgabenverwaltung an

Die Todo-Verfolgung bietet eine strukturierte Möglichkeit, Aufgaben zu verwalten und Benutzer über den Aufgabenfortschritt zu informieren. Das Claude Agent SDK enthält integrierte Todo-Funktionalität, die dabei hilft, komplexe Arbeitsabläufe zu organisieren und Benutzer über die Aufgabenprogression zu informieren.

<Note>
  Ab TypeScript Agent SDK 0.3.142 und Claude Code v2.1.142 verwenden Sitzungen die strukturierten Task-Tools `TaskCreate`, `TaskUpdate`, `TaskGet` und `TaskList` anstelle von `TodoWrite`. Das Python SDK erhält diese Änderung von der Claude Code CLI, die es startet, nicht von der Python-Paketversion: Der Wechsel gilt, sobald diese CLI — die im pip-Paket enthaltene Kopie oder eine, auf die Sie mit `cli_path` verweisen — v2.1.142 oder später ist. Siehe [Zu Task-Tools migrieren](#migrate-to-task-tools) für Informationen darüber, wie sich der Überwachungscode ändert. Die Beispiele auf dieser Seite setzen `CLAUDE_CODE_ENABLE_TASKS=0`, um weiterhin `TodoWrite` für Sitzungen anzuzeigen, die noch nicht migriert wurden.
</Note>

<h3 id="todo-lifecycle">
  Todo-Lebenszyklus
</h3>

Todos folgen einem vorhersehbaren Lebenszyklus:

1. **Erstellt** als `pending`, wenn Aufgaben identifiziert werden
2. **Aktiviert** zu `in_progress`, wenn die Arbeit beginnt
3. **Abgeschlossen**, wenn die Aufgabe erfolgreich beendet wird
4. **Entfernt**, wenn alle Aufgaben in einer Gruppe abgeschlossen sind

<h3 id="when-todos-are-used">
  Wann Todos verwendet werden
</h3>

Das SDK erstellt Todos für die meisten mehrstufigen Arbeiten, wie zum Beispiel:

* **Komplexe mehrstufige Aufgaben**, die 3 oder mehr unterschiedliche Aktionen erfordern
* **Von Benutzern bereitgestellte Aufgabenlisten**, wenn mehrere Elemente erwähnt werden
* **Nicht triviale Operationen**, die von der Fortschrittsverfolgung profitieren
* **Explizite Anfragen**, wenn Benutzer um Todo-Organisation bitten

Es kann Todos für sehr kurze oder einstufige Anfragen überspringen.

<h2 id="examples">
  Beispiele
</h2>

Bevor Sie diese Beispiele ausführen, installieren Sie das Claude Agent SDK, indem Sie dem [Schnellstart](/docs/de/agent-sdk/quickstart) folgen.

Jedes Beispiel wird ausgeführt, bis der Agent fertig ist und seine endgültige Ergebnismeldung liefert. Wenn eine Sitzung zuerst ihr Turnus-Limit erreicht, hat diese Ergebnismeldung den Subtyp `error_max_turns`. Überprüfen Sie `subtype`, um dieses Ende zu erkennen.

Diese Beispiele verwenden Single-Shot-`query()`-Aufrufe. Nach dem Liefern eines `error_max_turns`-Ergebnisses wirft `query()` einen Fehler aus, der `Reached maximum number of turns` enthält. Jedes Beispiel umhüllt seine Schleife in einem Try-Block, um sauber zu beenden, wenn dies geschieht.

Siehe [Handle the result](/docs/de/agent-sdk/agent-loop#handle-the-result) für die Ergebnis-Subtypen.

<h3 id="monitoring-todo-changes">
  Überwachung von Todo-Änderungen
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
  Echtzeit-Fortschrittsanzeige
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
  Zu Task-Tools migrieren
</h2>

Die Task-Tools teilen den einzelnen `TodoWrite`-Aufruf in `TaskCreate` für jedes neue Element und `TaskUpdate` für jede Statusänderung auf, wobei `TaskList` und `TaskGet` für das Modell verfügbar sind, um die aktuelle Liste zu lesen. Ihr Überwachungscode inspiziert weiterhin `tool_use`-Blöcke im Assistent-Stream, verwaltet aber eine Zuordnung mit Task-ID als Schlüssel, anstatt die gesamte Liste bei jedem Aufruf zu ersetzen. Die Task-Tools sind ab TypeScript Agent SDK 0.3.142 und Claude Code v2.1.142 die Standardeinstellung, daher ist keine Änderung von `options.env` erforderlich.

| Mit `TodoWrite`                                        | Mit Task-Tools                                                                                                                                                                                                                                                                                                     |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Ein Tool-Aufruf schreibt das gesamte `todos`-Array neu | `TaskCreate` fügt ein Element hinzu, `TaskUpdate` patcht ein Element nach `taskId`                                                                                                                                                                                                                                 |
| Abgleich `block.name === "TodoWrite"`                  | Abgleich `block.name === "TaskCreate"` oder `"TaskUpdate"`                                                                                                                                                                                                                                                         |
| Element-Form: `{ content, status, activeForm }`        | `TaskCreate`-Eingabe: `{ subject, description, activeForm?, metadata? }`. `TaskUpdate`-Eingabe: `{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`. `status` ist `"pending"`, `"in_progress"` oder `"completed"`; setzen Sie `status: "deleted"` zum Löschen |
| Rendern Sie `block.input.todos` direkt                 | Sammeln Sie Elemente über Aufrufe hinweg, oder lesen Sie einen Snapshot aus einem `TaskList`-Tool-Ergebnis                                                                                                                                                                                                         |

Die zugewiesene Task-ID befindet sich nicht in der `TaskCreate`-Eingabe. Sie kommt im entsprechenden `tool_result` als `{ task: { id, subject } }` zurück, daher erfassen Sie sie aus dem Ergebnis-Block, um Ihre Zuordnung zu schlüsseln. Das folgende Beispiel zeigt die minimale Änderung an der Schleife [Überwachung von Todo-Änderungen](#monitoring-todo-changes). Es liest nur `tool_use`-Eingaben und überspringt das Erfassen von IDs aus `tool_result`-Blöcken. Um eine vollständige Liste zu rendern, beobachten Sie ein `TaskList`-Tool-Ergebnis im Stream oder sammeln Sie `TaskCreate`-Ergebnisse und `TaskUpdate`-Eingaben in einer Zuordnung.

Der gestreamte `tool_use`-Input ist die rohe Form, die das Modell ausgegeben hat. Claude Code repariert einige nahezu korrekte, aber fehlerhafte Schlüsselnamen vor der Ausführung, indem es `id` oder `task_id` auf `taskId` und `active_form` auf `activeForm` abbildet, aber diese Reparatur wird nicht im Stream widergespiegelt. Lesen Sie `TaskUpdate`-Eingabefelder defensiv, wie die folgenden Beispiele zeigen, anstatt anzunehmen, dass der kanonische Name immer vorhanden ist.

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
  Zugehörige Dokumentation
</h2>

* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript)
* [Python SDK-Referenz](/docs/de/agent-sdk/python)
* [Streaming vs. Single Mode](/docs/de/agent-sdk/streaming-vs-single-mode)
* [Benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools)
