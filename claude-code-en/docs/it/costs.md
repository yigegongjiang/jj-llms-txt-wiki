> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestisci i costi in modo efficace

> Traccia l'utilizzo dei token, imposta i limiti di spesa del team e riduci i costi di Claude Code con la gestione del contesto, la selezione del modello, le impostazioni del pensiero esteso e gli hook di pre-elaborazione.

Claude Code addebita il consumo di token API. Per i prezzi dei piani di abbonamento (Pro, Max, Team, Enterprise), vedi [claude.com/pricing](https://claude.com/pricing). I costi per sviluppatore variano notevolmente in base alla selezione del modello, alle dimensioni della codebase e ai modelli di utilizzo come l'esecuzione di più istanze o l'automazione.

Nelle distribuzioni aziendali, il costo medio è di circa \$13 per sviluppatore per giorno attivo e \$150-250 per sviluppatore al mese, con costi che rimangono al di sotto di \$30 per giorno attivo per il 90% degli utenti. Per stimare la spesa per il tuo team, inizia con un piccolo gruppo pilota e utilizza gli strumenti di tracciamento di seguito per stabilire una baseline prima di un rollout più ampio.

Questa pagina spiega come [tracciare i tuoi costi](#track-your-costs), [gestire i costi per la tua organizzazione](#manage-costs-for-your-organization) e [ridurre l'utilizzo dei token](#reduce-token-usage).

<h2 id="track-your-costs">
  Traccia i tuoi costi
</h2>

<h3 id="using-the-/usage-command">
  Utilizzo del comando `/usage`
</h3>

<Note>
  Il blocco Session in `/usage` mostra l'utilizzo dei token API ed è destinato agli utenti API. I sottoscrittori di Claude Max e Pro hanno l'utilizzo incluso nel loro abbonamento, quindi la cifra del costo della sessione non è rilevante per scopi di fatturazione. I sottoscrittori vedono barre di utilizzo del piano, statistiche di attività e una suddivisione dell'utilizzo sulla stessa schermata.
</Note>

Il blocco Session in cima a `/usage` mostra statistiche dettagliate sull'utilizzo dei token per la tua sessione attuale. Claude Code calcola la cifra in dollari localmente dai conteggi dei token al prezzo di listino, a meno che non sia in vigore una tabella [`modelPricing`](/docs/it/settings-reference#modelpricing). Un amministratore ne imposta una nelle impostazioni gestite della tua organizzazione in modo che la cifra utilizzi le tue tariffe contrattuali, e mentre una tabella è in vigore la riga `Total cost` riporta la nota `at your organization's configured rates`. La cifra è una stima, quindi per la fatturazione autorevole vedi la pagina Usage nella [Claude Console](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 20s
Total duration (wall): 6h 33m 10s
Total code changes:    0 lines added, 0 lines removed
Usage by model:
   claude-sonnet-4-6:  1.2k input, 5.3k output, 940.0k cache read, 50.0k cache write ($0.55)
```

Questi totali si azzerano quando `/clear` avvia una nuova sessione, quindi il costo totale della sessione successiva ricomincia da \$0. Prima della v2.1.211, continuavano ad accumularsi attraverso `/clear` per la durata del processo Claude Code.

Per una risposta dall'API Claude fatturata al [tasso di residenza dei dati](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing) 1.1×, Claude Code moltiplica il prezzo di listino dei token di quella risposta per 1.1 nella cifra del costo della sessione. Lo stesso totale appare nel [campo costo della riga di stato](/docs/it/statusline#cost-and-duration-tracking), e la cifra moltiplicata conta anche verso [`--max-budget-usd`](/docs/it/cli-reference#cli-flags). Prima della v2.1.239, Claude Code non applicava l'1.1× a quelle risposte, quindi la cifra del costo della sessione era inferiore alla fattura.

<h4 id="prompt-cache-statistics">
  Statistiche della cache dei prompt
</h4>

Dopo la prima risposta API della conversazione principale, Claude Code aggiunge anche una riga `Prompt cache (main)` al blocco Session, riepilogando l'utilizzo della [cache dei prompt](/docs/it/prompt-caching) della sessione: il conteggio delle richieste, la quota di token di input serviti dalla cache, i mancati accessi alla cache e se la cache è calda in questo momento. Richiede Claude Code v2.1.251 o versione successiva.

```text theme={null}
Prompt cache (main):   14 requests · 91% of input tokens from cache · 2 misses (last 6m 10s ago, 310.2k tokens re-cached) · 1 expected rebuild (compaction or tool-result clearing) · warm (1h TTL, last activity 40s ago)
```

I mancati accessi, i rebuild previsti e le parti calde o fredde della riga significano quanto segue:

* **Misses**: richieste che hanno rielaborato contenuti che la cache già conteneva, con l'ora dell'ultimo mancato accesso e quanti token quelle richieste hanno scritto di nuovo nella cache. Claude Code conta una richiesta come mancato accesso quando la richiesta ha rielaborato più del 5% e almeno 2.000 token di ciò che avrebbe potuto leggere dalla cache. [Le azioni che invalidano la cache](/docs/it/prompt-caching#actions-that-invalidate-the-cache) elencano le cause usuali. Quando Claude Code riesce a identificare una probabile causa per l'ultimo mancato accesso, la riga la nomina anche, ad esempio `likely cause: tool definitions changed`. Il testo della probabile causa richiede Claude Code v2.1.260 o versione successiva.
* **Expected rebuilds**: quando Claude Code ha appena riscritto la conversazione, per [compattazione](/docs/it/prompt-caching#compacting-the-conversation) o per cancellazione dei vecchi risultati degli strumenti dal contesto, conta lo stesso tipo di mancato accesso come rebuild previsto. Questa parte appare solo dopo che si è verificato almeno un rebuild previsto.
* **Warm or cold**: se il prefisso memorizzato nella cache è ancora entro la sua [durata della cache](/docs/it/prompt-caching#cache-lifetime), con il TTL in vigore. Quando la cache è fredda, la riga mostra quanto tempo la sessione è rimasta inattiva. Quando nessuna risposta ha segnalato token della cache, la riga termina con `no prompt caching reported by the API` invece.

I conteggi provengono dai campi dei token della cache nelle risposte dell'API, quindi la riga funziona su ogni provider e gateway. Copre solo la conversazione principale, non i subagent. `/clear` la azzera insieme al resto del blocco Session.

Gli script della riga di stato possono leggere gli stessi numeri dall'[oggetto `prompt_cache`](/docs/it/statusline#prompt-cache-fields).

<h4 id="plan-usage-breakdown">
  Suddivisione dell'utilizzo del piano
</h4>

Su un piano Pro, Max, Team o Enterprise, `/usage` mostra anche una suddivisione di ciò che conta rispetto ai limiti del tuo piano:

* **Attribution**: utilizzo recente attribuito a skills, subagent, plugin e singoli server MCP, ciascuno mostrato come percentuale del totale. La quota di un server MCP conta solo le richieste che hanno consumato uno dei risultati dei suoi strumenti. Prima della v2.1.222, dopo una chiamata a un server MCP, Claude Code attribuiva ogni richiesta successiva a quel server, sovrastimando la sua quota.
* **Behavior flags**: comportamenti come contesto lungo o mancati accessi alla cache, contrassegnati quando uno rappresenta il 10% o più dell'utilizzo recente.
* **Loops**: una riga per ciascuno dei [`/loop` o altri task programmati](/docs/it/scheduled-tasks) più pesanti che sono stati eseguiti di recente, ordinati per token totali, con un conteggio del resto. Claude Code segnala con quale frequenza ogni task si attiva, quante volte è stato eseguito, i suoi token totali e per esecuzione, e quando è stato eseguito l'ultima volta. Claude Code identifica una riga dal prompt del task, quindi un loop che interrompi e ricrei rimane una riga. Richiede Claude Code v2.1.242 o versione successiva.

Premi `d` o `w` per passare tra le ultime 24 ore e gli ultimi 7 giorni. Le cifre sono approssimative e calcolate dalla cronologia della sessione locale su questa macchina, quindi l'utilizzo da altri dispositivi o da claude.ai non è incluso.

Nell'[estensione VS Code](/docs/it/vs-code#check-account-and-usage), le quote di attribuzione e i flag di comportamento appaiono nella finestra di dialogo Account & usage con un interruttore Day e Week, senza le righe Loops.

<h4 id="check-your-usage-credits-spend">
  Controlla la tua spesa in crediti di utilizzo
</h4>

`/usage` mostra anche una riga di crediti di utilizzo mentre i [crediti di utilizzo](#add-usage-credits-to-your-subscription) sono attivi. Ciò che la riga mostra dipende dal tuo piano:

* **Pro e Max**: la tua spesa per il mese corrente, misurata rispetto al tuo limite di spesa mensile quando ne hai impostato uno. Quando non hai impostato un limite, la riga mostra `Unlimited` e nessuna cifra di spesa.
* **Team e Enterprise**: la tua spesa per il mese corrente, misurata rispetto a qualsiasi [limite impostato dalla tua organizzazione](#claude-for-teams-and-enterprise) che si applica a te. Un limite che copre l'intera organizzazione non appare nella riga. Quando non hai un limite tuo, la riga mostra la tua spesa senza limite accanto. Mentre i crediti di utilizzo sono disattivati per te, `/usage` non mostra alcuna riga di crediti di utilizzo.

Quando hai un limite di spesa, la riga appare non appena i crediti di utilizzo sono attivi e mostra 0% fino a quando non spendi per la prima volta i crediti di utilizzo. Prima della v2.1.236, `/usage` mostrava la riga solo su piani Pro e Max, e una riga con un limite di spesa rimaneva nascosta fino a quando non avevi speso qualcosa.

<h4 id="when-the-usage-request-fails">
  Quando la richiesta di utilizzo non riesce
</h4>

Quando la richiesta per i limiti del tuo piano non riesce, il più delle volte perché l'endpoint di utilizzo è limitato dalla frequenza, `/usage` mostra le ultime barre di utilizzo caricate su questa macchina negli ultimi 60 minuti, insieme a una nota `Showing last-known usage` che indica quanto tempo fa sono stati recuperati i dati. Premi `r` per riprovare; un nuovo tentativo riuscito sostituisce le ultime barre conosciute con dati freschi. Senza uno snapshot degli ultimi 60 minuti, `/usage` segnala che l'endpoint di utilizzo è limitato dalla frequenza e offre lo stesso collegamento di riprovazione. Prima della v2.1.208, una richiesta limitata dalla frequenza in una sessione che non aveva ancora caricato l'utilizzo mostrava sempre l'errore senza barre.

<h3 id="analyze-your-usage-patterns">
  Analizza i tuoi modelli di utilizzo
</h3>

Esegui [`/insights`](/docs/it/commands#all-commands) per un rapporto su come lavori piuttosto che su quanti token hai utilizzato. Analizza le tue sessioni recenti su questa macchina e scrive un rapporto HTML che copre su cosa lavori, punti di attrito come richieste fraintese o codice buggy, e suggerimenti per utilizzare Claude Code più efficacemente. Una singola esecuzione analizza fino a 200 sessioni che non ha ancora visto e salta quelle molto brevi. Quando le sessioni vengono omesse, l'intestazione del rapporto mostra il conteggio analizzato con il totale tra parentesi, ad esempio `200 sessions (412 total)`.

Claude Code scrive l'ultimo rapporto in `~/.claude/usage-data/report.html` e salva una copia con timestamp di ogni esecuzione nella stessa directory, quindi i rapporti precedenti non vengono sovrascritti. Claude Code elimina i rapporti secondo la stessa pianificazione del resto dei tuoi dati di sessione: all'avvio, rimuove i file più vecchi di [`cleanupPeriodDays`](/docs/it/claude-directory#cleaned-up-automatically), 30 giorni per impostazione predefinita.

Puoi eseguire `/insights` su qualsiasi piano e con qualsiasi provider. L'analisi viene eseguita attraverso lo stesso provider e account delle tue sessioni regolari, e i token contano rispetto al tuo utilizzo del piano o dell'API. Le sessioni da altri dispositivi e da claude.ai non sono incluse.

<h3 id="add-usage-credits-to-your-subscription">
  Aggiungi crediti di utilizzo al tuo abbonamento
</h3>

I [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) ti permettono di continuare a lavorare oltre il limite di utilizzo del tuo piano. Per gestirli, esegui `/usage-credits` dopo aver effettuato l'accesso con il tuo abbonamento claude.ai tramite `/login`; il comando non è disponibile con l'autenticazione tramite chiave API. Nelle organizzazioni Enterprise self-serve, nei trial Enterprise e nelle organizzazioni Enterprise fatturate tramite AWS Marketplace, il comando richiede Claude Code v2.1.248 o versione successiva; le versioni precedenti lo rifiutano con [`Unknown command: /usage-credits`](/docs/it/errors#unknown-command). Ciò che apre dipende dal tuo ruolo:

| Il tuo ruolo                                             | Cosa fa `/usage-credits`                                                                                                                                                                                                                                                   |
| :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sottoscrittore Pro o Max                                 | Apre [**Settings > Usage**](https://claude.ai/settings/usage) su claude.ai nel browser. Nella sua sezione **Usage credits** puoi attivare o disattivare i crediti di utilizzo e controllare il saldo dei crediti, la spesa di questo mese e il tuo limite di spesa mensile |
| Membro Team o Enterprise con accesso alla fatturazione   | Apre le impostazioni di utilizzo della tua organizzazione, [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), nel browser                                                                                                                               |
| Membro Team o Enterprise senza accesso alla fatturazione | Ti chiede di confermare, quindi invia una richiesta agli amministratori della tua organizzazione. Prima della v2.1.211, Claude Code ha inviato la richiesta senza un passaggio di conferma                                                                                 |

Per i membri Team e Enterprise senza accesso alla fatturazione, la conferma appare solo nelle sessioni interattive: in modalità non interattiva con il flag `-p` e da [Remote Control](/docs/it/remote-control), il comando non invia alcuna richiesta e ti dice di eseguirlo in una sessione interattiva.

Se esegui `/usage-credits` di nuovo mentre la tua richiesta precedente è in attesa di un amministratore, Claude Code ti dice che una richiesta è già stata inviata piuttosto che inviarne una duplicata. Dopo che un amministratore ha respinto la tua richiesta, l'esecuzione del comando di nuovo invia una nuova. Prima della v2.1.222, una richiesta respinta bloccava anche le nuove richieste.

Su piani Pro e Max, quando raggiungi il tuo limite di spesa con crediti di utilizzo ancora disponibili, Claude Code ti chiede di aumentare o rimuovere il limite senza lasciare la CLI. Se il server rifiuta la modifica, vedi [Could not update your spend limit](/docs/it/errors#could-not-update-your-spend-limit).

<h2 id="manage-costs-for-your-organization">
  Gestione dei costi per la tua organizzazione
</h2>

I controlli che hai a disposizione dipendono da come la tua organizzazione accede a Claude Code: un piano Claude for Teams o Enterprise, la Claude Console, o un provider cloud. Nei piani Teams e Enterprise, l'utilizzo viene prelevato dall'indennità di posto di ogni membro. Nella Console e presso i provider cloud, l'utilizzo viene fatturato per token alla tua organizzazione. Se la tua organizzazione mescola metodi di accesso, ogni sviluppatore viene misurato in base a quello con cui si è autenticato.

La tabella mappa ogni configurazione a dove vedi la spesa, dove la limiti, e come estrai i numeri per utente. Su un piano individuale Pro o Max non hai un'organizzazione da gestire, quindi traccia il tuo utilizzo di crediti di spesa, incluso [fast mode](/docs/it/fast-mode#see-where-fast-mode-spend-appears), in [Aggiungi crediti di utilizzo al tuo abbonamento](#add-usage-credits-to-your-subscription).

| La tua configurazione                                                                  | Visualizza spesa                                                                                                                       | Limita spesa                             | Rapporto per utente                                                                                                                                                                                                         |
| :------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams o Enterprise](#claude-for-teams-and-enterprise)                      | [Rapporto spesa in analitiche org](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Limiti di spesa nelle impostazioni admin | [CSV rapporto spesa](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) su Enterprise |
| [Claude Console (API)](#claude-console)                                                | [Pagina utilizzo Console](https://platform.claude.com/usage)                                                                           | Limiti di spesa dell'area di lavoro      | [Dashboard Console](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                  |
| [Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry](#cloud-providers) | La tua console di fatturazione cloud                                                                                                   | I controlli di budget del tuo cloud      | [OpenTelemetry](/docs/it/monitoring-usage) o un [gateway LLM](/docs/it/llm-gateway)                                                                                                                                                   |

[L'esportazione OpenTelemetry](/docs/it/monitoring-usage) funziona su ogni configurazione ed è l'unica opzione che trasmette metriche di token e costo per utente nel tuo stack di osservabilità in tempo quasi reale.

<h3 id="report-spend-at-your-contracted-rates">
  Rapporta la spesa alle tue tariffe contrattuali
</h3>

Per impostazione predefinita, Claude Code calcola ogni cifra di costo che mostra agli sviluppatori al prezzo di listino, quindi se la tua organizzazione paga tariffe contrattuali, le cifre in `/usage`, la riga di stato e OpenTelemetry non corrispondono alla tua fattura. Per farle corrispondere, imposta l'impostazione gestita [`modelPricing`](/docs/it/settings-reference#modelpricing) alle tue tariffe. L'impostazione cambia ciò che Claude Code segnala, non ciò che Anthropic addebita. Richiede Claude Code v2.1.242 o successivo.

<Steps>
  <Step title="Prendi le tariffe dal tuo contratto">
    Inserisci le tariffe per milione di token dal tuo contratto. Claude Code non le recupera dalla Claude Console, quindi aggiorna l'impostazione quando il contratto cambia.
  </Step>

  <Step title="Scrivi l'impostazione">
    Imposta `multiplier` sotto 1 per uno sconto fisso o sopra 1 per un ricarico, elenca i quattro tassi per token di ogni modello in `overrides`, o fai entrambi. Un ricarico richiede Claude Code v2.1.271 o successivo. La [voce `modelPricing`](/docs/it/settings-reference#modelpricing) ha la forma e un esempio pronto per incollare.
  </Step>

  <Step title="Distribuiscilo attraverso le impostazioni gestite">
    Consegnalo come [impostazioni gestite](/docs/it/managed-settings): impostazioni gestite dal server, una politica MDM, `managed-settings.json`, o un [helper di politica](/docs/it/managed-settings#compute-the-policy-with-a-helper-program). Claude Code ignora la chiave nelle impostazioni utente, progetto e locali e in `--settings`.
  </Step>
</Steps>

Per confermare che le tariffe sono in vigore, esegui `/usage` in una sessione che ha [ricevuto le impostazioni gestite](/docs/it/managed-settings#read-the-source-in-%2Fstatus): il blocco Session della riga `Total cost` riporta la nota `at your organization's configured rates`. Le cifre sono ancora stime, non una fattura. I prezzi per milione di token nel selettore `/model` rimangono al prezzo di listino.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams e Enterprise
</h3>

Nei piani Claude for Teams e Enterprise, l'utilizzo di Claude Code di ogni membro viene prelevato da un'indennità per posto che si ripristina su una finestra mobile di cinque ore e una finestra settimanale. L'indennità è condivisa con Claude chat e Cowork, e la sua dimensione dipende dal [livello di posto](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) (Standard o Premium). I tuoi controlli si trovano nella console admin di claude.ai, non nella Claude Console.

* **Visualizza spesa**: il [rapporto spesa in analitiche org](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) mostra la spesa stimata per utente e per modello, con esportazione CSV, aggiornato quotidianamente. Il rapporto copre la spesa in crediti di utilizzo e appare una volta che i crediti di utilizzo sono attivati. L'utilizzo all'interno dell'indennità di posto non viene misurato in dollari.
* **Visualizza adozione**: il [dashboard analitiche](https://claude.ai/analytics/claude-code) mostra utenti attivi giornalieri, sessioni e metriche di contributo, con esportazione CSV dei dati di contributo. Vedi [traccia l'utilizzo del team con analitiche](/docs/it/analytics).
* **Limita spesa**: l'indennità di posto è il limite predefinito. Per consentire ai membri di continuare oltre, attiva i [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) e imposta limiti di spesa a livello organizzativo, di gruppo o di singolo membro.
* **Estrai numeri per utente**: nel piano Enterprise, l'[Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) restituisce rapporti di utilizzo e costo per utente su tutte le superfici Claude, incluso Claude Code. Un Primary Owner crea una chiave con l'ambito `read:analytics` su [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). Nel piano Teams, esporta il [CSV rapporto spesa](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), che elenca l'utilizzo dei token e la spesa stimata per utente e per modello.

La [guida al consumo Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) è il riferimento di pianificazione per gli amministratori. Spiega come il consumo differisce tra Claude chat, Claude Code e Cowork, e fornisce punti di partenza in dollari per utente per il budgeting. Stanzia di più per un posto di codifica rispetto a un posto di chat: ogni turno di Claude Code contiene contenuti di file, chiamate di strumenti e ragionamento multi-step, quindi una sessione di debug può consumare più di un giorno di chat.

<h3 id="claude-console">
  Claude Console
</h3>

Le organizzazioni API gestiscono la spesa di Claude Code attraverso [aree di lavoro](https://platform.claude.com/docs/en/build-with-claude/workspaces). Puoi [impostare limiti di spesa dell'area di lavoro](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) sulla spesa totale di Claude Code e [visualizzare rapporti di costo e utilizzo](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) nella Console.

<Note>
  Quando autentichi per la prima volta Claude Code con il tuo account Claude Console, viene creata automaticamente un'area di lavoro chiamata "Claude Code". Questa area di lavoro fornisce il tracciamento e la gestione centralizzati dei costi per tutto l'utilizzo di Claude Code nella tua organizzazione. Non puoi creare chiavi API per questa area di lavoro; è esclusivamente per l'autenticazione e l'utilizzo di Claude Code.

  Per le organizzazioni con limiti di velocità personalizzati, il traffico di Claude Code in questa area di lavoro conta verso i limiti di velocità API complessivi della tua organizzazione. Puoi impostare un [limite di velocità dell'area di lavoro](https://platform.claude.com/docs/en/api/rate-limits#setting-lower-limits-for-workspaces) sulla pagina Limits di questa area di lavoro nella Claude Console per limitare la quota di Claude Code e proteggere altri carichi di lavoro di produzione.
</Note>

Per la segnalazione per utente, il [dashboard Console](https://platform.claude.com/claude-code) mostra la spesa e le righe accettate per membro, e l'[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) restituisce le stesse metriche giornaliere per utente a livello di programmazione con una [chiave API Admin](https://platform.claude.com/settings/admin-keys). Vedi [analitiche per clienti API](/docs/it/analytics#access-analytics-for-api-customers).

<h4 id="rate-limit-recommendations">
  Raccomandazioni sui limiti di velocità
</h4>

Quando configuri Claude Code per i team, considera queste raccomandazioni Token Per Minuto (TPM) e Richieste Per Minuto (RPM) per utente in base alle dimensioni della tua organizzazione:

| Dimensione del team | TPM per utente | RPM per utente |
| ------------------- | -------------- | -------------- |
| 1-5 utenti          | 200k-300k      | 5-7            |
| 5-20 utenti         | 100k-150k      | 2.5-3.5        |
| 20-50 utenti        | 50k-75k        | 1.25-1.75      |
| 50-100 utenti       | 25k-35k        | 0.62-0.87      |
| 100-500 utenti      | 15k-20k        | 0.37-0.47      |
| 500+ utenti         | 10k-15k        | 0.25-0.35      |

Ad esempio, se hai 200 utenti, potresti richiedere 20k TPM per ogni utente, o 4 milioni di TPM totali (200\*20.000 = 4 milioni).

Il TPM per utente diminuisce man mano che le dimensioni del team crescono perché meno utenti tendono a utilizzare Claude Code contemporaneamente nelle organizzazioni più grandi. Questi limiti di velocità si applicano a livello organizzativo, non per singolo utente, il che significa che i singoli utenti possono temporaneamente consumare più della loro quota calcolata quando altri non stanno utilizzando attivamente il servizio.

<Note>
  Se prevedi scenari con utilizzo concorrente insolitamente elevato (come sessioni di formazione dal vivo con grandi gruppi), potresti aver bisogno di allocazioni TPM più elevate per utente.
</Note>

<h3 id="cloud-providers">
  Provider cloud
</h3>

Su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, Claude Code viene fatturato per token al tuo account cloud, e i controlli di spesa si trovano nella console di fatturazione del tuo provider cloud. Claude Code non invia metriche dal tuo cloud ad Anthropic, quindi i [dashboard analitiche](/docs/it/analytics) e l'Claude Code Analytics API non coprono questo utilizzo.

Per l'attribuzione dei costi per utente, hai tre opzioni:

* **OpenTelemetry**: [esporta metriche](/docs/it/monitoring-usage) dalla macchina di ogni sviluppatore nel tuo stack di osservabilità. Questo ti dà conteggi di token per utente, costi e attività di strumenti indipendentemente dal provider.
* **Un gateway di app Claude**: un [gateway di app Claude](/docs/it/claude-apps-gateway) self-hosted fornisce l'attribuzione dell'utilizzo per utente, metriche OTLP con conteggi dei token e [limiti di spesa per utente](/docs/it/claude-apps-gateway-spend-limits) su questi provider.
* **Un gateway LLM**: instrada tutto il traffico di Claude Code attraverso un proxy che traccia la spesa per chiave. Diversi grandi enterprise hanno riferito di utilizzare [LiteLLM](/docs/it/llm-gateway), uno strumento open-source che [traccia la spesa per chiave](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend). Questo progetto non è affiliato ad Anthropic e non è stato sottoposto a audit di sicurezza.

<h3 id="when-a-developer-asks-about-a-limit">
  Quando uno sviluppatore chiede informazioni su un limite
</h3>

Gli sviluppatori di solito portano domande sui limiti al loro amministratore, quindi è utile sapere quale limite hanno raggiunto. Queste situazioni significano cose diverse:

* **"Hai raggiunto il tuo limite di sessione" o "Hai raggiunto il tuo limite settimanale"**: una finestra di utilizzo basata su posto in un piano di abbonamento, condivisa su tutti i modelli, quindi lo sviluppatore non può ripristinare l'accesso cambiando modelli con `/model`. Il messaggio mostra quando la finestra si ripristina. Dopo il messaggio specifico del modello "Hai raggiunto il tuo limite Opus" o "Hai raggiunto il tuo limite Sonnet", cambiare a un modello al di fuori di quella famiglia con `/model` mantiene lo sviluppatore al lavoro. Vedi [errori di limite di utilizzo](/docs/it/errors#youve-hit-your-session-limit). Quello che lo sviluppatore può fare nel frattempo:
  * Esegui `/usage-credits` per richiedere utilizzo oltre l'indennità, se hai i [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) attivati.
  * Su Claude Code v2.1.234 o successivo, [attendi e continua il compito interrotto automaticamente dopo il ripristino](/docs/it/interactive-mode#wait-for-a-usage-limit-to-reset); quella sezione elenca quando Claude Code avvia l'attesa da solo e quando lo sviluppatore lo seleziona da `/rate-limit-options`. Per controllare per la tua flotta se Claude Code avvia quell'attesa da solo, imposta [`autoContinueAtUsageLimit`](/docs/it/settings-reference#autocontinueatusagelimit) nelle [impostazioni gestite](/docs/it/settings#settings-precedence).
* **"Hai raggiunto il tuo limite di spesa individuale", "limite di spesa mensile dell'organizzazione", o "budget condiviso del team"**: la richiesta dello sviluppatore verrebbe fatturata ai crediti di utilizzo, e quei crediti hanno raggiunto un limite di spesa che hai impostato. Per consentire allo sviluppatore di continuare, vai a [**Impostazioni Admin > Utilizzo**](https://claude.ai/admin-settings/usage) e aumenta il limite che il messaggio nomina. Quando il messaggio nomina anche un'ora di ripristino del piano, lo sviluppatore può invece attendere fino ad allora. Vedi il [riferimento degli errori](/docs/it/errors#youve-hit-your-monthly-spend-limit) per ogni variante.
* **Un messaggio di limite di spesa da un [gateway di app Claude](/docs/it/claude-apps-gateway)**: lo sviluppatore ha superato un limite di spesa che hai impostato sul tuo gateway self-hosted, e il gateway blocca le sue richieste fino al ripristino del periodo o fino a quando non aumenti il limite. Vedi [limiti di spesa del gateway](/docs/it/claude-apps-gateway-spend-limits) per i limiti, i programmi di ripristino e il messaggio che lo sviluppatore vede.
* **Un avviso di contesto o auto-compact**: non è un limite di utilizzo. La conversazione è cresciuta vicino alla [finestra auto-compact](/docs/it/model-config#set-the-auto-compact-window) della sessione, la soglia in cui Claude Code riassume la cronologia più vecchia per liberare spazio. Indirizza lo sviluppatore a [riduci l'utilizzo dei token](#reduce-token-usage).
* **Spesa inaspettatamente alta su un piano API o provider cloud**: di solito risale a sessioni lunghe che non sono mai state cancellate o a Opus lasciato come modello predefinito. Le abitudini con il maggiore impatto da condividere sono cancellare tra compiti non correlati e abbinare il modello al lavoro, entrambi coperti in [riduci l'utilizzo dei token](#reduce-token-usage).

<h3 id="agent-team-token-costs">
  Costi dei token del team di agenti
</h3>

I [team di agenti](/docs/it/agent-teams) generano più istanze di Claude Code, ognuna con la propria finestra di contesto. L'utilizzo dei token si ridimensiona con il numero di compagni di squadra attivi e per quanto tempo ognuno viene eseguito.

Per mantenere i costi del team di agenti gestibili:

* Utilizza Sonnet per i compagni di squadra. Bilancia la capacità e il costo per i compiti di coordinamento.
* Mantieni i team piccoli. Ogni compagno di squadra esegue la propria finestra di contesto, quindi l'utilizzo dei token è approssimativamente proporzionale alle dimensioni del team.
* Mantieni i prompt di generazione focalizzati. I compagni di squadra caricano automaticamente CLAUDE.md, i server MCP e le skills, ma tutto nel prompt di generazione si aggiunge al loro contesto dall'inizio.
* Spegni i compagni di squadra quando il loro lavoro è terminato. Ogni compagno di squadra attivo continua a consumare token finché non esce o la sessione termina.
* I team di agenti sono disabilitati per impostazione predefinita. Imposta `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` nel tuo [settings.json](/docs/it/settings) o nell'ambiente per abilitarli. Vedi [abilita i team di agenti](/docs/it/agent-teams#enable-agent-teams).

<h2 id="reduce-token-usage">
  Riduci l'utilizzo dei token
</h2>

I costi dei token si ridimensionano con la dimensione del contesto: più contesto Claude elabora, più token utilizzi. Claude Code ottimizza automaticamente i costi attraverso il [prompt caching](/docs/it/prompt-caching), che riduce i costi per il contenuto ripetuto come i prompt di sistema, e l'auto-compact, che riassume la cronologia della conversazione quando ci si avvicina ai limiti del contesto.

Le seguenti strategie ti aiutano a mantenere il contesto piccolo e ridurre i costi per messaggio.

<h3 id="manage-context-proactively">
  Gestisci il contesto in modo proattivo
</h3>

Utilizza `/usage` per controllare l'utilizzo attuale dei token, o [configura la tua linea di stato](/docs/it/statusline#context-window-usage) per visualizzarla continuamente.

* **Cancella tra i compiti**: Utilizza `/clear` per ricominciare da capo quando passi a lavori non correlati. Il contesto obsoleto spreca token su ogni messaggio successivo. Utilizza `/rename` prima di cancellare in modo da poter trovare facilmente la sessione in seguito, quindi `/resume` per tornare ad essa.
* **Aggiungi istruzioni di compaction personalizzate**: `/compact Focus on code samples and API usage` dice a Claude cosa preservare durante la sintesi. In una sessione nuova, `/compact` stampa `Not enough messages to compact.` perché non c'è ancora cronologia della conversazione da riassumere.

Puoi anche personalizzare il comportamento della compaction nel tuo file CLAUDE.md nella radice del tuo progetto:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Scegli il modello giusto
</h3>

Sonnet gestisce bene la maggior parte dei compiti di codifica e costa meno di Opus. Riserva Opus per decisioni architettoniche complesse o ragionamento multi-step. Utilizza `/model` per cambiare modello a metà sessione, o imposta un valore predefinito in `/config`. Un passaggio a Opus si applica anche ai [subagent che ereditano il modello della tua sessione](/docs/it/model-config#setting-your-model). Per semplici compiti subagent, specifica `model: haiku` nella tua [configurazione subagent](/docs/it/sub-agents#choose-a-model).

<h3 id="reduce-mcp-server-overhead">
  Riduci l'overhead del server MCP
</h3>

Le definizioni degli strumenti MCP sono [rinviate per impostazione predefinita](/docs/it/mcp#scale-with-mcp-tool-search), quindi solo i nomi degli strumenti e le istruzioni del server entrano nel contesto finché Claude non utilizza uno strumento specifico. Esegui `/context` per vedere cosa sta consumando spazio.

* **Preferisci gli strumenti CLI quando disponibili**: Strumenti come `gh`, `aws`, `gcloud` e `sentry-cli` sono ancora più efficienti dal punto di vista del contesto rispetto ai server MCP perché non aggiungono alcun elenco di strumenti per strumento. Claude può eseguire comandi CLI direttamente.
* **Disabilita i server inutilizzati**: Esegui `/mcp` per vedere i server configurati e disabilita quelli che non stai utilizzando attivamente.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Installa plugin di intelligenza del codice per i linguaggi tipizzati
</h3>

I [plugin di intelligenza del codice](/docs/it/plugins/code-intelligence) danno a Claude una navigazione precisa dei simboli invece della ricerca basata su testo, riducendo le letture di file non necessarie quando si esplora codice sconosciuto. Una singola chiamata "vai alla definizione" sostituisce quello che altrimenti potrebbe essere un grep seguito dalla lettura di più file candidati. I server di linguaggio installati segnalano anche gli errori di tipo automaticamente dopo le modifiche, quindi Claude cattura gli errori senza eseguire un compilatore.

<h3 id="offload-processing-to-hooks-and-skills">
  Offload dell'elaborazione agli hook e alle skills
</h3>

Gli [hook](/docs/it/hooks) personalizzati possono pre-elaborare i dati prima che Claude li veda. Invece di Claude che legge un file di log di 10.000 righe per trovare errori, un hook può cercare `ERROR` e restituire solo le righe corrispondenti, riducendo il contesto da decine di migliaia di token a centinaia.

Una [skill](/docs/it/skills) può dare a Claude la conoscenza del dominio in modo che non debba esplorare. Ad esempio, una skill "codebase-overview" potrebbe descrivere l'architettura del tuo progetto, le directory chiave e le convenzioni di denominazione. Quando Claude invoca la skill, ottiene questo contesto immediatamente invece di spendere token leggendo più file per comprendere la struttura.

Ad esempio, questo hook PreToolUse filtra l'output del test per mostrare solo i fallimenti:

<Tabs>
  <Tab title="settings.json">
    Aggiungi questo al tuo [settings.json](/docs/it/settings#where-settings-live) per eseguire l'hook prima di ogni comando Bash:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    L'hook chiama questo script. Crea la cartella con `mkdir -p ~/.claude/hooks`, salva lo script sottostante come `~/.claude/hooks/filter-test-output.sh` e rendilo eseguibile con `chmod +x ~/.claude/hooks/filter-test-output.sh`. Controlla se il comando è un test runner e lo modifica per mostrare solo i fallimenti:

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "$input" | jq --arg filtered "$filtered_cmd" \
        '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "allow", updatedInput: (.tool_input + {command: $filtered})}}'
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

Per verificare la configurazione, esegui `/hooks` e controlla che l'hook appaia sotto PreToolUse. Puoi anche avviare Claude Code con `claude --debug-file ./claude-debug.txt` e chiedere a Claude di eseguire `npm test`. Quando l'hook riscrive il comando, quel file di log contiene una riga `modified tool input keys` che elenca `command` e gli altri campi di input Bash.

<h3 id="move-instructions-from-claude-md-to-skills">
  Sposta le istruzioni da CLAUDE.md alle skills
</h3>

Il tuo file [CLAUDE.md](/docs/it/memory) viene caricato nel contesto all'inizio della sessione. Se contiene istruzioni dettagliate per flussi di lavoro specifici (come revisioni PR o migrazioni di database), quei token sono presenti anche quando stai facendo lavori non correlati. Le [skills](/docs/it/skills) si caricano su richiesta solo quando invocate, quindi spostare le istruzioni specializzate nelle skills mantiene il tuo contesto di base più piccolo. Mira a mantenere CLAUDE.md sotto 200 righe includendo solo gli elementi essenziali.

<h3 id="adjust-extended-thinking">
  Regola il pensiero esteso
</h3>

Il pensiero esteso è abilitato per impostazione predefinita perché migliora significativamente le prestazioni su compiti complessi di pianificazione e ragionamento. I token di pensiero vengono fatturati come token di output, e il budget predefinito può essere decine di migliaia di token per richiesta a seconda del modello.

Per compiti più semplici dove il ragionamento profondo non è necessario, puoi ridurre i costi abbassando il [livello di sforzo](/docs/it/model-config#adjust-effort-level) con `/effort` o in `/model`, o disabilitando il pensiero in `/config`. Non puoi disattivare il pensiero su Opus 5.5 o sui modelli Fable, che utilizzano sempre il pensiero esteso.

Sui modelli con un [budget di pensiero fisso](/docs/it/model-config#adaptive-reasoning-and-fixed-thinking-budgets), puoi anche abbassare il budget impostando la [variabile di ambiente](/docs/it/env-vars) `MAX_THINKING_TOKENS`, ad esempio `MAX_THINKING_TOKENS=8000`. I modelli di ragionamento adattivo ignorano i budget diversi da zero, quindi utilizza i livelli di sforzo lì.

<h3 id="delegate-verbose-operations-to-subagents">
  Delega le operazioni dettagliate ai subagent
</h3>

L'esecuzione di test, il recupero della documentazione o l'elaborazione di file di log possono consumare un contesto significativo. Delega questi ai [subagent](/docs/it/sub-agents#isolate-high-volume-operations) in modo che l'output dettagliato rimanga nel contesto del subagent mentre solo un riassunto ritorna alla tua conversazione principale.

<h3 id="manage-agent-team-costs">
  Gestisci i costi del team di agenti
</h3>

I team di agenti utilizzano approssimativamente 7 volte più token rispetto alle sessioni standard quando i compagni di squadra vengono eseguiti in plan mode, perché ogni compagno di squadra mantiene la propria finestra di contesto ed esegue come un'istanza Claude separata. Mantieni i compiti del team piccoli e autonomi per limitare l'utilizzo dei token per compagno di squadra. Vedi [team di agenti](/docs/it/agent-teams) per i dettagli.

<h3 id="write-specific-prompts">
  Scrivi prompt specifici
</h3>

Richieste vaghe come "migliora questa codebase" attivano una scansione ampia. Richieste specifiche come "aggiungi la convalida dell'input alla funzione di accesso in auth.ts" permettono a Claude di lavorare in modo efficiente con letture di file minime.

<h3 id="work-efficiently-on-complex-tasks">
  Lavora in modo efficiente su compiti complessi
</h3>

Per lavori più lunghi o complessi, queste abitudini aiutano a evitare token sprecati andando nella direzione sbagliata:

* **Utilizza plan mode per compiti complessi**: Premi Shift+Tab per entrare in [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) prima dell'implementazione. Claude esplora la codebase e propone un approccio per la tua approvazione, prevenendo la rielaborazione costosa quando la direzione iniziale è sbagliata.
* **Correggi la rotta presto**: Se Claude inizia a andare nella direzione sbagliata, premi Escape per fermarti immediatamente. Utilizza `/rewind` o doppio tocco Escape per ripristinare la conversazione e il codice a un checkpoint precedente.
* **Fornisci target di verifica**: Includi casi di test, incolla screenshot o definisci l'output previsto nel tuo prompt. Quando Claude può verificare il suo lavoro, cattura i problemi prima che tu debba richiedere correzioni.
* **Testa in modo incrementale**: Scrivi un file, testalo, quindi continua. Questo cattura i problemi presto quando sono economici da risolvere.

<h2 id="background-token-usage">
  Utilizzo dei token in background
</h2>

Claude Code utilizza token per alcune funzionalità in background anche quando inattivo:

* **Sintesi della conversazione**: Processi in background che riassumono le conversazioni precedenti per la funzione `claude --resume`
* **Elaborazione dei comandi**: Alcuni comandi come `/usage` possono generare richieste per controllare lo stato

Questi processi in background consumano una piccola quantità di token (in genere meno di \$0,04 per sessione) anche senza interazione attiva.

Quando i suggerimenti di prompt sono attivati, Claude Code invia anche una breve richiesta al modello che la sessione sta utilizzando dopo che Claude risponde, per [suggerire il prompt successivo](/docs/it/interactive-mode#prompt-suggestions). Quella richiesta riutilizza la cache dei prompt della conversazione, quindi è principalmente letture della cache più alcuni token di output. Claude Code [salta questa operazione quando l'account è vicino o ha raggiunto il limite di utilizzo](/docs/it/interactive-mode#when-claude-code-skips-suggestions). Per interrompere queste richieste, [disattivare i suggerimenti di prompt](/docs/it/interactive-mode#turn-prompt-suggestions-off).

<h2 id="why-usage-climbs-in-a-long-session">
  Perché l'utilizzo aumenta in una sessione lunga
</h2>

Una sessione che è rimasta aperta per ore può utilizzare molto più dei limiti del vostro piano di quanto l'attività suggerisca, solitamente per uno di questi motivi:

* **Contesto lungo**: Claude Code invia la vostra conversazione completa con ogni richiesta, e ogni volta che Claude utilizza gli strumenti invia un'altra richiesta contenente quel batch di risultati degli strumenti. Con [prompt caching](/docs/it/prompt-caching), Claude Code rilegge quella cronologia alla [cached token rate](https://platform.claude.com/docs/en/about-claude/pricing), quindi una domanda di una sola riga in una sessione che è rimasta aperta tutto il giorno consuma comunque utilizzo per l'intera conversazione. Consultate [Manage context proactively](#manage-context-proactively) per i modi di mantenere il vostro contesto piccolo
* **Cache misses**: il vostro primo messaggio dopo una pausa più lunga della [cache lifetime](/docs/it/prompt-caching#cache-lifetime) manca la cache e rielabora il vostro contesto completo. La durata è di un'ora su un abbonamento e scende a cinque minuti una volta che state utilizzando [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans); su una chiave API o provider cloud, è cinque minuti per impostazione predefinita. Per mantenere la durata di un'ora mentre utilizzate usage credits, [scegliete il TTL voi stessi](/docs/it/prompt-caching#choose-the-ttl-yourself). Sui piani Pro e Max, quando riprendete una sessione grande dopo una lunga pausa, Claude Code [offre di riprendere da un riepilogo](/docs/it/sessions#resume-from-a-summary) in modo che le richieste successive non portino la cronologia completa
* **Scheduled tasks**: un [scheduled task](/docs/it/scheduled-tasks) si attiva al suo intervallo anche mentre la sessione è inattiva, inviando il vostro contesto completo ogni volta
* **Cross-session messages**: Claude Code consegna un [message from another of your sessions](/docs/it/cross-session-messaging) come un nuovo turno quando questa sessione rimane inattiva, inviando il vostro contesto completo ogni volta. Per trattenere i messaggi in entrata invece di consegnarli, impostate [`crossSessionInbound`](/docs/it/settings-reference#crosssessioninbound) su `hold`
* **Goal check-ins**: mentre il lavoro in background mantiene un [goal](/docs/it/goal) attivo in attesa, Claude Code [chiede a Claude di controllare quel lavoro](/docs/it/goal#background-work-defers-evaluation) anche quando la sessione rimane inattiva, avviando un nuovo turno che invia il vostro contesto completo. Claude Code avvia al massimo tre check-in inattivi per goal tra i vostri prompt. Prima della v2.1.246, i check-in inattivi erano illimitati. Per disattivare i check-in, impostate [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/it/env-vars) su `0`. I check-in inattivi richiedono Claude Code v2.1.236 o successivo
* **Agent teammates**: ogni [teammate](#agent-team-token-costs) attivo continua a consumare token fino a quando non esce
* **Compaction**: `/compact` legge la conversazione che riassume, quindi [compacting a large context](/docs/it/prompt-caching#compacting-the-conversation) è essa stessa una richiesta grande. Quando volete un nuovo inizio invece della continuità, `/clear` non costa nulla

Su un piano Pro, Max, Team o Enterprise, il breakdown di `/usage` segnala i comportamenti che rappresentano il 10% o più dell'utilizzo recente, come il contesto lungo o i cache misses, ciascuno con un suggerimento per ridurlo.

<h2 id="understanding-changes-in-claude-code-behavior">
  Comprensione dei cambiamenti nel comportamento di Claude Code
</h2>

Claude Code riceve regolarmente aggiornamenti che possono modificare il funzionamento delle funzionalità, inclusa la segnalazione dei costi. Eseguire `claude --version` per verificare la versione corrente.

Per domande sulla fatturazione relative al tuo account specifico, contatta il supporto Anthropic tramite il messenger integrato nel prodotto:

* **Piani di abbonamento** (Pro, Max, Team, Enterprise): accedi a [claude.ai](https://claude.ai), fai clic sulle tue iniziali in basso a sinistra e seleziona **Ottieni aiuto**
* **Fatturazione Console (API)**: accedi a [platform.claude.com](https://platform.claude.com), fai clic sulle tue iniziali e seleziona **Ottieni aiuto**

Consulta [Come ottenere supporto](https://support.claude.com/en/articles/9015913-how-to-get-support) per il flusso completo, incluso chi può raggiungere un agente umano per ogni piano.
