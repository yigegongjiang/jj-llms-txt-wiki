> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ambienti self-hosted

> Esegui sessioni cloud di Claude Code su infrastrutture che controlli: configura un ambiente self-hosted, distribuisci runner e instrada le sessioni al tuo calcolo.

<Note>
  Gli ambienti self-hosted sono in beta pubblica sui piani Team ed Enterprise e sono disabilitati per impostazione predefinita. Vedi [Disponibilità e limitazioni](#availability-and-limitations) per il percorso di abilitazione e cosa è escluso.
</Note>

Un ambiente self-hosted esegue sessioni cloud di Claude Code su infrastrutture che la tua organizzazione gestisce. Una [sessione cloud](/docs/it/claude-code-on-the-web) è qualsiasi sessione che viene eseguita in un luogo diverso dalla macchina dello sviluppatore: gli sviluppatori le avviano da claude.ai, dalle app mobile e desktop, dal terminale con [`claude --cloud`](/docs/it/claude-code-on-the-web#from-terminal-to-cloud), e da [routine pianificate](/docs/it/routines), e per impostazione predefinita vengono eseguite su infrastrutture di Anthropic. In un ambiente self-hosted, quelle stesse sessioni vengono eseguite all'interno della tua rete, e l'esperienza dello sviluppatore è altrimenti la stessa a parte le differenze in [Disponibilità e limitazioni](#availability-and-limitations) e i [problemi noti](/docs/it/self-hosted-environments-deploy#known-issues-and-limitations) della pagina di distribuzione.

Se il tuo team non utilizza sessioni cloud, non c'è nulla da configurare qui: le sessioni in un terminale o IDE vengono sempre eseguite sulla macchina dello sviluppatore. Se desideri eseguire Claude Code sulla tua macchina sempre accesa e controllarla da altri dispositivi, utilizza [Remote Control](/docs/it/remote-control), che è disponibile anche sui piani Pro e Max. Quando sei pronto per la configurazione, vai direttamente alla [guida rapida](/docs/it/self-hosted-environments-quickstart); per rivedere prima il profilo di sicurezza, inizia con [Distribuisci in produzione](/docs/it/self-hosted-environments-deploy). Il resto di questa pagina spiega come funziona il self-hosting e quando sceglierlo.

<h2 id="how-self-hosted-environments-work">
  Come funzionano gli ambienti self-hosted
</h2>

Il self-hosting ha tre parti:

* **Environment**: una destinazione denominata a cui possono essere inviate sessioni cloud. La tua organizzazione crea ambienti nelle impostazioni di amministrazione di claude.ai, e ognuno raggruppa un insieme di runner.
* **Runner**: un programma in esecuzione su host all'interno della tua rete. I runner eseguono le sessioni; l'idea è la stessa di un runner CI self-hosted.
* **Session**: un'attività Claude Code avviata da uno sviluppatore.

Quando uno sviluppatore avvia una sessione cloud, l'interfaccia utente di avvio della sessione mostra un selettore di ambiente che elenca gli ambienti ospitati da Anthropic insieme a quelli creati dalla tua organizzazione. Se scelgono il tuo, il piano di controllo di Anthropic posiziona la sessione nella coda del tuo ambiente, dove un runner la rivendica, clona il repository scelto dallo sviluppatore e avvia un processo Claude Code sul tuo host per eseguirlo. Il runner si autentica al tuo host git con credenziali che configuri; [Configura git](/docs/it/self-hosted-environments-deploy#configure-git) copre le opzioni. Le sessioni raggiungono i tuoi servizi interni dall'interno della tua rete, e il tuo host git allo stesso modo quando è interno; il traffico verso Anthropic, il polling della coda, il flusso di eventi della sessione e l'inferenza del modello, è HTTPS in uscita verso `api.anthropic.com`, con il breve elenco di ulteriori host che le sessioni possono raggiungere in [Requisiti di rete](/docs/it/self-hosted-environments-deploy#network-requirements). Anthropic non si connette mai alla tua rete.

<div style={{maxWidth: "640px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=8056103fc1c5564c7f0ef219d260b99d" className="dark:hidden" alt="Diagramma dell'architettura di un ambiente self-hosted: il confine della tua rete contiene un runner, due processi di sessione Claude Code al suo interno e il tuo host git, con api.anthropic.com all'esterno che contiene coda, flusso di sessione e inferenza. Il runner esegue il polling della coda e raggiunge l'host git, ogni processo di sessione apre le proprie connessioni di flusso, inferenza e git, e ogni connessione è in uscita dalla tua rete, senza nessuna in entrata." width="680" height="320" data-path="images/self-hosted-network-paths.svg" />

    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths-dark.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=fec6aef3b0740d80eaf6d6a7000a2233" className="hidden dark:block" alt="Diagramma dell'architettura di un ambiente self-hosted: il confine della tua rete contiene un runner, due processi di sessione Claude Code al suo interno e il tuo host git, con api.anthropic.com all'esterno che contiene coda, flusso di sessione e inferenza. Il runner esegue il polling della coda e raggiunge l'host git, ogni processo di sessione apre le proprie connessioni di flusso, inferenza e git, e ogni connessione è in uscita dalla tua rete, senza nessuna in entrata." width="680" height="320" data-path="images/self-hosted-network-paths-dark.svg" />
  </Frame>
</div>

I due riquadri Claude Code nel diagramma sono processi di sessione: un runner che esegue due sessioni contemporaneamente, fino alla sua capacità configurata. Un runner serve un [proprietario](#key-concepts) alla volta e si blocca a quel proprietario quando rivendica la sua prima sessione, quindi il codice estratto non si mescola mai tra proprietari; [Ciclo di vita del runner](#runner-lifecycle) copre la regola.

Puoi avviare i runner tu stesso e mantenerli in esecuzione, oppure eseguire l'[orchestrator di autoscaling](/docs/it/self-hosted-environments-configuration#on-demand-runners), un secondo processo che ospiti, che avvia i runner mentre le sessioni si accodano; ogni runner esce da solo quando il suo lavoro finisce. In entrambi i casi, configuri l'ambiente una volta, e appare nel selettore su ogni superficie supportata.

<h2 id="availability-and-limitations">
  Disponibilità e limitazioni
</h2>

Controlla questi punti prima di pianificare un rollout:

* **Piani**: beta pubblica per organizzazioni Team ed Enterprise. Gli ambienti self-hosted sono disabilitati per impostazione predefinita; un [Owner](/docs/it/cloud-environments#organization-shared-environments) attiva **Allow self-hosted environments** sulla [pagina di amministrazione **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), che richiede che [cloud sessions](/docs/it/claude-code-on-the-web) siano abilitate per l'organizzazione.
* **Zero Data Retention**: non disponibile per organizzazioni con [Zero Data Retention](/docs/it/zero-data-retention) abilitato.
* **Inferenza del modello**: le sessioni utilizzano l'API Anthropic, e l'inferenza non può essere instradata attraverso [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/it/third-party-integrations), o un [gateway LLM](/docs/it/llm-gateway).
* **Superfici**: le sessioni avviate da [claude.ai/code](https://claude.ai/code), dalle app mobile e desktop, da [routine pianificate](/docs/it/routines), e dal terminale, con [`claude --cloud`](/docs/it/claude-code-on-the-web#from-terminal-to-cloud) o un [dispatch `--environment`](/docs/it/self-hosted-environments-testing#run-the-test-loop), possono essere eseguite in ambienti self-hosted. Le sessioni [Claude Tag](https://claude.com/docs/claude-tag/overview) possono essere eseguite in esse, ma Claude non può ancora utilizzare [Access bundles](https://claude.com/docs/claude-tag/concepts/glossary#access-bundle) in quelle sessioni. Le sessioni [Claude Security](/docs/it/claude-security) e [Code Review](/docs/it/code-review) non vengono ancora instradate ad esse. Il supporto per quelle due superfici segue separatamente.
* **Repository**: le sessioni estraggono repository da GitHub; vedi [Opzioni di autenticazione GitHub](/docs/it/claude-code-on-the-web#github-authentication-options).
* **Fatturazione**: le sessioni in un ambiente self-hosted consumano l'utilizzo di Claude Code della tua organizzazione allo stesso modo delle sessioni negli ambienti ospitati da Anthropic.

<h2 id="why-self-host">
  Perché fare self-hosting
</h2>

La maggior parte dei team è meglio servita da ambienti ospitati da Anthropic, che non richiedono infrastrutture da eseguire o mantenere. Il self-hosting è per team le cui esigenze di rete, tooling o conformità richiedono di mantenere l'esecuzione della sessione su infrastrutture che controllano. Se è così, pianifica la proprietà operativa che comporta: costruisci e mantieni l'immagine del runner, gestisci la flotta e controlli la sua rete.

In cambio, il self-hosting ti dà accesso alla rete, tooling personalizzato e controllo della conformità:

* **Accesso alla rete**: le sessioni vengono eseguite all'interno della tua rete e possono raggiungere servizi interni, database e registri senza esporli a Internet pubblico
* **Tooling personalizzato**: pre-installa compilatori, SDK e CLI interni nella tua immagine di runner in modo che ogni sessione inizi pronta a compilare
* **Conformità**: gli estratti di repository e gli artefatti di compilazione rimangono su infrastrutture che controlli. Il contenuto della sessione va comunque a `api.anthropic.com` per l'inferenza del modello.

<h2 id="environments-runners-and-sessions">
  Ambienti, runner e sessioni
</h2>

Gli ambienti vengono gestiti sulla pagina **Cloud environments** nelle impostazioni di amministrazione di claude.ai; i runner sono processi che avvii e gestisci sulla tua infrastruttura.

<h3 id="key-concepts">
  Concetti chiave
</h3>

Questi termini appaiono in tutte le pagine self-hosted:

| Termine            | Che cos'è                                                                                                                                                                                                                               |
| :----------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Environment        | Un gruppo denominato dei tuoi runner, creato nelle impostazioni di claude.ai. Le sessioni vengono instradate a un ambiente, non a un singolo runner.                                                                                    |
| Environment secret | La singola credenziale condivisa che i runner utilizzano per autenticarsi e registrarsi con l'ambiente. Mostrata una sola volta alla creazione dell'ambiente, etichettata come **environment key** nell'interfaccia di amministrazione. |
| Runner             | Il processo di lunga durata che distribuisci. Un runner si registra con l'ambiente, riceve un token di runner e esegue il polling per le sessioni.                                                                                      |
| Session            | Un'attività Claude Code, avviata da claude.ai, dall'app mobile o da un'altra superficie Anthropic come una routine pianificata o un agente. Ogni sessione viene eseguita come un processo Claude Code figlio che il runner genera.      |

Nei campi API, nelle rivendicazioni di token e nei nomi delle metriche, l'ambiente appare come `pool`, e l'ID dell'ambiente è il `pool_id`. Il [riferimento](/docs/it/self-hosted-environments-reference) mappa i due nomi, inclusi i nomi di flag `pool` deprecati.

Un runner serve un proprietario alla volta. La prima sessione che un runner raccoglie blocca il runner a quel proprietario della sessione, e il runner quindi esegue sessioni solo per quel proprietario, fino a una capacità configurata. Chi è il proprietario dipende da come è stata avviata la sessione:

* **Sessioni avviate da un utente**: il proprietario è l'account di quell'utente.
* **Sessioni del canale Claude Tag**: Claude le esegue senza alcun account utente allegato, quindi il proprietario è l'[agente Claude Tag](https://claude.com/docs/claude-tag/concepts/glossary#agent-identity) che ha avviato la sessione. Ogni sessione di canale che quell'agente avvia ha lo stesso proprietario, chiunque abbia inviato il messaggio Slack, quindi un runner bloccato ad esso serve sessioni che persone diverse hanno avviato quando lo esegui a una `--capacity` superiore a uno o con un `--drain-grace-sec` positivo. Un runner bloccato a un utente non raccoglie mai questi, e un runner bloccato a un agente Claude Tag non raccoglie mai le sessioni di un utente.

La dimensione minima della flotta è quindi il numero di proprietari che ti aspetti siano attivi contemporaneamente, contando utenti e agenti Claude Tag.

<h3 id="session-lifecycle">
  Ciclo di vita della sessione
</h3>

Quando uno sviluppatore avvia una sessione e seleziona il tuo ambiente, il piano di controllo di Anthropic posiziona la sessione nella coda dell'ambiente. Da lì:

1. Un runner con capacità libera rivendica la sessione e mantiene un lease su di essa.
2. Il runner clona il repository nella sua directory di lavoro e genera un processo Claude Code figlio.
3. Il figlio trasmette gli eventi indietro su HTTPS mentre il runner continua a eseguire il polling; ogni polling aggiorna il lease e funge anche da heartbeat.
4. Se il runner smette di eseguire il polling per circa 60 secondi, il server rimette in coda la sessione per un altro runner.

Il runner assegna a ogni richiesta di polling 10 secondi. Quando una richiesta scade, viene persa o riceve una risposta che il runner non può analizzare, il runner continua a servire le sue sessioni attive e riprova dopo un secondo o due invece di aspettare il prossimo polling programmato. Ad esempio, un proxy di intercettazione che risponde al polling con la sua stessa pagina produce una risposta che il runner non può analizzare. Ogni volta che un'altra richiesta fallisce in uno di questi modi, il runner raddoppia il gap prima del prossimo tentativo, fino a 20 secondi, e accorcia il gap ogni volta che il lease sta per scadere.

<h3 id="runner-lifecycle">
  Ciclo di vita del runner
</h3>

La prima sessione che un runner raccoglie blocca il runner a quel proprietario della sessione, e il runner esegue fino a `--capacity` sessioni concorrenti per quel proprietario. Mentre il runner ha sessioni attive e non ha ricevuto un segnale di arresto o raggiunto il suo tempo di ritiro, il runner continua a rivendicare il lavoro in coda del proprietario bloccato. Quello che succede una volta che finiscono dipende da [`--drain-grace-sec`](/docs/it/self-hosted-environments-reference#runner-cli-flags):

* **Al valore predefinito di `0`**: il runner esce non appena le sue sessioni attive finiscono, senza eseguire il polling per altri, quindi l'orchestrator che lo distribuisci, come Kubernetes, può riavviarlo con un disco fresco, pronto a servire qualsiasi proprietario.
* **A un valore positivo**: il runner continua a eseguire il polling della coda del proprietario bloccato per quel numero di secondi prima di uscire.

Questo ciclo di vita isola il codice estratto di ogni proprietario senza richiedere al runner di eliminare lo stato del disco tra proprietari.

Il modo in cui la tua infrastruttura arresta un runner decide se hai bisogno di `--retire-at`. Un kill che consegna `SIGTERM` non ha bisogno di flag: il runner drena come [Shutdown timing](/docs/it/self-hosted-environments-deploy#shutdown-timing) descrive, o continua a servire le sessioni che già tiene quando imposti [`--defer-shutdown-max-min`](/docs/it/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal). Se la tua infrastruttura invece distrugge gli host a un'ora di parete nota senza un segnale, o con un periodo di grazia troppo breve per drenare, come un limite di durata della sandbox o una reclama di istanza spot, passa `--retire-at <epoch-seconds>` impostato a pochi minuti prima di quel momento. Al momento del ritiro:

1. Il runner smette di accettare nuovo lavoro.
2. Il runner rilascia ogni sessione attiva attraverso lo stesso percorso di rilascio che il flag [`--release-idle-session-min`](/docs/it/self-hosted-environments-reference#runner-cli-flags) utilizza, quindi la sessione riprende su un runner fresco quando l'utente invia il suo prossimo messaggio. Quando il runner rilascia ogni sessione dipende dal suo stato:
   * Il runner rilascia una sessione che è a metà turno non appena quel turno finisce.
   * Quando un turno finisce e lascia attività in background in esecuzione, il runner aspetta fino a 60 secondi per loro, quindi rilascia la sessione anche se sono ancora in esecuzione. Se le attività hanno finito ma il turno successivo che legge i loro risultati non è ancora stato eseguito, il runner mantiene la sessione fino a quando quel turno finisce, e aspetta non più di [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](/docs/it/self-hosted-environments-reference#environment-variable-only-settings) per l'inizio di quel turno.
3. Il runner esce 0 una volta che tutte le sue sessioni sono rilasciate.

Un turno che sopravvive al kill è comunque perso; [Shutdown timing](/docs/it/self-hosted-environments-deploy#shutdown-timing) copre il dimensionamento del margine. Senza `--retire-at`, un kill di host senza segnale è indistinguibile da un crash: il piano di controllo registra un worker perso piuttosto che un rilascio pulito, e la sessione viene rimessa in coda a un altro runner.

<h3 id="network-paths">
  Percorsi di rete
</h3>

Il runner e le sue sessioni effettuano diversi tipi di connessione in uscita, e non è richiesta alcuna connettività in entrata da Anthropic:

* **Piano di controllo**: il runner esegue il polling di `api.anthropic.com` per il lavoro e pubblica gli eventi di progresso della configurazione e di errore, tutto HTTPS in uscita. Il polling funge anche da heartbeat del runner.
* **SCM connector**: l'orchestrator facoltativo [SCM connector](/docs/it/self-hosted-environments-reference#scm-connector-flags) tunnel è l'unica connessione WebSocket.
* **Git**: il runner clona da e spinge verso il tuo host git su HTTPS o SSH, autenticato con credenziali che la tua distribuzione fornisce; [Configura git](/docs/it/self-hosted-environments-deploy#configure-git) copre le opzioni, incluse credenziali coniate per sessione e il [proxy git Anthropic](/docs/it/self-hosted-environments-deploy#use-the-anthropic-git-proxy), che instrada git attraverso `api.anthropic.com` invece.
* **Session child**: il processo Claude Code figlio della sessione mantiene il flusso di eventi della sessione a `api.anthropic.com`, e effettua le proprie chiamate in uscita per l'inferenza del modello e per i comandi git eseguiti durante la sessione. Vedi [Requisiti di rete](/docs/it/self-hosted-environments-deploy#network-requirements) per l'elenco completo dell'uscita. Il [diagramma sopra](#how-self-hosted-environments-work) mostra questi percorsi, a parte l'SCM connector facoltativo.

L'inferenza del modello utilizza l'API Anthropic. Il piano di controllo consegna l'endpoint API a ogni sessione, e la sessione si autentica con un token OAuth emesso da Anthropic e limitato alla sessione, quindi l'inferenza non può essere instradata attraverso [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/it/third-party-integrations), o un [gateway LLM](/docs/it/llm-gateway) negli ambienti self-hosted.

I proxy di uscita aziendali sono supportati. Il runner e l'[orchestrator di autoscaling](/docs/it/self-hosted-environments-configuration#on-demand-runners) facoltativo rispettano il proxy e le variabili di ambiente mTLS descritte in [Configurazione di rete](/docs/it/network-config), come `HTTPS_PROXY` e `NO_PROXY`; impostale nell'ambiente di ogni processo. Le variabili coprono le chiamate del piano di controllo, il WebSocket [SCM connector](/docs/it/self-hosted-environments-reference#scm-connector-flags) dell'orchestrator, e il clone integrato per i remote HTTPS, e le sessioni le ereditano dal runner. Lo streaming della sessione utilizza server-sent events su HTTPS, quindi un proxy nel percorso non deve memorizzare le risposte nel buffer.

Se il tuo proxy richiede anche un'intestazione `Proxy-Authorization`, il runner può aggiungerla a ogni connessione che apre al proxy; vedi [Autentica a un proxy di uscita](/docs/it/self-hosted-environments-deploy#authenticate-to-an-egress-proxy).

<h2 id="what-stays-on-your-infrastructure">
  Cosa rimane sulla tua infrastruttura
</h2>

Gli estratti di repository, gli artefatti di compilazione, i segreti e tutti i file che una sessione crea o modifica rimangono sulle macchine che fornisci. La conversazione stessa, inclusi i prompt, le risposte e i risultati degli strumenti, va a `api.anthropic.com` per l'inferenza del modello, e Anthropic archivia la trascrizione della sessione in modo che tu possa riprendere la sessione da un'altra [superficie supportata](#availability-and-limitations).

Un ambiente self-hosted sposta l'esecuzione della sessione nella tua rete. Il piano di controllo rimane ospitato da Anthropic: l'orchestrazione della sessione, l'accodamento e l'interfaccia di claude.ai continuano a essere eseguiti su infrastrutture di Anthropic.

<h2 id="get-started">
  Inizia
</h2>

Le pagine degli ambienti self-hosted sono organizzate per quello che stai facendo:

* [Guida rapida](/docs/it/self-hosted-environments-quickstart): installa Claude Code, crea un ambiente, avvia un runner e instrada la tua prima sessione
* [Distribuisci in produzione](/docs/it/self-hosted-environments-deploy): hardening della sicurezza, uscita di rete, credenziali git, ricette Kubernetes e Compose, problemi noti e risoluzione dei problemi
* [Personalizza sessioni](/docs/it/self-hosted-environments-configuration): script wrapper per credenziali per sessione, hook del ciclo di vita, runner on-demand, server MCP e autorizzazioni
* [Testa end to end](/docs/it/self-hosted-environments-testing): un test di fumo CI che verifica un'immagine di runner prima di promuoverla
* [Riferimento](/docs/it/self-hosted-environments-reference): ogni flag CLI, variabile di ambiente, metrica e l'endpoint di salute
* [Verifica l'identità della sessione](/docs/it/self-hosted-environments-identity): convalida il token di sessione dai tuoi servizi prima di concedere l'accesso
