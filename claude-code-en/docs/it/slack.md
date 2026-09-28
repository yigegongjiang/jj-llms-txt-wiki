> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code in Slack

> Delega i compiti di codifica direttamente dal tuo workspace Slack. Anthropic sta ritirando questa versione precedente per i workspace Team ed Enterprise a favore di Claude Tag; rimane il percorso di configurazione per i piani Pro e Max.

<Warning>
  Questa pagina documenta la versione precedente di Claude Code in Slack, che esegue ogni sessione con l'account di un singolo utente.

  * **Piani Team ed Enterprise:** Anthropic sta ritirando questa versione a favore di [Claude Tag](https://claude.com/product/tag), che esegue @Claude come identità condivisa della tua organizzazione con accesso configurato dall'amministratore. La tua app Slack e il tuo handle @Claude rimangono, e il tuo team di account Anthropic può comunicarti la data di transizione. [Configura Claude Tag](https://claude.com/docs/claude-tag/overview) per un nuovo workspace; per spostarne uno che utilizza già questa versione, vedi [Migra dalla versione precedente di Claude in Slack](https://claude.com/docs/claude-tag/admins/migrate-from-earlier).
  * **Piani Pro e Max:** Claude Tag non è disponibile sui piani individuali, quindi questa pagina rimane il percorso di configurazione.
</Warning>

Claude Code in Slack porta la potenza di Claude Code direttamente nel tuo workspace Slack. Quando menzioni `@Claude` con un compito di codifica, Claude rileva automaticamente l'intento e crea una sessione Claude Code sul web, permettendoti di delegare il lavoro di sviluppo senza lasciare le conversazioni del tuo team.

Questa integrazione è costruita sull'app Claude for Slack esistente ma aggiunge un instradamento intelligente a Claude Code sul web per le richieste relative alla codifica. Ogni sessione viene eseguita con il tuo account Claude personale, utilizzando i tuoi repository connessi e i tuoi limiti di piano.

<h2 id="use-cases">
  Casi d'uso
</h2>

* **Investigazione e correzione di bug**: Chiedi a Claude di investigare e correggere i bug non appena vengono segnalati nei canali Slack.
* **Revisioni del codice rapide e modifiche**: Fai in modo che Claude implementi piccole funzionalità o effettui il refactoring del codice in base al feedback del team.
* **Debug collaborativo**: Quando le discussioni del team forniscono contesto cruciale (ad esempio, riproduzioni di errori o segnalazioni di utenti), Claude può utilizzare queste informazioni per informare il suo approccio al debug.
* **Esecuzione di attività parallele**: Avvia compiti di codifica in Slack mentre continui altri lavori, ricevendo notifiche al completamento.

<h2 id="prerequisites">
  Prerequisiti
</h2>

Prima di utilizzare Claude Code in Slack, assicurati di avere quanto segue:

| Requisito            | Dettagli                                                                                         |
| :------------------- | :----------------------------------------------------------------------------------------------- |
| Piano Claude         | Pro, Max, Team o Enterprise con accesso a Claude Code (posti premium o posti Chat + Claude Code) |
| Sessioni cloud       | Le [sessioni cloud](/docs/it/claude-code-on-the-web) sono abilitate per il tuo account                |
| Account GitHub       | Connesso a [claude.ai/code](https://claude.ai/code) con almeno un repository autenticato         |
| Autenticazione Slack | Il tuo account Slack collegato al tuo account Claude tramite l'app Claude                        |

<h2 id="setting-up-claude-code-in-slack">
  Configurazione di Claude Code in Slack
</h2>

<Steps>
  <Step title="Installa l'app Claude in Slack">
    Un amministratore del workspace deve installare l'app Claude dal Slack App Marketplace. Visita il [Slack App Marketplace](https://slack.com/marketplace/A08SF47R6P4) e fai clic su "Add to Slack" per iniziare il processo di installazione.
  </Step>

  <Step title="Connetti il tuo account Claude">
    Dopo l'installazione dell'app, autentica il tuo account Claude individuale:

    1. Apri l'app Claude in Slack facendo clic su "Claude" nella tua sezione App
    2. Apri la scheda App Home
    3. Fai clic su "Connect" per collegare il tuo account Slack al tuo account Claude
    4. Completa il flusso di autenticazione nel tuo browser
  </Step>

  <Step title="Configura le sessioni cloud">
    Assicurati che le sessioni cloud siano configurate correttamente per il tuo account:

    * Visita [claude.ai/code](https://claude.ai/code) e accedi con lo stesso account che hai connesso a Slack
    * Connetti il tuo account GitHub se non è già connesso
    * Autentica almeno un repository con cui desideri che Claude lavori
  </Step>

  <Step title="Scegli la tua modalità di instradamento">
    Dopo aver connesso i tuoi account, configura come Claude gestisce i tuoi messaggi in Slack. Apri la App Home di Claude in Slack per trovare l'impostazione **Routing Mode**.

    | Modalità        | Comportamento                                                                                                                                                                                                                                                    |
    | :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | **Code only**   | Claude instrada tutte le @mention a sessioni Claude Code. Ideale per i team che utilizzano Claude in Slack esclusivamente per compiti di sviluppo.                                                                                                               |
    | **Code + Chat** | Claude analizza ogni messaggio e instrada intelligentemente tra Claude Code (per compiti di codifica) e Claude Chat (per scrittura, analisi e domande generali). Ideale per i team che desiderano un unico punto di ingresso @Claude per tutti i tipi di lavoro. |

    <Note>
      In modalità Code + Chat, se Claude instrada un messaggio a Chat ma desideravi una sessione di codifica, puoi fare clic su "Retry as Code" per creare una sessione Claude Code. Allo stesso modo, se viene instradato a Code ma desideravi una sessione Chat, puoi scegliere quell'opzione in quel thread.
    </Note>
  </Step>

  <Step title="Aggiungi Claude ai canali">
    Claude non viene aggiunto automaticamente a nessun canale dopo l'installazione. Per utilizzare Claude in un canale, invitalo digitando `/invite @Claude` in quel canale. Claude può rispondere solo alle @mention nei canali in cui è stato aggiunto.
  </Step>
</Steps>

<h2 id="how-it-works">
  Come funziona
</h2>

<h3 id="automatic-detection">
  Rilevamento automatico
</h3>

In modalità Code + Chat routing, quando menzioni @Claude in un canale o thread Slack, Claude rileva automaticamente se il tuo messaggio è un compito di codifica. I compiti di codifica vanno a una sessione cloud Claude Code. Tutto il resto riceve una risposta chat regolare. In modalità Code only, ogni @mention va a Claude Code.

Puoi anche dire esplicitamente a Claude di gestire una richiesta come un compito di codifica, anche se non lo rileva automaticamente.

<Note>
  Claude Code in Slack funziona solo nei canali (pubblici o privati). Non funziona nei messaggi diretti (DM).
</Note>

<h3 id="context-gathering">
  Raccolta del contesto
</h3>

**Da thread**: Quando @menzioni Claude in un thread, raccoglie il contesto da tutti i messaggi in quel thread per comprendere la conversazione completa.

**Da canali**: Quando menzionato direttamente in un canale, Claude guarda i messaggi recenti del canale per il contesto rilevante.

Questo contesto aiuta Claude a comprendere il problema, selezionare il repository appropriato e informare il suo approccio al compito.

<Warning>
  Quando @Claude viene invocato in Slack, a Claude viene dato accesso al contesto della conversazione per comprendere meglio la tua richiesta. Claude può seguire le indicazioni da altri messaggi nel contesto, quindi gli utenti dovrebbero assicurarsi di utilizzare Claude solo in conversazioni Slack affidabili.
</Warning>

<h3 id="session-flow">
  Flusso della sessione
</h3>

1. **Avvio**: Menzioni @Claude con una richiesta di codifica
2. **Rilevamento**: Claude analizza il tuo messaggio e rileva l'intento di codifica
3. **Creazione della sessione**: Una nuova sessione Claude Code viene creata su claude.ai/code
4. **Aggiornamenti di avanzamento**: Claude pubblica aggiornamenti di stato nel tuo thread Slack mentre il lavoro progredisce
5. **Completamento**: Al termine, Claude ti @menziona con un riepilogo e pulsanti di azione
6. **Revisione**: Fai clic su "View Session" per vedere la trascrizione completa, o "Create PR" per aprire una pull request

<h2 id="user-interface-elements">
  Elementi dell'interfaccia utente
</h2>

<h3 id="message-actions">
  Azioni sui messaggi
</h3>

* **View Session**: Apre la sessione Claude Code completa nel tuo browser dove puoi vedere tutto il lavoro eseguito, continuare la sessione o fare richieste aggiuntive.
* **Create PR**: Crea una pull request direttamente dalle modifiche della sessione.
* **Retry as Code**: Se Claude inizialmente risponde come assistente chat ma desideravi una sessione di codifica, fai clic su questo pulsante per riprovare la richiesta come un compito Claude Code.
* **Change Repo**: Ti consente di selezionare un repository diverso se Claude ha scelto in modo errato.

<h3 id="repository-selection">
  Selezione del repository
</h3>

Claude seleziona automaticamente un repository in base al contesto della tua conversazione Slack. Se più repository potrebbero applicarsi, Claude potrebbe visualizzare un menu a discesa che ti consente di scegliere quello corretto.

<h2 id="access-and-permissions">
  Accesso e autorizzazioni
</h2>

<h3 id="user-level-access">
  Accesso a livello di utente
</h3>

| Tipo di accesso               | Requisito                                                                       |
| :---------------------------- | :------------------------------------------------------------------------------ |
| Sessioni Claude Code          | Ogni utente esegue sessioni con il proprio account Claude                       |
| Utilizzo e limiti di velocità | Le sessioni contano rispetto ai limiti del piano del singolo utente             |
| Accesso al repository         | Gli utenti possono accedere solo ai repository che hanno personalmente connesso |
| Cronologia sessioni           | Le sessioni appaiono nella tua cronologia Claude Code su claude.ai/code         |

<h3 id="workspace-level-access">
  Accesso a livello di workspace
</h3>

Gli amministratori del workspace Slack controllano se l'app Claude è disponibile nel loro workspace:

| Controllo                     | Descrizione                                                                                                                                    |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Installazione dell'app        | Gli amministratori del workspace decidono se installare l'app Claude dal Slack App Marketplace                                                 |
| Distribuzione Enterprise Grid | Per le organizzazioni Enterprise Grid, gli amministratori dell'organizzazione possono controllare quali workspace hanno accesso all'app Claude |
| Rimozione dell'app            | La rimozione dell'app da un workspace revoca immediatamente l'accesso per tutti gli utenti in quel workspace                                   |

<h3 id="channel-based-access-control">
  Controllo dell'accesso basato su canale
</h3>

L'installazione dell'app non aggiunge Claude a nessun canale. Claude risponde alle @mention solo nei canali in cui è stato aggiunto; invitalo con `/invite @Claude`. Funziona sia nei canali pubblici che privati. Gli amministratori possono controllare chi utilizza Claude Code gestendo quali canali Claude viene invitato e chi ha accesso a quei canali. Questo aggiunge un ulteriore livello di controllo dell'accesso oltre alle autorizzazioni a livello di workspace.

<h2 id="what’s-accessible-where">
  Cosa è accessibile dove
</h2>

**In Slack**: Vedrai aggiornamenti di stato, riepiloghi di completamento e pulsanti di azione. La trascrizione completa è preservata e sempre accessibile.

**Su claude.ai/code**: La sessione Claude Code completa con la cronologia della conversazione completa, tutte le modifiche al codice e le operazioni su file. Le sessioni rimangono nella tua cronologia Claude Code su [claude.ai/code](https://claude.ai/code), dove puoi continuare le sessioni passate, farvi riferimento o creare pull request.

Per gli account Enterprise e Team, le sessioni create da Claude in Slack sono automaticamente visibili all'organizzazione. Vedi [condivisione di sessioni](/docs/it/claude-code-on-the-web#share-sessions) per ulteriori dettagli.

<h2 id="best-practices">
  Best practices
</h2>

<h3 id="writing-effective-requests">
  Scrivere richieste efficaci
</h3>

* **Sii specifico**: Includi nomi di file, nomi di funzioni o messaggi di errore quando rilevante.
* **Fornisci contesto**: Menziona il repository o il progetto se non è chiaro dalla conversazione.
* **Definisci il successo**: Spiega come dovrebbe apparire "fatto"—Claude dovrebbe scrivere test? Aggiornare la documentazione? Creare una PR?
* **Usa thread**: Rispondi nei thread quando discuti di bug o funzionalità in modo che Claude possa raccogliere il contesto completo.

<h3 id="when-to-use-slack-vs-web">
  Quando utilizzare Slack rispetto al web
</h3>

**Usa Slack quando**: Il contesto esiste già in una discussione Slack, desideri avviare un compito in modo asincrono, o stai collaborando con compagni di team che hanno bisogno di visibilità.

**Usa il web direttamente quando**: Hai bisogno di caricare file, desideri un'interazione in tempo reale durante lo sviluppo, o stai lavorando su compiti più lunghi e complessi.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  "Claude Code is not enabled for your account"
</h3>

Questo errore significa che il tuo account Claude non ha ancora un ambiente cloud. Accedi a [claude.ai/code](https://claude.ai/code) una volta con lo stesso account che hai collegato a Slack e completa l'[onboarding web](/docs/it/web-quickstart#connect-github), che crea il tuo ambiente cloud predefinito o ti chiede di crearlo. L'errore si risolve alla prossima menzione. Ogni utente deve farlo individualmente.

<h3 id="sessions-not-starting">
  Sessions not starting
</h3>

1. Verifica che il tuo account Claude sia collegato in Claude App Home
2. Controlla che le sessioni cloud siano abilitate per il tuo account
3. Assicurati di avere almeno un repository GitHub collegato a Claude Code

<h3 id="sessions-from-a-claude-tag-channel-fail-to-start">
  Sessions from a Claude Tag channel fail to start
</h3>

Questa voce si applica agli spazi di lavoro che utilizzano [Claude Tag](https://claude.com/docs/claude-tag/overview), dove Claude lavora nei canali come identità condivisa della tua organizzazione, non come account di nessun membro. Se hai creato l'ambiente cloud del canale su [claude.ai/code](https://claude.ai/code), appartiene al tuo account personale e Claude non può avviare sessioni di canale in un ambiente personale. Claude Code interrompe immediatamente la sessione e riprovare non aiuta.

Se sei un Owner e l'ambiente è il tuo, [condividilo con l'organizzazione](/docs/it/cloud-environments#organization-shared-environments) dal selettore dell'ambiente. Altrimenti, un Owner lo ricrea come ambiente condiviso dell'organizzazione dalla pagina **Cloud environments** in [admin settings](https://claude.ai/admin-settings).

Puoi applicarlo in due modi:

* Impostalo come predefinito dell'organizzazione su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).
* [Impostalo sul canale](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) nelle impostazioni di amministrazione di Claude Tag.

Se non sei un Owner, invia questa voce a uno.

<h3 id="repository-not-showing">
  Repository not showing
</h3>

1. Collega il repository su [claude.ai/code](https://claude.ai/code)
2. Verifica i tuoi permessi GitHub per quel repository
3. Prova a disconnettere e riconnettere il tuo account GitHub

<h3 id="wrong-repository-selected">
  Wrong repository selected
</h3>

1. Fai clic sul pulsante "Change Repo" per selezionare un repository diverso
2. Includi il nome del repository nella tua richiesta per una selezione più accurata

<h3 id="authentication-errors">
  Authentication errors
</h3>

1. Disconnetti e riconnetti il tuo account Claude in App Home
2. Assicurati di aver effettuato l'accesso all'account Claude corretto nel tuo browser
3. Controlla che il tuo piano Claude includa l'accesso a Claude Code

<h2 id="current-limitations">
  Limitazioni attuali
</h2>

* **Solo GitHub**: i repository devono essere su GitHub.
* **Una PR alla volta**: ogni sessione può creare una pull request.
* **Accesso cloud sessions richiesto**: gli utenti devono avere accesso a [cloud sessions](/docs/it/claude-code-on-the-web); senza di esso, Claude risponde con risposte di chat standard.

<h2 id="related-resources">
  Risorse correlate
</h2>

<CardGroup>
  <Card title="Claude Code nel cloud" icon="cloud" href="/docs/it/claude-code-on-the-web">
    Scopri di più sulle sessioni cloud
  </Card>

  <Card title="Claude for Slack" icon="slack" href="https://claude.com/claude-and-slack">
    Documentazione generale di Claude for Slack
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    @Claude gestito dall'organizzazione in Slack con accesso configurato dall'amministratore
  </Card>

  <Card title="Slack App Marketplace" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    Installa l'app Claude dal Marketplace di Slack
  </Card>

  <Card title="Claude Help Center" icon="circle-question" href="https://support.claude.com">
    Ottieni supporto aggiuntivo
  </Card>
</CardGroup>
