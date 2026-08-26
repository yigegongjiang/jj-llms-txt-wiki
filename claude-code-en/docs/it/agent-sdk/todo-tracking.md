> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Elenchi Todo

> Traccia e visualizza i todo utilizzando Claude Agent SDK per una gestione organizzata delle attività

Il tracciamento dei todo fornisce un modo strutturato per gestire le attività e visualizzare i progressi agli utenti. Claude Agent SDK include funzionalità todo integrate che aiutano a organizzare flussi di lavoro complessi e mantengono gli utenti informati sulla progressione delle attività.

<Note>
  A partire da TypeScript Agent SDK 0.3.142 e Claude Code v2.1.142, le sessioni utilizzano i tool Task strutturati `TaskCreate`, `TaskUpdate`, `TaskGet` e `TaskList` al posto di `TodoWrite`. L'SDK Python riceve questo cambiamento dalla CLI Claude Code che avvia, non dalla versione del pacchetto Python: il passaggio si applica una volta che quella CLI — la copia inclusa nel pacchetto pip, o una a cui punti con `cli_path` — è v2.1.142 o successiva. Vedere [Migrazione ai tool Task](#migrate-to-task-tools) per come monitorare i cambiamenti del codice. Gli esempi in questa pagina impostano `CLAUDE_CODE_ENABLE_TASKS=0` per continuare a mostrare `TodoWrite` per le sessioni che non hanno ancora eseguito la migrazione.
</Note>

<h3 id="todo-lifecycle">
  Ciclo di vita dei Todo
</h3>

I todo seguono un ciclo di vita prevedibile:

1. **Creati** come `pending` quando le attività vengono identificate
2. **Attivati** a `in_progress` quando il lavoro inizia
3. **Completati** quando l'attività termina con successo
4. **Rimossi** quando tutte le attività in un gruppo sono completate

<h3 id="when-todos-are-used">
  Quando vengono utilizzati i Todo
</h3>

L'SDK crea todo per la maggior parte del lavoro multi-step, come:

* **Attività complesse multi-step** che richiedono 3 o più azioni distinte
* **Elenchi di attività forniti dall'utente** quando vengono menzionati più elementi
* **Operazioni non banali** che traggono beneficio dal tracciamento dei progressi
* **Richieste esplicite** quando gli utenti chiedono l'organizzazione dei todo

Potrebbe saltare i todo per richieste molto brevi o a singolo step.

<h2 id="examples">
  Esempi
</h2>

Prima di eseguire questi esempi, installare Claude Agent SDK seguendo la [guida rapida](/docs/it/agent-sdk/quickstart).

Ogni esempio viene eseguito fino a quando l'agente non termina e produce il suo messaggio di risultato finale. Se una sessione raggiunge prima il limite di turni, quel messaggio di risultato ha il sottotipo `error_max_turns`. Controllare `subtype` per rilevare quella conclusione.

Questi esempi utilizzano chiamate `query()` a singolo scatto. Dopo aver prodotto un risultato `error_max_turns`, `query()` genera un errore che include `Reached maximum number of turns`. Ogni esempio racchiude il suo ciclo in un blocco try per uscire correttamente quando ciò accade.

Vedere [Gestire il risultato](/docs/it/agent-sdk/agent-loop#handle-the-result) per i sottotipi di risultato.

<h3 id="monitoring-todo-changes">
  Monitoraggio dei cambiamenti dei Todo
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
  Visualizzazione dei progressi in tempo reale
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
  Migrazione ai tool Task
</h2>

I tool Task dividono la singola chiamata `TodoWrite` in `TaskCreate` per ogni nuovo elemento e `TaskUpdate` per ogni cambio di stato, con `TaskList` e `TaskGet` disponibili affinché il modello possa leggere di nuovo l'elenco corrente. Il codice di monitoraggio continua a ispezionare i blocchi `tool_use` nel flusso dell'assistente, ma mantiene una mappa con chiave dell'ID attività invece di sostituire l'intero elenco ad ogni chiamata. I tool Task sono l'impostazione predefinita a partire da TypeScript Agent SDK 0.3.142 e Claude Code v2.1.142, quindi non è necessario alcun cambio `options.env`.

| Con `TodoWrite`                                        | Con tool Task                                                                                                                                                                                                                                                                                              |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Una chiamata di tool riscrive l'intero array `todos`   | `TaskCreate` aggiunge un elemento, `TaskUpdate` modifica un elemento per `taskId`                                                                                                                                                                                                                          |
| Corrisponde a `block.name === "TodoWrite"`             | Corrisponde a `block.name === "TaskCreate"` o `"TaskUpdate"`                                                                                                                                                                                                                                               |
| Forma dell'elemento: `{ content, status, activeForm }` | Input `TaskCreate`: `{ subject, description, activeForm?, metadata? }`. Input `TaskUpdate`: `{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`. `status` è `"pending"`, `"in_progress"` o `"completed"`; impostare `status: "deleted"` per eliminare |
| Renderizza `block.input.todos` direttamente            | Accumula elementi tra le chiamate, o leggi uno snapshot da un risultato dello strumento `TaskList`                                                                                                                                                                                                         |

L'ID attività assegnato non è nell'input `TaskCreate`. Ritorna nel `tool_result` corrispondente come `{ task: { id, subject } }`, quindi acquisiscilo dal blocco del risultato per inserire la chiave nella mappa. L'esempio seguente mostra il cambio minimo al ciclo [Monitoraggio dei cambiamenti dei Todo](#monitoring-todo-changes). Legge solo gli input `tool_use` e salta l'acquisizione degli ID dai blocchi `tool_result`. Per renderizzare un elenco completo, guarda un risultato dello strumento `TaskList` nel flusso o accumula i risultati `TaskCreate` e gli input `TaskUpdate` in una mappa.

L'input `tool_use` trasmesso è la forma grezza che il modello ha emesso. Claude Code ripara alcuni nomi di chiave quasi corretti ma non del tutto prima dell'esecuzione, mappando `id` o `task_id` a `taskId` e `active_form` a `activeForm`, ma questa riparazione non si riflette nel flusso. Leggi i campi di input `TaskUpdate` in modo difensivo, come fanno gli esempi seguenti, piuttosto che assumere che il nome canonico sia sempre presente.

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
  Documentazione correlata
</h2>

* [Riferimento TypeScript SDK](/docs/it/agent-sdk/typescript)
* [Riferimento Python SDK](/docs/it/agent-sdk/python)
* [Streaming vs Single Mode](/docs/it/agent-sdk/streaming-vs-single-mode)
* [Custom Tools](/docs/it/agent-sdk/custom-tools)
