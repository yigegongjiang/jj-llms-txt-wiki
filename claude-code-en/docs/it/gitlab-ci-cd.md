> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Scopri come integrare Claude Code nel tuo flusso di lavoro di sviluppo con GitLab CI/CD

<Info>
  Claude Code per GitLab CI/CD è attualmente in beta. Le funzionalità e la funzionalità possono evolversi mentre perfezzioniamo l'esperienza.

  Questa integrazione è mantenuta da GitLab. Per il supporto, consultare il seguente [problema GitLab](https://gitlab.com/gitlab-org/gitlab/-/issues/573776).
</Info>

<Note>
  Questa integrazione è costruita sulla base di [Claude Code CLI e Agent SDK](/docs/it/agent-sdk/overview), consentendo l'uso programmatico di Claude nei vostri lavori CI/CD e flussi di lavoro di automazione personalizzati.
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  Perché utilizzare Claude Code con GitLab?
</h2>

* **Creazione istantanea di MR**: Descrivete ciò di cui avete bisogno e Claude propone un MR completo con modifiche e spiegazione
* **Implementazione automatizzata**: Trasformate i problemi in codice funzionante con un singolo comando o menzione
* **Consapevole del progetto**: Claude segue le vostre linee guida `CLAUDE.md` e i modelli di codice esistenti
* **Configurazione semplice**: Aggiungete un lavoro a `.gitlab-ci.yml` e una variabile CI/CD mascherata
* **Pronto per l'azienda**: Scegliete Claude API, Amazon Bedrock o Google Cloud's Agent Platform per soddisfare le esigenze di residenza dei dati e approvvigionamento
* **Sicuro per impostazione predefinita**: Viene eseguito nei vostri runner GitLab con la vostra protezione dei rami e approvazioni

<h2 id="how-it-works">
  Come funziona
</h2>

Claude Code utilizza GitLab CI/CD per eseguire attività di intelligenza artificiale in lavori isolati e eseguire il commit dei risultati tramite MR:

1. **Orchestrazione basata su eventi**: GitLab ascolta i trigger scelti (ad esempio, un commento che menziona `@claude` in un problema, MR o thread di revisione). Il lavoro raccoglie il contesto dal thread e dal repository, costruisce prompt da tale input ed esegue Claude Code.

2. **Astrazione del provider**: Utilizzate il provider che si adatta al vostro ambiente:
   * Claude API (SaaS)
   * Amazon Bedrock (accesso basato su IAM, opzioni multi-regione)
   * Google Cloud's Agent Platform (nativo GCP, Workload Identity Federation)

3. **Esecuzione in sandbox**: Ogni interazione viene eseguita in un contenitore con regole rigorose di rete e filesystem. Claude Code applica autorizzazioni con ambito workspace per limitare le scritture. Ogni modifica passa attraverso un MR in modo che i revisori vedano il diff e le approvazioni si applichino ancora.

Scegliete endpoint regionali per ridurre la latenza e soddisfare i requisiti di sovranità dei dati mentre utilizzate gli accordi cloud esistenti.

<h2 id="what-can-claude-do">
  Cosa può fare Claude?
</h2>

In una pipeline GitLab, Claude Code può:

* Creare e aggiornare MR da descrizioni di problemi o commenti
* Analizzare regressioni di prestazioni e proporre ottimizzazioni
* Implementare funzionalità direttamente in un ramo, quindi aprire un MR
* Correggere bug e regressioni identificati da test o commenti
* Rispondere ai commenti di follow-up per iterare sulle modifiche richieste

<h2 id="setup">
  Configurazione
</h2>

<h3 id="quick-setup">
  Configurazione rapida
</h3>

Il modo più veloce per iniziare è aggiungere un job minimo al vostro `.gitlab-ci.yml` e impostare la vostra chiave API come variabile mascherata.

1. **Aggiungere una variabile CI/CD mascherata**
   * Andare a **Settings** → **CI/CD** → **Variables**
   * Aggiungere `ANTHROPIC_API_KEY` (mascherata, protetta secondo le necessità)

2. **Aggiungere un job Claude a `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Regolate le regole per adattarle a come desiderate attivare il job:
  # - esecuzioni manuali
  # - eventi di merge request
  # - trigger web/API quando un commento contiene '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # L'installer posiziona claude in ~/.local/bin, che non è su PATH in questa immagine
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # Opzionale: avviare un server GitLab MCP se la vostra configurazione ne fornisce uno
    - /bin/gitlab-mcp-server || true
    # Utilizzare le variabili AI_FLOW_* quando si invoca tramite trigger web/API con payload di contesto
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Dopo aver aggiunto il job e la vostra variabile `ANTHROPIC_API_KEY`, testate eseguendo il job manualmente da **CI/CD** → **Pipelines**, oppure attivate il job da un MR per consentire a Claude di proporre aggiornamenti in un ramo e aprire un MR se necessario.

<Note>
  Per eseguire su Amazon Bedrock o su Google Cloud's Agent Platform invece dell'API Claude, consultate la sezione [Using with Amazon Bedrock and Google Cloud](#using-with-amazon-bedrock-and-google-cloud) di seguito per l'autenticazione e la configurazione dell'ambiente.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Configurazione manuale (consigliata per la produzione)
</h3>

Se preferite una configurazione più controllata o avete bisogno di provider aziendali:

1. **Configurare l'accesso al provider**:
   * **Claude API**: Creare e memorizzare `ANTHROPIC_API_KEY` come variabile CI/CD mascherata
   * **Amazon Bedrock**: **Configure GitLab** → **AWS OIDC** e creare un ruolo IAM per Amazon Bedrock
   * **Google Cloud's Agent Platform**: **Configure Workload Identity Federation for GitLab** → **GCP**

2. **Aggiungere credenziali di progetto per le operazioni dell'API GitLab**:
   * Utilizzare `CI_JOB_TOKEN` per impostazione predefinita, oppure creare un Project Access Token con ambito `api`
   * Memorizzare come `GITLAB_ACCESS_TOKEN` (mascherato) se si utilizza un PAT

3. **Aggiungere il job Claude a `.gitlab-ci.yml`**: utilizzare il job [Quick setup](#quick-setup) per l'API Claude, oppure un job provider da [Configuration examples](#configuration-examples)

4. **(Opzionale) Abilitare trigger basati su menzioni**:
   * Aggiungere un webhook di progetto per "Comments (notes)" al vostro listener di eventi (se ne utilizzate uno)
   * Far sì che il listener chiami l'API di attivazione della pipeline con variabili come `AI_FLOW_INPUT` e `AI_FLOW_CONTEXT` quando un commento contiene `@claude`

<h2 id="example-use-cases">
  Esempi di casi d'uso
</h2>

<h3 id="turn-issues-into-mrs">
  Trasformare i problemi in MR
</h3>

In un commento di un problema:

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude analizza il problema e la base di codice, scrive le modifiche in un ramo e apre un MR per la revisione.

<h3 id="get-implementation-help">
  Ottenere aiuto nell'implementazione
</h3>

In una discussione di un MR:

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude propone modifiche, aggiunge codice con caching appropriato e aggiorna l'MR.

<h3 id="fix-bugs-quickly">
  Correggere i bug rapidamente
</h3>

In un commento di un problema o di un MR:

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude individua il bug, implementa una correzione e aggiorna il ramo o apre un nuovo MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Utilizzo con Amazon Bedrock e Google Cloud
</h2>

Per gli ambienti aziendali, è possibile eseguire Claude Code interamente sull'infrastruttura cloud con la stessa esperienza di sviluppo.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Prerequisiti

    Prima di configurare Claude Code con Amazon Bedrock, è necessario disporre di:

    1. Un account AWS con accesso ad Amazon Bedrock per i modelli Claude desiderati
    2. GitLab configurato come provider di identità OIDC in AWS IAM
    3. Un ruolo IAM con autorizzazioni Amazon Bedrock e una politica di trust limitata al progetto/refs di GitLab
    4. Variabili CI/CD di GitLab per l'assunzione del ruolo:
       * `AWS_ROLE_TO_ASSUME` (ARN del ruolo)
       * `AWS_REGION` (regione Amazon Bedrock)

    ### Istruzioni di configurazione

    Configurare AWS per consentire ai job CI di GitLab di assumere un ruolo IAM tramite OIDC (senza chiavi statiche).

    **Configurazione richiesta:**

    1. Abilitare Amazon Bedrock e richiedere l'accesso ai modelli Claude di destinazione
    2. Creare un provider OIDC IAM per GitLab se non già presente
    3. Creare un ruolo IAM considerato attendibile dal provider OIDC di GitLab, limitato al progetto e ai refs protetti
    4. Allegare autorizzazioni con privilegi minimi per le API di invocazione di Amazon Bedrock

    Utilizzare l'[esempio di job Amazon Bedrock](#configuration-examples) per scambiare il token OIDC del job con credenziali AWS temporanee in fase di esecuzione.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Prerequisiti

    Prima di configurare Claude Code con Google Cloud's Agent Platform, è necessario disporre di:

    1. Un progetto Google Cloud con:
       * API di Google Cloud's Agent Platform abilitata
       * Workload Identity Federation configurata per considerare attendibile OIDC di GitLab
    2. Un account di servizio dedicato con solo i ruoli Google Cloud's Agent Platform richiesti
    3. Variabili CI/CD di GitLab:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (nome della risorsa provider senza il prefisso `//iam.googleapis.com/`, ad esempio `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`)
       * `GCP_SERVICE_ACCOUNT` (email dell'account di servizio)
       * `GCP_PROJECT_ID` (ID del progetto Google Cloud)

    ### Istruzioni di configurazione

    Configurare Google Cloud per consentire ai job CI di GitLab di rappresentare un account di servizio tramite Workload Identity Federation.

    **Configurazione richiesta:**

    1. Abilitare l'API IAM Credentials, l'API STS e l'API di Google Cloud's Agent Platform
    2. Creare un Workload Identity Pool e un provider per OIDC di GitLab
    3. Creare un account di servizio dedicato con i ruoli di Google Cloud's Agent Platform
    4. Concedere al principale WIF l'autorizzazione per rappresentare l'account di servizio

    Utilizzare l'[esempio di job Agent Platform](#configuration-examples) per autenticarsi senza archiviare le chiavi.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Esempi di configurazione
</h2>

Di seguito sono riportati frammenti pronti all'uso che potete adattare alla vostra pipeline.

<h3 id="amazon-bedrock-job-example-oidc">
  Esempio di job Amazon Bedrock (OIDC)
</h3>

**Prerequisiti:**

* Amazon Bedrock abilitato con accesso al vostro modello Claude scelto
* OIDC di GitLab configurato in AWS con un ruolo che si fida del vostro progetto GitLab e dei vostri ref
* Ruolo IAM con autorizzazioni Amazon Bedrock (si consiglia il principio del minimo privilegio)

**Variabili CI/CD richieste:**

* `AWS_ROLE_TO_ASSUME`: ARN del ruolo IAM per l'accesso ad Amazon Bedrock
* `AWS_REGION`: regione Amazon Bedrock (ad esempio, `us-west-2`)

GitLab crea il token OIDC del job dal blocco `id_tokens:` e lo espone come `GITLAB_OIDC_TOKEN`. Impostare `aud` al valore di audience che avete configurato sul provider di identità OIDC IAM in AWS, ad esempio l'URL della vostra istanza GitLab.

```yaml theme={null}
stages:
  - ai

claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apk add --no-cache bash curl jq git aws-cli
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Exchange the job's OIDC token for AWS credentials
    - export AWS_WEB_IDENTITY_TOKEN_FILE="/tmp/oidc_token"
    - printf "%s" "$GITLAB_OIDC_TOKEN" > "$AWS_WEB_IDENTITY_TOKEN_FILE"
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
    CLAUDE_CODE_USE_BEDROCK: "1"
```

<Note>
  Gli ID modello per Amazon Bedrock includono prefissi specifici della regione (ad esempio, `us.anthropic.claude-sonnet-4-6`). Passate il modello desiderato tramite la configurazione del vostro job o il prompt se il vostro flusso di lavoro lo supporta.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Esempio di job Agent Platform (Workload Identity Federation)
</h3>

**Prerequisiti:**

* API Agent Platform di Google Cloud abilitata nel vostro progetto GCP
* Workload Identity Federation configurata per fidarsi di OIDC di GitLab
* Un account di servizio con autorizzazioni Agent Platform di Google Cloud

**Variabili CI/CD richieste:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: nome della risorsa provider senza il prefisso `//iam.googleapis.com/`, ad esempio `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`
* `GCP_SERVICE_ACCOUNT`: email dell'account di servizio
* `GCP_PROJECT_ID`: ID progetto Google Cloud
* `CLOUD_ML_REGION`: regione Agent Platform di Google Cloud (ad esempio, `us-east5`)

GitLab crea il token OIDC del job dal blocco `id_tokens:` e lo espone come `GITLAB_OIDC_TOKEN`. Impostare `aud` al valore di audience che avete configurato sul provider del Workload Identity Pool, ad esempio l'URL della vostra istanza GitLab. Il job scrive il token in un file e la voce `credential_source` della configurazione delle credenziali dice alle librerie di autenticazione di Google di leggerlo da lì. Impostare `GOOGLE_APPLICATION_CREDENTIALS` al file di configurazione delle credenziali lo rende disponibile a Claude Code tramite [Application Default Credentials](/docs/it/google-vertex-ai#3-configure-gcp-credentials).

```yaml theme={null}
stages:
  - ai

claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Write the job's OIDC token where credential_source expects it
    - printf "%s" "$GITLAB_OIDC_TOKEN" > /tmp/oidc_token
    # Write the WIF credential configuration to a file (no downloaded keys)
    - |
      cat > /tmp/cred.json <<EOF
      {
        "type": "external_account",
        "audience": "//iam.googleapis.com/${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "token_url": "https://sts.googleapis.com/v1/token",
        "credential_source": {
          "file": "/tmp/oidc_token"
        },
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken"
      }
      EOF
    # Expose the credentials to Claude Code via Application Default Credentials
    - export GOOGLE_APPLICATION_CREDENTIALS=/tmp/cred.json
    # Authenticate the gcloud CLI with the same credential configuration
    - gcloud auth login --cred-file=/tmp/cred.json
    - gcloud config set project "$GCP_PROJECT_ID"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
    CLAUDE_CODE_USE_VERTEX: "1"
    ANTHROPIC_VERTEX_PROJECT_ID: "$GCP_PROJECT_ID"
```

<Note>
  Con Workload Identity Federation, non è necessario archiviare le chiavi dell'account di servizio. Utilizzate condizioni di fiducia specifiche del repository e account di servizio con privilegi minimi.
</Note>

<h2 id="best-practices">
  Best practices
</h2>

<h3 id="claude-md-configuration">
  Configurazione CLAUDE.md
</h3>

Creare un file `CLAUDE.md` nella radice del repository per definire gli standard di codifica, i criteri di revisione e le regole specifiche del progetto. Claude legge questo file durante le esecuzioni e segue le vostre convenzioni quando propone modifiche.

<h3 id="security-considerations">
  Considerazioni sulla sicurezza
</h3>

**Non eseguite mai il commit di chiavi API o credenziali cloud nel vostro repository**. Utilizzate sempre le variabili di GitLab CI/CD:

* Aggiungete `ANTHROPIC_API_KEY` come variabile mascherata (e proteggetela se necessario)
* Utilizzate OIDC specifico del provider dove possibile (nessuna chiave di lunga durata)
* Limitate i permessi dei job e l'uscita di rete
* Revisionate i MR di Claude come qualsiasi altro contributore

<h3 id="optimizing-performance">
  Ottimizzazione delle prestazioni
</h3>

* Mantenete `CLAUDE.md` focalizzato e conciso
* Fornite descrizioni chiare di issue/MR per ridurre le iterazioni
* Memorizzate nella cache npm e le installazioni di pacchetti nei runner dove possibile

<h3 id="ci-costs">
  Costi di CI
</h3>

Quando utilizzate Claude Code con GitLab CI/CD, siate consapevoli dei costi associati:

* **Tempo di GitLab Runner**:
  * Claude viene eseguito sui vostri runner GitLab e consuma minuti di calcolo
  * Consultate i dettagli di fatturazione del runner del vostro piano GitLab

* **Costi API**:
  * Ogni interazione di Claude consuma token in base alla dimensione del prompt e della risposta
  * L'utilizzo dei token varia in base alla complessità dell'attività e alla dimensione della codebase
  * Consultate [Anthropic pricing](https://platform.claude.com/docs/en/about-claude/pricing) per i dettagli

* **Suggerimenti per l'ottimizzazione dei costi**:
  * Utilizzate comandi `@claude` specifici per ridurre i turni non necessari
  * Impostate valori appropriati di `--max-turns` e `timeout` del job
  * Limitate la concorrenza per controllare le esecuzioni parallele

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude non risponde ai comandi @claude
</h3>

* Verificare che la pipeline sia attivata (manualmente, evento MR, o tramite listener di eventi di nota/webhook)
* Assicurarsi che le variabili `ANTHROPIC_API_KEY` o del provider cloud siano presenti
* Controllare che il commento contenga `@claude` (non `/claude`) e che il trigger di menzione sia configurato

<h3 id="job-can’t-write-comments-or-open-mrs">
  Il job non riesce a scrivere commenti o aprire MR
</h3>

* Assicurarsi che `CI_JOB_TOKEN` abbia autorizzazioni sufficienti per il progetto, oppure utilizzare un Project Access Token con scope `api`
* Verificare che lo strumento `mcp__gitlab` sia abilitato in `--allowedTools`
* Confermare che il job sia eseguito nel contesto dell'MR o disponga di contesto sufficiente tramite variabili `AI_FLOW_*`

<h3 id="authentication-errors">
  Errori di autenticazione
</h3>

* **Per Claude API**: Confermare che `ANTHROPIC_API_KEY` sia valida e non scaduta
* **Per Amazon Bedrock o Google Cloud's Agent Platform**: Verificare la configurazione OIDC/WIF, l'impersonazione del ruolo e i nomi dei segreti; confermare la disponibilità della regione e del modello

<h2 id="advanced-configuration">
  Configurazione avanzata
</h2>

<h3 id="common-parameters-and-variables">
  Parametri comuni e variabili
</h3>

Controllate le esecuzioni di Claude Code nei vostri job con questi flag CLI, parole chiave GitLab e variabili:

* `-p`: fornite istruzioni inline, ad esempio `claude -p "Review this MR"`
* `--max-turns`: limitate il numero di iterazioni avanti e indietro
* `timeout`: limitate il tempo totale di esecuzione del job con la parola chiave `timeout` a livello di job di GitLab, ad esempio `timeout: 30m`
* `ANTHROPIC_API_KEY`: richiesta per l'API Claude (non utilizzata per Amazon Bedrock o per la piattaforma Agent di Google Cloud)
* Ambiente specifico del provider: `AWS_REGION`, variabili di progetto/regione per la piattaforma Agent di Google Cloud

<Note>
  I flag e i parametri esatti possono variare a seconda della versione di `@anthropic-ai/claude-code`. Eseguite `claude --help` nel vostro job per visualizzare le opzioni supportate.
</Note>

<h3 id="customizing-claude’s-behavior">
  Personalizzazione del comportamento di Claude
</h3>

Potete guidare Claude in due modi principali:

1. **CLAUDE.md**: Definite gli standard di codifica, i requisiti di sicurezza e le convenzioni del progetto. Claude legge questo durante le esecuzioni e segue le vostre regole.
2. **Prompt personalizzati**: Passate istruzioni specifiche del task tramite `-p` nel job. Utilizzate prompt diversi per job diversi (ad esempio, review, implement, refactor).
