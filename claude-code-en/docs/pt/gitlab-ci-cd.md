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

Claude Code permite fluxos de trabalho poderosos de CI/CD que transformam a forma como você trabalha com código:

* Criar e atualizar MRs a partir de descrições ou comentários de problemas
* Analisar regressões de desempenho e propor otimizações
* Implementar recursos diretamente em um branch, depois abrir um MR
* Corrigir bugs e regressões identificados por testes ou comentários
* Responder a comentários de acompanhamento para iterar sobre as alterações solicitadas

<h2 id="setup">
  Configuração
</h2>

<h3 id="quick-setup">
  Configuração rápida
</h3>

A forma mais rápida de começar é adicionar um trabalho mínimo ao seu `.gitlab-ci.yml` e definir sua chave de API como uma variável mascarada.

1. **Adicione uma variável de CI/CD mascarada**
   * Vá para **Settings** → **CI/CD** → **Variables**
   * Adicione `ANTHROPIC_API_KEY` (mascarada, protegida conforme necessário)

2. **Adicione um trabalho Claude ao `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Ajuste as regras para se adequar a como você deseja disparar o trabalho:
  # - execuções manuais
  # - eventos de merge request
  # - gatilhos web/API quando um comentário contém '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    # Opcional: inicie um servidor GitLab MCP se sua configuração fornecer um
    - /bin/gitlab-mcp-server || true
    # Use variáveis AI_FLOW_* ao invocar via gatilhos web/API com payloads de contexto
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Após adicionar o trabalho e sua variável `ANTHROPIC_API_KEY`, teste executando o trabalho manualmente em **CI/CD** → **Pipelines**, ou dispare-o a partir de um MR para deixar Claude propor atualizações em um branch e abrir um MR se necessário.

<Note>
  Para executar no Amazon Bedrock ou na Plataforma de Agentes do Google Cloud em vez da Claude API, consulte a seção [Usando com Amazon Bedrock e Google Cloud](#using-with-amazon-bedrock-and-google-cloud) abaixo para configuração de autenticação e ambiente.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Configuração manual (recomendada para produção)
</h3>

Se você preferir uma configuração mais controlada ou precisar de provedores corporativos:

1. **Configure o acesso do provedor**:
   * **Claude API**: Crie e armazene `ANTHROPIC_API_KEY` como uma variável de CI/CD mascarada
   * **Amazon Bedrock**: **Configure GitLab** → **AWS OIDC** e crie uma função IAM para Amazon Bedrock
   * **Plataforma de Agentes do Google Cloud**: **Configure Workload Identity Federation para GitLab** → **GCP**

2. **Adicione credenciais de projeto para operações da API GitLab**:
   * Use `CI_JOB_TOKEN` por padrão, ou crie um Project Access Token com escopo `api`
   * Armazene como `GITLAB_ACCESS_TOKEN` (mascarado) se usar um PAT

3. **Adicione o trabalho Claude ao `.gitlab-ci.yml`** (veja exemplos abaixo)

4. **(Opcional) Ative gatilhos orientados por menção**:
   * Adicione um webhook de projeto para "Comments (notes)" ao seu ouvinte de eventos (se você usar um)
   * Faça o ouvinte chamar a API de gatilho de pipeline com variáveis como `AI_FLOW_INPUT` e `AI_FLOW_CONTEXT` quando um comentário contiver `@claude`

<h2 id="example-use-cases">
  Exemplos de casos de uso
</h2>

<h3 id="turn-issues-into-mrs">
  Transforme problemas em MRs
</h3>

Em um comentário de problema:

```text theme={null}
@claude implement this feature based on the issue description
```

Claude analisa o problema e a base de código, escreve alterações em um branch e abre um MR para revisão.

<h3 id="get-implementation-help">
  Obtenha ajuda de implementação
</h3>

Em uma discussão de MR:

```text theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude propõe alterações, adiciona código com cache apropriado e atualiza o MR.

<h3 id="fix-bugs-quickly">
  Corrija bugs rapidamente
</h3>

Em um comentário de problema ou MR:

```text theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude localiza o bug, implementa uma correção e atualiza o branch ou abre um novo MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Usando com Amazon Bedrock e Google Cloud
</h2>

Para ambientes corporativos, você pode executar Claude Code inteiramente em sua infraestrutura de nuvem com a mesma experiência do desenvolvedor.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Pré-requisitos

    Antes de configurar Claude Code com Amazon Bedrock, você precisa de:

    1. Uma conta AWS com acesso ao Amazon Bedrock para os modelos Claude desejados
    2. GitLab configurado como um provedor de identidade OIDC no AWS IAM
    3. Uma função IAM com permissões de Amazon Bedrock e uma política de confiança restrita ao seu projeto/refs do GitLab
    4. Variáveis de CI/CD do GitLab para assunção de função:
       * `AWS_ROLE_TO_ASSUME` (ARN da função)
       * `AWS_REGION` (região do Amazon Bedrock)

    ### Instruções de configuração

    Configure AWS para permitir que trabalhos de CI do GitLab assumam uma função IAM via OIDC (sem chaves estáticas).

    **Configuração necessária:**

    1. Ative Amazon Bedrock e solicite acesso aos seus modelos Claude alvo
    2. Crie um provedor OIDC do IAM para GitLab se ainda não estiver presente
    3. Crie uma função IAM confiável pelo provedor OIDC do GitLab, restrita ao seu projeto e refs protegidos
    4. Anexe permissões de privilégio mínimo para APIs de invocação do Amazon Bedrock

    **Valores necessários para armazenar em variáveis de CI/CD:**

    * `AWS_ROLE_TO_ASSUME`
    * `AWS_REGION`

    Adicione variáveis em Settings → CI/CD → Variables:

    ```yaml theme={null}
    # Para Amazon Bedrock:
    - AWS_ROLE_TO_ASSUME
    - AWS_REGION
    ```

    Use o exemplo de trabalho do Amazon Bedrock acima para trocar o token de trabalho do GitLab por credenciais AWS temporárias em tempo de execução.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Pré-requisitos

    Antes de configurar Claude Code com Google Cloud's Agent Platform, você precisa de:

    1. Um projeto Google Cloud com:
       * Google Cloud's Agent Platform API habilitada
       * Workload Identity Federation configurada para confiar no OIDC do GitLab
    2. Uma conta de serviço dedicada com apenas as funções necessárias do Google Cloud's Agent Platform
    3. Variáveis de CI/CD do GitLab para WIF:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (nome completo do recurso)
       * `GCP_SERVICE_ACCOUNT` (email da conta de serviço)

    ### Instruções de configuração

    Configure Google Cloud para permitir que trabalhos de CI do GitLab representem uma conta de serviço via Workload Identity Federation.

    **Configuração necessária:**

    1. Ative IAM Credentials API, STS API e Google Cloud's Agent Platform API
    2. Crie um Workload Identity Pool e provedor para OIDC do GitLab
    3. Crie uma conta de serviço dedicada com funções do Google Cloud's Agent Platform
    4. Conceda ao principal WIF permissão para representar a conta de serviço

    **Valores necessários para armazenar em variáveis de CI/CD:**

    * `GCP_WORKLOAD_IDENTITY_PROVIDER`
    * `GCP_SERVICE_ACCOUNT`

    Adicione variáveis em Settings → CI/CD → Variables:

    ```yaml theme={null}
    # Para Google Cloud's Agent Platform:
    - GCP_WORKLOAD_IDENTITY_PROVIDER
    - GCP_SERVICE_ACCOUNT
    - CLOUD_ML_REGION (por exemplo, us-east5)
    ```

    Use o exemplo de trabalho do Google Cloud's Agent Platform acima para autenticar sem armazenar chaves.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Exemplos de configuração
</h2>

Abaixo estão trechos prontos para usar que você pode adaptar ao seu pipeline.

<h3 id="basic-gitlab-ci-yml-claude-api">
  .gitlab-ci.yml básico (Claude API)
</h3>

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Summarize recent changes and suggest improvements'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  # Claude Code usará ANTHROPIC_API_KEY das variáveis de CI/CD
```

<h3 id="amazon-bedrock-job-example-oidc">
  Exemplo de trabalho Amazon Bedrock (OIDC)
</h3>

**Pré-requisitos:**

* Amazon Bedrock habilitado com acesso ao seu modelo Claude escolhido
* OIDC do GitLab configurado no AWS com uma função que confia no seu projeto e refs do GitLab
* Função IAM com permissões de Bedrock (privilégio mínimo recomendado)

**Variáveis de CI/CD necessárias:**

* `AWS_ROLE_TO_ASSUME`: ARN da função IAM para acesso ao Bedrock
* `AWS_REGION`: Região do Bedrock (por exemplo, `us-west-2`)

```yaml theme={null}
claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apk add --no-cache bash curl jq git python3 py3-pip
    - pip install --no-cache-dir awscli
    - curl -fsSL https://claude.ai/install.sh | bash
    # Troque o token OIDC do GitLab por credenciais AWS
    - export AWS_WEB_IDENTITY_TOKEN_FILE="${CI_JOB_JWT_FILE:-/tmp/oidc_token}"
    - if [ -n "${CI_JOB_JWT_V2}" ]; then printf "%s" "$CI_JOB_JWT_V2" > "$AWS_WEB_IDENTITY_TOKEN_FILE"; fi
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
```

<Note>
  IDs de modelo para Bedrock incluem prefixos específicos de região (por exemplo, `us.anthropic.claude-sonnet-4-6`). Passe o modelo desejado via sua configuração de trabalho ou prompt se seu fluxo de trabalho suportar.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Exemplo de trabalho Agent Platform (Workload Identity Federation)
</h3>

**Pré-requisitos:**

* API Agent Platform do Google Cloud habilitada em seu projeto GCP
* Workload Identity Federation configurada para confiar no OIDC do GitLab
* Uma conta de serviço com permissões Agent Platform do Google Cloud

**Variáveis de CI/CD necessárias:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: Nome completo do recurso do provedor
* `GCP_SERVICE_ACCOUNT`: Email da conta de serviço
* `CLOUD_ML_REGION`: Região do Agent Platform do Google Cloud (por exemplo, `us-east5`)

```yaml theme={null}
claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # Autentique no Google Cloud via WIF (sem chaves baixadas)
    - >
      gcloud auth login --cred-file=<(cat <<EOF
      {
        "type": "external_account",
        "audience": "${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken",
        "token_url": "https://sts.googleapis.com/v1/token"
      }
      EOF
      )
    - gcloud config set project "$(gcloud projects list --format='value(projectId)' --filter="name:${CI_PROJECT_NAMESPACE}" | head -n1)" || true
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
```

<Note>
  Com Workload Identity Federation, você não precisa armazenar chaves de conta de serviço. Use condições de confiança específicas do repositório e contas de serviço com privilégio mínimo.
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

**Nunca confirme chaves de API ou credenciais de nuvem em seu repositório**. Sempre use variáveis de CI/CD do GitLab:

* Adicione `ANTHROPIC_API_KEY` como uma variável mascarada (e proteja-a se necessário)
* Use OIDC específico do provedor onde possível (sem chaves de longa duração)
* Limite permissões de trabalho e saída de rede
* Revise os MRs do Claude como qualquer outro colaborador

<h3 id="optimizing-performance">
  Otimizando desempenho
</h3>

* Mantenha `CLAUDE.md` focado e conciso
* Forneça descrições claras de problema/MR para reduzir iterações
* Configure timeouts de trabalho sensatos para evitar execuções descontroladas
* Cache npm e instalações de pacotes em executores onde possível

<h3 id="ci-costs">
  Custos de CI
</h3>

Ao usar Claude Code com GitLab CI/CD, esteja ciente dos custos associados:

* **Tempo do GitLab Runner**:
  * Claude é executado em seus executores GitLab e consome minutos de computação
  * Consulte a cobrança de executor do seu plano GitLab para detalhes

* **Custos de API**:
  * Cada interação do Claude consome tokens com base no tamanho do prompt e resposta
  * O uso de tokens varia pela complexidade da tarefa e tamanho da base de código
  * Consulte [Preços da Anthropic](https://platform.claude.com/docs/pt/about-claude/pricing) para detalhes

* **Dicas de otimização de custos**:
  * Use comandos `@claude` específicos para reduzir turnos desnecessários
  * Defina valores apropriados de `max_turns` e timeout de trabalho
  * Limite concorrência para controlar execuções paralelas

<h2 id="security-and-governance">
  Segurança e governança
</h2>

* Cada trabalho é executado em um contêiner isolado com acesso de rede restrito
* As alterações do Claude fluem através de MRs para que os revisores vejam cada diff
* Regras de proteção de branch e aprovação se aplicam ao código gerado por IA
* Claude Code usa permissões com escopo de workspace para restringir gravações
* Os custos permanecem sob seu controle porque você traz suas próprias credenciais de provedor

<h2 id="troubleshooting">
  Solução de problemas
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude não responde aos comandos @claude
</h3>

* Verifique se seu pipeline está sendo disparado (manualmente, evento de MR ou via ouvinte de nota/webhook)
* Certifique-se de que as variáveis de CI/CD (`ANTHROPIC_API_KEY` ou configurações de provedor de nuvem) estão presentes e desmascaradas
* Verifique se o comentário contém `@claude` (não `/claude`) e se seu gatilho de menção está configurado

<h3 id="job-can’t-write-comments-or-open-mrs">
  O trabalho não consegue escrever comentários ou abrir MRs
</h3>

* Certifique-se de que `CI_JOB_TOKEN` tem permissões suficientes para o projeto, ou use um Project Access Token com escopo `api`
* Verifique se a ferramenta `mcp__gitlab` está habilitada em `--allowedTools`
* Confirme se o trabalho é executado no contexto do MR ou tem contexto suficiente via variáveis `AI_FLOW_*`

<h3 id="authentication-errors">
  Erros de autenticação
</h3>

* **Para Claude API**: Confirme que `ANTHROPIC_API_KEY` é válida e não expirou
* **Para Amazon Bedrock ou Google Cloud's Agent Platform**: Verifique configuração de OIDC/WIF, representação de função e nomes de segredos; confirme disponibilidade de região e modelo

<h2 id="advanced-configuration">
  Configuração avançada
</h2>

<h3 id="common-parameters-and-variables">
  Parâmetros e variáveis comuns
</h3>

Claude Code suporta estas entradas comumente usadas:

* `prompt` / `prompt_file`: Forneça instruções inline (`-p`) ou via arquivo
* `max_turns`: Limite o número de iterações de ida e volta
* `timeout_minutes`: Limite o tempo total de execução
* `ANTHROPIC_API_KEY`: Necessário para Claude API (não usado para Amazon Bedrock ou Google Cloud's Agent Platform)
* Ambiente específico do provedor: `AWS_REGION`, variáveis de projeto/região para Google Cloud's Agent Platform

<Note>
  Sinalizadores e parâmetros exatos podem variar por versão de `@anthropic-ai/claude-code`. Execute `claude --help` em seu trabalho para ver as opções suportadas.
</Note>

<h3 id="customizing-claude’s-behavior">
  Personalizando o comportamento do Claude
</h3>

Você pode guiar Claude de duas formas principais:

1. **CLAUDE.md**: Defina padrões de codificação, requisitos de segurança e convenções de projeto. Claude lê isso durante as execuções e segue suas regras.
2. **Prompts personalizados**: Passe instruções específicas da tarefa via `prompt`/`prompt_file` no trabalho. Use prompts diferentes para trabalhos diferentes (por exemplo, revisão, implementação, refatoração).
