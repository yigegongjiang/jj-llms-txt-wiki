> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estender Claude Code

> Entenda quando usar CLAUDE.md, Skills, subagents, hooks, MCP e plugins.

Claude Code combina um modelo que raciocina sobre seu código com [ferramentas integradas](/docs/pt/how-claude-code-works#tools) para operações de arquivo, busca, execução e acesso à web. As ferramentas integradas cobrem a maioria das tarefas de codificação. Este guia cobre a camada de extensão: recursos que você adiciona para personalizar o que Claude sabe, conectá-lo a serviços externos e automatizar fluxos de trabalho.

<Note>
  Para saber como o loop agentic principal funciona, consulte [Como Claude Code funciona](/docs/pt/how-claude-code-works).
</Note>

**Novo no Claude Code?** Comece com [CLAUDE.md](/docs/pt/memory) para convenções de projeto, depois adicione outras extensões [conforme gatilhos específicos surgirem](#build-your-setup-over-time).

<h2 id="overview">
  Visão geral
</h2>

As extensões se conectam a diferentes partes do loop agentic:

* **[CLAUDE.md](/docs/pt/memory)** adiciona contexto persistente que Claude vê a cada sessão
* **[Output styles](/docs/pt/output-styles)** definem o papel, tom e formato de resposta de Claude para cada resposta em uma sessão
* **[Skills](/docs/pt/skills)** adicionam conhecimento reutilizável e fluxos de trabalho invocáveis
* **[Code intelligence](/docs/pt/tools-reference#lsp-tool-behavior)** conecta Claude a um language server para navegação em nível de símbolo e erros de tipo em tempo real
* **[MCP](/docs/pt/mcp)** conecta Claude a serviços e ferramentas externas
* **[Subagents](/docs/pt/sub-agents)** executam seus próprios loops em contexto isolado, retornando resumos
* **[Dynamic workflows](/docs/pt/workflows)** executam muitos subagents a partir de um script que Claude escreve, retornando um resultado
* **[Cross-session messaging](/docs/pt/cross-session-messaging)** permite que Claude passe uma mensagem de uma de suas sessões para outra
* **[Hooks](/docs/pt/hooks-guide)** executam seu script, solicitação HTTP, chamada de ferramenta MCP, prompt ou subagent quando Claude Code atinge um evento de ciclo de vida
* **[Plugins](/docs/pt/plugins/overview)** e **[marketplaces](/docs/pt/plugins/overview)** empacotam e distribuem esses recursos

[Skills](/docs/pt/skills) são a extensão mais flexível. Uma skill é um arquivo markdown contendo conhecimento, fluxos de trabalho ou instruções. Você pode invocar skills com um comando como `/deploy`, ou Claude pode carregá-las automaticamente quando relevante. Skills podem ser executadas em sua conversa atual ou em contexto isolado via subagents.

<h2 id="match-features-to-your-goal">
  Corresponder recursos ao seu objetivo
</h2>

Os recursos variam de contexto sempre ativo que Claude vê a cada sessão, a capacidades sob demanda que você ou Claude podem invocar, a automação em segundo plano que é executada em eventos específicos. A tabela abaixo mostra o que está disponível e quando cada um faz sentido.

| Recurso                                                        | O que faz                                                                                     | Quando usar                                                                                                                            | Exemplo                                                                                                                            |
| -------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| **CLAUDE.md**                                                  | Contexto persistente carregado a cada conversa                                                | Convenções de projeto, regras "sempre faça X"                                                                                          | "Use pnpm, não npm. Execute testes antes de fazer commit."                                                                         |
| **[Output style](/docs/pt/output-styles)**                          | Instruções que definem o papel, tom e formato de resposta de Claude para uma sessão inteira   | Uma voz, comprimento ou formato que você quer em cada resposta, ou Claude trabalhando como algo diferente de um engenheiro de software | O estilo Concise integrado para respostas mais curtas; um estilo personalizado que responde cada pergunta com um diagrama primeiro |
| **Skill**                                                      | Instruções, conhecimento e fluxos de trabalho que Claude pode usar                            | Conteúdo reutilizável, documentos de referência, tarefas repetíveis                                                                    | `/deploy` executa sua lista de verificação de implantação; skill de documentação de API com padrões de endpoint                    |
| **Subagent**                                                   | Contexto de execução isolado que retorna resultados resumidos                                 | Isolamento de contexto, tarefas paralelas, trabalhadores especializados                                                                | Tarefa de pesquisa que lê muitos arquivos mas retorna apenas descobertas principais                                                |
| **[Dynamic workflow](/docs/pt/workflows)**                          | Script que Claude escreve que executa muitos subagents em segundo plano                       | Trabalho que cresce além de um punhado de subagents, ou descobertas que você quer verificadas                                          | Auditar uma base de código inteira, com um segundo conjunto de agentes verificando cada descoberta                                 |
| **[Cross-session messaging](/docs/pt/cross-session-messaging)**     | Claude entrega uma mensagem de uma de suas sessões para outra                                 | Sessões que você executa por conta própria que precisam das descobertas uma da outra no meio da tarefa                                 | Uma sessão avisa outra que uma mudança que fez quebra o que a outra está construindo                                               |
| **[Code intelligence](/docs/pt/tools-reference#lsp-tool-behavior)** | Navegação e diagnósticos do language server                                                   | Linguagens tipadas, grandes bases de código onde grep é lento ou impreciso                                                             | Ir para a definição de um símbolo em vez de ler o arquivo inteiro                                                                  |
| **MCP**                                                        | Conectar a serviços externos                                                                  | Dados ou ações externas                                                                                                                | Consultar seu banco de dados, postar no Slack, controlar um navegador                                                              |
| **Hook**                                                       | Script, solicitação HTTP, chamada de ferramenta MCP, prompt ou subagent disparado por eventos | Automação que deve ser executada em cada evento correspondente                                                                         | Executar ESLint após cada edição de arquivo                                                                                        |
| **[Artifact](/docs/pt/artifacts)**                                  | Publicar saída de sessão como uma página web privada e interativa                             | Saída que você quer ver ou compartilhar visualmente em vez de como texto de terminal                                                   | Uma linha do tempo de incidente que se atualiza conforme Claude investiga                                                          |

**[Plugins](/docs/pt/plugins/overview)** são a camada de empacotamento. Um plugin agrupa skills, hooks, subagents e servidores MCP em uma única unidade instalável. Skills de plugin são nomeadas (como `/my-plugin:review`) para que múltiplos plugins possam coexistir. Use plugins quando quiser reutilizar a mesma configuração em múltiplos repositórios ou distribuir para outros via um **[marketplace](/docs/pt/plugins/overview)**.

<h3 id="build-your-setup-over-time">
  Construir sua configuração ao longo do tempo
</h3>

Você não precisa configurar tudo antecipadamente. Cada recurso tem um gatilho reconhecível, e a maioria das equipes os adiciona aproximadamente nesta ordem:

| Gatilho                                                                                         | Adicionar                                                                                  |
| :---------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------- |
| Claude erra uma convenção ou comando duas vezes                                                 | Adicione a [CLAUDE.md](/docs/pt/memory)                                                         |
| Você continua pedindo a Claude para ser mais breve, explicar mais ou responder no mesmo formato | Defina um [output style](/docs/pt/output-styles)                                                |
| Você continua digitando o mesmo prompt para iniciar uma tarefa                                  | Salve como uma [skill](/docs/pt/skills) invocável pelo usuário                                  |
| Você cola o mesmo playbook ou procedimento de múltiplas etapas no chat pela terceira vez        | Capture como uma [skill](/docs/pt/skills)                                                       |
| Você continua copiando dados de uma aba do navegador que Claude não consegue ver                | Conecte esse sistema como um [servidor MCP](/docs/pt/mcp)                                       |
| Claude lê muitos arquivos para encontrar onde um símbolo é definido ou usado                    | Instale um [plugin de code intelligence](/docs/pt/plugins/code-intelligence) para sua linguagem |
| Uma tarefa secundária inunda sua conversa com saída que você não consultará novamente           | Encaminhe através de um [subagent](/docs/pt/sub-agents)                                         |
| Você quer que algo aconteça toda vez sem pedir                                                  | Escreva um [hook](/docs/pt/hooks-guide)                                                         |
| Um segundo repositório precisa da mesma configuração                                            | Empacote como um [plugin](/docs/pt/plugins/overview)                                            |

Os mesmos gatilhos dizem quando atualizar o que você já tem. Um erro repetido ou um comentário de revisão recorrente é uma edição de CLAUDE.md, não uma correção única no chat. Um fluxo de trabalho que você continua ajustando manualmente é uma skill que precisa de outra revisão.

<h3 id="compare-similar-features">
  Comparar recursos similares
</h3>

Alguns recursos podem parecer similares. Para uma análise mais profunda sobre como escolher entre eles, consulte [Steering Claude Code: when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more) no blog. Aqui está como diferenciá-los.

<Tabs>
  <Tab title="Skill vs Subagent">
    Skills e subagents resolvem problemas diferentes:

    * **Skills** são conteúdo reutilizável que você pode carregar em qualquer contexto
    * **Subagents** são trabalhadores isolados que são executados separadamente de sua conversa principal

    | Aspecto                                                 | Skill                                                        | Subagent                                                                           |
    | ------------------------------------------------------- | ------------------------------------------------------------ | ---------------------------------------------------------------------------------- |
    | **O que é**                                             | Instruções, conhecimento ou fluxos de trabalho reutilizáveis | Trabalhador isolado com seu próprio contexto                                       |
    | **Benefício principal**                                 | Compartilhar conteúdo entre contextos                        | Isolamento de contexto. O trabalho acontece separadamente, apenas o resumo retorna |
    | **Impacto da [janela de contexto](/docs/pt/context-window)** | Adiciona à sua janela principal                              | Usa uma janela separada com seus próprios tokens de entrada e saída                |
    | **Melhor para**                                         | Material de referência, fluxos de trabalho invocáveis        | Tarefas que leem muitos arquivos, trabalho paralelo, trabalhadores especializados  |

    **Skills podem ser referência ou ação.** Skills de referência fornecem conhecimento que Claude usa ao longo de sua sessão (como seu guia de estilo de API). Skills de ação dizem a Claude para fazer algo específico (como `/deploy` que executa seu fluxo de trabalho de implantação).

    **Use um subagent** quando você precisar de isolamento de contexto ou quando sua janela de contexto estiver ficando cheia. O subagent pode ler dezenas de arquivos ou executar buscas extensas, mas sua conversa principal recebe apenas um resumo. Como o trabalho do subagent não consome seu contexto principal, isso também é útil quando você não precisa que o trabalho intermediário permaneça visível. Subagents personalizados podem ter suas próprias instruções e podem pré-carregar skills.

    **Eles podem se combinar.** Um subagent pode pré-carregar skills específicas (campo `skills:`). Uma skill pode ser executada em contexto isolado usando `context: fork`. Consulte [Skills](/docs/pt/skills) para detalhes.
  </Tab>

  <Tab title="CLAUDE.md vs Skill">
    Ambos armazenam instruções, mas carregam de forma diferente e servem a propósitos diferentes.

    | Aspecto                              | CLAUDE.md                      | Skill                                                 |
    | ------------------------------------ | ------------------------------ | ----------------------------------------------------- |
    | **Carrega**                          | A cada sessão, automaticamente | Sob demanda                                           |
    | **Pode incluir arquivos**            | Sim, com importações `@path`   | Sim, com importações `@path`                          |
    | **Pode disparar fluxos de trabalho** | Não                            | Sim, com `/<name>`                                    |
    | **Melhor para**                      | Regras "sempre faça X"         | Material de referência, fluxos de trabalho invocáveis |

    **Coloque em CLAUDE.md** se Claude sempre deve saber: convenções de codificação, comandos de compilação, estrutura do projeto, regras "nunca faça X".

    **Coloque em uma skill** se for material de referência que Claude precisa às vezes (documentação de API, guias de estilo) ou um fluxo de trabalho que você dispara com `/<name>` (deploy, review, release).

    **Regra prática:** Mantenha CLAUDE.md com menos de 200 linhas. Se estiver crescendo, mova conteúdo de referência para skills ou divida em arquivos [`.claude/rules/`](/docs/pt/memory#organize-rules-with-claude/rules/).
  </Tab>

  <Tab title="CLAUDE.md vs Output style">
    Ambos dão a Claude instruções permanentes. CLAUDE.md carrega o que Claude deve saber, e um output style define como Claude responde.

    | Aspecto         | CLAUDE.md                                                 | Output style                                                                                      |
    | --------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
    | **Contém**      | Fatos e regras sobre seu projeto                          | Um papel, tom e formato de resposta                                                               |
    | **Alternância** | Sempre carregado                                          | Um ativo por vez; [alterne estilos](/docs/pt/output-styles#change-your-output-style) sempre que quiser |
    | **Melhor para** | Comandos de compilação, convenções, regras "nunca faça X" | Respostas mais curtas, explicações junto com código, um papel não-engenharia                      |

    **Coloque em CLAUDE.md** se for verdade do projeto em qualquer estilo que você esteja: convenções de codificação, comandos de compilação, estrutura do projeto.

    **Use um output style** se for sobre a resposta em si e você pode querer desativá-lo novamente: comprimento, formato, quanto Claude explica, ou um papel diferente como um assistente de escrita. Claude Code inclui [estilos integrados](/docs/pt/output-styles#built-in-output-styles), e você pode escrever os seus próprios.

    **Eles se combinam.** CLAUDE.md permanece carregado qualquer estilo que você escolha. Claude segue ambos como instruções, então nenhum é imposto. Para qualquer coisa que deve acontecer toda vez, use um [hook](/docs/pt/hooks-guide).
  </Tab>

  <Tab title="CLAUDE.md vs Rules vs Skills">
    Todos os três armazenam instruções, mas carregam de forma diferente:

    | Aspecto         | CLAUDE.md                                      | `.claude/rules/`                                              | Skill                                                 |
    | --------------- | ---------------------------------------------- | ------------------------------------------------------------- | ----------------------------------------------------- |
    | **Carrega**     | A cada sessão                                  | A cada sessão, ou quando arquivos correspondentes são abertos | Sob demanda, quando invocado ou relevante             |
    | **Escopo**      | Projeto inteiro                                | Pode ser limitado a caminhos de arquivo                       | Específico da tarefa                                  |
    | **Melhor para** | Convenções principais e comandos de compilação | Diretrizes específicas de linguagem ou diretório              | Material de referência, fluxos de trabalho repetíveis |

    **Use CLAUDE.md** para instruções que cada sessão precisa: comandos de compilação, convenções de teste, arquitetura do projeto.

    **Use rules** para manter CLAUDE.md focado. Rules com [frontmatter `paths`](/docs/pt/memory#path-specific-rules) carregam apenas quando Claude trabalha com arquivos correspondentes, economizando contexto.

    **Use skills** para conteúdo que Claude só precisa às vezes, como documentação de API ou uma lista de verificação de implantação que você dispara com `/<name>`.
  </Tab>

  <Tab title="Subagent vs Dynamic workflow">
    Ambos fazem trabalho fora de sua conversa principal. Com subagents, Claude decide a cada turno o que é executado a seguir. Em um workflow, o script decide:

    * **Subagents** são trabalhadores que Claude gera, cada um retornando um resumo para a conversa que o gerou
    * **[Dynamic workflows](/docs/pt/workflows)** são scripts que Claude escreve que executam muitos subagents em segundo plano e retornam um resultado

    **Use um subagent** quando você precisar de um trabalhador rápido e focado: pesquisar uma pergunta, verificar uma afirmação, revisar um arquivo. O subagent faz o trabalho e retorna um resumo, então sua conversa principal fica limpa. Subagents que Claude nomeou quando os gerou também podem [se mensagear um ao outro](/docs/pt/sub-agents#what-loads-at-startup).

    **Use um dynamic workflow** quando um trabalho [cresce além de um punhado de subagents](/docs/pt/workflows#when-to-use-a-workflow), ou quando você quer que as descobertas sejam verificadas antes de você vê-las, como uma auditoria em toda a base de código, uma grande migração ou um plano elaborado de vários ângulos. Para iniciar um, [peça um workflow em seu prompt](/docs/pt/workflows#ask-for-a-workflow-in-your-prompt).

    **Para passar uma descoberta de uma de suas sessões para outra**, peça ao Claude da primeira sessão para enviá-la. Claude a entrega com [cross-session messaging](/docs/pt/cross-session-messaging). [Executar agentes em paralelo](/docs/pt/agents) compara as outras formas de executar mais de um Claude de uma vez, incluindo sessões que você entrega e verifica depois.
  </Tab>

  <Tab title="MCP vs Skill">
    MCP conecta Claude a serviços externos. Skills estendem o que Claude sabe, incluindo como usar esses serviços efetivamente.

    | Aspecto      | MCP                                                                  | Skill                                                                                              |
    | ------------ | -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
    | **O que é**  | Protocolo para conectar a serviços externos                          | Conhecimento, fluxos de trabalho e material de referência                                          |
    | **Fornece**  | Ferramentas e acesso a dados                                         | Conhecimento, fluxos de trabalho, material de referência                                           |
    | **Exemplos** | Integração Slack, consultas de banco de dados, controle de navegador | Lista de verificação de revisão de código, fluxo de trabalho de implantação, guia de estilo de API |

    Esses resolvem problemas diferentes e funcionam bem juntos:

    **MCP** dá a Claude ferramentas específicas para um sistema externo, com a conexão e autenticação tratadas pelo servidor.

    **Skills** dão a Claude conhecimento sobre como usar essas ferramentas efetivamente, além de fluxos de trabalho que você pode disparar com `/<name>`. Uma skill pode incluir o esquema do banco de dados da sua equipe e padrões de consulta, ou um fluxo de trabalho `/post-to-slack` com as regras de formatação de mensagem da sua equipe.
  </Tab>

  <Tab title="Hook vs Skill">
    Claude Code executa um hook em um evento de ciclo de vida; carrega uma skill em contexto para Claude aplicar.

    | Aspecto               | Hook                                                                                   | Skill                                                                                              |
    | --------------------- | -------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
    | **Executa**           | Um comando shell, solicitação HTTP, chamada de ferramenta MCP, prompt LLM ou subagent  | Instruções que Claude lê e segue                                                                   |
    | **Disparado por**     | [Eventos de ciclo de vida](/docs/pt/hooks#hook-events) como `PostToolUse` ou `SessionStart` | Você digitando `/<name>`, ou Claude correspondendo a descrição à sua tarefa                        |
    | **Determinismo**      | Sempre dispara em seu evento; o gatilho é garantido                                    | Claude interpreta as instruções; o resultado pode variar                                           |
    | **Custo de contexto** | Zero a menos que o hook retorne saída                                                  | Descrição carrega a cada sessão; conteúdo completo carrega quando usado                            |
    | **Melhor para**       | Linting após edições, bloqueio de comandos inseguros, logging, notificações            | Fluxos de trabalho que precisam de raciocínio, material de referência, tarefas de múltiplas etapas |

    **Use um hook** quando a ação deve acontecer da mesma forma toda vez e não precisa Claude pensar. Por exemplo: formatar ao salvar, rejeitar `rm -rf /`, postar uma mensagem Slack quando uma sessão termina.

    **Use uma skill** quando Claude deve decidir como aplicar os passos, ou quando o conteúdo é conhecimento em vez de um script. Por exemplo: uma lista de verificação `/release`, seu guia de estilo de API, um playbook de depuração.

    **Coloque guardrails em hooks.** Uma instrução como "nunca edite `.env`" em CLAUDE.md ou uma skill é um pedido, não uma garantia. Um hook `PreToolUse` que bloqueia a edição é execução. Se uma regra deve valer toda vez, faça um hook em vez de uma instrução de prompt.

    **Saída de hook entra em contexto.** Um hook `PostToolUse` que executa seu linter alimenta resultados de volta como texto que Claude lê; uma skill `/fix-lint` diz a Claude como resolvê-los.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  Entender como os recursos se sobrepõem
</h3>

Os recursos podem ser definidos em múltiplos níveis: em toda a máquina, por projeto, via plugins ou através de políticas gerenciadas. Você também pode aninhar arquivos CLAUDE.md em subdiretórios ou colocar skills em pacotes específicos de um monorepo. Quando o mesmo recurso existe em múltiplos níveis, aqui está como eles se sobrepõem:

* **Arquivos CLAUDE.md** são aditivos: todos os níveis contribuem conteúdo ao contexto de Claude simultaneamente. Arquivos do seu diretório de trabalho e acima carregam no lançamento; subdiretórios carregam conforme você trabalha neles. Quando as instruções entram em conflito, Claude usa julgamento para reconciliá-las. Consulte [como arquivos CLAUDE.md carregam](/docs/pt/memory#how-claude-md-files-load).
* **Skills e subagents** substituem por nome: quando o mesmo nome existe em múltiplos níveis, uma definição vence com base na prioridade (gerenciado > usuário > projeto para skills; gerenciado > sinalizador CLI > projeto > usuário > plugin para subagents). Skills de plugin são [nomeadas](/docs/pt/plugins/components#skills) para evitar conflitos. Consulte [descoberta de skill](/docs/pt/skills#resolve-skills-that-share-a-name) e [escopo de subagent](/docs/pt/sub-agents#choose-the-subagent-scope).
* **Servidores MCP** substituem por nome: local > projeto > usuário. Consulte [escopo MCP](/docs/pt/mcp#scope-hierarchy-and-precedence).
* **Hooks** se mesclam: todos os hooks registrados disparam para seus eventos correspondentes independentemente da fonte. Consulte [hooks](/docs/pt/hooks-guide).

<h3 id="combine-features">
  Combinar recursos
</h3>

Cada extensão resolve um problema diferente: CLAUDE.md lida com contexto sempre ativo, skills lidam com conhecimento sob demanda e fluxos de trabalho, MCP lida com conexões externas, subagents lidam com isolamento e hooks lidam com automação. Configurações reais combinam eles com base em seu fluxo de trabalho.

Por exemplo, você pode usar CLAUDE.md para convenções de projeto, uma skill para seu fluxo de trabalho de implantação, MCP para conectar ao seu banco de dados e um hook para executar linting após cada edição. Cada recurso lida com o que é melhor.

| Padrão                 | Como funciona                                                                                     | Exemplo                                                                                             |
| ---------------------- | ------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| **Skill + MCP**        | MCP fornece a conexão; uma skill ensina a Claude como usá-la bem                                  | MCP conecta ao seu banco de dados, uma skill documenta seu esquema e padrões de consulta            |
| **Skill + Subagent**   | Uma skill gera subagents para trabalho paralelo                                                   | Skill `/audit` inicia subagents de segurança, desempenho e estilo que trabalham em contexto isolado |
| **CLAUDE.md + Skills** | CLAUDE.md contém regras sempre ativas; skills contêm material de referência carregado sob demanda | CLAUDE.md diz "siga nossas convenções de API," uma skill contém o guia de estilo de API completo    |
| **Hook + MCP**         | Um hook dispara ações externas através de MCP                                                     | Hook pós-edição envia uma notificação Slack quando Claude modifica arquivos críticos                |

<h2 id="understand-context-costs">
  Entender custos de contexto
</h2>

Cada recurso que você adiciona consome algum contexto de Claude. Muito pode preencher sua janela de contexto, mas também pode adicionar ruído que torna Claude menos eficaz; skills podem não disparar corretamente, ou Claude pode perder o controle de suas convenções. Entender esses trade-offs ajuda você a construir uma configuração eficaz. Para uma visualização interativa de como esses recursos se combinam em uma sessão em execução, consulte [Explorar a janela de contexto](/docs/pt/context-window).

<h3 id="context-cost-by-feature">
  Custo de contexto por recurso
</h3>

Cada recurso tem uma estratégia de carregamento e custo de contexto diferentes:

| Recurso               | Quando carrega                                            | O que carrega                                                                                                            | Custo de contexto                                 |
| --------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------- |
| **CLAUDE.md**         | Início da sessão                                          | Conteúdo completo                                                                                                        | A cada requisição                                 |
| **Estilos de saída**  | Início da sessão, e novamente quando você muda de estilos | As instruções completas do estilo ativo; nada para o estilo Padrão                                                       | A cada requisição                                 |
| **Skills**            | Início da sessão + quando usado                           | Descrições no início, conteúdo completo quando usado                                                                     | Baixo (descrições a cada requisição)\*            |
| **Servidores MCP**    | Início da sessão                                          | Nomes de ferramentas; esquemas completos sob demanda                                                                     | Baixo até uma ferramenta ser usada                |
| **Code intelligence** | Após edições de arquivo e sob demanda                     | Diagnósticos após edições; localizações de símbolos sob consulta                                                         | Baixo; reduz leituras de arquivo em outro lugar   |
| **Subagents**         | Quando gerado                                             | Contexto fresco com skills especificadas, ou a conversa pai para um [fork](/docs/pt/sub-agents#fork-the-current-conversation) | Isolado da sessão principal                       |
| **Hooks**             | No disparo                                                | Nada (executa externamente)                                                                                              | Zero, a menos que hook retorne contexto adicional |

\*Por padrão, descrições de skill carregam no início da sessão para que Claude possa decidir quando usá-las. Defina `disable-model-invocation: true` no frontmatter de uma skill para ocultá-la de Claude inteiramente até que você a invoque manualmente. Para uma skill que você não escreveu, defina [`skillOverrides`](/docs/pt/skills#override-skill-visibility-from-settings) em configurações para fazer o mesmo sem editar seu arquivo.

<h3 id="understand-how-features-load">
  Entender como os recursos carregam
</h3>

Cada recurso carrega em diferentes pontos em sua sessão. As abas abaixo explicam quando cada um carrega e o que entra em contexto.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" className="dark:hidden" alt="Carregamento de contexto: CLAUDE.md carrega no início da sessão e permanece em cada requisição. Nomes de ferramentas MCP carregam no início com esquemas completos adiados até o uso. Skills carregam descrições no início, conteúdo completo na invocação. Subagents obtêm contexto isolado. Hooks são executados externamente." width="720" height="382" data-path="images/context-loading.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/context-loading-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=b274089ef9612d9c760bca9838557626" className="hidden dark:block" alt="Carregamento de contexto: CLAUDE.md carrega no início da sessão e permanece em cada requisição. Nomes de ferramentas MCP carregam no início com esquemas completos adiados até o uso. Skills carregam descrições no início, conteúdo completo na invocação. Subagents obtêm contexto isolado. Hooks são executados externamente." width="720" height="382" data-path="images/context-loading-dark.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **Quando:** Início da sessão

    **O que carrega:** Conteúdo completo de todos os arquivos CLAUDE.md (níveis gerenciado, usuário e projeto).

    **Herança:** Claude lê arquivos CLAUDE.md do seu diretório de trabalho até a raiz e descobre aninhados em subdiretórios conforme acessa esses arquivos. Consulte [Como arquivos CLAUDE.md carregam](/docs/pt/memory#how-claude-md-files-load) para detalhes.

    <Tip>Mantenha CLAUDE.md com menos de 200 linhas. Mova material de referência para skills, que carregam sob demanda. Para obter [propostas de corte para um CLAUDE.md verificado](/docs/pt/memory#my-claude-md-is-too-large), execute `/doctor`.</Tip>
  </Tab>

  <Tab title="Skills">
    Skills são capacidades extras no kit de ferramentas de Claude. Podem ser material de referência (como um guia de estilo de API) ou fluxos de trabalho invocáveis que você dispara com `/<name>` (como `/deploy`). Claude Code inclui [skills agrupadas](/docs/pt/commands) como `/code-review`, `/batch` e `/debug` que funcionam imediatamente. Você também pode criar as suas próprias.

    **Quando:** Depende da configuração da skill. Por padrão, descrições carregam no início da sessão e conteúdo completo carrega quando usado. Para skills apenas de usuário (`disable-model-invocation: true`), nada carrega até que você as invoque.

    **O que carrega:** Para skills invocáveis por modelo, Claude vê nomes e descrições em cada requisição. Quando você invoca uma skill com `/<name>` ou Claude a carrega automaticamente, o conteúdo completo carrega em sua conversa.

    **Como Claude escolhe skills:** Claude corresponde sua tarefa contra descrições de skill para decidir quais são relevantes. Se descrições forem vagas ou se sobrepuserem, Claude pode carregar a skill errada ou perder uma que ajudaria. Para dizer a Claude para usar uma skill específica, invoque-a com `/<name>`. Skills com `disable-model-invocation: true` são invisíveis a Claude até que você as invoque.

    **Custo de contexto:** Baixo até ser usado. Skills apenas de usuário têm custo zero até invocação.

    **Em subagents:** Skills funcionam diferentemente em subagents. Em vez de carregamento sob demanda, skills listadas no campo `skills` do subagent são totalmente pré-carregadas em seu contexto no lançamento. Subagents ainda podem descobrir e invocar skills de projeto, usuário e plugin não listadas através da ferramenta Skill.

    <Tip>Use `disable-model-invocation: true` para skills com efeitos colaterais. Isso economiza contexto e garante que apenas você as dispare.</Tip>
  </Tab>

  <Tab title="Servidores MCP">
    **Quando:** Início da sessão.

    **O que carrega:** Nomes de ferramentas e instruções de servidor de servidores conectados. Esquemas JSON completos permanecem adiados até Claude precisar de uma ferramenta específica.

    **Custo de contexto:** [Busca de ferramentas](/docs/pt/mcp#scale-with-mcp-tool-search) está ativada por padrão, então ferramentas MCP ociosas consomem contexto mínimo.

    <Tip>Execute `/mcp` para ver status de conexão de cada servidor. Execute `/context all` para ver quantos tokens cada ferramenta MCP carregada usa. Claude Code [reconecta a servidores remotos automaticamente](/docs/pt/mcp#automatic-reconnection) se eles caírem, e você pode desconectar servidores que você não está usando ativamente.</Tip>
  </Tab>

  <Tab title="Code intelligence">
    **Quando:** Após edições de arquivo, e sob demanda quando Claude navega pelo código.

    **O que carrega:** Erros de tipo e avisos após cada edição de arquivo. Informações de definição, referência e tipo quando Claude procura um símbolo.

    **Custo de contexto:** Baixo. Consultas de símbolos frequentemente substituem leituras amplas de arquivo, então o uso de contexto líquido pode diminuir.

    <Tip>A ferramenta LSP fica inativa até que você instale um [plugin de code intelligence](/docs/pt/plugins/code-intelligence) para sua linguagem.</Tip>
  </Tab>

  <Tab title="Subagents">
    **Quando:** Sob demanda, quando você ou Claude gera um para uma tarefa.

    **O que carrega:** Contexto fresco e isolado contendo:

    * O prompt do sistema do agente, não o prompt do sistema de Claude Code
    * Conteúdo completo de skills listadas no campo `skills:` do agente
    * CLAUDE.md e status git, exceto os agentes Explore e Plan integrados [omitem ambos](/docs/pt/sub-agents#what-loads-at-startup), e um agente cuja definição define [`omitClaudeMd`](/docs/pt/sub-agents#supported-frontmatter-fields) pula os arquivos CLAUDE.md de usuário, projeto e local
    * Qualquer contexto que o agente principal passa no prompt

    Para um [fork](/docs/pt/sub-agents#fork-the-current-conversation), Claude Code carrega a conversa pai até agora, prompt do sistema e ferramentas em vez disso.

    **Custo de contexto:** Isolado da sessão principal.

    <Tip>Use subagents para trabalho que não precisa de seu contexto de conversa completo. Seu isolamento previne inchar sua sessão principal.</Tip>
  </Tab>

  <Tab title="Hooks">
    **Quando:** No disparo. Claude Code executa hooks em eventos de ciclo de vida específicos como execução de ferramenta, limites de sessão, envio de prompt, solicitações de permissão e compactação. Consulte [Hooks](/docs/pt/hooks) para a lista completa.

    **O que carrega:** Nada por padrão. Hooks são executados fora da conversa principal.

    **Custo de contexto:** Zero, a menos que o hook retorne saída que seja adicionada como mensagens à sua conversa.

    <Tip>Hooks são ideais para efeitos colaterais (linting, logging) que não precisam afetar o contexto de Claude.</Tip>
  </Tab>
</Tabs>

<h2 id="learn-more">
  Saiba mais
</h2>

Cada recurso tem seu próprio guia com instruções de configuração, exemplos e opções de configuração.

<CardGroup cols={2}>
  <Card title="CLAUDE.md" icon="file-lines" href="/docs/pt/memory">
    Armazenar contexto de projeto, convenções e instruções
  </Card>

  <Card title="Skills" icon="brain" href="/docs/pt/skills">
    Dar a Claude expertise de domínio e fluxos de trabalho reutilizáveis
  </Card>

  <Card title="Subagents" icon="users" href="/docs/pt/sub-agents">
    Descarregar trabalho para contexto isolado
  </Card>

  <Card title="Dynamic workflows" icon="network" href="/docs/pt/workflows">
    Executar muitos subagents a partir de um script
  </Card>

  <Card title="Cross-session messaging" icon="terminal" href="/docs/pt/cross-session-messaging">
    Permitir que Claude envie mensagens para suas outras sessões
  </Card>

  <Card title="MCP" icon="plug" href="/docs/pt/mcp">
    Conectar Claude a serviços externos
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/pt/hooks-guide">
    Automatizar ações com hooks
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/pt/plugins/overview">
    Empacotar e compartilhar conjuntos de recursos
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/pt/plugins/create-marketplace">
    Hospedar e distribuir coleções de plugins
  </Card>
</CardGroup>
