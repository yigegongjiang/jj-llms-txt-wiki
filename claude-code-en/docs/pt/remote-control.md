> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Continue sessões locais de qualquer dispositivo com Remote Control

> Continue uma sessão local do Claude Code do seu telefone, tablet ou qualquer navegador usando Remote Control. Funciona com claude.ai/code e o aplicativo Claude para dispositivos móveis.

<Note>
  Remote Control está em visualização de pesquisa e disponível em todos os planos. Em Team e Enterprise, ele fica desativado por padrão até que um Owner ative o toggle Remote Control nas [configurações de administrador do Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control conecta [claude.ai/code](https://claude.ai/code) ou o aplicativo Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) a uma sessão do Claude Code em execução na sua máquina. Inicie uma tarefa na sua mesa, depois continue a partir do seu telefone no sofá ou de um navegador em outro computador.

Quando você inicia uma sessão de Remote Control na sua máquina, Claude continua executando localmente o tempo todo, portanto seu código e acesso ao sistema de arquivos permanecem na sua máquina. Com Remote Control você pode:

* **Usar seu ambiente local completo remotamente**: seu sistema de arquivos, [MCP servers](/docs/pt/mcp), ferramentas e configuração do projeto permanecem disponíveis, e digitar `@` autocompleta caminhos de arquivo do seu projeto local
* **Trabalhar em ambas as superfícies ao mesmo tempo**: a conversa e o progresso de [subagentes](/docs/pt/sub-agents) e [fluxos de trabalho dinâmicos](/docs/pt/workflows) permanecem sincronizados em todos os dispositivos conectados, para que você possa enviar mensagens do seu terminal, navegador e telefone de forma intercambiável. Antes da v2.1.207, as sessões hospedadas pelo [aplicativo Desktop](/docs/pt/desktop) não enviavam progresso de subagentes ou fluxos de trabalho para dispositivos conectados.
* **Enviar imagens e arquivos do seu telefone ou navegador**: quando você adiciona um anexo no aplicativo Claude ou em claude.ai/code, Claude Code faz o download para sua máquina e o passa para Claude como uma referência de arquivo `@`, com ou sem legenda. Antes da v2.1.202, Claude Code podia descartar um anexo enviado sem legenda antes de chegar à sessão.
* **Sobreviver a interrupções**: se seu laptop dormir ou sua rede cair, a sessão se reconecta automaticamente quando sua máquina voltar a ficar online. Claude Code enfileira atualizações de status de subagentes e fluxos de trabalho enquanto a conexão está sendo reconstruída e as entrega assim que se recupera. Antes da v2.1.207, uma atualização enviada durante uma reconexão ou atualização de credenciais podia ser perdida, portanto o dispositivo conectado continuava mostrando uma tarefa concluída como em execução.

Diferentemente do [Claude Code na web](/docs/pt/claude-code-on-the-web), que é executado em infraestrutura em nuvem, as sessões de Remote Control são executadas diretamente na sua máquina e interagem com seu sistema de arquivos local. As interfaces web e móvel são apenas uma janela para essa sessão local.

Esta página aborda a configuração, como iniciar e conectar a sessões, e como Remote Control se compara ao Claude Code na web.

<h2 id="requirements">
  Requisitos
</h2>

Antes de usar Remote Control, confirme que seu ambiente atende a estas condições:

* **Assinatura**: disponível nos planos Pro, Max, Team e Enterprise. Chaves de API não são suportadas. Em Team e Enterprise, um Owner deve primeiro ativar o toggle Remote Control nas [configurações de administrador do Claude Code](https://claude.ai/admin-settings/claude-code).
* **Autenticação**: execute `claude` e use `/login` para fazer login através de claude.ai se você ainda não fez isso.
* **Endpoint de API**: não disponível no Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. A partir da v2.1.196, Remote Control também é desabilitado quando [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) aponta para um host diferente de `api.anthropic.com`, como um [gateway LLM](/docs/pt/llm-gateway) ou proxy. Desative a variável para usar Remote Control.
* **Confiança do workspace**: execute `claude` no diretório do seu projeto pelo menos uma vez para aceitar o diálogo de confiança do workspace.

<h2 id="start-a-remote-control-session">
  Inicie uma sessão de Remote Control
</h2>

Você pode iniciar uma sessão de Remote Control a partir da CLI ou da extensão VS Code. A CLI oferece três modos de invocação; VS Code usa o comando `/remote-control`.

<Tabs>
  <Tab title="Modo servidor">
    Navegue até o diretório do seu projeto e execute:

    ```bash theme={null}
    claude remote-control
    ```

    O processo continua em execução no seu terminal em modo servidor, aguardando conexões remotas. Ele exibe uma URL de sessão que você pode usar para [conectar de outro dispositivo](#connect-from-another-device), e você pode pressionar a barra de espaço para mostrar um código QR para acesso rápido do seu telefone. Enquanto uma sessão remota está ativa, o terminal mostra o status da conexão e a atividade da ferramenta.

    Sinalizadores disponíveis:

    | Sinalizador                                     | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
    | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Define um título de sessão personalizado visível na lista de sessões em claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
    | `--remote-control-session-name-prefix <prefix>` | Prefixo para nomes de sessão gerados automaticamente quando nenhum nome explícito é definido. O padrão é o nome do host da sua máquina, produzindo nomes como `myhost-graceful-unicorn`. Defina `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` para o mesmo efeito.                                                                                                                                                                                                                                                                          |
    | `-c`, `--continue`                              | Retome a sessão de Remote Control mais recente iniciada a partir deste diretório em vez de criar uma nova. Não pode ser combinado com `--session-id`, `--spawn`, `--capacity` ou `--create-session-in-dir`. Requer Claude Code v2.1.200 ou posterior; versões anteriores rejeitam o sinalizador como um argumento desconhecido.                                                                                                                                                                                                           |
    | `--session-id <id>`                             | Retome uma sessão de Remote Control específica pelo seu ID. Não pode ser combinado com `--continue`, `--spawn`, `--capacity` ou `--create-session-in-dir`. Requer Claude Code v2.1.200 ou posterior; versões anteriores rejeitam o sinalizador como um argumento desconhecido.                                                                                                                                                                                                                                                            |
    | `--spawn <mode>`                                | Como o servidor cria sessões.<br />• `same-dir` (padrão): todas as sessões compartilham o diretório de trabalho atual, portanto podem entrar em conflito se editarem os mesmos arquivos.<br />• `worktree`: cada sessão sob demanda obtém seu próprio [git worktree](/docs/pt/worktrees). Requer um repositório git.<br />• `session`: modo de sessão única. Serve exatamente uma sessão e rejeita conexões adicionais. Definido apenas na inicialização.<br />Pressione `w` em tempo de execução para alternar entre `same-dir` e `worktree`. |
    | `--capacity <N>`                                | Número máximo de sessões simultâneas. O padrão é 32. Não pode ser usado com `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
    | `--[no-]create-session-in-dir`                  | Pré-crie uma sessão no diretório atual quando o servidor inicia, para que você tenha um lugar para digitar imediatamente. Em modo `worktree`, essa sessão permanece no diretório atual enquanto as sessões sob demanda obtêm worktrees isoladas. Ativado por padrão; passe `--no-create-session-in-dir` para iniciar sem nenhuma.                                                                                                                                                                                                         |
    | `--verbose`                                     | Mostra logs detalhados de conexão e sessão.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
    | `--sandbox` / `--no-sandbox`                    | Ativa ou desativa [sandboxing](/docs/pt/sandboxing) para isolamento de sistema de arquivos e rede. Desativado por padrão.                                                                                                                                                                                                                                                                                                                                                                                                                      |
  </Tab>

  <Tab title="Sessão interativa">
    Para iniciar uma sessão normal interativa do Claude Code com Remote Control ativado, use a flag `--remote-control` (ou `--rc`):

    ```bash theme={null}
    claude --remote-control
    ```

    Opcionalmente, passe um nome para a sessão:

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    Isso oferece uma sessão interativa completa no seu terminal que você também pode controlar a partir de claude.ai ou do aplicativo Claude. Diferentemente de `claude remote-control` (modo servidor), você pode digitar mensagens localmente enquanto a sessão também está disponível remotamente.
  </Tab>

  <Tab title="De uma sessão existente">
    Se você já está em uma sessão do Claude Code e deseja continuá-la remotamente, use o comando `/remote-control` (ou `/rc`):

    ```text theme={null}
    /remote-control
    ```

    Passe um nome como argumento para definir um título de sessão personalizado:

    ```text theme={null}
    /remote-control My Project
    ```

    Isso inicia uma sessão de Remote Control que carrega seu histórico de conversa atual.

    As flags `--verbose`, `--sandbox` e `--no-sandbox` não estão disponíveis com este comando.
  </Tab>

  <Tab title="VS Code">
    Na [extensão VS Code do Claude Code](/docs/pt/vs-code), digite `/remote-control` ou `/rc` na caixa de prompt, ou abra o menu de comandos com `/` e selecione-o.

    ```text theme={null}
    /remote-control
    ```

    Um banner aparece acima da caixa de prompt mostrando o status da conexão. Uma vez conectado, clique em **Open in browser** no banner para ir diretamente para a sessão, ou encontre-a na lista de sessões em [claude.ai/code](https://claude.ai/code). A URL da sessão também é postada na conversa.

    Para desconectar, clique no ícone de fechar no banner ou execute `/remote-control` novamente.

    Diferentemente da CLI, o comando VS Code não aceita um argumento de nome ou exibe um código QR. O título da sessão é derivado do seu histórico de conversa ou primeiro prompt.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Verificar status da conexão
</h3>

Em uma sessão de terminal interativa, um indicador `/rc active` fica no rodapé abaixo da caixa de entrada enquanto a conexão está ativa, e fica oculto se o terminal for muito estreito para ajustá-lo. O texto do indicador é um link para a sessão em claude.ai. Selecione-o com a tecla de seta para baixo e pressione Enter, ou execute `/remote-control` novamente, para abrir um painel de status com a URL da sessão e um código QR que você pode usar para [conectar de outro dispositivo](#connect-from-another-device).

Se a conexão falhar, uma notificação aparece com o motivo da falha e o indicador desaparece do rodapé. Execute `/remote-control` novamente para tentar novamente.

<h3 id="connect-from-another-device">
  Conectar de outro dispositivo
</h3>

Depois que uma sessão de Remote Control está ativa, você tem algumas maneiras de conectar de outro dispositivo:

* **Abra a URL da sessão** em qualquer navegador para ir diretamente para a sessão em [claude.ai/code](https://claude.ai/code).
* **Escaneie o código QR** mostrado ao lado da URL da sessão para abri-lo diretamente no aplicativo Claude. Com `claude remote-control`, pressione a barra de espaço para alternar a exibição do código QR.
* **Abra [claude.ai/code](https://claude.ai/code) ou o aplicativo Claude** e encontre a sessão pelo nome na lista de sessões. No aplicativo móvel Claude, toque em **Code** na navegação para acessar a lista de sessões. As sessões de Remote Control mostram um ícone de computador com um ponto de status verde quando online.

Quando você se conecta, o dispositivo mostra quaisquer subagentes e fluxos de trabalho que a sessão já tem em execução em segundo plano. Antes da v2.1.208, um dispositivo conectado a uma sessão hospedada em um terminal interativo não mostrava subagentes e fluxos de trabalho que já estavam em execução até que um deles iniciasse ou parasse.

O título da sessão remota é escolhido nesta ordem:

1. O nome que você passou para `--name`, `--remote-control` ou `/remote-control`
2. O título que você definiu com `/rename`
3. A última mensagem significativa no histórico de conversa existente
4. Um nome gerado automaticamente como `myhost-graceful-unicorn`, onde `myhost` é o nome do host da sua máquina ou o prefixo que você definiu com `--remote-control-session-name-prefix`

Se você não definir um nome explícito, o título será atualizado para refletir seu prompt assim que você enviar um. A partir da Claude Code v2.1.176, títulos gerados automaticamente correspondem ao idioma da sua conversa, ou à configuração [`language`](/docs/pt/settings#available-settings) se uma estiver configurada. Renomear uma sessão a partir de claude.ai ou do aplicativo Claude também atualiza o título local mostrado em `claude --resume`.

Se o ambiente já tiver uma sessão ativa, você será perguntado se deseja continuá-la ou iniciar uma nova.

Se você ainda não tem o aplicativo Claude, use o comando `/mobile` dentro do Claude Code para exibir um código QR de download para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).

<h3 id="enable-remote-control-for-all-sessions">
  Ativar Remote Control para todas as sessões
</h3>

Remote Control só é ativado quando você executa explicitamente `claude remote-control`, `claude --remote-control` ou `/remote-control`, a menos que a conexão automática esteja ativada. Para ativá-lo automaticamente para cada sessão interativa, execute `/config` dentro do Claude Code e defina **Enable Remote Control for all sessions** como `true`. Defina-o como `false` para nunca conectar automaticamente, ou deixe-o não definido para seguir o padrão da sua organização. No aplicativo Desktop, você também pode alternar isso em **Settings → Claude Code → Enable remote control by default**. Na [extensão VS Code](/docs/pt/vs-code#use-the-prompt-box), o mesmo botão de alternância aparece como **Enable Remote Control for all sessions** na seção Configurações do menu de comandos; requer Claude Code v2.1.203 ou posterior.

Com essa configuração ativada, cada processo interativo do Claude Code registra uma sessão remota. Se você executar várias instâncias, cada uma obtém seu próprio ambiente e sessão. Para executar várias sessões simultâneas a partir de um único processo, use o [modo servidor](#start-a-remote-control-session) em vez disso.

<h2 id="connection-and-security">
  Conexão e segurança
</h2>

Sua sessão local do Claude Code faz apenas solicitações HTTPS de saída e nunca abre portas de entrada na sua máquina. Quando você inicia Remote Control, ele se registra na API Anthropic e faz polling para trabalho. Quando você conecta de outro dispositivo, o servidor roteia mensagens entre o cliente web ou móvel e sua sessão local através de uma conexão de streaming.

Todo o tráfego viaja através da API Anthropic sobre TLS, o mesmo transporte de segurança que qualquer sessão do Claude Code. A conexão usa múltiplas credenciais de curta duração, cada uma com escopo para um único propósito e expirando independentemente.

Enquanto Remote Control está conectado, a transcrição da sessão, incluindo suas mensagens, respostas do Claude e atividade de ferramentas, é armazenada nos servidores Anthropic. A transcrição armazenada mantém a conversa sincronizada em seus dispositivos e permite que a sessão se reconecte após uma queda de rede. A execução e o acesso ao sistema de arquivos permanecem na sua máquina, e as transcrições armazenadas são retidas sob a política de [Uso de dados](/docs/pt/data-usage).

Para desativar Remote Control completamente, use a configuração [`disableRemoteControl`](/docs/pt/settings#available-settings). Organizações com requisitos de conformidade, como Zero Data Retention, não podem ativar Remote Control.

<h2 id="trusted-devices">
  Dispositivos Confiáveis
</h2>

<Note>
  Dispositivos Confiáveis está atualmente em beta. Recursos e funcionalidades podem evoluir conforme a experiência é refinada.

  Dispositivos Confiáveis está disponível nos planos Team e Enterprise. Ele fica desativado por padrão até que um administrador o ative.
</Note>

Dispositivos Confiáveis é uma configuração em toda a organização que requer que os membros verifiquem seu dispositivo antes de poderem visualizar ou controlar sessões de Remote Control a partir de claude.ai, dos aplicativos Claude para dispositivos móveis ou Claude Desktop. Ele vincula o acesso ao Remote Control a um dispositivo conhecido e uma autenticação recente, não apenas a uma conta conectada.

Quando a configuração está ativada, interagir com uma sessão de Remote Control requer ambos os seguintes:

* **Um dispositivo inscrito**: cada navegador, telefone ou aplicativo desktop que um membro usa para Remote Control inscreve sua própria credencial. A inscrição é oferecida apenas pouco tempo após um login completo, portanto um dispositivo entra na lista confiável como parte de uma autenticação real em vez de silenciosamente em segundo plano.
* **Um login recente**: o login do membro não deve ter mais de 18 horas. Em vez de fazer login novamente a cada dia, os membros confirmam presença com Face ID, Touch ID, Windows Hello ou uma passkey. Esta etapa de autenticação biométrica atualiza a sessão imediatamente.

Verificações biométricas são executadas no dispositivo através do sistema operacional ou navegador, o mesmo mecanismo que o login com passkey. Anthropic nunca recebe ou armazena impressões digitais, dados faciais ou qualquer outra informação biométrica. Apenas a chave pública do dispositivo e metadados básicos como nome de exibição, plataforma e hora de inscrição são armazenados.

A configuração se aplica apenas ao Remote Control. Chat regular do Claude, Claude Code no terminal e uso de API não são afetados.

<h3 id="enable-trusted-devices-for-your-organization">
  Ativar Dispositivos Confiáveis para sua organização
</h3>

Administradores ativam a configuração a partir do console de administrador do Claude Code.

<Steps>
  <Step title="Abra as configurações de administrador do Claude Code">
    Vá para [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). O toggle **Require trusted devices** aparece sob a configuração Remote Control.
  </Step>

  <Step title="Ative Require trusted devices">
    A configuração se aplica a cada membro da organização e a sessões de Remote Control iniciadas após você ativar. Sessões que já estavam em execução antes do toggle ser ativado não são retroativamente protegidas e continuam sem o requisito de dispositivo até que terminem. Escopo por equipe ou por projeto não está disponível.
  </Step>

  <Step title="Informe aos membros o que esperar">
    A primeira vez que um membro visualiza ou controla uma nova sessão de Remote Control a partir de um navegador, telefone ou aplicativo desktop após a configuração ser ativada, ele é solicitado a inscrever esse dispositivo. Informá-los com antecedência evita confusão.
  </Step>
</Steps>

<h3 id="what-members-see">
  O que os membros veem
</h3>

A inscrição é uma etapa única por dispositivo. Depois disso, a única mudança visível é um prompt biométrico ocasional.

* **Primeiro uso em cada dispositivo**: o membro é solicitado a se inscrever. Se seu login não for recente, ele faz login primeiro através do seu fluxo normal, incluindo SSO se configurado, depois confirma a inscrição.
* **Dia a dia**: membros com um dispositivo inscrito e um login recente não veem prompts. Quando o login envelhece além de 18 horas, a próxima interação de Remote Control mostra um único prompt de Face ID, Touch ID, Windows Hello ou passkey.
* **Dispositivos não inscritos**: sessões de Remote Control não podem ser visualizadas ou controladas até que o dispositivo seja inscrito. Chat regular do Claude nesse dispositivo não é afetado.
* **Sem autenticador de plataforma**: membros em uma máquina sem Face ID, Touch ID ou Windows Hello podem usar uma chave de segurança de hardware ou fazer login novamente em vez de fazer uma autenticação.
* **No terminal**: a máquina executando Claude Code recebe sua própria credencial automaticamente quando o desenvolvedor faz login na CLI. Não há etapa de inscrição separada no terminal.

<h3 id="manage-enrolled-devices">
  Gerenciar dispositivos inscritos
</h3>

Os membros podem revisar e revogar seus próprios dispositivos a partir das configurações de conta.

Abra [claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) e encontre a seção **Trusted devices** para ver cada dispositivo inscrito com seu nome, plataforma e data de inscrição. Remover um dispositivo revoga sua credencial imediatamente, e o dispositivo pode se inscrever novamente mais tarde após um novo login. Credenciais também expiram por conta própria se não forem renovadas, portanto um dispositivo não utilizado sai da lista confiável automaticamente.

Para um dispositivo perdido ou roubado, o membro o remove desta página. Se o membro não conseguir fazer login, um administrador pode usar **Sign out everywhere** no console de administrador para revogar cada sessão e dispositivo inscrito para esse membro, após o qual o membro inscreve novamente os dispositivos que ainda possui.

<h2 id="remote-control-vs-claude-code-on-the-web">
  Remote Control vs Claude Code na web
</h2>

Remote Control e [Claude Code na web](/docs/pt/claude-code-on-the-web) usam a interface claude.ai/code. A diferença fundamental é onde a sessão é executada: Remote Control é executado na sua máquina, portanto seus MCP servers locais, ferramentas e configuração do projeto permanecem disponíveis. Claude Code na web é executado em infraestrutura em nuvem gerenciada pela Anthropic.

Use Remote Control quando você está no meio do trabalho local e deseja continuar de outro dispositivo. Use Claude Code na web quando você deseja iniciar uma tarefa sem nenhuma configuração local, trabalhar em um repositório que você não tem clonado ou executar várias tarefas em paralelo.

<h2 id="mobile-push-notifications">
  Notificações push móveis
</h2>

Quando Remote Control está ativo, Claude pode enviar notificações push para seu telefone.

Claude decide quando fazer push. Normalmente envia uma quando uma tarefa de longa duração termina ou quando precisa de uma decisão sua para continuar. Você também pode solicitar um push em seu prompt, por exemplo `notify me when the tests finish`. Além dos dois toggles on/off abaixo, não há configuração por evento.

Para configurar notificações push móveis:

<Steps>
  <Step title="Instale o aplicativo Claude para dispositivos móveis">
    Baixe o aplicativo Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).
  </Step>

  <Step title="Faça login com sua conta do Claude Code">
    Use a mesma conta e organização que você usa para Claude Code no terminal.
  </Step>

  <Step title="Permita notificações">
    Aceite o prompt de permissão de notificação do sistema operacional.
  </Step>

  <Step title="Ative push no Claude Code">
    No seu terminal, execute `/config` e ative **Push when Claude decides** para notificações proativas, **Push when actions required** para prompts de permissão e perguntas, ou ambas.
  </Step>
</Steps>

Se as notificações não chegarem:

* Se `/config` mostrar **No mobile registered**, abra o aplicativo Claude no seu telefone para que ele possa atualizar seu token de push. O aviso desaparece na próxima vez que Remote Control se conectar.
* No iOS, os modos Focus e resumos de notificações podem suprimir ou atrasar pushes. Verifique Configurações → Notificações → Claude.
* No Android, a otimização agressiva de bateria pode atrasar a entrega. Isente o aplicativo Claude da otimização de bateria nas configurações do sistema.

Claude Code pula notificações push móveis enquanto você está digitando ou focado no terminal conectado. A partir da v2.1.181, você pode definir [`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/pt/env-vars) para um caminho de arquivo marcador para estender isso para qualquer momento em que você esteja na máquina, mesmo em outra janela: notificações são puladas enquanto o arquivo existe. Configure um ouvinte de bloqueio de tela ou ferramenta similar para criar o arquivo quando sua tela desbloqueia e deletá-lo quando sua tela bloqueia.

<h2 id="limitations">
  Limitações
</h2>

* **Uma sessão remota por processo interativo**: fora do modo servidor, cada instância do Claude Code suporta uma sessão remota por vez. Use o [modo servidor](#start-a-remote-control-session) para executar várias sessões simultâneas a partir de um único processo.
* **O processo local deve continuar em execução**: Remote Control é executado como um processo local. Se você fechar o terminal, sair do VS Code ou parar o processo `claude`, a sessão termina.
* **Interrupção de rede estendida**: se sua máquina estiver ligada mas não conseguir alcançar a rede por mais de aproximadamente 10 minutos, a sessão expira e o processo sai. Execute `claude remote-control` novamente para iniciar uma nova sessão.
* **Ultraplan desconecta Remote Control**: iniciar uma sessão [ultraplan](/docs/pt/ultraplan) desconecta qualquer sessão de Remote Control ativa porque ambos os recursos ocupam a interface claude.ai/code e apenas um pode estar conectado por vez.
* **Alguns comandos são apenas locais**: comandos que funcionam apenas na interface do terminal, como `/plugin` ou `/resume`, funcionam apenas a partir da CLI local, independentemente de você passar um argumento ou não. Os seguintes funcionam a partir de dispositivos móveis e web:
  * Comandos de saída de texto: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits` (executa o formulário de texto em vez de abrir o diálogo na CLI), `/recap`, `/reload-plugins`
  * `/model`, `/effort`, `/fast`, `/color` e `/rename`: passe o valor como um argumento, por exemplo `/model sonnet` ou `/effort high`. A partir de dispositivos móveis e web, `/model` e `/effort` recebem o argumento no lugar do seletor do terminal ou controle deslizante.
  * `/mcp`, a partir da v2.1.166: a partir do aplicativo móvel, retorna um resumo de texto do status do servidor em vez de abrir o seletor. Na web, `/mcp` sozinho abre um diretório de [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) em vez de retornar o resumo. Os [subcomandos](/docs/pt/commands#all-commands) `reconnect`, `enable` e `disable` funcionam em ambos. Diferentemente da CLI local, `/mcp reconnect` sem um nome de servidor reconecta todos os servidores que falharam ou precisam de autenticação.
  * `/config`, a partir da v2.1.181: a partir do aplicativo móvel, passe `key=value` para definir uma configuração, ou execute sem argumentos para listar as chaves que você pode definir. Na web, `/config` abre a seção Claude Code das suas configurações, e ignora o texto após o comando.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control requires a claude.ai subscription"
</h3>

Você não está autenticado com uma conta claude.ai. Execute `claude auth login` e escolha a opção claude.ai. Se `ANTHROPIC_API_KEY` estiver definida em seu ambiente, desative-a primeiro.

Antes da v2.1.206, executar `/remote-control` enquanto desconectado relatava `Unknown command: /remote-control` em vez desta mensagem.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control requires a full-scope login token"
</h3>

Você está autenticado com um token de longa duração de `claude setup-token` ou da variável de ambiente `CLAUDE_CODE_OAUTH_TOKEN`. Esses tokens são limitados apenas a inferência e não podem estabelecer sessões de Remote Control. Execute `claude auth login` para autenticar com um token de sessão de escopo completo em vez disso.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Unable to determine your organization for Remote Control eligibility"
</h3>

Suas informações de conta em cache estão desatualizadas ou incompletas. Execute `claude auth login` para atualizá-las.

<h3 id="remote-control-is-not-yet-enabled-for-your-account">
  "Remote Control is not yet enabled for your account"
</h3>

A verificação de Remote Control não atingiu sua conta, ou seus direitos em cache estão desatualizados. Se você mudou de plano recentemente, execute `claude auth logout` e depois `claude auth login` para atualizá-los. Execute `claude doctor` para ver qual verificação de elegibilidade individual falhou. Conflitos de variáveis de ambiente, verificações inacessíveis e política organizacional cada um produzem sua própria mensagem, então este erro significa o próprio portão de verificação.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Couldn't verify Remote Control eligibility"
</h3>

Claude Code não conseguiu alcançar o serviço de sinalizador de recurso para verificar se Remote Control está habilitado para sua conta, normalmente porque você está offline ou um proxy está bloqueando a solicitação. Tente novamente quando tiver acesso à rede, ou execute `claude doctor` para obter detalhes. A mensagem relacionada "Couldn't verify your organization's Remote Control policy" tem a mesma causa e a mesma solução. Ambas as mensagens foram adicionadas na v2.1.178.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control is only available when using Claude via api.anthropic.com"
</h3>

A sessão não está se comunicando diretamente com a API Anthropic, portanto não há backend claude.ai para emparelhar. Isso acontece no Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. A partir da v2.1.196, também acontece quando [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) aponta para um host diferente de `api.anthropic.com`, como um [gateway LLM](/docs/pt/llm-gateway) ou proxy, mesmo se você entrar com claude.ai. Desative `ANTHROPIC_BASE_URL` e reinicie a sessão para usar Remote Control.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control is disabled by your organization's policy"
</h3>

Este erro tem quatro causas distintas. Execute `/status` primeiro para ver qual método de login e assinatura você está usando.

* **Você está autenticado com uma chave de API ou conta Console**: Remote Control requer OAuth claude.ai. Execute `/login` e escolha a opção claude.ai. Se `ANTHROPIC_API_KEY` estiver definida em seu ambiente, desative-a.
* **Um Proprietário não ativou para sua organização**: Remote Control fica desativado por padrão nos planos Team e Enterprise. Um Proprietário pode ativá-lo em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) ativando o toggle **Remote Control**. Este toggle é uma configuração de organização no lado do servidor.
* **O toggle do administrador está acinzentado**: sua organização tem uma configuração de retenção de dados ou conformidade que é incompatível com Remote Control. Isso não pode ser alterado no painel de administração. Entre em contato com o suporte da Anthropic para discutir opções.
* **O erro menciona `disableRemoteControl`**: seu administrador de TI desativou Remote Control neste dispositivo através de [configurações gerenciadas](/docs/pt/settings#settings-files), independentemente do toggle em toda a organização.

<h3 id="remote-credentials-fetch-failed">
  "Remote credentials fetch failed"
</h3>

Claude Code não conseguiu obter uma credencial de curta duração da API Anthropic para estabelecer a conexão. Execute novamente com `--verbose` para ver o erro completo:

```bash theme={null}
claude remote-control --verbose
```

Causas comuns:

* Não conectado: execute `claude` e use `/login` para autenticar com sua conta claude.ai. A autenticação por chave de API não é suportada para Remote Control.
* Problema de rede ou proxy: um firewall ou proxy pode estar bloqueando a solicitação HTTPS de saída. Remote Control requer acesso à API Anthropic na porta 443.
* Falha na criação de sessão: se você também vir `Session creation failed — see debug log`, a falha aconteceu anteriormente na configuração. Verifique se sua assinatura está ativa.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Couldn't reconnect to your Remote Control session"
</h3>

Quando você retoma uma conversa com `claude --resume` ou `claude --continue`, Claude Code se reconecta à sessão de Remote Control registrada nessa conversa. Esta mensagem significa que a reconexão falhou por um motivo que pode ser temporário, como uma interrupção de rede ou um erro de servidor, então Claude Code não pode confirmar se a sessão remota ainda existe. Quando o servidor confirma que a sessão anterior não existe mais, Claude Code cria uma nova sessão de Remote Control sem mostrar esta mensagem.

Sua sessão local continua funcionando sem Remote Control. Execute `/remote-control` para tentar novamente a conexão, ou inicie Claude Code sem `--resume` para criar uma nova sessão de Remote Control.

Antes da v2.1.200, uma falha de reconexão criava uma nova sessão de Remote Control em vez de mostrar esta mensagem, o que deixava sessões extras na lista de sessões em claude.ai/code.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  "Your organization requires Trusted Devices for Remote Control, but this device is not enrolled"
</h3>

Sua organização tem [Dispositivos Confiáveis](#trusted-devices) ativado e esta máquina não se inscreveu ainda. Execute `/login` no Claude Code. A inscrição acontece como parte do login, e não há comando de inscrição separado.

<h3 id="session-expired-for-trusted-device-check">
  "session expired for trusted-device check"
</h3>

Seu login tem mais de 18 horas. Execute `/login` no Claude Code, ou confirme com Face ID, Touch ID, Windows Hello ou uma passkey quando claude.ai ou o aplicativo móvel solicitar. Veja [Dispositivos Confiáveis](#trusted-devices).

<h2 id="choose-the-right-approach">
  Escolha a abordagem correta
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Claude Code na web](/docs/pt/claude-code-on-the-web): execute sessões em ambientes em nuvem gerenciados pela Anthropic em vez de na sua máquina
* [Ultraplan](/docs/pt/ultraplan): inicie uma sessão de planejamento em nuvem a partir do seu terminal e revise o plano no seu navegador
* [Channels](/docs/pt/channels): encaminhe Telegram, Discord ou iMessage para uma sessão para que Claude reaja a mensagens enquanto você está ausente
* [Dispatch](/docs/pt/desktop#sessions-from-dispatch): envie uma mensagem com uma tarefa do seu telefone e ela pode gerar uma sessão Desktop para lidar com isso
* [Autenticação](/docs/pt/authentication): configure `/login` e gerencie credenciais para claude.ai
* [Referência de CLI](/docs/pt/cli-reference): lista completa de flags e comandos incluindo `claude remote-control`
* [Segurança](/docs/pt/security): como as sessões de Remote Control se encaixam no modelo de segurança do Claude Code
* [Uso de dados](/docs/pt/data-usage): quais dados fluem através da API Anthropic durante sessões locais e remotas
