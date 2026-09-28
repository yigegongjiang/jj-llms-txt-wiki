> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Iniziare con l'app desktop

> Installa Claude Code su desktop e avvia la tua prima sessione di codifica

L'app desktop ti offre Claude Code con un'interfaccia grafica costruita per eseguire più sessioni affiancate: una barra laterale per gestire il lavoro parallelo, un layout con trascinamento della selezione con terminale integrato e editor di file, revisione visiva dei diff, anteprima live dell'app, monitoraggio dei PR di GitHub con merge automatico e attività pianificate. Non è richiesto alcun terminale.

<CardGroup cols={3}>
  <Card title="Scarica per macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Build universale per Intel e Apple Silicon
  </Card>

  <Card title="Scarica per Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Per processori x64
  </Card>

  <Card title="Ottieni Claude per Linux (beta)" icon="linux" href="/docs/it/desktop-linux">
    apt o .deb per Ubuntu e Debian
  </Card>
</CardGroup>

Per Windows ARM64, scarica l'[installer ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). Su Linux, installa con apt; vedi [Claude Desktop su Linux](/docs/it/desktop-linux).

<Note>
  Claude Code richiede un [abbonamento Pro, Max, Team o Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Questa pagina illustra l'installazione dell'app e l'avvio della tua prima sessione. Se sei già configurato, consulta [Usa Claude Code Desktop](/docs/it/desktop) per il riferimento completo.

L'app desktop ha tre schede:

* **Chat**: Conversazione generale senza accesso ai file, simile a claude.ai.
* **Cowork**: Un agente autonomo in background che lavora su attività in una macchina virtuale sandbox con il suo ambiente, funzionando indipendentemente mentre tu fai altro. Le sessioni Cowork on-device eseguono la VM sul tuo computer; le sessioni Cowork remote eseguono invece una VM gestita da Anthropic.
* **Code**: Un assistente di codifica interattivo con accesso diretto ai tuoi file locali. Dipendendo dalla modalità di autorizzazione, approvi ogni modifica mentre Claude la propone o rivedi le modifiche dopo che Claude le ha apportate.

Chat e Cowork sono trattati nel [Centro di supporto Claude](https://support.claude.com/); l'installazione e la distribuzione dell'app desktop sono trattate negli [articoli di supporto di Claude Desktop](https://support.claude.com/en/collections/16163169-claude-desktop). Questa pagina si concentra sulla scheda **Code**.

<h2 id="install">
  Installa
</h2>

<Steps>
  <Step title="Installa e accedi">
    Su macOS e Windows, scarica il programma di installazione dai link sopra ed eseguilo. Su Linux, segui i passaggi di installazione in [Claude Desktop su Linux](/docs/it/desktop-linux). Avvia Claude dalla cartella Applicazioni su macOS, dal menu Start su Windows, o dal tuo launcher di applicazioni su Linux, quindi accedi con il tuo account Anthropic.
  </Step>

  <Step title="Apri la scheda Code">
    Fai clic sulla scheda **Code** al centro in alto. Se facendo clic su Code ti viene chiesto di eseguire l'upgrade, devi prima [sottoscrivere un piano a pagamento](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade). Se ti viene chiesto di accedere online, completa l'accesso e riavvia l'app. Se vedi un errore 403, consulta [risoluzione dei problemi di autenticazione](/docs/it/desktop#403-or-authentication-errors-in-the-code-tab).
  </Step>
</Steps>

L'app desktop include Claude Code. Non è necessario installare Node.js o la CLI separatamente. Per utilizzare `claude` dal terminale, installa la CLI separatamente. Consulta [Iniziare con la CLI](/docs/it/quickstart).

<h2 id="start-your-first-session">
  Avvia la tua prima sessione
</h2>

Con la scheda Code aperta, scegli un progetto e dai a Claude qualcosa da fare.

<Steps>
  <Step title="Scegli un ambiente e una cartella">
    Seleziona **Local** per eseguire Claude sulla tua macchina utilizzando direttamente i tuoi file. Fai clic su **Select folder** e scegli la directory del tuo progetto.

    <Tip>
      Inizia con un piccolo progetto che conosci bene. È il modo più veloce per vedere cosa può fare Claude Code. Su Windows, [Git](https://git-scm.com/downloads/win) deve essere installato affinché le sessioni locali funzionino. La maggior parte dei Mac include Git per impostazione predefinita.
    </Tip>

    Puoi anche selezionare:

    * **Cloud**: Esegui sessioni nel cloud che continuano anche se chiudi l'app. Le sessioni cloud utilizzano la stessa infrastruttura di [Claude Code sul web](/docs/it/claude-code-on-the-web).
    * **SSH**: Connettiti a una macchina remota tramite SSH, come i tuoi server, VM cloud o dev container. Desktop installa Claude Code sulla macchina remota automaticamente la prima volta che ti connetti.
    * **WSL** (Windows): Esegui la sessione all'interno di una [distribuzione WSL 2](/docs/it/desktop-wsl); Claude Code, gli strumenti e git vengono eseguiti sul lato Linux con percorsi nativi.
  </Step>

  <Step title="Scegli un modello">
    Seleziona un modello dal menu a discesa accanto al pulsante di invio. Vedi [modelli](/docs/it/model-config#available-models) per un confronto dei modelli disponibili. Puoi cambiare il modello in seguito dallo stesso menu a discesa.
  </Step>

  <Step title="Dì a Claude cosa fare">
    Digita quello che vuoi che Claude faccia:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    Una [sessione](/docs/it/desktop#work-in-parallel-with-sessions) è una conversazione con Claude sul tuo codice. Ogni sessione tiene traccia del suo proprio contesto e delle sue modifiche, quindi puoi lavorare su più attività senza che si interferiscano a vicenda.
  </Step>

  <Step title="Rivedi e accetta le modifiche">
    Quello che succede dopo dipende dalla [modalità di autorizzazione](/docs/it/desktop#choose-a-permission-mode) mostrata nel selettore accanto al pulsante di invio:

    * **Auto o Accept edits**: Claude applica le sue modifiche ai file e appare un indicatore come `+12 -1` in modo che tu possa esaminarle nella vista diff
    * **Manual**: Claude propone ogni modifica e attende la tua approvazione prima di applicarla. I tuoi file non vengono modificati finché non accetti, e se rifiuti una modifica, Claude ti chiede come vorresti procedere invece

    In modalità Manual, vedrai:

    1. Una [vista diff](/docs/it/desktop#review-changes-with-diff-view) che mostra esattamente cosa cambierà in ogni file
    2. Pulsanti Accept/Reject per approvare o rifiutare ogni modifica
    3. Aggiornamenti in tempo reale mentre Claude elabora la tua richiesta
  </Step>
</Steps>

<h2 id="now-what">
  E adesso?
</h2>

Hai fatto la tua prima modifica. Per il riferimento completo su tutto ciò che Desktop può fare, consulta [Usa Claude Code Desktop](/docs/it/desktop). Ecco alcune cose da provare dopo.

**Interrompi e cambia direzione.** Puoi reindirizzare Claude in qualsiasi momento. Fai clic sul pulsante di arresto per interrompere immediatamente, oppure digita una correzione e premi **Invio** per inviarla senza interrompere l'azione in corso. In entrambi i casi, non devi aspettare che finisca o ricominciare da capo.

**Dai a Claude più contesto.** Digita `@filename` nella casella del prompt per inserire un file specifico nella conversazione, allega immagini e PDF utilizzando il pulsante di allegato, oppure trascina e rilascia i file direttamente nel prompt. Più contesto ha Claude, migliori saranno i risultati. Consulta [Aggiungi file e contesto](/docs/it/desktop#add-files-and-context-to-prompts).

**Usa skills per attività ripetibili.** Digita `/` o fai clic su **+** → **Slash commands** per sfogliare [comandi integrati](/docs/it/commands), [skills personalizzate](/docs/it/skills) e skills di plugin. Le skills sono prompt riutilizzabili che puoi invocare ogni volta che ne hai bisogno, come checklist di revisione del codice o passaggi di distribuzione.

**Rivedi le modifiche prima di eseguire il commit.** Dopo che Claude modifica i file, appare un indicatore `+12 -1`. Fai clic su di esso per aprire la [vista diff](/docs/it/desktop#review-changes-with-diff-view), rivedi le modifiche file per file e commenta righe specifiche. Claude legge i tuoi commenti e revisionerai. Fai clic su **Review code** per far valutare a Claude i diff stesso e lasciare suggerimenti inline.

**Regola il controllo che hai.** La tua [modalità di autorizzazione](/docs/it/desktop#choose-a-permission-mode) imposta quanto Claude può fare senza chiedere l'approvazione:

* **Auto**: un classificatore esamina le azioni in background e blocca quelle rischiose invece di chiederti.
* **Manual**: Claude chiede prima di modificare i file o eseguire comandi.
* **Accept edits**: Claude accetta automaticamente le modifiche ai file per un'iterazione più veloce.
* **Plan**: Claude propone un approccio senza modificare alcun file, il che è utile prima di un grande refactor.

**Aggiungi plugin per più funzionalità.** Fai clic sul pulsante **+** accanto alla casella del prompt e seleziona **Plugins** per sfogliare e installare [plugin](/docs/it/desktop#install-plugins) che aggiungono skills, agenti, server MCP e altro ancora.

**Organizza il tuo workspace.** Trascina i riquadri chat, diff, terminal, file e browser in qualsiasi layout desideri. Apri il terminal con **Ctrl+\`** per eseguire comandi insieme alla tua sessione, oppure fai clic su un percorso di file per aprirlo nel riquadro file. Consulta [Organizza il tuo workspace](/docs/it/desktop#arrange-your-workspace).

**Visualizza l'anteprima della tua app.** Quando esegui il tuo dev server nel desktop, la tua app si apre nel riquadro Browser, che può anche [aprire siti esterni](/docs/it/desktop#browse-external-sites). Claude può visualizzare l'app in esecuzione, testare endpoint, ispezionare i log e iterare su ciò che vede. Consulta [Visualizza l'anteprima della tua app](/docs/it/desktop#preview-your-app).

**Traccia la tua pull request.** Dopo aver aperto una PR, Claude Code monitora i risultati dei controlli CI e può correggere automaticamente gli errori o unire la PR una volta che tutti i controlli passano. Consulta [Monitora lo stato della pull request](/docs/it/desktop#monitor-pull-request-status).

**Metti Claude in programma.** Configura [attività pianificate](/docs/it/desktop-scheduled-tasks) per eseguire Claude automaticamente su base ricorrente: una revisione del codice giornaliera ogni mattina, un audit delle dipendenze settimanale, o un briefing che estrae dai tuoi strumenti connessi.

**Scala quando sei pronto.** Apri [sessioni parallele](/docs/it/desktop#work-in-parallel-with-sessions) dalla barra laterale per lavorare su più attività contemporaneamente, ognuna nel suo Git worktree, e apri il [riquadro tasks](/docs/it/desktop#watch-background-tasks) per guardare i subagenti e i comandi in background che una sessione ha in esecuzione. Apri una [side chat](/docs/it/desktop#ask-a-side-question-without-derailing-the-session) per fare una domanda senza deviare il thread principale. Invia [lavoro a lunga esecuzione al cloud](/docs/it/desktop#run-long-running-tasks-remotely) in modo che continui anche se chiudi l'app, oppure [continua una sessione sul web o nel tuo IDE](/docs/it/desktop#continue-in-another-surface) se un'attività richiede più tempo del previsto. [Connetti strumenti esterni](/docs/it/desktop#extend-claude-code) come GitHub, Slack e Linear per riunire il tuo flusso di lavoro.

<h2 id="what’s-next">
  Cosa fare dopo
</h2>

* [Usa Claude Code Desktop](/docs/it/desktop): modalità di autorizzazione, sessioni parallele, visualizzazione diff, connettori e configurazione aziendale
* [Provieni dalla CLI?](/docs/it/desktop#coming-from-the-cli): esegui Desktop e la CLI sullo stesso progetto, e confronta le funzionalità, gli equivalenti dei flag e ciò che non è disponibile in Desktop
* [Risoluzione dei problemi](/docs/it/desktop#troubleshooting): soluzioni agli errori comuni e ai problemi di configurazione
* [Best practices](/docs/it/best-practices): suggerimenti per scrivere prompt efficaci e ottenere il massimo da Claude Code
* [Flussi di lavoro comuni](/docs/it/common-workflows): tutorial per il debug, il refactoring, i test e altro ancora
