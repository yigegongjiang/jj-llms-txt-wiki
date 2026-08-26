> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Iniziare con Claude Code sul web

> Esegui Claude Code nel cloud dal tuo browser o telefono. Connetti un repository GitHub, invia un'attività e rivedi la PR senza configurazione locale.

<Note>
  Claude Code sul web è in anteprima di ricerca per gli utenti Pro, Max e Team, e per gli utenti Enterprise con posti premium o posti Chat + Claude Code.
</Note>

Claude Code sul web viene eseguito su un'infrastruttura cloud gestita da Anthropic invece che sulla tua macchina. Invia attività da [claude.ai/code](https://claude.ai/code) dal tuo browser o dall'app mobile Claude.

Avrai bisogno di un repository GitHub per [iniziare](#connect-github-and-create-an-environment). Claude lo clona in una macchina virtuale isolata, apporta modifiche e spinge un ramo per te da rivedere. Le sessioni persistono tra i dispositivi, quindi un'attività che inizi sul tuo laptop è pronta per essere rivista dal tuo telefono in seguito.

Claude Code sul web funziona bene per:

* **Attività parallele**: esegui diversi compiti indipendenti contemporaneamente, ognuno nella sua sessione e ramo, senza gestire più worktrees
* **Repository che non hai localmente**: Claude clona il repository da zero ogni sessione, quindi non hai bisogno che sia estratto
* **Attività che non richiedono frequenti correzioni**: invia un'attività ben definita, fai qualcos'altro e rivedi il risultato quando Claude ha finito
* **Domande sul codice ed esplorazione**: comprendi una base di codice o traccia come una funzione è implementata senza un checkout locale

Per il lavoro che necessita della tua configurazione locale, strumenti o ambiente, eseguire Claude Code localmente o utilizzare [Remote Control](/docs/it/remote-control) è una scelta migliore.

<h2 id="how-sessions-run">
  Come vengono eseguite le sessioni
</h2>

Quando invii un'attività:

1. **Clone e preparazione**: il tuo repository viene clonato in una VM gestita da Anthropic e il tuo [script di configurazione](/docs/it/claude-code-on-the-web#setup-scripts) viene eseguito se configurato.
2. **Configura la rete**: l'accesso a Internet viene impostato in base al [livello di accesso](/docs/it/claude-code-on-the-web#access-levels) del tuo ambiente.
3. **Lavoro**: Claude analizza il codice, apporta modifiche, esegue test e verifica il suo lavoro. Puoi guardare e guidare durante tutto il processo, oppure allontanarti e tornare quando ha finito.
4. **Spingere il ramo**: quando Claude raggiunge un punto di arresto, spinge il suo ramo su GitHub. Rivedi il diff, lascia commenti inline, crea una PR o invia un altro messaggio per continuare.

La sessione non si chiude quando il ramo viene spinto. La creazione di PR e ulteriori modifiche avvengono tutte all'interno della stessa conversazione.

<h2 id="compare-ways-to-run-claude-code">
  Confronta i modi per eseguire Claude Code
</h2>

Claude Code si comporta allo stesso modo ovunque. Quello che cambia è dove viene eseguito il codice e se la tua configurazione locale è disponibile. L'app Desktop offre sia sessioni locali che cloud, quindi le risposte di seguito dipendono da quale scegli:

|                                                        | Sul web                                                                                                                     | Remote Control                    | Terminal CLI               | App Desktop                  |
| :----------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :-------------------------------- | :------------------------- | :--------------------------- |
| **Il codice viene eseguito su**                        | VM cloud Anthropic                                                                                                          | La tua macchina                   | La tua macchina            | La tua macchina o VM cloud   |
| **Chatti da**                                          | claude.ai o app mobile                                                                                                      | claude.ai o app mobile            | Il tuo terminale           | L'interfaccia utente Desktop |
| **Utilizza la tua configurazione locale**              | No, solo repository                                                                                                         | Sì                                | Sì                         | Sì per locale, no per cloud  |
| **Richiede GitHub**                                    | Sì, o [raggruppa un repository locale](/docs/it/claude-code-on-the-web#send-local-repositories-without-github) tramite `--cloud` | No                                | No                         | Solo per sessioni cloud      |
| **Continua a funzionare se ti disconnetti**            | Sì                                                                                                                          | Mentre il terminale rimane aperto | No                         | Dipende dal tipo di sessione |
| **[Modalità di autorizzazione](/docs/it/permission-modes)** | Accetta modifiche, Plan, Auto                                                                                               | Manuale, Accetta modifiche, Plan  | Tutte le modalità          | Dipende dal tipo di sessione |
| **Accesso alla rete**                                  | Configurabile per ambiente                                                                                                  | La rete della tua macchina        | La rete della tua macchina | Dipende dal tipo di sessione |

Consulta la documentazione [quickstart del terminale](/docs/it/quickstart), [app Desktop](/docs/it/desktop) o [Remote Control](/docs/it/remote-control) per configurarli.

<h2 id="connect-github-and-create-an-environment">
  Connetti GitHub e crea un ambiente
</h2>

La configurazione è un processo una tantum. Se usi già la GitHub CLI, puoi [farlo dal tuo terminale](#connect-from-your-terminal) invece che dal browser.

<Steps>
  <Step title="Visita claude.ai/code">
    Vai a [claude.ai/code](https://claude.ai/code) e accedi con il tuo account Anthropic.
  </Step>

  <Step title="Installa l'app Claude GitHub">
    Dopo l'accesso, claude.ai/code ti chiede di connettere GitHub. Segui il prompt per installare l'app Claude GitHub e concederle l'accesso ai tuoi repository. Le sessioni cloud funzionano con i repository GitHub esistenti, quindi per avviare un nuovo progetto, [crea prima un repository vuoto su GitHub](https://github.com/new).
  </Step>

  <Step title="Crea il tuo ambiente">
    Dopo aver connesso GitHub, ti verrà chiesto di creare un ambiente cloud. L'ambiente controlla quale accesso di rete Claude ha durante le sessioni e cosa viene eseguito quando viene creata una nuova sessione. Consulta [Strumenti installati](/docs/it/claude-code-on-the-web#installed-tools) per vedere cosa è disponibile senza alcuna configurazione.

    Il modulo ha questi campi:

    * **Nome**: un'etichetta di visualizzazione. Utile quando hai più ambienti per progetti diversi o livelli di accesso.
    * **Accesso alla rete**: controlla cosa la sessione può raggiungere su Internet. L'impostazione predefinita, `Trusted`, consente connessioni a [registri di pacchetti comuni](/docs/it/claude-code-on-the-web#default-allowed-domains) come npm, PyPI e RubyGems mentre blocca l'accesso generale a Internet.
    * **Variabili di ambiente**: variabili facoltative disponibili in ogni sessione, in formato `.env`. Non racchiudere i valori tra virgolette, poiché le virgolette vengono archiviate come parte del valore. Questi sono visibili a chiunque possa modificare questo ambiente.
    * **Script di configurazione**: uno script Bash facoltativo che viene eseguito prima dell'avvio di Claude Code. Usalo per installare strumenti di sistema che la VM cloud non include, come `apt install -y gh`. Il risultato è [memorizzato nella cache](/docs/it/claude-code-on-the-web#environment-caching), quindi lo script non viene rieseguito ad ogni sessione. Consulta [Script di configurazione](/docs/it/claude-code-on-the-web#setup-scripts) per esempi e suggerimenti per il debug.

    Per un primo progetto, lascia i valori predefiniti e fai clic su **Crea ambiente**. Puoi [modificarlo in seguito o creare ambienti aggiuntivi](/docs/it/claude-code-on-the-web#configure-your-environment) per progetti diversi.
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Connetti dal tuo terminale
</h3>

Se usi già la GitHub CLI (`gh`), puoi configurare Claude Code sul web senza aprire un browser. Questo richiede la [Claude Code CLI](/docs/it/quickstart). `/web-setup` legge il tuo token `gh` locale, lo collega al tuo account Claude e crea un ambiente cloud predefinito se non ne hai uno.

<Note>
  Le organizzazioni con [Zero Data Retention](/docs/it/zero-data-retention) abilitato non possono utilizzare `/web-setup` o altre funzioni di sessione cloud. Se la GitHub CLI non è installata o autenticata, `/web-setup` apre il flusso di onboarding del browser.
</Note>

<Steps>
  <Step title="Autentica con la GitHub CLI">
    Nel tuo shell, autentica la GitHub CLI se non l'hai già fatto:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Accedi a Claude">
    Nella Claude Code CLI, esegui `/login` per accedere con il tuo account claude.ai. Salta questo passaggio se sei già connesso.
  </Step>

  <Step title="Esegui /web-setup">
    Nella Claude Code CLI, esegui:

    ```text theme={null}
    /web-setup
    ```

    Questo sincronizza il tuo token `gh` al tuo account Claude. Se non hai ancora un ambiente cloud, `/web-setup` ne crea uno con accesso alla rete Trusted e nessuno script di configurazione. Puoi [modificare l'ambiente o aggiungere variabili](/docs/it/claude-code-on-the-web#configure-your-environment) in seguito. Una volta completato `/web-setup`, puoi avviare sessioni cloud dal tuo terminale con [`--cloud`](/docs/it/claude-code-on-the-web#from-terminal-to-web) o configurare attività ricorrenti con [`/schedule`](/docs/it/routines).
  </Step>
</Steps>

<h2 id="start-a-task">
  Avvia un'attività
</h2>

Con GitHub connesso e un ambiente creato, sei pronto a inviare attività.

<Steps>
  <Step title="Seleziona un repository e un ramo">
    Da [claude.ai/code](https://claude.ai/code) o dalla scheda Code nell'app mobile Claude, fai clic sul selettore di repository sotto la casella di input e scegli un repository su cui Claude lavorerà. Ogni repository mostra un selettore di ramo. Cambialo per avviare Claude da un ramo di funzionalità invece di quello predefinito. Puoi aggiungere più repository per lavorare su di essi in una sessione.
  </Step>

  <Step title="Scegli una modalità di autorizzazione">
    Il menu a discesa della modalità accanto all'input è impostato per impostazione predefinita su **Accetta automaticamente le modifiche**, dove Claude apporta modifiche e spinge un ramo senza fermarsi per l'approvazione. Passa a **Plan** se vuoi che Claude proponga un approccio e attenda il tuo via libera prima di modificare i file. Le sessioni cloud non offrono autorizzazioni Manual o Bypass. Consulta l'[elenco completo delle modalità di autorizzazione](/docs/it/permission-modes#available-modes) per scoprire cosa consente ciascuna.
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
| `environment`  | Nome o ID dell'[ambiente](#connect-github-and-create-an-environment) da preselezionare.                                                                                                                 |

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

Una sessione cloud può utilizzare qualsiasi repository che l'account GitHub connesso può vedere, indipendentemente da quali repository l'app Claude GitHub è installata. Se un repository è mancante, verifica che l'account GitHub connesso abbia accesso ad esso su GitHub. Se vuoi anche [Auto-fix](/docs/it/claude-code-on-the-web#auto-fix-pull-requests) per un repository, installa l'app su di esso: su github.com, apri **Impostazioni → Applicazioni → Claude → Configura** e verifica che il repository sia elencato sotto **Accesso al repository**. I repository privati hanno bisogno della stessa autorizzazione di quelli pubblici.

<h3 id="the-page-only-shows-a-github-login-button">
  La pagina mostra solo un pulsante di accesso a GitHub
</h3>

Le sessioni cloud richiedono un account GitHub connesso. Connettiti tramite il flusso del browser sopra, o esegui `/web-setup` dal tuo terminale se usi la GitHub CLI. Se preferisci non connettere GitHub affatto, consulta [Remote Control](/docs/it/remote-control) per eseguire Claude Code sulla tua macchina e monitorarlo dal web.

<h3 id="not-available-for-the-selected-organization">
  "Non disponibile per l'organizzazione selezionata"
</h3>

Le organizzazioni Enterprise potrebbero aver bisogno che un amministratore abiliti Claude Code sul web. Contatta il tuo team di account Anthropic.

<h3 id="/web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` mostra "No commands match" o "Unknown command"
</h3>

`/web-setup` viene eseguito all'interno della Claude Code CLI, non nel tuo shell. Avvia `claude` prima, quindi digita `/web-setup` al prompt.

Se l'hai digitato all'interno di Claude Code e il menu dei comandi mostra `No commands match "/web-setup"`, o l'invio restituisce `Unknown command: /web-setup`, il comando è nascosto perché un requisito non è soddisfatto. La causa è solitamente che sei autenticato con una chiave API o un provider di terze parti invece di un abbonamento claude.ai. Esegui `/login` per accedere con il tuo account claude.ai.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud-or-ultraplan">
  "Could not create a cloud environment" o "No cloud environment available" quando si utilizza `--cloud` o ultraplan
</h3>

Le funzioni di sessione remota creano automaticamente un ambiente cloud predefinito se non ne hai uno. Se vedi "Could not create a cloud environment", la creazione automatica non è riuscita. Se vedi "No cloud environment available", la tua CLI è precedente alla creazione automatica. In entrambi i casi, esegui `/web-setup` nella Claude Code CLI per crearne uno manualmente, o visita [claude.ai/code](https://claude.ai/code) e segui il passaggio **Crea il tuo ambiente** sopra.

<h3 id="setup-script-failed">
  Lo script di configurazione non è riuscito
</h3>

Lo script di configurazione è uscito con uno stato diverso da zero, il che blocca l'avvio della sessione. Le cause comuni sono:

* Un'installazione di pacchetto non è riuscita perché il registro non è nel tuo [livello di accesso alla rete](/docs/it/claude-code-on-the-web#access-levels). `Trusted` copre la maggior parte dei gestori di pacchetti; `None` li blocca tutti.
* Lo script fa riferimento a un file o un percorso che non esiste in un clone fresco.
* Un comando che funziona localmente ha bisogno di una diversa invocazione su Ubuntu.

Per eseguire il debug, aggiungi `set -x` nella parte superiore dello script per vedere quale comando non è riuscito. Per i comandi non critici, aggiungi `|| true` in modo che non blocchino l'avvio della sessione.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Nuove sessioni si bloccano o scadono durante la configurazione
</h3>

Se le nuove sessioni si fermano al passaggio dello script di configurazione o falliscono con un errore generico del contenitore prima che lo script finisca, lo script probabilmente sta superando il budget di tempo di circa cinque minuti per la costruzione della [cache dell'ambiente](/docs/it/claude-code-on-the-web#environment-caching). I passaggi pesanti come il pull di grandi immagini Docker, la sincronizzazione di alberi di dipendenze completi o il download di pesi del modello spesso spingono il totale oltre il limite, soprattutto quando vengono eseguiti uno dopo l'altro.

Per risolvere questo, riduci lo script in modo che finisca in modo affidabile in meno di cinque minuti:

* Esegui installazioni indipendenti in parallelo con `&` e un `wait` finale invece di eseguirle in serie.
* Sposta i download più grandi fuori dallo script di configurazione e in un [hook SessionStart](/docs/it/claude-code-on-the-web#setup-scripts-vs-sessionstart-hooks) che li avvia in background, in modo che la sessione diventi utilizzabile mentre finiscono.
* Rimuovi i lunghi sleep di ripetizione dallo script di configurazione, poiché un ciclo di ripetizione bloccato conta nel budget.

<h3 id="session-keeps-running-after-closing-the-tab">
  La sessione continua a funzionare dopo la chiusura della scheda
</h3>

Questo è intenzionale. Chiudere la scheda o navigare via non interrompe la sessione. Continua a funzionare in background fino a quando Claude non finisce l'attività corrente, quindi rimane inattiva. Dalla barra laterale, puoi [archiviare una sessione](/docs/it/claude-code-on-the-web#archive-sessions) per nasconderla dal tuo elenco, o [eliminarla](/docs/it/claude-code-on-the-web#delete-sessions) per rimuoverla permanentemente.

<h2 id="next-steps">
  Passaggi successivi
</h2>

Ora che puoi inviare e rivedere attività, queste pagine coprono cosa viene dopo: avviare sessioni cloud dal tuo terminale, pianificare lavori ricorrenti e dare a Claude istruzioni permanenti.

* [Usa Claude Code sul web](/docs/it/claude-code-on-the-web): il riferimento completo, incluso il teletrasporto di sessioni al tuo terminale, script di configurazione, variabili di ambiente e configurazione di rete
* [Routines](/docs/it/routines): automatizza il lavoro su una pianificazione, tramite chiamata API o in risposta agli eventi di GitHub
* [CLAUDE.md](/docs/it/memory): dai a Claude istruzioni e contesto persistenti che si caricano all'inizio di ogni sessione
* Installa l'app mobile Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) per monitorare le sessioni dal tuo telefono. Dalla Claude Code CLI, `/mobile` mostra un codice QR.
