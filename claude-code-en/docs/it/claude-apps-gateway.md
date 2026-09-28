> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gateway di app Claude per Amazon Bedrock, Claude Platform su AWS, Google Cloud e Microsoft Foundry

> Esegui Claude Code attraverso Amazon Bedrock, Claude Platform su AWS, Google Cloud o Microsoft Foundry dietro un gateway auto-ospitato con accesso SSO, accesso ai modelli per gruppo e telemetria OTLP.

<Note>
  Il gateway di app Claude è progettato per le organizzazioni che devono — o preferiscono — instradare l'inferenza attraverso il proprio provider cloud, ad esempio per soddisfare i requisiti di [residenza dei dati](/docs/it/claude-apps-gateway-deploy#compliance-posture). Se non hai questo requisito e desideri accesso ad altre funzionalità come il provisioning SCIM o Claude Code su web e mobile, Claude Enterprise potrebbe essere una scelta migliore. Consulta la pagina di [disponibilità delle funzionalità](/docs/it/feature-availability) per un confronto completo di tutti i metodi di distribuzione.
</Note>

Claude apps gateway è un servizio auto-ospitato che si posiziona tra i client Claude Code dei tuoi sviluppatori e il tuo provider di modelli. Gli sviluppatori accedono con il tuo provider di identità aziendale (IdP) invece di detenere chiavi API o credenziali cloud. Il gateway contiene la credenziale upstream, applica l'accesso ai modelli e le [impostazioni gestite](/docs/it/managed-settings) per gruppo IdP, e trasmette la telemetria di utilizzo al tuo stack di osservabilità.

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
* **Controllo di accesso**: i tuoi gruppi IdP si mappano agli elenchi di modelli consentiti e alle politiche di [impostazioni gestite](/docs/it/managed-settings). Il gateway applica l'accesso ai modelli lato server, rifiutando le richieste per modelli non concessi, e seleziona la politica di impostazioni gestite di ogni gruppo, che il CLI applica al [livello di impostazioni gestite](/docs/it/settings#settings-precedence). Diversi team ottengono diversi modelli, strumenti e autorizzazioni, e uno sviluppatore non può ignorare ciò che la sua politica blocca.
* **Consegna delle impostazioni**: il gateway consegna le impostazioni gestite ai client connessi stesso, prendendo il posto delle [impostazioni gestite dal server](/docs/it/server-managed-settings) dalla console amministratore di claude.ai.
* **Telemetria**: ogni destinazione configurata riceve [metriche OpenTelemetry Protocol (OTLP)](/docs/it/monitoring-usage) con conteggi di token, modello, identità dell'utente e latenza per impostazione predefinita, con log e tracce come opt-in per destinazione.
* **Instradamento upstream**: i client parlano l'API Anthropic Messages al gateway, e il gateway traduce per ogni upstream, sia Bedrock, [Claude Platform su AWS](/docs/it/claude-platform-on-aws), Agent Platform di Google Cloud, Foundry o l'API Anthropic, con failover tra loro. Puoi cambiare regioni, provider o ordine di failover senza che gli sviluppatori se ne accorgano o riconfigurino.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagramma che mostra i client Claude Code e le schede Chat, Cowork e Code di Claude Desktop che si connettono tramite HTTPS con token bearer a un gateway di app Claude auto-ospitato all'interno della tua infrastruttura, che accede gli utenti rispetto al tuo IdP, archivia lo stato di autenticazione in PostgreSQL, trasmette la telemetria al tuo raccoglitore OTLP e inoltra l'inferenza ad Amazon Bedrock, Claude Platform su AWS, Google Cloud, Microsoft Foundry o all'API Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  Il piano dati del gateway stesso non invia nulla all'infrastruttura Anthropic a meno che l'API Anthropic non sia un upstream configurato. Controlli dove vanno la telemetria, i log di audit, le impostazioni gestite e l'identità IdP dei tuoi sviluppatori, e il gateway non li invia ad Anthropic. Per il traffico rimanente che il processo CLI può inviare e come chiuderlo, vedi [Compliance posture](/docs/it/claude-apps-gateway-deploy#compliance-posture).
</Note>

Per quali funzionalità di Claude Code funzionano attraverso il gateway e cosa supporta il server stesso, vedi [Disponibilità e limitazioni](#availability-and-limitations) di seguito. Per decisioni come costo, bypass, esecuzione di più gateway e piattaforme serverless, vedi la [guida di distribuzione](/docs/it/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Altre implementazioni di gateway
</h3>

Se esegui già un gateway LLM o un gateway API che soddisfa le tue esigenze, continua a usarlo; [Altri gateway LLM](/docs/it/llm-gateway) copre la configurazione di Claude Code rispetto ad esso.

La [guida di compatibilità del gateway](/docs/it/llm-gateway-protocol) documenta cosa Claude Code si aspetta da qualsiasi gateway: gli endpoint che chiama, le intestazioni e i campi del corpo da inoltrare, e cosa smette di funzionare quando vengono rimossi. Un gateway di app Claude in esecuzione serve anche il suo proprio riferimento di protocollo su `GET /protocol`, che descrive gli endpoint che espone ai client Claude Code: accesso SSO, inferenza, consegna di impostazioni gestite, scoperta di modelli e telemetria. Recuperalo con `curl https://claude-gateway.internal.example.com/protocol` da qualsiasi gateway distribuito, come quello che la [guida rapida](#quickstart) di seguito produce.

I cambiamenti di rottura del protocollo vengono annunciati in anticipo, ma la compatibilità all'indietro indefinita non è garantita.

<h2 id="quickstart">
  Guida rapida
</h2>

Questa guida rapida percorre il percorso minimo: registra un client OAuth nel tuo IdP, scrivi un `gateway.yaml`, esegui il gateway insieme a Postgres con Docker Compose, e verifica l'accesso end-to-end. Utilizza un upstream Amazon Bedrock; Claude Platform su AWS, Agent Platform di Google Cloud, Microsoft Foundry e l'API Anthropic sono ugualmente supportati scambiando il blocco `upstreams` come mostrato nel [riferimento di configurazione](/docs/it/claude-apps-gateway-config#upstreams). Alla fine hai un gateway a cui uno sviluppatore può `/login`.

<Note>
  **Distribuisci sulla tua rete privata.** Claude Code si connette solo a un gateway il cui indirizzo è privato. Questo è un meccanismo di sicurezza, perché un gateway affidabile può spingere impostazioni che eseguono comandi su macchine sviluppatore. Posiziona il gateway dietro un load balancer interno o una VPN e assegnagli un nome host che si risolve solo in IP privati. Se la tua rete interna è numerata da spazio IPv4 pubblico che la tua organizzazione possiede, vedi [Consenti un gateway su spazio di indirizzi pubblici che possiedi](#allow-a-gateway-on-public-address-space-you-own).
</Note>

<h3 id="prerequisites">
  Prerequisiti
</h3>

Avere questi in atto prima di iniziare:

| Hai bisogno                                | Dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 o successivo          | Il sottocomando `claude gateway` e il flusso di accesso al gateway vengono spediti in v2.1.195. Le build pubbliche precedenti non le includono. Sia la macchina che esegue il server gateway che la macchina di ogni sviluppatore devono essere su v2.1.195 o successivo; esegui `claude update` per ottenere l'ultimo rilascio. L'[upstream Claude Platform su AWS](/docs/it/claude-apps-gateway-config#claude-platform-on-aws) richiede Claude Code v2.1.198 o successivo sul server gateway.                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Provider di identità OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak, o Dex, o qualsiasi altro IdP conforme a OIDC come PingFederate. Il gateway esegue il discovery OIDC standard e il flusso del codice di autorizzazione rispetto ad esso. SAML e LDAP non sono supportati.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| PostgreSQL 14 o successivo                 | Supporta il flusso di accesso del dispositivo, dove il callback del browser scrive e il CLI di polling legge, più contatori di limite di velocità. Qualsiasi Postgres gestito funziona, incluso il livello più piccolo. Senza limiti di spesa configurati, il gateway archivia pochi KB di stato di autenticazione di breve durata; con [limiti di spesa](/docs/it/claude-apps-gateway-spend-limits), contiene anche tabelle di spesa durevole, audit e identità che dovrebbero essere sottoposte a backup. TLS tramite `?sslmode=require` è consigliato.                                                                                                                                                                                                                                                                                                                                                                   |
| Upstream del modello                       | Credenziali Amazon Bedrock, credenziali Claude Platform su AWS, credenziali Google Cloud, una risorsa Microsoft Foundry o una chiave API Anthropic. Sono supportati più upstream con failover.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| HTTPS                                      | Il gateway deve essere raggiungibile su `https://` dai laptop degli sviluppatori e da qualsiasi browser utilizzato per l'accesso; il gateway serve la pagina di verifica del dispositivo sullo stesso listener. Fornisci un certificato TLS tramite `listen.tls` o esegui dietro un ingresso che termina TLS, e imposta `listen.public_url` all'origine esterna in entrambi i casi. Un'origine `http://` semplice è accettata solo quando l'host del gateway è loopback: `localhost`, `127.0.0.1`, o `::1`.                                                                                                                                                                                                                                                                                                                                                                                                            |
| Indirizzo di rete privata                  | Su `/login`, Claude Code richiede che il nome host o l'indirizzo IP del gateway si risolvano solo in indirizzi privati: RFC 1918, link-local, CGNAT `100.64.0.0/10`, IPv6 ULA `fc00::/7`, o loopback. Per un gateway che ospiti, qualsiasi indirizzo pubblico al di fuori di un blocco che dichiari è rifiutato; vedi il [modello di minaccia](/docs/it/claude-apps-gateway-deploy#threat-model-summary) nella guida di distribuzione. Se le macchine degli sviluppatori instradano HTTPS attraverso un proxy aziendale, l'accesso richiede anche che l'host proxy si risolva in indirizzi privati; se non lo fa, aggiungi l'host del gateway a `NO_PROXY` in modo che il CLI si connetta direttamente. Se la tua rete interna è numerata da spazio IPv4 pubblico che la tua organizzazione possiede, [dichiara quei blocchi](#allow-a-gateway-on-public-address-space-you-own) in modo che `/login` accetti un gateway lì. |
| Runtime Linux                              | Il server gateway viene eseguito solo sul binario Linux nativo. macOS funziona per lo sviluppo locale. Windows non è supportato come piattaforma server.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h3 id="steps">
  Passaggi
</h3>

<Steps>
  <Step title="Registra un client OAuth nel tuo IdP">
    Decidi prima il nome host del gateway, perché l'URI di reindirizzamento deve corrispondere. Crea una nuova applicazione web OIDC e imposta l'URI di reindirizzamento su `https://claude-gateway.<your-domain>/oauth/callback`, dove l'host è lo stesso valore che imposti come [`listen.public_url`](/docs/it/claude-apps-gateway-config#listen) nel passaggio 3. Annota `client_id` e `client_secret`. Le istruzioni per IdP sono in [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Provisioning di un database PostgreSQL">
    Qualsiasi Postgres 14 o successivo funziona, incluso il livello gestito più piccolo. Il gateway esegue le proprie migrazioni dello schema all'avvio, quindi il ruolo del database ha bisogno dei diritti per creare e alterare le tabelle; vedi [`store`](/docs/it/claude-apps-gateway-config#store).
  </Step>

  <Step title="Scrivi gateway.yaml">
    I segreti vengono letti tramite l'espansione `${ENV_VAR}` in modo che il file stesso possa vivere nel controllo della versione. Usa un nome host `public_url` che si risolve in un IP privato sulla tua rete, perché `/login` rifiuta gli indirizzi pubblici. La configurazione minima ha cinque sezioni, e ogni altro campo ha un valore predefinito:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Obbligatorio a meno che l'host non sia un indirizzo loopback. Utilizzato per l'IdP
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
      L'upstream Amazon Bedrock ha bisogno di un principale AWS con `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` sia sugli ARN `inference-profile/us.anthropic.*` che sugli ARN `foundation-model/anthropic.*` sottostanti. Ha anche bisogno del modulo di caso d'uso una tantum di Anthropic inviato per l'account dalla console Bedrock Model catalog.

      Fornisci la credenziale con IRSA su EKS, un ruolo attività ECS o un profilo di istanza EC2 piuttosto che chiavi statiche. Il [riferimento `upstreams`](/docs/it/claude-apps-gateway-config#upstreams) ha i dettagli IAM completi, la matrice di credenziali cross-cloud e i blocchi `auth` per gli altri provider.
    </Note>
  </Step>

  <Step title="Eseguilo">
    Costruisci un'immagine container attorno al binario `claude` che soddisfi i [requisiti dell'immagine](/docs/it/claude-apps-gateway-deploy#container-image), quindi eseguila insieme a Postgres. Il file Compose fa riferimento all'immagine come `registry.example.com/claude-gateway:2.1.198`; sostituisci il tuo registro e tag immagine:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: registry.example.com/claude-gateway:2.1.198
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # Credenziali AWS: in produzione, ometti questi e usa un ruolo
          # di istanza. Per il test locale di Compose, passa i tuoi:
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

    Il gateway è un singolo binario Linux che legge la configurazione, si connette a Postgres e applica le migrazioni dello schema, esegue il discovery OIDC rispetto al tuo IdP, costruisce client upstream e inizia ad ascoltare.

    L'avvio è fail-closed per la configurazione, la connessione Postgres, il discovery OIDC e la costruzione del client upstream. Se uno di questi è irraggiungibile o non configurato correttamente, il gateway esce con un errore piuttosto che servire il traffico in uno stato degradato.

    Un avvio riuscito non convalida il percorso di inferenza, perché le credenziali dell'istanza Bedrock e Agent Platform si risolvono sulla prima richiesta, non all'avvio.

    Guarda stderr per la sequenza di avvio. Le righe di log utilizzano il formato `[gateway] <timestamp> <level> <message>`, gli eventi di audit sono JSON a riga singola con un campo `evt`, e un banner di avvio, omesso di seguito, viene stampato tra la migrazione e le righe di ascolto. Un database nuovo stampa una riga `migration N applied` per ogni migrazione dello schema; un database già migrato non stampa nulla. Dovresti vedere, in ordine:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.395Z info waiting for migration lock (another replica may be migrating; check pg_locks for key 6775156 if this persists)
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    …
    [gateway] 2026-06-10T17:03:21.431Z info migration 6 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    Il gateway registra anche un avviso che `access_control.allow_cidrs` è vuoto. Questo è previsto qui, perché nulla limita quali indirizzi client il gateway serve fino a quando non imposti un elenco di autorizzazione. Il [riferimento `access_control`](/docs/it/claude-apps-gateway-config#http-tuning) ha gli intervalli consigliati.

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
    Questo ultimo passaggio avviene su una macchina sviluppatore, non sul server. Imposta `forceLoginMethod` su `"gateway"` e `forceLoginGatewayUrl` su `public_url` del tuo gateway nel [file delle impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms) di quella macchina, quindi esegui `/login`, premi Invio sulla schermata **Cloud gateway** e completa l'accesso del browser. [Imposta l'URL del gateway](#set-the-gateway-url) di seguito copre la distribuzione di entrambe le chiavi su ogni macchina sviluppatore.
  </Step>
</Steps>

<h2 id="connect-developers">
  Connetti gli sviluppatori
</h2>

Gli sviluppatori si connettono dai loro laptop con un accesso al browser, utilizzando il loro account di lavoro aziendale. Non hanno bisogno di un account claude.ai, una chiave API o un abbonamento, perché le richieste al modello passano attraverso il gateway utilizzando la credenziale upstream dell'organizzazione. La connessione è guidata dalle [impostazioni gestite lato client](/docs/it/claude-apps-gateway-config#client-side-managed-settings) che spingere tramite MDM, quindi non c'è configurazione manuale sul lato dello sviluppatore; questa sezione copre cosa configura l'amministratore.

Il CLI impronta digitale il certificato foglia TLS del gateway al primo collegamento e lo blocca per nome host. Controlla di nuovo quel pin durante l'accesso, sui refresh di sessione silenziosi e sui recuperi di impostazioni gestite, mentre le richieste di inferenza utilizzano la convalida TLS standard senza il pin. Le richieste instradate attraverso un proxy HTTPS saltano il controllo del pin, quindi aggiungere l'host del gateway a `NO_PROXY` per mantenerle dirette.

Pubblica l'impronta digitale SHA-256 prevista insieme all'URL del gateway in modo che gli sviluppatori abbiano qualcosa da confrontare. Il prompt `/login` mostra i primi 16 caratteri dell'impronta digitale come esadecimale minuscolo senza due punti. Per stampare l'impronta digitale completa in quella forma dal file del certificato, eseguire:

```bash theme={null}
openssl x509 -noout -fingerprint -sha256 -in cert.pem | cut -d= -f2 | tr -d : | tr 'A-F' 'a-f'
```

Quando il certificato ruota, ogni sviluppatore vede di nuovo il prompt di fiducia, quindi tratta le rotazioni come un evento pianificato e ripubblica l'impronta digitale. Se la politica del gateway include [impostazioni che necessitano di approvazione](/docs/it/server-managed-settings#security-approval-dialogs), lo sviluppatore vede anche di nuovo quel dialogo di approvazione dopo aver accettato il nuovo certificato, perché Claude Code [memoria di approvazione](/docs/it/server-managed-settings#approval-memory) delle chiavi al certificato bloccato.

Un gateway può restituire il campo facoltativo `email` nella sua risposta di token per nominare l'account che un accesso ha utilizzato. Quando lo fa, lo sviluppatore conferma l'account prima che Claude Code salvi la credenziale. Dopo un accesso confermato, `/status` mostra l'account.

La conferma richiede Claude Code v2.1.275 o successivo sulla macchina dello sviluppatore; un client al di sotto di quella versione ignora il campo. Il server del gateway nel binario `claude` non restituisce il campo, quindi i suoi accessi si completano senza la conferma.

Una volta che lo sviluppatore accede, il [selettore di modelli](/docs/it/model-config) mostra i modelli nell'elenco di autorizzazione `availableModels` dello sviluppatore. Le impostazioni gestite si applicano all'avvio e si aggiornano ogni ora, e la telemetria si instrada al tuo raccoglitore.

Le sessioni si aggiornano silenziosamente prima della scadenza di `ttl_hours`. Quando un aggiornamento non riesce dopo il deprovision IdP, Claude Code richiede allo sviluppatore di accedere di nuovo.

<h3 id="set-the-gateway-url">
  Imposta l'URL del gateway
</h3>

Tre chiavi vanno nel file di [impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms) per OS che distribuisci tramite MDM o direttamente su disco. `forceLoginMethod` e `forceLoginGatewayUrl` aprono `/login` direttamente sulla schermata **Cloud gateway** con l'URL compilato, e `parentSettingsBehavior: "merge"` consente a Claude Desktop di consegnare l'elenco di autorizzazione di uscita del gateway alle sessioni di Claude Code che avvia, spiegato in [Consegna la politica alle sessioni di Claude Desktop](#deliver-policy-to-claude-desktop-sessions):

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

Lo sviluppatore preme Invio per connettersi. Il [prompt dell'impronta digitale TLS del primo collegamento](#connect-developers) appare ancora. Una volta che il file è su una macchina, uno sviluppatore che non ha completato l'accesso al gateway vede uno dei messaggi descritti in [La politica dell'amministratore richiede un accesso al Cloud gateway](/docs/it/errors#administrator-policy-requires-a-cloud-gateway-sign-in). Gli sviluppatori che selezionano un provider cloud attraverso una variabile di ambiente come `CLAUDE_CODE_USE_BEDROCK` non hanno bisogno dell'accesso al gateway.

Uno sviluppatore non può configurare questo manualmente. Il selettore di accesso non ha opzione gateway e `forceLoginGatewayUrl` viene ignorato nei file di impostazioni personali di uno sviluppatore. `forceLoginMethod` da solo, senza un URL, lascia lo sviluppatore con un messaggio "Contatta il tuo amministratore IT". Le chiavi di accesso appartengono al file che spingere alle macchine, non al blocco `managed.policies[].cli` del gateway, che raggiunge solo i client già connessi.

<h3 id="allow-a-gateway-on-public-address-space-you-own">
  Consenti un gateway su spazio di indirizzi pubblici che possiedi
</h3>

Alcune organizzazioni numerano la loro rete interna da un blocco IPv4 pubblico che possiedono, come lo spazio di indirizzi proprio di un operatore o un `/8` legacy, quindi il loro gateway non può avere un indirizzo privato. Elenca quei blocchi nell'impostazione gestita `gatewayInternalNetworks`. `/login` quindi accetta un gateway all'interno di un blocco elencato quando la macchina dello sviluppatore si connette ad esso da un indirizzo all'interno dello stesso blocco. Questo richiede Claude Code v2.1.268 o successivo sulla macchina dello sviluppatore; le versioni precedenti ignorano la chiave e applicano la regola dell'indirizzo privato.

<Warning>
  `gatewayInternalNetworks` è per le reti interne che accadono ad essere numerate da spazio di indirizzi pubblici. Non rende sicuro esporre un gateway a Internet: un gateway affidabile può spingere impostazioni che eseguono comandi sulle macchine degli sviluppatori.

  Mantieni il gateway irraggiungibile dall'esterno della tua rete con le regole del tuo firewall o del bilanciatore di carico. Imposta il [`access_control.allow_cidrs`](/docs/it/claude-apps-gateway-config#http-tuning) del gateway agli stessi blocchi che dichiari qui, quindi il gateway stesso rifiuta i client da qualsiasi altro luogo. Dietro un bilanciatore di carico o un ingresso, imposta anche `listen.trusted_proxies` a quel front end, perché il gateway altrimenti corrisponde a `allow_cidrs` rispetto all'indirizzo del front end stesso piuttosto che a quello dello sviluppatore.
</Warning>

Aggiungi la chiave alla stessa fonte di impostazioni gestite delle chiavi di accesso: il file di impostazioni gestite, il profilo MDM o la politica del registro. Claude Code la ignora nelle impostazioni gestite da utente, progetto e server.

Questo esempio dichiara un blocco. Sostituisci `203.0.113.0/24` con il tuo blocco. È un intervallo di documentazione e Claude Code rifiuta quelli.

```json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Claude Code convalida l'elenco a `/login` prima di contattare qualsiasi gateway:

* Ogni voce è un blocco IPv4 scritto come il suo primo indirizzo e un prefisso da `/8` a `/32`.
* L'elenco contiene al massimo quattro blocchi e nessuno si sovrappone.
* Nessun blocco si sovrappone allo spazio di indirizzi privati: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `127.0.0.0/8`, `169.254.0.0/16` e `100.64.0.0/10`. `/login` accetta già un gateway lì senza questa chiave.
* Nessun blocco si sovrappone allo spazio che non è mai la rete di un'organizzazione: `198.18.0.0/15` e `192.0.0.0/24`, che i client VPN e NAT64 mantengono come indirizzi locali; gli intervalli di documentazione `192.0.2.0/24`, `198.51.100.0/24` e `203.0.113.0/24`; e gli intervalli riservati `0.0.0.0/8`, `192.88.99.0/24` e multicast `224.0.0.0/4`. Puoi dichiarare blocchi all'interno di `240.0.0.0/4`, che alcune reti grandi utilizzano come spazio unicast interno.

I blocchi da `managed-settings.json` e i suoi file drop-in `managed-settings.d/` si combinano in un elenco e questi limiti si applicano all'elenco combinato. Per restringere un blocco, sostituisci la sua voce piuttosto che aggiungerne una seconda, sovrapposta in un drop-in; `/login` rifiuta la sovrapposizione.

Se una voce infrange una regola o il valore non è un elenco di stringhe, Claude Code rifiuta ogni nuovo accesso al gateway su quella macchina e nomina il problema nel messaggio. L'accesso a un gateway su un indirizzo privato fallisce anche e gli accessi esistenti continuano a funzionare. Prova il valore su una macchina prima di distribuirlo. Claude Code elenca anche un valore digitato erroneamente tra le [impostazioni gestite non valide che segnala](/docs/it/managed-settings#keys-that-fail-closed).

Con un elenco valido, `/login` applica tre controlli a un gateway il cui indirizzo è all'interno di un blocco elencato:

* Ogni indirizzo a cui il nome host del gateway si risolve è all'interno di quel blocco. Claude Code rifiuta un nome che ha anche record al di fuori di esso, indirizzi privati e IPv6 inclusi.
* La macchina dello sviluppatore si connette dall'interno dello stesso blocco. Claude Code rifiuta una macchina dietro NAT, all'interno di un contenitore o WSL2 o su una VPN il cui pool di indirizzi si trova al di fuori del blocco e nomina l'indirizzo da cui la macchina si è connessa.
* La connessione è diretta. Se `HTTPS_PROXY` si applica all'host del gateway, `/login` rifiuta e nomina la voce `NO_PROXY` da aggiungere.

Quando tutti e tre passano, il [prompt di fiducia](#connect-developers) aggiunge una riga che nomina l'indirizzo della macchina, l'indirizzo del gateway e il blocco dichiarato che contiene entrambi.

La chiave non cambia nulla per altri gateway: l'accesso a uno su un indirizzo privato funziona come prima e l'accesso a uno su un indirizzo pubblico al di fuori di ogni blocco elencato viene rifiutato come prima.

Un blocco dichiarato restringe chi può accedere ma non prova dove si trova una macchina, quindi dichiara solo spazio di indirizzi che la tua organizzazione controlla. Un blocco condiviso con altri tenant, come un intervallo pubblico di un provider cloud, lascia che chiunque al suo interno passi lo stesso controllo.

<h3 id="deliver-policy-to-claude-desktop-sessions">
  Consegna la politica alle sessioni di Claude Desktop
</h3>

Claude Desktop esegue le sue schede Cowork e Code, più la scheda Chat quando la abiliti, su sessioni di Claude Code incorporate e invia le loro richieste di modello attraverso il gateway. Passa la politica a ciascuna di quelle sessioni, costruita dalla configurazione che il gateway serve a `/user/bootstrap`: l'elenco di autorizzazione dei modelli, gli strumenti disabilitati e l'elenco di autorizzazione di uscita derivato dal blocco `cli` della politica corrispondente, più l'[overlay `desktop`](/docs/it/claude-apps-gateway-config#claude-desktop-overlay).

Altre chiavi `cli`, come hooks, `env` e regole di autorizzazione con ambito come `Bash(npm *)`, raggiungono solo i client che accedono tramite `/login`. Claude Desktop legge l'URL del gateway dalla sua propria configurazione gestita e accede con il suo proprio flusso, separato dalle chiavi `forceLoginMethod` e `forceLoginGatewayUrl` in [Imposta l'URL del gateway](#set-the-gateway-url).

Le impostazioni passate da un processo di avvio sono impostazioni padre. Claude Code ignora le impostazioni padre su qualsiasi macchina che ha una fonte gestita distribuita dall'amministratore, a meno che la [fonte che consegna la politica](/docs/it/managed-settings#which-managed-source-claude-code-uses) non imposti `parentSettingsBehavior: "merge"`.

<h4 id="which-machines-need-the-opt-in">
  Quali macchine hanno bisogno dell'opt-in
</h4>

Le macchine che eseguono solo Claude Desktop ne hanno bisogno. Claude Desktop applica l'elenco di modelli e l'elenco di strumenti disabilitati alle sessioni incorporate stesso, ma l'elenco di autorizzazione di uscita le raggiunge solo come impostazioni padre, sotto forma di regole di dominio `WebFetch` e regole di rete sandbox. Senza l'opt-in, quelle sessioni vengono eseguite senza la restrizione di uscita e nulla ti avverte. Il gateway rifiuta comunque le richieste di inferenza per i modelli che la politica non concede.

Le macchine in cui gli sviluppatori accedono tramite `/login` non ne hanno bisogno; ogni sessione di Claude Code recupera la sua politica dal gateway.

Le flotte il cui [`policyHelper`](/docs/it/settings-reference#policyhelper) fornisce impostazioni gestite non possono usarlo: Claude Code non unisce mai le impostazioni padre su quelle flotte, perché legge le impostazioni gestite solo dall'output dell'helper.

<h4 id="set-the-opt-in">
  Imposta l'opt-in
</h4>

Distribuisci lo snippet di impostazioni gestite da [Imposta l'URL del gateway](#set-the-gateway-url), specchialo su qualsiasi fonte lato client che supera il file, quindi verifica.

<Steps>
  <Step title="Distribuisci l'opt-in nel file di impostazioni gestite">
    Lo [snippet sopra](#set-the-gateway-url) include già `parentSettingsBehavior: "merge"`, quindi il file che spingere alle macchine lo contiene.
  </Step>

  <Step title="Specchia lo snippet su qualsiasi fonte che supera il file">
    Claude Code legge `parentSettingsBehavior` solo dalla [fonte selezionata](/docs/it/managed-settings#which-managed-source-claude-code-uses). Aggiungere qualsiasi chiave di politica a una fonte può rendere quella fonte quella selezionata, quindi in una fonte lato client, specchia l'intero snippet piuttosto che solo `parentSettingsBehavior`. [Le impostazioni gestite lato client](/docs/it/claude-apps-gateway-config#client-side-managed-settings) copre le flotte che consegnano la politica tramite Group Policy o profili di configurazione. Un plist di preferenze gestite su macOS o una politica HKLM su Windows supera il file `managed-settings.json` e le impostazioni gestite remote del gateway superano entrambi, quindi su macchine che accedono al gateway, imposta anche `parentSettingsBehavior` nel blocco [`cli`](/docs/it/claude-apps-gateway-config#managed) della politica del gateway.
  </Step>

  <Step title="Controlla quale fonte è selezionata">
    Su una macchina che esegue solo Claude Desktop, chiama il [`resolveSettings()`](/docs/it/agent-sdk/typescript#resolvesettings) dell'Agent SDK e leggi `policyOrigin` sulla voce `managed` nel suo elenco `sources`. Il valore nomina la fonte lato client selezionata, `plist`, `hklm` o `file`, che è la fonte che deve contenere lo snippet. Le sessioni incorporate di Claude Desktop non recuperano la politica del gateway, quindi il blocco `cli` del gateway non conta mai come fonte selezionata per loro.
  </Step>
</Steps>

<h3 id="restrict-parent-settings">
  Limita le impostazioni padre
</h3>

Una volta che distribuisci `parentSettingsBehavior: "merge"`, qualsiasi processo host che avvia Claude Code può fornire impostazioni padre, non solo Claude Desktop ma anche un'applicazione Agent SDK o un'estensione IDE.

Claude Code filtra le impostazioni padre rispetto a un elenco di autorizzazione di chiavi restrittive, ma alcune chiavi consentite possono concedere accesso piuttosto che limitarlo. A meno che non imposti i blocchi `allowManaged*Only`, le regole di autorizzazione di autorizzazione e gli elenchi di autorizzazione sandbox forniti dall'host si applicano comunque. Le regole di negazione e richiesta della tua politica rimangono in vigore comunque; [vengono valutate prima di qualsiasi regola di autorizzazione](/docs/it/permissions#manage-permissions).

Claude Code inoltra le voci [`sandbox.credentials`](/docs/it/settings-reference#sandbox-credentials) fornite dal padre in forma spogliata:

* **Voci `deny`**: inoltrate con solo il loro `path` o `name` e la modalità.
* **Voci di file con [`mode: mask`](/docs/it/sandboxing#mask-credential-files)**: inoltrate solo sentinel, come una maschera di file intero il cui `injectHosts` è l'elenco vuoto, quindi il proxy non sostituisce mai il valore reale per una voce fornita dal padre su nessuna piattaforma. Tutti i campi di mascheramento strutturato vengono eliminati anche, quindi un modello di estrazione fornito dal padre non può spostare una maschera più ristretta che un'altra fonte imposta per lo stesso percorso.
* **Voci `envVars` con `mode: mask`**: non inoltrate. `deny` è l'unica restrizione che il canale padre può esprimere attraverso le voci `envVars`.
* **[`awsPairs` e `sigv4`](/docs/it/sandboxing#re-sign-aws-requests)**: inoltrate solo restrizione. Da `sigv4`, vengono mantenuti solo i valori `deny` e un padre che definisce un blocco `sigv4` affatto blocca tutti e tre i moduli di richiesta, `streaming`, `presigned` e `sigv4a`, a `deny`. Una coppia `awsPairs` non viene mai inoltrata in una forma che può ri-firmare; una coppia che nomina una delle variabili AWS convenzionali viene sostituita da una voce inerte che mantiene l'accoppiamento automatico di `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN` soppresso.

<h4 id="deploy-the-locks">
  Distribuisci i blocchi
</h4>

Per mantenere le impostazioni padre il più vicino possibile a solo restrizione come il filtro supporta, aggiungi tutti e cinque i blocchi `allowManaged*Only` e gli elenchi di autorizzazione che governano, alle stesse fonti come l'opt-in di unione:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge",
  "allowManagedPermissionRulesOnly": true,
  "allowManagedMcpServersOnly": true,
  "allowManagedHooksOnly": true,
  "allowedMcpServers": [{ "serverUrl": "https://mcp.internal.example.com/*" }],
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    },
    "filesystem": {
      "allowManagedReadPathsOnly": true,
      "denyRead": ["~/"],
      "allowRead": ["~/projects"]
    }
  }
}
```

Una politica del sistema operativo, come una politica del registro HKLM o un plist di preferenze gestite, supera questo file, quindi consegna l'intero snippet attraverso di esso invece del file. Le impostazioni gestite remote del gateway superano le fonti della politica del sistema operativo e del file ma raggiungono solo i client connessi. Specchia i blocchi, gli elenchi di autorizzazione e l'opt-in di unione nel blocco [`cli`](/docs/it/claude-apps-gateway-config#managed) della politica e mantieni questo file distribuito, perché le macchine che non si connettono mai, incluse quelle che eseguono solo Claude Desktop, ottengono la loro politica dal file solo.

<h4 id="lock-behavior-across-sources">
  Comportamento del blocco tra le fonti
</h4>

Impostare un blocco non limita gli altri; ogni chiave è documentata nel [riferimento delle impostazioni](/docs/it/settings-reference#all-settings).

Da una fonte amministrativa sotto il vincitore, i due blocchi sandbox si applicano comunque e `allowManagedPermissionRulesOnly` blocca comunque le regole di autorizzazione fornite dal padre e `additionalDirectories`. Su Claude Code v2.1.273 o successivo, il blocco del server MCP si applica anche da una fonte sotto il vincitore e, mentre è attivo, l'elenco gestito `allowedMcpServers` proviene dalla fonte amministrativa con priorità più alta che ne imposta uno.

I blocchi hooks e `allowManagedPermissionRulesOnly` e l'effetto sulle regole personali dello sviluppatore hanno bisogno della fonte vincente per impostazione predefinita; secondo l'opt-in di unione `managedSourcesBehavior` in [come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources), Claude Code applica il valore più rigoroso che qualsiasi fonte imposta per ogni blocco. Sulle flotte [`policyHelper`](/docs/it/settings-reference#policyhelper), i blocchi vengono letti solo dall'output dell'helper.

Ogni blocco fa sì che Claude Code ignori le voci personali dello sviluppatore per quella impostazione, quindi includi gli elenchi di autorizzazione della tua organizzazione accanto ai blocchi:

* **Domini di rete**: bloccare con un elenco di dominio gestito vuoto blocca tutto il traffico in uscita in sandbox.
* **Server MCP**: bloccare senza `allowedMcpServers` gestito in nessuna fonte amministrativa o nelle impostazioni fornite dal padre carica ogni server che `deniedMcpServers` non blocca.
* **Percorsi di lettura**: le voci `allowRead` solo ri-consentono i percorsi all'interno delle regioni `denyRead`, quindi abbinale a un `denyRead` gestito.

<h4 id="settings-the-locks-don’t-cover">
  Impostazioni che i blocchi non coprono
</h4>

Sei impostazioni fornite dal padre passano il filtro anche con tutti e cinque i blocchi impostati. Secondo l'impostazione predefinita first-wins, il valore amministrativo che blocca quello del padre è quello nella fonte amministrativa con priorità più alta, tranne per `allowedMcpServers` mentre il [blocco del server MCP](#lock-behavior-across-sources) è attivo. Secondo l'opt-in di unione `managedSourcesBehavior`, [come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) dice quale valore della fonte si applica invece.

* **`forceLoginOrgUUID`**: Claude Code onora un valore fornito dal padre quando la fonte amministrativa con priorità più alta non imposta un UUID org. L'accesso al gateway non controlla questa chiave, quindi importa solo per le flotte che utilizzano anche accessi Anthropic di prima parte. Un UUID org nella fonte amministrativa con priorità più alta blocca il valore del padre ed è quello che Claude Code applica, quindi imposta `forceLoginOrgUUID` lì.
* **`allowedMcpServers`**: Claude Code onora un elenco di autorizzazione fornito dal padre quando nessuna fonte amministrativa ne imposta uno. `allowManagedMcpServersOnly` non lo blocca, perché il blocco applica qualunque elenco vinca come valore gestito, incluso un elenco fornito dal padre quando nessuna fonte amministrativa ne imposta uno. Un elenco nella fonte amministrativa con priorità più alta blocca quello del padre ed è l'elenco che Claude Code applica, quindi imposta `allowedMcpServers` lì, accanto al blocco. Prima della v2.1.223, un valore per una delle due chiavi in qualsiasi fonte amministrativa bloccava quello del padre.
* **`availableModels`**: Claude Code onora un elenco di modelli fornito dal padre quando la fonte gestita vincente non ne imposta uno. Se la tua flotta limita i modelli, imposta `availableModels` nella fonte vincente.
* **`strictKnownMarketplaces`**: Claude Code onora un elenco di autorizzazione del marketplace di plugin fornito dal padre quando la fonte gestita vincente non ne imposta uno. Se la tua flotta limita i marketplace, imposta `strictKnownMarketplaces` nella fonte vincente. Richiede Claude Code v2.1.282 o successivo.
* **`blockedMarketplaces`**: un elenco di blocco del marketplace fornito dal padre passa e si aggiunge a qualsiasi elenco di blocco che una fonte gestita imposta, poiché un elenco di blocco può solo limitare ulteriormente. Richiede Claude Code v2.1.282 o successivo.
* **`strictPluginOnlyCustomization`**: questa chiave passa il filtro indipendentemente da qualsiasi blocco e fa sì che Claude Code ignori la personalizzazione personale dello sviluppatore, inclusi gli hooks protettivi. Nessun blocco lo blocca.

<h3 id="connect-claude-desktop">
  Connetti Claude Desktop
</h3>

[Claude Desktop](/docs/it/desktop) si connette allo stesso gateway attraverso una chiave MDM diversa: imposta `bootstrapUrl` nella [configurazione gestita](https://claude.com/docs/third-party/claude-desktop/configuration) di Claude Desktop a `<listen.public_url>/user/bootstrap` e opt-in della politica dell'utente con una chiave `desktop`. [L'overlay di Claude Desktop](/docs/it/claude-apps-gateway-config#claude-desktop-overlay) copre entrambe le metà. Richiede Claude Code v2.1.203 o successivo sul server del gateway.

Claude Desktop firma lo sviluppatore attraverso il provider di identità del gateway con lo stesso passaggio SSO del browser, quindi recupera la sua configurazione dal gateway invece che da Anthropic. L'accesso ai modelli e la politica seguono le stesse regole per gruppo come il CLI. Uno sviluppatore che utilizza sia il CLI che Claude Desktop accede a ciascuno separatamente; la sessione del gateway non è condivisa tra loro.

Una volta connesso, Claude Desktop invia richieste di modello da ogni scheda abilitata attraverso il gateway. Mostra le schede Cowork e Code per impostazione predefinita. Per attivare anche la scheda Chat, imposta `chatTabEnabled` a `true` nella [configurazione gestita](https://claude.com/docs/third-party/claude-desktop/configuration) di Claude Desktop o nel blocco [`desktop`](/docs/it/claude-apps-gateway-config#claude-desktop-overlay) della politica su un gateway che esegue Claude Code v2.1.227 o successivo.

<h3 id="ci-pipelines-and-remote-machines">
  Pipeline CI e macchine remote
</h3>

Non c'è flusso di token di servizio per pipeline non presenziate. L'accesso al gateway esegue sempre il flusso del dispositivo del browser, quindi un lavoro CI senza uno sviluppatore per approvare l'accesso non può autenticarsi; configura quelli direttamente contro il tuo provider.

Una volta che uno sviluppatore ha effettuato l'accesso, ogni sessione di Claude Code su quella macchina utilizza la sessione del gateway, incluse le esecuzioni non interattive `claude -p` e le sessioni avviate da Agent SDK. Claude Code applica la [politica del gateway](/docs/it/claude-apps-gateway-config#managed) a ciascuna di esse.

Il flusso del dispositivo separa il CLI di polling dal browser di approvazione, quindi una scatola di sviluppo remoto senza display funziona ancora: lo sviluppatore esegue `/login` su SSH sulla macchina remota e apre il link di verifica nel browser sul suo laptop.

<h3 id="whats-enforced-on-developers">
  Cosa viene applicato agli sviluppatori
</h3>

Queste garanzie si applicano a ogni sessione connessa tramite `/login`. Le sessioni incorporate che Claude Desktop avvia ottengono la loro politica come descritto in [Consegna la politica alle sessioni di Claude Desktop](#deliver-policy-to-claude-desktop-sessions) e il punto di telemetria dice dove vanno le loro esportazioni.

* **Accesso ai modelli**: le richieste per modelli che la politica non concede restituiscono 400 e il selettore `/model` viene filtrato all'elenco di autorizzazione `availableModels` della politica. Imposta [`enforceAvailableModels: true`](/docs/it/model-config#default-model-behavior) nella politica in modo che l'opzione Predefinita si risolva in un modello all'interno di `availableModels` invece che al valore predefinito integrato di Claude Code; senza di esso, Predefinita rimane selezionabile e viene rifiutata al momento della richiesta se quel modello non è concesso.
* **Destinazione telemetria**: nelle sessioni connesse tramite `/login`, il CLI invia le sue esportazioni OTLP/HTTP al gateway piuttosto che a un `OTEL_EXPORTER_OTLP_ENDPOINT` impostato localmente, a meno che una politica non [nomini il tuo raccoglitore come endpoint](/docs/it/claude-apps-gateway-config#export-directly-to-your-collector). Il gateway inoltra le esportazioni che riceve alle destinazioni in [`telemetry.forward_to`](/docs/it/claude-apps-gateway-config#telemetry).
  * Nelle sessioni incorporate che [Claude Desktop avvia](#connect-claude-desktop), il CLI invia le sue esportazioni all'`OTEL_EXPORTER_OTLP_ENDPOINT` configurato. Il CLI allega il token di sessione del gateway a quelle esportazioni solo quando quell'endpoint punta al gateway stesso.
  * Senza una destinazione configurata per un segnale, il gateway lo accetta e lo scarta.
  * Se raccogli già la telemetria di Claude Code direttamente, aggiungi il tuo raccoglitore come destinazione `forward_to` o nominalo in una politica per saltare il relay.
* **Credenziali**: il token del gateway è l'unica credenziale della sessione. [Profili Anthropic](/docs/it/authentication#anthropic-profiles-and-federation-credentials) e qualsiasi accesso precedente a claude.ai vengono ignorati mentre connesso, quindi gli sviluppatori non hanno bisogno di disconnettersi da claude.ai per primo. Per una credenziale `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` configurata, vedi [La politica dell'amministratore richiede un accesso al Cloud gateway](/docs/it/errors#administrator-policy-requires-a-cloud-gateway-sign-in).
* **Impostazioni gestite**: le chiavi bloccate non possono essere ignorate localmente. Il CLI applica la politica all'avvio e applica le modifiche su ogni sondaggio orario, a parte le [modifiche che si applicano solo al prossimo avvio](/docs/it/server-managed-settings#fetch-and-caching-behavior).
* **Avvio con il gateway irraggiungibile**: le sessioni connesse escono all'avvio con un errore dopo circa 10 secondi piuttosto che iniziare senza le loro impostazioni.
* **Avvio dopo che il gateway termina la sessione**: vedi [Applica fail-closed startup](/docs/it/server-managed-settings#enforce-fail-closed-startup) per i lanci che si aprono disconnessi dal gateway e quelli che escono quando il gateway risponde con un `401`.
* **Deprovision**: una sessione il cui utente è disabilitato nell'IdP scade entro `ttl_hours` quando il prossimo aggiornamento fallisce.
* **Sign-out**: `/logout` elimina la credenziale del gateway dalla macchina dello sviluppatore.
  * Quando il documento di scoperta del gateway pubblicizza un `revocation_endpoint` sullo schema, host e porta dell'URL del gateway, `/logout` invia anche i token archiviati a quell'endpoint in modo che il gateway possa terminare la sessione dal suo lato. La richiesta è best effort, quindi il sign-out si completa sulla macchina dello sviluppatore indipendentemente dal fatto che l'endpoint risponda. La revoca richiede Claude Code v2.1.275 o successivo sulla macchina dello sviluppatore.
  * Il server del gateway nel binario `claude` non ne pubblicizza nessuno, quindi un sign-out da esso termina la sessione sulla macchina dello sviluppatore solo. Per forzare le sessioni fuori lato server, vedi [Rotazione della chiave JWT](/docs/it/claude-apps-gateway-deploy#jwt-secret-rotation).

<h3 id="what-the-organization-can-see">
  Cosa può vedere l'organizzazione
</h3>

La telemetria di utilizzo porta l'identità dello sviluppatore, i conteggi di token, il modello e la latenza al raccoglitore dell'organizzazione. Il gateway non registra o archivia il contenuto del prompt o del completamento. Se viene raccolta una telemetria più ricca come log e tracce, che può includere comandi e percorsi di file, è la [scelta per destinazione](/docs/it/claude-apps-gateway-config#telemetry) dell'organizzazione.

<h2 id="availability-and-limitations">
  Disponibilità e limitazioni
</h2>

La tabella copre quali funzionalità di Claude Code funzionano quando gli sviluppatori si connettono attraverso il gateway e cosa supporta il server gateway stesso. Dove qualcosa non è supportato, la colonna Note fornisce l'alternativa.

Il gateway consegna i valori [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) che il CLI invia a ogni upstream, quindi gli operatori non mantengono un elenco di autorizzazioni beta. Per Amazon Bedrock, che ignora l'intestazione, il gateway sposta i valori nel campo `anthropic_beta` del corpo della richiesta; gli altri upstream ricevono l'intestazione come inviata.

| Funzionalità                                                                                                                | Stato                              | Note                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| --------------------------------------------------------------------------------------------------------------------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Inoltro di inferenza (Amazon Bedrock, Claude Platform su AWS, Agent Platform di Google Cloud, Microsoft Foundry, Anthropic) | Disponibile                        | Con traduzione del modello per upstream e failover. L'upstream Amazon Bedrock utilizza l'endpoint `bedrock-runtime` e la catena di credenziali predefinita AWS; l'[endpoint Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint) di Amazon Bedrock non è un upstream supportato. L'[upstream Claude Platform su AWS](/docs/it/claude-apps-gateway-config#claude-platform-on-aws) richiede Claude Code v2.1.198 o successivo sul server gateway.                                                            |
| Accesso ai modelli e impostazioni gestite per gruppo IdP                                                                    | Disponibile                        | L'accesso ai modelli viene applicato lato server; le impostazioni gestite vengono consegnate per gruppo IdP e applicate dal CLI al [livello di impostazioni gestite](/docs/it/settings#settings-precedence)                                                                                                                                                                                                                                                                                               |
| Claude Desktop                                                                                                              | Disponibile con consenso esplicito | Il gateway fornisce la configurazione di Claude Desktop su `/user/bootstrap` una volta che una politica [acconsente con una chiave `desktop`](/docs/it/claude-apps-gateway-config#claude-desktop-overlay), e Claude Desktop invia richieste di modello dalle sue schede Cowork e Code, e dalla scheda Chat quando la abiliti, attraverso il gateway. Per attivare la scheda Chat, vedi [Connetti Claude Desktop](#connect-claude-desktop). Richiede Claude Code v2.1.203 o successivo sul server gateway. |
| Fan-out di telemetria (OTLP/HTTP)                                                                                           | Disponibile                        | Identità-timbrato per esportazione; entrambe le codifiche protobuf e JSON                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Provider di identità OIDC                                                                                                   | Disponibile                        | Qualsiasi IdP conforme a OIDC; il gateway esegue il discovery OIDC standard e il flusso del codice di autorizzazione. Vedi [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup) per la configurazione per IdP                                                                                                                                                                                                                                           |
| Limiti di spesa per utente e per gruppo                                                                                     | Disponibile                        | Vedi [Limiti di spesa](/docs/it/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Ricerca web lato server                                                                                                     | Non disponibile                    | Il CLI non può vedere quale provider upstream il gateway instrada, quindi non può verificare il supporto della ricerca web e disabilita WebSearch sulle sessioni del gateway                                                                                                                                                                                                                                                                                                                         |
| [Remote Control](/docs/it/remote-control)                                                                                        | Non disponibile                    | Il CLI mostra [un errore che nomina il gateway](/docs/it/errors#remote-control-requires-the-anthropic-api)                                                                                                                                                                                                                                                                                                                                                                                                |
| [`/design-sync`](/docs/it/commands#all-commands) e `/design-login`                                                               | Non disponibile                    | Entrambi hanno bisogno di claude.ai, che il CLI non contatta sulle sessioni del gateway, quindi nessuno dei due comandi appare lì                                                                                                                                                                                                                                                                                                                                                                    |
| Funzionalità che necessitano di recupero dei flag di funzionalità, come `/import` e `claude import`                         | Non disponibile                    | Il CLI salta il recupero dei flag sulle sessioni del gateway. [Funzionalità che necessitano di recupero dei flag di funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching) elenca cosa questo disattiva                                                                                                                                                                                                                                                                                |
| Caching del prompt standard                                                                                                 | Disponibile                        | Il gateway inoltri i breakpoint `cache_control` a ogni upstream. [Dove risiede la cache](/docs/it/prompt-caching#where-the-cache-lives) copre quali blocchi il CLI contrassegna, incluso il contesto di sistema che aggiunge a metà conversazione                                                                                                                                                                                                                                                         |
| TTL cache di 1 ora                                                                                                          | Non disponibile                    | Il CLI omette il beta della cache estesa sulle sessioni del gateway, perché non ogni upstream a cui il gateway può instradare supporta il TTL di 1 ora, quindi il caching del prompt attraverso il gateway utilizza il TTL di 5 minuti; vedi la nota dell'intestazione beta sopra                                                                                                                                                                                                                    |
| Modalità Auto                                                                                                               | Disponibile                        | Segue le [regole del provider di terze parti](/docs/it/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): solo i modelli idonei sui provider di terze parti possono usarlo. Prima della v2.1.207, la modalità auto sulle sessioni del gateway richiedeva l'impostazione di `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, consegnabile tramite il blocco `env` della politica gestita                                                                                                            |
| Ottimizzazioni solo per la prima parte come ambito cache globale e strumenti efficienti in termini di token                 | Non disponibile                    | Il CLI non le abilita sulle sessioni del gateway; vedi la nota dell'intestazione beta sopra                                                                                                                                                                                                                                                                                                                                                                                                          |
| OTLP/gRPC                                                                                                                   | Non supportato                     | OTLP su HTTP solo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| SAML, LDAP e altri auth non OIDC                                                                                            | Non supportato                     | Solo OIDC. Fronte con un ponte OIDC se necessario                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Multi-tenant (più emittenti OIDC)                                                                                           | Non supportato                     | Un emittente per gateway. Esegui istanze separate                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Server Windows                                                                                                              | Non supportato                     | Distribuisci su Linux. macOS solo per lo sviluppo locale                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Helm chart                                                                                                                  | Non disponibile                    | Il gateway viene eseguito come una Deployment stateless standard; vedi la [guida di distribuzione](/docs/it/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                                                                        |
| Interfaccia utente amministratore                                                                                           | Non disponibile                    | La configurazione è il file YAML; ridistribuisci per cambiarla                                                                                                                                                                                                                                                                                                                                                                                                                                       |

<h2 id="next-steps">
  Passaggi successivi
</h2>

La guida rapida ti lascia con una configurazione minima in esecuzione sotto Docker Compose. Per andare oltre:

* Espandi `gateway.yaml` oltre la configurazione minima, ad esempio per aggiungere RBAC per gruppo, failover multi-upstream o destinazioni di telemetria. Il [riferimento di configurazione](/docs/it/claude-apps-gateway-config) copre ogni opzione.
* Passa da Compose a una distribuzione di produzione su Kubernetes o Cloud Run, configura correttamente il tuo IdP e rivedi il modello di sicurezza. La [guida di distribuzione e operazioni](/docs/it/claude-apps-gateway-deploy) copre la configurazione per IdP, i requisiti dell'immagine container, i probe di salute e la risoluzione dei problemi.
* Metti limiti di spesa su singoli sviluppatori o gruppi in modo che un carico di lavoro incontrollato non possa consumare il tuo intero impegno. [Limiti di spesa](/docs/it/claude-apps-gateway-spend-limits) copre l'API amministratore e come funziona l'applicazione.
* Per un esempio completo su AWS, con ECS Fargate o EKS, Amazon RDS e Secrets Manager, vedi [Distribuisci su AWS](/docs/it/claude-apps-gateway-on-aws).
* Per un esempio completo su Google Cloud, con Cloud Run, Cloud SQL e Secret Manager, vedi [Distribuisci su Google Cloud](/docs/it/claude-apps-gateway-on-gcp).
