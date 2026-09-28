> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implantar gateway de aplicativos Claude na AWS

> Um exemplo prático de execução do gateway de aplicativos Claude na AWS: ECS Fargate ou EKS, Amazon RDS para PostgreSQL, AWS Secrets Manager e autenticação de função IAM para Amazon Bedrock.

<Note>
  Esta página apresenta uma forma de executar o gateway de aplicativos Claude na AWS. A configuração é um exemplo funcional para infraestrutura gerenciada pelo cliente em vez de uma implantação de produção suportada; use-a para ver como as peças se encaixam antes de adaptá-la ao seu próprio ambiente. Para os requisitos independentes de plataforma, consulte o [guia de implantação](/docs/pt/claude-apps-gateway-deploy).
</Note>

Este exemplo provisiona o gateway de aplicativos Claude na AWS com Amazon Bedrock como upstream de modelo, usando [Amazon ECS](https://aws.amazon.com/ecs/) em [AWS Fargate](https://aws.amazon.com/fargate/) ou [Amazon EKS](https://aws.amazon.com/eks/) para computação. [Okta](https://www.okta.com/) é o provedor de identidade (IdP) de exemplo, mas qualquer IdP compatível com OpenID Connect (OIDC) funciona; consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup) para detalhes específicos de cada IdP.

<Note>
  Bedrock não é o único upstream Claude na AWS. O gateway também suporta Claude Platform on AWS, a API Claude operada pela Anthropic com autenticação AWS e faturamento do AWS Marketplace, no lugar de Bedrock ou junto com ele. Sua entrada upstream, credenciais e permissões IAM diferem das específicas de Bedrock desta página; a [referência de upstream Claude Platform on AWS](/docs/pt/claude-apps-gateway-config#claude-platform-on-aws) cobre o que muda, e o resto desta página se aplica sem alterações.
</Note>

<h2 id="architecture">
  Arquitetura
</h2>

<Frame caption="A arquitetura de exemplo, com Amazon Bedrock como upstream de modelo. Um upstream Claude Platform on AWS ocupa a mesma posição.">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="Diagrama do gateway de aplicativos Claude na AWS: clientes Claude Code se conectam via HTTPS a um Application Load Balancer interno que fica na frente do gateway (ECS Fargate ou EKS), que é executado em subnets privadas junto com uma instância Amazon RDS para PostgreSQL para estado de sessão. O gateway faz login dos usuários via OIDC contra o IdP corporativo, lê segredos do AWS Secrets Manager, encaminha solicitações de modelo para Amazon Bedrock usando sua função IAM e extrai sua imagem do Amazon ECR na implantação." width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

O gateway é executado como um endpoint HTTPS privado em sua rede ao qual os desenvolvedores fazem login através de seu IdP. Suas sessões Claude Code alcançam modelos Claude no Amazon Bedrock através da função IAM do gateway, portanto nenhuma credencial de modelo chega às máquinas dos desenvolvedores. A configuração de referência provisiona:

* Serviço **Amazon ECS em AWS Fargate** ou **Amazon EKS** Deployment executando o contêiner do gateway
* Repositório **Amazon ECR** para a imagem do gateway
* Instância **Amazon RDS para PostgreSQL** em subnets privadas, não acessível publicamente, para o [store](/docs/pt/claude-apps-gateway-config#store) do gateway
* Segredos **AWS Secrets Manager** para a chave de assinatura JWT, o segredo do cliente OIDC e a URL do Postgres
* **Função IAM** com `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream` e `bedrock:CountTokens`, anexada como função de tarefa ECS ou vinculada via IAM Roles for Service Accounts (IRSA) no EKS
* **Application Load Balancer interno** para HTTPS

<h2 id="prerequisites">
  Pré-requisitos
</h2>

O passo a passo cria os próprios recursos do gateway, mas se baseia em infraestrutura de rede e identidade que você já possui. Antes de começar, você precisa:

* Uma conta AWS com permissão para criar os [recursos acima](#architecture)
* [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) instalada e [autenticada](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html), e [Docker](https://docs.docker.com/get-started/get-docker/) instalado localmente
* Uma [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html) com pelo menos duas [subnets privadas](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html) em diferentes Zonas de Disponibilidade, com acesso à internet de saída através de um [gateway NAT](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html); o balanceador de carga interno precisa de subnets em duas AZs, e o gateway precisa de saída para Bedrock e seu IdP
* Uma aplicação web OIDC Okta com URI de redirecionamento `https://<gateway-host>/oauth/callback`; consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup)
* Um nome de host TLS para o gateway, normalmente um nome DNS interno em uma [zona hospedada privada Route 53](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html) apontando para o balanceador de carga, com um [certificado ACM](https://docs.aws.amazon.com/acm/latest/userguide/gs.html) para esse nome, importado ou emitido por [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)

<h3 id="set-your-environment-variables">
  Defina suas variáveis de ambiente
</h3>

Cada comando nesta página lê quatro valores do seu shell: `AWS_REGION`, `ACCOUNT_ID`, `VPC_ID` e `PRIVATE_SUBNETS`.

Escolha uma região dos EUA onde Bedrock serve os modelos Claude que você precisa. O passo a passo depende do catálogo de modelos integrado do gateway, que resolve para perfis de inferência `us.anthropic.*`, e a política IAM concede esses ARNs. Em uma região fora dos EUA, adicione um [bloco `models:`](/docs/pt/claude-apps-gateway-config#models) com os IDs de perfil de inferência dessa região geográfica e altere o prefixo ARN da política IAM para corresponder.

Se você não tiver o ID da VPC à mão, liste suas VPCs com `aws ec2 describe-vpcs`, depois liste as subnets dessa VPC para encontrar duas privadas em diferentes Zonas de Disponibilidade:

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

Exporte todos os quatro antes de continuar:

```bash theme={null}
export AWS_REGION=us-east-1   # uma região dos EUA onde Bedrock serve os modelos Claude que você precisa
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  Implante o gateway
</h2>

As etapas abaixo provisionam a implantação completa com comandos `aws`.

<Steps>
  <Step title="Crie os grupos de segurança">
    Três grupos de segurança encadeiam o caminho do tráfego: sua rede corporativa alcança o balanceador de carga na porta 443, o balanceador de carga alcança o gateway na porta 8080 e o gateway alcança o Postgres na porta 5432. Nada mais é acessível. Como você os anexa depende da trilha de computação:

    * No ECS Fargate, a etapa de implantação anexa `$ALB_SG` ao balanceador de carga e `$GW_SG` ao serviço.
    * No EKS, o AWS Load Balancer Controller cria seu próprio grupo de segurança frontend para o ALB, portanto `$ALB_SG` e `$GW_SG` não são usados: a anotação `inbound-cidrs` da etapa de implantação restringe o listener à sua rede corporativa, e o grupo de segurança do banco de dados admite o grupo de segurança do cluster em vez de `$GW_SG`.

    ```bash theme={null}
    ALB_SG="$(aws ec2 create-security-group --group-name claude-gateway-alb \
      --description "Claude gateway ALB" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"
    GW_SG="$(aws ec2 create-security-group --group-name claude-gateway-svc \
      --description "Claude gateway service" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"
    DB_SG="$(aws ec2 create-security-group --group-name claude-gateway-db \
      --description "Claude gateway Postgres" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"

    aws ec2 authorize-security-group-ingress --group-id "$ALB_SG" \
      --protocol tcp --port 443 --cidr <your-corporate-cidr>
    aws ec2 authorize-security-group-ingress --group-id "$GW_SG" \
      --protocol tcp --port 8080 --source-group "$ALB_SG"
    aws ec2 authorize-security-group-ingress --group-id "$DB_SG" \
      --protocol tcp --port 5432 --source-group "$GW_SG"
    ```
  </Step>

  <Step title="Crie as funções IAM e envie o formulário de caso de uso">
    O gateway é executado com uma função de tarefa dedicada cuja única permissão é invocar modelos Claude no Bedrock. De acordo com a [referência de upstream Bedrock](/docs/pt/claude-apps-gateway-config#amazon-bedrock), a política deve cobrir tanto os ARNs de perfil de inferência entre regiões quanto os ARNs de modelo de fundação subjacentes:

    ```bash theme={null}
    cat > bedrock-invoke.json <<EOF
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Action": ["bedrock:InvokeModel", "bedrock:InvokeModelWithResponseStream", "bedrock:CountTokens"],
        "Resource": [
          "arn:aws:bedrock:${AWS_REGION}:${ACCOUNT_ID}:inference-profile/us.anthropic.*",
          "arn:aws:bedrock:*::foundation-model/anthropic.*"
        ]
      }]
    }
    EOF
    cat > ecs-trust.json <<'EOF'
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Principal": { "Service": "ecs-tasks.amazonaws.com" },
        "Action": "sts:AssumeRole"
      }]
    }
    EOF

    aws iam create-role --role-name claude-gateway-task \
      --assume-role-policy-document file://ecs-trust.json
    aws iam put-role-policy --role-name claude-gateway-task \
      --policy-name bedrock-invoke --policy-document file://bedrock-invoke.json
    ```

    ECS também precisa de uma função de execução, que o próprio agente ECS usa para extrair a imagem do ECR e injetar os valores do Secrets Manager criados posteriormente. É separada da função de tarefa que o AWS SDK do gateway usa em tempo de execução:

    ```bash theme={null}
    aws iam create-role --role-name claude-gateway-execution \
      --assume-role-policy-document file://ecs-trust.json
    aws iam attach-role-policy --role-name claude-gateway-execution \
      --policy-arn arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy
    cat > secrets-read.json <<EOF
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Action": ["secretsmanager:GetSecretValue", "secretsmanager:DescribeSecret"],
        "Resource": [
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-jwt-secret-??????",
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-oidc-client-secret-??????",
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-postgres-url-??????"
        ]
      }]
    }
    EOF
    aws iam put-role-policy --role-name claude-gateway-execution \
      --policy-name read-gateway-secrets --policy-document file://secrets-read.json
    ```

    A política nomeia um ARN por segredo em vez de um curinga simples `gateway-*`, que em uma conta compartilhada também corresponderia a segredos não relacionados; o sufixo `-??????` à direita corresponde exatamente aos seis caracteres aleatórios que o Secrets Manager anexa ao ARN de cada segredo. Um `-*` à direita seria um glob de prefixo simples e também corresponderia a nomes mais longos como `gateway-postgres-url-prod`.

    A política IAM concede ao gateway permissão para chamar Bedrock, e Bedrock habilita acesso ao modelo por padrão em regiões comerciais. O portão de nível de conta restante é o formulário de caso de uso único da Anthropic: se ninguém em sua conta o enviou, abra o [console Amazon Bedrock](https://console.aws.amazon.com/bedrock/), selecione um modelo Anthropic no catálogo de modelos e preencha o formulário. O acesso é concedido imediatamente após o envio; consulte [Claude Code no Amazon Bedrock](/docs/pt/amazon-bedrock#1-submit-use-case-details) para o formulário AWS Organizations e as permissões IAM que o remetente precisa.

    A trilha EKS reutiliza ambos os documentos de política em uma função IRSA em vez das duas funções ECS; consulte a etapa de implantação.
  </Step>

  <Step title="Provisione Amazon RDS para PostgreSQL">
    A instância é executada nas subnets privadas sem endereço público e com criptografia de armazenamento ativada. A versão do mecanismo é fixada em Postgres 16, que satisfaz o piso suportado do gateway de PostgreSQL 14 e garante que a família do grupo de parâmetros abaixo corresponda à instância.

    Primeiro, crie o grupo de subnets que coloca o banco de dados nas subnets privadas e um grupo de parâmetros com `rds.force_ssl=1` para que o servidor rejeite conexões em texto simples. A versão do mecanismo é fixada uma vez porque a família do grupo de parâmetros deve corresponder à versão principal do mecanismo que a instância executa:

    ```bash theme={null}
    aws rds create-db-subnet-group --db-subnet-group-name claude-gateway-db \
      --db-subnet-group-description "Claude gateway" --subnet-ids $PRIVATE_SUBNETS

    PG_VERSION=16
    PG_FAMILY="postgres${PG_VERSION}"
    aws rds create-db-parameter-group --db-parameter-group-name claude-gateway-db \
      --db-parameter-group-family "$PG_FAMILY" \
      --description "Claude gateway - require TLS on every connection"
    aws rds modify-db-parameter-group --db-parameter-group-name claude-gateway-db \
      --parameters "ParameterName=rds.force_ssl,ParameterValue=1,ApplyMethod=immediate"
    ```

    Depois crie a instância com uma senha mestre gerada:

    ```bash theme={null}
    PGPASS="$(openssl rand -hex 24)"
    aws rds create-db-instance --db-instance-identifier claude-gateway-db \
      --engine postgres --engine-version "$PG_VERSION" \
      --db-instance-class db.t4g.micro \
      --allocated-storage 20 --db-name claude_gateway \
      --master-username gateway --master-user-password "$PGPASS" \
      --db-subnet-group-name claude-gateway-db \
      --db-parameter-group-name claude-gateway-db \
      --vpc-security-group-ids "$DB_SG" \
      --no-publicly-accessible --storage-encrypted
    ```

    O argumento literal `--master-user-password` é visível na tabela de processos e nos logs de auditoria/EDR enquanto o comando é executado, a mesma exposição que a nota da etapa de segredos cobre. Em um host compartilhado ou monitorado, passe a senha via `--cli-input-json` de um arquivo `0600` em vez disso, da forma que o `setup.sh` do pacote faz.

    Aguarde a instância ficar ativa, o que pode levar vários minutos, depois leia seu endpoint privado e monte a string de conexão que o gateway usará:

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full` faz o gateway verificar a cadeia do certificado do servidor RDS e o nome do host, não apenas criptografar. A âncora de confiança é o [pacote de certificados AWS RDS](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem), que a etapa de construção de imagem abaixo copia para `/etc/claude/rds-global-bundle.pem` e confia via `NODE_EXTRA_CA_CERTS`. Não anexe um parâmetro `sslrootcert=` no estilo libpq à URL: o driver do gateway lê apenas `sslmode` da string de consulta e encaminharia `sslrootcert` para o Postgres como um parâmetro de inicialização, que o servidor rejeita.

    O serviço ECS ou os pods EKS devem ser executados nesta VPC para que possam alcançar o endpoint privado da instância, e o grupo de segurança `claude-gateway-db` apenas admite o grupo de segurança do gateway.
  </Step>

  <Step title="Escreva gateway.yaml">
    O bloco `upstreams` aponta para Bedrock com `auth: {}`, portanto o gateway se autentica via a cadeia de credenciais padrão AWS da função de tarefa no ECS ou da função IRSA no EKS. Consulte a [referência de configuração](/docs/pt/claude-apps-gateway-config) para cada campo.

    Dois campos `listen` descrevem o que está na frente do gateway:

    * `public_url`: a origem `https://` externa, obrigatória para qualquer bind não-loopback; consulte a [referência `listen`](/docs/pt/claude-apps-gateway-config#listen). O gateway constrói o `redirect_uri` do IdP e seu documento de descoberta apenas a partir deste valor, nunca a partir de cabeçalhos `X-Forwarded-*`.
    * `trusted_proxies`: os intervalos de origem do front-end. O gateway honra `X-Forwarded-For` apenas quando o par TCP está nesta lista, depois percorre a cadeia passando hops confiáveis, portanto os limites de taxa de login por IP e os eventos de auditoria registram IPs de desenvolvedores em vez do balanceador de carga.

    Em ambas as trilhas o front-end é um ALB interno, seja criado diretamente ou pelo AWS Load Balancer Controller, e os nós de um ALB recebem endereços das subnets às quais está anexado, portanto defina `trusted_proxies` para os CIDRs dessas subnets. Isso confia em cada host nessas subnets como um proxy. Evite que a origem de ingresso do ALB, seu CIDR corporativo, se sobreponha a eles, e não compartilhe as subnets com cargas de trabalho não confiáveis que possam falsificar IPs de cliente via `X-Forwarded-For`.

    O atributo de preservação de porta de cliente do ALB, `routing.http.xff_client_port.enabled`, pode permanecer em qualquer configuração: com ele ativado, o ALB escreve o cliente como `203.0.113.7:54321` ou `[2001:db8::1]:54321`, e o gateway lê ambos com a porta descartada.

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [<your-alb-subnet-cidrs>]

    oidc:
      issuer: https://example.okta.com
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}           # EKS: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # O servidor de autorização da organização Okta retorna um id_token fino que omite
      # email e grupos; o gateway os preenche de /userinfo.
      userinfo_fallback: true
      # Okta emite grupos apenas quando o escopo `groups` é solicitado e o
      # filtro de reivindicação de grupos do aplicativo os permite.
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # limita latência de desprovisionamento; diminua
    # para 1 para revogação mais apertada

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # corresponda a $AWS_REGION para que os ARNs da política IAM
    # a cubram
        auth: {} # cadeia de credenciais padrão AWS:
    # função de tarefa ECS, ou IRSA no EKS
    ```

    <Note>
      Apenas o bloco `oidc` é específico do Okta. Para usar Microsoft Entra ID em vez disso, defina `issuer` para `https://login.microsoftonline.com/<tenant-id>/v2.0`, remova `userinfo_fallback` e o escopo `groups`, e observe que Entra emite IDs de Objeto de grupo em vez de nomes, portanto [`managed.policies`](/docs/pt/claude-apps-gateway-config#managed) deve corresponder aos GUIDs, ou em App Roles com `oidc.groups_claim: roles`. Consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup).
    </Note>
  </Step>

  <Step title="Armazene segredos no AWS Secrets Manager">
    Crie três segredos; a função de execução da etapa IAM já pode lê-los:

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    Observe o ARN que cada chamada imprime; a definição de tarefa ECS referencia segredos por ARN.

    <Note>
      Argumentos literais `--secret-string` são visíveis na tabela de processos e nos logs de auditoria/EDR enquanto cada comando é executado. Em um host compartilhado ou monitorado, coloque o valor em um arquivo `0600` e passe `--secret-string file://<path>` em vez disso. O `setup.sh` do pacote mantém valores de segredo fora do argv do processo da mesma forma, passando arquivos temporários `0600` para `--cli-input-json`.
    </Note>

    Ao contrário dos segredos, o próprio `gateway.yaml` não contém valores de segredo, porque cada credencial é resolvida na inicialização através da [expansão `${VAR}` ou `${file:...}`](/docs/pt/claude-apps-gateway-config#secret-expansion). Como tudo chega ao contêiner difere por trilha:

    * No ECS, a etapa seguinte copia `gateway.yaml` na imagem em `/etc/claude/gateway.yaml`, e a definição de tarefa injeta os três segredos como variáveis de ambiente via seu campo `secrets`, portanto o YAML referencia `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` e `${GATEWAY_POSTGRES_URL}`.
    * No EKS, monte `gateway.yaml` de um ConfigMap e os segredos como arquivos em `/secrets`, referenciados como `${file:/secrets/...}`. Obtenha os Kubernetes Secrets do Secrets Manager com External Secrets Operator ou o provedor AWS do driver CSI Secrets Store, ou crie-os diretamente com `kubectl`.
  </Step>

  <Step title="Construa e envie a imagem para Amazon ECR">
    Construa a imagem de acordo com os [requisitos de imagem de contêiner](/docs/pt/claude-apps-gateway-deploy#container-image), colocando o binário glibc `linux-x64` em `./claude` no contexto de construção. Escreva seu próprio Dockerfile de acordo com esses requisitos ou comece com o [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile) do pacote, que copia o `gateway.yaml` preenchido das etapas anteriores na imagem em `/etc/claude/gateway.yaml`. No ECS essa cópia incorporada é como a configuração chega ao contêiner, razão pela qual a construção vem após o arquivo ser escrito. A trilha EKS em vez disso monta `gateway.yaml` de um ConfigMap na implantação, portanto a cópia incorporada não é usada lá.

    A imagem também carrega o pacote de certificados AWS RDS como a âncora de confiança para a string de conexão `sslmode=verify-full`, portanto baixe-o no contexto de construção primeiro. AWS rotaciona o pacote (novas CAs regionais são anexadas), portanto baixe-o por construção em vez de fixar um checksum ou confirmá-lo:

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    Os requisitos de imagem de contêiner não cobrem o pacote, portanto se você escrever seu próprio Dockerfile, adicione as duas linhas que copiam e confiam nele; o `Dockerfile` do pacote já inclui ambas:

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    Crie o repositório ECR e faça login do Docker nele. Tags imutáveis significam que a tag `<version>` que a etapa de implantação fixa não pode ser posteriormente apontada silenciosamente para uma imagem diferente:

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    Construa e envie a imagem. A definição de tarefa abaixo executa `linux/amd64`, portanto a plataforma deve corresponder aqui; para Fargate em ARM64 (Graviton), construa `linux/arm64` com o binário `linux-arm64` e defina `cpuArchitecture` para `ARM64` em vez disso:

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="Implante">
    <Tabs>
      <Tab title="ECS Fargate">
        Crie o cluster e um grupo de logs para stderr do gateway, que carrega seus eventos de auditoria e logs operacionais. A retenção é uma chamada separada, e sem uma CloudWatch mantém os logs para sempre; alinhe os 90 dias com sua política de retenção de auditoria:

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        Escreva a definição de tarefa. A função de tarefa carrega a permissão Bedrock e a função de execução injeta os segredos; use os ARNs de segredo da etapa Secrets Manager:

        ```json claude-gateway-task.json theme={null}
        {
          "family": "claude-gateway",
          "networkMode": "awsvpc",
          "requiresCompatibilities": ["FARGATE"],
          "cpu": "1024",
          "memory": "2048",
          "runtimePlatform": { "cpuArchitecture": "X86_64", "operatingSystemFamily": "LINUX" },
          "executionRoleArn": "arn:aws:iam::<account-id>:role/claude-gateway-execution",
          "taskRoleArn": "arn:aws:iam::<account-id>:role/claude-gateway-task",
          "containerDefinitions": [
            {
              "name": "gateway",
              "image": "<account-id>.dkr.ecr.<region>.amazonaws.com/claude-gateway:<version>",
              "portMappings": [{ "containerPort": 8080 }],
              "secrets": [
                { "name": "GATEWAY_JWT_SECRET",   "valueFrom": "<gateway-jwt-secret ARN>" },
                { "name": "OIDC_CLIENT_SECRET",   "valueFrom": "<gateway-oidc-client-secret ARN>" },
                { "name": "GATEWAY_POSTGRES_URL", "valueFrom": "<gateway-postgres-url ARN>" }
              ],
              "logConfiguration": {
                "logDriver": "awslogs",
                "options": {
                  "awslogs-group": "/ecs/claude-gateway",
                  "awslogs-region": "<region>",
                  "awslogs-stream-prefix": "gateway"
                }
              }
            }
          ]
        }
        ```

        Registre-a:

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        Coloque um ALB interno na frente com um grupo de destino que verifica a saúde do gateway. `--ip-address-type ipv4` importa: um ALB dual-stack interno publica registros AAAA de intervalo público, que a verificação de rede privada `/login` rejeita:

        ```bash theme={null}
        ALB_ARN="$(aws elbv2 create-load-balancer --name claude-gateway \
          --scheme internal --type application --ip-address-type ipv4 \
          --subnets $PRIVATE_SUBNETS --security-groups "$ALB_SG" \
          --query 'LoadBalancers[0].LoadBalancerArn' --output text)"

        TG_ARN="$(aws elbv2 create-target-group --name claude-gateway \
          --protocol HTTP --port 8080 --vpc-id "$VPC_ID" --target-type ip \
          --health-check-path /readyz \
          --query 'TargetGroups[0].TargetGroupArn' --output text)"
        ```

        Adicione o listener HTTPS. `--ssl-policy` fixa um piso TLS moderno, pois omiti-lo volta para o padrão legado `ELBSecurityPolicy-2016-08`, que ainda aceita TLS 1.0/1.1.

        O ALB fecha uma conexão após 60 segundos sem dados por padrão. Os pings de keepalive do gateway mantêm streams dentro desse padrão, portanto aumentar o tempo limite adiciona margem acima da cadência de ping; a linha [Troubleshooting](#troubleshooting) em streams descartados cobre o mecanismo e gateways mais antigos. Os comandos abaixo adicionam o listener e aumentam o tempo limite:

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        Crie o serviço. O disjuntor de implantação reverte uma implantação cujas tarefas continuam falhando, de uma imagem ruim ou uma configuração não inicializável, para o último estado estável em vez de relançar tarefas falhando para sempre:

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        O período de graça de 60 segundos dá a uma tarefa fria tempo para extrair a imagem, conectar ao store e responder sua primeira verificação de saúde antes de ECS começar a contar falhas contra a implantação. A verificação de saúde do grupo de destino em `GET /readyz` verifica se o store é acessível, portanto uma tarefa que não consegue alcançar Postgres nunca entra em rotação; consulte [Comportamento de interrupção](/docs/pt/claude-apps-gateway-deploy#outage-behavior) para o tradeoff e a alternativa `/healthz`.

        As tarefas são executadas em subnets privadas sem IP público, portanto toda saída (para Bedrock, seu IdP, Secrets Manager, ECR e CloudWatch Logs) passa pelo gateway NAT. Para manter o tráfego Bedrock fora do caminho público, crie um endpoint VPC de interface `bedrock-runtime` e aponte o `base_url` do upstream para ele, conforme mostrado na [referência de upstream Bedrock](/docs/pt/claude-apps-gateway-config#amazon-bedrock); o IdP ainda precisa de saída de internet.

        Termine dando aos desenvolvedores um nome de host privadamente resolvível: em uma zona hospedada privada Route 53, alias o nome DNS interno do gateway para o ALB e defina `listen.public_url` para esse nome de host. O próprio nome `*.elb.amazonaws.com` do ALB resolve para endereços privados em um ALB interno, mas não pode carregar seu certificado ACM, portanto use seu próprio nome.

        Atualize o URI de redirecionamento autorizado do cliente OAuth para `<public_url>/oauth/callback` antes do primeiro login. Após alterar `public_url`, reconstrua e envie a imagem sob uma nova tag, registre uma nova revisão de definição de tarefa e reimplante. No ECS a configuração vive no `gateway.yaml` incorporado da imagem, e o gateway constrói sua origem pública apenas a partir dessa configuração, ignorando `X-Forwarded-Host` e `X-Forwarded-Proto`. `X-Forwarded-For` é honrado para IPs de cliente apenas quando `listen.trusted_proxies` é definido.
      </Tab>

      <Tab title="EKS">
        Esta trilha precisa de `kubectl` e `eksctl` instalados localmente, e um cluster EKS existente com um provedor OIDC IAM e o AWS Load Balancer Controller instalado. O cluster deve estar em `$VPC_ID` para que os pods possam alcançar o endpoint privado RDS, e o grupo de segurança `claude-gateway-db` deve admitir o grupo de segurança do pod ou nó do cluster no lugar de `$GW_SG`.

        No EKS o gateway obtém suas credenciais Bedrock através de IRSA em vez das funções ECS. A política de confiança `ecs-tasks.amazonaws.com` da etapa IAM não se aplica aqui; IRSA precisa de uma função cuja política de confiança federe no provedor OIDC do cluster, escopo para `system:serviceaccount:claude-gateway:gateway`. `eksctl create iamserviceaccount` cria essa função, anexa as políticas e anota a conta de serviço Kubernetes com o ARN da função em uma etapa. Transforme os dois documentos de política da etapa IAM em políticas gerenciadas que ele pode anexar:

        ```bash theme={null}
        BEDROCK_POLICY_ARN="$(aws iam create-policy --policy-name claude-gateway-bedrock-invoke \
          --policy-document file://bedrock-invoke.json --query Policy.Arn --output text)"
        SECRETS_POLICY_ARN="$(aws iam create-policy --policy-name claude-gateway-secrets-read \
          --policy-document file://secrets-read.json --query Policy.Arn --output text)"

        kubectl create namespace claude-gateway
        eksctl create iamserviceaccount --cluster <your-cluster> --region "$AWS_REGION" \
          --namespace claude-gateway --name gateway --role-name claude-gateway \
          --attach-policy-arn "$BEDROCK_POLICY_ARN" \
          --attach-policy-arn "$SECRETS_POLICY_ARN" \
          --approve
        ```

        A política de segredos é necessária apenas quando os pods leem o Secrets Manager eles mesmos, como o provedor AWS do driver CSI Secrets Store faz usando a conta de serviço do pod de montagem; remova-a se você criar os Kubernetes Secrets de outra forma. O provedor precisa de ambas as ações da política: ele chama `DescribeSecret` quando reconcilia segredos rotacionados, portanto uma concessão somente `GetSecretValue` monta na primeira implantação mas para de pegar rotações.

        Implante o gateway como um Deployment padrão mais um Service e um Ingress, conforme descrito em [Implantação Kubernetes](/docs/pt/claude-apps-gateway-deploy#kubernetes), com:

        * `serviceAccountName: gateway`
        * `gateway.yaml` montado de um ConfigMap e os segredos montados em `/secrets`
        * a sonda de prontidão apontada para `GET /readyz`

        Para o front-end, um Ingress gerenciado pelo AWS Load Balancer Controller provisiona o ALB interno. Anote-o com:

        * `alb.ingress.kubernetes.io/scheme: internal` e `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`, para que nenhum registro AAAA de intervalo público seja publicado para a verificação de rede privada `/login` [rejeitar](/docs/pt/claude-apps-gateway#prerequisites)
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`, para que o grupo de segurança frontend gerenciado pelo controlador admita apenas sua rede corporativa no lugar de seu padrão `0.0.0.0/0`
        * `alb.ingress.kubernetes.io/certificate-arn` com o certificado ACM
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`, para que o listener não volte para a política padrão legada que aceita TLS 1.0 e 1.1
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`, uma margem acima do keepalive de streaming do gateway; consulte [Troubleshooting](#troubleshooting)

        Com IRSA, o AWS SDK lê um token de conta de serviço projetado e o troca com AWS STS, portanto o pod nunca precisa do serviço de metadados da instância EC2; uma NetworkPolicy de saída pode bloquear `169.254.169.254` para pods do gateway. O problema de limite de hop do nó em [Troubleshooting](#troubleshooting) abaixo se aplica apenas a clusters que pulam IRSA e dependem de funções de instância de nó.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Envie a URL do gateway para máquinas de desenvolvedores">
    O gateway agora está em execução, mas os desenvolvedores não conseguem alcançá-lo de `/login` até que a URL do gateway esteja em suas máquinas. Defina `forceLoginMethod` e `forceLoginGatewayUrl` no [arquivo de configurações gerenciadas](/docs/pt/claude-apps-gateway#set-the-gateway-url) que você implanta em cada dispositivo via MDM. Não há opção de gateway no seletor de login para um desenvolvedor selecionar manualmente.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Referência Terraform
</h2>

O pacote complementar em [`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws) empacota esta página como código:

* **`setup.sh`** roteiriza o passo a passo de provisionamento acima com os mesmos comandos `aws`, na trilha ECS Fargate. É idempotente: recursos existentes são detectados e pulados, portanto re-executá-lo é seguro, e qualquer padrão pode ser substituído via variável de ambiente. Você ainda cria o segredo do cliente OIDC Okta e o certificado ACM você mesmo: uma execução sem eles pula a implantação ECS/ALB, nomeia as entradas ausentes e imprime o comando `create-secret`; crie ambos e re-execute. O formulário de caso de uso Bedrock e o alias Route 53 imprimem como próximas etapas em vez de executar automaticamente, e o push MDM do cliente permanece uma etapa manual desta página.
* **`gateway.yaml.example`** é o modelo de configuração da etapa gateway.yaml, com as chaves opcionais incluídas comentadas. Copie-o para `gateway.yaml` e substitua cada `REPLACE_ME` antes de construir.
* **`Dockerfile`** constrói a imagem de tempo de execução a partir do binário pré-construído `linux-x64` e copia seu `gateway.yaml` preenchido em `/etc/claude/gateway.yaml`, mais o pacote de certificados AWS RDS que ancora o `sslmode=verify-full` do store. `setup.sh` baixa o pacote apenas quando ele não está já no contexto de construção; delete o arquivo e reconstrua sob uma nova tag para pegar uma rotação de CA AWS. O arquivo de configuração não contém valores de segredo, pois cada credencial é resolvida na inicialização através da expansão `${VAR}`. Uma edição de configuração portanto significa uma reconstrução sob uma nova tag; `setup.sh` automatiza isso marcando imagens com um hash do arquivo.
* **`terraform/`** provisiona o mesmo escopo ECS Fargate declarativamente: os grupos de segurança, funções IAM, repositório ECR, instância RDS, segredos Secrets Manager e o serviço ECS atrás do ALB interno. A VPC e subnets privadas permanecem pré-requisitos, passados como variáveis. Terraform cria o repositório ECR mas não constrói a imagem, e a definição de serviço referencia a imagem, portanto o apply é dois passes: um apply direcionado para o repositório, depois a construção e envio, depois o apply completo. O `terraform/README.md` do pacote cobre as variáveis, estado remoto e desmontagem.

Como esta página, o pacote é um exemplo funcional para infraestrutura gerenciada pelo cliente em vez de uma implantação de produção suportada; revise e adapte-o ao seu próprio ambiente antes de confiar nele.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Para erros de boot e login do gateway, consulte a tabela de [troubleshooting](/docs/pt/claude-apps-gateway-deploy#troubleshooting) independente de plataforma. As entradas abaixo são específicas da AWS.

| Sintoma                                                                                                                                    | Causa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Correção                                                                                                                                                                                                                                                                                                             |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | O nome do gateway resolve para pelo menos um endereço público. Um ALB dual-stack interno publica registros AAAA de intervalo público, e a [verificação de rede privada](/docs/pt/claude-apps-gateway#prerequisites) requer que cada endereço resolvido seja privado                                                                                                                                                                                                                                                                                                                              | Crie o ALB com `--ip-address-type ipv4`, ou sirva um nome DNS separado apenas interno sem registro AAAA público                                                                                                                                                                                                      |
| Cada solicitação Bedrock retorna 502; log mostra `Could not load credentials from any providers`                                           | A tarefa é executada no tipo de lançamento ECS EC2 sem uma função de tarefa, ou o pod é executado em um nó EKS sem IRSA, portanto as credenciais vêm de metadados de instância, que o limite de hop padrão IMDSv2 de 1 para dentro de um contêiner. Nenhuma trilha nesta página é afetada: funções de tarefa Fargate e IRSA não usam metadados de instância                                                                                                                                                                                                                                 | Prefira funções de tarefa e IRSA. Onde credenciais de instância são inevitáveis, aumente o limite de hop com `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`; a [tabela independente de plataforma](/docs/pt/claude-apps-gateway-deploy#troubleshooting) cobre os tradeoffs |
| Solicitações Bedrock retornam `403 AccessDeniedException`                                                                                  | A conta não enviou o formulário de caso de uso único da Anthropic, a assinatura automática do AWS Marketplace que começa na primeira invocação da conta não terminou ainda, ou a política da função de tarefa está faltando os ARNs de perfil de inferência ou modelo de fundação                                                                                                                                                                                                                                                                                                           | Envie o formulário de caso de uso do catálogo de modelos do console Bedrock; se foi apenas enviado ou esta é a primeira invocação da conta, tente novamente após alguns minutos. Conceda `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` em ambas as famílias de ARN.                                |
| Bedrock retorna uma `ValidationException` dizendo que throughput sob demanda não é suportado                                               | Uma entrada `models:` personalizada mapeia para um ID de modelo de fundação simples que a região serve apenas através de perfis de inferência                                                                                                                                                                                                                                                                                                                                                                                                                                               | Mapeie o modelo para seu ID de perfil de inferência entre regiões (`us.anthropic.*`) em vez disso; o catálogo integrado já faz isso                                                                                                                                                                                  |
| Tarefa ECS para com `ResourceInitializationError` antes do gateway registrar qualquer coisa                                                | A função de execução não consegue ler os segredos do Secrets Manager, ou as subnets privadas não têm caminho para Secrets Manager ou ECR                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Conceda `secretsmanager:GetSecretValue` nos ARNs dos três segredos `gateway-` para a função de execução, e forneça saída via gateway NAT, ou, sem um, endpoints de interface para Secrets Manager, ECR e CloudWatch Logs, que o driver `awslogs` precisa no mesmo estágio, mais um endpoint de gateway S3            |
| Boot do gateway sai com erro de tempo limite de conexão Postgres                                                                           | O grupo de segurança do banco de dados não admite o grupo de segurança do gateway na porta 5432, ou o serviço é executado fora da VPC do banco de dados                                                                                                                                                                                                                                                                                                                                                                                                                                     | Permita 5432 do grupo de segurança do gateway no do banco de dados, e execute o serviço na mesma VPC que o grupo de subnets do DB                                                                                                                                                                                    |
| Boot do gateway sai com erro de verificação de certificado TLS Postgres                                                                    | A string de conexão define `sslmode=verify-full` mas a imagem não confia no pacote de CA RDS: o pacote não foi copiado na imagem, ou `NODE_EXTRA_CA_CERTS` não aponta para ele                                                                                                                                                                                                                                                                                                                                                                                                              | Adicione as duas linhas do Dockerfile da etapa de construção que copiam o pacote e definem `NODE_EXTRA_CA_CERTS`, depois reconstrua, envie sob uma nova tag e reimplante                                                                                                                                             |
| Respostas de streaming caem no meio do stream após um período silencioso                                                                   | Um gateway mais antigo que v2.1.229 em um Bedrock ou Claude Platform na upstream AWS envia nada enquanto a upstream está silenciosa, por exemplo durante pensamento estendido sem saída transmitida. O ALB fecha uma conexão após 60 segundos sem dados por padrão, então corta o stream nessa lacuna. Gateways v2.1.229 e posteriores mantêm um stream silencioso sob esse tempo limite: nessas upstreams o gateway emite um evento SSE `ping` uma vez após cerca de 15 segundos passarem sem dados de stream, e em uma upstream de API Anthropic ele retransmite os próprios pings da API | Atualize o gateway para v2.1.229 ou posterior, ou defina o atributo `idle_timeout.timeout_seconds` para `3600`, via `modify-load-balancer-attributes` ou a anotação `load-balancer-attributes` do Ingress no EKS                                                                                                     |

<h2 id="telemetry">
  Telemetria
</h2>

O gateway oferece métricas de uso por desenvolvedor sem qualquer configuração OTEL por máquina. Claude Code emite métricas, logs e traces OpenTelemetry (OTLP) opcionais; [Monitorar uso](/docs/pt/monitoring-usage) cobre tudo que o CLI relata. Em sessões de gateway o CLI carimba cada exportação com os atributos de identidade IdP autenticados `user.id`, `user.email` e `user.groups`, portanto o uso se acumula por desenvolvedor sem encanamento `OTEL_RESOURCE_ATTRIBUTES`.

O gateway em si é um relé OTLP autenticado. Defina [`telemetry.forward_to`](/docs/pt/claude-apps-gateway-config#telemetry) junto com `listen.public_url`, e ele empurra as configurações do exportador OTEL para cada cliente conectado e encaminha seu tráfego OTLP verbatim para cada destino que você lista. Cada destino opta por métricas, logs e traces independentemente, e o padrão é apenas métricas; consulte a [referência `telemetry`](/docs/pt/claude-apps-gateway-config#telemetry) para os campos por sinal e seus tradeoffs de sensibilidade. O gateway não armazena em buffer, agrega ou armazena telemetria, portanto onde os dados chegam é inteiramente a configuração do exportador do coletor.

A telemetria do cliente está desativada por padrão; configurar `telemetry.forward_to` é o que a ativa para desenvolvedores conectados, e cada cliente interativo mostra um diálogo de aprovação de segurança para as configurações empurradas, conforme descrito na [referência de configuração](/docs/pt/claude-apps-gateway-config#telemetry). Na AWS, cada sinal mapeia para um destino da seguinte forma.

<h3 id="client-metrics-logs-and-traces">
  Métricas, logs e traces do cliente
</h3>

Aponte `telemetry.forward_to` para um coletor OpenTelemetry, como o [coletor AWS Distro for OpenTelemetry (ADOT)](https://aws-otel.github.io/), e exporte de lá para Amazon CloudWatch, Amazon Managed Service for Prometheus ou qualquer backend OTLP.

Execute o coletor como seu próprio serviço interno acessível via `https://`; a [referência `telemetry`](/docs/pt/claude-apps-gateway-config#telemetry) cobre a exceção de loopback e `CLAUDE_GATEWAY_ALLOW_LOOPBACK`.

<h3 id="gateway-logs">
  Logs do gateway
</h3>

No ECS Fargate, sem configuração extra: o driver `awslogs` entrega stderr do gateway, que carrega seus eventos de auditoria e logs operacionais, para o grupo de logs `/ecs/claude-gateway` criado acima. No EKS, logs de pod não chegam ao CloudWatch por padrão, portanto a trilha de auditoria é perdida até você instalar coleta de logs: o complemento Amazon CloudWatch Observability com captura de log de contêiner ativada, ou um DaemonSet Fluent Bit. Em qualquer trilha, consulte os logs com CloudWatch Logs Insights e dirija alarmes de filtros de métrica.

<h3 id="container-metrics">
  Métricas de contêiner
</h3>

Ative Container Insights no cluster com `aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled` para CPU, memória e rede por tarefa. No EKS, instale o complemento Amazon CloudWatch Observability.

<h3 id="spend">
  Gasto
</h3>

A telemetria mostra uso após o fato; [limites de gasto](/docs/pt/claude-apps-gateway-spend-limits) são a visão ao vivo do gateway por desenvolvedor e aplicação sobre a credencial upstream compartilhada.

<h2 id="next-steps">
  Próximas etapas
</h2>

* [Referência de configuração](/docs/pt/claude-apps-gateway-config): cada opção `gateway.yaml`, incluindo `managed.policies` e `telemetry`
* [Implantação e operações](/docs/pt/claude-apps-gateway-deploy): configuração de IdP, verificações de saúde, rotação de segredo JWT, upgrades e o modelo de segurança
* [Visão geral do gateway de aplicativos Claude](/docs/pt/claude-apps-gateway): quickstart e conexão de desenvolvedores
* [Amostras AWS para gateway de aplicativos Claude](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway): amostras de implantação mantidas pela AWS cobrindo uma variedade de ambientes de clientes
