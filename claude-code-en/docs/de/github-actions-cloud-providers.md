> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions mit Cloud-Anbietern verwenden

> Führen Sie Claude Code GitHub Actions über Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry statt über die Claude API aus

[Claude Code GitHub Actions](/docs/de/github-actions) ruft standardmäßig die Claude API auf. Um Inferenzen stattdessen über Ihr eigenes Cloud-Konto zu leiten, legen Sie die Provider-Eingabe der Claude Code GitHub Action fest und konfigurieren Sie Ihre Cloud so, dass sie das OpenID Connect (OIDC)-Token des Workflows vertraut. Der Workflow authentifiziert sich mit diesem Token, sodass Sie keine langlebigen Cloud-Anmeldedaten in Ihrem Repository speichern.

<Info>
  Diese Seite baut auf der [GitHub Actions-Einrichtung](/docs/de/github-actions#setup) auf. Sie setzt voraus, dass Sie bereits die Workflow-Datei und den `anthropics/claude-code-action`-Schritt kennen, und behandelt nur, was ein Cloud-Anbieter ändert.
</Info>

<h2 id="choose-your-provider">
  Wählen Sie Ihren Anbieter
</h2>

Die Claude Code GitHub Action unterstützt drei Anbieter, und die Einrichtungsschritte unterscheiden sich nur in der Cloud-seitigen Konfiguration. Verwenden Sie den Anbieter, bei dem Ihre Organisation bereits Zugriff auf Claude-Modelle hat. Sie teilen der Claude Code GitHub Action mit, welchen Anbieter Sie verwenden möchten, mit einer Eingabe im `with:`-Block des `anthropics/claude-code-action`-Schritts:

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Die vollständigen Workflow-Beispiele unter [Richten Sie die Integration ein](#set-up-the-integration) enthalten bereits die Eingabe für jeden Anbieter.

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Bevor Sie beginnen, benötigen Sie:

* Administratorzugriff auf das Repository, in dem die Claude Code GitHub Action ausgeführt wird, um eine GitHub App zu installieren und Geheimnisse hinzuzufügen
* Berechtigung zum Erstellen von Identitätsressourcen in Ihrem Cloud-Konto: IAM-Rollen und OIDC-Identitätsanbieter auf AWS, Workload Identity Federation-Ressourcen und Dienstkonten auf Google Cloud oder Microsoft Entra-Anwendungen auf Azure
* Claude-Modellzugriff auf Ihrem Anbieter:
  * **Amazon Bedrock**: Zugriff auf Claude-Modelle gewährt. Cross-Region-Inferenzprofile wie die `us.`-Modell-IDs in den Beispielen dieser Seite benötigen Zugriff in jeder Region ihrer Regionsgruppe. Siehe [Claude Code auf Amazon Bedrock](/docs/de/amazon-bedrock)
  * **Google Cloud's Agent Platform**: ein Projekt mit aktivierter Agent Platform API und Zugriff auf Claude-Modelle. Siehe [Claude Code auf Google Cloud's Agent Platform](/docs/de/google-vertex-ai)
  * **Microsoft Foundry**: eine Foundry-Ressource mit einer Claude-Modellbereitstellung. Siehe [Claude Code auf Microsoft Foundry](/docs/de/microsoft-foundry)

<h2 id="set-up-the-integration">
  Richten Sie die Integration ein
</h2>

Über die Voraussetzungen hinaus erstellen Sie eine GitHub-Identität für die Claude Code GitHub Action, die Cloud-seitige Vertrauenskonfiguration, die Repository-Geheimnisse und die Workflow-Datei. Die folgenden Schritte führen Sie durch jeden Punkt.

<Steps>
  <Step title="Wählen Sie eine GitHub-Identität">
    Die Claude Code GitHub Action pusht Commits und postet Kommentare über eine GitHub-Identität. Die [Schnelleinrichtung](/docs/de/github-actions#quick-setup) installiert die offizielle Claude GitHub App dafür. Mit einem Cloud-Anbieter wählen Sie die Identität selbst:

    * **Offizielle [Claude GitHub App](https://github.com/apps/claude)**: installieren Sie sie im Repository, oder überspringen Sie zum nächsten Schritt, wenn sie bereits installiert ist
    * **Benutzerdefinierte GitHub App**: erstellen Sie Ihre eigene App, wenn Sie nur die drei Berechtigungen möchten, die die Claude Code GitHub Action verwendet, anstelle des [vollständigen Satzes der offiziellen App](/docs/de/github-actions#github-app-permissions)
    * **GitHub's automatisches `GITHUB_TOKEN`**: keine App zu erstellen oder zu installieren, aber GitHub löst Ihre CI-Workflows nicht bei Commits aus, die damit erstellt wurden

    Die Workflow-Beispiele im vierten Schritt authentifizieren sich mit einer benutzerdefinierten App. Dieser Schritt sagt auch, was für die anderen beiden Optionen zu ändern ist.

    Um eine benutzerdefinierte App zu erstellen, [registrieren Sie eine neue GitHub App](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) mit deaktivierten Webhooks, da diese Integration sie nicht verwendet. Gewähren Sie ihr drei Repository-Berechtigungen:

    * **Contents**: Lesen und Schreiben
    * **Issues**: Lesen und Schreiben
    * **Pull requests**: Lesen und Schreiben

    Nach der Registrierung der App generieren Sie einen privaten Schlüssel und behalten die heruntergeladene `.pem`-Datei, notieren Sie die App-ID von der Einstellungsseite der App, und [installieren Sie die App](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app) im Repository, in dem die Claude Code GitHub Action ausgeführt wird. Sie fügen den Schlüssel und die ID als Geheimnisse im dritten Schritt hinzu.
  </Step>

  <Step title="Konfigurieren Sie die Cloud-Authentifizierung">
    Konfigurieren Sie Ihre Cloud so, dass sie das OIDC-Token vertraut, das GitHub dem Workflow ausstellt, damit jede Workflow-Ausführung kurzlebige Cloud-Anmeldedaten erhält. Die Aufzählungspunkte in jeder Registerkarte fassen zusammen, was zu erstellen ist, und jede Registerkarte verlinkt auf die eigene Anleitung des Cloud-Anbieters für die Schritte auf Konsolenebene.

    <Tabs>
      <Tab title="Amazon Bedrock">
        Erstellen Sie die Vertrauenskonfiguration in Ihrem AWS-Konto, indem Sie der [AWS-Anleitung zum Erstellen von OIDC-Identitätsanbietern](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html) folgen:

        * Fügen Sie einen GitHub OIDC-Identitätsanbieter mit Provider-URL `https://token.actions.githubusercontent.com` und Audience `sts.amazonaws.com` hinzu
        * Erstellen Sie eine IAM-Rolle, der dieser Anbieter als Web-Identität vertraut, und fügen Sie die scoped Invocation Policy aus [IAM-Konfiguration](/docs/de/amazon-bedrock#iam-configuration) an, die `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:ListInferenceProfiles` und `bedrock:GetInferenceProfile` sowie zwei `aws-marketplace`-Abonnementaktionen gewährt
        * Begrenzen Sie die Vertrauensrichtlinie der Rolle auf Ihr Repository mit einer Bedingung wie `repo:your-org/your-repo:*`. Siehe [GitHub's OIDC-Härtungsanleitung](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect) für das Anspruchsformat

        Notieren Sie sich das ARN der Rolle. Sie fügen es als Geheimnis im nächsten Schritt hinzu.
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Erstellen Sie die Verbundressourcen in Ihrem Google Cloud-Projekt, indem Sie der [Workload Identity Federation-Dokumentation](https://cloud.google.com/iam/docs/workload-identity-federation) folgen:

        * Aktivieren Sie drei APIs: IAM Credentials, Security Token Service (STS) und die Agent Platform API, deren Servicename `aiplatform.googleapis.com` ist
        * Erstellen Sie einen Workload Identity Pool mit einem GitHub OIDC-Anbieter, dessen Aussteller `https://token.actions.githubusercontent.com` ist, und fügen Sie eine Attributbedingung hinzu, die den Pool auf Ihr Repository beschränkt
        * Erstellen Sie ein dediziertes Dienstkonto nur mit der Rolle `Vertex AI User`, die `roles/aiplatform.user` ist, und erlauben Sie dem Pool, es zu imitieren

        Notieren Sie sich den vollständigen Ressourcennamen des Anbieters und die E-Mail-Adresse des Dienstkontos. Sie fügen sie als Geheimnisse im nächsten Schritt hinzu.
      </Tab>

      <Tab title="Microsoft Foundry">
        Erstellen Sie eine Microsoft Entra-Anwendung mit einer Verbundberechtigung für Ihr Repository, indem Sie [Microsofts Anleitung zur Authentifizierung von GitHub Actions](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect) befolgen:

        * Registrieren Sie eine Microsoft Entra-Anwendung und fügen Sie eine Verbundidentitätsberechtigung hinzu, die Tokens vertraut, die GitHub für Ihr Repository ausstellt. Eine benutzerzugewiesene verwaltete Identität funktioniert anstelle einer Anwendung. Beide haben die Client-ID, die Sie unten notieren
        * Weisen Sie der Anwendung die Rolle `Azure AI User` auf Ihrer Foundry-Ressource zu. Siehe [Azure RBAC-Konfiguration](/docs/de/microsoft-foundry#azure-rbac-configuration) für eine engere benutzerdefinierte Rolle

        Notieren Sie sich die Client-ID der Anwendung, Ihre Mandanten-ID und Ihre Abonnement-ID. Sie fügen sie als Geheimnisse im nächsten Schritt hinzu.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Fügen Sie Repository-Geheimnisse hinzu">
    Fügen Sie im Repository, in dem die Claude Code GitHub Action ausgeführt wird, die Geheimnisse für Ihren Anbieter sowie die beiden App-Geheimnisse hinzu, wenn Sie im ersten Schritt eine benutzerdefinierte GitHub App erstellt haben. Siehe GitHub's Anleitung zu [Verwendung von Geheimnissen in GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    | Geheimnis                        | Benötigt für                  | Wert                                               |
    | -------------------------------- | ----------------------------- | -------------------------------------------------- |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock                | Das ARN der IAM-Rolle                              |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud's Agent Platform | Der vollständige Ressourcennamen des Anbieters     |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud's Agent Platform | Die E-Mail-Adresse des Dienstkontos                |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry             | Die Client-ID der Entra-Anwendung                  |
    | `AZURE_TENANT_ID`                | Microsoft Foundry             | Ihre Microsoft Entra-Mandanten-ID                  |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry             | Ihre Azure-Abonnement-ID                           |
    | `APP_ID`                         | Benutzerdefinierte GitHub App | Die ID der GitHub App                              |
    | `APP_PRIVATE_KEY`                | Benutzerdefinierte GitHub App | Der Inhalt der `.pem`-Datei mit privatem Schlüssel |
  </Step>

  <Step title="Erstellen Sie die Workflow-Datei">
    Erstellen Sie eine Workflow-Datei für Ihren Anbieter, z. B. `.github/workflows/claude.yml`. Jedes Beispiel antwortet auf `@claude`-Erwähnungen, authentifiziert sich bei GitHub mit einer benutzerdefinierten App und enthält die Berechtigung `id-token: write`, die GitHub benötigt, um das OIDC-Token auszustellen, das Ihr Cloud-Anbieter gegen Anmeldedaten austauscht.

    Wenn Sie im ersten Schritt eine andere GitHub-Identität gewählt haben, passen Sie das Beispiel an:

    * **Offizielle Claude GitHub App**: löschen Sie den Schritt „Generate GitHub App token" und die Zeile `github_token`
    * **GitHub's automatisches Token**: löschen Sie den Token-Generierungsschritt und ändern Sie die Zeile `github_token` in `github_token: ${{ secrets.GITHUB_TOKEN }}`

    <Warning>
      In öffentlichen Repositories startet ein Kommentar mit dem Trigger-Ausdruck von jedem Benutzer diesen Workflow. Die Credential-Schritte werden ausgeführt, bevor die Claude Code GitHub Action die Schreibzugriffsberechtigung des Kommentators überprüft. Die Action lehnt daher nicht autorisierte Benutzer erst ab, nachdem der Workflow ein App-Token generiert und sich bei Ihrem Cloud-Anbieter angemeldet hat, was Audit-Log-Einträge hinterlässt und Actions-Minuten verbraucht. Um solche Ausführungen zu vermeiden, fügen Sie einen Schritt hinzu, der die Schreibzugriffsberechtigung des Kommentators vor den Credential-Schritten überprüft.
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        Ersetzen Sie den Wert `aws-region` durch Ihren eigenen. Der Credentials-Schritt exportiert ihn als `AWS_REGION` für den Rest des Jobs.

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
          Bedrock-Modell-IDs enthalten ein Cross-Region-Inferenzprofilpräfix wie `us.`. Verwenden Sie das Präfix für die Regionsgruppe, in der Sie Modellzugriff gewährt haben.
        </Tip>
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Ersetzen Sie den Wert `CLOUD_ML_REGION` durch Ihren eigenen. Sie müssen die Projekt-ID nicht hardcodieren, da der Workflow sie aus der Ausgabe des `auth`-Schritts liest.

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
        Ersetzen Sie `your-resource-name` durch Ihren Foundry-Ressourcennamen. Claude Code erstellt die Endpunkt-URL daraus. Der `azure/login`-Schritt meldet sich mit dem OIDC-Token des Workflows an, und Claude Code nimmt die Anmeldedaten über die Azure [Standard-Credential-Chain](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) auf.

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
          Verwenden Sie eine Modell-ID, die einer Claude-Bereitstellung in Ihrer Foundry-Ressource entspricht. Siehe [Claude Code auf Microsoft Foundry](/docs/de/microsoft-foundry) für Modellkonfiguration und Versionsfixierung.
        </Tip>
      </Tab>
    </Tabs>

    Mit jedem Anbieter können Sie die Lauflänge und Kosten begrenzen, indem Sie `--max-turns` zu `claude_args` hinzufügen. Siehe [Verwalten Sie Kosten](/docs/de/github-actions#manage-costs).
  </Step>

  <Step title="Testen Sie die Einrichtung">
    Erwähnen Sie `@claude` in einem Issue- oder PR-Kommentar, und beobachten Sie dann die Ausführung auf der Registerkarte „Actions" des Repositories. Claude antwortet in einem Kommentar zum gleichen Issue oder PR.
  </Step>
</Steps>

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Eine fehlgeschlagene Ausführung bricht normalerweise an einer von zwei Stellen:

* **Authentifizierungsfehler**: normalerweise eine OIDC-Fehlkonfiguration. Überprüfen Sie, dass der Workflow die Berechtigung `id-token: write` enthält, dass die Repository-Bedingung der Vertrauenskonfiguration genau mit Ihrem Repository übereinstimmt, und dass die Geheimnisnamen in Ihrem Workflow mit den hinzugefügten übereinstimmen
* **Trigger- und CI-Probleme**: diese verhalten sich gleich wie bei der Claude Code GitHub Action, die die Claude API aufruft. Siehe den [Fehlerbehebungsabschnitt](/docs/de/github-actions#troubleshooting) der Hauptseite und die [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) der Claude Code GitHub Action

<h2 id="what’s-next">
  Nächste Schritte
</h2>

* [Claude Code GitHub Actions](/docs/de/github-actions) für Beispiele, Parameter und Best Practices
* [Claude Code auf Amazon Bedrock](/docs/de/amazon-bedrock) für Bedrock-Modell-IDs und Regionen
* [Claude Code auf Google Cloud's Agent Platform](/docs/de/google-vertex-ai) für Agent Platform-Modell-IDs und Regionen
* [Claude Code auf Microsoft Foundry](/docs/de/microsoft-foundry) für Foundry-Modell- und Endpunktkonfiguration
