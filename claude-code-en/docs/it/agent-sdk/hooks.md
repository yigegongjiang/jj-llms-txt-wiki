> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Intercettare e controllare il comportamento dell'agente con hooks

> Intercettare e personalizzare il comportamento dell'agente nei punti chiave di esecuzione con hooks

Gli hooks sono funzioni di callback che eseguono il vostro codice in risposta agli eventi dell'agente, come una chiamata a uno strumento, l'avvio di una sessione o l'arresto dell'esecuzione. Con gli hooks, potete:

* **Bloccare operazioni pericolose** prima che vengano eseguite, come comandi shell distruttivi o accesso a file non autorizzato
* **Registrare e controllare** ogni chiamata a uno strumento per conformità, debug o analitiche
* **Trasformare input e output** per sanitizzare i dati, iniettare credenziali o reindirizzare i percorsi dei file
* **Richiedere approvazione umana** per azioni sensibili come scritture su database o chiamate API
* **Tracciare il ciclo di vita della sessione** per gestire lo stato, pulire le risorse o inviare notifiche

Questa guida copre come funzionano gli hooks, come configurarli e fornisce esempi per modelli comuni come il blocco di strumenti, la modifica degli input e l'inoltro di notifiche.

<h2 id="how-hooks-work">
  Come funzionano gli hooks
</h2>

<Steps>
  <Step title="Un evento si attiva">
    Qualcosa accade durante l'esecuzione dell'agente e l'SDK attiva un evento: uno strumento sta per essere chiamato (`PreToolUse`), uno strumento ha restituito un risultato (`PostToolUse`), un subagente è stato avviato o interrotto, l'agente è inattivo o l'esecuzione è terminata. Consultate l'[elenco completo degli eventi](#available-hooks).
  </Step>

  <Step title="L'SDK raccoglie gli hooks registrati">
    L'SDK verifica la presenza di hooks registrati per quel tipo di evento. Questo include gli hooks di callback che passate in `options.hooks` e gli hooks dei comandi shell dai file di impostazioni quando la voce [`settingSources`](/docs/it/agent-sdk/typescript#settingsource) o [`setting_sources`](/docs/it/agent-sdk/python#settingsource) corrispondente è abilitata, come avviene per le opzioni predefinite di `query()`.
  </Step>

  <Step title="I matcher filtrano quali hooks vengono eseguiti">
    Se un hook ha un modello [`matcher`](#matchers) (come `"Write|Edit"`), l'SDK lo testa rispetto al target dell'evento (ad esempio, il nome dello strumento). Gli hooks senza un matcher vengono eseguiti per ogni evento di quel tipo.
  </Step>

  <Step title="Le funzioni di callback vengono eseguite">
    Ogni hook corrispondente riceve la sua [funzione di callback](#callback-functions) con input su ciò che sta accadendo: il nome dello strumento, i suoi argomenti, l'ID della sessione e altri dettagli specifici dell'evento.
  </Step>

  <Step title="Il vostro callback restituisce una decisione">
    Dopo aver eseguito qualsiasi operazione (registrazione, chiamate API, convalida), il vostro callback restituisce un [oggetto di output](#outputs) che dice all'agente cosa fare: consentire l'operazione, bloccarla, modificare l'input o iniettare contesto nella conversazione.
  </Step>
</Steps>

L'esempio seguente mette insieme questi passaggi. Registra un hook `PreToolUse` (passaggio 1) con un matcher `"Write|Edit"` (passaggio 3) in modo che il callback si attivi solo per gli strumenti di scrittura di file. Quando attivato, il callback riceve l'input dello strumento (passaggio 4), verifica se il percorso del file è destinato a un file `.env` e restituisce `permissionDecision: "deny"` per bloccare l'operazione (passaggio 5):

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeSDKClient,
      ClaudeAgentOptions,
      HookMatcher,
      ResultMessage,
  )


  # Define a hook callback that receives tool call details
  async def protect_env_files(input_data, tool_use_id, context):
      # Extract the file path from the tool's input arguments
      file_path = input_data["tool_input"].get("file_path", "")
      file_name = file_path.split("/")[-1]

      # Block the operation if targeting a .env file
      if file_name == ".env":
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Cannot modify .env files",
              }
          }

      # Return empty object to allow the operation
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for PreToolUse events
              # The matcher filters to only Write and Edit tool calls
              "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
          }
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Update the database configuration")
          async for message in client.receive_response():
              # Filter for assistant and result messages
              if isinstance(message, (AssistantMessage, ResultMessage)):
                  print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback with the HookCallback type
  const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast input to the specific hook type for type safety
    const preInput = input as PreToolUseHookInput;

    // Cast tool_input to access its properties (typed as unknown in the SDK)
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;
    const fileName = filePath?.split("/").pop();

    // Block the operation if targeting a .env file
    if (fileName === ".env") {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Cannot modify .env files"
        }
      };
    }

    // Return empty object to allow the operation
    return {};
  };

  for await (const message of query({
    prompt: "Update the database configuration",
    options: {
      hooks: {
        // Register the hook for PreToolUse events
        // The matcher filters to only Write and Edit tool calls
        PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
      }
    }
  })) {
    // Filter for assistant and result messages
    if (message.type === "assistant" || message.type === "result") {
      console.log(message);
    }
  }
  ```
</CodeGroup>

<h2 id="available-hooks">
  Hook disponibili
</h2>

L'SDK fornisce hooks per diverse fasi dell'esecuzione dell'agente. Alcuni hooks sono disponibili in entrambi gli SDK, mentre altri sono solo per TypeScript.

| Hook Event                                             | Python SDK | TypeScript SDK | Cosa lo attiva                                                                                                   | Caso d'uso di esempio                                                                              |
| ------------------------------------------------------ | ---------- | -------------- | ---------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `PreToolUse`                                           | Sì         | Sì             | Richiesta di chiamata a uno strumento (può bloccare o modificare)                                                | Bloccare comandi shell pericolosi                                                                  |
| `PostToolUse`                                          | Sì         | Sì             | Risultato dell'esecuzione dello strumento                                                                        | Registrare tutte le modifiche ai file nel registro di controllo                                    |
| `PostToolUseFailure`                                   | Sì         | Sì             | Errore di esecuzione dello strumento                                                                             | Gestire o registrare errori dello strumento                                                        |
| `PostToolBatch`                                        | No         | Sì             | Un intero batch di chiamate a strumenti si risolve, una volta per batch prima della prossima chiamata al modello | Iniettare convenzioni una volta per l'intero batch                                                 |
| `UserPromptSubmit`                                     | Sì         | Sì             | Invio del prompt dell'utente                                                                                     | Iniettare contesto aggiuntivo nei prompt                                                           |
| [`UserPromptExpansion`](/docs/it/hooks#userpromptexpansion) | No         | Sì             | Un comando digitato dall'utente si espande in un prompt prima di raggiungere Claude                              | Bloccare un comando dall'invocazione diretta o aggiungere contesto quando viene digitata una skill |
| `MessageDisplay`                                       | No         | Sì             | Un messaggio dell'assistente con testo si completa, una volta per messaggio con il testo completo del messaggio  | Oscurare o riformattare il testo visualizzato senza modificare la trascrizione                     |
| `Stop`                                                 | Sì         | Sì             | Arresto dell'esecuzione dell'agente                                                                              | Salvare lo stato della sessione prima dell'uscita                                                  |
| `SubagentStart`                                        | Sì         | Sì             | Inizializzazione del subagente                                                                                   | Tracciare l'avvio di attività parallele                                                            |
| `SubagentStop`                                         | Sì         | Sì             | Completamento del subagente                                                                                      | Aggregare i risultati dalle attività parallele                                                     |
| `PreCompact`                                           | Sì         | Sì             | Richiesta di compattazione della conversazione                                                                   | Archiviare la trascrizione completa prima del riepilogo                                            |
| `PermissionRequest`                                    | Sì         | Sì             | La finestra di dialogo delle autorizzazioni verrebbe visualizzata                                                | Gestione personalizzata delle autorizzazioni                                                       |
| `SessionStart`                                         | No         | Sì             | Inizializzazione della sessione                                                                                  | Inizializzare la registrazione e la telemetria                                                     |
| `SessionEnd`                                           | No         | Sì             | Terminazione della sessione                                                                                      | Pulire le risorse temporanee                                                                       |
| `Notification`                                         | Sì         | Sì             | Messaggi di stato dell'agente                                                                                    | Inviare aggiornamenti dello stato dell'agente a Slack o PagerDuty                                  |
| `Setup`                                                | No         | Sì             | Configurazione/manutenzione della sessione                                                                       | Eseguire attività di inizializzazione                                                              |
| `TeammateIdle`                                         | No         | Sì             | Il compagno di squadra diventa inattivo                                                                          | Riassegnare il lavoro o notificare                                                                 |
| `TaskCompleted`                                        | No         | Sì             | L'attività in background si completa                                                                             | Aggregare i risultati dalle attività parallele                                                     |
| `ConfigChange`                                         | No         | Sì             | Il file di configurazione cambia                                                                                 | Ricaricare le impostazioni dinamicamente                                                           |
| `WorktreeCreate`                                       | No         | Sì             | Git worktree creato                                                                                              | Tracciare gli spazi di lavoro isolati                                                              |
| `WorktreeRemove`                                       | No         | Sì             | Git worktree rimosso                                                                                             | Pulire le risorse dello spazio di lavoro                                                           |

<h2 id="configure-hooks">
  Configurare gli hooks
</h2>

Per configurare un hook, passatelo nel campo `hooks` delle opzioni dell'agente (`ClaudeAgentOptions` in Python, l'oggetto `options` in TypeScript):

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
  )

  async with ClaudeSDKClient(options=options) as client:
      await client.query("Your prompt")
      async for message in client.receive_response():
          print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Your prompt",
    options: {
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

L'opzione `hooks` è un dizionario in Python o un oggetto in TypeScript, dove:

* **Le chiavi**: [nomi degli eventi hook](#available-hooks) come `'PreToolUse'`, `'PostToolUse'` e `'Stop'`
* **I valori**: array di [matcher](#matchers), ognuno contenente un modello di filtro opzionale e le vostre [funzioni di callback](#callback-functions)

<h3 id="matchers">
  Matchers
</h3>

Utilizzate i matcher per filtrare quando i vostri callback si attivano. Il campo `matcher` corrisponde a un valore diverso a seconda del tipo di evento hook. Ad esempio, gli hook basati su strumenti corrispondono al nome dello strumento, mentre gli hook `Notification` corrispondono al tipo di notifica. Consultate il [riferimento degli hooks di Claude Code](/docs/it/hooks#matcher-patterns) per l'elenco completo dei valori di matcher per ogni tipo di evento.

I matcher SDK seguono le stesse regole dei [matcher nei file di impostazioni](/docs/it/hooks#matcher-patterns). Un matcher contenente solo lettere, cifre, `_`, `-`, spazi, `,` e `|` viene confrontato come una stringa esatta, con alternative separate da `|` o `,` e spazi bianchi opzionali circostanti, quindi `Write|Edit` e `Write, Edit` corrispondono esattamente a questi due strumenti e `code-reviewer` corrisponde solo a quel tipo di agente. Un matcher di `*`, una stringa vuota, o l'omissione del matcher interamente corrisponde a ogni occorrenza dell'evento.

Un matcher contenente qualsiasi altro carattere viene valutato come un'espressione regolare non ancorata, quindi `^mcp__` corrisponde a ogni strumento MCP e `Edit.*` corrisponde sia a `Edit` che a `NotebookEdit`. Avvolgete un'espressione regolare in `^` e `$` quando avete bisogno di una corrispondenza di intera stringa.

Un matcher come `mcp__memory` o `mcp__brave-search` contiene solo caratteri di corrispondenza esatta, quindi viene confrontato come una stringa esatta e non corrisponde a nessuno strumento; utilizzate `mcp__memory__.*` per corrispondere a ogni strumento da quel server.

I trattini nel set di corrispondenza esatta richiedono un runtime di Claude Code v2.1.195 o successivo. Nelle versioni precedenti, un nome con trattino come `code-reviewer` viene valutato come un'espressione regolare non ancorata e deve essere ancorato come `^code-reviewer$` per corrispondere esattamente.

| Opzione   | Tipo             | Predefinito | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| --------- | ---------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `matcher` | `string`         | `undefined` | Modello abbinato al campo di filtro dell'evento, seguendo le regole di confronto sopra. Per gli hook degli strumenti, questo è il nome dello strumento. Gli strumenti incorporati includono `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Agent` e altri (consultate [Tipi di input degli strumenti](/docs/it/agent-sdk/typescript#tool-input-types) per l'elenco completo). Gli strumenti MCP utilizzano il modello `mcp__<server>__<action>`. |
| `hooks`   | `HookCallback[]` | -           | Obbligatorio. Array di funzioni di callback da eseguire quando il modello corrisponde                                                                                                                                                                                                                                                                                                                                                                      |
| `timeout` | `number`         | `60`        | Timeout in secondi                                                                                                                                                                                                                                                                                                                                                                                                                                         |

Utilizzate il modello `matcher` per indirizzare strumenti specifici quando possibile. Un matcher con `'Bash'` viene eseguito solo per i comandi Bash, mentre omettere il modello esegue i vostri callback per ogni occorrenza dell'evento.

Per gli hook basati su strumenti, i matcher filtrano solo per nome dello strumento, non per percorsi di file o altri argomenti. Per filtrare per percorso di file, controllate `tool_input.file_path` all'interno del vostro callback.

<Tip>
  **Scoprire i nomi degli strumenti:** Consultate [Tipi di input degli strumenti](/docs/it/agent-sdk/typescript#tool-input-types) per l'elenco completo dei nomi degli strumenti incorporati, oppure aggiungete un hook senza un matcher per registrare tutte le chiamate agli strumenti che la vostra sessione effettua.

  **Denominazione degli strumenti MCP:** Gli strumenti MCP iniziano sempre con `mcp__` seguito dal nome del server e dall'azione: `mcp__<server>__<action>`. Ad esempio, se configurate un server denominato `playwright`, i suoi strumenti saranno denominati `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click` e così via. Il nome del server proviene dalla chiave che utilizzate nella configurazione `mcpServers`.
</Tip>

<h3 id="callback-functions">
  Funzioni di callback
</h3>

<h4 id="inputs">
  Input
</h4>

Ogni callback hook riceve tre argomenti:

* **Dati di input:** un oggetto tipizzato contenente i dettagli dell'evento. Ogni tipo di hook ha la sua forma di input. Ad esempio, `PreToolUseHookInput` include `tool_name` e `tool_input`, mentre `NotificationHookInput` include `message`. Consultate le definizioni di tipo complete nei riferimenti SDK [TypeScript](/docs/it/agent-sdk/typescript#hookinput) e [Python](/docs/it/agent-sdk/python#hookinput).
  * Tutti gli input hook condividono `session_id`, `cwd` e `hook_event_name`.
  * `agent_id` e `agent_type` vengono popolati quando l'hook si attiva all'interno di un subagente. In TypeScript, questi si trovano sull'input hook di base e sono disponibili per tutti i tipi di hook. In Python, sono campi opzionali su `PreToolUse`, `PostToolUse`, `PostToolUseFailure` e `PermissionRequest`, e campi obbligatori su `SubagentStart` e `SubagentStop`.
* **ID di utilizzo dello strumento** (`str | None` / `string | undefined`): correla gli eventi `PreToolUse` e `PostToolUse` per la stessa chiamata a uno strumento.
* **Contesto:** in TypeScript, contiene una proprietà `signal` (`AbortSignal`) per l'annullamento. In Python, questo argomento è riservato per uso futuro.

<h4 id="outputs">
  Output
</h4>

Il vostro callback restituisce un oggetto con due categorie di campi:

* **Campi di livello superiore** funzionano allo stesso modo su ogni evento: `systemMessage` mostra un messaggio all'utente, e `continue` (`continue_` in Python) determina se l'agente continua a funzionare dopo questo hook.
* **`hookSpecificOutput`** controlla l'operazione corrente. I campi all'interno dipendono dal tipo di evento hook. Per gli hook `PreToolUse`, è qui che impostate `permissionDecision` (`"allow"`, `"deny"`, `"ask"` o `"defer"`), `permissionDecisionReason` e `updatedInput`. Restituire `"defer"` termina la query in modo da poter [riprendere in seguito](/docs/it/hooks#defer-a-tool-call-for-later). Per gli hook `PostToolUse`, potete impostare `additionalContext` per aggiungere informazioni al risultato dello strumento. Per sostituire l'output dello strumento prima che Claude lo veda, impostate `updatedToolOutput`, che funziona per qualsiasi strumento in entrambi gli SDK. Il campo più vecchio `updatedMCPToolOutput` sostituisce solo l'output dello strumento MCP ed è deprecato.

Restituite `{}` per consentire l'operazione senza modifiche. Gli hook di callback SDK utilizzano lo stesso formato di output JSON degli [hook dei comandi shell di Claude Code](/docs/it/hooks#json-output), che documenta ogni campo e opzione specifica dell'evento. Per le definizioni di tipo SDK, consultate i riferimenti SDK [TypeScript](/docs/it/agent-sdk/typescript#synchookjsonoutput) e [Python](/docs/it/agent-sdk/python#synchookjsonoutput).

<Note>
  Quando si applicano più hook o regole di autorizzazione, `deny` ha priorità su `defer`, che ha priorità su `ask`, che ha priorità su `allow`. Se un hook restituisce `deny`, l'operazione viene bloccata indipendentemente dagli altri hook.
</Note>

<h4 id="asynchronous-output">
  Output asincrono
</h4>

Per impostazione predefinita, l'agente attende che il vostro hook restituisca prima di procedere. Se il vostro hook esegue un effetto collaterale, come la registrazione o l'invio di un webhook, e non ha bisogno di influenzare il comportamento dell'agente, potete restituire un output asincrono. Questo dice all'agente di continuare immediatamente senza attendere il completamento dell'hook:

<CodeGroup>
  ```python Python theme={null}
  async def async_hook(input_data, tool_use_id, context):
      # Start a background task, then return immediately
      asyncio.create_task(send_to_logging_service(input_data))
      return {"async_": True, "asyncTimeout": 30000}
  ```

  ```typescript TypeScript theme={null}
  const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
    // Start a background task, then return immediately
    sendToLoggingService(input).catch(console.error);
    return { async: true, asyncTimeout: 30000 };
  };
  ```
</CodeGroup>

| Campo          | Tipo     | Descrizione                                                                                                                             |
| -------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `async`        | `true`   | Segnala la modalità asincrona. L'agente procede senza attendere. In Python, utilizzate `async_` per evitare la parola chiave riservata. |
| `asyncTimeout` | `number` | Timeout opzionale in millisecondi per l'operazione in background                                                                        |

<Note>
  Gli output asincroni non possono bloccare, modificare o iniettare contesto nell'operazione poiché l'agente ha già proseguito. Utilizzateli solo per effetti collaterali come registrazione, metriche o notifiche.
</Note>

<h2 id="examples">
  Esempi
</h2>

<h3 id="modify-tool-input">
  Modificare l'input dello strumento
</h3>

Questo esempio intercetta le chiamate allo strumento Write e riscrive l'argomento `file_path` per anteporre `/sandbox`, reindirizzando tutte le scritture di file a una directory sandbox. Il callback restituisce `updatedInput` con il percorso modificato e `permissionDecision: 'allow'` per approvare automaticamente l'operazione riscritta:

<CodeGroup>
  ```python Python theme={null}
  async def redirect_to_sandbox(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      if input_data["tool_name"] == "Write":
          original_path = input_data["tool_input"].get("file_path", "")
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "updatedInput": {
                      **input_data["tool_input"],
                      "file_path": f"/sandbox{original_path}",
                  },
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    if (preInput.tool_name === "Write") {
      const originalPath = toolInput.file_path as string;
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          updatedInput: {
            ...toolInput,
            file_path: `/sandbox${originalPath}`
          }
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<Note>
  Quando utilizzate `updatedInput`, dovete anche includere `permissionDecision: 'allow'` per approvare automaticamente l'input modificato o `permissionDecision: 'ask'` per mostrarlo all'utente. Con `'defer'`, `updatedInput` viene ignorato. Restituite sempre un nuovo oggetto piuttosto che mutare l'originale `tool_input`.
</Note>

<h3 id="add-context-and-block-a-tool">
  Aggiungere contesto e bloccare uno strumento
</h3>

Questo esempio blocca le scritture nella directory `/etc` e spiega il motivo sia al modello che all'utente:

* `permissionDecision: 'deny'` interrompe la chiamata dello strumento.
* `permissionDecisionReason` comunica al modello il motivo, in modo che eviti di ritentare.
* `systemMessage` mostra all'utente cosa è accaduto.

<CodeGroup>
  ```python Python theme={null}
  async def block_etc_writes(input_data, tool_use_id, context):
      file_path = input_data["tool_input"].get("file_path", "")

      if file_path.startswith("/etc"):
          return {
              # Top-level field: message shown to the user
              "systemMessage": "Remember: system directories like /etc are protected.",
              # hookSpecificOutput: block the operation
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Writing to /etc is not allowed",
              },
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;

    if (filePath?.startsWith("/etc")) {
      return {
        // Top-level field: message shown to the user
        systemMessage: "Remember: system directories like /etc are protected.",
        // hookSpecificOutput: block the operation
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Writing to /etc is not allowed"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="auto-approve-specific-tools">
  Approvare automaticamente strumenti specifici
</h3>

Per impostazione predefinita, l'agente potrebbe richiedere l'autorizzazione prima di utilizzare determinati strumenti. Questo esempio approva automaticamente gli strumenti del file system di sola lettura (Read, Glob, Grep) restituendo `permissionDecision: 'allow'`, consentendo loro di funzionare senza conferma dell'utente mentre lascia tutti gli altri strumenti soggetti ai normali controlli di autorizzazione:

<CodeGroup>
  ```python Python theme={null}
  async def auto_approve_read_only(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      read_only_tools = ["Read", "Glob", "Grep"]
      if input_data["tool_name"] in read_only_tools:
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "permissionDecisionReason": "Read-only tool auto-approved",
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const readOnlyTools = ["Read", "Glob", "Grep"];
    if (readOnlyTools.includes(preInput.tool_name)) {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          permissionDecisionReason: "Read-only tool auto-approved"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="register-multiple-hooks">
  Registrare più hook
</h3>

Quando un evento si attiva, tutti gli hook corrispondenti vengono eseguiti in parallelo. Per le decisioni di autorizzazione, il risultato più restrittivo vince: un singolo `deny` blocca la chiamata dello strumento indipendentemente da ciò che gli altri hook restituiscono. Poiché l'ordine di completamento è non deterministico, scrivete ogni hook per agire in modo indipendente piuttosto che fare affidamento su un altro hook che sia stato eseguito per primo.

L'esempio seguente registra tre controlli indipendenti per ogni chiamata a uno strumento:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              HookMatcher(hooks=[authorization_check]),
              HookMatcher(hooks=[input_validator]),
              HookMatcher(hooks=[audit_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        { hooks: [authorizationCheck] },
        { hooks: [inputValidator] },
        { hooks: [auditLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="filter-with-multi-tool-matchers">
  Filtrare con matcher multi-strumento
</h3>

Utilizzate matcher multi-strumento per condividere un callback tra strumenti correlati. Questo esempio registra tre matcher con ambiti diversi:

* Un elenco esatto separato da pipe (`Write|Edit|Delete`) attiva `file_security_hook` solo per gli strumenti di modifica dei file.
* Un'espressione regolare (`^mcp__`) attiva `mcp_audit_hook` per qualsiasi strumento MCP il cui nome inizia con `mcp__`.
* Un matcher omesso attiva `global_logger` per ogni chiamata a uno strumento indipendentemente dal nome.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|Delete", hooks=[file_security_hook]),
              # Match all MCP tools
              HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
              # Match everything (no matcher)
              HookMatcher(hooks=[global_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        // Match file modification tools
        { matcher: "Write|Edit|Delete", hooks: [fileSecurityHook] },

        // Match all MCP tools
        { matcher: "^mcp__", hooks: [mcpAuditHook] },

        // Match everything (no matcher)
        { hooks: [globalLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="track-subagent-activity">
  Tracciare l'attività dei subagenti
</h3>

Utilizzate gli hook `SubagentStop` per monitorare quando i subagenti completano il loro lavoro. Consultate il tipo di input completo nei riferimenti SDK [TypeScript](/docs/it/agent-sdk/typescript#hookinput) e [Python](/docs/it/agent-sdk/python#hookinput). Questo esempio registra un riepilogo ogni volta che un subagente si completa:

<CodeGroup>
  ```python Python theme={null}
  async def subagent_tracker(input_data, tool_use_id, context):
      # Log subagent details when it finishes
      print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
      print(f"  Transcript: {input_data['agent_transcript_path']}")
      print(f"  Tool use ID: {tool_use_id}")
      print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
      return {}


  options = ClaudeAgentOptions(
      hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
  )
  ```

  ```typescript TypeScript theme={null}
  import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

  const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to SubagentStopHookInput to access subagent-specific fields
    const subInput = input as SubagentStopHookInput;

    // Log subagent details when it finishes
    console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
    console.log(`  Transcript: ${subInput.agent_transcript_path}`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
    return {};
  };

  const options = {
    hooks: {
      SubagentStop: [{ hooks: [subagentTracker] }]
    }
  };
  ```
</CodeGroup>

<h3 id="make-http-requests-from-hooks">
  Effettuare richieste HTTP dagli hooks
</h3>

Gli hooks possono eseguire operazioni asincrone come richieste HTTP. Catturate gli errori all'interno del vostro hook invece di lasciarli propagare, poiché un'eccezione non gestita può interrompere l'agente.

Questo esempio invia un webhook dopo il completamento di ogni strumento, registrando quale strumento è stato eseguito e quando. L'hook cattura gli errori in modo che un webhook non riuscito non interrompa l'agente:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request
  from datetime import datetime


  def _send_webhook(tool_name):
      """Synchronous helper that POSTs tool usage data to an external webhook."""
      data = json.dumps(
          {
              "tool": tool_name,
              "timestamp": datetime.now().isoformat(),
          }
      ).encode()
      req = urllib.request.Request(
          "https://api.example.com/webhook",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def webhook_notifier(input_data, tool_use_id, context):
      # Only fire after a tool completes (PostToolUse), not before
      if input_data["hook_event_name"] != "PostToolUse":
          return {}

      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_webhook, input_data["tool_name"])
      except Exception as e:
          # Log the error but don't raise. A failed webhook shouldn't stop the agent
          print(f"Webhook request failed: {e}")

      return {}
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
    // Only fire after a tool completes (PostToolUse), not before
    if (input.hook_event_name !== "PostToolUse") return {};

    try {
      await fetch("https://api.example.com/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tool: (input as PostToolUseHookInput).tool_name,
          timestamp: new Date().toISOString()
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      // Handle cancellation separately from other errors
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Webhook request cancelled");
      }
      // Don't re-throw. A failed webhook shouldn't stop the agent
    }

    return {};
  };

  // Register as a PostToolUse hook
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      hooks: {
        PostToolUse: [{ hooks: [webhookNotifier] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="forward-notifications-to-slack">
  Inoltrare le notifiche a Slack
</h3>

Utilizzate gli hook `Notification` per ricevere notifiche di sistema dall'agente e inoltrarle a servizi esterni. Le notifiche si attivano per tipi di evento specifici come:

* `permission_prompt` quando Claude ha bisogno di autorizzazione
* `idle_prompt` quando Claude è in attesa di input
* `auth_success` quando l'autenticazione si completa
* `elicitation_dialog`, `elicitation_complete` e `elicitation_response` per i flussi di elicitazione dell'input dell'utente

Ogni notifica include un campo `message` con una descrizione leggibile dall'uomo e facoltativamente un `title`.

Questo esempio inoltra ogni notifica a un canale Slack. Richiede un [URL webhook in arrivo di Slack](https://api.slack.com/messaging/webhooks), che create aggiungendo un'app al vostro spazio di lavoro Slack e abilitando i webhook in arrivo:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request

  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


  def _send_slack_notification(message):
      """Synchronous helper that sends a message to Slack via incoming webhook."""
      data = json.dumps({"text": f"Agent status: {message}"}).encode()
      req = urllib.request.Request(
          "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def notification_handler(input_data, tool_use_id, context):
      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
      except Exception as e:
          print(f"Failed to send notification: {e}")

      # Return empty object. Notification hooks don't modify agent behavior
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for Notification events (no matcher needed)
              "Notification": [HookMatcher(hooks=[notification_handler])],
          },
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Analyze this codebase")
          async for message in client.receive_response():
              print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback that sends notifications to Slack
  const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to NotificationHookInput to access the message field
    const notification = input as NotificationHookInput;

    try {
      // POST the notification message to a Slack incoming webhook
      await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: `Agent status: ${notification.message}`
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Notification cancelled");
      } else {
        console.error("Failed to send notification:", error);
      }
    }

    // Return empty object. Notification hooks don't modify agent behavior
    return {};
  };

  // Register the hook for Notification events (no matcher needed)
  for await (const message of query({
    prompt: "Analyze this codebase",
    options: {
      hooks: {
        Notification: [{ hooks: [notificationHandler] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="fix-common-issues">
  Risolvere i problemi comuni
</h2>

<h3 id="hook-not-firing">
  Hook non si attiva
</h3>

* Verificate che il nome dell'evento hook sia corretto e sensibile alle maiuscole (`PreToolUse`, non `preToolUse`)
* Controllate che il vostro modello di matcher corrisponda esattamente al nome dello strumento
* Assicuratevi che l'hook sia sotto il tipo di evento corretto in `options.hooks`
* Per gli hook non basati su strumenti che supportano matcher, come `Notification` e `SubagentStop`, i matcher corrispondono a campi diversi, e `Stop` ignora completamente i matcher (consultate [modelli di matcher](/docs/it/hooks#matcher-patterns))
* Gli hooks potrebbero non attivarsi quando l'agente raggiunge il limite [`max_turns`](/docs/it/agent-sdk/python#claudeagentoptions) perché la sessione termina prima che gli hooks possano essere eseguiti

<h3 id="matcher-not-filtering-as-expected">
  Matcher non filtra come previsto
</h3>

I matcher corrispondono solo ai nomi degli strumenti, non ai percorsi dei file o ad altri argomenti. Per filtrare per percorso di file, controllate `tool_input.file_path` all'interno del vostro hook:

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // Skip non-markdown files
  // Process markdown files...
  return {};
};
```

<h3 id="hook-timeout">
  Timeout dell'hook
</h3>

* Aumentate il valore `timeout` nella configurazione `HookMatcher`
* Utilizzate `AbortSignal` dal terzo argomento del callback per gestire l'annullamento con eleganza in TypeScript

Un callback `UserPromptSubmit` o [`UserPromptExpansion`](/docs/it/hooks#userpromptexpansion) che supera il suo timeout blocca quel prompt con un messaggio di timeout e la sessione continua. L'interruzione della query mentre un callback è in sospeso annulla la chiamata dello strumento in sospeso. Prima della v2.1.208, un timeout del callback su questi eventi terminava la query con `error_during_execution`, e un'interruzione durante un callback `PreToolUse` in sospeso poteva consentire alla chiamata dello strumento di procedere.

<h3 id="tool-blocked-unexpectedly">
  Strumento bloccato inaspettatamente
</h3>

* Controllate tutti gli hook `PreToolUse` per i ritorni `permissionDecision: 'deny'`
* Aggiungete la registrazione ai vostri hook per vedere quale `permissionDecisionReason` stanno restituendo
* Verificate che i modelli di matcher non siano troppo ampi: un matcher vuoto corrisponde a tutti gli strumenti

<h3 id="modified-input-not-applied">
  Input modificato non applicato
</h3>

* Assicuratevi che `updatedInput` sia all'interno di `hookSpecificOutput`, non al livello superiore:

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* Restituite `permissionDecision: 'allow'` per approvare automaticamente l'input modificato, oppure `'ask'` per mostrarlo all'utente per l'approvazione

* Includete `hookEventName` in `hookSpecificOutput` per identificare quale tipo di hook è l'output

<h3 id="session-hooks-not-available-in-python">
  Hook di sessione non disponibili in Python
</h3>

`SessionStart` e `SessionEnd` possono essere registrati come hook di callback SDK in TypeScript, ma non sono disponibili nell'SDK Python perché il suo tipo `HookEvent` li omette. In Python, sono disponibili solo come [hook dei comandi shell](/docs/it/hooks#hook-events) definiti nei file di impostazioni come `.claude/settings.json`. Per caricare gli hook dei comandi shell dalla vostra applicazione SDK, includete la fonte di impostazione appropriata con [`setting_sources`](/docs/it/agent-sdk/python#settingsource) o [`settingSources`](/docs/it/agent-sdk/typescript#settingsource):

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["project"],  # Loads .claude/settings.json including hooks
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    settingSources: ["project"] // Loads .claude/settings.json including hooks
  };
  ```
</CodeGroup>

Per eseguire la logica di inizializzazione come callback SDK Python, utilizzate il primo messaggio da `client.receive_response()` come trigger.

<h3 id="subagent-permission-prompts-multiplying">
  I prompt di autorizzazione dei subagenti si moltiplicano
</h3>

Quando si avviano più subagenti, ognuno potrebbe richiedere autorizzazioni separatamente. I subagenti non ereditano automaticamente le autorizzazioni dell'agente genitore. Per evitare prompt ripetuti, utilizzate gli hook `PreToolUse` per approvare automaticamente strumenti specifici o configurate regole di autorizzazione che si applicano alle sessioni dei subagenti.

<h3 id="recursive-hook-loops-with-subagents">
  Loop ricorsivi di hook con subagenti
</h3>

Un hook `UserPromptSubmit` che avvia subagenti può creare loop infiniti se quei subagenti attivano lo stesso hook. Per prevenire questo:

* Controllate un indicatore di subagente nell'input dell'hook prima di avviare
* Utilizzate una variabile condivisa o lo stato della sessione per tracciare se siete già all'interno di un subagente
* Limitate gli hook per l'esecuzione solo per la sessione dell'agente di livello superiore

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage non appare nell'output
</h3>

Il campo `systemMessage` mostra un messaggio all'utente, non al modello. Per impostazione predefinita, l'SDK fa emergere l'output degli hook nel flusso dei messaggi solo per gli hook `SessionStart` e `Setup`, quindi un messaggio da qualsiasi altro evento hook non appare a meno che non impostiate `includeHookEvents` (`include_hook_events` in Python). Per passare il contesto al modello, restituite [`additionalContext`](/docs/it/hooks#add-context-for-claude).

Se avete bisogno di far emergere le decisioni degli hook alla vostra applicazione in modo affidabile, registratele separatamente o utilizzate un canale di output dedicato.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Riferimento degli hooks di Claude Code](/docs/it/hooks): schemi JSON di input/output completi, documentazione degli eventi e modelli di matcher
* [Guida agli hooks di Claude Code](/docs/it/hooks-guide): esempi di hook dei comandi shell e procedure dettagliate
* [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript): tipi di hook, definizioni di input/output e opzioni di configurazione
* [Riferimento SDK Python](/docs/it/agent-sdk/python): tipi di hook, definizioni di input/output e opzioni di configurazione
* [Autorizzazioni](/docs/it/agent-sdk/permissions): controllare cosa può fare il vostro agente
* [Strumenti personalizzati](/docs/it/agent-sdk/custom-tools): creare strumenti per estendere le capacità dell'agente
