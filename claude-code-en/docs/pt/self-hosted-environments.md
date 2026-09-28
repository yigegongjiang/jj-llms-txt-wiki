> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ambientes auto-hospedados

> Execute sessões de Claude Code na nuvem em infraestrutura que você controla: configure um ambiente auto-hospedado, implante runners e roteie sessões para sua própria computação.

<Note>
  Ambientes auto-hospedados estão em beta pública nos planos Team e Enterprise e estão desativados por padrão. Consulte [Disponibilidade e limitações](#availability-and-limitations) para o caminho de habilitação e o que está excluído.
</Note>

Um ambiente auto-hospedado executa sessões de Claude Code na nuvem em infraestrutura que sua organização opera. Uma [sessão na nuvem](/docs/pt/claude-code-on-the-web) é qualquer sessão que é executada em algum lugar que não seja a máquina do desenvolvedor: os desenvolvedores as iniciam a partir de claude.ai, dos aplicativos móvel e desktop, do terminal com [`claude --cloud`](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud) e [rotinas agendadas](/docs/pt/routines), e por padrão são executadas na infraestrutura da Anthropic. Em um ambiente auto-hospedado, essas mesmas sessões são executadas dentro de sua rede, e a experiência do desenvolvedor é a mesma, exceto pelas diferenças em [Disponibilidade e limitações](#availability-and-limitations) e os [problemas conhecidos](/docs/pt/self-hosted-environments-deploy#known-issues-and-limitations) da página de implantação.

Se sua equipe não usa sessões na nuvem, não há nada para configurar aqui: sessões em um terminal ou IDE sempre são executadas na máquina do próprio desenvolvedor. Se você deseja executar Claude Code em sua própria máquina sempre ativa e controlá-la a partir de outros dispositivos, use [Controle Remoto](/docs/pt/remote-control), que também está disponível nos planos Pro e Max. Quando estiver pronto para configurar, vá direto para o [guia de início rápido](/docs/pt/self-hosted-environments-quickstart); para revisar a postura de segurança primeiro, comece com [Implantar em produção](/docs/pt/self-hosted-environments-deploy). O resto desta página explica como funciona a auto-hospedagem e quando escolhê-la.

<h2 id="how-self-hosted-environments-work">
  Como funcionam os ambientes auto-hospedados
</h2>

A auto-hospedagem tem três partes:

* **Ambiente**: um destino nomeado para o qual as sessões na nuvem podem ser enviadas. Sua organização cria ambientes nas configurações de administrador de claude.ai, e cada um agrupa um conjunto de runners.
* **Runner**: um programa em execução em hosts dentro de sua rede. Os runners executam as sessões; a ideia é a mesma de um runner de CI auto-hospedado.
* **Sessão**: uma tarefa de Claude Code que um desenvolvedor iniciou.

Quando um desenvolvedor inicia uma sessão na nuvem, a interface de início de sessão mostra um seletor de ambiente listando ambientes hospedados pela Anthropic ao lado de qualquer um que sua organização tenha criado. Se escolherem o seu, o plano de controle da Anthropic coloca a sessão na fila do seu ambiente, onde um runner a reclama, clona o repositório que o desenvolvedor escolheu e inicia um processo de Claude Code em seu host para executá-lo. O runner se autentica em seu host git com credenciais que você configura; [Configurar git](/docs/pt/self-hosted-environments-deploy#configure-git) cobre as opções. As sessões alcançam seus serviços internos de dentro de sua rede, e seu host git da mesma forma quando é interno; o tráfego para Anthropic, sondagem de fila, o fluxo de eventos da sessão e inferência de modelo, é HTTPS de saída para `api.anthropic.com`, com a lista curta de hosts adicionais que as sessões podem alcançar em [Requisitos de rede](/docs/pt/self-hosted-environments-deploy#network-requirements). A Anthropic nunca se conecta em sua rede.

<div style={{maxWidth: "640px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=8056103fc1c5564c7f0ef219d260b99d" className="dark:hidden" alt="Diagrama de arquitetura de um ambiente auto-hospedado: o limite de sua rede contém um runner, dois processos de sessão de Claude Code dentro dele e seu host git, com api.anthropic.com fora contendo fila, fluxo de sessão e inferência. O runner sonda a fila e alcança o host git, cada processo de sessão abre suas próprias conexões de fluxo, inferência e git, e cada conexão é de saída de sua rede, sem nenhuma de entrada." width="680" height="320" data-path="images/self-hosted-network-paths.svg" />

    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths-dark.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=fec6aef3b0740d80eaf6d6a7000a2233" className="hidden dark:block" alt="Diagrama de arquitetura de um ambiente auto-hospedado: o limite de sua rede contém um runner, dois processos de sessão de Claude Code dentro dele e seu host git, com api.anthropic.com fora contendo fila, fluxo de sessão e inferência. O runner sonda a fila e alcança o host git, cada processo de sessão abre suas próprias conexões de fluxo, inferência e git, e cada conexão é de saída de sua rede, sem nenhuma de entrada." width="680" height="320" data-path="images/self-hosted-network-paths-dark.svg" />
  </Frame>
</div>

As duas caixas de Claude Code no diagrama são processos de sessão: um runner executando duas sessões ao mesmo tempo, até sua capacidade configurada. Um runner serve um [proprietário](#key-concepts) por vez e se bloqueia para esse proprietário quando reclama sua primeira sessão, portanto o código verificado nunca se mistura entre proprietários; [Ciclo de vida do runner](#runner-lifecycle) cobre a regra.

Você pode iniciar runners você mesmo e mantê-los em execução, ou executar o [orquestrador de dimensionamento automático](/docs/pt/self-hosted-environments-configuration#on-demand-runners), um segundo processo que você hospeda, que inicia runners conforme as sessões são enfileiradas; cada runner sai por conta própria quando seu trabalho termina. De qualquer forma, você configura o ambiente uma vez, e ele aparece no seletor em todas as superfícies suportadas.

<h2 id="availability-and-limitations">
  Disponibilidade e limitações
</h2>

Verifique estas antes de planejar um lançamento:

* **Planos**: beta pública para organizações Team e Enterprise. Ambientes auto-hospedados estão desativados por padrão; um [Proprietário](/docs/pt/cloud-environments#organization-shared-environments) ativa **Permitir ambientes auto-hospedados** na [página de administrador **Ambientes na nuvem**](https://claude.ai/admin-settings/cloud-environments), que requer que [sessões na nuvem](/docs/pt/claude-code-on-the-web) estejam habilitadas para a organização.
* **Zero Data Retention**: indisponível para organizações com [Zero Data Retention](/docs/pt/zero-data-retention) habilitado.
* **Inferência de modelo**: as sessões usam a API Anthropic, e a inferência não pode ser roteada através de [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/pt/third-party-integrations) ou um [gateway LLM](/docs/pt/llm-gateway).
* **Superfícies**: sessões iniciadas a partir de [claude.ai/code](https://claude.ai/code), dos aplicativos móvel e desktop, [rotinas agendadas](/docs/pt/routines) e do terminal, com [`claude --cloud`](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud) ou um [despacho `--environment`](/docs/pt/self-hosted-environments-testing#run-the-test-loop), podem ser executadas em ambientes auto-hospedados. Sessões de [Claude Tag](https://claude.com/docs/claude-tag/overview) também podem ser executadas neles, mas Claude ainda não pode usar [Pacotes de acesso](https://claude.com/docs/claude-tag/concepts/glossary#access-bundle) nessas sessões. Sessões de [Claude Security](/docs/pt/claude-security) e [Code Review](/docs/pt/code-review) ainda não são roteadas para eles. O suporte para essas duas superfícies segue separadamente.
* **Repositórios**: as sessões verificam repositórios do GitHub; consulte [Opções de autenticação do GitHub](/docs/pt/claude-code-on-the-web#github-authentication-options).
* **Faturamento**: as sessões em um ambiente auto-hospedado consomem o uso de Claude Code de sua organização da mesma forma que as sessões em ambientes hospedados pela Anthropic.

<h2 id="why-self-host">
  Por que auto-hospedar
</h2>

A maioria das equipes é melhor servida por ambientes hospedados pela Anthropic, que não precisam de infraestrutura para executar ou manter. A auto-hospedagem é para equipes cujos requisitos de rede, ferramentas ou conformidade exigem manter a execução da sessão em infraestrutura que controlam. Se esse for o seu caso, planeje pela propriedade operacional que ela carrega: você constrói e mantém a imagem do runner, opera a frota e controla sua rede.

Em troca, a auto-hospedagem oferece acesso à rede, ferramentas personalizadas e controle de conformidade:

* **Acesso à rede**: as sessões são executadas dentro de sua rede e podem alcançar serviços internos, bancos de dados e registros sem expô-los à internet pública
* **Ferramentas personalizadas**: pré-instale compiladores, SDKs e CLIs internos em sua imagem de runner para que cada sessão comece pronta para compilar
* **Conformidade**: as verificações de repositório e artefatos de compilação permanecem em infraestrutura que você controla. O conteúdo da sessão ainda vai para `api.anthropic.com` para inferência de modelo.

<h2 id="environments-runners-and-sessions">
  Ambientes, runners e sessões
</h2>

Os ambientes são gerenciados na página **Ambientes na nuvem** nas configurações de administrador de claude.ai; os runners são processos que você inicia e gerencia em sua própria infraestrutura.

<h3 id="key-concepts">
  Conceitos-chave
</h3>

Estes termos aparecem em todas as páginas auto-hospedadas:

| Termo               | O que é                                                                                                                                                                                                                                  |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ambiente            | Um grupo nomeado de seus runners, criado nas configurações de claude.ai. As sessões são roteadas para um ambiente, não para um runner individual.                                                                                        |
| Segredo do ambiente | A credencial compartilhada única que os runners usam para se autenticar e registrar no ambiente. Mostrado uma vez na criação do ambiente, rotulado como **chave de ambiente** na interface de administrador.                             |
| Runner              | O processo de longa duração que você implanta. Um runner se registra no ambiente, recebe um token de runner e sonda por sessões.                                                                                                         |
| Sessão              | Uma tarefa de Claude Code, iniciada a partir de claude.ai, do aplicativo móvel ou de outra superfície Anthropic, como uma rotina agendada ou um agente. Cada sessão é executada como um processo filho de Claude Code que o runner gera. |

Em campos de API, reivindicações de token e nomes de métrica, o ambiente aparece como `pool`, e o ID do ambiente é o `pool_id`. A [referência](/docs/pt/self-hosted-environments-reference) mapeia as duas grafias, incluindo os nomes de flag `pool` descontinuados.

Um runner serve um proprietário por vez. A primeira sessão que um runner pega bloqueia o runner para o proprietário dessa sessão, e o runner então executa sessões apenas para esse proprietário, até uma capacidade configurada. Quem é o proprietário depende de como a sessão foi iniciada:

* **Sessões que um usuário inicia**: o proprietário é a conta desse usuário.
* **Sessões de canal Claude Tag**: Claude as executa sem nenhuma conta de usuário anexada, portanto o proprietário é o [agente Claude Tag](https://claude.com/docs/claude-tag/concepts/glossary#agent-identity) que iniciou a sessão. Cada sessão de canal que esse agente inicia tem o mesmo proprietário, quem quer que tenha enviado a mensagem do Slack, portanto um runner bloqueado para ela serve sessões que diferentes pessoas iniciaram quando você a executa em um `--capacity` acima de um ou com um `--drain-grace-sec` positivo. Um runner bloqueado para um usuário nunca pega estes, e um runner bloqueado para um agente Claude Tag nunca pega as sessões de um usuário.

O tamanho mínimo da frota é, portanto, o número de proprietários que você espera estar ativos de uma vez, contando usuários e agentes Claude Tag.

<h3 id="session-lifecycle">
  Ciclo de vida da sessão
</h3>

Quando um desenvolvedor inicia uma sessão e seleciona seu ambiente, o plano de controle da Anthropic coloca a sessão na fila do ambiente. De lá:

1. Um runner com capacidade livre reclama a sessão e mantém uma concessão sobre ela.
2. O runner clona o repositório em seu diretório de trabalho e gera um processo filho de Claude Code.
3. O filho transmite eventos de volta por HTTPS enquanto o runner continua sondando; cada sondagem atualiza a concessão e funciona como o batimento cardíaco.
4. Se o runner parar de sondar por cerca de 60 segundos, o servidor recoloca a sessão na fila para outro runner.

O runner dá a cada solicitação de sondagem 10 segundos. Quando uma solicitação expira, é perdida ou recebe uma resposta que o runner não consegue analisar, o runner continua servindo suas sessões ativas e tenta novamente após um segundo ou dois em vez de esperar pela próxima sondagem agendada. Por exemplo, um proxy interceptador que responde à sondagem com sua própria página produz uma resposta que o runner não consegue analisar. Cada vez que outra solicitação falha de uma dessas maneiras, o runner dobra a lacuna antes da próxima tentativa, até 20 segundos, e encurta a lacuna sempre que a concessão está próxima de expirar.

<h3 id="runner-lifecycle">
  Ciclo de vida do runner
</h3>

A primeira sessão que um runner pega bloqueia o runner para o proprietário dessa sessão, e o runner executa até `--capacity` sessões simultâneas para esse proprietário. Enquanto o runner tem sessões ativas e não recebeu um sinal de desligamento ou atingiu seu tempo de aposentadoria, o runner continua reivindicando o trabalho enfileirado do proprietário bloqueado. O que acontece depois que terminam depende de [`--drain-grace-sec`](/docs/pt/self-hosted-environments-reference#runner-cli-flags):

* **No padrão de `0`**: o runner sai assim que suas sessões ativas terminam, sem sondar mais, portanto o orquestrador em que você o implanta, como Kubernetes, pode reiniciá-lo com um disco fresco, pronto para servir qualquer proprietário.
* **Em um valor positivo**: o runner continua sondando a fila do proprietário bloqueado por esse número de segundos antes de sair.

Este ciclo de vida isola o código verificado de cada proprietário sem exigir que o runner exclua o estado do disco entre proprietários.

Como sua infraestrutura para um runner decide se você precisa de `--retire-at`. Uma morte que entrega `SIGTERM` não precisa de flag: o runner drena conforme [Tempo de desligamento](/docs/pt/self-hosted-environments-deploy#shutdown-timing) descreve, ou continua servindo as sessões que já mantém quando você define [`--defer-shutdown-max-min`](/docs/pt/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal). Se sua infraestrutura em vez disso destrói hosts em um tempo de relógio de parede conhecido sem um sinal, ou com um período de carência muito curto para drenar, como um limite de tempo de vida de sandbox ou reclamação de instância spot, passe `--retire-at <epoch-seconds>` definido para alguns minutos antes desse tempo. No tempo de aposentadoria:

1. O runner para de aceitar novo trabalho.
2. O runner libera cada sessão ativa através do mesmo caminho de liberação que o flag [`--release-idle-session-min`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) usa, portanto a sessão retoma em um runner fresco quando o usuário envia sua próxima mensagem. Quando o runner libera cada sessão depende de seu estado:
   * O runner libera uma sessão que está no meio de uma volta assim que essa volta termina.
   * Quando uma volta termina e deixa tarefas em segundo plano em execução, o runner espera até 60 segundos por elas, depois libera a sessão mesmo que ainda estejam em execução. Se as tarefas terminaram mas a volta de acompanhamento que lê seus resultados ainda não foi executada, o runner mantém a sessão até que essa volta termine, e não espera mais do que [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](/docs/pt/self-hosted-environments-reference#environment-variable-only-settings) para que essa volta comece.
3. O runner sai 0 assim que todas as suas sessões são liberadas.

Uma volta que sobrevive à morte ainda é perdida; [Tempo de desligamento](/docs/pt/self-hosted-environments-deploy#shutdown-timing) cobre o dimensionamento da margem. Sem `--retire-at`, uma morte de host sem sinal é indistinguível de um crash: o plano de controle registra um worker perdido em vez de uma liberação limpa, e a sessão recoloca na fila para outro runner.

<h3 id="network-paths">
  Caminhos de rede
</h3>

O runner e suas sessões fazem vários tipos de conexão de saída, e nenhuma conectividade de entrada de Anthropic é necessária:

* **Plano de controle**: o runner sonda `api.anthropic.com` para trabalho e publica eventos de progresso de configuração e falha, tudo HTTPS de saída. A sondagem funciona como o batimento cardíaco do runner.
* **Conector SCM**: o orquestrador opcional [conector SCM](/docs/pt/self-hosted-environments-reference#scm-connector-flags) tunnel é a única conexão WebSocket.
* **Git**: o runner clona de e envia para seu host git por HTTPS ou SSH, autenticado com credenciais que sua implantação fornece; [Configurar git](/docs/pt/self-hosted-environments-deploy#configure-git) cobre as opções, incluindo credenciais cunhadas por sessão e o [proxy git Anthropic](/docs/pt/self-hosted-environments-deploy#use-the-anthropic-git-proxy), que roteia git através de `api.anthropic.com` em vez disso.
* **Filho da sessão**: o processo filho de Claude Code mantém o fluxo de eventos da sessão para `api.anthropic.com` e faz suas próprias chamadas de saída para inferência de modelo e para comandos git executados durante a sessão. Consulte [Requisitos de rede](/docs/pt/self-hosted-environments-deploy#network-requirements) para a lista completa de saída. O [diagrama acima](#how-self-hosted-environments-work) mostra esses caminhos, além do conector SCM opcional.

A inferência de modelo usa a API Anthropic. O plano de controle entrega o endpoint da API para cada sessão, e a sessão se autentica com um token OAuth emitido pela Anthropic, com escopo de sessão, portanto a inferência não pode ser roteada através de [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/pt/third-party-integrations) ou um [gateway LLM](/docs/pt/llm-gateway) em ambientes auto-hospedados.

Proxies de saída corporativos são suportados. O runner e o [orquestrador de dimensionamento automático](/docs/pt/self-hosted-environments-configuration#on-demand-runners) opcional honram o proxy e as variáveis de ambiente mTLS descritas em [Configuração de rede](/docs/pt/network-config), como `HTTPS_PROXY` e `NO_PROXY`; defina-as no ambiente de cada processo. As variáveis cobrem chamadas de plano de controle, o WebSocket [conector SCM](/docs/pt/self-hosted-environments-reference#scm-connector-flags) do orquestrador e o clone integrado para remotes HTTPS, e as sessões as herdam do runner. O streaming de sessão usa eventos enviados pelo servidor por HTTPS, portanto um proxy no caminho não deve armazenar em buffer as respostas.

Se seu proxy também exigir um cabeçalho `Proxy-Authorization`, o runner pode adicioná-lo a cada conexão que abre para o proxy; consulte [Autenticar em um proxy de saída](/docs/pt/self-hosted-environments-deploy#authenticate-to-an-egress-proxy).

<h2 id="what-stays-on-your-infrastructure">
  O que permanece em sua infraestrutura
</h2>

Verificações de repositório, artefatos de compilação, segredos e quaisquer arquivos que uma sessão cria ou modifica permanecem nas máquinas que você provisiona. A conversa em si, incluindo prompts, respostas e resultados de ferramentas, vai para `api.anthropic.com` para inferência de modelo, e Anthropic armazena a transcrição da sessão para que você possa retomar a sessão de outra [superfície suportada](#availability-and-limitations).

Um ambiente auto-hospedado move a execução da sessão para sua rede. O plano de controle permanece hospedado pela Anthropic: orquestração de sessão, enfileiramento e a interface de claude.ai continuam a ser executados na infraestrutura da Anthropic.

<h2 id="get-started">
  Comece
</h2>

As páginas de ambientes auto-hospedados são organizadas pelo que você está fazendo:

* [Guia de início rápido](/docs/pt/self-hosted-environments-quickstart): instale Claude Code, crie um ambiente, inicie um runner e roteie sua primeira sessão
* [Implantar em produção](/docs/pt/self-hosted-environments-deploy): endurecimento de segurança, saída de rede, credenciais git, receitas Kubernetes e Compose, problemas conhecidos e solução de problemas
* [Personalizar sessões](/docs/pt/self-hosted-environments-configuration): scripts de wrapper para credenciais por sessão, hooks de ciclo de vida, runners sob demanda, servidores MCP e permissões
* [Testar de ponta a ponta](/docs/pt/self-hosted-environments-testing): um teste de fumaça de CI que verifica uma imagem de runner antes de promovê-la
* [Referência](/docs/pt/self-hosted-environments-reference): cada flag de CLI, variável de ambiente, métrica e o endpoint de saúde
* [Verificar identidade da sessão](/docs/pt/self-hosted-environments-identity): valide o token de sessão de seus próprios serviços antes de conceder acesso
