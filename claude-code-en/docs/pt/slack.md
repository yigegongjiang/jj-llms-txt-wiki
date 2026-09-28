> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code no Slack

> Delegue tarefas de codificação diretamente do seu espaço de trabalho Slack. A Anthropic está descontinuando esta versão anterior para espaços de trabalho Team e Enterprise em favor do Claude Tag; ela permanece como o caminho de configuração nos planos Pro e Max.

<Warning>
  Esta página documenta a versão anterior do Claude Code no Slack, que executa cada sessão sob a conta de um usuário individual.

  * **Planos Team e Enterprise:** A Anthropic está descontinuando esta versão em favor do [Claude Tag](https://claude.com/product/tag), que executa @Claude como a identidade compartilhada da sua organização com acesso configurado pelo administrador. Seu aplicativo Slack existente e o identificador @Claude permanecem, e sua equipe de conta Anthropic pode informar a data de transição. [Configure o Claude Tag](https://claude.com/docs/claude-tag/overview) para um novo espaço de trabalho; para mover um que já usa esta versão, consulte [Migrar do Claude anterior no Slack](https://claude.com/docs/claude-tag/admins/migrate-from-earlier).
  * **Planos Pro e Max:** Claude Tag não está disponível em planos individuais, portanto esta página permanece como o caminho de configuração.
</Warning>

Claude Code no Slack traz o poder do Claude Code diretamente para seu espaço de trabalho Slack. Quando você menciona `@Claude` com uma tarefa de codificação, Claude detecta automaticamente a intenção e cria uma sessão Claude Code na web, permitindo que você delegue trabalho de desenvolvimento sem sair de suas conversas em equipe.

Esta integração é construída no aplicativo Claude for Slack existente, mas adiciona roteamento inteligente para Claude Code na web para solicitações relacionadas a codificação. Cada sessão é executada sob sua própria conta Claude, usando seus repositórios conectados e seus limites de plano.

<h2 id="use-cases">
  Casos de uso
</h2>

* **Investigação e correção de bugs**: Peça ao Claude para investigar e corrigir bugs assim que forem relatados nos canais do Slack.
* **Revisões rápidas de código e modificações**: Faça com que Claude implemente pequenos recursos ou refatore código com base no feedback da equipe.
* **Depuração colaborativa**: Quando discussões em equipe fornecem contexto crucial (por exemplo, reproduções de erros ou relatórios de usuários), Claude pode usar essas informações para informar sua abordagem de depuração.
* **Execução de tarefas paralelas**: Inicie tarefas de codificação no Slack enquanto continua outro trabalho, recebendo notificações quando concluído.

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de usar Claude Code no Slack, certifique-se de ter o seguinte:

| Requisito          | Detalhes                                                                                                |
| :----------------- | :------------------------------------------------------------------------------------------------------ |
| Plano Claude       | Pro, Max, Team ou Enterprise com acesso a Claude Code (assentos premium ou assentos Chat + Claude Code) |
| Sessões na nuvem   | [Sessões na nuvem](/docs/pt/claude-code-on-the-web) estão habilitadas para sua conta                         |
| Conta GitHub       | Conectada em [claude.ai/code](https://claude.ai/code) com pelo menos um repositório autenticado         |
| Autenticação Slack | Sua conta Slack vinculada à sua conta Claude por meio do aplicativo Claude                              |

<h2 id="setting-up-claude-code-in-slack">
  Configurando Claude Code no Slack
</h2>

<Steps>
  <Step title="Instale o aplicativo Claude no Slack">
    Um administrador do espaço de trabalho deve instalar o aplicativo Claude no Slack App Marketplace. Visite o [Slack App Marketplace](https://slack.com/marketplace/A08SF47R6P4) e clique em "Add to Slack" para começar o processo de instalação.
  </Step>

  <Step title="Conecte sua conta Claude">
    Após a instalação do aplicativo, autentique sua conta Claude individual:

    1. Abra o aplicativo Claude no Slack clicando em "Claude" na seção Aplicativos
    2. Abra a aba App Home
    3. Clique em "Connect" para vincular sua conta Slack com sua conta Claude
    4. Conclua o fluxo de autenticação em seu navegador
  </Step>

  <Step title="Configure sessões na nuvem">
    Certifique-se de que as sessões na nuvem estão devidamente configuradas para sua conta:

    * Visite [claude.ai/code](https://claude.ai/code) e faça login com a mesma conta que você conectou ao Slack
    * Conecte sua conta GitHub se ainda não estiver conectada
    * Autentique pelo menos um repositório com o qual você deseja que Claude trabalhe
  </Step>

  <Step title="Escolha seu modo de roteamento">
    Após conectar suas contas, configure como Claude lida com suas mensagens no Slack. Abra o App Home do Claude no Slack para encontrar a configuração **Routing Mode**.

    | Modo            | Comportamento                                                                                                                                                                                                                                                       |
    | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    | **Code only**   | Claude roteia todas as @menções para sessões Claude Code. Melhor para equipes que usam Claude no Slack exclusivamente para tarefas de desenvolvimento.                                                                                                              |
    | **Code + Chat** | Claude analisa cada mensagem e roteia inteligentemente entre Claude Code (para tarefas de codificação) e Claude Chat (para escrita, análise e perguntas gerais). Melhor para equipes que desejam um único ponto de entrada @Claude para todos os tipos de trabalho. |

    <Note>
      No modo Code + Chat, se Claude rotear uma mensagem para Chat, mas você queria uma sessão de codificação, você pode clicar em "Retry as Code" para criar uma sessão Claude Code. Da mesma forma, se for roteada para Code, mas você queria uma sessão Chat, você pode escolher essa opção nessa thread.
    </Note>
  </Step>

  <Step title="Adicione Claude aos canais">
    Claude não é adicionado automaticamente a nenhum canal após a instalação. Para usar Claude em um canal, convide-o digitando `/invite @Claude` nesse canal. Claude só pode responder a @menções em canais onde foi adicionado.
  </Step>
</Steps>

<h2 id="how-it-works">
  Como funciona
</h2>

<h3 id="automatic-detection">
  Detecção automática
</h3>

No modo de roteamento Code + Chat, quando você menciona @Claude em um canal ou thread do Slack, Claude detecta automaticamente se sua mensagem é uma tarefa de codificação. Tarefas de codificação vão para uma sessão Claude Code na nuvem. Qualquer outra coisa recebe uma resposta de chat regular. No modo Code only, cada @mention vai para Claude Code.

Você também pode dizer explicitamente ao Claude para lidar com uma solicitação como uma tarefa de codificação, mesmo que ele não a detecte automaticamente.

<Note>
  Claude Code no Slack funciona apenas em canais (públicos ou privados). Não funciona em mensagens diretas (DMs).
</Note>

<h3 id="context-gathering">
  Coleta de contexto
</h3>

**De threads**: Quando você @menciona Claude em uma thread, ele coleta contexto de todas as mensagens nessa thread para entender a conversa completa.

**De canais**: Quando mencionado diretamente em um canal, Claude analisa mensagens recentes do canal para contexto relevante.

Este contexto ajuda Claude a entender o problema, selecionar o repositório apropriado e informar sua abordagem para a tarefa.

<Warning>
  Quando @Claude é invocado no Slack, Claude recebe acesso ao contexto da conversa para entender melhor sua solicitação. Claude pode seguir direções de outras mensagens no contexto, portanto, os usuários devem garantir que usem Claude apenas em conversas Slack confiáveis.
</Warning>

<h3 id="session-flow">
  Fluxo de sessão
</h3>

1. **Iniciação**: Você @menciona Claude com uma solicitação de codificação
2. **Detecção**: Claude analisa sua mensagem e detecta intenção de codificação
3. **Criação de sessão**: Uma nova sessão Claude Code é criada em claude.ai/code
4. **Atualizações de progresso**: Claude publica atualizações de status em sua thread do Slack conforme o trabalho progride
5. **Conclusão**: Quando concluído, Claude o @menciona com um resumo e botões de ação
6. **Revisão**: Clique em "View Session" para ver a transcrição completa ou "Create PR" para abrir um pull request

<h2 id="user-interface-elements">
  Elementos da interface do usuário
</h2>

<h3 id="message-actions">
  Ações de mensagem
</h3>

* **View Session**: Abre a sessão Claude Code completa em seu navegador, onde você pode ver todo o trabalho realizado, continuar a sessão ou fazer solicitações adicionais.
* **Create PR**: Cria um pull request diretamente das alterações da sessão.
* **Retry as Code**: Se Claude inicialmente responder como um assistente de chat, mas você queria uma sessão de codificação, clique neste botão para tentar novamente a solicitação como uma tarefa Claude Code.
* **Change Repo**: Permite que você selecione um repositório diferente se Claude escolheu incorretamente.

<h3 id="repository-selection">
  Seleção de repositório
</h3>

Claude seleciona automaticamente um repositório com base no contexto de sua conversa no Slack. Se vários repositórios pudessem se aplicar, Claude pode exibir um dropdown permitindo que você escolha o correto.

<h2 id="access-and-permissions">
  Acesso e permissões
</h2>

<h3 id="user-level-access">
  Acesso no nível do usuário
</h3>

| Tipo de Acesso        | Requisito                                                             |
| :-------------------- | :-------------------------------------------------------------------- |
| Sessões Claude Code   | Cada usuário executa sessões em sua própria conta Claude              |
| Uso e Limites de Taxa | As sessões contam contra os limites do plano do usuário individual    |
| Acesso ao Repositório | Os usuários só podem acessar repositórios que conectaram pessoalmente |
| Histórico de Sessão   | As sessões aparecem no seu histórico Claude Code em claude.ai/code    |

<h3 id="workspace-level-access">
  Acesso no nível do espaço de trabalho
</h3>

Os administradores do espaço de trabalho Slack controlam se o aplicativo Claude está disponível em seu espaço de trabalho:

| Controle                        | Descrição                                                                                                                                      |
| :------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Instalação do aplicativo        | Administradores do espaço de trabalho decidem se devem instalar o aplicativo Claude no Slack App Marketplace                                   |
| Distribuição do Enterprise Grid | Para organizações do Enterprise Grid, administradores da organização podem controlar quais espaços de trabalho têm acesso ao aplicativo Claude |
| Remoção do aplicativo           | Remover o aplicativo de um espaço de trabalho revoga imediatamente o acesso para todos os usuários nesse espaço de trabalho                    |

<h3 id="channel-based-access-control">
  Controle de acesso baseado em canal
</h3>

A instalação do aplicativo não adiciona Claude a nenhum canal. Claude responde a @menções apenas em canais onde foi adicionado; convide-o com `/invite @Claude`. Funciona em canais públicos e privados. Os administradores podem controlar quem usa Claude Code gerenciando quais canais Claude é convidado e quem tem acesso a esses canais. Isso adiciona uma camada de controle de acesso além das permissões no nível do espaço de trabalho.

<h2 id="what’s-accessible-where">
  O que é acessível onde
</h2>

**No Slack**: Você verá atualizações de status, resumos de conclusão e botões de ação. A transcrição completa é preservada e sempre acessível.

**Em claude.ai/code**: A sessão Claude Code completa com histórico de conversa completo, todas as alterações de código e operações de arquivo. As sessões permanecem no seu histórico do Claude Code em [claude.ai/code](https://claude.ai/code), onde você pode continuar sessões anteriores, consultá-las ou criar pull requests.

Para contas Enterprise e Team, as sessões criadas a partir de Claude no Slack são automaticamente visíveis para a organização. Consulte [compartilhamento de sessões](/docs/pt/claude-code-on-the-web#share-sessions) para mais detalhes.

<h2 id="best-practices">
  Melhores práticas
</h2>

<h3 id="writing-effective-requests">
  Escrevendo solicitações eficazes
</h3>

* **Seja específico**: Inclua nomes de arquivos, nomes de funções ou mensagens de erro quando relevante.
* **Forneça contexto**: Mencione o repositório ou projeto se não estiver claro na conversa.
* **Defina o sucesso**: Explique como "feito" se parece. Claude deve escrever testes? Atualizar documentação? Criar um PR?
* **Use threads**: Responda em threads ao discutir bugs ou recursos para que Claude possa reunir o contexto completo.

<h3 id="when-to-use-slack-vs-web">
  Quando usar Slack vs. web
</h3>

**Use Slack quando**: O contexto já existe em uma discussão do Slack, você quer iniciar uma tarefa de forma assíncrona ou está colaborando com colegas de equipe que precisam de visibilidade.

**Use a web diretamente quando**: Você precisa fazer upload de arquivos, quer interação em tempo real durante o desenvolvimento ou está trabalhando em tarefas mais longas e complexas.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  "Claude Code não está habilitado para sua conta"
</h3>

Este erro significa que sua conta Claude ainda não tem um ambiente em nuvem. Faça login em [claude.ai/code](https://claude.ai/code) uma vez com a mesma conta que você conectou ao Slack e conclua a [integração na web](/docs/pt/web-quickstart#connect-github), que cria seu ambiente em nuvem padrão ou solicita que você o crie. O erro desaparece na sua próxima menção. Cada usuário deve fazer isso individualmente.

<h3 id="sessions-not-starting">
  Sessões não iniciando
</h3>

1. Verifique se sua conta Claude está conectada no App Home do Claude
2. Verifique se as sessões em nuvem estão habilitadas para sua conta
3. Certifique-se de ter pelo menos um repositório GitHub conectado ao Claude Code

<h3 id="sessions-from-a-claude-tag-channel-fail-to-start">
  Sessões de um canal Claude Tag falham ao iniciar
</h3>

Esta entrada se aplica a espaços de trabalho que usam [Claude Tag](https://claude.com/docs/claude-tag/overview), onde Claude funciona em canais como a identidade compartilhada de sua organização, não como a conta de nenhum membro. Se você criou o ambiente em nuvem do canal em [claude.ai/code](https://claude.ai/code), ele pertence à sua conta pessoal, e Claude não pode iniciar sessões de canal em um ambiente pessoal. Claude Code falha na sessão imediatamente, e tentar novamente não ajuda.

Se você for um Proprietário e o ambiente for seu, [compartilhe-o com a organização](/docs/pt/cloud-environments#organization-shared-environments) a partir do seletor de ambiente. Caso contrário, um Proprietário o recria como um ambiente compartilhado da organização a partir da página **Cloud environments** em [configurações de administrador](https://claude.ai/admin-settings).

Você pode aplicá-lo de duas maneiras:

* Defina-o como padrão da organização em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).
* [Defina-o no canal](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) nas configurações de administrador do Claude Tag.

Se você não for um Proprietário, envie esta entrada para um.

<h3 id="repository-not-showing">
  Repositório não aparecendo
</h3>

1. Conecte o repositório em [claude.ai/code](https://claude.ai/code)
2. Verifique suas permissões do GitHub para esse repositório
3. Tente desconectar e reconectar sua conta GitHub

<h3 id="wrong-repository-selected">
  Repositório errado selecionado
</h3>

1. Clique no botão "Change Repo" para selecionar um repositório diferente
2. Inclua o nome do repositório em sua solicitação para seleção mais precisa

<h3 id="authentication-errors">
  Erros de autenticação
</h3>

1. Desconecte e reconecte sua conta Claude no App Home
2. Certifique-se de estar conectado à conta Claude correta em seu navegador
3. Verifique se seu plano Claude inclui acesso a Claude Code

<h2 id="current-limitations">
  Limitações atuais
</h2>

* **Apenas GitHub**: repositórios devem estar no GitHub.
* **Um PR por vez**: cada sessão pode criar um pull request.
* **Acesso à sessão na nuvem necessário**: os usuários precisam ter acesso a [sessões na nuvem](/docs/pt/claude-code-on-the-web); sem ele, Claude responde com respostas de chat padrão.

<h2 id="related-resources">
  Recursos relacionados
</h2>

<CardGroup>
  <Card title="Claude Code na nuvem" icon="cloud" href="/docs/pt/claude-code-on-the-web">
    Saiba mais sobre sessões na nuvem
  </Card>

  <Card title="Claude for Slack" icon="slack" href="https://claude.com/claude-and-slack">
    Documentação geral do Claude for Slack
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    @Claude gerenciado pela organização no Slack com acesso configurado pelo administrador
  </Card>

  <Card title="Slack App Marketplace" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    Instale o aplicativo Claude no Slack Marketplace
  </Card>

  <Card title="Claude Help Center" icon="circle-question" href="https://support.claude.com">
    Obtenha suporte adicional
  </Card>
</CardGroup>
