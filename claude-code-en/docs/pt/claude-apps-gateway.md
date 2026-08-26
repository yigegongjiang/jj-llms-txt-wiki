> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gateway de aplicativos Claude para Amazon Bedrock, Claude Platform on AWS, Google Cloud e Microsoft Foundry

> Execute Claude Code através do Amazon Bedrock, Claude Platform on AWS, Google Cloud ou Microsoft Foundry atrás de um gateway auto-hospedado com sign-in SSO, acesso a modelos por grupo e telemetria OTLP.

<Note>
  O gateway de aplicativos Claude foi projetado para organizações que devem — ou preferem — rotear inferência através de seu próprio provedor de nuvem, por exemplo para atender aos requisitos de [residência de dados](/docs/pt/claude-apps-gateway-deploy#compliance-posture). Se você não tem esse requisito e deseja acesso a outros recursos como provisionamento SCIM ou Claude Code na web e mobile, Claude Enterprise pode ser uma opção melhor. Consulte a página de [disponibilidade de recursos](/docs/pt/feature-availability) para uma comparação completa de todos os métodos de implantação.
</Note>

Claude apps gateway é um serviço auto-hospedado que fica entre os clientes Claude Code dos seus desenvolvedores e seu provedor de modelo. Os desenvolvedores fazem login com seu provedor de identidade corporativo (IdP) em vez de manter chaves de API ou credenciais de nuvem. O gateway mantém a credencial upstream, aplica acesso a modelos e [configurações gerenciadas](/docs/pt/permissions#managed-settings) por grupo IdP, e retransmite telemetria de uso para sua própria pilha de observabilidade.

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
* **Controle de acesso**: seus grupos IdP mapeiam para listas de permissões de modelos e políticas de [configurações gerenciadas](/docs/pt/permissions#managed-settings). O gateway aplica acesso a modelos no lado do servidor, rejeitando solicitações para modelos não concedidos, e seleciona a política de configurações gerenciadas de cada grupo, que a CLI aplica no [nível de configurações gerenciadas](/docs/pt/settings#settings-precedence). Diferentes equipes obtêm diferentes modelos, ferramentas e permissões, e um desenvolvedor não pode substituir o que sua política bloqueia.
* **Entrega de configurações**: o gateway entrega configurações gerenciadas para clientes conectados, assumindo o lugar das [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) do console administrativo claude.ai.
* **Telemetria**: cada destino configurado, como Datadog, Splunk ou ClickHouse, recebe [métricas do OpenTelemetry Protocol (OTLP)](/docs/pt/monitoring-usage) com contagens de tokens, modelo, identidade do usuário e latência por padrão, com logs e rastreamentos como opt-ins por destino.
* **Roteamento upstream**: os clientes falam a API de Mensagens Anthropic para o gateway, e o gateway traduz para cada upstream, seja Amazon Bedrock, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), Agent Platform do Google Cloud, Microsoft Foundry ou a API Anthropic, com failover entre eles. Você pode alterar regiões, provedores ou ordem de failover sem que os desenvolvedores notem ou reconfiguram.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagrama mostrando clientes Claude Code conectando via HTTPS com tokens de portador a um gateway de aplicativos Claude auto-hospedado dentro de sua infraestrutura, que faz login dos usuários contra seu IdP, armazena estado de autenticação em PostgreSQL, retransmite telemetria para seu coletor OTLP, e encaminha inferência para Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry ou a API Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  O plano de dados do próprio gateway não envia nada para a infraestrutura Anthropic a menos que a API Anthropic seja um upstream configurado. Você controla para onde a telemetria, logs de auditoria, configurações gerenciadas e a identidade IdP dos seus desenvolvedores vão, e o gateway não envia nenhum deles para Anthropic. Para o tráfego restante que o processo CLI pode enviar e como fechá-lo, consulte [Postura de conformidade](/docs/pt/claude-apps-gateway-deploy#compliance-posture).
</Note>

Para quais recursos do Claude Code funcionam através do gateway e o que o próprio servidor suporta, consulte [Disponibilidade e limitações](#availability-and-limitations) abaixo. Para decisões como custo, bypass, executar múltiplos gateways e plataformas sem servidor, consulte o [guia de implantação](/docs/pt/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Outras implementações de gateway
</h3>

Se você já executa um gateway LLM ou gateway de API que atende às suas necessidades, continue usando-o; [Outros gateways LLM](/docs/pt/llm-gateway) cobre a configuração do Claude Code contra ele.

A [referência do protocolo de gateway](/docs/pt/llm-gateway-protocol) documenta o contrato que Claude Code espera de qualquer gateway: os endpoints que chama, os cabeçalhos e campos de corpo para encaminhar, e o que para de funcionar quando são removidos. Um gateway de aplicativos Claude em execução serve um superconjunto desse contrato em `GET /protocol`, adicionando os endpoints específicos do gateway de aplicativos Claude para sign-in SSO, entrega de configurações gerenciadas e telemetria. Busque-o com `curl https://claude-gateway.internal.example.com/protocol` de qualquer gateway implantado, como o que o [guia de início rápido](#quickstart) abaixo produz.

Mudanças significativas no protocolo são anunciadas com antecedência, mas compatibilidade retroativa indefinida não é garantida.

<h2 id="quickstart">
  Guia de início rápido
</h2>

Este guia de início rápido percorre o caminho mínimo: registre um cliente OAuth em seu IdP, escreva um `gateway.yaml`, execute o gateway junto com Postgres usando Docker Compose, e verifique o sign-in de ponta a ponta. Usa um upstream Amazon Bedrock; Claude Platform on AWS, Agent Platform do Google Cloud, Microsoft Foundry e a API Anthropic são igualmente suportados trocando o bloco `upstreams` conforme mostrado na [referência de configuração](/docs/pt/claude-apps-gateway-config#upstreams). No final você tem um gateway que um desenvolvedor pode fazer `/login`.

<Note>
  **Implante em sua rede privada.** Claude Code só se conecta a um gateway cujo endereço é privado. Esta é uma proteção de segurança, porque um gateway confiável pode enviar configurações que executam comandos em máquinas de desenvolvedores. Coloque o gateway atrás de um balanceador de carga interno ou VPN e dê a ele um nome de host que resolve apenas para IPs privados.

  Os endpoints de gateway público operados pela Anthropic são a exceção: `/login` os aceita via `https://`. Estes são um pequeno conjunto fixo de gateways que a própria Anthropic opera; não são uma opção de implantação que você pode selecionar ou configurar. A lista é compilada no Claude Code, portanto nenhuma configuração pode adicionar um nome de host a ela e nenhum gateway que você hospeda se qualifica para a isenção. Antes da v2.1.206, `/login` rejeitava esses endpoints como qualquer outro endereço público.
</Note>

<h3 id="prerequisites">
  Pré-requisitos
</h3>

Tenha estes em vigor antes de começar:

| Você precisa                                 | Detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Claude Code v2.1.195 ou posterior            | O subcomando `claude gateway` e o fluxo de sign-in do gateway são enviados na v2.1.195. Compilações públicas anteriores não as incluem. Tanto a máquina executando o servidor gateway quanto a máquina de cada desenvolvedor devem estar na v2.1.195 ou posterior; execute `claude update` para obter a versão mais recente. O [upstream Claude Platform on AWS](/docs/pt/claude-apps-gateway-config#claude-platform-on-aws) requer Claude Code v2.1.198 ou posterior no servidor gateway.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Provedor de identidade OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak ou Dex, ou qualquer outro IdP compatível com OIDC, como PingFederate. O gateway executa descoberta OIDC padrão e o fluxo de código de autorização contra ele. SAML e LDAP não são suportados.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| PostgreSQL 14 ou posterior                   | Faz backup do fluxo de sign-in do dispositivo, onde o callback do navegador escreve e a CLI de polling lê, além de contadores de limite de taxa. Qualquer Postgres gerenciado funciona, incluindo o menor nível. Sem limites de gastos configurados, o gateway armazena alguns KB de estado de autenticação de curta duração; com [limites de gastos](/docs/pt/claude-apps-gateway-spend-limits), também mantém tabelas de gastos, auditoria e identidade duráveis que devem ser feitas backup. TLS via `?sslmode=require` é recomendado.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Upstream de modelo                           | Credenciais do Amazon Bedrock, credenciais do Claude Platform on AWS, credenciais do Google Cloud, um recurso Microsoft Foundry ou uma chave de API Anthropic. Múltiplos upstreams são suportados com failover.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| HTTPS                                        | O gateway deve ser acessível via `https://` de laptops de desenvolvedores e de qualquer navegador usado para sign-in; o gateway serve a página de verificação do dispositivo no mesmo listener. Forneça um certificado TLS via `listen.tls` ou execute atrás de um ingress que termina TLS e defina `listen.public_url`. Uma origem `http://` simples é aceita apenas em loopback, para desenvolvimento local.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Endereço de rede privada                     | Em `/login`, Claude Code requer que o nome de host ou endereço IP do gateway resolva apenas para endereços privados: RFC 1918, CGNAT `100.64.0.0/10`, ULA IPv6 `fc00::/7` ou loopback para desenvolvimento local. A verificação é executada em cada IP resolvido, então se qualquer endereço para o qual o nome resolve for público, `/login` rejeita a URL. Se máquinas de desenvolvedores rotear HTTPS através de um proxy corporativo, o sign-in também requer que o host proxy resolva para endereços privados; se não resolver, adicione o host do gateway a `NO_PROXY` para que a CLI se conecte diretamente. Os endpoints de gateway público operados pela Anthropic estão isentos das verificações de endereço privado e proxy: `/login` os aceita via `https://` por correspondência exata de nome de host, portanto o requisito de rede privada se aplica apenas a um gateway que você hospeda. Antes da v2.1.206, `/login` rejeitava um endpoint operado pela Anthropic como qualquer outro endereço público. |
| Runtime Linux                                | O servidor gateway é executado apenas no binário Linux nativo. macOS funciona para desenvolvimento local. Windows não é suportado como plataforma de servidor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

O servidor gateway requer o binário `claude` nativo; baixe uma versão fixada conforme descrito em [Instalar Claude Code](/docs/pt/setup). O servidor usa recursos de runtime que não estão disponíveis quando Claude Code é executado sob Node. Se você vir `requires the native binary` na inicialização, mude para um dos métodos de instalação autônomos.

<h3 id="steps">
  Etapas
</h3>

<Steps>
  <Step title="Registre um cliente OAuth em seu IdP">
    Decida o nome de host do gateway primeiro, porque o URI de redirecionamento deve corresponder a ele. Crie um novo aplicativo web OIDC e defina o URI de redirecionamento para `https://claude-gateway.<seu-domínio>/oauth/callback`, onde o host é o mesmo valor que você define como [`listen.public_url`](/docs/pt/claude-apps-gateway-config#listen) na etapa 3. Anote o `client_id` e `client_secret`. As instruções por IdP estão em [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Provisione um banco de dados PostgreSQL">
    Qualquer Postgres 14 ou posterior funciona, incluindo o menor nível gerenciado. O gateway executa suas próprias migrações de esquema na inicialização, então o usuário do banco de dados precisa de permissão `CREATE TABLE`. Se sua política de segurança proíbe DDL de funções de aplicação, pré-crie o esquema em vez disso; consulte [`store`](/docs/pt/claude-apps-gateway-config#store).
  </Step>

  <Step title="Escreva gateway.yaml">
    Os segredos são lidos via expansão `${ENV_VAR}` para que o arquivo em si possa viver no controle de versão. Use um nome de host `public_url` que resolva para um IP privado em sua rede, porque `/login` rejeita endereços públicos. A configuração mínima tem cinco seções, e todos os outros campos têm um padrão:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Obrigatório atrás de qualquer proxy que termina TLS. Usado para o IdP
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
      O upstream Bedrock precisa de um principal AWS com `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` nos ARNs `inference-profile/us.anthropic.*` e nos ARNs `foundation-model/anthropic.*` subjacentes, e acesso a modelos habilitado no console Bedrock para os modelos Claude que você deseja. Forneça a credencial com IRSA no EKS, uma função de tarefa ECS ou um perfil de instância EC2 em vez de chaves estáticas. A [referência `upstreams`](/docs/pt/claude-apps-gateway-config#upstreams) tem os detalhes completos do IAM, a matriz de credencial entre nuvens e os blocos `auth` para os outros provedores.
    </Note>
  </Step>

  <Step title="Execute-o">
    Construa uma imagem de contêiner em torno do binário `claude` que atenda aos [requisitos de imagem](/docs/pt/claude-apps-gateway-deploy#container-image), então execute-a junto com Postgres:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: <seu-registro>/claude-gateway:<versão>
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

    O gateway é um único binário Linux que lê a configuração, executa descoberta OIDC contra seu IdP, aplica suas migrações de esquema Postgres, constrói clientes upstream e começa a escutar. A inicialização é fail-closed para a configuração, a conexão Postgres com um tempo limite de 5 segundos, descoberta OIDC e construção de cliente upstream. Se qualquer um desses for inacessível ou mal configurado, o gateway sai com um erro em vez de servir tráfego em um estado degradado.

    Uma inicialização bem-sucedida não valida o caminho de inferência, porque credenciais de instância Bedrock e Agent Platform resolvem na primeira solicitação, não na inicialização.

    Observe stderr para a sequência de inicialização. As linhas de log usam o formato `[gateway] <timestamp> <level> <message>`, eventos de auditoria são JSON de linha única com um campo `evt`, e um banner de inicialização, omitido abaixo, é impresso entre as linhas de migração e escuta. Você deve ver, em ordem:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    Se a inicialização sair antes da linha `claude gateway listening on`, a última linha de stderr nomeia o problema:

    * um Postgres inacessível
    * uma função Postgres sem permissão DDL
    * um documento de descoberta OIDC inacessível ou inválido
    * uma violação de esquema de configuração com o caminho de campo ofensivo

    Corrija-o e reinicie.

    Se você já tem um ingress que termina TLS, pule o Compose e execute o binário diretamente com `claude gateway --config gateway.yaml`. Defina `public_url` para a origem do ingress e vincule `listen` a um endereço loopback ou interno do cluster.
  </Step>

  <Step title="Verifique a superfície de autenticação">
    Três verificações confirmam que o gateway pode autenticar um usuário real antes de entregá-lo a um desenvolvedor.

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
    Esta última etapa acontece em uma máquina de desenvolvedor, não no servidor. Defina `forceLoginMethod` como `"gateway"` e `forceLoginGatewayUrl` como a `public_url` do seu gateway no [arquivo de configurações gerenciadas](/docs/pt/settings#settings-files) dessa máquina, então execute `/login`, pressione Enter na tela **Cloud gateway** e conclua o sign-in do navegador. [Defina a URL do gateway](#set-the-gateway-url) abaixo cobre a distribuição de ambas as chaves em escala.
  </Step>
</Steps>

<h2 id="connect-developers">
  Conectar desenvolvedores
</h2>

Os desenvolvedores se conectam de seus próprios laptops com um sign-in de navegador, usando sua conta de trabalho corporativa. Eles não precisam de uma conta claude.ai, uma chave de API ou uma assinatura, porque as solicitações para o modelo passam pelo gateway usando a credencial upstream da organização. A conexão é orientada pelas [configurações gerenciadas no lado do cliente](/docs/pt/claude-apps-gateway-config#client-side-managed-settings) que você envia via MDM, então não há configuração manual no lado do desenvolvedor; esta seção cobre o que o administrador configura.

A CLI coloca a impressão digital do certificado TLS folha do gateway na primeira conexão e a fixa por nome de host. Publique a impressão digital SHA-256 esperada junto com a URL do gateway para que os desenvolvedores tenham algo para comparar. Obtenha a impressão digital do arquivo de certificado com `openssl x509 -noout -fingerprint -sha256 -in cert.pem`; o prompt `/login` mostra os primeiros 16 caracteres do resumo como hexadecimal minúsculo sem separadores.

Quando o certificado é rotacionado, cada desenvolvedor vê o prompt de confiança novamente, então trate rotações como um evento planejado e republique a impressão digital.

Uma vez conectado, o [seletor de modelo](/docs/pt/model-config) mostra os modelos na lista de permissões `availableModels` do desenvolvedor, as configurações gerenciadas se aplicam na inicialização e atualizam a cada hora, e a telemetria é roteada para seu coletor. As sessões são atualizadas silenciosamente antes da expiração de `ttl_hours`, e uma atualização falhada após desprovisionamento de IdP solicita um novo login.

<h3 id="set-the-gateway-url">
  Defina a URL do gateway
</h3>

Defina ambas as chaves no arquivo de [configurações gerenciadas](/docs/pt/settings#settings-files) por SO que você implanta via MDM ou diretamente no disco, e `/login` abre diretamente na tela **Cloud gateway** com a URL preenchida:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

O desenvolvedor pressiona Enter para se conectar. O prompt de impressão digital TLS de primeira conexão ainda aparece.

Não há opção de gateway no seletor de login para um desenvolvedor selecionar manualmente, e `forceLoginGatewayUrl` é ignorado nos arquivos de configurações próprias de um desenvolvedor. `forceLoginMethod` sozinho, sem uma URL, deixa o desenvolvedor em uma mensagem "Entre em contato com seu administrador de TI". Ambas as chaves pertencem ao arquivo que você envia para máquinas, não ao bloco `managed.policies[].cli` do gateway, que só alcança clientes que já estão conectados.

<h3 id="ci-pipelines-and-remote-machines">
  Pipelines de CI e máquinas remotas
</h3>

Não há fluxo de token de serviço para pipelines não supervisionados. O sign-in do gateway sempre executa o fluxo de dispositivo do navegador, então um trabalho de CI sem um desenvolvedor para aprovar o sign-in não pode se autenticar; configure aqueles contra seu provedor diretamente.

Uma vez que um desenvolvedor tenha feito login, cada invocação do Claude Code nessa máquina usa a sessão do gateway, incluindo execuções não interativas `claude -p` e sessões iniciadas pelo Agent SDK, e a [política do gateway se aplica a todas elas](/docs/pt/claude-apps-gateway-config#managed).

O fluxo de dispositivo separa a CLI de polling do navegador aprovador, então uma caixa de desenvolvimento remota sem exibição ainda funciona: o desenvolvedor executa `/login` via SSH na máquina remota e abre o link de verificação no navegador em seu laptop.

<h3 id="what’s-enforced-on-developers">
  O que é aplicado aos desenvolvedores
</h3>

Estas garantias se aplicam a cada sessão de gateway conectada.

* **Acesso a modelos**: solicitações para modelos que a política não concede retornam 400, e o seletor `/model` é filtrado para a lista de permissões `availableModels` da política. Defina [`enforceAvailableModels: true`](/docs/pt/model-config#default-model-behavior) na política para que a opção Padrão resolva para um modelo dentro de `availableModels` em vez de para o padrão integrado do Claude Code; sem isso, Padrão permanece selecionável e é rejeitado no tempo de solicitação se esse modelo não for concedido.
* **Destino de telemetria**: quando [encaminhamento de telemetria](/docs/pt/claude-apps-gateway-config#telemetry) é configurado, o endpoint de exportação OTLP é fixado ao gateway, e a configuração enviada pelo gateway substitui variáveis `OTEL_*` definidas localmente.
* **Credenciais**: o token do gateway é a única credencial da sessão. `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, `apiKeyHelper` e qualquer login anterior de claude.ai são ignorados enquanto conectado, então os desenvolvedores não precisam fazer logout de claude.ai primeiro.
* **Configurações gerenciadas**: chaves bloqueadas não podem ser substituídas localmente. A CLI aplica a política na inicialização e em cada sondagem horária.
* **Inicialização**: sessões conectadas saem na inicialização com um erro após cerca de 10 segundos quando o gateway é inacessível, em vez de iniciar sem suas configurações.
* **Desprovisionamento**: uma sessão cujo usuário é desabilitado no IdP expira dentro de `ttl_hours` quando a próxima atualização falha.

<h3 id="what-the-organization-can-see">
  O que a organização pode ver
</h3>

A telemetria de uso carrega a identidade do desenvolvedor, contagens de tokens, modelo e latência para o coletor da organização. O gateway não registra ou armazena conteúdo de prompt ou conclusão. Se telemetria mais rica, como logs e rastreamentos, é coletada, que pode incluir comandos e caminhos de arquivo, é a [escolha por destino](/docs/pt/claude-apps-gateway-config#telemetry) da organização.

<h2 id="availability-and-limitations">
  Disponibilidade e limitações
</h2>

A tabela cobre quais recursos do Claude Code funcionam quando os desenvolvedores se conectam através do gateway e o que o próprio servidor gateway suporta. Onde algo não é suportado, a coluna Notas fornece a alternativa.

O gateway entrega os valores [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) que a CLI envia para cada upstream, então operadores não mantêm uma lista de permissões beta. Para Amazon Bedrock, que ignora o cabeçalho, o gateway move os valores para o campo `anthropic_beta` do corpo da solicitação; os outros upstreams recebem o cabeçalho conforme enviado.

O conjunto beta de sessão de gateway da CLI omite betas apenas de primeira parte e a beta de ttl de cache estendido, é por isso que essas linhas abaixo mostram como não disponíveis.

| Recurso                                                                                                                             | Status         | Notas                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ----------------------------------------------------------------------------------------------------------------------------------- | -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Encaminhamento de inferência (Amazon Bedrock, Claude Platform on AWS, Agent Platform do Google Cloud, Microsoft Foundry, Anthropic) | Disponível     | Com tradução de modelo por upstream e failover. O upstream Amazon Bedrock usa o endpoint `bedrock-runtime` e a cadeia de credencial padrão da AWS; o [endpoint Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint) do Amazon Bedrock não é um upstream suportado. O [upstream Claude Platform on AWS](/docs/pt/claude-apps-gateway-config#claude-platform-on-aws) requer Claude Code v2.1.198 ou posterior no servidor gateway. |
| Acesso a modelos e configurações gerenciadas por grupo IdP                                                                          | Disponível     | O acesso a modelos é aplicado no lado do servidor; as configurações gerenciadas são entregues por grupo IdP e aplicadas pela CLI no [nível de configurações gerenciadas](/docs/pt/settings#settings-precedence)                                                                                                                                                                                                                 |
| Fan-out de telemetria (OTLP/HTTP)                                                                                                   | Disponível     | Identidade-marcada por exportação; ambas as codificações protobuf e JSON                                                                                                                                                                                                                                                                                                                                                   |
| Provedores de identidade OIDC                                                                                                       | Disponível     | Qualquer IdP compatível com OIDC; o gateway executa descoberta OIDC padrão e o fluxo de código de autorização. Consulte [Configuração do provedor de identidade](/docs/pt/claude-apps-gateway-deploy#identity-provider-setup) para configuração por IdP                                                                                                                                                                         |
| Limites de gastos por usuário e por grupo                                                                                           | Disponível     | Consulte [Limites de gastos](/docs/pt/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                         |
| Busca na web no lado do servidor                                                                                                    | Não disponível | A CLI não pode ver qual provedor upstream o gateway roteia, então não pode verificar o suporte de busca na web e desabilita WebSearch em sessões de gateway                                                                                                                                                                                                                                                                |
| Cache de prompt padrão                                                                                                              | Disponível     | Os pontos de interrupção `cache_control` são encaminhados para cada upstream                                                                                                                                                                                                                                                                                                                                               |
| TTL de cache de 1 hora                                                                                                              | Não disponível | A CLI omite a beta de ttl de cache estendido em sessões de gateway, porque nem todo upstream que o gateway pode rotear suporta o TTL de 1 hora, então o cache de prompt através do gateway usa o TTL de 5 minutos; consulte a nota de cabeçalho beta acima                                                                                                                                                                 |
| Modo automático                                                                                                                     | Disponível     | Segue as [regras do provedor de terceiros](/docs/pt/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): apenas os modelos elegíveis em provedores de terceiros podem usá-lo. Antes da v2.1.207, o modo automático em sessões de gateway exigia a definição de `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, entregável através do bloco `env` da política gerenciada                                                   |
| Otimizações apenas de primeira parte, como escopo de cache global e ferramentas eficientes em tokens                                | Não disponível | A CLI não as habilita em sessões de gateway; consulte a nota de cabeçalho beta acima                                                                                                                                                                                                                                                                                                                                       |
| OTLP/gRPC                                                                                                                           | Não suportado  | OTLP sobre HTTP apenas                                                                                                                                                                                                                                                                                                                                                                                                     |
| SAML, LDAP e outras autenticações não-OIDC                                                                                          | Não suportado  | OIDC apenas. Coloque na frente com uma ponte OIDC se necessário                                                                                                                                                                                                                                                                                                                                                            |
| Multi-tenant (múltiplos emissores OIDC)                                                                                             | Não suportado  | Um emissor por gateway. Execute instâncias separadas                                                                                                                                                                                                                                                                                                                                                                       |
| Servidor Windows                                                                                                                    | Não suportado  | Implante no Linux. macOS apenas para desenvolvimento local                                                                                                                                                                                                                                                                                                                                                                 |
| Gráfico Helm                                                                                                                        | Não disponível | O gateway é executado como um Deployment sem estado padrão; consulte o [guia de implantação](/docs/pt/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                    |
| Interface do usuário de administração                                                                                               | Não disponível | A configuração é o arquivo YAML; reimplante para alterá-la                                                                                                                                                                                                                                                                                                                                                                 |

<h2 id="next-steps">
  Próximas etapas
</h2>

O guia de início rápido deixa você com uma configuração mínima em execução sob Docker Compose. Para ir além:

* Expanda `gateway.yaml` além da configuração mínima, por exemplo para adicionar RBAC por grupo, failover multi-upstream ou destinos de telemetria. A [referência de configuração](/docs/pt/claude-apps-gateway-config) cobre todas as opções.
* Mude do Compose para uma implantação de produção em Kubernetes ou Cloud Run, configure seu IdP adequadamente e revise o modelo de segurança. O [guia de implantação e operações](/docs/pt/claude-apps-gateway-deploy) cobre configuração por IdP, requisitos de imagem de contêiner, sondas de saúde e solução de problemas.
* Coloque limites de gastos em desenvolvedores individuais ou grupos para que uma carga de trabalho descontrolada não possa consumir todo o seu compromisso. [Limites de gastos](/docs/pt/claude-apps-gateway-spend-limits) cobre a API de administração e como a aplicação funciona.
* Para um exemplo completo trabalhado no Google Cloud, com Cloud Run, Cloud SQL e Secret Manager, consulte [Implante no Google Cloud](/docs/pt/claude-apps-gateway-on-gcp).
