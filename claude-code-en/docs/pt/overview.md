> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Visão geral

> Claude Code é uma ferramenta de codificação agentic que lê sua base de código, edita arquivos, executa comandos e se integra com suas ferramentas de desenvolvimento. Disponível em seu terminal, IDE, aplicativo de desktop e navegador.

Claude Code é um assistente de codificação alimentado por IA que ajuda você a construir recursos, corrigir bugs e automatizar tarefas de desenvolvimento. Ele compreende toda a sua base de código e pode trabalhar em vários arquivos e ferramentas para realizar tarefas.

<h2 id="get-started">
  Comece agora
</h2>

Claude Code é executado em várias superfícies: o terminal, extensões de IDE, um aplicativo de desktop e a web. Escolha uma das abas abaixo para começar. A maioria das superfícies requer uma [assinatura Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_pricing) ou uma conta do [Anthropic Console](https://platform.claude.com/). O Terminal CLI, VS Code e JetBrains também suportam [provedores de terceiros](/docs/pt/third-party-integrations).

<Tabs>
  <Tab title="Terminal">
    O CLI completo para trabalhar com Claude Code diretamente em seu terminal. Edite arquivos, execute comandos e gerencie todo o seu projeto a partir da linha de comando.

    Para instalar Claude Code, use um dos seguintes métodos:

    <Tabs>
      <Tab title="Instalação Nativa (Recomendado)">
        **macOS, Linux, WSL:**

        ```bash theme={null}
        curl -fsSL https://claude.ai/install.sh | bash
        ```

        **Windows PowerShell:**

        ```powershell theme={null}
        irm https://claude.ai/install.ps1 | iex
        ```

        **Windows CMD:**

        ```batch theme={null}
        curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
        ```

        Se você vir `The token '&&' is not a valid statement separator`, você está no PowerShell, não no CMD. Se você vir `'irm' is not recognized as an internal or external command`, você está no CMD, não no PowerShell. Seu prompt mostra `PS C:\` quando você está no PowerShell e `C:\` sem o `PS` quando você está no CMD.

        Se o comando de instalação falhar com `syntax error near unexpected token '<'`, um `403`, ou outro erro de curl, consulte [Solucionar problemas de instalação](/docs/pt/troubleshoot-install#find-your-error) para corresponder o erro a uma correção e para métodos alternativos de instalação.

        [Git for Windows](https://git-scm.com/downloads/win) é recomendado no Windows nativo para que Claude Code possa usar a ferramenta Bash. Se Git for Windows não estiver instalado, Claude Code usa PowerShell como ferramenta de shell. Configurações WSL não precisam de Git for Windows.

        <Info>
          As instalações nativas são atualizadas automaticamente em segundo plano para mantê-lo na versão mais recente.
        </Info>
      </Tab>

      <Tab title="Homebrew">
        ```bash theme={null}
        brew install --cask claude-code
        ```

        Homebrew oferece dois casks. `claude-code` rastreia o canal de versão estável, que normalmente fica cerca de uma semana atrás e pula versões com regressões importantes. `claude-code@latest` rastreia o canal mais recente e recebe novas versões assim que são lançadas.

        <Info>
          As instalações do Homebrew não são atualizadas automaticamente. Execute `brew upgrade claude-code` ou `brew upgrade claude-code@latest`, dependendo de qual cask você instalou, para obter os recursos mais recentes e correções de segurança.
        </Info>
      </Tab>

      <Tab title="WinGet">
        ```powershell theme={null}
        winget install Anthropic.ClaudeCode
        ```

        <Info>
          As instalações do WinGet não são atualizadas automaticamente. Execute `winget upgrade Anthropic.ClaudeCode` periodicamente para obter os recursos mais recentes e correções de segurança.
        </Info>
      </Tab>
    </Tabs>

    Você também pode instalar com [apt, dnf, ou apk](/docs/pt/setup#install-with-linux-package-managers) no Debian, Fedora, RHEL e Alpine.

    Em seguida, inicie Claude Code em qualquer projeto. Substitua `your-project` pelo caminho para um diretório de projeto em sua máquina:

    ```bash theme={null}
    cd your-project
    claude
    ```

    Você será solicitado a fazer login no primeiro uso. Se você tiver definido a variável de ambiente `ANTHROPIC_API_KEY`, Claude Code ignora o prompt de login e pede que você aprove a chave. É isso! [Continue com o Quickstart →](/docs/pt/quickstart)

    <Tip>
      Veja [configuração avançada](/docs/pt/setup) para opções de instalação, atualizações manuais ou instruções de desinstalação. Visite [troubleshooting de instalação](/docs/pt/troubleshoot-install) se você encontrar problemas.
    </Tip>
  </Tab>

  <Tab title="VS Code">
    A extensão VS Code fornece diffs inline, @-mentions, revisão de plano e histórico de conversa diretamente em seu editor.

    * [Instalar para VS Code](vscode:extension/anthropic.claude-code)
    * [Instalar para Cursor](cursor:extension/anthropic.claude-code)

    Ou procure por "Claude Code" na visualização de Extensões (`Cmd+Shift+X` no Mac, `Ctrl+Shift+X` no Windows/Linux). Após instalar, abra a Paleta de Comandos (`Cmd+Shift+P` / `Ctrl+Shift+P`), digite "Claude Code" e selecione **Abrir em Nova Aba**.

    [Comece com VS Code →](/docs/pt/vs-code#get-started)
  </Tab>

  <Tab title="Aplicativo de desktop">
    Um aplicativo independente para executar Claude Code fora de seu IDE ou terminal. Revise diffs visualmente, execute várias sessões lado a lado, agende tarefas recorrentes e inicie sessões na nuvem.

    Baixe e instale:

    * [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) (Intel e Apple Silicon)
    * [Windows](https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs) (x64)
    * [Windows ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)
    * No Ubuntu ou Debian, onde o aplicativo está em beta, instale-o com apt seguindo as [instruções de instalação do Linux](/docs/pt/desktop-linux)

    Após instalar, inicie Claude, faça login e clique na aba **Code** para começar a codificar. O aplicativo inclui Claude Code, portanto você não precisa instalar o CLI separadamente. Uma [assinatura paga](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_desktop_pricing) é necessária.

    [Saiba mais sobre o aplicativo de desktop →](/docs/pt/desktop-quickstart)
  </Tab>

  <Tab title="Web">
    Execute Claude Code em seu navegador sem configuração local. Inicie tarefas de longa duração e volte quando estiverem prontas, trabalhe em repositórios que você não tem localmente ou execute várias tarefas em paralelo. Para um corpo maior de trabalho, crie um [projeto](/docs/pt/claude-projects) e deixe Claude coordenar as sessões paralelas para você. Disponível em navegadores de desktop e [no aplicativo Claude para iOS e Android](/docs/pt/mobile).

    Comece a codificar em [claude.ai/code](https://claude.ai/code).

    [Comece →](/docs/pt/web-quickstart)
  </Tab>

  <Tab title="JetBrains">
    Um plugin para IntelliJ IDEA, PyCharm, WebStorm e outras IDEs JetBrains com visualização de diff interativa e compartilhamento de contexto de seleção.

    Instale o [plugin Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) do JetBrains Marketplace e reinicie sua IDE. O plugin requer o Claude Code CLI, instalado separadamente; veja as [etapas de configuração do JetBrains](/docs/pt/jetbrains#installation).

    [Comece com JetBrains →](/docs/pt/jetbrains)
  </Tab>
</Tabs>

<h2 id="what-you-can-do">
  O que você pode fazer
</h2>

Aqui estão algumas das maneiras como você pode usar Claude Code:

<AccordionGroup>
  <Accordion title="Automatize o trabalho que você continua adiando" icon="wand-magic-sparkles">
    Claude Code lida com as tarefas tediosas que consomem seu dia: escrever testes para código não testado, corrigir erros de lint em um projeto, resolver conflitos de mesclagem, atualizar dependências e escrever notas de lançamento.

    ```bash theme={null}
    claude "write tests for the auth module, run them, and fix any failures"
    ```
  </Accordion>

  <Accordion title="Construa recursos e corrija bugs" icon="hammer">
    Descreva o que você quer em linguagem simples. Claude Code planeja a abordagem, escreve o código em vários arquivos e verifica se funciona.

    Para bugs, cole uma mensagem de erro ou descreva o sintoma. Claude Code rastreia o problema em sua base de código, identifica a causa raiz e implementa uma correção. Veja [fluxos de trabalho comuns](/docs/pt/common-workflows) para mais exemplos.
  </Accordion>

  <Accordion title="Crie commits e pull requests" icon="code-branch">
    Claude Code funciona diretamente com git. Ele prepara alterações, escreve mensagens de commit, cria branches e abre pull requests.

    ```bash theme={null}
    claude "commit my changes with a descriptive message"
    ```

    Em CI, você pode automatizar revisão de código e triagem de problemas com [GitHub Actions](/docs/pt/github-actions) ou [GitLab CI/CD](/docs/pt/gitlab-ci-cd).
  </Accordion>

  <Accordion title="Conecte suas ferramentas com MCP" icon="plug">
    O [Model Context Protocol (MCP)](/docs/pt/mcp) é um padrão aberto para conectar ferramentas de IA a fontes de dados externas. Com MCP, Claude Code pode ler seus documentos de design no Google Drive, atualizar tickets no Jira, extrair dados do Slack ou usar suas próprias ferramentas personalizadas. O [guia de início rápido do MCP](/docs/pt/mcp-quickstart) conecta seu primeiro servidor de ponta a ponta.
  </Accordion>

  <Accordion title="Personalize com instruções, skills e hooks" icon="sliders">
    [`CLAUDE.md`](/docs/pt/memory) é um arquivo markdown que você adiciona à raiz do seu projeto que Claude Code lê no início de cada sessão. Use-o para definir padrões de codificação, decisões de arquitetura, bibliotecas preferidas e listas de verificação de revisão. Se seu repositório já tiver um `AGENTS.md` para outros agentes de codificação, Claude Code [pode ler isso](/docs/pt/memory#agents-md) por conta própria ou junto com `CLAUDE.md`. Claude também constrói [memória automática](/docs/pt/memory#auto-memory) conforme funciona, salvando aprendizados em sessões sem você escrever nada.

    Crie [skills](/docs/pt/skills) para empacotar fluxos de trabalho repetíveis que sua equipe pode compartilhar, como `/review-pr` ou `/deploy-staging`.

    [Hooks](/docs/pt/hooks) permitem que você execute comandos shell antes ou depois de ações do Claude Code, como formatação automática após cada edição de arquivo ou execução de lint antes de um commit.
  </Accordion>

  <Accordion title="Execute agentes em paralelo e construa agentes personalizados" icon="users">
    Gere [múltiplos agentes Claude Code](/docs/pt/sub-agents) que trabalham em diferentes partes de uma tarefa simultaneamente. Um agente líder coordena o trabalho, atribui subtarefas e mescla resultados.

    Para executar várias sessões completas em paralelo e observá-las de uma tela, use [agentes em segundo plano](/docs/pt/agent-view). Para fluxos de trabalho totalmente personalizados, o [Agent SDK](/docs/pt/agent-sdk/overview) permite que você construa seus próprios agentes alimentados pelas ferramentas e capacidades do Claude Code, com controle total sobre orquestração, acesso a ferramentas e permissões.
  </Accordion>

  <Accordion title="Pipe, script e automatize com o CLI" icon="terminal">
    Claude Code é composável e segue a filosofia Unix. Pipe logs nele, execute-o em CI ou encadeie-o com outras ferramentas:

    ```bash theme={null}
    # Analise a saída de log recente
    tail -200 app.log | claude -p "Slack me if you see any anomalies"

    # Automatize traduções em CI
    claude -p "translate new strings into French and raise a PR for review"

    # Operações em massa em arquivos
    git diff main --name-only | claude -p "review these changed files for security issues"
    ```

    Veja a [referência CLI](/docs/pt/cli-reference) para o conjunto completo de comandos e flags.
  </Accordion>

  <Accordion title="Agende tarefas recorrentes" icon="clock">
    Execute Claude em um cronograma para automatizar trabalho que se repete: revisões de PR matinais, análise de falhas de CI durante a noite, auditorias de dependência semanais ou sincronização de documentos após PRs serem mesclados.

    * [Routines](/docs/pt/routines) são executadas na nuvem, portanto continuam funcionando mesmo quando seu computador está desligado. Elas também podem ser acionadas por chamadas de API ou eventos do GitHub. Crie-as a partir da web, do aplicativo Desktop ou executando `/schedule` no CLI.
    * [Tarefas agendadas do Desktop](/docs/pt/desktop-scheduled-tasks) são executadas em sua máquina, com acesso direto aos seus arquivos e ferramentas locais
    * [`/loop`](/docs/pt/scheduled-tasks) repete um prompt dentro de uma sessão CLI para polling rápido
  </Accordion>

  <Accordion title="Trabalhe de qualquer lugar" icon="globe">
    As sessões não estão vinculadas a uma única superfície. Mova o trabalho entre elas conforme seu contexto muda:

    * Afaste-se de sua mesa e continue trabalhando do seu telefone ou qualquer navegador com [Remote Control](/docs/pt/remote-control)
    * Envie uma mensagem para [Dispatch](/docs/pt/desktop#sessions-from-dispatch) com uma tarefa do seu telefone e abra a sessão Desktop que ela cria
    * Inicie uma tarefa de longa duração na [web](/docs/pt/claude-code-on-the-web) ou no [aplicativo móvel Claude](/docs/pt/mobile), depois puxe-a para seu terminal com `claude --teleport`. Teleport requer uma assinatura claude.ai.
    * Execute `/desktop` para continuar sua sessão de terminal atual no [aplicativo Desktop](/docs/pt/desktop), onde você pode revisar diffs visualmente. A entrega `/desktop` requer uma assinatura claude.ai. Disponível em macOS e Windows x64.
    * Rotear tarefas do chat da equipe: mencione `@Claude` no [Slack](/docs/pt/slack) com um relatório de bug e obtenha um pull request de volta
  </Accordion>
</AccordionGroup>

<h2 id="use-claude-code-everywhere">
  Use Claude Code em qualquer lugar
</h2>

Cada [superfície](/docs/pt/glossary#surface) se conecta ao mesmo mecanismo Claude Code subjacente, portanto seus arquivos CLAUDE.md, configurações e MCP servers funcionam em todos eles.

Além dos ambientes [Terminal](/docs/pt/quickstart), [VS Code](/docs/pt/vs-code), [JetBrains](/docs/pt/jetbrains), [Desktop](/docs/pt/desktop) e [Web](/docs/pt/claude-code-on-the-web) acima, Claude Code se integra com CI/CD, chat e fluxos de trabalho do navegador:

| O que eu quero fazer                                                                    | Melhor opção                                                                                                           |
| --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Continuar uma sessão local do meu telefone ou outro dispositivo                         | [Remote Control](/docs/pt/remote-control)                                                                                   |
| Enviar eventos do Telegram, Discord, iMessage ou meus próprios webhooks para uma sessão | [Channels](/docs/pt/channels)                                                                                               |
| Iniciar uma tarefa localmente, continuar no celular                                     | [`claude --cloud`](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud), depois o [aplicativo Claude mobile](/docs/pt/mobile) |
| Executar Claude em um cronograma recorrente                                             | [Routines](/docs/pt/routines) ou [Tarefas agendadas do Desktop](/docs/pt/desktop-scheduled-tasks)                                |
| Automatizar revisões de PR e triagem de problemas                                       | [GitHub Actions](/docs/pt/github-actions) ou [GitLab CI/CD](/docs/pt/gitlab-ci-cd)                                               |
| Obter revisão automática de código em cada PR                                           | [GitHub Code Review](/docs/pt/code-review)                                                                                  |
| Rotear relatórios de bugs do Slack para pull requests                                   | [Slack](/docs/pt/slack)                                                                                                     |
| Depurar aplicações web ao vivo                                                          | [Chrome](/docs/pt/chrome)                                                                                                   |
| Construir agentes personalizados para seus próprios fluxos de trabalho                  | [Agent SDK](/docs/pt/agent-sdk/overview)                                                                                    |

<h2 id="next-steps">
  Próximos passos
</h2>

Depois de instalar Claude Code, estes guias ajudam você a aprofundar.

* [Quickstart](/docs/pt/quickstart): caminhe através de sua primeira tarefa real, desde explorar uma base de código até fazer commit de uma correção
* [Armazene instruções e memórias](/docs/pt/memory): dê ao Claude instruções persistentes com arquivos CLAUDE.md e memória automática
* [Fluxos de trabalho comuns](/docs/pt/common-workflows) e [melhores práticas](/docs/pt/best-practices): padrões para aproveitar ao máximo Claude Code
* [Claude Academy](https://academy.claude.com/): cursos gratuitos no seu próprio ritmo, incluindo [Claude Code 101](https://academy.claude.com/courses/claude-code-101) e [Claude Code in Action](https://academy.claude.com/courses/claude-code-in-action)
* [Um harness para cada tarefa](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): como o time Claude Code usa [fluxos de trabalho dinâmicos](/docs/pt/workflows) para orquestrar subagentes em escala
* [Configurações](/docs/pt/settings): personalize Claude Code para seu fluxo de trabalho
* [Troubleshooting](/docs/pt/troubleshooting): soluções para problemas comuns
* [code.claude.com](https://code.claude.com/): demos, preços e detalhes do produto
