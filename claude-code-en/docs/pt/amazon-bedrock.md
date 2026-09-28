> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code no Amazon Bedrock

> Saiba como configurar Claude Code através do Amazon Bedrock, incluindo configuração, configuração de IAM e resolução de problemas.

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

<ContactSalesCard surface="bedrock" />

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de configurar Claude Code com Amazon Bedrock, certifique-se de que você tem:

* Uma conta AWS com acesso ao Amazon Bedrock habilitado
* Acesso aos modelos Claude desejados (por exemplo, Claude Sonnet 4.6) no Amazon Bedrock
* AWS CLI instalado e configurado (opcional - necessário apenas se você não tiver outro mecanismo para obter credenciais)
* Permissões IAM apropriadas

Para entrar com suas próprias credenciais do Amazon Bedrock, siga [Entrar com Amazon Bedrock](#sign-in-with-bedrock) abaixo. Para implantar Claude Code em toda uma equipe, use as etapas de [configuração manual](#set-up-manually) e [fixe suas versões de modelo](#4-pin-model-versions) antes de fazer o lançamento.

<h2 id="sign-in-with-bedrock">
  Entrar com Bedrock
</h2>

Se você tem credenciais da AWS e deseja começar a usar Claude Code através do Amazon Bedrock, o assistente de login o guia através do processo. Você completa os pré-requisitos do lado da AWS uma vez por conta; o assistente cuida do lado do Claude Code.

<Steps>
  <Step title="Ativar modelos Anthropic em sua conta AWS">
    No [console do Amazon Bedrock](https://console.aws.amazon.com/bedrock/), abra o catálogo de modelos, selecione um modelo Anthropic e envie o formulário de caso de uso. O acesso é concedido imediatamente após o envio. Consulte [Enviar detalhes do caso de uso](#1-submit-use-case-details) para AWS Organizations e [configuração do IAM](#iam-configuration) para as permissões que sua função precisa.
  </Step>

  <Step title="Iniciar Claude Code e escolher Amazon Bedrock">
    Execute `claude`. No prompt de login, selecione **plataforma de terceiros**, depois **Amazon Bedrock**. Se você já está conectado e vê o prompt de chat, execute `/setup-bedrock` para abrir o assistente. Até que `CLAUDE_CODE_USE_BEDROCK=1` seja definido, Claude Code [oculta o comando do menu de comandos](/docs/pt/commands#how-the-command-menu-matches-what-you-type); digite-o por completo.
  </Step>

  <Step title="Siga os prompts do assistente">
    Escolha como você se autentica na AWS: um perfil AWS detectado do seu diretório `~/.aws`, uma chave de API do Amazon Bedrock, uma chave de acesso e segredo, ou credenciais já em seu ambiente. O assistente solicita sua região, verifica quais modelos Claude sua conta pode invocar e permite que você os fixe. Ele salva o resultado no bloco `env` do seu [arquivo de configurações do usuário](/docs/pt/settings), para que você não precise exportar variáveis de ambiente você mesmo.
  </Step>
</Steps>

Depois de entrar, execute `/setup-bedrock` a qualquer momento para reabrir o assistente e alterar suas credenciais, região ou fixações de modelo. A etapa de fixação de modelo começa a partir de seus modelos atualmente fixados. O assistente escreve em `~/.claude/settings.json`, ou em `$CLAUDE_CONFIG_DIR/settings.json` quando [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars#variables) está definido.

<h2 id="set-up-manually">
  Configurar manualmente
</h2>

Para configurar o Amazon Bedrock através de variáveis de ambiente em vez do assistente, por exemplo em CI ou em um rollout empresarial com script, siga os passos abaixo.

<h3 id="1-submit-use-case-details">
  1. Enviar detalhes do caso de uso
</h3>

Antes de invocar um modelo Anthropic pela primeira vez, envie detalhes do caso de uso. Você faz isso uma vez por conta AWS.

1. Certifique-se de que você tem as permissões IAM corretas descritas abaixo
2. Navegue até o [console do Amazon Bedrock](https://console.aws.amazon.com/bedrock/)
3. Selecione um modelo Anthropic no **catálogo de modelos**
4. Preencha o formulário de caso de uso. O acesso é concedido imediatamente após o envio.

Se você usar AWS Organizations, pode enviar o formulário uma vez a partir da conta de gerenciamento usando a [API `PutUseCaseForModelAccess`](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html). Esta chamada requer a permissão IAM `bedrock:PutUseCaseForModelAccess`. A aprovação se estende às contas filhas automaticamente.

<h3 id="2-configure-aws-credentials">
  2. Configurar credenciais AWS
</h3>

Claude Code usa a cadeia de credenciais padrão do AWS SDK. Configure suas credenciais usando um destes métodos:

**Opção A: Configuração da AWS CLI**

```bash theme={null}
aws configure
```

**Opção B: Variáveis de ambiente (chave de acesso)**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**Opção C: Variáveis de ambiente (perfil SSO)**

Substitua `your-profile-name` pelo nome do seu perfil AWS antes de executar estes comandos.

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code solicita credenciais de função da região do IAM Identity Center nomeada pelo `sso_region` do perfil, que não precisa corresponder à região em que você executa o Amazon Bedrock. Na v2.1.207, a região do Amazon Bedrock substituía `sso_region`, portanto um perfil cuja instância do IAM Identity Center está em uma região diferente falhava ao autenticar com um erro `Session token not found or invalid`.

**Opção D: Credenciais do AWS Management Console**

```bash theme={null}
aws login
```

[Saiba mais](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html) sobre `aws login`.

**Opção E: Chaves de API do Amazon Bedrock**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

As chaves de API do Amazon Bedrock fornecem um método de autenticação mais simples sem precisar de credenciais AWS completas. [Saiba mais sobre as chaves de API do Amazon Bedrock](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/).

<h4 id="credential-caching-and-resolution-timeout">
  Cache de credenciais e tempo limite de resolução
</h4>

Claude Code resolve a cadeia de provedor de credenciais padrão AWS uma vez e mantém as credenciais resolvidas na memória. Ele as reutiliza até cinco minutos antes de expirarem, ou por uma hora quando não têm expiração, portanto um perfil apoiado por SSO solicita credenciais do IAM Identity Center aproximadamente uma vez por tempo de vida de credencial. Um erro de credencial da API limpa o cache, e a tentativa novamente resolve credenciais novas. Requer Claude Code v2.1.207 ou posterior.

O cache cobre todas as opções de credencial acima, exceto uma chave de API do Amazon Bedrock, que não usa a cadeia de provedor. Para resolver a cadeia em cada solicitação em vez disso, defina [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/pt/env-vars).

Cada resolução da cadeia expira após 60 segundos. Se uma etapa na cadeia travar, por exemplo um auxiliar `credential_process` que aguarda entrada que não pode receber, a solicitação falha com [`AWS default-chain credential resolve timed out`](/docs/pt/errors#aws-default-chain-credential-resolve-timed-out). Se sua cadeia executa um login interativo que legitimamente precisa de mais tempo, como SSO baseado em navegador com MFA através de um wrapper como `aws-vault`, aumente o limite em milissegundos com [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/pt/env-vars). Antes da v2.1.207, uma resolução de credencial travada deixava a solicitação aguardando indefinidamente.

Exceto quando você autentica com uma chave de API do Amazon Bedrock, o [assistente de configuração](#sign-in-with-bedrock) aplica o mesmo limite a cada chamada AWS que faz ao verificar suas credenciais, e à busca de credenciais antes de cada verificação de modelo. Durante a verificação de credenciais, uma verificação que excede o limite falha com [`Timed out after 60s waiting for AWS`](/docs/pt/errors#bedrock-setup-verification-timed-out-waiting-for-aws).

<h4 id="advanced-credential-configuration">
  Configuração avançada de credenciais
</h4>

Claude Code suporta atualização automática de credenciais para AWS SSO e provedores de identidade corporativos. Adicione estas configurações ao seu arquivo de configurações do Claude Code (veja [Settings](/docs/pt/settings) para localizações de arquivo).

Estas duas configurações têm diferentes condições de acionamento:

* **`awsAuthRefresh`**: executa apenas quando Claude Code detecta que suas credenciais AWS expiraram, localmente com base em seu timestamp ou quando a API retorna um erro de credencial, depois tenta novamente a solicitação com credenciais atualizadas.
* **`awsCredentialExport`**: executa no início da sessão e em cada recarga de credencial, mesmo quando as credenciais em sua cadeia de provedor de credenciais padrão AWS ainda são válidas. Use isto quando sua conta do Amazon Bedrock requer credenciais entre contas que diferem das que a cadeia de provedor padrão resolveria.

Antes de executar o comando `awsAuthRefresh`, Claude Code faz uma chamada STS `GetCallerIdentity` para confirmar que suas credenciais realmente expiraram, e pula o comando quando ainda funcionam. Claude Code envia esta verificação através de sua [configuração de proxy](/docs/pt/network-config#proxy-configuration), honrando `HTTPS_PROXY` e `NO_PROXY`. Antes da v2.1.239, Claude Code enviava esta verificação diretamente e travava na inicialização em redes que apenas permitem saída através de um proxy.

<h5 id="example-configuration">
  Configuração de exemplo
</h5>

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

<h5 id="configuration-settings-explained">
  Configurações explicadas
</h5>

**`awsAuthRefresh`**: Use isto para comandos que modificam o diretório `.aws`, como atualizar credenciais, cache SSO ou arquivos de configuração. A saída do comando é exibida ao usuário, mas entrada interativa não é suportada. Isto funciona bem para fluxos SSO baseados em navegador onde a CLI exibe uma URL ou código e você completa a autenticação no navegador.

**`awsCredentialExport`**: Use isto apenas se você não puder modificar `.aws` e deve retornar credenciais diretamente. A saída é capturada silenciosamente e não mostrada ao usuário. O comando deve gerar JSON neste formato:

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

A saída plana de `aws configure export-credentials --format process` também é aceita, com as mesmas chaves no nível superior em vez de aninhadas sob `Credentials`.

`Expiration` é opcional. Quando o comando retorna um `Expiration` ISO 8601 válido, Claude Code armazena em cache as credenciais até cinco minutos antes desse tempo. Sem ele, as credenciais são armazenadas em cache por uma hora.

Quando você configura `awsCredentialExport` sem `awsAuthRefresh`, Claude Code usa as credenciais exportadas diretamente e não re-resolve a cadeia de provedor de credenciais padrão AWS na inicialização. Requer Claude Code v2.1.206 ou posterior.

<h3 id="3-configure-claude-code">
  3. Configurar Claude Code
</h3>

Defina as seguintes variáveis de ambiente para habilitar o Amazon Bedrock:

```bash theme={null}
# Habilitar integração Bedrock
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # opcional se seu perfil AWS já define uma região

# Opcional: Substituir a região AWS para o modelo pequeno/rápido (Bedrock e Mantle).
# No Bedrock, não tem efeito sem ANTHROPIC_DEFAULT_HAIKU_MODEL
# ou o ANTHROPIC_SMALL_FAST_MODEL descontinuado definido.
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# Opcional: Substituir a URL do endpoint Bedrock para endpoints personalizados ou gateways
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

Ao habilitar o Amazon Bedrock para Claude Code, tenha em mente o seguinte:

* Você só precisa definir `AWS_REGION` para substituir a região do seu perfil AWS ou quando seu perfil não tem região. Claude Code resolve a região nesta ordem:

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * a `region` definida em seu perfil AWS ativo, lida do arquivo de credenciais compartilhadas AWS primeiro e depois do arquivo de configuração compartilhada, correspondendo à precedência do AWS SDK
  * `us-east-1`

  Se um valor de qualquer uma dessas fontes não parecer um nome de região, Claude Code o trata como não definido e continua para baixo na ordem. Por exemplo, Claude Code trata um valor contendo uma barra, ponto ou espaço como não definido.

  O perfil ativo é `AWS_PROFILE` se definido, caso contrário `default`. Defina `AWS_SHARED_CREDENTIALS_FILE` ou `AWS_CONFIG_FILE` para apontar para caminhos de arquivo não padrão.

  Execute `/status` para ver a região resolvida. Quando a região veio de seus arquivos de configuração AWS ou do fallback padrão, Claude Code também anota a fonte na saída `/status`.
* Ao usar o Amazon Bedrock, o comando `/logout` não está disponível, pois a autenticação é tratada através de credenciais AWS.
* A ferramenta WebSearch não está disponível no Amazon Bedrock. Veja [Comportamento da ferramenta WebSearch](/docs/pt/tools-reference#websearch-tool-behavior).
* Você pode usar arquivos de configurações para variáveis de ambiente como `AWS_PROFILE` que você não quer vazar para outros processos. Veja [Settings](/docs/pt/settings) para mais informações.

<h3 id="4-pin-model-versions">
  4. Fixar versões de modelo
</h3>

<Warning>
  Fixe versões de modelo específicas ao implantar para múltiplos usuários. Sem fixação, aliases de modelo como `sonnet` e `opus` resolvem para o padrão integrado do Claude Code para Amazon Bedrock, que pode ficar atrás da versão mais recente e pode ainda não estar disponível em sua conta. Claude Code [volta](#startup-model-checks) para um modelo anterior ou de nível inferior na inicialização quando o padrão não está disponível, mas fixar permite que você controle quando seus usuários se movem para um novo modelo.
</Warning>

Defina estas variáveis de ambiente para IDs de modelo específicos do Amazon Bedrock.

Sem `ANTHROPIC_DEFAULT_OPUS_MODEL`, o alias `opus` no Amazon Bedrock resolve para Opus 5.5, e sem `ANTHROPIC_DEFAULT_SONNET_MODEL`, o alias `sonnet` resolve para Sonnet 4.5. Este exemplo fixa cada alias para uma versão específica:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

Estes IDs usam o prefixo de perfil de inferência entre regiões `us.`. Se você usar um prefixo de região diferente ou perfis de inferência de aplicação, ajuste de acordo. Em regiões AWS GovCloud, use o prefixo `us-gov.`.

Para manter os modelos padrão integrados e alterar apenas seu prefixo preferido, defina [`ANTHROPIC_BEDROCK_REGION_PREFIX`](#cross-region-inference-profile-prefixes) em vez de fixar. A diferença aparece no que o alias `opus` resolve para:

| Você define                                                   | O alias `opus` resolve para                                                  |
| :------------------------------------------------------------ | :--------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` | `us.anthropic.claude-opus-4-8`, o ID exato que você fixou                    |
| `ANTHROPIC_BEDROCK_REGION_PREFIX=eu`                          | `eu.anthropic.claude-opus-5-5`, o padrão integrado com seu prefixo preferido |

Para IDs de modelo atuais e legados, veja [Visão geral de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Para a lista completa de variáveis de ambiente de fixação, veja [Configuração de modelo](/docs/pt/model-config#pin-models-for-third-party-deployments).

Claude Code usa estes modelos padrão quando nenhuma variável de fixação é definida:

| Tipo de modelo        | Modelo padrão                                                                               |
| :-------------------- | :------------------------------------------------------------------------------------------ |
| Modelo primário       | Opus 5.5, por exemplo `us.anthropic.claude-opus-5-5` em uma região `us-*`                   |
| Modelo pequeno/rápido | Sonnet 4.5, por exemplo `us.anthropic.claude-sonnet-4-5-20250929-v1:0` em uma região `us-*` |

Tarefas em segundo plano, como geração de título de sessão, usam o modelo pequeno/rápido, normalmente um modelo da classe Haiku. No Amazon Bedrock, Claude Code usa o modelo Sonnet padrão para tarefas em segundo plano porque Haiku pode não estar habilitado em todas as contas ou regiões. Duas seleções mudam qual modelo as carrega:

* Quando você seleciona um modelo primário com `--model`, `ANTHROPIC_MODEL` ou a configuração `model`, as tarefas em segundo plano usam esse modelo. Quando Claude Code inicia a sessão no modelo que você define com [`ANTHROPIC_DEFAULT_MODEL`](/docs/pt/model-config#set-a-default-model-for-new-sessions), as tarefas em segundo plano usam esse modelo também. Definir `ANTHROPIC_DEFAULT_OPUS_MODEL` sem `ANTHROPIC_DEFAULT_SONNET_MODEL` também conta como uma seleção, porque o modelo Sonnet integrado pode não estar habilitado em uma conta que direciona seu próprio Opus.
* Para usar Haiku para tarefas em segundo plano, defina `ANTHROPIC_DEFAULT_HAIKU_MODEL` para um ID de modelo que está disponível em sua conta.

<Warning>
  Os modelos Opus têm um preço por token mais alto do que os modelos Sonnet, portanto uma implantação que não fixa um modelo primário é cobrada à taxa Opus uma vez que atualiza para v2.1.207 ou posterior. Para manter Sonnet 4.5 como o modelo primário, defina `ANTHROPIC_MODEL` para seu ID de modelo completo. Uma implantação que direciona o padrão com `ANTHROPIC_DEFAULT_SONNET_MODEL` e não define `ANTHROPIC_DEFAULT_OPUS_MODEL` mantém seu modelo Sonnet direcionado como o padrão.
</Warning>

Antes da v2.1.280, o modelo primário no Amazon Bedrock era padrão para Opus 5 e o alias `opus` resolveu para Opus 5 a partir da v2.1.219. Na v2.1.207 através v2.1.218, o modelo primário no Amazon Bedrock era padrão para Opus 4.8 e o alias `opus` resolveu para Opus 4.8. Antes da v2.1.207, o modelo primário era padrão para Sonnet 4.5, o alias `opus` resolveu para Opus 4.6, e as tarefas em segundo plano sempre usavam o modelo primário.

Para personalizar modelos ainda mais, use um destes métodos:

```bash theme={null}
# Usando ID de perfil de inferência
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# Usando ARN de perfil de inferência de aplicação
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# Opcional: Desabilitar cache de prompt se necessário
# export DISABLE_PROMPT_CACHING=1

# Opcional: Solicitar TTL de cache de prompt de 1 hora em vez do padrão de 5 minutos
# export ENABLE_PROMPT_CACHING_1H=1
```

O TTL de cache de 1 hora é cobrado a uma taxa mais alta do que o padrão de 5 minutos. Veja [tempo de vida do cache](/docs/pt/prompt-caching#cache-lifetime). Para definir TTLs diferentes para sua conversa principal e para as solicitações que Claude Code faz fora dela, [escolha o TTL você mesmo](/docs/pt/prompt-caching#choose-the-ttl-yourself).

<Note>O cache de prompt pode não estar disponível em todas as regiões do Amazon Bedrock. Se as contagens de tokens de cache permanecerem em zero, verifique [modelos, regiões e limites suportados](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) na documentação do Amazon Bedrock.</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  Mapear cada versão de modelo para um perfil de inferência
</h4>

As variáveis de ambiente `ANTHROPIC_DEFAULT_*_MODEL` configuram um perfil de inferência por família de modelo. Se sua organização precisa expor várias versões da mesma família no seletor `/model`, cada uma roteada para seu próprio ARN de perfil de inferência de aplicação, use a configuração `modelOverrides` em seu [arquivo de configurações](/docs/pt/settings#where-settings-live) em vez disso.

Este exemplo mapeia quatro versões de Opus para ARNs distintos para que os usuários possam alternar entre elas sem contornar os perfis de inferência de sua organização:

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

Quando um usuário seleciona uma dessas versões em `/model`, Claude Code chama o Amazon Bedrock com o ARN mapeado. O mesmo mapeamento se aplica quando você passa o ID de modelo Anthropic diretamente através de `--model` ou `ANTHROPIC_MODEL`. Versões sem uma substituição voltam para o ID de modelo Amazon Bedrock integrado ou qualquer perfil de inferência correspondente descoberto na inicialização. Antes da v2.1.200, valores `--model` e `ANTHROPIC_MODEL` chegavam ao Amazon Bedrock como estavam sem passar pelo mapa de substituição. Veja [Substituir IDs de modelo por versão](/docs/pt/model-config#override-model-ids-per-version) para detalhes sobre como as substituições interagem com `availableModels` e outras configurações de modelo.

<h2 id="startup-model-checks">
  Verificações de modelo na inicialização
</h2>

Quando Claude Code inicia com Amazon Bedrock configurado, ele verifica se os modelos que pretende usar estão acessíveis em sua conta.

Se você fixou uma versão de modelo mais antiga do que o padrão atual do Claude Code, e sua conta pode invocar a versão mais recente, Claude Code solicita que você atualize a fixação. Aceitar escreve o novo ID do modelo em seu [arquivo de configurações do usuário](/docs/pt/settings) e reinicia Claude Code. Recusar é lembrado até a próxima mudança de versão padrão. Fixações que apontam para um [ARN de perfil de inferência de aplicação](#map-each-model-version-to-an-inference-profile) são ignoradas, pois são gerenciadas pelo seu administrador.

Se você não fixou um modelo e o padrão atual não está disponível em sua conta, Claude Code faz fallback para a sessão atual e mostra um aviso. Ele tenta versões anteriores do modelo padrão primeiro e, quando o padrão é um modelo Opus e nenhuma versão Opus está disponível, faz fallback para o modelo Sonnet padrão. O fallback não é persistido. Ative o modelo mais recente em sua conta Amazon Bedrock ou [fixe uma versão](#4-pin-model-versions) para tornar a escolha permanente.

Quando você inicia a sessão em uma versão específica de Sonnet ou Opus, por exemplo com `--model`, `ANTHROPIC_MODEL`, ou a [configuração `model`](/docs/pt/settings-reference#model), essa versão atua como o padrão fixado da sessão para o alias `sonnet` ou `opus` correspondente. Claude Code ignora a verificação de disponibilidade do padrão integrado que seu modelo substitui e inicia no modelo que você configurou, sem aviso de fallback.

Aliases de modelo como `opus` não atuam como fixações, e nem um ID de modelo que Claude Code não reconhece, como um ARN de perfil de inferência de aplicação.

<h2 id="cross-region-inference-profile-prefixes">
  Prefixos de perfil de inferência entre regiões
</h2>

Na [API Invoke](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) do Amazon Bedrock, Claude Code resolve seus modelos padrão integrados para IDs de [perfil de inferência entre regiões](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html); para rotear versões de modelo através de seus próprios perfis de inferência, consulte [Mapear cada versão de modelo para um perfil de inferência](#map-each-model-version-to-an-inference-profile). Esta tabela mostra o prefixo que Claude Code prefere para cada região AWS resolvida:

| Região AWS                | Prefixo   |
| :------------------------ | :-------- |
| `us-gov-*` (AWS GovCloud) | `us-gov.` |
| `us-*`                    | `us.`     |
| `eu-*`                    | `eu.`     |
| `ap-*`                    | `apac.`   |
| Todas as outras regiões   | `global.` |

Defina `ANTHROPIC_BEDROCK_REGION_PREFIX` para escolher o prefixo que Claude Code tenta primeiro; quando Claude Code pode verificar a disponibilidade do perfil e não encontra um perfil correspondente para um modelo, ele volta conforme descrito na ordem de resolução abaixo. Os valores válidos são `us`, `eu`, `apac`, `jp`, `au` e `global`. Por exemplo, defina como `global` quando sua conta tem perfis `global.` habilitados, mas Claude Code derivaria um específico de uma geografia a partir de sua região AWS. Requer Claude Code v2.1.224 ou posterior.

Este exemplo roteia os modelos padrão através de perfis `global.`:

```bash theme={null}
export ANTHROPIC_BEDROCK_REGION_PREFIX=global
# Em uma região us-*, o modelo primário agora resolve para
# global.anthropic.claude-opus-5-5 em vez de us.anthropic.claude-opus-5-5
```

O prefixo preferido é uma preferência, não uma garantia, quer venha de sua região ou da variável. Como Claude Code o aplica depende de se ele pode verificar a disponibilidade do perfil em sua conta:

* Quando Claude Code pode [listar os perfis de inferência](#iam-configuration) em sua conta, ele resolve cada modelo nesta ordem:
  1. O perfil com seu prefixo preferido.
  2. Qualquer perfil correspondente, para um modelo que não tem perfil com esse prefixo.
  3. O ID de modelo integrado com seu prefixo preferido, para um modelo que não tem nenhum perfil correspondente. Claude Code aplica este ID sem verificar a disponibilidade nesta etapa; as [verificações de modelo de inicialização](#startup-model-checks) ainda cobrem os modelos padrão da sessão.
* Quando a descoberta de perfil não está disponível, Claude Code aplica o prefixo sem verificar a disponibilidade. Se sua conta não tem perfis de inferência com esse prefixo habilitados, as solicitações falham com um erro 400.

Claude Code não reescreve IDs de perfil de inferência do Amazon Bedrock ou ARNs que você configura você mesmo, ou valores de [`modelOverrides`](#map-each-model-version-to-an-inference-profile); IDs de modelo no formato Anthropic resolvem através do [mesmo mapeamento que o seletor `/model`](#map-each-model-version-to-an-inference-profile). Claude Code também ignora a variável em dois casos:

* Em regiões AWS GovCloud, Claude Code sempre usa `us-gov.`, o único prefixo que roteia dentro da partição GovCloud.
* Quando você define um valor que não é um dos valores válidos, Claude Code volta para o prefixo preferido derivado da região.

<h2 id="iam-configuration">
  Configuração de IAM
</h2>

Crie uma política de IAM com as permissões necessárias para Claude Code:

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

Para permissões mais restritivas, você pode limitar o Resource para ARNs de perfil de inferência específicos.

`bedrock:GetInferenceProfile` permite que Claude Code resolva um [ARN de perfil de inferência de aplicação](#map-each-model-version-to-an-inference-profile) para seu modelo de fundação de suporte, que é usado para selecionar a forma de solicitação correta para esse modelo.

Se o token não tiver essa permissão, Claude Code se recupera automaticamente tentando novamente uma vez com a forma alternativa, portanto as solicitações ainda têm sucesso, mas cada novo modelo adiciona uma viagem extra. Conceder a permissão evita a tentativa novamente. Isso se aplica com mais frequência a implantações `AWS_BEARER_TOKEN_BEDROCK`, onde a política do token é normalmente mais restrita do que uma função IAM completa.

Para detalhes, veja [documentação de IAM do Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html).

<Note>
  Crie uma conta AWS dedicada para Claude Code para simplificar o rastreamento de custos e controle de acesso.
</Note>

<h2 id="1m-token-context-window">
  Janela de contexto de 1M de tokens
</h2>

Claude Sonnet 5, Opus 4.6 e posteriores, e Sonnet 4.6 suportam a [janela de contexto de 1M de tokens](https://platform.claude.com/docs/pt/build-with-claude/context-windows#context-window-sizes-by-model) no Amazon Bedrock. Sonnet 5 sempre é executado com a janela de 1M tanto na API Invoke quanto no [endpoint Mantle](#use-the-mantle-endpoint), sem nenhuma variante `[1m]` para selecionar. Para os outros modelos na API Invoke, Claude Code habilita automaticamente a janela de contexto estendida quando você seleciona uma variante de modelo de 1M.

O [assistente de configuração](#sign-in-with-bedrock) oferece uma opção de contexto de 1M quando fixa modelos. Para habilitá-lo para um modelo fixado manualmente em vez disso, acrescente `[1m]` ao ID do modelo. Veja [Fixar modelos para implantações de terceiros](/docs/pt/model-config#pin-models-for-third-party-deployments) para detalhes.

<h2 id="service-tiers">
  Camadas de serviço
</h2>

[Camadas de serviço do Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html) permitem que você negocie custo contra latência. Defina `ANTHROPIC_BEDROCK_SERVICE_TIER` como `default`, `flex` ou `priority`:

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code envia isso como o cabeçalho `X-Amzn-Bedrock-Service-Tier` em cada solicitação. A disponibilidade de camada varia por modelo e região. A capacidade reservada usa um [ARN de throughput provisionado](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) como o ID do modelo em vez desta configuração.

<h2 id="aws-guardrails">
  AWS Guardrails
</h2>

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) permitem que você implemente filtragem de conteúdo para Claude Code. Crie um Guardrail no [console do Amazon Bedrock](https://console.aws.amazon.com/bedrock/), publique uma versão, então adicione os cabeçalhos do Guardrail ao seu [arquivo de configurações](/docs/pt/settings). Habilite inferência entre regiões em seu Guardrail se você estiver usando perfis de inferência entre regiões.

Exemplo de configuração:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

Se sua organização entrega os cabeçalhos do guardrail através de uma política de [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), eles contam como [configurações que precisam de aprovação](/docs/pt/server-managed-settings#environment-variables-and-the-approval-dialog).

<h2 id="use-the-mantle-endpoint">
  Usar o endpoint Mantle
</h2>

Mantle é um endpoint do Amazon Bedrock que serve modelos Claude através da forma de API Anthropic nativa em vez da API Invoke do Amazon Bedrock. Ele usa as mesmas [credenciais AWS](#2-configure-aws-credentials) e [configuração `awsAuthRefresh`](#advanced-credential-configuration).

Mantle tem suas próprias ações IAM sob o prefixo `bedrock-mantle:`, portanto as ações `bedrock:` em [configuração IAM](#iam-configuration) não a cobrem. Conceda à sua identidade IAM `bedrock-mantle:CreateInference` para inferência e `bedrock-mantle:CountTokens` para contagem de tokens. Veja [Fazendo solicitações de inferência](https://docs.aws.amazon.com/bedrock/latest/userguide/inference.html) e [Contando tokens](https://docs.aws.amazon.com/bedrock/latest/userguide/count-tokens.html) na documentação da AWS, e a [referência de autorização de serviço](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrockpoweredbyawsmantle.html) para cada ação Mantle.

<h3 id="enable-mantle">
  Habilitar Mantle
</h3>

Com credenciais AWS já configuradas, defina `CLAUDE_CODE_USE_MANTLE` para rotear solicitações para o endpoint Mantle:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code constrói a URL do endpoint a partir da região AWS, resolvida com a mesma precedência que [Amazon Bedrock acima](#3-configure-claude-code). Para substituir a URL por um endpoint personalizado ou gateway, defina `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`.

Execute `/status` dentro do Claude Code para confirmar. A linha do provedor mostra `Amazon Bedrock (Mantle)` quando Mantle está ativo.

<h3 id="select-a-mantle-model">
  Selecionar um modelo Mantle
</h3>

Mantle usa IDs de modelo com prefixo `anthropic.` e sem sufixo de versão, por exemplo `anthropic.claude-sonnet-5` ou `anthropic.claude-haiku-4-5`. Os modelos disponíveis para sua conta dependem do que sua organização foi concedida; IDs de modelo adicionais estão listados em seus materiais de integração da AWS. Entre em contato com sua equipe de conta AWS para solicitar acesso aos modelos permitidos.

Defina o modelo com a flag `--model` ou com `/model` dentro do Claude Code:

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  Executar Mantle junto com a API Invoke
</h3>

Os modelos disponíveis para você no Mantle podem não incluir todos os modelos que você usa hoje. Definir tanto `CLAUDE_CODE_USE_BEDROCK` quanto `CLAUDE_CODE_USE_MANTLE` permite que Claude Code chame ambos os endpoints da mesma sessão. IDs de modelo que correspondem ao formato Mantle são roteados para Mantle, e todos os outros IDs de modelo vão para a API Invoke do Amazon Bedrock.

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

Para exibir um modelo Mantle no seletor `/model`, liste seu ID em `availableModels` em seu [arquivo de configurações](/docs/pt/settings). Esta configuração também restringe o seletor às entradas listadas. Listar `anthropic.claude-haiku-4-5` remove o alias simples `haiku` do seletor, então também liste prefixos de versão ou IDs completos para as versões que você quer manter selecionáveis. O ID Mantle e o alias `haiku` resolvem para a mesma família de modelo, então a mesclagem mantém apenas a entrada mais específica. Veja [Comportamento de mesclagem](/docs/pt/model-config#merge-behavior):

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

Entradas com o prefixo `anthropic.` são adicionadas como opções de seletor personalizadas e roteadas para Mantle. Substitua `anthropic.claude-haiku-4-5` pelo ID de modelo que sua conta foi concedida. Veja [Restringir seleção de modelo](/docs/pt/model-config#restrict-model-selection) para como `availableModels` interage com outras configurações de modelo.

Quando ambos os provedores estão ativos, `/status` mostra `Amazon Bedrock + Amazon Bedrock (Mantle)`.

<h3 id="route-mantle-through-a-gateway">
  Rotear Mantle através de um gateway
</h3>

Se sua organização roteia tráfego de modelo através de um [gateway LLM](/docs/pt/llm-gateway) centralizado que injeta credenciais AWS no lado do servidor, desabilite a autenticação no lado do cliente para que Claude Code envie solicitações sem assinaturas SigV4 ou cabeçalhos `x-api-key`:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Variáveis de ambiente Mantle
</h3>

Estas variáveis são específicas para o endpoint Mantle. Veja [Variáveis de ambiente](/docs/pt/env-vars) para a lista completa.

| Variável                                | Propósito                                                                              |
| :-------------------------------------- | :------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_USE_MANTLE`                | Habilitar o endpoint Mantle. Defina como `1` ou `true`.                                |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | Substituir a URL do endpoint Mantle padrão                                             |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | Pular autenticação no lado do cliente para configurações de proxy                      |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | Substituir região AWS para o modelo da classe Haiku (compartilhado com Amazon Bedrock) |

<h2 id="troubleshooting">
  Resolução de problemas
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  Loop de autenticação com SSO e proxies corporativos
</h3>

Se abas do navegador aparecem repetidamente ao usar AWS SSO, remova a configuração `awsAuthRefresh` do seu [arquivo de configurações](/docs/pt/settings). Isso pode ocorrer quando VPNs corporativas ou proxies de inspeção TLS interrompem o fluxo do navegador SSO. Claude Code trata a conexão interrompida como uma falha de autenticação, executa novamente `awsAuthRefresh` e entra em loop indefinidamente.

Se seu ambiente de rede interfere com fluxos SSO automáticos baseados em navegador, use `aws sso login` manualmente antes de iniciar Claude Code em vez de depender de `awsAuthRefresh`.

<h3 id="certificate-errors-behind-a-tls-inspecting-proxy">
  Erros de certificado atrás de um proxy de inspeção TLS
</h3>

Claude Code aplica sua configuração de [armazenamento de certificado CA](/docs/pt/network-config#ca-certificate-store) às suas solicitações para AWS, incluindo:

* Descoberta de modelo
* Contagem de tokens
* As chamadas de credencial de função STS e SSO que resolvem suas credenciais AWS
* Verificação de credencial e verificações de modelo do [assistente de configuração](#sign-in-with-bedrock)

Para essas solicitações, um certificado raiz corporativo em seu armazenamento de confiança do SO ou pacote `NODE_EXTRA_CA_CERTS` não precisa de configuração específica do Amazon Bedrock.

Antes da v2.1.260, Claude Code aplicava sua configuração de CA a essas solicitações apenas quando elas passavam por um proxy configurado, e em uma conexão direta confiavam apenas no armazenamento de certificado padrão do runtime.

Antes da v2.1.261, a busca de credencial atrás das verificações de modelo do assistente de configuração com a opção **Use credentials already in my environment** ainda confiava apenas no armazenamento de certificado padrão do runtime. Atrás de um proxy de inspeção TLS cujo certificado raiz está apenas no armazenamento do SO, as solicitações afetadas falharam com `unable to get local issuer certificate`, ou o assistente mostrou modelos como `unreachable`, enquanto solicitações de inferência tiveram sucesso. Atualize para v2.1.261 ou posterior.

<h3 id="region-issues">
  Problemas de região
</h3>

Se você encontrar problemas de região:

* Verifique disponibilidade de modelo: `aws bedrock list-inference-profiles --region your-region`
* Mude para uma região suportada: `export AWS_REGION=us-east-1`
* Considere usar perfis de inferência para acesso entre regiões

Se você receber um erro "on-demand throughput isn't supported":

* Especifique o modelo como um ID de [perfil de inferência](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)

Claude Code usa a [API Invoke](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) do Amazon Bedrock e não suporta a API Converse.

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  Erros de streaming atrás de um gateway ou proxy
</h3>

Amazon Bedrock transmite respostas `InvokeModelWithResponseStream` em um formato de evento binário event-stream com o cabeçalho `Content-Type: application/vnd.amazon.eventstream`. Um gateway ou proxy entre Claude Code e Amazon Bedrock deve encaminhar o corpo da resposta e seus cabeçalhos, incluindo `Content-Type`, exatamente como Amazon Bedrock os enviou.

Se o gateway reescrever `Content-Type` para outro valor, Claude Code rejeita a resposta com um erro que começa com `Bedrock streaming response has content-type`, nomeando o valor que recebeu. A reescrita comum é `text/event-stream`, de uma integração que re-emite o stream como server-sent events.

Se o gateway descartar ou deixar em branco o cabeçalho, Claude Code assume que o corpo é o event stream do Amazon Bedrock e o decodifica, então um corpo que o gateway passou sem modificações continua transmitindo.

Se um gateway que descarta o cabeçalho também re-emite o stream como server-sent events, Claude Code não consegue decodificar o corpo e volta para um caminho mais lento sem streaming em cada turno: cada resposta aparece apenas quando está completa em vez de transmitir. Nesse caso, defina [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_DEFAULT=1`](/docs/pt/env-vars) para que Claude Code leia o corpo como server-sent events.

Para corrigir o erro ou o fallback, configure o gateway para encaminhar o corpo da resposta `InvokeModelWithResponseStream` e seu cabeçalho `Content-Type` sem modificações.

Um gateway que converte o stream para server-sent events não está mais servindo a API do Amazon Bedrock. Se também aceita solicitações da API Messages do Anthropic, conecte-se a ele como um [gateway LLM](/docs/pt/llm-gateway-connect) com `ANTHROPIC_BASE_URL` em vez de `CLAUDE_CODE_USE_BEDROCK`.

<h3 id="zero-token-counts-in-/context">
  Contagens de token zero em /context
</h3>

O comando `/context` conta tokens para cada grupo de ferramentas enviando os esquemas de ferramentas para a API count-tokens do Amazon Bedrock. Em versões do Claude Code anteriores à v2.1.196, Amazon Bedrock rejeitou essa solicitação porque os esquemas carregavam campos que sua API count-tokens não aceita, então cada grupo de ferramentas mostrava 0 tokens. Outras linhas na análise, como mensagens e arquivos de memória, não são afetadas.

Atualize para v2.1.196 ou posterior.

<h3 id="mantle-endpoint-errors">
  Erros de endpoint Mantle
</h3>

Se `/status` não mostra `Amazon Bedrock (Mantle)` depois que você defina `CLAUDE_CODE_USE_MANTLE`, a variável não está chegando ao processo. Confirme que ela é exportada no shell onde você lançou `claude`, ou defina-a no bloco `env` do seu [arquivo de configurações](/docs/pt/settings).

O que um `403` do endpoint Mantle significa depende de se o erro nomeia uma ação IAM:

* Se o erro nomeia uma ação `bedrock-mantle:`, conceda à sua identidade IAM essa ação.
* Se o erro não nomeia nenhuma ação e suas credenciais são válidas, sua conta AWS não foi concedida acesso ao modelo que você solicitou. Entre em contato com sua equipe de conta AWS para solicitar acesso.

Um `400` que nomeia o ID do modelo significa que esse modelo não é servido no Mantle. Mantle tem seu próprio lineup de modelo separado do catálogo Amazon Bedrock padrão, então IDs de perfil de inferência como `us.anthropic.claude-sonnet-4-6` não funcionarão. Use um ID de formato Mantle, ou habilite [ambos os endpoints](#run-mantle-alongside-the-invoke-api) para que Claude Code roteia cada solicitação para o endpoint onde o modelo está disponível.

<h2 id="additional-resources">
  Recursos adicionais
</h2>

* [Documentação do Amazon Bedrock](https://docs.aws.amazon.com/bedrock/)
* [Preços do Amazon Bedrock](https://aws.amazon.com/bedrock/pricing/)
* [Perfis de inferência do Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Burndown de token do Amazon Bedrock e cotas](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Claude Code no Amazon Bedrock: Guia de Configuração Rápida](https://builder.aws.com/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Implementação de Monitoramento do Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
