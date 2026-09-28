> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comece com o aplicativo de desktop

> Instale Claude Code no desktop e inicie sua primeira sessão de codificação

O aplicativo de desktop oferece Claude Code com uma interface gráfica construída para executar múltiplas sessões lado a lado: uma barra lateral para gerenciar trabalho paralelo, um layout com arrastar e soltar com terminal integrado e editor de arquivos, revisão visual de diff, visualização ao vivo do aplicativo, monitoramento de PR do GitHub com mesclagem automática e tarefas agendadas. Nenhum terminal necessário.

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

<Note>
  Claude Code requer uma [assinatura Pro, Max, Team ou Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Esta página orienta você na instalação do aplicativo e no início de sua primeira sessão. Se você já está configurado, consulte [Usar Claude Code Desktop](/docs/pt/desktop) para a referência completa.

O aplicativo de desktop tem três abas:

* **Chat**: Conversa geral sem acesso a arquivos, semelhante ao claude.ai.
* **Cowork**: Um agente autônomo em segundo plano que trabalha em tarefas em uma máquina virtual em sandbox com seu próprio ambiente, executando independentemente enquanto você faz outro trabalho. As sessões Cowork no dispositivo executam a VM no seu computador; as sessões Cowork remotas executam em uma VM gerenciada pela Anthropic.
* **Code**: Um assistente de codificação interativo com acesso direto aos seus arquivos locais. Dependendo do modo de permissão, você aprova cada alteração conforme Claude a propõe ou revisa as alterações após Claude fazê-las.

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
    Selecione **Local** para executar Claude em sua máquina usando seus arquivos diretamente. Clique em **Select folder** e escolha o diretório do seu projeto.

    <Tip>
      Comece com um pequeno projeto que você conhece bem. É a forma mais rápida de ver o que Claude Code pode fazer.
    </Tip>

    Você também pode selecionar:

    * **Cloud**: Execute sessões na nuvem que continuam mesmo se você fechar o aplicativo. Veja [Use Claude Code in the cloud](/docs/pt/claude-code-on-the-web) para saber como as sessões na nuvem funcionam.
    * **SSH**: Conecte-se a uma máquina remota via SSH, como seus próprios servidores, VMs na nuvem ou contêineres de desenvolvimento. O Desktop instala Claude Code na máquina remota automaticamente na primeira vez que você se conecta.
    * **WSL** (Windows): Execute a sessão dentro de uma [distribuição WSL 2](/docs/pt/desktop-wsl); Claude Code, ferramentas e git são executados no lado Linux com caminhos nativos.
  </Step>

  <Step title="Escolha um modelo">
    Selecione um modelo no menu suspenso ao lado do botão enviar. Veja [models](/docs/pt/model-config#available-models) para uma comparação dos modelos disponíveis. Você pode alterar o modelo posteriormente no mesmo menu suspenso.
  </Step>

  <Step title="Diga a Claude o que fazer">
    Digite o que você quer que Claude faça:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    Uma [session](/docs/pt/desktop#work-in-parallel-with-sessions) é uma conversa com Claude sobre seu código. Cada sessão rastreia seu próprio contexto e alterações.
  </Step>

  <Step title="Revise e aceite as alterações">
    O que acontece a seguir depende do [permission mode](/docs/pt/desktop#choose-a-permission-mode) mostrado no seletor ao lado do botão enviar:

    * **Auto or Accept edits**: Claude aplica suas alterações de arquivo, e um indicador como `+12 -1` aparece para que você possa revisá-las na visualização de diff
    * **Manual**: Claude propõe cada alteração e aguarda sua aprovação antes de aplicá-la. Seus arquivos não são modificados até que você aceite, e se você rejeitar uma alteração, Claude pergunta como você gostaria de proceder

    No modo Manual, você verá:

    1. Uma [diff view](/docs/pt/desktop#review-changes-with-diff-view) mostrando exatamente o que mudará em cada arquivo
    2. Botões Accept/Reject para aprovar ou recusar cada alteração
    3. Atualizações em tempo real conforme Claude trabalha em sua solicitação
  </Step>
</Steps>

<h2 id="now-what">
  E agora?
</h2>

Você fez sua primeira edição. Para a referência completa sobre tudo que o Claude Code Desktop pode fazer, consulte [Use Claude Code Desktop](/docs/pt/desktop). Aqui estão algumas coisas para tentar a seguir.

**Interrompa e redirecione.** Você pode redirecionar Claude em qualquer ponto. Clique no botão de parada para interromper imediatamente, ou digite uma correção e pressione **Enter** para enviá-la sem parar a ação em execução. De qualquer forma, você não precisa esperar que termine ou começar novamente.

**Dê mais contexto a Claude.** Digite `@filename` na caixa de prompt para puxar um arquivo específico para a conversa, anexe imagens e PDFs usando o botão de anexo, ou arraste e solte arquivos diretamente no prompt. Quanto mais contexto Claude tiver, melhores serão os resultados. Consulte [Add files and context](/docs/pt/desktop#add-files-and-context-to-prompts).

**Use skills para tarefas repetíveis.** Digite `/` ou clique em **+** → **Slash commands** para procurar [built-in commands](/docs/pt/commands), [custom skills](/docs/pt/skills) e skills de plugin. Skills são prompts reutilizáveis que você pode invocar sempre que precisar, como listas de verificação de revisão de código ou etapas de implantação.

**Revise as alterações antes de fazer commit.** Depois que Claude edita arquivos, um indicador `+12 -1` aparece. Clique nele para abrir a [diff view](/docs/pt/desktop#review-changes-with-diff-view), revise as modificações arquivo por arquivo e comente em linhas específicas. Claude lê seus comentários e revisa. Clique em **Review code** para que Claude avalie os diffs em si e deixe sugestões inline.

**Ajuste quanto controle você tem.** Seu [permission mode](/docs/pt/desktop#choose-a-permission-mode) define quanto Claude pode fazer sem pedir aprovação:

* **Auto**: um classificador revisa ações em segundo plano e bloqueia as arriscadas em vez de pedir a você.
* **Manual**: Claude pergunta antes de editar arquivos ou executar comandos.
* **Accept edits**: Claude aceita automaticamente edições de arquivo para iteração mais rápida.
* **Plan**: Claude propõe uma abordagem sem editar nenhum arquivo, o que é útil antes de um grande refactor.

**Adicione plugins para mais capacidades.** Clique no botão **+** ao lado da caixa de prompt e selecione **Plugins** para procurar e instalar [plugins](/docs/pt/desktop#install-plugins) que adicionam skills, agentes, servidores MCP e muito mais.

**Organize seu espaço de trabalho.** Arraste os painéis de chat, diff, terminal, arquivo e navegador para qualquer layout que desejar. Abra o terminal com **Ctrl+\`** para executar comandos ao lado de sua sessão, ou clique em um caminho de arquivo para abri-lo no painel de arquivo. Consulte [Arrange your workspace](/docs/pt/desktop#arrange-your-workspace).

**Visualize seu aplicativo.** Quando você executa seu servidor de desenvolvimento no desktop, seu aplicativo abre no painel do Browser, que também pode [open external sites](/docs/pt/desktop#browse-external-sites). Claude pode visualizar o aplicativo em execução, testar endpoints, inspecionar logs e iterar sobre o que vê. Consulte [Preview your app](/docs/pt/desktop#preview-your-app).

**Acompanhe sua solicitação de pull.** Depois de abrir um PR, Claude Code monitora os resultados das verificações de CI e pode corrigir automaticamente falhas ou mesclar o PR assim que todas as verificações passarem. Consulte [Monitor pull request status](/docs/pt/desktop#monitor-pull-request-status).

**Coloque Claude em um cronograma.** Configure [scheduled tasks](/docs/pt/desktop-scheduled-tasks) para executar Claude automaticamente em uma base recorrente: uma revisão de código diária todas as manhãs, uma auditoria de dependência semanal, ou um briefing que extrai de suas ferramentas conectadas.

**Escale quando estiver pronto.** Abra [parallel sessions](/docs/pt/desktop#work-in-parallel-with-sessions) na barra lateral para trabalhar em várias tarefas ao mesmo tempo, opcionalmente cada uma em seu próprio Git worktree, e abra o [tasks pane](/docs/pt/desktop#watch-background-tasks) para observar os subagentes e comandos em segundo plano que uma sessão está executando. Abra um [side chat](/docs/pt/desktop#ask-a-side-question-without-derailing-the-session) para fazer uma pergunta sem descarrilar a thread principal. Envie [long-running work to the cloud](/docs/pt/desktop#run-long-running-tasks-in-the-cloud) para que continue mesmo se você fechar o aplicativo, ou [continue a session on the web or in your IDE](/docs/pt/desktop#continue-in-another-surface) se uma tarefa levar mais tempo do que o esperado. [Connect external tools](/docs/pt/desktop#extend-claude-code) como GitHub, Slack e Linear para reunir seu fluxo de trabalho.

<h2 id="what’s-next">
  O que vem a seguir
</h2>

* [Use Claude Code Desktop](/docs/pt/desktop): modos de permissão, sessões paralelas, visualização de diff, conectores e configuração empresarial
* [Vindo da CLI?](/docs/pt/desktop#coming-from-the-cli): execute Desktop e a CLI no mesmo projeto, e compare recursos, equivalentes de sinalizadores e o que não está disponível no Desktop
* [Troubleshooting](/docs/pt/desktop#troubleshooting): soluções para erros comuns e problemas de configuração
* [Best practices](/docs/pt/best-practices): dicas para escrever prompts eficazes e aproveitar ao máximo o Claude Code
* [Common workflows](/docs/pt/common-workflows): tutoriais para depuração, refatoração, testes e muito mais
