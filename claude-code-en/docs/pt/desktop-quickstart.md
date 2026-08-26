> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comece com o aplicativo de desktop

> Instale Claude Code no desktop e inicie sua primeira sessão de codificação

O aplicativo de desktop oferece Claude Code com uma interface gráfica construída para executar múltiplas sessões lado a lado: uma barra lateral para gerenciar trabalho paralelo, um layout com arrastar e soltar com terminal integrado e editor de arquivos, revisão visual de diff, visualização ao vivo do aplicativo, monitoramento de PR do GitHub com mesclagem automática e tarefas agendadas. Nenhum terminal necessário.

<CardGroup cols={3}>
  <Card title="Download for macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universal build for Intel and Apple Silicon
  </Card>

  <Card title="Download for Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    For x64 processors
  </Card>

  <Card title="Get Claude for Linux (beta)" icon="linux" href="/docs/en/desktop-linux">
    apt or .deb for Ubuntu and Debian
  </Card>
</CardGroup>

For Windows ARM64, download the [ARM64 installer](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). On Linux, install with apt; see [Claude Desktop on Linux](/docs/en/desktop-linux).

<Note>
  Claude Code requer uma [assinatura Pro, Max, Team ou Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Esta página orienta você na instalação do aplicativo e no início de sua primeira sessão. Se você já está configurado, consulte [Usar Claude Code Desktop](/docs/pt/desktop) para a referência completa.

O aplicativo de desktop tem três abas:

* **Chat**: Conversa geral sem acesso a arquivos, semelhante ao claude.ai.
* **Cowork**: Um agente autônomo em segundo plano que trabalha em tarefas em uma máquina virtual em sandbox com seu próprio ambiente, executando independentemente enquanto você faz outro trabalho. As sessões Cowork no dispositivo executam a VM no seu computador; as sessões Cowork remotas executam em uma VM gerenciada pela Anthropic.
* **Code**: Um assistente de codificação interativo com acesso direto aos seus arquivos locais. Você revisa e aprova cada alteração em tempo real.

Chat e Cowork são cobertos no [Centro de Ajuda do Claude](https://support.claude.com/); a instalação e implantação do aplicativo de desktop são cobertas nos [artigos de suporte do Claude Desktop](https://support.claude.com/en/collections/16163169-claude-desktop). Esta página se concentra na aba **Code**.

<h2 id="install">
  Instalar
</h2>

<Steps>
  <Step title="Instale e faça login">
    No macOS e Windows, baixe o instalador dos links acima e execute-o. No Linux, siga as etapas de instalação em [Claude Desktop no Linux](/docs/pt/desktop-linux). Inicie Claude na sua pasta Applications no macOS, no menu Iniciar no Windows ou no seu inicializador de aplicativos no Linux e faça login com sua conta Anthropic.
  </Step>

  <Step title="Abra a aba Code">
    Clique na aba **Code** no topo do centro. Se clicar em Code solicitar que você faça upgrade, você precisa [se inscrever em um plano pago](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade) primeiro. Se solicitar que você faça login online, conclua o login e reinicie o aplicativo. Se você vir um erro 403, consulte [solução de problemas de autenticação](/docs/pt/desktop#403-or-authentication-errors-in-the-code-tab).
  </Step>
</Steps>

O aplicativo de desktop inclui Claude Code. Você não precisa instalar Node.js ou a CLI separadamente. Para usar `claude` do terminal, instale a CLI separadamente. Consulte [Comece com a CLI](/docs/pt/quickstart).

<h2 id="start-your-first-session">
  Inicie sua primeira sessão
</h2>

Com a aba Code aberta, escolha um projeto e dê a Claude algo para fazer.

<Steps>
  <Step title="Escolha um ambiente e pasta">
    Selecione **Local** para executar Claude em sua máquina usando seus arquivos diretamente. Clique em **Select folder** e escolha seu diretório de projeto.

    <Tip>
      Comece com um pequeno projeto que você conhece bem. É a forma mais rápida de ver o que Claude Code pode fazer. No Windows, [Git](https://git-scm.com/downloads/win) deve estar instalado para que as sessões locais funcionem. A maioria dos Macs inclui Git por padrão.
    </Tip>

    Você também pode selecionar:

    * **Remote**: Execute sessões na infraestrutura em nuvem da Anthropic que continuam mesmo se você fechar o aplicativo. As sessões remotas usam a mesma infraestrutura que [Claude Code na web](/docs/pt/claude-code-on-the-web).
    * **SSH**: Conecte-se a uma máquina remota via SSH, como seus próprios servidores, VMs em nuvem ou dev containers. O Desktop instala Claude Code na máquina remota automaticamente na primeira vez que você se conecta.
    * **WSL** (Windows): Execute a sessão dentro de uma [distribuição WSL 2](/docs/pt/desktop-wsl); Claude Code, ferramentas e git são executados no lado Linux com caminhos nativos.
  </Step>

  <Step title="Escolha um modelo">
    Selecione um modelo no dropdown ao lado do botão enviar. Consulte [modelos](/docs/pt/model-config#available-models) para uma comparação dos modelos disponíveis. Você pode alterar o modelo mais tarde no mesmo dropdown.
  </Step>

  <Step title="Diga a Claude o que fazer">
    Digite o que você quer que Claude faça:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    Uma [sessão](/docs/pt/desktop#work-in-parallel-with-sessions) é uma conversa com Claude sobre seu código. Cada sessão rastreia seu próprio contexto e alterações, para que você possa trabalhar em várias tarefas sem que elas interfiram uma com a outra.
  </Step>

  <Step title="Revise e aceite as alterações">
    Por padrão, a aba Code inicia no [modo Manual](/docs/pt/desktop#choose-a-permission-mode), onde Claude propõe alterações e aguarda sua aprovação antes de aplicá-las. Você verá:

    1. Uma [visualização de diff](/docs/pt/desktop#review-changes-with-diff-view) mostrando exatamente o que mudará em cada arquivo
    2. Botões Accept/Reject para aprovar ou recusar cada alteração
    3. Atualizações em tempo real conforme Claude trabalha em sua solicitação

    Se você recusar uma alteração, Claude perguntará como você gostaria de proceder de forma diferente. Seus arquivos não são modificados até que você aceite.
  </Step>
</Steps>

<h2 id="now-what">
  E agora?
</h2>

Você fez sua primeira edição. Para a referência completa sobre tudo que o Desktop pode fazer, consulte [Usar Claude Code Desktop](/docs/pt/desktop). Aqui estão algumas coisas para tentar a seguir.

**Interrompa e direcione.** Você pode redirecionar Claude a qualquer momento. Clique no botão parar para interromper imediatamente, ou digite uma correção e pressione **Enter** para enviá-la sem parar a ação em execução. De qualquer forma, você não precisa esperar que termine ou começar novamente.

**Dê a Claude mais contexto.** Digite `@filename` na caixa de prompt para puxar um arquivo específico para a conversa, anexe imagens e PDFs usando o botão de anexo, ou arraste e solte arquivos diretamente no prompt. Quanto mais contexto Claude tiver, melhores serão os resultados. Consulte [Adicionar arquivos e contexto](/docs/pt/desktop#add-files-and-context-to-prompts).

**Use skills para tarefas repetíveis.** Digite `/` ou clique em **+** → **Slash commands** para procurar [comandos integrados](/docs/pt/commands), [skills personalizadas](/docs/pt/skills) e skills de plugin. Skills são prompts reutilizáveis que você pode invocar sempre que precisar, como listas de verificação de revisão de código ou etapas de implantação.

**Revise as alterações antes de fazer commit.** Depois que Claude edita arquivos, um indicador `+12 -1` aparece. Clique nele para abrir a [visualização de diff](/docs/pt/desktop#review-changes-with-diff-view), revise as modificações arquivo por arquivo e comente em linhas específicas. Claude lê seus comentários e revisa. Clique em **Review code** para que Claude avalie os diffs e deixe sugestões inline.

**Ajuste quanto controle você tem.** Seu [modo de permissão](/docs/pt/desktop#choose-a-permission-mode) define quanto Claude pode fazer sem pedir aprovação:

* **Manual**: o padrão. Claude pede antes de editar arquivos ou executar comandos.
* **Accept edits**: Claude aceita automaticamente edições de arquivo para iteração mais rápida.
* **Plan**: Claude propõe uma abordagem sem editar nenhum arquivo, o que é útil antes de uma grande refatoração.

**Adicione plugins para mais capacidades.** Clique no botão **+** ao lado da caixa de prompt e selecione **Plugins** para procurar e instalar [plugins](/docs/pt/desktop#install-plugins) que adicionam skills, agentes, MCP servers e muito mais.

**Organize seu espaço de trabalho.** Arraste os painéis de chat, diff, terminal, arquivo e visualização para qualquer layout que desejar. Abra o terminal com **Ctrl+\`** para executar comandos ao lado de sua sessão, ou clique em um caminho de arquivo para abri-lo no painel de arquivo. Consulte [Organize seu espaço de trabalho](/docs/pt/desktop#arrange-your-workspace).

**Visualize seu aplicativo.** Quando você executa seu servidor de desenvolvimento no desktop, seu aplicativo abre no painel do Browser, que também pode [abrir sites externos](/docs/pt/desktop#browse-external-sites). Claude pode visualizar o aplicativo em execução, testar endpoints, inspecionar logs e iterar sobre o que vê. Consulte [Visualize seu aplicativo](/docs/pt/desktop#preview-your-app).

**Rastreie sua solicitação de pull.** Depois de abrir um PR, Claude Code monitora os resultados de verificação de CI e pode corrigir automaticamente falhas ou mesclar o PR assim que todas as verificações passarem. Consulte [Monitore o status da solicitação de pull](/docs/pt/desktop#monitor-pull-request-status).

**Coloque Claude em um cronograma.** Configure [tarefas agendadas](/docs/pt/desktop-scheduled-tasks) para executar Claude automaticamente em uma base recorrente: uma revisão de código diária todas as manhãs, uma auditoria de dependência semanal ou um briefing que extrai de suas ferramentas conectadas.

**Escale quando estiver pronto.** Abra [sessões paralelas](/docs/pt/desktop#work-in-parallel-with-sessions) na barra lateral para trabalhar em várias tarefas ao mesmo tempo, cada uma em seu próprio Git worktree, e abra o [painel de tarefas](/docs/pt/desktop#watch-background-tasks) para observar os subagentes e comandos em segundo plano que uma sessão está executando. Abra um [side chat](/docs/pt/desktop#ask-a-side-question-without-derailing-the-session) para fazer uma pergunta sem descarrilar a thread principal. Envie [trabalho de longa duração para a nuvem](/docs/pt/desktop#run-long-running-tasks-remotely) para que continue mesmo se você fechar o aplicativo, ou [continue uma sessão na web ou em seu IDE](/docs/pt/desktop#continue-in-another-surface) se uma tarefa levar mais tempo do que o esperado. [Conecte ferramentas externas](/docs/pt/desktop#extend-claude-code) como GitHub, Slack e Linear para reunir seu fluxo de trabalho.

<h2 id="coming-from-the-cli">
  Vindo da CLI?
</h2>

Desktop executa o mesmo mecanismo que a CLI com uma interface gráfica. Você pode executar ambos simultaneamente no mesmo projeto, e eles compartilham configuração (arquivos CLAUDE.md, MCP servers, hooks, skills e configurações). Para uma comparação completa de recursos, equivalentes de flag e o que não está disponível no Desktop, consulte [Comparação de CLI](/docs/pt/desktop#coming-from-the-cli).

<h2 id="what’s-next">
  Próximas etapas
</h2>

* [Usar Claude Code Desktop](/docs/pt/desktop): modos de permissão, sessões paralelas, visualização de diff, conectores e configuração corporativa
* [Solução de problemas](/docs/pt/desktop#troubleshooting): soluções para erros comuns e problemas de configuração
* [Melhores práticas](/docs/pt/best-practices): dicas para escrever prompts eficazes e aproveitar ao máximo Claude Code
* [Fluxos de trabalho comuns](/docs/pt/common-workflows): tutoriais para depuração, refatoração, testes e muito mais
