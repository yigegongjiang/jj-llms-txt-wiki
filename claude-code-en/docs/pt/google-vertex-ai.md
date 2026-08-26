> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code na Plataforma de Agentes do Google Cloud

> Saiba como configurar Claude Code através da Plataforma de Agentes do Google Cloud, anteriormente Vertex AI, incluindo configuração, configuração de IAM e resolução de problemas.

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

<ContactSalesCard surface="vertex" />

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de configurar Claude Code com Google Cloud's Agent Platform, anteriormente Vertex AI, certifique-se de que você tem:

* Uma conta do Google Cloud Platform (GCP) com faturamento ativado
* Um projeto GCP com a API Google Cloud's Agent Platform ativada
* Acesso aos modelos Claude desejados (por exemplo, Claude Sonnet 4.6)
* Google Cloud SDK (`gcloud`) instalado e configurado
* Cota alocada na região GCP desejada

Para entrar com suas próprias credenciais do Google Cloud's Agent Platform, siga [Entrar com Google Cloud's Agent Platform](#sign-in-with-agent-platform) abaixo. Para implantar Claude Code em toda uma equipe, use as etapas de [configuração manual](#set-up-manually) e [fixe suas versões de modelo](#5-pin-model-versions) antes de fazer o lançamento.

<h2 id="sign-in-with-agent-platform">
  Entrar com Plataforma de Agentes do Google Cloud
</h2>

Se você tem credenciais do Google Cloud e deseja começar a usar Claude Code através da Plataforma de Agentes do Google Cloud, o assistente de login o guia através disso. Você completa os pré-requisitos do lado do GCP uma vez por projeto; o assistente cuida do lado do Claude Code.

<Steps>
  <Step title="Ativar modelos Claude no seu projeto GCP">
    [Ative a API da Plataforma de Agentes do Google Cloud](#1-enable-agent-platform-api) para seu projeto, depois solicite acesso aos modelos Claude que você deseja no [Jardim de Modelos da Plataforma de Agentes do Google Cloud](https://console.cloud.google.com/vertex-ai/model-garden). Veja [Configuração de IAM](#iam-configuration) para as permissões que sua conta precisa.
  </Step>

  <Step title="Inicie Claude Code e escolha a Plataforma de Agentes do Google Cloud">
    Execute `claude`. No prompt de login, selecione **plataforma de terceiros**, depois **Google Vertex AI**, o rótulo que o prompt de login ainda usa para a Plataforma de Agentes do Google Cloud.
  </Step>

  <Step title="Siga os prompts do assistente">
    Escolha como você se autentica no Google Cloud: Application Default Credentials do `gcloud`, um arquivo de chave de conta de serviço, ou credenciais já em seu ambiente. O assistente detecta seu projeto e região, verifica quais modelos Claude seu projeto pode invocar, e permite que você os fixe. Ele salva o resultado no bloco `env` do seu [arquivo de configurações do usuário](/docs/pt/settings), para que você não precise exportar variáveis de ambiente você mesmo.
  </Step>
</Steps>

Depois de entrar, execute `/setup-vertex` a qualquer momento para reabrir o assistente e alterar suas credenciais, projeto, região ou fixações de modelo. A etapa de fixação de modelo começa a partir de seus modelos atualmente fixados. O assistente escreve em `~/.claude/settings.json`, ou em `$CLAUDE_CONFIG_DIR/settings.json` quando [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars#variables) está definido.

<h2 id="region-configuration">
  Configuração de região
</h2>

Claude Code suporta endpoints [globais](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai), multi-região e regionais do Google Cloud's Agent Platform. Defina `CLOUD_ML_REGION` como `global`, um local multi-região como `eu` ou `us`, ou uma região específica como `us-east5`. Claude Code seleciona o nome de host correto do Google Cloud's Agent Platform para cada formulário, incluindo os hosts `aiplatform.eu.rep.googleapis.com` e `aiplatform.us.rep.googleapis.com` para locais multi-região.

<Note>
  Google Cloud's Agent Platform pode não suportar os modelos padrão do Claude Code em todos os tipos de endpoint. A disponibilidade de modelos varia entre [regiões específicas](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models), locais multi-região e [endpoints globais](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models). Você pode precisar mudar para um local suportado ou especificar um modelo suportado.
</Note>

<h2 id="set-up-manually">
  Configurar manualmente
</h2>

Para configurar a Plataforma de Agentes do Google Cloud através de variáveis de ambiente em vez do assistente, por exemplo em CI ou um lançamento empresarial com script, siga as etapas abaixo.

<h3 id="1-enable-agent-platform-api">
  1. Ativar a API da Plataforma de Agentes
</h3>

Ative a API da Plataforma de Agentes do Google Cloud no seu projeto GCP:

```bash theme={null}
# Defina seu ID de projeto
gcloud config set project YOUR-PROJECT-ID

# Ativar a API da Plataforma de Agentes
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. Solicitar acesso ao modelo
</h3>

Solicite acesso aos modelos Claude na Plataforma de Agentes do Google Cloud:

1. Navegue até o [Jardim de Modelos da Plataforma de Agentes do Google Cloud](https://console.cloud.google.com/vertex-ai/model-garden)
2. Procure por modelos "Claude"
3. Solicite acesso aos modelos Claude desejados (por exemplo, Claude Sonnet 4.6)
4. Aguarde a aprovação (pode levar 24-48 horas)

<h3 id="3-configure-gcp-credentials">
  3) Configurar credenciais GCP
</h3>

Claude Code usa autenticação padrão do Google Cloud.

Para mais informações, consulte a [documentação de autenticação do Google Cloud](https://cloud.google.com/docs/authentication).

Claude Code v2.1.121 ou posterior suporta [Federação de Identidade de Carga de Trabalho baseada em certificado X.509](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates) através da mesma cadeia de Credenciais Padrão da Aplicação. Defina `GOOGLE_APPLICATION_CREDENTIALS` para o caminho do seu arquivo de configuração de credenciais.

<Note>
  Claude Code usa `ANTHROPIC_VERTEX_PROJECT_ID` como o ID do projeto para solicitações da Plataforma de Agentes do Google Cloud. As variáveis de ambiente `GCLOUD_PROJECT` e `GOOGLE_CLOUD_PROJECT` e o arquivo de credenciais referenciado por `GOOGLE_APPLICATION_CREDENTIALS` têm precedência sobre ele. Se nenhum destes estiver definido, o ID do projeto é resolvido a partir da sua configuração `gcloud` ou da conta de serviço anexada.
</Note>

<h4 id="advanced-credential-configuration">
  Configuração avançada de credenciais
</h4>

Claude Code suporta atualização automática de credenciais para GCP através da configuração `gcpAuthRefresh`. Quando Claude Code detecta que suas credenciais GCP expiraram ou não podem ser carregadas, ele executa o comando configurado para obter novas credenciais antes de tentar novamente a solicitação.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

A saída do comando é exibida ao usuário, mas entrada interativa não é suportada. Isso funciona bem para fluxos de autenticação baseados em navegador onde a CLI mostra uma URL e você completa a autenticação no navegador. O comando de atualização expira após três minutos se a autenticação não for concluída. Se você definir `gcpAuthRefresh` em configurações de projeto como `.claude/settings.json`, o comando é executado apenas após você aceitar o prompt de confiança do workspace.

<h3 id="4-configure-claude-code">
  4. Configurar Claude Code
</h3>

Defina as seguintes variáveis de ambiente:

```bash theme={null}
# Ativar integração da Plataforma de Agentes
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# Opcional: Substituir a URL do endpoint da Plataforma de Agentes para endpoints personalizados ou gateways
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# Opcional: Desativar prompt caching se necessário
export DISABLE_PROMPT_CACHING=1

# Opcional: Solicitar TTL de cache de prompt de 1 hora em vez do padrão de 5 minutos
export ENABLE_PROMPT_CACHING_1H=1

# Quando CLOUD_ML_REGION=global, substituir região para modelos que não suportam endpoints globais
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

A maioria das versões de modelo tem uma variável `VERTEX_REGION_CLAUDE_*` correspondente. Veja a [referência de variáveis de ambiente](/docs/pt/env-vars) para a lista completa. Verifique o [Jardim de Modelos da Plataforma de Agentes do Google Cloud](https://console.cloud.google.com/vertex-ai/model-garden) para determinar quais modelos suportam endpoints globais versus apenas regionais.

[Prompt caching](/docs/pt/prompt-caching) é ativado automaticamente. Para desativá-lo, defina `DISABLE_PROMPT_CACHING=1`. Para solicitar um TTL de cache de 1 hora em vez do padrão de 5 minutos, defina `ENABLE_PROMPT_CACHING_1H=1`; gravações de cache com TTL de 1 hora são cobradas a uma taxa mais alta. Para limites de taxa aumentados, entre em contato com o suporte do Google Cloud. Ao usar a Plataforma de Agentes do Google Cloud, o comando `/logout` não está disponível, pois a autenticação é tratada através das credenciais do Google Cloud.

Claude Code desativa [MCP tool search](/docs/pt/mcp#scale-with-mcp-tool-search) por padrão na Plataforma de Agentes do Google Cloud, portanto as definições de ferramenta MCP são carregadas antecipadamente. A Plataforma de Agentes do Google Cloud suporta busca de ferramentas para Claude Sonnet 4.5 e posterior e Claude Opus 4.5 e posterior. Defina `ENABLE_TOOL_SEARCH=true` para ativá-lo nesses modelos. Modelos anteriores na Plataforma de Agentes do Google Cloud não aceitam o cabeçalho beta necessário, e as solicitações falham se você ativar a busca de ferramentas com eles.

<h3 id="5-pin-model-versions">
  5. Fixar versões de modelo
</h3>

<Warning>
  Fixe versões de modelo específicas ao implantar para vários usuários. Sem fixação, aliases de modelo como `sonnet` e `opus` resolvem para o padrão integrado do Claude Code para a Plataforma de Agentes do Google Cloud, que pode ficar atrás da versão mais recente e pode ainda não estar ativado no seu projeto. Claude Code [volta](#startup-model-checks) para uma versão anterior ou modelo de nível inferior na inicialização quando o padrão não está disponível, mas fixar permite que você controle quando seus usuários se movem para um novo modelo.
</Warning>

Defina estas variáveis de ambiente para IDs de modelo específicos da Plataforma de Agentes do Google Cloud.

Sem `ANTHROPIC_DEFAULT_OPUS_MODEL`, o alias `opus` na Plataforma de Agentes do Google Cloud resolve para Opus 4.8, e sem `ANTHROPIC_DEFAULT_SONNET_MODEL`, o alias `sonnet` resolve para Sonnet 4.5. Este exemplo fixa cada alias a uma versão específica:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

Para IDs de modelo atuais e legados, veja [Visão geral de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Veja [Configuração de modelo](/docs/pt/model-config#pin-models-for-third-party-deployments) para a lista completa de variáveis de ambiente.

Claude Code usa estes modelos padrão quando nenhuma variável de fixação está definida:

| Tipo de modelo        | Valor padrão                 |
| :-------------------- | :--------------------------- |
| Modelo primário       | `claude-opus-4-8`            |
| Modelo pequeno/rápido | `claude-sonnet-4-5@20250929` |

Tarefas em segundo plano, como geração de título de sessão, usam o modelo pequeno/rápido, normalmente um modelo da classe Haiku. Na Plataforma de Agentes do Google Cloud, Claude Code usa o modelo Sonnet padrão para tarefas em segundo plano porque Haiku pode não estar ativado em todos os projetos ou regiões. Duas seleções mudam qual modelo as executa:

* Quando você seleciona um modelo primário com `--model`, `ANTHROPIC_MODEL`, ou a configuração `model`, tarefas em segundo plano usam esse modelo. Definir `ANTHROPIC_DEFAULT_OPUS_MODEL` sem `ANTHROPIC_DEFAULT_SONNET_MODEL` também conta como uma seleção, porque o modelo Sonnet integrado pode não estar ativado em um projeto que direciona seu próprio Opus.
* Para usar Haiku para tarefas em segundo plano, defina `ANTHROPIC_DEFAULT_HAIKU_MODEL` para um ID de modelo que esteja disponível no seu projeto.

<Warning>
  Modelos Opus têm um preço por token mais alto do que modelos Sonnet, portanto uma implantação que não fixa um modelo primário é cobrada à taxa Opus uma vez que atualiza para v2.1.207 ou posterior. Para manter Sonnet 4.5 como o modelo primário, defina `ANTHROPIC_MODEL` para seu ID de modelo completo. Uma implantação que direciona o padrão com `ANTHROPIC_DEFAULT_SONNET_MODEL` e não define `ANTHROPIC_DEFAULT_OPUS_MODEL` mantém seu modelo Sonnet direcionado como o padrão.
</Warning>

Antes de v2.1.207, o modelo primário na Plataforma de Agentes do Google Cloud era padrão para Sonnet 4.5, o alias `opus` resolvia para Opus 4.6, e tarefas em segundo plano sempre usavam o modelo primário.

Para personalizar modelos ainda mais:

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h2 id="startup-model-checks">
  Verificações de modelo na inicialização
</h2>

Quando Claude Code inicia com a Plataforma de Agentes do Google Cloud configurada, ele verifica que os modelos que pretende usar estão acessíveis no seu projeto.

Se você fixou uma versão de modelo que é mais antiga que o padrão atual do Claude Code, e seu projeto pode invocar a versão mais recente, Claude Code o solicita a atualizar a fixação. Aceitar escreve o novo ID de modelo no seu [arquivo de configurações do usuário](/docs/pt/settings) e reinicia Claude Code. Recusar é lembrado até a próxima mudança de versão padrão.

Se você não fixou um modelo e o padrão atual não está disponível no seu projeto, Claude Code volta para a versão anterior para a sessão atual e mostra um aviso. Ele tenta versões anteriores do modelo padrão primeiro e, quando o padrão é um modelo Opus e nenhuma versão Opus está disponível, volta para o modelo Sonnet padrão. O fallback não é persistido. Ative o modelo mais recente no [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) ou [fixe uma versão](#5-pin-model-versions) para tornar a escolha permanente.

<h2 id="iam-configuration">
  Configuração de IAM
</h2>

Atribua as permissões de IAM necessárias:

A função `roles/aiplatform.user` inclui as permissões necessárias:

* `aiplatform.endpoints.predict` - Necessário para invocação de modelo e contagem de tokens

Para permissões mais restritivas, crie uma função personalizada com apenas as permissões acima.

Para detalhes, veja a [documentação de IAM do Vertex](https://cloud.google.com/vertex-ai/docs/general/access-control).

<Note>
  Crie um projeto GCP dedicado para Claude Code para simplificar o rastreamento de custos e controle de acesso.
</Note>

<h2 id="1m-token-context-window">
  Janela de contexto de 1M de tokens
</h2>

Claude Sonnet 5, Opus 4.6 e posteriores, e Sonnet 4.6 suportam a [janela de contexto de 1M de tokens](https://platform.claude.com/docs/pt/build-with-claude/context-windows#context-window-sizes-by-model) na Plataforma de Agentes do Google Cloud. Sonnet 5 sempre é executado com a janela de 1M, sem nenhuma variante `[1m]` para selecionar. Para os outros modelos, Claude Code ativa automaticamente a janela de contexto estendida quando você seleciona uma variante de modelo 1M.

O [assistente de configuração](#sign-in-with-agent-platform) oferece uma opção de contexto 1M quando fixa modelos. Para ativá-lo para um modelo fixado manualmente em vez disso, acrescente `[1m]` ao ID do modelo. Veja [Fixar modelos para implantações de terceiros](/docs/pt/model-config#pin-models-for-third-party-deployments) para detalhes.

<h2 id="troubleshooting">
  Resolução de problemas
</h2>

Se você encontrar erros "Não foi possível carregar as credenciais padrão":

* Execute `gcloud auth application-default login` para configurar Credenciais Padrão da Aplicação
* Defina `GOOGLE_APPLICATION_CREDENTIALS` para um caminho de arquivo de chave de conta de serviço
* Consulte [Configurar credenciais do GCP](#3-configure-gcp-credentials) para todas as opções

Se você encontrar problemas de cota:

* Verifique cotas atuais ou solicite aumento de cota através do [Cloud Console](https://cloud.google.com/docs/quotas/view-manage)

Se você encontrar erros "modelo não encontrado" 404:

* Confirme que o modelo está Ativado no [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
* Verifique se o modelo está disponível no local que você especificou. Alguns modelos são oferecidos apenas em locais `global` ou multi-região como `eu` e `us`, não em regiões específicas
* Se estiver usando `CLOUD_ML_REGION=global`, verifique se seus modelos suportam endpoints globais no [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) em "Recursos suportados". Para modelos que não suportam endpoints globais, faça um dos seguintes:
  * Especifique um modelo suportado via `ANTHROPIC_MODEL` ou `ANTHROPIC_DEFAULT_HAIKU_MODEL`, ou
  * Defina uma região ou local multi-região usando variáveis de ambiente `VERTEX_REGION_<MODEL_NAME>`

Se você encontrar erros 429:

* Para endpoints regionais, certifique-se de que o modelo primário e o modelo pequeno/rápido são suportados em sua região selecionada
* Considere mudar para `CLOUD_ML_REGION=global` para melhor disponibilidade

<h2 id="additional-resources">
  Recursos adicionais
</h2>

* [Documentação do Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs)
* [Preços do Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/pricing)
* [Cotas e limites do Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs/quotas)
