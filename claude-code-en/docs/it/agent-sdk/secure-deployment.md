> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Distribuzione sicura di agenti AI

> Una guida per proteggere le distribuzioni di Claude Code e Agent SDK con isolamento, gestione delle credenziali e controlli di rete

Claude Code e l'Agent SDK sono strumenti potenti che possono eseguire codice, accedere a file e interagire con servizi esterni per conto vostro. Come qualsiasi strumento con queste capacità, distribuirli con attenzione garantisce di ottenere i vantaggi mantenendo controlli appropriati.

A differenza del software tradizionale che segue percorsi di codice predeterminati, questi strumenti generano le loro azioni dinamicamente in base al contesto e agli obiettivi. Questa flessibilità è ciò che li rende utili, ma significa anche che il loro comportamento può essere influenzato dal contenuto che elaborano: file, pagine web o input dell'utente. Questo è talvolta chiamato prompt injection. Ad esempio, se il README di un repository contiene istruzioni insolite, Claude Code potrebbe incorporarle nelle sue azioni in modi che l'operatore non aveva anticipato. Questa guida copre modi pratici per ridurre questo rischio.

La buona notizia è che proteggere una distribuzione di agenti non richiede infrastrutture esotiche. Gli stessi principi che si applicano all'esecuzione di qualsiasi codice semi-affidabile si applicano qui: isolamento, privilegio minimo e difesa in profondità. Claude Code include diverse funzionalità di sicurezza che aiutano con le preoccupazioni comuni, e questa guida le esamina insieme a opzioni di hardening aggiuntive per chi ne ha bisogno.

Non ogni distribuzione ha bisogno della massima sicurezza. Uno sviluppatore che esegue Claude Code sul proprio laptop ha requisiti diversi da un'azienda che elabora dati dei clienti in un ambiente multi-tenant. Questa guida presenta opzioni che vanno dalle funzionalità di sicurezza integrate di Claude Code alle architetture di produzione indurizzate, in modo che possiate scegliere ciò che si adatta alla vostra situazione.

<h2 id="threat-model">
  Modello di minaccia
</h2>

Gli agenti possono intraprendere azioni indesiderate a causa di prompt injection (istruzioni incorporate nel contenuto che elaborano) o errore del modello. I modelli Claude sono progettati per resistere a questo; consultate la [panoramica del modello](https://platform.claude.com/docs/it/about-claude/models/overview) e la scheda di sistema per il modello che distribuite per i dettagli di valutazione.

La difesa in profondità è comunque una buona pratica. Ad esempio, se un agente elabora un file dannoso che gli istruisce di inviare dati dei clienti a un server esterno, i controlli di rete possono bloccare completamente quella richiesta.

<h2 id="built-in-security-features">
  Funzionalità di sicurezza integrate
</h2>

Claude Code include diverse funzionalità di sicurezza che affrontano le preoccupazioni comuni. Consultate la [documentazione sulla sicurezza](/docs/it/security) per i dettagli completi.

* **Sistema di autorizzazioni**: Ogni strumento e comando bash può essere configurato per consentire, bloccare o richiedere l'approvazione dell'utente. Utilizzate i pattern glob per creare regole come "consenti tutti i comandi npm" o "blocca qualsiasi comando con sudo". Le organizzazioni possono impostare politiche che si applicano a tutti gli utenti. Consultate [autorizzazioni](/docs/it/permissions).
* **Analisi dei comandi per le autorizzazioni**: Prima di eseguire i comandi bash, Claude Code li analizza in un AST e confronta il risultato con le vostre regole di autorizzazione. I comandi che non possono essere analizzati correttamente, o che non corrispondono a una regola di consentimento, richiedono approvazione esplicita. Un piccolo insieme di costrutti come `eval` richiedono sempre approvazione indipendentemente dalle regole di consentimento. Questo è un gate di autorizzazione, non una sandbox; non deduce se un comando è pericoloso dal suo percorso di destinazione o effetti.
* **Riepilogo della ricerca web**: I risultati della ricerca vengono riepilogati piuttosto che passare il contenuto grezzo direttamente nel contesto, riducendo il rischio di prompt injection da contenuto web dannoso.
* **Modalità sandbox**: I comandi Bash possono essere eseguiti in un ambiente sandbox che limita l'accesso al filesystem e alla rete. Consultate la [documentazione del sandboxing](/docs/it/sandboxing) per i dettagli.

<h2 id="security-principles">
  Principi di sicurezza
</h2>

Per le distribuzioni che richiedono hardening aggiuntivo oltre ai valori predefiniti di Claude Code, questi principi guidano le opzioni disponibili.

<h3 id="security-boundaries">
  Confini di sicurezza
</h3>

Un confine di sicurezza separa i componenti con diversi livelli di fiducia. Per le distribuzioni ad alta sicurezza, potete posizionare le risorse sensibili (come le credenziali) al di fuori del confine che contiene l'agente. Se qualcosa va storto nell'ambiente dell'agente, le risorse al di fuori di quel confine rimangono protette.

Ad esempio, piuttosto che dare a un agente accesso diretto a una chiave API, potreste eseguire un proxy al di fuori dell'ambiente dell'agente che inietta la chiave nelle richieste. L'agente può effettuare chiamate API, ma non vede mai la credenziale stessa. Questo pattern è utile per le distribuzioni multi-tenant o quando si elabora contenuto non affidabile.

<h3 id="least-privilege">
  Privilegio minimo
</h3>

Quando necessario, potete limitare l'agente solo alle capacità richieste per il suo compito specifico:

| Risorsa             | Opzioni di restrizione                                          |
| ------------------- | --------------------------------------------------------------- |
| Filesystem          | Montare solo le directory necessarie, preferire la sola lettura |
| Rete                | Limitare a endpoint specifici tramite proxy                     |
| Credenziali         | Iniettare tramite proxy piuttosto che esporre direttamente      |
| Capacità di sistema | Eliminare le capacità Linux nei container                       |

<h3 id="defense-in-depth">
  Difesa in profondità
</h3>

Per gli ambienti ad alta sicurezza, stratificare più controlli fornisce protezione aggiuntiva. Le opzioni includono:

* Isolamento dei container
* Restrizioni di rete
* Controlli del filesystem
* Convalida delle richieste presso un proxy

La giusta combinazione dipende dal vostro modello di minaccia e dai requisiti operativi.

<h2 id="isolation-technologies">
  Tecnologie di isolamento
</h2>

Diverse tecnologie di isolamento offrono diversi compromessi tra forza di sicurezza, prestazioni e complessità operativa.

<Info>
  In tutte queste configurazioni, Claude Code (o la vostra applicazione Agent SDK) viene eseguito all'interno del confine di isolamento (la sandbox, il container o la VM). I controlli di sicurezza descritti di seguito limitano ciò che l'agente può accedere da quel confine.
</Info>

| Tecnologia             | Forza di isolamento                      | Overhead di prestazioni | Complessità |
| ---------------------- | ---------------------------------------- | ----------------------- | ----------- |
| Sandbox runtime        | Buona (impostazioni predefinite sicure)  | Molto bassa             | Bassa       |
| Container (Docker)     | Dipende dalla configurazione             | Bassa                   | Media       |
| gVisor                 | Eccellente (con configurazione corretta) | Media/Alta              | Media       |
| VM (Firecracker, QEMU) | Eccellente (con configurazione corretta) | Alta                    | Media/Alta  |

<h3 id="sandbox-runtime">
  Sandbox runtime
</h3>

Per l'isolamento leggero senza container, [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) applica restrizioni di filesystem e rete a livello del sistema operativo.

Il vantaggio principale è la semplicità: non è richiesta alcuna configurazione Docker, immagini di container o configurazione di rete. Il proxy e le restrizioni del filesystem sono integrati. Fornite un file di configurazione che specifica i domini e i percorsi consentiti.

**Come funziona:**

* **Filesystem**: Utilizza primitive del sistema operativo (`bubblewrap` su Linux, `sandbox-exec` su macOS) per limitare l'accesso in lettura/scrittura ai percorsi configurati
* **Rete**: Rimuove lo spazio dei nomi di rete (Linux) o utilizza i profili Seatbelt (macOS) per instradare il traffico di rete attraverso un proxy integrato
* **Configurazione**: Allowlist basate su JSON per domini e percorsi del filesystem

**Configurazione:**

```bash theme={null}
npm install @anthropic-ai/sandbox-runtime
```

Quindi create un file di configurazione che specifica i percorsi e i domini consentiti.

**Considerazioni sulla sicurezza:**

1. **Kernel dello stesso host**: A differenza delle VM, i processi in sandbox condividono il kernel dell'host. Una vulnerabilità del kernel potrebbe teoricamente abilitare un'evasione. Per alcuni modelli di minaccia questo è accettabile, ma se avete bisogno di isolamento a livello di kernel, utilizzate gVisor o una VM separata.

2. **Nessuna ispezione TLS**: Il proxy consente i domini in base al nome host fornito dal client e non termina o ispeziona il traffico crittografato. Il codice in esecuzione all'interno della sandbox può potenzialmente utilizzare [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) o tecniche simili per raggiungere host al di fuori dell'allowlist. Se il vostro modello di minaccia richiede garanzie più forti, configurate un [proxy che termina TLS](#traffic-forwarding). Consultate le [limitazioni di sicurezza del sandboxing](/docs/it/sandboxing#security-limitations) per ulteriori dettagli. Separatamente, se l'agente ha credenziali permissive per un dominio consentito, assicuratevi che non possa utilizzare quel dominio per attivare altre richieste di rete o per estrarre dati.

Per molti casi di uso a sviluppatore singolo e CI/CD, sandbox-runtime aumenta significativamente la barra con una configurazione minima. Le sezioni seguenti coprono container e VM per le distribuzioni che richiedono un isolamento più forte.

<h3 id="containers">
  Container
</h3>

I container forniscono isolamento attraverso i namespace di Linux. Ogni container ha la sua propria vista del filesystem, dell'albero dei processi e dello stack di rete, mentre condivide il kernel dell'host.

Una configurazione di container con hardening di sicurezza potrebbe assomigliare a questa:

```bash theme={null}
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

Ecco cosa fa ogni opzione:

| Opzione                            | Scopo                                                                                                                                                                            |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--cap-drop ALL`                   | Rimuove le capacità Linux come `NET_ADMIN` e `SYS_ADMIN` che potrebbero abilitare l'escalation dei privilegi                                                                     |
| `--security-opt no-new-privileges` | Impedisce ai processi di ottenere privilegi attraverso i binari setuid                                                                                                           |
| `--security-opt seccomp=...`       | Limita le syscall disponibili; il valore predefinito di Docker blocca \~44, i profili personalizzati possono bloccare di più                                                     |
| `--read-only`                      | Rende il filesystem root del container immutabile, impedendo all'agente di persistere i cambiamenti                                                                              |
| `--tmpfs /tmp:...`                 | Fornisce una directory temporanea scrivibile che viene cancellata quando il container si ferma                                                                                   |
| `--network none`                   | Rimuove tutte le interfacce di rete; l'agente comunica attraverso il socket Unix montato di seguito                                                                              |
| `--memory 2g`                      | Limita l'utilizzo della memoria per prevenire l'esaurimento delle risorse                                                                                                        |
| `--pids-limit 100`                 | Limita il conteggio dei processi per prevenire fork bomb                                                                                                                         |
| `--user 1000:1000`                 | Viene eseguito come utente non root                                                                                                                                              |
| `-v ...:/workspace:ro`             | Monta il codice in sola lettura in modo che l'agente possa analizzarlo ma non modificarlo. **Evitate di montare directory host sensibili come `~/.ssh`, `~/.aws` o `~/.config`** |
| `-v .../proxy.sock:...`            | Monta un socket Unix connesso a un proxy in esecuzione al di fuori del container (vedi sotto)                                                                                    |

**Architettura del socket Unix:**

Con `--network none`, il container non ha alcuna interfaccia di rete. L'unico modo per l'agente di raggiungere il mondo esterno è attraverso il socket Unix montato, che si connette a un proxy in esecuzione sull'host. Questo proxy può applicare allowlist di domini, iniettare credenziali e registrare tutto il traffico.

Questa è la stessa architettura utilizzata da [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime). Anche se l'agente è compromesso tramite prompt injection, non può estrarre dati a server arbitrari. Può solo comunicare attraverso il proxy, che controlla quali domini sono raggiungibili. Per ulteriori dettagli, consultate il [post del blog sul sandboxing di Claude Code](https://www.anthropic.com/engineering/claude-code-sandboxing).

**Opzioni di hardening aggiuntive:**

| Opzione          | Scopo                                                                                                                                             |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--userns-remap` | Mappa il root del container a un utente host senza privilegi; richiede la configurazione del daemon ma limita i danni dall'evasione del container |
| `--ipc private`  | Isola la comunicazione tra processi per prevenire attacchi tra container                                                                          |

<h3 id="gvisor">
  gVisor
</h3>

I container standard condividono il kernel dell'host: quando il codice all'interno di un container effettua una chiamata di sistema, va direttamente allo stesso kernel che esegue l'host. Questo significa che una vulnerabilità del kernel potrebbe consentire l'evasione del container. gVisor affronta questo intercettando le chiamate di sistema nello spazio utente prima che raggiungano il kernel dell'host, implementando il suo proprio livello di compatibilità che gestisce la maggior parte delle syscall senza coinvolgere il kernel reale.

Se un agente esegue codice dannoso (forse a causa di prompt injection), quel codice viene eseguito nel container e potrebbe tentare exploit del kernel. Con gVisor, la superficie di attacco è molto più piccola: il codice dannoso dovrebbe prima sfruttare l'implementazione dello spazio utente di gVisor e avrebbe accesso limitato al kernel reale.

Per utilizzare gVisor con Docker, installate il runtime `runsc` e configurate il daemon:

```json theme={null}
// /etc/docker/daemon.json
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

Quindi eseguite i container con:

```bash theme={null}
docker run --runtime=runsc agent-image
```

**Considerazioni sulle prestazioni:**

| Carico di lavoro   | Overhead                                                          |
| ------------------ | ----------------------------------------------------------------- |
| Calcolo CPU-bound  | \~0% (nessuna intercettazione di syscall)                         |
| Syscall semplici   | \~2× più lento                                                    |
| I/O file intensivo | Fino a 10-200× più lento per pattern di apertura/chiusura pesanti |

Per gli ambienti multi-tenant o quando si elabora contenuto non affidabile, l'isolamento aggiuntivo spesso vale l'overhead.

<h3 id="virtual-machines">
  Macchine virtuali
</h3>

Le VM forniscono isolamento a livello hardware attraverso le estensioni di virtualizzazione della CPU. Ogni VM esegue il suo proprio kernel, creando un confine forte. Una vulnerabilità nel kernel guest non compromette direttamente l'host. Tuttavia, le VM non sono automaticamente "più sicure" di alternative come gVisor. La sicurezza della VM dipende molto dall'hypervisor e dal codice di emulazione dei dispositivi.

Firecracker è progettato per l'isolamento leggero di microVM. Può avviare VM in meno di 125ms con meno di 5 MiB di overhead di memoria, eliminando l'emulazione dei dispositivi non necessaria per ridurre la superficie di attacco.

Con questo approccio, la VM dell'agente non ha alcuna interfaccia di rete esterna. Invece, comunica attraverso `vsock` (socket virtuali). Tutto il traffico viene instradato tramite vsock a un proxy sull'host, che applica allowlist e inietta credenziali prima di inoltrare le richieste.

<h3 id="cloud-deployments">
  Distribuzioni cloud
</h3>

Per le distribuzioni cloud, potete combinare qualsiasi tecnologia di isolamento di cui sopra con controlli di rete nativi del cloud:

1. Eseguite i container dell'agente in una subnet privata senza gateway internet
2. Configurate le regole del firewall cloud (AWS Security Groups, GCP VPC firewall) per bloccare tutto l'egress tranne verso il vostro proxy
3. Eseguite un proxy (come [Envoy](https://www.envoyproxy.io/) con il suo filtro `credential_injector`) che convalida le richieste, applica allowlist di domini, inietta credenziali e inoltra alle API esterne
4. Assegnate autorizzazioni IAM minime all'account di servizio dell'agente, instradando l'accesso sensibile attraverso il proxy dove possibile
5. Registrate tutto il traffico presso il proxy a scopo di audit

<h2 id="credential-management">
  Gestione delle credenziali
</h2>

Gli agenti spesso hanno bisogno di credenziali per chiamare API, accedere a repository o interagire con servizi cloud. La sfida è fornire questo accesso senza esporre le credenziali stesse.

<h3 id="the-proxy-pattern">
  Il pattern del proxy
</h3>

L'approccio consigliato è eseguire un proxy al di fuori del confine di sicurezza dell'agente che inietta le credenziali nelle richieste in uscita. L'agente invia richieste senza credenziali, il proxy le aggiunge e inoltra la richiesta alla sua destinazione.

Questo pattern ha diversi vantaggi:

1. L'agente non vede mai le credenziali effettive
2. Il proxy può applicare un allowlist di endpoint consentiti
3. Il proxy può registrare tutte le richieste per l'audit
4. Le credenziali vengono archiviate in un'unica posizione sicura piuttosto che distribuite a ogni agente

<h3 id="configuring-claude-code-to-use-a-proxy">
  Configurazione di Claude Code per utilizzare un proxy
</h3>

Claude Code supporta due metodi per instradare le richieste di sampling attraverso un proxy:

**Opzione 1: ANTHROPIC\_BASE\_URL (semplice ma solo per le richieste API di sampling)**

```bash theme={null}
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

Questo dice a Claude Code e all'Agent SDK di inviare le richieste di sampling al vostro proxy invece che direttamente all'API Claude. Il vostro proxy riceve richieste HTTP in testo semplice, può ispezionarle e modificarle (incluso l'iniezione di credenziali), quindi inoltra all'API reale.

**Opzione 2: HTTP\_PROXY / HTTPS\_PROXY (a livello di sistema)**

```bash theme={null}
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code e l'Agent SDK rispettano queste variabili di ambiente standard, instradando tutto il traffico HTTP attraverso il proxy. Per HTTPS, il proxy crea un tunnel CONNECT crittografato: non può vedere o modificare il contenuto delle richieste senza l'intercettazione TLS.

<h3 id="implementing-a-proxy">
  Implementazione di un proxy
</h3>

Potete costruire il vostro proxy o utilizzarne uno esistente:

* [Envoy Proxy](https://www.envoyproxy.io/): proxy di livello produzione con filtro `credential_injector` per aggiungere header di autenticazione
* [mitmproxy](https://mitmproxy.org/): proxy che termina TLS per ispezionare e modificare il traffico HTTPS
* [Squid](http://www.squid-cache.org/): proxy di caching con liste di controllo di accesso
* [LiteLLM](https://github.com/BerriAI/litellm): gateway LLM con iniezione di credenziali e rate limiting

<h3 id="credentials-for-other-services">
  Credenziali per altri servizi
</h3>

Oltre al sampling dall'API Claude, gli agenti spesso hanno bisogno di accesso autenticato ad altri servizi, come repository git, database e API interne. Ci sono due approcci principali:

<h4 id="custom-tools">
  Strumenti personalizzati
</h4>

Fornire accesso attraverso un server MCP o uno strumento personalizzato che instrada le richieste a un servizio in esecuzione al di fuori del confine di sicurezza dell'agente. L'agente chiama lo strumento, ma la richiesta autenticata effettiva avviene all'esterno. Le chiamate dello strumento vanno a un proxy che inietta le credenziali.

Ad esempio, un server MCP git potrebbe accettare comandi dall'agente ma inoltrarli a un proxy git in esecuzione sull'host, che aggiunge l'autenticazione prima di contattare il repository remoto. L'agente non vede mai le credenziali.

Vantaggi:

* **Nessuna intercettazione TLS**: Il servizio esterno effettua richieste autenticate direttamente
* **Le credenziali rimangono all'esterno**: L'agente vede solo l'interfaccia dello strumento, non le credenziali sottostanti

<h4 id="traffic-forwarding">
  Inoltro del traffico
</h4>

Per le chiamate all'API Claude, `ANTHROPIC_BASE_URL` vi consente di instradare le richieste a un proxy che può ispezionarle e modificarle in testo semplice. Ma per altri servizi HTTPS (GitHub, registri npm, API interne), il traffico è spesso crittografato end-to-end. Anche se lo instradate attraverso un proxy tramite `HTTP_PROXY`, il proxy vede solo un tunnel TLS opaco e non può iniettare credenziali.

Per modificare il traffico HTTPS verso servizi arbitrari, senza utilizzare uno strumento personalizzato, avete bisogno di un proxy che termina TLS che decrittografa il traffico, lo ispeziona o lo modifica, quindi lo ricripta prima di inoltrarlo. Questo richiede:

1. Esecuzione del proxy al di fuori del container dell'agente
2. Installazione del certificato CA del proxy nell'archivio di fiducia dell'agente (in modo che l'agente si fidi dei certificati del proxy)
3. Configurazione di `HTTP_PROXY`/`HTTPS_PROXY` per instradare il traffico attraverso il proxy

Questo approccio gestisce qualsiasi servizio basato su HTTP senza scrivere strumenti personalizzati, ma aggiunge complessità attorno alla gestione dei certificati.

Notate che non tutti i programmi rispettano `HTTP_PROXY`/`HTTPS_PROXY`. La maggior parte degli strumenti (curl, pip, npm, git) lo fa, ma alcuni potrebbero bypassare queste variabili e connettersi direttamente. Ad esempio, `fetch()` di Node.js ignora queste variabili per impostazione predefinita; in Node 24+ potete impostare `NODE_USE_ENV_PROXY=1` per abilitare il supporto. Per una copertura completa, potete utilizzare [proxychains](https://github.com/haad/proxychains) per intercettare le chiamate di rete, o configurare iptables per reindirizzare il traffico in uscita a un proxy trasparente.

<Info>
  Un **proxy trasparente** intercetta il traffico a livello di rete, quindi il client non ha bisogno di essere configurato per utilizzarlo. I proxy regolari richiedono ai client di connettersi esplicitamente e parlare HTTP CONNECT o SOCKS. I proxy trasparenti (come Squid o mitmproxy in modalità trasparente) possono gestire connessioni TCP grezze reindirizzate.
</Info>

Entrambi gli approcci richiedono ancora il proxy che termina TLS e il certificato CA affidabile. Assicurano semplicemente che il traffico raggiunga effettivamente il proxy.

<h2 id="filesystem-configuration">
  Configurazione del filesystem
</h2>

I controlli del filesystem determinano quali file l'agente può leggere e scrivere.

<h3 id="read-only-code-mounting">
  Montaggio del codice in sola lettura
</h3>

Quando l'agente ha bisogno di analizzare il codice ma non modificarlo, montate la directory in sola lettura:

```bash theme={null}
docker run -v /path/to/code:/workspace:ro agent-image
```

<Warning>
  Anche l'accesso in sola lettura a una directory di codice può esporre le credenziali. File comuni da escludere o bonificare prima del montaggio:

  | File                                                    | Rischio                                     |
  | ------------------------------------------------------- | ------------------------------------------- |
  | `.env`, `.env.local`                                    | Chiavi API, password del database, segreti  |
  | `~/.git-credentials`                                    | Password/token git in testo semplice        |
  | `~/.aws/credentials`                                    | Chiavi di accesso AWS                       |
  | `~/.config/gcloud/application_default_credentials.json` | Token ADC di Google Cloud                   |
  | `~/.azure/`                                             | Credenziali CLI di Azure                    |
  | `~/.docker/config.json`                                 | Token di autenticazione del registro Docker |
  | `~/.kube/config`                                        | Credenziali del cluster Kubernetes          |
  | `.npmrc`, `.pypirc`                                     | Token del registro dei pacchetti            |
  | `*-service-account.json`                                | Chiavi dell'account di servizio GCP         |
  | `*.pem`, `*.key`                                        | Chiavi private                              |

  Considerate di copiare solo i file sorgente necessari, o di utilizzare il filtraggio in stile `.dockerignore`.
</Warning>

<h3 id="writable-locations">
  Posizioni scrivibili
</h3>

Se l'agente ha bisogno di scrivere file, avete alcune opzioni a seconda che vogliate che i cambiamenti persistano:

Per gli spazi di lavoro effimeri nei container, utilizzate i montaggi `tmpfs` che esistono solo in memoria e vengono cancellati quando il container si ferma:

```bash theme={null}
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

Se volete rivedere i cambiamenti prima di renderli persistenti, un filesystem overlay consente all'agente di scrivere senza modificare i file sottostanti. I cambiamenti vengono archiviati in un livello separato che potete ispezionare, applicare o scartare. Per un output completamente persistente, montate un volume dedicato ma mantenetelo separato dalle directory sensibili.

<h2 id="further-reading">
  Letture ulteriori
</h2>

* [Documentazione sulla sicurezza di Claude Code](/docs/it/security)
* [Hosting dell'Agent SDK](/docs/it/agent-sdk/hosting)
* [Gestione delle autorizzazioni](/docs/it/agent-sdk/permissions)
* [Sandbox runtime](https://github.com/anthropic-experimental/sandbox-runtime)
* [The Lethal Trifecta for AI Agents](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
* [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
* [Docker Security Best Practices](https://docs.docker.com/engine/security/)
* [gVisor Documentation](https://gvisor.dev/docs/)
* [Firecracker Documentation](https://firecracker-microvm.github.io/)
