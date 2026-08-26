> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Scopri come integrare Claude Code nel tuo flusso di lavoro di sviluppo con Claude Code GitHub Actions

Claude Code GitHub Actions porta l'automazione basata su AI al tuo flusso di lavoro GitHub. Con una semplice menzione `@claude` in qualsiasi PR o issue, Claude può analizzare il tuo codice, creare pull request, implementare funzionalità e correggere bug - il tutto seguendo gli standard del tuo progetto. Per le revisioni automatiche pubblicate su ogni PR senza un trigger, vedi [GitHub Code Review](/docs/it/code-review).

<Note>
  Claude Code GitHub Actions è costruito sulla base dell'[Claude Agent SDK](/docs/it/agent-sdk/overview), che consente l'integrazione programmatica di Claude Code nelle tue applicazioni. Puoi utilizzare l'SDK per costruire flussi di lavoro di automazione personalizzati oltre GitHub Actions.
</Note>

<h2 id="why-use-claude-code-github-actions">
  Perché utilizzare Claude Code GitHub Actions?
</h2>

* **Creazione istantanea di PR**: Descrivi ciò di cui hai bisogno e Claude crea una PR completa con tutti i cambiamenti necessari
* **Implementazione automatica del codice**: Trasforma gli issue in codice funzionante con un singolo comando
* **Segue i tuoi standard**: Claude rispetta le tue linee guida `CLAUDE.md` e i pattern di codice esistenti
* **Configurazione semplice**: Inizia in pochi minuti con il nostro installer e la chiave API
* **Sicuro per impostazione predefinita**: Il tuo codice rimane sui runner di Github

<h2 id="what-can-claude-do">
  Cosa può fare Claude?
</h2>

Claude Code fornisce un potente GitHub Action che trasforma il modo in cui lavori con il codice:

<h3 id="claude-code-action">
  Claude Code Action
</h3>

Questo GitHub Action ti consente di eseguire Claude Code all'interno dei tuoi flussi di lavoro GitHub Actions. Puoi utilizzarlo per costruire qualsiasi flusso di lavoro personalizzato sulla base di Claude Code.

[Visualizza repository →](https://github.com/anthropics/claude-code-action)

<h2 id="setup">
  Setup
</h2>

<h2 id="quick-setup">
  Configurazione rapida
</h2>

Esegui `/install-github-app` nel terminale di Claude Code per configurare l'integrazione in modo interattivo. Il comando installa l'app GitHub di Claude nel tuo repository e poi ti guida attraverso l'aggiunta dei workflow di GitHub Actions e del secret della chiave API.

Dopo che l'app GitHub è installata, il comando chiede se continuare con la configurazione di GitHub Actions. In Claude Code v2.1.187 e versioni successive puoi scegliere **Salta per ora** per fermarti con solo l'app installata e tornare ai passaggi del workflow e del secret eseguendo di nuovo `/install-github-app`. Le versioni precedenti procedono direttamente alla selezione del workflow.

<Note>
  * Devi essere un amministratore del repository per installare l'app GitHub e aggiungere secret
  * L'app GitHub richiederà autorizzazioni di lettura e scrittura per Contents, Issues e Pull requests
  * Questo metodo di avvio rapido è disponibile solo per gli utenti diretti dell'API Claude. Se stai utilizzando Amazon Bedrock o Google Cloud's Agent Platform, consulta la sezione [Utilizzo con Amazon Bedrock e Google Cloud](#using-with-amazon-bedrock-and-google-cloud).
</Note>

<h2 id="manual-setup">
  Configurazione manuale
</h2>

Se il comando `/install-github-app` non riesce o preferisci una configurazione manuale, segui queste istruzioni di configurazione manuale:

1. **Installa l'app GitHub di Claude** nel tuo repository: [https://github.com/apps/claude](https://github.com/apps/claude)

   L'app GitHub di Claude richiede le seguenti autorizzazioni del repository:

   * **Contents**: Lettura e scrittura (per modificare i file del repository)
   * **Issues**: Lettura e scrittura (per rispondere agli issue)
   * **Pull requests**: Lettura e scrittura (per creare PR e spingere i cambiamenti)

   Per ulteriori dettagli sulla sicurezza e le autorizzazioni, vedi la [documentazione sulla sicurezza](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).
2. **Aggiungi ANTHROPIC\_API\_KEY** ai tuoi secret del repository ([Scopri come utilizzare i secret in GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions))
3. **Copia il file del flusso di lavoro** da [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) nella cartella `.github/workflows/` del tuo repository

<Tip>
  Dopo aver completato la configurazione rapida o manuale, testa l'action taggando `@claude` in un commento di issue o PR.
</Tip>

<h2 id="upgrading-from-beta">
  Aggiornamento dalla versione Beta
</h2>

<Warning>
  Claude Code GitHub Actions v1.0 introduce breaking changes che richiedono l'aggiornamento dei tuoi file di flusso di lavoro per eseguire l'upgrade a v1.0 dalla versione beta.
</Warning>

Se stai attualmente utilizzando la versione beta di Claude Code GitHub Actions, ti consigliamo di aggiornare i tuoi flussi di lavoro per utilizzare la versione GA. La nuova versione semplifica la configurazione aggiungendo potenti nuove funzionalità come il rilevamento automatico della modalità.

<h3 id="essential-changes">
  Cambiamenti essenziali
</h3>

Tutti gli utenti beta devono apportare questi cambiamenti ai loro file di flusso di lavoro per eseguire l'upgrade:

1. **Aggiorna la versione dell'action**: Cambia `@beta` a `@v1`
2. **Rimuovi la configurazione della modalità**: Elimina `mode: "tag"` o `mode: "agent"` (ora rilevata automaticamente)
3. **Aggiorna gli input del prompt**: Sostituisci `direct_prompt` con `prompt`
4. **Sposta le opzioni CLI**: Converti `max_turns`, `model`, `custom_instructions`, ecc. in `claude_args`

<h3 id="breaking-changes-reference">
  Breaking Changes Reference
</h3>

| Old Beta Input        | New v1.0 Input                        |
| --------------------- | ------------------------------------- |
| `mode`                | *(Removed - auto-detected)*           |
| `direct_prompt`       | `prompt`                              |
| `override_prompt`     | `prompt` with GitHub variables        |
| `custom_instructions` | `claude_args: --append-system-prompt` |
| `max_turns`           | `claude_args: --max-turns`            |
| `model`               | `claude_args: --model`                |
| `allowed_tools`       | `claude_args: --allowedTools`         |
| `disallowed_tools`    | `claude_args: --disallowedTools`      |
| `claude_env`          | `settings` JSON format                |

<h3 id="before-and-after-example">
  Esempio Prima e Dopo
</h3>

**Versione beta:**

```yaml theme={null}
- uses: anthropics/claude-code-action@beta
  with:
    mode: "tag"
    direct_prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    custom_instructions: "Follow our coding standards"
    max_turns: "10"
    model: "claude-sonnet-5"
```

**Versione GA (v1.0):**

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    claude_args: |
      --append-system-prompt "Follow our coding standards"
      --max-turns 10
      --model claude-sonnet-5
```

<Tip>
  L'action ora rileva automaticamente se eseguire in modalità interattiva (risponde alle menzioni `@claude`) o in modalità automazione (viene eseguita immediatamente con un prompt) in base alla tua configurazione.
</Tip>

<h2 id="example-use-cases">
  Esempi di casi d'uso
</h2>

Claude Code GitHub Actions può aiutarti con una varietà di attività. La [directory degli esempi](https://github.com/anthropics/claude-code-action/tree/main/examples) contiene flussi di lavoro pronti all'uso per diversi scenari.

<h3 id="basic-workflow">
  Flusso di lavoro di base
</h3>

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          # Responds to @claude mentions in comments
```

<h3 id="using-skills">
  Utilizzo di skills
</h3>

L'input `prompt` accetta un'invocazione di [skill](/docs/it/skills) così come testo semplice:

* Per una skill nella directory `.claude/skills/` del vostro repository, eseguite `actions/checkout` prima del passo dell'azione e passate `/skill-name`.
* Per una skill inclusa in un plugin, installate il plugin con gli input `plugin_marketplaces` e `plugins` e passate lo `/plugin-name:skill-name` con namespace.

Il seguente flusso di lavoro installa il plugin `code-review` ed esegue la sua skill su ogni pull request nuova o aggiornata:

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize]
jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
```

<h3 id="custom-automation-with-prompts">
  Automazione personalizzata con prompt
</h3>

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: "--model opus"
```

<h3 id="common-use-cases">
  Casi d'uso comuni
</h3>

Nei commenti di issue o PR:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude analizzerà automaticamente il contesto e risponderà in modo appropriato.

<h2 id="best-practices">
  Best practices
</h2>

<h3 id="claude-md-configuration">
  Configurazione CLAUDE.md
</h3>

Crea un file `CLAUDE.md` nella radice del tuo repository per definire le linee guida dello stile di codice, i criteri di revisione, le regole specifiche del progetto e i pattern preferiti. Questo file guida la comprensione di Claude degli standard del tuo progetto.

<h3 id="security-considerations">
  Considerazioni sulla sicurezza
</h3>

<Warning>Non eseguire mai il commit delle chiavi API direttamente nel tuo repository.</Warning>

Per una guida completa sulla sicurezza inclusa autorizzazioni, autenticazione e best practices, vedi la [documentazione sulla sicurezza di Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

Utilizza sempre GitHub Secrets per le chiavi API:

* Aggiungi la tua chiave API come secret del repository denominato `ANTHROPIC_API_KEY`
* Fai riferimento ad essa nei flussi di lavoro: `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`
* Limita le autorizzazioni dell'action solo a ciò che è necessario
* Rivedi i suggerimenti di Claude prima di eseguire il merge

Utilizza sempre GitHub Secrets (ad esempio, `${{ secrets.ANTHROPIC_API_KEY }}`) piuttosto che hardcodare le chiavi API direttamente nei tuoi file di flusso di lavoro.

<h3 id="optimizing-performance">
  Ottimizzazione delle prestazioni
</h3>

Utilizza i template di issue per fornire contesto, mantieni il tuo `CLAUDE.md` conciso e focalizzato, e configura timeout appropriati per i tuoi flussi di lavoro.

<h3 id="ci-costs">
  Costi CI
</h3>

Quando utilizzi Claude Code GitHub Actions, tieni presente i costi associati:

**Costi di GitHub Actions:**

* Claude Code viene eseguito su runner ospitati da GitHub, che consumano i tuoi minuti di GitHub Actions
* Vedi la [documentazione sulla fatturazione di GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) per i dettagli sui prezzi e i limiti di minuti

**Costi API:**

* Ogni interazione di Claude consuma token API in base alla lunghezza dei prompt e delle risposte
* L'utilizzo dei token varia in base alla complessità dell'attività e alla dimensione della codebase
* Vedi la [pagina dei prezzi di Claude](https://claude.com/platform/api) per i tassi di token attuali

**Suggerimenti per l'ottimizzazione dei costi:**

* Utilizza comandi specifici `@claude` per ridurre le chiamate API non necessarie
* Configura `--max-turns` appropriato in `claude_args` per prevenire iterazioni eccessive
* Imposta timeout a livello di flusso di lavoro per evitare job fuori controllo
* Considera l'utilizzo dei controlli di concorrenza di GitHub per limitare le esecuzioni parallele

<h2 id="configuration-examples">
  Esempi di configurazione
</h2>

Claude Code Action v1 semplifica la configurazione con parametri unificati:

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    prompt: "Your instructions here" # Optional
    claude_args: "--max-turns 5" # Optional CLI arguments
```

Caratteristiche principali:

* **Interfaccia prompt unificata** - Utilizza `prompt` per tutte le istruzioni
* **Skills** - Richiama [skills](/docs/it/skills) installate direttamente dal prompt
* **Passthrough CLI** - Qualsiasi argomento CLI di Claude Code tramite `claude_args`
* **Trigger flessibili** - Funziona con qualsiasi evento GitHub

Visita la [directory degli esempi](https://github.com/anthropics/claude-code-action/tree/main/examples) per i file di flusso di lavoro completi.

<Tip>
  Quando rispondi ai commenti di issue o PR, Claude risponde automaticamente alle menzioni @claude. Per altri eventi, utilizza il parametro `prompt` per fornire istruzioni.
</Tip>

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Utilizzo con Amazon Bedrock e Google Cloud
</h2>

Per ambienti aziendali, puoi utilizzare Claude Code GitHub Actions con la tua infrastruttura cloud. Questo approccio ti dà il controllo sulla residenza dei dati e sulla fatturazione mantenendo la stessa funzionalità.

<h3 id="prerequisites">
  Prerequisiti
</h3>

Prima di configurare Claude Code GitHub Actions con i provider cloud, hai bisogno di:

<h4 id="for-google-cloud’s-agent-platform">
  Per Google Cloud's Agent Platform:
</h4>

1. Un progetto Google Cloud con Google Cloud's Agent Platform abilitato
2. Workload Identity Federation configurato per GitHub Actions
3. Un account di servizio con le autorizzazioni richieste
4. Un'app GitHub (consigliato) o utilizza il GITHUB\_TOKEN predefinito

<h4 id="for-amazon-bedrock">
  Per Amazon Bedrock:
</h4>

1. Un account AWS con Amazon Bedrock abilitato
2. GitHub OIDC Identity Provider configurato in AWS
3. Un ruolo IAM con autorizzazioni Amazon Bedrock
4. Un'app GitHub (consigliato) o utilizza il GITHUB\_TOKEN predefinito

<Steps>
  <Step title="Crea un'app GitHub personalizzata (Consigliato per provider di terze parti)">
    Per il miglior controllo e sicurezza quando utilizzi provider di terze parti come Google Cloud's Agent Platform o Amazon Bedrock, ti consigliamo di creare la tua app GitHub:

    1. Vai a [https://github.com/settings/apps/new](https://github.com/settings/apps/new)
    2. Compila le informazioni di base:
       * **GitHub App name**: Scegli un nome univoco (ad es. "YourOrg Claude Assistant")
       * **Homepage URL**: Il sito web della tua organizzazione o l'URL del repository
    3. Configura le impostazioni dell'app:
       * **Webhooks**: Deseleziona "Active" (non necessario per questa integrazione)
    4. Imposta le autorizzazioni richieste:
       * **Repository permissions**:
         * Contents: Read & Write
         * Issues: Read & Write
         * Pull requests: Read & Write
    5. Fai clic su "Create GitHub App"
    6. Dopo la creazione, fai clic su "Generate a private key" e salva il file `.pem` scaricato
    7. Annota il tuo App ID dalla pagina delle impostazioni dell'app
    8. Installa l'app nel tuo repository:
       * Dalla pagina delle impostazioni della tua app, fai clic su "Install App" nella barra laterale sinistra
       * Seleziona il tuo account o organizzazione
       * Scegli "Only select repositories" e seleziona il repository specifico
       * Fai clic su "Install"
    9. Aggiungi la chiave privata come secret al tuo repository:
       * Vai a Settings → Secrets and variables → Actions del tuo repository
       * Crea un nuovo secret denominato `APP_PRIVATE_KEY` con il contenuto del file `.pem`
    10. Aggiungi l'App ID come secret:

    * Crea un nuovo secret denominato `APP_ID` con l'ID della tua app GitHub

    <Note>
      Questa app verrà utilizzata con l'action [actions/create-github-app-token](https://github.com/actions/create-github-app-token) per generare token di autenticazione nei tuoi flussi di lavoro.
    </Note>

    **Alternativa per Claude API o se non vuoi configurare la tua app Github**: Utilizza l'app ufficiale di Anthropic:

    1. Installa da: [https://github.com/apps/claude](https://github.com/apps/claude)
    2. Nessuna configurazione aggiuntiva necessaria per l'autenticazione
  </Step>

  <Step title="Configura l'autenticazione del provider cloud">
    Scegli il tuo provider cloud e configura l'autenticazione sicura:

    <AccordionGroup>
      <Accordion title="Amazon Bedrock">
        **Configura AWS per consentire a GitHub Actions di autenticarsi in modo sicuro senza archiviare le credenziali.**

        > **Security Note**: Utilizza configurazioni specifiche del repository e concedi solo le autorizzazioni minime richieste.

        **Required Setup**:

        1. **Enable Amazon Bedrock**:
           * Richiedi l'accesso ai modelli Claude in Amazon Bedrock
           * Per i modelli tra regioni, richiedi l'accesso in tutte le regioni richieste

        2. **Set up GitHub OIDC Identity Provider**:
           * Provider URL: `https://token.actions.githubusercontent.com`
           * Audience: `sts.amazonaws.com`

        3. **Create IAM Role for GitHub Actions**:
           * Trusted entity type: Web identity
           * Identity provider: `token.actions.githubusercontent.com`
           * Permissions: `AmazonBedrockFullAccess` policy
           * Configure trust policy for your specific repository

        **Required Values**:

        Dopo la configurazione, avrai bisogno di:

        * **AWS\_ROLE\_TO\_ASSUME**: L'ARN del ruolo IAM che hai creato

        <Tip>
          OIDC è più sicuro rispetto all'utilizzo di chiavi di accesso AWS statiche perché le credenziali sono temporanee e ruotate automaticamente.
        </Tip>

        Vedi la [documentazione AWS](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html) per le istruzioni dettagliate sulla configurazione di OIDC.
      </Accordion>

      <Accordion title="Google Cloud's Agent Platform">
        **Configura Google Cloud per consentire a GitHub Actions di autenticarsi in modo sicuro senza archiviare le credenziali.**

        > **Security Note**: Utilizza configurazioni specifiche del repository e concedi solo le autorizzazioni minime richieste.

        **Required Setup**:

        1. **Enable APIs** nel tuo progetto Google Cloud:
           * IAM Credentials API
           * Security Token Service (STS) API
           * Google Cloud's Agent Platform API

        2. **Create Workload Identity Federation resources**:
           * Crea un Workload Identity Pool
           * Aggiungi un provider OIDC GitHub con:
             * Issuer: `https://token.actions.githubusercontent.com`
             * Attribute mappings for repository and owner
             * **Security recommendation**: Utilizza condizioni di attributo specifiche del repository

        3. **Create a Service Account**:
           * Concedi solo il ruolo `Vertex AI User`
           * **Security recommendation**: Crea un account di servizio dedicato per repository

        4. **Configure IAM bindings**:
           * Consenti al Workload Identity Pool di rappresentare l'account di servizio
           * **Security recommendation**: Utilizza set di principali specifici del repository

        **Required Values**:

        Dopo la configurazione, avrai bisogno di:

        * **GCP\_WORKLOAD\_IDENTITY\_PROVIDER**: Il nome completo della risorsa provider
        * **GCP\_SERVICE\_ACCOUNT**: L'indirizzo email dell'account di servizio

        <Tip>
          Workload Identity Federation elimina la necessità di chiavi di account di servizio scaricabili, migliorando la sicurezza.
        </Tip>

        Per le istruzioni di configurazione dettagliate, consulta la [documentazione di Google Cloud Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation).
      </Accordion>
    </AccordionGroup>
  </Step>

  <Step title="Aggiungi Secret Richiesti">
    Aggiungi i seguenti secret al tuo repository (Settings → Secrets and variables → Actions):

    #### Per Claude API (Direct):

    1. **Per l'autenticazione API**:
       * `ANTHROPIC_API_KEY`: La tua chiave API Claude da [console.anthropic.com](https://console.anthropic.com)

    2. **Per GitHub App (se utilizzi la tua app)**:
       * `APP_ID`: L'ID della tua app GitHub
       * `APP_PRIVATE_KEY`: Il contenuto della chiave privata (.pem)

    #### Per Google Cloud's Agent Platform

    1. **Per l'autenticazione GCP**:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER`
       * `GCP_SERVICE_ACCOUNT`

    2. **Per GitHub App (se utilizzi la tua app)**:
       * `APP_ID`: L'ID della tua app GitHub
       * `APP_PRIVATE_KEY`: Il contenuto della chiave privata (.pem)

    #### Per Amazon Bedrock

    1. **Per l'autenticazione AWS**:
       * `AWS_ROLE_TO_ASSUME`

    2. **Per GitHub App (se utilizzi la tua app)**:
       * `APP_ID`: L'ID della tua app GitHub
       * `APP_PRIVATE_KEY`: Il contenuto della chiave privata (.pem)
  </Step>

  <Step title="Crea file di flusso di lavoro">
    Crea file di flusso di lavoro GitHub Actions che si integrano con il tuo provider cloud. Gli esempi seguenti mostrano configurazioni complete sia per Amazon Bedrock che per Google Cloud's Agent Platform:

    <AccordionGroup>
      <Accordion title="Amazon Bedrock workflow">
        **Prerequisites:**

        * Amazon Bedrock access enabled with Claude model permissions
        * GitHub configured as an OIDC identity provider in AWS
        * IAM role with Amazon Bedrock permissions that trusts GitHub Actions

        **Required GitHub secrets:**

        | Secret Name          | Description                                       |
        | -------------------- | ------------------------------------------------- |
        | `AWS_ROLE_TO_ASSUME` | ARN of the IAM role for Amazon Bedrock access     |
        | `APP_ID`             | Your GitHub App ID (from app settings)            |
        | `APP_PRIVATE_KEY`    | The private key you generated for your GitHub App |

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened, assigned]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
            runs-on: ubuntu-latest
            env:
              AWS_REGION: us-west-2
            steps:
              - name: Checkout repository
                uses: actions/checkout@v4

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Configure AWS Credentials (OIDC)
                uses: aws-actions/configure-aws-credentials@v4
                with:
                  role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
                  aws-region: us-west-2

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_bedrock: "true"
                  claude_args: '--model us.anthropic.claude-sonnet-4-6 --max-turns 10'
        ```

        <Tip>
          Il formato dell'ID del modello per Amazon Bedrock include un prefisso di regione (ad esempio, `us.anthropic.claude-sonnet-4-6`).
        </Tip>
      </Accordion>

      <Accordion title="Google Cloud's Agent Platform workflow">
        **Prerequisites:**

        * Google Cloud's Agent Platform API enabled in your GCP project
        * Workload Identity Federation configured for GitHub
        * Service account with Google Cloud's Agent Platform permissions

        **Required GitHub secrets:**

        | Secret Name                      | Description                                                     |
        | -------------------------------- | --------------------------------------------------------------- |
        | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Workload identity provider resource name                        |
        | `GCP_SERVICE_ACCOUNT`            | Service account email with Google Cloud's Agent Platform access |
        | `APP_ID`                         | Your GitHub App ID (from app settings)                          |
        | `APP_PRIVATE_KEY`                | The private key you generated for your GitHub App               |

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened, assigned]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v4

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Google Cloud
                id: auth
                uses: google-github-actions/auth@v2
                with:
                  workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
                  service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  trigger_phrase: "@claude"
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-4-5@20250929 --max-turns 10'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
                  VERTEX_REGION_CLAUDE_4_5_SONNET: us-east5
        ```

        <Tip>
          L'ID del progetto viene recuperato automaticamente dal passaggio di autenticazione di Google Cloud, quindi non è necessario codificarlo.
        </Tip>
      </Accordion>
    </AccordionGroup>
  </Step>
</Steps>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude non risponde ai comandi @claude
</h3>

Verifica che l'app GitHub sia installata correttamente, controlla che i flussi di lavoro siano abilitati, assicurati che la chiave API sia impostata nei secret del repository e conferma che il commento contenga `@claude` (non `/claude`).

<h3 id="ci-not-running-on-claude’s-commits">
  CI non in esecuzione sui commit di Claude
</h3>

Assicurati di utilizzare l'app GitHub o l'app personalizzata (non l'utente Actions), controlla che i trigger del flusso di lavoro includano gli eventi necessari e verifica che le autorizzazioni dell'app includano i trigger CI.

<h3 id="authentication-errors">
  Errori di autenticazione
</h3>

Conferma che la chiave API sia valida e abbia autorizzazioni sufficienti. Per Amazon Bedrock o Google Cloud's Agent Platform, controlla la configurazione delle credenziali e assicurati che i secret siano denominati correttamente nei flussi di lavoro.

<h2 id="advanced-configuration">
  Configurazione avanzata
</h2>

<h3 id="action-parameters">
  Parametri dell'action
</h3>

Claude Code Action v1 utilizza una configurazione semplificata:

| Parameter             | Description                                                                        | Required |
| --------------------- | ---------------------------------------------------------------------------------- | -------- |
| `prompt`              | Istruzioni per Claude (testo semplice o un nome di [skill](/docs/it/skills))            | No\*     |
| `claude_args`         | Argomenti CLI passati a Claude Code                                                | No       |
| `plugin_marketplaces` | Elenco separato da newline degli URL Git del marketplace dei plugin                | No       |
| `plugins`             | Elenco separato da newline dei nomi dei plugin da installare prima dell'esecuzione | No       |
| `anthropic_api_key`   | Chiave API Claude                                                                  | Yes\*\*  |
| `github_token`        | Token GitHub per l'accesso API                                                     | No       |
| `trigger_phrase`      | Frase trigger personalizzata (predefinito: "@claude")                              | No       |
| `use_bedrock`         | Utilizza Amazon Bedrock invece dell'API Claude                                     | No       |
| `use_vertex`          | Utilizza Google Cloud's Agent Platform invece dell'API Claude                      | No       |

\*Prompt è opzionale - quando omesso per i commenti di issue/PR, Claude risponde alla frase trigger\
\*\*Richiesto per l'API Claude diretta, non per Amazon Bedrock o Google Cloud's Agent Platform

<h4 id="pass-cli-arguments">
  Passa argomenti CLI
</h4>

Il parametro `claude_args` accetta qualsiasi argomento CLI di Claude Code:

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Argomenti comuni:

* `--max-turns`: Numero massimo di turni di conversazione (predefinito: 10)
* `--model`: Modello da utilizzare (ad es. `claude-sonnet-5`)
* `--mcp-config`: Percorso della configurazione MCP
* `--allowedTools`: Elenco separato da virgole degli strumenti consentiti. L'alias `--allowed-tools` funziona anche.
* `--debug`: Abilita l'output di debug

<h3 id="alternative-integration-methods">
  Metodi di integrazione alternativi
</h3>

Mentre il comando `/install-github-app` è l'approccio consigliato, potete anche:

* **Custom GitHub App**: Per le organizzazioni che necessitano di nomi utente personalizzati o flussi di autenticazione personalizzati. Create la vostra app GitHub con le autorizzazioni richieste (contents, issues, pull requests) e utilizzate l'action actions/create-github-app-token per generare token nei vostri flussi di lavoro.
* **Manual GitHub Actions**: Configurazione diretta del flusso di lavoro per la massima flessibilità
* **MCP Configuration**: Caricamento dinamico dei server Model Context Protocol

Consultate la [documentazione di Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs) per guide dettagliate su autenticazione, sicurezza e configurazione avanzata.

<h3 id="customizing-claude’s-behavior">
  Personalizzazione del comportamento di Claude
</h3>

Potete configurare il comportamento di Claude in due modi:

1. **CLAUDE.md**: Definite gli standard di codifica, i criteri di revisione e le regole specifiche del progetto in un file `CLAUDE.md` nella radice del vostro repository. Claude seguirà queste linee guida quando crea PR e risponde alle richieste. Consultate la nostra [documentazione Memory](/docs/it/memory) per ulteriori dettagli.
2. **Custom prompts**: Utilizzate il parametro `prompt` nel file del flusso di lavoro per fornire istruzioni specifiche del flusso di lavoro. Questo vi consente di personalizzare il comportamento di Claude per diversi flussi di lavoro o attività.

Claude seguirà queste linee guida quando crea PR e risponde alle richieste.
