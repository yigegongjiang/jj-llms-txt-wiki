> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configure Claude Code para sua organização

> Um mapa de decisão para administradores que implantam Claude Code, cobrindo provedores de API, configurações gerenciadas, aplicação de políticas, monitoramento de uso e tratamento de dados.

Claude Code aplica a política da organização através de configurações gerenciadas que têm precedência sobre a configuração local do desenvolvedor. Você entrega essas configurações a partir do console de administração Claude, seu sistema de gerenciamento de dispositivos móveis (MDM) ou um arquivo no disco. As configurações controlam quais ferramentas, comandos, servidores e destinos de rede Claude pode alcançar.

Esta página percorre as decisões de implantação em ordem. Cada linha vincula à seção abaixo e à página de referência para essa área.

<Note>
  SSO, provisionamento SCIM e atribuição de assentos são configurados no nível da conta Claude. Consulte o [Guia do Administrador Empresarial Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) e [atribuição de assentos](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) para essas etapas.
</Note>

| Decisão                                                                                    | O que você está escolhendo                                       | Referência                                                                                                                                                                    |
| :----------------------------------------------------------------------------------------- | :--------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Escolha seu provedor de API](#choose-your-api-provider)                                   | Onde Claude Code autentica e como é cobrado                      | [Authentication](/docs/pt/authentication), [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai), [Microsoft Foundry](/docs/pt/microsoft-foundry) |
| [Decida como as configurações chegam aos dispositivos](#decide-how-settings-reach-devices) | Como a política gerenciada chega às máquinas dos desenvolvedores | [Server-managed settings](/docs/pt/server-managed-settings), [Settings files](/docs/pt/settings#settings-files)                                                                         |
| [Decida o que aplicar](#decide-what-to-enforce)                                            | Quais ferramentas, comandos e integrações são permitidas         | [Permissions](/docs/pt/permissions), [Sandboxing](/docs/pt/sandboxing)                                                                                                                  |
| [Configure a visibilidade de uso](#set-up-usage-visibility)                                | Como você rastreia gastos e adoção                               | [Analytics](/docs/pt/analytics), [Monitoring](/docs/pt/monitoring-usage), [Costs](/docs/pt/costs)                                                                                            |
| [Revise o tratamento de dados](#review-data-handling)                                      | Retenção de dados e postura de conformidade                      | [Data usage](/docs/pt/data-usage), [Security](/docs/pt/security)                                                                                                                        |

<h2 id="choose-your-api-provider">
  Escolha seu provedor de API
</h2>

Claude Code se conecta ao Claude através de um dos vários provedores de API. Sua escolha afeta faturamento, autenticação, qual postura de conformidade você herda e quais recursos do Claude Code seus desenvolvedores podem usar.

| Provedor                      | Escolha isto quando                                                                                                                      |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| Claude for Teams / Enterprise | Você quer Claude Code e claude.ai sob uma assinatura por assento com nenhuma infraestrutura para executar. Esta é a recomendação padrão. |
| Claude Console                | Você é API-first ou quer faturamento pay-as-you-go                                                                                       |
| Amazon Bedrock                | Você quer herdar controles de conformidade e faturamento AWS existentes                                                                  |
| Google Cloud's Agent Platform | Você quer herdar controles de conformidade e faturamento GCP existentes                                                                  |
| Microsoft Foundry             | Você quer herdar controles de conformidade e faturamento Azure existentes                                                                |

Alguns recursos do Claude Code exigem uma conta claude.ai. [Claude Code on the web](/docs/pt/claude-code-on-the-web), [Routines](/docs/pt/routines), [Code Review](/docs/pt/code-review), [Remote Control](/docs/pt/remote-control) e a [Chrome extension](/docs/pt/chrome) não estão disponíveis apenas através de chaves da API Console ou credenciais de provedor de nuvem. Se você implantar através de Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, planeje se os desenvolvedores também precisam de assentos Claude for Teams ou Enterprise. Cada página de recurso lista seus requisitos de plano.

Para a comparação completa do provedor cobrindo autenticação, regiões e paridade de recursos, consulte a [visão geral de implantação empresarial](/docs/pt/third-party-integrations). A configuração de autenticação de cada provedor está em [Authentication](/docs/pt/authentication).

Os requisitos de proxy e firewall em [Network configuration](/docs/pt/network-config) se aplicam independentemente do provedor. Se você quiser um único endpoint na frente de vários provedores ou registro de solicitações centralizado, consulte [LLM gateway](/docs/pt/llm-gateway).

<h2 id="decide-how-settings-reach-devices">
  Decida como as configurações chegam aos dispositivos
</h2>

As configurações gerenciadas definem a política que tem precedência sobre a configuração local do desenvolvedor. Claude Code verifica as quatro fontes abaixo em ordem de prioridade e aplica a primeira que retorna uma configuração não vazia, com uma exceção: um pequeno conjunto de [chaves de bloqueio entre fontes](/docs/pt/settings#settings-precedence), como os bloqueios da lista de permissões de sandbox, é honrado quando qualquer fonte controlada por administrador os define.

| Mecanismo               | Entrega                                                                                                                                                                                             | Prioridade | Plataformas    |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------- | :------------- |
| Server-managed          | Console de administração claude.ai, ou um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado para sign-ins de gateway                                                          | Mais alta  | Todas          |
| plist / registry policy | macOS: `com.anthropic.claudecode` plist<br />Windows: `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                           | Alta       | macOS, Windows |
| File-based managed      | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux e WSL: `/etc/claude-code/managed-settings.json`<br />Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | Média      | Todas          |
| Windows user registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                 | Mais baixa | Apenas Windows |

Um [`policyHelper`](/docs/pt/settings#compute-managed-settings-with-a-policy-helper) configurado antecede todas as quatro fontes: sua saída se torna a única configuração gerenciada para a execução. Consulte [Settings precedence](/docs/pt/settings#settings-precedence).

As configurações gerenciadas pelo servidor chegam aos dispositivos no momento da autenticação e são atualizadas a cada hora durante sessões ativas, sem infraestrutura de endpoint. A entrega através do console de administração claude.ai requer um plano Claude for Teams ou Enterprise. Implantações em Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry podem obter a mesma entrega remota executando um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), ou usar um dos mecanismos baseados em arquivo ou de nível do SO.

Se sua organização mistura provedores, configure [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) para usuários claude.ai mais um [fallback baseado em arquivo ou plist/registry](/docs/pt/settings#settings-files) para que outros usuários ainda recebam política gerenciada.

Os locais de registro plist e HKLM funcionam com qualquer provedor e resistem a adulteração porque exigem privilégios de administrador para escrever. O registro de usuário do Windows em HKCU é gravável sem elevação, portanto, trate-o como um padrão de conveniência em vez de um canal de aplicação.

Por padrão, WSL lê apenas o caminho do arquivo Linux em `/etc/claude-code`. Para estender sua política de registro do Windows e `C:\Program Files\ClaudeCode` para WSL na mesma máquina, defina [`wslInheritsWindowsSettings: true`](/docs/pt/settings#available-settings) em uma das fontes do Windows somente para administrador.

Qualquer que seja o mecanismo escolhido, os valores gerenciados têm precedência sobre as configurações de usuário e projeto. As configurações de matriz, como `permissions.allow` e `permissions.deny`, mesclam entradas de todas as fontes, portanto, os desenvolvedores podem estender listas gerenciadas, mas não removê-las. Para [duas exceções](/docs/pt/settings#settings-precedence), `fallbackModel` e `availableModels`, o valor gerenciado substitui camadas inferiores em vez de mesclar.

Consulte [Server-managed settings](/docs/pt/server-managed-settings) e [Settings files and precedence](/docs/pt/settings#settings-files).

<h3 id="wsl-sessions-in-claude-code-desktop">
  Sessões WSL no Claude Code Desktop
</h3>

No Windows, [Claude Code Desktop pode executar sessões de Code dentro de uma distribuição WSL 2](/docs/pt/desktop-wsl). O processo Claude Code da sessão é executado dentro da distribuição, portanto, resolve as configurações gerenciadas através do caminho de descoberta WSL acima: fontes somente do Windows não o alcançam a menos que `wslInheritsWindowsSettings: true` seja implantado.

Em dispositivos onde as configurações gerenciadas estão presentes, as sessões WSL do Desktop estão indisponíveis por padrão. Se sua organização deseja habilitá-las, entre em contato com sua equipe de conta Anthropic. Quando estiverem habilitadas:

* Implante `wslInheritsWindowsSettings: true` através do registro HKLM ou do arquivo `C:\Program Files\ClaudeCode` para que as sessões WSL herdem a mesma política que as sessões do host.
* Verifique executando `/status` dentro de uma sessão WSL: a linha `Setting sources` deve mostrar `Enterprise managed settings` com a fonte do Windows que você implantou, `(HKLM)` ou `(file)`.

Os processos dentro da VM utilitária WSL 2 não são visíveis para os sensores de detecção de endpoint do lado do Windows. Se você usar CrowdStrike Falcon, ative o sensor Falcon para Linux no WSL 2 com as duas exclusões que a documentação WSL do CrowdStrike exige, para o processo da máquina virtual WSL e a imagem de disco da VM, para que a atividade de processo e arquivo dentro da distribuição seja observável. A [telemetria de execução de ferramentas OpenTelemetry](/docs/pt/monitoring-usage) do Claude Code é emitida de forma idêntica para sessões WSL e nativas.

<h2 id="decide-what-to-enforce">
  Decida o que aplicar
</h2>

As configurações gerenciadas podem bloquear ferramentas, execução de sandbox, restringir servidores MCP e fontes de plugins, e controlar quais hooks são executados. Cada linha é uma superfície de controle com as chaves de configuração que a controlam.

| Controle                                                                               | O que faz                                                                                                                                                                                                                                                                              | Configurações-chave                                                                                                |
| :------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| [Permission rules](/docs/pt/permissions)                                                    | Permitir, perguntar ou negar ferramentas e comandos específicos                                                                                                                                                                                                                        | `permissions.allow`, `permissions.deny`                                                                            |
| [Permission lockdown](/docs/pt/permissions#managed-only-settings)                           | Apenas regras de permissão gerenciadas se aplicam; desabilitar `--dangerously-skip-permissions`                                                                                                                                                                                        | `allowManagedPermissionRulesOnly`, `permissions.disableBypassPermissionsMode`                                      |
| [Sandboxing](/docs/pt/sandboxing)                                                           | Isolamento de sistema de arquivos e rede de nível do SO com listas de permissão de domínio                                                                                                                                                                                             | `sandbox.enabled`, `sandbox.network.allowedDomains`                                                                |
| [Managed policy CLAUDE.md](/docs/pt/memory#deploy-organization-wide-claude-md)              | Instruções em toda a organização carregadas em cada sessão, não podem ser excluídas                                                                                                                                                                                                    | Arquivo no caminho da política gerenciada                                                                          |
| [MCP server control](/docs/pt/managed-mcp)                                                  | Restringir quais servidores MCP os usuários podem adicionar ou conectar, ou implantar um conjunto fixo                                                                                                                                                                                 | `allowedMcpServers`, `deniedMcpServers`, `allowManagedMcpServersOnly`, ou um arquivo `managed-mcp.json` implantado |
| [Plugin marketplace control](/docs/pt/plugin-marketplaces#managed-marketplace-restrictions) | Restringir quais fontes de marketplace os usuários podem adicionar e instalar, rejeitar os sinalizadores CLI que carregam plugins, agents e servidores MCP para uma única execução, e criar uma lista de permissão de quais plugins dos marketplaces podem ser sugeridos               | `strictKnownMarketplaces`, `blockedMarketplaces`, `disableSideloadFlags`, `pluginSuggestionMarketplaces`           |
| [Customization lockdown](/docs/pt/settings#strictpluginonlycustomization)                   | Bloquear skills, agents, hooks e servidores MCP de fontes de usuário e projeto, para que possam vir apenas de plugins ou configurações gerenciadas                                                                                                                                     | `strictPluginOnlyCustomization`                                                                                    |
| [Hook restrictions](/docs/pt/settings#hook-configuration)                                   | Apenas hooks gerenciados são carregados; restringir URLs de hook HTTP                                                                                                                                                                                                                  | `allowManagedHooksOnly`, `allowedHttpHookUrls`                                                                     |
| [Login enforcement](/docs/pt/settings#available-settings)                                   | Restringir login interativo a um método específico ou organização Anthropic. Quando definido, sessões autenticadas por `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, ou `apiKeyHelper` são bloqueadas na inicialização; sessões de provedor de nuvem não são afetadas                   | `forceLoginMethod`, `forceLoginOrgUUID`                                                                            |
| [Disable agent view](/docs/pt/agent-view#how-background-sessions-are-hosted)                | Desativar `claude agents`, `--bg`, `/background` e o supervisor sob demanda                                                                                                                                                                                                            | `disableAgentView`                                                                                                 |
| [Model restrictions](/docs/pt/model-config#restrict-model-selection)                        | `availableModels` filtra quais modelos aparecem no seletor. Adicionar `enforceAvailableModels` também restringe o modelo padrão selecionado automaticamente. Consulte [surface coverage](/docs/pt/model-config#surface-coverage) para saber como essa configuração alcança a CLI, web e IDE | `availableModels`, `enforceAvailableModels`                                                                        |
| [Version floor](/docs/pt/settings)                                                          | Impedir que a atualização automática instale abaixo de um mínimo em toda a organização                                                                                                                                                                                                 | `minimumVersion`                                                                                                   |
| [Required version range](/docs/pt/settings)                                                 | Recusar iniciar completamente quando a versão em execução está fora de um intervalo aprovado pela organização. Mais forte que `minimumVersion`, que apenas bloqueia downgrades                                                                                                         | `requiredMinimumVersion`, `requiredMaximumVersion`                                                                 |

As organizações cujos membros se autenticam através de claude.ai ou da API Anthropic também podem governar modelos sem implantar configurações: [restrições de modelo da organização](/docs/pt/model-config#organization-model-restrictions) desabilitam modelos individuais, um [modelo padrão da organização](/docs/pt/model-config#organization-default-model) define em qual modelo novas sessões começam, e [limites de esforço da organização](/docs/pt/model-config#organization-effort-limits) limitam níveis de esforço por função. Todos os três controles exigem um plano Claude Enterprise. As restrições de modelo e limites de esforço são aplicados no servidor; o modelo padrão é um ponto de partida que os usuários podem alterar, a menos que a organização o aplique. A aplicação está disponível para um conjunto limitado de organizações; pergunte ao seu time de contas Anthropic sobre disponibilidade. Nenhum desses controles alcança sessões no Amazon Bedrock, na Agent Platform do Google Cloud, no Microsoft Foundry, ou [Claude Platform on AWS](/docs/pt/claude-platform-on-aws); nesses provedores, use `availableModels` acima para restrições e a chave `model` em configurações gerenciadas para um padrão.

[Claude Code on the web](/docs/pt/claude-code-on-the-web) tem sua própria superfície de administrador: na página de ambientes de nuvem nas configurações de administrador, proprietários e administradores criam [ambientes compartilhados da organização](/docs/pt/claude-code-on-the-web#organization-shared-environments) que definem o [nível de acesso à rede](/docs/pt/claude-code-on-the-web#network-access), variáveis de ambiente e script de configuração para sessões de nuvem dos membros, e escolhem o ambiente padrão da organização.

As regras de permissão e sandboxing cobrem camadas diferentes. Negar WebFetch bloqueia a ferramenta de busca do Claude, mas se Bash for permitido, `curl` e `wget` ainda podem alcançar qualquer URL. O sandboxing fecha essa lacuna com uma lista de permissão de domínio de rede aplicada no nível do SO.

Para o modelo de ameaça que esses controles defendem, consulte [Security](/docs/pt/security).

<h2 id="set-up-usage-visibility">
  Configure a visibilidade de uso
</h2>

Escolha monitoramento com base no que você precisa relatar. Os painéis, APIs e controles de gastos diferem entre os planos Claude for Teams ou Enterprise e as organizações Claude Console, portanto, verifique a coluna Disponibilidade antes de planejar seus relatórios em torno de uma capacidade.

| Capacidade             | O que você obtém                                                                                                       | Disponibilidade                                                                                                                                                                                                                                                                            | Por onde começar                                      |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| Usage monitoring       | Exportação OpenTelemetry de sessões, ferramentas e tokens                                                              | Todos os provedores                                                                                                                                                                                                                                                                        | [Monitoring usage](/docs/pt/monitoring-usage)              |
| Analytics dashboard    | Métricas de adoção e contribuição com um placar em Teams / Enterprise; métricas de uso e gastos por usuário em Console | Teams / Enterprise em [claude.ai/analytics](https://claude.ai/analytics/claude-code), Console em [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                                                                                | [Analytics](/docs/pt/analytics)                            |
| Programmatic reporting | Dados de uso e custo por usuário em uma API                                                                            | [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) para Enterprise, [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) para Console                                                             | [Costs](/docs/pt/costs#manage-costs-for-your-organization) |
| Spend controls         | Limites de gastos e limites de taxa                                                                                    | Configurações de administrador para Teams / Enterprise, limites de espaço de trabalho para Console; em nuvens de terceiros, controles de orçamento da nuvem ou um [Claude apps gateway](/docs/pt/claude-apps-gateway) com [limites de gastos](/docs/pt/claude-apps-gateway-spend-limits) por usuário | [Costs](/docs/pt/costs#manage-costs-for-your-organization) |

Em Teams e Enterprise, os números de uso e gastos por usuário vêm do [relatório de gastos](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) nas configurações de análise da sua organização, não do painel de análise. Os provedores de nuvem expõem gastos através do AWS Cost Explorer, GCP Billing ou Azure Cost Management. Para planejar orçamentos empresariais em Claude chat, Claude Code e Cowork, consulte o [guia de consumo Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide).

<h2 id="review-data-handling">
  Revise o tratamento de dados
</h2>

Nos planos Team, Enterprise, Claude API e provedor de nuvem, Anthropic não treina modelos em seu código ou prompts. Seu provedor de API determina a retenção e postura de conformidade.

| Tópico                    | O que saber                                                                                                   | Por onde começar                               |
| :------------------------ | :------------------------------------------------------------------------------------------------------------ | :--------------------------------------------- |
| Data usage policy         | O que Anthropic coleta, quanto tempo é retido, o que nunca é usado para treinamento                           | [Data usage](/docs/pt/data-usage)                   |
| Zero Data Retention (ZDR) | Nada armazenado após a conclusão da solicitação. Disponível para contas qualificadas no Claude for Enterprise | [Zero data retention](/docs/pt/zero-data-retention) |
| Security architecture     | Modelo de rede, criptografia, autenticação, trilha de auditoria                                               | [Security](/docs/pt/security)                       |

Se você precisar de registro de auditoria em nível de solicitação ou rotear tráfego por sensibilidade de dados, coloque um gateway entre desenvolvedores e seu provedor: um [Claude apps gateway](/docs/pt/claude-apps-gateway) auto-hospedado registra um log de auditoria por solicitação com identidade IdP, ou use outro [LLM gateway](/docs/pt/llm-gateway). Para requisitos regulatórios e certificações, consulte [Legal and compliance](/docs/pt/legal-and-compliance).

<h2 id="verify-and-onboard">
  Verifique e integre
</h2>

Após configurar as configurações gerenciadas, peça a um desenvolvedor para executar `/status` dentro de Claude Code. Na aba **Status**, a linha `Setting sources` mostra `Enterprise managed settings` seguida pela fonte entre parênteses, uma de `(remote)`, `(plist)`, `(HKLM)`, `(HKCU)` ou `(file)`. Consulte [Verificar configurações ativas](/docs/pt/settings#verify-active-settings).

Compartilhe esses recursos para ajudar os desenvolvedores a começar:

* [Quickstart](/docs/pt/quickstart): passo a passo da primeira sessão da instalação ao trabalho com um projeto
* [Common workflows](/docs/pt/common-workflows): padrões para tarefas cotidianas como revisão de código, refatoração e depuração
* [Claude 101](https://anthropic.skilljar.com/claude-101) e [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action): cursos de ritmo próprio da Anthropic Academy

Para problemas de login, direcione os desenvolvedores para [solução de problemas de autenticação](/docs/pt/troubleshoot-install#login-and-authentication). As correções mais comuns são:

* Execute `/logout` e depois `/login` para trocar de contas
* Execute `claude update` se a opção de autenticação empresarial estiver faltando
* Reinicie o terminal após atualizar

Se um desenvolvedor vir "You haven't been added to your organization yet," seu assento não inclui acesso a Claude Code e precisa ser atualizado no console de administração.

<h2 id="next-steps">
  Próximas etapas
</h2>

Com o provedor e mecanismo de entrega escolhidos, passe para a configuração detalhada:

* [Server-managed settings](/docs/pt/server-managed-settings): entregar política gerenciada a partir do console de administração Claude
* [Settings reference](/docs/pt/settings): cada chave de configuração, local de arquivo e regra de precedência
* [Monorepos and large repos](/docs/pt/large-codebases): padrões de configuração por diretório para organizações implantando em um monorepo
* [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai), [Microsoft Foundry](/docs/pt/microsoft-foundry): implantação específica do provedor
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, gerenciamento de assentos e playbook de implementação
