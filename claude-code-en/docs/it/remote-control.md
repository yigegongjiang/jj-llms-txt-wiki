> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Continua le sessioni locali da qualsiasi dispositivo con Remote Control

> Continua una sessione locale di Claude Code dal tuo telefono, tablet o da qualsiasi browser utilizzando Remote Control. Funziona con claude.ai/code e l'app Claude per dispositivi mobili.

<Note>
  Remote Control è in anteprima di ricerca ed è disponibile su tutti i piani. Su Team e Enterprise, è disabilitato per impostazione predefinita fino a quando un proprietario non abilita l'interruttore Remote Control nelle [impostazioni di amministrazione di Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control connette [claude.ai/code](https://claude.ai/code) o l'app Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) a una sessione di Claude Code in esecuzione sulla tua macchina. Avvia un'attività alla tua scrivania, quindi riprendi dal tuo telefono sul divano o da un browser su un altro computer.

Quando avvii una sessione Remote Control sulla tua macchina, Claude continua a funzionare localmente per tutto il tempo, quindi l'esecuzione del codice e l'accesso al filesystem rimangono sulla tua macchina. Con Remote Control puoi:

* **Utilizzare il tuo ambiente locale completo da remoto**: il tuo filesystem, i [server MCP](/docs/it/mcp), gli strumenti e la configurazione del progetto rimangono disponibili, e digitando `@` l'autocompletamento completa i percorsi dei file dal tuo progetto locale
* **Lavorare da entrambe le superfici contemporaneamente**: la conversazione e il progresso dei [subagent](/docs/it/sub-agents) e dei [flussi di lavoro dinamici](/docs/it/workflows) rimangono sincronizzati su tutti i dispositivi connessi, quindi puoi inviare messaggi dal tuo terminale, browser e telefono in modo intercambiabile. Prima della v2.1.207, le sessioni ospitate dall'[app Desktop](/docs/it/desktop) non inviavano il progresso dei subagent o dei flussi di lavoro ai dispositivi connessi.
* **Inviare immagini e file dal tuo telefono o browser**: quando aggiungi un allegato nell'app Claude o su claude.ai/code, Claude Code lo scarica sulla tua macchina e lo passa a Claude come riferimento a file `@`, con o senza didascalia. Prima della v2.1.202, Claude Code poteva eliminare un allegato inviato senza didascalia prima che raggiungesse la sessione.
* **Sopravvivere alle interruzioni**: se il tuo laptop va in sospensione o la tua rete si interrompe, la sessione si riconnette automaticamente quando la tua macchina torna online. Claude Code mette in coda gli aggiornamenti di stato dai subagent e dai flussi di lavoro mentre la connessione si sta ricostruendo e li consegna una volta che si recupera. Prima della v2.1.207, un aggiornamento inviato durante una riconnessione o un aggiornamento delle credenziali poteva andare perso, quindi il dispositivo connesso continuava a mostrare un'attività completata come in esecuzione.

A differenza di [Claude Code sul web](/docs/it/claude-code-on-the-web), che funziona su infrastrutture cloud, le sessioni Remote Control vengono eseguite direttamente sulla tua macchina e interagiscono con il tuo filesystem locale. Le interfacce web e mobile sono una finestra in quella sessione locale.

Questa pagina copre la configurazione, come avviare e connettersi alle sessioni, e come Remote Control si confronta con Claude Code sul web.

<h2 id="requirements">
  Requisiti
</h2>

Prima di utilizzare Remote Control, conferma che il tuo ambiente soddisfi queste condizioni:

* **Abbonamento**: disponibile su piani Pro, Max, Team e Enterprise. Le chiavi API non sono supportate. Su Team e Enterprise, un amministratore deve prima abilitare l'interruttore Remote Control nelle [impostazioni di amministrazione di Claude Code](https://claude.ai/admin-settings/claude-code).
* **Autenticazione**: esegui `claude` e utilizza `/login` per accedere tramite claude.ai se non l'hai già fatto.
* **Endpoint API**: non disponibile su Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. A partire dalla v2.1.196, Remote Control è anche disabilitato quando [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) punta a un host diverso da `api.anthropic.com`, come un [gateway LLM](/docs/it/llm-gateway) o proxy. Annulla l'impostazione della variabile per utilizzare Remote Control.
* **Fiducia dell'area di lavoro**: esegui `claude` nella directory del tuo progetto almeno una volta per accettare la finestra di dialogo di fiducia dell'area di lavoro.

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

    Il processo rimane in esecuzione nel tuo terminale in modalità server, in attesa di connessioni remote. Visualizza un URL di sessione che puoi utilizzare per [connetterti da un altro dispositivo](#connect-from-another-device), e puoi premere la barra spaziatrice per mostrare un codice QR per un accesso rapido dal tuo telefono. Mentre una sessione remota è attiva, il terminale mostra lo stato della connessione e l'attività dello strumento.

    Flag disponibili:

    | Flag                                            | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
    | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Imposta un titolo di sessione personalizzato visibile nell'elenco delle sessioni su claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
    | `--remote-control-session-name-prefix <prefix>` | Prefisso per i nomi di sessione generati automaticamente quando non è impostato un nome esplicito. Per impostazione predefinita è il nome host della tua macchina, producendo nomi come `myhost-graceful-unicorn`. Imposta `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` per lo stesso effetto.                                                                                                                                                                                                                                                                       |
    | `-c`, `--continue`                              | Riprendi la sessione Remote Control più recente avviata da questa directory invece di crearne una nuova. Non può essere combinato con `--session-id`, `--spawn`, `--capacity`, o `--create-session-in-dir`. Richiede Claude Code v2.1.200 o successivo; le versioni precedenti rifiutano il flag come argomento sconosciuto.                                                                                                                                                                                                                                        |
    | `--session-id <id>`                             | Riprendi una sessione Remote Control specifica dal suo ID. Non può essere combinato con `--continue`, `--spawn`, `--capacity`, o `--create-session-in-dir`. Richiede Claude Code v2.1.200 o successivo; le versioni precedenti rifiutano il flag come argomento sconosciuto.                                                                                                                                                                                                                                                                                        |
    | `--spawn <mode>`                                | Come il server crea le sessioni.<br />• `same-dir` (predefinito): tutte le sessioni condividono la directory di lavoro corrente, quindi possono entrare in conflitto se modificano gli stessi file.<br />• `worktree`: ogni sessione su richiesta ottiene il proprio [git worktree](/docs/it/worktrees). Richiede un repository git.<br />• `session`: modalità a sessione singola. Serve esattamente una sessione e rifiuta connessioni aggiuntive. Impostato solo all'avvio.<br />Premi `w` durante l'esecuzione per attivare/disattivare tra `same-dir` e `worktree`. |
    | `--capacity <N>`                                | Numero massimo di sessioni simultanee. Il valore predefinito è 32. Non può essere utilizzato con `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
    | `--[no-]create-session-in-dir`                  | Pre-crea una sessione nella directory corrente quando il server si avvia, così hai un posto dove digitare immediatamente. In modalità `worktree` questa sessione rimane nella directory corrente mentre le sessioni su richiesta ottengono worktree isolati. Abilitato per impostazione predefinita; passa `--no-create-session-in-dir` per avviare senza nessuna.                                                                                                                                                                                                  |
    | `--verbose`                                     | Mostra log dettagliati di connessione e sessione.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
    | `--sandbox` / `--no-sandbox`                    | Abilita o disabilita il [sandboxing](/docs/it/sandboxing) per l'isolamento del filesystem e della rete. Disabilitato per impostazione predefinita.                                                                                                                                                                                                                                                                                                                                                                                                                       |
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

    I flag `--verbose`, `--sandbox` e `--no-sandbox` non sono disponibili con questo comando.
  </Tab>

  <Tab title="VS Code">
    Nell'[estensione VS Code di Claude Code](/docs/it/vs-code), digita `/remote-control` o `/rc` nella casella del prompt, oppure apri il menu dei comandi con `/` e selezionalo.

    ```text theme={null}
    /remote-control
    ```

    Un banner appare sopra la casella del prompt mostrando lo stato della connessione. Una volta connesso, fai clic su **Open in browser** nel banner per andare direttamente alla sessione, oppure trovala nell'elenco delle sessioni su [claude.ai/code](https://claude.ai/code). L'URL della sessione è anche pubblicato nella conversazione.

    Per disconnetterti, fai clic sull'icona di chiusura sul banner o esegui `/remote-control` di nuovo.

    A differenza della CLI, il comando VS Code non accetta un argomento di nome e non visualizza un codice QR. Il titolo della sessione è derivato dalla cronologia della conversazione o dal primo prompt.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Verifica lo stato della connessione
</h3>

In una sessione di terminale interattiva, un indicatore `/rc active` si trova nel footer sotto la casella di input mentre la connessione è attiva, ed è nascosto se il terminale è troppo stretto per contenerlo. Il testo dell'indicatore è un collegamento alla sessione su claude.ai. Selezionalo con il tasto freccia giù e premi Invio, oppure esegui `/remote-control` di nuovo, per aprire un pannello di stato con l'URL della sessione e un codice QR che puoi utilizzare per [connetterti da un altro dispositivo](#connect-from-another-device).

Se la connessione non riesce, una notifica appare con il motivo dell'errore e l'indicatore scompare dal footer. Esegui `/remote-control` di nuovo per riprovare.

<h3 id="connect-from-another-device">
  Connettiti da un altro dispositivo
</h3>

Una volta che una sessione Remote Control è attiva, hai alcuni modi per connetterti da un altro dispositivo:

* **Apri l'URL della sessione** in qualsiasi browser per andare direttamente alla sessione su [claude.ai/code](https://claude.ai/code).
* **Scansiona il codice QR** mostrato accanto all'URL della sessione per aprirlo direttamente nell'app Claude. Con `claude remote-control`, premi la barra spaziatrice per attivare/disattivare la visualizzazione del codice QR.
* **Apri [claude.ai/code](https://claude.ai/code) o l'app Claude** e trova la sessione per nome nell'elenco delle sessioni. Nell'app mobile Claude, tocca **Code** nella navigazione per raggiungere l'elenco delle sessioni. Le sessioni Remote Control mostrano un'icona di computer con un punto di stato verde quando sono online.

Quando ti connetti, il dispositivo mostra tutti i subagent e i workflow che la sessione ha già in esecuzione in background. Prima della v2.1.208, un dispositivo che si connetteva a una sessione ospitata in un terminale interattivo non mostrava i subagent e i workflow che erano già in esecuzione fino a quando uno di essi non si avviava o si fermava.

Il titolo della sessione remota viene scelto in questo ordine:

1. Il nome che hai passato a `--name`, `--remote-control`, o `/remote-control`
2. Il titolo che hai impostato con `/rename`
3. L'ultimo messaggio significativo nella cronologia della conversazione esistente
4. Un nome generato automaticamente come `myhost-graceful-unicorn`, dove `myhost` è il nome host della tua macchina o il prefisso che hai impostato con `--remote-control-session-name-prefix`

Se non hai impostato un nome esplicito, il titolo si aggiorna per riflettere il tuo prompt una volta che ne invii uno. A partire da Claude Code v2.1.176, i titoli generati automaticamente corrispondono alla lingua della tua conversazione, o all'impostazione [`language`](/docs/it/settings#available-settings) se ne è configurata una. Rinominare una sessione da claude.ai o dall'app Claude aggiorna anche il titolo locale mostrato in `claude --resume`.

Se l'ambiente ha già una sessione attiva, ti verrà chiesto se continuarla o avviarne una nuova.

Se non hai ancora l'app Claude, utilizza il comando `/mobile` all'interno di Claude Code per visualizzare un codice QR di download per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).

<h3 id="enable-remote-control-for-all-sessions">
  Abilita Remote Control per tutte le sessioni
</h3>

Remote Control si attiva solo quando esegui esplicitamente `claude remote-control`, `claude --remote-control`, o `/remote-control`, a meno che l'auto-connessione non sia attivata. Per abilitarlo automaticamente per ogni sessione interattiva, esegui `/config` all'interno di Claude Code e imposta **Enable Remote Control for all sessions** su `true`. Impostalo su `false` per non auto-connetterti mai, oppure lascialo non impostato per seguire il valore predefinito della tua organizzazione. Nell'app Desktop, puoi anche attivare/disattivare questa opzione da **Settings → Claude Code → Enable remote control by default**. Nell'[estensione VS Code](/docs/it/vs-code#use-the-prompt-box), lo stesso interruttore appare come **Enable Remote Control for all sessions** nella sezione Impostazioni del menu dei comandi; richiede Claude Code v2.1.203 o successivo.

Con questa impostazione attiva, ogni processo Claude Code interattivo registra una sessione remota. Se esegui più istanze, ognuna ottiene il proprio ambiente e sessione. Per eseguire più sessioni simultanee da un singolo processo, utilizza invece la [modalità server](#start-a-remote-control-session).

<h2 id="connection-and-security">
  Connessione e sicurezza
</h2>

La tua sessione locale di Claude Code effettua solo richieste HTTPS in uscita e non apre mai porte in ingresso sulla tua macchina. Quando avvii Remote Control, si registra con l'API Anthropic e esegue il polling per il lavoro. Quando ti connetti da un altro dispositivo, il server instrada i messaggi tra il client web o mobile e la tua sessione locale su una connessione in streaming.

Tutto il traffico viaggia attraverso l'API Anthropic su TLS, lo stesso trasporto di sicurezza di qualsiasi sessione di Claude Code. La connessione utilizza più credenziali di breve durata, ognuna limitata a un singolo scopo e con scadenza indipendente.

Mentre Remote Control è connesso, la trascrizione della sessione, inclusi i tuoi messaggi, le risposte di Claude e l'attività degli strumenti, viene archiviata sui server Anthropic. La trascrizione archiviata mantiene la conversazione sincronizzata tra i tuoi dispositivi e consente alla sessione di riconnettersi dopo un'interruzione di rete. L'esecuzione e l'accesso al filesystem rimangono sulla tua macchina, e le trascrizioni archiviate vengono conservate secondo la politica di [utilizzo dei dati](/docs/it/data-usage).

Per disattivare completamente Remote Control, utilizza l'impostazione [`disableRemoteControl`](/docs/it/settings#available-settings). Le organizzazioni con requisiti di conformità come Zero Data Retention non possono abilitare Remote Control.

<h2 id="trusted-devices">
  Dispositivi affidabili
</h2>

<Note>
  Dispositivi affidabili è attualmente in beta. Le funzionalità e la funzionalità possono evolversi man mano che l'esperienza viene perfezionata.

  Dispositivi affidabili è disponibile su piani Team e Enterprise. È disabilitato per impostazione predefinita fino a quando un amministratore non lo abilita.
</Note>

Dispositivi affidabili è un'impostazione a livello di organizzazione che richiede ai membri di verificare il loro dispositivo prima di poter visualizzare o controllare le sessioni Remote Control da claude.ai, dalle app Claude per dispositivi mobili o da Claude Desktop. Lega l'accesso a Remote Control a un dispositivo noto e a un'autenticazione recente, non solo a un account connesso.

Quando l'impostazione è attiva, l'interazione con una sessione Remote Control richiede entrambi i seguenti:

* **Un dispositivo registrato**: ogni browser, telefono o app desktop che un membro utilizza per Remote Control registra le proprie credenziali. La registrazione viene offerta solo poco dopo un accesso completo, quindi un dispositivo si unisce all'elenco affidabile come parte di un'autenticazione reale piuttosto che silenziosamente in background.
* **Un accesso recente**: l'accesso del membro non deve essere più vecchio di 18 ore. Invece di accedere di nuovo ogni giorno, i membri confermano la presenza con Face ID, Touch ID, Windows Hello o una passkey. Questo passaggio di autenticazione biometrica aggiorna la sessione immediatamente.

I controlli biometrici vengono eseguiti sul dispositivo attraverso il sistema operativo o il browser, lo stesso meccanismo dell'accesso con passkey. Anthropic non riceve né archivia mai impronte digitali, dati facciali o altre informazioni biometriche. Solo la chiave pubblica del dispositivo e i metadati di base come il nome visualizzato, la piattaforma e l'ora di registrazione vengono archiviati.

L'impostazione si applica solo a Remote Control. La chat Claude regolare, Claude Code nel terminale e l'utilizzo dell'API non sono interessati.

<h3 id="enable-trusted-devices-for-your-organization">
  Abilita Dispositivi affidabili per la tua organizzazione
</h3>

Gli amministratori abilitano l'impostazione dalla console di amministrazione di Claude Code.

<Steps>
  <Step title="Apri le impostazioni di amministrazione di Claude Code">
    Vai a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). L'interruttore **Require trusted devices** appare sotto l'impostazione Remote Control.
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

<h2 id="remote-control-vs-claude-code-on-the-web">
  Remote Control vs Claude Code sul web
</h2>

Remote Control e [Claude Code sul web](/docs/it/claude-code-on-the-web) utilizzano entrambi l'interfaccia claude.ai/code. La differenza chiave è dove viene eseguita la sessione: Remote Control viene eseguito sulla tua macchina, quindi i tuoi server MCP locali, strumenti e configurazione del progetto rimangono disponibili. Claude Code sul web viene eseguito nell'infrastruttura cloud gestita da Anthropic.

Utilizza Remote Control quando sei nel mezzo di un lavoro locale e vuoi continuare da un altro dispositivo. Utilizza Claude Code sul web quando vuoi avviare un'attività senza alcuna configurazione locale, lavorare su un repository che non hai clonato, o eseguire più attività in parallelo.

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
* **Il processo locale deve continuare a funzionare**: Remote Control viene eseguito come processo locale. Se chiudi il terminale, esci da VS Code, o altrimenti interrompi il processo `claude`, la sessione termina.
* **Interruzione di rete prolungata**: se la tua macchina è accesa ma non riesce a raggiungere la rete per più di circa 10 minuti, la sessione scade e il processo esce. Esegui di nuovo `claude remote-control` per avviare una nuova sessione.
* **Ultraplan disconnette Remote Control**: avviare una sessione [ultraplan](/docs/it/ultraplan) disconnette qualsiasi sessione Remote Control attiva perché entrambe le funzioni occupano l'interfaccia claude.ai/code e solo una può essere connessa alla volta.
* **Alcuni comandi sono solo locali**: i comandi che funzionano solo nell'interfaccia del terminale, come `/plugin` o `/resume`, funzionano solo dalla CLI locale, indipendentemente dal fatto che tu passi un argomento o meno. I seguenti funzionano da mobile e web:
  * Comandi con output di testo: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits` (esegue il modulo di testo invece di aprire la finestra di dialogo in-CLI), `/recap`, `/reload-plugins`
  * `/model`, `/effort`, `/fast`, `/color` e `/rename`: passa il valore come argomento, ad esempio `/model sonnet` o `/effort high`. Da mobile e web, `/model` e `/effort` accettano l'argomento al posto del selettore del terminale o del cursore.
  * `/mcp`, dalla v2.1.166: dall'app mobile, restituisce un riepilogo testuale dello stato del server invece di aprire il selettore. Sul web, `/mcp` da solo apre una directory dei [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) invece di restituire il riepilogo. I [sottocomandi](/docs/it/commands#all-commands) `reconnect`, `enable` e `disable` funzionano da entrambi. A differenza della CLI locale, `/mcp reconnect` senza nome del server riconnette ogni server che ha avuto un errore o necessita autenticazione.
  * `/config`, dalla v2.1.181: dall'app mobile, passa `key=value` per impostare un'impostazione, o eseguilo senza argomenti per elencare le chiavi che puoi impostare. Sul web, `/config` apre la sezione Claude Code delle tue impostazioni, e ignora il testo dopo il comando.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control requires a claude.ai subscription"
</h3>

Non sei autenticato con un account claude.ai. Esegui `claude auth login` e scegli l'opzione claude.ai. Se `ANTHROPIC_API_KEY` è impostato nel tuo ambiente, annulla l'impostazione prima.

Prima della v2.1.206, l'esecuzione di `/remote-control` mentre non eri connesso segnalava `Unknown command: /remote-control` invece di questo messaggio.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control requires a full-scope login token"
</h3>

Sei autenticato con un token di lunga durata da `claude setup-token` o dalla variabile di ambiente `CLAUDE_CODE_OAUTH_TOKEN`. Questi token sono limitati solo all'inferenza e non possono stabilire sessioni Remote Control. Esegui `claude auth login` per autenticarti con un token di sessione a scopo completo.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Unable to determine your organization for Remote Control eligibility"
</h3>

Le informazioni dell'account memorizzate nella cache sono obsolete o incomplete. Esegui `claude auth login` per aggiornarlo.

<h3 id="remote-control-is-not-yet-enabled-for-your-account">
  "Remote Control is not yet enabled for your account"
</h3>

Il rollout di Remote Control non ha raggiunto il tuo account, oppure i tuoi diritti memorizzati nella cache sono obsoleti. Se hai recentemente cambiato piano, esegui `claude auth logout` quindi `claude auth login` per aggiornarli. Esegui `claude doctor` per vedere quale controllo di idoneità individuale ha fallito. I conflitti delle variabili di ambiente, i controlli non raggiungibili e la politica organizzativa producono ciascuno il proprio messaggio, quindi questo errore significa il gate di rollout stesso.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Couldn't verify Remote Control eligibility"
</h3>

Claude Code non ha potuto raggiungere il servizio di feature-flag per verificare se Remote Control è abilitato per il tuo account, in genere perché sei offline o un proxy sta bloccando la richiesta. Riprova una volta che hai accesso alla rete, oppure esegui `claude doctor` per i dettagli. Il messaggio correlato "Couldn't verify your organization's Remote Control policy" ha la stessa causa e la stessa soluzione. Entrambi i messaggi sono stati aggiunti nella v2.1.178.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control is only available when using Claude via api.anthropic.com"
</h3>

La sessione non sta comunicando direttamente con l'API Anthropic, quindi non c'è alcun backend claude.ai con cui associarsi. Questo accade su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. A partire dalla v2.1.196 accade anche quando [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) punta a un host diverso da `api.anthropic.com`, come un [gateway LLM](/docs/it/llm-gateway) o proxy, anche se accedi con claude.ai. Annulla l'impostazione di `ANTHROPIC_BASE_URL` e riavvia la sessione per utilizzare Remote Control.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control is disabled by your organization's policy"
</h3>

Questo errore ha quattro cause distinte. Esegui `/status` prima per vedere quale metodo di accesso e abbonamento stai utilizzando.

* **Sei autenticato con una chiave API o un account Console**: Remote Control richiede OAuth claude.ai. Esegui `/login` e scegli l'opzione claude.ai. Se `ANTHROPIC_API_KEY` è impostato nel tuo ambiente, annulla l'impostazione.
* **Un Owner non l'ha abilitato per la tua organizzazione**: Remote Control è disabilitato per impostazione predefinita su piani Team e Enterprise. Un Owner può abilitarlo su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) attivando l'interruttore **Remote Control**. Questo interruttore è un'impostazione organizzativa lato server.
* **L'interruttore di amministrazione è disattivato**: la tua organizzazione ha una configurazione di conservazione dei dati o conformità incompatibile con Remote Control. Questo non può essere modificato dal pannello di amministrazione. Contatta il supporto Anthropic per discutere le opzioni.
* **L'errore menziona `disableRemoteControl`**: il tuo amministratore IT ha disabilitato Remote Control su questo dispositivo tramite [impostazioni gestite](/docs/it/settings#settings-files), indipendentemente dall'interruttore a livello di organizzazione.

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

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Couldn't reconnect to your Remote Control session"
</h3>

Quando riprendi una conversazione con `claude --resume` o `claude --continue`, Claude Code si riconnette alla sessione Remote Control registrata in quella conversazione. Questo messaggio significa che la riconnessione non è riuscita per un motivo che potrebbe essere temporaneo, come un'interruzione di rete o un errore del server, quindi Claude Code non può confermare se la sessione remota esiste ancora. Quando il server conferma che la sessione precedente non esiste più, Claude Code crea una nuova sessione Remote Control senza mostrare questo messaggio.

La tua sessione locale continua a funzionare senza Remote Control. Esegui `/remote-control` per riprovare la connessione, o avvia Claude Code senza `--resume` per creare una nuova sessione Remote Control.

Prima della v2.1.200, un errore di riconnessione creava una nuova sessione Remote Control invece di mostrare questo messaggio, il che lasciava sessioni extra nell'elenco delle sessioni su claude.ai/code.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  "Your organization requires Trusted Devices for Remote Control, but this device is not enrolled"
</h3>

La tua organizzazione ha [Dispositivi affidabili](#trusted-devices) abilitato e questa macchina non si è ancora registrata. Esegui `/login` in Claude Code. La registrazione avviene come parte dell'accesso, e non c'è un comando di registrazione separato.

<h3 id="session-expired-for-trusted-device-check">
  "session expired for trusted-device check"
</h3>

Il tuo accesso è più vecchio di 18 ore. Esegui `/login` in Claude Code, o conferma con Face ID, Touch ID, Windows Hello o una passkey quando claude.ai o l'app mobile te lo chiede. Vedi [Dispositivi affidabili](#trusted-devices).

<h2 id="choose-the-right-approach">
  Scegli l'approccio giusto
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Claude Code sul web](/docs/it/claude-code-on-the-web): esegui sessioni in ambienti cloud gestiti da Anthropic invece che sulla tua macchina
* [Ultraplan](/docs/it/ultraplan): avvia una sessione di pianificazione cloud dal tuo terminale e rivedi il piano nel tuo browser
* [Canali](/docs/it/channels): inoltra Telegram, Discord o iMessage in una sessione in modo che Claude reagisca ai messaggi mentre sei assente
* [Dispatch](/docs/it/desktop#sessions-from-dispatch): invia un'attività dal tuo telefono e può generare una sessione Desktop per gestirla
* [Autenticazione](/docs/it/authentication): configura `/login` e gestisci le credenziali per claude.ai
* [Riferimento CLI](/docs/it/cli-reference): elenco completo di flag e comandi incluso `claude remote-control`
* [Sicurezza](/docs/it/security): come le sessioni Remote Control si adattano al modello di sicurezza di Claude Code
* [Utilizzo dei dati](/docs/it/data-usage): quali dati fluiscono attraverso l'API Anthropic durante le sessioni locali e remote
