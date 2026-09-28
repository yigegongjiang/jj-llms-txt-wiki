> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Saiba como integrar Claude Code no seu fluxo de trabalho de desenvolvimento com GitLab CI/CD

<Info>
  Claude Code para GitLab CI/CD está atualmente em beta. Os recursos e funcionalidades podem evoluir conforme refinamos a experiência.

  Esta integração é mantida pelo GitLab. Para obter suporte, consulte o seguinte [problema do GitLab](https://gitlab.com/gitlab-org/gitlab/-/issues/573776).
</Info>

<Note>
  Esta integração é construída sobre o [Claude Code CLI e Agent SDK](/docs/pt/agent-sdk/overview), permitindo o uso programático do Claude em seus trabalhos de CI/CD e fluxos de trabalho de automação personalizados.
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  Por que usar Claude Code com GitLab?
</h2>

* **Criação instantânea de MR**: Descreva o que você precisa, e Claude propõe um MR completo com alterações e explicação
* **Implementação automatizada**: Transforme problemas em código funcional com um único comando ou menção
* **Ciente do projeto**: Claude segue suas diretrizes `CLAUDE.md` e padrões de código existentes
* **Configuração simples**: Adicione um trabalho a `.gitlab-ci.yml` e uma variável de CI/CD mascarada
* **Pronto para empresas**: Escolha Claude API, Amazon Bedrock ou Google Cloud's Agent Platform para atender às necessidades de residência de dados e compras
* **Seguro por padrão**: Executa em seus executores GitLab com sua proteção de branch e aprovações

<h2 id="how-it-works">
  Como funciona
</h2>

Claude Code usa GitLab CI/CD para executar tarefas de IA em trabalhos isolados e confirmar resultados de volta via MRs:

1. **Orquestração orientada por eventos**: GitLab escuta seus gatilhos escolhidos (por exemplo, um comentário que menciona `@claude` em um problema, MR ou thread de revisão). O trabalho coleta contexto da thread e do repositório, constrói prompts a partir dessa entrada e executa Claude Code.

2. **Abstração de provedor**: Use o provedor que se adequa ao seu ambiente:
   * Claude API (SaaS)
   * Amazon Bedrock (acesso baseado em IAM, opções entre regiões)
   * Google Cloud's Agent Platform (nativo do GCP, Workload Identity Federation)

3. **Execução em sandbox**: Cada interação é executada em um contêiner com regras rigorosas de rede e sistema de arquivos. Claude Code impõe permissões com escopo de workspace para restringir gravações. Cada alteração flui através de um MR para que os revisores vejam o diff e as aprovações ainda se apliquem.

Escolha endpoints regionais para reduzir latência e atender aos requisitos de soberania de dados enquanto usa acordos de nuvem existentes.

<h2 id="what-can-claude-do">
  O que Claude pode fazer?
</h2>

Em um pipeline do GitLab, Claude Code pode:

* Criar e atualizar MRs a partir de descrições ou comentários de issues
* Analisar regressões de desempenho e propor otimizações
* Implementar recursos diretamente em uma branch, depois abrir uma MR
* Corrigir bugs e regressões identificados por testes ou comentários
* Responder a comentários de acompanhamento para iterar sobre as alterações solicitadas

<h2 id="setup">
  Configuração
</h2>

<h3 id="quick-setup">
  Configuração rápida
</h3>

A forma mais rápida de começar é adicionar um job mínimo ao seu `.gitlab-ci.yml` e definir sua chave de API como uma variável mascarada.

1. **Adicione uma variável CI/CD mascarada**
   * Vá para **Settings** → **CI/CD** → **Variables**
   * Adicione `ANTHROPIC_API_KEY` (mascarada, protegida conforme necessário)

2. **Adicione um job Claude ao `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Ajuste as regras para se adequar a como você deseja disparar o job:
  # - execuções manuais
  # - eventos de merge request
  # - acionadores web/API quando um comentário contém '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # O instalador coloca claude em ~/.local/bin, que não está no PATH nesta imagem
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # Opcional: inicie um servidor GitLab MCP se sua configuração fornecer um
    - /bin/gitlab-mcp-server || true
    # Use variáveis AI_FLOW_* ao invocar via acionadores web/API com payloads de contexto
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Após adicionar o job e sua variável `ANTHROPIC_API_KEY`, teste executando o job manualmente em **CI/CD** → **Pipelines**, ou dispare-o a partir de um MR para deixar Claude propor atualizações em uma branch e abrir um MR se necessário.

<Note>
  Para executar no Amazon Bedrock ou na Agent Platform do Google Cloud em vez da Claude API, consulte a seção [Using with Amazon Bedrock and Google Cloud](#using-with-amazon-bedrock-and-google-cloud) abaixo para configuração de autenticação e ambiente.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Configuração manual (recomendada para produção)
</h3>

Se você preferir uma configuração mais controlada ou precisar de provedores corporativos:

1. **Configure o acesso ao provedor**:
   * **Claude API**: Crie e armazene `ANTHROPIC_API_KEY` como uma variável CI/CD mascarada
   * **Amazon Bedrock**: **Configure GitLab** → **AWS OIDC** e crie uma função IAM para Amazon Bedrock
   * **Agent Platform do Google Cloud**: **Configure Workload Identity Federation for GitLab** → **GCP**

2. **Adicione credenciais de projeto para operações da API GitLab**:
   * Use `CI_JOB_TOKEN` por padrão, ou crie um Project Access Token com escopo `api`
   * Armazene como `GITLAB_ACCESS_TOKEN` (mascarada) se usar um PAT

3. **Adicione o job Claude ao `.gitlab-ci.yml`**: use o job [Quick setup](#quick-setup) para a Claude API, ou um job de provedor de [Configuration examples](#configuration-examples)

4. **(Opcional) Ative acionadores acionados por menção**:
   * Adicione um webhook de projeto para "Comments (notes)" ao seu ouvinte de eventos (se você usar um)
   * Faça o ouvinte chamar a API de acionamento de pipeline com variáveis como `AI_FLOW_INPUT` e `AI_FLOW_CONTEXT` quando um comentário contiver `@claude`

<h2 id="example-use-cases">
  Exemplos de casos de uso
</h2>

<h3 id="turn-issues-into-mrs">
  Transformar problemas em MRs
</h3>

Em um comentário de problema:

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude analisa o problema e a base de código, escreve alterações em uma ramificação e abre um MR para revisão.

<h3 id="get-implementation-help">
  Obter ajuda com implementação
</h3>

Em uma discussão de MR:

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude propõe alterações, adiciona código com cache apropriado e atualiza o MR.

<h3 id="fix-bugs-quickly">
  Corrigir bugs rapidamente
</h3>

Em um comentário de problema ou MR:

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude localiza o bug, implementa uma correção e atualiza a ramificação ou abre um novo MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Usando com Amazon Bedrock e Google Cloud
</h2>

Para ambientes empresariais, você pode executar Claude Code inteiramente em sua infraestrutura de nuvem com a mesma experiência de desenvolvedor.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Pré-requisitos

    Antes de configurar Claude Code com Amazon Bedrock, você precisa de:

    1. Uma conta AWS com acesso ao Amazon Bedrock para os modelos Claude desejados
    2. GitLab configurado como um provedor de identidade OIDC no AWS IAM
    3. Uma função IAM com permissões do Amazon Bedrock e uma política de confiança restrita ao seu projeto/refs do GitLab
    4. Variáveis do GitLab CI/CD para assunção de função:
       * `AWS_ROLE_TO_ASSUME` (ARN da função)
       * `AWS_REGION` (região do Amazon Bedrock)

    ### Instruções de configuração

    Configure AWS para permitir que trabalhos do GitLab CI assumam uma função IAM via OIDC (sem chaves estáticas).

    **Configuração obrigatória:**

    1. Ative o Amazon Bedrock e solicite acesso aos seus modelos Claude de destino
    2. Crie um provedor OIDC do IAM para GitLab se ainda não estiver presente
    3. Crie uma função IAM confiável pelo provedor OIDC do GitLab, restrita ao seu projeto e refs protegidas
    4. Anexe permissões de menor privilégio para APIs de invocação do Amazon Bedrock

    Use o [exemplo de trabalho do Amazon Bedrock](#configuration-examples) para trocar o token OIDC do trabalho por credenciais AWS temporárias em tempo de execução.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Pré-requisitos

    Antes de configurar Claude Code com Google Cloud's Agent Platform, você precisa de:

    1. Um projeto Google Cloud com:
       * API do Google Cloud's Agent Platform ativada
       * Workload Identity Federation configurada para confiar no OIDC do GitLab
    2. Uma conta de serviço dedicada com apenas as funções necessárias do Google Cloud's Agent Platform
    3. Variáveis do GitLab CI/CD:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (nome do recurso do provedor sem o prefixo `//iam.googleapis.com/`, como `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`)
       * `GCP_SERVICE_ACCOUNT` (email da conta de serviço)
       * `GCP_PROJECT_ID` (ID do projeto Google Cloud)

    ### Instruções de configuração

    Configure Google Cloud para permitir que trabalhos do GitLab CI representem uma conta de serviço via Workload Identity Federation.

    **Configuração obrigatória:**

    1. Ative a API de Credenciais do IAM, API STS e API do Google Cloud's Agent Platform
    2. Crie um Workload Identity Pool e provedor para OIDC do GitLab
    3. Crie uma conta de serviço dedicada com funções do Google Cloud's Agent Platform
    4. Conceda ao principal do WIF permissão para representar a conta de serviço

    Use o [exemplo de trabalho do Agent Platform](#configuration-examples) para autenticar sem armazenar chaves.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Exemplos de configuração
</h2>

Abaixo estão trechos prontos para uso que você pode adaptar ao seu pipeline.

<h3 id="amazon-bedrock-job-example-oidc">
  Exemplo de job do Amazon Bedrock (OIDC)
</h3>

**Pré-requisitos:**

* Amazon Bedrock habilitado com acesso ao(s) modelo(s) Claude escolhido(s)
* OIDC do GitLab configurado na AWS com uma função que confia no seu projeto GitLab e refs
* Função IAM com permissões do Amazon Bedrock (privilégio mínimo recomendado)

**Variáveis CI/CD obrigatórias:**

* `AWS_ROLE_TO_ASSUME`: ARN da função IAM para acesso ao Amazon Bedrock
* `AWS_REGION`: região do Amazon Bedrock (por exemplo, `us-west-2`)

O GitLab cria o token OIDC do job a partir do bloco `id_tokens:` e o expõe como `GITLAB_OIDC_TOKEN`. Defina `aud` para o valor de audiência que você configurou no provedor de identidade OIDC do IAM na AWS, por exemplo, a URL da sua instância GitLab.

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
  Os IDs de modelo para o Amazon Bedrock incluem prefixos específicos da região (por exemplo, `us.anthropic.claude-sonnet-4-6`). Passe o modelo desejado através da configuração do seu job ou prompt se seu fluxo de trabalho suportar.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Exemplo de job do Agent Platform (Workload Identity Federation)
</h3>

**Pré-requisitos:**

* API do Agent Platform do Google Cloud habilitada no seu projeto GCP
* Workload Identity Federation configurada para confiar no OIDC do GitLab
* Uma conta de serviço com permissões do Agent Platform do Google Cloud

**Variáveis CI/CD obrigatórias:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: nome do recurso do provedor sem o prefixo `//iam.googleapis.com/`, como `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`
* `GCP_SERVICE_ACCOUNT`: email da conta de serviço
* `GCP_PROJECT_ID`: ID do projeto Google Cloud
* `CLOUD_ML_REGION`: região do Agent Platform do Google Cloud (por exemplo, `us-east5`)

O GitLab cria o token OIDC do job a partir do bloco `id_tokens:` e o expõe como `GITLAB_OIDC_TOKEN`. Defina `aud` para o valor de audiência que você configurou no provedor do Workload Identity Pool, por exemplo, a URL da sua instância GitLab. O job escreve o token em um arquivo, e a entrada `credential_source` da configuração de credenciais diz às bibliotecas de autenticação do Google para lê-lo de lá. Definir `GOOGLE_APPLICATION_CREDENTIALS` para o arquivo de configuração de credenciais o torna disponível para Claude Code através de [Application Default Credentials](/docs/pt/google-vertex-ai#3-configure-gcp-credentials).

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
  Com o Workload Identity Federation, você não precisa armazenar chaves de conta de serviço. Use condições de confiança específicas do repositório e contas de serviço com privilégio mínimo.
</Note>

<h2 id="best-practices">
  Melhores práticas
</h2>

<h3 id="claude-md-configuration">
  Configuração CLAUDE.md
</h3>

Crie um arquivo `CLAUDE.md` na raiz do repositório para definir padrões de codificação, critérios de revisão e regras específicas do projeto. Claude lê este arquivo durante as execuções e segue suas convenções ao propor alterações.

<h3 id="security-considerations">
  Considerações de segurança
</h3>

**Nunca faça commit de chaves de API ou credenciais de nuvem no seu repositório**. Sempre use variáveis de GitLab CI/CD:

* Adicione `ANTHROPIC_API_KEY` como uma variável mascarada (e proteja-a se necessário)
* Use OIDC específico do provedor quando possível (sem chaves de longa duração)
* Limite as permissões de trabalho e a saída de rede
* Revise os MRs do Claude como qualquer outro colaborador

<h3 id="optimizing-performance">
  Otimizando o desempenho
</h3>

* Mantenha `CLAUDE.md` focado e conciso
* Forneça descrições claras de issue/MR para reduzir iterações
* Armazene em cache npm e instalações de pacotes em runners quando possível

<h3 id="ci-costs">
  Custos de CI
</h3>

Ao usar Claude Code com GitLab CI/CD, esteja ciente dos custos associados:

* **Tempo do GitLab Runner**:
  * Claude é executado em seus runners do GitLab e consome minutos de computação
  * Consulte os detalhes de faturamento do runner do seu plano GitLab

* **Custos de API**:
  * Cada interação do Claude consome tokens com base no tamanho do prompt e da resposta
  * O uso de tokens varia de acordo com a complexidade da tarefa e o tamanho da base de código
  * Consulte [Preços da Anthropic](https://platform.claude.com/docs/en/about-claude/pricing) para detalhes

* **Dicas de otimização de custos**:
  * Use comandos `@claude` específicos para reduzir turnos desnecessários
  * Defina valores apropriados de `--max-turns` e `timeout` de trabalho
  * Limite a concorrência para controlar execuções paralelas

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude não responde aos comandos @claude
</h3>

* Verifique se seu pipeline está sendo acionado (manualmente, evento MR ou via listener de evento de nota/webhook)
* Certifique-se de que suas variáveis `ANTHROPIC_API_KEY` ou do provedor de nuvem estão presentes
* Verifique se o comentário contém `@claude` (não `/claude`) e se seu gatilho de menção está configurado

<h3 id="job-can’t-write-comments-or-open-mrs">
  Job não consegue escrever comentários ou abrir MRs
</h3>

* Certifique-se de que `CI_JOB_TOKEN` tem permissões suficientes para o projeto, ou use um Project Access Token com escopo `api`
* Verifique se a ferramenta `mcp__gitlab` está habilitada em `--allowedTools`
* Confirme se o job é executado no contexto do MR ou tem contexto suficiente via variáveis `AI_FLOW_*`

<h3 id="authentication-errors">
  Erros de autenticação
</h3>

* **Para Claude API**: Confirme que `ANTHROPIC_API_KEY` é válida e não expirou
* **Para Amazon Bedrock ou Google Cloud's Agent Platform**: Verifique a configuração OIDC/WIF, impersonação de função e nomes de segredos; confirme a disponibilidade de região e modelo

<h2 id="advanced-configuration">
  Configuração avançada
</h2>

<h3 id="common-parameters-and-variables">
  Parâmetros e variáveis comuns
</h3>

Controle as execuções do Claude Code em seus jobs com esses sinalizadores CLI, palavras-chave do GitLab e variáveis:

* `-p`: forneça instruções inline, por exemplo `claude -p "Review this MR"`
* `--max-turns`: limite o número de iterações de ida e volta
* `timeout`: limite o tempo total de execução do job com a palavra-chave `timeout` de nível de job do GitLab, por exemplo `timeout: 30m`
* `ANTHROPIC_API_KEY`: obrigatório para a API Claude (não usado para Amazon Bedrock ou Agent Platform do Google Cloud)
* Ambiente específico do provedor: `AWS_REGION`, variáveis de projeto/região para Agent Platform do Google Cloud

<Note>
  Os sinalizadores e parâmetros exatos podem variar dependendo da versão de `@anthropic-ai/claude-code`. Execute `claude --help` em seu job para ver as opções suportadas.
</Note>

<h3 id="customizing-claude’s-behavior">
  Personalizando o comportamento do Claude
</h3>

Você pode orientar o Claude de duas maneiras principais:

1. **CLAUDE.md**: Defina padrões de codificação, requisitos de segurança e convenções de projeto. Claude lê isso durante as execuções e segue suas regras.
2. **Prompts personalizados**: Passe instruções específicas da tarefa via `-p` no job. Use prompts diferentes para jobs diferentes (por exemplo, review, implement, refactor).
