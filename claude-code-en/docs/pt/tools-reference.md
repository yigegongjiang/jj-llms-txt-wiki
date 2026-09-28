> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência de ferramentas

> Referência completa das ferramentas que Claude Code pode usar, incluindo requisitos de permissão e comportamento por ferramenta.

Claude Code tem acesso a um conjunto de ferramentas integradas que o ajudam a entender e modificar sua base de código. Os nomes das ferramentas são as strings exatas que você usa em [regras de permissão](/docs/pt/permissions#tool-specific-permission-rules), [listas de ferramentas de subagentes](/docs/pt/sub-agents) e [correspondências de hooks](/docs/pt/hooks).

Para controlar quais ferramentas Claude pode usar e quando ele pede primeiro, configure [regras de permissão](/docs/pt/permissions#tool-specific-permission-rules) em suas configurações, [hooks](/docs/pt/hooks) ou [lista de ferramentas de um subagente](/docs/pt/sub-agents#supported-frontmatter-fields). Veja [Configurar ferramentas com regras de permissão e hooks](#configure-tools-with-permission-rules-and-hooks) para cada local que aceita um nome de ferramenta.

Para adicionar ferramentas personalizadas, conecte um [servidor MCP](/docs/pt/mcp). Para estender Claude com fluxos de trabalho baseados em prompts reutilizáveis, escreva uma [skill](/docs/pt/skills), que é executada através da ferramenta `Skill` existente em vez de adicionar uma nova entrada de ferramenta.

<Info>
  Nos planos Pro, Max e Team, Claude Code inicia sessões em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), onde um classificador decide a maioria desses prompts em vez de você. A coluna `Permission required` mostra se a ferramenta solicita em [Modo Manual](/docs/pt/permission-modes) para caminhos dentro do diretório de trabalho. Ferramentas de acesso a arquivos marcadas como Não, incluindo `Read`, `Grep` e `Glob`, ainda solicitam para caminhos fora do [diretório de trabalho e diretórios adicionais](/docs/pt/permissions#working-directories). `Bash` é marcado como Sim, mas executa um conjunto integrado de [comandos somente leitura](/docs/pt/permissions#read-only-commands) sem solicitar.
</Info>

| Ferramenta             | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Permissão necessária |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `Agent`                | Cria um [subagente](/docs/pt/sub-agents) com sua própria janela de contexto para lidar com uma tarefa. Com [equipes de agentes](/docs/pt/agent-teams) ativadas, uma chamada que carrega um `name` pode iniciar um [colega de equipe](/docs/pt/agent-teams#how-claude-starts-agent-teams). Veja [Comportamento da ferramenta Agent](#agent-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Não                  |
| `Artifact`             | Publica um arquivo HTML ou Markdown como um [artefato](/docs/pt/artifacts): uma página privada e interativa no claude.ai. Você pode compartilhá-lo com um link público ou dentro de sua organização nos planos Team e Enterprise, onde o compartilhamento público requer que um Proprietário [o ative](/docs/pt/artifacts#control-public-sharing). Requer um plano Pro, Max, Team ou Enterprise e autenticação `/login`; veja [Disponibilidade](/docs/pt/artifacts#availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Sim                  |
| `AskUserQuestion`      | Faz perguntas de múltipla escolha para coletar requisitos ou esclarecer ambiguidades. As perguntas permanecem abertas até que você as responda por padrão. Veja [Comportamento da ferramenta AskUserQuestion](#askuserquestion-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Não                  |
| `Bash`                 | Executa comandos de shell em seu ambiente. Veja [Comportamento da ferramenta Bash](#bash-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Sim                  |
| `CronCreate`           | Agenda um prompt recorrente ou único dentro da sessão atual. As tarefas têm escopo de sessão e são restauradas em `--resume` ou `--continue` se não expiradas. Veja [tarefas agendadas](/docs/pt/scheduled-tasks)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Não                  |
| `CronDelete`           | Cancela uma tarefa agendada por ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Não                  |
| `CronList`             | Lista todas as tarefas agendadas na sessão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Não                  |
| `Edit`                 | Faz edições direcionadas em arquivos específicos. Veja [Comportamento da ferramenta Edit](#edit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Sim                  |
| `EndConversation`      | Encerra a sessão, em casos raros de entrada abusiva sustentada ou quando você pede a Claude para demonstrar a ferramenta. Requer Claude Code v2.1.213 ou posterior. Veja [Comportamento da ferramenta EndConversation](#endconversation-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Não                  |
| `EnterPlanMode`        | Muda para o modo de plano para projetar uma abordagem antes de codificar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Não                  |
| `EnterWorktree`        | Cria um [git worktree](/docs/pt/worktrees) isolado e muda para ele. Passe um `path` para mudar para um worktree existente em vez de criar um novo. Na primeira entrada, o alvo pode ser um worktree do repositório atual ou, em um espaço de trabalho multi-repositório, de um repositório aninhado dentro dele. Antes da v2.1.203, um worktree de repositório aninhado era rejeitado. Um `path` fora de `.claude/worktrees/` solicita sua aprovação antes de entrar, pois move o diretório de trabalho da sessão e o acesso de escrita para esse local. A criação de novo worktree e caminhos sob `.claude/worktrees/` não solicitam. Antes da v2.1.206, Claude entrava em caminhos fora de `.claude/worktrees/` sem solicitar. De dentro de uma sessão de worktree ou de um subagente com um diretório de trabalho fixado, como [`isolation: worktree`](/docs/pt/sub-agents#supported-frontmatter-fields), apenas a forma `path` está disponível e o alvo deve estar sob `.claude/worktrees/` do repositório da sessão                                                                   | Sim                  |
| `ExitPlanMode`         | Apresenta um plano para aprovação e sai do modo de plano                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Sim                  |
| `ExitWorktree`         | Sai de uma sessão de worktree e retorna ao diretório original. Não disponível para subagentes que já executam em seu próprio diretório de trabalho, como com [`isolation: worktree`](/docs/pt/sub-agents#supported-frontmatter-fields)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Não                  |
| `Glob`                 | Encontra arquivos com base em correspondência de padrões. Ausente por padrão no macOS, Linux e WSL. Veja [Comportamento da ferramenta Glob](#glob-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Não                  |
| `Grep`                 | Pesquisa padrões no conteúdo de arquivos. Ausente por padrão no macOS, Linux e WSL. Veja [Comportamento da ferramenta Grep](#grep-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Não                  |
| `ListAgents`           | Lista os agentes que Claude pode enviar mensagens com `SendMessage`: subagentes na sessão, [colegas de equipe](/docs/pt/agent-teams) de equipe de agentes, suas outras sessões locais de Claude Code e, enquanto esta sessão está conectada a [Controle Remoto](/docs/pt/remote-control), suas sessões de [Claude Code na web](/docs/pt/claude-code-on-the-web) e suas sessões de Controle Remoto em outras máquinas. Respalda o comando `/list-agents`. Veja [mensagens entre sessões](/docs/pt/cross-session-messaging). Requer Claude Code v2.1.224 ou posterior e aparece apenas em sessões onde [mensagens entre sessões estão ativadas](/docs/pt/cross-session-messaging#availability). Linhas de colegas de equipe e a primeira linha mostrando o próprio nome desta sessão requerem v2.1.239 ou posterior                                                                                                                                                                                                                                                                                         | Não                  |
| `ListMcpResourcesTool` | Lista recursos expostos por [servidores MCP](/docs/pt/mcp) conectados                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Não                  |
| `LSP`                  | Inteligência de código via servidores de linguagem: ir para definições, encontrar referências, relatar erros de tipo e avisos. Veja [Comportamento da ferramenta LSP](#lsp-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Não                  |
| `Monitor`              | Executa um comando em segundo plano e alimenta cada linha de saída de volta para Claude, para que ele possa reagir a entradas de log, mudanças de arquivo ou status consultado no meio da conversa. Também pode abrir um WebSocket e tratar cada mensagem recebida como um evento. Veja [Ferramenta Monitor](#monitor-tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Sim                  |
| `NotebookEdit`         | Modifica células de notebook Jupyter. Veja [Comportamento da ferramenta NotebookEdit](#notebookedit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Sim                  |
| `PowerShell`           | Executa comandos PowerShell nativamente. Veja [Ferramenta PowerShell](#powershell-tool) para disponibilidade                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Sim                  |
| `PushNotification`     | Envia uma notificação de desktop e um push de telefone quando [Controle Remoto](/docs/pt/remote-control) está conectado, para que uma tarefa de longa duração ou [tarefa agendada](/docs/pt/scheduled-tasks) possa alcançá-lo quando você se afastar. A entrega de push é executada através de infraestrutura hospedada pela Anthropic, que não é acessível do Amazon Bedrock, Claude Platform na AWS, Agent Platform do Google Cloud ou Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Não                  |
| `Read`                 | Lê o conteúdo de arquivos. Veja [Comportamento da ferramenta Read](#read-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Não                  |
| `ReadMcpResourceTool`  | Lê um recurso MCP específico por URI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Não                  |
| `RemoteTrigger`        | Cria, atualiza, executa e lista [Rotinas](/docs/pt/routines) no claude.ai. Respalda o comando `/schedule`. A [referência de entrada `RemoteTrigger`](/docs/pt/agent-sdk/typescript#remotetrigger) documenta cada ação e as políticas organizacionais que removem a ferramenta. Rotinas vivem no claude.ai e requerem um plano Pro, Max, Team ou Enterprise, então esta ferramenta não é acessível do Amazon Bedrock, Claude Platform na AWS, Agent Platform do Google Cloud ou Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Não                  |
| `ReportFindings`       | Relata descobertas de revisão de código como uma lista estruturada, com um arquivo, resumo e cenário de falha por descoberta, para que Claude Code possa renderizá-las em vez de imprimi-las como texto. Claude a chama quando instruções ativas de revisão de código dizem para fazê-lo. Requer Claude Code v2.1.196 ou posterior. A partir da v2.1.199, uma descoberta também pode carregar um slug `category` opcional, como `correctness` ou `test-coverage`, mostrado ao lado da localização do arquivo na lista renderizada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Não                  |
| `ScheduleWakeup`       | Reagenda a próxima iteração de um [`/loop` auto-paced](/docs/pt/scheduled-tasks#let-claude-choose-the-interval). Claude chama isso no final de cada iteração para escolher quando a próxima é executada, entre um minuto e uma hora; você não a chama diretamente. Para encerrar o loop em vez disso, Claude a chama com `stop: true`, que cancela o wakeup pendente. O campo `stop` requer Claude Code v2.1.202 ou posterior. O wakeup pendente aparece em `session_crons` em [Entrada do hook Stop](/docs/pt/hooks#stop-input)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Não                  |
| `SendFeedback`         | Redige um relatório de feedback sobre Claude Code, cobrindo um problema de produto ou o próprio comportamento de Claude na sessão, e o coloca na fila em sua máquina para você revisar. Claude Code não envia nada até que você escolha enviar o rascunho. Veja [Comportamento da ferramenta SendFeedback](#sendfeedback-tool-behavior). Requer Claude Code v2.1.238 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Não                  |
| `SendMessage`          | Envia uma mensagem para outro agente: um colega de equipe de [equipe de agentes](/docs/pt/agent-teams), um [subagente que ele retoma](/docs/pt/sub-agents#resume-subagents) por ID ou nome de agente, ou uma de suas outras sessões de Claude Code, nesta máquina ou além dela. Mensagens para outras sessões requerem Claude Code v2.1.224 ou posterior. [Mensagens entre sessões](/docs/pt/cross-session-messaging) cobrem quais sessões Claude pode alcançar, [como uma mensagem se parece quando chega](/docs/pt/cross-session-messaging#what-a-message-looks-like) e [como Claude recebe um aviso quando outra sessão fica inativa](/docs/pt/cross-session-messaging#get-a-notice-when-another-session-goes-idle). Claude pode incluir uma entrada `summary` opcional, normalmente 5-10 palavras, que Claude Code mostra como uma visualização de uma linha. Quando Claude a omite em uma [mensagem de texto simples](/docs/pt/cross-session-messaging#limitations), Claude Code usa a primeira linha da mensagem como resumo. Claude Code trunca um resumo mais longo que 200 caracteres com reticências | Não                  |
| `SendUserFile`         | Envia arquivos da sessão para você com uma legenda opcional, para que um relatório gerado, diagrama, captura de tela ou artefato construído chegue ao seu dispositivo em vez de apenas ser mencionado na transcrição. A partir da v2.1.196, a entrada `display` opcional controla a apresentação: `render` abre o arquivo inline no cliente, `attach` mostra apenas um cartão de download e quando não definido o cliente decide por tipo de arquivo. Disponível quando um cliente [Controle Remoto](/docs/pt/remote-control) está conectado ou em uma [sessão na nuvem](/docs/pt/claude-code-on-the-web). A entrega é executada através de infraestrutura hospedada pela Anthropic, então a ferramenta não está disponível no Amazon Bedrock, Agent Platform do Google Cloud ou Microsoft Foundry                                                                                                                                                                                                                                                                                         | Não                  |
| `ShareOnboardingGuide` | Carrega `ONBOARDING.md` e retorna um link de compartilhamento que colegas de equipe podem abrir no Claude Code. Chamado de `/team-onboarding` após o guia ser escrito. Disponível para assinantes do claude.ai nos planos Pro, Max, Team e Enterprise                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Sim                  |
| `Skill`                | Executa uma [skill](/docs/pt/skills#control-who-invokes-a-skill) dentro da conversa principal                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Sim                  |
| `SubagentHandback`     | Entrega o relatório final de um subagente para qualquer conversa que receba o resultado desse subagente. Fornecido apenas em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), para subagentes que a ferramenta Agent executa localmente, exceto [forks](/docs/pt/sub-agents#fork-the-current-conversation), e disponível no CLI do terminal, extensões IDE, sessões na nuvem e Agent SDK; o classificador revisa o relatório antes de ser entregue. Requer Claude Code v2.1.271 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Não                  |
| `TaskCreate`           | Cria uma nova tarefa na lista de tarefas. Fornecido por padrão apenas nos modelos listados em [Disponibilidade da ferramenta Task](#task-tool-availability) e em outros modelos quando você optar por participar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Não                  |
| `TaskGet`              | Recupera detalhes completos para uma tarefa específica. Fornecido por padrão apenas nos modelos listados em [Disponibilidade da ferramenta Task](#task-tool-availability) e em outros modelos quando você optar por participar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Não                  |
| `TaskList`             | Lista todas as tarefas com seu status atual. Fornecido por padrão apenas nos modelos listados em [Disponibilidade da ferramenta Task](#task-tool-availability) e em outros modelos quando você optar por participar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Não                  |
| `TaskOutput`           | Recupera saída de uma tarefa em segundo plano. Descontinuado em favor de `Read` no caminho do arquivo de saída da tarefa. Quando nenhuma tarefa corresponde ao ID, o erro lista os agentes de fundo em execução por ID e descrição. Antes da v2.1.203, o erro nomeava apenas o ID ausente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Não                  |
| `TaskStop`             | Para uma tarefa em segundo plano em execução por ID. Também aceita um colega de equipe de [equipe de agentes](/docs/pt/agent-teams) ou um agente de fundo nomeado por ID ou nome de agente. Antes da v2.1.198, aceitava apenas um ID de tarefa em segundo plano. Quando nenhuma tarefa corresponde ao ID, o erro lista os agentes de fundo em execução por ID e descrição, incluindo agentes que outro agente gerou. Antes da v2.1.203, o erro listava colegas de equipe em execução e agentes nomeados, mas não agentes de fundo que outro agente gerou, então eles não podiam ser identificados ou parados da conversa principal                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Não                  |
| `TaskUpdate`           | Atualiza status da tarefa, dependências, detalhes ou deleta tarefas. Fornecido por padrão apenas nos modelos listados em [Disponibilidade da ferramenta Task](#task-tool-availability) e em outros modelos quando você optar por participar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Não                  |
| `TodoWrite`            | Gerencia a lista de verificação de tarefas da sessão. Desativado por padrão em favor de `TaskCreate`, `TaskGet`, `TaskList` e `TaskUpdate`. Defina `CLAUDE_CODE_ENABLE_TASKS=0` para reativá-lo em [sessões que têm as ferramentas de rastreamento de tarefas](#task-tool-availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Não                  |
| `ToolSearch`           | Pesquisa e carrega ferramentas adiadas quando [pesquisa de ferramentas](/docs/pt/mcp#scale-with-mcp-tool-search) está ativada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Não                  |
| `WaitForMcpServers`    | Aguarda um ou mais [servidores MCP](/docs/pt/mcp) que ainda estão se conectando em segundo plano, para que uma solicitação possa usar suas ferramentas sem reiniciar a sessão. Claude a chama quando um servidor necessário ainda não está conectado. Aparece apenas quando [pesquisa de ferramentas](/docs/pt/mcp#scale-with-mcp-tool-search) está desativada, pois `ToolSearch` lida com a espera quando está ativada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Não                  |
| `WebFetch`             | Busca conteúdo de uma URL especificada. Veja [Comportamento da ferramenta WebFetch](#webfetch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Sim                  |
| `WebSearch`            | Realiza buscas na web. Veja [Comportamento da ferramenta WebSearch](#websearch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Sim                  |
| `Workflow`             | Executa um [fluxo de trabalho dinâmico](/docs/pt/workflows): um script que orquestra muitos subagentes em segundo plano e retorna um resultado consolidado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Sim                  |
| `Write`                | Cria ou sobrescreve arquivos. Veja [Comportamento da ferramenta Write](#write-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Sim                  |

<h2 id="configure-tools-with-permission-rules-and-hooks">
  Configure ferramentas com regras de permissão e hooks
</h2>

Na maioria dos casos, Claude decide quando usar essas ferramentas e você não precisa nomeá-las você mesmo ao interagir com Claude. Você referencia nomes de ferramentas diretamente ao definir permissões e outras configurações:

* em [`permissions.allow`](/docs/pt/settings-reference#permissions-allow) e [`permissions.deny`](/docs/pt/settings-reference#permissions-deny) em configurações, e na interface `/permissions`
* nos sinalizadores [CLI](/docs/pt/cli-reference) `--allowedTools` e `--disallowedTools`
* nas opções [`allowedTools` e `disallowedTools`](/docs/pt/agent-sdk/permissions#allow-and-deny-rules) do Agent SDK
* no frontmatter [`allowed-tools`](/docs/pt/skills#frontmatter-reference) de uma skill
* na condição [`if`](/docs/pt/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field) de um hook

Todos esses aceitam o mesmo formato de regra, `ToolName(specifier)`. O especificador depende da ferramenta, e várias ferramentas compartilham um formato:

| Formato de regra               | Aplica-se a               | Detalhes                                                                              |
| :----------------------------- | :------------------------ | :------------------------------------------------------------------------------------ |
| `Bash(npm run *)`              | Bash, Monitor             | [Correspondência de padrão de comando](/docs/pt/permissions#bash)                          |
| `PowerShell(Get-ChildItem *)`  | PowerShell                | [Correspondência de padrão de comando](/docs/pt/permissions#powershell)                    |
| `Read(~/secrets/**)`           | Read, Grep, Glob, LSP     | [Correspondência de padrão de caminho](/docs/pt/permissions#read-and-edit)                 |
| `Edit(/src/**)`                | Edit, Write, NotebookEdit | [Correspondência de padrão de caminho](/docs/pt/permissions#read-and-edit)                 |
| `Skill(deploy *)`              | Skill                     | [Correspondência de nome de skill](/docs/pt/skills#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)`               | Agent                     | [Correspondência de tipo de subagent](/docs/pt/permissions#agent-subagents)                |
| `WebFetch(domain:example.com)` | WebFetch                  | [Correspondência de domínio](/docs/pt/permissions#webfetch)                                |
| `WebSearch`                    | WebSearch                 | Sem especificador; permitir ou negar a ferramenta como um todo                        |

Ferramentas não listadas aqui, como `ExitPlanMode` ou `ShareOnboardingGuide`, aceitam apenas o nome da ferramenta sem especificador.

Uma regra de permissão `Edit(...)` também concede acesso de leitura ao mesmo caminho, portanto você não precisa de uma regra `Read(...)` correspondente. Uma regra de negação `Read(...)` também bloqueia as ferramentas Edit e Write no mesmo caminho, incluindo a criação de um novo arquivo lá, porque ambas as ferramentas alteram conteúdo que Claude precisa ser capaz de ler novamente. A verificação de negação `Read` requer Claude Code v2.1.208 ou posterior em edições, e v2.1.228 ou posterior em escritas.

Os campos `matcher` do Hook usam nomes de ferramentas simples, não o formato entre parênteses. Veja [padrões de matcher](/docs/pt/hooks#matcher-patterns) para as regras de correspondência. Para os nomes de campo que cada ferramenta passa para `tool_input` em hooks, veja a [referência de entrada PreToolUse](/docs/pt/hooks#pretooluse-input).

<h2 id="agent-tool-behavior">
  Comportamento da ferramenta Agent
</h2>

A ferramenta Agent cria um subagente em uma janela de contexto separada. O subagente trabalha através de sua tarefa autonomamente, depois retorna seu resultado para a conversa pai. O pai não vê as chamadas de ferramentas intermediárias ou saídas do subagente, apenas esse resultado final. Com [agent teams](/docs/pt/agent-teams) ativados, uma chamada que carrega um `name` pode iniciar um [teammate](/docs/pt/agent-teams#how-claude-starts-agent-teams), que relata através de mensagens de equipe em vez de retornar um resultado.

Para limitar quantas voltas um subagente executa, defina `maxTurns` na [definição do subagente](/docs/pt/sub-agents#supported-frontmatter-fields). Quando o subagente atinge o limite, Claude Code marca o resultado retornado como saída parcial, e Claude pode [retomar o subagente](/docs/pt/sub-agents#resume-subagents) para continuar.

A mesma ferramenta Agent também inicia [subagentes bifurcados](/docs/pt/sub-agents#fork-the-current-conversation) onde [fork mode](/docs/pt/sub-agents#turn-fork-mode-on-or-off) está ativado. Uma bifurcação herda a conversa pai completa em vez de começar do zero, executa em segundo plano além dos [casos que permanecem em primeiro plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background), e ainda exibe prompts de permissão em seu terminal. O resto desta seção descreve subagentes não-bifurcados.

Quais ferramentas um subagente não-bifurcado pode usar depende dos campos `tools` e `disallowedTools` na [definição do subagente](/docs/pt/sub-agents):

* **Nenhum campo definido**: o subagente herda todas as [ferramentas disponíveis para subagentes](/docs/pt/sub-agents#available-tools).
* **Apenas `tools`**: o subagente obtém apenas as ferramentas listadas.
* **Apenas `disallowedTools`**: o subagente obtém todas as ferramentas pai, exceto as listadas.
* **Ambos definidos**: `disallowedTools` tem precedência. Uma ferramenta listada em ambos é removida.

Em todos os casos, o conjunto resolvido é limitado às [ferramentas disponíveis para subagentes](/docs/pt/sub-agents#available-tools): uma ferramenta que não está disponível para subagentes nunca é concedida, mesmo quando listada em `tools`. Onde as condições na entrada da tabela de ferramentas `SubagentHandback` se mantêm, Claude Code também fornece ao subagente essa ferramenta, mesmo se você a deixar de fora de `tools` ou a listar em `disallowedTools`.

Se cada entrada na lista `tools` de um subagente falhar em corresponder a uma ferramenta utilizável, a ferramenta Agent geralmente retorna um erro nomeando as entradas em vez de iniciar o subagente; veja [Agent would be spawned with zero tools](/docs/pt/errors#agent-would-be-spawned-with-zero-tools) para a mensagem e como corrigir cada entrada.

Iniciar o subagente não solicita permissão por si só. Claude Code verifica as chamadas de ferramentas do próprio subagente contra suas regras de permissão conforme executa.

Onde você vê os prompts de permissão de um subagente depende se ele executa em primeiro plano ou em segundo plano. Claude Code executa subagentes em segundo plano por padrão, além dos [casos que executam em primeiro plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background).

* **Subagentes em primeiro plano** mostram os mesmos prompts de permissão que você veria na conversa principal, no momento em que cada chamada de ferramenta acontece.
* **Subagentes em segundo plano** exibem prompts de permissão em sua sessão principal a partir da v2.1.186. O prompt nomeia qual subagente está solicitando, e pressionar Esc nega essa chamada de ferramenta sem parar o subagente. Antes da v2.1.186, subagentes em segundo plano negavam automaticamente qualquer chamada de ferramenta que de outra forma solicitaria e continuavam sem essa ferramenta.

Para [limitar o que um subagente pode alcançar](/docs/pt/sub-agents#control-subagent-capabilities) em primeiro lugar, restrinja seu campo `tools`, por exemplo deixando Bash fora da lista, ou defina regras de negação em suas configurações.

<h2 id="askuserquestion-tool-behavior">
  Comportamento da ferramenta AskUserQuestion
</h2>

Claude usa `AskUserQuestion` para fazer perguntas de múltipla escolha quando precisa de uma decisão ou esclarecimento. Responda escolhendo uma opção ou digite seu próprio texto através da linha `Other` ou do campo de notas.

Quando você responde digitando seu próprio texto, Claude Code retransmite a resposta com redação neutra para que Claude siga o que você escreveu, incluindo um pedido para aguardar ou explicar primeiro.

<h3 id="question-auto-continue-timeout">
  Tempo limite de continuação automática de perguntas
</h3>

As perguntas permanecem abertas até que você as responda. Se você quiser que uma pergunta que deixar sem resposta eventualmente feche e permita que Claude continue sem você, defina a configuração [`askUserQuestionTimeout`](/docs/pt/settings-reference#askuserquestiontimeout) como `60s`, `5m` ou `10m`, seja no seu `settings.json` do usuário ou na linha **Question auto-continue timeout** em `/config`.

Depois que uma pergunta fica tanto tempo sem entrada, o diálogo fecha automaticamente: ele envia todas as opções que você já havia selecionado e diz a Claude que você pode estar longe do teclado, então Claude procede com seu próprio julgamento e pode fazer perguntas novamente mais tarde. Você vê uma contagem regressiva nos últimos 20 segundos. Pressione qualquer tecla para reiniciar o temporizador; em terminais que relatam foco, alternar para a janela também o reinicia.

O tempo limite se aplica apenas às perguntas de múltipla escolha do `AskUserQuestion`; prompts de permissão, incluindo aprovação de plano, nunca se resolvem automaticamente quando inativo.

<h2 id="bash-tool-behavior">
  Comportamento da ferramenta Bash
</h2>

A ferramenta Bash executa cada comando em um processo separado.

<h3 id="what-persists-between-commands">
  O que persiste entre comandos
</h3>

* Quando Claude executa `cd` na sessão principal, o novo diretório de trabalho é mantido para comandos Bash posteriores, desde que permaneça dentro do diretório do projeto ou de um [diretório de trabalho adicional](/docs/pt/permissions#working-directories) que você adicionou com `--add-dir`, `/add-dir`, ou `additionalDirectories` nas configurações. Isso inclui comandos que Claude executa em resposta às suas mensagens posteriores.
  * Sessões de subagentos nunca mantêm mudanças de diretório de trabalho.
  * Se `cd` sair desses diretórios, Claude Code redefine para o diretório do projeto e anexa `Shell cwd was reset to <dir>` ao resultado da ferramenta.
  * Para desabilitar esse carregamento de forma que cada comando Bash comece no diretório do projeto, defina `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1`.
* Variáveis de ambiente não persistem. Um `export` em um comando não estará disponível no próximo.
* Aliases e funções de shell definidas em seu arquivo de inicialização de shell estão disponíveis. No início da sessão, Claude Code carrega `~/.zshrc`, `~/.bashrc`, ou `~/.profile` dependendo do seu shell, captura os aliases, funções e opções de shell resultantes, e os aplica a cada comando Bash.

Ative seu virtualenv ou ambiente conda antes de iniciar Claude Code. Para fazer variáveis de ambiente persistirem entre comandos Bash, defina [`CLAUDE_ENV_FILE`](/docs/pt/env-vars) para um script de shell antes de iniciar Claude Code, ou use um [hook SessionStart](/docs/pt/hooks#persist-environment-variables) para preenchê-lo dinamicamente.

<h3 id="timeout-and-output-limits">
  Limites de timeout e saída
</h3>

Cada comando é executado sob um timeout, e Claude o gerencia: quando quer mais tempo do que o padrão para um comando, ele passa o parâmetro `timeout` com essa chamada — você nunca define um timeout por comando. Duas [variáveis de ambiente](/docs/pt/env-vars) limitam o que Claude obtém:

* `BASH_DEFAULT_TIMEOUT_MS` — o padrão quando Claude não passa timeout; dois minutos por padrão
* `BASH_MAX_TIMEOUT_MS` — com o padrão, define o limite máximo que limita o que Claude solicita: o limite efetivo é o maior dos dois, dez minutos por padrão

<h4 id="output-limits">
  Limites de saída
</h4>

Claude Code transmite a saída de um comando para um arquivo de trabalho enquanto o comando é executado; um comando cuja saída ultrapassa 5 GB é interrompido. Quando o comando termina, Claude Code lê a saída de volta desse arquivo, até a janela de releitura descrita abaixo. Quanto da saída chega a Claude inline depende se Claude Code trata o resultado como uma falha:

| Resultado | O que Claude obtém                                                                                                                                                                                                                                                      |
| :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Válido    | Inline até aproximadamente 30.000 caracteres por padrão; além disso, o caminho de um arquivo salvo no diretório da sessão e truncado após 64 MiB, mais uma visualização de até os primeiros 2.000 caracteres, e Claude lê ou pesquisa o arquivo quando precisa do resto |
| Falha     | Inline até aproximadamente 10.000 caracteres; além disso, um trecho de cabeça e cauda desse tamanho cortado da janela de releitura, sem caminho de arquivo                                                                                                              |

Um comando que sai com código 1 conta como um resultado válido para a ferramenta Bash apenas quando Claude Code reconhece o código de saída 1 como um resultado benigno para esse comando: `grep`, `rg`, `egrep`, `fgrep`, `find`, `diff`, `test`, e `[`, mais `git diff` e `git grep`. Todo outro comando que sai com código 1 conta como uma falha, mesmo quando o código de saída 1 é um resultado informacional benigno: sem correspondências para `pgrep` e `jq -e`, arquivos que diferem para `cmp`.

[`BASH_MAX_OUTPUT_LENGTH`](/docs/pt/env-vars) define quantos caracteres de saída Claude Code lê de volta do arquivo de trabalho para o resultado de um comando: 30.000 por padrão, até um limite máximo de 150.000. Aumente quando seus comandos rotineiramente ultrapassarem essa janela, como um build verboso ou um log de suite de testes completo. Aumentá-lo amplia a janela de releitura, que também é a janela de onde um trecho de comando com falha é cortado. Não aumenta os limites inline: um resultado válido acima do limite inline chega como um caminho de arquivo mais visualização independentemente dessa variável.

Para alterar quanto de um resultado válido Claude recebe inline, defina a configuração [`bashOutputMaxChars`](/docs/pt/settings-reference#bashoutputmaxchars) em vez disso, até 128.000 caracteres. Ela dimensiona o limite inline e a janela de releitura juntos, e Claude Code então ignora `BASH_MAX_OUTPUT_LENGTH`. Requer Claude Code v2.1.261 ou posterior.

<h3 id="background-commands">
  Comandos em background
</h3>

Para processos de longa duração, como servidores de desenvolvimento ou builds de observação, Claude pode definir `run_in_background: true` para iniciar o comando como uma tarefa em background e continuar trabalhando enquanto é executado. Liste e interrompa tarefas em background com `/tasks`. Depois que você interromper uma lá, ou de um cliente conectado como o aplicativo desktop, Claude continua em vez de esperar. Se um subagentos iniciou o comando, é esse subagentos que continua.

Um comando que um [subagentos em foreground](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) iniciou para quando esse subagentos dá sua resposta final. Um comando que a conversa principal ou um subagentos em background iniciou continua sendo executado após uma resposta final. Em modo não interativo com a flag `-p`, [comandos em background terminam logo após o resultado final da execução](/docs/pt/headless#background-tasks-at-exit).

Quando um comando atinge seu timeout sem terminar, Claude Code o move para o background em vez de interrompê-lo, a menos que o comando comece com `sleep`. Claude continua trabalhando enquanto o comando continua. Claude Code aplica as mesmas regras de tempo de vida a um comando movido quanto a qualquer outro comando em background, então ainda termina o comando de um subagentos em foreground na resposta final desse subagentos. Definir [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=1`](/docs/pt/env-vars#variables) desabilita o auto-backgrounding junto com o resto da funcionalidade de tarefas em background.

O resultado de um comando movido para o background declara o que aconteceu:

* Quando o timeout dispara a mudança, o resultado relata explicitamente: `Command did not complete within its 120s timeout and was moved to the background`, com os segundos correspondendo ao timeout que se aplicou, seguido pela ID da tarefa e o caminho do arquivo para o qual a saída está sendo escrita.
* Um `cd`, `pushd`, `popd`, ou `chdir` dentro de um comando que é movido para o background nunca é mantido: o resultado declara `Session cwd remains <dir>; directory changes made by the backgrounded command do not apply to subsequent commands.`, então Claude não age em uma mudança de diretório que não aconteceu.

<h3 id="memory-limit-on-linux-and-wsl">
  Limite de memória em Linux e WSL
</h3>

Em Linux e WSL, defina [`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](/docs/pt/env-vars#variables) para um tamanho como `4G` para limitar a memória que comandos Bash, PowerShell e ferramenta [Monitor](#monitor-tool) podem usar, para que um build descontrolado não consuma a memória que o resto da sessão precisa. Requer Claude Code v2.1.233 ou posterior. Antes de v2.1.246, comandos da ferramenta Monitor eram executados fora do limite.

* Escreva o tamanho como um número de bytes ou com um sufixo `K`, `M`, `G`, ou `T`. Defina `0`, `off`, `false`, `no`, ou `none` para desativar o limite. Claude Code ignora qualquer outro valor que não consiga ler como um tamanho, como `4e9`.
* Claude Code conta todos os comandos Bash, PowerShell e Monitor de uma sessão contra o único limite, não cada comando por si só.
* Claude Code aplica o limite com um cgroup de memória. Quando não consegue configurar o cgroup, comandos são executados sem um limite, e o log de debug de `claude --debug` diz por quê.
* Depois que o primeiro processo que Claude Code inicia ativou o limite, ou o desativou por causa de um valor off ou uma configuração de cgroup falhada, Claude Code mantém esse resultado até você relançar. Para aplicar um valor alterado ou removido, ou uma configuração corrigida, inicie `claude` novamente.
* Quando comandos não conseguem ficar sob o limite, o kernel interrompe um comando, e nada em seu resultado nomeia o limite.

Claude Code também pode contar outros tipos de processos que inicia contra o mesmo limite. Defina [`CLAUDE_CODE_TOOL_MEMORY_CGROUP_EXCLUDE`](/docs/pt/env-vars#variables) para uma lista separada por vírgulas dos tipos a isentar do limite; Claude Code aplica o limite a cada tipo não em sua lista. Defina para `none` para limitar cada tipo, ou para `all-new` para limitar apenas comandos Bash, PowerShell e ferramenta Monitor. Requer Claude Code v2.1.246 ou posterior. Os tipos que você pode nomear:

* `mcp`: [servidores MCP](/docs/pt/mcp) locais
* `lsp`: [servidores de linguagem](#lsp-tool-behavior)
* `hooks`: comandos [hook](/docs/pt/hooks)
* `plugin`: comandos que [plugins](/docs/pt/plugins/overview) executam
* `helper`: comandos auxiliares próprios de Claude Code, como `git`
* `agent`: processos Claude Code filhos, como [colegas de equipe agentes](/docs/pt/agent-teams)

Seja qual for sua lista, estas regras se aplicam:

* **Nomes desconhecidos**: Claude Code ignora nomes que não reconhece
* **Bash, PowerShell e Monitor**: Claude Code mantém comandos Bash, PowerShell e ferramenta Monitor sob o limite seja qual for sua lista
* **Variável não definida**: Claude Code pega o conjunto de outros tipos limitados da configuração que Anthropic entrega do servidor, e esse conjunto pode mudar ao longo do tempo, então defina a variável quando você precisa de um conjunto que não muda
* **Hooks com gate de permissão**: mesmo com cada tipo limitado, Claude Code exclui do limite um hook que pode bloquear ou alterar o resultado de uma ação, e qualquer servidor MCP que tal hook chama, então o kernel matando um hook com gate de permissão não pode permitir a ação que estava bloqueando

<h2 id="edit-tool-behavior">
  Comportamento da ferramenta Edit
</h2>

A ferramenta Edit realiza substituição exata de strings. Ela recebe uma `old_string` e uma `new_string` e substitui a primeira pela segunda. Ela não usa regex ou correspondência aproximada.

Três verificações devem passar para que uma edição seja aplicada. Antes de qualquer uma delas, um caminho correspondido por uma [regra de negação `Read`](/docs/pt/permissions#tool-specific-permission-rules) é recusado, incluindo a criação de um novo arquivo lá. A recusa requer Claude Code v2.1.208 ou posterior.

* **Read-before-edit**: Claude lê o arquivo na conversa atual antes de editá-lo, e uma leitura interrompida com um aviso [`PARTIAL view`](#read-tool-behavior) não conta. Claude Opus 4.6, Claude Haiku 4.5 e modelos mais antigos sempre exigem a leitura. Modelos mais novos podem editar um arquivo não lido quando a leitura não precisaria de um prompt de permissão e a ferramenta Read está disponível.
* **Match**: `old_string` deve aparecer no arquivo exatamente como escrito. Uma única diferença de caractere de espaço em branco ou indentação é suficiente para não corresponder.
* **Uniqueness**: `old_string` deve aparecer exatamente uma vez. Quando aparece mais de uma vez, Claude fornece uma string mais longa com contexto circundante suficiente para identificar uma ocorrência, ou define `replace_all: true` para substituir todas elas.

Um arquivo que mudou no disco depois que Claude o leu pela última vez ainda pode ser editado quando `old_string` corresponde ao conteúdo atual exatamente e sem ambiguidade e Claude Code pode ler o arquivo sem solicitar. Corresponder contra o conteúdo atual do arquivo mantém isso seguro, e o resultado observa que o arquivo contém outras alterações para que Claude o releia antes de edições que dependem do conteúdo circundante. Em qualquer outro caso, como uma `old_string` desatualizada ou uma que corresponde mais de uma vez sem `replace_all`, Claude lê o arquivo novamente antes de editar. O tratamento relaxado de arquivos não lidos e alterados requer Claude Code v2.1.208 ou posterior; antes disso, Claude Code recusava qualquer edição em um arquivo que não havia lido na conversa ou que mudou no disco após a leitura.

Visualizar um arquivo com Bash também satisfaz o requisito read-before-edit quando o comando é `cat`, `nl`, `bat`, `batcat`, `head`, `tail`, `sed -n 'X,Yp'`, `grep`, `egrep`, `fgrep`, ou `rg` em um único arquivo sem pipes ou redirecionamentos. Saída com pipe e outros comandos Bash não contam para a verificação read-before-edit.

Visualizar um arquivo com Bash afeta apenas a elegibilidade de edição, não as permissões. Consulte [Regras de permissão Read e Edit](/docs/pt/permissions#read-and-edit) para saber quais comandos Bash suas regras de negação `Read` e `Edit` cobrem.

<h2 id="endconversation-tool-behavior">
  Comportamento da ferramenta EndConversation
</h2>

A ferramenta EndConversation encerra a sessão atual. Claude a utiliza apenas em duas situações:

* como último recurso contra entrada abusiva sustentada, após tentativas de redirecionar a conversa terem falhado e após um aviso claro em uma mensagem anterior
* quando você solicita explicitamente ver a ferramenta demonstrada e confirma que deseja encerrar a sessão

Frustração geral, profanidade ou uma tarefa correndo mal não se qualificam, assim como solicitações de conteúdo prejudicial, que Claude recusa em vez de encerrar a sessão. Claude Code segue a mesma abordagem que claude.ai, que pode [encerrar um subconjunto raro de chats](https://www.anthropic.com/research/end-subset-conversations).

Depois que Claude encerra uma sessão interativa, a sessão é bloqueada. Novos prompts e a maioria dos comandos retornam `Claude ended this conversation. Start a new session (or /clear) to continue.`, e apenas `/clear`, `/resume`, `/help`, `/exit` e `/feedback` ainda funcionam. Claude Code registra o encerramento na transcrição da sessão, portanto retomar uma sessão encerrada restaura o bloqueio; o histórico da sessão não é excluído.

Retomar uma sessão encerrada em [modo não interativo](/docs/pt/headless) com a flag `-p` gera erro e sai com código 1, portanto um script não lê a execução encerrada como um sucesso.

A ferramenta nunca solicita permissão, e [hooks PreToolUse](/docs/pt/hooks#pretooluse) não são executados para ela. Enquanto qualquer outra ferramenta permanecer, você também não pode bloqueá-la: [regras de negação e solicitação](/docs/pt/permissions#tool-specific-permission-rules) nomeando `EndConversation` não têm efeito, e nem `--disallowedTools` nem uma lista `--tools` podem removê-la. A isenção é deliberada: a ferramenta não faz nada além de encerrar a conversa, nunca lendo ou modificando arquivos ou dados, e uma salvaguarda desse tipo só funciona se a sessão à qual se aplica não puder desativá-la. Quando suas regras de negação removem todas as outras ferramentas e também correspondem a `EndConversation`, como `"*"` faz, Claude Code a remove também em vez de deixá-la como a única ferramenta, a menos que uma regra de permissão nomeie `EndConversation` explicitamente. Uma lista de negação que remove todas as outras ferramentas sem corresponder a `EndConversation` a deixa em vigor.

[Subagentes](/docs/pt/sub-agents) nunca recebem a ferramenta. Tarefas em segundo plano que compartilham a lista de ferramentas da conversa principal a veem, mas chamá-la lá não encerra nada.

A ferramenta aparece apenas quando todos os itens a seguir se aplicam:

* **Versão**: Claude Code v2.1.213 ou posterior.
* **Modelo**: o modelo da sessão é Claude Opus 4.8, Claude Sonnet 5, Claude Fable 5 ou uma versão posterior de uma dessas famílias.
* **Superfície**: uma sessão de terminal interativa, incluindo uma sessão `claude` no terminal integrado de um IDE, que é como o [plugin JetBrains](/docs/pt/jetbrains) a executa. Outras superfícies não incluem a ferramenta, como:
  * execuções não interativas `-p`
  * sessões através dos pacotes TypeScript e Python do [Agent SDK](/docs/pt/agent-sdk/overview)
  * o painel da [extensão VS Code](/docs/pt/vs-code), que agrupa sua própria CLI
  * [GitHub Actions](/docs/pt/github-actions)
  * [Claude Code na web](/docs/pt/claude-code-on-the-web)
* **Modo de inicialização**: não uma sessão [`--bare`](/docs/pt/headless#start-faster-with-bare-mode). O modo bare carrega apenas ferramentas de shell e arquivo, portanto a ferramenta nunca é registrada lá.
* **Provedor**: não disponível em [Amazon Bedrock](/docs/pt/amazon-bedrock), [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) ou [Microsoft Foundry](/docs/pt/microsoft-foundry), ou em sessões conectadas através de um [cloud gateway](/docs/pt/claude-apps-gateway).

<h2 id="glob-tool-behavior">
  Comportamento da ferramenta Glob
</h2>

A ferramenta Glob encontra arquivos por padrão de nome. No Windows, ela faz parte do conjunto de ferramentas padrão. No macOS, Linux e WSL, Claude Code deixa Glob e [Grep](#grep-tool-behavior) fora do conjunto de ferramentas padrão, e Claude pesquisa com `find` e `grep` através da ferramenta Bash. No shell do Claude, esses dois comandos executam versões incorporadas de `bfs` e `ugrep`, e as pesquisas alcançam seus hooks e regras de permissão como chamadas `Bash`.

No macOS, Linux e WSL, você recupera as ferramentas Glob e Grep nestes casos:

* Você nomeia `Glob` ou `Grep` em [`--tools` ou `--allowedTools`](/docs/pt/cli-reference#cli-flags) quando inicia a sessão, ou nas [opções equivalentes do Agent SDK](/docs/pt/agent-sdk/overview). Com `--tools` você obtém os que lista, e nomear qualquer ferramenta em `--allowedTools` restaura ambas. Uma regra de permissão em um arquivo de configurações não tem esse efeito.
* Uma [regra de negação](/docs/pt/permissions#match-all-uses-of-a-tool) de permissões, o sinalizador `--disallowedTools`, ou [`--restricted`](/docs/pt/cli-reference#cli-flags) remove `Bash` da sessão.
* Um [subagente](/docs/pt/sub-agents#available-tools) lista `Glob` ou `Grep` em seu campo `tools` e deixa `Bash` de fora. As ferramentas listadas voltam apenas para esse subagente, ou para toda a sessão quando é executado como o agente de sessão principal através de [`--agent`](/docs/pt/sub-agents#invoke-subagents-explicitly) ou da configuração `agent`.

Glob suporta sintaxe glob padrão, incluindo `**` para correspondência recursiva de diretórios:

* `**/*.js` corresponde a todos os arquivos `.js` em qualquer profundidade
* `src/**/*.ts` corresponde a todos os arquivos `.ts` sob `src/`
* `*.{json,yaml}` corresponde a arquivos `.json` e `.yaml` no diretório atual

Os resultados são classificados por tempo de modificação e limitados a 100 arquivos. Se o limite for atingido, Claude vê um sinalizador de truncamento no resultado e pode estreitar o padrão.

Glob não respeita `.gitignore` por padrão, portanto encontra arquivos ignorados pelo git junto com os rastreados. Isso difere de [Grep](#grep-tool-behavior), que ignora arquivos gitignored. Para fazer Glob respeitar `.gitignore`, defina `CLAUDE_CODE_GLOB_NO_IGNORE=false` antes de iniciar Claude Code.

Claude Code decide a permissão para uma chamada Glob antes de verificar se o diretório de pesquisa existe. Ele ainda executa a verificação de permissão de leitura para um `path` ausente fora dos [diretórios de trabalho](/docs/pt/permissions#working-directories), portanto um prompt de permissão para um caminho não significa que o caminho existe.

Um valor `pattern` ou `path` que contém um byte nulo retorna um erro pedindo a Claude para removê-lo.&#x20;

<h2 id="grep-tool-behavior">
  Comportamento da ferramenta Grep
</h2>

A ferramenta Grep pesquisa padrões no conteúdo dos arquivos. Enquanto [Glob](#glob-tool-behavior) encontra arquivos por nome, Grep encontra linhas dentro deles. No macOS, Linux e WSL, Grep está ausente por padrão sob as mesmas condições que Glob. Veja [Comportamento da ferramenta Glob](#glob-tool-behavior) para quando ambas as ferramentas estão disponíveis.

Grep é construído em [ripgrep](https://github.com/BurntSushi/ripgrep) e usa a sintaxe regex do ripgrep, não grep POSIX. Padrões que incluem metacaracteres regex precisam ser escapados. Por exemplo, encontrar `interface{}` em código Go requer o padrão `interface\{\}`.

Um padrão, glob ou tipo de arquivo que ripgrep rejeita retorna um erro que inclui o diagnóstico do ripgrep, para que Claude possa corrigir a entrada e pesquisar novamente. Antes da v2.1.208, Claude Code relatava uma entrada rejeitada como `No files found` em vez de um erro, mesmo quando o texto pesquisado existia nos arquivos de destino.

Três modos de saída controlam o que é retornado:

* `files_with_matches`: apenas caminhos de arquivo, sem conteúdo de linha. Este é o padrão.
* `content`: linhas correspondentes com arquivo e número de linha. Quando o parâmetro `offset` da ferramenta aponta para além da última correspondência de um padrão que tem correspondências, Grep retorna `No entries at this offset`, para que Claude amplie ou redefina o offset em vez de concluir que o padrão não corresponde.
* `count`: contagem de correspondências por arquivo, seguida por um total em todos os arquivos correspondentes. O total cobre cada correspondência mesmo quando os parâmetros `head_limit` ou `offset` da ferramenta truncam as entradas listadas por arquivo. Antes da v2.1.208, o total apenas somava as entradas listadas.

Claude pode escopar resultados por arquivo com o parâmetro `glob`, como `**/*.tsx`, ou por linguagem com o parâmetro `type`, como `py` ou `rust`. Por padrão, padrões correspondem dentro de uma única linha. Claude pode definir `multiline: true` para corresponder através de limites de linha.

Grep respeita `.gitignore`, portanto arquivos ignorados pelo git são ignorados. Para pesquisar um arquivo ignorado pelo git, Claude passa seu caminho diretamente.

Claude Code decide permissão para uma chamada Grep antes de verificar se o `path` da pesquisa existe. Ele ainda executa a verificação de permissão de leitura para um `path` ausente fora dos [diretórios de trabalho](/docs/pt/permissions#working-directories), portanto um prompt de permissão para um caminho não significa que o caminho existe.

<h2 id="lsp-tool-behavior">
  Comportamento da ferramenta LSP
</h2>

A ferramenta LSP fornece ao Claude inteligência de código de um servidor de linguagem em execução. Após cada edição de arquivo, ela relata automaticamente erros de tipo e avisos para que Claude possa corrigir problemas sem uma etapa de compilação separada. Claude também pode chamá-la diretamente para navegar no código:

* Ir para a definição de um símbolo
* Encontrar todas as referências a um símbolo
* Obter informações de tipo em uma posição
* Listar símbolos em um arquivo
* Pesquisar um símbolo por nome em todo o workspace
* Encontrar implementações de uma interface
* Rastrear hierarquias de chamadas

Claude Code mantém a ferramenta inativa até que você instale um [plugin de inteligência de código](/docs/pt/plugins/code-intelligence) para sua linguagem. Em [sessões na nuvem](/docs/pt/claude-code-on-the-web), Claude Code não inicia servidores de linguagem de plugin, portanto a ferramenta LSP permanece inativa lá. Claude Code obtém a configuração do servidor de linguagem do plugin, e você instala o binário do servidor você mesmo.

Claude Code retorna um resultado de erro para cada chamada LSP em um arquivo cujo servidor de linguagem não consegue iniciar.

<h2 id="monitor-tool">
  Ferramenta Monitor
</h2>

A ferramenta Monitor permite que Claude observe algo em segundo plano e reaja quando muda, sem pausar a conversa. Peça a Claude para:

* Monitorar um arquivo de log e sinalizar erros conforme aparecem
* Pesquisar um PR ou trabalho de CI e relatar quando seu status muda
* Observar um diretório para mudanças de arquivo
* Rastrear saída de qualquer script de longa duração que você apontar
* Conectar a um feed WebSocket e relatar cada mensagem conforme chega

Para a maioria das observações, Claude escreve um pequeno script, o executa em segundo plano e recebe cada linha de saída conforme chega. Para um servidor que já envia eventos, Claude pode abrir uma [WebSocket](#websocket-source) em vez de executar um script.

Você continua trabalhando na mesma sessão e Claude intervém quando um evento chega.

Cada observação que Claude inicia tem um prazo: 5 minutos por padrão, no máximo 30 minutos, e no máximo 10 minutos em uma execução [não interativa](/docs/pt/headless) com um único prompt com `-p`.

No prazo, a observação termina. Claude recebe um aviso, para que possa iniciar a observação novamente se ainda for necessária.

Interrompa um monitor pedindo a Claude para cancelá-lo ou encerrando a sessão. Quando você interrompe um [subagente](/docs/pt/sub-agents) que iniciou monitors, por exemplo de `/tasks`, esses monitors param com ele.

Quando Monitor executa um comando, ele usa as mesmas [regras de permissão que Bash](/docs/pt/permissions#tool-specific-permission-rules), portanto os padrões `allow` e `deny` que você definiu para Bash também se aplicam aqui. Enquanto [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) está ativo, Claude Code reserva regras de permissão que nomeiam o próprio `Monitor`, junto com as outras [regras de permissão amplas que ele descarta](/docs/pt/permission-modes#how-the-classifier-evaluates-actions), portanto o classificador revisa comandos Monitor da mesma forma que revisa comandos Bash.

A [fonte WebSocket](#websocket-source) tem seu próprio prompt de aprovação, que o classificador também decide em modo automático.

A ferramenta não está disponível no Amazon Bedrock, na Agent Platform do Google Cloud ou no Microsoft Foundry. Também não está disponível quando `DISABLE_TELEMETRY` ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` está definido.

Plugins podem declarar monitors que iniciam automaticamente quando o plugin está ativo, em vez de pedir a Claude para iniciá-los. Veja [plugin monitors](/docs/pt/plugins/components#monitors).

<h3 id="websocket-source">
  Fonte WebSocket
</h3>

<Note>
  A fonte WebSocket requer Claude Code v2.1.195 ou posterior.
</Note>

Quando um servidor já envia eventos por WebSocket, Claude pode se conectar diretamente em vez de escrever um script de pesquisa. Cada tipo de atividade de socket se torna um evento ou encerra a observação:

* **Mensagens de texto**: cada uma se torna um evento, mesmo quando a mensagem abrange várias linhas.
* **Mensagens binárias**: não são passadas. Claude recebe uma linha de espaço reservado como `[binary frame, 512 bytes]`.
* **Mensagens maiores que 1 MiB**: a observação termina, portanto inscreva-se em um feed filtrado onde existe um.
* **Fechamento de socket**: a observação termina e Claude recebe o código de fechamento.

Uma observação WebSocket usa uma entrada `ws` no lugar de `command`, e uma única chamada Monitor não pode combinar os dois. A entrada `ws` tem dois campos:

| Campo       | Obrigatório | Descrição                                                                                                                                                      |
| :---------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`       | Sim         | O endpoint para conectar. Deve ser uma URL `ws://` ou `wss://` sem credenciais ou espaços em branco incorporados, usando apenas caracteres ASCII               |
| `protocols` | Não         | Nomes de subprotocolo WebSocket para oferecer durante o handshake. Cada entrada deve ser um token de subprotocolo válido, e a lista não pode conter duplicatas |

O prazo `timeout_ms` também se aplica a uma observação WebSocket: a observação termina no prazo, e `TaskStop` a cancela antecipadamente.

Abrir uma WebSocket solicita aprovação; em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) o classificador decide em vez disso. O prompt não oferece uma opção para pular prompts futuros para o mesmo host.

Claude Code nega URLs que apontam para um endereço privado, link-local ou de metadados de nuvem, incluindo nomes de host que resolvem para um. Também nega hosts em `sandbox.network.deniedDomains`, e quando [`allowManagedDomainsOnly`](/docs/pt/settings-reference#sandbox-network-allowmanageddomainsonly) está definido em configurações gerenciadas, qualquer host fora da lista de permissões gerenciada.

<h2 id="notebookedit-tool-behavior">
  Comportamento da ferramenta NotebookEdit
</h2>

NotebookEdit modifica um notebook Jupyter uma célula por vez, direcionando células por seu `cell_id`. Ela não realiza substituição de string em todo o notebook da forma que [Edit](#edit-tool-behavior) faz em arquivos simples.

Três modos de edição controlam o que acontece com a célula alvo:

* `replace`: sobrescreve a fonte da célula. Este é o padrão.
* `insert`: adiciona uma nova célula após a alvo. Sem `cell_id`, a nova célula vai no início do notebook. Requer `cell_type` definido como `code` ou `markdown`.
* `delete`: remove a célula alvo.

Regras de permissão usam o formato de caminho `Edit(...)`. Uma regra como `Edit(notebooks/**)` cobre chamadas NotebookEdit em arquivos nesse diretório.

<h2 id="powershell-tool">
  Ferramenta PowerShell
</h2>

A ferramenta PowerShell permite que Claude execute comandos PowerShell nativamente. No Windows, isso significa que os comandos são executados no PowerShell em vez de serem roteados através do Git Bash. Como a ferramenta fica disponível depende da sua plataforma:

* **Windows sem Git Bash**: a ferramenta é ativada automaticamente.
* **Windows com Git Bash instalado**: a ferramenta está ativada por padrão para contas claude.ai e Console; defina `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` para ativá-la em sessões do Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, ou `0` para desativá-la.
* **Linux, macOS e WSL**: a ferramenta é opcional.

Seus [hooks PreToolUse](/docs/pt/hooks#powershell) recebem a string de comando da ferramenta em `tool_input.command`, com os mesmos campos que a ferramenta Bash.

Combine `Bash|PowerShell` em hooks que inspecionam comandos de shell; a [seção de entrada do hook PowerShell](/docs/pt/hooks#powershell) explica por que combinar apenas `Bash` não é suficiente.

<h3 id="enable-the-powershell-tool">
  Ativar a ferramenta PowerShell
</h3>

Defina `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` em seu ambiente ou em `settings.json`:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

No Windows, defina a variável como `0` para desativar a ferramenta. No Linux, macOS e WSL, a ferramenta requer PowerShell 7 ou posterior: instale `pwsh` e certifique-se de que está em seu `PATH`.

No Windows, Claude Code detecta automaticamente `pwsh.exe` para PowerShell 7+ com fallback para `powershell.exe` para PowerShell 5.1. Quando a ferramenta está ativada, Claude trata PowerShell como o shell principal. A ferramenta Bash permanece disponível para scripts POSIX quando Git Bash está instalado.

Claude Code inicia PowerShell com `-ExecutionPolicy Bypass` apenas no escopo do processo, portanto scripts `.ps1` e importações de módulos funcionam em instalações padrão do Windows sem alterar a política da máquina. O bypass no escopo do processo não substitui a Group Policy `MachinePolicy` ou `UserPolicy`, portanto as políticas corporativas ainda se aplicam. Para respeitar a política de execução efetiva da máquina, defina `CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1`.

<h3 id="shell-selection-in-settings-hooks-and-skills">
  Seleção de shell em configurações, hooks e skills
</h3>

Três configurações adicionais controlam onde PowerShell é usado:

* `"defaultShell": "powershell"` em [`settings.json`](/docs/pt/settings-reference#all-settings): roteia comandos interativos `!` através do PowerShell. Requer que a ferramenta PowerShell esteja ativada.
* `"shell": "powershell"` em [hooks de comando](/docs/pt/hooks#command-hook-fields) individuais: executa esse hook no PowerShell. Os hooks iniciam PowerShell diretamente, portanto isso funciona independentemente de `CLAUDE_CODE_USE_POWERSHELL_TOOL`.
* `shell: powershell` em [frontmatter de skill](/docs/pt/skills#frontmatter-reference): executa blocos `` !`command` `` no PowerShell. Requer que a ferramenta PowerShell esteja ativada.

O mesmo comportamento de redefinição de diretório de trabalho da sessão principal descrito na seção da ferramenta Bash se aplica aos comandos PowerShell, incluindo a variável de ambiente `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR`.

A partir da v2.1.196, o código de saída 1 de `grep`, `rg`, `egrep`, `fgrep`, `findstr` e `git grep` significa nenhuma correspondência. O código de saída 1 de `git diff` significa que existem diferenças. Nenhum resultado é relatado ao Claude como uma falha de comando. Para `robocopy`, códigos de saída de 0 a 7 são resultados informativos, como arquivos copiados ou arquivos extras detectados. Códigos de saída de 8 ou superior contam como falhas.

<h3 id="windows-encoding-and-exit-codes">
  Codificação do Windows e códigos de saída
</h3>

No Windows, os seguintes comportamentos de codificação e código de saída do PowerShell requerem Claude Code v2.1.214 ou posterior:

* Redirecionamento com `>` e `>>` escreve arquivos UTF-8 no PowerShell 5.1
* Claude Code codifica texto canalizado para a entrada padrão de um comando nativo como UTF-8
* Claude Code captura saída de erro sem sequências de escape ANSI
* Um comando cujo processo filho aguarda na entrada padrão recebe fim de arquivo em vez de travar
* O código de saída 1 de `where.exe` significa nenhuma correspondência, e de `fc.exe` e `diff.exe` significa que os arquivos diferem, portanto quando o comando produz saída, Claude Code trata esse código de saída como uma resposta negativa válida em vez de um erro de comando. Claude Code ainda relata uma forma silenciada, como `where.exe /Q` ou um redirecionamento para `$null`, como uma falha no código de saída 1

Antes da v2.1.214, `>` no PowerShell 5.1 escrevia arquivos UTF-16LE, entrada não-ASCII canalizada chegava como `?`, e scripts Python poderiam falhar com um `UnicodeEncodeError` ao imprimir caracteres não-ASCII.

<h3 id="preview-limitations">
  Limitações de visualização
</h3>

A ferramenta PowerShell tem as seguintes limitações conhecidas durante a visualização:

* Perfis do PowerShell não são carregados
* No Windows, sandboxing não é suportado

<h2 id="read-tool-behavior">
  Comportamento da ferramenta Read
</h2>

A ferramenta Read recebe um caminho de arquivo e retorna o conteúdo com números de linha. Claude é instruído a sempre passar caminhos absolutos.

Por padrão, Read retorna o arquivo desde o início. Quando uma leitura de arquivo inteiro excede o limite de tokens, Read retorna a primeira página com um aviso de `PARTIAL view` que informa a Claude quanto do arquivo foi recebido e como ler mais com `offset` e `limit`. Uma leitura que passa um `offset` ou `limit` explícito e ainda assim excede o limite de tokens retorna um erro.

Uma leitura com um `limit` explícito para assim que as linhas selecionadas excedem o que o limite de tokens poderia caber e retorna um erro sem carregar o resto do intervalo. O erro informa a Claude para usar um `limit` menor, ou para procurar conteúdo específico com [Grep](#grep-tool-behavior) em vez disso quando uma única linha é tão grande. Antes da v2.1.208, Claude Code carregava todo o intervalo na memória antes de rejeitá-lo, então ler um arquivo com uma única linha extremamente longa poderia fazer com que ficasse sem memória.

Ler um arquivo vazio retorna um aviso de que o arquivo existe mas seu conteúdo está vazio, e um `offset` além da última linha retorna um aviso informando a contagem de linhas do arquivo. Antes da v2.1.208, ler um arquivo vazio retornava o aviso past-the-end em vez disso.

Read lida com vários tipos de arquivo além de texto simples:

* **Imagens**: PNG, JPG e outros formatos de imagem são retornados como conteúdo visual que Claude pode ver, não como bytes brutos. Claude Code redimensiona e recompacta imagens grandes para se adequarem aos limites de tamanho de imagem do modelo antes de enviá-las, então Claude pode ver uma versão reduzida de uma captura de tela grande. A partir da v2.1.196, uma imagem que ainda é maior que 500KB após esse redimensionamento é re-codificada como JPEG com qualidade reduzida com suas dimensões de pixel inalteradas. Se Claude perder detalhes de nível de pixel fino em uma imagem grande, peça-lhe para cortar a região de interesse primeiro, por exemplo com ImageMagick via Bash.
* **PDFs**: Claude lê arquivos `.pdf` curtos por inteiro. Para PDFs com mais de 10 páginas, ele lê em intervalos com um parâmetro `pages`, como `"1-5"`, até 20 páginas por vez.
* **Notebooks Jupyter**: arquivos `.ipynb` retornam todas as células com suas saídas, incluindo código, markdown e visualizações. Claude Code recusa-se a ler um arquivo de notebook com mais de 100 MB; o erro informa a Claude como ler uma porção do notebook em vez disso, como uma fatia de células, com um comando shell.

Read apenas lê arquivos, não diretórios. Claude lista o conteúdo do diretório com um comando shell como `ls`.

<h2 id="sendfeedback-tool-behavior">
  Comportamento da ferramenta SendFeedback
</h2>

O feedback redigido por Claude é um relatório de feedback sobre Claude Code que Claude escreve para você. Requer Claude Code v2.1.238 ou posterior. Claude Code salva cada rascunho em sua máquina em `~/.claude/feedback/drafts/`, e nada chega à Anthropic até que você o envie. Claude redige um com a ferramenta SendFeedback quando:

* Uma ferramenta ou comando continua falhando
* Não consegue ajudá-lo com algo que você pediu
* Você aponta um erro que cometeu, ou ele detecta um
* Você pede para registrar feedback

<h3 id="what-you-see-when-claude-drafts">
  O que você vê quando Claude redige
</h3>

Depois que Claude coloca um rascunho na fila, você vê um cartão acima do seu prompt com o título do rascunho. Pressione `1` para revisar o rascunho, pressione `2` duas vezes para enviá-lo como está escrito, ou pressione `0` para descartá-lo. Um rascunho descartado permanece em sua fila. Depois de descartar um cartão, Claude Code pergunta se deseja desativar o feedback redigido por Claude. Ele para de perguntar depois que você recusa duas vezes.

Por padrão, você vê no máximo três cartões em uma sessão; a Anthropic pode ajustar esse limite do servidor sem uma versão. Após o limite, e sempre que você definir [`feedbackDrafts`](/docs/pt/settings-reference#feedbackdrafts) como `quiet`, você vê apenas uma contagem de rascunhos na fila no rodapé do prompt.

<h3 id="review-and-edit-a-draft">
  Revisar e editar um rascunho
</h3>

Execute `/feedback` sem argumentos para abrir sua fila. Ele lista todos os rascunhos na fila de todas as suas sessões, incluindo rascunhos cujos cartões você descartou ou nunca viu. Selecione um rascunho para abri-lo para revisão, onde você pode:

* Editar o título, área e detalhes
* Definir **Enviar transcrição** como `sim` ou `não`. Quando a transcrição da sessão em que Claude colocou o rascunho na fila ainda está disponível, ela começa em `sim`, o que envia essa conversa para a Anthropic; `não` envia apenas o relatório
* Enviar o rascunho, descartá-lo ou deixá-lo na fila para depois

Para escrever um relatório você mesmo, pressione `w` para o diálogo de feedback padrão. `/feedback` com texto após ele, e `/bug`, abrem esse diálogo diretamente.

<h3 id="send-a-draft">
  Enviar um rascunho
</h3>

Quando você envia um rascunho, Claude Code o envia da mesma forma que um relatório `/feedback`, com a mesma [retenção](/docs/pt/data-usage#feedback-using-the-%2Ffeedback-command), e exclui o rascunho de sua máquina. Quando você envia do cartão, ele mostra `✓ Enviado`; quando você envia da fila, ele fecha com uma ID de recebimento.

O relatório contém:

* Seu título, área e detalhes
* Informações de ambiente, como sua versão do Claude Code, sistema operacional e modelo
* Os IDs de solicitações de API recentes
* A transcrição da conversa, quando você deixou **Enviar transcrição** em `sim` na tela de revisão. Enviar do cartão nunca inclui a transcrição

Claude Code mantém seu diretório de trabalho no rascunho local para que possa encontrar a transcrição, e não envia o diretório.

Em [organizações com retenção zero de dados](/docs/pt/zero-data-retention#features-disabled-under-zdr), Claude Code deixa a ferramenta de fora, assim como faz para `/feedback`. Se uma sessão em tal organização ainda oferecer a ferramenta, os rascunhos permanecem em sua máquina, e o envio falha com `Feedback collection is not available for organizations with custom data retention policies.`

<h3 id="discard-or-keep-a-draft">
  Descartar ou manter um rascunho
</h3>

Quando você descarta um rascunho, Claude Code o exclui de sua máquina. Um rascunho que você deixa na fila expira após 30 dias, ou após [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays) quando esse for mais curto. A fila contém 10 rascunhos em todas as suas sessões, e quando Claude coloca um décimo primeiro na fila, Claude Code exclui o mais antigo. Quando você executa `/exit` com rascunhos da sessão ainda na fila, Claude Code pergunta se deseja revisá-los ou descartá-los antes de sair.

<h3 id="turn-claude-drafted-feedback-off">
  Desativar feedback redigido por Claude
</h3>

Defina **Claude-drafted feedback** como `off` em `/config`, que escreve a configuração [`feedbackDrafts`](/docs/pt/settings-reference#feedbackdrafts), ou defina [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/pt/env-vars) para uma sessão. Com qualquer um deles, Claude não pode colocar rascunhos na fila. Para manter a redação ativada sem cartões, defina `feedbackDrafts` como `quiet`. Administradores podem definir `feedbackDrafts` em [configurações gerenciadas](/docs/pt/managed-settings), que tem precedência sobre sua própria configuração.

<h3 id="sessions-without-claude-drafted-feedback">
  Sessões sem feedback redigido por Claude
</h3>

Claude Code inclui a ferramenta em sessões de terminal interativas em sua própria máquina que usam a API Claude em vez de um provedor de nuvem. Ele deixa a ferramenta de fora de:

* Execuções não interativas `-p` e sessões do [Agent SDK](/docs/pt/agent-sdk/overview), que não têm tela para revisar a fila
* [Sessões em nuvem](/docs/pt/claude-code-on-the-web), que não conseguem escrever na fila em sua máquina
* Sessões em [Amazon Bedrock](/docs/pt/amazon-bedrock), [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai), ou [Microsoft Foundry](/docs/pt/microsoft-foundry)
* Sessões em que você definiu [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/pt/env-vars) ou [`DISABLE_FEEDBACK_COMMAND=1`](/docs/pt/env-vars), definiu `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` para qualquer valor não vazio, ou desativou [busca de sinalizador de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching)
* Organizações que desativaram feedback de produto, e [organizações com retenção zero de dados](/docs/pt/zero-data-retention#features-disabled-under-zdr)

<h2 id="task-tool-availability">
  Disponibilidade da ferramenta Task
</h2>

As ferramentas de rastreamento de tarefas, `TaskCreate`, `TaskGet`, `TaskUpdate`, `TaskList` e `TodoWrite`, estão disponíveis por padrão apenas nos modelos Claude 3.x, Opus 4 até 4.7, Sonnet 4 até 4.6 e Haiku 4.5. Sempre que as ferramentas estão disponíveis, você obtém as quatro ferramentas Task, ou `TodoWrite` quando você define [`CLAUDE_CODE_ENABLE_TASKS=0`](/docs/pt/env-vars).

Em todos os outros modelos, Claude Code deixa as ferramentas de fora a menos que você opte por usá-las. O mesmo se aplica a um ID de modelo que Claude Code não reconhece, como um nome de modelo personalizado servido através de um [gateway LLM](/docs/pt/llm-gateway). Em modelos mais novos, Claude acompanha o trabalho em várias etapas sem uma lista de verificação escrita, e as definições e lembretes das ferramentas ocupam contexto. Sem as ferramentas, Claude não adiciona nada à [lista de tarefas](/docs/pt/interactive-mode#task-list) enquanto trabalha.

Se você gostaria de usar essas ferramentas em um modelo que não as possui por padrão, faça um dos seguintes:

* Exporte [`CLAUDE_CODE_ENABLE_TODO_TOOLS=1`](/docs/pt/env-vars) antes de iniciar Claude Code, por exemplo `CLAUDE_CODE_ENABLE_TODO_TOOLS=1 claude`. Claude Code então fornece as mesmas ferramentas em todos os modelos e todos os provedores
* Nomeie uma das ferramentas em [`--allowedTools`](/docs/pt/cli-reference#cli-flags), por exemplo `claude --allowedTools TaskCreate`
* Liste as ferramentas em [`--tools`](/docs/pt/cli-reference#cli-flags), que restringe as ferramentas integradas da sessão àquelas que ela nomeia. Inclua as ferramentas que você deseja junto com as outras ferramentas integradas que você usa
* No Agent SDK, as opções [`allowedTools` e `tools`](/docs/pt/agent-sdk/todo-tracking#model-availability) funcionam da mesma forma que os dois sinalizadores

Em [sessões em segundo plano](/docs/pt/agent-view) e em [Claude Code na web](/docs/pt/claude-code-on-the-web), Claude Code fornece as mesmas ferramentas em todos os modelos, listados ou não.

Claude Code fornece a um suagente as ferramentas apenas quando sua sessão as possui, mesmo quando o suagente executa um modelo diferente. Um colega de [equipe de agentes](/docs/pt/agent-teams) em processo segue sua sessão da mesma forma, enquanto um colega em seu próprio [painel dividido](/docs/pt/agent-teams#choose-a-display-mode) é executado como um processo Claude Code separado, portanto seu próprio modelo decide. Sem as ferramentas Task, um agente coordena com sua equipe através de mensagens em vez da [lista de tarefas compartilhada](/docs/pt/agent-teams#assign-and-claim-tasks).

O conjunto padrão descrito aqui se aplica no Claude Code v2.1.268 e posterior.

<h2 id="webfetch-tool-behavior">
  Comportamento da ferramenta WebFetch
</h2>

WebFetch recebe uma URL e um prompt descrevendo o que extrair. Ele busca a página, converte a resposta para Markdown quando o servidor retorna HTML, e executa o prompt contra o conteúdo usando um modelo pequeno e rápido. Para a maioria das buscas, Claude recebe a resposta desse modelo, não a página bruta. A etapa de conversão não é configurável.

Isso torna WebFetch lossy por design. O prompt de extração determina o que chega a Claude, então um resultado que diz que uma página não menciona algo pode apenas significar que o prompt não perguntou sobre isso. Peça a Claude para buscar novamente com um prompt mais específico, ou use `curl` via Bash para a página não processada.

Alguns comportamentos moldam a resposta que Claude recebe:

* WebFetch recusa `localhost` e qualquer outro nome de host sem um ponto, como um nome de intranet simples, antes de fazer uma solicitação. O [erro que retorna](/docs/pt/errors#webfetch-cannot-fetch-localhost) diz a Claude para alcançar servidores locais com `curl` através do Bash.
* URLs HTTP são automaticamente atualizadas para HTTPS.
* Páginas grandes são truncadas para um limite de caracteres fixo antes do processamento.
* WebFetch armazena em cache cada resposta por 15 minutos por padrão, então buscas repetidas da mesma URL retornam rapidamente. No Claude Code v2.1.233 ou posterior, defina [`CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS`](/docs/pt/env-vars#variables) para alterar quanto tempo WebFetch mantém cada resposta.
* Uma página que não terminou de fazer download em cinco minutos, incluindo qualquer redirecionamento que WebFetch segue, falha com um erro de deadline. No Claude Code v2.1.268 ou posterior, defina [`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`](/docs/pt/env-vars#variables) para alterar o limite, ou para `0` para removê-lo.
* Quando uma URL redireciona para um host diferente, WebFetch retorna um resultado de texto que nomeia a URL original e o alvo de redirecionamento em vez de segui-lo. Claude então busca a nova URL com uma segunda chamada WebFetch.
* Quando a etapa de extração atinge uma API sobrecarregada, Claude Code tenta novamente com backoff; uma busca que ainda falha retorna um resultado de erro. Antes da v2.1.212, o texto de erro da API poderia chegar a Claude como se fosse o conteúdo da página extraída.

Nos modos Manual e `acceptEdits` [permission modes](/docs/pt/permission-modes), WebFetch solicita antes de buscar, exceto para domínios que suas [permission rules](/docs/pt/permissions#manage-permissions) já permitem ou negam e um conjunto integrado de domínios de documentação pré-aprovados que buscam sem um prompt. Qualquer que seja suas regras permitam, uma busca também passa pela [WebFetch domain safety check](/docs/pt/data-usage#webfetch-domain-safety-check) primeiro; essa seção cobre o que a verificação envia e a configuração que a ignora. O prompt oferece três opções:

* **Yes**: aprova apenas esta busca. A próxima chamada WebFetch solicita novamente, mesmo para o mesmo domínio.
* **Yes, and don't ask again for `<domain>`**: aprova a busca e salva uma regra de permissão `WebFetch(domain:...)` para esse domínio em `.claude/settings.local.json` para esse repositório. Veja [como as aprovações salvas persistem](/docs/pt/permissions#permission-system). Quando sua organização define [`allowManagedPermissionRulesOnly`](/docs/pt/permissions#managed-only-settings), Claude Code oculta essa opção.
* **No, and tell Claude what to do differently**: rejeita a busca.

Para permitir um domínio antecipadamente sem um prompt, adicione uma regra de permissão como `WebFetch(domain:example.com)`; `WebFetch(domain:*)` permite todos os domínios. Os modos de permissão `auto` e `bypassPermissions` [permission modes](/docs/pt/permissions#permission-modes) ignoram o prompt, exceto para um domínio que uma regra `ask` explícita corresponde.

Uma regra `WebFetch(domain:...)` explícita em `deny`, `ask` ou `allow` tem precedência sobre o conjunto pré-aprovado, então você pode bloquear um domínio pré-aprovado ou exigir um prompt para ele.

WebFetch define um cabeçalho `User-Agent` começando com `Claude-User`, e um cabeçalho `Accept` que prefere Markdown sobre HTML para que servidores que suportam negociação de conteúdo possam retornar Markdown diretamente.

Comandos em sandbox não herdam o conjunto integrado de domínios de documentação pré-aprovados do WebFetch. Para permitir que um comando em sandbox alcance um domínio sem um prompt, adicione o domínio a [`allowedDomains`](/docs/pt/settings-reference#sandbox-network-alloweddomains) ou permita-o com uma regra `WebFetch(domain:...)`, que o [sandbox também honra](/docs/pt/sandboxing#network-isolation). WebFetch nunca lê a lista de permissões do sandbox em troca, então adicionar um domínio a um sandbox ou lista de permissões de rede da organização não impede que WebFetch solicite por ele.

<h2 id="websearch-tool-behavior">
  Comportamento da ferramenta WebSearch
</h2>

WebSearch executa uma consulta no backend de [web search](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) da Anthropic e retorna títulos e URLs dos resultados. Ele não busca as páginas de resultados. Para ler uma página que Claude encontra nos resultados de busca, ele faz um acompanhamento com [WebFetch](#webfetch-tool-behavior).

A ferramenta pode emitir até oito buscas de backend por chamada, refinando a busca internamente antes de retornar resultados. Claude pode delimitar resultados com `allowed_domains` para incluir apenas certos hosts, ou `blocked_domains` para excluí-los. As duas listas não podem ser combinadas em uma única chamada.

Quando a solicitação de busca atinge uma API sobrecarregada, Claude Code tenta novamente com backoff; uma chamada que ainda falha retorna um resultado de erro. Antes da v2.1.212, o texto de erro da API poderia chegar a Claude como se fossem resultados de busca.

As regras de permissão do WebSearch não usam especificador. Uma entrada `WebSearch` simples em `allow` ou `deny` é a única forma.

O backend de busca não é configurável. Para buscar com um provedor diferente, adicione um [servidor MCP](/docs/pt/mcp) que exponha uma ferramenta de busca.

<Note>
  WebSearch está disponível na Claude API e na [Claude Platform on AWS](/docs/pt/claude-platform-on-aws). No Microsoft Foundry, ele requer uma [implantação hospedada na Anthropic](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options): implantações hospedadas no Azure não suportam ferramentas do lado do servidor, portanto a chamada do WebSearch falha. Na Agent Platform do Google Cloud, funciona com Claude 4 e modelos posteriores, incluindo Opus, Sonnet e Haiku. Amazon Bedrock não expõe a ferramenta de web search do lado do servidor.
</Note>

<h3 id="session-search-limit">
  Limite de busca da sessão
</h3>

Uma sessão pode fazer no máximo 200 chamadas do WebSearch, contadas em toda a conversa principal e em cada [subagent](/docs/pt/sub-agents) que ela gera, portanto as buscas feitas por fan-outs de pesquisa paralela contam contra o mesmo limite. O limite requer Claude Code v2.1.212 ou posterior. Quando Claude atinge o limite, chamadas posteriores retornam um aviso dizendo a Claude para continuar com as informações que já reuniu, em vez de um erro que convidaria a uma tentativa novamente. Você não vê o aviso: uma chamada limitada aparece na conversa como uma busca que não fez nada, e se Claude precisar de mais buscas, o aviso diz a ele para pedir que você aumente o limite.

Defina a variável de ambiente [`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](/docs/pt/env-vars) para alterar o limite; ela aceita um número inteiro positivo, portanto o limite pode ser aumentado, mas não desativado. Executar [`/clear`](/docs/pt/commands#all-commands) redefine a contagem. Se o trabalho que ainda pode gerar [subagents](/docs/pt/sub-agents) sobreviver à limpeza, como um fluxo de trabalho em execução, a contagem é mantida.

<h2 id="write-tool-behavior">
  Comportamento da ferramenta Write
</h2>

A ferramenta Write cria um novo arquivo ou sobrescreve um existente com o conteúdo completo fornecido. Ela não anexa ou mescla.

Se Claude deve ler um arquivo existente na conversa atual antes de sobrescrevê-lo depende do modelo e do arquivo:

* Claude Opus 4.6, Claude Haiku 4.5 e modelos mais antigos sempre exigem a leitura, portanto uma Write para um arquivo existente não lido falha com um erro.
* Modelos mais novos podem sobrescrever um arquivo que nunca leram nesta sessão sob as mesmas condições que [read-before-edit](#edit-tool-behavior): lê-lo não precisaria de um prompt de permissão e a ferramenta Read está disponível.
* Notebooks Jupyter e arquivos que Claude leu apenas parcialmente com um aviso [`PARTIAL view`](#read-tool-behavior) exigem a leitura em todos os modelos.

Esta restrição não se aplica a novos arquivos. Antes da v2.1.228, todos os modelos exigiam a leitura antes de sobrescrever um arquivo existente.

Visualizar o arquivo com Bash também satisfaz este requisito sob as mesmas regras descritas em [Comportamento da ferramenta Edit](#edit-tool-behavior).

Para alterações parciais em um arquivo existente, Claude usa Edit em vez de Write.

<h2 id="check-which-tools-are-available">
  Verificar quais ferramentas estão disponíveis
</h2>

Seu conjunto exato de ferramentas depende do seu provedor, plataforma e configurações. Para verificar o que está carregado em uma sessão em execução, pergunte a Claude diretamente:

```text theme={null}
What tools do you have access to?
```

Claude fornece um resumo conversacional. Para nomes exatos de ferramentas MCP, execute `/mcp`.

<Note>
  A [ferramenta advisor](/docs/pt/advisor) é uma [ferramenta de servidor](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool) que a API executa, em vez de uma ferramenta que Claude Code implementa. Ela não tem um nome que você possa referenciar em regras de permissão ou correspondências de hook.
</Note>

<h2 id="see-also">
  Veja também
</h2>

* [Servidores MCP](/docs/pt/mcp): adicione ferramentas personalizadas conectando servidores externos
* [Permissões](/docs/pt/permissions): sistema de permissões, sintaxe de regras e padrões específicos de ferramentas
* [Subagents](/docs/pt/sub-agents): configure o acesso a ferramentas para subagents
* [Hooks](/docs/pt/hooks-guide): execute comandos personalizados antes ou depois da execução da ferramenta
