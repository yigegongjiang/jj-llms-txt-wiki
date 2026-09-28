> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Continua le sessioni locali da qualsiasi dispositivo con Remote Control

> Continua una sessione locale di Claude Code dal tuo telefono, tablet o da qualsiasi browser utilizzando Remote Control. Funziona con claude.ai/code e l'app Claude per dispositivi mobili.

<Note>
  Remote Control è disponibile su tutti i piani. Su Team e Enterprise, è disabilitato per impostazione predefinita fino a quando un proprietario non abilita l'interruttore Remote Control nelle [impostazioni di amministrazione di Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control connette [claude.ai/code](https://claude.ai/code) o l'app Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) a una sessione di Claude Code in esecuzione sulla tua macchina. Avvia un'attività alla tua scrivania, quindi riprendi dal tuo telefono sul divano o da un browser su un altro computer.

Quando avvii una sessione Remote Control sulla tua macchina, Claude continua a funzionare localmente per tutto il tempo, quindi l'esecuzione del codice e l'accesso al filesystem rimangono sulla tua macchina. Con Remote Control puoi:

* **Utilizzare il tuo ambiente locale completo da remoto**: il tuo filesystem, i [server MCP](/docs/it/mcp), gli strumenti e la configurazione del progetto rimangono disponibili, e digitando `@` l'autocompletamento completa i percorsi dei file dal tuo progetto locale.
* **Lavorare da entrambe le superfici contemporaneamente**: la conversazione e il progresso dei [subagent](/docs/it/sub-agents) e dei [flussi di lavoro dinamici](/docs/it/workflows) rimangono sincronizzati su tutti i dispositivi connessi, quindi puoi inviare messaggi dal tuo terminale, browser e telefono in modo intercambiabile.
* **Inviare immagini e file dal tuo telefono o browser**: allega una foto o un file nell'app Claude o su claude.ai/code, con o senza didascalia. Claude vede le foto allegate direttamente come parte del tuo messaggio. Claude Code scarica altri file sulla tua macchina e li passa a Claude come riferimenti a file `@`.
* **Sopravvivere alle interruzioni**: se il tuo laptop va in sospensione o la tua rete si interrompe, Claude Code si riconnette automaticamente quando la tua macchina torna online. Mentre la connessione si sta ricostruendo, Claude Code mette in coda i messaggi, i prompt di autorizzazione e gli aggiornamenti di stato dai subagent e dai flussi di lavoro, e li consegna una volta che la connessione si recupera.

A differenza di [Claude Code sul web](/docs/it/claude-code-on-the-web), che funziona su infrastrutture cloud, le sessioni Remote Control vengono eseguite direttamente sulla tua macchina e interagiscono con il tuo filesystem locale. Le interfacce web e mobile sono una finestra in quella sessione locale.

Questa pagina copre la configurazione, come avviare e connettersi alle sessioni, e come Remote Control si confronta con Claude Code sul web.

<h2 id="requirements">
  Requisiti
</h2>

Prima di utilizzare Remote Control, conferma che il tuo ambiente soddisfi queste condizioni:

* **Abbonamento**: disponibile su piani Pro, Max, Team e Enterprise. Le chiavi API non sono supportate. Su Team e Enterprise, un proprietario deve prima abilitare l'interruttore Remote Control nelle [impostazioni di amministrazione di Claude Code](https://claude.ai/admin-settings/claude-code).
* **Autenticazione**: esegui `claude` e utilizza `/login` per accedere tramite claude.ai se non l'hai già fatto. Senza un accesso idoneo, `claude remote-control` esce con un errore, mentre `claude --remote-control` avvia comunque una sessione interattiva e mostra una notifica di errore di Remote Control poco dopo l'avvio.
* **Endpoint API**: non disponibile in nessuna di queste configurazioni:
  * Utilizzi Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry.
  * Punti [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) a un host diverso da `api.anthropic.com`, come un [gateway LLM](/docs/it/llm-gateway) o proxy. Annulla l'impostazione della variabile per utilizzare Remote Control. Prima della v2.1.196, Claude Code consentiva Remote Control con un `ANTHROPIC_BASE_URL` personalizzato.
  * Accedi tramite un [gateway di app Claude](/docs/it/claude-apps-gateway) aziendale.
* **Valutazione dei flag di funzionalità**: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` e `DISABLE_GROWTHBOOK`](/docs/it/env-vars) disabilitano ciascuno la valutazione dei flag di funzionalità da cui dipende la disponibilità di Remote Control. Annulla l'impostazione della variabile ovunque sia impostata, nel tuo ambiente shell o nel blocco `env` di un [file `settings.json`](/docs/it/settings-reference#all-settings), per utilizzare Remote Control.
* **Fiducia dell'area di lavoro**: esegui `claude` nella directory del tuo progetto almeno una volta per accettare la finestra di dialogo di fiducia dell'area di lavoro. La finestra di dialogo di fiducia all'avvio non salva mai la fiducia per la directory home, quindi avvia Remote Control da una directory di progetto.

<h2 id="start-a-remote-control-session">
  Avvia una sessione Remote Control
</h2>

Puoi avviare una sessione Remote Control dalla CLI o dall'estensione VS Code. La CLI offre tre modalità di invocazione; VS Code utilizza il comando `/remote-control`.

<Tabs>
  <Tab title="Modalità server">
    Accedi alla directory del tuo progetto ed esegui:

    ```bash theme={null}
    claude remote-control
    ```

    Finché non accetti la conferma una tantum di Remote Control, `claude remote-control` spiega cosa fa e chiede `Enable Remote Control? (y/n)` prima di avviare il server. Rispondi `y` per accettare e avviare il server. Se rifiuti, Claude Code esce senza avviare il server e chiede di nuovo la prossima volta che esegui il comando.

    Il processo rimane in esecuzione nel tuo terminale in modalità server, in attesa di connessioni remote. Visualizza un URL di sessione che puoi utilizzare per [connetterti da un altro dispositivo](#connect-from-another-device), e puoi premere la barra spaziatrice per mostrare un codice QR per un accesso rapido dal tuo telefono. Mentre una sessione remota è attiva, il terminale mostra lo stato della connessione e l'attività dello strumento.

    Flag disponibili:

    | Flag                                            | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
    | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Imposta un titolo di sessione personalizzato visibile nell'elenco delle sessioni su claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
    | `--remote-control-session-name-prefix <prefix>` | Prefisso per i nomi di sessione generati automaticamente quando non è impostato un nome esplicito. Per impostazione predefinita è il nome host della tua macchina, producendo nomi come `myhost-graceful-unicorn`. Imposta `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` per lo stesso effetto.                                                                                                                                                                                                                                                                       |
    | `-c`, `--continue`                              | Riprendi la sessione che l'ultimo server in questa directory ha avviato, invece di crearne una nuova. Vedi [Riprendi le sessioni dopo aver fermato il server](#resume-sessions-after-stopping-the-server). Non può essere combinato con `--session-id`, `--spawn`, `--capacity`, o `--create-session-in-dir`. Richiede Claude Code v2.1.200 o successivo; le versioni precedenti rifiutano il flag come argomento sconosciuto.                                                                                                                                      |
    | `--session-id <id>`                             | Riprendi una sessione dal suo ID. Vedi [Riprendi le sessioni dopo aver fermato il server](#resume-sessions-after-stopping-the-server). Non può essere combinato con `--continue`, `--spawn`, `--capacity`, o `--create-session-in-dir`. Richiede Claude Code v2.1.200 o successivo; le versioni precedenti rifiutano il flag come argomento sconosciuto.                                                                                                                                                                                                            |
    | `--spawn <mode>`                                | Come il server crea le sessioni.<br />• `same-dir` (predefinito): tutte le sessioni condividono la directory di lavoro corrente, quindi possono entrare in conflitto se modificano gli stessi file.<br />• `worktree`: ogni sessione su richiesta ottiene il proprio [git worktree](/docs/it/worktrees). Richiede un repository git.<br />• `session`: modalità a sessione singola. Serve esattamente una sessione e rifiuta connessioni aggiuntive. Impostato solo all'avvio.<br />Premi `w` durante l'esecuzione per attivare/disattivare tra `same-dir` e `worktree`. |
    | `--capacity <N>`                                | Numero massimo di sessioni simultanee. Il valore predefinito è 32. Non può essere utilizzato con `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
    | `--[no-]create-session-in-dir`                  | Pre-crea una sessione nella directory corrente quando il server si avvia, così hai un posto dove digitare immediatamente. In modalità `worktree` questa sessione rimane nella directory corrente mentre le sessioni su richiesta ottengono worktree isolati. Abilitato per impostazione predefinita. Se passi `--no-create-session-in-dir` per avviare senza nessuna, Claude Code archivia le sessioni del server quando le fermi, quindi non c'è nulla da [riprendere](#resume-sessions-after-stopping-the-server).                                                |
    | `--permission-mode <mode>`                      | Imposta la [modalità di autorizzazione](/docs/it/permission-modes) iniziale per le sessioni del server, come `acceptEdits`. Accetta `manual` come alias per `default`; una modalità non riconosciuta ferma il server all'avvio ed elenca le modalità valide.                                                                                                                                                                                                                                                                                                             |
    | `--debug-file <path>`                           | Scrivi i log di debug nel file specificato.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
    | `--verbose`                                     | Mostra log dettagliati di connessione e sessione.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
    | `--sandbox` / `--no-sandbox`                    | Abilita o disabilita il [sandboxing](/docs/it/sandboxing) per l'isolamento del filesystem e della rete. Disabilitato per impostazione predefinita.                                                                                                                                                                                                                                                                                                                                                                                                                       |

    Fornisci questi flag dopo `remote-control`.

    Se passi un flag `claude` globale prima di `remote-control`, o uno script wrapper ne aggiunge uno, Claude Code non trasporta il flag alle sessioni che il server crea. Claude Code lascia passare il flag solo quando eliminarlo non è noto per cambiare ciò che quelle sessioni possono fare, come `--verbose` o `--model`. Per qualsiasi altro flag, come `--settings`, Claude Code [rifiuta di avviare](/docs/it/errors#not-carried-over-to-the-sessions-remote-control-starts) e nomina il flag da rimuovere. Prima della v2.1.248, qualsiasi opzione prima di `remote-control` faceva sì che Claude Code rifiutasse i flag dopo di essa con un errore `unknown option`.

    Claude Code verifica l'idoneità di Remote Control prima di stampare la guida, quindi `claude remote-control --help` restituisce un errore invece di questo elenco di flag quando non sei connesso con un account idoneo.
  </Tab>

  <Tab title="Sessione interattiva">
    Per avviare una normale sessione interattiva di Claude Code con Remote Control abilitato, utilizza il flag `--remote-control` (o `--rc`):

    ```bash theme={null}
    claude --remote-control
    ```

    Facoltativamente passa un nome per la sessione:

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    Questo ti dà una sessione interattiva completa nel tuo terminale che puoi anche controllare da claude.ai o dall'app Claude. A differenza di `claude remote-control` (modalità server), puoi digitare messaggi localmente mentre la sessione è anche disponibile da remoto.
  </Tab>

  <Tab title="Da una sessione esistente">
    Se sei già in una sessione di Claude Code e vuoi continuarla da remoto, utilizza il comando `/remote-control` (o `/rc`):

    ```text theme={null}
    /remote-control
    ```

    Passa un nome come argomento per impostare un titolo di sessione personalizzato:

    ```text theme={null}
    /remote-control My Project
    ```

    Questo avvia una sessione Remote Control che mantiene la cronologia della conversazione corrente.

    Finché non accetti la conferma una tantum di Remote Control, una finestra di dialogo appare prima che `/remote-control` si connetta. Seleziona **Enable Remote Control** per accettare e connetterti. Se selezioni **Never mind** o premi Esc, Claude Code non si connette e chiede di nuovo la prossima volta che esegui `/remote-control`.

    I flag `--verbose`, `--sandbox` e `--no-sandbox` non sono disponibili con questo comando.
  </Tab>

  <Tab title="VS Code">
    Nell'[estensione VS Code di Claude Code](/docs/it/vs-code), digita `/remote-control` o `/rc` nella casella del prompt.

    ```text theme={null}
    /remote-control
    ```

    Mentre Remote Control è attivo, Claude Code mostra un indicatore **Remote Control** nel footer della casella del prompt. Una volta che la sessione si connette, fai clic sull'indicatore per andare direttamente alla sessione, oppure trovala nell'elenco delle sessioni su [claude.ai/code](https://claude.ai/code). Claude Code pubblica anche l'URL della sessione nella conversazione. Per disconnetterti, esegui `/remote-control` di nuovo.

    A differenza della CLI, il comando VS Code non accetta un argomento di nome e non visualizza un codice QR. Il titolo della sessione è derivato dalla cronologia della conversazione o dal primo prompt.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Verifica lo stato della connessione
</h3>

In una sessione interattiva, mentre Remote Control è connesso, il terminale mostra un indicatore `/rc active` che si collega alla sessione su claude.ai. L'indicatore è nascosto quando il terminale è troppo stretto per contenerlo. Per vedere l'URL della sessione e un codice QR per [connetterti da un altro dispositivo](#connect-from-another-device), esegui `/remote-control` di nuovo per aprire il pannello di stato. Il pannello ti consente anche di disconnettere Remote Control mentre la tua sessione locale continua a funzionare.

<span id="session-ended-elsewhere" />Se la connessione non riesce in una sessione interattiva, l'indicatore cambia per mostrare l'errore, e Claude Code mostra il motivo in una notifica e lo aggiunge alla conversazione. Esegui `/remote-control` per riconnetterti, a meno che il motivo non dica che la sessione è stata modificata altrove:

* **Un'altra connessione ha preso il controllo di questa sessione**: un altro dispositivo o una sessione di Claude Code ce l'ha ora. Esegui `/remote-control` solo se vuoi riprendertela.
* **Questa sessione è stata terminata o archiviata da un altro dispositivo o app**: esegui `/remote-control` solo se la vuoi indietro. Claude Code riapre una sessione archiviata.
* **Il server non segnala più questa sessione**: potrebbe essere stata eliminata da un altro dispositivo o app.

<h3 id="session-url-reminders">
  Promemoria dell'URL della sessione
</h3>

Mentre Remote Control è connesso, Claude Code ti ricorda l'URL della sessione quando passare al tuo telefono o browser è più utile, così non devi trovare il collegamento in `/remote-control`. Un promemoria appare sopra la casella del prompt in uno di questi momenti:

* **Turno lungo**: quando un turno dura più a lungo di una soglia sintonizzata dal server, Claude Code mostra una notifica **Still working** con un collegamento **Check in from your phone**, così puoi seguire il turno dal tuo telefono o browser invece di aspettare al terminale. Claude Code la rimuove quando il turno termina.
* **Prompt di autorizzazione ripetuti**: dopo aver risposto a diversi [prompt di autorizzazione](/docs/it/permissions) in una sessione, una notifica **Approve tool calls from your phone** mostra l'URL della sessione. Claude Code la rimuove quando il tuo prossimo turno inizia.

I promemoria possono apparire in qualsiasi sessione connessa, incluse quelle in cui Remote Control [si connette automaticamente](#enable-remote-control-for-all-sessions). Non appaiono ogni volta che si verificano queste condizioni, e ognuno appare solo poche volte in totale tra le sessioni. Non puoi configurarli o disattivarli; ognuno si cancella da solo.

<h3 id="connect-from-another-device">
  Connettiti da un altro dispositivo
</h3>

Una volta che una sessione Remote Control è attiva, hai alcuni modi per connetterti da un altro dispositivo:

* **Apri l'URL della sessione** in qualsiasi browser per andare direttamente alla sessione su [claude.ai/code](https://claude.ai/code).
* **Scansiona il codice QR** mostrato accanto all'URL della sessione per aprirlo direttamente nell'app Claude. Con `claude remote-control`, premi la barra spaziatrice per attivare/disattivare la visualizzazione del codice QR.
* **Apri [claude.ai/code](https://claude.ai/code) o l'app Claude** e trova la sessione per nome nell'elenco delle sessioni. Nell'app mobile Claude, tocca **Code** nella navigazione per raggiungere l'elenco delle sessioni. Le sessioni Remote Control mostrano un'icona di computer con un punto di stato verde quando sono online.

Quando ti connetti, il dispositivo mostra tutti i subagent e i workflow che la sessione ha già in esecuzione in background. Ferma uno di essi dal dispositivo, e Claude Code ferma quel compito sulla tua macchina.

Il titolo della sessione remota viene scelto in questo ordine:

1. Il nome che hai passato a `--name`, `--remote-control`, o `/remote-control`
2. Il titolo che hai impostato con `/rename`
3. L'ultimo messaggio significativo nella cronologia della conversazione esistente
4. Un nome generato automaticamente come `myhost-graceful-unicorn`, dove `myhost` è il nome host della tua macchina o il prefisso che hai impostato con `--remote-control-session-name-prefix`

Se non hai impostato un nome esplicito, Claude Code aggiorna il titolo per riflettere il tuo prompt una volta che ne invii uno. Claude Code abbina i titoli generati automaticamente alla lingua della tua conversazione, o all'impostazione [`language`](/docs/it/settings-reference#language) se ne è configurata una.

Quando rinomini una sessione da claude.ai o dall'app Claude, Claude Code aggiorna anche il titolo locale mostrato in `claude --resume`. Claude Code applica lo stesso rinomina al nome della sessione mostrato sulla barra del prompt, e nell'elenco `claude agents` quando la sessione [funziona in background](/docs/it/agent-view). Prima della v2.1.221, il rinomina da claude.ai o dall'app Claude aggiornava solo il titolo, e la CLI manteneva il suo nome di sessione precedente; `/rename`, che funziona nella CLI stessa, imposta il nome su qualsiasi versione.

Se non hai ancora l'app Claude, utilizza il comando `/mobile` all'interno di Claude Code per visualizzare un codice QR per [claude.ai/mobile](https://claude.ai/mobile), che apre il negozio di app corretto per il tuo telefono.

<h3 id="what-connected-devices-see">
  Cosa vedono i dispositivi connessi
</h3>

Un dispositivo connesso mostra la conversazione nel tuo terminale mentre accade. Questi casi vanno oltre i messaggi ordinari:

* **Compattazione e `/clear`**: mentre Claude Code [compatta la conversazione](/docs/it/context-window#what-survives-compaction), i dispositivi connessi mostrano il progresso e poi dove la conversazione è stata compattata. Quando esegui `/clear`, la conversazione si ripristina anche sui dispositivi connessi.
* **Cambio di conversazioni con `/resume`**: il dispositivo connesso non riceve il titolo della conversazione commutata o la cronologia precedente, ma i nuovi messaggi in entrambe le direzioni vanno verso e da qualunque conversazione sia aperta nel tuo terminale. Per lavorare di nuovo sulla conversazione originale dal dispositivo, esegui `/resume` nel tuo terminale e torna a essa.
* **Estrazione di una sessione con `/teleport`**: quando estrai una [sessione cloud](/docs/it/claude-code-on-the-web#from-cloud-to-terminal) nel tuo terminale con `/teleport`, il dispositivo connesso non riceve la cronologia precedente della conversazione estratta. I nuovi messaggi in entrambe le direzioni vanno verso e da quella conversazione estratta, che è ora quella aperta nel tuo terminale.
* **Messaggi dalle tue altre sessioni**: con [messaggistica tra sessioni](/docs/it/cross-session-messaging), la stessa connessione trasporta messaggi tra le tue stesse sessioni su macchine diverse e dalle tue [sessioni cloud](/docs/it/claude-code-on-the-web), attraverso i server Anthropic come il resto del traffico Remote Control. [Messaggi alle sessioni su altre macchine](/docs/it/cross-session-messaging#message-sessions-on-other-machines) copre le regole di consegna e [Controlla i messaggi in entrata](/docs/it/cross-session-messaging#control-inbound-messages) copre i controlli in entrata. Richiede Claude Code v2.1.224 o successivo.
* **Prompt che invii a metà turno**: quando invii un prompt da un dispositivo connesso prima che il turno corrente termini, Claude Code lo mette in coda e lo mantiene nella trascrizione del dispositivo dopo che quel turno finisce.
* **Diff dei tuoi cambiamenti**: quando la directory della sessione è in un repository git, il pannello diff di un dispositivo connesso mostra il diff dei tuoi cambiamenti. Il dispositivo richiede il diff sulla connessione, e Claude Code lo calcola sulla tua macchina. Su un ramo che ha commit davanti al ramo predefinito del repository, il pannello mostra i cambiamenti da quando il ramo si è separato da esso, inclusi i tuoi edit non committati. Sul ramo predefinito stesso, o su un ramo che non è davanti a esso, il pannello mostra solo i tuoi cambiamenti non committati. Prima della v2.1.247, Claude Code segnalava il diff ai dispositivi connessi solo nelle sessioni servite da `claude remote-control`.
* **Modello**: quando scegli un [modello](/docs/it/model-config) da un dispositivo connesso, Claude Code esegue la sessione su quel modello. Il picker `/model` del terminale, `/status`, e `/config` mostrano quel modello. Richiede Claude Code v2.1.238 o successivo.
  * Un modello che scegli dal controllo del modello del dispositivo si applica solo alla sessione corrente. Quando invii `/model <name>` dal dispositivo a una sessione interattiva, Claude Code imposta anche il tuo predefinito per le nuove sessioni.
  * Se invii un nome che Claude Code non riconosce, come un nome visualizzato dove è previsto un ID modello, Claude Code [rifiuta la scelta](/docs/it/errors#model-is-not-a-recognized-model-id) e la sessione mantiene il suo modello corrente. Prima della v2.1.260, Claude Code salvava una scelta non riconosciuta dal controllo del modello del dispositivo, e il tuo prossimo messaggio falliva.
* **Livello di sforzo**: quando imposti il [livello di sforzo](/docs/it/model-config#adjust-effort-level) da un dispositivo connesso, con `/effort` o il controllo dello sforzo del dispositivo, Claude Code lo applica alla sessione sulla tua macchina, e claude.ai/code mostra il livello che la sessione sta utilizzando. Se hai fissato un livello con `CLAUDE_CODE_EFFORT_LEVEL`, la sessione mantiene quel livello, e Claude Code rifiuta una scelta diversa dal controllo dello sforzo. Scegliere un livello dal controllo dello sforzo richiede Claude Code v2.1.234 o successivo sulla tua macchina.
* **Riconnessione dopo un errore di connessione**: esegui `/remote-control` per riconnetterti. Se la compattazione ha riscritto la conversazione o hai cambiato conversazioni con `/resume` nel frattempo, Claude Code archivia la sessione del server che stava utilizzando invece di lasciarla nell'elenco delle sessioni. Puoi comunque trovarla [filtrando per sessioni archiviate](/docs/it/claude-code-on-the-web#archive-sessions). Cambiare conversazioni mentre un dispositivo è ancora connesso non archivia la sessione.

<h3 id="enable-remote-control-for-all-sessions">
  Abilita Remote Control per tutte le sessioni
</h3>

Remote Control si attiva solo quando esegui esplicitamente `claude remote-control`, `claude --remote-control`, o `/remote-control`, a meno che l'auto-connessione non sia attivata. Per attivare l'auto-connessione per ogni sessione interattiva, esegui `/config` all'interno di Claude Code e imposta **Enable Remote Control for all sessions**. L'interruttore accetta tre valori:

* **`true`**: connettiti automaticamente quando una sessione interattiva si avvia.
* **`false`**: disattiva l'auto-connessione, anche se un `true` dalle [impostazioni gestite](/docs/it/managed-settings) lo supera, perché Claude Code salva la scelta alle tue impostazioni utente. Un `false` nelle impostazioni di progetto o locali (`.claude/settings.json`, `.claude/settings.local.json`) disattiva l'auto-connessione anche su un `true` gestito.
* **`default`**: cancella la tua scelta e segui il valore predefinito dell'amministratore della tua organizzazione se ne è impostato uno, altrimenti il valore predefinito attuale di Claude Code.

Lo stesso interruttore appare al di fuori della CLI:

* **App Desktop**: **Settings > Claude Code > Enable remote control by default**.
* **Estensione VS Code**: **Enable Remote Control for all sessions** nella sezione Impostazioni del [menu dei comandi](/docs/it/vs-code#use-the-prompt-box). Richiede Claude Code v2.1.203 o successivo.

Per attivare l'auto-connessione da un file di impostazioni, imposta [`remoteControlAtStartup`](/docs/it/settings-reference#remotecontrolatstartup) a `true` nel tuo file `~/.claude/settings.json` utente o nelle [impostazioni gestite](/docs/it/managed-settings). Nelle impostazioni di progetto o locali (`.claude/settings.json`, `.claude/settings.local.json`), Claude Code onora un `false` e disattiva l'auto-connessione per quel repository, ma ignora un `true`, quindi un file controllato non può attivare Remote Control per tutti coloro che aprono il repository.

L'auto-connessione accede con il tuo account claude.ai, quindi una sessione che avvia appare solo nelle tue app Claude e non concede a nessun altro l'accesso.

Con questa impostazione attiva, ogni processo Claude Code interattivo registra una sessione remota. Se esegui più istanze, ognuna ottiene la sua sessione remota. Per eseguire più sessioni simultanee da un singolo processo, utilizza invece la [modalità server](#start-a-remote-control-session).

<h3 id="resume-sessions-after-stopping-the-server">
  Riprendi le sessioni dopo aver fermato il server
</h3>

Quando fermi `claude remote-control` con Ctrl+C, le sessioni che stava servendo smettono di rispondere dal tuo telefono o browser. Finché non stavi eseguendo un altro `claude remote-control` nella stessa directory e non hai avviato questo con `--no-create-session-in-dir`, Claude Code non le archivia. Per riportarle indietro, esegui uno di questi comandi nella stessa directory:

* **`claude remote-control`**: riporta indietro ogni sessione che il server stava servendo.
* **`claude remote-control --continue`**: riporta indietro solo la sessione con cui il server si è avviato, ed esce quando quella sessione termina. Se questa directory non ha un record, Claude Code utilizza il più recente dai worktree git di questo repository.
* **`claude remote-control --session-id <id>`**: riporta indietro solo la sessione il cui ID passi, ed esce quando quella sessione termina. L'ID è la parte dell'URL della sessione su claude.ai/code tra `/code/` e qualsiasi `?`.

Questi comandi funzionano per circa quattro ore dopo che il server si è fermato. Dopo di che, esegui `claude remote-control` per avviare una nuova sessione. Se hai archiviato una sessione nel frattempo, `--continue` e `--session-id` la disarchiviano su Claude Code v2.1.228 o successivo.

Per riportare indietro una sessione che hai avviato con `claude --remote-control` o `/remote-control`, riprendi la conversazione con `claude --continue` o `claude --resume`. Se Claude Code si riconnette, e a quale sessione, dipende dal [record di riconnessione](#resume-outcomes) della conversazione.

Se riprendi la conversazione in un secondo terminale mentre il primo ha ancora Remote Control attivo, Claude Code stampa un avviso nel secondo terminale e lascia Remote Control disattivo lì invece di prendere la sessione dal primo. Mentre Remote Control rimane disattivo lì, Claude in quel terminale non vede [le tue sessioni su altre macchine](/docs/it/cross-session-messaging#see-which-sessions-claude-can-reach), e non possono raggiungerlo. Esegui `/remote-control` nel secondo terminale per spostare Remote Control lì.

Quando riprendi una conversazione in Claude Desktop o un'estensione IDE che aveva Remote Control attivo, Claude Code la riallega alla sessione claude.ai esistente invece di aggiungerne una nuova all'elenco delle sessioni.

<h2 id="connection-and-security">
  Connessione e sicurezza
</h2>

La tua sessione locale di Claude Code effettua solo richieste HTTPS in uscita e non apre mai porte in ingresso sulla tua macchina. Quando avvii Remote Control, si registra con l'API Anthropic e esegue il polling per il lavoro. Quando ti connetti da un altro dispositivo, il server instrada i messaggi tra il client web o mobile e la tua sessione locale su una connessione in streaming.

Tutto il traffico viaggia attraverso l'API Anthropic su TLS, lo stesso trasporto di sicurezza di qualsiasi sessione di Claude Code. La connessione utilizza più credenziali di breve durata, ognuna limitata a un singolo scopo e con scadenza indipendente. Quando la credenziale di registrazione di un server `claude remote-control` scade, il server si registra di nuovo con l'API Anthropic e continua a servire le sue sessioni.

Mentre Remote Control è connesso, la trascrizione della sessione, inclusi i tuoi messaggi, le risposte di Claude e l'attività degli strumenti, viene archiviata sui server Anthropic. La trascrizione archiviata mantiene la conversazione sincronizzata tra i tuoi dispositivi e consente alla sessione di riconnettersi dopo un'interruzione di rete. L'esecuzione e l'accesso al filesystem rimangono sulla tua macchina, e le trascrizioni archiviate vengono conservate secondo la politica di [utilizzo dei dati](/docs/it/data-usage).

Per disattivare completamente Remote Control, utilizza l'impostazione [`disableRemoteControl`](/docs/it/settings-reference#disableremotecontrol). Le organizzazioni con requisiti di conformità come Zero Data Retention non possono abilitare Remote Control.

<h2 id="trusted-devices">
  Dispositivi affidabili
</h2>

<Note>
  Dispositivi affidabili è attualmente in beta. Le funzionalità e la funzionalità possono evolversi man mano che l'esperienza viene perfezionata.

  Dispositivi affidabili è disponibile su piani Pro, Max, Team ed Enterprise ed è disabilitato per impostazione predefinita. Su piani Team ed Enterprise, un Owner lo abilita per l'organizzazione. Su piani Pro e Max, Lei abilita **Require trusted devices** da sola nelle impostazioni, sulla pagina Cowork o Account.
</Note>

Dispositivi affidabili richiede a ogni membro della vostra organizzazione, o a Lei sola su un piano Pro o Max, di verificare il proprio dispositivo prima di poter visualizzare o controllare le sessioni Remote Control da claude.ai, dalle app Claude per dispositivi mobili o da Claude Desktop. Lega l'accesso a Remote Control a un dispositivo noto e a un'autenticazione recente, non solo a un account connesso.

Quando l'impostazione è attiva, l'interazione con una sessione Remote Control richiede entrambi i seguenti:

* **Un dispositivo registrato**: ogni browser, telefono o app desktop che un membro utilizza per Remote Control registra le proprie credenziali. La registrazione viene offerta solo poco dopo un accesso completo, quindi un dispositivo si unisce all'elenco affidabile come parte di un'autenticazione reale piuttosto che silenziosamente in background.
* **Un accesso recente**: l'accesso del membro non deve essere più vecchio di 18 ore. Invece di accedere di nuovo ogni giorno, i membri confermano la presenza con Face ID, Touch ID, Windows Hello o una passkey. Questo passaggio di autenticazione biometrica aggiorna la sessione immediatamente.

I controlli biometrici vengono eseguiti sul dispositivo attraverso il sistema operativo o il browser, lo stesso meccanismo dell'accesso con passkey. Anthropic non riceve né archivia mai impronte digitali, dati facciali o altre informazioni biometriche. Solo la chiave pubblica del dispositivo e i metadati di base come il nome visualizzato, la piattaforma e l'ora di registrazione vengono archiviati.

L'impostazione si applica solo a Remote Control. La chat Claude regolare, Claude Code nel terminale e l'utilizzo dell'API non sono interessati.

<h3 id="enable-trusted-devices-for-your-organization">
  Abilita Dispositivi affidabili per un'organizzazione Team o Enterprise
</h3>

Un Owner abilita l'impostazione dalle impostazioni dell'organizzazione di claude.ai.

<Steps>
  <Step title="Vai alla pagina Capabilities">
    Vai a [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities). L'interruttore **Require trusted devices** appare in quella sezione.
  </Step>

  <Step title="Attiva Require trusted devices">
    L'impostazione si applica a ogni membro dell'organizzazione e alle sessioni Remote Control avviate dopo aver abilitato l'interruttore. Le sessioni che erano già in esecuzione prima dell'attivazione dell'interruttore non sono protette retroattivamente e continuano senza il requisito del dispositivo fino a quando non terminano. L'ambito per team o per progetto non è disponibile.
  </Step>

  <Step title="Comunica ai membri cosa aspettarsi">
    La prima volta che un membro visualizza o controlla una nuova sessione Remote Control da un browser, telefono o app desktop dopo l'abilitazione dell'impostazione, gli viene chiesto di registrare quel dispositivo. Informarli in anticipo evita confusione.
  </Step>
</Steps>

<h3 id="what-members-see">
  Cosa vedono i membri
</h3>

La registrazione è un passaggio una tantum per dispositivo. Dopo di che, l'unico cambiamento visibile è un occasionale prompt biometrico.

* **Primo utilizzo su ogni dispositivo**: al membro viene chiesto di registrarsi. Se il suo accesso non è recente, accede prima attraverso il tuo flusso normale, incluso SSO se configurato, quindi conferma la registrazione.
* **Giorno per giorno**: i membri con un dispositivo registrato e un accesso recente non vedono prompt. Quando l'accesso invecchia oltre 18 ore, la prossima interazione Remote Control mostra un singolo prompt Face ID, Touch ID, Windows Hello o passkey.
* **Dispositivi non registrati**: le sessioni Remote Control non possono essere visualizzate o controllate fino a quando il dispositivo non è registrato. La chat Claude regolare su quel dispositivo non è interessata.
* **Nessun autenticatore di piattaforma**: i membri su una macchina senza Face ID, Touch ID o Windows Hello possono utilizzare una chiave di sicurezza hardware, o accedere di nuovo invece di eseguire l'autenticazione.
* **Nel terminale**: la macchina che esegue Claude Code riceve le proprie credenziali automaticamente quando lo sviluppatore accede alla CLI. Non c'è un passaggio di registrazione separato nel terminale.

<h3 id="manage-enrolled-devices">
  Gestisci i dispositivi registrati
</h3>

I membri possono rivedere e revocare i propri dispositivi dalle impostazioni dell'account.

Apri [claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) e trova la sezione **Trusted devices** per vedere ogni dispositivo registrato con il suo nome, piattaforma e data di registrazione. La rimozione di un dispositivo revoca immediatamente le sue credenziali, e il dispositivo può registrarsi di nuovo in seguito dopo un accesso aggiornato. Le credenziali scadono anche da sole se non rinnovate, quindi un dispositivo inutilizzato cade automaticamente dall'elenco affidabile.

Per un dispositivo perso o rubato, il membro lo rimuove da questa pagina. Se il membro non riesce ad accedere, un amministratore può utilizzare **Sign out everywhere** nella console di amministrazione per revocare ogni sessione e dispositivo registrato per quel membro, dopo di che il membro registra di nuovo i dispositivi che ancora possiede.

<h2 id="remote-control-vs-cloud-sessions">
  Remote Control vs cloud sessions
</h2>

Remote Control e [cloud sessions](/docs/it/claude-code-on-the-web) utilizzano entrambi l'interfaccia claude.ai/code. La differenza chiave è dove viene eseguita la sessione: Remote Control viene eseguito sulla tua macchina, quindi i tuoi server MCP locali, strumenti e configurazione del progetto rimangono disponibili. Una cloud session viene eseguita su infrastruttura cloud, gestita da Anthropic per impostazione predefinita.

Utilizza Remote Control quando sei nel mezzo di un lavoro locale e vuoi continuare da un altro dispositivo. Utilizza una cloud session quando vuoi avviare un'attività senza alcuna configurazione locale, lavorare su un repository che non hai clonato, o eseguire più attività in parallelo.

<h2 id="mobile-push-notifications">
  Notifiche push mobili
</h2>

Quando Remote Control è attivo, Claude può inviare notifiche push al tuo telefono.

Claude decide quando inviare una notifica. Tipicamente ne invia una quando un'attività a lunga esecuzione termina o quando ha bisogno di una decisione da te per continuare. Puoi anche richiedere una notifica nel tuo prompt, ad esempio `notify me when the tests finish`. Oltre ai due interruttori on/off qui sotto, non c'è configurazione per evento.

Per configurare le notifiche push mobili:

<Steps>
  <Step title="Installa l'app Claude per dispositivi mobili">
    Scarica l'app Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).
  </Step>

  <Step title="Accedi con il tuo account Claude Code">
    Utilizza lo stesso account e organizzazione che usi per Claude Code nel terminale.
  </Step>

  <Step title="Consenti le notifiche">
    Accetta il prompt di autorizzazione delle notifiche dal sistema operativo.
  </Step>

  <Step title="Abilita push in Claude Code">
    Nel tuo terminale, esegui `/config` e abilita **Push when Claude decides** per notifiche proattive, **Push when actions required** per prompt di autorizzazione e domande, o entrambi.
  </Step>
</Steps>

Se le notifiche non arrivano:

* Se `/config` mostra **No mobile registered**, apri l'app Claude sul tuo telefono in modo che possa aggiornare il suo token push. L'avviso si cancella la prossima volta che Remote Control si connette.
* Su iOS, le modalità Focus e i riepiloghi delle notifiche possono sopprimere o ritardare le notifiche push. Controlla Impostazioni → Notifiche → Claude.
* Su Android, l'ottimizzazione aggressiva della batteria può ritardare la consegna. Escludi l'app Claude dall'ottimizzazione della batteria nelle impostazioni di sistema.

Claude Code salta le notifiche push mobili mentre stai digitando o sei concentrato sul terminale connesso. A partire da v2.1.181, puoi impostare [`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/it/env-vars) su un percorso di file marcatore per estendere questo a qualsiasi momento in cui sei alla macchina, anche in un'altra finestra: le notifiche vengono saltate mentre il file esiste. Configura un listener di blocco dello schermo o uno strumento simile per creare il file quando lo schermo si sblocca ed eliminarlo quando lo schermo si blocca.

<h2 id="limitations">
  Limitazioni
</h2>

* **Una sessione remota per processo interattivo**: al di fuori della modalità server, ogni istanza di Claude Code supporta una sessione remota alla volta. Utilizza la [modalità server](#start-a-remote-control-session) per eseguire più sessioni simultanee da un singolo processo.
* **Il processo locale deve continuare a funzionare**: Remote Control viene eseguito come processo locale. Se chiudi il terminale, esci da VS Code, o altrimenti interrompi il processo `claude`, la sessione va offline finché non la [ripristini](#resume-sessions-after-stopping-the-server). A meno che Claude non sia nel mezzo di un'attività, claude.ai e l'app Claude mostrano la sessione come offline entro pochi secondi dopo l'uscita del processo. Per mantenere una sessione in esecuzione su una macchina remota dopo la disconnessione da SSH, avviala all'interno di `tmux` o `screen`.
* **Sessioni bloccate in modalità server**: se una sessione servita da `claude remote-control` si blocca, inviale un messaggio da un dispositivo connesso. Claude Code la serve di nuovo. Non è necessario riavviare il server. Richiede Claude Code v2.1.238 o successivo.
* **Rifiuti HTTP 403 su una sessione connessa**: una volta che una sessione interattiva è connessa, Claude Code continua a riprovare per un massimo di tre minuti quando qualcosa tra la tua macchina e i server di Anthropic risponde con HTTP 403, come può accadere dopo un cambio VPN o di rete. Se i rifiuti durano più a lungo, Claude Code si disconnette e il motivo indica cosa ha rifiutato: un edge di rete, o un proxy, VPN o firewall sulla tua rete.
* **Interruzione di rete prolungata**: se la tua macchina è accesa ma non riesce a raggiungere la rete, quello che fai dopo dipende dalla modalità:
  * **Modalità server**: Claude Code rinuncia dopo circa 10 minuti e il processo `claude remote-control` esce. Esegui di nuovo `claude remote-control` per avviare una nuova sessione.
  * **Sessione interattiva**: continua a lavorare localmente. Claude Code continua a riprovare finché dura l'interruzione e si riconnette automaticamente quando la rete ritorna.
* **Heartbeat di presenza non riusciti**: se una sessione interattiva si disconnette con `could not reach the Remote Control server for about 30 minutes`, esegui `/remote-control` per riconnetterti. Claude Code mostra questo messaggio solo quando gli heartbeat di presenza della sessione non hanno funzionato mentre il resto della connessione è rimasto attivo; registra di nuovo la sessione per circa 30 minuti prima di disconnettersi.
* **Dialoghi inoltrati scadono**: Claude Code mantiene aperti i prompt di autorizzazione e le domande `AskUserQuestion` finché non le rispondi. Quando Claude Code inoltra un altro tipo di dialogo alla sessione remota, come il prompt di scelta del modello mostrato dopo un rifiuto di sicurezza, attende cinque minuti per impostazione predefinita, quindi chiude il dialogo e continua con il valore predefinito senza azione del dialogo. Imposta [`dialogExpiry`](/docs/it/settings-reference#dialogexpiry) per regolare o disabilitare la scadenza. Richiede Claude Code v2.1.224 o successivo.
* **Il prompt di consenso dei crediti di utilizzo Fable non viene inoltrato**: Claude Code mostra il prompt di consenso dei crediti di utilizzo [Fable](/docs/it/model-config#fable-and-usage-credits) a metà sessione solo dove viene eseguita la sessione, non sul tuo dispositivo. Quando la sessione viene eseguita in un terminale e nessuno lì risponde prima che Claude Code chiuda il prompt, il turno termina senza inviare la richiesta; vedi [Il prompt per confermare non ha ricevuto risposta](/docs/it/errors#the-prompt-to-confirm-went-unanswered).
* **Alcuni comandi sono solo locali**: i comandi che funzionano solo nell'interfaccia del terminale, come `/plugin` o `/resume`, funzionano solo dalla CLI locale, indipendentemente dal fatto che tu passi un argomento o meno. I seguenti funzionano da mobile e web:
  * Comandi con output di testo: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits`, `/recap` e `/reload-plugins`. `/usage-credits` stampa l'URL di fatturazione invece di aprire un browser. `/reload-plugins` funziona solo quando la sessione viene eseguita in un terminale interattivo; una sessione senza uno lo rifiuta.
  * `/model`, `/effort`, `/fast`, `/color` e `/rename`: passa il valore come argomento, ad esempio `/model sonnet` o `/effort high`. Da mobile e web, `/model` e `/effort` accettano l'argomento al posto del selettore del terminale o del cursore.
  * `/mcp`: dall'app mobile, restituisce un riepilogo testuale dello stato del server invece di aprire il selettore. Sul web, `/mcp` da solo apre una directory dei [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) invece di restituire il riepilogo. I sottocomandi `reconnect`, `enable` e `disable` [](/docs/it/commands#all-commands) funzionano da entrambi. A differenza della CLI locale, `/mcp reconnect` senza nome del server riconnette ogni server che ha avuto un errore o necessita autenticazione.
  * `/config`: dall'app mobile, passa `key=value` per impostare un'impostazione, o eseguilo senza argomenti per elencare le chiavi che puoi impostare. Sul web, `/config` apre la sezione Claude Code delle tue impostazioni, e ignora il testo dopo il comando.
  * Su Team ed Enterprise, `/usage-credits` da mobile o web non invia una [richiesta di crediti di utilizzo al tuo amministratore](/docs/it/costs#add-usage-credits-to-your-subscription). L'invio richiede una conferma che appare solo nella CLI interattiva, quindi il comando ti dice di eseguirlo lì. Prima della v2.1.211, il modulo di testo inviava la richiesta senza conferma.
  * `/autocompact`, dalla v2.1.221: passa la dimensione della finestra come argomento, ad esempio `/autocompact 500k`. Senza argomenti, stampa la dimensione della finestra corrente come testo invece di aprire il dialogo che il comando mostra in una sessione di terminale.
  * `/advisor`, dalla v2.1.260: passa il modello come argomento, ad esempio `/advisor opus`, o passa `off` per disattivare l'advisor. Entrambi i moduli si applicano solo alla sessione corrente e lasciano invariato il tuo valore predefinito salvato. Senza argomenti, stampa l'advisor corrente come testo invece di aprire il selettore.
  * `/output-style`, dalla v2.1.269: passa il nome dello stile come argomento, ad esempio `/output-style concise`, o eseguilo senza argomenti per elencare gli stili. Da mobile e web, puoi elencare e selezionare solo gli [stili incorporati](/docs/it/output-styles#built-in-output-styles). Per utilizzare uno [stile personalizzato](/docs/it/output-styles#create-a-custom-output-style), selezionalo nella sessione stessa.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control requires a claude.ai subscription"
</h3>

Non sei autenticato con un account claude.ai, oppure un'altra credenziale sta prendendo precedenza sul tuo accesso. Il messaggio assume una di queste forme:

* Non connesso, da `/remote-control` o `--remote-control`: `Remote Control requires a claude.ai subscription.` o `/remote-control requires a claude.ai subscription.`
* Non connesso, da `claude remote-control`: `You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions.`
* Connesso, ma una chiave API o un token è in uso: `Remote Control requires claude.ai subscription auth.` seguito dalla credenziale in uso, come `ANTHROPIC_API_KEY is set, so this session is using API-key auth`. Un'impostazione `apiKeyHelper` e `ANTHROPIC_AUTH_TOKEN` sono denominate allo stesso modo.

Esegui `claude auth login` e scegli l'opzione claude.ai. Se il messaggio nomina `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN`, rimuovilo ovunque sia impostato: il tuo ambiente shell o il blocco `env` di un [file di impostazioni](/docs/it/settings-reference#env). Se nomina `apiKeyHelper`, rimuovi quella impostazione.

Prima della v2.1.206, l'esecuzione di `/remote-control` mentre non eri connesso segnalava `Unknown command: /remote-control` invece di questo messaggio.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control requires a full-scope login token"
</h3>

Sei autenticato con un token di lunga durata da `claude setup-token` o dalla variabile di ambiente `CLAUDE_CODE_OAUTH_TOKEN`. Questi token possono solo effettuare richieste di modello, quindi non possono stabilire sessioni Remote Control. Esegui `claude auth login` per autenticarti con un token di sessione a scopo completo.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Unable to determine your organization for Remote Control eligibility"
</h3>

Le informazioni dell'account memorizzate nella cache sono obsolete o incomplete. Esegui `claude auth login` per aggiornarle.

<h3 id="remote-control-isn’t-enabled-for-this-account">
  "Remote Control isn't enabled for this account"
</h3>

Claude Code ha verificato la disponibilità di Remote Control per l'account con cui sei autenticato e il controllo è risultato disabilitato. La causa più comune è la cache dei diritti che è obsoleta dopo un cambio di piano. Esegui `claude auth logout` quindi `claude auth login` per aggiornarli, e aggiorna Claude Code se stai utilizzando una versione precedente.

Esegui `claude doctor` per vedere quale controllo di idoneità individuale ha fallito. I conflitti delle variabili di ambiente, i controlli non raggiungibili e l'impostazione Remote Control della tua organizzazione producono ciascuno il proprio messaggio, quindi questo errore significa il controllo a livello di account stesso.

Prima della v2.1.239, questo messaggio diceva "Remote Control is not yet enabled for your account". Prima della v2.1.154, una variabile che disabilita la valutazione dei feature-flag, come `DISABLE_TELEMETRY` o `DO_NOT_TRACK`, produceva anche questo messaggio; la voce "Remote Control requires feature-flag evaluation" di seguito copre quella configurazione.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Couldn't verify Remote Control eligibility"
</h3>

Claude Code non ha potuto raggiungere il servizio di feature-flag per verificare se Remote Control è abilitato per il tuo account, in genere perché sei offline o un proxy sta bloccando la richiesta. Riprova una volta che hai accesso alla rete, oppure esegui `claude doctor` per i dettagli. Il messaggio correlato "Couldn't verify your organization's Remote Control policy" significa che Claude Code non ha potuto leggere quella politica, e ha la stessa soluzione. Entrambi i messaggi sono stati aggiunti nella v2.1.178.

<h3 id="remote-control-requires-feature-flag-evaluation">
  "Remote Control requires feature-flag evaluation"
</h3>

Una di queste variabili è impostata: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, o `DISABLE_GROWTHBOOK`](/docs/it/env-vars). Ognuna di esse disabilita la valutazione dei feature-flag da cui dipende la disponibilità di Remote Control, e il messaggio completo nomina la variabile che Claude Code ha trovato. Annulla l'impostazione di quella variabile ovunque sia impostata, nel tuo ambiente shell o nel blocco `env` di un [file `settings.json`](/docs/it/settings-reference#all-settings). Nelle versioni precedenti alla 2.1.154, la stessa configurazione produce "Remote Control is not yet enabled for your account" invece.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control is only available when using Claude via api.anthropic.com"
</h3>

La sessione non sta comunicando direttamente con l'API Anthropic, quindi non c'è alcun backend claude.ai con cui associarsi. Questo accade su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. Accade anche quando [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) punta a un host diverso da `api.anthropic.com`, come un [gateway LLM](/docs/it/llm-gateway) o proxy, anche se accedi con claude.ai. Prima della v2.1.196, Claude Code non mostrava questo messaggio per un `ANTHROPIC_BASE_URL` personalizzato. Vedi il [riferimento degli errori](/docs/it/errors#remote-control-requires-the-anthropic-api) per l'elenco completo delle cause.

Il messaggio nomina cosa ha instradato la sessione lontano dall'API Anthropic, come `CLAUDE_CODE_USE_BEDROCK` o un `ANTHROPIC_BASE_URL` personalizzato. Se hai un accesso claude.ai idoneo, annulla l'impostazione della variabile denominata, rimuovila dalla chiave `env` nelle [impostazioni](/docs/it/settings) se l'hai impostata lì, e riavvia la sessione. Prima della v2.1.219, il messaggio era solo la frase nell'intestazione di questa sezione, quindi nelle versioni precedenti controlla il tuo ambiente stesso per le variabili del provider come `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, e per `ANTHROPIC_BASE_URL`.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control is disabled by your organization's policy"
</h3>

Una politica blocca Remote Control, oppure Claude Code non ha potuto caricare la politica della tua organizzazione su questa macchina e mantiene Remote Control disabilitato nel frattempo. Controlla queste cause in ordine:

* **L'errore menziona `disableRemoteControl`**: il tuo amministratore IT ha disabilitato Remote Control su questo dispositivo tramite [impostazioni gestite](/docs/it/managed-settings), indipendentemente dall'interruttore a livello di organizzazione e da come sei autenticato.
* **Il tuo piano claude.ai è Pro o Max**: Claude Code è ancora autenticato con un'organizzazione Team o Enterprise da un accesso precedente, quindi controlla la politica Remote Control di quell'organizzazione. Esegui `/status` per vedere quale piano e organizzazione usa il tuo accesso. Esegui `claude auth logout` quindi `claude auth login` per accedere di nuovo con il tuo piano attuale.
* **La politica dell'organizzazione non è stata caricata su questa macchina**: esegui `claude doctor` e leggi la riga `Organization policy`. Se la riga mostra che la politica non è caricata, è quello che mantiene Remote Control disabilitato. Prima della v2.1.261, `claude doctor` non stampava questa riga.
* **Il messaggio non dice di contattare l'amministratore della tua organizzazione**: la tua organizzazione ha una configurazione HIPAA incompatibile con Remote Control, e `/status` elenca `HIPAA` nella sua riga `Compliance`. In questo stato l'interruttore Remote Control del pannello di amministrazione è disattivato, quindi un Owner non può modificarlo lì. Contatta il supporto Anthropic per discutere le opzioni. Prima della v2.1.267, questo caso mostrava "Remote Control isn't available for your organization due to its compliance policy" invece.
* **Altrimenti, un Owner non l'ha abilitato per la tua organizzazione**: Remote Control è disabilitato per impostazione predefinita su piani Team e Enterprise. Un Owner può abilitarlo su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) attivando l'interruttore **Remote Control**. Questo interruttore è un'impostazione organizzativa lato server.

<h3 id="remote-credentials-fetch-failed">
  "Remote credentials fetch failed"
</h3>

Claude Code non ha potuto ottenere una credenziale di breve durata dall'API Anthropic per stabilire la connessione. Esegui di nuovo con `--verbose` per vedere l'errore completo:

```bash theme={null}
claude remote-control --verbose
```

Cause comuni:

* Non sei connesso: esegui `claude` e utilizza `/login` per autenticarti con il tuo account claude.ai. L'autenticazione con chiave API non è supportata per Remote Control.
* Problema di rete o proxy: un firewall o proxy potrebbe bloccare la richiesta HTTPS in uscita. Remote Control richiede l'accesso all'API Anthropic sulla porta 443.
* Creazione della sessione non riuscita: se vedi anche `Session creation failed — see debug log`, l'errore si è verificato in precedenza nella configurazione. Verifica che il tuo abbonamento sia attivo.

Un token di accesso obsoleto non causa questo errore. Quando l'API Anthropic rifiuta il token salvato, ad esempio perché un altro processo Claude Code l'ha già aggiornato, Claude Code aggiorna il token e riprova da solo. Prima della v2.1.224, un token obsoleto non riusciva ad avviare Remote Control con questo messaggio, quindi le sessioni impostate per [connettersi automaticamente](#enable-remote-control-for-all-sessions) potevano non riuscire intermittentemente all'avvio.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Couldn't reconnect to your Remote Control session"
</h3>

Quando riprendi una conversazione con `claude --resume` o `claude --continue`, Claude Code si riconnette alla sessione Remote Control registrata in quella conversazione. Questo messaggio significa che la riconnessione non è riuscita per un motivo che potrebbe essere temporaneo, come un'interruzione di rete o un errore del server, quindi Claude Code non può confermare se la sessione remota esiste ancora.

Esegui `/remote-control` per riprovare la connessione, o avvia una nuova sessione con `claude --remote-control` per creare una nuova sessione Remote Control. La tua sessione locale continua a funzionare senza Remote Control nel frattempo.

<span id="resume-outcomes" />Quando riprendi, puoi anche ottenere uno di questi risultati invece di questo messaggio:

* **Il server segnala che la sessione registrata è scomparsa, oppure il record di riconnessione nomina un account diverso**: Claude Code si basa su quello che dice il record di riconnessione della conversazione:
  * **Il record nomina il tuo account autenticato**: Claude Code avvia una sessione sostitutiva con un nome generato automaticamente e lascia i messaggi precedenti della conversazione fuori da essa. Ottieni questo dopo aver eliminato la sessione da claude.ai o dall'app Claude, ad esempio.
  * **Il record nomina un account diverso**: Claude Code avvia una nuova sessione senza i messaggi precedenti della conversazione e senza mostrare un messaggio, indipendentemente dal fatto che la sessione registrata esista ancora.
  * **Il record non dice quale account possedeva la sessione, oppure Claude Code non può leggere il tuo accesso salvato**: Claude Code mostra [`Previous session is unavailable — run /remote-control to start a new one`](#previous-session-is-unavailable) invece di questo messaggio, non avvia nulla, e rimuove il record dalla conversazione.
* **Hai disattivato Remote Control prima di riprendere**: a meno che l'app che ospita Claude Code non gli abbia detto che l'app possiede la sessione claude.ai, Claude Code ha rimosso il record di riconnessione quando hai disattivato Remote Control dal [pannello di stato](#check-connection-status) della CLI, dall'estensione VS Code, o da un host costruito su [Agent SDK](/docs/it/agent-sdk/overview), quindi non si riconnette. Quando un'app proprietaria l'ha disattivato, Claude Code ha mantenuto il record e si riconnette.
* **Un altro Claude Code su questa macchina ha ancora la sessione**: vedi un avviso che inizia con `Remote Control not started here`, e Claude Code [lascia Remote Control disabilitato nella sessione ripresa](#resume-sessions-after-stopping-the-server). Esegui `/remote-control` lì per spostarlo.

<span id="reconnect-history" />Prima della v2.1.232, Claude Code ha risposto diversamente quando il server ha segnalato che la sessione registrata era scomparsa. Dalla v2.1.227 alla v2.1.231, Claude Code ha rifiutato di avviare una sostituzione anche quando il record corrispondeva al tuo account. Fino alla v2.1.226, Claude Code ha avviato una sostituzione indipendentemente dal fatto che il record corrispondesse al tuo account, e nella v2.1.224 fino alla v2.1.226 l'ha creata con l'account autenticato su quella macchina, mai con un altro account, senza caricare i messaggi precedenti della conversazione su di essa. Prima della v2.1.200, Claude Code ha creato una nuova sessione dopo qualsiasi errore di riconnessione.

<h3 id="previous-session-is-unavailable">
  "Previous session is unavailable — run /remote-control to start a new one"
</h3>

Claude Code non ha potuto ripristinare la sessione Remote Control precedente e si è fermato invece di avviare una nuova da solo. Puoi vedere questo messaggio dopo aver ripreso una conversazione con `claude --resume` o `claude --continue`, oppure dopo che Claude Code [si è riconnesso da solo dopo una disconnessione](/docs/it/errors#remote-control-couldnt-refresh-your-login).

Esegui `/remote-control` per avviare una nuova sessione Remote Control con l'accesso attuale; la tua sessione locale continua a funzionare senza Remote Control nel frattempo. Il messaggio correlato `Remote Control could not verify the signed-in account — run /remote-control to reconnect` ha la stessa soluzione; Claude Code lo mostra quando l'account autenticato è cambiato o non poteva essere letto tra la sua convalida e la riconnessione. Se esegui `/remote-control` dopo `Previous session is unavailable` senza riavviare Claude Code prima, Claude Code lascia i messaggi precedenti della conversazione fuori dalla nuova sessione.

Al ripristino, Claude Code [avvia una nuova sessione al suo posto](#resume-outcomes) solo se il record di riconnessione della conversazione nomina l'account che possedeva la sessione, perché il server segnala una sessione che hai eliminato e una sessione posseduta da un altro account allo stesso modo. Claude Code prima della v2.1.227 non ha registrato quell'account, e Claude Code non può controllare il record quando non può leggere il tuo accesso salvato. Claude Code prima della v2.1.232 mostrava `Remote Control could not resume the previous session under the current login — run /remote-control to start fresh` invece, in [un diverso insieme di casi](#reconnect-history).

<h3 id="remote-control-got-an-unexpected-server-response">
  "Remote Control got an unexpected server response"
</h3>

Il server Remote Control ha accettato una richiesta ma ha risposto in una forma che questa versione di Claude Code non poteva leggere, durante la creazione della sessione remota o il recupero delle sue credenziali. Riprovare sulla stessa versione fallisce allo stesso modo. Esegui `claude update`, quindi esegui `/remote-control` per riconnetterti. Questo messaggio è stato aggiunto nella v2.1.225.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  "Your organization requires Trusted Devices for Remote Control, but this device is not enrolled"
</h3>

La tua organizzazione ha [Trusted Devices](#trusted-devices) abilitato e questa macchina non si è ancora registrata. Esegui `/login` in Claude Code. La registrazione avviene come parte dell'accesso, e non c'è un comando di registrazione separato.

<h3 id="session-expired-for-trusted-device-check">
  "session expired for trusted-device check"
</h3>

Il tuo accesso è più vecchio di 18 ore. Esegui `/login` in Claude Code, o conferma con Face ID, Touch ID, Windows Hello, o una passkey quando claude.ai o l'app mobile te lo chiede. Vedi [Trusted Devices](#trusted-devices).

<h2 id="choose-the-right-approach">
  Scegli l'approccio giusto
</h2>

Claude Code offre diversi modi di lavorare quando non sei al tuo terminale. Differiscono in ciò che attiva il lavoro, dove Claude viene eseguito e quanto setup è necessario.

|                                                          | Trigger                                                                                               | Claude viene eseguito su                                                                    | Setup                                                                                                                             | Migliore per                                                         |
| :------------------------------------------------------- | :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| [Dispatch](/docs/it/desktop#sessions-from-dispatch)           | Invia un'attività dall'app mobile Claude                                                              | La tua macchina (Desktop)                                                                   | [Associa l'app mobile a Desktop](https://support.claude.com/en/articles/13947068)                                                 | Delegare il lavoro mentre sei via, setup minimo                      |
| [Remote Control](/docs/it/remote-control)                     | Guida una sessione in esecuzione da [claude.ai/code](https://claude.ai/code) o dall'app mobile Claude | La tua macchina (CLI o VS Code)                                                             | Esegui `claude remote-control`                                                                                                    | Guidare il lavoro in corso da un altro dispositivo                   |
| [Channels](/docs/it/channels)                                 | Invia eventi da un'app di chat come Telegram o Discord, o dal tuo server                              | La tua macchina (CLI)                                                                       | [Installa un plugin channel](/docs/it/channels#quickstart) o [crea il tuo](/docs/it/channels-reference)                                     | Reagire a eventi esterni come errori CI o messaggi di chat           |
| [Slack](/docs/it/slack)                                       | Menziona `@Claude` in un canale del team                                                              | Cloud Anthropic                                                                             | [Installa l'app Slack](/docs/it/slack#setting-up-claude-code-in-slack) con [Claude Code sul web](/docs/it/claude-code-on-the-web) abilitato | PR e revisioni dalla chat del team                                   |
| [Self-hosted environments](/docs/it/self-hosted-environments) | Avvia una [sessione cloud](/docs/it/claude-code-on-the-web) e scegli l'ambiente della tua organizzazione   | L'infrastruttura della tua organizzazione                                                   | [Distribuisci runner](/docs/it/self-hosted-environments-quickstart), su piani Team e Enterprise                                        | Sessioni cloud che devono essere eseguite all'interno della tua rete |
| [Scheduled tasks](/docs/it/scheduled-tasks)                   | Imposta una pianificazione                                                                            | [CLI](/docs/it/scheduled-tasks), [Desktop](/docs/it/desktop-scheduled-tasks), o [cloud](/docs/it/routines) | Scegli una frequenza                                                                                                              | Automazione ricorrente come revisioni giornaliere                    |

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Claude Code sul web](/docs/it/claude-code-on-the-web): esegui sessioni nel cloud invece che sulla tua macchina, configurate tramite [ambienti cloud](/docs/it/cloud-environments)
* [Messaggistica tra sessioni](/docs/it/cross-session-messaging): consenti a Claude di inviare messaggi alle tue sessioni su altre macchine o su [sessioni cloud](/docs/it/claude-code-on-the-web)
* [Canali](/docs/it/channels): inoltra Telegram, Discord o iMessage in una sessione in modo che Claude reagisca ai messaggi mentre sei assente
* [Dispatch](/docs/it/desktop#sessions-from-dispatch): invia un'attività dal tuo telefono e può generare una sessione Desktop per gestirla
* [Autenticazione](/docs/it/authentication): configura `/login` e gestisci le credenziali per claude.ai
* [Riferimento CLI](/docs/it/cli-reference): elenco completo di flag e comandi incluso `claude remote-control`
* [Sicurezza](/docs/it/security): come le sessioni Remote Control si adattano al modello di sicurezza di Claude Code
* [Utilizzo dei dati](/docs/it/data-usage): quali dati fluiscono attraverso l'API Anthropic durante le sessioni locali, Remote Control e cloud
