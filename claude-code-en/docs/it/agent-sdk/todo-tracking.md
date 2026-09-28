> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Traccia todo

> Traccia i todo nelle sessioni di Agent SDK e visualizza i progressi di Claude nella tua applicazione da chiamate di strumenti strutturate

Claude Code fornisce i [task-tracking tools](/docs/it/tools-reference#task-tool-availability) per impostazione predefinita solo sui modelli elencati in [Disponibilità del modello](#model-availability). I modelli più recenti traccia il lavoro multi-step senza un elenco todo scritto, quindi su quelli non hai bisogno di nulla in questa pagina affinché Claude lavori attraverso attività multi-step.

In una sessione che ha i task-tracking tools, Claude mantiene un elenco todo scritto, aggiornando lo stato di ogni elemento mentre lavora. Vedi ogni cambiamento nel flusso dei messaggi come una chiamata di strumento strutturata. Abilita una sessione solo quando la tua applicazione legge quelle chiamate di strumento, sia per registrare l'attività delle attività che per renderizzare il proprio display di progresso.

<h2 id="model-availability">
  Disponibilità del modello
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

Su un modello che non dispone degli strumenti per impostazione predefinita, a meno che non abiliti una sessione, non vedrai blocchi `tool_use` per loro nel flusso dei messaggi. L'Agent SDK applica questi valori predefiniti attraverso il binario Claude Code che raggruppa. Se punti `pathToClaudeCodeExecutable` (TypeScript) o `cli_path` (Python) alla tua installazione di Claude Code, otterrai gli strumenti che quella installazione fornisce, secondo i suoi valori predefiniti. Per vedere l'insieme esatto in una sessione in esecuzione, [controlla quali strumenti sono disponibili](/docs/it/tools-reference#check-which-tools-are-available). Per abilitare una sessione, fai uno dei seguenti:

* Nomina uno degli strumenti nell'opzione [`allowedTools`](/docs/it/agent-sdk/permissions#allow-and-deny-rules) (TypeScript) o `allowed_tools` (Python)
* Elenca gli strumenti nell'opzione `tools`, che limita gli strumenti integrati della sessione a quelli che nomina. Includi gli strumenti che desideri insieme agli altri strumenti integrati che utilizzi
* Imposta `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` nell'opzione `env`, come fanno gli esempi in questa pagina. In TypeScript, `env` sostituisce l'ambiente del sottoprocesso, quindi diffondere `...process.env` per mantenere le variabili ereditate. In Python, `env` viene unito sopra l'ambiente ereditato

<h2 id="todo-lifecycle">
  Ciclo di vita dei todo
</h2>

Claude sposta ogni todo attraverso un ciclo di vita prevedibile:

1. **Creato**: Claude aggiunge il todo come `pending` quando identifica un'attività
2. **Attivato**: Claude imposta il todo su `in_progress` quando inizia il lavoro
3. **Completato**: Claude lo contrassegna come completato quando l'attività termina con successo
4. **Rimosso**: Claude elimina un todo che non ha più bisogno impostando `status: "deleted"` in una chiamata `TaskUpdate`

<h2 id="when-claude-creates-todos">
  Quando Claude crea i todo
</h2>

In una [sessione che ha i task-tracking tools](#model-availability), Claude crea todo per la maggior parte del lavoro multi-step, come:

* **Attività complesse multi-step** che richiedono tre o più azioni distinte
* **Elenchi di attività forniti dall'utente** quando vengono menzionati più elementi
* **Operazioni più lunghe** che traggono beneficio dal tracciamento dei progressi
* **Richieste esplicite** quando gli utenti chiedono l'organizzazione dei todo

Claude potrebbe saltare i todo per richieste molto brevi o a singolo step.

<h2 id="examples">
  Esempi
</h2>

Prima di eseguire questi esempi, installa Claude Agent SDK seguendo la [guida rapida](/docs/it/agent-sdk/quickstart). Ogni esempio in questa pagina condivide la stessa configurazione di permessi e comportamento di uscita:

* **Modalità di permesso**: gli esempi di prompt chiedono a Claude di fare lavoro reale su un progetto, quindi ogni esempio imposta `permissionMode: "acceptEdits"` (TypeScript) o `permission_mode="acceptEdits"` (Python) per approvare automaticamente le modifiche ai file che il lavoro produce. Vedi [Modalità di permesso](/docs/it/agent-sdk/permissions#permission-modes) per le alternative.
* **Limite di turni**: ogni esempio viene eseguito fino a quando l'agente non termina e produce il suo messaggio di risultato finale. Se una sessione raggiunge prima il limite di turni, quel messaggio di risultato ha il sottotipo `error_max_turns`. Controlla `subtype` per rilevare quella conclusione.
* **Gestione degli errori**: questi esempi utilizzano singole chiamate `query()`. Dopo aver prodotto un risultato `error_max_turns`, `query()` genera un errore che include `Reached maximum number of turns`. Ogni esempio racchiude il suo ciclo in un blocco try per uscire correttamente quando ciò accade. Vedi [Gestire il risultato](/docs/it/agent-sdk/agent-loop#handle-the-result) per i sottotipi di risultato.

<Note>
  I messaggi di sistema del task, [`SDKTaskNotificationMessage`](/docs/it/agent-sdk/typescript#sdktasknotificationmessage) (TypeScript) o [`TaskNotificationMessage`](/docs/it/agent-sdk/python#tasknotificationmessage) (Python) tra loro, segnalano attività di background come comandi in background e subagenti. Nel flusso dei messaggi, vedi l'attività dei todo come blocchi `tool_use` nei messaggi dell'assistente.
</Note>

<h3 id="monitor-todo-changes">
  Monitora i cambiamenti dei todo
</h3>

L'esempio seguente osserva il flusso dell'assistente per i blocchi `tool_use` `TaskCreate` e `TaskUpdate` e stampa una riga `+` con il soggetto di ogni nuovo compito e una riga di aggiornamento con l'ID del compito di ogni cambio di stato e il nuovo stato. Usa questa forma quando desideri un registro dell'attività delle attività piuttosto che un display renderizzato. Le righe `+` non includono gli ID assegnati, quindi questo registro non può far corrispondere gli aggiornamenti ai loro creati. Per mantenere quella correlazione, cattura gli ID come fa [Visualizza i progressi in tempo reale](#display-progress-in-real-time).

L'input `tool_use` trasmesso è la forma grezza che il modello ha emesso. Claude Code ripara alcuni nomi di chiave quasi corretti ma non del tutto prima dell'esecuzione, mappando `id` o `task_id` a `taskId` e `active_form` a `activeForm`, ma questa riparazione non si riflette nel flusso. Leggi i campi di input `TaskUpdate` in modo difensivo, come fanno entrambi gli esempi in questa pagina, piuttosto che assumere che il nome canonico sia sempre presente.

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
  Visualizza i progressi in tempo reale
</h3>

L'esempio seguente osserva il flusso dell'assistente per i blocchi `tool_use` `TaskCreate` e `TaskUpdate` e mantiene una mappa di attività con chiave dell'ID attività in una classe `TaskTracker`, renderizzando un riepilogo dei progressi ad ogni cambiamento. Il riepilogo conta le attività completate e in corso e mostra l'etichetta `activeForm` di ogni elemento attivo al posto del suo `subject`. Usa questa forma quando la tua applicazione mantiene un display di progresso invece di registrare ogni evento.

L'ID attività assegnato non è nell'input `TaskCreate`. Claude Code fornisce l'output strutturato di ogni strumento nel messaggio dell'utente che porta il suo blocco `tool_result`, nel campo `tool_use_result`. Per `TaskCreate`, quell'oggetto è documentato per TypeScript come `TaskCreateOutput` in [Tool Output Types](/docs/it/agent-sdk/typescript#tool-output-types), e in Python il campo è un semplice dict della stessa forma. Il tracker abbina ogni blocco `tool_result` alla sua chiamata `tool_use` per `tool_use_id` e legge `task.id` dal `tool_use_result` del messaggio abbinato. Claude può leggere l'elenco di nuovo con `TaskList` e i dettagli completi di un'attività con `TaskGet`.

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
  Documentazione correlata
</h2>

* [Riferimento Agent SDK - TypeScript](/docs/it/agent-sdk/typescript): le opzioni, i tipi e gli schemi degli strumenti per l'SDK TypeScript, inclusi i tipi di input e output dello strumento Task
* [Riferimento Agent SDK - Python](/docs/it/agent-sdk/python): le opzioni, i tipi e la documentazione degli strumenti per l'SDK Python
* [Streaming Input](/docs/it/agent-sdk/streaming-vs-single-mode): i due modalità di input, e quando utilizzare lo streaming input invece delle chiamate a singolo scatto che questi esempi utilizzano
* [Dai a Claude strumenti personalizzati](/docs/it/agent-sdk/custom-tools): definisci i tuoi strumenti con il server MCP in-process dell'SDK
