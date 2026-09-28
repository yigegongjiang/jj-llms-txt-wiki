> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tracciare costi e utilizzo

> Scopri come tracciare l'utilizzo dei token, stimare i costi e configurare la memorizzazione nella cache dei prompt con Claude Agent SDK.

Claude Agent SDK fornisce informazioni dettagliate sull'utilizzo dei token per ogni interazione con Claude. Questa guida spiega come tracciare correttamente l'utilizzo e comprendere la segnalazione dei costi, soprattutto quando si affrontano usi paralleli di strumenti e conversazioni multi-step.

Per la documentazione API completa, consulta il [riferimento TypeScript SDK](/docs/it/agent-sdk/typescript) e il [riferimento Python SDK](/docs/it/agent-sdk/python).

<Warning>
  I campi `total_cost_usd` e `costUSD` sono stime lato client, non dati di fatturazione autorevoli. L'SDK li calcola localmente da una tabella dei prezzi inclusa al momento della compilazione, a meno che non sia in vigore una tabella [`modelPricing`](/docs/it/settings-reference#modelpricing). Possono divergere da ciò che viene effettivamente fatturato quando:

  * i prezzi cambiano
  * la versione dell'SDK installata non riconosce un modello
  * si applicano regole di fatturazione che il client non può modellare

  Una regola di fatturazione che l'SDK modella è la [determinazione dei prezzi per la residenza dei dati](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing). Quando la `usage` di una risposta segnala `inference_geo: "us"`, l'SDK moltiplica il prezzo di listino dei token di quella risposta per 1,1. Le tariffe per richiesta come la ricerca web non vengono moltiplicate. Richiede TypeScript Agent SDK v0.3.239 o successivo, oppure Python Agent SDK v0.2.144 o successivo.

  Utilizza questi campi per approfondimenti di sviluppo e budget approssimativi. Per la fatturazione autorevole, utilizza l'[API di utilizzo e costi](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) o la pagina Utilizzo nella [Claude Console](https://platform.claude.com/usage). Non fatturare gli utenti finali o attivare decisioni finanziarie da questi campi.
</Warning>

<h2 id="understand-token-usage">
  Comprendere l'utilizzo dei token
</h2>

Gli SDK TypeScript e Python espongono gli stessi dati di utilizzo con nomi di campi diversi:

* **TypeScript** fornisce suddivisioni dei token per fase su ogni messaggio dell'assistente (`message.message.id`, `message.message.usage`), costo per modello tramite `modelUsage` sul messaggio risultato, e un totale cumulativo sul messaggio risultato.
* **Python** fornisce suddivisioni dei token per fase su ogni messaggio dell'assistente come `message.usage` e `message.message_id`, costo per modello tramite `model_usage` sul messaggio risultato, e il totale cumulativo sul messaggio risultato come `total_cost_usd`.

Entrambi gli SDK utilizzano lo stesso modello di costo sottostante ed espongono la stessa granularità. La differenza è nella denominazione dei campi e nel modo in cui l'utilizzo per fase è annidato.

Il tracciamento dei costi dipende dalla comprensione di come l'SDK delimita i dati di utilizzo:

* **Chiamata `query()`:** una singola invocazione della funzione `query()` dell'SDK. Una singola chiamata può coinvolgere più fasi: Claude risponde, utilizza strumenti, ottiene risultati e risponde di nuovo. Ogni chiamata produce un messaggio [`result`](/docs/it/agent-sdk/typescript#sdkresultmessage) alla fine, tranne in [modalità input streaming](/docs/it/agent-sdk/streaming-vs-single-mode), dove una chiamata `query()` comporta più turni dell'utente e ogni turno emette il proprio messaggio `result`.
* **Fase:** un singolo ciclo richiesta/risposta all'interno di una chiamata `query()`. Ogni fase produce messaggi dell'assistente con utilizzo dei token.
* **Sessione:** una serie di chiamate `query()` collegate da un ID di sessione tramite l'opzione `resume`. I risultati di una chiamata ripresa segnalano la spesa totale della sessione, non solo quella della chiamata stessa. Vedere [Accumulare i costi su più chiamate](#accumulate-costs-across-multiple-calls) per come i totali si trasferiscono.

Il diagramma seguente mostra il flusso di messaggi da una singola chiamata `query()`, con l'utilizzo dei token segnalato ad ogni fase e la stima cumulativa alla fine:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" className="dark:hidden" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/message-usage-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=8ea95085abc0a6b7f55ecef498bd4d14" className="hidden dark:block" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow-dark.svg" />

<Steps>
  <Step title="Ogni fase produce messaggi dell'assistente">
    Quando Claude risponde, invia uno o più messaggi dell'assistente. In TypeScript, ogni messaggio dell'assistente contiene un `BetaMessage` annidato (accessibile tramite `message.message`) con un `id` e un oggetto [`usage`](https://platform.claude.com/docs/en/api/messages) con conteggi dei token (`input_tokens`, `output_tokens`). In Python, la dataclass `AssistantMessage` espone gli stessi dati direttamente tramite `message.usage` e `message.message_id`. Quando Claude utilizza più strumenti in un turno, tutti i messaggi in quel turno condividono lo stesso ID, quindi deduplicare per ID per evitare il doppio conteggio.
  </Step>

  <Step title="Il messaggio risultato fornisce la stima cumulativa">
    Quando la chiamata `query()` si completa, l'SDK emette un messaggio risultato con `total_cost_usd` e `usage` cumulativo, tipizzato come [`SDKResultMessage`](/docs/it/agent-sdk/typescript#sdkresultmessage) in TypeScript e [`ResultMessage`](/docs/it/agent-sdk/python#resultmessage) in Python. Se è necessario solo il totale stimato, è possibile ignorare l'utilizzo per fase e leggere questo singolo valore.

    Se si effettuano più chiamate `query()` indipendenti, ogni risultato riflette solo il costo di quella singola chiamata. Una chiamata che riprende una sessione conta anche la spesa precedente della sessione.

    In modalità input streaming, ogni turno emette il proprio messaggio risultato. Vedere [Tracciare i costi in modalità input streaming](#track-costs-in-streaming-input-mode) per come leggere i totali delle chiamate in quella modalità.
  </Step>
</Steps>

<h2 id="track-costs-in-streaming-input-mode">
  Tracciare i costi in modalità di input in streaming
</h2>

In [modalità di input in streaming](/docs/it/agent-sdk/streaming-vs-single-mode), una singola chiamata `query()` contiene più turni utente e ogni turno emette il proprio messaggio di risultato. I campi del risultato differiscono in ambito:

* **`usage`**: copre solo quel turno, e all'interno di esso solo il ciclo principale dell'agente, non eventuali subagenti che ha eseguito.
* **`total_cost_usd` e `modelUsage`, o `model_usage` in Python**: portano il totale cumulativo per l'intera chiamata fino a quel momento, più qualsiasi spesa ripristinata quando la chiamata ha ripreso una sessione.

In una chiamata in cui la vostra app non invia mai `/clear`, `/reset`, o `/new`, leggete il risultato più recente per i totali della chiamata piuttosto che sommare i risultati.

I totali cumulativi ricominciamo ogni volta che la vostra app invia uno di questi tre comandi, e all'interno di una chiamata `query()` nient'altro li ripristina. Tre risultati sono importanti per la vostra contabilità:

* **Il risultato del turno `/clear`**: copre solo ciò che è stato eseguito dal ripristino, e porta un nuovo `session_id`.
* **Ogni risultato successivo**: continua a contare da quel ripristino.
* **L'ultimo risultato prima di ogni `/clear`**: contiene il totale per i turni dal ripristino precedente.

Per totalizzare l'intera chiamata, aggiungete l'ultimo risultato prima di ogni `/clear` al risultato finale della chiamata. Ogni altro risultato, incluso quello del turno `/clear`, è sostituito da uno successivo.

In TypeScript, l'SDK emette anche un [`SDKConversationResetMessage`](/docs/it/agent-sdk/typescript#sdkconversationresetmessage) ad ogni ripristino, quindi potete rilevare i ripristini dal flusso. In Python, l'SDK emette analogamente un `ConversationResetMessage`. Prima della versione Python SDK v0.2.137, l'iteratore Python ha eliminato quel messaggio, quindi su quelle versioni contate i ripristini voi stessi dai turni `/clear` che la vostra app invia.

`maxBudgetUsd` (TypeScript) o `max_budget_usd` (Python) conta solo la spesa della chiamata stessa: i totali ripristinati da una sessione ripresa non contano rispetto ad esso, e un `/clear` avvia il budget da capo.

<h2 id="get-the-total-cost-of-a-query">
  Ottenere il costo totale di una query
</h2>

Il messaggio di risultato, tipizzato come [`SDKResultMessage`](/docs/it/agent-sdk/typescript#sdkresultmessage) in TypeScript e [`ResultMessage`](/docs/it/agent-sdk/python#resultmessage) in Python, segna la fine del ciclo dell'agente per una chiamata `query()`. Include `total_cost_usd`, il costo stimato cumulativo su tutti i passaggi in quella chiamata. Una chiamata che riprende una sessione conta anche la spesa precedente della sessione. Due avvertenze si applicano quando leggete il valore:

* In Python il campo è tipizzato come opzionale, quindi verificate che non sia `None` prima di leggerlo.
* I risultati di successo e di errore lo portano entrambi, anche se il risultato finale di un [crash della sessione](#recover-totals-after-a-session-crash) potrebbe portarlo azzerato.

In modalità di input in streaming, leggete i totali delle chiamate come descritto in [Track costs in streaming input mode](#track-costs-in-streaming-input-mode).

I tre campi a livello di risultato differiscono in ciò che contano quando l'agente genera [subagenti](/docs/it/agent-sdk/subagents). Utilizzate `modelUsage`, o `model_usage` in Python, per la contabilità dei token dell'intero albero; il campo `usage` sottoconta non appena si verifica l'annidamento.

| Campo                        | Attività del subagente                                                                                                             |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Escluso. Conta solo il ciclo dell'agente di primo livello, quindi i token consumati all'interno dei subagenti non vengono aggiunti |
| `total_cost_usd`             | Incluso. Conta le richieste dei subagenti insieme al ciclo di primo livello                                                        |
| `modelUsage` / `model_usage` | Incluso. Conta le richieste dei subagenti insieme al ciclo di primo livello, suddiviso per modello                                 |

In [modalità di input a messaggio singolo](/docs/it/agent-sdk/streaming-vs-single-mode#single-message-input), quando i subagenti in background sono ancora in esecuzione alla fine del turno finale, Claude Code li attende, fino al limite descritto in [background tasks at exit](/docs/it/headless#background-tasks-at-exit), prima di emettere il risultato. Il `total_cost_usd`, `duration_api_ms` e `modelUsage` del risultato, o `model_usage` in Python, includono il lavoro svolto durante l'attesa.

I seguenti esempi iterano sul flusso di messaggi da una chiamata `query()` e stampano il costo totale quando arriva il messaggio `result`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({ prompt: "Summarize this project" })) {
      if (message.type === "result") {
        console.log(`Total cost: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, it still carried total_cost_usd and the
    // branch above has already run; connection or process failures yield
    // no result message.
    console.error(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      try:
          async for message in query(prompt="Summarize this project"):
              if isinstance(message, ResultMessage):
                  print(f"Total cost: ${message.total_cost_usd or 0}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the branch above has already run;
          # connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Per limitare quanto i subagenti possono aggiungere a `total_cost_usd`, impostate i [limiti di profondità, concorrenza e spesa](/docs/it/agent-sdk/subagents#cap-subagent-depth-concurrency-and-spend) sulla query.

<h2 id="track-per-step-and-per-model-usage">
  Tracciare l'utilizzo per step e per modello
</h2>

Gli esempi in questa sezione utilizzano nomi di campi TypeScript. In Python, i campi equivalenti sono [`AssistantMessage.usage`](/docs/it/agent-sdk/python#assistantmessage) e `AssistantMessage.message_id` per l'utilizzo per step, e [`ResultMessage.model_usage`](/docs/it/agent-sdk/python#resultmessage) per i dettagli per modello.

<h3 id="track-per-step-usage">
  Tracciare l'utilizzo per step
</h3>

Ogni messaggio dell'assistente contiene un `BetaMessage` annidato (accessibile tramite `message.message`) con un `id` e un oggetto `usage` con i conteggi dei token. Quando Claude utilizza gli strumenti in parallelo, più messaggi condividono lo stesso `id` con dati di utilizzo identici. Tenere traccia degli ID che hai già contato e saltare i duplicati per evitare totali gonfiati.

<Warning>
  I valori per step deduplicati sono accurati per i token di input e cache. L'`output_tokens` per step è un placeholder, quindi [leggi i token di output dal messaggio di risultato](#read-output-tokens-from-the-result-message).
</Warning>

L'esempio seguente accumula i token di input in tutti gli step, contando ogni ID di messaggio del loop principale univoco una sola volta e saltando i messaggi dei subagent, e legge il totale di output dal messaggio di risultato, che copre il loop principale:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let resultOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant" && !message.parent_tool_use_id) {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
      }
    }
    if (message.type === "result") {
      // Per-step output_tokens is a placeholder; the result message
      // carries the accumulated output total.
      resultOutputTokens = message.usage.output_tokens;
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // input total below still reflects the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${resultOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Dettagliare l'utilizzo per modello
</h3>

Il messaggio di risultato include [`modelUsage`](/docs/it/agent-sdk/typescript#modelusage), una mappa del nome del modello ai conteggi dei token per modello e al costo. Questo è utile quando esegui più modelli (ad esempio, Haiku per i subagent e Opus per l'agente principale) e desideri vedere dove vanno i token.

Il `costBasis` di ogni voce indica quale tabella dei prezzi ha determinato il prezzo della richiesta più recente di quel modello: `list` per il prezzo di listino, `managed` per una tabella [`modelPricing`](/docs/it/settings-reference#modelpricing), o `unknown` quando nessuno dei due corrisponde all'ID del modello. Il campo richiede Claude Code v2.1.246 o successivo.

L'esempio seguente esegue una query e stampa il costo e il dettaglio dei token per ogni modello utilizzato:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type !== "result") continue;

    for (const [modelName, usage] of Object.entries(message.modelUsage)) {
      console.log(`${modelName}: $${usage.costUSD.toFixed(4)}`);
      console.log(`  Input tokens: ${usage.inputTokens}`);
      console.log(`  Output tokens: ${usage.outputTokens}`);
      console.log(`  Cache read: ${usage.cacheReadInputTokens}`);
      console.log(`  Cache creation: ${usage.cacheCreationInputTokens}`);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result. If the
  // failure was an error result, the per-model breakdown above has already
  // printed; connection or process failures yield no result message.
  console.error(`Session ended with an error: ${error}`);
}
```

<h2 id="accumulate-costs-across-multiple-calls">
  Accumulare i costi su più chiamate
</h2>

Ogni chiamata `query()` restituisce `total_cost_usd` sui suoi risultati. Come combinare i valori dipende dal fatto che le chiamate condividano una sessione:

* **Chiamate indipendenti, senza opzione `resume` o `continue`**: ogni risultato copre solo la propria chiamata, quindi aggiungete i totali voi stessi, come fanno gli esempi seguenti.
* **Chiamate che riprendono la stessa sessione**: Claude Code salva i totali della sessione nel suo [transcript](/docs/it/sessions#where-transcripts-are-stored) quando il processo esce normalmente e li ripristina quando una chiamata successiva riprende o effettua il fork della sessione. Ogni risultato include già la spesa precedente della sessione. Leggete l'ultimo risultato per il totale della sessione; sommare i risultati conta due volte la spesa ripristinata. Prima della v2.1.277, una sessione che avevate ripreso tramite l'SDK o `claude -p` iniziava i suoi totali a zero, quindi i risultati di ogni chiamata coprivano solo quella chiamata.

In modalità input streaming, leggete il totale di ogni chiamata come descritto in [Track costs in streaming input mode](#track-costs-in-streaming-input-mode). Per una chiamata che si è conclusa con un crash, vedere [Recover totals after a session crash](#recover-totals-after-a-session-crash).

I seguenti esempi eseguono due chiamate `query()` in sequenza, aggiungono il `total_cost_usd` di ogni chiamata a un totale progressivo e stampano sia il costo per singola chiamata che il costo combinato:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track cumulative cost across multiple query() calls
  let totalSpend = 0;

  const prompts = [
    "Read the files in src/ and summarize the architecture",
    "List all exported functions in src/auth.ts"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({ prompt })) {
        if (message.type === "result") {
          totalSpend += message.total_cost_usd;
          console.log(`This call: $${message.total_cost_usd}`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, this call's cost was already counted;
      // connection or process failures yield no result message. Continue
      // with the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }

  console.log(`Total spend: $${totalSpend.toFixed(4)}`);
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      # Track cumulative cost across multiple query() calls
      total_spend = 0.0

      prompts = [
          "Read the files in src/ and summarize the architecture",
          "List all exported functions in src/auth.ts",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt):
                  if isinstance(message, ResultMessage):
                      cost = message.total_cost_usd or 0
                      total_spend += cost
                      print(f"This call: ${cost}")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. If
              # the failure was an error result, this call's cost was already
              # counted; connection or process failures yield no result message.
              # Continue with the next prompt.
              print(f"Call failed: {error}")

      print(f"Total spend: ${total_spend:.4f}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="handle-errors-caching-and-output-token-counts">
  Gestire errori, caching e conteggi dei token di output
</h2>

Per un tracciamento accurato dei costi, tenere conto del conteggio di output segnaposto sui messaggi dell'assistente, dei token che una conversazione non riuscita ha consumato e dei prezzi dei token della cache.

<h3 id="read-output-tokens-from-the-result-message">
  Leggere i token di output dal messaggio di risultato
</h3>

Claude Code costruisce ogni messaggio dell'assistente dall'utilizzo che l'API ha segnalato quando la risposta è iniziata, quindi il `output_tokens` del messaggio è solo il conteggio che l'API aveva segnalato a `message_start`, prima che la risposta fosse generata. Una risposta API può produrre diversi messaggi dell'assistente, e ognuno di essi porta lo stesso segnaposto.

L'API segnala il conteggio di output reale alla fine della risposta, e Claude Code lo aggiunge al messaggio di risultato. Leggere i token di output dal `usage` del risultato, o da `modelUsage` per una suddivisione per modello.

Per osservare il conteggio di output di una risposta crescere mentre viene trasmesso in streaming, impostare `includePartialMessages`, o `include_partial_messages` in Python, e leggere `usage` da ogni evento di flusso `message_delta`, tipizzato come [`SDKPartialAssistantMessage`](/docs/it/agent-sdk/typescript#sdkpartialassistantmessage) in TypeScript e [`StreamEvent`](/docs/it/agent-sdk/python#streamevent) in Python.

<h3 id="track-costs-on-failed-conversations">
  Tracciare i costi su conversazioni non riuscite
</h3>

Sia i messaggi di risultato di successo che di errore includono `usage` e `total_cost_usd`; in Python entrambi i campi sono tipizzati come opzionali, quindi verificare che non siano `None` prima di leggerli.

Se una conversazione non riesce a metà strada, hai comunque consumato token fino al punto del fallimento. Leggere i dati di costo da ogni messaggio di risultato, indipendentemente dal fatto che il suo `subtype` sia `success` o uno dei sottotipi di errore. Su alcuni risultati di errore, `usage` segnala meno di quanto la chiamata ha speso:

* **`error_during_execution` dopo un [arresto anomalo della sessione](#recover-totals-after-a-session-crash)**: ogni campo di costo può essere azzerato.
* **`error_max_budget_usd`**: `usage` omette la risposta che ha superato il budget, mentre `total_cost_usd` e `modelUsage` la includono.

Dove hai la scelta, contabilizzare da `total_cost_usd` o `modelUsage` piuttosto che da `usage`.

<h3 id="recover-totals-after-a-session-crash">
  Recuperare i totali dopo un arresto anomalo della sessione
</h3>

Quando il processo Claude Code si arresta in modo anomalo, emette un risultato `error_during_execution` finale e esce, sia in modalità input single-shot che in streaming. Quel risultato può portare `usage`, `total_cost_usd` e `modelUsage` azzerati, quindi recuperare i totali della chiamata da ciò che è arrivato prima. Il passaggio 1 recupera i totali completi ogni volta che esiste un risultato precedente; il fallback nel passaggio 2 recupera solo i token di input e cache del ciclo principale.

1. Utilizzare il risultato del turno prima dell'arresto anomalo. In modalità input streaming, contiene il totale in esecuzione descritto in [Track costs in streaming input mode](#track-costs-in-streaming-input-mode). Passare al passaggio 2 invece quando quel risultato non può aiutarti:
   * La chiamata era single-shot, quindi non esiste alcun risultato precedente.
   * L'arresto anomalo è avvenuto al primo turno.
   * Il turno prima dell'arresto anomalo era il `/clear` stesso, quindi il suo risultato copre solo il ripristino.
2. Sommare invece il `usage` sui messaggi dell'assistente, contando ogni risposta API una volta, come fa l'esempio [Track per-step usage](#track-per-step-usage). In modalità single-shot, sommare tutti; in modalità input streaming, sommare quelli arrivati dopo l'ultimo risultato. Questo ti dà i token di input e cache del ciclo principale. L'utilizzo dei subagent non è recuperabile in questo modo, e nemmeno i token di output o il costo in USD, perché [il `output_tokens` per passaggio è un segnaposto](#read-output-tokens-from-the-result-message).

<h3 id="track-cache-tokens">
  Tracciare i token della cache
</h3>

L'Agent SDK utilizza automaticamente [prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) per ridurre i costi su contenuti ripetuti. Non è necessario configurare il caching da soli. L'oggetto usage include due campi aggiuntivi per il tracciamento della cache:

* `cache_creation_input_tokens`: token utilizzati per creare nuove voci della cache (addebitati a una tariffa più alta rispetto ai token di input standard).
* `cache_read_input_tokens`: token letti da voci della cache esistenti (addebitati a una tariffa ridotta).

Tracciare questi separatamente da `input_tokens` per comprendere i risparmi della cache. In TypeScript, questi campi sono tipizzati sull'oggetto [`Usage`](/docs/it/agent-sdk/typescript#usage). In Python, appaiono come chiavi nel dizionario [`ResultMessage.usage`](/docs/it/agent-sdk/python#resultmessage) (ad esempio, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Estendere il TTL della cache del prompt a un'ora
</h3>

I tuoi turni rientrano nel [bucket TTL della conversazione principale](/docs/it/prompt-caching#which-ttl-each-request-gets), insieme ai helper che Claude Code esegue inline con essi. Le richieste che Claude Code effettua al di fuori di quella conversazione, come i [subagent](/docs/it/agent-sdk/subagents), hanno un [controllo TTL separato](/docs/it/prompt-caching#choose-the-ttl-yourself).

Le voci della cache per i tuoi turni utilizzano un TTL di 5 minuti per impostazione predefinita quando ti autentichi con una chiave API o esegui su Amazon Bedrock, Agent Platform di Google Cloud, Microsoft Foundry, o [Claude Platform on AWS](/docs/it/claude-platform-on-aws). Se il tuo carico di lavoro esegue molte sessioni brevi rispetto allo stesso prompt di sistema e contesto con gap più lunghi di 5 minuti tra di essi, la cache scade tra le sessioni e ogni nuova sessione paga il prezzo di input completo.

Per richiedere un TTL di 1 ora sulle scritture della cache, impostare la variabile di ambiente [`ENABLE_PROMPT_CACHING_1H`](/docs/it/env-vars). Puoi esportarla nel tuo ambiente shell o container, o passarla attraverso `options.env`.

L'esempio seguente abilita il TTL di 1 ora per un agente in esecuzione su Amazon Bedrock. Poiché imposta `CLAUDE_CODE_USE_BEDROCK`, richiede credenziali AWS funzionanti per [Amazon Bedrock](/docs/it/amazon-bedrock); senza di esse la query non riesce.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import ClaudeAgentOptions, query
  import asyncio


  async def main():
      options = ClaudeAgentOptions(
          env={
              "CLAUDE_CODE_USE_BEDROCK": "1",
              "ENABLE_PROMPT_CACHING_1H": "1",
          },
      )

      async for message in query(prompt="Summarize this project", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    env: {
      ...process.env,
      CLAUDE_CODE_USE_BEDROCK: "1",
      ENABLE_PROMPT_CACHING_1H: "1",
    },
  };

  for await (const message of query({ prompt: "Summarize this project", options })) {
    console.log(message);
  }
  ```
</CodeGroup>

Le scritture della cache con un TTL di 1 ora sono fatturate a una tariffa più alta rispetto alle scritture di 5 minuti, quindi abilitare questo scambia un costo di scrittura più elevato per più letture della cache. Vedi [prompt caching pricing](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) per i dettagli. Su un abbonamento Claude all'interno dell'utilizzo incluso nel tuo piano, ottieni il TTL di 1 ora sui tuoi turni, e su alcune delle richieste helper che Claude Code effettua accanto a essi, senza impostare questa variabile, e Claude Code riduce quei turni al TTL di 5 minuti una volta che stai attingendo ai [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans).

`ENABLE_PROMPT_CACHING_1H` richiede il TTL di 1 ora su ogni richiesta in entrambi i bucket. Per scegliere un TTL per ogni bucket separatamente, utilizza invece questi controlli. Ognuno accetta `5m` o `1h` e ha la precedenza su `ENABLE_PROMPT_CACHING_1H`:

* Conversazione principale: la variabile di ambiente [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/it/env-vars), o l'impostazione [`promptCacheTtl`](/docs/it/settings-reference#promptcachettl)
* Tutto il resto: la variabile di ambiente `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`, o l'impostazione [`subagentPromptCacheTtl`](/docs/it/settings-reference#subagentpromptcachettl)

Impostare `promptCacheTtl` a `1h` mantiene la cache di 1 ora sulla conversazione principale mentre stai attingendo ai crediti di utilizzo. Per l'ordine di precedenza completo, vedi [choose the TTL yourself](/docs/it/prompt-caching#choose-the-ttl-yourself).

<h2 id="related-documentation">
  Documentazione correlata
</h2>

* [Riferimento TypeScript SDK](/docs/it/agent-sdk/typescript) - Documentazione API completa
* [Panoramica SDK](/docs/it/agent-sdk/overview) - Introduzione all'SDK
* [Autorizzazioni SDK](/docs/it/agent-sdk/permissions) - Gestione delle autorizzazioni degli strumenti
