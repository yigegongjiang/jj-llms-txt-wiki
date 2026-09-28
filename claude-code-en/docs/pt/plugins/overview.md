> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Visão geral de plugins

> Entenda o que é um plugin Claude Code, quando você precisa de um em vez de uma skill ou servidor MCP independente, e qual página ler para instalar ou criar um.

Um plugin Claude Code é um diretório de skills, agentes, hooks, servidores MCP ou outros componentes que Claude Code instala e carrega como uma unidade. A maioria dos plugins vem de um marketplace, que é um catálogo que lista plugins e onde buscar cada um. Você também pode carregar um plugin de uma pasta que alguém lhe dá, ou [criar o seu próprio](/docs/pt/plugins/create).

<Note>
  Se você usa claude.ai chat ou Cowork e não Claude Code, veja [Plugins no claude.ai e no Cowork](https://claude.com/docs/plugins/overview).
</Note>

Para experimentar um plugin agora, execute `/plugin` em uma sessão de terminal Claude Code e instale um na aba **Discover**, que lista os plugins do marketplace oficial da Anthropic e qualquer marketplace que você tenha adicionado. De lá:

* [Instalar e gerenciar plugins](/docs/pt/plugins/install): as etapas completas de instalação, escopos e outras superfícies
* [Criar um plugin](/docs/pt/plugins/create): crie o seu próprio
* [Decida se você precisa de um plugin](#decide-whether-you-need-a-plugin): se um plugin é a ferramenta certa para o que você quer

<h2 id="understand-what-a-plugin-is">
  Entenda o que é um plugin
</h2>

Um plugin é um diretório de componentes, geralmente com um manifesto. O manifesto, um arquivo JSON em `.claude-plugin/plugin.json`, dá ao plugin seu nome e pode adicionar uma versão, uma descrição e outros [metadados](/docs/pt/plugins/manifest-reference). Os componentes são o que o plugin adiciona ao Claude Code, como:

* [**Skills**](/docs/pt/plugins/components#skills): instruções `SKILL.md` que Claude carrega quando relevante, e que você também pode executar como um comando
* [**Agents**](/docs/pt/plugins/components#agents): definições de subagentes que Claude pode delegar
* [**Hooks**](/docs/pt/plugins/components#hooks): comandos que Claude Code executa em pontos do seu ciclo de vida, como após cada edição
* [**MCP servers**](/docs/pt/plugins/components#mcp-servers): servidores de ferramentas aos quais Claude Code se conecta enquanto o plugin está ativado

Este diagrama mostra um plugin chamado `my-plugin` que contém um de cada um desses componentes, e o que você obtém de cada arquivo uma vez que o plugin é carregado.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Diagrama em duas colunas unidas por cinco setas retas. À esquerda, o diretório de um plugin chamado my-plugin, contendo um manifesto em .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json e outros componentes. À direita, o que cada arquivo oferece em sua sessão: o manifesto define o nome do plugin, my-plugin; a skill é executada como /my-plugin:review; o arquivo do agente é um subagente que Claude pode delegar; o arquivo de hooks contém hooks que são executados em eventos do ciclo de vida; e .mcp.json adiciona um servidor MCP que oferece ferramentas a Claude." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Diagrama em duas colunas unidas por cinco setas retas. À esquerda, o diretório de um plugin chamado my-plugin, contendo um manifesto em .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json e outros componentes. À direita, o que cada arquivo oferece em sua sessão: o manifesto define o nome do plugin, my-plugin; a skill é executada como /my-plugin:review; o arquivo do agente é um subagente que Claude pode delegar; o arquivo de hooks contém hooks que são executados em eventos do ciclo de vida; e .mcp.json adiciona um servidor MCP que oferece ferramentas a Claude." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

Para cada tipo de componente que um plugin pode conter, com um exemplo de cada, veja [Plugin components](/docs/pt/plugins/components). Para ver onde cada peça está localizada no diretório de um plugin, use o [plugin explorer](/docs/pt/plugins/components#explore-the-plugin-directory) nessa página.

<h3 id="decide-whether-you-need-a-plugin">
  Decida se você precisa de um plugin
</h3>

Skills, subagentes, hooks e servidores MCP funcionam por conta própria, sem um plugin. Uma skill que você salva em `~/.claude/skills/`, por exemplo, está disponível em cada projeto em sua máquina. Para configurar uma por conta própria, veja [Skills](/docs/pt/skills), [Subagents](/docs/pt/sub-agents), [Hooks](/docs/pt/hooks-guide) ou [MCP](/docs/pt/mcp).

Use um plugin quando você quer várias skills, subagentes, hooks ou servidores MCP empacotados como uma unidade. Instale um para obter uma configuração que alguém construiu, com um comando e atualizações de seu marketplace. Crie um para dar sua própria configuração aos colegas de trabalho, instalá-lo em muitos projetos ou publicar versões lançadas.

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  O que um plugin ativado adiciona às suas sessões
</h3>

Um plugin ativado faz parte de cada sessão, não apenas das sessões onde você o usa. Isso tem algumas consequências que vale a pena saber antes de instalar um:

* **Contexto e uso**: para cada skill, agente e comando que [Claude pode invocar por conta própria](/docs/pt/skills#control-who-invokes-a-skill), o nome e a descrição estão no contexto de Claude a cada turno para que Claude saiba que existe. Esses tokens contam para seu uso e deixam menos espaço na [janela de contexto](/docs/pt/context-window) mesmo em sessões onde nada do plugin é executado. O texto completo de uma skill ou agente é carregado apenas quando é usado. O que os servidores MCP do plugin adicionam por turno segue [MCP tool search](/docs/pt/mcp#scale-with-mcp-tool-search).
* **Processos**: servidores MCP que o plugin define são executados junto com cada sessão onde está ativado, e seus hooks disparam em seus eventos.
* **Permissões**: o que o plugin executa, ele executa como você. Veja [Plugin security and trust](/docs/pt/plugins/security) para o que revisar primeiro.

Você pode verificar a pegada de um plugin em cada estágio:

* **Antes de instalar**: abra o plugin na aba **Marketplaces** em `/plugin`. Plugins no marketplace oficial da Anthropic mostram uma estimativa de **Context cost** lá.
* **Depois de instalar**: [Measure what a plugin costs](/docs/pt/plugins/measure#measure-what-a-plugin-costs) mostra como ler a pegada de um plugin, e a aba **Installed** do grupo **Not used recently** lista plugins que você poderia desativar.
* **Para parar sem desinstalar**: desative o plugin com `/plugin` ou, em seu shell, `claude plugin disable`. Veja [Manage installed plugins](/docs/pt/plugins/install#manage-installed-plugins).

<h2 id="get-plugins-from-a-marketplace">
  Obtenha plugins de um marketplace
</h2>

Um marketplace é um repositório ou diretório com um arquivo `.claude-plugin/marketplace.json` que lista plugins e onde buscar cada um. É um catálogo, não uma loja hospedada. Você adiciona um marketplace uma vez, depois instala plugins dele por nome, como `commit-commands@claude-plugins-official`.

<Note>
  Um marketplace de plugin não é [Claude Marketplace](https://claude.com/marketplace). Claude Marketplace é o site em claude.com/marketplace onde você navega por plugins, conectores, produtos de parceiros e parceiros de serviço. Não é um marketplace que você adiciona com `/plugin marketplace add`.
</Note>

Claude Code adiciona o marketplace oficial da Anthropic na primeira vez que você inicia uma sessão de terminal interativa, a menos que uma [managed policy](/docs/pt/plugins/org#allow-the-official-marketplace-and-your-own) o bloqueie. Claude Code não adiciona nenhum outro marketplace por conta própria, incluindo os marketplaces comunitários e de demonstração da Anthropic. Para distinguir os três marketplaces da Anthropic, leia [Anthropic's marketplaces](/docs/pt/plugins/anthropic-marketplaces). Para ver o que o oficial lista, abra a aba **Discover** de `/plugin` em uma sessão ou navegue por [Claude Marketplace](https://claude.com/marketplace/plugins).

Este diagrama mostra o caminho de um marketplace para sua sessão. Um marketplace lista um plugin, você instala esse plugin, e Claude Code carrega seus componentes.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Diagrama do caminho do marketplace em três caixas, da esquerda para a direita. Um marketplace, um catálogo de plugins, lista um plugin. O plugin é um diretório instalado como uma unidade, contendo skills, agentes, hooks, servidores MCP e outros componentes. Você instala o plugin no Claude Code, que carrega seus componentes." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Diagrama do caminho do marketplace em três caixas, da esquerda para a direita. Um marketplace, um catálogo de plugins, lista um plugin. O plugin é um diretório instalado como uma unidade, contendo skills, agentes, hooks, servidores MCP e outros componentes. Você instala o plugin no Claude Code, que carrega seus componentes." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[Install and manage plugins](/docs/pt/plugins/install#install-a-plugin) tem as etapas de instalação para cada lugar onde você executa Claude Code. Enquanto você está desenvolvendo um plugin, você não precisa de um marketplace: carregue-o diretamente de sua pasta com `--plugin-dir`, como [Develop without a marketplace](/docs/pt/plugins/create#develop-without-a-marketplace) mostra.

<h3 id="make-an-installed-plugin-available-in-your-session">
  Disponibilize um plugin instalado em sua sessão
</h3>

Antes de um plugin que você instalou oferecer uma skill que você pode executar, ele tem que estar presente em cada uma dessas camadas:

* **Settings**: suas configurações listam os marketplaces que você adicionou e os plugins que estão ativados.
* **Disk**: `~/.claude/plugins/` contém o que Claude Code buscou e instalou.
* **Session**: plugins são carregados na inicialização, ou quando você [reload plugins](/docs/pt/plugins/loading#check-which-stage-a-plugin-reached).

Leia [Plugin loading reference](/docs/pt/plugins/loading) para as regras em cada camada, incluindo qual arquivo de configurações tem precedência e onde os arquivos estão no disco.

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Diferencie os marketplaces da Anthropic dos de terceiros
</h2>

O nome de um marketplace o coloca em um de três níveis. Claude Code aceita os nomes oficiais e comunitários apenas para marketplaces originários de repositórios `github.com/anthropics/`:

* **Official**: marketplaces com um dos [official marketplace names](/docs/pt/plugins/security#official-marketplace-names) da Anthropic, incluindo `claude-plugins-official` e o marketplace de demonstração `claude-code-plugins`.
* **Community**: marketplaces com um dos nomes comunitários da Anthropic, como `claude-community`. [Identify Anthropic's marketplaces by name](/docs/pt/plugins/security#marketplace-tiers) os lista.
* **Third-party**: todos os outros marketplaces. Um marketplace que seu colega de trabalho ou sua organização publica é de terceiros.

Qualquer que seja o nível, um plugin que você instala pode executar código com seus privilégios de usuário. Leia [Plugin security and trust](/docs/pt/plugins/security) para como revisar um plugin antes de instalá-lo.

Através de [managed settings](/docs/pt/settings#settings-files), uma organização pode colocar na lista de permissões ou bloquear marketplaces, forçar a instalação de plugins e desativar o carregamento apenas de sessão. Leia [Manage plugins for your organization](/docs/pt/plugins/org) para esses controles.

<h2 id="understand-install-scopes">
  Entenda os escopos de instalação
</h2>

Quando você instala um plugin, você escolhe um escopo, e o escopo decide para quem o plugin está ativado:

* **User scope**: ativado para você em cada projeto neste computador
* **Project scope**: ativado para todos que trabalham neste repositório, através do `.claude/settings.json` confirmado. Cada colaborador ainda [instala em sua própria máquina](/docs/pt/plugins/loading#enabled-in-project-settings-but-not-installed)
* **Local scope**: ativado para você apenas neste repositório

Um plugin que você instala no escopo do usuário no terminal, nas sessões locais do aplicativo desktop ou na extensão VS Code está disponível nos outros dois naquele computador, porque todos os três leem os mesmos arquivos de configurações. Veja [Choose an install scope](/docs/pt/plugins/install#choose-an-install-scope) para como escolher um.

Uma sessão em nuvem, incluindo uma no navegador em claude.ai/code, não carrega os plugins em suas configurações locais. Para etapas de instalação no terminal, VS Code e aplicativo desktop, e para o que uma sessão em nuvem carrega, veja [Install a plugin](/docs/pt/plugins/install#install-a-plugin).

<Note>
  O mesmo formato de plugin também instala no claude.ai e no Cowork, onde um conjunto diferente de componentes é carregado. Para essas superfícies, veja [Plugins on claude.ai and in Cowork](https://claude.com/docs/plugins/overview) em claude.com.
</Note>

<h2 id="next-steps">
  Próximas etapas
</h2>

A maioria das pessoas começa instalando um plugin do marketplace oficial da Anthropic, que Claude Code adiciona na primeira vez que você inicia uma sessão de terminal interativa. Execute `/plugin` em uma sessão de terminal para navegá-lo, ou siga [Install and manage plugins](/docs/pt/plugins/install), que também cobre o aplicativo desktop e VS Code. Para ver o que está naquele marketplace antes de abrir Claude Code, navegue por [Claude Marketplace](https://claude.com/marketplace/plugins) na web.

Para construir o seu próprio, [Create a plugin](/docs/pt/plugins/create) começa com um diretório vazio e termina com um plugin funcionando.

Uma vez que você tenha instalado ou construído um plugin, essas páginas cobrem o que vem a seguir:

* **Compartilhe o que você construiu**: [Publish and distribute a plugin](/docs/pt/plugins/publish)
* **Verifique se funciona e é usado**: [Test plugins with evals](/docs/pt/plugin-evals) e [Measure plugin cost and usage](/docs/pt/plugins/measure)
* **Execute um marketplace para sua equipe**: [Create a marketplace](/docs/pt/plugins/create-marketplace), depois [Host and maintain a marketplace](/docs/pt/plugins/host-marketplace)
* **Defina a política de plugin para uma organização**: [Manage plugins for your organization](/docs/pt/plugins/org)
* **Corrija um problema**: [Troubleshoot plugins](/docs/pt/plugins/troubleshooting)
