> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Saiba como integrar Claude Code no seu fluxo de trabalho de desenvolvimento com Claude Code GitHub Actions

Claude Code GitHub Actions traz automação alimentada por IA para seu fluxo de trabalho do GitHub. Com uma simples menção `@claude` em qualquer PR ou issue, Claude pode analisar seu código, criar pull requests, implementar recursos e corrigir bugs - tudo enquanto segue os padrões do seu projeto. Para análises automáticas postadas em cada PR sem um gatilho, consulte [GitHub Code Review](/docs/pt/code-review).

<Note>
  Claude Code GitHub Actions é construído sobre o [Claude Agent SDK](/docs/pt/agent-sdk/overview), que permite integração programática do Claude Code em suas aplicações. Você pode usar o SDK para construir fluxos de trabalho de automação personalizados além do GitHub Actions.
</Note>

<h2 id="why-use-claude-code-github-actions">
  Por que usar Claude Code GitHub Actions?
</h2>

* **Criação instantânea de PR**: Descreva o que você precisa, e Claude cria um PR completo com todas as alterações necessárias
* **Implementação de código automatizada**: Transforme issues em código funcional com um único comando
* **Segue seus padrões**: Claude respeita suas diretrizes `CLAUDE.md` e padrões de código existentes
* **Configuração simples**: Comece em minutos com nosso instalador e chave de API
* **Seguro por padrão**: Seu código permanece nos runners do Github

<h2 id="what-can-claude-do">
  O que Claude pode fazer?
</h2>

Claude Code fornece uma poderosa GitHub Action que transforma como você trabalha com código:

<h3 id="claude-code-action">
  Claude Code Action
</h3>

Esta GitHub Action permite que você execute Claude Code dentro de seus fluxos de trabalho do GitHub Actions. Você pode usar isso para construir qualquer fluxo de trabalho personalizado sobre Claude Code.

[Ver repositório →](https://github.com/anthropics/claude-code-action)

<h2 id="setup">
  Configuração
</h2>

<h2 id="quick-setup">
  Configuração rápida
</h2>

Execute `/install-github-app` no terminal do Claude Code para configurar a integração de forma interativa. O comando instala o Claude GitHub App no seu repositório e depois o orienta através da adição dos workflows do GitHub Actions e do secret da chave de API.

Após o GitHub App ser instalado, o comando pergunta se deseja continuar com a configuração do GitHub Actions. No Claude Code v2.1.187 e posterior, você pode escolher **Pular por enquanto** para parar apenas com o App instalado e retornar aos passos de workflow e secret executando `/install-github-app` novamente. Versões anteriores prosseguem direto para a seleção de workflow.

<Note>
  * Você deve ser um administrador do repositório para instalar o aplicativo GitHub e adicionar secrets
  * O aplicativo GitHub solicitará permissões de leitura e escrita para Contents, Issues e Pull requests
  * Este método de início rápido está disponível apenas para usuários diretos da Claude API. Se você está usando Amazon Bedrock ou Google Cloud's Agent Platform, consulte a seção [Usando com Amazon Bedrock e Google Cloud](#using-with-amazon-bedrock-and-google-cloud).
</Note>

<h2 id="manual-setup">
  Configuração manual
</h2>

Se o comando `/install-github-app` falhar ou você preferir configuração manual, siga estas instruções de configuração manual:

1. **Instale o aplicativo Claude GitHub** em seu repositório: [https://github.com/apps/claude](https://github.com/apps/claude)

   O aplicativo Claude GitHub requer as seguintes permissões de repositório:

   * **Contents**: Leitura e escrita (para modificar arquivos do repositório)
   * **Issues**: Leitura e escrita (para responder a issues)
   * **Pull requests**: Leitura e escrita (para criar PRs e fazer push de alterações)

   Para mais detalhes sobre segurança e permissões, consulte a [documentação de segurança](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).
2. **Adicione ANTHROPIC\_API\_KEY** aos seus secrets do repositório ([Saiba como usar secrets no GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions))
3. **Copie o arquivo de fluxo de trabalho** de [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) para a pasta `.github/workflows/` do seu repositório

<Tip>
  Após concluir a configuração rápida ou manual, teste a action marcando `@claude` em um comentário de issue ou PR.
</Tip>

<h2 id="upgrading-from-beta">
  Atualizando da versão Beta
</h2>

<Warning>
  Claude Code GitHub Actions v1.0 introduz mudanças significativas que exigem atualizar seus arquivos de fluxo de trabalho para fazer upgrade da versão beta para v1.0.
</Warning>

Se você está usando a versão beta do Claude Code GitHub Actions, recomendamos que você atualize seus fluxos de trabalho para usar a versão GA. A nova versão simplifica a configuração enquanto adiciona recursos poderosos como detecção automática de modo.

<h3 id="essential-changes">
  Mudanças essenciais
</h3>

Todos os usuários beta devem fazer essas alterações em seus arquivos de fluxo de trabalho para fazer upgrade:

1. **Atualize a versão da action**: Mude `@beta` para `@v1`
2. **Remova a configuração de modo**: Delete `mode: "tag"` ou `mode: "agent"` (agora detectado automaticamente)
3. **Atualize as entradas de prompt**: Substitua `direct_prompt` por `prompt`
4. **Mova as opções de CLI**: Converta `max_turns`, `model`, `custom_instructions`, etc. para `claude_args`

<h3 id="breaking-changes-reference">
  Referência de Mudanças Significativas
</h3>

| Entrada Beta Antiga   | Nova Entrada v1.0                        |
| --------------------- | ---------------------------------------- |
| `mode`                | *(Removido - detectado automaticamente)* |
| `direct_prompt`       | `prompt`                                 |
| `override_prompt`     | `prompt` com variáveis do GitHub         |
| `custom_instructions` | `claude_args: --append-system-prompt`    |
| `max_turns`           | `claude_args: --max-turns`               |
| `model`               | `claude_args: --model`                   |
| `allowed_tools`       | `claude_args: --allowedTools`            |
| `disallowed_tools`    | `claude_args: --disallowedTools`         |
| `claude_env`          | `settings` formato JSON                  |

<h3 id="before-and-after-example">
  Exemplo Antes e Depois
</h3>

**Versão beta:**

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

**Versão GA (v1.0):**

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
  A action agora detecta automaticamente se deve executar em modo interativo (responde a menções `@claude`) ou modo de automação (executa imediatamente com um prompt) com base em sua configuração.
</Tip>

<h2 id="example-use-cases">
  Exemplos de casos de uso
</h2>

Claude Code GitHub Actions pode ajudá-lo com uma variedade de tarefas. O [diretório de exemplos](https://github.com/anthropics/claude-code-action/tree/main/examples) contém fluxos de trabalho prontos para uso em diferentes cenários.

<h3 id="basic-workflow">
  Fluxo de trabalho básico
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
  Usando skills
</h3>

A entrada `prompt` aceita uma invocação de [skill](/docs/pt/skills) bem como texto simples:

* Para uma skill no diretório `.claude/skills/` do seu repositório, execute `actions/checkout` antes da etapa de ação e passe `/skill-name`.
* Para uma skill empacotada em um plugin, instale o plugin com as entradas `plugin_marketplaces` e `plugins` e passe a `/plugin-name:skill-name` com namespace.

O fluxo de trabalho a seguir instala o plugin `code-review` e executa sua skill em cada pull request novo ou atualizado:

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
  Automação personalizada com prompts
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
  Casos de uso comuns
</h3>

Em comentários de issue ou PR:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude analisará automaticamente o contexto e responderá apropriadamente.

<h2 id="best-practices">
  Melhores práticas
</h2>

<h3 id="claude-md-configuration">
  Configuração CLAUDE.md
</h3>

Crie um arquivo `CLAUDE.md` na raiz do seu repositório para definir diretrizes de estilo de código, critérios de revisão, regras específicas do projeto e padrões preferidos. Este arquivo guia a compreensão de Claude dos padrões do seu projeto.

<h3 id="security-considerations">
  Considerações de segurança
</h3>

<Warning>Nunca faça commit de chaves de API diretamente em seu repositório.</Warning>

Para orientação abrangente de segurança incluindo permissões, autenticação e melhores práticas, consulte a [documentação de segurança do Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

Sempre use GitHub Secrets para chaves de API:

* Adicione sua chave de API como um secret do repositório nomeado `ANTHROPIC_API_KEY`
* Referencie-a em fluxos de trabalho: `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`
* Limite as permissões da action apenas ao necessário
* Revise as sugestões de Claude antes de fazer merge

Sempre use GitHub Secrets (por exemplo, `${{ secrets.ANTHROPIC_API_KEY }}`) em vez de codificar chaves de API diretamente em seus arquivos de fluxo de trabalho.

<h3 id="optimizing-performance">
  Otimizando desempenho
</h3>

Use templates de issue para fornecer contexto, mantenha seu `CLAUDE.md` conciso e focado, e configure timeouts apropriados para seus fluxos de trabalho.

<h3 id="ci-costs">
  Custos de CI
</h3>

Ao usar Claude Code GitHub Actions, esteja ciente dos custos associados:

**Custos do GitHub Actions:**

* Claude Code é executado em runners hospedados pelo GitHub, que consomem seus minutos do GitHub Actions
* Consulte a [documentação de faturamento do GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) para detalhes de preços e limites de minutos

**Custos de API:**

* Cada interação com Claude consome tokens de API com base no comprimento de prompts e respostas
* O uso de tokens varia pela complexidade da tarefa e tamanho da base de código
* Consulte a [página de preços do Claude](https://claude.com/platform/api) para as taxas de token atuais

**Dicas de otimização de custos:**

* Use comandos específicos `@claude` para reduzir chamadas de API desnecessárias
* Configure `--max-turns` apropriado em `claude_args` para evitar iterações excessivas
* Defina timeouts no nível do fluxo de trabalho para evitar jobs descontrolados
* Considere usar controles de concorrência do GitHub para limitar execuções paralelas

<h2 id="configuration-examples">
  Exemplos de configuração
</h2>

A Claude Code Action v1 simplifica a configuração com parâmetros unificados:

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    prompt: "Your instructions here" # Optional
    claude_args: "--max-turns 5" # Optional CLI arguments
```

Recursos principais:

* **Interface de prompt unificada** - Use `prompt` para todas as instruções
* **Skills** - Invoque [skills](/docs/pt/skills) instaladas diretamente do prompt
* **Passagem de CLI** - Qualquer argumento de CLI do Claude Code via `claude_args`
* **Gatilhos flexíveis** - Funciona com qualquer evento do GitHub

Visite o [diretório de exemplos](https://github.com/anthropics/claude-code-action/tree/main/examples) para arquivos de fluxo de trabalho completos.

<Tip>
  Ao responder a comentários de issue ou PR, Claude responde automaticamente a menções @claude. Para outros eventos, use o parâmetro `prompt` para fornecer instruções.
</Tip>

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Usando com Amazon Bedrock e Google Cloud
</h2>

Para ambientes empresariais, você pode usar Claude Code GitHub Actions com sua própria infraestrutura em nuvem. Esta abordagem oferece controle sobre residência de dados e faturamento enquanto mantém a mesma funcionalidade.

<h3 id="prerequisites">
  Pré-requisitos
</h3>

Antes de configurar Claude Code GitHub Actions com provedores de nuvem, você precisa:

<h4 id="for-google-cloud’s-agent-platform">
  Para Google Cloud's Agent Platform:
</h4>

1. Um Projeto Google Cloud com Google Cloud's Agent Platform habilitado
2. Workload Identity Federation configurado para GitHub Actions
3. Uma conta de serviço com as permissões necessárias
4. Uma GitHub App (recomendado) ou use o GITHUB\_TOKEN padrão

<h4 id="for-amazon-bedrock">
  Para Amazon Bedrock:
</h4>

1. Uma conta AWS com Amazon Bedrock habilitado
2. GitHub OIDC Identity Provider configurado na AWS
3. Uma função IAM com permissões do Amazon Bedrock
4. Uma GitHub App (recomendado) ou use o GITHUB\_TOKEN padrão

<Steps>
  <Step title="Crie uma GitHub App personalizada (Recomendado para Provedores 3P)">
    Para melhor controle e segurança ao usar provedores 3P como Google Cloud's Agent Platform ou Amazon Bedrock, recomendamos criar sua própria GitHub App:

    1. Vá para [https://github.com/settings/apps/new](https://github.com/settings/apps/new)
    2. Preencha as informações básicas:
       * **Nome da GitHub App**: Escolha um nome único (por exemplo, "YourOrg Claude Assistant")
       * **URL da Homepage**: O site da sua organização ou a URL do repositório
    3. Configure as configurações da app:
       * **Webhooks**: Desmarque "Active" (não necessário para esta integração)
    4. Defina as permissões necessárias:
       * **Permissões do Repositório**:
         * Contents: Read & Write
         * Issues: Read & Write
         * Pull requests: Read & Write
    5. Clique em "Create GitHub App"
    6. Após a criação, clique em "Generate a private key" e salve o arquivo `.pem` baixado
    7. Anote seu App ID na página de configurações da app
    8. Instale a app em seu repositório:
       * Na página de configurações da sua app, clique em "Install App" na barra lateral esquerda
       * Selecione sua conta ou organização
       * Escolha "Only select repositories" e selecione o repositório específico
       * Clique em "Install"
    9. Adicione a chave privada como um secret ao seu repositório:
       * Vá para Settings → Secrets and variables → Actions do seu repositório
       * Crie um novo secret nomeado `APP_PRIVATE_KEY` com o conteúdo do arquivo `.pem`
    10. Adicione o App ID como um secret:

    * Crie um novo secret nomeado `APP_ID` com o ID da sua GitHub App

    <Note>
      Esta app será usada com a action [actions/create-github-app-token](https://github.com/actions/create-github-app-token) para gerar tokens de autenticação em seus fluxos de trabalho.
    </Note>

    **Alternativa para Claude API ou se você não quiser configurar sua própria Github app**: Use a app oficial do Anthropic:

    1. Instale de: [https://github.com/apps/claude](https://github.com/apps/claude)
    2. Nenhuma configuração adicional necessária para autenticação
  </Step>

  <Step title="Configure a autenticação do provedor de nuvem">
    Escolha seu provedor de nuvem e configure autenticação segura:

    <AccordionGroup>
      <Accordion title="Amazon Bedrock">
        **Configure a AWS para permitir que GitHub Actions se autentique com segurança sem armazenar credenciais.**

        > **Nota de Segurança**: Use configurações específicas do repositório e conceda apenas as permissões mínimas necessárias.

        **Configuração Necessária**:

        1. **Habilite Amazon Bedrock**:
           * Solicite acesso aos modelos Claude no Amazon Bedrock
           * Para modelos entre regiões, solicite acesso em todas as regiões necessárias

        2. **Configure GitHub OIDC Identity Provider**:
           * URL do Provedor: `https://token.actions.githubusercontent.com`
           * Audience: `sts.amazonaws.com`

        3. **Crie Função IAM para GitHub Actions**:
           * Tipo de entidade confiável: Web identity
           * Provedor de identidade: `token.actions.githubusercontent.com`
           * Permissões: política `AmazonBedrockFullAccess`
           * Configure política de confiança para seu repositório específico

        **Valores Necessários**:

        Após a configuração, você precisará:

        * **AWS\_ROLE\_TO\_ASSUME**: O ARN da função IAM que você criou

        <Tip>
          OIDC é mais seguro do que usar chaves de acesso AWS estáticas porque as credenciais são temporárias e rotacionadas automaticamente.
        </Tip>

        Consulte a [documentação da AWS](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html) para instruções detalhadas de configuração de OIDC.
      </Accordion>

      <Accordion title="Google Cloud's Agent Platform">
        **Configure o Google Cloud para permitir que GitHub Actions se autentique com segurança sem armazenar credenciais.**

        > **Nota de Segurança**: Use configurações específicas do repositório e conceda apenas as permissões mínimas necessárias.

        **Configuração Necessária**:

        1. **Habilite APIs** em seu projeto Google Cloud:
           * IAM Credentials API
           * Security Token Service (STS) API
           * Google Cloud's Agent Platform API

        2. **Crie recursos de Workload Identity Federation**:
           * Crie um Workload Identity Pool
           * Adicione um provedor OIDC do GitHub com:
             * Issuer: `https://token.actions.githubusercontent.com`
             * Mapeamentos de atributos para repositório e proprietário
             * **Recomendação de segurança**: Use condições de atributo específicas do repositório

        3. **Crie uma Conta de Serviço**:
           * Conceda apenas a função `Vertex AI User`
           * **Recomendação de segurança**: Crie uma conta de serviço dedicada por repositório

        4. **Configure vinculações IAM**:
           * Permita que o Workload Identity Pool represente a conta de serviço
           * **Recomendação de segurança**: Use conjuntos de principais específicos do repositório

        **Valores Necessários**:

        Após a configuração, você precisará:

        * **GCP\_WORKLOAD\_IDENTITY\_PROVIDER**: O nome completo do recurso do provedor
        * **GCP\_SERVICE\_ACCOUNT**: O endereço de email da conta de serviço

        <Tip>
          Workload Identity Federation elimina a necessidade de chaves de conta de serviço para download, melhorando a segurança.
        </Tip>

        Para instruções de configuração detalhadas, consulte a [documentação de Workload Identity Federation do Google Cloud](https://cloud.google.com/iam/docs/workload-identity-federation).
      </Accordion>
    </AccordionGroup>
  </Step>

  <Step title="Adicione Secrets Necessários">
    Adicione os seguintes secrets ao seu repositório (Settings → Secrets and variables → Actions):

    #### Para Claude API (Direto):

    1. **Para Autenticação de API**:
       * `ANTHROPIC_API_KEY`: Sua chave de API Claude de [console.anthropic.com](https://console.anthropic.com)

    2. **Para GitHub App (se usar sua própria app)**:
       * `APP_ID`: O ID da sua GitHub App
       * `APP_PRIVATE_KEY`: O conteúdo da chave privada (.pem)

    #### Para Google Cloud's Agent Platform

    1. **Para Autenticação GCP**:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER`
       * `GCP_SERVICE_ACCOUNT`

    2. **Para GitHub App (se usar sua própria app)**:
       * `APP_ID`: O ID da sua GitHub App
       * `APP_PRIVATE_KEY`: O conteúdo da chave privada (.pem)

    #### Para Amazon Bedrock

    1. **Para Autenticação AWS**:
       * `AWS_ROLE_TO_ASSUME`

    2. **Para GitHub App (se usar sua própria app)**:
       * `APP_ID`: O ID da sua GitHub App
       * `APP_PRIVATE_KEY`: O conteúdo da chave privada (.pem)
  </Step>

  <Step title="Crie arquivos de fluxo de trabalho">
    Crie arquivos de fluxo de trabalho do GitHub Actions que se integrem com seu provedor de nuvem. Os exemplos abaixo mostram configurações completas para Amazon Bedrock e Google Cloud's Agent Platform:

    <AccordionGroup>
      <Accordion title="Fluxo de trabalho Amazon Bedrock">
        **Pré-requisitos:**

        * Acesso ao Amazon Bedrock habilitado com permissões de modelo Claude
        * GitHub configurado como um provedor de identidade OIDC na AWS
        * Função IAM com permissões do Amazon Bedrock que confia no GitHub Actions

        **Secrets necessários do GitHub:**

        | Nome do Secret       | Descrição                                          |
        | -------------------- | -------------------------------------------------- |
        | `AWS_ROLE_TO_ASSUME` | ARN da função IAM para acesso ao Amazon Bedrock    |
        | `APP_ID`             | Seu ID de GitHub App (das configurações da app)    |
        | `APP_PRIVATE_KEY`    | A chave privada que você gerou para sua GitHub App |

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
          O formato de ID de modelo para Amazon Bedrock inclui um prefixo de região (por exemplo, `us.anthropic.claude-sonnet-4-6`).
        </Tip>
      </Accordion>

      <Accordion title="Fluxo de trabalho Google Cloud's Agent Platform">
        **Pré-requisitos:**

        * Google Cloud's Agent Platform API habilitada em seu projeto GCP
        * Workload Identity Federation configurada para GitHub
        * Conta de serviço com permissões do Google Cloud's Agent Platform

        **Secrets necessários do GitHub:**

        | Nome do Secret                   | Descrição                                                             |
        | -------------------------------- | --------------------------------------------------------------------- |
        | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Nome do recurso do provedor de identidade de workload                 |
        | `GCP_SERVICE_ACCOUNT`            | Email da conta de serviço com acesso ao Google Cloud's Agent Platform |
        | `APP_ID`                         | Seu ID de GitHub App (das configurações da app)                       |
        | `APP_PRIVATE_KEY`                | A chave privada que você gerou para sua GitHub App                    |

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
          O ID do projeto é recuperado automaticamente da etapa de autenticação do Google Cloud, portanto você não precisa codificá-lo.
        </Tip>
      </Accordion>
    </AccordionGroup>
  </Step>
</Steps>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude não responde aos comandos @claude
</h3>

Verifique se a GitHub App está instalada corretamente, confirme que os fluxos de trabalho estão habilitados, garanta que a chave de API está definida nos secrets do repositório e confirme que o comentário contém `@claude` (não `/claude`).

<h3 id="ci-not-running-on-claude’s-commits">
  CI não está sendo executado nos commits de Claude
</h3>

Garanta que você está usando a GitHub App ou app personalizada (não usuário Actions), verifique se os gatilhos do fluxo de trabalho incluem os eventos necessários e confirme que as permissões da app incluem gatilhos de CI.

<h3 id="authentication-errors">
  Erros de autenticação
</h3>

Confirme que a chave de API é válida e tem permissões suficientes. Para Amazon Bedrock ou Google Cloud's Agent Platform, verifique a configuração de credenciais e garanta que os secrets estejam nomeados corretamente nos fluxos de trabalho.

<h2 id="advanced-configuration">
  Configuração avançada
</h2>

<h3 id="action-parameters">
  Parâmetros da Action
</h3>

A Claude Code Action v1 usa uma configuração simplificada:

| Parâmetro             | Descrição                                                                              | Necessário |
| --------------------- | -------------------------------------------------------------------------------------- | ---------- |
| `prompt`              | Instruções para Claude (texto simples ou um nome de [skill](/docs/pt/skills))               | Não\*      |
| `claude_args`         | Argumentos de CLI passados para Claude Code                                            | Não        |
| `plugin_marketplaces` | Lista separada por quebra de linha de URLs Git do marketplace de plugins               | Não        |
| `plugins`             | Lista separada por quebra de linha de nomes de plugins para instalar antes da execução | Não        |
| `anthropic_api_key`   | Chave de API Claude                                                                    | Sim\*\*    |
| `github_token`        | Token do GitHub para acesso à API                                                      | Não        |
| `trigger_phrase`      | Frase de gatilho personalizada (padrão: "@claude")                                     | Não        |
| `use_bedrock`         | Use Amazon Bedrock em vez da Claude API                                                | Não        |
| `use_vertex`          | Use Google Cloud's Agent Platform em vez da Claude API                                 | Não        |

\*Prompt é opcional - quando omitido para comentários de issue/PR, Claude responde à frase de gatilho\
\*\*Necessário para Claude API direto, não para Amazon Bedrock ou Google Cloud's Agent Platform

<h4 id="pass-cli-arguments">
  Passe argumentos de CLI
</h4>

O parâmetro `claude_args` aceita qualquer argumento de CLI do Claude Code:

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Argumentos comuns:

* `--max-turns`: Máximo de turnos de conversa (padrão: 10)
* `--model`: Modelo a usar (por exemplo, `claude-sonnet-5`)
* `--mcp-config`: Caminho para configuração MCP
* `--allowedTools`: Lista separada por vírgula de ferramentas permitidas. O alias `--allowed-tools` também funciona.
* `--debug`: Habilitar saída de debug

<h3 id="alternative-integration-methods">
  Métodos de integração alternativos
</h3>

Enquanto o comando `/install-github-app` é a abordagem recomendada, você também pode:

* **GitHub App Personalizada**: Para organizações que precisam de nomes de usuário personalizados ou fluxos de autenticação personalizados. Crie sua própria GitHub App com permissões necessárias (contents, issues, pull requests) e use a action actions/create-github-app-token para gerar tokens em seus fluxos de trabalho.
* **GitHub Actions Manual**: Configuração direta de fluxo de trabalho para máxima flexibilidade
* **Configuração MCP**: Carregamento dinâmico de servidores Model Context Protocol

Consulte a [documentação do Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs) para guias detalhados sobre autenticação, segurança e configuração avançada.

<h3 id="customizing-claude’s-behavior">
  Personalizando o comportamento de Claude
</h3>

Você pode configurar o comportamento de Claude de duas maneiras:

1. **CLAUDE.md**: Defina padrões de codificação, critérios de revisão e regras específicas do projeto em um arquivo `CLAUDE.md` na raiz do seu repositório. Claude seguirá essas diretrizes ao criar PRs e responder a solicitações. Confira nossa [documentação de Memory](/docs/pt/memory) para mais detalhes.
2. **Prompts personalizados**: Use o parâmetro `prompt` no arquivo de fluxo de trabalho para fornecer instruções específicas do fluxo de trabalho. Isso permite que você personalize o comportamento de Claude para diferentes fluxos de trabalho ou tarefas.

Claude seguirá essas diretrizes ao criar PRs e responder a solicitações.
