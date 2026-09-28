> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare ambienti cloud

> Configurare ambienti cloud per le sessioni cloud di Claude Code: livelli di accesso di rete, variabili di ambiente, script di configurazione e caching dell'ambiente.

<Note>
  Gli ambienti cloud si applicano alle [sessioni cloud](/docs/it/claude-code-on-the-web), che sono disponibili sui piani Pro, Max e Team, e per gli utenti Enterprise con [posti premium o posti Chat + Claude Code](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan).
</Note>

Ogni [sessione cloud](/docs/it/claude-code-on-the-web) viene eseguita in un ambiente cloud. È possibile configurare un ambiente per consentire o negare l'[accesso di rete](#access-levels), [impostare variabili di ambiente](#set-environment-variables) per la sessione, sui piani Pro e Max memorizzare [credenziali API](#add-api-credentials) che le sessioni utilizzano senza vederle, ed eseguire uno [script di configurazione](#setup-scripts) prima che Claude inizi a lavorare.

Gli stessi ambienti si applicano ovunque avviate una sessione cloud: l'[app Desktop](/docs/it/desktop), l'[app mobile Claude](/docs/it/mobile), il vostro browser su [claude.ai/code](https://claude.ai/code), il terminale con [`claude --cloud`](/docs/it/claude-code-on-the-web#from-terminal-to-cloud), [routine](/docs/it/routines) e [Claude Tag](https://claude.com/docs/claude-tag/overview). Ognuna di queste superfici può anche instradare a un [ambiente self-hosted](/docs/it/self-hosted-environments). [Disponibilità e limitazioni](/docs/it/self-hosted-environments#availability-and-limitations) copre cosa Claude non può ancora utilizzare quando una sessione Claude Tag viene eseguita in uno.

<Info>
  Le sessioni di [Remote Control](/docs/it/remote-control) collegano le interfacce web e mobile a una sessione sulla vostra macchina, che utilizza la rete e i file della vostra macchina, non un ambiente cloud. Le sessioni del canale Claude Tag utilizzano solo ambienti a livello di organizzazione, sia [ambienti condivisi](#organization-shared-environments) che [ambienti self-hosted](/docs/it/self-hosted-environments).
</Info>

<h2 id="the-default-environment">
  L'ambiente Default
</h2>

Se non avete ancora un ambiente, l'onboarding configura l'ambiente **Default** per voi. Come dipende da dove eseguite l'onboarding:

* **Flussi CLI come `/web-setup`**: creano **Default** per voi
* **Onboarding web su Pro e Max**: crea **Default** per voi
* **Onboarding web su Team ed Enterprise**: mostra un modulo **Create your first cloud environment** a meno che un Owner non abbia attivato [Quick web setup](/docs/it/claude-code-on-the-web#github-authentication-options); mantenete i valori predefiniti del modulo e fate clic su **Create & finish** per ottenere lo stesso ambiente **Default**

**Default** non ha alcuna configurazione propria:

* [Accesso di rete **Trusted**](#access-levels): le sessioni raggiungono i registri dei pacchetti e altri [domini consentiti](#default-allowed-domains), e nient'altro attraverso la rete della sessione.
* Nessun'altra configurazione: **Default** non definisce variabili di ambiente o script di configurazione, quindi le sessioni iniziano con solo gli [strumenti preinstallati](#installed-tools).

Con solo **Default** disponibile, ogni sessione viene eseguita in esso. Quando si dispone di più di un ambiente, le sessioni ne scelgono uno per superficie:

* Nell'app Desktop, nell'app mobile e su claude.ai/code, le sessioni che avviate voi stessi utilizzano l'ambiente mostrato nel [selettore](#configure-your-environment). Un [default dell'organizzazione](#organization-shared-environments) impostato da un Owner riempie la selezione quando non ne avete scelto uno. I thread in un [progetto](/docs/it/claude-projects#project-settings-reference) utilizzano invece l'ambiente impostato nelle impostazioni del progetto.
* Dalla CLI, Claude Code utilizza la vostra scelta [`/remote-env`](#select-an-environment-from-the-cli), o ricade nell'ambiente ospitato da Anthropic quando il vostro elenco ne ha uno, e altrimenti nel primo ambiente nel vostro elenco che non è un ambiente bridge, una voce [Remote Control](/docs/it/remote-control) che registra per rappresentare la vostra macchina piuttosto che un ambiente cloud. Per un [ambiente self-hosted](/docs/it/self-hosted-environments), passare `--environment <environment-id>` con il suo ID `ccpool_` [quando inviate una sessione](/docs/it/self-hosted-environments-testing#run-the-test-loop) sostituisce la scelta `/remote-env` e il fallback per quella invocazione. Claude Code rifiuta gli ID `env_` ospitati da Anthropic passati al flag, quindi utilizzate `/remote-env` per indirizzare quelli. Il flag richiede Claude Code v2.1.224 o successiva.

Configurate un ambiente quando il default non è sufficiente: quando Claude ha bisogno di raggiungere domini al di fuori della [lista di consentiti predefinita](#default-allowed-domains), ha bisogno di variabili di ambiente impostate per le sue sessioni, o ha bisogno di dipendenze installate prima di iniziare a lavorare.

<h2 id="configure-your-environment">
  Configurare il vostro ambiente
</h2>

Create, modificate e archiviate gli ambienti dal selettore di ambiente, che raggiungete su [claude.ai/code](https://claude.ai/code) dopo l'[onboarding web](/docs/it/web-quickstart), oppure dalla casella di messaggio nell'[app Desktop](/docs/it/desktop#cloud-sessions). Gli ambienti che create sono personali al vostro account; gli [ambienti condivisi](#organization-shared-environments) creati da un Owner appaiono nello stesso selettore. Consultate [Strumenti installati](#installed-tools) per vedere cosa è disponibile senza alcuna configurazione.

<Steps>
  <Step title="Aprire il selettore di ambiente">
    Su [claude.ai/code](https://claude.ai/code), selezionate l'icona cloud che mostra il nome dell'ambiente corrente, nella riga sopra la casella di messaggio. Non c'è una pagina di impostazioni o un URL diretto per il selettore.

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-selector.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=cc2813a5664519eaf5a89d793ce5af26" alt="Il selettore di ambiente aperto sopra la casella di messaggio su claude.ai/code. Il pulsante cloud che mostra il nome dell'ambiente Default si trova nella riga sopra la casella di messaggio. Il menu aperto elenca una riga Local con etichette Download e Desktop only, una sezione Cloud dove l'ambiente Default è selezionato con un segno di spunta e mostra un'icona di ingranaggio delle impostazioni al passaggio del mouse, un'opzione Add cloud environment e una sezione Remote Control con istruzioni di configurazione." width="1672" height="682" data-path="images/cloud-environment-selector.png" />
    </Frame>
  </Step>

  <Step title="Aggiungere o modificare un ambiente">
    Selezionate **Add cloud environment**, oppure passate il mouse su un ambiente esistente e selezionate l'icona delle impostazioni che appare a destra. La finestra di dialogo include il nome, il livello di accesso di rete, le variabili di ambiente e lo script di configurazione. Quando modificate un ambiente cloud esistente su un piano Pro o Max, la finestra di dialogo include anche [credenziali API](#add-api-credentials).

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-dialog.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=30d4478b31d1f879f7ee287ddab32505" alt="La finestra di dialogo New cloud environment. Un campo Name con il testo segnaposto Default, un selettore Network access impostato su Trusted con link alla politica di rete e ai livelli di accesso, una casella Environment variables che mostra il testo segnaposto in formato .env con una nota che i valori sono visibili a chiunque utilizzi l'ambiente, una casella Setup script descritta come uno script Bash che viene eseguito quando inizia una nuova sessione prima che Claude Code si avvii, e pulsanti Cancel e Create environment." width="874" height="1372" data-path="images/cloud-environment-dialog.png" />
    </Frame>
  </Step>
</Steps>

<h3 id="set-environment-variables">
  Impostare le variabili di ambiente
</h3>

Le variabili di ambiente utilizzano il formato `.env`, una coppia `KEY=value` per riga. I valori semplici non hanno bisogno di virgolette, e se quotate un valore con una coppia corrispondente, le virgolette non diventano parte del valore. Quotate un valore che si estende su più righe o contiene un `#`: in un valore non quotato, `#` inizia un commento e il resto della riga viene eliminato.

L'esempio seguente definisce tre variabili.

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

Ogni sessione copia i valori dell'ambiente una volta, all'avvio, in variabili di ambiente ordinarie che qualsiasi comando eseguito da Claude può leggere. Poiché le sessioni in esecuzione non rileggono la configurazione, la modifica o l'aggiunta di variabili influisce sulle sessioni che avviate in seguito; le sessioni già in esecuzione mantengono i valori con cui sono state avviate.

Una sessione cloud imposta anche alcune variabili da sola quando si avvia. Per [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/it/claude-code-on-the-web#manage-context), il valore che la sessione imposta sostituisce uno che aggiungete qui, quindi aggiungere quella chiave qui non ha effetto.

Chiunque utilizzi l'ambiente può leggere i valori. Sui piani Pro e Max, utilizzate una [credenziale API](#add-api-credentials) invece per una chiave che il proxy dell'agente può allegare a una richiesta. Le [richieste che non ricevono mai una credenziale](#requests-that-never-get-the-credential) sono elencate lì.

<h3 id="add-api-credentials">
  Aggiungere credenziali API
</h3>

Una credenziale API è una chiave API o un token che memorizzate in un ambiente cloud in modo che Claude possa chiamare quell'API da qualsiasi sessione nell'ambiente senza vedere la chiave. Il proxy dell'agente di Anthropic aggiunge la chiave alle richieste per gli host che elencate, dopo che ogni richiesta esce dalla VM della sessione. La chiave non raggiunge mai Claude, i comandi che esegue, o le variabili di ambiente della sessione.

Le credenziali API sono disponibili sui piani Pro e Max. Non sono ancora disponibili sui piani Team o Enterprise, quindi la sezione **API credentials** non appare nella finestra di dialogo dell'ambiente su quei piani.

<h4 id="requirements">
  Requisiti
</h4>

Due di questi decidono se potete aggiungere una credenziale, e due decidono se il proxy dell'agente può utilizzarla una volta aggiunta:

* **Ruolo**: un ruolo di amministratore dell'organizzazione nella vostra organizzazione claude.ai
  * Su Team ed Enterprise, gli Owner lo detengono e gli Admin no
  * Su Pro e Max, lo detenete nella vostra organizzazione personale
  * Senza di esso, vedete una nota invece dell'elenco delle credenziali, anche sui vostri ambienti personali. Chiedete a un Owner di aggiungere la credenziale a un ambiente condiviso ed eseguite le vostre sessioni lì
* **Tipo di ambiente**: un ambiente cloud ospitato da Anthropic che già esiste. Un [ambiente self-hosted](/docs/it/self-hosted-environments) non ha credenziali API
* **Raggiungibilità API**: l'API accetta connessioni da internet, perché le richieste escono dalla rete di Anthropic
* **Chiavi di crittografia**: se la vostra organizzazione utilizza chiavi di crittografia gestite dal cliente, non potete salvare credenziali

<h4 id="add-a-credential">
  Aggiungere una credenziale
</h4>

Aggiungete le credenziali una alla volta dall'editor di un ambiente che già esiste. La finestra di dialogo per un nuovo ambiente non le offre. Non c'è nemmeno modifica. Per modificare gli host o il valore di una credenziale, cancellatela e aggiungetela di nuovo.

<Steps>
  <Step title="Aprire le credenziali API dell'ambiente">
    [Aprite l'ambiente per la modifica](#configure-your-environment) su [claude.ai/code](https://claude.ai/code). Nella finestra di dialogo **Update cloud environment**, trovate **API credentials** sotto **Environment variables**. Vedete le credenziali già sull'ambiente, ognuna con gli host a cui si applica.
  </Step>

  <Step title="Aggiungere la credenziale">
    Selezionate **Add credential** e compilate il modulo. Mantenete il **Credential type** predefinito, **Bearer**, per una chiave API che viaggia in un'intestazione di richiesta, e compilate questi campi:

    * **Name**: un'etichetta per la credenziale, come `Internal billing API`
    * **Allowed websites**: gli host dell'API, come `api.example.com`. Un `*.` iniziale corrisponde a ogni sottodominio
    * **Custom headers**: una riga per l'intestazione che trasporta la chiave. La riga inizia con `Authorization` come **Name** dell'intestazione e `Bearer` come suo **Prefix**; incollate la chiave stessa come **Value**. Per un'intestazione come `X-Api-Key` che accetta il valore nudo, cambiate il nome e cancellate il prefisso

    Per un'API che si autentica in un altro modo, scegliete un **Credential type** diverso. L'elenco è lo stesso che [Claude Tag](https://claude.com/docs/claude-tag/overview), l'integrazione Slack per i piani Team ed Enterprise, offre per le [connessioni](https://claude.com/docs/claude-tag/admins/add-connections).
  </Step>

  <Step title="Salvare la credenziale">
    Selezionate **Connect**. La credenziale appare nell'elenco con i suoi host, salvata senza il pulsante **Save changes** della finestra di dialogo. Non potete visualizzare il valore di nuovo dopo il salvataggio.
  </Step>
</Steps>

Per confermare che la credenziale funziona, avviate una sessione nell'ambiente e chiedete a Claude di chiamare l'API, ad esempio con `curl`. L'API risponde come se la chiave fosse nella richiesta, e la chiave non appare nelle variabili di ambiente della sessione o in nessun file. Se l'elenco contrassegna una credenziale **Not sent**, la nota sotto di essa dice perché e cosa fare. Due credenziali i cui host si sovrappongono senza corrispondere esattamente non ricevono alcun marcatore, e il proxy dell'agente ne invia solo una.

<h4 id="which-requests-get-the-credential">
  Quali richieste ricevono la credenziale
</h4>

Il proxy dell'agente allega una credenziale a una richiesta quando l'host della richiesta corrisponde a uno che avete elencato su quella credenziale. Le sessioni possono raggiungere quegli host anche quando il [livello di accesso di rete](#access-levels) dell'ambiente non lo permetterebbe altrimenti, tranne gli [host che non ricevono mai la credenziale](#requests-that-never-get-the-credential). La credenziale si applica in ogni sessione che viene eseguita nell'ambiente, chiunque l'abbia avviata, finché non la cancellate.

<h4 id="requests-that-never-get-the-credential">
  Richieste che non ricevono mai la credenziale
</h4>

Il proxy dell'agente non allega mai una credenziale che aggiungete a queste richieste:

* **GitHub**: il [proxy GitHub](#github-proxy) autentica le richieste a GitHub invece, quindi non avete bisogno di una credenziale API per esso
* **L'API Anthropic e i registri di pacchetti pubblici**: `api.anthropic.com`, `registry.npmjs.org`, `jsr.io`, `npm.jsr.io`, `pypi.org`, `files.pythonhosted.org`, `index.crates.io` e `proxy.golang.org`
* **Richieste dello script di configurazione**: Claude Code si connette al proxy dell'agente quando si avvia, dopo che lo [script di configurazione](#setup-scripts) è stato eseguito

<h3 id="select-an-environment-from-the-cli">
  Selezionare un ambiente dalla CLI
</h3>

Eseguite `/remote-env` nel vostro terminale per scegliere l'ambiente predefinito per le sessioni cloud che create dalla CLI, come [`claude --cloud`](/docs/it/claude-code-on-the-web#from-terminal-to-cloud). Il comando apre un selettore dei vostri ambienti esistenti e salva la vostra scelta nella chiave `remote.defaultEnvironmentId` nelle vostre [impostazioni utente](/docs/it/settings#where-settings-live), quindi si applica in ogni progetto sulla vostra macchina fino a quando non la cambiate, a meno che la stessa chiave non sia impostata a un [livello di impostazioni](/docs/it/settings#settings-precedence) di precedenza più alta, come le impostazioni del progetto di un repository.

Un ID di [ambiente self-hosted](/docs/it/self-hosted-environments), che ha la forma `ccpool_...`, segue una regola di origine più ristretta. Consultate [`remote.defaultEnvironmentId`](/docs/it/settings-reference#remote-defaultenvironmentid) per i livelli di impostazioni che Claude Code onora da esso.

`/remote-env` imposta solo il default: non avvia una sessione e non può aggiungere o modificare ambienti. Gestite gli ambienti dal [selettore di ambiente](#configure-your-environment).

<h3 id="archive-an-environment">
  Archiviare un ambiente
</h3>

Per archiviare uno dei vostri ambienti, apritelo per la modifica e selezionate **Archive**. Un Owner archivia un [ambiente condiviso](#organization-shared-environments) dalla pagina **Cloud environments** nelle impostazioni di amministrazione. Non potete eliminare un ambiente, solo archiviarlo.

L'archiviazione influisce sulle nuove sessioni, non su quelle in esecuzione:

* Le sessioni già in esecuzione nell'ambiente continuano a funzionare.
* L'ambiente scompare dal selettore e da `/remote-env`, quindi non potete sceglierlo per le nuove sessioni.
* Le credenziali API sull'ambiente rimangono allegate nelle sue sessioni in esecuzione. Cancellate quelle che non desiderate più prima di archiviare.
* Nessuna nuova sessione può iniziare in un ambiente archiviato, su nessuna superficie. Se l'ambiente era il vostro [default CLI](#select-an-environment-from-the-cli) salvato, Claude Code avvia le sessioni cloud CLI nell'ambiente ospitato da Anthropic quando il vostro elenco ne ha uno, e altrimenti nel primo ambiente nel vostro elenco che non è un [ambiente bridge Remote Control](#the-default-environment). Qualsiasi cosa configurata con l'ambiente esplicitamente, come una [routine](/docs/it/routines#environments-and-network-access), non può avviare nuove sessioni in esso. Puntate a un altro ambiente.

<h3 id="organization-shared-environments">
  Ambienti condivisi dell'organizzazione
</h3>

Sui piani Team ed Enterprise, un Owner può creare ambienti cloud che sono condivisi con ogni membro dell'organizzazione. Lo stesso ruolo gestisce tutto il resto sulla pagina **Cloud environments** dell'amministrazione, inclusi gli [ambienti self-hosted](/docs/it/self-hosted-environments); il ruolo Admin non può aprire la pagina. L'elenco completo dei ruoli che possono aprirla è quello per [gestire le impostazioni gestite dal server](/docs/it/server-managed-settings#access-control).

Gli ambienti condivisi appaiono nel [selettore di ambiente](#configure-your-environment) di ogni membro sotto un'intestazione **Organization**, dopo gli ambienti personali del membro sotto **Personal**, quindi un team può standardizzare su una configurazione invece di farla ricreare a ogni membro. Selezionando l'icona delle impostazioni di un ambiente condiviso lì apre un riepilogo di sola lettura della sua configurazione per ogni membro, Owner inclusi.

Un Owner rende un ambiente disponibile all'organizzazione in uno di due modi:

* **Creare un ambiente condiviso**: utilizzate la pagina **Cloud environments** nelle [impostazioni di amministrazione](https://claude.ai/admin-settings), che è anche dove gli Owner modificano e archiviano gli ambienti condivisi. Ognuno ha un nome, un [livello di accesso di rete](#access-levels), [variabili di ambiente](#set-environment-variables) in formato `.env` e uno [script di configurazione](#setup-scripts).
* **Condividere un ambiente personale**: aprite uno dei vostri ambienti per la modifica nel selettore di ambiente, quindi condividetelo dalla riga **Who can use it**. L'ambiente mantiene il suo ID, quindi le sessioni e le routine che lo utilizzano già non sono interessate, e ogni membro può quindi vederlo e avviare sessioni in esso.

Gli Owner scelgono l'[ambiente predefinito](#the-default-environment) dell'organizzazione separatamente, su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

Le sessioni di ogni membro in un ambiente condiviso leggono le sue variabili, quindi non includete segreti in esse. Le [credenziali API](#add-api-credentials), che danno alle sessioni una chiave che non possono leggere, non sono ancora disponibili sui piani Team o Enterprise.

<h3 id="set-the-environment-a-claude-tag-channel-uses">
  Impostare l'ambiente che un canale Claude Tag utilizza
</h3>

Nei canali [Claude Tag](https://claude.com/docs/claude-tag/overview), Claude lavora come identità condivisa della vostra organizzazione, non come nessun membro, quindi le sessioni dei canali utilizzano solo ambienti a livello di organizzazione, sia ambienti condivisi che [ambienti self-hosted](/docs/it/self-hosted-environments). Per dare a un canale un toolchain che non è [preinstallato](#installed-tools), come .NET, un Owner può creare un [ambiente condiviso](#organization-shared-environments) dalla pagina **Cloud environments** dell'amministrazione con uno [script di configurazione](#setup-scripts) che lo installa. Puntate il canale a un ambiente in uno di due modi:

* Impostate un ambiente condiviso o self-hosted come l'[ambiente predefinito](#the-default-environment) dell'organizzazione su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).
* [Fissate uno a un canale](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) nelle impostazioni di amministrazione di Claude Tag.

<h2 id="network-access">
  Accesso di rete
</h2>

Ogni ambiente imposta un livello di accesso di rete, che controlla le connessioni in uscita che le sue sessioni possono effettuare. Il livello predefinito, **Trusted**, consente i registri dei pacchetti e altri [domini consentiti](#default-allowed-domains); **Custom** accetta il vostro elenco di domini.

Per modificare l'accesso di rete di un ambiente, [apritelo per la modifica](#configure-your-environment) e utilizzate il selettore **Network access** nella finestra di dialogo. Un [ambiente condiviso](#organization-shared-environments) si apre in sola lettura lì, quindi un Owner modifica il suo accesso di rete dalla pagina **Cloud environments** nelle [impostazioni di amministrazione](https://claude.ai/admin-settings) invece. L'icona cloud che apre il selettore appare sulle superfici dell'app elencate sotto [L'ambiente Default](#the-default-environment) e nell'[editor di routine](/docs/it/routines#environments-and-network-access); gli ambienti personali non hanno una pagina separata nelle impostazioni del vostro account claude.ai.

<Note>
  I connettori MCP che abilitate su una sessione o routine funzionano senza aggiungere i loro host ai **Allowed domains**, perché il traffico del connettore viaggia attraverso i server di Anthropic piuttosto che attraverso la rete della sessione. Questo si basa sullo stesso canale legato ad Anthropic notato sotto [Sicurezza e isolamento](/docs/it/claude-code-on-the-web#security-and-isolation). Disabilitate qualsiasi connettore che non vi serve per limitare quali strumenti Claude può raggiungere.
</Note>

<h3 id="access-levels">
  Livelli di accesso
</h3>

Il campo **Network access** nella [finestra di dialogo dell'ambiente](#configure-your-environment) accetta uno di quattro livelli:

| Livello     | Connessioni in uscita                                                                         |
| :---------- | :-------------------------------------------------------------------------------------------- |
| **None**    | Nessun accesso di rete in uscita attraverso la rete della sessione                            |
| **Trusted** | [Domini consentiti](#default-allowed-domains) solo: registri dei pacchetti, GitHub, cloud SDK |
| **Full**    | Qualsiasi dominio                                                                             |
| **Custom**  | Il vostro elenco di consentiti, opzionalmente includendo i default                            |

Qualunque livello scegliate, le sessioni possono ancora raggiungere questi, perché ognuno prende un percorso che non passa attraverso l'elenco di consentiti di rete della sessione:

* GitHub, attraverso il suo [proxy separato](#github-proxy)
* I [connettori MCP](#network-access) che abilitate, il cui traffico viaggia attraverso i server di Anthropic
* Gli host che avete elencato sulle [credenziali API](#add-api-credentials) dell'ambiente, tranne gli [host che non ricevono mai la credenziale](#requests-that-never-get-the-credential)
* L'API Anthropic, per le richieste di Claude Code stesso, anche a **None**, come notato sotto [Sicurezza e isolamento](/docs/it/claude-code-on-the-web#security-and-isolation)

<h3 id="allow-specific-domains">
  Consentire domini specifici
</h3>

Per consentire domini che non sono nella lista Trusted, selezionate **Custom** nelle impostazioni di accesso di rete dell'ambiente, quindi elencate un dominio per riga nel campo **Allowed domains**. Questo esempio consente tre host che un progetto interno potrebbe necessitare.

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Le sessioni in questo ambiente possono ora raggiungere `api.example.com`, qualsiasi sottodominio di `internal.example.com` e `registry.example.com`, e nessun altro dominio attraverso la rete della sessione. Il [traffico GitHub](#github-proxy), il [traffico del connettore MCP](#network-access) e le richieste agli host delle [credenziali API](#add-api-credentials) dell'ambiente, diversi dagli [host che non ricevono mai la credenziale](#requests-that-never-get-the-credential), non passano attraverso questo elenco di consentiti. Un `*.` iniziale corrisponde a ogni sottodominio. Per mantenere anche i [domini Trusted](#default-allowed-domains), selezionate **Also include default list of common package managers**; lasciatelo deselezionato per consentire solo quello che elencate.

Se la vostra organizzazione utilizza gli [artifact](/docs/it/artifacts#availability), non avete bisogno di `*.frame.claudeusercontent.com` nell'elenco affinché le sessioni li leggano. Quando l'elenco lascia fuori quell'host, Claude Code legge il contenuto dell'artifact attraverso la connessione della sessione ad Anthropic invece. Mantenete l'host in un elenco di consentiti in due situazioni:

* **Le sessioni in questo ambiente aprono gli artifact pubblici di un'altra organizzazione**: Claude Code li recupera dall'host direttamente, quindi aggiungetelo a questo elenco.
* **State configurando la CLI locale o un runner self-hosted**: mantenete l'host in quell'elenco di consentiti. Consultate i [requisiti di accesso di rete](/docs/it/network-config#network-access-requirements) e i [requisiti di rete](/docs/it/self-hosted-environments-deploy#network-requirements) self-hosted.

Ogni ambiente ha il suo elenco di domini consentiti; non c'è un elenco di consentiti a livello di organizzazione che gli amministratori possono spingere agli ambienti di ogni membro. Le [impostazioni gestite dal server](/docs/it/server-managed-settings) si applicano ancora all'interno delle sessioni cloud, ma nessuna di esse aggiunge domini all'elenco di consentiti di rete dell'ambiente. Per dare a un team un elenco standard, un Owner può creare un [ambiente condiviso dall'organizzazione](#organization-shared-environments) con accesso di rete **Custom** e quell'elenco.

<h3 id="github-proxy">
  Proxy GitHub
</h3>

Negli ambienti ospitati da Anthropic, tutte le operazioni GitHub passano attraverso un proxy dedicato che mantiene le vostre credenziali GitHub reali al di fuori della VM della sessione, indipendentemente dal [livello di accesso](#access-levels) dell'ambiente. Le sessioni in un ambiente self-hosted si autenticano con le operazioni git con le credenziali che la vostra distribuzione fornisce; [Configurare git](/docs/it/self-hosted-environments-deploy#configure-git) copre le opzioni, incluse le credenziali coniate per sessione e un opt-in a questo stesso proxy. Il proxy fornisce:

* **Credenziali Git**: il client git all'interno della VM utilizza una credenziale con ambito, che il proxy verifica e scambia con il vostro token GitHub effettivo.
* **Richieste API**: le richieste dagli strumenti GitHub integrati e da `gh` sotto il [segnaposto `proxy-injected`](#work-with-github-issues-and-pull-requests), vengono inviate con le vostre credenziali reali sostituite.
* **Protezione push**: `git push` funziona solo contro il ramo di lavoro corrente della sessione; la clonazione, il recupero e le operazioni PR funzionano normalmente.
* **Ambito del repository**: le richieste API GitHub e di asset di rilascio raggiungono solo i repository collegati alla sessione, quindi uno script di configurazione che scarica asset di rilascio da un repository non collegato riceve un 403.
* **Restrizioni GraphQL**: il proxy serve solo un set fisso di operazioni GraphQL per i flussi di lavoro delle pull request. Il proxy rifiuta tutto il resto sull'endpoint GraphQL con un 403 che dice `This GraphQL query is not enabled for this session` e nomina il fallback REST, `gh api repos/{owner}/{repo}/...`. La restrizione si applica a ogni richiesta attraverso il proxy indipendentemente dalle credenziali che fornite, quindi un `GH_TOKEN` che impostate riceve lo stesso 403. Claude non può raggiungere le API GitHub che esistono solo in GraphQL, come Projects v2, attraverso il proxy.

I file sottoposti a commit dai repository pubblici arrivano tramite `raw.githubusercontent.com`, che il [proxy di sicurezza](#security-proxy) gestisce invece. Quel dominio è nella lista [Trusted](#default-allowed-domains) predefinita, quindi quei file rimangono raggiungibili a meno che il [livello di accesso](#access-levels) dell'ambiente non lo escluda.

<h3 id="security-proxy">
  Proxy di sicurezza
</h3>

Le sessioni cloud negli ambienti ospitati da Anthropic vengono eseguite dietro un proxy di rete HTTP/HTTPS per scopi di sicurezza e prevenzione degli abusi; in un [ambiente self-hosted](/docs/it/self-hosted-environments-deploy#default-deny-egress), il traffico in uscita esce attraverso il vostro confine di rete invece. Tutto il traffico internet in uscita da una sessione ospitata da Anthropic passa attraverso questo proxy, che fornisce:

* Protezione contro richieste dannose
* Limitazione della velocità e prevenzione degli abusi
* Filtro dei contenuti per una sicurezza migliorata
* Un audit trail a livello DNS dei nomi host richiesti

<h2 id="what’s-available-in-cloud-sessions">
  Cosa è disponibile nelle sessioni cloud
</h2>

Negli ambienti ospitati da Anthropic, ogni sessione ottiene una macchina virtuale (VM) fresca che esegue Ubuntu 24.04 su x86\_64, indipendentemente dal vostro sistema operativo e dall'architettura della CPU, con il vostro repository clonato e i toolchain comuni preinstallati. Quando una dipendenza fornisce binari precompilati, come gem Ruby con estensioni native o wheel Python precostruiti, utilizzate la sua build Linux x86\_64 per corrispondere alla VM. Questa sezione copre i default ospitati da Anthropic, gli strumenti GitHub integrati, come [eseguire test e servizi](#run-tests-start-services-and-add-packages), e i [limiti di risorse](#resource-limits) che ogni VM ottiene.

<Note>
  Le sessioni che la vostra organizzazione instrada a un [ambiente self-hosted](/docs/it/self-hosted-environments) vengono eseguite sui vostri runner invece, con gli strumenti che la vostra immagine runner fornisce.
</Note>

<h3 id="what-carries-over-from-your-setup">
  Cosa viene trasferito dalla vostra configurazione
</h3>

Le sessioni cloud iniziano da un clone fresco del vostro repository. Qualsiasi cosa che sottoponete a commit nel repository è disponibile. Qualsiasi cosa che avete installato o configurato solo sulla vostra macchina non è disponibile nella sessione. La politica della vostra organizzazione arriva separatamente attraverso le [impostazioni gestite dal server](/docs/it/server-managed-settings).

|                                                                                                                                                                                                           | Disponibile nelle sessioni cloud                                  | Perché                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Il vostro `CLAUDE.md` del repository                                                                                                                                                                      | Sì                                                                | Parte del clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| I vostri hook `.claude/settings.json` del repository e le regole di permesso                                                                                                                              | Sì, in una sessione con un repository                             | Parte del clone. Una sessione con diversi repository, incluso un thread di [progetto](/docs/it/claude-projects#what-threads-pick-up-from-your-repositories), inizia sopra i clone e non li legge                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| I vostri server MCP `.mcp.json` del repository                                                                                                                                                            | Sì, in una sessione con un repository                             | Parte del clone, trovato dalla directory di lavoro della sessione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Il vostro `.claude/rules/` del repository                                                                                                                                                                 | Sì                                                                | Parte del clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Il vostro `.claude/skills/`, `.claude/agents/`, `.claude/commands/` del repository                                                                                                                        | Sì                                                                | Parte del clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Plugin e marketplace dichiarati nel vostro `.claude/settings.json` del repository                                                                                                                         | No                                                                | Una sessione cloud non installa i plugin che un repository attiva sotto [`enabledPlugins`](/docs/it/settings-reference#enabledplugins), inclusi quelli dai marketplace che elenca sotto [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces)                                                                                                                                                                                                                                                                                                                                                                                          |
| Le [impostazioni gestite dal server](/docs/it/server-managed-settings) della vostra organizzazione                                                                                                             | Sì                                                                | Recuperate dai server di Anthropic quando la sessione inizia. Consultate [Copertura della superficie](/docs/it/model-config#surface-coverage) per come `availableModels` viene applicato nelle sessioni cloud. Le impostazioni distribuite al vostro dispositivo tramite MDM o file di impostazioni gestite non si applicano, perché la sessione viene eseguita su una VM gestita da Anthropic; in un [ambiente self-hosted](/docs/it/self-hosted-environments), le sessioni leggono anche il file di impostazioni gestite nell'immagine runner, per [come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) |
| Il vostro `~/.claude/CLAUDE.md` utente                                                                                                                                                                    | No                                                                | Vive sulla vostra macchina, non nel repository                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Il vostro `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` utente                                                                                                                          | No                                                                | Vivono sulla vostra macchina, non nel repository. Sottoponete a commit nel directory `.claude/` del repository. Le sessioni cloud caricano automaticamente le skill che abilitate su claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Plugin abilitati solo nelle vostre impostazioni utente                                                                                                                                                    | No                                                                | L'`enabledPlugins` con ambito utente vive in `~/.claude/settings.json` sulla vostra macchina                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Server MCP che avete aggiunto con `claude mcp add` all'ambito locale predefinito o all'ambito utente                                                                                                      | No                                                                | Quelli scrivono su `~/.claude.json` sulla vostra macchina, non nel repository. Aggiungete il server con `claude mcp add --scope project`, che scrive il [`.mcp.json`](/docs/it/mcp#project-scope) del repository, e sottoponete a commit quel file. Una sessione con un repository lo carica                                                                                                                                                                                                                                                                                                                                                               |
| Variabili di trasporto nel vostro blocco `env` di `.claude/settings.json` del repository, come `NODE_EXTRA_CA_CERTS` e le [variabili del certificato client mTLS](/docs/it/network-config#mtls-authentication) | No                                                                | L'ambiente di hosting gestisce la connessione API della sessione, quindi Claude Code ignora queste chiavi e annota ogni chiave ignorata nel log di debug della sessione                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Chiavi API e token per i servizi che Claude chiama                                                                                                                                                        | Sui piani Pro e Max, come [credenziali API](#add-api-credentials) | Aggiungete la chiave una volta sull'ambiente e il proxy dell'agente la allega alle richieste per gli host che elencate. Una chiave che il proxy dell'agente [non può allegare](#requests-that-never-get-the-credential), o qualsiasi chiave su un piano Team o Enterprise, rimane in una variabile di ambiente                                                                                                                                                                                                                                                                                                                                        |
| Auth interattivo come AWS SSO                                                                                                                                                                             | No                                                                | Non supportato. SSO richiede un login basato su browser che non può essere eseguito in una sessione cloud                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

Per rendere disponibile la vostra configurazione nelle sessioni cloud, sottoponete a commit nel repository.

Chiunque utilizzi l'ambiente può leggere le sue variabili di ambiente e lo script di configurazione. La nota della finestra di dialogo sotto **Environment variables** lo dice e avverte contro l'aggiunta di segreti lì. Sui piani Pro e Max, memorizzate una chiave che il proxy dell'agente può allegare come [credenziale API](#add-api-credentials) invece.

<h3 id="installed-tools">
  Strumenti installati
</h3>

Le sessioni cloud vengono fornite con runtime di linguaggio comuni, strumenti di build e database preinstallati. La tabella seguente riassume cosa è incluso per categoria.

| Categoria    | Incluso                                                                |
| :----------- | :--------------------------------------------------------------------- |
| **Python**   | Python 3.x con pip, poetry, uv, black, mypy, pytest, ruff              |
| **Node.js**  | 20, 21 e 22, con npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**     | 3.1, 3.2, 3.3 con gem, bundler, rbenv                                  |
| **PHP**      | 8.3 con Composer                                                       |
| **Java**     | OpenJDK 21 con Maven e Gradle                                          |
| **Go**       | Go con supporto dei moduli                                             |
| **Rust**     | rustc e cargo                                                          |
| **C/C++**    | GCC, Clang, cmake, ninja, conan                                        |
| **Docker**   | docker, dockerd, docker compose                                        |
| **Database** | PostgreSQL 16, Redis 7.0                                               |
| **Utilità**  | git, gh, jq, yq, ripgrep, tmux, vim, nano                              |

¹ Bun è installato ma ha [problemi di compatibilità](#install-dependencies-with-a-sessionstart-hook) noti con il proxy per il recupero dei pacchetti.

Per ottenere le versioni della maggior parte degli strumenti in questa tabella, chiedete a Claude di eseguire `check-tools` in una sessione cloud. È un comando shell installato sulla VM della sessione, non un comando che digitate con `/`; chiedete a Claude perché [Claude esegue tutti i comandi della VM per voi](#run-tests-start-services-and-add-packages). Per uno strumento che non segnala, come Ruby, PHP, bun, PostgreSQL o Redis, chiedete a Claude di eseguire il comando di versione dello strumento stesso, ad esempio `psql --version`.

Le versioni di Node.js sono installate su `/opt/node20`, `/opt/node21` e `/opt/node22`, con 22 su `PATH` per impostazione predefinita. Per lavorare con una versione diversa, chiedete a Claude di anteporre la directory `bin` di quella versione, come `/opt/node20/bin`, a `PATH`.

I toolchain al di fuori di questo elenco, come .NET SDK, non sono preinstallati anche quando i loro registri di pacchetti sono sulla [lista di consentiti predefinita](#default-allowed-domains). Installateli con uno [script di configurazione](#setup-scripts).

<h3 id="work-with-github-issues-and-pull-requests">
  Lavorare con i problemi e le pull request di GitHub
</h3>

Le sessioni cloud includono strumenti GitHub integrati che consentono a Claude di leggere i problemi, elencare le pull request, recuperare i diff e pubblicare commenti senza alcuna configurazione. Questi strumenti si autenticano attraverso il [proxy GitHub](#github-proxy) utilizzando il metodo che avete configurato sotto [Opzioni di autenticazione GitHub](/docs/it/claude-code-on-the-web#github-authentication-options), quindi il vostro token non entra mai nel contenitore.

Potete impostare `GH_TOKEN` o `GITHUB_TOKEN` voi stessi nelle [impostazioni di ambiente](#set-environment-variables), o lasciare entrambi non impostati e lasciare che il [proxy GitHub](#github-proxy) si autentichi per voi:

* Se impostate un token, passa attraverso al contenitore invariato, quindi i vostri script e il [`gh` CLI](https://cli.github.com) di GitHub lo utilizzano direttamente.
* Se non impostate nessuno e il [proxy GitHub](#github-proxy) sta gestendo l'autenticazione per la vostra sessione, entrambe le variabili leggono come la stringa segnaposto `proxy-injected` nei comandi che Claude esegue, e il proxy sostituisce le vostre credenziali reali sulle richieste GitHub in uscita. `gh` funziona senza un token vostro, ma uno script che legge `GITHUB_TOKEN` direttamente ottiene il segnaposto, non un token utilizzabile.

Un token che impostate è una variabile di ambiente ordinaria, quindi chiunque utilizzi l'ambiente può leggerlo; il percorso del proxy mantiene la credenziale fuori dalla configurazione dell'ambiente e dalla VM della sessione.

Per verificare quale caso si applica alla vostra sessione, chiedete a Claude di eseguire `echo $GH_TOKEN`.

Il [`gh` CLI](https://cli.github.com) di GitHub è preinstallato. Se avete bisogno di un comando `gh` che gli strumenti integrati non coprono, come `gh release` o `gh workflow run`, chiedete a Claude di eseguirlo. `gh` legge `GH_TOKEN` automaticamente, quindi non avete bisogno di eseguire `gh auth login`.

<h3 id="link-output-back-to-the-session">
  Collegare l'output di nuovo alla sessione
</h3>

Ogni sessione cloud ha un URL di trascrizione su claude.ai, e la sessione può leggere il suo ID dalla variabile di ambiente `CLAUDE_CODE_REMOTE_SESSION_ID`. Utilizzate questo per mettere un link tracciabile nei corpi PR, nei messaggi di commit, nei post Slack o nei report generati in modo che un revisore possa aprire l'esecuzione che li ha prodotti.

I commit che Claude crea in una sessione cloud includono un trailer git `Claude-Session: <url>`, e i corpi PR includono l'URL della sessione su una riga propria. Per omettere il trailer e il link nel corpo PR, impostate [`attribution.sessionUrl`](/docs/it/settings-reference#attribution-sessionurl) su `false`.

Per includere il link della sessione in qualcosa di diverso da un commit o PR, come un messaggio Slack che Claude pubblica o un file di report che scrive, chiedete a Claude di eseguire il comando seguente e utilizzate il suo output. Il comando converte il prefisso `cse_` nel valore della variabile di ambiente al prefisso `session_` che l'URL della trascrizione si aspetta:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Eseguire test, avviare servizi e aggiungere pacchetti
</h3>

Non avete una shell nella VM della sessione. Claude esegue ogni comando per voi, quindi formulate i compiti in questa sezione come richieste nel vostro prompt.

<h4 id="run-tests">
  Eseguire test
</h4>

Claude esegue i test come parte del lavoro su un compito. Chiedete nel vostro prompt, come "fix the failing tests in `tests/`" o "run pytest after each change." I test runner che vengono con i [toolchain preinstallati](#installed-tools), come pytest e cargo test, funzionano senza configurazione aggiuntiva. Un runner che il vostro progetto dichiara come dipendenza, come jest, si installa con le vostre dipendenze.

<h4 id="start-services">
  Avviare servizi
</h4>

PostgreSQL e Redis sono preinstallati ma non in esecuzione per impostazione predefinita. Chiedete a Claude di avviare quello di cui avete bisogno; i comandi che esegue sono:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker è disponibile per l'esecuzione di servizi containerizzati. Chiedete a Claude di eseguire `docker compose up` per avviare i servizi del vostro progetto. L'accesso di rete per il pull delle immagini segue il [livello di accesso](#access-levels) del vostro ambiente, e i [default Trusted](#default-allowed-domains) includono Docker Hub e altri registri comuni.

Se le vostre immagini sono grandi o lente da estrarre, aggiungete `docker compose pull` o `docker compose build` al vostro [script di configurazione](#setup-scripts). La [cache dell'ambiente](#environment-caching) mantiene le immagini estratte, quindi ogni nuova sessione le ha su disco. La cache memorizza solo file, non processi in esecuzione, quindi Claude avvia comunque i contenitori ogni sessione.

<h4 id="add-packages">
  Aggiungere pacchetti
</h4>

Per aggiungere pacchetti che non sono preinstallati, utilizzate uno [script di configurazione](#setup-scripts). La [cache dell'ambiente](#environment-caching) mantiene quello che lo script installa, quindi i pacchetti che installate lì sono disponibili all'inizio di ogni sessione senza reinstallare ogni volta. Potete anche chiedere a Claude di installare pacchetti a metà sessione, ma quelle installazioni non si trasferiscono ad altre sessioni.

<h3 id="resource-limits">
  Limiti di risorse
</h3>

Le sessioni cloud negli ambienti ospitati da Anthropic vengono eseguite con limiti di risorse approssimativi che possono cambiare nel tempo:

* 4 vCPU
* 16 GB di RAM
* 30 GB di disco

La VM può interrompere i compiti che necessitano di significativamente più memoria, come grandi lavori di build o test ad alta intensità di memoria. Per carichi di lavoro oltre questi limiti, utilizzate [Remote Control](/docs/it/remote-control) per eseguire Claude Code sul vostro hardware, o eseguite le sessioni cloud in un [ambiente self-hosted](/docs/it/self-hosted-environments) su compute che la vostra organizzazione gestisce.

<h2 id="setup-scripts">
  Script di configurazione
</h2>

Uno script di configurazione è uno script Bash che viene eseguito quando inizia una nuova sessione cloud, prima che Claude Code si avvii. Utilizzare gli script di configurazione per installare dipendenze, configurare strumenti o recuperare qualsiasi cosa di cui la sessione ha bisogno e che non è preinstallata.

Gli script vengono eseguiti come root su Ubuntu 24.04, quindi `apt install` e la maggior parte dei gestori di pacchetti del linguaggio funzionano.

Per aggiungere uno script di configurazione, aprire la finestra di dialogo delle impostazioni dell'ambiente e inserire lo script nel campo **Setup script**.

Questo esempio installa [ShellCheck](https://www.shellcheck.net/), che non è preinstallato.

```bash theme={null}
#!/bin/bash
apt update && apt install -y shellcheck
```

<h3 id="script-requirements">
  Requisiti dello script
</h3>

Uno script di configurazione ha tre vincoli da considerare:

* **Exit zero**: se lo script esce con un codice diverso da zero, la sessione non si avvia. Aggiungere `|| true` ai comandi non critici in modo che un errore di installazione intermittente non blocchi la sessione.
* **Completamento entro cinque minuti**: mantenere il tempo di esecuzione totale dello script sotto circa cinque minuti in modo che la [cache dell'ambiente](#environment-caching) possa essere costruita. Eseguire le installazioni indipendenti in parallelo con `&` e `wait`, e spostare qualsiasi singolo download che non rientra in un [hook SessionStart](#setup-scripts-vs-sessionstart-hooks) che lo avvia in background.
* **Accesso di rete per le installazioni**: le installazioni di pacchetti devono raggiungere i registri. Il livello **Trusted** predefinito copre i [domini consentiti comuni](#default-allowed-domains) inclusi npm, PyPI, RubyGems e crates.io; con accesso di rete **None**, le installazioni non riescono.

<h3 id="environment-caching">
  Cache dell'ambiente
</h3>

Lo script di configurazione viene eseguito la prima volta che si avvia una sessione in un ambiente. Dopo il completamento, Anthropic crea uno snapshot del filesystem e riutilizza quello snapshot come punto di partenza per le sessioni successive. Le nuove sessioni iniziano con le dipendenze, gli strumenti e le immagini Docker già sul disco, e saltano il passaggio dello script di configurazione. Questo mantiene l'avvio veloce anche quando lo script installa grandi toolchain o estrae immagini di container.

La cache è uno snapshot del filesystem, quindi mantiene ciò che lo script di configurazione scrive su disco e perde tutto ciò che era solo in esecuzione. I pacchetti installati, le immagini Docker estratte e i file scritti vengono tutti trasferiti. Un database avviato dallo script, uno stack `docker compose up` o qualsiasi altro processo in background no; avviare quelli per sessione chiedendo a Claude o con un [hook SessionStart](#setup-scripts-vs-sessionstart-hooks).

Lo script di configurazione viene eseguito di nuovo per ricostruire la cache quando si modifica lo script di configurazione dell'ambiente o gli host di rete consentiti, e quando la cache raggiunge la scadenza dopo circa sette giorni. La ripresa di una sessione esistente non riesegue mai lo script di configurazione.

Non è necessario abilitare la cache o gestire gli snapshot da soli.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Script di configurazione vs. hook SessionStart
</h3>

Utilizzare uno script di configurazione per il provisioning della VM stessa: toolchain e strumenti CLI che non sono [preinstallati](#installed-tools). Utilizzare un [hook SessionStart](/docs/it/hooks#sessionstart) per la configurazione del progetto che dovrebbe essere eseguita ovunque, cloud e locale, come `npm install`.

Gli script di configurazione e gli hook SessionStart vengono eseguiti in un ordine fisso quando inizia una sessione cloud. La tabella confronta dove li si configura, quando vengono eseguiti e dove vengono eseguiti.

|                             | Script di configurazione                                                                                                                                                                                  | Hook SessionStart                                                                                                                                                                                                                                        |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Dove li si configura**    | La finestra di dialogo dell'ambiente su [claude.ai/code](https://claude.ai/code), più la pagina di amministrazione **Cloud environments** per gli [ambienti condivisi](#organization-shared-environments) | Un [file di impostazioni](/docs/it/settings#where-settings-live) come il `.claude/settings.json` del repository; vedere [Cosa viene trasferito dalla configurazione](#what-carries-over-from-your-setup) per sapere quali file raggiungono una sessione cloud |
| **Quando vengono eseguiti** | Prima che Claude Code si avvii, saltati quando esiste un [ambiente memorizzato nella cache](#environment-caching)                                                                                         | Dopo che Claude Code si avvia, su ogni sessione inclusa quella ripresa                                                                                                                                                                                   |
| **Dove vengono eseguiti**   | Solo sessioni cloud                                                                                                                                                                                       | Sessioni locali e cloud                                                                                                                                                                                                                                  |

Se si hanno hook SessionStart nel file `~/.claude/settings.json` a livello di utente, non aspettarsi che siano nel cloud. Le impostazioni a livello di utente rimangono sulla macchina. Quali altri hook vengono eseguiti dipende da dove viene eseguita la sessione:

* **Ambiente ospitato da Anthropic**: Claude Code esegue gli hook dal repository e dalle [impostazioni gestite dal server](/docs/it/server-managed-settings) dell'organizzazione.
* **[Ambiente self-hosted](/docs/it/self-hosted-environments-configuration#permissions-and-tool-approval)**: Claude Code esegue anche gli hook che l'operatore ha seminato da `~/.claude/` dell'host del runner, e gli hook nel file di impostazioni gestite dell'immagine del runner quando quel file è uno dei [fonti gestite che Claude Code applica](/docs/it/managed-settings#how-claude-code-combines-managed-sources).

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Installare dipendenze con un hook SessionStart
</h3>

Per installare dipendenze solo nelle sessioni cloud, abbinare un hook SessionStart con uno script che controlla dove viene eseguito.

Innanzitutto, aggiungere un hook SessionStart al `.claude/settings.json` del repository. Questa configurazione dice a Claude Code di eseguire `scripts/install_pkgs.sh` dal repository ogni volta che una sessione si avvia o riprende:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

Il `matcher` limita l'hook agli eventi `startup` e `resume`, e `$CLAUDE_PROJECT_DIR` si risolve nella radice del repository, quindi l'hook trova lo script indipendentemente dalla directory di lavoro della sessione.

Successivamente, creare lo script in `scripts/install_pkgs.sh`. Esce immediatamente al di fuori del cloud, quindi installa le dipendenze:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

Il controllo `CLAUDE_CODE_REMOTE` è ciò che limita l'installazione alle sessioni cloud: la VM della sessione trasporta quella variabile come `true`, non è mai `true` localmente, quindi sul laptop lo script esce prima di installare qualsiasi cosa.

Insieme, i due file danno a ogni sessione cloud un `npm install` e `pip install` freschi all'avvio mentre lasciano le sessioni locali intatte.

<h4 id="limitations-in-cloud-sessions">
  Limitazioni nelle sessioni cloud
</h4>

Gli hook SessionStart si comportano allo stesso modo nel cloud che localmente, con questi avvertimenti:

* **Una repository per sessione**: una sessione con più repository non carica gli hook da nessuno dei `.claude/settings.json` della repository, quindi un hook SessionStart che si definisce lì non viene eseguito. Installare le dipendenze per quelle sessioni con uno [script di configurazione](#setup-scripts) invece.
* **Nessun ambito solo cloud**: gli hook vengono eseguiti sia nelle sessioni locali che in quelle cloud. Per saltare l'esecuzione locale, uscire anticipatamente a meno che la variabile di ambiente `CLAUDE_CODE_REMOTE` non sia `true`, come fa lo [script di installazione delle dipendenze](#install-dependencies-with-a-sessionstart-hook).
* **Richiede accesso di rete**: i comandi di installazione devono raggiungere i registri dei pacchetti. Se l'ambiente utilizza accesso di rete **None**, questi hook non riescono. L'[elenco consentiti predefinito](#default-allowed-domains) sotto **Trusted** copre npm, PyPI, RubyGems e crates.io.
* **Compatibilità proxy**: negli ambienti ospitati da Anthropic, tutto il traffico in uscita passa attraverso un [proxy di sicurezza](#security-proxy), e alcuni gestori di pacchetti non funzionano correttamente con esso; Bun è un esempio noto. In un [ambiente self-hosted](/docs/it/self-hosted-environments-deploy#default-deny-egress), il traffico in uscita passa attraverso il proprio confine di rete.
* **Aggiunge latenza di avvio**: gli hook vengono eseguiti ogni volta che una sessione si avvia o riprende, a differenza degli script di configurazione che beneficiano della [cache dell'ambiente](#environment-caching). Mantenere gli script di installazione veloci controllando se le dipendenze sono già presenti prima di reinstallarle.

Per personalizzare l'immagine di base, utilizzare uno script di configurazione per installare ciò di cui si ha bisogno sopra l'[immagine fornita](#installed-tools), o eseguire la propria immagine come container insieme a Claude con `docker compose`. La sostituzione completa dell'immagine di base non è ancora supportata.

<h2 id="default-allowed-domains">
  Domini consentiti predefiniti
</h2>

Con accesso di rete **Trusted**, le sessioni possono raggiungere i seguenti domini per impostazione predefinita. I domini contrassegnati con `*` indicano la corrispondenza del sottodominio con carattere jolly, quindi `*.gcr.io` consente qualsiasi sottodominio di `gcr.io`.

<AccordionGroup>
  <Accordion title="Servizi Anthropic">
    * api.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Controllo versione">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Registri di contenitori">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Piattaforme cloud">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
    * \*.r2.cloudflarestorage.com
  </Accordion>

  <Accordion title="Gestori di pacchetti JavaScript e Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
    * jsr.io
    * npm.jsr.io
  </Accordion>

  <Accordion title="Gestori di pacchetti Python">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Gestori di pacchetti Ruby">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Gestori di pacchetti Rust">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Gestori di pacchetti Go">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="Gestori di pacchetti JVM">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * maven.google.com
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * plugins-artifacts.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Altri gestori di pacchetti">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Distribuzioni Linux">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Strumenti di sviluppo e piattaforme">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Servizi cloud e monitoraggio">
    * http-intake.logs.datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Distribuzione di contenuti e mirror">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Schema e configurazione">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Cloud sessions reference](/docs/it/claude-code-on-the-web): avviare, gestire e condividere sessioni cloud
* [Cloud sessions quickstart](/docs/it/web-quickstart): connettere GitHub e avviare la vostra prima sessione cloud
* [Claude Tag](https://claude.com/docs/claude-tag/overview): le sessioni che Claude avvia da Slack vengono eseguite negli stessi ambienti
* [Routine](/docs/it/routines): le esecuzioni programmate utilizzano gli stessi ambienti e livelli di accesso di rete
* [Remote Control](/docs/it/remote-control): eseguire sessioni sulla rete e sui file della vostra macchina invece
* [Ambienti self-hosted](/docs/it/self-hosted-environments): eseguire sessioni cloud sull'infrastruttura propria della vostra organizzazione
* [SessionStart hooks](/docs/it/hooks#sessionstart): configurazione sottoposta a commit nel repository che viene eseguita nelle sessioni locali e cloud
* [Impostazioni gestite dal server](/docs/it/server-managed-settings): politica dell'organizzazione che raggiunge le sessioni cloud
