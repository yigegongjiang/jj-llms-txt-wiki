> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# API sessione TypeScript SDK V2 (rimosso)

> Riferimento per l'API sessione rimosso V2 TypeScript Agent SDK, con pattern send/stream basati su sessione per conversazioni multi-turno.

<Warning>
  L'API sessione V2 non è più supportata. TypeScript Agent SDK 0.3.142 rimuove `unstable_v2_createSession`, `unstable_v2_resumeSession`, `unstable_v2_prompt` e i tipi `SDKSession` e `SDKSessionOptions`.

  Per eseguire la migrazione, utilizzare l'[API `query()`](/docs/it/agent-sdk/typescript) e le [opzioni sessione](/docs/it/agent-sdk/sessions) che accetta. Passare un `AsyncIterable<SDKUserMessage>` per conversazioni multi-turno, oppure `options.resume` per continuare una sessione salvata. Questa pagina viene mantenuta come riferimento se si mantiene il codice su Agent SDK 0.2.x o versioni precedenti.
</Warning>

V2 era un'API sessione sperimentale che eliminava la necessità di generatori asincroni e coordinamento yield. Invece di gestire lo stato del generatore tra i turni, ogni turno era un ciclo `send()`/`stream()` separato. La superficie API si riduceva a tre concetti:

* `createSession()` / `resumeSession()`: Avviare o continuare una conversazione
* `session.send()`: Inviare un messaggio
* `session.stream()`: Ottenere la risposta

<h2 id="installation">
  Installazione
</h2>

Agent SDK 0.2.x è l'ultima versione che include l'interfaccia V2. La versione del pacchetto è passata da 0.2.x direttamente a 0.3.142, quindi la versione di rimozione sopra e il pin di installazione sottostante descrivono lo stesso limite. Per installare l'ultima versione compatibile con V2, fissare la versione principale e secondaria:

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

<Note>
  L'SDK raggruppa un binario Claude Code nativo per la vostra piattaforma come dipendenza opzionale, quindi non è necessario installare Claude Code separatamente.
</Note>

<h2 id="quick-start">
  Avvio rapido
</h2>

<h3 id="one-shot-prompt">
  Prompt singolo
</h3>

Per semplici query a turno singolo dove non è necessario mantenere una sessione, utilizzare `unstable_v2_prompt()`. Questo esempio invia una domanda matematica e registra la risposta:

```typescript theme={null}
import { unstable_v2_prompt } from "@anthropic-ai/claude-agent-sdk";

const result = await unstable_v2_prompt("What is 2 + 2?", {
  model: "claude-opus-4-7"
});
if (result.subtype === "success") {
  console.log(result.result);
}
```

<details>
  <summary>Vedere la stessa operazione in V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "What is 2 + 2?",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "result" && msg.subtype === "success") {
      console.log(msg.result);
    }
  }
  ```
</details>

<h3 id="basic-session">
  Sessione di base
</h3>

Per interazioni oltre un singolo prompt, creare una sessione. V2 separa l'invio e lo streaming in passaggi distinti:

* `send()` invia il vostro messaggio
* `stream()` trasmette la risposta

Questa separazione esplicita rende più facile aggiungere logica tra i turni (come elaborare le risposte prima di inviare i follow-up).

L'esempio seguente crea una sessione, invia "Hello!" a Claude e stampa la risposta di testo. Utilizza [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) (TypeScript 5.2+) per chiudere automaticamente la sessione quando il blocco esce. Potete anche chiamare `session.close()` manualmente.

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Hello!");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>Vedere la stessa operazione in V1</summary>

  In V1, sia l'input che l'output fluiscono attraverso un singolo generatore asincrono. Per un prompt di base questo appare simile, ma aggiungere logica multi-turno richiede di ristrutturare per utilizzare un generatore di input.

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "Hello!",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="multi-turn-conversation">
  Conversazione multi-turno
</h3>

Le sessioni mantengono il contesto attraverso più scambi. Per continuare una conversazione, chiamare `send()` di nuovo sulla stessa sessione. Claude ricorda i turni precedenti.

Questo esempio pone una domanda matematica, quindi pone un follow-up che fa riferimento alla risposta precedente:

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

// Turn 1
await session.send("What is 5 + 3?");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}

// Turn 2
await session.send("Multiply that by 2");
for await (const msg of session.stream()) {
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>Vedere la stessa operazione in V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Must create an async iterable to feed messages
  async function* createInputStream() {
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "What is 5 + 3?" }] },
      parent_tool_use_id: null
    };
    // Must coordinate when to yield next message
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "Multiply by 2" }] },
      parent_tool_use_id: null
    };
  }

  const q = query({
    prompt: createInputStream(),
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="session-resume">
  Ripresa della sessione
</h3>

Se avete un ID di sessione da un'interazione precedente, potete riprenderla in seguito. Questo è utile per flussi di lavoro di lunga durata o quando è necessario persistere le conversazioni tra i riavvii dell'applicazione.

Questo esempio crea una sessione, memorizza il suo ID, la chiude, quindi riprende la conversazione:

```typescript theme={null}
import {
  unstable_v2_createSession,
  unstable_v2_resumeSession,
  type SDKMessage
} from "@anthropic-ai/claude-agent-sdk";

// Helper to extract text from assistant messages
function getAssistantText(msg: SDKMessage): string | null {
  if (msg.type !== "assistant") return null;
  return msg.message.content
    .filter((block) => block.type === "text")
    .map((block) => block.text)
    .join("");
}

// Create initial session and have a conversation
const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Remember this number: 42");

// Get the session ID from any received message
let sessionId: string | undefined;
for await (const msg of session.stream()) {
  sessionId = msg.session_id;
  const text = getAssistantText(msg);
  if (text) console.log("Initial response:", text);
}

console.log("Session ID:", sessionId);
session.close();

// Later: resume the session using the stored ID
await using resumedSession = unstable_v2_resumeSession(sessionId!, {
  model: "claude-opus-4-7"
});

await resumedSession.send("What number did I ask you to remember?");
for await (const msg of resumedSession.stream()) {
  const text = getAssistantText(msg);
  if (text) console.log("Resumed response:", text);
}
```

<details>
  <summary>Vedere la stessa operazione in V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Create initial session
  const initialQuery = query({
    prompt: "Remember this number: 42",
    options: { model: "claude-opus-4-7" }
  });

  // Get session ID from any message
  let sessionId: string | undefined;
  for await (const msg of initialQuery) {
    sessionId = msg.session_id;
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Initial response:", text);
    }
  }

  console.log("Session ID:", sessionId);

  // Later: resume the session
  const resumedQuery = query({
    prompt: "What number did I ask you to remember?",
    options: {
      model: "claude-opus-4-7",
      resume: sessionId
    }
  });

  for await (const msg of resumedQuery) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Resumed response:", text);
    }
  }
  ```
</details>

<h3 id="cleanup">
  Pulizia
</h3>

Le sessioni possono essere chiuse manualmente o automaticamente utilizzando [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management), una funzionalità di TypeScript 5.2+ per la pulizia automatica delle risorse. Se state utilizzando una versione precedente di TypeScript o riscontrate problemi di compatibilità, utilizzate invece la pulizia manuale.

**Pulizia automatica (TypeScript 5.2+):**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**Pulizia manuale:**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

<h2 id="api-reference">
  Riferimento API
</h2>

<h3 id="unstable_v2_createsession">
  `unstable_v2_createSession()`
</h3>

Crea una nuova sessione per conversazioni multi-turno.

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

<h3 id="unstable_v2_resumesession">
  `unstable_v2_resumeSession()`
</h3>

Riprende una sessione esistente per ID.

```typescript theme={null}
function unstable_v2_resumeSession(
  sessionId: string,
  options: {
    model: string;
    // Additional options supported
  }
): SDKSession;
```

<h3 id="unstable_v2_prompt">
  `unstable_v2_prompt()`
</h3>

Funzione di convenienza one-shot per query a turno singolo.

```typescript theme={null}
function unstable_v2_prompt(
  prompt: string,
  options: {
    model: string;
    // Additional options supported
  }
): Promise<SDKResultMessage>;
```

<h3 id="sdksession-interface">
  Interfaccia SDKSession
</h3>

```typescript theme={null}
interface SDKSession {
  readonly sessionId: string;
  send(message: string | SDKUserMessage): Promise<void>;
  stream(): AsyncGenerator<SDKMessage, void>;
  close(): void;
}
```

<h2 id="feature-availability">
  Disponibilità delle funzionalità
</h2>

L'API sessione V2 non supporta tutte le funzionalità V1. Le seguenti richiedono l'[SDK V1](/docs/it/agent-sdk/typescript):

* Forking della sessione (opzione `forkSession`)
* Alcuni pattern di input streaming avanzati

<h2 id="see-also">
  Vedere anche
</h2>

* [Riferimento SDK TypeScript (V1)](/docs/it/agent-sdk/typescript) - Documentazione completa dell'SDK V1
* [Panoramica SDK](/docs/it/agent-sdk/overview) - Concetti generali dell'SDK
* [Esempi V2 su GitHub](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - Esempi di codice funzionanti
