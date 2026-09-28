> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Persistere le sessioni nell'archiviazione esterna

> Eseguire il mirroring dei trascritti di sessione Agent SDK nel vostro object store, key-value store o database in modo che altri host possano riprendere le vostre sessioni.

Per impostazione predefinita, l'SDK scrive i trascritti di sessione in file JSONL in `~/.claude/projects/` nel filesystem locale. Un adattatore `SessionStore` consente di eseguire il mirroring di questi trascritti nel vostro backend, come un object store, un key-value store o un database, in modo che una sessione creata su un host possa essere ripresa su un altro host in esecuzione da una directory di lavoro corrispondente.

Motivi comuni per utilizzare un session store:

* **Distribuzioni multi-host.** Le funzioni serverless, i worker con scalabilità automatica e i runner CI non condividono un filesystem. Un store condiviso consente alle repliche di riprendere le sessioni reciproche.
* **Durabilità.** I container locali sono effimeri. Uno store esterno sopravvive ai riavvii e ai ridistribuzioni.
* **Conformità e audit.** Mantenete i trascritti nell'archiviazione che già controllate, con le vostre regole di conservazione, crittografia e controlli di accesso.

<h2 id="the-sessionstore-interface">
  L'interfaccia `SessionStore`
</h2>

Un `SessionStore` è un oggetto con due metodi obbligatori, `append` e `load`, e quattro metodi facoltativi. L'SDK chiama `append` per scrivere le voci di trascritto durante una query e `load` per leggerle di nuovo per la ripresa.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

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
    listSessionSummaries?(projectKey: string): Promise<SessionSummaryEntry[]>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };

  type SessionSummaryEntry = {
    sessionId: string;
    mtime: number;
    data: Record<string, unknown>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

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
      async def list_session_summaries(
          self, project_key: str
      ) -> list[SessionSummaryEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...

  class SessionSummaryEntry(TypedDict):
      session_id: str
      mtime: int
      data: dict[str, Any]
  ```
</CodeGroup>

`SessionKey` indirizza un trascritto. `projectKey` è una codifica stabile e sicura per il filesystem della directory di lavoro, `sessionId` è l'UUID della sessione, e `subpath` è impostato quando la voce appartiene a un trascritto di subagent o a un file sidecar piuttosto che alla conversazione principale.

Poiché `projectKey` codifica la directory di lavoro, riprendere o continuare dal negozio da una directory di lavoro che corrisponda all'esecuzione originale. In TypeScript, se impostate [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/it/sessions#name-the-project-directory-yourself) accanto a `CLAUDE_CONFIG_DIR` nell'opzione [`env`](/docs/it/agent-sdk/typescript#options) di una query, l'SDK codifica le voci di quella query e le sue ricerche `resume` e `continue` per quel nome. Poiché helper autonomi come `listSessions` e `deleteSession` non accettano `env` e leggono l'ambiente del processo, impostate `CLAUDE_CONFIG_DIR` e lo stesso nome nell'ambiente del processo host. Richiede Agent SDK v0.3.234 o successivo.

Trattate `subpath` come una chiave suffisso opaca; segue il layout su disco, ad esempio `subagents/agent-<id>`. Quando `subpath` non è definito, la chiave si riferisce al trascritto principale.

| Metodo                 | Obbligatorio | Chiamato quando                                                                                                                                                                                                                                                                                                                                                                               |
| :--------------------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | Sì           | Dopo che ogni batch di voci di trascritto viene scritto localmente. Le voci sono oggetti JSON-safe, uno per riga nel JSONL locale.                                                                                                                                                                                                                                                            |
| `load`                 | Sì           | Prima che il subprocess venga generato quando `resume` è impostato o `continue: true` risolve la sessione del negozio più recente, e una volta per sessione quando l'elenco ricade da `listSessionSummaries`. Restituire `null` se la sessione è sconosciuta.                                                                                                                                 |
| `listSessions`         | No           | Da `listSessions({ sessionStore })` e da `query()`/`startup()` con `continue: true`. Se non definito, `continue: true` genera un'eccezione, e `listSessions({ sessionStore })` genera un'eccezione a meno che `listSessionSummaries` non sia implementato.                                                                                                                                    |
| `listSessionSummaries` | No           | Da `listSessions({ sessionStore })` per leggere i metadati per tutte le sessioni in una sola chiamata. Mantenete i riepiloghi all'interno di `append`. Se non definito, l'elenco ricade a `listSessions` più un `load` per sessione.                                                                                                                                                          |
| `delete`               | No           | Da `deleteSession({ sessionStore })`. L'eliminazione della chiave principale (nessun `subpath`) deve propagarsi a tutte le subchiavi per quella sessione e anche rimuovere la voce di riepilogo della sessione, in modo che una sessione eliminata smetta di apparire in `listSessionSummaries`. Se non definito, l'eliminazione è un'operazione nulla, che si adatta ai backend append-only. |
| `listSubkeys`          | No           | Durante la ripresa, per scoprire i trascritti dei subagent. Se non definito, viene ripristinato solo il trascritto principale.                                                                                                                                                                                                                                                                |

In una `SessionSummaryEntry`, `mtime` è il tempo di scrittura dell'archiviazione del sidecar e deve condividere una fonte di clock con i valori `mtime` che `listSessions` restituisce. `data` è uno stato di proprietà dell'SDK opaco; persistetelo verbatim senza interpretarlo.

Costruite le voci chiamando l'helper esportato `foldSessionSummary`, `fold_session_summary` in Python, su ogni batch all'interno di `append`. Saltate i batch la cui chiave ha un `subpath`; i trascritti dei subagent non devono contribuire al riepilogo della sessione principale. Il fold non imposta mai `mtime`: contrassegnatelo al momento della persistenza, tramite l'argomento `options.mtime` in TypeScript o sovrascrivendo il campo sulla voce restituita in Python. Le chiamate `append` simultanee per la stessa sessione possono correre sul sidecar, quindi serializzate la lettura-fold-scrittura con una transazione, un compare-and-swap, o un blocco per sessione; il fold stesso è puro.

Per ciò che l'SDK fa con il trascritto che `load` restituisce, vedere [Riprendere dal negozio](#resume-from-the-store).

<h2 id="quick-start">
  Avvio rapido
</h2>

L'SDK fornisce un `InMemorySessionStore` per lo sviluppo e i test. L'esempio seguente esegue una query con lo store allegato, acquisisce l'ID della sessione dal messaggio di risultato, quindi riprende dallo store in una seconda chiamata `query()`. La seconda chiamata passa la stessa istanza dello store più `resume`, in modo che l'SDK carichi il trascritto dallo store invece dal filesystem locale:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  try {
    for await (const message of query({
      prompt: "List the TypeScript files under src/",
      options: { sessionStore: store },
    })) {
      if (message.type === "result") {
        sessionId = message.session_id;
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, sessionId was already captured by the loop
    // above; connection or process failures yield no result message.
    console.error(`Session ended with an error: ${error}`);
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
      try:
          async for message in query(
              prompt="List the Python files under src/",
              options=ClaudeAgentOptions(session_store=store),
          ):
              if isinstance(message, ResultMessage):
                  session_id = message.session_id
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, session_id was already captured by the
          # loop above; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")

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

Implementate `append` e `load` rispetto al vostro backend. Aggiungete `listSessions`, `listSessionSummaries`, `delete` e `listSubkeys` se desiderate che `listSessions()`, letture di metadati in una sola chiamata, `deleteSession()` e la ripresa dei subagent funzionino rispetto allo store.

Le voci passate a `append` sono digitate come `SessionStoreEntry` (un oggetto `{ type: string; ... }`). Trattate le come valori JSON-safe opachi: persistetele in ordine e restituitetele da `load` nello stesso ordine. `load` deve restituire voci che siano deep-equal a quelle aggiunte; la serializzazione byte-equal non è richiesta, quindi un backend che riordina le chiavi degli oggetti, come un tipo di colonna JSON binaria, va bene.

<h2 id="reference-implementations">
  Implementazioni di riferimento
</h2>

Entrambi i repository SDK includono adattatori di riferimento eseguibili in [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) in TypeScript e [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) in Python. C'è un adattatore per tipo di archiviazione, e ognuno mostra come `append` e `load` si mappano su quel tipo di backend. Non sono pubblicati come pacchetti; copiate l'adattatore per il tipo più vicino al vostro backend nel vostro progetto, installate il client del vostro backend, e adattatelo.

| Tipo di archiviazione                 | Modello di archiviazione                                                                                              | Adattatore di esempio                                                                                                                                                                                                                                      |
| :------------------------------------ | :-------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Object store                          | Un file di parte per `append()`; `load()` elenca le parti, le ordina e le concatena.                                  | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py))                   |
| Key-value store                       | Una lista per trascritto che `append()` inserisce e `load()` legge in intervallo, più un indice ordinato di sessioni. | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py))          |
| Database relazionale o document store | Una riga o documento per voce, archiviato come JSON e ordinato per una chiave assegnata all'inserimento.              | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

Ogni adattatore accetta un'istanza client preconfigurata, in modo che controlliate le credenziali, TLS, la regione e il pooling. L'esempio seguente collega l'adattatore object-store in `query()` e quindi riprende da esso su un altro host:

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

In TypeScript, copiate [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) dalla directory di esempio nella vostra suite di test. In Python, la suite è fornita nel pacchetto. Per eseguirla con pytest, che non è una dipendenza dell'SDK, installate prima pytest:

```bash theme={null}
pip install pytest
```

Quindi passate il vostro adattatore alla suite in un file di test come factory a zero argomenti, che `run_session_store_conformance` chiama una volta per contratto per costruire un nuovo store:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

Passare la classe `MyRedisStore` stessa, come fa questo esempio, funziona quando il costruttore non accetta argomenti. Per un adattatore che accetta un client preconfigurato, passate invece una lambda che costruisce lo store. Poiché i contratti riutilizzano le stesse chiavi di sessione, ogni store che la factory restituisce deve iniziare con archiviazione vuota, quindi fate in modo che la lambda fornisca archiviazione di supporto isolata per ogni chiamata, come un nuovo fake in memoria, un prefisso di chiave univoco o un nuovo database di test.

<h2 id="behavior-notes">
  Note sul comportamento
</h2>

<h3 id="dual-write-architecture">
  Architettura a doppia scrittura
</h3>

Il subprocess Claude Code scrive sempre ogni batch di voci di trascritto su disco locale per primo, e l'SDK quindi invia lo stesso batch a `append()` del vostro store, quindi lo store è un mirror del trascritto locale piuttosto che una sostituzione per esso. Quale copia sopravvive all'esecuzione dipende da come è stata avviata l'esecuzione:

* **Sessione nuova, o una ripresa quando lo store non ha nulla per la sessione**: il trascritto locale nella vostra directory di configurazione sopravvive all'esecuzione, e lo store riceve una copia.
* **Esecuzione [ripresa dallo store](#resume-from-the-store)**: la copia locale viene eliminata alla fine dell'esecuzione, quindi lo store contiene l'unica copia durevole.

Se non desiderate che una sessione nuova lasci un trascritto su disco locale, impostate `CLAUDE_CONFIG_DIR` a una directory temporanea in `options.env`. Un'esecuzione ripresa dallo store elimina già la sua copia locale, quindi non ha bisogno di tale impostazione. In TypeScript, diffondete anche `process.env` in `env`, poiché l'[opzione `env`](/docs/it/agent-sdk/typescript#options) sostituisce l'ambiente del subprocess.

Se la vostra app accede tramite file nella directory di configurazione, come credenziali OAuth o un `apiKeyHelper` nel vostro `settings.json` utente, copiate prima quei file nella directory temporanea, oppure impostate `ANTHROPIC_API_KEY` in `env` invece. Altrimenti l'esecuzione fallisce con `Not logged in`.

Due opzioni entrano in conflitto con il mirror, e l'SDK genera un'eccezione all'avvio se combinate una di esse con uno store:

* **`persistSession: false`** in TypeScript: disattiva le scritture locali su cui è costruito il mirror. L'SDK Python non ha un'opzione equivalente.
* **File checkpointing**, `enableFileCheckpointing` in TypeScript o `enable_file_checkpointing` in Python: scrive i suoi backup di file direttamente su disco locale, e l'SDK non li esegue il mirroring nello store.

<h3 id="resume-from-the-store">
  Ripresa dallo store
</h3>

Quando passate `resume`, o `continue: true` in TypeScript o `continue_conversation=True` in Python, insieme a uno store, l'SDK chiede allo store un trascritto prima di generare il subprocess:

* **`resume`**: l'SDK chiede la sessione il cui ID avete passato.
* **`continue: true`** o **`continue_conversation=True`**: l'SDK chiede la sessione più recente dello store.

Quando lo store restituisce il trascritto, l'SDK lo scrive in una directory di configurazione temporanea, esegue il subprocess con `CLAUDE_CONFIG_DIR` che punta lì, ed elimina la directory quando l'esecuzione termina. Il trascritto locale che quell'esecuzione scrive viene eliminato con essa, motivo per cui lo store contiene l'unica copia durevole su questo percorso.

L'SDK semina anche la directory temporanea con file dalla vostra directory di configurazione reale. Ciò che copia differisce per linguaggio:

* **TypeScript**: credenziali, `.claude.json`, e il vostro `settings.json` utente. Da `settings.json` rimuove le chiavi che si comportano male in una directory di configurazione temporanea: `enabledPlugins`, `extraKnownMarketplaces`, il suo alias [`additionalMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces), e qualsiasi `CLAUDE_CONFIG_DIR` nel blocco `env` del file. Prima di Agent SDK v0.3.232, l'SDK non rimuoveva l'alias. L'autenticazione configurata nelle impostazioni, come [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper), funziona quando riprendete dallo store. Prima di Agent SDK v0.3.222, l'SDK TypeScript copiava solo credenziali e `.claude.json`.
* **Python**: solo credenziali e `.claude.json`, quindi un'app che si autentica tramite `apiKeyHelper` nel vostro `settings.json` utente fallisce con `Not logged in` quando riprende da uno store. Un `apiKeyHelper` nelle impostazioni gestite o di progetto funziona ancora, perché Claude Code legge quei file da posizioni che `CLAUDE_CONFIG_DIR` non influenza.

Quando lo store non ha nulla per la sessione, l'SDK viene eseguito nella vostra directory di configurazione reale, e il risultato dipende da quale opzione avete passato:

* **`resume`**: entrambi gli SDK passano l'ID al subprocess, che riprende il trascritto locale esattamente come `resume` fa senza uno store.
* **`continue: true`** in TypeScript: l'SDK avvia una sessione nuova.
* **`continue_conversation=True`** in Python: l'SDK continua dalla sessione locale più recente.

<h3 id="mirror-writes-are-best-effort">
  Le scritture mirror sono best-effort
</h3>

Se `append()` rifiuta, l'SDK ritenta il batch fino a due volte in più con un breve backoff, per un massimo di tre tentativi in totale. Una chiamata che scade non viene ritentata, poiché la chiamata originale potrebbe comunque arrivare. Se il batch continua a fallire, l'SDK registra l'errore, emette un messaggio `{ type: "system", subtype: "mirror_error" }` nell'iteratore, scarta il batch e continua la query. Poiché un batch ritentato può ri-consegnare voci che hanno già raggiunto la destinazione, deduplicare per `entry.uuid` nella vostra implementazione di `append()`.

Un'interruzione dello store non interrompe l'agente, poiché il subprocess scrive localmente per primo. Monitorate `mirror_error` se dovete rilevare la perdita di dati dello store. Su un'esecuzione [ripresa dallo store](#resume-from-the-store), un batch scartato non ha una copia sopravvivente una volta che l'esecuzione termina.

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

L'SDK non elimina mai dal vostro store di sua iniziativa. La conservazione è responsabilità dell'adattatore: implementate TTL, politiche del ciclo di vita S3 o pulizia pianificata secondo i vostri requisiti di conformità.

I trascritti locali in `CLAUDE_CONFIG_DIR` vengono puliti indipendentemente dall'impostazione `cleanupPeriodDays`, seguendo le [regole di pulizia della conservazione](/docs/it/claude-directory#cleaned-up-automatically). Un'esecuzione [ripresa dallo store](#resume-from-the-store) non lascia alcun trascritto locale, quindi per quelle esecuzioni la conservazione del vostro store è l'unica conservazione che esiste.

<h2 id="supported-on">
  Supportato su
</h2>

Le seguenti funzioni SDK TypeScript accettano un'opzione `sessionStore` e operano rispetto allo store invece del filesystem locale quando viene fornita:

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

In Python SDK, impostare `session_store` in [`ClaudeAgentOptions`](/docs/it/agent-sdk/python#claudeagentoptions) per eseguire `query()` rispetto a uno store. Le operazioni rimanenti hanno ciascuna una funzione Python supportata da store che accetta lo store come argomento: `list_sessions_from_store()`, `get_session_info_from_store()`, `get_session_messages_from_store()`, `list_subagents_from_store()`, `get_subagent_messages_from_store()`, `rename_session_via_store()`, `tag_session_via_store()`, `delete_session_via_store()` e `fork_session_via_store()`. `startup()` non ha un equivalente Python. Le funzioni standalone documentate nel [riferimento Python SDK](/docs/it/agent-sdk/python#functions), come `list_sessions()`, leggono i file di sessione locali.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Lavorare con le sessioni](/docs/it/agent-sdk/sessions): Continuare, riprendere e fare un fork senza uno store personalizzato
* [Ospitare l'SDK](/docs/it/agent-sdk/hosting): Modelli di distribuzione per ambienti multi-host
* [TypeScript `Options`](/docs/it/agent-sdk/typescript#options): Riferimento completo delle opzioni
* [Implementazioni di riferimento](#reference-implementations): Adattatori di esempio eseguibili per uno store di oggetti, uno store chiave-valore e un database, in entrambi i repository SDK
