> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Distribuisci ambienti self-hosted in produzione

> Esegui runner self-hosted in produzione: hardening della sicurezza, controllo dell'egress di rete, credenziali git, ricette Kubernetes e Compose, e risoluzione dei problemi.

<Note>
  Gli ambienti self-hosted sono in beta pubblica sui piani Team ed Enterprise; [Disponibilità e limitazioni](/docs/it/self-hosted-environments#availability-and-limitations) copre il percorso di abilitazione. Questa pagina copre l'esecuzione della flotta in produzione; consulta la [guida rapida](/docs/it/self-hosted-environments-quickstart) per il tuo primo runner e sessione.
</Note>

Un [ambiente self-hosted](/docs/it/self-hosted-environments) esegue [sessioni cloud](/docs/it/claude-code-on-the-web) di Claude Code su runner che distribuisci all'interno della tua rete, e in produzione quelle sessioni eseguono codice diretto dal modello per conto di chiunque possa inviare una sessione all'ambiente. Questa pagina è per l'operatore che porta un ambiente funzionante in produzione. Funziona attraverso la distribuzione in ordine: cosa bloccare prima di connettere sistemi reali, l'egress di cui ha bisogno la flotta, come le sessioni si autenticano al tuo host git, le ricette di distribuzione stesse, e cosa controllare quando le sessioni si comportano male.

<h2 id="harden-your-deployment">
  Hardening della tua distribuzione
</h2>

Un runner self-hosted esegue codice arbitrario diretto dal modello sulla tua infrastruttura per conto di chiunque possa inviare una sessione al suo ambiente. Questo è qualsiasi membro della tua organizzazione Anthropic, e chiunque possa avviare una sessione del canale [Claude Tag](https://claude.com/docs/claude-tag/overview) in un ambito che un Owner ha instradato all'ambiente. Lavora su ogni elemento prima di connettere un ambiente ai sistemi di produzione:

* **Container effimeri per sessione**: esegui ogni processo runner in un container o VM fresco che viene distrutto quando il processo esce, con `--capacity 1` e il valore predefinito `--drain-grace-sec 0` in modo che ogni container serva esattamente una sessione. A una capacità più alta, o con un drain grace positivo, un container serve più sessioni dallo stesso [owner bloccato](/docs/it/self-hosted-environments#key-concepts); vedi [Ciclo di vita del runner](/docs/it/self-hosted-environments#runner-lifecycle). Non riutilizzare un filesystem tra i riavvii del runner, tranne nella configurazione deliberata [pre-warmed checkout](#reuse-a-pre-warmed-checkout), e mai tra owner.
* **Nessuna credenziale ampia nell'immagine**: non includere chiavi SSH di lunga durata, credenziali del provider cloud, o token di accesso personale che concedono più di quanto una sessione necessita. Crea credenziali utilizzate durante una sessione, come token push o API, per sessione dal tuo [script wrapper](/docs/it/self-hosted-environments-configuration#wrapper-scripts). Per il clone iniziale, che avviene prima che lo script wrapper venga eseguito, usa un [`checkout` lifecycle hook](/docs/it/self-hosted-environments-configuration#checkout) o [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy); vedi [Configura git](#configure-git).
* **Mantieni il segreto dell'ambiente lontano dagli host che eseguono sessioni**: il segreto dell'ambiente può registrare runner e raccogliere qualsiasi sessione in coda sull'ambiente. Su una flotta fissa vive su ogni host runner, dove il codice di qualsiasi sessione può leggere il file segreto. Preferisci [runner on-demand](/docs/it/self-hosted-environments-configuration#on-demand-runners), dove il segreto rimane sull'host dell'orchestrator, che non esegue mai codice utente, e ogni runner riceve un ordine di lavoro monouso che registra esattamente un runner. Su una flotta fissa, tratta il file environment-secret come leggibile da ogni sessione e ruota il segreto dopo qualsiasi sospetto compromesso della sessione.
* **Egress di rete default-deny**: limita il traffico in uscita del container runner e sessione al tuo confine di rete su ogni ambiente; [Default-deny egress](#default-deny-egress) copre cosa consentire e perché.
* **IAM host con privilegi minimi**: l'identità di calcolo allegata all'host runner, come un profilo di istanza o un account di servizio del nodo, dovrebbe concedere solo ciò di cui il runner stesso ha bisogno. Le sessioni dovrebbero ottenere le proprie credenziali attraverso il tuo script wrapper piuttosto che ereditare quelle dell'host.
* **Blocca l'endpoint dei metadati cloud dalle sessioni**: mantenere le sessioni fuori dall'identità dell'host richiede il blocco del loro accesso all'endpoint dei metadati cloud, e le politiche di egress a livello di subnet non intercettano il traffico dei metadati link-local, quindi bloccalo nel container stesso:

  * IMDSv2 con un hop limit di uno
  * GKE Workload Identity con metadata concealment
  * Un esplicito deny per `169.254.169.254` nello spazio dei nomi di rete del container della sessione

  Il blocco si applica anche al tuo script wrapper e ai lifecycle hook, poiché condividono il container. Autentica qualsiasi scambio di token con il [session JWT](/docs/it/self-hosted-environments-identity) contro il tuo servizio di token su egress allowlisted, o usa un'identità web basata su file come IAM Roles for Service Accounts (IRSA) su Amazon EKS.
* **Isolamento del filesystem per runner**: ogni processo runner ottiene la propria directory di lavoro che nessun altro processo sull'host può leggere o scrivere. Rendi `--hooks-dir`, lo script wrapper, e la `~/.claude/` dell'host di sola lettura per la sessione, sia incorporato nell'immagine che montato in sola lettura.
* **Dispatch non ha controllo di accesso per ambiente**: qualsiasi membro della tua organizzazione Anthropic può inviare una sessione a qualsiasi suo ambiente. Se un Owner [instrada i canali Claude Tag all'ambiente](/docs/it/cloud-environments#set-the-environment-a-claude-tag-channel-uses), chiunque l'[impostazione di accesso Claude Tag](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude) ammette può avviare sessioni di canale che vengono eseguite lì. Per impostazione predefinita, questo è chiunque nell'area di lavoro Slack connessa, con o senza un account Claude. Tratta ogni host runner come raggiungibile per l'esecuzione di codice da parte di chiunque possa inviare a esso, e posiziona sull'host runner solo i dati e le credenziali che tutte quelle persone sono autorizzate a leggere. [`--lock-to-account`](/docs/it/self-hosted-environments-reference#runner-cli-flags) limita quale account le sessioni di un dato host eseguono, ma non restringe chi può inviare all'ambiente. Per rendere gli ambienti self-hosted l'unica opzione di selezione, un [Owner](/docs/it/cloud-environments#organization-shared-environments) può nascondere gli ambienti ospitati da Anthropic per l'intera organizzazione dalla pagina [**Cloud environments**](https://claude.ai/admin-settings/cloud-environments).
* **Applica la guardia repo-settings**: scegli la modalità di guardia con [`--confine-repo-settings`](/docs/it/self-hosted-environments-reference#runner-cli-flags). Il valore predefinito `warn` registra una violazione e comunque genera la sessione, `enforce` rifiuta la sessione, e `off` disabilita la scansione. Il runner scansiona le impostazioni impegnate di ogni repository per:

  * Una concessione che si risolve al di fuori dello spazio di lavoro della sessione stessa: una voce `additionalDirectories`, una regola `Edit`, `Write`, o `NotebookEdit` in `permissions.allow`, o una voce `sandbox.filesystem.allowWrite` o `allowRead`
  * Un blocco `env` non vuoto
  * Un override della postura dell'operatore come `sandbox.enabled: false`

  La guardia viene eseguita indipendentemente da [`--trust-workspace`](/docs/it/self-hosted-environments-reference#runner-cli-flags), e non copre i repository hook, `.mcp.json`, o le regole Bash; vedi [Permessi e approvazione degli strumenti](/docs/it/self-hosted-environments-configuration#permissions-and-tool-approval) per dove quelle concessioni appartengono.

<Note>
  La lista di indirizzi IP consentiti della tua organizzazione non copre il traffico del runner self-hosted per impostazione predefinita. Non fare affidamento su di essa come controllo di rete per il traffico del runner o della sessione; applica invece default-deny egress al tuo confine di rete, e contatta il tuo team di account Anthropic se desideri l'applicazione della lista di indirizzi IP consentiti per la tua organizzazione.
</Note>

<h2 id="network-requirements">
  Requisiti di rete
</h2>

Il runner e i figli della sessione che genera fanno connessioni in uscita agli host di seguito. Limita l'egress del container della sessione a questi host e ai servizi interni specifici che le sessioni devono raggiungere; [Default-deny egress](#default-deny-egress) copre come e perché.

Questi host sono sempre richiesti:

| Host                                                               | Porta                                      | Utilizzato per                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :----------------------------------------------------------------- | :----------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                                                | 443, HTTPS; WSS solo per il connettore SCM | Piano di controllo del runner e streaming della sessione, inferenza del modello, flag delle funzionalità, analitiche dei prodotti, recuperi della chiave [JWKS](/docs/it/self-hosted-environments-identity), firma dei commit, il proxy git quando `--use-anthropic-git-proxy` è impostato, e il tunnel [SCM connector](/docs/it/self-hosted-environments-reference#scm-connector-flags) dell'orchestrator quando `--scm-connector-host` è impostato |
| Il tuo host git, come `github.com` o il tuo host GitHub Enterprise | 443 o 22                                   | Clonazione e push dei repository. Non necessario se il runner usa `--use-anthropic-git-proxy`, che instrada il traffico git attraverso `api.anthropic.com`.                                                                                                                                                                                                                                                                                |

Se questi host sono necessari dipende dalla tua configurazione:

| Host                                 | Porta | Quando richiesto                                                                                                                                                                                                                                                                                                                                                    |
| :----------------------------------- | :---- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `downloads.claude.ai`                | 443   | Al momento dell'installazione, quando installi o aggiorni Claude Code sull'host con il programma di installazione nativo; lo script `install.sh` stesso viene servito da `claude.ai`. Al momento dell'esecuzione della sessione, solo quando le sessioni installano plugin dal marketplace ufficiale di Anthropic.                                                  |
| `storage.googleapis.com`             | 443   | Al momento dell'esecuzione della sessione, per i conteggi di installazione dei plugin e i metadati mostrati in `/plugin`.                                                                                                                                                                                                                                           |
| `code.claude.com` e `claude.com`     | 443   | Ricerche di documentazione dall'agente claude-code-guide integrato e richieste WebFetch pre-approvate durante le sessioni. Il blocco di questi host influisce solo sulle ricerche di documentazione.                                                                                                                                                                |
| `*.frame.claudeusercontent.com`      | 443   | Solo quando lo [strumento Artifact](/docs/it/artifacts#availability) è disponibile per le sessioni nella tua organizzazione; i valori predefiniti variano in base al piano, secondo la tabella di disponibilità lì. Imposta `CLAUDE_CODE_DISABLE_ARTIFACT=1` sul runner per mantenere lo strumento disabilitato indipendentemente dall'impostazione dell'organizzazione. |
| `registry.npmjs.org`                 | 443   | Quando una sessione installa un plugin, sia per il recupero dei pacchetti plugin da fonte npm che per l'installazione delle dipendenze Node.js di un plugin, o quando un server MCP lanciato da `npx` viene eseguito                                                                                                                                                |
| `http-intake.logs.us5.datadoghq.com` | 443   | Metriche operative di Anthropic. Solo quando `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1` è impostato; disabilitato per impostazione predefinita negli ambienti self-hosted.                                                                                                                                                                                                 |
| `browser-intake-us5-datadoghq.com`   | 443   | Caricamenti di rapporti di errore di Anthropic, inviati solo quando la [segnalazione di errori](/docs/it/data-usage#telemetry-services) è abilitata per l'account della sessione. Soppressa da `DISABLE_ERROR_REPORTING=1` o `DISABLE_TELEMETRY=1`.                                                                                                                      |

Il runner non raggiunge `statsig.anthropic.com`, `*.sentry.io`, `claude.ai`, o `platform.claude.com`. Questi host appaiono in alcuni elenchi di controllo di rete enterprise più vecchi, ma non è necessario aggiungerli alla lista di indirizzi consentiti per il traffico del runner o della sessione: i recuperi dei flag delle funzionalità vanno a `api.anthropic.com`, e il runner si autentica con il segreto dell'ambiente piuttosto che con OAuth interattivo. Due flussi lato host raggiungono `claude.ai`, quindi eseguili da un host il cui egress lo consente piuttosto che ampliare l'egress del container della sessione: il programma di installazione a una riga recupera `install.sh` da `claude.ai` al momento dell'installazione, e il `claude auth login` interattivo, che la [configurazione guidata](/docs/it/self-hosted-environments-quickstart#set-up-an-environment-and-runner), la modalità firmata di `doctor`, e il [dispatch da CI](/docs/it/self-hosted-environments-testing#authenticate-from-ci) usano, accede attraverso `claude.ai`, `claude.com`, e `platform.claude.com`. `mcp-proxy.anthropic.com` non è richiesto neanche: le sessioni self-hosted non lo usano, e la consegna dei tuoi connettori claude.ai dell'organizzazione alle sessioni, quando abilitata per la tua organizzazione, viene instradata attraverso `api.anthropic.com`. Vedi [Server MCP](/docs/it/self-hosted-environments-configuration#mcp-servers).

<h3 id="default-deny-egress">
  Default-deny egress
</h3>

Distribuisci container runner e sessione in un segmento di rete o namespace il cui traffico in uscita è limitato agli host nella [tabella dei requisiti di rete](#network-requirements), al tuo host git, e ai servizi interni specifici che le sessioni devono raggiungere. Il prodotto non può verificare o applicare questo, quindi applicalo al tuo confine di rete su ogni ambiente. Il codice della sessione è diretto dal modello e può tentare connessioni a host arbitrari; default-deny egress a livello di rete limita dove questi tentativi possono atterrare. Questo si applica indipendentemente dalla modalità di permesso: il set di strumenti pre-approvato predefinito include già `Bash`, quindi l'egress della shell viene eseguito senza un prompt anche senza [modalità auto](/docs/it/self-hosted-environments-configuration#permissions-and-tool-approval).

Per i dettagli su quale telemetria ogni sessione emette e come disattivarla, vedi [Telemetria](/docs/it/self-hosted-environments-reference#telemetry).

<h3 id="authenticate-to-an-egress-proxy">
  Autentica a un proxy di egress
</h3>

Alcuni proxy di egress aziendali richiedono un'intestazione `Proxy-Authorization` su ogni connessione. Il token in quell'intestazione spesso ruota troppo velocemente per essere scritto nell'URL del proxy che imposti in `HTTPS_PROXY`. Imposta `HTTPS_PROXY` o `HTTP_PROXY` all'URL del tuo proxy come al solito, quindi imposta `--proxy-authorization-command` o `--proxy-authorization-file` per dire al runner dove leggere il valore dell'intestazione. Entrambi i flag richiedono Claude Code v2.1.238 o successivo.

<h4 id="choose-where-the-proxy-authorization-value-comes-from">
  Scegli da dove viene il valore `Proxy-Authorization`
</h4>

Scegli il flag che corrisponde a come produci il token `Proxy-Authorization`:

* **[`--proxy-authorization-command <command>`](/docs/it/self-hosted-environments-reference#runner-cli-flags)**: scegli questo per un token che generi su richiesta. Il runner esegue il comando della shell e usa il suo stdout ritagliato come valore dell'intestazione, ad esempio `Bearer <token>`.
* **[`--proxy-authorization-file <path>`](/docs/it/self-hosted-environments-reference#runner-cli-flags)**: scegli questo per un token che un altro processo ruota in posizione. Il runner legge il file e usa i suoi contenuti ritagliati come valore dell'intestazione.

<h4 id="configurations-the-runner-refuses-to-start-with">
  Configurazioni che il runner rifiuta di avviare con
</h4>

Ogni flag ha anche una forma di variabile di ambiente, elencata accanto ad esso nel [riferimento dei flag CLI del runner](/docs/it/self-hosted-environments-reference#runner-cli-flags). Prima che il runner contatti il tuo proxy o il piano di controllo, controlla i flag e le loro variabili, e rifiuta di avviare in tre casi:

* **Entrambi i flag impostati**: un flag più la variabile di ambiente dell'altro flag conta come impostazione di entrambi.
* **Nessun URL proxy**: né `HTTPS_PROXY` né `HTTP_PROXY` contiene un URL `http://` o `https://`. Il runner legge entrambe le variabili in maiuscole o minuscole, e non consulta `ALL_PROXY`.
* **Uno dei flag passato al sottocomando orchestrator**: `self-hosted-runner orchestrator` non accetta i flag o le loro variabili di ambiente. Passa il flag a ogni runner che l'orchestrator avvia invece.

<h4 id="what-the-runner-changes-while-a-proxy-authorization-flag-is-set">
  Cosa cambia il runner mentre un flag proxy-authorization è impostato
</h4>

Con uno dei flag impostati, il runner avvia un listener proprio e invia il traffico proxy da se stesso, dai suoi lifecycle hook, e dalle sue sessioni attraverso quel listener. Il listener aggiunge l'intestazione `Proxy-Authorization` sulla strada verso il tuo proxy.

* **Listener**: il listener è un proxy forward su `127.0.0.1`. Il runner avvia il listener prima di registrarsi con il piano di controllo, e esce all'avvio se il listener non può avviarsi.
* **Variabili proxy**: il runner riscrive quale di `HTTPS_PROXY` e `HTTP_PROXY` hai impostato in modo che punti al listener. Quel valore riscritto raggiunge il runner stesso, i suoi lifecycle hook, e ogni sessione che esegue.
* **Rotazione del token**: un token ruotato ha effetto senza un riavvio. Per ogni connessione che il listener apre al tuo proxy, il runner esegue il tuo comando o legge di nuovo il tuo file e aggiunge il risultato come intestazione.
* **Ambiente della sessione**: una sessione raggiunge il tuo proxy solo attraverso il listener. Nell'ambiente di ogni sessione il runner rimuove `ALL_PROXY`, rimuove qualsiasi ortografia di `HTTPS_PROXY` o `HTTP_PROXY` che non hai impostato, e fissa `NO_PROXY` al valore del runner stesso.
* **Log**: il runner non registra mai il valore dell'intestazione.

<h2 id="configure-git">
  Configura git
</h2>

Il runner gestisce i checkout dei repository ma non configura l'identità git o le credenziali per impostazione predefinita. Controlli l'immagine e l'ambiente del processo del runner, quindi controlli la configurazione git. Scegli uno di due approcci:

* **Lascia che il runner configuri git**: avvia il runner con `--configure-git` per fargli scrivere la stessa identità e configurazione di firma dei commit che usano le sessioni ospitate da Anthropic
* **Spedisci la configurazione git nella tua immagine**: imposta l'identità e le credenziali push tu stesso, ad esempio per eseguire il commit sotto la tua identità bot

Piani minimi di versione git sull'host runner: [`--configure-git`](#let-the-runner-configure-git) la firma dei commit SSH richiede Git 2.34 o più recente, [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) richiede 2.32 o più recente, e la ripresa delle sessioni da rami spinti da [`--push-outcome-on-release`](/docs/it/self-hosted-environments-reference#runner-cli-flags) richiede 2.29 o più recente. Git 2.24 è sufficiente se ometti tutti e tre e gestisci l'identità git tu stesso.

<h3 id="let-the-runner-configure-git">
  Lascia che il runner configuri git
</h3>

Avvia il runner con `--configure-git`, o imposta `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`, per fargli scrivere la configurazione git globale all'avvio:

* `user.name = Claude` e `user.email = noreply@anthropic.com`, corrispondendo alle sessioni ospitate da Anthropic
* Firma dei commit e dei tag in formato SSH, instradata attraverso uno shim gestito dal runner che firma ogni commit tramite il servizio di firma di Anthropic usando le credenziali della sessione stessa. Le firme sono verificabili su GitHub rispetto alla chiave di firma SSH pubblicata di Anthropic.
* `push.negotiate = true`, in modo che git chieda al tuo host git quali commit ha già prima di impacchettare un push. Richiede Claude Code v2.1.257 o successivo.
* `core.hooksPath` che punta a una directory di hook gestita dal runner. I suoi hook `commit-msg` e `prepare-commit-msg` aggiungono un trailer `Co-authored-by:` per il creatore della sessione a ogni commit, costruito dall'email in [`CCR_SESSION_ACCOUNT_EMAIL`](/docs/it/self-hosted-environments-configuration#wrapper-scripts) e omesso quando quella variabile non è impostata. Se la tua immagine imposta già `core.hooksPath`, il runner lascia la tua impostazione in posizione, salta l'installazione di questi hook, e stampa un avviso `[runner:git]`.

La firma dei commit richiede git 2.34 o più recente; il runner controlla all'avvio e esce con un errore se il tuo git è più vecchio. Questo flag non configura le credenziali push, che fornisci comunque nell'immagine.

<h3 id="ship-git-config-in-your-image">
  Spedisci la configurazione git nella tua immagine
</h3>

L'identità git è richiesta per qualsiasi commit. Impostala a livello di sistema nel tuo Dockerfile in modo che la configurazione si applichi indipendentemente da quale utente il processo runner esegue:

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

Senza un'identità, `git commit` fallisce con `Please tell me who you are` e le sessioni non possono fare progressi. Puoi usare la tua identità bot invece; il runner non sovrascrive questi valori.

Non incorporare credenziali push di lunga durata o ampiamente scoped in un'immagine runner condivisa: una credenziale nell'immagine è disponibile a ogni sessione che l'immagine esegue, chiunque l'abbia avviata. Invece, crea un token a breve durata, con scope minimo per sessione dal tuo [script wrapper](/docs/it/self-hosted-environments-configuration#wrapper-scripts), usando l'identità del creatore della sessione decodificata dal session JWT. Abbinalo a un container per sessione effimero, che richiede `--capacity 1`, in modo che nessuna credenziale sopravviva alla sessione che l'ha creata; vedi la [sezione hardening](#harden-your-deployment).

Se devi configurare le credenziali push a livello di immagine, ad esempio per una chiave di distribuzione di sola lettura, limitale il più possibile:

* Una chiave di distribuzione SSH limitata a un repository con una riscrittura `url.<base>.insteadOf`
* Un `credential.helper` che restituisce un token con scope minimo
* `GIT_SSH_COMMAND` che punta a una chiave con scope ristretto

Qualsiasi meccanismo tu configuri deve funzionare senza un prompt, perché il clone integrato del runner e il fetch disabilitano i prompt che git, SSH, e Git Credential Manager mostrerebbero altrimenti:

* Il runner imposta `GIT_TERMINAL_PROMPT=0`, in modo che git non chieda un nome utente o una password.
* Il runner esegue SSH con `BatchMode=yes`, aggiunto al tuo `GIT_SSH_COMMAND` se ne imposti uno, in modo che SSH non chieda una passphrase o una conferma dell'host.
* Il runner imposta `GCM_INTERACTIVE=never`, in modo che Git Credential Manager non apra una finestra di dialogo di accesso.
* Il runner cancella `core.askPass`, quindi se usi un helper askpass, impostalo attraverso la variabile di ambiente `GIT_ASKPASS` invece.

Se il tuo host git rifiuta la credenziale, o non ne hai configurata una, il runner riprova alcune volte e poi fallisce la preparazione del repository quando il repository è quello in cui la sessione spinge i risultati. Per un repository dal quale la sessione legge solo, [Troubleshooting](#troubleshooting) copre quando il runner lo salta invece. Il runner non passa queste impostazioni nell'ambiente della sessione.

Se le directory di checkout sono di proprietà di un uid diverso dal processo runner, git rifiuta di operare su di esse; aggiungi `safe.directory`:

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

<h3 id="use-the-anthropic-git-proxy">
  Usa il proxy git di Anthropic
</h3>

Avvia il runner con `--use-anthropic-git-proxy`, o imposta `CLAUDE_RUNNER_USE_GIT_PROXY=1`, per fargli clonare attraverso il proxy git di Anthropic, autenticato con il token a breve durata della sessione stessa. Per le sessioni utente ordinarie, il proxy usa il token OAuth di GitHub o GitHub Enterprise memorizzato per il creatore della sessione; per le sessioni bot e agente, usa il token di installazione dell'app GitHub della tua organizzazione. In entrambi i casi, l'immagine del runner non ha bisogno di credenziali git: nessuna chiave SSH, nessun credential helper, nessun `.netrc`. Questo è lo stesso percorso di autenticazione che usano gli ambienti ospitati da Anthropic.

Il proxy richiede `--capacity 1` perché l'URL del proxy è per sessione, e git 2.32 o più recente perché git più vecchio ignora il meccanismo di configurazione che il proxy usa per isolare le sessioni l'una dall'altra. Il runner rifiuta di avviarsi se uno dei due requisiti non è soddisfatto. Poiché il proxy recupera dal lato di Anthropic, il tuo host git deve essere raggiungibile dall'infrastruttura di Anthropic, lo stesso requisito che hanno le sessioni ospitate da Anthropic; per un host git che è solo instradabile all'interno della tua rete, usa un [`checkout` lifecycle hook](/docs/it/self-hosted-environments-configuration#checkout) invece. Ogni processo runner gestisce una sessione alla volta, quindi esegui più repliche per il parallelismo. Quando il proxy è abilitato, `--git-host-rewrite` e `--git-ssh-rewrite` non hanno effetto: l'URL del proxy punta a `api.anthropic.com`, non al tuo host git.

Il runner segnala anche l'opt-in ad Anthropic quando si registra, stampando `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)` all'avvio. La segnalazione dell'opt-in richiede Claude Code v2.1.267 o successivo, e le versioni precedenti accettano il flag senza segnalarlo o stampare quella riga. Ogni sessione su un runner con opt-in usa quindi o git gestito da Anthropic o l'URL del proxy per sessione. Quando una sessione usa l'URL del proxy per sessione, il runner registra una riga `[runner:warn]` dicendo così.

<h3 id="rewrite-git-urls-for-private-networks">
  Riscrivi gli URL git per le reti private
</h3>

Gli URL dei repository arrivano dal piano di controllo come HTTPS, con il nome host del tuo host git; per GitHub Enterprise, questo è il nome host che hai configurato per l'[integrazione GitHub Enterprise](/docs/it/github-enterprise-server) nelle impostazioni di amministrazione di Claude Code su claude.ai. Due flag ripetibili riscrivono quegli URL prima del clone:

* `--git-host-rewrite <from>=<to>`: per split-horizon DNS, dove Anthropic raggiunge il tuo host git tramite un nome host esterno ma i runner devono usarne uno interno
* `--git-ssh-rewrite <host>`: per host git che accettano solo SSH, riscrivendo `https://<host>/owner/repo` a `git@<host>:owner/repo`

La riscrittura dell'host viene eseguita per prima, quindi elenca il nome host interno in `--git-ssh-rewrite` se hai bisogno di entrambi. Per il controllo completo del checkout, usa un [`checkout` lifecycle hook](/docs/it/self-hosted-environments-configuration#checkout).

<h2 id="build-the-runner-image">
  Costruisci l'immagine del runner
</h2>

Anthropic non pubblica un'immagine runner pre-costruita. Costruisci la tua intorno al binario `claude`, stratificando qualsiasi toolchain di cui i tuoi repository hanno bisogno: runtime di linguaggio, compilatori, gestori di pacchetti, e sidecar [MCP](/docs/it/mcp).

Le ricette di seguito usano `--capacity 4`, in modo che un container serva fino a quattro sessioni concorrenti dallo stesso owner bloccato. Questo non fornisce l'isolamento del container per sessione nella [sezione hardening](#harden-your-deployment): prima di connettere un ambiente ai sistemi di produzione, esegui le ricette a `--capacity 1` con un container per sessione, o usa [runner on-demand](/docs/it/self-hosted-environments-configuration#on-demand-runners), che mantengono anche il segreto dell'ambiente lontano dagli host che eseguono sessioni.

Questo Dockerfile è un punto di partenza minimo:

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

Scambia `linux-x64` con `linux-arm64` se i tuoi nodi sono ARM, o con `linux-x64-musl` o `linux-arm64-musl` su un'immagine basata su musl come Alpine; vedi [Configurazione Alpine Linux](/docs/it/setup#alpine-linux-and-musl-based-distributions) per i pacchetti extra di cui le immagini musl hanno bisogno. L'URL è la posizione di rilascio standard di Claude Code, quindi puoi verificare il binario scaricato rispetto al manifesto firmato del rilascio come descritto in [Integrità binaria e firma del codice](/docs/it/setup#binary-integrity-and-code-signing). Costruisci l'immagine con Claude Code versione 2.1.224 o successiva, quindi spingila al tuo registro e fai riferimento ad essa nelle ricette di seguito:

```bash theme={null}
docker build --build-arg CLAUDE_CODE_VERSION=2.1.267 -t <your-registry>/claude-runner:latest .
```

<h2 id="size-cpu-and-memory-for-sessions">
  Dimensiona CPU e memoria per le sessioni
</h2>

Dimensiona il container o l'host di un runner per le sessioni che esegue piuttosto che per il processo runner. Il runner stesso esegue il polling per il lavoro, prepara il checkout di ogni sessione, esegue i tuoi [lifecycle hook](/docs/it/self-hosted-environments-configuration#lifecycle-hooks), e avvia e supervisiona i processi della sessione. Il carico proviene dalle sessioni: ognuna è un processo Claude Code più tutto ciò che avvia, come build, suite di test, installazioni di pacchetti, e [server MCP](/docs/it/mcp).

Per una sessione, inizia con i seguenti valori, indicati come richieste e limiti di Kubernetes o l'equivalente della tua piattaforma, e trattali come un punto di partenza piuttosto che un requisito:

* **Memoria**: una richiesta e un limite di 4 GiB ciascuno, che soddisfa il minimo di 4 GB nei [requisiti di sistema](/docs/it/setup#system-requirements) di Claude Code. Mantieni i due uguali in modo che lo scheduler conti la memoria completa del container. Quando il container raggiunge il suo limite di memoria, il kernel uccide i processi al suo interno, il che può terminare una sessione a metà compito.
* **CPU**: una richiesta di 2 CPU e un limite di 4 CPU, in modo che una sessione possa scoppiare sopra la richiesta durante le build. Il kernel limita un container al suo limite di CPU piuttosto che uccidere i processi al suo interno, quindi le sessioni al limite vengono eseguite più lentamente ma continuano a funzionare.

In una specifica di container Kubernetes, imposta quei valori iniziali con il seguente blocco `resources`:

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

Le build e i test sono solitamente la parte più grande e più variabile del carico di una sessione, quindi esegui una build rappresentativa del tuo repository, misura il suo picco di CPU e memoria, e aumenta qualsiasi valore iniziale che non lascia spazio per il processo Claude Code in cima a quel picco.

Il runner usa `--capacity` per limitare quante sessioni esegue contemporaneamente. Non divide CPU o memoria tra di loro, quindi le sessioni su un runner condividono la CPU e la memoria del container. Per limitare la quota di una sessione, applica limiti dal tuo [script wrapper](/docs/it/self-hosted-environments-configuration#wrapper-scripts). Cosa dare a un container quindi dipende da quante sessioni serve contemporaneamente:

* **Una sessione per runner**: dai a ogni container i valori di una sessione. Usa questo dimensionamento a `--capacity 1`, che la [sezione hardening](#harden-your-deployment) consiglia, e per [runner on-demand](/docs/it/self-hosted-environments-configuration#on-demand-runners), dove imposti i valori sul carico di lavoro che il tuo hook [`spawn-runner`](/docs/it/self-hosted-environments-configuration#the-spawn-runner-hook) invia, come il modello di pod di un Kubernetes Job.
* **Diverse sessioni per runner**: a un `--capacity` sopra uno, moltiplica i valori di una sessione per la capacità, perché fino a quel numero di sessioni possono essere eseguite nel container contemporaneamente. Le ricette [Kubernetes](#kubernetes) e [Docker Compose](#docker-compose) eseguono `--capacity 4` senza limiti di CPU o memoria, quindi aggiungi limiti dimensionati per la capacità che esegui.

<h2 id="kubernetes">
  Kubernetes
</h2>

Il runner serve `GET /healthz` sulla porta 8080 per impostazione predefinita, configurabile con `--health-port`, quindi i probe di Kubernetes funzionano senza configurazione extra. L'endpoint restituisce `200` ogni volta che il processo è vivo, quindi i probe di seguito rilevano un processo morto, non uno bloccato; per catturare un runner che ha smesso di eseguire il polling, avvisa sulla serie `last_poll_age_seconds` da [`/metrics`](/docs/it/self-hosted-environments-reference#prometheus-metrics). Il Deployment di seguito monta il segreto dell'ambiente da un Kubernetes Secret, punta i probe di liveness e readiness a `/healthz`, e imposta un periodo di grazia di terminazione di 90 secondi. Vedi [Shutdown timing](#shutdown-timing) per il motivo per cui il periodo di grazia è importante.

Il manifesto non imposta `resources` di CPU o memoria sul container runner. Aggiungi un blocco dimensionato per la capacità che esegui, come [Dimensiona CPU e memoria per le sessioni](#size-cpu-and-memory-for-sessions) descrive.

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

Il Deployment di sopra vive in uno spazio dei nomi `claude-runners`. Crea prima lo spazio dei nomi:

```bash theme={null}
kubectl create namespace claude-runners
```

Crea il Secret di supporto da un file locale che contiene il valore che hai copiato nel passaggio [**Copy environment key**](/docs/it/self-hosted-environments-quickstart#set-up-an-environment-and-runner) dell'interfaccia utente di amministrazione, in modo che il segreto non appaia mai nella tua cronologia della shell. Esegui `(umask 077 && cat > ./environment-secret)`, incolla il segreto, premi Invio, quindi Ctrl-D. Quindi crea il Secret e cancella il file:

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

<h2 id="docker-compose">
  Docker Compose
</h2>

Il servizio Compose di seguito riavvia il runner ogni volta che esce, il che copre sia i crash che l'uscita normale dopo il drenaggio. Una politica di riavvio di Docker riavvia lo stesso container con il suo strato scrivibile intatto, quindi il runner torna su un filesystem riutilizzato piuttosto che su uno fresco che la [postura hardening](#harden-your-deployment) consiglia; usa questa ricetta per la valutazione, e per la produzione ricrea il container per esecuzione o usa un orchestrator che lo fa.

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

<h2 id="shutdown-timing">
  Shutdown timing
</h2>

Su `SIGTERM`, il runner smette di accettare nuovo lavoro e, a meno che tu non imposti [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), aspetta fino a `--drain-wait-sec`, zero per impostazione predefinita, affinché i turni in volo finiscano, termina il processo tree di ogni sessione, ed esegue il [`post-session` lifecycle hook](/docs/it/self-hosted-environments-configuration#post-session). Quel process tree include i comandi che Claude stava ancora eseguendo nella sessione.

Il percorso di drenaggio completo ha bisogno di fino a `--session-stop-grace-sec` + `--drain-wait-sec` + `--post-session-hook-timeout-sec`, più 15 secondi di overhead fisso per la pulizia del processo, più 30 secondi in più quando [`--push-outcome-on-release`](/docs/it/self-hosted-environments-reference#runner-cli-flags) è impostato. Questo è 80 secondi ai valori predefiniti, e il runner registra il totale all'avvio. Le sessioni si drenano in parallelo sotto questo unico budget, quindi il totale non cresce con `--capacity`.

Al valore predefinito `--drain-wait-sec 0`, un riavvio rolling interrompe i turni in volo; ogni sessione riprende su un altro runner, perdendo il lavoro non spinto come descritto sotto [Problemi noti](#additional-limitations). Imposta `--drain-wait-sec`, e aumenta il periodo di grazia per corrispondere, per lasciare che i turni finiscano per primi.

Durante tutto quel percorso, il runner continua a fare heartbeat al piano di controllo a capacità zero, in modo che il lease della sessione non scada e venga rimesso in coda a un altro runner mentre il hook `post-session` sta ancora scrivendo il lavoro non impegnato. L'heartbeat si ferma proprio prima che il runner si deregistri.

Dai al runner almeno il totale che registra all'avvio prima che l'host lo fermi. Dove imposti questo dipende da come i tuoi host si fermano:

* **Con un periodo di grazia `SIGTERM`**: imposta `terminationGracePeriodSeconds` su Kubernetes, `stop_grace_period` su Docker Compose, o l'equivalente del tuo orchestrator ad almeno quel totale. Il valore predefinito di Kubernetes di 30 secondi è più breve del percorso di drenaggio del runner, quindi Kubernetes ferma il pod prima che il runner finisca il drenaggio.
* **Con [`--retire-at`](/docs/it/self-hosted-environments-reference#runner-cli-flags)**: dimensiona il margine tra il tempo di ritiro e il tempo di arresto dell'host per coprire i turni tipici, più il hold del compito di background che [Runner lifecycle](/docs/it/self-hosted-environments#runner-lifecycle) descrive, più quel stesso totale. Calcola il tempo di ritiro ad ogni lancio, ad esempio `date +%s` più la durata prevista del runner.
* **Con [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)**: aggiungi due parti in più al totale del percorso di drenaggio. La prima è i minuti che configuri. La seconda è la grazia post-release che [Defer the drain past the first signal](#defer-the-drain-past-the-first-signal) descrive, 75 secondi ai valori predefiniti. Con il flag impostato, il runner stampa anche la figura combinata all'avvio, dopo il totale del percorso di drenaggio.

<h3 id="defer-the-drain-past-the-first-signal">
  Defer the drain past the first signal
</h3>

Imposta [`--defer-shutdown-max-min <n>`](/docs/it/self-hosted-environments-reference#runner-cli-flags) se desideri che un runner che stai riavviando continui a servire le sessioni che tiene per fino a `n` minuti, invece di drenare su il primo segnale. Al primo `SIGTERM` o `SIGINT`, il runner smette di accettare nuovo lavoro e continua a servire le sessioni che tiene. Continua a eseguire il polling in modo che il piano di controllo non rimetta in coda quelle sessioni. Richiede Claude Code v2.1.238 o successivo.

<h4 id="what-happens-to-the-sessions-the-runner-holds-after-the-first-signal">
  Cosa succede alle sessioni che il runner tiene dopo il primo segnale
</h4>

Nei primi due stadi che seguono il segnale, il runner rilascia le sessioni, e una sessione rilasciata riprende su un runner fresco quando il suo utente invia il suo prossimo messaggio. Contando dal primo segnale, il runner si muove attraverso tre stadi:

* **Per i primi `n` minuti**: il runner serve le sue sessioni normalmente e continua a applicare `--startup-timeout-min` e `--kill-session-after-min`. Se imposti anche [`--release-idle-session-min`](/docs/it/self-hosted-environments-reference#runner-cli-flags), il runner rilascia qualsiasi sessione il cui utente è stato inattivo per quel tempo; senza di esso, le sessioni inattive rimangono sul runner.
* **Quando i `n` minuti scadono**: il runner rilascia ogni sessione che ancora tiene, inattiva o no. Il runner aspetta che il turno di una sessione a metà turno finisca, e fino a 60 secondi in più per i compiti di background di un turno, prima di rilasciare quella sessione.
* **Quando la grazia post-release scade**: il runner drena qualsiasi sessione che ancora tiene, e il piano di controllo rimette in coda ogni sessione drenata a un altro runner subito. La grazia post-release inizia quando i `n` minuti scadono ed è 75 secondi ai valori predefiniti. Se imposti `--drain-wait-sec` sopra 60 secondi, la grazia post-release è `--drain-wait-sec` più 15 secondi invece.

In qualsiasi stadio, il runner esce 0 non appena non tiene sessioni. Un secondo segnale taglia gli stadi corti: il runner drena immediatamente, come fa al primo segnale senza `--defer-shutdown-max-min`. Una volta che un drenaggio è in corso, il prossimo segnale forza l'uscita del runner. Questo vale se un secondo segnale o la grazia post-release che scade ha avviato il drenaggio.

<h4 id="size-the-stop-timeout">
  Dimensiona il timeout di arresto
</h4>

Dai al timeout di arresto del tuo host almeno la somma di tre parti: i `n` minuti che configuri, la grazia post-release, e il percorso di drenaggio completo che [Shutdown timing](#shutdown-timing) descrive. Con le impostazioni predefinite la grazia post-release è 75 secondi e il percorso di drenaggio è 80 secondi, quindi consenti `n` minuti più 155 secondi. Il runner stampa questa somma all'avvio ogni volta che `--defer-shutdown-max-min` è impostato.

Se il timeout di arresto scade prima che il runner finisca, l'host uccide il runner. Le sessioni che ancora tiene non ottengono nessun hook `post-session`. Il runner non si deregistra, e il piano di controllo rimette in coda le sessioni circa un minuto dopo. Se non puoi dare al timeout di arresto quella somma, lascia `--defer-shutdown-max-min` non impostato in modo che il runner dreni al primo segnale invece.

<h3 id="what-reaches-a-running-post-session-hook">
  Cosa raggiunge un hook post-session in esecuzione
</h3>

L'hook `post-session` e il figlio della sessione Claude ciascuno vengono eseguiti nel loro proprio gruppo di processo POSIX, separato da quello del runner, quindi i meccanismi di arresto li raggiungono diversamente:

* **Un `SIGTERM` mentre il runner sta già drenando**: forza l'uscita del runner immediatamente, saltando tutto ciò che rimane del percorso di drenaggio. Senza [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), questo è il secondo `SIGTERM` che il runner riceve. Niente segnala un hook `post-session` in esecuzione a metà, quindi su un host nudo dove un processo init adotta orfani, finisce da solo, ma non supervisionato: il suo budget di timeout non si applica più, e una scrittura al tubo di log chiuso può ucciderlo con `SIGPIPE`, quindi un hook che ha bisogno di sopravvivere a un'uscita forzata lì dovrebbe reindirizzare il suo output a un file. Nelle ricette di container su questa pagina il runner è il PID 1 del container e la sua uscita termina il container, e sotto il `KillMode=control-group` predefinito di systemd l'uccisione a livello di cgroup raggiunge anche l'hook, come la voce **Cgroup-wide kills** descrive; in entrambi, tratta un'uscita forzata come fatale per l'hook e fai affidamento al periodo di grazia invece.
* **Segnali a livello di gruppo di processo**, come `kill -- -<pid>` in uno script wrapper, controllo del lavoro della shell, o un watchdog a livello di gruppo: raggiungono il runner e un sottoprocesso di hook `checkout` a metà, che rimane allegato al gruppo deliberatamente, ma non un hook `post-session` a metà esecuzione o il figlio della sessione.
* **Uccisioni a livello di cgroup**, come il `KillMode=control-group` predefinito di systemd o il `SIGKILL` che Kubernetes consegna all'intero container quando `terminationGracePeriodSeconds` scade: raggiungono tutto, incluso l'hook. L'isolamento del gruppo di processo non protegge da questi, motivo per cui il periodo di grazia deve coprire il percorso di drenaggio completo.
* **Il timeout dell'hook stesso**: quando un hook supera `--post-session-hook-timeout-sec`, il runner invia `SIGTERM` all'intero gruppo di processo dell'hook, quindi `SIGKILL` due secondi dopo, in modo che un worker che l'hook ha biforcato, come tar, rsync, o git, termini con la shell wrapper invece di sopravvivere come orfano. La supervisione del runner termina una volta che l'stdio dell'hook si chiude: un worker che ha reindirizzato il suo output a un file e sopravvive allo stadio `SIGTERM` è oltre la portata del runner.

Quando il drenaggio inizia, e di nuovo su un'uscita forzata, il runner registra quanti hook `post-session` sono ancora in esecuzione, in modo che tu possa distinguere un drenaggio tranquillo da uno che è a metà snapshot.

<h2 id="keep-the-base-directory-and-capacity-identical-across-runners">
  Mantieni la directory di base e la capacità identiche tra i runner
</h2>

Se un runner muore a metà sessione, il server rimette in coda la sessione e un altro runner nell'ambiente la raccoglie. Quel runner deriva il percorso di checkout dal suo proprio `--base-dir` e `--capacity`: `--capacity 1` controlla direttamente sotto `--base-dir`, e un `--capacity` sopra `1` usa worktree per sessione invece. Quando i runner nello stesso ambiente usano valori diversi per uno dei due flag, la directory di lavoro della sessione ripresa cambia, e i percorsi assoluti che l'agente ha registrato in precedenza, in modifiche, chiamate di strumenti, o le sue stesse note, puntano a una posizione che non esiste più.

Usa lo stesso `--base-dir` e `--capacity` su ogni runner in un ambiente, e non usare un valore per host come un ID istanza o nome host.

La directory di base è predefinita a `/workspace`, con l'eccezione che la riga di riferimento [`--base-dir`](/docs/it/self-hosted-environments-reference#runner-cli-flags) registra. Il runner ha bisogno di accesso in scrittura ad essa. All'avvio, prima di registrarsi, il runner crea la directory e conferma che può scrivere ad essa, e esce con `cannot create or write to base directory` quando non può. Un runner avviato come root crea il `/workspace` predefinito da solo. Per un runner non root, crea la directory e dai al runner la proprietà dell'utente prima di avviare il runner, o punta `--base-dir` a una directory che l'utente già possiede.

<h2 id="reuse-a-pre-warmed-checkout">
  Riutilizza un checkout pre-riscaldato
</h2>

Per i repository grandi, il clone può dominare l'avvio della sessione. A `--capacity 1` senza un [`checkout` hook](/docs/it/self-hosted-environments-configuration#checkout), il runner mantiene un clone canonico per repository a `<base-dir>/<repo-owner>/<repo>` e lo riutilizza tra le sessioni: recupera il ref richiesto, stacca `HEAD`, e lo resetta duramente, il che è quasi istantaneo quando poco è cambiato. Per saltare il clone freddo, fornisci il clone in uno di due modi:

* **Clone nell'immagine**: costruisci il clone nella tua immagine runner a quel percorso. Ogni container fresco inizia quindi con il clone caldo senza riutilizzare un disco.
* **Clone su un volume persistente**: su runner che pre-blocchi a un account di un utente con [`--lock-to-account`](/docs/it/self-hosted-environments-reference#runner-cli-flags), punta `--base-dir` a un volume persistente, in modo che il disco serva solo quell'account. Un runner pre-bloccato non raccoglie mai sessioni di canale Claude Tag, quindi questa opzione non si applica ai runner che le servono.

Cosa il percorso di riutilizzo fa e non garantisce:

* **Qualsiasi forma di clone funziona**: un clone completo, shallow, o single-branch al percorso viene usato così com'è. Il runner non passa mai `--depth` quando recupera in un clone esistente, quindi un pre-warm completo mantiene la sua cronologia completa e uno shallow rimane shallow. `CLAUDE_RUNNER_FETCH_DEPTH` (`full`, `0`, o un numero; predefinito 50) controlla solo il clone freddo che il runner fa quando nessun clone esiste ancora.
* **Le modifiche tracciate si resettano, i file non tracciati persistono**: ogni sessione inizia da un reset duro che cancella le modifiche tracciate della sessione precedente, ma il runner non esegue mai `git clean`, quindi i file non tracciati dalle sessioni precedenti dell'owner bloccato rimangono nell'albero.
* **Directory per sessione persistono anche**: accanto al checkout, il runner crea voci per sessione sotto `<base-dir>/_sessions/` per ogni sessione che esegue. La directory di configurazione Claude della sessione contiene una copia locale della trascrizione della conversazione. Accanto ad essa si trovano i file caricati della sessione, quando la sessione ne ha. La directory della sessione si trova lì anche: contiene qualsiasi worktree per sessione e checkout di hook `checkout` mentre la sessione è in esecuzione, e mantiene qualsiasi altra cosa Claude abbia scritto in essa.

  Per impostazione predefinita il runner lascia questi in posizione quando la sessione termina, quindi su un disco che sopravvive al processo runner si accumulano. Ogni sessione viene eseguita come l'utente del runner stesso, quindi qualsiasi sessione successiva che il disco serve può leggerli. Se mantieni un `--base-dir` persistente, dimensiona il volume per quella crescita. Lo stesso si applica a qualsiasi configurazione che riavvia il runner sullo stesso filesystem, inclusa la [ricetta Docker Compose](#docker-compose).
* **Con `--remove-session-state`, le directory per sessione non persistono**: avvia il runner con [`--remove-session-state`](/docs/it/self-hosted-environments-reference#runner-cli-flags) per fargli eliminare le directory per sessione di ogni sessione mentre la sessione termina. L'eliminazione è best-effort: le directory rimangono quando il runner viene ucciso prima che la sua pulizia venga eseguita. Il clone canonico e i file che una sessione ha scritto altrove sull'host, come la directory temporanea, rimangono comunque.
* **Con il proxy git, il reset diventa un checkout**: con [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), il runner sanitizza il `.git/` del clone prima di ogni sessione, mantenendo l'object store, i ref, e lo stato shallow ma eliminando l'indice, in modo che ogni sessione paghi un checkout completo dell'albero di lavoro invece di un reset quasi istantaneo; comunque non ri-clona mai. I pre-warm dei submodule non sono supportati sotto il proxy.
* **I clone lunghi non hanno bisogno di workaround**: il runner limita ogni operazione git con un watchdog senza progresso di 120 secondi e un hard cap di 30 minuti, non un timeout piatto, quindi un clone freddo lento che continua a segnalare progresso si completa.

<h2 id="pin-the-version">
  Fissa la versione
</h2>

Il processo Claude Code figlio di ogni sessione esegue il binario del runner stesso, e il runner disattiva l'auto-aggiornamento all'interno delle sessioni che genera, quindi ogni sessione esegue la versione che hai installato sull'host o costruito nell'immagine. Un aggiornamento a livello di host ha effetto la prossima volta che il runner si avvia.

* **Per mantenere una flotta su una versione**: costruisci l'immagine con una versione fissata, o su un host nudo installa una versione specifica e [disabilita gli auto-aggiornamenti](/docs/it/setup#disable-auto-updates)
* **Per aggiornare**: installa la versione più recente o ricostruisci l'immagine, quindi riavvia i runner
* **Plugin**: i marketplace dei plugin non si auto-aggiornano neanche; imposta `FORCE_AUTOUPDATE_PLUGINS=1` nell'ambiente del runner per lasciare che i plugin si auto-aggiornino mentre il binario rimane fissato

<h2 id="scale-the-fleet">
  Scala la flotta
</h2>

Il tuo orchestrator decide quando aggiungere o rimuovere runner. A causa del [blocco one-owner-per-runner](/docs/it/self-hosted-environments#runner-lifecycle), il numero minimo di repliche è il numero di utenti e agenti Claude Tag che ti aspetti siano attivi contemporaneamente; `--capacity` controlla il parallelismo all'interno delle sessioni di un owner, non tra owner.

Due approcci di scaling sono disponibili:

* **Flotta fissa**: esegui un set statico di repliche runner e scala sulle [metriche Prometheus](/docs/it/self-hosted-environments-reference#prometheus-metrics) che ogni runner serve
* **Runner on-demand**: esegui il sottocomando `claude self-hosted-runner orchestrator`, che esegue il polling di Anthropic per le sessioni in coda senza runner disponibile e invoca il tuo hook `spawn-runner` per avviarne uno per sessione. Vedi [Runner on-demand](/docs/it/self-hosted-environments-configuration#on-demand-runners).

<h2 id="known-issues-and-limitations">
  Problemi noti e limitazioni
</h2>

Le seguenti sono le limitazioni in questa versione, con workaround dove uno esiste.

<h3 id="connector-traffic-leaves-your-network">
  Il traffico del connettore lascia la tua rete
</h3>

Anthropic chiama gli strumenti del connettore dalla sua stessa infrastruttura piuttosto che dal tuo runner. Gli strumenti del connettore sono i connettori claude.ai, come GitHub, Slack, e Linear. Quando Claude usa un connettore in una sessione self-hosted, quel traffico va attraverso `api.anthropic.com` piuttosto che originare all'interno del tuo confine di rete.

Per mantenere un connettore fuori dalle sessioni self-hosted, filtralo con le [impostazioni di politica `allowedMcpServers` e `deniedMcpServers`](/docs/it/managed-mcp#policy-based-control-with-allowlists-and-denylists). Claude Code applica queste impostazioni ai connettori che Anthropic consegna così come ai server che semini dall'host runner e ai server che gli utenti aggiungono, quindi se distribuisci una lista di indirizzi consentiti per altri server, Claude Code blocca anche i connettori consegnati. Per mantenere i connettori disponibili insieme a una lista di indirizzi consentiti basata su URL, aggiungi voci che corrispondono ai percorsi proxy di Anthropic per i connettori consegnati:

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

Se il traffico dello strumento deve rimanere all'interno della tua rete, esegui gli strumenti equivalenti come server MCP locali sull'immagine del runner invece. Vedi [Server MCP](/docs/it/self-hosted-environments-configuration#mcp-servers).

<h3 id="some-sessions-don’t-count-as-idle">
  Alcune sessioni non contano come inattive
</h3>

Una sessione che tiene un compito di background che non finisce mai non conta come inattiva, quindi `--release-idle-session-min` non rilascerà lo slot di quella sessione. Una sessione che sta aspettando un'approvazione richiesta dall'interno di una chiamata di strumento in esecuzione non conta neanche come inattiva. Imposta sempre `--kill-session-after-min` insieme ad essa come un hard backstop in modo che nessuna sessione possa tenere uno slot indefinitamente.

`--kill-session-after-min` è un backstop per le sessioni runaway. Su un runner su v2.1.260 o successivo, una sessione che raggiunge il limite non viene terminata immediatamente. Il runner le dà una finestra di grazia, 15 minuti per impostazione predefinita, che puoi cambiare con [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](/docs/it/self-hosted-environments-reference#environment-variable-only-settings):

* Se la sessione sta aspettando il suo utente, il runner la rilascia. Se il suo turno è finito e tiene solo compiti di background, il runner aspetta fino a 60 secondi affinché questi compiti finiscano e poi la rilascia. La sessione riprende quando il suo utente invia il suo prossimo messaggio.
* Se un turno è ancora in esecuzione, il runner aspetta che il turno finisca, o che la sessione aspetti successivamente il suo utente, e poi la rilascia.
* Se la sessione è ancora sul runner quando la finestra di grazia scade, il runner la termina, e il lavoro di qualsiasi turno in esecuzione è perso. Un turno che aspetta un'approvazione richiesta dall'interno di una chiamata di strumento in esecuzione è un modo in cui una sessione sopravvive alla finestra.

Una sessione rilasciata riprende da un clone fresco, quindi il lavoro che non aveva spinto è comunque perso; vedi [Le sessioni riprese perdono il lavoro non spinto](#additional-limitations). Prima di v2.1.260, il runner terminava ogni sessione al limite, dopo aver aspettato al massimo la finestra di grazia affinché un turno in esecuzione finisca.

Imposta il flag sopra la tua sessione più lunga prevista, come `--kill-session-after-min 480` per 8 ore. Per liberare slot dalle conversazioni che diventano inattive, usa `--release-idle-session-min` invece.

<h3 id="additional-limitations">
  Limitazioni aggiuntive
</h3>

* **Le sessioni riprese perdono il lavoro non spinto**: quando una sessione viene rilasciata o il suo runner viene riavviato, e l'utente invia un altro messaggio, la sessione riprende su un runner fresco che clona il repository di nuovo dal suo ramo iniziale, quindi il lavoro che la sessione non aveva spinto è perso. Imposta [`--push-outcome-on-release`](/docs/it/self-hosted-environments-reference#runner-cli-flags) per fare in modo che il runner faccia un best-effort push dei rami di risultato della sessione prima di rilasciarla, in modo che la sessione ripresa inizi da quei commit invece; questo preserva il lavoro impegnato, non un albero di lavoro sporco. Prima di abilitarlo, limita chi può spingere ai ref `claude/*` sul remote di origine, ad esempio con un branch ruleset: al momento della ripresa, il runner recupera il ramo precedentemente spinto senza verificare chi l'ha spinto, quindi chiunque abbia accesso push a quei ref può posizionare contenuto nello spazio di lavoro ripreso. Il runner scarta anche la configurazione per sessione al momento della ripresa, il che significa la directory di configurazione Claude della sessione e qualsiasi stato della shell che la sessione ha scritto; `--push-outcome-on-release` non copre quelli.
* **I repository privati non possono essere aggiunti a metà sessione**: un repository aggiunto a una sessione dopo che è iniziato non viene clonato con credenziali su un runner self-hosted, quindi l'aggiunta fallisce. Seleziona ogni repository di cui la sessione ha bisogno quando la crei.
* **Alcuni connettori non appaiono nelle sessioni self-hosted**: un connettore che non hai ancora connesso nelle Impostazioni di claude.ai non è elencato in una sessione self-hosted, e la sessione non ti chiederà di connettarlo. Connettilo prima nelle Impostazioni, quindi avvia una sessione fresca. L'aggiunta di un connettore a una sessione già in esecuzione non rende i suoi strumenti disponibili a Claude; avvia una sessione fresca per raccogliere un connettore appena aggiunto.

<h3 id="report-an-issue">
  Segnala un problema
</h3>

Per i problemi con gli ambienti self-hosted, contatta il tuo team di account Anthropic.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Per una diagnosi guidata, eseguire il subcommand doctor sull'host del runner. Il subcommand doctor avvia una sessione Claude Code interattiva con i log e lo stato del runner allegati. Accedere con `claude auth login` su quell'host prima in modo che la sessione possa interrogare l'ambiente, i suoi runner e le sue sessioni in coda. Senza questo accesso, ad esempio quando l'host si autentica con una chiave API, è limitato all'endpoint di salute locale, alle metriche e al log del runner, e legge il log solo se è stato avviato il runner con `--log-file`.

```bash theme={null}
claude self-hosted-runner doctor
```

Problemi comuni:

* **Il runner non appare nell'ambiente**: confermare che l'host possa raggiungere `api.anthropic.com` su HTTPS, che il segreto dell'ambiente sia attuale e che l'orologio dell'host sia entro cinque minuti dall'ora reale; uno scostamento maggiore causa il fallimento dell'autenticazione. Il runner registra `[runner:fatal]` con il motivo del rifiuto in caso di errore di autenticazione.
* **Il runner esce all'avvio con `cannot create or write to base directory`**: il runner non può creare o scrivere in `--base-dir`, che per impostazione predefinita è `/workspace`. Correggere la proprietà della directory o puntare `--base-dir` a un percorso scrivibile, come descritto in [Keep the base directory and capacity identical across runners](#keep-the-base-directory-and-capacity-identical-across-runners). Se il runner registra invece `[runner:fatal]` dicendo che il controllo della directory di base è scaduto, la directory si trova su un mount NFS o CSI bloccato. Controllare l'integrità del mount piuttosto che i permessi. Il runner stampa entrambi questi errori di avvio su stderr prima di aprire `--log-file`, quindi cercarli nel terminale o nei log del container della piattaforma piuttosto che nel file di log. Prima della v2.1.225, il runner non controllava la directory di base all'avvio e questa configurazione errata causava il fallimento delle sessioni dopo il pickup.
* **Le sessioni rimangono in coda**: ogni runner online può essere bloccato a un proprietario diverso. Controllare la [metrica](/docs/it/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_locked_account` di ogni runner o il campo `locked_account` della sua riga di log `[runner:health]` per vedere chi la detiene. Entrambi mostrano l'email del proprietario solo dopo che il runner ha ricevuto un token di sessione con un claim `act.email`, che le sessioni di un agente Claude Tag non hanno mai. Senza il claim, il runner non emette alcuna serie `locked_account` e registra `locked_account=yes`, il che indica che il runner è bloccato ma non a quale proprietario. Aggiungere repliche o attendere che un runner esistente si svuoti e si riavvii. Se l'ambiente utilizza runner on-demand, controllare l'orchestrator; vedere [On-demand runners](/docs/it/self-hosted-environments-configuration#on-demand-runners).
* **Le sessioni falliscono immediatamente dopo il pickup**: aprire la sessione in claude.ai/code per vedere l'errore. Le cause più comuni sono le [credenziali git](#configure-git) mancanti nell'immagine del runner e gli strumenti di compilazione non installati. Una directory di base non scrivibile arresta il runner all'avvio invece di far fallire le sessioni. Vedere la voce **Il runner esce all'avvio con `cannot create or write to base directory`** in questo elenco.
* **Le sessioni non riescono a raggiungere la rete attraverso un proxy di uscita autenticante**: quando l'origine impostata con [`--proxy-authorization-command` o `--proxy-authorization-file`](#authenticate-to-an-egress-proxy) fallisce, scade dopo 30 secondi o produce un valore vuoto, il runner risponde a quella connessione con `502 Bad Gateway` e registra il motivo. Il runner redige lo stderr del comando in quel log e non registra mai il valore dell'intestazione. Con `--proxy-authorization-command`, eseguire il comando stesso sull'host per confermare che stampa l'intero valore dell'intestazione su stdout. Se il runner esce invece all'avvio con `could not start the proxy-authorization listener`, non ha potuto aprire il suo listener di loopback.
* **Il runner registra righe `Poll failed` contenenti `rejecting the malformed poll response`**: il runner ha ricevuto una risposta di work-poll il cui corpo non è il JSON previsto dalla coda, il più delle volte perché qualcosa tra il runner e `api.anthropic.com`, come un proxy intercettante o un portale captive, ha risposto con la sua stessa pagina. Il runner rifiuta la risposta, la conta sotto il tipo `transport` della [metrica](/docs/it/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_poll_errors_total`, e riprova secondo la pianificazione di poll fallito descritta in [Session lifecycle](/docs/it/self-hosted-environments#session-lifecycle). Il runner continua a servire le sue sessioni live. Configurare il proxy per passare le risposte da `api.anthropic.com` inalterate. Prima della v2.1.246, il runner leggeva tale risposta come una coda di lavoro vuota, il che potrebbe terminare le sue sessioni live o farla uscire.
* **Il ramo di una sessione non esiste più sul remoto**: per un'origine git che la sessione legge solo, il runner salta quella origine e continua con le rimanenti. Per l'origine a cui la sessione spinge i risultati, un ramo eliminato, tipicamente perché è stato unito e auto-eliminato, fa fallire la sessione con un errore che nomina il repository e il ramo e chiede di ripristinare il ramo e riprovare. Il runner fa fallire la sessione con lo stesso errore quando saltare lascerebbe senza alcun repository. Prima della v2.1.228, tale sessione iniziava in una directory vuota.
* **Una sessione inizia senza uno dei suoi repository**: su un runner senza un [hook `checkout`](/docs/it/self-hosted-environments-configuration#checkout), l'host git può rifiutare il controllo di accesso del runner per un repository che la sessione legge solo. Il runner quindi salta quel repository, registra una riga `[runner:warn] could not access context source` che nomina il rifiuto, e avvia la sessione su quelli rimanenti.

  Il runner salta solo un rifiuto chiaro: l'host risponde che il repository non è stato trovato, git non trova credenziali per l'host, o l'autenticazione fallisce. Un errore di rete, un timeout, o un HTTP `403` comunque fa fallire l'avvio della sessione, così come un rifiuto per un repository a cui la sessione spinge i risultati. Il runner comunque fa fallire una sessione che saltare lascerebbe senza alcun repository. Con [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), il runner salta solo un repository che il git proxy stesso nega.

  Il controllo di accesso viene eseguito di nuovo ogni volta che la sessione inizia su un runner, quindi una volta che l'identità git del runner ha accesso in lettura, il prossimo avvio clona il repository. Prima della v2.1.274, ognuno di questi rifiuti faceva fallire l'avvio della sessione.
* **Le sessioni impiegano minuti per avviarsi**: il clone iniziale di solito domina. Osservare la [metrica](/docs/it/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_session_init_duration_seconds` per confermare e ridurre il clone con un [pre-warmed checkout](#reuse-a-pre-warmed-checkout) o un `CLAUDE_RUNNER_FETCH_DEPTH` più piccolo.
* **I turni falliscono con un 401**: ogni sessione autentica le chiamate del modello con il token di breve durata [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/it/self-hosted-environments-configuration#wrapper-scripts) che il runner recupera da Anthropic e ruota sullo stdin della sessione. Quando un turno termina con un 401 o 403 dall'API del modello, il runner recupera un token fresco e lo passa alla sessione. Il turno fallito non viene riprovato.

  Quando un recupero fallisce, il runner registra una riga `inference_token refresh failed` che dice quando riproverà, e continua a riprovare finché la sessione è in esecuzione.

  Se ogni chiamata inizia a fallire circa 30 minuti in una sessione, uno script wrapper ha probabilmente reciso lo stdin della sessione, quindi le rotazioni dei token non possono raggiungerlo; vedere [Keep stdin and file descriptor 3 attached](/docs/it/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached).

  Prima della v2.1.274, il runner smetteva di riprovare un recupero fallito dopo alcuni tentativi e attendeva il prossimo programmato. Un turno fallito non ha attivato un recupero, quindi ogni turno falliva con un 401 fino al prossimo recupero programmato.
* **Il pod viene terminato durante lo scarico**: aumentare `terminationGracePeriodSeconds` ad almeno il valore che il runner registra all'avvio. Vedere [Shutdown timing](#shutdown-timing).

Una volta inizializzata la registrazione, il runner scrive il suo log del ciclo di vita, incluse le righe `[runner:fatal]`, su stdout e l'output di debug su stderr, il tutto come righe di testo semplice piuttosto che JSON. Gli errori di avvio descritti nelle voci di troubleshooting sopra stampano su stderr prima di quel punto. Acquisire entrambi i flussi con `--log-file`, che consente anche a `self-hosted-runner doctor` di seguirli, o con la raccolta di log della piattaforma.

Il processo figlio di ogni sessione scrive un log di debug separato. In caso di errore il runner visualizza il log della coda insieme alla sessione in claude.ai/code. A meno che non sia stato avviato il runner con [`--remove-session-state`](/docs/it/self-hosted-environments-reference#runner-cli-flags), mantiene anche il log di una sessione fallita su disco e stampa il suo percorso nel log del runner.

<h2 id="what’s-next">
  Cosa c'è dopo
</h2>

* [Personalizza le sessioni](/docs/it/self-hosted-environments-configuration): script wrapper, lifecycle hook, runner on-demand, server MCP, e permessi
* [Testa end to end](/docs/it/self-hosted-environments-testing): verifica una nuova immagine runner da CI prima di promuoverla
* [Riferimento](/docs/it/self-hosted-environments-reference): ogni flag CLI, variabile di ambiente, e metrica
