> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code GitHub Actions com provedores de nuvem

> Execute Claude Code GitHub Actions através do Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry em vez da Claude API

[Claude Code GitHub Actions](/docs/pt/github-actions) chama a Claude API por padrão. Para rotear a inferência através de sua própria conta de nuvem, defina a entrada do provedor da Claude Code GitHub Action e configure sua nuvem para confiar no token OpenID Connect (OIDC) do fluxo de trabalho. O fluxo de trabalho se autentica com esse token, portanto você não armazena nenhuma credencial de nuvem de longa duração em seu repositório.

<Info>
  Esta página se baseia na [configuração do GitHub Actions](/docs/pt/github-actions#setup). Ela assume que você já conhece o arquivo de fluxo de trabalho e a etapa `anthropics/claude-code-action`, e cobre apenas o que um provedor de nuvem muda.
</Info>

<h2 id="choose-your-provider">
  Escolha seu provedor
</h2>

A Claude Code GitHub Action suporta três provedores, e as etapas de configuração abaixo diferem apenas na configuração do lado da nuvem. Use aquele onde sua organização já tem acesso ao modelo Claude. Você diz à Claude Code GitHub Action qual provedor usar com uma entrada no bloco `with:` da etapa `anthropics/claude-code-action`:

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Os exemplos de fluxo de trabalho completos em [Configurar a integração](#set-up-the-integration) já incluem a entrada para cada provedor.

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de começar, você precisa de:

* Acesso de administrador ao repositório onde a Claude Code GitHub Action é executada, para instalar um GitHub App e adicionar segredos
* Permissão para criar recursos de identidade em sua conta de nuvem: funções IAM e provedores de identidade OIDC no AWS, recursos de Workload Identity Federation e contas de serviço no Google Cloud, ou aplicativos Microsoft Entra no Azure
* Acesso ao modelo Claude em seu provedor:
  * **Amazon Bedrock**: acesso concedido aos modelos Claude. Perfis de inferência entre regiões, como os IDs de modelo `us.` nos exemplos desta página, precisam de acesso concedido em cada região de seu grupo de regiões. Veja [Claude Code no Amazon Bedrock](/docs/pt/amazon-bedrock)
  * **Google Cloud's Agent Platform**: um projeto com a API Agent Platform ativada e acesso aos modelos Claude. Veja [Claude Code no Google Cloud's Agent Platform](/docs/pt/google-vertex-ai)
  * **Microsoft Foundry**: um recurso Foundry com uma implantação de modelo Claude. Veja [Claude Code no Microsoft Foundry](/docs/pt/microsoft-foundry)

<h2 id="set-up-the-integration">
  Configurar a integração
</h2>

Além dos pré-requisitos, você cria uma identidade GitHub para a Claude Code GitHub Action, a configuração de confiança do lado da nuvem, os segredos do repositório e o arquivo de fluxo de trabalho. As etapas abaixo orientam você em cada uma.

<Steps>
  <Step title="Escolha uma identidade GitHub">
    A Claude Code GitHub Action envia commits e publica comentários através de uma identidade GitHub. A [configuração rápida](/docs/pt/github-actions#quick-setup) instala o Claude GitHub App oficial para isso. Com um provedor de nuvem, você escolhe a identidade você mesmo:

    * **[Claude GitHub App](https://github.com/apps/claude) oficial**: instale-a no repositório, ou pule para a próxima etapa se já estiver instalada
    * **GitHub App personalizado**: crie seu próprio app quando você quiser apenas as três permissões que a Claude Code GitHub Action usa em vez do [conjunto completo do app oficial](/docs/pt/github-actions#github-app-permissions)
    * **Token automático `GITHUB_TOKEN` do GitHub**: nenhum app para criar ou instalar, mas o GitHub não dispara seus fluxos de trabalho de CI em commits feitos com ele

    Os exemplos de fluxo de trabalho na quarta etapa se autenticam com um app personalizado. Essa etapa também diz o que mudar para as outras duas opções.

    Para criar um app personalizado, [registre um novo GitHub App](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) com webhooks desativados, já que essa integração não os usa. Conceda a ele três permissões de repositório:

    * **Contents**: leitura e escrita
    * **Issues**: leitura e escrita
    * **Pull requests**: leitura e escrita

    Após registrar o app, gere uma chave privada e mantenha o arquivo `.pem` baixado, anote o ID do App na página de configurações do app, e [instale o app](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app) no repositório onde a Claude Code GitHub Action é executada. Você adiciona a chave e o ID como segredos na terceira etapa.
  </Step>

  <Step title="Configurar autenticação na nuvem">
    Configure sua nuvem para confiar no token OIDC que o GitHub emite para o fluxo de trabalho, para que cada execução de fluxo de trabalho obtenha credenciais de nuvem de curta duração. Os pontos em cada aba resumem o que criar, e cada aba vincula o guia do próprio fornecedor de nuvem para as etapas no nível do console.

    <Tabs>
      <Tab title="Amazon Bedrock">
        Crie a configuração de confiança em sua conta AWS, seguindo o [guia AWS para criar provedores de identidade OIDC](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html):

        * Adicione um provedor de identidade OIDC do GitHub com URL do provedor `https://token.actions.githubusercontent.com` e público `sts.amazonaws.com`
        * Crie uma função IAM confiável por esse provedor como uma identidade web, e anexe a política de invocação com escopo de [Configuração IAM](/docs/pt/amazon-bedrock#iam-configuration), que concede `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:ListInferenceProfiles` e `bedrock:GetInferenceProfile`, junto com duas ações de assinatura `aws-marketplace`
        * Limite a política de confiança da função ao seu repositório com uma condição de assunto como `repo:your-org/your-repo:*`. Veja o [guia de endurecimento OIDC do GitHub](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect) para o formato de reclamação

        Anote o ARN da função. Você o adiciona como um segredo na próxima etapa.
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Crie os recursos de federação em seu projeto Google Cloud, seguindo a [documentação de Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation):

        * Ative três APIs: IAM Credentials, Security Token Service (STS) e a API Agent Platform, cujo nome de serviço é `aiplatform.googleapis.com`
        * Crie um Workload Identity Pool com um provedor OIDC do GitHub cujo emissor é `https://token.actions.githubusercontent.com`, e adicione uma condição de atributo que limite o pool ao seu repositório
        * Crie uma conta de serviço dedicada com apenas a função `Vertex AI User`, que é `roles/aiplatform.user`, e permita que o pool a represente

        Anote o nome completo do recurso do provedor e o endereço de email da conta de serviço. Você os adiciona como segredos na próxima etapa.
      </Tab>

      <Tab title="Microsoft Foundry">
        Crie um aplicativo Microsoft Entra com uma credencial federada para seu repositório, seguindo o [guia da Microsoft para autenticação do GitHub Actions](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect):

        * Registre um aplicativo Microsoft Entra e adicione uma credencial de identidade federada que confie em tokens que o GitHub emite para seu repositório. Uma identidade gerenciada atribuída pelo usuário funciona no lugar de um aplicativo. Ambos têm o ID do cliente que você anota abaixo
        * Atribua ao aplicativo a função `Azure AI User` em seu recurso Foundry. Veja [Configuração RBAC do Azure](/docs/pt/microsoft-foundry#azure-rbac-configuration) para uma função personalizada mais estreita

        Anote o ID do cliente do aplicativo, seu ID de locatário e seu ID de assinatura. Você os adiciona como segredos na próxima etapa.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Adicionar segredos do repositório">
    No repositório onde a Claude Code GitHub Action é executada, adicione os segredos para seu provedor, mais os dois segredos do app se você criou um GitHub App personalizado na primeira etapa. Veja o guia do GitHub para [usar segredos no GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    | Segredo                          | Necessário para               | Valor                                         |
    | -------------------------------- | ----------------------------- | --------------------------------------------- |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock                | O ARN da função IAM                           |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud's Agent Platform | O nome completo do recurso do provedor        |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud's Agent Platform | O endereço de email da conta de serviço       |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry             | O ID do cliente do aplicativo Entra           |
    | `AZURE_TENANT_ID`                | Microsoft Foundry             | Seu ID de locatário Microsoft Entra           |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry             | Seu ID de assinatura do Azure                 |
    | `APP_ID`                         | GitHub App personalizado      | O ID do GitHub App                            |
    | `APP_PRIVATE_KEY`                | GitHub App personalizado      | O conteúdo do arquivo de chave privada `.pem` |
  </Step>

  <Step title="Criar o arquivo de fluxo de trabalho">
    Crie um arquivo de fluxo de trabalho para seu provedor, como `.github/workflows/claude.yml`. Cada exemplo responde a menções `@claude`, se autentica no GitHub com um app personalizado e inclui a permissão `id-token: write`, que o GitHub exige para emitir o token OIDC que seu provedor de nuvem troca por credenciais.

    Se você escolheu uma identidade GitHub diferente na primeira etapa, ajuste o exemplo:

    * **Claude GitHub App oficial**: delete a etapa Generate GitHub App token e a linha `github_token`
    * **Token automático do GitHub**: delete a etapa de geração de token e mude a linha `github_token` para `github_token: ${{ secrets.GITHUB_TOKEN }}`

    <Warning>
      Em repositórios públicos, um comentário contendo a frase de gatilho de qualquer usuário inicia este fluxo de trabalho. As etapas de credencial são executadas antes da Claude Code GitHub Action verificar o acesso de escrita do comentarista, portanto a ação rejeita usuários não autorizados apenas após o fluxo de trabalho ter gerado um token de App e se conectado ao seu provedor de nuvem, o que deixa entradas de log de auditoria e consome minutos de Actions. Para evitar essas execuções, adicione uma etapa que verifique o acesso de escrita do comentarista antes das etapas de credencial.
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        Substitua o valor `aws-region` pelo seu próprio. A etapa de credenciais o exporta como `AWS_REGION` para o resto do trabalho.

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
          Os IDs de modelo Bedrock incluem um prefixo de perfil de inferência entre regiões como `us.`. Use o prefixo para o grupo de regiões onde você concedeu acesso ao modelo.
        </Tip>
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Substitua o valor `CLOUD_ML_REGION` pelo seu próprio. Você não precisa codificar o ID do projeto, porque o fluxo de trabalho o lê da saída da etapa `auth`.

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
        Substitua `your-resource-name` pelo nome do seu recurso Foundry. Claude Code constrói a URL do endpoint a partir dele. A etapa `azure/login` se conecta com o token OIDC do fluxo de trabalho, e Claude Code pega as credenciais através da [cadeia de credencial padrão](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) do Azure.

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
          Use um ID de modelo que corresponda a uma implantação Claude em seu recurso Foundry. Veja [Claude Code no Microsoft Foundry](/docs/pt/microsoft-foundry) para configuração de modelo e fixação de versão.
        </Tip>
      </Tab>
    </Tabs>

    Com qualquer provedor, você pode limitar o tempo de execução e o custo adicionando `--max-turns` a `claude_args`. Veja [Gerenciar custos](/docs/pt/github-actions#manage-costs).
  </Step>

  <Step title="Testar a configuração">
    Mencione `@claude` em um comentário de issue ou PR, depois observe a execução na aba Actions do repositório. Claude responde em um comentário na mesma issue ou PR.
  </Step>
</Steps>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Uma execução com falha geralmente quebra em um de dois lugares:

* **Erros de autenticação**: geralmente uma configuração incorreta de OIDC. Verifique se o fluxo de trabalho inclui a permissão `id-token: write`, se a condição do repositório da configuração de confiança corresponde exatamente ao seu repositório, e se os nomes dos segredos em seu fluxo de trabalho correspondem aos que você adicionou
* **Problemas de gatilho e CI**: esses se comportam da mesma forma que quando a Claude Code GitHub Action chama a API Claude. Veja a [seção de troubleshooting](/docs/pt/github-actions#troubleshooting) da página principal e o [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) da Claude Code GitHub Action

<h2 id="what’s-next">
  Próximos passos
</h2>

* [Claude Code GitHub Actions](/docs/pt/github-actions) para exemplos, parâmetros e melhores práticas
* [Claude Code no Amazon Bedrock](/docs/pt/amazon-bedrock) para IDs de modelo Bedrock e regiões
* [Claude Code no Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) para IDs de modelo Agent Platform e regiões
* [Claude Code no Microsoft Foundry](/docs/pt/microsoft-foundry) para configuração de modelo e endpoint Foundry
