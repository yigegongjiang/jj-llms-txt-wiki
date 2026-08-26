> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatizzare il lavoro con le routine

> Metti Claude Code in modalità automatica. Definisci routine che vengono eseguite secondo una pianificazione, attivate da chiamate API o che reagiscono agli eventi di GitHub dall'infrastruttura cloud gestita da Anthropic.

<Note>
  Le routine sono in anteprima di ricerca. Il comportamento, i limiti e la superficie dell'API potrebbero cambiare.
</Note>

Una routine è una configurazione salvata di Claude Code: un prompt, uno o più repository e un set di [connectors](/docs/it/mcp), confezionati una volta ed eseguiti automaticamente. Le routine vengono eseguite su infrastruttura cloud gestita da Anthropic, quindi continuano a funzionare quando il tuo laptop è chiuso.

Ogni routine può avere uno o più trigger collegati:

* **Scheduled**: eseguire su una cadenza ricorrente come oraria, notturna o settimanale, oppure una volta a un momento futuro specifico
* **API**: attivare su richiesta inviando un POST HTTP a un endpoint per routine con un token bearer
* **GitHub**: eseguire automaticamente in risposta agli eventi del repository come pull request o release

Una singola routine può combinare trigger. Ad esempio, una routine di revisione PR può essere eseguita di notte, attivata da uno script di distribuzione e anche reagire a ogni nuovo PR.

Le routine sono disponibili sui piani Pro, Max, Team ed Enterprise con [Claude Code sul web](/docs/it/claude-code-on-the-web) abilitato. Creale e gestiscile su [claude.ai/code/routines](https://claude.ai/code/routines), oppure dalla CLI con `/schedule`.

Gli amministratori di Team ed Enterprise possono disabilitare le routine per tutti i membri con l'interruttore Routines su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Quando disabilitate, le routine esistenti smettono di funzionare e i membri non possono crearne di nuove.

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

Le sezioni seguenti illustrano come creare una routine e configurare ognuno di questi tipi di trigger.

<h2 id="create-a-routine">
  Creare una routine
</h2>

Crea una routine dal web su [claude.ai/code/routines](https://claude.ai/code/routines), dall'app Desktop o dalla CLI. Tutte e tre le superfici scrivono nello stesso account cloud, quindi una routine che crei in una appare nelle altre immediatamente. Nell'app Desktop, fai clic su **Routines** nella barra laterale, quindi su **New routine**, e scegli **Remote**; scegliere **Local** invece crea un [Desktop scheduled task](/docs/it/desktop-scheduled-tasks), che viene eseguito sulla tua macchina piuttosto che nel cloud.

Il modulo di creazione configura il prompt della routine, i repository, l'ambiente, i connector e i trigger.

Le routine vengono eseguite autonomamente come sessioni cloud complete di Claude Code: non c'è un selettore di modalità di autorizzazione e nessun prompt di approvazione durante un'esecuzione. La sessione può eseguire comandi shell, utilizzare [skills](/docs/it/skills) impegnate nel repository clonato e chiamare qualsiasi connector incluso. Ciò che una routine può raggiungere è determinato dai repository che selezioni e dalla loro impostazione di branch-push, dall'[ambiente](/docs/it/claude-code-on-the-web#the-cloud-environment) accesso di rete e variabili, e dai connector che includi. Delimita ognuno di questi a ciò di cui la routine ha effettivamente bisogno.

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

    L'input del prompt include un selettore di modello. Claude utilizza il modello selezionato su ogni esecuzione.
  </Step>

  <Step title="Seleziona i repository">
    Aggiungi uno o più repository GitHub per cui Claude possa lavorare. Ogni repository viene clonato all'inizio di un'esecuzione, a partire dal ramo predefinito. Claude crea rami con prefisso `claude/` per le sue modifiche.
  </Step>

  <Step title="Seleziona un ambiente">
    Scegli un [cloud environment](/docs/it/claude-code-on-the-web#the-cloud-environment) per la routine. Gli ambienti controllano a cosa ha accesso la sessione cloud:

    * **Network access**: imposta il livello di accesso a Internet disponibile durante ogni esecuzione
    * **Environment variables**: fornisci chiavi API, token o altri segreti che Claude può utilizzare
    * **Setup script**: installa le dipendenze e gli strumenti di cui la routine ha bisogno. Il risultato è [cached](/docs/it/claude-code-on-the-web#environment-caching), quindi lo script non viene rieseguito su ogni sessione

    Un ambiente **Default** è fornito con accesso di rete **Trusted**, che consente l'[insieme predefinito](/docs/it/claude-code-on-the-web#default-allowed-domains) di registri di pacchetti, API di provider cloud, registri di container e domini di sviluppo comuni, ma blocca tutto il resto. Se la tua routine ha bisogno di raggiungere i tuoi servizi o un dominio al di fuori di tale elenco, modifica l'[accesso di rete](/docs/it/claude-code-on-the-web#network-access) dell'ambiente prima di eseguire. Per utilizzare un ambiente separato, [creane uno](/docs/it/claude-code-on-the-web#configure-your-environment) prima.
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

  <Step title="Rivedi i connector e le autorizzazioni">
    Le schede **Connectors** e **Permissions** in fondo al modulo controllano ciò che la routine può raggiungere.

    Sotto Connectors, tutti i tuoi [MCP connectors](/docs/it/mcp) connessi sono inclusi per impostazione predefinita. Rimuovi quelli che la routine non necessita. Claude può utilizzare ogni strumento da un connector incluso, incluse le scritture, senza chiedere il permesso durante un'esecuzione.

    Sotto Permissions, abilita **Allow unrestricted branch pushes** per qualsiasi repository dove Claude dovrebbe essere in grado di eseguire il push ai rami esistenti invece che solo a quelli con prefisso `claude/`.
  </Step>

  <Step title="Crea la routine">
    Fai clic su **Create**. La routine appare nell'elenco e viene eseguita la prossima volta che uno dei suoi trigger corrisponde. Per avviare un'esecuzione immediatamente, fai clic su **Run now** nella pagina dei dettagli della routine.

    Ogni esecuzione crea una nuova sessione insieme alle tue altre sessioni, dove puoi vedere cosa ha fatto Claude, rivedere le modifiche e creare una pull request.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  Creare dalla CLI
</h3>

Esegui `/schedule` in qualsiasi sessione per creare una routine pianificata in modo conversazionale. Puoi anche passare una descrizione direttamente, per una routine ricorrente come `/schedule daily PR review at 9am` o una una tantum come `/schedule clean up feature flag in one week`. Claude esamina le stesse informazioni che il modulo web raccoglie, quindi salva la routine nel tuo account.

Una partenza riuscita assomiglia a una conversazione: Claude pone domande di follow-up sulla pianificazione, i repository e il prompt prima di salvare. Se Claude invece risponde che devi autenticarti o che non riesce a connettersi al tuo account remoto claude.ai, nessuna routine è stata creata; vedi [Troubleshooting](#troubleshooting).

`/schedule` nella CLI crea solo routine pianificate. Per aggiungere un trigger API o GitHub, modifica la routine sul web su [claude.ai/code/routines](https://claude.ai/code/routines).

La CLI supporta anche la gestione delle routine esistenti. Esegui `/schedule list` per vedere tutte le routine, `/schedule update` per modificarne una, o `/schedule run` per attivarla immediatamente.

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

<Note>
  La pianificazione una tantum dalla CLI è in fase di rollout graduale e potrebbe non essere ancora disponibile sul tuo account. Se `/schedule` offre solo pianificazioni ricorrenti, crea l'esecuzione una tantum dal web su [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

Crea un'esecuzione una tantum dalla CLI descrivendo l'ora in linguaggio naturale. Claude risolve la frase rispetto all'ora attuale e conferma il timestamp assoluto prima di salvare.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

La stessa conversione da locale a UTC come per le pianificazioni ricorrenti si applica ai timestamp una tantum.

Le esecuzioni una tantum non contano rispetto al limite di esecuzione della routine giornaliera. Consumano l'utilizzo della sottoscrizione regolare del tuo piano come qualsiasi altra sessione. Vedi [Usage and limits](#usage-and-limits) per i dettagli.

<h3 id="add-an-api-trigger">
  Aggiungi un trigger API
</h3>

Un trigger API fornisce a una routine un endpoint HTTP dedicato. POSTing all'endpoint con il token bearer della routine avvia una nuova sessione e restituisce un URL di sessione. Usalo per collegare Claude Code nei sistemi di avviso, pipeline di distribuzione, strumenti interni o ovunque tu possa fare una richiesta HTTP autenticata.

I trigger API vengono aggiunti a una routine esistente dal web. La CLI attualmente non può creare o revocare token.

<Steps>
  <Step title="Apri la routine per la modifica">
    Vai a [claude.ai/code/routines](https://claude.ai/code/routines), fai clic sulla routine che desideri attivare tramite API, quindi fai clic sull'icona della matita per aprire **Edit routine**.
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

Per il riferimento API completo, incluse tutte le risposte di errore, le regole di convalida e i limiti dei campi, vedi [Trigger a routine via API](https://platform.claude.com/docs/it/api/claude-code/routines-fire) nella documentazione della piattaforma Claude.

L'endpoint `/fire` è disponibile solo per gli utenti di claude.ai e non fa parte della superficie dell'API della piattaforma Claude.

<h3 id="add-a-github-trigger">
  Aggiungi un trigger GitHub
</h3>

Un trigger GitHub avvia una nuova sessione automaticamente quando si verifica un evento corrispondente su un repository connesso. Ogni evento corrispondente avvia la sua sessione.

<Note>
  Durante l'anteprima di ricerca, gli eventi webhook di GitHub sono soggetti a limiti orari per routine e per account. Gli eventi oltre il limite vengono eliminati fino al ripristino della finestra. Vedi i tuoi limiti attuali su [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

I trigger GitHub vengono configurati solo dall'interfaccia utente web.

<Steps>
  <Step title="Apri la routine per la modifica">
    Vai a [claude.ai/code/routines](https://claude.ai/code/routines), fai clic sulla routine, quindi fai clic sull'icona della matita per aprire **Edit routine**.
  </Step>

  <Step title="Aggiungi un trigger di evento GitHub">
    Scorri fino alla sezione **Select a trigger**, fai clic su **Add another trigger** e scegli **GitHub event**.
  </Step>

  <Step title="Installa l'app Claude GitHub">
    L'app Claude GitHub deve essere installata sul repository a cui desideri sottoscriverti. La configurazione del trigger ti chiede di installarla se non lo è già.

    <Note>
      L'esecuzione di `/web-setup` nella CLI concede l'accesso al repository per la clonazione, ma non installa l'app Claude GitHub e non abilita la consegna del webhook. I trigger GitHub richiedono l'installazione dell'app Claude GitHub, che la configurazione del trigger ti chiede di fare.
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

<h4 id="how-sessions-map-to-events">
  Come le sessioni si mappano agli eventi
</h4>

Ogni evento GitHub corrispondente avvia una nuova sessione. Il riutilizzo della sessione tra gli eventi non è disponibile per le routine attivate da GitHub, quindi due aggiornamenti PR producono due sessioni indipendenti.

<h2 id="manage-routines">
  Gestisci le routine
</h2>

Fai clic su una routine nell'elenco per aprire la sua pagina di dettagli. La pagina dei dettagli mostra i repository della routine, i connector, il prompt, la pianificazione, i token API, i trigger GitHub e un elenco delle esecuzioni passate.

<h3 id="view-and-interact-with-runs">
  Visualizza e interagisci con le esecuzioni
</h3>

Fai clic su qualsiasi esecuzione per aprirla come sessione completa. Da lì puoi vedere cosa ha fatto Claude, rivedere le modifiche, creare una pull request o continuare la conversazione. Ogni sessione di esecuzione funziona come qualsiasi altra sessione: usa il menu a discesa accanto al titolo della sessione per rinominare, archiviare o eliminare.

<Note>
  Uno stato verde nell'elenco delle esecuzioni significa che la sessione è stata avviata e chiusa senza un errore di infrastruttura. Non significa che l'attività nel tuo prompt sia riuscita. Apri l'esecuzione per leggere la trascrizione e confermare cosa ha effettivamente fatto Claude. Le richieste di rete bloccate, gli strumenti connector mancanti e i guasti a livello di attività vengono visualizzati lì piuttosto che nell'indicatore di stato.
</Note>

<h3 id="edit-and-control-routines">
  Modifica e controlla le routine
</h3>

Dalla pagina dei dettagli della routine puoi:

* Fai clic su **Run now** per avviare un'esecuzione immediatamente senza aspettare l'ora pianificata successiva.
* Usa l'interruttore nella sezione **Repeats** per mettere in pausa o riprendere la pianificazione. Le routine in pausa mantengono la loro configurazione ma non vengono eseguite fino a quando non le riabiliti.
* Fai clic sull'icona della matita per aprire **Edit routine** e modificare il nome, il prompt, i repository, l'ambiente, i connector o uno qualsiasi dei trigger della routine. La sezione **Select a trigger** è dove aggiungi o rimuovi pianificazioni, token API e trigger di eventi GitHub.
* Fai clic sull'icona di eliminazione per rimuovere la routine. Le sessioni passate create dalla routine rimangono nell'elenco delle sessioni.

<h3 id="repositories-and-branch-permissions">
  Repository e autorizzazioni di ramo
</h3>

Le routine necessitano dell'accesso a GitHub per clonare i repository. Quando crei una routine dalla CLI con `/schedule`, Claude verifica se il tuo account ha GitHub connesso e ti chiede di eseguire `/web-setup` se non lo è. Vedi [Opzioni di autenticazione GitHub](/docs/it/claude-code-on-the-web#github-authentication-options) per i due modi per concedere l'accesso.

Ogni repository che aggiungi viene clonato su ogni esecuzione. Claude inizia dal ramo predefinito del repository a meno che il tuo prompt non specifichi diversamente.

Per impostazione predefinita, Claude può solo eseguire il push ai rami con prefisso `claude/`. Questo impedisce alle routine di modificare accidentalmente rami protetti o di lunga durata. Per rimuovere questa restrizione per un repository specifico, abilita **Allow unrestricted branch pushes** per quel repository quando crei o modifichi la routine.

<h3 id="connectors">
  Connector
</h3>

Le routine possono utilizzare i tuoi MCP connector connessi per leggere e scrivere nei servizi esterni durante ogni esecuzione. Ad esempio, una routine che triage le richieste di supporto potrebbe leggere da un canale Slack e creare problemi in Linear.

I connector sono le [integrazioni claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) sul tuo account. I server MCP che hai aggiunto localmente nella CLI con `claude mcp add` sono archiviati sulla tua macchina piuttosto che sul tuo account claude.ai, quindi non compaiono nell'elenco dei connector. Per utilizzare uno di questi server in una routine, aggiungilo come connector su [claude.ai/customize/connectors](https://claude.ai/customize/connectors), oppure dichiaralo in un [`.mcp.json`](/docs/it/mcp#project-scope) committato in modo che faccia parte del repository clonato.

Quando crei una routine, tutti i tuoi connector attualmente connessi sono inclusi per impostazione predefinita. Rimuovi quelli non necessari per limitare a quali strumenti Claude ha accesso durante l'esecuzione. Puoi anche aggiungere connector direttamente dal modulo della routine.

Per gestire o aggiungere connector al di fuori del modulo della routine, visita **Settings > Connectors** su claude.ai o usa `/schedule update` nella CLI.

<h3 id="environments-and-network-access">
  Ambienti e accesso di rete
</h3>

Ogni routine viene eseguita in un [cloud environment](/docs/it/claude-code-on-the-web#the-cloud-environment) che controlla l'accesso di rete, le variabili di ambiente e gli script di configurazione. La routine eredita la politica di rete dell'ambiente su ogni esecuzione.

L'ambiente **Default** utilizza l'accesso di rete **Trusted**: l'[elenco di autorizzazione predefinito](/docs/it/claude-code-on-the-web#default-allowed-domains) dei registri di pacchetti, delle API dei provider cloud, dei registri di container e dei domini di sviluppo comuni è raggiungibile, ma i domini arbitrari non lo sono. Le richieste in uscita verso altri host non riescono con `403` e `x-deny-reason: host_not_allowed`. Il traffico del connector MCP viene instradato attraverso i server di Anthropic, quindi i connector che aggiungi alla routine funzionano senza aggiungere i loro host ai **Allowed domains**. Rimuovi eventuali connector che non ti servono in [Connector](#connectors).

Per consentire domini aggiuntivi:

<Steps>
  <Step title="Apri la routine per la modifica">
    Sulla pagina dei dettagli della routine, fai clic sull'icona della matita per aprire **Edit routine**.
  </Step>

  <Step title="Apri il selettore di ambiente">
    Sotto la casella **Instructions**, seleziona l'icona cloud che mostra il nome del tuo ambiente, ad esempio **Default**.
  </Step>

  <Step title="Apri le impostazioni dell'ambiente">
    Passa il mouse sull'ambiente nell'elenco e fai clic sull'icona delle impostazioni che appare a destra.
  </Step>

  <Step title="Modifica il livello di accesso di rete">
    Nella finestra di dialogo **Update cloud environment**, modifica **Network access** in **Custom** e inserisci i tuoi domini in **Allowed domains**. Seleziona **Also include default list of common package managers** per mantenere l'[elenco di autorizzazione predefinito](/docs/it/claude-code-on-the-web#default-allowed-domains) insieme ai tuoi domini personalizzati. Seleziona invece **Full** per l'accesso senza restrizioni.
  </Step>

  <Step title="Salva">
    Fai clic su **Save changes**. La nuova politica si applica dalla prossima esecuzione.
  </Step>
</Steps>

Vedi [Network access](/docs/it/claude-code-on-the-web#network-access) per i dettagli sui livelli di accesso e l'elenco di autorizzazione predefinito.

<h2 id="usage-and-limits">
  Utilizzo e limiti
</h2>

Le routine riducono l'utilizzo dell'abbonamento allo stesso modo delle sessioni interattive. Oltre ai limiti di abbonamento standard, le routine hanno un limite giornaliero su quante esecuzioni possono iniziare per account. Vedi il tuo consumo attuale e le esecuzioni di routine giornaliere rimanenti su [claude.ai/code/routines](https://claude.ai/code/routines) o [claude.ai/settings/usage](https://claude.ai/settings/usage).

Quando una routine raggiunge il limite giornaliero o il limite di utilizzo dell'abbonamento, le organizzazioni con utilizzo extra abilitato possono continuare a eseguire routine su eccedenza misurata. Senza utilizzo extra, le esecuzioni aggiuntive vengono rifiutate fino al ripristino della finestra. Abilita l'utilizzo extra da **Settings > Billing** su claude.ai.

Le esecuzioni una tantum non contano rispetto al limite giornaliero di esecuzioni di routine. Riducono l'utilizzo regolare dell'abbonamento come qualsiasi altra sessione, ma sono esenti dall'indennità giornaliera di esecuzioni di routine per account.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="/schedule-returns-unknown-command">
  `/schedule` restituisce "Unknown command"
</h3>

La CLI nasconde `/schedule` quando uno dei suoi requisiti non è soddisfatto: il menu dei comandi mostra `No commands match "/schedule"` mentre digiti, e l'invio restituisce `Unknown command: /schedule`. La causa è solitamente una delle seguenti:

* Sei autenticato con una chiave API Console o un provider cloud come Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry. `/schedule` richiede un accesso con abbonamento claude.ai. Se `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN` è impostato nella tua shell, o `apiKeyHelper` è impostato in `settings.json`, rimuovilo prima, poiché questi hanno la precedenza su un accesso claude.ai
* `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, o `DISABLE_GROWTHBOOK` è impostato nell'ambiente della tua shell o nel blocco `env` di un [file `settings.json`](/docs/it/settings#available-settings). Questi disabilitano il recupero dei flag di funzionalità, da cui `/schedule` dipende
* Sei all'interno di una sessione Claude Code sul web. Gestisci le routine dall'[interfaccia web](https://claude.ai/code/routines) invece

Puoi sempre creare e gestire le routine su [claude.ai/code/routines](https://claude.ai/code/routines) indipendentemente da come è configurata la CLI.

<h3 id="/schedule-asks-you-to-authenticate">
  `/schedule` ti chiede di autenticarti
</h3>

Se `/schedule` viene eseguito ma Claude risponde che devi prima autenticarti con un account claude.ai, la CLI non ha alcun accesso claude.ai memorizzato. Gli account API non sono supportati per le routine. Esegui `/login`, accedi con il tuo account claude.ai, quindi esegui di nuovo `/schedule`.

<h3 id="routines-are-disabled-by-your-organization’s-policy">
  "Routines are disabled by your organization's policy"
</h3>

Un Owner nella tua organizzazione Team o Enterprise ha probabilmente disattivato l'interruttore **Routines** su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Questa è un'impostazione dell'organizzazione lato server, quindi non può essere ignorata dalla tua configurazione locale. Chiedi a un Owner di abilitare le routine per la tua organizzazione.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [`/loop` e pianificazione in-sessione](/docs/it/scheduled-tasks): pianifica attività locali all'interno di una sessione CLI aperta
* [Desktop scheduled tasks](/docs/it/desktop-scheduled-tasks): attività pianificate locali che vengono eseguite sulla tua macchina con accesso ai file locali
* [Cloud environment](/docs/it/claude-code-on-the-web#the-cloud-environment): configura l'ambiente di runtime per le sessioni cloud
* [MCP connectors](/docs/it/mcp): connetti servizi esterni come Slack, Linear e Google Drive
* [GitHub Actions](/docs/it/github-actions): esegui Claude nella tua pipeline CI su eventi del repository
