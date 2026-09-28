> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lavorare con le sessioni

> Come le sessioni mantengono la cronologia della conversazione dell'agente, e quando utilizzare continue, resume e fork per tornare a un'esecuzione precedente.

Una sessione è la cronologia della conversazione che l'SDK accumula mentre il tuo agente lavora. Contiene il tuo prompt, ogni chiamata di strumento che l'agente ha effettuato, ogni risultato dello strumento e ogni risposta. L'SDK la scrive su disco automaticamente in modo che tu possa tornare ad essa in seguito.

Tornare a una sessione significa che l'agente ha il contesto completo da prima: file che ha già letto, analisi che ha già eseguito, decisioni che ha già preso. Puoi fare una domanda di follow-up, recuperare da un'interruzione o diramati per provare un approccio diverso.

<Note>
  Le sessioni mantengono la **conversazione**, non il filesystem. Per fare uno snapshot e ripristinare i cambiamenti ai file che l'agente ha apportato, utilizza [file checkpointing](/docs/it/agent-sdk/file-checkpointing).
</Note>

Questa guida copre come scegliere l'approccio giusto per la tua app, le interfacce SDK che tracciano automaticamente le sessioni, come acquisire gli ID di sessione e utilizzare manualmente `resume` e `fork`, e cosa sapere sul ripristino delle sessioni tra host.

<h2 id="choose-an-approach">
  Scegli un approccio
</h2>

Quanto gestione della sessione hai bisogno dipende dalla forma della tua applicazione. La gestione della sessione entra in gioco quando invii più prompt che dovrebbero condividere il contesto. All'interno di una singola chiamata `query()`, l'agente già fa tutti i turni di cui ha bisogno, e i prompt di autorizzazione e `AskUserQuestion` sono [gestiti in-loop](/docs/it/agent-sdk/user-input) (non terminano la chiamata).

| Cosa stai costruendo                                           | Cosa usare                                                                                                                                                                                                                                                                                          |
| :------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Attività una tantum: singolo prompt, nessun follow-up          | Niente di extra. Una singola chiamata `query()` la gestisce.                                                                                                                                                                                                                                        |
| Chat multi-turno in un processo                                | [`ClaudeSDKClient` (Python) o `continue: true` (TypeScript)](#automatic-session-management). L'SDK traccia la sessione per te senza gestione degli ID.                                                                                                                                              |
| Riprendere da dove hai lasciato dopo un riavvio del processo   | `continue_conversation=True` (Python) / `continue: true` (TypeScript). Riprende la sessione più recente nella directory, nessun ID necessario.                                                                                                                                                      |
| Riprendere una sessione passata specifica (non la più recente) | Acquisisci l'ID di sessione e passalo a `resume`.                                                                                                                                                                                                                                                   |
| Provare un approccio alternativo senza perdere l'originale     | Fai un fork della sessione.                                                                                                                                                                                                                                                                         |
| Attività senza stato, non vuoi che nulla sia scritto su disco  | Imposta [`persistSession: false`](/docs/it/agent-sdk/typescript#options) (solo TypeScript). La sessione esiste solo in memoria per la durata della chiamata. In Python, imposta [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/it/env-vars) nell'opzione `env` per sopprimere invece le scritture di trascrizione. |

<h3 id="continue-resume-and-fork">
  Continue, resume e fork
</h3>

Continue, resume e fork sono campi opzionali che imposti su `query()` ([`ClaudeAgentOptions`](/docs/it/agent-sdk/python#claudeagentoptions) in Python, [`Options`](/docs/it/agent-sdk/typescript#options) in TypeScript).

**Continue** e **resume** entrambi riprendono una sessione esistente e la aggiungono. La differenza è come trovano quella sessione:

* **Continue** trova la sessione più recente nella directory corrente. Non traccia nulla. Funziona bene quando la tua app esegue una conversazione alla volta.
* **Resume** accetta un ID di sessione specifico. Traccia l'ID. Richiesto quando hai più sessioni (ad esempio, una per utente in un'app multi-utente) o vuoi tornare a una che non è la più recente.

**Fork** è diverso: crea una nuova sessione che inizia con una copia della cronologia dell'originale. L'originale rimane invariato. Usa fork per provare una direzione diversa mantenendo l'opzione di tornare indietro.

<h2 id="automatic-session-management">
  Gestione automatica della sessione
</h2>

Entrambi gli SDK offrono un'interfaccia che traccia lo stato della sessione per te tra le chiamate, quindi non passi gli ID in giro manualmente. Usali per conversazioni multi-turno all'interno di un singolo processo.

<h3 id="python-claudesdkclient">
  Python: `ClaudeSDKClient`
</h3>

[`ClaudeSDKClient`](/docs/it/agent-sdk/python#claudesdkclient) gestisce gli ID di sessione internamente. Ogni chiamata a `client.query()` continua automaticamente la stessa sessione. Chiama [`client.receive_response()`](/docs/it/agent-sdk/python#claudesdkclient) per iterare sui messaggi per la query corrente. Usa il client come gestore di contesto asincrono in modo che la configurazione della connessione e lo smontaggio siano gestiti per te, oppure chiama `connect()` e `disconnect()` manualmente.

Questo esempio esegue due query contro lo stesso `client`. La prima chiede all'agente di analizzare un modulo; la seconda gli chiede di refactorizzare quel modulo. Poiché entrambe le chiamate passano attraverso la stessa istanza del client, la seconda query ha il contesto completo dalla prima senza alcun `resume` esplicito o ID di sessione:

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ResultMessage,
    TextBlock,
)


def print_response(message):
    """Print only the human-readable parts of a message."""
    if isinstance(message, AssistantMessage):
        for block in message.content:
            if isinstance(block, TextBlock):
                print(block.text)
    elif isinstance(message, ResultMessage):
        cost = (
            f"${message.total_cost_usd:.4f}"
            if message.total_cost_usd is not None
            else "N/A"
        )
        print(f"[done: {message.subtype}, cost: {cost}]")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Edit", "Glob", "Grep"],
    )

    async with ClaudeSDKClient(options=options) as client:
        # First query: client captures the session ID internally
        await client.query("Analyze the auth module")
        async for message in client.receive_response():
            print_response(message)

        # Second query: automatically continues the same session
        await client.query("Now refactor it to use JWT")
        async for message in client.receive_response():
            print_response(message)


asyncio.run(main())
```

Ogni query stampa la risposta di testo dell'agente seguita da una riga di stato dal messaggio di risultato, come `[done: success, cost: $0.0042]`.

Vedi il [riferimento Python SDK](/docs/it/agent-sdk/python#choosing-between-query-and-claudesdkclient) per i dettagli su quando usare `ClaudeSDKClient` rispetto alla funzione standalone `query()`.

<h3 id="typescript-continue-true">
  TypeScript: `continue: true`
</h3>

L'SDK TypeScript non ha un oggetto client che tiene la sessione come il `ClaudeSDKClient` di Python. Invece, passa `continue: true` su ogni successiva chiamata `query()` e l'SDK raccoglie la sessione più recente nella directory corrente. Nessun tracciamento degli ID richiesto.

Questo esempio effettua due separate chiamate `query()`. La prima crea una sessione nuova; la seconda imposta `continue: true`, che dice all'SDK di trovare e riprendere la sessione più recente su disco. L'agente ha il contesto completo dalla prima chiamata:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// First query: creates a new session
try {
  for await (const message of query({
    prompt: "Analyze the auth module",
    options: { allowedTools: ["Read", "Glob", "Grep"] }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // so the follow-up query below still runs.
  console.error(`Session ended with an error: ${error}`);
}

// Second query: continue: true resumes the most recent session
for await (const message of query({
  prompt: "Now refactor it to use JWT",
  options: {
    continue: true,
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

<Note>
  L'API di sessione sperimentale [V2](/docs/it/agent-sdk/typescript-v2-preview), che forniva `createSession()` con un pattern `send` / `stream`, è stata rimossa in TypeScript Agent SDK 0.3.142. Usa la funzione `query()` e le opzioni di sessione descritte in questa pagina.
</Note>

<h2 id="use-session-options-with-query">
  Usa le opzioni di sessione con `query()`
</h2>

<h3 id="capture-the-session-id">
  Acquisisci l'ID di sessione
</h3>

Resume e fork richiedono un ID di sessione. Leggilo dal campo `session_id` sul messaggio di risultato ([`ResultMessage`](/docs/it/agent-sdk/python#resultmessage) in Python, [`SDKResultMessage`](/docs/it/agent-sdk/typescript#sdkresultmessage) in TypeScript), che è presente su ogni risultato indipendentemente dal successo o dall'errore. In TypeScript l'ID è disponibile anche prima come campo diretto sul `SystemMessage` di init; in Python è annidato dentro `SystemMessage.data`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      session_id = None

      try:
          async for message in query(
              prompt="Analyze the auth module and suggest improvements",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if isinstance(message, ResultMessage):
                  session_id = message.session_id
                  if message.subtype == "success":
                      print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the loop above already captured session_id;
          # connection or process failures yield no result message, so session_id stays None.
          print(f"Session ended with an error: {error}")

      print(f"Session ID: {session_id}")
      return session_id


  session_id = asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Analyze the auth module and suggest improvements",
      options: { allowedTools: ["Read", "Glob", "Grep"] }
    })) {
      if (message.type === "result") {
        sessionId = message.session_id;
        if (message.subtype === "success") {
          console.log(message.result);
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the loop above already captured sessionId;
    // connection or process failures yield no result message, so sessionId stays undefined.
    console.error(`Session ended with an error: ${error}`);
  }

  console.log(`Session ID: ${sessionId}`);
  ```
</CodeGroup>

Quando la query si completa, lo script stampa la risposta dell'agente seguita da una riga come `Session ID: 5b3f2c1a-8d4e-4f6b-9a7c-2e1d0f9b8a6c`. Nelle sezioni successive, passerai questo ID a `resume`.

<h3 id="resume-by-id">
  Riprendi per ID
</h3>

Passa un ID di sessione a `resume` per tornare a quella sessione specifica. L'agente riprende con il contesto completo da dove la sessione si è fermata. Motivi comuni per riprendere:

* **Seguire un'attività completata.** L'agente ha già analizzato qualcosa; ora vuoi che agisca su quell'analisi senza rileggere i file.
* **Recuperare da un limite.** La prima esecuzione è terminata con `error_max_turns` o `error_max_budget_usd` (vedi [Gestisci il risultato](/docs/it/agent-sdk/agent-loop#handle-the-result)); riprendi con un limite più alto. In una singola chiamata `query()` l'SDK genera un'eccezione dopo aver restituito quel risultato di errore, quindi cattura l'errore prima di riprendere.
* **Riavvia il tuo processo.** Hai acquisito l'ID prima dell'arresto e vuoi ripristinare la conversazione.

Questo esempio riprende la sessione da [Acquisisci l'ID di sessione](#capture-the-session-id) con un prompt di follow-up. Poiché stai riprendendo, l'agente ha già l'analisi precedente nel contesto:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  session_id = "..."  # The ID you captured in the previous example


  async def main():
      # Earlier session analyzed the code; now build on that analysis
      async for message in query(
          prompt="Now implement the refactoring you suggested",
          options=ClaudeAgentOptions(
              resume=session_id,
              allowed_tools=["Read", "Edit", "Write", "Glob", "Grep"],
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Earlier session analyzed the code; now build on that analysis
  for await (const message of query({
    prompt: "Now implement the refactoring you suggested",
    options: {
      resume: sessionId,
      allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Dovresti vedere una risposta che si basa sull'analisi precedente invece di iniziare da zero. Questo conferma che l'agente ha ripreso la sessione con il suo contesto precedente intatto.

<Tip>
  Claude Code archivia le sessioni sotto `~/.claude/projects/<encoded-cwd>/*.jsonl`. Se imposti la variabile di ambiente `CLAUDE_CONFIG_DIR`, cerca invece sotto `$CLAUDE_CONFIG_DIR/projects/`.

  Per trovare la directory della tua sessione, sostituisci ogni carattere non alfanumerico nella directory di lavoro assoluta con `-`: `/Users/me/proj` diventa `-Users-me-proj`. Per una directory di lavoro il cui nome convertito supera 200 caratteri, Claude Code [tronca il nome e aggiunge un hash](/docs/it/sessions#where-transcripts-are-stored), quindi fai corrispondere i primi 200 caratteri del nome convertito quando elenchi `projects/`.

  Se imposti [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/it/sessions#name-the-project-directory-yourself) accanto a `CLAUDE_CONFIG_DIR`, cerca invece sotto quel nome in `projects/`. Richiede Agent SDK TypeScript v0.3.234 o successivo, oppure Agent SDK Python v0.2.140 o successivo.

  Puoi riprendere da qualsiasi directory di lavoro:

  * **Ricerca tra directory**: Claude Code cerca oltre la directory del progetto corrente per trovare l'ID; vedi [Riprendi una sessione](/docs/it/sessions#resume-a-session) per l'ordine di ricerca esatto e come vengono gestite le copie duplicate.
  * **Solo sulla stessa macchina**: il file di sessione deve ancora esistere sulla macchina corrente.

  Prima della v2.1.223, la ricerca era limitata alla directory del progetto corrente e ai suoi git worktrees; le versioni SDK che includono un CLI più vecchio si comportano ancora in questo modo.
</Tip>

Per riprendere sessioni tra macchine o in ambienti serverless, specchia i transcript in un archivio condiviso con un adattatore [`SessionStore`](/docs/it/agent-sdk/session-storage).

<h3 id="fork-to-explore-alternatives">
  Fai un fork per esplorare alternative
</h3>

Il forking crea una nuova sessione che inizia con una copia della cronologia dell'originale ma diverge da quel punto. Il fork ottiene il suo proprio ID di sessione; l'ID e la cronologia dell'originale rimangono invariati. Finisci con due sessioni indipendenti che puoi riprendere separatamente.

<Note>
  Il forking dirama la cronologia della conversazione, non il filesystem. Se un agente con fork modifica i file, quei cambiamenti sono reali e visibili a qualsiasi sessione che lavora nella stessa directory. Per diramazioni e ripristino dei cambiamenti ai file, utilizza [file checkpointing](/docs/it/agent-sdk/file-checkpointing).
</Note>

Questo esempio si basa su [Acquisisci l'ID di sessione](#capture-the-session-id): hai già analizzato un modulo di autenticazione in `session_id` e vuoi esplorare OAuth2 senza perdere il thread focalizzato su JWT. Il primo blocco fa un fork della sessione e acquisisce l'ID del fork (`forked_id`); il secondo blocco riprende il `session_id` originale per continuare lungo il percorso JWT. Ora hai due ID di sessione che puntano a due cronologie separate:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  session_id = "..."  # The ID you captured in the previous example


  async def main():
      # Fork: branch from session_id into a new session
      forked_id = None
      try:
          async for message in query(
              prompt="Instead of JWT, outline how OAuth2 would work for the auth module",
              options=ClaudeAgentOptions(
                  resume=session_id,
                  fork_session=True,
                  max_turns=5,
              ),
          ):
              if isinstance(message, ResultMessage):
                  forked_id = message.session_id  # The fork's ID, distinct from session_id
                  if message.subtype == "success":
                      print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, forked_id was already captured by the
          # loop above; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")

      print(f"Forked session: {forked_id}")

      # Original session is untouched; resuming it continues the JWT thread
      try:
          async for message in query(
              prompt="Continue with the JWT approach",
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Fork: branch from sessionId into a new session
  let forkedId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Instead of JWT, outline how OAuth2 would work for the auth module",
      options: {
        resume: sessionId,
        forkSession: true,
        maxTurns: 5
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        forkedId = message.session_id; // The fork's ID, distinct from sessionId
      }
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, forkedId was already captured by the loop
    // above; connection or process failures yield no result message.
    console.error(`Session ended with an error: ${error}`);
  }

  console.log(`Forked session: ${forkedId}`);

  // Original session is untouched; resuming it continues the JWT thread
  try {
    for await (const message of query({
      prompt: "Continue with the JWT approach",
      options: { resume: sessionId }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

Dovresti vedere che `forkedId` differisce dall'ID di sessione originale. Riprendere la sessione originale continua ancora il thread JWT, il che conferma che il fork non ha modificato la cronologia originale.

<h2 id="resume-across-hosts">
  Riprendi tra host
</h2>

I file di sessione sono locali alla macchina che li ha creati. Per riprendere una sessione su un host diverso (worker CI, container effimeri, serverless), scegli l'approccio che si adatta meglio:

* **Passa un session store.** Allega un adattatore [`sessionStore` / `session_store`](/docs/it/agent-sdk/session-storage) in modo che l'SDK rispecchi i transcript nel tuo backend e un altro host possa riprenderli. La chiave di ricerca dello store deriva dalla directory di lavoro, quindi riprendi da un `cwd` che corrisponda all'esecuzione originale.

* **Sposta il file di sessione.** Persisti `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl` dalla prima esecuzione e ripristinalo all'interno di qualsiasi directory sotto `~/.claude/projects/` sul nuovo host prima di chiamare `resume`.

  Claude Code cerca oltre la directory del progetto corrente per trovare l'ID; vedi [Riprendi una sessione](/docs/it/sessions#resume-a-session) per l'ordine di ricerca esatto e come vengono gestite le copie duplicate. Prima della v2.1.223, la ricerca era limitata alla directory del progetto corrente e ai suoi git worktrees; le versioni dell'SDK che raggruppano un CLI più vecchio si comportano ancora in questo modo.

* **Non fare affidamento sul ripristino della sessione.** Acquisisci i risultati di cui hai bisogno (output di analisi, decisioni, diff di file) come stato dell'applicazione e passali nel prompt di una sessione nuova. Questo è spesso più robusto che spedire file di transcript in giro.

Entrambi gli SDK espongono funzioni per enumerare le sessioni su disco e leggere i loro messaggi: [`listSessions()`](/docs/it/agent-sdk/typescript#listsessions) e [`getSessionMessages()`](/docs/it/agent-sdk/typescript#getsessionmessages) in TypeScript, [`list_sessions()`](/docs/it/agent-sdk/python#list_sessions) e [`get_session_messages()`](/docs/it/agent-sdk/python#get_session_messages) in Python. Usali per costruire selettori di sessione personalizzati, logica di pulizia o visualizzatori di transcript.

Entrambi gli SDK espongono anche funzioni per cercare e mutare sessioni individuali: [`get_session_info()`](/docs/it/agent-sdk/python#get_session_info), [`rename_session()`](/docs/it/agent-sdk/python#rename_session) e [`tag_session()`](/docs/it/agent-sdk/python#tag_session) in Python, e [`getSessionInfo()`](/docs/it/agent-sdk/typescript#getsessioninfo), [`renameSession()`](/docs/it/agent-sdk/typescript#renamesession) e [`tagSession()`](/docs/it/agent-sdk/typescript#tagsession) in TypeScript. Usali per organizzare le sessioni per tag o dare loro titoli leggibili dall'uomo.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Come funziona il ciclo dell'agente](/docs/it/agent-sdk/agent-loop): Comprendi i turni, i messaggi e l'accumulo del contesto all'interno di una sessione
* [File checkpointing](/docs/it/agent-sdk/file-checkpointing): Traccia e ripristina i cambiamenti ai file tra le sessioni
* [Python `ClaudeAgentOptions`](/docs/it/agent-sdk/python#claudeagentoptions): Riferimento completo delle opzioni di sessione per Python
* [TypeScript `Options`](/docs/it/agent-sdk/typescript#options): Riferimento completo delle opzioni di sessione per TypeScript
