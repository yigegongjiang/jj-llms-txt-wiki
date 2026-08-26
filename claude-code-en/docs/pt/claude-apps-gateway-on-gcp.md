> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implantar gateway de aplicativos Claude no Google Cloud

> Um exemplo prático de execução do gateway de aplicativos Claude no Google Cloud: Cloud Run ou GKE, Cloud SQL para PostgreSQL, Secret Manager e autenticação de conta de serviço para Agent Platform do Google Cloud.

<Note>
  Esta página apresenta uma forma de executar o gateway de aplicativos Claude no Google Cloud. A configuração é um exemplo funcional para infraestrutura gerenciada pelo cliente em vez de uma implantação de produção suportada; use-a para ver como as peças se encaixam antes de adaptá-la ao seu próprio ambiente. Para os requisitos independentes de plataforma, consulte o [guia de implantação](/docs/pt/claude-apps-gateway-deploy).
</Note>

Este exemplo provisiona o gateway de aplicativos Claude no Google Cloud com o Agent Platform do Google Cloud como upstream de modelo, usando Cloud Run ou GKE para computação. Google Workspace é o exemplo de provedor de identidade (IdP), mas qualquer IdP compatível com OpenID Connect (OIDC) funciona; apenas o bloco `oidc` muda. Consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup) para detalhes específicos por IdP.

<h2 id="what-you’ll-build">
  O que você construirá
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Diagrama do gateway de aplicativos Claude no Google Cloud: clientes Claude Code se conectam via HTTPS ao gateway (Cloud Run ou GKE), que é executado dentro de um VPC ao lado de um banco de dados Cloud SQL com IP privado para estado de sessão. O gateway faz login dos usuários via OIDC contra Google Workspace, lê configuração e segredos do Secret Manager, encaminha solicitações de modelo para Agent Platform e extrai sua imagem do Artifact Registry na implantação." width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

A configuração de referência provisiona:

* Serviço **Cloud Run** ou **GKE** Deployment executando o contêiner do gateway
* Repositório **Artifact Registry** para a imagem do gateway
* Instância **Cloud SQL para PostgreSQL**, apenas IP privado, para o [store](/docs/pt/claude-apps-gateway-config#store) do gateway
* Segredos **Secret Manager** para `gateway.yaml`, a chave de assinatura JWT, o segredo do cliente OIDC e a URL do Postgres
* **Conta de serviço** com `roles/aiplatform.user`, anexada diretamente no Cloud Run ou vinculada via Workload Identity no GKE
* **Internal Application Load Balancer** no Cloud Run, ou um **GKE Ingress** interno de classe `gce-internal` no GKE, para HTTPS

<h2 id="prerequisites">
  Pré-requisitos
</h2>

* Um projeto GCP com faturamento habilitado e permissão para criar os recursos acima
* A CLI `gcloud`, autenticada com `gcloud auth login`, e Docker instalado localmente
* Para o caminho GKE: `kubectl` e um cluster GKE no VPC criado no passo a passo abaixo
* Acesso aos modelos Claude que você precisa no Model Garden, em uma região que os publica
* Um cliente de aplicação web OAuth 2.0 do Google Workspace com URI de redirecionamento `https://<gateway-host>/oauth/callback`; consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup)
* Um nome de host TLS para o gateway, normalmente um nome DNS interno apontando para o balanceador de carga

Defina o projeto e a região uma vez:

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  Implantar o gateway
</h2>

Os passos abaixo provisionam a implantação completa com comandos `gcloud`.

<Steps>
  <Step title="Habilitar APIs">
    Habilite as APIs de serviço que o passo a passo usa:

    ```bash theme={null}
    gcloud services enable \
      aiplatform.googleapis.com \
      artifactregistry.googleapis.com \
      sqladmin.googleapis.com \
      secretmanager.googleapis.com \
      iamcredentials.googleapis.com \
      iam.googleapis.com \
      compute.googleapis.com \
      servicenetworking.googleapis.com \
      run.googleapis.com \
      container.googleapis.com
    ```

    As APIs que você precisa dependem do caminho de implantação:

    * `compute` e `servicenetworking`: necessárias para o caminho Cloud SQL com IP privado
    * `run`: apenas Cloud Run
    * `container`: apenas GKE
  </Step>

  <Step title="Criar a conta de serviço e conceder IAM">
    O gateway é executado como uma conta de serviço dedicada com permissão para chamar Agent Platform. Ele alcança Cloud SQL sobre o VPC com um usuário de senha, portanto nenhuma função IAM do Cloud SQL é necessária:

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    Em seguida, habilite os modelos Claude para o projeto no Model Garden; os modelos são publicados em regiões específicas, portanto verifique cada cartão de modelo.
  </Step>

  <Step title="Construir e enviar a imagem para Artifact Registry">
    Construa a imagem de acordo com os [requisitos de imagem de contêiner](/docs/pt/claude-apps-gateway-deploy#container-image), usando o binário glibc `linux-x64`, e envie-a:

    ```bash theme={null}
    gcloud artifacts repositories create claude-gateway \
      --repository-format=docker --location="$REGION"
    gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet

    # Cloud Run requires linux/amd64. --provenance=false avoids a buildx OCI
    # image index that Cloud Run rejects.
    docker build --platform=linux/amd64 --provenance=false \
      -t "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" .
    docker push "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>"
    ```
  </Step>

  <Step title="Provisionar Cloud SQL para PostgreSQL">
    Crie a instância em um VPC via Private Services Access para que ela não tenha IP público; isso também satisfaz projetos onde `constraints/sql.restrictPublicIp` é aplicado:

    ```bash theme={null}
    VPC=cc-gateway-vpc
    gcloud compute networks create "$VPC" --subnet-mode=custom
    gcloud compute networks subnets create cc-gateway-subnet \
      --network="$VPC" --region="$REGION" --range=10.0.0.0/24

    # Private Services Access: one-time per VPC
    gcloud compute addresses create "google-managed-services-${VPC}" \
      --global --purpose=VPC_PEERING --prefix-length=16 --network="$VPC"
    gcloud services vpc-peerings connect \
      --service=servicenetworking.googleapis.com \
      --ranges="google-managed-services-${VPC}" --network="$VPC"

    gcloud sql instances create claude-gateway-db \
      --database-version=POSTGRES_16 --tier=db-g1-small --region="$REGION" \
      --network="projects/${PROJECT_ID}/global/networks/${VPC}" --no-assign-ip
    gcloud sql databases create claude_gateway --instance=claude-gateway-db
    PGPASS="$(openssl rand -hex 24)"
    gcloud sql users create gateway --instance=claude-gateway-db --password="$PGPASS"

    PRIVATE_IP="$(gcloud sql instances describe claude-gateway-db \
      --format='value(ipAddresses[0].ipAddress)')"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${PRIVATE_IP}:5432/claude_gateway?sslmode=require"
    ```

    O runtime Cloud Run ou GKE deve estar neste VPC ou roteado para ele.
  </Step>

  <Step title="Escrever gateway.yaml">
    O bloco `upstreams` aponta para Agent Platform com `auth: {}`, portanto o gateway autentica via Application Default Credentials da conta de serviço do runtime. Consulte a [referência de configuração](/docs/pt/claude-apps-gateway-config) para cada campo.

    Dois campos `listen` dependem do que está na frente do gateway:

    * `public_url`: necessário atrás de Cloud Run ou um GKE Ingress. O gateway constrói o `redirect_uri` do IdP e seu documento de descoberta apenas a partir deste valor, nunca a partir de cabeçalhos `X-Forwarded-*`.
    * `trusted_proxies`: os intervalos de origem do front-end. O gateway honra `X-Forwarded-For` apenas quando o par TCP está nesta lista, depois percorre a cadeia passando hops confiáveis, portanto os limites de taxa de login por IP e eventos de auditoria registram IPs de desenvolvedores em vez do balanceador de carga.

    Defina `trusted_proxies` para corresponder ao seu front-end. Um GKE Ingress externo de classe `gce` não está listado: ele provisiona um endereço de regra de encaminhamento público, que a verificação [rede privada](/docs/pt/claude-apps-gateway#prerequisites) do `/login` rejeita.

    | Front-end                                                 | `trusted_proxies`                                        |
    | --------------------------------------------------------- | -------------------------------------------------------- |
    | Cloud Run alcançado diretamente, sem balanceador de carga | `[169.254.0.0/16]`                                       |
    | Internal Application Load Balancer na frente do Cloud Run | `169.254.0.0/16` mais CIDR da sua sub-rede somente proxy |
    | GKE internal Ingress, classe `gce-internal`               | CIDR da sua sub-rede somente proxy                       |

    O exemplo abaixo usa os valores do balanceador de carga interno na frente do Cloud Run.

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [169.254.0.0/16, <your-proxy-only-subnet-cidr>]

    oidc:
      issuer: https://accounts.google.com
      client_id: <your-oauth-client-id>
      client_secret: ${OIDC_CLIENT_SECRET}           # GKE: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # Google ignores offline_access; these yield refresh tokens:
      scopes: [openid, profile, email]
      extra_auth_params: { access_type: offline, prompt: consent }

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # GKE: ${file:/secrets/jwt-secret}

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # GKE: ${file:/secrets/postgres-url}

    upstreams:
      - provider: vertex
        region: <your-region>                        # must match $REGION
        project_id: <your-project>
        auth: {} # ADC via the runtime service account
    ```

    <Note>
      Os id\_tokens do Google não carregam nenhuma reivindicação `groups`. Para usar políticas baseadas em grupos em [`managed.policies`](/docs/pt/claude-apps-gateway-config#managed) com Google Workspace como IdP, configure [`oidc.google_groups`](/docs/pt/claude-apps-gateway-config#oidc), que procura os grupos de cada usuário através da API Admin SDK Directory usando uma conta de serviço com delegação em todo o domínio. Sem isso, corresponda em `email_domain` em vez disso.
    </Note>
  </Step>

  <Step title="Armazenar segredos no Secret Manager">
    Crie quatro segredos e conceda `roles/secretmanager.secretAccessor` à conta de serviço `claude-gateway`:

    | Segredo                      | Fonte                                       |
    | ---------------------------- | ------------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`                   |
    | `gateway-oidc-client-secret` | Google Cloud Console → cliente OAuth        |
    | `gateway-postgres-url`       | `$GATEWAY_POSTGRES_URL` do passo Cloud SQL  |
    | `gateway-config`             | o `gateway.yaml` completo do passo anterior |

    Como os segredos chegam ao contêiner difere por caminho:

    * No GKE eles são montados como arquivos via o driver CSI do Secret Manager, e `gateway.yaml` referencia `${file:/secrets/...}`.
    * No Cloud Run, que não pode montar múltiplos segredos em um diretório, `gateway.yaml` é montado como arquivo e os outros três são injetados como variáveis de ambiente, portanto `gateway.yaml` referencia `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` e `${GATEWAY_POSTGRES_URL}` em vez disso.
  </Step>

  <Step title="Implantar">
    <Tabs>
      <Tab title="Cloud Run">
        O comando abaixo implanta para produção atrás de um balanceador de carga interno.

        ```bash theme={null}
        gcloud run deploy claude-gateway \
          --image="${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" \
          --region="$REGION" \
          --service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --min-instances=1 \
          --timeout=3600 \
          --ingress=internal-and-cloud-load-balancing \
          --network="$VPC" --subnet=cc-gateway-subnet --vpc-egress=private-ranges-only \
          --set-secrets=/etc/claude/gateway.yaml=gateway-config:latest,GATEWAY_JWT_SECRET=gateway-jwt-secret:latest,OIDC_CLIENT_SECRET=gateway-oidc-client-secret:latest,GATEWAY_POSTGRES_URL=gateway-postgres-url:latest \
          --no-invoker-iam-check
        ```

        Egresso VPC direto, via `--network`, `--subnet` e `--vpc-egress=private-ranges-only`, permite que o serviço alcance o IP privado do Cloud SQL diretamente. Egresso público para os endpoints do Agent Platform e `accounts.google.com` vai diretamente para a internet em vez de através do VPC, portanto nenhum Cloud NAT é necessário.

        A verificação de IAM do invoker deve estar aberta ou desabilitada. O gateway executa seu próprio OIDC e seus clientes não carregam nenhum token GCP, portanto a verificação de invoker do Cloud Run tem que admitir solicitações não autenticadas. A autenticação OIDC do gateway autentica a solicitação uma vez que ela alcança o contêiner, com `allowed_email_domains` controlando quais domínios podem fazer login.

        Dois sinalizadores admitem solicitações não autenticadas:

        * `--no-invoker-iam-check`: desabilita a verificação sem nenhuma vinculação `allUsers` para gerenciar, e funciona sob Domain Restricted Sharing
        * `--allow-unauthenticated`: concede ao `allUsers` a função `run.invoker`; use-o se sua organização não permitir `--no-invoker-iam-check`

        Restrição de ingresso via `--ingress` é uma camada separada e independente da verificação de invoker; mantenha-a definida para limitar o serviço à sua rede corporativa.

        Por padrão, a URL `*.run.app` do Cloud Run resolve para um endereço público, que a verificação [rede privada](/docs/pt/claude-apps-gateway#prerequisites) do `/login` rejeita. Duas topologias fornecem aos desenvolvedores um nome de host resolvível privadamente, e o Cloud Run não provisiona nenhuma para você:

        * **Internal Application Load Balancer**, a topologia que o comando de implantação acima assume: implante com `--ingress=internal-and-cloud-load-balancing`, provisione um Internal Application Load Balancer na frente do serviço com um nome DNS interno e certificado, e defina `listen.public_url` para esse nome de host.
        * **Ingresso somente interno sem balanceador de carga**: implante com `--ingress=internal` e deixe `listen.public_url` como a URL `*.run.app`, o padrão nos [ativos de referência](#terraform-reference) abaixo. Para `*.run.app` resolver privadamente, sua equipe de rede deve já operar um endpoint Private Service Connect para APIs do Google, uma zona privada Cloud DNS resolvendo `*.run.app` para ele, e roteamento no local para esse endpoint.

        O [guia de rede privada do Google para Cloud Run](https://cloud.google.com/run/docs/securing/private-networking) cobre a infraestrutura que ambas as opções precisam. Verifique o login uma vez que o gateway esteja servindo em um nome de host privado; até então, confirme que o contêiner inicializou a partir de seus logs no Cloud Run.

        Atualize o URI de redirecionamento autorizado do cliente OAuth para `<public_url>/oauth/callback` antes do primeiro login. Reimplante após alterar `public_url`, porque o gateway constrói sua origem pública apenas a partir dessa configuração e ignora `X-Forwarded-Host` e `X-Forwarded-Proto`. `X-Forwarded-For` é honrado para IPs de cliente apenas quando `listen.trusted_proxies` está definido.
      </Tab>

      <Tab title="GKE">
        O cluster deve estar no `$VPC` criado no passo Cloud SQL para que os pods possam alcançar o IP privado do banco de dados; peering de VPC sozinho não funciona, porque o IP privado do Cloud SQL é em si uma rede peered e peering é não-transitivo. Para criar um novo cluster nesse VPC, passe `--network="$VPC" --subnetwork=cc-gateway-subnet` para `gcloud container clusters create`.

        Habilite Workload Identity no cluster e seus node pools, depois vincule a conta de serviço do Google à conta de serviço do Kubernetes para que os pods herdem suas credenciais:

        ```bash theme={null}
        gcloud container clusters update <cluster> --region="$REGION" \
          --workload-pool="${PROJECT_ID}.svc.id.goog"
        # On a Standard cluster, existing node pools also need GKE_METADATA;
        # Autopilot enables this by default.
        gcloud container node-pools update <pool> --cluster=<cluster> \
          --region="$REGION" --workload-metadata=GKE_METADATA

        kubectl create namespace claude-gateway
        kubectl create serviceaccount gateway -n claude-gateway

        gcloud iam service-accounts add-iam-policy-binding \
          "claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --role roles/iam.workloadIdentityUser \
          --member "serviceAccount:${PROJECT_ID}.svc.id.goog[claude-gateway/gateway]"

        kubectl annotate serviceaccount gateway -n claude-gateway \
          iam.gke.io/gcp-service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"
        ```

        Implante o gateway como um Deployment padrão mais um Service e um Ingress interno, classe `gce-internal`, conforme descrito em [Implantação Kubernetes](/docs/pt/claude-apps-gateway-deploy#kubernetes), com:

        * `serviceAccountName: gateway`
        * o driver CSI do Secret Manager montando segredos em `/secrets`
        * a sonda de prontidão apontada para `GET /readyz`

        Anexe um BackendConfig com um `timeoutSec` elevado ao Service do gateway: o serviço backend do balanceador de carga atrás do GKE Ingress padrão para um tempo limite de 30 segundos, que corta respostas de streaming longas.

        Não aplique uma NetworkPolicy de egresso que bloqueie `169.254.169.254` em um cluster Workload Identity; o pod deve alcançar o servidor de metadados para credenciais. A [proteção SSRF](/docs/pt/claude-apps-gateway-deploy#threat-model-summary) integrada do gateway é a defesa lá.

        O gateway registra um aviso de inicialização que o endpoint de metadados é alcançável e sugere aplicar uma NetworkPolicy de egresso. Sob Workload Identity esse aviso é esperado, porque o pod precisa do endpoint.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Enviar a URL do gateway para máquinas de desenvolvedores">
    O gateway agora está em execução, mas os desenvolvedores não podem alcançá-lo a partir de `/login` até que a URL do gateway esteja em suas máquinas. Defina `forceLoginMethod` e `forceLoginGatewayUrl` no [arquivo de configurações gerenciadas](/docs/pt/claude-apps-gateway#set-the-gateway-url) que você implanta em cada dispositivo via MDM. Não há opção de gateway no seletor de login para um desenvolvedor selecionar manualmente.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Referência Terraform
</h2>

Os [ativos de implantação de referência](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp) automatizam o caminho Cloud Run nesta página; os ativos de configuração e imagem se aplicam a ambos os caminhos:

* `setup.sh`: um provisionador `gcloud` idempotente que percorre o caminho completo do Cloud Run, desde a habilitação de APIs até a primeira implantação
* `terraform/`: a mesma implantação como infraestrutura como código, para uma implantação greenfield: uma aplicação direcionada para criar o repositório Artifact Registry, depois construir e enviar a imagem, depois uma aplicação completa
* `gateway.yaml.example` e um `Dockerfile` para a imagem de runtime distroless

Os artefatos padrão do ingresso Cloud Run para `internal`, portanto nenhum balanceador de carga é necessário. Para corresponder à implantação de produção atrás de um ALB desta página, execute `setup.sh` com `INGRESS=internal-and-cloud-load-balancing`, ou defina a variável Terraform `ingress` para `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER`. Os artefatos também padrão da camada de invoker para uma concessão `allUsers` `run.invoker` em vez de `--no-invoker-iam-check`, o inverso do passo a passo desta página; ambos funcionam, e a escolha depende das restrições de política da sua organização.

Os ativos são fornecidos como exemplos funcionais, não como um artefato de produção suportado; revise e adapte-os ao seu ambiente.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Para erros de inicialização e login do gateway, consulte a [tabela de troubleshooting](/docs/pt/claude-apps-gateway-deploy#troubleshooting) independente de plataforma. As entradas abaixo são específicas do Google Cloud.

| Sintoma                                                                                     | Causa                                                                                                                                                          | Correção                                                                                                                                                                                                                       |
| ------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Cloud Run retorna `403 Forbidden` antes de alcançar o contêiner                             | A verificação de IAM do invoker ainda está habilitada                                                                                                          | Implante com `--no-invoker-iam-check`, ou conceda ao `allUsers` a função `run.invoker` com `--allow-unauthenticated`                                                                                                           |
| `--no-invoker-iam-check` rejeitado com `invoker_iam_disabled is not currently available`    | Bloqueado por `constraints/run.managed.requireInvokerIam`                                                                                                      | Use `--allow-unauthenticated`. Se Domain Restricted Sharing via `constraints/iam.allowedPolicyMemberDomains` também bloquear isso, use o caminho GKE, que expõe o gateway na camada de rede sem nenhuma vinculação `allUsers`. |
| `Container manifest type … must support amd64/linux` na implantação                         | A imagem foi construída em um host não-amd64, ou buildx emitiu um índice de imagem OCI                                                                         | Construa com `--platform=linux/amd64 --provenance=false`                                                                                                                                                                       |
| A inicialização do gateway sai com um erro de tempo limite de conexão Postgres no Cloud Run | O serviço não está anexado ao VPC, ou Cloud SQL não tem IP privado nesse VPC; o store para de esperar após 5 segundos                                          | Implante com `--network` e `--subnet` para egresso VPC direto, e crie a instância Cloud SQL com `--no-assign-ip` e `--network` apontando para o mesmo VPC                                                                      |
| Solicitações do Agent Platform retornam `403 PERMISSION_DENIED`                             | O runtime não está usando a conta de serviço `claude-gateway`, ou o modelo não está habilitado no Model Garden para o projeto                                  | Defina `--service-account` no Cloud Run ou vincule Workload Identity no GKE, e habilite cada modelo Claude no Model Garden para a região de destino                                                                            |
| Respostas de streaming são cortadas após uma duração fixa                                   | Tempo limite de solicitação do front-end: o serviço backend do balanceador de carga atrás do GKE Ingress padrão para 30 segundos e Cloud Run para 300 segundos | Anexe um BackendConfig com um `timeoutSec` elevado no GKE, ou implante com `--timeout=3600` no Cloud Run                                                                                                                       |

<h2 id="next-steps">
  Próximos passos
</h2>

* [Referência de configuração](/docs/pt/claude-apps-gateway-config): cada opção `gateway.yaml`, incluindo `managed.policies` e `telemetry`
* [Implantação e operações](/docs/pt/claude-apps-gateway-deploy): configuração de IdP, verificações de saúde, rotação de segredo JWT, upgrades e o modelo de segurança
* [Visão geral do gateway de aplicativos Claude](/docs/pt/claude-apps-gateway): quickstart e conectando desenvolvedores
