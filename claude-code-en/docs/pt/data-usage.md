> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Uso de dados

> Saiba mais sobre as políticas de uso de dados da Anthropic para Claude

<h2 id="data-policies">
  Políticas de dados
</h2>

<h3 id="data-training-policy">
  Política de treinamento de dados
</h3>

**Usuários consumidores (planos Free, Pro e Max)**:
Oferecemos a você a opção de permitir que seus dados sejam usados para melhorar futuros modelos Claude. Treinaremos novos modelos usando dados de contas Free, Pro e Max quando essa configuração estiver ativada (inclusive quando você usa Claude Code dessas contas).

**Usuários comerciais**: (planos Team e Enterprise, API, plataformas de terceiros e Claude Gov) mantêm as políticas existentes: a Anthropic não treina modelos generativos usando código ou prompts enviados para Claude Code sob termos comerciais, a menos que o cliente tenha optado por fornecer seus dados para melhorias de modelo (por exemplo, o [Development Partner Program](https://support.claude.com/pt/articles/11174108-about-the-development-partner-program)).

<h3 id="development-partner-program">
  Development Partner Program
</h3>

Se você optar explicitamente por métodos para nos fornecer materiais para treinar, como através do [Development Partner Program](https://support.claude.com/pt/articles/11174108-about-the-development-partner-program), podemos usar esses materiais fornecidos para treinar nossos modelos. Um administrador da organização pode optar explicitamente pelo Development Partner Program para sua organização. Observe que este programa está disponível apenas para API de primeira parte da Anthropic, e não para usuários de Amazon Bedrock ou Google Cloud's Agent Platform.

<h3 id="feedback-using-the-/feedback-command">
  Feedback usando o comando `/feedback`
</h3>

Se você optar por nos enviar feedback sobre Claude Code usando o comando `/feedback`, podemos usar seu feedback para melhorar nossos produtos e serviços. As transcrições compartilhadas via `/feedback` são retidas por 5 anos.

<h3 id="session-quality-surveys">
  Pesquisas de qualidade de sessão
</h3>

Quando você vê o prompt "How is Claude doing this session?" em Claude Code, responder a esta pesquisa, inclusive selecionando "Dismiss", registra apenas sua classificação. Não coletamos ou armazenamos nenhuma transcrição de conversa, entradas, saídas ou outros dados de sessão como parte da pesquisa de classificação em si. Diferentemente do feedback com polegar para cima/para baixo ou relatórios `/feedback`, esta pesquisa de qualidade de sessão é uma métrica simples de satisfação do produto.

Após a pesquisa de classificação, você pode ver uma pergunta de acompanhamento separada perguntando "Can Anthropic look at your session transcript to help us improve Claude Code?" (Pode a Anthropic examinar sua transcrição de sessão para nos ajudar a melhorar Claude Code?). Esta é uma segunda etapa opcional distinta da classificação:

* **Yes**: carrega sua transcrição de conversa, qualquer transcrição de subagentos e o arquivo de log de sessão bruto do disco para a Anthropic. Padrões conhecidos de chave de API e token são redatados antes do carregamento. Código-fonte, conteúdo de arquivo e outro conteúdo de conversa são carregados como estão. As transcrições compartilhadas são retidas por até 6 meses. No Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e sessões do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) conectadas, Yes escreve o mesmo payload em um arquivo local em `~/.claude/feedback-bundles/` em vez de fazer upload; nada sai de sua máquina até que você encaminhe esse arquivo.
* **No** (Não): recusa sem enviar nada
* **Don't ask again** (Não perguntar novamente): recusa e impede que este acompanhamento apareça em futuras sessões

Nada é carregado a menos que você selecione explicitamente **Yes**. Organizações com [zero data retention](/docs/pt/zero-data-retention), ou onde o feedback de produto é desabilitado pela política da organização, ou onde `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` está definido, nunca veem este acompanhamento. Suas respostas a esta pesquisa, inclusive transcrições de sessão enviadas após a pesquisa de classificação, não afetam suas preferências de treinamento de dados e não podem ser usadas para treinar nossos modelos de IA.

Para desabilitar essas pesquisas, defina `CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1`. A pesquisa também é desabilitada quando `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` está definido. Organizações que bloqueiam tráfego não essencial, mas capturam respostas de pesquisa através de seu próprio [coletor OpenTelemetry](/docs/pt/monitoring-usage) podem optar pela pesquisa novamente definindo `CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL=1`. A pesquisa então registra classificações apenas no coletor configurado. O acompanhamento de compartilhamento de transcrição e todo o outro tráfego de feedback vinculado à Anthropic permanecem desabilitados. Para controlar a frequência em vez de desabilitar, defina [`feedbackSurveyRate`](/docs/pt/settings#available-settings) em seu arquivo de configurações para uma probabilidade entre `0` e `1`.

<h3 id="data-retention">
  Retenção de dados
</h3>

A Anthropic retém dados de Claude Code com base no tipo de conta e preferências.

**Usuários consumidores (planos Free, Pro e Max)**:

* Usuários que permitem o uso de dados para melhorias de modelo: período de retenção de 5 anos para suportar desenvolvimento de modelo e melhorias de segurança
* Usuários que não permitem o uso de dados para melhorias de modelo: período de retenção de 30 dias
* As configurações de privacidade podem ser alteradas a qualquer momento em [claude.ai/settings/data-privacy-controls](https://claude.ai/settings/data-privacy-controls).

**Usuários comerciais (Team, Enterprise e API)**:

* Padrão: período de retenção de 30 dias
* [Zero data retention](/docs/pt/zero-data-retention): disponível para Claude Code no Claude for Enterprise. ZDR não está incluído no plano Enterprise padrão; é habilitado por organização após sua equipe de conta confirmar a elegibilidade
* Cache local: os clientes de Claude Code armazenam transcrições de sessão localmente em texto simples em `~/.claude/projects/` por 30 dias por padrão para permitir retomada de sessão. Ajuste o período com `cleanupPeriodDays`. Consulte [dados da aplicação](/docs/pt/claude-directory#application-data) para saber o que é armazenado e como limpá-lo.

Você pode excluir sessões individuais de Claude Code na web a qualquer momento. Excluir uma sessão remove permanentemente os dados de evento da sessão. Para instruções sobre como excluir sessões, consulte [Excluir sessões](/docs/pt/claude-code-on-the-web#delete-sessions).

Saiba mais sobre práticas de retenção de dados em nosso [Privacy Center](https://privacy.anthropic.com/).

Para detalhes completos, consulte nossos [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms) (para usuários de Team, Enterprise e API) ou [Consumer Terms](https://www.anthropic.com/legal/consumer-terms) (para usuários de Free, Pro e Max) e [Privacy Policy](https://www.anthropic.com/legal/privacy).

<h2 id="data-access">
  Acesso a dados
</h2>

Para todos os usuários de primeira parte, você pode aprender mais sobre quais dados são registrados para [Claude Code local](#local-claude-code-data-flow-and-dependencies) e [Claude Code remoto](#cloud-execution-data-flow-and-dependencies). As sessões de [Remote Control](/docs/pt/remote-control) seguem o fluxo de dados local, pois toda a execução acontece em sua máquina; enquanto conectado, a transcrição da sessão também é armazenada nos servidores da Anthropic para sincronizar a conversa entre dispositivos, conforme descrito em [Conexão e segurança](/docs/pt/remote-control#connection-and-security). Observe que para Claude Code remoto, Claude acessa o repositório onde você inicia sua sessão de Claude Code. Claude não acessa repositórios que você conectou mas não iniciou uma sessão.

<h2 id="local-claude-code-data-flow-and-dependencies">
  Local Claude Code: Fluxo de dados e dependências
</h2>

O diagrama abaixo mostra como Claude Code se conecta a serviços externos durante a instalação e operação normal. Linhas sólidas indicam conexões obrigatórias, enquanto linhas tracejadas representam fluxos de dados opcionais ou iniciados pelo usuário.

<img src="https://mintcdn.com/claude-code/YR4DRZyI3CdsXkiT/images/claude-code-data-flow.svg?fit=max&auto=format&n=YR4DRZyI3CdsXkiT&q=85&s=2846ea92cfc2297b8620c31c82b482ad" alt="Diagram showing Claude Code's external connections: install/update connects to the distribution server, and user requests connect to Anthropic's Console auth and public-api, with optional telemetry flows carrying metrics and error reports to Anthropic and third-party services. Feedback sent with /feedback goes to Google Cloud Storage and optionally creates a GitHub issue" width="720" height="520" data-path="images/claude-code-data-flow.svg" />

Claude Code é executado localmente. Para interagir com o LLM, Claude Code envia dados pela rede. Esses dados incluem todos os prompts do usuário e saídas do modelo, criptografados em trânsito via TLS 1.2+. Claude Code é compatível com a maioria dos VPNs e proxies LLM populares.

A criptografia em repouso depende do seu provedor de modelo:

| Provedor                      | Criptografia em repouso                                                                                                                                 |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Anthropic API                 | Criptografia de disco em nível de infraestrutura (AES-256). Ative [Zero Data Retention](/docs/pt/zero-data-retention) para nenhuma persistência no servidor. |
| Amazon Bedrock                | AES-256 com chaves gerenciadas pela AWS. Chaves gerenciadas pelo cliente disponíveis via AWS KMS.                                                       |
| Google Cloud's Agent Platform | Chaves de criptografia gerenciadas pelo Google. CMEK disponível.                                                                                        |
| Microsoft Foundry             | Solicitações são roteadas para infraestrutura Anthropic com criptografia de disco AES-256.                                                              |

Claude Code é construído nas APIs da Anthropic. Para detalhes sobre controles de segurança da API, incluindo procedimentos de logging de API, consulte os artefatos de conformidade no [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="cloud-execution-data-flow-and-dependencies">
  Cloud execution: Fluxo de dados e dependências
</h3>

Ao usar [Claude Code on the web](/docs/pt/claude-code-on-the-web), as sessões são executadas em máquinas virtuais gerenciadas pela Anthropic em vez de localmente. Em ambientes de nuvem:

* **Armazenamento de código e dados:** Seu repositório é clonado para uma VM isolada. Código e dados de sessão estão sujeitos às políticas de retenção e uso para seu tipo de conta (consulte a seção Retenção de dados acima)
* **Credenciais:** A autenticação do GitHub é tratada através de um proxy seguro; suas credenciais do GitHub nunca entram na sandbox
* **Tráfego de rede:** Todo o tráfego de saída passa por um proxy de segurança para logging de auditoria e prevenção de abuso
* **Dados de sessão:** Prompts, alterações de código e saídas seguem as mesmas políticas de dados que o uso local de Claude Code

Para detalhes de segurança sobre execução em nuvem, consulte [Security](/docs/pt/security#cloud-execution-security).

<h2 id="telemetry-services">
  Serviços de telemetria
</h2>

Claude Code envia dois tipos de telemetria operacional: métricas de uso e relatórios de erro. Você pode desativar cada um individualmente com as variáveis de ambiente abaixo, ou desabilitar todo o tráfego não essencial de uma vez definindo `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`.

**Métricas**: latência, confiabilidade e padrões de uso, enviados para a Anthropic e para infraestrutura de logging de terceiros sobre TLS. As métricas nunca incluem seu código, prompts ou caminhos de arquivo. Defina `DISABLE_TELEMETRY=1` para desativar.

**Relatórios de erro**: mensagens de erro e stack traces dos internals próprios do Claude Code, enviados para um serviço de rastreamento de erros de terceiros sobre TLS. Claude Code remove padrões conhecidos de segredos, caminhos de arquivo, endereços de email e outras informações pessoais antes de qualquer coisa sair de sua máquina. Defina `DISABLE_ERROR_REPORTING=1` para desativar.

O relatório de erro está ativado apenas quando todos estes se aplicam:

* você faz login com uma assinatura Claude Pro ou Max
* você está executando Claude Code v2.1.198 ou posterior
* você está se conectando diretamente à API Claude
* sua organização não tem um acordo de retenção zero de dados ou HIPAA

Quando você executa o comando `/feedback`, uma cópia do histórico de conversa incluindo código é enviada para a Anthropic. Antes de enviar, você escolhe quanto histórico incluir: apenas a sessão atual, que é o padrão, ou também outras sessões do mesmo projeto nos últimos 24 horas ou 7 dias. Os dados são criptografados em trânsito via TLS e armazenados no Google Cloud Storage, que criptografa dados armazenados em repouso por padrão. Opcionalmente, um problema do GitHub é criado no repositório público. Para desabilitar, defina a variável de ambiente `DISABLE_FEEDBACK_COMMAND` como `1`.

Quando você usa um provedor de terceiros como Amazon Bedrock ou Google Cloud's Agent Platform, ou não tem credenciais da Anthropic configuradas, `/feedback` escreve o relatório em um arquivo local sob `~/.claude/feedback-bundles/` em vez de enviá-lo para a Anthropic. Padrões conhecidos de chave de API e token são removidos antes do arquivo ser escrito. Nada sai de sua máquina até que você envie esse arquivo para seu representante de conta da Anthropic ou o anexe a uma solicitação de suporte.

<h2 id="default-behaviors-by-api-provider">
  Comportamentos padrão por provedor de API
</h2>

Por padrão, relatório de erros, telemetria e relatório de bugs são desabilitados ao usar Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou Claude Platform on AWS. Pesquisas de qualidade de sessão e a verificação de segurança de domínio WebFetch são exceções e são executadas independentemente do provedor. Em uma sessão de [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) conectada, análise de uso, relatório de erros e classificações de pesquisa para Anthropic são desabilitados pela credencial do gateway em si, sem configuração para reabilitá-los. Você pode desabilitar todo o tráfego não essencial, incluindo pesquisas, de uma vez definindo `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Esta variável não afeta a verificação WebFetch, que tem seu próprio opt-out. Aqui estão os comportamentos padrão completos:

| Serviço                                          | Claude API                                                                                                                    | Google Cloud's Agent Platform API                                                                | Amazon Bedrock API                                                                               | Microsoft Foundry API                                                                            | Claude Platform on AWS                                                                           |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------ |
| **Métricas**                                     | Padrão ativado.<br />`DISABLE_TELEMETRY=1` para desabilitar.                                                                  | Padrão desativado.<br />`CLAUDE_CODE_USE_VERTEX` deve ser 1.                                     | Padrão desativado.<br />`CLAUDE_CODE_USE_BEDROCK` deve ser 1.                                    | Padrão desativado.<br />`CLAUDE_CODE_USE_FOUNDRY` deve ser 1.                                    | Padrão desativado.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` deve ser 1.                              |
| **Relatórios de erro**                           | Ativado para inscrições Pro e Max em v2.1.198+, caso contrário desativado.<br />`DISABLE_ERROR_REPORTING=1` para desabilitar. | Padrão desativado.<br />`CLAUDE_CODE_USE_VERTEX` deve ser 1.                                     | Padrão desativado.<br />`CLAUDE_CODE_USE_BEDROCK` deve ser 1.                                    | Padrão desativado.<br />`CLAUDE_CODE_USE_FOUNDRY` deve ser 1.                                    | Padrão desativado.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` deve ser 1.                              |
| **Claude API (relatórios `/feedback`)**          | Padrão ativado.<br />`DISABLE_FEEDBACK_COMMAND=1` para desabilitar.                                                           | Padrão desativado.<br />`CLAUDE_CODE_USE_VERTEX` deve ser 1.                                     | Padrão desativado.<br />`CLAUDE_CODE_USE_BEDROCK` deve ser 1.                                    | Padrão desativado.<br />`CLAUDE_CODE_USE_FOUNDRY` deve ser 1.                                    | Padrão desativado.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` deve ser 1.                              |
| **Pesquisas de qualidade de sessão**             | Padrão ativado.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desabilitar.                                                | Padrão ativado.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desabilitar.                   | Padrão ativado.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desabilitar.                   | Padrão ativado.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desabilitar.                   | Padrão ativado.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desabilitar.                   |
| **Verificação de segurança de domínio WebFetch** | Padrão ativado.<br />`skipWebFetchPreflight: true` em [settings](/docs/pt/settings) para desabilitar.                              | Padrão ativado.<br />`skipWebFetchPreflight: true` em [settings](/docs/pt/settings) para desabilitar. | Padrão ativado.<br />`skipWebFetchPreflight: true` em [settings](/docs/pt/settings) para desabilitar. | Padrão ativado.<br />`skipWebFetchPreflight: true` em [settings](/docs/pt/settings) para desabilitar. | Padrão ativado.<br />`skipWebFetchPreflight: true` em [settings](/docs/pt/settings) para desabilitar. |

Todas as variáveis de ambiente podem ser verificadas em `settings.json` (consulte [referência de configurações](/docs/pt/settings)).

A partir da v2.1.126, quando uma plataforma host define `CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`, as métricas são ativadas por padrão para Google Cloud's Agent Platform, Amazon Bedrock e Microsoft Foundry, e seguem o opt-out padrão `DISABLE_TELEMETRY`. O relatório de erros e os relatórios `/feedback` permanecem desativados por padrão nesses provedores.

<h3 id="webfetch-domain-safety-check">
  Verificação de segurança de domínio WebFetch
</h3>

Antes de buscar uma URL, a ferramenta WebFetch envia o nome do host solicitado para `api.anthropic.com` para verificá-lo em relação a uma lista de bloqueio de segurança mantida pela Anthropic. Apenas o nome do host é enviado, não a URL completa, caminho ou conteúdo da página. Os resultados são armazenados em cache por nome do host por cinco minutos.

Esta verificação é executada independentemente de qual provedor de modelo você usa e não é afetada por `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Se sua rede bloqueia `api.anthropic.com`, as solicitações WebFetch falham até que você permita o domínio ou defina `skipWebFetchPreflight: true` em [settings](/docs/pt/settings). Desabilitar a verificação significa que WebFetch tenta recuperar qualquer URL sem consultar a lista de bloqueio, portanto combine com [regras de permissão `WebFetch`](/docs/pt/permissions#webfetch) se precisar restringir quais domínios Claude pode acessar.
