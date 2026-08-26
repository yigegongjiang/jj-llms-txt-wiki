> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestire approvazioni e input dell'utente

> Presenta le richieste di approvazione e le domande di chiarimento di Claude agli utenti, quindi restituisci le loro decisioni all'SDK.

Durante il lavoro su un'attività, Claude a volte ha bisogno di verificare con gli utenti. Potrebbe aver bisogno di autorizzazione prima di eliminare file, oppure potrebbe aver bisogno di chiedere quale database utilizzare per un nuovo progetto. La tua applicazione deve presentare queste richieste agli utenti in modo che Claude possa continuare con il loro input.

Claude richiede input dell'utente in due situazioni: quando ha bisogno di **autorizzazione per utilizzare uno strumento** (come eliminare file o eseguire comandi), e quando ha **domande di chiarimento** (tramite lo strumento `AskUserQuestion`). Entrambi attivano il tuo callback `canUseTool`, che mette in pausa l'esecuzione fino a quando non restituisci una risposta. Questo è diverso dai normali turni di conversazione in cui Claude finisce e attende il tuo prossimo messaggio.

Per le domande di chiarimento, Claude genera le domande e le opzioni. Il tuo ruolo è presentarle agli utenti e restituire le loro selezioni. Non puoi aggiungere le tue domande a questo flusso; se hai bisogno di chiedere qualcosa agli utenti tu stesso, fallo separatamente nella logica dell'applicazione.

Il callback può rimanere in sospeso indefinitamente. L'esecuzione rimane in pausa fino a quando il callback non restituisce, e l'SDK annulla l'attesa solo quando la query stessa viene annullata. Se un utente potrebbe impiegare più tempo per rispondere di quanto il tuo processo possa ragionevolmente rimanere in esecuzione, restituisci la decisione [`defer` hook](/docs/it/hooks#defer-a-tool-call-for-later), che consente al processo di uscire e riprendere in seguito dalla sessione persistente.

Questa guida ti mostra come rilevare ogni tipo di richiesta e rispondere in modo appropriato.

<h2 id="detect-when-claude-needs-input">
  Rilevare quando Claude ha bisogno di input
</h2>

Passa un callback `canUseTool` nelle opzioni della query. Il callback si attiva ogni volta che Claude ha bisogno di input dell'utente, ricevendo il nome dello strumento e l'input come argomenti:

<CodeGroup>
  ```python Python theme={null}
  async def handle_tool_request(tool_name, input_data, context):
      # Chiedi all'utente e restituisci allow o deny
      ...


  options = ClaudeAgentOptions(can_use_tool=handle_tool_request)
  ```

  ```typescript TypeScript theme={null}
  async function handleToolRequest(toolName, input, options) {
    // options includes { signal: AbortSignal, suggestions?: PermissionUpdate[] }
    // Chiedi all'utente e restituisci allow o deny
  }

  const options = { canUseTool: handleToolRequest };
  ```
</CodeGroup>

Il callback si attiva in due casi:

1. **Lo strumento ha bisogno di approvazione**: Claude vuole utilizzare uno strumento che non è approvato automaticamente da una [regola di autorizzazione](/docs/it/agent-sdk/permissions) o da una modalità di autorizzazione. Controlla `tool_name` per lo strumento (ad es. `"Bash"`, `"Write"`).
2. **Claude pone una domanda**: Claude chiama lo strumento `AskUserQuestion`. Controlla se `tool_name == "AskUserQuestion"` per gestirlo diversamente. Se specifichi un array `tools`, includi `AskUserQuestion` affinché funzioni. Vedi [Gestire domande di chiarimento](#handle-clarifying-questions) per i dettagli.

<Warning>
  **Il callback non si attiva mai per gli strumenti approvati automaticamente.** Qualsiasi approvazione precedente nel [flusso di valutazione delle autorizzazioni](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated), una regola di consentimento o una modalità come `acceptEdits` o `bypassPermissions`, risolve la chiamata prima che `canUseTool` sia consultato. Se elenchi uno strumento direttamente in `allowed_tools`, un controllo `canUseTool` per quello strumento non viene mai eseguito a meno che una regola di richiesta o la modalità `plan` non reindirizzi la chiamata a un prompt. Per la logica che deve applicarsi a ogni chiamata di strumento, utilizza un [hook `PreToolUse`](/docs/it/agent-sdk/hooks), che viene eseguito prima del resto del flusso e può consentire, negare o modificare le richieste.

  `AskUserQuestion`, gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool), e gli strumenti connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) raggiungono il callback anche quando una regola di consentimento corrisponde. In modalità `dontAsk` queste chiamate vengono invece negate, senza invocare il callback.
</Warning>

Puoi anche utilizzare l'[hook `PermissionRequest`](/docs/it/agent-sdk/hooks#available-hooks) per inviare notifiche esterne (Slack, email, push) quando Claude è in attesa di approvazione.

<h2 id="handle-tool-approval-requests">
  Gestire le richieste di approvazione dello strumento
</h2>

Una volta passato un callback `canUseTool` nelle opzioni della query, si attiva quando Claude vuole utilizzare uno strumento che nulla prima nel flusso di autorizzazione ha approvato. Il tuo callback riceve tre argomenti:

| Argomento                           | Descrizione                                                                                                                                                                                                                                                                                                                                       |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `toolName`                          | Il nome dello strumento che Claude vuole utilizzare (ad es. `"Bash"`, `"Write"`, `"Edit"`)                                                                                                                                                                                                                                                        |
| `input`                             | I parametri che Claude sta passando allo strumento. Il contenuto varia a seconda dello strumento.                                                                                                                                                                                                                                                 |
| `options` (TS) / `context` (Python) | Contesto aggiuntivo incluso `suggestions` opzionale (voci `PermissionUpdate` proposte per evitare di ripetere le domande) e un segnale di annullamento. In TypeScript, `signal` è un `AbortSignal`; in Python, il campo signal è riservato per uso futuro. Vedi [`ToolPermissionContext`](/docs/it/agent-sdk/python#toolpermissioncontext) per Python. |

L'oggetto `input` contiene parametri specifici dello strumento. Esempi comuni:

| Strumento | Campi di input                          |
| --------- | --------------------------------------- |
| `Bash`    | `command`, `description`, `timeout`     |
| `Write`   | `file_path`, `content`                  |
| `Edit`    | `file_path`, `old_string`, `new_string` |
| `Read`    | `file_path`, `offset`, `limit`          |

Vedi il riferimento SDK per gli schemi di input completi: [Python](/docs/it/agent-sdk/python#tool-input%2Foutput-types) | [TypeScript](/docs/it/agent-sdk/typescript#tool-input-types).

Puoi visualizzare queste informazioni all'utente in modo che possa decidere se consentire o rifiutare l'azione, quindi restituire la risposta appropriata.

L'esempio seguente chiede a Claude di creare ed eliminare un file di test. Quando Claude tenta ogni operazione, il callback stampa la richiesta dello strumento nel terminale e chiede l'approvazione s/n.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import (
      HookMatcher,
      PermissionResultAllow,
      PermissionResultDeny,
      ToolPermissionContext,
  )


  async def can_use_tool(
      tool_name: str, input_data: dict, context: ToolPermissionContext
  ) -> PermissionResultAllow | PermissionResultDeny:
      # Visualizza la richiesta dello strumento
      print(f"\nTool: {tool_name}")
      if tool_name == "Bash":
          print(f"Command: {input_data.get('command')}")
          if input_data.get("description"):
              print(f"Description: {input_data.get('description')}")
      else:
          print(f"Input: {input_data}")

      # Ottieni l'approvazione dell'utente
      response = input("Allow this action? (y/n): ")

      # Restituisci allow o deny in base alla risposta dell'utente
      if response.lower() == "y":
          # Allow: lo strumento viene eseguito con l'input originale (o modificato)
          return PermissionResultAllow(updated_input=input_data)
      else:
          # Deny: lo strumento non viene eseguito, Claude vede il messaggio
          return PermissionResultDeny(message="User denied this action")


  # Workaround richiesto: dummy hook mantiene il flusso aperto per can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Create a test file in /tmp and then delete it",
          },
      }


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline";

  // Helper per chiedere all'utente l'input nel terminale
  function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    return new Promise((resolve) =>
      rl.question(question, (answer) => {
        rl.close();
        resolve(answer);
      })
    );
  }

  for await (const message of query({
    prompt: "Create a test file in /tmp and then delete it",
    options: {
      canUseTool: async (toolName, input) => {
        // Visualizza la richiesta dello strumento
        console.log(`\nTool: ${toolName}`);
        if (toolName === "Bash") {
          console.log(`Command: ${input.command}`);
          if (input.description) console.log(`Description: ${input.description}`);
        } else {
          console.log(`Input: ${JSON.stringify(input, null, 2)}`);
        }

        // Ottieni l'approvazione dell'utente
        const response = await prompt("Allow this action? (y/n): ");

        // Restituisci allow o deny in base alla risposta dell'utente
        if (response.toLowerCase() === "y") {
          // Allow: lo strumento viene eseguito con l'input originale (o modificato)
          return { behavior: "allow", updatedInput: input };
        } else {
          // Deny: lo strumento non viene eseguito, Claude vede il messaggio
          return { behavior: "deny", message: "User denied this action" };
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<Note>
  In Python, `can_use_tool` richiede la [modalità streaming](/docs/it/agent-sdk/streaming-vs-single-mode). Quando passi un flusso di messaggi finito attraverso `query(prompt=generator)` o `ClaudeSDKClient.connect(prompt=async_iterable)`, l'SDK chiude il flusso di input dopo l'ultimo messaggio, prima che il callback di autorizzazione possa essere invocato, a meno che un hook registrato o un server MCP in-process non lo mantenga aperto. L'esempio precedente lo mantiene aperto con un hook `PreToolUse` che restituisce `{"continue_": True}`. La connessione senza prompt e l'invio di messaggi attraverso `ClaudeSDKClient.query()` mantiene il flusso aperto di per sé e non ha bisogno di alcun hook.
</Note>

Questo esempio utilizza un flusso s/n in cui qualsiasi input diverso da `y` viene trattato come un rifiuto. In pratica, potresti creare un'interfaccia utente più ricca che consenta agli utenti di modificare la richiesta, fornire feedback o reindirizzare completamente Claude. Vedi [Rispondere alle richieste dello strumento](#respond-to-tool-requests) per tutti i modi in cui puoi rispondere.

<h3 id="respond-to-tool-requests">
  Rispondere alle richieste dello strumento
</h3>

Il tuo callback restituisce uno di due tipi di risposta:

| Risposta  | Python                                     | TypeScript                            |
| --------- | ------------------------------------------ | ------------------------------------- |
| **Allow** | `PermissionResultAllow(updated_input=...)` | `{ behavior: "allow", updatedInput }` |
| **Deny**  | `PermissionResultDeny(message=...)`        | `{ behavior: "deny", message }`       |

Quando consenti, lo strumento viene eseguito con l'input che Claude ha richiesto a meno che non restituisca un input modificato, `updatedInput` in TypeScript o `updated_input` in Python. Prima della v2.1.207, Claude Code rifiutava un risultato di allow che ometteva `updatedInput` e negava la chiamata dello strumento con un errore di convalida.

Quando neghi, fornisci un messaggio che spiega il motivo. Claude vede questo messaggio e potrebbe adattare il suo approccio.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk.types import PermissionResultAllow, PermissionResultDeny

  # Consenti l'esecuzione dello strumento
  return PermissionResultAllow(updated_input=input_data)

  # Blocca lo strumento
  return PermissionResultDeny(message="User rejected this action")
  ```

  ```typescript TypeScript theme={null}
  // Consenti l'esecuzione dello strumento
  return { behavior: "allow", updatedInput: input };

  // Blocca lo strumento
  return { behavior: "deny", message: "User rejected this action" };
  ```
</CodeGroup>

Oltre a consentire o negare, puoi modificare l'input dello strumento o fornire contesto che aiuta Claude ad adattare il suo approccio:

* **Approva**: consenti l'esecuzione dello strumento come richiesto da Claude
* **Approva con modifiche**: modifica l'input prima dell'esecuzione (ad es. sanitizza i percorsi, aggiungi vincoli)
* **Approva e ricorda**: ripeti una regola di autorizzazione suggerita in modo che le chiamate corrispondenti saltino il prompt la prossima volta
* **Rifiuta**: blocca lo strumento e spiega a Claude il motivo
* **Suggerisci alternativa**: blocca ma guida Claude verso ciò che l'utente vuole invece
* **Reindirizza completamente**: utilizza [input streaming](/docs/it/agent-sdk/streaming-vs-single-mode) per inviare a Claude un'istruzione completamente nuova

<Tabs>
  <Tab title="Approva">
    L'utente approva l'azione così com'è. Passa l'`input` dal tuo callback invariato e lo strumento viene eseguito esattamente come richiesto da Claude.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          print(f"Claude wants to use {tool_name}")
          approved = await ask_user("Allow this action?")

          if approved:
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        console.log(`Claude wants to use ${toolName}`);
        const approved = await askUser("Allow this action?");

        if (approved) {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Approva con modifiche">
    L'utente approva ma vuole modificare la richiesta prima. Puoi cambiare l'input prima che lo strumento venga eseguito. Claude vede il risultato ma non gli viene detto che hai cambiato qualcosa. Utile per sanitizzare i parametri, aggiungere vincoli o limitare l'accesso.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash":
              # L'utente ha approvato, ma limita tutti i comandi alla sandbox
              sandboxed_input = {**input_data}
              sandboxed_input["command"] = input_data["command"].replace(
                  "/tmp", "/tmp/sandbox"
              )
              return PermissionResultAllow(updated_input=sandboxed_input)
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash") {
          // L'utente ha approvato, ma limita tutti i comandi alla sandbox
          const sandboxedInput = {
            ...input,
            command: input.command.replace("/tmp", "/tmp/sandbox")
          };
          return { behavior: "allow", updatedInput: sandboxedInput };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Approva e ricorda">
    L'utente approva e non vuole essere chiesto di nuovo per questo tipo di chiamata. Il terzo argomento del callback contiene `suggestions`, un array di voci [`PermissionUpdate`](/docs/it/agent-sdk/typescript#permissionupdate) già pronte. Ripeti una di queste in `updatedPermissions` per applicarla. Un suggerimento con la destinazione `localSettings` scrive la regola in `.claude/settings.local.json` in modo che le sessioni future saltino il prompt per le chiamate corrispondenti.

    L'esempio Python richiede `claude-agent-sdk` 0.1.80 o successivo.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          choice = await ask_user(f"Allow {tool_name}?", ["once", "always", "no"])

          if choice == "always":
              persist = [
                  s for s in context.suggestions if s.destination == "localSettings"
              ]
              return PermissionResultAllow(
                  updated_input=input_data, updated_permissions=persist
              )
          if choice == "once":
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input, { suggestions = [] }) => {
        const choice = await askUser(`Allow ${toolName}?`, ["once", "always", "no"]);

        if (choice === "always") {
          const persist = suggestions.filter(
            (s) => s.destination === "localSettings"
          );
          return {
            behavior: "allow",
            updatedInput: input,
            updatedPermissions: persist
          };
        }
        if (choice === "once") {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Rifiuta">
    L'utente non vuole che questa azione accada. Blocca lo strumento e fornisci un messaggio che spiega il motivo. Claude vede questo messaggio e potrebbe provare un approccio diverso.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          approved = await ask_user(f"Allow {tool_name}?")

          if not approved:
              return PermissionResultDeny(message="User rejected this action")
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        const approved = await askUser(`Allow ${toolName}?`);

        if (!approved) {
          return {
            behavior: "deny",
            message: "User rejected this action"
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Suggerisci alternativa">
    L'utente non vuole questa azione specifica, ma ha un'idea diversa. Blocca lo strumento e includi una guida nel tuo messaggio. Claude leggerà questo e deciderà come procedere in base al tuo feedback.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash" and "rm" in input_data.get("command", ""):
              # L'utente non vuole eliminare, suggerisci di comprimere invece
              return PermissionResultDeny(
                  message="User doesn't want to delete files. They asked if you could compress them into an archive instead."
              )
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash" && input.command.includes("rm")) {
          // L'utente non vuole eliminare, suggerisci di comprimere invece
          return {
            behavior: "deny",
            message:
              "User doesn't want to delete files. They asked if you could compress them into an archive instead."
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Reindirizza completamente">
    Per un cambio di direzione completo (non solo una spinta), utilizza [input streaming](/docs/it/agent-sdk/streaming-vs-single-mode) per inviare a Claude una nuova istruzione direttamente. Questo bypassa la richiesta dello strumento corrente e dà a Claude istruzioni completamente nuove da seguire.
  </Tab>
</Tabs>

<h2 id="handle-clarifying-questions">
  Gestire domande di chiarimento
</h2>

Quando Claude ha bisogno di più direzione su un'attività con più approcci validi, chiama lo strumento `AskUserQuestion`. Questo attiva il tuo callback `canUseTool` con `toolName` impostato su `AskUserQuestion`. L'input contiene le domande di Claude come opzioni a scelta multipla, che visualizzi all'utente e restituisci le sue selezioni.

<Tip>
  Le domande di chiarimento sono particolarmente comuni nella [modalità `plan`](/docs/it/agent-sdk/permissions#plan-mode-plan), dove Claude esplora la base di codice e pone domande prima di proporre un piano. Questo rende la modalità plan ideale per flussi di lavoro interattivi in cui vuoi che Claude raccolga i requisiti prima di apportare modifiche.
</Tip>

I seguenti passaggi mostrano come gestire le domande di chiarimento:

<Steps>
  <Step title="Passa un callback canUseTool">
    Passa un callback `canUseTool` nelle opzioni della query. Per impostazione predefinita, `AskUserQuestion` è disponibile. Se specifichi un array `tools` per limitare le capacità di Claude (ad esempio, un agente di sola lettura con solo `Read`, `Glob` e `Grep`), includi `AskUserQuestion` in quell'array. Altrimenti, Claude non sarà in grado di porre domande di chiarimento:

    <CodeGroup>
      ```python Python theme={null}
      async for message in query(
          prompt="Analyze this codebase",
          options=ClaudeAgentOptions(
              # Includi AskUserQuestion nella tua lista di strumenti
              tools=["Read", "Glob", "Grep", "AskUserQuestion"],
              can_use_tool=can_use_tool,
          ),
      ):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      for await (const message of query({
        prompt: "Analyze this codebase",
        options: {
          // Includi AskUserQuestion nella tua lista di strumenti
          tools: ["Read", "Glob", "Grep", "AskUserQuestion"],
          canUseTool: async (toolName, input) => {
            // Gestisci le domande di chiarimento qui
          }
        }
      })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Rileva AskUserQuestion">
    Nel tuo callback, controlla se `toolName` è uguale a `AskUserQuestion` per gestirlo diversamente da altri strumenti:

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name: str, input_data: dict, context):
          if tool_name == "AskUserQuestion":
              # La tua implementazione per raccogliere risposte dall'utente
              return await handle_clarifying_questions(input_data)
          # Gestisci altri strumenti normalmente
          return await prompt_for_approval(tool_name, input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "AskUserQuestion") {
          // La tua implementazione per raccogliere risposte dall'utente
          return handleClarifyingQuestions(input);
        }
        // Gestisci altri strumenti normalmente
        return promptForApproval(toolName, input);
      };
      ```
    </CodeGroup>
  </Step>

  <Step title="Analizza l'input della domanda">
    L'input contiene le domande di Claude in un array `questions`. Ogni domanda ha una `question` (il testo da visualizzare), `options` (le scelte) e `multiSelect` (se sono consentite più selezioni):

    ```json theme={null}
    {
      "questions": [
        {
          "question": "How should I format the output?",
          "header": "Format",
          "options": [
            { "label": "Summary", "description": "Brief overview" },
            { "label": "Detailed", "description": "Full explanation" }
          ],
          "multiSelect": false
        },
        {
          "question": "Which sections should I include?",
          "header": "Sections",
          "options": [
            { "label": "Introduction", "description": "Opening context" },
            { "label": "Conclusion", "description": "Final summary" }
          ],
          "multiSelect": true
        }
      ]
    }
    ```

    Vedi [Formato della domanda](#question-format) per le descrizioni complete dei campi.
  </Step>

  <Step title="Raccogli risposte dall'utente">
    Presenta le domande all'utente e raccogli le sue selezioni. Come lo fai dipende dalla tua applicazione: un prompt del terminale, un modulo web, una finestra di dialogo mobile, ecc.
  </Step>

  <Step title="Restituisci le risposte a Claude">
    Costruisci l'oggetto `answers` come un record in cui ogni chiave è il testo `question` e ogni valore è l'`label` dell'opzione selezionata:

    | Dall'oggetto domanda                                          | Usa come |
    | ------------------------------------------------------------- | -------- |
    | Campo `question` (ad es. `"How should I format the output?"`) | Chiave   |
    | Campo `label` dell'opzione selezionata (ad es. `"Summary"`)   | Valore   |

    Per le domande a selezione multipla, passa un array di etichette o uniscile con `", "`. Se [supporti input di testo libero](#support-free-text-input), utilizza il testo personalizzato dell'utente come valore.

    <CodeGroup>
      ```python Python theme={null}
      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": {
                  "How should I format the output?": "Summary",
                  "Which sections should I include?": ["Introduction", "Conclusion"],
              },
          }
      )
      ```

      ```typescript TypeScript theme={null}
      return {
        behavior: "allow",
        updatedInput: {
          questions: input.questions,
          answers: {
            "How should I format the output?": "Summary",
            "Which sections should I include?": "Introduction, Conclusion"
          }
        }
      };
      ```
    </CodeGroup>
  </Step>
</Steps>

<h3 id="question-format">
  Formato della domanda
</h3>

L'input contiene le domande generate da Claude in un array `questions`. Ogni domanda ha questi campi:

| Campo         | Descrizione                                                                                                                              |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `question`    | Il testo completo della domanda da visualizzare                                                                                          |
| `header`      | Etichetta breve per la domanda (max 12 caratteri)                                                                                        |
| `options`     | Array di 2-4 scelte, ognuna con `label` e `description`. TypeScript: opzionalmente `preview` (vedi [sotto](#option-previews-typescript)) |
| `multiSelect` | Se `true`, gli utenti possono selezionare più opzioni                                                                                    |

La struttura che il tuo callback riceve:

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview of key points" },
        { "label": "Detailed", "description": "Full explanation with examples" }
      ],
      "multiSelect": false
    }
  ]
}
```

<h4 id="option-previews-typescript">
  Anteprime delle opzioni (TypeScript)
</h4>

`toolConfig.askUserQuestion.previewFormat` aggiunge un campo `preview` a ogni opzione in modo che la tua app possa mostrare un mockup visivo insieme all'etichetta. Senza questa impostazione, Claude non genera anteprime e il campo è assente.

| `previewFormat`             | `preview` contiene                                                                                                            |
| :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| non impostato (predefinito) | Il campo è assente. Claude non genera anteprime.                                                                              |
| `"markdown"`                | ASCII art e blocchi di codice recintati                                                                                       |
| `"html"`                    | Un frammento `<div>` stilizzato (l'SDK rifiuta `<script>`, `<style>` e `<!DOCTYPE>` prima che il tuo callback venga eseguito) |

Il formato si applica a tutte le domande nella sessione. Claude include `preview` sulle opzioni in cui un confronto visivo aiuta (scelte di layout, schemi di colori) e lo omette dove non lo farebbe (conferme sì/no, scelte solo testo). Controlla `undefined` prima di eseguire il rendering.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me choose a card layout",
  options: {
    toolConfig: {
      askUserQuestion: { previewFormat: "html" }
    },
    canUseTool: async (toolName, input) => {
      // input.questions[].options[].preview è una stringa HTML o undefined
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  // ...
}
```

Un'opzione con un'anteprima HTML:

```json theme={null}
{
  "label": "Compact",
  "description": "Title and metric value only",
  "preview": "<div style=\"padding:12px;border:1px solid #ddd;border-radius:8px\"><div style=\"font-size:12px;color:#666\">Active users</div><div style=\"font-size:28px;font-weight:600\">1,284</div></div>"
}
```

<h3 id="response-format">
  Formato della risposta
</h3>

Restituisci un oggetto `answers` che mappa il campo `question` di ogni domanda all'`label` dell'opzione selezionata:

| Campo       | Descrizione                                                                                                 |
| ----------- | ----------------------------------------------------------------------------------------------------------- |
| `questions` | Passa l'array di domande originale (obbligatorio per l'elaborazione dello strumento)                        |
| `answers`   | Oggetto in cui le chiavi sono il testo della domanda e i valori sono le etichette selezionate               |
| `response`  | Risposta facoltativa in testo libero che l'utente ha digitato invece di rispondere alle domande strutturate |

Per le domande a selezione multipla, passa un array di etichette o uniscile con `", "`. Per input di testo libero per domanda, come un'opzione "Other", inserisci il testo dell'utente in `answers[question]` come mostrato in [Supporta input di testo libero](#support-free-text-input). Imposta `response` solo quando la tua interfaccia utente consente all'utente di chiudere la scheda della domanda e digitare una risposta generale che non sia una risposta a nessuna domanda specifica. Quando `response` è impostato, Claude riceve "L'utente ha risposto: …" invece dell'elenco di risposte per domanda.

```json theme={null}
{
  "questions": [
    // ...
  ],
  "answers": {
    "How should I format the output?": "Summary",
    "Which sections should I include?": ["Introduction", "Conclusion"]
  }
}
```

<h4 id="support-free-text-input">
  Supporta input di testo libero
</h4>

Le opzioni predefinite di Claude non copriranno sempre ciò che gli utenti vogliono. Per consentire agli utenti di digitare la propria risposta:

* Visualizza una scelta "Other" aggiuntiva dopo le opzioni di Claude che accetta input di testo
* Utilizza il testo personalizzato dell'utente come valore della risposta (non la parola "Other")

Vedi l'[esempio completo](#complete-example) di seguito per un'implementazione completa.

<h3 id="complete-example">
  Esempio completo
</h3>

Claude pone domande di chiarimento quando ha bisogno di input dell'utente per procedere. Ad esempio, quando gli viene chiesto di aiutare a decidere su uno stack tecnologico per un'app mobile, Claude potrebbe chiedere informazioni su cross-platform vs nativo, preferenze di backend o piattaforme di destinazione. Queste domande aiutano Claude a prendere decisioni che corrispondono alle preferenze dell'utente piuttosto che indovinare.

Questo esempio gestisce quelle domande in un'applicazione terminale. Ecco cosa accade ad ogni passaggio:

1. **Instrada la richiesta**: Il callback `canUseTool` controlla se il nome dello strumento è `"AskUserQuestion"` e instrada a un gestore dedicato
2. **Visualizza le domande**: Il gestore scorre l'array `questions` e stampa ogni domanda con opzioni numerate
3. **Raccogli input**: L'utente può inserire un numero per selezionare un'opzione, o digitare testo libero direttamente (ad es. "jquery", "i don't know")
4. **Mappa le risposte**: Il codice controlla se l'input è numerico (utilizza l'etichetta dell'opzione) o testo libero (utilizza il testo direttamente)
5. **Restituisci a Claude**: La risposta include sia l'array `questions` originale che la mappatura `answers`

Salva la versione TypeScript come `ask.ts` ed eseguila con `npx tsx ask.ts`, oppure salva la versione Python come `ask.py` ed eseguila con `python ask.py`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import HookMatcher, PermissionResultAllow


  def parse_response(response: str, options: list) -> str:
      """Analizza l'input dell'utente come numero(i) di opzione o testo libero."""
      try:
          indices = [int(s.strip()) - 1 for s in response.split(",")]
          labels = [options[i]["label"] for i in indices if 0 <= i < len(options)]
          return ", ".join(labels) if labels else response
      except ValueError:
          return response


  async def handle_ask_user_question(input_data: dict) -> PermissionResultAllow:
      """Visualizza le domande di Claude e raccogli le risposte dell'utente."""
      answers = {}

      for q in input_data.get("questions", []):
          print(f"\n{q['header']}: {q['question']}")

          options = q["options"]
          for i, opt in enumerate(options):
              print(f"  {i + 1}. {opt['label']} - {opt['description']}")
          if q.get("multiSelect"):
              print("  (Enter numbers separated by commas, or type your own answer)")
          else:
              print("  (Enter a number, or type your own answer)")

          response = input("Your choice: ").strip()
          answers[q["question"]] = parse_response(response, options)

      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": answers,
          }
      )


  async def can_use_tool(
      tool_name: str, input_data: dict, context
  ) -> PermissionResultAllow:
      # Instrada AskUserQuestion al nostro gestore di domande
      if tool_name == "AskUserQuestion":
          return await handle_ask_user_question(input_data)
      # Auto-approva altri strumenti per questo esempio
      return PermissionResultAllow(updated_input=input_data)


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Help me decide on the tech stack for a new mobile app",
          },
      }


  # Workaround richiesto: dummy hook mantiene il flusso aperto per can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline/promises";

  // Helper per chiedere all'utente l'input nel terminale
  async function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
    const answer = await rl.question(question);
    rl.close();
    return answer;
  }

  // Analizza l'input dell'utente come numero(i) di opzione o testo libero
  function parseResponse(response: string, options: any[]): string {
    const indices = response.split(",").map((s) => parseInt(s.trim()) - 1);
    const labels = indices
      .filter((i) => !isNaN(i) && i >= 0 && i < options.length)
      .map((i) => options[i].label);
    return labels.length > 0 ? labels.join(", ") : response;
  }

  // Visualizza le domande di Claude e raccogli le risposte dell'utente
  async function handleAskUserQuestion(input: any) {
    const answers: Record<string, string> = {};

    for (const q of input.questions) {
      console.log(`\n${q.header}: ${q.question}`);

      const options = q.options;
      options.forEach((opt: any, i: number) => {
        console.log(`  ${i + 1}. ${opt.label} - ${opt.description}`);
      });
      if (q.multiSelect) {
        console.log("  (Enter numbers separated by commas, or type your own answer)");
      } else {
        console.log("  (Enter a number, or type your own answer)");
      }

      const response = (await prompt("Your choice: ")).trim();
      answers[q.question] = parseResponse(response, options);
    }

    // Restituisci le risposte a Claude (deve includere le domande originali)
    return {
      behavior: "allow",
      updatedInput: { questions: input.questions, answers }
    };
  }

  async function main() {
    for await (const message of query({
      prompt: "Help me decide on the tech stack for a new mobile app",
      options: {
        canUseTool: async (toolName, input) => {
          // Instrada AskUserQuestion al nostro gestore di domande
          if (toolName === "AskUserQuestion") {
            return handleAskUserQuestion(input);
          }
          // Auto-approva altri strumenti per questo esempio
          return { behavior: "allow", updatedInput: input };
        }
      }
    })) {
      if ("result" in message) console.log(message.result);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="limitations">
  Limitazioni
</h2>

* **Subagenti**: `AskUserQuestion` non è attualmente disponibile nei subagenti generati tramite lo strumento Agent
* **Limiti delle domande**: ogni chiamata `AskUserQuestion` supporta 1-4 domande con 2-4 opzioni ciascuna

<h2 id="other-ways-to-get-user-input">
  Altri modi per ottenere input dall'utente
</h2>

Il callback `canUseTool` e lo strumento `AskUserQuestion` coprono la maggior parte degli scenari di approvazione e chiarimento, ma l'SDK offre altri modi per ottenere input dagli utenti:

<h3 id="streaming-input">
  Input streaming
</h3>

Utilizza [input streaming](/docs/it/agent-sdk/streaming-vs-single-mode) quando hai bisogno di:

* **Interrompere l'agente a metà attività**: invia un segnale di annullamento o cambia direzione mentre Claude sta lavorando
* **Fornire contesto aggiuntivo**: aggiungi informazioni di cui Claude ha bisogno senza aspettare che le chieda
* **Costruire interfacce di chat**: consenti agli utenti di inviare messaggi di follow-up durante operazioni di lunga durata

L'input streaming è ideale per interfacce conversazionali in cui gli utenti interagiscono con l'agente durante l'esecuzione, non solo nei checkpoint di approvazione.

<h3 id="custom-tools">
  Strumenti personalizzati
</h3>

Utilizza [strumenti personalizzati](/docs/it/agent-sdk/custom-tools) quando hai bisogno di:

* **Raccogliere input strutturato**: costruisci moduli, procedure guidate o flussi di lavoro multi-step che vanno oltre il formato a scelta multipla di `AskUserQuestion`
* **Integrare sistemi di approvazione esterni**: connettiti a piattaforme di ticketing, flusso di lavoro o approvazione esistenti
* **Implementare interazioni specifiche del dominio**: crea strumenti personalizzati per le esigenze della tua applicazione, come interfacce di revisione del codice o elenchi di controllo di distribuzione

Gli strumenti personalizzati ti danno il controllo completo sull'interazione, ma richiedono più lavoro di implementazione rispetto all'utilizzo del callback `canUseTool` integrato.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Configura autorizzazioni](/docs/it/agent-sdk/permissions): configura modalità e regole di autorizzazione
* [Controlla l'esecuzione con hook](/docs/it/agent-sdk/hooks): esegui codice personalizzato nei punti chiave del ciclo di vita dell'agente
* [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript#canusetool): documentazione API completa di canUseTool
