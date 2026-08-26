> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implantação e operação do gateway de aplicativos Claude

> Registre o gateway com seu IdP, crie o contêiner, implante no Kubernetes ou Cloud Run e o opere: verificações de integridade, rotação de segredos, atualizações e segurança.

Esta página cobre o lado operacional da execução do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway): registrar um cliente OAuth em seu provedor de identidade (IdP), implantar o gateway como um contêiner e executá-lo no dia a dia. Para cada opção no arquivo `gateway.yaml` que o gateway lê na inicialização, consulte a [Referência de configuração](/docs/pt/claude-apps-gateway-config).

Uma implantação em produção segue quatro etapas em ordem, e as seções abaixo as correspondem. As duas primeiras são onde você faz escolhas; as duas últimas são material de referência para consultar quando estiver em execução.

1. [Configure seu provedor de identidade](#identity-provider-setup): registre o cliente OAuth e verifique as notas específicas do IdP para Okta, Entra e Google
2. [Implante o gateway](#deployment): crie uma imagem de contêiner fixada e execute-a no Kubernetes, Cloud Run ou sua própria plataforma. Esta seção também cobre decisões de custo, bypass, múltiplos gateways e serverless
3. [Configure operações](#operations): logs, sondas de integridade, comportamento de interrupção, rotação de segredos e atualizações. Referência para quando você estiver conectando monitoramento e runbooks
4. [Revise a postura de segurança](#security): para onde os dados fluem, o modelo de ameaça e respostas de conformidade. Referência para uma revisão de segurança

Se uma entrada ou inicialização falhar no caminho, vá direto para [Solução de problemas](#troubleshooting), que é indexada no erro que você vê.

<Note>
  **Implante em sua rede privada.** Claude Code apenas se conecta a um gateway cujo endereço é privado. Esta é uma proteção de segurança, porque um gateway confiável pode enviar configurações que executam comandos em máquinas de desenvolvedor. Coloque o gateway que você implanta atrás de um balanceador de carga interno ou VPN e dê a ele um nome de host que seja resolvido apenas para IPs privados.

  Os endpoints de gateway público operados pela Anthropic são a exceção: `/login` os aceita sobre `https://`. Estes são um pequeno conjunto fixo de gateways que a própria Anthropic opera; eles não são uma opção de implantação que você pode selecionar ou configurar. A lista é compilada no Claude Code, portanto nenhuma configuração pode adicionar um nome de host a ela e nenhum gateway que você hospeda se qualifica para a isenção. Antes da v2.1.206, `/login` rejeitava esses endpoints como qualquer outro endereço público.
</Note>

<h2 id="identity-provider-setup">
  Configuração do provedor de identidade
</h2>

Registre um aplicativo web OAuth/OpenID Connect (OIDC) confidencial com um único URI de redirecionamento, `https://<gateway>/oauth/callback`, e atribua-o aos usuários ou grupos que devem ter acesso ao gateway.

Qualquer IdP compatível com OIDC funciona: Okta, Microsoft Entra ID, Google Workspace, Keycloak, Dex, PingFederate e outros. O IdP deve atender a três requisitos:

* Serve `/.well-known/openid-configuration`, sobre HTTPS em produção; o gateway aceita um [emissor `http://`](/docs/pt/claude-apps-gateway-config#oidc), e um emissor de loopback adicionalmente requer `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`
* Suporta o fluxo de código de autorização. PKCE (Proof Key for Code Exchange) está ativado por padrão; desative-o com `oidc.use_pkce: false` para IdPs que não o suportam
* Retorna `email` e opcionalmente `groups` no id\_token, ou os serve do endpoint userinfo com `oidc.userinfo_fallback: true`

Para PKI privada, defina `oidc.ca_cert_pem`.

Alguns provedores lidam com email e reivindicações de grupo de forma diferente:

* **Okta**: o servidor de autorização da organização em `https://example.okta.com` retorna um id\_token fino que omite `email` e `groups`, então defina `oidc.userinfo_fallback: true` sempre que o usar como `issuer`. Um servidor de autorização personalizado como `https://example.okta.com/oauth2/default` que inclui `email` e opcionalmente `groups` no id\_token os emite diretamente e não precisa de fallback. Okta emite `groups` apenas quando o escopo `groups` é solicitado em `oidc.scopes` e o filtro de reivindicação de grupos do aplicativo o permite; `userinfo_fallback` não pode preencher uma reivindicação que o IdP não foi solicitado.
* **Microsoft Entra ID**: `issuer` = `https://login.microsoftonline.com/<tenant-id>/v2.0`. Entra emite Object IDs de grupo em vez de nomes, então use os GUIDs em `managed.policies.match.groups`, ou use App Roles para nomes legíveis por humanos. Se seu locatário emite funções sob `roles` em vez de `groups`, defina `oidc.groups_claim: roles`.
* **Google Workspace**: `issuer` = `https://accounts.google.com`. O id\_token do Google não carrega grupos. Para usar `allowed_groups` baseado em grupo ou `managed.policies` com Google como IdP, configure [`oidc.google_groups`](/docs/pt/claude-apps-gateway-config#oidc), que procura os grupos de cada usuário através da API do Directory do Admin SDK usando uma conta de serviço com delegação em todo o domínio. Sem isso, use `oidc.allowed_email_domains` para gating de associação e `managed.policies.match.email_domain` para atribuição de política. Google também ignora o escopo padrão `offline_access`. Para tokens de atualização, defina `oidc.scopes: [openid, profile, email]` e `oidc.extra_auth_params: { access_type: offline, prompt: consent }`.

Para suporte com um provedor de identidade não abordado acima, consulte [Troubleshooting](#troubleshooting).

<Warning>
  Tokens de atualização permitem que o gateway renove a sessão de um desenvolvedor silenciosamente, sem enviar o desenvolvedor de volta ao navegador. Eles também impulsionam o desprovisionamento, porque quando o IdP desativa um usuário, a próxima atualização falha e a sessão termina dentro de `ttl_hours`. O gateway solicita `offline_access` por padrão para obter um token de atualização. Se seu IdP exigir consentimento explícito para acesso offline, configure o cliente OAuth para permitir.

  Se seu IdP não conseguir emitir tokens de atualização, o gateway ainda funciona, mas não há renovação silenciosa, então os desenvolvedores executam novamente o login do navegador quando sua sessão expira. Para evitar que isso aconteça a cada hora, aumente [`session.ttl_hours`](/docs/pt/claude-apps-gateway-config#session) para `8` ou `12`. A compensação é a latência de desprovisionamento, porque sem tokens de atualização um usuário desativado mantém acesso até que o TTL mais longo decorra.
</Warning>

<h2 id="deployment">
  Implantação
</h2>

O gateway é um único binário Linux. Ele escala horizontalmente porque as réplicas são sem estado e o Postgres é a camada de coordenação compartilhada. Execute-o como você executa serviços sem estado em seu ambiente. O resto desta seção descreve o que a imagem precisa, com notas breves para Kubernetes e Cloud Run.

O gateway foi projetado para ser executado dentro de sua rede, porque mantém sua credencial upstream e atua como o único ponto de saída para inferência. Pode ser executado em qualquer lugar que seus desenvolvedores e seu IdP possam alcançar via HTTPS; trate-o como qualquer outro serviço que mantém uma credencial de produção.

Algumas decisões moldam a implantação além de onde ela é executada:

* **Custo**: não há licença separada ou taxa por assento para o gateway; é parte do binário `claude`. Você paga pela inferência através de seu compromisso de nuvem ou Anthropic existente, mais a computação para o contêiner e seu coletor de telemetria.
* **Bypass**: o gateway não impõe que a única rota para um modelo passe por ele. Um desenvolvedor com sua própria credencial ainda pode chamar o provedor diretamente, então fechar esse caminho é uma decisão de política de rede, por exemplo, bloqueando a saída para `api.anthropic.com` exceto do gateway. Bloquear essa saída também quebra a [verificação de segurança de domínio WebFetch](/docs/pt/data-usage#webfetch-domain-safety-check), que chama `api.anthropic.com` de cada máquina do desenvolvedor; defina `skipWebFetchPreflight: true` na política gerenciada para desativá-lo.
* **Múltiplos gateways**: cada gateway é uma implantação separada com sua própria configuração. O CLI armazena sua impressão digital de confiança e credenciais por nome de host do gateway, então diferentes equipes podem se conectar a diferentes gateways sem conflito. Para servir múltiplos emissores OIDC, execute instâncias separadas.
* **Serverless**: Cloud Run funciona; defina `min-instances: 1` para evitar descoberta OIDC fria. Lambda e Cloud Functions não funcionam, porque o gateway é um servidor HTTP de longa duração.

Cada topologia de produção aqui coloca um proxy L7, como um Ingress, o front-end do Cloud Run ou um ALB, na frente de réplicas HTTP simples. Defina [`listen.trusted_proxies`](/docs/pt/claude-apps-gateway-config#listen) para os intervalos de origem do proxy para que o gateway leia IPs de cliente de `X-Forwarded-For`. O gateway honra o cabeçalho apenas quando o par TCP é confiável; o [exemplo trabalhado do Google Cloud](/docs/pt/claude-apps-gateway-on-gcp) tem valores concretos por topologia. Sem proxies confiáveis, cada solicitação parece vir do IP do proxy, o que colapsa limites de taxa por IP em um balde compartilhado e registra o IP do proxy em eventos de auditoria.

<h3 id="container-image">
  Imagem de contêiner
</h3>

Construa sua própria imagem em torno do binário nativo `claude` da versão padrão do Claude Code:

1. Baixe a compilação Linux para a arquitetura de sua imagem de uma versão fixada; consulte [Instalar uma versão específica](/docs/pt/setup#install-a-specific-version) para a URL de download.
2. Verifique-a contra o `manifest.json` assinado por GPG da versão conforme descrito em [Integridade binária e assinatura de código](/docs/pt/setup#binary-integrity-and-code-signing).
3. Copie-a para o contexto de compilação.

Espelhe a versão em seu registro interno se suas compilações não conseguirem alcançar o host de versão, e fixe a versão que sua frota executa.

Além do binário, a imagem precisa:

* **Uma imagem baseada em glibc**: a compilação glibc tem apenas dependências dinâmicas de bibliotecas glibc. Imagens baseadas em Musl precisam da compilação `linux-x64-musl` ou `linux-arm64-musl` mais pacotes adicionais; consulte [Configuração do Alpine Linux](/docs/pt/setup#alpine-linux-and-musl-based-distributions).
* **Um diretório de estado gravável**: o gateway é executado como qualquer usuário, mas imagens mínimas não têm home gravável. Defina `CLAUDE_CONFIG_DIR` para um caminho gravável como `/tmp/.claude`.
* **O comando do contêiner**: `claude gateway --config /etc/claude/gateway.yaml`, com o arquivo de configuração montado como somente leitura e segredos fornecidos como variáveis de ambiente; o gateway escuta em `listen.port`, padrão `8080`.

<h3 id="kubernetes">
  Kubernetes
</h3>

Execute o gateway como uma Deployment, como qualquer serviço sem estado:

* Monte a configuração de um ConfigMap e segredos de um Secret; referencie segredos no YAML via `${file:/path/to/secret}` ou como variáveis de ambiente
* Termine TLS no Ingress e defina `listen.public_url` para o nome de host do Ingress
* Aponte a sonda de prontidão para `GET /readyz` e a sonda de vivacidade para `GET /healthz`

<Note>
  **Identidade de carga de trabalho**

  Prefira a identidade de carga de trabalho da plataforma em relação a chaves estáticas: IRSA no EKS para Bedrock e para Claude Platform na AWS, Workload Identity no GKE para Agent Platform e identidade de carga de trabalho no AKS para Foundry. Defina `auth: {}` no bloco upstream, ou `use_azure_ad: true` para Foundry, e o gateway pega a identidade do pod através da cadeia de credencial padrão desse provedor. Para um emparelhamento entre nuvens, como um upstream Bedrock no GKE, defina credenciais explícitas no bloco `auth` do upstream. A [referência `upstreams`](/docs/pt/claude-apps-gateway-config#upstreams) tem detalhes de configuração por plataforma.
</Note>

<h3 id="cloud-run">
  Cloud Run
</h3>

Configure o serviço da seguinte forma:

* Deixe `listen.port` em seu padrão de `8080`, que corresponde ao `PORT` padrão do Cloud Run, ou defina `port: ${PORT}`
* Defina `public_url` para a origem externamente alcançável. Para produção, isso normalmente é o nome de host de um balanceador de carga interno, porque `/login` [rejeita endereços públicos](/docs/pt/claude-apps-gateway#prerequisites) e a URL `*.run.app` se resolve para um, então a URL do Cloud Run sozinha funciona apenas para um teste de fumaça `curl` ou navegador. A exceção é uma rede onde `*.run.app` se resolve privadamente através do Private Service Connect e uma zona privada do Cloud DNS; nessa topologia a URL do Cloud Run é um `public_url` válido. O [exemplo trabalhado do Google Cloud](/docs/pt/claude-apps-gateway-on-gcp#deploy-the-gateway) cobre ambos.
* Monte a configuração como um volume secreto
* Defina `min-instances: 1` para evitar descoberta OIDC fria na primeira solicitação

<Note>
  Para um exemplo trabalhado completo no Google Cloud, cobrindo Cloud Run ou GKE, Cloud SQL e Secret Manager, consulte [Implantar no Google Cloud](/docs/pt/claude-apps-gateway-on-gcp).
</Note>

<h3 id="push-the-gateway-url-to-developer-machines">
  Envie a URL do gateway para máquinas de desenvolvedores
</h3>

Assim que o gateway estiver servindo, envie `forceLoginMethod` e `forceLoginGatewayUrl` para a máquina de cada desenvolvedor através de configurações gerenciadas, via MDM ou escrevendo o `managed-settings.json` por SO diretamente. Sem isso, `/login` mostra o seletor de conta padrão sem opção de gateway. Consulte [Configurações gerenciadas do lado do cliente](/docs/pt/claude-apps-gateway-config#client-side-managed-settings) para os caminhos de arquivo.

<h2 id="operations">
  Operações
</h2>

Assim que o gateway estiver servindo tráfego, a operação do dia a dia é ler seus logs, sondar sua saúde e girar seus segredos em seu cronograma. As subseções cobrem cada uma, mais o que o Postgres mantém e como atualizações e reversões se comportam.

<h3 id="logs">
  Logs
</h3>

O gateway escreve dois fluxos para stderr, ambos amigáveis a JSON:

* **Eventos de auditoria**: JSON de linha única por evento relevante para segurança. Canalize stderr para seu agregador de logs. Os eventos emitidos incluem `config.load`, `session.mint`, `session.refresh`, `device.authorize`, `device.verify`, `auth.denied`, `access.denied`, `inference`, `managed.serve`, `spend.blocked` e `admin.denied`. Os campos variam por evento:
  * Eventos de mint e refresh bem-sucedidos carregam `sub`, `email`, `client_ip` e o resultado
  * Eventos de negação carregam o motivo, caminho e IP do cliente, já que nenhuma identidade existe na negação
  * `inference` registra qual upstream serviu a solicitação e o status da resposta
  * `admin.denied` registra uma tentativa de autenticação de API de administrador rejeitada com o motivo (`invalid_key` ou `no_credentials`), IP do cliente, método e caminho, sem o material de chave apresentado
* **Logs operacionais**: linhas legíveis por humanos com prefixo `[gateway]` para inicialização, avisos e erros upstream. A variável de ambiente `CLAUDE_GATEWAY_LOG_LEVEL` controla a verbosidade e aceita `info`, `warn` ou `error`, com `info` como padrão. Não afeta eventos de auditoria, que são sempre emitidos.

<h3 id="health">
  Saúde
</h3>

O gateway serve `GET /healthz` como uma sonda de vivacidade e `GET /readyz` como uma sonda de prontidão; `/readyz` verifica se o armazenamento é alcançável. Ambos estão isentos de `access_control.allow_cidrs`, então as sondas continuam funcionando em um listener bloqueado.

O documento de descoberta OAuth em `/.well-known/oauth-authorization-server` também retorna `200` apenas após carregamento de configuração, descoberta OIDC, construção de cliente upstream e migração do Postgres, então funciona como uma verificação de inicialização de ponta a ponta.

Um gateway em execução também serve uma descrição dos caminhos e formas de solicitação que aceita em `<public_url>/protocol`, correspondida à versão que você está executando. O conteúdo não é estável entre versões.

<h3 id="outage-behavior">
  Comportamento de interrupção
</h3>

Se o Postgres cair, o gateway em si continua servindo desenvolvedores conectados e novas entradas falham. Se os desenvolvedores realmente continuam funcionando depende de como seu orquestrador lida com prontidão:

* **Sessões existentes**: tokens portadores validam localmente com o segredo JWT, atualizações de sessão não tocam o armazenamento e o processo do gateway ainda pode servir inferência
* **Novas entradas**: falham até que o Postgres se recupere, porque o fluxo de dispositivo e seus contadores de limite de taxa vivem no Postgres
* **[Aplicação de limite de gastos](/docs/pt/claude-apps-gateway-spend-limits#postgres-availability)**: falha aberta por padrão durante a interrupção, então a inferência ainda flui; inverta para falha fechada se preferir bloquear do que executar sem medição
* **Prontidão**: `/readyz` relata não-pronto durante a interrupção, então orquestradores que controlam tráfego na prontidão removem cada réplica da rotação de uma vez. Nessa topologia todo tráfego, incluindo inferência que o gateway ainda poderia servir, falha no balanceador de carga até que o Postgres se recupere. A sonda de vivacidade em `/healthz` continua passando, então as réplicas não são reiniciadas. Aponte a sonda de prontidão para `/healthz` em vez disso se preferir que desenvolvedores conectados continuem funcionando através de uma interrupção de armazenamento; o custo é que novas entradas falham contra uma réplica que ainda relata pronto.

Se seu IdP cair, as sessões existentes funcionam até `ttl_hours`, e novas entradas e atualizações falham. Defina um `ttl_hours` mais longo se seu IdP tiver janelas de manutenção frequentes.

<h3 id="jwt-secret-rotation">
  Rotação de segredo JWT
</h3>

Gire o segredo de assinatura em três etapas para que as sessões existentes permaneçam válidas:

1. Gere um novo segredo. Coloque-o no início da matriz `session.jwt_secret`.
2. Implante a implantação. Novos tokens assinam com o novo segredo; tokens antigos ainda verificam.
3. Após `ttl_hours` mais uma margem, remova o segredo antigo e implante novamente.

A rotação também é a única maneira de forçar sessões para fora antes de expirarem: tokens portadores validam localmente contra o segredo JWT, então não há revogação por sessão. Substituir o segredo completamente, sem manter o antigo na matriz, invalida cada sessão pendente de uma vez. Para offboarding individual, desprovision o usuário em seu IdP; sua sessão termina dentro de `ttl_hours`.

<h3 id="postgres">
  Postgres
</h3>

O gateway mantém cinco tabelas, todas criadas por suas migrações de tempo de inicialização:

| Tabela             | Conteúdo                                                                                      | Retenção                                                          |
| ------------------ | --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `kv`               | Concessões de dispositivo (TTL de 10 minutos) e contadores de limite de taxa                  | TTL por linha                                                     |
| `spend`            | Contadores de gastos período-até-data por principal, em centavos                              | `admin.spend_retention_months`, padrão 13                         |
| `spend_limits`     | Limites de gastos configurados                                                                | Até deletado via API                                              |
| `admin_audit`      | Trilha de mutação de API de administrador                                                     | `admin.audit_retention_days`, padrão 365                          |
| `principal_emails` | Email, nome de exibição e grupos de IdP de cada principal vistos pela última vez. Contém PII. | `admin.identity_retention_days` desde última atividade, padrão 90 |

Um loop de 30 segundos expira linhas `kv` após seu TTL, e uma varredura horária impõe as janelas de retenção nas tabelas de gastos, então nada cresce sem limite. Sem [limites de gastos](/docs/pt/claude-apps-gateway-spend-limits) configurados, apenas `kv` é escrito. Se sua política de segurança proíbe DDL da função de aplicativo, pré-crie essas tabelas e `_migrations` com uma função de administrador e conceda à função de aplicativo `SELECT, INSERT, UPDATE, DELETE` em cada uma.

Com limites de gastos em uso, um banco de dados perdido significa rastreamento de gastos e limites perdidos, não apenas re-entradas de desenvolvedores, então execute backups regulares. Para apagar um desenvolvedor que partiu imediatamente em vez de esperar pela retenção, execute `DELETE FROM principal_emails WHERE principal = '<sub>'` diretamente; isso remove a única tabela que mantém seu email, nome e grupos. Linhas `spend` e `admin_audit` referenciam apenas o `sub` OIDC pseudônimo.

<h3 id="upgrades">
  Atualizações
</h3>

As réplicas são sem estado, então uma reinicialização contínua é segura a qualquer momento. O gateway executa migrações de esquema na inicialização, o que significa que implantar o novo binário auto-migra o banco de dados. Se a função de banco de dados não conseguir executar DDL, pré-crie o esquema, incluindo a tabela `_migrations` semeada para a versão atual; caso contrário, a inicialização falha ao tentar `CREATE TABLE`.

As migrações são apenas anexadas, então reverter para um binário anterior que conhece menos migrações é seguro; ele ignora as linhas extras. A reversão também re-valida o YAML contra o esquema do binário mais antigo, então uma configuração que adotou uma chave introduzida pela versão mais nova falha na inicialização no mais antigo. Remova a nova chave antes de reverter.

Como você fixa a versão do gateway em sua própria imagem, correções em novas versões do Claude Code, incluindo correções de segurança, chegam à sua implantação apenas quando você atualiza o pino e reimplanta. Inclua o gateway no mesmo ciclo de patch que você usa para outros serviços que mantêm credenciais de produção.

<h2 id="security">
  Segurança
</h2>

Esta seção responde às perguntas que uma revisão de segurança faz: quais dados fluem através do gateway e para onde vão, quais ataques o design se defende e quais respostas pertencem a um questionário de conformidade.

<h3 id="data-flow">
  Fluxo de dados
</h3>

| Dados                                                                                                       | Caminho                                                        | Enviado para Anthropic pelo gateway                   |
| ----------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------- |
| Inferência (prompts, conclusões)                                                                            | CLI → gateway → seu upstream                                   | Apenas se a API Anthropic for um upstream configurado |
| Telemetria (métricas OTLP, mais [logs e rastreamentos opcionais](/docs/pt/claude-apps-gateway-config#telemetry)) | CLI → gateway → seu coletor                                    | Nunca                                                 |
| Identidade (email, grupos, sub)                                                                             | IdP → gateway → JWT → CLI; o CLI o carimba em exportações OTLP | Nunca                                                 |
| Configurações gerenciadas                                                                                   | Seu YAML de gateway → CLI                                      | Nunca                                                 |
| Log de auditoria                                                                                            | Gateway stderr → seu agregador                                 | Nunca                                                 |

<h3 id="threat-model-summary">
  Resumo do modelo de ameaça
</h3>

O gateway fica dentro do perímetro de rede, mas laptops de desenvolvedores individuais não são tratados como confiáveis. O design leva isso em conta de três maneiras:

* Os desenvolvedores mantêm JWTs de curta duração em vez de chaves upstream brutas. A perna CLI-para-gateway usa a concessão de dispositivo RFC 8628, e a troca de código de autorização do gateway com o IdP executa PKCE na configuração padrão, então um código de autorização IdP interceptado é inútil.
* A página de verificação de dispositivo impõe POST de mesma origem e um limite de taxa por IP por RFC 8628 §5.1. Consulte [Resistência de força bruta de código de usuário](#user-code-brute-force-resistance).
* Solicitações de saída passam por uma proteção de falsificação de solicitação do lado do servidor (SSRF) que resolve DNS, bloqueia endereços link-local e metadados de nuvem mais loopback por padrão e fixa a conexão ao IP resolvido, então URLs influenciadas pelo operador como o IdP e destinos OTLP não podem ser redirecionados para endpoints de metadados de nuvem. Intervalos privados RFC 1918 são deliberadamente permitidos, porque IdPs e coletores OTLP comumente vivem em IPs privados. Para desenvolvimento local contra um IdP de loopback ou coletor, defina `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` no ambiente do gateway; deixe desconfigurado em produção.

Se você adicionar seus próprios controles de saída, o gateway deve alcançar o servidor de metadados sempre que usar credenciais de metadados de instância como identidade de carga de trabalho.

Duas ameaças estão fora do escopo porque são sua infraestrutura para proteger:

* **Um host de gateway comprometido**: o host mantém a credencial upstream e distribui [configurações gerenciadas](/docs/pt/claude-apps-gateway-config#managed) para cada desenvolvedor conectado, então o controle sobre a configuração do gateway é comparável ao controle sobre seu MDM. O diálogo de aprovação única do CLI para configurações capazes de shell limita mudanças silenciosas, mas não substitui a segurança do host.
* **Um provedor OIDC malicioso**: o provedor assina os id\_tokens que o gateway confia, então pode afirmar qualquer identidade. Verificar e proteger seu IdP é sua responsabilidade.

<h3 id="user-code-brute-force-resistance">
  Resistência de força bruta de código de usuário
</h3>

O `user_code` que um desenvolvedor digita na página de verificação `/device` tem 8 caracteres extraídos de um alfabeto de 20 caracteres, o que produz 20⁸ ou cerca de 2,56×10¹⁰ combinações, e expira após 10 minutos.

O gateway aplica limites de taxa por IP nos endpoints de concessão de dispositivo, configuráveis via [`rate_limits`](/docs/pt/claude-apps-gateway-config#http-tuning). Aumente os limites se muitos desenvolvedores entrarem de um único endereço NAT corporativo compartilhado. Os limites se aplicam apenas ao fluxo de entrada, não à inferência.

<h3 id="compliance-posture">
  Postura de conformidade
</h3>

* **Residência de dados**: o plano de dados do próprio gateway não envia nada para Anthropic a menos que a API Anthropic seja um upstream configurado; quando é, seu acordo de tratamento de dados existente se aplica ao caminho de inferência. Telemetria, auditoria, identidade e configurações vão apenas para os destinos que você configura.
* **Tráfego de processo de host**: o processo de host é o CLI do Claude Code, que pode enviar análises de inicialização e verificações de atualização para Anthropic. Para implantações de saída estrita, defina `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` no ambiente do contêiner do gateway.
* **Análises do cliente**: o CLI desativa sua própria análise de uso enquanto conectado a um gateway, e o relatório de erros está desativado por padrão em superfícies de API de terceiros.
* **Máquinas do cliente**: CLIs de desenvolvedores ainda enviam verificações de nome de host WebFetch e verificações de versão para Anthropic a menos que `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` e `skipWebFetchPreflight: true` sejam definidos. Consulte [uso de dados](/docs/pt/data-usage).
* **Classificações de pesquisa**: a credencial do gateway desativa o coletor vinculado a Anthropic, então as classificações não são enviadas para Anthropic.
* **Compartilhamento de transcrição**: escolher Sim em um prompt de compartilhamento de transcrição de pesquisa escreve um arquivo local em `~/.claude/feedback-bundles/` em vez de fazer upload para Anthropic.
* **Atualizações do cliente**: verificações de atualização são separadas do tráfego do gateway. Fixe versões através de sua própria distribuição e defina `DISABLE_UPDATES` se laptops não devem buscar versões. `DISABLE_AUTOUPDATER` para apenas atualizações de fundo enquanto `claude update` ainda funciona.
* **TLS**: sirva `public_url` sobre HTTPS em produção, seja do próprio listener do gateway via `listen.tls` ou de um ingress que termina TLS na frente de réplicas HTTP simples com `listen.public_url` definido. O gateway não recusa HTTP simples. O IdP deve servir HTTPS em produção, e o Postgres suporta `?sslmode=require`. Defina `Strict-Transport-Security` em seu ingress.
* **Divulgação de vulnerabilidade**: siga [Relatando problemas de segurança](/docs/pt/security#reporting-security-issues)

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Para perguntas e feedback, use [Suporte do Claude Code](https://support.claude.com/en/collections/14445694-claude-code), ou abra um problema no [repositório GitHub do Claude Code](https://github.com/anthropics/claude-code/issues). Ao relatar um problema, inclua:

* **Problema do gateway**: o stderr do gateway para a janela relevante, seu `gateway.yaml` com segredos redigidos, a versão do gateway, mostrada na página de destino em `/` e no cabeçalho de resposta `x-cc-gateway-version` em `/managed/settings`, e o que mudou recentemente
* **Problema de entrada**: o desenvolvedor executa `claude --debug-file ./claude-debug.txt`, reproduz e envia esse arquivo mais o log de auditoria do gateway para a mesma janela
* **Problema de inferência**: o modelo solicitado, os upstreams configurados e o log de auditoria do gateway para a solicitação, que registra qual upstream o serviu e o status da resposta

O stderr do gateway inclui o fluxo de eventos de auditoria, o log de auditoria registra identidades de desenvolvedores, e o arquivo de debug registra saída de hook e servidor MCP da máquina do desenvolvedor. Revise e remova essas informações antes de postar em uma issue pública.

| Sintoma                                                                                                                                                                       | Causa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Correção                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A `/login` de um desenvolvedor mostra o seletor de conta padrão em vez da tela **Cloud gateway**                                                                              | `forceLoginMethod` ou `forceLoginGatewayUrl` não está definido em configurações gerenciadas nessa máquina                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Implante o [arquivo de configurações gerenciadas](/docs/pt/claude-apps-gateway#set-the-gateway-url) no dispositivo; `/login` lê a URL do gateway de lá                                                                                                                                                                                                                                                                                                                                                                     |
| A inicialização mostra `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.`                                  | A compilação do Claude Code instalada é anterior ao suporte do gateway                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Peça ao desenvolvedor para atualizar o Claude Code para uma versão que inclua suporte do Cloud gateway                                                                                                                                                                                                                                                                                                                                                                                                                |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>`                                    | O nome de host do gateway se resolve para pelo menos um endereço IP público. Claude Code verifica cada endereço resolvido e requer que cada um seja privado. Uma causa comum é um nome de pilha dupla onde uma família se resolve para um endereço público, incluindo balanceadores de carga de pilha dupla internos da AWS, que retornam endereços AAAA de intervalo público. Os endpoints de gateway público operados pela Anthropic estão isentos da verificação, e `/login` os aceita sobre `https://`. Antes da v2.1.206, `/login` os rejeitava como qualquer outro endereço público | Faça o nome do gateway se resolver apenas para endereços privados em máquinas de desenvolvedores. Para um nome de pilha dupla, solte o registro de intervalo público ou sirva um nome DNS apenas interno separado. Consulte o [pré-requisito de rede privada](/docs/pt/claude-apps-gateway#prerequisites).                                                                                                                                                                                                                 |
| CLI `/login`: `Gateway login requires a direct connection and does not support connecting through an HTTP proxy`                                                              | Um `HTTPS_PROXY` ou `HTTP_PROXY` se aplica ao host do gateway e o nome de host do proxy se resolve para um endereço público. Um proxy cujo host se resolve apenas para endereços privados é permitido e não dispara esse erro                                                                                                                                                                                                                                                                                                                                                             | Adicione o host do gateway a `NO_PROXY` na máquina do desenvolvedor para que a conexão seja direta, ou use um proxy cujo nome de host se resolve para endereços privados                                                                                                                                                                                                                                                                                                                                              |
| CLI `/login`: `Could not resolve gateway host <host>`                                                                                                                         | A máquina não consegue resolver o nome DNS interno do gateway, normalmente porque não está na rede corporativa                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Peça ao desenvolvedor para se conectar à sua rede ou VPN e tente `/login` novamente                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| A inicialização sai com um erro de validação de configuração nomeando `store.postgres_url`                                                                                    | Nenhum Postgres configurado; o gateway requer Postgres                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Defina `store.postgres_url`. Para desenvolvimento local, use um contêiner descartável: `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.                                                                                                                                                                                                                                                                                                                                                    |
| A inicialização sai: `requires the native binary`                                                                                                                             | Executando sob Node em vez do binário nativo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Instale o Claude Code com um dos [métodos de instalação autônoma](/docs/pt/setup)                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| A inicialização sai com um erro de descoberta OIDC após `config.load`                                                                                                         | `oidc.issuer` inacessível, ou cadeia TLS não confiável                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Verifique se o emissor é alcançável do pod e serve `/.well-known/openid-configuration`. Defina `ca_cert_pem` para PKI privada.                                                                                                                                                                                                                                                                                                                                                                                        |
| A inicialização sai com um erro de permissão do Postgres                                                                                                                      | A função de aplicativo carece de `CREATE TABLE`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Pré-crie o esquema com uma função de administrador e conceda DML à função de aplicativo, ou conceda DDL temporariamente para inicializações que aplicam novas migrações                                                                                                                                                                                                                                                                                                                                               |
| `/oauth/callback` mostra "Sign-in could not be completed"                                                                                                                     | Domínio de email rejeitado, validação de id\_token falhou, ou `email_verified` é explicitamente `false`, que o gateway sempre rejeita sem substituição                                                                                                                                                                                                                                                                                                                                                                                                                                    | Verifique `allowed_email_domains` e que o IdP retorna uma reivindicação `email` verificada. Para `email_verified: false`, corrija a verificação do lado do IdP. Se seu IdP emite email sob um nome de reivindicação diferente, defina `oidc.email_claim`.                                                                                                                                                                                                                                                             |
| Log: `token exchange failed: id_token missing email claim`                                                                                                                    | O IdP não está incluindo `email` no id\_token por padrão. Esta rejeição dispara apenas quando `allowed_email_domains` está definido; sem ele, um email ausente cunha uma sessão sem email                                                                                                                                                                                                                                                                                                                                                                                                 | Configure o IdP para emitir `email` no id\_token. Okta: adicione `email` às reivindicações de token de ID de um servidor de autorização personalizado. Entra: adicione `email` como uma reivindicação opcional no registro do aplicativo. PingFederate: ative uma Política OpenID Connect que emite `email`. Se o IdP serve `email` do endpoint userinfo mas não o incluirá no id\_token, como o servidor de autorização da organização Okta, defina `oidc.userinfo_fallback: true`.                                  |
| Cada solicitação do Amazon Bedrock retorna 502; o log mostra `Could not load credentials from any providers`                                                                  | No EC2, o hop limit padrão do IMDSv2 de 1 bloqueia a solicitação de metadados de instância de dentro do contêiner. A inicialização e `/readyz` passam mesmo assim porque o AWS SDK resolve credenciais de instância na primeira solicitação, não na construção do cliente                                                                                                                                                                                                                                                                                                                 | Aumente o hop limit com `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`, ou defina-o no modelo de lançamento. A mudança se aplica a cada contêiner na instância. Prefira funções de tarefa ECS onde disponível, que leem credenciais do endpoint de credenciais do contêiner ECS e evitam a mudança completamente, ou aplique a mudança em uma instância de gateway dedicada para limitar a exposição.                                                                  |
| Erro do IdP: escopo desconhecido ou não suportado                                                                                                                             | O IdP rejeita escopos que não reconhece                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Defina `oidc.scopes` para exatamente a lista que seu IdP aceita; deve incluir `openid`. O padrão é `openid profile email offline_access`.                                                                                                                                                                                                                                                                                                                                                                             |
| As sessões não se renovam silenciosamente após definir `oidc.scopes`                                                                                                          | `offline_access` foi removido da substituição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Adicione `offline_access` de volta se seu IdP o suportar. Sem um token de atualização, os desenvolvedores executam novamente o login do navegador a cada `session.ttl_hours`.                                                                                                                                                                                                                                                                                                                                         |
| O navegador mostra "This request came from another site and was blocked"                                                                                                      | POST de formulário entre sites, bloqueado como proteção CSRF. Esperado para páginas incorporadas ou proxied                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Abra o link de verificação diretamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Chrome bloqueia o botão Approve com "Refused to send form data … violates … Content Security Policy directive: form-action", mas a mesma página funciona no Safari ou Firefox | Chrome impõe `form-action` contra toda a cadeia de redirecionamento. Seu IdP redireciona para um segundo host que não está na lista de permissões.                                                                                                                                                                                                                                                                                                                                                                                                                                        | Adicione cada origem adicional na cadeia de redirecionamento a `oidc.form_action_origins`. Abra Chrome DevTools → Console na página Approve para ver qual origem foi bloqueada.                                                                                                                                                                                                                                                                                                                                       |
| A entrada é concluída no IdP, mas o callback falha, com um erro de CSP no Chrome ou "this sign-in link has expired" no Safari                                                 | O IdP retornou o código via `response_mode=form_post`, que o auto-envia entre origens via POST para `/oauth/callback`. Chrome bloqueia isso sob um CSP estrito; Safari permite o envio, mas o callback lê apenas a string de consulta.                                                                                                                                                                                                                                                                                                                                                    | Certifique-se de que seu IdP honra `response_mode=query`, que o gateway solicita explicitamente para que o callback seja um redirecionamento simples                                                                                                                                                                                                                                                                                                                                                                  |
| O login funciona localmente, mas falha atrás de um ALB                                                                                                                        | `public_url` não definido, então o IdP obtém a origem interna `http://` como `redirect_uri`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Defina `listen.public_url` para a origem externa `https://`                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| O desenvolvedor vê o prompt de confiança repetidamente                                                                                                                        | O certificado TLS está girando por réplica ou por solicitação                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Use um certificado estável no ingress, ou termine TLS uma vez e execute réplicas sobre HTTP simples internamente                                                                                                                                                                                                                                                                                                                                                                                                      |
| CLI `/login`: "Could not verify the gateway's TLS certificate" ou `SELF_SIGNED_CERT_IN_CHAIN`                                                                                 | A cadeia TLS do gateway é assinada por uma CA privada não no armazenamento de confiança do host CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Claude Code lê o armazenamento de confiança do SO por padrão no binário nativo e no Node 22.15 ou posterior; [`CLAUDE_CODE_CERT_STORE`](/docs/pt/network-config#ca-certificate-store) controla esse comportamento. Se a CA está instalada no armazenamento de confiança do SO, certifique-se de que os desenvolvedores estão em um tempo de execução atual. Caso contrário, defina `NODE_EXTRA_CA_CERTS` para o PEM do certificado CA antes de iniciar. O prompt de impressão digital de primeira conexão ainda se aplica. |

<h2 id="related">
  Relacionado
</h2>

* [Visão geral do gateway de aplicativos Claude](/docs/pt/claude-apps-gateway): início rápido e conexão de desenvolvedor
* [Referência de configuração](/docs/pt/claude-apps-gateway-config): cada opção `gateway.yaml`
