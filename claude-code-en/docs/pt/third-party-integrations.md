> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Visão geral da implantação empresarial

> Saiba como Claude Code pode se integrar com vários serviços de terceiros e infraestrutura para atender aos requisitos de implantação empresarial.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

As organizações podem implantar Claude Code através da Anthropic diretamente ou através de um provedor de nuvem. Esta página ajuda você a escolher a configuração correta.

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  Comparar opções de implantação
</h2>

Para a maioria das organizações, Claude for Teams ou Claude for Enterprise oferece a melhor experiência. Os membros da equipe obtêm acesso tanto a Claude Code quanto a Claude na web com uma única assinatura, faturamento centralizado e nenhuma configuração de infraestrutura necessária.

**Claude for Teams** é de autoatendimento e inclui recursos de colaboração, ferramentas de administração e gerenciamento de faturamento. Melhor para equipes menores que precisam começar rapidamente.

**Claude for Enterprise** adiciona SSO e captura de domínio, permissões baseadas em funções, acesso à API de conformidade e configurações de política gerenciada para implantar configurações de Claude Code em toda a organização. Melhor para organizações maiores com requisitos de segurança e conformidade.

Saiba mais sobre [planos de equipe](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) e [planos empresariais](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

Se sua organização tem requisitos de infraestrutura específicos, compare as opções abaixo:

<table>
  <thead>
    <tr>
      <th>Recurso</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform, formerly Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>Melhor para</td>
      <td>Maioria das organizações (recomendado)</td>
      <td>Desenvolvedores individuais</td>
      <td>Implantações nativas da AWS</td>
      <td>Faturamento do AWS Marketplace com recursos da API Claude</td>
      <td>Implantações nativas do GCP</td>
      <td>Implantações nativas do Azure</td>
    </tr>

    <tr>
      <td>Faturamento</td>
      <td><strong>Teams:</strong> \$150/assento (Premium) com PAYG disponível<br /><strong>Enterprise:</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">Entre em contato com vendas</a></td>
      <td>PAYG</td>
      <td>PAYG através da AWS</td>
      <td>PAYG através do AWS Marketplace</td>
      <td>PAYG através do GCP</td>
      <td>PAYG através do Azure</td>
    </tr>

    <tr>
      <td>Regiões</td>
      <td>[Países](https://www.anthropic.com/supported-countries) suportados</td>
      <td>[Países](https://www.anthropic.com/supported-countries) suportados</td>
      <td>Múltiplas [regiões](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html) da AWS</td>
      <td>Múltiplas regiões da AWS</td>
      <td>Múltiplas [regiões](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) do GCP</td>
      <td>Múltiplas [regiões](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/) do Azure</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>Ativado por padrão</td>
      <td>Ativado por padrão</td>
      <td>Ativado por padrão</td>
      <td>Ativado por padrão</td>
      <td>Ativado por padrão</td>
      <td>Ativado por padrão</td>
    </tr>

    <tr>
      <td>Autenticação</td>
      <td>Claude.ai SSO ou email</td>
      <td>Chave de API</td>
      <td>Chave de API ou credenciais da AWS</td>
      <td>Chave de API ou credenciais da AWS</td>
      <td>Credenciais do GCP</td>
      <td>Chave de API ou Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>Rastreamento de custos</td>
      <td>Painel de uso</td>
      <td>Painel de uso</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>Faturamento do GCP</td>
      <td>Gerenciamento de custos do Azure</td>
    </tr>

    <tr>
      <td>Inclui Claude na web</td>
      <td>Sim</td>
      <td>Não</td>
      <td>Não</td>
      <td>Não</td>
      <td>Não</td>
      <td>Não</td>
    </tr>

    <tr>
      <td>Recursos empresariais</td>
      <td>Gerenciamento de equipe, SSO, monitoramento de uso</td>
      <td>Nenhum</td>
      <td>Políticas de IAM, CloudTrail</td>
      <td>Políticas de IAM, CloudTrail</td>
      <td>Funções de IAM, Cloud Audit Logs</td>
      <td>Políticas de RBAC, Azure Monitor</td>
    </tr>
  </tbody>
</table>

Para uma análise detalhada de recursos do que está disponível em cada opção, consulte [Disponibilidade de recursos](/docs/pt/feature-availability).

Selecione uma opção de implantação para visualizar as instruções de configuração:

* [Claude for Teams ou Enterprise](/docs/pt/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/pt/authentication#claude-console-authentication)
* [Claude apps gateway](/docs/pt/claude-apps-gateway), um gateway auto-hospedado que adiciona entrada de IdP na frente do Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry ou da API Anthropic
* [Amazon Bedrock](/docs/pt/amazon-bedrock)
* [Claude Platform on AWS](/docs/pt/claude-platform-on-aws)
* [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai)
* [Microsoft Foundry](/docs/pt/microsoft-foundry)

Para Amazon Bedrock e Google Vertex AI, você também pode executar `claude` e selecionar **plataforma de terceiros** no prompt de login para iniciar um assistente de configuração interativo.

<h2 id="configure-proxies-and-gateways">
  Configurar proxies e gateways
</h2>

A maioria das organizações pode usar um provedor de nuvem diretamente sem configuração adicional. No entanto, você pode precisar configurar um proxy corporativo ou gateway LLM se sua organização tiver requisitos específicos de rede ou gerenciamento. Estas são configurações diferentes que podem ser usadas juntas:

* **Proxy corporativo**: Roteia o tráfego através de um proxy HTTP/HTTPS. Use isto se sua organização exigir que todo o tráfego de saída passe por um servidor proxy para monitoramento de segurança, conformidade ou aplicação de política de rede. Configure com as variáveis de ambiente `HTTPS_PROXY` ou `HTTP_PROXY`. Saiba mais em [Configuração de rede empresarial](/docs/pt/network-config).
* **Gateway LLM**: Um serviço que fica entre Claude Code e o provedor de nuvem para lidar com autenticação e roteamento. Use isto se você precisar de rastreamento de uso centralizado entre equipes, limitação de taxa personalizada ou orçamentos, ou gerenciamento de autenticação centralizado. Configure com as variáveis de ambiente `ANTHROPIC_BASE_URL`, `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, ou `ANTHROPIC_FOUNDRY_BASE_URL`. Saiba mais em [Gateways LLM](/docs/pt/llm-gateway).

Os exemplos a seguir mostram as variáveis de ambiente a definir no seu shell ou perfil de shell (`.bashrc`, `.zshrc`). Veja [Configurações](/docs/pt/settings) para outros métodos de configuração.

<h3 id="amazon-bedrock">
  Amazon Bedrock
</h3>

<Tabs>
  <Tab title="Proxy corporativo">
    Rotear o tráfego do Amazon Bedrock através do seu proxy corporativo definindo as seguintes [variáveis de ambiente](/docs/pt/env-vars):

    ```bash theme={null}
    # Ativar Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1
    export AWS_REGION=us-east-1

    # Configurar proxy corporativo
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Gateway LLM">
    Rotear o tráfego do Amazon Bedrock através do seu gateway LLM definindo as seguintes [variáveis de ambiente](/docs/pt/env-vars):

    ```bash theme={null}
    # Ativar Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1

    # Configurar gateway LLM
    export ANTHROPIC_BEDROCK_BASE_URL='https://your-llm-gateway.com/bedrock'
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1  # Se o gateway lidar com autenticação da AWS
    ```
  </Tab>
</Tabs>

<h3 id="microsoft-foundry">
  Microsoft Foundry
</h3>

<Tabs>
  <Tab title="Proxy corporativo">
    Rotear o tráfego do Microsoft Foundry através do seu proxy corporativo definindo as seguintes [variáveis de ambiente](/docs/pt/env-vars):

    ```bash theme={null}
    # Ativar Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1
    export ANTHROPIC_FOUNDRY_RESOURCE=your-resource
    export ANTHROPIC_FOUNDRY_API_KEY=your-api-key  # Ou omitir para autenticação Entra ID

    # Configurar proxy corporativo
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Gateway LLM">
    Rotear o tráfego do Microsoft Foundry através do seu gateway LLM definindo as seguintes [variáveis de ambiente](/docs/pt/env-vars):

    ```bash theme={null}
    # Ativar Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1

    # Configurar gateway LLM
    export ANTHROPIC_FOUNDRY_BASE_URL='https://your-llm-gateway.com'
    export ANTHROPIC_FOUNDRY_API_KEY=your-gateway-key  # Enviado como x-api-key
    ```
  </Tab>
</Tabs>

<h3 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h3>

<Tabs>
  <Tab title="Proxy corporativo">
    Rotear o tráfego do Google Cloud's Agent Platform através do seu proxy corporativo definindo as seguintes [variáveis de ambiente](/docs/pt/env-vars):

    ```bash theme={null}
    # Ativar Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    export ANTHROPIC_VERTEX_PROJECT_ID=your-project-id

    # Configurar proxy corporativo
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Gateway LLM">
    Rotear o tráfego do Google Cloud's Agent Platform através do seu gateway LLM definindo as seguintes [variáveis de ambiente](/docs/pt/env-vars):

    ```bash theme={null}
    # Ativar Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1

    # Configurar gateway LLM
    export ANTHROPIC_VERTEX_BASE_URL='https://your-llm-gateway.com/vertex'
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1  # Se o gateway lidar com autenticação do GCP
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>
</Tabs>

<Tip>
  Use `/status` em Claude Code para verificar se a configuração do seu proxy e gateway foi aplicada corretamente. Por exemplo, com a configuração do gateway Bedrock acima, a saída inclui linhas como:

  ```
  API provider: Amazon Bedrock
  Bedrock base URL: https://your-llm-gateway.com/bedrock
  AWS region: us-east-1
  AWS auth skipped
  ```

  Se você configurou um proxy corporativo, `/status` também mostra uma linha `Proxy` com a URL do seu proxy.
</Tip>

<h2 id="best-practices-for-organizations">
  Melhores práticas para organizações
</h2>

<h3 id="invest-in-documentation-and-memory">
  Investir em documentação e memória
</h3>

Recomendamos fortemente investir em documentação para que Claude Code compreenda sua base de código. As organizações podem implantar arquivos CLAUDE.md em múltiplos níveis:

* **Em toda a organização**: Implante em diretórios do sistema como `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS), `/etc/claude-code/CLAUDE.md` (Linux e WSL), ou `C:\Program Files\ClaudeCode\CLAUDE.md` (Windows) para padrões em toda a empresa
* **Nível de repositório**: Crie arquivos `CLAUDE.md` nas raízes dos repositórios contendo arquitetura do projeto, comandos de compilação e diretrizes de contribuição. Verifique-os no controle de origem para que todos os usuários se beneficiem

Saiba mais em [Memória e arquivos CLAUDE.md](/docs/pt/memory).

<h3 id="simplify-deployment">
  Simplificar a implantação
</h3>

Se você tiver um ambiente de desenvolvimento personalizado, descobrimos que criar uma maneira "com um clique" de instalar Claude Code é fundamental para aumentar a adoção em toda uma organização.

<h3 id="start-with-guided-usage">
  Começar com uso orientado
</h3>

Incentive novos usuários a experimentar Claude Code para perguntas sobre a base de código, ou em correções de bugs menores ou solicitações de recursos. Peça a Claude Code para fazer um plano. Verifique as sugestões de Claude e forneça feedback se estiver fora do caminho. Com o tempo, conforme os usuários entendem melhor esse novo paradigma, eles serão mais eficazes em permitir que Claude Code funcione de forma mais autônoma.

<h3 id="pin-model-versions-for-cloud-providers">
  Fixar versões de modelo para provedores de nuvem
</h3>

Se você implantar através de [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai), [Microsoft Foundry](/docs/pt/microsoft-foundry), ou [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), fixe versões de modelo específicas usando `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, e `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Sem fixação, os aliases de modelo resolvem para o padrão integrado de Claude Code para esse provedor, o que pode ficar atrás da versão mais recente e pode ainda não estar ativado em sua conta. Fixar permite que você controle quando seus usuários passam para um novo modelo. Veja [Configuração de modelo](/docs/pt/model-config#pin-models-for-third-party-deployments) para o que cada provedor faz quando o padrão não está disponível.

<h3 id="configure-security-policies">
  Configurar políticas de segurança
</h3>

As equipes de segurança podem configurar permissões gerenciadas para o que Claude Code é e não é permitido fazer, o que não pode ser substituído pela configuração local. [Saiba mais](/docs/pt/security).

<h3 id="leverage-mcp-for-integrations">
  Aproveitar MCP para integrações
</h3>

MCP é uma ótima maneira de dar a Claude Code mais informações, como conectar a sistemas de gerenciamento de tickets ou logs de erro. Recomendamos que uma equipe central configure servidores MCP e verifique uma configuração `.mcp.json` na base de código para que todos os usuários se beneficiem. [Saiba mais](/docs/pt/mcp).

Na Anthropic, confiamos em Claude Code para potencializar o desenvolvimento em todas as bases de código da Anthropic. Esperamos que você aproveite usar Claude Code tanto quanto nós.

<h2 id="next-steps">
  Próximas etapas
</h2>

Depois de escolher uma opção de implantação e configurar o acesso para sua equipe:

1. **Implante em sua equipe**: Compartilhe instruções de instalação e peça aos membros da equipe para [instalar Claude Code](/docs/pt/setup) e autenticar com suas credenciais.
2. **Configurar configuração compartilhada**: Crie um [arquivo CLAUDE.md](/docs/pt/memory) em seus repositórios para ajudar Claude Code a compreender sua base de código e padrões de codificação.
3. **Configurar permissões**: Revise [configurações de segurança](/docs/pt/security) para definir o que Claude Code pode e não pode fazer em seu ambiente.
