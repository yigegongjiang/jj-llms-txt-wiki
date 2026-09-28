> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usa Claude Code nel cloud

> Esegui sessioni Claude Code nel cloud dal tuo browser, telefono, app desktop o terminale, spostale con --cloud e --teleport, e correggi automaticamente le pull request.

<Note>
  Le sessioni cloud sono disponibili sui piani Pro, Max e Team, e per gli utenti Enterprise con posti premium o posti Chat + Claude Code.
</Note>

Una sessione cloud è una sessione Claude Code che viene eseguita su infrastruttura cloud invece che sulla tua macchina. Per impostazione predefinita viene eseguita su infrastruttura gestita da Anthropic, oppure sull'[ambiente self-hosted](/docs/it/self-hosted-environments) della tua organizzazione quando instradato lì. La sessione continua a essere eseguita dopo che chiudi il tuo laptop, e puoi controllarla o dirigerla da qualsiasi dispositivo.

Puoi avviare una sessione cloud da una qualsiasi di queste superfici:

* **Browser**: [claude.ai/code](https://claude.ai/code), chiamato anche Claude Code sul web
* **Mobile**: la scheda **Code** nell'[app Claude](/docs/it/mobile)
* **App desktop**: seleziona **Cloud** invece di **Local** quando [avvii una sessione](/docs/it/desktop#run-long-running-tasks-in-the-cloud)
* **Terminale**: [`claude --cloud`](#from-terminal-to-cloud)
* **Routine**: [esecuzioni pianificate e attivate](/docs/it/routines) vengono eseguite ciascuna come sessione cloud

Per fare in modo che Claude avvii e tenga traccia di molte sessioni cloud per un corpo di lavoro, usa un [progetto](/docs/it/claude-projects). Una sessione nel tuo terminale, nel tuo IDE, o nell'app Desktop con **Local** selezionato viene eseguita sulla tua macchina. Per dirigere una di quelle sessioni locali dal tuo telefono o browser, usa [Remote Control](/docs/it/remote-control).

<Tip>
  Nuovo a sessioni cloud? Inizia con [Guida introduttiva](/docs/it/web-quickstart) per connettere il tuo account GitHub e inviare il tuo primo compito.
</Tip>

Questa pagina copre:

* [Ambienti cloud](#cloud-environments): dove vengono eseguite le sessioni, e dove configurare questo
* [Opzioni di autenticazione GitHub](#github-authentication-options): due modi per connettere GitHub
* [Sposta attività tra terminale e cloud](#move-tasks-between-terminal-and-cloud) con `--cloud` e `--teleport`
* [Lavora con le sessioni](#work-with-sessions): modalità di autorizzazione, revisione, condivisione, archiviazione, eliminazione
* [Correzione automatica delle pull request](#auto-fix-pull-requests): rispondi automaticamente ai fallimenti CI e ai commenti di revisione
* [Sicurezza e isolamento](#security-and-isolation): come le sessioni sono isolate
* [Limitazioni](#limitations): limiti di velocità e restrizioni della piattaforma

<h2 id="cloud-environments">
  Ambienti cloud
</h2>

Ogni sessione cloud viene eseguita in un [ambiente cloud](/docs/it/cloud-environments), la configurazione salvata che controlla l'accesso alla rete, le variabili di ambiente e gli script di configurazione. Se non hai ancora un ambiente, l'onboarding configura un ambiente **Default** con accesso alla rete [**Trusted**](/docs/it/cloud-environments#access-levels), creandolo per te o chiedendoti di crearlo. Vedi [L'ambiente Default](/docs/it/cloud-environments#the-default-environment) per sapere quale di questi accade sul tuo piano e come le sessioni scelgono un ambiente quando ne hai più di uno.

Gli stessi ambienti si applicano ovunque tu avvii una sessione cloud: il browser, il terminale, [Claude Tag](https://claude.com/docs/claude-tag/overview), [routine](/docs/it/routines) e le app mobile e Desktop. Le sessioni dei canali Claude Tag utilizzano solo ambienti a livello di organizzazione, sia [ambienti condivisi](/docs/it/cloud-environments#organization-shared-environments) che [ambienti self-hosted](/docs/it/self-hosted-environments).

Vedi [Configura ambienti cloud](/docs/it/cloud-environments) per modificare cosa consente un ambiente, impostare variabili o aggiungere uno script di configurazione, e [Strumenti installati](/docs/it/cloud-environments#installed-tools) per quello che le sessioni includono senza alcuna configurazione.

<h2 id="github-authentication-options">
  Opzioni di autenticazione GitHub
</h2>

Le sessioni cloud hanno bisogno di accesso ai tuoi repository GitHub per clonare il codice e inviare i rami. Puoi concedere l'accesso in due modi:

| Metodo           | Come ti connetti                                                                                     | Repository che le sessioni possono raggiungere                                                                         | Migliore per                                                                    |
| :--------------- | :--------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------ |
| **GitHub App**   | Autorizza l'app Claude GitHub durante l'[onboarding web](/docs/it/web-quickstart)                         | Qualsiasi repository pubblico e repository privati su cui è installata l'app Claude GitHub                             | Onboarding del browser; team che desiderano [Auto-fix](#auto-fix-pull-requests) |
| **`/web-setup`** | Esegui `/web-setup` nel tuo terminale per inviare il tuo token CLI `gh` locale al tuo account Claude | Qualsiasi repository a cui il tuo token `gh` può accedere, indipendentemente dal fatto che l'app sia installata o meno | Sviluppatori individuali che già usano `gh`                                     |

L'installazione dell'app Claude GitHub su un repository abilita anche [Auto-fix](#auto-fix-pull-requests) per le pull request in esso.

I thread in un [progetto](/docs/it/claude-projects) hanno bisogno che l'app sia installata su ogni repository che clonano, indipendentemente dal metodo con cui ti sei connesso. Vedi [Configura l'accesso a GitHub](/docs/it/claude-projects#set-up-github-access).

Per come `/schedule` verifica l'accesso al repository prima di creare una routine, vedi [Repository e autorizzazioni di ramo](/docs/it/routines#repositories-and-branch-permissions). Vedi [Connetti dal tuo terminale](/docs/it/web-quickstart#connect-from-your-terminal) per la procedura dettagliata di `/web-setup`, incluso ciò che `/web-setup` memorizza e come rimuoverlo.

La configurazione rapida del web è un'impostazione dell'organizzazione che consente ai membri di connettere GitHub con `/web-setup`, salta il prompt di installazione dell'app Claude GitHub durante l'onboarding del browser e fa sì che l'onboarding del browser crei l'[ambiente **Default**](/docs/it/cloud-environments#the-default-environment) per loro invece di mostrare il modulo dell'ambiente. Nei piani Team ed Enterprise è disabilitato per impostazione predefinita, il che nasconde `/web-setup`. Un [Proprietario](/docs/it/server-managed-settings#access-control) lo attiva con l'interruttore **Quick web setup** su [**Impostazioni di amministrazione > Claude Code**](https://claude.ai/admin-settings/claude-code).

<Note>
  Le organizzazioni con [Zero Data Retention](/docs/it/zero-data-retention) abilitato non possono utilizzare `/web-setup` o altre funzionalità di sessione cloud.
</Note>

<h2 id="move-tasks-between-terminal-and-cloud">
  Sposta attività tra terminale e cloud
</h2>

Questi flussi di lavoro richiedono la [Claude Code CLI](/docs/it/quickstart) connessa allo stesso account claude.ai. Puoi avviare nuove sessioni cloud dal tuo terminale o estrarre sessioni cloud nel tuo terminale per continuare localmente. Le sessioni cloud persistono anche se chiudi il tuo laptop e puoi monitorarle da qualsiasi luogo inclusa l'app mobile Claude.

<Note>
  Dalla CLI, l'handoff della sessione è unidirezionale: puoi estrarre sessioni cloud nel tuo terminale con `--teleport`, ma non puoi inviare una sessione terminale esistente al cloud. Il flag `--cloud` con una descrizione di compito crea una nuova sessione cloud per il tuo repository attuale; con `-p` e un ID di sessione o URL claude.ai/code invece [accoda un messaggio in quella sessione esistente](/docs/it/claude-code-on-the-web#send-follow-ups-from-the-cli). L'[app Desktop](/docs/it/desktop#continue-in-another-surface) fornisce un menu **Continua in** che può inviare una sessione locale al cloud.
</Note>

<h3 id="from-terminal-to-cloud">
  Dal terminale al cloud
</h3>

Avvia una sessione cloud dalla riga di comando con il flag `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Questo crea una nuova sessione cloud su claude.ai. La VM cloud clona il remoto GitHub della tua directory attuale al tuo ramo attuale, non il tuo checkout locale, quindi esegui il push prima se hai commit locali. Vedi [Invia repository locali senza GitHub](#send-local-repositories-without-github) per i casi in cui Claude Code carica il tuo repository locale invece di clonare.

`--cloud` funziona con un singolo repository alla volta. L'attività viene eseguita nel cloud mentre continui a lavorare localmente. L'ortografia più vecchia `--remote` funziona ancora come alias deprecato per `--cloud`.

Mentre il contenitore cloud si avvia, la CLI mostra un elenco di controllo in tempo reale dei passaggi di configurazione, come la clonazione del repository e l'esecuzione dello [script di configurazione](/docs/it/cloud-environments#setup-scripts). Accoda i messaggi che digiti durante il provisioning e li invia una volta che la sessione è pronta.

<Note>
  `--cloud` crea sessioni cloud. `--remote-control` non è correlato: ti consente di monitorare e controllare una sessione CLI locale da claude.ai o dall'app Claude. Vedi [Remote Control](/docs/it/remote-control).
</Note>

Apri la sessione su claude.ai o l'app mobile Claude per controllare l'avanzamento o interagire direttamente. Da lì puoi guidare Claude, fornire feedback o rispondere a domande proprio come in qualsiasi altra conversazione.

Se Claude pone una domanda e la sessione rimane inattiva, puoi comunque rispondere quando torni, fino a [scadenza dell'ambiente](#environment-expired), e la sessione continua dalla tua risposta.

<h4 id="tips-for-cloud-tasks">
  Suggerimenti per attività cloud
</h4>

**Pianifica localmente, esegui nel cloud**: per attività complesse, avvia Claude in plan mode per collaborare sull'approccio, quindi invia il lavoro al cloud:

```bash theme={null}
claude --permission-mode plan
```

In plan mode, Claude legge i file, esegue comandi per esplorare e propone un piano senza modificare il codice sorgente. Una volta soddisfatto, salva il piano nel repository, esegui il commit e il push in modo che la VM cloud possa clonarlo. Quindi avvia una sessione cloud per l'esecuzione autonoma:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

**Esegui attività in parallelo**: ogni comando `--cloud` crea la sua propria sessione cloud che viene eseguita indipendentemente. Puoi avviare più attività e verranno tutte eseguite simultaneamente in sessioni separate:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Quando una sessione si completa, puoi creare una PR da claude.ai/code o [teletrasportare](#from-cloud-to-terminal) la sessione nel tuo terminale per continuare a lavorare.

<h4 id="send-local-repositories-without-github">
  Invia repository locali senza GitHub
</h4>

Quando esegui `claude --cloud` da un repository che non ha un remoto git, o da un repository github.com su cui l'app GitHub di Claude non è installata, Claude Code raggruppa il tuo repository locale e lo carica direttamente nella sessione cloud. Questo si applica anche se hai connesso GitHub con `/web-setup`. Il bundle include la tua cronologia completa del repository su tutti i rami, più eventuali modifiche non sottoposte a commit ai file tracciati.

Su macOS, Linux e WSL, Claude Code lascia fuori dall'upload le modifiche non sottoposte a commit ai file denominati come credenziali o chiavi e nomina i file che ha lasciato fuori. Questo copre i file `.env`, i file Terraform `*.tfvars` e i file di chiave come `id_rsa` e `*.pem`. La sessione inizia con la versione sottoposta a commit di ognuno, o senza il file se nessuno è sottoposto a commit. In un worktree collegato, submodulo o layout simile, Claude Code carica queste modifiche con il resto e nomina i file che carica.

Per caricare un bundle anche quando Claude Code altrimenti clonerebbe dal remoto, imposta `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

I repository raggruppati devono soddisfare questi limiti:

* La directory deve essere un repository git con almeno un commit
* Il repository raggruppato deve essere inferiore a 100 MB. I repository più grandi ricadono nel raggruppamento solo del ramo attuale, quindi in uno snapshot squashed singolo dell'albero di lavoro e falliscono solo se lo snapshot è ancora troppo grande
* I file non tracciati non sono inclusi; esegui `git add` sui file che desideri che la sessione cloud veda
* Le sessioni create da un bundle possono eseguire il push di nuovo a un remoto GitHub solo quando la tua [connessione GitHub](#github-authentication-options) ha accesso push a quel repository

<h3 id="send-follow-ups-from-the-cli">
  Invia follow-up dalla CLI
</h3>

Una volta che una sessione cloud è in esecuzione, ovunque venga eseguita, inviala un messaggio di follow-up dalla CLI `claude` su qualsiasi macchina dove sei connesso con `claude auth login`. La CLI si autentica con le credenziali del tuo account Anthropic e non invia alcuno stato di sessione locale, quindi il comando non ha bisogno di essere eseguito dalla macchina che ha avviato la sessione, ed è lo stesso in ogni shell, incluso PowerShell.

Il comando pubblica un messaggio e esce:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

La CLI accoda il messaggio nella sessione e esce senza attendere una risposta. Usalo per guidare una sessione di lunga durata, accodare il passaggio successivo mentre quello attuale sta ancora finendo, o inviare follow-up da uno [script CI](/docs/it/self-hosted-environments-testing#run-the-test-loop). Puoi anche inviare il messaggio su stdin invece di passarlo come argomento: `echo "your message" | claude -p --cloud <session-id>`.

Per `<session-id>`, passa l'ID nudo, come `session_...` o `cse_...`, o l'URL `claude.ai/code/<id>` della sessione, con o senza lo schema o la stringa di query. Trova l'ID nel tuo elenco di sessioni su claude.ai/code.

<Note>
  `--cloud` richiede un account Anthropic. Non è disponibile quando Claude Code è configurato per Amazon Bedrock, Google Cloud's Agent Platform o un altro provider di terze parti. Un [gateway LLM](/docs/it/llm-gateway) configurato solo tramite `ANTHROPIC_BASE_URL` non conta come provider di terze parti per questo controllo, ma devi comunque accedere con `claude auth login`. La politica `allow_remote_sessions` della tua organizzazione deve anche essere abilitata. Un Proprietario può attivarla nelle impostazioni di amministrazione di Claude Code su claude.ai/admin-settings/claude-code.
</Note>

<h4 id="output-and-errors">
  Output e errori
</h4>

Al successo, il comando stampa l'ID della sessione e un link per visualizzare la sessione:

```
Sent to cloud session.
Session ID: session_01DiUkqY2kzbUbDmW1w96rfi
View: https://claude.ai/code/session_01DiUkqY2kzbUbDmW1w96rfi?from=cli&m=0
```

Passa `--output-format json` per un risultato leggibile da macchina: `{ok, session_id, url}` al successo, o `{ok: false, session_id, error}` quando l'invio fallisce, ad esempio quando la sessione è mancante o archiviata. Gli errori di configurazione, come un provider non supportato o una politica organizzativa disabilitata, vengono stampati su stderr senza JSON. `--output-format stream-json` non è supportato con `--cloud <session-id>`.

La CLI antepone gli errori con `Error: `. Un'entrega fallita è avvolta come `failed to send message to cloud session <id>: <reason>`.

| Messaggio                                                                                                                   | Cosa significa                                                                                                                                                                                                                                                                                                                                                    |
| --------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Cloud sessions aren't available with <provider>. They run on Anthropic's infrastructure and require an Anthropic account.` | Claude Code è configurato per un provider di terze parti. Il messaggio nomina il provider con l'etichetta che la tua configurazione utilizza, come `Amazon Bedrock` o `Google Vertex AI`. Rimuovi la configurazione di quel provider, ad esempio annullando l'impostazione di `CLAUDE_CODE_USE_BEDROCK`, e accedi con un account Anthropic (`claude auth login`). |
| `Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.`                | La politica organizzativa `allow_remote_sessions` è disattivata.                                                                                                                                                                                                                                                                                                  |
| `Couldn't verify your organization's policy for cloud sessions. Check your network connection and try again.`               | Claude Code non ha potuto recuperare la politica della tua organizzazione, quindi rifiuta l'invio piuttosto che assumere che le sessioni cloud siano consentite. Controlla la tua connessione di rete e riprova.                                                                                                                                                  |
| `Attaching to an existing cloud session is not enabled for your account.`                                                   | Hai eseguito `--cloud <session-id>` senza `-p`. Invia il messaggio con `claude -p "your message" --cloud <session-id>`.                                                                                                                                                                                                                                           |
| `Session not found: <id>`                                                                                                   | L'ID o l'URL non corrisponde a una sessione a cui puoi accedere. Controllalo rispetto all'URL claude.ai/code della sessione.                                                                                                                                                                                                                                      |
| `cloud session <id> is archived and cannot accept new messages`                                                             | La sessione è stata archiviata. Avvia una nuova sessione invece.                                                                                                                                                                                                                                                                                                  |

<h3 id="from-cloud-to-terminal">
  Dal cloud al terminale
</h3>

Estrai una sessione cloud nel tuo terminale usando uno di questi:

* **Usando `--teleport`**: dalla riga di comando, esegui `claude --teleport` per un selettore di sessione interattivo, o `claude --teleport <session-id>` per riprendere una sessione specifica direttamente. Se hai modifiche non sottoposte a commit, ti verrà chiesto di archiviarle prima.
* **Usando `/teleport`**: all'interno di una sessione CLI esistente, esegui `/teleport` o `/tp` per aprire lo stesso selettore di sessione senza riavviare Claude Code.
* **Da `/tasks`**: esegui `/tasks` per vedere le tue sessioni in background, quindi premi `t` per teletrasportarti in una.
* **Da claude.ai/code**: seleziona **Open in > Terminal** dal menu della sessione per copiare un comando che puoi incollare nel tuo terminale.
* **Dall'interno della sessione cloud**: digita `/teleport` e Claude Code risponde con il comando esatto `claude --teleport <session-id>` per quella sessione, pronto per essere eseguito da un checkout del repository. Richiede Claude Code v2.1.223 o successivo nell'ambiente della sessione.

Quando teletrasporti una sessione, Claude verifica che sei nel repository corretto, recupera e controlla il ramo dalla sessione cloud e carica la cronologia completa della conversazione nel tuo terminale. Il terminale ottiene la sua propria copia della sessione: il nuovo lavoro lì rimane locale e non appare nella sessione cloud su claude.ai o nell'app mobile Claude. Per continuare a guidare dal tuo telefono dopo il teletrasporto, avvia [`/remote-control`](/docs/it/remote-control) nella sessione locale.

`--teleport` è distinto da `--resume`. `--resume` riapre una conversazione dalla cronologia locale di questa macchina e non elenca le sessioni cloud; `--teleport` estrae una sessione cloud e il suo ramo.

<h4 id="teleport-requirements">
  Requisiti per il teletrasporto
</h4>

Il teletrasporto verifica questi requisiti prima di riprendere una sessione. Se un requisito non è soddisfatto, vedrai un errore o ti verrà chiesto di risolvere il problema.

| Requisito           | Dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Stato git pulito    | La tua directory di lavoro non deve avere modifiche non sottoposte a commit. Il teletrasporto ti chiede di archiviare le modifiche se necessario.                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Repository corretto | Devi eseguire `--teleport` da un checkout dello stesso repository, non da un fork. Se lo esegui da un checkout di un repository diverso, Claude Code mostra un errore che nomina sia il repository della sessione che il repository del tuo checkout. Prima della v2.1.219, l'errore non nominava il repository del tuo checkout. Se Claude Code non riesce ad analizzare il tuo remoto in un nome host, ad esempio un alias host SSH come `git@work:owner/repo.git`, ti chiede di confermare e accetta il checkout quando il proprietario del remoto e il nome del repository corrispondono al repository della sessione. |
| Ramo disponibile    | Il ramo dalla sessione cloud deve essere stato inviato al remoto. Il teletrasporto lo recupera e lo controlla automaticamente.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Stesso account      | Devi essere autenticato allo stesso account claude.ai utilizzato nella sessione cloud.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<h4 id="teleport-is-unavailable">
  `--teleport` non è disponibile
</h4>

Il teletrasporto richiede l'autenticazione dell'abbonamento claude.ai. Se sei autenticato tramite chiave API, esegui `/login` per accedere con il tuo account claude.ai. Se il messaggio di errore nomina il tuo provider, le sessioni cloud non sono disponibili tramite provider di terze parti; vedi la [tabella degli errori](#output-and-errors). Se sei già connesso tramite claude.ai e `--teleport` non è ancora disponibile, la tua organizzazione potrebbe aver disabilitato le sessioni cloud.

<h2 id="work-with-sessions">
  Lavorare con le sessioni
</h2>

Le sessioni appaiono nella barra laterale su claude.ai/code. Da lì è possibile rivedere le modifiche, condividere con i colleghi, archiviare il lavoro completato o eliminare le sessioni in modo permanente.

<h3 id="take-back-a-queued-message">
  Riprendere un messaggio in coda
</h3>

Se inviate un messaggio mentre Claude sta lavorando, il messaggio rimane in coda finché Claude non lo legge. Per riprendere un messaggio in coda, fate clic sulla ✕ su di esso. Il testo ritorna nella casella dei messaggi in modo che possiate modificarlo o inviare qualcos'altro.

Se Claude ha già letto il messaggio, rimane nella conversazione.

<h3 id="manage-context">
  Gestire il contesto
</h3>

Le sessioni cloud supportano [comandi integrati](/docs/it/commands) che producono output di testo. I comandi che vengono eseguiti solo nell'interfaccia del terminale, come `/plugin` o `/resume`, non sono disponibili. I comandi che aprono un selettore o un pannello nel terminale si comportano diversamente nelle sessioni cloud:

* **`/model`, `/effort`, `/color` e `/rename`**: passare il valore come argomento, ad esempio `/model sonnet`, invece di aprire il selettore del terminale o il cursore. Le forme di argomento richiedono Claude Code v2.1.205 o successivo nell'ambiente della sessione e seguono le [note di disponibilità](/docs/it/commands#all-commands) di ciascun comando.
* **`/fast`**: attiva/disattiva la [modalità veloce](/docs/it/fast-mode#use-fast-mode-in-cloud-sessions) per la sessione quando la modalità veloce è [disponibile nel vostro account](/docs/it/fast-mode#requirements). Richiede Claude Code v2.1.271 o successivo nell'ambiente della sessione.
* **`/config`**: nel vostro browser su claude.ai/code, apre la sezione Claude Code delle impostazioni invece di impostare un valore, e il testo dopo il comando, incluso `key=value`, viene ignorato. Per modificare un'impostazione per una sessione cloud, impostare una [variabile di ambiente](/docs/it/cloud-environments#set-environment-variables) nell'ambiente, oppure in una sessione con un repository, eseguire il commit della chiave nel file `.claude/settings.json` di quel repository. [Impostazioni nelle sessioni cloud](/docs/it/settings#settings-in-cloud-sessions) elenca cosa legge ogni sessione.

Per la gestione del contesto in particolare:

| Comando    | Funziona nelle sessioni cloud | Note                                                                                                                        |
| :--------- | :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `/compact` | Sì                            | Riassume la conversazione per liberare contesto. Accetta istruzioni di focus opzionali come `/compact keep the test output` |
| `/context` | Sì                            | Mostra cosa è attualmente nella finestra di contesto                                                                        |
| `/clear`   | No                            | Avviare una nuova sessione dalla barra laterale                                                                             |

La compattazione automatica viene eseguita automaticamente quando la finestra di contesto si avvicina alla capacità. Le sessioni cloud impostano [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/it/env-vars) da sole, quindi la compattazione si attiva a metà della [finestra di auto-compattazione](/docs/it/model-config#set-the-auto-compact-window) piuttosto che quando la finestra si riempie. Questo valore sostituisce uno che si aggiunge nelle [variabili di ambiente](/docs/it/cloud-environments#set-environment-variables), quindi aggiungere la variabile lì non cambia quando si attiva la compattazione.

Per modificare la finestra di auto-compattazione, impostare [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/it/env-vars) nelle variabili di ambiente, oppure eseguire [`/autocompact`](/docs/it/commands#all-commands) con un conteggio di token in una sessione in cui la variabile non è impostata.

I [Subagenti](/docs/it/sub-agents) funzionano allo stesso modo di quelli locali. Claude può generarli con lo strumento Agent per delegare la ricerca o il lavoro parallelo a una finestra di contesto separata, mantenendo la conversazione principale più leggera. I subagenti definiti nella directory `.claude/agents/` del repository vengono rilevati automaticamente.

I [team di agenti](/docs/it/agent-teams) sono disabilitati per impostazione predefinita ma possono essere abilitati aggiungendo `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` alle [variabili di ambiente](/docs/it/cloud-environments#set-environment-variables).

<h3 id="permission-modes-in-cloud-sessions">
  Modalità di autorizzazione nelle sessioni cloud
</h3>

Si sceglie la [modalità di autorizzazione](/docs/it/permission-modes) di una sessione cloud dal [menu a discesa della modalità](/docs/it/permission-modes#switch-permission-modes), sia quando si crea l'attività che mentre la sessione è in esecuzione. Quando si riapre una sessione il cui [ambiente ospitato da Anthropic è scaduto](#environment-expired), o si invia un messaggio a una sessione che un runner self-hosted [ha rilasciato mentre era inattivo](/docs/it/self-hosted-environments-reference#runner-cli-flags), Claude Code riprende la sessione nella modalità di autorizzazione in cui si trovava.

<h3 id="review-changes">
  Rivedere le modifiche
</h3>

Ogni sessione mostra un indicatore di diff con righe aggiunte e rimosse, come `+42 -18`. Selezionarlo per aprire la visualizzazione diff, lasciare commenti in linea su righe specifiche e inviarli a Claude con il messaggio successivo.

La visualizzazione diff confronta le modifiche della sessione rispetto al suo ramo base per impostazione predefinita. Per confrontare con qualsiasi altro ramo nel repository, selezionare **Compare against** e sceglierne uno.

Claude Code calcola questi diff, inclusi i diff per file mostrati mentre Claude modifica, dal contenuto grezzo del blob git, quindi i driver diff e i filtri `textconv` configurati nel repository non si applicano. Per un file in un repository che non è uno dei checkout della sessione stessa, come uno clonato all'interno dell'area di lavoro durante la sessione, il diff per file mostra la modifica di Claude stessa piuttosto che un confronto git.

Vedere [Rivedere e iterare](/docs/it/web-quickstart#review-and-iterate) per la procedura dettagliata completa inclusa la creazione di PR. Per fare in modo che Claude monitori automaticamente la PR per errori CI e commenti di revisione, vedere [Correzione automatica delle pull request](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Condividere sessioni
</h3>

Per condividere una sessione, attivare/disattivare la sua visibilità in base ai tipi di account di seguito. Dopo di che, condividere il collegamento della sessione così com'è. I destinatari vedono lo stato più recente quando aprono il collegamento, ma la loro visualizzazione non si aggiorna in tempo reale.

<h4 id="share-from-an-enterprise-or-team-account">
  Condividere da un account Enterprise o Team
</h4>

Per gli account Enterprise e Team, le due opzioni di visibilità sono **Private** e **Team**. La visibilità Team rende la sessione visibile agli altri membri dell'organizzazione claude.ai. Le sessioni di [Claude in Slack](/docs/it/slack) vengono condivise automaticamente con visibilità Team.

La verifica dell'accesso al repository è abilitata per impostazione predefinita, in base all'account GitHub connesso all'account del destinatario. Il nome visualizzato dell'account è visibile a tutti i destinatari con accesso.

<h4 id="share-from-a-max-or-pro-account">
  Condividere da un account Max o Pro
</h4>

Per gli account Max e Pro, le due opzioni di visibilità sono **Private** e **Public**. La visibilità Public rende la sessione visibile a qualsiasi utente connesso a claude.ai.

Controllare la sessione per contenuti sensibili prima di condividere. Le sessioni possono contenere codice e credenziali da repository GitHub privati. La verifica dell'accesso al repository non è abilitata per impostazione predefinita.

Per richiedere ai destinatari di avere accesso al repository, o per nascondere il nome dalle sessioni condivise, andare a [**Settings > Claude Code > Sharing settings**](https://claude.ai/settings/claude-code).

<h3 id="archive-sessions">
  Archiviare sessioni
</h3>

È possibile archiviare le sessioni per mantenere organizzato l'elenco delle sessioni. Le sessioni archiviate sono nascoste dall'elenco di sessioni predefinito ma possono essere visualizzate filtrando per sessioni archiviate.

Per archiviare una sessione, passare il mouse sulla sessione nella barra laterale e selezionare l'icona di archivio.

<h3 id="delete-sessions">
  Eliminare sessioni
</h3>

L'eliminazione di una sessione rimuove in modo permanente la sessione e i suoi dati. Questa azione non può essere annullata. È possibile eliminare una sessione in due modi:

* **Dalla barra laterale**: filtrare per sessioni archiviate, quindi passare il mouse sulla sessione che si desidera eliminare e selezionare l'icona di eliminazione
* **Dal menu della sessione**: aprire una sessione, selezionare l'elenco a discesa accanto al titolo della sessione e selezionare **Delete**

Ti verrà chiesto di confermare prima che una sessione venga eliminata.

<h2 id="auto-fix-pull-requests">
  Correzione automatica delle pull request
</h2>

Claude può monitorare una pull request e rispondere automaticamente ai fallimenti CI e ai commenti di revisione. Claude si iscrive all'attività GitHub sulla PR e, quando un controllo fallisce o un revisore lascia un commento, Claude indaga e invia una correzione se una è chiara.

<Note>
  Auto-fix richiede che l'app Claude GitHub sia installata nel tuo repository. Se non l'hai già fatto, installala dalla [pagina dell'app GitHub](https://github.com/apps/claude).
</Note>

Ci sono alcuni modi per attivare auto-fix a seconda da dove proviene la PR e quale dispositivo stai utilizzando:

* **PR create in una sessione cloud**: apri la sessione su claude.ai/code, apri la barra di stato CI e seleziona **Auto-fix**
* **Dal tuo terminale**: esegui [`/autofix-pr`](/docs/it/commands) mentre sei sul ramo della PR. Claude Code rileva la PR aperta con `gh`, genera una sessione cloud e attiva auto-fix in un passaggio
* **Dall'app mobile**: dì a Claude di correggere automaticamente la PR, ad esempio "watch this PR and fix any CI failures or review comments"
* **Qualsiasi PR esistente**: incolla l'URL della PR in una sessione e dì a Claude di correggerla automaticamente

Auto-fix è un interruttore per PR. Per smettere di monitorare, apri la barra di stato CI nella sessione su claude.ai/code e deseleziona l'interruttore **Auto-fix**, oppure dì a Claude di smettere di monitorare la PR.

<h3 id="how-claude-responds-to-pr-activity">
  Come Claude risponde all'attività PR
</h3>

Quando auto-fix è attivo, Claude riceve eventi GitHub per la PR inclusi nuovi commenti di revisione e fallimenti di controllo CI. Per ogni evento, Claude indaga e decide come procedere:

* **Correzioni chiare**: se Claude è sicuro di una correzione e non entra in conflitto con le istruzioni precedenti, Claude apporta la modifica, la invia e spiega cosa è stato fatto nella sessione
* **Richieste ambigue**: se il commento di un revisore potrebbe essere interpretato in più modi o coinvolge qualcosa di architettonicamente significativo, Claude ti chiede prima di agire
* **Eventi duplicati o senza azione**: se un evento è un duplicato o non richiede modifiche, Claude lo annota nella sessione e continua

GitHub non emette un webhook quando il ramo base avanza e crea un conflitto di merge, quindi auto-fix non può reagire ai conflitti da solo. Per risolvere un conflitto, apri la sessione e chiedi a Claude di eseguire il rebase.

Claude potrebbe rispondere ai thread di commenti di revisione su GitHub come parte della loro risoluzione. Queste risposte vengono pubblicate utilizzando il tuo account GitHub, quindi appaiono sotto il tuo nome utente, ma ogni risposta è etichettata come proveniente da Claude Code in modo che i revisori sappiano che è stata scritta dall'agente e non da te direttamente.

<Warning>
  Se il tuo repository utilizza automazione attivata da commenti come Atlantis, Terraform Cloud o GitHub Actions personalizzate che vengono eseguite su eventi `issue_comment`, tieni presente che Claude può rispondere per tuo conto, il che può attivare questi flussi di lavoro. Rivedi l'automazione del tuo repository prima di abilitare auto-fix e considera di disabilitare auto-fix per i repository in cui un commento PR può distribuire infrastruttura o eseguire operazioni privilegiate.
</Warning>

<h2 id="security-and-isolation">
  Sicurezza e isolamento
</h2>

Ogni sessione cloud è separata dalla tua macchina e dalle altre sessioni attraverso diversi livelli:

* **Macchine virtuali isolate**: ogni sessione viene eseguita in una VM isolata gestita da Anthropic. Le sessioni che la tua organizzazione instrada a un [ambiente self-hosted](/docs/it/self-hosted-environments) vengono eseguite sulla tua infrastruttura, dove l'isolamento è responsabilità della tua distribuzione
* <span id="default-allowed-domains" />**Controlli di accesso alla rete**: negli ambienti ospitati da Anthropic, l'accesso alla rete è limitato per impostazione predefinita e può essere disabilitato. Vedi [Accesso alla rete](/docs/it/cloud-environments#network-access) per i livelli di accesso, i [domini consentiti per impostazione predefinita](/docs/it/cloud-environments#default-allowed-domains) e il traffico che non passa attraverso l'allowlist. In un ambiente self-hosted, limiti l'uscita della sessione al tuo confine di rete. Quando viene eseguito con l'accesso alla rete disabilitato, Claude Code può comunque comunicare con l'API Anthropic, che potrebbe consentire ai dati di uscire dalla VM.
* **Protezione delle credenziali**: negli ambienti ospitati da Anthropic, le credenziali git e le chiavi di firma rimangono al di fuori della sandbox e un proxy autentica per conto della sessione con credenziali con ambito. In un ambiente self-hosted, la tua distribuzione fornisce credenziali git; vedi [Configura git](/docs/it/self-hosted-environments-deploy#configure-git)
* **Credenziali API**: negli ambienti ospitati da Anthropic nei piani Pro e Max, le chiavi che [aggiungi a un ambiente cloud](/docs/it/cloud-environments#add-api-credentials) rimangono al di fuori della sandbox allo stesso modo, allegate alle richieste corrispondenti dopo che lasciano la sessione. Un ambiente self-hosted non ha credenziali API e i piani Team ed Enterprise non le hanno ancora
* **Analisi sicura**: il codice viene analizzato e modificato all'interno dell'ambiente isolato della sessione prima di creare PR

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Per gli errori API di runtime che appaiono nella conversazione come `API Error: 500`, `529 Overloaded`, `429` o `Prompt is too long`, vedi il [riferimento degli errori](/docs/it/errors). Questi errori e le loro correzioni sono condivisi con la CLI e l'app Desktop. Le sezioni seguenti coprono i problemi specifici delle sessioni cloud.

<h3 id="session-creation-failed">
  Creazione della sessione non riuscita
</h3>

Se una nuova sessione non si avvia con `Session creation failed` o si blocca al provisioning, Claude Code non ha potuto allocare una VM per la sessione.

* Controlla [status.claude.com](https://status.claude.com) per gli incidenti delle sessioni cloud
* Riprova dopo un minuto, poiché la capacità viene fornita su richiesta
* Conferma che la tua connessione GitHub possa raggiungere il repository seguendo [Nessun repository appare dopo la connessione di GitHub](/docs/it/web-quickstart#no-repositories-appear-after-connecting-github)

<h3 id="unable-to-get-organization-uuid">
  Impossibile ottenere l'UUID dell'organizzazione
</h3>

`claude --cloud` e `claude --teleport` richiedono l'accesso con un account claude.ai. Se ti autentichi con una chiave API, o i dettagli dell'account archiviati sono obsoleti, questi comandi falliscono con `Unable to get organization UUID` o un messaggio che l'autenticazione con chiave API non è sufficiente. Con l'autenticazione con chiave API o i dettagli dell'account obsoleti, l'esecuzione di `claude --teleport` senza un ID di sessione mostra `Error loading Claude Code sessions` nel selettore di sessione invece di uno dei due messaggi, e la stessa correzione si applica.

Esegui `/login` per accedere con il tuo account claude.ai, quindi riprova il comando. Se il messaggio nomina il tuo provider, vedi la [tabella degli errori](#output-and-errors): le sessioni cloud non sono disponibili tramite provider di terze parti.

<h3 id="remote-control-session-expired-or-access-denied">
  Sessione Remote Control scaduta o accesso negato
</h3>

`--teleport` si connette attraverso la stessa infrastruttura della sessione Remote Control che le sessioni cloud utilizzano, quindi gli errori di autenticazione e scadenza della sessione si presentano con la terminologia Remote Control. Potresti vedere `Remote Control session expired` o `Access denied`. Il token di connessione è di breve durata e limitato al tuo account.

* Esegui `/login` localmente per aggiornare le tue credenziali, quindi riconnettiti
* Conferma che sei connesso allo stesso account che possiede la sessione
* Se vedi `Remote Control may not be available for this organization`, un Proprietario non ha abilitato le sessioni cloud per la tua organizzazione

<h3 id="environment-expired">
  Ambiente scaduto
</h3>

Le sessioni cloud si fermano dopo un periodo di inattività e la VM della sessione viene recuperata. Una sessione conta come inattiva mentre attende che tu approvi una chiamata dello strumento [MCP connector](/docs/it/cloud-environments#network-access) o che tu acceda a un server MCP, e può scadere durante quell'attesa.

Riapri la sessione da [claude.ai/code](https://claude.ai/code) per fornire una VM fresca con la cronologia della conversazione ripristinata. Il lavoro in background che era ancora in esecuzione quando la VM è stata recuperata, come subagent e comandi shell, non viene ripristinato.

<h2 id="limitations">
  Limitazioni
</h2>

Prima di fare affidamento sulle sessioni cloud per un flusso di lavoro, tieni conto di questi vincoli:

* **Limiti di velocità**: le sessioni cloud condividono i limiti di velocità con tutti gli altri utilizzi di Claude e Claude Code all'interno del tuo account. L'esecuzione di più attività in parallelo consuma più limiti di velocità proporzionalmente. Non esiste alcun addebito di calcolo separato per la VM cloud.
* **Autenticazione del repository**: puoi estrarre una sessione cloud nel tuo terminale solo quando sei autenticato allo stesso account
* **Restrizioni della piattaforma**: il clonaggio del repository e la creazione di pull request richiedono GitHub. Le istanze self-hosted di [GitHub Enterprise Server](/docs/it/github-enterprise-server) sono supportate per i piani Team ed Enterprise. Puoi inviare un repository GitLab, Bitbucket o altro non-GitHub a una sessione cloud come [bundle locale](#send-local-repositories-without-github) impostando `CCR_FORCE_BUNDLE=1`, ma la sessione non può eseguire il push dei risultati di nuovo a quel remoto
* **Elenco IP consentiti dell'organizzazione**: le sessioni cloud chiamano l'API Anthropic dall'infrastruttura gestita da Anthropic, non dalla tua rete, mentre le sessioni in un [ambiente self-hosted](/docs/it/self-hosted-environments) la chiamano dalla tua rete. Se la tua organizzazione ha [IP allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) abilitato, ogni sessione cloud ospitata da Anthropic fallisce con un errore di autenticazione. Lo stesso vale per [Code Review](/docs/it/code-review) e per [routine](/docs/it/routines) che vengono eseguite su ambienti ospitati da Anthropic; una routine instradata a un ambiente self-hosted chiama l'API dalla tua rete. Contatta il [supporto Anthropic](https://support.claude.com/) per esentare i servizi ospitati da Anthropic dall'elenco IP consentiti della tua organizzazione.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Ambienti cloud](/docs/it/cloud-environments): configura l'accesso alla rete, le variabili di ambiente e gli script di configurazione per le sessioni cloud
* [Progetti](/docs/it/claude-projects): una conversazione in cui Claude coordina sessioni cloud parallele sui tuoi repository e riferisce i risultati
* [Ultrareview](/docs/it/ultrareview): esegui una profonda revisione del codice multi-agente in una sandbox cloud
* [Routine](/docs/it/routines): automatizza il lavoro su una pianificazione, tramite chiamata API o in risposta agli eventi GitHub
* [Configurazione degli hook](/docs/it/hooks): esegui script agli eventi del ciclo di vita della sessione
* [Tutte le impostazioni](/docs/it/settings-reference): tutte le opzioni di configurazione
* [Sicurezza](/docs/it/security): garanzie di isolamento e gestione dei dati
* [Utilizzo dei dati](/docs/it/data-usage): cosa Anthropic conserva dalle sessioni cloud
* [Claude Tag](https://claude.com/docs/claude-tag/overview): un @Claude gestito dall'organizzazione in Slack che viene eseguito sulla stessa infrastruttura cloud
