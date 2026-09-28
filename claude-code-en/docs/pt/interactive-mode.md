> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modo interativo

> Referência completa para atalhos de teclado, modos de entrada e recursos interativos em sessões do Claude Code.

<h2 id="keyboard-shortcuts">
  Atalhos de teclado
</h2>

<Note>
  Os atalhos de teclado podem variar por plataforma e terminal. Na [renderização em tela cheia](/docs/pt/fullscreen), pressione `?` no visualizador de transcrição para ver os atalhos disponíveis lá.

  **Usuários de macOS**: Os atalhos da tecla Option/Alt (`Alt+B`, `Alt+F`, `Alt+D`, `Alt+Y`, `Alt+P`) exigem configurar Option como Meta no seu terminal. Consulte [Ativar atalhos da tecla Option no macOS](/docs/pt/terminal-config#enable-option-key-shortcuts-on-macos) para a configuração em cada terminal.
</Note>

<h3 id="general-controls">
  Controles gerais
</h3>

| Atalho                                                                                         | Descrição                                                                                                                                                                                                                                                                                        | Contexto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :--------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                                                                       | Interromper ou limpar entrada                                                                                                                                                                                                                                                                    | Interrompe uma operação em execução. Se nada estiver em execução, o primeiro pressionamento limpa a entrada do prompt e um segundo pressionamento sai do Claude Code                                                                                                                                                                                                                                                                                                                                                                               |
| `Ctrl+X Ctrl+K`                                                                                | Parar todos os [subagentes em segundo plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) nesta sessão e desativar [respostas automáticas de artefatos](/docs/pt/artifacts#let-claude-reply-to-comments-on-its-own) para o resto dela. Pressione duas vezes em 3 segundos para confirmar | Controle de subagente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `Ctrl+D`                                                                                       | Sair da sessão do Claude Code                                                                                                                                                                                                                                                                    | O primeiro pressionamento mostra uma dica de confirmação e um segundo pressionamento em 800ms sai. Quando o prompt tem texto, `Ctrl+D` deleta o caractere após o cursor                                                                                                                                                                                                                                                                                                                                                                            |
| `Ctrl+G` ou `Ctrl+X Ctrl+E`                                                                    | Abrir no editor de texto padrão                                                                                                                                                                                                                                                                  | Edite seu prompt ou resposta personalizada no seu editor de texto padrão. `Ctrl+X Ctrl+E` é a vinculação nativa do readline. Ative **Mostrar última resposta no editor externo** em `/config` para adicionar a resposta anterior do Claude como contexto comentado com `#` acima do seu prompt; Claude Code remove o bloco de comentário quando você salva                                                                                                                                                                                         |
| `Ctrl+L`                                                                                       | Redesenhar a tela                                                                                                                                                                                                                                                                                | Força um redesenho completo do terminal, mantendo a entrada e o histórico de conversa. Use isso para recuperar se a exibição ficar corrompida ou parcialmente em branco. Consulte [Limpar a conversa](/docs/pt/fullscreen#clear-the-conversation) para renderização em tela cheia                                                                                                                                                                                                                                                                       |
| `Ctrl+O`                                                                                       | Alternar visualizador de transcrição                                                                                                                                                                                                                                                             | Mostra uso detalhado de ferramentas e execução, com um timestamp e o modelo usado em cada mensagem do assistente. Também expande linhas que são recolhidas por padrão, como chamadas MCP, mostradas como uma única linha `Called slack 3 times`, e [mensagens de suas outras sessões](/docs/pt/cross-session-messaging#what-a-message-looks-like), mostradas como uma visualização de uma linha `Message from @<sender>`                                                                                                                                |
| `Ctrl+R`                                                                                       | Pesquisa reversa do histórico de comandos                                                                                                                                                                                                                                                        | Pesquise através de comandos anteriores interativamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `Ctrl+V` ou `Cmd+V` (iTerm2) ou `Alt+V` (Windows e WSL)                                        | Colar imagem da área de transferência                                                                                                                                                                                                                                                            | Insere um chip `[Image #N]` no cursor para que você possa referenciá-lo posicionalmente no seu prompt. No WSL, tanto `Ctrl+V` quanto `Alt+V` estão vinculados; use `Alt+V` se seu terminal interceptar `Ctrl+V`                                                                                                                                                                                                                                                                                                                                    |
| `Ctrl+B`                                                                                       | Tarefas em execução em segundo plano                                                                                                                                                                                                                                                             | Coloca comandos Bash e agentes em segundo plano. Usuários de Tmux pressionam duas vezes                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `Ctrl+T`                                                                                       | Alternar lista de tarefas do Claude                                                                                                                                                                                                                                                              | Mostrar ou ocultar [lista de tarefas do Claude](#task-list) na área de status. Esta não é a visualização de tarefas em segundo plano; use [`/tasks`](/docs/pt/commands) para ver shells e subagentes em execução                                                                                                                                                                                                                                                                                                                                        |
| `Ctrl+S`                                                                                       | Guardar ou restaurar prompt                                                                                                                                                                                                                                                                      | Com texto na entrada, guarda-o e limpa o prompt. Pressionado novamente em um prompt vazio, restaura o texto guardado, posição do cursor, conteúdo colado e modo de entrada, para que um `!` guardado [comando shell](#shell-mode-with-prefix) volte em modo shell                                                                                                                                                                                                                                                                                  |
| `Ctrl+Z`                                                                                       | Suspender Claude Code                                                                                                                                                                                                                                                                            | Apenas Unix. Suspende o processo para seu shell; execute `fg` para retomar                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `Setas Esquerda/Direita`                                                                       | Ciclar através de abas de diálogo                                                                                                                                                                                                                                                                | Navegue entre abas em diálogos de permissão e menus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `Tab`                                                                                          | Aceitar uma sugestão de preenchimento automático ou adicionar um comentário a uma resposta de permissão                                                                                                                                                                                          | Enquanto as sugestões de preenchimento automático estão sendo mostradas na entrada do prompt, aceita a sugestão selecionada. Na maioria dos prompts de permissão, com **Sim** ou **Não** focado, abre um campo de comentário nessa opção, e pressioná-lo novamente fecha o campo. Consulte [adicionar um comentário quando você responde a um prompt de permissão](/docs/pt/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                                              |
| `Setas Para Cima/Para Baixo` ou `Ctrl+P`/`Ctrl+N`                                              | Mover cursor ou navegar no histórico de comandos                                                                                                                                                                                                                                                 | Quando a entrada abrange mais de uma linha visual, seja envolvida ou multilinha, primeiro move o cursor dentro do prompt. Uma vez que o cursor está na primeira ou última linha visual, pressioná-lo novamente navega no histórico de comandos. Enquanto você tem mensagens enfileiradas, `Para Cima` da primeira linha em vez disso [as retira](#take-back-what-you-queued)                                                                                                                                                                       |
| `Esc`                                                                                          | Interromper Claude ou fechar um diálogo                                                                                                                                                                                                                                                          | Pare a resposta atual ou chamada de ferramenta no meio da volta para que você possa redirecionar. Claude mantém o trabalho feito até agora. Se você tiver [mensagens enfileiradas](#queue-messages-while-claude-works), Claude Code as envia a seguir. Quando um diálogo está aberto, `Esc` fecha o diálogo. Em um prompt de permissão, `Esc` recusa a ação, o mesmo que [**Não** sem um comentário](/docs/pt/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                            |
| `Esc` + `Esc`                                                                                  | Limpar rascunho de entrada ou retroceder                                                                                                                                                                                                                                                         | Quando a entrada do prompt contém texto, duplo `Esc` limpa-o e salva o rascunho no histórico para que `Para Cima` o recupere. Quando a entrada está vazia, duplo `Esc` abre o [menu de retrocesso](/docs/pt/checkpointing) para restaurar ou resumir código e conversa de um ponto anterior                                                                                                                                                                                                                                                             |
| `Ctrl+Enter` ou `Ctrl+X Ctrl+S`                                                                | Enviar mensagens enfileiradas agora                                                                                                                                                                                                                                                              | Envia suas [mensagens enfileiradas](#queue-messages-while-claude-works) e seu rascunho com elas imediatamente. [Quando Claude Code envia o que você enfileirou](#when-claude-code-sends-what-you-queued) cobre o que acontece com a volta em que Claude está trabalhando. No [modo shell](#shell-mode-with-prefix), a tecla apenas enfileira seu comando. Em terminais que não relatam chaves estendidas, `Ctrl+Enter` chega como `Enter` simples; `Ctrl+X Ctrl+S` funciona em qualquer terminal. Requer Claude Code v2.1.275 ou posterior         |
| `Shift+Tab`, ou `Alt+M` no Windows quando o runtime Node ou Bun não ativa o modo de entrada VT | Ciclar modos de permissão                                                                                                                                                                                                                                                                        | Cicle através de `default` (rotulado Manual no indicador de modo), `acceptEdits`, `plan` e, quando disponível, `bypassPermissions` e depois `auto`. De `auto`, o primeiro pressionamento muda para `default`. Consulte [modos de permissão](/docs/pt/permission-modes). Em um prompt de permissão de arquivo, a mesma tecla fecha um [campo de comentário](/docs/pt/permissions#add-a-comment-when-you-answer-a-permission-prompt) aberto. Sem campo aberto, seleciona a opção que permite a ação para o resto da sessão, quando o prompt oferece essa opção |
| `Option+P` (macOS) ou `Alt+P` (Windows/Linux)                                                  | Alternar modelo                                                                                                                                                                                                                                                                                  | Alterne modelos sem limpar seu prompt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `Option+T` (macOS) ou `Alt+T` (Windows/Linux)                                                  | Alternar pensamento estendido                                                                                                                                                                                                                                                                    | Ativar ou desativar o modo de pensamento estendido. Não tem efeito no Opus 5.5 ou nos modelos Fable, que sempre usam pensamento estendido. Funciona no macOS sem configurar Option como Meta                                                                                                                                                                                                                                                                                                                                                       |
| `Option+O` (macOS) ou `Alt+O` (Windows/Linux)                                                  | Alternar modo rápido                                                                                                                                                                                                                                                                             | Ativar ou desativar [modo rápido](/docs/pt/fast-mode)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

<h3 id="text-editing">
  Edição de texto
</h3>

| Atalho                     | Descrição                                        | Contexto                                                                                                                                                                                                       |
| :------------------------- | :----------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                   | Mover cursor para o início da linha atual        | Em entrada multilinha, move para o início da linha lógica atual                                                                                                                                                |
| `Ctrl+E`                   | Mover cursor para o final da linha atual         | Em entrada multilinha, move para o final da linha lógica atual                                                                                                                                                 |
| `Ctrl+K`                   | Deletar até o final da linha                     | Armazena texto deletado para colagem                                                                                                                                                                           |
| `Ctrl+U`                   | Deletar do cursor até o início da linha          | Armazena texto deletado para colagem. Repita para limpar entre linhas em entrada multilinha. No macOS, emuladores de terminal incluindo iTerm2 e Terminal.app mapeiam `Cmd+Backspace` para este atalho         |
| `Ctrl+W`                   | Deletar de volta até o espaço em branco anterior | Armazena texto deletado para colagem. Um pressionamento remove um caminho inteiro ou `--flag=value`. Para deletar apenas a palavra anterior, pressione `Option+Delete` no macOS ou `Ctrl+Backspace` no Windows |
| `Ctrl+Y`                   | Colar texto deletado                             | Cola o texto que você deletou por último com um dos atalhos de deleção de palavra ou linha, como `Ctrl+K`, `Ctrl+U` ou `Ctrl+W`                                                                                |
| `Alt+Y` (após `Ctrl+Y`)    | Ciclar histórico de colagem                      | Após colar, cicle através do texto deletado anteriormente. Requer [Option como Meta](#keyboard-shortcuts) no macOS                                                                                             |
| `Alt+B`                    | Mover cursor de volta uma palavra                | Navegação de palavra. Requer [Option como Meta](#keyboard-shortcuts) no macOS                                                                                                                                  |
| `Alt+F`                    | Mover cursor para frente uma palavra             | Move para o final da palavra atual ou para o final da próxima palavra quando o cursor está entre palavras. Requer [Option como Meta](#keyboard-shortcuts) no macOS                                             |
| `Alt+D`                    | Deletar até o final da palavra                   | Deleta até o final da palavra atual ou até o final da próxima palavra quando o cursor está entre palavras. Armazena texto deletado para colagem. Requer [Option como Meta](#keyboard-shortcuts) no macOS       |
| `Ctrl+_` ou `Ctrl+Shift+-` | Desfazer última edição de entrada                | Restaura o texto de entrada anterior e a posição do cursor                                                                                                                                                     |

<h3 id="make-ctrl-w-delete-back-to-whitespace">
  Limites de palavra em atalhos de edição
</h3>

Os atalhos de palavra `Alt+B`, `Alt+F`, `Alt+D`, `Option+Delete` e `Ctrl+Backspace` tratam uma palavra como uma sequência de letras e dígitos, então pontuação como `_`, `.` e `/` separa palavras. Com `src/utils/foo.ts` no prompt, pressionamentos repetidos de `Alt+B` param no início de `ts`, `foo`, `utils` e `src`.

`Ctrl+W` é diferente: ignora pontuação e deleta de volta até o espaço em branco anterior, então um pressionamento remove tudo de `src/utils/foo.ts`.

Em texto escrito sem espaços, como chinês ou japonês, os atalhos de palavra ainda se movem ou deletam uma palavra por vez.

Essas convenções readline se aplicam no Claude Code v2.1.261 e posterior. A configuração [`keybindingFlavor`](/docs/pt/settings-reference#keybindingflavor) que as ativou em versões anteriores está descontinuada e não tem efeito.

Você não pode remapear esses atalhos no [arquivo de configuração de atalhos de teclado](/docs/pt/keybindings), que não tem ações para eles.

<h3 id="theme-and-display">
  Tema e exibição
</h3>

| Atalho   | Descrição                                          | Contexto                                                                                                                  |
| :------- | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `Ctrl+T` | Alternar destaque de sintaxe para blocos de código | Funciona apenas dentro do menu do seletor `/theme`. Controla se o código nas respostas do Claude usa coloração de sintaxe |

<h3 id="multiline-input">
  Entrada multilinha
</h3>

| Método                | Atalho            | Contexto                                                                                                                                                                                     |
| :-------------------- | :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Escape rápido         | `\` + `Enter`     | Funciona em todos os terminais                                                                                                                                                               |
| Tecla Option          | `Option+Enter`    | Após ativar [Option como Meta](/docs/pt/terminal-config#enable-option-key-shortcuts-on-macos) no macOS                                                                                            |
| Shift+Enter           | `Shift+Enter`     | Nativo em iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal. Para outros terminais, consulte [Inserir prompts multilinha](/docs/pt/terminal-config#enter-multiline-prompts) |
| Sequência de controle | `Ctrl+J`          | Funciona em qualquer terminal sem configuração                                                                                                                                               |
| Modo de colagem       | Colar diretamente | Para blocos de código, logs                                                                                                                                                                  |

<h3 id="quick-commands">
  Comandos rápidos
</h3>

| Atalho               | Descrição                          | Notas                                                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------- | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/` no início        | Comando ou skill                   | Consulte [comandos](#commands) e [skills](/docs/pt/skills)                                                                                                                                                                                                                                                                                                                                                         |
| `!` no início        | Modo shell                         | Execute um comando diretamente, adicione sua saída à sessão e faça Claude responder a ela                                                                                                                                                                                                                                                                                                                     |
| `@`                  | Menção de caminho de arquivo       | Ativar preenchimento automático de caminho de arquivo. Em sessões com [mensagens entre sessões](/docs/pt/cross-session-messaging#message-another-session), quando você digita pelo menos uma letra após o `@`, Claude Code também sugere suas outras sessões ativas nesta máquina, para que você possa dizer ao Claude para enviar uma mensagem para a que você escolher. Requer Claude Code v2.1.232 ou posterior |
| `:`                  | Código de emoji                    | Digite um `:name:` completo para inserir o emoji ou dois ou mais caracteres para sugestões. Consulte [Códigos de emoji](#emoji-shortcodes). Requer Claude Code v2.1.217 ou posterior                                                                                                                                                                                                                          |
| `?` em entrada vazia | Alternar painel de ajuda de atalho | Digitar `?` quando a entrada já contém texto insere o caractere                                                                                                                                                                                                                                                                                                                                               |

<h3 id="transcript-viewer">
  Visualizador de transcrição
</h3>

Quando o visualizador de transcrição está aberto (alternado com `Ctrl+O`), esses atalhos estão disponíveis. Execute `/tui` sem argumento para verificar qual renderizador está ativo. `Ctrl+E` pode ser remapeado via [`transcript:toggleShowAll`](/docs/pt/keybindings).

| Atalho               | Descrição                                                                                                                                                                                                                                          |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Alternar o painel de ajuda de atalho de teclado. Requer [renderização em tela cheia](/docs/pt/fullscreen)                                                                                                                                               |
| `{` / `}`            | Pular para o prompt do usuário anterior ou próximo, como movimento de parágrafo vim. Requer [renderização em tela cheia](/docs/pt/fullscreen)                                                                                                           |
| `Ctrl+E`             | Alternar mostrar todo o conteúdo. Disponível apenas no renderizador clássico, não na [renderização em tela cheia](/docs/pt/fullscreen)                                                                                                                  |
| `[`                  | Escrever a conversa completa para o scrollback nativo do seu terminal para que `Cmd+F`, modo de cópia tmux e outras ferramentas nativas possam pesquisá-lo. Requer [renderização em tela cheia](/docs/pt/fullscreen#search-and-review-the-conversation) |
| `v`                  | Escrever a conversa para um arquivo temporário e abri-lo em `$VISUAL` ou `$EDITOR`. Requer [renderização em tela cheia](/docs/pt/fullscreen)                                                                                                            |
| `q`, `Ctrl+C`, `Esc` | Sair da visualização de transcrição. Todos os três podem ser remapeados via [`transcript:exit`](/docs/pt/keybindings)                                                                                                                                   |

<h3 id="voice-input">
  Entrada de voz
</h3>

| Atalho                  | Descrição     | Notas                                                                                                                                                                                                  |
| :---------------------- | :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Manter ou tocar `Space` | Ditado de voz | Requer [ditado de voz](/docs/pt/voice-dictation) estar ativado. Mantenha pressionado para gravar ou execute `/voice tap` para alternar ao tocar. [Remapeável](/docs/pt/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Comandos
</h2>

Digite `/` no Claude Code para ver os comandos disponíveis para você, ou digite `/` seguido de qualquer letra para filtrar. O menu `/` lista comandos integrados, [skills](/docs/pt/skills) agrupadas e criadas por usuários, e comandos contribuídos por [plugins](/docs/pt/plugins/overview) e [servidores MCP](/docs/pt/mcp#use-mcp-prompts-as-commands). Nem todos os comandos integrados são visíveis para todos os usuários, pois alguns dependem da sua plataforma ou plano, e [alguns comandos disponíveis estão ocultos do menu por design](/docs/pt/commands#how-the-command-menu-matches-what-you-type) e são executados quando você digita seu nome completo.

Na [renderização em tela cheia](/docs/pt/fullscreen#use-the-mouse), o comando `/` e as listas de sugestão de arquivo `@` também respondem ao mouse: passar o mouse destaca uma linha e clicar a aceita.

Consulte a [referência de comandos](/docs/pt/commands) para obter a lista completa de comandos incluídos no Claude Code.

<h3 id="complete-a-command-mid-prompt">
  Completar um comando no meio do prompt
</h3>

A conclusão de comando também funciona no meio de um prompt: digite `/` após um espaço, depois as primeiras letras de um nome, como em `executar os testes, depois /com`. Apenas comandos cujos nomes começam com essas letras correspondem, portanto um caminho de arquivo como `/tmp/notes.md` não mantém uma lista aberta. Claude Code executa um comando apenas quando o comando [inicia sua mensagem](/docs/pt/commands).

* **Na [renderização em tela cheia](/docs/pt/fullscreen)**: as correspondências aparecem como uma lista enquanto você digita, sem nenhuma linha destacada, portanto `Enter` ainda envia seu prompt conforme digitado. Pressione `Tab` para inserir a correspondência superior, ou escolha uma linha com as setas e `Enter`.
* **Fora da tela cheia**: o resto da correspondência superior aparece como texto fantasma no seu cursor, com uma contagem como `+2` quando mais comandos correspondem. Pressione `Tab` para inserir a única correspondência, ou para abrir a lista quando várias correspondem, depois escolha uma linha com as setas e `Enter`.

Em ambos os renderizadores, pressione `Tab` em um `/` nu no meio do prompt para listar todos os comandos.

Uma skill de plugin corresponde também ao seu nome nu, portanto `/deploy` encontra uma skill nomeada `myplugin:deploy-app`. Quando você insere a correspondência, Claude Code escreve o `/myplugin:deploy-app` completo.

<h2 id="vim-editor-mode">
  Modo editor Vim
</h2>

Ative a edição no estilo vim via `/config` → Editor mode.

Claude Code mantém seu modo vim e posição do cursor quando você alterna o [visualizador de transcrição](#transcript-viewer) com `Ctrl+O` ou abre e fecha um painel como `/config`. Se você deixar o prompt em modo NORMAL, ele ainda estará em modo NORMAL quando você retornar, com o cursor onde você o deixou.

<h3 id="mode-switching">
  Alternância de modo
</h3>

| Comando           | Ação                                                                                                             | Do modo        |
| :---------------- | :--------------------------------------------------------------------------------------------------------------- | :------------- |
| `Esc` ou `Ctrl+[` | Entrar no modo NORMAL. Em terminais que usam o protocolo de teclado Kitty, `Ctrl+[` requer v2.1.242 ou posterior | INSERT, VISUAL |
| `i`               | Inserir antes do cursor                                                                                          | NORMAL         |
| `I`               | Inserir no início da linha                                                                                       | NORMAL         |
| `a`               | Inserir após o cursor                                                                                            | NORMAL         |
| `A`               | Inserir no final da linha                                                                                        | NORMAL         |
| `o`               | Abrir linha abaixo                                                                                               | NORMAL         |
| `O`               | Abrir linha acima                                                                                                | NORMAL         |
| `v`               | Iniciar seleção visual por caractere                                                                             | NORMAL         |
| `V`               | Iniciar seleção visual por linha                                                                                 | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Remapear sequências de teclas no modo INSERT
</h3>

A configuração [`vimInsertModeRemaps`](/docs/pt/settings-reference#viminsertmoderemaps) mapeia uma sequência de dois caracteres no modo INSERT para Escape, então um mapeamento como `jj` o retorna ao modo NORMAL. Requer Claude Code v2.1.208 ou posterior.

O exemplo `~/.claude/settings.json` a seguir ativa o modo vim e mapeia `jj` para Escape:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Cada chave é exatamente dois caracteres imprimíveis digitados em sequência, e `"<Esc>"` é o único alvo suportado. Entradas com um comprimento ou alvo diferente são ignoradas.

Digitar o primeiro caractere de uma sequência o insere normalmente. Pressionar o segundo caractere dentro de um segundo remove esse caractere pendente e muda para o modo NORMAL, deixando nenhum caractere em sua entrada. Após a janela de um segundo, ou se uma chave diferente seguir, ambos os caracteres permanecem como texto literal, então você ainda pode digitar uma palavra contendo a sequência fazendo uma pausa entre os dois caracteres.

Claude Code lê essa configuração do seu arquivo de configurações do usuário, da flag `--settings` e de [configurações gerenciadas](/docs/pt/managed-settings) apenas. Entradas no `.claude/settings.json` ou `.claude/settings.local.json` de um projeto são ignoradas, então um repositório verificado não pode remapear seus pressionamentos de tecla.

<h3 id="navigation-normal-mode">
  Navegação (modo NORMAL)
</h3>

| Comando         | Ação                                                                                                                                                            |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Mover esquerda/baixo/cima/direita                                                                                                                               |
| `Space`         | Mover direita                                                                                                                                                   |
| `w`             | Próxima palavra                                                                                                                                                 |
| `e`             | Fim da palavra                                                                                                                                                  |
| `b`             | Palavra anterior                                                                                                                                                |
| `0`             | Início da linha                                                                                                                                                 |
| `$`             | Fim da linha                                                                                                                                                    |
| `^`             | Primeiro caractere não em branco                                                                                                                                |
| `gg`            | Início da entrada                                                                                                                                               |
| `G`             | Fim da entrada                                                                                                                                                  |
| `f{char}`       | Pular para a próxima ocorrência do caractere                                                                                                                    |
| `F{char}`       | Pular para a ocorrência anterior do caractere                                                                                                                   |
| `t{char}`       | Pular para logo antes da próxima ocorrência do caractere                                                                                                        |
| `T{char}`       | Pular para logo após a ocorrência anterior do caractere                                                                                                         |
| `;`             | Repetir o último movimento f/F/t/T                                                                                                                              |
| `,`             | Repetir o último movimento f/F/t/T em ordem inversa                                                                                                             |
| `/`             | Abrir busca de histórico reverso, igual a `Ctrl+R`. O prompt de busca vazio mostra uma dica: pressione `Esc` depois `i` depois `/` para abrir o menu de comando |

<Note>
  No modo NORMAL do vim, se o cursor estiver no início ou fim da entrada e não puder se mover mais, `j`/`k` e `↑`/`↓` navegam pelo histórico de comandos. `←` em um prompt vazio abre a [visualização de agente](/docs/pt/agent-view) do modo NORMAL assim como INSERT; antes de v2.1.219, `←` em um prompt vazio não fazia nada no modo NORMAL.
</Note>

<h3 id="editing-normal-mode">
  Edição (modo NORMAL)
</h3>

| Comando               | Ação                                                                                                                     |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| `x`                   | Deletar caractere                                                                                                        |
| `dd`                  | Deletar linha                                                                                                            |
| `D`                   | Deletar até o fim da linha                                                                                               |
| `dw`/`de`/`db`        | Deletar palavra/até o fim/para trás                                                                                      |
| `df{char}`/`dt{char}` | Deletar até e incluindo, ou até, a próxima ocorrência de um caractere                                                    |
| `cc`                  | Mudar linha                                                                                                              |
| `C`                   | Mudar até o fim da linha                                                                                                 |
| `cw`/`ce`/`cb`        | Mudar palavra/até o fim/para trás                                                                                        |
| `s`                   | Substituir caractere: deletar o caractere sob o cursor e entrar no modo INSERT. Requer Claude Code v2.1.211 ou posterior |
| `S`                   | Substituir linha: limpar a linha e entrar no modo INSERT. Requer Claude Code v2.1.211 ou posterior                       |
| `yy`/`Y`              | Yankar (copiar) linha                                                                                                    |
| `yw`/`ye`/`yb`        | Yankar palavra/até o fim/para trás                                                                                       |
| `p`                   | Colar após o cursor                                                                                                      |
| `P`                   | Colar antes do cursor                                                                                                    |
| `>>`                  | Indentar linha                                                                                                           |
| `<<`                  | Desindentação de linha                                                                                                   |
| `J`                   | Juntar linhas                                                                                                            |
| `u`                   | Desfazer                                                                                                                 |
| `.`                   | Repetir última mudança                                                                                                   |

<h3 id="text-objects-normal-mode">
  Objetos de texto (modo NORMAL)
</h3>

Objetos de texto funcionam com operadores como `d`, `c` e `y`:

| Comando   | Ação                                                       |
| :-------- | :--------------------------------------------------------- |
| `iw`/`aw` | Palavra interna/ao redor                                   |
| `iW`/`aW` | PALAVRA interna/ao redor (delimitada por espaço em branco) |
| `i"`/`a"` | Aspas duplas internas/ao redor                             |
| `i'`/`a'` | Aspas simples internas/ao redor                            |
| `i(`/`a(` | Parênteses internos/ao redor                               |
| `i[`/`a[` | Colchetes internos/ao redor                                |
| `i{`/`a{` | Chaves internas/ao redor                                   |

<h3 id="visual-mode">
  Modo visual
</h3>

Pressione `v` para seleção por caractere ou `V` para seleção por linha. Os movimentos estendem a seleção e os operadores atuam sobre ela diretamente.

| Comando          | Ação                                                      |
| :--------------- | :-------------------------------------------------------- |
| `d`/`x`          | Deletar seleção                                           |
| `y`              | Yankar seleção                                            |
| `c`/`s`          | Mudar seleção                                             |
| `p`              | Substituir seleção pelo conteúdo do registro              |
| `r{char}`        | Substituir cada caractere selecionado por `{char}`        |
| `~`/`u`/`U`      | Alternar, minúsculas ou maiúsculas na seleção             |
| `>`/`<`          | Indentar ou desindentação de linhas selecionadas          |
| `J`              | Juntar linhas selecionadas                                |
| `o`              | Trocar cursor e âncora                                    |
| `iw`/`aw`/`i"`/… | Selecionar um objeto de texto                             |
| `v`/`V`          | Alternar entre seleção por caractere e por linha, ou sair |

O modo visual por bloco com `Ctrl+V` não é suportado.

<h2 id="command-history">
  Histórico de comandos
</h2>

Claude Code mantém um histórico dos prompts que você digita, e a recuperação com seta para cima alcança prompts de sessões anteriores do mesmo projeto:

* O histórico de entrada é armazenado por diretório de trabalho
* Executar `/clear` inicia uma nova sessão: a recuperação então lista os prompts da nova sessão primeiro, com os prompts de sessões anteriores depois deles. A conversa da sessão anterior é preservada e pode ser retomada.
* Enviar o mesmo prompt duas vezes seguidas registra uma entrada de histórico, então pressionar Seta para cima vai para o prompt anterior distinto
* Quando você recupera um prompt que incluía texto colado, Claude Code envia o conteúdo colado completo novamente quando você reenvia. Se o conteúdo foi [limpo](/docs/pt/claude-directory#cleaned-up-automatically), Claude Code não envia a string literal `[Pasted text #N]`; veja [Colar conteúdo grande](/docs/pt/terminal-config#paste-large-content) para o que acontece com o prompt
* A expansão de histórico com `!` está desabilitada por padrão

<h3 id="reverse-search-with-ctrl-r">
  Busca reversa com Ctrl+R
</h3>

Pressione `Ctrl+R` para pesquisar interativamente através do seu histórico de comandos. Na [renderização em tela cheia](/docs/pt/fullscreen), `Ctrl+R` abre um diálogo de pesquisa: digite para filtrar, pressione `Seta para cima` e `Seta para baixo` para se mover através das correspondências, e pressione `Ctrl+S` para alternar o escopo através desta sessão, este projeto e todos os projetos. Pressione `Enter` ou `Tab` para colocar uma correspondência na entrada do prompt, ou `Esc` para cancelar. Os passos abaixo descrevem a pesquisa inline do renderizador clássico:

1. **Iniciar pesquisa**: pressione `Ctrl+R` para ativar a busca reversa de histórico
2. **Digite a consulta**: insira o texto a ser pesquisado em comandos anteriores. O termo de pesquisa é destacado nos resultados correspondentes
3. **Navegue pelas correspondências**: pressione `Ctrl+R` novamente para alternar entre correspondências mais antigas
4. **Escopo de pesquisa**: a pesquisa inline sempre pesquisa prompts de todos os projetos
5. **Aceitar correspondência**:
   * Pressione `Tab` ou `Esc` para aceitar a correspondência atual e continuar editando
   * Pressione `Enter` para aceitar e executar o comando imediatamente
6. **Cancelar pesquisa**:
   * Pressione `Ctrl+C` para cancelar e restaurar sua entrada original
   * Pressione `Backspace` em pesquisa vazia para cancelar

A pesquisa inline verifica seu histórico de prompt completo, mais recente primeiro, com duplicatas recolhidas para a ocorrência mais recente. O diálogo em tela cheia pesquisa todo o seu histórico de prompt no escopo selecionado, mais recente primeiro, com duplicatas recolhidas para a ocorrência mais recente: os prompts mais recentes aparecem imediatamente, e correspondências de prompts mais antigos preenchem conforme Claude Code carrega o resto. Os prompts correspondentes são exibidos com o termo de pesquisa destacado, para que você possa encontrar e reutilizar entradas anteriores.

Aceitar uma correspondência ou cancelar a pesquisa entra em efeito imediatamente, mesmo enquanto Claude Code ainda está carregando o histórico.

<h2 id="background-bash-commands">
  Comandos Bash em segundo plano
</h2>

Claude Code suporta a execução de comandos Bash em segundo plano, permitindo que você continue trabalhando enquanto processos de longa duração são executados.

<h3 id="how-backgrounding-works">
  Como o segundo plano funciona
</h3>

Quando Claude Code executa um comando em segundo plano, ele executa o comando de forma assíncrona e retorna imediatamente um ID de tarefa em segundo plano. Claude Code pode responder a novos prompts enquanto o comando continua sendo executado em segundo plano.

Para executar comandos em segundo plano, você pode:

* Solicitar ao Claude Code que execute um comando em segundo plano
* Pressionar `Ctrl+B` para mover uma invocação regular da ferramenta Bash para o segundo plano. Usuários de Tmux devem pressionar `Ctrl+B` duas vezes devido à chave de prefixo do tmux.

**Recursos principais:**

* A saída é escrita em um arquivo e Claude pode recuperá-la usando a ferramenta Read
* As tarefas em segundo plano têm IDs exclusivos para rastreamento e recuperação de saída
* As tarefas em segundo plano são limpas automaticamente quando Claude Code sai. No macOS e Linux, quando você interrompe uma tarefa em segundo plano de [`/tasks`](/docs/pt/commands) ou Claude Code a interrompe ao sair, os processos que se desvincularam do shell da tarefa, como aqueles iniciados sob `setsid` ou `timeout`, também param
* Se você colocar a sessão em segundo plano em vez de sair dela, suas tarefas em segundo plano continuarão sendo executadas na sessão em segundo plano. Veja [colocar uma sessão em execução em segundo plano](/docs/pt/agent-view#from-inside-a-session)
* As tarefas em segundo plano são automaticamente encerradas se a saída exceder 5GB, com uma nota em stderr explicando o motivo
* No macOS e Linux, Claude Code encerra as tarefas em segundo plano em execução quando o sistema operacional sinaliza pressão de memória crítica, desde que a sessão tenha ficado ociosa por pelo menos 30 minutos e nenhuma volta ou subagente esteja em execução. Requer Claude Code v2.1.193 ou posterior
  * O [log de depuração](/docs/pt/debug-your-config) diz por que as tarefas foram interrompidas, ou por que um evento de pressão as deixou em execução
  * Defina [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/pt/env-vars) como `1` para desativar paradas por pressão de memória
* Comandos em segundo plano pertencentes a um [subagente](/docs/pt/sub-agents) não têm limite de tempo, exceto que um comando pertencente a um subagente em execução em primeiro plano termina quando esse subagente fornece sua resposta final; veja [Comandos em segundo plano](/docs/pt/tools-reference#background-commands) na referência de ferramentas. Antes da v2.1.218, nem a recolha de pressão de memória nem o limite anterior de 60 minutos em comandos de subagente cobriam comandos movidos para o segundo plano com `Ctrl+B`

Para desabilitar toda a funcionalidade de tarefas em segundo plano, defina a variável de ambiente `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` como `1`. Veja [Variáveis de ambiente](/docs/pt/env-vars) para detalhes.

**Comandos comuns colocados em segundo plano:**

* Ferramentas de compilação (webpack, vite, make)
* Gerenciadores de pacotes (npm, yarn, pnpm)
* Executores de testes (jest, pytest)
* Servidores de desenvolvimento
* Processos de longa duração (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Modo shell com prefixo `!`
</h3>

Execute comandos shell diretamente sem passar por Claude prefixando sua entrada com `!`:

```bash theme={null}
! npm test
! git status
! ls -la
```

Modo shell:

* Adiciona o comando e sua saída ao contexto da conversa
* Mostra progresso e saída em tempo real
* Suporta o mesmo backgrounding `Ctrl+B` para comandos de longa duração
* Não requer que Claude interprete ou aprove o comando
* Suporta preenchimento automático baseado em histórico: digite um comando parcial e pressione `Tab` para completar a partir de comandos `!` anteriores no projeto atual
* Suporta preenchimento automático de caminho de arquivo ao vivo a partir da v2.1.193 em todas as plataformas: digite um token contendo uma barra, como `./src/` ou `~/`, para ver uma lista suspensa de arquivos e diretórios correspondentes, depois pressione `Tab` para aceitar. Use barras para frente no Windows também; a lista suspensa é acionada por `/`, não `\`
* Saia com `Escape`, `Backspace` ou `Ctrl+U` em um prompt vazio
* Colar texto que começa com `!` em um prompt vazio entra no modo shell automaticamente, correspondendo ao comportamento de `!` digitado

A menos que sua sessão seja uma daquelas listadas em [modo de sandbox estrito](/docs/pt/sandboxing#the-unsandboxed-retry-escape-hatch), os comandos que você digita no modo shell são executados fora da [sandbox](/docs/pt/sandboxing) mesmo quando você ativou sandboxing, porque a sandbox se aplica aos comandos que Claude executa.

Claude responde automaticamente à saída do comando assim que ela chega à transcrição, para que você possa executar `! npm test` e obter uma explicação das falhas sem um segundo prompt. A resposta custa o mesmo que enviar um prompt normal. Para restaurar o comportamento anterior onde a saída é adicionada ao contexto sem uma resposta, defina [`respondToBashCommands`](/docs/pt/settings-reference#respondtobashcommands) como `false` em `settings.json`. Antes da v2.1.186, o modo shell sempre adicionava saída ao contexto sem uma resposta.

<h2 id="queue-messages-while-claude-works">
  Enfileirar mensagens enquanto Claude trabalha
</h2>

Digite uma mensagem e pressione `Enter` enquanto Claude está trabalhando. Claude Code enfileira a mensagem em vez de interromper a rodada, e lista as entradas enfileiradas acima da caixa de entrada até enviá-las. Você pode enfileirar `!` [comandos shell](#shell-mode-with-prefix) e a maioria dos [comandos](/docs/pt/commands) da mesma forma, com exceção dos comandos, como `/status`, que Claude Code executa assim que você os envia.

Mensagens enviadas e enfileiradas aparecem em cinza até Claude começar a responder a elas, para que você possa saber quais mensagens Claude ainda não começou.

<h3 id="when-claude-code-sends-what-you-queued">
  Quando Claude Code envia o que você enfileirou
</h3>

Quando uma entrada enfileirada é enviada por Claude Code depende do que você enfileirou.

* Mensagens: se você enfileirar uma mensagem enquanto Claude está executando chamadas de ferramentas, Claude Code a passa para Claude assim que essas chamadas de ferramentas terminam, dentro da mesma rodada. Quando a rodada termina com mensagens ainda enfileiradas, elas saem sem outra pressão de tecla, na ordem em que você as digitou
* Comandos e comandos shell: Claude Code os mantém até o final da rodada, depois os executa um de cada vez, mantendo a ordem em que você os enfileirou

Para enviar o que você enfileirou sem esperar, pressione `Ctrl+Enter`. Suas mensagens enfileiradas saem imediatamente, com seu rascunho enfileirado atrás delas se você tiver digitado um. Requer Claude Code v2.1.275 ou posterior.

Se você enfileirou um comando `!` shell à frente de suas mensagens, a tecla interrompe a rodada. Caso contrário, o que acontece com a rodada depende do que Claude está fazendo quando você pressiona a tecla:

* Executando comandos shell, subagentes ou outro trabalho que pode se mover para o [background](#background-bash-commands): esse trabalho se move para o background e continua em execução, e Claude lê suas mensagens na mesma rodada
* Apenas escrevendo uma resposta, ou executando algo que não pode se mover para o background: Claude Code interrompe a rodada e envia suas mensagens em seguida. Antes da v2.1.281, a tecla interrompia a rodada em ambos os casos

Em [modo shell](#shell-mode-with-prefix), a tecla apenas enfileira seu comando. Em terminais que não relatam teclas estendidas, `Ctrl+Enter` chega como `Enter` simples e enfileira o rascunho em vez disso; `Ctrl+X Ctrl+S` funciona em qualquer terminal. Ambas as teclas são vinculações da ação [`chat:sendNow`](/docs/pt/keybindings#chat-actions).

Pressione `Esc` para interromper a rodada sem enviar seu rascunho. Claude Code mantém o que você enfileirou e o envia imediatamente.

Claude Code executa alguns comandos assim que você os envia em vez de enfileirá-los, entre eles `/model`, `/effort` e `/fast`. Cada um dos três altera uma configuração: o modelo, o nível de esforço ou o modo rápido. Se Claude Code aplica a nova configuração à rodada em que Claude já está trabalhando, ou apenas a partir de sua próxima rodada, difere por comando:

* [`/model`](/docs/pt/model-config#setting-your-model): uma vez que você confirme o [aviso de cache](/docs/pt/prompt-caching#switching-models), se Claude Code mostrar um, Claude Code aplica sua alteração à próxima solicitação que faz nessa rodada
* [`/effort`](/docs/pt/model-config#adjust-effort-level): uma vez que você confirme o [aviso de cache](/docs/pt/prompt-caching#changing-effort-level), se Claude Code mostrar um, Claude Code aplica sua alteração à próxima solicitação que faz nessa rodada
* [`/fast`](/docs/pt/fast-mode#toggle-fast-mode): Claude Code mantém a configuração de modo rápido que estava ativa quando a rodada começou, portanto sua alteração de velocidade se aplica a partir de sua próxima rodada. Se seu modelo atual não suportar modo rápido, ativá-lo também [alterna seu modelo](/docs/pt/prompt-caching#turning-on-fast-mode), e Claude Code usa o novo modelo a partir de sua próxima solicitação nessa rodada

<h3 id="take-back-what-you-queued">
  Recupere o que você enfileirou
</h3>

Pressione `Up` a partir da primeira linha da caixa de entrada para recuperar as mensagens e comandos enfileirados. Claude Code os remove da fila e os coloca na caixa de entrada, um por linha, à frente de qualquer texto que você tenha digitado. Edite o texto e pressione `Enter` para enfileirá-lo novamente como uma entrada, ou limpe a caixa de entrada para descartá-lo.

Claude Code recupera comandos shell enfileirados apenas quando a caixa de entrada está vazia e você não tem nada mais enfileirado, e alterna a caixa de entrada para modo shell quando o faz. Caso contrário, os deixa na fila, listados com seu prefixo `!`, e os executa após o término da rodada.

<h2 id="prompt-suggestions">
  Sugestões de prompt
</h2>

Quando você abre uma sessão pela primeira vez, Claude Code mostra um comando de exemplo esmaecido na entrada de prompt para ajudá-lo a começar. Ele escolhe isso do histórico git do seu projeto, portanto o exemplo reflete arquivos nos quais você trabalhou recentemente.

Depois que Claude responde, Claude Code pode sugerir seu próximo prompt com base no histórico da conversa, como uma etapa de acompanhamento de uma solicitação de várias partes ou uma continuação natural do seu fluxo de trabalho.

* Pressione `Tab` ou `Seta para a direita` para colocar a sugestão na entrada de prompt e depois `Enter` para enviar
* Comece a digitar para descartá-la

Claude Code gera cada uma dessas sugestões de próximo prompt com uma solicitação em segundo plano para o mesmo modelo que sua sessão está usando. A solicitação conta para os limites de uso do seu plano ou seus custos de API. Como reutiliza o cache de prompt da conversa, é principalmente leituras de cache mais alguns tokens de saída, portanto o custo adicional é pequeno.

<h3 id="when-claude-code-skips-suggestions">
  Quando Claude Code pula sugestões
</h3>

No modo interativo, Claude Code deixa as sugestões de prompt desativadas por padrão e oculta o toggle **Prompt suggestions** em `/config` em uma [sessão que não busca sinalizadores de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching), como uma em um provedor de terceiros ou através de um gateway de aplicativos Claude, e em uma [primeira sessão após uma instalação ou atualização](/docs/pt/env-vars#first-session-after-an-install-or-upgrade) cujos sinalizadores ainda não chegaram.

Claude Code também pula sugestões individuais em várias situações, incluindo:

* O cache de prompt está frio, para evitar custo desnecessário
* Após a primeira volta de uma conversa, em algumas sessões
* A resposta anterior terminou em um erro
* Enquanto você está no Plan Mode
* Sua conta está próxima ou no limite de uso. Para manter as sugestões ativadas até atingir o limite, defina [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/pt/env-vars) como `true`. Antes da v2.1.238, Claude Code as pulava perto do limite mesmo com a variável definida como `true`
* Em uma [equipe de agentes](/docs/pt/agent-teams), nas sessões de colegas de equipe por padrão. A sessão do líder mostra sugestões

No modo de impressão, Claude Code não gera sugestões por padrão. Passe [`--prompt-suggestions`](/docs/pt/cli-reference#cli-flags) com `-p "<prompt>" --output-format stream-json --verbose` para que Claude Code emita uma mensagem `prompt_suggestion` após cada volta que gera uma. O gerador pula conversas muito curtas e caches de prompt frios aqui também, portanto uma única consulta `-p` curta pode não emitir nenhuma.

<h3 id="turn-prompt-suggestions-off">
  Desativar sugestões de prompt
</h3>

Para desativar completamente as sugestões de prompt, use qualquer uma das seguintes opções:

* Desative **Prompt suggestions** em `/config`
* Defina [`promptSuggestionEnabled`](/docs/pt/settings-reference#promptsuggestionenabled) como `false` no seu arquivo de configurações
* Defina a variável de ambiente [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/pt/env-vars) como `false`, que tem precedência sobre a configuração:
  ```bash theme={null}
  export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
  ```

Para desativar as sugestões de prompt em toda uma organização, defina `promptSuggestionEnabled` como `false` em [configurações gerenciadas](/docs/pt/managed-settings). Também defina `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION` como `false` sob a chave [`env`](/docs/pt/settings-reference#env) gerenciada para que os usuários não possam reativá-las com sua própria variável de ambiente.

<h2 id="emoji-shortcodes">
  Códigos de emoji
</h2>

Digite um `:` seguido por um código de emoji no campo de entrada do prompt para inserir o emoji. Requer Claude Code v2.1.217 ou posterior.

* Digite um código completo como `:heart:` e Claude Code o substitui por ❤️ assim que você digita o `:` de fechamento
* Digite `:` mais pelo menos dois caracteres de um nome, como `:hea`, para abrir um popup de sugestão, depois pressione `Tab` ou `Enter` para inserir o emoji destacado

O código deve começar a entrada ou seguir um espaço, portanto um `:` dentro de uma palavra ou URL não abre sugestões.

Para desativar o recurso, defina [`emojiCompletionEnabled`](/docs/pt/settings-reference#emojicompletionenabled) como `false` em `settings.json`. Isso desativa tanto o popup de sugestão quanto a substituição inline.

<h2 id="check-spelling-as-you-type">
  Verificar ortografia enquanto você digita
</h2>

Claude Code pode sublinhar palavras com erros de ortografia na entrada do prompt enquanto você digita. Ele verifica apenas o texto na caixa de entrada, nunca as respostas do Claude ou seus arquivos. Ele também não verifica nada enquanto a caixa de entrada está em [modo shell](#shell-mode-with-prefix), busca de histórico `Ctrl+R`, ou [ditado por voz](/docs/pt/voice-dictation).

A verificação de ortografia está desativada por padrão, e Claude Code não verifica nada em [modo leitor de tela](/docs/pt/accessibility). Requer Claude Code v2.1.235 ou posterior.

<h3 id="prerequisites">
  Pré-requisitos
</h3>

* Instale [aspell](https://github.com/GNUAspell/aspell), [hunspell](https://github.com/hunspell/hunspell), ou [ispell](https://en.wikipedia.org/wiki/Ispell) e certifique-se de que está em seu `PATH`. Claude Code executa o primeiro dos três que encontra, nessa ordem, em todas as plataformas, incluindo um shim `.cmd` que um gerenciador de pacotes instala no Windows.
* Para verificar se o programa está em seu `PATH`, execute `aspell --version`, `hunspell --version`, ou `ispell -v` em seu terminal. Um erro "command not found" significa que ainda não está em seu `PATH`.

<h3 id="turn-spell-checking-on-or-off">
  Ativar ou desativar a verificação de ortografia
</h3>

Claude Code lê a configuração [`spellcheck`](/docs/pt/settings-reference#spellcheck) de três lugares e a ignora no `.claude/settings.json` e `.claude/settings.local.json` de um projeto. Ative-a em qualquer um que você use:

<Tabs>
  <Tab title="Configurações do usuário">
    Adicione `spellcheck` a `~/.claude/settings.json`. Ela se aplica em todos os projetos que você abre, como o resto de suas [configurações do usuário](/docs/pt/settings#where-settings-live):

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>

  <Tab title="Linha de comando">
    Salve `spellcheck` em um arquivo JSON, como `spellcheck.json`:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```

    Em seguida, passe o arquivo para `--settings`. Ela se aplica apenas a essa sessão:

    ```bash theme={null}
    claude --settings spellcheck.json
    ```
  </Tab>

  <Tab title="Configurações gerenciadas">
    Adicione `spellcheck` a uma das [fontes de configurações gerenciadas](/docs/pt/permissions#managed-settings) de sua organização. Ela se aplica a todos os usuários que recebem essas configurações, e eles não podem desativá-la:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>
</Tabs>

Para verificar se a verificação de ortografia está ativada, digite uma palavra com erro de ortografia e um espaço. Claude Code sublinha a palavra. Se não fizer isso, consulte [Quando Claude Code não sublinha nada](#when-claude-code-underlines-nothing). Para desativar a verificação de ortografia novamente, defina `enabled` como `false` no mesmo lugar ou remova `spellcheck`.

Para escolher qual dos três programas Claude Code executa, qual dicionário ele usa ou a cor do sublinhado, adicione qualquer um desses campos ao lado de `enabled`, no mesmo lugar:

* `checker`: `aspell`, `hunspell`, ou `ispell`. Claude Code não volta de um verificador que você nomeia e trata qualquer outro valor como `auto`.
* `language`: um nome de dicionário na forma do seu verificador, como `en_GB`. Claude Code ignora qualquer valor que não seja um nome de dicionário simples, como um caminho ou um nome com espaços, e o verificador usa seu dicionário padrão.
* `color`: um nome de cor como `yellow`, ou um valor `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, ou `ansi:<name>`. Claude Code usa a cor de erro do seu tema por padrão e para qualquer valor que não reconheça.

Por exemplo, essa configuração `spellcheck` executa hunspell com seu dicionário `en_GB` e sublinha palavras em amarelo. Funciona da mesma forma em `~/.claude/settings.json`, no arquivo que você passa para `--settings` e nas configurações gerenciadas:

```json theme={null}
{
  "spellcheck": {
    "enabled": true,
    "checker": "hunspell",
    "language": "en_GB",
    "color": "yellow"
  }
}
```

Se mais de um dos três lugares tiver uma configuração `spellcheck`, Claude Code usa apenas um deles: configurações gerenciadas primeiro, depois `--settings`, depois configurações do usuário. Ele não combina campos de dois lugares. Por exemplo, quando `--settings` define `spellcheck`, um `language` em suas configurações do usuário não tem efeito.

<h3 id="what-claude-code-underlines">
  O que Claude Code sublinha
</h3>

Pouco depois que você pausa de digitar, Claude Code sublinha as palavras que o dicionário não conhece. Ele deixa a palavra que você ainda está digitando sozinha até que você se mova além dela, e nunca altera seu texto. Ele também pula texto que parece código:

* Comandos como `/help`, menções `@`, URLs, caminhos de arquivo e sinalizadores como `--verbose`
* Palavras com dígitos, sublinhados ou uma letra maiúscula após a primeira, e texto entre backticks

Claude Code também pula texto em chinês, japonês, coreano, tailandês, laosiano, khmer e birmanês.

Claude Code não tem sua própria lista de palavras: uma palavra está com erro de ortografia quando seu verificador diz que está. Para impedir que Claude Code sublinhe uma palavra, adicione a palavra ao dicionário pessoal do seu verificador, seguindo a documentação do próprio verificador. Claude Code pega a nova palavra após reiniciá-lo.

<h3 id="when-claude-code-underlines-nothing">
  Quando Claude Code não sublinha nada
</h3>

Claude Code não sublinha nada quando não consegue manter um verificador em execução:

* Nenhum verificador está instalado, ou o que você nomeou em `checker` está faltando
* O verificador falha duas vezes seguidas, na inicialização ou posteriormente na sessão. Claude Code o reinicia após a primeira falha e para de verificar após a segunda, até que você reinicie Claude Code
* O verificador leva mais de 15 segundos para responder, três vezes. Cada vez, Claude Code deixa as palavras que estava esperando sem marcar; após a terceira, ele para de verificar até que você reinicie Claude Code

Para descobrir qual desses aconteceu, inicie `claude --debug` com a verificação de ortografia ativada e digite uma palavra. Em seguida, procure pelas linhas `[spellcheck]` no log de depuração em `~/.claude/debug/<session-id>.txt`. Uma linha nomeia o programa que Claude Code iniciou ou lista os que procurou e não encontrou. Linhas posteriores dizem por que parou. Um erro de dicionário ausente lá significa que o verificador não tem dicionário para seu valor `language`, ou nenhum padrão quando `language` não está definido. Instale um ou defina `language` para um dicionário que você tenha.

<h2 id="invisible-characters-in-prompts">
  Caracteres invisíveis em prompts
</h2>

O texto colado pode conter caracteres Unicode que um terminal desenha como nada, como caracteres de tag, controles bidirecionais e espaços de largura zero, portanto um prompt pode conter texto que você nunca vê. Para evitar que o texto copiado carregue instruções que seu terminal não desenha, Claude Code remove esses caracteres quando você pressiona Enter, antes de enviar qualquer coisa. Ele limpa tanto o prompt quanto o conteúdo de qualquer [referência de texto colado](/docs/pt/terminal-config#paste-large-content) que o prompt inclui. Claude Code mantém os juntadores que scripts persa e índico escrevem e os seletores dentro de sequências de emoji.

Se Claude Code removeu algo, esse Enter não envia nada. O prompt limpo volta para a caixa de entrada com um aviso como `Removed 3 invisible characters · review and press Enter to send`, e pressionar Enter novamente envia o texto conforme mostrado.

Quando você passa um prompt na linha de comando, como em `claude "fix the login bug"`, ou canaliza um para uma sessão interativa, Claude Code não espera por um segundo Enter. Ele remove os caracteres, mostra um aviso e envia o prompt limpo. Se o prompt limpo começaria com `/`, Claude Code o coloca na caixa de entrada para você revisar e enviar.

<h2 id="review-changes-with-/diff">
  Revise alterações com /diff
</h2>

Execute `/diff` para revisar as alterações em sua árvore de trabalho sem sair do Claude Code. Você vê as edições que Claude fez até agora junto com qualquer outra coisa que você não tenha confirmado.

Nas alterações que `/diff` lê do git, um submódulo aparece como uma única entrada, e apenas quando o commit para o qual aponta muda; edições de arquivos dentro do submódulo não aparecem lá.

Na [renderização em tela cheia](/docs/pt/fullscreen), `/diff` abre o [painel de diff](#diff-panel) ao lado da conversa, que permanece aberto e se atualiza enquanto você continua trabalhando. No renderizador clássico, `/diff` abre o [visualizador de diff](#diff-viewer) no lugar do prompt, e você o fecha quando terminar de ler.

<h3 id="diff-panel">
  Painel de diff
</h3>

O painel de diff lista os arquivos alterados com suas contagens de linhas adicionadas e removidas, e mostra o diff de cada arquivo sob a lista. Claude Code o atualiza cada vez que Claude edita um arquivo ou executa um comando shell. Para fechá-lo, execute `/diff` novamente ou clique no `✕` em seu cabeçalho.

Para usar o painel você precisa:

* [Renderização em tela cheia](/docs/pt/fullscreen)
* Um repositório git
* Um terminal com pelo menos 110 colunas de largura
* Claude Code v2.1.260 ou posterior

Quando o painel não consegue abrir, `/diff` abre o visualizador de diff em seu lugar ou informa o motivo.

O painel também abre por conta própria assim que Claude começa a editar arquivos, se seu terminal tiver pelo menos 144 colunas de largura. Depois que você o abrir com `/diff`, as sessões posteriores o abrem assim que Claude edita um arquivo em qualquer terminal com largura suficiente para ajustá-lo. Feche o painel e ele permanecerá fechado, nesta sessão e nas posteriores, até que você execute `/diff` novamente.

Enquanto o painel está aberto, você pode:

* **Ir para um arquivo**: clique em sua linha na lista. Role o painel com a roda do mouse. Quando a lista de arquivos em si é muito longa para caber, role-a com `Alt+Up` e `Alt+Down`, ou `Ctrl+Up` e `Ctrl+Down`.
* **Pergunte a Claude sobre linhas específicas**: selecione-as no painel com o mouse. Claude Code anexa a seleção ao seu próximo prompt e mostra uma contagem de linhas na entrada até que você o envie.
  * Para enviar o prompt sem a seleção, mova o cursor para logo após o indicador de contagem de linhas e pressione `Backspace` para deletá-lo. Requer Claude Code v2.1.271 ou posterior.
* **Mostrar os arquivos que o painel deixa de fora**: a lista pula arquivos de teste e arquivos gerados, e collapsa alterações de antes desta sessão em uma linha na parte inferior. Clique em qualquer linha de contagem para expandi-la.
* **Alterar com o que o painel compara**: pressione `Ctrl+X B` para alternar entre as alterações desta sessão, suas alterações não confirmadas como uma lista, para tudo desde que sua ramificação se dividiu da ramificação padrão. Claude Code lembra a escolha para cada projeto.

Para vincular teclas a essas ações, consulte [Ações do painel de diff](/docs/pt/keybindings#diff-panel-actions).

<h3 id="diff-viewer">
  Visualizador de diff
</h3>

O visualizador de diff ocupa o lugar do prompt até que você o feche. Sua visualização **Current** mostra suas alterações não confirmadas do git, ou, quando não há nenhuma, o que sua ramificação adiciona na ramificação padrão. O visualizador também tem uma visualização de turno para cada prompt após o qual Claude editou arquivos, mostrando apenas essas edições. Claude Code constrói as visualizações de turno a partir das edições de arquivo do Claude em vez do git, portanto uma alteração que Claude faz através de um comando shell aparece apenas em Current.

Use essas teclas no visualizador:

* **Esquerda e Direita**: mova-se entre Current e as visualizações de turno.
* **Cima e Baixo**: selecione um arquivo.
* **Enter**: abra o diff do arquivo selecionado. Role-o com Cima e Baixo, ou PageUp e PageDown.
* **Esc**: retorne do diff de um arquivo para a lista, ou feche o visualizador da lista.

Para reatribuir essas teclas, consulte [Ações de diff](/docs/pt/keybindings#diff-actions).

<h2 id="side-questions-with-/btw">
  Perguntas laterais com /btw
</h2>

Use `/btw` para fazer uma pergunta sobre seu trabalho atual sem adicionar ao histórico de conversa.

```
/btw what was the name of that config file again?
```

Claude responde uma pergunta lateral a partir do que já está na conversa: suas mensagens, suas respostas e os resultados de ferramentas que coletou. Você pode perguntar sobre código que Claude já leu, decisões que tomou anteriormente ou qualquer outra coisa da sessão. Uma pergunta lateral posterior também vê suas perguntas laterais anteriores: Claude Code reproduz as 20 trocas mais recentes com cada pergunta, até você limpá-las. A pergunta e a resposta nunca entram no histórico de conversa. No terminal, elas aparecem em uma sobreposição dispensável. O terminal mantém o thread na memória: pressione `x` para limpar as trocas anteriores, e desaparece quando você sai do Claude Code.

Na [extensão do VS Code](/docs/pt/vs-code#use-the-prompt-box) do painel de chat, `/btw` abre um painel em vez da sobreposição que esta seção descreve, e você faz perguntas de acompanhamento direto no painel. O thread do painel sobrevive a recarregamentos de janela, no cronograma de retenção que essa página descreve. Você precisa da extensão na versão 2.1.227 ou posterior. Versões anteriores da extensão não oferecem `/btw`.

* **Disponível enquanto Claude está trabalhando**: você pode executar `/btw` mesmo enquanto Claude está processando uma resposta. A pergunta lateral é executada independentemente e não interrompe a volta principal. Ela vê tudo na conversa até agora, exceto a resposta que Claude ainda está escrevendo.
* **Sem acesso a ferramentas**: perguntas laterais respondem apenas a partir do que já está em contexto. Claude não pode ler arquivos, executar comandos ou pesquisar ao responder uma pergunta lateral. Se Claude escrever chamadas de ferramentas como texto mesmo assim, a resposta termina com uma nota de que nada foi executado.
* **Resposta única**: não há turnos de acompanhamento na sobreposição. Para continuar o thread, faça outra pergunta `/btw`. Para continuar com acesso total a ferramentas em uma sessão local, pressione `f` para bifurcar esta pergunta e resposta em um [subagentesubagente de fundo](/docs/pt/sub-agents#fork-the-current-conversation).
* **Baixo custo**: enquanto o [cache de prompt](/docs/pt/prompt-caching) da conversa está aquecido, uma pergunta lateral custa pouco além da resposta em si.

Suas cinco perguntas laterais anteriores mais recentes aparecem como uma lista esmaecida acima da resposta atual, com uma contagem de qualquer uma mais antiga. Elas ficam fora do histórico de conversa.

Para retornar à sobreposição após dispensá-la, execute `/btw` sem uma pergunta. A sobreposição reabre em sua troca mais recente. Antes da v2.1.212, `/btw` sem uma pergunta imprimia uma mensagem de uso em vez disso.

Assim que a resposta aparecer, a sobreposição aceita essas teclas.

| Tecla                        | Ação                                                                                                                                                                                                                                                                                                                                                                                                   |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space`, `Enter`, `Escape`   | Dispensar a resposta e retornar ao prompt                                                                                                                                                                                                                                                                                                                                                              |
| `Up` / `Down`                | Rolar a resposta                                                                                                                                                                                                                                                                                                                                                                                       |
| `Shift+Left` / `Shift+Right` | Passar entre esta resposta e suas respostas anteriores `/btw`. `Shift+Left` move para respostas mais antigas e `Shift+Right` retorna para a atual. `[` e `]` fazem o mesmo, para terminais que não relatam `Shift` com teclas de seta. `Tab` / `Shift+Tab` percorrem as mesmas respostas. Requer Claude Code v2.1.257 ou posterior. Entre v2.1.187 e v2.1.256, as teclas eram `Left` / `Right` simples |
| `c`                          | Copiar a resposta para sua área de transferência como Markdown bruto. Use isso em vez de seleção de mouse, que captura a renderização do terminal com quebra de linha rígida em vez do texto de origem                                                                                                                                                                                                 |
| `f`                          | Iniciar um [subagentesubagente bifurcado](/docs/pt/sub-agents#fork-the-current-conversation) que herda a conversa pai mais esta pergunta e resposta, para que possa continuar com acesso total a ferramentas. Você permanece na sessão atual e encontra a bifurcação no [painel abaixo do seu prompt](/docs/pt/sub-agents#observe-and-steer-running-forks). Disponível apenas em sessões locais                  |
| `x`                          | Limpar a lista de trocas anteriores `/btw` mostradas acima da resposta atual                                                                                                                                                                                                                                                                                                                           |

Em uma [sessão de fundo](/docs/pt/agent-view#attach-to-a-session) anexada, `Left` desanexa e retorna você à visualização de agente, mesmo enquanto a resposta ainda está chegando. A pergunta lateral continua em execução enquanto você está ausente. Na próxima vez que você anexar à sessão, a sobreposição reabre com a pergunta lateral ou com sua resposta. Antes da v2.1.257, `Left` não desanexava lá.

`/btw` vê sua conversa completa mas não tem ferramentas. Um [subagentesubagente](/docs/pt/sub-agents) tem ferramentas e começa a partir do prompt que recebe, ou, para uma [bifurcação](/docs/pt/sub-agents#fork-the-current-conversation), a partir de uma cópia desta conversa. Use `/btw` para perguntar sobre o que Claude já sabe desta sessão; use um subagentesubagente para descobrir algo novo.

<h2 id="task-list">
  Lista de tarefas
</h2>

A lista de tarefas é a lista de verificação de Claude: itens que Claude criou para planejar trabalho em várias etapas, com indicadores mostrando o que está pendente, em progresso ou concluído. É separada da visualização de tarefa em segundo plano. Para ver shells em execução e subagentes, use [`/tasks`](/docs/pt/commands) em vez disso.

A lista é preenchida apenas em sessões que possuem as ferramentas de rastreamento de tarefas, que Claude Code fornece por padrão em [modelos Claude 3.x, Opus 4 até 4.7, Sonnet 4 até 4.6 e Haiku 4.5](/docs/pt/tools-reference#task-tool-availability). Em qualquer outro modelo, incluindo um ID de modelo que Claude Code não reconhece, a lista permanece vazia a menos que você opte por participar com `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` ou uma das outras maneiras em [Disponibilidade da ferramenta de tarefas](/docs/pt/tools-reference#task-tool-availability). Quando a sessão possui as ferramentas, a lista de tarefas funciona da seguinte forma:

* Pressione `Ctrl+T` para alternar a visualização da lista de tarefas. A exibição mostra até cinco tarefas por vez. Quando Claude ainda não criou nenhum item de lista de verificação, o alternador não tem efeito visível porque não há nada para exibir
* Se você deixar a lista expandida, Claude Code restaura a visualização expandida na próxima vez que você iniciar uma sessão que ainda tenha tarefas, como com `--resume` ou `--continue`. Quando a lista de tarefas está vazia, Claude Code a inicia recolhida
* Para ver todas as tarefas ou limpá-las, peça a Claude diretamente: "show me all tasks" ou "clear all tasks"
* As tarefas persistem entre compactações de contexto, ajudando Claude a se manter organizado em projetos maiores
* Para compartilhar uma lista de tarefas entre sessões, defina `CLAUDE_CODE_TASK_LIST_ID` para usar um diretório nomeado em `~/.claude/tasks/`: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Recapitulação da sessão
</h2>

Quando você retorna ao terminal após se afastar, Claude Code mostra um resumo de uma linha do que aconteceu na sessão até agora. O resumo é gerado em segundo plano uma vez que pelo menos três minutos tenham passado desde o último turno concluído e o terminal esteja sem foco, para que esteja pronto quando você voltar. Os resumos aparecem apenas uma vez que a sessão tenha pelo menos três turnos, e nunca duas vezes seguidas.

Execute `/recap` para gerar um resumo sob demanda. Claude Code limita tanto os resumos automáticos quanto a saída de `/recap` a 400 caracteres. Para desativar os resumos automáticos, abra `/config` e desative **Session recap**.

A recapitulação da sessão está ativada por padrão para todos os planos e provedores. O resumo é sempre ignorado no modo não interativo.

<h2 id="wait-for-a-usage-limit-to-reset">
  Aguarde o reset de um limite de uso
</h2>

Quando um [limite de uso](/docs/pt/errors#youve-hit-your-session-limit) do claude.ai interrompe Claude no meio de uma tarefa, Claude Code aguarda na sessão aberta e continua a tarefa automaticamente após o reset do limite. A continuação automática está ativada por padrão em sessões interativas conectadas com uma assinatura do claude.ai. Requer Claude Code v2.1.234 ou posterior.

Enquanto Claude Code aguarda, uma linha na parte inferior da sessão mostra quando ele continuará:

```text theme={null}
Usage limit reached · continuing automatically at 3:45pm · esc to cancel
```

Mantenha a sessão aberta. O que acontece a seguir depende de como a espera termina:

* **No reset**: a linha lê `continuing shortly`, depois `Usage limit reset · continuing automatically`, e Claude Code envia a Claude um prompt fixo para retomar a tarefa de onde parou. Ele não reenvia sua última mensagem.
* **Após seu computador dormir**: se ele dormiu por mais de cerca de 30 minutos e o limite foi resetado enquanto dormia, a linha lê `Your usage limit has reset · press enter to continue`. Pressione `Enter` para continuar. Após um sono mais curto, Claude Code continua automaticamente.
* **Antecipadamente**: quando você termina de adicionar [créditos de uso](/docs/pt/costs#add-usage-credits-to-your-subscription) com `/usage-credits`, faz login novamente após `/upgrade`, ou muda de modelo com `/model` durante a espera, Claude Code verifica se o uso está disponível novamente e continua imediatamente se estiver. Ele não verifica após uma atualização ou compra que você faz em um navegador por conta própria. Em [`opusplan`](/docs/pt/model-config#opusplan-model-setting) e outras configurações de modelo que executam o modo de plano em um modelo diferente, Claude Code aguarda o reset.

A tarefa continuada é executada como qualquer outro turno. Claude Code ainda solicita [permissões](/docs/pt/permissions) como de costume, portanto a tarefa pode parar em um prompt enquanto você está ausente. Se atingir o limite novamente, Claude Code reativa a espera automaticamente no máximo duas vezes seguidas, depois para e mostra `Automatic continue stopped after repeated usage-limit hits · /rate-limit-options to try again`.

<h3 id="cancel-the-wait">
  Cancelar a espera
</h3>

Pressione `Esc` em um prompt vazio, ou `Ctrl+C`, enquanto a linha é exibida, ou execute [`/rate-limit-options`](/docs/pt/commands#all-commands) e escolha **Don't continue automatically**. Claude Code confirma com uma linha que começa com `Automatic continue cancelled`.

Após um cancelamento, nada continua até você enviar um prompt ou escolher a linha que começa com **Wait here, then continue automatically** de `/rate-limit-options` novamente. Claude Code não inicia uma espera por conta própria novamente para essa janela de reset; a próxima janela de reset começa do zero.

A espera também termina sem continuar a tarefa nestes casos:

* **Você envia um prompt**: Claude Code executa seu prompt em vez de aguardar.
* **Você sai do Claude Code**: a espera não reinicia quando você retoma a sessão.
* **A conversa muda de mãos**: você muda de conta com `/login`, limpa ou retrocede a conversa, `/resume` outra sessão, puxa uma com `/teleport`, relança com `/tui`, ou entrega a sessão ao Claude Desktop, uma sessão em segundo plano, ou a nuvem.
* **A configuração é desativada, ou o reset passa de 24 horas**: isso encerra apenas uma espera que Claude Code iniciou por conta própria. Uma espera que você escolheu de `/rate-limit-options` continua contando regressivamente.
* **A continuação é bloqueada**: um hook [`UserPromptSubmit`](/docs/pt/hooks#userpromptsubmit) que bloqueia o prompt de continuação, ou uma falha antes de atingir o modelo, encerra a espera. Claude Code informa que a continuação não foi executada. Envie um prompt para continuar.

<h3 id="start-a-wait-yourself">
  Inicie uma espera você mesmo
</h3>

Claude Code não inicia a espera por conta própria nestes casos:

* **Sessões de Remote Control e colega de equipe de agente**: uma pessoa nesse terminal ainda pode iniciar uma.
* **Um reset a mais de 24 horas de distância**: um limite semanal pode resetar dias depois.
* **Um limite de Opus ou Sonnet enquanto você executa um modelo fora dessa família**: seu próximo turno pode não atingir esse limite. [`opusplan`](/docs/pt/model-config#opusplan-model-setting) e outras configurações de modelo que executam o modo de plano na família limitada não recebem essa exceção.

Nesses casos, e sempre que a continuação automática está desativada, Claude Code abre o menu de opções de limite de uso uma vez por janela de reset quando você atinge um limite em seu próprio terminal. Escolha a linha que começa com **Wait here, then continue automatically** para iniciar a espera. Em uma sessão de [Remote Control](/docs/pt/remote-control) ou colega de [equipe de agente](/docs/pt/agent-teams), execute `/rate-limit-options` você mesmo para abrir o menu.

Claude Code não oferece a espera em absoluto nestes casos:

* **Sessões em segundo plano e execuções `-p`**: a linha do menu não está disponível.
* **Chaves de API, provedores de nuvem e faturamento baseado em uso**: o uso lá é medido por solicitação, portanto não há reset para aguardar.
* **Um [gateway LLM](/docs/pt/llm-gateway#subscriptions-and-gateways) sem um login do claude.ai salvo**: Claude Code oferece a espera apenas enquanto um login do claude.ai salvo é a credencial ativa.

<h3 id="turn-automatic-continue-off">
  Desative a continuação automática
</h3>

Em `/config`, desative **Continue automatically at usage limit**, ou defina [`autoContinueAtUsageLimit`](/docs/pt/settings-reference#autocontinueatusagelimit) como `false` em suas configurações de usuário. `/config autoContinueAtUsageLimit=false` também funciona, inclusive com `-p`, mas o formulário `key=value` não pode ativá-lo novamente, porque a configuração concede execução autônoma. Quais arquivos de configuração Claude Code lê para essa chave está na [referência de configurações](/docs/pt/settings-reference#autocontinueatusagelimit).

<h2 id="pr-review-status">
  Status de revisão de PR
</h2>

Ao trabalhar em uma branch com um pull request aberto, Claude Code exibe um link de PR clicável no rodapé, como "PR #446". O link tem um sublinhado colorido indicando o estado da revisão:

* Verde: aprovado
* Amarelo: revisão pendente
* Vermelho: alterações solicitadas
* Cinza: rascunho

O badge desaparece assim que o pull request é mesclado ou fechado.

`Cmd+click` (macOS) ou `Ctrl+click` (Windows/Linux) no link para abrir o pull request no seu navegador.

O status é atualizado assim que um `git push`, ou um comando `gh pr` que altera o pull request, como `gh pr create` ou `gh pr merge`, é executado com sucesso na sessão.

Claude Code renderiza o badge como um hiperlink mesmo quando não consegue detectar suporte a hiperlinks no seu terminal, o que geralmente acontece via SSH ou em tmux. Defina [`FORCE_HYPERLINK=0`](/docs/pt/env-vars) para renderizar o badge como texto simples.

Quando você define [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars), Claude Code não verifica o status do pull request ou merge request.

<Note>
  O status de PR para repositórios GitHub precisa de um token GitHub. Claude Code encontra um com base no host do remote:

  * **github.com**: `GH_TOKEN` ou `GITHUB_TOKEN`, ou o token salvo por `gh auth login`. Sem um, o rodapé mostra `install gh for PR status` quando a CLI `gh` não está instalada, ou `gh auth login for PR status` quando está
  * **Um host GitHub Enterprise definido como `GH_HOST`**: `GH_ENTERPRISE_TOKEN` ou `GITHUB_ENTERPRISE_TOKEN`, ou o token salvo por `gh auth login --hostname <host>`. Sem um, o rodapé mostra as mesmas dicas
  * **Qualquer outro host GitHub**: o token salvo por `gh auth login --hostname <host>`. Sem um, Claude Code não mostra nenhum badge e nenhuma dica
</Note>

<h3 id="gitlab-merge-requests">
  Merge requests do GitLab
</h3>

Quando você trabalha em uma branch com um merge request aberto do GitLab, Claude Code mostra um badge `MR !N` clicável no slot do rodapé que de outra forma conteria o link do PR do GitHub. `!N` é a própria sintaxe de referência do GitLab para o número N do merge request. O sublinhado colorido mostra o estado do merge request:

* Verde: GitLab relata o merge request como mesclável
* Amarelo: qualquer outro estado aberto
* Cinza: rascunho

O badge desaparece assim que o merge request é mesclado ou fechado.

Ele é atualizado assim que um `git push`, ou um comando `glab mr` que altera o merge request, como `glab mr create` ou `glab mr merge`, é executado com sucesso na sessão.

Para obter o badge, você precisa de:

* Claude Code v2.1.234 ou posterior
* Um remote de repositório que aponte para seu host GitLab, seja gitlab.com ou uma instância auto-gerenciada
* A [`glab` CLI](https://gitlab.com/gitlab-org/cli) no seu `PATH`, autenticada com `glab auth login`

Claude Code ignora as variáveis de ambiente de token do `glab`, como `GITLAB_TOKEN`, quando verifica o status, portanto você não obtém nenhum badge apenas de um token exportado. Claude Code também procura por `glab` e por seu login uma vez por sessão, portanto reinicie Claude Code depois de instalar `glab` ou executar `glab auth login`.

<h2 id="issue-reference-links">
  Links de referência de problemas
</h2>

Quando Claude menciona um problema como `owner/repo#123`, você pode clicar na referência para abri-lo, desde que seu terminal suporte hiperlinks. Se Claude Code não detectar suporte a hiperlinks em seu terminal, defina [`FORCE_HYPERLINK`](/docs/pt/env-vars) como `1` para ativar os links, ou como `0` para manter as referências como texto simples.

Você obtém um link apenas para o formulário de duas partes `owner/repo#123`. Estes permanecem como texto simples:

* Um `#123` isolado
* Um caminho GitLab aninhado como `group/subgroup/project#123`
* Qualquer referência dentro de um intervalo de código ou bloco de código

Claude Code constrói o link para o host do repositório que identifica a partir de seu git remote, não para o repositório que a referência nomeia:

| Host do seu repositório                                                    | Onde `owner/repo#123` vincula                       |
| :------------------------------------------------------------------------- | :-------------------------------------------------- |
| github.com, um host GitHub Enterprise, ou qualquer host não listado abaixo | `https://<host>/owner/repo/issues/123`              |
| gitlab.com                                                                 | `https://gitlab.com/owner/repo/-/issues/123`        |
| bitbucket.org, codeberg.org, ou gitea.com                                  | Sem link; a referência permanece como texto simples |

<h2 id="see-also">
  Veja também
</h2>

* [Skills](/docs/pt/skills) - Prompts e fluxos de trabalho personalizados
* [Checkpointing](/docs/pt/checkpointing) - Retroceder edições do Claude e restaurar estados anteriores
* [Referência CLI](/docs/pt/cli-reference) - Sinalizadores e opções de linha de comando
* [Configurações](/docs/pt/settings) - Opções de configuração
* [Gerenciamento de memória](/docs/pt/memory) - Gerenciando arquivos CLAUDE.md
