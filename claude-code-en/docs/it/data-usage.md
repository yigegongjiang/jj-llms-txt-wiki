> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utilizzo dei dati

> Scopri le politiche di utilizzo dei dati di Anthropic per Claude

<h2 id="data-policies">
  Politiche sui dati
</h2>

<h3 id="data-training-policy">
  Politica di addestramento dei dati
</h3>

**Utenti consumer (piani Free, Pro e Max)**:
Vi diamo la possibilità di consentire l'utilizzo dei vostri dati per migliorare i futuri modelli Claude. Addestreremo nuovi modelli utilizzando i dati degli account Free, Pro e Max quando questa impostazione è attiva (incluso quando utilizzate Claude Code da questi account).

**Utenti commerciali**: (piani Team ed Enterprise, API, piattaforme di terze parti e Claude Gov) mantengono le politiche esistenti: Anthropic non addestra modelli generativi utilizzando codice o prompt inviati a Claude Code secondo i termini commerciali, a meno che il cliente non abbia scelto di fornirci i propri dati per il miglioramento del modello (ad esempio, il [Development Partner Program](https://support.claude.com/it/articles/11174108-about-the-development-partner-program)).

<h3 id="development-partner-program">
  Development Partner Program
</h3>

Se vi iscrivete esplicitamente a metodi per fornirci materiali su cui addestrare, come tramite il [Development Partner Program](https://support.claude.com/it/articles/11174108-about-the-development-partner-program), potremmo utilizzare tali materiali forniti per addestrare i nostri modelli. Un amministratore dell'organizzazione può iscriversi esplicitamente al Development Partner Program per la propria organizzazione. Si noti che questo programma è disponibile solo per l'API di Anthropic di prima parte e non per gli utenti di Amazon Bedrock o della piattaforma agenti di Google Cloud.

<h3 id="feedback-using-the-/feedback-command">
  Feedback utilizzando il comando `/feedback`
</h3>

Se scegliete di inviarci feedback su Claude Code utilizzando il comando `/feedback`, potremmo utilizzare il vostro feedback per migliorare i nostri prodotti e servizi. I transcript condivisi tramite `/feedback` vengono conservati per 5 anni.

<h3 id="session-quality-surveys">
  Sondaggi sulla qualità della sessione
</h3>

Quando vedete il prompt "Come sta andando Claude in questa sessione?" in Claude Code, rispondendo a questo sondaggio, inclusa la selezione di "Ignora", viene registrato solo il vostro voto. Non raccogliamo né archiviamo alcun transcript di conversazione, input, output o altri dati di sessione come parte del prompt di valutazione stesso. A differenza del feedback con pollice su/giù o dei report `/feedback`, questo sondaggio sulla qualità della sessione è una semplice metrica di soddisfazione del prodotto.

Dopo il prompt di valutazione, potete vedere una domanda di follow-up separata che chiede "Anthropic può guardare il transcript della vostra sessione per aiutarci a migliorare Claude Code?". Questo è un secondo passaggio facoltativo distinto dalla valutazione:

* **Sì**: carica il transcript della vostra conversazione, i transcript di qualsiasi subagent e il file di log della sessione non elaborato dal disco su Anthropic. I modelli di chiave API e token noti vengono oscurati prima del caricamento. Il codice sorgente, i contenuti dei file e altri contenuti della conversazione vengono caricati così come sono. I transcript condivisi vengono conservati fino a 6 mesi. Su Amazon Bedrock, la piattaforma agenti di Google Cloud, Microsoft Foundry e sessioni del [gateway app Claude](/docs/it/claude-apps-gateway) con accesso effettuato, Sì scrive lo stesso payload in un archivio locale sotto `~/.claude/feedback-bundles/` invece di caricare; nulla lascia la vostra macchina finché non inoltrate quel file.
* **No**: rifiuta senza inviare nulla
* **Non chiedere più**: rifiuta e impedisce che questo follow-up appaia nelle sessioni future

Nulla viene caricato a meno che non selezioniate esplicitamente **Sì**. Le organizzazioni con [zero data retention](/docs/it/zero-data-retention), o dove il feedback sui prodotti è disabilitato dalla politica dell'organizzazione, o dove `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` è impostato, non vedono mai questo follow-up. Le vostre risposte a questo sondaggio, inclusi i transcript delle sessioni inviati dopo il prompt di valutazione, non influiscono sulle vostre preferenze di addestramento dei dati e non possono essere utilizzate per addestrare i nostri modelli di IA.

Per disabilitare questi sondaggi, impostate `CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1`. Il sondaggio viene anche disabilitato quando `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` è impostato. Le organizzazioni che bloccano il traffico non essenziale ma acquisiscono le risposte ai sondaggi tramite il loro [OpenTelemetry collector](/docs/it/monitoring-usage) possono ripristinare il sondaggio impostando `CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL=1`. Il sondaggio registra quindi le valutazioni solo al collector configurato. Il follow-up di condivisione dei transcript e tutto il resto del traffico di feedback legato ad Anthropic rimangono disabilitati. Per controllare la frequenza invece di disabilitare, impostate [`feedbackSurveyRate`](/docs/it/settings#available-settings) nel vostro file di impostazioni su una probabilità tra `0` e `1`.

<h3 id="data-retention">
  Conservazione dei dati
</h3>

Anthropic conserva i dati di Claude Code in base al tipo di account e alle preferenze dell'utente.

**Utenti consumer (piani Free, Pro e Max)**:

* Utenti che consentono l'utilizzo dei dati per il miglioramento del modello: periodo di conservazione di 5 anni per supportare lo sviluppo del modello e i miglioramenti della sicurezza
* Utenti che non consentono l'utilizzo dei dati per il miglioramento del modello: periodo di conservazione di 30 giorni
* Le impostazioni sulla privacy possono essere modificate in qualsiasi momento su [claude.ai/settings/data-privacy-controls](https://claude.ai/settings/data-privacy-controls).

**Utenti commerciali (Team, Enterprise e API)**:

* Standard: periodo di conservazione di 30 giorni
* [Zero data retention](/docs/it/zero-data-retention): disponibile per Claude Code su Claude for Enterprise. ZDR non è incluso nel piano Enterprise standard; viene abilitato su base per organizzazione dal vostro team di account dopo aver confermato l'idoneità
* Caching locale: i client di Claude Code archiviano i transcript delle sessioni localmente in testo semplice sotto `~/.claude/projects/` per 30 giorni per impostazione predefinita per abilitare la ripresa della sessione. Regolate il periodo con `cleanupPeriodDays`. Consultate [application data](/docs/it/claude-directory#application-data) per sapere cosa viene archiviato e come cancellarlo.

Potete eliminare le singole sessioni di Claude Code sul web in qualsiasi momento. L'eliminazione di una sessione rimuove permanentemente i dati dell'evento della sessione. Per istruzioni su come eliminare le sessioni, consultate [Eliminare le sessioni](/docs/it/claude-code-on-the-web#delete-sessions).

Scopri di più sulle pratiche di conservazione dei dati nel nostro [Privacy Center](https://privacy.anthropic.com/).

Per i dettagli completi, consultate i nostri [Termini di servizio commerciali](https://www.anthropic.com/legal/commercial-terms) (per gli utenti Team, Enterprise e API) o [Termini consumer](https://www.anthropic.com/legal/consumer-terms) (per gli utenti Free, Pro e Max) e [Informativa sulla privacy](https://www.anthropic.com/legal/privacy).

<h2 id="data-access">
  Accesso ai dati
</h2>

Per tutti gli utenti di prima parte, potete scoprire di più su quali dati vengono registrati per [Claude Code locale](#local-claude-code-data-flow-and-dependencies) e [Claude Code remoto](#cloud-execution-data-flow-and-dependencies). Le sessioni di [Remote Control](/docs/it/remote-control) seguono il flusso di dati locale poiché tutta l'esecuzione avviene sulla vostra macchina; mentre connessi, la trascrizione della sessione viene anche archiviata sui server di Anthropic per sincronizzare la conversazione tra i dispositivi, come descritto in [Connection and security](/docs/it/remote-control#connection-and-security). Si noti che per Claude Code remoto, Claude accede al repository in cui avviate la vostra sessione di Claude Code. Claude non accede ai repository che avete collegato ma in cui non avete avviato una sessione.

<h2 id="local-claude-code-data-flow-and-dependencies">
  Claude Code locale: flusso di dati e dipendenze
</h2>

Il diagramma sottostante mostra come Claude Code si connette ai servizi esterni durante l'installazione e il funzionamento normale. Le linee continue indicano connessioni richieste, mentre le linee tratteggiate rappresentano flussi di dati facoltativi o avviati dall'utente.

<img src="https://mintcdn.com/claude-code/YR4DRZyI3CdsXkiT/images/claude-code-data-flow.svg?fit=max&auto=format&n=YR4DRZyI3CdsXkiT&q=85&s=2846ea92cfc2297b8620c31c82b482ad" alt="Diagramma che mostra le connessioni esterne di Claude Code: install/update si connette al server di distribuzione, e le richieste dell'utente si connettono a Console auth e public-api di Anthropic, con flussi di telemetria facoltativi che trasportano metriche e rapporti di errore ad Anthropic e servizi di terze parti. Il feedback inviato con /feedback va a Google Cloud Storage e facoltativamente crea un problema GitHub" width="720" height="520" data-path="images/claude-code-data-flow.svg" />

Claude Code viene eseguito localmente. Per interagire con l'LLM, Claude Code invia dati sulla rete. Questi dati includono tutti i prompt dell'utente e gli output del modello, crittografati in transito tramite TLS 1.2+. Claude Code è compatibile con la maggior parte dei VPN e dei proxy LLM più diffusi.

La crittografia a riposo dipende dal vostro provider di modelli:

| Provider                      | Crittografia a riposo                                                                                                                                       |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Anthropic API                 | Crittografia del disco a livello di infrastruttura (AES-256). Abilitate [Zero Data Retention](/docs/it/zero-data-retention) per nessuna persistenza lato server. |
| Amazon Bedrock                | AES-256 con chiavi gestite da AWS. Chiavi gestite dal cliente disponibili tramite AWS KMS.                                                                  |
| Google Cloud's Agent Platform | Chiavi di crittografia gestite da Google. CMEK disponibile.                                                                                                 |
| Microsoft Foundry             | Le richieste vengono instradate all'infrastruttura Anthropic con crittografia del disco AES-256.                                                            |

Claude Code è costruito sulle API di Anthropic. Per i dettagli sui controlli di sicurezza della nostra API, incluse le nostre procedure di registrazione dell'API, consultate gli artefatti di conformità offerti nel [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="cloud-execution-data-flow-and-dependencies">
  Esecuzione nel cloud: flusso di dati e dipendenze
</h3>

Quando si utilizza [Claude Code sul web](/docs/it/claude-code-on-the-web), le sessioni vengono eseguite in macchine virtuali gestite da Anthropic invece che localmente. Negli ambienti cloud:

* **Archiviazione di codice e dati:** Il vostro repository viene clonato su una VM isolata. Il codice e i dati della sessione sono soggetti alle politiche di conservazione e utilizzo per il vostro tipo di account (consultate la sezione Conservazione dei dati sopra)
* **Credenziali:** L'autenticazione GitHub viene gestita tramite un proxy sicuro; le vostre credenziali GitHub non entrano mai nella sandbox
* **Traffico di rete:** Tutto il traffico in uscita passa attraverso un proxy di sicurezza per la registrazione di audit e la prevenzione degli abusi
* **Dati della sessione:** I prompt, le modifiche al codice e gli output seguono le stesse politiche sui dati dell'utilizzo locale di Claude Code

Per i dettagli sulla sicurezza dell'esecuzione nel cloud, consultate [Sicurezza](/docs/it/security#cloud-execution-security).

<h2 id="telemetry-services">
  Servizi di telemetria
</h2>

Claude Code invia due tipi di telemetria operativa: metriche di utilizzo e rapporti di errore. Potete disattivare ciascuno individualmente con le variabili di ambiente di seguito, oppure disabilitare tutto il traffico non essenziale contemporaneamente impostando `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`.

**Metriche**: latenza, affidabilità e modelli di utilizzo, inviati ad Anthropic e all'infrastruttura di registrazione di terze parti su TLS. Le metriche non includono mai il vostro codice, i prompt o i percorsi dei file. Impostate `DISABLE_TELEMETRY=1` per rinunciare.

**Rapporti di errore**: messaggi di errore e stack trace dagli interni di Claude Code, inviati a un servizio di tracciamento degli errori di terze parti su TLS. Claude Code oscura i modelli noti di segreti, percorsi di file, indirizzi email e altre informazioni personali prima che qualsiasi cosa lasci la vostra macchina. Impostate `DISABLE_ERROR_REPORTING=1` per rinunciare.

La registrazione degli errori è attiva solo quando si applicano tutte queste condizioni:

* vi accedete con un abbonamento Claude Pro o Max
* state eseguendo Claude Code v2.1.198 o versione successiva
* vi state connettendo direttamente all'API Claude
* la vostra organizzazione non ha un accordo di conservazione dati zero o HIPAA

Quando eseguite il comando `/feedback`, una copia della cronologia della conversazione incluso il codice viene inviata ad Anthropic. Prima di inviare, scegliete quanta cronologia includere: la sessione corrente soltanto, che è l'impostazione predefinita, oppure anche altre sessioni dello stesso progetto negli ultimi 24 ore o 7 giorni. I dati vengono crittografati in transito via TLS e archiviati in Google Cloud Storage, che crittografa i dati archiviati a riposo per impostazione predefinita. Facoltativamente, viene creato un problema GitHub nel repository pubblico. Per rinunciare, impostate la variabile di ambiente `DISABLE_FEEDBACK_COMMAND` su `1`.

Quando utilizzate un provider di terze parti come Amazon Bedrock o Google Cloud's Agent Platform, oppure non avete credenziali Anthropic configurate, `/feedback` scrive il rapporto in un archivio locale sotto `~/.claude/feedback-bundles/` invece di inviarlo ad Anthropic. I modelli di chiave API e token noti vengono oscurati prima che l'archivio venga scritto. Nulla lascia la vostra macchina finché non inviate quel file al vostro rappresentante dell'account Anthropic o lo allegate a una richiesta di supporto.

<h2 id="default-behaviors-by-api-provider">
  Comportamenti predefiniti per provider API
</h2>

Per impostazione predefinita, la segnalazione degli errori, la telemetria e la segnalazione dei bug sono disabilitati quando si utilizza Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o Claude Platform su AWS. I sondaggi sulla qualità della sessione e il controllo di sicurezza del dominio WebFetch sono eccezioni e vengono eseguiti indipendentemente dal provider. Su una sessione [gateway app Claude](/docs/it/claude-apps-gateway) con accesso effettuato, l'analisi dell'utilizzo, la segnalazione degli errori e i rating dei sondaggi ad Anthropic sono disabilitati dalle credenziali del gateway stesso, senza alcuna impostazione per riattivarli. Potete rinunciare a tutto il traffico non essenziale, inclusi i sondaggi, contemporaneamente impostando `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Questa variabile non influisce sul controllo WebFetch, che ha il suo proprio opt-out. Ecco i comportamenti predefiniti completi:

| Servizio                                        | Claude API                                                                                                            | Google Cloud's Agent Platform API                                                                                     | Amazon Bedrock API                                                                                                    | Microsoft Foundry API                                                                                                 | Claude Platform su AWS                                                                                                |
| ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| **Metriche**                                    | Attivo per impostazione predefinita.<br />`DISABLE_TELEMETRY=1` per disabilitare.                                     | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_VERTEX` deve essere 1.                                  | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_BEDROCK` deve essere 1.                                 | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_FOUNDRY` deve essere 1.                                 | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` deve essere 1.                           |
| **Segnalazioni di errore**                      | Attivo per accessi Pro e Max su v2.1.198+, altrimenti disattivo.<br />`DISABLE_ERROR_REPORTING=1` per disabilitare.   | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_VERTEX` deve essere 1.                                  | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_BEDROCK` deve essere 1.                                 | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_FOUNDRY` deve essere 1.                                 | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` deve essere 1.                           |
| **Claude API (report `/feedback`)**             | Attivo per impostazione predefinita.<br />`DISABLE_FEEDBACK_COMMAND=1` per disabilitare.                              | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_VERTEX` deve essere 1.                                  | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_BEDROCK` deve essere 1.                                 | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_FOUNDRY` deve essere 1.                                 | Disattivo per impostazione predefinita.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` deve essere 1.                           |
| **Sondaggi sulla qualità della sessione**       | Attivo per impostazione predefinita.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` per disabilitare.                   | Attivo per impostazione predefinita.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` per disabilitare.                   | Attivo per impostazione predefinita.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` per disabilitare.                   | Attivo per impostazione predefinita.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` per disabilitare.                   | Attivo per impostazione predefinita.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` per disabilitare.                   |
| **Controllo di sicurezza del dominio WebFetch** | Attivo per impostazione predefinita.<br />`skipWebFetchPreflight: true` in [settings](/docs/it/settings) per disabilitare. | Attivo per impostazione predefinita.<br />`skipWebFetchPreflight: true` in [settings](/docs/it/settings) per disabilitare. | Attivo per impostazione predefinita.<br />`skipWebFetchPreflight: true` in [settings](/docs/it/settings) per disabilitare. | Attivo per impostazione predefinita.<br />`skipWebFetchPreflight: true` in [settings](/docs/it/settings) per disabilitare. | Attivo per impostazione predefinita.<br />`skipWebFetchPreflight: true` in [settings](/docs/it/settings) per disabilitare. |

Tutte le variabili di ambiente possono essere controllate in `settings.json` (consultate [riferimento delle impostazioni](/docs/it/settings)).

A partire dalla v2.1.126, quando una piattaforma host imposta `CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`, le metriche sono attive per impostazione predefinita per Google Cloud's Agent Platform, Amazon Bedrock e Microsoft Foundry, e seguono l'opt-out standard `DISABLE_TELEMETRY`. La segnalazione degli errori e i report `/feedback` rimangono disattivi per impostazione predefinita su questi provider.

<h3 id="webfetch-domain-safety-check">
  Controllo di sicurezza del dominio WebFetch
</h3>

Prima di recuperare un URL, lo strumento WebFetch invia il nome host richiesto a `api.anthropic.com` per verificarlo rispetto a un elenco di blocco della sicurezza mantenuto da Anthropic. Viene inviato solo il nome host, non l'URL completo, il percorso o il contenuto della pagina. I risultati vengono memorizzati nella cache per nome host per cinque minuti.

Questo controllo viene eseguito indipendentemente da quale provider di modelli utilizzate e non è influenzato da `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Se la vostra rete blocca `api.anthropic.com`, le richieste WebFetch non riescono finché non consentite il dominio o non impostate `skipWebFetchPreflight: true` in [settings](/docs/it/settings). La disabilitazione del controllo significa che WebFetch tenta di recuperare qualsiasi URL senza consultare l'elenco di blocco, quindi combinatelo con le [regole di autorizzazione `WebFetch`](/docs/it/permissions#webfetch) se dovete limitare quali domini Claude può raggiungere.
