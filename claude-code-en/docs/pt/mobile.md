> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code no celular

> Inicie, monitore e dirija tarefas do Claude Code do seu telefone com o aplicativo Claude para iOS e Android.

O aplicativo Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) é um cliente para sessões do Claude Code em vez de um lugar onde o código é executado. Do seu telefone você acessa [sessões na nuvem](#start-and-monitor-cloud-sessions) e [projetos](/docs/pt/claude-projects) na nuvem, uma sessão em execução em sua própria máquina através do [Remote Control](#continue-a-local-session-with-remote-control), ou o aplicativo Desktop através do [Dispatch](/docs/pt/desktop#sessions-from-dispatch).

<Note>
  Claude Code não tem um aplicativo móvel separado: sessões na nuvem e Remote Control vivem na aba **Code** no aplicativo Claude, e Dispatch é uma tarefa para a qual você envia mensagens no aplicativo.
</Note>

<h2 id="get-the-app">
  Obtenha o aplicativo
</h2>

<Steps>
  <Step title="Baixe o aplicativo Claude">
    Instale o aplicativo Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Em um iPad, instale o mesmo aplicativo iOS.

    <Tip>
      Execute `/mobile` em uma sessão do Claude Code para exibir um código QR para [claude.ai/mobile](https://claude.ai/mobile), que abre a loja de aplicativos correta para seu telefone. `/ios` e `/android` fazem a mesma coisa.
    </Tip>
  </Step>

  <Step title="Faça login">
    Faça login com a mesma conta claude.ai e organização que você usa para Claude Code. Sessões na nuvem e Remote Control exigem uma conta claude.ai, portanto não são acessíveis com uma chave de API do Anthropic Console ou de um provedor terceirizado como Amazon Bedrock.
  </Step>

  <Step title="Abra a aba Code">
    Toque em **Code** na navegação do aplicativo para acessar suas sessões, ou abra [claude.ai/code/new](https://claude.ai/code/new) no seu telefone para iniciar uma nova sessão Code no aplicativo. Se você não vir a aba Code, seu plano ou organização pode não incluir esses recursos; consulte [disponibilidade por plano de assinatura](/docs/pt/feature-availability#availability-by-subscription-plan).
  </Step>
</Steps>

<h2 id="work-from-your-phone">
  Trabalhe do seu telefone
</h2>

Do aplicativo você pode iniciar sessões na nuvem, abrir um projeto, dirigir uma sessão do Claude Code em execução no seu computador, ou enviar uma tarefa para o Dispatch. O aplicativo é o mesmo para cada um; eles diferem em onde o trabalho acontece.

| Recurso                                        | O que você conecta                                                          | Quando usar                                                                                                                                                                                 |
| :--------------------------------------------- | :-------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Cloud sessions](/docs/pt/claude-code-on-the-web)   | Uma sessão na infraestrutura de nuvem, gerenciada pela Anthropic por padrão | Seu repositório está no GitHub e a tarefa deve continuar em execução depois que você guardar seu telefone. Consulte o [guia de início rápido da nuvem](/docs/pt/web-quickstart) para configurar. |
| [Projects](/docs/pt/claude-projects)                | Uma conversa onde Claude coordena sessões na nuvem paralelas como threads   | Você tem um fluxo de trabalho relacionado em vez de uma tarefa e quer ver quais threads terminaram ou precisam de você.                                                                     |
| [Remote Control](/docs/pt/remote-control)           | Uma sessão do Claude Code em execução no seu computador                     | O trabalho precisa do seu sistema de arquivos local, ferramentas ou servidores MCP.                                                                                                         |
| [Dispatch](/docs/pt/desktop#sessions-from-dispatch) | O aplicativo Desktop no seu computador                                      | Você quer enviar uma tarefa e deixar o Dispatch decidir como executá-la. Requer um plano Pro ou Max.                                                                                        |

Se seu computador estiver desligado, use cloud sessions ou um projeto, que são executados na nuvem e continuam com seu laptop fechado. Remote Control e Dispatch dirigem sua própria máquina, portanto ela precisa permanecer ligada com Claude Code ou o aplicativo Desktop em execução. Se sua máquina hibernar durante uma sessão Remote Control, Claude Code se reconecta quando a máquina volta a ficar online.

Para uma comparação mais completa, consulte [trabalhe quando estiver longe do seu terminal](/docs/pt/platforms#work-when-you-are-away-from-your-terminal).

Cloud sessions e Remote Control são executadas a partir da aba **Code**. Para Dispatch, que você envia como uma tarefa no aplicativo, consulte [sessões do Dispatch](/docs/pt/desktop#sessions-from-dispatch).

<h3 id="start-and-monitor-cloud-sessions">
  Inicie e monitore cloud sessions
</h3>

Cloud sessions executam tarefas na infraestrutura de nuvem, gerenciada pela Anthropic por padrão, portanto uma sessão continua depois que você guarda seu telefone. Na aba Code, selecione um repositório e branch, descreva a tarefa e envie-a. As sessões persistem entre dispositivos: uma tarefa que você inicia no seu laptop está pronta para revisar do seu telefone, e uma que você inicia do seu telefone está esperando quando você volta à sua mesa.

Abra uma sessão no aplicativo para verificar o progresso, responder às perguntas do Claude ou dirigi-lo em uma nova direção. Você também pode dizer ao Claude para [observar um pull request](/docs/pt/claude-code-on-the-web#auto-fix-pull-requests) e corrigir falhas de CI ou comentários de revisão conforme chegam. Para conectar o GitHub e configurar seu ambiente, siga o [guia de início rápido da nuvem](/docs/pt/web-quickstart), e consulte [Use Claude Code na nuvem](/docs/pt/claude-code-on-the-web) para tudo que as cloud sessions podem fazer.

<h3 id="continue-a-local-session-with-remote-control">
  Continue uma sessão local com Remote Control
</h3>

Remote Control conecta o aplicativo Claude a uma sessão do Claude Code em execução em sua máquina, portanto a execução de código e o acesso ao sistema de arquivos permanecem locais enquanto você dirige a sessão do seu telefone. Inicie a sessão no seu computador com `claude remote-control`, ou execute `/remote-control` em uma sessão que já está aberta. Em seguida, escaneie o código QR que o terminal pode exibir, ou abra o aplicativo Claude, toque em **Code** e escolha a sessão na lista. Consulte [conectar de outro dispositivo](/docs/pt/remote-control#connect-from-another-device) para cada opção.

Quando você adiciona um anexo no aplicativo Claude, ele também chega à sessão local:

* **Fotos**: Claude vê fotos anexadas diretamente como parte de sua mensagem. Claude Code também salva cada foto em `~/.claude/uploads/` e diz ao Claude o caminho do arquivo salvo, para que Claude possa copiar a imagem em arquivos que cria.
* **Outros arquivos**: Claude Code os baixa para sua máquina e os passa para Claude como referências de arquivo `@`.

Para requisitos, modos de invocação e solução de problemas, consulte a [visão geral do Remote Control](/docs/pt/remote-control).

<h3 id="get-push-notifications">
  Obtenha notificações push
</h3>

Quando Remote Control está ativo, Claude pode enviar notificações push para seu telefone, normalmente quando uma tarefa de longa duração termina ou quando precisa de uma decisão sua. Você também pode solicitar uma em seu prompt, como `notify me when the tests finish`. Consulte [notificações push móveis](/docs/pt/remote-control#mobile-push-notifications) para os dois toggles `/config` e solução de problemas de entrega.

Dispatch envia sua própria notificação quando uma sessão Code que ele gerou termina ou precisa de sua aprovação, descrito em [sessões do Dispatch](/docs/pt/desktop#sessions-from-dispatch).

<h2 id="limitations">
  Limitações
</h2>

O cliente móvel cobre a maioria do que uma sessão precisa, com algumas limitações:

* **Comandos somente locais**: comandos que só são executados na interface do terminal, como `/plugin` e `/resume`, não funcionam do aplicativo. As [limitações do Remote Control](/docs/pt/remote-control#limitations) listam os comandos que funcionam do celular e como seu comportamento difere.
* **Modos de permissão**: sessões na nuvem oferecem Accept edits, Plan e Auto no menu suspenso de modo, e sessões Remote Control oferecem Manual, Accept edits e Plan. Você não pode selecionar Bypass permissions do aplicativo em nenhum dos casos, e você não pode selecionar Auto para uma sessão Remote Control. Consulte [alternar modos de permissão](/docs/pt/permission-modes#switch-permission-modes).
* **Planos do Dispatch**: Dispatch requer um plano Pro ou Max e não está disponível em Team ou Enterprise.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Plataformas e integrações](/docs/pt/platforms): compare todas as superfícies em que Claude Code é executado
* [Claude Code na web](/docs/pt/claude-code-on-the-web): como as sessões na nuvem são executadas e como mover trabalho para e do seu terminal
* [Configure ambientes na nuvem](/docs/pt/cloud-environments): níveis de acesso à rede, variáveis de ambiente e scripts de configuração para sessões na nuvem
* [Remote Control](/docs/pt/remote-control): continue uma sessão local de qualquer dispositivo
* [Sessões do Dispatch](/docs/pt/desktop#sessions-from-dispatch): como as tarefas do Dispatch se tornam sessões Code no aplicativo Desktop
* [Channels](/docs/pt/channels): pergunte algo ao Claude do seu telefone via Telegram, Discord ou iMessage enquanto o trabalho é executado em sua máquina
* [Claude Code no Slack](/docs/pt/slack): delegue tarefas de codificação do seu espaço de trabalho Slack mencionando `@Claude`
