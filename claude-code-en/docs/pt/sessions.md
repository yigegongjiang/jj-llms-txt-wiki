> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gerenciar sessões

> Nomeie, retome, ramifique e alterne entre conversas do Claude Code. Abrange `--continue`, `--resume`, `--from-pr`, o seletor `/resume`, nomeação de sessão, exportação de transcritos e onde os transcritos são armazenados.

Uma sessão é uma conversa salva vinculada a um diretório de projeto. Claude Code a armazena localmente conforme você trabalha, para que você possa retomar de onde parou, ramificar para tentar uma abordagem diferente ou alternar entre tarefas.

O [aplicativo desktop](/docs/pt/desktop#work-in-parallel-with-sessions), [Claude Code na web](/docs/pt/claude-code-on-the-web) e a [extensão VS Code](/docs/pt/vs-code#resume-past-conversations) mantêm seu próprio histórico de sessões. Esta página abrange a CLI.

<h2 id="resume-a-session">
  Retomar uma sessão
</h2>

As sessões são salvas continuamente em [arquivos de transcrição locais](#export-and-locate-session-data) conforme você trabalha, para que você possa retornar a uma após sair ou executar `/clear`. Use estes pontos de entrada:

| Comando                             | O que faz                                                                                                               |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `claude --continue`                 | Retoma a conversa mais recente no diretório atual                                                                       |
| `claude --resume`                   | Abre o [seletor de sessão](#use-the-session-picker)                                                                     |
| `claude --resume <name>`            | Retoma a sessão nomeada diretamente                                                                                     |
| `claude --resume <transcript-path>` | Retoma a conversa armazenada no arquivo de [transcrição](#where-transcripts-are-stored) `.jsonl` nesse caminho absoluto |
| `claude --from-pr <number>`         | Abre o seletor de sessão filtrado para sessões vinculadas a esse pull request                                           |
| `/resume`                           | Alterna para uma conversa diferente de dentro de uma sessão ativa                                                       |

Claude Code deixa as sessões criadas com [`claude -p`](/docs/pt/headless) ou o [Agent SDK](/docs/pt/agent-sdk/overview) fora do seletor de sessão e fora de `claude --continue`. Você ainda pode retomar uma passando seu ID de sessão para `claude --resume <session-id>`. Com `claude --continue`, Claude Code também pula [sessões cuja primeira solicitação foi `/loop`](#where-the-session-picker-looks). Quando você executa [`claude -p --continue`](/docs/pt/headless#continue-conversations), Claude Code inclui sessões `-p`, SDK e `/loop`.

`claude --continue` abre uma [sessão em background](/docs/pt/agent-view) que foi concluída, mas não uma que ainda está em execução; abrir sessões em background concluídas requer Claude Code v2.1.257 ou posterior. Se sua conversa mais recente for uma que você [moveu para o background](/docs/pt/agent-view#send-the-session-to-the-background) e ainda estiver em execução lá, Claude Code sai com `Your most recent conversation is running in the background` e o ID dessa sessão. Anexe à sessão a partir de [`claude agents`](/docs/pt/agent-view#attach-to-a-session), ou execute `claude --resume` para escolher outra.

Você pode executar `claude --resume <session-id>` de qualquer diretório: Claude Code procura o ID no diretório do projeto atual e seus git worktrees primeiro, depois em todos os outros projetos nesta máquina, para que encontre uma sessão que começou em outro lugar ou se moveu com [`/cd`](/docs/pt/commands). A busca entre projetos resolve o ID apenas quando exatamente um outro projeto contém uma transcrição com mensagens para ele, portanto uma duplicata copiada manualmente faz Claude Code relatar não encontrado em vez de retomar uma cópia arbitrária. Se nenhuma sessão armazenada corresponder ao ID, Claude Code relata `No conversation found with session ID: <session-id>`. Antes da v2.1.223, a busca parava no diretório do projeto atual e seus git worktrees, portanto você tinha que retomar do diretório em que a sessão trabalhou por último.

<h3 id="what-a-resumed-session-restores">
  O que uma sessão retomada restaura
</h3>

Uma sessão retomada restaura a conversa junto com o estado salvo nela:

* Histórico de conversa: o histórico completo, incluindo chamadas de ferramentas e resultados. Uma ferramenta que ainda estava em execução quando o processo anterior terminou, por exemplo em uma falha, não termina ou executa novamente quando você retoma. Claude vê a chamada marcada como interrompida antes de seu resultado ser registrado e é instruído a verificar se ela teve efeito antes de executá-la novamente, a menos que [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](/docs/pt/env-vars#variables) esteja definido. Antes da v2.1.281, Claude Code descartava a chamada interrompida da conversa ou a mostrava a Claude como uma que você interrompeu.
* Modelo: a sessão continua no modelo que estava usando. O modelo não é restaurado quando foi descontinuado ou não é permitido por `availableModels`, quando uma flag `--model` ou uma variável de ambiente da família `ANTHROPIC_MODEL` escolhe um no lançamento, ou em provedores que usam IDs de implantação específicos do provedor, como [Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry](/docs/pt/third-party-integrations); veja [configuração de modelo](/docs/pt/model-config#setting-your-model) para a ordem de resolução.
* Agente: uma sessão iniciada com [`--agent`](/docs/pt/sub-agents#invoke-subagents-explicitly) ou a configuração `agent` continua como esse agente, mantendo suas restrições de ferramentas e modelo. Passe `--agent` ao retomar para escolher um diferente; para o prompt do sistema em ambos os casos, veja [Flags de prompt do sistema em conversas retomadas](/docs/pt/cli-reference#system-prompt-flags-in-resumed-conversations). Claude Code procura o agente em dois lugares: o diretório original da sessão, desde que você tenha [confiado nesse workspace](/docs/pt/permissions#project-allow-rules-and-workspace-trust), e depois o diretório de onde você retoma, para que um agente com escopo de projeto ainda carregue quando você retoma de outro diretório. Se Claude Code não encontrar o agente em nenhum dos dois lugares, a sessão retoma com as ferramentas padrão e mostra um [aviso nomeando o agente](/docs/pt/errors#session-agent-no-longer-available).
* Modo de permissão: se você retomar de um terminal com `claude --continue`, `claude --resume <session-id>` ou `claude --resume <name>` quando o nome corresponde a uma sessão, sem `-p`, Claude Code restaura o modo de permissão em que a sessão estava, exceto nos casos em [modo de permissão ao retomar](#permission-mode-on-resume), que também cobre o seletor de sessão, `/resume` e retomar com `claude -p`. Passe `--permission-mode` ou `--dangerously-skip-permissions` para substituir o modo restaurado.
* Objetivo ativo: um [objetivo](/docs/pt/goal#resume-with-an-active-goal) que ainda estava ativo quando a sessão terminou é transferido; sua contagem de turnos, temporizador e linha de base de gasto de tokens são redefinidos.
* Tarefas agendadas: [tarefas que não expiraram](/docs/pt/scheduled-tasks#limitations) são restauradas. Tarefas Bash em background e tarefas de monitoramento não são.

Nem toda flag de configuração do lançamento original é restaurada. Se a sessão dependia de `--mcp-config`, `--settings`, `--plugin-dir`, `--fallback-model` ou diretórios adicionados com `--add-dir`, passe-os novamente quando você retomar; diretórios adicionados no meio da sessão com `/add-dir` também não são restaurados, embora o seletor de sessão ainda os use para localizar a sessão. Os arquivos de configurações padrão, como `settings.json` e `settings.local.json`, são relidos no lançamento, portanto a configuração que reside neles não precisa ser passada novamente. Para `--system-prompt` e `--append-system-prompt`, veja [Flags de prompt do sistema em conversas retomadas](/docs/pt/cli-reference#system-prompt-flags-in-resumed-conversations).

<h4 id="permission-mode-on-resume">
  Modo de permissão ao retomar
</h4>

Qual modo de permissão Claude Code inicia uma sessão retomada depende de como você retoma:

* Terminal: `claude --continue`, `claude --resume <session-id>` ou `claude --resume <name>` quando o nome corresponde a uma sessão, sem `-p`. Claude Code restaura o modo de permissão em que a sessão estava, exceto nos casos da tabela. Passe `--permission-mode` ou `--dangerously-skip-permissions` para substituir o modo restaurado.
* Não interativo: `claude -p --resume` ou `claude -p --continue`. Claude Code inicia a execução no modo de permissão em que uma nova execução `claude -p` iniciaria, exceto que uma sessão que terminou em modo de plano retoma em modo de plano sob as [condições abaixo](#resume-in-plan-mode-with-p).
* VS Code: o painel de conversa da extensão. A tabela cobre apenas uma conversa que terminou em modo de plano; para o resto, veja [retomar conversas passadas](/docs/pt/vs-code#resume-past-conversations).
* Seletor de sessão no lançamento: uma sessão que você seleciona do [seletor de sessão](#use-the-session-picker), se você o abriu com `claude --resume` sozinho, `claude --from-pr` ou um nome que corresponde a mais de uma sessão. Claude Code não restaura o modo de permissão armazenado. Ele inicia a sessão no modo de permissão em que iniciaria uma nova sessão a partir da mesma linha de comando.
* `/resume` dentro de uma sessão, com ou sem um argumento: Claude Code não restaura o modo de permissão armazenado. A conversa para a qual você alterna continua no modo de permissão em que sua sessão atual está.

Restaurar modo de plano nos caminhos não interativo e VS Code requer Claude Code v2.1.246 ou posterior. Cada linha nomeia o modo de permissão em que a sessão terminou, qual dos caminhos terminal, não interativo e VS Code você a retoma, e o modo de permissão em que Claude Code inicia a sessão retomada.

| Sessão terminou em  | Como você retoma                                                       | Modo de permissão após você retomar                                                                                                                                                                                                                                                                                                                                                 |
| :------------------ | :--------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bypassPermissions` | Terminal                                                               | O modo de permissão em que uma nova sessão iniciaria. Para [ignorar permissões](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode) novamente, ative-o no lançamento com uma de suas flags de lançamento ou `permissions.defaultMode: "bypassPermissions"` em [configurações de usuário, `--settings` ou gerenciadas](/docs/pt/settings-reference#permissions-defaultmode) |
| `plan`              | Terminal                                                               | O modo de permissão em que uma nova sessão iniciaria                                                                                                                                                                                                                                                                                                                                |
| `auto`              | Terminal                                                               | `auto`, apenas quando sua conta ainda atende aos [requisitos do modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                                                                                                                                                   |
| Manual              | Terminal                                                               | Manual quando uma nova sessão iniciaria em modo auto a partir do [padrão integrado](/docs/pt/permission-modes#which-mode-a-session-starts-in). Quando um `defaultMode` de um arquivo de configurações [entra em vigor](/docs/pt/permission-modes#which-mode-a-session-starts-in), Claude Code inicia a sessão retomada nesse modo                                                             |
| `plan`              | Não interativo, sob as [condições abaixo](#resume-in-plan-mode-with-p) | Modo de plano                                                                                                                                                                                                                                                                                                                                                                       |
| Qualquer modo       | Não interativo, em qualquer outro caso                                 | O modo de permissão em que uma nova execução `claude -p` iniciaria                                                                                                                                                                                                                                                                                                                  |
| `plan`              | VS Code                                                                | Modo de plano, com [as exceções na página VS Code](/docs/pt/vs-code#resume-past-conversations)                                                                                                                                                                                                                                                                                           |

<h5 id="resume-in-plan-mode-with-p">
  Retomar em modo de plano com `-p`
</h5>

Uma execução `claude -p --resume` ou `claude -p --continue` retoma em modo de plano apenas quando todas as quatro condições se mantêm:

* Você passa [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags), para que Claude Code possa apresentar o plano para aprovação
* Você não passa `--permission-mode` ou `--dangerously-skip-permissions`
* Você não passa `--fork-session`
* A execução não é iniciada através de [canais](/docs/pt/channels)

<h3 id="resume-from-a-summary">
  Retomar de um resumo
</h3>

Em um plano Pro ou Max, quando você retoma uma sessão que ficou inativa por mais de uma hora e tem mais de 100.000 tokens, Claude Code restaura a conversa e depois abre um diálogo antes de você enviar sua primeira mensagem. O [cache de prompt](/docs/pt/prompt-caching#cache-lifetime) da sessão terá expirado até então, portanto a próxima solicitação processa o histórico completo uma vez, não importa qual das opções do diálogo você escolha.

O diálogo oferece três maneiras de continuar a sessão. Elas diferem em quanto da conversa cada uma carrega para solicitações posteriores, o que é uma troca entre manter cada detalhe e enviar menos tokens por solicitação:

* **Retomar do resumo**: executa [`/compact`](/docs/pt/context-window#what-survives-compaction) imediatamente. Claude Code envia uma solicitação de resumo sobre o histórico completo, depois substitui o histórico pelo resumo, suas trocas mais recentes e até cinco arquivos lidos recentemente. Solicitações posteriores carregam o resumo em vez do histórico completo.
* **Retomar sessão completa como está**: carrega a conversa inalterada. Depois que você envia sua primeira mensagem, Claude Code reprocessa e re-armazena em cache o histórico completo, depois o relê do cache em solicitações posteriores enquanto o cache permanece aquecido.
* **Não me pergunte novamente**: retoma a sessão completa e para de mostrar o diálogo em todas as futuras retomadas.

Retomar como está mantém cada detalhe da conversa disponível, a um custo por solicitação que escala com o tamanho da conversa. Retomar do resumo custa menos em cada solicitação posterior porque carrega o resumo em vez do histórico completo, mas o que quer que o resumo deixe de fora não está mais no contexto de Claude. Veja [por que o uso sobe em uma sessão longa](/docs/pt/costs#why-usage-climbs-in-a-long-session) para onde esse custo por solicitação vem.

<h3 id="where-the-session-picker-looks">
  Onde o seletor de sessão procura
</h3>

Claude Code armazena sessões por diretório de projeto. Por padrão, o seletor de sessão mostra:

* Sessões da worktree atual, incluindo [sessões em background](/docs/pt/agent-view), que são marcadas `bg` na lista
* Sessões iniciadas em outro lugar que adicionaram o diretório atual com `/add-dir`

Use `Ctrl+W` para expandir para todas as worktrees do repositório ou `Ctrl+A` para expandir para cada projeto nesta máquina.

Sessões cuja primeira solicitação foi um comando [`/loop`](/docs/pt/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) não aparecem no seletor, e `claude --continue` também as pula. Executar `/loop` mais tarde em uma conversa não oculta a sessão. Antes da v2.1.211, uma execução `/loop` no início de uma conversa ocultava a sessão do seletor permanentemente.

Mover uma sessão com [`/cd`](/docs/pt/commands) a relocata para o armazenamento de projeto do novo diretório, para que apareça no seletor desse diretório depois. A partir da v2.1.196, uma sessão movida fica fora do seletor do diretório antigo mesmo após uma falha ou saída forçada. Em versões anteriores, ela também poderia reaparecer na lista do diretório antigo após uma saída que não foi limpa quando o caminho antigo continha caracteres especiais como sublinhados.

Quando você seleciona uma sessão de outra worktree do mesmo repositório, Claude Code a retoma no local; quando a própria worktree da sessão não existe mais, Claude Code [a retoma no seu diretório atual](/docs/pt/worktrees#resume-a-worktree-session). Quando você seleciona uma sessão de um projeto não relacionado, Claude Code copia um comando `cd` e retoma para sua área de transferência. Se o diretório desse projeto não existir mais, Claude Code retoma a sessão no seu diretório atual em vez de copiar um comando `cd` que falharia.

Retomar por nome resolve no repositório atual e suas worktrees. Ambas as formas procuram por uma correspondência exata e a retomam diretamente mesmo que resida em uma worktree diferente:

| Comando                  | Correspondência exata | Nome ambíguo                                                                    |
| :----------------------- | :-------------------- | :------------------------------------------------------------------------------ |
| `claude --resume <name>` | Retoma diretamente    | Abre o seletor de sessão com o nome pré-preenchido como termo de pesquisa       |
| `/resume <name>`         | Retoma diretamente    | Relata um erro; execute `/resume` sem argumentos para abrir o seletor de sessão |

<h2 id="name-your-sessions">
  Nomeie suas sessões
</h2>

Dê às sessões nomes descritivos para que sejam encontráveis no seletor de sessão e retomáveis por nome. Isso é mais importante quando você está trabalhando em várias tarefas em paralelo.

| Quando                               | Como definir o nome                                                                                                                                                            |
| :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Na inicialização                     | `claude -n auth-refactor`                                                                                                                                                      |
| Durante uma sessão                   | `/rename auth-refactor`. O nome também aparece na barra de prompt                                                                                                              |
| Do seletor de sessão                 | Destaque uma sessão e pressione `Ctrl+R`                                                                                                                                       |
| Na aceitação do plano                | Aceitar um plano no [Plan Mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) dá à sessão um título gerado com base no plano, a menos que você já tenha nomeado |
| Do claude.ai ou do aplicativo Claude | Renomeie uma [sessão Remote Control](/docs/pt/remote-control#connect-from-another-device); Claude Code aplica o mesmo nome na CLI. Requer Claude Code v2.1.221 ou posterior         |
| Do aplicativo desktop                | Renomeie uma sessão no [aplicativo desktop](/docs/pt/desktop#work-in-parallel-with-sessions)                                                                                        |

Depois que você nomeia uma sessão através de uma rota CLI ou do claude.ai, retorne a ela com `claude --resume <name>` ou `/resume <name>`; uma sessão do aplicativo desktop retoma no aplicativo, que mantém seu próprio histórico de sessão. Veja [Retomar uma sessão](#resume-a-session) para saber como a resolução de nomes se comporta entre worktrees.

Quando você inicia ou retoma uma sessão interativa com um nome que outra sessão ativa nesta máquina já usa, ou renomeia uma sessão para tal nome, Claude Code deixa o nome com a sessão que já o possui, renomeia a sua para uma variante com um sufixo de duas palavras, como `auth-refactor-graceful-unicorn`, e avisa você. Execute `/rename` com um novo nome se preferir escolher um você mesmo. Antes da v2.1.232, ambas as sessões mantinham o nome.

Em três casos Claude Code não renomeia a duplicata, então você ainda pode ver duas sessões com o mesmo nome em listagens:

* Não verifica títulos gerados por IA ou nomes de exibição padrão.
* Não verifica o `--name` de uma sessão [background](/docs/pt/agent-view#from-your-shell) ou `-p` na inicialização.
* Não consegue renomear uma sessão em uma versão anterior do Claude Code.

Sessões que você não nomeia ainda recebem dois rótulos que Claude Code atribui. Apenas o título gerado funciona como um identificador de retomada:

* Nome de exibição padrão: sessões interativas que você nunca nomeia ainda recebem um nome de exibição padrão quando iniciam. Requer Claude Code v2.1.196 ou posterior. O padrão combina o nome do diretório de trabalho com um sufixo de dois caracteres, por exemplo `my-app-3f`, e identifica a sessão em listagens de sessões em execução, como [agent view](/docs/pt/agent-view) e saída de `claude agents --json`. O padrão não é um identificador de retomada. Se você o passar para `claude --resume` ou `/resume`, Claude Code não encontra a sessão. Nomear a sessão substitui o padrão nessas listagens, e assim faz aceitar um plano.
* Título gerado: se você não nomear uma sessão, Claude Code gera um título de sessão para ela. O título é um resumo breve do seu primeiro prompt, escrito por uma solicitação em background para o modelo pequeno/rápido, normalmente um modelo da classe Haiku. Um `claude -p` executado que você inicia diretamente de um shell ou script não recebe um.

  Aceitar um plano substitui o título do primeiro prompt por um título baseado no plano. Nomear a sessão também o substitui.

  Você vê o título do primeiro prompt no [seletor de sessão](#use-the-session-picker) e no campo [`session_name`](/docs/pt/statusline) da statusline quando nenhum nome está definido. O título do plano aparece nos mesmos dois lugares e também nas listagens de sessões em execução, onde substitui o nome de exibição padrão.

  Você pode passar qualquer um dos títulos para `claude --resume` ou `/resume`, e Claude Code o resolve da mesma forma que um nome que você definiu.

<h2 id="use-the-session-picker">
  Use o seletor de sessão
</h2>

Execute `/resume` dentro de uma sessão ou `claude --resume` sem argumentos para abrir o seletor de sessão interativo. Use estes atalhos de teclado para navegar, pesquisar e expandir a lista:

| Atalho                                                    | Ação                                                                                                                                                                      |
| :-------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `↓`                                                 | Navegar entre sessões                                                                                                                                                     |
| `→` / `←`                                                 | Expandir ou recolher sessões agrupadas                                                                                                                                    |
| `Enter`                                                   | Retomar a sessão destacada                                                                                                                                                |
| `Space`                                                   | Visualizar o conteúdo da sessão. `Ctrl+V` também funciona em terminais que não o capturam como colar                                                                      |
| `Ctrl+R`                                                  | Renomear a sessão destacada                                                                                                                                               |
| `/` ou qualquer caractere imprimível diferente de `Space` | Entrar no modo de pesquisa e filtrar sessões. Cole uma URL de pull ou merge request do GitHub, GitHub Enterprise, GitLab ou Bitbucket para encontrar a sessão que a criou |
| `Ctrl+A`                                                  | Mostrar sessões de todos os projetos nesta máquina. Pressione novamente para retornar ao repositório atual                                                                |
| `Ctrl+W`                                                  | Mostrar sessões de todas as worktrees do repositório atual. Pressione novamente para retornar à worktree atual. Mostrado apenas em repositórios com múltiplas worktrees   |
| `Ctrl+B`                                                  | Filtrar para sessões do branch git atual. Pressione novamente para mostrar todos os branches                                                                              |
| `Esc`                                                     | Sair do seletor de sessão ou modo de pesquisa                                                                                                                             |

Cada linha mostra o nome da sessão se você definir um, caso contrário, o título de sessão gerado por IA, resumo da conversa ou primeiro prompt, junto com o tempo desde a última atividade, branch git e tamanho do arquivo. Expanda para todos os projetos com `Ctrl+A` para também ver o caminho do projeto de cada sessão.

As sessões criadas com `/branch` ou `--fork-session` recebem seus próprios IDs de sessão e aparecem como linhas separadas. Quando o seletor encontra mais de uma entrada para a mesma sessão, ele as agrupa sob uma única linha. Pressione `→` para expandir um grupo.

Se Claude Code não conseguir carregar a sessão que você selecionar no seletor `claude --resume`, ele imprime [`Failed to resume the conversation`](/docs/pt/errors#failed-to-resume-the-conversation) com um comando para tentar novamente e sai com código 1. No seletor `/resume` dentro de uma sessão, Claude Code relata a falha e sua conversa atual continua em execução.

<h2 id="branch-a-session">
  Ramificar uma sessão
</h2>

Ramificar cria uma cópia da conversa até agora e o coloca nela, deixando o original intacto. Use-o para tentar uma abordagem diferente sem perder o caminho em que você estava.

De dentro de uma sessão, execute `/branch` com um nome opcional:

```text theme={null}
/branch try-streaming-approach
```

Se você omitir o nome, Claude Code nomeia o novo branch após o primeiro prompt na conversa. A partir da v2.1.198, isso também se aplica após [compaction](/docs/pt/how-claude-code-works#when-context-fills-up); versões anteriores voltavam para o nome literal `Branched conversation` em vez de procurar além do resumo de compaction para o primeiro prompt original.

Na linha de comando, combine `--continue` ou `--resume` com `--fork-session`:

```bash theme={null}
claude --continue --fork-session
```

A confirmação `/branch` imprime dois IDs de sessão: o novo branch em que você está agora e o original. O original permanece inalterado no disco e continua no seletor de sessão; retorne a ele com `/resume <original-name>` ou passando seu ID para `/resume`.

`/branch` copia o transcrição e muda o processo Claude Code em execução para escrever nela. Essa distinção determina o que o branch herda:

| Estado                                                                                                                                                                     | Após `/branch`                                                                                                                                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Histórico de conversa                                                                                                                                                      | Copiado para o branch até o ponto em que você executou `/branch`                                                                                                                                                                      |
| Permissões de concessão "Permitir para esta sessão"                                                                                                                        | Transferidas; o branch é executado no mesmo processo, portanto suas concessões existentes ainda se aplicam. Se você bifurcar em um processo separado com `--fork-session`, o novo processo começa sem elas e você aprova novamente lá |
| [Subagentes em background](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) em voo e [comandos Bash em background](/docs/pt/interactive-mode#background-bash-commands) | Continuam em execução. Sua saída aparece no novo branch para o qual você mudou, não na sessão original                                                                                                                                |
| Conexão [Remote Control](/docs/pt/remote-control)                                                                                                                               | Permanece conectada. Um telefone ou navegador conectado à sessão o segue para o branch e continua recebendo novas mensagens lá                                                                                                        |

Se você retomar a mesma sessão em dois terminais sem bifurcar, as mensagens de ambos se intercalam em um transcrição. Para rewind baseado em checkpoint dentro de uma única sessão, veja [Checkpointing](/docs/pt/checkpointing).

<h2 id="manage-context-within-a-session">
  Gerenciar contexto dentro de uma sessão
</h2>

Estes comandos controlam o que está na janela de contexto sem deixar a sessão:

* **`/clear`**: comece do zero com um contexto vazio. Claude Code salva a conversa anterior; retome-a com `/resume`, ou, no mesmo processo Claude Code, a partir da [entrada de sessão anterior do menu de rewind](/docs/pt/checkpointing#rewind-past-a-cleared-conversation). Sem argumentos, a nova conversa mantém um nome que você definiu com `--name` ou `/rename`, mas não um título de sessão gerado por IA. Para nomear a conversa que você está deixando, passe o nome, como em `/clear release-prep`; a nova conversa então começa sem nome
* **`/compact [instructions]`**: substitua o histórico por um resumo, opcionalmente focado no que você especificar
* **`/context`**: mostrar o que está consumindo contexto atualmente

Para saber como a compactação interage com CLAUDE.md, skills e regras, veja o [guia de janela de contexto](/docs/pt/context-window). Para estratégias sobre quando limpar versus compactar, veja [Melhores práticas](/docs/pt/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Exportar e localizar dados de sessão
</h2>

Execute `/export` para abrir um menu que permite copiar a conversa atual para sua área de transferência ou salvá-la como um arquivo de texto simples, com mensagens e saídas de ferramentas renderizadas como texto legível. Passe um nome de arquivo para pular o menu e escrever diretamente nesse arquivo.

<h3 id="access-conversations-from-scripts">
  Acessar conversas a partir de scripts
</h3>

`/export` produz uma transcrição renderizada para uma pessoa ler. As interfaces abaixo produzem dados estruturados para um script analisar: um resultado JSON de uma execução, o caminho para o arquivo de transcrição de uma sessão, ou um fluxo ao vivo de eventos. Escolha pelo que dispara o script:

* **Executar Claude uma vez e capturar o resultado**: invoque `claude -p` com [`--output-format json` ou `stream-json`](/docs/pt/headless#get-structured-output) para capturar o resultado, ID da sessão, uso e custo de uma execução não interativa como JSON estruturado.
* **Fazer uma pergunta a uma sessão existente**: passe um ID de sessão para [`claude -p --resume`](/docs/pt/headless#continue-conversations) para enviar um prompt de acompanhamento, como uma solicitação de resumo, e capturar a resposta estruturada.
* **Reagir a eventos de sessão**: leia o campo `transcript_path` que [hooks](/docs/pt/hooks#common-input-fields) e [comandos de linha de status](/docs/pt/statusline#available-data) recebem como entrada. Um hook `SessionEnd` pode arquivar a transcrição quando uma sessão termina.
* **Incorporar Claude em um aplicativo TypeScript ou Python**: use o [Agent SDK](/docs/pt/agent-sdk/overview) para receber cada mensagem programaticamente.

O exemplo abaixo usa a segunda interface. Ele envia um prompt de acompanhamento para uma sessão existente e lê a resposta com `jq`:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Onde os transcritos são armazenados
</h3>

Por padrão, Claude Code armazena transcritos como JSONL em `~/.claude/projects/<project>/<session-id>.jsonl`, onde `<project>` é o caminho do seu diretório de trabalho com caracteres não alfanuméricos substituídos por `-`. Para um diretório de trabalho cujo nome convertido excede 200 caracteres, Claude Code trunca o nome para 200 caracteres e anexa um hash do caminho completo, para que o nome do diretório permaneça dentro dos limites do sistema de arquivos.

Cada linha é um objeto JSON para uma mensagem, uso de ferramenta ou entrada de metadados. O formato de entrada é interno ao Claude Code e muda entre versões, portanto scripts que analisam esses arquivos diretamente podem quebrar em qualquer versão. Para construir sobre dados de sessão, use `/export` ou as [interfaces de script](#access-conversations-from-scripts) em vez disso.

A localização, retenção e comportamento de gravação são configuráveis:

| Para                                                                                                                    | Defina                                                                                      | Onde                                                                |
| ----------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Mover armazenamento para fora de `~/.claude`                                                                            | [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars)                                                         | Variável de ambiente                                                |
| [Nomear o diretório `<project>` você mesmo](#name-the-project-directory-yourself)                                       | [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/pt/env-vars)                                              | Variável de ambiente                                                |
| Alterar a retenção de 30 dias                                                                                           | [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays)                             | `settings.json`                                                     |
| Definir um limite de idade para [transcritos do Claude Desktop e Cowork](/docs/pt/claude-directory#cleaned-up-automatically) | [`desktopSessionCleanupPeriodDays`](/docs/pt/settings-reference#desktopsessioncleanupperioddays) | Configurações do usuário, configurações gerenciadas ou `--settings` |
| Suprimir gravações de transcrição em todos os modos                                                                     | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/pt/env-vars)                                           | Variável de ambiente                                                |
| Suprimir gravações para uma execução não interativa                                                                     | [`--no-session-persistence`](/docs/pt/cli-reference)                                             | Sinalizador CLI com `claude -p`                                     |

<h3 id="delete-session-data">
  Excluir dados de sessão
</h3>

Os transcritos envelhecem sob as [regras de varredura de retenção](/docs/pt/claude-directory#cleaned-up-automatically). Para excluir os transcritos de um projeto e o estado relacionado mais cedo, execute [`claude project purge`](/docs/pt/claude-directory#clear-local-data). Se você excluir uma [sessão em segundo plano](/docs/pt/agent-view) com [`claude rm <id>`](/docs/pt/agent-view#what-deleting-a-session-removes), seu transcrição permanece no disco e permanece disponível através de `claude --resume`.

<h3 id="name-the-project-directory-yourself">
  Nomear o diretório do projeto você mesmo
</h3>

Por padrão, Claude Code deriva o nome `<project>` do caminho completo do diretório de trabalho. Para escolher o nome você mesmo, defina `CLAUDE_CODE_PROJECT_DIR_NAME` junto com `CLAUDE_CONFIG_DIR`. Claude Code então armazena os transcritos dessa sessão e [memória automática](/docs/pt/memory#auto-memory) sob seu nome. Isso é adequado para um host que incorpora Claude Code e fornece a cada sessão seu próprio diretório de configuração. Requer Claude Code v2.1.234 ou posterior.

Por exemplo, este lançamento mantém os dados do inquilino A sob `/srv/tenant-a` e nomeia seu diretório de projeto como `work`:

```bash theme={null}
CLAUDE_CONFIG_DIR=/srv/tenant-a CLAUDE_CODE_PROJECT_DIR_NAME=work claude
```

Claude Code escreve os transcritos da sessão para `/srv/tenant-a/projects/work/` e sua memória automática para `/srv/tenant-a/projects/work/memory/`, seja qual for o diretório de trabalho.

Três regras se aplicam quando você o define:

* **Defina `CLAUDE_CONFIG_DIR` também**: o nome não varia com o diretório de trabalho, portanto sob o padrão `~/.claude` ele mesclaria os transcritos e memória automática de cada projeto em um diretório. Claude Code ignora `CLAUDE_CODE_PROJECT_DIR_NAME` quando `CLAUDE_CONFIG_DIR` não está definido.
* **Use 1-64 letras, dígitos, hífens ou sublinhados**: não use um nome de dispositivo Windows como `con`. Claude Code ignora qualquer outro valor e usa o nome derivado.
* **Defina-o no ambiente shell que inicia `claude`**: Claude Code o lê uma vez na inicialização desse ambiente, portanto um bloco `env` em um arquivo de configurações não pode defini-lo.

Depois de nomear o diretório do projeto de um diretório de configuração, continue lançando com esse nome. Se você iniciar Claude Code com o mesmo `CLAUDE_CONFIG_DIR` mas sem `CLAUDE_CODE_PROJECT_DIR_NAME`, ele lê e escreve o diretório derivado novamente. As sessões armazenadas sob seu nome permanecem no disco: pressione `Ctrl+A` no [seletor de sessão](#use-the-session-picker) para listar sessões de cada diretório de projeto sob esse diretório de configuração, o fixado incluído, e de qualquer forma que você lance, [`claude --resume <session-id>`](#resume-a-session) encontra uma sessão armazenada sob qualquer um dos nomes.

<h2 id="see-also">
  Veja também
</h2>

Estas páginas cobrem mecânicas relacionadas de sessão e paralelismo:

* [Worktrees](/docs/pt/worktrees): execute sessões paralelas isoladas em branches separados
* [Checkpointing](/docs/pt/checkpointing): retroceda código e conversa para um ponto anterior
* [Janela de contexto](/docs/pt/context-window): o que preenche o contexto e o que sobrevive à compactação
* [Modo não interativo](/docs/pt/headless): comportamento de sessão sob `claude -p`
