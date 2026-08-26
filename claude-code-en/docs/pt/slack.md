> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code no Slack

> Delegue tarefas de codificação diretamente do seu espaço de trabalho Slack

<Note>
  Claude Code no Slack está sendo substituído por [Claude Tag](https://claude.com/product/tag) para espaços de trabalho Team e Enterprise. Claude Tag executa @Claude como a identidade compartilhada da sua organização com acesso configurado pelo administrador, sob o mesmo aplicativo Slack, portanto não há nada para reinstalar e as configurações existentes continuam funcionando durante a transição. Para alternar um espaço de trabalho, consulte [Migrar do Claude anterior no Slack](https://claude.com/docs/claude-tag/admins/migrate-from-earlier).
</Note>

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
| Claude Code na web | O acesso a [Claude Code na web](/docs/pt/claude-code-on-the-web) deve estar habilitado                       |
| Conta GitHub       | Conectada ao Claude Code na web com pelo menos um repositório autenticado                               |
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
    2. Navegue até a aba App Home
    3. Clique em "Connect" para vincular sua conta Slack com sua conta Claude
    4. Conclua o fluxo de autenticação em seu navegador
  </Step>

  <Step title="Configure Claude Code na web">
    Certifique-se de que seu Claude Code na web está devidamente configurado:

    * Visite [claude.ai/code](https://claude.ai/code) e faça login com a mesma conta que você conectou ao Slack
    * Conecte sua conta GitHub se ainda não estiver conectada
    * Autentique pelo menos um repositório com o qual você deseja que Claude trabalhe
  </Step>

  <Step title="Escolha seu modo de roteamento">
    Após conectar suas contas, configure como Claude lida com suas mensagens no Slack. Navegue até o App Home do Claude no Slack para encontrar a configuração **Routing Mode**.

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

Quando você menciona @Claude em um canal ou thread do Slack, Claude analisa automaticamente sua mensagem para determinar se é uma tarefa de codificação. Se Claude detectar intenção de codificação, ele roteará sua solicitação para Claude Code na web em vez de responder como um assistente de chat regular.

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

<h3 id="app-home">
  App Home
</h3>

A aba App Home mostra seu status de conexão e permite que você conecte ou desconecte sua conta Claude do Slack.

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

Claude não é adicionado automaticamente a nenhum canal após a instalação. Os usuários devem convidar explicitamente Claude aos canais onde desejam usá-lo:

* **Convite necessário**: Digite `/invite @Claude` em qualquer canal para adicionar Claude a esse canal
* **Associação ao canal controla o acesso**: Claude só pode responder a @menções em canais onde foi adicionado
* **Controle de acesso através de canais**: Administradores podem controlar o uso de Claude Code gerenciando quais canais Claude é convidado e quem tem acesso a esses canais
* **Suporte a canal privado**: Claude funciona em canais públicos e privados, dando às equipes flexibilidade no controle de visibilidade

Este modelo baseado em canal permite que as equipes restrinjam o uso de Claude Code a canais específicos, fornecendo uma camada adicional de controle de acesso além das permissões no nível do espaço de trabalho.

<h2 id="what’s-accessible-where">
  O que é acessível onde
</h2>

**No Slack**: Você verá atualizações de status, resumos de conclusão e botões de ação. A transcrição completa é preservada e sempre acessível.

**Na web**: A sessão Claude Code completa com histórico de conversa completo, todas as alterações de código, operações de arquivo e a capacidade de continuar a sessão ou criar pull requests.

Para contas Enterprise e Team, as sessões criadas a partir de Claude no Slack são automaticamente visíveis para a organização. Consulte [Compartilhamento de sessões do Claude Code na Web](/docs/pt/claude-code-on-the-web#share-sessions) para mais detalhes.

<h2 id="best-practices">
  Melhores práticas
</h2>

<h3 id="writing-effective-requests">
  Escrevendo solicitações eficazes
</h3>

* **Seja específico**: Inclua nomes de arquivos, nomes de funções ou mensagens de erro quando relevante.
* **Forneça contexto**: Mencione o repositório ou projeto se não estiver claro na conversa.
* **Defina o sucesso**: Explique como "feito" se parece—Claude deve escrever testes? Atualizar documentação? Criar um PR?
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

Este erro significa que sua conta Claude ainda não tem um ambiente em nuvem, não que um administrador precise habilitar algo. Faça login em [claude.ai/code](https://claude.ai/code) uma vez com a mesma conta que você conectou ao Slack. A primeira visita cria seu ambiente em nuvem padrão, e o erro desaparece na sua próxima menção. Cada usuário deve fazer isso individualmente.

<h3 id="sessions-not-starting">
  Sessões não iniciando
</h3>

1. Verifique se sua conta Claude está conectada no App Home do Claude
2. Verifique se você tem acesso a Claude Code na web habilitado
3. Certifique-se de ter pelo menos um repositório GitHub conectado ao Claude Code

<h3 id="repository-not-showing">
  Repositório não aparecendo
</h3>

1. Conecte o repositório em Claude Code na web em [claude.ai/code](https://claude.ai/code)
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

<h3 id="session-expiration">
  Expiração de sessão
</h3>

1. As sessões permanecem acessíveis no seu histórico Claude Code na web
2. Você pode continuar ou fazer referência a sessões passadas em [claude.ai/code](https://claude.ai/code)

<h2 id="current-limitations">
  Limitações atuais
</h2>

* **Apenas GitHub**: Atualmente suporta repositórios no GitHub.
* **Um PR por vez**: Cada sessão pode criar um pull request.
* **Limites de taxa se aplicam**: As sessões usam os limites de taxa do plano Claude individual.
* **Acesso à web necessário**: Os usuários devem ter acesso a Claude Code na web; aqueles sem ele receberão apenas respostas de chat Claude padrão.

<h2 id="related-resources">
  Recursos relacionados
</h2>

<CardGroup>
  <Card title="Claude Code na web" icon="globe" href="/docs/pt/claude-code-on-the-web">
    Saiba mais sobre Claude Code na web
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
