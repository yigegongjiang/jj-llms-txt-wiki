> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatizar trabalho com rotinas

> Coloque Claude Code no piloto automático. Defina rotinas que são executadas em um cronograma, acionadas em chamadas de API ou reagem a eventos do GitHub a partir da infraestrutura em nuvem.

<Note>
  As rotinas estão em visualização de pesquisa. O comportamento, limites e a superfície da API podem mudar.
</Note>

Uma rotina é uma configuração salva do Claude Code: um prompt, um ou mais repositórios e um conjunto de [conectores](/docs/pt/mcp), empacotados uma vez e executados automaticamente. As rotinas são executadas na infraestrutura em nuvem gerenciada pela Anthropic, ou no [ambiente auto-hospedado](/docs/pt/self-hosted-environments) da sua organização quando roteadas para lá, portanto continuam funcionando quando seu laptop está fechado.

Cada rotina pode ter um ou mais acionadores anexados a ela:

* **Agendada**: executada em uma cadência recorrente como horária, noturna ou semanal, ou uma vez em um momento futuro específico
* **API**: acionada sob demanda enviando um POST HTTP para um endpoint por rotina com um token de portador
* **GitHub**: executada automaticamente em resposta a eventos de repositório, como pull requests ou lançamentos

Uma única rotina pode combinar acionadores. Por exemplo, uma rotina de revisão de PR pode ser executada à noite, acionada a partir de um script de implantação e também reagir a cada novo PR.

As rotinas estão disponíveis nos planos Pro, Max, Team e Enterprise. Crie e gerencie-as em [claude.ai/code/routines](https://claude.ai/code/routines), ou a partir da CLI com `/schedule`.

Os proprietários de Team e Enterprise podem desabilitar rotinas para todos os membros com o botão de alternância Rotinas em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Quando desabilitadas, as rotinas existentes param de ser executadas e os membros não podem criar novas.

Esta página aborda a criação de uma rotina, a configuração de cada tipo de acionador, o gerenciamento de execuções e como os limites de uso se aplicam.

<h2 id="example-use-cases">
  Exemplos de casos de uso
</h2>

Cada exemplo emparelha um tipo de acionador com o tipo de trabalho para o qual as rotinas são adequadas: sem supervisão, repetível e vinculado a um resultado claro.

**Manutenção de backlog.** Um acionador de cronograma é executado todas as noites da semana contra seu rastreador de problemas via um conector. A rotina lê os problemas abertos desde a última execução, aplica rótulos, atribui proprietários com base na área de código referenciada e publica um resumo no Slack para que a equipe comece o dia com uma fila organizada.

**Triagem de alertas.** Sua ferramenta de monitoramento chama o endpoint da API da rotina quando um limite de erro é ultrapassado, passando o corpo do alerta como `text`. A rotina investiga o alerta no payload de incêndio, portanto extrai o rastreamento de pilha, correlaciona-o com commits recentes no repositório e abre um pull request de rascunho com uma correção proposta e um link de volta ao alerta. O responsável pela chamada revisa o PR em vez de começar a partir de um terminal em branco.

**Revisão de código personalizada.** Um acionador do GitHub é executado em `pull_request.opened`. A rotina aplica a lista de verificação de revisão da sua equipe, deixa comentários inline para problemas de segurança, desempenho e estilo, e adiciona um comentário de resumo para que os revisores humanos possam se concentrar no design em vez de verificações mecânicas.

**Verificação de implantação.** Seu pipeline de CD chama o endpoint da API da rotina após cada implantação em produção. A rotina executa verificações de fumaça contra a nova compilação, verifica logs de erro para regressões e publica um go ou no-go no canal de lançamento antes que a janela de implantação se feche.

**Desvio de documentação.** Um acionador de cronograma é executado semanalmente. A rotina verifica PRs mesclados desde a última execução, sinaliza documentação que referencia APIs alteradas e abre PRs de atualização contra o repositório de documentação para um editor revisar.

**Porta de biblioteca.** Um acionador do GitHub é executado em `pull_request.closed` filtrado para PRs mesclados em um repositório de SDK. A rotina porta a alteração para um SDK paralelo em outro idioma e abre um PR correspondente, mantendo as duas bibliotecas em sincronia sem um humano reimplementar cada alteração.

<h2 id="create-a-routine">
  Criar uma rotina
</h2>

Crie uma rotina a partir da web em [claude.ai/code/routines](https://claude.ai/code/routines), do aplicativo Desktop ou da CLI. Todas as três superfícies escrevem na mesma conta em nuvem, portanto uma rotina que você cria em uma aparece nas outras imediatamente. No aplicativo Desktop, clique em **Routines** na barra lateral ou no menu **More** da barra lateral, depois em **New routine**, e escolha **Cloud**; escolher **Local** em vez disso cria uma [tarefa agendada do Desktop](/docs/pt/desktop-scheduled-tasks), que é executada em sua máquina em vez de na nuvem.

O formulário de criação configura o prompt da rotina, repositórios, ambiente, conectores e acionadores.

As rotinas são executadas autonomamente como sessões completas de Claude Code em nuvem: não há seletor de modo de permissão, e a sessão executa comandos shell, usa [skills](/docs/pt/skills) confirmadas no repositório clonado e chama qualquer conector que você incluir, tudo sem parar para aprovação, exceto por algumas ações de [artifact](/docs/pt/artifacts).

O que uma rotina pode alcançar é determinado pelos repositórios que você seleciona, o [acesso à rede do ambiente](/docs/pt/cloud-environments) e variáveis, e os conectores que você inclui. Escopo cada um desses para o que a rotina realmente precisa.

Quando o cronograma da rotina ou **Run now** inicia uma execução, Claude republica um artifact existente sem perguntar apenas quando todos estes se aplicam:

* Você pode editar o artifact e ele pertence à sua própria organização
* O artifact não é compartilhado publicamente e não é compartilhado com pessoas específicas ou sua organização com a versão mais recente escolhida como a versão que os visualizadores veem
* A publicação carrega apenas a página, sem arquivos de suporte ou qualquer outra coisa adicionada, e não força sobre uma versão mais recente
* A página não contém uma concessão que vai além da página, como [chamadas de conector](/docs/pt/artifacts#pull-live-data-with-mcp-connectors)

Em todos os outros casos, incluindo publicar um novo artifact, Claude pergunta primeiro. Quando o trabalho de uma rotina é manter uma página atual, dê a ela um artifact que você já publicou.

As rotinas pertencem à sua conta individual claude.ai. Elas não são compartilhadas com colegas de equipe e contam contra a permissão de execução diária da sua conta. Qualquer coisa que uma rotina faz através de sua identidade do GitHub conectada ou conectores aparece como você: commits e pull requests carregam seu usuário do GitHub, e mensagens do Slack, tickets do Linear ou outras ações de conector usam suas contas vinculadas para esses serviços.

<h3 id="create-from-the-web">
  Criar a partir da web
</h3>

<Steps>
  <Step title="Abrir o formulário de criação">
    Visite [claude.ai/code/routines](https://claude.ai/code/routines) e clique em **New routine**.
  </Step>

  <Step title="Nomeie a rotina e escreva o prompt">
    Dê à rotina um nome descritivo e escreva o prompt que Claude executa cada vez. O prompt é a parte mais importante: a rotina é executada autonomamente, portanto o prompt deve ser autossuficiente e explícito sobre o que fazer e como o sucesso se parece.

    Quando um acionador dispara, a sessão recebe o prompt salvo da rotina como sua tarefa atribuída e o executa, em vez de tratá-lo como conteúdo não confiável que chegou no meio da conversa. O acionador apenas atesta que o prompt foi armazenado antecipadamente por uma sessão autorizada em sua conta, portanto o prompt disparado não é entrada de usuário ao vivo e não pode agir como aprovação ou consentimento para ações durante a execução. O conteúdo que a sessão busca durante a execução mantém seu tratamento normal. Antes da v2.1.213, a sessão recebia o mesmo prompt enquadrado como uma notificação de fundo não confiável e poderia recusar agir sobre ele.

    A entrada do prompt inclui um seletor de modelo. Claude usa o modelo selecionado em cada execução.
  </Step>

  <Step title="Selecionar repositórios">
    Adicione um ou mais repositórios do GitHub para Claude trabalhar. Cada repositório é clonado no início de uma execução, começando a partir do branch padrão. Claude cria branches com prefixo `claude/` para suas alterações.
  </Step>

  <Step title="Selecionar um ambiente">
    Escolha um [ambiente em nuvem](/docs/pt/cloud-environments) para a rotina. Os ambientes controlam o que a sessão em nuvem tem acesso:

    * **Acesso à rede**: defina o nível de acesso à internet disponível durante cada execução
    * **Variáveis de ambiente**: forneça valores que Claude pode usar durante cada execução. Elas são [visíveis para qualquer pessoa que use o ambiente](/docs/pt/cloud-environments#what-carries-over-from-your-setup), portanto em planos Pro e Max, armazene chaves para as APIs que Claude chama durante uma execução como [credenciais de API](/docs/pt/cloud-environments#add-api-credentials) em vez disso. Essa seção também lista as solicitações que nunca recebem uma credencial
    * **Script de configuração**: instale dependências e ferramentas que a rotina precisa. O resultado é [armazenado em cache](/docs/pt/cloud-environments#environment-caching), portanto o script não é executado novamente em cada sessão

    Um ambiente **Default** é fornecido com acesso à rede **Trusted**, que permite apenas a [lista de permissões padrão](/docs/pt/cloud-environments#default-allowed-domains) de registros de pacotes, APIs de provedores de nuvem, registros de contêineres e domínios de desenvolvimento comuns através da rede da sessão. Os conectores que você adiciona à rotina alcançam seus serviços através dos servidores da Anthropic, portanto não precisam de alterações na lista de permissões. Se sua rotina precisar alcançar seus próprios serviços diretamente ou um domínio fora dessa lista, edite o [acesso à rede](/docs/pt/cloud-environments#network-access) do ambiente antes de executar. Para usar um ambiente separado, [crie um](/docs/pt/cloud-environments#configure-your-environment) primeiro.
  </Step>

  <Step title="Selecionar um acionador">
    Em **Select a trigger**, escolha como a rotina inicia. Você pode escolher um tipo de acionador ou combinar vários.

    <Tabs>
      <Tab title="Schedule">
        Escolha uma frequência predefinida para uma execução recorrente ou agende uma execução única em um timestamp específico. Consulte [Add a schedule trigger](#add-a-schedule-trigger) para tratamento de fuso horário, escalonamento, intervalos cron personalizados e execuções únicas.
      </Tab>

      <Tab title="GitHub event">
        Selecione o repositório, o evento para reagir e filtros opcionais. Consulte [Add a GitHub trigger](#add-a-github-trigger) para a lista completa de eventos suportados e campos de filtro.
      </Tab>

      <Tab title="API">
        Selecione **API** aqui e salve a rotina. A URL e o token são gerados após a rotina ser salva, pois dependem do ID da rotina. Consulte [Add an API trigger](#add-an-api-trigger) para copiar a URL e gerar um token.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Revisar conectores">
    Em **Connectors** na parte inferior do formulário, todos os seus [conectores MCP](/docs/pt/mcp) conectados são incluídos por padrão. Remova qualquer um que a rotina não precise: Claude pode usar todas as ferramentas de um conector incluído, incluindo escritas, sem pedir permissão durante uma execução.
  </Step>

  <Step title="Criar a rotina">
    Clique em **Create**. A rotina aparece na lista e é executada na próxima vez que um de seus acionadores corresponder. Para iniciar uma execução imediatamente, clique em **Run now** na página de detalhes da rotina.

    Cada execução cria uma nova sessão ao lado de suas outras sessões, onde você pode ver o que Claude fez, revisar alterações e criar um pull request.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  Criar a partir da CLI
</h3>

Execute `/schedule` em qualquer sessão para criar uma rotina agendada conversacionalmente. Você também pode passar uma descrição diretamente, para uma rotina recorrente como `/schedule daily PR review at 9am` ou uma única como `/schedule clean up feature flag in one week`. Claude percorre as mesmas informações que o formulário web coleta e salva a rotina em sua conta. O comando também está disponível sob o alias `/routines`.

Uma inicialização bem-sucedida parece uma conversa: Claude faz perguntas de acompanhamento sobre o cronograma, repositórios e prompt antes de salvar. Se Claude em vez disso responder que você precisa se autenticar ou que não consegue se conectar à sua conta remota claude.ai, nenhuma rotina foi criada; consulte [Troubleshooting](#troubleshooting).

`/schedule` na CLI cria rotinas agendadas. Para adicionar um acionador de API, edite a rotina na web em [claude.ai/code/routines](https://claude.ai/code/routines). Você pode adicionar um [acionador do GitHub](#add-a-github-trigger) a partir da web ou da CLI. O caminho da CLI requer Claude Code v2.1.225 ou posterior.

Uma rotina sem acionador de cronograma, como uma iniciada apenas por chamadas de API ou eventos do GitHub, não tem próximo tempo de execução, e a CLI não mostra nenhum quando Claude salva ou atualiza. Antes da v2.1.211, a CLI relatava um próximo tempo de execução no ano 1 para essas rotinas.

<h2 id="configure-triggers">
  Configurar acionadores
</h2>

Uma rotina inicia quando um de seus acionadores corresponde. Você pode anexar qualquer combinação de cronograma, API e acionadores do GitHub à mesma rotina e adicioná-los ou removê-los a qualquer momento na seção **Selecionar um acionador** do formulário de edição da rotina.

<h3 id="add-a-schedule-trigger">
  Adicionar um acionador de cronograma
</h3>

Um acionador de cronograma executa a rotina em uma cadência recorrente ou uma única vez em um momento futuro específico. Escolha uma frequência predefinida na seção **Selecionar um acionador**: horária, diária, dias da semana ou semanal. Os horários são inseridos em seu fuso horário local e convertidos automaticamente, portanto a rotina é executada naquele horário de parede independentemente de onde a infraestrutura em nuvem está localizada.

As execuções podem começar alguns minutos após o horário agendado devido ao escalonamento. O deslocamento é consistente para cada rotina.

Para um intervalo personalizado, como a cada duas horas ou no primeiro de cada mês, escolha a predefinição mais próxima no formulário e execute `/schedule update` na CLI para definir uma expressão cron específica. O intervalo mínimo é uma hora; expressões que são executadas com mais frequência são rejeitadas.

<h4 id="schedule-a-one-off-run">
  Agendar uma execução única
</h4>

Uma execução única dispara a rotina uma única vez em um timestamp específico. Use-a para lembrá-lo mais tarde na semana, para abrir um PR de limpeza após um rollout terminar ou para iniciar uma tarefa de acompanhamento quando uma mudança upstream chegar. Após a rotina disparar, ela se desativa automaticamente e a interface web a marca como **Executada**. Para executá-la novamente, edite a rotina e defina um novo horário único.

Crie uma execução única a partir da CLI descrevendo o horário em linguagem natural. Claude resolve a frase em relação ao horário atual e confirma o timestamp absoluto antes de salvar.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

A mesma conversão local-para-UTC que os cronogramas recorrentes se aplica aos timestamps únicos.

As execuções únicas não contam contra o limite diário de execução de rotina. Consulte [Uso e limites](#usage-and-limits) para obter detalhes.

<h3 id="add-an-api-trigger">
  Adicionar um acionador de API
</h3>

Um acionador de API fornece a uma rotina um endpoint HTTP dedicado. POSTando para o endpoint com o token de portador da rotina inicia uma nova sessão e retorna uma URL de sessão. Use isso para conectar Claude Code em sistemas de alerta, pipelines de implantação, ferramentas internas ou em qualquer lugar onde você possa fazer uma solicitação HTTP autenticada.

Os acionadores de API são adicionados a uma rotina existente a partir da web. A CLI atualmente não pode criar ou revogar tokens.

<Steps>
  <Step title="Abrir a rotina para edição">
    Vá para [claude.ai/code/routines](https://claude.ai/code/routines), clique na rotina que deseja acionar via API, então abra o menu ao lado do nome da rotina e selecione **Editar**.
  </Step>

  <Step title="Adicionar um acionador de API">
    Role até a seção **Selecionar um acionador** abaixo da caixa **Instruções**, clique em **Adicionar outro acionador** e escolha **API**.
  </Step>

  <Step title="Copiar a URL e gerar um token">
    O modal mostra a URL para esta rotina junto com um comando curl de exemplo. Copie a URL, então clique em **Gerar token** e copie o token imediatamente. O token é mostrado uma vez e não pode ser recuperado posteriormente, portanto armazene-o em algum lugar seguro, como o armazenamento de segredos da sua ferramenta de alerta.
  </Step>

  <Step title="Chamar o endpoint">
    Envie o token no cabeçalho `Authorization: Bearer` quando você POST para a URL. A seção [Acionar uma rotina](#trigger-a-routine) abaixo mostra um exemplo completo.
  </Step>
</Steps>

Cada rotina tem seu próprio token, limitado ao acionamento apenas dessa rotina. Para rotacioná-lo ou revogá-lo, retorne ao mesmo modal e clique em **Regenerar** ou **Revogar**.

<h4 id="trigger-a-routine">
  Acionar uma rotina
</h4>

Envie uma solicitação POST para o endpoint `/fire` com o token de portador no cabeçalho `Authorization`. O corpo da solicitação aceita um campo `text` opcional para contexto específico da execução, como um corpo de alerta ou um log com falha, passado para a rotina junto com seu prompt salvo. O valor é texto livre e não é analisado: se você enviar JSON ou outra carga estruturada, a rotina a recebe como uma string literal.

O valor `text` não chega à rotina como uma mensagem simples. Ele chega envolvido em um bloco `<routine-fire-payload>` que o rotula como dados não confiáveis e diz ao Claude para não seguir instruções dentro dele a menos que o próprio prompt da rotina diga para fazer isso. O mesmo envolvimento se aplica ao texto fornecido com **Executar agora** na interface web.

Isso significa que o prompt salvo de uma rotina deve optar por agir no texto de disparo: escreva o prompt para referenciar a carga explicitamente, por exemplo "Investigue o alerta descrito no bloco routine-fire-payload", ou a rotina trata o texto como contexto inerte. Qualquer pessoa que tenha o token de portador pode enviar `text`, portanto o wrapper faz com que o texto de disparo de um token vazado chegue rotulado como dados não confiáveis em vez de como instruções diretas para sua rotina.

O exemplo abaixo aciona uma rotina a partir de um shell. O ID da rotina e o token mostrados são placeholders: substitua-os pela URL e token que você copiou ao [adicionar o acionador de API](#add-an-api-trigger), ou a solicitação falhará com um erro de autenticação `401`:

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

Uma solicitação bem-sucedida retorna um corpo JSON com o novo ID de sessão e URL:

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

Abra a URL da sessão em um navegador para assistir à execução em tempo real, revisar alterações ou continuar a conversa manualmente.

<Warning>
  O endpoint `/fire` é enviado sob o cabeçalho beta `experimental-cc-routine-2026-04-01`. As formas de solicitação e resposta, limites de taxa e semântica de token podem mudar enquanto o recurso está em visualização de pesquisa. As alterações significativas são enviadas atrás de novas versões de cabeçalho beta datadas, e as duas versões de cabeçalho anteriores mais recentes continuam funcionando para que os chamadores tenham tempo para migrar.
</Warning>

<h4 id="api-reference">
  Referência de API
</h4>

Para a referência completa da API, incluindo todas as respostas de erro, regras de validação e limites de campo, consulte [Acionar uma rotina via API](https://platform.claude.com/docs/en/api/claude-code/routines-fire) na documentação da Plataforma Claude.

O endpoint `/fire` está disponível apenas para usuários de claude.ai e não faz parte da superfície da API da Plataforma Claude.

<h3 id="add-a-github-trigger">
  Adicionar um acionador do GitHub
</h3>

Um acionador do GitHub inicia uma nova sessão automaticamente quando um evento correspondente ocorre em um repositório conectado. Claude Code não reutiliza sessões entre eventos, portanto duas atualizações de PR produzem duas sessões independentes.

<Note>
  Durante a visualização de pesquisa, os eventos de webhook do GitHub estão sujeitos a limites por hora por rotina e por conta. Os eventos além do limite são descartados até que a janela seja redefinida. Veja seus limites atuais em [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

O aplicativo Claude GitHub deve ser instalado no repositório ao qual você deseja se inscrever, qualquer que seja a superfície a partir da qual você configura o acionador.

* Configure acionadores do GitHub a partir da interface do usuário da web, que solicita que você instale o aplicativo quando ele estiver faltando. Siga as etapas abaixo para configurar um na web.
* A partir da CLI, instale o aplicativo a partir da [página do Aplicativo GitHub](https://github.com/apps/claude) primeiro, depois peça ao Claude para anexar um acionador do GitHub a uma rotina existente, por exemplo `/schedule add a GitHub trigger to my nightly review for pull requests opened in acme/webapp`. O caminho da CLI requer Claude Code v2.1.225 ou posterior. Quando Claude adiciona o acionador, ele responde com um link para a rotina que o acionador dispara.

<Steps>
  <Step title="Abrir a rotina para edição">
    Vá para [claude.ai/code/routines](https://claude.ai/code/routines), clique na rotina, então abra o menu ao lado do nome da rotina e selecione **Editar**.
  </Step>

  <Step title="Adicionar um acionador de evento do GitHub">
    Role até a seção **Selecionar um acionador**, clique em **Adicionar outro acionador** e escolha **Evento do GitHub**.

    <Note>
      Executar `/web-setup` na CLI concede acesso ao repositório para clonagem, mas não instala o aplicativo Claude GitHub e não ativa a entrega de webhook.
    </Note>
  </Step>

  <Step title="Configurar o acionador">
    Selecione o repositório, escolha um evento da lista de [eventos suportados](#supported-events) e opcionalmente adicione filtros. Salve o acionador.
  </Step>
</Steps>

<h4 id="supported-events">
  Eventos suportados
</h4>

Os acionadores do GitHub podem se inscrever em uma das seguintes categorias de eventos. Dentro de cada categoria, você pode escolher uma ação específica, como `pull_request.opened`, ou reagir a todas as ações na categoria.

| Evento       | Acionadores quando                                                                      |
| :----------- | :-------------------------------------------------------------------------------------- |
| Pull request | Um PR é aberto, fechado, atribuído, rotulado, sincronizado ou atualizado de outra forma |
| Lançamento   | Um lançamento é criado, publicado, editado ou excluído                                  |

<h4 id="filter-pull-requests">
  Filtrar pull requests
</h4>

Use filtros para restringir quais pull requests iniciam uma nova sessão. Todas as condições de filtro devem corresponder para a rotina ser acionada. Os campos de filtro disponíveis são:

| Filtro                  | Corresponde                              |
| :---------------------- | :--------------------------------------- |
| Autor                   | Nome de usuário do GitHub do autor do PR |
| Título                  | Texto do título do PR                    |
| Corpo                   | Texto da descrição do PR                 |
| Branch base             | Branch que o PR tem como alvo            |
| Branch de origem (head) | Branch de onde o PR vem                  |
| Rótulos                 | Rótulos aplicados ao PR                  |
| É rascunho              | Se o PR está em estado de rascunho       |
| É mesclado              | Se o PR foi mesclado                     |

Cada filtro emparelha um campo com um operador: equals, contains, starts with, is one of, is not one of ou matches regex.

O operador `matches regex` testa o valor do campo inteiro, não uma substring dentro dele. Para corresponder a qualquer título contendo `hotfix`, escreva `.*hotfix.*`. Sem o `.*` circundante, o filtro corresponde apenas a um título que é exatamente `hotfix` sem nada antes ou depois. Para correspondência de substring literal sem sintaxe regex, use o operador `contains` em vez disso.

Alguns exemplos de combinações de filtro:

* **Revisão do módulo de autenticação**: branch base `main`, branch de origem (head) contém `auth-provider`. Envia qualquer PR que toque em autenticação para um revisor focado.
* **Pronto para revisão apenas**: é rascunho é `false`. Pula rascunhos para que a rotina seja executada apenas quando o PR estiver pronto para revisão.
* **Backport condicionado a rótulo**: rótulos incluem `needs-backport`. Aciona uma rotina que porta a alteração para outro branch apenas quando um mantenedor marca o PR.

<h2 id="manage-routines">
  Gerenciar rotinas
</h2>

Clique em uma rotina na lista para abrir sua página de detalhes. A página de detalhes mostra os repositórios da rotina, conectores, prompt, cronograma, tokens de API, acionadores do GitHub e uma lista de execuções anteriores.

<h3 id="view-and-interact-with-runs">
  Visualizar e interagir com execuções
</h3>

Clique em qualquer execução para abri-la como uma sessão completa. De lá você pode ver o que Claude fez, revisar alterações, criar um pull request ou continuar a conversa. Cada sessão de execução funciona como qualquer outra sessão: use o menu suspenso ao lado do título da sessão para renomear, arquivar ou excluir.

<Note>
  Um status verde na lista de execuções significa que a sessão iniciou e saiu sem um erro de infraestrutura. Isso não significa que a tarefa em seu prompt foi bem-sucedida. Abra a execução para ler a transcrição e confirmar o que Claude realmente fez. Solicitações de rede bloqueadas, ferramentas de conectores ausentes e falhas no nível da tarefa aparecem lá em vez de no indicador de status.
</Note>

<h3 id="edit-and-control-routines">
  Editar e controlar rotinas
</h3>

Na página de detalhes da rotina você pode:

* Clique em **Executar agora** para iniciar uma execução imediatamente sem esperar pelo próximo horário agendado. Você pode opcionalmente fornecer texto específico da execução, que chega à rotina da mesma forma que o campo `text` do acionador de API.
* Use o botão de alternância na parte superior da página para pausar ou retomar o cronograma. As rotinas pausadas mantêm sua configuração mas não são executadas até que você as reative.
* Abra o menu ao lado do nome da rotina e selecione **Editar** para alterar o nome, prompt, repositórios, ambiente, conectores ou qualquer um dos acionadores da rotina. A seção **Selecionar um acionador** é onde você adiciona ou remove cronogramas, tokens de API e acionadores de eventos do GitHub.
* Abra o mesmo menu e selecione **Excluir** para excluir a rotina.

<h3 id="manage-routines-from-the-cli">
  Gerenciar rotinas a partir da CLI
</h3>

A CLI suporta o gerenciamento de rotinas existentes. Execute `/schedule list` para ver todas as rotinas, `/schedule update` para alterar uma, ou `/schedule run` para acioná-la imediatamente.

Você também pode fazer perguntas sobre o histórico de execuções de uma rotina, por exemplo `/schedule why did my nightly review do nothing this morning?`. Claude lista as execuções recentes da rotina com seu status e um link para [abrir cada execução na web](#view-and-interact-with-runs), e lê o log de uma execução para explicar o que aconteceu, incluindo erros de ferramentas, negações de permissão e o resultado final. Requer Claude Code v2.1.227 ou posterior.

<h3 id="repositories-and-branch-permissions">
  Repositórios e permissões de branch
</h3>

As rotinas precisam de acesso ao GitHub para clonar repositórios. Quando você cria uma rotina a partir da CLI com `/schedule`, Claude verifica se sua conta tem acesso ao GitHub para o repositório do qual você a executou e, se não tiver, adiciona uma nota de configuração nomeando como conceder acesso. Consulte [Opções de autenticação do GitHub](/docs/pt/claude-code-on-the-web#github-authentication-options) para as duas maneiras de conceder acesso.

Se sua conexão do GitHub estiver ausente ou expirada quando uma execução estiver programada, a rotina pula execuções até que você se reconecte, por até 72 horas. Reconecte o GitHub dentro dessa janela e a rotina retoma automaticamente. Após 72 horas sem uma conexão, a rotina se desativa e você a reativa após reconectar o GitHub.

Cada repositório que você adiciona é clonado em cada execução. Claude começa a partir do branch padrão do repositório, a menos que seu prompt especifique o contrário.

Claude faz push de seu trabalho para branches com prefixo `claude/`, que são sempre aceitos. Quando seu prompt direciona Claude para fazer push para outro branch, Claude Code verifica o push primeiro e o rejeita se qualquer um dos seguintes for verdadeiro:

* O branch é protegido no GitHub
* Alguém mais tem um pull request aberto a partir desse branch
* O branch contém commits criados por alguém que não é você

<h3 id="connectors">
  Conectores
</h3>

As rotinas podem usar seus conectores MCP conectados para ler e escrever em serviços externos durante cada execução. Por exemplo, uma rotina que faz triagem de solicitações de suporte pode ler de um canal do Slack e criar problemas no Linear.

Conectores são as [integrações do claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) em sua conta. Servidores MCP que você adicionou localmente na CLI com `claude mcp add` são armazenados em sua máquina em vez de sua conta claude.ai, portanto não aparecem na lista de conectores. Para usar um desses servidores em uma rotina, adicione-o como um conector em [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Para uma rotina com um repositório, você pode em vez disso declará-lo em um [`.mcp.json`](/docs/pt/mcp#project-scope) confirmado para que faça parte do repositório clonado.

Quando você cria uma rotina, todos os seus conectores atualmente conectados são incluídos por padrão. Remova qualquer um que não seja necessário para limitar quais ferramentas Claude tem acesso durante a execução. Você também pode adicionar conectores diretamente do formulário de rotina.

Para gerenciar ou adicionar conectores fora do formulário de rotina, visite [claude.ai/customize/connectors](https://claude.ai/customize/connectors) ou use `/schedule update` na CLI.

<h3 id="environments-and-network-access">
  Ambientes e acesso à rede
</h3>

Cada rotina usa um [ambiente em nuvem](/docs/pt/cloud-environments) que controla acesso à rede, variáveis de ambiente e scripts de configuração. A rotina herda a política de rede do ambiente em cada execução.

O ambiente **Padrão** usa acesso à rede **Confiável**, que permite apenas a [lista de permissões padrão](/docs/pt/cloud-environments#default-allowed-domains) através da rede da sessão. Solicitações nesse caminho para hosts fora da lista de permissões falham com `403` e `x-deny-reason: host_not_allowed`. O tráfego do conector MCP é roteado através dos servidores da Anthropic em vez desse caminho, portanto os conectores que você adiciona à rotina funcionam sem adicionar seus hosts aos **Domínios permitidos**. Remova qualquer conectores que você não precise em [Conectores](#connectors).

Para permitir domínios adicionais em um de seus próprios ambientes, siga estas etapas. Um [ambiente compartilhado pela organização](/docs/pt/cloud-environments#organization-shared-environments) abre como somente leitura aqui, portanto um Proprietário altera seu acesso à rede na página **Ambientes em nuvem** em [configurações de administração](https://claude.ai/admin-settings).

<Steps>
  <Step title="Abra a rotina para edição">
    Na página de detalhes da rotina, abra o menu ao lado do nome da rotina e selecione **Editar**.
  </Step>

  <Step title="Abra o seletor de ambiente">
    Abaixo da caixa **Instruções**, selecione o ícone de nuvem mostrando o nome do seu ambiente, como **Padrão**.
  </Step>

  <Step title="Abra as configurações de ambiente">
    Passe o mouse sobre o ambiente na lista e clique no ícone de configurações que aparece à direita.
  </Step>

  <Step title="Altere o nível de acesso à rede">
    Na caixa de diálogo **Atualizar ambiente em nuvem**, altere **Acesso à rede** para **Personalizado** e insira seus domínios em **Domínios permitidos**. Marque **Também incluir lista padrão de gerenciadores de pacotes comuns** para manter a [lista de permissões padrão](/docs/pt/cloud-environments#default-allowed-domains) junto com seus domínios personalizados. Selecione **Completo** em vez disso para acesso irrestrito.
  </Step>

  <Step title="Salvar">
    Clique em **Salvar alterações**. A nova política se aplica a partir da próxima execução.
  </Step>
</Steps>

Consulte [Acesso à rede](/docs/pt/cloud-environments#network-access) para detalhes sobre níveis de acesso e a lista de permissões padrão.

<h2 id="usage-and-limits">
  Uso e limites
</h2>

As rotinas reduzem o uso da assinatura da mesma forma que as sessões interativas. Além dos limites de assinatura padrão, as rotinas têm um limite diário de quantas execuções podem começar por conta. Veja seu consumo atual e execuções de rotina diárias restantes em [claude.ai/code/routines](https://claude.ai/code/routines) ou [claude.ai/settings/usage](https://claude.ai/settings/usage).

Quando uma rotina atinge o limite diário ou seu limite de uso de assinatura, organizações com créditos de uso ativados podem continuar executando rotinas em excesso medido. Sem créditos de uso, execuções adicionais são rejeitadas até que a janela seja redefinida. Ative os créditos de uso em [claude.ai/settings/usage](https://claude.ai/settings/usage). Nos planos Team e Enterprise, um administrador os ativa para a organização em [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

As execuções únicas não contam contra o limite diário de execução de rotina. Elas reduzem seu uso de assinatura regular como qualquer outra sessão.

Enquanto sua assinatura está pausada, suas rotinas são colocadas em espera e não são executadas. Assim que sua assinatura estiver ativa novamente, ative-as novamente.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

<h3 id="schedule-returns-unknown-command">
  `/schedule` retorna "Unknown command"
</h3>

A CLI oculta `/schedule` quando um de seus requisitos não é atendido: o menu de comandos mostra `No commands match "/schedule"` enquanto você digita. Enviá-lo retorna `Unknown command: /schedule`, exceto nos casos abaixo que indicam uma resposta diferente.

A causa geralmente é uma das seguintes:

* Você está autenticado com uma chave de API do Console, um [perfil Anthropic ou credencial de federação](/docs/pt/authentication#anthropic-profiles-and-federation-credentials), ou um provedor de nuvem como Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. `/schedule` requer um login de assinatura claude.ai. Com uma chave de API do Console ou um perfil, e busca de sinalizadores de recursos habilitada, enviar `/schedule` mostra `/schedule is available with Claude for Enterprise — ask your admin about migrating from API-key access`. Com um login de provedor de nuvem, você ainda vê `Unknown command: /schedule`. Se `ANTHROPIC_API_KEY` ou `ANTHROPIC_AUTH_TOKEN` estiver definido em seu shell, ou `apiKeyHelper` estiver definido em `settings.json`, remova-o primeiro, pois esses têm precedência sobre um login claude.ai. Um perfil ou credencial de federação também tem precedência, então desative isso também
* Você está totalmente desconectado, sem chave de API ou outra credencial. Com busca de sinalizadores de recursos habilitada, enviar `/schedule` mostra `/schedule requires a claude.ai subscription. Run /login to sign in with your claude.ai account.` Antes da v2.1.268, uma sessão desconectada mostrava a mesma mensagem Claude for Enterprise que uma chave de API do Console
* Você está dentro de uma sessão na nuvem, onde enviar `/schedule` responde que o comando não está disponível nesse ambiente. Gerencie rotinas a partir da [interface web](https://claude.ai/code/routines)
* A política da sua organização desabilita [Claude Code na web](/docs/pt/claude-code-on-the-web), na qual as rotinas são executadas. Neste caso, enviar `/schedule` responde [`Cloud sessions are disabled by your organization's policy`](/docs/pt/errors#cloud-sessions-are-disabled-by-your-organizations-policy). Antes da v2.1.268, retornava `Unknown command: /schedule`
* Um Owner [desativou rotinas](#routines-are-disabled-by-your-organizations-policy) para sua organização Team ou Enterprise. Antes da v2.1.227, o comando ainda aparecia neste caso, e claude.ai rejeitava a rotina quando Claude tentava criá-la ou executá-la

A menos que a política da sua organização desabilite rotinas ou Claude Code na web, você pode criar e gerenciar rotinas em [claude.ai/code/routines](https://claude.ai/code/routines) independentemente de como a CLI está configurada.

<h3 id="routines-are-disabled-by-your-organizations-policy">
  "As rotinas estão desabilitadas pela política da sua organização"
</h3>

Um Owner em sua organização Team ou Enterprise provavelmente desativou o botão de alternância **Routines** em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). No Claude Code v2.1.227 ou posterior, o mesmo botão de alternância também oculta `/schedule` na CLI. Esta é uma configuração de organização no servidor, portanto não pode ser substituída pela sua configuração local. Peça a um Owner que habilite rotinas para sua organização.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [`/loop` e agendamento em sessão](/docs/pt/scheduled-tasks): agende tarefas locais dentro de uma sessão CLI aberta
* [Tarefas agendadas do Desktop](/docs/pt/desktop-scheduled-tasks): tarefas agendadas locais que são executadas em sua máquina com acesso a arquivos locais
* [Ambientes em nuvem](/docs/pt/cloud-environments): configure acesso à rede, variáveis de ambiente e scripts de configuração para sessões em nuvem
* [Projetos](/docs/pt/claude-projects): trabalho contínuo que Claude coordena em sessões em nuvem paralelas; rotinas criadas a partir de um projeto aparecem na aba **Rotinas**
* [Conectores MCP](/docs/pt/mcp): conecte serviços externos como Slack, Linear e Google Drive
* [GitHub Actions](/docs/pt/github-actions): execute Claude em seu pipeline de CI em eventos de repositório
