> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuração do gateway de aplicativos Claude

> Referência para cada opção de gateway.yaml: listener e TLS, OIDC, sessão, armazenamento Postgres, Amazon Bedrock, Claude Platform on AWS, Agent Platform do Google Cloud e upstreams Microsoft Foundry, roteamento de modelos, políticas gerenciadas e telemetria.

Uma implantação de gateway de aplicativos Claude é configurada por um arquivo YAML, convencionalmente `gateway.yaml`. O arquivo define tudo o que o gateway faz: onde ele escuta, como os desenvolvedores fazem login, para onde a inferência vai e quais políticas e telemetria se aplicam. Esta página é a referência para cada opção nesse arquivo.

Para escrever o seu primeiro, comece pelo [quickstart](/docs/pt/claude-apps-gateway#quickstart), que constrói uma configuração mínima funcional e a executa. Uma vez que você tenha uma configuração com a qual esteja satisfeito, o [guia de implantação](/docs/pt/claude-apps-gateway-deploy) cobre a containerização e hospedagem no Kubernetes, Cloud Run ou sua própria plataforma.

O gateway lê o arquivo uma vez, na inicialização, com `claude gateway --config /path/to/gateway.yaml`. Cada opção é validada contra um esquema na inicialização, portanto uma configuração malformada falha no início com um erro no nível do campo em vez de no primeiro uso.

O [exemplo completo](#complete-example) no final desta página exercita cada seção.

<h2 id="file-structure">
  Estrutura do arquivo
</h2>

Cinco seções são [obrigatórias](#required-sections). Todas as outras seções são [opcionais](#optional-sections), e uma seção omitida assume seus padrões. Chaves desconhecidas falham na inicialização, portanto um erro de digitação aparece como um erro nomeado em vez de uma configuração silenciosamente ignorada.

**Seções obrigatórias:**

* [`listen`](#listen): endereço de vinculação, URL pública, terminação TLS
* [`oidc`](#oidc): seu provedor de identidade (IdP), incluindo emissor, cliente, mapeamento de declarações e quem pode fazer login
* [`session`](#session): os tokens de portador que o gateway emite, com segredo e tempo de vida
* [`store`](#store): PostgreSQL, para concessões de dispositivo e contadores de limite de taxa
* [`upstreams`](#upstreams): para onde a inferência vai, seja Anthropic, Amazon Bedrock, Claude Platform na AWS, Agent Platform do Google Cloud ou Microsoft Foundry

**Seções opcionais:**

* [`admin`](#admin): autenticação da API de administração e retenção para limites de gastos
* [`enforcement`](#enforcement): comportamento de falha aberta ou fechada do limite de gastos
* [`models`](#models) e `auto_include_builtin_models`: lista de modelos curada pelo administrador e IDs por upstream
* [`managed`](#managed): políticas de configurações gerenciadas por grupo IdP
* [`telemetry`](#telemetry): encaminhamento OTLP para sua pilha de observabilidade
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): permitir/negar IP, limites de tamanho de solicitação, tempo até o primeiro byte upstream e limites de login por IP

<h2 id="secret-expansion">
  Expansão de segredos
</h2>

Não escreva segredos como `client_secret`, `jwt_secret` ou `postgres_url` diretamente em `gateway.yaml`. Faça referência a eles com um dos formulários abaixo, e o gateway resolve o valor na inicialização a partir de uma variável de ambiente ou um arquivo:

| Formulário      | Resolve para                                                                 | Use para                                                                   |
| --------------- | ---------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `${VAR}`        | A variável de ambiente `VAR`. A inicialização falha se não estiver definida. | Variáveis de ambiente de contêiner, AWS Secrets Manager via injeção de env |
| `${file:/path}` | Conteúdo do arquivo, aparado                                                 | Montagens de volume do Kubernetes Secret, Vault Agent, SOPS                |

<h2 id="required-sections">
  Seções obrigatórias
</h2>

<h3 id="listen">
  `listen`
</h3>

O bloco `listen` controla onde o gateway serve: o endereço de vinculação e porta, a origem visível externamente e terminação TLS opcional.

| Campo                  | Obrigatório       | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ---------------------- | ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | Não               | Endereço de vinculação. Padrão `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `port`                 | Não               | Porta de vinculação. Padrão `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `public_url`           | Atrás de um proxy | A origem `https://` visível externamente, usada para construir o `redirect_uri` do IdP e metadados de descoberta. Obrigatório atrás de qualquer proxy que termine TLS, como um ALB, Ingress ou Cloud Run, porque o gateway não confia em cabeçalhos `X-Forwarded-*` ao construir sua própria origem; eles são falsificáveis pelo cliente. `trusted_proxies` abaixo governa apenas a resolução de IP do cliente. Também obrigatório para habilitar [telemetria](#telemetry), porque o gateway constrói o endpoint OTLP que envia para clientes a partir desta URL. |
| `tls.cert` / `tls.key` | Não               | Caminhos PEM se o gateway termina TLS por si mesmo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `trusted_proxies`      | Não               | CIDRs ou IPs de balanceadores de carga na frente do gateway. Quando definido, o gateway confia em `X-Forwarded-For` apenas desses pares e registra o IP do cliente real para limitação de taxa por IP e auditoria. Equivalente ao nginx `set_real_ip_from`.                                                                                                                                                                                                                                                                                                       |

<h3 id="oidc">
  `oidc`
</h3>

O bloco `oidc` conecta o gateway ao seu provedor de identidade e decide quem pode fazer login. Ele nomeia o emissor e cliente OAuth, mapeia as declarações que carregam email e grupos e restringe o login por domínio de email ou grupo.

OpenID Connect (OIDC) é o protocolo SSO que o gateway usa com seu provedor de identidade; consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup) para saber o que registrar no lado do IdP.

| Campo                           | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `issuer`                        | Sim         | Base de descoberta OIDC. Deve servir descoberta em `/.well-known/openid-configuration`. Use HTTPS em produção; o gateway aceita um emissor `http://`. Um emissor de loopback como `http://localhost:8081` é rejeitado pela [proteção SSRF](/docs/pt/claude-apps-gateway-deploy#threat-model-summary) a menos que `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` esteja definido no ambiente do gateway.                                                                                                                                                                                                                                                                                                                  |
| `client_id` / `client_secret`   | Sim         | Do seu registro de cliente OAuth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `allowed_email_domains`         | Não         | Rejeite id\_tokens cuja declaração `email` não esteja em um desses domínios, insensível a maiúsculas/minúsculas. Defesa em profundidade contra configuração incorreta de IdP multi-tenant. Independentemente dessa configuração, um id\_token cuja declaração `email_verified` é explicitamente `false` é sempre rejeitado.                                                                                                                                                                                                                                                                                                                                                                            |
| `allowed_groups`                | Não         | Restrinja o login a membros desses grupos IdP, comparados com `groups_claim`. Um usuário em um domínio de email permitido mas em nenhum desses grupos é rejeitado. Requer que o IdP emita a declaração de grupos.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `groups_claim`                  | Não         | Qual declaração id\_token carrega associação de grupo. Padrão `groups`. Microsoft Entra emite funções de aplicativo sob `roles`. Aceita uma chave simples ou um JSON Pointer RFC 6901 como `/resource_access/gateway/roles` para declarações aninhadas.                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `google_groups`                 | Não         | Procure os grupos do usuário conectado através da API do Diretório do Google Workspace Admin SDK, porque o id\_token do Google não carrega nenhuma declaração de grupos. Defina `service_account_json_path` para um arquivo de chave de conta de serviço com delegação em todo o domínio no escopo `https://www.googleapis.com/auth/admin.directory.group.readonly` e `admin_email` para um administrador do Workspace que a conta de serviço representa; a API do Diretório requer um assunto de administrador real. Os endereços de email do grupo de cada usuário se tornam sua declaração de grupos, portanto `allowed_groups` e `managed.policies.match.groups` correspondem aos emails do grupo. |
| `email_claim`                   | Não         | Qual declaração id\_token carrega o email do usuário. Padrão `email`. Alguns IdPs, como ADFS e Entra B2C, emitem `upn` ou `preferred_username`. Aceita uma chave simples, um JSON Pointer ou uma lista de chaves de fallback onde a primeira chave presente é usada.                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `scopes`                        | Não         | Substituição completa dos escopos OIDC que o gateway solicita. Padrão `[openid, profile, email, offline_access]`. Defina quando seu IdP rejeita escopos que não reconhece ou requer um escopo personalizado para emitir grupos ou email. Deve incluir `openid`. Descartar `offline_access` desabilita tokens de atualização, portanto os desenvolvedores executam novamente o login do navegador a cada `session.ttl_hours`. Consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup) para receitas de escopo por IdP, como o fluxo de token de atualização do Google.                                                                                |
| `extra_auth_params`             | Não         | Parâmetros de consulta extras anexados à solicitação de autorização do IdP, literalmente. Este é o mecanismo de substituição para comportamento específico do IdP, como `access_type: offline` para tokens de atualização do Google, `domain_hint` para alguns locatários Entra ou `acr_values` para fluxos de step-up. Não pode substituir os parâmetros de protocolo gerenciados pelo gateway: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` e `client_id`.                                                                                                                                                                                                      |
| `userinfo_fallback`             | Não         | Quando o id\_token omite email ou grupos, busque-os em `/userinfo`. Necessário para tokens de acesso leve do Keycloak, o servidor org do Okta e tokens mínimos do ADFS. O id\_token permanece autoritário; userinfo apenas preenche lacunas. Padrão `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `use_pkce`                      | Não         | Envie um desafio PKCE (S256) na solicitação de autorização. Padrão `true`. Defina `false` apenas se seu IdP rejeitar PKCE para este cliente confidencial.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `clock_skew_seconds`            | Não         | Tolere desvio de relógio ao validar declarações de tempo id\_token. Padrão `0`, que é rigoroso. Aumente se você vir erros "token expirado / ainda não válido" logo após o login devido a desvio de relógio host/IdP.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `token_endpoint_auth_method`    | Não         | Substitua o método de autenticação do endpoint de token. Aceita `client_secret_basic` ou `client_secret_post`. Negociado automaticamente por padrão.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `id_token_signed_response_alg`  | Não         | Algoritmo de assinatura id\_token esperado. Padrão `RS256`. Defina para IdPs que assinam com ES256, PS256 ou EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `additional_authorized_parties` | Não         | Valores `azp` extras para aceitar além de `client_id`, para fluxos de broker e troca de token do Keycloak                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `discovery_url`                 | Não         | Busque o documento de descoberta desta URL em vez de derivá-lo de `issuer`, para IdPs atrás de um proxy que reescreve o host do emissor. O caminho deve conter `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `form_action_origins`           | Não         | Origens adicionais para a diretiva `Content-Security-Policy: form-action` da página `/device`. O gateway já permite `'self'` e a origem `authorization_endpoint` descoberta, mas o Chrome impõe `form-action` contra toda a cadeia de redirecionamento. Se seu IdP redireciona através de um segundo host, como Azure AD federado para ADFS, Okta hub-spoke ou um interceptador SSO corporativo, liste cada origem pela qual a solicitação de autorização pode redirecionar.                                                                                                                                                                                                                           |
| `ca_cert_pem`                   | Não         | Certificado CA PEM que substitui o armazenamento de confiança do sistema apenas para solicitações do IdP. Use para Keycloak ou Dex atrás de PKI corporativa.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

<h3 id="session">
  `session`
</h3>

O bloco `session` molda os tokens de portador que o gateway emite após o login: o segredo que os assina e quanto tempo eles vivem.

| Campo        | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------ | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | Sim         | Pelo menos 32 bytes de entropia, por exemplo de `openssl rand -base64 32`. Assina os tokens de portador HS256 do gateway. Aceita uma única string ou um array para rotação: o índice 0 assina e todas as entradas verificam. Para girar, coloque um novo segredo na frente, aguarde `ttl_hours` e depois remova o antigo.                                                                                                                                                                                           |
| `ttl_hours`  | Não         | Tempo de vida do token de portador do gateway. Padrão `1`. O CLI atualiza silenciosamente antes da expiração quando o IdP emite tokens de atualização. Um tempo de vida mais curto desprovisiona mais rápido; um mais longo faz menos viagens de ida e volta do IdP. Se seu IdP não puder emitir tokens de atualização porque `offline_access` não está disponível, não há atualização silenciosa, portanto aumente para `8` ou `12` para evitar enviar desenvolvedores de volta ao login do navegador a cada hora. |

<h3 id="store">
  `store`
</h3>

O bloco `store` aponta o gateway para seu banco de dados PostgreSQL, que contém concessões de dispositivo e contadores de limite de taxa.

| Campo             | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ----------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | Sim         | URL `postgres://` ou `postgresql://`. Obrigatório: o encontro de concessão de dispositivo, onde o callback do navegador escreve e o CLI de sondagem lê, precisa de estado entre réplicas. O gateway executa suas próprias migrações de esquema na inicialização, portanto a função precisa de `CREATE TABLE` no esquema de destino. Se sua política de segurança proíbe DDL da função de aplicativo, execute as migrações com uma função de administrador, inicialmente e novamente sempre que uma nova versão envie migrações, e conceda à função de aplicativo `SELECT, INSERT, UPDATE, DELETE` nas tabelas do gateway. Consulte [Atualizações](/docs/pt/claude-apps-gateway-deploy#upgrades) e [Postgres](/docs/pt/claude-apps-gateway-deploy#postgres). |
| `username`        | Não         | Substitui o usuário em `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `password`        | Não         | Credencial do banco de dados. Defina aqui em vez de em `postgres_url` para que a credencial fique fora da URL. Aceita qualquer caractere e tem precedência sobre credenciais de URL.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `max_connections` | Não         | Tamanho do pool de conexão Postgres por réplica. Padrão `5`, que é conservador e amigável para bancos de dados compartilhados. Com [limites de gastos](#admin) habilitados, o caminho quente faz algumas operações por solicitação de inferência, portanto aumente para um banco de dados dedicado sob carga e mantenha réplicas × isto abaixo do `max_connections` do banco de dados.                                                                                                                                                                                                                                                                                                                                                            |

Para desenvolvimento local, aponte `postgres_url` para um contêiner Postgres descartável, por exemplo `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` é uma lista ordenada. O gateway encaminha inferência para o primeiro upstream que resolve o modelo solicitado. Em `5xx`, `429`, `401`, `403`, `404` ou timeout, ele falha para o próximo; outro `4xx` não, porque esses erros são atribuíveis à solicitação em vez do upstream. Um `401` ou `403` significa que a credencial do próprio gateway falhou contra esse upstream, e um `404` significa que esse upstream não serve o modelo solicitado, portanto um upstream posterior na lista ainda pode.

Failover em `404` requer gateway v2.1.198 ou posterior. Versões anteriores retornavam o primeiro `404` ao cliente mesmo quando um upstream posterior na lista servia o modelo.

Múltiplos upstreams do mesmo provedor devem definir um `name:` distinto.

Clientes Bedrock, Claude Platform on AWS, Agent Platform e Foundry são construídos uma vez na inicialização, e seus SDKs atualizam credenciais internamente, portanto girar credenciais de nuvem não requer reinicialização. Chaves de API Anthropic estáticas e portadores são lidos na inicialização; consulte [Anthropic API](#anthropic-api).

<h4 id="anthropic-api">
  Anthropic API
</h4>

O upstream Anthropic mínimo é uma chave de API do [Claude Console](https://platform.claude.com):

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # OU um portador OAuth (por exemplo, um token trocado por Workload-Identity-Federation):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # padrão; substitua por um proxy de encaminhamento
```

As duas formas de credencial diferem no cabeçalho que enviam:

* **`api_key`**: envia `x-api-key`. Gire-a no Claude Console e atualize a variável env.
* **`oauth_token`**: envia `Authorization: Bearer`. Use a forma de portador quando sua organização emite tokens de curta duração em vez de chaves de API de longa duração. O portador é lido uma vez na inicialização, portanto atualize remontando o segredo e reiniciando.

Em vez de uma chave estática ou portador, você pode usar Workload Identity Federation. Crie uma regra de federação seguindo o [guia de Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), depois monte o JWT OIDC da sua carga de trabalho como um arquivo, como um token de conta de serviço projetado do Kubernetes ou um id-token de plataforma CI. O gateway troca o JWT por um portador de curta duração e o atualiza automaticamente. O arquivo de token é relido em cada troca, portanto tokens projetados girados são coletados sem reinicialização.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # obrigatório se a regra cobre >1 workspace
      # service_account_id: svac_...   # verificação de alvo esperado opcional
```

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Para a implantação Bedrock do lado do cliente que o gateway substitui ou está na frente, consulte [Claude Code on Amazon Bedrock](/docs/pt/amazon-bedrock). O upstream do lado do gateway:

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # preferido: cadeia de credencial padrão AWS
    # OU credenciais explícitas:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # OU um token de portador da API Bedrock:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Substitua o endpoint bedrock-runtime para implantações FIPS ou VPC-endpoint:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

Um bloco `auth` vazio usa a cadeia de credencial padrão do AWS SDK: variáveis env, `~/.aws/credentials`, função de tarefa ECS, metadados de instância EC2 ou IRSA no EKS. Em produção, dê ao pod do gateway uma função IAM em vez de incorporar chaves estáticas em uma imagem de contêiner.

Credenciais explícitas devem ser completas: o gateway falha na inicialização quando `aws_access_key_id` e `aws_secret_access_key` não estão definidos juntos, ou quando `aws_session_token` está definido sem eles. Antes da v2.1.207, um bloco `auth:` parcial passou na validação.

| Configuração            | Como                                                                                                                                                                                                                                                                                                                                                   |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Permissões IAM          | Conceda ao principal do gateway `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` nos ARNs do perfil de inferência e nos ARNs do modelo de fundação subjacente. Para o catálogo integrado em regiões dos EUA: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` e `arn:aws:bedrock:*::foundation-model/anthropic.*`. |
| Acesso ao modelo        | No console Bedrock, por região, solicite e habilite acesso ao modelo para os modelos Claude que você deseja. Perfis de inferência entre regiões (`us.anthropic.*`) requerem acesso ao modelo em cada região que o perfil abrange.                                                                                                                      |
| EKS (IRSA)              | Crie uma função IAM com a política acima e uma política de confiança para o provedor OIDC do seu cluster com escopo para a conta de serviço do gateway. Anote a conta de serviço com `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` a coleta.                                                                       |
| ECS / EC2               | Anexe a função IAM à definição de tarefa ou perfil de instância. `auth: {}` a coleta.                                                                                                                                                                                                                                                                  |
| Em qualquer outro lugar | Passe credenciais através das variáveis env `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN`, ou defina-as explicitamente em `auth:` com expansão `${VAR}`                                                                                                                                                                           |
| Região                  | `region:` é a região do endpoint da API. Perfis de inferência entre regiões roteiam através da geo (US, EU, APAC) independentemente de qual você escolher. Para regiões fora dos EUA ou ARNs de throughput provisionado, adicione um bloco [`models:`](#models) com os IDs corretos por upstream.                                                      |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS serve a primeira API Anthropic em infraestrutura AWS em `aws-external-anthropic.<region>.api.aws`. Usa IDs de modelo de primeira parte, honra cabeçalhos `anthropic-beta` conforme enviados e serve `count_tokens`, portanto nenhuma tradução específica do Bedrock se aplica. O provedor `anthropicAws` requer Claude Code v2.1.198 ou posterior; versões anteriores do gateway o rejeitam na inicialização.

Para a implantação do lado do cliente da mesma plataforma, consulte [Claude Code on Claude Platform on AWS](/docs/pt/claude-platform-on-aws). O upstream do lado do gateway:

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # enviado como x-api-key
    # OU SigV4 através da cadeia de credencial padrão AWS:
    # auth: {}
    # OU credenciais SigV4 explícitas:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Substitua o endpoint derivado:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

A plataforma é executada em uma conta AWS separada do Amazon Bedrock e assina solicitações SigV4 para seu próprio nome de serviço, `aws-external-anthropic`, portanto uma função IAM com escopo Bedrock não a autoriza. Uma chave de API em `auth.api_key` tem precedência quando credenciais SigV4 também estão definidas. Um bloco `auth` vazio usa a cadeia de credencial padrão do AWS SDK, a mesma cadeia que o upstream [Amazon Bedrock](#amazon-bedrock) usa.

| Campo                                                   | Obrigatório | Descrição                                                                                                                                          |
| ------------------------------------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `region`                                                | Sim         | Região AWS, letras minúsculas, dígitos e hífens. O gateway deriva o endpoint a partir dela como `https://aws-external-anthropic.<region>.api.aws`. |
| `workspace_id`                                          | Sim         | Enviado como um cabeçalho em cada solicitação; a plataforma o requer                                                                               |
| `auth.api_key`                                          | Não         | Chave de API para a plataforma, enviada como `x-api-key`. Não é um token de portador: os dois modos de autenticação são uma chave de API ou SigV4. |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | Não         | Credenciais SigV4 explícitas. Definir um sem o outro falha na inicialização. `auth.aws_session_token` é aceito junto com eles.                     |
| `base_url`                                              | Não         | Substitua o endpoint derivado                                                                                                                      |

Como a plataforma resolve IDs de modelo de primeira parte, o catálogo integrado roteia para ela sem um bloco [`models:`](#models). Quando você cura uma lista `models:`, chave a entrada `anthropicAws:` com o ID de primeira parte.

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

Para a configuração equivalente do lado do cliente, consulte [Claude Code on Google Cloud](/docs/pt/google-vertex-ai). O upstream do lado do gateway:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # preferido: Credenciais Padrão de Aplicativo
    # OU um arquivo de chave de conta de serviço:
    # auth: { service_account_json: /secrets/sa.json }
    # Substitua o endpoint aiplatform para Private Service Connect:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

Um bloco `auth` vazio usa Credenciais Padrão de Aplicativo: `GOOGLE_APPLICATION_CREDENTIALS`, metadados GCE ou Workload Identity do GKE. Arquivos de chave JSON de conta de serviço são suportados mas desencorajados; use Workload Identity ou anexe uma conta de serviço à instância GCE ou Cloud Run.

Defina `region: global` para usar o [endpoint global do Agent Platform](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) em vez de um regional. O Google então roteia cada solicitação para uma região disponível, portanto você não rastreia disponibilidade de modelo por região. Definir uma região específica fixa cada solicitação a ela.

| Configuração            | Como                                                                                                                                                                                                                                    |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Permissões IAM          | Conceda à conta de serviço do gateway `roles/aiplatform.user` no projeto, ou uma função personalizada com `aiplatform.endpoints.predict`. Habilite a API do Agent Platform (`aiplatform.googleapis.com`).                               |
| Acesso ao modelo        | No Model Garden, habilite os modelos Claude para seu projeto. Eles publicam em regiões específicas; verifique o cartão do modelo para regiões suportadas.                                                                               |
| GKE (Workload Identity) | Vincule uma conta de serviço GCP à conta de serviço Kubernetes do gateway e anote a KSA com `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`. `auth: {}` a coleta.                                       |
| Cloud Run / GCE         | Defina a conta de serviço do serviço para uma com `roles/aiplatform.user`. `auth: {}` a coleta.                                                                                                                                         |
| Em qualquer outro lugar | `auth: { service_account_json: /secrets/sa.json }`, o caminho para um arquivo de chave JSON montado como um segredo. O campo leva um caminho de arquivo, não o conteúdo da chave, portanto nenhuma expansão `${file:…}` está envolvida. |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Para a implantação Foundry do lado do cliente, consulte [Claude Code on Microsoft Foundry](/docs/pt/microsoft-foundry). O upstream do lado do gateway:

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # preferido: DefaultAzureCredential / Managed Identity
    # OU uma chave de API:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` resolve através de `DefaultAzureCredential`: Managed Identity no AKS, ACI ou App Service; a CLI do Azure; ou credenciais de ambiente. Chaves de API funcionam mas são em todo o projeto e não giram automaticamente. O endpoint do Foundry é derivado de `resource:`; defina o `base_url` opcional para substituí-lo para nuvens soberanas como Azure Government.

| Configuração            | Como                                                                                                                                                                                        |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                    | Conceda à identidade do gateway `Azure AI User` ou `Cognitive Services User` no recurso Foundry                                                                                             |
| Implantações            | Foundry usa nomes de implantação escolhidos pelo administrador, não IDs de modelo canônicos. Adicione um bloco [`models:`](#models) mapeando cada ID canônico para seu nome de implantação. |
| AKS (workload identity) | Federe uma Managed Identity Atribuída pelo Usuário com o emissor OIDC do cluster e vincule-a à conta de serviço do gateway. `use_azure_ad: true` a coleta via `WorkloadIdentityCredential`. |
| ACI / App Service       | Habilite identidade gerenciada atribuída pelo sistema ou pelo usuário no recurso. `use_azure_ad: true` a coleta.                                                                            |
| Em qualquer outro lugar | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Cite `${…}` dentro de `{ }`.                                                                                                                     |

<h4 id="multiple-upstreams">
  Múltiplos upstreams
</h4>

O mesmo provedor pode aparecer mais de uma vez com um `name:` distinto. Isso cobre diferentes regiões, diferentes contas através de diferentes cadeias de credencial, throughput provisionado versus sob demanda e fallback entre provedores.

O gateway tenta upstreams em ordem. `5xx`, `429`, `401`, `403`, `404`, timeouts e endpoint ausente (`501`) falham; outro `4xx` não.

`429` é capacidade por upstream, portanto esgotamento de throughput provisionado (PT) falha para sob demanda. `404` é disponibilidade de modelo por upstream, portanto um upstream que não habilitou um modelo não bloqueia um upstream posterior que o serve. Um upstream que não pode resolver o modelo solicitado é pulado sem uma viagem de rede.

Este exemplo roteia uma alocação de throughput provisionado Bedrock primeiro, transborda para sob demanda e uma segunda conta, e volta para a API Anthropic por último:

```yaml theme={null}
upstreams:
  # Primário: throughput provisionado em sua região inicial.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Transbordamento: sob demanda entre regiões.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Conta diferente: uma alocação Bedrock separada através de credenciais de função assumida.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Último recurso: API Anthropic direta.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# IDs de modelo por upstream são codificados no `name:` do upstream; um upstream
# sem um `name:` assume como padrão sua string de provedor (por exemplo, `bedrock`). Qualquer
# upstream não listado para um modelo é pulado, que é como você roteia um modelo
# para throughput provisionado enquanto tudo mais fica sob demanda.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Alavanca                        | Como                                                                                                                                                                                                                                              |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Diferentes regiões              | Um upstream Bedrock por região, cada um com sua própria `region:`. Com [`auto_include_builtin_models: true`](#models) os perfis de inferência entre regiões roteiam automaticamente; para implantações fixadas por região use um bloco `models:`. |
| Diferentes contas               | Um upstream Bedrock por conta, cada um com suas próprias credenciais em `auth:`. A cadeia padrão (`auth: {}`) usa a identidade do pod; para uma segunda conta, defina credenciais explícitas ou um token de portador.                             |
| Throughput provisionado         | Mapeie o modelo para o ARN de throughput provisionado em `models:` para o nome desse upstream. Outros upstreams mantêm o ID sob demanda, portanto a capacidade PT é esgotada antes de falhar.                                                     |
| Endpoints VPC / FIPS            | Defina `base_url:` no upstream para sua URL de endpoint VPC ou FIPS                                                                                                                                                                               |
| Roteamento com escopo de modelo | Omita um upstream do mapa `upstream_model:` de um modelo e esse upstream é pulado para esse modelo. Por exemplo, rotear Opus para throughput provisionado e Sonnet e Haiku para sob demanda.                                                      |

Falhar entre provedores de nuvem ou para a API Anthropic direta muda qual acordo, geografia e outros termos governam a solicitação.

O CLI aplica o mesmo feature gating a gateways independentemente de qual upstream serve uma determinada solicitação, portanto o failover não envia um campo de corpo que um upstream rejeitaria.

<h2 id="optional-sections">
  Seções opcionais
</h2>

<h3 id="admin">
  `admin`
</h3>

Opcional. Habilita `/v1/organizations/spend_limits`, que espelha a API de Administração Pública da Anthropic, e aplicação de gastos por desenvolvedor em `/v1/messages`. Consulte [Limites de gastos](/docs/pt/claude-apps-gateway-spend-limits) para saber como os limites são definidos e aplicados; esta seção cobre as chaves `gateway.yaml` que ativam o recurso e o ajustam.

```yaml theme={null}
admin:
  # Chaves de API estáticas nomeadas para os endpoints de administração, enviadas como x-api-key.
  # O id aparece no log de auditoria como admin-key:<id> portanto cada chave é
  # atribuível. Array para rotação: adicione a nova chave, role clientes,
  # remova a antiga.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # Grupos IdP concedidos acesso total de administrador através do JWT normal do gateway (sem chave de API).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Campo                     | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                    |
| ------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `write_keys`              | Não         | Array de `{id, key}`. Um `x-api-key` correspondente a um desses pode listar, definir e deletar limites de gastos. Os valores das chaves devem ter pelo menos 32 caracteres; `id`s devem ser únicos em `read_keys` e `write_keys`.                                                                            |
| `read_keys`               | Não         | Array de `{id, key}`. Somente leitura: cada endpoint `GET`, incluindo listagem de limites, busca de um por ID e leitura de [`/effective`](/docs/pt/claude-apps-gateway-spend-limits#%2Feffective) e [`/audit`](/docs/pt/claude-apps-gateway-spend-limits#%2Faudit).                                                    |
| `admin_groups`            | Não         | Nomes de grupos IdP. Um JWT do gateway cuja declaração `groups` inclui um desses tem acesso total de administrador, leitura e escrita, e audita como `oidc:<sub>`. Use isso para administradores humanos; use chaves de API para máquinas.                                                                   |
| `blocked_message`         | Não         | Anexado literalmente ao `429 billing_error` que um desenvolvedor bloqueado vê. Escreva a instrução completa, como uma URL ou canal Slack. Não definido, o erro é `spend limit reached`.                                                                                                                      |
| `audit_retention_days`    | Não         | Padrão `365`. Linhas `admin_audit` mais antigas são varridas.                                                                                                                                                                                                                                                |
| `spend_retention_months`  | Não         | Padrão `13`. Linhas do contador `spend` mais antigas que isso são varridas. O padrão mantém um ano completo mais o mês parcial atual para relatórios ano a ano.                                                                                                                                              |
| `identity_retention_days` | Não         | Padrão `90`. TTL de última visualização para linhas `principal_emails`, que contêm email, nome de exibição e grupos de cada desenvolvedor (PII). Deliberadamente mais curto que retenção de gastos para que uma identidade desprovisionada envelheça enquanto seus contadores de gastos anônimos permanecem. |
| `group_limit_mode`        | Não         | `min` (padrão) ou `max`. Quando um desenvolvedor está em vários grupos com limites, `min` aplica o mais restritivo e `max` o menos. Usado tanto por aplicação quanto por `/effective`.                                                                                                                       |

<h3 id="enforcement">
  `enforcement`
</h3>

O bloco `enforcement` controla como as verificações de limite de gastos se comportam quando o armazenamento está indisponível.

| Campo                  | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                        |
| ---------------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | Não         | Padrão `false`. A aplicação de gastos falha aberta em uma interrupção do Postgres, portanto a inferência fica ativa. Defina `true` para falhar fechado: desenvolvedores acima do limite são bloqueados, mas também todos se o armazenamento estiver inacessível. Não tem efeito sem um bloco [`admin:`](#admin). |

<h3 id="models">
  `models`
</h3>

O bloco `models` é uma lista de modelos curada pelo administrador opcional, servida em `/v1/models` e usada para traduzir IDs de modelo por upstream. É obrigatório para regiões Bedrock fora dos EUA, ARNs de throughput provisionado Bedrock e nomes de implantação Foundry.

```yaml theme={null}
auto_include_builtin_models: true   # false: expor apenas a lista abaixo
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: texto opcional mostrado em clientes que o exibem
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # ou um ARN de perfil de inferência
      foundry: your-opus-deployment-name
```

<h3 id="managed">
  `managed`
</h3>

O bloco `managed` define políticas de acesso baseadas em função codificadas em grupos IdP ou domínio de email. As políticas são avaliadas em ordem; a primeira correspondência é selecionada, depois mesclada na base `match: {}` catch-all descrita abaixo. Elas são servidas por usuário em `GET /managed/settings` com cache ETag/304.

```yaml theme={null}
managed:
  policies:
    # Grupos específicos primeiro.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Catch-all padrão por último: corresponde a todos que se autenticaram.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Um catch-all `match: {}`, convencionalmente listado por último, é tratado como uma camada base. Cada outra política herda qualquer chave que não defina do catch-all, portanto entradas por função apenas precisam listar o que difere do padrão da organização. As regras de mesclagem dependem do tipo de chave:

* **Listas de permissão**: `availableModels` e `permissions.allow`. A lista de uma política específica substitui completamente a da base.
* **Listas de negação e arrays de hook**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` e cada array de tipo de evento `hooks`. Estes tomam a união de base e política, portanto uma negação em toda a organização ou hook de auditoria não pode ser acidentalmente descartada por uma substituição por função.
* **Chaves do tipo registro**: `env`, `modelOverrides` e `skillOverrides`. Estes mesclam superficialmente, portanto um bloco `env` por função substitui as chaves que define e herda o resto da base.

`availableModels` também é aplicado no servidor em `/v1/messages`, portanto um modelo negado retorna `400` independentemente do que o cliente envia.

| Correspondente                                      | Comportamento                                                                                                                                                                |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Corresponde a cada usuário autenticado. Comece com um desses e adicione políticas com escopo de grupo acima mais tarde.                                                      |
| `match: { groups: [a, b] }`                         | Corresponde se a declaração `groups` do JWT contiver qualquer um dos grupos listados. Sensível a maiúsculas/minúsculas: grupos devem corresponder ao invólucro exato do IdP. |
| `match: { email_domain: example.com }`              | Corresponde à parte após o último `@` na declaração `email` do JWT, insensível a maiúsculas/minúsculas. Aceita um domínio por política.                                      |
| `match: { groups: [a], email_domain: example.com }` | Ambas as condições devem corresponder                                                                                                                                        |

Um usuário autenticado que não corresponde a nenhuma política obtém os padrões do gateway, o que significa cada modelo no catálogo e nenhuma configuração gerenciada. Adicione um catch-all `match: {}` por último se quiser uma política padrão garantida.

<Note>
  O gateway não mantém seu próprio diretório de usuários. Ele autoriza cada solicitação do token IdP do usuário, lendo associação de grupo da declaração `groups` do token e avaliando políticas contra ela. Não há lista para enumerar e nenhuma conta para pré-criar, e portanto nenhum endpoint SCIM, porque não há nada para SCIM sincronizar.

  Execute gerenciamento de ciclo de vida de usuário e grupo na fonte de verdade, que é o provisionamento SCIM nativo do seu IdP ou uma plataforma dedicada de governança de identidade. Associação e desprovisionamento governados lá fluem para o gateway automaticamente através do token. Se você quiser provisionamento SCIM de contas Claude em si, essa é uma capacidade [Claude for Enterprise](/docs/pt/admin-setup).

  Dois relógios de propagação se aplicam:

  * **Conteúdo da política**: editar uma política e reimplantar alcança clientes conectados em sua próxima sondagem de configurações gerenciadas, dentro de uma hora
  * **Associação de grupo**: mudar a associação de grupo de um usuário muda qual política os corresponde. Isso entra em vigor na próxima re-cunhagem de sessão, significando a próxima atualização silenciosa, limitada por `session.ttl_hours`.
</Note>

<h4 id="what-goes-in-cli">
  O que vai em `cli`
</h4>

Cada valor `cli` é um documento `managed-settings.json` completo do Claude Code, o mesmo esquema que você implantaria via MDM ou `/etc/claude-code/managed-settings.json`, expresso aqui como YAML. O CLI aplica o documento entregue na camada gerenciada, acima das configurações de usuário e projeto.

O gateway valida cada documento contra o esquema de configurações do CLI na inicialização, portanto uma chave de nível superior não reconhecida ou uma chave reconhecida com um valor malformado falha na inicialização com um erro nomeando cada chave ofensiva. Partes deliberadamente abertas do esquema ainda aceitam valores arbitrários, porque clientes mais novos podem reconhecer entradas que o esquema do gateway não reconhece. Essas chaves abertas são `env`, `pluginConfigs` e chaves aninhadas sob `permissions`.

Como a validação usa o esquema agrupado com a versão instalada do gateway, colocar uma chave de configurações de nível superior introduzida por uma versão mais nova do Claude Code na configuração gerenciada requer atualizar o gateway primeiro. Teste uma nova política em um cliente antes de implantá-la.

A referência de chave completa está em [Configurações do Claude Code](/docs/pt/settings#available-settings). As chaves que os operadores mais procuram primeiro:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Acesso ao modelo (também aplicado no servidor em /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Política de permissão
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # bloqueia --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignora regras de permissão de usuário/projeto

        # Ambiente empurrado para o processo CLI. DISABLE_UPDATES bloqueia
        # atualizações de fundo e manuais; DISABLE_AUTOUPDATER para apenas
        # atualizações de fundo.
        env:
          DISABLE_UPDATES: "1"                    # fixe versões através de sua própria distribuição

        # Hooks em toda a organização. Comandos de hook executam em máquinas de desenvolvedor, não no
        # gateway, portanto o caminho deve existir em cada SO do cliente na política.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Chave                                      | Aplicada por  | Efeito                                                                                                                                                                                                                               |
| ------------------------------------------ | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `availableModels`                          | Gateway + CLI | Lista de permissão de modelo. Também verificada em `/v1/messages`, portanto um cliente corrigido não pode contorná-la.                                                                                                               |
| `permissions.allow` / `.deny`              | CLI           | Regras de ferramenta e comando. Consulte [Permissões](/docs/pt/permissions).                                                                                                                                                              |
| `permissions.disableBypassPermissionsMode` | CLI           | Defina como `disable` para bloquear [`bypassPermissions`](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode), o modo que aprova automaticamente cada chamada de ferramenta, e a flag `--dangerously-skip-permissions` |
| `allowManagedPermissionRulesOnly`          | CLI           | Quando `true`, regras de permissão de usuário e projeto são ignoradas; apenas regras deste documento se aplicam                                                                                                                      |
| `env`                                      | CLI           | Variáveis de ambiente mescladas no processo CLI. Use para telemetria, atualização automática e substituições de nome de modelo.                                                                                                      |
| `hooks`                                    | CLI           | [Hooks](/docs/pt/hooks) em toda a organização                                                                                                                                                                                             |

Como essas configurações chegam pela rede, o CLI mostra a cada desenvolvedor um diálogo de aprovação de segurança única antes de aplicar qualquer coisa que possa executar um comando shell ou alterar para onde o tráfego vai. O diálogo cobre:

* `hooks`
* Variáveis `env` que não estão na lista segura integrada do CLI
* Configurações de execução de shell como `apiKeyHelper` e `statusLine`
* Conteúdo CLAUDE.md gerenciado

A lista segura determina quais variáveis `env` se aplicam sem aprovação:

* **Na lista segura**: variáveis de atualização automática e nome de modelo
* **Não na lista segura**: variáveis de proxy, variáveis de URL base e `OTEL_EXPORTER_OTLP_ENDPOINT`

A configuração de [telemetria](#telemetry) do gateway empurra `OTEL_EXPORTER_OTLP_ENDPOINT`, portanto definir `telemetry.forward_to` dispara o diálogo em cada cliente interativo. Uma execução não interativa com a flag `-p` não pode mostrar o diálogo. Ela aplica as configurações empurradas apenas para essa execução e não as registra como aprovadas, portanto a próxima sessão interativa do desenvolvedor ainda mostra o diálogo. Antes da v2.1.207, uma execução não interativa salvava as configurações como aprovadas e nenhuma sessão interativa posterior mostrava o diálogo para elas.

Se um desenvolvedor recusar, Claude Code sai em vez de aplicar a política. Empurrar um novo hook ou variável env não segura para uma política ampla portanto significa um prompt de aprovação em cada inicialização do desenvolvedor correspondente.

A chave `cli` foi nomeada `settings` em versões anteriores. Essa ortografia ainda é aceita como um alias, mas novas implantações devem usar `cli`.

<h4 id="precedence-with-other-managed-sources">
  Precedência com outras fontes gerenciadas
</h4>

Se um dispositivo também tiver um `managed-settings.json` local ou política entregue por MDM, as fontes gerenciadas não mesclam. A fonte de maior prioridade fornece todas as configurações de política, classificadas nesta ordem com maior prioridade primeiro:

1. O [auxiliar de política](/docs/pt/settings#compute-managed-settings-with-a-policy-helper)
2. Configurações entregues pelo gateway
3. MDM, via registro HKLM no Windows ou plist no macOS
4. O arquivo `managed-settings.json`
5. O registro HKCU, apenas no Windows

Hosts de incorporação podem fornecer política através da opção SDK `managedSettings`. É ignorado por padrão e se aplica apenas quando uma fonte gerenciada opta por [`parentSettingsBehavior: "merge"`](/docs/pt/settings#available-settings), filtrado para que possa apertar a política mas não afrouxá-la.

A exceção é um pequeno conjunto de chaves entre fontes, honradas quando qualquer fonte de administrador as define; a camada HKCU gravável pelo usuário é excluída:

* `sandbox.network.allowManagedDomainsOnly` e `sandbox.filesystem.allowManagedReadPathsOnly`: quando bloqueadas, as listas de permissão correspondentes são unidas entre fontes
* [`allowAllClaudeAiMcps`](/docs/pt/settings#available-settings): substituição de permissão apenas para a lista de permissão do servidor MCP claude.ai
* `sandbox.bwrapPath` e `sandbox.socatPath`: caminhos do sistema de arquivos para os binários auxiliares [sandbox](/docs/pt/sandboxing)
* [`forceRemoteSettingsRefresh`](/docs/pt/server-managed-settings): bloqueia a inicialização até que as configurações gerenciadas remotas sejam buscadas recentemente, portanto uma política MDM ou arquivo que a define é honrada mesmo quando um payload remoto em cache que carece da chave é a fonte de maior prioridade

Cada outra chave, incluindo `allowManagedPermissionRulesOnly` e `disableBypassPermissionsMode`, vem apenas da fonte de maior prioridade. Consulte [Precedência de configurações](/docs/pt/settings#settings-precedence) para a mesma regra na página de configurações.

As políticas do gateway se aplicam a cada invocação do Claude Code na máquina, incluindo execuções não interativas `claude -p` e sessões geradas pelo Agent SDK. Se o gateway estiver inacessível na inicialização, sessões assinadas saem com um erro em vez de executar sem sua política.

<Warning>
  `mcpServers` dentro de um bloco `cli` de política é rejeitado na inicialização do gateway. Distribuição de MCP por grupo não está disponível; implante servidores MCP via `managed-mcp.json` baseado em arquivo em cada dispositivo ou deixe desenvolvedores adicioná-los localmente.
</Warning>

<h3 id="telemetry">
  `telemetry`
</h3>

O CLI envia métricas, logs e, quando habilitado, rastreamentos do OpenTelemetry Protocol (OTLP) sobre HTTP para o gateway, que os retransmite literalmente para cada destino configurado. Consulte [Monitoramento de uso](/docs/pt/monitoring-usage) para as métricas e eventos que o CLI emite.

O CLI carimba cada exportação com a identidade do usuário autenticado, lida do JWT emitido pelo gateway: os atributos `user.id`, `user.email` e `user.groups`. A atribuição de custo e uso por desenvolvedor portanto funciona sem nenhuma configuração do lado do desenvolvedor.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Opt-in por sinal. Padrão: apenas métricas.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Cada destino opta por `metrics`, `logs` e `traces` independentemente, e o padrão é apenas métricas. Os sinais diferem em sensibilidade:

  * **Métricas**: contadores agregados como contagens de token, contagens de solicitação e latência
  * **Logs e rastreamentos**: podem carregar comandos bash completos, entradas de ferramenta e caminhos de arquivo, cobrindo qualquer coisa que Claude Code faz na máquina de um desenvolvedor

  Habilite logs e rastreamentos apenas em destinos com os controles de acesso e política de retenção que os dados justificam.
</Warning>

A telemetria está desativada no CLI por padrão. Configurar `telemetry.forward_to` junto com `listen.public_url` a ativa. O gateway empurra cinco variáveis env para cada cliente conectado através de `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER=otlp`
* `OTEL_LOGS_EXPORTER=otlp`
* `OTEL_TRACES_EXPORTER=otlp`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`

O endpoint empurrado é construído a partir da URL pública, portanto métricas e logs não precisam de nenhuma configuração OTEL de desenvolvedores ou políticas. A configuração empurrada é aplicada na camada gerenciada, substituindo variáveis `OTEL_*` que um desenvolvedor define localmente.

[Rastreamentos](/docs/pt/monitoring-usage#traces-beta) adicionalmente requerem `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` em cada cliente. O gateway não empurra essa variável, portanto defina-a através do bloco `env` de uma política gerenciada. Não está na lista segura do CLI, portanto entregá-la através de uma política é coberta pelo mesmo [diálogo de aprovação de segurança](#managed) que o endpoint OTLP empurrado já dispara.

Ambas as codificações OTLP protobuf e JSON são retransmitidas, e qualquer backend compatível com OpenTelemetry funciona como destino.

<h3 id="http-tuning">
  Ajuste HTTP
</h3>

Quatro blocos opcionais de nível superior, `access_control`, `limits`, `timeouts` e `rate_limits`, ajustam a superfície HTTP. Os padrões se adequam à maioria das implantações.

| Bloco            | Chave                                          | Padrão       | Descrição                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------- | ---------------------------------------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | vazio        | Permitir/negar IP de entrada por endereço do cliente, após resolução `trusted_proxies`. `deny_cidrs` é verificado primeiro; um cliente que corresponde é rejeitado mesmo se `allow_cidrs` também corresponder. Se `allow_cidrs` não estiver vazio o gateway é padrão-negar. `/healthz` e `/readyz` estão isentos de `allow_cidrs`.                                                                              |
| `limits`         | `max_request_bytes`                            | 32 MiB       | Corpo de solicitação de entrada máximo; solicitações de tamanho excessivo obtêm `413` antes do corpo ser armazenado em buffer. Aumente para solicitações de arquivo ou imagem grandes.                                                                                                                                                                                                                          |
| `limits`         | `max_request_header_bytes`                     | não definido | Quando definido, cabeçalhos de tamanho excessivo retornam `431`                                                                                                                                                                                                                                                                                                                                                 |
| `limits`         | `max_url_length`                               | não definido | Quando definido, uma URL muito longa retorna `414`                                                                                                                                                                                                                                                                                                                                                              |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000       | Espera máxima pelos cabeçalhos de resposta do upstream (tempo até o primeiro byte). O corpo da resposta então flui sem limite de relógio de parede. Aplica-se ao caminho upstream Anthropic direto; cada outro provedor é limitado pelo próprio timeout do SDK do provedor.                                                                                                                                     |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600     | Limite de taxa por IP no endpoint de autorização de dispositivo não autenticado. Aumente para uma grande organização atrás de um IP de saída compartilhado ou NAT. Esses limites se aplicam apenas ao fluxo de concessão de dispositivo de login, não a `/v1/messages` inferência. Consulte [Resistência de força bruta de código de usuário](/docs/pt/claude-apps-gateway-deploy#user-code-brute-force-resistance). |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600     | Limite de taxa por IP em envios `user_code` em `/device`                                                                                                                                                                                                                                                                                                                                                        |

<h2 id="complete-example">
  Exemplo completo
</h2>

Esta configuração de referência completa exercita cada seção principal; os blocos [ajuste HTTP](#http-tuning) mantêm seus padrões. Copie-a, delete o que você não precisa e preencha seus valores. A configuração no [Quickstart](/docs/pt/claude-apps-gateway#quickstart) é uma versão mínima disso.

```yaml gateway.yaml theme={null}
# Execute com:
#   claude gateway --config gateway.yaml
#
# A verbosidade do log operacional é controlada pela variável de ambiente
# CLAUDE_GATEWAY_LOG_LEVEL (info | warn | error; padrão info). Não afeta
# eventos de auditoria, que são sempre emitidos.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omita o bloco tls ao executar atrás de um ingress que termina TLS.
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # Obrigatório quando o emissor é o servidor org Okta, cujos id_tokens
  # podem omitir email e grupos; o gateway os preenche de /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta emite grupos apenas quando o escopo `groups` é solicitado e o
  # filtro de declaração de grupos do aplicativo os permite. A política de contratados abaixo
  # corresponde em grupos, portanto o escopo é solicitado aqui.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Funções de aplicativo Entra: use `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5

# Habilita /v1/organizations/spend_limits (espelha a API de Administração Anthropic)
# e aplicação de gastos por desenvolvedor em /v1/messages. Omita para desabilitar.
# Os limites em si são definidos via API de administração, não aqui.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # Restrinja o seletor Padrão à opção availableModels em vez de
        # o padrão de camada, portanto contratados não obtêm um 400 no padrão.
        enforceAvailableModels: true
        # allow aprova automaticamente essas ferramentas; não bloqueia o resto.
        # Adicione regras deny para restringir ferramentas.
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  Configurações gerenciadas do lado do cliente
</h2>

Tudo acima configura o servidor gateway. Apontar máquinas de desenvolvedor para ele é configurado separadamente, em cada dispositivo, através das [configurações gerenciadas](/docs/pt/settings#settings-files) do Claude Code. O gateway não pode empurrar essas chaves em si, porque são o que dizem ao cliente onde o gateway está.

Para o CLI, defina ambas as chaves no `managed-settings.json` por SO:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Implante esse arquivo em cada dispositivo, tipicamente via sua plataforma MDM. O caminho do arquivo difere por plataforma:

| Plataforma  | Caminho                                                                                                                              |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| macOS       | `/Library/Application Support/ClaudeCode/managed-settings.json`, ou o domínio de preferências gerenciadas `com.anthropic.claudecode` |
| Linux e WSL | `/etc/claude-code/managed-settings.json`                                                                                             |
| Windows     | `C:\Program Files\ClaudeCode\managed-settings.json`, ou Group Policy via registro HKLM                                               |

`forceLoginGatewayUrl` e o valor `"gateway"` de `forceLoginMethod` são honrados apenas da camada gerenciada controlada pelo administrador. Um desenvolvedor definindo-os em seu próprio `~/.claude/settings.json` não tem efeito.

<h2 id="related">
  Relacionado
</h2>

* [Visão geral do gateway de aplicativos Claude](/docs/pt/claude-apps-gateway): quickstart e conexão de desenvolvedor
* [Guia de implantação](/docs/pt/claude-apps-gateway-deploy): configuração de IdP, imagem de contêiner, Kubernetes e Cloud Run e operações
* [Limites de gastos](/docs/pt/claude-apps-gateway-spend-limits): limites por desenvolvedor e a API de Administração
