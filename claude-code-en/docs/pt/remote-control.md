> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Continue sessões locais de qualquer dispositivo com Remote Control

> Continue uma sessão local do Claude Code do seu telefone, tablet ou qualquer navegador usando Remote Control. Funciona com claude.ai/code e o aplicativo Claude para dispositivos móveis.

<Note>
  Remote Control está disponível em todos os planos. Em Team e Enterprise, ele fica desativado por padrão até que um Owner ative o toggle Remote Control nas [configurações de administrador do Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control conecta [claude.ai/code](https://claude.ai/code) ou o aplicativo Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) a uma sessão do Claude Code em execução na sua máquina. Inicie uma tarefa na sua mesa, depois continue a partir do seu telefone no sofá ou de um navegador em outro computador.

Quando você inicia uma sessão de Remote Control na sua máquina, Claude continua executando localmente o tempo todo, portanto sua execução de código e acesso ao sistema de arquivos permanecem na sua máquina. Com Remote Control você pode:

* **Usar seu ambiente local completo remotamente**: seu sistema de arquivos, [MCP servers](/docs/pt/mcp), ferramentas e configuração do projeto permanecem disponíveis, e digitar `@` autocompleta caminhos de arquivo do seu projeto local.
* **Trabalhar em ambas as superfícies ao mesmo tempo**: a conversa e o progresso de [subagentes](/docs/pt/sub-agents) e [fluxos de trabalho dinâmicos](/docs/pt/workflows) permanecem sincronizados em todos os dispositivos conectados, para que você possa enviar mensagens do seu terminal, navegador e telefone de forma intercambiável.
* **Enviar imagens e arquivos do seu telefone ou navegador**: anexe uma foto ou arquivo no aplicativo Claude ou em claude.ai/code, com ou sem legenda. Claude vê fotos anexadas diretamente como parte da sua mensagem. Claude Code faz o download de outros arquivos para sua máquina e os passa para Claude como referências de arquivo `@`.
* **Sobreviver a interrupções**: se seu laptop dormir ou sua rede cair, Claude Code se reconecta automaticamente quando sua máquina voltar a ficar online. Enquanto a conexão está sendo reconstruída, Claude Code enfileira mensagens, prompts de permissão e atualizações de status de subagentes e fluxos de trabalho, e as entrega assim que a conexão se recupera.

Diferentemente do [Claude Code na web](/docs/pt/claude-code-on-the-web), que é executado em infraestrutura em nuvem, as sessões de Remote Control são executadas diretamente na sua máquina e interagem com seu sistema de arquivos local. As interfaces web e móvel são uma janela para essa sessão local.

Esta página aborda a configuração, como iniciar e conectar a sessões, e como Remote Control se compara ao Claude Code na web.

<h2 id="requirements">
  Requisitos
</h2>

Antes de usar Remote Control, confirme que seu ambiente atende a estas condições:

* **Assinatura**: disponível nos planos Pro, Max, Team e Enterprise. Chaves de API não são suportadas. Em Team e Enterprise, um Owner deve primeiro ativar o toggle Remote Control nas [configurações de administrador do Claude Code](https://claude.ai/admin-settings/claude-code).
* **Autenticação**: execute `claude` e use `/login` para fazer login através de claude.ai se você ainda não fez isso. Sem um login elegível, `claude remote-control` sai com um erro, enquanto `claude --remote-control` ainda inicia uma sessão interativa e mostra uma notificação de falha de Remote Control logo após o lançamento.
* **Endpoint de API**: não disponível em nenhuma destas configurações:
  * Você usa Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.
  * Você aponta [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) para um host diferente de `api.anthropic.com`, como um [gateway LLM](/docs/pt/llm-gateway) ou proxy. Desative a variável para usar Remote Control. Antes da v2.1.196, Claude Code permitia Remote Control com um `ANTHROPIC_BASE_URL` customizado.
  * Você faz login através de um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) empresarial.
* **Avaliação de feature-flag**: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` e `DISABLE_GROWTHBOOK`](/docs/pt/env-vars) cada uma desabilita a avaliação de feature-flag da qual a disponibilidade de Remote Control depende. Desative a variável onde quer que esteja definida, no seu ambiente de shell ou no bloco `env` de um [arquivo `settings.json`](/docs/pt/settings-reference#all-settings), para usar Remote Control.
* **Confiança do workspace**: execute `claude` no diretório do seu projeto pelo menos uma vez para aceitar o diálogo de confiança do workspace. O diálogo de confiança na inicialização nunca salva confiança para seu diretório home, então inicie Remote Control a partir de um diretório de projeto.

<h2 id="start-a-remote-control-session">
  Inicie uma sessão de Remote Control
</h2>

Você pode iniciar uma sessão de Remote Control a partir da CLI ou da extensão VS Code. A CLI oferece três modos de invocação; VS Code usa o comando `/remote-control`.

<Tabs>
  <Tab title="Modo servidor">
    No diretório do seu projeto, execute:

    ```bash theme={null}
    claude remote-control
    ```

    Até que você aceite a confirmação única do Remote Control, `claude remote-control` explica o que faz e pergunta `Enable Remote Control? (y/n)` antes de iniciar o servidor. Responda `y` para aceitar e iniciar o servidor. Se você recusar, Claude Code sai sem iniciar o servidor e pergunta novamente na próxima vez que você executar o comando.

    O processo continua em execução no seu terminal em modo servidor, aguardando conexões remotas. Ele exibe uma URL de sessão que você pode usar para [conectar de outro dispositivo](#connect-from-another-device), e você pode pressionar a barra de espaço para mostrar um código QR para acesso rápido do seu telefone. Enquanto uma sessão remota está ativa, o terminal mostra o status da conexão e a atividade da ferramenta.

    Sinalizadores disponíveis:

    | Sinalizador                                     | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
    | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Define um título de sessão personalizado visível na lista de sessões em claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
    | `--remote-control-session-name-prefix <prefix>` | Prefixo para nomes de sessão gerados automaticamente quando nenhum nome explícito é definido. O padrão é o nome do host da sua máquina, produzindo nomes como `myhost-graceful-unicorn`. Defina `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` para o mesmo efeito.                                                                                                                                                                                                                                                                          |
    | `-c`, `--continue`                              | Retome a sessão que o último servidor neste diretório iniciou, em vez de criar uma nova. Consulte [Retomar sessões após parar o servidor](#resume-sessions-after-stopping-the-server). Não pode ser combinado com `--session-id`, `--spawn`, `--capacity` ou `--create-session-in-dir`. Requer Claude Code v2.1.200 ou posterior; versões anteriores rejeitam o sinalizador como um argumento desconhecido.                                                                                                                               |
    | `--session-id <id>`                             | Retome uma sessão pelo seu ID. Consulte [Retomar sessões após parar o servidor](#resume-sessions-after-stopping-the-server). Não pode ser combinado com `--continue`, `--spawn`, `--capacity` ou `--create-session-in-dir`. Requer Claude Code v2.1.200 ou posterior; versões anteriores rejeitam o sinalizador como um argumento desconhecido.                                                                                                                                                                                           |
    | `--spawn <mode>`                                | Como o servidor cria sessões.<br />• `same-dir` (padrão): todas as sessões compartilham o diretório de trabalho atual, portanto podem entrar em conflito se editarem os mesmos arquivos.<br />• `worktree`: cada sessão sob demanda obtém seu próprio [git worktree](/docs/pt/worktrees). Requer um repositório git.<br />• `session`: modo de sessão única. Serve exatamente uma sessão e rejeita conexões adicionais. Definido apenas na inicialização.<br />Pressione `w` em tempo de execução para alternar entre `same-dir` e `worktree`. |
    | `--capacity <N>`                                | Número máximo de sessões simultâneas. O padrão é 32. Não pode ser usado com `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
    | `--[no-]create-session-in-dir`                  | Pré-crie uma sessão no diretório atual quando o servidor inicia, para que você tenha um lugar para digitar imediatamente. Em modo `worktree`, essa sessão permanece no diretório atual enquanto as sessões sob demanda obtêm worktrees isoladas. Ativado por padrão. Se você passar `--no-create-session-in-dir` para iniciar sem nenhuma, Claude Code arquiva as sessões do servidor quando você o para, portanto não há nada para [retomar](#resume-sessions-after-stopping-the-server).                                                |
    | `--permission-mode <mode>`                      | Define o [modo de permissão](/docs/pt/permission-modes) inicial para as sessões do servidor, como `acceptEdits`. Aceita `manual` como um alias para `default`; um modo não reconhecido para o servidor na inicialização e lista os modos válidos.                                                                                                                                                                                                                                                                                              |
    | `--debug-file <path>`                           | Escreva logs de depuração no arquivo fornecido.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
    | `--verbose`                                     | Mostra logs detalhados de conexão e sessão.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
    | `--sandbox` / `--no-sandbox`                    | Ativa ou desativa [sandboxing](/docs/pt/sandboxing) para isolamento de sistema de arquivos e rede. Desativado por padrão.                                                                                                                                                                                                                                                                                                                                                                                                                      |

    Forneça esses sinalizadores após `remote-control`.

    Se você passar um sinalizador global `claude` antes de `remote-control`, ou um script wrapper adicionar um, Claude Code não carrega o sinalizador para as sessões que o servidor cria. Claude Code deixa o sinalizador passar apenas quando descartá-lo é conhecido por não alterar o que essas sessões podem fazer, como `--verbose` ou `--model`. Para qualquer outro sinalizador, como `--settings`, Claude Code [recusa iniciar](/docs/pt/errors#not-carried-over-to-the-sessions-remote-control-starts) e nomeia o sinalizador a remover. Antes da v2.1.248, qualquer opção antes de `remote-control` fazia Claude Code rejeitar os sinalizadores após ela com um erro `unknown option`.

    Claude Code verifica a elegibilidade do Remote Control antes de imprimir a ajuda, portanto `claude remote-control --help` retorna um erro em vez desta lista de sinalizadores quando você não está conectado com uma conta elegível.
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

    Até que você aceite a confirmação única do Remote Control, um diálogo aparece antes de `/remote-control` se conectar. Selecione **Enable Remote Control** para aceitar e conectar. Se você selecionar **Never mind** ou pressionar Esc, Claude Code não se conecta e pergunta novamente na próxima vez que você executar `/remote-control`.

    As flags `--verbose`, `--sandbox` e `--no-sandbox` não estão disponíveis com este comando.
  </Tab>

  <Tab title="VS Code">
    Na [extensão VS Code do Claude Code](/docs/pt/vs-code), digite `/remote-control` ou `/rc` na caixa de prompt.

    ```text theme={null}
    /remote-control
    ```

    Enquanto Remote Control está ativado, Claude Code mostra um indicador **Remote Control** no rodapé da caixa de prompt. Depois que a sessão se conecta, clique no indicador para ir diretamente para a sessão, ou encontre-a na lista de sessões em [claude.ai/code](https://claude.ai/code). Claude Code também publica a URL da sessão na conversa. Para desconectar, execute `/remote-control` novamente.

    Diferentemente da CLI, o comando VS Code não aceita um argumento de nome ou exibe um código QR. O título da sessão é derivado do seu histórico de conversa ou primeiro prompt.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Verificar status da conexão
</h3>

Em uma sessão interativa, enquanto Remote Control está conectado, o terminal mostra um indicador `/rc active` que vincula à sessão em claude.ai. O indicador fica oculto quando o terminal é muito estreito para ajustá-lo. Para ver a URL da sessão e um código QR para [conectar de outro dispositivo](#connect-from-another-device), execute `/remote-control` novamente para abrir o painel de status. O painel também permite que você desconecte Remote Control enquanto sua sessão local continua em execução.

<span id="session-ended-elsewhere" />Se a conexão falhar em uma sessão interativa, o indicador muda para mostrar a falha, e Claude Code mostra o motivo em uma notificação e o adiciona à conversa. Execute `/remote-control` para reconectar, a menos que o motivo diga que a sessão mudou em outro lugar:

* **Outra conexão assumiu esta sessão**: outro dispositivo ou sessão do Claude Code a tem agora. Execute `/remote-control` apenas se você quiser recuperá-la.
* **Esta sessão foi encerrada ou arquivada de outro dispositivo ou aplicativo**: execute `/remote-control` apenas se você quiser a sessão de volta. Claude Code reabre uma sessão arquivada.
* **O servidor não relata mais esta sessão**: ela pode ter sido deletada de outro dispositivo ou aplicativo.

<h3 id="session-url-reminders">
  Lembretes de URL de sessão
</h3>

Enquanto Remote Control está conectado, Claude Code o lembra da URL da sessão ao mudar para seu telefone ou navegador ajuda mais, para que você não tenha que encontrar o link em `/remote-control`. Um lembrete aparece acima da caixa de prompt em um destes momentos:

* **Turno longo**: quando um turno é executado por mais tempo que um limite ajustado pelo servidor, Claude Code mostra uma notificação **Still working** com um link **Check in from your phone**, para que você possa acompanhar o turno do seu telefone ou navegador em vez de esperar no terminal. Claude Code a remove quando o turno termina.
* **Prompts de permissão repetidos**: depois que você responde a vários [prompts de permissão](/docs/pt/permissions) em uma sessão, uma notificação **Approve tool calls from your phone** mostra a URL da sessão. Claude Code a remove quando seu próximo turno começa.

Os lembretes podem aparecer em qualquer sessão conectada, incluindo aquelas onde Remote Control [se conecta automaticamente](#enable-remote-control-for-all-sessions). Eles não aparecem toda vez que essas condições ocorrem, e cada um aparece apenas algumas vezes no total entre sessões. Você não pode configurar ou desativá-los; cada um se limpa por conta própria.

<h3 id="connect-from-another-device">
  Conectar de outro dispositivo
</h3>

Depois que uma sessão de Remote Control está ativa, você tem algumas maneiras de conectar de outro dispositivo:

* **Abra a URL da sessão** em qualquer navegador para ir diretamente para a sessão em [claude.ai/code](https://claude.ai/code).
* **Escaneie o código QR** mostrado ao lado da URL da sessão para abri-lo diretamente no aplicativo Claude. Com `claude remote-control`, pressione a barra de espaço para alternar a exibição do código QR.
* **Abra [claude.ai/code](https://claude.ai/code) ou o aplicativo Claude** e encontre a sessão pelo nome na lista de sessões. No aplicativo móvel Claude, toque em **Code** na navegação para acessar a lista de sessões. As sessões de Remote Control mostram um ícone de computador com um ponto de status verde quando online.

Quando você se conecta, o dispositivo mostra quaisquer subagentes e fluxos de trabalho que a sessão já tem em execução em segundo plano. Pare um deles do dispositivo, e Claude Code para essa tarefa em sua máquina.

O título da sessão remota é escolhido nesta ordem:

1. O nome que você passou para `--name`, `--remote-control` ou `/remote-control`
2. O título que você definiu com `/rename`
3. A última mensagem significativa no histórico de conversa existente
4. Um nome gerado automaticamente como `myhost-graceful-unicorn`, onde `myhost` é o nome do host da sua máquina ou o prefixo que você definiu com `--remote-control-session-name-prefix`

Se você não definir um nome explícito, Claude Code atualiza o título para refletir seu prompt assim que você enviar um. Claude Code corresponde títulos gerados automaticamente ao idioma da sua conversa, ou à configuração [`language`](/docs/pt/settings-reference#language) se uma estiver configurada.

Quando você renomeia uma sessão a partir de claude.ai ou do aplicativo Claude, Claude Code também atualiza o título local mostrado em `claude --resume`. Claude Code aplica o mesmo renome ao nome da sessão mostrado na barra de prompt, e na listagem `claude agents` quando a sessão [é executada em segundo plano](/docs/pt/agent-view). Antes da v2.1.221, renomear a partir da lista de sessões em claude.ai ou no aplicativo Claude atualizava apenas o título, e a CLI mantinha seu nome de sessão anterior; `/rename`, que é executado na própria CLI, define o nome em qualquer versão.

Se você ainda não tem o aplicativo Claude, execute `/mobile` dentro do Claude Code para mostrar um código QR para [claude.ai/mobile](https://claude.ai/mobile), que abre a loja de aplicativos correta para seu telefone.

<h3 id="what-connected-devices-see">
  O que dispositivos conectados veem
</h3>

Um dispositivo conectado mostra a conversa no seu terminal conforme acontece. Estes casos vão além de mensagens ordinárias:

* **Compactação e `/clear`**: enquanto Claude Code [compacta a conversa](/docs/pt/context-window#what-survives-compaction), dispositivos conectados mostram o progresso e então onde a conversa foi compactada. Quando você executa `/clear`, a conversa é redefinida em dispositivos conectados também.
* **Alternando conversas com `/resume`**: o dispositivo conectado não recebe o título da conversa alternada ou histórico anterior, mas novas mensagens em ambas as direções vão para e vêm de qualquer conversa que esteja aberta no seu terminal. Para trabalhar na conversa original do dispositivo novamente, execute `/resume` no seu terminal e volte para ela.
* **Puxando uma sessão com `/teleport`**: quando você puxa uma [sessão Claude Code na web](/docs/pt/claude-code-on-the-web#from-cloud-to-terminal) para seu terminal com `/teleport`, o dispositivo conectado não recebe o histórico anterior da conversa puxada. Novas mensagens em ambas as direções vão para e vêm da conversa puxada, que agora é a que está aberta no seu terminal.
* **Mensagens de suas outras sessões**: com [mensagens entre sessões](/docs/pt/cross-session-messaging), a mesma conexão carrega mensagens entre suas próprias sessões em diferentes máquinas e de suas sessões [Claude Code na web](/docs/pt/claude-code-on-the-web), através de servidores Anthropic como o resto do tráfego Remote Control. [Mensagens de sessões em outras máquinas](/docs/pt/cross-session-messaging#message-sessions-on-other-machines) cobre as regras de entrega e [Controlar mensagens de entrada](/docs/pt/cross-session-messaging#control-inbound-messages) cobre os controles de entrada. Requer Claude Code v2.1.224 ou posterior.
* **Prompts que você envia no meio do turno**: quando você envia um prompt de um dispositivo conectado antes do turno atual terminar, Claude Code o coloca na fila e o mantém na transcrição do dispositivo depois que esse turno termina.
* **Diff de suas alterações**: quando o diretório da sessão está em um repositório git, o painel de diff de um dispositivo conectado mostra suas alterações. O dispositivo solicita o diff pela conexão, e Claude Code o computa em sua máquina. Em um branch que tem commits à frente do branch padrão do repositório, o painel mostra as alterações desde que o branch divergiu dele, incluindo suas edições não confirmadas. No branch padrão em si, ou em um branch que não está à frente dele, o painel mostra apenas suas alterações não confirmadas. Antes da v2.1.247, Claude Code relatava o diff para dispositivos conectados apenas em sessões servidas por `claude remote-control`.
* **Modelo**: quando você escolhe um [modelo](/docs/pt/model-config) de um dispositivo conectado, Claude Code executa a sessão nesse modelo. O seletor `/model` do terminal, `/status` e `/config` mostram esse modelo. Requer Claude Code v2.1.238 ou posterior.
  * Um modelo que você escolhe do controle de modelo do dispositivo se aplica apenas à sessão atual. Quando você envia `/model <name>` do dispositivo para uma sessão interativa, Claude Code também define seu padrão para novas sessões.
  * Se você enviar um nome que Claude Code não reconheça, como um nome de exibição onde um ID de modelo é esperado, Claude Code [recusa a escolha](/docs/pt/errors#model-is-not-a-recognized-model-id) e a sessão mantém seu modelo atual. Antes da v2.1.260, Claude Code salvava uma escolha não reconhecida do controle de modelo do dispositivo, e sua próxima mensagem falhava.
* **Nível de esforço**: quando você define o [nível de esforço](/docs/pt/model-config#adjust-effort-level) de um dispositivo conectado, com `/effort` ou o controle de esforço do dispositivo, Claude Code o aplica à sessão em sua máquina, e claude.ai/code mostra o nível que a sessão está usando. Se você fixou um nível com `CLAUDE_CODE_EFFORT_LEVEL`, a sessão mantém esse nível, e Claude Code recusa uma escolha diferente do controle de esforço. Escolher um nível do controle de esforço requer Claude Code v2.1.234 ou posterior em sua máquina.
* **Reconectando após uma falha de conexão**: execute `/remote-control` para reconectar. Se a compactação reescreveu a conversa ou você alternou conversas com `/resume` enquanto isso, Claude Code arquiva a sessão do servidor que estava usando em vez de deixá-la na lista de sessões. Você ainda pode encontrá-la [filtrando por sessões arquivadas](/docs/pt/claude-code-on-the-web#archive-sessions). Alternar conversas enquanto um dispositivo ainda está conectado não arquiva a sessão.

<h3 id="enable-remote-control-for-all-sessions">
  Ativar Remote Control para todas as sessões
</h3>

Remote Control só é ativado quando você executa explicitamente `claude remote-control`, `claude --remote-control` ou `/remote-control`, a menos que a conexão automática esteja ativada. Para ativar a conexão automática para cada sessão interativa, execute `/config` dentro do Claude Code e defina **Enable Remote Control for all sessions**. O botão de alternância tem três valores:

* **`true`**: conectar automaticamente quando uma sessão interativa inicia.
* **`false`**: desativar a conexão automática, embora um `true` de [configurações gerenciadas](/docs/pt/managed-settings) o supere, porque Claude Code salva a escolha em suas configurações de usuário. Um `false` em configurações de projeto ou local (`.claude/settings.json`, `.claude/settings.local.json`) desativa a conexão automática mesmo sobre um `true` gerenciado.
* **`default`**: limpar sua escolha e seguir o padrão do administrador da sua organização se um estiver definido, caso contrário o padrão atual do Claude Code.

O mesmo botão de alternância aparece fora da CLI:

* **Aplicativo Desktop**: **Settings > Claude Code > Enable remote control by default**.
* **Extensão VS Code**: **Enable Remote Control for all sessions** na seção Configurações do [menu de comandos](/docs/pt/vs-code#use-the-prompt-box). Requer Claude Code v2.1.203 ou posterior.

Para ativar a conexão automática a partir de um arquivo de configurações em vez disso, defina [`remoteControlAtStartup`](/docs/pt/settings-reference#remotecontrolatstartup) como `true` em seu usuário `~/.claude/settings.json` ou em [configurações gerenciadas](/docs/pt/managed-settings). Em configurações de projeto ou local (`.claude/settings.json`, `.claude/settings.local.json`), Claude Code honra um `false` e desativa a conexão automática para esse repositório, mas ignora um `true`, para que um arquivo verificado não possa ativar Remote Control para todos que abrem o repositório.

A conexão automática se conecta com sua própria conta claude.ai, portanto uma sessão que ela inicia aparece apenas nos seus próprios aplicativos Claude e não concede acesso a ninguém mais.

Com essa configuração ativada, cada processo interativo do Claude Code registra uma sessão remota. Se você executar várias instâncias, cada uma obtém sua própria sessão remota. Para executar várias sessões simultâneas a partir de um único processo, use o [modo servidor](#start-a-remote-control-session) em vez disso.

<h3 id="resume-sessions-after-stopping-the-server">
  Retomar sessões após parar o servidor
</h3>

Quando você para `claude remote-control` com Ctrl+C, as sessões que ele estava servindo param de responder do seu telefone ou navegador. Contanto que você não estivesse executando outro `claude remote-control` no mesmo diretório e não iniciou este com `--no-create-session-in-dir`, Claude Code não as arquiva. Para trazê-las de volta, execute um destes comandos no mesmo diretório:

* **`claude remote-control`**: traz de volta cada sessão que o servidor estava servindo.
* **`claude remote-control --continue`**: traz de volta apenas a sessão que o servidor iniciou, e sai quando essa sessão termina. Se este diretório não tem registro, Claude Code usa o mais recente de outros git worktrees deste repositório.
* **`claude remote-control --session-id <id>`**: traz de volta apenas a sessão cujo ID você passa, e sai quando essa sessão termina. O ID é a parte da URL da sessão em claude.ai/code entre `/code/` e qualquer `?`.

Estes comandos funcionam por cerca de quatro horas após o servidor parar. Depois disso, execute `claude remote-control` para iniciar uma nova sessão. Se você arquivou uma sessão enquanto isso, `--continue` e `--session-id` a desarchivam em Claude Code v2.1.228 ou posterior.

Para trazer de volta uma sessão que você iniciou com `claude --remote-control` ou `/remote-control`, retome a conversa com `claude --continue` ou `claude --resume`. Se Claude Code se reconecta, e a qual sessão, depende do [registro de reconexão](#resume-outcomes) da conversa.

Se você retomar a conversa em um segundo terminal enquanto o primeiro ainda tem Remote Control ativado, Claude Code imprime um aviso no segundo terminal e deixa Remote Control desativado lá em vez de tirar a sessão do primeiro. Enquanto Remote Control fica desativado lá, Claude naquele terminal não vê [suas sessões em outras máquinas](/docs/pt/cross-session-messaging#see-which-sessions-claude-can-reach), e elas não conseguem alcançá-lo. Execute `/remote-control` no segundo terminal para mover Remote Control para ele.

Quando você retoma uma conversa no Claude Desktop ou em uma extensão IDE que tinha Remote Control ativado, Claude Code a reanexa à sessão claude.ai existente em vez de adicionar uma nova à lista de sessões.

<h2 id="connection-and-security">
  Conexão e segurança
</h2>

Sua sessão local do Claude Code faz apenas solicitações HTTPS de saída e nunca abre portas de entrada na sua máquina. Quando você inicia Remote Control, ele se registra na API Anthropic e faz polling para trabalho. Quando você conecta de outro dispositivo, o servidor roteia mensagens entre o cliente web ou móvel e sua sessão local através de uma conexão de streaming.

Todo o tráfego viaja através da API Anthropic sobre TLS, o mesmo transporte de segurança que qualquer sessão do Claude Code. A conexão usa múltiplas credenciais de curta duração, cada uma com escopo para um único propósito e expirando independentemente. Quando a credencial de registro de um servidor `claude remote-control` expira, o servidor se registra novamente na API Anthropic e continua servindo suas sessões.

Enquanto Remote Control está conectado, a transcrição da sessão, incluindo suas mensagens, respostas do Claude e atividade de ferramentas, é armazenada nos servidores Anthropic. A transcrição armazenada mantém a conversa sincronizada em seus dispositivos e permite que a sessão se reconecte após uma queda de rede. A execução e o acesso ao sistema de arquivos permanecem na sua máquina, e as transcrições armazenadas são retidas sob a política de [Uso de dados](/docs/pt/data-usage).

Para desativar Remote Control completamente, use a configuração [`disableRemoteControl`](/docs/pt/settings-reference#disableremotecontrol). Organizações com requisitos de conformidade, como Zero Data Retention, não podem ativar Remote Control.

<h2 id="trusted-devices">
  Dispositivos Confiáveis
</h2>

<Note>
  Dispositivos Confiáveis está atualmente em beta. Recursos e funcionalidades podem evoluir conforme a experiência é refinada.

  Dispositivos Confiáveis está disponível nos planos Pro, Max, Team e Enterprise e fica desativado por padrão. Nos planos Team e Enterprise, um Owner o ativa para a organização. Nos planos Pro e Max, você ativa **Require trusted devices** você mesmo nas suas configurações, na página Cowork ou Account.
</Note>

Dispositivos Confiáveis requer que cada membro da sua organização, ou você sozinho em um plano Pro ou Max, verifique seu dispositivo antes de poder visualizar ou controlar sessões de Remote Control a partir de claude.ai, dos aplicativos Claude para dispositivos móveis ou Claude Desktop. Ele vincula o acesso ao Remote Control a um dispositivo conhecido e uma autenticação recente, não apenas a uma conta conectada.

Quando a configuração está ativada, interagir com uma sessão de Remote Control requer ambos os seguintes:

* **Um dispositivo inscrito**: cada navegador, telefone ou aplicativo desktop que um membro usa para Remote Control inscreve sua própria credencial. A inscrição é oferecida apenas pouco tempo após um login completo, portanto um dispositivo entra na lista confiável como parte de uma autenticação real em vez de silenciosamente em segundo plano.
* **Um login recente**: o login do membro não deve ter mais de 18 horas. Em vez de fazer login novamente a cada dia, os membros confirmam presença com Face ID, Touch ID, Windows Hello ou uma passkey. Esta etapa de autenticação biométrica atualiza a sessão imediatamente.

Verificações biométricas são executadas no dispositivo através do sistema operacional ou navegador, o mesmo mecanismo que o login com passkey. Anthropic nunca recebe ou armazena impressões digitais, dados faciais ou qualquer outra informação biométrica. Apenas a chave pública do dispositivo e metadados básicos como nome de exibição, plataforma e hora de inscrição são armazenados.

A configuração se aplica apenas ao Remote Control. Chat regular do Claude, Claude Code no terminal e uso de API não são afetados.

<h3 id="enable-trusted-devices-for-your-organization">
  Ativar Dispositivos Confiáveis para uma organização Team ou Enterprise
</h3>

Um Owner ativa a configuração a partir das configurações de organização do claude.ai.

<Steps>
  <Step title="Acesse a página Capabilities">
    Vá para [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities). O toggle **Require trusted devices** aparece nessa seção.
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

<h2 id="remote-control-vs-cloud-sessions">
  Remote Control vs sessões na nuvem
</h2>

Remote Control e [sessões na nuvem](/docs/pt/claude-code-on-the-web) usam a interface claude.ai/code. A diferença fundamental é onde a sessão é executada: Remote Control é executado na sua máquina, portanto seus MCP servers locais, ferramentas e configuração do projeto permanecem disponíveis. Uma sessão na nuvem é executada na infraestrutura de nuvem, gerenciada pela Anthropic por padrão.

Use Remote Control quando você está no meio do trabalho local e deseja continuar de outro dispositivo. Use uma sessão na nuvem quando você deseja iniciar uma tarefa sem nenhuma configuração local, trabalhar em um repositório que você não tem clonado ou executar várias tarefas em paralelo.

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
* **O processo local deve continuar em execução**: Remote Control é executado como um processo local. Se você fechar o terminal, sair do VS Code ou parar o processo `claude`, a sessão fica offline até que você a [retome](#resume-sessions-after-stopping-the-server). A menos que Claude esteja no meio de uma tarefa, claude.ai e o aplicativo Claude mostram a sessão como offline segundos após o processo sair. Para manter uma sessão em execução em uma máquina remota após desconectar do SSH, inicie-a dentro de `tmux` ou `screen`.
* **Sessões travadas no modo servidor**: se uma sessão servida por `claude remote-control` travar, envie uma mensagem para ela a partir de um dispositivo conectado. Claude Code a serve novamente. Você não precisa reiniciar o servidor. Requer Claude Code v2.1.238 ou posterior.
* **Recusas HTTP 403 em uma sessão conectada**: uma vez que uma sessão interativa está conectada, Claude Code continua tentando por até três minutos quando algo entre sua máquina e os servidores da Anthropic responde com HTTP 403, o que pode acontecer após uma mudança de VPN ou rede. Se as recusas durarem mais tempo, Claude Code desconecta e o motivo nomeia o que recusou: uma borda de rede ou um proxy, VPN ou firewall em sua própria rede.
* **Interrupção de rede estendida**: se sua máquina estiver ligada mas não conseguir alcançar a rede, o que você faz a seguir depende do modo:
  * **Modo servidor**: Claude Code desiste após aproximadamente 10 minutos e o processo `claude remote-control` sai. Execute `claude remote-control` novamente para iniciar uma nova sessão.
  * **Sessão interativa**: continue trabalhando localmente. Claude Code tenta novamente enquanto a interrupção durar e se reconecta automaticamente quando a rede retorna.
* **Falhas de heartbeat de presença**: se uma sessão interativa desconectar com `could not reach the Remote Control server for about 30 minutes`, execute `/remote-control` para se reconectar. Claude Code mostra esta mensagem apenas quando os heartbeats de presença da sessão falharam enquanto o resto da conexão permaneceu ativo; ele registra novamente a sessão por aproximadamente 30 minutos antes de desconectar.
* **Diálogos encaminhados expiram**: Claude Code mantém prompts de permissão e perguntas `AskUserQuestion` abertas até que você as responda. Quando Claude Code encaminha outro tipo de diálogo para a sessão remota, como o prompt de escolha de modelo mostrado após uma recusa de segurança, ele aguarda cinco minutos por padrão, depois fecha o diálogo e continua com o padrão sem ação do diálogo. Defina [`dialogExpiry`](/docs/pt/settings-reference#dialogexpiry) para ajustar ou desabilitar o prazo. Requer Claude Code v2.1.224 ou posterior.
* **O prompt de consentimento de créditos de uso Fable não é encaminhado**: Claude Code mostra o prompt de consentimento de créditos de uso [Fable](/docs/pt/model-config#fable-and-usage-credits) no meio da sessão apenas onde a sessão é executada, não no seu dispositivo. Quando a sessão é executada em um terminal e ninguém lá responde antes de Claude Code fechar o prompt, a volta termina sem enviar a solicitação; veja [O prompt para confirmar não foi respondido](/docs/pt/errors#the-prompt-to-confirm-went-unanswered).
* **Alguns comandos são apenas locais**: comandos que funcionam apenas na interface do terminal, como `/plugin` ou `/resume`, funcionam apenas a partir da CLI local, independentemente de você passar um argumento ou não. Os seguintes funcionam a partir de dispositivos móveis e web:
  * Comandos de saída de texto: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits`, `/recap` e `/reload-plugins`. `/usage-credits` imprime a URL de faturamento em vez de abrir um navegador. `/reload-plugins` funciona apenas quando a sessão é executada em um terminal interativo; uma sessão sem um recusa.
  * `/model`, `/effort`, `/fast`, `/color` e `/rename`: passe o valor como um argumento, por exemplo `/model sonnet` ou `/effort high`. A partir de dispositivos móveis e web, `/model` e `/effort` recebem o argumento no lugar do seletor do terminal ou controle deslizante.
  * `/mcp`: a partir do aplicativo móvel, retorna um resumo de texto do status do servidor em vez de abrir o seletor. Na web, `/mcp` sozinho abre um diretório de [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) em vez de retornar o resumo. Os [subcomandos](/docs/pt/commands#all-commands) `reconnect`, `enable` e `disable` funcionam em ambos. Diferentemente da CLI local, `/mcp reconnect` sem um nome de servidor reconecta todos os servidores que falharam ou precisam de autenticação.
  * `/config`: a partir do aplicativo móvel, passe `key=value` para definir uma configuração, ou execute sem argumentos para listar as chaves que você pode definir. Na web, `/config` abre a seção Claude Code das suas configurações e ignora o texto após o comando.
  * No Team e Enterprise, `/usage-credits` a partir de dispositivos móveis ou web não envia uma [solicitação de créditos de uso para seu administrador](/docs/pt/costs#add-usage-credits-to-your-subscription). O envio requer uma confirmação que aparece apenas na CLI interativa, então o comando diz para você executá-lo lá. Antes da v2.1.211, o formulário de texto enviava a solicitação sem confirmação.
  * `/autocompact`, a partir da v2.1.221: passe o tamanho da janela como um argumento, por exemplo `/autocompact 500k`. Sem argumento, ele imprime o tamanho da janela atual como texto em vez de abrir o diálogo que o comando mostra em uma sessão de terminal.
  * `/advisor`, a partir da v2.1.260: passe o modelo como um argumento, por exemplo `/advisor opus`, ou passe `off` para desativar o advisor. Ambas as formas se aplicam apenas à sessão atual e deixam seu padrão salvo inalterado. Sem argumento, ele imprime o advisor atual como texto em vez de abrir o seletor.
  * `/output-style`, a partir da v2.1.269: passe o nome do estilo como um argumento, por exemplo `/output-style concise`, ou execute sem argumento para listar os estilos. A partir de dispositivos móveis e web, você pode listar e selecionar apenas [estilos integrados](/docs/pt/output-styles#built-in-output-styles). Para usar um [estilo personalizado](/docs/pt/output-styles#create-a-custom-output-style), selecione-o na sessão em si.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control requires a claude.ai subscription"
</h3>

Você não está autenticado com uma conta claude.ai, ou outra credencial está tendo precedência sobre seu login. A mensagem assume uma destas formas:

* Desconectado, de `/remote-control` ou `--remote-control`: `Remote Control requires a claude.ai subscription.` ou `/remote-control requires a claude.ai subscription.`
* Desconectado, de `claude remote-control`: `You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions.`
* Autenticado, mas uma chave de API ou token está em uso: `Remote Control requires claude.ai subscription auth.` seguido pela credencial em uso, como `ANTHROPIC_API_KEY is set, so this session is using API-key auth`. Uma configuração `apiKeyHelper` e `ANTHROPIC_AUTH_TOKEN` são nomeadas da mesma forma.

Execute `claude auth login` e escolha a opção claude.ai. Se a mensagem nomear `ANTHROPIC_API_KEY` ou `ANTHROPIC_AUTH_TOKEN`, remova-a onde quer que esteja definida: seu ambiente de shell ou o bloco `env` de um [arquivo de configurações](/docs/pt/settings-reference#env). Se nomear `apiKeyHelper`, remova essa configuração.

Antes da v2.1.206, executar `/remote-control` enquanto desconectado relatava `Unknown command: /remote-control` em vez desta mensagem.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control requires a full-scope login token"
</h3>

Você está autenticado com um token de longa duração de `claude setup-token` ou da variável de ambiente `CLAUDE_CODE_OAUTH_TOKEN`. Esses tokens podem apenas fazer solicitações de modelo, então não podem estabelecer sessões de Remote Control. Execute `claude auth login` para autenticar com um token de sessão de escopo completo em vez disso.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Unable to determine your organization for Remote Control eligibility"
</h3>

Suas informações de conta em cache estão desatualizadas ou incompletas. Execute `claude auth login` para atualizá-las.

<h3 id="remote-control-isn’t-enabled-for-this-account">
  "Remote Control isn't enabled for this account"
</h3>

Claude Code verificou a disponibilidade de Remote Control para a conta com a qual você está autenticado e a verificação retornou desativada. A causa usual é direitos em cache que estão desatualizados após uma mudança de plano. Execute `claude auth logout` e depois `claude auth login` para atualizá-los, e atualize Claude Code se você estiver em uma versão antiga.

Execute `claude doctor` para ver qual verificação de elegibilidade individual falhou. Conflitos de variáveis de ambiente, verificações inacessíveis e a configuração de Remote Control da sua organização cada um produzem sua própria mensagem, então este erro significa que a verificação no nível da conta em si.

Antes da v2.1.239, esta mensagem lia "Remote Control is not yet enabled for your account". Antes da v2.1.154, uma variável que desativa a avaliação de sinalizador de recurso, como `DISABLE_TELEMETRY` ou `DO_NOT_TRACK`, também produzia esta mensagem; a entrada "Remote Control requires feature-flag evaluation" abaixo cobre essa configuração.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Couldn't verify Remote Control eligibility"
</h3>

Claude Code não conseguiu alcançar o serviço de sinalizador de recurso para verificar se Remote Control está habilitado para sua conta, normalmente porque você está offline ou um proxy está bloqueando a solicitação. Tente novamente quando tiver acesso à rede, ou execute `claude doctor` para obter detalhes. A mensagem relacionada "Couldn't verify your organization's Remote Control policy" significa que Claude Code não conseguiu ler essa política, e tem a mesma solução. Ambas as mensagens foram adicionadas na v2.1.178.

<h3 id="remote-control-requires-feature-flag-evaluation">
  "Remote Control requires feature-flag evaluation"
</h3>

Uma destas variáveis está definida: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, ou `DISABLE_GROWTHBOOK`](/docs/pt/env-vars). Cada uma delas desativa a avaliação de sinalizador de recurso da qual a disponibilidade de Remote Control depende, e a mensagem completa nomeia a variável que Claude Code encontrou. Desative essa variável onde quer que esteja definida, em seu ambiente de shell ou no bloco `env` de um [arquivo `settings.json`](/docs/pt/settings-reference#all-settings). Em versões anteriores a 2.1.154, a mesma configuração produz "Remote Control is not yet enabled for your account" em vez disso.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control is only available when using Claude via api.anthropic.com"
</h3>

A sessão não está se comunicando diretamente com a API Anthropic, então não há backend claude.ai para emparelhar. Isso acontece no Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. Também acontece quando [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) aponta para um host diferente de `api.anthropic.com`, como um [gateway LLM](/docs/pt/llm-gateway) ou proxy, mesmo se você entrar com claude.ai. Antes da v2.1.196, Claude Code não mostrava esta mensagem para um `ANTHROPIC_BASE_URL` personalizado. Veja a [referência de erros](/docs/pt/errors#remote-control-requires-the-anthropic-api) para a lista completa de causas.

A mensagem nomeia o que roteou a sessão para longe da API Anthropic, como `CLAUDE_CODE_USE_BEDROCK` ou um `ANTHROPIC_BASE_URL` personalizado. Se você tiver um login claude.ai elegível, desative a variável nomeada, remova-a da chave `env` em [configurações](/docs/pt/settings) se você a definiu lá, e reinicie a sessão. Antes da v2.1.219, a mensagem era apenas a sentença no cabeçalho desta seção, então em versões mais antigas verifique seu ambiente você mesmo para variáveis de provedor como `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, e para `ANTHROPIC_BASE_URL`.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control is disabled by your organization's policy"
</h3>

Uma política bloqueia Remote Control, ou Claude Code não conseguiu carregar a política da sua organização nesta máquina e mantém Remote Control desativado enquanto isso. Verifique estas causas em ordem:

* **O erro menciona `disableRemoteControl`**: seu administrador de TI desativou Remote Control neste dispositivo através de [configurações gerenciadas](/docs/pt/managed-settings), independentemente do toggle em toda a organização e de como você está autenticado.
* **Seu plano claude.ai é Pro ou Max**: Claude Code ainda está autenticado sob uma organização Team ou Enterprise de um login anterior, então verifica a política de Remote Control dessa organização. Execute `/status` para ver qual plano e organização seu login usa. Execute `claude auth logout` e depois `claude auth login` para entrar novamente sob seu plano atual.
* **A política da organização não foi carregada nesta máquina**: execute `claude doctor` e leia a linha `Organization policy`. Se a linha mostrar que a política não está carregada, é isso que está mantendo Remote Control desativado. Antes da v2.1.261, `claude doctor` não imprimia esta linha.
* **A mensagem não diz para entrar em contato com seu administrador da organização**: sua organização tem uma configuração HIPAA que é incompatível com Remote Control, e `/status` lista `HIPAA` em sua linha `Compliance`. Neste estado, o toggle de Remote Control do painel de administração fica acinzentado, então um Proprietário não pode alterá-lo lá. Entre em contato com o suporte da Anthropic para discutir opções. Antes da v2.1.267, este caso mostrava "Remote Control isn't available for your organization due to its compliance policy" em vez disso.
* **Caso contrário, um Proprietário não ativou para sua organização**: Remote Control fica desativado por padrão nos planos Team e Enterprise. Um Proprietário pode ativá-lo em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) ativando o toggle **Remote Control**. Este toggle é uma configuração de organização no lado do servidor.

<h3 id="remote-credentials-fetch-failed">
  "Remote credentials fetch failed"
</h3>

Claude Code não conseguiu obter uma credencial de curta duração da API Anthropic para estabelecer a conexão. Execute novamente com `--verbose` para ver o erro completo:

```bash theme={null}
claude remote-control --verbose
```

Causas comuns:

* Não autenticado: execute `claude` e use `/login` para autenticar com sua conta claude.ai. A autenticação por chave de API não é suportada para Remote Control.
* Problema de rede ou proxy: um firewall ou proxy pode estar bloqueando a solicitação HTTPS de saída. Remote Control requer acesso à API Anthropic na porta 443.
* Falha na criação de sessão: se você também vir `Session creation failed — see debug log`, a falha aconteceu anteriormente na configuração. Verifique se sua assinatura está ativa.

Um token de login desatualizado não causa este erro. Quando a API Anthropic rejeita o token salvo, por exemplo porque outro processo Claude Code já o atualizou, Claude Code atualiza o token e tenta novamente por conta própria. Antes da v2.1.224, um token desatualizado falhava na inicialização de Remote Control com esta mensagem, então sessões definidas para [conectar automaticamente](#enable-remote-control-for-all-sessions) poderiam falhar intermitentemente na inicialização.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Couldn't reconnect to your Remote Control session"
</h3>

Quando você retoma uma conversa com `claude --resume` ou `claude --continue`, Claude Code se reconecta à sessão de Remote Control registrada nessa conversa. Esta mensagem significa que a reconexão falhou por um motivo que pode ser temporário, como uma interrupção de rede ou um erro de servidor, então Claude Code não pode confirmar se a sessão remota ainda existe.

Execute `/remote-control` para tentar novamente a conexão, ou inicie uma nova sessão com `claude --remote-control` para criar uma nova sessão de Remote Control. Sua sessão local continua funcionando sem Remote Control enquanto isso.

<span id="resume-outcomes" />Quando você retoma, você também pode obter um destes resultados em vez desta mensagem:

* **O servidor relata a sessão registrada desaparecida, ou o registro de reconexão nomeia uma conta diferente**: Claude Code vai pelo que o registro de reconexão da conversa diz:
  * **O registro nomeia sua conta autenticada**: Claude Code inicia uma sessão de substituição com um nome gerado automaticamente e deixa as mensagens anteriores da conversa fora dela. Você obtém isso depois de deletar a sessão de claude.ai ou do aplicativo Claude, por exemplo.
  * **O registro nomeia uma conta diferente**: Claude Code inicia uma nova sessão sem as mensagens anteriores da conversa e sem mostrar uma mensagem, independentemente de a sessão registrada ainda existir.
  * **O registro não diz qual conta possuía a sessão, ou Claude Code não consegue ler seu login salvo**: Claude Code mostra [`Previous session is unavailable — run /remote-control to start a new one`](#previous-session-is-unavailable) em vez desta mensagem, não inicia nada, e remove o registro da conversa.
* **Você desativou Remote Control antes de retomar**: a menos que o aplicativo hospedando Claude Code tivesse dito a ele que o aplicativo possui a sessão claude.ai, Claude Code removeu o registro de reconexão quando você desativou Remote Control do [painel de status do CLI](#check-connection-status), da extensão VS Code, ou de um host construído no [Agent SDK](/docs/pt/agent-sdk/overview), então não se reconecta. Quando um aplicativo proprietário desativou, Claude Code manteve o registro e se reconecta.
* **Outro Claude Code nesta máquina ainda tem a sessão**: você vê um aviso que começa com `Remote Control not started here`, e Claude Code [deixa Remote Control desativado na sessão retomada](#resume-sessions-after-stopping-the-server). Execute `/remote-control` lá para movê-lo.

<span id="reconnect-history" />Antes da v2.1.232, Claude Code respondeu diferentemente quando o servidor relatou a sessão registrada desaparecida. De v2.1.227 até v2.1.231, Claude Code recusou iniciar uma substituição mesmo quando o registro correspondia à sua conta. Até v2.1.226, Claude Code iniciou uma substituição independentemente de o registro corresponder à sua conta, e em v2.1.224 até v2.1.226 a criou sob a conta autenticada naquela máquina, nunca de outra conta, sem fazer upload das mensagens anteriores da conversa para ela. Antes da v2.1.200, Claude Code criava uma nova sessão após qualquer falha de reconexão.

<h3 id="previous-session-is-unavailable">
  "Previous session is unavailable — run /remote-control to start a new one"
</h3>

Claude Code não conseguiu trazer de volta a sessão anterior de Remote Control e parou em vez de iniciar uma nova por conta própria. Você pode ver esta mensagem depois de retomar uma conversa com `claude --resume` ou `claude --continue`, ou depois que Claude Code [se reconecta por conta própria após uma desconexão](/docs/pt/errors#remote-control-couldnt-refresh-your-login).

Execute `/remote-control` para iniciar uma nova sessão de Remote Control sob o login atual; sua sessão local continua funcionando sem Remote Control enquanto isso. A mensagem relacionada `Remote Control could not verify the signed-in account — run /remote-control to reconnect` tem a mesma solução; Claude Code a mostra quando a conta autenticada mudou ou não conseguiu ser lida entre validá-la e se reconectar. Se você executar `/remote-control` após `Previous session is unavailable` sem reiniciar Claude Code primeiro, Claude Code deixa as mensagens anteriores da conversa fora da nova sessão.

Na retomada, Claude Code [inicia uma nova sessão em seu lugar](#resume-outcomes) apenas se o registro de reconexão da conversa nomear a conta que possuía a sessão, porque o servidor relata uma sessão que você deletou e uma sessão possuída por outra conta da mesma forma. Claude Code antes da v2.1.227 não registrou essa conta, e Claude Code não pode verificar o registro quando não consegue ler seu login salvo. Claude Code antes da v2.1.232 mostrou `Remote Control could not resume the previous session under the current login — run /remote-control to start fresh` em vez disso, em [um conjunto diferente de casos](#reconnect-history).

<h3 id="remote-control-got-an-unexpected-server-response">
  "Remote Control got an unexpected server response"
</h3>

O servidor de Remote Control aceitou uma solicitação mas respondeu de uma forma que esta versão de Claude Code não conseguiu ler, ao criar a sessão remota ou buscar suas credenciais. Tentar novamente na mesma versão falha da mesma forma. Execute `claude update`, depois execute `/remote-control` para se reconectar. Esta mensagem foi adicionada na v2.1.225.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  "Your organization requires Trusted Devices for Remote Control, but this device is not enrolled"
</h3>

Sua organização tem [Trusted Devices](#trusted-devices) ativado e esta máquina não se inscreveu ainda. Execute `/login` no Claude Code. A inscrição acontece como parte do login, e não há comando de inscrição separado.

<h3 id="session-expired-for-trusted-device-check">
  "session expired for trusted-device check"
</h3>

Seu login tem mais de 18 horas. Execute `/login` no Claude Code, ou confirme com Face ID, Touch ID, Windows Hello, ou uma passkey quando claude.ai ou o aplicativo móvel solicitar. Veja [Trusted Devices](#trusted-devices).

<h2 id="choose-the-right-approach">
  Escolha a abordagem correta
</h2>

Claude Code oferece várias maneiras de trabalhar quando você não está no seu terminal. Elas diferem no que dispara o trabalho, onde Claude é executado e quanto você precisa configurar.

|                                                          | Gatilho                                                                                                          | Claude é executado em                                                                        | Configuração                                                                                                                         | Melhor para                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------- |
| [Dispatch](/docs/pt/desktop#sessions-from-dispatch)           | Envie uma tarefa a partir do aplicativo móvel Claude                                                             | Sua máquina (Desktop)                                                                        | [Emparelhe o aplicativo móvel com Desktop](https://support.claude.com/en/articles/13947068)                                          | Delegar trabalho enquanto você está ausente, configuração mínima |
| [Remote Control](/docs/pt/remote-control)                     | Dirija uma sessão em execução a partir de [claude.ai/code](https://claude.ai/code) ou do aplicativo móvel Claude | Sua máquina (CLI ou VS Code)                                                                 | Execute `claude remote-control`                                                                                                      | Orientar trabalho em andamento de outro dispositivo              |
| [Channels](/docs/pt/channels)                                 | Envie eventos de um aplicativo de chat como Telegram ou Discord, ou seu próprio servidor                         | Sua máquina (CLI)                                                                            | [Instale um plugin de canal](/docs/pt/channels#quickstart) ou [crie o seu próprio](/docs/pt/channels-reference)                                | Reagir a eventos externos como falhas de CI ou mensagens de chat |
| [Slack](/docs/pt/slack)                                       | Mencione `@Claude` em um canal de equipe                                                                         | Nuvem Anthropic                                                                              | [Instale o aplicativo Slack](/docs/pt/slack#setting-up-claude-code-in-slack) com [Claude Code na web](/docs/pt/claude-code-on-the-web) ativado | PRs e revisões do chat da equipe                                 |
| [Self-hosted environments](/docs/pt/self-hosted-environments) | Inicie uma [sessão na nuvem](/docs/pt/claude-code-on-the-web) e escolha o ambiente da sua organização                 | Infraestrutura da sua organização                                                            | [Implante runners](/docs/pt/self-hosted-environments-quickstart), em planos Team e Enterprise                                             | Sessões na nuvem que devem ser executadas dentro da sua rede     |
| [Scheduled tasks](/docs/pt/scheduled-tasks)                   | Defina um cronograma                                                                                             | [CLI](/docs/pt/scheduled-tasks), [Desktop](/docs/pt/desktop-scheduled-tasks), ou [nuvem](/docs/pt/routines) | Escolha uma frequência                                                                                                               | Automação recorrente como revisões diárias                       |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Claude Code na web](/docs/pt/claude-code-on-the-web): execute sessões na nuvem em vez de na sua máquina, configurado através de [ambientes em nuvem](/docs/pt/cloud-environments)
* [Mensagens entre sessões](/docs/pt/cross-session-messaging): deixe Claude enviar mensagens para suas sessões em outras máquinas ou em [sessões em nuvem](/docs/pt/claude-code-on-the-web)
* [Channels](/docs/pt/channels): encaminhe Telegram, Discord ou iMessage para uma sessão para que Claude reaja a mensagens enquanto você está ausente
* [Dispatch](/docs/pt/desktop#sessions-from-dispatch): envie uma mensagem com uma tarefa do seu telefone e ela pode gerar uma sessão Desktop para lidar com isso
* [Autenticação](/docs/pt/authentication): configure `/login` e gerencie credenciais para claude.ai
* [Referência de CLI](/docs/pt/cli-reference): lista completa de flags e comandos incluindo `claude remote-control`
* [Segurança](/docs/pt/security): como as sessões de Remote Control se encaixam no modelo de segurança do Claude Code
* [Uso de dados](/docs/pt/data-usage): quais dados fluem através da API Anthropic durante sessões locais, Remote Control e em nuvem
