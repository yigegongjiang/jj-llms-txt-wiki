> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tracciare costi e utilizzo

> Scopri come tracciare l'utilizzo dei token, stimare i costi e configurare la memorizzazione nella cache dei prompt con Claude Agent SDK.

Claude Agent SDK fornisce informazioni dettagliate sull'utilizzo dei token per ogni interazione con Claude. Questa guida spiega come tracciare correttamente l'utilizzo e comprendere la segnalazione dei costi, soprattutto quando si affrontano utilizzi di strumenti paralleli e conversazioni multi-step.

Per la documentazione API completa, consulta il [riferimento TypeScript SDK](/docs/it/agent-sdk/typescript) e il [riferimento Python SDK](/docs/it/agent-sdk/python).

<Warning>
  I campi `total_cost_usd` e `costUSD` sono stime lato client, non dati di fatturazione autorevoli. L'SDK li calcola localmente da una tabella dei prezzi inclusa al momento della compilazione, quindi possono divergere da ciò che viene effettivamente fatturato quando:

  * i prezzi cambiano
  * la versione dell'SDK installata non riconosce un modello
  * si applicano regole di fatturazione che il client non può modellare

  Utilizza questi campi per approfondimenti di sviluppo e budget approssimativi. Per la fatturazione autorevole, utilizza l'[API di utilizzo e costi](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) o la pagina Utilizzo nella [Console Claude](https://platform.claude.com/usage). Non fatturare gli utenti finali o attivare decisioni finanziarie da questi campi.
</Warning>

<h2 id="understand-token-usage">
  Comprendere l'utilizzo dei token
</h2>

Gli SDK TypeScript e Python espongono gli stessi dati di utilizzo con nomi di campi diversi:

* **TypeScript** fornisce scomposizioni di token per step su ogni messaggio dell'assistente (`message.message.id`, `message.message.usage`), costi per modello tramite `modelUsage` sul messaggio risultato e un totale cumulativo sul messaggio risultato.
* **Python** fornisce scomposizioni di token per step su ogni messaggio dell'assistente (`message.usage`, `message.message_id`), costi per modello tramite `model_usage` sul messaggio risultato e il totale accumulato sul messaggio risultato (`total_cost_usd` e dizionario `usage`).

Entrambi gli SDK utilizzano lo stesso modello di costo sottostante ed espongono la stessa granularità. La differenza è nella denominazione dei campi e nel punto in cui l'utilizzo per step è annidato.

Il tracciamento dei costi dipende dalla comprensione di come l'SDK delimita i dati di utilizzo:

* **Chiamata `query()`:** una singola invocazione della funzione `query()` dell'SDK. Una singola chiamata può coinvolgere più step (Claude risponde, utilizza strumenti, ottiene risultati, risponde di nuovo). Ogni chiamata produce un messaggio [`result`](/docs/it/agent-sdk/typescript#sdkresultmessage) alla fine.
* **Step:** un singolo ciclo di richiesta/risposta all'interno di una chiamata `query()`. Ogni step produce messaggi dell'assistente con utilizzo dei token.
* **Sessione:** una serie di chiamate `query()` collegate da un ID di sessione (utilizzando l'opzione `resume`). Ogni chiamata `query()` all'interno di una sessione segnala il proprio costo in modo indipendente.

Il diagramma seguente mostra il flusso di messaggi da una singola chiamata `query()`, con utilizzo dei token segnalato ad ogni step e la stima cumulativa alla fine:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" alt="Diagramma che mostra una query che produce due step di messaggi. Lo Step 1 ha quattro messaggi dell'assistente che condividono lo stesso ID e utilizzo (contare una volta), lo Step 2 ha un messaggio dell'assistente con un nuovo ID e il messaggio risultato finale mostra il total_cost_usd stimato." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<Steps>
  <Step title="Ogni step produce messaggi dell'assistente">
    Quando Claude risponde, invia uno o più messaggi dell'assistente. In TypeScript, ogni messaggio dell'assistente contiene un `BetaMessage` annidato (accessibile tramite `message.message`) con un `id` e un oggetto [`usage`](https://platform.claude.com/docs/en/api/messages) con conteggi di token (`input_tokens`, `output_tokens`). In Python, la classe dataclass `AssistantMessage` espone gli stessi dati direttamente tramite `message.usage` e `message.message_id`. Quando Claude utilizza più strumenti in un turno, tutti i messaggi in quel turno condividono lo stesso ID, quindi deduplicare per ID per evitare il doppio conteggio.
  </Step>

  <Step title="Il messaggio risultato fornisce la stima cumulativa">
    Quando la chiamata `query()` si completa, l'SDK emette un messaggio risultato con `total_cost_usd` e `usage` cumulativo. Questo è disponibile sia in TypeScript ([`SDKResultMessage`](/docs/it/agent-sdk/typescript#sdkresultmessage)) che in Python ([`ResultMessage`](/docs/it/agent-sdk/python#resultmessage)). Se effettui più chiamate `query()` (ad esempio, in una sessione multi-turno), ogni risultato riflette solo il costo di quella singola chiamata. Se hai bisogno solo della stima totale, puoi ignorare l'utilizzo per step e leggere questo singolo valore.
  </Step>
</Steps>

<h2 id="get-the-total-cost-of-a-query">
  Ottenere il costo totale di una query
</h2>

Il messaggio risultato ([TypeScript](/docs/it/agent-sdk/typescript#sdkresultmessage), [Python](/docs/it/agent-sdk/python#resultmessage)) segna la fine del ciclo dell'agente per una chiamata `query()`. Include `total_cost_usd`, il costo stimato cumulativo su tutti gli step in quella chiamata. Questo funziona sia per risultati di successo che di errore. Se utilizzi sessioni per effettuare più chiamate `query()`, ogni risultato riflette solo il costo di quella singola chiamata.

I tre campi a livello di risultato differiscono in ciò che contano quando l'agente genera [subagenti](/docs/it/agent-sdk/subagents). Utilizza `modelUsage`, o `model_usage` in Python, per la contabilità dei token dell'intero albero; il campo `usage` sottoconta non appena si verifica l'annidamento.

| Campo                        | Attività subagente                                                                                                                 |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Escluso. Conta solo il ciclo dell'agente di primo livello, quindi i token consumati all'interno dei subagenti non vengono aggiunti |
| `total_cost_usd`             | Incluso. Conta le richieste dei subagenti insieme al ciclo di primo livello                                                        |
| `modelUsage` / `model_usage` | Incluso. Conta le richieste dei subagenti insieme al ciclo di primo livello, suddiviso per modello                                 |

Gli esempi seguenti iterano sul flusso di messaggi da una chiamata `query()` e stampano il costo totale quando arriva il messaggio `result`:

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
          # failure was an error result, it still carried total_cost_usd and the
          # branch above has already run; connection or process failures yield
          # no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="track-per-step-and-per-model-usage">
  Tracciare l'utilizzo per step e per modello
</h2>

Gli esempi in questa sezione utilizzano nomi di campi TypeScript. In Python, i campi equivalenti sono [`AssistantMessage.usage`](/docs/it/agent-sdk/python#assistantmessage) e `AssistantMessage.message_id` per l'utilizzo per step, e [`ResultMessage.model_usage`](/docs/it/agent-sdk/python#resultmessage) per le scomposizioni per modello.

<h3 id="track-per-step-usage">
  Tracciare l'utilizzo per step
</h3>

Ogni messaggio dell'assistente contiene un `BetaMessage` annidato (accessibile tramite `message.message`) con un `id` e un oggetto `usage` con conteggi di token. Quando Claude utilizza strumenti in parallelo, più messaggi condividono lo stesso `id` con dati di utilizzo identici. Traccia quali ID hai già contato e salta i duplicati per evitare totali gonfiati.

<Warning>
  Le chiamate di strumenti paralleli producono più messaggi dell'assistente il cui `BetaMessage` annidato condivide lo stesso `id` e utilizzo identico. Deduplicare sempre per ID per ottenere conteggi di token per step accurati.
</Warning>

L'esempio seguente accumula token di input e output su tutti gli step, contando ogni ID di messaggio univoco una sola volta:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let totalOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant") {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
        totalOutputTokens += message.message.usage.output_tokens;
      }
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // totals below still reflect the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${totalOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Scomporre l'utilizzo per modello
</h3>

Il messaggio risultato include [`modelUsage`](/docs/it/agent-sdk/typescript#modelusage), una mappa del nome del modello ai conteggi di token e costi per modello. Questo è utile quando esegui più modelli (ad esempio, Haiku per subagenti e Opus per l'agente principale) e vuoi vedere dove vanno i token.

L'esempio seguente esegue una query e stampa il costo e la scomposizione dei token per ogni modello utilizzato:

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
  Accumulare costi su più chiamate
</h2>

Ogni chiamata `query()` restituisce il suo `total_cost_usd`. L'SDK non fornisce un totale a livello di sessione, quindi se la tua applicazione effettua più chiamate `query()` (ad esempio, in una sessione multi-turno o tra diversi utenti), accumula i totali tu stesso.

Gli esempi seguenti eseguono due chiamate `query()` in sequenza, aggiungono il `total_cost_usd` di ogni chiamata a un totale in esecuzione e stampano sia il costo per chiamata che quello combinato:

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

<h2 id="handle-errors-caching-and-token-discrepancies">
  Gestire errori, caching e discrepanze di token
</h2>

Per un tracciamento accurato dei costi, tieni conto di conversazioni non riuscite, prezzi dei token in cache e occasionali incoerenze di segnalazione.

<h3 id="resolve-output-token-discrepancies">
  Risolvere discrepanze di token di output
</h3>

In rari casi, potresti osservare valori `output_tokens` diversi per messaggi con lo stesso ID. Quando ciò accade:

1. **Utilizza il valore più alto:** il messaggio finale in un gruppo contiene in genere il totale accurato.
2. **Preferisci il messaggio risultato:** il `total_cost_usd` nel messaggio risultato riflette la stima accumulata dell'SDK su tutti gli step, quindi è più affidabile che sommare i valori per step tu stesso. È comunque una stima e può differire dalla tua fattura effettiva.
3. **Segnala incoerenze:** archivia i problemi nel [repository GitHub Claude Code](https://github.com/anthropics/claude-code/issues).

<h3 id="track-costs-on-failed-conversations">
  Tracciare i costi su conversazioni non riuscite
</h3>

Sia i messaggi risultato di successo che di errore includono `usage` e `total_cost_usd`. Se una conversazione fallisce a metà, hai comunque consumato token fino al punto di errore. Leggi sempre i dati di costo dal messaggio risultato indipendentemente dal suo `subtype`.

<h3 id="track-cache-tokens">
  Tracciare i token in cache
</h3>

Agent SDK utilizza automaticamente la [memorizzazione nella cache dei prompt](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) per ridurre i costi su contenuti ripetuti. Non è necessario configurare il caching tu stesso. L'oggetto di utilizzo include due campi aggiuntivi per il tracciamento della cache:

* `cache_creation_input_tokens`: token utilizzati per creare nuove voci di cache (addebitati a una tariffa più alta rispetto ai token di input standard).
* `cache_read_input_tokens`: token letti da voci di cache esistenti (addebitati a una tariffa ridotta).

Traccia questi separatamente da `input_tokens` per comprendere i risparmi di caching. In TypeScript, questi campi sono tipizzati sull'oggetto [`Usage`](/docs/it/agent-sdk/typescript#usage). In Python, appaiono come chiavi nel dizionario [`ResultMessage.usage`](/docs/it/agent-sdk/python#resultmessage) (ad esempio, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Estendere il TTL della cache dei prompt a un'ora
</h3>

Le voci di cache scritte dall'SDK utilizzano un TTL di 5 minuti per impostazione predefinita quando ti autentichi con una chiave API o esegui su Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Se il tuo carico di lavoro esegue molte sessioni brevi rispetto allo stesso prompt di sistema e contesto con gap più lunghi di 5 minuti tra loro, la cache scade tra le sessioni e ogni nuova sessione paga il prezzo di input completo.

Per richiedere un TTL di 1 ora sulle scritture della cache, imposta la variabile di ambiente [`ENABLE_PROMPT_CACHING_1H`](/docs/it/env-vars). Puoi esportarla nel tuo ambiente shell o container, oppure passarla tramite `options.env`.

L'esempio seguente abilita il TTL di 1 ora per un agente in esecuzione su Amazon Bedrock:

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

Le scritture della cache con un TTL di 1 ora sono fatturate a una tariffa più alta rispetto alle scritture di 5 minuti, quindi abilitare questa opzione scambia un costo di scrittura più elevato per più letture della cache. Consulta i [prezzi della memorizzazione nella cache dei prompt](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) per i dettagli. Gli utenti con abbonamento Claude ricevono già automaticamente il TTL di 1 ora e non hanno bisogno di impostare questa variabile.

<h2 id="related-documentation">
  Documentazione correlata
</h2>

* [Riferimento TypeScript SDK](/docs/it/agent-sdk/typescript) - Documentazione API completa
* [Panoramica SDK](/docs/it/agent-sdk/overview) - Introduzione all'SDK
* [Autorizzazioni SDK](/docs/it/agent-sdk/permissions) - Gestione delle autorizzazioni degli strumenti
