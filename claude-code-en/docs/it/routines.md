> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatizzare il lavoro con le routine

> Metti Claude Code in modalità automatica. Definisci routine che vengono eseguite secondo una pianificazione, attivate da chiamate API o che reagiscono agli eventi di GitHub dall'infrastruttura cloud gestita da Anthropic.

<Note>
  Le routine sono in anteprima di ricerca. Il comportamento, i limiti e la superficie dell'API potrebbero cambiare.
</Note>

Una routine è una configurazione salvata di Claude Code: un prompt, uno o più repository e un set di [connectors](/docs/it/mcp), confezionati una volta ed eseguiti automaticamente. Le routine vengono eseguite su infrastruttura cloud gestita da Anthropic, oppure sull'[ambiente self-hosted](/docs/it/self-hosted-environments) della tua organizzazione quando indirizzate lì, quindi continuano a funzionare quando il tuo laptop è chiuso.

Ogni routine può avere uno o più trigger collegati:

* **Scheduled**: eseguire su una cadenza ricorrente come oraria, notturna o settimanale, oppure una volta a un momento futuro specifico
* **API**: attivare su richiesta inviando un POST HTTP a un endpoint per routine con un token bearer
* **GitHub**: eseguire automaticamente in risposta agli eventi del repository come pull request o release

Una singola routine può combinare trigger. Ad esempio, una routine di revisione PR può essere eseguita di notte, attivata da uno script di distribuzione e anche reagire a ogni nuovo PR.

Le routine sono disponibili sui piani Pro, Max, Team ed Enterprise. Creale e gestiscile su [claude.ai/code/routines](https://claude.ai/code/routines), oppure dalla CLI con `/schedule`.

I proprietari di Team ed Enterprise possono disabilitare le routine per tutti i membri con l'interruttore Routines su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Quando disabilitate, le routine esistenti smettono di funzionare e i membri non possono crearne di nuove.

Questa pagina copre la creazione di una routine, la configurazione di ogni tipo di trigger, la gestione delle esecuzioni e come si applicano i limiti di utilizzo.

<h2 id="example-use-cases">
  Esempi di casi d'uso
</h2>

Ogni esempio abbina un tipo di trigger al tipo di lavoro per cui le routine sono adatte: incustodito, ripetibile e legato a un risultato chiaro.

**Manutenzione del backlog.** Un trigger di pianificazione viene eseguito ogni sera feriale rispetto al tuo issue tracker tramite un connector. La routine legge i problemi aperti dall'ultima esecuzione, applica etichette, assegna proprietari in base all'area di codice referenziata e pubblica un riepilogo su Slack in modo che il team inizi la giornata con una coda curata.

**Triage degli avvisi.** Il tuo strumento di monitoraggio chiama l'endpoint API della routine quando viene superata una soglia di errore, passando il corpo dell'avviso come `text`. La routine estrae la traccia dello stack, la correla con i commit recenti nel repository e apre una pull request in bozza con una correzione proposta e un collegamento all'avviso. L'on-call esamina il PR invece di iniziare da un terminale vuoto.

**Revisione del codice personalizzata.** Un trigger GitHub viene eseguito su `pull_request.opened`. La routine applica la tua lista di controllo di revisione del team, lascia commenti inline per problemi di sicurezza, prestazioni e stile e aggiunge un commento di riepilogo in modo che i revisori umani possano concentrarsi sulla progettazione invece di controlli meccanici.

**Verifica della distribuzione.** La tua pipeline CD chiama l'endpoint API della routine dopo ogni distribuzione in produzione. La routine esegue controlli di fumo rispetto alla nuova build, scansiona i log degli errori per regressioni e pubblica un go o no-go nel canale di rilascio prima che la finestra di distribuzione si chiuda.

**Drift della documentazione.** Un trigger di pianificazione viene eseguito settimanalmente. La routine scansiona i PR uniti dall'ultima esecuzione, contrassegna la documentazione che fa riferimento alle API modificate e apre PR di aggiornamento rispetto al repository della documentazione per un editor da rivedere.

**Porting della libreria.** Un trigger GitHub viene eseguito su `pull_request.closed` filtrato per PR uniti in un repository SDK. La routine porta la modifica a un SDK parallelo in un'altra lingua e apre un PR corrispondente, mantenendo le due librerie sincronizzate senza che un umano reimplementi ogni modifica.

<h2 id="create-a-routine">
  Creare una routine
</h2>

Crea una routine dal web su [claude.ai/code/routines](https://claude.ai/code/routines), dall'app Desktop o dalla CLI. Tutte e tre le superfici scrivono nello stesso account cloud, quindi una routine che crei in una appare nelle altre immediatamente. Nell'app Desktop, nella scheda **Code**, fai clic su **Routines** nella barra laterale o nel menu **More** della barra laterale, quindi su **New routine**, e scegli **Cloud**; scegliere **Local** invece crea un [Desktop scheduled task](/docs/it/desktop-scheduled-tasks), che viene eseguito sulla tua macchina piuttosto che nel cloud.

Il modulo di creazione configura il prompt della routine, i repository, l'ambiente, i connector e i trigger.

Le routine vengono eseguite autonomamente come sessioni cloud complete di Claude Code: non c'è un selettore di modalità di autorizzazione e la sessione esegue comandi shell, utilizza [skills](/docs/it/skills) impegnate nel repository clonato e chiama qualsiasi connector incluso, il tutto senza fermarsi per l'approvazione a parte alcune azioni di [artifact](/docs/it/artifacts).

Ciò che una routine può raggiungere è determinato dai repository che selezioni, dall'[ambiente](/docs/it/cloud-environments) accesso di rete e variabili, e dai connector che includi. Delimita ognuno di questi a ciò di cui la routine ha effettivamente bisogno.

Quando la pianificazione della routine o **Run now** avvia un'esecuzione, Claude ripubblica un artifact esistente senza chiedere solo quando tutti questi elementi sono veri:

* Puoi modificare l'artifact e appartiene alla tua organizzazione
* L'artifact non è condiviso pubblicamente e non è condiviso con persone specifiche o la tua organizzazione con l'ultima versione scelta come versione che i visualizzatori vedono
* La pubblicazione contiene solo la pagina, senza file di supporto o altro aggiunto, e non forza una versione più recente
* La pagina non contiene alcun grant che vada oltre la pagina, come [connector calls](/docs/it/artifacts#pull-live-data-with-mcp-connectors)

In ogni altro caso, inclusa la pubblicazione di un nuovo artifact, Claude chiede prima. Quando il lavoro di una routine è mantenere una pagina aggiornata, dagli un artifact che hai già pubblicato.

Le routine appartengono al tuo account claude.ai individuale. Non sono condivise con i compagni di squadra e contano rispetto al limite di esecuzione giornaliero del tuo account. Tutto ciò che una routine fa attraverso la tua identità GitHub connessa o i connector appare come te: i commit e le pull request portano il tuo utente GitHub e i messaggi Slack, i ticket Linear o altre azioni del connector utilizzano i tuoi account collegati per quei servizi.

<h3 id="create-from-the-web">
  Creare dal web
</h3>

<Steps>
  <Step title="Apri il modulo di creazione">
    Visita [claude.ai/code/routines](https://claude.ai/code/routines) e fai clic su **New routine**.
  </Step>

  <Step title="Nomina la routine e scrivi il prompt">
    Dai alla routine un nome descrittivo e scrivi il prompt che Claude esegue ogni volta. Il prompt è la parte più importante: la routine viene eseguita autonomamente, quindi il prompt deve essere autonomo ed esplicito su cosa fare e come appare il successo.

    Quando un trigger si attiva, la sessione riceve il prompt salvato della routine come compito assegnato e lo esegue, piuttosto che trattarlo come contenuto non attendibile arrivato a metà conversazione. Il trigger attesta solo che il prompt è stato archiviato in anticipo da una sessione autorizzata sul tuo account, quindi il prompt attivato non è input utente live e non può agire come approvazione o consenso per le azioni durante l'esecuzione. Il contenuto che la sessione recupera durante l'esecuzione mantiene la sua gestione normale. Prima della v2.1.213, la sessione riceveva lo stesso prompt inquadrato come notifica di background non attendibile e poteva rifiutare di agire su di esso.

    L'input del prompt include un selettore di modello. Claude utilizza il modello selezionato su ogni esecuzione.
  </Step>

  <Step title="Seleziona i repository">
    Aggiungi uno o più repository GitHub per cui Claude possa lavorare. Ogni repository viene clonato all'inizio di un'esecuzione, a partire dal ramo predefinito. Claude crea rami con prefisso `claude/` per le sue modifiche.
  </Step>

  <Step title="Seleziona un ambiente">
    Scegli un [cloud environment](/docs/it/cloud-environments) per la routine. Gli ambienti controllano a cosa ha accesso la sessione cloud:

    * **Network access**: imposta il livello di accesso a Internet disponibile durante ogni esecuzione
    * **Environment variables**: fornisci valori che Claude può utilizzare durante ogni esecuzione. Sono [visibili a chiunque utilizzi l'ambiente](/docs/it/cloud-environments#what-carries-over-from-your-setup), quindi nei piani Pro e Max, archivia le chiavi per le API che Claude chiama durante un'esecuzione come [API credentials](/docs/it/cloud-environments#add-api-credentials) invece. Quella sezione elenca anche le richieste che non ricevono mai una credenziale
    * **Setup script**: installa le dipendenze e gli strumenti di cui la routine ha bisogno. Il risultato è [cached](/docs/it/cloud-environments#environment-caching), quindi lo script non viene rieseguito su ogni sessione

    Un ambiente **Default** è fornito con accesso di rete **Trusted**, che consente solo l'[elenco predefinito](/docs/it/cloud-environments#default-allowed-domains) di registri di pacchetti, API di provider cloud, registri di container e domini di sviluppo comuni attraverso la rete della sessione. I connector che aggiungi alla routine raggiungono i loro servizi attraverso i server di Anthropic, quindi non hanno bisogno di modifiche all'elenco consentito. Se la tua routine ha bisogno di raggiungere i tuoi servizi direttamente o un dominio al di fuori di tale elenco, modifica l'[accesso di rete](/docs/it/cloud-environments#network-access) dell'ambiente prima di eseguire. Per utilizzare un ambiente separato, [creane uno](/docs/it/cloud-environments#configure-your-environment) prima.
  </Step>

  <Step title="Seleziona un trigger">
    Sotto **Select a trigger**, scegli come inizia la routine. Puoi scegliere un tipo di trigger o combinarne diversi.

    <Tabs>
      <Tab title="Schedule">
        Scegli una frequenza preimpostata per un'esecuzione ricorrente, o pianifica un'esecuzione una tantum in un timestamp specifico. Vedi [Add a schedule trigger](#add-a-schedule-trigger) per la gestione del fuso orario, lo sfasamento, gli intervalli cron personalizzati e le esecuzioni una tantum.
      </Tab>

      <Tab title="GitHub event">
        Seleziona il repository, l'evento a cui reagire e filtri facoltativi. Vedi [Add a GitHub trigger](#add-a-github-trigger) per l'elenco completo degli eventi supportati e dei campi di filtro.
      </Tab>

      <Tab title="API">
        Seleziona **API** qui, quindi salva la routine. L'URL e il token vengono generati dopo il salvataggio della routine, poiché dipendono dall'ID della routine. Vedi [Add an API trigger](#add-an-api-trigger) per copiare l'URL e generare un token.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Rivedi i connector">
    Sotto **Connectors** in fondo al modulo, tutti i tuoi [MCP connectors](/docs/it/mcp) connessi sono inclusi per impostazione predefinita. Rimuovi quelli che la routine non necessita: Claude può utilizzare ogni strumento da un connector incluso, incluse le scritture, senza chiedere il permesso durante un'esecuzione.
  </Step>

  <Step title="Crea la routine">
    Fai clic su **Create**. La routine appare nell'elenco e viene eseguita la prossima volta che uno dei suoi trigger corrisponde. Per avviare un'esecuzione immediatamente, fai clic su **Run now** nella pagina dei dettagli della routine.

    Ogni esecuzione crea una nuova sessione insieme alle tue altre sessioni, dove puoi vedere cosa ha fatto Claude, rivedere le modifiche e creare una pull request.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  Creare dalla CLI
</h3>

Esegui `/schedule` in qualsiasi sessione per creare una routine pianificata in modo conversazionale. Puoi anche passare una descrizione direttamente, per una routine ricorrente come `/schedule daily PR review at 9am` o una una tantum come `/schedule clean up feature flag in one week`. Claude esamina le stesse informazioni che il modulo web raccoglie, quindi salva la routine nel tuo account. Il comando è disponibile anche con l'alias `/routines`.

Una partenza riuscita assomiglia a una conversazione: Claude pone domande di follow-up sulla pianificazione, i repository e il prompt prima di salvare. Se Claude invece risponde che devi autenticarti o che non riesce a connettersi al tuo account remoto claude.ai, nessuna routine è stata creata; vedi [Troubleshooting](#troubleshooting).

`/schedule` nella CLI crea routine pianificate. Per aggiungere un trigger API, modifica la routine sul web su [claude.ai/code/routines](https://claude.ai/code/routines). Puoi aggiungere un [GitHub trigger](#add-a-github-trigger) dal web o dalla CLI. Il percorso CLI richiede Claude Code v2.1.225 o successivo.

Una routine senza trigger di pianificazione, come una avviata solo da chiamate API o eventi GitHub, non ha un'ora di esecuzione successiva, e la CLI non mostra nulla quando Claude la salva o l'aggiorna. Prima della v2.1.211, la CLI segnalava un'ora di esecuzione successiva nell'anno 1 per queste routine.

<h2 id="configure-triggers">
  Configurare i trigger
</h2>

Una routine inizia quando uno dei suoi trigger corrisponde. Puoi allegare qualsiasi combinazione di trigger di pianificazione, API e GitHub alla stessa routine e aggiungerli o rimuoverli in qualsiasi momento dalla sezione **Select a trigger** del modulo di modifica della routine.

<h3 id="add-a-schedule-trigger">
  Aggiungi un trigger di pianificazione
</h3>

Un trigger di pianificazione esegue la routine su una cadenza ricorrente, o una sola volta a un momento futuro specifico. Scegli una frequenza preimpostata nella sezione **Select a trigger**: oraria, giornaliera, giorni feriali o settimanale. I tempi vengono inseriti nella tua zona locale e convertiti automaticamente, quindi la routine viene eseguita a quell'ora di parete indipendentemente da dove si trova l'infrastruttura cloud.

Le esecuzioni possono iniziare alcuni minuti dopo l'ora pianificata a causa dello sfasamento. L'offset è coerente per ogni routine.

Per un intervallo personalizzato come ogni due ore o il primo di ogni mese, scegli il preset più vicino nel modulo, quindi esegui `/schedule update` nella CLI per impostare un'espressione cron specifica. L'intervallo minimo è un'ora; le espressioni che vengono eseguite più frequentemente vengono rifiutate.

<h4 id="schedule-a-one-off-run">
  Pianifica un'esecuzione una tantum
</h4>

Una pianificazione una tantum attiva la routine una sola volta a un timestamp specifico. Usala per ricordarti più tardi nella settimana, per aprire una PR di pulizia dopo che un rollout finisce, o per avviare un'attività di follow-up quando un cambiamento upstream arriva. Dopo che la routine si attiva, si disabilita automaticamente e l'interfaccia utente web la contrassegna come **Ran**. Per eseguirla di nuovo, modifica la routine e imposta un nuovo orario una tantum.

Crea un'esecuzione una tantum dalla CLI descrivendo l'ora in linguaggio naturale. Claude risolve la frase rispetto all'ora attuale e conferma il timestamp assoluto prima di salvare.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

La stessa conversione da locale a UTC come per le pianificazioni ricorrenti si applica ai timestamp una tantum.

Le esecuzioni una tantum non contano rispetto al limite di esecuzione della routine giornaliera. Vedi [Usage and limits](#usage-and-limits) per i dettagli.

<h3 id="add-an-api-trigger">
  Aggiungi un trigger API
</h3>

Un trigger API fornisce a una routine un endpoint HTTP dedicato. POSTing all'endpoint con il token bearer della routine avvia una nuova sessione e restituisce un URL di sessione. Usalo per collegare Claude Code nei sistemi di avviso, pipeline di distribuzione, strumenti interni o ovunque tu possa fare una richiesta HTTP autenticata.

I trigger API vengono aggiunti a una routine esistente dal web. La CLI attualmente non può creare o revocare token.

<Steps>
  <Step title="Apri la routine per la modifica">
    Vai a [claude.ai/code/routines](https://claude.ai/code/routines), fai clic sulla routine che desideri attivare tramite API, quindi apri il menu accanto al nome della routine e seleziona **Edit**.
  </Step>

  <Step title="Aggiungi un trigger API">
    Scorri fino alla sezione **Select a trigger** sotto il box **Instructions**, fai clic su **Add another trigger** e scegli **API**.
  </Step>

  <Step title="Copia l'URL e genera un token">
    Il modale mostra l'URL per questa routine insieme a un comando curl di esempio. Copia l'URL, quindi fai clic su **Generate token** e copia il token immediatamente. Il token viene mostrato una sola volta e non può essere recuperato in seguito, quindi archivialo in un luogo sicuro come l'archivio segreto del tuo strumento di avviso.
  </Step>

  <Step title="Chiama l'endpoint">
    Invia il token nell'intestazione `Authorization: Bearer` quando POST all'URL. La sezione [Trigger a routine](#trigger-a-routine) di seguito mostra un esempio completo.
  </Step>
</Steps>

Ogni routine ha il suo token, limitato all'attivazione di quella routine sola. Per ruotarlo o revocarlo, torna allo stesso modale e fai clic su **Regenerate** o **Revoke**.

<h4 id="trigger-a-routine">
  Attiva una routine
</h4>

Invia una richiesta POST all'endpoint `/fire` con il token bearer nell'intestazione `Authorization`. Il corpo della richiesta accetta un campo `text` facoltativo per il contesto specifico dell'esecuzione come un corpo di avviso o un log in errore, passato alla routine insieme al suo prompt salvato. Il valore è testo libero e non viene analizzato: se invii JSON o un altro payload strutturato, la routine lo riceve come stringa letterale.

Il valore `text` non raggiunge la routine come un messaggio nudo. Arriva avvolto in un blocco `<routine-fire-payload>` che lo etichetta come dati non attendibili e dice a Claude di non seguire le istruzioni al suo interno a meno che il prompt della routine stessa non lo dica. Lo stesso avvolgimento si applica al testo fornito con **Run now** nell'interfaccia utente web.

Ciò significa che il prompt salvato di una routine deve acconsentire ad agire sul testo di fuoco: scrivi il prompt per fare riferimento al payload in modo esplicito, ad esempio "Investigate the alert described in the routine-fire-payload block", oppure la routine tratta il testo come contesto inerte. Chiunque detenga il token bearer può inviare `text`, quindi il wrapper fa sì che il testo di fuoco da un token trapelato arrivi etichettato come dati non attendibili piuttosto che come istruzioni dirette alla tua routine.

L'esempio seguente attiva una routine da una shell. L'ID della routine e il token mostrati sono segnaposti: sostituiscili con l'URL e il token che hai copiato quando hai [aggiunto il trigger API](#add-an-api-trigger), altrimenti la richiesta fallisce con un errore di autenticazione `401`:

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

Una richiesta riuscita restituisce un corpo JSON con il nuovo ID di sessione e l'URL:

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

Apri l'URL della sessione in un browser per guardare l'esecuzione in tempo reale, rivedere le modifiche o continuare la conversazione manualmente.

<Warning>
  L'endpoint `/fire` viene fornito con l'intestazione beta `experimental-cc-routine-2026-04-01`. Le forme di richiesta e risposta, i limiti di velocità e la semantica dei token potrebbero cambiare mentre la funzione è in anteprima di ricerca. Le modifiche di rilievo vengono fornite dietro nuove versioni di intestazione beta con data, e le due versioni di intestazione precedenti più recenti continuano a funzionare in modo che i chiamanti abbiano tempo per migrare.
</Warning>

<h4 id="api-reference">
  Riferimento API
</h4>

Per il riferimento API completo, incluse tutte le risposte di errore, le regole di convalida e i limiti dei campi, vedi [Trigger a routine via API](https://platform.claude.com/docs/en/api/claude-code/routines-fire) nella documentazione della piattaforma Claude.

L'endpoint `/fire` è disponibile solo per gli utenti di claude.ai e non fa parte della superficie dell'API della piattaforma Claude.

<h3 id="add-a-github-trigger">
  Aggiungi un trigger GitHub
</h3>

Un trigger GitHub avvia una nuova sessione automaticamente quando si verifica un evento corrispondente su un repository connesso. Claude Code non riutilizza le sessioni tra gli eventi, quindi due aggiornamenti PR producono due sessioni indipendenti.

<Note>
  Durante l'anteprima di ricerca, gli eventi webhook di GitHub sono soggetti a limiti orari per routine e per account. Gli eventi oltre il limite vengono eliminati fino al ripristino della finestra. Vedi i tuoi limiti attuali su [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

L'app Claude GitHub deve essere installata sul repository a cui desideri sottoscriverti, indipendentemente dalla superficie da cui configuri il trigger.

* Configura i trigger GitHub dall'interfaccia utente web, che ti chiede di installare l'app quando manca. Segui i passaggi seguenti per configurarne uno sul web.
* Dalla CLI, installa l'app dalla [pagina GitHub App](https://github.com/apps/claude) per prima, quindi chiedi a Claude di allegare un trigger GitHub a una routine esistente, ad esempio `/schedule add a GitHub trigger to my nightly review for pull requests opened in acme/webapp`. Il percorso CLI richiede Claude Code v2.1.225 o successivo. Quando Claude aggiunge il trigger, risponde con un link alla routine che il trigger attiva.

<Steps>
  <Step title="Apri la routine per la modifica">
    Vai a [claude.ai/code/routines](https://claude.ai/code/routines), fai clic sulla routine, quindi apri il menu accanto al nome della routine e seleziona **Edit**.
  </Step>

  <Step title="Aggiungi un trigger di evento GitHub">
    Scorri fino alla sezione **Select a trigger**, fai clic su **Add another trigger** e scegli **GitHub event**.

    <Note>
      L'esecuzione di `/web-setup` nella CLI concede l'accesso al repository per la clonazione, ma non installa l'app Claude GitHub e non abilita la consegna del webhook.
    </Note>
  </Step>

  <Step title="Configura il trigger">
    Seleziona il repository, scegli un evento dall'elenco [supported events](#supported-events) e facoltativamente aggiungi filtri. Salva il trigger.
  </Step>
</Steps>

<h4 id="supported-events">
  Eventi supportati
</h4>

I trigger GitHub possono sottoscriversi a una delle seguenti categorie di eventi. All'interno di ogni categoria puoi scegliere un'azione specifica, come `pull_request.opened`, o reagire a tutte le azioni nella categoria.

| Event        | Triggers when                                                                             |
| :----------- | :---------------------------------------------------------------------------------------- |
| Pull request | Un PR viene aperto, chiuso, assegnato, etichettato, sincronizzato o altrimenti aggiornato |
| Release      | Un rilascio viene creato, pubblicato, modificato o eliminato                              |

<h4 id="filter-pull-requests">
  Filtra le pull request
</h4>

Usa i filtri per restringere quali pull request avviano una nuova sessione. Tutte le condizioni di filtro devono corrispondere affinché la routine si attivi. I campi di filtro disponibili sono:

| Filter      | Matches                               |
| :---------- | :------------------------------------ |
| Author      | Nome utente GitHub dell'autore del PR |
| Title       | Testo del titolo del PR               |
| Body        | Testo della descrizione del PR        |
| Base branch | Ramo a cui il PR è destinato          |
| Head branch | Ramo da cui proviene il PR            |
| Labels      | Etichette applicate al PR             |
| Is draft    | Se il PR è in stato di bozza          |
| Is merged   | Se il PR è stato unito                |

Ogni filtro abbina un campo a un operatore: equals, contains, starts with, is one of, is not one of o matches regex.

L'operatore `matches regex` testa l'intero valore del campo, non una sottostringa al suo interno. Per abbinare qualsiasi titolo contenente `hotfix`, scrivi `.*hotfix.*`. Senza il `.*` circostante, il filtro corrisponde solo a un titolo che è esattamente `hotfix` senza nulla prima o dopo. Per l'abbinamento di sottostringa letterale senza sintassi regex, usa l'operatore `contains`.

Alcuni esempi di combinazioni di filtri:

* **Auth module review**: base branch `main`, head branch contains `auth-provider`. Invia qualsiasi PR che tocca l'autenticazione a un revisore focalizzato.
* **Ready-for-review only**: is draft is `false`. Salta le bozze in modo che la routine venga eseguita solo quando il PR è pronto per la revisione.
* **Label-gated backport**: labels include `needs-backport`. Attiva una routine di port-to-another-branch solo quando un manutentore etichetta il PR.

<h2 id="manage-routines">
  Gestire le routine
</h2>

Fare clic su una routine nell'elenco per aprire la relativa pagina di dettaglio. La pagina di dettaglio mostra i repository della routine, i connettori, il prompt, la pianificazione, i token API, i trigger GitHub e un elenco delle esecuzioni passate.

<h3 id="view-and-interact-with-runs">
  Visualizzare e interagire con le esecuzioni
</h3>

Fare clic su qualsiasi esecuzione per aprirla come sessione completa. Da lì è possibile vedere cosa ha fatto Claude, esaminare le modifiche, creare una pull request o continuare la conversazione. Ogni sessione di esecuzione funziona come qualsiasi altra sessione: utilizzare il menu a discesa accanto al titolo della sessione per rinominare, archiviare o eliminare.

<Note>
  Uno stato verde nell'elenco delle esecuzioni significa che la sessione è stata avviata e chiusa senza un errore infrastrutturale. Non significa che l'attività nel prompt sia riuscita. Aprire l'esecuzione per leggere la trascrizione e confermare cosa ha effettivamente fatto Claude. Le richieste di rete bloccate, gli strumenti del connettore mancanti e i guasti a livello di attività vengono visualizzati lì piuttosto che nell'indicatore di stato.
</Note>

<h3 id="edit-and-control-routines">
  Modificare e controllare le routine
</h3>

Dalla pagina di dettaglio della routine è possibile:

* Fare clic su **Run now** per avviare un'esecuzione immediatamente senza attendere l'orario pianificato successivo. È possibile fornire facoltativamente un testo specifico dell'esecuzione, che raggiunge la routine nello stesso modo del campo `text` del trigger API.
* Utilizzare l'interruttore on/off nella parte superiore della pagina per mettere in pausa o riprendere la pianificazione. Le routine in pausa mantengono la loro configurazione ma non vengono eseguite finché non le abilitate di nuovo.
* Aprire il menu accanto al nome della routine e selezionare **Edit** per modificare il nome, il prompt, i repository, l'ambiente, i connettori o uno qualsiasi dei trigger della routine. La sezione **Select a trigger** è dove aggiungere o rimuovere pianificazioni, token API e trigger di eventi GitHub.
* Aprire lo stesso menu e selezionare **Delete** per eliminare la routine.

<h3 id="manage-routines-from-the-cli">
  Gestire le routine dalla CLI
</h3>

La CLI supporta la gestione delle routine esistenti. Eseguire `/schedule list` per visualizzare tutte le routine, `/schedule update` per modificarne una, o `/schedule run` per attivarla immediatamente.

È inoltre possibile chiedere informazioni sulla cronologia delle esecuzioni di una routine, ad esempio `/schedule why did my nightly review do nothing this morning?`. Claude elenca le esecuzioni recenti della routine con il loro stato e un collegamento per [aprire ogni esecuzione sul web](#view-and-interact-with-runs), e legge il log di un'esecuzione per spiegare cosa è accaduto, inclusi gli errori degli strumenti, i rifiuti di autorizzazione e il risultato finale. Richiede Claude Code v2.1.227 o versione successiva.

<h3 id="repositories-and-branch-permissions">
  Repository e autorizzazioni dei rami
</h3>

Le routine necessitano dell'accesso a GitHub per clonare i repository. Quando si crea una routine dalla CLI con `/schedule`, Claude verifica se l'account ha accesso a GitHub per il repository da cui è stata eseguita e, se non lo ha, aggiunge una nota di configurazione che indica come concedere l'accesso. Vedere [Opzioni di autenticazione GitHub](/docs/it/claude-code-on-the-web#github-authentication-options) per i due modi per concedere l'accesso.

Se la connessione GitHub è mancante o scaduta quando un'esecuzione è dovuta, la routine salta le esecuzioni finché non si riconnette, fino a 72 ore. Riconnettere GitHub entro quella finestra e la routine riprende automaticamente. Dopo 72 ore senza una connessione, la routine si disattiva e la si riattiva dopo aver riconnesso GitHub.

Ogni repository aggiunto viene clonato ad ogni esecuzione. Claude inizia dal ramo predefinito del repository a meno che il prompt non specifichi diversamente.

Claude spinge il suo lavoro su rami con prefisso `claude/`, che sono sempre accettati. Quando il prompt indirizza Claude a eseguire il push su un altro ramo, Claude Code verifica prima il push e lo rifiuta se una qualsiasi delle seguenti condizioni è vera:

* Il ramo è protetto su GitHub
* Qualcun altro ha una pull request aperta da quel ramo
* Il ramo contiene commit creati da qualcuno diverso da voi

<h3 id="connectors">
  Connettori
</h3>

Le routine possono utilizzare i connettori MCP connessi per leggere e scrivere su servizi esterni durante ogni esecuzione. Ad esempio, una routine che triage le richieste di supporto potrebbe leggere da un canale Slack e creare problemi in Linear.

I connettori sono le [integrazioni di claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) sul vostro account. I server MCP aggiunti localmente nella CLI con `claude mcp add` sono archiviati sulla vostra macchina piuttosto che sul vostro account claude.ai, quindi non vengono visualizzati nell'elenco dei connettori. Per utilizzare uno di questi server in una routine, aggiungetelo come connettore su [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Per una routine con un repository, è possibile invece dichiararlo in un [`.mcp.json`](/docs/it/mcp#project-scope) sottoposto a commit in modo che faccia parte del repository clonato.

Quando si crea una routine, tutti i connettori attualmente connessi vengono inclusi per impostazione predefinita. Rimuovere quelli non necessari per limitare a quali strumenti Claude ha accesso durante l'esecuzione. È inoltre possibile aggiungere connettori direttamente dal modulo della routine.

Per gestire o aggiungere connettori al di fuori del modulo della routine, visitare [claude.ai/customize/connectors](https://claude.ai/customize/connectors) o utilizzare `/schedule update` nella CLI.

<h3 id="environments-and-network-access">
  Ambienti e accesso alla rete
</h3>

Ogni routine utilizza un [ambiente cloud](/docs/it/cloud-environments) che controlla l'accesso alla rete, le variabili di ambiente e gli script di configurazione. La routine eredita la politica di rete dell'ambiente ad ogni esecuzione.

L'ambiente **Default** utilizza l'accesso alla rete **Trusted**, che consente solo l'[elenco di autorizzazione predefinito](/docs/it/cloud-environments#default-allowed-domains) attraverso la rete della sessione. Le richieste su quel percorso agli host al di fuori dell'elenco di autorizzazione non riescono con `403` e `x-deny-reason: host_not_allowed`. Il traffico del connettore MCP viene instradato attraverso i server di Anthropic piuttosto che su quel percorso, quindi i connettori aggiunti alla routine funzionano senza aggiungere i loro host ai **Allowed domains**. Rimuovere eventuali connettori non necessari in [Connettori](#connectors).

Per consentire domini aggiuntivi su uno dei vostri ambienti, seguire questi passaggi. Un [ambiente condiviso dall'organizzazione](/docs/it/cloud-environments#organization-shared-environments) si apre in sola lettura qui, quindi un Proprietario cambia il suo accesso alla rete dalla pagina **Cloud environments** nelle [impostazioni di amministrazione](https://claude.ai/admin-settings).

<Steps>
  <Step title="Aprire la routine per la modifica">
    Sulla pagina di dettaglio della routine, aprire il menu accanto al nome della routine e selezionare **Edit**.
  </Step>

  <Step title="Aprire il selettore dell'ambiente">
    Sotto la casella **Instructions**, selezionare l'icona cloud che mostra il nome dell'ambiente, ad esempio **Default**.
  </Step>

  <Step title="Aprire le impostazioni dell'ambiente">
    Passare il mouse sull'ambiente nell'elenco e fare clic sull'icona delle impostazioni che appare a destra.
  </Step>

  <Step title="Modificare il livello di accesso alla rete">
    Nella finestra di dialogo **Update cloud environment**, modificare **Network access** in **Custom** e immettere i domini in **Allowed domains**. Selezionare **Also include default list of common package managers** per mantenere l'[elenco di autorizzazione predefinito](/docs/it/cloud-environments#default-allowed-domains) insieme ai domini personalizzati. Selezionare invece **Full** per un accesso senza restrizioni.
  </Step>

  <Step title="Salvare">
    Fare clic su **Save changes**. La nuova politica si applica dalla prossima esecuzione.
  </Step>
</Steps>

Vedere [Network access](/docs/it/cloud-environments#network-access) per i dettagli sui livelli di accesso e l'elenco di autorizzazione predefinito.

<h2 id="usage-and-limits">
  Utilizzo e limiti
</h2>

Le routine riducono l'utilizzo dell'abbonamento allo stesso modo delle sessioni interattive. Oltre ai limiti di abbonamento standard, le routine hanno un limite giornaliero su quante esecuzioni possono iniziare per account. Vedi il tuo consumo attuale e le esecuzioni di routine giornaliere rimanenti su [claude.ai/code/routines](https://claude.ai/code/routines) o [claude.ai/settings/usage](https://claude.ai/settings/usage).

Quando una routine raggiunge il limite giornaliero o il limite di utilizzo dell'abbonamento, le organizzazioni con crediti di utilizzo attivati possono continuare a eseguire routine su eccedenza misurata. Senza crediti di utilizzo, le esecuzioni aggiuntive vengono rifiutate fino al ripristino della finestra. Attiva i crediti di utilizzo su [claude.ai/settings/usage](https://claude.ai/settings/usage). Nei piani Team ed Enterprise, un amministratore li attiva per l'organizzazione su [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

Le esecuzioni una tantum non contano rispetto al limite giornaliero di routine. Riducono l'utilizzo regolare dell'abbonamento come qualsiasi altra sessione.

Mentre l'abbonamento è in pausa, le routine vengono messe in sospeso e non vengono eseguite. Una volta che l'abbonamento è di nuovo attivo, riattivale.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="schedule-returns-unknown-command">
  `/schedule` restituisce "Unknown command"
</h3>

La CLI nasconde `/schedule` quando uno dei suoi requisiti non è soddisfatto: il menu dei comandi mostra `No commands match "/schedule"` mentre digitate, e l'invio restituisce `Unknown command: /schedule`, tranne nei casi sottostanti che indicano una risposta diversa.

La causa è solitamente una delle seguenti:

* Siete autenticati con una chiave API Console, un [profilo Anthropic o credenziale di federazione](/docs/it/authentication#anthropic-profiles-and-federation-credentials), o un provider cloud come Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry. `/schedule` richiede un accesso con abbonamento claude.ai. Con una chiave API Console o un profilo, e il recupero dei flag di funzionalità abilitato, l'invio di `/schedule` mostra invece `/schedule is available with Claude for Enterprise — ask your admin about migrating from API-key access`. Con un accesso da provider cloud, vedete ancora `Unknown command: /schedule`. Se `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN` è impostato nella vostra shell, o `apiKeyHelper` è impostato in `settings.json`, rimuovetelo prima, poiché questi hanno la precedenza su un accesso claude.ai. Un profilo o credenziale di federazione ha la precedenza anche, quindi disattivate anche quello
* Siete completamente disconnessi, senza chiave API o altra credenziale. Con il recupero dei flag di funzionalità abilitato, l'invio di `/schedule` mostra `/schedule requires a claude.ai subscription. Run /login to sign in with your claude.ai account.` Prima della v2.1.268, una sessione disconnessa mostrava lo stesso messaggio Claude for Enterprise di una chiave API Console
* Siete all'interno di una sessione cloud, dove l'invio di `/schedule` risponde che il comando non è disponibile in quell'ambiente. Gestite le routine dall'[interfaccia web](https://claude.ai/code/routines) invece
* La politica della vostra organizzazione disabilita [le sessioni cloud](/docs/it/claude-code-on-the-web), su cui le routine vengono eseguite. In questo caso, l'invio di `/schedule` risponde [`Cloud sessions are disabled by your organization's policy`](/docs/it/errors#cloud-sessions-are-disabled-by-your-organizations-policy) invece. Prima della v2.1.268, restituiva `Unknown command: /schedule`
* Un Owner ha [disattivato le routine](#routines-are-disabled-by-your-organizations-policy) per la vostra organizzazione Team o Enterprise. Prima della v2.1.227, il comando appariva ancora in questo caso, e claude.ai rifiutava la routine quando Claude tentava di crearla o eseguirla

A meno che la politica della vostra organizzazione non disabiliti le routine o le sessioni cloud, potete creare e gestire le routine su [claude.ai/code/routines](https://claude.ai/code/routines) indipendentemente da come è configurata la CLI.

<h3 id="routines-are-disabled-by-your-organizations-policy">
  "Routines are disabled by your organization's policy"
</h3>

Un Owner nella vostra organizzazione Team o Enterprise ha probabilmente disattivato l'interruttore **Routines** su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Su Claude Code v2.1.227 o successivo, lo stesso interruttore nasconde anche `/schedule` nella CLI. Questa è un'impostazione dell'organizzazione lato server, quindi non può essere ignorata dalla vostra configurazione locale. Chiedete a un Owner di abilitare le routine per la vostra organizzazione.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [`/loop` e pianificazione in-sessione](/docs/it/scheduled-tasks): pianifica attività locali all'interno di una sessione CLI aperta
* [Desktop scheduled tasks](/docs/it/desktop-scheduled-tasks): attività pianificate locali che vengono eseguite sulla tua macchina con accesso ai file locali
* [Cloud environments](/docs/it/cloud-environments): configura l'accesso di rete, le variabili di ambiente e gli script di configurazione per le sessioni cloud
* [Projects](/docs/it/claude-projects): lavoro in corso che Claude coordina tra sessioni cloud parallele; le routine create da un progetto vengono visualizzate nella scheda **Routines**
* [MCP connectors](/docs/it/mcp): connetti servizi esterni come Slack, Linear e Google Drive
* [GitHub Actions](/docs/it/github-actions): esegui Claude nella tua pipeline CI su eventi del repository
