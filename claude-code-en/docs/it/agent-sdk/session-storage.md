> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Persistere le sessioni nell'archiviazione esterna

> Eseguire il mirroring dei trascritti di sessione su S3, Redis o il vostro backend in modo che qualsiasi host possa riprendere le sessioni.

Per impostazione predefinita, l'SDK scrive i trascritti di sessione in file JSONL in `~/.claude/projects/` nel filesystem locale. Un adattatore `SessionStore` consente di eseguire il mirroring di questi trascritti nel vostro backend, come S3, Redis o un database, in modo che una sessione creata su un host possa essere ripresa su un altro.

Motivi comuni per utilizzare un session store:

* **Distribuzioni multi-host.** Le funzioni serverless, i worker con scalabilità automatica e i runner CI non condividono un filesystem. Un store condiviso consente a qualsiasi replica di riprendere qualsiasi sessione.
* **Durabilità.** I container locali sono effimeri. Uno store supportato da S3 o da un database sopravvive ai riavvii e ai ridistribuzioni.
* **Conformità e audit.** Mantenete i trascritti nell'archiviazione che già controllate, con le vostre regole di conservazione, crittografia e controlli di accesso.

<h2 id="the-sessionstore-interface">
  L'interfaccia `SessionStore`
</h2>

Un `SessionStore` è un oggetto con due metodi obbligatori, `append` e `load`, e tre metodi facoltativi. L'SDK chiama `append` per scrivere le voci di trascritto durante una query e `load` per leggerle di nuovo per la ripresa.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry.

  type SessionKey = {
    projectKey: string;
    sessionId: string;
    subpath?: string;
  };

  type SessionStore = {
    // Required
    append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
    load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

    // Optional
    listSessions?(
      projectKey: string,
    ): Promise<Array<{ sessionId: string; mtime: number }>>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry.

  class SessionKey(TypedDict):
      project_key: str
      session_id: str
      subpath: NotRequired[str]

  class SessionStore(Protocol):
      # Required
      async def append(
          self, key: SessionKey, entries: list[SessionStoreEntry]
      ) -> None: ...
      async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

      # Optional — omit or raise NotImplementedError
      async def list_sessions(
          self, project_key: str
      ) -> list[SessionStoreListEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...
  ```
</CodeGroup>

`SessionKey` indirizza un trascritto. `projectKey` è una codifica stabile e sicura per il filesystem della directory di lavoro, `sessionId` è l'UUID della sessione, e `subpath` è impostato quando la voce appartiene a un trascritto di subagent o a un file sidecar piuttosto che alla conversazione principale. Trattate `subpath` come una chiave suffisso opaca; segue il layout su disco, ad esempio `subagents/agent-<id>`. Quando `subpath` non è definito, la chiave si riferisce al trascritto principale.

| Metodo         | Obbligatorio | Chiamato quando                                                                                                                                                                                                                                        |
| :------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`       | Sì           | Dopo che ogni batch di voci di trascritto viene scritto localmente. Le voci sono oggetti JSON-safe, uno per riga nel JSONL locale.                                                                                                                     |
| `load`         | Sì           | Una volta prima che il subprocess venga generato, quando `resume` è impostato. Restituire `null` se la sessione è sconosciuta.                                                                                                                         |
| `listSessions` | No           | Da `listSessions({ sessionStore })` e da `query()`/`startup()` con `continue: true`. Se non definito, queste chiamate generano un'eccezione.                                                                                                           |
| `delete`       | No           | Da `deleteSession({ sessionStore })`. L'eliminazione della chiave principale (nessun `subpath`) deve propagarsi a tutte le subchiavi per quella sessione. Se non definito, l'eliminazione è un'operazione nulla, che si adatta ai backend append-only. |
| `listSubkeys`  | No           | Durante la ripresa, per scoprire i trascritti dei subagent. Se non definito, viene ripristinato solo il trascritto principale.                                                                                                                         |

<h2 id="quick-start">
  Avvio rapido
</h2>

L'SDK fornisce un `InMemorySessionStore` per lo sviluppo e i test. L'esempio seguente esegue una query con lo store allegato, acquisisce l'ID della sessione dal messaggio di risultato, quindi riprende dallo store in una seconda chiamata `query()`. La seconda chiamata passa la stessa istanza dello store più `resume`, in modo che l'SDK carichi il trascritto dallo store invece dal filesystem locale:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  for await (const message of query({
    prompt: "List the TypeScript files under src/",
    options: { sessionStore: store },
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
    }
  }

  // Resume from the store. The agent has full context from the first call.
  for await (const message of query({
    prompt: "Summarize what those files do",
    options: { sessionStore: store, resume: sessionId },
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeAgentOptions,
      InMemorySessionStore,
      ResultMessage,
      query,
  )

  store = InMemorySessionStore()


  async def main():
      session_id = None
      async for message in query(
          prompt="List the Python files under src/",
          options=ClaudeAgentOptions(session_store=store),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id

      # Resume from the store. The agent has full context from the first call.
      async for message in query(
          prompt="Summarize what those files do",
          options=ClaudeAgentOptions(session_store=store, resume=session_id),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

La seconda query stampa un riepilogo dei file dalla prima query, il che mostra che l'agente ha ripreso con il contesto completo dallo store.

<h2 id="write-your-own-adapter">
  Scrivere il vostro adattatore
</h2>

Implementate `append` e `load` rispetto al vostro backend. Aggiungete `listSessions`, `delete` e `listSubkeys` se desiderate che `listSessions()`, `deleteSession()` e la ripresa dei subagent funzionino rispetto allo store.

Le voci passate a `append` sono digitate come `SessionStoreEntry` (un oggetto `{ type: string; ... }`). Trattate le come valori JSON-safe opachi: persistetele in ordine e restituitetele da `load` nello stesso ordine. `load` deve restituire voci che siano deep-equal a quelle aggiunte; la serializzazione byte-equal non è richiesta, quindi i backend come Postgres `jsonb` che riordinano le chiavi degli oggetti vanno bene.

<h2 id="reference-implementations">
  Implementazioni di riferimento
</h2>

Il repository TypeScript SDK include adattatori di riferimento eseguibili per S3, Redis e Postgres in [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores). Non sono pubblicati su npm; copiate il file `src/` di cui avete bisogno nel vostro progetto e installate il client backend corrispondente.

| Adattatore                                                                                                                     | Client backend       | Modello di archiviazione                                                             |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :----------------------------------------------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | Un file di parte JSONL per `append()`; `load()` elenca, ordina e concatena.          |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | Lista `RPUSH`/`LRANGE` per trascritto, più un indice di set ordinato della sessione. |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | Una riga per voce in una tabella `jsonb`, ordinata per `BIGSERIAL`.                  |

Ogni adattatore accetta un'istanza client preconfigurata, in modo che controlliate le credenziali, TLS, la regione e il pooling. Ad esempio, con S3:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

<h3 id="validate-your-adapter">
  Convalidare il vostro adattatore
</h3>

Entrambi gli SDK forniscono una suite di conformità che asserisce il contratto comportamentale che `append`, `load` e i metodi facoltativi devono soddisfare. I test per i metodi facoltativi vengono saltati automaticamente quando questi metodi non sono implementati.

In TypeScript, copiate [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) dalla directory di esempio nella vostra suite di test. In Python, la suite è fornita nel pacchetto:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  Note sul comportamento
</h2>

<h3 id="dual-write-architecture">
  Architettura a doppia scrittura
</h3>

Lo store è un mirror, non una sostituzione. Il subprocess Claude Code scrive sempre su disco locale per primo; l'SDK quindi invia ogni batch a `append()`. Se desiderate che la copia locale sia effimera, puntate `CLAUDE_CONFIG_DIR` a una directory temporanea in `options.env`. Poiché il mirror dipende dalle scritture locali, `sessionStore` non può essere combinato con `persistSession: false`; l'SDK genera un'eccezione se impostate entrambi. Genera anche un'eccezione se combinato con `enableFileCheckpointing`, poiché i blob di backup della cronologia dei file vengono scritti direttamente su disco locale e non vengono sottoposti a mirroring nello store.

<h3 id="mirror-writes-are-best-effort">
  Le scritture mirror sono best-effort
</h3>

Se `append()` rifiuta, l'SDK ritenta il batch fino a due volte in più con un breve backoff, per un massimo di tre tentativi in totale. Una chiamata che scade non viene ritentata, poiché la chiamata originale potrebbe comunque arrivare. Se il batch continua a fallire, l'errore viene registrato, un messaggio `{ type: "system", subtype: "mirror_error" }` viene emesso nell'iteratore, il batch viene scartato e la query continua. Il trascritto locale è già durevole su disco, quindi un'interruzione dello store non interrompe l'agente o perde dati localmente. Monitorate `mirror_error` se dovete rilevare la perdita di dati dello store. Poiché un batch ritentato può ri-consegnare voci che hanno già raggiunto la destinazione, deduplicare per `entry.uuid` nella vostra implementazione di `append()`.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` restituisce la catena post-compattazione
</h3>

`getSessionMessages({ sessionStore })` restituisce la catena di messaggi collegati che l'agente vedrebbe al momento della ripresa. Dopo la compattazione automatica, i turni precedenti vengono sostituiti da un riepilogo, quindi una sessione il cui store contiene 503 voci grezze può restituire 18 messaggi da `getSessionMessages`. Per la cronologia grezza completa, inclusi i turni pre-compattazione e le voci di metadati, chiamate `store.load(key)` direttamente.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` non è una copia byte
</h3>

`forkSession({ sessionStore })` legge le voci di origine, riscrive ogni campo `sessionId` e rimappa gli UUID dei messaggi, quindi aggiunge le voci trasformate sotto una nuova chiave. Una copia a livello di adattatore o un collegamento `CopyObject` produrrebbe un trascritto che fa ancora riferimento all'ID della sessione precedente, quindi l'SDK non ne utilizza uno.

<h3 id="subagent-transcripts">
  Trascritti dei subagent
</h3>

I trascritti dei subagent vengono sottoposti a mirroring in `subpath: "subagents/agent-<id>"`. `listSubagents({ sessionStore })` richiede che l'adattatore implementi `listSubkeys`; `getSubagentMessages({ sessionStore })` lo utilizza quando disponibile ma ricade al subpath diretto quando non è definito. La ripresa chiama anche `listSubkeys` per ripristinare i file dei subagent; senza di esso, viene materializzato solo il trascritto principale.

<h3 id="retention">
  Conservazione
</h3>

L'SDK non elimina mai dal vostro store di sua iniziativa. La conservazione è responsabilità dell'adattatore: implementate TTL, politiche del ciclo di vita S3 o pulizia pianificata secondo i vostri requisiti di conformità. I trascritti locali in `CLAUDE_CONFIG_DIR` vengono puliti indipendentemente dall'impostazione `cleanupPeriodDays`.

<h2 id="supported-on">
  Supportato su
</h2>

Le seguenti funzioni SDK accettano un'opzione `sessionStore` e operano rispetto allo store invece del filesystem locale quando viene fornita:

* [`query()`](/docs/it/agent-sdk/typescript#query)
* [`startup()`](/docs/it/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/it/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/it/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/it/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/it/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/it/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/it/agent-sdk/typescript)
* [`forkSession()`](/docs/it/agent-sdk/typescript)
* [`listSubagents()`](/docs/it/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/it/agent-sdk/typescript)

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Lavorare con le sessioni](/docs/it/agent-sdk/sessions): Continuare, riprendere e fare un fork senza uno store personalizzato
* [Ospitare l'SDK](/docs/it/agent-sdk/hosting): Modelli di distribuzione per ambienti multi-host
* [TypeScript `Options`](/docs/it/agent-sdk/typescript#options): Riferimento completo delle opzioni
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores): Adattatori di riferimento S3, Redis e Postgres eseguibili
