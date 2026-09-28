> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usa Claude Code GitHub Actions con i provider cloud

> Esegui Claude Code GitHub Actions tramite Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry invece dell'API Claude

[Claude Code GitHub Actions](/docs/it/github-actions) chiama l'API Claude per impostazione predefinita. Per instradare l'inferenza attraverso il tuo account cloud, imposta l'input del provider dell'azione GitHub di Claude Code e configura il tuo cloud per fidarsi del token OpenID Connect (OIDC) del flusso di lavoro. Il flusso di lavoro si autentica con quel token, quindi non memorizzi alcuna credenziale cloud di lunga durata nel tuo repository.

<Info>
  Questa pagina si basa sulla [configurazione di GitHub Actions](/docs/it/github-actions#setup). Presuppone che tu conosca già il file del flusso di lavoro e il passaggio `anthropics/claude-code-action`, e copre solo ciò che cambia un provider cloud.
</Info>

<h2 id="choose-your-provider">
  Scegli il tuo provider
</h2>

Claude Code GitHub Action supporta tre provider e i passaggi di configurazione di seguito differiscono solo nella configurazione lato cloud. Usa quello dove la tua organizzazione ha già accesso ai modelli Claude. Comunichi all'azione GitHub di Claude Code quale provider utilizzare con un input nel blocco `with:` del passaggio `anthropics/claude-code-action`:

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Gli esempi di flusso di lavoro completi nella sezione [Configura l'integrazione](#set-up-the-integration) includono già l'input per ogni provider.

<h2 id="prerequisites">
  Prerequisiti
</h2>

Prima di iniziare, hai bisogno di:

* Accesso amministratore al repository dove viene eseguita l'azione GitHub di Claude Code, per installare un'app GitHub e aggiungere segreti
* Autorizzazione per creare risorse di identità nel tuo account cloud: ruoli IAM e provider di identità OIDC su AWS, risorse Workload Identity Federation e account di servizio su Google Cloud, o applicazioni Microsoft Entra su Azure
* Accesso ai modelli Claude sul tuo provider:
  * **Amazon Bedrock**: accesso concesso ai modelli Claude. I profili di inferenza tra regioni, come gli ID modello `us.` negli esempi di questa pagina, necessitano dell'accesso concesso in ogni regione del loro gruppo di regioni. Vedi [Claude Code su Amazon Bedrock](/docs/it/amazon-bedrock)
  * **Google Cloud's Agent Platform**: un progetto con l'API Agent Platform abilitata e accesso ai modelli Claude. Vedi [Claude Code su Google Cloud's Agent Platform](/docs/it/google-vertex-ai)
  * **Microsoft Foundry**: una risorsa Foundry con una distribuzione di modello Claude. Vedi [Claude Code su Microsoft Foundry](/docs/it/microsoft-foundry)

<h2 id="set-up-the-integration">
  Configurare l'integrazione
</h2>

Oltre ai prerequisiti, è necessario creare un'identità GitHub per l'azione GitHub di Claude Code, la configurazione della fiducia lato cloud, i segreti del repository e il file del workflow. I passaggi seguenti illustrano ciascuno di essi.

<Steps>
  <Step title="Scegliere un'identità GitHub">
    L'azione GitHub di Claude Code esegue il push dei commit e pubblica commenti attraverso un'identità GitHub. La [configurazione rapida](/docs/it/github-actions#quick-setup) installa l'app GitHub ufficiale di Claude per questo scopo. Con un provider cloud, scegli l'identità tu stesso:

    * **App GitHub ufficiale di [Claude](https://github.com/apps/claude)**: installala nel repository, oppure salta al passaggio successivo se è già installata
    * **App GitHub personalizzata**: crea la tua app quando desideri solo i tre permessi che l'azione GitHub di Claude Code utilizza piuttosto che l'[insieme completo dell'app ufficiale](/docs/it/github-actions#github-app-permissions)
    * **Token `GITHUB_TOKEN` automatico di GitHub**: nessuna app da creare o installare, ma GitHub non attiva i tuoi workflow CI sui commit effettuati con esso

    Gli esempi di workflow nel quarto passaggio si autenticano con un'app personalizzata. Quel passaggio spiega anche cosa modificare per le altre due opzioni.

    Per creare un'app personalizzata, [registra una nuova app GitHub](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) con i webhook disabilitati, poiché questa integrazione non li utilizza. Concedi tre permessi del repository:

    * **Contents**: lettura e scrittura
    * **Issues**: lettura e scrittura
    * **Pull requests**: lettura e scrittura

    Dopo aver registrato l'app, genera una chiave privata e conserva il file `.pem` scaricato, annota l'ID app dalla pagina delle impostazioni dell'app, e [installa l'app](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app) nel repository dove viene eseguita l'azione GitHub di Claude Code. Aggiungi la chiave e l'ID come segreti nel terzo passaggio.
  </Step>

  <Step title="Configurare l'autenticazione cloud">
    Configura il tuo cloud per fidarsi del token OIDC che GitHub emette al workflow, in modo che ogni esecuzione del workflow ottenga credenziali cloud di breve durata. I punti elenco in ogni scheda riassumono cosa creare, e ogni scheda collega la guida del fornitore cloud per i passaggi a livello di console.

    <Tabs>
      <Tab title="Amazon Bedrock">
        Crea la configurazione della fiducia nel tuo account AWS, seguendo la [guida AWS per la creazione di provider di identità OIDC](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html):

        * Aggiungi un provider di identità OIDC GitHub con URL del provider `https://token.actions.githubusercontent.com` e audience `sts.amazonaws.com`
        * Crea un ruolo IAM di cui il provider si fida come identità web, e allega la politica di invocazione con ambito dalla [configurazione IAM](/docs/it/amazon-bedrock#iam-configuration), che concede `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:ListInferenceProfiles`, e `bedrock:GetInferenceProfile`, insieme a due azioni di sottoscrizione `aws-marketplace`
        * Limita la politica di fiducia del ruolo al tuo repository con una condizione di soggetto come `repo:your-org/your-repo:*`. Vedi la [guida di hardening OIDC di GitHub](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect) per il formato del claim

        Annota l'ARN del ruolo. Lo aggiungerai come segreto nel passaggio successivo.
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Crea le risorse di federazione nel tuo progetto Google Cloud, seguendo la [documentazione di Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation):

        * Abilita tre API: IAM Credentials, Security Token Service (STS), e l'API Agent Platform, il cui nome del servizio è `aiplatform.googleapis.com`
        * Crea un Workload Identity Pool con un provider OIDC GitHub il cui emittente è `https://token.actions.githubusercontent.com`, e aggiungi una condizione di attributo che limita il pool al tuo repository
        * Crea un account di servizio dedicato con solo il ruolo `Vertex AI User`, che è `roles/aiplatform.user`, e consenti al pool di rappresentarlo

        Annota il nome della risorsa completa del provider e l'indirizzo email dell'account di servizio. Li aggiungerai come segreti nel passaggio successivo.
      </Tab>

      <Tab title="Microsoft Foundry">
        Crea un'applicazione Microsoft Entra con una credenziale federata per il tuo repository, seguendo [la guida di Microsoft per l'autenticazione da GitHub Actions](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect):

        * Registra un'applicazione Microsoft Entra e aggiungi una credenziale di identità federata che si fida dei token che GitHub emette al tuo repository. Un'identità gestita assegnata dall'utente funziona al posto di un'applicazione. Entrambe hanno l'ID client che annoti di seguito
        * Assegna all'applicazione il ruolo `Azure AI User` sulla tua risorsa Foundry. Vedi [configurazione RBAC di Azure](/docs/it/microsoft-foundry#azure-rbac-configuration) per un ruolo personalizzato più ristretto

        Annota l'ID client dell'applicazione, il tuo ID tenant e il tuo ID sottoscrizione. Li aggiungerai come segreti nel passaggio successivo.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Aggiungere i segreti del repository">
    Nel repository dove viene eseguita l'azione GitHub di Claude Code, aggiungi i segreti per il tuo provider, più i due segreti dell'app se hai creato un'app GitHub personalizzata nel primo passaggio. Vedi la guida di GitHub su [come usare i segreti in GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    | Segreto                          | Necessario per                | Valore                                            |
    | -------------------------------- | ----------------------------- | ------------------------------------------------- |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock                | L'ARN del ruolo IAM                               |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud's Agent Platform | Il nome della risorsa completa del provider       |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud's Agent Platform | L'indirizzo email dell'account di servizio        |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry             | L'ID client dell'applicazione Entra               |
    | `AZURE_TENANT_ID`                | Microsoft Foundry             | Il tuo ID tenant Microsoft Entra                  |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry             | Il tuo ID sottoscrizione Azure                    |
    | `APP_ID`                         | App GitHub personalizzata     | L'ID dell'app GitHub                              |
    | `APP_PRIVATE_KEY`                | App GitHub personalizzata     | Il contenuto del file della chiave privata `.pem` |
  </Step>

  <Step title="Creare il file del workflow">
    Crea un file di workflow per il tuo provider, come `.github/workflows/claude.yml`. Ogni esempio risponde alle menzioni `@claude`, si autentica su GitHub con un'app personalizzata, e include il permesso `id-token: write`, che GitHub richiede per emettere il token OIDC che il tuo provider cloud scambia per le credenziali.

    Se hai scelto un'identità GitHub diversa nel primo passaggio, regola l'esempio:

    * **App GitHub ufficiale di Claude**: elimina il passaggio Generate GitHub App token e la riga `github_token`
    * **Token automatico di GitHub**: elimina il passaggio di generazione del token e cambia la riga `github_token` in `github_token: ${{ secrets.GITHUB_TOKEN }}`

    <Warning>
      Nei repository pubblici, un commento contenente la frase di attivazione da qualsiasi utente avvia questo workflow. I passaggi delle credenziali vengono eseguiti prima che l'azione GitHub di Claude Code verifichi l'accesso in scrittura del commentatore, quindi l'azione rifiuta gli utenti non autorizzati solo dopo che il workflow ha generato un token dell'app e ha effettuato l'accesso al tuo provider cloud, il che lascia voci nel registro di audit e consuma minuti di Actions. Per evitare queste esecuzioni, aggiungi un passaggio che verifica l'accesso in scrittura del commentatore prima dei passaggi delle credenziali.
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        Sostituisci il valore `aws-region` con il tuo. Il passaggio delle credenziali lo esporta come `AWS_REGION` per il resto del job.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

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
                  claude_args: '--model us.anthropic.claude-sonnet-4-6'
        ```

        <Tip>
          Gli ID dei modelli Bedrock includono un prefisso del profilo di inferenza tra regioni come `us.`. Usa il prefisso per il gruppo di regioni dove hai concesso l'accesso al modello.
        </Tip>
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Sostituisci il valore `CLOUD_ML_REGION` con il tuo. Non è necessario codificare l'ID del progetto, perché il workflow lo legge dall'output del passaggio `auth`.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

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
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
        ```
      </Tab>

      <Tab title="Microsoft Foundry">
        Sostituisci `your-resource-name` con il nome della tua risorsa Foundry. Claude Code costruisce l'URL dell'endpoint da esso. Il passaggio `azure/login` effettua l'accesso con il token OIDC del workflow, e Claude Code raccoglie le credenziali attraverso la [catena di credenziali predefinita](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) di Azure.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Azure
                uses: azure/login@v2
                with:
                  client-id: ${{ secrets.AZURE_CLIENT_ID }}
                  tenant-id: ${{ secrets.AZURE_TENANT_ID }}
                  subscription-id: ${{ secrets.AZURE_SUBSCRIPTION_ID }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_foundry: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_FOUNDRY_RESOURCE: your-resource-name
        ```

        <Tip>
          Usa un ID modello che corrisponda a un deployment di Claude nella tua risorsa Foundry. Vedi [Claude Code su Microsoft Foundry](/docs/it/microsoft-foundry) per la configurazione del modello e il pinning della versione.
        </Tip>
      </Tab>
    </Tabs>

    Con qualsiasi provider, puoi limitare la durata dell'esecuzione e il costo aggiungendo `--max-turns` a `claude_args`. Vedi [Gestire i costi](/docs/it/github-actions#manage-costs).
  </Step>

  <Step title="Testare la configurazione">
    Menziona `@claude` in un commento su un issue o PR, quindi guarda l'esecuzione nella scheda Actions del repository. Claude risponde in un commento sullo stesso issue o PR.
  </Step>
</Steps>

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Un'esecuzione non riuscita di solito si interrompe in uno di due punti:

* **Errori di autenticazione**: di solito una configurazione OIDC errata. Verifica che il flusso di lavoro includa il permesso `id-token: write`, che la condizione del repository della configurazione di trust corrisponda esattamente al tuo repository, e che i nomi dei segreti nel tuo flusso di lavoro corrispondano a quelli che hai aggiunto
* **Problemi di trigger e CI**: si comportano allo stesso modo di quando l'azione GitHub di Claude Code chiama l'API Claude. Vedi la [sezione di risoluzione dei problemi](/docs/it/github-actions#troubleshooting) della pagina principale e le [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) dell'azione GitHub di Claude Code

<h2 id="what’s-next">
  Cosa fare dopo
</h2>

* [Claude Code GitHub Actions](/docs/it/github-actions) per esempi, parametri e best practice
* [Claude Code su Amazon Bedrock](/docs/it/amazon-bedrock) per gli ID modello Bedrock e le regioni
* [Claude Code su Google Cloud's Agent Platform](/docs/it/google-vertex-ai) per gli ID modello di Agent Platform e le regioni
* [Claude Code su Microsoft Foundry](/docs/it/microsoft-foundry) per la configurazione del modello e dell'endpoint di Foundry
