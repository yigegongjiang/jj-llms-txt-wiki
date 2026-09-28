> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usa Claude Code con Chrome

> Connetti Claude Code al tuo browser Chrome per testare app web, eseguire il debug con i log della console, automatizzare la compilazione di moduli ed estrarre dati dalle pagine web.

Claude Code si integra con l'[estensione Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) per darti capacità di automazione del browser dalla CLI o dall'[estensione VS Code](/docs/it/vs-code#automate-browser-tasks-with-chrome). Costruisci il tuo codice, quindi testa ed esegui il debug nel browser senza cambiare contesto.

Claude apre nuove schede per le attività del browser e condivide lo stato di accesso del tuo browser, quindi può accedere a qualsiasi sito in cui sei già connesso. Le azioni del browser vengono eseguite in una finestra Chrome visibile in tempo reale. Quando Claude incontra una pagina di accesso o un CAPTCHA, si ferma e ti chiede di gestirlo manualmente.

L'estensione raccoglie le schede che Claude apre in un gruppo di schede Chrome associato alla tua sessione. Nelle sessioni locali, se Claude Code chiude quel gruppo quando la sessione termina dipende da come termina:

* Quando digiti `/clear`, Claude Code chiude il gruppo, incluse le pagine aperte, a meno che il lavoro che sopravvive al clear non sia ancora in esecuzione
* Quando cambi sessione con un comando come `/resume`, esci da Claude Code, o esegui un `/clear` mentre il lavoro che sopravvive è ancora in esecuzione, Claude Code chiude il gruppo solo se contiene nient'altro che nuove schede vuote, quindi le pagine che potresti ancora stare leggendo rimangono aperte

<Note>
  L'integrazione con Chrome funziona con Google Chrome e Microsoft Edge. Claude Code rileva anche l'estensione e configura la connessione in altri browser basati su Chromium, inclusi Brave, Arc, Vivaldi e Opera. L'integrazione con Chrome non è supportata in Windows Subsystem for Linux (WSL).
</Note>

<h2 id="capabilities">
  Capacità
</h2>

Con Chrome connesso, puoi concatenare azioni del browser con attività di codifica in un singolo flusso di lavoro:

* **Debug in tempo reale**: leggi gli errori della console e lo stato del DOM direttamente, quindi correggi il codice che li ha causati
* **Verifica del design**: costruisci un'interfaccia utente da un mock di Figma, quindi aprila nel browser per verificare che corrisponda
* **Test di app web**: testa la convalida dei moduli, verifica la presenza di regressioni visive o verifica i flussi utente
* **App web autenticate**: interagisci con Google Docs, Gmail, Notion o qualsiasi app in cui sei connesso senza connettori API
* **Estrazione di dati**: estrai informazioni strutturate dalle pagine web e salvale localmente
* **Automazione delle attività**: automatizza le attività ripetitive del browser come l'immissione di dati, la compilazione di moduli o i flussi di lavoro multi-sito
* **Caricamento di file**: allega file dal tuo computer ai campi di caricamento sulle pagine web
* **Registrazione della sessione**: registra le interazioni del browser come GIF per documentare o condividere ciò che è accaduto

<h2 id="prerequisites">
  Prerequisiti
</h2>

Prima di utilizzare Claude Code con Chrome, hai bisogno di:

* [Google Chrome](https://www.google.com/chrome/), [Microsoft Edge](https://www.microsoft.com/edge), o un altro browser basato su Chromium come Brave, Arc, Vivaldi, oppure Opera
* Estensione [Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versione 1.0.36 o superiore, disponibile nel Chrome Web Store
* [Claude Code](/docs/it/quickstart#step-1-install-claude-code)
* Un piano Anthropic diretto (Pro, Max, Team, o Enterprise)

L'integrazione con Chrome richiede anche l'accesso con `/login`. Se ti autentichi con una chiave API o un token di lunga durata da [`claude setup-token`](/docs/it/authentication#generate-a-long-lived-token), Claude Code mantiene l'integrazione con Chrome disattivata, anche quando passi `--chrome`, perché l'estensione del browser non può autenticarsi con quelle credenziali. Prima della versione 2.1.216, queste sessioni potevano abilitare l'integrazione con Chrome, ma ogni tentativo di connettersi all'estensione del browser falliva con un errore 403.

<Note>
  L'integrazione con Chrome non è disponibile tramite provider di terze parti come Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry. Se accedi a Claude esclusivamente tramite un provider di terze parti, hai bisogno di un account claude.ai separato per utilizzare questa funzione.
</Note>

<h2 id="get-started-in-the-cli">
  Inizia nella CLI
</h2>

<Steps>
  <Step title="Avvia Claude Code con Chrome">
    Avvia Claude Code con il flag `--chrome`:

    ```bash theme={null}
    claude --chrome
    ```

    La prima volta che avvii con Chrome, Claude Code mostra una finestra di dialogo una tantum che introduce l'integrazione e spiega come funzionano i permessi del sito. Premi Invio per continuare.

    Per abilitare Chrome per le sessioni future senza il flag, vedi [Abilita Chrome per impostazione predefinita](#enable-chrome-by-default).
  </Step>

  <Step title="Chiedi a Claude di usare il browser">
    Questo esempio naviga verso una pagina, interagisce con essa e segnala ciò che trova, tutto dal tuo terminale o editor:

    ```text wrap theme={null}
    Go to code.claude.com/docs, click on the search box,
    type "hooks", and tell me what results appear
    ```

    Se Claude Code chiede il permesso prima di un'azione del browser, approvalo. La finestra di dialogo inizia con `Claude in Chrome wants to` e offre un'opzione per consentire tutte le azioni su quel sito per la sessione. Claude apre una nuova scheda e avvia l'attività.
  </Step>
</Steps>

Esegui `/chrome` in qualsiasi momento per verificare lo stato della connessione, gestire le autorizzazioni, riconnettere l'estensione o scegliere quale browser connesso utilizzare. L'integrazione funziona quando il pannello di stato mostra "Status: Enabled" e "Extension: Installed".

Se più di un browser è connesso, scegli quale Claude utilizza. Quando un'azione del browser inizia prima che tu abbia scelto, Claude ti chiede di sceglierne uno. Per cambiare browser in seguito, esegui `/chrome` e seleziona **Select browser…**. Claude continua a utilizzare la tua scelta anche quando un altro browser si connette.

Per VS Code, vedi [automazione del browser in VS Code](/docs/it/vs-code#automate-browser-tasks-with-chrome).

<h3 id="install-the-extension-when-claude-asks">
  Installa l'estensione quando Claude lo chiede
</h3>

Quando Claude ha bisogno del tuo browser in una sessione interattiva e Claude Code non rileva l'estensione, Claude Code mostra un prompt di installazione intitolato "Claude wants to use your browser". Claude Code chiede al massimo una volta per sessione.

Il prompt offre tre scelte:

* **Install extension**: apre la pagina di installazione dell'estensione nel tuo browser e avvia una configurazione guidata. Claude Code attende l'installazione, connette l'estensione e abilita gli strumenti del browser nella stessa sessione. Quando la connessione è pronta, seleziona "Continue with browser tools" e Claude riprende l'attività nel tuo browser. Puoi uscire dalla configurazione selezionando "Continue without browser tools" e completare in seguito con `/chrome`.
* **Not now**: continua l'attività senza strumenti del browser. Claude Code può chiedere di nuovo in una sessione successiva.
* **Don't ask again**: interrompe il prompt nelle sessioni future. Puoi comunque configurare l'integrazione in qualsiasi momento con `/chrome`.

Se la tua organizzazione blocca il server MCP `claude-in-chrome` con l'[impostazione gestita `deniedMcpServers`](/docs/it/managed-mcp#policy-based-control-with-allowlists-and-denylists), Claude Code non mostra il prompt di installazione.

<h3 id="enable-chrome-by-default">
  Abilita Chrome per impostazione predefinita
</h3>

Per evitare di passare `--chrome` ogni sessione, esegui `/chrome` e seleziona "Enabled by default".

Claude Code si avvia normalmente quando Chrome non è in esecuzione. Prima della v2.1.211, l'avvio potrebbe bloccarsi quando l'integrazione di Chrome era abilitata ma Chrome non era in esecuzione.

Nell'[estensione VS Code](/docs/it/vs-code#automate-browser-tasks-with-chrome), Chrome è disponibile ogni volta che l'estensione Chrome è installata. Non è necessario alcun flag aggiuntivo.

<Note>
  L'abilitazione di Chrome per impostazione predefinita nella CLI aumenta l'utilizzo del contesto poiché gli strumenti del browser vengono sempre caricati. Se noti un aumento del consumo di contesto, disabilita questa impostazione e utilizza `--chrome` solo quando necessario.
</Note>

<h3 id="manage-site-permissions">
  Gestisci le autorizzazioni del sito
</h3>

Le autorizzazioni a livello di sito vengono ereditate dall'estensione Chrome. Gestisci le autorizzazioni nelle impostazioni dell'estensione Chrome per controllare quali siti Claude può navigare, fare clic e digitare.

<h3 id="browser-tools-in-plan-mode">
  Strumenti del browser in plan mode
</h3>

In [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode), un prompt di autorizzazione appare prima che Claude registri una GIF, apra una nuova scheda o esegua una scorciatoia. Se [la modalità bypass permissions è disponibile](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode) nella tua sessione e [il recupero del feature flag](/docs/it/env-vars#features-that-need-feature-flag-fetching) è disattivato, queste chiamate vengono eseguite senza un prompt.

Una chiamata `tabs_context_mcp` richiede anche un prompt quando imposta `createIfEmpty`, e così fa una chiamata `browser_batch` che include una qualsiasi di queste azioni.

<h2 id="example-workflows">
  Flussi di lavoro di esempio
</h2>

Questi esempi mostrano i modi comuni per combinare azioni del browser con attività di codifica. Esegui `/mcp`, seleziona `claude-in-chrome`, quindi seleziona **View tools** per vedere l'elenco completo degli strumenti del browser disponibili.

<h3 id="test-a-local-web-application">
  Testa un'applicazione web locale
</h3>

Quando sviluppi un'app web, chiedi a Claude di verificare che le tue modifiche funzionino correttamente:

```text wrap theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude naviga verso il tuo server locale, interagisce con il modulo e segnala ciò che osserva.

<h3 id="debug-with-console-logs">
  Debug con i log della console
</h3>

Claude può leggere l'output della console per aiutare a diagnosticare i problemi. Dì a Claude quali modelli cercare piuttosto che chiedere tutto l'output della console, poiché i log possono essere dettagliati:

```text wrap theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude legge i messaggi della console e può filtrare per modelli specifici o tipi di errore.

<h3 id="automate-form-filling">
  Automatizza la compilazione dei moduli
</h3>

Velocizza le attività ripetitive di immissione dati:

```text wrap theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude legge il tuo file locale, naviga nell'interfaccia web e immette i dati per ogni record.

<h3 id="upload-files-to-web-pages">
  Carica file su pagine web
</h3>

Claude può allegare file dalla tua macchina ai campi di caricamento su una pagina. Claude Code legge il file e invia i suoi contenuti al browser, quindi i caricamenti funzionano sia nelle sessioni locali che remote. Richiede Claude Code v2.1.211 o successivo.

Questo esempio allega un file di log a un modulo:

```text wrap theme={null}
Open the bug tracker at bugs.example.com, create a new issue,
and attach logs/session.log to it
```

Tre restrizioni si applicano ai caricamenti:

* **Autorizzazioni**: Claude può caricare un file solo quando la sessione è autorizzata a leggerlo, quindi le [regole di autorizzazione](/docs/it/settings-reference#permission-settings) che negano l'accesso `Read` a un file bloccano anche il caricamento.
* **Dimensione**: un singolo caricamento può includere fino a 10 MB di file in totale.
* **Hard link**: Claude rifiuta i file che hanno più hard link, il che è comune all'interno di archivi di gestori di pacchetti come `node_modules`. Copia il file e carica la copia.

<h3 id="draft-content-in-google-docs">
  Bozza di contenuto in Google Docs
</h3>

Usa Claude per scrivere direttamente nei tuoi documenti senza configurazione API:

```text wrap theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude apre il documento, fa clic nell'editor e digita il contenuto. Questo funziona con qualsiasi app web in cui sei connesso: Gmail, Notion, Sheets e altro.

<h3 id="extract-data-from-web-pages">
  Estrai dati dalle pagine web
</h3>

Estrai informazioni strutturate dai siti web:

```text wrap theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude naviga verso la pagina, legge il contenuto e compila i dati in un formato strutturato.

<h3 id="run-multi-site-workflows">
  Esegui flussi di lavoro multi-sito
</h3>

Coordina le attività su più siti web:

```text wrap theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude lavora su più schede per raccogliere informazioni e completare il flusso di lavoro.

<h3 id="record-a-demo-gif">
  Registra una GIF demo
</h3>

Crea registrazioni condivisibili delle interazioni del browser:

```text wrap theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude registra la sequenza di interazione e la salva come file GIF. La registrazione cattura tutto ciò che è visibile nel browser, inclusi i dettagli dell'account su pagine con accesso effettuato, quindi revisionala prima di condividerla al di fuori del tuo team.

<h3 id="save-screenshots-to-disk">
  Salva screenshot su disco
</h3>

Chiedi a Claude di mantenere uno screenshot come file:

```text wrap theme={null}
Take a screenshot of the checkout page and save it to disk
```

Claude salva l'immagine su disco e segnala il percorso del file. Prima della v2.1.211, l'opzione `save_to_disk` dello strumento screenshot non scriveva un file.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="extension-not-detected">
  Estensione non rilevata
</h3>

Se Claude Code non riesce a rilevare l'estensione Chrome:

1. Verifica che l'estensione Chrome sia installata e abilitata in `chrome://extensions`
2. Verifica che Claude Code sia aggiornato eseguendo `claude --version`
3. Verifica che Chrome sia in esecuzione
4. Esegui `/chrome` e seleziona "Reconnect extension" per ristabilire la connessione
5. Se il problema persiste, riavvia sia Claude Code che Chrome

La prima volta che abiliti l'integrazione con Chrome, Claude Code installa un file di configurazione dell'host di messaggistica nativa. Chrome legge questo file all'avvio, quindi se l'estensione non viene rilevata al primo tentativo, riavvia Chrome per raccogliere la nuova configurazione.

Claude Code apre una scheda del browser che ti chiede di connettere l'estensione solo al primo install. Claude Code non la riapre quando una sessione successiva riscrive il file di configurazione, ad esempio dopo il passaggio tra build o directory di configurazione.

Se la connessione continua a non funzionare, verifica che il file di configurazione dell'host esista in:

Per Chrome:

* **macOS**: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: controlla `HKCU\Software\Google\Chrome\NativeMessagingHosts\` nel Registro di Windows

Per Edge:

* **macOS**: `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: controlla `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\` nel Registro di Windows

Gli altri browser basati su Chromium leggono lo stesso file dalla loro directory di configurazione, denominata in base al browser. Ad esempio, Brave su macOS utilizza `~/Library/Application Support/BraveSoftware/Brave-Browser/NativeMessagingHosts/`, e su Windows ogni browser ha la propria chiave di registro, come `HKCU\Software\BraveSoftware\Brave-Browser\NativeMessagingHosts\`.

<h3 id="browser-not-responding">
  Browser non risponde
</h3>

Se i comandi del browser di Claude smettono di funzionare:

1. Verifica se una finestra di dialogo modale (avviso, conferma, prompt) sta bloccando la pagina. Le finestre di dialogo JavaScript bloccano gli eventi del browser e impediscono a Claude di ricevere comandi. Chiudi manualmente la finestra di dialogo, quindi dì a Claude di continuare.
2. Chiedi a Claude di creare una nuova scheda e riprovare
3. Riavvia l'estensione Chrome disabilitandola e riabilitandola in `chrome://extensions`

<h3 id="connection-drops-during-long-sessions">
  La connessione si interrompe durante le sessioni lunghe
</h3>

Il service worker dell'estensione Chrome può diventare inattivo durante le sessioni estese, il che interrompe la connessione. Se gli strumenti del browser smettono di funzionare dopo un periodo di inattività, esegui `/chrome` e seleziona "Reconnect extension".

<h3 id="windows-specific-issues">
  Problemi specifici di Windows
</h3>

Su Windows, potresti riscontrare:

* **Conflitti di named pipe (EADDRINUSE)**: se un altro processo sta utilizzando la stessa named pipe, riavvia Claude Code. Chiudi tutte le altre sessioni di Claude Code che potrebbero utilizzare Chrome.
* **Errori dell'host di messaggistica nativa**: se l'host di messaggistica nativa si arresta in modo anomalo all'avvio, prova a reinstallare Claude Code per rigenerare la configurazione dell'host.
* **Le pagine di configurazione non si aprono**: aggiorna Claude Code. Prima della v2.1.211, la scheda del browser che ti chiede di connettere l'estensione potrebbe non aprirsi su Windows.

<h3 id="common-error-messages">
  Messaggi di errore comuni
</h3>

Questi sono gli errori più frequentemente riscontrati e come risolverli:

| Errore                                      | Causa                                                                                                                                                                   | Soluzione                                                                                                                                                                                                                                                         |
| ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Browser extension is not connected"        | L'host di messaggistica nativa non può raggiungere l'estensione, oppure l'allowlist IP della tua organizzazione rifiuta la connessione a `bridge.claudeusercontent.com` | Riavvia Chrome e Claude Code, quindi esegui `/chrome` per riconnetterti. Se la tua organizzazione utilizza l'allowlist IP e l'errore persiste, vedi [Organization IP allowlists and proxy egress](/docs/it/network-config#organization-ip-allowlists-and-proxy-egress) |
| Extension shows "Not detected" in `/chrome` | L'estensione Chrome non è installata o è disabilitata                                                                                                                   | Installa o abilita l'estensione in `chrome://extensions`                                                                                                                                                                                                          |
| "No tab available"                          | Claude ha tentato di agire prima che una scheda fosse pronta                                                                                                            | Chiedi a Claude di creare una nuova scheda e riprovare                                                                                                                                                                                                            |
| "Receiving end does not exist"              | Il service worker dell'estensione è diventato inattivo                                                                                                                  | Esegui `/chrome` e seleziona "Reconnect extension"                                                                                                                                                                                                                |

<h2 id="see-also">
  Vedi anche
</h2>

* [Uso del computer](/docs/it/computer-use): controlla le app macOS native quando un'attività non può essere eseguita in un browser
* [Usa Claude Code in VS Code](/docs/it/vs-code#automate-browser-tasks-with-chrome): automazione del browser nell'estensione VS Code
* [Riferimento CLI](/docs/it/cli-reference): flag della riga di comando incluso `--chrome`
* [Flussi di lavoro comuni](/docs/it/common-workflows): altri modi per utilizzare Claude Code
* [Dati e privacy](/docs/it/data-usage): come Claude Code gestisce i tuoi dati
* [Introduzione a Claude in Chrome](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome): documentazione completa per l'estensione Chrome, incluse scorciatoie, pianificazione e autorizzazioni
