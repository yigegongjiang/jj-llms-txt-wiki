> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usa Claude Code in VS Code

> Installa e configura l'estensione Claude Code per VS Code. Ottieni assistenza di codifica con IA con diff inline, @-mention, revisione del piano e scorciatoie da tastiera.

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="Editor VS Code con il pannello dell'estensione Claude Code aperto sul lato destro, che mostra una conversazione con Claude" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

L'estensione VS Code fornisce un'interfaccia grafica nativa per Claude Code, integrata direttamente nel vostro IDE. Questo è il modo consigliato per utilizzare Claude Code in VS Code.

Con l'estensione, potete rivedere e modificare i piani di Claude prima di accettarli, accettare automaticamente le modifiche mentre vengono apportate, @-mention file con intervalli di righe specifici dalla vostra selezione, accedere alla cronologia delle conversazioni e aprire più conversazioni in schede o finestre separate.

<h2 id="prerequisites">
  Prerequisiti
</h2>

Prima di installare, assicurati di avere:

* VS Code 1.94.0 o superiore
* Un account Anthropic: qualsiasi abbonamento Claude a pagamento (Pro, Max, Team o Enterprise) o un account Claude Console funziona, e non è richiesta alcuna chiave API. Accederai [con questo account](/docs/it/authentication#log-in-to-claude-code) quando aprirai l'estensione per la prima volta. Se accedi a Claude tramite un provider di terze parti come Amazon Bedrock o Google Cloud's Agent Platform, consulta [Usa provider di terze parti](#use-third-party-providers) per le istruzioni di configurazione.

<Tip>
  L'estensione include la propria copia della CLI (interfaccia della riga di comando) per il pannello di chat. Per eseguire `claude` nel terminale integrato di VS Code, hai anche bisogno dell'[installazione CLI standalone](/docs/it/setup). Consulta [Estensione VS Code vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) per i dettagli.
</Tip>

<h2 id="install-the-extension">
  Installa l'estensione
</h2>

Fai clic sul link per il tuo IDE per installare direttamente:

* [Installa per VS Code](vscode:extension/anthropic.claude-code)
* [Installa per Cursor](cursor:extension/anthropic.claude-code)

Oppure in VS Code, premi `Cmd+Shift+X` (Mac) o `Ctrl+Shift+X` (Windows/Linux) per aprire la visualizzazione Estensioni, cerca "Claude Code" e fai clic su **Installa**.

L'estensione si installa anche in altri fork di VS Code come Devin Desktop o Kiro. Cerca "Claude Code" nella visualizzazione Estensioni dell'editor, oppure installa dal [registro Open VSX](https://open-vsx.org/extension/Anthropic/claude-code). Se il tuo editor non riesce a installare l'estensione, [installa la CLI](/docs/it/quickstart) ed esegui `claude` nel suo terminale integrato. La CLI funziona in qualsiasi terminale.

<Note>Se l'estensione non appare dopo l'installazione, riavvia VS Code o esegui "Developer: Reload Window" dalla Tavolozza dei comandi.</Note>

<h2 id="get-started">
  Iniziare
</h2>

Una volta installato, puoi iniziare a utilizzare Claude Code attraverso l'interfaccia di VS Code:

<Steps>
  <Step title="Apri il pannello Claude Code">
    In tutto VS Code, l'icona Spark indica Claude Code: <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Icona Spark" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    Il modo più veloce per aprire Claude è fare clic sull'icona Spark nella **Barra degli strumenti dell'editor** (angolo in alto a destra dell'editor). L'icona appare solo quando hai un file aperto.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="Editor VS Code che mostra l'icona Spark nella Barra degli strumenti dell'editor" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Altri modi per aprire Claude Code:

    * **Activity Bar**: fai clic sull'icona Spark nella barra laterale sinistra per aprire l'elenco delle sessioni. Fai clic su qualsiasi sessione per aprirla nella tua [posizione preferita](#extension-settings), o avvia una nuova. Questa icona è sempre visibile nella Activity Bar.
    * **Command Palette**: `Cmd+Shift+P` (Mac) o `Ctrl+Shift+P` (Windows/Linux), digita "Claude Code" e seleziona un'opzione come "Open in New Tab"
    * **Status Bar**: se hai impostato [`preferredLocation`](#extension-settings) su `sidebar`, o hai aperto Claude con **Claude Code: Open in Side Bar**, fai clic su **✻ Claude Code** nell'angolo in basso a destra della finestra. Questo funziona anche quando nessun file è aperto.

    Puoi trascinare il pannello Claude per riposizionarlo ovunque in VS Code. Vedi [Personalizza il tuo flusso di lavoro](#customize-your-workflow) per i dettagli.
  </Step>

  <Step title="Accedi">
    La prima volta che apri il pannello, appare una schermata di accesso. Fai clic su **Sign in** e completa l'autorizzazione nel tuo browser.

    Se vedi **Not logged in · Please run /login** in seguito, l'estensione riaprirà automaticamente la schermata di accesso. Se non appare, ricarica la finestra dalla Command Palette con **Developer: Reload Window**.

    Se hai `ANTHROPIC_API_KEY` impostato nella tua shell ma vedi ancora il prompt di accesso, VS Code potrebbe non aver ereditato l'ambiente della tua shell. Avvia VS Code da un terminale con `code .` in modo che erediti le tue variabili di ambiente, oppure accedi con il tuo account Claude.

    Dopo aver effettuato l'accesso, appare una checklist **Learn Claude Code**. Completa ogni elemento facendo clic su **Show me**, oppure chiudila con la X. Per riaprirla in seguito, deseleziona **Hide Onboarding** nelle impostazioni di VS Code in Extensions → Claude Code.
  </Step>

  <Step title="Invia un prompt">
    Chiedi a Claude di aiutarti con il tuo codice o i tuoi file, che si tratti di spiegare come funziona qualcosa, eseguire il debug di un problema o apportare modifiche.

    <Tip>Claude vede automaticamente il testo selezionato. Premi `Option+K` (Mac) / `Alt+K` (Windows/Linux) per inserire anche un riferimento @-mention (come `@file.ts#5-10`) nel tuo prompt.</Tip>

    Ecco un esempio di domanda su una riga particolare in un file:

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="Editor VS Code con le righe 2-3 selezionate in un file Python, e il pannello Claude Code che mostra una domanda su quelle righe con un riferimento @-mention" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Rivedi le modifiche">
    Quello che vedi dipende dalla [modalità di autorizzazione](/docs/it/permission-modes#which-mode-a-session-starts-in) mostrata in fondo alla casella del prompt:

    * In modalità Auto o Edit automatically, Claude modifica la maggior parte dei file nel tuo workspace senza chiedere.
    * In modalità Manual, quando Claude vuole modificare un file, mostra un confronto affiancato dell'originale e delle modifiche proposte, quindi chiede l'autorizzazione. Puoi accettare, rifiutare o dire a Claude cosa fare invece. Se modifichi il contenuto proposto direttamente nella vista diff prima di accettare, Claude viene informato che l'hai modificato in modo che non assuma che il file corrisponda alla sua proposta originale.

          <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code che mostra un diff delle modifiche proposte da Claude con un prompt di autorizzazione che chiede se effettuare la modifica" width="3292" height="1876" data-path="images/vs-code-edits.png" />

    Per rivedere una modifica proposta una modifica alla volta, utilizza i pulsanti **Accept this change** e **Reject this change** sotto ogni modifica nel diff. Rifiutare una modifica la ripristina nel contenuto proposto; accettarla la contrassegna come revisionata. Accettare o rifiutare l'intero file completa comunque la revisione. Un diff con più di 100 modifiche si apre senza i pulsanti per modifica, quindi revisionalo come un intero file. La revisione per modifica richiede Claude Code v2.1.275 o successivo.

    Le stesse azioni sono disponibili al cursore dal menu contestuale dell'editor e dalla Command Palette come **Claude Code: Accept Change at Cursor** e **Claude Code: Reject Change at Cursor**.
  </Step>
</Steps>

Per altre idee su cosa puoi fare con Claude Code, vedi [Flussi di lavoro comuni](/docs/it/common-workflows).

<Tip>
  Esegui "Claude Code: Open Walkthrough" dalla Command Palette per una visita guidata delle nozioni di base.
</Tip>

<h2 id="use-the-prompt-box">
  Usa la casella di prompt
</h2>

La casella di prompt supporta diverse funzionalità:

* **Modalità di autorizzazione**: fai clic sull'indicatore di modalità nella parte inferiore della casella di prompt per cambiare le modalità di autorizzazione. Nei piani Pro, Max e Team, Auto è la modalità di autorizzazione predefinita all'avvio. Consulta [come l'estensione sceglie la modalità di autorizzazione iniziale](/docs/it/permission-modes#switch-permission-modes) per sapere cosa la cambia e ogni modalità di autorizzazione che l'indicatore offre.
  * **Auto**: un classificatore esamina la maggior parte delle azioni invece di chiederti. Consulta [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) per sapere cosa esamina e blocca.
  * **Manual**: Claude chiede l'autorizzazione prima delle modifiche ai file e della maggior parte dei comandi shell.
  * **Plan**: Claude descrive cosa farà e attende l'approvazione prima di apportare modifiche. VS Code apre automaticamente il piano come documento Markdown completo dove puoi aggiungere commenti inline per fornire feedback prima che Claude inizi.

    Puoi anche digitare `/plan` nella casella di prompt. Richiede Claude Code v2.1.280 o successivo.

    * `/plan`: passa alla modalità plan. Se sei già in modalità plan, mostra il piano attuale invece.
    * `/plan` con un compito, come `/plan fix the auth bug`: passa alla modalità plan e inizia a pianificare quel compito.
    * `/plan open`: quando sei già in modalità plan, apre il file del piano nell'editor.
  * **Edit automatically**: Claude apporta modifiche senza chiedere.
* **Model**: seleziona **Switch model…** dal menu dei comandi per cambiare il modello durante la sessione. Puoi anche fare clic sul nome del modello nella parte inferiore della casella di prompt per aprire lo stesso selettore.

  Quando il modello attuale supporta [livelli di sforzo](/docs/it/model-config#adjust-effort-level), il selettore mostra anche una riga **Effort** e il pulsante del nome del modello mostra il livello selezionato. Quando scegli un livello diverso da `max`, Claude Code lo salva per il modello attuale come predefinito, sotto [`modelSettings`](/docs/it/settings-reference#modelsettings) nelle impostazioni utente; `max` si applica solo alla sessione corrente. Il pulsante del nome del modello e la riga **Effort** richiedono Claude Code v2.1.257 o successivo.
* **Command menu**: fai clic su `/` o digita `/` per aprire il menu dei comandi. Le opzioni includono l'allegazione di file, il cambio di modelli e l'attivazione del pensiero esteso.

  La sezione Customize fornisce accesso ai server MCP, comandi, stili di output, hooks, memoria, istruzioni, autorizzazioni e plugin. Gli elementi con un'icona di terminale si aprono nel terminale integrato.

  * Per sfogliare comandi come `/usage` o [`/remote-control`](/docs/it/remote-control), seleziona **Slash commands** nella sezione Customize. Una finestra di dialogo li elenca con una casella di filtro. Scegline uno per eseguirlo. Digitare `/` nella casella di prompt suggerisce comunque i comandi inline. Richiede Claude Code v2.1.257 o successivo.

    Digitare `/skills` apre anche questa finestra di dialogo. Ogni riga di [skill](/docs/it/skills) mostra la sua [visibilità](/docs/it/skills#override-skill-visibility-from-settings), come **On** o **Name only**. Fai clic sulla visibilità per cambiarla, tranne sulle righe contrassegnate come **locked**, come le skill dei plugin. La scorciatoia `/skills` e i controlli di visibilità richiedono Claude Code v2.1.280 o successivo.
  * Seleziona **Output styles** nella sezione Customize per scegliere uno [stile di output](/docs/it/output-styles), inclusi i tuoi stili personalizzati. Richiede Claude Code v2.1.257 o successivo.

    Per creare uno stile personalizzato, seleziona **Build a custom style** dal menu **Output styles**. Claude Code scrive il [file di stile](/docs/it/output-styles#create-a-custom-output-style) per te a livello di progetto o utente. Richiede Claude Code v2.1.261 o successivo.
  * Seleziona **Hooks** nella sezione Customize per visualizzare gli [hooks](/docs/it/hooks) caricati nella sessione, raggruppati per evento. Puoi aggiungere, modificare o rimuovere gli hooks salvati nei tuoi file di impostazioni utente, progetto e locale. Gli hooks da altre fonti, come impostazioni gestite o plugin, sono di sola lettura. Richiede Claude Code v2.1.269 o successivo.
  * Seleziona **Permissions** nella sezione Customize per visualizzare le [regole di autorizzazione](/docs/it/permissions) della sessione, raggruppate in Allow, Ask e Deny. Puoi aggiungere regole ai tuoi file di impostazioni utente, progetto o locale e rimuovere le regole salvate lì. Le regole da altre fonti, come impostazioni gestite o approvazioni effettuate solo per questa sessione, sono di sola lettura. Richiede Claude Code v2.1.269 o successivo.
  * Seleziona **Memory** nella sezione Customize per attivare o disattivare la [memoria automatica](/docs/it/memory#auto-memory). Mentre è attiva, puoi anche sfogliare i ricordi che Claude ha salvato e rivelare le cartelle che li archiviano nel tuo file manager. Richiede Claude Code v2.1.274 o successivo.

    Fai clic su un ricordo salvato per leggerlo nella finestra di dialogo, dove puoi modificare il testo, eliminare il ricordo, o aprire il suo file nell'editor. La visualizzazione, la modifica e l'eliminazione di un ricordo nella finestra di dialogo richiedono Claude Code v2.1.275 o successivo.
  * Seleziona **Instructions** nella sezione Customize per modificare i [file CLAUDE.md](/docs/it/memory#claude-md-files) che Claude legge. Scegli un file per aprirlo nell'editor. Se il file non esiste ancora, Claude Code lo crea prima. Richiede Claude Code v2.1.274 o successivo.
  * Seleziona **Status** nella sezione Customize, o digita `/status`, per controllare la versione di Claude Code della sessione, l'account, il modello e i dettagli del server MCP. Richiede Claude Code v2.1.280 o successivo.
  * Seleziona **Sandbox** nella sezione Customize, o digita `/sandbox`, per vedere se i comandi Bash di Claude vengono eseguiti [in sandbox](/docs/it/sandboxing). Puoi cambiare la modalità sandbox e aggiungere [comandi esclusi](/docs/it/settings-reference#sandbox-excludedcommands) lì. Richiede Claude Code v2.1.280 o successivo.
  * Seleziona **Claude in Chrome** nella sezione Customize, o digita `/chrome`, per controllare e gestire la connessione [Claude in Chrome](/docs/it/chrome). Entrambi richiedono l'accesso con un account claude.ai. Richiede Claude Code v2.1.280 o successivo.
  * Seleziona **Export conversation** nella sezione Context, o digita `/export`, per copiare la conversazione come testo semplice o salvarla in un file. Aggiungi un nome di file, come `/export notes.txt`, per saltare la finestra di dialogo e scegliere dove salvare il file. Richiede Claude Code v2.1.280 o successivo.
  * La sezione Settings include **Enable Remote Control for all sessions**, che imposta [`remoteControlAtStartup`](/docs/it/settings-reference#remotecontrolatstartup) per controllare se [le nuove sessioni interattive si connettono a Remote Control automaticamente](/docs/it/remote-control#enable-remote-control-for-all-sessions). Richiede Claude Code v2.1.203 o successivo.

    Quando attivi o disattivi l'interruttore in una finestra di VS Code, la modifica si applica alle sessioni già aperte in quella finestra di VS Code, non solo alle sessioni che avvii successivamente. Se lo disattivi, le sessioni aperte si disconnettono. Con Claude Code v2.1.261 o successivo, la modifica raggiunge anche le sessioni aperte nelle altre finestre di VS Code.
  * La sezione Settings include anche **Focus view**, che nasconde le chiamate di strumenti, i risultati degli strumenti e il pensiero dietro righe espandibili, lasciando i tuoi prompt e le risposte di Claude. Attivalo lì, con `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux), o dalla Tavolozza dei comandi con **Claude Code: Toggle Focus view**. La modifica si applica a ogni sessione aperta e persiste tra le sessioni. Richiede Claude Code v2.1.221 o successivo.

    L'elenco di cose da fare più recente di Claude rimane visibile, così come il testo di una domanda in sospeso da Claude; questo richiede Claude Code v2.1.225 o successivo. Mentre Claude esegue [subagenti](/docs/it/sub-agents), le righe di progresso in tempo reale con la loro attività più recente appaiono sotto il gruppo di chiamate di strumenti che le ha avviate. Questo richiede Claude Code v2.1.269 o successivo.
  * Per disconnetterti dal tuo account Anthropic, seleziona **Sign out** nella sezione Settings, o digita `/logout`. Su un [provider di terze parti](#use-third-party-providers), il menu non offre nessuno dei due. Richiede Claude Code v2.1.277 o successivo.
  * Per segnalare un bug, fai clic su **Report a problem** nella parte inferiore del menu, o digita `/bug` o `/feedback` con una descrizione facoltativa che precompila il rapporto. Quando invii il rapporto e sei connesso ad Anthropic su una connessione di prima parte, Claude Code lo invia ad Anthropic. Su un provider di terze parti, o senza credenziali Anthropic, la finestra di dialogo si apre comunque, ma l'invio mostra un errore e non invia nulla: a differenza di `/bug` della CLI, l'estensione non scrive un archivio locale. Richiede Claude Code v2.1.229 o successivo.

    Se la politica della tua organizzazione disattiva il feedback sui prodotti, **Report a problem** non appare nel menu, e `/bug` e `/feedback` mostrano un avviso `Feedback is turned off by your organization's policy or this environment's settings.` invece di aprire il rapporto.
* **Side questions**: digita `/btw` seguito da una domanda per chiedere informazioni sulla tua sessione [senza aggiungere alla conversazione](/docs/it/interactive-mode#side-questions-with-%2Fbtw). La risposta si apre in un pannello accanto alla chat, dove puoi fare domande di follow-up. Il thread sopravvive ai ricaricamenti della finestra. Claude Code mantiene i 20 scambi più recenti e scade i thread archiviati secondo la pianificazione [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays), purché Claude Code possa [determinare in sicurezza il periodo di conservazione](/docs/it/claude-directory#cleaned-up-automatically). Per cancellare un thread, fai clic sull'icona del cestino nel pannello. Richiede Claude Code v2.1.227 o successivo.
* **Copy a response**: passa il mouse su una risposta e fai clic su **Copy response** per copiarla negli appunti, o digita `/copy` per copiare l'ultima risposta. `/copy 2` copia la penultima. Richiede Claude Code v2.1.277 o successivo.
* **Context indicator**: la casella di prompt mostra quanto della finestra di contesto di Claude stai utilizzando. Claude compatta automaticamente quando necessario, oppure puoi eseguire `/compact` manualmente.
* **Prompt cache clock**: un'icona di orologio accanto all'indicatore di contesto stima quanto tempo rimane alla [prompt cache](/docs/it/prompt-caching) della conversazione prima che scada. Fa il conto alla rovescia dalla [durata](/docs/it/prompt-caching#cache-lifetime) della cache di cinque minuti o un'ora, e ogni risposta che utilizza la cache riavvia il conto alla rovescia. A parte la compattazione, le [azioni che invalidano la cache](/docs/it/prompt-caching#actions-that-invalidate-the-cache) non ripristinano l'orologio, quindi può ancora mostrare minuti rimasti dopo che cambi modelli.
  * Fino a quando il conto alla rovescia non termina, l'icona mostra i minuti rimasti, come **12m**.
  * Quando il conto alla rovescia termina, i minuti scompaiono e l'icona diventa rossa, o il colore di errore del tuo tema, fino alla risposta successiva. La cache probabilmente è scaduta, quindi aspettati una risposta più lenta e più costosa al tuo prossimo messaggio mentre la cache si ricostruisce. Se la durata di cinque minuti continua a scadere tra i tuoi messaggi, consulta [Scegli il TTL tu stesso](/docs/it/prompt-caching#choose-the-ttl-yourself).
  * Subito dopo che la conversazione è stata [compattata](/docs/it/prompt-caching#compacting-the-conversation), l'icona diventa anche rossa senza minuti fino alla risposta successiva, perché la cache non copre ancora la conversazione compattata.
* **Agent map**: quando la conversazione include [subagenti](/docs/it/sub-agents), un conteggio di agenti come **2 agents** appare nella parte inferiore della casella di prompt. Il suo punto mostra se un subagente sta lavorando o in attesa della tua autorizzazione.

  Fai clic sul conteggio degli agenti per aprire la mappa degli agenti, che disegna i subagenti della conversazione come un albero sotto l'agente principale, ognuno con il suo stato, tempo trascorso e conteggio dei token. Fai clic su un subagente per vedere il suo prompt e le chiamate di strumenti, aprire la sua trascrizione di sola lettura, o fermarlo mentre è in esecuzione. Richiede Claude Code v2.1.269 o successivo.

  La mappa elenca anche gli altri [compiti in background](/docs/it/tools-reference#background-commands) della sessione, come comandi shell in background e [monitor](/docs/it/tools-reference#monitor-tool), sotto gli agenti. Fai clic su una riga per aprire la scheda del compito e fermarlo lì.

  Per aprire la mappa quando non è visualizzato alcun conteggio di agenti, ad esempio quando Claude ha avviato una shell in background ma nessun subagente, digita `/tasks` nella casella di prompt. I compiti in background nella mappa e il `/tasks` digitato richiedono Claude Code v2.1.277 o successivo.
* **Extended thinking**: consente a Claude di dedicare più tempo al ragionamento su problemi complessi. Attivalo tramite il menu dei comandi (`/`). Il ragionamento di Claude appare nella conversazione come blocchi compressi: fai clic su un blocco per leggerlo, o premi `Ctrl+O` per espandere o comprimere ogni blocco di pensiero nella sessione. Consulta [Extended thinking](/docs/it/model-config#extended-thinking) per i dettagli.
* **Multi-line input**: premi `Shift+Enter` per aggiungere una nuova riga senza inviare. Questo funziona anche nell'input di testo libero "Other" delle finestre di dialogo delle domande.

<h3 id="reference-files-and-folders">
  Riferimenti a file e cartelle
</h3>

Usa le menzioni @-mention per fornire a Claude il contesto su file o cartelle specifiche. Quando digiti `@` seguito da un nome di file o cartella, Claude legge quel contenuto e può rispondere a domande su di esso o apportare modifiche. Claude Code supporta la corrispondenza fuzzy, quindi puoi digitare nomi parziali per trovare quello che ti serve:

```text wrap theme={null}
Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
What's in @src/components/ (include a trailing slash for folders)
```

Per i PDF di grandi dimensioni, puoi chiedere a Claude di leggere pagine specifiche invece dell'intero file: una singola pagina, un intervallo come pagine 1-10, o un intervallo aperto come pagina 3 in poi.

Quando selezioni il testo nell'editor, Claude può vedere il tuo codice evidenziato automaticamente. Il piè di pagina della casella di prompt mostra quante righe sono selezionate. Premi `Option+K` (Mac) / `Alt+K` (Windows/Linux) per inserire una menzione @-mention con il percorso del file e i numeri di riga (ad es. `@app.ts#5-10`). Fai clic sulla **X** sull'indicatore di selezione per rimuoverlo in modo che Claude non riceva la selezione. L'indicatore riappare quando selezioni altro testo.

L'estensione trattiene il testo selezionato da alcuni file. Quando il file si trova all'interno del tuo workspace e corrisponde alle tue impostazioni `files.exclude` o `search.exclude`, Claude riceve al massimo il percorso del file e non il testo che hai selezionato. Lo stesso vale per un file che git ignora, purché l'impostazione `search.useIgnoreFiles` di VS Code e l'impostazione [`respectGitIgnore`](#extension-settings) dell'estensione siano entrambe attive, il che è l'impostazione predefinita. Questo filtro copre solo il pannello della chat: quando Claude Code viene eseguito nel terminale integrato, la CLI invia il tuo testo selezionato indipendentemente dal file, quindi aggiungi una [regola di negazione `Read`](#the-built-in-ide-mcp-server) per impedire che i contenuti di un file raggiungano Claude lì.

Claude vede anche quale file hai aperto nell'editor, anche quando nulla è selezionato, e la casella di prompt mostra il suo nome. Per aggiungere solo il testo selezionato, disattiva l'[impostazione Attach Open File](vscode://settings/claudeCode.attachOpenFile). L'impostazione richiede Claude Code v2.1.271 o successivo.

Puoi anche allegare immagini e file al tuo messaggio:

* Per allegare un'immagine, incollala dagli appunti nella casella di prompt.
* Per allegare file, tieni premuto `Shift` mentre li trascini nella casella di prompt.
* Per rimuovere un allegato dal contesto, fai clic sulla X su di esso.

<h3 id="paste-text">
  Incolla testo
</h3>

Il testo che incolla rimane visibile nella casella di prompt, piuttosto che comprimersi in un segnaposto come fa [nel terminale](/docs/it/terminal-config#paste-large-content). Nelle sessioni in cui Claude Code [contrassegna il testo incollato](/docs/it/terminal-config#how-claude-treats-pasted-text), Claude vede comunque un grande incollamento come testo che hai incollato piuttosto che digitato.

Claude Code rimuove anche [caratteri Unicode invisibili](/docs/it/interactive-mode#invisible-characters-in-prompts) dal testo che incolla nella casella di prompt e da qualsiasi altra cosa tu invii:

* Se un avviso come `Removed 3 invisible characters from the pasted text` appare quando incoli, il testo è entrato senza quei caratteri.
* Se un avviso sui caratteri rimossi appare quando invii, nulla è stato inviato. Il testo pulito è di nuovo nella casella di prompt. Invia di nuovo per inviare il testo come mostrato.

<h3 id="resume-past-conversations">
  Riprendi conversazioni passate
</h3>

Fai clic sul pulsante **Session history** nella parte superiore del pannello Claude Code per accedere alla cronologia delle conversazioni. Puoi cercare per parola chiave o sfogliare per ora.

Fai clic su qualsiasi conversazione per riprenderla con la cronologia completa dei messaggi. Se la conversazione è già aperta in un'altra scheda della finestra corrente, facendo clic su di essa passerai a quella scheda. Per ulteriori informazioni sulla ripresa delle sessioni, consulta [Manage sessions](/docs/it/sessions).

* **Session titles**: le nuove sessioni ricevono titoli generati dall'IA in base al tuo primo messaggio.
* **Rename and archive**: passa il mouse su una sessione per rivelare queste azioni. Rinomina per darle un titolo descrittivo, o archivia per spostarla nel gruppo **Archived sessions** nella parte inferiore dell'elenco.

Per impostazione predefinita, una sessione senza attività per 14 giorni si sposta automaticamente in **Archived sessions**, a meno che non sia aperta, non letta o in un [gruppo](#organize-sessions-into-groups). L'archiviazione automatica richiede Claude Code v2.1.265 o successivo. Per modificare il periodo o disattivarlo, apri l'[impostazione Archive Inactive Sessions](vscode://settings/claudeCode.archiveInactiveSessions) e seleziona un numero di giorni o **Never**.

Per ripristinare una sessione archiviata, espandi **Archived sessions** e fai clic su **Unarchive session**. Per ripristinare ogni sessione archiviata contemporaneamente, passa il mouse sull'intestazione **Archived sessions** nell'elenco delle sessioni nella Activity Bar e fai clic sulla sua icona di ripristino, che richiede Claude Code v2.1.277 o successivo. Prima della v2.1.257, l'azione era **Delete session**, che nascondeva una sessione senza modo di ripristinarla. Le sessioni che hai eliminato allora appaiono sotto **Archived sessions** dopo l'aggiornamento.

Quando la conversazione che riprendi è terminata in modalità plan, Claude Code ripristina la modalità plan. Richiede Claude Code v2.1.246 o successivo. Claude Code non la ripristina in due casi:

* L'estensione [sceglie la modalità di autorizzazione iniziale](/docs/it/permission-modes#switch-permission-modes) da `claudeCode.initialPermissionMode` o una scelta che si trasporta da una conversazione precedente
* Hai `claudeCode.claudeProcessWrapper` configurato

<h3 id="resume-cloud-sessions-from-claude-ai">
  Riprendi sessioni cloud da Claude.ai
</h3>

Se usi [Claude Code sul web](/docs/it/claude-code-on-the-web), puoi riprendere quelle sessioni cloud direttamente in VS Code. Questo richiede l'accesso con **Claude.ai Subscription**, non Anthropic Console.

<Steps>
  <Step title="Open session history">
    Fai clic sul pulsante **Session history** nella parte superiore del pannello Claude Code.
  </Step>

  <Step title="Select the Web tab">
    La finestra di dialogo mostra due schede: Local e Web. Fai clic su **Web** per vedere le sessioni da claude.ai.
  </Step>

  <Step title="Select a session to resume">
    Sfoglia o cerca le tue sessioni cloud. Fai clic su qualsiasi sessione per scaricarla e continuare la conversazione localmente.
  </Step>
</Steps>

<Note>
  Solo le sessioni web avviate con un repository GitHub appaiono nella scheda Web. Il ripristino carica la cronologia della conversazione localmente; le modifiche non vengono sincronizzate di nuovo a claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Controlla account e utilizzo
</h3>

Esegui `/usage` per aprire la finestra di dialogo Account & usage. Mostra il tuo account connesso, e l'utilizzo che riporta differisce in base all'accesso:

* **Piano claude.ai**: barre di utilizzo per i limiti del tuo piano, come la sessione corrente e la settimana. Ogni barra mostra quanto tempo rimane prima che il suo limite si ripristini.

  La finestra di dialogo suddivide anche ciò che contribuisce ai limiti del tuo piano. Contrassegna i comportamenti che rappresentano il 10% o più dell'utilizzo recente, come mancate cache, contesto lungo e sessioni pesanti di subagent o altamente parallele, ognuna con un suggerimento per ridurlo. Le tabelle di attribuzione mostrano quanto utilizzo è provenuto da ogni skill, subagent, plugin e server MCP.

  Usa l'interruttore Day e Week per passare tra le ultime 24 ore e gli ultimi 7 giorni. Le cifre sono approssimative e calcolate dalle sessioni locali su questa macchina, quindi l'utilizzo da altri dispositivi o da claude.ai non è incluso.
* **Altri accessi**: quando i limiti del piano non si applicano al tuo accesso, ad esempio su un [provider di terze parti](#use-third-party-providers) o con una chiave API, la sezione Usage mostra il costo della sessione e l'utilizzo dei token invece. Il `/usage` della CLI mostra gli stessi totali nel suo [blocco Session](/docs/it/costs#track-your-costs). L'elenco delle sessioni nella Activity Bar mostra anche i totali della sessione attiva sotto la sua intestazione **Account & usage**. Richiede Claude Code v2.1.277 o successivo.

Per ulteriori informazioni sul tracciamento e la riduzione dell'utilizzo, consulta [Track your costs](/docs/it/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Personalizza il tuo flusso di lavoro
</h2>

Puoi riposizionare il pannello Claude, eseguire più conversazioni, organizzare l'elenco delle sessioni in gruppi o passare alla modalità terminale.

<h3 id="choose-where-claude-lives">
  Scegli dove Claude si trova
</h3>

Puoi trascinare il pannello Claude per riposizionarlo ovunque in VS Code. Afferra la scheda o la barra del titolo del pannello e trascinalo in:

* **Barra laterale secondaria**: il lato destro della finestra. Mantiene Claude visibile mentre codifichi.
* **Barra laterale primaria**: la barra laterale sinistra con icone per Explorer, Search, ecc.
* **Area editor**: apre Claude come scheda accanto ai tuoi file. Utile per attività secondarie.

Quando Claude apre una scheda in un nuovo gruppo di editor, l'estensione blocca quel gruppo, quindi i file che apri mentre la scheda Claude è attiva vanno in un altro gruppo invece che accanto ad essa.

Per impedire all'estensione di bloccare i gruppi, disattiva l'[impostazione Lock Editor Groups](vscode://settings/claudeCode.lockEditorGroups). I gruppi già bloccati rimangono bloccati finché non li sblocchi. L'impostazione richiede Claude Code v2.1.274 o successivo.

<Tip>
  Usa la barra laterale per la tua sessione Claude principale e apri schede aggiuntive per attività secondarie. Claude ricorda la tua posizione preferita. L'icona dell'elenco delle sessioni della Activity Bar è separata dal pannello Claude: l'elenco delle sessioni è sempre visibile nella Activity Bar, mentre l'icona del pannello Claude appare lì solo quando il pannello è ancorato alla barra laterale sinistra.
</Tip>

Dopo aver eseguito **Developer: Reload Window** o riavviato VS Code, se una chat torna con la sua conversazione dipende da dove era aperta:

* **Scheda editor**: la conversazione torna con la sua scheda.
* **Barra laterale**: la conversazione torna se hai inviato un messaggio o Claude ha risposto in essa negli ultimi 10 minuti. Se non torna, riprendi la conversazione da [Cronologia sessioni](#resume-past-conversations).

Se il ricaricamento ha interrotto Claude a metà di un passaggio, Claude continua quel passaggio quando la conversazione torna, e un avviso nella chat contrassegna la continuazione. Richiede Claude Code v2.1.274 o successivo. Se il passaggio è stato interrotto più di un'ora fa o la sessione è aperta altrove, la conversazione torna inattiva.

Per disattivare la continuazione, apri l'[impostazione Continue After Reload](vscode://settings/claudeCode.continueAfterReload) e deselezionala.

<h3 id="run-multiple-conversations">
  Esegui più conversazioni
</h3>

Usa **Open in New Tab** o **Open in New Window** dalla Command Palette per avviare conversazioni aggiuntive. Ogni conversazione mantiene la propria cronologia e contesto, permettendoti di lavorare su diversi compiti in parallelo.

Quando usi le schede, un piccolo punto colorato sull'icona della scintilla indica lo stato: blu significa che una richiesta di autorizzazione è in sospeso, arancione significa che Claude ha terminato mentre la scheda era nascosta.

<h3 id="organize-sessions-into-groups">
  Organizza le sessioni in gruppi
</h3>

Nell'elenco delle sessioni nella Activity Bar, puoi raccogliere le sessioni correlate in gruppi denominati e comprimibili. Richiede Claude Code v2.1.229 o successivo.

* **Raggruppa o separa una sessione**: fai clic con il pulsante destro del mouse su una sessione per creare un gruppo da essa, spostarla in un gruppo esistente o rimuoverla dal suo gruppo. Ogni sessione appartiene a un gruppo alla volta, quindi spostarla in un altro gruppo la rimuove dal primo.
* **Sposta più sessioni contemporaneamente**: `Cmd`-click (Mac) / `Ctrl`-click (Windows/Linux) su ogni sessione, o `Shift`-click per selezionare un intervallo, quindi fai clic con il pulsante destro del mouse sulla selezione.
* **Raggruppa una sessione dalla sua scheda**: esegui **Claude Code: Add Session Tab to Group** dalla Command Palette, quindi scegli o crea un gruppo. Richiede Claude Code v2.1.257 o successivo.
* **Rinomina o elimina un gruppo**: fai clic con il pulsante destro del mouse su un'intestazione di gruppo. L'eliminazione di un gruppo rimuove solo il gruppo e le sue sessioni tornano all'elenco non raggruppato.

L'estensione salva i gruppi per cartella di workspace, quindi sopravvivono ai ricaricamenti della finestra e appaiono in ogni finestra in cui apri la stessa cartella. Quando cerchi nell'elenco, l'estensione mostra i risultati in un elenco piatto su tutti i gruppi.

<h3 id="switch-to-terminal-mode">
  Passa alla modalità terminale
</h3>

Per impostazione predefinita, l'estensione apre un pannello di chat grafico. Se preferisci l'interfaccia in stile CLI, apri l'[impostazione Use Terminal](vscode://settings/claudeCode.useTerminal) e seleziona la casella.

Puoi anche aprire le impostazioni di VS Code (`Cmd+,` su Mac o `Ctrl+,` su Windows/Linux), vai a Extensions → Claude Code e seleziona **Use Terminal**.

<h2 id="manage-plugins">
  Gestire i plugin
</h2>

L'estensione VS Code include un'interfaccia grafica per installare e gestire i [plugin](/docs/it/plugins/overview). Digitate `/plugins` nella casella del prompt per aprire l'interfaccia **Manage plugins**.

<h3 id="install-plugins">
  Installare i plugin
</h3>

La finestra di dialogo dei plugin mostra due schede: **Plugins** e **Marketplaces**.

Nella scheda Plugins:

* I **plugin installati** appaiono in alto con interruttori per abilitarli o disabilitarli
* I **plugin disponibili** dai vostri marketplace configurati appaiono di seguito
* Cercate per filtrare i plugin per nome o descrizione
* Fate clic su **Install** su qualsiasi plugin disponibile

Quando installate un plugin, scegliete l'ambito di installazione:

* **Install for you**: disponibile in tutti i vostri progetti (ambito utente)
* **Install for this project**: condiviso con i collaboratori del progetto (ambito progetto)
* **Install locally**: solo per voi, solo in questo repository (ambito locale)

<h3 id="share-a-plugin-install-link">
  Condividere un collegamento di installazione del plugin
</h3>

Per inviare a qualcuno un collegamento diretto all'installazione di un plugin specifico, fornitegli l'URL `install-plugin` dell'estensione. L'apertura di questo URL avvia o mette a fuoco VS Code, apre il pannello Claude Code e apre la finestra di dialogo **Manage plugins** sulla scelta dell'ambito di quel plugin. Nulla viene installato finché la persona non sceglie un ambito. Se il marketplace del plugin non è ancora configurato in Claude Code, la finestra di dialogo chiede prima di aggiungerlo.

```text theme={null}
vscode://anthropic.claude-code/install-plugin?plugin=code-review&marketplace=anthropics/claude-plugins-official
```

L'URL accetta due parametri di query:

| Parametro     | Descrizione                                                                                                                                                                                       |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugin`      | Il nome del plugin come lo elenca il suo marketplace. Obbligatorio.                                                                                                                               |
| `marketplace` | Da dove proviene il plugin: un `owner/repo` di GitHub, un URL `https://` o un URL SSH git come `git@github.com:owner/repo.git`. Predefinito a `anthropics/claude-plugins-official` quando omesso. |

Alcuni valori che la [scheda Marketplaces](#manage-marketplaces) accetta non funzionano in un collegamento, come un percorso locale o un indirizzo `http://`. Per questi, VS Code mostra un messaggio di errore e la finestra di dialogo non si apre.

Due casi terminano con un messaggio nella finestra di dialogo invece della scelta dell'ambito:

* **Il marketplace non elenca un plugin con quel nome**: la finestra di dialogo segnala che il plugin non è stato trovato. Controllate il valore `plugin` rispetto all'elenco del marketplace.
* **Il plugin è già installato**: la finestra di dialogo lo comunica e nulla cambia.

I README di GitHub, i problemi e alcuni altri host Markdown rimuovono i collegamenti il cui schema non è `http` o `https`, quindi un collegamento `vscode://` lì viene visualizzato come testo semplice. Inserite l'URL in un blocco di codice su questi host, come [Il collegamento viene visualizzato come testo semplice invece di essere cliccabile](/docs/it/deep-links#the-link-renders-as-plain-text-instead-of-being-clickable) descrive per i collegamenti `claude-cli://`.

<h3 id="manage-marketplaces">
  Gestire i marketplace
</h3>

Passate alla scheda **Marketplaces** per aggiungere o rimuovere fonti di plugin:

* Inserite un repository GitHub, un URL o un percorso locale per aggiungere un nuovo marketplace
* Fate clic sull'icona di aggiornamento per aggiornare l'elenco dei plugin di un marketplace
* Fate clic sull'icona del cestino per rimuovere un marketplace

Le modifiche ai plugin che apportate nella finestra di dialogo si applicano immediatamente alle sessioni Claude Code aperte in quella finestra VS Code. Se la sessione da cui avete aperto la finestra di dialogo non riesce a ricaricare i suoi plugin, la finestra di dialogo offre di riprovare o di riavviare Claude in quella sessione.

<Note>
  La gestione dei plugin in VS Code utilizza gli stessi comandi CLI dietro le quinte. I plugin e i marketplace che configurate nell'estensione sono disponibili anche nella CLI, e viceversa.
</Note>

Per ulteriori informazioni sul sistema dei plugin, consultate [Plugins](/docs/it/plugins/overview) e [Plugin marketplaces](/docs/it/plugins/overview).

<h2 id="automate-browser-tasks-with-chrome">
  Automatizzare attività del browser con Chrome
</h2>

Connetti Claude al tuo browser Chrome per testare app web, eseguire il debug con i log della console e automatizzare i flussi di lavoro del browser senza lasciare VS Code. Questo richiede l'[estensione Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versione 1.0.36 o superiore.

Digita `@browser` nella casella del prompt seguito da ciò che desideri che Claude faccia:

```text wrap theme={null}
@browser go to localhost:3000 and check the console for errors
```

Puoi anche aprire il menu degli allegati per selezionare strumenti specifici del browser come aprire una nuova scheda o leggere il contenuto della pagina.

Claude apre nuove schede per le attività del browser e condivide lo stato di accesso del tuo browser, quindi può accedere a qualsiasi sito a cui sei già connesso.

Per le istruzioni di configurazione, l'elenco completo delle funzionalità e la risoluzione dei problemi, consulta [Usa Claude Code con Chrome](/docs/it/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  Comandi e scorciatoie da tastiera di VS Code
</h2>

Apri il Riquadro comandi (`Cmd+Shift+P` su Mac o `Ctrl+Shift+P` su Windows/Linux) e digita "Claude Code" per visualizzare tutti i comandi VS Code disponibili per l'estensione Claude Code.

Alcuni scorciatoie dipendono da quale pannello è "attivo" (riceve input da tastiera). Quando il cursore è in un file di codice, l'editor è attivo. Quando il cursore è nella casella di prompt di Claude, Claude è attivo. Usa `Cmd+Esc` / `Ctrl+Esc` per alternare tra loro.

<Note>
  Questi sono comandi VS Code per controllare l'estensione. Non tutti i comandi Claude Code integrati sono disponibili nell'estensione. Vedi [Estensione VS Code vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) per i dettagli.
</Note>

| Comando                    | Scorciatoia                                              | Descrizione                                                                                                                                                                                                                                                                                                          |
| -------------------------- | -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Alterna lo stato attivo tra editor e Claude                                                                                                                                                                                                                                                                          |
| Focus last message         | -                                                        | Sposta lo stato attivo della tastiera al messaggio più recente nella conversazione, o a un prompt di autorizzazione in attesa, in modo da poter leggere da lì con la tastiera o un lettore di schermo. Non disponibile in [modalità terminale](#switch-to-terminal-mode). Richiede Claude Code v2.1.268 o successivo |
| Open in Side Bar           | -                                                        | Apri Claude nella barra laterale                                                                                                                                                                                                                                                                                     |
| Open in Terminal           | -                                                        | Apri Claude in modalità terminale                                                                                                                                                                                                                                                                                    |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Apri una nuova conversazione come scheda editor                                                                                                                                                                                                                                                                      |
| Open in New Window         | -                                                        | Apri una nuova conversazione in una finestra separata                                                                                                                                                                                                                                                                |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Avvia una nuova conversazione. Richiede che Claude sia attivo e `enableNewConversationShortcut` impostato su `true`                                                                                                                                                                                                  |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Riapri la scheda sessione Claude chiusa più di recente. Ricade al normale riapertura-editor-chiuso di VS Code quando l'ultima scheda chiusa non era una sessione Claude. Disabilita con `enableReopenClosedSessionShortcut`                                                                                          |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Inserisci un riferimento al file corrente e alla selezione (richiede che l'editor sia attivo)                                                                                                                                                                                                                        |
| Accept Change at Cursor    | -                                                        | Accetta la modifica al cursore mentre [esamini una modifica proposta](#get-started) una modifica alla volta. Richiede Claude Code v2.1.275 o successivo                                                                                                                                                              |
| Reject Change at Cursor    | -                                                        | Ripristina la modifica al cursore mentre esamini una modifica proposta una modifica alla volta. Richiede Claude Code v2.1.275 o successivo                                                                                                                                                                           |
| Toggle Focus view          | `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux)     | Nascondi o mostra l'attività dello strumento nella conversazione. Funziona mentre un pannello Claude o la barra laterale è visibile. Richiede Claude Code v2.1.221 o successivo                                                                                                                                      |
| Rename Session Tab         | -                                                        | Rinomina la sessione nella scheda Claude attiva. Richiede Claude Code v2.1.257 o successivo                                                                                                                                                                                                                          |
| Add Session Tab to Group   | -                                                        | Aggiungi la sessione nella scheda Claude attiva a un [gruppo di sessioni](#organize-sessions-into-groups) che scegli o crei. Richiede Claude Code v2.1.257 o successivo                                                                                                                                              |
| Mark Session as Unread     | -                                                        | Contrassegna la sessione nella scheda Claude attiva come non letta nell'elenco delle sessioni. Richiede Claude Code v2.1.257 o successivo                                                                                                                                                                            |
| Show Logs                  | -                                                        | Visualizza i log di debug dell'estensione                                                                                                                                                                                                                                                                            |
| Logout                     | -                                                        | Esci dal tuo account Anthropic                                                                                                                                                                                                                                                                                       |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Avvia una scheda VS Code da altri strumenti
</h3>

L'estensione registra un gestore URI in `vscode://anthropic.claude-code/open`. Usalo per aprire una nuova scheda Claude Code dal tuo strumento: un alias shell, un segnalibro del browser o qualsiasi script che possa aprire un URL. Se VS Code non è già in esecuzione, l'apertura dell'URL lo avvia prima. Se VS Code è già in esecuzione, l'URL si apre nella finestra attualmente attiva.

Richiama il gestore con l'opener URL del tuo sistema operativo.

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```

    Il comando `xdg-open` proviene dal pacchetto `xdg-utils`. Se la shell segnala che non è trovato, vedi [xdg-open is not found on Linux](/docs/it/deep-links#xdg-open-is-not-found-on-linux).
  </Tab>

  <Tab title="Windows">
    In PowerShell:

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    In `cmd.exe`, `start` tratta il suo primo argomento tra virgolette come titolo della finestra, quindi passa un titolo vuoto prima dell'URL:

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

Il gestore accetta due parametri di query facoltativi:

| Parametro | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                            |
| --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Testo da pre-compilare nella casella di prompt. Deve essere codificato in URL. Il prompt è pre-compilato ma non inviato automaticamente.                                                                                                                                                                                                                                                                                               |
| `session` | Un ID di sessione da riprendere invece di avviare una nuova conversazione. La sessione deve appartenere all'area di lavoro attualmente aperta in VS Code. Se la sessione non viene trovata, viene avviata una conversazione nuova. Se la sessione è già aperta in una scheda, quella scheda è attiva. Per acquisire un ID di sessione a livello di programmazione, vedi [Continue conversations](/docs/it/headless#continue-conversations). |

Ad esempio, per aprire una scheda pre-compilata con "review my changes":

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

L'estensione gestisce anche `vscode://anthropic.claude-code/install-plugin`, che [apre la finestra di dialogo del plugin su un plugin](#share-a-plugin-install-link). Per avviare una sessione terminale invece di una scheda VS Code, usa il gestore `claude-cli://` della CLI. Vedi [Launch sessions from links](/docs/it/deep-links).

<h2 id="configure-settings">
  Configurare le impostazioni
</h2>

L'estensione ha due tipi di impostazioni:

* **Impostazioni dell'estensione** in VS Code: controllano il comportamento dell'estensione all'interno di VS Code. Aprire con `Cmd+,` (Mac) o `Ctrl+,` (Windows/Linux), quindi andare a Estensioni → Claude Code. È anche possibile digitare `/` e selezionare **General config…** per aprire le impostazioni.
* **Impostazioni di Claude Code** in `~/.claude/settings.json`: condivise tra l'estensione e CLI. Utilizzarle per i comandi consentiti, le variabili di ambiente, gli hooks e i server MCP. Nei piani Pro, Max e Team, è anche uno degli input della modalità di autorizzazione con cui iniziano le conversazioni. [Switch permission modes](/docs/it/permission-modes#switch-permission-modes) elenca l'ordine. Vedere [Settings](/docs/it/settings) per i dettagli.

<Tip>
  Aggiungere `"$schema": "https://json.schemastore.org/claude-code-settings.json"` al vostro `settings.json` per ottenere il completamento automatico e la convalida inline per tutte le impostazioni disponibili direttamente in VS Code.
</Tip>

<h3 id="extension-settings">
  Impostazioni dell'estensione
</h3>

VS Code legge `initialPermissionMode` dalle impostazioni utente e ignora i valori dell'area di lavoro. Prima della v2.1.225, VS Code impostava per impostazione predefinita l'impostazione su `default` e applicava i valori dell'area di lavoro.

| Impostazione                        | Predefinito | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ----------------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false`     | Avvia Claude in modalità terminale invece di pannello grafico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `initialPermissionMode`             | -           | Controlla i prompt di approvazione per le nuove conversazioni: `default`, `plan`, `acceptEdits` o `bypassPermissions`. `manual` è un alias per `default` e seleziona la modalità etichettata **Manual** nell'indicatore di modalità. Quando lo lasciate non impostato, l'estensione sceglie la modalità di autorizzazione iniziale come descritto in [Switch permission modes](/docs/it/permission-modes#switch-permission-modes).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `preferredLocation`                 | `panel`     | Dove Claude si apre: `sidebar` (destra) o `panel` (nuova scheda)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `lockEditorGroups`                  | `true`      | [Blocca i gruppi di editor che Claude avvia per le sue schede](#choose-where-claude-lives), in modo che i file aperti mentre una scheda Claude è attiva vadano a un altro gruppo. Quando disattivato, l'estensione non blocca mai un gruppo di editor. Richiede Claude Code v2.1.274 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `autosave`                          | `true`      | Salva automaticamente i file prima che Claude li legga o scriva                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `attachOpenFile`                    | `true`      | Aggiunge il file aperto nell'editor ai vostri messaggi e lo mostra nella casella dei prompt. Quando disattivato, viene aggiunto solo il testo selezionato. Richiede Claude Code v2.1.271 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `useCtrlEnterToSend`                | `false`     | Usa Ctrl/Cmd+Invio invece di Invio per inviare i prompt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `scrollToBottomOnSend`              | `true`      | Scorri la conversazione verso il basso quando inviate un messaggio. Quando disattivato, la conversazione rimane dove l'avete lasciata. Richiede Claude Code v2.1.275 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `enableNewConversationShortcut`     | `false`     | Abilita Cmd/Ctrl+N per avviare una nuova conversazione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `enableReopenClosedSessionShortcut` | `true`      | Usa Cmd/Ctrl+Maiusc+T per riaprire la scheda della sessione Claude chiusa più di recente. Quando l'ultima scheda chiusa non era una sessione Claude, la scorciatoia esegue il comando di riapertura dell'editor chiuso normale di VS Code.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `archiveInactiveSessions`           | `14`        | [Archivia una sessione automaticamente](#resume-past-conversations) dopo questo numero di giorni senza attività: `1`, `2`, `7` o `14`. Impostare `0` per disattivarlo. Richiede Claude Code v2.1.265 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `continueAfterReload`               | `true`      | Dopo un ricaricamento della finestra, Claude [continua il passaggio che è stato interrotto](#choose-where-claude-lives) nella sessione ripristinata. Richiede Claude Code v2.1.274 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `hideOnboarding`                    | `false`     | Nascondi la checklist di onboarding (icona del berretto di laurea)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `focusView`                         | `false`     | Nascondi le chiamate di strumenti, i risultati degli strumenti e il pensiero dietro righe espandibili, lasciando i vostri prompt e le risposte di Claude. L'elenco di cose da fare più recente di Claude rimane visibile; questo richiede Claude Code v2.1.225 o successivo. È anche possibile attivare/disattivare Focus view dal menu dei comandi. Richiede Claude Code v2.1.221 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `respectGitIgnore`                  | `true`      | Escludere i modelli .gitignore dalle ricerche di file e da [selection context](#reference-files-and-folders)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `usePythonEnvironment`              | `true`      | Attiva l'ambiente Python dell'area di lavoro quando si esegue Claude. Richiede l'estensione Python.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `environmentVariables`              | `[]`        | Imposta le variabili di ambiente per il processo Claude. Utilizzare le impostazioni di Claude Code per la configurazione condivisa.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `disableLoginPrompt`                | `false`     | Salta i prompt di autenticazione (per configurazioni di provider di terze parti)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `allowDangerouslySkipPermissions`   | `false`     | Aggiunge Bypass permissions al selettore di modalità. Utilizzarlo solo in sandbox senza accesso a Internet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `claudeProcessWrapper`              | -           | Eseguibile utilizzato per avviare il processo Claude. Il percorso binario in bundle viene passato come argomento quando presente. Impostarlo su un binario `claude` installato separatamente se la build dell'estensione non ne include uno per la vostra piattaforma. In una configurazione con wrapper, le conversazioni iniziano in modalità Manual a meno che non impostiate `initialPermissionMode` o non abbiate scelto Manual, Edit automatically o Auto in una conversazione precedente, perché l'estensione salta i passaggi delle impostazioni e del valore predefinito incorporato lì; vedere [Switch permission modes](/docs/it/permission-modes#switch-permission-modes). Un errore "Unsupported platform" all'attivazione significa che nessun binario è in bundle per la vostra piattaforma; vedere [which platforms have prebuilt binaries](/docs/it/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="use-a-screen-reader">
  Utilizzare un lettore di schermo
</h2>

Il pannello di chat dell'estensione funziona con i lettori di schermo. Non è necessario attivare nulla: l'estensione annuncia l'attività della conversazione per ogni utente, senza alcun cambiamento visivo. Questo è separato dalla [modalità lettore di schermo](/docs/it/accessibility) opzionale della CLI, che adatta l'interfaccia del terminale.

Il supporto del lettore di schermo nel pannello di chat richiede Claude Code v2.1.236 o versione successiva.

Durante una conversazione, l'estensione annuncia:

* **Risposte di Claude**: l'estensione annuncia ogni risposta una sola volta, quando è completa, e rimane silenziosa mentre il testo viene trasmesso. Il lettore di schermo legge i blocchi di codice come un riepilogo del conteggio delle righe, legge i link in base alla loro etichetta e legge le tabelle cella per cella; la risposta completa rimane leggibile nella trascrizione.
* **Richieste di autorizzazione e domande**: l'estensione annuncia una richiesta quando appare il suo prompt di autorizzazione, indicando lo strumento che Claude vuole utilizzare. Annuncia allo stesso modo quando Claude ti pone una domanda e quando Claude termina un piano e attende la tua revisione.
* **Cambiamenti di stato**: l'estensione annuncia quando Claude inizia a lavorare, quando Claude è pronto per il tuo input e quando Claude Code inizia a compattare la conversazione.
* **Errori e prompt del modello**: l'estensione annuncia gli errori nella conversazione e annuncia quando appare il [prompt di consenso per i crediti di utilizzo](/docs/it/model-config#fable-and-usage-credits) o il [prompt di richiesta contrassegnata](/docs/it/model-config#ask-before-switching).

Mentre Claude lavora, il lettore di schermo legge un'etichetta di testo al posto dell'animazione dello spinner di avanzamento.

Quando riapri una sessione o passi a un'altra, l'estensione non annuncia nulla: la cronologia ripristinata, i prompt di autorizzazione in sospeso e lo stato in corso rimangono silenziosi fino a quando non accade qualcosa di nuovo.

<h3 id="use-the-chat-panel-from-the-keyboard">
  Utilizzare il pannello di chat dalla tastiera
</h3>

Ogni turno nella trascrizione inizia con un'intestazione visivamente nascosta etichettata con il prompt che ha avviato il turno, in modo da poter saltare tra i turni utilizzando la navigazione per intestazioni del lettore di schermo.

All'interno di un turno, il lettore di schermo annuncia di chi è il messaggio mentre ti muovi attraverso di esso:

* **I tuoi messaggi**: "Tu"
* **Messaggi di Claude**: "Claude"
* **Passaggi dello strumento**: "Claude" più il nome dello strumento, ad esempio "Claude, Bash"
* **Blocchi di pensiero**: "Claude, thinking"

Poiché l'estensione espone la trascrizione come una regione etichettata, puoi anche spostare lo stato attivo sulla trascrizione stessa con `Tab` e leggerla al tuo ritmo. Per spostare lo stato attivo al messaggio più recente o a un prompt di autorizzazione in attesa, esegui **Claude Code: Focus last message** dalla [Command Palette](#vs-code-commands-and-shortcuts).

Quando un'opzione su un prompt di autorizzazione salva una regola di autorizzazione o l'accesso a una directory, la sua etichetta termina indicando dove viene salvata l'approvazione, ad esempio "tutti i progetti" o "questa sessione". Con l'opzione attiva, premi il tasto freccia `Sinistra` o `Destra` per cambiare la destinazione, e l'estensione annuncia ogni destinazione mentre vi arrivi. Puoi anche fare clic sulla destinazione nell'etichetta. I tasti freccia richiedono Claude Code v2.1.268 o versione successiva.

<h2 id="vs-code-extension-vs-claude-code-cli">
  Estensione VS Code vs. Claude Code CLI
</h2>

Claude Code è disponibile sia come estensione VS Code (pannello grafico) che come CLI (interfaccia da riga di comando nel terminale). Alcune funzionalità sono disponibili solo nella CLI. Se hai bisogno di una funzionalità disponibile solo nella CLI, esegui `claude` nel terminale integrato di VS Code. Ciò richiede l'[installazione CLI standalone](/docs/it/setup): l'estensione non aggiunge `claude` al vostro PATH. Vedi [Eseguire CLI in VS Code](#run-cli-in-vs-code).

| Funzionalità              | CLI                   | Estensione VS Code                                                                                  |
| ------------------------- | --------------------- | --------------------------------------------------------------------------------------------------- |
| Comandi e skills          | [Tutti](/docs/it/commands) | Sottoinsieme (digita `/` per vedere quelli disponibili)                                             |
| Configurazione server MCP | Sì                    | Sì ([aggiungi e gestisci server](#connect-to-external-tools-with-mcp) con `/mcp` nel pannello chat) |
| Checkpoints               | Sì                    | Sì                                                                                                  |
| Scorciatoia bash `!`      | Sì                    | No                                                                                                  |
| Completamento scheda      | Sì                    | No                                                                                                  |

<h3 id="rewind-with-checkpoints">
  Rewind con checkpoints
</h3>

L'estensione VS Code supporta i checkpoints, che tracciano le modifiche ai file di Claude e consentono di tornare a uno stato precedente. Passa il mouse su qualsiasi messaggio per rivelare il pulsante di rewind, quindi scegli tra tre opzioni:

* **Fork conversation from here**: avvia un nuovo ramo di conversazione da questo messaggio mantenendo intatte tutte le modifiche al codice
* **Rewind code to here**: ripristina le modifiche ai file fino a questo punto nella conversazione mantenendo la cronologia completa della conversazione
* **Fork conversation and rewind code**: avvia un nuovo ramo di conversazione e ripristina le modifiche ai file fino a questo punto

Per i dettagli completi su come funzionano i checkpoints e le loro limitazioni, vedi [Checkpointing](/docs/it/checkpointing).

<h3 id="run-cli-in-vs-code">
  Eseguire CLI in VS Code
</h3>

Per utilizzare la CLI rimanendo in VS Code, apri il terminale integrato (`` Ctrl+` `` su Windows/Linux o `` Cmd+` `` su Mac) ed esegui `claude`. La CLI si integra automaticamente con il vostro IDE per funzionalità come la visualizzazione dei diff e la condivisione dei diagnostici.

L'installazione dell'estensione non aggiunge `claude` al vostro PATH della shell. L'estensione raggruppa una copia privata della CLI per il suo pannello chat, ma digitare `claude` in un terminale richiede l'[installazione CLI standalone](/docs/it/setup). Esegui l'installazione una volta e i comandi su questa pagina, inclusi `claude mcp add` e `claude --resume`, funzionano in qualsiasi terminale. Se `claude` non viene ancora trovato dopo l'installazione, [verifica il vostro PATH](/docs/it/troubleshoot-install#verify-your-path).

Se utilizzi un terminale esterno, esegui `/ide` all'interno di Claude Code per collegarlo a VS Code.

<h3 id="switch-between-extension-and-cli">
  Passare tra estensione e CLI
</h3>

L'estensione e la CLI condividono la stessa cronologia di conversazione. Per continuare una conversazione dell'estensione nella CLI, esegui `claude --resume` nel terminale. Questo apre un selettore interattivo dove puoi cercare e selezionare la tua conversazione.

<h3 id="include-terminal-output-in-prompts">
  Includere l'output del terminale nei prompt
</h3>

Fai riferimento all'output del terminale nei tuoi prompt utilizzando `@terminal:name` dove `name` è il titolo del terminale. Questo consente a Claude di vedere l'output dei comandi, i messaggi di errore o i log senza copia-incolla.

<h3 id="monitor-background-processes">
  Monitorare i processi in background
</h3>

Digita `/tasks` nella casella del prompt per aprire la [mappa dell'agente](#use-the-prompt-box), che elenca i task in background della sessione, come un server di sviluppo che Claude ha lasciato in esecuzione come comando shell in background. Fai clic su un task per aprire la sua scheda e fermarlo lì. Richiede Claude Code v2.1.277 o versione successiva.

<h3 id="connect-to-external-tools-with-mcp">
  Connettere strumenti esterni con MCP
</h3>

I server MCP (Model Context Protocol) danno a Claude accesso a strumenti esterni, database e API.

Per gestire i server MCP senza lasciare VS Code, digita `/mcp` nel pannello chat. Dalla finestra di dialogo che si apre, puoi aggiungere server, rimuovere server salvati a livello locale, utente o progetto [scope](/docs/it/mcp#mcp-installation-scopes), abilitare o disabilitare server, riconnettersi a un server e gestire l'autenticazione OAuth. L'aggiunta e la rimozione di server nella finestra di dialogo richiede Claude Code v2.1.261 o versione successiva.

Puoi anche eseguire `claude mcp add` nel terminale integrato di VS Code (`` Ctrl+` `` o `` Cmd+` ``). La finestra di dialogo e il comando del terminale salvano nella stessa configurazione MCP, e le modifiche da entrambi hanno effetto nelle conversazioni che avvii successivamente. L'esempio seguente aggiunge il server MCP remoto di GitHub, che si autentica con un [personal access token](https://github.com/settings/personal-access-tokens) passato come intestazione:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Sostituisci `YOUR_GITHUB_PAT` con il tuo personal access token. Il comando `claude mcp add` salva la configurazione senza convalidare le credenziali, quindi un valore segnaposto è accettato qui ma il server non riesce a connettersi in seguito. Per verificare la connessione, avvia una nuova conversazione, digita `/mcp` e controlla che il server mostri **Connected**. Un server con credenziali errate mostra **Failed**.

Una volta configurato, chiedi a Claude di utilizzare gli strumenti (ad esempio, "Review PR #456").

Per trovare server a cui connettersi, vedi [Find and build MCP servers](/docs/it/mcp#find-and-build-mcp-servers).

<h2 id="work-with-git">
  Lavorare con git
</h2>

Claude Code si integra con git per aiutare con i flussi di lavoro di controllo versione direttamente in VS Code. Chiedete a Claude di eseguire il commit delle modifiche, creare pull request o lavorare tra i rami. Per avviare Claude in un worktree isolato con i propri file e ramo, consultate [Eseguire sessioni parallele con worktrees](/docs/it/worktrees).

<h3 id="create-commits-and-pull-requests">
  Creare commit e pull request
</h3>

Claude può mettere in stage le modifiche, scrivere messaggi di commit e creare pull request in base al vostro lavoro:

```text wrap theme={null}
commit my changes with a descriptive message
create a pr for this feature
summarize the changes I've made to the auth module
```

Quando create pull request, Claude genera descrizioni basate sulle modifiche effettive del codice e può aggiungere contesto riguardante le decisioni di test o implementazione.

<h2 id="use-third-party-providers">
  Utilizzare provider di terze parti
</h2>

Per impostazione predefinita, Claude Code si connette direttamente all'API di Anthropic. Se la vostra organizzazione utilizza Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry per accedere a Claude, configurate l'estensione per utilizzare il vostro provider:

<Steps>
  <Step title="Disabilitare il prompt di accesso">
    Aprite l'[impostazione Disable Login Prompt](vscode://settings/claudeCode.disableLoginPrompt) e spuntate la casella.

    Potete anche aprire le impostazioni di VS Code (`Cmd+,` su Mac o `Ctrl+,` su Windows/Linux), cercare "Claude Code login" e spuntare **Disable Login Prompt**.
  </Step>

  <Step title="Configurare il vostro provider">
    Seguite la guida di configurazione per il vostro provider:

    * [Claude Code on Amazon Bedrock](/docs/it/amazon-bedrock)
    * [Claude Code on Google Cloud's Agent Platform](/docs/it/google-vertex-ai)
    * [Claude Code on Microsoft Foundry](/docs/it/microsoft-foundry)

    Queste guide coprono la configurazione del vostro provider in `~/.claude/settings.json`, che garantisce che le vostre impostazioni siano condivise tra l'estensione VS Code e la CLI.
  </Step>
</Steps>

Su un provider di terze parti, l'estensione non offre funzionalità che richiedono un account claude.ai, come le barre di tracciamento dell'utilizzo, la [dettatura vocale](/docs/it/voice-dictation) e la scheda Web per le [sessioni cloud](#resume-cloud-sessions-from-claude-ai). Per informazioni su ciò che la finestra di dialogo Account e utilizzo mostra su questi accessi, consultate [Controllare account e utilizzo](#check-account-and-usage).

Un accesso claude.ai rimasto da un precedente `/login` rimane inutilizzato: l'estensione non lo invia con alcuna richiesta.

<h2 id="security-and-privacy">
  Sicurezza e privacy
</h2>

Il vostro codice rimane privato. Claude Code elabora il vostro codice per fornire assistenza ma non lo utilizza per addestrare i modelli. Per dettagli sulla gestione dei dati e su come disattivare la registrazione, consultate [Data and privacy](/docs/it/data-usage).

Con le autorizzazioni di auto-edit abilitate, Claude Code può modificare i file di configurazione di VS Code (come `settings.json` o `tasks.json`) che VS Code potrebbe eseguire automaticamente. Per ridurre il rischio quando si lavora con codice non attendibile:

* Abilitate [VS Code Restricted Mode](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) per gli spazi di lavoro non attendibili
* Utilizzate la modalità Manual invece di Edit automatically o Auto per le modifiche
* Esaminate attentamente le modifiche prima di accettarle

<h3 id="the-built-in-ide-mcp-server">
  The built-in IDE MCP server
</h3>

Quando l'estensione è attiva, esegue un server MCP locale a cui il CLI si connette automaticamente. È così che il CLI apre i diff nel visualizzatore diff nativo di VS Code, legge la selezione corrente per le menzioni `@` e — quando state lavorando in un notebook Jupyter — chiede a VS Code di eseguire le celle.

Il server è denominato `ide` ed è nascosto da `/mcp` perché non c'è nulla da configurare. Se la vostra organizzazione utilizza un hook `PreToolUse` per creare un elenco di strumenti MCP consentiti, tuttavia, dovrete sapere che esiste.

**Selection and open-file context.** Mentre è connesso, il CLI include la selezione dell'editor corrente e il percorso del file attivo come contesto su ogni prompt che inviate. La trascrizione mostra una riga `⧉ Selected N lines from <file>` quando ciò accade.

Per escludere un file sensibile come `.env`, aggiungete una [regola di negazione `Read`](/docs/it/permissions#read-and-edit) per il suo percorso. Una regola di negazione corrispondente impedisce sia il testo selezionato che l'avviso del file aperto per quel file di raggiungere Claude.

Se disattivate l'impostazione [Attach Open File](#extension-settings), il CLI riceve il percorso del file attivo solo mentre avete testo selezionato in esso.

**Transport and authentication.** Il server si associa a `127.0.0.1` su una porta casuale nell'intervallo 10000–65535, e la porta non è configurabile. Il trasporto è `ws://` non crittografato; poiché il socket è solo loopback, qualsiasi processo che potrebbe catturare il traffico può anche leggere il token dal file di blocco, quindi TLS non aggiungerebbe protezione. Ogni attivazione dell'estensione genera un token di autenticazione casuale fresco, lo scrive in un file di blocco in `~/.claude/ide/<port>.lock`, e il CLI deve presentarlo come intestazione `X-Claude-Code-Ide-Authorization` per connettersi. Il file di blocco ha autorizzazioni `0600` in una directory `0700`, quindi solo l'utente che esegue VS Code può leggerlo. Se `CLAUDE_CONFIG_DIR` è impostato, il file di blocco viene scritto in `$CLAUDE_CONFIG_DIR/ide/` invece.

**Tools exposed to the model.** Il server ospita una dozzina di strumenti, ma solo due sono visibili al modello. Il resto è RPC interno che il CLI utilizza per la propria interfaccia utente, come l'apertura di diff, la lettura di selezioni e il salvataggio di file. Vengono filtrati prima che l'elenco degli strumenti raggiunga Claude.

| Tool name (as seen by hooks) | What it does                                                                                                                                  | Read-only |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| `mcp__ide__getDiagnostics`   | Restituisce i diagnostici del language server: gli errori e gli avvisi nel pannello Problemi di VS Code. Facoltativamente limitato a un file. | Yes       |
| `mcp__ide__executeCode`      | Esegue il codice Python nel kernel del notebook Jupyter attivo. Consultate il flusso di conferma di seguito.                                  | No        |

**Jupyter execution always asks first.** `mcp__ide__executeCode` non può eseguire nulla silenziosamente. Ad ogni chiamata, il codice viene inserito come una nuova cella alla fine del notebook attivo, VS Code lo scorre in vista, e un Quick Pick nativo vi chiede di **Execute** o **Cancel**. Annullare, o chiudere il picker con `Esc`, restituisce un errore a Claude e nulla viene eseguito. Lo strumento rifiuta anche completamente quando non c'è un notebook attivo, quando l'estensione Jupyter (`ms-toolsai.jupyter`) non è installata, o quando il kernel non è Python.

<Note>
  La conferma Quick Pick è separata dagli hook `PreToolUse`. Una voce di elenco consentito per `mcp__ide__executeCode` consente a Claude di *proporre* l'esecuzione di una cella; il Quick Pick all'interno di VS Code è ciò che le consente di *effettivamente* eseguirla.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Risolvere i problemi comuni
</h2>

<h3 id="extension-won’t-install">
  L'estensione non si installa
</h3>

* Assicurati di avere una versione compatibile di VS Code (1.94.0 o successiva)
* Verifica che VS Code abbia il permesso di installare estensioni
* Prova a installare direttamente dal [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)

<h3 id="spark-icon-not-visible">
  L'icona Spark non è visibile
</h3>

L'icona Spark appare nella **Editor Toolbar** (in alto a destra dell'editor) quando hai un file aperto. Se non la vedi:

1. **Apri un file**: L'icona richiede che un file sia aperto. Avere solo una cartella aperta non è sufficiente.
2. **Controlla la versione di VS Code**: Richiede 1.94.0 o superiore (Help → About)
3. **Riavvia VS Code**: Esegui "Developer: Reload Window" dalla Command Palette
4. **Disabilita le estensioni in conflitto**: Disabilita temporaneamente altre estensioni AI (Cline, Continue, ecc.)
5. **Controlla la fiducia dell'area di lavoro**: L'estensione non funziona in Restricted Mode

In alternativa, se hai impostato [`preferredLocation`](#extension-settings) su `sidebar`, o hai aperto Claude con **Claude Code: Open in Side Bar**, fai clic su "✻ Claude Code" nella **Status Bar** (angolo in basso a destra). Questo funziona anche senza un file aperto. Puoi anche usare la **Command Palette** (`Cmd+Shift+P` / `Ctrl+Shift+P`) e digitare "Claude Code".

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc non fa nulla su macOS
</h3>

Su macOS Tahoe e versioni successive, la scorciatoia di sistema Game Overlay è associata a `Cmd+Esc` per impostazione predefinita e intercetta la pressione del tasto prima che raggiunga VS Code. Per liberare la scorciatoia:

1. Apri System Settings
2. Vai a Keyboard, quindi Keyboard Shortcuts, quindi Game Controllers
3. Deseleziona la casella di controllo Game Overlay

In alternativa, riassegna l'estensione a un tasto diverso: apri l'editor [Keyboard Shortcuts](https://code.visualstudio.com/docs/configure/keybindings) di VS Code (`Cmd+K Cmd+S`), cerca `Claude Code: Focus input` e assegna una nuova associazione.

<h3 id="claude-code-never-responds">
  Claude Code non risponde mai
</h3>

Se Claude Code non risponde ai tuoi prompt:

1. **Controlla la tua connessione Internet**: Assicurati di avere una connessione Internet stabile
2. **Avvia una nuova conversazione**: Prova ad avviare una conversazione nuova per vedere se il problema persiste
3. **Prova la CLI**: Esegui `claude` dal terminale per vedere se ricevi messaggi di errore più dettagliati

Se i problemi persistono, [segnala un problema su GitHub](https://github.com/anthropics/claude-code/issues) con i dettagli dell'errore.

<h2 id="uninstall-the-extension">
  Disinstallare l'estensione
</h2>

Per disinstallare l'estensione Claude Code:

1. Aprire la visualizzazione Estensioni (`Cmd+Shift+X` su Mac o `Ctrl+Shift+X` su Windows/Linux)
2. Cercare "Claude Code"
3. Fare clic su **Disinstalla**

Se eseguite `claude` in un terminale integrato di VS Code, Claude Code reinstalla automaticamente l'estensione. Per mantenerla disinstallata, disattivate **Auto-install IDE extension** in `/config`, oppure impostate [`autoInstallIdeExtension`](/docs/it/settings-reference#autoinstallideextension) su `false`. Potete anche impostare la variabile di ambiente [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/it/env-vars) su `1`.

Per rimuovere anche i dati dell'estensione e ripristinare tutte le impostazioni, eliminate la directory di archiviazione dell'estensione per la vostra piattaforma.

Su macOS:

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

Su Linux:

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

Su Windows, in PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

Per ulteriore aiuto, consultate la [guida alla risoluzione dei problemi](/docs/it/troubleshooting).

<h2 id="next-steps">
  Passaggi successivi
</h2>

Ora che hai Claude Code configurato in VS Code:

* [Esplora i flussi di lavoro comuni](/docs/it/common-workflows) per ottenere il massimo da Claude Code
* [Configura i server MCP](/docs/it/mcp) per estendere le capacità di Claude con strumenti esterni. Aggiungili e gestiscili con `/mcp` nel pannello della chat.
* [Configura le impostazioni di Claude Code](/docs/it/settings) per personalizzare i comandi consentiti, gli hook e altro ancora. Queste impostazioni sono condivise tra l'estensione e la CLI.
