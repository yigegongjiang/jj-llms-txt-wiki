> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hosting dell'Agent SDK

> Distribuisci l'Agent SDK in produzione: architettura subprocess, persistenza della sessione, scaling, osservabilità e isolamento multi-tenant per Docker, Kubernetes e provider sandbox.

L'Agent SDK genera e supervisiona un subprocess `claude` CLI che possiede una shell, una directory di lavoro e file di sessione su disco. Ospitarlo non è come ospitare un wrapper API senza stato. Ogni agente in esecuzione è un processo di lunga durata legato allo stato locale, il che influenza il modo in cui allochi risorse, persisti sessioni e scala tra tenant.

Questa pagina copre l'auto-hosting sulla tua infrastruttura: comprendi [il modello subprocess](#the-subprocess-model), [scegli un pattern di sessione](#choose-a-session-pattern), [provisiona il container](#provision-the-container) e [gestisci le preoccupazioni di produzione](#handle-production-concerns) come persistenza, osservabilità, autenticazione e isolamento multi-tenant. Per Dockerfile distribuibili e manifest Kubernetes, vedi il [hosting cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Se non hai bisogno di controllo dell'infrastruttura, isolamento personalizzato o il tuo piano dati, considera invece [Managed Agents](https://platform.claude.com/docs/it/managed-agents/overview): un'API REST ospitata dove Anthropic esegue l'agente e la sandbox, quindi la tua applicazione invia eventi e riceve risultati in streaming senza infrastruttura di hosting da gestire.

<Info>
  Per l'indurimento della sicurezza oltre il sandboxing di base, inclusi i controlli di rete, la gestione delle credenziali e le opzioni di isolamento, vedi [Secure Deployment](/docs/it/agent-sdk/secure-deployment).
</Info>

<h2 id="the-subprocess-model">
  Il modello subprocess
</h2>

Ogni decisione di hosting su questa pagina deriva da come l'SDK esegue l'agente. Quando il vostro codice chiama `query()`, l'SDK genera un processo CLI `claude` separato e comunica con esso tramite stdio. Quel subprocess possiede la shell, la directory di lavoro e i transcript della sessione JSONL su disco locale.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" alt="Flusso di richiesta: dal client alla vostra app, che genera un subprocess CLI claude su stdio all'interno del container; il subprocess scrive su disco locale e chiama api.anthropic.com su HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

Una sessione agente corrisponde a un subprocess. L'esecuzione di N sessioni concorrenti significa N subprocess, ognuno con il proprio albero di processi e file di transcript. Per impostazione predefinita ereditano tutti la directory di lavoro della vostra applicazione, quindi passate `cwd` su ogni chiamata `query()` quando le sessioni necessitano di filesystem separati:

<CodeGroup>
  ```typescript TypeScript theme={null}
  query({ prompt, options: { cwd: "/work/session-a" } })
  ```

  ```python Python theme={null}
  query(prompt=prompt, options=ClaudeAgentOptions(cwd="/work/session-a"))
  ```
</CodeGroup>

<h3 id="state-that-lives-on-local-disk">
  Stato che risiede su disco locale
</h3>

Tre tipi di stato agente risiedono sul filesystem del container per impostazione predefinita. Nessuno di essi sopravvive a un riavvio del container, a una riduzione della scala o a uno spostamento su un nodo diverso.

| Stato                               | Posizione predefinita                                                                                       |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Transcript della sessione           | `~/.claude/projects/`, o la directory `projects/` sotto `CLAUDE_CONFIG_DIR` se impostata                    |
| File di memoria `CLAUDE.md`         | `~/.claude/CLAUDE.md` per il livello utente e la directory di lavoro della sessione per il livello progetto |
| Artefatti della directory di lavoro | La directory di lavoro della sessione                                                                       |

Per persistere i transcript tra host, configurate un adattatore [`SessionStore`](/docs/it/agent-sdk/session-storage). I file di memoria e altri artefatti della directory di lavoro necessitano della loro propria strategia di archiviazione, come un volume montato o una sincronizzazione di object-store.

Per come le sessioni, la ripresa e il forking funzionano a livello di API, vedere [Sessions](/docs/it/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Scegliere un modello di sessione
</h2>

Questi quattro modelli coprono il ciclo di vita della sessione: quanto a lungo un container vive rispetto alle sessioni che serve. Per dove il container viene eseguito, il [hosting cookbook](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) ha [codice distribuibile](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) per Docker locale, Modal e Kubernetes. Scegliere un modello di sessione qui e un target di distribuzione dal cookbook.

<h3 id="ephemeral-sessions">
  Sessioni effimere
</h3>

Creare un container per ogni attività dell'utente e distruggerlo quando l'attività si completa. Migliore per attività una tantum. L'utente può ancora interagire con l'IA mentre l'attività si sta completando, ma una volta completata il container viene distrutto.

I carichi di lavoro di esempio includono l'investigazione e la correzione di bug, l'estrazione di fatture e ricevute, la traduzione di documenti e la trasformazione di media.

Il container esegue un entrypoint one-shot che chiama l'SDK ed esce. L'esempio seguente mostra una versione TypeScript minima. Salvarla come `entrypoint.mts` o impostare `"type": "module"` in `package.json` in modo che `await` di livello superiore sia disponibile.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const prompt = process.env.TASK_PROMPT!;
for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
  console.log(message);
}
```

<h3 id="long-running-sessions">
  Sessioni a lunga esecuzione
</h3>

Eseguire istanze di container persistenti, spesso ospitando più processi SDK per container, per servire il lavoro in corso. Migliore per agenti che intraprendono azioni autonome, servono contenuti o gestiscono flussi di messaggi ad alto volume.

I carichi di lavoro di esempio includono un agente di posta elettronica che triage e risponde alla posta in arrivo, un generatore di siti che ospita un sito modificabile per utente attraverso le porte del container e un chatbot che gestisce il traffico continuo da una piattaforma come Slack.

Il container espone un endpoint HTTP o WebSocket e mappa ogni sessione attiva a una query a lunga durata e al sottoprocesso dietro di essa. In TypeScript, utilizzare [`streamInput()`](/docs/it/agent-sdk/typescript#query-object) per aggiungere turni a una sessione attiva e [`startup()`](/docs/it/agent-sdk/typescript#startup) per pre-riscaldare i sottoprocessi prima del traffico in arrivo. In Python, utilizzare [`ClaudeSDKClient`](/docs/it/agent-sdk/python#claudesdkclient) per mantenere una sessione aperta tra i turni. Dimensionare il container in modo che possa contenere il numero massimo di sessioni simultanee in memoria.

<h3 id="hybrid-sessions">
  Sessioni ibride
</h3>

Container effimeri che si idratano da un [`SessionStore`](/docs/it/agent-sdk/session-storage) all'avvio e persistono gli aggiornamenti indietro. Migliore per sessioni che si estendono su molte interazioni ma rimangono inattive tra di esse. Il container si spegne durante i periodi di inattività e si riaccende quando l'utente ritorna.

I carichi di lavoro di esempio includono un project manager personale con check-in intermittenti, ricerca approfondita che si interrompe e riprende nel corso di ore e un agente di supporto clienti che carica la cronologia dei ticket tra le interazioni.

Regolare il timeout di inattività del provider in base alla frequenza con cui ci si aspetta che gli utenti ritornino. Spegnere un container senza un `SessionStore` configurato perde la trascrizione con esso, quindi lo store è richiesto per questo modello, non opzionale.

Il modello si basa sulla ripresa di una sessione per ID con uno store condiviso allegato:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // S3, Redis, Postgres, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=user_input,
      options=ClaudeAgentOptions(
          resume=session_id,            # looked up from your database by user
          session_store=session_store,  # S3, Redis, Postgres, or your own adapter
      ),
  ):
      ...
  ```
</CodeGroup>

Vedere [Session storage](/docs/it/agent-sdk/session-storage) per l'interfaccia `SessionStore` completa e gli adapter di riferimento.

<h3 id="multi-agent-container">
  Container multi-agente
</h3>

Eseguire più sottoprocessi SDK all'interno di un container. Migliore per agenti che devono collaborare strettamente, ad esempio simulazioni multi-agente in cui gli agenti interagiscono tra loro in un ambiente condiviso.

Dare a ogni agente la propria directory di lavoro in modo che non sovrascrivano i file l'uno dell'altro e isolare il caricamento delle impostazioni in modo che i file `CLAUDE.md` per agente non si diffondano tra gli agenti. Vedere [Multi-tenant isolation](#multi-tenant-isolation) per le opzioni specifiche.

<h2 id="provision-the-container">
  Provisioning del container
</h2>

<h3 id="container-based-sandboxing">
  Sandboxing basato su container
</h3>

Esegui l'SDK all'interno di un container sandbox per l'isolamento dei processi, i limiti delle risorse, il controllo della rete e un filesystem effimero. Diversi provider si specializzano in ambienti container sandbox che si adattano al modello dell'Agent SDK.

Domande a cui rispondere quando si sceglie un provider:

* **Chi gestisce la sandbox**: un provider sandbox-as-a-service gestisce l'infrastruttura per te, mentre le opzioni self-hosted ti forniscono il software da eseguire sulla tua infrastruttura.
* **Latenza di cold-start**: quanto tempo passa da "crea una sandbox" a "pronto ad accettare la prima richiesta". I pattern effimeri necessitano di avvii sub-secondo. I pattern a lunga durata tollerano di più.
* **Archiviazione persistente**: se il provider offre volumi durevoli o solo disco effimero. Il pattern ibrido necessita di archiviazione durevole da qualche parte, sia nella sandbox che accanto ad essa.
* **Modello di prezzo**: fatturazione per secondo, per richiesta o oraria fissa. La tariffazione per secondo si adatta ai carichi di lavoro effimeri bursty. La tariffazione oraria si adatta alle sessioni a lunga durata.
* **Networking**: supporto per regole di egress personalizzate, proxy in uscita e peering VPC privato per ambienti regolamentati.

Provider da valutare:

* [Modal Sandbox](https://modal.com/docs/guide/sandbox), con un'[implementazione demo](https://modal.com/docs/examples/claude-slack-gif-creator)
* [Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)
* [Daytona](https://www.daytona.io/)
* [E2B](https://e2b.dev/)
* [Fly Machines](https://fly.io/docs/machines/)
* [Vercel Sandbox](https://vercel.com/docs/functions/sandbox)

Per opzioni self-hosted come Docker, gVisor e Firecracker, e configurazione dettagliata dell'isolamento, vedi [Isolation Technologies](/docs/it/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Dipendenze di runtime
</h3>

Il container ha bisogno solo del runtime del linguaggio dell'SDK:

* Python 3.10+ per l'SDK Python, o Node.js 18+ per l'SDK TypeScript
* Entrambi i pacchetti SDK includono un binario Claude Code nativo per la piattaforma host, quindi non è necessaria un'installazione separata di Claude Code o Node.js per la CLI generata

Il binario incluso è bloccato alla versione del pacchetto SDK, quindi aggiornare l'SDK è il modo in cui aggiorni la CLI. L'SDK segue semver: accetta continuamente i rilasci patch e rivedi il changelog di [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) o [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) prima di accettare una versione minore.

<h3 id="resources">
  Risorse
</h3>

1 GiB di RAM, 5 GiB di disco e 1 CPU per agente è un punto di partenza ragionevole per un'istanza appena avviata. L'utilizzo della memoria cresce con la durata della sessione e l'attività degli strumenti, quindi dimensiona per le lunghezze di sessione e la concorrenza di cui hai effettivamente bisogno piuttosto che per la baseline inattiva. Vedi [Scaling and concurrency](#scaling-and-concurrency) per come calcolare gli agenti per host.

<h3 id="network">
  Rete
</h3>

L'SDK necessita di HTTPS in uscita verso `api.anthropic.com`, o verso l'endpoint regionale del tuo provider quando eseguito su Amazon Bedrock o Google Cloud's Agent Platform. Se i tuoi agenti utilizzano [server MCP](/docs/it/agent-sdk/mcp) o strumenti esterni, necessitano anche di accesso in uscita a quegli endpoint. Per la produzione, instrada il traffico in uscita attraverso un proxy di egress che applica allowlist di domini, inietta credenziali e registra le richieste. Vedi [Secure Deployment](/docs/it/agent-sdk/secure-deployment) per il pattern completo.

Per il traffico in ingresso, esponi una porta HTTP o WebSocket sul container. La tua applicazione gestisce le richieste dei client su quella porta e chiama l'SDK internamente; il sottoprocesso stesso non ascolta sulla rete.

<h2 id="handle-production-concerns">
  Gestire le preoccupazioni di produzione
</h2>

Affrontate queste decisioni prima di distribuire un agente self-hosted.

<h3 id="session-and-state-persistence">
  Persistenza della sessione e dello stato
</h3>

Il disco locale predefinito viene perso al riavvio, al ridimensionamento verso il basso o al trasferimento a un nodo diverso. Per qualsiasi sessione che un utente si aspetta di riprendere, eseguire il mirroring della trascrizione su un archivio durevole con un adattatore [`SessionStore`](/docs/it/agent-sdk/session-storage). Vedere [Implementazioni di riferimento](/docs/it/agent-sdk/session-storage#reference-implementations) per gli adattatori S3, Redis e Postgres e una suite di conformità per il vostro.

Tre cose da sapere su come si comporta `SessionStore`:

* **Solo trascrizioni**: `SessionStore` esegue il mirroring delle trascrizioni, non dei file di memoria `CLAUDE.md` o di altri artefatti della directory di lavoro. Montate un volume condiviso o sincronizzate quelli separatamente.
* **Mirroring, non sostituzione**: il sottoprocesso scrive su disco locale per primo, e l'archivio riceve una copia di ogni batch. Le scritture locali rimangono autorevoli.
* **Messaggi `mirror_error`**: un batch che l'archivio rifiuta viene inviato fino a tre volte in totale, con un breve backoff prima di ogni nuovo tentativo; una chiamata scaduta non viene ritentata. Se il batch continua a fallire, l'SDK lo scarta, emette un messaggio `{ type: "system", subtype: "mirror_error" }` e continua la query. Avvertite su questi se la durabilità dell'archivio è importante.

<h3 id="observability">
  Osservabilità
</h3>

Gli agenti SDK dell'Agent SDK sono processi a lunga durata che generano chiamate di strumenti su molti round-trip API. Senza telemetria non potete vedere quali strumenti sono stati eseguiti, quanto tempo hanno impiegato o dove una sessione si è bloccata.

L'SDK eredita la configurazione di OpenTelemetry dall'ambiente. Impostate le variabili di ambiente OTEL a livello di container o orchestrator in modo che ogni chiamata `query()` esporti span, metriche ed eventi di log al vostro collector. L'esempio seguente abilita l'esportazione OTLP per tutti e tre i segnali. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` è richiesto solo per le tracce; omettetelo se esportate solo metriche e log.

```bash title=".env' theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

Il testo del prompt e gli input degli strumenti non sono inclusi nelle esportazioni per impostazione predefinita. Vedere [Controllare i dati sensibili nelle esportazioni](/docs/it/agent-sdk/observability#control-sensitive-data-in-exports) per i flag di opt-in e [Osservabilità](/docs/it/agent-sdk/observability) per il catalogo completo dei segnali.

<h3 id="auth-and-secrets">
  Autenticazione e segreti
</h3>

Tre preoccupazioni di autenticazione sono importanti al momento dell'hosting:

* **API Anthropic**: il sottoprocesso legge `ANTHROPIC_API_KEY` dal suo ambiente. Forniscilo dal vostro gestore di segreti, oppure imposta `ANTHROPIC_BASE_URL` per instradare le chiamate del modello attraverso un proxy che inietta la chiave al di fuori del container. Vedere [Gestione delle credenziali](/docs/it/agent-sdk/secure-deployment#credential-management) per il pattern del proxy e la [Panoramica dell'SDK](/docs/it/agent-sdk/overview#get-started) per i metodi di autenticazione supportati.
* **In entrata**: mettete l'autenticazione a un gateway davanti al container dell'agente. L'agente dovrebbe ricevere richieste pre-autenticate e non dovrebbe essere il componente che convalida i token dell'utente.
* **Strumenti in uscita**: mantenete le credenziali degli strumenti fuori dall'ambiente dell'agente. Instradare le chiamate in uscita attraverso un proxy che inietta le chiavi API dopo che la richiesta esce dal container. L'agente effettua la chiamata; il proxy aggiunge la credenziale.

<h3 id="scaling-and-concurrency">
  Scalabilità e concorrenza
</h3>

Ogni sessione viene eseguita nel suo sottoprocesso, quindi la concorrenza su un host è limitata da quanti sottoprocessi la sua RAM può contenere.

Dimensionate ogni host con questa formula:

```text theme={null}
agenti per host = (RAM host - overhead) / (limite massimo RAM per sessione)
```

Misurate il limite massimo per sessione eseguendo una sessione rappresentativa fino alla vostra lunghezza target sotto il carico di strumenti previsto e registrando il picco RSS. Il punto di partenza di 1 GiB in [Risorse](#resources) è un minimo, non il limite massimo.

L'instradamento con scalabilità orizzontale dipende dal vostro pattern. Per sessioni a lunga esecuzione, dove i container contengono molte sessioni, eseguite un pool di container dietro un load balancer e fissate ogni sessione a un container utilizzando l'hashing coerente su `sessionId`. Una sessione fissata continua a colpire lo stesso container, e quindi lo stesso sottoprocesso in esecuzione, fino a quando non viene rimossa o il container non si riavvia.

I grandi fanout di [subagenti](/docs/it/agent-sdk/subagents) concorrenti da una singola sessione possono raggiungere i limiti di velocità dell'API. Suddividete il lavoro in batch più piccoli piuttosto che emettere un dispatch ampio.

<h3 id="cost">
  Costo
</h3>

Il costo dei token di Anthropic in genere domina il costo dell'infrastruttura del container di un ordine di grandezza o più. Un container minimamente provisioning funziona approssimativamente a \$0,05 all'ora, mentre una singola sessione di agente lungo può spendere dollari in token. Vedere [Tracciamento dei costi](/docs/it/agent-sdk/cost-tracking) per la contabilità dei token per sessione.

<h3 id="multi-tenant-isolation">
  Isolamento multi-tenant
</h3>

Il comportamento predefinito dell'SDK legge le impostazioni e i file di memoria `CLAUDE.md` dal filesystem. In un container condiviso che serve più tenant, questi file possono far trapelare il contesto di un tenant nella sessione di un altro tenant.

Per isolare i tenant all'interno di un container condiviso:

* Passate `settingSources: []` in TypeScript o `setting_sources=[]` in Python in modo che nessuna impostazione del filesystem venga caricata.
* Impostate `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in `env`. La [memoria automatica](/docs/it/memory#auto-memory) in `~/.claude/projects/<project>/memory/` viene caricata nel prompt di sistema indipendentemente da `settingSources`. Vedere [Cosa settingSources non controlla](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control) per gli altri input che vengono caricati incondizionatamente.
* Puntate `CLAUDE_CONFIG_DIR` a una directory per tenant in modo che i tenant non condividano la configurazione globale `~/.claude.json`.
* Utilizzate una directory di lavoro per tenant. Passate `cwd` esplicitamente su ogni chiamata `query()`.
* Applicate regole di uscita per tenant al vostro proxy, come IP in uscita distinti, credenziali o elenchi di domini consentiti, in modo che un tenant compromesso non possa esfiltare dati tramite la politica in uscita di un altro tenant.

L'esempio seguente applica insieme le quattro opzioni a livello di SDK. Costruite `tenantDir` e `configDir` in modo che ogni tenant ottenga un percorso che nessun altro tenant possa leggere. In TypeScript, `env` sostituisce l'ambiente del sottoprocesso, quindi diffondete `...process.env` per mantenere le variabili ereditate come `PATH` e `ANTHROPIC_API_KEY`. In Python, `env` viene unito sopra l'ambiente ereditato.

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
  ```
</CodeGroup>

Per i controlli di rete per tenant, vedere [Distribuzione Sicura](/docs/it/agent-sdk/secure-deployment).

<h2 id="known-limitations">
  Limitazioni note
</h2>

Pianifica intorno a queste nella progettazione della tua distribuzione.

| Limitazione                                                                    | Cosa fare                                                                                                                                                                                                                                                                                                              |
| ------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Nessun timeout di sessione di primo livello                                    | Una sessione non scade da sola. Imposta `maxTurns` in `Options` per limitare quanti round trip di utilizzo di strumenti l'agente esegue prima di fermarsi.                                                                                                                                                             |
| Crescita della memoria durante sessioni lunghe                                 | Limita la lunghezza della sessione o ricicla i sottoprocessi periodicamente. Vedi [Scaling and concurrency](#scaling-and-concurrency).                                                                                                                                                                                 |
| I grandi fanout di subagent paralleli possono raggiungere i limiti di velocità | Suddividi il lavoro in batch più piccoli piuttosto che emettere un'unica distribuzione ampia.                                                                                                                                                                                                                          |
| Nessuna scadenza wall-clock per subagent                                       | Limita ogni [subagent](/docs/it/agent-sdk/subagents) con `maxTurns` nella sua `AgentDefinition`. Solo per subagent in background, `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` imposta un watchdog di stallo che si attiva quando un subagent `run_in_background` smette di produrre output; non è una scadenza di runtime totale. |

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Hosting cookbook](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): procedura dettagliata del notebook con [codice distribuibile](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) per Docker, Modal e Kubernetes.
* [Session storage](/docs/it/agent-sdk/session-storage): mantieni i transcript tra gli host con un adattatore `SessionStore`.
* [Observability](/docs/it/agent-sdk/observability): esporta tracce OTEL, metriche e log al tuo collector.
* [Secure deployment](/docs/it/agent-sdk/secure-deployment): controlli di rete, gestione delle credenziali e indurimento dell'isolamento.
* [Cost tracking](/docs/it/agent-sdk/cost-tracking): contabilità dei token e dei costi per sessione.
