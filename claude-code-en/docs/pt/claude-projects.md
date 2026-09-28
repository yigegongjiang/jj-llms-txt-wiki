> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Deixe Claude coordenar trabalho contínuo com Projects

> Dê a Claude um corpo de trabalho relacionado em uma conversa e deixe-o coordenar sessões em nuvem paralelas que compartilham repositórios, instruções e memória.

<Note>
  Projects estão em beta público nos planos Pro e Max e estão sendo implementados gradualmente, começando com contas que usaram [sessões em nuvem](/docs/pt/claude-code-on-the-web) e não têm projetos existentes no chat claude.ai ou Cowork. Ainda não estão disponíveis nos planos Team ou Enterprise. Se **Projects** não aparecer na barra lateral em [claude.ai/code](https://claude.ai/code) ou na aba Code do [aplicativo desktop](/docs/pt/desktop), a implementação ainda não chegou à sua conta, e você pode [entrar na lista de espera](https://claude.com/form/projects). [Executar agentes em paralelo](/docs/pt/agents) lista o que você pode usar enquanto isso.
</Note>

Um projeto é uma conversa contínua onde Claude coordena um fluxo de trabalho relacionado para você. Você diz o que precisa ser feito e ele inicia uma thread para cada tarefa.

Cada thread é geralmente uma [sessão em nuvem](/docs/pt/claude-code-on-the-web): Claude Code executando na nuvem em vez de na sua máquina. Quando uma tarefa precisa de algo que apenas seu computador tem, você pode pedir a Claude para executar essa thread no seu computador em vez disso através do [Remote Control](/docs/pt/remote-control). As threads são executadas em paralelo e você pode verificá-las e direcioná-las do seu telefone. As threads em nuvem continuam funcionando depois que você fecha o laptop.

Sem um projeto, executar várias sessões significa fazer a coordenação você mesmo: você decide no que cada uma trabalha, repete o mesmo contexto no início de cada uma e verifica qual terminou ou precisa de uma resposta. Com um projeto, você:

* **Envia trabalho para um único lugar**: cole um relatório de bug, um rastreamento de pilha ou uma lista de tarefas na conversa sempre que surgir. Claude inicia uma thread para cada peça de trabalho ou a passa para a thread já trabalhando nessa área, e responde perguntas rápidas no local.
* **Define o contexto uma vez**: cada nova thread começa com as instruções do projeto, então uma regra que você declara uma vez, como qual branch direcionar, alcança todas elas.
* **Saia e volte para o trabalho concluído**: quando você voltar uma hora depois ou na manhã seguinte, o painel **Overview** mostra quais threads terminaram, quais pull requests estão prontos para revisão e qual thread está aguardando sua resposta.

Se você já sabe o trabalho que deseja que um projeto execute, vá direto para [Criar um projeto](#create-a-project).

<h2 id="when-to-use-a-project">
  Quando usar um projeto
</h2>

Um projeto vale a pena criar quando o trabalho tem um objetivo que dura mais de uma sessão e continua produzindo tarefas. Esses tipos de trabalho funcionam bem em um projeto:

* **Um objetivo em muitos repositórios**: "Trazer cada serviço para a nova configuração de lint." Claude pode executar uma thread por repositório, cada uma com seu próprio pull request, e o painel [**Overview**](#see-what-needs-you-in-overview) mostra quais estão prontos para revisão.
* **Uma área que você continua alimentando**: os bugs, rastreamentos de pilha e solicitações de revisão para um serviço, colados na conversa conforme chegam até você. Uma armadilha que você diz a Claude para lembrar após uma correção está na [memória do projeto](#give-a-project-standing-context) para a próxima.
* **Uma compilação ou migração maior que uma sessão**: "Construir o que `docs/spec.md` descreve" ou "Mover o aplicativo do ORM descontinuado." O trabalho se divide em threads que cada uma pega uma parte, decisões que você pede a Claude para lembrar no início chegam às threads posteriores, e a especificação muda e bugs que você encontra durante a compilação vão para a mesma conversa.
* **Trabalho que não é código**: uma pasta de contratos ou uma exportação de ticket de suporte que você continua voltando com novas perguntas, como "encontre os dez erros de integração mais comuns nesses tickets." Carregue os documentos em vez de adicionar um repositório, e as threads entregam cada relatório como um arquivo na aba [**Library**](#see-what-needs-you-in-overview) do projeto.

Em qualquer um deles você pode enviar um lote de tarefas, dizer a Claude para começar sem pedir que você confirme, sair e encontrar as threads que precisam de você em [**Waiting on you**](#see-what-needs-you-in-overview) quando voltar, ou pedir a Claude para colocar parte do trabalho em um cronograma como uma [routine](/docs/pt/routines). Se uma dessas é sua situação, [crie um projeto](#create-a-project).

<h3 id="when-something-else-fits-better">
  Quando algo mais se encaixa melhor
</h3>

As threads funcionam em repositórios GitHub e nos arquivos, pastas e pastas do Google Drive que você carrega no projeto, não em arquivos ou ferramentas que existem apenas na sua máquina. Se uma tarefa precisa de sua máquina, peça a Claude para executar sua thread lá através de [Remote Control](/docs/pt/remote-control). [Limitações](#limitations) lista o que isso precisa. Algo mais se encaixa melhor nestes casos:

* **Uma tarefa que cabe em uma sessão**: "Corrigir o teste de login instável." Inicie uma [sessão em nuvem](/docs/pt/claude-code-on-the-web) você mesmo.
* **Trabalho onde cada tarefa precisa de sua máquina**: um banco de dados local, um emulador de dispositivo, ou uma API atrás de sua VPN. Use uma sessão local, ou [agent view](/docs/pt/agent-view) para executar várias de uma vez. Se o trabalho só precisa de arquivos locais, carregue-os no projeto.
* **Uma tarefa que se repete em um cronograma sem conversa ao redor**: "Postar um relatório de dependência toda segunda-feira." Crie uma [routine](/docs/pt/routines) por conta própria.
* **Várias pessoas dando trabalho a Claude e direcionando-o juntas em um canal Slack**: veja [Claude Tag](https://claude.com/docs/claude-tag/overview).

Um projeto usa os mesmos limites de plano que suas outras sessões Claude Code e os usa mais rapidamente. [Uso e custo](#usage-and-cost) cobre o que usa seu plano e como mantê-lo baixo.

<h2 id="how-a-project-is-organized">
  Como um projeto é organizado
</h2>

Um projeto é uma conversa coordenadora com Claude mais as threads que ele inicia para fazer o trabalho. Estas são suas partes:

* **A conversa do projeto**: uma sessão de longa duração onde Claude atua como coordenador. Ele pega o que você envia, decide o que se torna uma thread e acompanha cada thread que iniciou. Ele vê o que as threads relatam, não cada passo que elas dão.
* **Threads**: os trabalhadores. Cada uma é uma sessão separada com sua própria janela de contexto que faz uma peça de trabalho e relata de volta à conversa quando termina. Uma thread em nuvem trabalha em seu próprio branch e abre um pull request quando o trabalho exigir um.
* **O que cada thread em nuvem começa com**:
  * Os repositórios e arquivos do projeto, mais suas [instruções e memória](#give-a-project-standing-context)
  * O `CLAUDE.md` e skills em [cada um dos repositórios do projeto](#what-threads-pick-up-from-your-repositories), e em um projeto com um repositório, as regras de permissão e hooks desse repositório também
  * Os [connectors](#get-skills-plugins-connectors-and-tools-into-threads) na sua conta claude.ai
  * Um [ambiente em nuvem](#choose-an-environment-for-threads) que define seu acesso à rede, variáveis de ambiente, credenciais de API e ferramentas instaladas
* **O painel Overview**: onde você [vê todas as threads de uma vez](#see-what-needs-you-in-overview) e quais delas precisam de você. Suas outras abas são **Library** para os arquivos que você adicionou e os arquivos que as threads produziram, **Pull requests** para os que as threads abriram, e **Routines** para trabalho agendado no projeto.

As threads em nuvem não pegam nada da configuração Claude Code na sua própria máquina. [Obter skills, plugins, connectors e ferramentas em threads](#get-skills-plugins-connectors-and-tools-into-threads) cobre como dar a elas o que de outra forma estariam faltando.

Aqui está como essas partes se conectam, de você através da conversa para as threads fazendo o trabalho, com **Overview** rastreando seu estado:

<Frame>
  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=dbf446f69f0bbdb9961d21af207cb93b" className="dark:hidden" alt="Diagrama de um projeto. Você escreve na conversa do projeto, onde Claude responde ou inicia uma thread. Cada thread em nuvem trabalha em seu próprio branch e pull request. O painel Overview lista threads por estado, como pronto para revisão, aguardando você e trabalhando." width="600" height="250" data-path="images/claude-projects-overview.svg" />

  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview-dark.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=549a5ba9fea8433729babc37a1f6e9c8" className="hidden dark:block" alt="Diagrama de um projeto. Você escreve na conversa do projeto, onde Claude responde ou inicia uma thread. Cada thread em nuvem trabalha em seu próprio branch e pull request. O painel Overview lista threads por estado, como pronto para revisão, aguardando você e trabalhando." width="600" height="250" data-path="images/claude-projects-overview-dark.svg" />
</Frame>

<h2 id="create-a-project">
  Criar um projeto
</h2>

Você cria e usa projects em [claude.ai/code](https://claude.ai/code), na aba Code do aplicativo desktop, ou no aplicativo móvel Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). No navegador e no aplicativo desktop existem duas maneiras de iniciar um projeto:

* **Do zero**, quando você sabe o fluxo de trabalho que deseja que Claude execute: abra o diálogo **New project** e nomeie-o. [Iniciar um novo projeto do zero](#start-a-new-project-from-scratch) percorre o diálogo.
* **De uma sessão em nuvem que já está fazendo o trabalho**: escolha **Continue as a project** no menu dessa sessão, e Claude propõe a configuração do projeto a partir do que a sessão estava fazendo. Veja [Iniciar a partir de uma sessão em nuvem existente](#start-from-an-existing-cloud-session).

De qualquer forma, [verifique os pré-requisitos](#check-the-prerequisites) primeiro.

<h3 id="check-the-prerequisites">
  Verificar os pré-requisitos
</h3>

Antes de criar um projeto, verifique seu plano, sua configuração do GitHub e o que o trabalho precisa alcançar:

* **Plano**: você está no Pro ou Max e **Projects** aparece na sua barra lateral.
* **GitHub, se o projeto funcionará em código**: seu código está em github.com em vez de GitHub Enterprise Server, GitLab ou Bitbucket, sua conta GitHub conectada tem acesso push a ele, e o Claude GitHub App está instalado nele. Se você conectou GitHub com [`/web-setup`](/docs/pt/web-quickstart#connect-from-your-terminal), esse token permite que suas outras sessões em nuvem alcancem um repositório, mas não é suficiente para threads de projeto, que precisam do Claude GitHub App. [Configurar acesso ao GitHub](#set-up-github-access) tem os passos.
* **Rede, credenciais e ferramentas**: para threads em nuvem, estas vêm do [ambiente em nuvem](#choose-an-environment-for-threads) do projeto. O ambiente padrão já alcança [registros de pacotes comuns](/docs/pt/cloud-environments#default-allowed-domains), então verifique isso apenas se o trabalho precisar de outros domínios, um segredo ou uma ferramenta que não está pré-instalada. Se o trabalho precisa de um servidor MCP, verifique se ele aparece como conectado em seus [connectors claude.ai](https://claude.ai/customize/connectors).

<h3 id="start-a-new-project-from-scratch">
  Iniciar um novo projeto do zero
</h3>

Iniciar um projeto do zero significa abrir o diálogo **New project**, nomear o fluxo de trabalho e opcionalmente dar a ele um objetivo e os repositórios e arquivos em que funciona. Apenas o nome é obrigatório, então você pode criar o projeto primeiro e preencher o resto conforme o trabalho toma forma.

<Steps>
  <Step title="Abrir Projects">
    Em [claude.ai/code](https://claude.ai/code) ou na aba Code do aplicativo desktop, selecione **Projects** na barra lateral esquerda e depois selecione **New project**. Em um navegador você também pode ir direto para [claude.ai/code/projects/browse](https://claude.ai/code/projects/browse).
  </Step>

  <Step title="Preencher o diálogo New project">
    Escopo do projeto para um fluxo de trabalho que você continuará adicionando, como tudo o que é necessário para manter uma API sob seu alvo de latência. [Quando usar um projeto](#when-to-use-a-project) tem mais exemplos. Então preencha os campos do diálogo:

    * **Name**: como o projeto aparece na lista **Projects**.
    * **Goal** (opcional): uma linha do que você está tentando realizar, como "Manter latência p95 da API abaixo de 200 ms". Claude na conversa trabalha em direção a isso. Sem um objetivo, Claude trabalha a partir das tarefas que você envia, e você pode adicionar um objetivo mais tarde em **Project settings > General**.
    * **Context** (opcional): os repositórios GitHub em que este projeto funciona, mais quaisquer arquivos, pastas ou pastas do Google Drive que as threads devem ler. Clique **Add** para cada um. Adicione os repositórios que a maioria das tarefas precisa em vez de cada um que o trabalho pode tocar; [Decidir quais repositórios adicionar](#decide-which-repositories-to-add) cobre a escolha, e você pode adicionar mais tarde em **Project settings > Environment**.

    Regras permanentes para como as threads devem funcionar vão em [instruções do projeto](#give-a-project-standing-context), que você define após o projeto existir.
  </Step>

  <Step title="Criar o projeto">
    Clique **Create project**. A conversa do projeto abre com uma caixa de mensagem na parte inferior, onde você descreve trabalho para Claude.

    No seu primeiro projeto, Claude toma uma volta por conta própria assim que o projeto é criado, a menos que você envie uma mensagem primeiro. Essa volta usa seu plano. Nela, Claude pode:

    * Iniciar uma thread que explora o repositório sem fazer alterações e propõe próximos passos, se o projeto tiver um repositório que ele possa ler.
    * Postar **Setup recommendations** extraídas de suas sessões em nuvem recentes: repositórios para adicionar, routines para criar e threads que ele poderia iniciar. Cada repositório e routine recomendados começam ligados. Desligue os que você não quer, depois clique **Update setup** para adicionar o resto, ou ignore as recomendações e descreva o trabalho você mesmo.
  </Step>
</Steps>

O projeto agora está listado em **Projects** na barra lateral, e sua conversa está aberta. [Seu primeiro lote](#your-first-batch) cobre o que configurar antes de enviar trabalho a ele.

<h3 id="start-from-an-existing-cloud-session">
  Iniciar a partir de uma sessão em nuvem existente
</h3>

Se você já tem uma sessão em nuvem fazendo trabalho que pertence a um projeto, abra o menu da sessão na barra lateral e escolha **Continue as a project** ou **Move to project**:

* **Continue as a project** cria um novo projeto nomeado após a sessão e o abre. Claude lê a sessão e posta **Setup recommendations** na conversa para você confirmar. A sessão original permanece na sua lista de sessões, e se estava no meio de uma volta ela continua funcionando, então pare-a você mesmo se não quiser que ambas funcionem ao mesmo tempo. Se você usar o banner **Set up project** que pode aparecer acima da caixa de mensagem da sessão em nuvem, o resultado é o mesmo, exceto que a volta em execução da sessão para uma vez que o projeto abre.
* **Move to project** traz o trabalho da sessão para um projeto existente. Ele posta uma mensagem na conversa desse projeto pedindo a Claude para ler a sessão e continuar de onde parou, e o novo trabalho continua nas próprias threads do projeto. A sessão original permanece na sua lista de sessões, inalterada.

<h3 id="set-up-github-access">
  Configurar acesso ao GitHub
</h3>

A maioria da configuração do GitHub acontece uma vez, não por projeto. Você conecta sua conta GitHub a Claude uma vez, e o Claude GitHub App é instalado uma vez por repositório, ou uma vez para toda uma organização GitHub se você der a ela todos os repositórios. Você volta a essas etapas quando adiciona um repositório que o Claude GitHub App ainda não cobre ou um em uma organização GitHub que impõe SSO.

<Steps>
  <Step title="Conectar sua conta GitHub">
    Se você nunca usou claude.ai/code antes, sua primeira visita o orienta através da conexão do GitHub; veja [Conectar GitHub](/docs/pt/web-quickstart#connect-github). Caso contrário, use uma das [opções de autenticação do GitHub](/docs/pt/claude-code-on-the-web#github-authentication-options).
  </Step>

  <Step title="Instalar o Claude GitHub App nos repositórios do projeto">
    Instale o [Claude GitHub App](https://github.com/apps/claude) e conceda a ele os repositórios que o projeto usará. Em um repositório pertencente a uma organização GitHub, apenas um proprietário da organização pode concluir a instalação; se você não for um, o GitHub envia ao proprietário uma solicitação de instalação e o projeto não pode usar o repositório até que ele aprove.
  </Step>

  <Step title="Autorizar SSO para organizações que o impõem">
    Se uma organização GitHub impõe SAML SSO, reconecte GitHub e autorize o aplicativo Claude para essa organização. Até que você faça isso, os repositórios privados dessa organização não aparecem no diálogo **New project** ou **Project settings > Environment**.
  </Step>
</Steps>

Quando uma dessas etapas está incompleta, o diálogo **New project** e a página do projeto nomeiam a etapa ausente e vinculam a onde você a conclui. Conclua a etapa lá, depois clique **Check again** se o diálogo oferecer. Se um repositório ainda estiver faltando na lista depois, abra a instalação do Claude GitHub App no GitHub, em [github.com/settings/installations](https://github.com/settings/installations) para uma conta pessoal, e confirme que o repositório está listado em **Repository access**. Para as mensagens de erro que uma thread ou o projeto relata quando o acesso ainda está errado, veja [Erros de acesso ao repositório](#repository-access-errors).

<h2 id="work-in-a-project">
  Trabalhar em um projeto
</h2>

Dê trabalho a Claude através da conversa do projeto: tarefas uma de cada vez ou várias de uma vez, mais atualizações e pensamentos soltos conforme surgem. Claude roteia cada mensagem, e as threads fazem o trabalho e relatam de volta.

<h3 id="your-first-batch">
  Seu primeiro lote
</h3>

Antes de enviar a um novo projeto um lote de trabalho, configure-o para que as primeiras threads voltem da maneira que você quer:

1. [Escrever instruções do projeto](#write-project-instructions): o resumo que cada thread começa, como qual branch direcionar, como uma thread verifica seu trabalho e o que precisa de sua aprovação.
2. Envie uma pequena peça do trabalho real, ou inicie uma das threads que Claude sugeriu, e abra a thread quando terminar para ver como ela relata de volta e o que fez em seu branch. Se ela assumiu algo errado ou não conseguiu alcançar o que precisava, [Threads adivinharam ou travaram em vez de perguntar](#threads-guessed-or-stalled-instead-of-asking) cobre onde corrigir isso.
3. Verifique **Thread model** e **Thread effort** em **Project settings > General**. Um novo projeto executa cada thread em Opus com alto esforço, que usa seu plano mais rapidamente; [Escolher modelos e deixar Claude gerenciar contexto](#choose-models-and-let-claude-manage-context) cobre as alternativas.
4. Peça a Claude para [propor threads antes de iniciá-las e executar algumas de cada vez](#tune-how-claude-runs-a-project), e solte esses limites uma vez que algumas threads voltem da maneira que você quer.

<h3 id="send-work-and-read-results">
  Enviar trabalho e ler resultados
</h3>

Claude decide para onde cada mensagem que você envia na conversa vai:

* Uma pergunta rápida geralmente recebe uma resposta na conversa.
* Novo trabalho vai para uma nova thread ou para uma thread já trabalhando nessa área, e Claude diz qual. Cada nova thread aparece sob sua mensagem como um cartão: uma caixa com o título e status da thread, que você clica para abrir a thread.
* Várias tarefas não relacionadas em uma mensagem se tornam threads separadas.

Se Claude rotear algo diferente do que você queria, diga. [Ajustar como Claude executa um projeto](#tune-how-claude-runs-a-project) lista coisas que você pode dizer a ele, como reutilizar uma thread existente para acompanhamentos ou responder no local em vez de iniciar uma thread.

Os resultados completos de uma thread permanecem na thread, e você abre seu cartão na conversa para lê-los. Os arquivos que uma thread produziu também estão na aba **Library** em **Overview**.

Às vezes Claude propõe threads em vez de iniciá-las, em uma lista **Suggested threads**. Clique na seta em uma sugestão para iniciar essa thread. Quando várias estão listadas, um botão sob a lista inicia todas elas.

<h3 id="review-a-thread’s-pull-request">
  Revisar o pull request de uma thread
</h3>

Quando uma thread muda código, é isso que ela faz a menos que você diga o contrário:

* **Branch**: funciona em um novo branch, iniciado a partir do branch padrão do repositório.
* **Pull request**: abre um quando você pede, e pode abrir um por conta própria para uma correção de bug ou outra mudança concreta.
* **Depois que abre**: observa o pull request com [auto-fix](/docs/pt/claude-code-on-the-web#auto-fix-pull-requests) ligado, independentemente de auto-fix estar ligado para suas outras sessões em nuvem. Ele empurra correções quando CI falha, aborda comentários de revisão e responde na thread quando as verificações passam e o pull request está pronto para você.

Quando uma thread empurrou um branch ou abriu um pull request, seu cartão na conversa pode mostrar um botão para o próximo passo:

* **Resolve conflicts**, **Fix CI**, **Address comments** e **Merge it** enviam essa instrução para a thread como uma mensagem sua, então você pode solicitar a thread você mesmo em vez de esperar que ela reaja ao pull request.
* **Review PR** abre o pull request no GitHub.
* **Create PR** aparece quando uma thread ociosa empurrou um branch mas não abriu um pull request. Clicar nele cria o pull request desse branch diretamente em vez de enviar à thread uma instrução para abrir um.

Para mudar quando as threads abrem pull requests, por exemplo apenas quando você pede, ou qual branch elas começam, diga na tarefa ou em [instruções do projeto](#write-project-instructions).

<h3 id="see-what-needs-you-in-overview">
  Ver o que precisa de você em Overview
</h3>

O painel **Overview** ao lado da conversa rastreia as threads do projeto. Ele já está aberto na primeira vez que você abre um novo projeto. O botão **Overview** no cabeçalho do projeto o fecha e reabre, e mostra um ponto quando uma thread está aguardando você.

No aplicativo desktop, você também recebe uma notificação desktop quando Claude posta na conversa, uma thread atinge um erro ou uma thread precisa de sua entrada, então você não precisa manter o projeto aberto para descobrir. Para também receber uma cada vez que uma thread termina uma volta, ou para desligá-las para um projeto, escolha **Notifications** no menu da barra lateral do projeto. Essas notificações são apenas desktop: em um navegador, verifique o ponto no botão **Overview**.

A aba **Threads** do painel agrupa threads por estado:

| Grupo                | O que está nele                                                                                                                                                                                                                             |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Ready for review** | Threads cujo pull request está aberto e aguardando revisão                                                                                                                                                                                  |
| **Waiting on you**   | Threads que precisam de sua resposta ou aprovação, ou que falharam                                                                                                                                                                          |
| **Working**          | Threads ainda em execução                                                                                                                                                                                                                   |
| **Landing**          | Threads cujo pull request é aprovado ou enfileirado para mesclar                                                                                                                                                                            |
| **Idle**             | Threads que terminaram e não estão aguardando nada                                                                                                                                                                                          |
| **Resolved**         | Threads marcadas como concluídas: por você no menu da thread, por Claude uma vez que você tenha tomado o último passo, como mesclar seu pull request, ou automaticamente após uma semana sem atividade. Você pode reabrir uma no mesmo menu |

As outras abas do painel são **Library** para os arquivos e pastas que você adicionou e os arquivos que as threads produziram, **Pull requests** uma vez que as threads abriram algum, e **Routines** para as [routines](/docs/pt/routines) que Claude configurou a partir deste projeto.

<h3 id="open-a-thread-when-you-need-control">
  Abrir uma thread quando você precisa de controle
</h3>

Clique no cartão de uma thread na conversa ou sua linha em **Overview** para abrir sua transcrição no painel Overview. De lá você pode:

* Ler o que Claude fez, passo a passo.
* Direcionar a tarefa escrevendo na caixa de mensagem própria da thread. Uma mensagem lá vai direto para essa thread, enquanto um acompanhamento na conversa do projeto a alcança apenas quando Claude corresponde o acompanhamento a essa thread.
* Responder a um prompt de permissão que a thread está aguardando.
* Interromper a thread com **Stop**, que substitui o botão enviar enquanto a thread está funcionando, ou pressionando Esc.

<h3 id="choose-models-and-let-claude-manage-context">
  Escolher modelos e deixar Claude gerenciar contexto
</h3>

Defina modelos e esforço em **Project settings > General**. Um novo projeto executa Opus em todos os lugares, com alto [esforço](/docs/pt/model-config#adjust-effort-level) para threads e baixo esforço para a conversa:

* **Thread model** e **Thread effort** se aplicam a threads. Para usar um modelo diferente para uma tarefa, peça na tarefa; para uma thread já em execução, use o seletor de modelo dessa thread.
* **Coordinator model** e **Coordinator effort** se aplicam a Claude na conversa do projeto.

Você não gerencia janelas de contexto em um projeto. As threads compactam automaticamente, e a conversa funciona a partir de mensagens recentes, threads recentes e memória do projeto em vez de seu histórico completo, então continua enquanto o projeto funciona. Coloque qualquer coisa que nunca deve ser descartada em [memória do projeto](#give-a-project-standing-context). Se uma thread ultrapassar seu contexto, ela mostra [Claude ficou sem contexto nesta volta](#context-limit).

<h3 id="tune-how-claude-runs-a-project">
  Ajustar como Claude executa um projeto
</h3>

Diga a Claude na conversa quantas threads executar de uma vez, quando postar atualizações e quando abrir pull requests. Se Claude está coordenando de uma maneira que você não quer, diga. Por exemplo, você pode dizer:

* "Proponha threads e aguarde minha aprovação antes de iniciá-las" ou "Inicie estas agora sem me pedir para confirmar"
* "Execute no máximo duas threads de uma vez" ou "Reutilize uma thread existente para acompanhamentos na mesma área"
* "Poste atualizações mais curtas" ou "Apenas poste quando algo terminar ou ficar bloqueado"
* "Dê-me uma atualização de status em cada thread"
* "Faça esta tarefa com um modelo menor"
* "Não abra um pull request até que eu tenha visto o plano"
* "Diga-me o que está errado nesses repositórios e não corrija nada ainda", quando você quer passar pelos achados antes que qualquer um deles se torne uma thread
* "Responda isso aqui em vez de iniciar uma thread", quando Claude inicia uma thread para algo que você quis dizer como uma pergunta rápida

Claude salva preferências como essas em [memória do projeto](#give-a-project-standing-context) por conta própria e as segue em threads posteriores. São instruções que Claude segue, não configurações impostas, então um limite de thread que você dá dessa maneira não é um limite rígido. Adicione um às instruções do projeto quando quiser que seja redigido exatamente e aplicado a cada thread desde o início.

<h3 id="unblock-a-thread-waiting-on-approval">
  Desbloquear uma thread aguardando aprovação
</h3>

As threads são executadas em [modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) quando o modelo da thread o suporta, então a maioria das chamadas de ferramenta são executadas sem pedir a você. Quando uma thread precisa de sua aprovação, o prompt está dentro dessa thread e a thread aguarda até que você responda lá. Dizer a Claude na conversa do projeto para prosseguir não a alcança.

Cada aprovação cobre esse prompt, ou o resto dessa thread se você escolher a opção mais ampla. Para deixar cada thread executar certos comandos sem perguntar, ou para bloquear alguns, adicione [regras de permissão](/docs/pt/permissions) ao `.claude/settings.json` do repositório. As threads as aplicam apenas em um projeto com um repositório; veja [O que as threads pegam de seus repositórios](#what-threads-pick-up-from-your-repositories).

<h2 id="give-a-project-standing-context">
  Dar contexto permanente a um projeto
</h2>

Memória do projeto, instruções do projeto e os repositórios, arquivos e ambiente do projeto carregam contexto entre threads. Você define cada um uma vez.

| Contexto                          | O que carrega                                                                                                                                                                                                                    | Como você define                                                                                                                                                                                                    |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Memória do projeto                | Notas que Claude mantém sobre o projeto, como requisitos, decisões e armadilhas, armazenadas como arquivos. Cada thread em nuvem lê o arquivo de índice `MEMORY.md` quando começa e abre os outros arquivos quando precisa deles | Peça a Claude na conversa do projeto ou em qualquer thread em nuvem para lembrar um requisito, uma decisão ou uma armadilha, ou para esquecer um. Leia, edite e delete os arquivos em **Project settings > Memory** |
| Instruções do projeto             | Texto enviado para cada nova thread e para Claude na conversa do projeto, até 16.000 caracteres. [Escrever instruções do projeto](#write-project-instructions) cobre o que colocar nele                                          | **Project settings > Memory > Project instructions**, ou peça a Claude para mudar as instruções                                                                                                                     |
| Repositórios, arquivos e ambiente | Os repositórios que cada thread em nuvem clona, as pastas e arquivos que ela pode ler em `/mnt/project-files`, e o ambiente em nuvem em que ela é executada                                                                      | Repositórios e ambiente em **Project settings > Environment**, ou peça a Claude na conversa para adicionar um repositório ao projeto. Arquivos e pastas de **Add** na aba **Library** em **Overview**               |

**Project settings > Memory** lista esses arquivos em **Auto memory**, porque Claude os escreve a si mesmo conforme trabalha no projeto. Eles são separados da [memória automática](/docs/pt/memory) que Claude Code mantém na sua máquina, mesmo que ambas usem um índice `MEMORY.md`. A memória do projeto também é separada dos arquivos `CLAUDE.md` nos repositórios do projeto. Cada thread em nuvem ainda lê esses arquivos `CLAUDE.md` de seu clone quando começa, então coloque instruções sobre um repositório em seu `CLAUDE.md` e notas sobre o projeto em memória do projeto.

<h3 id="write-project-instructions">
  Escrever instruções do projeto
</h3>

Instruções do projeto são o resumo que cada nova thread começa. Clique no ícone de engrenagem no cabeçalho do projeto para abrir **Project settings**, depois vá para **Memory > Project instructions**. Um resumo útil cobre:

* Para que serve o projeto
* Onde o trabalho acontece: quais repositórios, qual branch começar, como nomear pull requests
* Como uma thread verifica seu próprio trabalho antes de chamá-lo de concluído
* O que fazer quando algo que precisa está faltando
* O que precisa de sua aprovação primeiro

Por exemplo:

```text theme={null}
Este projeto mantém a latência p95 da API de pagamentos abaixo de 200 ms: criação de perfil, correções de consulta e cache, e as atualizações de dependência que vêm com elas, no repositório payments-api.

- Ramifique a partir de main e abra um pull request de rascunho por thread.
- Antes de chamar o trabalho de concluído, execute `make test` e `make lint` e cole as linhas de resumo em sua mensagem final.
- Se você não conseguir alcançar algo que precisa, como um repositório, um segredo, uma API ou um connector, diga exatamente o que está faltando em sua primeira mensagem e pare. Não substitua, simule ou adivinhe.
- Não mescle, force-push ou mude a configuração de CI sem me perguntar na thread.
```

Regras sobre um repositório, como seus comandos de compilação, pertencem ao `CLAUDE.md` desse repositório, que cada thread em nuvem lê quando o repositório faz parte do projeto. Uma vez que o trabalho está em andamento, quando você corrige uma thread, também diga a Claude para lembrar da correção: ela vai para [memória do projeto](#give-a-project-standing-context) e threads posteriores começam com ela.

<h3 id="decide-which-repositories-to-add">
  Decidir quais repositórios adicionar
</h3>

Os repositórios que você adiciona a um projeto vêm com tudo neles, seu código, `CLAUDE.md` e skills, em cada thread em nuvem. Repositórios que você não adiciona ainda estão ao alcance: uma thread em nuvem pode adicionar um a si mesma quando sua tarefa precisa. A maioria dos projetos usa ambos:

* **Adicione-o ao projeto**, no diálogo **New project**, em **Project settings > Environment**, ou pedindo a Claude na conversa para adicionar ao projeto. Cada thread a partir de então clona e começa com seu `CLAUDE.md` e skills carregados, independentemente de a tarefa tocá-lo. Ir de um repositório para vários também muda o que as threads pegam do `.claude/settings.json` de cada repositório; veja [O que as threads pegam de seus repositórios](#what-threads-pick-up-from-your-repositories).
* **Deixe-o de fora e deixe as threads adicionarem quando necessário.** Uma thread em nuvem cuja tarefa precisa de um repositório que o projeto não tem pode adicioná-lo a si mesma, e uma nota na thread diz que foi adicionado apenas a essa thread. O clone acontece no meio da tarefa, então o `CLAUDE.md` e skills desse repositório não estavam lá quando a thread começou. A próxima thread começa sem ele novamente. Um repositório que uma thread adiciona precisa dos mesmos [pré-requisitos](#check-the-prerequisites) que um repositório de projeto: o Claude GitHub App instalado nele e acesso push de sua conta GitHub.

Um projeto não precisa de um repositório. Suas threads em nuvem ainda podem pesquisar, escrever documentos e escrever e executar código em seu próprio sandbox, e entregam arquivos à aba **Library**. Qualquer uma de suas threads em nuvem ainda pode adicionar um repositório a si mesma quando uma tarefa exigir.

Uma vez que o projeto tem repositórios, Claude só pode adicionar repositórios de um proprietário GitHub que o projeto já usa, seja adicionando um ao projeto ou uma thread adicionando um a si mesma. Para trazer um repositório de um proprietário diferente, adicione-o ao projeto você mesmo em **Project settings > Environment**.

Para um projeto que abrange muitos repositórios, como um recurso com código de servidor, web, mobile e desktop, adicione o um ou dois repositórios que quase cada tarefa toca e nomeie os outros em [instruções do projeto](#write-project-instructions) para que Claude saiba onde o resto do código vive. As threads em nuvem então começam pequenas e puxam os outros repositórios apenas para as tarefas que precisam deles.

<h3 id="what-threads-pick-up-from-your-repositories">
  O que as threads pegam de seus repositórios
</h3>

Cada thread em nuvem clona cada repositório no projeto e carrega `CLAUDE.md` e skills de todos eles. Regras de permissão, hooks e `env` vêm apenas do `.claude/settings.json` no diretório em que a thread começa: dentro do repositório quando o projeto tem um, e acima dos clones quando tem vários, onde nenhum arquivo de repositório é lido para eles.

| Em cada repositório                                                     | Um repositório                                                                                                                            | Vários repositórios                                                             |
| :---------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------ |
| `CLAUDE.md`                                                             | Carregado quando a thread começa                                                                                                          | Carregado de cada repositório quando a thread começa                            |
| Skills, agentes e comandos em `.claude/`                                | Carregado                                                                                                                                 | Carregado de cada repositório                                                   |
| Plugins habilitados em `.claude/settings.json`                          | Não carregado. Adicione o plugin em **Project settings > Plugins** em vez disso                                                           | Não carregado. Adicione o plugin em **Project settings > Plugins** em vez disso |
| Regras de permissão, hooks e `env` definidos em `.claude/settings.json` | Aplicam-se à thread, exceto as chaves `env` que [nenhuma sessão em nuvem honra](/docs/pt/cloud-environments#what-carries-over-from-your-setup) | Não se aplicam                                                                  |

Em um projeto com vários repositórios, cada clone é anexado à thread como um [diretório adicional](/docs/pt/memory#load-from-additional-directories) com carregamento de `CLAUDE.md` ligado, é por isso que o `CLAUDE.md` e skills de cada repositório carregam no início mesmo que a thread comece acima deles. Em tal projeto, coloque regras permanentes em instruções do projeto e dê às threads variáveis de ambiente através do [ambiente em nuvem](#choose-an-environment-for-threads).

<h3 id="choose-an-environment-for-threads">
  Escolher um ambiente para threads
</h3>

Cada nova thread em nuvem começa no [ambiente em nuvem](/docs/pt/cloud-environments) do projeto. O ambiente define quais domínios as threads podem alcançar, quais variáveis de ambiente elas têm, quais credenciais de API são adicionadas a suas solicitações e o que o script de configuração instala antes de Claude começar. As threads em nuvem usam um ambiente padrão hospedado pela Anthropic até que você escolha um em **Project settings > Environment**.

Se as threads em nuvem precisam alcançar uma API interna ou um registro de pacotes privado, ou precisam de um token que sua máquina normalmente mantém, mude o ambiente em vez do projeto: veja [Acesso à rede](/docs/pt/cloud-environments#network-access), [Adicionar credenciais de API](/docs/pt/cloud-environments#add-api-credentials) e [Scripts de configuração](/docs/pt/cloud-environments#setup-scripts).

<h3 id="get-skills-plugins-connectors-and-tools-into-threads">
  Obter skills, plugins, connectors e ferramentas em threads
</h3>

As threads em nuvem não têm os skills, servidores MCP, plugins e ferramentas instalados apenas na sua máquina. Uma thread que Claude executa na sua máquina através de [Remote Control](/docs/pt/remote-control) usa o que está instalado lá. Para disponibilizar cada um desses para threads em nuvem:

* Skills, subagentes e comandos: confirme-os em um repositório que você adicionou ao projeto, por exemplo um skill em `.claude/skills/<skill-name>/SKILL.md`. Cada thread em nuvem clona cada repositório no projeto e carrega `.claude/skills/`, `.claude/agents/` e `.claude/commands/` de cada um deles, então um skill confirmado em um repositório está disponível em cada nova thread em nuvem. As threads em nuvem também carregam os skills que você habilita para sua conta claude.ai.
* Plugins: adicione-os em **Project settings > Plugins**; eles carregam em cada nova thread em nuvem. Plugins que um repositório declara em seu `.claude/settings.json` [não carregam em threads em nuvem](/docs/pt/cloud-environments#what-carries-over-from-your-setup).
* Servidores MCP: as threads em nuvem obtêm suas ferramentas MCP dos connectors em sua conta claude.ai, que são servidores MCP que você conecta uma vez em [claude.ai/customize/connectors](https://claude.ai/customize/connectors) ou através do link **Manage connectors** em **Project settings > Environment**. Cada thread em nuvem pode usar todos eles sem configuração por projeto. A conversa do projeto em si não tem connectors, então envie trabalho que precisa de um como uma tarefa para uma thread em nuvem. Em um projeto com um repositório, as threads em nuvem também carregam servidores MCP do [`.mcp.json`](/docs/pt/cloud-environments#what-carries-over-from-your-setup) desse repositório. [Como connectors alcançam Claude Code](/docs/pt/mcp#how-connectors-reach-claude-code) lista as regras para sessões em nuvem e as configurações que desligam connectors.
* Ferramentas de linha de comando e pacotes: instale-os no [script de configuração](/docs/pt/cloud-environments#setup-scripts) do ambiente.

Para ver quais connectors uma thread em nuvem em execução tem em claude.ai/code, abra a thread e selecione **Connectors** no menu **+** ao lado de sua caixa de mensagem. Desligar um connector lá o remove dessa thread e salva isso como seu padrão de conta, então novas threads em nuvem e chats claude.ai começam sem ele até que você o ligue novamente. Uma thread em nuvem pega um connector que você adiciona ou reconecta após a próxima mensagem que você envia a ela.

<h2 id="project-settings-reference">
  Referência de configurações do projeto
</h2>

Você muda as configurações do projeto em claude.ai/code ou no aplicativo desktop, não em `settings.json`. Abra **Project settings** de **Settings** no menu da barra lateral do projeto ou do ícone de engrenagem no cabeçalho do projeto.

As configurações são salvas conforme você as altera; um campo de texto que você está editando, como o objetivo ou instruções, mostra **Save changes** e **Discard** até que você o deixe. Mudanças em instruções, repositórios, plugins e ambiente em **Project settings** alcançam novas threads, não threads já em execução.

| Configuração                    | Seção       | O que controla                                                                                                      |
| :------------------------------ | :---------- | :------------------------------------------------------------------------------------------------------------------ |
| Nome, ícone e objetivo          | General     | O nome e ícone do projeto na barra lateral e seu objetivo de uma linha                                              |
| Modelo e esforço do coordenador | General     | O modelo e [nível de esforço](/docs/pt/model-config#adjust-effort-level) para Claude na conversa do projeto              |
| Modelo e esforço da thread      | General     | O modelo e nível de esforço para threads                                                                            |
| Instruções do projeto           | Memory      | [Regras permanentes](#give-a-project-standing-context) que cada nova thread recebe                                  |
| Repositórios do projeto         | Environment | Os repositórios que novas threads clonam                                                                            |
| Ambiente em nuvem               | Environment | O [ambiente em nuvem](#choose-an-environment-for-threads) em que novas threads são executadas                       |
| Connectors                      | Environment | Um link para gerenciar os connectors claude.ai que as threads obtêm                                                 |
| Plugins                         | Plugins     | Os plugins que carregam em cada nova thread                                                                         |
| Usage                           | Usage       | [Uso de token](#usage-and-cost) por thread e por modelo                                                             |
| Memory                          | Memory      | Os [arquivos de memória](#give-a-project-standing-context) do projeto                                               |
| Restart Claude                  | General     | Reinicia a conversa do projeto quando [Claude para de responder lá](#claude-hasnt-responded)                        |
| Pause, Archive, Delete          | General     | Para, oculta ou remove o projeto; veja [Pausar, arquivar ou deletar um projeto](#pause-archive-or-delete-a-project) |

<h3 id="pause-archive-or-delete-a-project">
  Pausar, arquivar ou deletar um projeto
</h3>

Todos os três controles estão na parte inferior de **Project settings > General**:

* **Pause**: para tudo de uma vez. Cada thread em execução e a conversa são interrompidas, nenhuma nova thread começa, routines não são executadas e o projeto não aceita mensagens até que você o retome. Clique **Resume** no mesmo lugar ou no banner acima da caixa de mensagem do projeto; uma thread pausada continua quando você envia uma mensagem a ela depois disso.
* **Archive**: oculta o projeto da barra lateral e arquiva suas threads, o que para qualquer thread que estava em execução ou observando um pull request. Routines no projeto não são executadas enquanto está arquivado. Para trazer o projeto de volta, abra-o na página Projects e clique **Unarchive**. Suas threads permanecem arquivadas até que você as desarquive individualmente da lista de sessões.
* **Delete**: remove permanentemente o projeto junto com suas threads, sua memória e seus arquivos, e desliga as routines do projeto. Isso não pode ser desfeito. Branches e pull requests que as threads empurraram para o GitHub não são afetados.

<h2 id="usage-and-cost">
  Uso e custo
</h2>

O uso do projeto conta contra os mesmos [limites de plano](/docs/pt/errors#youve-hit-your-session-limit) que suas outras sessões Claude Code, e um projeto não pode gastar além desses limites por conta própria.

Uma thread que atinge o limite do seu plano aguarda e continua por conta própria quando o limite é redefinido, então o trabalho que você deixou em execução começa a usar sua próxima janela de uso sem uma mensagem sua. [Uma thread atingiu o limite de uso](#usage-limit-reached) cobre o que você vê, como pará-la e o único caso que não aguarda.

O trabalho vai além dos limites do seu plano apenas se você tiver ligado [créditos de uso](/docs/pt/costs#add-usage-credits-to-your-subscription) para sua conta. Uma thread não pode ligá-los para você.

<h3 id="what-draws-on-your-plan">
  O que usa seu plano
</h3>

Um projeto usa seus limites mais rapidamente que uma única sessão, e em um plano Pro em particular você deve esperar atingir seu limite mais cedo em dias em que executa um. Estas são as partes de um projeto que usam seu plano:

* **Threads em execução**: cada uma é uma sessão completa, e várias podem ser executadas ao mesmo tempo. Não há um número fixo; Claude inicia quantas o trabalho exigir, e um limite que você [pede](#tune-how-claude-runs-a-project) é uma preferência em vez de um limite. O limite imposto é 200 novas threads por dia em seus projetos.
* **A conversa**: Claude usa tokens lendo o que as threads relatam e decidindo o que fazer a seguir.
* **Threads observando um pull request**: uma thread ociosa acorda e usa seu plano novamente quando CI falha ou um comentário de revisão chega em seu pull request. Para parar isso, peça na thread para ela parar de observar o pull request.

Um projeto sem threads em execução, sem pull requests observados e sem novas mensagens não usa seu plano enquanto fica ocioso, e nem um projeto arquivado.

<h3 id="see-and-reduce-a-project’s-usage">
  Ver e reduzir o uso de um projeto
</h3>

Abra **Usage** em **Project settings** para ver o uso de token por thread e por modelo, e quanto foi para a conversa do projeto. Para reduzi-lo:

* Um acompanhamento roteado para uma thread que está ociosa há mais tempo que o [tempo de vida do cache](/docs/pt/prompt-caching#cache-lifetime), uma hora em Pro e Max dentro dos limites do seu plano, relê toda a conversa dessa thread antes de fazer qualquer coisa. Para novo trabalho, pedir a Claude para iniciar uma thread fresca pode usar menos que reviver uma grande antiga.
* Para trabalho que não precisa do maior modelo, [escolha um modelo menor ou um nível de esforço mais baixo](#choose-models-and-let-claude-manage-context) para threads, a conversa ou ambos.
* Peça a Claude na conversa do projeto para executar menos threads de uma vez, ou para responder pequenas perguntas ela mesma em vez de iniciar uma thread.

<h2 id="how-projects-relate-to-other-claude-code-features">
  Como projetos se relacionam com outros recursos Claude Code
</h2>

Vários recursos Claude Code permitem que mais de uma sessão funcione ao mesmo tempo, então executar trabalho em paralelo não é por si só para que um projeto serve. Em um projeto, Claude inicia e rastreia as sessões em vez de você, e cada uma começa a partir das mesmas instruções. É assim que cada recurso vizinho se conecta a um projeto:

* **Claude Tag**: [Claude Tag](https://claude.com/docs/claude-tag/overview) é Claude nos canais Slack da sua equipe, em planos Team e Enterprise. Qualquer pessoa em um canal pode dar trabalho a ele, todos no canal veem e o direcionam, e usa conexões que um admin configurou para esse canal. Um projeto é seu: você é o único que envia trabalho a ele ou vê suas threads, usa seu próprio acesso GitHub e connectors, e está em Pro e Max. [Como Claude Tag difere de Cowork e Claude Code](https://claude.com/docs/claude-tag/concepts/how-it-works#how-claude-tag-differs-from-cowork-and-claude-code) tem o lado a lado.
* **Sessões em nuvem**: cada thread é uma [sessão em nuvem](/docs/pt/claude-code-on-the-web), a menos que você peça a Claude para executá-la na sua máquina. De qualquer forma, Claude inicia e rastreia em vez de você. Uma sessão em nuvem que você iniciou pode se tornar um projeto ou alimentar um através de [**Continue as a project** ou **Move to project**](#start-from-an-existing-cloud-session).
* **Routines**: quando você pede trabalho agendado em um projeto, Claude cria uma [routine](/docs/pt/routines) que é executada como threads nesse projeto e aparece em sua aba **Routines**. Routines que você cria fora de um projeto continuam funcionando por conta própria.
* **Sessões locais e agent view**: uma sessão que você inicia na sua máquina em seu terminal, IDE ou no ambiente local do aplicativo desktop não pode ser adicionada a um projeto. Um projeto alcança sua máquina apenas executando uma thread lá através de [Remote Control](/docs/pt/remote-control). [Agent view](/docs/pt/agent-view) é uma tela para rastrear várias sessões locais que você iniciou; não tem coordenador.
* **Worktrees**: um [worktree](/docs/pt/worktrees) dá a cada sessão local sua própria cópia de trabalho de um repositório para que sessões paralelas na sua máquina não se sobrescrevam. As threads em nuvem não precisam deles: cada uma clona seus repositórios em seu próprio sandbox em nuvem e funciona em seu próprio branch.
* **Agent teams**: um [agent team](/docs/pt/agent-teams) é uma sessão que inicia sessões de colega de trabalho para uma única tarefa, na sua máquina ou dentro de uma sessão em nuvem, e termina com essa tarefa.
* **Projects no chat claude.ai e Cowork**: a [experiência anterior de Projects](https://support.claude.com/en/articles/9517075-what-are-projects), que agrupa conversas e arquivos de referência sem threads ou um coordenador. Esses projetos continuam funcionando como fazem hoje até que a experiência redesenhada os alcance.

[Executar agentes em paralelo](/docs/pt/agents) compara essas opções lado a lado.

<h2 id="limitations">
  Limitações
</h2>

* Projects estão disponíveis em claude.ai/code, no aplicativo desktop e no aplicativo móvel Claude, não no CLI do terminal ou através de Amazon Bedrock, Agent Platform do Google Cloud ou Microsoft Foundry. O comando [`claude project`](/docs/pt/cli-reference) do CLI, que gerencia o estado local do Claude Code para um diretório, não está relacionado.
* As threads do projeto são [sessões em nuvem](/docs/pt/claude-code-on-the-web), ou sessões em sua própria máquina através de [Remote Control](/docs/pt/remote-control), com Anthropic como provedor de modelo em ambos os casos. [Segurança](/docs/pt/security) e [Uso de dados](/docs/pt/data-usage) cobrem como as sessões em nuvem são isoladas e o que é retido, e [Conexão e segurança](/docs/pt/remote-control#connection-and-security) cobre como uma thread em sua máquina se conecta e o que é armazenado.
* Você não pode adicionar uma sessão que iniciou você mesmo em sua máquina a um projeto. Para permitir que um projeto execute uma thread em sua máquina, conecte a pasta em que deve funcionar através de [Remote Control](/docs/pt/remote-control#requirements): ative Remote Control em **Settings > Claude Code** no aplicativo desktop Claude, ou execute `claude remote-control` na pasta e deixe-a em execução. Essa máquina precisa do Claude Code v2.1.280 ou posterior. Um projeto também não pode executar uma thread em sua máquina enquanto **Require trusted devices** está ativado em suas configurações de claude.ai.
* O sandbox de uma thread em nuvem pausa entre voltas e retoma quando a thread continua. Se o sandbox não puder ser retomado, a thread continua de um clone fresco, então mudanças não confirmadas podem ser perdidas. Em tarefas longas, peça a Claude para confirmar e enviar trabalho em progresso.
* Um projeto pertence a um usuário. Você não pode compartilhar um projeto ou suas threads com outro usuário, e transcrições de thread não têm a opção de compartilhamento que outras sessões em nuvem têm. Não há controles de nível de organização para projetos durante o beta.
* Uma thread pertence ao único projeto que a iniciou. Você não pode mover ou copiar uma thread para outro projeto, ou movê-la para ficar sozinha. [**Move to project**](#start-from-an-existing-cloud-session) vai apenas na outra direção: traz o trabalho de uma sessão em nuvem para um projeto.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

Para os prompts de configuração do GitHub no diálogo **New project**, veja [Configurar acesso ao GitHub](#set-up-github-access).

<h3 id="a-thread-looks-stuck">
  Uma thread parece estar travada
</h3>

Claude não posta cada passo que uma thread toma, então uma thread que mostra como em execução sem novas mensagens na conversa do projeto geralmente ainda está funcionando. Uma nova thread em nuvem também provisiona seu [ambiente em nuvem](/docs/pt/cloud-environments) antes de Claude começar, então sua primeira atualização leva um momento. Abra a thread para ler sua transcrição. Se a thread está aguardando um prompt de permissão, responda lá.

<h3 id="threads-guessed-or-stalled-instead-of-asking">
  Threads adivinharam ou travaram em vez de perguntar
</h3>

Quando várias threads voltam tendo assumido algo errado, contornado acesso ausente ou parado com "bloqueado", a causa geralmente é a mesma lacuna na configuração do projeto em vez de um problema com cada tarefa. Classifique quais threads são sólidas antes de corrigir qualquer coisa:

1. Peça a Claude na conversa: "Para cada thread aberta, liste o que você pediu a ela para fazer, o que ela assumiu ou não conseguiu alcançar e no que está aguardando." Claude lê cada thread e responde na conversa.
2. Para threads que começaram de uma suposição errada, abra a thread de **Overview** e marque-a como resolvida de seu menu, ou diga a ela o que fazer em vez disso em sua caixa de mensagem. Seu branch e qualquer pull request permanecem no GitHub até que você os delete.
3. Corrija a lacuna uma vez, em [instruções do projeto](#give-a-project-standing-context) ou no [ambiente](#choose-an-environment-for-threads), depois envie uma thread antes de enviar o resto do trabalho novamente como novas threads.

<h3 id="claude-hasnt-responded">
  Claude não respondeu
</h3>

A conversa do projeto mostra um banner "Claude hasn't responded" quando Claude está em execução mas suas respostas não estão alcançando o projeto. Clique **Restart Claude** no banner, ou vá para **Project settings > General** e clique **Restart** na linha **Restart Claude**. Claude se reconecta à conversa; qualquer resposta que estava no meio de escrever é perdida, e as threads não são afetadas.

<h3 id="repository-access-errors">
  Erros de acesso ao repositório
</h3>

Três mensagens significam que uma thread ou o projeto não consegue alcançar um de seus repositórios. Uma thread de projeto em nuvem precisa dos [pré-requisitos do GitHub](#check-the-prerequisites) mesmo quando suas outras sessões em nuvem clonam o mesmo repositório sem problemas.

* **"Couldn't start the session — Claude doesn't have GitHub access to this project's repository"**, relatado antes da thread começar, quando o Claude GitHub App não está instalado nesse repositório, está suspenso ou não está vinculado à conta GitHub que você conectou.
* **"Unable to access your repository"**, relatado por uma thread quando seu clone falha: GitHub rejeitou o clone, o repositório não foi encontrado sob o nome que o projeto tem, ou o branch do qual a thread foi pedida para começar não existe.
* **"Claude can't access"** um repositório, mostrado quando você salva repositórios no diálogo **New project** ou **Project settings**. A mensagem continua com um link de instalação e um link de reconexão. Use o link de instalação se o Claude GitHub App não estiver nesse repositório, e o link de reconexão se estiver, já que o GitHub App pode ser instalado no GitHub sem estar vinculado à conta que você conectou a Claude. Se a mensagem disser que o GitHub App está suspenso ou não inclui esse repositório, siga seu link para o GitHub para corrigir isso.

Para corrigir qualquer um deles, clique no botão que a mensagem oferece, como **Install GitHub App** ou **Select repositories on GitHub**, depois **Check again**. Quando o bloqueio está no lado da organização GitHub, como um proprietário que não aprovou o app ou uma lista de permissão de IP que exclui Claude, a mensagem mostra um link **See how to fix**. Se não houver botão, siga [Configurar acesso ao GitHub](#set-up-github-access), depois envie outra mensagem para tentar novamente.

<h3 id="usage-limit-reached">
  Uma thread atingiu o limite de uso
</h3>

Quando uma thread ou a conversa do projeto atinge o limite de cinco horas ou semanal do seu plano, ela continua tentando por conta própria e continua quando o limite é redefinido. Enquanto aguarda, a thread mostra **Service is busy** com "Claude is still retrying and will continue automatically." Você não precisa fazer nada para o trabalho continuar. Se preferir que não use sua próxima janela de uso, clique **Stop** na thread, ou [pause o projeto](#pause-archive-or-delete-a-project) para manter cada thread. Uma thread que uma routine iniciou não aguarda: sua volta para com um erro de limite, e você envia uma mensagem a ela após o limite ser redefinido.

[Erros de limite de uso](/docs/pt/errors#youve-hit-your-session-limit) explicam os limites e quando eles são redefinidos.

<h3 id="additional-usage-credits-are-required">
  Créditos de uso adicionais são necessários
</h3>

Uma thread ou a conversa do projeto fez uma solicitação que seu plano cobre apenas com créditos de uso, como uma para um modelo ou tamanho de contexto que seu plano não inclui, e créditos de uso não estão ligados para sua conta. [Adicionar créditos de uso à sua assinatura](/docs/pt/costs#add-usage-credits-to-your-subscription) cobre quem pode ligá-los ou comprá-los em cada plano. Uma vez que créditos estão disponíveis, envie outra mensagem para tentar novamente.

<h3 id="context-limit">
  Outras mensagens
</h3>

Essas mensagens nomeiam sua própria causa. A tabela dá o próximo passo para cada uma.

| Mensagem                                                                                      | O que fazer                                                                                                                                                                                                                                            |
| :-------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Unable to connect to repository" com "Claude couldn't reach GitHub to fetch your repository" | Aguarde um momento, depois envie outra mensagem para tentar novamente                                                                                                                                                                                  |
| "Unable to connect to repository" com "Claude couldn't access your repository or environment" | Sua conta GitHub precisa de acesso push ao repositório, e o ambiente ainda deve existir. Verifique ambos em **Project settings > Environment**, depois tente novamente                                                                                 |
| "Couldn't show the setup proposal"                                                            | O app que você tem aberto é mais antigo que as **Setup recommendations** que Claude enviou. Atualize a página ou reinicie o aplicativo desktop, ou peça a Claude para propor a configuração novamente                                                  |
| "The project's environment was removed"                                                       | Escolha um ambiente diferente em **Project settings > Environment**; a mudança se aplica a novas threads                                                                                                                                               |
| "Setup script failed"                                                                         | Clique **Edit setup script** no erro, corrija o script no ambiente, depois envie outra mensagem. [Setup script failed](/docs/pt/web-quickstart#setup-script-failed) lista causas comuns                                                                     |
| "Claude ran out of context on this turn"                                                      | A thread preencheu sua janela de contexto. Se a mensagem disser que a thread continua em uma sessão fresca, ela continua por conta própria; caso contrário, peça a Claude na conversa do projeto para iniciar uma nova thread para o trabalho restante |
| "Reached the turn limit"                                                                      | A thread atingiu o limite de passos agentic que [`CLAUDE_CODE_MAX_TURNS`](/docs/pt/env-vars) define. Envie outra mensagem para continuar, ou aumente ou remova essa variável onde está definida                                                             |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Usar Claude Code na nuvem](/docs/pt/claude-code-on-the-web): como as sessões em nuvem por trás de cada thread funcionam, incluindo opções de acesso ao GitHub e auto-fix em pull requests
* [Configurar ambientes em nuvem](/docs/pt/cloud-environments): mude o que as threads podem alcançar na rede, dê a elas variáveis de ambiente e credenciais de API, e instale ferramentas com um script de configuração
* [Automatizar trabalho com routines](/docs/pt/routines): cronogramas, gatilhos e gerenciamento para routines, incluindo as que Claude cria a partir de um projeto
* [Gerenciar múltiplos agentes com agent view](/docs/pt/agent-view): execute e rastreie várias sessões na sua própria máquina quando o trabalho precisa de ferramentas ou serviços que apenas sua máquina pode alcançar
* [Projects redesigned: from folder to conversation](https://claude.com/blog/projects-redesigned): o anúncio de lançamento, com o raciocínio por trás de tornar um projeto uma conversa com Claude
