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
* [`models`](#models) e `auto_include_builtin_models`: elenco di modelli curato dall'amministratore e ID per upstream
* [`managed`](#managed): politiche di impostazioni gestite per gruppo IdP
* [`telemetry`](#telemetry): inoltro OTLP al vostro stack di osservabilità
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): IP allow/deny, limiti di dimensione delle richieste, time-to-first-byte upstream e limiti di accesso per IP

<h2 id="secret-expansion">
  Espansione dei segreti
</h2>

Non scrivete segreti come `client_secret`, `jwt_secret` o `postgres_url` direttamente in `gateway.yaml`. Fate riferimento ad essi con uno dei moduli sottostanti e il gateway risolve il valore all'avvio da una variabile di ambiente o da un file:

| Modulo          | Si risolve in                                                     | Usare per                                                                        |
| --------------- | ----------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `${VAR}`        | La variabile di ambiente `VAR`. L'avvio fallisce se non definita. | Variabili di ambiente del contenitore, AWS Secrets Manager tramite iniezione env |
| `${file:/path}` | Contenuti del file, ritagliati                                    | Montaggi di volumi Kubernetes Secret, Vault Agent, SOPS                          |

<h2 id="required-sections">
  Sezioni obbligatorie
</h2>

<h3 id="listen">
  `listen`
</h3>

Il blocco `listen` controlla dove il gateway serve: l'indirizzo di binding e la porta, l'origine visibile esternamente e la terminazione TLS facoltativa.

| Campo                  | Obbligatorio    | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------- | --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `host`                 | No              | Indirizzo di binding. Predefinito `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `port`                 | No              | Porta di binding. Predefinito `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `public_url`           | Dietro un proxy | L'origine `https://` visibile esternamente, utilizzata per costruire il `redirect_uri` dell'IdP e i metadati di scoperta. Obbligatorio dietro qualsiasi proxy che termina TLS come un ALB, Ingress o Cloud Run, perché il gateway non si fida degli header `X-Forwarded-*` quando costruisce la sua stessa origine; sono falsificabili dal client. `trusted_proxies` di seguito governa solo la risoluzione dell'IP del client. Obbligatorio anche per abilitare la [telemetria](#telemetry), perché il gateway costruisce l'endpoint OTLP che spinge ai client da questo URL. |
| `tls.cert` / `tls.key` | No              | Percorsi PEM se il gateway termina TLS stesso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `trusted_proxies`      | No              | CIDR o IP dei load balancer davanti al gateway. Quando impostato, il gateway si fida di `X-Forwarded-For` solo da questi peer e registra l'IP client reale per il rate limiting per IP e l'audit. Equivalente a nginx `set_real_ip_from`.                                                                                                                                                                                                                                                                                                                                      |

<h3 id="oidc">
  `oidc`
</h3>

Il blocco `oidc` connette il gateway al vostro provider di identità e decide chi può accedere. Nomina l'emittente e il client OAuth, mappa i claim che portano email e gruppi e limita l'accesso per dominio email o gruppo.

OpenID Connect (OIDC) è il protocollo SSO che il gateway utilizza con il vostro provider di identità; vedere [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup) per ciò che registrare sul lato IdP.

| Campo                           | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ------------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Sì           | Base di scoperta OIDC. Deve servire la scoperta su `/.well-known/openid-configuration`. Usate HTTPS in produzione; il gateway accetta un emittente `http://`. Un emittente loopback come `http://localhost:8081` è rifiutato dalla [guardia SSRF](/docs/it/claude-apps-gateway-deploy#threat-model-summary) a meno che `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` non sia impostato nell'ambiente del gateway.                                                                                                                                                                                                                                                                                                                          |
| `client_id` / `client_secret`   | Sì           | Dalla vostra registrazione del client OAuth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `allowed_email_domains`         | No           | Rifiutate i token id i cui claim `email` non sono in uno di questi domini, case-insensitive. Difesa in profondità contro la misconfiguration dell'IdP multi-tenant. Indipendentemente da questa impostazione, un id\_token il cui claim `email_verified` è esplicitamente `false` è sempre rifiutato.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowed_groups`                | No           | Limitate l'accesso ai membri di questi gruppi IdP, abbinati rispetto a `groups_claim`. Un utente in un dominio email consentito ma in nessuno di questi gruppi è rifiutato. Richiede che l'IdP emetta il claim dei gruppi.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `groups_claim`                  | No           | Quale claim id\_token porta l'appartenenza al gruppo. Predefinito `groups`. Microsoft Entra emette i ruoli dell'app sotto `roles`. Accetta una chiave flat o un JSON Pointer RFC 6901 come `/resource_access/gateway/roles` per i claim annidati.                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `google_groups`                 | No           | Cercate i gruppi dell'utente che ha effettuato l'accesso tramite l'API Google Workspace Admin SDK Directory, perché il token id di Google non porta alcun claim di gruppi. Impostate `service_account_json_path` su un file di chiave dell'account di servizio con delega a livello di dominio sull'ambito `https://www.googleapis.com/auth/admin.directory.group.readonly` e `admin_email` su un amministratore di Workspace che l'account di servizio rappresenta; l'API Directory richiede un soggetto amministratore reale. Gli indirizzi email dei gruppi di ogni utente diventano il loro claim di gruppi, quindi `allowed_groups` e `managed.policies.match.groups` corrispondono agli indirizzi email dei gruppi. |
| `email_claim`                   | No           | Quale claim id\_token porta l'email dell'utente. Predefinito `email`. Alcuni IdP, come ADFS ed Entra B2C, emettono `upn` o `preferred_username` invece. Accetta una chiave flat, un JSON Pointer o un elenco di chiavi di fallback dove viene utilizzata la prima chiave presente.                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `scopes`                        | No           | Override completo degli ambiti OIDC che il gateway richiede. Predefinito `[openid, profile, email, offline_access]`. Impostate quando il vostro IdP rifiuta gli ambiti che non riconosce o richiede un ambito personalizzato per emettere gruppi o email. Deve includere `openid`. Eliminare `offline_access` disabilita i token di aggiornamento, quindi gli sviluppatori rieseguono l'accesso al browser ogni `session.ttl_hours`. Vedere [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup) per ricette di ambito per IdP come il flusso di token di aggiornamento di Google.                                                                                           |
| `extra_auth_params`             | No           | Parametri di query extra aggiunti alla richiesta di autorizzazione dell'IdP, verbatim. Questo è il meccanismo di override per il comportamento specifico dell'IdP, come `access_type: offline` per i token di aggiornamento di Google, `domain_hint` per alcuni tenant Entra o `acr_values` per i flussi step-up. Non può sovrascrivere i parametri del protocollo gestiti dal gateway: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` e `client_id`.                                                                                                                                                                                                                                  |
| `userinfo_fallback`             | No           | Quando l'id\_token omette email o gruppi, recuperateli da `/userinfo`. Necessario per i token di accesso leggeri di Keycloak, il server org di Okta e i token minimi di ADFS. L'id\_token rimane autorevole; userinfo riempie solo i vuoti. Predefinito `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `use_pkce`                      | No           | Inviate una sfida PKCE (S256) sulla richiesta di autorizzazione. Predefinito `true`. Impostate `false` solo se il vostro IdP rifiuta PKCE per questo client confidenziale.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `clock_skew_seconds`            | No           | Tollerare la deriva dell'orologio quando si convalidano i claim temporali dell'id\_token. Predefinito `0`, che è rigoroso. Aumentate se vedete errori "token scaduto / non ancora valido" subito dopo l'accesso a causa della deriva dell'orologio host/IdP.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `token_endpoint_auth_method`    | No           | Override del metodo di autenticazione dell'endpoint del token. Accetta `client_secret_basic` o `client_secret_post`. Negoziato automaticamente per impostazione predefinita.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `id_token_signed_response_alg`  | No           | Algoritmo di firma id\_token previsto. Predefinito `RS256`. Impostate per gli IdP che firmano con ES256, PS256 o EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `additional_authorized_parties` | No           | Valori `azp` extra da accettare oltre a `client_id`, per i flussi di broker e scambio di token di Keycloak                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `discovery_url`                 | No           | Recuperate il documento di scoperta da questo URL invece di derivarlo da `issuer`, per gli IdP dietro un proxy che riscrive l'host dell'emittente. Il percorso deve contenere `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `form_action_origins`           | No           | Origini aggiuntive per la direttiva `Content-Security-Policy: form-action` della pagina `/device`. Il gateway consente già `'self'` e l'origine dell'`authorization_endpoint` scoperta, ma Chrome applica `form-action` all'intera catena di reindirizzamento. Se il vostro IdP reindirizza attraverso un secondo host, come Azure AD federato ad ADFS, Okta hub-spoke o un intercettore SSO aziendale, elencate ogni origine attraverso cui la richiesta di autorizzazione può reindirizzare.                                                                                                                                                                                                                            |
| `ca_cert_pem`                   | No           | Certificato CA PEM che sostituisce l'archivio di fiducia del sistema solo per le richieste IdP. Usate per Keycloak o Dex dietro PKI aziendale.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

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

| Campo             | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ----------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | Sì           | URL `postgres://` o `postgresql://`. Obbligatorio: il rendezvous della concessione del dispositivo, dove il callback del browser scrive e la CLI di polling legge, ha bisogno di uno stato cross-replica. Il gateway esegue le sue migrazioni dello schema all'avvio, quindi il ruolo ha bisogno di `CREATE TABLE` sullo schema di destinazione. Se la vostra politica di sicurezza proibisce DDL dal ruolo dell'applicazione, eseguite le migrazioni con un ruolo admin, inizialmente e di nuovo ogni volta che una nuova versione spedisce migrazioni, e concedete al ruolo dell'app `SELECT, INSERT, UPDATE, DELETE` sulle tabelle del gateway. Vedere [Aggiornamenti](/docs/it/claude-apps-gateway-deploy#upgrades) e [Postgres](/docs/it/claude-apps-gateway-deploy#postgres). |
| `username`        | No           | Sovrascrive l'utente in `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `password`        | No           | Credenziale del database. Impostatela qui piuttosto che in `postgres_url` in modo che la credenziale rimanga fuori dall'URL. Accetta qualsiasi carattere e ha la precedenza sulle credenziali dell'URL.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `max_connections` | No           | Dimensione del pool di connessioni Postgres per replica. Predefinito `5`, che è conservativo e amichevole ai database condivisi. Con i [limiti di spesa](#admin) abilitati, il percorso caldo esegue alcune operazioni per richiesta di inferenza, quindi aumentatelo per un database dedicato sotto carico e mantenete repliche × questo sotto il `max_connections` del database.                                                                                                                                                                                                                                                                                                                                                                                        |

Per lo sviluppo locale, puntate `postgres_url` a un contenitore Postgres usa e getta, ad esempio `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` è un elenco ordinato. Il gateway inoltra l'inferenza al primo upstream che risolve il modello richiesto. Su `5xx`, `429`, `401`, `403`, `404` o timeout esegue il failover al successivo; altri `4xx` no, perché questi errori sono attribuibili alla richiesta piuttosto che all'upstream. Un `401` o `403` significa che la credenziale del gateway stesso non ha funzionato contro quell'upstream, e un `404` significa che quell'upstream non serve il modello richiesto, quindi un upstream successivo nell'elenco può ancora servirlo.

Il failover su `404` richiede gateway v2.1.198 o successivo. Le versioni precedenti hanno restituito il primo `404` al client anche quando un upstream successivo nell'elenco serviva il modello.

Più upstream dello stesso provider devono impostare un `name:` distinto.

I client Amazon Bedrock, Claude Platform on AWS, Google Cloud Agent Platform e Microsoft Foundry sono costruiti una volta all'avvio e i loro SDK aggiornano le credenziali internamente, quindi la rotazione delle credenziali cloud non richiede un riavvio. Le chiavi API Anthropic statiche e i bearer sono letti all'avvio; vedere [API Anthropic](#anthropic-api).

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

| Configurazione     | Come                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Autorizzazioni IAM | Concedete al principale del gateway `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` sia sugli ARN del profilo di inferenza che sugli ARN del modello di fondazione sottostante. Per il catalogo integrato nelle regioni US: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` e `arn:aws:bedrock:*::foundation-model/anthropic.*`. |
| Accesso al modello | Nella console Bedrock, per regione, richiedete e abilitate l'accesso al modello per i modelli Claude che desiderate. I profili di inferenza cross-region (`us.anthropic.*`) richiedono l'accesso al modello in ogni regione che il profilo copre.                                                                                                                      |
| EKS (IRSA)         | Create un ruolo IAM con la politica sopra e una politica di fiducia per il provider OIDC del vostro cluster limitato all'account di servizio del gateway. Annotate l'account di servizio con `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` lo raccoglie.                                                                           |
| ECS / EC2          | Allegate il ruolo IAM alla definizione dell'attività o al profilo dell'istanza. `auth: {}` lo raccoglie.                                                                                                                                                                                                                                                               |
| Altrove            | Passate le credenziali tramite le variabili env `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN`, o impostatele esplicitamente in `auth:` con l'espansione `${VAR}`                                                                                                                                                                                  |
| Regione            | `region:` è la regione dell'endpoint API. I profili di inferenza cross-region instradano attraverso la geo (US, EU, APAC) indipendentemente da quale scegliete. Per le regioni non-US o gli ARN di throughput provisioned, aggiungete un blocco [`models:`](#models) con gli ID per upstream corretti.                                                                 |

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

`429` è capacità per upstream, quindi l'esaurimento del throughput provisioned (PT) esegue il failover a on-demand. `404` è disponibilità del modello per upstream, quindi un upstream che non ha abilitato un modello non blocca un upstream successivo che lo serve. Un upstream che non può risolvere il modello richiesto viene saltato senza un round-trip di rete.

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

# Per-upstream model IDs are keyed on the upstream's `name:`; an upstream
# without a `name:` defaults to its provider string (e.g. `bedrock`). Any
# upstream not listed for a model is skipped, which is how you route a model
# to provisioned throughput while everything else stays on-demand.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Leva                            | Come                                                                                                                                                                                                                                                |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Regioni diverse                 | Un upstream Bedrock per regione, ciascuno con la sua `region:`. Con [`auto_include_builtin_models: true`](#models) i profili di inferenza cross-region instradano automaticamente; per distribuzioni fissate per regione usate un blocco `models:`. |
| Account diversi                 | Un upstream Bedrock per account, ciascuno con le sue credenziali in `auth:`. La catena predefinita (`auth: {}`) usa l'identità del pod; per un secondo account, impostate credenziali esplicite o un token bearer.                                  |
| Throughput provisioned          | Mappate il modello all'ARN di throughput provisioned in `models:` per il nome di quell'upstream. Gli altri upstream mantengono l'ID on-demand, quindi la capacità PT è esaurita prima del failover.                                                 |
| Endpoint VPC / FIPS             | Impostate `base_url:` sull'upstream al vostro URL di endpoint VPC o FIPS                                                                                                                                                                            |
| Instradamento scoped al modello | Omettete un upstream dalla mappa `upstream_model:` di un modello e quell'upstream viene saltato per quel modello. Ad esempio, instradare Opus al throughput provisioned e Sonnet e Haiku a on-demand.                                               |

Il failover tra provider cloud o all'API Anthropic diretto cambia quale accordo, geografia e altri termini governano la richiesta.

La CLI applica lo stesso feature gating ai gateway indipendentemente da quale upstream serve una data richiesta, quindi il failover non invia un campo del corpo che un upstream rifiuterebbe.

<h2 id="optional-sections">
  Sezioni facoltative
</h2>

<h3 id="admin">
  `admin`
</h3>

Facoltativo. Abilita `/v1/organizations/spend_limits`, che rispecchia l'API Admin pubblica di Anthropic, e l'applicazione della spesa per sviluppatore su `/v1/messages`. Vedere [Limiti di spesa](/docs/it/claude-apps-gateway-spend-limits) per come i cap sono impostati e applicati; questa sezione copre le chiavi `gateway.yaml` che attivano la funzione e la sintonizzano.

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

| Campo                     | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                       |
| ------------------------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | No           | Array di `{id, key}`. Un `x-api-key` che corrisponde a uno di questi può elencare, impostare ed eliminare i limiti di spesa. I valori delle chiavi devono essere almeno 32 caratteri; gli `id` devono essere univoci tra `read_keys` e `write_keys`.                                                              |
| `read_keys`               | No           | Array di `{id, key}`. Sola lettura: ogni endpoint `GET`, incluso l'elenco dei cap, il recupero di uno per ID e la lettura di [`/effective`](/docs/it/claude-apps-gateway-spend-limits#%2Feffective) e [`/audit`](/docs/it/claude-apps-gateway-spend-limits#%2Faudit).                                                       |
| `admin_groups`            | No           | Nomi di gruppi IdP. Un gateway JWT il cui claim `groups` include uno di questi ha accesso admin completo, lettura e scrittura, e audit come `oidc:<sub>`. Usate questo per gli amministratori umani; usate le chiavi API per le macchine.                                                                         |
| `blocked_message`         | No           | Aggiunto verbatim al `429 billing_error` che uno sviluppatore bloccato vede. Scrivete l'intera istruzione, come un URL o un canale Slack. Non impostato, l'errore è `spend limit reached`.                                                                                                                        |
| `audit_retention_days`    | No           | Predefinito `365`. Le righe `admin_audit` più vecchie vengono spazzate.                                                                                                                                                                                                                                           |
| `spend_retention_months`  | No           | Predefinito `13`. Le righe del contatore `spend` più vecchie di questo vengono spazzate. Il predefinito mantiene un anno completo più il mese parziale corrente per il reporting anno su anno.                                                                                                                    |
| `identity_retention_days` | No           | Predefinito `90`. TTL last-seen per le righe `principal_emails`, che contengono l'email, il nome visualizzato e i gruppi di ogni sviluppatore (PII). Deliberatamente più breve della conservazione della spesa in modo che un'identità deprovisioned invecchi mentre i suoi contatori di spesa anonimi rimangono. |
| `group_limit_mode`        | No           | `min` (predefinito) o `max`. Quando uno sviluppatore è in più gruppi con cap, `min` applica il più restrittivo e `max` il meno. Usato sia dall'applicazione che da `/effective`.                                                                                                                                  |

<h3 id="enforcement">
  `enforcement`
</h3>

Il blocco `enforcement` controlla come i controlli dei limiti di spesa si comportano quando l'archivio non è disponibile.

| Campo                  | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                        |
| ---------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | No           | Predefinito `false`. L'applicazione della spesa fallisce aperta su un'interruzione di Postgres, quindi l'inferenza rimane attiva. Impostate `true` per fallire chiuso: gli sviluppatori over-cap sono bloccati, ma lo è anche chiunque altro se l'archivio non è raggiungibile. Non ha effetto senza un blocco [`admin:`](#admin). |

<h3 id="models">
  `models`
</h3>

Il blocco `models` è un elenco di modelli curato dall'amministratore facoltativo, servito su `/v1/models` e utilizzato per tradurre gli ID dei modelli per upstream. È obbligatorio per le regioni Bedrock non-US, gli ARN di throughput provisioned di Bedrock e i nomi di distribuzione di Foundry.

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

<h3 id="managed">
  `managed`
</h3>

Il blocco `managed` definisce le politiche di accesso basate sui ruoli chiave sui gruppi IdP o sul dominio email. Le politiche vengono valutate in ordine; la prima corrispondenza viene selezionata, quindi unita alla base catch-all `match: {}` descritta di seguito. Vengono servite per utente su `GET /managed/settings` con caching ETag/304.

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

Un catch-all `match: {}`, convenzionalmente elencato per ultimo, è trattato come uno strato di base. Ogni altra politica eredita qualsiasi chiave che non imposta dal catch-all, quindi le voci per ruolo hanno solo bisogno di elencare ciò che differisce dall'impostazione predefinita dell'organizzazione. Le regole di unione dipendono dal tipo di chiave:

* **Allow-list**: `availableModels` e `permissions.allow`. L'elenco di una politica specifica sostituisce completamente quello della base.
* **Deny-list e array di hook**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` e ogni array di tipo di evento `hooks`. Questi prendono l'unione di base e politica, quindi un hook di deny o audit a livello di organizzazione non può essere accidentalmente eliminato da un override per ruolo.
* **Chiavi di tipo record**: `env`, `modelOverrides` e `skillOverrides`. Questi shallow-merge, quindi un blocco `env` per ruolo sovrascrive le chiavi che imposta e eredita il resto dalla base.

`availableModels` è anche applicato lato server su `/v1/messages`, quindi un modello negato restituisce `400` indipendentemente da ciò che il client invia.

| Matcher                                             | Comportamento                                                                                                                                       |
| --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Corrisponde a ogni utente autenticato. Iniziate con uno di questi e aggiungete politiche scoped al gruppo sopra di esso in seguito.                 |
| `match: { groups: [a, b] }`                         | Corrisponde se il claim `groups` del JWT contiene uno dei gruppi elencati. Case-sensitive: i gruppi devono corrispondere al casing esatto dell'IdP. |
| `match: { email_domain: example.com }`              | Corrisponde alla parte dopo l'ultimo `@` nel claim `email` del JWT, case-insensitive. Accetta un dominio per politica.                              |
| `match: { groups: [a], email_domain: example.com }` | Entrambe le condizioni devono corrispondere                                                                                                         |

Un utente autenticato che non corrisponde a nessuna politica ottiene i valori predefiniti del gateway, il che significa ogni modello nel catalogo e nessuna impostazione gestita. Aggiungete un catch-all `match: {}` per ultimo se desiderate una politica predefinita garantita.

<Note>
  Il gateway non mantiene alcuna directory utente propria. Autorizza ogni richiesta dal token IdP dell'utente, leggendo l'appartenenza al gruppo dal claim `groups` del token e valutando le politiche rispetto ad esso. Non c'è roster da enumerare e nessun account da pre-creare, e quindi nessun endpoint SCIM, perché non c'è nulla per SCIM da sincronizzare.

  Eseguite la gestione del ciclo di vita dell'utente e del gruppo alla fonte della verità, che è il provisioning SCIM nativo del vostro IdP o una piattaforma di governance dell'identità dedicata. L'appartenenza e il deprovisioning governati lì fluiscono nel gateway automaticamente attraverso il token. Se desiderate il provisioning SCIM degli account Claude stessi, questa è una capacità di [Claude for Enterprise](/docs/it/admin-setup).

  Due orologi di propagazione si applicano:

  * **Contenuti della politica**: modificare una politica e ridistribuire raggiunge i client connessi al loro prossimo sondaggio di impostazioni gestite, entro un'ora
  * **Appartenenza al gruppo**: cambiare l'appartenenza al gruppo di un utente cambia quale politica li corrisponde. Questo ha effetto al prossimo re-mint della sessione, il che significa il prossimo aggiornamento silenzioso, limitato da `session.ttl_hours`.
</Note>

<h4 id="what-goes-in-cli">
  Cosa va in `cli`
</h4>

Ogni valore `cli` è un documento `managed-settings.json` completo di Claude Code, lo stesso schema che distribuireste tramite MDM o `/etc/claude-code/managed-settings.json`, espresso qui come YAML. La CLI applica il documento consegnato al livello gestito, sopra le impostazioni dell'utente e del progetto.

Il gateway convalida ogni documento rispetto allo schema delle impostazioni della CLI all'avvio, quindi una chiave di primo livello non riconosciuta o una chiave riconosciuta con un valore malformato fallisce l'avvio con un errore che nomina ogni chiave offensiva. Le parti deliberatamente aperte dello schema accettano ancora valori arbitrari, perché i client più recenti possono riconoscere voci che lo schema del gateway non riconosce. Queste chiavi aperte sono `env`, `pluginConfigs` e chiavi annidate sotto `permissions`.

Poiché la convalida utilizza lo schema fornito con la versione installata del gateway, mettere una chiave di impostazioni di primo livello introdotta da una versione più recente di Claude Code nella configurazione gestita richiede l'aggiornamento del gateway per primo. Smoke-test una nuova politica su un client prima di distribuirla.

Il riferimento completo della chiave è in [Impostazioni di Claude Code](/docs/it/settings#available-settings). Le chiavi che gli operatori raggiungono per primi:

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

| Chiave                                     | Applicata da  | Effetto                                                                                                                                                                                                                          |
| ------------------------------------------ | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Gateway + CLI | Allowlist di modelli. Anche controllato su `/v1/messages`, quindi un client patchato non può bypassarlo.                                                                                                                         |
| `permissions.allow` / `.deny`              | CLI           | Regole di strumenti e comandi. Vedere [Autorizzazioni](/docs/it/permissions).                                                                                                                                                         |
| `permissions.disableBypassPermissionsMode` | CLI           | Impostate su `disable` per bloccare [`bypassPermissions`](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode), la modalità che auto-approva ogni chiamata di strumento, e il flag `--dangerously-skip-permissions` |
| `allowManagedPermissionRulesOnly`          | CLI           | Quando `true`, le regole di autorizzazione dell'utente e del progetto vengono ignorate; si applicano solo le regole da questo documento                                                                                          |
| `env`                                      | CLI           | Variabili di ambiente unite nel processo CLI. Usate per telemetria, auto-update e override dei nomi dei modelli.                                                                                                                 |
| `hooks`                                    | CLI           | [Hook](/docs/it/hooks) a livello di organizzazione                                                                                                                                                                                    |

Poiché queste impostazioni arrivano sulla rete, la CLI mostra a ogni sviluppatore una finestra di dialogo di approvazione della sicurezza una tantum prima di applicare qualsiasi cosa che possa eseguire un comando shell o alterare dove va il traffico. La finestra di dialogo copre:

* `hooks`
* Variabili `env` che non sono nell'elenco sicuro integrato della CLI
* Impostazioni di esecuzione shell come `apiKeyHelper` e `statusLine`
* Contenuto CLAUDE.md gestito

L'elenco sicuro determina quali variabili `env` si applicano senza approvazione:

* **Nell'elenco sicuro**: variabili di auto-update e nome del modello
* **Non nell'elenco sicuro**: variabili proxy, variabili base-URL e `OTEL_EXPORTER_OTLP_ENDPOINT`

La configurazione della [telemetria](#telemetry) del gateway spinge `OTEL_EXPORTER_OTLP_ENDPOINT`, quindi l'impostazione di `telemetry.forward_to` attiva la finestra di dialogo su ogni client interattivo. Le esecuzioni non interattive con il flag `-p` saltano la finestra di dialogo e applicano le impostazioni senza approvazione. La finestra di dialogo protegge la macchina dello sviluppatore da un gateway compromesso o ostile, non l'organizzazione dallo sviluppatore, quindi il salto `-p` è intenzionale piuttosto che un gap.

Se uno sviluppatore rifiuta, Claude Code esce piuttosto che applicare la politica. Spingere un nuovo hook o una variabile env non sicura a una politica ampia significa quindi un prompt di approvazione all'avvio successivo di ogni sviluppatore corrispondente.

La chiave `cli` era denominata `settings` nelle versioni precedenti. Questo spelling è ancora accettato come alias, ma le nuove distribuzioni dovrebbero usare `cli`.

<h4 id="precedence-with-other-managed-sources">
  Precedenza con altre fonti gestite
</h4>

Se un dispositivo ha anche un `managed-settings.json` locale o una politica consegnata da MDM, le fonti gestite non si uniscono. La fonte con priorità più alta fornisce tutte le impostazioni della politica, classificate in questo ordine con priorità più alta per primo:

1. L'[helper della politica](/docs/it/settings#compute-managed-settings-with-a-policy-helper)
2. Impostazioni consegnate dal gateway
3. MDM, tramite il registro HKLM su Windows o un plist su macOS
4. Il file `managed-settings.json`
5. Il registro HKCU, solo su Windows

Gli host di incorporamento possono fornire politica tramite l'opzione SDK `managedSettings`. Viene ignorata per impostazione predefinita e si applica solo quando una fonte gestita acconsente con [`parentSettingsBehavior: "merge"`](/docs/it/settings#available-settings), filtrata in modo che possa stringere la politica ma non allentarla.

L'eccezione è un piccolo insieme di chiavi cross-source, onorate quando qualsiasi fonte admin le imposta; il livello HKCU scrivibile dall'utente è escluso:

* `sandbox.network.allowManagedDomainsOnly` e `sandbox.filesystem.allowManagedReadPathsOnly`: quando bloccate, gli allowlist corrispondenti vengono uniti tra le fonti
* [`allowAllClaudeAiMcps`](/docs/it/settings#available-settings): override di allow-only per l'allowlist del server MCP claude.ai
* `sandbox.bwrapPath` e `sandbox.socatPath`: percorsi del file system ai binari helper [sandbox](/docs/it/sandboxing)
* [`forceRemoteSettingsRefresh`](/docs/it/server-managed-settings): blocca l'avvio fino a quando le impostazioni gestite remote non vengono recuperate di recente, quindi una politica MDM o file che la imposta è onorata anche quando un payload remoto memorizzato nella cache che manca la chiave è la fonte con priorità più alta

Ogni altra chiave, inclusi `allowManagedPermissionRulesOnly` e `disableBypassPermissionsMode`, proviene solo dalla fonte con priorità più alta. Vedere [Precedenza delle impostazioni](/docs/it/settings#settings-precedence) per la stessa regola nella pagina delle impostazioni.

Le politiche del gateway si applicano a ogni invocazione di Claude Code sulla macchina, incluse le esecuzioni non interattive `claude -p` e le sessioni generate dall'Agent SDK. Se il gateway non è raggiungibile all'avvio, le sessioni firmate escono con un errore piuttosto che eseguire senza la loro politica.

<Warning>
  `mcpServers` dentro il blocco `cli` di una politica è rifiutato all'avvio del gateway. La distribuzione MCP per gruppo non è disponibile; distribuire i server MCP tramite il `managed-mcp.json` basato su file su ogni dispositivo o lasciare che gli sviluppatori li aggiungano localmente.
</Warning>

<h3 id="telemetry">
  `telemetry`
</h3>

La CLI invia OpenTelemetry Protocol (OTLP) su metriche, log e, quando abilitato, tracce HTTP al gateway, che le inoltra verbatim a ogni destinazione configurata. Vedere [Monitoraggio dell'utilizzo](/docs/it/monitoring-usage) per le metriche e gli eventi che la CLI emette.

La CLI timbra ogni esportazione con l'identità dell'utente autenticato, letta dal JWT emesso dal gateway: gli attributi `user.id`, `user.email` e `user.groups`. L'attribuzione del costo e dell'utilizzo per sviluppatore funziona quindi senza alcuna configurazione lato sviluppatore.

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
  Ogni destinazione acconsente a `metrics`, `logs` e `traces` indipendentemente e il predefinito è solo metriche. I segnali differiscono in sensibilità:

  * **Metriche**: contatori aggregati come conteggi di token, conteggi di richieste e latenza
  * **Log e tracce**: possono portare comandi bash completi, input di strumenti e percorsi di file, coprendo qualsiasi cosa Claude Code fa sulla macchina di uno sviluppatore

  Abilitate log e tracce solo su destinazioni con i controlli di accesso e la politica di conservazione che i dati garantiscono.
</Warning>

La telemetria è disattivata nella CLI per impostazione predefinita. La configurazione di `telemetry.forward_to` insieme a `listen.public_url` la attiva. Il gateway spinge cinque variabili env a ogni client connesso tramite `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER=otlp`
* `OTEL_LOGS_EXPORTER=otlp`
* `OTEL_TRACES_EXPORTER=otlp`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`

L'endpoint spinto è costruito dall'URL pubblico, quindi le metriche e i log non hanno bisogno di alcuna configurazione OTEL da sviluppatori o politiche. La configurazione spinta viene applicata al livello gestito, sovrascrivendo le variabili `OTEL_*` che uno sviluppatore imposta localmente.

Le [tracce](/docs/it/monitoring-usage#traces-beta) richiedono inoltre `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` su ogni client. Il gateway non spinge quella variabile, quindi impostatela tramite il blocco `env` di una politica gestita. Non è nell'elenco sicuro della CLI, quindi consegnarla tramite una politica è coperto dalla stessa [finestra di dialogo di approvazione della sicurezza](#managed) che l'endpoint OTLP spinto già attiva.

Sia le codifiche OTLP protobuf che JSON vengono inoltrate e qualsiasi backend compatibile con OpenTelemetry funziona come destinazione.

<h3 id="http-tuning">
  Sintonizzazione HTTP
</h3>

Quattro blocchi facoltativi di primo livello, `access_control`, `limits`, `timeouts` e `rate_limits`, sintonizzano la superficie HTTP. I valori predefiniti si adattano alla maggior parte delle distribuzioni.

| Blocco           | Chiave                                         | Predefinito   | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ---------------- | ---------------------------------------------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | vuoto         | Inbound IP allow/deny per indirizzo client, dopo la risoluzione di `trusted_proxies`. `deny_cidrs` viene controllato per primo; un client che corrisponde è rifiutato anche se `allow_cidrs` corrisponde anche. Se `allow_cidrs` è non vuoto il gateway è default-deny. `/healthz` e `/readyz` sono esenti da `allow_cidrs`.                                                                                                |
| `limits`         | `max_request_bytes`                            | 32 MiB        | Corpo della richiesta in entrata massimo; le richieste di dimensioni eccessive ottengono `413` prima che il corpo sia bufferizzato. Aumentate per richieste di file o immagini di grandi dimensioni.                                                                                                                                                                                                                        |
| `limits`         | `max_request_header_bytes`                     | non impostato | Quando impostato, gli header di dimensioni eccessive restituiscono `431`                                                                                                                                                                                                                                                                                                                                                    |
| `limits`         | `max_url_length`                               | non impostato | Quando impostato, un URL troppo lungo restituisce `414`                                                                                                                                                                                                                                                                                                                                                                     |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000        | Attesa massima per gli header di risposta dell'upstream (time to first byte). Il corpo della risposta quindi scorre senza un cap di wall-clock. Si applica al percorso upstream Anthropic diretto; ogni altro provider è limitato dal timeout proprio dell'SDK del provider.                                                                                                                                                |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600      | Limite di velocità per IP sull'endpoint di autorizzazione del dispositivo non autenticato. Aumentate per una grande organizzazione dietro un IP di uscita condiviso o NAT. Questi limiti si applicano solo al flusso di accesso della concessione del dispositivo, non all'inferenza `/v1/messages`. Vedere [Resistenza al brute-force del codice utente](/docs/it/claude-apps-gateway-deploy#user-code-brute-force-resistance). |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600      | Limite di velocità per IP sui invii di `user_code` su `/device`                                                                                                                                                                                                                                                                                                                                                             |

<h2 id="complete-example">
  Esempio completo
</h2>

Questo config di riferimento completo esercita ogni sezione principale; i blocchi di [sintonizzazione HTTP](#http-tuning) mantengono i loro valori predefiniti. Copiatelo, eliminate ciò che non vi serve e riempite i vostri valori. La configurazione nella [Guida rapida](/docs/it/claude-apps-gateway#quickstart) è una versione minima di questa.

```yaml gateway.yaml theme={null}
# Run with:
#   claude gateway --config gateway.yaml
#
# Operational log verbosity is controlled by the CLAUDE_GATEWAY_LOG_LEVEL
# environment variable (info | warn | error; default info). It does not
# affect audit events, which are always emitted.

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

Tutto quanto sopra configura il server del gateway. Puntare le macchine degli sviluppatori ad esso è configurato separatamente, su ogni dispositivo, tramite le [impostazioni gestite](/docs/it/settings#settings-files) di Claude Code. Il gateway non può spingere queste chiavi stesso, perché sono ciò che dice al client dove si trova il gateway.

Per la CLI, impostate entrambe le chiavi nel `managed-settings.json` per OS:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Distribuire quel file a ogni dispositivo, tipicamente tramite la vostra piattaforma MDM. Il percorso del file differisce per piattaforma:

| Piattaforma | Percorso                                                                                                                          |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-settings.json`, o il dominio delle preferenze gestite `com.anthropic.claudecode` |
| Linux e WSL | `/etc/claude-code/managed-settings.json`                                                                                          |
| Windows     | `C:\Program Files\ClaudeCode\managed-settings.json`, o Group Policy tramite il registro HKLM                                      |

`forceLoginGatewayUrl` e il valore `"gateway"` di `forceLoginMethod` sono onorati solo dal livello gestito controllato dall'amministratore. Uno sviluppatore che li imposta nel suo `~/.claude/settings.json` non ha effetto.

<h2 id="related">
  Correlati
</h2>

* [Panoramica del gateway delle app Claude](/docs/it/claude-apps-gateway): guida rapida e connessione dello sviluppatore
* [Guida alla distribuzione](/docs/it/claude-apps-gateway-deploy): configurazione IdP, immagine del contenitore, Kubernetes e Cloud Run e operazioni
* [Limiti di spesa](/docs/it/claude-apps-gateway-spend-limits): cap per sviluppatore e API Admin
