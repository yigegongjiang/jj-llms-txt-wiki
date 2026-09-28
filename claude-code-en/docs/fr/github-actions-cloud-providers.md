> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utiliser Claude Code GitHub Actions avec les fournisseurs cloud

> Exécutez Claude Code GitHub Actions via Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry au lieu de l'API Claude

[Claude Code GitHub Actions](/docs/fr/github-actions) appelle l'API Claude par défaut. Pour acheminer l'inférence via votre propre compte cloud à la place, définissez l'entrée du fournisseur de l'action GitHub Claude Code et configurez votre cloud pour faire confiance au jeton OpenID Connect (OIDC) du workflow. Le workflow s'authentifie avec ce jeton, vous n'avez donc pas besoin de stocker d'identifiants cloud de longue durée dans votre référentiel.

<Info>
  Cette page s'appuie sur la [configuration de GitHub Actions](/docs/fr/github-actions#setup). Elle suppose que vous connaissez déjà le fichier de workflow et l'étape `anthropics/claude-code-action`, et couvre uniquement ce qu'un fournisseur cloud change.
</Info>

<h2 id="choose-your-provider">
  Choisissez votre fournisseur
</h2>

Claude Code GitHub Action prend en charge trois fournisseurs, et les étapes de configuration ci-dessous ne diffèrent que dans la configuration côté cloud. Utilisez celui où votre organisation a déjà accès aux modèles Claude. Vous indiquez à Claude Code GitHub Action quel fournisseur utiliser avec une entrée dans le bloc `with:` de l'étape `anthropics/claude-code-action` :

* **Amazon Bedrock** : `use_bedrock: "true"`
* **Google Cloud's Agent Platform** : `use_vertex: "true"`
* **Microsoft Foundry** : `use_foundry: "true"`

Les exemples de workflow complets sous [Configurer l'intégration](#set-up-the-integration) incluent déjà l'entrée pour chaque fournisseur.

<h2 id="prerequisites">
  Conditions préalables
</h2>

Avant de commencer, vous avez besoin de :

* Un accès administrateur au référentiel où Claude Code GitHub Action s'exécute, pour installer une application GitHub et ajouter des secrets
* La permission de créer des ressources d'identité dans votre compte cloud : rôles IAM et fournisseurs d'identité OIDC sur AWS, ressources Workload Identity Federation et comptes de service sur Google Cloud, ou applications Microsoft Entra sur Azure
* Un accès aux modèles Claude sur votre fournisseur :
  * **Amazon Bedrock** : accès accordé aux modèles Claude. Les profils d'inférence inter-régions, tels que les ID de modèle `us.` dans les exemples de cette page, nécessitent un accès accordé dans chaque région de leur groupe de régions. Voir [Claude Code sur Amazon Bedrock](/docs/fr/amazon-bedrock)
  * **Google Cloud's Agent Platform** : un projet avec l'API Agent Platform activée et l'accès aux modèles Claude. Voir [Claude Code sur Google Cloud's Agent Platform](/docs/fr/google-vertex-ai)
  * **Microsoft Foundry** : une ressource Foundry avec un déploiement de modèle Claude. Voir [Claude Code sur Microsoft Foundry](/docs/fr/microsoft-foundry)

<h2 id="set-up-the-integration">
  Configurer l'intégration
</h2>

Au-delà des conditions préalables, vous créez une identité GitHub pour Claude Code GitHub Action, la configuration de confiance côté cloud, les secrets du référentiel et le fichier de workflow. Les étapes ci-dessous vous guident à travers chacune d'elles.

<Steps>
  <Step title="Choisir une identité GitHub">
    Claude Code GitHub Action pousse les commits et publie les commentaires via une identité GitHub. La [configuration rapide](/docs/fr/github-actions#quick-setup) installe l'application Claude GitHub officielle pour cela. Avec un fournisseur cloud, vous choisissez l'identité vous-même :

    * **[Application Claude GitHub](https://github.com/apps/claude) officielle** : installez-la sur le référentiel, ou passez à l'étape suivante si elle est déjà installée
    * **Application GitHub personnalisée** : créez votre propre application lorsque vous souhaitez uniquement les trois permissions que Claude Code GitHub Action utilise plutôt que l'[ensemble complet de l'application officielle](/docs/fr/github-actions#github-app-permissions)
    * **`GITHUB_TOKEN` automatique de GitHub** : aucune application à créer ou installer, mais GitHub ne déclenche pas vos workflows CI sur les commits effectués avec celui-ci

    Les exemples de workflow à la quatrième étape s'authentifient avec une application personnalisée. Cette étape indique également ce qu'il faut modifier pour les deux autres options.

    Pour créer une application personnalisée, [enregistrez une nouvelle application GitHub](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) avec les webhooks désactivés, car cette intégration ne les utilise pas. Accordez-lui trois permissions de référentiel :

    * **Contents** : lecture et écriture
    * **Issues** : lecture et écriture
    * **Pull requests** : lecture et écriture

    Après avoir enregistré l'application, générez une clé privée et conservez le fichier `.pem` téléchargé, notez l'ID de l'application à partir de la page des paramètres de l'application, et [installez l'application](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app) sur le référentiel où Claude Code GitHub Action s'exécute. Vous ajoutez la clé et l'ID en tant que secrets à la troisième étape.
  </Step>

  <Step title="Configurer l'authentification cloud">
    Configurez votre cloud pour faire confiance au jeton OIDC que GitHub émet au workflow, afin que chaque exécution de workflow obtienne des identifiants cloud de courte durée. Les puces dans chaque onglet résument ce qu'il faut créer, et chaque onglet renvoie au guide du fournisseur cloud pour les étapes au niveau de la console.

    <Tabs>
      <Tab title="Amazon Bedrock">
        Créez la configuration de confiance dans votre compte AWS, en suivant le [guide AWS pour créer des fournisseurs d'identité OIDC](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html) :

        * Ajoutez un fournisseur d'identité OIDC GitHub avec l'URL du fournisseur `https://token.actions.githubusercontent.com` et l'audience `sts.amazonaws.com`
        * Créez un rôle IAM approuvé par ce fournisseur en tant qu'identité web, et attachez la politique d'invocation délimitée de [Configuration IAM](/docs/fr/amazon-bedrock#iam-configuration), qui accorde `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:ListInferenceProfiles` et `bedrock:GetInferenceProfile`, ainsi que deux actions d'abonnement `aws-marketplace`
        * Limitez la politique de confiance du rôle à votre référentiel avec une condition de sujet telle que `repo:your-org/your-repo:*`. Voir le [guide de renforcement OIDC de GitHub](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect) pour le format de la réclamation

        Notez l'ARN du rôle. Vous l'ajoutez en tant que secret à l'étape suivante.
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Créez les ressources de fédération dans votre projet Google Cloud, en suivant la [documentation Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation) :

        * Activez trois API : IAM Credentials, Security Token Service (STS) et l'API Agent Platform, dont le nom du service est `aiplatform.googleapis.com`
        * Créez un pool Workload Identity avec un fournisseur OIDC GitHub dont l'émetteur est `https://token.actions.githubusercontent.com`, et ajoutez une condition d'attribut qui limite le pool à votre référentiel
        * Créez un compte de service dédié avec uniquement le rôle `Vertex AI User`, qui est `roles/aiplatform.user`, et autorisez le pool à l'emprunter

        Notez le nom de ressource complet du fournisseur et l'adresse e-mail du compte de service. Vous les ajoutez en tant que secrets à l'étape suivante.
      </Tab>

      <Tab title="Microsoft Foundry">
        Créez une application Microsoft Entra avec une identité fédérée pour votre référentiel, en suivant le [guide Microsoft pour l'authentification à partir de GitHub Actions](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect) :

        * Enregistrez une application Microsoft Entra et ajoutez une identité fédérée qui fait confiance aux jetons que GitHub émet à votre référentiel. Une identité gérée affectée par l'utilisateur fonctionne à la place d'une application. Les deux ont l'ID client que vous notez ci-dessous
        * Attribuez à l'application le rôle `Azure AI User` sur votre ressource Foundry. Voir [Configuration Azure RBAC](/docs/fr/microsoft-foundry#azure-rbac-configuration) pour un rôle personnalisé plus étroit

        Notez l'ID client de l'application, votre ID de locataire et votre ID d'abonnement. Vous les ajoutez en tant que secrets à l'étape suivante.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Ajouter les secrets du référentiel">
    Dans le référentiel où Claude Code GitHub Action s'exécute, ajoutez les secrets pour votre fournisseur, plus les deux secrets d'application si vous avez créé une application GitHub personnalisée à la première étape. Voir le guide GitHub sur l'[utilisation des secrets dans GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    | Secret                           | Nécessaire pour                  | Valeur                                     |
    | -------------------------------- | -------------------------------- | ------------------------------------------ |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock                   | L'ARN du rôle IAM                          |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud's Agent Platform    | Le nom de ressource complet du fournisseur |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud's Agent Platform    | L'adresse e-mail du compte de service      |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry                | L'ID client de l'application Entra         |
    | `AZURE_TENANT_ID`                | Microsoft Foundry                | Votre ID de locataire Microsoft Entra      |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry                | Votre ID d'abonnement Azure                |
    | `APP_ID`                         | Application GitHub personnalisée | L'ID de l'application GitHub               |
    | `APP_PRIVATE_KEY`                | Application GitHub personnalisée | Le contenu du fichier de clé privée `.pem` |
  </Step>

  <Step title="Créer le fichier de workflow">
    Créez un fichier de workflow pour votre fournisseur, tel que `.github/workflows/claude.yml`. Chaque exemple répond aux mentions `@claude`, s'authentifie auprès de GitHub avec une application personnalisée et inclut la permission `id-token: write`, que GitHub exige pour émettre le jeton OIDC que votre fournisseur cloud échange contre des identifiants.

    Si vous avez choisi une identité GitHub différente à la première étape, ajustez l'exemple :

    * **Application Claude GitHub officielle** : supprimez l'étape Generate GitHub App token et la ligne `github_token`
    * **Jeton automatique de GitHub** : supprimez l'étape de génération de jeton et modifiez la ligne `github_token` en `github_token: ${{ secrets.GITHUB_TOKEN }}`

    <Warning>
      Sur les référentiels publics, un commentaire contenant la phrase déclencheur de n'importe quel utilisateur démarre ce workflow. Les étapes d'identifiants s'exécutent avant que Claude Code GitHub Action vérifie l'accès en écriture du commentateur, de sorte que l'action rejette les utilisateurs non autorisés uniquement après que le workflow a généré un jeton d'application et s'est connecté à votre fournisseur cloud, ce qui laisse des entrées de journal d'audit et consomme des minutes d'Actions. Pour éviter ces exécutions, ajoutez une étape qui vérifie l'accès en écriture du commentateur avant les étapes d'identifiants.
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        Remplacez la valeur `aws-region` par la vôtre. L'étape d'identifiants l'exporte en tant que `AWS_REGION` pour le reste du travail.

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
          Les ID de modèle Bedrock incluent un préfixe de profil d'inférence inter-régions tel que `us.`. Utilisez le préfixe pour le groupe de régions où vous avez accordé l'accès au modèle.
        </Tip>
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Remplacez la valeur `CLOUD_ML_REGION` par la vôtre. Vous n'avez pas besoin de coder en dur l'ID du projet, car le workflow le lit à partir de la sortie de l'étape `auth`.

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
        Remplacez `your-resource-name` par le nom de votre ressource Foundry. Claude Code construit l'URL du point de terminaison à partir de celui-ci. L'étape `azure/login` se connecte avec le jeton OIDC du workflow, et Claude Code récupère les identifiants via la [chaîne d'identifiants par défaut](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) Azure.

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
          Utilisez un ID de modèle qui correspond à un déploiement Claude dans votre ressource Foundry. Voir [Claude Code sur Microsoft Foundry](/docs/fr/microsoft-foundry) pour la configuration du modèle et l'épinglage de version.
        </Tip>
      </Tab>
    </Tabs>

    Avec n'importe quel fournisseur, vous pouvez limiter la durée d'exécution et les coûts en ajoutant `--max-turns` à `claude_args`. Voir [Gérer les coûts](/docs/fr/github-actions#manage-costs).
  </Step>

  <Step title="Tester la configuration">
    Mentionnez `@claude` dans un commentaire de problème ou de demande de tirage, puis regardez l'exécution dans l'onglet Actions du référentiel. Claude répond dans un commentaire sur le même problème ou la même demande de tirage.
  </Step>
</Steps>

<h2 id="troubleshooting">
  Dépannage
</h2>

Une exécution défaillante se casse généralement à l'un de ces deux endroits :

* **Erreurs d'authentification** : généralement une mauvaise configuration OIDC. Vérifiez que le workflow inclut la permission `id-token: write`, que la condition de référentiel de la configuration de confiance correspond exactement à votre référentiel, et que les noms de secrets dans votre workflow correspondent à ceux que vous avez ajoutés
* **Problèmes de déclenchement et CI** : ils se comportent de la même manière que lorsque Claude Code GitHub Action appelle l'API Claude. Voir la [section dépannage](/docs/fr/github-actions#troubleshooting) de la page principale et la [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) de Claude Code GitHub Action

<h2 id="what’s-next">
  Prochaines étapes
</h2>

* [Claude Code GitHub Actions](/docs/fr/github-actions) pour les exemples, les paramètres et les meilleures pratiques
* [Claude Code sur Amazon Bedrock](/docs/fr/amazon-bedrock) pour les ID de modèle Bedrock et les régions
* [Claude Code sur Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) pour les ID de modèle Agent Platform et les régions
* [Claude Code sur Microsoft Foundry](/docs/fr/microsoft-foundry) pour la configuration du modèle Foundry et du point de terminaison
