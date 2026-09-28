> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurazione di rete aziendale

> Configurare Claude Code per ambienti aziendali con server proxy, Autorità di Certificazione (CA) personalizzate e autenticazione Transport Layer Security (mTLS) reciproca.

Claude Code supporta varie configurazioni di rete e sicurezza aziendali attraverso variabili di ambiente. Ciò include l'instradamento del traffico attraverso server proxy aziendali, la fiducia in Autorità di Certificazione (CA) personalizzate e l'autenticazione con certificati Transport Layer Security (mTLS) reciproco per una sicurezza migliorata.

Impostare queste variabili di ambiente prima di avviare Claude Code. Le variabili esportate nella shell vengono lette una sola volta all'avvio, quindi una sessione in esecuzione non rileva i successivi cambiamenti nell'ambiente della shell.

<Note>
  Tutte le variabili di ambiente mostrate in questa pagina possono essere configurate anche in [`settings.json`](/docs/it/settings).
</Note>

<h2 id="proxy-configuration">
  Configurazione del proxy
</h2>

<h3 id="environment-variables">
  Variabili di ambiente
</h3>

Claude Code rispetta le variabili di ambiente proxy standard. Nelle sessioni di Claude Desktop in cui l'app gestisce la connessione del provider, Claude Code le legge solo dalle impostazioni gestite e da `~/.claude/settings.json`; vedere [autenticazione mTLS](#mtls-authentication) per le regole di ambito.

```bash theme={null}
# Proxy HTTPS (consigliato)
export HTTPS_PROXY=https://proxy.example.com:8080

# Proxy HTTP (se HTTPS non disponibile)
export HTTP_PROXY=http://proxy.example.com:8080

# Ignora il proxy per richieste specifiche - formato separato da spazi
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Ignora il proxy per richieste specifiche - formato separato da virgole
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Ignora il proxy per tutte le richieste
export NO_PROXY="*"
```

Funzionano anche le varianti in minuscolo, e Claude Code utilizza la prima che è impostata nell'ordine `https_proxy`, `HTTPS_PROXY`, `http_proxy`, `HTTP_PROXY`.

Claude Code non invia mai le sue connessioni WebSocket a `localhost`, `::1`, o `127.0.0.0/8` attraverso il proxy, quindi non è necessario un voce di loopback in `NO_PROXY` per loro.

<Note>
  Claude Code non supporta proxy SOCKS.
</Note>

<h3 id="basic-authentication">
  Autenticazione di base
</h3>

Se il proxy richiede l'autenticazione di base, includere le credenziali nell'URL del proxy:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Evitare di codificare le password negli script. Utilizzare variabili di ambiente o archiviazione sicura delle credenziali.
</Warning>

<Tip>
  Per proxy che richiedono autenticazione avanzata (NTLM, Kerberos, ecc.), considerare l'utilizzo di un servizio LLM Gateway che supporti il metodo di autenticazione.
</Tip>

<h2 id="ca-certificate-store">
  Archivio certificati CA
</h2>

Per impostazione predefinita, Claude Code si fida sia dei certificati CA Mozilla in bundle che dell'archivio certificati del sistema operativo. La lettura dell'archivio del sistema operativo richiede un runtime con `tls.getCACertificates`: il programma di installazione nativo lo ha sempre, e gli install npm necessitano di Node 22.15 o versioni successive. Su versioni di Node più vecchie, si applicano solo il set in bundle e `NODE_EXTRA_CA_CERTS`. I proxy di ispezione TLS aziendali funzionano senza configurazione aggiuntiva quando il loro certificato radice è installato nell'archivio di fiducia del sistema operativo e il runtime può leggerlo.

`CLAUDE_CODE_CERT_STORE` accetta un elenco separato da virgole di fonti. I valori riconosciuti sono `bundled` per il set CA Mozilla fornito con Claude Code e `system` per l'archivio di fiducia del sistema operativo. L'impostazione predefinita è `bundled,system`.

Per fidarsi solo del set CA Mozilla in bundle:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Per fidarsi solo dell'archivio certificati del sistema operativo:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` non ha una chiave dello schema `settings.json` dedicata. Impostarla tramite il blocco `env` in `~/.claude/settings.json` o direttamente nell'ambiente del processo.
</Note>

<h2 id="custom-ca-certificates">
  Certificati CA personalizzati
</h2>

Se l'ambiente aziendale utilizza una CA personalizzata, configurare Claude Code per fidarsi di essa direttamente:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Autenticazione mTLS
</h2>

Per gli ambienti aziendali che richiedono l'autenticazione tramite certificato client:

```bash theme={null}
# Certificato client per l'autenticazione
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Chiave privata del client
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Facoltativo: Passphrase per la chiave privata crittografata
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code legge i file del certificato e della chiave all'avvio e li rilegge ogni volta che applica le impostazioni, ad esempio quando l'organizzazione modifica il blocco `env` nelle [impostazioni gestite](/docs/it/server-managed-settings) durante la sessione.

Per ruotare il certificato e la chiave, sostituire i file negli stessi percorsi. Claude Code rileva la sostituzione in una sessione in esecuzione senza necessità di riavvio. Quando una richiesta API non riesce con un errore a livello di connessione, ad esempio una reimpostazione della connessione o un errore di handshake TLS, rilegge entrambi i file e ritenta la richiesta con la nuova coppia. Prima della v2.1.232, Claude Code non rileggeva gli errori di connessione, quindi manteneva la coppia già caricata fino al prossimo applicamento delle impostazioni o al riavvio.

Claude Code rilegge i file in risposta alle richieste non riuscite, non monitorando i file per le modifiche:

* **Tempistica**: Claude Code non fa nulla nel momento in cui si sostituiscono i file. Presenta la nuova coppia al nuovo tentativo dopo un errore qualificante, o alla richiesta successiva dopo l'applicazione delle impostazioni, a seconda di quale avvenga per primo.
* **Rifiuti del gateway**: Claude Code rilegge quando il gateway reimposta la connessione o rifiuta l'handshake TLS dopo aver smesso di accettare la coppia precedente. Non rilegge quando il gateway completa l'handshake e risponde con un errore HTTP. In questo caso, Claude Code carica la nuova coppia quando applica successivamente le impostazioni o quando lo si riavvia.
* **Rotazioni parzialmente scritte**: quando Claude Code rilegge mentre la rotazione è in corso di scrittura, ad esempio leggendo un certificato e una chiave che non corrispondono tra loro, mantiene la coppia precedente e rilegge al prossimo errore.
* **Esportatori di telemetria OTLP**: Claude Code mantiene il certificato che gli [esportatori](/docs/it/monitoring-usage#mtls-authentication) hanno caricato al primo utilizzo, quindi riavviare Claude Code affinché un certificato ruotato raggiunga il collettore di telemetria.
* **Disattivare il ricaricamento**: impostare [`CLAUDE_CODE_DISABLE_MTLS_RELOAD_ON_STALE_CONNECTION=1`](/docs/it/env-vars#variables) per disattivare la rilettura degli errori di connessione. Claude Code quindi rileva i file ruotati solo quando applica successivamente le impostazioni o al prossimo avvio.

Per confermare che Claude Code ha rilevato una rotazione, [avviare la sessione con la registrazione del debug](#verify-your-configuration) e cercare `Stale connection — reloaded rotated mTLS client material` nel log. Claude Code non registra questa riga quando rileva la rotazione durante l'applicazione delle impostazioni, quindi una riga mancante da sola non significa che la rotazione non sia riuscita.

Sostituire i file prima che la coppia corrente scada in modo che Claude Code non carichi una coppia già scaduta al prossimo avvio.

Nelle [sessioni cloud](/docs/it/claude-code-on-the-web), l'ambiente di hosting gestisce la connessione all'API, quindi Claude Code ignora le seguenti variabili quando provengono da un blocco `env` del file di impostazioni:

* `CLAUDE_CODE_CLIENT_CERT`
* `CLAUDE_CODE_CLIENT_KEY`
* `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`
* `NODE_EXTRA_CA_CERTS`
* `NODE_TLS_REJECT_UNAUTHORIZED`
* `CLAUDE_CODE_OAUTH_SCOPES`

Claude Code annota ogni chiave ignorata nel log di debug della sessione.

Nelle sessioni di [Claude Desktop](/docs/it/desktop) in cui l'app gestisce la connessione del provider, ad esempio la scheda Code su un [provider di terze parti](/docs/it/third-party-integrations) e le sessioni Cowork, Claude Code legge queste variabili e le variabili proxy `HTTP_PROXY`, `HTTPS_PROXY` e `NO_PROXY` solo dalle [impostazioni gestite](/docs/it/managed-settings) e da `~/.claude/settings.json`: le ignora nei file di impostazioni del repository, quindi un repository estratto non può reindirizzare il percorso TLS o proxy di una sessione le cui credenziali provengono dall'app. In una sessione della scheda Code locale, SSH o WSL acceduta tramite claude.ai, l'app non gestisce la connessione, e Claude Code legge queste variabili da ogni ambito di impostazioni, come qualsiasi sessione di terminale; le [sessioni cloud](/docs/it/claude-code-on-the-web) seguono le regole della sessione cloud sopra ovunque le si avvii. Prima della v2.1.217, Claude Code ignorava queste variabili in ogni file di impostazioni quando l'app gestiva la connessione.

<h2 id="verify-your-configuration">
  Verificare la configurazione
</h2>

Di solito si scopre un indirizzo proxy errato o un percorso certificato non valido da un [errore di connessione o certificato](/docs/it/errors#network-and-connection-errors) su una richiesta successiva, poiché Claude Code non convalida la maggior parte di queste impostazioni quando le legge. L'unica impostazione che controlla all'avvio è l'URL del proxy: quando non riesce ad analizzare il valore, ad esempio uno che manca dello schema `http://`, Claude Code interrompe l'avvio con un errore che nomina la variabile da correggere.

Per confermare che la configurazione è stata caricata prima di inviare una richiesta, avviare Claude Code con la registrazione di debug:

```bash theme={null}
claude --debug
```

L'output di debug va a `~/.claude/debug/<session-id>.txt` anziché al terminale, o a un percorso impostato con `--debug-file <path>`. Nel log, cercare le righe che confermano il caricamento di ogni file:

```text theme={null}
CA certs: Appended extra certificates from NODE_EXTRA_CA_CERTS (/etc/ssl/certs/corp-ca.pem)
mTLS: Loaded client certificate from CLAUDE_CODE_CLIENT_CERT
mTLS: Loaded client key from CLAUDE_CODE_CLIENT_KEY
```

Se Claude Code non riesce a leggere uno di questi file, il log mostra una riga `Failed to read` o `Failed to load` con il motivo.

È inoltre possibile eseguire `/status` in una sessione interattiva e controllare queste righe:

* **Proxy**: mostra l'URL proxy attivo e contrassegna un valore che non riesce ad analizzare come non valido e ignorato.
* **mTLS client cert** e **mTLS client key**: appaiono solo quando i file sono stati caricati, quindi una riga mancante significa che il caricamento non è riuscito e il log di debug contiene il motivo.
* **Additional CA cert(s)**: mostra il percorso `NODE_EXTRA_CA_CERTS` senza verificare che il file sia stato caricato, quindi confermare questo nel log di debug.

<h2 id="apply-network-settings-to-background-agents">
  Applicare le impostazioni di rete agli agenti in background
</h2>

Gli [agenti in background](/docs/it/agent-view) non vengono eseguiti all'interno del terminale che li ha inviati. Un processo supervisore per utente si avvia su richiesta, sopravvive alla shell e ospita ogni sessione `claude agents`, `--bg` e `/background`. Vedere [Come vengono ospitate le sessioni in background](/docs/it/agent-view#how-background-sessions-are-hosted). Questo cambia il modo in cui la configurazione su questa pagina raggiunge quelle sessioni.

<h3 id="set-network-variables-in-settings-not-the-shell">
  Impostare le variabili di rete nelle impostazioni, non nella shell
</h3>

Il supervisore è un processo unico condiviso da ogni terminale. Eredita l'ambiente della shell che lo avvia per prima, e un supervisore installato dal sistema operativo non riceve alcun ambiente di shell. Se esportate un proxy, un percorso CA o una variabile mTLS solo nella vostra shell, raggiunge gli agenti in background quando quella shell ha avviato a freddo il supervisore, e silenziosamente non lo fa quando una shell diversa lo ha fatto.

Inserite le stesse variabili nel blocco `env` di `~/.claude/settings.json` o nelle [impostazioni gestite](/docs/it/settings). Ogni variabile su questa pagina può essere impostata lì, e le impostazioni sono l'unica configurazione che raggiunge ogni sessione in background su ogni macchina.

<h3 id="configure-a-corporate-launcher-as-a-setting">
  Configurare un launcher aziendale come impostazione
</h3>

Alcune organizzazioni richiedono che ogni processo Claude Code si avvii attraverso un launcher aziendale che applica sandboxing, controlli di rete o iniezione di credenziali. Il supervisore e i suoi worker avviano Claude Code da un percorso fisso piuttosto che cercando `claude` su `PATH`, quindi ogni agente in background bypassa un wrapper che posizionate prima su `PATH`.

Impostate l'impostazione [`processWrapper`](/docs/it/settings-reference#processwrapper) per anteporre il supervisore, i suoi worker e gli altri processi in background elencati in [Cosa copre il launcher](/docs/it/corporate-launcher#what-the-launcher-covers) con il vostro launcher. La variabile di ambiente equivalente [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/it/env-vars) ha la precedenza quando entrambe sono impostate, ed è soggetta alla stessa regola: consegnatela attraverso le impostazioni gestite o `~/.claude/settings.json`, non un'esportazione di shell. [Eseguire Claude Code dietro un launcher aziendale](/docs/it/corporate-launcher) copre il contratto che il launcher deve soddisfare, cosa raggiunge e cosa non raggiunge, e come implementarlo.

<Note>
  Un supervisore già in esecuzione mantiene la configurazione di avvio con cui è stato avviato. Dopo aver distribuito l'impostazione del launcher, eseguite [`claude daemon stop --any`](/docs/it/agent-view#the-supervisor-process) in modo che il prossimo `claude agents` o `--bg` avvii un supervisore che la rispetti. Un servizio installato richiede `claude daemon stop` senza `--any`.
</Note>

<h2 id="streaming-idle-watchdogs">
  Watchdog di inattività dello streaming
</h2>

Claude Code esegue quattro timer indipendenti che interrompono una risposta del modello in streaming quando rimane silenziosa, in modo che una connessione morta fallisca e riprovi invece di rimanere bloccata. La scadenza del primo byte copre l'attesa degli header di risposta, prima che sia arrivata una parte della risposta. Ognuno degli altri tre monitora una risposta attiva per un segnale diverso.

| Timer                           | Interrompe quando                                                                                                                                                                                                                                   | Viene eseguito su                                                                                                                                                                                                                                                                                                                                                                 | Timeout predefinito                                                                                                 |
| :------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| Scadenza del primo byte         | Nessun header di risposta arriva dopo che Claude Code invia la richiesta                                                                                                                                                                            | API Anthropic diretta e [Claude Platform on AWS](/docs/it/claude-platform-on-aws), incluso attraverso un proxy HTTPS, ma non quando `ANTHROPIC_BASE_URL` o `ANTHROPIC_AWS_BASE_URL` le instradano attraverso un [gateway](/docs/it/gateways). Opt-in su Amazon Bedrock con `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; non viene eseguito su Google Cloud's Agent Platform o Microsoft Foundry | 180 secondi sull'API Anthropic diretta, 300 secondi altrove, più un secondo per ogni 32KB del corpo della richiesta |
| Watchdog a livello di evento    | Nessun evento di risposta viene analizzato. Su connessioni in cui viene eseguito il watchdog a livello di byte, i byte in arrivo, inclusi i ping keep-alive, ripristinano anche questo watchdog, per circa cinque minuti senza un evento analizzato | Ogni provider                                                                                                                                                                                                                                                                                                                                                                     | 300 secondi                                                                                                         |
| Watchdog a livello di byte      | Nessun byte arriva sul filo, inclusi i ping keep-alive SSE                                                                                                                                                                                          | API Anthropic diretta, [Claude Platform on AWS](/docs/it/claude-platform-on-aws), e [gateway](/docs/it/gateways) connessioni, incluso un `ANTHROPIC_BASE_URL` personalizzato. Opt-in su risposte Amazon Bedrock `vnd.amazon.eventstream` con `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; non viene eseguito su Google Cloud's Agent Platform o Microsoft Foundry                               | 180 secondi sull'API Anthropic diretta, 300 secondi altrove                                                         |
| Timeout di inattività del corpo | Nessun byte arriva per 5 minuti                                                                                                                                                                                                                     | Provider diversi dall'API Anthropic diretta e da Claude Platform on AWS, a meno che [`API_FORCE_IDLE_TIMEOUT`](/docs/it/env-vars) non lo modifichi                                                                                                                                                                                                                                     | 5 minuti                                                                                                            |

Configurare i timer con queste variabili, ognuna descritta in dettaglio nel [riferimento delle variabili di ambiente](/docs/it/env-vars):

* `CLAUDE_ENABLE_STREAM_WATCHDOG` e `CLAUDE_ENABLE_BYTE_WATCHDOG` forzano il watchdog corrispondente attivo con `1` o spento con `0`, all'interno delle connessioni elencate nella tabella; nessuna delle due variabili estende un watchdog a un tipo di connessione che non copre. `CLAUDE_ENABLE_BYTE_WATCHDOG` impostato su `0` disattiva anche la scadenza del primo byte.
* `CLAUDE_STREAM_IDLE_TIMEOUT_MS` imposta il timeout di entrambi i watchdog. Claude Code aumenta i valori inferiori a 5 minuti a 5 minuti e limita il valore a 30 minuti per il watchdog a livello di byte.
* `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` imposta il timeout del watchdog a livello di byte senza modificare quello del watchdog a livello di evento, limitato tra 10 secondi e 30 minuti, e ha la precedenza su `CLAUDE_STREAM_IDLE_TIMEOUT_MS` per quel watchdog.
* `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` imposta la scadenza del primo byte direttamente. Lasciarlo non impostato e Claude Code utilizza il timeout del watchdog a livello di byte, quindi `CLAUDE_STREAM_IDLE_TIMEOUT_MS` e `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` modificano anche la scadenza. Per i limiti, l'indennità di caricamento, il limite `API_TIMEOUT_MS` e quanto tempo l'attesa di ripetizione dopo un'interruzione senza risposta, vedere [Nessuna risposta dall'API](/docs/it/errors#no-response-from-api).
* `API_FORCE_IDLE_TIMEOUT` impostato su `0` disattiva il timeout di inattività del corpo, e impostato su `1` lo attiva per ogni provider. I watchdog vengono eseguiti indipendentemente da esso, quindi per consentire a uno stream di pausa più lungo dei loro soglie, aumentare o disabilitare anche loro.

Quando un watchdog interrompe uno stream bloccato, Claude Code tratta l'interruzione come un errore a metà flusso, e quello che si vede dipende da quanto lontano era arrivata la risposta. Claude Code riprova la richiesta o termina il turno con un errore, mantiene l'output completato e mostra un [avviso di risposta incompleta](/docs/it/errors#the-response-above-may-be-incomplete), o termina il turno normalmente. [Tentativi automatici](/docs/it/errors#automatic-retries) dice dove si applica ogni risultato.

In una [sessione non interattiva](/docs/it/headless), e per la risposta di un subagent in qualsiasi sessione, Claude Code potrebbe prima richiedere a Claude di continuare la risposta interrotta; [la voce di quell'avviso](/docs/it/errors#the-response-above-may-be-incomplete) dice quando lo fa e quando si vede ancora l'avviso.

Quando la scadenza del primo byte si attiva, nessuna risposta è iniziata, quindi non c'è output parziale da mantenere. Per come Claude Code rinvia la richiesta e quando il turno termina invece, vedere [Nessuna risposta dall'API](/docs/it/errors#no-response-from-api).

<h2 id="network-access-requirements">
  Requisiti di accesso alla rete
</h2>

Claude Code richiede accesso ai seguenti URL. Inserite questi indirizzi nella lista di autorizzazione della vostra configurazione proxy e delle regole firewall, soprattutto in ambienti di rete containerizzati o limitati. Il controllo di connettività della configurazione al primo avvio punta qui quando non riesce a raggiungere `api.anthropic.com` o `platform.claude.com`; consultate [Impossibile connettersi ai servizi Anthropic](/docs/it/errors#unable-to-connect-to-anthropic-services) per i messaggi del controllo e i passaggi di recupero.

| URL                                  | Richiesto per                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                  | Richieste API Claude, incluso il [controllo di sicurezza del dominio](/docs/it/data-usage#webfetch-domain-safety-check) WebFetch, recuperi dei flag di funzionalità e registrazione degli eventi di telemetria                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `claude.ai`                          | Autenticazione dell'account claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `claude.com`                         | L'accesso all'account claude.ai apre una pagina `claude.com` nel browser, che reindirizza a `claude.ai`; le ricerche di documentazione WebFetch pre-approvate raggiungono anche questo host dalla CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `platform.claude.com`                | Autenticazione dell'account Anthropic Console. Lo scambio, il rinnovo e la revoca dei token OAuth vanno anche a questo host per gli account claude.ai, quindi sia gli accessi a Console che a claude.ai lo richiedono                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `mcp-proxy.anthropic.com`            | [Connettori MCP da claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai), inclusi i connettori che un amministratore dell'organizzazione configura. Il traffico dei connettori viene instradato attraverso questo proxy; i connettori sono abilitati per impostazione predefinita per gli utenti autenticati con claude.ai. Per impedire a Claude Code di recuperarli, impostate [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/it/env-vars) o l'impostazione [`disableClaudeAiConnectors`](/docs/it/settings-reference#disableclaudeaiconnectors)                                                                                                                                     |
| `downloads.claude.ai`                | Download degli eseguibili dei plugin; programma di installazione nativo, aggiornamento automatico nativo e controlli della versione di aggiornamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `storage.googleapis.com`             | Conteggi di installazione dei plugin e metadati mostrati in `/plugin`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `storage.googleapis.com`             | Programma di installazione nativo e aggiornamento automatico nativo nelle versioni precedenti a 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `registry.npmjs.org`                 | Installazioni di plugin (recupero di pacchetti plugin da fonte npm e installazione delle dipendenze dei pacchetti Node.js dei plugin), server MCP lanciati con `npx` e il registro dei pacchetti per le installazioni npm e bun di Claude Code stesso                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `bridge.claudeusercontent.com`       | Bridge WebSocket dell'estensione [Claude in Chrome](/docs/it/chrome)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `*.frame.claudeusercontent.com`      | Letture di contenuto [Artifact](/docs/it/artifacts). La CLI recupera i file di un artifact da questo host quando Claude ne apre uno, e solo quando lo strumento Artifact è [disponibile](/docs/it/artifacts#availability) per il vostro account. Per disattivare lo strumento e eliminare questo requisito, impostate [`"enableArtifact": false`](/docs/it/settings-reference#enableartifact) o [`CLAUDE_CODE_DISABLE_ARTIFACT=1`](/docs/it/env-vars); Claude Code rispetta anche l'impostazione deprecata [`disableArtifact`](/docs/it/settings-reference#disableartifact). Consultate [Disabilitare gli artifact](/docs/it/artifacts#disable-artifacts) per come queste impostazioni interagiscono |
| `github.com`                         | Clonazione di [marketplace di plugin](/docs/it/plugins/overview) e plugin ospitati su GitHub, incluso il marketplace ufficiale Anthropic, su HTTPS o SSH. Per clonare sorgenti GitHub `owner/repo` solo su HTTPS, impostate [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/it/env-vars)                                                                                                                                                                                                                                                                                                                                                                                             |
| `raw.githubusercontent.com`          | Feed del changelog per [`/release-notes`](/docs/it/commands). Nelle sessioni interattive, Claude Code lo recupera anche in background all'avvio quando il changelog memorizzato nella cache non copre ancora la versione in esecuzione, ad esempio al primo avvio dopo un aggiornamento; le sessioni non interattive e cloud non lo recuperano mai                                                                                                                                                                                                                                                                                                                          |
| `*-review.googlesource.com`          | Ricerca di modifiche Gerrit su checkout `googlesource.com`. Quando una sessione di una scheda Claude Desktop Code si avvia o riprende su un checkout [attendibile](/docs/it/permissions#project-allow-rules-and-workspace-trust) il cui `origin` è un host `googlesource.com`, Claude Code chiede anonimamente al server `-review` di quell'host la modifica aperta corrispondente al `Change-Id` di HEAD, una volta per avvio o ripresa. Altri tipi di sessione saltano la ricerca e nessun altro host Gerrit viene contattato. Facoltativo: disabilitare con [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars)                                                   |
| `http-intake.logs.us5.datadoghq.com` | Eventi di telemetria operazionale, inviati solo quando la CLI utilizza direttamente l'API Anthropic, mai per Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Facoltativo: disabilitare con [`DISABLE_TELEMETRY`](/docs/it/data-usage#telemetry-services) o `DO_NOT_TRACK`                                                                                                                                                                                                                                                                                                                                                                                |
| `browser-intake-us5-datadoghq.com`   | Rapporti di errore operazionali, inviati quando la CLI utilizza direttamente l'API Anthropic e un gate di rollout lato server li abilita. Facoltativo: disabilitare con `DISABLE_ERROR_REPORTING` o `DISABLE_TELEMETRY`; consultate [Servizi di telemetria](/docs/it/data-usage#telemetry-services)                                                                                                                                                                                                                                                                                                                                                                         |
| `formulae.brew.sh`                   | Controlli della versione di aggiornamento nelle installazioni Homebrew. Altri metodi di installazione non contattano questo host                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `code.claude.com`                    | Ricerche di documentazione Claude Code dall'agente claude-code-guide integrato e richieste WebFetch pre-approvate. Bloccare questo host influisce solo sulle ricerche di documentazione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

Se installate Claude Code tramite npm o gestite la vostra distribuzione binaria, gli utenti finali non hanno bisogno dei usi del programma di installazione nativo e dell'aggiornamento automatico di `downloads.claude.ai`, ma le installazioni npm e bun hanno bisogno del loro registro dei pacchetti, `registry.npmjs.org`, a meno che la vostra organizzazione non lo specchi. Gli altri usi nella tabella si applicano indipendentemente dal metodo di installazione.

I due host di intake Datadog trasportano solo telemetria operazionale facoltativa, e l'impostazione [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars) disabilita entrambi. Le sessioni su provider di terze parti non inviano mai a questi host, anche quando una piattaforma imposta [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/it/env-vars) e le metriche di telemetria sono abilitate per impostazione predefinita. Consultate [Servizi di telemetria](/docs/it/data-usage#telemetry-services) per tutto ciò che Claude Code invia e come disabilitarlo prima di finalizzare la vostra lista di autorizzazione.

Quando si utilizza [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry) o una sessione [gateway di app Claude](/docs/it/claude-apps-gateway) con accesso effettuato, il traffico del modello e l'autenticazione vanno al vostro provider o gateway invece di `api.anthropic.com`, `claude.ai` o `platform.claude.com`. Lo strumento WebFetch chiama ancora `api.anthropic.com` per il suo [controllo di sicurezza del dominio](/docs/it/data-usage#webfetch-domain-safety-check) a meno che non impostiate `skipWebFetchPreflight: true` nelle [impostazioni](/docs/it/settings).

Quando si instrada attraverso un [gateway LLM](/docs/it/llm-gateway) con [`ANTHROPIC_BASE_URL`](/docs/it/llm-gateway-connect#set-the-base-url-and-credential), il controllo di disponibilità della [modalità veloce](/docs/it/fast-mode) chiama ancora `api.anthropic.com` piuttosto che l'URL di base del gateway. Il controllo rispetta un proxy HTTP configurato, quindi dove un blocco di rete è la causa, una voce di lista di autorizzazione per `api.anthropic.com` nel proxy è la soluzione. Un blocco di rete non riesce il controllo solo dove l'host non è raggiungibile nemmeno attraverso il proxy, e la modalità veloce segnala quindi un errore di connettività. Lo stesso errore di connettività appare quando il controllo presenta una credenziale emessa dal gateway che Anthropic rifiuta; l'inserimento nella lista di autorizzazione non aiuta lì, poiché nulla è bloccato. Consultate [usare la modalità veloce dietro proxy e gateway LLM](/docs/it/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) per le variabili che la ripristinano.

<h3 id="organization-ip-allowlists-and-proxy-egress">
  Liste di autorizzazione IP dell'organizzazione e uscita proxy
</h3>

Se la vostra organizzazione ha [inserimento in lista di autorizzazione IP](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) abilitato per Claude, instradare `bridge.claudeusercontent.com` attraverso la stessa uscita proxy di `claude.ai` e `api.anthropic.com`, ad esempio inserendolo nello stesso segmento di app Zscaler o nella stessa politica di steering Netskope. Se non potete instradarlo in quel modo, aggiungete l'indirizzo di uscita che il vostro proxy utilizza per quell'host alla lista di autorizzazione IP della vostra organizzazione, ma solo quando quell'indirizzo è dedicato alla vostra organizzazione: un intervallo di uscita proxy condiviso ammette anche gli altri clienti del fornitore del proxy.

Anthropic controlla le connessioni a `bridge.claudeusercontent.com` rispetto alla lista di autorizzazione IP della vostra organizzazione utilizzando l'indirizzo da cui arrivano. Se il vostro proxy invia il traffico per quell'host attraverso un indirizzo che non è in quella lista di autorizzazione, Claude Code non può connettersi all'estensione [Claude in Chrome](/docs/it/chrome) anche se il resto di Claude Code funziona.

<h3 id="github-allow-lists-and-firewalls">
  Liste di autorizzazione GitHub e firewall
</h3>

[Claude Code sul web](/docs/it/claude-code-on-the-web) negli ambienti ospitati da Anthropic e [Code Review](/docs/it/code-review) si connettono ai vostri repository dall'infrastruttura gestita da Anthropic; le sessioni in un [ambiente auto-ospitato](/docs/it/self-hosted-environments) si connettono dall'interno della vostra rete, a meno che il runner non opti per il [proxy git Anthropic](/docs/it/self-hosted-environments-deploy#use-the-anthropic-git-proxy), che recupera dal lato di Anthropic.

Se la vostra organizzazione GitHub Enterprise Cloud limita l'accesso per indirizzo IP, abilitate [l'ereditarietà della lista di autorizzazione IP per le app GitHub installate](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps) e inoltre [aggiungete una voce di lista di autorizzazione](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) per gli [indirizzi IP in uscita](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) di Anthropic. L'ereditarietà copre solo le richieste che l'app GitHub di Claude fa come installazione, non le richieste che fa per conto dei vostri utenti. Per altri firewall, consultate gli [indirizzi IP API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

Per istanze [GitHub Enterprise Server](/docs/it/github-enterprise-server) auto-ospitate dietro un firewall, inserite nella lista di autorizzazione gli [indirizzi IP in uscita](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) di Anthropic in modo che l'infrastruttura Anthropic possa raggiungere il vostro host GHES per clonare repository e pubblicare commenti di revisione. Le sessioni in un [ambiente auto-ospitato](/docs/it/self-hosted-environments-deploy#configure-git) raggiungono il vostro host GHES dall'interno della vostra rete invece, quindi tale esposizione si applica solo alle sessioni ospitate da Anthropic, ai flussi pre-sessione ospitati come il selettore di repository, e ai runner auto-ospitati che optano per il [proxy git Anthropic](/docs/it/self-hosted-environments-deploy#use-the-anthropic-git-proxy), che recupera dal lato di Anthropic. Per un host GHES che è instradabile solo all'interno della vostra rete, il [connettore SCM](/docs/it/self-hosted-environments-reference#scm-connector-flags) trasporta i flussi pre-sessione ospitati su una connessione in uscita invece, quindi la lista di autorizzazione non è necessaria per loro.

<h3 id="desktop-and-claude-ai">
  Desktop e claude.ai
</h3>

La tabella precedente copre la CLI autonoma. L'app Claude Desktop e claude.ai in un browser caricano il loro codice di applicazione e il contenuto dell'utente da host CDN Anthropic aggiuntivi, inclusi `assets-proxy.anthropic.com` e gli altri origini `*.claudeusercontent.com` che servono [artifact](/docs/it/artifacts) in quelle app. Consentire `claude.ai` mentre si bloccano quegli host produce una pagina vuota piuttosto che un errore. Consultate [requisiti di accesso alla rete](/docs/it/desktop#network-access-requirements) nella pagina Desktop.

Un [artifact](/docs/it/artifacts) che carica un carattere da [Google Fonts](/docs/it/artifacts#improve-the-visual-design) richiede anche `fonts.googleapis.com` e `fonts.gstatic.com`. Entrambi gli host sono facoltativi. Se li bloccate, gli artifact vengono renderizzati in caratteri di fallback. Bloccate con un rifiuto veloce piuttosto che un drop silenzioso in modo che la richiesta di carattere fallisca immediatamente invece di ritardare il primo rendering della pagina.

Gli artifact possono anche caricare librerie JavaScript, come React o un pacchetto di grafici, da `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` e `unpkg.com`, e da nessun altro host esterno. Se bloccate quegli host, le parti di un artifact che dipendono da una libreria non funzionano, e a differenza di un carattere bloccato, una libreria bloccata non ha fallback. Bloccate con un rifiuto veloce anche qui, in modo che una richiesta di libreria bloccata fallisca immediatamente piuttosto che rimanere in sospeso fino al timeout.

<h2 id="additional-resources">
  Risorse aggiuntive
</h2>

* [File di configurazione e precedenza](/docs/it/settings)
* [Riferimento delle variabili di ambiente](/docs/it/env-vars)
* [Guida alla risoluzione dei problemi](/docs/it/troubleshooting)
