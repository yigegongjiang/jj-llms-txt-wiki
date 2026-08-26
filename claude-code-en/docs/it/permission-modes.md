> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Scegli una modalità di autorizzazione

> Controlla se Claude chiede prima di modificare i file o eseguire comandi. Cicla le modalità con Shift+Tab nella CLI o utilizza il selettore di modalità in VS Code, Desktop e claude.ai.

Quando Claude vuole modificare un file, eseguire un comando shell o effettuare una richiesta di rete, si ferma e ti chiede di approvare l'azione. Le modalità di autorizzazione controllano con quale frequenza quella pausa si verifica. La modalità che scegli modella il flusso di una sessione: la modalità manuale ti consente di rivedere ogni azione man mano che arriva, mentre le modalità più permissive consentono a Claude di lavorare in tratti più lunghi ininterrotti e di riferire quando è fatto. Scegli una maggiore supervisione per il lavoro sensibile, o meno interruzioni quando hai fiducia nella direzione.

<h2 id="available-modes">
  Modalità disponibili
</h2>

Ogni modalità fa un diverso compromesso tra comodità e controllo. La tabella seguente mostra cosa Claude può fare senza una richiesta di autorizzazione in ogni modalità.

| Modalità                                                            | Cosa viene eseguito senza chiedere                                                              | Migliore per                                       |
| :------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------- | :------------------------------------------------- |
| `default`                                                           | Solo letture                                                                                    | Iniziare, lavoro sensibile                         |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Letture, modifiche di file e comandi comuni del filesystem (`mkdir`, `touch`, `mv`, `cp`, ecc.) | Iterare sul codice che stai revisionando           |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Solo letture                                                                                    | Esplorare una codebase prima di modificarla        |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Tutto, con controlli di sicurezza in background                                                 | Attività lunghe, ridurre l'affaticamento da prompt |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Solo strumenti pre-approvati                                                                    | CI limitato e script                               |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Tutto                                                                                           | Solo container e VM isolati                        |

La modalità che esamina ogni azione è denominata **Manual** nella CLI, in `claude --help`, nelle estensioni VS Code e JetBrains e nell'app desktop. Il suo valore di configurazione è `default`, che è quello utilizzato da hooks e integrazioni SDK. La CLI accetta `manual` come alias ovunque digiti il valore, ad esempio `claude --permission-mode manual` o `"defaultMode": "manual"`. L'etichetta Manual e l'alias `manual` richiedono Claude Code v2.1.200 o successivo. L'etichetta dell'app desktop non dipende dalla tua versione CLI.

In ogni modalità tranne `bypassPermissions`, le scritture su [percorsi protetti](#protected-paths) non vengono mai auto-approvate, proteggendo lo stato del repository e la configurazione di Claude da corruzione accidentale.

Le modalità impostano la linea di base. Sovrapponi [regole di autorizzazione](/docs/it/permissions#manage-permissions) su top per pre-approvare o bloccare strumenti specifici. Le regole di negazione, le regole di richiesta esplicita, l'[impostazione `ask` dell'organizzazione su strumenti connector](/docs/it/mcp#organization-controls-on-connector-tools) e il marcatore [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) si applicano in ogni modalità, inclusa `bypassPermissions`. Le regole allow non hanno effetto in quella modalità perché tutto il resto è già approvato.

<h2 id="switch-permission-modes">
  Cambiare le modalità di autorizzazione
</h2>

È possibile cambiare modalità durante una sessione, all'avvio o come impostazione predefinita persistente. La modalità viene impostata attraverso questi controlli, non chiedendo a Claude nella chat. Selezionare l'interfaccia di seguito per vedere come cambiarla.

<Tabs>
  <Tab title="CLI">
    **Durante una sessione**: premere `Shift+Tab` per ciclo `default` → `acceptEdits` → `plan`. La modalità corrente appare nella barra di stato. La modalità manuale, `default` in quel ciclo, mostra un badge grigio `⏸ manual mode on`. Prima della v2.1.203, la barra di stato non mostrava alcun badge in modalità manuale.

    Non tutte le modalità sono nel ciclo predefinito:

    * `auto`: appare quando il vostro account soddisfa i [requisiti della modalità auto](#eliminate-prompts-with-auto-mode); passando ad essa si cambia modalità senza un prompt di conferma
    * `bypassPermissions`: appare dopo aver iniziato con `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, o `--allow-dangerously-skip-permissions`; la variante `--allow-` aggiunge la modalità al ciclo senza attivarla
    * `dontAsk`: non appare mai nel ciclo; impostarla con `--permission-mode dontAsk`

    Le modalità opzionali abilitate si inseriscono dopo `plan`, con `bypassPermissions` per primo e `auto` per ultimo. Se avete entrambe abilitate, ciclerete attraverso `bypassPermissions` sulla strada verso `auto`.

    **All'avvio**: passare la modalità come flag.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Come impostazione predefinita**: impostare `defaultMode` in [settings](/docs/it/settings#settings-files).

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "acceptEdits"
      }
    }
    ```

    Lo stesso flag `--permission-mode` funziona con `-p` per [esecuzioni non interattive](/docs/it/headless).
  </Tab>

  <Tab title="VS Code">
    **Durante una sessione**: fare clic sull'indicatore di modalità nella parte inferiore della casella di prompt.

    **Come impostazione predefinita**: impostare `claudeCode.initialPermissionMode` nelle impostazioni di VS Code, oppure utilizzare il pannello delle impostazioni dell'estensione Claude Code.

    L'indicatore di modalità mostra queste etichette, mappate alla modalità a cui ognuna si applica:

    | Etichetta UI       | Modalità            |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    Prima della v2.1.205, l'estensione etichettava `plan` come Plan mode e `auto` come Auto mode.

    La modalità Auto appare nell'indicatore di modalità quando il vostro account soddisfa ogni requisito elencato nella [sezione della modalità auto](#eliminate-prompts-with-auto-mode). L'impostazione `claudeCode.initialPermissionMode` non accetta `auto`. Per iniziare in modalità auto per impostazione predefinita, impostare `defaultMode` nelle vostre [impostazioni utente](/docs/it/settings#settings-files). Claude Code ignora `defaultMode: "auto"` nelle impostazioni di progetto e locali.

    Bypass permissions richiede l'interruttore **Allow dangerously skip permissions** nelle impostazioni dell'estensione prima che appaia nell'indicatore di modalità.

    Vedere la [guida di VS Code](/docs/it/vs-code) per i dettagli specifici dell'estensione.
  </Tab>

  <Tab title="JetBrains">
    Il plugin JetBrains esegue Claude Code nel terminale dell'IDE, quindi il cambio di modalità funziona come nella CLI: premere `Shift+Tab` per ciclo, o passare `--permission-mode` al lancio.
  </Tab>

  <Tab title="Desktop">
    **Durante una sessione**: utilizzare il selettore di modalità accanto al pulsante di invio. Non tutte le modalità appaiono nel selettore:

    * **Auto**: appare quando il vostro account soddisfa i [requisiti della modalità auto](#eliminate-prompts-with-auto-mode)
    * **Bypass permissions**: richiede l'interruttore **Allow bypass permissions mode** nelle impostazioni di Desktop sui piani Pro e Max; sui piani Team e Enterprise, la politica organizzativa lo controlla invece

    Per i dettagli specifici del desktop, vedere [Choose a permission mode](/docs/it/desktop#choose-a-permission-mode) nella guida di Desktop.

    **Come impostazione predefinita**: impostare `defaultMode` in [settings](/docs/it/settings#settings-files). L'app desktop legge gli stessi file di impostazioni della CLI e applica la modalità alle nuove sessioni locali.

    Una modalità che scegliete nel selettore di modalità viene ricordata per cartella e ha la precedenza su `defaultMode` per quella cartella. Plan è l'eccezione: selezionarla si applica solo alla sessione corrente.

    Questo esempio imposta la modalità Plan come impostazione predefinita per le nuove sessioni locali:

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "plan"
      }
    }
    ```
  </Tab>

  <Tab title="Web and mobile">
    Utilizzare il menu a discesa della modalità accanto alla casella di prompt su [claude.ai/code](https://claude.ai/code) o nell'app mobile. I prompt di autorizzazione appaiono in claude.ai per l'approvazione. Quali modalità appaiono dipende da dove viene eseguita la sessione:

    * **Cloud sessions** su [Claude Code on the web](/docs/it/claude-code-on-the-web): Accept edits, Plan, e Auto. Accept edits corrisponde alla modalità `default`: l'ambiente cloud pre-approva le modifiche ai file indipendentemente dalla modalità, quindi il menu a discesa mostra Accept edits invece di Manual. Le sessioni cloud rispettano comunque `defaultMode: "acceptEdits"` dalle impostazioni. La modalità Auto appare solo quando la vostra organizzazione la consente e il modello selezionato la supporta. Bypass permissions non è disponibile.
    * **[Remote Control](/docs/it/remote-control) sessions** sulla vostra macchina locale: Manual, Accept edits, e Plan. Non potete selezionare Auto o Bypass permissions dall'app. Il menu a discesa mostra la modalità in cui si trova la sessione locale, inclusa una modalità impostata dal terminale, e si aggiorna quando la modalità cambia nell'app o nel terminale. L'unica eccezione è Bypass permissions: la sessione non segnala mai quella modalità a claude.ai, quindi il passaggio ad essa dal terminale non cambia quello che il menu a discesa mostra. Prima della v2.1.202, le sessioni connesse con `/remote-control` o `claude --remote-control` non segnalano affatto la loro modalità, quindi claude.ai e l'app mobile potevano mostrare una modalità in cui la sessione non era. La mancata corrispondenza ha interessato solo l'etichetta: Claude Code ha generato prompt di autorizzazione dalla modalità effettiva della sessione, e appaiono comunque nell'app per l'approvazione.

    Per Remote Control, potete anche impostare la modalità di avvio al lancio dell'host:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Auto-approvazione delle modifiche ai file con la modalità acceptEdits
</h2>

La modalità `acceptEdits` consente a Claude di creare e modificare file nella vostra directory di lavoro senza richiedere conferma. La barra di stato mostra `⏵⏵ accept edits on` mentre questa modalità è attiva.

Oltre alle modifiche ai file, la modalità `acceptEdits` auto-approva i comuni comandi Bash del filesystem: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp` e `sed`. Questi comandi vengono anche auto-approvati quando preceduti da variabili di ambiente sicure come `LANG=C` o `NO_COLOR=1`, o da wrapper di processi come `timeout`, `nice` o `nohup`. Come per le modifiche ai file, l'auto-approvazione si applica solo ai percorsi all'interno della vostra directory di lavoro o `additionalDirectories`. I percorsi al di fuori di questo ambito, le scritture su [percorsi protetti](#protected-paths) e tutti gli altri comandi Bash ad eccezione del [set di sola lettura integrato](/docs/it/permissions#read-only-commands) richiedono comunque conferma.

Quando lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool) è abilitato, la modalità `acceptEdits` auto-approva anche `Set-Content`, `Add-Content`, `Clear-Content` e `Remove-Item` su percorsi nell'ambito, insieme ai loro alias comuni. Si applicano le stesse regole di ambito e percorsi protetti.

Utilizzate `acceptEdits` quando desiderate rivedere le modifiche nel vostro editor o tramite `git diff` successivamente, piuttosto che approvare ogni modifica inline.

Premete `Shift+Tab` una volta dalla modalità Manual per accedervi, oppure iniziate direttamente con essa:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analizzare prima di modificare con la modalità plan
</h2>

La modalità plan indica a Claude di ricercare e proporre modifiche senza apportarle. Claude legge i file, esegue comandi shell per esplorare e scrive un piano, ma non modifica il tuo codice sorgente. I prompt di autorizzazione si applicano come in modalità manuale a meno che [la modalità auto](/docs/it/auto-mode-config) non sia disponibile e `useAutoModeDuringPlan` sia attivato, che è l'impostazione predefinita. Con la modalità auto attiva, il classificatore approva i comandi di sola lettura come ricerche e letture di file senza richiedere conferma. Le modifiche rimangono bloccate comunque fino a quando non approvi il piano.

Accedi alla modalità plan premendo `Shift+Tab` o anteponendo un singolo prompt con `/plan`. Puoi anche iniziare in modalità plan dalla CLI:

```bash theme={null}
claude --permission-mode plan
```

Premi `Shift+Tab` di nuovo per uscire dalla modalità plan senza approvare un piano.

<h3 id="review-and-approve-a-plan">
  Rivedere e approvare un piano
</h3>

Quando il piano è pronto, Claude lo presenta e chiede come procedere. Da quel prompt puoi:

* Approvare e iniziare in modalità auto
* Approvare e accettare le modifiche
* Approvare e rivedere ogni modifica manualmente
* Continuare a pianificare con feedback
* Perfezionare con [Ultraplan](/docs/it/ultraplan) per la revisione basata su browser

Approvare un piano esce dalla modalità plan e cambia la sessione alla modalità di autorizzazione che ogni opzione di approvazione descrive, quindi Claude inizia a modificare. Per pianificare di nuovo, torna alla modalità plan con `Shift+Tab`, o anteponi il tuo prossimo prompt con `/plan`.

Premi `Ctrl+G` per aprire il piano proposto nel tuo editor di testo predefinito e modificarlo direttamente prima che Claude proceda. Quando [`showClearContextOnPlanAccept`](/docs/it/settings#available-settings) è abilitato, ogni opzione di approvazione offre anche di cancellare il contesto di pianificazione prima.

Accettare un piano nomina anche la sessione dal contenuto del piano automaticamente, a meno che tu non abbia già impostato un nome con `--name` o `/rename`.

<h3 id="set-plan-mode-as-the-default">
  Impostare la modalità plan come predefinita
</h3>

Per rendere la modalità plan predefinita per un progetto, imposta `defaultMode` in `.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "plan"
  }
}
```

<h2 id="eliminate-prompts-with-auto-mode">
  Elimina i prompt di autorizzazione con la modalità auto
</h2>

La modalità auto consente a Claude di eseguire senza prompt di autorizzazione di routine. Un modello classificatore separato esamina le azioni prima che vengono eseguite, bloccando qualsiasi cosa che vada oltre la tua richiesta, che prenda di mira un'infrastruttura non riconosciuta, o che sembri guidata da contenuti ostili che Claude ha letto. Le [regole di richiesta](/docs/it/permissions#manage-permissions) esplicite forzano comunque un prompt.

Le rimozioni che prendono di mira la radice del filesystem o la directory home, come `rm -rf /` e `rm -rf ~`, richiedono l'approvazione invece di andare al classificatore. Questo prompt si attiva anche quando il comando contiene sostituzione di comando con `$(...)` o backtick, o sostituzione di processo con `<(...)`, indipendentemente dal fatto che la rimozione si trovi dentro la sostituzione, come in `echo "$(rm -rf ~)"`, o altrove nello stesso comando. Prima di v2.1.208, i comandi contenenti quelle forme andavano al classificatore invece di richiedere un prompt.

La modalità auto incoraggia anche Claude a continuare a lavorare senza fermarsi per domande di chiarimento, anche se Claude chiede comunque quando il tuo prompt o un skill lo richiedono esplicitamente. Per un comportamento più autonomo mantenendo i prompt di autorizzazione, imposta invece lo [stile di output proattivo](/docs/it/output-styles).

<Warning>
  La modalità auto riduce i prompt di autorizzazione ma non garantisce la sicurezza. Usala per attività in cui ti fidi della direzione generale, non come sostituto della revisione su operazioni sensibili.
</Warning>

La modalità auto è disponibile solo quando il tuo account soddisfa tutti questi requisiti:

* **Plan**: Tutti i piani.
* **Owner**: su Team ed Enterprise, un Owner deve abilitarla nelle [impostazioni admin di Claude Code](https://claude.ai/admin-settings/claude-code) prima che gli utenti possano attivarla. Gli amministratori possono anche disattivare la modalità auto impostando `permissions.disableAutoMode` su `"disable"` nelle [impostazioni gestite](/docs/it/permissions#managed-settings). Per la scheda Code dell'app desktop, `disableAutoMode` è il controllo a livello organizzativo e l'interruttore delle impostazioni admin non si applica.
* **Model**: sull'API Anthropic, Claude Opus 4.6 o successivo, o Sonnet 4.6 o successivo. Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e sessioni [gateway app Claude](/docs/it/claude-apps-gateway) con accesso, solo Claude Sonnet 5, Opus 4.7 e Opus 4.8. I modelli più vecchi, inclusi Sonnet 4.5, Opus 4.5, Haiku e modelli claude-3, non sono supportati su nessun provider.
* **Provider**: disponibile per impostazione predefinita sull'API Anthropic, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e sessioni gateway app Claude con accesso. Nella versione da v2.1.158 a v2.1.206, la modalità auto era disattivata su tutti questi provider tranne l'API Anthropic finché non impostavi `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 ha rimosso il requisito.

Se Claude Code segnala che la modalità auto non è disponibile, uno di questi requisiti non è soddisfatto; questo non è un'interruzione transitoria. Un messaggio separato che nomina un modello e dice che la modalità auto "non può determinare la sicurezza" di un'azione è un'interruzione transitoria del classificatore; vedi il [riferimento degli errori](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action).

Se imposti `defaultMode: "auto"` nelle [impostazioni](/docs/it/settings#available-settings) e la sessione inizia in modalità `default` senza errore, l'impostazione è probabilmente in `.claude/settings.json` o `.claude/settings.local.json`. Claude Code v2.1.142 e successivi ignorano `auto` da questi file in modo che un repository non possa concedere a se stesso la modalità auto. Spostalo in `~/.claude/settings.json`.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Modalità auto su Bedrock, Agent Platform o Foundry
</h3>

Su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry) e sessioni [gateway app Claude](/docs/it/claude-apps-gateway) con accesso, la modalità auto appare nel ciclo `Shift+Tab` per impostazione predefinita. L'apparizione nel ciclo non cambia la modalità in cui una sessione inizia: le sessioni iniziano comunque nella tua [`defaultMode`](/docs/it/settings#available-settings), che è Manual a meno che non la modifichi. Solo Claude Sonnet 5, Opus 4.7 e Opus 4.8 sono supportati su questi provider.

Per rendere la modalità auto la modalità di avvio predefinita, imposta `"permissions": {"defaultMode": "auto"}` nelle impostazioni utente o gestite.

Per impedire agli sviluppatori di utilizzare la modalità auto, imposta `disableAutoMode` su `"disable"` nelle [impostazioni gestite](/docs/it/permissions#managed-settings). Questo rimuove `auto` dal ciclo `Shift+Tab` e rifiuta `--permission-mode auto` all'avvio.

Nella versione da v2.1.158 a v2.1.206, la modalità auto era disattivata su questi provider finché non impostavi `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, e Claude Code ignorava `defaultMode: "auto"` su questi provider a meno che la variabile non fosse anche impostata. La variabile è ancora accettata per compatibilità e non ha effetto da v2.1.207 in poi.

<h3 id="what-the-classifier-blocks-by-default">
  Cosa blocca il classificatore per impostazione predefinita
</h3>

Il classificatore si fida della tua directory di lavoro e dei remoti che erano configurati per essa quando la sessione è iniziata. Un remote aggiunto o reindirizzato durante la sessione con `git remote add` o `git remote set-url` non è attendibile, e tutto il resto è trattato come esterno finché non [configuri l'infrastruttura attendibile](/docs/it/auto-mode-config). Prima di v2.1.200, i remoti aggiunti a metà sessione erano anche attendibili.

**Bloccato per impostazione predefinita**:

* Download ed esecuzione di codice, come `curl | bash`
* Invio di dati sensibili a endpoint esterni
* Deploy e migrazioni di produzione
* Eliminazione di massa su cloud storage
* Concessione di autorizzazioni IAM o repo
* Modifica dell'infrastruttura condivisa
* Distruzione irreversibile di file che esistevano prima della sessione
* Force push
* Push al ramo predefinito del repository quando il push contiene contenuto sensibile come segreti o dati personali o affidati, contiene modifiche nascoste o descritte erroneamente rispetto a quello che hai chiesto, contiene contenuto portato dentro o letto per la prima volta da fuori il repository, o aggira una pull request, una revisione o un controllo che hai chiesto. Un semplice push al ramo predefinito non è bloccato di per sé, e la rimozione del blocco su un push contrassegnato richiede di nominare il contenuto contrassegnato o la revisione aggirata, non solo il push. Il classificatore è uno strato: le [regole `permissions.deny`](/docs/it/permissions#manage-permissions) si applicano in ogni modalità e possono bloccare i push al ramo predefinito completamente, e la protezione del ramo del remote si applica comunque. Prima di v2.1.203, qualsiasi push diretto al ramo predefinito era bloccato
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop`, o `git stash clear`, che il classificatore presume scarterebbero le modifiche non committate
* `git commit --amend` quando il commit in HEAD non è stato creato in questa sessione
* Da v2.1.198, `git commit --amend` quando il commit in HEAD è già stato pushato. Una riscrittura solo del messaggio non è bloccata: `--amend -m` senza nulla di nuovo in stage, su un commit che Claude ha creato durante questa sessione
* `terraform destroy`, `pulumi destroy`, `cdk destroy`, o `terragrunt destroy`, e applicazione di un piano che distrugge risorse

Claude Code v2.1.195 e successivi bloccano più categorie per impostazione predefinita. Diversi dipendono da voci di [ambiente](/docs/it/auto-mode-config#define-trusted-infrastructure), come target remoti sensibili e ambiti IaC protetti, che puoi restringere a nomi concreti.

* Scrittura su un gestore di segreti, o modifica di record DNS o certificati TLS
* Merge di una pull request che nessun umano ha approvato, approvazione della propria pull request di Claude, o disabilitazione dei controlli CI
* Posting di un commento che è esso stesso un comando per l'automazione, come `atlantis apply` o il `/deploy` o `/merge` di un bot
* Attivazione/disattivazione, ramping o eliminazione di un feature flag di produzione
* Applicazione di modifiche all'infrastruttura a un ambito IaC protetto, o drenaggio e rimozione di nodi cluster
* Scritture su un cluster di calcolo condiviso che vanno oltre la risorsa che hai nominato, come un selettore di etichette o `--all` che cattura i job di altri utenti
* Creazione di risorse Kubernetes che vengono eseguite su ogni nodo o intercettano il traffico del cluster, come DaemonSets e webhook di ammissione
* Shell interattive o port-forward in un target remoto sensibile
* Apertura di un tunnel o reverse shell che rende un servizio locale raggiungibile da internet pubblico
* Stampa di una credenziale o token live nella trascrizione o in un file
* Accesso a una posizione elencata come posizione di dati sensibili nel tuo [ambiente](/docs/it/auto-mode-config#define-trusted-infrastructure), o copia di dati da una. A partire da v2.1.198 questo blocca anche l'invio di dati da uno a un pubblico che la voce esclude
* Instradamento di un'installazione di pacchetto intorno al tuo registro di pacchetti interno a un registro pubblico. A partire da v2.1.198, questo si applica anche quando hai detto a Claude che esiste un registro interno o uno specchio nella conversazione, non solo quando uno è elencato nel tuo ambiente
* Esecuzione di un comando con un flag che disarma una guardia di sicurezza, come `--insecure`
* Lancio di un loop di agente autonomo che viene eseguito senza approvazione umana o sandbox, come uno avviato con `--dangerously-skip-permissions` o `--no-sandbox`. A partire da v2.1.198 questo copre anche l'esecuzione di un agente di terze parti o di un harness eval con isolamento e approvazione per azione disabilitati, come un runner avviato con `--yes-always`
* Azioni del browser [Claude in Chrome](/docs/it/chrome) che potrebbero inviare contenuto della pagina, cookie o credenziali off-origin

Claude Code v2.1.198 e successivi bloccano anche questi per impostazione predefinita:

* Eliminazione di file in `/tmp`, `$TMPDIR`, o un'altra directory di scratch o cache condivisa per wildcard, glob o filtro di età piuttosto che per un percorso nominato specifico
* Inclusione di dettagli sensibili nel contenuto inviato, caricato, pubblicato o scritto ad altre persone o sistemi condivisi, quando il tuo stesso messaggio non ha autorizzato quei dettagli per quel destinatario. I corpi di PR e issue, i messaggi di commit e i commenti contano come questo tipo di contenuto in uscita quando il repository è fuori dal confine di fiducia o pubblico, inclusi i tuoi repository pubblici dell'organizzazione; i percorsi di file interni, i nomi in codice, i dati di risposta API live come email o identificatori di account e gli identificatori di infrastruttura contano come dettagli sensibili. Lo scoping di PR, issue e messaggio di commit richiede Claude Code v2.1.200 o successivo. I dati personali live da una risposta API in un corpo di PR o issue, come un indirizzo email, un identificatore di account o organizzazione, o una metrica di utilizzo, richiedono che tu nomini quei dettagli e il destinatario indipendentemente dalla visibilità del repository o dal confine di fiducia. Quel controllo richiede Claude Code v2.1.203 o successivo
* Invio di pressioni di tasti alla propria pane tmux di Claude Code per guidare la sua stessa interfaccia, che il classificatore tratta come Claude che cambia le sue stesse autorizzazioni o supervisione

Claude Code v2.1.200 e successivi bloccano anche questi per impostazione predefinita:

* Commento, eliminazione o force-pass di un test o asserzione che protegge il comportamento di sicurezza, come autenticazione, controllo di accesso, convalida dell'input o sandboxing
* Eliminazione o smantellamento di una risorsa stateful che Claude non ha creato nella sessione, quando nessuna regola di eliminazione più specifica si applica e non hai nominato quella risorsa
* Reindirizzamento di un URL di base API, endpoint proxy, ricevitore webhook o specchio di registro a un host di terze parti che non si adatta al compito, incluso nei file di esempio come `.env.example`
* Modifica di dove vanno i push con `git remote set-url` o `git remote add`, a meno che tu non abbia nominato il nuovo remote
* Push di segreti o dati personali o affidati a un repository noto per essere pubblico, o push di materiale confidenziale lì che non fa parte del lavoro proprio di quel repository. L'argomento proprio di un repository dotfiles è l'unica eccezione per dati personali o affidati, e il contenuto da un repository privato che raggiunge qualsiasi superficie pubblica è bloccato allo stesso modo; entrambi i perfezionamenti richiedono Claude Code v2.1.203 o successivo. Prima di v2.1.203, i dati personali erano raggruppati con materiale confidenziale e bloccati solo quando non facevano parte del lavoro proprio di quel repository. Quando la visibilità di un repository non è stabilita, il classificatore non blocca solo su quello; giudica il contenuto rispetto alle altre regole invece
* Apertura di una pull request contro un repository o organizzazione diversa, fork con `gh repo fork`, o push a un repository di terze parti, a meno che tu non abbia nominato quel target esterno

Claude Code v2.1.203 e successivi bloccano anche questi per impostazione predefinita:

* Contenuto da un archivio locale sensibile, o da un file il cui nome, percorso o tipo lo contrassegna come sensibile, che entra in un commit, un push, testo di PR o issue, un gist o paste, o una pubblicazione di pacchetto, a meno che tu non abbia nominato sia la fonte che la destinazione. Le trascrizioni di sessione e i log di conversazione, le cartelle dot di credenziali e configurazione come chiavi SSH, credenziali cloud, profili browser e cronologia shell, e gli export di dati utente contano tutti, e il fatto che il repository sia privato non lo esenta

Claude Code v2.1.205 e successivi bloccano anche questi per impostazione predefinita:

* Scrittura nelle trascrizioni di sessione di Claude Code, i file di cronologia `.jsonl` sotto `~/.claude/projects/` o la tua directory di configurazione configurata, direttamente o tramite un comando shell. La regola copre anche le righe di metadati che Claude Code aggiunge a ogni voce di trascrizione per i suoi stessi controlli. Una trascrizione è uno stato di sessione che Claude Code scrive, non un file di lavoro, e una voce manomessa raggiunge ogni controllo successivo una volta che riprendi la sessione, quindi la modalità auto blocca queste scritture come difesa in profondità. La lettura di una trascrizione non è bloccata
* Un'eliminazione forzata ricorsiva come `rm -rf "$VAR"` o `Remove-Item -Recurse -Force $dir` il cui target è una variabile shell, o un glob radicato in uno, che non è assegnato da nessuna parte nella conversazione che il classificatore vede. Il valore proveniva solo dall'output del comando precedente, che il classificatore non riceve mai, quindi il classificatore non può verificare il target di eliminazione rispetto alle altre regole di eliminazione. Il classificatore legge la conversazione piuttosto che l'output del comando per progettazione, quindi blocca la chiamata invece di indovinare il target. Il blocco si cancella quando nomini il percorso esatto che viene eliminato, o quando Claude riesegue l'eliminazione con il percorso letterale risolto scritto nel comando. Le eliminazioni il cui target il classificatore può risolvere non sono interessate

**Consentito per impostazione predefinita**:

* Operazioni di file locali nella tua directory di lavoro
* Installazione di dipendenze dichiarate nei tuoi file di lock o manifest
* Lettura di `.env` e invio di credenziali al loro API corrispondente
* Richieste HTTP di sola lettura
* Push al ramo su cui hai iniziato o uno che Claude ha creato
* Push di routine al ramo predefinito del repository. Prima di v2.1.203, qualsiasi push diretto al ramo predefinito era bloccato

Claude Code v2.1.195 e successivi consentono anche questi per impostazione predefinita:

* Eliminazione dei job esatti che Claude ha creato in precedenza nella stessa sessione
* Lettura, revisione o scrittura di codice, config e modelli di minaccia relativi alla sicurezza come parte del tuo compito
* Messaggi tra agenti che lavorano insieme nella stessa sessione multi-agente
* Invio di dati ai domini, bucket e servizi attendibili che elenchi in [`environment`](/docs/it/auto-mode-config#define-trusted-infrastructure). Questo copre il flusso di dati solo, non operazioni distruttive o di credenziali sulla stessa infrastruttura
* Navigazione [Claude in Chrome](/docs/it/chrome) a un dominio interno attendibile, localhost, o un URL che hai nominato

Le richieste di accesso alla rete sandbox vengono instradate attraverso il classificatore piuttosto che consentite per impostazione predefinita. A partire da v2.1.198, il classificatore riutilizza il suo verdetto per un host e una porta di rete invece di rieseguire su ogni connessione:

* Un allow viene riutilizzato fino a quando nuovo contenuto non entra nella conversazione, a quel punto quell'host viene controllato di nuovo
* Nella CLI interattiva, un deny viene eliminato quando il turno termina
* In [modalità non interattiva](/docs/it/headless) e sessioni Agent SDK non c'è confine di turno, quindi un deny viene riutilizzato per il resto dell'esecuzione
* La modifica della tua modalità di autorizzazione o delle regole elimina tutti i verdetti memorizzati nella cache

Esegui `claude auto-mode defaults` per vedere gli elenchi di regole completi. Se le azioni di routine vengono bloccate, un amministratore può aggiungere repo, bucket e servizi attendibili tramite l'impostazione `autoMode.environment`: vedi [Configura modalità auto](/docs/it/auto-mode-config).

Pushing al tuo ramo di lavoro, facendo un push di routine al ramo predefinito del repository, e creando una pull request che corrisponde alla tua richiesta vengono tutti eseguiti senza un prompt. Il classificatore blocca un push solo quando comporta rischio, come un force push o contenuto che aggira una revisione che hai impostato. Per richiedere un checkpoint umano prima di queste azioni mentre rimani in modalità auto, aggiungi regole `permissions.ask`: vedi [Confini comuni](/docs/it/auto-mode-config#common-boundaries).

<h3 id="boundaries-you-state-in-conversation">
  Confini che dichiari nella conversazione
</h3>

Il classificatore tratta i confini che dichiari nella conversazione come un segnale di blocco. Se dici a Claude "non pushare" o "aspetta che io riveda prima di deployare", il classificatore blocca le azioni corrispondenti anche quando le regole predefinite le consentirebbero. Un confine rimane in vigore fino a quando non lo sollevi in un messaggio successivo. Il giudizio proprio di Claude che una condizione era soddisfatta non lo solleva.

I confini non vengono archiviati come regole. Il classificatore li rilegge dalla trascrizione su ogni controllo, quindi un confine può andare perso se la [compattazione del contesto](/docs/it/costs#reduce-token-usage) rimuove il messaggio che lo ha dichiarato. Per una garanzia dura, aggiungi una [regola deny](/docs/it/permissions#permission-rule-syntax) invece.

<h3 id="when-auto-mode-falls-back">
  Quando la modalità auto passa ai prompt manuali
</h3>

Ogni azione negata mostra una notifica e appare in `/permissions` sotto la scheda Recently denied, dove puoi premere `r` per ritentarla con un'approvazione manuale.

Se il classificatore blocca un'azione 3 volte di seguito o 20 volte in totale, la modalità auto si mette in pausa e Claude Code riprende a chiedere. L'approvazione dell'azione richiesta riprende la modalità auto. Questi soglie non sono configurabili. Qualsiasi azione consentita ripristina il contatore consecutivo, mentre il contatore totale persiste per la sessione e si ripristina solo quando il suo limite attiva un fallback.

In [modalità non interattiva](/docs/it/headless) con il flag `-p`, i blocchi ripetuti interrompono la sessione poiché non c'è un utente a cui chiedere.

I blocchi ripetuti di solito significano che il classificatore manca di contesto sulla tua infrastruttura. Usa `/feedback` per segnalare falsi positivi, o fai in modo che un amministratore [configuri l'infrastruttura attendibile](/docs/it/auto-mode-config).

<AccordionGroup>
  <Accordion title="Come il classificatore valuta le azioni">
    Ogni azione passa attraverso un ordine di decisione fisso. Il primo passaggio corrispondente vince:

    1. Le azioni che corrispondono alle tue [regole allow, ask, o deny](/docs/it/permissions#manage-permissions) si risolvono immediatamente. Le scritture su [percorsi protetti](#protected-paths) vengono instradate al classificatore anche quando una regola allow corrisponde. Gli strumenti connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) ti chiedono direttamente anche quando una regola allow corrisponde. Le regole ask con ambito di contenuto ricadono in un prompt di autorizzazione
    2. Le azioni di sola lettura e le modifiche di file nella tua directory di lavoro vengono auto-approvate, tranne le scritture su [percorsi protetti](#protected-paths)
    3. Tutto il resto va al classificatore. Uno strumento connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) salta il classificatore e ti chiede direttamente, quindi un'approvazione richiesta dall'organizzazione non viene mai auto-approvata. A partire da v2.1.199, uno strumento MCP contrassegnato con [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) salta il classificatore e ti chiede direttamente, quindi un passaggio di consenso non viene mai auto-approvato per conto dell'autore dello strumento
    4. Se il classificatore blocca, Claude riceve il motivo e prova un'alternativa

    Entrando in modalità auto, le regole allow ampie che concedono l'esecuzione arbitraria di codice vengono eliminate:

    * Blanket `Bash(*)` o `PowerShell(*)`
    * Interpreti con wildcard come `Bash(python*)`
    * Comandi di esecuzione del gestore di pacchetti
    * Regole allow `Agent`

    Le regole strette come `Bash(npm test)` vengono trasportate. Le regole eliminate vengono ripristinate quando esci dalla modalità auto.

    Il classificatore vede i messaggi dell'utente, le chiamate di strumenti e il contenuto di CLAUDE.md. I risultati degli strumenti vengono rimossi, quindi il contenuto ostile in un file o pagina web non può manipolarlo direttamente. Una sonda separata lato server scansiona i risultati degli strumenti in arrivo e contrassegna il contenuto sospetto prima che Claude lo legga. Per ulteriori informazioni su come questi strati lavorano insieme, vedi l'[annuncio della modalità auto](https://claude.com/blog/auto-mode) e il [deep dive di ingegneria](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Come la modalità auto gestisce i subagent">
    Il classificatore controlla il lavoro dei [subagent](/docs/it/sub-agents) in tre punti:

    1. Prima che un subagent inizi, la descrizione del compito delegato viene valutata, quindi un compito che sembra pericoloso viene bloccato al momento dello spawn.
    2. Mentre il subagent viene eseguito, ognuna delle sue azioni passa attraverso il classificatore con le stesse regole della sessione padre, e qualsiasi `permissionMode` nel frontmatter del subagent viene ignorato.
    3. Quando il subagent finisce, il classificatore esamina la sua cronologia di azioni completa; se quel controllo di ritorno contrassegna una preoccupazione, un avviso di sicurezza viene anteposto ai risultati del subagent.

    Il passaggio 1 richiede Claude Code v2.1.178 o successivo. Le versioni precedenti applicavano il classificatore ai passaggi 2 e 3, ma non valutavano la descrizione del compito prima che il subagent iniziasse.
  </Accordion>

  <Accordion title="Costo e latenza">
    Il classificatore viene eseguito su un modello configurato dal server che è indipendente dalla tua selezione `/model`, quindi il cambio di modelli non cambia la disponibilità del classificatore. Le chiamate del classificatore contano verso il tuo utilizzo di token. Ogni controllo invia una porzione della trascrizione più l'azione in sospeso, aggiungendo un round-trip prima dell'esecuzione. Le letture e le modifiche della directory di lavoro al di fuori dei percorsi protetti saltano il classificatore, quindi l'overhead proviene principalmente da comandi shell e operazioni di rete. A partire da v2.1.198, un verdetto di rete sandbox per un host e una porta viene riutilizzato invece di essere ri-classificato su ogni connessione, quindi le connessioni ripetute allo stesso host non aggiungono ciascuna un controllo. [Cosa blocca il classificatore per impostazione predefinita](#what-the-classifier-blocks-by-default) descrive quanto a lungo un allow e un deny durano.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Consenti solo strumenti pre-approvati con la modalità dontAsk
</h2>

Se imposti la modalità `dontAsk`, Claude Code nega automaticamente ogni chiamata di strumento che altrimenti richiederebbe una richiesta. Claude esegue solo le azioni che corrispondono alle tue regole `permissions.allow`, ai [comandi Bash di sola lettura](/docs/it/permissions#read-only-commands) e alle chiamate approvate da un [hook PreToolUse](/docs/it/permissions#extend-permissions-with-hooks). Utilizza questa modalità per pipeline CI o ambienti limitati in cui pre-definisci esattamente cosa Claude può fare; la sessione non attende mai input. La barra di stato mostra `⏵⏵ don't ask on` mentre questa modalità è attiva.

Claude Code nega le chiamate che corrispondono alle tue regole [`ask`](/docs/it/permissions#manage-permissions) esplicite piuttosto che richiedere una conferma. Nega anche lo strumento integrato `AskUserQuestion` e gli strumenti connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), anche se le tue regole di autorizzazione corrispondono. Nega gli strumenti MCP contrassegnati [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) allo stesso modo, perché la loro scheda di approvazione necessita di una risposta che questa modalità non raccoglie mai; questo richiede Claude Code v2.1.199 o successivo.

Le sessioni cloud su [Claude Code on the web](/docs/it/claude-code-on-the-web) ignorano `defaultMode: "dontAsk"`; vedi [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) per i dettagli.

Impostalo all'avvio con il flag:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Ignora tutti i controlli con la modalità bypassPermissions
</h2>

La modalità `bypassPermissions` disabilita i prompt di autorizzazione e i controlli di sicurezza in modo che le chiamate agli strumenti vengano eseguite immediatamente, incluse le scritture su [percorsi protetti](#protected-paths). Prima della v2.1.126, le scritture su percorsi protetti richiedevano comunque un prompt in questa modalità.

Le [regole ask](/docs/it/permissions#manage-permissions) esplicite e gli strumenti connector [che la vostra organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) forzano comunque un prompt in questa modalità. Gli strumenti MCP contrassegnati con [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) richiedono comunque un prompt; questo richiede Claude Code v2.1.199 o successivo.

Le rimozioni che interessano la radice del filesystem o la directory home, come `rm -rf /` e `rm -rf ~`, richiedono comunque un prompt come protezione contro gli errori del modello. La protezione si attiva anche quando il comando contiene sostituzione di comando con `$(...)` o backtick, o sostituzione di processo con `<(...)`, indipendentemente dal fatto che la rimozione si trovi all'interno della sostituzione, come in `echo "$(rm -rf ~)"`, o altrove nello stesso comando. La forma semplice, digitata come comando autonomo, ha richiesto un prompt in questa modalità sin dall'introduzione della protezione; prima della v2.1.208, i comandi contenenti quelle forme non richiedevano un prompt.

<Warning>
  Utilizzare questa modalità solo in ambienti isolati come container, VM o dev container senza accesso a Internet, dove Claude Code non può danneggiare il sistema host.
</Warning>

Non è possibile accedere a `bypassPermissions` da una sessione avviata senza uno dei flag di abilitazione; riavviare con uno di essi per abilitarla:

```bash theme={null}
claude --permission-mode bypassPermissions
```

Il flag `--dangerously-skip-permissions` è equivalente.

Su Linux e macOS, Claude Code rifiuta di avviarsi in questa modalità quando viene eseguito come root o sotto `sudo`:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

Il controllo viene saltato automaticamente all'interno di una sandbox riconosciuta. Per eseguire in modo autonomo in un container, utilizzare la configurazione [dev container](/docs/it/devcontainer), che esegue Claude Code come utente non root.

[Claude Code sul web](/docs/it/claude-code-on-the-web) non rispetta `defaultMode: "bypassPermissions"` o `"dontAsk"` dai file di impostazioni, quindi le impostazioni archiviate di un repository non possono avviare una sessione cloud in modalità bypass-permissions. L'impostazione viene ignorata silenziosamente e la sessione si avvia nella modalità mostrata nel menu a discesa della modalità. Vedere [Cambia modalità di autorizzazione](#switch-permission-modes) per le modalità offerte dalle sessioni cloud.

<Warning>
  `bypassPermissions` non offre protezione contro l'iniezione di prompt o azioni indesiderate. Per i controlli di sicurezza di background con molti meno prompt di autorizzazione, utilizzare invece la [modalità auto](#eliminate-prompts-with-auto-mode). Gli amministratori possono bloccare questa modalità impostando `permissions.disableBypassPermissionsMode` su `"disable"` nelle [impostazioni gestite](/docs/it/permissions#managed-settings).
</Warning>

<h2 id="protected-paths">
  Percorsi protetti
</h2>

Le scritture in un piccolo insieme di percorsi non vengono mai approvate automaticamente, in ogni modalità tranne `bypassPermissions`. Ciò previene la corruzione accidentale dello stato del repository e della configurazione di Claude.

| Modalità                         | Scritture su percorsi protetti |
| :------------------------------- | :----------------------------- |
| `default`, `acceptEdits`, `plan` | Richiesta di conferma          |
| `auto`                           | Indirizzato al classificatore  |
| `dontAsk`                        | Negato                         |
| `bypassPermissions`              | Consentito                     |

Le regole [`permissions.allow`](/docs/it/permissions#manage-permissions) nei file di impostazioni non pre-approvano le scritture su percorsi protetti. Il controllo di sicurezza viene eseguito prima che Claude Code valuti le regole allow dalle impostazioni, quindi una voce come `Edit(.claude/**)` in `~/.claude/settings.json` o `.claude/settings.json` non modifica il risultato per modalità nella tabella precedente. Nelle modalità che richiedono conferma, il prompt per una scrittura `.claude/` offre **Sì, e consenti a Claude di modificare le proprie impostazioni per questa sessione**, che approva le successive scritture `.claude/` in quella sessione senza richiedere di nuovo la conferma.

Directory protette:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, ad eccezione di `.claude/worktrees` dove Claude memorizza i propri git worktrees

File protetti:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="see-also">
  Vedi anche
</h2>

* [Permissions](/docs/it/permissions): regole allow, ask e deny; politiche gestite
* [Configure auto mode](/docs/it/auto-mode-config): comunica al classificatore quale infrastruttura la tua organizzazione ritiene affidabile
* [Hooks](/docs/it/hooks): logica di autorizzazione personalizzata tramite hook `PreToolUse` e `PermissionRequest`
* [Ultraplan](/docs/it/ultraplan): esegui la modalità plan in una sessione Claude Code sul web con revisione basata su browser
* [Security](/docs/it/security): misure di sicurezza e best practice
* [Sandboxing](/docs/it/sandboxing): isolamento del filesystem e della rete per i comandi Bash
* [Non-interactive mode](/docs/it/headless): esegui Claude Code con il flag `-p`
