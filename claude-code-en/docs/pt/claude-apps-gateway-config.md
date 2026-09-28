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
* [`pricing`](#pricing): taxas contratadas e um multiplicador de desconto para o medidor de gastos e para os valores de custo que os desenvolvedores veem
* [`models`](#models) e `auto_include_builtin_models`: lista de modelos curada pelo administrador e IDs por upstream
* [`managed`](#managed): políticas de configurações gerenciadas por grupo IdP
* [`telemetry`](#telemetry): encaminhamento OTLP para sua pilha de observabilidade
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): permitir/negar IP, limites de tamanho de solicitação, tempo até o primeiro byte upstream e limites de login por IP
* [`load_test_mode`](#load_test_mode): teste de carga do gateway sem chamar um provedor de modelo

<h2 id="secret-expansion">
  Expansão de segredos
</h2>

Não escreva segredos como `client_secret`, `jwt_secret` ou `postgres_url` diretamente em `gateway.yaml`. Faça referência a eles com um dos formulários abaixo, e o gateway resolve o valor na inicialização a partir de uma variável de ambiente ou um arquivo:

| Formulário      | Resolve para                                                                                                                                                                                                                                                                               | Use para                                                                   |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------- |
| `${VAR}`        | A variável de ambiente `VAR`. A inicialização falha se não estiver definida.                                                                                                                                                                                                               | Variáveis de ambiente de contêiner, AWS Secrets Manager via injeção de env |
| `${file:/path}` | Conteúdo do arquivo nesse caminho absoluto, aparado. A referência deve ser o valor inteiro do campo: diferentemente de `${VAR}`, não é expandida dentro de uma string mais longa, então para uma senha de banco de dados defina `store.password` em vez de incorporá-la em `postgres_url`. | Montagens de volume do Kubernetes Secret, Vault Agent, SOPS                |

<h2 id="required-sections">
  Seções obrigatórias
</h2>

<h3 id="listen">
  `listen`
</h3>

O bloco `listen` controla onde o gateway serve: o endereço de vinculação e porta, a origem visível externamente e terminação TLS opcional.

| Campo                  | Obrigatório                      | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------------------- | -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | Não                              | Endereço de vinculação. Padrão `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `port`                 | Não                              | Porta de vinculação. Padrão `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `public_url`           | A menos que `host` seja loopback | A origem `https://` visível externamente, usada para construir o `redirect_uri` do IdP e metadados de descoberta. Obrigatório sempre que `host` não for um endereço de loopback, seja TLS terminado em um proxy como um ALB, Ingress ou Cloud Run ou no próprio gateway através de `tls`, porque o gateway nunca deriva sua própria origem de cabeçalhos `X-Forwarded-*`; eles são falsificáveis pelo cliente. Falha na inicialização sem ele. `trusted_proxies` abaixo governa apenas a resolução de IP do cliente. Também obrigatório para habilitar [telemetria](#telemetry), porque o gateway constrói o endpoint OTLP que envia para clientes a partir desta URL. |
| `tls.cert` / `tls.key` | Não                              | Caminhos PEM se o gateway termina TLS por si mesmo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `trusted_proxies`      | Não                              | CIDRs ou IPs de balanceadores de carga na frente do gateway. Quando definido, o gateway confia em `X-Forwarded-For` apenas desses pares e registra o IP do cliente real para limitação de taxa por IP e auditoria. Equivalente ao nginx `set_real_ip_from`. Entradas `X-Forwarded-For` escritas como `ipv4:port` ou `[ipv6]:port`, como alguns balanceadores de carga fazem, são lidas com a porta descartada. Um endereço IPv6 com uma porta anexada e sem colchetes pode ser lido como um endereço diferente ou não ser lido, portanto desative a opção de porta em qualquer proxy que escreva esse formulário.                                                      |

<h3 id="oidc">
  `oidc`
</h3>

O bloco `oidc` conecta o gateway ao seu provedor de identidade e decide quem pode fazer login. Ele nomeia o emissor e cliente OAuth, mapeia as declarações que carregam email e grupos e restringe o login por domínio de email ou grupo.

OpenID Connect (OIDC) é o protocolo SSO que o gateway usa com seu provedor de identidade; consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup) para saber o que registrar no lado do IdP.

| Campo                           | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------------------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Sim         | Base de descoberta OIDC. Deve servir descoberta em `/.well-known/openid-configuration`. Use HTTPS em produção; o gateway aceita um emissor `http://`. Um emissor de loopback como `http://localhost:8081` é rejeitado pela [proteção SSRF](/docs/pt/claude-apps-gateway-deploy#threat-model-summary) a menos que `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` esteja definido no ambiente do gateway.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `client_id` / `client_secret`   | Sim         | Do seu registro de cliente OAuth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `allowed_email_domains`         | Não         | Rejeite id\_tokens cuja declaração `email` não esteja em um desses domínios, insensível a maiúsculas/minúsculas. Defesa em profundidade contra configuração incorreta de IdP multi-tenant. Independentemente dessa configuração, um id\_token cuja declaração `email_verified` é explicitamente `false` é sempre rejeitado.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `allowed_groups`                | Não         | Restrinja o login a membros desses grupos IdP, comparados com `groups_claim`. Um usuário em um domínio de email permitido mas em nenhum desses grupos é rejeitado. Requer que o IdP emita a declaração de grupos. A correspondência é uma comparação de string exata e sensível a maiúsculas/minúsculas contra os valores nessa declaração, e o gateway não expande grupos aninhados: para admitir membros de um subgrupo, liste o subgrupo aqui ou configure o IdP para emitir associação achatada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `groups_claim`                  | Não         | Qual declaração id\_token carrega associação de grupo. Padrão `groups`. Microsoft Entra emite funções de aplicativo sob `roles`. Aceita uma chave simples ou um JSON Pointer RFC 6901 como `/resource_access/gateway/roles` para declarações aninhadas.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `google_groups`                 | Não         | Procure os grupos do usuário conectado através da API do Diretório do Google Workspace Admin SDK, porque o id\_token do Google não carrega nenhuma declaração de grupos. Defina `service_account_json_path` para um arquivo de chave de conta de serviço com delegação em todo o domínio no escopo `https://www.googleapis.com/auth/admin.directory.group.readonly` e `admin_email` para um administrador do Workspace que a conta de serviço representa; a API do Diretório requer um assunto de administrador real. Os endereços de email do grupo de cada usuário se tornam sua declaração de grupos, portanto `allowed_groups` e `managed.policies.match.groups` correspondem aos emails do grupo.                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `email_claim`                   | Não         | Qual declaração id\_token carrega o email do usuário. Padrão `email`. Alguns IdPs, como ADFS e Entra B2C, emitem `upn` ou `preferred_username`. Aceita uma chave simples, um JSON Pointer ou uma lista de chaves de fallback onde a primeira chave presente é usada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `scopes`                        | Não         | Substituição completa dos escopos OIDC que o gateway solicita. Padrão `[openid, profile, email, offline_access]`. Defina quando seu IdP rejeita escopos que não reconhece ou requer um escopo personalizado para emitir grupos ou email. Deve incluir `openid`. Descartar `offline_access` desabilita tokens de atualização, portanto os desenvolvedores executam novamente o login do navegador a cada `session.ttl_hours`. Consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup) para receitas de escopo por IdP, como o fluxo de token de atualização do Google.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `scope_on_refresh`              | Não         | Também envie `scope`, com a mesma lista que a solicitação de login, quando o gateway troca um token de atualização. Padrão `false`: a solicitação de atualização omite `scope`. A maioria dos IdPs retorna um id\_token em cada atualização e não precisa disso. Defina `true` quando seu IdP retorna um id\_token na atualização apenas se solicitado `openid` novamente, que Okta documenta para sua concessão de atualização. Sem um id\_token, cada atualização depende do endpoint userinfo do IdP aceitar o token de acesso atualizado. Se você controlar o login ou corresponder políticas em grupos e o id\_token de atualização do seu IdP os omitir, também defina `userinfo_fallback: true` para que o gateway os preencha a partir do endpoint userinfo. Um IdP que concedeu menos escopos do que solicitado pode rejeitar a atualização com `invalid_scope`, incluindo para sessões existentes se você adicionar entradas a `scopes` enquanto isso estiver ativado. Desdefina a chave se as atualizações começarem a falhar em `token_endpoint` depois que você a definir. Requer Claude Code v2.1.260 ou posterior no servidor gateway. |
| `extra_auth_params`             | Não         | Parâmetros de consulta extras anexados à solicitação de autorização do IdP, literalmente. Este é o mecanismo de substituição para comportamento específico do IdP, como `access_type: offline` para tokens de atualização do Google, `domain_hint` para alguns locatários Entra ou `acr_values` para fluxos de step-up. Não pode substituir os parâmetros de protocolo gerenciados pelo gateway: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` e `client_id`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `userinfo_fallback`             | Não         | Quando o id\_token omite email ou grupos, busque-os em `/userinfo`. Necessário para tokens de acesso leve do Keycloak, o servidor org do Okta e tokens mínimos do ADFS. O id\_token permanece autoritário; userinfo apenas preenche lacunas. Padrão `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `use_pkce`                      | Não         | Envie um desafio PKCE (S256) na solicitação de autorização. Padrão `true`. Defina `false` apenas se seu IdP rejeitar PKCE para este cliente confidencial.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `clock_skew_seconds`            | Não         | Tolere desvio de relógio ao validar declarações de tempo id\_token. Padrão `0`, que é rigoroso. Aumente se você vir erros "token expirado / ainda não válido" logo após o login devido a desvio de relógio host/IdP.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `token_endpoint_auth_method`    | Não         | Substitua o método de autenticação do endpoint de token. Aceita `client_secret_basic` ou `client_secret_post`. Negociado automaticamente por padrão.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `id_token_signed_response_alg`  | Não         | Algoritmo de assinatura id\_token esperado. Padrão `RS256`. Defina para IdPs que assinam com ES256, PS256 ou EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `additional_authorized_parties` | Não         | Valores `azp` extras para aceitar além de `client_id`, para fluxos de broker e troca de token do Keycloak                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `discovery_url`                 | Não         | Busque o documento de descoberta desta URL em vez de derivá-lo de `issuer`, para IdPs atrás de um proxy que reescreve o host do emissor. O caminho deve conter `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `use_proxy`                     | Não         | Envie as próprias solicitações do IdP do gateway através do proxy de encaminhamento em `HTTPS_PROXY` ou `HTTP_PROXY`, honrando `NO_PROXY`. `false` mantém essas solicitações diretas. Requer v2.1.227 ou posterior; consulte [Solicitações do IdP através de um proxy de encaminhamento](#idp-requests-through-a-forward-proxy) abaixo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `form_action_origins`           | Não         | Origens adicionais para a diretiva `Content-Security-Policy: form-action` da página `/device`. O gateway já permite `'self'` e a origem `authorization_endpoint` descoberta, mas o Chrome impõe `form-action` contra toda a cadeia de redirecionamento. Se seu IdP redireciona através de um segundo host, como Azure AD federado para ADFS, Okta hub-spoke ou um interceptador SSO corporativo, liste cada origem pela qual a solicitação de autorização pode redirecionar.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `ca_cert_pem`                   | Não         | O certificado CA codificado em PEM em si, não um caminho para um arquivo. Ele substitui o armazenamento de confiança do sistema apenas para solicitações do IdP. Para carregar um arquivo montado, escreva `${file:/etc/gateway/idp-ca.pem}`. Use para Keycloak ou Dex atrás de PKI corporativa.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h4 id="idp-requests-through-a-forward-proxy">
  Solicitações do IdP através de um proxy de encaminhamento
</h4>

Os upstreams de inferência honram `HTTPS_PROXY` e `HTTP_PROXY` em cada versão. As próprias solicitações do gateway para o IdP, descoberta, JWKS, token e userinfo, vão diretas a menos que você defina `oidc.use_proxy: true`, que requer v2.1.227 ou posterior. Quando uma variável de proxy está definida, `use_proxy` está indefinido e o emissor não é coberto por `NO_PROXY`, o gateway mantém essas solicitações diretas e registra um aviso na inicialização pedindo que você escolha; `use_proxy: false` as mantém diretas e silencia o aviso.

Com `use_proxy: true`, o pod resolve o nome do host de cada endpoint do IdP e pede ao proxy para `CONNECT` ao endereço IP resolvido, portanto o proxy deve aceitar `CONNECT` ao endereço IP de cada host que o documento de descoberta nomeia, não apenas o emissor. Use uma URL de proxy `http://`. `ca_cert_pem` e a [proteção SSRF](/docs/pt/claude-apps-gateway-deploy#threat-model-summary) se aplicam no caminho proxied também.

[Egresso apenas proxy](#proxy-only-egress) muda ambos: enquanto estiver ativo, solicitações do IdP seguem o proxy a menos que você defina `use_proxy: false`, e o gateway entrega ao proxy cada nome do host do IdP sem resolvê-lo primeiro.

<h4 id="proxy-only-egress">
  Egresso apenas proxy
</h4>

Defina `CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1` no ambiente do gateway, ao lado de `HTTPS_PROXY`, quando o pod alcança outros hosts apenas através desse proxy de encaminhamento e não pode resolver nomes DNS públicos por si mesmo, ou quando o proxy recusa `CONNECT` a um endereço IP. Requer v2.1.277 ou posterior. É uma variável de ambiente em vez de uma chave `gateway.yaml` para que nada no arquivo de configuração possa relaxar a verificação de endereço do gateway.

```bash theme={null}
export HTTPS_PROXY=http://proxy.corp.example.com:3128
export NO_PROXY=
export no_proxy=
export CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1
```

O gateway registra uma linha `network:` na inicialização enquanto o egresso apenas proxy está ativo.

Cada linha abaixo é uma classe de solicitação de saída em um gateway com `HTTPS_PROXY` definido, por padrão e enquanto o egresso apenas proxy está ativo.

| Solicitação de saída                                                                                                              | Padrão                                                                                                                                                                              | Egresso apenas proxy ativo                                                                         |
| --------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Upstreams `provider: anthropic`, troca de token de Workload Identity Federation, exportações `telemetry.forward_to`               | Resolvido e verificado localmente, depois `CONNECT` ao endereço IP verificado através do proxy. Um coletor de telemetria listado em `NO_PROXY` é alcançado diretamente em vez disso | Nome do host entregue ao proxy                                                                     |
| Descoberta do IdP, JWKS, token e userinfo                                                                                         | Direto a menos que [`oidc.use_proxy: true`](#idp-requests-through-a-forward-proxy), depois `CONNECT` ao endereço IP verificado                                                      | Nome do host entregue ao proxy, a menos que `oidc.use_proxy: false` mantenha um IdP interno direto |
| Amazon Bedrock, Claude Platform on AWS, Agent Platform do Google Cloud e upstreams Microsoft Foundry; procuras de grupo do Google | Nome do host entregue ao proxy                                                                                                                                                      | Inalterado                                                                                         |

O egresso apenas proxy permanece desativado a menos que o ambiente do gateway atenda a todas as três dessas condições:

* `HTTPS_PROXY` ou `HTTP_PROXY` está definido.
* `NO_PROXY` e `no_proxy` estão vazios. Se sua plataforma injeta um deles em pods, defina ambos para um valor vazio no contêiner do gateway. Listar um coletor de telemetria em `NO_PROXY` mantém o egresso apenas proxy desativado.
* `CLAUDE_GATEWAY_ALLOW_LOOPBACK` não está ativado. Um coletor ou IdP no próprio loopback do pod não pode ser combinado com egresso apenas proxy, porque um endereço de loopback entregue ao proxy seria o próprio do host proxy, portanto dê a esses serviços um endereço que o proxy possa alcançar em vez disso. Pela mesma razão, o gateway recusa nomes de estilo `localhost` completamente enquanto o egresso apenas proxy está ativo.

Quando uma dessas condições não é atendida, o gateway registra um aviso na inicialização nomeando a variável que a impediu e mantém o comportamento padrão.

Uma vez que o egresso apenas proxy está ativo, permita cada destino no proxy, incluindo um coletor interno e qualquer host configurado por endereço IP. Você ainda pode manter um IdP interno direto com [`oidc.use_proxy: false`](#idp-requests-through-a-forward-proxy).

<Warning>
  Ative isso apenas quando a lista de permissões do proxy for pelo menos tão rigorosa quanto a verificação do próprio gateway. O proxy deve recusar endpoints de metadados de nuvem como `169.254.169.254` e `metadata.google.internal`, endereços link-local e o próprio loopback do host proxy, e deve recusá-los pelo endereço que um nome resolve, não apenas pelo nome, porque o gateway não captura mais um nome do host que resolve para um deles. Um proxy que se conecta em qualquer lugar que é solicitado remove a [proteção SSRF](/docs/pt/claude-apps-gateway-deploy#threat-model-summary) do gateway para essas solicitações.
</Warning>

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

| Campo                     | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`            | Sim         | URL `postgres://` ou `postgresql://`. Obrigatório: o encontro de concessão de dispositivo, onde o callback do navegador escreve e o CLI de sondagem lê, precisa de estado entre réplicas. O gateway executa suas próprias migrações de esquema na inicialização e na atualização, portanto a função precisa de direitos para criar e alterar tabelas no esquema de destino. Consulte [Atualizações](/docs/pt/claude-apps-gateway-deploy#upgrades) e [Postgres](/docs/pt/claude-apps-gateway-deploy#postgres). |
| `username`                | Não         | Substitui o usuário em `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `password`                | Não         | Credencial do banco de dados. Defina aqui em vez de em `postgres_url` para que a credencial fique fora da URL. Aceita qualquer caractere e tem precedência sobre credenciais de URL.                                                                                                                                                                                                                                                                                                                |
| `max_connections`         | Não         | Tamanho do pool de conexão Postgres por réplica. Padrão `5`, que é conservador e amigável para bancos de dados compartilhados. Com [limites de gastos](#admin) habilitados, o caminho quente faz algumas operações por solicitação de inferência, portanto aumente para um banco de dados dedicado sob carga e mantenha réplicas × isto abaixo do `max_connections` do banco de dados.                                                                                                              |
| `connect_timeout_seconds` | Não         | Segundos que o gateway aguarda quando abre uma conexão Postgres. Um número inteiro de `1` a `60`, padrão `5`. Aumente se as tentativas de conexão expirem quando uma nova instância do gateway inicia. Requer Claude Code v2.1.274 ou posterior no servidor gateway. Versões anteriores recusam iniciar quando a chave está definida.                                                                                                                                                               |

Para desenvolvimento local, aponte `postgres_url` para um contêiner Postgres descartável, por exemplo `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` é uma lista ordenada. O gateway encaminha inferência para o primeiro upstream que resolve o modelo solicitado.

Em `5xx`, `429`, `401`, `403`, `404` ou timeout, o gateway falha para o próximo upstream; outro `4xx` não, porque esses erros são atribuíveis à solicitação em vez do upstream. Um `401` ou `403` significa que a credencial do próprio gateway falhou contra esse upstream. Um `404` significa que esse upstream não serve o modelo solicitado, portanto um upstream posterior na lista ainda pode.

Se você definir `forward_user_identity: true` em um upstream, um `429` que ele retorna para uma solicitação que carregava o email do desenvolvedor não falha. Consulte [como uma negação de limite por usuário chega ao desenvolvedor](#per-user-identity-headers-for-a-proxy-you-run).

Failover em `404` requer gateway v2.1.198 ou posterior. Versões anteriores retornavam o primeiro `404` ao cliente mesmo quando um upstream posterior na lista servia o modelo.

Múltiplos upstreams do mesmo provedor devem definir um `name:` distinto.

Clientes Amazon Bedrock, Claude Platform on AWS, Agent Platform do Google Cloud e Microsoft Foundry são construídos uma vez na inicialização, e seus SDKs atualizam credenciais internamente, portanto girar credenciais de nuvem não requer reinicialização. Chaves de API Anthropic estáticas e portadores são lidos na inicialização; consulte [Anthropic API](#anthropic-api).

<h4 id="upstream-error-messages">
  Mensagens de erro do upstream
</h4>

O gateway retorna a resposta de erro de um upstream ou seu próprio `502`, dependendo de como os upstreams responderam:

* **Um upstream retornou um status no qual o gateway não [falha](#upstreams)**: essa resposta do upstream. O gateway não tenta mais upstreams.
* **Cada upstream que o gateway tentou falhou de uma forma na qual [falha](#upstreams)**: o último `429`. Quando nenhum retornou um `429`, o gateway prefere, em ordem, o último `401` ou `403`, o último `404` e o último `501`. Quando nenhum retornou nenhum desses, o próprio `502` do gateway, `all upstreams failed (N attempted)`, onde N conta cada entrada em [`upstreams`](#upstreams), incluindo entradas que o gateway pulou porque não servem o modelo solicitado.

Quando o gateway retorna a resposta de um upstream, ele mantém o código de status do upstream. Se ele mantém a mensagem do upstream depende do provedor. O corpo de erro de um upstream da API Anthropic chega ao desenvolvedor inalterado.

Os upstreams Amazon Bedrock, Claude Platform on AWS, Agent Platform do Google Cloud e Microsoft Foundry podem nomear seus IDs de conta, ARNs de função e IDs de projeto no texto de erro. O gateway registra esse texto completo no [log operacional](/docs/pt/claude-apps-gateway-deploy#logs). O que o desenvolvedor vê desses upstreams depende da rejeição:

* `400` ou `413` no envelope de erro padrão da Anthropic: a mensagem do próprio upstream, como `prompt is too long`. Claude Platform on AWS, Agent Platform e Microsoft Foundry retornam esse envelope para rejeições de API de modelo.
* `400` ou `413` na forma própria do provedor: um token `capability_rejected:`. Quando o gateway não pode classificar a rejeição, `upstream rejected the request` em um `400` ou `request too large for this upstream` em um `413`.
* Qualquer outro status: cópia genérica por status, como `upstream rate limit exceeded` em um `429`.

Por exemplo, o gateway substitui `Input is too long for requested model.` do Amazon Bedrock por `capability_rejected: prompt_too_long`. Claude Code [compacta automaticamente](/docs/pt/errors#prompt-is-too-long) nesse token, como faz em `prompt is too long`.

Manter a mensagem `400` ou `413` de um upstream de nuvem ou substituí-la por um token `capability_rejected:` requer gateway v2.1.233 ou posterior.

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

<a id="per-user-identity-headers-for-a-proxy-you-run" />

<h5 id="per-user-identity-headers-for-a-proxy-you-run">
  Cabeçalhos de identidade por usuário para um proxy que você executa
</h5>

Você pode apontar o `base_url` de um upstream `provider: anthropic` para um proxy que você executa em vez de para a API Anthropic. Para dizer a esse proxy qual desenvolvedor enviou cada solicitação, defina `forward_user_identity: true` nesse upstream. O proxy pode então atribuir gastos por desenvolvedor. Requer um gateway executando Claude Code v2.1.233 ou posterior.

Por exemplo, para um proxy em `upstream-gateway.internal.example.com`:

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # padrão false
```

O gateway adiciona esses cabeçalhos a cada solicitação que encaminha para esse upstream.

| Cabeçalho                     | Valor                                                            |
| ----------------------------- | ---------------------------------------------------------------- |
| `x-litellm-end-user-id`       | O email do desenvolvedor, quando o IdP forneceu um.              |
| `x-claude-gateway-user-id`    | O assunto do IdP do desenvolvedor, da declaração `sub` do token. |
| `x-claude-gateway-user-email` | O email do desenvolvedor, quando o IdP forneceu um.              |

Quando o token do IdP não carrega email, o gateway envia apenas `x-claude-gateway-user-id` e omite os dois cabeçalhos de email. Se seu IdP coloca o email em uma declaração diferente, defina [`oidc.email_claim`](#oidc) para essa declaração.

Quando seu proxy responde `429` para uma solicitação que carregava o email do desenvolvedor, o gateway retorna essa resposta ao desenvolvedor como está em vez de falhar para o próximo upstream, portanto seu orçamento por usuário ou limite de taxa do proxy se mantém. As outras respostas do proxy seguem as [regras de failover](#upstreams) ordinárias. Se o token do IdP de um desenvolvedor não carrega email, o gateway encaminha suas solicitações sem os cabeçalhos de email, portanto um `429` para uma dessas solicitações conta como capacidade de upstream e falha. Antes da v2.1.267 no servidor gateway, cada `429` falhava.

Defina `forward_user_identity` apenas em um upstream cujo `base_url` é um proxy que você opera. O gateway envia emails de desenvolvedor para qualquer servidor que esse `base_url` nomeia. Se o `base_url` for a API Anthropic, que é o padrão, o gateway se recusa a iniciar.

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

| Configuração            | Como                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Permissões IAM          | Conceda ao principal do gateway `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` nos ARNs do perfil de inferência e nos ARNs do modelo de fundação subjacente. Para o catálogo integrado em regiões dos EUA: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` e `arn:aws:bedrock:*::foundation-model/anthropic.*`. Também conceda `bedrock:CountTokens` nos ARNs do modelo de fundação. O gateway o usa, sem custo, para contar os tokens de entrada de uma solicitação que o cliente abandonou, portanto [limites de gastos](#admin) permanecem precisos. Sem ele, o gateway volta para uma solicitação Bedrock de um token para essa contagem. |
| Acesso ao modelo        | Amazon Bedrock habilita acesso ao modelo por padrão em regiões comerciais. O portão de nível de conta restante é o formulário de caso de uso único da Anthropic: se ninguém em sua conta AWS o enviou, abra o console Amazon Bedrock, selecione um modelo Anthropic do catálogo de modelos e complete o formulário. Consulte [Enviar detalhes de caso de uso](/docs/pt/amazon-bedrock#1-submit-use-case-details) para o formulário AWS Organizations e as permissões que o remetente precisa.                                                                                                                                                                                             |
| EKS (IRSA)              | Crie uma função IAM com a política acima e uma política de confiança para o provedor OIDC do seu cluster com escopo para a conta de serviço do gateway. Anote a conta de serviço com `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` a coleta.                                                                                                                                                                                                                                                                                                                                                                                                     |
| ECS / EC2               | Anexe a função IAM à definição de tarefa ou perfil de instância. `auth: {}` a coleta.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Em qualquer outro lugar | Passe credenciais através das variáveis env `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN`, ou defina-as explicitamente em `auth:` com expansão `${VAR}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Região                  | `region:` é a região do endpoint da API. Perfis de inferência entre regiões roteiam através da geo (US, EU, APAC) independentemente de qual você escolher. Para regiões fora dos EUA ou ARNs de throughput provisionado, adicione um bloco [`models:`](#models) com os IDs corretos por upstream.                                                                                                                                                                                                                                                                                                                                                                                    |

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

Defina `region: global` para usar o [endpoint global do Agent Platform do Google Cloud](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) em vez de um regional. O Google então roteia cada solicitação para uma região disponível, portanto você não rastreia disponibilidade de modelo por região. Definir uma região específica fixa cada solicitação a ela.

| Configuração            | Como                                                                                                                                                                                                                                    |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Permissões IAM          | Conceda à conta de serviço do gateway `roles/aiplatform.user` no projeto, ou uma função personalizada com `aiplatform.endpoints.predict`. Habilite a API do Agent Platform do Google Cloud (`aiplatform.googleapis.com`).               |
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

| Configuração            | Como                                                                                                                                                                                                  |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                    | Conceda à identidade do gateway `Azure AI User` ou `Cognitive Services User` no recurso Foundry                                                                                                       |
| Implantações            | Microsoft Foundry usa nomes de implantação escolhidos pelo administrador, não IDs de modelo canônicos. Adicione um bloco [`models:`](#models) mapeando cada ID canônico para seu nome de implantação. |
| AKS (workload identity) | Federe uma Managed Identity Atribuída pelo Usuário com o emissor OIDC do cluster e vincule-a à conta de serviço do gateway. `use_azure_ad: true` a coleta via `WorkloadIdentityCredential`.           |
| ACI / App Service       | Habilite identidade gerenciada atribuída pelo sistema ou pelo usuário no recurso. `use_azure_ad: true` a coleta.                                                                                      |
| Em qualquer outro lugar | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Cite `${…}` dentro de `{ }`.                                                                                                                               |

<h4 id="static-headers-on-upstream-requests">
  Cabeçalhos estáticos em solicitações de upstream
</h4>

Para adicionar cabeçalhos fixos às solicitações que o gateway envia para um upstream, defina `headers:` nesse upstream. Use-o quando um proxy que você executa na frente do provedor roteia ou atribui tráfego por um cabeçalho.

`headers:` requer Claude Code v2.1.277 ou posterior no servidor gateway. Um gateway anterior se recusa a iniciar quando encontra a chave. Atualize cada réplica antes de adicionar a chave e remova a chave antes de reverter para uma versão anterior.

Os cabeçalhos vão para o servidor que `base_url` nomeia, ou para o endpoint do próprio provedor quando `base_url` não está definido. O provedor os recebe também a menos que seu proxy os remova.

Este exemplo alcança um upstream `provider: vertex` através de um proxy em `upstream-proxy.internal.example.com`. Ele define o cabeçalho `x-source` que o proxy lê e envia um token da variável de ambiente `PROXY_TOKEN` como `x-proxy-token`:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    base_url: https://upstream-proxy.internal.example.com
    auth: {}
    headers:
      x-source: claude-apps-gateway
      x-proxy-token: ${PROXY_TOKEN}
```

Os valores são texto ASCII imprimível sem espaço em nenhuma extremidade. Cite um número, `true` ou `false` para que YAML o leia como texto.

Para manter um segredo fora do arquivo de configuração, use [expansão de segredo](#secret-expansion) para carregar o valor de uma variável de ambiente com `${VAR}` ou de um arquivo com `${file:/path}`. Um `${VAR}` que resolve para um valor vazio impede o gateway de iniciar.

`headers:` funciona em cada provedor, e cada upstream envia apenas o seu.

Nem toda solicitação que o gateway envia para um upstream carrega eles:

| Solicitação que o gateway envia para este upstream                                   | Carrega `headers:`                    |
| ------------------------------------------------------------------------------------ | ------------------------------------- |
| `/v1/messages`, streaming ou não, e `/v1/messages/count_tokens`                      | Sim                                   |
| Uma solicitação que falhou de outro upstream                                         | Sim, apenas `headers:` deste upstream |
| Chamada `CountTokens` do Amazon Bedrock para uma solicitação que o cliente abandonou | Não                                   |
| A troca de token de Workload Identity Federation                                     | Não                                   |

Em um upstream Amazon Bedrock ou Claude Platform on AWS que assina solicitações com AWS SigV4, esses cabeçalhos fazem parte da assinatura, portanto seu proxy deve passá-los inalterados.

Se você usar um nome que o gateway reserva, ele se recusa a iniciar, e o erro de inicialização nomeia o cabeçalho. Os nomes reservados incluem:

* `authorization` e `x-api-key`
* `host`, `content-type` e `user-agent`
* Qualquer nome começando com `anthropic-`, `x-goog-`, `x-amz-` ou `x-amzn-`

<h4 id="multiple-upstreams">
  Múltiplos upstreams
</h4>

O mesmo provedor pode aparecer mais de uma vez com um `name:` distinto. Isso cobre diferentes regiões, diferentes contas através de diferentes cadeias de credencial, throughput provisionado versus sob demanda e fallback entre provedores.

O gateway tenta upstreams em ordem. `5xx`, `429`, `401`, `403`, `404`, timeouts e endpoint ausente (`501`) falham; outro `4xx` não.

`429` é capacidade por upstream, portanto esgotamento de throughput provisionado (PT) falha para sob demanda. Se você definir [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run) em um upstream, um `429` para uma solicitação que carregava o email do desenvolvedor é uma negação por usuário em vez disso e não falha.

Cada solicitação começa no primeiro upstream. Uma solicitação alcança um upstream posterior apenas quando cada upstream à sua frente falhou ou não serve o modelo solicitado.

O gateway não mantém registro de upstreams falhados, portanto enquanto um upstream está inativo, cada solicitação que o alcança ainda o tenta e aguarda sua falha antes de prosseguir.

Para um upstream da API Anthropic, [`timeouts.upstream_ttfb_ms`](#http-tuning) limita a espera em um upstream inativo. Essa configuração não se aplica aos outros provedores, onde o gateway aguarda até uma hora para um upstream começar a responder.

`404` é disponibilidade de modelo por upstream, portanto um upstream que não habilitou um modelo não bloqueia um upstream posterior que o serve. Um upstream que não pode resolver o modelo solicitado é pulado sem uma viagem de rede.

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

# IDs de modelo por upstream são codificados no `name:` do upstream.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Alavanca                        | Como                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Diferentes regiões              | Um upstream Bedrock por região, cada um com sua própria `region:`. Com [`auto_include_builtin_models: true`](#models) os perfis de inferência entre regiões roteiam automaticamente; para implantações fixadas por região use um bloco `models:`.                                                                                                                                                                                                                               |
| Diferentes contas               | Um upstream Bedrock por conta, cada um com suas próprias credenciais em `auth:`. A cadeia padrão (`auth: {}`) usa a identidade do pod; para uma segunda conta, defina credenciais explícitas ou um token de portador.                                                                                                                                                                                                                                                           |
| Throughput provisionado         | Mapeie o modelo para o ARN de throughput provisionado em `models:` para o nome desse upstream. Outros upstreams mantêm o ID sob demanda, portanto a capacidade PT é esgotada antes de falhar.                                                                                                                                                                                                                                                                                   |
| Endpoints VPC / FIPS            | Defina `base_url:` no upstream para sua URL de endpoint VPC ou FIPS                                                                                                                                                                                                                                                                                                                                                                                                             |
| Roteamento com escopo de modelo | Apenas um modelo `id` personalizado, que não é um modelo Claude integrado, pula os upstreams ausentes de seu mapa `upstream_model:`. O gateway tenta modelos integrados em cada upstream em ordem e usa o ID padrão do provedor onde o mapa não tem entrada, portanto para modelos integrados o mapa muda qual ID um upstream recebe em vez de se é tentado; um upstream que rejeita o ID segue as mesmas [regras de failover](#upstreams) que qualquer outro erro de upstream. |

Falhar entre provedores de nuvem ou para a API Anthropic direta muda qual acordo, geografia e outros termos governam a solicitação.

O CLI aplica o mesmo feature gating a gateways independentemente de qual upstream serve uma determinada solicitação, portanto o failover não envia um campo de corpo que um upstream rejeitaria.

<h2 id="optional-sections">
  Seções opcionais
</h2>

<h3 id="admin">
  `admin`
</h3>

Opcional. Ativa `/v1/organizations/spend_limits`, que espelha a Admin API pública da Anthropic, e aplicação de gastos por desenvolvedor em `/v1/messages`. Veja [Spend limits](/docs/pt/claude-apps-gateway-spend-limits) para saber como os limites são definidos e aplicados; esta seção cobre as chaves `gateway.yaml` que ativam o recurso e o ajustam.

```yaml theme={null}
admin:
  # Named static API keys for the admin endpoints, sent as x-api-key.
  # The id appears in the audit log as admin-key:<id> so each key is
  # attributable. Array for rotation: add the new key, roll clients,
  # remove the old.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # IdP groups granted full admin via the normal gateway JWT (no API key).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Campo                     | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | Não         | Array de `{id, key}`. Um `x-api-key` correspondente a um destes pode listar, definir e excluir limites de gastos. Os valores das chaves devem ter pelo menos 32 caracteres; os `id`s devem ser únicos em `read_keys` e `write_keys`.                                                                                                                                                                                                          |
| `read_keys`               | Não         | Array de `{id, key}`. Somente leitura: todos os endpoints `GET`, incluindo listagem de limites, busca de um por ID e leitura de [`/effective`](/docs/pt/claude-apps-gateway-spend-limits#%2Feffective) e [`/audit`](/docs/pt/claude-apps-gateway-spend-limits#%2Faudit).                                                                                                                                                                                |
| `admin_groups`            | Não         | Nomes de grupos do IdP. Um gateway JWT cuja declaração `groups` inclui um destes tem acesso administrativo completo, leitura e escrita, e audita como `oidc:<sub>`. Use isto para administradores humanos; use chaves de API para máquinas. Uma entrada vazia nesta lista interrompe o gateway na inicialização. Veja [Valores de correspondência que interrompem o gateway na inicialização](#matcher-values-that-stop-the-gateway-at-boot). |
| `blocked_message`         | Não         | Anexado literalmente ao `429 billing_error` que um desenvolvedor bloqueado vê. Escreva a instrução completa, como uma URL ou um canal do Slack. Quando não definido, o gateway envia apenas a mensagem padrão. Veja [Como a aplicação funciona](/docs/pt/claude-apps-gateway-spend-limits#how-enforcement-works).                                                                                                                                  |
| `audit_retention_days`    | Não         | Padrão `365`. Linhas `admin_audit` mais antigas são removidas.                                                                                                                                                                                                                                                                                                                                                                                |
| `spend_retention_months`  | Não         | Padrão `13`. Linhas do contador `spend` mais antigas que isto são removidas. O padrão mantém um ano completo mais o mês parcial atual para relatórios ano a ano.                                                                                                                                                                                                                                                                              |
| `identity_retention_days` | Não         | Padrão `90`. TTL de última visualização para linhas `principal_emails`, que contêm o email, nome de exibição e grupos de cada desenvolvedor (PII). Deliberadamente mais curto que a retenção de gastos para que uma identidade desprovisionada expire enquanto seus contadores de gastos anônimos permanecem.                                                                                                                                 |
| `group_limit_mode`        | Não         | `min` (padrão) ou `max`. Quando um desenvolvedor está em vários grupos com limites, `min` aplica o mais restritivo e `max` o menos restritivo. Usado tanto pela aplicação quanto por `/effective`.                                                                                                                                                                                                                                            |

<h3 id="enforcement">
  `enforcement`
</h3>

O bloco `enforcement` controla como as verificações de limite de gastos se comportam quando o armazenamento está indisponível.

| Campo                  | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ---------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | Não         | Padrão `false`. A aplicação de limite de gastos falha aberta em uma interrupção do Postgres, para que a inferência permaneça ativa. Defina como `true` para falhar fechada: desenvolvedores acima do limite são bloqueados, mas todos também são se o armazenamento estiver inacessível. Requer um bloco [`admin:`](#admin): a aplicação de limite de gastos só é executada quando `admin` está configurado, e o gateway se recusa a iniciar se você definir isto como `true` sem um. |

<h3 id="pricing">
  `pricing`
</h3>

O bloco `pricing` informa ao medidor de gastos o que cobrar em vez do preço de lista em USD, para que os limites e [`/effective`](/docs/pt/claude-apps-gateway-spend-limits#%2Feffective) reflitam suas taxas contratadas. Os valores permanecem em USD e continuam sendo uma estimativa, não uma fatura. Dois pré-requisitos:

* Claude Code v2.1.227 ou posterior no servidor do gateway. Versões anteriores rejeitam a chave desconhecida na inicialização.
* Um bloco [`admin:`](#admin) ou, em v2.1.268 ou posterior, um bloco [`managed:`](#managed) com pelo menos uma política. O gateway se recusa a iniciar com `pricing` definido e nenhum bloco, porque nada o leria.

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| Campo        | Obrigatório | Descrição                                                                                                                                                                                                                           |
| ------------ | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | Não         | Padrão `1`. O medidor multiplica cada valor medido por isto, seja com preço de lista ou substituído, então `0.85` cobra 85% do preço. Deve ser maior que 0 e no máximo 10, e um valor acima de 1 é uma [marcação](#mark-prices-up). |
| `overrides`  | Não         | Linhas de `{upstream, model, input, output, cache_read, cache_write}` em USD por milhão de tokens. Todas as quatro taxas são obrigatórias. Cada uma deve ser maior que 0 e no máximo 10000.                                         |

Como o medidor corresponde a uma linha de substituição:

* Uma linha substitui o preço de lista para solicitações que `upstream`, um [`upstreams[].name`](#upstreams), serve para `model`. Isto inclui a taxa de [modo rápido](/docs/pt/fast-mode#understand-the-cost-tradeoff) mais alta, então solicitações de modo rápido e padrão medem as mesmas quatro taxas.
* Um ID integrado como `claude-sonnet-4-6`, correspondido como [`models[].id`](#models), cobre cada forma datada, forma regional do Amazon Bedrock, ou forma da Plataforma de Agentes do Google Cloud que o medidor precifica como esse modelo. Qualquer outra string, como um alias ou um ARN de perfil de inferência, corresponde ao ID que o cliente enviou ou à string enviada upstream, sem distinção de maiúsculas e minúsculas.
* Onde as linhas se sobrepõem, o medidor escolhe a linha mais específica em vez da primeira linha: uma linha cujo `model` é a string de modelo exata enviada upstream, depois uma linha correspondendo ao ID exato que o cliente enviou, depois uma linha nomeando o modelo integrado.
* Um nome de upstream desconhecido falha na inicialização, assim como duas linhas para um upstream que nomeiam o mesmo modelo, incluindo duas grafias de um modelo integrado. O gateway avisa na inicialização sobre uma linha que nenhum modelo solicitável pode usar.
* Solicitações de busca na web permanecem no preço de lista de \$0,01; o multiplicador ainda se aplica a elas.

Para taxas por região, dê a cada região seu próprio upstream nomeado e uma linha por upstream.

<h4 id="mark-prices-up">
  Marcar preços para cima
</h4>

Com v2.1.271 ou posterior no servidor do gateway, você pode definir `multiplier` acima de 1, até 10, para medir mais do que o provedor cobra, por exemplo uma taxa de reembolso interno. Este exemplo mede cada solicitação em 120% do preço:

```yaml theme={null}
pricing:
  multiplier: 1.2
```

Com um bloco [`admin:`](#admin), a marcação também se aplica aos limites de gastos. O medidor conta 120% do preço, então desenvolvedores atingem seus limites mais cedo. O gateway registra um aviso na inicialização que diz isto.

O multiplicador não muda o que o provedor upstream cobra pelas solicitações.

Se o gateway também [envia as taxas para clientes conectados](#send-the-rates-to-signed-in-clients), desenvolvedores precisam de Claude Code v2.1.271 ou posterior para ver a marcação. Clientes anteriores ignoram um `multiplier` acima de 1 e mostram custos sem ele.

Um servidor de gateway anterior a v2.1.271 se recusa a iniciar se você definir um `multiplier` acima de 1.

<h4 id="send-the-rates-to-signed-in-clients">
  Enviar as taxas para clientes conectados
</h4>

Com v2.1.268 ou posterior no servidor do gateway, o gateway também coloca as taxas de `pricing` nas políticas [`managed`](#managed) que serve, como a configuração gerenciada [`modelPricing`](/docs/pt/settings-reference#modelpricing). Desenvolvedores correspondidos por uma política então veem as taxas de `pricing` para o primeiro upstream que serve cada ID de modelo em `/usage`, a linha de status e OpenTelemetry. Um desenvolvedor que não corresponde a nenhuma política não recebe configurações gerenciadas, então seus valores permanecem no preço de lista. Clientes aplicam a configuração em Claude Code v2.1.242 ou posterior.

* O que o gateway adiciona: a menos que o bloco `cli` de uma política já defina `modelPricing`, o gateway adiciona o `multiplier` e, para cada ID de modelo que um cliente pode solicitar, a linha de substituição do primeiro upstream que serve esse ID. Uma taxa que apenas um upstream de failover cobra permanece no gateway.
* Optar uma política por: defina `modelPricing` como `{}` no bloco `cli` dessa política, e seus desenvolvedores permanecem no preço de lista.
* Manter as próprias taxas de uma política: uma política cujo bloco `cli` define `modelPricing` com seu próprio `multiplier` ou `overrides` mantém esse `modelPricing` inteiro, e o gateway não adiciona nenhuma taxa de sua própria a ele.

<h3 id="models">
  `models`
</h3>

O bloco `models` é uma lista de modelos opcional curada por administrador, servida em `/v1/models` e usada para traduzir IDs de modelo por upstream. É obrigatório para regiões não-US do Amazon Bedrock, ARNs de throughput provisionado do Amazon Bedrock e nomes de implantação do Microsoft Foundry.

```yaml theme={null}
auto_include_builtin_models: true   # false: expose only the list below
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optional text shown in clients that surface it
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # or an inference-profile ARN
      foundry: your-opus-deployment-name
```

Cada chave sob `upstream_model` deve corresponder ao `name` de um upstream configurado, que é padrão para o nome do provedor. Uma chave que não corresponde a nenhum upstream falha na inicialização, então omita as linhas para provedores que você não usa.

<h3 id="managed">
  `managed`
</h3>

O bloco `managed` define políticas de acesso baseadas em funções com chave em grupos do IdP ou domínio de email. As políticas são avaliadas em ordem; a primeira correspondência é selecionada, depois mesclada na base de captura `match: {}`. Elas são servidas por usuário em `GET /managed/settings` com cache ETag/304.

```yaml theme={null}
managed:
  policies:
    # Specific groups first.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Default catch-all last: matches everyone who authenticated.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Uma captura `match: {}`, convencionalmente listada por último, é tratada como uma camada base. Cada outra política herda qualquer chave que não define da captura, então entradas por função só precisam listar o que difere do padrão da organização. As regras de mesclagem dependem do tipo de chave:

* **Listas de permissão**: `availableModels` e `permissions.allow`. A lista de uma política específica substitui completamente a da base.
* **Listas de negação e arrays de hook**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` e cada array de tipo de evento `hooks`. Estes tomam a união de base e política, então um hook de negação ou auditoria em toda a organização não pode ser acidentalmente descartado por uma substituição por função.
* **Chaves de tipo registro**: `env`, `modelOverrides` e `skillOverrides`. Estas mesclam superficialmente, então um bloco `env` por função substitui as chaves que define e herda o resto da base.

`availableModels` também é aplicado no lado do servidor em `/v1/messages`, então um modelo negado retorna `400` independentemente do que o cliente envia.

O gateway valida o valor `model` em si antes de retransmitir uma solicitação, então um valor malformado nunca atinge um upstream. Ele rejeita a solicitação com um `400` em dois casos:

* Quando o valor está faltando ou vazio, o gateway rejeita a solicitação com a mensagem `model is required`. Essa verificação requer um gateway executando Claude Code v2.1.228 ou posterior.
* Quando o valor está presente mas não é uma string, o gateway rejeita a solicitação com a mensagem `model must be a string`. Requer um gateway executando Claude Code v2.1.221 ou posterior.

| Correspondência                                     | Comportamento                                                                                                                                                            |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `match: {}`                                         | Corresponde a cada usuário autenticado. Comece com um destes e adicione políticas com escopo de grupo acima dele depois.                                                 |
| `match: { groups: [a, b] }`                         | Corresponde se a declaração `groups` do JWT contém qualquer um dos grupos listados. Sensível a maiúsculas e minúsculas: grupos devem corresponder à grafia exata do IdP. |
| `match: { email_domain: example.com }`              | Corresponde à parte após o último `@` na declaração `email` do JWT, sem distinção de maiúsculas e minúsculas. Aceita um domínio por política.                            |
| `match: { groups: [a], email_domain: example.com }` | Ambas as condições devem corresponder                                                                                                                                    |

Um usuário autenticado que não corresponde a nenhuma política obtém os padrões do gateway, o que significa cada modelo no catálogo e nenhuma configuração gerenciada. Adicione uma captura `match: {}` por último se você quiser uma política padrão garantida.

<Note>
  O gateway não mantém seu próprio diretório de usuários. Ele autoriza cada solicitação do token do IdP do usuário, lendo a associação de grupo da declaração `groups` do token e avaliando políticas contra ela. Não há lista para enumerar e nenhuma conta para pré-criar, e portanto nenhum endpoint SCIM, porque não há nada para SCIM sincronizar.

  Execute gerenciamento de ciclo de vida de usuário e grupo na fonte de verdade, que é o provisionamento SCIM nativo do seu IdP ou uma plataforma dedicada de governança de identidade. A associação e desprovisionamento governados lá fluem para o gateway automaticamente através do token. Se você quiser provisionamento SCIM de contas Claude em si, essa é uma capacidade de [Claude for Enterprise](/docs/pt/admin-setup).

  Dois relógios de propagação se aplicam:

  * **Conteúdo da política**: editar uma política e reimplantar atinge clientes conectados em sua próxima pesquisa de configurações gerenciadas, dentro de uma hora, além das [mudanças que se aplicam apenas no próximo lançamento](/docs/pt/server-managed-settings#fetch-and-caching-behavior)
  * **Associação de grupo**: mudar a associação de grupo de um usuário muda qual política o corresponde. Isto entra em vigor na próxima remintagem de sessão, significando o próximo refresh silencioso, limitado por `session.ttl_hours`.
</Note>

<h4 id="matcher-values-that-stop-the-gateway-at-boot">
  Valores de correspondência que interrompem o gateway na inicialização
</h4>

Na inicialização, o gateway verifica o bloco `match` de cada política e a lista [`admin_groups`](#admin). Qualquer um destes valores interrompe o gateway com um erro que nomeia o campo:

* Uma lista `groups` vazia
* Uma entrada vazia em `groups` ou em `admin_groups`
* Um `email_domain` vazio
* Um `email_domain` que contém `@`, espaço em branco ou uma vírgula. O gateway remove espaço em branco do valor e remove um `@` inicial antes desta verificação. Escreva um domínio simples, como `example.com`.

Antes de v2.1.232, o gateway iniciava com estes valores. Cada valor tinha este efeito:

* Um `email_domain` vazio: o gateway pulava a verificação de domínio, então uma política com um `email_domain` vazio e nenhuma lista `groups` correspondia a cada usuário autenticado
* Uma lista `groups` vazia: a política não correspondia a ninguém
* Um `email_domain` contendo `@`, espaço em branco ou uma vírgula: a política não correspondia a ninguém
* Uma entrada vazia em `groups` ou em `admin_groups`: a entrada correspondia a um usuário apenas quando a declaração `groups` do IdP desse usuário também continha uma entrada vazia. Em `admin_groups`, essa correspondência concedia acesso administrativo. Se sua lista `admin_groups` nunca continha uma entrada vazia, ninguém ganhava acesso administrativo desta forma.

<h4 id="what-goes-in-cli">
  O que vai em `cli`
</h4>

Cada valor `cli` é um documento completo de `managed-settings.json` do Claude Code, o mesmo esquema que você implantaria via MDM ou `/etc/claude-code/managed-settings.json`, expresso aqui como YAML. O CLI aplica o documento entregue na camada gerenciada, acima das configurações de usuário e projeto, no lugar das configurações gerenciadas pelo servidor. Portanto, ignora as configurações [restritas a fontes de política no nível do SO](/docs/pt/server-managed-settings#current-limitations), como `policyHelper` e `wslInheritsWindowsSettings`.

O gateway valida cada documento contra o esquema de configurações do CLI na inicialização, então uma chave de nível superior não reconhecida falha na inicialização com um erro nomeando cada chave ofensiva. Partes deliberadamente abertas do esquema ainda aceitam valores arbitrários, porque clientes mais novos podem reconhecer entradas que o esquema do gateway não. Estas chaves abertas incluem `env`, `pluginConfigs` e chaves aninhadas sob `permissions`.

Como a validação usa o esquema agrupado com a versão instalada do gateway, colocar uma chave de configurações de nível superior introduzida por um lançamento mais novo do Claude Code em configuração gerenciada requer atualizar o gateway primeiro. Teste uma nova política em um cliente antes de implantá-la amplamente.

A referência de chave completa está em [Claude Code settings](/docs/pt/settings-reference#all-settings). As chaves que operadores mais procuram primeiro:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Model access (also enforced server-side at /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Permission policy
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blocks --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignore user/project permission rules

        # Environment pushed into the CLI process. DISABLE_UPDATES blocks
        # background and manual updates; DISABLE_AUTOUPDATER stops only
        # background updates.
        env:
          DISABLE_UPDATES: "1"                    # pin versions via your own distribution

        # Org-wide hooks. Hook commands run on developer machines, not the
        # gateway, so the path must exist on every client OS in the policy.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Chave                                      | Aplicada por  | Efeito                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------------------------------ | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Gateway + CLI | Lista de permissão de modelo. Também verificada em `/v1/messages`, então um cliente corrigido não pode contorná-la.                                                                                                                                                                                                                                                                           |
| `permissions.allow` / `.deny`              | CLI           | Regras de ferramenta e comando. Veja [Permissions](/docs/pt/permissions).                                                                                                                                                                                                                                                                                                                          |
| `permissions.disableBypassPermissionsMode` | CLI           | Defina como `disable` para bloquear [`bypassPermissions`](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode), o modo que pula prompts de permissão, e a flag `--dangerously-skip-permissions`                                                                                                                                                                                  |
| `allowManagedPermissionRulesOnly`          | CLI           | Quando `true`, as configurações gerenciadas se tornam a única fonte de configurações de regras de permissão. A entrada [`allowManagedPermissionRulesOnly`](/docs/pt/settings-reference#allowmanagedpermissionrulesonly) lista cada fonte que Claude Code então ignora.                                                                                                                             |
| `env`                                      | CLI           | Variáveis de ambiente mescladas no processo do CLI. Use para telemetria, atualização automática e substituições de nome de modelo.                                                                                                                                                                                                                                                            |
| `hooks`                                    | CLI           | [hooks](/docs/pt/hooks) em toda a organização                                                                                                                                                                                                                                                                                                                                                      |
| `managedMcpServers`                        | CLI           | Servidores MCP remotos [fornecidos a cada desenvolvedor correspondido](/docs/pt/managed-mcp#provide-servers-through-managed-settings) ao lado dos servidores que eles adicionam a si mesmos, `http` e `sse` apenas. Veja [MCP servers in a policy](#mcp-servers-in-a-policy). Requer Claude Code v2.1.259 ou posterior no servidor do gateway e nos clientes. Clientes anteriores ignoram a chave. |

Como estas configurações chegam pela rede, o CLI mostra a cada desenvolvedor um diálogo de aprovação de segurança antes de aplicar as configurações listadas abaixo:

* `hooks`
* Variáveis `env` que requerem aprovação do desenvolvedor, como variáveis de proxy e URL base
* configurações de execução de shell como `apiKeyHelper` e `statusLine`
* as configurações de binário sandbox `sandbox.bwrapPath`, `sandbox.socatPath` e `sandbox.ripgrep`
* Configurações de Sandbox que interceptam tráfego, injetam credenciais ou enfraquecem isolamento, como `sandbox.network.tlsTerminate` e as configurações de porta de proxy. [Security approval dialogs](/docs/pt/server-managed-settings#security-approval-dialogs) lista todas elas.

[Approval memory](/docs/pt/server-managed-settings#approval-memory) cobre quanto tempo uma aprovação dura e quando o diálogo aparece novamente.

Claude Code aplica algumas variáveis `env` entregues sem mostrar ao desenvolvedor o diálogo de aprovação, como configurações de seleção de modelo e limites numéricos. Outras variáveis entregues podem exigir aprovação do desenvolvedor antes de entrarem em vigor; um valor de proxy, URL base ou `OTEL_EXPORTER_OTLP_ENDPOINT` não vazio sempre faz. Quando uma variável entregue precisa de aprovação, o diálogo a nomeia.

[Environment variables and the approval dialog](/docs/pt/server-managed-settings#environment-variables-and-the-approval-dialog) tem os detalhes, incluindo quatro toggles de privacidade cujo valor entregue decide se precisam de aprovação. Antes de v2.1.218, Claude Code aplicava menos variáveis sem perguntar ao desenvolvedor, então mais variáveis entregues acionavam o diálogo.

A configuração de [telemetry](#telemetry) do gateway empurra `OTEL_EXPORTER_OTLP_ENDPOINT`, então definir `telemetry.forward_to` aciona o diálogo em cada cliente interativo. O diálogo protege a máquina do desenvolvedor de um gateway comprometido ou hostil, não a organização do desenvolvedor.

Uma execução não interativa com a flag `-p` não pode mostrar o diálogo. Ela aplica as configurações empurradas para essa execução apenas e não as registra como aprovadas, então a próxima sessão interativa do desenvolvedor ainda mostra o diálogo para elas. Antes de v2.1.207, uma execução não interativa salvava as configurações como aprovadas e nenhuma sessão interativa posterior mostrava o diálogo para elas.

Se um desenvolvedor recusa, Claude Code sai dessa sessão em vez de aplicar a política. Quando você empurra um novo hook, ou qualquer variável env que aciona o diálogo, para uma política ampla, Claude Code portanto mostra o diálogo a cada desenvolvedor correspondido. Ele mostra o diálogo em uma sessão em execução na próxima pesquisa horária, e caso contrário na próxima inicialização do desenvolvedor.

A chave `cli` foi nomeada `settings` em lançamentos anteriores. Essa grafia ainda é aceita como um alias, mas novas implantações devem usar `cli`.

<h4 id="mcp-servers-in-a-policy">
  MCP servers in a policy
</h4>

Para fornecer servidores MCP aos clientes Claude Code que uma política corresponde, defina [`managedMcpServers`](/docs/pt/managed-mcp#provide-servers-through-managed-settings) no bloco `cli` dessa política. Você precisa de Claude Code v2.1.259 ou posterior no servidor do gateway e nos clientes.

O gateway verifica cada entrada na inicialização com [as mesmas regras que Claude Code aplica no cliente](/docs/pt/managed-mcp#what-an-entry-can-contain), e se uma entrada falha uma verificação, o gateway se recusa a iniciar e nomeia a entrada.

Se você escrever uma referência `${VAR}` em `gateway.yaml`, o gateway a resolve de seu ambiente na inicialização através de [secret expansion](#secret-expansion) antes de executar as verificações de entrada, então cada cliente correspondido recebe o valor literal e pode lê-lo. A [header guidance for provided servers](/docs/pt/managed-mcp#provide-servers-through-managed-settings) se aplica ao valor expandido.

O gateway rejeita a grafia `.mcp.json` `mcpServers` em um bloco `cli`, e seu erro de inicialização nomeia `managedMcpServers` como a chave a usar. Antes de v2.1.259, o gateway rejeitava qualquer definição de servidor MCP em um bloco `cli`.

<h4 id="claude-desktop-overlay">
  Claude Desktop overlay
</h4>

Se sua organização também implanta [Claude Desktop](/docs/pt/desktop), o mesmo gateway serve ambos os clientes. Aponte `bootstrapUrl`, na [managed configuration](https://claude.com/docs/third-party/claude-desktop/configuration) do Claude Desktop, para `<listen.public_url>/user/bootstrap`. Claude Desktop deriva o emissor OAuth dessa URL, executa o mesmo sign-in de código de dispositivo contra este gateway e busca sua configuração da resposta.

<Note>
  Requer Claude Code v2.1.203 ou posterior no servidor do gateway, e uma opção explícita: `/user/bootstrap` retorna 404 a menos que a política correspondendo o usuário carregue uma chave `desktop`. Um `desktop: {}` vazio opta uma política, e uma chave `desktop` na camada base `match: {}` opta em cada política que a herda. O log de auditoria registra cada solicitação como `desktop_bootstrap.serve` ou `desktop_bootstrap.denied`.
</Note>

O gateway deriva muito da resposta do bloco `cli` da política correspondida e da configuração do gateway de nível superior:

* A lista de modelos, de `availableModels`
* Ferramentas desabilitadas, de entradas `permissions.deny` de nome de ferramenta simples. Se você definir `disabledBuiltinTools` no bloco `desktop` da política, o gateway serve a união de seu valor e a lista derivada, então você pode desabilitar mais ferramentas desta forma mas não pode reabilitar uma que você desabilitou através de `permissions.deny`
* A lista de permissão de egresso, de `sandbox.network.allowedDomains`. Se você definir `coworkEgressAllowedHosts` no bloco `desktop` da política, o gateway usa esse valor em vez da lista derivada
* Um endpoint OTLP que aponta para o próprio gateway, e os atributos de identidade do usuário conectado. O gateway retransmite as exportações que recebe nesse endpoint para seus destinos `forward_to`. Ele inclui o endpoint e os atributos quando você define tanto [`telemetry.forward_to`](#telemetry) quanto `listen.public_url`.

  Claude Desktop exporta cada sinal com uma codificação: `http/protobuf`, ou `http/json` quando você define `OTEL_EXPORTER_OTLP_PROTOCOL` ou uma de suas variantes por sinal para `http/json` no `env` da política. Antes de Claude Code v2.1.261 no servidor do gateway, a resposta definia `http/json` independentemente, então um coletor que aceita apenas protobuf rejeitava as exportações do Claude Desktop

Para definir `disabledBuiltinTools`, `coworkEgressAllowedHosts` ou a configuração `managedMcpServers` própria do Claude Desktop em um bloco `desktop` de uma política, você precisa de Claude Code v2.1.232 ou posterior no servidor do gateway. O `managedMcpServers` do Claude Desktop toma um valor de array em vez de um objeto.

O gateway omite chaves sem equivalente do Claude Desktop, como `hooks` e regras de permissão com escopo como `Bash(npm *)`, da resposta de bootstrap.

Adicione o bloco `desktop` opcional ao lado de `cli` para definir configurações do Claude Desktop diretamente. Escreva configurações da [managed configuration reference](https://claude.com/docs/third-party/claude-desktop/configuration) do Claude Desktop como nomes de chave simples. Deixe de fora chaves que Claude Desktop lê apenas de MDM ou arquivos locais, como `bootstrapUrl`; o gateway as rejeita na inicialização. Antes de v2.1.232, o gateway aceitava uma lista fixa de 11 chaves de portão de recurso, como `chatTabEnabled` e `disableAutoUpdates`, e rejeitava cada outra chave na inicialização. Antes de v2.1.227, o gateway também rejeitava `chatTabEnabled` e `chatAdvancedFileAnalysisEnabled` na inicialização.

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

Cada chave é opcional; Claude Desktop aplica seu próprio padrão para qualquer chave que você omita. O gateway valida cada bloco `desktop` na inicialização contra o esquema de configuração que o próprio Claude Desktop usa, então um erro aparece na inicialização do gateway como um erro nomeando a chave em vez de atingir cada desktop conectado. O gateway falha na inicialização quando um bloco contém:

* Uma chave desconhecida
* Uma chave reconhecida cujo valor Claude Desktop rejeitaria ou descartaria silenciosamente, como um valor vazio ou uma sub-chave digitada incorretamente dentro de uma entrada aninhada. Antes de v2.1.260, o gateway descartava silenciosamente um campo digitado incorretamente dentro de um objeto aninhado de uma entrada `managedMcpServers` ou `orgPluginSettings` em vez de falhar na inicialização.
* Uma chave que o gateway computa a si mesmo: a conexão de inferência, a lista de modelos e o relé OTLP. Configure aqueles através de [`upstreams`](#upstreams), [`models`](#models) e a seção [`telemetry`](#telemetry) `forward_to`.
* Um alias legado de uma chave atual. No erro de inicialização, o gateway nomeia a chave canônica a escrever.

Se você usar um valor ou forma de entrada descontinuada, como uma entrada `managedMcpServers` sem `transport`, o gateway inicia e registra um aviso nomeando a substituição.

O gateway valida um bloco `desktop` contra o esquema agrupado com sua versão instalada, como faz com o bloco `cli`. Para entregar uma configuração introduzida por um lançamento mais novo do Claude Desktop, atualize o gateway primeiro. Por exemplo, `userPluginMarketplacesEnabled` e `userPluginUploadsEnabled` precisam de Claude Code v2.1.260 ou posterior no servidor do gateway e Claude Desktop 1.37937.0 ou posterior nas máquinas dos membros.

Se você definir `orgPluginSettings` em um bloco `desktop` de uma política, o gateway o serve na forma de array que Claude Desktop 1.15200.0 e posterior lê. Desktops mais antigos ignoram o array e não aplicam nenhuma política de ferramenta de plugin, então atualize membros para 1.15200.0 ou posterior antes de confiar nisso.

O gateway preenche chaves que um bloco `desktop` de uma política não define a partir do bloco `desktop` da captura `match: {}`, da mesma forma que preenche um bloco `cli` de uma política a partir da base. Se você definir `disabledBuiltinTools` ou `builtinToolPolicy` tanto na base quanto em uma política de função, o gateway mantém a restrição da base:

* `disabledBuiltinTools`: o gateway usa a união da lista da base e da lista da política
* `builtinToolPolicy`: se você definir uma ferramenta para um valor diferente de `allow` na base, o gateway mantém esse valor mesmo se você definir `allow` para a mesma ferramenta em uma política de função

Para cada outra chave, se você a definir na política de função, o gateway usa o valor da política de função. O gateway substitui um array ou um objeto aninhado como `banner` inteiro, então se você definir `banner.text` em uma política de função, o gateway descarta o `banner.backgroundColor` da base.

Se você não implanta Claude Desktop, deixe `desktop` de fora de suas políticas inteiramente; o gateway então retorna 404 de `/user/bootstrap` para cada usuário.

<h4 id="precedence-with-other-managed-sources">
  Precedência com outras fontes gerenciadas
</h4>

Se um dispositivo também tem uma política entregue por MDM ou um `managed-settings.json` local, as configurações entregues pelo gateway classificam primeiro. [Precedence within the managed tier](/docs/pt/managed-settings#precedence-within-the-managed-tier) na página de configurações gerenciadas diz quando as fontes locais se aplicam, e tem as [chaves que Claude Code lê de cada fonte de administrador](/docs/pt/managed-settings#keys-read-from-every-admin-source) independentemente de qual fonte selecionou, como as chaves de bloqueio de sandbox, `forceRemoteSettingsRefresh` e o `env` por variável mesclado. Um [`policyHelper`](/docs/pt/settings-reference#policyhelper) configurado em um perfil MDM ou no arquivo de configurações gerenciadas é executado apenas quando o gateway não entrega configurações; a entrada diz o que sua saída substitui.

Hosts de incorporação como [Claude Desktop](/docs/pt/desktop) podem fornecer política através da opção SDK `managedSettings`. [Parent settings from embedding hosts](/docs/pt/managed-settings#parent-settings-from-embedding-hosts) diz quando Claude Code a aplica, e [Restrict parent settings](/docs/pt/claude-apps-gateway#restrict-parent-settings) lista quais configurações de direção de permissão ainda se aplicam sem os bloqueios `allowManaged*Only`.

As políticas do gateway se aplicam a cada invocação do Claude Code na máquina, incluindo execuções não interativas `claude -p` e sessões geradas pelo Agent SDK. Se o gateway estiver inacessível na inicialização, sessões conectadas saem com um erro em vez de executar sem sua política.

<h3 id="telemetry">
  `telemetry`
</h3>

O CLI envia métricas, logs e, quando habilitado, rastreamentos para o gateway, que os retransmite verbatim para cada destino configurado. As exportações usam OpenTelemetry Protocol (OTLP) sobre HTTP. Para pular o relé e ter sessões exportar diretamente para seu coletor, [nomeie o coletor em uma política](#export-directly-to-your-collector). Veja [Monitoring usage](/docs/pt/monitoring-usage) para as métricas e eventos que o CLI emite.

O CLI carimba cada exportação com a identidade do usuário autenticado, lida do JWT emitido pelo gateway: os atributos `user.id`, `user.email` e `user.groups`. A atribuição de custo e uso por desenvolvedor portanto funciona sem nenhuma configuração no lado do desenvolvedor.

[Claude Desktop](#claude-desktop-overlay) e sessões Cowork conectadas através do gateway carimbam sua telemetria com `user.email` e `user.groups` ao lado de `enduser.id`, então você pode cobrir uso de terminal, Desktop e Cowork com uma consulta em `user.email` ou `user.groups`. `user.groups` é a lista de grupo do IdP separada por vírgula.

Desktop e telemetria Cowork também carregam `enduser.sub`, a declaração `sub` que seu provedor de identidade emite para o usuário, que permanece a mesma quando o email de um usuário muda. Sessões de terminal carimbam o mesmo valor sob `user.id`, então uma consulta que corresponde `enduser.sub` contra `user.id` de terminal cobre uso de terminal, Desktop e Cowork de um usuário junto. Em exportações Desktop e Cowork, `user.id` é um identificador anônimo, não o assunto.

Como todos os dados OpenTelemetry do Claude Code, estes atributos vão apenas para destinos que sua organização configura, nunca para Anthropic.

Se a lista de grupos de um usuário é mais longa que 255 caracteres uma vez codificada em percentual, ou um nome de grupo contém uma vírgula ou sinal de igual, o gateway deixa `user.groups` de fora da telemetria Desktop e Cowork desse usuário em vez de truncá-la. As sessões de terminal desse usuário ainda carregam a lista completa.

O gateway deixa `enduser.sub` de fora quando o assunto é mais longo que 255 caracteres uma vez codificado em percentual, ou contém um espaço, um caractere fora de ASCII imprimível, ou um de `,` `;` `=` `\` `"` `%`. A telemetria Desktop e Cowork desse usuário mantém seus outros atributos.

Você precisa de Claude Code v2.1.265 ou posterior no servidor do gateway para `user.email` e `user.groups` na telemetria Desktop e Cowork, e Claude Desktop 1.24012 ou posterior em cada máquina do desenvolvedor para `user.groups`.

Você precisa de Claude Code v2.1.274 ou posterior no servidor do gateway para `enduser.sub`.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Per-signal opt-in. Default: metrics only.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Cada destino opta em `metrics`, `logs` e `traces` independentemente, e o padrão é apenas métricas. Os sinais diferem em sensibilidade:

  * **Metrics**: contadores agregados como contagens de tokens, contagens de solicitações e latência
  * **Logs and traces**: podem carregar comandos Bash completos, entradas de ferramentas e caminhos de arquivo, cobrindo qualquer coisa que Claude Code faz na máquina de um desenvolvedor

  Habilite logs e rastreamentos apenas em destinos com os controles de acesso e política de retenção que os dados justificam.
</Warning>

Cada URL `forward_to` deve usar `https://`, com uma exceção para um coletor na própria interface de loopback do gateway:

* `http://localhost:<port>` passa validação de configuração, mas a [SSRF guard](/docs/pt/claude-apps-gateway-deploy#threat-model-summary) bloqueia cada exportação com `ECONNREFUSED_SSRF` a menos que você defina `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` no ambiente do gateway
* `http://127.0.0.1:<port>` ou `http://[::1]:<port>` falha na inicialização a menos que essa variável esteja definida

Para um coletor em cluster, exponha-o sobre HTTPS em seu próprio endereço interno, ou execute-o como um sidecar com a variável definida.

Quando `HTTPS_PROXY` está definido, o gateway envia exportações através desse proxy.

Para alcançar um coletor interno diretamente, adicione-o a `NO_PROXY` por nome de host ou por um domínio com um ponto inicial como `.internal.example.com`, que requer Claude Code v2.1.277 ou posterior no servidor do gateway. Certifique-se de que o gateway pode alcançar o coletor sem o proxy. Uma entrada sem um ponto inicial corresponde apenas a esse nome exato, não a nomes sob ele. Intervalos CIDR não correspondem.

Com [proxy-only egress](#proxy-only-egress) ligado, permita o coletor no proxy em vez disso, já que qualquer entrada `NO_PROXY` mantém proxy-only egress desligado.

Telemetria está desligada no CLI por padrão. Quando você define tanto `telemetry.forward_to` quanto `listen.public_url`, o gateway a liga para clientes conectados empurrando seis variáveis de ambiente através de `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` e `OTEL_TRACES_EXPORTER`, cada um definido para `otlp` se pelo menos um destino `forward_to` habilita esse sinal e para `none` caso contrário
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

Antes de Claude Code v2.1.265 no servidor do gateway, o gateway empurrava todos os três seletores de exportador como `otlp`, incluindo para sinais que nenhum destino optou.

O endpoint empurrado é construído a partir da URL pública, então métricas e logs não precisam de nenhuma configuração OTEL de desenvolvedores ou políticas.

Desenvolvedores conectados através de `/login` não podem redirecionar exportações com sua própria configuração OTEL:

* **Variáveis definidas localmente**: Claude Code aplica as variáveis empurradas na camada gerenciada, então cada uma substitui o valor que um desenvolvedor define para ela localmente.
* **Endpoints configurados localmente**: com exportação OTLP/HTTP habilitada, o CLI ignora qualquer endpoint configurado localmente, independentemente de o gateway ter empurrado as variáveis de telemetria. Suas exportações vão para o gateway a menos que uma política [nomeie seu coletor como o endpoint](#export-directly-to-your-collector).

Sem um destino `forward_to` para um sinal, o gateway o aceita e descarta. Se desenvolvedores já exportam telemetria do Claude Code para um de seus coletores, adicione-o como um destino `forward_to`, com logs ou rastreamentos habilitados se eles exportarem aqueles, então continua recebendo seus dados depois que eles se conectam. Para pular o relé em vez disso, [nomeie o coletor em uma política](#export-directly-to-your-collector).

[Traces](/docs/pt/monitoring-usage#traces-beta) também requerem `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` em cada cliente. Defina-o no bloco `env` de uma política gerenciada, já que o gateway não o empurra. Desenvolvedores o aprovam no mesmo [security approval dialog](#managed) que o endpoint empurrado já aciona.

Defina-o para `1` apenas nas políticas cujos grupos você quer rastreados. Uma política que não o define herda o valor de sua política de captura `match: {}` se essa política define um, por [merge rules](#managed). Para impedir que os clientes de um grupo enviem rastreamentos mesmo quando um desenvolvedor define a variável localmente, defina-a para `0` na política desse grupo.

Ambas as codificações OTLP protobuf e JSON são retransmitidas, e qualquer backend compatível com OpenTelemetry funciona como um destino.

<h4 id="export-directly-to-your-collector">
  Exportar diretamente para seu coletor
</h4>

Para ter sessões conectadas através de `/login` enviar telemetria diretamente para seu coletor em vez de através do relé, defina `OTEL_EXPORTER_OTLP_ENDPOINT` para a URL base `https://` do coletor no bloco `env` de uma [managed policy](#managed). Claude Code anexa `/v1/metrics`, `/v1/logs` ou `/v1/traces` à URL que você define, como `https://otel-collector.example.com:4318`, e exporta cada sinal lá sobre OTLP/HTTP. Requer Claude Code v2.1.265 ou posterior em cada máquina do desenvolvedor. Clientes anteriores exportam através do relé.

Para autenticar para o coletor, defina `OTEL_EXPORTER_OTLP_HEADERS` no mesmo bloco `env`. Sessões nunca enviam o token de sessão do gateway do desenvolvedor para um coletor nomeado desta forma.

Quando você adiciona ou muda este endpoint em uma política, Claude Code pede a cada desenvolvedor para aprová-lo no [security approval dialog](#managed) antes de aplicá-lo em uma sessão interativa.

Claude Code verifica o endpoint antes de exportar um sinal diretamente, e mantém esse sinal no relé quando uma verificação falha. As verificações incluem:

* O endpoint vem do próprio gateway. Se você definir a mesma variável em um perfil MDM ou um `managed-settings.json` local, exportações permanecem no relé.
* A URL usa `https://`, ou `http://` para um endereço de loopback
* A URL resolve para um caminho terminando em `/v1/<signal>`, sem consulta ou fragmento. Claude Code constrói esse caminho a si mesmo a partir da variável genérica. Ele usa uma variável por sinal como `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` conforme escrito, então inclua o caminho completo lá.
* A URL não é o próprio host do gateway. Um endpoint endereçado ao gateway mantém o caminho de relé e seu token de sessão.
* Nem você nem o desenvolvedor configurou [`otelHeadersHelper`](/docs/pt/settings-reference#otelheadershelper) em nenhuma fonte de configurações. Com um helper configurado, cada sinal permanece no relé.

O endpoint que você nomeia muda apenas para onde as exportações vão. Você ainda escolhe quais sinais exportam em tudo com os seletores `OTEL_*_EXPORTER`.

O endpoint sozinho não liga a exportação, então também defina as variáveis que fazem, a menos que o gateway já as empurre:

* Se o gateway já [empurra as variáveis de telemetria](#telemetry), elas cobrem habilitação, seletores e protocolo, e seu endpoint explícito substitui o valor `<public_url>` empurrado. Defina um seletor `OTEL_*_EXPORTER` para `otlp` você mesmo apenas para um sinal que nenhum destino `forward_to` habilita.
* Se não, também defina `CLAUDE_CODE_ENABLE_TELEMETRY=1`, os seletores `OTEL_*_EXPORTER` e `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`.

Quando o desenvolvedor se desconecta, ou se conecta a um gateway diferente, exportações para o coletor param e Claude Code descarta cada lote restante em vez de enviá-lo.

<h4 id="when-a-destination-fails">
  Quando um destino falha
</h4>

O gateway não armazena em buffer, tenta novamente ou armazena telemetria, então descarta uma exportação que não atinge um destino em vez de entregá-la tarde. Cada destino sucede ou falha por conta própria, e o cliente exportador recebe uma resposta de sucesso de qualquer forma, então uma entrega falhada aparece apenas no log do gateway.

Após cinco falhas consecutivas de entrega para um destino, o gateway pausa o encaminhamento para ele em trechos de 30 segundos, registrando cada pausa, até que uma entrega suceda. Qualquer resposta de erro, timeout ou erro de conexão conta como uma falha de entrega, exceto `400`, `413`, `415`, `422` e `431`, que significam que o coletor rejeitou a carga dessa exportação como malformada ou muito grande.

Uma carga rejeitada nem avança nem reseta a contagem de falhas: o gateway continua encaminhando para o destino e registra um aviso nomeando-o e o status, na primeira recusa do destino e a cada centésima depois.

<h3 id="http-tuning">
  HTTP tuning
</h3>

Quatro blocos opcionais de nível superior, `access_control`, `limits`, `timeouts` e `rate_limits`, ajustam a superfície HTTP. Os padrões se adequam à maioria das implantações.

| Bloco            | Chave                                          | Padrão       | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------- | ---------------------------------------------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | vazio        | Inbound IP permitir/negar por endereço do cliente, após resolução de `trusted_proxies`. `deny_cidrs` é verificado primeiro; um cliente que corresponde é rejeitado mesmo se `allow_cidrs` também corresponde. Se `allow_cidrs` não está vazio o gateway é padrão-negar. `/healthz` e `/readyz` estão isentos de `allow_cidrs`. Quando um proxy confiável envia uma entrada `X-Forwarded-For` que não é um endereço IP, o cliente real é desconhecido e o gateway registra um aviso uma vez nomeando o que verificar. Onde qualquer lista se aplica à solicitação, ela a recusa com `403` e razão de auditoria `xff_unparseable`. Onde nenhuma se aplica, ela serve a solicitação e usa o endereço do próprio proxy como o IP do cliente para limites de taxa por IP e auditoria. |
| `limits`         | `max_request_bytes`                            | 32 MiB       | Corpo de solicitação inbound máximo; solicitações de tamanho excessivo obtêm `413` antes do corpo ser armazenado em buffer. Aumente para solicitações de arquivo ou imagem grandes.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `limits`         | `max_request_header_bytes`                     | não definido | Quando definido, cabeçalhos de tamanho excessivo retornam `431`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `limits`         | `max_url_length`                               | não definido | Quando definido, uma URL muito longa retorna `414`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000       | Espera máxima pelos cabeçalhos de resposta do upstream (tempo até o primeiro byte). O corpo da resposta então flui sem limite de relógio de parede. Aplica-se ao caminho direto do upstream Anthropic; em cada outro provedor o gateway aguarda até uma hora para a resposta começar.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600     | Limite de taxa por IP no endpoint de autorização de dispositivo não autenticado. Aumente para uma grande organização atrás de um IP de egresso compartilhado ou NAT. [Large rollouts](/docs/pt/claude-apps-gateway-deploy#large-rollouts) mostra como dimensioná-lo. Estes limites se aplicam apenas ao fluxo de concessão de dispositivo de sign-in, não à inferência `/v1/messages`. Veja [User-code brute-force resistance](/docs/pt/claude-apps-gateway-deploy#user-code-brute-force-resistance).                                                                                                                                                                                                                                                                                      |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600     | Limite de taxa por IP em envios de `user_code` em `/device`. É o que impede alguém de adivinhar o código de outro desenvolvedor. [Large rollouts](/docs/pt/claude-apps-gateway-deploy#large-rollouts) mostra até onde elevá-lo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

Se você deixar ambas as listas `access_control` vazias, que é o padrão, o gateway serve qualquer endereço de cliente, então apenas sua rede restringe quem pode alcançá-lo. Isto importa porque um gateway pode empurrar [managed settings](#managed) que executam comandos em máquinas de desenvolvedores.

Enquanto `allow_cidrs` está vazio, o gateway avisa em dois lugares, sem mudar como responde a qualquer solicitação:

* **Na inicialização**: um aviso no log operacional recomenda permitir apenas os intervalos privados `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `100.64.0.0/10`, `127.0.0.0/8`, `::1/128` e `fc00::/7`, mais qualquer outro intervalo interno de onde seus desenvolvedores se conectam. Se você vincular o gateway a um endereço de loopback e não definir nem `trusted_proxies` nem `public_url`, como em desenvolvimento local, o aviso não aparece.
* **Em tempo de execução**: a primeira vez que uma solicitação chega de um endereço fora desses intervalos privados, o gateway registra um aviso e emite um [`access.public_client` audit event](/docs/pt/claude-apps-gateway-deploy#logs) carregando o IP do cliente. Ambos disparam uma vez por processo. Endereços link-local, `169.254.0.0/16` e `fe80::/10`, não contam como públicos. O gateway responde `/healthz` e `/readyz` antes desta verificação ser executada, então sondas de saúde de intervalos públicos não a acionam.

Ambos os sinais usam o endereço do cliente conforme o gateway o resolve. Se um balanceador de carga, port-forward ou túnel retransmite tráfego e não está listado em `listen.trusted_proxies`, o gateway vê o endereço do relé, que é geralmente privado, então nem o aviso em tempo de execução nem uma lista de permissão privada o captura.

Atrás de tal front end, defina [`listen.trusted_proxies`](#listen) primeiro para que o gateway veja endereços de cliente reais, e mantenha o gateway e tudo na frente dele inacessível da internet pública independentemente.

<h3 id="load_test_mode">
  `load_test_mode`
</h3>

O bloco `load_test_mode` permite que você teste a carga de um gateway sem chamar um provedor de modelo. Enquanto está ligado, o gateway constrói e assina cada solicitação de provedor como de costume, descarta-a em vez de enviá-la e transmite uma resposta enlatada de volta através de seu caminho de resposta normal. A resposta é texto de preenchimento que começa com uma frase dizendo que é enlatada.

Requer v2.1.283 ou posterior. Versões anteriores se recusam a iniciar quando a chave está definida, então atualize cada réplica antes de adicionar o bloco e remova-o antes de fazer rollback.

O exemplo abaixo liga o modo com os padrões, uma resposta de aproximadamente 750 tokens de saída transmitida em cerca de 10 segundos:

```yaml theme={null}
load_test_mode:
  enabled: true
  reply_tokens: 750     # roughly how many tokens of text each canned reply carries
  reply_seconds: 9.5    # how long a streamed reply takes
```

| Campo           | Obrigatório | Descrição                                                                                                                                                                                 |
| --------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`       | Sim         | `true` liga o modo. `false` mantém seus números no arquivo com o modo desligado. O gateway se recusa a iniciar se o bloco está presente sem ele.                                          |
| `reply_tokens`  | Não         | Padrão `750`. Aproximadamente quantos tokens de texto cada resposta enlatada carrega, um número inteiro de 1 a 100000.                                                                    |
| `reply_seconds` | Não         | Padrão `9.5`. Quanto tempo uma resposta transmitida leva, de 0 a 600. `0` envia a resposta inteira de uma vez. Uma resposta para uma solicitação não transmitida sempre volta de uma vez. |

Um teste de carga neste modo cobre o gateway, seu Postgres e tudo na frente do gateway. Não cobre os limites, velocidade ou caminho de rede do provedor.

Enquanto o modo está ligado, uma solicitação pode carregar um cabeçalho `x-load-test-user` contendo um número inteiro de até sete dígitos, e o gateway conta cada número como um desenvolvedor separado com o email e grupos do desenvolvedor cujo token veio com a solicitação. Dê à implantação de teste de carga seu próprio banco de dados vazio, porque o gateway se recusa a iniciar com o modo ligado contra um banco de dados no qual qualquer desenvolvedor já gastou algo.

<Warning>
  Nunca ligue isto para um gateway que desenvolvedores usam. Cada solicitação obtém a resposta enlatada e nenhum modelo é chamado. O gateway registra um aviso `load_test_mode is on` na inicialização e marca cada [audit event](/docs/pt/claude-apps-gateway-deploy#logs) de `inference` com `load_test: true` enquanto o modo está ligado.
</Warning>

<h2 id="complete-example">
  Exemplo completo
</h2>

Esta configuração de referência completa exercita cada seção principal; os [blocos de ajuste HTTP](#http-tuning) mantêm seus padrões. Copie-a, delete o que você não precisa e preencha seus valores. A configuração no [Quickstart](/docs/pt/claude-apps-gateway#quickstart) é uma versão mínima desta.

```yaml gateway.yaml theme={null}
# Execute com:
#   claude gateway --config gateway.yaml
#
# A verbosidade do log operacional é controlada pela variável de ambiente
# CLAUDE_GATEWAY_LOG_LEVEL (debug | info | warn | error; padrão info). debug
# também registra os nomes de claims em cada id_token, para diagnóstico de groups_claim.
# Isso não afeta eventos de auditoria, que são sempre emitidos.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omita o bloco tls ao executar atrás de um ingress que encerra TLS.
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
  # Obrigatório quando o emissor é o servidor da organização Okta, cujos id_tokens
  # podem omitir email e groups; o gateway os preenche a partir de /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta emite groups apenas quando o escopo `groups` é solicitado e o
  # filtro de claim de grupos do aplicativo os permite. A política de contractors abaixo
  # corresponde a groups, então o escopo é solicitado aqui.
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
  # connect_timeout_seconds: 5

# Habilita /v1/organizations/spend_limits (espelha a API Admin do Anthropic)
# e aplicação de gastos por desenvolvedor em /v1/messages. Omita para desabilitar.
# Os limites em si são definidos via API admin, não aqui.
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

# Teste de carga desta implantação sem chamar um provedor de modelo. Nunca em um
# gateway que desenvolvedores usam: cada solicitação recebe uma resposta enlatada.
# load_test_mode:
#   enabled: true
#   # reply_tokens: 750
#   # reply_seconds: 9.5

# Meça em taxas contratadas em vez de preço de lista USD. Requer admin: ou uma
# política managed:. Com managed:, as mesmas taxas também vão para clientes conectados.
# As taxas abaixo são placeholders, não preços de contrato reais.
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

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
        # Restrinja a opção do seletor Padrão aos availableModels em vez de
        # o padrão de tier, para que contractors não recebam um 400 no padrão.
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
  Configurações gerenciadas no lado do cliente
</h2>

Tudo acima configura o servidor gateway. Você aponta máquinas de desenvolvedores para o gateway separadamente, em cada dispositivo, através das [configurações gerenciadas](/docs/pt/managed-settings) do Claude Code. O gateway não pode enviar as chaves de login por si só, porque são elas que dizem ao cliente onde o gateway está.

Para a CLI, defina essas chaves no `managed-settings.json` por SO. As duas chaves de login encaminham cada `/login` do desenvolvedor para seu gateway:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"` mantém a entrega da lista de permissões de saída do Claude Desktop para suas sessões incorporadas do Claude Code funcionando; [Deliver policy to Claude Desktop sessions](/docs/pt/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) explica o mecanismo e onde a aceitação deve estar.

Implante o arquivo `managed-settings.json` em cada dispositivo, normalmente através de sua plataforma MDM. O caminho do arquivo difere por plataforma. Veja [onde cada mecanismo armazena a política](/docs/pt/managed-settings#where-each-mechanism-stores-the-policy).

Por padrão, uma política de registro no Windows ou um plist de preferências gerenciadas no macOS substitui o arquivo `managed-settings.json` em vez de mesclar com ele, exceto pelas [chaves de exceção e verificações entre fontes acima](#precedence-with-other-managed-sources). Todas as três chaves neste trecho seguem a regra de fonte de prioridade mais alta, portanto frotas que entregam política através de Group Policy ou perfis de configuração devem colocar todas as três nesse mecanismo.

Para Claude Desktop, defina a chave `bootstrapUrl` na própria [configuração gerenciada](https://claude.com/docs/third-party/claude-desktop/configuration) do Claude Desktop como `<listen.public_url>/user/bootstrap`. O fluxo de entrada e a política por grupo correspondem aos da CLI uma vez que uma política aceita no servidor com uma chave `desktop`; sem a aceitação, `/user/bootstrap` retorna 404. Veja [Claude Desktop overlay](#claude-desktop-overlay) para a metade do servidor.

Claude Code honra [`forceLoginGatewayUrl`](/docs/pt/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/pt/settings-reference#gatewayinternalnetworks) e o valor `"gateway"` de [`forceLoginMethod`](/docs/pt/settings-reference#forceloginmethod) apenas de uma fonte gerenciada na máquina: `managed-settings.json`, o plist do macOS ou registro HKLM do Windows, ou um auxiliar de política. Um desenvolvedor configurando-os em seu próprio `~/.claude/settings.json` não tem efeito, e tampouco tem efeito configurá-los na carga útil do gateway.

<h2 id="related">
  Relacionado
</h2>

* [Visão geral do gateway de aplicativos Claude](/docs/pt/claude-apps-gateway): quickstart e conexão de desenvolvedor
* [Guia de implantação](/docs/pt/claude-apps-gateway-deploy): configuração de IdP, imagem de contêiner, Kubernetes e Cloud Run e operações
* [Limites de gastos](/docs/pt/claude-apps-gateway-spend-limits): limites por desenvolvedor e a API de Administração
