> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gerencie custos de forma eficaz

> Rastreie o uso de tokens, defina limites de gastos da equipe e reduza os custos do Claude Code com gerenciamento de contexto, seleção de modelo, configurações de pensamento estendido e hooks de pré-processamento.

Claude Code cobra pelo consumo de tokens da API. Para preços do plano de assinatura (Pro, Max, Team, Enterprise), consulte [claude.com/pricing](https://claude.com/pricing). Os custos por desenvolvedor variam amplamente com base na seleção de modelo, tamanho da base de código e padrões de uso, como executar múltiplas instâncias ou automação.

Em implantações empresariais, o custo médio é de cerca de \$13 por desenvolvedor por dia ativo e \$150-250 por desenvolvedor por mês, com custos permanecendo abaixo de \$30 por dia ativo para 90% dos usuários. Para estimar gastos para sua própria equipe, comece com um pequeno grupo piloto e use as ferramentas de rastreamento abaixo para estabelecer uma linha de base antes de um lançamento mais amplo.

Esta página aborda como [rastrear seus custos](#track-your-costs), [gerenciar custos para sua organização](#manage-costs-for-your-organization) e [reduzir o uso de tokens](#reduce-token-usage).

<h2 id="track-your-costs">
  Rastreie seus custos
</h2>

<h3 id="using-the-/usage-command">
  Usando o comando `/usage`
</h3>

<Note>
  O bloco Session em `/usage` mostra o uso de tokens da API e é destinado a usuários de API. Assinantes do Claude Max e Pro têm uso incluído em sua assinatura, portanto, a figura de custo da sessão não é relevante para fins de faturamento. Os assinantes veem barras de uso do plano, estatísticas de atividade e um detalhamento de uso na mesma tela.
</Note>

O bloco Session no topo de `/usage` mostra estatísticas detalhadas de uso de tokens para sua sessão atual. Claude Code calcula a figura em dólares localmente a partir de contagens de tokens ao preço de tabela, a menos que uma tabela [`modelPricing`](/docs/pt/settings-reference#modelpricing) esteja em vigor. Um administrador define uma nas configurações gerenciadas de sua organização para que a figura use suas taxas contratadas, e enquanto uma tabela está em vigor, a linha `Total cost` carrega a nota `at your organization's configured rates`. A figura é uma estimativa, portanto, para faturamento autorizado, consulte a página de Uso no [Claude Console](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 20s
Total duration (wall): 6h 33m 10s
Total code changes:    0 lines added, 0 lines removed
Usage by model:
   claude-sonnet-4-6:  1.2k input, 5.3k output, 940.0k cache read, 50.0k cache write ($0.55)
```

Esses totais são redefinidos quando `/clear` inicia uma nova sessão, portanto, o custo total da próxima sessão começa em \$0. Antes da v2.1.211, eles continuavam acumulando em `/clear` durante a vida útil do processo Claude Code.

Para uma resposta da API Claude faturada na [taxa de residência de dados](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing) de 1,1×, Claude Code multiplica o preço de tabela dos tokens dessa resposta por 1,1 na figura de custo da sessão. O mesmo total aparece na [linha de status do campo de custo](/docs/pt/statusline#cost-and-duration-tracking), e a figura multiplicada também conta para [`--max-budget-usd`](/docs/pt/cli-reference#cli-flags). Antes da v2.1.239, Claude Code não aplicava o 1,1× a essas respostas, portanto, a figura de custo da sessão era menor que a fatura.

<h4 id="prompt-cache-statistics">
  Estatísticas de cache de prompt
</h4>

Após a primeira resposta da API da conversa principal, Claude Code também adiciona uma linha `Prompt cache (main)` ao bloco Session, resumindo o uso de [cache de prompt](/docs/pt/prompt-caching) da sessão: a contagem de solicitações, a proporção de tokens de entrada servidos do cache, falhas de cache e se o cache está aquecido agora. Requer Claude Code v2.1.251 ou posterior.

```text theme={null}
Prompt cache (main):   14 requests · 91% of input tokens from cache · 2 misses (last 6m 10s ago, 310.2k tokens re-cached) · 1 expected rebuild (compaction or tool-result clearing) · warm (1h TTL, last activity 40s ago)
```

As falhas, reconstruções esperadas e partes aquecidas ou frias da linha significam o seguinte:

* **Misses**: solicitações que reprocessaram conteúdo que o cache já continha, com a hora da última falha e quantos tokens essas solicitações escreveram de volta no cache. Claude Code conta uma solicitação como uma falha quando a solicitação reprocessou mais de 5% e pelo menos 2.000 tokens do que poderia ter lido do cache. [Ações que invalidam o cache](/docs/pt/prompt-caching#actions-that-invalidate-the-cache) lista as causas usuais. Quando Claude Code pode identificar uma causa provável para a última falha, a linha a nomeia também, por exemplo `likely cause: tool definitions changed`. O texto de causa provável requer Claude Code v2.1.260 ou posterior.
* **Expected rebuilds**: quando Claude Code reescreveu a conversa, por [compactação](/docs/pt/prompt-caching#compacting-the-conversation) ou limpando resultados de ferramentas antigas do contexto, ele conta o mesmo tipo de falha como uma reconstrução esperada. Esta parte aparece apenas após pelo menos uma reconstrução esperada ter acontecido.
* **Warm or cold**: se o prefixo em cache ainda está dentro de seu [tempo de vida do cache](/docs/pt/prompt-caching#cache-lifetime), com o TTL em vigor. Quando o cache está frio, a linha mostra há quanto tempo a sessão está ociosa. Quando nenhuma resposta relatou tokens de cache, a linha termina com `no prompt caching reported by the API`.

As contagens vêm dos campos de token de cache nas respostas da API, portanto, a linha funciona em todos os provedores e gateways. Ela cobre apenas a conversa principal, não subagentes. `/clear` a redefine junto com o resto do bloco Session.

Scripts de linha de status podem ler os mesmos números do [objeto `prompt_cache`](/docs/pt/statusline#prompt-cache-fields).

<h4 id="plan-usage-breakdown">
  Detalhamento de uso do plano
</h4>

Em um plano Pro, Max, Team ou Enterprise, `/usage` também mostra um detalhamento do que conta contra seus limites de plano:

* **Attribution**: uso recente atribuído a skills, subagents, plugins e servidores MCP individuais, cada um mostrado como uma porcentagem do total. A participação de um servidor MCP conta apenas as solicitações que consumiram um de seus resultados de ferramenta. Antes da v2.1.222, após uma chamada para um servidor MCP, Claude Code atribuía cada solicitação subsequente a esse servidor, superestimando sua participação.
* **Behavior flags**: comportamentos como contexto longo ou falhas de cache, sinalizados quando um representa 10% ou mais do uso recente.
* **Loops**: uma linha para cada uma das tarefas [`/loop` ou outras tarefas agendadas](/docs/pt/scheduled-tasks) mais pesadas que foram executadas recentemente, ordenadas por tokens totais, com uma contagem do resto. Claude Code relata com que frequência cada tarefa é acionada, quantas vezes foi executada, seus tokens totais e por execução, e quando foi executada pela última vez. Claude Code identifica uma linha pela solicitação da tarefa, portanto, um loop que você interrompe e recria permanece uma linha. Requer Claude Code v2.1.242 ou posterior.

Pressione `d` ou `w` para alternar entre as últimas 24 horas e os últimos 7 dias. As figuras são aproximadas e calculadas a partir do histórico de sessão local nesta máquina, portanto, o uso de outros dispositivos ou claude.ai não está incluído.

Na [extensão do VS Code](/docs/pt/vs-code#check-account-and-usage), as participações de atribuição e sinalizadores de comportamento aparecem no diálogo Account & usage com um alternador Day e Week, sem as linhas de Loops.

<h4 id="check-your-usage-credits-spend">
  Verifique seu gasto em créditos de uso
</h4>

`/usage` também mostra uma linha de créditos de uso enquanto [créditos de uso](#add-usage-credits-to-your-subscription) estão ativados. O que a linha mostra depende do seu plano:

* **Pro e Max**: seu gasto para o mês atual, medido contra seu limite de gasto mensal quando você definiu um. Quando você não definiu um limite, a linha mostra `Unlimited` e nenhuma figura de gasto.
* **Team e Enterprise**: seu próprio gasto para o mês atual, medido contra qualquer [limite que sua organização definiu](#claude-for-teams-and-enterprise) que se aplique a você. Um limite que cobre toda a organização não aparece na linha. Quando você não tem limite próprio, a linha mostra seu gasto sem limite ao lado. Enquanto créditos de uso estão desativados para você, `/usage` não mostra uma linha de créditos de uso.

Quando você tem um limite de gasto, a linha aparece assim que créditos de uso estão ativados e mostra 0% até você gastar créditos de uso pela primeira vez. Antes da v2.1.236, `/usage` mostrava a linha apenas em planos Pro e Max, e uma linha com um limite de gasto permanecia oculta até você ter gasto algo.

<h4 id="when-the-usage-request-fails">
  Quando a solicitação de uso falha
</h4>

Quando a solicitação de seus limites de plano falha, na maioria das vezes porque o endpoint de uso está com limite de taxa, `/usage` mostra as últimas barras de uso que carregou nesta máquina nos últimos 60 minutos, junto com uma nota `Showing last-known usage` indicando há quanto tempo esses dados foram obtidos. Pressione `r` para tentar novamente; uma tentativa bem-sucedida substitui as últimas barras conhecidas por dados atualizados. Sem um snapshot dos últimos 60 minutos, `/usage` relata que o endpoint de uso está com limite de taxa e oferece o mesmo atalho de tentativa. Antes da v2.1.208, uma solicitação com limite de taxa em uma sessão que ainda não havia carregado uso sempre mostrava o erro sem barras.

<h3 id="analyze-your-usage-patterns">
  Analise seus padrões de uso
</h3>

Execute [`/insights`](/docs/pt/commands#all-commands) para um relatório sobre como você trabalha em vez de quantos tokens você usou. Ele analisa suas sessões recentes nesta máquina e escreve um relatório HTML cobrindo no que você trabalha, pontos de fricção como solicitações mal compreendidas ou código com bugs, e sugestões para usar Claude Code mais efetivamente. Uma única execução analisa até 200 sessões que não viu antes e pula as muito curtas. Quando sessões são deixadas de fora, o cabeçalho do relatório mostra a contagem analisada com o total entre parênteses, por exemplo `200 sessions (412 total)`.

Claude Code escreve o relatório mais recente em `~/.claude/usage-data/report.html` e salva uma cópia com timestamp de cada execução no mesmo diretório, portanto, relatórios anteriores não são sobrescritos. Claude Code exclui relatórios no mesmo cronograma que o resto de seus dados de sessão: na inicialização, ele remove arquivos mais antigos que [`cleanupPeriodDays`](/docs/pt/claude-directory#cleaned-up-automatically), 30 dias por padrão.

Você pode executar `/insights` em qualquer plano e com qualquer provedor. A análise é executada através do mesmo provedor e conta que suas sessões regulares, e os tokens contam contra seu plano ou uso de API. Sessões de outros dispositivos e claude.ai não estão incluídas.

<h3 id="add-usage-credits-to-your-subscription">
  Adicione créditos de uso à sua assinatura
</h3>

[Créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) permitem que você continue trabalhando além do limite de uso do seu plano. Para gerenciá-los, execute `/usage-credits` após fazer login com sua assinatura claude.ai através de `/login`; o comando não está disponível com autenticação de chave de API. Em organizações Enterprise de autoatendimento, testes Enterprise e organizações Enterprise faturadas através do AWS Marketplace, o comando requer Claude Code v2.1.248 ou posterior; versões anteriores o rejeitam com [`Unknown command: /usage-credits`](/docs/pt/errors#unknown-command). O que ele abre depende de sua função:

| Sua função                                             | O que `/usage-credits` faz                                                                                                                                                                                                                           |
| :----------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Assinante Pro ou Max                                   | Abre [**Settings > Usage**](https://claude.ai/settings/usage) em claude.ai no navegador. Em sua seção **Usage credits** você pode ativar ou desativar créditos de uso e verificar seu saldo de crédito, gasto deste mês e seu limite de gasto mensal |
| Membro de Team ou Enterprise com acesso de faturamento | Abre as configurações de uso de sua organização, [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), no navegador                                                                                                                  |
| Membro de Team ou Enterprise sem acesso de faturamento | Pede que você confirme, então envia uma solicitação aos administradores de sua organização. Antes da v2.1.211, Claude Code enviava a solicitação sem uma etapa de confirmação                                                                        |

Para membros de Team e Enterprise sem acesso de faturamento, a confirmação aparece apenas em sessões interativas: em modo não interativo com a flag `-p` e de [Remote Control](/docs/pt/remote-control), o comando não envia solicitação e diz que você execute em uma sessão interativa.

Se você executar `/usage-credits` novamente enquanto sua solicitação anterior está aguardando um administrador, Claude Code diz que uma solicitação já foi enviada em vez de enviar uma duplicata. Após um administrador descartar sua solicitação, executar o comando novamente envia uma nova. Antes da v2.1.222, uma solicitação descartada também bloqueava novas solicitações.

Em planos Pro e Max, quando você atinge seu limite de gasto com créditos de uso ainda disponíveis, Claude Code o solicita a aumentar ou remover o limite sem sair da CLI. Se o servidor rejeitar a mudança, consulte [Could not update your spend limit](/docs/pt/errors#could-not-update-your-spend-limit).

<h2 id="manage-costs-for-your-organization">
  Gerenciar custos para sua organização
</h2>

Quais controles você tem depende de como sua organização acessa Claude Code: um plano Claude for Teams ou Enterprise, o Claude Console, ou um provedor de nuvem. Nos planos Teams e Enterprise, o uso é extraído da cota de cada membro. No Console e em provedores de nuvem, o uso é faturado por token para sua organização. Se sua organização mistura métodos de login, cada desenvolvedor é medido de acordo com aquele com o qual se autenticou.

A tabela mapeia cada configuração para onde você vê gastos, onde você os limita e como você extrai números por usuário. Em um plano individual Pro ou Max, você não tem organização para gerenciar, então rastreie seu próprio gasto de créditos de uso, incluindo [modo rápido](/docs/pt/fast-mode#see-where-fast-mode-spend-appears), em [Adicionar créditos de uso à sua assinatura](#add-usage-credits-to-your-subscription).

| Sua configuração                                                                                | Ver gastos                                                                                                                                           | Limitar gastos                                       | Relatório por usuário                                                                                                                                                                                                               |
| :---------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams ou Enterprise](#claude-for-teams-and-enterprise)                              | [Relatório de gastos em análises da organização](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Limites de gastos nas configurações de administrador | [CSV do relatório de gastos](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) no Enterprise |
| [Claude Console (API)](#claude-console)                                                         | [Página de uso do Console](https://platform.claude.com/usage)                                                                                        | Limites de gastos do workspace                       | [Dashboard do Console](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                       |
| [Amazon Bedrock, Plataforma de Agentes do Google Cloud, ou Microsoft Foundry](#cloud-providers) | Seu console de faturamento da nuvem                                                                                                                  | Controles de orçamento da sua nuvem                  | [OpenTelemetry](/docs/pt/monitoring-usage) ou um [gateway LLM](/docs/pt/llm-gateway)                                                                                                                                                          |

[Exportação OpenTelemetry](/docs/pt/monitoring-usage) funciona em todas as configurações e é a única opção que transmite métricas de token e custo por usuário para sua própria pilha de observabilidade em tempo quase real.

<h3 id="report-spend-at-your-contracted-rates">
  Relatar gastos em suas taxas contratadas
</h3>

Por padrão, Claude Code calcula cada valor de custo que mostra aos desenvolvedores ao preço de tabela, então se sua organização paga taxas contratadas, os valores em `/usage`, a linha de status e OpenTelemetry não correspondem à sua fatura. Para fazê-los corresponder, defina a configuração gerenciada [`modelPricing`](/docs/pt/settings-reference#modelpricing) para suas taxas. A configuração muda o que Claude Code relata, não o que Anthropic cobra. Requer Claude Code v2.1.242 ou posterior.

<Steps>
  <Step title="Pegue as taxas do seu contrato">
    Digite as taxas por milhão de tokens do seu contrato. Claude Code não as busca do Claude Console, então atualize a configuração quando o contrato mudar.
  </Step>

  <Step title="Escreva a configuração">
    Defina `multiplier` abaixo de 1 para um desconto fixo ou acima de 1 para uma margem, liste as quatro taxas por token de cada modelo em `overrides`, ou faça ambos. Uma margem requer Claude Code v2.1.271 ou posterior. A [entrada `modelPricing`](/docs/pt/settings-reference#modelpricing) tem a forma e um exemplo pronto para colar.
  </Step>

  <Step title="Implante através de configurações gerenciadas">
    Entregue como [configurações gerenciadas](/docs/pt/managed-settings): configurações gerenciadas por servidor, uma política MDM, `managed-settings.json`, ou um [auxiliar de política](/docs/pt/managed-settings#compute-the-policy-with-a-helper-program). Claude Code ignora a chave em configurações de usuário, projeto e local e em `--settings`.
  </Step>
</Steps>

Para confirmar que as taxas estão em vigor, execute `/usage` em uma sessão que [recebeu as configurações gerenciadas](/docs/pt/managed-settings#read-the-source-in-%2Fstatus): a linha `Total cost` do bloco Session carrega a nota `at your organization's configured rates`. Os valores ainda são estimativas, não uma fatura. Os preços por milhão de tokens no seletor `/model` permanecem ao preço de tabela.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams e Enterprise
</h3>

Nos planos Claude for Teams e Enterprise, o uso de Claude Code de cada membro é extraído de uma cota por assento que é redefinida em uma janela de cinco horas contínuas e uma janela semanal. A cota é compartilhada com Claude chat e Cowork, e seu tamanho depende do [nível de assento](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) (Standard ou Premium). Seus controles ficam no console de administrador claude.ai, não no Claude Console.

* **Ver gastos**: o [relatório de gastos em análises da organização](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) mostra gastos estimados por usuário e por modelo, com exportação CSV, atualizado diariamente. O relatório cobre gastos de créditos de uso e aparece uma vez que os créditos de uso são ativados. O uso dentro da cota por assento não é medido em dólares.
* **Ver adoção**: o [dashboard de análises](https://claude.ai/analytics/claude-code) mostra usuários ativos diários, sessões e métricas de contribuição, com exportação CSV de dados de contribuição. Veja [rastrear uso da equipe com análises](/docs/pt/analytics).
* **Limitar gastos**: a cota por assento é o teto padrão. Para permitir que membros continuem além disso, ative [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) e defina limites de gastos no nível da organização, grupo ou membro individual.
* **Extrair números por usuário**: no plano Enterprise, a [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) retorna relatórios de uso e custo por usuário em todas as superfícies Claude, incluindo Claude Code. Um Proprietário Primário cria uma chave com o escopo `read:analytics` em [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). No plano Teams, exporte o [CSV do relatório de gastos](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), que lista uso de tokens e gastos estimados por usuário e por modelo.

O [guia de consumo Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) é a referência de planejamento para administradores. Ele explica como o consumo difere entre Claude chat, Claude Code e Cowork, e fornece pontos de partida em dólares por usuário para orçamento. Orçamente mais para um assento de codificação do que um assento de chat: cada turno de Claude Code carrega conteúdo de arquivo, chamadas de ferramenta e raciocínio em múltiplas etapas, então uma sessão de depuração pode consumir mais do que um dia de chat.

<h3 id="claude-console">
  Claude Console
</h3>

Organizações de API gerenciam gastos de Claude Code através de [workspaces](https://platform.claude.com/docs/en/build-with-claude/workspaces). Você pode [definir limites de gastos do workspace](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) no gasto total de Claude Code e [visualizar relatórios de custo e uso](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) no Console.

<Note>
  Quando você autentica pela primeira vez o Claude Code com sua conta do Claude Console, um workspace chamado "Claude Code" é criado automaticamente para você. Este workspace fornece rastreamento e gerenciamento centralizado de custos para todo o uso do Claude Code em sua organização. Você não pode criar chaves de API para este workspace; é exclusivamente para autenticação e uso do Claude Code.

  Para organizações com limites de taxa personalizados, o tráfego do Claude Code neste workspace conta para os limites de taxa geral da API da sua organização. Você pode definir um [limite de taxa do workspace](https://platform.claude.com/docs/en/api/rate-limits#setting-lower-limits-for-workspaces) na página Limits deste workspace no Claude Console para limitar a cota do Claude Code e proteger outras cargas de trabalho de produção.
</Note>

Para relatório por usuário, o [dashboard do Console](https://platform.claude.com/claude-code) mostra gastos e linhas aceitas por membro, e a [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) retorna as mesmas métricas diárias por usuário programaticamente com uma [chave de API de Administrador](https://platform.claude.com/settings/admin-keys). Veja [análises para clientes de API](/docs/pt/analytics#access-analytics-for-api-customers).

<h4 id="rate-limit-recommendations">
  Recomendações de limite de taxa
</h4>

Ao configurar Claude Code para equipes, considere estas recomendações de Token Por Minuto (TPM) e Requisição Por Minuto (RPM) por usuário com base no tamanho da sua organização:

| Tamanho da equipe | TPM por usuário | RPM por usuário |
| ----------------- | --------------- | --------------- |
| 1-5 usuários      | 200k-300k       | 5-7             |
| 5-20 usuários     | 100k-150k       | 2.5-3.5         |
| 20-50 usuários    | 50k-75k         | 1.25-1.75       |
| 50-100 usuários   | 25k-35k         | 0.62-0.87       |
| 100-500 usuários  | 15k-20k         | 0.37-0.47       |
| 500+ usuários     | 10k-15k         | 0.25-0.35       |

Por exemplo, se você tiver 200 usuários, você pode solicitar 20k TPM para cada usuário, ou 4 milhões de TPM total (200\*20.000 = 4 milhões).

O TPM por usuário diminui conforme o tamanho da equipe cresce porque menos usuários tendem a usar Claude Code simultaneamente em organizações maiores. Esses limites de taxa se aplicam no nível da organização, não por usuário individual, o que significa que usuários individuais podem consumir temporariamente mais do que sua cota calculada quando outros não estão usando ativamente o serviço.

<Note>
  Se você antecipar cenários com uso concorrente incomumente alto (como sessões de treinamento ao vivo com grandes grupos), você pode precisar de alocações de TPM mais altas por usuário.
</Note>

<h3 id="cloud-providers">
  Provedores de nuvem
</h3>

No Amazon Bedrock, na Plataforma de Agentes do Google Cloud e no Microsoft Foundry, Claude Code é faturado por token para sua conta de nuvem, e os controles de gastos ficam no console de faturamento do seu provedor de nuvem. Claude Code não envia métricas de sua nuvem de volta para Anthropic, então os [dashboards de análises](/docs/pt/analytics) e a Claude Code Analytics API não cobrem este uso.

Para atribuição de custo por usuário, você tem três opções:

* **OpenTelemetry**: [exporte métricas](/docs/pt/monitoring-usage) da máquina de cada desenvolvedor para sua própria pilha de observabilidade. Isso fornece contagens de tokens por usuário, custos e atividade de ferramenta independentemente do provedor.
* **Um gateway de aplicativos Claude**: um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado fornece atribuição de uso por usuário, métricas OTLP com contagens de tokens e [limites de gastos por usuário](/docs/pt/claude-apps-gateway-spend-limits) nesses provedores.
* **Um gateway LLM**: rotear todo o tráfego de Claude Code através de um proxy que rastreia gastos por chave. Vários grandes empresas relataram usar [LiteLLM](/docs/pt/llm-gateway), uma ferramenta de código aberto que [rastreia gastos por chave](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend). Este projeto não é afiliado à Anthropic e não foi auditado para segurança.

<h3 id="when-a-developer-asks-about-a-limit">
  Quando um desenvolvedor pergunta sobre um limite
</h3>

Desenvolvedores geralmente trazem perguntas sobre limites para seu administrador, então é útil saber qual teto eles atingiram. Essas situações significam coisas diferentes:

* **"Você atingiu seu limite de sessão" ou "Você atingiu seu limite semanal"**: uma janela de uso baseada em assento em um plano de assinatura, compartilhada em todos os modelos, então o desenvolvedor não pode restaurar o acesso mudando de modelos com `/model`. A mensagem mostra quando a janela é redefinida. Após a mensagem específica do modelo "Você atingiu seu limite de Opus" ou "Você atingiu seu limite de Sonnet", mudar para um modelo fora dessa família com `/model` mantém o desenvolvedor trabalhando. Veja [erros de limite de uso](/docs/pt/errors#youve-hit-your-session-limit). O que o desenvolvedor pode fazer enquanto isso:
  * Execute `/usage-credits` para solicitar uso além da cota, se você tiver [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) ativados.
  * No Claude Code v2.1.234 ou posterior, [aguarde e continue a tarefa interrompida automaticamente após o reset](/docs/pt/interactive-mode#wait-for-a-usage-limit-to-reset); essa seção lista quando Claude Code inicia a espera por conta própria e quando o desenvolvedor a escolhe em `/rate-limit-options`. Para controlar sua frota se Claude Code inicia essa espera por conta própria, defina [`autoContinueAtUsageLimit`](/docs/pt/settings-reference#autocontinueatusagelimit) em [configurações gerenciadas](/docs/pt/settings#settings-precedence).
* **"Você atingiu seu limite de gastos individual", "limite de gastos mensal da organização", ou "orçamento compartilhado da equipe"**: a solicitação do desenvolvedor seria faturada para créditos de uso, e esses créditos atingiram um limite de gastos que você definiu. Para permitir que o desenvolvedor continue, vá para [**Configurações de Administrador > Uso**](https://claude.ai/admin-settings/usage) e aumente o limite que a mensagem nomeia. Quando a mensagem também nomeia um tempo de reset do plano, o desenvolvedor pode esperar até então. Veja [a referência de erro](/docs/pt/errors#youve-hit-your-monthly-spend-limit) para cada variante.
* **Uma mensagem de limite de gastos de um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway)**: o desenvolvedor passou um limite de gastos que você definiu em seu gateway auto-hospedado, e o gateway bloqueia suas solicitações até o período ser redefinido ou você aumentar o limite. Veja [limites de gastos do gateway](/docs/pt/claude-apps-gateway-spend-limits) para limites, cronogramas de reset e a mensagem que o desenvolvedor vê.
* **Um aviso de contexto ou auto-compact**: não é um limite de uso. A conversa cresceu perto da [janela de auto-compact](/docs/pt/model-config#set-the-auto-compact-window) da sessão, o limite onde Claude Code resume o histórico mais antigo para liberar espaço. Aponte o desenvolvedor para [reduzir uso de tokens](#reduce-token-usage).
* **Gastos inesperadamente altos em um plano de API ou provedor de nuvem**: geralmente rastreia de volta para sessões longas que nunca foram limpas ou para Opus deixado como o modelo padrão. Os hábitos de maior impacto para compartilhar são limpar entre tarefas não relacionadas e corresponder o modelo ao trabalho, ambos cobertos em [reduzir uso de tokens](#reduce-token-usage).

<h3 id="agent-team-token-costs">
  Custos de tokens de equipes de agentes
</h3>

[Equipes de agentes](/docs/pt/agent-teams) geram múltiplas instâncias do Claude Code, cada uma com sua própria janela de contexto. O uso de tokens escala com o número de colegas de equipe ativos e quanto tempo cada um executa.

Para manter os custos das equipes de agentes gerenciáveis:

* Use Sonnet para colegas de equipe. Ele equilibra capacidade e custo para tarefas de coordenação.
* Mantenha equipes pequenas. Cada colega de equipe executa sua própria janela de contexto, portanto, o uso de tokens é aproximadamente proporcional ao tamanho da equipe.
* Mantenha prompts de geração focados. Colegas de equipe carregam CLAUDE.md, servidores MCP e skills automaticamente, mas tudo no prompt de geração adiciona ao seu contexto desde o início.
* Desligue colegas de equipe quando o trabalho estiver concluído. Cada colega de equipe ativo continua consumindo tokens até sair ou a sessão terminar.
* Equipes de agentes são desabilitadas por padrão. Defina `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` em seu [settings.json](/docs/pt/settings) ou ambiente para habilitá-las. Veja [habilitar equipes de agentes](/docs/pt/agent-teams#enable-agent-teams).

<h2 id="reduce-token-usage">
  Reduza o uso de tokens
</h2>

Os custos de tokens escalam com o tamanho do contexto: quanto mais contexto Claude processa, mais tokens você usa. Claude Code otimiza automaticamente os custos através do [prompt caching](/docs/pt/prompt-caching), que reduz custos para conteúdo repetido como prompts do sistema, e auto-compaction, que resume o histórico de conversa ao se aproximar dos limites de contexto.

As seguintes estratégias ajudam você a manter o contexto pequeno e reduzir custos por mensagem.

<h3 id="manage-context-proactively">
  Gerencie o contexto proativamente
</h3>

Use `/usage` para verificar seu uso atual de tokens, ou [configure sua linha de status](/docs/pt/statusline#context-window-usage) para exibi-la continuamente.

* **Limpe entre tarefas**: Use `/clear` para começar do zero ao mudar para trabalho não relacionado. Contexto obsoleto desperdiça tokens em cada mensagem subsequente. Use `/rename` antes de limpar para que você possa encontrar facilmente a sessão depois, então `/resume` para retornar a ela.
* **Adicione instruções de compactação personalizadas**: `/compact Focus on code samples and API usage` diz a Claude o que preservar durante a sumarização. Em uma sessão nova, `/compact` imprime `Not enough messages to compact.` porque não há histórico de conversa para sumarizar ainda.

Você também pode personalizar o comportamento de compactação em seu arquivo CLAUDE.md na raiz do seu projeto:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Escolha o modelo certo
</h3>

Sonnet lida bem com a maioria das tarefas de codificação e custa menos que Opus. Reserve Opus para decisões arquitetônicas complexas ou raciocínio em múltiplas etapas. Use `/model` para alternar modelos no meio da sessão, ou defina um padrão em `/config`. Uma mudança para Opus também se aplica aos [subagentes que herdam o modelo da sua sessão](/docs/pt/model-config#setting-your-model). Para tarefas simples de subagente, especifique `model: haiku` em sua [configuração de subagente](/docs/pt/sub-agents#choose-a-model).

<h3 id="reduce-mcp-server-overhead">
  Reduza a sobrecarga do servidor MCP
</h3>

As definições de ferramentas MCP são [adiadas por padrão](/docs/pt/mcp#scale-with-mcp-tool-search), portanto apenas nomes de ferramentas e instruções do servidor entram no contexto até Claude usar uma ferramenta específica. Execute `/context` para ver o que está consumindo espaço.

* **Prefira ferramentas CLI quando disponíveis**: Ferramentas como `gh`, `aws`, `gcloud` e `sentry-cli` são ainda mais eficientes em contexto do que servidores MCP porque não adicionam nenhuma listagem por ferramenta. Claude pode executar comandos CLI diretamente.
* **Desabilite servidores não utilizados**: Execute `/mcp` para ver servidores configurados e desabilite qualquer um que você não esteja usando ativamente.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Instale plugins de inteligência de código para linguagens tipadas
</h3>

[Plugins de inteligência de código](/docs/pt/plugins/code-intelligence) dão a Claude navegação de símbolo precisa em vez de busca baseada em texto, reduzindo leituras de arquivo desnecessárias ao explorar código desconhecido. Uma única chamada "ir para definição" substitui o que poderia ser um grep seguido de leitura de múltiplos arquivos candidatos. Servidores de linguagem instalados também relatam erros de tipo automaticamente após edições, portanto Claude detecta erros sem executar um compilador.

<h3 id="offload-processing-to-hooks-and-skills">
  Descarregue o processamento para hooks e skills
</h3>

[Hooks](/docs/pt/hooks) personalizados podem pré-processar dados antes de Claude vê-los. Em vez de Claude ler um arquivo de log de 10.000 linhas para encontrar erros, um hook pode fazer grep para `ERROR` e retornar apenas linhas correspondentes, reduzindo contexto de dezenas de milhares de tokens para centenas.

Uma [skill](/docs/pt/skills) pode dar a Claude conhecimento de domínio para que não tenha que explorar. Por exemplo, uma skill "codebase-overview" poderia descrever a arquitetura do seu projeto, diretórios-chave e convenções de nomenclatura. Quando Claude invoca a skill, obtém este contexto imediatamente em vez de gastar tokens lendo múltiplos arquivos para entender a estrutura.

Por exemplo, este hook PreToolUse filtra a saída de teste para mostrar apenas falhas:

<Tabs>
  <Tab title="settings.json">
    Adicione isto ao seu [settings.json](/docs/pt/settings#where-settings-live) para executar o hook antes de cada comando Bash:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    O hook chama este script. Crie a pasta com `mkdir -p ~/.claude/hooks`, salve o script abaixo como `~/.claude/hooks/filter-test-output.sh` e torne-o executável com `chmod +x ~/.claude/hooks/filter-test-output.sh`. Ele verifica se o comando é um executor de teste e o modifica para mostrar apenas falhas:

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "$input" | jq --arg filtered "$filtered_cmd" \
        '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "allow", updatedInput: (.tool_input + {command: $filtered})}}'
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

Para verificar a configuração, execute `/hooks` e verifique se o hook aparece sob PreToolUse. Você também pode iniciar Claude Code com `claude --debug-file ./claude-debug.txt` e pedir a Claude para executar `npm test`. Quando o hook reescreve o comando, esse arquivo de log contém uma linha `modified tool input keys` listando `command` e os outros campos de entrada Bash.

<h3 id="move-instructions-from-claude-md-to-skills">
  Mova instruções de CLAUDE.md para skills
</h3>

Seu arquivo [CLAUDE.md](/docs/pt/memory) é carregado no contexto no início da sessão. Se contiver instruções detalhadas para fluxos de trabalho específicos (como revisões de PR ou migrações de banco de dados), esses tokens estão presentes mesmo quando você está fazendo trabalho não relacionado. [Skills](/docs/pt/skills) carregam sob demanda apenas quando invocadas, portanto mover instruções especializadas para skills mantém seu contexto base menor. Procure manter CLAUDE.md com menos de 200 linhas incluindo apenas essenciais.

<h3 id="adjust-extended-thinking">
  Ajuste o pensamento estendido
</h3>

O pensamento estendido é habilitado por padrão porque melhora significativamente o desempenho em tarefas complexas de planejamento e raciocínio. Tokens de pensamento são faturados como tokens de saída, e o orçamento padrão pode ser dezenas de milhares de tokens por solicitação dependendo do modelo.

Para tarefas mais simples onde raciocínio profundo não é necessário, você pode reduzir custos baixando o [nível de esforço](/docs/pt/model-config#adjust-effort-level) com `/effort` ou em `/model`, ou desabilitando pensamento em `/config`. Você não pode desativar pensamento em Opus 5.5 ou nos modelos Fable, que sempre usam pensamento estendido.

Em modelos com um [orçamento de pensamento fixo](/docs/pt/model-config#adaptive-reasoning-and-fixed-thinking-budgets), você também pode baixar o orçamento definindo a [variável de ambiente](/docs/pt/env-vars) `MAX_THINKING_TOKENS`, por exemplo `MAX_THINKING_TOKENS=8000`. Modelos de raciocínio adaptativo ignoram orçamentos diferentes de zero, portanto use níveis de esforço lá em vez disso.

<h3 id="delegate-verbose-operations-to-subagents">
  Delegue operações verbosas para subagentes
</h3>

Executar testes, buscar documentação ou processar arquivos de log pode consumir contexto significativo. Delegue estes para [subagentes](/docs/pt/sub-agents#isolate-high-volume-operations) para que a saída verbosa permaneça no contexto do subagente enquanto apenas um resumo retorna à sua conversa principal.

<h3 id="manage-agent-team-costs">
  Gerencie custos de equipes de agentes
</h3>

Equipes de agentes usam aproximadamente 7x mais tokens do que sessões padrão quando colegas de equipe executam em modo de plano, porque cada colega de equipe mantém sua própria janela de contexto e executa como uma instância Claude separada. Mantenha tarefas de equipe pequenas e auto-contidas para limitar o uso de tokens por colega de equipe. Veja [equipes de agentes](/docs/pt/agent-teams) para detalhes.

<h3 id="write-specific-prompts">
  Escreva prompts específicos
</h3>

Solicitações vagas como "melhorar esta base de código" disparam varredura ampla. Solicitações específicas como "adicionar validação de entrada à função de login em auth.ts" deixam Claude trabalhar eficientemente com leituras de arquivo mínimas.

<h3 id="work-efficiently-on-complex-tasks">
  Trabalhe eficientemente em tarefas complexas
</h3>

Para trabalho mais longo ou complexo, esses hábitos ajudam a evitar tokens desperdiçados por seguir o caminho errado:

* **Use modo de plano para tarefas complexas**: Pressione Shift+Tab para entrar em [modo de plano](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) antes da implementação. Claude explora a base de código e propõe uma abordagem para sua aprovação, prevenindo retrabalho caro quando a direção inicial está errada.
* **Corrija o curso cedo**: Se Claude começar a seguir a direção errada, pressione Escape para parar imediatamente. Use `/rewind` ou toque duplo em Escape para restaurar conversa e código para um checkpoint anterior.
* **Dê alvos de verificação**: Inclua casos de teste, cole capturas de tela ou defina saída esperada em seu prompt. Quando Claude pode verificar seu próprio trabalho, detecta problemas antes de você precisar solicitar correções.
* **Teste incrementalmente**: Escreva um arquivo, teste-o, depois continue. Isto detecta problemas cedo quando são baratos de corrigir.

<h2 id="background-token-usage">
  Uso de tokens em segundo plano
</h2>

Claude Code usa tokens para algumas funcionalidades em segundo plano mesmo quando ocioso:

* **Sumarização de conversa**: Trabalhos em segundo plano que resumem conversas anteriores para o recurso `claude --resume`
* **Processamento de comando**: Alguns comandos como `/usage` podem gerar solicitações para verificar status

Esses processos em segundo plano consomem uma pequena quantidade de tokens (tipicamente menos de \$0.04 por sessão) mesmo sem interação ativa.

Quando as sugestões de prompt estão ativadas, Claude Code também envia uma solicitação curta para o modelo que sua sessão está usando após Claude responder, para [sugerir seu próximo prompt](/docs/pt/interactive-mode#prompt-suggestions). Essa solicitação reutiliza o cache de prompt da conversa, portanto é principalmente leituras de cache mais alguns tokens de saída. Claude Code [pula isso quando sua conta está próxima ou no limite de uso](/docs/pt/interactive-mode#when-claude-code-skips-suggestions). Para parar essas solicitações, [desative as sugestões de prompt](/docs/pt/interactive-mode#turn-prompt-suggestions-off).

<h2 id="why-usage-climbs-in-a-long-session">
  Por que o uso aumenta em uma sessão longa
</h2>

Uma sessão que está aberta há horas pode usar muito mais dos limites do seu plano do que sua atividade sugere, geralmente por um destes motivos:

* **Contexto longo**: Claude Code envia sua conversa completa com cada solicitação, e cada vez que Claude usa ferramentas, envia outra solicitação contendo esse lote de resultados de ferramentas. Com [prompt caching](/docs/pt/prompt-caching), Claude Code relê esse histórico na [taxa de token em cache](https://platform.claude.com/docs/en/about-claude/pricing), então uma pergunta de uma linha em uma sessão que está aberta o dia todo ainda consome uso para toda a conversa. Veja [Gerenciar contexto proativamente](#manage-context-proactively) para formas de manter seu contexto pequeno
* **Cache misses**: sua primeira mensagem após uma pausa mais longa que o [tempo de vida do cache](/docs/pt/prompt-caching#cache-lifetime) perde o cache e reprocessa seu contexto completo. O tempo de vida é uma hora em uma assinatura e cai para cinco minutos quando você está usando [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans); em uma chave de API ou provedor de nuvem, é cinco minutos por padrão. Para manter o tempo de vida de uma hora enquanto usa créditos de uso, [escolha o TTL você mesmo](/docs/pt/prompt-caching#choose-the-ttl-yourself). Nos planos Pro e Max, quando você retoma uma sessão grande após uma pausa longa, Claude Code [oferece retomar de um resumo](/docs/pt/sessions#resume-from-a-summary) para que solicitações posteriores não carreguem o histórico completo
* **Tarefas agendadas**: uma [tarefa agendada](/docs/pt/scheduled-tasks) é executada em seu intervalo mesmo enquanto a sessão está inativa, enviando seu contexto completo cada vez
* **Mensagens entre sessões**: Claude Code entrega uma [mensagem de outra de suas sessões](/docs/pt/cross-session-messaging) como um novo turno quando esta sessão fica inativa, enviando seu contexto completo cada vez. Para manter mensagens de entrada em vez de entregá-las, defina [`crossSessionInbound`](/docs/pt/settings-reference#crosssessioninbound) como `hold`
* **Verificações de objetivo**: enquanto o trabalho em segundo plano mantém um [objetivo](/docs/pt/goal) ativo aguardando, Claude Code [pede ao Claude para verificar esse trabalho](/docs/pt/goal#background-work-defers-evaluation) mesmo quando a sessão fica inativa, iniciando um novo turno que envia seu contexto completo. Claude Code inicia no máximo três verificações inativas por objetivo entre seus prompts. Antes da v2.1.246, as verificações inativas eram ilimitadas. Para desativar as verificações, defina [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/pt/env-vars) como `0`. As verificações inativas exigem Claude Code v2.1.236 ou posterior
* **Colegas de equipe agentes**: cada [colega de equipe](#agent-team-token-costs) ativo continua consumindo tokens até sair
* **Compactação**: `/compact` lê a conversa que resume, então [compactar um contexto grande](/docs/pt/prompt-caching#compacting-the-conversation) é em si uma solicitação grande. Quando você quer um novo começo em vez de continuidade, `/clear` não custa nada

Em um plano Pro, Max, Team ou Enterprise, a divisão `/usage` sinaliza comportamentos que representam 10% ou mais de seu uso recente, como contexto longo ou cache misses, cada um com uma dica para reduzi-lo.

<h2 id="understanding-changes-in-claude-code-behavior">
  Compreendendo mudanças no comportamento do Claude Code
</h2>

Claude Code recebe atualizações regularmente que podem alterar como os recursos funcionam, incluindo relatórios de custo. Execute `claude --version` para verificar sua versão atual.

Para dúvidas sobre faturamento da sua conta específica, entre em contato com o suporte da Anthropic através do mensageiro integrado no produto:

* **Planos de assinatura** (Pro, Max, Team, Enterprise): faça login em [claude.ai](https://claude.ai), clique em suas iniciais no canto inferior esquerdo e selecione **Obter ajuda**
* **Faturamento do Console (API)**: faça login em [platform.claude.com](https://platform.claude.com), clique em suas iniciais e selecione **Obter ajuda**

Consulte [Como obter suporte](https://support.claude.com/en/articles/9015913-how-to-get-support) para o fluxo completo, incluindo quem pode alcançar um agente humano em cada plano.
