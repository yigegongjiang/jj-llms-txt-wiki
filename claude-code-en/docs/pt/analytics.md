> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rastrear o uso da equipe com análise

> Visualize as métricas de uso do Claude Code, rastreie a adoção e meça a velocidade de engenharia no painel de análise.

Claude Code fornece painéis de análise para ajudar as organizações a entender os padrões de uso dos desenvolvedores, rastrear métricas de contribuição e medir como Claude Code impacta a velocidade de engenharia. Acesse o painel para seu plano:

| Plano                         | URL do Painel                                                              | Inclui                                                                                            | Saiba mais                                            |
| ----------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| Claude for Teams / Enterprise | [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code) | Métricas de uso, métricas de contribuição com integração GitHub, leaderboard, exportação de dados | [Detalhes](#access-analytics-for-team-and-enterprise) |
| API (Claude Console)          | [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | Métricas de uso, rastreamento de gastos, insights da equipe                                       | [Detalhes](#access-analytics-for-api-customers)       |

<h2 id="access-analytics-for-team-and-enterprise">
  Acessar análise para Team e Enterprise
</h2>

Navegue até [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code). Administradores e Proprietários podem visualizar o painel.

O painel Team e Enterprise inclui:

* **Métricas de uso**: linhas de código aceitas, taxa de aceitação de sugestões, usuários ativos diários e sessões
* **Métricas de contribuição**: PRs e linhas de código enviadas com assistência do Claude Code, com [integração GitHub](#enable-contribution-metrics)
* **Leaderboard**: principais contribuidores classificados por uso do Claude Code
* **Exportação de dados**: baixe dados de contribuição como CSV para relatórios personalizados

Para contagens de tokens por usuário e estimativas de custo, configure [exportação OpenTelemetry](/docs/pt/monitoring-usage), ou exporte o [relatório de gastos](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) das configurações de análise de sua organização, que lista o uso de tokens e o gasto estimado em créditos de uso por usuário e por modelo.

<h3 id="enable-contribution-metrics">
  Ativar métricas de contribuição
</h3>

<Note>
  As métricas de contribuição estão em beta público e disponíveis nos planos Claude for Teams e Claude for Enterprise. Essas métricas cobrem apenas usuários dentro de sua organização claude.ai. O uso através da API Claude Console ou integrações de terceiros não está incluído.
</Note>

Os dados de uso e adoção estão disponíveis para todas as contas Claude for Teams e Claude for Enterprise. As métricas de contribuição requerem configuração adicional para conectar sua organização GitHub.

Você precisa da função Proprietário para configurar as definições de análise. Um administrador GitHub deve instalar o aplicativo GitHub.

<Warning>
  As métricas de contribuição não estão disponíveis para organizações com [Zero Data Retention](/docs/pt/zero-data-retention) ativado. O painel de análise mostrará apenas métricas de uso.
</Warning>

<Steps>
  <Step title="Instalar o aplicativo GitHub">
    Um administrador GitHub instala o aplicativo Claude GitHub na conta GitHub de sua organização em [github.com/apps/claude](https://github.com/apps/claude).
  </Step>

  <Step title="Ativar análise do Claude Code">
    Um Proprietário Claude navega até [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) e ativa o recurso de análise do Claude Code.
  </Step>

  <Step title="Ativar análise do GitHub">
    Na mesma página, ative o botão "GitHub analytics".
  </Step>

  <Step title="Autenticar com GitHub">
    Conclua o fluxo de autenticação do GitHub e selecione quais organizações GitHub incluir na análise.
  </Step>
</Steps>

Os dados normalmente aparecem dentro de 24 horas após a ativação, com atualizações diárias. Se nenhum dado aparecer, você pode ver uma destas mensagens:

* **"GitHub app required"**: instale o aplicativo GitHub para visualizar métricas de contribuição
* **"Data processing in progress"**: verifique novamente em alguns dias e confirme se o aplicativo GitHub está instalado se os dados não aparecerem

As métricas de contribuição suportam GitHub Cloud e GitHub Enterprise Server.

<h3 id="review-summary-metrics">
  Revisar métricas de resumo
</h3>

<Note>
  Essas métricas são deliberadamente conservadoras e representam uma subestimativa do impacto real do Claude Code. Apenas linhas e PRs onde há alta confiança no envolvimento do Claude Code são contadas.
</Note>

O painel exibe essas métricas de resumo no topo:

* **PRs with CC**: contagem total de pull requests mesclados que contêm pelo menos uma linha de código escrita com Claude Code
* **Lines of code with CC**: total de linhas de código em todos os PRs mesclados que foram escritos com assistência do Claude Code. Apenas "linhas efetivas" são contadas: linhas com mais de 3 caracteres após normalização, excluindo linhas vazias e linhas com apenas colchetes ou pontuação trivial.
* **PRs with Claude Code (%)**: percentual de todos os PRs mesclados que contêm código assistido por Claude Code
* **Suggestion accept rate**: percentual de vezes que os usuários aceitam as sugestões de edição de código do Claude Code, incluindo o uso das ferramentas Edit, Write e NotebookEdit
* **Lines of code accepted**: total de linhas de código escritas por Claude Code que os usuários aceitaram em suas sessões. Isso exclui sugestões rejeitadas e não rastreia exclusões subsequentes.

<h3 id="explore-the-charts">
  Explorar os gráficos
</h3>

O painel inclui vários gráficos para visualizar tendências ao longo do tempo.

<h4 id="track-adoption">
  Rastrear adoção
</h4>

O gráfico de Adoção mostra tendências de uso diário:

* **users**: usuários ativos diários
* **sessions**: número de sessões ativas do Claude Code por dia

<h4 id="measure-prs-per-user">
  Medir PRs por usuário
</h4>

Este gráfico exibe a atividade do desenvolvedor individual ao longo do tempo:

* **PRs per user**: número total de PRs mesclados por dia dividido por usuários ativos diários
* **users**: usuários ativos diários

Use isso para entender como a produtividade individual muda conforme a adoção do Claude Code aumenta.

<h4 id="view-pull-requests-breakdown">
  Visualizar detalhamento de pull requests
</h4>

O gráfico Pull requests mostra um detalhamento diário de PRs mesclados:

* **PRs with CC**: pull requests contendo código assistido por Claude Code
* **PRs without CC**: pull requests sem código assistido por Claude Code

Alterne para a visualização **Lines of code** para ver o mesmo detalhamento por linhas de código em vez de contagem de PR.

<h4 id="find-top-contributors">
  Encontrar principais contribuidores
</h4>

O Leaderboard mostra os 10 principais usuários classificados por volume de contribuição. Alterne entre:

* **Pull requests**: mostra PRs com Claude Code vs Todos os PRs para cada usuário
* **Lines of code**: mostra linhas com Claude Code vs Todas as linhas para cada usuário

Clique em **Export all users** para baixar dados de contribuição completos para todos os usuários como um arquivo CSV. A exportação inclui todos os usuários, não apenas os 10 principais exibidos.

<h3 id="pr-attribution">
  Atribuição de PR
</h3>

Quando as métricas de contribuição estão ativadas, Claude Code analisa pull requests mesclados para determinar qual código foi escrito com assistência do Claude Code. Isso é feito combinando a atividade da sessão do Claude Code com o código em cada PR.

<h4 id="tagging-criteria">
  Critérios de marcação
</h4>

PRs são marcados como "with Claude Code" se contiverem pelo menos uma linha de código escrita durante uma sessão do Claude Code. O sistema usa correspondência conservadora: apenas código onde há alta confiança no envolvimento do Claude Code é contado como assistido.

<h4 id="attribution-process">
  Processo de atribuição
</h4>

Quando um pull request é mesclado:

1. Linhas adicionadas são extraídas do diff do PR
2. Sessões do Claude Code que editaram arquivos correspondentes dentro de uma janela de tempo são identificadas
3. Linhas de PR são comparadas com a saída do Claude Code usando múltiplas estratégias
4. Métricas são calculadas para linhas assistidas por IA e linhas totais

Antes da comparação, as linhas são normalizadas: espaços em branco são aparados, múltiplos espaços são recolhidos, aspas são padronizadas e o texto é convertido para minúsculas.

Pull requests mesclados contendo linhas assistidas por Claude Code são marcados como `claude-code-assisted` no GitHub.

<h4 id="time-window">
  Janela de tempo
</h4>

Sessões de 21 dias antes a 2 dias após a data de mesclagem do PR são consideradas para correspondência de atribuição.

<h4 id="excluded-files">
  Arquivos excluídos
</h4>

Certos arquivos são automaticamente excluídos da análise porque são gerados automaticamente:

* Arquivos de bloqueio: package-lock.json, yarn.lock, Cargo.lock e similares
* Código gerado: saídas Protobuf, artefatos de compilação, arquivos minificados
* Diretórios de compilação: dist/, build/, node\_modules/, target/
* Fixtures de teste: snapshots, cassettes, dados simulados
* Linhas com mais de 1.000 caracteres, que provavelmente são minificadas ou geradas

<h4 id="attribution-notes">
  Notas de atribuição
</h4>

Tenha em mente esses detalhes adicionais ao interpretar dados de atribuição:

* Código substancialmente reescrito por desenvolvedores, com mais de 20% de diferença, não é atribuído ao Claude Code
* Sessões fora da janela de 21 dias não são consideradas
* O algoritmo não considera o branch de origem ou destino do PR ao executar a atribuição

<h3 id="get-the-most-from-analytics">
  Aproveitar ao máximo a análise
</h3>

Use métricas de contribuição para demonstrar ROI, identificar padrões de adoção e encontrar membros da equipe que podem ajudar outros a começar.

<h4 id="monitor-adoption">
  Monitorar adoção
</h4>

Rastreie o gráfico de Adoção e contagens de usuários para identificar:

* Usuários ativos que podem compartilhar melhores práticas
* Tendências gerais de adoção em sua organização
* Quedas no uso que podem indicar atrito ou problemas

<h4 id="measure-roi">
  Medir ROI
</h4>

As métricas de contribuição ajudam a responder "Esta ferramenta vale o investimento?" com dados de sua própria base de código:

* Rastreie mudanças em PRs por usuário ao longo do tempo conforme a adoção aumenta
* Compare PRs e linhas de código enviadas com vs. sem Claude Code
* Use junto com [métricas DORA](https://dora.dev/), velocidade de sprint ou outros KPIs de engenharia para entender mudanças ao adotar Claude Code

<h4 id="identify-power-users">
  Identificar usuários avançados
</h4>

O Leaderboard ajuda você a encontrar membros da equipe com alta adoção do Claude Code que podem:

* Compartilhar técnicas de prompting e fluxos de trabalho com a equipe
* Fornecer feedback sobre o que está funcionando bem
* Ajudar a integrar novos usuários

<h4 id="access-data-programmatically">
  Acessar dados programaticamente
</h4>

No plano Enterprise, a [Claude Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) retorna relatórios de engajamento, uso e custo por usuário para sua organização em todas as superfícies Claude, incluindo Claude Code. Um Proprietário Primário cria uma chave com o escopo `read:analytics` em [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). A API não está disponível no plano Teams.

Para consultar dados de contribuição através do GitHub, procure por PRs marcados com `claude-code-assisted`.

<h2 id="access-analytics-for-api-customers">
  Acessar análise para clientes de API
</h2>

Clientes de API usando Claude Console podem acessar análise em [platform.claude.com/claude-code](https://platform.claude.com/claude-code). Você precisa da permissão UsageView para acessar o painel, que é concedida aos papéis Developer, Billing, Admin, Owner e Primary Owner. Para extrair as mesmas métricas diárias por usuário programaticamente, use a [Claude Code Analytics API](https://platform.claude.com/docs/pt/build-with-claude/claude-code-analytics-api) com uma chave de API Admin.

<Note>
  As métricas de contribuição com integração GitHub não estão disponíveis para clientes de API. O painel Console mostra apenas métricas de uso e gastos.
</Note>

O painel Console exibe:

* **Lines of code accepted**: total de linhas de código escritas por Claude Code que os usuários aceitaram em suas sessões. Isso exclui sugestões rejeitadas e não rastreia exclusões subsequentes.
* **Suggestion accept rate**: percentual de vezes que os usuários aceitam o uso da ferramenta de edição de código, incluindo as ferramentas Edit, Write e NotebookEdit.
* **Activity**: usuários ativos diários e sessões mostradas em um gráfico.
* **Spend**: custos diários da API em dólares ao lado da contagem de usuários.

<h3 id="view-team-insights">
  Visualizar insights da equipe
</h3>

A tabela de insights da equipe mostra métricas por usuário:

* **Members**: todos os usuários que se autenticaram no Claude Code. Usuários de chave de API são exibidos por identificador de chave, usuários OAuth são exibidos por endereço de email.
* **Spend this month**: custos totais da API por usuário para o mês atual.
* **Lines this month**: total por usuário de linhas de código aceitas para o mês atual.

<Note>
  Os valores de gastos no painel Console são estimativas para fins de análise. Para custos reais, consulte sua página de faturamento.
</Note>

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Monitoramento com OpenTelemetry](/docs/pt/monitoring-usage): exporte métricas e eventos em tempo real para sua pilha de observabilidade
* [Gerenciar custos efetivamente](/docs/pt/costs): defina limites de gastos e otimize o uso de tokens
* [Permissões](/docs/pt/permissions): configure papéis e permissões
