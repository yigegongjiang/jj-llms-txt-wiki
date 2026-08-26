> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Distribuzione e operazioni del gateway delle app Claude

> Registrare il gateway con il vostro IdP, costruire il container, distribuire su Kubernetes o Cloud Run, e gestirlo: controlli di integrità, rotazione dei segreti, aggiornamenti e sicurezza.

Questa pagina copre il lato operativo dell'esecuzione del [gateway delle app Claude](/docs/it/claude-apps-gateway): registrazione di un client OAuth nel vostro provider di identità (IdP), distribuzione del gateway come container, e gestione quotidiana. Per ogni opzione nel file `gateway.yaml` che il gateway legge all'avvio, consultare il [Riferimento di configurazione](/docs/it/claude-apps-gateway-config).

Una distribuzione in produzione segue quattro passaggi in ordine, e le sezioni sottostanti li corrispondono. I primi due sono dove fate scelte; i secondi due sono materiale di riferimento da consultare una volta che è in esecuzione.

1. [Configurare il vostro provider di identità](#identity-provider-setup): registrare il client OAuth e controllare le note specifiche per IdP per Okta, Entra e Google
2. [Distribuire il gateway](#deployment): costruire un'immagine container con versione fissa ed eseguirla su Kubernetes, Cloud Run, o la vostra piattaforma. Questa sezione copre anche decisioni relative a costi, bypass, gateway multipli e serverless
3. [Configurare le operazioni](#operations): log, sonde di integrità, comportamento in caso di interruzione, rotazione dei segreti e aggiornamenti. Riferimento per quando state collegando il monitoraggio e i runbook
4. [Esaminare la postura di sicurezza](#security): dove fluiscono i dati, il modello di minaccia e le risposte sulla conformità. Riferimento per una revisione della sicurezza

Se un accesso o un avvio fallisce lungo il percorso, andare direttamente a [Troubleshooting](#troubleshooting), che è indicizzato in base all'errore che vedete.

<Note>
  **Distribuire sulla vostra rete privata.** Claude Code si connette solo a un gateway il cui indirizzo è privato. Questo è un meccanismo di sicurezza, perché un gateway affidabile può inviare impostazioni che eseguono comandi sulle macchine degli sviluppatori. Mettere il gateway dietro un load balancer interno o una VPN e dargli un nome host che si risolve solo in indirizzi IP privati.

  Gli endpoint del gateway pubblico gestiti da Anthropic sono l'eccezione: `/login` li accetta su `https://`. Questi sono un piccolo insieme fisso di gateway che Anthropic stesso gestisce; non sono un'opzione di distribuzione che potete selezionare o configurare. L'elenco è compilato in Claude Code, quindi nessuna configurazione può aggiungere un nome host ad esso e nessun gateway che ospitate si qualifica per l'esenzione. Prima della v2.1.206, `/login` rifiutava questi endpoint come qualsiasi altro indirizzo pubblico.
</Note>

<h2 id="identity-provider-setup">
  Configurazione del provider di identità
</h2>

Registra un'applicazione web OAuth/OpenID Connect (OIDC) confidenziale con un singolo URI di reindirizzamento, `https://<gateway>/oauth/callback`, e assegnala agli utenti o ai gruppi che dovrebbero avere accesso al gateway.

Qualsiasi IdP conforme a OIDC funziona: Okta, Microsoft Entra ID, Google Workspace, Keycloak, Dex, PingFederate e altri. L'IdP deve soddisfare tre requisiti:

* Serve `/.well-known/openid-configuration`, su HTTPS in produzione; il gateway accetta un [`http://` issuer](/docs/it/claude-apps-gateway-config#oidc), e un issuer loopback richiede inoltre `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`
* Supporta il flusso authorization-code. PKCE (Proof Key for Code Exchange) è attivo per impostazione predefinita; disabilitalo con `oidc.use_pkce: false` per IdP che non lo supportano
* Restituisce `email` e facoltativamente `groups` nell'id\_token, o li serve dall'endpoint userinfo con `oidc.userinfo_fallback: true`

Per PKI privata, imposta `oidc.ca_cert_pem`.

Alcuni provider gestiscono i claim di email e gruppo diversamente:

* **Okta**: il server di autorizzazione dell'organizzazione in `https://example.okta.com` restituisce un id\_token sottile che omette `email` e `groups`, quindi imposta `oidc.userinfo_fallback: true` ogni volta che lo usi come `issuer`. Un server di autorizzazione personalizzato come `https://example.okta.com/oauth2/default` che include `email` e facoltativamente `groups` nell'id\_token li emette direttamente e non ha bisogno di fallback. Okta emette `groups` solo quando lo scope `groups` è richiesto in `oidc.scopes` e il filtro dei claim dei gruppi dell'app lo consente; `userinfo_fallback` non può riempire un claim per il quale l'IdP non è stato interrogato.
* **Microsoft Entra ID**: `issuer` = `https://login.microsoftonline.com/<tenant-id>/v2.0`. Entra emette Object ID dei gruppi piuttosto che nomi, quindi usa i GUID in `managed.policies.match.groups`, o usa App Roles per nomi leggibili. Se il tuo tenant emette ruoli sotto `roles` invece di `groups`, imposta `oidc.groups_claim: roles`.
* **Google Workspace**: `issuer` = `https://accounts.google.com`. L'id\_token di Google non contiene gruppi. Per usare `allowed_groups` basato su gruppi o `managed.policies` con Google come IdP, configura [`oidc.google_groups`](/docs/it/claude-apps-gateway-config#oidc), che cerca i gruppi di ogni utente tramite l'API Admin SDK Directory usando un account di servizio con delega a livello di dominio. Senza di esso, usa `oidc.allowed_email_domains` per il gating dell'appartenenza e `managed.policies.match.email_domain` per l'assegnazione della policy. Google ignora anche lo scope standard `offline_access`. Per i token di aggiornamento, imposta `oidc.scopes: [openid, profile, email]` e `oidc.extra_auth_params: { access_type: offline, prompt: consent }`.

Per supporto con un provider di identità non trattato sopra, vedi [Troubleshooting](#troubleshooting).

<Warning>
  I token di aggiornamento consentono al gateway di rinnovare silenziosamente la sessione di uno sviluppatore, senza inviare lo sviluppatore al browser. Guidano anche il deprovisioning, perché quando l'IdP disabilita un utente, il prossimo aggiornamento fallisce e la sessione termina entro `ttl_hours`. Il gateway richiede `offline_access` per impostazione predefinita per ottenere un token di aggiornamento. Se il tuo IdP richiede il consenso esplicito per l'accesso offline, configura il client OAuth per consentirlo.

  Se il tuo IdP non può emettere token di aggiornamento affatto, il gateway funziona comunque, ma non c'è rinnovamento silenzioso, quindi gli sviluppatori rieseguono l'accesso al browser quando la loro sessione scade. Per evitare che ciò accada ogni ora, aumenta [`session.ttl_hours`](/docs/it/claude-apps-gateway-config#session) a `8` o `12`. Il compromesso è la latenza di deprovisioning, perché senza token di aggiornamento un utente disabilitato mantiene l'accesso fino a quando il TTL più lungo non scade.
</Warning>

<h2 id="deployment">
  Distribuzione
</h2>

Il gateway è un singolo binario Linux. Si scala orizzontalmente perché le repliche sono senza stato e Postgres è il livello di coordinamento condiviso. Eseguilo come esegui i servizi senza stato nel tuo ambiente. Il resto di questa sezione afferma cosa l'immagine ha bisogno, con brevi note per Kubernetes e Cloud Run.

Il gateway è progettato per funzionare all'interno della tua rete, perché contiene le tue credenziali upstream e agisce come l'unico punto di uscita per l'inferenza. Può funzionare ovunque i tuoi sviluppatori e il tuo IdP possono raggiungere su HTTPS; trattalo come qualsiasi altro servizio che contiene una credenziale di produzione.

Alcune decisioni modellano la distribuzione oltre a dove viene eseguita:

* **Costo**: non c'è una licenza separata o una tassa per posto per il gateway; fa parte del binario `claude`. Paghi per l'inferenza attraverso il tuo impegno cloud o Anthropic esistente, più il calcolo per il container e il tuo collettore di telemetria.
* **Bypass**: il gateway non applica che l'unica rotta a un modello passi attraverso di esso. Uno sviluppatore con la propria credenziale può comunque chiamare il provider direttamente, quindi chiudere quel percorso è una decisione di policy di rete, ad esempio bloccando l'uscita verso `api.anthropic.com` tranne dal gateway. Bloccare anche quell'uscita interrompe il [controllo di sicurezza del dominio WebFetch](/docs/it/data-usage#webfetch-domain-safety-check), che chiama `api.anthropic.com` da ogni macchina dello sviluppatore; imposta `skipWebFetchPreflight: true` nella policy gestita per disabilitarlo.
* **Gateway multipli**: ogni gateway è una distribuzione separata con la sua configurazione. Il CLI memorizza la sua impronta digitale di fiducia e le credenziali per nome host del gateway, quindi diversi team possono connettersi a gateway diversi senza conflitto. Per servire più emittenti OIDC, esegui istanze separate.
* **Serverless**: Cloud Run funziona; imposta `min-instances: 1` per evitare il cold start della scoperta OIDC. Lambda e Cloud Functions no, perché il gateway è un server HTTP a lunga durata.

Ogni topologia di produzione qui mette un proxy L7, come un Ingress, il front-end di Cloud Run, o un ALB, davanti alle repliche HTTP semplici. Imposta [`listen.trusted_proxies`](/docs/it/claude-apps-gateway-config#listen) agli intervalli di origine del proxy in modo che il gateway legga gli IP client da `X-Forwarded-For`. Il gateway onora l'intestazione solo quando il peer TCP è affidabile; l'[esempio elaborato di Google Cloud](/docs/it/claude-apps-gateway-on-gcp) ha valori concreti per topologia. Senza proxy affidabili, ogni richiesta sembra provenire dall'IP del proxy, il che comprime i limiti di velocità per IP in un bucket condiviso e registra l'IP del proxy negli eventi di audit.

<h3 id="container-image">
  Immagine del container
</h3>

Costruisci la tua immagine attorno al binario `claude` nativo dalla versione standard di Claude Code:

1. Scarica la build Linux per l'architettura della tua immagine da una versione fissa; consulta [Installa una versione specifica](/docs/it/setup#install-a-specific-version) per l'URL di download.
2. Verificalo rispetto al `manifest.json` firmato con GPG della versione come descritto in [Integrità binaria e firma del codice](/docs/it/setup#binary-integrity-and-code-signing).
3. Copialo nel contesto di build.

Specchia la versione nel tuo registro interno se le tue build non possono raggiungere l'host della versione, e fissa la versione che la tua flotta esegue.

Oltre al binario, l'immagine ha bisogno di:

* **Un'immagine basata su glibc**: la build glibc ha solo dipendenze dinamiche dalle librerie glibc. Le immagini basate su Musl hanno bisogno della build `linux-x64-musl` o `linux-arm64-musl` più pacchetti aggiuntivi; consulta [Configurazione di Alpine Linux](/docs/it/setup#alpine-linux-and-musl-based-distributions).
* **Una directory di stato scrivibile**: il gateway funziona come qualsiasi utente, ma le immagini minime non hanno home scrivibile. Imposta `CLAUDE_CONFIG_DIR` a un percorso scrivibile come `/tmp/.claude`.
* **Il comando del container**: `claude gateway --config /etc/claude/gateway.yaml`, con il file di configurazione montato in sola lettura e i segreti forniti come variabili di ambiente; il gateway ascolta su `listen.port`, predefinito `8080`.

<h3 id="kubernetes">
  Kubernetes
</h3>

Esegui il gateway come Deployment, come qualsiasi servizio senza stato:

* Monta la configurazione da una ConfigMap e i segreti da un Secret; fai riferimento ai segreti nel YAML tramite `${file:/path/to/secret}` o come variabili di ambiente
* Termina TLS all'Ingress e imposta `listen.public_url` al nome host dell'Ingress
* Punta la sonda di readiness a `GET /readyz` e la sonda di liveness a `GET /healthz`

<Note>
  **Workload identity**

  Preferisci l'identità del workload della piattaforma rispetto alle chiavi statiche: IRSA su EKS per Bedrock e per Claude Platform su AWS, Workload Identity su GKE per Agent Platform, e workload identity su AKS per Foundry. Imposta `auth: {}` nel blocco upstream, o `use_azure_ad: true` per Foundry, e il gateway raccoglie l'identità del pod attraverso la catena di credenziali predefinita di quel provider. Per un accoppiamento cross-cloud, come un upstream Bedrock su GKE, imposta credenziali esplicite nel blocco `auth` dell'upstream. Il [riferimento `upstreams`](/docs/it/claude-apps-gateway-config#upstreams) ha dettagli di configurazione per piattaforma.
</Note>

<h3 id="cloud-run">
  Cloud Run
</h3>

Configura il servizio come segue:

* Lascia `listen.port` al suo predefinito di `8080`, che corrisponde al `PORT` predefinito di Cloud Run, o imposta `port: ${PORT}`
* Imposta `public_url` all'origine esternamente raggiungibile. Per la produzione questo è normalmente il nome host di un load balancer interno, perché `/login` [rifiuta gli indirizzi pubblici](/docs/it/claude-apps-gateway#prerequisites) e l'URL `*.run.app` si risolve in uno, quindi l'URL di Cloud Run da solo funziona solo per un test di fumo `curl` o browser. L'eccezione è una rete dove `*.run.app` si risolve privatamente tramite Private Service Connect e una zona privata di Cloud DNS; in quella topologia l'URL di Cloud Run è un `public_url` valido. L'[esempio elaborato di Google Cloud](/docs/it/claude-apps-gateway-on-gcp#deploy-the-gateway) copre entrambi.
* Monta la configurazione come volume segreto
* Imposta `min-instances: 1` per evitare il cold start della scoperta OIDC alla prima richiesta

<Note>
  Per un esempio elaborato completo su Google Cloud, che copre Cloud Run o GKE, Cloud SQL e Secret Manager, consulta [Distribuisci su Google Cloud](/docs/it/claude-apps-gateway-on-gcp).
</Note>

<h3 id="push-the-gateway-url-to-developer-machines">
  Invia l'URL del gateway alle macchine degli sviluppatori
</h3>

Una volta che il gateway è in servizio, invia `forceLoginMethod` e `forceLoginGatewayUrl` a ogni macchina dello sviluppatore tramite impostazioni gestite, tramite MDM o scrivendo direttamente il `managed-settings.json` per OS. Senza questo, `/login` mostra il selettore di account standard senza opzione gateway. Consulta [Impostazioni gestite lato client](/docs/it/claude-apps-gateway-config#client-side-managed-settings) per i percorsi dei file.

<h2 id="operations">
  Operazioni
</h2>

Una volta che il gateway sta servendo il traffico, l'operazione quotidiana consiste nel leggere i suoi log, sondare la sua salute e ruotare i suoi segreti secondo il tuo programma. Le sottosezioni coprono ciascuno, più quello che Postgres contiene e come gli aggiornamenti e i rollback si comportano.

<h3 id="logs">
  Log
</h3>

Il gateway scrive due flussi su stderr, entrambi JSON-friendly:

* **Eventi di audit**: JSON a riga singola per evento rilevante per la sicurezza. Invia stderr al tuo aggregatore di log. Gli eventi emessi includono `config.load`, `session.mint`, `session.refresh`, `device.authorize`, `device.verify`, `auth.denied`, `access.denied`, `inference`, `managed.serve`, `spend.blocked`, e `admin.denied`. I campi variano per evento:
  * Gli eventi di mint e refresh riusciti portano `sub`, `email`, `client_ip`, e il risultato
  * Gli eventi di negazione portano il motivo, il percorso e l'IP client, poiché nessuna identità esiste al momento della negazione
  * `inference` registra quale upstream ha servito la richiesta e lo stato della risposta
  * `admin.denied` registra un tentativo di autenticazione dell'API admin rifiutato con il motivo (`invalid_key` o `no_credentials`), IP client, metodo e percorso, senza il materiale della chiave presentato
* **Log operazionali**: righe leggibili con prefisso `[gateway]` per avvio, avvertimenti e errori upstream. La variabile di ambiente `CLAUDE_GATEWAY_LOG_LEVEL` controlla la verbosità e accetta `info`, `warn`, o `error`, con `info` come predefinito. Non influisce sugli eventi di audit, che sono sempre emessi.

<h3 id="health">
  Salute
</h3>

Il gateway serve `GET /healthz` come sonda di liveness e `GET /readyz` come sonda di readiness; `/readyz` verifica che lo store sia raggiungibile. Entrambi sono esenti da `access_control.allow_cidrs`, quindi le sonde continuano a funzionare su un listener bloccato.

Il documento di scoperta OAuth in `/.well-known/oauth-authorization-server` restituisce anche `200` solo dopo il caricamento della configurazione, la scoperta OIDC, la costruzione del client upstream e la migrazione di Postgres, quindi funge anche da controllo di avvio end-to-end.

Un gateway in esecuzione serve anche una descrizione dei percorsi e delle forme di richiesta che accetta in `<public_url>/protocol`, abbinata alla versione che stai eseguendo. I contenuti non sono stabili tra le versioni.

<h3 id="outage-behavior">
  Comportamento in caso di interruzione
</h3>

Se Postgres si arresta, il gateway stesso continua a servire gli sviluppatori che hanno effettuato l'accesso e i nuovi accessi falliscono. Se gli sviluppatori effettivamente continuano a lavorare dipende da come il tuo orchestrator gestisce la readiness:

* **Sessioni esistenti**: i bearer token si convalidano localmente con il segreto JWT, gli aggiornamenti della sessione non toccano lo store, e il processo del gateway può comunque servire l'inferenza
* **Nuovi accessi**: falliscono fino al recupero di Postgres, perché il flusso del dispositivo e i suoi contatori di limite di velocità vivono in Postgres
* **[Applicazione del limite di spesa](/docs/it/claude-apps-gateway-spend-limits#postgres-availability)**: fallisce aperto per impostazione predefinita durante l'interruzione, quindi l'inferenza continua a fluire; capovolgilo per fallire chiuso se preferisci bloccare piuttosto che eseguire senza misurazione
* **Readiness**: `/readyz` segnala non-ready durante l'interruzione, quindi gli orchestrator che controllano il traffico sulla readiness rimuovono ogni replica dalla rotazione contemporaneamente. In quella topologia tutto il traffico, inclusa l'inferenza che il gateway potrebbe comunque servire, fallisce al load balancer fino al recupero di Postgres. La sonda di liveness su `/healthz` continua a passare, quindi le repliche non vengono riavviate. Punta la sonda di readiness a `/healthz` invece se preferisci che gli sviluppatori che hanno effettuato l'accesso continuino a lavorare attraverso un'interruzione dello store; il costo è che i nuovi accessi falliscono contro una replica che ancora segnala ready.

Se il tuo IdP si arresta, le sessioni esistenti funzionano fino a `ttl_hours`, e i nuovi accessi e aggiornamenti falliscono. Imposta un `ttl_hours` più lungo se il tuo IdP ha frequenti finestre di manutenzione.

<h3 id="jwt-secret-rotation">
  Rotazione del segreto JWT
</h3>

Ruota il segreto di firma in tre passaggi in modo che le sessioni esistenti rimangono valide:

1. Genera un nuovo segreto. Anteponi l'array `session.jwt_secret`.
2. Esegui il rollout della distribuzione. I nuovi token firmano con il nuovo segreto; i vecchi token si convalidano comunque.
3. Dopo `ttl_hours` più un margine, rimuovi il vecchio segreto ed esegui di nuovo il rollout.

La rotazione è anche l'unico modo per forzare le sessioni prima che scadano: i bearer token si convalidano localmente rispetto al segreto JWT, quindi non c'è revoca per sessione. Sostituire il segreto completamente, senza mantenere il vecchio nell'array, invalida ogni sessione in sospeso contemporaneamente. Per l'offboarding individuale, esegui il deprovisioning dell'utente nel tuo IdP; la loro sessione termina entro `ttl_hours`.

<h3 id="postgres">
  Postgres
</h3>

Il gateway contiene cinque tabelle, tutte create dalle sue migrazioni al momento dell'avvio:

| Tabella            | Contenuti                                                                                   | Conservazione                                                        |
| ------------------ | ------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `kv`               | Concessioni di dispositivi (TTL di 10 minuti) e contatori di limite di velocità             | TTL per riga                                                         |
| `spend`            | Contatori di spesa da inizio periodo per principale, in centesimi                           | `admin.spend_retention_months`, predefinito 13                       |
| `spend_limits`     | Limiti di spesa configurati                                                                 | Fino all'eliminazione tramite l'API                                  |
| `admin_audit`      | Traccia di mutazione dell'API admin                                                         | `admin.audit_retention_days`, predefinito 365                        |
| `principal_emails` | Email dell'ultimo accesso di ogni principale, nome visualizzato e gruppi IdP. Contiene PII. | `admin.identity_retention_days` dall'ultima attività, predefinito 90 |

Un ciclo di 30 secondi scade le righe `kv` oltre il loro TTL, e una pulizia oraria applica le finestre di conservazione sulle tabelle di spesa, quindi nulla cresce senza limiti. Senza [limiti di spesa](/docs/it/claude-apps-gateway-spend-limits) configurati, solo `kv` viene scritto. Se la tua policy di sicurezza proibisce DDL dal ruolo dell'applicazione, pre-crea queste tabelle e `_migrations` con un ruolo admin e concedi al ruolo dell'app `SELECT, INSERT, UPDATE, DELETE` su ciascuno.

Con limiti di spesa in uso, un database perso significa perdita di tracciamento della spesa e limiti, non solo re-accessi degli sviluppatori, quindi esegui backup regolari. Per cancellare immediatamente uno sviluppatore partito piuttosto che aspettare la conservazione, esegui `DELETE FROM principal_emails WHERE principal = '<sub>'` direttamente; questo rimuove l'unica tabella che contiene la loro email, nome e gruppi. Le righe `spend` e `admin_audit` fanno riferimento solo allo pseudonimo OIDC `sub`.

<h3 id="upgrades">
  Aggiornamenti
</h3>

Le repliche sono senza stato, quindi un riavvio rolling è sicuro in qualsiasi momento. Il gateway esegue migrazioni dello schema all'avvio, il che significa che distribuire il nuovo binario auto-migra il database. Se il ruolo del database non può eseguire DDL, pre-crea lo schema, inclusa la tabella `_migrations` seminata alla versione corrente; altrimenti l'avvio fallisce tentando `CREATE TABLE`.

Le migrazioni sono append-only, quindi il rollback a un binario precedente che conosce meno migrazioni è sicuro; ignora le righe extra. Il rollback ri-convalida anche il YAML rispetto allo schema del binario più vecchio, quindi una configurazione che ha adottato una chiave introdotta dalla versione più recente fallisce l'avvio su quella più vecchia. Rimuovi la nuova chiave prima di eseguire il rollback.

Poiché fissi la versione del gateway nella tua immagine, le correzioni nelle nuove versioni di Claude Code, incluse le correzioni di sicurezza, raggiungono la tua distribuzione solo quando aggiorni il pin e ridistribuisci. Includi il gateway nello stesso ciclo di patching che usi per altri servizi che contengono credenziali di produzione.

<h2 id="security">
  Sicurezza
</h2>

Questa sezione risponde alle domande che una revisione di sicurezza pone: quali dati fluiscono attraverso il gateway e dove vanno, quali attacchi il design difende, e quali risposte appartengono a un questionario di conformità.

<h3 id="data-flow">
  Flusso di dati
</h3>

| Dati                                                                                            | Percorso                                                           | Inviato ad Anthropic dal gateway                  |
| ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------- |
| Inferenza (prompt, completamenti)                                                               | CLI → gateway → il tuo upstream                                    | Solo se l'API Anthropic è un upstream configurato |
| Telemetria (metriche OTLP, più [log e tracce opt-in](/docs/it/claude-apps-gateway-config#telemetry)) | CLI → gateway → il tuo collettore                                  | Mai                                               |
| Identità (email, gruppi, sub)                                                                   | IdP → gateway → JWT → CLI; il CLI lo marca sulle esportazioni OTLP | Mai                                               |
| Impostazioni gestite                                                                            | Il tuo gateway YAML → CLI                                          | Mai                                               |
| Log di audit                                                                                    | Gateway stderr → il tuo aggregatore                                | Mai                                               |

<h3 id="threat-model-summary">
  Riepilogo del modello di minaccia
</h3>

Il gateway si trova all'interno del perimetro della tua rete, ma i singoli laptop degli sviluppatori non sono considerati affidabili. Il design tiene conto di questo in tre modi:

* Gli sviluppatori detengono JWT di breve durata invece di chiavi upstream grezze. La gamba CLI-to-gateway utilizza la concessione del dispositivo RFC 8628, e lo scambio di autorizzazione del gateway con l'IdP esegue PKCE nella configurazione predefinita, quindi un codice di autorizzazione IdP intercettato è inutile.
* La pagina di verifica del dispositivo applica POST della stessa origine e un limite di velocità per IP per RFC 8628 §5.1. Consulta [Resistenza al brute-force del codice utente](#user-code-brute-force-resistance).
* Le richieste in uscita passano attraverso una guardia SSRF (Server-Side Request Forgery) che risolve DNS, blocca gli indirizzi link-local e cloud-metadata più loopback per impostazione predefinita, e fissa la connessione all'IP risolto, quindi gli URL influenzati dall'operatore come l'IdP e le destinazioni OTLP non possono essere reindirizzati agli endpoint dei metadati cloud. Gli intervalli privati RFC 1918 sono deliberatamente consentiti, perché gli IdP e i collettori OTLP comunemente vivono su IP privati. Per lo sviluppo locale rispetto a un IdP loopback o collettore, imposta `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` nell'ambiente del gateway; lascialo non impostato in produzione.

Se aggiungi i tuoi controlli di uscita, il gateway deve raggiungere il server dei metadati ogni volta che utilizza credenziali di metadati dell'istanza come workload identity.

Due minacce sono fuori ambito perché sono la tua infrastruttura da proteggere:

* **Un host gateway compromesso**: l'host sia contiene la credenziale upstream che distribuisce [impostazioni gestite](/docs/it/claude-apps-gateway-config#managed) a ogni sviluppatore connesso, quindi il controllo sulla configurazione del gateway è paragonabile al controllo sul tuo MDM. La finestra di dialogo di approvazione una tantum del CLI per le impostazioni in grado di shell limita i cambiamenti silenziosi ma non sostituisce la sicurezza dell'host.
* **Un provider OIDC malintenzionato**: il provider firma gli id\_token che il gateway si fida, quindi può asserire qualsiasi identità. Il vetting e la protezione del tuo IdP è tua responsabilità.

<h3 id="user-code-brute-force-resistance">
  Resistenza al brute-force del codice utente
</h3>

Il `user_code` che uno sviluppatore digita nella pagina di verifica `/device` è di 8 caratteri tratti da un alfabeto di 20 caratteri, che produce 20⁸ o circa 2,56×10¹⁰ combinazioni, e scade dopo 10 minuti.

Il gateway applica limiti di velocità per IP sugli endpoint di concessione del dispositivo, configurabili tramite [`rate_limits`](/docs/it/claude-apps-gateway-config#http-tuning). Aumenta i limiti se molti sviluppatori accedono da un singolo indirizzo NAT aziendale condiviso. I limiti si applicano solo al flusso di accesso, non all'inferenza.

<h3 id="compliance-posture">
  Postura di conformità
</h3>

* **Residenza dei dati**: il piano dati del gateway stesso non invia nulla ad Anthropic a meno che l'API Anthropic non sia un upstream configurato; quando lo è, il tuo accordo di gestione dei dati esistente si applica al percorso di inferenza. Telemetria, audit, identità e impostazioni vanno solo alle destinazioni che configuri.
* **Traffico del processo host**: il processo host è il CLI di Claude Code, che può inviare analitiche di avvio e controlli di aggiornamento ad Anthropic. Per distribuzioni con uscita ristretta, imposta `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` nell'ambiente del container del gateway.
* **Analitiche client**: il CLI disabilita la sua stessa analittica di utilizzo mentre è connesso a un gateway, e la segnalazione degli errori è disattivata per impostazione predefinita su superfici API di terze parti.
* **Macchine client**: i CLI degli sviluppatori inviano comunque controlli del nome host WebFetch e controlli della versione ad Anthropic a meno che `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` e `skipWebFetchPreflight: true` non siano impostati. Consulta [utilizzo dei dati](/docs/it/data-usage).
* **Valutazioni del sondaggio**: la credenziale del gateway disabilita il sink di valutazione legato ad Anthropic, quindi le valutazioni non vengono inviate ad Anthropic.
* **Condivisione della trascrizione**: scegliere Sì su un prompt di condivisione della trascrizione di un sondaggio scrive un file locale sotto `~/.claude/feedback-bundles/` invece di caricare su Anthropic.
* **Aggiornamenti client**: i controlli di aggiornamento sono separati dal traffico del gateway. Fissa le versioni attraverso la tua distribuzione e imposta `DISABLE_UPDATES` se i laptop non devono recuperare le versioni. `DISABLE_AUTOUPDATER` interrompe solo gli aggiornamenti in background mentre `claude update` funziona ancora.
* **TLS**: servi `public_url` su HTTPS in produzione, sia dal listener proprio del gateway tramite `listen.tls` che da un ingress che termina TLS davanti alle repliche HTTP semplici con `listen.public_url` impostato. Il gateway non rifiuta HTTP semplice. L'IdP deve servire HTTPS in produzione, e Postgres supporta `?sslmode=require`. Imposta `Strict-Transport-Security` al tuo ingress.
* **Divulgazione di vulnerabilità**: segui [Segnalazione di problemi di sicurezza](/docs/it/security#reporting-security-issues)

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Per domande e feedback, usa [Supporto di Claude Code](https://support.claude.com/en/collections/14445694-claude-code), o apri un problema nel [repository GitHub di Claude Code](https://github.com/anthropics/claude-code/issues). Quando segnali un problema, includi:

* **Problema del gateway**: lo stderr del gateway per la finestra rilevante, il tuo `gateway.yaml` con i segreti redatti, la versione del gateway, mostrata nella pagina di destinazione in `/` e nell'intestazione della risposta `x-cc-gateway-version` su `/managed/settings`, e cosa è cambiato di recente
* **Problema di accesso**: lo sviluppatore esegue `claude --debug-file ./claude-debug.txt`, riproduce, e invia quel file più il log di audit del gateway per la stessa finestra
* **Problema di inferenza**: il modello richiesto, gli upstream configurati, e il log di audit del gateway per la richiesta, che registra quale upstream l'ha servita e lo stato della risposta

Lo stderr del gateway include il flusso di eventi di audit, il log di audit registra le identità degli sviluppatori e il file di debug registra l'output di hook e MCP server dalla macchina dello sviluppatore. Rivedere e oscurare questi elementi prima di pubblicare in un problema pubblico.

| Sintomo                                                                                                                                                                         | Causa                                                                                                                                                                                                                                                                                                                                                           | Correzione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Lo `/login` di uno sviluppatore mostra il selettore di account standard invece della schermata **Cloud gateway**                                                                | `forceLoginMethod` o `forceLoginGatewayUrl` non è impostato nelle impostazioni gestite su quella macchina                                                                                                                                                                                                                                                       | Distribuisci il [file di impostazioni gestite](/docs/it/claude-apps-gateway#set-the-gateway-url) al dispositivo; `/login` legge l'URL del gateway da lì                                                                                                                                                                                                                                                                                                                                                                                                  |
| L'avvio mostra `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.`                                            | La build di Claude Code installata precede il supporto del gateway                                                                                                                                                                                                                                                                                              | Fai aggiornare lo sviluppatore a Claude Code a una versione che include il supporto del gateway cloud                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>`                                      | Il nome host del gateway si risolve in almeno un indirizzo IP pubblico. Claude Code controlla ogni indirizzo risolto e richiede che ogni uno sia privato. Una causa comune è un nome dual-stack dove una famiglia si risolve in un indirizzo pubblico, inclusi i load balancer dual-stack interni AWS, che restituiscono indirizzi AAAA di intervallo pubblico. | Fai risolvere il nome del gateway solo in indirizzi privati sulle macchine degli sviluppatori. Per un nome dual-stack, elimina il record di intervallo pubblico o servi un nome DNS solo interno separato. Consulta il [prerequisito di rete privata](/docs/it/claude-apps-gateway#prerequisites).                                                                                                                                                                                                                                                       |
| CLI `/login`: `Gateway login requires a direct connection and does not support connecting through an HTTP proxy`                                                                | Un `HTTPS_PROXY` o `HTTP_PROXY` si applica all'host del gateway e il nome host del proxy si risolve in un indirizzo pubblico. Un proxy il cui host si risolve solo in indirizzi privati è consentito e non attiva questo errore                                                                                                                                 | Aggiungi l'host del gateway a `NO_PROXY` sulla macchina dello sviluppatore in modo che la connessione sia diretta, o usa un proxy il cui nome host si risolve in indirizzi privati                                                                                                                                                                                                                                                                                                                                                                  |
| CLI `/login`: `Could not resolve gateway host <host>`                                                                                                                           | La macchina non può risolvere il nome DNS interno del gateway, tipicamente perché non è sulla rete aziendale                                                                                                                                                                                                                                                    | Fai connettere lo sviluppatore alla tua rete o VPN, quindi riprova `/login`                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| L'avvio esce con un errore di convalida della configurazione che nomina `store.postgres_url`                                                                                    | Nessun Postgres configurato; il gateway richiede Postgres                                                                                                                                                                                                                                                                                                       | Imposta `store.postgres_url`. Per lo sviluppo locale, usa un container usa e getta: `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.                                                                                                                                                                                                                                                                                                                                                                                     |
| L'avvio esce: `requires the native binary`                                                                                                                                      | Esecuzione sotto Node invece del binario nativo                                                                                                                                                                                                                                                                                                                 | Installa Claude Code con uno dei [metodi di installazione standalone](/docs/it/setup)                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| L'avvio esce con un errore di scoperta OIDC dopo `config.load`                                                                                                                  | `oidc.issuer` non raggiungibile, o catena TLS non affidabile                                                                                                                                                                                                                                                                                                    | Controlla che l'emittente sia raggiungibile dal pod e serva `/.well-known/openid-configuration`. Imposta `ca_cert_pem` per PKI privata.                                                                                                                                                                                                                                                                                                                                                                                                             |
| L'avvio esce con un errore di permesso di Postgres                                                                                                                              | Il ruolo dell'app manca `CREATE TABLE`                                                                                                                                                                                                                                                                                                                          | Pre-crea lo schema con un ruolo admin e concedi DML al ruolo dell'app, o concedi DDL temporaneamente per gli avvii che applicano nuove migrazioni                                                                                                                                                                                                                                                                                                                                                                                                   |
| `/oauth/callback` mostra "Sign-in could not be completed"                                                                                                                       | Dominio email rifiutato, convalida id\_token fallita, o `email_verified` è esplicitamente `false`, che il gateway rifiuta sempre senza override                                                                                                                                                                                                                 | Controlla `allowed_email_domains` e che l'IdP restituisca un claim `email` verificato. Per `email_verified: false`, correggi la verifica lato IdP. Se il tuo IdP emette email sotto un nome di claim diverso, imposta `oidc.email_claim`.                                                                                                                                                                                                                                                                                                           |
| Log: `token exchange failed: id_token missing email claim`                                                                                                                      | L'IdP non include `email` nell'id\_token per impostazione predefinita. Questo rifiuto si attiva solo quando `allowed_email_domains` è impostato; senza di esso, un'email mancante conia una sessione senza email                                                                                                                                                | Configura l'IdP per emettere `email` nell'id\_token. Okta: aggiungi `email` ai claim id\_token di un server di autorizzazione personalizzato. Entra: aggiungi `email` come claim facoltativo sulla registrazione dell'app. PingFederate: abilita una Policy OpenID Connect che emette `email`. Se l'IdP serve `email` dall'endpoint userinfo ma non lo includerà nell'id\_token, come il server di autorizzazione dell'organizzazione Okta, imposta `oidc.userinfo_fallback: true`.                                                                 |
| Ogni richiesta Bedrock restituisce 502; il log mostra `Could not load credentials from any providers`                                                                           | Su EC2, il limite di hop predefinito di IMDSv2 di 1 blocca la richiesta di metadati dell'istanza dall'interno del container. L'avvio e `/readyz` passano comunque perché l'AWS SDK risolve le credenziali dell'istanza alla prima richiesta, non alla costruzione del client                                                                                    | Aumenta il limite di hop con `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`, o impostalo nel modello di lancio. Il cambiamento si applica a ogni container sull'istanza. Preferisci i ruoli delle attività ECS dove disponibili, che leggono le credenziali dall'endpoint delle credenziali del container ECS e evitano completamente il cambiamento, o applica il cambiamento su un'istanza del gateway dedicata per limitare l'esposizione.                                                        |
| Errore IdP: unknown or unsupported scope                                                                                                                                        | L'IdP rifiuta gli scope che non riconosce                                                                                                                                                                                                                                                                                                                       | Imposta `oidc.scopes` esattamente all'elenco che il tuo IdP accetta; deve includere `openid`. Il predefinito è `openid profile email offline_access`.                                                                                                                                                                                                                                                                                                                                                                                               |
| Le sessioni non si rinnovano silenziosamente dopo aver impostato `oidc.scopes`                                                                                                  | `offline_access` è stato eliminato dall'override                                                                                                                                                                                                                                                                                                                | Aggiungi `offline_access` di nuovo se il tuo IdP lo supporta. Senza un token di aggiornamento, gli sviluppatori rieseguono l'accesso al browser ogni `session.ttl_hours`.                                                                                                                                                                                                                                                                                                                                                                           |
| Il browser mostra "This request came from another site and was blocked"                                                                                                         | POST del modulo cross-site, bloccato come protezione CSRF. Previsto per pagine incorporate o proxy                                                                                                                                                                                                                                                              | Apri il collegamento di verifica direttamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Chrome blocca il pulsante Approve con "Refused to send form data … violates … Content Security Policy directive: form-action", ma la stessa pagina funziona in Safari o Firefox | Chrome applica `form-action` all'intera catena di reindirizzamento. Il tuo IdP reindirizza ulteriormente a un secondo host che non è nella whitelist.                                                                                                                                                                                                           | Aggiungi ogni origine aggiuntiva nella catena di reindirizzamento a `oidc.form_action_origins`. Apri Chrome DevTools → Console sulla pagina Approve per vedere quale origine è stata bloccata.                                                                                                                                                                                                                                                                                                                                                      |
| L'accesso si completa all'IdP ma il callback fallisce, con un errore CSP in Chrome o "this sign-in link has expired" in Safari                                                  | L'IdP ha restituito il codice tramite `response_mode=form_post`, che lo auto-invia cross-origin tramite POST a `/oauth/callback`. Chrome blocca quello sotto una CSP ristretta; Safari consente l'invio ma il callback legge solo la stringa di query.                                                                                                          | Assicurati che il tuo IdP onori `response_mode=query`, che il gateway richiede esplicitamente in modo che il callback sia un reindirizzamento semplice                                                                                                                                                                                                                                                                                                                                                                                              |
| L'accesso funziona localmente ma fallisce dietro un ALB                                                                                                                         | `public_url` non impostato, quindi l'IdP ottiene l'origine interna `http://` come `redirect_uri`                                                                                                                                                                                                                                                                | Imposta `listen.public_url` all'origine esterna `https://`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Lo sviluppatore vede il prompt di fiducia ripetutamente                                                                                                                         | Il certificato TLS ruota per replica o per richiesta                                                                                                                                                                                                                                                                                                            | Usa un certificato stabile all'ingress, o termina TLS una volta ed esegui le repliche su HTTP semplice internamente                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| CLI `/login`: "Could not verify the gateway's TLS certificate" o `SELF_SIGNED_CERT_IN_CHAIN`                                                                                    | La catena TLS del gateway è firmata da una CA privata non nell'archivio di fiducia dell'host CLI                                                                                                                                                                                                                                                                | Claude Code legge l'archivio di fiducia del sistema operativo per impostazione predefinita sul binario nativo e su Node 22.15 o successivo; [`CLAUDE_CODE_CERT_STORE`](/docs/it/network-config#ca-certificate-store) controlla questo comportamento. Se la CA è installata nell'archivio di fiducia del sistema operativo, assicurati che gli sviluppatori siano su un runtime corrente. Altrimenti imposta `NODE_EXTRA_CA_CERTS` al certificato CA PEM prima di lanciare. Il prompt dell'impronta digitale della prima connessione si applica comunque. |

<h2 id="related">
  Correlati
</h2>

* [Panoramica del gateway delle app Claude](/docs/it/claude-apps-gateway): quickstart e connessione dello sviluppatore
* [Riferimento di configurazione](/docs/it/claude-apps-gateway-config): ogni opzione di `gateway.yaml`
