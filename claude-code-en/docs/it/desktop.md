> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Applicazione desktop

> Sfrutta al massimo Claude Code Desktop: sessioni parallele con isolamento Git, layout dei pannelli drag-and-drop, terminale integrato e editor di file, chat laterali, utilizzo del computer, Dispatch sessioni dal tuo telefono, revisione visiva dei diff, anteprime delle app, monitoraggio dei PR, connettori e configurazione aziendale.

L'app Claude Desktop ha tre schede: **Chat** per le conversazioni, **Cowork** per [Dispatch e lavoro agentico più lungo](https://claude.com/product/cowork), e **Code** per lo sviluppo software. Questa pagina è il riferimento per la scheda Code.

<CardGroup cols={3}>
  <Card title="Download for macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universal build for Intel and Apple Silicon
  </Card>

  <Card title="Download for Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    For x64 processors
  </Card>

  <Card title="Get Claude for Linux (beta)" icon="linux" href="/docs/en/desktop-linux">
    apt or .deb for Ubuntu and Debian
  </Card>
</CardGroup>

For Windows ARM64, download the [ARM64 installer](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). On Linux, install with apt; see [Claude Desktop on Linux](/docs/en/desktop-linux).

Dopo l'installazione, avvia Claude, accedi e fai clic sulla scheda **Code**. La prima volta che lo apri su Windows, hai bisogno di [Git for Windows](https://git-scm.com/downloads/win) installato; riavvia l'app dopo averlo installato. Per una procedura dettagliata della tua prima sessione, consulta la [guida introduttiva](/docs/it/desktop-quickstart).

Nella scheda Code, ogni conversazione è una **sessione**: ha la sua propria cronologia chat, cartella di progetto e modifiche al codice, indipendente da qualsiasi altra sessione. La barra laterale elenca le tue sessioni e ti consente di eseguirne diverse in parallelo. All'interno di una sessione puoi:

* [Rivedere e commentare i diff](#review-changes-with-diff-view), quindi [monitorare il PR risultante attraverso CI](#monitor-pull-request-status)
* [Visualizzare in anteprima la tua app in esecuzione](#preview-your-app) nel riquadro Browser mentre Claude verifica le sue stesse modifiche, e [aprire siti esterni](#browse-external-sites) accanto ad esso
* [Organizzare i pannelli](#arrange-your-workspace) per la chat, diff, browser, terminale e editor di file uno accanto all'altro
* Porre una [domanda laterale](#ask-a-side-question-without-derailing-the-session) che utilizza il contesto della sessione senza deviare da essa
* [Connettere strumenti esterni](#connect-external-tools) come GitHub, Slack e Linear
* Consentire a Claude di [aprire app e controllare il tuo schermo](#let-claude-use-your-computer)
* Eseguire sulla tua macchina, nel [cloud](#run-long-running-tasks-remotely), o su [SSH](#ssh-sessions)

Per [lavoro ricorrente pianificato](/docs/it/desktop-scheduled-tasks), [scorciatoie da tastiera](#keyboard-shortcuts), o [invio di attività dal tuo telefono](#sessions-from-dispatch), consulta le pagine e le sezioni collegate. Se utilizzi già il CLI basato su terminale, consulta il [confronto CLI](#coming-from-the-cli) per vedere cosa si trasferisce.

<h2 id="start-a-session">
  Avvia una sessione
</h2>

Prima di inviare il tuo primo messaggio, configura quattro cose nell'area del prompt:

* **Ambiente**: scegli dove Claude viene eseguito. Seleziona **Local** per la tua macchina, **Remote** per sessioni cloud ospitate da Anthropic, una [**connessione SSH**](#ssh-sessions) per una macchina remota che gestisci, o su Windows una [**distribuzione WSL**](/docs/it/desktop-wsl). Vedi [configurazione dell'ambiente](#environment-configuration).
* **Cartella del progetto**: seleziona la cartella o il repository su cui Claude lavora. Per le sessioni remote, puoi aggiungere [più repository](#run-long-running-tasks-remotely).
* **Modello**: scegli un [modello](/docs/it/model-config#available-models) dal menu a discesa accanto al pulsante di invio. Puoi cambiare questo durante la sessione.
* **Modalità di autorizzazione**: scegli quanta autonomia ha Claude dal [selettore di modalità](#choose-a-permission-mode). Puoi cambiare questo durante la sessione.

Digita il tuo compito e premi **Invio** per iniziare. Ogni sessione traccia il suo proprio contesto e le modifiche in modo indipendente.

<h2 id="work-with-code">
  Lavora con il codice
</h2>

Dai a Claude il contesto giusto, controlla quanto fa da solo e rivedi cosa ha cambiato.

<h3 id="use-the-prompt-box">
  Usa la casella del prompt
</h3>

Digita quello che vuoi che Claude faccia e premi **Invio** per inviare. Claude legge i file del tuo progetto, apporta modifiche ed esegue comandi in base alla tua [modalità di autorizzazione](#choose-a-permission-mode). Puoi interrompere Claude in qualsiasi momento: fai clic sul pulsante di arresto per interrompere immediatamente, oppure digita una correzione e premi **Invio** per inviarla senza interrompere l'azione in corso. Claude legge la correzione non appena l'azione corrente si completa e si adatta prima del suo passo successivo.

Il pulsante **+** accanto alla casella del prompt ti dà accesso agli allegati di file, [skills](#use-skills), [connettori](#connect-external-tools) e [plugin](#install-plugins).

<h3 id="add-files-and-context-to-prompts">
  Aggiungi file e contesto ai prompt
</h3>

La casella del prompt supporta due modi per portare contesto esterno:

* **@mention file**: digita `@` seguito da un nome di file per aggiungere un file al contesto della conversazione. Claude può quindi leggere e fare riferimento a quel file. @mention non è disponibile nelle sessioni cloud o WSL.
* **Allega file**: allega immagini, PDF e altri file al tuo prompt usando il pulsante di allegato, o trascina e rilascia i file direttamente nel prompt. Questo è utile per condividere screenshot di bug, mockup di design o documenti di riferimento.

<h3 id="choose-a-permission-mode">
  Scegli una modalità di autorizzazione
</h3>

Le modalità di autorizzazione controllano quanta autonomia ha Claude durante una sessione: se chiede prima di modificare file, eseguire comandi o entrambi. Puoi cambiare modalità in qualsiasi momento usando il selettore di modalità accanto al pulsante di invio. Inizia con Manual per vedere esattamente cosa fa Claude, quindi passa a Accept edits o Plan man mano che acquisisci familiarità.

Per impostare una modalità predefinita per le nuove sessioni locali, aggiungi `permissions.defaultMode` al tuo [file di impostazioni](/docs/it/settings#settings-files). L'app desktop legge gli stessi file di impostazioni della CLI. Una modalità che scegli nel selettore viene ricordata per cartella e ha la precedenza su `defaultMode` per quella cartella, tranne Plan, che si applica solo alla sessione corrente.

| Modalità               | Chiave di impostazione | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ---------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Manual**             | `default`              | Claude chiede prima di modificare file o eseguire comandi. Vedi un diff e puoi accettare o rifiutare ogni modifica. Consigliato per i nuovi utenti.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| **Accept edits**       | `acceptEdits`          | Claude accetta automaticamente le modifiche ai file e i comandi comuni del filesystem come `mkdir`, `touch` e `mv`, ma chiede comunque prima di eseguire altri comandi di terminale. Usa questo quando ti fidi delle modifiche ai file e vuoi un'iterazione più veloce.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| **Plan**               | `plan`                 | Claude legge i file ed esegue comandi per esplorare, quindi propone un piano senza modificare il tuo codice sorgente. Buono per compiti complessi dove vuoi rivedere l'approccio prima.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| **Auto**               | `auto`                 | Claude esegue tutte le azioni con controlli di sicurezza in background che verificano l'allineamento con la tua richiesta. Riduce i prompt di autorizzazione mantenendo la supervisione. Appare quando il tuo account soddisfa i [requisiti di disponibilità](#auto-mode-availability) di seguito; non c'è un interruttore Impostazioni separato per esso.                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Bypass permissions** | `bypassPermissions`    | Claude viene eseguito senza prompt di autorizzazione, tranne quelli forzati da [regole di richiesta](/docs/it/permissions#manage-permissions) esplicite, strumenti connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool), o classificatori di sicurezza quando Claude [agisce su siti esterni](#browse-external-sites); equivalente a `--dangerously-skip-permissions` nella CLI. Su piani Pro e Max, abilitalo in Impostazioni → Claude Code sotto "Allow bypass permissions mode"; su piani Team e Enterprise non c'è un interruttore Impostazioni, e la politica organizzativa lo controlla invece. Usa solo in container sandbox o VM. |

Le versioni precedenti della scheda Code etichettavano queste modalità Ask permissions, Auto accept edits e Plan mode.

La modalità di autorizzazione `dontAsk` è disponibile solo nella [CLI](/docs/it/permission-modes#allow-only-pre-approved-tools-with-dontask-mode).

<span id="auto-mode-availability" />

Auto mode è disponibile a tutti gli utenti sull'API Anthropic e richiede Claude Opus 4.6 o successivo, o Sonnet 4.6 o successivo. Gli amministratori dell'organizzazione possono disattivare auto mode con la chiave `disableAutoMode` in [impostazioni gestite](#managed-settings).

Nelle distribuzioni Enterprise che instradano Desktop a Google Cloud's Agent Platform, auto mode è [disponibile per impostazione predefinita](/docs/it/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry), e solo Claude Sonnet 5, Opus 4.7 e Opus 4.8 sono supportati lì. Prima di Claude Code v2.1.207, le distribuzioni Enterprise su Google Cloud's Agent Platform dovevano impostare `CLAUDE_CODE_ENABLE_AUTO_MODE` per abilitare auto mode.

<Tip title="Best practice">
  Inizia compiti complessi in Plan in modo che Claude mappi un approccio prima di apportare modifiche. Una volta approvato il piano, passa a Accept edits o Manual per eseguirlo. Vedi [esplora prima, poi pianifica, poi codifica](/docs/it/best-practices#explore-first-then-plan-then-code) per ulteriori informazioni su questo flusso di lavoro.
</Tip>

Le sessioni cloud supportano Accept edits, Plan e Auto. Accept edits corrisponde alla modalità `default`: le sessioni cloud pre-approvano le modifiche ai file, quindi il selettore mostra Accept edits invece di Manual. Bypass permissions non è disponibile perché l'ambiente cloud è già sandbox.

Gli amministratori aziendali possono limitare quali modalità di autorizzazione sono disponibili. Vedi [configurazione aziendale](#enterprise-configuration) per i dettagli.

<h3 id="preview-your-app">
  Anteprima della tua app
</h3>

Claude può avviare un server di sviluppo e aprirlo nel riquadro Browser per verificare le sue modifiche. Questo funziona per app web frontend così come per server backend: Claude può testare endpoint API, visualizzare log del server e iterare su problemi che trova. Nella maggior parte dei casi, Claude avvia il server automaticamente dopo aver modificato i file del progetto. Puoi anche chiedere a Claude di visualizzare un'anteprima in qualsiasi momento. Per impostazione predefinita, Claude [verifica automaticamente](#auto-verify-changes) le modifiche dopo ogni modifica.

Il riquadro Browser può anche aprire file HTML statici, PDF, immagini e video dal tuo progetto. Fai clic su un percorso HTML, PDF, immagine o video nella chat per aprirlo lì.

Dal riquadro Browser, puoi:

* Interagire con la tua app in esecuzione direttamente nel riquadro Browser
* Guardare Claude verificare automaticamente le sue stesse modifiche: scatta screenshot, ispeziona il DOM, fa clic su elementi, compila moduli e corregge i problemi che trova
* Avviare o arrestare server dal menu a discesa del server nella barra degli strumenti della sessione
* Persistere cookie e archiviazione locale tra i riavvii del server selezionando **Persist sessions** nel menu a discesa, in modo da non dover effettuare di nuovo l'accesso durante lo sviluppo
* Modificare la configurazione del server o arrestare tutti i server contemporaneamente

Claude crea la configurazione iniziale del server in base al tuo progetto. Se la tua app utilizza un comando dev personalizzato, modifica `.claude/launch.json` per adattarlo alla tua configurazione. Vedi [Configura server di anteprima](#configure-preview-servers) per il riferimento completo.

Per cancellare i dati della sessione salvati, o per disattivare completamente il Browser, usa gli interruttori in Impostazioni → Claude Code.

<h3 id="browse-external-sites">
  Sfoglia siti esterni
</h3>

Il riquadro Browser è un browser a schede, quindi puoi aprire documentazione, tracker di problemi o qualsiasi altro sito accanto alla tua app in esecuzione. Per aprire il Browser, premi **Cmd+Maiusc+B** su macOS o **Ctrl+Maiusc+B** su Windows, o selezionalo dal menu **Views**. Quando fai clic su un collegamento esterno nella chat, un selettore offre **Open in app** per usare il riquadro Browser o **Default browser** per usare il tuo; **Cmd**-clic su macOS o **Ctrl**-clic su Windows apre un collegamento nel tuo browser di sistema direttamente. Puoi accedere ai siti nel riquadro, inclusi flussi di accesso popup come Google OAuth.

Claude può leggere e interagire con pagine esterne usando gli stessi strumenti che usa per [verificare la tua app](#preview-your-app), con due controlli di sicurezza aggiuntivi:

* I classificatori di sicurezza esaminano le azioni di scrittura di Claude su pagine esterne, come fare clic e digitare, in ogni modalità di autorizzazione. Questi sono gli stessi classificatori che [auto mode](#choose-a-permission-mode) usa, e quando contrassegnano un'azione, ricevi un prompt di autorizzazione indipendentemente dalla modalità.
* Nelle modalità di autorizzazione diverse da Auto e Bypass permissions, un controllo della lista di autorizzazione del dominio si applica anche prima che Claude navighi verso un nuovo sito.

<h4 id="approve-claude’s-actions-on-a-site">
  Approva le azioni di Claude su un sito
</h4>

La prima volta che Claude agisce su un sito esterno, appare una scheda di autorizzazione e Claude attende la tua scelta: **Allow once**, **Always allow** o **Deny**. **Allow once** approva l'azione senza salvare nulla. **Always allow** salva l'approvazione per quel sito sul tuo dispositivo, e puoi revocarla in Impostazioni. Ogni sito ha bisogno della sua propria approvazione, inclusi i sottodomini. I tuoi server di sviluppo locali e i file del progetto non hanno bisogno di approvazione, quindi [auto-verify](#auto-verify-changes) continua a funzionare senza prompt.

Anche su un sito approvato, Claude non acquisterà articoli, creerà account o aggirerà CAPTCHA senza il tuo input. La navigazione nel riquadro Browser utilizza lo stesso modello di sicurezza dell'[estensione Claude in Chrome](/docs/it/chrome). Vedi [Utilizzo di Claude in Chrome in sicurezza](https://support.claude.com/en/articles/12902428-using-claude-in-chrome-safely) per come Claude gestisce siti sensibili e azioni rischiose.

<h4 id="choose-between-the-browser-and-the-chrome-extension">
  Scegli tra il Browser e l'estensione Chrome
</h4>

Il riquadro Browser utilizza un profilo browser pulito, separato dal tuo browser personale, senza nessuno dei tuoi accessi salvati o cronologia. Usalo per costruire e testare la tua app e per siti che non hanno bisogno della tua identità. Quando vuoi che Claude agisca come te nelle tue sessioni di accesso, usa invece l'[estensione Claude in Chrome](/docs/it/chrome), che condivide lo stato di accesso del tuo browser.

<h4 id="restrict-external-browsing-for-your-organization">
  Limita la navigazione esterna per la tua organizzazione
</h4>

Il Browser segue gli stessi [controlli della lista di autorizzazione e blocco del sito](https://support.claude.com/en/articles/13065128-claude-in-chrome-admin-controls) dell'estensione Claude in Chrome. Se la tua organizzazione ha già configurato quelle liste per l'estensione, il Browser le rispetta automaticamente. Gli amministratori possono anche disattivare gli strumenti di Claude su pagine esterne con l'impostazione gestita [`browserExternalPageTools`](#managed-settings). Con gli strumenti disabilitati, gli utenti possono ancora navigare verso siti esterni; gli strumenti di Claude non possono leggerli o agire su di essi.

Per disattivare completamente la navigazione esterna, imposta l'impostazione gestita [`disableBrowserExternalNavigation`](#managed-settings) su `true`. Questo blocca tutta la navigazione esterna nel Browser, inclusi i siti nella lista di autorizzazione della tua organizzazione; i server dev localhost e le anteprime di file continuano a funzionare. Usa `browserExternalPageTools` per consentire agli utenti di continuare a sfogliare siti esterni senza gli strumenti di Claude, e `disableBrowserExternalNavigation` per bloccare i siti esterni sia per gli utenti che per Claude.

<h3 id="review-changes-with-diff-view">
  Rivedi le modifiche con la visualizzazione diff
</h3>

Dopo che Claude apporta modifiche al tuo codice, la visualizzazione diff ti consente di rivedere le modifiche file per file prima di creare una pull request.

Quando Claude modifica i file, appare un indicatore di statistiche diff che mostra il numero di righe aggiunte e rimosse, come `+12 -1`. Fai clic su questo indicatore per aprire il visualizzatore diff, che visualizza un elenco di file a sinistra e le modifiche per ogni file a destra.

Per commentare righe specifiche, fai clic su qualsiasi riga nel diff per aprire una casella di commento. Digita il tuo feedback e premi **Invio** per aggiungere il commento. Dopo aver aggiunto commenti a più righe, invia tutti i commenti contemporaneamente:

* **macOS**: premi **Cmd+Invio**
* **Windows**: premi **Ctrl+Invio**

Claude legge i tuoi commenti e apporta le modifiche richieste, che appaiono come un nuovo diff che puoi rivedere.

<h3 id="review-your-code">
  Rivedi il tuo codice
</h3>

Nella visualizzazione diff, fai clic su **Review code** nella barra degli strumenti in alto a destra per chiedere a Claude di valutare le modifiche prima di eseguire il commit. Claude esamina i diff attuali e lascia commenti direttamente nella visualizzazione diff. Puoi rispondere a qualsiasi commento o chiedere a Claude di rivedere.

La revisione si concentra su problemi ad alto segnale: errori di compilazione, errori logici definitivi, vulnerabilità di sicurezza e bug ovvi. Non contrassegna stile, formattazione, problemi preesistenti o qualsiasi cosa che un linter catturebbe.

<h3 id="monitor-pull-request-status">
  Monitora lo stato della pull request
</h3>

Dopo aver aperto una pull request, una barra di stato CI appare nella sessione. Claude Code utilizza GitHub CLI per eseguire il polling dei risultati dei controlli e visualizzare i guasti.

* **Auto-fix**: quando abilitato, Claude tenta automaticamente di correggere i controlli CI non riusciti leggendo l'output del guasto e iterando.
* **Auto-merge**: quando abilitato, Claude unisce il PR una volta che tutti i controlli passano. Il metodo di merge è squash. Auto-merge deve essere [abilitato nelle impostazioni del tuo repository GitHub](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository) affinché questo funzioni.

Usa gli interruttori **Auto-fix** e **Auto-merge** nella barra di stato CI per abilitare una delle due opzioni. Claude Code invia anche una notifica desktop quando CI termina. Per archiviare la sessione automaticamente una volta che il PR si unisce o si chiude, attiva [auto-archive](#work-in-parallel-with-sessions) in Impostazioni → Claude Code.

<Note>
  Il monitoraggio dei PR richiede che [GitHub CLI (`gh`)](https://cli.github.com/) sia installato e autenticato sulla tua macchina. Se `gh` non è installato, Desktop ti chiede di installarlo la prima volta che tenti di creare un PR.
</Note>

<h2 id="arrange-your-workspace">
  Organizza l'area di lavoro
</h2>

La scheda Code è costruita attorno a pannelli che puoi organizzare in qualsiasi layout: chat, diff, browser, terminale, file, plan, tasks e subagent. Trascina un pannello dal suo header per riposizionarlo, o trascina un bordo del pannello per ridimensionarlo. Premi **Cmd+\\** su macOS o **Ctrl+\\** su Windows per chiudere il pannello focalizzato. Apri pannelli aggiuntivi dal menu **Views** nella barra degli strumenti della sessione.

<Note>
  Il layout del pannello, il terminale, l'editor di file e le modalità di visualizzazione in questa sezione richiedono Claude Desktop v1.2581.0 o successivo. Apri **Claude → Check for Updates** su macOS o **Help → Check for Updates** su Windows per aggiornare.
</Note>

<h3 id="run-commands-in-the-terminal">
  Esegui comandi nel terminale
</h3>

Il terminale integrato ti consente di eseguire comandi insieme alla tua sessione senza passare a un'altra app. Aprilo dal menu **Views** o premi **Ctrl+\`** su macOS o Windows. Il terminale si apre nella directory di lavoro della tua sessione e condivide lo stesso ambiente di Claude, quindi comandi come `npm test` o `git status` vedono gli stessi file che Claude sta modificando. Per aprire una seconda scheda del terminale, fai clic su **+** nell'header del pannello del terminale o fai clic con il pulsante destro su una cartella nella chat per scegliere **Open in terminal**. Il terminale è disponibile solo nelle sessioni locali.

<h3 id="open-and-edit-files">
  Apri e modifica file
</h3>

Fai clic su un percorso di file nella chat o nel visualizzatore diff per aprirlo nel pannello file. I percorsi HTML, PDF, immagine e video si aprono nel [pannello Browser](#preview-your-app) invece. Fai modifiche spot e fai clic su **Save** per scriverle di nuovo. Se il file è cambiato su disco da quando l'hai aperto, il pannello ti avverte e ti consente di sovrascrivere o scartare. Fai clic su **Discard** per ripristinare le tue modifiche, o fai clic sul percorso nell'header del pannello per copiare il percorso assoluto.

Il pannello file è disponibile nelle sessioni locali e SSH. Per le sessioni cloud, chiedi a Claude di apportare la modifica.

<h3 id="open-files-in-other-apps">
  Apri file in altre app
</h3>

Fai clic con il pulsante destro su qualsiasi percorso di file nella chat, nel visualizzatore diff o nel pannello file per aprire un menu di contesto:

* **Attach as context**: aggiungi il file al tuo prossimo prompt
* **Open in**: apri il file in un editor installato come VS Code, Cursor o Zed
* **Show in Finder** su macOS, **Show in Explorer** su Windows: apri la cartella contenente
* **Copy path**: copia il percorso assoluto negli appunti

<h3 id="switch-view-modes">
  Cambia modalità di visualizzazione
</h3>

Le modalità di visualizzazione controllano quanti dettagli appaiono nella trascrizione della chat. Cambia modalità dal menu a discesa **Transcript view** accanto al pulsante di invio, o premi **Ctrl+O** su macOS o Windows per scorrere tra loro.

| Modalità    | Cosa mostra                                                                          |
| ----------- | ------------------------------------------------------------------------------------ |
| **Normal**  | Chiamate di strumenti compresse in riepiloghi, con risposte di testo completo        |
| **Verbose** | Ogni chiamata di strumento, lettura di file e passaggio intermedio che Claude compie |
| **Summary** | Solo le risposte finali di Claude e le modifiche che ha apportato                    |

Usa Verbose quando esegui il debug del motivo per cui Claude ha intrapreso una particolare azione. Usa Summary quando stai eseguendo più sessioni e vuoi scansionare i risultati rapidamente.

<h3 id="keyboard-shortcuts">
  Scorciatoie da tastiera
</h3>

Premi **Cmd+/** su macOS o **Ctrl+/** su Windows per vedere tutte le scorciatoie disponibili nella scheda Code. Su Windows, usa **Ctrl** al posto di **Cmd** per le scorciatoie di seguito. Il ciclo della sessione, l'interruttore del terminale e l'interruttore della modalità di visualizzazione usano **Ctrl** su ogni piattaforma.

| Scorciatoia                           | Azione                                  |
| ------------------------------------- | --------------------------------------- |
| `Cmd` `/`                             | Mostra scorciatoie da tastiera          |
| `Cmd` `N`                             | Nuova sessione                          |
| `Cmd` `W`                             | Chiudi sessione                         |
| `Ctrl` `Tab` / `Ctrl` `Shift` `Tab`   | Sessione successiva o precedente        |
| `Cmd` `Shift` `]` / `Cmd` `Shift` `[` | Sessione successiva o precedente        |
| `Esc`                                 | Arresta la risposta di Claude           |
| `Cmd` `Shift` `D`                     | Attiva/disattiva pannello diff          |
| `Cmd` `Shift` `B`                     | Attiva/disattiva pannello Browser       |
| `Cmd` `Shift` `S`                     | Seleziona un elemento nel Browser       |
| `Ctrl` `` ` ``                        | Attiva/disattiva pannello terminale     |
| `Cmd` `\`                             | Chiudi pannello focalizzato             |
| `Cmd` `;`                             | Apri chat laterale                      |
| `Ctrl` `O`                            | Scorrere le modalità di visualizzazione |
| `Cmd` `Shift` `M`                     | Apri menu modalità di autorizzazione    |
| `Cmd` `Shift` `I`                     | Apri menu modello                       |
| `Cmd` `Shift` `E`                     | Apri menu sforzo                        |
| `1`–`9`                               | Seleziona elemento in un menu aperto    |

Queste scorciatoie si applicano solo alla scheda Code. Le scorciatoie della [modalità interattiva](/docs/it/interactive-mode#keyboard-shortcuts) basata su terminale, come `Shift+Tab` per scorrere le modalità, non si applicano in Desktop.

<h3 id="check-usage">
  Controlla l'utilizzo
</h3>

Fai clic sull'anello di utilizzo accanto al selettore di modello per vedere l'utilizzo della finestra di contesto corrente e l'utilizzo del piano per il periodo. L'utilizzo del contesto è per sessione; l'utilizzo del piano è condiviso su tutte le tue superfici Claude Code.

<h2 id="let-claude-use-your-computer">
  Lascia che Claude usi il tuo computer
</h2>

L'utilizzo del computer consente a Claude di aprire le tue app, controllare lo schermo e lavorare direttamente sulla tua macchina come faresti tu. Chiedi a Claude di testare un'app nativa in un simulatore mobile, interagire con uno strumento desktop che non ha CLI, o automatizzare qualcosa che funziona solo tramite una GUI.

<Note>
  L'utilizzo del computer è un'anteprima di ricerca su macOS e Windows che richiede un piano Pro o Max. Non è disponibile su piani Team o Enterprise. L'app Claude Desktop deve essere in esecuzione.
</Note>

L'utilizzo del computer è disabilitato per impostazione predefinita. [Abilitalo in Impostazioni](#enable-computer-use) prima che Claude possa controllare lo schermo. Su macOS, devi anche concedere i permessi di Accessibilità e Registrazione dello schermo.

<Warning>
  A differenza dello [strumento Bash sandbox](/docs/it/sandboxing), l'utilizzo del computer viene eseguito sul tuo desktop effettivo con accesso a tutto ciò che approvi. Claude controlla ogni azione e contrassegna potenziali iniezioni di prompt dal contenuto sullo schermo, ma il limite di fiducia è diverso. Vedi la [guida alla sicurezza dell'utilizzo del computer](https://support.claude.com/en/articles/14128542) per le best practice.
</Warning>

<h3 id="when-computer-use-applies">
  Quando si applica l'utilizzo del computer
</h3>

Claude ha diversi modi per interagire con un'app o un servizio, e l'utilizzo del computer è il più ampio e lento. Prova prima lo strumento più preciso:

* Se hai un [connettore](#connect-external-tools) per un servizio, Claude usa il connettore.
* Se l'attività è un comando shell, Claude usa Bash.
* Se l'attività è lavoro nel browser e hai [Claude in Chrome](/docs/it/chrome) configurato, Claude usa quello.
* Se nessuno di questi si applica, Claude usa l'utilizzo del computer.

I [livelli di accesso per app](#app-permissions) rafforzano questo: i browser sono limitati a sola visualizzazione, e i terminali e gli IDE a solo clic, indirizzando Claude verso lo strumento dedicato anche quando l'utilizzo del computer è attivo. Il controllo dello schermo è riservato a cose che nient'altro può raggiungere, come app native, pannelli di controllo hardware, simulatori mobili o strumenti proprietari senza un'API.

<h3 id="enable-computer-use">
  Abilita l'utilizzo del computer
</h3>

L'utilizzo del computer è disabilitato per impostazione predefinita. Se chiedi a Claude di fare qualcosa che ne ha bisogno mentre è disabilitato, Claude ti dice che potrebbe fare l'attività se abiliti l'utilizzo del computer in Impostazioni.

<Steps>
  <Step title="Aggiorna l'app desktop">
    Assicurati di avere l'ultima versione di Claude Desktop. Su macOS e Windows, scarica o aggiorna su [claude.com/download](https://claude.com/download); su Linux, aggiorna tramite il tuo gestore di pacchetti ([istruzioni](/docs/it/desktop-linux)). Quindi riavvia l'app.
  </Step>

  <Step title="Attiva l'interruttore">
    Nell'app desktop, vai a **Impostazioni > Generale** (sotto **App Desktop**). Trova l'interruttore **Utilizzo del computer** e attivalo. Su Windows, l'interruttore ha effetto immediatamente e la configurazione è completa. Su macOS, continua al passaggio successivo.

    Se non vedi l'interruttore, conferma che sei su macOS o Windows con un piano Pro o Max, quindi aggiorna e riavvia l'app.
  </Step>

  <Step title="Concedi i permessi macOS">
    Su macOS, concedi due permessi di sistema prima che l'interruttore abbia effetto:

    * **Accessibilità**: consente a Claude di fare clic, digitare e scorrere
    * **Registrazione dello schermo**: consente a Claude di vedere cosa c'è sullo schermo

    La pagina Impostazioni mostra lo stato attuale di ogni permesso. Se uno è negato, fai clic sul badge per aprire il riquadro Impostazioni di sistema pertinente.
  </Step>
</Steps>

<h3 id="app-permissions">
  Permessi delle app
</h3>

La prima volta che Claude ha bisogno di usare un'app, appare un prompt nella tua sessione. Fai clic su **Allow for this session** o **Deny**. Le approvazioni durano per la sessione corrente, o 30 minuti nelle [sessioni generate da Dispatch](#sessions-from-dispatch).

Il prompt mostra anche quale livello di controllo Claude ottiene per quell'app. Questi livelli sono fissi per categoria di app e non possono essere modificati:

| Livello      | Cosa può fare Claude                                                  | Si applica a                    |
| :----------- | :-------------------------------------------------------------------- | :------------------------------ |
| View only    | Vedere l'app negli screenshot                                         | Browser, piattaforme di trading |
| Click only   | Fare clic e scorrere, ma non digitare o usare scorciatoie da tastiera | Terminali, IDE                  |
| Full control | Fare clic, digitare, trascinare e usare scorciatoie da tastiera       | Tutto il resto                  |

Le app con ampia portata, come terminali, Finder o File Explorer, e Impostazioni di sistema o Impostazioni, mostrano un avviso aggiuntivo nel prompt in modo che tu sappia cosa approvare loro concede.

Puoi configurare due impostazioni in **Impostazioni > Generale** (sotto **App Desktop**):

* **Denied apps**: aggiungi app qui per rifiutarle senza chiedere. Claude potrebbe comunque influenzare un'app negata indirettamente tramite azioni in un'app consentita, ma non può interagire direttamente con l'app negata.
* **Unhide apps when Claude finishes**: mentre Claude sta lavorando, le tue altre finestre sono nascoste in modo che interagisca solo con l'app approvata. Quando Claude finisce, le finestre nascoste vengono ripristinate a meno che non disattivi questa impostazione.

<h2 id="manage-sessions">
  Gestisci le sessioni
</h2>

Ogni sessione è una conversazione indipendente con il suo proprio contesto e modifiche. Puoi eseguire più sessioni in parallelo, diramazioni di chat laterali, inviare il lavoro al cloud, o lasciare che Dispatch avvii sessioni per te dal tuo telefono.

<h3 id="work-in-parallel-with-sessions">
  Lavora in parallelo con le sessioni
</h3>

Fai clic su **+ New session** nella barra laterale, o premi **Cmd+N** su macOS o **Ctrl+N** su Windows, per lavorare su più compiti in parallelo. Premi **Ctrl+Tab** e **Ctrl+Shift+Tab** per scorrere le sessioni nella barra laterale. Per i repository Git, ogni sessione ottiene la sua copia isolata del tuo progetto usando [Git worktrees](/docs/it/worktrees), in modo che le modifiche in una sessione non influiscano su altre sessioni fino a quando non le esegui il commit.

Per visualizzare due sessioni contemporaneamente, tieni premuto **Cmd** su macOS o **Ctrl** su Windows e fai clic su una sessione nella barra laterale. La sessione si apre in un secondo riquadro accanto a quello che hai già aperto. Mentre la divisione è attiva, facendo clic su un'altra sessione della barra laterale si sostituisce il riquadro che ha il focus. Premi **Cmd+\\** su macOS o **Ctrl+\\** su Windows per chiudere il riquadro con focus e tornare a una singola sessione.

I worktree sono archiviati in `<project-root>/.claude/worktrees/` per impostazione predefinita. Puoi cambiare questo in una directory personalizzata in Impostazioni → Claude Code sotto "Worktree location". Puoi anche impostare un prefisso di ramo che viene anteposto a ogni nome di ramo worktree, il che è utile per mantenere organizzati i rami creati da Claude. Per rimuovere un worktree quando hai finito, passa il mouse sulla sessione nella barra laterale e fai clic sull'icona di archivio. Per avere sessioni che si archiviano automaticamente quando il loro pull request si unisce o si chiude, attiva **Auto-archive after PR merge or close** in Impostazioni → Claude Code. Auto-archive si applica solo alle sessioni locali che hanno finito di funzionare.

Per includere file gitignored come `.env` nei nuovi worktree, crea un [file `.worktreeinclude`](/docs/it/worktrees#copy-gitignored-files-into-worktrees) nella radice del tuo progetto.

<Note>
  L'isolamento della sessione richiede [Git](https://git-scm.com/downloads). La maggior parte dei Mac include Git per impostazione predefinita. Esegui `git --version` in Terminal per verificare. Su Windows, Git è richiesto affinché la scheda Code funzioni: [scarica Git per Windows](https://git-scm.com/downloads/win), installalo e riavvia l'app. Se riscontri errori Git, chiedi a Claude nella scheda [Cowork](https://claude.com/product/cowork) di aiutarti a risolvere i problemi della tua configurazione.
</Note>

Usa i controlli in cima alla barra laterale per filtrare le sessioni per stato, progetto o ambiente, e per raggruppare le sessioni per progetto. Per rinominare una sessione, fai clic sul titolo della sessione nella barra degli strumenti in cima alla sessione attiva. Per controllare l'utilizzo del contesto, vedi [Controlla l'utilizzo](#check-usage). Quando il contesto si riempie, Claude riassume automaticamente la conversazione e continua a lavorare. Puoi anche digitare `/compact` per attivare la compattazione prima e liberare spazio di contesto. Vedi [la finestra di contesto](/docs/it/how-claude-code-works#the-context-window) per i dettagli su come funziona la compattazione.

L'app desktop invia una notifica del sistema operativo quando una sessione Code termina un'attività e non stai visualizzando quella sessione.

<h3 id="ask-a-side-question-without-derailing-the-session">
  Chiedi una domanda laterale senza deviare la sessione
</h3>

Una chat laterale ti consente di chiedere a Claude una domanda che utilizza il contesto della tua sessione ma non aggiunge nulla di nuovo alla conversazione principale. Usala quando vuoi capire un pezzo di codice, verificare un'assunzione o esplorare un'idea senza sterzare la sessione fuori rotta.

Premi **Cmd+;** su macOS o **Ctrl+;** su Windows per aprire una chat laterale, o digita `/btw` nella casella del prompt. La chat laterale può leggere tutto nel thread principale fino a quel punto. Quando hai finito, chiudi la chat laterale e continua la sessione principale da dove l'hai lasciata. Le chat laterali sono disponibili nelle sessioni locali, SSH e WSL.

<h3 id="watch-background-tasks">
  Guarda le attività in background
</h3>

Il pannello attività mostra il lavoro in background in esecuzione all'interno della sessione corrente: subagent, comandi shell in background e [flussi di lavoro dinamici](/docs/it/workflows). Aprilo dal menu **Views** o trascinalo nel tuo layout.

Fai clic su qualsiasi voce per vedere il suo output nel pannello subagent o fermarlo. Per vedere cosa stanno facendo altre sessioni, usa la [barra laterale](#work-in-parallel-with-sessions).

<h3 id="run-long-running-tasks-remotely">
  Esegui attività a lunga esecuzione in remoto
</h3>

Per grandi refactor, suite di test, migrazioni o altre attività a lunga esecuzione, seleziona **Remote** invece di **Local** quando avvii una sessione. Le sessioni remote vengono eseguite sull'infrastruttura cloud di Anthropic e continuano anche se chiudi l'app o spegni il computer. Torna indietro in qualsiasi momento per vedere i progressi o indirizzare Claude in una direzione diversa. Puoi anche monitorare le sessioni remote da [claude.ai/code](https://claude.ai/code) o dall'app Claude iOS.

Le sessioni remote supportano anche più repository. Dopo aver selezionato un ambiente cloud, fai clic sul pulsante **+** accanto alla pillola del repo per aggiungere repository aggiuntivi alla sessione. Ogni repo ottiene il suo selettore di ramo. Questo è utile per compiti che si estendono su più codebase, come l'aggiornamento di una libreria condivisa e dei suoi consumatori.

Vedi [Claude Code sul web](/docs/it/claude-code-on-the-web) per ulteriori informazioni su come funzionano le sessioni remote.

<h3 id="continue-in-another-surface">
  Continua su un'altra superficie
</h3>

Il menu **Continue in**, accessibile dall'icona VS Code in basso a destra della barra degli strumenti della sessione, ti consente di spostare la tua sessione su un'altra superficie:

* **Claude Code sul web**: invia la tua sessione locale per continuare l'esecuzione in remoto. Desktop esegue il push del tuo ramo, genera un riepilogo della conversazione e crea una nuova sessione remota con il contesto completo. Puoi quindi scegliere di archiviare la sessione locale o mantenerla. Questo richiede un albero di lavoro pulito e non è disponibile per le sessioni SSH.
* **Il tuo IDE**: apre il tuo progetto in un IDE supportato nella directory di lavoro corrente.

<h3 id="sessions-from-dispatch">
  Sessioni da Dispatch
</h3>

[Dispatch](https://support.claude.com/en/articles/13947068) è una conversazione persistente con Claude che vive nella scheda [Cowork](https://claude.com/product/cowork). Invii a Dispatch un'attività, e decide come gestirla.

Un'attività può finire come una sessione Code in due modi: chiedi direttamente una, come "apri una sessione Claude Code e correggi il bug di accesso", o Dispatch decide che l'attività è lavoro di sviluppo e ne genera una automaticamente. Le attività che tipicamente vengono indirizzate a Code includono correzione di bug, aggiornamento delle dipendenze, esecuzione di test o apertura di pull request. La ricerca, la modifica di documenti e il lavoro con fogli di calcolo rimangono in Cowork.

In entrambi i casi, la sessione Code appare nella barra laterale della scheda Code con un badge **Dispatch**. Ricevi una notifica push sul tuo telefono quando finisce o ha bisogno della tua approvazione.

Se hai [l'utilizzo del computer](#let-claude-use-your-computer) abilitato, le sessioni Code generate da Dispatch possono usarlo anche. Le approvazioni delle app in quelle sessioni scadono dopo 30 minuti e ripromptano, piuttosto che durare l'intera sessione come le sessioni Code regolari.

Per la configurazione, l'accoppiamento e le impostazioni di Dispatch, vedi l'[articolo di aiuto di Dispatch](https://support.claude.com/en/articles/13947068). Dispatch richiede un piano Pro o Max e non è disponibile su piani Team o Enterprise.

Dispatch è uno dei diversi modi per lavorare con Claude quando sei lontano dal tuo terminale. Vedi [Piattaforme e integrazioni](/docs/it/platforms#work-when-you-are-away-from-your-terminal) per confrontarlo con Remote Control, Channels, Slack e attività pianificate.

<h2 id="extend-claude-code">
  Estendi Claude Code
</h2>

Connetti servizi esterni, aggiungi flussi di lavoro riutilizzabili, personalizza il comportamento di Claude e configura server di anteprima. Per gestire connettori, skills e plugin in un unico posto, fai clic su **Customize** nella barra laterale.

<h3 id="connect-external-tools">
  Connetti strumenti esterni
</h3>

Per le sessioni locali e [SSH](#ssh-sessions), fai clic sul pulsante **+** accanto alla casella del prompt e seleziona **Connectors** per aggiungere integrazioni come Google Calendar, Slack, GitHub, Linear, Notion e altri. Puoi aggiungere connettori prima o durante una sessione. Il pulsante **+** non è disponibile nelle sessioni cloud o WSL, ma le [routine](/docs/it/routines) configurano i connettori al momento della creazione della routine.

Per gestire o disconnettere i connettori, vai a Impostazioni → Connectors nell'app desktop, o seleziona **Manage connectors** dal menu Connectors nella casella del prompt.

Una volta connesso, Claude può leggere il tuo calendario, inviare messaggi, creare problemi e interagire direttamente con i tuoi strumenti. Puoi chiedere a Claude quali connettori sono configurati nella tua sessione.

I connettori sono [MCP servers](/docs/it/mcp) con un flusso di configurazione grafico. Usali per l'integrazione rapida con i servizi supportati. Per le integrazioni non elencate in Connectors, aggiungi MCP servers manualmente tramite [file di impostazioni](/docs/it/mcp#installing-mcp-servers). Puoi anche [creare connettori personalizzati](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp).

<h3 id="use-skills">
  Usa skills
</h3>

[Skills](/docs/it/skills) estendono quello che Claude può fare. Claude le carica automaticamente quando rilevante, o puoi invocarne una direttamente: digita `/` nella casella del prompt o fai clic sul pulsante **+** e seleziona **Slash commands** per sfogliare cosa è disponibile. Questo include [comandi incorporati](/docs/it/commands), le tue [skill personalizzate](/docs/it/skills#create-your-first-skill), skill del progetto dal tuo codebase e skill da qualsiasi [plugin installato](/docs/it/plugins). Selezionane uno e appare evidenziato nel campo di input. Digita il tuo compito dopo di esso e invia come al solito.

Puoi inviare un comando mentre Claude sta lavorando, come qualsiasi altro messaggio, e la sessione torna inattiva una volta che il turno finisce. Prima della v2.1.206, un comando inviato a metà turno potrebbe lasciare la sessione in esecuzione e i messaggi che hai inviato in seguito non venivano consegnati.

<h3 id="install-plugins">
  Installa plugin
</h3>

[Plugins](/docs/it/plugins) sono pacchetti riutilizzabili che aggiungono skills, agent, hooks, MCP servers e configurazioni LSP a Claude Code. Puoi installare plugin dall'app desktop senza usare il terminale.

Per le sessioni locali e [SSH](#ssh-sessions), fai clic sul pulsante **+** accanto alla casella del prompt e seleziona **Plugins** per vedere i tuoi plugin installati e i loro skills. Per aggiungere un plugin, seleziona **Add plugin** dal sottomenu per aprire il browser dei plugin, che mostra i plugin disponibili dai tuoi [marketplace](/docs/it/plugin-marketplaces) configurati incluso il marketplace ufficiale di Anthropic. Seleziona **Manage plugins** per abilitare, disabilitare o disinstallare plugin.

I plugin possono essere limitati al tuo account utente, a un progetto specifico o solo locali. Se la tua organizzazione gestisce i plugin centralmente, quei plugin sono disponibili nelle sessioni desktop nello stesso modo in cui lo sono nella CLI. I plugin non sono disponibili per le sessioni cloud o WSL. Per il riferimento completo dei plugin inclusa la creazione dei tuoi plugin, vedi [plugin](/docs/it/plugins).

<h3 id="configure-preview-servers">
  Configura server di anteprima
</h3>

Claude rileva automaticamente la tua configurazione del server di sviluppo e archivia la configurazione in `.claude/launch.json` alla radice della cartella che hai selezionato quando hai avviato la sessione. Preview utilizza questa cartella come directory di lavoro, quindi se hai selezionato una cartella padre, le sottocartelle con i loro stessi server di sviluppo non verranno rilevate automaticamente. Per lavorare con il server di una sottocartella, avvia una sessione in quella cartella direttamente o aggiungi una configurazione manualmente.

Per personalizzare come il tuo server si avvia, ad esempio per usare `yarn dev` invece di `npm run dev` o per cambiare la porta, modifica il file manualmente o fai clic su **Edit configuration** nel menu a discesa del server per aprirlo nel tuo editor di codice. Il file supporta JSON con commenti.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

Puoi definire più configurazioni per eseguire server diversi dallo stesso progetto, come un frontend e un'API. Vedi gli [esempi](#examples) di seguito.

<h4 id="auto-verify-changes">
  Auto-verify changes
</h4>

Quando `autoVerify` è abilitato, Claude verifica automaticamente le modifiche al codice dopo aver modificato i file. Scatta screenshot, controlla gli errori e conferma che le modifiche funzionano prima di completare la sua risposta.

Auto-verify è abilitato per impostazione predefinita. Disabilitalo per progetto aggiungendo `"autoVerify": false` a `.claude/launch.json`, o attiva/disattivalo dal menu a discesa del server.

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
  "configurations": [...]
}
```

Quando disabilitato, gli strumenti di anteprima sono ancora disponibili e puoi chiedere a Claude di verificare in qualsiasi momento. Auto-verify lo rende automatico dopo ogni modifica.

<h4 id="configuration-fields">
  Configuration fields
</h4>

Ogni voce nell'array `configurations` accetta i seguenti campi:

| Campo               | Tipo      | Descrizione                                                                                                                                                                                                                                                                                            |
| ------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`              | string    | Un identificatore univoco per questo server                                                                                                                                                                                                                                                            |
| `runtimeExecutable` | string    | Il comando da eseguire, come `npm`, `yarn` o `node`                                                                                                                                                                                                                                                    |
| `runtimeArgs`       | string\[] | Argomenti passati a `runtimeExecutable`, come `["run", "dev"]`                                                                                                                                                                                                                                         |
| `port`              | number    | La porta su cui il tuo server ascolta. Predefinito a 3000                                                                                                                                                                                                                                              |
| `cwd`               | string    | Directory di lavoro relativa alla radice del tuo progetto. Predefinito alla radice del progetto. Usa `${workspaceFolder}` per fare riferimento alla radice del progetto esplicitamente                                                                                                                 |
| `env`               | object    | Variabili di ambiente aggiuntive come coppie chiave-valore, come `{ "NODE_ENV": "development" }`. Non mettere segreti qui poiché questo file viene eseguito il commit nel tuo repo. Per passare segreti al tuo server di sviluppo, impostali nell'[editor di ambiente locale](#local-sessions) invece. |
| `autoPort`          | boolean   | Come gestire i conflitti di porta. Vedi di seguito                                                                                                                                                                                                                                                     |
| `program`           | string    | Uno script da eseguire con `node`. Vedi [quando usare `program` vs `runtimeExecutable`](#when-to-use-program-vs-runtimeexecutable)                                                                                                                                                                     |
| `args`              | string\[] | Argomenti passati a `program`. Usato solo quando `program` è impostato                                                                                                                                                                                                                                 |

<a id="when-to-use-program-vs-runtimeexecutable" />

<h5 id="when-to-use-program-vs-runtimeexecutable">
  When to use `program` vs `runtimeExecutable`
</h5>

Usa `runtimeExecutable` con `runtimeArgs` per avviare un server di sviluppo tramite un gestore di pacchetti. Ad esempio, `"runtimeExecutable": "npm"` con `"runtimeArgs": ["run", "dev"]` esegue `npm run dev`.

Usa `program` quando hai uno script autonomo che vuoi eseguire con `node` direttamente. Ad esempio, `"program": "server.js"` esegue `node server.js`. Passa flag aggiuntivi con `args`.

<h4 id="port-conflicts">
  Port conflicts
</h4>

Il campo `autoPort` controlla cosa succede quando la tua porta preferita è già in uso:

* **`true`**: Claude trova e utilizza una porta libera automaticamente. Adatto per la maggior parte dei server di sviluppo.
* **`false`**: Claude fallisce con un errore. Usa questo quando il tuo server deve usare una porta specifica, come per i callback OAuth o gli allowlist CORS.
* **Non impostato (predefinito)**: Claude chiede se il server ha bisogno di quella porta esatta, quindi salva la tua risposta.

Quando Claude sceglie una porta diversa, passa la porta assegnata al tuo server tramite la variabile di ambiente `PORT`.

<h4 id="examples">
  Examples
</h4>

Queste configurazioni mostrano configurazioni comuni per diversi tipi di progetto:

<Tabs>
  <Tab title="Next.js">
    Questa configurazione esegue un'app Next.js usando Yarn sulla porta 3000:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "web",
          "runtimeExecutable": "yarn",
          "runtimeArgs": ["dev"],
          "port": 3000
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Multiple servers">
    Per un monorepo con un frontend e un server API, definisci più configurazioni. Il frontend usa `autoPort: true` in modo che scelga una porta libera se 3000 è occupata, mentre il server API richiede la porta 8080 esattamente:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "frontend",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "dev"],
          "cwd": "apps/web",
          "port": 3000,
          "autoPort": true
        },
        {
          "name": "api",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "start"],
          "cwd": "server",
          "port": 8080,
          "env": { "NODE_ENV": "development" },
          "autoPort": false
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Node.js script">
    Per eseguire uno script Node.js direttamente invece di usare un comando del gestore di pacchetti, usa il campo `program`:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "server",
          "program": "server.js",
          "args": ["--verbose"],
          "port": 4000
        }
      ]
    }
    ```
  </Tab>
</Tabs>

<h2 id="environment-configuration">
  Configurazione dell'ambiente
</h2>

L'ambiente che scegli quando [avvii una sessione](#start-a-session) determina dove Claude viene eseguito e come ti connetti:

* **Local**: viene eseguito sulla tua macchina con accesso diretto ai tuoi file
* **Remote**: viene eseguito sull'infrastruttura cloud di Anthropic. Le sessioni continuano anche se chiudi l'app.
* **SSH**: viene eseguito su una macchina remota a cui ti connetti tramite SSH, come i tuoi stessi server, VM cloud o dev container
* **WSL** (Windows): viene eseguito all'interno di una [distribuzione WSL 2](/docs/it/desktop-wsl) sulla tua macchina, utilizzando la sua toolchain Linux e i percorsi nativi

<h3 id="local-sessions">
  Local sessions
</h3>

L'app desktop non sempre eredita il tuo ambiente shell completo. Su macOS, quando avvii l'app dal Dock o Finder, legge il tuo profilo shell, come `~/.zshrc` o `~/.bashrc`, per estrarre `PATH` e un insieme fisso di variabili Claude Code, ma altre variabili che esporti lì non vengono raccolte. Su Windows, l'app eredita variabili di ambiente utente e di sistema ma non legge i profili PowerShell.

Per impostare variabili di ambiente per le sessioni locali e i server di sviluppo su qualsiasi piattaforma, apri il menu a discesa dell'ambiente nella casella del prompt, passa il mouse su **Local** e fai clic sull'icona dell'ingranaggio per aprire l'editor di ambiente locale. Le variabili che salvi qui vengono archiviate crittografate sulla tua macchina e si applicano a ogni sessione locale e server di anteprima che avvii. Puoi anche aggiungere variabili alla chiave `env` nel tuo file `~/.claude/settings.json`, anche se queste raggiungono solo le sessioni Claude e non i server di sviluppo. Vedi [variabili di ambiente](/docs/it/env-vars) per l'elenco completo delle variabili supportate.

[Extended thinking](/docs/it/model-config#extended-thinking) è abilitato per impostazione predefinita, il che migliora le prestazioni su compiti di ragionamento complesso ma utilizza token aggiuntivi. Per disabilitare il thinking, imposta `MAX_THINKING_TOKENS` a `0` nell'editor di ambiente locale; questo non ha effetto su Fable 5, che utilizza sempre extended thinking. Su [provider di terze parti](/docs/it/third-party-integrations), `0` omette il parametro `thinking` invece, e i modelli di adaptive-reasoning potrebbero comunque pensare. Su modelli con [adaptive reasoning](/docs/it/model-config#adjust-effort-level), qualsiasi altro valore `MAX_THINKING_TOKENS` viene ignorato perché il adaptive reasoning controlla la profondità del thinking. Su Opus 4.6 e Sonnet 4.6, imposta `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` a `1` per usare un budget di thinking fisso; Fable 5, Sonnet 5, e Opus 4.7 e versioni successive usano sempre il adaptive reasoning e non hanno una modalità di budget fisso.

<h3 id="cloud-sessions">
  Cloud sessions
</h3>

Le sessioni cloud continuano in background anche se chiudi l'app. L'utilizzo conta verso i limiti del tuo [piano di abbonamento](/docs/it/costs) senza costi di calcolo separati.

Puoi creare ambienti cloud personalizzati con diversi livelli di accesso alla rete e variabili di ambiente. Seleziona il menu a discesa dell'ambiente quando avvii una sessione cloud e scegli **Add environment**. Vedi [l'ambiente cloud](/docs/it/claude-code-on-the-web#the-cloud-environment) per i dettagli sulla configurazione dell'accesso alla rete e delle variabili di ambiente.

<h3 id="ssh-sessions">
  SSH sessions
</h3>

Le sessioni SSH ti consentono di eseguire Claude Code su una macchina remota mentre usi l'app desktop come tua interfaccia. Questo è utile per lavorare con codebase che vivono su VM cloud, dev container o server con hardware o dipendenze specifiche.

Per aggiungere una connessione SSH, fai clic sul menu a discesa dell'ambiente prima di avviare una sessione e seleziona **+ Add SSH connection**. La finestra di dialogo chiede:

* **Name**: un'etichetta amichevole per questa connessione
* **SSH Host**: `user@hostname` o un host definito in `~/.ssh/config`
* **SSH Port**: predefinito a 22 se lasciato vuoto, o utilizza la porta dal tuo SSH config
* **Identity File**: percorso della tua chiave privata, come `~/.ssh/id_rsa`. Lascia vuoto per usare la chiave predefinita o il tuo SSH config.

Una volta aggiunta, la connessione appare nel menu a discesa dell'ambiente. Selezionala per avviare una sessione su quella macchina. Claude viene eseguito sulla macchina remota con accesso ai suoi file e strumenti.

La macchina remota deve eseguire Linux o macOS. L'app desktop installa Claude Code sulla macchina remota automaticamente la prima volta che ti connetti. Una volta connesso, le sessioni SSH supportano modalità di autorizzazione, connettori, plugin e MCP server.

<h4 id="pre-configure-ssh-connections-for-your-team">
  Pre-configure SSH connections for your team
</h4>

Gli amministratori possono distribuire connessioni SSH ai membri del team aggiungendo `sshConfigs` a un file di [impostazioni gestite](/docs/it/settings#settings-precedence). Le connessioni definite in questo modo appaiono nel menu a discesa dell'ambiente di ogni utente automaticamente e vengono mostrate come gestite, quindi gli utenti possono selezionarle ma non possono modificarle o eliminarle nell'app.

L'esempio seguente pre-configura una singola connessione che si apre in `~/projects` sull'host remoto:

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519",
      "startDirectory": "~/projects"
    }
  ]
}
```

Ogni voce richiede `id`, `name` e `sshHost`. I campi `sshPort`, `sshIdentityFile` e `startDirectory` sono facoltativi. Gli utenti possono anche aggiungere `sshConfigs` al loro `~/.claude/settings.json`, che è dove vengono archiviate le connessioni aggiunte tramite la finestra di dialogo.

<h4 id="restrict-which-ssh-hosts-users-can-connect-to">
  Restrict which SSH hosts users can connect to
</h4>

Gli amministratori possono limitare le sessioni SSH di Desktop a un insieme approvato di host aggiungendo `sshHostAllowlist` a un file di [impostazioni gestite](/docs/it/settings#settings-precedence). Quando impostato, gli utenti possono connettersi solo a host il cui nome host risolto corrisponde a uno dei modelli. Impostalo su un array vuoto per disabilitare completamente le sessioni SSH.

L'esempio seguente consente connessioni a qualsiasi host sotto `devboxes.example.com` e a un singolo host bastion denominato:

```json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

I modelli sono case-insensitive. `*` corrisponde a qualsiasi host, e `*.example.com` corrisponde a `example.com` e a qualsiasi sottodominio. Tutto il resto è una corrispondenza esatta. Il controllo viene eseguito sul nome host dopo la risoluzione di `~/.ssh/config` tramite `ssh -G`, quindi gli alias `Host` e le voci `ProxyCommand`/`ProxyJump` sono consentiti purché il `HostName` risolto corrisponda.

`sshHostAllowlist` viene letto solo dalle impostazioni gestite; i valori nelle impostazioni utente o progetto vengono ignorati. Solo l'app Claude Desktop onora questa impostazione; la CLI Claude Code e le estensioni IDE non la leggono, e non limita i comandi `ssh` eseguiti tramite lo strumento Bash. Governa a quali host l'app Desktop si connette, non l'uscita di rete, quindi abbinalo ai controlli di rete della tua organizzazione o zero-trust se hai bisogno di un confine rigido.

<h2 id="enterprise-configuration">
  Configurazione aziendale
</h2>

Le organizzazioni su piani Team o Enterprise possono gestire il comportamento dell'app desktop tramite controlli della console di amministrazione, file di impostazioni gestiti e criteri di gestione dei dispositivi.

<h3 id="admin-console-controls">
  Controlli della console di amministrazione
</h3>

Queste impostazioni sono configurate tramite la [console delle impostazioni di amministrazione](https://claude.ai/admin-settings/claude-code):

* **Code in the desktop**: controlla se gli utenti della tua organizzazione possono accedere a Claude Code nell'app desktop
* **Code in the web**: abilita o disabilita le [sessioni web](/docs/it/claude-code-on-the-web) per la tua organizzazione
* **Remote Control**: abilita o disabilita [Remote Control](/docs/it/remote-control) per la tua organizzazione
* **Disable Bypass permissions mode**: impedisci agli utenti della tua organizzazione di abilitare la modalità bypass permissions

<h3 id="managed-settings">
  Impostazioni gestite
</h3>

Le impostazioni gestite sovrascrivono le impostazioni del progetto e dell'utente e si applicano alle sessioni Claude Code in Desktop. Puoi impostare queste chiavi nel file [impostazioni gestite](/docs/it/settings#settings-precedence) della tua organizzazione o inviarle in remoto tramite la console di amministrazione.

| Chiave                                     | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissions.disableBypassPermissionsMode` | imposta su `"disable"` per impedire agli utenti di abilitare la modalità bypass permissions.                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `disableAutoMode`                          | imposta su `"disable"` per impedire agli utenti di abilitare la modalità [Auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode). Rimuove Auto dal selettore di modalità. Accettato anche sotto `permissions`.                                                                                                                                                                                                                                                                                                                                          |
| `autoMode`                                 | personalizza cosa il classificatore della modalità auto si fida e blocca in tutta la tua organizzazione. Vedi [Configura la modalità auto](/docs/it/auto-mode-config).                                                                                                                                                                                                                                                                                                                                                                                             |
| `browserExternalPageTools`                 | imposta su `"disabled"` per impedire a Claude di utilizzare strumenti per leggere o agire su pagine esterne nel [riquadro Browser](#browse-external-sites). Gli utenti possono comunque navigare verso siti esterni da soli, e le anteprime del server di sviluppo locale non sono interessate.                                                                                                                                                                                                                                                               |
| `disableBrowserExternalNavigation`         | imposta su `true` per disattivare completamente la navigazione esterna nel [riquadro Browser](#browse-external-sites). Né gli utenti né Claude possono navigare verso siti esterni, e le anteprime del server localhost dev non sono interessate. Il valore deve essere il booleano JSON `true`; la stringa `"true"` viene ignorata.                                                                                                                                                                                                                          |
| `sshConfigs`                               | pre-configura le [connessioni SSH](#pre-configure-ssh-connections-for-your-team) che appaiono nel menu a discesa dell'ambiente. Gli utenti non possono modificare o eliminare le connessioni gestite.                                                                                                                                                                                                                                                                                                                                                         |
| `sshHostAllowlist`                         | limita le [sessioni SSH](#restrict-which-ssh-hosts-users-can-connect-to) agli host il cui nome host risolto corrisponde a uno di questi modelli. Un array vuoto disabilita le sessioni SSH. Letto solo dalle impostazioni gestite.                                                                                                                                                                                                                                                                                                                            |
| `managedMcpServers`                        | invia le configurazioni del server MCP a tutti gli utenti in una distribuzione di terze parti. Ogni voce specifica un trasporto di `"http"`, `"sse"` o `"stdio"`, i dettagli della connessione e facoltativamente una mappa `toolPolicy` che limita quali strumenti in quel server gli utenti possono invocare. Disponibile solo nelle distribuzioni Desktop di terze parti (3P). Fornisci questa chiave tramite il file di impostazioni gestite o MDM, poiché le distribuzioni di terze parti non ricevono le impostazioni della console di amministrazione. |

Quali impostazioni gestite raggiungono una sessione Desktop dipende da dove quella sessione viene eseguita. Le restrizioni del modello come [`availableModels`](/docs/it/model-config#restrict-model-selection) vengono applicate nelle sessioni Claude Code di Desktop nello stesso modo che nel CLI del terminale; vedi [copertura della superficie](/docs/it/model-config#surface-coverage).

* **Sessioni locali su questa macchina**: un file di impostazioni gestite distribuito su disco si applica. Le impostazioni gestite inviate in remoto tramite la console di amministrazione raggiungono anche queste sessioni sull'API di Anthropic quando la sessione si autentica con un accesso dell'organizzazione o una chiave API configurata direttamente, seguendo la stessa [precedenza delle impostazioni](/docs/it/settings#settings-precedence) del CLI del terminale.
* **[Sessioni cloud](#cloud-sessions)**: vengono eseguite su VM gestite da Anthropic e ricevono solo [impostazioni gestite dal server](/docs/it/server-managed-settings).
* **[Sessioni SSH](#ssh-sessions)**: la sessione legge il file di impostazioni gestite dall'host remoto. Desktop stesso legge `sshConfigs` e `sshHostAllowlist` dalle impostazioni gestite della macchina locale quando crea la connessione.

`permissions.disableBypassPermissionsMode` e `disableAutoMode` funzionano anche nelle impostazioni dell'utente e del progetto, ma metterli nelle impostazioni gestite impedisce agli utenti di sovrascriverli.

Claude Code legge `autoMode` dalle impostazioni dell'utente, dal flag `--settings` e dalle impostazioni gestite, ma non da `.claude/settings.json` o `.claude/settings.local.json`: entrambi i file si trovano nella directory del repository, quindi un repository clonato o un passaggio di build non può iniettare le sue stesse regole del classificatore. Prima della v2.1.207, Claude Code leggeva anche `.claude/settings.local.json`.

Per l'elenco completo delle impostazioni solo gestite incluse `allowManagedPermissionRulesOnly` e `allowManagedHooksOnly`, vedi [impostazioni solo gestite](/docs/it/permissions#managed-only-settings).

<h3 id="device-management-policies">
  Criteri di gestione dei dispositivi
</h3>

I team IT possono gestire l'app desktop tramite MDM su macOS o criteri di gruppo su Windows. I criteri disponibili includono l'abilitazione o la disabilitazione della funzione Claude Code, il controllo degli aggiornamenti automatici e l'impostazione di un URL di distribuzione personalizzato.

* **macOS**: configura tramite il dominio di preferenza `com.anthropic.claudefordesktop` usando strumenti come Jamf o Kandji
* **Windows**: configura tramite il registro in `SOFTWARE\Policies\Claude`

<h3 id="network-access-requirements">
  Requisiti di accesso alla rete
</h3>

Desktop carica il suo codice applicativo e il contenuto dell'utente dagli host CDN di Anthropic.

```text theme={null}
anthropic.com
*.anthropic.com
claude.ai
*.claude.ai
claude.com
*.claude.com
claude.app
*.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

Il traffico è HTTPS sulla porta 443 a meno che non configuri una porta personalizzata per [OTLP](/docs/it/monitoring-usage), un gateway LLM o un server MCP.

Per i server proxy, autorità di certificazione personalizzate, mTLS e i domini di cui ha bisogno il CLI standalone, vedi [configurazione della rete](/docs/it/network-config).

Per ridurre il numero di wildcard del firewall, consenti invece questi host Anthropic. Alcuni sottodomini vengono generati dinamicamente e devono rimanere wildcard.

```text theme={null}
anthropic.com
api.anthropic.com
a-api.anthropic.com
a-cdn.anthropic.com
s-cdn.anthropic.com
assets-proxy.anthropic.com
claude.ai
a.claude.ai
a-cdn.claude.ai
assets.claude.ai
downloads.claude.ai
*.livepreview.claude.ai
claude.com
platform.claude.com
*.livepreview.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

<h3 id="authentication-and-sso">
  Autenticazione e SSO
</h3>

Le organizzazioni aziendali possono richiedere SSO per tutti gli utenti. Vedi [autenticazione](/docs/it/authentication) per i dettagli a livello di piano e [Configurazione di SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso) per la configurazione SAML; la configurazione OIDC è coperta nella [Guida dell'amministratore aziendale di Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide).

<h3 id="data-handling">
  Gestione dei dati
</h3>

Claude Code elabora il tuo codice localmente nelle sessioni locali o sull'infrastruttura cloud di Anthropic nelle sessioni cloud. Le conversazioni e il contesto del codice vengono inviati all'API di Anthropic per l'elaborazione. Vedi [gestione dei dati](/docs/it/data-usage) per i dettagli sulla conservazione dei dati, la privacy e la conformità.

<h3 id="deployment">
  Distribuzione
</h3>

Desktop può essere distribuito tramite strumenti di distribuzione aziendale:

* **macOS**: distribuisci tramite MDM come Jamf o Kandji usando il programma di installazione `.dmg`
* **Windows**: distribuisci tramite il pacchetto MSIX. Vedi [Distribuisci Claude Desktop per Windows](https://support.claude.com/en/articles/12622703-deploy-claude-desktop-for-windows) per le opzioni di distribuzione aziendale inclusa l'installazione silenziosa

Per i domini da inserire nella whitelist del firewall, vedi [requisiti di accesso alla rete](#network-access-requirements) sopra. Per le impostazioni proxy, le autorità di certificazione personalizzate e i gateway LLM, vedi [configurazione della rete](/docs/it/network-config).

Per il riferimento completo della configurazione aziendale, vedi la [guida alla configurazione aziendale](https://support.claude.com/en/articles/12622667-enterprise-configuration).

<h2 id="coming-from-the-cli">
  Provieni dalla CLI?
</h2>

Se usi già la CLI di Claude Code, Desktop esegue lo stesso motore sottostante con un'interfaccia grafica. Puoi eseguire entrambi contemporaneamente sulla stessa macchina, anche sullo stesso progetto. Ognuno mantiene una storia di sessione separata, ma condividono configurazione e memoria del progetto tramite file CLAUDE.md.

Per spostare una sessione CLI in Desktop, esegui `/desktop` nel terminale. Claude salva la tua sessione e l'apre nell'app desktop, quindi esce dalla CLI. Questo comando è disponibile su macOS e Windows quando sei connesso con un abbonamento Claude. Non è disponibile con autenticazione tramite chiave API o su Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry.

<Tip>
  Quando usare Desktop vs CLI: usa Desktop quando vuoi gestire sessioni parallele in una finestra, organizzare pannelli uno accanto all'altro, o rivedere le modifiche visivamente. Usa la CLI quando hai bisogno di scripting, automazione, o preferisci un flusso di lavoro di terminale.
</Tip>

<h3 id="cli-flag-equivalents">
  Equivalenti dei flag CLI
</h3>

Questa tabella mostra l'equivalente dell'app desktop per i flag CLI comuni. I flag non elencati non hanno equivalente desktop perché sono progettati per scripting o automazione.

| CLI                                   | Equivalente desktop                                                                                                                                                                             |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--model sonnet`                      | Menu a discesa del modello accanto al pulsante di invio                                                                                                                                         |
| `--resume`, `--continue`              | Fai clic su una sessione nella barra laterale                                                                                                                                                   |
| `--permission-mode`                   | Selettore di modalità accanto al pulsante di invio                                                                                                                                              |
| `--dangerously-skip-permissions`      | Modalità Bypass permissions. Su piani Pro e Max, abilitala in Impostazioni → Claude Code → "Allow bypass permissions mode"; su piani Team ed Enterprise, la politica organizzativa la controlla |
| `--add-dir`                           | Aggiungi più repo con il pulsante **+** nelle sessioni cloud                                                                                                                                    |
| `--allowedTools`, `--disallowedTools` | Nessun equivalente per sessione. Le regole di autorizzazione nei [file di impostazioni](/docs/it/settings) si applicano ancora.                                                                      |
| `--verbose`                           | Modalità di visualizzazione [Verbose view mode](#switch-view-modes) nel menu a discesa della vista Transcript                                                                                   |
| `--print`, `--output-format`          | Non disponibile. Desktop è solo interattivo.                                                                                                                                                    |
| `ANTHROPIC_MODEL` env var             | Menu a discesa del modello accanto al pulsante di invio                                                                                                                                         |
| `MAX_THINKING_TOKENS` env var         | Imposta nell'editor di ambiente locale. Vedi [configurazione dell'ambiente](#environment-configuration).                                                                                        |

<h3 id="shared-configuration">
  Configurazione condivisa
</h3>

Desktop e CLI leggono gli stessi file di configurazione, quindi la tua configurazione viene trasferita:

* I file **[CLAUDE.md](/docs/it/memory)** e `CLAUDE.local.md` nel tuo progetto vengono utilizzati da entrambi
* I **[MCP servers](/docs/it/mcp)** configurati in `~/.claude.json` o `.mcp.json` funzionano in entrambi
* **[Hooks](/docs/it/hooks)** e **[skills](/docs/it/skills)** definiti nelle impostazioni si applicano a entrambi
* **[Impostazioni](/docs/it/settings)** in `~/.claude.json` e `~/.claude/settings.json` sono condivise. Le regole di autorizzazione, gli strumenti consentiti e altre impostazioni in `settings.json` si applicano alle sessioni Desktop.
* **Modelli**: gli stessi [modelli](/docs/it/model-config#available-models) sono disponibili in entrambi. In Desktop, seleziona il modello dal menu a discesa accanto al pulsante di invio. Puoi cambiare il modello durante una sessione dal stesso menu a discesa.

<Note>
  **MCP servers dall'app desktop chat Claude**: l'app Desktop carica i MCP servers da `claude_desktop_config.json` nelle sessioni della scheda Code, insieme ai server da `~/.claude.json` e `.mcp.json`. Un server definito in `claude_desktop_config.json` è disponibile sia nella superficie desktop chat che nella scheda Code.

  La CLI standalone non legge `claude_desktop_config.json`. Su macOS e WSL, esegui `claude mcp add-from-claude-desktop` per copiare questi server in `~/.claude.json`. Vedi [Importa MCP servers da Claude Desktop](/docs/it/mcp#import-mcp-servers-from-claude-desktop) per il flusso di importazione e le opzioni di ambito.
</Note>

<h3 id="feature-comparison">
  Confronto delle funzionalità
</h3>

Questa tabella confronta le capacità principali tra CLI e Desktop. Per un elenco completo dei flag CLI, vedi il [riferimento CLI](/docs/it/cli-reference).

| Funzionalità                                            | CLI                                                              | Desktop                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------------------- | ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Modalità di autorizzazione                              | Tutte le modalità inclusa `dontAsk`                              | Manuale, Accetta modifiche, Plan e Auto. Bypass permissions appare nel selettore di modalità una volta abilitato: tramite l'interruttore Impostazioni su piani Pro e Max, o tramite la politica organizzativa su piani Team ed Enterprise                                                                                                                                               |
| `--dangerously-skip-permissions`                        | Flag CLI                                                         | Modalità Bypass permissions. Su piani Pro e Max, abilitala in Impostazioni → Claude Code → "Allow bypass permissions mode"; su piani Team ed Enterprise, la politica organizzativa la controlla                                                                                                                                                                                         |
| [Provider di terze parti](/docs/it/third-party-integrations) | Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry | API di Anthropic per impostazione predefinita. Per il routing tramite gateway, vedi [connetti l'app desktop a un gateway](/docs/it/llm-gateway-connect#desktop-app). Per eseguire la scheda Code su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o un gateway LLM self-hosted, vedi [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview). |
| [MCP servers](/docs/it/mcp)                                  | Configura nei file di impostazioni                               | UI Connectors per sessioni locali e SSH, o file di impostazioni                                                                                                                                                                                                                                                                                                                         |
| [Plugins](/docs/it/plugins)                                  | Comando `/plugin`                                                | UI gestore plugin                                                                                                                                                                                                                                                                                                                                                                       |
| @mention file                                           | Basato su testo                                                  | Con autocomplete; sessioni locali e SSH solo                                                                                                                                                                                                                                                                                                                                            |
| Allegati di file                                        | Non disponibile                                                  | Immagini, PDF                                                                                                                                                                                                                                                                                                                                                                           |
| Isolamento della sessione                               | Flag [`--worktree`](/docs/it/cli-reference)                           | Worktree automatici                                                                                                                                                                                                                                                                                                                                                                     |
| Sessioni multiple                                       | Terminali separati                                               | Schede della barra laterale                                                                                                                                                                                                                                                                                                                                                             |
| Attività ricorrenti                                     | Cron job, pipeline CI                                            | [Attività pianificate](/docs/it/desktop-scheduled-tasks)                                                                                                                                                                                                                                                                                                                                     |
| Utilizzo del computer                                   | [Abilita tramite `/mcp`](/docs/it/computer-use) su macOS              | [Controllo di app e schermo](#let-claude-use-your-computer) su macOS e Windows                                                                                                                                                                                                                                                                                                          |
| Integrazione Dispatch                                   | Non disponibile                                                  | [Sessioni Dispatch](#sessions-from-dispatch) nella barra laterale                                                                                                                                                                                                                                                                                                                       |
| Scripting e automazione                                 | [`--print`](/docs/it/cli-reference), [Agent SDK](/docs/it/headless)        | Non disponibile                                                                                                                                                                                                                                                                                                                                                                         |

<h3 id="what’s-not-available-in-desktop">
  Cosa non è disponibile in Desktop
</h3>

Le seguenti funzionalità sono disponibili solo nella CLI o nell'estensione VS Code, tranne dove diversamente indicato:

* **Provider di terze parti**: Desktop si connette all'API di Anthropic per impostazione predefinita. Per instradare Desktop attraverso un gateway, vedi [connetti l'app desktop a un gateway](/docs/it/llm-gateway-connect#desktop-app). Le distribuzioni Enterprise possono configurare Google Cloud's Agent Platform e provider gateway tramite [impostazioni gestite](https://claude.com/docs/third-party/claude-desktop/configuration). Per Amazon Bedrock o Microsoft Foundry nella CLI, vedi la [guida rapida](/docs/it/quickstart). Come eccezione alla sezione precedente, [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) esegue la scheda Code su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o un gateway LLM self-hosted.
* **Linux (beta)**: Computer Use non è ancora disponibile nell'app desktop Linux. Vedi [Claude Desktop su Linux](/docs/it/desktop-linux).
* **Suggerimenti di codice inline**: Desktop non fornisce suggerimenti in stile autocomplete. Funziona tramite prompt conversazionali e modifiche di codice esplicite.
* **Team di agent**: le sessioni parallele di Claude Code che si messaggiano tra loro sono disponibili nella [CLI](/docs/it/agent-teams), non in Desktop. Per il lavoro multi-agent all'interno di una sessione, usa i [flussi di lavoro dinamici](/docs/it/workflows), che vengono eseguiti in Desktop.
* **Comandi terminal-dialog**: i comandi integrati che aprono un pannello interattivo nel terminale si comportano diversamente nella scheda Code. Modifica direttamente i [file di impostazioni](/docs/it/settings) per gestire le regole di autorizzazione e la configurazione, oppure esegui i comandi dalla CLI standalone.
  * I comandi senza forma di argomento, come `/permissions`, rispondono con `isn't available in this environment`.
  * `/config` apre Impostazioni → Claude Code. Il testo dopo il comando viene ignorato, quindi `/config theme=dark` non imposta il tema.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Le sezioni di seguito coprono i problemi specifici dell'app desktop. Per gli errori API di runtime che appaiono nella chat come `API Error: 500`, `529 Overloaded`, `429` o `Prompt is too long`, vedi il [riferimento degli errori](/docs/it/errors). Questi errori e le loro correzioni sono gli stessi su CLI, desktop e web.

<h3 id="check-your-version">
  Check your version
</h3>

Per vedere quale versione dell'app desktop stai eseguendo:

* **macOS**: fai clic su **Claude** nella barra dei menu, quindi **About Claude**
* **Windows**: fai clic su **Help**, quindi **About**

Fai clic sul numero di versione per copiarlo negli appunti.

<h3 id="403-or-authentication-errors-in-the-code-tab">
  403 or authentication errors in the Code tab
</h3>

Se vedi `Error 403: Forbidden` o altri errori di autenticazione quando usi la scheda Code:

1. Esci e accedi di nuovo dal menu dell'app. Questo è il fix più comune.
2. Verifica di avere un abbonamento a pagamento attivo: Pro, Max, Team o Enterprise.
3. Se la CLI funziona ma Desktop no, esci completamente dall'app desktop, non solo chiudere la finestra, quindi riapri e accedi di nuovo.
4. Controlla la tua connessione Internet e le impostazioni del proxy.

<h3 id="blank-or-stuck-screen-on-launch">
  Blank or stuck screen on launch
</h3>

Se l'app si apre ma mostra una schermata vuota o non reattiva:

1. Riavvia l'app.
2. Controlla gli aggiornamenti in sospeso. Su macOS e Windows l'app si aggiorna automaticamente al lancio; su Linux, aggiorna tramite apt come descritto in [Claude Desktop on Linux](/docs/it/desktop-linux).
3. Su una rete gestita, conferma che il tuo firewall consente gli host CDN nei [requisiti di accesso alla rete](#network-access-requirements).
4. Su Windows, controlla Event Viewer per i log di crash sotto **Windows Logs → Application**.

<h3 id="failed-to-load-session">
  "Failed to load session"
</h3>

Se vedi `Failed to load session`, la cartella selezionata potrebbe non esistere più, un repository Git potrebbe richiedere Git LFS che non è installato, o i permessi dei file potrebbero impedire l'accesso. Prova a selezionare una cartella diversa o riavvia l'app.

<h3 id="session-not-finding-installed-tools">
  Session not finding installed tools
</h3>

Se Claude non riesce a trovare strumenti come `npm`, `node` o altri comandi CLI, verifica che gli strumenti funzionino nel tuo terminale regolare, controlla che il tuo profilo shell configuri correttamente PATH e riavvia l'app desktop per ricaricare le variabili di ambiente.

<h3 id="git-and-git-lfs-errors">
  Git and Git LFS errors
</h3>

Su Windows, Git è richiesto affinché la scheda Code avvii sessioni locali. Se vedi "Git is required," installa [Git per Windows](https://git-scm.com/downloads/win) e riavvia l'app.

Se vedi "Git LFS is required by this repository but is not installed," installa Git LFS da [git-lfs.com](https://git-lfs.com/), esegui `git lfs install` e riavvia l'app.

<h3 id="mcp-servers-not-working-on-windows">
  MCP servers not working on Windows
</h3>

Se gli interruttori del server MCP non rispondono o i server non riescono a connettersi su Windows, controlla che il server sia configurato correttamente nelle tue impostazioni, riavvia l'app, verifica che il processo del server sia in esecuzione in Task Manager e rivedi i log del server per gli errori di connessione.

<h3 id="app-won’t-quit">
  App won't quit
</h3>

* **macOS**: premi Cmd+Q. Se l'app non risponde, usa Force Quit con Cmd+Option+Esc, seleziona Claude e fai clic su Force Quit.
* **Windows**: usa Task Manager con Ctrl+Shift+Esc per terminare il processo Claude.

<h3 id="windows-specific-issues">
  Windows-specific issues
</h3>

* **PATH not updated after install**: apri una nuova finestra di terminale. Gli aggiornamenti di PATH si applicano solo alle nuove sessioni di terminale.
* **Concurrent installation error**: se vedi un errore su un'altra installazione in corso ma non ce n'è una, prova a eseguire il programma di installazione come Amministratore.

<h3 id="branch-doesn’t-exist-yet-when-opening-in-cli">
  "Branch doesn't exist yet" when opening in CLI
</h3>

Le sessioni cloud possono creare rami che non esistono sulla tua macchina locale. Fai clic sul nome del ramo nella barra degli strumenti della sessione per copiarlo, quindi recuperalo localmente:

```bash theme={null}
git fetch origin <branch-name>
git checkout <branch-name>
```

<h3 id="still-stuck">
  Still stuck?
</h3>

* Apri Help → Get Support nell'app desktop, o visita il [centro di supporto Claude](https://support.claude.com/) direttamente
* Per i problemi che si riproducono anche nella CLI `claude` standalone, cerca o segnala un bug su [GitHub Issues](https://github.com/anthropics/claude-code/issues)

Quando segnali un problema, includi la versione dell'app desktop, il tuo sistema operativo, il messaggio di errore esatto e i log pertinenti. Su macOS, controlla Console.app. Su Windows, controlla Event Viewer → Windows Logs → Application.
