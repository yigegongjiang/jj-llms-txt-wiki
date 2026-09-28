> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hosting dell'Agent SDK

> Distribuisci l'Agent SDK in produzione: architettura subprocess, persistenza della sessione, scalabilità, osservabilità e isolamento multi-tenant per Docker, Kubernetes e provider sandbox.

L'Agent SDK genera e supervisiona un subprocess `claude` CLI che possiede una shell, una directory di lavoro e file di sessione su disco. Ospitarlo non è come ospitare un wrapper API senza stato. Ogni agente in esecuzione è un processo di lunga durata legato allo stato locale, il che influenza il modo in cui allochi risorse, persisti sessioni e scala tra tenant.

Questa pagina copre l'auto-hosting sulla tua infrastruttura. Per Dockerfile distribuibili e manifesti Kubernetes, consulta il [hosting cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Se non hai bisogno di eseguire il ciclo dell'agente sulla tua infrastruttura, considera invece gli [Managed Agents](https://platform.claude.com/docs/en/managed-agents/overview). Anthropic ospita il ciclo dell'agente e la tua applicazione invia eventi e riceve risultati in streaming attraverso gli SDK client o l'API REST. L'esecuzione degli strumenti viene eseguita in una sandbox cloud gestita da Anthropic o in una [sandbox auto-ospitata](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) sulla tua infrastruttura.

<h2 id="the-subprocess-model">
  Il modello subprocess
</h2>

Ogni decisione di hosting su questa pagina deriva da come l'SDK esegue l'agente. Quando il vostro codice chiama `query()`, l'SDK genera un processo `claude` CLI separato e comunica con esso tramite stdio. Quel subprocess possiede la shell, la directory di lavoro e i transcript della sessione JSONL su disco locale.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" className="dark:hidden" alt="Flusso di richiesta: dal client alla vostra app, che genera un subprocess CLI claude su stdio all'interno del container; il subprocess scrive su disco locale e chiama api.anthropic.com su HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/hosting-subprocess-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=3fdeff3d7f44b2b67762668acfbb25f5" className="hidden dark:block" alt="Flusso di richiesta: dal client alla vostra app, che genera un subprocess CLI claude su stdio all'interno del container; il subprocess scrive su disco locale e chiama api.anthropic.com su HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess-dark.svg" />

Una sessione agente corrisponde a un subprocess. L'esecuzione di N sessioni concorrenti significa N subprocess, ognuno con il proprio albero di processi e file di transcript. Per impostazione predefinita, ereditano tutti la directory di lavoro della vostra applicazione. Quando le sessioni necessitano di filesystem separati, passate un `cwd` distinto nelle opzioni della chiamata `query()` di ogni sessione:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the files in this directory",
    options: { cwd: "/work/session-a" },
  })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt="Summarize the files in this directory",
          options=ClaudeAgentOptions(cwd="/work/session-a"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

Gli esempi TypeScript su questa pagina utilizzano `await` di livello superiore, quindi salvateli come file `.mts` o impostate `"type": "module"` in `package.json`.

<h3 id="state-that-lives-on-local-disk">
  Stato che risiede su disco locale
</h3>

Tre tipi di stato agente risiedono nel filesystem del container per impostazione predefinita. Nessuno di essi sopravvive a un riavvio del container, a una riduzione della scala o a uno spostamento su un nodo diverso.

| Stato                               | Posizione predefinita                                                                                       |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Transcript della sessione           | `~/.claude/projects/`, o la directory `projects/` sotto `CLAUDE_CONFIG_DIR` se impostata                    |
| File di memoria `CLAUDE.md`         | `~/.claude/CLAUDE.md` per il livello utente e la directory di lavoro della sessione per il livello progetto |
| Artefatti della directory di lavoro | La directory di lavoro della sessione                                                                       |

Per persistere i transcript tra host, configurate un adattatore [`SessionStore`](/docs/it/agent-sdk/session-storage). I file di memoria e altri artefatti della directory di lavoro necessitano della loro propria strategia di archiviazione, come un volume montato o una sincronizzazione di object-store.

Per informazioni su come sessioni, ripresa e fork funzionano a livello API, consultate [Sessions](/docs/it/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Scegliere un modello di sessione
</h2>

Questi quattro modelli coprono il ciclo di vita della sessione: quanto tempo vive un container rispetto alle sessioni che serve. Per dove il container viene eseguito, il [manuale di hosting](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) contiene [codice distribuibile](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) per Docker locale, Modal e Kubernetes. Scegliere un modello di sessione qui e una destinazione di distribuzione dal manuale.

<h3 id="ephemeral-sessions">
  Sessioni effimere
</h3>

Creare un container per ogni attività dell'utente e distruggerlo quando l'attività si completa. Ideale per attività una tantum. L'utente può ancora interagire con l'IA mentre l'attività si sta completando, ma una volta completata il container viene distrutto.

I carichi di lavoro di esempio includono l'indagine e la correzione di bug, l'estrazione di fatture e ricevute, la traduzione di documenti e la trasformazione di media.

Il container esegue un punto di ingresso una tantum che legge l'attività dalla variabile di ambiente `TASK_PROMPT`, chiama l'SDK ed esce.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompt = process.env.TASK_PROMPT!;
  for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt=os.environ["TASK_PROMPT"],
          options=ClaudeAgentOptions(max_turns=20),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

Lo script stampa ogni messaggio man mano che arriva, incluso un messaggio di risultato il cui `subtype` è `success` quando l'attività si completa entro il limite di turni. Se l'attività raggiunge il limite di 20 turni, il `subtype` del messaggio di risultato è `error_max_turns` e la chiamata `query()` genera un errore dopo averlo restituito, quindi avvolgere il ciclo in un blocco try se il container deve uscire correttamente. Vedere [Gestire il risultato](/docs/it/agent-sdk/agent-loop#handle-the-result) per i sottotipi di errore.

<h3 id="long-running-sessions">
  Sessioni a lunga durata
</h3>

Eseguire istanze di container persistenti, spesso ospitando più processi SDK per container, per servire il lavoro in corso. Ideale per agenti che intraprendono azioni autonome, servono contenuti o gestiscono flussi di messaggi ad alto volume.

I carichi di lavoro di esempio includono un agente di posta che triage e risponde alla posta in arrivo, un generatore di siti che ospita un sito modificabile per utente attraverso le porte del container e un chatbot che gestisce il traffico continuo da una piattaforma come Slack.

Il container espone un endpoint HTTP o WebSocket e mappa ogni sessione attiva a una query di lunga durata e al sottoprocesso dietro di essa. In TypeScript, utilizzare [`streamInput()`](/docs/it/agent-sdk/typescript#query-object) per aggiungere turni a una sessione attiva e [`startup()`](/docs/it/agent-sdk/typescript#startup) per pre-riscaldare i sottoprocessi prima del traffico in arrivo. In Python, utilizzare [`ClaudeSDKClient`](/docs/it/agent-sdk/python#claudesdkclient) per mantenere una sessione aperta tra i turni. Dimensionare il container in modo che possa contenere il numero massimo di sessioni simultanee in memoria.

<h3 id="hybrid-sessions">
  Sessioni ibride
</h3>

Container effimeri che si idratano da un [`SessionStore`](/docs/it/agent-sdk/session-storage) all'avvio e persistono gli aggiornamenti indietro. Ideale per sessioni che si estendono su molte interazioni ma rimangono inattive tra di esse. Il container si spegne durante i periodi di inattività e si riaccende quando l'utente ritorna.

I carichi di lavoro di esempio includono un gestore di progetti personali con check-in intermittenti, ricerca approfondita che si interrompe e riprende nel corso di ore e un agente di supporto clienti che carica la cronologia dei ticket tra le interazioni.

Regolare il timeout di inattività del provider in base alla frequenza con cui ci si aspetta che gli utenti ritornino. Lo spegnimento di un container senza un `SessionStore` configurato perde la trascrizione con esso, quindi lo store è richiesto per questo modello, non facoltativo.

Il modello si basa sulla ripresa di una sessione per ID con uno store condiviso allegato:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // an object store, key-value store, database, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, SessionStore
  import asyncio

  user_input: str = ...
  session_id: str = ...              # looked up from your database by user
  session_store: SessionStore = ...  # an object store, key-value store, database, or your own adapter


  async def main():
      async for message in query(
          prompt=user_input,
          options=ClaudeAgentOptions(
              resume=session_id,
              session_store=session_store,
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="multi-agent-container">
  Container multi-agente
</h3>

Eseguire più sottoprocessi SDK all'interno di un container. Ideale per agenti che devono collaborare strettamente, ad esempio simulazioni multi-agente in cui gli agenti interagiscono tra loro in un ambiente condiviso.

Assegnare a ogni agente la propria directory di lavoro in modo che non sovrascrivano i file l'uno dell'altro e isolare il caricamento delle impostazioni in modo che i file `CLAUDE.md` per agente non si diffondano tra gli agenti. Vedere [Isolamento multi-tenant](#multi-tenant-isolation) per le opzioni specifiche.

<h2 id="provision-the-container">
  Provisioning del container
</h2>

<h3 id="container-based-sandboxing">
  Sandboxing basato su container
</h3>

Esegui l'SDK all'interno di un container sandbox per l'isolamento dei processi, i limiti delle risorse, il controllo della rete e un filesystem effimero.

Domande a cui rispondere quando si sceglie un provider:

* **Chi gestisce la sandbox**: un provider sandbox-as-a-service gestisce l'infrastruttura per te, mentre le opzioni self-hosted ti forniscono il software da eseguire sulla tua infrastruttura.
* **Latenza di cold-start**: quanto tempo passa da "crea una sandbox" a "pronto ad accettare la prima richiesta". I modelli effimeri necessitano di avvii inferiori al secondo. I modelli a lunga durata tollerano di più.
* **Archiviazione persistente**: se il provider offre volumi durevoli o solo disco effimero. Il modello ibrido necessita di archiviazione durevole da qualche parte, sia nella sandbox che accanto ad essa.
* **Modello di prezzo**: fatturazione al secondo, per richiesta o a tariffa oraria fissa. La tariffazione al secondo si adatta bene ai carichi di lavoro effimeri bursty. La tariffa oraria si adatta bene alle sessioni a lunga durata.
* **Networking**: supporto per regole di egress personalizzate, proxy in uscita e peering VPC privato per ambienti regolamentati.

Per le opzioni self-hosted come Docker, gVisor e Firecracker, e la configurazione dettagliata dell'isolamento, vedi [Isolation Technologies](/docs/it/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Dipendenze di runtime
</h3>

Il container necessita del runtime del linguaggio del tuo SDK:

* Python 3.10+ per l'SDK Python, o Node.js 18+ per l'SDK TypeScript
* Sia gli SDK TypeScript che Python includono un binario Claude Code nativo per la maggior parte delle installazioni, e la CLI generata non necessita di un'installazione separata di Node.js. Vedi la [nota di installazione della guida rapida](/docs/it/agent-sdk/quickstart) per le installazioni che necessitano di un'installazione separata di Claude Code nativo.

Il binario incluso è bloccato alla versione del pacchetto SDK, quindi aggiornare l'SDK è il modo per aggiornare la CLI. L'SDK segue semver: accetta continuamente le versioni patch e rivedi il changelog [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) o [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) prima di accettare una versione minore.

<h3 id="resources">
  Risorse
</h3>

1 GiB di RAM, 5 GiB di disco e 1 CPU per agente è un punto di partenza ragionevole per un'istanza appena avviata. L'utilizzo della memoria cresce con la durata della sessione e l'attività degli strumenti, quindi dimensiona in base alle lunghezze di sessione e alla concorrenza di cui hai effettivamente bisogno piuttosto che alla baseline inattiva. Vedi [Scaling and concurrency](#scaling-and-concurrency) per scoprire come calcolare gli agenti per host.

<h3 id="network">
  Rete
</h3>

L'SDK necessita di HTTPS in uscita verso `api.anthropic.com`, o verso l'endpoint regionale del tuo provider quando eseguito su Amazon Bedrock o sulla piattaforma agente di Google Cloud. Se i tuoi agenti utilizzano [MCP servers](/docs/it/agent-sdk/mcp) o strumenti esterni, necessitano anche di accesso in uscita a quegli endpoint. Per la produzione, instrada il traffico in uscita attraverso un proxy di egress che applica allowlist di domini, inietta credenziali e registra le richieste. Vedi [Secure Deployment](/docs/it/agent-sdk/secure-deployment) per il modello completo.

Per il traffico in ingresso, esponi una porta HTTP o WebSocket sul container. La tua applicazione gestisce le richieste dei client su quella porta e chiama l'SDK internamente; il sottoprocesso stesso non ascolta sulla rete.

<h2 id="handle-production-concerns">
  Gestire i problemi di produzione
</h2>

Affrontate queste decisioni prima di distribuire un agente self-hosted.

<h3 id="session-and-state-persistence">
  Persistenza della sessione e dello stato
</h3>

Il disco locale predefinito viene perso al riavvio, al ridimensionamento o al trasferimento a un nodo diverso. Per qualsiasi sessione che un utente si aspetta di riprendere, eseguire il mirroring della trascrizione su un archivio durevole con un adattatore [`SessionStore`](/docs/it/agent-sdk/session-storage). Vedere [Reference implementations](/docs/it/agent-sdk/session-storage#reference-implementations) per gli adattatori di esempio per un object store, un key-value store e un database, e una suite di conformità per il vostro.

Tre cose da sapere su come si comporta `SessionStore`:

* **Solo trascrizioni**: `SessionStore` esegue il mirroring delle trascrizioni, non dei file di memoria `CLAUDE.md` o di altri artefatti della directory di lavoro. Montate un volume condiviso o sincronizzate quelli separatamente.
* **Mirroring, non sostituzione**: il subprocess scrive prima su disco locale e l'SDK invia una copia di ogni batch all'archivio. La trascrizione locale di una sessione nuova sopravvive all'esecuzione; un'esecuzione ripresa dall'archivio elimina la sua copia locale alla fine, quindi l'archivio contiene l'unica copia durevole. Vedere [Dual-write architecture](/docs/it/agent-sdk/session-storage#dual-write-architecture).
* **Messaggi `mirror_error`**: quando l'SDK non riesce a consegnare un batch all'archivio, scarta il batch, emette un messaggio `{ type: "system", subtype: "mirror_error" }` e continua la query. Avvertite su questi se la durabilità dell'archivio è importante. Vedere [Mirror writes are best-effort](/docs/it/agent-sdk/session-storage#mirror-writes-are-best-effort) per il comportamento di retry e timeout.

<h3 id="observability">
  Osservabilità
</h3>

Gli agenti Agent SDK sono processi di lunga durata che generano chiamate di strumenti su molti round-trip API. Senza telemetria non potete vedere quali strumenti sono stati eseguiti, quanto tempo hanno impiegato o dove una sessione si è bloccata.

L'SDK eredita la configurazione OpenTelemetry dall'ambiente. Impostate le variabili di ambiente OTEL a livello di container o orchestrator in modo che ogni chiamata `query()` esporti span, metriche ed eventi di log al vostro collector. L'esempio seguente abilita l'esportazione OTLP per tutti e tre i segnali. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` è richiesto solo per le tracce; omettetelo se esportate solo metriche e log.

```bash title=".env" theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

Il testo del prompt e gli input degli strumenti non sono inclusi nelle esportazioni per impostazione predefinita. Vedere [Control sensitive data in exports](/docs/it/agent-sdk/observability#control-sensitive-data-in-exports) per i flag opt-in e [Observability](/docs/it/agent-sdk/observability) per il catalogo completo dei segnali.

<h3 id="auth-and-secrets">
  Autenticazione e segreti
</h3>

Tre problemi di autenticazione sono importanti al momento dell'hosting:

* **Anthropic API**: il subprocess legge `ANTHROPIC_API_KEY` dal suo ambiente. Forniscilo dal vostro gestore di segreti, oppure imposta `ANTHROPIC_BASE_URL` per instradare le chiamate del modello attraverso un proxy che inietta la chiave al di fuori del container. Vedere [Credential management](/docs/it/agent-sdk/secure-deployment#credential-management) per il modello proxy e [Setup in the SDK quickstart](/docs/it/agent-sdk/quickstart#setup) per i metodi di autenticazione supportati.
* **In entrata**: mettete l'autenticazione a un gateway davanti al container dell'agente. L'agente dovrebbe ricevere richieste pre-autenticate e non dovrebbe essere il componente che convalida i token dell'utente.
* **Strumenti in uscita**: mantenete le credenziali degli strumenti fuori dall'ambiente dell'agente. Instradare le chiamate in uscita attraverso un proxy che inietta le chiavi API dopo che la richiesta esce dal container. L'agente effettua la chiamata; il proxy aggiunge la credenziale.

<h3 id="scaling-and-concurrency">
  Scaling e concorrenza
</h3>

Ogni sessione viene eseguita nel suo proprio subprocess, quindi la concorrenza su un host è limitata da quanti subprocess la sua RAM può contenere.

Dimensionate ogni host con questa formula:

```text theme={null}
agents per host = (host RAM - overhead) / (per-session RAM ceiling)
```

Misurate il ceiling per sessione eseguendo una sessione rappresentativa fino alla vostra lunghezza target sotto il carico di strumenti previsto e registrando il picco RSS. Il punto di partenza di 1 GiB in [Resources](#resources) è un floor, non il ceiling.

Il routing con scaling orizzontale dipende dal vostro modello. Per le sessioni di lunga durata, dove i container contengono molte sessioni, eseguite un pool di container dietro un load balancer e fissate ogni sessione a un container utilizzando l'hashing coerente su `sessionId`. Una sessione fissata continua a colpire lo stesso container, e quindi lo stesso subprocess in esecuzione, fino a quando non viene rimossa o il container non si riavvia.

<h3 id="cost">
  Costo
</h3>

Il costo dei token Anthropic in genere domina il costo dell'infrastruttura del container di un ordine di grandezza o più. Un container minimamente provisioning funziona approssimativamente a \$0.05 all'ora, mentre una singola sessione di agente lungo può spendere dollari in token. Vedere [Cost tracking](/docs/it/agent-sdk/cost-tracking) per la contabilità dei token per sessione.

<h3 id="multi-tenant-isolation">
  Isolamento multi-tenant
</h3>

Il comportamento predefinito dell'SDK legge le impostazioni e i file di memoria `CLAUDE.md` dal filesystem. In un container condiviso che serve più tenant, questi file possono far trapelare il contesto di un tenant nella sessione di un altro tenant.

Per isolare i tenant all'interno di un container condiviso:

* Passate `settingSources: []` in TypeScript o `setting_sources=[]` in Python per saltare le impostazioni utente, progetto e locali.
* Impostate `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in `env`. [Auto memory](/docs/it/memory#auto-memory) a `~/.claude/projects/<project>/memory/` si carica nel prompt di sistema indipendentemente da `settingSources`. Vedere [What settingSources does not control](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control) per gli altri input che si caricano incondizionatamente.
* Puntate `CLAUDE_CONFIG_DIR` a una directory per tenant in modo che i tenant non condividano la configurazione globale `~/.claude.json`. Quando ogni directory di configurazione serve una directory di lavoro e non condividete un [`SessionStore`](/docs/it/agent-sdk/session-storage) tra i tenant, potete anche impostare [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/it/sessions#name-the-project-directory-yourself) in `env` per mantenere i percorsi delle trascrizioni sotto di esso brevi. Richiede Agent SDK TypeScript v0.3.234 o successivo, oppure Agent SDK Python v0.2.140 o successivo.
* Utilizzate una directory di lavoro per tenant. Passate `cwd` esplicitamente su ogni chiamata `query()`.
* Applicate regole di egress per tenant al vostro proxy, come IP in uscita distinti, credenziali o allowlist di domini, in modo che un tenant compromesso non possa esfiltare dati tramite la politica in uscita di un altro tenant.

L'esempio seguente applica le opzioni di impostazioni, memoria automatica, directory di configurazione e directory di lavoro insieme. Costruite `tenantDir` e `configDir` in modo che ogni tenant ottenga un percorso che nessun altro tenant possa leggere. In TypeScript, `env` sostituisce l'ambiente del subprocess, quindi diffondete `...process.env` per mantenere le variabili ereditate come `PATH` e `ANTHROPIC_API_KEY`. In Python, `env` viene unito sopra l'ambiente ereditato.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio

  prompt: str = ...
  tenant_dir: str = ...
  config_dir: str = ...


  async def main():
      async for message in query(
          prompt=prompt,
          options=ClaudeAgentOptions(
              cwd=tenant_dir,
              setting_sources=[],
              env={
                  "CLAUDE_CONFIG_DIR": config_dir,
                  "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="known-limitations">
  Limitazioni note
</h2>

Pianificate questi aspetti nella progettazione della vostra distribuzione.

| Limitazione                                                                    | Cosa fare                                                                                                                                                                                                                                                           |
| ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Nessun timeout di sessione di primo livello                                    | Una sessione non scade automaticamente. Impostare `maxTurns` in TypeScript o `max_turns` in Python per limitare quanti round trip di utilizzo di strumenti l'agente esegue prima di fermarsi.                                                                       |
| Crescita della memoria durante sessioni lunghe                                 | Limitare la lunghezza della sessione o riciclare i sottoprocessi periodicamente. Vedere [Scaling and concurrency](#scaling-and-concurrency).                                                                                                                        |
| I grandi fanout di subagent paralleli possono raggiungere i limiti di velocità | Suddividere il lavoro in batch più piccoli anziché emettere un'unica distribuzione ampia.                                                                                                                                                                           |
| Nessuna scadenza wall-clock per subagent                                       | Limitare ogni [subagent](/docs/it/agent-sdk/subagents) con `maxTurns` nella sua `AgentDefinition`. `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` imposta un watchdog di stallo che si attiva quando un subagent smette di produrre output; non è una scadenza di runtime totale. |

<h2 id="troubleshoot-deployment-failures">
  Risolvere i problemi di distribuzione
</h2>

Utilizza questa sezione quando un agente che funziona sulla tua macchina non riesce in un servizio distribuito. Ogni elemento sottostante nomina un errore e collega la voce che lo copre:

* **CLI non trovata all'avvio del servizio**: in Python, un contenitore o un gestore di servizi esegue la tua applicazione con un `PATH` diverso dalla tua shell, quindi un'installazione che funziona localmente non è visibile al processo. In TypeScript, la compilazione dell'immagine ha saltato le dipendenze opzionali dell'SDK, oppure `pathToClaudeCodeExecutable` punta a un file che non esiste nell'immagine. Vedi [Claude Code non trovato](/docs/it/agent-sdk/troubleshooting#clinotfounderror-claude-code-not-found).
* **CLI presente nell'immagine ma non si avvia**: Claude Code non può avviarsi da un binario che non corrisponde all'architettura del contenitore o a libc, oppure da un file che ha perso il permesso di esecuzione nella compilazione dell'immagine. Vedi [Impossibile avviare Claude Code](/docs/it/agent-sdk/troubleshooting#cliconnectionerror-failed-to-start-claude-code).
* **Il processo Claude Code esce durante l'esecuzione**: l'errore che la tua applicazione riceve dipende dal linguaggio dell'SDK e dal fatto che la CLI abbia segnalato un risultato di errore per primo. Le voci sotto [Uscita del processo CLI](/docs/it/agent-sdk/troubleshooting#cli-process-exit) coprono ogni messaggio.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Hosting cookbook](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): procedura dettagliata del notebook con [codice distribuibile](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) per Docker, Modal e Kubernetes.
* [Session storage](/docs/it/agent-sdk/session-storage): mantieni i transcript tra gli host con un adattatore `SessionStore`.
* [Observability](/docs/it/agent-sdk/observability): esporta tracce OTEL, metriche e log al tuo collector.
* [Secure deployment](/docs/it/agent-sdk/secure-deployment): controlli di rete, gestione delle credenziali e indurimento dell'isolamento.
* [Cost tracking](/docs/it/agent-sdk/cost-tracking): contabilità dei token e dei costi per sessione.
