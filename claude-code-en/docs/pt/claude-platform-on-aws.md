> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code no Claude Platform on AWS

> Configure Claude Code para usar a API Claude operada pela Anthropic com autenticação AWS, controle de acesso IAM e faturamento do AWS Marketplace.

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

export const Experiment = ({flag, treatment, children}) => {
  const VID_KEY = 'exp_vid';
  const CONSENT_COUNTRIES = new Set(['AT', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 'EE', 'FI', 'FR', 'DE', 'GR', 'HU', 'IE', 'IT', 'LV', 'LT', 'LU', 'MT', 'NL', 'PL', 'PT', 'RO', 'SK', 'SI', 'ES', 'SE', 'RE', 'GP', 'MQ', 'GF', 'YT', 'BL', 'MF', 'PM', 'WF', 'PF', 'NC', 'AW', 'CW', 'SX', 'FO', 'GL', 'AX', 'GB', 'UK', 'AI', 'BM', 'IO', 'VG', 'KY', 'FK', 'GI', 'MS', 'PN', 'SH', 'TC', 'GG', 'JE', 'IM', 'CA', 'BR', 'IN']);
  const fnv1a = s => {
    let h = 0x811c9dc5;
    for (let i = 0; i < s.length; i++) {
      h ^= s.charCodeAt(i);
      h += (h << 1) + (h << 4) + (h << 7) + (h << 8) + (h << 24);
    }
    return h >>> 0;
  };
  const bucket = (seed, vid) => fnv1a(fnv1a(seed + vid) + '') % 10000 < 5000 ? 'control' : 'treatment';
  const [decision] = useState(() => {
    const params = new URLSearchParams(location.search);
    const preBucketed = document.documentElement.dataset['gb_' + flag.replace(/-/g, '_')];
    const force = params.get('gb-force');
    if (force) {
      for (const p of force.split(',')) {
        const [k, v] = p.split(':');
        if (k === flag) return {
          variant: v || 'treatment',
          track: false
        };
      }
    }
    if (navigator.globalPrivacyControl) {
      return {
        variant: 'control',
        track: false
      };
    }
    const prefsMatch = document.cookie.match(/(?:^|; )anthropic-consent-preferences=([^;]+)/);
    if (prefsMatch) {
      try {
        if (JSON.parse(decodeURIComponent(prefsMatch[1])).analytics !== true) {
          return {
            variant: 'control',
            track: false
          };
        }
      } catch {
        return {
          variant: 'control',
          track: false
        };
      }
    } else {
      const country = params.get('country')?.toUpperCase() || (document.cookie.match(/(?:^|; )cf_geo=([A-Z]{2})/) || [])[1];
      if (!country || CONSENT_COUNTRIES.has(country)) {
        return {
          variant: 'control',
          track: false
        };
      }
    }
    let vid;
    try {
      const ajsMatch = document.cookie.match(/(?:^|; )ajs_anonymous_id=([^;]+)/);
      if (ajsMatch) {
        vid = decodeURIComponent(ajsMatch[1]).replace(/^"|"$/g, '');
      } else {
        vid = localStorage.getItem(VID_KEY);
        if (!vid) {
          vid = crypto.randomUUID();
        }
        document.cookie = `ajs_anonymous_id=${vid}; domain=.claude.com; path=/; Secure; SameSite=Lax; max-age=31536000`;
      }
      try {
        localStorage.setItem(VID_KEY, vid);
      } catch {}
    } catch {
      return {
        variant: 'control',
        track: false
      };
    }
    const variant = preBucketed === '1' ? 'treatment' : preBucketed === '0' ? 'control' : bucket(flag, vid);
    return {
      variant,
      track: true,
      vid
    };
  });
  useEffect(() => {
    if (!decision.track) return;
    fetch('https://api.anthropic.com/api/event_logging/v2/batch', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-service-name': 'claude_code_docs'
      },
      body: JSON.stringify({
        events: [{
          event_type: 'GrowthbookExperimentEvent',
          event_data: {
            device_id: decision.vid,
            anonymous_id: decision.vid,
            timestamp: new Date().toISOString(),
            experiment_id: flag,
            variation_id: decision.variant === 'treatment' ? 1 : 0,
            environment: 'production'
          }
        }]
      }),
      keepalive: true
    }).catch(() => {});
  }, []);
  return decision.variant === 'treatment' ? treatment : children;
};

<Experiment flag="docs-contact-sales-cta" treatment={<ContactSalesCard surface="claude_platform_on_aws" />} />

Claude Platform on AWS é a API Claude operada pela Anthropic com autenticação AWS, controle de acesso IAM e faturamento do AWS Marketplace. As solicitações chegam diretamente à API da Anthropic, portanto você obtém os mesmos modelos e recursos de API que a [Claude API](https://platform.claude.com/docs) no mesmo cronograma de lançamento. Os recursos do lado do cliente que Claude Code ativa através do serviço de sinalizadores de recursos da Anthropic, como [`/loop` auto-ritmo](/docs/pt/scheduled-tasks#let-claude-choose-the-interval), estão desativados por padrão, e a [ferramenta advisor](/docs/pt/advisor) não está disponível. Consulte a [matriz de disponibilidade de recursos](/docs/pt/feature-availability#summary-by-provider) para obter a lista completa. Você se autentica com credenciais AWS ou uma chave de API do workspace, e paga através do AWS Marketplace.

Use este guia para apontar Claude Code para um workspace que você já provisionou através do Claude Platform on AWS. Para a assinatura AWS e configuração do workspace que vem antes disso, consulte a [documentação do Claude Platform on AWS](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws).

<Note>
  Assinar através do AWS Marketplace provisiona uma nova organização Anthropic vinculada à sua conta AWS. Esta organização é separada de qualquer organização que você já tenha com a Anthropic, e as credenciais não são transferidas entre elas. Use o ID do workspace e as chaves de API da organização vinculada ao AWS, não de uma conta Claude Console pré-existente.
</Note>

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de configurar Claude Code, você precisa de:

* Uma assinatura ativa do Claude Platform on AWS através do AWS Marketplace
* Um workspace em sua organização Anthropic vinculada ao AWS, com seu ID de workspace
* Um principal IAM com permissão para invocar o serviço Anthropic, ou uma chave de API com escopo para o workspace
* Credenciais AWS em seu ambiente, em `~/.aws/credentials`, ou de uma função IAM anexada se você quiser autenticação SigV4. A AWS CLI é necessária apenas para o fluxo de login SSO.

<h2 id="setup">
  Configuração
</h2>

<h3 id="1-configure-aws-credentials">
  1. Configure credenciais AWS
</h3>

Claude Code suporta dois métodos de autenticação para Claude Platform on AWS. Escolha o método que se adequa à forma como sua equipe gerencia o acesso.

**Opção A: Credenciais AWS com SigV4**

Claude Code assina solicitações com SigV4 usando a cadeia de credenciais AWS padrão: variáveis de ambiente, credenciais compartilhadas em `~/.aws/credentials`, funções IAM, sessões AWS SSO e qualquer outra fonte que o AWS SDK suporte.

Para uso local, faça login com a AWS CLI antes de iniciar Claude Code. O exemplo abaixo usa um perfil SSO, mas qualquer método que produza credenciais nos locais padrão funciona.

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

Para CI e automação, dê ao executor uma função IAM com permissão para invocar o serviço Anthropic e defina `AWS_REGION`. A cadeia de credenciais pega a função automaticamente.

Se suas credenciais SSO expirarem durante a sessão, configure [`awsAuthRefresh`](/docs/pt/amazon-bedrock#advanced-credential-configuration) para que Claude Code execute novamente seu comando de login e tente novamente em vez de falhar. A atualização automática no Claude Platform on AWS requer Claude Code v2.1.198 ou posterior; versões anteriores param com um prompt para executar `/login`, que não pode atualizar credenciais AWS. Adicione o comando ao seu `settings.json`:

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

Com `awsAuthRefresh` configurado, `/login` mostra uma opção **Claude Platform on AWS · atualizar credenciais** em **Usando plataformas de terceiros**. Selecioná-la executa o comando configurado e relê suas credenciais AWS sem reiniciar Claude Code.

**Opção B: Chave de API do Workspace**

Uma chave de API do workspace é um segredo de longa duração, útil quando você não quer gerenciar credenciais AWS federadas. Gere uma no Console AWS em **Claude Platform on AWS → API keys** e defina-a como `ANTHROPIC_AWS_API_KEY`:

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

A chave é enviada como `x-api-key` e tem precedência sobre SigV4, portanto qualquer credencial AWS em seu ambiente é ignorada. Chaves de API de uma organização Claude Console separada não funcionarão aqui.

Trate chaves de API do workspace como qualquer outra credencial de produção. O bloco `env` do [arquivo de configurações do usuário](/docs/pt/settings) é uma maneira conveniente de escopar a chave para sua máquina sem exportá-la globalmente.

<Note>
  Os comandos `/login` e `/logout` não o autenticam em uma assinatura Claude.ai para Claude Platform on AWS. A autenticação é executada através de suas credenciais AWS ou chave de API do workspace. A exceção é a opção **atualizar credenciais** que `/login` mostra quando `awsAuthRefresh` está configurado, que relê suas credenciais AWS conforme descrito acima.
</Note>

<h3 id="2-configure-claude-code">
  2. Configure Claude Code
</h3>

Defina as variáveis de ambiente que rotearão Claude Code através do Claude Platform on AWS em vez da API Anthropic padrão.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` é obrigatório e é enviado em cada solicitação como o cabeçalho `anthropic-workspace-id`. A URL base é calculada a partir de `AWS_REGION` como `https://aws-external-anthropic.{region}.api.aws`. Para substituir a URL diretamente, defina `ANTHROPIC_AWS_BASE_URL`.

Claude Platform on AWS é opt-in mesmo quando credenciais AWS estão presentes em seu ambiente. Amazon Bedrock e Microsoft Foundry têm precedência no roteamento de provedores, portanto desdefina `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_FOUNDRY` se estiverem definidas.

<h3 id="3-pin-model-versions">
  3. Fixe versões de modelo
</h3>

Claude Platform on AWS usa os mesmos IDs de modelo que a API Claude direta.

Os aliases padrão `fable`, `opus`, `sonnet` e `haiku` resolvem para os padrões integrados do Claude Code para Claude Platform on AWS, que podem ficar atrás da versão mais recente. Sem `ANTHROPIC_DEFAULT_OPUS_MODEL`, o alias `opus` resolve para Opus 4.8. Antes da v2.1.207, ele resolvia para Opus 4.7.

Se você implantar Claude Code para uma equipe, fixe os IDs de modelo explicitamente para que um novo lançamento não mova todos de uma vez:

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

Para a lista completa de IDs de modelo e aliases, consulte [Visão geral de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Para outras variáveis relacionadas a modelos, consulte [Configuração de modelo](/docs/pt/model-config).

[Prompt caching](/docs/pt/prompt-caching) é ativado automaticamente. Para solicitar um TTL de cache de 1 hora em vez do padrão de 5 minutos, defina `ENABLE_PROMPT_CACHING_1H=1`. A API cobra gravações de cache de 1 hora a uma taxa mais alta. Consulte [preços de prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) para as taxas.

<h2 id="use-the-agent-sdk">
  Use o Agent SDK
</h2>

O [Agent SDK](/docs/pt/agent-sdk/overview) lê as mesmas variáveis de ambiente que a CLI, portanto qualquer programa que gere o subprocesso Claude Code pode direcionar Claude Platform on AWS exportando `CLAUDE_CODE_USE_ANTHROPIC_AWS`, `ANTHROPIC_AWS_WORKSPACE_ID` e `ANTHROPIC_AWS_API_KEY` ou credenciais AWS antes da chamada.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

Este exemplo depende da cadeia de credenciais AWS ambiente para SigV4. Para autenticar com uma chave de API do workspace em vez disso, defina `ANTHROPIC_AWS_API_KEY` da mesma forma. Para a superfície mais ampla do Agent SDK, consulte [Visão geral do Agent SDK](/docs/pt/agent-sdk/overview).

<h2 id="route-through-a-corporate-proxy">
  Rotear através de um proxy corporativo
</h2>

Para rotear tráfego através de um proxy ou [gateway LLM](/docs/pt/llm-gateway), defina `ANTHROPIC_AWS_BASE_URL` para o endereço do proxy. Claude Code envia solicitações para essa URL com os mesmos cabeçalhos de workspace e autenticação, portanto qualquer gateway que os encaminhe inalterados funciona.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

Se seu gateway assina solicitações em si, defina `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1` para que Claude Code envie solicitações não assinadas e deixe o gateway adicionar cabeçalhos SigV4 antes de encaminhar para AWS. Se o gateway requer seu próprio token, defina-o em `ANTHROPIC_AUTH_TOKEN`.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Execute `/status` para ver o provedor resolvido e qualquer ID de workspace, região, substituição de URL base e configuração de salto de autenticação explicitamente configurados. Esta é a maneira mais rápida de confirmar que Claude Code está direcionando Claude Platform on AWS.

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  `403 Forbidden` ou `AccessDenied` em cada solicitação
</h3>

O principal IAM que Claude Code resolveu provavelmente não tem permissão para invocar o serviço Anthropic em seu workspace. Verifique a função anexada ao seu perfil AWS ou ao executor que iniciou Claude Code, e verifique se ela tem as ações `aws-external-anthropic` documentadas na [referência de ação IAM](https://platform.claude.com/docs/pt/api/claude-platform-on-aws-iam-actions).

Se você definir `ANTHROPIC_AWS_API_KEY`, a chave tem precedência sobre SigV4 e uma chave obsoleta produz o mesmo erro. Regenere a chave no Console AWS em **Claude Platform on AWS → API keys** ou desdefina a variável para voltar às suas credenciais AWS.

<h3 id="requests-fail-with-a-missing-workspace-error">
  Solicitações falham com um erro de workspace ausente
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID` provavelmente está desdefido ou vazio. Cada solicitação do Claude Platform on AWS deve incluir o ID do workspace. Não é implícito por suas credenciais AWS. Encontre o ID em **Workspaces** na página de serviço do Console AWS e exporte-o antes de iniciar Claude Code.

<h3 id="requests-still-go-to-api-anthropic-com">
  Solicitações ainda vão para `api.anthropic.com`
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS` provavelmente está desdefido ou definido para um valor que não é analisado como verdadeiro. Defina-o como `1` e execute `/status` para confirmar o provedor resolvido. Se `CLAUDE_CODE_USE_BEDROCK` ou `CLAUDE_CODE_USE_FOUNDRY` também estiver definido, esses têm precedência sobre Claude Platform on AWS.

<h2 id="additional-resources">
  Recursos adicionais
</h2>

A assinatura do Claude Platform on AWS, configuração de workspace e IAM que vem antes de configurar Claude Code é coberta na documentação da plataforma:

* [Visão geral do Claude Platform on AWS](https://platform.claude.com/docs/pt/build-with-claude/claude-platform-on-aws): assinatura, configuração de workspace e referência de produto
* [Referência de ação IAM](https://platform.claude.com/docs/pt/api/claude-platform-on-aws-iam-actions): permissões e políticas gerenciadas
