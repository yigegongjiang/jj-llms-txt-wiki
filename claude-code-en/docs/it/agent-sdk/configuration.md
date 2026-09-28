> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configura il tuo agente

> Configura le sessioni dell'Agent SDK: componi l'oggetto options, imposta il modello, l'ambiente e i limiti, e trova la pagina di ogni opzione di funzionalità.

Una sessione dell'Agent SDK legge la configurazione da file di impostazioni, variabili d'ambiente e dall'oggetto `options` che passi quando la avvii. Questa pagina mostra come comporre l'oggetto `options` e quali file di impostazioni e variabili d'ambiente lo controllano.

Per ogni tipo di opzione e valore predefinito, consulta i riferimenti [`Options`](/docs/it/agent-sdk/typescript#options) (TypeScript) e [`ClaudeAgentOptions`](/docs/it/agent-sdk/python#claudeagentoptions) (Python).

<h2 id="pass-options-to-a-session">
  Passa le opzioni a una sessione
</h2>

Ogni chiamata `query()` accetta un oggetto options: `Options` in TypeScript, `ClaudeAgentOptions` in Python. Ogni campo è facoltativo e una sessione avviata senza opzioni viene eseguita con i valori predefiniti dell'SDK. L'esempio seguente configura una sessione di sola lettura che riassume i TODO aperti di un progetto. Le coppie si leggono come TypeScript / Python dove gli spelling differiscono:

* **`model`**: sceglie il modello
* **`allowedTools` / `allowed_tools`**: pre-approva un elenco di strumenti di sola lettura
* **`maxTurns` / `max_turns`**: limita il numero di turni
* **`cwd`**: imposta la directory di lavoro

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the open TODOs in this repo",
    options: {
      model: "claude-sonnet-5",
      allowedTools: ["Read", "Glob", "Grep"],
      maxTurns: 8,
      cwd: "/path/to/repo",
    },
  })) {
    if (message.type === "result" && message.subtype === "success" && !message.is_error) {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

  async def main():
      options = ClaudeAgentOptions(
          model="claude-sonnet-5",
          allowed_tools=["Read", "Glob", "Grep"],
          max_turns=8,
          cwd="/path/to/repo",
      )

      async for message in query(
          prompt="Summarize the open TODOs in this repo",
          options=options,
      ):
          if isinstance(message, ResultMessage) and not message.is_error:
              print(message.result)

  asyncio.run(main())
  ```
</CodeGroup>

Puntate `cwd` a uno dei vostri progetti e eseguite l'esempio. Il riassunto dei TODO aperti di quel progetto viene stampato quando arriva il messaggio di risultato.

`allowedTools` (TypeScript) o `allowed_tools` (Python) pre-approva gli strumenti elencati, quindi le chiamate a essi vengono eseguite senza fermarsi per l'approvazione. Gli strumenti al di fuori dell'elenco rimangono disponibili. Quando Claude chiama uno strumento non elencato, la modalità di autorizzazione decide se la chiamata viene eseguita. Per ulteriori informazioni, consultate [Allow and deny rules](/docs/it/agent-sdk/permissions#allow-and-deny-rules).

<h2 id="load-settings-files">
  Carica i file di impostazioni
</h2>

I file di impostazioni forniscono configurazioni oltre all'oggetto options. Due opzioni controllano come vengono caricati:

* **`settingSources` / `setting_sources`**: controlla quali fonti del filesystem caricano: user, project e local. I file di impostazioni e i file CLAUDE.md arrivano attraverso queste fonti.
* **`settings`**: carica un percorso di file di impostazioni o una stringa JSON inline in entrambi i linguaggi, e TypeScript accetta anche un oggetto settings. Qualunque forma passiate sostituisce le impostazioni del filesystem user, project e local; solo le impostazioni di policy gestite hanno una priorità più alta. I riferimenti documentano l'ordine di precedenza completo in [Settings precedence](/docs/it/agent-sdk/typescript#settings-precedence) per TypeScript e [Settings precedence](/docs/it/agent-sdk/python#settings-precedence) per Python.

Passate `[]` per disabilitare le impostazioni user, project e local. Per ulteriori informazioni, consultate [Use Claude Code features in the SDK](/docs/it/agent-sdk/claude-code-features).

<h2 id="choose-a-model">
  Scegli un modello
</h2>

A meno che l'opzione `model`, le vostre impostazioni o il vostro ambiente non selezionino un modello, una nuova sessione si avvia sul [modello predefinito di Claude Code](/docs/it/model-config#default-model-setting). Per l'ordine di queste fonti, consultate [Setting your model](/docs/it/model-config#setting-your-model). Impostate `model` per fissare un modello specifico, o per sceglierne uno più piccolo per agenti più veloci e economici. Il valore accetta un alias di modello o un nome di modello completo; gli alias e le versioni a cui si risolvono sono elencati in [Model aliases](/docs/it/model-config#model-aliases).

Impostate `fallbackModel` (TypeScript) o `fallback_model` (Python) per nominare un modello di backup. Quando il primario è sovraccarico o non disponibile, la sessione passa al backup. Il primario viene ritentato all'inizio di ogni turno dell'utente, quindi la sessione ritorna ad esso una volta che l'interruzione passa.

In entrambi i linguaggi, l'opzione accetta un singolo modello o un elenco separato da virgole di backup. Per l'ordine e il limite della catena, consultate [Fallback model chains](/docs/it/model-config#fallback-model-chains). In TypeScript, un fallback uguale a `model` genera un errore all'avvio.

Gli esempi seguenti mostrano un elenco di fallback in TypeScript e un singolo fallback in Python:

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    model: "claude-fable-5",
    fallbackModel: "claude-opus-5,claude-sonnet-5",
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      model="claude-fable-5",
      fallback_model="claude-opus-5",
  )
  ```
</CodeGroup>

<span id="sampling-parameters" />

<Note>
  I parametri della richiesta dell'[API Messages](https://platform.claude.com/docs/en/api/messages) `temperature`, `top_p` e `max_tokens` non hanno campi sull'oggetto options in nessuno dei due linguaggi. Impostate il [livello di sforzo](/docs/it/agent-sdk/agent-loop#effort-level) o un [limite di spesa](#limit-turns-and-spend) invece, oppure chiamate l'API Messages quando avete bisogno di quei parametri direttamente.
</Note>

<h2 id="set-environment-variables">
  Imposta le variabili d'ambiente
</h2>

L'opzione `env` imposta le variabili d'ambiente per il processo Claude Code che esegue la vostra sessione. Se i vostri valori sostituiscono l'ambiente ereditato o si uniscono ad esso differisce per linguaggio:

* **TypeScript**: `env` sostituisce l'ambiente del subprocess
* **Python**: l'SDK unisce i vostri valori all'ambiente ereditato, e i vostri valori sostituiscono quelli ereditati

In TypeScript, diffondete `process.env` in `env` per mantenere le variabili ereditate come `PATH`, `HOME` e `ANTHROPIC_API_KEY`. Quando lasciate `env` non impostato, il subprocess eredita il vostro ambiente in entrambi i linguaggi.

L'esempio instrada il traffico API attraverso un gateway impostando `ANTHROPIC_BASE_URL`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
  )
  ```
</CodeGroup>

Le variabili che passate possono anche configurare Claude Code stesso. Per le variabili che il processo Claude Code legge, consultate [Environment variables](/docs/it/env-vars). Per sintonizzare i timeout dell'API e il rilevamento di stallo in questo modo, seguite la sezione Handle slow or stalled API responses nel riferimento [TypeScript](/docs/it/agent-sdk/typescript#handle-slow-or-stalled-api-responses) o nel riferimento [Python](/docs/it/agent-sdk/python#handle-slow-or-stalled-api-responses).

<h2 id="set-the-working-directory">
  Imposta la directory di lavoro
</h2>

Impostate `cwd` per eseguire la sessione in una directory specifica. Quando lasciate `cwd` non impostato, la sessione viene eseguita nella directory di lavoro del vostro processo. Nessuno dei due SDK ha un setter per `cwd`. Per eseguire in una directory diversa, avviate un'altra sessione con quel `cwd`.

Claude Code legge la directory di lavoro per determinare:

* **Project settings and hooks**: quale [impostazioni e hook del progetto caricano](/docs/it/agent-sdk/claude-code-features)
* **Skills**: dove [le skill della sessione vengono scoperte](/docs/it/agent-sdk/skills)
* **Session storage**: quale progetto [una sessione memorizzata appartiene a](/docs/it/agent-sdk/session-storage)

Per consentire agli strumenti di raggiungere file al di fuori della directory di lavoro, aggiungete percorsi con `additionalDirectories` (TypeScript) o `add_dirs` (Python). Per l'ambito di quella concessione, consultate [Additional directories grant file access, not configuration](/docs/it/permissions#additional-directories-grant-file-access-not-configuration).

<h2 id="limit-turns-and-spend">
  Limita i turni e la spesa
</h2>

Limitate i turni e la spesa con `maxTurns` / `max_turns` e `maxBudgetUsd` / `max_budget_usd`. Entrambi i limiti sono disattivati quando non impostati. Quando una sessione raggiunge un limite, l'esecuzione termina con un messaggio di risultato il cui sottotipo nomina il limite, `error_max_turns` o `error_max_budget_usd`. Quello che succede dopo differisce per modalità di input:

* **Single-shot `query()`**: l'SDK produce il risultato del limite e poi genera un'eccezione, quindi avvolgete il ciclo in un blocco try per continuare oltre l'errore
* **Streaming input**: la sessione rimane attiva oltre un risultato di limite, e il conteggio dei turni massimi ricomincia per ogni messaggio in coda. Il totale del budget si accumula tra i messaggi, e una volta che la spesa raggiunge il limite, i messaggi successivi nella stessa conversazione terminano con lo stesso risultato di budget. Un [`/clear`](/docs/it/agent-sdk/cost-tracking) ricomincia il budget

I due limiti trattano `0` diversamente:

* **`maxTurns` / `max_turns`**: `0` esegue la sessione senza un limite di turni, lo stesso che lasciare l'opzione non impostata
* **`maxBudgetUsd` / `max_budget_usd`**: la CLI rifiuta `0` come importo non valido all'avvio, e la sessione non viene mai eseguita

Per ulteriori informazioni su entrambi i limiti, inclusa la spesa dei subagenti, consultate [Turns and budget](/docs/it/agent-sdk/agent-loop#turns-and-budget).

<h2 id="change-configuration-mid-session">
  Cambia la configurazione durante la sessione
</h2>

Quando avviate una sessione con [streaming input](/docs/it/agent-sdk/streaming-vs-single-mode), potete cambiare il suo modello e la modalità di autorizzazione mentre è in esecuzione. Dove chiamate i setter differisce per linguaggio:

* **TypeScript**: metodi sull'oggetto che `query()` restituisce
* **Python**: metodi su [`ClaudeSDKClient`](/docs/it/agent-sdk/python#claudesdkclient), poiché `query()` restituisce un iteratore semplice senza metodi di controllo

Entrambi i linguaggi hanno gli stessi setter:

* **`setModel()` / `set_model()`**: cambia il modello. Chiamatelo senza modello per passare al [modello predefinito di Claude Code](/docs/it/model-config#default-model-setting) piuttosto che al `model` che avete passato nelle opzioni.
* **`setPermissionMode()` / `set_permission_mode()`**: cambia la modalità di autorizzazione

TypeScript ha anche `applyFlagSettings()` e `updateSettings()`:

* **`applyFlagSettings()`**: applica le impostazioni in fase di esecuzione, come in `await session.applyFlagSettings({ effortLevel: "high" })`. Il metodo accetta chiavi di file di impostazioni piuttosto che campi di opzioni, quindi controllate il riferimento [`applyFlagSettings()`](/docs/it/agent-sdk/typescript#applyflagsettings) per lo schema e per quali chiavi hanno effetto durante la sessione.
* **`updateSettings()`**: scrive una chiave consentita in un file di impostazioni. Il riferimento [`updateSettings()`](/docs/it/agent-sdk/typescript#updatesettings) nomina la chiave che ogni sorgente accetta e il floor della versione.
  * Passate `"localSettings"` per scrivere il file di impostazioni locali del progetto, come in `await session.updateSettings("localSettings", { outputStyle: "Explanatory" })`. La chiave scritta ha effetto sulla richiesta successiva della sessione e persiste per le sessioni successive che caricano le impostazioni `local`.
  * Passate `"userSettings"` per scrivere `effortLevel`, l'unica chiave che la sorgente accetta. Claude Code la salva come il livello di sforzo predefinito per il modello corrente della sessione, e lo sforzo della sessione in esecuzione non cambia.

L'esempio seguente esegue una sessione a due turni, cambia la configurazione tra i turni e stampa il modello che ha risposto a ogni turno. In TypeScript, il flusso del prompt tiene il secondo messaggio fino a quando i setter non hanno funzionato, e il secondo turno viene eseguito sul nuovo modello.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

  function userMessage(text: string): SDKUserMessage {
    return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
  }

  // Hold the second prompt until the setters have run.
  let startSecondTurn!: () => void;
  const secondTurnReady = new Promise<void>((resolve) => {
    startSecondTurn = resolve;
  });

  async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
    yield userMessage("Reply with exactly: ready");
    await secondTurnReady;
    yield userMessage("Reply with exactly: done");
  }

  const session = query({
    prompt: turnPrompts(),
    options: {
      model: "claude-sonnet-5",
    },
  });

  let turnModel = "";
  let completedTurns = 0;

  for await (const message of session) {
    if (message.type === "assistant") {
      turnModel = message.message.model;
    } else if (message.type === "result") {
      completedTurns += 1;
      if (completedTurns === 1) {
        console.log(`First turn model: ${turnModel}`);
        await session.setModel("claude-opus-5");
        await session.setPermissionMode("acceptEdits");
        startSecondTurn();
      } else {
        console.log(`Second turn model: ${turnModel}`);
        break;
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

  async def main():
      options = ClaudeAgentOptions(model="claude-sonnet-5")

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Reply with exactly: ready")
          first_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  first_model = message.model

          await client.set_model("claude-opus-5")
          await client.set_permission_mode("acceptEdits")

          await client.query("Reply with exactly: done")
          second_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  second_model = message.model

      print(f"First turn model: {first_model}")
      print(f"Second turn model: {second_model}")

  asyncio.run(main())
  ```
</CodeGroup>

Sull'API Claude, il programma stampa `First turn model: claude-sonnet-5`, poi `Second turn model: claude-opus-5` dopo il cambio.

<Note>
  Ogni modello ha la sua propria cache del prompt, quindi dopo un cambio durante la sessione la richiesta successiva ricalcola la conversazione completa non memorizzata nella cache alle tariffe del nuovo modello. Per ulteriori informazioni, consultate [Switching models](/docs/it/prompt-caching#switching-models).
</Note>

<h2 id="configure-specific-features">
  Configura funzionalità specifiche
</h2>

La tabella seguente mappa ogni opzione alla funzionalità che configura. Per le opzioni che questa pagina non copre, consultate i riferimenti [TypeScript](/docs/it/agent-sdk/typescript#options) e [Python](/docs/it/agent-sdk/python#claudeagentoptions). Se conoscete il vostro obiettivo ma non quale opzione lo serve, iniziate da [Choose the right feature](/docs/it/agent-sdk/claude-code-features#choose-the-right-feature).

| TypeScript                | Python                      | Controlla                                                       | Coperto in                                                                                                                                                                                                             |
| ------------------------- | --------------------------- | --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionMode`          | `permission_mode`           | Cosa l'agente può fare senza approvazione                       | [Configure permissions](/docs/it/agent-sdk/permissions)                                                                                                                                                                     |
| `allowedTools`            | `allowed_tools`             | Quali chiamate di strumenti sono pre-approvate                  | [Configure permissions](/docs/it/agent-sdk/permissions)                                                                                                                                                                     |
| `canUseTool`              | `can_use_tool`              | Il vostro callback di approvazione per le chiamate di strumenti | [Handle tool approval requests](/docs/it/agent-sdk/user-input#handle-tool-approval-requests)                                                                                                                                |
| `systemPrompt`            | `system_prompt`             | Le istruzioni dell'agente                                       | [Modifying system prompts](/docs/it/agent-sdk/modifying-system-prompts)                                                                                                                                                     |
| `settingSources`          | `setting_sources`           | Quali impostazioni del filesystem caricano                      | [Use Claude Code features in the SDK](/docs/it/agent-sdk/claude-code-features)                                                                                                                                              |
| `mcpServers`              | `mcp_servers`               | Server di strumenti esterni                                     | [Connect to external tools with MCP](/docs/it/agent-sdk/mcp)                                                                                                                                                                |
| `agents`                  | `agents`                    | Definizioni di subagenti                                        | [Subagents](/docs/it/agent-sdk/subagents)                                                                                                                                                                                   |
| `hooks`                   | `hooks`                     | Callback nei punti del ciclo di vita                            | [Hooks](/docs/it/agent-sdk/hooks)                                                                                                                                                                                           |
| `skills`                  | `skills`                    | Quali skill caricano                                            | [Extend agents with skills](/docs/it/agent-sdk/skills)                                                                                                                                                                      |
| `plugins`                 | `plugins`                   | Quali plugin caricano                                           | [Plugins](/docs/it/agent-sdk/plugins)                                                                                                                                                                                       |
| `outputFormat`            | `output_format`             | Schemi di output strutturati                                    | [Structured outputs](/docs/it/agent-sdk/structured-outputs)                                                                                                                                                                 |
| `resume`                  | `resume`                    | Continuazione di una sessione memorizzata                       | [Sessions](/docs/it/agent-sdk/sessions)                                                                                                                                                                                     |
| `forkSession`             | `fork_session`              | Diramazione di una sessione                                     | [Sessions](/docs/it/agent-sdk/sessions)                                                                                                                                                                                     |
| `sessionStore`            | `session_store`             | Persistenza della sessione esterna                              | [Session storage](/docs/it/agent-sdk/session-storage)                                                                                                                                                                       |
| `enableFileCheckpointing` | `enable_file_checkpointing` | Modifiche di file riavvolgibili                                 | [File checkpointing](/docs/it/agent-sdk/file-checkpointing)                                                                                                                                                                 |
| `effort`                  | `effort`                    | Quanto lavoro Claude mette nelle risposte                       | [Effort level](/docs/it/agent-sdk/agent-loop#effort-level)                                                                                                                                                                  |
| `sandbox`                 | `sandbox`                   | Comportamento della sandbox per l'esecuzione degli strumenti    | [TypeScript](/docs/it/agent-sdk/typescript#sandbox-configuration) e [Python](/docs/it/agent-sdk/python#sandbox-configuration) riferimenti, con contesto di distribuzione in [Secure deployment](/docs/it/agent-sdk/secure-deployment) |

<h2 id="next-steps">
  Passaggi successivi
</h2>

Per vedere la configurazione composta in agenti funzionanti:

* **[Quickstart](/docs/it/agent-sdk/quickstart)**: costruisci ed esegui un primo agente da capo a fondo
* **[Examples](/docs/it/agent-sdk/examples)**: trova un progetto completo e eseguibile o una ricetta guidata di Claude Cookbook che corrisponde a quello che vuoi costruire
* **[Multi-tenant isolation](/docs/it/agent-sdk/hosting#multi-tenant-isolation)**: isola le impostazioni e la memoria di ogni tenant con `settingSources` / `setting_sources`, `env` e `cwd`
