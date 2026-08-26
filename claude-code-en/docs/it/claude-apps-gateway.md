> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gateway di app Claude per Amazon Bedrock, Claude Platform su AWS, Google Cloud e Microsoft Foundry

> Esegui Claude Code attraverso Amazon Bedrock, Claude Platform su AWS, Google Cloud o Microsoft Foundry dietro un gateway auto-ospitato con accesso SSO, accesso ai modelli per gruppo e telemetria OTLP.

<Note>
  Il gateway di app Claude è progettato per le organizzazioni che devono — o preferiscono — instradare l'inferenza attraverso il proprio provider cloud, ad esempio per soddisfare i requisiti di [residenza dei dati](/docs/it/claude-apps-gateway-deploy#compliance-posture). Se non hai questo requisito e desideri accesso ad altre funzionalità come il provisioning SCIM o Claude Code su web e mobile, Claude Enterprise potrebbe essere una scelta migliore. Consulta la pagina di [disponibilità delle funzionalità](/docs/it/feature-availability) per un confronto completo di tutti i metodi di distribuzione.
</Note>

Claude apps gateway è un servizio auto-ospitato che si posiziona tra i client Claude Code dei tuoi sviluppatori e il tuo provider di modelli. Gli sviluppatori accedono con il tuo provider di identità aziendale (IdP) invece di detenere chiavi API o credenziali cloud. Il gateway contiene la credenziale upstream, applica l'accesso ai modelli e le [impostazioni gestite](/docs/it/permissions#managed-settings) per gruppo IdP, e trasmette la telemetria di utilizzo al tuo stack di osservabilità.

È incluso nel binario `claude`, quindi lo stesso eseguibile che esegue Claude Code su un laptop esegue il server gateway con `claude gateway --config gateway.yaml`.

Questa pagina copre:

* [Perché Claude apps gateway](#why-claude-apps-gateway), cosa aggiunge rispetto all'esecuzione della tua, e quando qualcos'altro si adatta meglio
* Una [guida rapida](#quickstart) con [prerequisiti](#prerequisites) che porta un gateway da zero a uno sviluppatore connesso
* [Connessione degli sviluppatori](#connect-developers), inclusa l'impostazione dell'URL del gateway attraverso le impostazioni gestite
* [Disponibilità e limitazioni](#availability-and-limitations) che coprono quali funzionalità di Claude Code funzionano attraverso il gateway e cosa supporta il server

Le pagine complementari approfondiscono. Il [riferimento di configurazione](/docs/it/claude-apps-gateway-config) copre ogni opzione nel file YAML che la guida rapida scrive, e la [guida di distribuzione](/docs/it/claude-apps-gateway-deploy) copre la configurazione per IdP, la distribuzione su Kubernetes e Cloud Run, e le operazioni.

<h2 id="why-claude-apps-gateway">
  Perché Claude apps gateway
</h2>

La [panoramica del gateway](/docs/it/gateways) copre cosa fa un gateway e perché ne eseguiresti uno. Claude apps gateway è il gateway di Anthropic, integrato nel binario `claude` e testato insieme a ogni rilascio di Claude Code, quindi inoltra le intestazioni e i campi di richiesta che Claude Code invia senza che gli operatori mantengano un elenco di autorizzazioni separato. Una volta distribuito, ti offre:

* **Credenziali**: la chiave API upstream o la credenziale cloud vive solo nella tua infrastruttura. Gli sviluppatori si autenticano con SSO aziendale e ricevono token bearer di breve durata, quindi l'offboarding avviene nel tuo IdP. Deprovision un utente e il suo accesso al gateway scade entro la durata della sessione, un'ora per impostazione predefinita.
* **Controllo di accesso**: i tuoi gruppi IdP si mappano agli elenchi di modelli consentiti e alle politiche di [impostazioni gestite](/docs/it/permissions#managed-settings). Il gateway applica l'accesso ai modelli lato server, rifiutando le richieste per modelli non concessi, e seleziona la politica di impostazioni gestite di ogni gruppo, che il CLI applica al [livello di impostazioni gestite](/docs/it/settings#settings-precedence). Diversi team ottengono diversi modelli, strumenti e autorizzazioni, e uno sviluppatore non può ignorare ciò che la sua politica blocca.
* **Consegna delle impostazioni**: il gateway consegna le impostazioni gestite ai client connessi stesso, prendendo il posto delle [impostazioni gestite dal server](/docs/it/server-managed-settings) dalla console amministratore di claude.ai.
* **Telemetria**: ogni destinazione configurata, come Datadog, Splunk o ClickHouse, riceve [metriche OpenTelemetry Protocol (OTLP)](/docs/it/monitoring-usage) con conteggi di token, modello, identità dell'utente e latenza per impostazione predefinita, con log e tracce come opt-in per destinazione.
* **Instradamento upstream**: i client parlano l'API Anthropic Messages al gateway, e il gateway traduce per ogni upstream, sia Bedrock, [Claude Platform su AWS](/docs/it/claude-platform-on-aws), Agent Platform di Google Cloud, Foundry o l'API Anthropic, con failover tra loro. Puoi cambiare regioni, provider o ordine di failover senza che gli sviluppatori se ne accorgano o riconfigurino.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagramma che mostra i client Claude Code che si connettono tramite HTTPS con token bearer a un gateway di app Claude auto-ospitato all'interno della tua infrastruttura, che accede gli utenti rispetto al tuo IdP, archivia lo stato di autenticazione in PostgreSQL, trasmette la telemetria al tuo raccoglitore OTLP e inoltra l'inferenza ad Amazon Bedrock, Claude Platform su AWS, Google Cloud, Microsoft Foundry o all'API Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  Il piano dati del gateway stesso non invia nulla all'infrastruttura Anthropic a meno che l'API Anthropic non sia un upstream configurato. Controlli dove vanno la telemetria, i log di audit, le impostazioni gestite e l'identità IdP dei tuoi sviluppatori, e il gateway non li invia ad Anthropic. Per il traffico rimanente che il processo CLI può inviare e come chiuderlo, vedi [Compliance posture](/docs/it/claude-apps-gateway-deploy#compliance-posture).
</Note>

Per quali funzionalità di Claude Code funzionano attraverso il gateway e cosa supporta il server stesso, vedi [Disponibilità e limitazioni](#availability-and-limitations) di seguito. Per decisioni come costo, bypass, esecuzione di più gateway e piattaforme serverless, vedi la [guida di distribuzione](/docs/it/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Altre implementazioni di gateway
</h3>

Se esegui già un gateway LLM o un gateway API che soddisfa le tue esigenze, continua a usarlo; [Altri gateway LLM](/docs/it/llm-gateway) copre la configurazione di Claude Code rispetto ad esso.

Il [riferimento del protocollo gateway](/docs/it/llm-gateway-protocol) documenta il contratto che Claude Code si aspetta da qualsiasi gateway: gli endpoint che chiama, le intestazioni e i campi del corpo da inoltrare, e cosa smette di funzionare quando vengono rimossi. Un gateway di app Claude in esecuzione serve un superset di quel contratto su `GET /protocol`, aggiungendo gli endpoint specifici del gateway di app Claude per l'accesso SSO, la consegna delle impostazioni gestite e la telemetria. Recuperalo con `curl https://claude-gateway.internal.example.com/protocol` da qualsiasi gateway distribuito, come quello che la [guida rapida](#quickstart) di seguito produce.

I cambiamenti di rottura del protocollo vengono annunciati in anticipo, ma la compatibilità all'indietro indefinita non è garantita.

<h2 id="quickstart">
  Guida rapida
</h2>

Questa guida rapida percorre il percorso minimo: registra un client OAuth nel tuo IdP, scrivi un `gateway.yaml`, esegui il gateway insieme a Postgres con Docker Compose, e verifica l'accesso end-to-end. Utilizza un upstream Amazon Bedrock; Claude Platform su AWS, Agent Platform di Google Cloud, Microsoft Foundry e l'API Anthropic sono ugualmente supportati scambiando il blocco `upstreams` come mostrato nel [riferimento di configurazione](/docs/it/claude-apps-gateway-config#upstreams). Alla fine hai un gateway a cui uno sviluppatore può `/login`.

<Note>
  **Distribuisci sulla tua rete privata.** Claude Code si connette solo a un gateway il cui indirizzo è privato. Questo è un meccanismo di sicurezza, perché un gateway affidabile può spingere impostazioni che eseguono comandi su macchine sviluppatore. Posiziona il gateway dietro un load balancer interno o una VPN e assegnagli un nome host che si risolve solo in IP privati.

  Gli endpoint del gateway pubblico gestiti da Anthropic sono l'eccezione: `/login` li accetta su `https://`. Questi sono un piccolo insieme fisso di gateway che Anthropic stesso gestisce; non sono un'opzione di distribuzione che puoi selezionare o configurare. L'elenco è compilato in Claude Code, quindi nessuna configurazione può aggiungere un nome host ad esso e nessun gateway che ospiti si qualifica per l'esenzione. Prima della v2.1.206, `/login` rifiutava questi endpoint come qualsiasi altro indirizzo pubblico.
</Note>

<h3 id="prerequisites">
  Prerequisiti
</h3>

Avere questi in atto prima di iniziare:

| Hai bisogno                                | Dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 o successivo          | Il sottocomando `claude gateway` e il flusso di accesso al gateway vengono spediti in v2.1.195. Le build pubbliche precedenti non le includono. Sia la macchina che esegue il server gateway che la macchina di ogni sviluppatore devono essere su v2.1.195 o successivo; esegui `claude update` per ottenere l'ultimo rilascio. L'[upstream Claude Platform su AWS](/docs/it/claude-apps-gateway-config#claude-platform-on-aws) richiede Claude Code v2.1.198 o successivo sul server gateway.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Provider di identità OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak, o Dex, o qualsiasi altro IdP conforme a OIDC come PingFederate. Il gateway esegue il discovery OIDC standard e il flusso del codice di autorizzazione rispetto ad esso. SAML e LDAP non sono supportati.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| PostgreSQL 14 o successivo                 | Supporta il flusso di accesso del dispositivo, dove il callback del browser scrive e il CLI di polling legge, più contatori di limite di velocità. Qualsiasi Postgres gestito funziona, incluso il livello più piccolo. Senza limiti di spesa configurati, il gateway archivia pochi KB di stato di autenticazione di breve durata; con [limiti di spesa](/docs/it/claude-apps-gateway-spend-limits), contiene anche tabelle di spesa durevole, audit e identità che dovrebbero essere sottoposte a backup. TLS tramite `?sslmode=require` è consigliato.                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Upstream del modello                       | Credenziali Amazon Bedrock, credenziali Claude Platform su AWS, credenziali Google Cloud, una risorsa Microsoft Foundry o una chiave API Anthropic. Sono supportati più upstream con failover.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| HTTPS                                      | Il gateway deve essere raggiungibile su `https://` dai laptop degli sviluppatori e da qualsiasi browser utilizzato per l'accesso; il gateway serve la pagina di verifica del dispositivo sullo stesso listener. Fornisci un certificato TLS tramite `listen.tls` o esegui dietro un ingresso che termina TLS e imposta `listen.public_url`. Un'origine `http://` semplice è accettata solo su loopback, per lo sviluppo locale.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Indirizzo di rete privata                  | Su `/login`, Claude Code richiede che il nome host o l'indirizzo IP del gateway si risolvano solo in indirizzi privati: RFC 1918, CGNAT `100.64.0.0/10`, IPv6 ULA `fc00::/7` o loopback per lo sviluppo locale. Il controllo viene eseguito su ogni IP risolto, quindi se qualsiasi indirizzo a cui il nome si risolve è pubblico, `/login` rifiuta l'URL. Se le macchine degli sviluppatori instradano HTTPS attraverso un proxy aziendale, l'accesso richiede anche che l'host proxy si risolva in indirizzi privati; se non lo fa, aggiungi l'host del gateway a `NO_PROXY` in modo che il CLI si connetta direttamente. Gli endpoint del gateway gestiti da Anthropic sono esenti dai controlli di indirizzo privato e proxy: `/login` li accetta su `https://` per corrispondenza esatta del nome host, quindi il requisito di rete privata si applica solo a un gateway che ospiti tu stesso. Prima della v2.1.206, `/login` rifiutava un endpoint gestito da Anthropic come qualsiasi altro indirizzo pubblico. |
| Runtime Linux                              | Il server gateway viene eseguito solo sul binario Linux nativo. macOS funziona per lo sviluppo locale. Windows non è supportato come piattaforma server.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

Il server gateway richiede il binario `claude` nativo; scarica un rilascio bloccato come descritto in [Installa Claude Code](/docs/it/setup). Il server utilizza funzionalità di runtime che non sono disponibili quando Claude Code viene eseguito in Node. Se vedi `requires the native binary` all'avvio, passa a uno dei metodi di installazione standalone.

<h3 id="steps">
  Passaggi
</h3>

<Steps>
  <Step title="Registra un client OAuth nel tuo IdP">
    Decidi prima il nome host del gateway, perché l'URI di reindirizzamento deve corrispondere. Crea una nuova applicazione web OIDC e imposta l'URI di reindirizzamento su `https://claude-gateway.<your-domain>/oauth/callback`, dove l'host è lo stesso valore che imposti come [`listen.public_url`](/docs/it/claude-apps-gateway-config#listen) nel passaggio 3. Annota `client_id` e `client_secret`. Le istruzioni per IdP sono in [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Provisioning di un database PostgreSQL">
    Qualsiasi Postgres 14 o successivo funziona, incluso il livello gestito più piccolo. Il gateway esegue le proprie migrazioni dello schema all'avvio, quindi l'utente del database ha bisogno dell'autorizzazione `CREATE TABLE`. Se la tua politica di sicurezza proibisce DDL dai ruoli dell'applicazione, pre-crea lo schema invece; vedi [`store`](/docs/it/claude-apps-gateway-config#store).
  </Step>

  <Step title="Scrivi gateway.yaml">
    I segreti vengono letti tramite l'espansione `${ENV_VAR}` in modo che il file stesso possa vivere nel controllo della versione. Usa un nome host `public_url` che si risolve in un IP privato sulla tua rete, perché `/login` rifiuta gli indirizzi pubblici. La configurazione minima ha cinque sezioni, e ogni altro campo ha un valore predefinito:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Obbligatorio dietro qualsiasi proxy che termina TLS. Utilizzato per l'IdP
      # redirect_uri e il documento di discovery.
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # deve servire /.well-known/openid-configuration
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # rifiuta id_tokens al di fuori della tua organizzazione
      userinfo_fallback: true                  # per IdP il cui id_token omette email/groups; innocuo altrimenti

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # limita anche la latenza di revoca su deprovision IdP

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # aggiungi ?sslmode=require per Postgres gestito

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # vuoto: catena di credenziali predefinita AWS
    # (IRSA, ruolo attività EC2/ECS, variabili env, ~/.aws)

    # I modelli vengono tradotti per upstream automaticamente. Il catalogo integrato
    # mappa claude-opus-4-8 a us.anthropic.claude-opus-4-8 e così via per ogni
    # modello Claude supportato da Bedrock. Imposta false e aggiungi un elenco `models:` per
    # esporre solo modelli specifici.
    auto_include_builtin_models: true
    ```

    Questa configurazione è sufficiente per un ciclo di accesso funzionante con il catalogo di modelli Bedrock predefinito. Una volta in esecuzione, aggiungi RBAC per gruppo tramite [`managed.policies`](/docs/it/claude-apps-gateway-config#managed), fan-out di telemetria tramite [`telemetry`](/docs/it/claude-apps-gateway-config#telemetry), e failover multi-upstream, ARN di throughput provisioning o regioni non statunitensi tramite [`models`](/docs/it/claude-apps-gateway-config#models).

    <Note>
      L'upstream Bedrock ha bisogno di un principale AWS con `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` sia sugli ARN `inference-profile/us.anthropic.*` che sugli ARN `foundation-model/anthropic.*` sottostanti, e l'accesso ai modelli abilitato nella console Bedrock per i modelli Claude che desideri. Fornisci la credenziale con IRSA su EKS, un ruolo attività ECS o un profilo di istanza EC2 piuttosto che chiavi statiche. Il [riferimento `upstreams`](/docs/it/claude-apps-gateway-config#upstreams) ha i dettagli IAM completi, la matrice di credenziali cross-cloud e i blocchi `auth` per gli altri provider.
    </Note>
  </Step>

  <Step title="Eseguilo">
    Costruisci un'immagine container attorno al binario `claude` che soddisfi i [requisiti dell'immagine](/docs/it/claude-apps-gateway-deploy#container-image), quindi eseguila insieme a Postgres:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: <your-registry>/claude-gateway:<version>
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # Credenziali AWS: in produzione, ometti questi e usa un ruolo di istanza
          # Per il test locale di Compose, passa i tuoi:
          AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
          AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
          AWS_SESSION_TOKEN: ${AWS_SESSION_TOKEN}
        depends_on:
          postgres:
            condition: service_healthy
      postgres:
        image: postgres:16-alpine
        environment: { POSTGRES_USER: gw, POSTGRES_PASSWORD: pw, POSTGRES_DB: gateway }
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U gw"]
          interval: 5s
        volumes: ["pgdata:/var/lib/postgresql/data"]
    volumes: { pgdata: }
    ```

    Il gateway è un singolo binario Linux che legge la configurazione, esegue il discovery OIDC rispetto al tuo IdP, applica le migrazioni dello schema Postgres, costruisce client upstream e inizia ad ascoltare. L'avvio è fail-closed per la configurazione, la connessione Postgres con un timeout di 5 secondi, il discovery OIDC e la costruzione del client upstream. Se uno di questi è irraggiungibile o non configurato correttamente, il gateway esce con un errore piuttosto che servire il traffico in uno stato degradato.

    Un avvio riuscito non convalida il percorso di inferenza, perché le credenziali dell'istanza Bedrock e Agent Platform si risolvono sulla prima richiesta, non all'avvio.

    Guarda stderr per la sequenza di avvio. Le righe di log utilizzano il formato `[gateway] <timestamp> <level> <message>`, gli eventi di audit sono JSON a riga singola con un campo `evt`, e un banner di avvio, omesso di seguito, viene stampato tra la migrazione e le righe di ascolto. Dovresti vedere, in ordine:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    Se l'avvio esce prima della riga `claude gateway listening on`, l'ultima riga di stderr nomina il problema:

    * un Postgres irraggiungibile
    * un ruolo Postgres senza autorizzazione DDL
    * un documento di discovery OIDC irraggiungibile o non valido
    * una violazione dello schema di configurazione con il percorso del campo offensivo

    Correggilo e riavvia.

    Se hai già un ingresso che termina TLS, salta Compose ed esegui il binario direttamente con `claude gateway --config gateway.yaml`. Imposta `public_url` all'origine dell'ingresso e associa `listen` a un indirizzo loopback o interno al cluster.
  </Step>

  <Step title="Verifica la superficie di autenticazione">
    Tre controlli confermano che il gateway può autenticare un utente reale prima di consegnarlo a uno sviluppatore.

    Gli esempi utilizzano l'URL pubblico del gateway; per la configurazione locale di Compose senza un ingresso, sostituisci `http://localhost:8080` nei primi due controlli. Il terzo controllo apre `verification_uri_complete`, che è costruito da `public_url`, quindi per Compose locale imposta `public_url: http://localhost:8080` in `gateway.yaml` e aggiungi `http://localhost:8080/oauth/callback` come secondo URI di reindirizzamento sul client OAuth dal passaggio 1, perché il gateway costruisce l'IdP `redirect_uri` da `public_url`. Il link di verifica si apre quindi nel tuo browser locale.

    In Windows PowerShell, esegui `curl.exe`; il `curl` semplice è un alias per `Invoke-WebRequest` e rifiuta questi flag.

    Per primo, recupera il documento di discovery, che conferma che il gateway è attivo, la configurazione è valida e tutti i controlli di avvio sono passati:

    ```bash theme={null}
    curl -s https://claude-gateway.internal.example.com/.well-known/oauth-authorization-server | jq
    ```

    ```json theme={null}
    {
      "issuer": "https://claude-gateway.internal.example.com",
      "device_authorization_endpoint": "…/oauth/device_authorization",
      "token_endpoint": "…/oauth/token",
      "grant_types_supported": ["urn:ietf:params:oauth:grant-type:device_code", "refresh_token"]
    }
    ```

    La risposta include campi aggiuntivi, come `response_types_supported` e `scopes_supported`.

    Secondo, richiedi un'autorizzazione del dispositivo, che conferma che il flusso di accesso del dispositivo funziona e Postgres è raggiungibile e scrivibile:

    ```bash theme={null}
    curl -s -X POST https://claude-gateway.internal.example.com/oauth/device_authorization | jq
    ```

    ```json theme={null}
    {
      "device_code": "…",
      "user_code": "WDJB-MJHT",
      "verification_uri": "https://claude-gateway.internal.example.com/device",
      "verification_uri_complete": "https://claude-gateway.internal.example.com/device?user_code=WDJB-MJHT",
      "expires_in": 600,
      "interval": 5
    }
    ```

    Terzo, testa la parte del browser aprendo `verification_uri_complete` in un browser e confermando il codice. Dovresti essere reindirizzato alla pagina di accesso del tuo IdP e, dopo l'accesso, tornare al gateway con una conferma di accesso.

    Usa il primo controllo che fallisce per individuare il problema:

    * **Il primo controllo fallisce**: l'avvio non è stato completato; controlla stderr
    * **Il secondo controllo fallisce**: Postgres non è raggiungibile dal gateway o il ruolo non può scrivere; controlla la stringa di connessione e le autorizzazioni
    * **Il terzo controllo non raggiunge l'IdP**: controlla che l'URI di reindirizzamento dell'IdP corrisponda esattamente a `https://<gateway>/oauth/callback`
    * **Il terzo controllo raggiunge l'IdP ma rimbalza indietro con un errore**: leggi il log di audit del gateway, che registra ogni rifiuto di autenticazione con il motivo, come `email domain not allowed`
  </Step>

  <Step title="Accedi a uno sviluppatore">
    Questo ultimo passaggio avviene su una macchina sviluppatore, non sul server. Imposta `forceLoginMethod` su `"gateway"` e `forceLoginGatewayUrl` su `public_url` del tuo gateway nel [file delle impostazioni gestite](/docs/it/settings#settings-files) di quella macchina, quindi esegui `/login`, premi Invio sulla schermata **Cloud gateway** e completa l'accesso del browser. [Imposta l'URL del gateway](#set-the-gateway-url) di seguito copre la distribuzione di entrambe le chiavi su larga scala.
  </Step>
</Steps>

<h2 id="connect-developers">
  Connetti gli sviluppatori
</h2>

Gli sviluppatori si connettono dai loro laptop con un accesso al browser, utilizzando il loro account di lavoro aziendale. Non hanno bisogno di un account claude.ai, una chiave API o un abbonamento, perché le richieste al modello passano attraverso il gateway utilizzando la credenziale upstream dell'organizzazione. La connessione è guidata dalle [impostazioni gestite lato client](/docs/it/claude-apps-gateway-config#client-side-managed-settings) che spingere tramite MDM, quindi non c'è configurazione manuale sul lato dello sviluppatore; questa sezione copre cosa configura l'amministratore.

Il CLI impronta digitale il certificato foglia TLS del gateway al primo collegamento e lo blocca per nome host. Pubblica l'impronta digitale SHA-256 prevista insieme all'URL del gateway in modo che gli sviluppatori abbiano qualcosa da confrontare. Ottieni l'impronta digitale dal file del certificato con `openssl x509 -noout -fingerprint -sha256 -in cert.pem`; il prompt `/login` mostra i primi 16 caratteri del digest come esadecimale minuscolo senza separatori.

Quando il certificato ruota, ogni sviluppatore vede di nuovo il prompt di fiducia, quindi tratta le rotazioni come un evento pianificato e ripubblica l'impronta digitale.

Una volta connesso, il [selettore di modelli](/docs/it/model-config) mostra i modelli nell'elenco di autorizzazione `availableModels` dello sviluppatore, le impostazioni gestite si applicano all'avvio e si aggiornano ogni ora, e la telemetria si instrada al tuo raccoglitore. Le sessioni si aggiornano silenziosamente prima della scadenza di `ttl_hours` e un aggiornamento non riuscito dopo il deprovision IdP richiede un nuovo accesso.

<h3 id="set-the-gateway-url">
  Imposta l'URL del gateway
</h3>

Imposta entrambe le chiavi nel file di [impostazioni gestite](/docs/it/settings#settings-files) per OS che distribuisci tramite MDM o direttamente su disco, e `/login` si apre direttamente sulla schermata **Cloud gateway** con l'URL compilato:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Lo sviluppatore preme Invio per connettersi. Il prompt dell'impronta digitale TLS del primo collegamento appare ancora.

Non c'è opzione gateway nel selettore di accesso per uno sviluppatore da selezionare manualmente, e `forceLoginGatewayUrl` viene ignorato nei file di impostazioni personali di uno sviluppatore. `forceLoginMethod` da solo, senza un URL, lascia lo sviluppatore con un messaggio "Contatta il tuo amministratore IT". Entrambe le chiavi appartengono al file che spingere alle macchine, non al blocco `managed.policies[].cli` del gateway, che raggiunge solo i client già connessi.

<h3 id="ci-pipelines-and-remote-machines">
  Pipeline CI e macchine remote
</h3>

Non c'è flusso di token di servizio per pipeline non presenziate. L'accesso al gateway esegue sempre il flusso del dispositivo del browser, quindi un lavoro CI senza uno sviluppatore per approvare l'accesso non può autenticarsi; configura quelli direttamente contro il tuo provider.

Una volta che uno sviluppatore ha effettuato l'accesso, ogni invocazione di Claude Code su quella macchina utilizza la sessione del gateway, incluse le esecuzioni non interattive `claude -p` e le sessioni avviate da Agent SDK, e la [politica del gateway si applica a tutte](/docs/it/claude-apps-gateway-config#managed).

Il flusso del dispositivo separa il CLI di polling dal browser di approvazione, quindi una scatola di sviluppo remoto senza display funziona ancora: lo sviluppatore esegue `/login` su SSH sulla macchina remota e apre il link di verifica nel browser sul suo laptop.

<h3 id="what’s-enforced-on-developers">
  Cosa viene applicato agli sviluppatori
</h3>

Queste garanzie si applicano a ogni sessione gateway connessa.

* **Accesso ai modelli**: le richieste per modelli che la politica non concede restituiscono 400 e il selettore `/model` viene filtrato all'elenco di autorizzazione `availableModels` della politica. Imposta [`enforceAvailableModels: true`](/docs/it/model-config#default-model-behavior) nella politica in modo che l'opzione Predefinita si risolva in un modello all'interno di `availableModels` invece che al valore predefinito integrato di Claude Code; senza di esso, Predefinita rimane selezionabile e viene rifiutata al momento della richiesta se quel modello non è concesso.
* **Destinazione telemetria**: quando è configurato l'[inoltro di telemetria](/docs/it/claude-apps-gateway-config#telemetry), l'endpoint di esportazione OTLP è bloccato al gateway e la configurazione spinta dal gateway sostituisce le variabili `OTEL_*` impostate localmente.
* **Credenziali**: il token del gateway è l'unica credenziale della sessione. `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, `apiKeyHelper` e qualsiasi accesso precedente a claude.ai vengono ignorati mentre connesso, quindi gli sviluppatori non hanno bisogno di disconnettersi da claude.ai per primo.
* **Impostazioni gestite**: le chiavi bloccate non possono essere ignorate localmente. Il CLI applica la politica all'avvio e su ogni sondaggio orario.
* **Avvio**: le sessioni connesse escono all'avvio con un errore dopo circa 10 secondi quando il gateway è irraggiungibile, piuttosto che iniziare senza le loro impostazioni.
* **Deprovision**: una sessione il cui utente è disabilitato nell'IdP scade entro `ttl_hours` quando il prossimo aggiornamento fallisce.

<h3 id="what-the-organization-can-see">
  Cosa può vedere l'organizzazione
</h3>

La telemetria di utilizzo porta l'identità dello sviluppatore, i conteggi di token, il modello e la latenza al raccoglitore dell'organizzazione. Il gateway non registra o archivia il contenuto del prompt o del completamento. Se viene raccolta una telemetria più ricca come log e tracce, che può includere comandi e percorsi di file, è la [scelta per destinazione](/docs/it/claude-apps-gateway-config#telemetry) dell'organizzazione.

<h2 id="availability-and-limitations">
  Disponibilità e limitazioni
</h2>

La tabella copre quali funzionalità di Claude Code funzionano quando gli sviluppatori si connettono attraverso il gateway e cosa supporta il server gateway stesso. Dove qualcosa non è supportato, la colonna Note fornisce l'alternativa.

Il gateway consegna i valori [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) che il CLI invia a ogni upstream, quindi gli operatori non mantengono un elenco di autorizzazioni beta. Per Amazon Bedrock, che ignora l'intestazione, il gateway sposta i valori nel campo `anthropic_beta` del corpo della richiesta; gli altri upstream ricevono l'intestazione come inviata.

L'insieme beta della sessione del gateway del CLI omette i beta solo per la prima parte e il beta della cache estesa, ecco perché quelle righe di seguito mostrano come non disponibili.

| Funzionalità                                                                                                                | Stato           | Note                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| --------------------------------------------------------------------------------------------------------------------------- | --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Inoltro di inferenza (Amazon Bedrock, Claude Platform su AWS, Agent Platform di Google Cloud, Microsoft Foundry, Anthropic) | Disponibile     | Con traduzione del modello per upstream e failover. L'upstream Amazon Bedrock utilizza l'endpoint `bedrock-runtime` e la catena di credenziali predefinita AWS; l'[endpoint Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint) di Amazon Bedrock non è un upstream supportato. L'[upstream Claude Platform su AWS](/docs/it/claude-apps-gateway-config#claude-platform-on-aws) richiede Claude Code v2.1.198 o successivo sul server gateway. |
| Accesso ai modelli e impostazioni gestite per gruppo IdP                                                                    | Disponibile     | L'accesso ai modelli viene applicato lato server; le impostazioni gestite vengono consegnate per gruppo IdP e applicate dal CLI al [livello di impostazioni gestite](/docs/it/settings#settings-precedence)                                                                                                                                                                                                                                    |
| Fan-out di telemetria (OTLP/HTTP)                                                                                           | Disponibile     | Identità-timbrato per esportazione; entrambe le codifiche protobuf e JSON                                                                                                                                                                                                                                                                                                                                                                 |
| Provider di identità OIDC                                                                                                   | Disponibile     | Qualsiasi IdP conforme a OIDC; il gateway esegue il discovery OIDC standard e il flusso del codice di autorizzazione. Vedi [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup) per la configurazione per IdP                                                                                                                                                                                |
| Limiti di spesa per utente e per gruppo                                                                                     | Disponibile     | Vedi [Limiti di spesa](/docs/it/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                              |
| Ricerca web lato server                                                                                                     | Non disponibile | Il CLI non può vedere quale provider upstream il gateway instrada, quindi non può verificare il supporto della ricerca web e disabilita WebSearch sulle sessioni del gateway                                                                                                                                                                                                                                                              |
| Caching del prompt standard                                                                                                 | Disponibile     | I breakpoint `cache_control` vengono inoltrati a ogni upstream                                                                                                                                                                                                                                                                                                                                                                            |
| TTL cache di 1 ora                                                                                                          | Non disponibile | Il CLI omette il beta della cache estesa sulle sessioni del gateway, perché non ogni upstream a cui il gateway può instradare supporta il TTL di 1 ora, quindi il caching del prompt attraverso il gateway utilizza il TTL di 5 minuti; vedi la nota dell'intestazione beta sopra                                                                                                                                                         |
| Modalità Auto                                                                                                               | Disponibile     | Segue le [regole del provider di terze parti](/docs/it/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): solo i modelli idonei sui provider di terze parti possono usarlo. Prima della v2.1.207, la modalità auto sulle sessioni del gateway richiedeva l'impostazione di `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, consegnabile tramite il blocco `env` della politica gestita                                                 |
| Ottimizzazioni solo per la prima parte come ambito cache globale e strumenti efficienti in termini di token                 | Non disponibile | Il CLI non le abilita sulle sessioni del gateway; vedi la nota dell'intestazione beta sopra                                                                                                                                                                                                                                                                                                                                               |
| OTLP/gRPC                                                                                                                   | Non supportato  | OTLP su HTTP solo                                                                                                                                                                                                                                                                                                                                                                                                                         |
| SAML, LDAP e altri auth non OIDC                                                                                            | Non supportato  | Solo OIDC. Fronte con un ponte OIDC se necessario                                                                                                                                                                                                                                                                                                                                                                                         |
| Multi-tenant (più emittenti OIDC)                                                                                           | Non supportato  | Un emittente per gateway. Esegui istanze separate                                                                                                                                                                                                                                                                                                                                                                                         |
| Server Windows                                                                                                              | Non supportato  | Distribuisci su Linux. macOS solo per lo sviluppo locale                                                                                                                                                                                                                                                                                                                                                                                  |
| Helm chart                                                                                                                  | Non disponibile | Il gateway viene eseguito come una Deployment stateless standard; vedi la [guida di distribuzione](/docs/it/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                             |
| Interfaccia utente amministratore                                                                                           | Non disponibile | La configurazione è il file YAML; ridistribuisci per cambiarla                                                                                                                                                                                                                                                                                                                                                                            |

<h2 id="next-steps">
  Passaggi successivi
</h2>

La guida rapida ti lascia con una configurazione minima in esecuzione sotto Docker Compose. Per andare oltre:

* Espandi `gateway.yaml` oltre la configurazione minima, ad esempio per aggiungere RBAC per gruppo, failover multi-upstream o destinazioni di telemetria. Il [riferimento di configurazione](/docs/it/claude-apps-gateway-config) copre ogni opzione.
* Passa da Compose a una distribuzione di produzione su Kubernetes o Cloud Run, configura correttamente il tuo IdP e rivedi il modello di sicurezza. La [guida di distribuzione e operazioni](/docs/it/claude-apps-gateway-deploy) copre la configurazione per IdP, i requisiti dell'immagine container, i probe di salute e la risoluzione dei problemi.
* Metti limiti di spesa su singoli sviluppatori o gruppi in modo che un carico di lavoro incontrollato non possa consumare il tuo intero impegno. [Limiti di spesa](/docs/it/claude-apps-gateway-spend-limits) copre l'API amministratore e come funziona l'applicazione.
* Per un esempio completo su Google Cloud, con Cloud Run, Cloud SQL e Secret Manager, vedi [Distribuisci su Google Cloud](/docs/it/claude-apps-gateway-on-gcp).
