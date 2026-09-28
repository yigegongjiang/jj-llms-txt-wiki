> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Aplicativo Desktop

> Aproveite ao máximo o Claude Code Desktop: sessões paralelas com isolamento Git, layout de painel com arrastar e soltar, terminal integrado e editor de arquivo, chats laterais, computer use, Dispatch sessions do seu telefone, revisão visual de diff, visualizações de aplicativos, monitoramento de PR, conectores e configuração corporativa.

O aplicativo Claude Desktop tem três abas: **Chat** para conversas, **Cowork** para [Dispatch e trabalho agentic mais longo](https://claude.com/product/cowork), e **Code** para desenvolvimento de software. Esta página é a referência para a aba Code.

<CardGroup cols={3}>
  <Card title="Baixar para macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Compilação universal para Intel e Apple Silicon
  </Card>

  <Card title="Baixar para Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Para processadores x64
  </Card>

  <Card title="Obter Claude para Linux (beta)" icon="linux" href="/docs/pt/desktop-linux">
    apt ou .deb para Ubuntu e Debian
  </Card>
</CardGroup>

Para Windows ARM64, baixe o [instalador ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). No Linux, instale com apt; consulte [Claude Desktop no Linux](/docs/pt/desktop-linux).

Após instalar, inicie Claude, faça login e clique na aba **Code**. Para um passo a passo de sua primeira sessão, consulte o [guia de primeiros passos](/docs/pt/desktop-quickstart).

Na aba Code, cada conversa é uma **sessão**: ela tem seu próprio histórico de chat e pasta de projeto, independente de qualquer outra sessão. A barra lateral lista suas sessões e permite que você execute várias em paralelo. Dentro de uma sessão você pode:

* [Revisar e comentar em diffs](#review-changes-with-diff-view), depois [monitorar o PR resultante através do CI](#monitor-pull-request-status)
* [Visualizar seu aplicativo em execução](#preview-your-app) no painel do navegador enquanto Claude verifica suas próprias alterações, e [abrir sites externos](#browse-external-sites) ao lado dele
* Assistir Claude [executar e testar seu aplicativo iOS](/docs/pt/desktop-ios-simulator) no painel do iOS Simulator
* [Organizar painéis](#arrange-your-workspace) para o chat, diff, navegador, terminal e editor de arquivo lado a lado
* Fazer uma [pergunta lateral](#ask-a-side-question-without-derailing-the-session) que usa o contexto da sessão sem desviá-la
* Permitir que Claude [verifique, envie mensagens ou arquive suas outras sessões](#work-across-sessions)
* [Conectar ferramentas externas](#connect-external-tools) como GitHub, Slack e Linear
* Permitir que Claude [abra aplicativos e controle sua tela](#let-claude-use-your-computer)
* Executar em sua máquina, na [nuvem](#run-long-running-tasks-in-the-cloud), ou sobre [SSH](#ssh-sessions)

Para [trabalho recorrente agendado](/docs/pt/desktop-scheduled-tasks), [atalhos de teclado](#keyboard-shortcuts), ou [envio de tarefas do seu telefone](#sessions-from-dispatch), consulte as páginas e seções vinculadas. Se você já usa o CLI baseado em terminal, consulte a [comparação CLI](#coming-from-the-cli) para ver o que é transferido.

<h2 id="start-a-session">
  Iniciar uma sessão
</h2>

Antes de enviar sua primeira mensagem, configure quatro coisas na área de prompt:

* **Ambiente**: escolha onde Claude é executado. Selecione **Local** para sua máquina, **Cloud** para uma [sessão em nuvem](#cloud-sessions) que continua após você fechar o aplicativo, uma [**conexão SSH**](#ssh-sessions) para uma máquina remota que você gerencia, ou no Windows uma [**distribuição WSL**](/docs/pt/desktop-wsl). Veja [configuração de ambiente](#environment-configuration).
* **Pasta do projeto**: selecione a pasta ou repositório em que Claude trabalha. Para sessões em nuvem, você pode adicionar [múltiplos repositórios](#run-long-running-tasks-in-the-cloud).
* **Modelo**: escolha um [modelo](/docs/pt/model-config#available-models) no menu suspenso ao lado do botão enviar. Você pode alterar isso durante a sessão.
* **Modo de permissão**: escolha quanto de autonomia Claude tem no [seletor de modo](#choose-a-permission-mode). Você pode alterar isso durante a sessão.

Digite sua tarefa e pressione **Enter** para começar. Cada sessão rastreia seu próprio contexto e alterações independentemente.

<h2 id="work-with-code">
  Trabalhar com código
</h2>

Dê a Claude o contexto certo, controle quanto ele faz por conta própria e revise o que ele alterou.

<h3 id="use-the-prompt-box">
  Use a caixa de prompt
</h3>

Digite o que você quer que Claude faça e pressione **Enter** para enviar. Claude lê seus arquivos de projeto, faz alterações e executa comandos com base no seu [modo de permissão](#choose-a-permission-mode). Você pode redirecionar Claude a qualquer momento: clique no botão parar para interromper imediatamente, ou digite uma correção e pressione **Enter** para enviá-la sem parar a ação em execução. Claude lê a correção assim que a ação atual é concluída e se ajusta antes de seu próximo passo.

O botão **+** ao lado da caixa de prompt oferece acesso a anexos de arquivo, [skills](#use-skills), [conectores](#connect-external-tools) e [plugins](#install-plugins).

<h3 id="add-files-and-context-to-prompts">
  Adicionar arquivos e contexto aos prompts
</h3>

A caixa de prompt suporta duas maneiras de trazer contexto externo:

* **@mention de arquivos**: digite `@` seguido de um nome de arquivo para adicionar um arquivo ao contexto da conversa. Claude pode então ler e referenciar esse arquivo. @mention não está disponível em sessões na nuvem ou WSL.
* **Anexar arquivos**: anexe imagens, PDFs e outros arquivos ao seu prompt usando o botão de anexo, ou arraste e solte arquivos diretamente no prompt. Isso é útil para compartilhar capturas de tela de bugs, mockups de design ou documentos de referência.

<h3 id="choose-a-permission-mode">
  Escolher um modo de permissão
</h3>

Os modos de permissão controlam quanto de autonomia Claude tem durante uma sessão: se ele pergunta antes de editar arquivos, executar comandos ou ambos. Você pode alternar modos de permissão a qualquer momento usando o seletor de modo ao lado do botão enviar. Para aprovar cada alteração você mesmo, mude para Manual.

Para definir um modo padrão para novas sessões locais, adicione `permissions.defaultMode` ao seu [arquivo de configurações](/docs/pt/settings#where-settings-live). O aplicativo desktop lê os mesmos arquivos de configurações que o CLI. Um modo que você escolhe no seletor é lembrado por pasta e tem precedência sobre `defaultMode` para essa pasta, exceto Plan, que se aplica apenas à sessão atual.

| Modo                   | Chave de configuração | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ---------------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manual**             | `default`             | Claude pergunta antes de editar arquivos ou executar comandos. Você vê um diff e pode aceitar ou rejeitar cada alteração.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Accept edits**       | `acceptEdits`         | Claude aceita automaticamente edições de arquivo e comandos comuns do sistema de arquivos como `mkdir`, `touch` e `mv`, mas ainda pergunta antes de executar outros comandos de terminal. Use isso quando você confia em alterações de arquivo e quer iteração mais rápida.                                                                                                                                                                                                                                                                                                                                                                                                           |
| **Plan**               | `plan`                | Claude lê arquivos e executa comandos para explorar, depois propõe um plano sem editar seu código-fonte. Bom para tarefas complexas onde você quer revisar a abordagem primeiro.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| **Auto**               | `auto`                | Claude executa todas as ações com verificações de segurança em segundo plano que verificam o alinhamento com sua solicitação. Reduz prompts de permissão mantendo supervisão. Aparece quando [auto mode está disponível](#auto-mode-availability); não há toggle de Configurações separado para isso.                                                                                                                                                                                                                                                                                                                                                                                 |
| **Bypass permissions** | `bypassPermissions`   | Claude é executado sem prompts de permissão, exceto para as [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves), classificadores de segurança quando Claude [atua em sites externos](#browse-external-sites), ou ações de desktop onde Claude sempre pergunta primeiro, como [arquivar uma sessão](#work-across-sessions). Equivalente a `--dangerously-skip-permissions` no CLI. Em planos Pro e Max, ative em suas Configurações → Claude Code em "Allow bypass permissions mode"; em planos Team e Enterprise não há toggle de Configurações, e a política organizacional controla isso. Use apenas em containers ou VMs sandboxed. |

Versões anteriores da aba Code rotulavam esses modos como Ask permissions, Auto accept edits e Plan mode.

O modo de permissão `dontAsk` está disponível apenas no [CLI](/docs/pt/permission-modes#allow-only-pre-approved-tools-with-dontask-mode).

<span id="auto-mode-availability" />

Auto mode está disponível para todos os usuários na API Anthropic e requer Claude Opus 4.6 ou posterior, Sonnet 4.6 ou posterior, ou um [modelo Fable](/docs/pt/model-config#work-with-fable). Administradores da organização podem desativar auto mode com a chave `disableAutoMode` em [configurações gerenciadas](#managed-settings).

Em implantações Enterprise que roteiam Desktop para Google Cloud's Agent Platform, auto mode também está disponível por padrão; veja [Auto mode no Bedrock, Agent Platform ou Foundry](/docs/pt/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry) para os modelos suportados.

<Tip title="Melhor prática">
  Comece tarefas complexas em Plan para que Claude mapeie uma abordagem antes de fazer alterações. Depois de aprovar o plano, mude para Accept edits ou Manual para executá-lo. Veja [explorar primeiro, depois planejar, depois codificar](/docs/pt/best-practices#explore-first-then-plan-then-code) para mais sobre esse fluxo de trabalho.
</Tip>

Sessões na nuvem suportam Accept edits, Plan e Auto. Accept edits corresponde ao modo `default`: sessões na nuvem pré-aprovam edições de arquivo, então o seletor mostra Accept edits em vez de Manual. Bypass permissions não está disponível em sessões na nuvem, incluindo sessões em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments).

Administradores corporativos podem restringir quais modos de permissão estão disponíveis. Veja [configuração corporativa](#enterprise-configuration) para detalhes.

<h3 id="preview-your-app">
  Visualizar seu aplicativo
</h3>

Claude pode iniciar um servidor de desenvolvimento e abrir um navegador incorporado para verificar suas alterações. Isso funciona para aplicativos web frontend e também para servidores backend: Claude pode testar endpoints de API, visualizar logs do servidor e iterar em problemas que encontra. Na maioria dos casos, Claude inicia o servidor automaticamente após editar arquivos de projeto. Você também pode pedir a Claude para visualizar a qualquer momento. Por padrão, Claude [verifica automaticamente](#auto-verify-changes) alterações após cada edição.

O painel de navegador também pode abrir arquivos HTML estáticos, PDFs, imagens e vídeos do seu projeto. Clique em um caminho HTML, PDF, imagem ou vídeo no chat para abri-lo lá.

No painel de navegador, você pode:

* Interagir com seu aplicativo em execução diretamente no painel de navegador
* Assistir Claude verificar suas próprias alterações automaticamente: ele tira capturas de tela, inspeciona o DOM, clica em elementos, preenche formulários e corrige problemas que encontra
* Iniciar ou parar servidores no menu suspenso de servidor na barra de ferramentas da sessão
* Persistir cookies e armazenamento local entre reinicializações do servidor selecionando **Persist sessions** no menu suspenso, para que você não tenha que fazer login novamente durante o desenvolvimento
* Editar a configuração do servidor ou parar todos os servidores de uma vez

Claude cria a configuração inicial do servidor com base em seu projeto. Se seu aplicativo usa um comando dev personalizado, edite `.claude/launch.json` para corresponder à sua configuração. Veja [Configurar servidores de visualização](#configure-preview-servers) para a referência completa.

Para limpar dados de sessão salvos, ou para desativar o navegador completamente, use os toggles em Configurações → Claude Code.

<h3 id="browse-external-sites">
  Navegar em sites externos
</h3>

O painel de navegador é um navegador com abas, então você pode abrir documentação, rastreadores de problemas ou qualquer outro site ao lado do seu aplicativo em execução. Para abrir o navegador, pressione **Cmd+Shift+B** no macOS ou **Ctrl+Shift+B** no Windows, ou selecione-o no menu **Views**. Quando você clica em um link externo no chat, um seletor oferece **Open in app** para usar o painel de navegador ou **Default browser** para usar o seu próprio; **Cmd**-clique no macOS ou **Ctrl**-clique no Windows abre um link no seu navegador do sistema diretamente. Você pode fazer login em sites no painel, incluindo fluxos de login em popup como Google OAuth.

Claude pode ler e interagir com páginas externas usando as mesmas ferramentas que usa para [verificar seu aplicativo](#preview-your-app), com duas verificações de segurança adicionais:

* Classificadores de segurança revisam as ações de escrita de Claude em páginas externas, como clicar e digitar, em todos os modos de permissão. Estes são os mesmos classificadores que [auto mode](#choose-a-permission-mode) usa, e quando eles sinalizam uma ação, você recebe um prompt de permissão independentemente do modo.
* Em modos de permissão diferentes de Auto e Bypass permissions, uma verificação de lista de permissões de domínio também se aplica antes de Claude navegar para um novo site.

<h4 id="approve-claude’s-actions-on-a-site">
  Aprovar as ações de Claude em um site
</h4>

A primeira vez que Claude atua em um site externo, um cartão de permissão aparece e Claude aguarda sua escolha: **Allow once**, **Always allow** ou **Deny**. **Allow once** aprova a ação sem salvar nada. **Always allow** salva a aprovação para esse site em seu dispositivo, e você pode revogá-la em Configurações. Cada site precisa de sua própria aprovação, incluindo subdomínios. Seus servidores dev locais e arquivos de projeto não precisam de aprovação, então [auto-verify](#auto-verify-changes) continua funcionando sem prompts.

Mesmo em um site aprovado, Claude não comprará itens, criará contas ou contornará CAPTCHAs sem sua entrada. Navegar no painel de navegador usa o mesmo modelo de segurança que a [extensão Claude no Chrome](/docs/pt/chrome). Veja [Usando Claude no Chrome com segurança](https://support.claude.com/en/articles/12902428-using-claude-in-chrome-safely) para como Claude lida com sites sensíveis e ações arriscadas.

<h4 id="choose-between-the-browser-and-the-chrome-extension">
  Escolher entre o painel de navegador e a extensão Chrome
</h4>

O painel de navegador usa um perfil de navegador limpo, separado do seu navegador pessoal, sem nenhum de seus logins salvos ou histórico. Use-o para construir e testar seu aplicativo e para sites que não precisam de sua identidade. Quando você quer que Claude aja como você em suas sessões conectadas, use a [extensão Claude no Chrome](/docs/pt/chrome) em vez disso, que compartilha o estado de login do seu navegador.

<h4 id="restrict-external-browsing-for-your-organization">
  Restringir navegação externa para sua organização
</h4>

O painel de navegador segue os mesmos [controles de lista de permissões e bloqueio de site](https://support.claude.com/en/articles/13065128-claude-in-chrome-admin-controls) que a extensão Claude no Chrome. Se sua organização já configurou essas listas para a extensão, o painel de navegador as respeita automaticamente. Administradores também podem desativar as ferramentas de Claude em páginas externas com a configuração gerenciada [`browserExternalPageTools`](#managed-settings). Com ferramentas desativadas, os usuários ainda podem navegar para sites externos; as ferramentas de Claude não podem ler ou agir sobre eles.

Para desativar a navegação externa completamente, defina a configuração gerenciada [`disableBrowserExternalNavigation`](#managed-settings) como `true`. Isso bloqueia toda navegação externa no painel de navegador, incluindo sites na lista de permissões de sua organização; servidores dev localhost e visualizações de arquivo continuam funcionando. Use `browserExternalPageTools` para permitir que os usuários continuem navegando em sites externos sem as ferramentas de Claude, e `disableBrowserExternalNavigation` para bloquear sites externos para usuários e Claude.

<h3 id="review-changes-with-diff-view">
  Revisar alterações com visualização de diff
</h3>

Depois que Claude faz alterações em seu código, a visualização de diff permite que você revise modificações arquivo por arquivo antes de criar um pull request.

Quando Claude altera arquivos, um indicador de estatísticas de diff aparece mostrando o número de linhas adicionadas e removidas, como `+12 -1`. Clique neste indicador para abrir o visualizador de diff, que exibe uma lista de arquivos à esquerda e as alterações para cada arquivo à direita.

Para comentar em linhas específicas, clique em qualquer linha no diff para abrir uma caixa de comentário. Digite seu feedback e pressione **Enter** para adicionar o comentário. Depois de adicionar comentários a várias linhas, envie todos os comentários de uma vez:

* **macOS**: pressione **Cmd+Enter**
* **Windows**: pressione **Ctrl+Enter**

Claude lê seus comentários e faz as alterações solicitadas, que aparecem como um novo diff que você pode revisar.

<h3 id="review-your-code">
  Revisar seu código
</h3>

Na visualização de diff, clique em **Review code** na barra de ferramentas superior direita para pedir a Claude para avaliar as alterações antes de você fazer commit. Claude examina os diffs atuais e deixa comentários diretamente na visualização de diff. Você pode responder a qualquer comentário ou pedir a Claude para revisar.

A revisão se concentra em problemas de alto sinal: erros de compilação, erros de lógica definidos, vulnerabilidades de segurança e bugs óbvios. Não sinaliza estilo, formatação, problemas pré-existentes ou qualquer coisa que um linter capturaria.

<h3 id="monitor-pull-request-status">
  Monitorar status de pull request
</h3>

Depois de abrir um pull request, uma barra de status de CI aparece na sessão. Claude Code usa o GitHub CLI para pesquisar resultados de verificação e exibir falhas.

* **Auto-fix**: quando ativado, Claude tenta automaticamente corrigir verificações de CI falhando lendo a saída de falha e iterando.
* **Auto-merge**: quando ativado, Claude mescla o PR assim que todas as verificações passam. O método de mesclagem é squash. Ative auto-merge em suas [configurações do repositório GitHub](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository) primeiro; sem isso, Claude não pode mesclar o PR.

Use os toggles **Auto-fix** e **Auto-merge** na barra de status de CI para ativar qualquer opção. Claude Code também envia uma notificação de desktop quando CI termina. Para arquivar a sessão automaticamente assim que o PR mescla ou fecha, ative [auto-archive](#work-in-parallel-with-sessions) em Configurações → Claude Code.

<Note>
  O monitoramento de PR requer que o [GitHub CLI (`gh`)](https://cli.github.com/) esteja instalado e autenticado em sua máquina. Se `gh` não estiver instalado, Desktop o solicita a instalar na primeira vez que você tentar criar um PR.
</Note>

<h2 id="arrange-your-workspace">
  Organizar seu workspace
</h2>

A aba Code é construída em torno de painéis que você pode organizar em qualquer layout: chat, diff, browser, terminal, file, plan, tasks e subagent, junto com o [iOS Simulator](/docs/pt/desktop-ios-simulator) no macOS. Arraste um painel por seu cabeçalho para reposicioná-lo, ou arraste uma borda de painel para redimensioná-lo. Pressione **Cmd+\\** no macOS ou **Ctrl+\\** no Windows para fechar o painel focado. Abra painéis adicionais no menu **Views** na barra de ferramentas da sessão.

Para trabalhar em várias telas, extraia um painel como o diff ou terminal para sua própria janela e encaixe-o novamente quando terminar. Claude continua trabalhando na janela principal.

<Note>
  O layout do painel, terminal, editor de arquivo e modos de visualização nesta seção requerem Claude Desktop v1.2581.0 ou posterior. Abra **Claude → Check for Updates** no macOS ou **Help → Check for Updates** no Windows para atualizar.
</Note>

<h3 id="run-commands-in-the-terminal">
  Executar comandos no terminal
</h3>

O terminal integrado permite que você execute comandos ao lado de sua sessão sem alternar para outro aplicativo. Abra-o no menu **Views** ou pressione **Ctrl+\`** no macOS ou Windows. O terminal abre no diretório de trabalho de sua sessão e compartilha o mesmo ambiente que Claude, então comandos como `npm test` ou `git status` veem os mesmos arquivos que Claude está editando. Para abrir uma segunda aba de terminal, clique em **+** no cabeçalho do painel de terminal ou clique com o botão direito em uma pasta no chat para escolher **Open in terminal**. O terminal está disponível apenas em sessões locais.

<h3 id="open-and-edit-files">
  Abrir e editar arquivos
</h3>

Clique em um caminho de arquivo no chat ou visualizador de diff para abri-lo no painel de arquivo. Caminhos HTML, PDF, imagem e vídeo abrem no [painel de browser](#preview-your-app) em vez disso. Faça edições pontuais e clique em **Save** para escrevê-las de volta. Se o arquivo mudou no disco desde que você o abriu, o painel o avisa e permite que você sobrescreva ou descarte. Clique em **Discard** para reverter suas edições, ou clique no caminho no cabeçalho do painel para copiar o caminho absoluto.

O painel de arquivo está disponível em sessões locais e SSH. Para sessões remotas, peça a Claude para fazer a alteração.

<h3 id="open-files-in-other-apps">
  Abrir arquivos em outros aplicativos
</h3>

Clique com o botão direito em qualquer caminho de arquivo no chat, visualizador de diff ou painel de arquivo para abrir um menu de contexto:

* **Attach as context**: adicione o arquivo ao seu próximo prompt
* **Open in**: abra o arquivo em um editor instalado como VS Code, Cursor ou Zed
* **Show in Finder** no macOS, **Show in Explorer** no Windows: abra a pasta contendo
* **Copy path**: copie o caminho absoluto para sua área de transferência

<h3 id="switch-view-modes">
  Alternar modos de visualização
</h3>

Os modos de visualização controlam quanto detalhe aparece na transcrição do chat. Alterne modos no menu suspenso **Transcript view** ao lado do botão enviar, ou pressione **Ctrl+O** no macOS ou Windows para ciclar através deles. O modo Thinking aparece no menu suspenso apenas depois que Claude produziu thinking na sessão que você está visualizando.

| Modo         | O que mostra                                                                                                    |
| ------------ | --------------------------------------------------------------------------------------------------------------- |
| **Normal**   | Chamadas de ferramenta recolhidas em resumos, com respostas de texto completo                                   |
| **Thinking** | Chamadas de ferramenta recolhidas em resumos, mais o thinking de Claude                                         |
| **Verbose**  | Cada chamada de ferramenta, leitura de arquivo e passo intermediário que Claude toma, mais o thinking de Claude |

Use Thinking para seguir o raciocínio de Claude com chamadas de ferramenta ainda recolhidas. Use Verbose ao depurar por que Claude tomou uma ação particular. As versões do Claude Desktop anteriores a 1.46388.1 também listam um modo Summary, e uma sessão ainda definida para Summary abre em Normal assim que você atualiza.

<h3 id="keyboard-shortcuts">
  Atalhos de teclado
</h3>

Pressione **Cmd+/** no macOS ou **Ctrl+/** no Windows para ver todos os atalhos disponíveis na aba Code. No Windows, use **Ctrl** no lugar de **Cmd** para os atalhos abaixo. Ciclagem de sessão, alternância de terminal e alternância de modo de visualização usam **Ctrl** em todas as plataformas.

| Atalho                                | Ação                              |
| ------------------------------------- | --------------------------------- |
| `Cmd` `/`                             | Mostrar atalhos de teclado        |
| `Cmd` `N`                             | Nova sessão                       |
| `Cmd` `W`                             | Fechar sessão                     |
| `Ctrl` `Tab` / `Ctrl` `Shift` `Tab`   | Próxima ou sessão anterior        |
| `Cmd` `Shift` `]` / `Cmd` `Shift` `[` | Próxima ou sessão anterior        |
| `Esc`                                 | Parar resposta de Claude          |
| `Cmd` `Shift` `D`                     | Alternar painel de diff           |
| `Cmd` `Shift` `B`                     | Alternar painel de browser        |
| `Cmd` `Shift` `S`                     | Selecionar um elemento no browser |
| `Ctrl` `` ` ``                        | Alternar painel de terminal       |
| `Cmd` `\`                             | Fechar painel focado              |
| `Cmd` `;`                             | Abrir chat lateral                |
| `Ctrl` `O`                            | Ciclar modos de visualização      |
| `Cmd` `Shift` `M`                     | Abrir menu de modo de permissão   |
| `Cmd` `Shift` `I`                     | Abrir menu de modelo              |
| `Cmd` `Shift` `E`                     | Abrir menu de esforço             |
| `1`–`9`                               | Selecionar item em um menu aberto |

Esses atalhos se aplicam apenas à aba Code. Os [atalhos de modo interativo](/docs/pt/interactive-mode#keyboard-shortcuts) baseados em terminal, como `Shift+Tab` para ciclar modos, não se aplicam em Desktop.

<h3 id="check-usage">
  Verificar uso
</h3>

Clique no anel de uso ao lado do seletor de modelo para ver seu uso atual da janela de contexto e seu uso do plano para o período. O uso de contexto é por sessão; o uso do plano é compartilhado em todas as suas superfícies Claude Code.

<h2 id="let-claude-use-your-computer">
  Deixar Claude usar seu computador
</h2>

Computer use permite que Claude abra seus aplicativos, controle sua tela e trabalhe diretamente em sua máquina da forma como você faria. Peça a Claude para interagir com uma ferramenta de desktop que não tem CLI ou automatize algo que só funciona através de uma GUI. Para executar e testar aplicativos iOS, Desktop abre o painel dedicado do [iOS Simulator](/docs/pt/desktop-ios-simulator) em vez de controlar sua tela; o painel funciona sem ativar computer use.

<Note>
  Computer use é uma visualização de pesquisa no macOS e Windows que requer um plano Pro ou Max. Não está disponível em planos Team ou Enterprise. O aplicativo Claude Desktop deve estar em execução.
</Note>

Computer use está desativado por padrão. [Ative-o em Configurações](#enable-computer-use) antes que Claude possa controlar sua tela. No macOS, você também precisa conceder permissões de Acessibilidade e Gravação de Tela.

No macOS, computer use também pode ser executado em segundo plano: Claude trabalha nos aplicativos que você aprovou enquanto você continua trabalhando.

<Warning>
  Diferentemente da [ferramenta Bash sandboxed](/docs/pt/sandboxing), computer use é executado em seu desktop real com acesso a tudo que você aprova. Claude verifica cada ação e sinaliza possível injeção de prompt do conteúdo na tela, mas o limite de confiança é diferente. Veja o [guia de segurança de computer use](https://support.claude.com/en/articles/14128542) para melhores práticas.
</Warning>

<h3 id="when-computer-use-applies">
  Quando computer use se aplica
</h3>

Claude tem várias maneiras de interagir com um aplicativo ou serviço, e computer use é a mais ampla e lenta. Ele tenta a ferramenta mais precisa primeiro:

* Se você tem um [connector](#connect-external-tools) para um serviço, Claude usa o connector.
* Se a tarefa é um comando shell, Claude usa Bash.
* Se a tarefa é trabalho de navegador e você tem [Claude no Chrome](/docs/pt/chrome) configurado, Claude usa isso.
* Se a tarefa é executar ou testar um aplicativo iOS, Claude usa o [painel iOS Simulator](/docs/pt/desktop-ios-simulator), que não usa controle de tela.
* Se nenhum desses se aplica, Claude usa computer use.

Os [níveis de acesso por aplicativo](#app-permissions) reforçam isso: navegadores são limitados a apenas visualização, e terminais e IDEs a apenas clique, direcionando Claude para a ferramenta dedicada mesmo quando computer use está ativo. O controle de tela é reservado para coisas que nada mais pode alcançar, como aplicativos nativos, painéis de controle de hardware ou ferramentas proprietárias sem uma API.

<h3 id="enable-computer-use">
  Ativar computer use
</h3>

Computer use está desativado por padrão. Se você pedir a Claude para fazer algo que precisa disso enquanto está desativado, Claude diz que poderia fazer a tarefa se você ativar computer use em Configurações.

<Steps>
  <Step title="Atualizar o aplicativo desktop">
    Certifique-se de que você tem a versão mais recente do Claude Desktop. No macOS e Windows, baixe ou atualize em [claude.com/download](https://claude.com/download); no Linux, atualize através do seu gerenciador de pacotes ([instruções](/docs/pt/desktop-linux)). Depois reinicie o aplicativo.
  </Step>

  <Step title="Ativar o toggle">
    No aplicativo desktop, vá para **Configurações > Geral** (em **Aplicativo Desktop**). Encontre o toggle **Computer use** e ative-o. No Windows, o toggle entra em efeito imediatamente e a configuração está completa. No macOS, continue para o próximo passo.

    Se você não vir o toggle, confirme que você está em macOS ou Windows com um plano Pro ou Max, depois atualize e reinicie o aplicativo.
  </Step>

  <Step title="Conceder permissões macOS">
    No macOS, conceda duas permissões do sistema antes do toggle entrar em efeito:

    * **Accessibility**: permite que Claude clique, digite e role
    * **Screen Recording**: permite que Claude veja o que está em sua tela

    A página de Configurações mostra o status atual de cada permissão. Se alguma for negada, clique no badge para abrir o painel de Configurações do Sistema relevante.
  </Step>
</Steps>

<h3 id="app-permissions">
  Permissões de aplicativo
</h3>

A primeira vez que Claude precisa usar um aplicativo, um prompt aparece em sua sessão. Clique em **Allow for this session** ou **Deny**. As aprovações duram para a sessão atual, ou 30 minutos em [sessões geradas por Dispatch](#sessions-from-dispatch).

O prompt também mostra que nível de controle Claude obtém para esse aplicativo. Esses níveis são fixos por categoria de aplicativo e não podem ser alterados:

| Nível        | O que Claude pode fazer                                    | Se aplica a                            |
| :----------- | :--------------------------------------------------------- | :------------------------------------- |
| View only    | Ver o aplicativo em capturas de tela                       | Navegadores, plataformas de negociação |
| Click only   | Clicar e rolar, mas não digitar ou usar atalhos de teclado | Terminais, IDEs                        |
| Full control | Clicar, digitar, arrastar e usar atalhos de teclado        | Tudo mais                              |

Aplicativos com alcance amplo, como terminais, Finder ou File Explorer, e System Settings ou Settings, mostram um aviso extra no prompt para que você saiba o que aprovar concede.

Você pode configurar duas configurações em **Configurações > Geral** (em **Aplicativo Desktop**):

* **Denied apps**: adicione aplicativos aqui para rejeitá-los sem solicitar. Claude ainda pode afetar um aplicativo negado indiretamente através de ações em um aplicativo permitido, mas não pode interagir com o aplicativo negado diretamente.
* **Unhide apps when Claude finishes**: quando computer use não está sendo executado em segundo plano, Claude oculta suas outras janelas enquanto trabalha para que interaja apenas com o aplicativo aprovado. Quando Claude termina, as janelas ocultas são restauradas a menos que você desative essa configuração.

<h2 id="manage-sessions">
  Gerenciar sessões
</h2>

Cada sessão é uma conversa independente com seu próprio contexto e alterações. Você pode executar múltiplas sessões em paralelo, ramificar chats laterais, deixar Claude verificar e enviar mensagens para suas outras sessões, enviar trabalho para a nuvem ou deixar Dispatch iniciar sessões para você do seu telefone.

<h3 id="work-in-parallel-with-sessions">
  Trabalhar em paralelo com sessões
</h3>

Clique em **+ New session** na barra lateral, ou pressione **Cmd+N** no macOS ou **Ctrl+N** no Windows, para trabalhar em múltiplas tarefas em paralelo. Pressione **Ctrl+Tab** e **Ctrl+Shift+Tab** para ciclar através de sessões na barra lateral. Para repositórios Git, selecione a opção **worktree** ao lado do nome do branch para dar à sessão sua própria cópia isolada do seu projeto usando [Git worktrees](/docs/pt/worktrees), para que alterações em uma sessão não afetem outras sessões até que você as faça commit.

Para visualizar duas sessões ao mesmo tempo, mantenha **Cmd** no macOS ou **Ctrl** no Windows e clique em uma sessão na barra lateral. A sessão abre em um segundo painel ao lado daquele que você já tem aberto. Enquanto a divisão está ativa, clicar em outra sessão da barra lateral substitui o painel que tem foco. Pressione **Cmd+\\** no macOS ou **Ctrl+\\** no Windows para fechar o painel focado e retornar a uma única sessão.

Worktrees são armazenadas em `<project-root>/.claude/worktrees/` por padrão. Você pode alterar isso para um diretório personalizado em Configurações → Claude Code em "Worktree location". Você também pode definir um prefixo de branch que é adicionado a cada nome de branch worktree, o que é útil para manter branches criadas por Claude organizadas. Para remover um worktree quando terminar, passe o mouse sobre a sessão na barra lateral e clique no ícone de arquivo. Para ter sessões se arquivarem automaticamente quando seu pull request mescla ou fecha, ative **Auto-archive after PR merge or close** em Configurações → Claude Code. Auto-archive se aplica apenas a sessões locais que terminaram de executar.

Para incluir arquivos gitignored como `.env` em novos worktrees, crie um [arquivo `.worktreeinclude`](/docs/pt/worktrees#copy-gitignored-files-into-worktrees) na raiz do seu projeto.

<Note>
  O isolamento de sessão requer [Git](https://git-scm.com/downloads). A maioria dos Macs inclui Git por padrão. Execute `git --version` no Terminal para verificar; se imprimir um número de versão, Git está instalado. Se você encontrar erros de Git, peça a Claude na aba [Cowork](https://claude.com/product/cowork) para ajudar a solucionar problemas de sua configuração.
</Note>

Use os controles no topo da barra lateral para filtrar sessões por status, projeto ou ambiente, e para agrupar sessões por projeto. Para renomear uma sessão, clique no título da sessão na barra de ferramentas no topo da sessão ativa.

Para verificar o uso de contexto, veja [Verificar uso](#check-usage). Quando o contexto se enche, Claude automaticamente resume a conversa e continua trabalhando. Você também pode digitar `/compact` para disparar a sumarização mais cedo e liberar espaço de contexto. Veja [a janela de contexto](/docs/pt/how-claude-code-works#the-context-window) para detalhes sobre como a compactação funciona.

O aplicativo desktop envia uma notificação do SO quando uma sessão de Code termina uma tarefa e você não está visualizando essa sessão no momento. Para sessões que pertencem a um [projeto](/docs/pt/claude-projects#see-what-needs-you-in-overview), você recebe as notificações do projeto em vez disso.

<h3 id="ask-a-side-question-without-derailing-the-session">
  Fazer uma pergunta lateral sem descarrilar a sessão
</h3>

Um chat lateral permite que você faça a Claude uma pergunta que usa o contexto de sua sessão mas não adiciona nada de volta à conversa principal. Use-o quando você quer entender um pedaço de código, verificar uma suposição ou explorar uma ideia sem descarrilar a sessão.

Pressione **Cmd+;** no macOS ou **Ctrl+;** no Windows para abrir um chat lateral, ou digite `/btw` na caixa de prompt. O chat lateral pode ler tudo no thread principal até esse ponto. Quando terminar, feche o chat lateral e continue a sessão principal onde deixou.

Chats laterais estão disponíveis em sessões locais, SSH e WSL. O aplicativo desktop não salva chats laterais em disco, então você não pode retornar a um após fechar o aplicativo.

<h3 id="watch-background-tasks">
  Assistir tarefas em segundo plano
</h3>

O painel de tarefas mostra o trabalho em segundo plano em execução dentro da sessão atual: subagents, comandos shell em segundo plano e [workflows dinâmicos](/docs/pt/workflows). Abra-o no menu **Views** ou arraste-o para seu layout.

Clique em qualquer entrada para ver sua saída no painel de subagent ou pará-la. Para ver o que outras sessões estão fazendo, use a [barra lateral](#work-in-parallel-with-sessions), ou peça a Claude para [verificar nelas para você](#work-across-sessions).

<h3 id="work-across-sessions">
  Trabalhar entre sessões
</h3>

Claude pode listar suas outras sessões da aba Code, ler o que cada uma tem feito e enviar mensagens entre elas. Pergunte em linguagem natural: "qual sessão tocou na refatoração de autenticação?", "o que a sessão de API concluiu?" ou "diga à sessão de pagamentos que o schema mudou". Você também pode pedir a Claude para renomear ou arquivar uma sessão. Claude arquiva uma sessão da mesma forma que o ícone de arquivo da barra lateral faz, então peça a ela para limpar sessões cujos PRs foram mesclados.

Através desta superfície, Claude vê apenas as sessões que o aplicativo desktop executa por si mesmo: local, [SSH](#ssh-sessions) e sessões [WSL](/docs/pt/desktop-wsl) na aba Code. Claude não vê sessões em nuvem, ou sessões que você iniciou do CLI do terminal ou da extensão VS Code, mesmo em worktrees do mesmo projeto, então com nove worktrees de terminal abertos e duas sessões desktop, Claude respondendo em uma delas relata a outra sessão desktop. Claude nunca lista a sessão de que você está perguntando. Por padrão, ele vê as 20 sessões mais recentemente ativas e pula sessões arquivadas a menos que você peça por elas. [Mensagens entre sessões](/docs/pt/cross-session-messaging) permite separadamente que Claude envie mensagens para [suas outras sessões Claude Code](/docs/pt/cross-session-messaging#see-which-sessions-claude-can-reach), incluindo sessões de terminal.

Quando Claude envia uma mensagem para outra sessão através desta superfície, Claude Code a mostra lá como um card rotulado com o título da sessão de envio e um link de volta, para que você sempre possa dizer de onde uma mensagem veio. Se a sessão receptora está no meio de uma tarefa, Claude Code segura a mensagem e Claude a lê uma vez que o trabalho atual termina. O Claude receptor pode responder, e Claude Code entrega a resposta de volta através desta superfície. Claude não pode entregar a uma sessão arquivada e avisa você quando uma mensagem não passa.

Claude Code aplica quatro comportamentos de segurança entre sessões:

* Antes de arquivar qualquer sessão, Claude pede sua aprovação primeiro. Você vê o card de aprovação em todos os modos de permissão, incluindo permissões Auto e Bypass.
* Através desta superfície, Claude não pode enviar mensagens entre sessões de uma sessão que ninguém está observando, como uma execução de tarefa agendada, e não pode entregar mensagens em uma.
* Claude Code verifica cada mensagem desta superfície contra os [controles de entrada](/docs/pt/cross-session-messaging#control-inbound-messages) da sessão receptora, mesmo quando a sessão receptora não tem [mensagens entre sessões](/docs/pt/cross-session-messaging#availability) por si mesma. Se você definir [`crossSessionInbound`](/docs/pt/settings-reference#crosssessioninbound) para `refuse` na sessão receptora, Claude Code descarta mensagens desta superfície. Claude Code relata a recusa ao aplicativo desktop Claude. Antes da v2.1.234, Claude Code descartava todas as mensagens desta superfície para uma sessão receptora sem mensagens entre sessões.
* Claude Code cita cada mensagem de entrada e a atribui à sessão que a enviou, e Claude ainda segue as configurações de permissão próprias da sessão receptora ao agir em uma.

Claude também pode sugerir novas sessões. Quando ele nota algo que vale a pena corrigir que está fora do escopo da tarefa atual, ele oferece o trabalho como um chip de tarefa no chat. Clique no chip para iniciar esse trabalho em uma nova sessão com seu próprio worktree; Claude continua sua sessão atual ininterruptamente.

<h3 id="run-long-running-tasks-in-the-cloud">
  Executar tarefas de longa duração na nuvem
</h3>

Para grandes refatorações, suites de teste, migrações ou outras tarefas de longa duração, selecione **Cloud** em vez de **Local** ao iniciar uma sessão. Sessões em nuvem são executadas na infraestrutura gerenciada pela Anthropic por padrão e continuam mesmo se você fechar o aplicativo ou desligar seu computador. Verifique a qualquer momento para ver o progresso ou direcionar Claude em uma direção diferente. Você também pode monitorar sessões em nuvem de [claude.ai/code](https://claude.ai/code) ou do [aplicativo Claude mobile](/docs/pt/mobile).

Sessões em nuvem também suportam múltiplos repositórios. Depois de selecionar um ambiente em nuvem, clique no botão **+** ao lado do repositório selecionado para adicionar mais repositórios à sessão. Cada repo obtém seu próprio seletor de branch. Isso é útil para tarefas que abrangem múltiplas bases de código, como atualizar uma biblioteca compartilhada e seus consumidores.

Veja [Use Claude Code na nuvem](/docs/pt/claude-code-on-the-web) para mais sobre como sessões em nuvem funcionam. Quando um corpo de trabalho precisa de muitas sessões em nuvem, selecione **Projects** na barra lateral para criar um [projeto](/docs/pt/claude-projects), onde Claude inicia e rastreia as sessões para você de uma conversa.

<h3 id="continue-in-another-surface">
  Continuar em outra superfície
</h3>

O menu **Continue in**, acessível do ícone VS Code no canto inferior direito da barra de ferramentas da sessão, permite que você mova sua sessão para outra superfície:

* **Claude Code on the Web**: envia sua sessão local para continuar executando na nuvem. Desktop envia seu branch, gera um resumo da conversa e cria uma nova sessão em nuvem com o contexto completo. Você pode então escolher arquivar a sessão local ou mantê-la. Isso requer uma árvore de trabalho limpa e não está disponível para sessões SSH.
* **Your IDE**: abre seu projeto em um IDE suportado no diretório de trabalho atual.

<h3 id="sessions-from-dispatch">
  Sessões do Dispatch
</h3>

[Dispatch](https://support.claude.com/en/articles/13947068) é uma conversa persistente com Claude que vive na aba [Cowork](https://claude.com/product/cowork). Você envia uma mensagem ao Dispatch com uma tarefa, e ele decide como lidar com ela.

Uma tarefa pode acabar como uma sessão de Code de duas maneiras: você pede uma diretamente, como "abra uma sessão Claude Code e corrija o bug de login", ou Dispatch decide que a tarefa é trabalho de desenvolvimento e gera uma por conta própria. Tarefas que normalmente são roteadas para Code incluem corrigir bugs, atualizar dependências, executar testes ou abrir pull requests. Pesquisa, edição de documentos e trabalho em planilhas ficam em Cowork.

De qualquer forma, a sessão de Code aparece na barra lateral da aba Code com um badge **Dispatch**. Você recebe uma notificação push em seu telefone quando termina ou precisa de sua aprovação.

Se você tem [computer use](#let-claude-use-your-computer) ativado, sessões de Code geradas por Dispatch também podem usá-lo. As aprovações de aplicativo nessas sessões expiram após 30 minutos e solicitam novamente, em vez de durarem a sessão completa como sessões de Code regulares.

Para configuração, emparelhamento e configurações de Dispatch, veja o [artigo de ajuda do Dispatch](https://support.claude.com/en/articles/13947068). Dispatch requer um plano Pro ou Max e não está disponível em planos Team ou Enterprise.

Dispatch é uma de várias maneiras de trabalhar com Claude quando você está longe de seu terminal. Para uma comparação com as outras opções, veja [Plataformas e integrações](/docs/pt/platforms#work-when-you-are-away-from-your-terminal).

<h2 id="extend-claude-code">
  Estender Claude Code
</h2>

Conecte serviços externos, adicione fluxos de trabalho reutilizáveis, customize o comportamento de Claude e configure servidores de visualização. Para gerenciar conectores, skills e plugins em um único lugar, clique em **Customize** na barra lateral. A aba [Cowork](https://claude.com/product/cowork) no aplicativo Desktop obtém seus skills, plugins e conectores dessa configuração de Customize, que sincroniza através de sua conta claude.ai, não do diretório `~/.claude` da CLI.

Claude Code também carrega os skills e plugins habilitados para sua conta claude.ai em sessões de terminal onde você se conecta com a mesma conta. Veja [Skills sincronizados de claude.ai](/docs/pt/skills#how-synced-skills-behave) e [Plugins sincronizados de claude.ai](/docs/pt/plugins/loading#synced-plugins).

<h3 id="connect-external-tools">
  Conectar ferramentas externas
</h3>

Para sessões locais e [SSH](#ssh-sessions), clique no botão **+** ao lado da caixa de prompt e selecione **Connectors** para adicionar integrações como Google Calendar, Slack, GitHub, Linear, Notion e muito mais. Você pode adicionar conectores antes ou durante uma sessão. O botão **+** não está disponível em sessões cloud ou WSL, mas [routines](/docs/pt/routines) configuram conectores no momento da criação da rotina.

Para gerenciar ou desconectar conectores, vá para Settings → Connectors no aplicativo desktop, ou selecione **Manage connectors** no menu Connectors na caixa de prompt.

Uma vez conectado, Claude pode ler seu calendário, enviar mensagens, criar problemas e interagir com suas ferramentas diretamente. Você pode perguntar a Claude quais conectores estão configurados em sua sessão.

Conectores são [MCP servers](/docs/pt/mcp) com um fluxo de configuração gráfica. Use-os para integração rápida com serviços suportados. Para integrações não listadas em Connectors, adicione MCP servers manualmente via [arquivos de configuração](/docs/pt/mcp#installing-mcp-servers). Você também pode [criar conectores personalizados](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp).

<h3 id="use-skills">
  Use skills
</h3>

[Skills](/docs/pt/skills) estendem o que Claude pode fazer. Claude as carrega automaticamente quando relevante, ou você pode invocar uma diretamente: digite `/` na caixa de prompt ou clique no botão **+** e selecione **Slash commands** para navegar pelo que está disponível. Isso inclui [comandos integrados](/docs/pt/commands), suas [skills personalizadas](/docs/pt/skills#create-your-first-skill), skills de projeto de sua base de código e skills de qualquer [plugins instalados](/docs/pt/plugins/install). Selecione uma e ela aparece destacada no campo de entrada. Digite sua tarefa depois dela e envie como usual.

Você pode enviar um comando enquanto Claude está trabalhando, da mesma forma que qualquer outra mensagem, e a sessão retorna ao estado ocioso uma vez que a rodada termina. Antes da v2.1.206, um comando enviado no meio da rodada poderia deixar a sessão mostrando como em execução e as mensagens que você enviou depois não eram entregues.

Sessões locais carregam seus skills pessoais de `~/.claude/skills/`. Uma sessão [SSH](#ssh-sessions) lê `~/.claude/skills/` do diretório home do host remoto, não de sua máquina.

Sessões locais e cloud também carregam os skills habilitados para sua conta claude.ai. Sessões cloud os carregam em vez de `~/.claude/skills/`, como [Skills em sessões Cowork e cloud](/docs/pt/skills#skills-in-cowork-and-cloud-sessions) descreve.

<h3 id="install-plugins">
  Instalar plugins
</h3>

[Plugins](/docs/pt/plugins/overview) são pacotes reutilizáveis que adicionam skills, agents, hooks, MCP servers e configurações LSP ao Claude Code. Você pode instalar plugins do aplicativo desktop sem usar o terminal.

Para sessões locais e [SSH](#ssh-sessions), clique no botão **+** ao lado da caixa de prompt e selecione **Plugins** para ver seus plugins instalados e seus skills. Para adicionar um plugin, selecione **Add plugin** no submenu para abrir o navegador de plugins, que mostra plugins disponíveis de seus [marketplaces](/docs/pt/plugins/overview) configurados incluindo o marketplace oficial da Anthropic. Selecione **Manage plugins** para ativar, desativar ou desinstalar plugins.

Você pode escopar plugins para sua conta de usuário, um projeto específico ou apenas local. Se sua organização gerencia plugins centralmente, esses plugins estão disponíveis em sessões desktop da mesma forma que estão no CLI.

O navegador de plugins não está disponível em sessões cloud, e plugins que você instala do aplicativo desktop não estão disponíveis para sessões cloud. Uma sessão cloud também não instala plugins que o `.claude/settings.json` do repositório declara, como [O que é transferido de sua configuração](/docs/pt/cloud-environments#what-carries-over-from-your-setup) explica. Plugins não estão disponíveis em sessões WSL. Para a referência completa de plugins incluindo criar seus próprios plugins, veja [plugins](/docs/pt/plugins/overview).

<h3 id="configure-preview-servers">
  Configurar servidores de visualização
</h3>

Claude detecta automaticamente sua configuração de servidor de desenvolvimento e armazena a configuração em `.claude/launch.json` na raiz da pasta que você selecionou ao iniciar a sessão. Preview usa essa pasta como seu diretório de trabalho, então se você selecionou uma pasta pai, subpastas com seus próprios servidores de desenvolvimento não serão detectadas automaticamente. Para trabalhar com o servidor de uma subpasta, inicie uma sessão nessa pasta diretamente ou adicione uma configuração manualmente.

Para personalizar como seu servidor inicia, por exemplo para usar `yarn dev` em vez de `npm run dev` ou para alterar a porta, edite o arquivo manualmente ou clique em **Edit configuration** no menu dropdown do servidor para abri-lo em seu editor de código. O arquivo suporta JSON com comentários.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

Você pode definir múltiplas configurações para executar diferentes servidores do mesmo projeto, como um frontend e uma API. Veja os [exemplos](#examples) abaixo.

<h4 id="auto-verify-changes">
  Auto-verify changes
</h4>

Quando `autoVerify` está ativado, Claude verifica automaticamente alterações de código após editar arquivos. Ele tira capturas de tela, verifica erros e confirma que as alterações funcionam antes de completar sua resposta.

Auto-verify está ativado por padrão. Desative-o por projeto adicionando `"autoVerify": false` a `.claude/launch.json`, ou alterne-o no menu dropdown do servidor.

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

Quando desativado, ferramentas de visualização ainda estão disponíveis e você pode pedir a Claude para verificar a qualquer momento. Auto-verify torna isso automático após cada edição.

<h4 id="configuration-fields">
  Configuration fields
</h4>

Cada entrada no array `configurations` aceita os seguintes campos:

| Campo               | Tipo      | Descrição                                                                                                                                                                                                                                                                                             |
| ------------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`              | string    | Um identificador único para este servidor                                                                                                                                                                                                                                                             |
| `runtimeExecutable` | string    | O comando a executar, como `npm`, `yarn` ou `node`                                                                                                                                                                                                                                                    |
| `runtimeArgs`       | string\[] | Argumentos passados para `runtimeExecutable`, como `["run", "dev"]`                                                                                                                                                                                                                                   |
| `port`              | number    | A porta em que seu servidor escuta. Padrão é 3000                                                                                                                                                                                                                                                     |
| `cwd`               | string    | Diretório de trabalho relativo à raiz do seu projeto. Padrão é a raiz do projeto. Use `${workspaceFolder}` para referenciar a raiz do projeto explicitamente                                                                                                                                          |
| `env`               | object    | Variáveis de ambiente adicionais como pares chave-valor, como `{ "NODE_ENV": "development" }`. Não coloque segredos aqui já que este arquivo é commitado em seu repo. Para passar segredos ao seu servidor de desenvolvimento, defina-os no [editor de ambiente local](#local-sessions) em vez disso. |
| `autoPort`          | boolean   | Como lidar com conflitos de porta. Veja [Conflitos de porta](#port-conflicts)                                                                                                                                                                                                                         |
| `program`           | string    | Um script a executar com `node`. Veja [quando usar `program` vs `runtimeExecutable`](#when-to-use-program-vs-runtimeexecutable)                                                                                                                                                                       |
| `args`              | string\[] | Argumentos passados para `program`. Usado apenas quando `program` está definido                                                                                                                                                                                                                       |
| `url`               | string    | O endereço que a visualização abre em vez de `http://localhost:<port>`. Veja [abrir a visualização em uma URL específica](#open-the-preview-at-a-specific-url)                                                                                                                                        |

<a id="when-to-use-program-vs-runtimeexecutable" />

<h5 id="when-to-use-program-vs-runtimeexecutable">
  When to use `program` vs `runtimeExecutable`
</h5>

Use `runtimeExecutable` com `runtimeArgs` para iniciar um servidor de desenvolvimento através de um gerenciador de pacotes. Por exemplo, `"runtimeExecutable": "npm"` com `"runtimeArgs": ["run", "dev"]` executa `npm run dev`.

Use `program` quando você tem um script independente que quer executar com `node` diretamente. Por exemplo, `"program": "server.js"` executa `node server.js`. Passe flags adicionais com `args`.

<a id="open-the-preview-at-a-specific-url" />

<h5 id="open-the-preview-at-a-specific-url">
  Open the preview at a specific URL
</h5>

Por padrão, a visualização abre `http://localhost:<port>`. Defina `url` quando seu servidor precisa de um endereço diferente. Casos comuns são servidores que requerem HTTPS local, aplicativos que usam subdomínios `*.localhost` e aplicativos que o conectam através de um redirecionamento.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 8443,
      "url": "https://localhost:8443"
    }
  ]
}
```

Endereços localhost abrem diretamente, exatamente como o endereço de porta padrão. Isso inclui `localhost`, qualquer subdomínio `*.localhost`, `127.0.0.1` e `::1`. Por segurança, uma `url` localhost deve ser apenas a origem do seu servidor — sem caminho ou query, e a porta deve corresponder à porta da entrada. Para mostrar uma página específica, peça a Claude para navegar lá após a visualização abrir. Uma `url` localhost com um caminho, query ou porta incompatível é relatada como um erro de configuração que nomeia a url e mostra a correção.

Para qualquer outro endereço, Desktop pede sua permissão na primeira vez que a visualização o abre, da mesma forma que quando você navega para um novo site na visualização. Endereços externos podem incluir caminhos. Escolha **Always allow** para pular o prompt para esse site no futuro. Políticas organizacionais que restringem sites externos na visualização ainda se aplicam.

Para visualizar um servidor que você já executa você mesmo, defina `url` sem um comando. Claude anexa a visualização ao seu servidor em execução em vez de iniciar um:

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "url": "https://app.localhost:3000"
    }
  ]
}
```

A `url` deve ser `http` ou `https`, e não deve conter um nome de usuário ou senha.

<h4 id="port-conflicts">
  Port conflicts
</h4>

O campo `autoPort` controla o que acontece quando sua porta preferida já está em uso:

* **`true`**: Claude encontra e usa uma porta livre automaticamente. Adequado para a maioria dos servidores de desenvolvimento.
* **`false`**: Claude falha com um erro. Use isso quando seu servidor deve usar uma porta específica, como para callbacks OAuth ou allowlists CORS.
* **Não definido (padrão)**: Claude pergunta se o servidor precisa dessa porta exata, depois salva sua resposta.

Quando Claude escolhe uma porta diferente, ele passa a porta atribuída ao seu servidor via a variável de ambiente `PORT`.

<h4 id="examples">
  Examples
</h4>

Essas configurações mostram setups comuns para diferentes tipos de projeto:

<Tabs>
  <Tab title="Next.js">
    Esta configuração executa um aplicativo Next.js usando Yarn na porta 3000:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "web",
          "runtimeExecutable": "yarn",
          "runtimeArgs": ["dev"],
          "port": 3000
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Multiple servers">
    Para um monorepo com um servidor frontend e API, defina múltiplas configurações. O frontend usa `autoPort: true` para que escolha uma porta livre se 3000 estiver ocupada, enquanto o servidor API requer a porta 8080 exatamente:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "frontend",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "dev"],
          "cwd": "apps/web",
          "port": 3000,
          "autoPort": true
        },
        {
          "name": "api",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "start"],
          "cwd": "server",
          "port": 8080,
          "env": { "NODE_ENV": "development" },
          "autoPort": false
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Node.js script">
    Para executar um script Node.js diretamente em vez de usar um comando do gerenciador de pacotes, use o campo `program`:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "server",
          "program": "server.js",
          "args": ["--verbose"],
          "port": 4000
        }
      ]
    }
    ```
  </Tab>
</Tabs>

<h2 id="environment-configuration">
  Configuração de ambiente
</h2>

O ambiente que você escolhe ao [iniciar uma sessão](#start-a-session) determina onde Claude é executado e como você se conecta:

* **Local**: é executado em sua máquina com acesso direto aos seus arquivos
* **Cloud**: é executado na infraestrutura gerenciada pela Anthropic por padrão. Sessões continuam mesmo se você fechar o aplicativo.
* **SSH**: é executado em uma máquina remota à qual você se conecta via SSH, como seus próprios servidores, VMs em nuvem ou dev containers
* **WSL** (Windows): é executado dentro de uma [distribuição WSL 2](/docs/pt/desktop-wsl) em sua máquina, usando sua cadeia de ferramentas Linux e caminhos nativos

<h3 id="local-sessions">
  Local sessions
</h3>

O aplicativo desktop nem sempre herda seu ambiente de shell completo. No macOS, quando você inicia o aplicativo do Dock ou Finder, ele lê seu perfil de shell, como `~/.zshrc` ou `~/.bashrc`, para extrair `PATH` e um conjunto fixo de variáveis Claude Code, mas outras variáveis que você exporta lá não são capturadas. No Windows, o aplicativo herda variáveis de ambiente de usuário e sistema mas não lê perfis PowerShell.

Para definir variáveis de ambiente para sessões locais e servidores de desenvolvimento em qualquer plataforma, abra o menu suspenso de ambiente na caixa de prompt, passe o mouse sobre **Local** e clique no ícone de engrenagem para abrir o editor de ambiente local. Variáveis que você salva aqui são armazenadas criptografadas em sua máquina e se aplicam a cada sessão local e servidor de visualização que você inicia. Você também pode adicionar variáveis à chave `env` em seu arquivo `~/.claude/settings.json`, embora essas alcancem apenas sessões Claude e não servidores de desenvolvimento. Veja [variáveis de ambiente](/docs/pt/env-vars) para a lista completa de variáveis suportadas.

[Extended thinking](/docs/pt/model-config#extended-thinking) está ativado por padrão, o que melhora o desempenho em tarefas de raciocínio complexo mas usa tokens adicionais. Na API Anthropic, defina `MAX_THINKING_TOKENS` para `0` no editor de ambiente local para desativar o thinking; isso não tem efeito em Opus 5.5 ou nos modelos Fable, que sempre usam extended thinking. Com o thinking desativado na API Anthropic, Claude Code envia esforço `high` em vez de um nível superior para modelos que você sabe [não aceitam essa combinação](/docs/pt/errors#effort-isnt-available-with-thinking-turned-off), como Opus 5.

Em modelos com [adaptive reasoning](/docs/pt/model-config#adjust-effort-level), valores de `MAX_THINKING_TOKENS` diferentes de `0` são ignorados porque adaptive reasoning controla a profundidade do thinking. Em Opus 4.6 e Sonnet 4.6, defina `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` para `1` para usar um orçamento de thinking fixo; modelos Fable, Sonnet 5 e Opus 4.7 e posterior sempre usam adaptive reasoning e não têm modo de orçamento fixo.

<h4 id="local-sessions-on-managed-devices">
  Local sessions on managed devices
</h4>

Seu administrador pode desativar sessões locais com a [configuração gerenciada `disableDesktopLocalSessions`](#managed-settings). Quando tiverem feito isso, **Local** permanece no menu suspenso de ambiente mas fica acinzentado e não pode ser selecionado, com uma dica de ferramenta dizendo que sua organização o desativou, e no Windows a entrada [WSL](/docs/pt/desktop-wsl), cuja disponibilidade em dispositivos gerenciados é [governada separadamente](/docs/pt/admin-setup#wsl-sessions-in-claude-code-desktop), fica acinzentada da mesma forma. Novas sessões usam como padrão a primeira conexão SSH se uma estiver configurada, e o Desktop mostra uma mensagem de que sessões locais não estão disponíveis neste dispositivo se você tentar continuar uma existente. Escolha um ambiente [SSH](#ssh-sessions) ou [cloud](#cloud-sessions) em vez disso, ou entre em contato com sua equipe de TI.

<h3 id="cloud-sessions">
  Cloud sessions
</h3>

Sessões em nuvem continuam em segundo plano mesmo se você fechar o aplicativo. O uso conta para seus [limites do plano de assinatura](/docs/pt/costs) sem cobranças de computação separadas.

Você pode criar ambientes em nuvem personalizados com diferentes níveis de acesso de rede e variáveis de ambiente. Quando você inicia uma sessão em nuvem, abra o menu suspenso de ambiente na caixa de prompt para gerenciá-los:

* **Add an environment**: selecione **Add cloud environment**
* **Edit or archive one of your own environments**: passe o mouse sobre ele e clique no ícone de engrenagem

Veja [Configure cloud environments](/docs/pt/cloud-environments) para detalhes sobre configuração de acesso de rede e variáveis de ambiente.

<h3 id="ssh-sessions">
  SSH sessions
</h3>

Sessões SSH permitem que você execute Claude Code em uma máquina remota enquanto usa o aplicativo desktop como sua interface. Isso é útil para trabalhar com bases de código que vivem em VMs em nuvem, dev containers ou servidores com hardware ou dependências específicas.

Para adicionar uma conexão SSH, clique no menu suspenso de ambiente antes de iniciar uma sessão e selecione **+ Add SSH connection**. O diálogo solicita:

* **Name**: um rótulo amigável para esta conexão
* **SSH Host**: `user@hostname` ou um host definido em `~/.ssh/config`
* **SSH Port**: padrão é 22 se deixado vazio, ou usa a porta de seu SSH config
* **Identity File**: caminho para sua chave privada, como `~/.ssh/id_rsa`. Deixe vazio para usar a chave padrão ou seu SSH config.

Uma vez adicionada, a conexão aparece no menu suspenso de ambiente. Selecione-a para iniciar uma sessão naquela máquina. Claude é executado na máquina remota com acesso aos seus arquivos e ferramentas.

A máquina remota deve executar Linux ou macOS. O aplicativo desktop instala Claude Code na máquina remota automaticamente na primeira vez que você se conecta. Uma vez conectado, sessões SSH suportam modos de permissão, conectores, plugins e MCP servers.

<h4 id="pre-configure-ssh-connections-for-your-team">
  Pre-configure SSH connections for your team
</h4>

Administradores podem distribuir conexões SSH para membros da equipe adicionando `sshConfigs` a um arquivo de [managed settings](/docs/pt/managed-settings). Conexões definidas desta forma aparecem no menu suspenso de ambiente de cada usuário automaticamente e são mostradas como gerenciadas, para que os usuários possam selecioná-las mas não possam editá-las ou deletá-las no aplicativo.

O exemplo a seguir pré-configura uma única conexão:

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519"
    }
  ]
}
```

Cada entrada requer `id`, `name` e `sshHost`. Os campos `sshPort` e `sshIdentityFile` são opcionais. Os usuários também podem adicionar `sshConfigs` ao seu próprio `~/.claude/settings.json`, que é onde as conexões adicionadas através do diálogo são armazenadas.

<h4 id="restrict-which-ssh-hosts-users-can-connect-to">
  Restrict which SSH hosts users can connect to
</h4>

Administradores podem limitar as sessões SSH do Desktop a um conjunto aprovado de hosts adicionando `sshHostAllowlist` a um arquivo de [managed settings](/docs/pt/managed-settings). Quando definido, os usuários podem se conectar apenas a hosts cujo nome de host resolvido corresponde a um dos padrões. Defina-o como um array vazio para desabilitar sessões SSH completamente.

O exemplo a seguir permite conexões a qualquer host sob `devboxes.example.com` e a um único host bastion nomeado:

```json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

Padrões são insensíveis a maiúsculas e minúsculas. `*` corresponde a qualquer host, e `*.example.com` corresponde a `example.com` e qualquer subdomínio. Qualquer outra coisa é uma correspondência exata. A verificação é executada contra o nome de host após resolução `~/.ssh/config` via `ssh -G`, portanto entradas `Host` aliases e `ProxyCommand`/`ProxyJump` são permitidas desde que o `HostName` resolvido corresponda.

`sshHostAllowlist` é lido apenas de managed settings; valores em configurações de usuário ou projeto são ignorados. Apenas o aplicativo Claude Desktop honra esta configuração; a CLI Claude Code e extensões IDE não a leem, e não restringe comandos `ssh` executados através da ferramenta Bash. Governa quais hosts o aplicativo Desktop se conecta, não saída de rede, portanto combine-o com controles de rede ou zero-trust da sua organização se você precisar de um limite rígido.

<h2 id="enterprise-configuration">
  Configuração corporativa
</h2>

Organizações em planos Team ou Enterprise podem gerenciar o comportamento do aplicativo desktop através de controles do console de administração, arquivos de configurações gerenciadas e políticas de gerenciamento de dispositivos.

<h3 id="admin-console-controls">
  Controles do console de administração
</h3>

Essas configurações são configuradas através do [console de configurações de administração](https://claude.ai/admin-settings/claude-code):

* **Code in the desktop**: controle se usuários em sua organização podem acessar Claude Code no aplicativo desktop
* **Code in the web**: ative ou desative [sessões web](/docs/pt/claude-code-on-the-web) para sua organização
* **Remote Control**: ative ou desative [Remote Control](/docs/pt/remote-control) para sua organização
* **Disable Bypass permissions mode**: impeça usuários em sua organização de ativar o modo bypass permissions

<h3 id="managed-settings">
  Managed settings
</h3>

Configurações gerenciadas sobrescrevem configurações de projeto e usuário e se aplicam a sessões Claude Code no Desktop. Você pode definir essas chaves no arquivo de [configurações gerenciadas](/docs/pt/managed-settings) de sua organização ou enviá-las remotamente através do console de administração.

| Chave                                      | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissions.disableBypassPermissionsMode` | defina como `"disable"` para impedir usuários de ativar o modo Bypass permissions.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `disableAutoMode`                          | defina como `"disable"` para remover o modo [Auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) do seletor de modo. Também aceito em `permissions`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `autoMode`                                 | customize o que o classificador de modo auto confia e bloqueia em sua organização. Veja [Configurar o modo auto](/docs/pt/auto-mode-config).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `browserExternalPageTools`                 | defina como `"disabled"` para impedir Claude de usar ferramentas para ler ou agir em páginas externas no [painel Browser](#browse-external-sites). Os usuários ainda podem navegar para sites externos por conta própria, e as visualizações do servidor de desenvolvimento local não são afetadas.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `disableMobileSimulatorTools`              | defina como `true` para bloquear as ferramentas de Claude para controlar e capturar dispositivos no [painel iOS Simulator](/docs/pt/desktop-ios-simulator#turn-off-simulator-access). O painel permanece utilizável para os toques do próprio usuário; apenas o acesso de Claude é removido. O valor deve ser o booleano JSON `true`; a string `"true"` é ignorada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `disableBrowserExternalNavigation`         | defina como `true` para desativar a navegação externa no [painel Browser](#browse-external-sites) inteiramente. Nem usuários nem Claude podem navegar para sites externos, e as visualizações do servidor localhost dev não são afetadas. O valor deve ser o booleano JSON `true`; a string `"true"` é ignorada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `sshConfigs`                               | pré-configure [conexões SSH](#pre-configure-ssh-connections-for-your-team) que aparecem no dropdown de ambiente. Usuários não podem editar ou excluir conexões gerenciadas.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `sshHostAllowlist`                         | restrinja [sessões SSH](#restrict-which-ssh-hosts-users-can-connect-to) a hosts cujo nome de host resolvido corresponde a um desses padrões. Uma matriz vazia desativa sessões SSH. Lido apenas de configurações gerenciadas.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `disableDesktopLocalSessions`              | defina como `true` para desativar [sessões Code que são executadas no dispositivo](#local-sessions-on-managed-devices), deixando sessões SSH para outros hosts e sessões em nuvem disponíveis. O valor deve ser o booleano JSON `true`. Lido apenas de configurações gerenciadas. Requer Claude Desktop v1.37937.0 ou posterior.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `managedMcpServers`                        | envie configurações de servidor MCP para todos os usuários. Disponível apenas em implantações Desktop de terceiros (3P). Em cada entrada, defina um transporte de `"http"`, `"sse"` ou `"stdio"`, detalhes de conexão e opcionalmente um mapa `toolPolicy` para restringir quais ferramentas desse servidor os usuários podem invocar. Entregue através do arquivo de configurações gerenciadas, MDM ou da política de bloco [`desktop`](/docs/pt/claude-apps-gateway-config#claude-desktop-overlay) de um gateway de aplicativos Claude, já que implantações de terceiros não recebem configurações do console de administração. Para entregá-lo através do gateway, você precisa de Claude Code v2.1.232 ou posterior no servidor do gateway. Esta é a chave do próprio aplicativo desktop; Claude Code lê uma [configuração gerenciada de mesmo nome](/docs/pt/managed-mcp#provide-servers-through-managed-settings) de sua própria, com uma forma de entrada diferente. |

Quais configurações gerenciadas alcançam uma sessão Desktop depende de onde essa sessão é executada. Restrições de modelo como [`availableModels`](/docs/pt/model-config#restrict-model-selection) são aplicadas em sessões Claude Code do Desktop da mesma forma que na CLI do terminal; veja [cobertura de superfície](/docs/pt/model-config#surface-coverage).

* **Sessões locais nesta máquina**: um arquivo de configurações gerenciadas implantado em disco se aplica. Configurações gerenciadas enviadas remotamente através do console de administração também alcançam essas sessões na API da Anthropic quando a sessão se autentica com um [login ou chave elegível](/docs/pt/server-managed-settings#platform-availability), seguindo a mesma [precedência de configurações](/docs/pt/settings#settings-precedence) que a CLI do terminal.
* **[Sessões em nuvem](#cloud-sessions)**: recebem [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings); arquivos implantados em dispositivos não as alcançam, porque são executadas em VMs gerenciadas pela Anthropic. Sessões roteadas para um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) também leem o arquivo de configurações gerenciadas na imagem do executor. [Como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) diz quando esse arquivo se aplica.
* **[Sessões SSH](#ssh-sessions)**: a sessão lê o arquivo de configurações gerenciadas do host remoto. O Desktop em si lê `sshConfigs`, `sshHostAllowlist` e `disableDesktopLocalSessions` das configurações gerenciadas da máquina local.
* **[Sessões Cowork](https://claude.com/docs/cowork/overview)**: em uma sessão Cowork nesta máquina, Claude Code nunca busca configurações do console de administração, mesmo quando o usuário se conecta com uma conta Team ou Enterprise, e lê a política implantada na máquina a menos que sua configuração Claude Desktop defina `requireCoworkFullVmSandbox`. Sessões Cowork remotas não recebem nenhuma. Veja [onde e quando uma política se aplica](/docs/pt/managed-settings#where-and-when-a-policy-applies) para quais arquivos de dispositivo alcançam Cowork, e [regras de permissão MCP](/docs/pt/permissions#mcp) para como as regras `Bash` e `WebFetch` se aplicam às ferramentas de Cowork.

Em sessões locais e SSH, o aplicativo desktop entrega cada conector claude.ai conectado do usuário para Claude Code diretamente. Nenhuma configuração MCP ou `managed-mcp.json` alcança esses conectores, qualquer que seja a fonte de configurações ou local de arquivo que você use. Para bloquear as ferramentas de um conector nessas sessões, use os [controles de ferramentas de conector](/docs/pt/mcp#organization-controls-on-connector-tools) de sua organização. [Como conectores alcançam Claude Code](/docs/pt/mcp#how-connectors-reach-claude-code) mostra quais configurações governam conectores em cada tipo de sessão.

`permissions.disableBypassPermissionsMode` e `disableAutoMode` também funcionam em configurações de usuário e projeto, mas colocá-los em configurações gerenciadas impede que usuários os sobrescrevam.

Para a permissão, plugin e chaves de entrega que apenas uma fonte gerenciada pode definir, veja [Chaves que apenas configurações gerenciadas podem definir](/docs/pt/managed-settings#managed-only-settings).

<h3 id="device-management-policies">
  Políticas de gerenciamento de dispositivos
</h3>

Equipes de TI podem gerenciar o aplicativo desktop através de MDM em macOS ou group policy no Windows. As políticas disponíveis incluem ativar ou desativar o recurso Claude Code, controlar atualizações automáticas e definir uma URL de implantação personalizada.

* **macOS**: configure via domínio de preferência `com.anthropic.claudefordesktop` usando ferramentas como Jamf ou Kandji
* **Windows**: configure via registro em `SOFTWARE\Policies\Claude`

<h3 id="network-access-requirements">
  Requisitos de acesso à rede
</h3>

Desktop carrega seu código de aplicação e conteúdo do usuário de hosts CDN da Anthropic.

```text theme={null}
anthropic.com
*.anthropic.com
claude.ai
*.claude.ai
claude.com
*.claude.com
claude.app
*.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

O tráfego é HTTPS na porta 443 a menos que você configure uma porta personalizada para [OTLP](/docs/pt/monitoring-usage), um gateway LLM ou um servidor MCP.

Para servidores proxy, autoridades de certificado personalizadas, mTLS e os domínios que a CLI autônoma precisa, veja [configuração de rede](/docs/pt/network-config).

Para reduzir o número de wildcards de firewall, permita esses hosts da Anthropic em vez disso. Certos subdomínios são gerados dinamicamente e devem permanecer como wildcards.

```text theme={null}
anthropic.com
api.anthropic.com
a-api.anthropic.com
a-cdn.anthropic.com
s-cdn.anthropic.com
assets-proxy.anthropic.com
claude.ai
a.claude.ai
a-cdn.claude.ai
assets.claude.ai
downloads.claude.ai
*.livepreview.claude.ai
claude.com
platform.claude.com
*.livepreview.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

Se sua organização tiver [IP allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) ativado para Claude, rotear `bridge.claudeusercontent.com` através do mesmo proxy egress que `claude.ai` e `api.anthropic.com`. Se você não conseguir rotear dessa forma, adicione o endereço egress que seu proxy usa para esse host à lista de permissões de IP de sua organização, mas apenas quando esse endereço for dedicado à sua organização: um intervalo de proxy egress compartilhado também admite outros clientes do fornecedor de proxy.

A Anthropic verifica conexões para esse host contra a lista de permissões de IP de sua organização usando o endereço de onde chegam. Se seu proxy enviar tráfego para ele através de um endereço que não está nessa lista de permissões, Claude no Chrome e outros recursos que se conectam através da ponte deixam de funcionar enquanto o resto do aplicativo continua funcionando.

Um [artefato](/docs/pt/artifacts) que carrega uma fonte tipográfica do [Google Fonts](/docs/pt/artifacts#improve-the-visual-design) também solicita `fonts.googleapis.com` e `fonts.gstatic.com`. Ambos os hosts são opcionais. Se você bloqueá-los, artefatos são renderizados em fontes tipográficas de fallback. Bloqueie com uma rejeição rápida em vez de uma queda silenciosa para que a solicitação de fonte falhe imediatamente em vez de atrasar a primeira renderização da página.

Artefatos também podem carregar bibliotecas JavaScript, como React ou um pacote de gráficos, de `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` e `unpkg.com`, e de nenhum outro host externo. Se você bloquear esses hosts, as partes de um artefato que dependem de uma biblioteca não funcionam, e diferentemente de uma fonte bloqueada, uma biblioteca bloqueada não tem fallback. Bloqueie com uma rejeição rápida aqui também, para que uma solicitação de biblioteca bloqueada falhe imediatamente em vez de ficar pendurada até expirar.

<h3 id="authentication-and-sso">
  Autenticação e SSO
</h3>

Organizações corporativas podem exigir SSO para todos os usuários. Veja [autenticação](/docs/pt/authentication) para detalhes de nível de plano e [Configurando SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso) para configuração SAML; a configuração OIDC é coberta no [Guia do Administrador Corporativo Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide).

<h3 id="data-handling">
  Manipulação de dados
</h3>

Claude Code processa seu código localmente em sessões locais ou em sessões em nuvem na infraestrutura gerenciada pela Anthropic, a menos que sua organização as rotear para um [ambiente auto-hospedado](/docs/pt/self-hosted-environments). Sessões em nuvem, incluindo em um ambiente auto-hospedado, enviam conversas e contexto de código para a API da Anthropic para processamento; sessões locais e SSH as enviam para qualquer [provedor de modelo](#feature-comparison) que sua implantação configure, a API da Anthropic por padrão. Veja [manipulação de dados](/docs/pt/data-usage) para detalhes sobre retenção de dados, privacidade e conformidade.

<h3 id="deployment">
  Implantação
</h3>

Desktop pode ser distribuído através de ferramentas de implantação corporativa:

* **macOS**: distribua via MDM como Jamf ou Kandji usando o instalador `.dmg`
* **Windows**: implante via pacote MSIX. Veja [Deploy Claude Desktop for Windows](https://support.claude.com/en/articles/12622703-deploy-claude-desktop-for-windows) para opções de implantação corporativa incluindo instalação silenciosa

Para os domínios a permitir na sua firewall, veja [requisitos de acesso à rede](#network-access-requirements) acima. Para configurações de proxy, autoridades de certificado personalizadas e gateways LLM, veja [configuração de rede](/docs/pt/network-config).

Para a referência completa de configuração corporativa, veja o [guia de configuração corporativa](https://support.claude.com/en/articles/12622667-enterprise-configuration).

<h2 id="coming-from-the-cli">
  Vindo do CLI?
</h2>

Se você já usa o CLI do Claude Code, Desktop executa o mesmo mecanismo subjacente com uma interface gráfica. Você pode executar ambos simultaneamente na mesma máquina, até mesmo no mesmo projeto. Cada um mantém sua própria lista de sessão, e você pode trazer uma sessão CLI para Desktop. Eles compartilham configuração e memória de projeto via arquivos CLAUDE.md.

Para mover uma sessão CLI para Desktop, execute `/desktop` no terminal. Claude salva sua sessão e a abre no aplicativo desktop, depois sai do CLI. Este comando está disponível em macOS e Windows x64 quando você está conectado com uma assinatura Claude. Não está disponível com autenticação de chave de API ou em Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.

Para retomar uma sessão CLI de dentro do Desktop, digite `/resume` na caixa de prompt. Desktop lista as sessões que você iniciou a partir do CLI, e você pode pesquisá-las por título, pasta ou branch e visualizar onde cada uma parou. Selecione uma sessão e ela continua no aplicativo com sua conversa completa e contexto.

<Tip>
  Quando usar Desktop vs CLI: use Desktop quando você quer gerenciar sessões paralelas em uma janela, organizar painéis lado a lado ou revisar alterações visualmente. Use o CLI quando você precisa de scripting, automação ou prefere um fluxo de trabalho de terminal.
</Tip>

<h3 id="cli-flag-equivalents">
  CLI flag equivalents
</h3>

Esta tabela mostra o equivalente do aplicativo desktop para flags CLI comuns. Flags não listadas não têm equivalente desktop porque são projetadas para scripting ou automação.

| CLI                                        | Equivalente desktop                                                                                                                                                                   |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--model sonnet`                           | Menu suspenso de modelo ao lado do botão enviar                                                                                                                                       |
| `--resume`, `--continue`                   | Clique em uma sessão na barra lateral, ou digite `/resume` na caixa de prompt para retomar uma sessão que você iniciou a partir do CLI                                                |
| `--permission-mode`                        | Seletor de modo ao lado do botão enviar                                                                                                                                               |
| `--dangerously-skip-permissions`           | Modo Bypass permissions. Em planos Pro e Max, ative em Configurações → Claude Code → "Allow bypass permissions mode"; em planos Team e Enterprise, a política organizacional controla |
| `--add-dir`                                | Adicione múltiplos repos com o botão **+** em sessões na nuvem                                                                                                                        |
| `--allowedTools`, `--disallowedTools`      | Nenhum equivalente por sessão. Regras de permissão em [arquivos de configuração](/docs/pt/settings) ainda se aplicam.                                                                      |
| `--verbose`                                | [Modo de visualização Verbose](#switch-view-modes) no menu suspenso Transcript view                                                                                                   |
| `--print`, `--output-format`               | Não disponível. Desktop é apenas interativo.                                                                                                                                          |
| Variável de ambiente `ANTHROPIC_MODEL`     | Menu suspenso de modelo ao lado do botão enviar                                                                                                                                       |
| Variável de ambiente `MAX_THINKING_TOKENS` | Defina no editor de ambiente local. Veja [configuração de ambiente](#environment-configuration).                                                                                      |

<h3 id="shared-configuration">
  Shared configuration
</h3>

Desktop e CLI leem os mesmos arquivos de configuração, então sua configuração é transferida:

* Arquivos **[CLAUDE.md](/docs/pt/memory)** e `CLAUDE.local.md` em seu projeto são usados por ambos
* **[MCP servers](/docs/pt/mcp)** configurados em `~/.claude.json` ou `.mcp.json` funcionam em ambos
* **[Hooks](/docs/pt/hooks)** e **[skills](/docs/pt/skills)** definidos em configurações se aplicam a ambos
* **[Configurações](/docs/pt/settings)** em `~/.claude.json` e `~/.claude/settings.json` são compartilhadas. Regras de permissão, ferramentas permitidas e outras configurações em `settings.json` se aplicam a sessões Desktop.
* **Modelos**: os mesmos [modelos](/docs/pt/model-config#available-models) estão disponíveis em ambos. Em Desktop, selecione o modelo no menu suspenso ao lado do botão enviar. Você pode alterar o modelo durante a sessão a partir do mesmo menu suspenso.

<h4 id="mcp-servers-from-the-claude-desktop-chat-app">
  MCP servers from the Claude Desktop chat app
</h4>

O aplicativo Desktop carrega MCP servers de `claude_desktop_config.json` em sessões da aba Code local, juntamente com servers de `~/.claude.json` e `.mcp.json`. Um server definido em `claude_desktop_config.json` está disponível tanto na superfície de chat Desktop quanto em sessões da aba Code local.

Se você definir o mesmo nome de server em `claude_desktop_config.json` e em `~/.claude.json` ou `.mcp.json`, a aba Code em sessões locais se conecta uma vez e usa a definição de `claude_desktop_config.json`.

O aplicativo também re-entrega stdio servers de `~/.claude.json` para o CLI incorporado em sessões locais. Quando o nível superior de `~/.claude.json` (escopo de usuário) e `.mcp.json` definem o mesmo nome de stdio server, a aba Code usa a definição de `~/.claude.json`, diferindo da [hierarquia de escopo](/docs/pt/mcp#scope-hierarchy-and-precedence) do CLI.

<Note>
  O CLI autônomo não lê `claude_desktop_config.json`. Em macOS e WSL, execute `claude mcp add-from-claude-desktop` para copiar esses servers em `~/.claude.json`. Veja [Importar MCP servers do Claude Desktop](/docs/pt/mcp#import-mcp-servers-from-claude-desktop) para o fluxo de importação e opções de escopo.
</Note>

<h3 id="feature-comparison">
  Feature comparison
</h3>

Esta tabela compara capacidades principais entre CLI e Desktop. Para uma lista completa de flags CLI, veja a [referência CLI](/docs/pt/cli-reference).

| Recurso                                                 | CLI                                                                           | Desktop                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------------------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Modos de permissão                                      | Todos os modos incluindo `dontAsk`                                            | Manual, Aceitar edições, Plan e Auto. Bypass permissions aparece no seletor de modo uma vez habilitado: através do toggle Configurações em planos Pro e Max, ou através da política organizacional em planos Team e Enterprise                                                                                                                                            |
| [Provedores de terceiros](/docs/pt/third-party-integrations) | Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry              | API da Anthropic por padrão. Para roteamento de gateway, veja [conectar o aplicativo desktop a um gateway](/docs/pt/llm-gateway-connect#desktop-app). Para executar a aba Code em Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou um gateway LLM auto-hospedado, veja [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview). |
| [MCP servers](/docs/pt/mcp)                                  | Configure em arquivos de configuração                                         | UI de Connectors para sessões locais e SSH, ou arquivos de configuração                                                                                                                                                                                                                                                                                                   |
| [Plugins](/docs/pt/plugins/overview)                         | Comando `/plugin`                                                             | UI do gerenciador de plugins                                                                                                                                                                                                                                                                                                                                              |
| @mention de arquivos                                    | Baseado em texto                                                              | Com autocompletar; sessões locais e SSH apenas                                                                                                                                                                                                                                                                                                                            |
| Anexos de arquivo                                       | Não disponível                                                                | Imagens, PDFs                                                                                                                                                                                                                                                                                                                                                             |
| Isolamento de sessão                                    | Flag [`--worktree`](/docs/pt/cli-reference)                                        | Opção **worktree** ao iniciar uma sessão                                                                                                                                                                                                                                                                                                                                  |
| Múltiplas sessões                                       | Terminais separados                                                           | Abas na barra lateral                                                                                                                                                                                                                                                                                                                                                     |
| Tarefas recorrentes                                     | Cron jobs, pipelines CI                                                       | [Tarefas agendadas](/docs/pt/desktop-scheduled-tasks)                                                                                                                                                                                                                                                                                                                          |
| Computer use                                            | [Ativar via `/mcp`](/docs/pt/computer-use) no macOS                                | [Controle de aplicativo e tela](#let-claude-use-your-computer) no macOS e Windows                                                                                                                                                                                                                                                                                         |
| iOS simulator                                           | Dirija o simulador via [computer use](/docs/pt/computer-use#test-a-simulator-flow) | [Painel iOS Simulator](/docs/pt/desktop-ios-simulator) abre automaticamente                                                                                                                                                                                                                                                                                                    |
| Integração Dispatch                                     | Não disponível                                                                | [Sessões Dispatch](#sessions-from-dispatch) na barra lateral                                                                                                                                                                                                                                                                                                              |
| Scripting e automação                                   | [`--print`](/docs/pt/cli-reference), [Agent SDK](/docs/pt/headless)                     | Não disponível                                                                                                                                                                                                                                                                                                                                                            |

<h3 id="what’s-not-available-in-desktop">
  What's not available in Desktop
</h3>

Os seguintes recursos não estão disponíveis em Desktop, exceto onde observado:

* **Provedores de terceiros**: Desktop se conecta à API da Anthropic por padrão. Para rotear Desktop através de um gateway, ou para executar a aba Code em Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou um gateway LLM auto-hospedado, siga os links na [linha Provedores de terceiros](#feature-comparison).
* **Linux (beta)**: Computer Use ainda não está disponível no aplicativo desktop Linux. Veja [Claude Desktop no Linux](/docs/pt/desktop-linux).
* **Sugestões de código inline**: Desktop não fornece sugestões no estilo autocompletar. Funciona através de prompts conversacionais e alterações de código explícitas.
* **Equipes de agentes**: equipes coordenadas, onde Claude como líder da equipe atribui tarefas aos colegas de equipe de uma lista de tarefas compartilhada, estão disponíveis no [CLI](/docs/pt/agent-teams), não em Desktop. Para trabalho multi-agente dentro de uma sessão, use [dynamic workflows](/docs/pt/workflows), que são executados em Desktop; Claude também pode [enviar mensagens e gerenciar suas outras sessões](#work-across-sessions) diretamente.
* **Comandos terminal-dialog**: comandos integrados que abrem um painel interativo no terminal se comportam de forma diferente na aba Code. Edite [arquivos de configuração](/docs/pt/settings) diretamente para gerenciar regras de permissão e configuração, ou execute os comandos a partir do CLI autônomo.
  * Comandos sem forma de argumento, como `/permissions`, respondem com `isn't available in this environment`.
  * `/config` abre Configurações → Claude Code. Texto após o comando é ignorado, então `/config theme=dark` não define o tema.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

As seções abaixo cobrem problemas específicos do aplicativo desktop. Para erros de API de tempo de execução que aparecem no chat como `API Error: 500`, `529 Overloaded`, `429` ou `Prompt is too long`, veja a [referência de erros](/docs/pt/errors). Esses erros e suas correções são os mesmos em CLI, desktop e web.

<h3 id="check-your-version">
  Verificar sua versão
</h3>

Para ver qual versão do aplicativo desktop você está executando:

* **macOS**: clique em **Claude** na barra de menu, depois **About Claude**
* **Windows**: clique em **Help**, depois **About**

Clique no número da versão para copiá-lo para sua área de transferência.

<h3 id="403-or-authentication-errors-in-the-code-tab">
  Erros 403 ou autenticação na aba Code
</h3>

Se você vê `Error 403: Forbidden` ou outras falhas de autenticação ao usar a aba Code:

1. Saia e entre novamente no menu do aplicativo. Esta é a correção mais comum.
2. Verifique se você tem uma assinatura paga ativa: Pro, Max, Team ou Enterprise.
3. Se o CLI funciona mas Desktop não, saia completamente do aplicativo desktop, não apenas feche a janela, depois reabra e entre novamente.
4. Verifique sua conexão de internet e configurações de proxy.

<h3 id="blank-or-stuck-screen-on-launch">
  Tela em branco ou travada ao iniciar
</h3>

Se o aplicativo abre mas mostra uma tela em branco ou não responsiva:

1. Reinicie o aplicativo.
2. Verifique se há atualizações pendentes. Em macOS e Windows, o aplicativo se atualiza automaticamente ao iniciar; em Linux, atualize através do apt conforme descrito em [Claude Desktop no Linux](/docs/pt/desktop-linux).
3. Em uma rede gerenciada, confirme que seu firewall permite os hosts CDN em [requisitos de acesso à rede](#network-access-requirements).
4. No Windows, verifique o Event Viewer para logs de crash em **Windows Logs → Application**.

<h3 id="failed-to-load-session">
  "Failed to load session"
</h3>

Se você vê `Failed to load session`, a pasta selecionada pode não existir mais, um repositório Git pode exigir Git LFS que não está instalado, ou permissões de arquivo podem impedir acesso. Tente selecionar uma pasta diferente ou reinicie o aplicativo.

<h3 id="session-not-finding-installed-tools">
  Sessão não encontrando ferramentas instaladas
</h3>

Se Claude não consegue encontrar ferramentas como `npm`, `node` ou outros comandos CLI, verifique se as ferramentas funcionam em seu terminal regular, verifique se seu perfil de shell configura adequadamente PATH e reinicie o aplicativo desktop para recarregar variáveis de ambiente.

<h3 id="git-and-git-lfs-errors">
  Erros de Git e Git LFS
</h3>

Sessões que executam em seu próprio worktree precisam de Git. Se você vê "Git is required," instale [Git](https://git-scm.com/downloads), ou [Git para Windows](https://git-scm.com/downloads/win) no Windows, e tente novamente. No Windows, versões do Claude Desktop anteriores a 1.49585.0 pediam Git antes de iniciar qualquer sessão local; se você vê esse prompt e não está usando worktrees, atualize o aplicativo.

Se você vê "Git LFS is required by this repository but is not installed," instale Git LFS de [git-lfs.com](https://git-lfs.com/), execute `git lfs install` e reinicie o aplicativo.

<h3 id="mcp-servers-not-working-on-windows">
  MCP servers não funcionando no Windows
</h3>

Se toggles de MCP server não respondem ou servidores falham em conectar no Windows, verifique se o servidor está adequadamente configurado em suas configurações, reinicie o aplicativo, verifique se o processo do servidor está em execução no Task Manager e revise logs do servidor para erros de conexão.

<h3 id="app-won’t-quit">
  Aplicativo não quer sair
</h3>

* **macOS**: pressione Cmd+Q. Se o aplicativo não responder, use Force Quit com Cmd+Option+Esc, selecione Claude e clique Force Quit.
* **Windows**: use Task Manager com Ctrl+Shift+Esc para encerrar o processo Claude.

<h3 id="windows-specific-issues">
  Problemas específicos do Windows
</h3>

* **PATH não atualizado após instalação**: abra uma nova janela de terminal. PATH é atualizado apenas para novas sessões de terminal.
* **Erro de instalação concorrente**: se você vê um erro sobre outra instalação em progresso mas não há uma, tente executar o instalador como Administrador.

<h3 id="branch-doesn’t-exist-yet-when-opening-in-cli">
  "Branch doesn't exist yet" ao abrir em CLI
</h3>

Sessões na nuvem podem criar branches que não existem em sua máquina local. Clique no nome do branch na barra de ferramentas da sessão para copiá-lo, depois busque-o localmente:

```bash theme={null}
git fetch origin <branch-name>
git checkout <branch-name>
```

<h3 id="still-stuck">
  Ainda preso?
</h3>

* Abra Help → Get Support no aplicativo desktop, ou visite o [centro de suporte Claude](https://support.claude.com/) diretamente
* Para problemas que também se reproduzem no CLI `claude` autossuficiente, pesquise ou registre um bug em [GitHub Issues](https://github.com/anthropics/claude-code/issues)

Ao registrar um problema, inclua a versão do seu aplicativo desktop, seu sistema operacional, a mensagem de erro exata e logs relevantes. Em macOS, verifique Console.app. No Windows, verifique Event Viewer → Windows Logs → Application. Revise trechos de log antes de postá-los em um problema público; eles podem incluir caminhos de arquivo e outros detalhes do seu ambiente.
