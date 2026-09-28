> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plataformas e integrações

> Escolha onde executar Claude Code e o que conectar a ele. Compare a CLI, Desktop, VS Code, JetBrains, web, mobile e integrações como Chrome, Slack e CI/CD.

Claude Code executa o mesmo mecanismo subjacente em todos os lugares, mas cada superfície é ajustada para uma forma diferente de trabalhar. Esta página ajuda você a escolher a plataforma certa para seu fluxo de trabalho e conectar as ferramentas que você já usa.

<h2 id="where-to-run-claude-code">
  Onde executar Claude Code
</h2>

Escolha uma plataforma com base em como você gosta de trabalhar e onde seu projeto está localizado.

| Plataforma                        | Melhor para                                                                                                                   | O que você obtém                                                                                                                                                                             |
| :-------------------------------- | :---------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CLI](/docs/pt/quickstart)             | Fluxos de trabalho de terminal, scripts, servidores remotos                                                                   | Conjunto completo de recursos, [Agent SDK](/docs/pt/headless), [computer use](/docs/pt/computer-use) em macOS (Pro e Max), provedores de terceiros                                                     |
| [Desktop](/docs/pt/desktop)            | Revisão visual, sessões paralelas, configuração gerenciada                                                                    | Visualizador de diff, visualização de aplicativo, [computer use](/docs/pt/desktop#let-claude-use-your-computer) e [Dispatch](/docs/pt/desktop#sessions-from-dispatch) em Pro e Max                     |
| [VS Code](/docs/pt/vs-code)            | Trabalhar dentro do VS Code sem mudar para um terminal                                                                        | Diffs inline, terminal integrado, contexto de arquivo                                                                                                                                        |
| [JetBrains](/docs/pt/jetbrains)        | Trabalhar dentro do IntelliJ, PyCharm, WebStorm ou outros IDEs JetBrains                                                      | Visualizador de diff, compartilhamento de seleção, sessão de terminal                                                                                                                        |
| [Web](/docs/pt/claude-code-on-the-web) | Tarefas de longa duração que não precisam de muito direcionamento, ou trabalho que deve continuar quando você estiver offline | Nuvem, gerenciada pela Anthropic por padrão; continua após você se desconectar                                                                                                               |
| [Mobile](/docs/pt/mobile)              | Iniciar e monitorar tarefas enquanto estiver longe de seu computador                                                          | Sessões em nuvem do aplicativo Claude para iOS e Android, [Remote Control](/docs/pt/remote-control) para sessões locais, [Dispatch](/docs/pt/desktop#sessions-from-dispatch) para Desktop em Pro e Max |

A CLI é a superfície mais completa para trabalho nativo de terminal: scripts e o Agent SDK são apenas CLI. Provedores de terceiros também funcionam em [VS Code](/docs/pt/vs-code#use-third-party-providers) e em [JetBrains](/docs/pt/feature-availability#features-available-on-every-provider), que executa a CLI no terminal do seu IDE. Implantações [Desktop](/docs/pt/desktop) corporativas suportam Google Cloud's Agent Platform, e Desktop suporta [provedores de gateway](/docs/pt/llm-gateway-connect#desktop-app); para Amazon Bedrock ou Microsoft Foundry, use a CLI ou uma extensão IDE, ou [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview), que executa a aba Code nesses provedores. Desktop e as extensões IDE trocam alguns recursos apenas CLI por revisão visual e integração mais estreita do editor. A web é executada na nuvem, portanto as tarefas continuam após você se desconectar. Mobile é um cliente fino nessas mesmas sessões em nuvem ou em uma sessão local via Remote Control, e pode enviar tarefas para Desktop com Dispatch.

Você pode misturar superfícies no mesmo projeto. Configuração, memória do projeto e servidores MCP são compartilhados entre as superfícies locais.

<h2 id="connect-your-tools">
  Conecte suas ferramentas
</h2>

Integrações permitem que Claude trabalhe com serviços fora de sua base de código.

| Integração                                       | O que faz                                                                                                      | Use para                                                                                        |
| :----------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| [Chrome](/docs/pt/chrome)                             | Controla seu navegador com suas sessões conectadas                                                             | Testar aplicativos web, preencher formulários, automatizar sites sem uma API                    |
| [GitHub Actions](/docs/pt/github-actions)             | Executa Claude em seu pipeline CI                                                                              | Revisões automatizadas de PR, triagem de problemas, manutenção agendada                         |
| [GitLab CI/CD](/docs/pt/gitlab-ci-cd)                 | O mesmo que GitHub Actions para GitLab                                                                         | Automação orientada por CI no GitLab                                                            |
| [Code Review](/docs/pt/code-review)                   | Revisa cada PR automaticamente                                                                                 | Capturando bugs antes da revisão humana                                                         |
| [Slack](/docs/pt/slack)                               | Responde a menções `@Claude` em seus canais                                                                    | Transformando relatórios de bugs em pull requests do chat da equipe                             |
| [Claude Tag](https://claude.com/docs/claude-tag) | Executa `@Claude` como a identidade compartilhada de sua organização com acesso configurado pelo administrador | Acesso compartilhado da equipe em planos Team e Enterprise, em vez de sessões Slack por usuário |

Para integrações não listadas aqui, [servidores MCP](/docs/pt/mcp) e [conectores](/docs/pt/desktop#connect-external-tools) permitem que você conecte quase qualquer coisa: Linear, Notion, Google Drive ou suas próprias APIs internas.

<h2 id="work-when-you-are-away-from-your-terminal">
  Trabalhe quando você estiver longe de seu terminal
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

Se você não tem certeza por onde começar, [instale a CLI](/docs/pt/quickstart) e execute-a em um diretório de projeto. Se você preferir não usar um terminal, [Desktop](/docs/pt/desktop-quickstart) oferece o mesmo mecanismo com uma interface gráfica.

<h2 id="related-resources">
  Recursos relacionados
</h2>

<h3 id="platforms">
  Plataformas
</h3>

* [CLI quickstart](/docs/pt/quickstart): instale e execute seu primeiro comando no terminal
* [Desktop](/docs/pt/desktop): revisão visual de diff, sessões paralelas, computer use e Dispatch
* [VS Code](/docs/pt/vs-code): a extensão Claude Code dentro de seu editor
* [JetBrains](/docs/pt/jetbrains): a extensão para IntelliJ, PyCharm e outros IDEs JetBrains
* [Web](/docs/pt/claude-code-on-the-web): sessões em nuvem do seu navegador em claude.ai/code que continuam sendo executadas quando você se desconecta
* [Projects](/docs/pt/claude-projects): uma conversa onde Claude coordena muitas sessões em nuvem para um corpo de trabalho e relata de volta
* [Mobile](/docs/pt/mobile): o aplicativo Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) para iniciar e monitorar tarefas enquanto estiver longe de seu computador

<h3 id="integrations">
  Integrações
</h3>

* [Chrome](/docs/pt/chrome): automatize tarefas do navegador com suas sessões conectadas
* [Computer use](/docs/pt/computer-use): deixe Claude abrir aplicativos e controlar sua tela em macOS
* [GitHub Actions](/docs/pt/github-actions): execute Claude em seu pipeline CI
* [GitLab CI/CD](/docs/pt/gitlab-ci-cd): o mesmo para GitLab
* [Code Review](/docs/pt/code-review): revisão automática em cada pull request
* [Slack](/docs/pt/slack): envie tarefas do chat da equipe, obtenha PRs de volta
* [Claude Tag](https://claude.com/docs/claude-tag): execute `@Claude` como a identidade compartilhada de sua organização em planos Team e Enterprise

<h3 id="remote-access">
  Acesso remoto
</h3>

* [Dispatch](/docs/pt/desktop#sessions-from-dispatch): envie uma mensagem com uma tarefa do seu telefone e ela pode gerar uma sessão Desktop
* [Remote Control](/docs/pt/remote-control): dirija uma sessão em execução do seu telefone ou navegador
* [Channels](/docs/pt/channels): envie eventos de aplicativos de chat ou seus próprios servidores para uma sessão
* [Scheduled tasks](/docs/pt/scheduled-tasks): execute prompts em um cronograma recorrente
