> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Disponibilidade de recursos

> Compare quais recursos do Claude Code estão disponíveis em planos de assinatura Anthropic, Anthropic Console, Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform e Microsoft Foundry.

O CLI do Claude Code e tudo que é executado localmente funcionam de forma idêntica em todos os provedores. Para instruções de configuração por provedor, consulte a [visão geral de implantação empresarial](/docs/pt/third-party-integrations). Para ir direto ao que está faltando no seu provedor, consulte as abas [resumo por provedor](#summary-by-provider).

Nas tabelas abaixo, ✓ significa disponível, ✗ significa não disponível, e "Ver nota" vincula a uma nota de rodapé para suporte parcial. Um qualificador após ✓ restringe a disponibilidade a esse subconjunto, e "Admin-enabled" significa que o recurso está desativado até que um administrador da organização o ative.

<h2 id="availability-by-model-provider">
  Disponibilidade por provedor de modelo
</h2>

Como você se autentica determina quais recursos o Claude Code pode acessar. Para uma única lista do que está faltando no seu provedor, consulte as abas [resumo por provedor](#summary-by-provider). Para encontrar sua coluna nas tabelas:

* **Assinatura Claude**: você faz login com uma conta claude.ai no plano Pro, Max, Team ou Enterprise
* **Anthropic Console**: você se autentica com uma chave de API Anthropic
* **Amazon Bedrock**: você usa modelos Claude do catálogo de modelos Bedrock e define `CLAUDE_CODE_USE_BEDROCK`. O [endpoint Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint) (`CLAUDE_CODE_USE_MANTLE`) é coberto por esta coluna
* **Claude Platform on AWS**: você comprou Claude através do AWS Marketplace, mas chama a API Anthropic, e define `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Google Cloud's Agent Platform**: operado pelo Google; você define `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry**: operado pela Anthropic no Azure; você define `CLAUDE_CODE_USE_FOUNDRY`

<h3 id="features-available-on-every-provider">
  Recursos disponíveis em todos os provedores
</h3>

Estes funcionam em todos os provedores:

* [CLI](/docs/pt/quickstart) e [Agent SDK](/docs/pt/agent-sdk/overview)
* Extensões [VS Code](/docs/pt/vs-code) e [JetBrains](/docs/pt/jetbrains)
* [Subagents](/docs/pt/sub-agents), [hooks](/docs/pt/hooks-guide), [commands](/docs/pt/commands) e [skills](/docs/pt/skills)
* Memória [CLAUDE.md](/docs/pt/memory), [plugins](/docs/pt/plugins) e [servidores MCP](/docs/pt/mcp)
* [Checkpoints](/docs/pt/checkpointing), [sandboxing](/docs/pt/sandboxing) e [Workflows](/docs/pt/workflows)
* Métricas [OpenTelemetry](/docs/pt/monitoring-usage) e o [arquivo de configurações gerenciado](/docs/pt/settings#settings-files)

Três destes têm diferenças específicas do provedor:

* **Servidores MCP**: [conectores do claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) carregam apenas quando sua assinatura claude.ai é o método de autenticação ativo, e [busca de ferramentas](/docs/pt/mcp#configure-tool-search) está desativada por padrão no Google Cloud's Agent Platform e quando `ANTHROPIC_BASE_URL` aponta para um host não-first-party
* **Subagents**: o [Explore subagent](/docs/pt/sub-agents#built-in-subagents) integrado limita seu modelo herdado a Opus na Claude API, e herda o modelo da conversa principal diretamente em qualquer outro provedor, incluindo Claude Platform on AWS
* **[Commands](/docs/pt/commands#all-commands)**: `/design-sync` e `/radio` não estão disponíveis no Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Claude Platform on AWS, e `/voice` requer uma conta claude.ai

<h3 id="features-that-require-a-claude-subscription">
  Recursos que requerem uma assinatura Claude
</h3>

Estes requerem login com uma conta claude.ai e não são acessíveis com uma chave de API Anthropic Console ou de um provedor terceirizado:

* [Claude Code na web](/docs/pt/claude-code-on-the-web), Claude Code no celular e [Claude Code no Slack](/docs/pt/slack)
* [Claude Code Desktop](/docs/pt/desktop)
* [Routines](/docs/pt/routines) (`/schedule`)
* [Ultraplan](/docs/pt/ultraplan) e [Ultrareview](/docs/pt/ultrareview)
* [Code Review](/docs/pt/code-review): planos Team e Enterprise
* [Remote Control](/docs/pt/remote-control)
* [Extensão Chrome](/docs/pt/chrome)
* [Computer use](/docs/pt/computer-use): planos Pro e Max
* [Artifacts](/docs/pt/artifacts): planos Pro, Max, Team e Enterprise
* [Voice dictation](/docs/pt/voice-dictation)

Desktop é a exceção parcial: [roteamento de gateway pode ser configurado no aplicativo ou por um administrador](/docs/pt/llm-gateway-connect#desktop-app), implantações Enterprise podem rotear Desktop para Google Cloud's Agent Platform ou um provedor de gateway via [configurações gerenciadas](https://claude.com/docs/third-party/claude-desktop/configuration), e [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) executa a aba Code no Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou um gateway LLM auto-hospedado. Para disponibilidade por plano desses recursos, consulte [Disponibilidade por plano de assinatura](#availability-by-subscription-plan).

<h3 id="cli-capabilities-that-vary-by-provider">
  Recursos de CLI que variam por provedor
</h3>

Estes recursos funcionam no CLI local, mas dependem de uma capacidade do lado do servidor que nem todo provedor expõe.

<table>
  <thead>
    <tr>
      <th>Recurso</th>
      <th>Assinatura Claude</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Web search](/docs/pt/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>Ver nota <sup><a href="#fn1">1</a></sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[Fast mode](/docs/pt/fast-mode)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](/docs/pt/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Ver nota <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>Ver nota <sup><a href="#fn2">2</a></sup></td>
      <td>Ver nota <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/pt/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Channels](/docs/pt/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[`/loop` scheduled tasks](/docs/pt/scheduled-tasks)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Ver nota <sup><a href="#fn3">3</a></sup></td>
      <td>Ver nota <sup><a href="#fn3">3</a></sup></td>
      <td>Ver nota <sup><a href="#fn3">3</a></sup></td>
      <td>Ver nota <sup><a href="#fn3">3</a></sup></td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/pt/github-actions) e [GitLab CI/CD](/docs/pt/gitlab-ci-cd)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

<h3 id="admin-and-analytics">
  Admin e análise
</h3>

Controles no nível da organização e visibilidade de uso.

<table>
  <thead>
    <tr>
      <th>Recurso</th>
      <th>Assinatura Claude</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Analytics dashboard e API](/docs/pt/analytics)</td>
      <td>✓ (dashboard: Team e Enterprise; API: Enterprise)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Server-managed settings](/docs/pt/server-managed-settings)</td>
      <td>✓ (Team e Enterprise)</td>
      <td>✓ (Team e Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](/docs/pt/zero-data-retention)</td>
      <td>✓ (contas Enterprise qualificadas)</td>
      <td>✓ (contas qualificadas)</td>
      <td>Ver nota <sup><a href="#fn4">4</a></sup></td>
      <td>✓ (contas qualificadas)</td>
      <td>Ver nota <sup><a href="#fn4">4</a></sup></td>
      <td>Ver nota <sup><a href="#fn4">4</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> No Google Cloud's Agent Platform, web search está disponível para modelos Claude 4 e posteriores.<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> Nesses provedores, auto mode suporta apenas Claude Sonnet 5, Opus 4.7 e Opus 4.8. Consulte [Configuração de Auto mode](/docs/pt/auto-mode-config). Na v2.1.158 até v2.1.206, auto mode nesses provedores também exigia definir `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 removeu o requisito.<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> Intervalos explícitos como `/loop every 2 hours` funcionam em todos os provedores. No Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform e Microsoft Foundry, `/loop` não pode escolher seu próprio intervalo ou fornecer o prompt de manutenção padrão, portanto um prompt sem intervalo é executado a cada 10 minutos, e `/loop` sem argumentos mostra a mensagem de uso. Consulte [Scheduled tasks](/docs/pt/scheduled-tasks).<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> Sujeito ao seu acordo com o provedor de nuvem.<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> Dashboard e API apenas. [Contribution metrics](/docs/pt/analytics#enable-contribution-metrics) requer uma organização Claude.ai Team ou Enterprise.

<Note>
  Se você se autenticar através de um [LLM gateway](/docs/pt/llm-gateway), a disponibilidade de recursos corresponde ao provedor subjacente para o qual o gateway encaminha. Alguns recursos exclusivos da Anthropic, como o [Advisor](/docs/pt/advisor), funcionam apenas se o gateway encaminha solicitações intactas para a API Anthropic.
</Note>

<h3 id="summary-by-provider">
  Resumo por provedor
</h3>

Cada aba lista o que não está disponível ou tem suporte parcial nesse provedor, com alternativas onde uma existe. Tudo não listado funciona da mesma forma que em uma assinatura Claude, além das [diferenças específicas do provedor](#features-available-on-every-provider) observadas acima. No Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Claude Platform on AWS, relatório de erros e telemetria para Anthropic estão desativados por padrão. Consulte [comportamentos padrão por provedor de API](/docs/pt/data-usage#default-behaviors-by-api-provider) para saber qual tráfego ainda chega à Anthropic e como desativar.

<Tabs>
  <Tab title="Amazon Bedrock">
    **Não disponível:** todos os [recursos que requerem uma assinatura Claude](#features-that-require-a-claude-subscription), além de [web search](/docs/pt/tools-reference#websearch-tool-behavior), [fast mode](/docs/pt/fast-mode), [Advisor](/docs/pt/advisor), [Channels](/docs/pt/channels), o [analytics dashboard](/docs/pt/analytics), [server-managed settings](/docs/pt/server-managed-settings) e os [comandos `/design-sync` e `/radio`](/docs/pt/commands#all-commands).

    **Suporte parcial:**

    * [Desktop](/docs/pt/desktop): apenas via [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/pt/auto-mode-config): Sonnet 5, Opus 4.7 e Opus 4.8 apenas
    * [`/loop`](/docs/pt/scheduled-tasks): apenas intervalos explícitos
    * [Zero Data Retention](/docs/pt/zero-data-retention): sujeito ao seu acordo AWS

    **Alternativas:** para agendamento, use [`/loop`](/docs/pt/scheduled-tasks) com um intervalo explícito em vez de `/schedule`. Para sessões em nuvem, use [GitHub Actions](/docs/pt/github-actions) ou [GitLab CI/CD](/docs/pt/gitlab-ci-cd). Para pesquisas na web, use a [ferramenta WebFetch](/docs/pt/tools-reference#webfetch-tool-behavior) com uma URL específica.
  </Tab>

  <Tab title="Claude Platform on AWS">
    **Não disponível:** todos os [recursos que requerem uma assinatura Claude](#features-that-require-a-claude-subscription), além de [fast mode](/docs/pt/fast-mode), [Advisor](/docs/pt/advisor), [Channels](/docs/pt/channels), o [analytics dashboard](/docs/pt/analytics), [server-managed settings](/docs/pt/server-managed-settings) e os [comandos `/design-sync` e `/radio`](/docs/pt/commands#all-commands).

    **Disponível onde Amazon Bedrock não é:** [web search](/docs/pt/tools-reference#websearch-tool-behavior).

    **Suporte parcial:**

    * [`/loop`](/docs/pt/scheduled-tasks): apenas intervalos explícitos

    **Alternativas:** para agendamento, use [`/loop`](/docs/pt/scheduled-tasks) com um intervalo explícito em vez de `/schedule`. Para sessões em nuvem, use [GitHub Actions](/docs/pt/github-actions) ou [GitLab CI/CD](/docs/pt/gitlab-ci-cd).
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    **Não disponível:** todos os [recursos que requerem uma assinatura Claude](#features-that-require-a-claude-subscription), além de [fast mode](/docs/pt/fast-mode), [Advisor](/docs/pt/advisor), [Channels](/docs/pt/channels), o [analytics dashboard](/docs/pt/analytics), [server-managed settings](/docs/pt/server-managed-settings) e os [comandos `/design-sync` e `/radio`](/docs/pt/commands#all-commands).

    **Suporte parcial:**

    * [Desktop](/docs/pt/desktop): via [configurações gerenciadas](https://claude.com/docs/third-party/claude-desktop/configuration) ou [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Web search](/docs/pt/tools-reference#websearch-tool-behavior): modelos Claude 4 e posteriores
    * [Auto mode](/docs/pt/auto-mode-config): Sonnet 5, Opus 4.7 e Opus 4.8 apenas
    * [`/loop`](/docs/pt/scheduled-tasks): apenas intervalos explícitos
    * [Zero Data Retention](/docs/pt/zero-data-retention): sujeito ao seu acordo Google Cloud

    **Alternativas:** para agendamento, use [`/loop`](/docs/pt/scheduled-tasks) com um intervalo explícito em vez de `/schedule`. Para sessões em nuvem, use [GitHub Actions](/docs/pt/github-actions) ou [GitLab CI/CD](/docs/pt/gitlab-ci-cd).
  </Tab>

  <Tab title="Microsoft Foundry">
    **Não disponível:** todos os [recursos que requerem uma assinatura Claude](#features-that-require-a-claude-subscription), além de [fast mode](/docs/pt/fast-mode), [Advisor](/docs/pt/advisor), [Channels](/docs/pt/channels), [GitHub Actions](/docs/pt/github-actions) e [GitLab CI/CD](/docs/pt/gitlab-ci-cd), o [analytics dashboard](/docs/pt/analytics), [server-managed settings](/docs/pt/server-managed-settings) e os [comandos `/design-sync` e `/radio`](/docs/pt/commands#all-commands).

    **Suporte parcial:**

    * [Desktop](/docs/pt/desktop): apenas via [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/pt/auto-mode-config): Sonnet 5, Opus 4.7 e Opus 4.8 apenas
    * [`/loop`](/docs/pt/scheduled-tasks): apenas intervalos explícitos
    * [Zero Data Retention](/docs/pt/zero-data-retention): sujeito ao seu acordo Azure

    **Alternativas:** para agendamento, use [`/loop`](/docs/pt/scheduled-tasks) com um intervalo explícito em vez de `/schedule`.
  </Tab>

  <Tab title="Anthropic Console">
    **Não disponível:** todos os [recursos que requerem uma assinatura Claude](#features-that-require-a-claude-subscription).

    Tudo em [Recursos de CLI que variam por provedor](#cli-capabilities-that-vary-by-provider) está disponível, assim como [server-managed settings](/docs/pt/server-managed-settings) quando a chave de API pertence a uma organização Team ou Enterprise.
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  Disponibilidade por plano de assinatura
</h2>

Se você se autenticar através de Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou uma chave de API Anthropic Console, esta seção não se aplica a você. Quando você faz login com uma conta claude.ai, seu plano determina quais dos recursos abaixo estão disponíveis.

| Recurso                                                                     | Pro | Max | Team          | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :------------ | :-------------------------------- |
| [Claude Code na web](/docs/pt/claude-code-on-the-web)                            | ✓   | ✓   | ✓             | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/pt/routines)                                                    | ✓   | ✓   | ✓             | ✓                                 |
| [Remote Control](/docs/pt/remote-control)                                        | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Channels](/docs/pt/channels)                                                    | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Computer use](/docs/pt/computer-use)                                            | ✓   | ✓   | ✗             | ✗                                 |
| Dispatch ([Desktop](/docs/pt/desktop#sessions-from-dispatch))                    | ✓   | ✓   | ✗             | ✗                                 |
| [Code Review](/docs/pt/code-review)                                              | ✗   | ✗   | ✓             | ✓                                 |
| [Artifacts](/docs/pt/artifacts)                                                  | ✓   | ✓   | ✓             | Admin-enabled                     |
| [Dashboard de análises e métricas de contribuição](/docs/pt/analytics)           | ✗   | ✗   | ✓             | ✓                                 |
| [API Enterprise Analytics](/docs/pt/analytics#access-data-programmatically)      | ✗   | ✗   | ✗             | ✓                                 |
| [Server-managed settings](/docs/pt/server-managed-settings)                      | ✗   | ✗   | ✓             | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓             | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗             | ✓                                 |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance)        | ✗   | ✗   | ✗             | ✓                                 |
| [Zero Data Retention](/docs/pt/zero-data-retention)                              | ✗   | ✗   | ✗             | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> No Enterprise, requer um assento premium ou um assento Chat + Claude Code. Consulte [Claude Code na web](/docs/pt/claude-code-on-the-web).<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> Não incluído no plano Enterprise padrão. Requer habilitação separada pela Anthropic para contas qualificadas. Consulte [Zero Data Retention](/docs/pt/zero-data-retention).

Para preços e a comparação completa de planos, consulte [Planos Team](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) e [Planos Enterprise](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

<h2 id="model-availability">
  Disponibilidade de modelo
</h2>

Para saber quais modelos Claude e tamanhos de janela de contexto estão disponíveis por provedor e região, consulte [Configuração de modelo](/docs/pt/model-config) e a [visão geral de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Vision, entrada de PDF e pensamento estendido são capacidades de modelo em vez de recursos do Claude Code e funcionam em todos os provedores que oferecem o modelo. [Prompt caching](/docs/pt/prompt-caching) funciona da mesma forma na maioria dos provedores; no Amazon Bedrock, o suporte varia por modelo.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Visão geral de implantação empresarial](/docs/pt/third-party-integrations): compare autenticação, faturamento e regiões entre provedores
* Guias de configuração do provedor: [Amazon Bedrock](/docs/pt/amazon-bedrock), [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai), [Microsoft Foundry](/docs/pt/microsoft-foundry)
* [Plataformas e integrações](/docs/pt/platforms): onde o Claude Code é executado, incluindo CLI, Desktop, extensões IDE, web, celular e CI/CD
