> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Iniziare con Claude Code nel cloud

> Esegui Claude Code nel cloud dal tuo browser o telefono. Connetti un repository GitHub, invia un'attività e rivedi la PR senza configurazione locale.

<Note>
  Le sessioni cloud sono disponibili sui piani Pro, Max e Team, e per gli utenti Enterprise con posti premium o posti Chat + Claude Code.
</Note>

Una sessione cloud esegue Claude Code su un'infrastruttura cloud invece che sulla tua macchina, gestita da Anthropic per impostazione predefinita. Questo quickstart ne avvia una da [claude.ai/code](https://claude.ai/code) nel tuo browser. Puoi anche avviarne una dall'app mobile Claude, dall'app Desktop o dal tuo terminale con `claude --cloud`.

Avrai bisogno di un repository GitHub per [iniziare](#connect-github). Claude lo clona in una macchina virtuale isolata, apporta modifiche e spinge un ramo per te da rivedere. Le sessioni persistono tra i dispositivi, quindi un'attività che inizi sul tuo laptop è pronta per essere rivista dal tuo telefono in seguito.

Le sessioni cloud funzionano bene per:

* **Attività parallele**: esegui diversi compiti indipendenti contemporaneamente, ognuno nella sua sessione e ramo, senza gestire più worktrees
* **Repository che non hai localmente**: Claude clona il repository da zero ogni sessione, quindi non hai bisogno che sia estratto
* **Attività che non richiedono frequenti correzioni**: invia un'attività ben definita, fai qualcos'altro e rivedi il risultato quando Claude ha finito
* **Domande sul codice ed esplorazione**: comprendi una base di codice o traccia come una funzione è implementata senza un checkout locale

Per il lavoro che necessita della tua configurazione locale, strumenti o ambiente, eseguire Claude Code localmente o utilizzare [Remote Control](/docs/it/remote-control) è una scelta migliore.

<h2 id="how-sessions-run">
  Come vengono eseguite le sessioni
</h2>

I passaggi seguenti descrivono le sessioni ospitate da Anthropic. In un [ambiente self-hosted](/docs/it/self-hosted-environments), il clone e tutto ciò che segue vengono eseguiti sui runner della tua organizzazione, dove i confini di rete, la configurazione e il comportamento di push sono configurati dall'operatore. Quando invii un'attività:

1. **Clone e preparazione**: il tuo repository viene clonato in una VM gestita da Anthropic e il tuo [script di configurazione](/docs/it/cloud-environments#setup-scripts) viene eseguito se configurato.
2. **Configura la rete**: l'accesso a Internet viene impostato in base al [livello di accesso](/docs/it/cloud-environments#access-levels) del tuo ambiente.
3. **Lavoro**: Claude analizza il codice, apporta modifiche, esegue test e verifica il suo lavoro. Puoi guardare e guidare durante tutto il processo, oppure allontanarti e tornare quando ha finito.
4. **Spingere il ramo**: quando Claude raggiunge un punto di arresto, spinge il suo ramo su GitHub. Rivedi il diff, lascia commenti inline, crea una PR o invia un altro messaggio per continuare.

La sessione non si chiude quando il ramo viene spinto. La creazione di PR e ulteriori modifiche avvengono tutte all'interno della stessa conversazione.

<h2 id="compare-ways-to-run-claude-code">
  Confronta i modi per eseguire Claude Code
</h2>

Claude Code si comporta allo stesso modo ovunque. Quello che cambia è dove viene eseguita la sessione e se la tua configurazione locale è disponibile:

|                                                        | Sessione cloud                                                                                                              | Sessione locale                                                                                                                                       | Sessione locale con [Remote Control](/docs/it/remote-control)       |
| :----------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- |
| **Il codice viene eseguito su**                        | VM cloud, gestita da Anthropic per impostazione predefinita                                                                 | La tua macchina                                                                                                                                       | La tua macchina                                                |
| **La avvii da**                                        | claude.ai/code, l'app mobile Claude, l'app Desktop con **Cloud** selezionato, o `claude --cloud`                            | Il tuo terminale, il tuo IDE, o l'app Desktop con **Local** selezionato                                                                               | Il tuo terminale, l'estensione VS Code, o l'app Desktop        |
| **Chatti da**                                          | claude.ai, l'app mobile, o l'app Desktop                                                                                    | Dove l'hai avviata                                                                                                                                    | claude.ai o l'app mobile, così come dove l'hai avviata         |
| **Utilizza la tua configurazione locale**              | No, solo repository                                                                                                         | Sì                                                                                                                                                    | Sì                                                             |
| **Richiede GitHub**                                    | Sì, o [raggruppa un repository locale](/docs/it/claude-code-on-the-web#send-local-repositories-without-github) tramite `--cloud` | No                                                                                                                                                    | No                                                             |
| **Continua a funzionare se ti disconnetti**            | Sì                                                                                                                          | No                                                                                                                                                    | Mentre la sessione rimane aperta sulla tua macchina            |
| **[Modalità di autorizzazione](/docs/it/permission-modes)** | Accetta modifiche, Plan, Auto                                                                                               | Tutte le modalità nel terminale; consulta [Cambia modalità di autorizzazione](/docs/it/permission-modes#switch-permission-modes) per l'IDE e l'app Desktop | Manuale, Accetta modifiche, o Plan da claude.ai e l'app mobile |
| **Accesso alla rete**                                  | Configurabile per ambiente                                                                                                  | La rete della tua macchina                                                                                                                            | La rete della tua macchina                                     |

Consulta la documentazione [quickstart del terminale](/docs/it/quickstart), [app Desktop](/docs/it/desktop), o [Remote Control](/docs/it/remote-control) per configurare le sessioni locali.

<h2 id="connect-github">
  Connetti GitHub
</h2>

La connessione di GitHub è un passaggio una tantum. Se usi già la GitHub CLI, puoi [farlo dal tuo terminale](#connect-from-your-terminal) invece che dal browser.

<Note>
  Nei piani Team ed Enterprise, il passaggio **Sign in with GitHub** funziona solo dopo che un [Owner](/docs/it/server-managed-settings#access-control) della tua organizzazione Claude attiva il connettore GitHub in [**Admin settings > Connectors**](https://claude.ai/admin-settings/connectors). Fino ad allora, quel passaggio mostra "GitHub access is required for Claude Code on the web" invece di un pulsante di accesso. Dopo che il connettore è attivo, ricarica [claude.ai/code](https://claude.ai/code) e ricomincia dal primo passaggio. Un secondo interruttore, [Quick web setup](/docs/it/claude-code-on-the-web#github-authentication-options) in [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code), è facoltativo: con esso attivo, `/web-setup` funziona e l'onboarding crea l'ambiente per i membri.
</Note>

<Steps>
  <Step title="Visit claude.ai/code">
    Vai a [claude.ai/code](https://claude.ai/code) e accedi con il tuo account claude.ai.
  </Step>

  <Step title="Sign in with GitHub">
    Dopo l'accesso, claude.ai/code ti chiede di connettere GitHub. Segui il prompt e claude.ai/code ti invia alla pagina di autorizzazione di GitHub. Approva la richiesta di autorizzazione e GitHub ti restituisce a claude.ai/code. Le sessioni cloud funzionano con i repository GitHub esistenti. Per avviare un nuovo progetto, [crea prima un repository vuoto su GitHub](https://github.com/new).

    Con questa connessione, una sessione può clonare qualsiasi repository pubblico, ma può lavorare in un repository privato solo quando l'App Claude GitHub è installata su di esso. [Installa l'App](https://github.com/apps/claude/installations/new) su ogni account GitHub o organizzazione i cui repository privati desideri utilizzare. Su un'organizzazione GitHub, un proprietario dell'organizzazione potrebbe aver bisogno di approvare l'installazione. L'installazione dell'App abilita anche [Auto-fix](/docs/it/claude-code-on-the-web#auto-fix-pull-requests), che consente a Claude di rispondere ai fallimenti CI e ai commenti di revisione sulle pull request in quei repository.

    Se l'onboarding ti chiede di installare l'App a questo punto e preferisci farlo in seguito, fai clic su **Skip**.
  </Step>

  <Step title="Set up your Default environment">
    Un [cloud environment](/docs/it/cloud-environments) è la configurazione salvata che controlla quale accesso di rete Claude ha durante le sessioni e cosa viene eseguito quando una sessione si avvia. Quello che accade dopo che connetti GitHub dipende dal tuo piano:

    * **Pro e Max**: l'onboarding crea un ambiente denominato **Default** per te.
    * **Team ed Enterprise**: l'onboarding mostra un modulo **Create your first cloud environment**. Lascia il nome precompilato e l'accesso alla rete invariati e fai clic su **Create & finish** per creare l'ambiente **Default**. Se un Owner ha attivato [Quick web setup](/docs/it/claude-code-on-the-web#github-authentication-options), l'onboarding crea **Default** per te invece.

    **Default** utilizza [`Trusted` network access](/docs/it/cloud-environments#access-levels): le sessioni raggiungono [registri di pacchetti comuni](/docs/it/cloud-environments#default-allowed-domains) e altri domini nella lista di autorizzazione, e nient'altro attraverso la rete della sessione. Vedi [Installed tools](/docs/it/cloud-environments#installed-tools) per quello che è disponibile senza alcuna configurazione.

    Per un primo progetto, l'ambiente **Default** funziona così com'è. Per modificare il suo accesso alla rete, aggiungere variabili di ambiente o eseguire uno [script di configurazione](/docs/it/cloud-environments#setup-scripts) prima dell'avvio delle sessioni, [modificalo o crea ambienti aggiuntivi](/docs/it/cloud-environments#configure-your-environment).
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Connetti dal tuo terminale
</h3>

Se usi già la GitHub CLI (`gh`), puoi connettere GitHub per le sessioni cloud dal tuo terminale. Questo richiede la [Claude Code CLI](/docs/it/quickstart). Nei piani Team ed Enterprise, `/web-setup` è disponibile solo dopo che un Owner attiva [Quick web setup](/docs/it/claude-code-on-the-web#github-authentication-options).

Quando esegui `/web-setup`, Claude Code legge il token che `gh auth token` stampa, ti chiede di confermare e invia il token ad Anthropic. Anthropic lo archivia crittografato con il tuo account claude.ai e le tue sessioni cloud lo utilizzano per l'accesso a GitHub fino a quando non lo [rimuovi](#remove-the-web-setup-token). Una sessione cloud che avvii tu stesso può quindi accedere a qualsiasi repository a cui quel token può accedere, senza alcuna installazione dell'App Claude GitHub. I thread in un [progetto](/docs/it/claude-projects#set-up-github-access) hanno ancora bisogno dell'App.

Se hai già connesso GitHub nel browser, `/web-setup` ti avverte che continuare sostituisce quella connessione per le tue sessioni cloud.

<Note>
  Le organizzazioni con [Zero Data Retention](/docs/it/zero-data-retention) abilitato non possono utilizzare `/web-setup` o altre funzioni di sessione cloud. Se la GitHub CLI non è installata o non è autenticata, Claude Code apre il flusso di onboarding del browser.
</Note>

<Steps>
  <Step title="Authenticate with the GitHub CLI">
    Nel tuo shell, autentica la GitHub CLI se non l'hai già fatto:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Sign in to Claude">
    Nella Claude Code CLI, esegui `/login` per accedere con il tuo account claude.ai. Salta questo passaggio se sei già connesso con un account claude.ai. L'autenticazione con una chiave API non conta. Per verificare, esegui `/status` e conferma che la riga **Login method** mostra un account claude.ai.
  </Step>

  <Step title="Run /web-setup">
    Nella Claude Code CLI, esegui:

    ```text theme={null}
    /web-setup
    ```

    Conferma il prompt per inviare il tuo token `gh` al tuo account Claude. Se ha successo, Claude Code stampa `Connected as <your-github-username>` e apre [claude.ai/code](https://claude.ai/code) nel tuo browser. Se non hai ancora un ambiente cloud, `/web-setup` ne crea uno con accesso alla rete Trusted e nessuno script di configurazione. Puoi [modificare l'ambiente o aggiungere variabili](/docs/it/cloud-environments#configure-your-environment) in seguito. Una volta completato `/web-setup`, puoi avviare sessioni cloud dal tuo terminale con [`--cloud`](/docs/it/claude-code-on-the-web#from-terminal-to-cloud) o configurare attività ricorrenti con [`/schedule`](/docs/it/routines).
  </Step>
</Steps>

<h4 id="remove-the-web-setup-token">
  Rimuovi il token `/web-setup`
</h4>

Per rimuovere il token dal tuo account Claude, disconnetti GitHub in [claude.ai/customize/connectors](https://claude.ai/customize/connectors). La disconnessione elimina le credenziali GitHub che le tue sessioni cloud utilizzano, che provengano dal browser o da `/web-setup`, quindi le sessioni cloud perdono l'accesso a GitHub fino a quando non ti connetti di nuovo. Il tuo `gh` locale rimane connesso e il token rimane valido su GitHub.

Per invalidare il token stesso, revocalo su GitHub. Se hai effettuato l'accesso a `gh` tramite il browser, il token appartiene alla voce **GitHub CLI** in [**Settings > Applications > Authorized OAuth Apps**](https://github.com/settings/applications) su GitHub, e revocare quella voce disconnette anche la GitHub CLI sulle tue macchine. Le sessioni cloud perdono quindi l'accesso a GitHub fino a quando non esegui di nuovo `gh auth login` e `/web-setup`.

<h2 id="start-a-task">
  Avvia un'attività
</h2>

Con GitHub connesso e un ambiente creato, sei pronto a inviare attività.

<Steps>
  <Step title="Seleziona un repository e un ramo">
    Da [claude.ai/code](https://claude.ai/code) o dalla scheda Code nell'app mobile Claude, fai clic sul selettore di repository sotto la casella di input e scegli un repository su cui Claude lavorerà. Ogni repository mostra un selettore di ramo. Cambialo per avviare Claude da un ramo di funzionalità invece di quello predefinito. Puoi aggiungere più repository per lavorare su di essi in una sessione.
  </Step>

  <Step title="Scegli una modalità di autorizzazione">
    Il menu a discesa della modalità accanto all'input mostra la modalità in cui verrà eseguita la sessione:

    * **Auto**: un classificatore esamina le azioni di Claude invece di chiederti. Appare quando la tua organizzazione consente la modalità auto e il modello selezionato la supporta
    * **Accept edits**: Claude apporta modifiche e spinge un ramo senza fermarsi per l'approvazione
    * **Plan**: Claude propone un approccio e attende il tuo via libera prima di modificare i file

    Le sessioni cloud non offrono autorizzazioni Manual o Bypass. Consulta l'[elenco completo delle modalità di autorizzazione](/docs/it/permission-modes#available-modes) per scoprire cosa consente ciascuna.
  </Step>

  <Step title="Descrivi l'attività e invia">
    Digita una descrizione di quello che vuoi e premi Invio. Sii specifico:

    * Nomina il file o la funzione: "Aggiungi un README con istruzioni di configurazione" o "Correggi il test di autenticazione non riuscito in `tests/test_auth.py`" è meglio di "correggi i test"
    * Incolla l'output dell'errore se lo hai
    * Descrivi il comportamento previsto, non solo il sintomo

    Claude clona i repository, esegue il tuo script di configurazione se configurato e inizia a lavorare. Ogni attività ottiene la sua sessione e il suo ramo, quindi non hai bisogno di aspettare che una finisca prima di avviarne un'altra.
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  Precompila le sessioni
</h2>

Puoi precompilare il prompt, i repository e l'ambiente per una nuova sessione aggiungendo parametri di query all'URL [claude.ai/code](https://claude.ai/code). Usalo per creare integrazioni come un pulsante nel tuo tracker di problemi che apre Claude Code con la descrizione del problema come prompt.

| Parametro      | Descrizione                                                                                                                                                                                             |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `prompt`       | Testo del prompt da precompilare nella casella di input. È accettato anche l'alias `q`.                                                                                                                 |
| `prompt_url`   | URL da cui recuperare il testo del prompt, per i prompt troppo lunghi da incorporare in una stringa di query. L'URL deve consentire richieste cross-origin. Ignorato quando è impostato anche `prompt`. |
| `repositories` | Elenco separato da virgole di slug `owner/repo` da preselezionare. È accettato anche l'alias `repo`.                                                                                                    |
| `environment`  | Nome o ID dell'[ambiente](#connect-github) da preselezionare.                                                                                                                                           |

Codifica URL ogni valore. L'esempio seguente apre il modulo con un prompt e un repository già selezionati:

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  Rivedi e itera
</h2>

Quando Claude finisce, rivedi le modifiche, lascia feedback su righe specifiche e continua finché il diff non sembra giusto.

<Steps>
  <Step title="Apri la vista diff">
    Un indicatore diff mostra le righe aggiunte e rimosse durante la sessione, ad esempio `+42 -18`. Selezionalo per aprire la vista diff, con un elenco di file a sinistra e le modifiche a destra.

    Il diff confronta le modifiche della sessione rispetto al suo ramo base per impostazione predefinita. Per confrontare rispetto a un ramo diverso, seleziona **Confronta con** e scegline uno.
  </Step>

  <Step title="Lascia commenti inline">
    Seleziona qualsiasi riga nel diff, digita il tuo feedback e premi Invio. I commenti si accumulano fino a quando non invii il tuo prossimo messaggio, quindi vengono raggruppati con esso. Claude vede "a `src/auth.ts:47`, non catturare l'errore qui" insieme alla tua istruzione principale, quindi non devi descrivere dove si trova il problema.
  </Step>

  <Step title="Crea una pull request">
    Quando il diff sembra giusto, seleziona **Crea PR** nella parte superiore della vista diff. Puoi aprirla come una PR completa, una bozza, o saltare alla pagina di composizione di GitHub con un titolo e una descrizione generati.
  </Step>

  <Step title="Continua a iterare dopo la PR">
    La sessione rimane attiva dopo la creazione della PR. Incolla l'output di errore CI o i commenti dei revisori nella chat e chiedi a Claude di affrontarli. Per fare in modo che Claude monitori la PR automaticamente, consulta [Auto-fix pull requests](/docs/it/claude-code-on-the-web#auto-fix-pull-requests).
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  Risolvi i problemi di configurazione
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  Nessun repository appare dopo la connessione a GitHub
</h3>

Se hai connesso GitHub nel browser, le sessioni possono clonare qualsiasi repository pubblico, ma un repository privato appare solo quando l'app Claude GitHub è installata sull'account o sull'organizzazione che lo possiede e l'accesso ai repository dell'installazione lo include. [Installa l'app Claude GitHub](https://github.com/apps/claude/installations/new) lì, oppure chiedi a un proprietario dell'organizzazione di installarla o approvarla.

Se hai connesso con `/web-setup`, le sessioni raggiungono ogni repository a cui il tuo token `gh` può accedere. Esegui `gh repo view OWNER/REPO` nella tua shell per verificare che il tuo accesso GitHub CLI possa vedere il repository, ed esegui `/web-setup` di nuovo se hai cambiato account `gh` da quando ti sei connesso.

<h3 id="the-page-only-shows-a-github-login-button">
  La pagina mostra solo un pulsante di accesso a GitHub
</h3>

Le sessioni cloud richiedono un account GitHub connesso. Connettiti tramite il flusso del browser sopra, o esegui `/web-setup` dal tuo terminale se usi la GitHub CLI. Se preferisci non connettere GitHub affatto, consulta [Remote Control](/docs/it/remote-control) per eseguire Claude Code sulla tua macchina e monitorarlo dal browser o dal telefono.

<h3 id="not-available-for-the-selected-organization">
  "Non disponibile per l'organizzazione selezionata"
</h3>

Le organizzazioni Enterprise potrebbero aver bisogno che un proprietario abiliti le sessioni cloud. Contatta il tuo team di account Anthropic.

<h3 id="/web-setup-says-not-signed-in-to-claude">
  `/web-setup` dice "Not signed in to Claude"
</h3>

Se `/web-setup` risponde con "Not signed in to Claude. Run /login first.", la CLI non ha un accesso valido a claude.ai. Questo può accadere anche quando un accesso precedente è scaduto. Esegui `/login`, accedi con il tuo account claude.ai, quindi esegui `/web-setup` di nuovo.

<h3 id="/web-setup-warns-that-your-token-doesn’t-have-the-workflow-scope">
  `/web-setup` avverte che il tuo token non ha lo scope `workflow`
</h3>

Se `/web-setup` dice che il tuo token GitHub CLI non ha lo scope `workflow`, puoi continuare, ma GitHub può rifiutare alcuni push effettuati con quel token, come i push che modificano i file del flusso di lavoro GitHub Actions. Per aggiungere lo scope, esegui `gh auth refresh -s workflow` nella tua shell, quindi esegui `/web-setup` di nuovo.

<h3 id="web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` mostra "No commands match" o "Unknown command"
</h3>

`/web-setup` viene eseguito all'interno della Claude Code CLI, non nel tuo shell. Avvia `claude` prima, quindi digita `/web-setup` al prompt.

Se l'hai digitato all'interno di Claude Code e il menu dei comandi mostra `No commands match "/web-setup"`, o l'invio restituisce `Unknown command: /web-setup`, il comando è nascosto perché un requisito non è soddisfatto. La causa è solitamente che sei autenticato con una chiave API o un provider di terze parti invece di un abbonamento claude.ai. Esegui `/login` per accedere con il tuo account claude.ai.

Nei piani Team e Enterprise, il comando è nascosto per impostazione predefinita: il [Quick web setup toggle](/docs/it/claude-code-on-the-web#github-authentication-options) è disattivato fino a quando un proprietario non lo attiva. Mentre è disattivato, [connetti GitHub dal browser](#connect-github) invece.

Il comando è anche nascosto in altri due casi:

* Un amministratore ha disabilitato le sessioni cloud per la tua organizzazione. In questo caso, l'invio di `/web-setup` restituisce [`Cloud sessions are disabled by your organization's policy`](/docs/it/errors#cloud-sessions-are-disabled-by-your-organizations-policy). Prima della v2.1.268, questo caso restituiva anche `Unknown command: /web-setup`.
* La tua organizzazione Enterprise ha [Zero Data Retention](/docs/it/zero-data-retention) abilitato, il che rende le sessioni cloud non disponibili.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud">
  "Could not create a cloud environment" o "No cloud environment available" quando si utilizza `--cloud`
</h3>

Le funzioni di sessione cloud creano automaticamente un ambiente cloud predefinito se non ne hai uno. Se vedi "Could not create a cloud environment", la creazione automatica non è riuscita. Se vedi "No cloud environment available", la tua CLI è precedente alla creazione automatica. In entrambi i casi, esegui `/web-setup` nella Claude Code CLI, o aggiungi un ambiente dal [selettore di ambiente](/docs/it/cloud-environments#configure-your-environment) su [claude.ai/code](https://claude.ai/code).

<h3 id="setup-script-failed">
  Lo script di configurazione non è riuscito
</h3>

Lo script di configurazione è uscito con uno stato diverso da zero, il che blocca l'avvio della sessione. Le cause comuni sono:

* Un'installazione di pacchetto non è riuscita perché il registro non è nel tuo [livello di accesso alla rete](/docs/it/cloud-environments#access-levels). `Trusted` copre la maggior parte dei gestori di pacchetti; `None` li blocca tutti.
* Lo script fa riferimento a un file o un percorso che non esiste in un clone fresco.
* Un comando che funziona localmente ha bisogno di una diversa invocazione su Ubuntu.

Per eseguire il debug, aggiungi `set -x` nella parte superiore dello script per vedere quale comando non è riuscito. Per i comandi non critici, aggiungi `|| true` in modo che non blocchino l'avvio della sessione.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Nuove sessioni si bloccano o scadono durante la configurazione
</h3>

Se le nuove sessioni si fermano al passaggio dello script di configurazione o falliscono con un errore generico del contenitore prima che lo script finisca, lo script probabilmente sta superando il budget di tempo di circa cinque minuti per la costruzione della [cache dell'ambiente](/docs/it/cloud-environments#environment-caching). I passaggi pesanti come il pull di grandi immagini Docker, la sincronizzazione di alberi di dipendenze completi o il download di pesi del modello spesso spingono il totale oltre il limite, soprattutto quando vengono eseguiti uno dopo l'altro.

Per risolvere questo, riduci lo script in modo che finisca in modo affidabile in meno di cinque minuti:

* Esegui installazioni indipendenti in parallelo con `&` e un `wait` finale invece di eseguirle in serie.
* Sposta i download più grandi fuori dallo script di configurazione e in un [hook SessionStart](/docs/it/cloud-environments#setup-scripts-vs-sessionstart-hooks) che li avvia in background, in modo che la sessione diventi utilizzabile mentre finiscono.
* Rimuovi i lunghi sleep di ripetizione dallo script di configurazione, poiché un ciclo di ripetizione bloccato conta nel budget.

<h3 id="session-keeps-running-after-closing-the-tab">
  La sessione continua a funzionare dopo la chiusura della scheda
</h3>

Questo è intenzionale. Chiudere la scheda o navigare via non interrompe la sessione. Continua a funzionare in background fino a quando Claude non finisce l'attività corrente, quindi rimane inattiva. Dalla barra laterale, puoi [archiviare una sessione](/docs/it/claude-code-on-the-web#archive-sessions) per nasconderla dal tuo elenco, o [eliminarla](/docs/it/claude-code-on-the-web#delete-sessions) per rimuoverla permanentemente.

<h2 id="next-steps">
  Passaggi successivi
</h2>

Ora che puoi inviare e rivedere attività, queste pagine coprono cosa viene dopo: avviare sessioni cloud dal tuo terminale, pianificare lavori ricorrenti e dare a Claude istruzioni permanenti.

* [Usa Claude Code sul web](/docs/it/claude-code-on-the-web): il riferimento completo, incluso il teletrasporto di sessioni al tuo terminale, condivisione di sessioni e correzione automatica delle pull request
* [Configura ambienti cloud](/docs/it/cloud-environments): livelli di accesso alla rete, variabili di ambiente e script di configurazione per sessioni cloud
* [Routines](/docs/it/routines): automatizza il lavoro su una pianificazione, tramite chiamata API o in risposta agli eventi di GitHub
* [CLAUDE.md](/docs/it/memory): dai a Claude istruzioni e contesto persistenti che si caricano all'inizio di ogni sessione
* Installa l'app mobile Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) per monitorare le sessioni dal tuo telefono. Dalla Claude Code CLI, `/mobile` mostra un codice QR per [claude.ai/mobile](https://claude.ai/mobile) che apre il negozio di app corretto per il tuo telefono.
