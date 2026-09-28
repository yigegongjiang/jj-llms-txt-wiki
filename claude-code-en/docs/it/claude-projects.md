> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lascia che Claude coordini il lavoro in corso con Projects

> Fornisci a Claude un corpo di lavoro correlato in una conversazione e lascia che coordini sessioni cloud parallele che condividono repository, istruzioni e memoria.

<Note>
  Projects è in beta pubblica sui piani Pro e Max e sta venendo implementato gradualmente, a partire da account che hanno utilizzato [sessioni cloud](/docs/it/claude-code-on-the-web) e non hanno progetti esistenti in claude.ai chat o Cowork. Non sono ancora disponibili sui piani Team o Enterprise. Se **Projects** non appare nella barra laterale su [claude.ai/code](https://claude.ai/code) o nella scheda Code dell'[app desktop](/docs/it/desktop), l'implementazione non ha ancora raggiunto il tuo account, e puoi [unirti alla lista d'attesa](https://claude.com/form/projects). [Esegui agenti in parallelo](/docs/it/agents) elenca cosa puoi utilizzare nel frattempo.
</Note>

Un progetto è una conversazione in corso dove Claude coordina un flusso di lavoro correlato per voi. Gli dite cosa deve essere fatto e lui avvia un thread per ogni attività.

Ogni thread è solitamente una [sessione cloud](/docs/it/claude-code-on-the-web): Claude Code in esecuzione nel cloud piuttosto che sulla vostra macchina. Quando un'attività ha bisogno di qualcosa che solo il vostro computer ha, potete chiedere a Claude di eseguire quel thread sul vostro computer invece attraverso [Remote Control](/docs/it/remote-control). I thread vengono eseguiti in parallelo e potete controllarli e guidarli dal vostro telefono. I thread cloud continuano dopo che chiudete il laptop.

Senza un progetto, eseguire più sessioni significa fare il coordinamento da solo: decidi su cosa lavora ciascuno, ripeti lo stesso contesto all'inizio di ciascuno, e controlli indietro per vedere quale è terminato o ha bisogno di una risposta. Con un progetto, invece:

* **Invia il lavoro in un unico posto**: incolla una segnalazione di bug, una traccia dello stack, o un elenco di attività nella conversazione ogni volta che ne viene fuori una. Claude avvia un thread per ogni pezzo di lavoro o lo passa al thread già al lavoro in quell'area, e risponde a domande rapide sul posto.
* **Imposta il contesto una volta**: ogni nuovo thread inizia con le istruzioni del progetto, quindi una regola che dichiari una volta, come quale branch scegliere come target, raggiunge tutti loro.
* **Allontanati e torna al lavoro finito**: quando torni un'ora dopo o la mattina successiva, il riquadro **Overview** mostra quali thread sono terminati, quali pull request sono pronte per la revisione, e quale thread sta aspettando la tua risposta.

Se conosci già il lavoro che desideri che un progetto esegua, vai direttamente a [Crea un progetto](#create-a-project).

<h2 id="when-to-use-a-project">
  Quando creare un progetto
</h2>

Un progetto vale la pena crearlo quando il lavoro ha un obiettivo che va oltre una singola sessione e continua a generare attività. Questi tipi di lavoro si adattano bene a un progetto:

* **Un obiettivo su molti repository**: "Portare ogni servizio alla nuova configurazione lint." Claude può eseguire un thread per repository, ognuno con la propria pull request, e il riquadro [**Overview**](#see-what-needs-you-in-overview) mostra quali sono pronti per la revisione.
* **Un'area che continui ad alimentare**: i bug, le stack trace e le richieste di revisione per un servizio, incollati nella conversazione man mano che li ricevi. Un errore che dici a Claude di ricordare dopo una correzione è nella [memoria del progetto](#give-a-project-standing-context) per il prossimo.
* **Una build o migrazione più grande di una sessione**: "Costruisci quello che `docs/spec.md` descrive" o "Sposta l'app dal vecchio ORM deprecato." Il lavoro si divide in thread che affrontano ciascuno una parte, le decisioni che chiedi a Claude di ricordare all'inizio raggiungono i thread successivi, e le specifiche e i bug che trovi durante la build vanno nella stessa conversazione.
* **Lavoro che non è codice**: una cartella di contratti o un'esportazione di ticket di supporto a cui continui a tornare con nuove domande, come "trova i dieci errori di integrazione più comuni in questi ticket." Carica i documenti invece di aggiungere un repository, e i thread consegnano ogni relazione come file nella scheda [**Library**](#see-what-needs-you-in-overview) del progetto.

In ognuno di essi puoi inviare un batch di attività, dire a Claude di iniziare senza chiederti di confermare, allontanarti e trovare i thread che hanno bisogno di te sotto [**Waiting on you**](#see-what-needs-you-in-overview) quando torni, oppure chiedere a Claude di mettere parte del lavoro in un programma come [routine](/docs/it/routines). Se una di queste è la tua situazione, [crea un progetto](#create-a-project).

<h3 id="when-something-else-fits-better">
  Quando qualcos'altro si adatta meglio
</h3>

I thread cloud funzionano su repository GitHub e sui file, cartelle e cartelle Google Drive che carichi nel progetto, non su file o strumenti che esistono solo sulla tua macchina. Se un'attività ha bisogno della tua macchina, chiedi a Claude di eseguire il suo thread lì attraverso [Remote Control](/docs/it/remote-control). [Limitations](#limitations) elenca di cosa ha bisogno. Qualcos'altro si adatta meglio in questi casi:

* **Un'attività che rientra in una sessione**: "Correggi il test di login instabile." Avvia una [cloud session](/docs/it/claude-code-on-the-web) tu stesso.
* **Lavoro dove ogni attività ha bisogno della tua macchina**: un database locale, un emulatore di dispositivo, o un'API dietro il tuo VPN. Usa una sessione locale, o [agent view](/docs/it/agent-view) per eseguirne diversi contemporaneamente. Se il lavoro ha bisogno solo di file locali, caricali nel progetto invece.
* **Un'attività che si ripete in un programma senza conversazione intorno**: "Pubblica un rapporto di dipendenza ogni lunedì." Crea una [routine](/docs/it/routines) da sola.
* **Più persone che danno a Claude lavoro e lo guidano insieme in un canale Slack**: vedi [Claude Tag](https://claude.com/docs/claude-tag/overview).

Un progetto attinge ai stessi limiti di piano delle tue altre sessioni Claude Code e li utilizza più velocemente. [Usage and cost](#usage-and-cost) copre cosa attinge al tuo piano e come mantenerlo basso.

<h2 id="how-a-project-is-organized">
  Come è organizzato un progetto
</h2>

Un progetto è una conversazione coordinata con Claude più i thread che avvia per svolgere il lavoro. Queste sono le sue parti:

* **La conversazione del progetto**: una sessione di lunga durata in cui Claude agisce come coordinatore. Prende quello che invii, decide cosa diventa un thread e tiene traccia di ogni thread che ha avviato. Vede cosa riferiscono i thread, non ogni passaggio che compiono.
* **Thread**: i lavoratori. Ognuno è una sessione separata con la propria finestra di contesto che svolge un pezzo di lavoro e riferisce alla conversazione quando finisce. Un thread cloud lavora sul proprio ramo e apre una pull request quando il lavoro lo richiede.
* **Quello con cui ogni thread cloud inizia**:
  * I repository e i file del progetto, più le sue [istruzioni e memoria](#give-a-project-standing-context)
  * Il `CLAUDE.md` e le skills in [ogni repository del progetto](#what-threads-pick-up-from-your-repositories), e in un progetto con un repository, anche le regole di permesso e gli hooks di quel repository
  * I [connectors](#get-skills-plugins-connectors-and-tools-into-threads) sul tuo account claude.ai
  * Un [ambiente cloud](#choose-an-environment-for-threads) che imposta il suo accesso di rete, le variabili di ambiente, le credenziali API e gli strumenti installati
* **Il riquadro Overview**: dove [vedi tutti i thread contemporaneamente](#see-what-needs-you-in-overview) e quali di loro hanno bisogno di te. Le sue altre schede sono **Library** per i file che hai aggiunto e i file che i thread hanno prodotto, **Pull requests** per quelli che i thread hanno aperto, e **Routines** per il lavoro programmato nel progetto.

I thread cloud non raccolgono nulla dalla configurazione di Claude Code sulla tua macchina. [Ottieni skills, plugins, connectors e strumenti nei thread](#get-skills-plugins-connectors-and-tools-into-threads) spiega come dare loro quello che altrimenti mancherebbe.

Ecco come queste parti si collegano, da te attraverso la conversazione ai thread che svolgono il lavoro, con **Overview** che traccia il loro stato:

<Frame>
  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=dbf446f69f0bbdb9961d21af207cb93b" className="dark:hidden" alt="Diagram of a project. You write in the project conversation, where Claude answers or starts a thread. Each cloud thread works on its own branch and pull request. The Overview pane lists threads by state, such as ready for review, waiting on you, and working." width="600" height="250" data-path="images/claude-projects-overview.svg" />

  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview-dark.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=549a5ba9fea8433729babc37a1f6e9c8" className="hidden dark:block" alt="Diagram of a project. You write in the project conversation, where Claude answers or starts a thread. Each cloud thread works on its own branch and pull request. The Overview pane lists threads by state, such as ready for review, waiting on you, and working." width="600" height="250" data-path="images/claude-projects-overview-dark.svg" />
</Frame>

<h2 id="create-a-project">
  Crea un project
</h2>

Crei e usi i project su [claude.ai/code](https://claude.ai/code), nella scheda Code dell'app desktop, o nell'app mobile Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Nel browser e nell'app desktop ci sono due modi per avviare un project:

* **Da zero**, quando conosci il flusso di lavoro che vuoi che Claude esegua: apri la finestra di dialogo **New project** e assegnagli un nome. [Avvia un nuovo project da zero](#start-a-new-project-from-scratch) ti guida attraverso la finestra di dialogo.
* **Da una sessione cloud che sta già facendo il lavoro**: scegli **Continue as a project** dal menu di quella sessione, e Claude propone la configurazione del project da quello che la sessione stava facendo. Vedi [Avvia da una sessione cloud esistente](#start-from-an-existing-cloud-session).

In entrambi i casi, [controlla i prerequisiti](#check-the-prerequisites) prima.

<h3 id="check-the-prerequisites">
  Controlla i prerequisiti
</h3>

Prima di creare un project, controlla il tuo piano, la tua configurazione GitHub e cosa il lavoro ha bisogno di raggiungere:

* **Piano**: sei su Pro o Max e **Projects** appare nella tua barra laterale.
* **GitHub, se il project lavorerà su codice**: il tuo codice è su github.com piuttosto che su GitHub Enterprise Server, GitLab o Bitbucket, il tuo account GitHub connesso ha accesso push, e l'app Claude GitHub è installata su di esso. Se hai connesso GitHub con [`/web-setup`](/docs/it/web-quickstart#connect-from-your-terminal), quel token lascia che le tue altre sessioni cloud raggiungano un repository ma non è sufficiente per i thread del project, che hanno bisogno dell'app Claude GitHub. [Configura l'accesso a GitHub](#set-up-github-access) ha i passaggi.
* **Rete, credenziali e strumenti**: per i thread cloud, questi provengono dal [cloud environment](#choose-an-environment-for-threads) del project. L'ambiente predefinito raggiunge già [registri di pacchetti comuni](/docs/it/cloud-environments#default-allowed-domains), quindi controlla questo solo se il lavoro ha bisogno di altri domini, un segreto o uno strumento che non è preinstallato. Se il lavoro ha bisogno di un server MCP, controlla che appaia come connesso nei tuoi [connectors claude.ai](https://claude.ai/customize/connectors).

<h3 id="start-a-new-project-from-scratch">
  Avvia un nuovo project da zero
</h3>

Avviare un project da zero significa aprire la finestra di dialogo **New project**, assegnare un nome al flusso di lavoro e facoltativamente dargli un obiettivo e i repository e i file su cui lavora. Solo il nome è obbligatorio, quindi puoi creare il project prima e riempire il resto mentre il lavoro prende forma.

<Steps>
  <Step title="Apri Projects">
    Su [claude.ai/code](https://claude.ai/code) o nella scheda Code dell'app desktop, seleziona **Projects** nella barra laterale sinistra, quindi seleziona **New project**. In un browser puoi anche andare direttamente a [claude.ai/code/projects/browse](https://claude.ai/code/projects/browse).
  </Step>

  <Step title="Compila la finestra di dialogo New project">
    Limita il project a un flusso di lavoro che continuerai ad aggiungere, come tutto quello che serve per mantenere un'API sotto il suo obiettivo di latenza. [Quando usare un project](#when-to-use-a-project) ha più esempi. Quindi compila i campi della finestra di dialogo:

    * **Name**: come il project appare nell'elenco **Projects**.
    * **Goal** (facoltativo): una riga di quello che stai cercando di realizzare, come "Mantieni la latenza API p95 sotto 200 ms". Claude nella conversazione lavora verso di esso. Senza un obiettivo, Claude lavora dai compiti che invii, e puoi aggiungere un obiettivo in seguito in **Project settings > General**.
    * **Context** (facoltativo): i repository GitHub su cui questo project lavora, più eventuali file, cartelle o cartelle Google Drive che i thread dovrebbero leggere. Fai clic su **Add** per ognuno. Aggiungi i repository di cui la maggior parte dei compiti ha bisogno piuttosto che ogni uno che il lavoro potrebbe toccare; [Decidi quali repository aggiungere](#decide-which-repositories-to-add) copre la scelta, e puoi aggiungerne altri in seguito in **Project settings > Environment**.

    Le regole permanenti su come i thread dovrebbero funzionare vanno in [project instructions](#give-a-project-standing-context), che imposti dopo che il project esiste.
  </Step>

  <Step title="Crea il project">
    Fai clic su **Create project**. La conversazione del project si apre con una casella di messaggio in basso, dove descrivi il lavoro per Claude.

    Nel tuo primo project, Claude prende un turno da solo non appena il project viene creato, a meno che tu non invii un messaggio per primo. Quel turno usa il tuo piano. In esso, Claude potrebbe:

    * Avviare un thread che esplora il repository senza cambiare nulla e propone i prossimi passi, se il project ha un repository che può leggere.
    * Pubblicare **Setup recommendations** tratte dalle tue recenti sessioni cloud: repository da aggiungere, routine da creare e thread che potrebbe avviare. Ogni repository e routine consigliati iniziano attivati. Disattiva quelli che non vuoi, quindi fai clic su **Update setup** per aggiungere il resto, o ignora le raccomandazioni e descrivi il lavoro da solo.
  </Step>
</Steps>

Il project è ora elencato sotto **Projects** nella barra laterale, e la sua conversazione è aperta. [Il tuo primo batch](#your-first-batch) copre cosa configurare prima di inviargli il lavoro.

<h3 id="start-from-an-existing-cloud-session">
  Avvia da una sessione cloud esistente
</h3>

Se hai già una sessione cloud che sta facendo lavoro che appartiene a un project, apri il menu della sessione nella barra laterale e scegli **Continue as a project** o **Move to project**:

* **Continue as a project** crea un nuovo project denominato dopo la sessione e lo apre. Claude legge la sessione e pubblica **Setup recommendations** nella conversazione per te da confermare. La sessione originale rimane nel tuo elenco di sessioni, e se era nel mezzo di un turno continua a funzionare, quindi fermala da solo se non vuoi che entrambi funzionino contemporaneamente. Se usi il banner **Set up project** che può apparire sopra la casella di messaggio della sessione cloud, il risultato è lo stesso, tranne che il turno in esecuzione della sessione si ferma una volta che il project si apre.
* **Move to project** porta il lavoro della sessione in un project esistente. Pubblica un messaggio in quella conversazione del project chiedendo a Claude di leggere la sessione e riprendere da dove l'ha lasciata, e il nuovo lavoro continua nei propri thread del project. La sessione originale rimane nel tuo elenco di sessioni, invariata.

<h3 id="set-up-github-access">
  Configura l'accesso a GitHub
</h3>

La maggior parte della configurazione di GitHub accade una volta, non per project. Connetti il tuo account GitHub a Claude una volta, e l'app Claude GitHub è installata una volta per repository, o una volta per un'intera organizzazione GitHub se le dai tutti i repository. Torni a questi passaggi quando aggiungi un repository che l'app Claude GitHub non copre ancora o uno in un'organizzazione GitHub che applica SSO.

<Steps>
  <Step title="Connetti il tuo account GitHub">
    Se non hai mai usato claude.ai/code prima, la tua prima visita ti guida attraverso la connessione di GitHub; vedi [Connetti GitHub](/docs/it/web-quickstart#connect-github). Altrimenti usa una delle [opzioni di autenticazione GitHub](/docs/it/claude-code-on-the-web#github-authentication-options).
  </Step>

  <Step title="Installa l'app Claude GitHub sui repository del project">
    Installa l'[app Claude GitHub](https://github.com/apps/claude) e concedile i repository che il project userà. Su un repository di proprietà di un'organizzazione GitHub, solo un proprietario dell'organizzazione può completare l'installazione; se non lo sei, GitHub invia al proprietario una richiesta di installazione e il project non può usare il repository fino a quando non la approva.
  </Step>

  <Step title="Autorizza SSO per le organizzazioni che lo applicano">
    Se un'organizzazione GitHub applica SAML SSO, riconnetti GitHub e autorizza l'app Claude per quell'organizzazione. Fino a quando non lo fai, i repository privati di quell'organizzazione non appaiono nella finestra di dialogo **New project** o **Project settings > Environment**.
  </Step>
</Steps>

Quando uno di questi passaggi è incompleto, la finestra di dialogo **New project** e la pagina del project denominano il passaggio mancante e collegano a dove lo completi. Completa il passaggio lì, quindi fai clic su **Check again** se la finestra di dialogo lo offre. Se un repository è ancora mancante dall'elenco in seguito, apri l'installazione dell'app Claude GitHub su GitHub, su [github.com/settings/installations](https://github.com/settings/installations) per un account personale, e conferma che il repository è elencato sotto **Repository access**. Per i messaggi di errore che un thread o il project segnala quando l'accesso è ancora sbagliato, vedi [Errori di accesso al repository](#repository-access-errors).

<h2 id="work-in-a-project">
  Lavora in un project
</h2>

Dai a Claude il lavoro attraverso la conversazione del project: compiti uno alla volta o più contemporaneamente, più aggiornamenti e pensieri sciolti mentre si presentano. Claude instrada ogni messaggio, e i thread fanno il lavoro e riportano.

<h3 id="your-first-batch">
  Il tuo primo batch
</h3>

Prima di inviare a un nuovo project un batch di lavoro, configuralo in modo che i primi thread tornino nel modo che vuoi:

1. [Scrivi le istruzioni del project](#write-project-instructions): il briefing da cui ogni thread inizia, come quale branch scegliere come destinazione, come un thread controlla il suo lavoro e cosa ha bisogno della tua approvazione.
2. Invia un piccolo pezzo del lavoro reale, o avvia uno dei thread che Claude ha suggerito se ne ha offerti, e apri il thread quando finisce per vedere come riporta e cosa ha fatto sul suo branch. Se ha assunto qualcosa di sbagliato o non poteva raggiungere quello che aveva bisogno, [I thread hanno indovinato o si sono bloccati invece di chiedere](#threads-guessed-or-stalled-instead-of-asking) copre dove correggere.
3. Controlla **Thread model** e **Thread effort** in **Project settings > General**. Un nuovo project esegue ogni thread su Opus ad alto sforzo, che attinge al tuo piano più velocemente; [Scegli modelli e lascia che Claude gestisca il contesto](#choose-models-and-let-claude-manage-context) copre le alternative.
4. Chiedi a Claude di [proporre thread prima di avviarli e di eseguirne pochi contemporaneamente](#tune-how-claude-runs-a-project), e abbassa quei limiti una volta che pochi thread tornano nel modo che vuoi.

<h3 id="send-work-and-read-results">
  Invia il lavoro e leggi i risultati
</h3>

Claude decide dove va ogni messaggio che invii nella conversazione:

* Una domanda rapida di solito ottiene una risposta nella conversazione.
* Il nuovo lavoro va a un nuovo thread o a un thread già al lavoro in quell'area, e Claude ti dice quale. Ogni nuovo thread appare sotto il tuo messaggio come una carta: una scatola con il titolo e lo stato del thread, che fai clic per aprire il thread.
* Diversi compiti non correlati in un messaggio diventano thread separati.

Se Claude instrada qualcosa diversamente da come volevi, dillo. [Sintonizza come Claude esegue un project](#tune-how-claude-runs-a-project) elenca le cose che puoi dirgli, come riutilizzare un thread esistente per i follow-up o rispondere sul posto invece di avviare un thread.

I risultati completi di un thread rimangono nel thread, e apri la sua carta nella conversazione per leggerli. I file che un thread ha prodotto sono anche sulla scheda **Library** in **Overview**.

A volte Claude propone thread invece di avviarli, in un elenco **Suggested threads**. Fai clic sulla freccia su un suggerimento per avviare quel thread. Quando più sono elencati, un pulsante sotto l'elenco avvia tutti loro.

<h3 id="review-a-thread’s-pull-request">
  Rivedi la pull request di un thread
</h3>

Quando un thread cambia il codice, questo è quello che fa a meno che tu non gli dica diversamente:

* **Branch**: lavora su un nuovo branch, iniziato dal branch predefinito del repository.
* **Pull request**: apre una quando chiedi, e può aprirne una da sola per una correzione di bug o un altro cambiamento concreto.
* **Dopo che si apre**: guarda la pull request con [auto-fix](/docs/it/claude-code-on-the-web#auto-fix-pull-requests) attivato, indipendentemente dal fatto che auto-fix sia attivato per le tue altre sessioni cloud. Spinge correzioni quando CI fallisce, affronta i commenti di revisione, e risponde nel thread quando i controlli passano e la pull request è pronta per te.

Quando un thread ha spinto un branch o aperto una pull request, la sua carta nella conversazione può mostrare un pulsante per il prossimo passo:

* **Resolve conflicts**, **Fix CI**, **Address comments**, e **Merge it** inviano quell'istruzione al thread come messaggio da te, quindi puoi spingere il thread da solo invece di aspettare che reagisca alla pull request.
* **Review PR** apre la pull request su GitHub.
* **Create PR** appare quando un thread inattivo ha spinto un branch ma non ha aperto una pull request. Facendovi clic crei la pull request da quel branch direttamente piuttosto che inviare al thread un'istruzione per aprirne una.

Per cambiare quando i thread aprono le pull request, ad esempio solo quando chiedi, o quale branch iniziano, dillo nel compito o nelle [istruzioni del project](#write-project-instructions).

<h3 id="see-what-needs-you-in-overview">
  Vedi cosa ti serve in Overview
</h3>

Il riquadro **Overview** accanto alla conversazione traccia i thread del project. È già aperto la prima volta che apri un nuovo project. Il pulsante **Overview** nell'intestazione del project lo chiude e lo riapre, e mostra un punto quando un thread ti sta aspettando.

Nell'app desktop, ricevi anche una notifica desktop quando Claude pubblica nella conversazione, un thread colpisce un errore, o un thread ha bisogno del tuo input, quindi non devi tenere il project aperto per scoprirlo. Per riceverne anche una ogni volta che un thread finisce un turno, o per disattivarle per un project, scegli **Notifications** nel menu della barra laterale del project. Queste notifiche sono solo desktop: in un browser, controlla il punto sul pulsante **Overview**.

La scheda **Threads** del riquadro raggruppa i thread per stato:

| Gruppo               | Cosa c'è dentro                                                                                                                                                                                                                          |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Ready for review** | Thread la cui pull request è aperta e in attesa di revisione                                                                                                                                                                             |
| **Waiting on you**   | Thread che hanno bisogno della tua risposta o approvazione, o che hanno fallito                                                                                                                                                          |
| **Working**          | Thread ancora in esecuzione                                                                                                                                                                                                              |
| **Landing**          | Thread la cui pull request è approvata o in coda per l'unione                                                                                                                                                                            |
| **Idle**             | Thread che hanno finito e non stanno aspettando nulla                                                                                                                                                                                    |
| **Resolved**         | Thread contrassegnati come fatto: da te dal menu del thread, da Claude una volta che hai preso l'ultimo passo, come unire la sua pull request, o automaticamente dopo una settimana senza attività. Puoi riaprirne uno dallo stesso menu |

Le altre schede del riquadro sono **Library** per i file e le cartelle che hai aggiunto e i file che i thread hanno prodotto, **Pull requests** una volta che i thread ne hanno aperto, e **Routines** per le [routine](/docs/it/routines) che Claude ha configurato da questo project.

<h3 id="open-a-thread-when-you-need-control">
  Apri un thread quando hai bisogno di controllo
</h3>

Fai clic sulla carta di un thread nella conversazione o sulla sua riga in **Overview** per aprire la sua trascrizione nel riquadro Overview. Da lì puoi:

* Leggere quello che Claude ha fatto, passo dopo passo.
* Guidare il compito scrivendo nella casella di messaggio del thread. Un messaggio lì va direttamente a quel thread, mentre un follow-up nella conversazione del project lo raggiunge solo quando Claude abbina il follow-up a quel thread.
* Rispondere a un prompt di permesso su cui il thread sta aspettando.
* Interrompere il thread con **Stop**, che sostituisce il pulsante di invio mentre il thread sta funzionando, o premendo Esc.

<h3 id="choose-models-and-let-claude-manage-context">
  Scegli modelli e lascia che Claude gestisca il contesto
</h3>

Imposta modelli e sforzo in **Project settings > General**. Un nuovo project esegue Opus ovunque, con alto [sforzo](/docs/it/model-config#adjust-effort-level) per i thread e basso sforzo per la conversazione:

* **Thread model** e **Thread effort** si applicano ai thread. Per usare un modello diverso per un compito, chiedilo nel compito; per un thread già in esecuzione, usa il selettore di modello di quel thread.
* **Coordinator model** e **Coordinator effort** si applicano a Claude nella conversazione del project.

Non gestisci le finestre di contesto in un project. I thread si compattano automaticamente, e la conversazione funziona da messaggi recenti, thread recenti e project memory piuttosto che dalla sua storia completa, quindi continua finché il project funziona. Metti qualsiasi cosa che non deve mai essere eliminata in [project memory](#give-a-project-standing-context). Se un thread supera il suo contesto, mostra [Claude ha esaurito il contesto in questo turno](#context-limit).

<h3 id="tune-how-claude-runs-a-project">
  Sintonizza come Claude esegue un project
</h3>

Dì a Claude nella conversazione quanti thread eseguire contemporaneamente, quando pubblicare aggiornamenti e quando aprire pull request. Se Claude sta coordinando in un modo che non vuoi, dillo. Ad esempio, puoi dire:

* "Proponi thread e aspetta la mia approvazione prima di avviarli" o "Avvia questi ora senza chiedermi di confermare"
* "Esegui al massimo due thread contemporaneamente" o "Riutilizza un thread esistente per i follow-up nella stessa area"
* "Pubblica aggiornamenti più brevi" o "Pubblica solo quando qualcosa finisce o è bloccato"
* "Dammi un aggiornamento di stato su ogni thread"
* "Fai questo compito con un modello più piccolo"
* "Non aprire una pull request finché non ho visto il piano"
* "Dimmi cosa c'è di sbagliato in questi repository e non correggere nulla ancora", quando vuoi esaminare i risultati prima che uno di loro diventi un thread
* "Rispondi qui invece di avviare un thread", quando Claude avvia un thread per qualcosa che intendevi come una domanda rapida

Claude salva preferenze come queste in [project memory](#give-a-project-standing-context) da solo e le segue nei thread successivi. Sono istruzioni che Claude segue, non impostazioni applicate, quindi un limite di thread che dai in questo modo non è un limite rigido. Aggiungine uno alle istruzioni del project quando vuoi che sia formulato esattamente e applicato a ogni thread dall'inizio.

<h3 id="unblock-a-thread-waiting-on-approval">
  Sblocca un thread in attesa di approvazione
</h3>

I thread vengono eseguiti in [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) quando il modello del thread lo supporta, quindi la maggior parte delle chiamate di strumento vengono eseguite senza chiederti. Quando un thread ha bisogno della tua approvazione, il prompt è dentro quel thread e il thread aspetta finché non rispondi lì. Dire a Claude nella conversazione del project di procedere non lo raggiunge.

Ogni approvazione copre quel prompt, o il resto di quel thread se scegli l'opzione più ampia. Per lasciare che ogni thread esegua determinati comandi senza chiedere, o per bloccarne alcuni, aggiungi [regole di permesso](/docs/it/permissions) al `.claude/settings.json` del repository. I thread le applicano solo in un project con un repository; vedi [Cosa i thread raccolgono dai tuoi repository](#what-threads-pick-up-from-your-repositories).

<h2 id="give-a-project-standing-context">
  Fornire contesto permanente al progetto
</h2>

La memoria del progetto, le istruzioni del progetto e i repository, i file e l'ambiente del progetto portano il contesto tra i thread. Imposti ciascuno una sola volta.

| Contesto                    | Cosa contiene                                                                                                                                                                                                    | Come lo imposti                                                                                                                                                                                                                |
| :-------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Memoria del progetto        | Note che Claude mantiene sul progetto, come requisiti, decisioni e insidie, archiviate come file. Ogni thread cloud legge il file di indice `MEMORY.md` quando inizia e apre gli altri file quando ne ha bisogno | Chiedi a Claude nella conversazione del progetto o in qualsiasi thread cloud di ricordare un requisito, una decisione o un'insidia, oppure di dimenticarla. Leggi, modifica ed elimina i file in **Project settings > Memory** |
| Istruzioni del progetto     | Testo inviato a ogni nuovo thread e a Claude nella conversazione del progetto, fino a 16.000 caratteri. [Write project instructions](#write-project-instructions) copre cosa inserirvi                           | **Project settings > Memory > Project instructions**, oppure chiedi a Claude di modificare le istruzioni                                                                                                                       |
| Repository, file e ambiente | I repository che ogni thread cloud clona, le cartelle e i file che può leggere in `/mnt/project-files`, e l'ambiente cloud in cui vengono eseguiti i thread                                                      | Repository e ambiente in **Project settings > Environment**, oppure chiedi a Claude nella conversazione di aggiungere un repository al progetto. File e cartelle da **Add** nella scheda **Library** in **Overview**           |

**Project settings > Memory** elenca questi file in **Auto memory**, perché Claude li scrive da solo mentre lavora nel progetto. Sono separati dall'[auto memory](/docs/it/memory) che Claude Code mantiene sulla tua macchina, anche se entrambi utilizzano un indice `MEMORY.md`. La memoria del progetto è anche separata dai file `CLAUDE.md` nei repository del progetto. Ogni thread cloud legge comunque quei file `CLAUDE.md` dal suo clone quando inizia, quindi inserisci le istruzioni su un repository nel suo `CLAUDE.md` e le note sul progetto nella memoria del progetto.

<h3 id="write-project-instructions">
  Scrivi le istruzioni del progetto
</h3>

Le istruzioni del progetto sono il briefing da cui ogni nuovo thread inizia. Fai clic sull'icona dell'ingranaggio nell'intestazione del progetto per aprire **Project settings**, quindi vai a **Memory > Project instructions**. Un briefing utile copre:

* A cosa serve il progetto
* Dove avviene il lavoro: quali repository, da quale branch iniziare, come denominare le pull request
* Come un thread verifica il proprio lavoro prima di considerarlo completato
* Cosa fare quando manca qualcosa di cui ha bisogno
* Cosa ha bisogno della tua approvazione prima

Ad esempio:

```text theme={null}
Questo progetto mantiene la latenza p95 per l'API dei pagamenti sotto 200 ms: profiling, correzioni di query e caching, e gli upgrade delle dipendenze che ne derivano, nel repository payments-api.

- Crea un branch da main e apri una pull request in bozza per thread.
- Prima di considerare il lavoro completato, esegui `make test` e `make lint` e incolla le righe di riepilogo nel tuo messaggio finale.
- Se non riesci a raggiungere qualcosa di cui hai bisogno, come un repository, un secret, un'API o un connettore, dichiara esattamente cosa manca nel tuo primo messaggio e fermati. Non sostituire, simulare o indovinare.
- Non eseguire merge, force-push o modificare la configurazione CI senza chiedermi nel thread.
```

Le regole su un repository, come i suoi comandi di build, appartengono al `CLAUDE.md` di quel repository, che ogni thread cloud legge quando il repository fa parte del progetto. Una volta che il lavoro è in corso, quando correggi un thread, chiedi anche a Claude di ricordare la correzione: va nella [memoria del progetto](#give-a-project-standing-context) e i thread successivi iniziano con essa.

<h3 id="decide-which-repositories-to-add">
  Decidi quali repository aggiungere
</h3>

I repository che aggiungi a un progetto vengono forniti con tutto ciò che contengono, il loro codice, `CLAUDE.md` e skills, in ogni thread cloud. I repository che non aggiungi sono comunque raggiungibili: un thread cloud può aggiungerne uno a se stesso quando il suo compito lo richiede. La maggior parte dei progetti utilizza entrambi:

* **Aggiungilo al progetto**, nella finestra di dialogo **New project**, in **Project settings > Environment**, oppure chiedendo a Claude nella conversazione di aggiungerlo al progetto. Ogni thread da quel momento in poi lo clona e inizia con il suo `CLAUDE.md` e le sue skills caricati, indipendentemente dal fatto che il compito lo tocchi o meno. Passare da un repository a diversi cambia anche ciò che i thread prendono dal `.claude/settings.json` di ogni repository; vedi [What threads pick up from your repositories](#what-threads-pick-up-from-your-repositories).
* **Lascialo fuori e lascia che i thread lo aggiungano quando necessario.** Un thread cloud il cui compito richiede un repository che il progetto non ha può aggiungerlo a se stesso, e una nota nel thread dice che è stato aggiunto solo a questo thread. Il clone avviene a metà del compito, quindi il `CLAUDE.md` e le skills di quel repository non erano presenti quando il thread è iniziato. Il thread successivo inizia di nuovo senza di esso. Un repository che un thread aggiunge a se stesso ha bisogno degli stessi [prerequisiti](#check-the-prerequisites) di un repository del progetto: l'app GitHub di Claude installata su di esso e accesso push dal tuo account GitHub.

Un progetto non ha bisogno di un repository affatto. I suoi thread cloud possono comunque fare ricerche, scrivere documenti, scrivere ed eseguire codice nella loro sandbox, e consegnare file alla scheda **Library**. Un thread cloud lì può anche aggiungere un repository a se stesso quando un compito lo richiede.

Una volta che il progetto ha repository, Claude può aggiungere solo repository da un proprietario GitHub che il progetto già utilizza, sia che aggiunga uno al progetto sia che un thread ne aggiunga uno a se stesso. Per portare un repository da un proprietario diverso, aggiungilo tu stesso al progetto in **Project settings > Environment**.

Per un progetto che si estende su molti repository, come una funzionalità con codice server, web, mobile e desktop, aggiungi uno o due repository che quasi ogni compito tocca e nomina gli altri nelle [istruzioni del progetto](#write-project-instructions) in modo che Claude sappia dove vive il resto del codice. I thread iniziano quindi in piccolo e tirano in gli altri repository solo per i compiti che ne hanno bisogno.

<h3 id="what-threads-pick-up-from-your-repositories">
  Cosa i thread prendono dai tuoi repository
</h3>

Ogni thread cloud clona ogni repository nel progetto e carica `CLAUDE.md` e skills da tutti loro. Le regole di permesso, gli hook e `env` provengono solo dal `.claude/settings.json` nella directory in cui il thread inizia: all'interno del repository quando il progetto ne ha uno, e sopra i cloni quando ne ha diversi, dove il file di nessun repository viene letto per loro.

| In ogni repository                                                   | Un repository                                                                                                                               | Diversi repository                                                 |
| :------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------- |
| `CLAUDE.md`                                                          | Caricato quando il thread inizia                                                                                                            | Caricato da ogni repository quando il thread inizia                |
| Skills, agenti e comandi in `.claude/`                               | Caricati                                                                                                                                    | Caricati da ogni repository                                        |
| Plugin abilitati in `.claude/settings.json`                          | Non caricati. Aggiungi il plugin in **Project settings > Plugins**                                                                          | Non caricati. Aggiungi il plugin in **Project settings > Plugins** |
| Regole di permesso, hook e `env` definiti in `.claude/settings.json` | Si applicano al thread, tranne le chiavi `env` che [nessuna sessione cloud onora](/docs/it/cloud-environments#what-carries-over-from-your-setup) | Non si applicano                                                   |

In un progetto con diversi repository, ogni clone è collegato al thread come una [directory aggiuntiva](/docs/it/memory#load-from-additional-directories) con il caricamento di `CLAUDE.md` attivato, motivo per cui il `CLAUDE.md` e le skills di ogni repository si caricano all'avvio anche se il thread inizia sopra di loro. In un tale progetto, inserisci le regole permanenti nelle istruzioni del progetto e fornisci ai thread le variabili di ambiente attraverso l'[ambiente cloud](#choose-an-environment-for-threads).

<h3 id="choose-an-environment-for-threads">
  Scegli un ambiente per i thread
</h3>

Ogni nuovo thread cloud inizia nell'[ambiente cloud](/docs/it/cloud-environments) del progetto. L'ambiente imposta quali domini i thread possono raggiungere, quali variabili di ambiente hanno, quali credenziali API vengono aggiunte alle loro richieste, e cosa lo script di setup installa prima che Claude inizi. I thread cloud utilizzano un ambiente predefinito ospitato da Anthropic fino a quando non ne scegli uno in **Project settings > Environment**.

Se i thread cloud hanno bisogno di raggiungere un'API interna o un registro di pacchetti privato, o hanno bisogno di un token che la tua macchina normalmente contiene, cambia l'ambiente piuttosto che il progetto: vedi [Network access](/docs/it/cloud-environments#network-access), [Add API credentials](/docs/it/cloud-environments#add-api-credentials), e [Setup scripts](/docs/it/cloud-environments#setup-scripts).

<h3 id="get-skills-plugins-connectors-and-tools-into-threads">
  Ottieni skills, plugin, connettori e strumenti nei thread
</h3>

I thread cloud non hanno le skills, i server MCP, i plugin e gli strumenti installati solo sulla tua macchina. Un thread che Claude esegue sulla tua macchina attraverso [Remote Control](/docs/it/remote-control) utilizza ciò che è installato lì. Per rendere ciascuno di questi disponibile ai thread cloud:

* Skills, subagenti e comandi: eseguine il commit in un repository che hai aggiunto al progetto, ad esempio una skill in `.claude/skills/<skill-name>/SKILL.md`. Ogni thread cloud clona ogni repository nel progetto e carica `.claude/skills/`, `.claude/agents/` e `.claude/commands/` da ciascuno di loro, quindi una skill sottoposta a commit in un repository è disponibile in ogni nuovo thread cloud. I thread cloud caricano anche le skills che abiliti per il tuo account claude.ai.
* Plugin: aggiungili in **Project settings > Plugins**; si caricano in ogni nuovo thread cloud. I plugin che un repository dichiara nel suo `.claude/settings.json` [non si caricano nei thread cloud](/docs/it/cloud-environments#what-carries-over-from-your-setup).
* Server MCP: i thread cloud ottengono i loro strumenti MCP dai connettori sul tuo account claude.ai, che sono server MCP che colleghi una volta in [claude.ai/customize/connectors](https://claude.ai/customize/connectors) o attraverso il link **Manage connectors** in **Project settings > Environment**. Ogni thread cloud può utilizzarli tutti senza alcuna configurazione per progetto. La conversazione del progetto stessa non ha connettori, quindi invia il lavoro che ne richiede uno come compito per un thread cloud. In un progetto con un repository, i thread cloud caricano anche i server MCP dal [`.mcp.json`](/docs/it/cloud-environments#what-carries-over-from-your-setup) di quel repository. [How connectors reach Claude Code](/docs/it/mcp#how-connectors-reach-claude-code) elenca le regole per le sessioni cloud e le impostazioni che disattivano i connettori.
* Strumenti da riga di comando e pacchetti: installali nello [script di setup](/docs/it/cloud-environments#setup-scripts) dell'ambiente.

Per vedere quali connettori ha un thread cloud in esecuzione in claude.ai/code, apri il thread e seleziona **Connectors** dal menu **+** accanto alla sua casella di messaggio. Disattivare un connettore lì lo rimuove da quel thread e lo salva come impostazione predefinita del tuo account, quindi i nuovi thread cloud e le chat claude.ai iniziano senza di esso fino a quando non lo riattivi. Un thread cloud raccoglie un connettore che aggiungi o riconnetti dopo il prossimo messaggio che gli invii.

<h2 id="project-settings-reference">
  Riferimento delle impostazioni del project
</h2>

Cambi le impostazioni del project su claude.ai/code o nell'app desktop, non in `settings.json`. Apri **Project settings** da **Settings** nel menu della barra laterale del project o dall'icona dell'ingranaggio nell'intestazione del project.

Le impostazioni vengono salvate mentre le modifichi; un campo di testo che stai modificando, come l'obiettivo o le istruzioni, mostra **Save changes** e **Discard** finché non lo lasci. Le modifiche alle istruzioni, ai repository, ai plugin e all'ambiente in **Project settings** raggiungono i nuovi thread, non i thread già in esecuzione.

| Impostazione               | Scheda      | Cosa controlla                                                                                                        |
| :------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------- |
| Nome, icona e obiettivo    | General     | Il nome e l'icona del project nella barra laterale e il suo obiettivo di una riga                                     |
| Coordinator model e effort | General     | Il modello e il [livello di sforzo](/docs/it/model-config#adjust-effort-level) per Claude nella conversazione del project  |
| Thread model e effort      | General     | Il modello e il livello di sforzo per i thread                                                                        |
| Project instructions       | Memory      | [Regole permanenti](#give-a-project-standing-context) che ogni nuovo thread riceve                                    |
| Project repositories       | Environment | I repository che i nuovi thread clonano                                                                               |
| Cloud environment          | Environment | Il [cloud environment](#choose-an-environment-for-threads) in cui i nuovi thread vengono eseguiti                     |
| Connectors                 | Environment | Un link per gestire i connectors claude.ai che i thread ottengono                                                     |
| Plugins                    | Plugins     | I plugin che si caricano in ogni nuovo thread                                                                         |
| Usage                      | Usage       | [Utilizzo di token](#usage-and-cost) per thread e per modello                                                         |
| Memory                     | Memory      | I file di [memory](#give-a-project-standing-context) del project                                                      |
| Restart Claude             | General     | Riavvia la conversazione del project quando [Claude smette di rispondere lì](#claude-hasnt-responded)                 |
| Pause, Archive, Delete     | General     | Ferma, nascondi o rimuovi il project; vedi [Pausa, archivia o elimina un project](#pause-archive-or-delete-a-project) |

<h3 id="pause-archive-or-delete-a-project">
  Pausa, archivia o elimina un project
</h3>

Tutti e tre i controlli sono in fondo a **Project settings > General**:

* **Pause**: ferma tutto contemporaneamente. Ogni thread in esecuzione e la conversazione vengono interrotti, nessun nuovo thread inizia, le routine non vengono eseguite e il project non accetta messaggi finché non lo riprendi. Fai clic su **Resume** nello stesso posto o sul banner sopra la casella di messaggio del project; un thread in pausa continua quando gli invii un messaggio dopo.
* **Archive**: nasconde il project dalla barra laterale e archivia i suoi thread, che ferma qualsiasi thread che era in esecuzione o guardava una pull request. Le routine nel project non vengono eseguite mentre è archiviato. Per riportare il project, aprilo dalla pagina Projects e fai clic su **Unarchive**. I suoi thread rimangono archiviati finché non li disarchivia individualmente dall'elenco delle sessioni.
* **Delete**: rimuove permanentemente il project insieme ai suoi thread, alla sua memoria e ai suoi file, e disattiva le routine del project. Questo non può essere annullato. I branch e le pull request che i thread hanno spinto a GitHub non sono interessati.

<h2 id="usage-and-cost">
  Utilizzo e costo
</h2>

L'utilizzo del project conta rispetto agli stessi [limiti di piano](/docs/it/errors#youve-hit-your-session-limit) delle tue altre sessioni Claude Code, e un project non può spendere oltre quei limiti da solo.

Un thread che raggiunge il limite del tuo piano aspetta e continua da solo quando il limite si ripristina, quindi il lavoro che hai lasciato in esecuzione inizia a usare la tua prossima finestra di utilizzo senza un messaggio da te. [Un thread ha raggiunto il limite di utilizzo](#usage-limit-reached) copre quello che vedi, come fermarlo e l'unico caso che non aspetta.

Il lavoro va oltre i limiti del tuo piano solo se hai attivato [crediti di utilizzo](/docs/it/costs#add-usage-credits-to-your-subscription) per il tuo account. Un thread non può attivarli per te.

<h3 id="what-draws-on-your-plan">
  Cosa attinge al tuo piano
</h3>

Un project usa i tuoi limiti più velocemente di una singola sessione, e su un piano Pro in particolare dovresti aspettarti di raggiungere il tuo limite prima nei giorni in cui ne esegui uno. Queste sono le parti di un project che usano il tuo piano:

* **Thread in esecuzione**: ognuno è una sessione completa, e più possono essere eseguiti contemporaneamente. Non c'è un numero fisso; Claude avvia quanti il lavoro richiede, e un limite che [chiedi](#tune-how-claude-runs-a-project) è una preferenza piuttosto che un limite. Il limite applicato è 200 nuovi thread al giorno nei tuoi project.
* **La conversazione**: Claude usa token da solo leggendo quello che i thread riportano e decidendo cosa fare dopo.
* **Thread che guardano una pull request**: un thread inattivo si sveglia e usa il tuo piano di nuovo quando CI fallisce o un commento di revisione arriva sulla sua pull request. Per fermare questo, chiedi nel thread di smettere di guardare la pull request.

Un project senza thread in esecuzione, nessuna pull request guardato e nessun nuovo messaggio non usa il tuo piano mentre rimane inattivo, e nemmeno un project archiviato.

<h3 id="see-and-reduce-a-project’s-usage">
  Vedi e riduci l'utilizzo di un project
</h3>

Apri la scheda **Usage** in **Project settings** per vedere l'utilizzo di token per thread e per modello, e quanto è andato alla conversazione del project. Per ridurlo:

* Un follow-up instradato a un thread che è stato inattivo più a lungo della [durata della cache](/docs/it/prompt-caching#cache-lifetime), un'ora su Pro e Max entro i limiti del tuo piano, rilegge l'intera conversazione di quel thread prima di fare qualsiasi cosa. Per il nuovo lavoro, chiedere a Claude di avviare un thread fresco può usare meno che far rivivere uno grande e vecchio.
* Per il lavoro che non ha bisogno del modello più grande, [scegli un modello più piccolo o un livello di sforzo inferiore](#choose-models-and-let-claude-manage-context) per i thread, la conversazione o entrambi.
* Chiedi a Claude nella conversazione del project di eseguire meno thread contemporaneamente, o di rispondere da solo a piccole domande invece di avviare un thread.

<h2 id="how-projects-relate-to-other-claude-code-features">
  Come i project si relazionano ad altre funzioni Claude Code
</h2>

Diverse funzioni Claude Code permettono a più di una sessione di funzionare contemporaneamente, quindi eseguire il lavoro in parallelo non è di per sé quello per cui è un project. In un project, Claude avvia e traccia le sessioni invece di te, e ognuno inizia dalle stesse istruzioni. Ecco come ogni funzione vicina si connette a un project:

* **Claude Tag**: [Claude Tag](https://claude.com/docs/claude-tag/overview) è Claude nei canali Slack del tuo team, su piani Team e Enterprise. Chiunque in un canale può dargli lavoro, tutti nel canale lo vedono e lo guidano, e usa connessioni che un admin ha configurato per quel canale. Un project è solo tuo: sei l'unico che gli invia lavoro o vede i suoi thread, usa il tuo accesso GitHub e i tuoi connectors, ed è su Pro e Max. [Come Claude Tag differisce da Cowork e Claude Code](https://claude.com/docs/claude-tag/concepts/how-it-works#how-claude-tag-differs-from-cowork-and-claude-code) ha il confronto fianco a fianco.
* **Sessioni cloud**: ogni thread è una [sessione cloud](/docs/it/claude-code-on-the-web) a meno che non chiedi a Claude di eseguirla sulla tua macchina. In entrambi i casi, Claude la avvia e la traccia invece di te. Una sessione cloud che hai avviato da solo può diventare un project o alimentarne uno attraverso [**Continue as a project** o **Move to project**](#start-from-an-existing-cloud-session).
* **Routine**: quando chiedi il lavoro programmato in un project, Claude crea una [routine](/docs/it/routines) che viene eseguita come thread in quel project e appare sulla sua scheda **Routines**. Le routine che crei al di fuori di un project continuano a funzionare da sole.
* **Sessioni locali e agent view**: una sessione che avvii da solo nel tuo terminale, IDE o nell'ambiente locale dell'app desktop non può essere aggiunta a un project. Un project raggiunge la tua macchina solo eseguendo un thread lì attraverso [Remote Control](/docs/it/remote-control). [Agent view](/docs/it/agent-view) è una schermata per tracciare diverse sessioni locali che hai avviato da solo; non ha un coordinatore.
* **Worktrees**: un [worktree](/docs/it/worktrees) dà a ogni sessione locale la sua copia di lavoro di un repository in modo che le sessioni parallele sulla tua macchina non si sovrascrivano a vicenda. I thread cloud non ne hanno bisogno: ognuno clona i suoi repository nella sua sandbox cloud e lavora sul suo branch.
* **Agent teams**: un [agent team](/docs/it/agent-teams) è una sessione che avvia sessioni di compagno per un singolo compito, sulla tua macchina o dentro una sessione cloud, e finisce con quel compito.
* **Projects in claude.ai chat e Cowork**: l'[esperienza Projects precedente](https://support.claude.com/en/articles/9517075-what-are-projects), che raggruppa conversazioni e file di riferimento senza thread o un coordinatore. Questi project continuano a funzionare come fanno oggi fino a quando l'esperienza riprogettata li raggiunge.

[Esegui agenti in parallelo](/docs/it/agents) confronta queste opzioni fianco a fianco.

<h2 id="limitations">
  Limitazioni
</h2>

* I project sono disponibili su claude.ai/code, nell'app desktop e nell'app mobile di Claude, non nel CLI del terminale o attraverso Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Il comando [`claude project`](/docs/it/cli-reference) del CLI, che gestisce lo stato locale di Claude Code per una directory, non è correlato.
* I thread del project sono [sessioni cloud](/docs/it/claude-code-on-the-web), o sessioni sulla vostra macchina attraverso [Remote Control](/docs/it/remote-control), con Anthropic come fornitore di modello in entrambi i casi. [Sicurezza](/docs/it/security) e [Utilizzo dei dati](/docs/it/data-usage) coprono come le sessioni cloud sono isolate e cosa viene conservato, e [Connessione e sicurezza](/docs/it/remote-control#connection-and-security) copre come un thread sulla vostra macchina si connette e cosa viene archiviato.
* Non potete aggiungere una sessione che avete avviato voi stessi sulla vostra macchina a un project. Per consentire a un project di eseguire un thread sulla vostra macchina, collegate la cartella su cui dovrebbe lavorare attraverso [Remote Control](/docs/it/remote-control#requirements): attivate Remote Control in **Settings > Claude Code** nell'app desktop di Claude, oppure eseguite `claude remote-control` nella cartella e lasciatela in esecuzione. Quella macchina ha bisogno di Claude Code v2.1.280 o successivo. Un project inoltre non può eseguire un thread sulla vostra macchina mentre **Require trusted devices** è attivato nelle vostre impostazioni di claude.ai.
* La sandbox di un thread cloud si mette in pausa tra i turni e riprende quando il thread continua. Se la sandbox non può essere ripresa, il thread continua da un clone fresco, quindi i cambiamenti non committati possono essere persi. Su compiti lunghi, chiedete a Claude di committare e spingere il lavoro in corso.
* Un project appartiene a un utente. Non puoi condividere un project o i suoi thread con un altro utente, e le trascrizioni dei thread non hanno l'opzione di condivisione che hanno altre sessioni cloud. Non ci sono controlli a livello di organizzazione per i project durante la beta.
* Un thread appartiene all'unico project che l'ha avviato. Non puoi spostare o copiare un thread in un altro project, o spostarlo fuori per stare da solo. [**Move to project**](#start-from-an-existing-cloud-session) va solo nell'altro modo: porta il lavoro di una sessione cloud in un project.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Per i prompt di configurazione di GitHub nella finestra di dialogo **New project**, vedi [Configura l'accesso a GitHub](#set-up-github-access).

<h3 id="a-thread-looks-stuck">
  Un thread sembra bloccato
</h3>

Claude non pubblica ogni passo che un thread compie, quindi un thread che mostra come in esecuzione senza nuovi messaggi nella conversazione del project di solito sta ancora funzionando. Un nuovo thread cloud provisiona anche il suo [cloud environment](/docs/it/cloud-environments) prima che Claude inizi, quindi il suo primo aggiornamento richiede un momento. Apri il thread per leggere la sua trascrizione. Se il thread sta aspettando un prompt di permesso, rispondi lì.

<h3 id="threads-guessed-or-stalled-instead-of-asking">
  I thread hanno indovinato o si sono bloccati invece di chiedere
</h3>

Quando più thread tornano avendo assunto qualcosa di sbagliato, hanno aggirato l'accesso mancante o si sono fermati con "bloccato", la causa di solito è lo stesso gap nella configurazione del project piuttosto che un problema con ogni compito. Ordina quali thread sono solidi prima di correggere qualsiasi cosa:

1. Chiedi a Claude nella conversazione: "Per ogni thread aperto, elenca quello che gli hai chiesto di fare, cosa ha assunto o non poteva raggiungere e cosa sta aspettando." Claude legge ogni thread e risponde nella conversazione.
2. Per i thread che hanno iniziato da un'assunzione sbagliata, apri il thread da **Overview** e contrassegnalo come risolto dal suo menu, o digli cosa fare invece nella sua casella di messaggio. Il suo branch e qualsiasi pull request rimangono su GitHub finché non li elimini.
3. Correggi il gap una volta, nelle [istruzioni del project](#give-a-project-standing-context) o nell'[ambiente](#choose-an-environment-for-threads), quindi invia un thread prima di inviare il resto del lavoro di nuovo come nuovi thread.

<h3 id="claude-hasnt-responded">
  Claude non ha risposto
</h3>

La conversazione del project mostra un banner "Claude hasn't responded" quando Claude è in esecuzione ma le sue risposte non raggiungono il project. Fai clic su **Restart Claude** sul banner, o vai a **Project settings > General** e fai clic su **Restart** nella riga **Restart Claude**. Claude si riconnette alla conversazione; qualsiasi risposta che stava scrivendo viene persa, e i thread non sono interessati.

<h3 id="repository-access-errors">
  Errori di accesso al repository
</h3>

Tre messaggi significano che un thread o il project non può raggiungere uno dei suoi repository. Un thread del project ha bisogno dei [prerequisiti di GitHub](#check-the-prerequisites) anche quando le tue altre sessioni cloud clonano lo stesso repository senza problemi.

* **"Couldn't start the session — Claude doesn't have GitHub access to this project's repository"**, segnalato prima che il thread inizi, quando l'app Claude GitHub non è installata su quel repository, è sospesa o non è collegata all'account GitHub che hai connesso.
* **"Unable to access your repository"**, segnalato da un thread quando il suo clone fallisce: GitHub ha rifiutato il clone, il repository non è stato trovato sotto il nome che il project ha, o il branch da cui al thread è stato chiesto di iniziare non esiste.
* **"Claude can't access"** un repository, mostrato quando salvi i repository nella finestra di dialogo **New project** o **Project settings**. Il messaggio continua con un link di installazione e un link di riconnessione. Usa il link di installazione se l'app Claude GitHub non è su quel repository, e il link di riconnessione se lo è, poiché l'app GitHub può essere installata su GitHub senza essere collegata all'account che hai connesso a Claude. Se il messaggio dice che l'app GitHub è sospesa o non include questo repository, segui il suo link a GitHub per correggere.

Per correggere uno qualsiasi di essi, fai clic sul pulsante che il messaggio offre, come **Install GitHub App** o **Select repositories on GitHub**, quindi **Check again**. Quando il blocco è dal lato dell'organizzazione GitHub, come un proprietario che non ha approvato l'app o un elenco di indirizzi IP che esclude Claude, il messaggio mostra un link **See how to fix** invece. Se non c'è un pulsante, segui [Configura l'accesso a GitHub](#set-up-github-access), quindi invia un altro messaggio per riprovare.

<h3 id="usage-limit-reached">
  Un thread ha raggiunto il limite di utilizzo
</h3>

Quando un thread o la conversazione del project raggiunge il limite di cinque ore o settimanale del tuo piano, continua a riprovare da solo e continua quando il limite si ripristina. Mentre aspetta, il thread mostra **Service is busy** con "Claude is still retrying and will continue automatically." Non devi fare nulla perché il lavoro continui. Se preferisci che non usi la tua prossima finestra di utilizzo, fai clic su **Stop** nel thread, o [metti in pausa il project](#pause-archive-or-delete-a-project) per tenere ogni thread. Un thread che una routine ha avviato non aspetta: il suo turno si ferma con un errore di limite, e gli invii un messaggio dopo che il limite si ripristina.

[Errori di limite di utilizzo](/docs/it/errors#youve-hit-your-session-limit) spiegano i limiti e quando si ripristinano.

<h3 id="additional-usage-credits-are-required">
  Sono richiesti crediti di utilizzo aggiuntivi
</h3>

Un thread o la conversazione del project ha fatto una richiesta che il tuo piano copre solo con crediti di utilizzo, come una a un modello o una dimensione di contesto che il tuo piano non include, e i crediti di utilizzo non sono attivati per il tuo account. [Aggiungi crediti di utilizzo al tuo abbonamento](/docs/it/costs#add-usage-credits-to-your-subscription) copre chi può attivarli o comprarli su ogni piano. Una volta che i crediti sono disponibili, invia un altro messaggio per riprovare.

<h3 id="context-limit">
  Altri messaggi
</h3>

Questi messaggi nominano la loro stessa causa. La tabella fornisce il prossimo passo per ognuno.

| Messaggio                                                                                     | Cosa fare                                                                                                                                                                                                                                              |
| :-------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Unable to connect to repository" con "Claude couldn't reach GitHub to fetch your repository" | Aspetta un momento, quindi invia un altro messaggio per riprovare                                                                                                                                                                                      |
| "Unable to connect to repository" con "Claude couldn't access your repository or environment" | Il tuo account GitHub ha bisogno di accesso push al repository, e l'ambiente deve ancora esistere. Controlla entrambi in **Project settings > Environment**, quindi riprova                                                                            |
| "Couldn't show the setup proposal"                                                            | L'app che hai aperto è più vecchia delle **Setup recommendations** che Claude ha inviato. Aggiorna la pagina o riavvia l'app desktop, o chiedi a Claude di proporre di nuovo la configurazione                                                         |
| "The project's environment was removed"                                                       | Scegli un ambiente diverso in **Project settings > Environment**; il cambiamento si applica ai nuovi thread                                                                                                                                            |
| "Setup script failed"                                                                         | Fai clic su **Edit setup script** sull'errore, correggi lo script nell'ambiente, quindi invia un altro messaggio. [Setup script failed](/docs/it/web-quickstart#setup-script-failed) elenca le cause comuni                                                 |
| "Claude ran out of context on this turn"                                                      | Il thread ha riempito la sua finestra di contesto. Se il messaggio dice che il thread continua in una sessione fresca, continua da solo; altrimenti chiedi a Claude nella conversazione del project di avviare un nuovo thread per il lavoro rimanente |
| "Reached the turn limit"                                                                      | Il thread ha raggiunto il limite sui passaggi agentic che [`CLAUDE_CODE_MAX_TURNS`](/docs/it/env-vars) imposta. Invia un altro messaggio per continuare, o aumenta o rimuovi quella variabile dove è impostata                                              |

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Usa Claude Code nel cloud](/docs/it/claude-code-on-the-web): come funzionano le sessioni cloud dietro ogni thread, incluse le opzioni di accesso a GitHub e auto-fix sulle pull request
* [Configura cloud environment](/docs/it/cloud-environments): cambia cosa i thread possono raggiungere sulla rete, dai loro variabili di ambiente e credenziali API, e installa strumenti con uno script di configurazione
* [Automatizza il lavoro con le routine](/docs/it/routines): programmi, trigger e gestione per le routine, incluse quelle che Claude crea da un project
* [Gestisci più agenti con agent view](/docs/it/agent-view): esegui e traccia più sessioni sulla tua macchina quando il lavoro ha bisogno di strumenti o servizi che solo la tua macchina può raggiungere
* [Projects redesigned: from folder to conversation](https://claude.com/blog/projects-redesigned): l'annuncio di lancio, con il ragionamento dietro la trasformazione di un project in una conversazione con Claude
