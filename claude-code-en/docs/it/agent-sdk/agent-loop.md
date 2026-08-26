> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Come funziona il ciclo dell'agente

> Comprendere il ciclo di vita dei messaggi, l'esecuzione degli strumenti, la finestra di contesto e l'architettura che alimentano gli agenti SDK.

L'Agent SDK consente di incorporare il ciclo dell'agente autonomo di Claude Code nelle proprie applicazioni. L'SDK è un pacchetto autonomo che fornisce il controllo programmatico su strumenti, autorizzazioni, limiti di costo e output. Non è necessario avere Claude Code CLI installato per utilizzarlo.

Quando avviate un agente, l'SDK esegue lo stesso [ciclo di esecuzione che alimenta Claude Code](/docs/it/how-claude-code-works#the-agentic-loop): Claude valuta il vostro prompt, chiama gli strumenti per agire, riceve i risultati e ripete fino al completamento dell'attività. Questa pagina spiega cosa accade all'interno di quel ciclo in modo che possiate costruire, eseguire il debug e ottimizzare i vostri agenti in modo efficace.

<h2 id="the-loop-at-a-glance">
  Il ciclo a colpo d'occhio
</h2>

Ogni sessione dell'agente segue lo stesso ciclo:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-loop-diagram.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=1c6e8f28d80dba14a7287419656f1237" alt="Diagram of the agent loop: your prompt enters the agentic loop, where Claude evaluates and either requests tool calls, whose results feed back into another evaluation, or returns the final answer" width="720" height="212" data-path="images/agent-loop-diagram.svg" />

1. **Ricevere il prompt.** Claude riceve il vostro prompt, insieme al prompt di sistema, alle definizioni degli strumenti e alla cronologia della conversazione. L'SDK produce un [`SystemMessage`](#message-types) con sottotipo `"init"` contenente i metadati della sessione.
2. **Valutare e rispondere.** Claude valuta lo stato attuale e determina come procedere. Può rispondere con testo, richiedere una o più chiamate di strumenti, o entrambi. L'SDK produce un [`AssistantMessage`](#message-types) contenente il testo e le richieste di chiamate di strumenti.
3. **Eseguire gli strumenti.** L'SDK esegue ogni strumento richiesto e raccoglie i risultati. Ogni set di risultati degli strumenti viene restituito a Claude per la decisione successiva. Potete utilizzare [hooks](/docs/it/agent-sdk/hooks) per intercettare, modificare o bloccare le chiamate di strumenti prima che vengano eseguite.
4. **Ripetere.** I passaggi 2 e 3 si ripetono come un ciclo. Ogni ciclo completo è un turno. Claude continua a chiamare gli strumenti ed elaborare i risultati fino a quando non produce una risposta senza chiamate di strumenti.
5. **Restituire il risultato.** L'SDK produce un [`AssistantMessage`](#message-types) finale con la risposta di testo (senza chiamate di strumenti), seguito da un [`ResultMessage`](#message-types) con il testo finale, l'utilizzo dei token, il costo e l'ID della sessione.

Una domanda rapida ("quali file ci sono qui?") potrebbe richiedere uno o due turni di chiamata di `Glob` e risposta con i risultati. Un'attività complessa ("refactorizza il modulo di autenticazione e aggiorna i test") può concatenare dozzine di chiamate di strumenti su molti turni, leggendo file, modificando codice ed eseguendo test, con Claude che adatta il suo approccio in base a ogni risultato.

<h2 id="turns-and-messages">
  Turni e messaggi
</h2>

Un turno è un viaggio di andata e ritorno all'interno del ciclo: Claude produce output che include chiamate di strumenti, l'SDK esegue quegli strumenti e i risultati vengono restituiti a Claude automaticamente. Questo accade senza cedere il controllo al vostro codice. I turni continuano fino a quando Claude non produce output senza chiamate di strumenti, a quel punto il ciclo termina e il risultato finale viene consegnato.

Considerate come potrebbe apparire una sessione completa per il prompt "Correggi i test falliti in auth.ts".

Per prima cosa, l'SDK invia il vostro prompt a Claude e produce un [`SystemMessage`](#message-types) con i metadati della sessione. Poi il ciclo inizia:

1. **Turno 1:** Claude chiama `Bash` per eseguire `npm test`. L'SDK produce un [`AssistantMessage`](#message-types) con la chiamata dello strumento, esegue il comando, poi produce un [`UserMessage`](#message-types) con l'output (tre errori).
2. **Turno 2:** Claude chiama `Read` su `auth.ts` e `auth.test.ts`. L'SDK restituisce il contenuto dei file e produce un `AssistantMessage`.
3. **Turno 3:** Claude chiama `Edit` per correggere `auth.ts`, poi chiama `Bash` per rieseguire `npm test`. Tutti e tre i test passano. L'SDK produce un `AssistantMessage`.
4. **Turno finale:** Claude produce una risposta solo di testo senza chiamate di strumenti: "Corretto il bug di autenticazione, tutti e tre i test passano ora." L'SDK produce un `AssistantMessage` finale con questo testo, poi un [`ResultMessage`](#message-types) con lo stesso testo più costo e utilizzo.

Erano quattro turni: tre con chiamate di strumenti, uno con risposta solo di testo finale.

Potete limitare il ciclo con `max_turns` / `maxTurns`, che conta solo i turni di utilizzo degli strumenti. Ad esempio, `max_turns=2` nel ciclo precedente si sarebbe fermato prima del passaggio di modifica. Potete anche utilizzare `max_budget_usd` / `maxBudgetUsd` per limitare i turni in base a una soglia di spesa.

Senza limiti, il ciclo viene eseguito fino a quando Claude non termina da solo, il che va bene per attività ben definite ma può durare a lungo su prompt aperti ("migliora questo codebase"). Impostare un budget è una buona impostazione predefinita per gli agenti di produzione. Vedere [Turni e budget](#turns-and-budget) di seguito per il riferimento alle opzioni.

<h2 id="message-types">
  Tipi di messaggi
</h2>

Mentre il ciclo viene eseguito, l'SDK produce un flusso di messaggi. Ogni messaggio ha un tipo che vi dice da quale fase del ciclo proviene. I cinque tipi principali sono:

* **`SystemMessage`:** eventi del ciclo di vita della sessione. Il campo `subtype` li distingue:

  * `"init"`: metadati della sessione per l'esecuzione. Quando un hook `SessionStart` o `Setup` viene eseguito durante l'avvio della sessione, i suoi [messaggi del ciclo di vita dell'hook](/docs/it/agent-sdk/typescript#sdkhookstartedmessage) arrivano prima del messaggio `init`
  * `"compact_boundary"`: si attiva dopo la [compattazione](#automatic-compaction)
  * `"informational"`: banner di stato in testo semplice dal ciclo
  * `"worker_shutting_down"`: il ciclo terminerà dopo il turno corrente perché l'host sta uscendo o Remote Control si è disconnesso

  In TypeScript, ogni sottotipo diverso da `"init"` è il suo proprio tipo nell'unione [`SDKMessage`](/docs/it/agent-sdk/typescript#sdkmessage) piuttosto che un sottotipo di `SDKSystemMessage`.
* **`AssistantMessage`:** emesso dopo ogni risposta di Claude, inclusa quella finale solo di testo. Contiene blocchi di contenuto di testo e blocchi di chiamate di strumenti da quel turno.
* **`UserMessage`:** emesso dopo ogni esecuzione di strumento con il risultato dello strumento inviato di nuovo a Claude. Emesso anche per qualsiasi input dell'utente che trasmettete a metà ciclo.
* **`StreamEvent`:** emesso solo quando i messaggi parziali sono abilitati. Contiene eventi di streaming API grezzi (delta di testo, chunk di input dello strumento). Vedere [Stream responses](/docs/it/agent-sdk/streaming-output).
* **`ResultMessage`:** segna la fine del ciclo dell'agente. Contiene il risultato di testo finale, l'utilizzo dei token, il costo e l'ID della sessione. Controllate il campo `subtype` per determinare se l'attività ha avuto successo o ha raggiunto un limite. Un piccolo numero di eventi di sistema finali, come `prompt_suggestion`, può arrivare dopo di esso, quindi iterate il flusso fino al completamento piuttosto che interrompere al risultato. Vedere [Gestire il risultato](#handle-the-result).

Questi cinque tipi coprono l'intero ciclo di vita del ciclo dell'agente in entrambi gli SDK. L'SDK TypeScript produce anche eventi di osservabilità aggiuntivi (eventi di hook, progresso dello strumento, limiti di velocità, notifiche di attività) che forniscono dettagli extra ma non sono necessari per guidare il ciclo. Vedere il [riferimento ai tipi di messaggi Python](/docs/it/agent-sdk/python#message-types) e il [riferimento ai tipi di messaggi TypeScript](/docs/it/agent-sdk/typescript#message-types) per gli elenchi completi.

<h3 id="handle-messages">
  Gestire i messaggi
</h3>

Quali messaggi gestite dipende da ciò che state costruendo:

* **Solo risultati finali:** gestite `ResultMessage` per ottenere l'output, il costo e se l'attività ha avuto successo o ha raggiunto un limite.
* **Aggiornamenti di progresso:** gestite `AssistantMessage` per vedere cosa sta facendo Claude ad ogni turno, inclusi gli strumenti che ha chiamato.
* **Streaming in tempo reale:** abilitate i messaggi parziali (`include_partial_messages` in Python, `includePartialMessages` in TypeScript) per ottenere messaggi `StreamEvent` in tempo reale. Vedere [Stream responses in real-time](/docs/it/agent-sdk/streaming-output).

Come controllate i tipi di messaggi dipende dall'SDK:

* **Python:** controllate i tipi di messaggi con `isinstance()` rispetto alle classi importate da `claude_agent_sdk` (ad esempio, `isinstance(message, ResultMessage)`).
* **TypeScript:** controllate il campo stringa `type` (ad esempio, `message.type === "result"`). `AssistantMessage` e `UserMessage` avvolgono il messaggio API grezzo in un campo `.message`, quindi i blocchi di contenuto si trovano in `message.message.content`, non in `message.content`.

<Accordion title="Esempio: Controllare i tipi di messaggi e gestire i risultati">
  <CodeGroup>
    ```python Python theme={null}
    import asyncio
    from claude_agent_sdk import query, AssistantMessage, ResultMessage


    async def main():
        try:
            async for message in query(prompt="Summarize this project"):
                if isinstance(message, AssistantMessage):
                    print(f"Turn completed: {len(message.content)} content blocks")
                if isinstance(message, ResultMessage):
                    if message.subtype == "success":
                        print(message.result)
                    else:
                        print(f"Stopped: {message.subtype}")
        except Exception as error:
            # A single-shot query() raises after yielding an error result. If the
            # failure was an error result, the error subtype branches above have
            # already run; connection or process failures yield no result message.
            print(f"Session ended with an error: {error}")


    asyncio.run(main())
    ```

    ```typescript TypeScript theme={null}
    import { query } from "@anthropic-ai/claude-agent-sdk";

    try {
      for await (const message of query({ prompt: "Summarize this project" })) {
        if (message.type === "assistant") {
          console.log(`Turn completed: ${message.message.content.length} content blocks`);
        }
        if (message.type === "result") {
          if (message.subtype === "success") {
            console.log(message.result);
          } else {
            console.log(`Stopped: ${message.subtype}`);
          }
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, the error subtype branches above have
      // already run; connection or process failures yield no result message.
      console.log(`Session ended with an error: ${error}`);
    }
    ```
  </CodeGroup>
</Accordion>

<h2 id="tool-execution">
  Esecuzione degli strumenti
</h2>

Gli strumenti danno al vostro agente la capacità di agire. Senza strumenti, Claude può solo rispondere con testo. Con gli strumenti, Claude può leggere file, eseguire comandi, cercare codice e interagire con servizi esterni.

<h3 id="built-in-tools">
  Strumenti integrati
</h3>

L'SDK include gli stessi strumenti che alimentano Claude Code:

| Categoria              | Strumenti                                                       | Cosa fanno                                                                               |
| :--------------------- | :-------------------------------------------------------------- | :--------------------------------------------------------------------------------------- |
| **Operazioni su file** | `Read`, `Edit`, `Write`                                         | Leggere, modificare e creare file                                                        |
| **Ricerca**            | `Glob`, `Grep`                                                  | Trovare file per pattern, cercare contenuto con regex                                    |
| **Esecuzione**         | `Bash`                                                          | Eseguire comandi shell, script, operazioni git                                           |
| **Web**                | `WebSearch`, `WebFetch`                                         | Cercare il web, recuperare e analizzare pagine                                           |
| **Scoperta**           | `ToolSearch`                                                    | Trovare e caricare dinamicamente gli strumenti su richiesta invece di precaricarli tutti |
| **Orchestrazione**     | `Agent`, `Skill`, `AskUserQuestion`, `TaskCreate`, `TaskUpdate` | Generare subagenti, invocare skills, chiedere all'utente, tracciare attività             |

Oltre agli strumenti integrati, potete:

* **Connettere servizi esterni** con [server MCP](/docs/it/agent-sdk/mcp) (database, browser, API)
* **Definire strumenti personalizzati** con [gestori di strumenti personalizzati](/docs/it/agent-sdk/custom-tools)
* **Caricare skills del progetto** tramite [setting sources](/docs/it/agent-sdk/claude-code-features) per flussi di lavoro riutilizzabili

<h3 id="tool-permissions">
  Autorizzazioni degli strumenti
</h3>

Claude determina quali strumenti chiamare in base all'attività, ma voi controllate se quelle chiamate sono autorizzate a essere eseguite. Potete approvare automaticamente strumenti specifici, bloccare altri completamente, o richiedere l'approvazione per tutto. Tre opzioni lavorano insieme per determinare cosa viene eseguito:

* **`allowed_tools` / `allowedTools`** approva automaticamente gli strumenti elencati. Un agente di sola lettura con `["Read", "Glob", "Grep"]` nel suo elenco di strumenti consentiti esegue quegli strumenti senza chiedere. Gli strumenti non elencati sono ancora disponibili ma richiedono autorizzazione.
* **`disallowed_tools` / `disallowedTools`** blocca gli strumenti elencati, indipendentemente da altre impostazioni. Vedere [Autorizzazioni](/docs/it/agent-sdk/permissions) per l'ordine in cui le regole vengono controllate prima che uno strumento venga eseguito.
* **`permission_mode` / `permissionMode`** controlla cosa accade agli strumenti che non sono coperti da regole di consentimento o negazione. Vedere [Modalità di autorizzazione](#permission-mode) per le modalità disponibili.

Potete anche limitare gli strumenti individuali con regole come `"Bash(npm *)"` per consentire solo comandi specifici. Vedere [Autorizzazioni](/docs/it/agent-sdk/permissions) per la sintassi completa delle regole.

Quando uno strumento viene negato, Claude riceve un messaggio di rifiuto come risultato dello strumento e in genere tenta un approccio diverso o segnala che non potrebbe procedere.

<h3 id="parallel-tool-execution">
  Esecuzione parallela degli strumenti
</h3>

Quando Claude richiede più chiamate di strumenti in un singolo turno, entrambi gli SDK possono eseguirli contemporaneamente o sequenzialmente a seconda dello strumento. Gli strumenti di sola lettura (come `Read`, `Glob`, `Grep` e strumenti MCP contrassegnati come di sola lettura) possono essere eseguiti contemporaneamente. Gli strumenti che modificano lo stato (come `Edit`, `Write` e `Bash`) vengono eseguiti sequenzialmente per evitare conflitti.

Gli strumenti personalizzati predefiniti per l'esecuzione sequenziale. Per abilitare l'esecuzione parallela per uno strumento personalizzato, impostare `readOnlyHint` nelle sue annotazioni. Sia l'SDK [TypeScript](/docs/it/agent-sdk/typescript#tool) che [Python](/docs/it/agent-sdk/python#tool) utilizzano questo nome di campo dall'SDK MCP.

<h2 id="control-how-the-loop-runs">
  Controllare come viene eseguito il ciclo
</h2>

Potete limitare quanti turni il ciclo prende, quanto costa, quanto profondamente Claude ragiona e se gli strumenti richiedono approvazione prima di essere eseguiti. Tutti questi sono campi su [`ClaudeAgentOptions`](/docs/it/agent-sdk/python#claudeagentoptions) (Python) / [`Options`](/docs/it/agent-sdk/typescript#options) (TypeScript).

<h3 id="turns-and-budget">
  Turni e budget
</h3>

| Opzione                                        | Cosa controlla                                 | Predefinito   |
| :--------------------------------------------- | :--------------------------------------------- | :------------ |
| Max turni (`max_turns` / `maxTurns`)           | Massimi round trip di utilizzo degli strumenti | Nessun limite |
| Max budget (`max_budget_usd` / `maxBudgetUsd`) | Costo massimo prima di fermarsi                | Nessun limite |

Quando uno dei due limiti viene raggiunto, l'SDK restituisce un `ResultMessage` con un sottotipo di errore corrispondente (`error_max_turns` o `error_max_budget_usd`). Vedere [Gestire il risultato](#handle-the-result) per come controllare questi sottotipi e [`ClaudeAgentOptions`](/docs/it/agent-sdk/python#claudeagentoptions) / [`Options`](/docs/it/agent-sdk/typescript#options) per la sintassi.

Con [streaming input](/docs/it/agent-sdk/streaming-vs-single-mode), un messaggio che inviate mentre un turno è ancora in esecuzione rimane in coda quando quel turno termina al limite di max-turns, e inizia il suo turno con il suo limite di max-turns. Prima della v2.1.205, un messaggio che arrivava nell'iterazione finale del turno poteva essere consumato nel turno in chiusura e perso senza mai raggiungere il modello.

<h3 id="effort-level">
  Livello di sforzo
</h3>

L'opzione `effort` controlla quanto ragionamento Claude applica. I livelli di sforzo inferiori utilizzano meno token per turno e riducono il costo. Non tutti i modelli supportano il parametro di sforzo. Vedere [Effort](https://platform.claude.com/docs/en/build-with-claude/effort) per quali modelli lo supportano.

| Livello    | Comportamento                        | Buono per                                                                    |
| :--------- | :----------------------------------- | :--------------------------------------------------------------------------- |
| `"low"`    | Ragionamento minimo, risposte veloci | Ricerche di file, elenco di directory                                        |
| `"medium"` | Ragionamento equilibrato             | Modifiche di routine, attività standard                                      |
| `"high"`   | Analisi approfondita                 | Refactoring, debug                                                           |
| `"xhigh"`  | Profondità di ragionamento estesa    | Attività di codifica e agentic; consigliato su Fable 5, Opus 4.7+ e Sonnet 5 |
| `"max"`    | Profondità di ragionamento massima   | Problemi multi-step che richiedono analisi profonda                          |

Se non impostate `effort`, entrambi gli SDK lasciano il parametro non impostato e rimandano al comportamento predefinito del modello.

<Note>
  `effort` scambia latenza e costo dei token per profondità di ragionamento all'interno di ogni risposta. [Extended thinking](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) è una funzione separata che produce blocchi di catena di pensiero visibili nell'output. Sono indipendenti: potete impostare `effort: "low"` con extended thinking abilitato, o `effort: "max"` senza di esso.
</Note>

Utilizzate uno sforzo inferiore per gli agenti che eseguono attività semplici e ben definite (come elencare file o eseguire un singolo grep) per ridurre il costo e la latenza. Impostare `effort` nelle opzioni di livello superiore `query()` per l'intera sessione, o per subagente con il campo `effort` su [`AgentDefinition`](/docs/it/agent-sdk/subagents#agentdefinition-configuration) per sovrascrivere il livello di sessione.

<h3 id="permission-mode">
  Modalità di autorizzazione
</h3>

L'opzione della modalità di autorizzazione (`permission_mode` in Python, `permissionMode` in TypeScript) controlla se l'agente chiede l'approvazione prima di utilizzare gli strumenti:

| Modalità              | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"default"`           | Gli strumenti non coperti da regole di consentimento attivano il vostro callback di approvazione; nessun callback significa negare                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `"acceptEdits"`       | Approva automaticamente le modifiche ai file e i comandi comuni del filesystem (`mkdir`, `touch`, `mv`, `cp`, ecc.); altri comandi Bash seguono le regole predefinite                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `"plan"`              | Claude esplora e pianifica senza modificare i vostri file sorgente; le modifiche ai file non vengono mai approvate automaticamente e vengono richieste tramite il vostro callback `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `"dontAsk"`           | Non chiede mai. Gli strumenti pre-approvati da [regole di autorizzazione](/docs/it/settings#permission-settings) vengono eseguiti; tutto il resto viene negato. `AskUserQuestion`, strumenti connettore [impostati dalla vostra organizzazione su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) vengono negati anche se li avete consentiti                                                                                                                                                                                                        |
| `"auto"`              | Utilizza un classificatore di modello per approvare o negare ogni chiamata di strumento. Vedere [Modalità Auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) per disponibilità e comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `"bypassPermissions"` | Esegue tutti gli strumenti consentiti senza chiedere, tranne gli strumenti corrispondenti a una regola [`ask`](/docs/it/settings#permission-settings) esplicita, strumenti connettore [impostati dalla vostra organizzazione su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e strumenti che richiedono l'interazione dell'utente; vedere [Come vengono valutate le autorizzazioni](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) per l'ordine di precedenza. Non può essere utilizzato quando si esegue come root su Unix. Utilizzare solo in ambienti isolati dove le azioni dell'agente non possono influenzare i sistemi che vi interessano |

Per le applicazioni interattive, utilizzate `"default"` con un callback di approvazione dello strumento per visualizzare i prompt di approvazione. Per gli agenti autonomi su una macchina di sviluppo, `"acceptEdits"` approva automaticamente le modifiche ai file e i comandi comuni del filesystem (`mkdir`, `touch`, `mv`, `cp`, ecc.) mentre ancora limita altri comandi `Bash` dietro le regole di consentimento. Riservate `"bypassPermissions"` per CI, container o altri ambienti isolati. Vedere [Autorizzazioni](/docs/it/agent-sdk/permissions) per i dettagli completi.

<h3 id="model">
  Modello
</h3>

Se non impostate `model`, l'SDK utilizza il predefinito di Claude Code, che dipende dal vostro metodo di autenticazione e dall'abbonamento. Impostatelo esplicitamente (ad esempio, `model="claude-sonnet-5"`) per fissare un modello specifico o per utilizzare un modello più piccolo per agenti più veloci e economici. Vedere [models](https://platform.claude.com/docs/en/about-claude/models) per gli ID disponibili.

<h2 id="the-context-window">
  La finestra di contesto
</h2>

La finestra di contesto è la quantità totale di informazioni disponibili a Claude durante una sessione. Non si ripristina tra i turni all'interno di una sessione. Tutto si accumula: il prompt di sistema, le definizioni degli strumenti, la cronologia della conversazione, gli input degli strumenti e gli output degli strumenti. Il contenuto che rimane lo stesso tra i turni (prompt di sistema, definizioni degli strumenti, CLAUDE.md) viene automaticamente [prompt cached](https://platform.claude.com/docs/it/build-with-claude/prompt-caching), il che riduce il costo e la latenza per i prefissi ripetuti.

<h3 id="what-consumes-context">
  Cosa consuma il contesto
</h3>

Ecco come ogni componente influisce sul contesto nell'SDK:

| Fonte                              | Quando viene caricato                                                                 | Impatto                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :--------------------------------- | :------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Prompt di sistema**              | Ogni richiesta                                                                        | Costo fisso piccolo, sempre presente                                                                                                                                                                                                                                                                                                                                                                                                              |
| **File CLAUDE.md**                 | Inizio della sessione, tramite [`settingSources`](/docs/it/agent-sdk/claude-code-features) | Contenuto completo in ogni richiesta (ma prompt-cached, quindi solo la prima richiesta paga il costo completo)                                                                                                                                                                                                                                                                                                                                    |
| **Definizioni degli strumenti**    | Ogni richiesta; schemi MCP differiti per impostazione predefinita                     | Gli schemi degli strumenti integrati vengono caricati ad ogni richiesta. [Tool search](/docs/it/agent-sdk/mcp#mcp-tool-search) differisce gli schemi degli strumenti MCP per impostazione predefinita, ricadendo nel caricamento anticipato su Google Cloud's Agent Platform o su un `ANTHROPIC_BASE_URL` non di prima parte. Vedere [Configurare la ricerca degli strumenti](/docs/it/agent-sdk/tool-search#configure-tool-search) per la matrice completa |
| **Cronologia della conversazione** | Si accumula nel corso dei turni                                                       | Cresce con ogni turno: prompt, risposte, input degli strumenti, output degli strumenti                                                                                                                                                                                                                                                                                                                                                            |
| **Descrizioni delle skills**       | Inizio della sessione, tramite setting sources                                        | Brevi riassunti; il contenuto completo viene caricato solo quando invocato                                                                                                                                                                                                                                                                                                                                                                        |

I grandi output degli strumenti consumano un contesto significativo. Leggere un file grande o eseguire un comando con output dettagliato può utilizzare migliaia di token in un singolo turno. Il contesto si accumula nel corso dei turni, quindi le sessioni più lunghe con molte chiamate di strumenti accumulano significativamente più contesto rispetto a quelle brevi.

<h3 id="automatic-compaction">
  Compattazione automatica
</h3>

Quando la finestra di contesto si avvicina al suo limite, l'SDK compatta automaticamente la conversazione: riassume la cronologia più vecchia per liberare spazio, mantenendo intatti gli scambi più recenti e le decisioni chiave. L'SDK emette un messaggio con `type: "system"` e `subtype: "compact_boundary"` nel flusso quando questo accade (in Python questo è un `SystemMessage`; in TypeScript è un tipo separato `SDKCompactBoundaryMessage`).

La compattazione sostituisce i messaggi più vecchi con un riassunto, quindi le istruzioni specifiche dall'inizio della conversazione potrebbero non essere preservate. Le regole persistenti appartengono a CLAUDE.md (caricato tramite [`settingSources`](/docs/it/agent-sdk/claude-code-features)) piuttosto che al prompt iniziale, perché il contenuto di CLAUDE.md viene reiniettato ad ogni richiesta.

Potete personalizzare il comportamento della compattazione in diversi modi:

* **Istruzioni di riassunto in CLAUDE.md:** Il compattatore legge il vostro CLAUDE.md come qualsiasi altro contesto, quindi potete includere una sezione che gli dice cosa preservare quando riassume. L'intestazione della sezione è libera (non una stringa magica); il compattatore corrisponde all'intento.
* **Hook `PreCompact`:** Eseguire logica personalizzata prima che si verifichi la compattazione, ad esempio per archiviare la trascrizione completa. L'hook riceve un campo `trigger` (`manual` o `auto`). Vedere [hooks](/docs/it/agent-sdk/hooks).
* **Compattazione manuale:** Inviare `/compact` come stringa di prompt per attivare la compattazione su richiesta. I comandi inviati in questo modo sono input SDK, non scorciatoie solo CLI. Vedere [slash commands](/docs/it/agent-sdk/slash-commands).

<Accordion title="Esempio: Istruzioni di riassunto in CLAUDE.md">
  Aggiungete una sezione al vostro CLAUDE.md del progetto dicendo al compattatore cosa preservare. Il nome dell'intestazione non è speciale; utilizzate qualsiasi etichetta chiara.

  ```markdown CLAUDE.md theme={null}
  # Summary instructions

  When summarizing this conversation, always preserve:
  - The current task objective and acceptance criteria
  - File paths that have been read or modified
  - Test results and error messages
  - Decisions made and the reasoning behind them
  ```
</Accordion>

<h3 id="keep-context-efficient">
  Mantenere il contesto efficiente
</h3>

Alcune strategie per gli agenti a lunga durata:

* **Utilizzare subagenti per sottoattività.** Ogni subagente inizia con una conversazione fresca (nessuna cronologia di messaggi precedenti, anche se carica il suo prompt di sistema e il contesto a livello di progetto come CLAUDE.md). Non vede i turni del genitore e solo la sua risposta finale ritorna al genitore come risultato dello strumento. Il contesto dell'agente principale cresce per quel riassunto, non per la trascrizione completa della sottoattività. Vedere [Cosa ereditano i subagenti](/docs/it/agent-sdk/subagents#what-subagents-inherit) per i dettagli.
* **Essere selettivi con gli strumenti.** Ogni definizione di strumento occupa spazio di contesto. Utilizzate il campo `tools` su [`AgentDefinition`](/docs/it/agent-sdk/subagents#agentdefinition-configuration) per limitare i subagenti al set minimo di cui hanno bisogno.
* **Controllare i costi dei server MCP.** [MCP tool search](/docs/it/agent-sdk/mcp#mcp-tool-search) differisce gli schemi degli strumenti MCP per impostazione predefinita e li carica su richiesta. Quando la ricerca degli strumenti è disattivata, su Google Cloud's Agent Platform, o dietro un `ANTHROPIC_BASE_URL` non di prima parte, ogni server MCP aggiunge tutti i suoi schemi di strumenti ad ogni richiesta, quindi pochi server con molti strumenti possono consumare un contesto significativo prima che l'agente faccia qualsiasi lavoro.
* **Utilizzare uno sforzo inferiore per attività di routine.** Impostare [effort](#effort-level) a `"low"` per gli agenti che hanno solo bisogno di leggere file o elencare directory. Questo riduce l'utilizzo dei token e il costo.

Per una ripartizione dettagliata dei costi di contesto per funzione, vedere [Comprendere i costi di contesto](/docs/it/features-overview#understand-context-costs).

<h2 id="sessions-and-continuity">
  Sessioni e continuità
</h2>

Ogni interazione con l'SDK crea o continua una sessione. Catturate l'ID della sessione da `ResultMessage.session_id` (disponibile in entrambi gli SDK) per riprendere in seguito. L'SDK TypeScript lo espone anche come campo diretto sul `SystemMessage` init; in Python è annidato in `SystemMessage.data`.

Quando riprendete, il contesto completo dai turni precedenti viene ripristinato: file che sono stati letti, analisi che è stata eseguita e azioni che sono state intraprese. Potete anche fare un fork di una sessione per diramarvisi in un approccio diverso senza modificare l'originale.

Vedere [Gestione della sessione](/docs/it/agent-sdk/sessions) per la guida completa sui pattern di ripresa, continuazione e fork.

<Note>
  In Python, `ClaudeSDKClient` gestisce gli ID di sessione automaticamente tra più chiamate. Vedere il [riferimento SDK Python](/docs/it/agent-sdk/python#choosing-between-query-and-claudesdkclient) per i dettagli.
</Note>

<h2 id="handle-the-result">
  Gestire il risultato
</h2>

Quando il ciclo termina, il `ResultMessage` vi dice cosa è successo e vi fornisce l'output. Il campo `subtype` (disponibile in entrambi gli SDK) è il modo principale per controllare lo stato di terminazione.

| Sottotipo del risultato               | Cosa è successo                                                                                                                                                                                                                     | Campo `result` disponibile? |
| :------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------: |
| `success`                             | Claude ha completato l'attività normalmente                                                                                                                                                                                         |              Sì             |
| `error_max_turns`                     | Ha raggiunto il limite di `maxTurns` prima di terminare                                                                                                                                                                             |              No             |
| `error_max_budget_usd`                | Ha raggiunto il limite di `maxBudgetUsd` prima di terminare                                                                                                                                                                         |              No             |
| `error_during_execution`              | Un errore ha interrotto il ciclo (ad esempio, un errore API o una richiesta annullata)                                                                                                                                              |              No             |
| `error_max_structured_output_retries` | Nessun output strutturato valido è stato prodotto entro il limite di tentativi configurato: ogni tentativo ha fallito la convalida, oppure un fallback del modello ha ritrattato l'output completato senza alcun tentativo riuscito |              No             |

Il campo `result` (l'output di testo finale) è presente solo sulla variante `success`, quindi controllate sempre il sottotipo prima di leggerlo. Tutti i sottotipi di risultato portano `total_cost_usd`, `usage`, `num_turns` e `session_id` in modo che possiate tracciare il costo e riprendere anche dopo gli errori. In Python, `total_cost_usd` e `usage` sono tipizzati come opzionali e possono essere `None` su alcuni percorsi di errore, quindi proteggete prima di formattarli. Vedere [Tracciamento di costi e utilizzo](/docs/it/agent-sdk/cost-tracking) per i dettagli sull'interpretazione dei campi `usage`.

<Note>
  Quando una query termina con un risultato di errore:

  * Una singola chiamata `query()` produce il messaggio di risultato finale, quindi genera un errore che include il testo dell'errore, come `Reached maximum number of turns`. L'eccezione è intenzionale — avvolgete il ciclo in un blocco try se il vostro codice deve continuare oltre. Anche il processo Claude Code sottostante esce con un codice diverso da zero.
  * Una sessione di input in streaming rimane attiva e potete continuare a inviare messaggi.
</Note>

Il risultato include anche un campo `stop_reason` (`string | null` in TypeScript, `str | None` in Python) che indica perché il modello ha smesso di generare al suo turno finale. I valori comuni sono `end_turn` (il modello ha terminato normalmente), `max_tokens` (ha raggiunto il limite di token di output) e `refusal` (il modello ha rifiutato la richiesta). Su sottotipi di risultato di errore, `stop_reason` porta il valore dall'ultima risposta dell'assistente prima che il ciclo terminasse. Per rilevare i rifiuti, controllate `stop_reason === "refusal"` (TypeScript) o `stop_reason == "refusal"` (Python). Vedere [`SDKResultMessage`](/docs/it/agent-sdk/typescript#sdkresultmessage) (TypeScript) o [`ResultMessage`](/docs/it/agent-sdk/python#resultmessage) (Python) per il tipo completo.

<h2 id="hooks">
  Hooks
</h2>

[Hooks](/docs/it/agent-sdk/hooks) sono callback che si attivano in punti specifici del ciclo: prima che uno strumento venga eseguito, dopo che ritorna, quando l'agente termina e così via. Alcuni hook comunemente utilizzati sono:

| Hook                             | Quando si attiva                                | Usi comuni                                                 |
| :------------------------------- | :---------------------------------------------- | :--------------------------------------------------------- |
| `PreToolUse`                     | Prima che uno strumento venga eseguito          | Convalidare gli input, bloccare i comandi pericolosi       |
| `PostToolUse`                    | Dopo che uno strumento ritorna                  | Controllare gli output, attivare effetti collaterali       |
| `UserPromptSubmit`               | Quando un prompt viene inviato                  | Iniettare contesto aggiuntivo nei prompt                   |
| `Stop`                           | Quando l'agente termina                         | Convalidare il risultato, salvare lo stato della sessione  |
| `SubagentStart` / `SubagentStop` | Quando un subagente viene generato o completato | Tracciare e aggregare i risultati delle attività parallele |
| `PreCompact`                     | Prima della compattazione del contesto          | Archiviare la trascrizione completa prima di riassumere    |

Gli hook vengono eseguiti nel vostro processo di applicazione, non all'interno della finestra di contesto dell'agente, quindi non consumano contesto. Gli hook possono anche cortocircuitare il ciclo: un hook `PreToolUse` che rifiuta una chiamata di strumento impedisce che venga eseguita e Claude riceve il messaggio di rifiuto invece.

Entrambi gli SDK supportano tutti gli eventi precedenti. L'SDK TypeScript include eventi aggiuntivi che Python non supporta ancora. Vedere [Controllare l'esecuzione con gli hooks](/docs/it/agent-sdk/hooks) per l'elenco completo degli eventi, la disponibilità per SDK e l'API di callback completa.

<h2 id="put-it-all-together">
  Mettere tutto insieme
</h2>

Questo esempio combina i concetti chiave di questa pagina in un singolo agente che corregge i test falliti. Configura l'agente con strumenti consentiti (pre-approvati in modo che l'agente funzioni autonomamente), impostazioni del progetto e limiti di sicurezza su turni e sforzo di ragionamento. Mentre il ciclo viene eseguito, cattura l'ID della sessione per una potenziale ripresa, gestisce il risultato finale e stampa il costo totale.

Poiché una singola chiamata `query()` genera un'eccezione dopo aver restituito un risultato di errore, il ciclo è avvolto in un blocco try in modo che lo script esca correttamente quando viene raggiunto un limite.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def run_agent():
      session_id = None

      try:
          async for message in query(
              prompt="Find and fix the bug causing test failures in the auth module",
              options=ClaudeAgentOptions(
                  allowed_tools=[
                      "Read",
                      "Edit",
                      "Bash",
                      "Glob",
                      "Grep",
                  ],  # Listing tools here auto-approves them (no prompting)
                  setting_sources=[
                      "project"
                  ],  # Load CLAUDE.md, skills, hooks from current directory
                  max_turns=30,  # Prevent runaway sessions
                  effort="high",  # Thorough reasoning for complex debugging
              ),
          ):
              # Handle the final result
              if isinstance(message, ResultMessage):
                  session_id = message.session_id  # Save for potential resumption

                  if message.subtype == "success":
                      print(f"Done: {message.result}")
                  elif message.subtype == "error_max_turns":
                      # Agent ran out of turns. Resume with a higher limit.
                      print(f"Hit turn limit. Resume session {session_id} to continue.")
                  elif message.subtype == "error_max_budget_usd":
                      print("Hit budget limit.")
                  else:
                      print(f"Stopped: {message.subtype}")
                  if message.total_cost_usd is not None:
                      print(f"Cost: ${message.total_cost_usd:.4f}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branches above have
          # already run; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(run_agent())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Find and fix the bug causing test failures in the auth module",
      options: {
        allowedTools: ["Read", "Edit", "Bash", "Glob", "Grep"], // Listing tools here auto-approves them (no prompting)
        settingSources: ["project"], // Load CLAUDE.md, skills, hooks from current directory
        maxTurns: 30, // Prevent runaway sessions
        effort: "high" // Thorough reasoning for complex debugging
      }
    })) {
      // Save the session ID to resume later if needed
      if (message.type === "system" && message.subtype === "init") {
        sessionId = message.session_id;
      }

      // Handle the final result
      if (message.type === "result") {
        if (message.subtype === "success") {
          console.log(`Done: ${message.result}`);
        } else if (message.subtype === "error_max_turns") {
          // Agent ran out of turns. Resume with a higher limit.
          console.log(`Hit turn limit. Resume session ${sessionId} to continue.`);
        } else if (message.subtype === "error_max_budget_usd") {
          console.log("Hit budget limit.");
        } else {
          console.log(`Stopped: ${message.subtype}`);
        }
        console.log(`Cost: $${message.total_cost_usd.toFixed(4)}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branches above have
    // already run; connection or process failures yield no result message.
    console.log(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Passaggi successivi
</h2>

Ora che comprendete il ciclo, ecco dove andare a seconda di ciò che state costruendo:

* **Non avete ancora eseguito un agente?** Iniziate con la [quickstart](/docs/it/agent-sdk/quickstart) per ottenere l'SDK installato e vedere un esempio completo in esecuzione da capo a fondo.
* **Pronti a collegarvi al vostro progetto?** [Caricate CLAUDE.md, skills e filesystem hooks](/docs/it/agent-sdk/claude-code-features) in modo che l'agente segua automaticamente le convenzioni del vostro progetto.
* **State costruendo un'interfaccia utente interattiva?** Abilitate lo [streaming](/docs/it/agent-sdk/streaming-output) per mostrare testo e chiamate di strumenti in tempo reale mentre il ciclo viene eseguito.
* **Avete bisogno di un controllo più stretto su ciò che l'agente può fare?** Bloccate l'accesso agli strumenti con [autorizzazioni](/docs/it/agent-sdk/permissions) e utilizzate [hooks](/docs/it/agent-sdk/hooks) per controllare, bloccare o trasformare le chiamate di strumenti prima che vengano eseguite.
* **State eseguendo attività lunghe o costose?** Delegate il lavoro isolato ai [subagenti](/docs/it/agent-sdk/subagents) per mantenere il vostro contesto principale snello.

Per il quadro concettuale più ampio del ciclo agentic (non specifico dell'SDK), vedere [Come funziona Claude Code](/docs/it/how-claude-code-works). Per una guida pratica alla progettazione di cicli in Claude Code, dai cicli basati su turni ai cicli basati su obiettivi e cicli proattivi, vedere [Loop engineering: getting started with loops](https://claude.com/blog/getting-started-with-loops) sul blog.
