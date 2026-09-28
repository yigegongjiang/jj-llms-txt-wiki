> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurazione del gateway delle app Claude

> Riferimento per ogni opzione di gateway.yaml: listener e TLS, OIDC, sessione, archivio Postgres, upstream Amazon Bedrock, Claude Platform su AWS, Agent Platform di Google Cloud e Microsoft Foundry, routing dei modelli, criteri gestiti e telemetria.

Una distribuzione del gateway delle app Claude è configurata da un file YAML, convenzionalmente `gateway.yaml`. Il file definisce tutto ciò che il gateway fa: dove ascolta, come gli sviluppatori accedono, dove va l'inferenza e quali criteri e telemetria si applicano. Questa pagina è il riferimento per ogni opzione in quel file.

Per scrivere il vostro primo file, iniziate dalla [guida rapida](/docs/it/claude-apps-gateway#quickstart), che crea una configurazione minima funzionante e la esegue. Una volta che avete una configurazione con cui siete soddisfatti, la [guida alla distribuzione](/docs/it/claude-apps-gateway-deploy) copre la containerizzazione e l'hosting su Kubernetes, Cloud Run o la vostra piattaforma.

Il gateway legge il file una volta, all'avvio, con `claude gateway --config /path/to/gateway.yaml`. Ogni opzione è convalidata rispetto a uno schema all'avvio, quindi una configurazione malformata non riesce all'inizio con un errore a livello di campo piuttosto che al primo utilizzo.

L'[esempio completo](#complete-example) alla fine di questa pagina esercita ogni sezione.

<h2 id="file-structure">
  Struttura del file
</h2>

Cinque sezioni sono [obbligatorie](#required-sections). Ogni altra sezione è [facoltativa](#optional-sections) e una sezione omessa assume i suoi valori predefiniti. Le chiavi sconosciute causano un errore all'avvio, quindi un errore di battitura emerge come errore denominato piuttosto che come impostazione ignorata silenziosamente.

**Sezioni obbligatorie:**

* [`listen`](#listen): indirizzo di binding, URL pubblico, terminazione TLS
* [`oidc`](#oidc): il vostro provider di identità (IdP), inclusi emittente, client, mappatura dei claim e chi può accedere
* [`session`](#session): i token bearer che il gateway conia, con segreto e durata
* [`store`](#store): PostgreSQL, per le concessioni dei dispositivi e i contatori dei limiti di velocità
* [`upstreams`](#upstreams): dove va l'inferenza, sia Anthropic, Amazon Bedrock, Claude Platform su AWS, Agent Platform di Google Cloud o Microsoft Foundry

**Sezioni facoltative:**

* [`admin`](#admin): autenticazione dell'API Admin e conservazione dei limiti di spesa
* [`enforcement`](#enforcement): comportamento fail-open o fail-closed dei limiti di spesa
* [`pricing`](#pricing): tariffe contrattuali e un moltiplicatore di sconto per il misuratore di spesa e per le cifre di costo che gli sviluppatori vedono
* [`models`](#models) e `auto_include_builtin_models`: elenco di modelli curato dall'amministratore e ID per upstream
* [`managed`](#managed): politiche di impostazioni gestite per gruppo IdP
* [`telemetry`](#telemetry): inoltro OTLP al vostro stack di osservabilità
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): IP allow/deny, limiti di dimensione delle richieste, time-to-first-byte upstream e limiti di accesso per IP

<h2 id="secret-expansion">
  Espansione dei segreti
</h2>

Non scrivete segreti come `client_secret`, `jwt_secret` o `postgres_url` direttamente in `gateway.yaml`. Fate riferimento ad essi con uno dei moduli sottostanti e il gateway risolve il valore all'avvio da una variabile di ambiente o da un file:

| Modulo          | Si risolve in                                                                                                                                                                                                                                                                                                          | Usare per                                                                        |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `${VAR}`        | La variabile di ambiente `VAR`. L'avvio fallisce se non definita.                                                                                                                                                                                                                                                      | Variabili di ambiente del contenitore, AWS Secrets Manager tramite iniezione env |
| `${file:/path}` | Contenuti del file al percorso assoluto specificato, ritagliati. Il riferimento deve essere l'intero valore del campo: a differenza di `${VAR}`, non viene espanso all'interno di una stringa più lunga, quindi per una password del database impostare `store.password` piuttosto che incorporarla in `postgres_url`. | Montaggi di volumi Kubernetes Secret, Vault Agent, SOPS                          |

<h2 id="required-sections">
  Sezioni obbligatorie
</h2>

<h3 id="listen">
  `listen`
</h3>

Il blocco `listen` controlla dove il gateway serve: l'indirizzo di binding e la porta, l'origine visibile esternamente e la terminazione TLS facoltativa.

| Campo                  | Obbligatorio      | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---------------------- | ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | No                | Indirizzo di binding. Predefinito `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `port`                 | No                | Porta di binding. Predefinito `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `public_url`           | Se non è loopback | L'origine `https://` visibile esternamente, utilizzata per costruire il `redirect_uri` dell'IdP e i metadati di scoperta. Obbligatorio ogni volta che `host` non è un indirizzo loopback, sia che TLS termini a un proxy come un ALB, Ingress o Cloud Run o al gateway stesso tramite `tls`, perché il gateway non deriva mai la sua stessa origine dagli header `X-Forwarded-*`; sono falsificabili dal client. L'avvio fallisce senza di esso. `trusted_proxies` di seguito governa solo la risoluzione dell'IP del client. Obbligatorio anche per abilitare la [telemetria](#telemetry), perché il gateway costruisce l'endpoint OTLP che spinge ai client da questo URL. |
| `tls.cert` / `tls.key` | No                | Percorsi PEM se il gateway termina TLS stesso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `trusted_proxies`      | No                | CIDR o IP dei load balancer davanti al gateway. Quando impostato, il gateway si fida di `X-Forwarded-For` solo da questi peer e registra l'IP client reale per il rate limiting per IP e l'audit. Equivalente a nginx `set_real_ip_from`. Le voci `X-Forwarded-For` scritte come `ipv4:port` o `[ipv6]:port`, come fanno alcuni load balancer, vengono lette con la porta eliminata. Un indirizzo IPv6 con una porta aggiunta e senza parentesi può essere letto come un indirizzo diverso o non letto affatto, quindi disattivate l'opzione della porta su qualsiasi proxy che scrive quella forma.                                                                         |

<h3 id="oidc">
  `oidc`
</h3>

Il blocco `oidc` connette il gateway al vostro provider di identità e decide chi può accedere. Nomina l'emittente e il client OAuth, mappa i claim che portano email e gruppi e limita l'accesso per dominio email o gruppo.

OpenID Connect (OIDC) è il protocollo SSO che il gateway utilizza con il vostro provider di identità; vedere [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup) per ciò che registrare sul lato IdP.

| Campo                           | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Sì           | Base di scoperta OIDC. Deve servire la scoperta su `/.well-known/openid-configuration`. Usate HTTPS in produzione; il gateway accetta un emittente `http://`. Un emittente loopback come `http://localhost:8081` è rifiutato dalla [guardia SSRF](/docs/it/claude-apps-gateway-deploy#threat-model-summary) a meno che `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` non sia impostato nell'ambiente del gateway.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `client_id` / `client_secret`   | Sì           | Dalla vostra registrazione del client OAuth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `allowed_email_domains`         | No           | Rifiutate i token id i cui claim `email` non sono in uno di questi domini, case-insensitive. Difesa in profondità contro la misconfiguration dell'IdP multi-tenant. Indipendentemente da questa impostazione, un id\_token il cui claim `email_verified` è esplicitamente `false` è sempre rifiutato.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `allowed_groups`                | No           | Limitate l'accesso ai membri di questi gruppi IdP, abbinati rispetto a `groups_claim`. Un utente in un dominio email consentito ma in nessuno di questi gruppi è rifiutato. Richiede che l'IdP emetta il claim dei gruppi. L'abbinamento è un confronto di stringhe esatto e case-sensitive rispetto ai valori in quel claim, e il gateway non espande i gruppi annidati: per ammettere i membri di un sottogruppo, elencate il sottogruppo qui o configurate l'IdP per emettere l'appartenenza appiattita.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `groups_claim`                  | No           | Quale claim id\_token porta l'appartenenza al gruppo. Predefinito `groups`. Microsoft Entra emette i ruoli dell'app sotto `roles`. Accetta una chiave flat o un JSON Pointer RFC 6901 come `/resource_access/gateway/roles` per i claim annidati.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `google_groups`                 | No           | Cercate i gruppi dell'utente che ha effettuato l'accesso tramite l'API Google Workspace Admin SDK Directory, perché il token id di Google non porta alcun claim di gruppi. Impostate `service_account_json_path` su un file di chiave dell'account di servizio con delega a livello di dominio sull'ambito `https://www.googleapis.com/auth/admin.directory.group.readonly` e `admin_email` su un amministratore di Workspace che l'account di servizio rappresenta; l'API Directory richiede un soggetto amministratore reale. Gli indirizzi email dei gruppi di ogni utente diventano il loro claim di gruppi, quindi `allowed_groups` e `managed.policies.match.groups` corrispondono agli indirizzi email dei gruppi.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `email_claim`                   | No           | Quale claim id\_token porta l'email dell'utente. Predefinito `email`. Alcuni IdP, come ADFS ed Entra B2C, emettono `upn` o `preferred_username` invece. Accetta una chiave flat, un JSON Pointer o un elenco di chiavi di fallback dove viene utilizzata la prima chiave presente.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `scopes`                        | No           | Override completo degli ambiti OIDC che il gateway richiede. Predefinito `[openid, profile, email, offline_access]`. Impostate quando il vostro IdP rifiuta gli ambiti che non riconosce o richiede un ambito personalizzato per emettere gruppi o email. Deve includere `openid`. Eliminare `offline_access` disabilita i token di aggiornamento, quindi gli sviluppatori rieseguono l'accesso al browser ogni `session.ttl_hours`. Vedere [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup) per ricette di ambito per IdP come il flusso di token di aggiornamento di Google.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `scope_on_refresh`              | No           | Inviate anche `scope`, con lo stesso elenco della richiesta di accesso, quando il gateway scambia un token di aggiornamento. Predefinito `false`: la richiesta di aggiornamento omette `scope`. La maggior parte degli IdP restituisce un id\_token ad ogni aggiornamento e non ha bisogno di questo. Impostate `true` quando il vostro IdP restituisce un id\_token al momento dell'aggiornamento solo se richiesto di nuovo `openid`, che Okta documenta per il suo grant di aggiornamento. Senza un id\_token, ogni aggiornamento dipende dall'endpoint userinfo dell'IdP che accetta il token di accesso aggiornato. Se controllate l'accesso o abbinate le politiche sui gruppi e l'id\_token dell'IdP al momento dell'aggiornamento li omette, impostate anche `userinfo_fallback: true` in modo che il gateway li riempia dall'endpoint userinfo. Un IdP che ha concesso meno ambiti di quelli richiesti può rifiutare l'aggiornamento con `invalid_scope`, incluso per le sessioni esistenti se aggiungete voci a `scopes` mentre questo è attivo. Deselezionate la chiave se gli aggiornamenti iniziano a fallire su `token_endpoint` dopo averla impostata. Richiede Claude Code v2.1.260 o successivo sul server del gateway. |
| `extra_auth_params`             | No           | Parametri di query extra aggiunti alla richiesta di autorizzazione dell'IdP, verbatim. Questo è il meccanismo di override per il comportamento specifico dell'IdP, come `access_type: offline` per i token di aggiornamento di Google, `domain_hint` per alcuni tenant Entra o `acr_values` per i flussi step-up. Non può sovrascrivere i parametri del protocollo gestiti dal gateway: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` e `client_id`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `userinfo_fallback`             | No           | Quando l'id\_token omette email o gruppi, recuperateli da `/userinfo`. Necessario per i token di accesso leggeri di Keycloak, il server org di Okta e i token minimi di ADFS. L'id\_token rimane autorevole; userinfo riempie solo i vuoti. Predefinito `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `use_pkce`                      | No           | Inviate una sfida PKCE (S256) sulla richiesta di autorizzazione. Predefinito `true`. Impostate `false` solo se il vostro IdP rifiuta PKCE per questo client confidenziale.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `clock_skew_seconds`            | No           | Tollerare la deriva dell'orologio quando si convalidano i claim temporali dell'id\_token. Predefinito `0`, che è rigoroso. Aumentate se vedete errori "token scaduto / non ancora valido" subito dopo l'accesso a causa della deriva dell'orologio host/IdP.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `token_endpoint_auth_method`    | No           | Override del metodo di autenticazione dell'endpoint del token. Accetta `client_secret_basic` o `client_secret_post`. Negoziato automaticamente per impostazione predefinita.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `id_token_signed_response_alg`  | No           | Algoritmo di firma id\_token previsto. Predefinito `RS256`. Impostate per gli IdP che firmano con ES256, PS256 o EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `additional_authorized_parties` | No           | Valori `azp` extra da accettare oltre a `client_id`, per i flussi di broker e scambio di token di Keycloak                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `discovery_url`                 | No           | Recuperate il documento di scoperta da questo URL invece di derivarlo da `issuer`, per gli IdP dietro un proxy che riscrive l'host dell'emittente. Il percorso deve contenere `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `use_proxy`                     | No           | Inviate le richieste IdP del gateway stesso attraverso il proxy forward in `HTTPS_PROXY` o `HTTP_PROXY`, onorando `NO_PROXY`. Non impostato o `false`, quelle richieste vanno dirette. Richiede v2.1.227 o successivo; vedere [Richieste IdP attraverso un proxy forward](#idp-requests-through-a-forward-proxy) di seguito.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `form_action_origins`           | No           | Origini aggiuntive per la direttiva `Content-Security-Policy: form-action` della pagina `/device`. Il gateway consente già `'self'` e l'origine dell'`authorization_endpoint` scoperta, ma Chrome applica `form-action` all'intera catena di reindirizzamento. Se il vostro IdP reindirizza attraverso un secondo host, come Azure AD federato ad ADFS, Okta hub-spoke o un intercettore SSO aziendale, elencate ogni origine attraverso cui la richiesta di autorizzazione può reindirizzare.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `ca_cert_pem`                   | No           | Il certificato CA PEM stesso, non un percorso a un file. Sostituisce l'archivio di fiducia del sistema solo per le richieste IdP. Per caricare un file montato, scrivete `${file:/etc/gateway/idp-ca.pem}`. Usate per Keycloak o Dex dietro PKI aziendale.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h4 id="idp-requests-through-a-forward-proxy">
  Richieste IdP attraverso un proxy forward
</h4>

Gli upstream di inferenza onorare `HTTPS_PROXY` e `HTTP_PROXY` su ogni versione. Le richieste del gateway stesso all'IdP, scoperta, JWKS, token e userinfo, vanno dirette a meno che non impostiate `oidc.use_proxy: true`, che richiede v2.1.227 o successivo. Quando una variabile proxy è impostata, `use_proxy` non è impostato e l'emittente non è coperto da `NO_PROXY`, il gateway mantiene quelle richieste dirette e registra un avviso all'avvio chiedendovi di scegliere; `use_proxy: false` le mantiene dirette e silenzia l'avviso.

Con `use_proxy: true`, il pod risolve il nome host di ogni endpoint IdP stesso e chiede al proxy di `CONNECT` all'indirizzo IP risolto, quindi il proxy deve accettare `CONNECT` all'indirizzo IP di ogni host che il documento di scoperta nomina, non solo l'emittente. Usate un URL proxy `http://`. `ca_cert_pem` e la [guardia SSRF](/docs/it/claude-apps-gateway-deploy#threat-model-summary) si applicano anche sul percorso proxato.

<h3 id="session">
  `session`
</h3>

Il blocco `session` modella i token bearer che il gateway conia dopo l'accesso: il segreto che li firma e quanto a lungo vivono.

| Campo        | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------ | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | Sì           | Almeno 32 byte di entropia, ad esempio da `openssl rand -base64 32`. Firma i token bearer HS256 del gateway. Accetta una singola stringa o un array per la rotazione: l'indice 0 firma e tutte le voci verificano. Per ruotare, antepone un nuovo segreto, attendi `ttl_hours`, quindi elimina quello vecchio.                                                                                                                                                                               |
| `ttl_hours`  | No           | Durata del token bearer del gateway. Predefinito `1`. La CLI aggiorna silenziosamente prima della scadenza quando l'IdP emette token di aggiornamento. Una durata più breve deprovvede più velocemente; una più lunga fa meno round-trip IdP. Se il vostro IdP non può emettere token di aggiornamento perché `offline_access` non è disponibile, non c'è aggiornamento silenzioso, quindi aumentate a `8` o `12` per evitare di rimandare gli sviluppatori all'accesso al browser ogni ora. |

<h3 id="store">
  `store`
</h3>

Il blocco `store` punta il gateway al suo database PostgreSQL, che contiene le concessioni dei dispositivi e i contatori dei limiti di velocità.

| Campo             | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ----------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | Sì           | URL `postgres://` o `postgresql://`. Obbligatorio: il rendezvous della concessione del dispositivo, dove il callback del browser scrive e la CLI di polling legge, ha bisogno di uno stato cross-replica. Il gateway esegue le sue migrazioni dello schema all'avvio e all'aggiornamento, quindi il ruolo ha bisogno di diritti per creare e alterare le tabelle sullo schema di destinazione. Vedere [Aggiornamenti](/docs/it/claude-apps-gateway-deploy#upgrades) e [Postgres](/docs/it/claude-apps-gateway-deploy#postgres). |
| `username`        | No           | Sovrascrive l'utente in `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `password`        | No           | Credenziale del database. Impostatela qui piuttosto che in `postgres_url` in modo che la credenziale rimanga fuori dall'URL. Accetta qualsiasi carattere e ha la precedenza sulle credenziali dell'URL.                                                                                                                                                                                                                                                                                                               |
| `max_connections` | No           | Dimensione del pool di connessioni Postgres per replica. Predefinito `5`, che è conservativo e amichevole ai database condivisi. Con i [limiti di spesa](#admin) abilitati, il percorso caldo esegue alcune operazioni per richiesta di inferenza, quindi aumentatelo per un database dedicato sotto carico e mantenete repliche × questo sotto il `max_connections` del database.                                                                                                                                    |

Per lo sviluppo locale, puntate `postgres_url` a un contenitore Postgres usa e getta, ad esempio `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` è un elenco ordinato. Il gateway inoltra l'inferenza al primo upstream che risolve il modello richiesto.

Su `5xx`, `429`, `401`, `403`, `404` o timeout il gateway esegue il failover al successivo; altri `4xx` no, perché questi errori sono attribuibili alla richiesta piuttosto che all'upstream. Un `401` o `403` significa che la credenziale del gateway stesso non ha funzionato contro quell'upstream. Un `404` significa che quell'upstream non serve il modello richiesto, quindi un upstream successivo nell'elenco può ancora servirlo.

Se impostate `forward_user_identity: true` su un upstream, un `429` che restituisce a una richiesta che portava l'email dello sviluppatore non esegue il failover. Vedere [come un rifiuto di limite per utente raggiunge lo sviluppatore](#per-user-identity-headers-for-a-proxy-you-run).

Il failover su `404` richiede gateway v2.1.198 o successivo. Le versioni precedenti hanno restituito il primo `404` al client anche quando un upstream successivo nell'elenco serviva il modello.

Più upstream dello stesso provider devono impostare un `name:` distinto.

I client Amazon Bedrock, Claude Platform on AWS, Google Cloud Agent Platform e Microsoft Foundry sono costruiti una volta all'avvio e i loro SDK aggiornano le credenziali internamente, quindi la rotazione delle credenziali cloud non richiede un riavvio. Le chiavi API Anthropic statiche e i bearer sono letti all'avvio; vedere [API Anthropic](#anthropic-api).

<h4 id="upstream-error-messages">
  Messaggi di errore dell'upstream
</h4>

Il gateway restituisce la risposta di errore di un upstream o il suo proprio `502`, a seconda di come gli upstream hanno risposto:

* **Un upstream ha restituito uno stato su cui il gateway non [esegue il failover](#multiple-upstreams)**: quella risposta dell'upstream. Il gateway non prova ulteriori upstream.
* **Ogni upstream che il gateway ha provato ha fallito in un modo su cui [esegue il failover](#multiple-upstreams)**: l'ultimo `429`. Quando nessuno ha restituito un `429`, il gateway preferisce, in ordine, l'ultimo `401` o `403`, l'ultimo `404` e l'ultimo `501`. Quando nessuno ha restituito nessuno di quelli, il proprio `502` del gateway, `all upstreams failed (N attempted)`, dove N conta ogni voce in [`upstreams`](#upstreams), incluse le voci che il gateway ha saltato perché non servono il modello richiesto.

Quando il gateway restituisce la risposta di un upstream, mantiene il codice di stato dell'upstream. Se mantiene il messaggio dell'upstream dipende dal provider. Il corpo di errore di un upstream API Anthropic raggiunge lo sviluppatore invariato.

Gli upstream Amazon Bedrock, Claude Platform on AWS, Google Cloud Agent Platform e Microsoft Foundry possono nominare i vostri ID account, ARN di ruolo e ID di progetto nel loro testo di errore. Il gateway registra quel testo completo nel [log operativo](/docs/it/claude-apps-gateway-deploy#logs). Ciò che lo sviluppatore vede da questi upstream dipende dal rifiuto:

* `400` o `413` nell'envelope di errore standard di Anthropic: il messaggio dell'upstream stesso, come `prompt is too long`. Claude Platform on AWS, Agent Platform e Microsoft Foundry restituiscono questo envelope per i rifiuti dell'API del modello.
* `400` o `413` nella forma propria del provider: un token `capability_rejected:`. Quando il gateway non può classificare il rifiuto, `upstream rejected the request` su un `400` o `request too large for this upstream` su un `413`.
* Qualsiasi altro stato: copia generica per stato, come `upstream rate limit exceeded` su un `429`.

Ad esempio, il gateway sostituisce `Input is too long for requested model.` di Amazon Bedrock con `capability_rejected: prompt_too_long`. Claude Code [compatta automaticamente](/docs/it/errors#prompt-is-too-long) su quel token, come fa su `prompt is too long`.

Mantenere il messaggio `400` o `413` di un upstream cloud o sostituirlo con un token `capability_rejected:` richiede gateway v2.1.233 o successivo.

<h4 id="anthropic-api">
  API Anthropic
</h4>

L'upstream Anthropic minimo è una chiave API dalla [Console Claude](https://platform.claude.com):

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # OR an OAuth bearer (e.g. a Workload-Identity-Federation-exchanged token):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # default; override for a forward proxy
```

Le due forme di credenziale differiscono nell'header che inviano:

* **`api_key`**: invia `x-api-key`. Ruotatela nella Console Claude e aggiornate la variabile env.
* **`oauth_token`**: invia `Authorization: Bearer`. Usate la forma bearer quando la vostra organizzazione emette token a breve durata invece di chiavi API a lunga durata. Il bearer è letto una volta all'avvio, quindi aggiornate rimontando il segreto e riavviando.

Invece di una chiave statica o un bearer, potete usare Workload Identity Federation. Create una regola di federazione seguendo la [guida Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), quindi montate il JWT OIDC del vostro carico di lavoro come file, come un token dell'account di servizio proiettato di Kubernetes o un id-token della piattaforma CI. Il gateway scambia il JWT per un bearer a breve durata e lo aggiorna automaticamente. Il file del token viene riletto ad ogni scambio, quindi i token proiettati ruotati vengono ripresi senza un riavvio.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # required if the rule covers >1 workspace
      # service_account_id: svac_...   # optional expected-target check
```

<a id="per-user-identity-headers-for-a-proxy-you-run" />

<h5 id="per-user-identity-headers-for-a-proxy-you-run">
  Intestazioni di identità per utente per un proxy che gestite
</h5>

Potete puntare l'`base_url` di un upstream `provider: anthropic` a un proxy che gestite invece che all'API Anthropic. Per dire a quel proxy quale sviluppatore ha inviato ogni richiesta, impostate `forward_user_identity: true` su quell'upstream. Il proxy può quindi attribuire la spesa per sviluppatore. Richiede un gateway che esegue Claude Code v2.1.233 o successivo.

Ad esempio, per un proxy su `upstream-gateway.internal.example.com`:

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # default false
```

Il gateway aggiunge questi intestazioni a ogni richiesta che inoltra a quell'upstream.

| Intestazione                  | Valore                                                         |
| ----------------------------- | -------------------------------------------------------------- |
| `x-litellm-end-user-id`       | L'email dello sviluppatore, quando l'IdP l'ha fornita.         |
| `x-claude-gateway-user-id`    | Il soggetto IdP dello sviluppatore, dal claim `sub` del token. |
| `x-claude-gateway-user-email` | L'email dello sviluppatore, quando l'IdP l'ha fornita.         |

Quando il token IdP non porta email, il gateway invia solo `x-claude-gateway-user-id` e omette i due intestazioni email. Se il vostro IdP mette l'email in un claim diverso, impostate [`oidc.email_claim`](#oidc) a quel claim.

Quando il vostro proxy risponde `429` a una richiesta che portava l'email dello sviluppatore, il gateway restituisce quella risposta allo sviluppatore così com'è invece di eseguire il failover al successivo upstream, quindi il vostro budget per utente del proxy o il limite di velocità tiene. Le altre risposte del proxy seguono le [regole di failover](#upstreams) ordinarie. Se il token IdP di uno sviluppatore non porta email, il gateway inoltra le sue richieste senza gli intestazioni email, quindi un `429` a una di quelle richieste conta come capacità upstream e esegue il failover. Prima della v2.1.267 sul server del gateway, ogni `429` eseguiva il failover.

Impostate `forward_user_identity` solo su un upstream il cui `base_url` è un proxy che gestite. Il gateway invia email degli sviluppatori a qualsiasi server che `base_url` nomina. Se `base_url` è l'API Anthropic, che è il predefinito, il gateway si rifiuta di avviarsi.

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Per la distribuzione Bedrock lato client che il gateway sostituisce o fronteggia, vedere [Claude Code su Amazon Bedrock](/docs/it/amazon-bedrock). L'upstream lato gateway:

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # preferred: AWS default credential chain
    # OR explicit credentials:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # OR a Bedrock API bearer token:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Override the bedrock-runtime endpoint for FIPS or VPC-endpoint deployments:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

Un blocco `auth` vuoto usa la catena di credenziali predefinita dell'AWS SDK: variabili env, `~/.aws/credentials`, ruolo di attività ECS, metadati dell'istanza EC2 o IRSA su EKS. In produzione, date al pod del gateway un ruolo IAM invece di incorporare chiavi statiche in un'immagine del contenitore.

Le credenziali esplicite devono essere complete: il gateway non riesce all'avvio quando `aws_access_key_id` e `aws_secret_access_key` non sono impostati insieme, o quando `aws_session_token` è impostato senza di loro. Prima della v2.1.207, un blocco `auth:` parziale ha superato la convalida.

| Configurazione     | Come                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Autorizzazioni IAM | Concedete al principale del gateway `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` sia sugli ARN del profilo di inferenza che sugli ARN del modello di fondazione sottostante. Per il catalogo integrato nelle regioni US: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` e `arn:aws:bedrock:*::foundation-model/anthropic.*`. Concedete anche `bedrock:CountTokens` sugli ARN del modello di fondazione. Il gateway lo usa, senza costi, per contare i token di input di una richiesta che il client ha abbandonato, quindi i [limiti di spesa](#admin) rimangono accurati. Senza di esso il gateway ricade a una richiesta Bedrock di un token per quel conteggio. |
| Accesso al modello | Amazon Bedrock abilita l'accesso al modello per impostazione predefinita nelle regioni commerciali. Il gate a livello di account rimanente è quello di Anthropic: se nessuno nel vostro account AWS l'ha inviato, aprite la console Amazon Bedrock, selezionate un modello Anthropic dal catalogo dei modelli e completate il modulo. Vedere [Inviare i dettagli del caso d'uso](/docs/it/amazon-bedrock#1-submit-use-case-details) per il modulo AWS Organizations e le autorizzazioni di cui il mittente ha bisogno.                                                                                                                                                                                            |
| EKS (IRSA)         | Create un ruolo IAM con la politica sopra e una politica di fiducia per il provider OIDC del vostro cluster limitato all'account di servizio del gateway. Annotate l'account di servizio con `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` lo raccoglie.                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ECS / EC2          | Allegate il ruolo IAM alla definizione dell'attività o al profilo dell'istanza. `auth: {}` lo raccoglie.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Altrove            | Passate le credenziali tramite le variabili env `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN`, o impostatele esplicitamente in `auth:` con l'espansione `${VAR}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Regione            | `region:` è la regione dell'endpoint API. I profili di inferenza cross-region instradano attraverso la geo (US, EU, APAC) indipendentemente da quale scegliete. Per le regioni non-US o gli ARN di throughput provisioned, aggiungete un blocco [`models:`](#models) con gli ID per upstream corretti.                                                                                                                                                                                                                                                                                                                                                                                                       |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS serve l'API Anthropic di prima parte su infrastruttura AWS su `aws-external-anthropic.<region>.api.aws`. Utilizza ID modello di prima parte, onora gli header `anthropic-beta` come inviati e serve `count_tokens`, quindi nessuna della traduzione specifica di Bedrock si applica. Il provider `anthropicAws` richiede Claude Code v2.1.198 o successivo; le versioni precedenti del gateway lo rifiutano all'avvio.

Per la distribuzione lato client della stessa piattaforma, vedere [Claude Code su Claude Platform on AWS](/docs/it/claude-platform-on-aws). L'upstream lato gateway:

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # sent as x-api-key
    # OR SigV4 via the AWS default credential chain:
    # auth: {}
    # OR explicit SigV4 credentials:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Override the derived endpoint:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

La piattaforma viene eseguita in un account AWS separato da Amazon Bedrock e firma le richieste SigV4 per il suo nome di servizio, `aws-external-anthropic`, quindi un ruolo IAM limitato a Bedrock non lo autorizza. Una chiave API in `auth.api_key` ha la precedenza quando le credenziali SigV4 sono anche impostate. Un blocco `auth` vuoto usa la catena di credenziali predefinita dell'AWS SDK, la stessa catena che l'upstream [Amazon Bedrock](#amazon-bedrock) utilizza.

| Campo                                                   | Obbligatorio | Descrizione                                                                                                                                    |
| ------------------------------------------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `region`                                                | Sì           | Regione AWS, lettere minuscole, cifre e trattini. Il gateway deriva l'endpoint da essa come `https://aws-external-anthropic.<region>.api.aws`. |
| `workspace_id`                                          | Sì           | Inviato come header su ogni richiesta; la piattaforma lo richiede                                                                              |
| `auth.api_key`                                          | No           | Chiave API per la piattaforma, inviata come `x-api-key`. Non un token bearer: le due modalità di autenticazione sono una chiave API o SigV4.   |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | No           | Credenziali SigV4 esplicite. L'impostazione di una senza l'altra non riesce all'avvio. `auth.aws_session_token` è accettato insieme a loro.    |
| `base_url`                                              | No           | Override dell'endpoint derivato                                                                                                                |

Poiché la piattaforma risolve ID modello di prima parte, il catalogo integrato instrada ad essa senza un blocco [`models:`](#models). Quando curate un elenco `models:`, chiave l'entry `anthropicAws:` con l'ID di prima parte.

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

Per la configurazione equivalente lato client, vedere [Claude Code su Google Cloud](/docs/it/google-vertex-ai). L'upstream lato gateway:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # preferred: Application Default Credentials
    # OR a service account key file:
    # auth: { service_account_json: /secrets/sa.json }
    # Override the aiplatform endpoint for Private Service Connect:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

Un blocco `auth` vuoto usa Application Default Credentials: `GOOGLE_APPLICATION_CREDENTIALS`, metadati GCE o GKE Workload Identity. I file di chiave JSON dell'account di servizio sono supportati ma sconsigliati; usate Workload Identity o allegate un account di servizio all'istanza GCE o Cloud Run.

Impostate `region: global` per usare l'[endpoint globale di Agent Platform](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) invece di uno regionale. Google quindi instrada ogni richiesta a una regione disponibile, quindi non tracciate la disponibilità del modello per regione. L'impostazione di una regione specifica fissa ogni richiesta ad essa.

| Configurazione          | Come                                                                                                                                                                                                                                     |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Autorizzazioni IAM      | Concedete all'account di servizio del gateway `roles/aiplatform.user` sul progetto, o un ruolo personalizzato con `aiplatform.endpoints.predict`. Abilitate l'API Agent Platform (`aiplatform.googleapis.com`).                          |
| Accesso al modello      | In Model Garden, abilitate i modelli Claude per il vostro progetto. Pubblicano in regioni specifiche; controllate la scheda del modello per le regioni supportate.                                                                       |
| GKE (Workload Identity) | Legate un account di servizio GCP all'account di servizio Kubernetes del gateway e annotate il KSA con `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`. `auth: {}` lo raccoglie.                         |
| Cloud Run / GCE         | Impostate l'account di servizio del servizio su uno con `roles/aiplatform.user`. `auth: {}` lo raccoglie.                                                                                                                                |
| Altrove                 | `auth: { service_account_json: /secrets/sa.json }`, il percorso a un file di chiave JSON montato come segreto. Il campo accetta un percorso di file, non i contenuti della chiave, quindi non è coinvolta alcuna espansione `${file:…}`. |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Per la distribuzione Foundry lato client, vedere [Claude Code su Microsoft Foundry](/docs/it/microsoft-foundry). L'upstream lato gateway:

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # preferred: DefaultAzureCredential / Managed Identity
    # OR an API key:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` si risolve tramite `DefaultAzureCredential`: Managed Identity su AKS, ACI o App Service; l'Azure CLI o le credenziali di ambiente. Le chiavi API funzionano ma sono a livello di progetto e non ruotano automaticamente. L'endpoint di Foundry è derivato da `resource:`; impostate l'`base_url` facoltativo per sovrascriverlo per cloud sovrani come Azure Government.

| Configurazione          | Come                                                                                                                                                                                                        |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                    | Concedete all'identità del gateway `Azure AI User` o `Cognitive Services User` sulla risorsa Foundry                                                                                                        |
| Distribuzioni           | Foundry usa nomi di distribuzione scelti dall'amministratore, non ID di modello canonici. Aggiungete un blocco [`models:`](#models) che mappa ogni ID canonico al vostro nome di distribuzione.             |
| AKS (workload identity) | Federate un'Identità Gestita Assegnata dall'Utente con l'emittente OIDC del cluster e legatela all'account di servizio del gateway. `use_azure_ad: true` lo raccoglie tramite `WorkloadIdentityCredential`. |
| ACI / App Service       | Abilitate l'identità gestita assegnata dal sistema o assegnata dall'utente sulla risorsa. `use_azure_ad: true` lo raccoglie.                                                                                |
| Altrove                 | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Quotate `${…}` dentro `{ }`.                                                                                                                                     |

<h4 id="multiple-upstreams">
  Più upstream
</h4>

Lo stesso provider può apparire più di una volta con un `name:` distinto. Questo copre regioni diverse, account diversi tramite catene di credenziali diverse, throughput provisioned rispetto a on-demand e failover cross-provider.

Il gateway prova gli upstream in ordine. `5xx`, `429`, `401`, `403`, `404`, timeout e endpoint mancante (`501`) eseguono il failover; altri `4xx` no.

`429` è capacità per upstream, quindi l'esaurimento del throughput provisioned (PT) esegue il failover a on-demand. Se impostate [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run) su un upstream, un `429` a una richiesta che portava l'email dello sviluppatore è un rifiuto per utente invece e non esegue il failover.

`404` è disponibilità del modello per upstream, quindi un upstream che non ha abilitato un modello non blocca un upstream successivo che lo serve. Un upstream che non può risolvere il modello richiesto viene saltato senza un round-trip di rete.

Questo esempio instrada un'allocazione Bedrock di throughput provisioned per primo, trabocca a on-demand e un secondo account e ricade all'API Anthropic per ultimo:

```yaml theme={null}
upstreams:
  # Primary: provisioned throughput in your home region.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Overflow: on-demand cross-region.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Different account: a separate Bedrock allotment via assumed-role creds.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Last resort: direct Anthropic API.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# Per-upstream model IDs are keyed on the upstream's `name:`.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Leva                            | Come                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Regioni diverse                 | Un upstream Bedrock per regione, ciascuno con la sua `region:`. Con [`auto_include_builtin_models: true`](#models) i profili di inferenza cross-region instradano automaticamente; per distribuzioni fissate per regione usate un blocco `models:`.                                                                                                                                                                                                                                                            |
| Account diversi                 | Un upstream Bedrock per account, ciascuno con le sue credenziali in `auth:`. La catena predefinita (`auth: {}`) usa l'identità del pod; per un secondo account, impostate credenziali esplicite o un token bearer.                                                                                                                                                                                                                                                                                             |
| Throughput provisioned          | Mappate il modello all'ARN di throughput provisioned in `models:` per il nome di quell'upstream. Gli altri upstream mantengono l'ID on-demand, quindi la capacità PT è esaurita prima del failover.                                                                                                                                                                                                                                                                                                            |
| Endpoint VPC / FIPS             | Impostate `base_url:` sull'upstream al vostro URL di endpoint VPC o FIPS                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Instradamento scoped al modello | Solo un modello `id` personalizzato, uno che non è un modello Claude integrato, salta gli upstream assenti dalla sua mappa `upstream_model:`. Il gateway prova i modelli integrati su ogni upstream in ordine e usa l'ID predefinito del provider dove la mappa non ha voce, quindi per i modelli integrati la mappa cambia quale ID un upstream riceve piuttosto che se viene provato; un upstream che rifiuta l'ID segue le stesse [regole di failover](#upstreams) di qualsiasi altro errore dell'upstream. |

Il failover tra provider cloud o all'API Anthropic diretto cambia quale accordo, geografia e altri termini governano la richiesta.

La CLI applica lo stesso feature gating ai gateway indipendentemente da quale upstream serve una data richiesta, quindi il failover non invia un campo del corpo che un upstream rifiuterebbe.

<h2 id="optional-sections">
  Sezioni facoltative
</h2>

<h3 id="admin">
  `admin`
</h3>

Facoltativo. Abilita `/v1/organizations/spend_limits`, che rispecchia l'API Admin pubblica di Anthropic, e l'applicazione di limiti di spesa per sviluppatore su `/v1/messages`. Vedi [Spend limits](/docs/it/claude-apps-gateway-spend-limits) per come vengono impostati e applicati i cap; questa sezione copre le chiavi `gateway.yaml` che attivano la funzione e la sintonizzano.

```yaml theme={null}
admin:
  # Named static API keys for the admin endpoints, sent as x-api-key.
  # The id appears in the audit log as admin-key:<id> so each key is
  # attributable. Array for rotation: add the new key, roll clients,
  # remove the old.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # IdP groups granted full admin via the normal gateway JWT (no API key).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Campo                     | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | No           | Array di `{id, key}`. Un `x-api-key` che corrisponde a uno di questi può elencare, impostare ed eliminare i limiti di spesa. I valori delle chiavi devono essere almeno 32 caratteri; gli `id` devono essere univoci tra `read_keys` e `write_keys`.                                                                                                                                                    |
| `read_keys`               | No           | Array di `{id, key}`. Sola lettura: ogni endpoint `GET`, incluso l'elenco dei cap, il recupero di uno per ID e la lettura di [`/effective`](/docs/it/claude-apps-gateway-spend-limits#%2Feffective) e [`/audit`](/docs/it/claude-apps-gateway-spend-limits#%2Faudit).                                                                                                                                             |
| `admin_groups`            | No           | Nomi dei gruppi IdP. Un JWT del gateway il cui claim `groups` include uno di questi ha accesso admin completo, lettura e scrittura, e controlla come `oidc:<sub>`. Usa questo per gli admin umani; usa le chiavi API per le macchine. Una voce vuota in questo elenco arresta il gateway all'avvio. Vedi [Matcher values that stop the gateway at boot](#matcher-values-that-stop-the-gateway-at-boot). |
| `blocked_message`         | No           | Aggiunto verbatim al `429 billing_error` che uno sviluppatore bloccato vede. Scrivi l'intera istruzione, come un URL o un canale Slack. Se non impostato, il gateway invia solo il messaggio predefinito. Vedi [How enforcement works](/docs/it/claude-apps-gateway-spend-limits#how-enforcement-works).                                                                                                     |
| `audit_retention_days`    | No           | Predefinito `365`. Le righe `admin_audit` più vecchie vengono eliminate.                                                                                                                                                                                                                                                                                                                                |
| `spend_retention_months`  | No           | Predefinito `13`. Le righe del contatore `spend` più vecchie di questo vengono eliminate. Il valore predefinito mantiene un anno completo più il mese parziale corrente per i rapporti anno su anno.                                                                                                                                                                                                    |
| `identity_retention_days` | No           | Predefinito `90`. TTL dell'ultimo accesso per le righe `principal_emails`, che contengono l'email, il nome visualizzato e i gruppi di ogni sviluppatore (PII). Deliberatamente più breve della conservazione della spesa in modo che un'identità deprovisioning invecchi mentre i suoi contatori di spesa anonimi rimangono.                                                                            |
| `group_limit_mode`        | No           | `min` (predefinito) o `max`. Quando uno sviluppatore è in diversi gruppi con cap, `min` applica il più restrittivo e `max` il meno restrittivo. Utilizzato sia dall'applicazione che da `/effective`.                                                                                                                                                                                                   |

<h3 id="enforcement">
  `enforcement`
</h3>

Il blocco `enforcement` controlla il comportamento dei controlli dei limiti di spesa quando l'archivio non è disponibile.

| Campo                  | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | No           | Predefinito `false`. L'applicazione della spesa fallisce aperta in caso di interruzione di Postgres, quindi l'inferenza rimane attiva. Imposta `true` per fallire chiuso: gli sviluppatori oltre il cap vengono bloccati, ma lo è anche chiunque altro se l'archivio non è raggiungibile. Richiede un blocco [`admin:`](#admin): l'applicazione della spesa viene eseguita solo quando `admin` è configurato, e il gateway rifiuta di avviarsi se imposti questo `true` senza uno. |

<h3 id="pricing">
  `pricing`
</h3>

Il blocco `pricing` dice al misuratore di spesa cosa addebitare invece del prezzo di listino USD, in modo che i cap e [`/effective`](/docs/it/claude-apps-gateway-spend-limits#%2Feffective) riflettano le tue tariffe contrattuali. Gli importi rimangono in USD e rimangono una stima, non una fattura. Due prerequisiti:

* Claude Code v2.1.227 o successivo sul server del gateway. Le versioni precedenti rifiutano la chiave sconosciuta all'avvio.
* Un blocco [`admin:`](#admin) o, in v2.1.268 o successivo, un blocco [`managed:`](#managed) con almeno una policy. Il gateway rifiuta di avviarsi con `pricing` impostato e nessuno dei due blocchi, perché nulla lo leggerebbe.

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| Campo        | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                 |
| ------------ | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | No           | Predefinito `1`. Il misuratore moltiplica ogni importo misurato per questo, sia che sia a prezzo di listino che sovrascritto, quindi `0.85` addebita l'85% del prezzo. Deve essere maggiore di 0 e al massimo 10, e un valore superiore a 1 è un [markup](#mark-prices-up). |
| `overrides`  | No           | Righe di `{upstream, model, input, output, cache_read, cache_write}` in USD per milione di token. Tutti e quattro i tassi sono obbligatori. Ognuno deve essere maggiore di 0 e al massimo 10000.                                                                            |

Come il misuratore abbina una riga di override:

* Una riga sostituisce il prezzo di listino per le richieste che `upstream`, un [`upstreams[].name`](#upstreams), serve per `model`. Questo include il tasso [fast mode](/docs/it/fast-mode#understand-the-cost-tradeoff) più alto, quindi le richieste fast e standard vengono misurate agli stessi quattro tassi.
* Un ID incorporato come `claude-sonnet-4-6`, abbinato come [`models[].id`](#models), copre ogni forma datata, forma regionale di Amazon Bedrock, o forma di Google Cloud's Agent Platform che il misuratore prezza come quel modello. Qualsiasi altra stringa, come un alias o un ARN del profilo di inferenza, abbina l'ID che il client ha inviato o la stringa inviata upstream, senza distinzione tra maiuscole e minuscole.
* Dove le righe si sovrappongono, il misuratore sceglie la riga più specifica piuttosto che la prima riga: una riga il cui `model` è la stringa di modello esatta inviata upstream, quindi una riga che corrisponde all'ID esatto che il client ha inviato, quindi una riga che nomina il modello incorporato.
* Un nome upstream sconosciuto fallisce all'avvio, così come due righe per uno upstream che nominano lo stesso modello, incluse due ortografie di un modello incorporato. Il gateway avverte all'avvio di una riga che nessun modello richiedibile può utilizzare.
* Le richieste di ricerca web rimangono al prezzo di listino \$0.01; il moltiplicatore si applica comunque a loro.

Per tariffe per regione, dai a ogni regione il suo upstream denominato e una riga per upstream.

<h4 id="mark-prices-up">
  Aumenta i prezzi
</h4>

Con v2.1.271 o successivo sul server del gateway, puoi impostare `multiplier` sopra 1, fino a 10, per misurare più di quanto il provider addebita, ad esempio un tasso di chargeback interno. Questo esempio misura ogni richiesta al 120% del prezzo:

```yaml theme={null}
pricing:
  multiplier: 1.2
```

Con un blocco [`admin:`](#admin), il markup si applica anche ai limiti di spesa. Il misuratore conta il 120% del prezzo, quindi gli sviluppatori raggiungono i loro cap più velocemente. Il gateway registra un avviso all'avvio che lo dice.

Il moltiplicatore non cambia quello che il provider upstream addebita per le richieste.

Se il gateway inoltre [invia le tariffe ai client firmati](#send-the-rates-to-signed-in-clients), gli sviluppatori hanno bisogno di Claude Code v2.1.271 o successivo per vedere il markup. I client precedenti ignorano un `multiplier` superiore a 1 e mostrano i costi senza di esso.

Un server gateway precedente a v2.1.271 rifiuta di avviarsi se imposti un `multiplier` superiore a 1.

<h4 id="send-the-rates-to-signed-in-clients">
  Inviare le tariffe ai client firmati
</h4>

Con v2.1.268 o successivo sul server del gateway, il gateway mette anche le tariffe da `pricing` nelle policy [`managed`](#managed) che serve, come l'impostazione gestita [`modelPricing`](/docs/it/settings-reference#modelpricing). Gli sviluppatori abbinati da una policy vedono quindi le tariffe `pricing` per il primo upstream che serve ogni ID modello in `/usage`, la riga di stato e OpenTelemetry. Uno sviluppatore che non corrisponde a nessuna policy non riceve impostazioni gestite, quindi le sue cifre rimangono al prezzo di listino. I client applicano l'impostazione in Claude Code v2.1.242 o successivo.

* Cosa aggiunge il gateway: a meno che il blocco `cli` di una policy non imposti già `modelPricing`, il gateway aggiunge il `multiplier` e, per ogni ID modello che un client può richiedere, la riga di override del primo upstream che serve quell'ID. Un tasso che solo un upstream di failover addebita rimane sul gateway.
* Escludi una policy: imposta `modelPricing` a `{}` nel blocco `cli` di quella policy, e i suoi sviluppatori rimangono al prezzo di listino.
* Mantieni le tariffe proprie di una policy: una policy il cui blocco `cli` imposta `modelPricing` con il suo `multiplier` o `overrides` mantiene quel `modelPricing` intero, e il gateway non aggiunge tariffe proprie a esso.

<h3 id="models">
  `models`
</h3>

Il blocco `models` è un elenco di modelli curato da admin facoltativo, servito su `/v1/models` e utilizzato per tradurre gli ID modello per upstream. È obbligatorio per le regioni non statunitensi di Amazon Bedrock, gli ARN di throughput con provisioning di Amazon Bedrock e i nomi di distribuzione di Microsoft Foundry.

```yaml theme={null}
auto_include_builtin_models: true   # false: expose only the list below
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optional text shown in clients that surface it
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # or an inference-profile ARN
      foundry: your-opus-deployment-name
```

Ogni chiave sotto `upstream_model` deve corrispondere al `name` di un upstream configurato, che per impostazione predefinita è il nome del provider. Una chiave che non corrisponde a nessun upstream fallisce all'avvio, quindi ometti le righe per i provider che non usi.

<h3 id="managed">
  `managed`
</h3>

Il blocco `managed` definisce policy di accesso basate su ruoli basate su gruppi IdP o dominio di posta elettronica. Le policy vengono valutate in ordine; la prima corrispondenza viene selezionata, quindi unita alla base catch-all `match: {}`. Vengono servite per utente su `GET /managed/settings` con caching ETag/304.

```yaml theme={null}
managed:
  policies:
    # Specific groups first.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Default catch-all last: matches everyone who authenticated.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Un catch-all `match: {}`, convenzionalmente elencato per ultimo, viene trattato come un livello base. Ogni altra policy eredita qualsiasi chiave che non imposta dalla catch-all, quindi le voci per ruolo devono solo elencare ciò che differisce dal valore predefinito dell'organizzazione. Le regole di unione dipendono dal tipo di chiave:

* **Allow-lists**: `availableModels` e `permissions.allow`. L'elenco di una policy specifica sostituisce completamente quello della base.
* **Deny-lists e hook arrays**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` e ogni array di tipo evento `hooks`. Questi prendono l'unione di base e policy, quindi un deny a livello di organizzazione o un hook di audit non può essere accidentalmente eliminato da un override per ruolo.
* **Record-typed keys**: `env`, `modelOverrides` e `skillOverrides`. Questi si uniscono superficialmente, quindi un blocco `env` per ruolo sostituisce le chiavi che imposta e eredita il resto dalla base.

`availableModels` viene anche applicato lato server su `/v1/messages`, quindi un modello negato restituisce `400` indipendentemente da quello che il client invia.

Il gateway convalida il valore `model` stesso prima di inoltrare una richiesta, quindi un valore malformato non raggiunge mai un upstream. Rifiuta la richiesta con un `400` in due casi:

* Quando il valore è mancante o vuoto, il gateway rifiuta la richiesta con il messaggio `model is required`. Questo controllo richiede un gateway che esegue Claude Code v2.1.228 o successivo.
* Quando il valore è presente ma non è una stringa, il gateway rifiuta la richiesta con il messaggio `model must be a string`. Richiede un gateway che esegue Claude Code v2.1.221 o successivo.

| Matcher                                             | Comportamento                                                                                                                                                      |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `match: {}`                                         | Corrisponde a ogni utente autenticato. Inizia con uno di questi e aggiungi policy con ambito di gruppo sopra di esso in seguito.                                   |
| `match: { groups: [a, b] }`                         | Corrisponde se il claim `groups` del JWT contiene uno dei gruppi elencati. Sensibile alle maiuscole: i gruppi devono corrispondere alle maiuscole esatte dell'IdP. |
| `match: { email_domain: example.com }`              | Corrisponde alla parte dopo l'ultimo `@` nel claim `email` del JWT, senza distinzione tra maiuscole e minuscole. Accetta un dominio per policy.                    |
| `match: { groups: [a], email_domain: example.com }` | Entrambe le condizioni devono corrispondere                                                                                                                        |

Un utente autenticato che non corrisponde a nessuna policy ottiene i valori predefiniti del gateway, il che significa ogni modello nel catalogo e nessuna impostazione gestita. Aggiungi un catch-all `match: {}` per ultimo se vuoi una policy predefinita garantita.

<Note>
  Il gateway non mantiene una propria directory utente. Autorizza ogni richiesta dal token IdP dell'utente, leggendo l'appartenenza al gruppo dal claim `groups` del token e valutando le policy rispetto ad esso. Non c'è roster da enumerare e nessun account da pre-creare, e quindi nessun endpoint SCIM, perché non c'è nulla per SCIM da sincronizzare.

  Esegui la gestione del ciclo di vita di utenti e gruppi alla fonte della verità, che è il provisioning SCIM nativo del tuo IdP o una piattaforma dedicata di governance dell'identità. L'appartenenza e il deprovisioning governati lì fluiscono nel gateway automaticamente attraverso il token. Se vuoi il provisioning SCIM degli account Claude stessi, questa è una capacità di [Claude for Enterprise](/docs/it/admin-setup).

  Si applicano due orologi di propagazione:

  * **Contenuti della policy**: modificare una policy e ridistribuire raggiunge i client connessi al loro prossimo sondaggio di impostazioni gestite, entro un'ora, a parte i [cambiamenti che si applicano solo al prossimo avvio](/docs/it/server-managed-settings#fetch-and-caching-behavior)
  * **Appartenenza al gruppo**: cambiare l'appartenenza al gruppo di un utente cambia quale policy lo corrisponde. Questo ha effetto al prossimo rinnovo della sessione, il che significa il prossimo aggiornamento silenzioso, limitato da `session.ttl_hours`.
</Note>

<h4 id="matcher-values-that-stop-the-gateway-at-boot">
  Matcher values that stop the gateway at boot
</h4>

All'avvio, il gateway controlla il blocco `match` di ogni policy e l'elenco [`admin_groups`](#admin). Uno qualsiasi di questi valori arresta il gateway con un errore che nomina il campo:

* Un elenco `groups` vuoto
* Una voce vuota in `groups` o in `admin_groups`
* Un `email_domain` vuoto
* Un `email_domain` che contiene `@`, spazi bianchi o una virgola. Il gateway taglia il valore e rimuove un `@` iniziale prima di questo controllo. Scrivi un dominio nudo, come `example.com`.

Prima di v2.1.232, il gateway si avviava con questi valori. Ogni valore aveva questo effetto:

* Un `email_domain` vuoto: il gateway ha saltato il controllo del dominio, quindi una policy con un `email_domain` vuoto e nessun elenco `groups` corrisponde a ogni utente autenticato
* Un elenco `groups` vuoto: la policy non corrisponde a nessuno
* Un `email_domain` contenente `@`, spazi bianchi o una virgola: la policy non corrisponde a nessuno
* Una voce vuota in `groups` o in `admin_groups`: la voce corrisponde a un utente solo quando il claim `groups` dell'IdP di quell'utente conteneva anche una voce vuota. In `admin_groups`, quella corrispondenza ha concesso l'accesso admin. Se il tuo elenco `admin_groups` non ha mai contenuto una voce vuota, nessuno ha ottenuto l'accesso admin in questo modo.

<h4 id="what-goes-in-cli">
  What goes in `cli`
</h4>

Ogni valore `cli` è un documento completo di `managed-settings.json` di Claude Code, lo stesso schema che distribuiresti tramite MDM o `/etc/claude-code/managed-settings.json`, espresso qui come YAML. La CLI applica il documento consegnato al livello gestito, sopra le impostazioni di utente e progetto, al posto delle impostazioni gestite dal server. Ignora quindi le impostazioni [ristrette alle fonti di policy a livello di sistema operativo](/docs/it/server-managed-settings#current-limitations), come `policyHelper` e `wslInheritsWindowsSettings`.

Il gateway convalida ogni documento rispetto allo schema delle impostazioni della CLI all'avvio, quindi una chiave di primo livello non riconosciuta fallisce all'avvio con un errore che nomina ogni chiave offensiva. Le parti deliberatamente aperte dello schema accettano ancora valori arbitrari, perché i client più recenti potrebbero riconoscere voci che lo schema del gateway non riconosce. Queste chiavi aperte includono `env`, `pluginConfigs` e chiavi annidate sotto `permissions`.

Poiché la convalida utilizza lo schema fornito con la versione installata del gateway, mettere una chiave di impostazioni di primo livello introdotta da una versione più recente di Claude Code nella configurazione gestita richiede prima l'aggiornamento del gateway. Smoke-test una nuova policy su un client prima di distribuirla.

Il riferimento completo della chiave è in [Claude Code settings](/docs/it/settings-reference#all-settings). Le chiavi che gli operatori raggiungono per prime:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Model access (also enforced server-side at /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Permission policy
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blocks --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignore user/project permission rules

        # Environment pushed into the CLI process. DISABLE_UPDATES blocks
        # background and manual updates; DISABLE_AUTOUPDATER stops only
        # background updates.
        env:
          DISABLE_UPDATES: "1"                    # pin versions via your own distribution

        # Org-wide hooks. Hook commands run on developer machines, not the
        # gateway, so the path must exist on every client OS in the policy.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Chiave                                     | Applicato da  | Effetto                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------------------------ | ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Gateway + CLI | Allowlist del modello. Anche controllato su `/v1/messages`, quindi un client patchato non può bypassarlo.                                                                                                                                                                                                                                                                     |
| `permissions.allow` / `.deny`              | CLI           | Regole di strumenti e comandi. Vedi [Permissions](/docs/it/permissions).                                                                                                                                                                                                                                                                                                           |
| `permissions.disableBypassPermissionsMode` | CLI           | Imposta su `disable` per bloccare [`bypassPermissions`](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode), la modalità che salta i prompt di autorizzazione, e il flag `--dangerously-skip-permissions`                                                                                                                                                       |
| `allowManagedPermissionRulesOnly`          | CLI           | Quando `true`, le impostazioni gestite diventano l'unica fonte di impostazioni delle regole di autorizzazione. La voce [`allowManagedPermissionRulesOnly`](/docs/it/settings-reference#allowmanagedpermissionrulesonly) elenca ogni fonte che Claude Code ignora.                                                                                                                  |
| `env`                                      | CLI           | Variabili di ambiente unite nel processo CLI. Usa per telemetria, auto-aggiornamento e override dei nomi dei modelli.                                                                                                                                                                                                                                                         |
| `hooks`                                    | CLI           | [hooks](/docs/it/hooks) a livello di organizzazione                                                                                                                                                                                                                                                                                                                                |
| `managedMcpServers`                        | CLI           | Server MCP remoti [forniti a ogni sviluppatore corrispondente](/docs/it/managed-mcp#provide-servers-through-managed-settings) insieme ai server che aggiungono loro stessi, `http` e `sse` solo. Vedi [MCP servers in a policy](#mcp-servers-in-a-policy). Richiede Claude Code v2.1.259 o successivo sul server del gateway e sui client. I client precedenti ignorano la chiave. |

Poiché queste impostazioni arrivano sulla rete, la CLI mostra a ogni sviluppatore una finestra di dialogo di approvazione della sicurezza prima di applicare le impostazioni elencate di seguito:

* `hooks`
* Variabili `env` che richiedono l'approvazione dello sviluppatore, come variabili proxy e base-URL
* impostazioni di esecuzione della shell come `apiKeyHelper` e `statusLine`
* le impostazioni binarie della sandbox `sandbox.bwrapPath`, `sandbox.socatPath` e `sandbox.ripgrep`
* Impostazioni della sandbox che intercettano il traffico, iniettano credenziali o indeboliscono l'isolamento, come `sandbox.network.tlsTerminate` e le impostazioni della porta proxy. [Security approval dialogs](/docs/it/server-managed-settings#security-approval-dialogs) le elenca tutte.

[Approval memory](/docs/it/server-managed-settings#approval-memory) copre quanto dura un'approvazione e quando la finestra di dialogo appare di nuovo.

Claude Code applica alcune variabili `env` consegnate senza mostrare allo sviluppatore la finestra di dialogo di approvazione, come le impostazioni di selezione del modello e i limiti numerici. Altre variabili consegnate possono richiedere l'approvazione dello sviluppatore prima di avere effetto; un valore proxy, base-URL o `OTEL_EXPORTER_OTLP_ENDPOINT` non vuoto lo fa sempre. Quando una variabile consegnata ha bisogno di approvazione, la finestra di dialogo la nomina.

[Environment variables and the approval dialog](/docs/it/server-managed-settings#environment-variables-and-the-approval-dialog) ha i dettagli, inclusi quattro interruttori di privacy il cui valore consegnato decide se hanno bisogno di approvazione. Prima di v2.1.218, Claude Code applicava meno variabili senza chiedere allo sviluppatore, quindi più variabili consegnate attivavano la finestra di dialogo.

La configurazione [telemetry](#telemetry) del gateway spinge `OTEL_EXPORTER_OTLP_ENDPOINT`, quindi impostare `telemetry.forward_to` attiva la finestra di dialogo su ogni client interattivo. La finestra di dialogo protegge la macchina dello sviluppatore da un gateway compromesso o ostile, non l'organizzazione dallo sviluppatore.

Un'esecuzione non interattiva con il flag `-p` non può mostrare la finestra di dialogo. Applica le impostazioni spinte per quella sola esecuzione e non le registra come approvate, quindi la prossima sessione interattiva dello sviluppatore mostra comunque la finestra di dialogo per loro. Prima di v2.1.207, un'esecuzione non interattiva salvava le impostazioni come approvate e nessuna sessione interattiva successiva mostrava la finestra di dialogo per loro.

Se uno sviluppatore rifiuta, Claude Code esce da quella sessione piuttosto che applicare la policy. Quando spingi un nuovo hook, o qualsiasi variabile env che attiva la finestra di dialogo, a una policy ampia, Claude Code mostra quindi la finestra di dialogo a ogni sviluppatore corrispondente. Mostra la finestra di dialogo in una sessione in esecuzione al prossimo sondaggio orario, e altrimenti all'avvio successivo dello sviluppatore.

La chiave `cli` era denominata `settings` nelle versioni precedenti. Questo spelling è ancora accettato come alias, ma le nuove distribuzioni dovrebbero usare `cli`.

<h4 id="mcp-servers-in-a-policy">
  MCP servers in a policy
</h4>

Per fornire server MCP ai client Claude Code che una policy corrisponde, imposta [`managedMcpServers`](/docs/it/managed-mcp#provide-servers-through-managed-settings) nel blocco `cli` di quella policy. Hai bisogno di Claude Code v2.1.259 o successivo sul server del gateway e sui client.

Il gateway controlla ogni voce all'avvio con [le stesse regole che Claude Code applica sul client](/docs/it/managed-mcp#what-an-entry-can-contain), e se una voce fallisce un controllo, il gateway rifiuta di avviarsi e nomina la voce.

Se scrivi un riferimento `${VAR}` in `gateway.yaml`, il gateway lo risolve dal suo ambiente all'avvio attraverso [secret expansion](#secret-expansion) prima di eseguire i controlli della voce, quindi ogni client corrispondente riceve il valore letterale e può leggerlo. La [header guidance for provided servers](/docs/it/managed-mcp#provide-servers-through-managed-settings) si applica al valore espanso.

Il gateway rifiuta lo spelling `.mcp.json` `mcpServers` in un blocco `cli`, e il suo errore di avvio nomina `managedMcpServers` come la chiave da usare. Prima di v2.1.259, il gateway rifiutava qualsiasi definizione di server MCP in un blocco `cli`.

<h4 id="claude-desktop-overlay">
  Claude Desktop overlay
</h4>

Se la tua organizzazione distribuisce anche [Claude Desktop](/docs/it/desktop), lo stesso gateway serve entrambi i client. Punta `bootstrapUrl`, nella [managed configuration](https://claude.com/docs/third-party/claude-desktop/configuration) di Claude Desktop, a `<listen.public_url>/user/bootstrap`. Claude Desktop deriva l'emittente OAuth da quell'URL, esegue lo stesso accesso con codice dispositivo rispetto a questo gateway e recupera la sua configurazione dalla risposta.

<Note>
  Richiede Claude Code v2.1.203 o successivo sul server del gateway e un opt-in esplicito: `/user/bootstrap` restituisce 404 a meno che la policy che corrisponde all'utente non porti una chiave `desktop`. Un `desktop: {}` vuoto opta una policy, e una chiave `desktop` sul livello base `match: {}` opta in ogni policy che la eredita. Il registro di audit registra ogni richiesta come `desktop_bootstrap.serve` o `desktop_bootstrap.denied`.
</Note>

Il gateway deriva gran parte della risposta dal blocco `cli` della policy corrispondente e dalla configurazione del gateway di primo livello:

* L'elenco dei modelli, da `availableModels`
* Strumenti disabilitati, da voci `permissions.deny` con nome di strumento nudo. Se imposti `disabledBuiltinTools` nel blocco `desktop` della policy, il gateway serve l'unione del tuo valore e dell'elenco derivato, quindi puoi disabilitare più strumenti in questo modo ma non puoi riabilitarne uno che hai disabilitato tramite `permissions.deny`
* L'allowlist di uscita, da `sandbox.network.allowedDomains`. Se imposti `coworkEgressAllowedHosts` nel blocco `desktop` della policy, il gateway usa quel valore invece dell'elenco derivato
* Un endpoint OTLP che punta al gateway stesso, e gli attributi di identità dell'utente firmato. Il gateway inoltra le esportazioni che riceve a quell'endpoint alle tue destinazioni `forward_to`. Include l'endpoint e gli attributi quando imposti sia [`telemetry.forward_to`](#telemetry) che `listen.public_url`.

  Claude Desktop esporta ogni segnale con una codifica: `http/protobuf`, o `http/json` quando imposti `OTEL_EXPORTER_OTLP_PROTOCOL` o uno dei suoi varianti per segnale a `http/json` nel `env` della policy. Prima di Claude Code v2.1.261 sul server del gateway, la risposta impostava `http/json` indipendentemente, quindi un collettore che accetta solo protobuf rifiutava le esportazioni di Claude Desktop

Per impostare `disabledBuiltinTools`, `coworkEgressAllowedHosts` o l'impostazione `managedMcpServers` di Claude Desktop stesso nel blocco `desktop` di una policy, hai bisogno di Claude Code v2.1.232 o successivo sul server del gateway. L'impostazione `managedMcpServers` di Claude Desktop accetta un valore di array piuttosto che un oggetto.

Il gateway omette le chiavi senza equivalente di Claude Desktop, come `hooks` e regole di autorizzazione con ambito come `Bash(npm *)`, dalla risposta di bootstrap.

Aggiungi il blocco `desktop` facoltativo insieme a `cli` per impostare le impostazioni di Claude Desktop direttamente. Scrivi le impostazioni dal [managed configuration reference](https://claude.com/docs/third-party/claude-desktop/configuration) di Claude Desktop come nomi di chiave piatti. Lascia fuori le chiavi che Claude Desktop legge solo da MDM o file locali, come `bootstrapUrl`; il gateway le rifiuta all'avvio. Prima di v2.1.232, il gateway accettava un elenco fisso di 11 chiavi di feature-gate, come `chatTabEnabled` e `disableAutoUpdates`, e rifiutava ogni altra chiave all'avvio. Prima di v2.1.227, il gateway rifiutava anche `chatTabEnabled` e `chatAdvancedFileAnalysisEnabled` all'avvio.

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

Ogni chiave è facoltativa; Claude Desktop applica il suo valore predefinito per qualsiasi chiave che ometti. Il gateway convalida ogni blocco `desktop` all'avvio rispetto allo schema di configurazione che Claude Desktop stesso usa, quindi un errore emerge all'avvio del gateway come un errore che nomina la chiave piuttosto che raggiungere ogni desktop connesso. Il gateway fallisce all'avvio quando un blocco contiene:

* Una chiave sconosciuta
* Una chiave riconosciuta il cui valore Claude Desktop rifiuterebbe o lascerebbe cadere silenziosamente, come un valore vuoto o un nome di sub-chiave errato all'interno di una voce annidata. Prima di v2.1.260, il gateway lasciava cadere silenziosamente un campo errato all'interno di un oggetto annidato di una voce `managedMcpServers` o `orgPluginSettings` invece di fallire all'avvio.
* Una chiave che il gateway calcola da solo: la connessione di inferenza, l'elenco dei modelli e l'inoltro OTLP. Configura quelli attraverso [`upstreams`](#upstreams), [`models`](#models) e la sezione [`telemetry`](#telemetry) `forward_to`.
* Un alias legacy di una chiave attuale. Nell'errore di avvio, il gateway nomina la chiave canonica da scrivere.

Se usi un valore o una forma di voce deprecata, come una voce `managedMcpServers` senza `transport`, il gateway si avvia e registra un avviso che nomina la sostituzione.

Il gateway convalida un blocco `desktop` rispetto allo schema fornito con la sua versione installata, come fa con il blocco `cli`. Per consegnare un'impostazione introdotta da una versione più recente di Claude Desktop, aggiorna il gateway prima. Ad esempio, `userPluginMarketplacesEnabled` e `userPluginUploadsEnabled` hanno bisogno di Claude Code v2.1.260 o successivo sul server del gateway e Claude Desktop 1.37937.0 o successivo sulle macchine dei membri.

Se imposti `orgPluginSettings` nel blocco `desktop` di una policy, il gateway lo serve nella forma di array che Claude Desktop 1.15200.0 e successivo legge. I desktop più vecchi ignorano l'array e non applicano alcuna policy di strumento plugin, quindi aggiorna i membri a 1.15200.0 o successivo prima di fare affidamento su di esso.

Il gateway riempie le chiavi che il blocco `desktop` di una policy non imposta dal blocco `desktop` della catch-all `match: {}`, nello stesso modo in cui riempie il blocco `cli` di una policy dalla base. Se imposti `disabledBuiltinTools` o `builtinToolPolicy` sia nella base che in una policy per ruolo, il gateway mantiene la restrizione della base:

* `disabledBuiltinTools`: il gateway usa l'unione dell'elenco della base e dell'elenco della policy
* `builtinToolPolicy`: se imposti uno strumento a un valore diverso da `allow` nella base, il gateway mantiene quel valore anche se imposti `allow` per lo stesso strumento in una policy per ruolo

Per ogni altra chiave, se la imposti nella policy per ruolo, il gateway usa il valore della policy per ruolo. Il gateway sostituisce un array o un oggetto annidato come `banner` interamente, quindi se imposti `banner.text` in una policy per ruolo, il gateway elimina il `banner.backgroundColor` della base.

Se non distribuisci Claude Desktop, lascia `desktop` completamente fuori dalle tue policy; il gateway restituisce quindi 404 da `/user/bootstrap` per ogni utente.

<h4 id="precedence-with-other-managed-sources">
  Precedence with other managed sources
</h4>

Se un dispositivo ha anche una policy consegnata da MDM o un `managed-settings.json` locale, le impostazioni consegnate dal gateway hanno la priorità. [Precedence within the managed tier](/docs/it/managed-settings#precedence-within-the-managed-tier) sulla pagina delle impostazioni gestite dice quando si applicano le fonti locali, e ha le [chiavi che Claude Code legge da ogni fonte admin](/docs/it/managed-settings#keys-read-from-every-admin-source) indipendentemente da quale fonte ha selezionato, come le chiavi di blocco della sandbox, `forceRemoteSettingsRefresh` e il `env` per variabile. Un [`policyHelper`](/docs/it/settings-reference#policyhelper) configurato in un profilo MDM o nel file delle impostazioni gestite viene eseguito solo quando il gateway non consegna impostazioni; la voce dice cosa sostituisce il suo output.

Gli host di incorporamento come [Claude Desktop](/docs/it/desktop) possono fornire policy attraverso l'opzione SDK `managedSettings`. [Parent settings from embedding hosts](/docs/it/managed-settings#parent-settings-from-embedding-hosts) dice quando Claude Code lo applica, e [Restrict parent settings](/docs/it/claude-apps-gateway#restrict-parent-settings) elenca quali impostazioni di direzione di autorizzazione si applicano ancora senza i blocchi `allowManaged*Only`.

Le policy del gateway si applicano a ogni invocazione di Claude Code sulla macchina, incluse le esecuzioni non interattive `claude -p` e le sessioni generate dall'Agent SDK. Se il gateway non è raggiungibile all'avvio, le sessioni firmate escono con un errore piuttosto che eseguire senza la loro policy.

<h3 id="telemetry">
  `telemetry`
</h3>

La CLI invia metriche, log e, quando abilitato, tracce al gateway, che le inoltra verbatim a ogni destinazione configurata. Le esportazioni utilizzano OpenTelemetry Protocol (OTLP) su HTTP. Per saltare l'inoltro e avere sessioni esportate direttamente al tuo collettore, [nomina il collettore in una policy](#export-directly-to-your-collector). Vedi [Monitoring usage](/docs/it/monitoring-usage) per le metriche e gli eventi che la CLI emette.

La CLI timbra ogni esportazione con l'identità dell'utente autenticato, letta dal JWT emesso dal gateway: gli attributi `user.id`, `user.email` e `user.groups`. L'attribuzione di costo e utilizzo per sviluppatore funziona quindi senza configurazione lato sviluppatore.

[Claude Desktop](#claude-desktop-overlay) e le sessioni Cowork firmate attraverso il gateway timbrano la loro telemetria con `user.email` e `user.groups` insieme a `enduser.id`, quindi puoi coprire l'utilizzo di terminale, Desktop e Cowork con una query su `user.email` o `user.groups`. `user.groups` è l'elenco di gruppi IdP separato da virgole.

Come tutti i dati OpenTelemetry da Claude Code, questi attributi vanno solo alle destinazioni che la tua organizzazione configura, mai ad Anthropic.

Se l'elenco di gruppi di un utente è più lungo di 255 caratteri una volta codificato in percentuale, o un nome di gruppo contiene una virgola o un segno di uguale, il gateway lascia `user.groups` fuori dalla telemetria Desktop e Cowork di quell'utente piuttosto che troncarla. Le sessioni di terminale di quell'utente portano comunque l'elenco completo.

Hai bisogno di Claude Code v2.1.265 o successivo sul server del gateway per `user.email` e `user.groups` sulla telemetria Desktop e Cowork, e Claude Desktop 1.24012 o successivo su ogni macchina dello sviluppatore per `user.groups`.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Per-signal opt-in. Default: metrics only.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Ogni destinazione opta in `metrics`, `logs` e `traces` indipendentemente, e il valore predefinito è solo metriche. I segnali differiscono in sensibilità:

  * **Metrics**: contatori aggregati come conteggi di token, conteggi di richieste e latenza
  * **Logs and traces**: possono portare comandi bash completi, input di strumenti e percorsi di file, coprendo tutto ciò che Claude Code fa sulla macchina di uno sviluppatore

  Abilita log e tracce solo su destinazioni con i controlli di accesso e la policy di conservazione che i dati garantiscono.
</Warning>

Ogni URL `forward_to` deve usare `https://`, con un'eccezione per un collettore sull'interfaccia loopback del gateway stesso:

* `http://localhost:<port>` passa la convalida della configurazione, ma la [SSRF guard](/docs/it/claude-apps-gateway-deploy#threat-model-summary) blocca ogni esportazione con `ECONNREFUSED_SSRF` a meno che non imposti `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` nell'ambiente del gateway
* `http://127.0.0.1:<port>` o `http://[::1]:<port>` fallisce all'avvio a meno che quella variabile non sia impostata

Per un collettore in-cluster, esponilo su HTTPS al suo indirizzo interno, o eseguilo come sidecar con la variabile impostata.

La telemetria è disattivata nella CLI per impostazione predefinita. Quando imposti sia `telemetry.forward_to` che `listen.public_url`, il gateway la attiva per i client connessi spingendo sei variabili di ambiente attraverso `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` e `OTEL_TRACES_EXPORTER`, ognuno impostato a `otlp` se almeno una destinazione `forward_to` abilita quel segnale e a `none` altrimenti
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

Prima di Claude Code v2.1.265 sul server del gateway, il gateway spingeva tutti e tre i selettori di esportazione come `otlp`, incluso per i segnali che nessuna destinazione ha optato.

L'endpoint spinto è costruito dall'URL pubblico, quindi metriche e log non hanno bisogno di configurazione OTEL da sviluppatori o policy.

Gli sviluppatori firmati attraverso `/login` non possono reindirizzare le esportazioni con la loro configurazione OTEL:

* **Variabili impostate localmente**: Claude Code applica le variabili spinte al livello gestito, quindi ognuna sostituisce il valore che uno sviluppatore imposta per essa localmente.
* **Endpoint configurati localmente**: con l'esportazione OTLP/HTTP abilitata, la CLI ignora qualsiasi endpoint configurato localmente, indipendentemente dal fatto che il gateway abbia spinto le variabili di telemetria. Le sue esportazioni vanno al gateway a meno che una policy non [nomini il tuo collettore come endpoint](#export-directly-to-your-collector).

Senza una destinazione `forward_to` per un segnale, il gateway lo accetta e lo scarta. Se gli sviluppatori già esportano telemetria di Claude Code a uno dei tuoi collettori, aggiungilo come destinazione `forward_to`, con log o tracce abilitate se esportano quelli, in modo che continui a ricevere i loro dati dopo che si firmano. Per saltare l'inoltro invece, [nomina il collettore in una policy](#export-directly-to-your-collector).

[Traces](/docs/it/monitoring-usage#traces-beta) richiedono anche `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` su ogni client. Impostalo nel blocco `env` di una policy gestita, poiché il gateway non lo spinge. Gli sviluppatori lo approvano nella stessa [security approval dialog](#managed) che l'endpoint spinto già attiva.

Impostalo a `1` solo nelle policy i cui gruppi vuoi tracciati. Una policy che non lo imposta eredita il valore dalla tua policy catch-all `match: {}` se quella policy ne imposta uno, per le [merge rules](#managed). Per impedire ai client di un gruppo di inviare tracce anche quando uno sviluppatore imposta la variabile localmente, impostala a `0` nella policy di quel gruppo.

Sia la codifica protobuf che JSON OTLP vengono inoltrate, e qualsiasi backend compatibile con OpenTelemetry funziona come destinazione.

<h4 id="export-directly-to-your-collector">
  Export directly to your collector
</h4>

Per avere sessioni firmate attraverso `/login` inviare telemetria direttamente al tuo collettore invece che attraverso l'inoltro, imposta `OTEL_EXPORTER_OTLP_ENDPOINT` all'URL base `https://` del collettore nel blocco `env` di una [managed policy](#managed). Claude Code aggiunge `/v1/metrics`, `/v1/logs` o `/v1/traces` all'URL che imposti, come `https://otel-collector.example.com:4318`, ed esporta ogni segnale lì su OTLP/HTTP. Richiede Claude Code v2.1.265 o successivo su ogni macchina dello sviluppatore. I client precedenti esportano attraverso l'inoltro.

Per autenticarti al collettore, imposta `OTEL_EXPORTER_OTLP_HEADERS` nello stesso blocco `env`. Le sessioni non inviano mai il token di sessione del gateway dello sviluppatore a un collettore nominato in questo modo.

Quando aggiungi o cambi questo endpoint in una policy, Claude Code chiede a ogni sviluppatore di approvarlo nella [security approval dialog](#managed) prima di applicarlo in una sessione interattiva.

Claude Code controlla l'endpoint prima di esportare un segnale direttamente, e mantiene quel segnale sull'inoltro quando un controllo fallisce. I controlli includono:

* L'endpoint viene dal gateway stesso. Se imposti la stessa variabile in un profilo MDM o in un `managed-settings.json` locale, le esportazioni rimangono sull'inoltro.
* L'URL usa `https://`, o `http://` a un indirizzo loopback
* L'URL si risolve in un percorso che termina in `/v1/<signal>`, senza query o frammento. Claude Code costruisce quel percorso da solo dalla variabile generica. Usa una variabile per segnale come `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` come scritto, quindi includi il percorso completo lì.
* L'URL non è l'host del gateway stesso. Un endpoint indirizzato al gateway mantiene il percorso di inoltro e il suo token di sessione.
* Né tu né lo sviluppatore avete configurato [`otelHeadersHelper`](/docs/it/settings-reference#otelheadershelper) in nessuna fonte di impostazioni. Con un helper configurato, ogni segnale rimane sull'inoltro.

L'endpoint che nomini cambia solo dove vanno le esportazioni. Scegli comunque quali segnali esportare affatto con i selettori `OTEL_*_EXPORTER`.

L'endpoint da solo non attiva l'esportazione, quindi imposta anche le variabili che lo fanno, a meno che il gateway non le spinga già:

* Se il gateway già [spinge le variabili di telemetria](#telemetry), coprono l'abilitazione, i selettori e il protocollo, e il tuo endpoint esplicito sostituisce il valore `<public_url>` spinto. Imposta un selettore `OTEL_*_EXPORTER` a `otlp` tu stesso solo per un segnale che nessuna destinazione `forward_to` abilita.
* Se non lo fa, imposta anche `CLAUDE_CODE_ENABLE_TELEMETRY=1`, i selettori `OTEL_*_EXPORTER` e `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`.

Quando lo sviluppatore si firma, o si firma a un gateway diverso, le esportazioni al collettore si fermano e Claude Code elimina ogni batch rimanente piuttosto che inviarlo.

<h4 id="when-a-destination-fails">
  When a destination fails
</h4>

Il gateway non bufferizza, ritenta o archivia telemetria, quindi elimina un'esportazione che non raggiunge una destinazione piuttosto che consegnarla in ritardo. Ogni destinazione ha successo o fallisce da sola, e il client che esporta riceve una risposta di successo comunque, quindi una consegna fallita appare solo nel log del gateway.

Dopo cinque consegne consecutive fallite a una destinazione, il gateway pausa l'inoltro a essa in tratti di 30 secondi, registrando ogni pausa, fino a quando una consegna ha successo. Qualsiasi risposta di errore, timeout o errore di connessione conta come una consegna fallita, tranne `400`, `413`, `415`, `422` e `431`, che significano che il collettore ha rifiutato il payload di quell'esportazione come malformato o troppo grande.

Un payload rifiutato non avanza né ripristina il conteggio dei fallimenti: il gateway continua a inoltrare alla destinazione e registra un avviso che la nomina e lo stato, al primo rifiuto della destinazione e ogni centesimo dopo.

<h3 id="http-tuning">
  HTTP tuning
</h3>

Quattro blocchi facoltativi di primo livello, `access_control`, `limits`, `timeouts` e `rate_limits`, sintonizzano la superficie HTTP. I valori predefiniti si adattano alla maggior parte delle distribuzioni.

| Blocco           | Chiave                                         | Predefinito   | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ---------------- | ---------------------------------------------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | vuoto         | Inbound IP allow/deny per indirizzo client, dopo la risoluzione di `trusted_proxies`. `deny_cidrs` viene controllato per primo; un client che corrisponde viene rifiutato anche se `allow_cidrs` corrisponde anche. Se `allow_cidrs` è non vuoto il gateway è default-deny. `/healthz` e `/readyz` sono esenti da `allow_cidrs`. Quando un proxy attendibile invia una voce `X-Forwarded-For` che non è un indirizzo IP, il client reale è sconosciuto e il gateway registra un avviso una volta nominando cosa controllare. Dove uno dei due elenchi si applica alla richiesta, la rifiuta con `403` e motivo di audit `xff_unparseable`. Dove nessuno dei due lo fa, serve la richiesta e usa l'indirizzo del proxy stesso come IP client per i limiti di velocità per IP e audit. |
| `limits`         | `max_request_bytes`                            | 32 MiB        | Max corpo della richiesta in entrata; le richieste di dimensioni eccessive ottengono `413` prima che il corpo sia bufferizzato. Aumenta per richieste di file o immagini di grandi dimensioni.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `limits`         | `max_request_header_bytes`                     | non impostato | Quando impostato, le intestazioni di dimensioni eccessive restituiscono `431`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `limits`         | `max_url_length`                               | non impostato | Quando impostato, un URL troppo lungo restituisce `414`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000        | Max attesa per le intestazioni di risposta dell'upstream (tempo al primo byte). Il corpo della risposta quindi scorre senza cap di wall-clock. Si applica al percorso upstream diretto di Anthropic; ogni altro provider è limitato dal timeout proprio dell'SDK del provider.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600      | Limite di velocità per IP sull'endpoint di autorizzazione del dispositivo non autenticato. Aumenta per una grande organizzazione dietro un IP di uscita condiviso o NAT. Questi limiti si applicano solo al flusso di accesso con concessione del dispositivo, non all'inferenza `/v1/messages`. Vedi [User-code brute-force resistance](/docs/it/claude-apps-gateway-deploy#user-code-brute-force-resistance).                                                                                                                                                                                                                                                                                                                                                                           |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600      | Limite di velocità per IP su invii di `user_code` su `/device`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

Se lasci entrambi gli elenchi `access_control` vuoti, che è il valore predefinito, il gateway serve qualsiasi indirizzo client, quindi solo la tua rete limita chi può raggiungerlo. Questo è importante perché un gateway può spingere [impostazioni gestite](#managed) che eseguono comandi sulle macchine degli sviluppatori.

Mentre `allow_cidrs` è vuoto, il gateway avverte in due posti, senza cambiare come risponde a nessuna richiesta:

* **All'avvio**: un avviso nel log operativo consiglia di consentire solo gli intervalli privati `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `100.64.0.0/10`, `127.0.0.0/8`, `::1/128` e `fc00::/7`, più qualsiasi altro intervallo interno da cui i tuoi sviluppatori si connettono. Se leghi il gateway a un indirizzo loopback e non imposti né `trusted_proxies` né `public_url`, come nello sviluppo locale, l'avviso non appare.
* **A runtime**: la prima volta che una richiesta arriva da un indirizzo al di fuori di quegli intervalli privati, il gateway registra un avviso e emette un evento di audit [`access.public_client`](/docs/it/claude-apps-gateway-deploy#logs) che porta l'IP client. Entrambi si attivano una volta per processo. Gli indirizzi link-local, `169.254.0.0/16` e `fe80::/10`, non contano come pubblici. Il gateway risponde a `/healthz` e `/readyz` prima che questo controllo venga eseguito, quindi i probe di salute da intervalli pubblici non lo attivano.

Entrambi i segnali usano l'indirizzo client come il gateway lo risolve. Se un load balancer, port-forward o tunnel inoltra il traffico e non è elencato in `listen.trusted_proxies`, il gateway vede l'indirizzo del relay, che di solito è privato, quindi né l'avviso a runtime né un elenco di autorizzazione privato lo cattura.

Dietro un tale front end, imposta [`listen.trusted_proxies`](#listen) per primo in modo che il gateway veda gli indirizzi client reali, e mantieni il gateway e tutto davanti ad esso irraggiungibile da internet pubblico indipendentemente.

<h2 id="complete-example">
  Esempio completo
</h2>

Questo config di riferimento completo esercita ogni sezione principale; i blocchi di [sintonizzazione HTTP](#http-tuning) mantengono i loro valori predefiniti. Copiatelo, eliminate ciò che non vi serve e riempite i vostri valori. La configurazione nella [Guida rapida](/docs/it/claude-apps-gateway#quickstart) è una versione minima di questa.

```yaml gateway.yaml theme={null}
# Run with:
#   claude gateway --config gateway.yaml
#
# Operational log verbosity is controlled by the CLAUDE_GATEWAY_LOG_LEVEL
# environment variable (debug | info | warn | error; default info). debug
# also logs the claim names in each id_token, for groups_claim diagnosis.
# It does not affect audit events, which are always emitted.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omit the tls block when running behind a TLS-terminating ingress.
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # Required when the issuer is the Okta org server, whose id_tokens
  # can omit email and groups; the gateway fills them from /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta emits groups only when the `groups` scope is requested and the
  # app's groups claim filter allows them. The contractors policy below
  # matches on groups, so the scope is requested here.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Entra app roles: use `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5

# Enables /v1/organizations/spend_limits (mirrors the Anthropic Admin API)
# and per-developer spend enforcement on /v1/messages. Omit to disable.
# Caps themselves are set via the admin API, not here.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

# Meter at contracted rates instead of USD list price. Requires admin: or a
# managed: policy. With managed:, the same rates also go to signed-in clients.
# Rates below are placeholders, not real contract prices.
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # Constrain the Default picker option to availableModels instead of
        # the tier default, so contractors don't get a 400 on the default.
        enforceAvailableModels: true
        # allow auto-approves these tools; it does not block the rest.
        # Add deny rules to restrict tools.
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  Impostazioni gestite lato client
</h2>

Tutto quanto sopra configura il server gateway. Puntate le macchine degli sviluppatori al gateway separatamente, su ogni dispositivo, attraverso le [impostazioni gestite](/docs/it/managed-settings) di Claude Code. Il gateway non può inviare le chiavi di accesso stesso, perché sono loro che indicano al client dove si trova il gateway.

Per la CLI, impostate queste chiavi nel file `managed-settings.json` per ogni sistema operativo. Le due chiavi di accesso instradano il `/login` di ogni sviluppatore al vostro gateway:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"` mantiene il funzionamento della consegna della lista di egress di Claude Desktop alle sue sessioni Claude Code incorporate; [Deliver policy to Claude Desktop sessions](/docs/it/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) spiega il meccanismo e dove deve trovarsi l'opt-in.

Distribuite il file `managed-settings.json` a ogni dispositivo, tipicamente tramite la vostra piattaforma MDM. Il percorso del file differisce per piattaforma. Consultate [dove ogni meccanismo memorizza la policy](/docs/it/managed-settings#where-each-mechanism-stores-the-policy).

Per impostazione predefinita, una policy del registro su Windows o un plist di preferenze gestite su macOS sostituisce il file `managed-settings.json` piuttosto che unirsi ad esso, ad eccezione delle [chiavi di eccezione e dei controlli tra fonti sopra](#precedence-with-other-managed-sources). Tutte e tre le chiavi in questo frammento seguono la regola della fonte con priorità più alta, quindi i fleet che distribuiscono la policy tramite Group Policy o profili di configurazione devono inserire tutte e tre in quel meccanismo invece.

Per Claude Desktop, impostate la chiave `bootstrapUrl` nella propria [configurazione gestita](https://claude.com/docs/third-party/claude-desktop/configuration) di Claude Desktop su `<listen.public_url>/user/bootstrap`. Il flusso di accesso e la policy per gruppo corrispondono quindi a quelli della CLI una volta che una policy si attiva lato server con una chiave `desktop`; senza l'opt-in, `/user/bootstrap` restituisce 404. Consultate [Claude Desktop overlay](#claude-desktop-overlay) per la metà lato server.

Claude Code rispetta [`forceLoginGatewayUrl`](/docs/it/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/it/settings-reference#gatewayinternalnetworks), e il valore `"gateway"` di [`forceLoginMethod`](/docs/it/settings-reference#forceloginmethod) solo da una fonte gestita sulla macchina: `managed-settings.json`, il plist macOS o il registro HKLM di Windows, o un policy helper. Uno sviluppatore che li imposta nel proprio `~/.claude/settings.json` non ha alcun effetto, e nemmeno impostarli nel payload del gateway.

<h2 id="related">
  Correlati
</h2>

* [Panoramica del gateway delle app Claude](/docs/it/claude-apps-gateway): guida rapida e connessione dello sviluppatore
* [Guida alla distribuzione](/docs/it/claude-apps-gateway-deploy): configurazione IdP, immagine del contenitore, Kubernetes e Cloud Run e operazioni
* [Limiti di spesa](/docs/it/claude-apps-gateway-spend-limits): cap per sviluppatore e API Admin
