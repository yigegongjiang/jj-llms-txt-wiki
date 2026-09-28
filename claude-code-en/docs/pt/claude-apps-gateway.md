> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gateway de aplicativos Claude para Amazon Bedrock, Claude Platform on AWS, Google Cloud e Microsoft Foundry

> Execute Claude Code através do Amazon Bedrock, Claude Platform on AWS, Google Cloud ou Microsoft Foundry atrás de um gateway auto-hospedado com sign-in SSO, acesso a modelos por grupo e telemetria OTLP.

<Note>
  O gateway de aplicativos Claude foi projetado para organizações que devem — ou preferem — rotear inferência através de seu próprio provedor de nuvem, por exemplo para atender aos requisitos de [residência de dados](/docs/pt/claude-apps-gateway-deploy#compliance-posture). Se você não tem esse requisito e deseja acesso a outros recursos como provisionamento SCIM ou Claude Code na web e mobile, Claude Enterprise pode ser uma opção melhor. Consulte a página de [disponibilidade de recursos](/docs/pt/feature-availability) para uma comparação completa de todos os métodos de implantação.
</Note>

Claude apps gateway é um serviço auto-hospedado que fica entre os clientes Claude Code dos seus desenvolvedores e seu provedor de modelo. Os desenvolvedores fazem login com seu provedor de identidade corporativo (IdP) em vez de manter chaves de API ou credenciais de nuvem. O gateway mantém a credencial upstream, aplica acesso a modelos e [configurações gerenciadas](/docs/pt/managed-settings) por grupo IdP, e retransmite telemetria de uso para sua própria pilha de observabilidade.

Está incluído no binário `claude`, então o mesmo executável que executa Claude Code em um laptop executa o servidor gateway com `claude gateway --config gateway.yaml`.

Esta página cobre:

* [Por que Claude apps gateway](#why-claude-apps-gateway), o que adiciona em relação a executar o seu próprio, e quando algo mais se encaixa melhor
* Um [guia de início rápido](#quickstart) com [pré-requisitos](#prerequisites) que leva um gateway de zero a um desenvolvedor conectado
* [Conectando desenvolvedores](#connect-developers), incluindo definir a URL do gateway através de configurações gerenciadas
* [Disponibilidade e limitações](#availability-and-limitations) cobrindo quais recursos do Claude Code funcionam através do gateway e o que o servidor suporta

As páginas complementares vão mais fundo. A [referência de configuração](/docs/pt/claude-apps-gateway-config) cobre todas as opções no arquivo YAML que o guia de início rápido escreve, e o [guia de implantação](/docs/pt/claude-apps-gateway-deploy) cobre configuração por IdP, implantação em Kubernetes e Cloud Run, e operações.

<h2 id="why-claude-apps-gateway">
  Por que Claude apps gateway
</h2>

A [visão geral do gateway](/docs/pt/gateways) cobre o que um gateway faz e por que você executaria um. Claude apps gateway é o próprio gateway da Anthropic, integrado ao binário `claude` e testado junto com cada lançamento do Claude Code, então encaminha os cabeçalhos e campos de solicitação que Claude Code envia sem operadores mantendo uma lista de permissões separada. Uma vez implantado, ele oferece:

* **Credenciais**: a chave de API upstream ou credencial de nuvem vive apenas em sua infraestrutura. Os desenvolvedores se autenticam com SSO corporativo e recebem tokens de portador de curta duração, então o offboarding acontece em seu IdP. Desprovisione um usuário e seu acesso ao gateway expira dentro do tempo de vida da sessão, uma hora por padrão.
* **Controle de acesso**: seus grupos IdP mapeiam para listas de permissões de modelos e políticas de [configurações gerenciadas](/docs/pt/managed-settings). O gateway aplica acesso a modelos no lado do servidor, rejeitando solicitações para modelos não concedidos, e seleciona a política de configurações gerenciadas de cada grupo, que a CLI aplica no [nível de configurações gerenciadas](/docs/pt/settings#settings-precedence). Diferentes equipes obtêm diferentes modelos, ferramentas e permissões, e um desenvolvedor não pode substituir o que sua política bloqueia.
* **Entrega de configurações**: o gateway entrega configurações gerenciadas para clientes conectados, assumindo o lugar das [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) do console administrativo claude.ai.
* **Telemetria**: cada destino configurado recebe [métricas do OpenTelemetry Protocol (OTLP)](/docs/pt/monitoring-usage) com contagens de tokens, modelo, identidade do usuário e latência por padrão, com logs e rastreamentos como opt-ins por destino.
* **Roteamento upstream**: os clientes falam a API de Mensagens Anthropic para o gateway, e o gateway traduz para cada upstream, seja Amazon Bedrock, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), Agent Platform do Google Cloud, Microsoft Foundry ou a API Anthropic, com failover entre eles. Você pode alterar regiões, provedores ou ordem de failover sem que os desenvolvedores notem ou reconfiguram.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagrama mostrando clientes Claude Code e abas Chat, Cowork e Code do Claude Desktop conectando via HTTPS com tokens de portador a um gateway de aplicativos Claude auto-hospedado dentro de sua infraestrutura, que faz login dos usuários contra seu IdP, armazena estado de autenticação em PostgreSQL, retransmite telemetria para seu coletor OTLP, e encaminha inferência para Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry ou a API Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  O plano de dados do próprio gateway não envia nada para a infraestrutura Anthropic a menos que a API Anthropic seja um upstream configurado. Você controla para onde a telemetria, logs de auditoria, configurações gerenciadas e a identidade IdP dos seus desenvolvedores vão, e o gateway não envia nenhum deles para Anthropic. Para o tráfego restante que o processo CLI pode enviar e como fechá-lo, consulte [Postura de conformidade](/docs/pt/claude-apps-gateway-deploy#compliance-posture).
</Note>

Para quais recursos do Claude Code funcionam através do gateway e o que o próprio servidor suporta, consulte [Disponibilidade e limitações](#availability-and-limitations) abaixo. Para decisões como custo, bypass, executar múltiplos gateways e plataformas sem servidor, consulte o [guia de implantação](/docs/pt/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Outras implementações de gateway
</h3>

Se você já executa um gateway LLM ou gateway de API que atende às suas necessidades, continue usando-o; [Outros gateways LLM](/docs/pt/llm-gateway) cobre a configuração do Claude Code contra ele.

O [guia de compatibilidade de gateway](/docs/pt/llm-gateway-protocol) documenta o que Claude Code espera de qualquer gateway: os endpoints que chama, os cabeçalhos e campos de corpo para encaminhar, e o que para de funcionar quando são removidos. Um gateway de aplicativos Claude em execução também serve sua própria referência de protocolo em `GET /protocol`, que descreve os endpoints que expõe para clientes Claude Code: sign-in SSO, inferência, entrega de configurações gerenciadas, descoberta de modelos e telemetria. Busque-o com `curl https://claude-gateway.internal.example.com/protocol` de qualquer gateway implantado, como o que o [guia de início rápido](#quickstart) abaixo produz.

Mudanças significativas no protocolo são anunciadas com antecedência, mas compatibilidade retroativa indefinida não é garantida.

<h2 id="quickstart">
  Guia de início rápido
</h2>

Este guia de início rápido percorre o caminho mínimo: registre um cliente OAuth em seu IdP, escreva um `gateway.yaml`, execute o gateway junto com Postgres usando Docker Compose, e verifique o sign-in de ponta a ponta. Usa um upstream Amazon Bedrock; Claude Platform on AWS, Agent Platform do Google Cloud, Microsoft Foundry e a API Anthropic são igualmente suportados trocando o bloco `upstreams` conforme mostrado na [referência de configuração](/docs/pt/claude-apps-gateway-config#upstreams). No final você tem um gateway que um desenvolvedor pode fazer `/login`.

<Note>
  **Implante em sua rede privada.** Claude Code só se conecta a um gateway cujo endereço é privado. Esta é uma proteção de segurança, porque um gateway confiável pode enviar configurações que executam comandos em máquinas de desenvolvedores. Coloque o gateway atrás de um balanceador de carga interno ou VPN e dê a ele um nome de host que resolve apenas para IPs privados. Se sua rede interna for numerada a partir do espaço IPv4 público que sua organização possui, consulte [Permitir um gateway em espaço de endereço público que você possui](#allow-a-gateway-on-public-address-space-you-own).
</Note>

<h3 id="prerequisites">
  Pré-requisitos
</h3>

Tenha estes em vigor antes de começar:

| Você precisa                                 | Detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 ou posterior            | O subcomando `claude gateway` e o fluxo de sign-in do gateway são enviados na v2.1.195. Compilações públicas anteriores não as incluem. Tanto a máquina executando o servidor gateway quanto a máquina de cada desenvolvedor devem estar na v2.1.195 ou posterior; execute `claude update` para obter a versão mais recente. O [upstream Claude Platform on AWS](/docs/pt/claude-apps-gateway-config#claude-platform-on-aws) requer Claude Code v2.1.198 ou posterior no servidor gateway.                                                                                                                                                                                                                                                                                                                                                                                                         |
| Provedor de identidade OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak ou Dex, ou qualquer outro IdP compatível com OIDC, como PingFederate. O gateway executa descoberta OIDC padrão e o fluxo de código de autorização contra ele. SAML e LDAP não são suportados.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| PostgreSQL 14 ou posterior                   | Faz backup do fluxo de sign-in do dispositivo, onde o callback do navegador escreve e a CLI de polling lê, além de contadores de limite de taxa. Qualquer Postgres gerenciado funciona, incluindo o menor nível. Sem limites de gastos configurados, o gateway armazena alguns KB de estado de autenticação de curta duração; com [limites de gastos](/docs/pt/claude-apps-gateway-spend-limits), também mantém tabelas de gastos, auditoria e identidade duráveis que devem ser feitas backup. TLS via `?sslmode=require` é recomendado.                                                                                                                                                                                                                                                                                                                                                          |
| Upstream de modelo                           | Credenciais do Amazon Bedrock, credenciais do Claude Platform on AWS, credenciais do Google Cloud, um recurso Microsoft Foundry ou uma chave de API Anthropic. Múltiplos upstreams são suportados com failover.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| HTTPS                                        | O gateway deve ser acessível via `https://` de laptops de desenvolvedores e de qualquer navegador usado para sign-in; o gateway serve a página de verificação do dispositivo no mesmo listener. Forneça um certificado TLS via `listen.tls` ou execute atrás de um ingress que termina TLS e defina `listen.public_url` para a origem externa em ambos os casos. Uma origem `http://` simples é aceita apenas quando o host do gateway é loopback: `localhost`, `127.0.0.1` ou `::1`.                                                                                                                                                                                                                                                                                                                                                                                                         |
| Endereço de rede privada                     | Em `/login`, Claude Code requer que o nome de host ou endereço IP do gateway resolva apenas para endereços privados: RFC 1918, link-local, CGNAT `100.64.0.0/10`, ULA IPv6 `fc00::/7` ou loopback. Para um gateway que você hospeda, qualquer endereço público fora de um bloco que você declara é rejeitado; consulte o [modelo de ameaça](/docs/pt/claude-apps-gateway-deploy#threat-model-summary) no guia de implantação. Se máquinas de desenvolvedores rotear HTTPS através de um proxy corporativo, o sign-in também requer que o host proxy resolva para endereços privados; se não resolver, adicione o host do gateway a `NO_PROXY` para que a CLI se conecte diretamente. Se sua rede interna for numerada a partir do espaço IPv4 público que sua organização possui, [declare esses blocos](#allow-a-gateway-on-public-address-space-you-own) para que `/login` aceite um gateway lá. |
| Runtime Linux                                | O servidor gateway é executado apenas no binário Linux nativo. macOS funciona para desenvolvimento local. Windows não é suportado como plataforma de servidor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h3 id="steps">
  Etapas
</h3>

<Steps>
  <Step title="Registre um cliente OAuth em seu IdP">
    Decida o nome de host do gateway primeiro, porque o URI de redirecionamento deve corresponder a ele. Crie um novo aplicativo web OIDC e defina o URI de redirecionamento para `https://claude-gateway.<seu-domínio>/oauth/callback`, onde o host é o mesmo valor que você define como [`listen.public_url`](/docs/pt/claude-apps-gateway-config#listen) na etapa 3. Anote o `client_id` e `client_secret`. As instruções por IdP estão em [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Provisione um banco de dados PostgreSQL">
    Qualquer Postgres 14 ou posterior funciona, incluindo o menor nível gerenciado. O gateway executa suas próprias migrações de esquema na inicialização, então o usuário do banco de dados precisa de direitos para criar e alterar tabelas; consulte [`store`](/docs/pt/claude-apps-gateway-config#store).
  </Step>

  <Step title="Escreva gateway.yaml">
    Os segredos são lidos via expansão `${ENV_VAR}` para que o arquivo em si possa viver no controle de versão. Use um nome de host `public_url` que resolva para um IP privado em sua rede, porque `/login` rejeita endereços públicos. A configuração mínima tem cinco seções, e todos os outros campos têm um padrão:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Obrigatório a menos que o host seja um endereço loopback. Usado para o IdP
      # redirect_uri e o documento de descoberta.
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # deve servir /.well-known/openid-configuration
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # rejeitar id_tokens fora de sua organização
      userinfo_fallback: true                  # para IdPs cujo id_token omite email/grupos; inofensivo caso contrário

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # também limita a latência de revogação no desprovisionamento de IdP

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # adicione ?sslmode=require para Postgres gerenciado

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # vazio: cadeia de credencial padrão da AWS
    # (IRSA, função de tarefa EC2/ECS, variáveis de ambiente, ~/.aws)

    # Os modelos são traduzidos por upstream automaticamente. O catálogo integrado
    # mapeia claude-opus-4-8 para us.anthropic.claude-opus-4-8 e assim por diante para cada
    # modelo Claude suportado pelo Bedrock. Defina como false e adicione uma lista `models:` para
    # expor apenas modelos específicos.
    auto_include_builtin_models: true
    ```

    Esta configuração é suficiente para um loop de sign-in funcionando com o catálogo de modelos Bedrock padrão. Uma vez em execução, adicione RBAC por grupo e configurações gerenciadas via [`managed.policies`](/docs/pt/claude-apps-gateway-config#managed), fan-out de telemetria via [`telemetry`](/docs/pt/claude-apps-gateway-config#telemetry), e failover multi-upstream, ARNs de throughput provisionado ou regiões não-US via [`models`](/docs/pt/claude-apps-gateway-config#models).

    <Note>
      O upstream Amazon Bedrock precisa de um principal AWS com `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` nos ARNs `inference-profile/us.anthropic.*` e nos ARNs `foundation-model/anthropic.*` subjacentes. Ele também precisa do formulário de caso de uso único da Anthropic enviado para a conta a partir do catálogo de modelos do console Bedrock.

      Forneça a credencial com IRSA no EKS, uma função de tarefa ECS ou um perfil de instância EC2 em vez de chaves estáticas. A [referência `upstreams`](/docs/pt/claude-apps-gateway-config#upstreams) tem os detalhes completos do IAM, a matriz de credencial entre nuvens e os blocos `auth` para os outros provedores.
    </Note>
  </Step>

  <Step title="Execute-o">
    Construa uma imagem de contêiner em torno do binário `claude` que atenda aos [requisitos de imagem](/docs/pt/claude-apps-gateway-deploy#container-image), então execute-a junto com Postgres. O arquivo Compose referencia a imagem como `registry.example.com/claude-gateway:2.1.198`; substitua seu próprio registro e tag de imagem:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: registry.example.com/claude-gateway:2.1.198
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # Credenciais AWS: em produção, omita estas e use uma função de instância.
          # Para teste local do Compose, passe as suas próprias:
          AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
          AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
          AWS_SESSION_TOKEN: ${AWS_SESSION_TOKEN}
        depends_on:
          postgres:
            condition: service_healthy
      postgres:
        image: postgres:16-alpine
        environment: { POSTGRES_USER: gw, POSTGRES_PASSWORD: pw, POSTGRES_DB: gateway }
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U gw"]
          interval: 5s
        volumes: ["pgdata:/var/lib/postgresql/data"]
    volumes: { pgdata: }
    ```

    O gateway é um único binário Linux que lê a configuração, se conecta ao Postgres e aplica suas migrações de esquema, executa descoberta OIDC contra seu IdP, constrói clientes upstream e começa a escutar. A inicialização é fail-closed para a configuração, a conexão Postgres, descoberta OIDC e construção de cliente upstream. Se qualquer um desses for inacessível ou mal configurado, o gateway sai com um erro em vez de servir tráfego em um estado degradado.

    Uma inicialização bem-sucedida não valida o caminho de inferência, porque credenciais de instância Bedrock e Agent Platform resolvem na primeira solicitação, não na inicialização.

    Observe stderr para a sequência de inicialização. As linhas de log usam o formato `[gateway] <timestamp> <level> <message>`, eventos de auditoria são JSON de linha única com um campo `evt`, e um banner de inicialização, omitido abaixo, é impresso entre as linhas de migração e escuta. Um banco de dados novo imprime uma linha `migration N applied` por migração de esquema; um banco de dados já migrado não imprime nenhuma. Você deve ver, em ordem:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.395Z info waiting for migration lock (another replica may be migrating; check pg_locks for key 6775156 if this persists)
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    …
    [gateway] 2026-06-10T17:03:21.431Z info migration 6 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    O gateway também registra um aviso de que `access_control.allow_cidrs` está vazio. Isso é esperado aqui, porque nada limita quais endereços de cliente o gateway serve até que você defina uma lista de permissões. A [referência `access_control`](/docs/pt/claude-apps-gateway-config#http-tuning) tem os intervalos recomendados.

    Se a inicialização sair antes da linha `claude gateway listening on`, a última linha de stderr nomeia o problema:

    * um Postgres inacessível
    * uma função Postgres sem permissão DDL
    * um documento de descoberta OIDC inacessível ou inválido
    * uma violação de esquema de configuração com o caminho de campo ofensivo

    Corrija-o e reinicie.

    Se você já tem um ingress que termina TLS, pule o Compose e execute o binário diretamente com `claude gateway --config gateway.yaml`. Defina `public_url` para a origem do ingress e vincule `listen` a um endereço loopback ou interno do cluster.
  </Step>

  <Step title="Verifique a superfície de autenticação">
    Três verificações confirmam que o gateway pode autenticar um usuário real antes de você compartilhá-lo com um desenvolvedor.

    Os exemplos usam a URL pública do gateway; para a configuração local do Compose sem um ingress, substitua `http://localhost:8080` nas duas primeiras verificações. A terceira verificação abre `verification_uri_complete`, que é construída a partir de `public_url`, então para Compose local defina `public_url: http://localhost:8080` em `gateway.yaml` e adicione `http://localhost:8080/oauth/callback` como um segundo URI de redirecionamento no cliente OAuth da etapa 1, porque o gateway constrói o `redirect_uri` do IdP a partir de `public_url`. O link de verificação então abre em seu navegador local.

    No Windows PowerShell, execute `curl.exe`; o `curl` simples é um alias para `Invoke-WebRequest` e rejeita esses sinalizadores.

    Primeiro, busque o documento de descoberta, que confirma que o gateway está ativo, a configuração é válida e todas as verificações de inicialização passaram:

    ```bash theme={null}
    curl -s https://claude-gateway.internal.example.com/.well-known/oauth-authorization-server | jq
    ```

    ```json theme={null}
    {
      "issuer": "https://claude-gateway.internal.example.com",
      "device_authorization_endpoint": "…/oauth/device_authorization",
      "token_endpoint": "…/oauth/token",
      "grant_types_supported": ["urn:ietf:params:oauth:grant-type:device_code", "refresh_token"]
    }
    ```

    A resposta inclui campos adicionais, como `response_types_supported` e `scopes_supported`.

    Segundo, solicite uma autorização de dispositivo, que confirma que o fluxo de sign-in do dispositivo funciona e Postgres é acessível e gravável:

    ```bash theme={null}
    curl -s -X POST https://claude-gateway.internal.example.com/oauth/device_authorization | jq
    ```

    ```json theme={null}
    {
      "device_code": "…",
      "user_code": "WDJB-MJHT",
      "verification_uri": "https://claude-gateway.internal.example.com/device",
      "verification_uri_complete": "https://claude-gateway.internal.example.com/device?user_code=WDJB-MJHT",
      "expires_in": 600,
      "interval": 5
    }
    ```

    Terceiro, teste a perna do navegador abrindo `verification_uri_complete` em um navegador e confirmando o código. Você deve ser redirecionado para a página de sign-in do seu IdP e, após fazer login, voltar ao gateway com uma confirmação de sign-in.

    Use a primeira verificação que falha para localizar o problema:

    * **Primeira verificação falha**: a inicialização não foi concluída; verifique stderr
    * **Segunda verificação falha**: Postgres não é acessível do gateway ou a função não pode escrever; verifique a string de conexão e as concessões
    * **Terceira verificação não alcança o IdP**: verifique que o URI de redirecionamento do IdP corresponde exatamente a `https://<gateway>/oauth/callback`
    * **Terceira verificação alcança o IdP mas volta com um erro**: leia o log de auditoria do gateway, que registra cada rejeição de autenticação com o motivo, como `email domain not allowed`
  </Step>

  <Step title="Faça login de um desenvolvedor">
    Esta última etapa acontece em uma máquina de desenvolvedor, não no servidor. Defina `forceLoginMethod` como `"gateway"` e `forceLoginGatewayUrl` como a `public_url` do seu gateway no [arquivo de configurações gerenciadas](/docs/pt/managed-settings#delivery-mechanisms) dessa máquina, então execute `/login`, pressione Enter na tela **Cloud gateway** e conclua o sign-in do navegador. [Defina a URL do gateway](#set-the-gateway-url) abaixo cobre a distribuição de ambas as chaves em cada máquina de desenvolvedor.
  </Step>
</Steps>

<h2 id="connect-developers">
  Conectar desenvolvedores
</h2>

Os desenvolvedores se conectam de seus próprios laptops com um sign-in de navegador, usando sua conta de trabalho corporativa. Eles não precisam de uma conta claude.ai, uma chave de API ou uma assinatura, porque as solicitações para o modelo passam pelo gateway usando a credencial upstream da organização. A conexão é orientada pelas [configurações gerenciadas no lado do cliente](/docs/pt/claude-apps-gateway-config#client-side-managed-settings) que você envia via MDM, então não há configuração manual no lado do desenvolvedor; esta seção cobre o que o administrador configura.

A CLI coloca a impressão digital do certificado TLS folha do gateway na primeira conexão e a fixa por nome de host. Ela verifica esse pino novamente durante o sign-in, em atualizações de sessão silenciosas e em buscas de configurações gerenciadas, enquanto solicitações de inferência usam validação TLS padrão sem o pino. Solicitações roteadas através de um proxy HTTPS pulam a verificação de pino, então adicione o host do gateway a `NO_PROXY` para mantê-las diretas.

Publique a impressão digital SHA-256 esperada junto com a URL do gateway para que os desenvolvedores tenham algo para comparar. O prompt `/login` mostra os primeiros 16 caracteres da impressão digital como hexadecimal minúsculo sem dois-pontos. Para imprimir a impressão digital completa nesse formato a partir do arquivo de certificado, execute:

```bash theme={null}
openssl x509 -noout -fingerprint -sha256 -in cert.pem | cut -d= -f2 | tr -d : | tr 'A-F' 'a-f'
```

Quando o certificado é rotacionado, cada desenvolvedor vê o prompt de confiança novamente, então trate rotações como um evento planejado e republique a impressão digital. Se sua política de gateway incluir [configurações que precisam de aprovação](/docs/pt/server-managed-settings#security-approval-dialogs), o desenvolvedor também vê esse diálogo de aprovação novamente após aceitar o novo certificado, porque Claude Code vincula [memória de aprovação](/docs/pt/server-managed-settings#approval-memory) ao certificado fixado.

Um gateway pode retornar o campo `email` opcional em sua resposta de token para nomear a conta que um sign-in usou. Quando faz isso, o desenvolvedor confirma a conta antes de Claude Code salvar a credencial. Após um sign-in confirmado, `/status` mostra a conta.

A confirmação requer Claude Code v2.1.275 ou posterior na máquina do desenvolvedor; um cliente abaixo dessa versão ignora o campo. O servidor do gateway no binário `claude` não retorna o campo, então seus sign-ins são concluídos sem a confirmação.

Uma vez que o desenvolvedor faz login, o [seletor de modelo](/docs/pt/model-config) mostra os modelos na lista de permissões `availableModels` do desenvolvedor. As configurações gerenciadas se aplicam na inicialização e atualizam a cada hora, e a telemetria é roteada para seu coletor.

As sessões são atualizadas silenciosamente antes da expiração de `ttl_hours`. Quando uma atualização falha após desprovisionamento de IdP, Claude Code solicita que o desenvolvedor faça login novamente.

<h3 id="set-the-gateway-url">
  Defina a URL do gateway
</h3>

Três chaves vão no arquivo de [configurações gerenciadas](/docs/pt/managed-settings#delivery-mechanisms) por SO que você implanta via MDM ou diretamente no disco. `forceLoginMethod` e `forceLoginGatewayUrl` abrem `/login` diretamente na tela **Cloud gateway** com a URL preenchida, e `parentSettingsBehavior: "merge"` permite que Claude Desktop entregue a lista de permissões de egresso do gateway para as sessões Claude Code que ele inicia, explicado em [Entregar política para sessões Claude Desktop](#deliver-policy-to-claude-desktop-sessions):

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

O desenvolvedor pressiona Enter para se conectar. O [prompt de impressão digital TLS de primeira conexão](#connect-developers) ainda aparece. Uma vez que o arquivo está em uma máquina, um desenvolvedor que não completou o sign-in do gateway vê uma das mensagens descritas em [A política do administrador requer um sign-in Cloud gateway](/docs/pt/errors#administrator-policy-requires-a-cloud-gateway-sign-in). Desenvolvedores que selecionam um provedor de nuvem através de uma variável de ambiente como `CLAUDE_CODE_USE_BEDROCK` não precisam do sign-in do gateway.

Um desenvolvedor não pode configurar isso manualmente. O seletor de login não tem opção de gateway, e `forceLoginGatewayUrl` é ignorado nos arquivos de configurações próprias de um desenvolvedor. `forceLoginMethod` sozinho, sem uma URL, deixa o desenvolvedor em uma mensagem "Entre em contato com seu administrador de TI". As chaves de login pertencem ao arquivo que você envia para máquinas, não ao bloco `managed.policies[].cli` do gateway, que só alcança clientes que já estão conectados.

<h3 id="allow-a-gateway-on-public-address-space-you-own">
  Permitir um gateway em espaço de endereço público que você possui
</h3>

Algumas organizações numerem sua rede interna a partir de um bloco IPv4 público que possuem, como o espaço de endereço próprio de uma operadora ou um `/8` legado, então seu gateway não pode ter um endereço privado. Liste esses blocos na configuração gerenciada `gatewayInternalNetworks`. `/login` então aceita um gateway dentro de um bloco listado quando a máquina do desenvolvedor se conecta a ele a partir de um endereço dentro do mesmo bloco. Isso requer Claude Code v2.1.268 ou posterior na máquina do desenvolvedor; versões anteriores ignoram a chave e aplicam a regra de endereço privado.

<Warning>
  `gatewayInternalNetworks` é para redes internas que acontecem de ser numeradas a partir de espaço de endereço público. Não torna seguro expor um gateway para a internet: um gateway confiável pode enviar configurações que executam comandos em máquinas de desenvolvedores.

  Mantenha o gateway inacessível de fora de sua rede com suas regras de firewall ou balanceador de carga. Defina o [`access_control.allow_cidrs`](/docs/pt/claude-apps-gateway-config#http-tuning) do gateway para os mesmos blocos que você declara aqui, então o gateway em si recusa clientes de qualquer outro lugar. Atrás de um balanceador de carga ou ingress, defina `listen.trusted_proxies` para esse front end também, porque o gateway de outra forma corresponde `allow_cidrs` contra o próprio endereço do front end em vez do desenvolvedor.
</Warning>

Adicione a chave à mesma fonte de configurações gerenciadas que as chaves de login: o arquivo de configurações gerenciadas, perfil MDM ou política de registro. Claude Code a ignora em configurações de usuário, projeto e gerenciadas pelo servidor.

Este exemplo declara um bloco. Substitua `203.0.113.0/24` pelo seu próprio bloco. É um intervalo de documentação, e Claude Code recusa aqueles.

```json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Claude Code valida a lista em `/login` antes de contatar qualquer gateway:

* Cada entrada é um bloco IPv4 escrito como seu primeiro endereço e um prefixo de `/8` a `/32`.
* A lista contém no máximo quatro blocos, e nenhum dois se sobrepõem.
* Nenhum bloco se sobrepõe ao espaço de endereço privado: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `127.0.0.0/8`, `169.254.0.0/16` e `100.64.0.0/10`. `/login` já aceita um gateway lá sem essa chave.
* Nenhum bloco se sobrepõe ao espaço que nunca é a rede de uma organização: `198.18.0.0/15` e `192.0.0.0/24`, que clientes VPN e NAT64 mantêm como endereços locais; os intervalos de documentação `192.0.2.0/24`, `198.51.100.0/24` e `203.0.113.0/24`; e os intervalos reservados `0.0.0.0/8`, `192.88.99.0/24` e multicast `224.0.0.0/4`. Você pode declarar blocos dentro de `240.0.0.0/4`, que algumas redes grandes usam como espaço unicast interno.

Blocos de `managed-settings.json` e seus arquivos drop-in `managed-settings.d/` se combinam em uma lista, e esses limites se aplicam à lista combinada. Para estreitar um bloco, substitua sua entrada em vez de adicionar uma segunda, sobreposta em um drop-in; `/login` recusa a sobreposição.

Se uma entrada quebra uma regra, ou o valor não é uma lista de strings, Claude Code recusa cada novo sign-in de gateway nessa máquina e nomeia o problema na mensagem. O sign-in para um gateway em um endereço privado também falha, e sign-ins existentes continuam funcionando. Tente o valor em uma máquina antes de implantá-lo. Claude Code também lista um valor digitado incorretamente entre as [configurações gerenciadas inválidas que relata](/docs/pt/managed-settings#keys-that-fail-closed).

Com uma lista válida, `/login` aplica três verificações a um gateway cujo endereço está dentro de um bloco listado:

* Cada endereço para o qual o nome de host do gateway é resolvido está dentro daquele bloco. Claude Code recusa um nome que também tem registros fora dele, endereços privados e IPv6 incluídos.
* A máquina do desenvolvedor se conecta de dentro do mesmo bloco. Claude Code recusa uma máquina atrás de NAT, dentro de um container ou WSL2, ou em uma VPN cujo pool de endereços fica fora do bloco, e nomeia o endereço a partir do qual a máquina se conectou.
* A conexão é direta. Se `HTTPS_PROXY` se aplica ao host do gateway, `/login` recusa e nomeia a entrada `NO_PROXY` a adicionar.

Quando todos os três passam, o [prompt de confiança](#connect-developers) adiciona uma linha nomeando o endereço da máquina, o endereço do gateway e o bloco declarado que contém ambos.

A chave não muda nada para outros gateways: o sign-in para um em um endereço privado funciona como antes, e o sign-in para um em um endereço público fora de cada bloco listado é recusado como antes.

Um bloco declarado estreita quem pode fazer sign-in mas não prova onde uma máquina está, então declare apenas espaço de endereço que sua organização controla. Um bloco compartilhado com outros tenants, como um intervalo público de um provedor de nuvem, deixa qualquer um nele passar na mesma verificação.

<h3 id="deliver-policy-to-claude-desktop-sessions">
  Entregar política para sessões Claude Desktop
</h3>

Claude Desktop executa suas abas Cowork e Code, mais a aba Chat quando você a habilita, em sessões Claude Code incorporadas e envia suas solicitações de modelo através do gateway. Ele passa política para cada uma dessas sessões, construída a partir da configuração que o gateway serve em `/user/bootstrap`: a lista de permissões de modelo, ferramentas desabilitadas e lista de permissões de egresso derivada do bloco `cli` da política correspondente, mais a [sobreposição `desktop`](/docs/pt/claude-apps-gateway-config#claude-desktop-overlay).

Outras chaves `cli`, como hooks, `env` e regras de permissão com escopo como `Bash(npm *)`, alcançam apenas clientes que fazem login através de `/login`. Claude Desktop lê a URL do gateway de sua própria configuração gerenciada e faz login com seu próprio fluxo, separado das chaves `forceLoginMethod` e `forceLoginGatewayUrl` em [Defina a URL do gateway](#set-the-gateway-url).

As configurações passadas por um processo de inicialização são configurações pai. Claude Code ignora configurações pai em qualquer máquina que tenha uma fonte gerenciada implantada por administrador, a menos que a [fonte que entrega a política](/docs/pt/managed-settings#which-managed-source-claude-code-uses) defina `parentSettingsBehavior: "merge"`.

<h4 id="which-machines-need-the-opt-in">
  Quais máquinas precisam da aceitação
</h4>

Máquinas que executam apenas Claude Desktop precisam dela. Claude Desktop aplica a lista de modelos e a lista de ferramentas desabilitadas às sessões incorporadas em si, mas a lista de permissões de egresso as alcança apenas como configurações pai, na forma de regras de domínio `WebFetch` e regras de rede sandbox. Sem a aceitação, essas sessões são executadas sem a restrição de egresso, e nada o avisa. O gateway ainda rejeita solicitações de inferência para modelos que a política não concede.

Máquinas onde desenvolvedores fazem login através de `/login` não precisam dela; cada sessão Claude Code busca sua política do gateway.

Frotas cujo [`policyHelper`](/docs/pt/settings-reference#policyhelper) fornece configurações gerenciadas não podem usá-la: Claude Code nunca mescla configurações pai nessas frotas, porque lê configurações gerenciadas apenas da saída do helper.

<h4 id="set-the-opt-in">
  Defina a aceitação
</h4>

Implante o trecho de configurações gerenciadas de [Defina a URL do gateway](#set-the-gateway-url), espelhe-o para qualquer fonte no lado do cliente que supere o arquivo, então verifique.

<Steps>
  <Step title="Implante a aceitação no arquivo de configurações gerenciadas">
    O [trecho acima](#set-the-gateway-url) já inclui `parentSettingsBehavior: "merge"`, então o arquivo que você envia para máquinas o carrega.
  </Step>

  <Step title="Espelhe o trecho para qualquer fonte que supere o arquivo">
    Claude Code lê `parentSettingsBehavior` apenas da [fonte selecionada](/docs/pt/managed-settings#which-managed-source-claude-code-uses). Adicionar qualquer chave de política a uma fonte pode fazer dessa fonte a selecionada, então em uma fonte no lado do cliente, espelhe o trecho inteiro em vez de apenas `parentSettingsBehavior`. [Configurações gerenciadas no lado do cliente](/docs/pt/claude-apps-gateway-config#client-side-managed-settings) cobre frotas que entregam política através de Group Policy ou perfis de configuração. Um plist de preferências gerenciadas no macOS ou uma política HKLM no Windows supera o arquivo `managed-settings.json`, e as configurações gerenciadas remotas do gateway superam ambas, então em máquinas que fazem login no gateway, também defina `parentSettingsBehavior` no bloco [`cli`](/docs/pt/claude-apps-gateway-config#managed) da política do gateway.
  </Step>

  <Step title="Verifique qual fonte está selecionada">
    Em uma máquina que executa apenas Claude Desktop, chame o [`resolveSettings()`](/docs/pt/agent-sdk/typescript#resolvesettings) do Agent SDK e leia `policyOrigin` na entrada `managed` em sua lista `sources`. O valor nomeia a fonte selecionada no lado do cliente, `plist`, `hklm` ou `file`, que é a fonte que deve carregar o trecho. As sessões incorporadas do Claude Desktop não buscam a política do gateway, então o bloco `cli` do gateway nunca conta como a fonte selecionada para elas.
  </Step>
</Steps>

<h3 id="restrict-parent-settings">
  Restringir configurações pai
</h3>

Uma vez que você implante `parentSettingsBehavior: "merge"`, qualquer processo host que inicie Claude Code pode fornecer configurações pai, não apenas Claude Desktop mas também um aplicativo Agent SDK ou uma extensão IDE.

Claude Code filtra configurações pai contra uma lista de permissões de chaves restritivas, mas algumas chaves permitidas podem conceder acesso em vez de restringir. A menos que você defina os bloqueios `allowManaged*Only`, regras de permissão de acesso e listas de permissões de sandbox fornecidas pelo host ainda se aplicam. As regras de negação e pergunta da sua política permanecem em vigor de qualquer forma; [elas são avaliadas antes de qualquer regra de acesso](/docs/pt/permissions#manage-permissions).

Claude Code encaminha entradas [`sandbox.credentials`](/docs/pt/settings-reference#sandbox-credentials) fornecidas pelo pai em forma reduzida:

* **Entradas `deny`**: encaminhadas apenas com seu `path` ou `name` e o modo.
* **Entradas de arquivo com [`mode: mask`](/docs/pt/sandboxing#mask-credential-files)**: encaminhadas apenas com sentinela, como uma máscara de arquivo inteiro cujo `injectHosts` é a lista vazia, então o proxy nunca substitui o valor real por uma entrada fornecida pelo pai em qualquer plataforma. Todos os campos de mascaramento estruturado também são descartados, então um padrão de extração fornecido pelo pai não pode deslocar uma máscara mais rigorosa que outra fonte define para o mesmo caminho.
* **Entradas `envVars` com `mode: mask`**: não encaminhadas. `deny` é a única restrição que o canal pai pode expressar através de entradas `envVars`.
* **[`awsPairs` e `sigv4`](/docs/pt/sandboxing#re-sign-aws-requests)**: encaminhadas apenas restrição. De `sigv4`, apenas valores `deny` são mantidos, e um pai que define um bloco `sigv4` em tudo fixa todas as três formas de solicitação, `streaming`, `presigned` e `sigv4a`, para `deny`. Um par `awsPairs` nunca é encaminhado em uma forma que possa re-assinar; um par que nomeia uma das variáveis AWS convencionais é substituído por uma entrada inerte que mantém o emparelhamento automático de `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN` suprimido.

<h4 id="deploy-the-locks">
  Implante os bloqueios
</h4>

Para manter configurações pai tão próximas de apenas restrição quanto o filtro suporta, adicione todos os cinco bloqueios `allowManaged*Only` e as listas de permissões que eles governam às mesmas fontes que a aceitação de mesclagem:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge",
  "allowManagedPermissionRulesOnly": true,
  "allowManagedMcpServersOnly": true,
  "allowManagedHooksOnly": true,
  "allowedMcpServers": [{ "serverUrl": "https://mcp.internal.example.com/*" }],
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    },
    "filesystem": {
      "allowManagedReadPathsOnly": true,
      "denyRead": ["~/"],
      "allowRead": ["~/projects"]
    }
  }
}
```

Uma política do SO, como uma política de registro HKLM ou um plist de preferências gerenciadas, supera esse arquivo, então entregue o trecho inteiro através dela em vez do arquivo. As configurações gerenciadas remotas do gateway superam as fontes de política do SO e arquivo mas alcançam apenas clientes conectados. Espelhe os bloqueios, as listas de permissões e a aceitação de mesclagem no bloco [`cli`](/docs/pt/claude-apps-gateway-config#managed) da política e mantenha esse arquivo implantado, porque máquinas que nunca se conectam, incluindo aquelas que executam apenas Claude Desktop, obtêm sua política apenas do arquivo.

<h4 id="lock-behavior-across-sources">
  Comportamento de bloqueio entre fontes
</h4>

Definir um bloqueio não restringe os outros; cada chave é documentada na [referência de configurações](/docs/pt/settings-reference#all-settings). De uma fonte de administrador abaixo do vencedor, os dois bloqueios de sandbox ainda se aplicam, e `allowManagedPermissionRulesOnly` ainda bloqueia regras de acesso fornecidas pelo pai e `additionalDirectories`. No Claude Code v2.1.273 ou posterior, o bloqueio de servidor MCP também se aplica de uma fonte abaixo do vencedor, e enquanto estiver ativado, a lista gerenciada `allowedMcpServers` vem da fonte de administrador de prioridade mais alta que define uma.

O bloqueio de hooks e o efeito de `allowManagedPermissionRulesOnly` nas regras próprias do desenvolvedor precisam da fonte vencedora por padrão; sob a aceitação de mesclagem `managedSourcesBehavior` em [como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources), Claude Code aplica o valor mais rigoroso que qualquer fonte define para cada bloqueio. Em frotas [`policyHelper`](/docs/pt/settings-reference#policyhelper), Claude Code lê os bloqueios apenas da saída do helper.

Cada bloqueio faz Claude Code ignorar as entradas próprias do desenvolvedor para essa configuração, então inclua as listas de permissões da sua organização ao lado dos bloqueios:

* **Domínios de rede**: bloquear com uma lista de domínios gerenciados vazia bloqueia todo o tráfego de saída em sandbox.
* **Servidores MCP**: bloquear sem `allowedMcpServers` em qualquer fonte de administrador ou nas configurações fornecidas pelo pai carrega cada servidor que `deniedMcpServers` não bloqueia.
* **Caminhos de leitura**: entradas `allowRead` apenas re-permitem caminhos dentro de regiões `denyRead`, então emparelhe-as com um `denyRead` gerenciado.

<h4 id="settings-the-locks-don’t-cover">
  Configurações que os bloqueios não cobrem
</h4>

Seis configurações fornecidas pelo pai passam pelo filtro mesmo com todos os cinco bloqueios definidos. Sob a configuração padrão de primeiro vencedor, o valor de administrador que bloqueia o do pai é aquele na fonte de administrador de prioridade mais alta, exceto para `allowedMcpServers` enquanto o [bloqueio de servidor MCP](#lock-behavior-across-sources) está ativado. Sob a aceitação de mesclagem `managedSourcesBehavior`, [como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) diz qual valor da fonte se aplica em vez disso.

* **`forceLoginOrgUUID`**: Claude Code honra um valor fornecido pelo pai quando a fonte de administrador de prioridade mais alta não define um UUID de organização. O sign-in do gateway não verifica essa chave, então importa apenas para frotas que também usam logins Anthropic de primeira parte. Um UUID de organização na fonte de administrador de prioridade mais alta bloqueia o valor do pai e é aquele que Claude Code aplica, então defina `forceLoginOrgUUID` lá.
* **`allowedMcpServers`**: Claude Code honra uma lista de permissões fornecida pelo pai quando nenhuma lista de administrador está em vigor. `allowManagedMcpServersOnly` não a bloqueia, porque o bloqueio aplica qualquer lista que vença como o valor gerenciado, incluindo uma lista fornecida pelo pai quando nenhuma fonte de administrador fornece uma lista. Uma lista na fonte de administrador de prioridade mais alta bloqueia a do pai e é a lista que Claude Code aplica, então defina `allowedMcpServers` lá, ao lado do bloqueio. Antes da v2.1.223, um valor para qualquer chave em qualquer fonte de administrador bloqueava a do pai.
* **`availableModels`**: Claude Code honra uma lista de modelos fornecida pelo pai quando a fonte gerenciada vencedora não define uma. Se sua frota restringe modelos, defina `availableModels` na fonte vencedora.
* **`strictKnownMarketplaces`**: Claude Code honra uma lista de permissões de marketplace de plugin fornecida pelo pai quando a fonte gerenciada vencedora não define uma. Se sua frota restringe marketplaces, defina `strictKnownMarketplaces` na fonte vencedora. Requer Claude Code v2.1.282 ou posterior.
* **`blockedMarketplaces`**: uma lista de bloqueio de marketplace fornecida pelo pai passa e adiciona a qualquer lista de bloqueio que uma fonte gerenciada define, já que uma lista de bloqueio pode apenas restringir ainda mais. Requer Claude Code v2.1.282 ou posterior.
* **`strictPluginOnlyCustomization`**: essa chave passa pelo filtro independentemente de qualquer bloqueio, e faz Claude Code ignorar a customização própria do desenvolvedor, incluindo hooks protetores. Nenhum bloqueio a bloqueia.

<h3 id="connect-claude-desktop">
  Conectar Claude Desktop
</h3>

[Claude Desktop](/docs/pt/desktop) se conecta ao mesmo gateway através de uma chave MDM diferente: defina `bootstrapUrl` na [configuração gerenciada](https://claude.com/docs/third-party/claude-desktop/configuration) do Claude Desktop para `<listen.public_url>/user/bootstrap`, e opte pela política do usuário com uma chave `desktop`. [Sobreposição Claude Desktop](/docs/pt/claude-apps-gateway-config#claude-desktop-overlay) cobre ambas as metades. Requer Claude Code v2.1.203 ou posterior no servidor do gateway.

Claude Desktop faz o desenvolvedor fazer login através do provedor de identidade do gateway com o mesmo passo de SSO do navegador, então busca sua configuração do gateway em vez de da Anthropic. O acesso a modelos e a política seguem as mesmas regras por grupo que a CLI. Um desenvolvedor que usa tanto a CLI quanto Claude Desktop faz login em cada um separadamente; a sessão do gateway não é compartilhada entre eles.

Uma vez conectado, Claude Desktop envia solicitações de modelo de cada aba habilitada através do gateway. Ele mostra as abas Cowork e Code por padrão. Para ativar também a aba Chat, defina `chatTabEnabled` para `true` na [configuração gerenciada](https://claude.com/docs/third-party/claude-desktop/configuration) do Claude Desktop, ou no bloco [`desktop`](/docs/pt/claude-apps-gateway-config#claude-desktop-overlay) da política em um gateway executando Claude Code v2.1.227 ou posterior.

<h3 id="ci-pipelines-and-remote-machines">
  Pipelines de CI e máquinas remotas
</h3>

Não há fluxo de token de serviço para pipelines não supervisionados. O sign-in do gateway sempre executa o fluxo de dispositivo do navegador, então um trabalho de CI sem um desenvolvedor para aprovar o sign-in não pode se autenticar; configure aqueles contra seu provedor diretamente.

Uma vez que um desenvolvedor tenha feito login, cada sessão Claude Code nessa máquina usa a sessão do gateway, incluindo execuções não interativas `claude -p` e sessões iniciadas pelo Agent SDK. Claude Code aplica a [política do gateway](/docs/pt/claude-apps-gateway-config#managed) a cada uma delas.

O fluxo de dispositivo separa a CLI de polling do navegador aprovador, então uma caixa de desenvolvimento remota sem exibição ainda funciona: o desenvolvedor executa `/login` via SSH na máquina remota e abre o link de verificação no navegador em seu laptop.

<h3 id="whats-enforced-on-developers">
  O que é aplicado aos desenvolvedores
</h3>

Estas garantias se aplicam a cada sessão conectada através de `/login`. As sessões incorporadas que Claude Desktop inicia obtêm sua política conforme descrito em [Entregar política para sessões Claude Desktop](#deliver-policy-to-claude-desktop-sessions), e o ponto de telemetria diz para onde suas exportações vão.

* **Acesso a modelos**: solicitações para modelos que a política não concede retornam 400, e o seletor `/model` é filtrado para a lista de permissões `availableModels` da política. Defina [`enforceAvailableModels: true`](/docs/pt/model-config#default-model-behavior) na política para que a opção Padrão resolva para um modelo dentro de `availableModels` em vez de para o padrão integrado do Claude Code; sem isso, Padrão permanece selecionável e é rejeitado no tempo de solicitação se esse modelo não for concedido.
* **Destino de telemetria**: em sessões conectadas através de `/login`, a CLI envia suas exportações OTLP/HTTP para o gateway em vez de para um `OTEL_EXPORTER_OTLP_ENDPOINT` definido localmente, a menos que uma política [nomeie seu coletor como o endpoint](/docs/pt/claude-apps-gateway-config#export-directly-to-your-collector). O gateway retransmite as exportações que recebe para os destinos em [`telemetry.forward_to`](/docs/pt/claude-apps-gateway-config#telemetry).
  * Nas sessões incorporadas que [Claude Desktop inicia](#connect-claude-desktop), a CLI envia suas exportações para o `OTEL_EXPORTER_OTLP_ENDPOINT` configurado. A CLI anexa o token de sessão do gateway àquelas exportações apenas quando esse endpoint aponta para o próprio gateway.
  * Sem destino configurado para um sinal, o gateway o aceita e descarta.
  * Se você já coleta telemetria Claude Code diretamente, adicione seu coletor como um destino `forward_to`, ou nomeie-o em uma política para pular a retransmissão.
* **Credenciais**: o token do gateway é a única credencial da sessão. [Perfis Anthropic](/docs/pt/authentication#anthropic-profiles-and-federation-credentials) e qualquer login anterior de claude.ai são ignorados enquanto conectado, então os desenvolvedores não precisam fazer logout de claude.ai primeiro. Para uma credencial `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` configurada, veja [A política do administrador requer um sign-in Cloud gateway](/docs/pt/errors#administrator-policy-requires-a-cloud-gateway-sign-in).
* **Configurações gerenciadas**: chaves bloqueadas não podem ser substituídas localmente. A CLI aplica a política na inicialização e aplica mudanças em cada sondagem horária, além das [mudanças que se aplicam apenas no próximo lançamento](/docs/pt/server-managed-settings#fetch-and-caching-behavior).
* **Inicialização com o gateway inacessível**: sessões conectadas saem na inicialização com um erro após cerca de 10 segundos em vez de iniciar sem suas configurações.
* **Inicialização após o gateway encerrar a sessão**: veja [Aplicar falha fechada na inicialização](/docs/pt/server-managed-settings#enforce-fail-closed-startup) para quais lançamentos abrem desconectados do gateway e quais saem quando o gateway responde com um `401`.
* **Desprovisionamento**: uma sessão cujo usuário é desabilitado no IdP expira dentro de `ttl_hours` quando a próxima atualização falha.
* **Sign-out**: `/logout` deleta a credencial do gateway da máquina do desenvolvedor.
  * Quando o documento de descoberta do gateway anuncia um `revocation_endpoint` no esquema, host e porta próprios da URL do gateway, `/logout` também envia os tokens armazenados para esse endpoint para que o gateway possa encerrar a sessão em seu lado. A solicitação é melhor esforço, então o sign-out é concluído na máquina do desenvolvedor independentemente de o endpoint responder. A revogação requer Claude Code v2.1.275 ou posterior na máquina do desenvolvedor.
  * O servidor do gateway no binário `claude` não anuncia nenhum, então um sign-out dele encerra a sessão na máquina do desenvolvedor apenas. Para forçar sessões para fora no lado do servidor, veja [Rotação de segredo JWT](/docs/pt/claude-apps-gateway-deploy#jwt-secret-rotation).

<h3 id="what-the-organization-can-see">
  O que a organização pode ver
</h3>

A telemetria de uso carrega a identidade do desenvolvedor, contagens de tokens, modelo e latência para o coletor da organização. O gateway não registra ou armazena conteúdo de prompt ou conclusão. Se telemetria mais rica, como logs e rastreamentos, é coletada, que pode incluir comandos e caminhos de arquivo, é a [escolha por destino](/docs/pt/claude-apps-gateway-config#telemetry) da organização.

<h2 id="availability-and-limitations">
  Disponibilidade e limitações
</h2>

A tabela cobre quais recursos do Claude Code funcionam quando os desenvolvedores se conectam através do gateway e o que o próprio servidor gateway suporta. Onde algo não é suportado, a coluna Notas fornece a alternativa.

O gateway entrega os valores [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) que a CLI envia para cada upstream, então operadores não mantêm uma lista de permissões beta. Para Amazon Bedrock, que ignora o cabeçalho, o gateway move os valores para o campo `anthropic_beta` do corpo da solicitação; os outros upstreams recebem o cabeçalho conforme enviado.

| Recurso                                                                                                                             | Status                | Notas                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ----------------------------------------------------------------------------------------------------------------------------------- | --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Encaminhamento de inferência (Amazon Bedrock, Claude Platform on AWS, Agent Platform do Google Cloud, Microsoft Foundry, Anthropic) | Disponível            | Com tradução de modelo por upstream e failover. O upstream Amazon Bedrock usa o endpoint `bedrock-runtime` e a cadeia de credencial padrão da AWS; o [endpoint Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint) do Amazon Bedrock não é um upstream suportado. O [upstream Claude Platform on AWS](/docs/pt/claude-apps-gateway-config#claude-platform-on-aws) requer Claude Code v2.1.198 ou posterior no servidor gateway.                                                    |
| Acesso a modelos e configurações gerenciadas por grupo IdP                                                                          | Disponível            | O acesso a modelos é aplicado no lado do servidor; as configurações gerenciadas são entregues por grupo IdP e aplicadas pela CLI no [nível de configurações gerenciadas](/docs/pt/settings#settings-precedence)                                                                                                                                                                                                                                                                    |
| Claude Desktop                                                                                                                      | Disponível com opt-in | O gateway fornece a configuração do Claude Desktop em `/user/bootstrap` uma vez que uma política [opta por um `desktop` key](/docs/pt/claude-apps-gateway-config#claude-desktop-overlay), e o Claude Desktop envia solicitações de modelo de suas abas Cowork e Code, e da aba Chat quando você a habilita, através do gateway. Para ativar a aba Chat, consulte [Conectar Claude Desktop](#connect-claude-desktop). Requer Claude Code v2.1.203 ou posterior no servidor gateway. |
| Fan-out de telemetria (OTLP/HTTP)                                                                                                   | Disponível            | Identidade-marcada por exportação; ambas as codificações protobuf e JSON                                                                                                                                                                                                                                                                                                                                                                                                      |
| Provedores de identidade OIDC                                                                                                       | Disponível            | Qualquer IdP compatível com OIDC; o gateway executa descoberta OIDC padrão e o fluxo de código de autorização. Consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup) para configuração por IdP                                                                                                                                                                                                                            |
| Limites de gastos por usuário e por grupo                                                                                           | Disponível            | Consulte [Limites de gastos](/docs/pt/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                                            |
| Busca na web no lado do servidor                                                                                                    | Não disponível        | A CLI não pode ver qual provedor upstream o gateway roteia, então não pode verificar o suporte de busca na web e desabilita WebSearch em sessões de gateway                                                                                                                                                                                                                                                                                                                   |
| [Remote Control](/docs/pt/remote-control)                                                                                                | Não disponível        | A CLI mostra [um erro nomeando o gateway](/docs/pt/errors#remote-control-requires-the-anthropic-api)                                                                                                                                                                                                                                                                                                                                                                               |
| [`/design-sync`](/docs/pt/commands#all-commands) e `/design-login`                                                                       | Não disponível        | Ambos precisam de claude.ai, que a CLI não contatará em sessões de gateway, então nenhum comando aparece lá                                                                                                                                                                                                                                                                                                                                                                   |
| Recursos que precisam de busca de sinalizador de recurso, como `/import` e `claude import`                                          | Não disponível        | A CLI pula a busca de sinalizador em sessões de gateway. [Recursos que precisam de busca de sinalizador de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching) lista o que isso desativa                                                                                                                                                                                                                                                                          |
| Cache de prompt padrão                                                                                                              | Disponível            | O gateway encaminha pontos de interrupção `cache_control` para cada upstream. [Onde o cache reside](/docs/pt/prompt-caching#where-the-cache-lives) cobre quais blocos a CLI marca, incluindo o contexto do sistema que ela anexa no meio da conversa                                                                                                                                                                                                                               |
| TTL de cache de 1 hora                                                                                                              | Não disponível        | A CLI omite a beta de ttl de cache estendido em sessões de gateway, porque nem todo upstream que o gateway pode rotear suporta o TTL de 1 hora, então o cache de prompt através do gateway usa o TTL de 5 minutos; consulte a nota de cabeçalho beta acima                                                                                                                                                                                                                    |
| Modo automático                                                                                                                     | Disponível            | Segue as [regras do provedor de terceiros](/docs/pt/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): apenas os modelos elegíveis em provedores de terceiros podem usá-lo. Antes da v2.1.207, o modo automático em sessões de gateway exigia a definição de `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, entregável através do bloco `env` da política gerenciada                                                                                                      |
| Otimizações apenas de primeira parte, como escopo de cache global e ferramentas eficientes em tokens                                | Não disponível        | A CLI não as habilita em sessões de gateway; consulte a nota de cabeçalho beta acima                                                                                                                                                                                                                                                                                                                                                                                          |
| OTLP/gRPC                                                                                                                           | Não suportado         | OTLP sobre HTTP apenas                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| SAML, LDAP e outras autenticações não-OIDC                                                                                          | Não suportado         | OIDC apenas. Coloque na frente com uma ponte OIDC se necessário                                                                                                                                                                                                                                                                                                                                                                                                               |
| Multi-tenant (múltiplos emissores OIDC)                                                                                             | Não suportado         | Um emissor por gateway. Execute instâncias separadas                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Servidor Windows                                                                                                                    | Não suportado         | Implante no Linux. macOS apenas para desenvolvimento local                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Gráfico Helm                                                                                                                        | Não disponível        | O gateway é executado como um Deployment sem estado padrão; consulte o [guia de implantação](/docs/pt/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                                                       |
| Interface do usuário de administração                                                                                               | Não disponível        | A configuração é o arquivo YAML; reimplante para alterá-la                                                                                                                                                                                                                                                                                                                                                                                                                    |

<h2 id="next-steps">
  Próximas etapas
</h2>

O guia de início rápido deixa você com uma configuração mínima em execução sob Docker Compose. Para ir além:

* Expanda `gateway.yaml` além da configuração mínima, por exemplo para adicionar RBAC por grupo, failover multi-upstream ou destinos de telemetria. A [referência de configuração](/docs/pt/claude-apps-gateway-config) cobre todas as opções.
* Mude do Compose para uma implantação de produção em Kubernetes ou Cloud Run, configure seu IdP adequadamente e revise o modelo de segurança. O [guia de implantação e operações](/docs/pt/claude-apps-gateway-deploy) cobre configuração por IdP, requisitos de imagem de contêiner, sondas de saúde e solução de problemas.
* Coloque limites de gastos em desenvolvedores individuais ou grupos para que uma carga de trabalho descontrolada não possa consumir todo o seu compromisso. [Limites de gastos](/docs/pt/claude-apps-gateway-spend-limits) cobre a API de administração e como a aplicação funciona.
* Para um exemplo completo trabalhado no AWS, com ECS Fargate ou EKS, Amazon RDS e Secrets Manager, consulte [Implante no AWS](/docs/pt/claude-apps-gateway-on-aws).
* Para um exemplo completo trabalhado no Google Cloud, com Cloud Run, Cloud SQL e Secret Manager, consulte [Implante no Google Cloud](/docs/pt/claude-apps-gateway-on-gcp).
